use crate::{api, Result};
use core::fmt::Debug;
use eyre::{bail, ContextCompat};
use petgraph::{
    dot::{Config, Dot},
    graph::DiGraph,
    visit::{EdgeRef, IntoNodeIdentifiers},
    Directed, Direction, Graph,
};
use rune::{
    alloc::clone::TryClone,
    runtime::{Function, Object, Shared, SyncFunction, VmError, VmResult},
    termcolor::{ColorChoice, StandardStream},
    vm_try, Diagnostics, FromValue, Source, Sources, ToValue, Value, Vm,
};
use std::{borrow::Borrow, cell::RefCell, collections::HashMap, path::Path, sync::Arc};
use tokio::task::JoinSet;
use tracing::{debug, instrument, span, trace, warn, Level};

use rune::alloc::Vec as RVec;

type Action = Option<Arc<SyncFunction>>;

#[derive(Clone)]
pub struct Node {
    name: String,
    before: Vec<String>,
    after: Vec<String>,
    action: Action,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("name", &self.name)
            .field("before", &self.before)
            .field("after", &self.after)
            .field("action", &"<hidden>")
            .finish()
    }
}

impl FromValue for Node {
    fn from_value(value: Value) -> VmResult<Self> {
        let mut raw: Object = vm_try!(rune::from_value(value));

        VmResult::Ok(Node {
            name: vm_try!(remove(&mut raw, "name")),
            after: vm_try!(remove_or_default(&mut raw, "after")),
            before: vm_try!(remove_or_default(&mut raw, "before")),
            action: match remove::<Function, _>(&mut raw, "action") {
                Ok(f) => Some(Arc::new(vm_try!(f.into_sync()))),
                Err(_) => None,
            },
        })
    }
}

#[derive(Debug)]
enum Dependency {
    Weak,
    Strong,
}

#[derive(Debug, Default)]
struct NodeList {
    inner: Vec<Node>,
}

impl Borrow<Vec<Node>> for NodeList {
    fn borrow(&self) -> &Vec<Node> {
        &self.inner
    }
}

impl FromValue for NodeList {
    fn from_value(value: Value) -> VmResult<Self> {
        match value {
            Value::Vec(v) => {
                let v = vm_try!(v.take()).into_inner();
                let mut vfinal = Vec::new();
                for elem in v {
                    let res = vm_try!(Node::from_value(elem));
                    vfinal.push(res);
                }
                VmResult::Ok(Self { inner: vfinal })
            }
            Value::EmptyTuple => VmResult::Ok(Default::default()),
            Value::Tuple(t) => {
                let t = vm_try!(t.take()).to_vec();
                let mut vfinal = Vec::new();
                for elem in t.into_iter() {
                    let res = vm_try!(Node::from_value(elem));
                    vfinal.push(res);
                }
                VmResult::Ok(Self { inner: vfinal })
            }
            v @ Value::Object(_) => {
                let res = vm_try!(Node::from_value(v));
                VmResult::Ok(Self { inner: vec![res] })
            }
            _ => VmResult::panic("failed to parse result as a NodeList"),
        }
    }
}

/// Both removes and coerces from an object
fn remove<T, S>(object: &mut Object, name: S) -> Result<T, VmError>
where
    T: rune::FromValue,
    S: AsRef<str>,
{
    let name = name.as_ref();
    let v = object.remove(name);
    let v = v.ok_or_else(|| VmError::panic(format!("Object doesn't contain key {}", name)))?;
    rune::from_value(v)
}

/// Both removes and coerces from an object
fn remove_or_default<T, S>(object: &mut Object, name: S) -> Result<T, VmError>
where
    T: rune::FromValue + std::default::Default,
    S: AsRef<str>,
{
    let name = name.as_ref();
    let v = object.remove(name);
    match v {
        Some(v) => rune::from_value(v),
        None => Ok(Default::default()),
    }
}

pub async fn eval<P: AsRef<Path>>(manifest: P) -> Result<()> {
    let vm = mk_vm(manifest)?;

    let res = vm
        .try_clone()?
        .send_execute(["mk_nodes"], ())?
        .async_complete()
        .await
        .unwrap();

    let nodes: Vec<Node> = rune::from_value(res)?;
    debug!(?nodes);

    let mut graph: Graph<Node, _> = DiGraph::new();
    let mut all_nodes = HashMap::new();

    // all_nodes.add_node(weight)
    for n in nodes.iter() {
        let idx = graph.add_node(n.clone());
        all_nodes.insert(n.name.as_str(), idx);
    }

    // let idxs: Vec<_> = graph.node_indices().collect();

    for n in nodes.iter() {
        for after in &n.after {
            let idx = all_nodes[n.name.as_str()];
            let jxd = all_nodes.get(after.as_str());
            let jdx = jxd.wrap_err(format!(
                "{} is after {}, but it doesn't exist",
                n.name, after
            ))?;
            graph.add_edge(*jdx, idx, Dependency::Strong);
        }

        for before in &n.before {
            let idx = all_nodes[n.name.as_str()];
            let jxd = all_nodes.get(before.as_str());
            let jdx = jxd.wrap_err(format!(
                "{} is before {}, but it doesn't exist",
                n.name, before
            ))?;
            graph.add_edge(idx, *jdx, Dependency::Strong);
        }
    }

    drop(all_nodes);

    debug!(?graph);
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    run_graph(&mut graph, vm.try_clone()?).await?;

    Ok(())
}

/// Basic Rune initialisation
fn mk_vm<P: AsRef<Path>>(manifest: P) -> eyre::Result<Vm> {
    let mut context = rune_modules::default_context()?;
    context.install(api::module()?)?;

    let runtime = Arc::new(context.runtime()?);

    let mut sources = Sources::new();
    sources.insert(Source::from_path(manifest)?)?;

    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources)?;
    }

    let unit = result?;

    let vm = Vm::new(runtime, Arc::new(unit));

    Ok(vm)
}

#[derive(Debug)]
enum State {
    Finished,
    Running,
    Waiting,
}

async fn run_graph<E>(g: &mut Graph<Node, E, Directed>, _vm: Vm) -> eyre::Result<()> {
    let mut states = HashMap::new();
    for x in g.node_identifiers() {
        states.insert(x, State::Waiting);
    }

    let mut joinset = JoinSet::new();

    for i in 1..100 {
        trace!(?i);
        for idx in g.node_indices() {
            let span = span!(Level::DEBUG, "node", ?idx);
            let _enter = span.enter();

            if matches!(states[&idx], State::Waiting) {
                let all_parents_finished =
                    g.edges_directed(idx, Direction::Incoming)
                        .fold(true, |acc, edge| {
                            let idx_src = edge.source();
                            let parent_state = &states[&idx_src];
                            if matches!(parent_state, State::Finished) {
                                acc
                            } else {
                                false
                            }
                        });

                if all_parents_finished {
                    let f = g[idx].action.take();

                    let name = g[idx].name.clone();
                    joinset.spawn(async move {
                        let span = span!(Level::DEBUG, "task", ?name);
                        let _enter = span.enter();

                        let result = match f {
                            Some(f) => {
                                let res: VmResult<NodeList> = f.async_send_call(()).await;
                                res
                                // VmResult::Ok(res)
                            }
                            None => VmResult::Ok(Default::default()),
                        };

                        (idx, result)
                    });
                }
            }
        }

        while let Some(next) = joinset.join_next().await {
            let (idx, vmresult) = next?;
            if let VmResult::Err(err) = vmresult {
                bail!(err);
            }
            *states.get_mut(&idx).unwrap() = State::Finished;
        }

        let all_finished = states.iter().all(|(_, s)| matches!(s, State::Finished));
        if all_finished {
            debug!("all tasks finished");
            if joinset.len() != 0 {
                joinset.shutdown().await;
                bail!("joinset still contained tasks");
            }
            return Ok(());
        }
    }

    warn!("Iteration limit reached, exiting");
    Ok(())
}

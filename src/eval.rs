use core::fmt::{Debug, Formatter};
use eyre::{bail, ensure, ContextCompat};
use petgraph::{
    dot::{Config, Dot},
    graph::DiGraph,
    visit::{EdgeRef, IntoNodeIdentifiers, IntoNodeReferences, Visitable},
    Directed, Direction, Graph,
};
use rune::{
    alloc::clone::TryClone,
    runtime::{Function, Object, Shared, Struct, SyncFunction, VmError, VmResult},
    termcolor::{ColorChoice, StandardStream},
    vm_try, Any, Diagnostics, FromValue, Hash, Module, Source, Sources, Value, Vm,
};
use std::{
    any::type_name,
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    convert::identity,
    path::Path,
    sync::Arc,
    time::Duration,
};
use tokio::{runtime::Runtime, task::JoinSet};
use tracing::{debug, span, trace, warn, Level};

use crate::{api, Result};

#[derive(Debug, Clone)]
pub struct Node
// manually mark them for me to remember
where
    Self: Send + Sync,
{
    name: String,
    before: Vec<String>,
    after: Vec<String>,
    action: Hash,
}

impl FromValue for Node {
    fn from_value(value: Value) -> VmResult<Self> {
        let mut raw: Object = vm_try!(rune::from_value(value));

        VmResult::Ok(Node {
            name: vm_try!(remove(&mut raw, "name")),
            after: vm_try!(remove_or_default(&mut raw, "after")),
            before: vm_try!(remove_or_default(&mut raw, "before")),
            action: {
                let f: SyncFunction = vm_try!(remove(&mut raw, "action"));
                f.type_hash()
            },
        })
    }
}

#[derive(Debug)]
enum Dependency {
    Weak,
    Strong,
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

    let mut graph = DiGraph::new();
    let mut all_nodes = HashMap::new();

    // all_nodes.add_node(weight)
    for n in nodes.iter() {
        let idx = graph.add_node(n.clone());
        all_nodes.insert(n.name.as_str(), idx);
    }

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

async fn run_graph<E>(g: &mut Graph<Node, E, Directed>, vm: Vm) -> eyre::Result<()> {
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
                    let hash = g[idx].action;
                    let execution = vm.try_clone()?.send_execute(hash, ())?;
                    *states.get_mut(&idx).unwrap() = State::Running;

                    let name = g[idx].name.clone();
                    joinset.spawn(async move {
                        let span = span!(Level::DEBUG, "task", ?name);
                        let _enter = span.enter();
                        let res = execution.async_complete().await;
                        // values are not thread safe, return something else
                        match res {
                            VmResult::Ok(_) => (idx, Ok(())),
                            VmResult::Err(err) => (idx, Err(err)),
                        }
                    });
                }
            }
        }

        while let Some(next) = joinset.join_next().await {
            debug!(?next, "task finished");
            let (idx, vmresult) = next?;
            if let Err(err) = vmresult {
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

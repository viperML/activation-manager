use core::fmt::{Debug, Formatter};
use petgraph::{
    dot::{Config, Dot},
    graph::DiGraph,
    visit::{EdgeRef, IntoNodeIdentifiers, IntoNodeReferences, Visitable},
    Directed, Direction, Graph,
};
use rune::{
    alloc::clone::TryClone,
    runtime::{Function, Shared, Struct, SyncFunction, VmError, VmResult},
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
    // before: Vec<String>,
    after: Vec<String>,
    action: Hash,
}

impl FromValue for Node {
    fn from_value(value: Value) -> VmResult<Self> {
        let node: api::Node = rune::from_value(value).unwrap();
        let action: SyncFunction = rune::from_value(node.action).unwrap();

        VmResult::Ok(Node {
            name: node.name,
            after: node.after,
            action: action.type_hash(),
        })
    }
}

/// Gets an element from an struct by replacing it with the empty tuple
fn get_owned<T, S>(r#struct: &mut Struct, name: S) -> Result<T, VmError>
where
    T: rune::FromValue,
    S: AsRef<str>,
{
    let name = name.as_ref();

    let refmut = r#struct
        .get_mut(name)
        .ok_or_else(|| VmError::panic("failed to get key"))?;
    let value = std::mem::take(refmut);

    rune::from_value(value)
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

    // all_nodes.add_node(weight)
    for n in nodes {
        let idx = graph.add_node(n);
    }

    let mut new_graph = graph.clone();
    for (idx, node) in graph.node_references() {
        for after in &node.after {
            for (jdx, node2) in graph.node_references() {
                if node2.name == *after {
                    new_graph.add_edge(jdx, idx, ());
                }
            }
        }
    }

    debug!(?new_graph);
    println!("{:?}", Dot::with_config(&new_graph, &[Config::EdgeNoLabel]));

    run_graph(&mut new_graph, vm.try_clone()?).await?;

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

            debug!(?all_parents_finished);

            if all_parents_finished && matches!(states[&idx], State::Waiting) {
                *states.get_mut(&idx).unwrap() = State::Running;
                let hash = g[idx].action;
                let execution = vm.try_clone()?.send_execute(hash, ())?;

                joinset.spawn(async move {
                    execution.async_complete().await.unwrap();
                    idx
                });
            }
        }

        while let Some(next) = joinset.join_next().await {
            debug!(?next, "task finished");
            let idx = next?;
            *states.get_mut(&idx).unwrap() = State::Finished;
        }

        let all_finished = states.iter().all(|(_, s)| matches!(s, State::Finished));
        if all_finished {
            debug!("all tasks finished");
            return Ok(());
        }
    }

    warn!("Iteration limit reached, exiting");
    Ok(())
}

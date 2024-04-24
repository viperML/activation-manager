use core::fmt::{Debug, Formatter};
use rune::{
    alloc::clone::TryClone,
    runtime::{Function, Shared, Struct, SyncFunction, VmError, VmResult},
    termcolor::{ColorChoice, StandardStream},
    Any, Diagnostics, FromValue, Hash, Module, Source, Sources, Value, Vm,
};
use std::{any::type_name, borrow::Borrow, path::Path, sync::Arc, time::Duration};
use tokio::runtime::Runtime;
use tracing::debug;

use crate::Result;

#[derive(Debug)]
pub struct Node
// manually mark them for me to remember
where
    Self: Send + Sync,
{
    name: String,
    // before: Vec<String>,
    // after: Vec<String>,
    action: Hash,
}

impl FromValue for Node {
    fn from_value(value: Value) -> VmResult<Self> {
        if let Value::Struct(s) = value {
            let mut s = s.borrow_mut().unwrap();

            let name: String = get_owned(&mut s, "name").unwrap();

            let action: SyncFunction = rune::from_value(std::mem::take(
                s.get_mut("action")
                    .ok_or_else(|| VmError::panic("failed to get key"))
                    .unwrap(),
            ))
            .unwrap();

            let h = action.type_hash();

            VmResult::Ok(Self { name, action: h })
        } else {
            todo!();
        }
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
    let mut context = rune_modules::default_context()?;

    let mut module = Module::new();
    // module.ty::<Node>()?;
    context.install(module)?;

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

    let res = vm
        .try_clone()?
        .send_execute(["mk_nodes"], ())?
        .async_complete()
        .await
        .unwrap();

    let nodes: Vec<Node> = rune::from_value(res)?;
    debug!(?nodes);

    for node in nodes {
        debug!(?node);

        let exec = vm.try_clone()?.send_execute(node.action, ())?;
        let res = exec.async_complete().await.unwrap();
        let subnodes: Vec<Node> = rune::from_value(res)?;

        for subnode in subnodes {
            let exec = vm.try_clone()?.send_execute(subnode.action, ())?;
            let res = exec.async_complete().await.unwrap();
        }

        // let f = node.action.take()?.into_sync().unwrap();
        // let exec = vm.try_clone()?.send_execute(f.type_hash(), ())?;
        // let hash = tokio::spawn(async {
        //     let res = exec.async_complete().await.unwrap();
        //     let f = Function::from_value(res).unwrap();
        //     let hash = f.type_hash();
        //     hash
        // })
        // .await
        // .unwrap();

        // let exec2 = vm.try_clone()?.send_execute(hash, ())?;
        // exec2.async_complete().await.unwrap();
    }

    // tokio::time::sleep(Duration::from_secs(3)).await;

    Ok(())
}

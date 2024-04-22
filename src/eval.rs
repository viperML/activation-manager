use core::fmt::{Debug, Formatter};
use rune::{
    alloc::clone::TryClone,
    runtime::{Function, Shared, SyncFunction},
    termcolor::{ColorChoice, StandardStream},
    Any, Diagnostics, Module, Source, Sources, Value, Vm,
};
use std::{any::type_name, borrow::Borrow, path::Path, sync::Arc, time::Duration};
use tokio::runtime::Runtime;
use tracing::debug;

use crate::Result;

#[derive(Debug, Any)]
#[rune(constructor)]
pub struct Node {
    #[rune(get, set)]
    name: String,
    #[rune(get, set)]
    before: Vec<String>,
    #[rune(get, set)]
    after: Vec<String>,
    #[rune(get, set)]
    action: Shared<Function>,
}

pub async fn eval<P: AsRef<Path>>(manifest: P) -> Result<()> {
    let mut context = rune_modules::default_context()?;

    let mut module = Module::new();
    module.ty::<Node>()?;
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
        let f = node.action.take()?.into_sync().unwrap();
        let exec = vm.try_clone()?.send_execute(f.type_hash(), ())?;
        let res = tokio::spawn(async {
            let res = exec.async_complete().await.unwrap();
            println!("done");
        });
    }

    tokio::time::sleep(Duration::from_secs(3)).await;

    Ok(())
}

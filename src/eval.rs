use rune::{
    termcolor::{ColorChoice, StandardStream}, Diagnostics, Source, Sources, Vm
};
use tracing::debug;
use std::{path::Path, sync::Arc};


use crate::Result;

pub async fn eval<P: AsRef<Path>>(manifest: P) -> Result<()> {
    let manifest = manifest.as_ref();

    let context = rune_modules::default_context()?;
    let runtime = Arc::new(context.runtime()?);

    // let mut sources = rune::sources! {
    //     entry => {
    //         async fn main(timeout) {
    //             time::delay_for(time::Duration::from_secs(timeout)).await
    //         }
    //     }
    // };
    let source = Source::from_path(manifest)?;
    let mut sources = Sources::new();
    sources.insert(source)?;

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
    let res = vm.send_execute(["mk_nodes"], ())?.async_complete().await.unwrap();

    debug!(?res);

    Ok(())
}

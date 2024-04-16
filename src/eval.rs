use rune::{
    termcolor::{ColorChoice, StandardStream},
    Diagnostics, Vm,
};
use std::{path::Path, sync::Arc};

use crate::Result;

pub async fn eval<P: AsRef<Path>>(manifest: P) -> Result<()> {
    let manifest = manifest.as_ref();

    let context = rune_modules::default_context()?;
    let runtime = Arc::new(context.runtime()?);

    let mut sources = rune::sources! {
        entry => {
            async fn main(timeout) {
                time::delay_for(time::Duration::from_secs(timeout)).await
            }
        }
    };

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

    Ok(())
}

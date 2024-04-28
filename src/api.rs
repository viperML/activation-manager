use rune::Module;

pub(crate) fn module() -> eyre::Result<Module> {
    let mut m = Module::new();

    m.function_meta(debug)?;

    Ok(m)
}

#[rune::function]
fn debug(message: String) {
    tracing::debug!("{}", message);
}

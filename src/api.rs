use rune::{
    runtime::{Function, Shared, VmResult},
    vm_try, Any, Module, Value,
};

// Only exposed locally
#[derive(Debug, Any)]
#[rune(constructor)]
pub struct Node {
    #[rune(get, set)]
    pub name: String,
    // before: Vec<String>,
    #[rune(get, set)]
    pub after: Vec<String>,
    #[rune(get, set)]
    pub before: Vec<String>,
    #[rune(get, set)]
    pub action: Value,
}

pub(crate) fn module() -> eyre::Result<Module> {
    let mut m = Module::new();

    m.ty::<Node>()?;

    m.function_meta(debug)?;

    Ok(m)
}

#[rune::function]
fn debug(message: String) {
    tracing::debug!("{}", message);
}

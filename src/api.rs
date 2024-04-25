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
    // after: Vec<String>,
    #[rune(get, set)]
    pub action: Value,
}

pub(crate) fn module() -> eyre::Result<Module> {
    let mut module = Module::new();

    module.ty::<Node>()?;

    Ok(module)
}

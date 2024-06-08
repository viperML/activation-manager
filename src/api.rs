use rune::{
    runtime::{VmError, VmResult},
    vm_try, Module,
};

pub(crate) fn module() -> eyre::Result<Module> {
    let mut m = Module::new();

    m.function_meta(debug)?;
    m.function_meta(run)?;

    Ok(m)
}

#[rune::function]
fn debug(message: String) {
    tracing::debug!("{}", message);
}

#[rune::function]
async fn run(command: String) -> VmResult<()> {
    let mut args = command.split_whitespace();
    let argv0 = vm_try!(args.next().ok_or_else(|| VmError::panic("FIXME")));

    let mut cmd = tokio::process::Command::new(argv0);
    cmd.args(args);

    let mut child = vm_try!(cmd.spawn().map_err(|_| VmError::panic("FIXME")));

    let res = vm_try!(child.wait().await.map_err(|_| VmError::panic("FIXME")));

    match res.success() {
        true => VmResult::Ok(()),
        false => VmResult::panic("FIXME"),
    }
}

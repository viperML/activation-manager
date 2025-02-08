#![allow(dead_code)]

mod node;
mod api;
mod exec;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    api::main()?;

    Ok(())
}

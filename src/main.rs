mod node;
mod api;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    api::main()?;

    Ok(())
}

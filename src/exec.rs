use color_eyre::owo_colors::OwoColorize;
use tracing::trace;

use crate::node::Node;

pub fn run_graph(nodes: &Vec<Node>, dry: bool) -> eyre::Result<()> {
    for node in nodes {
        print!("{} Activating: ", ">".green());
        if let Some(desc) = &node.description {
            println!("{}", desc.bright_black());
        } 

        if !dry {
            node.kind.exec()?;
        }
    }

    Ok(())
}

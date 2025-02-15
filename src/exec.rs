use color_eyre::owo_colors::OwoColorize;
use tracing::warn;

use crate::node::Node;

pub fn run_graph(nodes: &Vec<Node>, dry: bool) -> eyre::Result<()> {
    for node in nodes {
        print!("{} Activating: ", ">".green());
        if let Some(desc) = &node.description {
            println!("{}", desc.bright_black());
        }

        if !dry {
            let result = node.kind.exec();

            if let Err(report) = result {
                warn!("{:#}", report);
            }
        }
    }

    Ok(())
}

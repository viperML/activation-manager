use tracing::trace;

use crate::node::Node;

pub fn run_graph(nodes: &Vec<Node>) -> eyre::Result<()> {
    for node in nodes {
        trace!("Running node {}", node.id);
        node.kind.exec()?;
    }

    Ok(())
}

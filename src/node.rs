use std::fmt;

use mlua::prelude::*;
use mlua::Table;

#[derive(Debug)]
pub struct Node {
    before: Vec<String>,
    after: Vec<String>,
    kind: Box<dyn NodeExec>,
}

pub trait NodeExec: fmt::Debug {
    fn exec(&self);
}

impl Default for Node {
    fn default() -> Self {
        Self {
            kind: Box::new(NilNode),
            before: Default::default(),
            after: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct NilNode;
impl NodeExec for NilNode {
    fn exec(&self) {}
}

#[derive(Debug)]
pub struct File {
    from: String,
    to: String,
}

impl NodeExec for File {
    fn exec(&self) {
        println!("Executing file");
    }
}

pub fn file_from_lua(table: Table) -> eyre::Result<Node> {
    let from: String = table.get("from")?;
    let to: String = table.get("to")?;

    let kind = File { from, to };

    let node = Node {
        kind: Box::new(kind),
        ..Default::default()
    };

    Ok(node)
}

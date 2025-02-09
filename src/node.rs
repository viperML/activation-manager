use std::fmt;

use mlua::prelude::*;
use mlua::Table;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub before: Vec<String>,
    pub after: Vec<String>,
    pub kind: Box<dyn NodeExec>,
}

pub trait NodeExec: fmt::Debug {
    fn exec(&self);
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

pub fn file_from_lua(table: Table) -> LuaResult<Node> {
    let from: String = table.get("from")?;
    let to: String = table.get("to")?;

    let kind = File { from, to };

    let mut hasher = Sha256::new();
    hasher.update(&kind.from);
    hasher.update(&kind.to);

    let node = Node {
        id: base16::encode_lower(&hasher.finalize()),
        kind: Box::new(kind),
        after: vec![],
        before: vec![],
    };

    Ok(node)
}

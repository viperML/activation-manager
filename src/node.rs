use std::fmt;
use std::fs;
use std::os::unix;
use std::path::PathBuf;

use eyre::ContextCompat;
use mlua::prelude::*;
use mlua::Table;
use sha2::{Digest, Sha256};
use tracing::debug;
use tracing::trace;

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub before: Vec<String>,
    pub after: Vec<String>,
    pub kind: Box<dyn NodeExec>,
    pub description: Option<String>,
}

pub trait NodeExec: fmt::Debug {
    fn exec(&self) -> eyre::Result<()>;
}

#[derive(Debug)]
pub struct File {
    link: String,
    target: String,
}

impl NodeExec for File {
    fn exec(&self) -> eyre::Result<()> {
        if let Ok(meta) = fs::symlink_metadata(&self.link) {
            if meta.is_symlink() {
                fs::remove_file(&self.link)?;
            }
        }
        let link = PathBuf::from(&self.link);
        let parent = link.parent().wrap_err("Failed to get parent")?;
        fs::create_dir_all(parent)?;
        unix::fs::symlink(&self.target, &self.link)?;
        Ok(())
    }
}

pub fn file_from_lua(table: Table) -> LuaResult<Node> {
    let link: String = table.get("link")?;
    let target: String = table.get("target")?;
    let id: Option<String> = table.get("id").ok();

    let kind = File { link, target };

    let id = match id {
        Some(x) => x,
        None => {
            let mut hasher = Sha256::new();
            hasher.update(&kind.link);
            hasher.update(&kind.target);
            let hash = base16::encode_lower(&hasher.finalize());
            format!("node-{hash}")
        }
    };

    let description = Some(format!("Symlink {} -> {}", kind.link, kind.target));

    let node = Node {
        id,
        kind: Box::new(kind),
        after: vec![],
        before: vec![],
        description,
    };

    Ok(node)
}

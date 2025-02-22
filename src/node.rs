use std::fmt;
use std::fs;
use std::os::unix;
use std::path::PathBuf;
use std::sync::atomic::AtomicU32;
use std::sync::Mutex;

use eyre::ContextCompat;
use mlua::prelude::*;
use mlua::Table;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use sha2::{Digest, Sha256};
use tracing::debug;
use tracing::trace;

#[derive(Debug)]
pub struct Node {
    pub meta: NodeMetadata,
    pub kind: Box<dyn NodeExec>,
}

#[derive(Debug)]
pub struct NodeMetadata {
    pub id: String,
    pub before: Vec<String>,
    pub after: Vec<String>,
    pub description: Option<String>,
}

pub trait NodeExec: fmt::Debug {
    fn exec(&self) -> eyre::Result<()>;
}

#[derive(Debug)]
pub struct FileNodeKind {
    link: String,
    target: String,
}

impl NodeExec for FileNodeKind {
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

#[derive(Debug)]
pub struct IdGenerator(Mutex<u64>);

static ID_GENERATOR: Lazy<IdGenerator> = Lazy::new(|| {
    IdGenerator(Mutex::new(0))
});

impl IdGenerator {
    fn get_next(&self) -> String {
        let mut x = self.0.lock().unwrap();
        *x = *x + 1;
        return format!("node-internal-{}", *x);
    }
}



impl NodeMetadata {
    pub fn from_table(table: &Table) -> Self {
        let before = table.get("before").unwrap_or_default();
        let after = table.get("after").unwrap_or_default();
        let description = table.get("description").unwrap_or_default();

        Self {
            id: ID_GENERATOR.get_next(),
            before,
            after,
            description,
        }
    }
}

pub fn file_from_lua(table: Table) -> LuaResult<Node> {
    let link: String = table.get("link")?;
    let target: String = table.get("target")?;
    let id: Option<String> = table.get("id").ok();
    let mut meta = NodeMetadata::from_table(&table);
    let copy: bool = table.get("copy").ok().unwrap_or(false);

    let kind = FileNodeKind { link, target };

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

    meta.description = meta
        .description
        .or_else(|| Some(format!("Symlink {} -> {}", kind.link, kind.target)));

    let node = Node {
        kind: Box::new(kind),
        meta,
    };

    Ok(node)
}

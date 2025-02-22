use std::process::Stdio;

use eyre::bail;

use crate::node::{Node, NodeExec, NodeMetadata};

#[derive(Debug)]
pub struct ExecNode {
    command: Vec<String>,
}

impl NodeExec for ExecNode {
    fn exec(&self) -> eyre::Result<()> {
        let mut _command = self.command.clone().into_iter();
        let out = std::process::Command::new(_command.next().unwrap())
            .args(_command)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;

        if !out.status.success() {
            bail!("Failed to execute command!");
        }

        //
        Ok(())
    }
}

pub(crate) fn lua_to_exec(input: mlua::Table) -> mlua::Result<Node> {
    let command: Vec<String> = input.get("command")?;
    let mut meta = NodeMetadata::from_table(&input);

    let kind = ExecNode { command };
    meta.description = meta.description.or_else(|| Some(kind.command.join(" ")));

    Ok(Node {
        meta,
        kind: Box::new(kind),
    })
}

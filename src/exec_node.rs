use std::process::Stdio;

use eyre::bail;

use crate::node::{before_after, Node, NodeExec};

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
    let (before, after) = before_after(&input);

    let kind = ExecNode { command };
    let description = kind.command.join(" ");

    Ok(Node {
        before,
        after,
        description: Some(description),
        kind: Box::new(kind),
    })
}

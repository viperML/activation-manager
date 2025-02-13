use eyre::bail;
use mlua::Table;
use tracing::trace;

use crate::node::{before_after, Node, NodeExec};

#[derive(Debug)]
pub struct DconfNode {
    pub key: String,
    pub value: String,
}

impl NodeExec for DconfNode {
    #[tracing::instrument(level = "trace")]
    fn exec(&self) -> eyre::Result<()> {
        let mut schema = self
            .key
            .strip_prefix("/")
            .unwrap()
            .split("/")
            .collect::<Vec<_>>();

        let key = schema.pop().unwrap();

        trace!(?schema, ?key);

        let out = std::process::Command::new("gsettings")
            .arg("set")
            .arg(schema.join("."))
            .arg(key)
            .arg(&self.value)
            .output()?;

        if !out.status.success() {
            bail!("Command failed");
        }

        trace!(?out);

        Ok(())
    }
}

pub fn dconf_node(input: Table) -> mlua::Result<Node> {
    let (before, after) = before_after(&input);
    let key: String = input.get("key")?;
    let value: String = input.get("value")?;

    let kind = DconfNode { key, value };

    let description = Some(format!("{} ⇒  {}", kind.key, kind.value));

    Ok(Node {
        id: format!("FIXME"),
        before,
        after,
        kind: Box::new(kind),
        description,
    })
}

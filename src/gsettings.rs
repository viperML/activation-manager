use std::process::Command;

use eyre::bail;
use mlua::Table;
use once_cell::sync::Lazy;
use tracing::trace;

use crate::node::{before_after, Node, NodeExec};

#[derive(Debug)]
pub struct GsettingsNode {
    pub schema: Vec<String>,
    pub key: String,
    pub value: String,
}

static SCHEMAS: Lazy<eyre::Result<()>> = Lazy::new(|| {
    let out = Command::new("gsettings")
        .args(["list-schemas", "--print-paths"])
        .output()?;
    if !out.status.success() {
        bail!("Command failed");
    }

    let stdout = String::from_utf8(out.stdout)?;

    for line in stdout.lines() {
        let (key, path) = line.split_once(" ").unwrap();
        trace!(?key, ?path);
    }

    Ok(())
});

impl NodeExec for GsettingsNode {
    #[tracing::instrument(level = "trace")]
    fn exec(&self) -> eyre::Result<()> {
        match Lazy::<_>::force(&SCHEMAS) {
            Ok(_) => {},
            Err(err) => bail!(err),
        }

        let out = std::process::Command::new("gsettings")
            .arg("set")
            .arg(self.schema.join("."))
            .arg(&self.key)
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
    let dconf_key: String = input.get("key")?;
    let value: String = input.get("value")?;

    let mut schema = vec![];
    for elem in dconf_key.strip_prefix("/").unwrap().split("/") {
        schema.push(elem.to_string());
    }
    let key = schema.pop().unwrap();

    let kind = GsettingsNode { key, schema, value };

    let description = Some(format!("{dconf_key} => {}", kind.value));

    Ok(Node {
        // id: format!("FIXME"),
        before,
        after,
        kind: Box::new(kind),
        description,
    })
}

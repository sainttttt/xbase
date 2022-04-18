use anyhow::Result;

/// Rename file + class
#[derive(Debug)]
pub struct RenameFile {
    pub path: String,
    pub name: String,
    pub new_name: String,
}

impl RenameFile {
    pub const KEY: &'static str = "rename_file";
    pub fn new(args: Vec<&str>) -> Result<Self> {
        let path = args.get(0);
        let new_name = args.get(2);
        let name = args.get(1);
        if path.is_none() || name.is_none() || new_name.is_none() {
            anyhow::bail!(
                "Missing arugments: [path: {:?}, old_name: {:?}, name_name: {:?} ]",
                path,
                name,
                new_name
            )
        }

        Ok(Self {
            path: path.unwrap().to_string(),
            name: name.unwrap().to_string(),
            new_name: new_name.unwrap().to_string(),
        })
    }

    pub fn request(path: &str, name: &str, new_name: &str) -> Result<()> {
        crate::Daemon::execute(&[Self::KEY, path, name, new_name])
    }

    #[cfg(feature = "lua")]
    pub fn lua(
        lua: &mlua::Lua,
        (path, name, new_name): (String, String, String),
    ) -> mlua::Result<()> {
        use crate::LuaExtension;
        lua.trace(&format!("Rename command called"))?;
        Self::request(&path, &name, &new_name).map_err(mlua::Error::external)
    }
}

#[async_trait::async_trait]
#[cfg(feature = "daemon")]
impl crate::DaemonCommandExt for RenameFile {
    async fn handle(&self, _state: crate::SharedState) -> Result<()> {
        tracing::info!("Reanmed command");
        Ok(())
    }
}

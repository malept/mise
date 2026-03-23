use indexmap::IndexMap;
use std::path::PathBuf;

use mlua::{FromLua, IntoLua, Lua, LuaSerdeExt, Value, prelude::LuaError};

use crate::{Plugin, error::Result};

/// Lockfile asset metadata passed to the `BackendInstall` Lua hook.
///
/// Verification of checksum and size is the plugin's responsibility.
#[derive(Debug, Default)]
pub struct BackendInstallAsset {
    pub url: Option<String>,
    pub checksum: Option<String>,
    pub size: Option<u64>,
    /// Local file path when mise downloaded and verified the file (attestation declared).
    /// `None` when the plugin handles its own download.
    pub file: Option<PathBuf>,
}

#[derive(Debug)]
pub struct BackendInstallContext {
    pub tool: String,
    pub version: String,
    pub install_path: PathBuf,
    pub download_path: PathBuf,
    pub options: IndexMap<String, String>,
    pub asset: BackendInstallAsset,
}

#[derive(Debug)]
pub struct BackendInstallResponse {}

impl Plugin {
    pub async fn backend_install(
        &self,
        ctx: BackendInstallContext,
    ) -> Result<BackendInstallResponse> {
        debug!("[vfox:{}] backend_install", &self.name);
        self.eval_async(chunk! {
            require "hooks/backend_install"
            return PLUGIN:BackendInstall($ctx)
        })
        .await
    }
}

impl IntoLua for BackendInstallContext {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<Value> {
        let table = lua.create_table()?;
        table.set("tool", self.tool)?;
        table.set("version", self.version)?;
        table.set(
            "install_path",
            self.install_path.to_string_lossy().to_string(),
        )?;
        table.set(
            "download_path",
            self.download_path.to_string_lossy().to_string(),
        )?;
        table.set("options", lua.to_value(&self.options)?)?;
        table.set("asset", self.asset)?;
        Ok(Value::Table(table))
    }
}

impl IntoLua for BackendInstallAsset {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<Value> {
        let table = lua.create_table()?;
        if let Some(url) = self.url {
            table.set("url", url)?;
        }
        if let Some(checksum) = self.checksum {
            table.set("checksum", checksum)?;
        }
        if let Some(size) = self.size {
            table.set("size", size)?;
        }
        if let Some(file) = self.file {
            table.set("file", file.to_string_lossy().to_string())?;
        }
        Ok(Value::Table(table))
    }
}

impl FromLua for BackendInstallResponse {
    fn from_lua(value: Value, _: &Lua) -> std::result::Result<Self, LuaError> {
        match value {
            Value::Table(_) => Ok(BackendInstallResponse {}),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "BackendInstallResponse".to_string(),
                message: Some("Expected table".to_string()),
            }),
        }
    }
}

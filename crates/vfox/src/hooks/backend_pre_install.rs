use indexmap::IndexMap;

use mlua::prelude::LuaError;
use mlua::{FromLua, IntoLua, Lua, LuaSerdeExt, Value};

use crate::Plugin;
use crate::error::Result;
use crate::runtime::Runtime;

#[derive(Debug, Clone)]
pub struct BackendPreInstallContext {
    pub tool: String,
    pub version: String,
    pub options: IndexMap<String, String>,
}

#[derive(Debug, Default)]
pub struct BackendPreInstallResponse {
    pub url: Option<String>,
    pub sha256: Option<String>,
    pub sha512: Option<String>,
    pub sha1: Option<String>,
    pub md5: Option<String>,
}

impl Plugin {
    pub async fn backend_pre_install(
        &self,
        ctx: BackendPreInstallContext,
    ) -> Result<BackendPreInstallResponse> {
        debug!("[vfox:{}] backend_pre_install", &self.name);
        self.eval_async(chunk! {
            require "hooks/backend_pre_install"
            return PLUGIN:BackendPreInstall($ctx)
        })
        .await
    }

    pub async fn backend_pre_install_for_platform(
        &self,
        ctx: BackendPreInstallContext,
        os: &str,
        arch: &str,
    ) -> Result<BackendPreInstallResponse> {
        debug!(
            "[vfox:{}] backend_pre_install_for_platform os={} arch={}",
            &self.name, os, arch
        );
        let target_os = os.to_string();
        let target_arch = arch.to_string();
        let target_runtime = Runtime::with_platform(self.dir.clone(), os, arch);
        self.eval_async(chunk! {
            require "hooks/backend_pre_install"
            -- Override globals with target platform for cross-platform URL generation
            local saved_os = OS_TYPE
            local saved_arch = ARCH_TYPE
            local saved_runtime = RUNTIME
            OS_TYPE = $target_os
            ARCH_TYPE = $target_arch
            RUNTIME = $target_runtime
            local result = PLUGIN:BackendPreInstall($ctx)
            -- Restore original values
            OS_TYPE = saved_os
            ARCH_TYPE = saved_arch
            RUNTIME = saved_runtime
            return result
        })
        .await
    }
}

impl IntoLua for BackendPreInstallContext {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<Value> {
        let table = lua.create_table()?;
        table.set("tool", self.tool)?;
        table.set("version", self.version)?;
        table.set("options", lua.to_value(&self.options)?)?;
        Ok(Value::Table(table))
    }
}

impl FromLua for BackendPreInstallResponse {
    fn from_lua(value: Value, _: &Lua) -> std::result::Result<Self, LuaError> {
        match value {
            Value::Table(table) => {
                let get_optional_string =
                    |key: &str| -> std::result::Result<Option<String>, LuaError> {
                        match table.get::<Value>(key)? {
                            Value::Nil => Ok(None),
                            Value::String(s) => Ok(Some(s.to_str()?.to_string())),
                            other => Err(LuaError::FromLuaConversionError {
                                from: other.type_name(),
                                to: "Option<String>".to_string(),
                                message: Some(format!("Expected string or nil for {key}")),
                            }),
                        }
                    };
                Ok(BackendPreInstallResponse {
                    url: get_optional_string("url")?,
                    sha256: get_optional_string("sha256")?,
                    sha512: get_optional_string("sha512")?,
                    sha1: get_optional_string("sha1")?,
                    md5: get_optional_string("md5")?,
                })
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "BackendPreInstallResponse".to_string(),
                message: Some("Expected table".to_string()),
            }),
        }
    }
}

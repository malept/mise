use mlua::{ExternalResult, Lua, MultiValue, Table};
use std::path::Path;

pub fn mod_sigstore(lua: &Lua) -> mlua::Result<()> {
    let package: Table = lua.globals().get("package")?;
    let loaded: Table = package.get("loaded")?;
    loaded.set(
        "sigstore",
        lua.create_table_from(vec![(
            "verify_github_attestation",
            lua.create_async_function(|_lua: Lua, input| async move {
                verify_github_attestation(&_lua, input).await
            })?,
        )])?,
    )
}

/// Retrieves the GitHub API token from either `MISE_GITHUB_TOKEN` or `GITHUB_TOKEN`
/// environment variables.
fn github_token() -> Result<String, std::env::VarError> {
    std::env::var("MISE_GITHUB_TOKEN").or_else(|_| std::env::var("GITHUB_TOKEN"))
}

async fn verify_github_attestation(_lua: &Lua, input: MultiValue) -> mlua::Result<bool> {
    let token = github_token().into_lua_err()?;
    let args: Vec<String> = input
        .into_iter()
        .map(|v| v.to_string())
        .collect::<mlua::Result<_>>()?;
    let artifact_path = Path::new(&args[0]);
    let owner = &args[1];
    let repo = &args[2];
    let signer_workflow = if args.len() > 3 {
        Some(args[3].as_str())
    } else {
        None
    };
    sigstore_verification::verify_github_attestation(
        artifact_path,
        owner,
        repo,
        Some(token.as_str()),
        signer_workflow,
    )
    .await
    .into_lua_err()
}

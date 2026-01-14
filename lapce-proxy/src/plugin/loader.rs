// Stub module for removed WASI plugin runtime.
// WASM plugin loading has been disabled in this build.
// Only static/embedded LSP servers are supported.

use std::{
    collections::HashMap,
    fs,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow};
use lapce_core::directory::Directory;
use lapce_rpc::plugin::{VoltID, VoltInfo, VoltMetadata};

use super::PluginCatalogRpcHandler;

/// Stub - WASM plugin loading disabled.
/// Only loads volt metadata for themes, not WASM execution.
pub fn load_all_volts(
    plugin_rpc: PluginCatalogRpcHandler,
    extra_plugin_paths: &[PathBuf],
    disabled_volts: Vec<VoltID>,
) {
    // Load volts for themes only, not WASM execution
    let all_volts = find_all_volts(extra_plugin_paths);
    let volts: Vec<VoltMetadata> = all_volts
        .into_iter()
        .filter_map(|meta| {
            // Skip WASM plugins - only keep theme-only plugins
            if meta.wasm.is_some() {
                // Still notify about installed plugins for UI
                let icon = super::volt_icon(&meta);
                plugin_rpc.core_rpc.volt_installed(meta.clone(), icon);
                // But don't activate WASM plugins
                return None;
            }
            // Non-WASM plugins (themes) can be loaded
            let icon = super::volt_icon(&meta);
            plugin_rpc.core_rpc.volt_installed(meta.clone(), icon);
            if disabled_volts.contains(&meta.id()) {
                return None;
            }
            Some(meta)
        })
        .collect();
    if let Err(err) = plugin_rpc.unactivated_volts(volts) {
        tracing::error!("{:?}", err);
    }
}

/// Find all installed volts (metadata only, no WASM loading).
pub fn find_all_volts(extra_plugin_paths: &[PathBuf]) -> Vec<VoltMetadata> {
    let Some(plugin_dir) = Directory::plugins_directory() else {
        return Vec::new();
    };

    let mut plugins: Vec<VoltMetadata> = plugin_dir
        .read_dir()
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|result| {
            let entry = result.ok()?;
            let metadata = entry.metadata().ok()?;
            if metadata.is_file() || entry.file_name().to_str()?.starts_with('.') {
                return None;
            }
            Some(entry.path())
        })
        .filter_map(|path| match load_volt(&path) {
            Ok(metadata) => Some(metadata),
            Err(e) => {
                tracing::error!("Failed to load plugin: {:?}", e);
                None
            }
        })
        .collect();

    for plugin_path in extra_plugin_paths {
        let mut metadata = match load_volt(plugin_path) {
            Ok(metadata) => metadata,
            Err(e) => {
                tracing::error!("Failed to load extra plugin: {:?}", e);
                continue;
            }
        };

        let pos = plugins.iter().position(|meta| {
            meta.name == metadata.name && meta.author == metadata.author
        });

        if let Some(pos) = pos {
            std::mem::swap(&mut plugins[pos], &mut metadata);
        } else {
            plugins.push(metadata);
        }
    }

    plugins
}

/// Load volt metadata from volt.toml file.
pub fn load_volt(path: &Path) -> Result<VoltMetadata> {
    let path = path.canonicalize()?;
    let mut file = fs::File::open(path.join("volt.toml"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut meta: VoltMetadata = toml::from_str(&contents)?;

    meta.dir = Some(path.clone());
    meta.wasm = meta.wasm.as_ref().and_then(|wasm| {
        Some(path.join(wasm).canonicalize().ok()?.to_str()?.to_string())
    });
    meta.color_themes = meta.color_themes.as_ref().map(|themes| {
        themes
            .iter()
            .filter_map(|theme| {
                Some(path.join(theme).canonicalize().ok()?.to_str()?.to_string())
            })
            .collect()
    });
    meta.icon_themes = meta.icon_themes.as_ref().map(|themes| {
        themes
            .iter()
            .filter_map(|theme| {
                Some(path.join(theme).canonicalize().ok()?.to_str()?.to_string())
            })
            .collect()
    });

    Ok(meta)
}

/// Stub - WASM plugin execution disabled.
pub fn enable_volt(
    plugin_rpc: PluginCatalogRpcHandler,
    volt: VoltInfo,
) -> Result<()> {
    let path = Directory::plugins_directory()
        .ok_or_else(|| anyhow!("can't get plugin directory"))?
        .join(volt.id().to_string());
    let meta = load_volt(&path)?;

    // Only enable non-WASM plugins (themes)
    if meta.wasm.is_some() {
        return Err(anyhow!("WASM plugin execution disabled"));
    }

    plugin_rpc.unactivated_volts(vec![meta])?;
    Ok(())
}

/// Stub - WASM plugin execution disabled.
pub fn start_volt(
    _workspace: Option<PathBuf>,
    _configurations: Option<HashMap<String, serde_json::Value>>,
    _plugin_rpc: PluginCatalogRpcHandler,
    meta: VoltMetadata,
) -> Result<()> {
    // WASM execution disabled - log and return error
    tracing::warn!(
        "WASM plugin execution disabled. Plugin '{}' by '{}' cannot be started.",
        meta.name,
        meta.author
    );
    Err(anyhow!("WASM plugin execution disabled in this build"))
}

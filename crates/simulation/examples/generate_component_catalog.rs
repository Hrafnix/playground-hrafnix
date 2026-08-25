//! Generates the checked-in component catalog from installed registry metadata.

use simulation::builtins::register_signal_builtins;
use simulation::catalog_docs::registry_markdown;
use simulation::registry::ComponentRegistry;
use std::path::PathBuf;

/// Writes deterministic component documentation to an optional output path.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os()
        .nth(1)
        .map_or_else(|| PathBuf::from("COMPONENT_CATALOG.md"), PathBuf::from);
    let mut registry = ComponentRegistry::new();
    register_signal_builtins(&mut registry)
        .map_err(|error| std::io::Error::other(format!("{error:?}")))?;
    std::fs::write(path, registry_markdown(&registry))?;
    Ok(())
}

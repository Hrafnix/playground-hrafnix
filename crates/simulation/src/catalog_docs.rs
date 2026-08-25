//! Deterministic component documentation generated from registry metadata.

use crate::component::{ComponentDefinition, PortDirection};
use crate::registry::ComponentRegistry;
use std::fmt::Write;

/// Renders the installed component catalog as stable Markdown.
#[must_use]
pub fn registry_markdown(registry: &ComponentRegistry) -> String {
    let mut output = String::from("# Component Catalog\n\n");
    for definition in registry.iter() {
        write_definition(&mut output, definition);
    }
    output
}

/// Appends one registry definition to generated Markdown.
fn write_definition(output: &mut String, definition: &ComponentDefinition) {
    let _ = writeln!(
        output,
        "## {}\n\n`{}` | {} | {}.{}.{}\n\n{}\n",
        definition.display_name,
        definition.type_id,
        definition.category,
        definition.version.major,
        definition.version.minor,
        definition.version.patch,
        definition.documentation,
    );
    if !definition.parameters.is_empty() {
        output.push_str(
            "### Parameters\n\n| Key | Type | Default | Description |\n|---|---|---|---|\n",
        );
        for parameter in &definition.parameters {
            let _ = writeln!(
                output,
                "| `{}` | `{:?}` | `{}` | {} |",
                parameter.key,
                parameter.value_type,
                parameter.default_expression,
                parameter.description,
            );
        }
        output.push('\n');
    }
    output.push_str("### Ports\n\n| Key | Direction | Type | Description |\n|---|---|---|---|\n");
    for port in &definition.ports {
        let direction = match port.direction {
            PortDirection::Input => "input",
            PortDirection::Output => "output",
        };
        let _ = writeln!(
            output,
            "| `{}` | {} | `{:?}` | {} |",
            port.key, direction, port.value_type, port.description,
        );
    }
    output.push('\n');
}

#[cfg(test)]
mod tests {
    use super::registry_markdown;
    use crate::builtins::register_signal_builtins;
    use crate::registry::ComponentRegistry;

    #[test]
    fn every_registered_component_has_complete_generated_documentation() {
        let mut registry = ComponentRegistry::new();
        register_signal_builtins(&mut registry).unwrap();

        for definition in registry.iter() {
            assert!(!definition.documentation.as_str().is_empty());
            assert!(!definition.category.as_str().is_empty());
            assert!(
                definition
                    .parameters
                    .iter()
                    .all(|parameter| !parameter.description.as_str().is_empty())
            );
            assert!(
                definition
                    .ports
                    .iter()
                    .all(|port| !port.description.as_str().is_empty())
            );
        }

        let markdown = registry_markdown(&registry);
        assert!(markdown.starts_with("# Component Catalog\n"));
        assert_eq!(markdown.matches("\n## ").count(), 20);
        assert!(markdown.contains("`signal.assertion`"));
        assert!(markdown.contains("`signal.lookup`"));
        assert_eq!(markdown, include_str!("../COMPONENT_CATALOG.md"));
    }
}

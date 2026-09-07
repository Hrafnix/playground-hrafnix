//! Command-line interface definitions for Hrafnix.

use clap::{
    Arg, ArgAction, Command, CommandFactory, Parser, Subcommand,
    error::{ContextKind, ContextValue, ErrorKind},
};
use shareable_string::{SharedStringStore, SharedStringTranslationMap};
use std::env;
use translation::generate_translation_map;

/// Parses commands supplied to the Hrafnix command-line interface.
#[derive(Debug, Parser)]
pub struct Cli {
    /// The operation to perform if one was supplied.
    #[command(subcommand)]
    pub command: Option<Action>,
}

/// Languages supported by the Hrafnix command-line interface.
#[derive(Clone, Copy, Debug)]
pub enum Language {
    /// English.
    En,
    /// Chinese.
    Zh,
    /// German.
    De,
    /// Spanish.
    Es,
    /// French.
    Fr,
    /// Japanese.
    Ja,
    /// Korean.
    Ko,
}

impl Language {
    /// Returns the language's supported locale code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Zh => "zh",
            Self::De => "de",
            Self::Es => "es",
            Self::Fr => "fr",
            Self::Ja => "ja",
            Self::Ko => "ko",
        }
    }
}

/// Operations available from the Hrafnix command-line interface.
#[derive(Debug, Subcommand)]
pub enum Action {
    /// Creates a new Hrafnix project.
    New,
    /// Formats a Hrafnix project.
    Format,
    /// Simulates a Hrafnix project.
    Simulate,
}

/// Determines the help language from the process locale.
#[must_use]
pub fn selected_language() -> Language {
    ["LC_ALL", "LANGUAGE", "LANG"]
        .into_iter()
        .filter_map(|variable| env::var(variable).ok())
        .find_map(|locale| parse_locale(&locale))
        .unwrap_or(Language::En)
}

/// Builds a localized Hrafnix command-line interface from Cargo package metadata.
#[must_use]
pub fn build_command(language: Language) -> Command {
    let store = SharedStringStore::new();
    let translations = generate_translation_map(&store);

    let command = Cli::command()
        .name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .disable_help_flag(true)
        .disable_help_subcommand(true)
        .disable_version_flag(true)
        .about(translation(&translations, language, "hrafnix_about"))
        .arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .action(ArgAction::Version)
                .help(translation(&translations, language, "hrafnix_version"))
                .help_heading(translation(&translations, language, "hrafnix_options")),
        );

    localize_subcommands(
        localize_help(command, &translations, language),
        &translations,
        language,
        "hrafnix",
    )
}

/// Renders a command-line parsing error in the selected language.
#[must_use]
pub fn localized_error(error: &clap::Error, command: &mut Command, language: Language) -> String {
    let store = SharedStringStore::new();
    let translations = generate_translation_map(&store);
    let invalid_input = || translation(&translations, language, "hrafnix_error_invalid_input");
    let missing_value = |argument: &str| {
        interpolate(
            &translation(&translations, language, "hrafnix_error_value_required"),
            "argument",
            argument,
        )
    };
    let invalid_value = || {
        let argument = context_string(error, ContextKind::InvalidArg);
        let value = context_string(error, ContextKind::InvalidValue);
        match (argument, value) {
            (Some(argument), Some("")) => missing_value(argument),
            (Some(argument), Some(value)) => {
                let message = interpolate(
                    &translation(&translations, language, "hrafnix_error_invalid_value"),
                    "value",
                    value,
                );
                interpolate(&message, "argument", argument)
            }
            _ => invalid_input(),
        }
    };
    let message = match error.kind() {
        ErrorKind::InvalidSubcommand => context_string(error, ContextKind::InvalidSubcommand)
            .map_or_else(&invalid_input, |value| {
                interpolate(
                    &translation(
                        &translations,
                        language,
                        "hrafnix_error_unrecognized_subcommand",
                    ),
                    "value",
                    value,
                )
            }),
        ErrorKind::UnknownArgument => {
            context_string(error, ContextKind::InvalidArg).map_or_else(&invalid_input, |value| {
                interpolate(
                    &translation(&translations, language, "hrafnix_error_unexpected_argument"),
                    "value",
                    value,
                )
            })
        }
        ErrorKind::InvalidValue | ErrorKind::ValueValidation | ErrorKind::TooManyValues => {
            invalid_value()
        }
        ErrorKind::NoEquals | ErrorKind::TooFewValues => {
            context_string(error, ContextKind::InvalidArg)
                .map_or_else(&invalid_input, missing_value)
        }
        ErrorKind::MissingRequiredArgument => context_strings(error, ContextKind::InvalidArg)
            .map_or_else(invalid_input, |arguments| {
                missing_value(&arguments.join(", "))
            }),
        ErrorKind::WrongNumberOfValues
        | ErrorKind::ArgumentConflict
        | ErrorKind::MissingSubcommand
        | ErrorKind::InvalidUtf8 => invalid_input(),
        ErrorKind::DisplayHelp
        | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        | ErrorKind::DisplayVersion
        | ErrorKind::Io
        | ErrorKind::Format => error.to_string(),
        _ => format!("{}: {error}", invalid_input()),
    };

    let mut output = format!(
        "{}: {message}",
        translation(&translations, language, "hrafnix_error")
    );
    if let Some(suggestions) = context_strings(error, ContextKind::SuggestedSubcommand) {
        let suggestion = interpolate(
            &translation(&translations, language, "hrafnix_error_similar_subcommand"),
            "values",
            &quoted_values(suggestions),
        );
        output.push_str("\n\n  ");
        output.push_str(&translation(&translations, language, "hrafnix_tip"));
        output.push_str(": ");
        output.push_str(&suggestion);
    }
    if let Some(suggestions) = context_strings(error, ContextKind::SuggestedArg) {
        let suggestion = interpolate(
            &translation(&translations, language, "hrafnix_error_similar_argument"),
            "values",
            &quoted_values(suggestions),
        );
        output.push_str("\n\n  ");
        output.push_str(&translation(&translations, language, "hrafnix_tip"));
        output.push_str(": ");
        output.push_str(&suggestion);
    }

    let usage = command.render_usage().to_string().replacen(
        "Usage:",
        &translation(&translations, language, "hrafnix_usage"),
        1,
    );
    output.push_str("\n\n");
    output.push_str(&usage);
    output.push_str("\n\n");
    output.push_str(&translation(
        &translations,
        language,
        "hrafnix_more_information",
    ));
    output.push_str(" '--help'.\n");
    output
}

/// Retrieves a string value from clap's structured error context.
fn context_string(error: &clap::Error, kind: ContextKind) -> Option<&str> {
    match error.get(kind) {
        Some(ContextValue::String(value)) => Some(value),
        _ => None,
    }
}

/// Retrieves a collection of string values from clap's structured error context.
fn context_strings(error: &clap::Error, kind: ContextKind) -> Option<&[String]> {
    match error.get(kind) {
        Some(ContextValue::Strings(values)) => Some(values),
        _ => None,
    }
}

/// Quotes values as they appear in clap's suggestions.
fn quoted_values(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("'{value}'"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Replaces a named placeholder in a localized message.
fn interpolate(template: &str, name: &str, value: &str) -> String {
    template.replace(&format!("{{{name}}}"), value)
}

/// Localizes help shared by the root command and its subcommands.
fn localize_help(
    command: Command,
    translations: &SharedStringTranslationMap,
    language: Language,
) -> Command {
    command
        .help_template(format!(
            "{{before-help}}{{name}} {{version}}\n{{about-with-newline}}\n{} {{usage}}\n\n{{all-args}}{{after-help}}",
            translation(translations, language, "hrafnix_usage")
        ))
        .subcommand_help_heading(translation(translations, language, "hrafnix_commands"))
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .action(ArgAction::Help)
                .help(translation(translations, language, "hrafnix_help"))
                .help_heading(translation(translations, language, "hrafnix_options"))
        )
}

/// Localizes every subcommand, its arguments, and any nested subcommands.
fn localize_subcommands(
    mut command: Command,
    translations: &SharedStringTranslationMap,
    language: Language,
    parent_key: &str,
) -> Command {
    let names: Vec<String> = command
        .get_subcommands()
        .map(|subcommand| subcommand.get_name().to_owned())
        .collect();

    for name in names {
        let command_key = format!("{parent_key}_{name}");
        command = command.mut_subcommand(name, |subcommand| {
            let subcommand = subcommand.about(translation(translations, language, &command_key));
            let subcommand = localize_arguments(subcommand, translations, language, &command_key);
            let subcommand = localize_subcommands(subcommand, translations, language, &command_key);

            localize_help(subcommand, translations, language)
        });
    }

    command
}

/// Localizes arguments using keys derived from their command path and argument ID.
fn localize_arguments(
    mut command: Command,
    translations: &SharedStringTranslationMap,
    language: Language,
    command_key: &str,
) -> Command {
    let arguments: Vec<(String, bool)> = command
        .get_arguments()
        .map(|argument| {
            (
                argument.get_id().to_string(),
                argument.get_index().is_some(),
            )
        })
        .collect();

    for (name, is_positional) in arguments {
        let translation_key = format!("{command_key}_{name}");
        let heading_key = if is_positional {
            "hrafnix_arguments"
        } else {
            "hrafnix_options"
        };
        command = command.mut_arg(name, |argument| {
            argument
                .help(translation(translations, language, &translation_key))
                .help_heading(translation(translations, language, heading_key))
        });
    }

    command
}

/// Parses the language code from a locale string.
fn parse_language(locale: &str) -> Option<Language> {
    match locale.split(['.', '_', '-']).next()? {
        "en" => Some(Language::En),
        "zh" => Some(Language::Zh),
        "de" => Some(Language::De),
        "es" => Some(Language::Es),
        "fr" => Some(Language::Fr),
        "ja" => Some(Language::Ja),
        "ko" => Some(Language::Ko),
        _ => None,
    }
}

/// Selects the first supported language from a locale preference list.
fn parse_locale(locale: &str) -> Option<Language> {
    locale.split(':').find_map(parse_language)
}

/// Retrieves a localized string from the built-in translation catalog.
fn translation(translations: &SharedStringTranslationMap, language: Language, key: &str) -> String {
    match translations.get_translation(key, language.code(), None) {
        Some(value) => value.to_string(),
        None => panic!("missing built-in translation for `{key}`"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies path-based localization for arguments and nested commands.
    #[test]
    fn localizes_nested_subcommands_and_their_arguments() {
        let translations = SharedStringTranslationMap::new();
        for (key, value) in [
            ("hrafnix_usage", "Verwendung:"),
            ("hrafnix_commands", "Befehle"),
            ("hrafnix_arguments", "Argumente"),
            ("hrafnix_options", "Optionen"),
            ("hrafnix_help", "Hilfe anzeigen"),
            ("hrafnix_parent", "Übergeordneter Befehl"),
            ("hrafnix_parent_input", "Eingabe"),
            ("hrafnix_parent_child", "Unterbefehl"),
            ("hrafnix_parent_child_force", "Ausführung erzwingen"),
        ] {
            translations.set_translation(key, "de", value);
        }

        let command = Command::new("test").disable_help_flag(true).subcommand(
            Command::new("parent")
                .arg(Arg::new("input").index(1))
                .subcommand(Command::new("child").arg(Arg::new("force").long("force"))),
        );
        let command = localize_subcommands(command, &translations, Language::De, "hrafnix");
        let parent = command
            .get_subcommands()
            .find(|subcommand| subcommand.get_name() == "parent")
            .unwrap_or_else(|| panic!("missing parent subcommand"));
        let input = parent
            .get_arguments()
            .find(|argument| argument.get_id() == "input")
            .unwrap_or_else(|| panic!("missing input argument"));
        let child = parent
            .get_subcommands()
            .find(|subcommand| subcommand.get_name() == "child")
            .unwrap_or_else(|| panic!("missing child subcommand"));
        let force = child
            .get_arguments()
            .find(|argument| argument.get_id() == "force")
            .unwrap_or_else(|| panic!("missing force argument"));

        assert_eq!(
            parent.get_about().map(ToString::to_string).as_deref(),
            Some("Übergeordneter Befehl")
        );
        assert_eq!(
            input.get_help().map(ToString::to_string).as_deref(),
            Some("Eingabe")
        );
        assert_eq!(input.get_help_heading(), Some("Argumente"));
        assert_eq!(
            child.get_about().map(ToString::to_string).as_deref(),
            Some("Unterbefehl")
        );
        assert_eq!(
            force.get_help().map(ToString::to_string).as_deref(),
            Some("Ausführung erzwingen")
        );
        assert_eq!(force.get_help_heading(), Some("Optionen"));
    }
}

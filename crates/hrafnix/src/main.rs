//! Hrafnix command-line application.

use clap::{FromArgMatches, error::ErrorKind};
use hrafnix::{Action, Cli, build_command, localized_error, selected_language};
use std::io::Write;
use std::process::ExitCode;

fn main() -> ExitCode {
    let language = selected_language();
    let mut command = build_command(language);
    let matches = match command.clone().try_get_matches() {
        Ok(matches) => matches,
        Err(error)
            if matches!(
                error.kind(),
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
            ) =>
        {
            if error.print().is_err() {
                return ExitCode::FAILURE;
            }
            return ExitCode::SUCCESS;
        }
        Err(error) => {
            eprint!("{}", localized_error(&error, &mut command, language));
            return ExitCode::from(2);
        }
    };
    let cli = match Cli::from_arg_matches(&matches) {
        Ok(cli) => cli,
        Err(error) => {
            eprint!("{}", localized_error(&error, &mut command, language));
            return ExitCode::from(2);
        }
    };

    match cli.command {
        Some(Action::New | Action::Format | Action::Simulate) => {}
        None => {
            if command.print_long_help().is_err() || writeln!(std::io::stdout()).is_err() {
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}

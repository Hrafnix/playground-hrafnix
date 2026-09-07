//! Integration tests for the Hrafnix command-line interface.

use std::process::Command;

#[test]
fn accepts_each_supported_subcommand() {
    for subcommand in ["new", "format", "simulate"] {
        let output = match Command::new(env!("CARGO_BIN_EXE_hrafnix"))
            .arg(subcommand)
            .output()
        {
            Ok(output) => output,
            Err(error) => panic!("failed to run `{subcommand}`: {error}"),
        };

        assert!(
            output.status.success(),
            "`{subcommand}` failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn reports_the_package_name_and_version() {
    let output = match Command::new(env!("CARGO_BIN_EXE_hrafnix"))
        .arg("--version")
        .output()
    {
        Ok(output) => output,
        Err(error) => panic!("failed to run `--version`: {error}"),
    };

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains(&format!(
        "{} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )));
}

#[test]
fn localizes_help_for_each_supported_language() {
    for (language, expected_description) in [
        ("en", "Create, format, and simulate Hrafnix projects."),
        ("zh", "创建、格式化和模拟 Hrafnix 项目。"),
        (
            "de",
            "Hrafnix-Projekte erstellen, formatieren und simulieren.",
        ),
        ("es", "Crea, da formato y simula proyectos de Hrafnix."),
        ("fr", "Crée, formate et simule des projets Hrafnix."),
        (
            "ja",
            "Hrafnix プロジェクトを作成、整形、シミュレートします。",
        ),
        (
            "ko",
            "Hrafnix 프로젝트를 생성, 서식 지정 및 시뮬레이션합니다.",
        ),
    ] {
        let output = match Command::new(env!("CARGO_BIN_EXE_hrafnix"))
            .arg("--help")
            .env("LC_ALL", language)
            .env_remove("LANGUAGE")
            .env_remove("LANG")
            .output()
        {
            Ok(output) => output,
            Err(error) => panic!("failed to get `{language}` help: {error}"),
        };

        assert!(output.status.success(), "`{language}` help command failed");
        assert!(
            String::from_utf8_lossy(&output.stdout).contains(expected_description),
            "`{language}` help is missing its localized description"
        );
    }
}

#[test]
fn localizes_subcommand_help() {
    let output = match Command::new(env!("CARGO_BIN_EXE_hrafnix"))
        .args(["new", "--help"])
        .env("LC_ALL", "de")
        .env_remove("LANGUAGE")
        .env_remove("LANG")
        .output()
    {
        Ok(output) => output,
        Err(error) => panic!("failed to get localized subcommand help: {error}"),
    };

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Erstellt ein neues Hrafnix-Projekt"));
    assert!(String::from_utf8_lossy(&output.stdout).contains("Verwendung:"));
}

#[test]
fn localizes_invalid_subcommand_errors_for_each_supported_language() {
    for (language, expected_error, expected_tip, expected_usage) in [
        ("en", "unrecognized subcommand 'new2'", "tip:", "Usage:"),
        ("zh", "无法识别的子命令“new2”", "提示:", "用法："),
        (
            "de",
            "nicht erkannter Unterbefehl „new2“",
            "Tipp:",
            "Verwendung:",
        ),
        (
            "es",
            "subcomando no reconocido «new2»",
            "sugerencia:",
            "Uso:",
        ),
        (
            "fr",
            "sous-commande non reconnue «new2»",
            "conseil:",
            "Utilisation :",
        ),
        (
            "ja",
            "認識されないサブコマンド「new2」",
            "ヒント:",
            "使用法:",
        ),
        ("ko", "인식할 수 없는 하위 명령 'new2'", "힌트:", "사용법:"),
    ] {
        let output = match Command::new(env!("CARGO_BIN_EXE_hrafnix"))
            .arg("new2")
            .env("LC_ALL", language)
            .env_remove("LANGUAGE")
            .env_remove("LANG")
            .output()
        {
            Ok(output) => output,
            Err(error) => panic!("failed to run `{language}` invalid subcommand: {error}"),
        };

        assert_eq!(output.status.code(), Some(2));
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(expected_error),
            "`{language}` error was not localized"
        );
        assert!(
            stderr.contains(expected_tip),
            "`{language}` tip was not localized"
        );
        assert!(
            stderr.contains(expected_usage),
            "`{language}` usage was not localized"
        );
    }
}

#[test]
fn rejects_language_options() {
    for arguments in [["-l", "de"], ["--language", "de"]] {
        let output = match Command::new(env!("CARGO_BIN_EXE_hrafnix"))
            .args(arguments)
            .output()
        {
            Ok(output) => output,
            Err(error) => panic!("failed to test removed language option: {error}"),
        };

        assert_eq!(output.status.code(), Some(2));
    }
}

#[test]
fn selects_the_first_supported_language_from_language_preferences() {
    let output = match Command::new(env!("CARGO_BIN_EXE_hrafnix"))
        .arg("--help")
        .env_remove("LC_ALL")
        .env("LANGUAGE", "unknown:de:en")
        .env_remove("LANG")
        .output()
    {
        Ok(output) => output,
        Err(error) => panic!("failed to use locale preferences: {error}"),
    };

    assert!(output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("Hrafnix-Projekte erstellen, formatieren und simulieren.")
    );
}

#[test]
fn shows_localized_help_when_no_command_is_given() {
    let output = match Command::new(env!("CARGO_BIN_EXE_hrafnix"))
        .env("LC_ALL", "de")
        .env_remove("LANGUAGE")
        .env_remove("LANG")
        .output()
    {
        Ok(output) => output,
        Err(error) => panic!("failed to run without a command: {error}"),
    };

    assert!(output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("Hrafnix-Projekte erstellen, formatieren und simulieren.")
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("Befehle:"));
    assert!(String::from_utf8_lossy(&output.stdout).contains("Optionen:"));
}

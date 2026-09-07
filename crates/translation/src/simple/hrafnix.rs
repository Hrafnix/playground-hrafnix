use shareable_string::SharedStringTranslationMap;
use std::collections::HashMap;

/// Adds translations for the Hrafnix command-line interface.
pub(crate) fn add_hrafnix_translation_map(translation_map: &mut SharedStringTranslationMap) {
    translation_map.set_translation_key(
        "hrafnix_about",
        HashMap::from([
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
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_language",
        HashMap::from([
            ("en", "Language used for command-line help"),
            ("zh", "用于命令行帮助的语言"),
            ("de", "Sprache für die Befehlszeilenhilfe"),
            ("es", "Idioma usado para la ayuda de la línea de comandos"),
            ("fr", "Langue utilisée pour l’aide en ligne de commande"),
            ("ja", "コマンドラインヘルプで使用する言語"),
            ("ko", "명령줄 도움말에 사용할 언어"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_new",
        HashMap::from([
            ("en", "Creates a new Hrafnix project"),
            ("zh", "创建新的 Hrafnix 项目"),
            ("de", "Erstellt ein neues Hrafnix-Projekt"),
            ("es", "Crea un nuevo proyecto de Hrafnix"),
            ("fr", "Crée un nouveau projet Hrafnix"),
            ("ja", "新しい Hrafnix プロジェクトを作成します"),
            ("ko", "새 Hrafnix 프로젝트를 생성합니다"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_format",
        HashMap::from([
            ("en", "Formats a Hrafnix project"),
            ("zh", "格式化 Hrafnix 项目"),
            ("de", "Formatiert ein Hrafnix-Projekt"),
            ("es", "Da formato a un proyecto de Hrafnix"),
            ("fr", "Formate un projet Hrafnix"),
            ("ja", "Hrafnix プロジェクトを整形します"),
            ("ko", "Hrafnix 프로젝트의 서식을 지정합니다"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_simulate",
        HashMap::from([
            ("en", "Simulates a Hrafnix project"),
            ("zh", "模拟 Hrafnix 项目"),
            ("de", "Simuliert ein Hrafnix-Projekt"),
            ("es", "Simula un proyecto de Hrafnix"),
            ("fr", "Simule un projet Hrafnix"),
            ("ja", "Hrafnix プロジェクトをシミュレートします"),
            ("ko", "Hrafnix 프로젝트를 시뮬레이션합니다"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_usage",
        HashMap::from([
            ("en", "Usage:"),
            ("zh", "用法："),
            ("de", "Verwendung:"),
            ("es", "Uso:"),
            ("fr", "Utilisation :"),
            ("ja", "使用法:"),
            ("ko", "사용법:"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_commands",
        HashMap::from([
            ("en", "Commands"),
            ("zh", "命令"),
            ("de", "Befehle"),
            ("es", "Comandos"),
            ("fr", "Commandes"),
            ("ja", "コマンド"),
            ("ko", "명령"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_options",
        HashMap::from([
            ("en", "Options"),
            ("zh", "选项"),
            ("de", "Optionen"),
            ("es", "Opciones"),
            ("fr", "Options"),
            ("ja", "オプション"),
            ("ko", "옵션"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_arguments",
        HashMap::from([
            ("en", "Arguments"),
            ("zh", "参数"),
            ("de", "Argumente"),
            ("es", "Argumentos"),
            ("fr", "Arguments"),
            ("ja", "引数"),
            ("ko", "인수"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_help",
        HashMap::from([
            ("en", "Print help (see a summary with '-h')"),
            ("zh", "显示帮助（使用 '-h' 查看摘要）"),
            ("de", "Hilfe anzeigen (Zusammenfassung mit '-h')"),
            ("es", "Muestra la ayuda (consulta un resumen con '-h')"),
            ("fr", "Affiche l’aide (voir le résumé avec '-h')"),
            ("ja", "ヘルプを表示します（概要は '-h' で表示）"),
            ("ko", "도움말을 표시합니다('-h'로 요약 보기)"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_version",
        HashMap::from([
            ("en", "Print version"),
            ("zh", "显示版本"),
            ("de", "Version anzeigen"),
            ("es", "Muestra la versión"),
            ("fr", "Affiche la version"),
            ("ja", "バージョンを表示します"),
            ("ko", "버전을 표시합니다"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_supported_languages",
        HashMap::from([
            (
                "en",
                "Supported languages: en (English), zh (Chinese), de (German), es (Spanish), fr (French), ja (Japanese), ko (Korean)",
            ),
            (
                "zh",
                "支持的语言：en（英语）、zh（中文）、de（德语）、es（西班牙语）、fr（法语）、ja（日语）、ko（韩语）",
            ),
            (
                "de",
                "Unterstützte Sprachen: en (Englisch), zh (Chinesisch), de (Deutsch), es (Spanisch), fr (Französisch), ja (Japanisch), ko (Koreanisch)",
            ),
            (
                "es",
                "Idiomas compatibles: en (inglés), zh (chino), de (alemán), es (español), fr (francés), ja (japonés), ko (coreano)",
            ),
            (
                "fr",
                "Langues prises en charge : en (anglais), zh (chinois), de (allemand), es (espagnol), fr (français), ja (japonais), ko (coréen)",
            ),
            (
                "ja",
                "対応言語: en（英語）、zh（中国語）、de（ドイツ語）、es（スペイン語）、fr（フランス語）、ja（日本語）、ko（韓国語）",
            ),
            (
                "ko",
                "지원 언어: en(영어), zh(중국어), de(독일어), es(스페인어), fr(프랑스어), ja(일본어), ko(한국어)",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_error",
        HashMap::from([
            ("en", "error"),
            ("zh", "错误"),
            ("de", "Fehler"),
            ("es", "error"),
            ("fr", "erreur"),
            ("ja", "エラー"),
            ("ko", "오류"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_error_invalid_input",
        HashMap::from([
            ("en", "invalid command-line input"),
            ("zh", "无效的命令行输入"),
            ("de", "ungültige Befehlszeileneingabe"),
            ("es", "entrada de línea de comandos no válida"),
            ("fr", "entrée de ligne de commande non valide"),
            ("ja", "無効なコマンドライン入力"),
            ("ko", "잘못된 명령줄 입력"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_error_unrecognized_subcommand",
        HashMap::from([
            ("en", "unrecognized subcommand '{value}'"),
            ("zh", "无法识别的子命令“{value}”"),
            ("de", "nicht erkannter Unterbefehl „{value}“"),
            ("es", "subcomando no reconocido «{value}»"),
            ("fr", "sous-commande non reconnue «{value}»"),
            ("ja", "認識されないサブコマンド「{value}」"),
            ("ko", "인식할 수 없는 하위 명령 '{value}'"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_error_unexpected_argument",
        HashMap::from([
            ("en", "unexpected argument '{value}' found"),
            ("zh", "发现意外的参数“{value}”"),
            ("de", "unerwartetes Argument „{value}“ gefunden"),
            ("es", "se encontró el argumento inesperado «{value}»"),
            ("fr", "argument inattendu «{value}» trouvé"),
            ("ja", "予期しない引数「{value}」が見つかりました"),
            ("ko", "예기치 않은 인수 '{value}'을(를) 찾았습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_error_value_required",
        HashMap::from([
            (
                "en",
                "a value is required for '{argument}' but none was supplied",
            ),
            ("zh", "“{argument}”需要一个值，但未提供"),
            (
                "de",
                "für „{argument}“ ist ein Wert erforderlich, es wurde jedoch keiner angegeben",
            ),
            (
                "es",
                "se requiere un valor para «{argument}», pero no se proporcionó ninguno",
            ),
            (
                "fr",
                "une valeur est requise pour «{argument}», mais aucune n’a été fournie",
            ),
            ("ja", "「{argument}」には値が必要ですが、指定されていません"),
            ("ko", "'{argument}'에는 값이 필요하지만 제공되지 않았습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_error_invalid_value",
        HashMap::from([
            ("en", "invalid value '{value}' for '{argument}'"),
            ("zh", "“{argument}”的值“{value}”无效"),
            ("de", "ungültiger Wert „{value}“ für „{argument}“"),
            ("es", "valor no válido «{value}» para «{argument}»"),
            ("fr", "valeur non valide «{value}» pour «{argument}»"),
            ("ja", "「{argument}」の値「{value}」は無効です"),
            ("ko", "'{argument}'의 값 '{value}'이(가) 잘못되었습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_tip",
        HashMap::from([
            ("en", "tip"),
            ("zh", "提示"),
            ("de", "Tipp"),
            ("es", "sugerencia"),
            ("fr", "conseil"),
            ("ja", "ヒント"),
            ("ko", "힌트"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_error_similar_subcommand",
        HashMap::from([
            ("en", "a similar subcommand exists: {values}"),
            ("zh", "存在相似的子命令：{values}"),
            ("de", "ein ähnlicher Unterbefehl existiert: {values}"),
            ("es", "existe un subcomando similar: {values}"),
            ("fr", "une sous-commande similaire existe : {values}"),
            ("ja", "似たサブコマンドがあります: {values}"),
            ("ko", "비슷한 하위 명령이 있습니다: {values}"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_error_similar_argument",
        HashMap::from([
            ("en", "a similar argument exists: {values}"),
            ("zh", "存在相似的参数：{values}"),
            ("de", "ein ähnliches Argument existiert: {values}"),
            ("es", "existe un argumento similar: {values}"),
            ("fr", "un argument similaire existe : {values}"),
            ("ja", "似た引数があります: {values}"),
            ("ko", "비슷한 인수가 있습니다: {values}"),
        ]),
    );
    translation_map.set_translation_key(
        "hrafnix_more_information",
        HashMap::from([
            ("en", "For more information, try"),
            ("zh", "如需更多信息，请尝试"),
            ("de", "Weitere Informationen erhalten Sie mit"),
            ("es", "Para más información, pruebe"),
            ("fr", "Pour plus d’informations, essayez"),
            ("ja", "詳細については、次を試してください"),
            ("ko", "자세한 내용은 다음을 시도하세요"),
        ]),
    );
}

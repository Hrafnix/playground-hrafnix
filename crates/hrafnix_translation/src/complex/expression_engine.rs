use hrafnix_shareable_string::SharedStringTranslationMap;
use std::collections::HashMap;

/// Add translations for the Expression Engine to the provided `SharedStringTranslationMap`.
pub(crate) fn add_expression_engine_translation_map(
    translation_map: &mut SharedStringTranslationMap,
) {
    translation_map.set_translation_key(
        "expression_engine_lexer_invalid_character",
        HashMap::from([
            ("en", "Invalid character in expression: '%{character}'"),
            ("zh", "表达式中存在无效字符：'%{character}'"),
            ("de", "Ungültiges Zeichen im Ausdruck: '%{character}'"),
            ("es", "Carácter no válido en la expresión: '%{character}'"),
            (
                "fr",
                "Caractère non valide dans l’expression : '%{character}'",
            ),
            ("ja", "式に無効な文字があります: '%{character}'"),
            ("ko", "식에 잘못된 문자가 있습니다: '%{character}'"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_lexer_invalid_number",
        HashMap::from([
            ("en", "Invalid number in expression: '%{number}'"),
            ("zh", "表达式中的数字无效：'%{number}'"),
            ("de", "Ungültige Zahl im Ausdruck: '%{number}'"),
            ("es", "Número no válido en la expresión: '%{number}'"),
            ("fr", "Nombre non valide dans l’expression : '%{number}'"),
            ("ja", "式に無効な数値があります: '%{number}'"),
            ("ko", "식에 잘못된 숫자가 있습니다: '%{number}'"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_lexer_invalid_operator",
        HashMap::from([
            ("en", "Invalid operator in expression: '%{operator}'"),
            ("zh", "表达式中的运算符无效：'%{operator}'"),
            ("de", "Ungültiger Operator im Ausdruck: '%{operator}'"),
            ("es", "Operador no válido en la expresión: '%{operator}'"),
            (
                "fr",
                "Opérateur non valide dans l’expression : '%{operator}'",
            ),
            ("ja", "式に無効な演算子があります: '%{operator}'"),
            ("ko", "식에 잘못된 연산자가 있습니다: '%{operator}'"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_lexer_invalid_string",
        HashMap::from([
            ("en", "Invalid string in expression: '%{string}'"),
            ("zh", "表达式中的字符串无效：'%{string}'"),
            ("de", "Ungültige Zeichenkette im Ausdruck: '%{string}'"),
            ("es", "Cadena no válida en la expresión: '%{string}'"),
            ("fr", "Chaîne non valide dans l’expression : '%{string}'"),
            ("ja", "式に無効な文字列があります: '%{string}'"),
            ("ko", "식에 잘못된 문자열이 있습니다: '%{string}'"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_lexer_unterminated_string_literal",
        HashMap::from([
            ("en", "Unterminated string literal"),
            ("zh", "未终止的字符串字面量"),
            ("de", "Nicht abgeschlossenes Zeichenkettenliteral"),
            ("es", "Literal de cadena sin terminar"),
            ("fr", "Littéral de chaîne non terminé"),
            ("ja", "文字列リテラルが終了していません"),
            ("ko", "문자열 리터럴이 종료되지 않았습니다"),
        ]),
    );

    translation_map.set_translation_key(
        "expression_engine_evaluation_function_name_empty",
        HashMap::from([
            ("en", "Function name must not be empty."),
            ("zh", "函数名称不能为空。"),
            ("de", "Der Funktionsname darf nicht leer sein."),
            ("es", "El nombre de la función no debe estar vacío."),
            ("fr", "Le nom de la fonction ne doit pas être vide."),
            ("ja", "関数名を空にすることはできません。"),
            ("ko", "함수 이름은 비워둘 수 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_missing_required_global",
        HashMap::from([
            ("en", "Missing required global: %{global}"),
            ("zh", "缺少必需的全局变量：%{global}"),
            ("de", "Fehlendes erforderliches globales Element: %{global}"),
            ("es", "Falta el valor global requerido: %{global}"),
            ("fr", "Variable globale requise manquante : %{global}"),
            ("ja", "必要なグローバル変数がありません: %{global}"),
            ("ko", "필수 전역 항목 누락: %{global}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_missing_required_parameter",
        HashMap::from([
            ("en", "Missing required parameter: %{parameter}"),
            ("zh", "缺少必需参数：%{parameter}"),
            ("de", "Fehlender erforderlicher Parameter: %{parameter}"),
            ("es", "Falta el parámetro requerido: %{parameter}"),
            ("fr", "Paramètre obligatoire manquant : %{parameter}"),
            ("ja", "必須パラメーターがありません: %{parameter}"),
            ("ko", "필수 매개변수 누락: %{parameter}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_missing_required_variable",
        HashMap::from([
            ("en", "Missing required variable: %{variable}"),
            ("zh", "缺少必需变量：%{variable}"),
            ("de", "Fehlende erforderliche Variable: %{variable}"),
            ("es", "Falta la variable requerida: %{variable}"),
            ("fr", "Variable obligatoire manquante : %{variable}"),
            ("ja", "必要な変数がありません: %{variable}"),
            ("ko", "필수 변수 누락: %{variable}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_missing_required_function",
        HashMap::from([
            ("en", "Missing required function: %{function}"),
            ("zh", "缺少必需函数：%{function}"),
            ("de", "Fehlende erforderliche Funktion: %{function}"),
            ("es", "Falta la función requerida: %{function}"),
            ("fr", "Fonction requise manquante : %{function}"),
            ("ja", "必要な関数がありません: %{function}"),
            ("ko", "필수 함수 누락: %{function}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_expected_end_of_input",
        HashMap::from([
            (
                "en",
                "Invalid expression: expected end of input, found %{token}",
            ),
            ("zh", "表达式无效：预期输入结束，但发现 %{token}"),
            (
                "de",
                "Ungültiger Ausdruck: Ende der Eingabe erwartet, %{token} gefunden",
            ),
            (
                "es",
                "Expresión no válida: se esperaba el final de la entrada, se encontró %{token}",
            ),
            (
                "fr",
                "Expression non valide : fin de l’entrée attendue, %{token} trouvé",
            ),
            (
                "ja",
                "無効な式: 入力の終わりが予期されましたが、%{token} が見つかりました",
            ),
            (
                "ko",
                "잘못된 표현식: 입력이 끝나야 하지만 %{token}이(가) 발견되었습니다",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_expected_expression",
        HashMap::from([
            ("en", "Invalid expression: expected an identifier, a number, or a prefix operator, found %{token}"),
            ("zh", "表达式无效：需要标识符、数字或前缀运算符，但找到 %{token}"),
            ("de", "Ungültiger Ausdruck: Erwartet wurde ein Bezeichner, eine Zahl oder ein Präfixoperator. %{token} wurde gefunden"),
            ("es", "Expresión no válida: se esperaba un identificador, un número o un operador de prefijo, se encontró %{token}"),
            ("fr", "Expression non valide : identifiant, nombre ou opérateur de préfixe attendu, %{token} trouvé"),
            ("ja", "無効な式: 識別子、数値、または接頭辞演算子が必要でしたが、%{token} が見つかりました"),
            ("ko", "잘못된 표현식: 식별자, 숫자 또는 전위 연산자가 필요하지만 %{token}이(가) 발견되었습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_expected_operator",
        HashMap::from([
            (
                "en",
                "Invalid expression: expected an operator, found %{token}",
            ),
            ("zh", "表达式无效：需要一个运算符，但找到了 %{token}"),
            (
                "de",
                "Ungültiger Ausdruck: Operator erwartet, %{token} gefunden",
            ),
            (
                "es",
                "Expresión no válida: se esperaba un operador, se encontró %{token}",
            ),
            (
                "fr",
                "Expression non valide : opérateur attendu, %{token} trouvé",
            ),
            (
                "ja",
                "無効な式: 演算子が必要でしたが、%{token} が見つかりました",
            ),
            (
                "ko",
                "잘못된 표현식: 연산자가 필요하지만 %{token}이(가) 발견되었습니다",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_expected_specific_operator",
        HashMap::from([
            (
                "en",
                "Invalid expression: expected operator '%{expected}', found %{token}",
            ),
            (
                "zh",
                "表达式无效：预期运算符“%{expected}”，但找到了 %{token}",
            ),
            (
                "de",
                "Ungültiger Ausdruck: Operator „%{expected}“ erwartet, %{token} gefunden",
            ),
            (
                "es",
                "Expresión no válida: se esperaba el operador '%{expected}', se encontró %{token}",
            ),
            (
                "fr",
                "Expression non valide : opérateur « %{expected} » attendu, %{token} trouvé",
            ),
            (
                "ja",
                "無効な式: 演算子 '%{expected}' が必要ですが、%{token} が見つかりました",
            ),
            (
                "ko",
                "잘못된 표현식: 연산자 '%{expected}'이(가) 필요하지만 %{token}이(가) 발견되었습니다",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_function_name_required_number",
        HashMap::from([
            ("en", "Invalid expression: function calls require a function name, found number %{value}"),
            ("zh", "表达式无效：函数调用需要函数名称，但找到了数值 %{value}"),
            ("de", "Ungültiger Ausdruck: Funktionsaufrufe erfordern einen Funktionsnamen, Zahl %{value} gefunden"),
            ("es", "Expresión no válida: las llamadas a funciones requieren un nombre de función, se encontró el número %{value}"),
            ("fr", "Expression non valide : les appels de fonction nécessitent un nom de fonction, nombre %{value} trouvé"),
            ("ja", "無効な式: 関数呼び出しには関数名が必要ですが、数値 %{value} が見つかりました"),
            ("ko", "잘못된 표현식: 함수 호출에는 함수 이름이 필요하지만 숫자 %{value}이(가) 발견되었습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_function_name_required_text",
        HashMap::from([
            ("en", "Invalid expression: function calls require a function name, found \"%{value}\""),
            ("zh", "表达式无效：函数调用需要函数名称，找到“%{value}”"),
            ("de", "Ungültiger Ausdruck: Funktionsaufrufe erfordern einen Funktionsnamen, gefunden „%{value}“"),
            ("es", "Expresión no válida: las llamadas a funciones requieren un nombre de función, se encontró \"%{value}\""),
            ("fr", "Expression non valide : les appels de fonction nécessitent un nom de fonction, \"%{value}\" trouvé"),
            ("ja", "無効な式: 関数呼び出しには関数名が必要ですが、\"%{value}\" が見つかりました"),
            ("ko", "잘못된 표현식: 함수 호출에는 함수 이름이 필요하지만 \"%{value}\"이(가) 발견되었습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_function_name_required_expression",
        HashMap::from([
            ("en", "Invalid expression: function calls require a function name, found expression starting with operator %{operator}"),
            ("zh", "表达式无效：函数调用需要函数名称，找到以运算符 %{operator} 开头的表达式"),
            ("de", "Ungültiger Ausdruck: Funktionsaufrufe erfordern einen Funktionsnamen. Der gefundene Ausdruck beginnt mit dem Operator %{operator}"),
            ("es", "Expresión no válida: las llamadas a funciones requieren un nombre de función; se encontró una expresión que comienza con el operador %{operator}"),
            ("fr", "Expression non valide : les appels de fonction nécessitent un nom de fonction, expression commençant par l’opérateur %{operator} trouvée"),
            ("ja", "無効な式: 関数呼び出しには関数名が必要ですが、演算子 %{operator} で始まる式が見つかりました"),
            ("ko", "잘못된 표현식: 함수 호출에는 함수 이름이 필요합니다. 연산자 %{operator}(으)로 시작하는 표현식이 발견되었습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_invalid_prefix_operator",
        HashMap::from([
            ("en", "Invalid prefix operator in expression: '%{operator}'"),
            ("zh", "表达式中的前缀运算符无效：“%{operator}”"),
            ("de", "Ungültiger Präfixoperator im Ausdruck: „%{operator}“"),
            (
                "es",
                "Operador de prefijo no válido en la expresión: '%{operator}'",
            ),
            (
                "fr",
                "Opérateur de préfixe non valide dans l'expression : '%{operator}'",
            ),
            ("ja", "式内の接頭辞演算子が無効です: '%{operator}'"),
            ("ko", "표현식의 전위 연산자가 잘못되었습니다: '%{operator}'"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_binary_missing_left_operand",
        HashMap::from([
            ("en", "Binary operator is missing its left operand."),
            ("zh", "二元运算符缺少左操作数。"),
            ("de", "Dem Binäroperator fehlt der linke Operand."),
            ("es", "Al operador binario le falta su operando izquierdo."),
            ("fr", "L'opérateur binaire n'a pas son opérande gauche."),
            ("ja", "二項演算子に左オペランドがありません。"),
            ("ko", "이항 연산자에 왼쪽 피연산자가 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_binary_missing_right_operand",
        HashMap::from([
            ("en", "Binary operator is missing its right operand."),
            ("zh", "二元运算符缺少右操作数。"),
            ("de", "Dem Binäroperator fehlt der rechte Operand."),
            ("es", "Al operador binario le falta su operando derecho."),
            ("fr", "L'opérateur binaire n'a pas son opérande droit."),
            ("ja", "二項演算子に右側のオペランドがありません。"),
            ("ko", "이항 연산자에 오른쪽 피연산자가 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_unary_missing_operand",
        HashMap::from([
            ("en", "Unary operator is missing its operand."),
            ("zh", "一元运算符缺少操作数。"),
            ("de", "Dem unären Operator fehlt sein Operand."),
            ("es", "Al operador unario le falta su operando."),
            ("fr", "L'opérateur unaire n'a pas son opérande."),
            ("ja", "単項演算子にオペランドがありません。"),
            ("ko", "단항 연산자에 피연산자가 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_unary_plus_missing_operand",
        HashMap::from([
            ("en", "Unary '+' operator is missing its operand."),
            ("zh", "一元“+”运算符缺少其操作数。"),
            ("de", "Dem unären „+“-Operator fehlt sein Operand."),
            ("es", "Al operador unario '+' le falta su operando."),
            ("fr", "L'opérateur unaire '+' n'a pas son opérande."),
            ("ja", "単項「+」演算子にオペランドがありません。"),
            ("ko", "단항 '+' 연산자에 피연산자가 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_index_missing_target",
        HashMap::from([
            ("en", "Index operator is missing its target."),
            ("zh", "索引运算符缺少其目标。"),
            ("de", "Dem Indexoperator fehlt der zu indizierende Wert."),
            (
                "es",
                "Al operador de índice le falta el valor que se debe indexar.",
            ),
            ("fr", "L’opérateur d’index n’a pas de cible."),
            ("ja", "インデックス演算子にターゲットがありません。"),
            ("ko", "인덱스 연산자에 대상이 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_index_missing_index",
        HashMap::from([
            ("en", "Index operator is missing its index."),
            ("zh", "索引运算符缺少索引。"),
            ("de", "Dem Indexoperator fehlt sein Index."),
            ("es", "Al operador de índice le falta su índice."),
            ("fr", "L’opérateur d’index n’a pas son index."),
            ("ja", "インデックス演算子にインデックスがありません。"),
            ("ko", "인덱스 연산자에 인덱스가 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_index_target_must_be_identifier",
        HashMap::from([
            ("en", "Only named values can be indexed, found '%{target}'."),
            ("zh", "只能对命名值进行索引，但找到的是“%{target}”。"),
            ("de", "Nur benannte Werte können indiziert werden, gefunden wurde '%{target}'."),
            ("es", "Solo se pueden indexar valores con nombre; se encontró '%{target}'."),
            ("fr", "Seules les valeurs nommées peuvent être indexées, « %{target} » trouvé."),
            ("ja", "インデックスを付けられるのは名前付きの値のみですが、'%{target}' が見つかりました。"),
            ("ko", "이름이 있는 값만 인덱싱할 수 있지만 '%{target}'이(가) 발견되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_invalid_numeric_literal",
        HashMap::from([
            ("en", "Invalid numeric literal: %{value}"),
            ("zh", "无效的数值字面量：%{value}"),
            ("de", "Ungültiges numerisches Literal: %{value}"),
            ("es", "Literal numérico no válido: %{value}"),
            ("fr", "Littéral numérique non valide : %{value}"),
            ("ja", "無効な数値リテラル: %{value}"),
            ("ko", "잘못된 숫자 리터럴: %{value}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_unsupported_operator",
        HashMap::from([
            ("en", "Unsupported operator: %{operator}"),
            ("zh", "不支持的运算符：%{operator}"),
            ("de", "Nicht unterstützter Operator: %{operator}"),
            ("es", "Operador no admitido: %{operator}"),
            ("fr", "Opérateur non pris en charge : %{operator}"),
            ("ja", "サポートされていない演算子: %{operator}"),
            ("ko", "지원되지 않는 연산자: %{operator}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_floating_point_not_finite",
        HashMap::from([
            ("en", "Floating-point results must be finite."),
            ("zh", "浮点结果必须是有限的。"),
            ("de", "Gleitkommaergebnisse müssen endlich sein."),
            ("es", "Los resultados de punto flotante deben ser finitos."),
            (
                "fr",
                "Les résultats à virgule flottante doivent être finis.",
            ),
            ("ja", "浮動小数点の結果は有限でなければなりません。"),
            ("ko", "부동 소수점 결과는 유한해야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_variable_not_found",
        HashMap::from([
            ("en", "Variable '%{variable}' not found in computed data. If you want to use a literal string, wrap it in quotes \"%{variable}\"."),
            ("zh", "在计算数据中找不到变量“%{variable}”。如果要使用字符串字面量，请将其用引号括起来：\"%{variable}\"。"),
            ("de", "Die Variable „%{variable}“ wurde in den berechneten Daten nicht gefunden. Wenn Sie ein Zeichenkettenliteral verwenden möchten, setzen Sie es in doppelte Anführungszeichen \"%{variable}\"."),
            ("es", "La variable '%{variable}' no se encuentra en los datos calculados. Si desea utilizar una cadena literal, enciérrela entre comillas \"%{variable}\"."),
            ("fr", "La variable '%{variable}' est introuvable dans les données calculées. Si vous souhaitez utiliser une chaîne littérale, placez-la entre guillemets \"%{variable}\"."),
            ("ja", "変数「%{variable}」が計算データに見つかりません。リテラル文字列を使用する場合は、引用符 \"%{variable}\" で囲みます。"),
            ("ko", "계산된 데이터에서 변수 '%{variable}'을(를) 찾을 수 없습니다. 문자열 리터럴을 사용하려면 \"%{variable}\"처럼 따옴표로 묶으세요."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_integer_overflow",
        HashMap::from([
            ("en", "Integer overflow."),
            ("zh", "整数溢出。"),
            ("de", "Ganzzahlüberlauf."),
            ("es", "Desbordamiento de enteros."),
            ("fr", "Débordement d'entier."),
            ("ja", "整数オーバーフロー。"),
            ("ko", "정수 오버플로."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_invalid_unary_operation",
        HashMap::from([
            ("en", "Invalid unary operation."),
            ("zh", "无效的一元运算。"),
            ("de", "Ungültige unäre Operation."),
            ("es", "Operación unaria no válida."),
            ("fr", "Opération unaire invalide."),
            ("ja", "無効な単項演算です。"),
            ("ko", "단항 연산이 잘못되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_unsupported_operator",
        HashMap::from([
            ("en", "Unsupported operator %{operator} for %{type} values."),
            ("zh", "对于 %{type} 值，不支持运算符 %{operator}。"),
            (
                "de",
                "Nicht unterstützter Operator %{operator} für %{type}-Werte.",
            ),
            (
                "es",
                "Operador no admitido %{operator} para valores %{type}.",
            ),
            (
                "fr",
                "Opérateur %{operator} non pris en charge pour les valeurs %{type}.",
            ),
            (
                "ja",
                "%{type} 値の演算子 %{operator} はサポートされていません。",
            ),
            (
                "ko",
                "%{type} 값에 대해 지원되지 않는 연산자 %{operator}입니다.",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_division_by_zero",
        HashMap::from([
            ("en", "Division by zero."),
            ("zh", "除以零。"),
            ("de", "Division durch Null."),
            ("es", "División por cero."),
            ("fr", "Division par zéro."),
            ("ja", "ゼロ除算。"),
            ("ko", "0으로 나누기."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_modulus_by_zero",
        HashMap::from([
            ("en", "Modulus by zero."),
            ("zh", "取模运算的除数为零。"),
            ("de", "Modulo durch Null."),
            ("es", "Módulo por cero."),
            ("fr", "Modulo par zéro."),
            ("ja", "剰余演算の除数がゼロです。"),
            ("ko", "나머지 연산의 제수가 0입니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_invalid_integer_exponent",
        HashMap::from([
            (
                "en",
                "Integer exponent must be non-negative and fit within u32.",
            ),
            ("zh", "整数指数必须为非负数，且在 u32 的取值范围内。"),
            (
                "de",
                "Der ganzzahlige Exponent darf nicht negativ sein und muss im Wertebereich von u32 liegen.",
            ),
            (
                "es",
                "El exponente entero debe ser no negativo y caber dentro de u32.",
            ),
            (
                "fr",
                "L’exposant entier doit être non négatif et tenir dans un u32.",
            ),
            (
                "ja",
                "整数の指数は非負で、u32 の範囲内である必要があります。",
            ),
            (
                "ko",
                "정수 지수는 음수가 아니어야 하며 u32의 범위 안에 있어야 합니다.",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_function_not_defined",
        HashMap::from([
            ("en", "Function '%{function}' is not defined."),
            ("zh", "未定义函数“%{function}”。"),
            ("de", "Die Funktion „%{function}“ ist nicht definiert."),
            ("es", "La función '%{function}' no está definida."),
            ("fr", "La fonction '%{function}' n'est pas définie."),
            ("ja", "関数「%{function}」が定義されていません。"),
            ("ko", "'%{function}' 함수가 정의되지 않았습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_function_wrong_argument_count_exact",
        HashMap::from([
            ("en", "Function '%{function}' requires exactly %{expected} arguments, got %{actual}."),
            ("zh", "函数“%{function}”需要恰好 %{expected} 个参数，但实际提供了 %{actual} 个。"),
            ("de", "Die Funktion „%{function}“ erfordert genau %{expected} Argumente, erhalten wurden %{actual}."),
            ("es", "La función '%{function}' requiere exactamente %{expected} argumentos, se recibieron %{actual}."),
            ("fr", "La fonction '%{function}' nécessite exactement %{expected} arguments, %{actual} reçus."),
            ("ja", "関数 '%{function}' にはちょうど %{expected} 個の引数が必要ですが、%{actual} 個が指定されました。"),
            ("ko", "'%{function}' 함수에는 정확히 %{expected}개의 인수가 필요하지만 %{actual}개가 제공되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_function_wrong_argument_count_minimum",
        HashMap::from([
            ("en", "Function '%{function}' requires at least %{minimum} arguments, got %{actual}."),
            ("zh", "函数“%{function}”至少需要 %{minimum} 个参数，但实际提供了 %{actual} 个。"),
            ("de", "Die Funktion „%{function}“ erfordert mindestens %{minimum} Argumente, erhalten wurden %{actual}."),
            ("es", "La función '%{function}' requiere al menos %{minimum} argumentos, se recibieron %{actual}."),
            ("fr", "La fonction '%{function}' nécessite au moins %{minimum} arguments, %{actual} reçus."),
            ("ja", "関数 '%{function}' には少なくとも %{minimum} 個の引数が必要ですが、%{actual} 個が指定されました。"),
            ("ko", "함수 '%{function}'에는 최소 %{minimum}개의 인수가 필요하지만 %{actual}개가 제공되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_function_wrong_argument_count_maximum",
        HashMap::from([
            ("en", "Function '%{function}' allows at most %{maximum} arguments, got %{actual}."),
            ("zh", "函数“%{function}”最多允许 %{maximum} 个参数，但实际提供了 %{actual} 个。"),
            ("de", "Die Funktion „%{function}“ erlaubt höchstens %{maximum} Argumente, erhalten wurden %{actual}."),
            ("es", "La función '%{function}' permite como máximo %{maximum} argumentos, se recibieron %{actual}."),
            ("fr", "La fonction '%{function}' autorise au plus %{maximum} arguments, %{actual} reçus."),
            ("ja", "関数 '%{function}' に指定できる引数は最大 %{maximum} 個ですが、%{actual} 個が指定されました。"),
            ("ko", "함수 '%{function}'에 허용되는 인수는 최대 %{maximum}개이지만 %{actual}개가 제공되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_function_wrong_argument_count_range",
        HashMap::from([
            ("en", "Function '%{function}' requires between %{minimum} and %{maximum} arguments, got %{actual}."),
            ("zh", "函数“%{function}”需要 %{minimum} 到 %{maximum} 个参数，但实际提供了 %{actual} 个。"),
            ("de", "Die Funktion „%{function}“ erfordert zwischen %{minimum} und %{maximum} Argumente, erhalten wurden %{actual}."),
            ("es", "La función '%{function}' requiere entre %{minimum} y %{maximum} argumentos, se recibieron %{actual}."),
            ("fr", "La fonction '%{function}' nécessite entre %{minimum} et %{maximum} arguments, %{actual} reçus."),
            ("ja", "関数 '%{function}' には %{minimum} 個以上 %{maximum} 個以下の引数が必要ですが、%{actual} 個が指定されました。"),
            ("ko", "'%{function}' 함수에는 %{minimum}개 이상 %{maximum}개 이하의 인수가 필요하지만 %{actual}개가 제공되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_invalid_index_count",
        HashMap::from([
            ("en", "Indexing requires exactly 2 or 4 indices, got %{actual}."),
            ("zh", "索引操作需要恰好 2 或 4 个索引，但实际提供了 %{actual} 个。"),
            ("de", "Die Indizierung erfordert genau 2 oder 4 Indizes, erhalten wurden %{actual}."),
            ("es", "La indexación requiere exactamente 2 o 4 índices, se recibieron %{actual}."),
            ("fr", "L’indexation nécessite exactement 2 ou 4 indices, %{actual} reçus."),
            ("ja", "インデックスアクセスにはちょうど 2 個または 4 個のインデックスが必要ですが、%{actual} 個が指定されました。"),
            ("ko", "인덱싱에는 정확히 2개 또는 4개의 인덱스가 필요하지만 %{actual}개가 제공되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_index_count",
        HashMap::from([
            ("en", "Table indexing requires exactly 2 indices, got %{actual}."),
            ("zh", "表索引操作需要恰好 2 个索引，但实际提供了 %{actual} 个。"),
            ("de", "Die Tabellenindizierung erfordert genau 2 Indizes, erhalten wurden %{actual}."),
            ("es", "La indexación de tablas requiere exactamente 2 índices, se obtuvieron %{actual}."),
            ("fr", "L’indexation de table nécessite exactement 2 indices, %{actual} reçus."),
            ("ja", "テーブルのインデックスアクセスにはちょうど 2 個のインデックスが必要ですが、%{actual} 個が指定されました。"),
            ("ko", "테이블 인덱싱에는 정확히 2개의 인덱스가 필요하지만 %{actual}개가 제공되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_missing_first_index",
        HashMap::from([
            ("en", "Missing first index."),
            ("zh", "缺少第一个索引。"),
            ("de", "Fehlender erster Index."),
            ("es", "Falta el primer índice."),
            ("fr", "Premier index manquant."),
            ("ja", "最初のインデックスがありません。"),
            ("ko", "첫 번째 인덱스가 누락되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_missing_second_index",
        HashMap::from([
            ("en", "Missing second index."),
            ("zh", "缺少第二个索引。"),
            ("de", "Fehlender zweiter Index."),
            ("es", "Falta el segundo índice."),
            ("fr", "Deuxième index manquant."),
            ("ja", "2 番目のインデックスがありません。"),
            ("ko", "두 번째 인덱스가 누락되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_table_for_indexing",
        HashMap::from([
            ("en", "Expected a table for table indexing."),
            ("zh", "需要一个用于表索引的表。"),
            (
                "de",
                "Erwartet wurde eine Tabelle für die Tabellenindizierung.",
            ),
            ("es", "Se esperaba una tabla para indexar tablas."),
            (
                "fr",
                "Une table était attendue pour l’indexation des tables.",
            ),
            ("ja", "テーブルのインデックス付けにはテーブルが必要です。"),
            ("ko", "테이블 인덱싱을 위한 테이블이 필요합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_row_index_out_of_bounds",
        HashMap::from([
            ("en", "Row index %{index} is out of bounds for a table with %{count} rows."),
            ("zh", "对于包含 %{count} 行的表，行索引 %{index} 超出范围。"),
            ("de", "Der Zeilenindex %{index} liegt außerhalb des zulässigen Bereichs für eine Tabelle mit %{count} Zeilen."),
            ("es", "El índice de fila %{index} está fuera de los límites de una tabla con %{count} filas."),
            ("fr", "L'index de ligne %{index} est hors limites pour une table comportant %{count} lignes."),
            ("ja", "行インデックス %{index} は、%{count} 行を持つテーブルの範囲外です。"),
            ("ko", "행 인덱스 %{index}은(는) %{count}개 행이 있는 테이블의 범위를 벗어났습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_table_row_index",
        HashMap::from([
            (
                "en",
                "Expected an integer index for the table, got %{actual}.",
            ),
            ("zh", "需要表的整数索引，得到 %{actual}。"),
            (
                "de",
                "Für die Tabelle wurde ein ganzzahliger Index erwartet, erhalten wurde %{actual}.",
            ),
            (
                "es",
                "Se esperaba un índice entero para la tabla, pero se recibió %{actual}.",
            ),
            (
                "fr",
                "Un index entier était attendu pour la table ; valeur reçue : %{actual}.",
            ),
            (
                "ja",
                "テーブルには整数のインデックスが必要ですが、%{actual} が指定されました。",
            ),
            (
                "ko",
                "테이블에는 정수 인덱스가 필요하지만 %{actual}이(가) 제공되었습니다.",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_field_not_found",
        HashMap::from([
            ("en", "Field '%{field}' not found in the table row."),
            ("zh", "在表行中找不到字段“%{field}”。"),
            (
                "de",
                "Das Feld „%{field}“ wurde in der Tabellenzeile nicht gefunden.",
            ),
            (
                "es",
                "El campo '%{field}' no se encuentra en la fila de la tabla.",
            ),
            (
                "fr",
                "Champ « %{field} » introuvable dans la ligne du tableau.",
            ),
            ("ja", "フィールド「%{field}」がテーブル行に見つかりません。"),
            ("ko", "테이블 행에서 '%{field}' 필드를 찾을 수 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_column_index_out_of_bounds",
        HashMap::from([
            ("en", "Column index %{index} is out of bounds for a table with %{count} columns."),
            ("zh", "对于具有 %{count} 列的表，列索引 %{index} 超出范围。"),
            ("de", "Der Spaltenindex %{index} liegt außerhalb des zulässigen Bereichs für eine Tabelle mit %{count} Spalten."),
            ("es", "El índice de columna %{index} está fuera de los límites de una tabla con %{count} columnas."),
            ("fr", "L'index de colonne %{index} est hors limites pour une table comportant %{count} colonnes."),
            ("ja", "列インデックス %{index} は、%{count} 列を持つテーブルの範囲外です。"),
            ("ko", "열 인덱스 %{index}은(는) %{count}개 열이 있는 테이블의 범위를 벗어났습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_table_field_index",
        HashMap::from([
            ("en", "Expected a string or integer index for the table field, got %{actual}."),
            ("zh", "需要表字段的字符串或整数索引，得到 %{actual}。"),
            ("de", "Für das Tabellenfeld wurde ein Zeichenketten- oder Ganzzahlindex erwartet, erhalten wurde %{actual}."),
            ("es", "Se esperaba un índice de tipo cadena o entero para el campo de la tabla, pero se recibió %{actual}."),
            ("fr", "Le champ de la table nécessite une chaîne ou un index entier ; valeur reçue : %{actual}."),
            ("ja", "テーブルのフィールドには文字列または整数のインデックスが必要ですが、%{actual} が指定されました。"),
            ("ko", "테이블 필드에는 문자열 또는 정수 인덱스가 필요하지만 %{actual}이(가) 제공되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_invalid_choice",
        HashMap::from([
            ("en", "Value '%{value}' is not a valid choice."),
            ("zh", "值“%{value}”不是有效的选择。"),
            ("de", "Der Wert „%{value}“ ist keine gültige Auswahl."),
            ("es", "El valor '%{value}' no es una opción válida."),
            ("fr", "La valeur « %{value} » n'est pas un choix valide."),
            ("ja", "値「%{value}」は有効な選択ではありません。"),
            ("ko", "'%{value}' 값은 유효한 선택이 아닙니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_invalid_unit_id",
        HashMap::from([
            ("en", "Value '%{value}' is not a valid unit ID."),
            ("zh", "值“%{value}”不是有效的单位 ID。"),
            ("de", "Der Wert „%{value}“ ist keine gültige Einheiten-ID."),
            ("es", "El valor '%{value}' no es un ID de unidad válido."),
            (
                "fr",
                "La valeur « %{value} » n'est pas un ID d'unité valide.",
            ),
            ("ja", "値「%{value}」は有効な単位 ID ではありません。"),
            ("ko", "값 '%{value}'은(는) 유효한 단위 ID가 아닙니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_invalid_unit_for_family",
        HashMap::from([
            (
                "en",
                "Value '%{value}' is not a valid unit ID for %{family}.",
            ),
            ("zh", "值“%{value}”不是 %{family} 的有效单位 ID。"),
            (
                "de",
                "Der Wert „%{value}“ ist keine gültige Einheiten-ID für %{family}.",
            ),
            (
                "es",
                "El valor '%{value}' no es un ID de unidad válido para %{family}.",
            ),
            (
                "fr",
                "La valeur « %{value} » n'est pas un ID d'unité valide pour %{family}.",
            ),
            (
                "ja",
                "値「%{value}」は、%{family} の有効な単位 ID ではありません。",
            ),
            (
                "ko",
                "값 '%{value}'은(는) %{family}의 유효한 단위 ID가 아닙니다.",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_unit_value",
        HashMap::from([
            ("en", "Expected a unit value, but got %{actual}."),
            ("zh", "预期有一个单位值，但得到的是 %{actual}。"),
            (
                "de",
                "Es wurde ein Einheitenwert erwartet, erhalten wurde jedoch %{actual}.",
            ),
            (
                "es",
                "Se esperaba un valor de unidad, pero se recibió %{actual}.",
            ),
            (
                "fr",
                "Une valeur d’unité était attendue ; valeur reçue : %{actual}.",
            ),
            (
                "ja",
                "単位を表す値が必要ですが、%{actual} が指定されました。",
            ),
            (
                "ko",
                "단위를 나타내는 값이 필요하지만 %{actual}이(가) 제공되었습니다.",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_definition_value",
        HashMap::from([
            ("en", "Expected a %{expected} value, but got %{actual}."),
            ("zh", "需要 %{expected} 类型的值，但得到的是 %{actual}。"),
            (
                "de",
                "Es wurde ein %{expected}-Wert erwartet, aber %{actual} erhalten.",
            ),
            (
                "es",
                "Se esperaba un valor de tipo %{expected}, pero se recibió %{actual}.",
            ),
            (
                "fr",
                "Une valeur de type %{expected} était attendue ; valeur reçue : %{actual}.",
            ),
            (
                "ja",
                "%{expected} 型の値が必要ですが、%{actual} が指定されました。",
            ),
            (
                "ko",
                "%{expected} 유형의 값이 필요하지만 %{actual}이(가) 제공되었습니다.",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_value_below_minimum",
        HashMap::from([
            (
                "en",
                "Value %{value} is less than the minimum allowed value of %{minimum}.",
            ),
            ("zh", "值 %{value} 小于允许的最小值 %{minimum}。"),
            (
                "de",
                "Der Wert %{value} ist kleiner als der minimal zulässige Wert von %{minimum}.",
            ),
            (
                "es",
                "El valor %{value} es menor que el valor mínimo permitido de %{minimum}.",
            ),
            (
                "fr",
                "La valeur %{value} est inférieure à la valeur minimale autorisée de %{minimum}.",
            ),
            (
                "ja",
                "値 %{value} は、許容される最小値 %{minimum} より小さいです。",
            ),
            (
                "ko",
                "%{value} 값은 허용되는 최솟값 %{minimum}보다 작습니다.",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_value_above_maximum",
        HashMap::from([
            (
                "en",
                "Value %{value} is greater than the maximum allowed value of %{maximum}.",
            ),
            ("zh", "值 %{value} 大于允许的最大值 %{maximum}。"),
            (
                "de",
                "Der Wert %{value} ist größer als der maximal zulässige Wert von %{maximum}.",
            ),
            (
                "es",
                "El valor %{value} es mayor que el valor máximo permitido de %{maximum}.",
            ),
            (
                "fr",
                "La valeur %{value} est supérieure à la valeur maximale autorisée de %{maximum}.",
            ),
            (
                "ja",
                "値 %{value} は、許容最大値 %{maximum} を超えています。",
            ),
            ("ko", "%{value} 값은 허용되는 최댓값 %{maximum}보다 큽니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_unit_conversion_failed",
        HashMap::from([
            ("en", "Unit conversion failed: %{error}"),
            ("zh", "单位转换失败：%{error}"),
            ("de", "Einheitenumrechnung fehlgeschlagen: %{error}"),
            ("es", "Error en la conversión de unidades: %{error}"),
            ("fr", "Échec de la conversion d'unité : %{error}"),
            ("ja", "単位変換に失敗しました: %{error}"),
            ("ko", "단위 변환 실패: %{error}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_incompatible_units",
        HashMap::from([
            ("en", "Cannot apply '%{operator}' to incompatible units %{left} and %{right}"),
            ("zh", "无法对不兼容的单位 %{left} 和 %{right} 应用“%{operator}”"),
            ("de", "'%{operator}' kann nicht auf die inkompatiblen Einheiten %{left} und %{right} angewendet werden"),
            ("es", "No se puede aplicar '%{operator}' a las unidades incompatibles %{left} y %{right}"),
            ("fr", "Impossible d'appliquer « %{operator} » aux unités incompatibles %{left} et %{right}"),
            ("ja", "互換性のない単位 %{left} と %{right} に「%{operator}」を適用できません"),
            ("ko", "호환되지 않는 단위 %{left}와(과) %{right}에 '%{operator}'을(를) 적용할 수 없습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_offset_temperature_arithmetic",
        HashMap::from([
            ("en", "Cannot apply '%{operator}' to %{unit} temperatures; use an absolute scale such as kelvin"),
            ("zh", "无法对 %{unit} 温度应用“%{operator}”；请使用开尔文等绝对温标"),
            ("de", "'%{operator}' kann nicht auf Temperaturen in %{unit} angewendet werden; verwenden Sie eine absolute Skala wie Kelvin"),
            ("es", "No se puede aplicar '%{operator}' a temperaturas en %{unit}; use una escala absoluta como el kelvin"),
            ("fr", "Impossible d'appliquer « %{operator} » à des températures en %{unit} ; utilisez une échelle absolue comme le kelvin"),
            ("ja", "%{unit} の温度に「%{operator}」を適用できません。ケルビンなどの絶対温度目盛を使用してください"),
            ("ko", "%{unit} 온도에 '%{operator}'을(를) 적용할 수 없습니다. 켈빈과 같은 절대 온도 단위를 사용하세요"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_number_with_units_definition",
        HashMap::from([
            ("en", "Expected a number-with-units definition."),
            ("zh", "需要一个带有单位的数字定义。"),
            (
                "de",
                "Erwartet wurde eine Definition für eine Zahl mit Einheiten.",
            ),
            ("es", "Se esperaba una definición de número con unidades."),
            ("fr", "Une définition de nombre avec unités était attendue."),
            ("ja", "単位付きの数値定義が必要です。"),
            ("ko", "단위가 포함된 숫자 정의가 필요합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_unknown_unit",
        HashMap::from([
            ("en", "Unknown unit '%{unit}'."),
            ("zh", "未知单位“%{unit}”。"),
            ("de", "Unbekannte Einheit „%{unit}“."),
            ("es", "Unidad desconocida '%{unit}'."),
            ("fr", "Unité inconnue '%{unit}'."),
            ("ja", "不明な単位「%{unit}」。"),
            ("ko", "알 수 없는 단위 '%{unit}'."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_column_requirement_failed_increasing",
        HashMap::from([
            ("en", "Column '%{column}', row %{row}: value %{current} must be greater than the previous value %{previous}."),
            ("zh", "列“%{column}”第 %{row} 行：值 %{current} 必须大于前一个值 %{previous}。"),
            ("de", "Spalte „%{column}“, Zeile %{row}: Wert %{current} muss größer als der vorherige Wert %{previous} sein."),
            ("es", "Columna '%{column}', fila %{row}: el valor %{current} debe ser mayor que el valor anterior %{previous}."),
            ("fr", "Colonne « %{column} », ligne %{row} : la valeur %{current} doit être supérieure à la valeur précédente %{previous}."),
            ("ja", "列「%{column}」、行 %{row}: 値 %{current} は前の値 %{previous} より大きい必要があります。"),
            ("ko", "'%{column}' 열, %{row}행: 값 %{current}은(는) 이전 값 %{previous}보다 커야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_column_requirement_failed_constant_or_increasing",
        HashMap::from([
            ("en", "Column '%{column}', row %{row}: value %{current} must be greater than or equal to the previous value %{previous}."),
            ("zh", "列“%{column}”第 %{row} 行：值 %{current} 必须大于或等于前一个值 %{previous}。"),
            ("de", "Spalte „%{column}“, Zeile %{row}: Wert %{current} muss größer als oder gleich dem vorherigen Wert %{previous} sein."),
            ("es", "Columna '%{column}', fila %{row}: el valor %{current} debe ser mayor o igual que el valor anterior %{previous}."),
            ("fr", "Colonne « %{column} », ligne %{row} : la valeur %{current} doit être supérieure ou égale à la valeur précédente %{previous}."),
            ("ja", "列「%{column}」、行 %{row}: 値 %{current} は前の値 %{previous} 以上である必要があります。"),
            ("ko", "'%{column}' 열, %{row}행: 값 %{current}은(는) 이전 값 %{previous} 이상이어야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_column_requirement_failed_decreasing",
        HashMap::from([
            ("en", "Column '%{column}', row %{row}: value %{current} must be less than the previous value %{previous}."),
            ("zh", "列“%{column}”第 %{row} 行：值 %{current} 必须小于前一个值 %{previous}。"),
            ("de", "Spalte „%{column}“, Zeile %{row}: Wert %{current} muss kleiner als der vorherige Wert %{previous} sein."),
            ("es", "Columna '%{column}', fila %{row}: el valor %{current} debe ser menor que el valor anterior %{previous}."),
            ("fr", "Colonne « %{column} », ligne %{row} : la valeur %{current} doit être inférieure à la valeur précédente %{previous}."),
            ("ja", "列「%{column}」、行 %{row}: 値 %{current} は前の値 %{previous} より小さい必要があります。"),
            ("ko", "'%{column}' 열, %{row}행: 값 %{current}은(는) 이전 값 %{previous}보다 작아야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_column_requirement_failed_constant_or_decreasing",
        HashMap::from([
            ("en", "Column '%{column}', row %{row}: value %{current} must be less than or equal to the previous value %{previous}."),
            ("zh", "列“%{column}”第 %{row} 行：值 %{current} 必须小于或等于前一个值 %{previous}。"),
            ("de", "Spalte „%{column}“, Zeile %{row}: Wert %{current} muss kleiner als oder gleich dem vorherigen Wert %{previous} sein."),
            ("es", "Columna '%{column}', fila %{row}: el valor %{current} debe ser menor o igual que el valor anterior %{previous}."),
            ("fr", "Colonne « %{column} », ligne %{row} : la valeur %{current} doit être inférieure ou égale à la valeur précédente %{previous}."),
            ("ja", "列「%{column}」、行 %{row}: 値 %{current} は前の値 %{previous} 以下である必要があります。"),
            ("ko", "'%{column}' 열, %{row}행: 값 %{current}은(는) 이전 값 %{previous} 이하여야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_column_requirement_failed_constant",
        HashMap::from([
            ("en", "Column '%{column}', row %{row}: value %{current} must be equal to the previous value %{previous}."),
            ("zh", "列“%{column}”第 %{row} 行：值 %{current} 必须等于前一个值 %{previous}。"),
            ("de", "Spalte „%{column}“, Zeile %{row}: Wert %{current} muss gleich dem vorherigen Wert %{previous} sein."),
            ("es", "Columna '%{column}', fila %{row}: el valor %{current} debe ser igual al valor anterior %{previous}."),
            ("fr", "Colonne « %{column} », ligne %{row} : la valeur %{current} doit être égale à la valeur précédente %{previous}."),
            ("ja", "列「%{column}」、行 %{row}: 値 %{current} は前の値 %{previous} と等しい必要があります。"),
            ("ko", "'%{column}' 열, %{row}행: 값 %{current}은(는) 이전 값 %{previous}와(과) 같아야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_column_requirement_missing_cell",
        HashMap::from([
            ("en", "Column '%{column}', row %{row}: missing value for a constrained column."),
            ("zh", "列“%{column}”第 %{row} 行：受约束的列缺少值。"),
            ("de", "Spalte „%{column}“, Zeile %{row}: fehlender Wert in einer Spalte mit Einschränkung."),
            ("es", "Columna '%{column}', fila %{row}: falta un valor en una columna con restricciones."),
            ("fr", "Colonne « %{column} », ligne %{row} : valeur manquante dans une colonne soumise à une contrainte."),
            ("ja", "列「%{column}」、行 %{row}: 制約のある列に値がありません。"),
            ("ko", "'%{column}' 열, %{row}행: 제약 조건이 있는 열에 값이 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_column_count_mismatch",
        HashMap::from([
            ("en", "Parameter '%{parameter}' references a table with %{actual} columns, but the current table expects %{expected} columns."),
            ("zh", "参数“%{parameter}”引用具有 %{actual} 列的表，但当前表需要 %{expected} 列。"),
            ("de", "Der Parameter „%{parameter}“ verweist auf eine Tabelle mit %{actual} Spalten, aber die aktuelle Tabelle erwartet %{expected} Spalten."),
            ("es", "El parámetro '%{parameter}' hace referencia a una tabla con %{actual} columnas, pero la tabla actual espera %{expected} columnas."),
            ("fr", "Le paramètre « %{parameter} » fait référence à une table de %{actual} colonnes, mais la table actuelle attend %{expected} colonnes."),
            ("ja", "パラメーター「%{parameter}」は %{actual} 列を持つテーブルを参照していますが、現在のテーブルは %{expected} 列を想定しています。"),
            ("ko", "매개변수 '%{parameter}'은(는) %{actual}개 열이 있는 테이블을 참조하지만 현재 테이블에는 %{expected}개 열이 필요합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_missing_column_definition",
        HashMap::from([
            ("en", "Parameter '%{parameter}' references a table with no column definition at index %{index}."),
            ("zh", "参数“%{parameter}”引用索引 %{index} 处没有列定义的表。"),
            ("de", "Der Parameter „%{parameter}“ verweist auf eine Tabelle ohne Spaltendefinition am Index %{index}."),
            ("es", "El parámetro '%{parameter}' hace referencia a una tabla sin definición de columna en el índice %{index}."),
            ("fr", "Le paramètre '%{parameter}' fait référence à une table sans définition de colonne à l'index %{index}."),
            ("ja", "パラメーター「%{parameter}」は、インデックス %{index} に列定義のないテーブルを参照しています。"),
            ("ko", "매개변수 '%{parameter}'은(는) 인덱스 %{index}에 열 정의가 없는 테이블을 참조합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_value_below_minimum",
        HashMap::from([
            ("en", "Value %{value} in column '%{column}' is less than the minimum allowed value of %{minimum}."),
            ("zh", "“%{column}”列中的值 %{value} 小于允许的最小值 %{minimum}。"),
            ("de", "Der Wert %{value} in der Spalte „%{column}“ ist kleiner als der minimal zulässige Wert von %{minimum}."),
            ("es", "El valor %{value} en la columna '%{column}' es menor que el valor mínimo permitido de %{minimum}."),
            ("fr", "La valeur %{value} dans la colonne « %{column} » est inférieure à la valeur minimale autorisée de %{minimum}."),
            ("ja", "列 '%{column}' の値 %{value} が、許容される最小値 %{minimum} 未満です。"),
            ("ko", "'%{column}' 열의 %{value} 값이 허용되는 최솟값 %{minimum}보다 작습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_value_above_maximum",
        HashMap::from([
            ("en", "Value %{value} in column '%{column}' is greater than the maximum allowed value of %{maximum}."),
            ("zh", "“%{column}”列中的值 %{value} 大于允许的最大值 %{maximum}。"),
            ("de", "Der Wert %{value} in der Spalte „%{column}“ ist größer als der maximal zulässige Wert von %{maximum}."),
            ("es", "El valor %{value} en la columna '%{column}' es mayor que el valor máximo permitido de %{maximum}."),
            ("fr", "La valeur %{value} dans la colonne « %{column} » est supérieure à la valeur maximale autorisée de %{maximum}."),
            ("ja", "列 '%{column}' の値 %{value} が、許容最大値 %{maximum} を超えています。"),
            ("ko", "'%{column}' 열의 %{value} 값이 허용되는 최댓값 %{maximum}보다 큽니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_table_parameter",
        HashMap::from([
            ("en", "Parameter '%{parameter}' is expected to reference a table, but got %{actual}."),
            ("zh", "参数“%{parameter}”应引用表，但得到的是 %{actual}。"),
            ("de", "Der Parameter „%{parameter}“ soll auf eine Tabelle verweisen, erhalten wurde jedoch %{actual}."),
            ("es", "Se espera que el parámetro '%{parameter}' haga referencia a una tabla, pero se recibió %{actual}."),
            ("fr", "Le paramètre « %{parameter} » doit faire référence à une table ; valeur reçue : %{actual}."),
            ("ja", "パラメーター「%{parameter}」はテーブルを参照する必要がありますが、%{actual} が指定されました。"),
            ("ko", "'%{parameter}' 매개변수는 테이블을 참조해야 하지만 %{actual}이(가) 제공되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_cell_missing_column_definition",
        HashMap::from([
            ("en", "No column definition exists for table cell at index %{index}."),
            ("zh", "索引 %{index} 处的表单元格不存在列定义。"),
            ("de", "Für die Tabellenzelle am Index %{index} ist keine Spaltendefinition vorhanden."),
            ("es", "No existe ninguna definición de columna para la celda de la tabla en el índice %{index}."),
            ("fr", "Aucune définition de colonne n'existe pour la cellule du tableau à l'index %{index}."),
            ("ja", "インデックス %{index} のテーブルセルには列定義が存在しません。"),
            ("ko", "인덱스 %{index}의 테이블 셀에 대한 열 정의가 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_table_cell_number",
        HashMap::from([
            ("en", "Expected a numeric value for table cell, but got %{actual}."),
            ("zh", "表格单元格需要一个数值，但得到的是 %{actual}。"),
            ("de", "Es wurde ein numerischer Wert für die Tabellenzelle erwartet, erhalten wurde jedoch %{actual}."),
            ("es", "Se esperaba un valor numérico para la celda de la tabla, pero se recibió %{actual}."),
            ("fr", "Une valeur numérique était attendue pour la cellule de la table ; valeur reçue : %{actual}."),
            ("ja", "テーブルセルには数値が必要ですが、%{actual} が指定されました。"),
            ("ko", "테이블 셀에는 숫자 값이 필요하지만 %{actual}이(가) 제공되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_unit_count_mismatch",
        HashMap::from([
            ("en", "Parameter '%{parameter}' references a table with %{actual} units for %{expected} columns."),
            ("zh", "参数“%{parameter}”引用的表有 %{expected} 列，但配置的单位数量为 %{actual}。"),
            ("de", "Der Parameter „%{parameter}“ verweist auf eine Tabelle mit %{actual} Einheiten für %{expected} Spalten."),
            ("es", "El parámetro '%{parameter}' hace referencia a una tabla con %{actual} unidades para %{expected} columnas."),
            ("fr", "Le paramètre « %{parameter} » fait référence à une table avec %{actual} unités pour %{expected} colonnes."),
            ("ja", "パラメーター「%{parameter}」が参照するテーブルには %{expected} 列ありますが、単位の設定数は %{actual} 個です。"),
            ("ko", "매개변수 '%{parameter}'이(가) 참조하는 테이블에는 %{expected}개의 열이 있지만 설정된 단위는 %{actual}개입니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_unit_conversion_failed",
        HashMap::from([
            ("en", "Cannot convert parameter '%{parameter}' column '%{column}' from %{source_unit} to %{target_unit}: %{error}"),
            ("zh", "无法将参数“%{parameter}”列“%{column}”从 %{source_unit} 转换为 %{target_unit}：%{error}"),
            ("de", "Die Spalte „%{column}“ des Parameters „%{parameter}“ kann nicht von %{source_unit} in %{target_unit} konvertiert werden: %{error}"),
            ("es", "No se puede convertir la columna '%{column}' del parámetro '%{parameter}' de %{source_unit} a %{target_unit}: %{error}"),
            ("fr", "Impossible de convertir la colonne « %{column} » du paramètre « %{parameter} » de %{source_unit} en %{target_unit} : %{error}"),
            ("ja", "パラメーター「%{parameter}」の列「%{column}」を %{source_unit} から %{target_unit} に変換できません: %{error}"),
            ("ko", "매개변수 '%{parameter}'의 열 '%{column}'을(를) %{source_unit}에서 %{target_unit}(으)로 변환할 수 없습니다: %{error}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_cell_missing_unit",
        HashMap::from([
            ("en", "No unit exists for table cell at index %{index}."),
            ("zh", "索引 %{index} 处的表格单元格不存在单位。"),
            (
                "de",
                "Für die Tabellenzelle am Index %{index} ist keine Einheit vorhanden.",
            ),
            (
                "es",
                "No existe ninguna unidad para la celda de la tabla en el índice %{index}.",
            ),
            (
                "fr",
                "Aucune unité n'existe pour la cellule du tableau à l'index %{index}.",
            ),
            (
                "ja",
                "インデックス %{index} のテーブルセルには単位が設定されていません。",
            ),
            ("ko", "인덱스 %{index}의 테이블 셀에 대한 단위가 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_sum_requires_numeric_arguments",
        HashMap::from([
            ("en", "sum() expects numeric arguments, got %{actual}."),
            ("zh", "sum() 需要数字参数，得到 %{actual}。"),
            (
                "de",
                "sum() erwartet numerische Argumente, erhalten wurde %{actual}.",
            ),
            (
                "es",
                "sum() espera argumentos numéricos, se recibió %{actual}.",
            ),
            (
                "fr",
                "sum() attend des arguments numériques ; valeur reçue : %{actual}.",
            ),
            (
                "ja",
                "sum() には数値の引数が必要ですが、%{actual} が指定されました。",
            ),
            (
                "ko",
                "sum()에는 숫자 인수가 필요하지만 %{actual}이(가) 제공되었습니다.",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_add_requires_numeric_arguments",
        HashMap::from([
            ("en", "add expects numeric arguments."),
            ("zh", "add 需要数字参数。"),
            ("de", "add erwartet numerische Argumente."),
            ("es", "add espera argumentos numéricos."),
            ("fr", "add attend des arguments numériques."),
            ("ja", "add は数値引数を必要とします。"),
            ("ko", "add에는 숫자 인수가 필요합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_must_be_finite",
        HashMap::from([
            ("en", "%{function} function argument must be finite."),
            ("zh", "%{function} 函数参数必须是有限的。"),
            (
                "de",
                "Das Argument der Funktion %{function} muss endlich sein.",
            ),
            (
                "es",
                "El argumento de la función %{function} debe ser finito.",
            ),
            (
                "fr",
                "L’argument de la fonction %{function} doit être fini.",
            ),
            ("ja", "%{function} 関数の引数は有限である必要があります。"),
            ("ko", "%{function} 함수 인수는 유한해야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_out_of_integer_range",
        HashMap::from([
            ("en", "%{function} function argument is out of range for an integer."),
            ("zh", "%{function} 函数参数超出整数范围。"),
            ("de", "Das Argument der Funktion %{function} liegt außerhalb des gültigen Bereichs für eine Ganzzahl."),
            ("es", "El argumento de la función %{function} está fuera del rango para un número entero."),
            ("fr", "L’argument de la fonction %{function} est hors plage pour un nombre entier."),
            ("ja", "%{function} 関数の引数が整数の範囲外です。"),
            ("ko", "%{function} 함수 인수가 정수 범위를 벗어났습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_integer_conversion_failed",
        HashMap::from([
            ("en", "%{function} function argument could not be converted to an integer."),
            ("zh", "%{function} 函数参数无法转换为整数。"),
            ("de", "Das Argument der Funktion %{function} konnte nicht in eine Ganzzahl konvertiert werden."),
            ("es", "El argumento de la función %{function} no se pudo convertir a un número entero."),
            ("fr", "L'argument de la fonction %{function} n'a pas pu être converti en nombre entier."),
            ("ja", "%{function} 関数の引数を整数に変換できませんでした。"),
            ("ko", "%{function} 함수 인수를 정수로 변환할 수 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_float_precision_loss",
        HashMap::from([
            ("en", "%{function} function argument is too large to convert to a float without losing precision."),
            ("zh", "%{function} 函数参数太大，无法在不损失精度的情况下转换为浮点数。"),
            ("de", "Das Argument der Funktion %{function} ist zu groß, um es ohne Präzisionsverlust in eine Gleitkommazahl umzuwandeln."),
            ("es", "El argumento de la función %{function} es demasiado grande para convertirlo en flotante sin perder precisión."),
            ("fr", "L'argument de la fonction %{function} est trop volumineux pour être converti en flottant sans perte de précision."),
            ("ja", "%{function} 関数の引数が大きすぎるため、精度を失わずに浮動小数点数に変換できません。"),
            ("ko", "%{function} 함수 인수가 너무 커서 정밀도를 잃지 않고 부동 소수점으로 변환할 수 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_float_conversion_failed",
        HashMap::from([
            ("en", "%{function} function argument could not be converted to a float."),
            ("zh", "%{function} 函数参数无法转换为浮点数。"),
            ("de", "Das Argument der Funktion %{function} konnte nicht in eine Gleitkommazahl konvertiert werden."),
            ("es", "El argumento de la función %{function} no se pudo convertir en un valor flotante."),
            ("fr", "L'argument de la fonction %{function} n'a pas pu être converti en flottant."),
            ("ja", "%{function} 関数の引数を浮動小数点数に変換できませんでした。"),
            ("ko", "%{function} 함수 인수를 부동 소수점으로 변환할 수 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_missing_expected_argument",
        HashMap::from([
            (
                "en",
                "%{function} function is missing an expected argument.",
            ),
            ("zh", "%{function} 函数缺少预期参数。"),
            (
                "de",
                "Der Funktion %{function} fehlt ein erwartetes Argument.",
            ),
            (
                "es",
                "A la función %{function} le falta un argumento esperado.",
            ),
            (
                "fr",
                "Il manque un argument attendu à la fonction %{function}.",
            ),
            ("ja", "%{function} 関数に予期された引数がありません。"),
            ("ko", "%{function} 함수에 예상 인수가 누락되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_must_be_float",
        HashMap::from([
            ("en", "%{function} function argument must be a float."),
            ("zh", "%{function} 函数参数必须是浮点数。"),
            (
                "de",
                "Das Argument der Funktion %{function} muss eine Gleitkommazahl sein.",
            ),
            (
                "es",
                "El argumento de la función %{function} debe ser un flotante.",
            ),
            (
                "fr",
                "L’argument de la fonction %{function} doit être un flottant.",
            ),
            (
                "ja",
                "%{function} 関数の引数は浮動小数点数でなければなりません。",
            ),
            ("ko", "%{function} 함수 인수는 부동 소수점이어야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_arguments_mixed_numeric_types",
        HashMap::from([
            ("en", "%{function} function arguments must all be the same numeric type."),
            ("zh", "%{function} 函数参数必须全部为相同的数字类型。"),
            ("de", "Die Argumente der Funktion %{function} müssen alle vom gleichen numerischen Typ sein."),
            ("es", "Todos los argumentos de la función %{function} deben ser del mismo tipo numérico."),
            ("fr", "Les arguments de la fonction %{function} doivent tous être du même type numérique."),
            ("ja", "%{function} 関数の引数はすべて同じ数値型である必要があります。"),
            ("ko", "%{function} 함수 인수는 모두 동일한 숫자 유형이어야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_must_be_number",
        HashMap::from([
            ("en", "%{function} function argument must be a number."),
            ("zh", "%{function} 函数参数必须是数字。"),
            (
                "de",
                "Das Argument der Funktion %{function} muss eine Zahl sein.",
            ),
            (
                "es",
                "El argumento de la función %{function} debe ser un número.",
            ),
            (
                "fr",
                "L’argument de la fonction %{function} doit être un nombre.",
            ),
            ("ja", "%{function} 関数の引数は数値である必要があります。"),
            ("ko", "%{function} 함수 인수는 숫자여야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_clamp_minimum_exceeds_maximum",
        HashMap::from([
            ("en", "clamp function minimum argument must not be greater than its maximum argument."),
            ("zh", "clamp 函数的最小值参数不得大于最大值参数。"),
            ("de", "Das Minimumargument der Funktion clamp darf nicht größer sein als ihr Maximumargument."),
            ("es", "El argumento mínimo de la función clamp no debe ser mayor que su argumento máximo."),
            ("fr", "L’argument minimum de la fonction clamp ne doit pas être supérieur à son argument maximum."),
            ("ja", "clamp 関数の最小値の引数は最大値の引数を超えてはなりません。"),
            ("ko", "clamp 함수의 최솟값 인수는 최댓값 인수보다 클 수 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_incompatible_units",
        HashMap::from([
            ("en", "%{function} function cannot combine incompatible units %{left} and %{right}"),
            ("zh", "%{function} 函数无法组合不兼容的单位 %{left} 和 %{right}"),
            ("de", "Die Funktion %{function} kann die inkompatiblen Einheiten %{left} und %{right} nicht kombinieren"),
            ("es", "La función %{function} no puede combinar las unidades incompatibles %{left} y %{right}"),
            ("fr", "La fonction %{function} ne peut pas combiner les unités incompatibles %{left} et %{right}"),
            ("ja", "%{function} 関数は互換性のない単位 %{left} と %{right} を組み合わせることができません"),
            ("ko", "%{function} 함수는 호환되지 않는 단위 %{left}와(과) %{right}을(를) 결합할 수 없습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_length_result_too_large",
        HashMap::from([
            ("en", "len function result is too large to fit in an integer."),
            ("zh", "len 函数结果太大，无法用整数表示。"),
            ("de", "Das Ergebnis der len-Funktion ist zu groß, um in eine ganze Zahl zu passen."),
            ("es", "El resultado de la función len es demasiado grande para caber en un número entero."),
            ("fr", "Le résultat de la fonction len est trop grand pour tenir dans un entier."),
            ("ja", "len 関数の結果が大きすぎて整数に収まりません。"),
            ("ko", "len 함수 결과가 너무 커서 정수에 맞지 않습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_must_be_string",
        HashMap::from([
            ("en", "%{function} function argument must be a string."),
            ("zh", "%{function} 函数参数必须是字符串。"),
            (
                "de",
                "Das Argument der Funktion %{function} muss eine Zeichenkette sein.",
            ),
            (
                "es",
                "El argumento de la función %{function} debe ser una cadena.",
            ),
            (
                "fr",
                "L’argument de la fonction %{function} doit être une chaîne.",
            ),
            ("ja", "%{function} 関数の引数は文字列である必要があります。"),
            ("ko", "%{function} 함수 인수는 문자열이어야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_if_condition_must_be_boolean",
        HashMap::from([
            ("en", "if function first argument must be a boolean."),
            ("zh", "if 函数第一个参数必须是布尔值。"),
            (
                "de",
                "Das erste Argument der Funktion if muss ein boolescher Wert sein.",
            ),
            (
                "es",
                "El primer argumento de la función if debe ser booleano.",
            ),
            (
                "fr",
                "Le premier argument de la fonction if doit être un booléen.",
            ),
            ("ja", "if 関数の最初の引数はブール値でなければなりません。"),
            ("ko", "if 함수의 첫 번째 인수는 부울 값이어야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_custom_function_failed",
        HashMap::from([
            ("en", "Custom function failed: %{actual}"),
            ("zh", "自定义函数失败：%{actual}"),
            (
                "de",
                "Benutzerdefinierte Funktion fehlgeschlagen: %{actual}",
            ),
            ("es", "Error en la función personalizada: %{actual}"),
            ("fr", "Échec de la fonction personnalisée : %{actual}"),
            ("ja", "カスタム関数が失敗しました: %{actual}"),
            ("ko", "사용자 정의 함수 실패: %{actual}"),
        ]),
    );
}

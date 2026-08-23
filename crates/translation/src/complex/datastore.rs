use shareable_string::SharedStringTranslationMap;
use std::collections::HashMap;

/// Adds translations for datastore messages to the provided translation map.
pub(crate) fn add_datastore_translation_map(translation_map: &mut SharedStringTranslationMap) {
    translation_map.set_translation_key(
        "datastore_key_empty",
        HashMap::from([
            ("en", "Invalid key: key cannot be empty"),
            ("zh", "无效键：键不能为空"),
            (
                "de",
                "Ungültiger Schlüssel: Der Schlüssel darf nicht leer sein",
            ),
            ("es", "Clave no válida: la clave no puede estar vacía"),
            ("fr", "Clé non valide : la clé ne peut pas être vide"),
            ("ja", "無効なキー: キーを空にすることはできません"),
            ("ko", "잘못된 키: 키는 비워둘 수 없습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "datastore_key_invalid_character",
        HashMap::from([
            (
                "en",
                "Invalid key: '%{key}'. Keys must only contain a-z, 0-9 and _",
            ),
            ("zh", "无效键：“%{key}”。键只能包含 a-z、0-9 和 _"),
            (
                "de",
                "Ungültiger Schlüssel: '%{key}'. Schlüssel dürfen nur a-z, 0-9 und _ enthalten",
            ),
            (
                "es",
                "Clave no válida: '%{key}'. Las claves solo pueden contener a-z, 0-9 y _",
            ),
            (
                "fr",
                "Clé non valide : '%{key}'. Les clés ne peuvent contenir que a-z, 0-9 et _",
            ),
            (
                "ja",
                "無効なキー: '%{key}'。キーには a-z、0-9、_ のみを使用できます",
            ),
            (
                "ko",
                "잘못된 키: '%{key}'. 키에는 a-z, 0-9 및 _만 사용할 수 있습니다",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "datastore_key_invalid_prefix",
        HashMap::from([
            (
                "en",
                "Invalid key: '%{key}'. Key is missing the required prefix",
            ),
            ("zh", "无效键：“%{key}”。键缺少必需的前缀"),
            (
                "de",
                "Ungültiger Schlüssel: '%{key}'. Dem Schlüssel fehlt das erforderliche Präfix",
            ),
            (
                "es",
                "Clave no válida: '%{key}'. Falta el prefijo obligatorio en la clave",
            ),
            (
                "fr",
                "Clé non valide : '%{key}'. Il manque le préfixe requis à la clé",
            ),
            ("ja", "無効なキー: '%{key}'。必要な接頭辞がありません"),
            ("ko", "잘못된 키: '%{key}'. 필요한 접두사가 없습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "datastore_key_conflict",
        HashMap::from([
            ("en", "Key conflict: %{key}"),
            ("zh", "键冲突：%{key}"),
            ("de", "Schlüsselkonflikt: %{key}"),
            ("es", "Conflicto de clave: %{key}"),
            ("fr", "Conflit de clé : %{key}"),
            ("ja", "キーの競合: %{key}"),
            ("ko", "키 충돌: %{key}"),
        ]),
    );
    translation_map.set_translation_key(
        "datastore_key_reserved",
        HashMap::from([
            ("en", "Key reserved: %{key}"),
            ("zh", "键已保留：%{key}"),
            ("de", "Schlüssel reserviert: %{key}"),
            ("es", "Clave reservada: %{key}"),
            ("fr", "Clé réservée : %{key}"),
            ("ja", "キーは予約済みです: %{key}"),
            ("ko", "예약된 키: %{key}"),
        ]),
    );
    translation_map.set_translation_key(
        "datastore_key_not_found",
        HashMap::from([
            ("en", "Key not found"),
            ("zh", "未找到键"),
            ("de", "Schlüssel nicht gefunden"),
            ("es", "Clave no encontrada"),
            ("fr", "Clé introuvable"),
            ("ja", "キーが見つかりません"),
            ("ko", "키를 찾을 수 없습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "datastore_index_not_found",
        HashMap::from([
            ("en", "Index not found"),
            ("zh", "未找到索引"),
            ("de", "Index nicht gefunden"),
            ("es", "Índice no encontrado"),
            ("fr", "Index introuvable"),
            ("ja", "インデックスが見つかりません"),
            ("ko", "인덱스를 찾을 수 없습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "datastore_schema_mismatch",
        HashMap::from([
            ("en", "Map items must all use the same entry schema"),
            ("zh", "映射项必须全部使用相同的条目架构"),
            (
                "de",
                "Alle Map-Elemente müssen dasselbe Eintragsschema verwenden",
            ),
            (
                "es",
                "Todos los elementos del mapa deben usar el mismo esquema de entrada",
            ),
            (
                "fr",
                "Tous les éléments de la map doivent utiliser le même schéma d’entrée",
            ),
            (
                "ja",
                "マップ項目はすべて同じエントリスキーマを使用する必要があります",
            ),
            ("ko", "맵 항목은 모두 동일한 항목 스키마를 사용해야 합니다"),
        ]),
    );
    translation_map.set_translation_key(
        "datastore_missing_schema",
        HashMap::from([
            ("en", "Missing schema"),
            ("zh", "缺少架构"),
            ("de", "Schema fehlt"),
            ("es", "Falta el esquema"),
            ("fr", "Schéma manquant"),
            ("ja", "スキーマがありません"),
            ("ko", "스키마가 없습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "datastore_map_value_set_not_supported",
        HashMap::from([
            (
                "en",
                "Cannot set a value for a map item directly. Use the appropriate methods to modify the map.",
            ),
            (
                "zh",
                "不能直接为映射项设置值。请使用相应的方法修改映射。",
            ),
            (
                "de",
                "Ein Wert für ein Map-Element kann nicht direkt gesetzt werden. Verwenden Sie die entsprechenden Methoden, um die Map zu ändern.",
            ),
            (
                "es",
                "No se puede establecer directamente un valor para un elemento del mapa. Use los métodos adecuados para modificar el mapa.",
            ),
            (
                "fr",
                "Impossible de définir directement une valeur pour un élément de la map. Utilisez les méthodes appropriées pour modifier la map.",
            ),
            (
                "ja",
                "マップ項目の値を直接設定することはできません。マップを変更するには適切なメソッドを使用してください。",
            ),
            (
                "ko",
                "맵 항목의 값을 직접 설정할 수 없습니다. 맵을 수정하려면 적절한 메서드를 사용하세요.",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "datastore_tab_or_separator_value_set_not_supported",
        HashMap::from([
            ("en", "Cannot set a value for a tab or separator item."),
            ("zh", "不能为选项卡或分隔符项设置值。"),
            (
                "de",
                "Für ein Registerkarten- oder Trennelement kann kein Wert gesetzt werden.",
            ),
            (
                "es",
                "No se puede establecer un valor para un elemento de pestaña o separador.",
            ),
            (
                "fr",
                "Impossible de définir une valeur pour un élément d’onglet ou de séparateur.",
            ),
            ("ja", "タブまたは区切り項目に値を設定することはできません。"),
            ("ko", "탭 또는 구분 기호 항목의 값을 설정할 수 없습니다."),
        ]),
    );
}

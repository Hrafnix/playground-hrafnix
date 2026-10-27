use hrafnix_shareable_string::SharedStringTranslationMap;
use std::collections::HashMap;

/// Adds translations for file diagnostics to the provided translation map.
pub(crate) fn add_files_translation_map(translation_map: &mut SharedStringTranslationMap) {
    translation_map.set_translation_key(
        "files_io_replaced_sync_failed",
        HashMap::from([
            ("en", "File '%{path}' has been replaced, but syncing its parent directory failed; persistence across a crash is uncertain"),
            ("zh", "文件“%{path}”已被替换，但其父目录同步失败；无法确定崩溃后数据是否持久保存"),
            ("de", "Datei '%{path}' wurde ersetzt, aber das Synchronisieren des übergeordneten Verzeichnisses ist fehlgeschlagen; die dauerhafte Speicherung bei einem Absturz ist ungewiss"),
            ("es", "El archivo '%{path}' se ha reemplazado, pero no se pudo sincronizar su directorio padre; no se garantiza la persistencia en caso de fallo"),
            ("fr", "Le fichier '%{path}' a été remplacé, mais la synchronisation de son répertoire parent a échoué ; la persistance en cas de panne est incertaine"),
            ("ja", "ファイル '%{path}' は置き換えられましたが、親ディレクトリの同期に失敗しました。クラッシュ後のデータ保持は保証できません"),
            ("ko", "파일 '%{path}'이(가) 교체되었지만 상위 디렉터리 동기화에 실패했습니다. 충돌 후 데이터 보존은 확실하지 않습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_error",
        HashMap::from([
            ("en", "File I/O error for '%{path}'"),
            ("zh", "文件“%{path}”发生输入/输出错误"),
            ("de", "Datei-E/A-Fehler bei '%{path}'"),
            ("es", "Error de E/S en el archivo '%{path}'"),
            ("fr", "Erreur d’entrée/sortie du fichier '%{path}'"),
            ("ja", "ファイル '%{path}' の入出力エラー"),
            ("ko", "파일 '%{path}'의 입출력 오류"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_not_found",
        HashMap::from([
            ("en", "File or directory not found: '%{path}'"),
            ("zh", "找不到文件或目录：“%{path}”"),
            ("de", "Datei oder Verzeichnis nicht gefunden: '%{path}'"),
            ("es", "No se encuentra el archivo o directorio: '%{path}'"),
            ("fr", "Fichier ou répertoire introuvable : '%{path}'"),
            (
                "ja",
                "ファイルまたはディレクトリが見つかりません: '%{path}'",
            ),
            ("ko", "파일 또는 디렉터리를 찾을 수 없습니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_permission_denied",
        HashMap::from([
            ("en", "Permission denied for '%{path}'"),
            ("zh", "没有访问“%{path}”的权限"),
            ("de", "Zugriff auf '%{path}' verweigert"),
            ("es", "Permiso denegado para '%{path}'"),
            ("fr", "Accès refusé à '%{path}'"),
            ("ja", "'%{path}' へのアクセスが拒否されました"),
            ("ko", "'%{path}'에 대한 접근 권한이 없습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_not_a_file",
        HashMap::from([
            ("en", "Path is not a file: '%{path}'"),
            ("zh", "路径不是文件：“%{path}”"),
            ("de", "Pfad ist keine Datei: '%{path}'"),
            ("es", "La ruta no es un archivo: '%{path}'"),
            ("fr", "Le chemin n’est pas un fichier : '%{path}'"),
            ("ja", "パスはファイルではありません: '%{path}'"),
            ("ko", "경로가 파일이 아닙니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_connection_refused",
        HashMap::from([
            ("en", "Connection refused while accessing '%{path}'"),
            ("zh", "访问“%{path}”时连接被拒绝"),
            ("de", "Verbindung beim Zugriff auf '%{path}' verweigert"),
            ("es", "Conexión rechazada al acceder a '%{path}'"),
            ("fr", "Connexion refusée lors de l’accès à '%{path}'"),
            ("ja", "'%{path}' へのアクセス中に接続が拒否されました"),
            ("ko", "'%{path}'에 접근하는 동안 연결이 거부되었습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_connection_reset",
        HashMap::from([
            ("en", "Connection reset while accessing '%{path}'"),
            ("zh", "访问“%{path}”时连接被重置"),
            ("de", "Verbindung beim Zugriff auf '%{path}' zurückgesetzt"),
            (
                "es",
                "Conexión restablecida por el servidor al acceder a '%{path}'",
            ),
            ("fr", "Connexion réinitialisée lors de l’accès à '%{path}'"),
            ("ja", "'%{path}' へのアクセス中に接続がリセットされました"),
            ("ko", "'%{path}'에 접근하는 동안 연결이 재설정되었습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_host_unreachable",
        HashMap::from([
            ("en", "Remote host is unreachable while accessing '%{path}'"),
            ("zh", "访问“%{path}”时无法连接远程主机"),
            (
                "de",
                "Entfernter Rechner beim Zugriff auf '%{path}' nicht erreichbar",
            ),
            (
                "es",
                "No se puede acceder al host remoto al acceder a '%{path}'",
            ),
            (
                "fr",
                "Hôte distant inaccessible lors de l’accès à '%{path}'",
            ),
            (
                "ja",
                "'%{path}' へのアクセス中にリモートホストに到達できません",
            ),
            (
                "ko",
                "'%{path}'에 접근하는 동안 원격 호스트에 연결할 수 없습니다",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_network_unreachable",
        HashMap::from([
            ("en", "Network is unreachable while accessing '%{path}'"),
            ("zh", "访问“%{path}”时网络不可达"),
            ("de", "Netzwerk beim Zugriff auf '%{path}' nicht erreichbar"),
            ("es", "La red es inaccesible al acceder a '%{path}'"),
            ("fr", "Réseau inaccessible lors de l’accès à '%{path}'"),
            (
                "ja",
                "'%{path}' へのアクセス中にネットワークに到達できません",
            ),
            (
                "ko",
                "'%{path}'에 접근하는 동안 네트워크에 연결할 수 없습니다",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_connection_aborted",
        HashMap::from([
            ("en", "Connection aborted while accessing '%{path}'"),
            ("zh", "访问“%{path}”时连接被中止"),
            ("de", "Verbindung beim Zugriff auf '%{path}' abgebrochen"),
            ("es", "Conexión interrumpida al acceder a '%{path}'"),
            ("fr", "Connexion interrompue lors de l’accès à '%{path}'"),
            ("ja", "'%{path}' へのアクセス中に接続が中断されました"),
            ("ko", "'%{path}'에 접근하는 동안 연결이 중단되었습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_not_connected",
        HashMap::from([
            ("en", "No network connection for '%{path}'"),
            ("zh", "“%{path}”没有网络连接"),
            ("de", "Keine Netzwerkverbindung für '%{path}'"),
            ("es", "No hay conexión de red para '%{path}'"),
            ("fr", "Aucune connexion réseau pour '%{path}'"),
            ("ja", "'%{path}' のネットワーク接続がありません"),
            ("ko", "'%{path}'에 대한 네트워크 연결이 없습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_address_in_use",
        HashMap::from([
            ("en", "Network address is already in use for '%{path}'"),
            ("zh", "“%{path}”的网络地址已被占用"),
            ("de", "Netzwerkadresse für '%{path}' wird bereits verwendet"),
            ("es", "La dirección de red ya está en uso para '%{path}'"),
            ("fr", "L’adresse réseau est déjà utilisée pour '%{path}'"),
            (
                "ja",
                "'%{path}' のネットワークアドレスは既に使用されています",
            ),
            ("ko", "'%{path}'의 네트워크 주소가 이미 사용 중입니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_address_not_available",
        HashMap::from([
            ("en", "Network address is not available for '%{path}'"),
            ("zh", "“%{path}”的网络地址不可用"),
            ("de", "Netzwerkadresse für '%{path}' ist nicht verfügbar"),
            (
                "es",
                "La dirección de red no está disponible para '%{path}'",
            ),
            ("fr", "L’adresse réseau n’est pas disponible pour '%{path}'"),
            ("ja", "'%{path}' のネットワークアドレスは利用できません"),
            ("ko", "'%{path}'의 네트워크 주소를 사용할 수 없습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_network_down",
        HashMap::from([
            ("en", "Network is down while accessing '%{path}'"),
            ("zh", "访问“%{path}”时网络已断开"),
            ("de", "Netzwerk beim Zugriff auf '%{path}' ausgefallen"),
            ("es", "La red está caída al acceder a '%{path}'"),
            ("fr", "Réseau hors service lors de l’accès à '%{path}'"),
            (
                "ja",
                "'%{path}' へのアクセス中にネットワークが停止しています",
            ),
            ("ko", "'%{path}'에 접근하는 동안 네트워크가 중단되었습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_broken_pipe",
        HashMap::from([
            ("en", "Pipe closed while accessing '%{path}'"),
            ("zh", "访问“%{path}”时管道已关闭"),
            ("de", "Pipe beim Zugriff auf '%{path}' geschlossen"),
            ("es", "La tubería se cerró al acceder a '%{path}'"),
            ("fr", "Tube fermé lors de l’accès à '%{path}'"),
            ("ja", "'%{path}' へのアクセス中にパイプが閉じられました"),
            ("ko", "'%{path}'에 접근하는 동안 파이프가 닫혔습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_already_exists",
        HashMap::from([
            ("en", "File or directory already exists: '%{path}'"),
            ("zh", "文件或目录已存在：“%{path}”"),
            ("de", "Datei oder Verzeichnis existiert bereits: '%{path}'"),
            ("es", "El archivo o directorio ya existe: '%{path}'"),
            ("fr", "Le fichier ou répertoire existe déjà : '%{path}'"),
            (
                "ja",
                "ファイルまたはディレクトリが既に存在します: '%{path}'",
            ),
            ("ko", "파일 또는 디렉터리가 이미 존재합니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_would_block",
        HashMap::from([
            ("en", "File operation would block: '%{path}'"),
            ("zh", "文件操作会阻塞：“%{path}”"),
            ("de", "Dateizugriff würde blockieren: '%{path}'"),
            ("es", "La operación de archivo se bloquearía: '%{path}'"),
            ("fr", "L’opération de fichier serait bloquante : '%{path}'"),
            ("ja", "ファイル操作はブロックされます: '%{path}'"),
            ("ko", "파일 작업이 대기 상태가 됩니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_not_a_directory",
        HashMap::from([
            ("en", "A path component is not a directory: '%{path}'"),
            ("zh", "路径中的某个部分不是目录：“%{path}”"),
            ("de", "Ein Pfadbestandteil ist kein Verzeichnis: '%{path}'"),
            (
                "es",
                "Un componente de la ruta no es un directorio: '%{path}'",
            ),
            (
                "fr",
                "Un composant du chemin n’est pas un répertoire : '%{path}'",
            ),
            ("ja", "パスの一部がディレクトリではありません: '%{path}'"),
            ("ko", "경로의 일부가 디렉터리가 아닙니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_directory_not_empty",
        HashMap::from([
            ("en", "Directory is not empty: '%{path}'"),
            ("zh", "目录不为空：“%{path}”"),
            ("de", "Verzeichnis ist nicht leer: '%{path}'"),
            ("es", "El directorio no está vacío: '%{path}'"),
            ("fr", "Le répertoire n’est pas vide : '%{path}'"),
            ("ja", "ディレクトリが空ではありません: '%{path}'"),
            ("ko", "디렉터리가 비어 있지 않습니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_read_only_filesystem",
        HashMap::from([
            ("en", "Cannot modify '%{path}': the filesystem is read-only"),
            ("zh", "无法修改“%{path}”：文件系统为只读"),
            (
                "de",
                "'%{path}' kann nicht geändert werden: Das Dateisystem ist schreibgeschützt",
            ),
            (
                "es",
                "No se puede modificar '%{path}': el sistema de archivos es de solo lectura",
            ),
            (
                "fr",
                "Impossible de modifier '%{path}' : le système de fichiers est en lecture seule",
            ),
            (
                "ja",
                "'%{path}' を変更できません: ファイルシステムは読み取り専用です",
            ),
            (
                "ko",
                "'%{path}'을(를) 수정할 수 없습니다: 파일 시스템이 읽기 전용입니다",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_stale_network_file_handle",
        HashMap::from([
            ("en", "Network file handle is no longer valid: '%{path}'"),
            ("zh", "网络文件句柄已失效：“%{path}”"),
            (
                "de",
                "Netzwerk-Dateihandle ist nicht mehr gültig: '%{path}'",
            ),
            (
                "es",
                "El identificador de archivo de red ya no es válido: '%{path}'",
            ),
            (
                "fr",
                "Le descripteur de fichier réseau n’est plus valide : '%{path}'",
            ),
            (
                "ja",
                "ネットワークファイルのハンドルが無効になっています: '%{path}'",
            ),
            (
                "ko",
                "네트워크 파일 핸들이 더 이상 유효하지 않습니다: '%{path}'",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_invalid_input",
        HashMap::from([
            ("en", "Invalid path or file operation argument: '%{path}'"),
            ("zh", "路径或文件操作参数无效：“%{path}”"),
            (
                "de",
                "Ungültiger Pfad oder ungültiges Argument für den Dateizugriff: '%{path}'",
            ),
            (
                "es",
                "Ruta o argumento de operación de archivo no válido: '%{path}'",
            ),
            (
                "fr",
                "Chemin ou argument d’opération de fichier non valide : '%{path}'",
            ),
            ("ja", "パスまたはファイル操作の引数が無効です: '%{path}'"),
            ("ko", "경로 또는 파일 작업 인수가 잘못되었습니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_invalid_data",
        HashMap::from([
            ("en", "Invalid file data: '%{path}'"),
            ("zh", "文件数据无效：“%{path}”"),
            ("de", "Ungültige Dateidaten: '%{path}'"),
            ("es", "Datos de archivo no válidos: '%{path}'"),
            ("fr", "Données de fichier non valides : '%{path}'"),
            ("ja", "ファイルのデータが無効です: '%{path}'"),
            ("ko", "파일 데이터가 잘못되었습니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_timed_out",
        HashMap::from([
            ("en", "File operation timed out: '%{path}'"),
            ("zh", "文件操作超时：“%{path}”"),
            ("de", "Zeitüberschreitung beim Dateizugriff: '%{path}'"),
            (
                "es",
                "Se agotó el tiempo de la operación de archivo: '%{path}'",
            ),
            (
                "fr",
                "Le délai de l’opération de fichier a expiré : '%{path}'",
            ),
            ("ja", "ファイル操作がタイムアウトしました: '%{path}'"),
            ("ko", "파일 작업 시간이 초과되었습니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_write_zero",
        HashMap::from([
            ("en", "Could not write all data to '%{path}'"),
            ("zh", "无法将所有数据写入“%{path}”"),
            (
                "de",
                "Nicht alle Daten konnten nach '%{path}' geschrieben werden",
            ),
            ("es", "No se pudieron escribir todos los datos en '%{path}'"),
            (
                "fr",
                "Impossible d’écrire toutes les données dans '%{path}'",
            ),
            ("ja", "'%{path}' にすべてのデータを書き込めませんでした"),
            ("ko", "'%{path}'에 모든 데이터를 쓸 수 없습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_storage_full",
        HashMap::from([
            ("en", "Cannot write to '%{path}': storage is full"),
            ("zh", "无法写入“%{path}”：存储空间已满"),
            (
                "de",
                "'%{path}' kann nicht geschrieben werden: Der Speicher ist voll",
            ),
            (
                "es",
                "No se puede escribir en '%{path}': el almacenamiento está lleno",
            ),
            (
                "fr",
                "Impossible d’écrire dans '%{path}' : l’espace de stockage est plein",
            ),
            ("ja", "'%{path}' に書き込めません: ストレージがいっぱいです"),
            (
                "ko",
                "'%{path}'에 쓸 수 없습니다: 저장 공간이 가득 찼습니다",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_not_seekable",
        HashMap::from([
            ("en", "Cannot seek within '%{path}'"),
            ("zh", "无法在“%{path}”中定位"),
            (
                "de",
                "Positionierung innerhalb von '%{path}' ist nicht möglich",
            ),
            ("es", "No se puede cambiar la posición dentro de '%{path}'"),
            ("fr", "Impossible de se déplacer dans '%{path}'"),
            ("ja", "'%{path}' 内でシークできません"),
            ("ko", "'%{path}' 내에서 위치를 이동할 수 없습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_quota_exceeded",
        HashMap::from([
            ("en", "Resource quota exceeded for '%{path}'"),
            ("zh", "“%{path}”的资源配额已超出"),
            ("de", "Ressourcenkontingent für '%{path}' überschritten"),
            ("es", "Se superó la cuota de recursos para '%{path}'"),
            ("fr", "Quota de ressources dépassé pour '%{path}'"),
            ("ja", "'%{path}' のリソース使用量が割り当てを超えています"),
            ("ko", "'%{path}'의 리소스 할당량을 초과했습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_file_too_large",
        HashMap::from([
            ("en", "File exceeds the supported size limit: '%{path}'"),
            ("zh", "文件超出支持的大小限制：“%{path}”"),
            (
                "de",
                "Die Datei überschreitet die unterstützte Größenbegrenzung: '%{path}'",
            ),
            (
                "es",
                "El archivo supera el límite de tamaño admitido: '%{path}'",
            ),
            (
                "fr",
                "Le fichier dépasse la taille maximale prise en charge : '%{path}'",
            ),
            (
                "ja",
                "ファイルがサポートされるサイズ制限を超えています: '%{path}'",
            ),
            ("ko", "파일이 지원되는 크기 제한을 초과합니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_resource_busy",
        HashMap::from([
            ("en", "File or resource is busy: '%{path}'"),
            ("zh", "文件或资源正忙：“%{path}”"),
            ("de", "Datei oder Ressource ist belegt: '%{path}'"),
            ("es", "El archivo o recurso está ocupado: '%{path}'"),
            ("fr", "Le fichier ou la ressource est occupé : '%{path}'"),
            ("ja", "ファイルまたはリソースが使用中です: '%{path}'"),
            ("ko", "파일 또는 리소스가 사용 중입니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_executable_file_busy",
        HashMap::from([
            (
                "en",
                "Cannot modify an executable that is running: '%{path}'",
            ),
            ("zh", "无法修改正在运行的可执行文件：“%{path}”"),
            (
                "de",
                "Eine laufende ausführbare Datei kann nicht geändert werden: '%{path}'",
            ),
            (
                "es",
                "No se puede modificar un ejecutable en ejecución: '%{path}'",
            ),
            (
                "fr",
                "Impossible de modifier un exécutable en cours d’exécution : '%{path}'",
            ),
            ("ja", "実行中の実行可能ファイルは変更できません: '%{path}'"),
            ("ko", "실행 중인 실행 파일을 수정할 수 없습니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_deadlock",
        HashMap::from([
            ("en", "File locking would cause a deadlock: '%{path}'"),
            ("zh", "文件锁定会导致死锁：“%{path}”"),
            (
                "de",
                "Die Dateisperre würde eine Verklemmung verursachen: '%{path}'",
            ),
            (
                "es",
                "El bloqueo del archivo causaría un interbloqueo: '%{path}'",
            ),
            (
                "fr",
                "Le verrouillage du fichier provoquerait un interblocage : '%{path}'",
            ),
            (
                "ja",
                "ファイルのロックによりデッドロックが発生します: '%{path}'",
            ),
            ("ko", "파일 잠금으로 인해 교착 상태가 발생합니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_crosses_devices",
        HashMap::from([
            ("en", "Operation crosses devices or filesystems: '%{path}'"),
            ("zh", "操作跨越设备或文件系统：“%{path}”"),
            (
                "de",
                "Der Vorgang überschreitet Geräte- oder Dateisystemgrenzen: '%{path}'",
            ),
            (
                "es",
                "La operación cruza dispositivos o sistemas de archivos: '%{path}'",
            ),
            (
                "fr",
                "L’opération traverse plusieurs périphériques ou systèmes de fichiers : '%{path}'",
            ),
            (
                "ja",
                "操作がデバイスまたはファイルシステムの境界を越えています: '%{path}'",
            ),
            (
                "ko",
                "작업이 장치 또는 파일 시스템 경계를 넘습니다: '%{path}'",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_too_many_links",
        HashMap::from([
            ("en", "Too many hard links to '%{path}'"),
            ("zh", "指向“%{path}”的硬链接过多"),
            ("de", "Zu viele Hardlinks auf '%{path}'"),
            ("es", "Demasiados enlaces físicos a '%{path}'"),
            ("fr", "Trop de liens physiques vers '%{path}'"),
            ("ja", "'%{path}' へのハードリンクが多すぎます"),
            ("ko", "'%{path}'에 대한 하드 링크가 너무 많습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_invalid_filename",
        HashMap::from([
            ("en", "Filename is invalid or too long: '%{path}'"),
            ("zh", "文件名无效或过长：“%{path}”"),
            ("de", "Dateiname ist ungültig oder zu lang: '%{path}'"),
            (
                "es",
                "El nombre de archivo no es válido o es demasiado largo: '%{path}'",
            ),
            (
                "fr",
                "Le nom de fichier est non valide ou trop long : '%{path}'",
            ),
            ("ja", "ファイル名が無効または長すぎます: '%{path}'"),
            ("ko", "파일 이름이 잘못되었거나 너무 깁니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_argument_list_too_long",
        HashMap::from([
            ("en", "Program argument list is too long for '%{path}'"),
            ("zh", "“%{path}”的程序参数列表过长"),
            (
                "de",
                "Die Liste der Programmargumente für '%{path}' ist zu lang",
            ),
            (
                "es",
                "La lista de argumentos del programa es demasiado larga para '%{path}'",
            ),
            (
                "fr",
                "La liste des arguments du programme est trop longue pour '%{path}'",
            ),
            ("ja", "'%{path}' のプログラム引数リストが長すぎます"),
            ("ko", "'%{path}'의 프로그램 인수 목록이 너무 깁니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_interrupted",
        HashMap::from([
            ("en", "File operation was interrupted: '%{path}'"),
            ("zh", "文件操作被中断：“%{path}”"),
            ("de", "Dateizugriff wurde unterbrochen: '%{path}'"),
            ("es", "Se interrumpió la operación de archivo: '%{path}'"),
            ("fr", "L’opération de fichier a été interrompue : '%{path}'"),
            ("ja", "ファイル操作が中断されました: '%{path}'"),
            ("ko", "파일 작업이 중단되었습니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_unsupported",
        HashMap::from([
            ("en", "File operation is not supported: '%{path}'"),
            ("zh", "不支持此文件操作：“%{path}”"),
            ("de", "Dateizugriff wird nicht unterstützt: '%{path}'"),
            ("es", "La operación de archivo no es compatible: '%{path}'"),
            (
                "fr",
                "L’opération de fichier n’est pas prise en charge : '%{path}'",
            ),
            ("ja", "ファイル操作はサポートされていません: '%{path}'"),
            ("ko", "지원되지 않는 파일 작업입니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_unexpected_eof",
        HashMap::from([
            ("en", "Unexpected end of file: '%{path}'"),
            ("zh", "文件意外结束：“%{path}”"),
            ("de", "Unerwartetes Dateiende: '%{path}'"),
            ("es", "Fin de archivo inesperado: '%{path}'"),
            ("fr", "Fin de fichier inattendue : '%{path}'"),
            ("ja", "予期しないファイルの終端です: '%{path}'"),
            ("ko", "예기치 않은 파일 끝입니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_out_of_memory",
        HashMap::from([
            ("en", "Not enough memory for file operation: '%{path}'"),
            ("zh", "内存不足，无法执行文件操作：“%{path}”"),
            (
                "de",
                "Nicht genügend Arbeitsspeicher für den Dateizugriff: '%{path}'",
            ),
            (
                "es",
                "No hay suficiente memoria para la operación de archivo: '%{path}'",
            ),
            (
                "fr",
                "Mémoire insuffisante pour l’opération de fichier : '%{path}'",
            ),
            (
                "ja",
                "ファイル操作に必要なメモリが不足しています: '%{path}'",
            ),
            ("ko", "파일 작업을 위한 메모리가 부족합니다: '%{path}'"),
        ]),
    );
    translation_map.set_translation_key(
        "files_io_other",
        HashMap::from([
            ("en", "Other file I/O error for '%{path}'"),
            ("zh", "文件“%{path}”发生其他输入/输出错误"),
            ("de", "Sonstiger Datei-E/A-Fehler bei '%{path}'"),
            ("es", "Otro error de E/S en el archivo '%{path}'"),
            ("fr", "Autre erreur d’entrée/sortie du fichier '%{path}'"),
            ("ja", "ファイル '%{path}' のその他の入出力エラー"),
            ("ko", "파일 '%{path}'의 기타 입출력 오류"),
        ]),
    );
    translation_map.set_translation_key(
        "files_json_syntax",
        HashMap::from([
            (
                "en",
                "'%{path}': Invalid JSON syntax at line %{line}, column %{column}",
            ),
            (
                "zh",
                "“%{path}”：第 %{line} 行第 %{column} 列的 JSON 语法无效",
            ),
            (
                "de",
                "'%{path}': Ungültige JSON-Syntax in Zeile %{line}, Spalte %{column}",
            ),
            (
                "es",
                "'%{path}': Sintaxis JSON no válida en la línea %{line}, columna %{column}",
            ),
            (
                "fr",
                "'%{path}' : Syntaxe JSON non valide à la ligne %{line}, colonne %{column}",
            ),
            (
                "ja",
                "'%{path}': %{line} 行 %{column} 列の JSON 構文が無効です",
            ),
            (
                "ko",
                "'%{path}': %{line}행 %{column}열의 JSON 구문이 잘못되었습니다",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "files_json_unexpected_eof",
        HashMap::from([
            ("en", "'%{path}': Unexpected end of JSON data at line %{line}, column %{column}"),
            ("zh", "“%{path}”：JSON 数据在第 %{line} 行第 %{column} 列意外结束"),
            ("de", "'%{path}': Unerwartetes Ende der JSON-Daten in Zeile %{line}, Spalte %{column}"),
            ("es", "'%{path}': Fin inesperado de los datos JSON en la línea %{line}, columna %{column}"),
            ("fr", "'%{path}' : Fin inattendue des données JSON à la ligne %{line}, colonne %{column}"),
            ("ja", "'%{path}': %{line} 行 %{column} 列で JSON データが予期せず終了しました"),
            ("ko", "'%{path}': %{line}행 %{column}열에서 JSON 데이터가 예기치 않게 끝났습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_json_data",
        HashMap::from([
            ("en", "'%{path}': JSON data does not match the expected structure at line %{line}, column %{column}"),
            ("zh", "“%{path}”：第 %{line} 行第 %{column} 列的 JSON 数据与预期结构不匹配"),
            ("de", "'%{path}': JSON-Daten entsprechen in Zeile %{line}, Spalte %{column} nicht der erwarteten Struktur"),
            ("es", "'%{path}': Los datos JSON no coinciden con la estructura esperada en la línea %{line}, columna %{column}"),
            ("fr", "'%{path}' : Les données JSON ne correspondent pas à la structure attendue à la ligne %{line}, colonne %{column}"),
            ("ja", "'%{path}': %{line} 行 %{column} 列の JSON データが期待される構造と一致しません"),
            ("ko", "'%{path}': %{line}행 %{column}열의 JSON 데이터가 예상 구조와 일치하지 않습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_json_serialization",
        HashMap::from([
            ("en", "'%{path}': Data could not be serialized as JSON"),
            ("zh", "“%{path}”：数据无法序列化为 JSON"),
            (
                "de",
                "'%{path}': Daten konnten nicht als JSON serialisiert werden",
            ),
            (
                "es",
                "'%{path}': No se pudieron serializar los datos como JSON",
            ),
            (
                "fr",
                "'%{path}' : Les données n’ont pas pu être sérialisées en JSON",
            ),
            (
                "ja",
                "'%{path}': データを JSON にシリアライズできませんでした",
            ),
            ("ko", "'%{path}': 데이터를 JSON으로 직렬화할 수 없습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_invalid_utf8",
        HashMap::from([
            ("en", "'%{path}': Invalid UTF-8 at byte %{position} (invalid sequence length: %{length})"),
            ("zh", "“%{path}”：字节 %{position} 处包含无效的 UTF-8（无效序列长度：%{length}）"),
            ("de", "'%{path}': Ungültiges UTF-8 bei Byte %{position} (Länge der ungültigen Sequenz: %{length})"),
            ("es", "'%{path}': UTF-8 no válido en el byte %{position} (longitud de la secuencia no válida: %{length})"),
            ("fr", "'%{path}' : UTF-8 invalide à l’octet %{position} (longueur de la séquence invalide : %{length})"),
            ("ja", "'%{path}': バイト位置 %{position} に無効な UTF-8 があります（無効なシーケンスの長さ: %{length}）"),
            ("ko", "'%{path}': 바이트 %{position}에 잘못된 UTF-8이 있습니다(잘못된 시퀀스 길이: %{length})"),
        ]),
    );
    translation_map.set_translation_key(
        "files_incomplete_utf8",
        HashMap::from([
            (
                "en",
                "'%{path}': Incomplete UTF-8 sequence at byte %{position}",
            ),
            (
                "zh",
                "“%{path}”：字节 %{position} 处包含不完整的 UTF-8 序列",
            ),
            (
                "de",
                "'%{path}': Unvollständige UTF-8-Sequenz bei Byte %{position}",
            ),
            (
                "es",
                "'%{path}': Secuencia UTF-8 incompleta en el byte %{position}",
            ),
            (
                "fr",
                "'%{path}' : Séquence UTF-8 incomplète à l’octet %{position}",
            ),
            (
                "ja",
                "'%{path}': バイト位置 %{position} に不完全な UTF-8 シーケンスがあります",
            ),
            (
                "ko",
                "'%{path}': 바이트 %{position}에 불완전한 UTF-8 시퀀스가 있습니다",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "files_missing_file_version",
        HashMap::from([
            ("en", "'%{path}': Missing top-level \"file-version\" entry"),
            ("zh", "“%{path}”：缺少顶层 \"file-version\" 条目"),
            (
                "de",
                "'%{path}': Der Eintrag \"file-version\" auf oberster Ebene fehlt",
            ),
            (
                "es",
                "'%{path}': Falta la entrada \"file-version\" de nivel superior",
            ),
            (
                "fr",
                "'%{path}' : L’entrée \"file-version\" de premier niveau est manquante",
            ),
            (
                "ja",
                "'%{path}': トップレベルの \"file-version\" エントリがありません",
            ),
            ("ko", "'%{path}': 최상위 \"file-version\" 항목이 없습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_duplicate_file_version",
        HashMap::from([
            (
                "en",
                "'%{path}': Duplicate top-level \"file-version\" entry",
            ),
            ("zh", "“%{path}”：顶层 \"file-version\" 条目重复"),
            (
                "de",
                "'%{path}': Doppelter Eintrag \"file-version\" auf oberster Ebene",
            ),
            (
                "es",
                "'%{path}': Entrada \"file-version\" de nivel superior duplicada",
            ),
            (
                "fr",
                "'%{path}' : Entrée \"file-version\" de premier niveau en double",
            ),
            (
                "ja",
                "'%{path}': トップレベルの \"file-version\" エントリが重複しています",
            ),
            (
                "ko",
                "'%{path}': 최상위 \"file-version\" 항목이 중복되었습니다",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "files_invalid_file_version",
        HashMap::from([
            ("en", "'%{path}': Invalid file-version: %{version}"),
            ("zh", "“%{path}”：无效的 file-version：%{version}"),
            ("de", "'%{path}': Ungültige file-version: %{version}"),
            ("es", "'%{path}': file-version no válida: %{version}"),
            ("fr", "'%{path}' : file-version non valide : %{version}"),
            ("ja", "'%{path}': 無効な file-version: %{version}"),
            ("ko", "'%{path}': 잘못된 file-version: %{version}"),
        ]),
    );
    translation_map.set_translation_key(
        "files_unsupported_file_version",
        HashMap::from([
            ("en", "'%{path}': Unsupported file-version: %{version}"),
            ("zh", "“%{path}”：不支持的 file-version：%{version}"),
            (
                "de",
                "'%{path}': Nicht unterstützte file-version: %{version}",
            ),
            ("es", "'%{path}': file-version no compatible: %{version}"),
            (
                "fr",
                "'%{path}' : file-version non prise en charge : %{version}",
            ),
            ("ja", "'%{path}': 未対応の file-version: %{version}"),
            ("ko", "'%{path}': 지원되지 않는 file-version: %{version}"),
        ]),
    );
    translation_map.set_translation_key(
        "files_expected_root",
        HashMap::from([
            (
                "en",
                "'%{path}': Expected a JSON object with a \"%{root}\" entry",
            ),
            ("zh", "“%{path}”：预期为包含 \"%{root}\" 条目的 JSON 对象"),
            (
                "de",
                "'%{path}': Ein JSON-Objekt mit einem Eintrag \"%{root}\" wurde erwartet",
            ),
            (
                "es",
                "'%{path}': Se esperaba un objeto JSON con una entrada \"%{root}\"",
            ),
            (
                "fr",
                "'%{path}' : Un objet JSON avec une entrée \"%{root}\" était attendu",
            ),
            (
                "ja",
                "'%{path}': \"%{root}\" エントリを持つ JSON オブジェクトが必要です",
            ),
            (
                "ko",
                "'%{path}': \"%{root}\" 항목이 있는 JSON 객체가 필요합니다",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "files_duplicate_root",
        HashMap::from([
            ("en", "'%{path}': Duplicate top-level \"%{root}\" entry"),
            ("zh", "“%{path}”：顶层 \"%{root}\" 条目重复"),
            (
                "de",
                "'%{path}': Doppelter Eintrag \"%{root}\" auf oberster Ebene",
            ),
            (
                "es",
                "'%{path}': Entrada \"%{root}\" de nivel superior duplicada",
            ),
            (
                "fr",
                "'%{path}' : Entrée \"%{root}\" de premier niveau en double",
            ),
            (
                "ja",
                "'%{path}': トップレベルの \"%{root}\" エントリが重複しています",
            ),
            ("ko", "'%{path}': 최상위 \"%{root}\" 항목이 중복되었습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "files_unknown_top_level_key",
        HashMap::from([
            ("en", "'%{path}': Unknown top-level entry \"%{key}\""),
            ("zh", "“%{path}”：未知的顶层条目 \"%{key}\""),
            (
                "de",
                "'%{path}': Unbekannter Eintrag \"%{key}\" auf oberster Ebene",
            ),
            (
                "es",
                "'%{path}': Entrada de nivel superior desconocida \"%{key}\"",
            ),
            (
                "fr",
                "'%{path}' : Entrée de premier niveau inconnue \"%{key}\"",
            ),
            (
                "ja",
                "'%{path}': 不明なトップレベルのエントリ \"%{key}\" があります",
            ),
            (
                "ko",
                "'%{path}': 알 수 없는 최상위 항목 \"%{key}\"이(가) 있습니다",
            ),
        ]),
    );
}

# fabrewk: referencia de comandos

Generada desde la declaración de comandos; lista exactamente los comandos que existen.

## `fabrewk status [<id>] [--json]`

Muestra la salud del registro, la política vigente y el estado de cada etapa y tarea.

- `<id>`: Una etapa o una tarea, por su identificador (s-… o t-…): la muestra con sus eventos.
- `--json`: Da la salida como un solo documento JSON con un campo schema.
- `--help`: Muestra esta ayuda.

Movimientos que puede causar: ninguno.

Documentos con --json: `fabrewk.status/1`, `fabrewk.subject/1`

## `fabrewk config [<key>] [--json]`

Muestra cada ajuste efectivo con su origen, los cambios no adoptados, las llaves, lo que tiene este equipo y cada control.

- `<key>`: Un solo ajuste, por su ruta de clave: muestra solo su valor y su origen.
- `--json`: Da la salida como un solo documento JSON con un campo schema.
- `--help`: Muestra esta ayuda.

Movimientos que puede causar: ninguno.

Documentos con --json: `fabrewk.config/1`, `fabrewk.value/1`

Rechazos

- `POLICY_ABSENT`: No hay una política adoptada, así que no hay configuración efectiva ni mantenedores.
- `SETTINGS_INVALID`: Un archivo de ajustes no se lee o no valida.
- `SETTINGS_KEY_NOT_PERSONAL`: Un archivo personal tiene una clave que solo el archivo del proyecto puede fijar.
- `ALIAS_MISMATCH`: El archivo personal nombra a otro mantenedor que su directorio.
- `PRIVATE_FOLDER_PERMISSIONS`: La carpeta privada da acceso a alguien más que su dueño, así que Fabrewk no la lee.
- `KEY_UNKNOWN`: Ningún ajuste está declarado en esta ruta de clave.

## `fabrewk check [--json]`

Verifica los ajustes, la política, el registro, el dominio, las credenciales y la carpeta privada.

- `--json`: Da la salida como un solo documento JSON con un campo schema.
- `--help`: Muestra esta ayuda.

Movimientos que puede causar: ninguno.

Documentos con --json: `fabrewk.check/1`

| | Verifica | Códigos | Control |
| --- | --- | --- | --- |
| C1 | Los archivos de ajustes se leen y validan, y el de cada mantenedor tiene solo claves personales. | `SETTINGS_INVALID`, `SETTINGS_KEY_NOT_PERSONAL`, `ALIAS_MISMATCH` | efectivo |
| C2 | Hay una política adoptada, y el archivo del proyecto es igual a ella o su cambio está pendiente. | `POLICY_ABSENT`, `POLICY_PENDING` | efectivo |
| C3 | El registro se reproduce: versiones, resúmenes, forma canónica, cadenas, bifurcaciones, transiciones, firmas, identificadores. | `RECORD_VERSION_UNKNOWN`, `RECORD_BROKEN`, `RECORD_DIGEST_MISMATCH`, `RECORD_NONCANONICAL`, `RECORD_FORK`, `RECORD_TRANSITION_INVALID`, `ATTEMPT_ENDED`, `RECORD_SIGNATURE_INVALID`, `RECORD_ID_COLLISION` | efectivo |
| C4 | El registro existe y no ha retrocedido: efectivo donde se recorre la historia de Git, cooperativo en un solo equipo. | `RECORD_ABSENT`, `RECORD_ROLLBACK`, `MARK_INVALID` | efectivo |
| C8 | Las entradas del dominio, los alias, su archivo de ignorados y la regla de ignorados del producto. | `DOMAIN_UNKNOWN_ENTRY`, `ALIAS_INVALID`, `GITIGNORE_CHANGED`, `DOMAIN_NOT_IGNORED` | efectivo |
| C9 | Ningún archivo parece una credencial: una heurística. | `CREDENTIAL_SUSPECTED` | cooperativo |
| C10 | Los archivos del registro están confirmados donde el modo de publicación los confirma. | `RECORD_UNCOMMITTED` | efectivo |
| C11 | La carpeta privada da acceso solo a su dueño. | `PRIVATE_FOLDER_PERMISSIONS` | efectivo |

## El programa solo

- `fabrewk --help`: Muestra esta ayuda.
- `fabrewk --version`: Muestra la versión de Fabrewk.

Documentos con --json: `fabrewk.refused/1`, `fabrewk.usage/1`, `fabrewk.internal/1`, `fabrewk.version/1`

## Estados de salida

- `0`: hecho.
- `1`: rechazado: se imprimió un código y no se escribió nada.
- `2`: error de uso: no se aceptó la línea de comandos.
- `3`: la verificación encontró un problema.
- `70`: error interno.

# Los comandos

Cada comando, con lo que hace. La [referencia de comandos](../../crates/fabrewk/doc/reference.es.md) se genera a
partir de la propia declaración de comandos del programa, así que enumera exactamente los comandos que existen, cada
uno con sus opciones, los movimientos del registro que puede causar, sus documentos JSON y sus rechazos; `fabrewk
<comando> --help` imprime su resumen, sus opciones, el formato de su archivo cuando recibe uno, y sus rechazos. Los comandos y las opciones están en inglés en todos los idiomas.

## Leer, sin escribir nada

| Comando | Qué hace |
| --- | --- |
| `fabrewk status [<id>]` | La salud del registro, la política vigente y el estado de cada etapa y tarea; con un identificador, esa etapa o tarea con sus eventos y el digest que cita una decisión sobre ella. |
| `fabrewk config [<clave>]` | Cada valor efectivo de la configuración con su origen, los cambios sin adoptar, las llaves de los mantenedores, lo que tiene este equipo y cada control. |
| `fabrewk check` | Verifica la configuración, la política, el registro, el dominio, las credenciales y la carpeta privada; sale con 3 ante un error, con 0 si solo hay advertencias. |
| `fabrewk assess [<opciones>]` | Evalúa el repositorio sin escribir nada: el veredicto, sus razones y el plan, con su digest y el comando `apply`. |

## La instalación

| Comando | Qué hace |
| --- | --- |
| `fabrewk onboard` | Hace las preguntas de la instalación en la terminal, muestra el plan y lo aplica con tu sí y tus palabras. |
| `fabrewk apply <plan> <opciones> --say <palabras>` | Un maestro aplica el plan que imprimió `assess`, con las mismas opciones. |

## Las decisiones de un maestro

Cada una lleva `--say '<palabras>'`, tus propias palabras, registradas tal como las escribes y firmadas con tu llave,
y opcionalmente `--say-lang <etiqueta>`, su idioma; cada una se niega dentro de un agente.

| Comando | Qué hace |
| --- | --- |
| `fabrewk adopt --say <palabras> [--widen <clave>]…` | Adopta el archivo del proyecto como la política vigente. |
| `fabrewk authorize <id> --seen <digest> --say <palabras> [--widen <clave>]… [--run]` | Autoriza una tarea, o una etapa y sus tareas, tal como las mostró `status`; con `--run`, la ejecuta de inmediato. |
| `fabrewk run <id> --say <palabras>` | Ejecuta, aislada, una tarea autorizada, o la siguiente de una etapa, ronda tras ronda, hasta una tarea verificada o una detención; termina la integración de una tarea aceptada. |
| `fabrewk accept <tarea> --seen <digest> --say <palabras>` | Acepta el candidato de una tarea tal como lo mostró `status` y lo integra. |
| `fabrewk reject <tarea> --say <palabras>` | Rechaza una tarea, diciendo por qué. |
| `fabrewk approve <propuesta> --say <palabras>` | Aprueba una propuesta y aplica su edición al alambique; `approve alambique` aprueba el alambique tal como está. |
| `fabrewk discard <propuesta> --say <palabras>` | Descarta una propuesta pendiente, diciendo por qué. |
| `fabrewk close <etapa> --say <palabras>` | Cierra una etapa cuyo trabajo terminó. |

## Registrar, sin conceder nada

| Comando | Qué hace |
| --- | --- |
| `fabrewk propose <archivo>` | Registra una etapa y sus tareas, o tareas para una etapa, desde un archivo JSON. |
| `fabrewk sweep <alias> --answer <archivo>` | Registra la respuesta del integrador al fermento de un mantenedor y la escribe debajo del separador. |

## El programa solo, y las extensiones

- `fabrewk --help` y `fabrewk --version` (con `--json`, la versión como documento).
- `fabrewk <nombre>` ejecuta la extensión `fabrewk-<nombre>` cuando la configuración lo permite; consulta
  [Trabajar con Fabrewk](working.md#extensiones).

## Salida y estados de salida

Todo comando integrado salvo `onboard` acepta `--json`, que imprime un documento JSON con un campo `schema`; un
rechazo con `--json` también es un documento, y `--help` sigue siendo texto. Los argumentos de una extensión son
suyos. Un rechazo imprime primero su código, por ejemplo `PLAN_CHANGED: …`, con la razón y
el siguiente paso.

| Estado | Significado |
| --- | --- |
| `0` | Hecho. |
| `1` | Rechazado: se imprimió un código, con su razón y el siguiente paso. Un rechazo antes de que se escriban los eventos del comando no deja nada escrito; uno posterior dice qué dejó y cómo se termina. |
| `2` | Error de uso: no se aceptó la línea de comandos. |
| `3` | La verificación encontró un problema. |
| `4` | La ejecución se detuvo y espera a una persona: la tarea quedó detenida. |
| `10` | La extensión no soporta el protocolo de esta versión de Fabrewk. |
| `70` | Error interno. |

## Autocompletado

El paquete de la versión trae archivos de autocompletado para fish y para bash (zsh con `bashcompinit`), generados
desde la misma declaración; [Instalar Fabrewk](installation.md#autocompletado) dice dónde ponerlos.

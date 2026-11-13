# fabrewk: referencia de comandos

Generada desde la declaración de comandos; lista exactamente los comandos que existen.

## `fabrewk status [<id>] [--json]`

Muestra la salud del registro, la política vigente y el estado de cada etapa y tarea.

- `<id>`: Una etapa o una tarea, por su identificador (s-… o t-…): la muestra con sus eventos.
- `--json`: Da la salida como un solo documento JSON con un campo schema.
- `--help`: Muestra esta ayuda.

Movimientos que puede causar: ninguno.

Documentos con --json: `fabrewk.status/3`, `fabrewk.subject/3`

## `fabrewk config [<key>] [--json]`

Muestra cada ajuste efectivo con su origen, los cambios no adoptados, las llaves, lo que tiene este equipo y cada control.

- `<key>`: Un solo ajuste, por su ruta de clave: muestra solo su valor y su origen.
- `--json`: Da la salida como un solo documento JSON con un campo schema.
- `--help`: Muestra esta ayuda.

Movimientos que puede causar: ninguno.

Documentos con --json: `fabrewk.config/2`, `fabrewk.value/1`

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
| C5 | STATUS.md y RULES.md son lo que da una generación nueva a partir del registro. | `VIEW_STALE` | efectivo |
| C6 | Cada fermento tiene su separador una sola vez fuera de bloques de código, dentro de una versión que Fabrewk escribe. | `SEPARATOR_MISSING`, `SEPARATOR_AMBIGUOUS`, `SEPARATOR_DAMAGED` | efectivo |
| C7 | El alambique es el texto que un maestro aprobó por última vez. | `ALAMBIQUE_UNAPPROVED` | efectivo |
| C8 | Las entradas del dominio, los alias, su archivo de ignorados y la regla de ignorados del producto. | `DOMAIN_UNKNOWN_ENTRY`, `ALIAS_INVALID`, `GITIGNORE_CHANGED`, `DOMAIN_NOT_IGNORED` | efectivo |
| C9 | Ningún archivo parece una credencial: una heurística. | `CREDENTIAL_SUSPECTED` | cooperativo |
| C10 | Los archivos del registro están confirmados donde el modo de publicación los confirma. | `RECORD_UNCOMMITTED` | efectivo |
| C11 | La carpeta privada da acceso solo a su dueño. | `PRIVATE_FOLDER_PERMISSIONS` | efectivo |
| C12 | Cada directorio de mantenedor tiene un fermento y está listado en la política. | `MAINTAINER_UNLISTED`, `FERMENTO_MISSING` | efectivo |

## `fabrewk adopt --say <words> [--say-lang <tag>] [--widen <key>]… [--json]`

Un maestro adopta el archivo del proyecto como la política vigente, nombrando cada ampliación que hace.

- `--say <words>`: Tus propias palabras para esta decisión, guardadas tal cual y firmadas. Se rechazan las copiadas de un ejemplo, como «Estoy de acuerdo.».
- `--say-lang <tag>`: La etiqueta de idioma de tus palabras; por omisión, tu languages.agentInteraction.
- `--widen <key>`: Nombra una ampliación de autoridad que hace este movimiento, por su ruta de clave; una vez por cada una.
- `--json`: Da la salida como un solo documento JSON con un campo schema.
- `--help`: Muestra esta ayuda.

Movimientos que puede causar: `policy-adopted` (política adoptada).

Documentos con --json: `fabrewk.adopted/1`

Rechazos

- `AGENT_ENVIRONMENT`: Esto corre dentro de un agente o una extensión, cuyos marcadores están en el entorno, y solo una persona hace este movimiento. El control es cooperativo: un proceso puede limpiar su entorno.
- `ROLE_FORBIDDEN`: La llave que firma este movimiento no tiene en la política ningún rol que pueda hacerlo.
- `WORDS_MISSING`: Un movimiento de persona necesita tus propias palabras, dadas con --say: no vacías y en texto UTF-8 válido.
- `WORDS_COPIED`: Las palabras dadas son el ejemplo que imprime la ayuda, no las tuyas.
- `SETTINGS_INVALID`: Un archivo de ajustes no se lee o no valida.
- `SETTINGS_KEY_NOT_PERSONAL`: Un archivo personal tiene una clave que solo el archivo del proyecto puede fijar.
- `ALIAS_MISMATCH`: El archivo personal nombra a otro mantenedor que su directorio.
- `ROLE_NOT_ENFORCED`: La política lista un supervisor, un rol que Fabrewk aún no hace cumplir.
- `POLICY_UNCOMMITTED`: El archivo del proyecto no está como se confirmó en el head de la rama del registro.
- `WIDENING_UNNAMED`: Un cambio amplía la autoridad sin nombrarse.
- `RECORD_ABSENT`: Aquí no hay registro.
- `POLICY_ABSENT`: No hay una política adoptada, así que no hay configuración efectiva ni mantenedores.
- `RECORD_BROKEN`: El registro está roto: sus archivos no forman una sola cadena sin cortes.
- `RECORD_DIGEST_MISMATCH`: El contenido de un archivo de evento no es el que su nombre registra.
- `RECORD_NONCANONICAL`: Un archivo de evento no está en forma canónica, así que no se puede confiar en su resumen.
- `RECORD_FORK`: Dos eventos ocupan el mismo lugar de un flujo: dos clones escribieron ahí.
- `RECORD_TRANSITION_INVALID`: Un evento está donde la tabla de transiciones no lo permite.
- `ATTEMPT_ENDED`: El registro termina un intento que no está abierto.
- `RECORD_SIGNATURE_INVALID`: El evento de una persona no está firmado con la llave y el rol que la política da a quien firma.
- `RECORD_VERSION_UNKNOWN`: Un evento no es de un esquema que esta versión de Fabrewk conozca.
- `RECORD_ID_COLLISION`: Dos etapas o dos tareas recibieron el mismo identificador.
- `RECORD_ROLLBACK`: El registro perdió eventos que tenía antes: un retroceso.
- `MARK_INVALID`: La marca del registro de este equipo (local/cache/heads.json) está dañada o nombra un evento que el registro no tiene en ese lugar.
- `CHECK_UNKNOWN`: Una verificación no pudo correr, así que su resultado es desconocido, y un resultado desconocido nunca es un pase.
- `DOMAIN_NOT_IGNORED`: El repositorio del producto no ignora .brewstillery/, así que un commit ahí lo publicaría.
- `NOT_A_REPOSITORY`: Esta no es la raíz del árbol de trabajo principal del repositorio.
- `HEAD_DETACHED`: El head de Git no nombra ninguna rama, así que no hay rama del registro.
- `PRIVATE_FOLDER_PERMISSIONS`: La carpeta privada da acceso a alguien más que su dueño, así que Fabrewk no la lee.
- `LOCK_HELD`: Otro comando de Fabrewk tiene el candado del registro de este repositorio, u otra tarea está en ejecución: un escritor y una ejecución a la vez.
- `RECORD_HEAD_MOVED`: Algo escribió en el registro sin su candado mientras corría este comando; este comando no escribió nada.
- `RECORD_COMMIT_RACE`: La rama se movió mientras Fabrewk confirmaba el registro; sus eventos están escritos pero no confirmados.
- `TOOL_FAILED`: Una herramienta externa falló; lo que imprimió se guarda como evidencia y nunca se muestra aquí.
- `TOOL_MISSING`: Una herramienta que esto necesita no está en este equipo.

## `fabrewk propose <file> [--json]`

Propone una etapa y sus tareas, o tareas para una etapa del registro, desde un archivo JSON; no autoriza nada.

- `<file>`: El archivo JSON que describe la etapa y sus tareas; su formato va abajo.
- `--json`: Da la salida como un solo documento JSON con un campo schema.
- `--help`: Muestra esta ayuda.

Movimientos que puede causar: `stage-proposed` (etapa propuesta), `task-proposed` (tarea propuesta).

Documentos con --json: `fabrewk.proposed/1`

El formato del archivo

```text
stage: un objeto con estos campos, o el identificador (s-…) de una etapa del registro
  lang: una etiqueta de idioma (BCP 47)
  goal: texto
  budget: un objeto con estos campos
    dispatches: un número entero, 1 o más
    minutes: un número entero, 1 o más
    tokens (opcional): un número entero, 1 o más
  checks: un objeto cuyos campos tienen cada uno un objeto con estos campos
    command: una lista, cada elemento texto
    network: true o false
    minutes: un número entero, 1 o más
    tools: una lista, cada elemento texto
  settings: un objeto cuyos campos tienen cada uno el valor de un ajuste
tasks: una lista de una tarea o más, cada una un objeto con estos campos
  name (opcional): un nombre para la tarea, que una tarea posterior del archivo puede dar entre sus dependencias
  lang: una etiqueta de idioma (BCP 47)
  goal: texto
  requirements: una lista, cada elemento un objeto con estos campos
    text: texto
    criterion: texto
  base: un commit (40 o 64 dígitos hexadecimales), o null
  dependencies: una lista, cada elemento el identificador de una tarea (t-…), o el nombre de una tarea anterior del archivo
  allowed: una lista, cada elemento texto
  profiles: un objeto con estos campos
    build: texto
    review: texto
  checks: una lista, cada elemento texto
  limits: un objeto con estos campos
    rounds: un número entero, 1 o más
    dispatches: un número entero, 1 o más
    minutes: un número entero, 1 o más
    tokens (opcional): un número entero, 1 o más
  delegationLanguage: una etiqueta de idioma (BCP 47)
  settings: un objeto cuyos campos tienen cada uno el valor de un ajuste
```

Rechazos

- `STAGE_CLOSED`: La etapa está cerrada, así que no se le puede agregar ninguna tarea.
- `TASK_INVALID`: Los campos de una etapa o de una tarea no son válidos.
- `PROTECTED_PATH`: Los archivos permitidos, o la copia del agente, alcanzan una ruta protegida (.git o .brewstillery).
- `PROTECTED_PATH_WILDCARD`: Un comodín entre los archivos permitidos coincide con una ruta o un nombre protegidos.
- `CHECK_DUPLICATE`: Una comprobación tiene dos definiciones distintas.
- `RECORD_ABSENT`: Aquí no hay registro.
- `POLICY_ABSENT`: No hay una política adoptada, así que no hay configuración efectiva ni mantenedores.
- `RECORD_BROKEN`: El registro está roto: sus archivos no forman una sola cadena sin cortes.
- `RECORD_DIGEST_MISMATCH`: El contenido de un archivo de evento no es el que su nombre registra.
- `RECORD_NONCANONICAL`: Un archivo de evento no está en forma canónica, así que no se puede confiar en su resumen.
- `RECORD_FORK`: Dos eventos ocupan el mismo lugar de un flujo: dos clones escribieron ahí.
- `RECORD_TRANSITION_INVALID`: Un evento está donde la tabla de transiciones no lo permite.
- `ATTEMPT_ENDED`: El registro termina un intento que no está abierto.
- `RECORD_SIGNATURE_INVALID`: El evento de una persona no está firmado con la llave y el rol que la política da a quien firma.
- `RECORD_VERSION_UNKNOWN`: Un evento no es de un esquema que esta versión de Fabrewk conozca.
- `RECORD_ID_COLLISION`: Dos etapas o dos tareas recibieron el mismo identificador.
- `RECORD_ROLLBACK`: El registro perdió eventos que tenía antes: un retroceso.
- `MARK_INVALID`: La marca del registro de este equipo (local/cache/heads.json) está dañada o nombra un evento que el registro no tiene en ese lugar.
- `CHECK_UNKNOWN`: Una verificación no pudo correr, así que su resultado es desconocido, y un resultado desconocido nunca es un pase.
- `DOMAIN_NOT_IGNORED`: El repositorio del producto no ignora .brewstillery/, así que un commit ahí lo publicaría.
- `NOT_A_REPOSITORY`: Esta no es la raíz del árbol de trabajo principal del repositorio.
- `HEAD_DETACHED`: El head de Git no nombra ninguna rama, así que no hay rama del registro.
- `PRIVATE_FOLDER_PERMISSIONS`: La carpeta privada da acceso a alguien más que su dueño, así que Fabrewk no la lee.
- `SETTINGS_INVALID`: Un archivo de ajustes no se lee o no valida.
- `LOCK_HELD`: Otro comando de Fabrewk tiene el candado del registro de este repositorio, u otra tarea está en ejecución: un escritor y una ejecución a la vez.
- `RECORD_HEAD_MOVED`: Algo escribió en el registro sin su candado mientras corría este comando; este comando no escribió nada.
- `RECORD_COMMIT_RACE`: La rama se movió mientras Fabrewk confirmaba el registro; sus eventos están escritos pero no confirmados.
- `TOOL_FAILED`: Una herramienta externa falló; lo que imprimió se guarda como evidencia y nunca se muestra aquí.
- `TOOL_MISSING`: Una herramienta que esto necesita no está en este equipo.

## `fabrewk authorize <id> --seen <digest> --say <words> [--say-lang <tag>] [--widen <key>]… [--run] [--json]`

Un maestro autoriza una tarea, o una etapa y sus tareas, tal como status las mostró, nombrando cada ampliación que hace.

- `<id>`: La tarea o la etapa, por su identificador (t-… o s-…).
- `--seen <digest>`: El digest que status imprimió de lo que leíste; si el sujeto cambió desde entonces, el movimiento se niega.
- `--say <words>`: Tus propias palabras para esta decisión, guardadas tal cual y firmadas. Se rechazan las copiadas de un ejemplo, como «Estoy de acuerdo.».
- `--say-lang <tag>`: La etiqueta de idioma de tus palabras; por omisión, tu languages.agentInteraction.
- `--widen <key>`: Nombra una ampliación de autoridad que hace este movimiento, por su ruta de clave; una vez por cada una.
- `--run`: Inicia la ejecución en el mismo comando, después de la autorización.
- `--json`: Da la salida como un solo documento JSON con un campo schema.
- `--help`: Muestra esta ayuda.

Movimientos que puede causar: `task-authorized` (tarea autorizada), `stage-authorized` (etapa autorizada), `run-started` (ejecución iniciada), `attempt-started` (intento iniciado), `attempt-ended` (intento terminado), `candidate-recorded` (candidato registrado), `check-recorded` (comprobación registrada), `review-recorded` (revisión registrada), `task-verified` (tarea verificada), `task-halted` (tarea detenida).

Documentos con --json: `fabrewk.authorized/2`

Rechazos

- `AGENT_ENVIRONMENT`: Esto corre dentro de un agente o una extensión, cuyos marcadores están en el entorno, y solo una persona hace este movimiento. El control es cooperativo: un proceso puede limpiar su entorno.
- `MOVE_NOT_ALLOWED`: La tabla de transiciones no permite este movimiento desde el estado del sujeto.
- `ROLE_FORBIDDEN`: La llave que firma este movimiento no tiene en la política ningún rol que pueda hacerlo.
- `WORDS_MISSING`: Un movimiento de persona necesita tus propias palabras, dadas con --say: no vacías y en texto UTF-8 válido.
- `WORDS_COPIED`: Las palabras dadas son el ejemplo que imprime la ayuda, no las tuyas.
- `TASK_CHANGED`: La tarea, o el head de su flujo, cambió después de mostrarse.
- `CHECK_DUPLICATE`: Una comprobación tiene dos definiciones distintas.
- `PROTECTED_PATH`: Los archivos permitidos, o la copia del agente, alcanzan una ruta protegida (.git o .brewstillery).
- `PROTECTED_PATH_WILDCARD`: Un comodín entre los archivos permitidos coincide con una ruta o un nombre protegidos.
- `WIDENING_UNNAMED`: Un cambio amplía la autoridad sin nombrarse.
- `LIMIT_UNOBSERVABLE`: Hay un límite de tokens para una herramienta que no informa los tokens que usa, así que el límite no se podría cumplir.
- `DEPENDENCY_OPEN`: Una tarea de la que depende aún no está integrada.
- `BASE_MOVED`: El producto en la punta de la rama del registro no es el que dejaron la base de la tarea o la última integración de sus dependencias.
- `LOCK_HELD`: Otro comando de Fabrewk tiene el candado del registro de este repositorio, u otra tarea está en ejecución: un escritor y una ejecución a la vez.
- `ALAMBIQUE_UNAPPROVED`: El alambique no es el texto que un maestro aprobó por última vez: cambió a mano, o nunca se aprobó.
- `RECORD_ABSENT`: Aquí no hay registro.
- `POLICY_ABSENT`: No hay una política adoptada, así que no hay configuración efectiva ni mantenedores.
- `RECORD_BROKEN`: El registro está roto: sus archivos no forman una sola cadena sin cortes.
- `RECORD_DIGEST_MISMATCH`: El contenido de un archivo de evento no es el que su nombre registra.
- `RECORD_NONCANONICAL`: Un archivo de evento no está en forma canónica, así que no se puede confiar en su resumen.
- `RECORD_FORK`: Dos eventos ocupan el mismo lugar de un flujo: dos clones escribieron ahí.
- `RECORD_TRANSITION_INVALID`: Un evento está donde la tabla de transiciones no lo permite.
- `ATTEMPT_ENDED`: El registro termina un intento que no está abierto.
- `RECORD_SIGNATURE_INVALID`: El evento de una persona no está firmado con la llave y el rol que la política da a quien firma.
- `RECORD_VERSION_UNKNOWN`: Un evento no es de un esquema que esta versión de Fabrewk conozca.
- `RECORD_ID_COLLISION`: Dos etapas o dos tareas recibieron el mismo identificador.
- `RECORD_ROLLBACK`: El registro perdió eventos que tenía antes: un retroceso.
- `MARK_INVALID`: La marca del registro de este equipo (local/cache/heads.json) está dañada o nombra un evento que el registro no tiene en ese lugar.
- `CHECK_UNKNOWN`: Una verificación no pudo correr, así que su resultado es desconocido, y un resultado desconocido nunca es un pase.
- `DOMAIN_NOT_IGNORED`: El repositorio del producto no ignora .brewstillery/, así que un commit ahí lo publicaría.
- `NOT_A_REPOSITORY`: Esta no es la raíz del árbol de trabajo principal del repositorio.
- `HEAD_DETACHED`: El head de Git no nombra ninguna rama, así que no hay rama del registro.
- `PRIVATE_FOLDER_PERMISSIONS`: La carpeta privada da acceso a alguien más que su dueño, así que Fabrewk no la lee.
- `SETTINGS_INVALID`: Un archivo de ajustes no se lee o no valida.
- `RECORD_HEAD_MOVED`: Algo escribió en el registro sin su candado mientras corría este comando; este comando no escribió nada.
- `RECORD_COMMIT_RACE`: La rama se movió mientras Fabrewk confirmaba el registro; sus eventos están escritos pero no confirmados.
- `TOOL_FAILED`: Una herramienta externa falló; lo que imprimió se guarda como evidencia y nunca se muestra aquí.
- `TOOL_MISSING`: Una herramienta que esto necesita no está en este equipo.

## `fabrewk reject <task> --say <words> [--say-lang <tag>] [--json]`

Un maestro rechaza una tarea y dice por qué.

- `<task>`: La tarea, por su identificador (t-…).
- `--say <words>`: Tus propias palabras para esta decisión, guardadas tal cual y firmadas. Se rechazan las copiadas de un ejemplo, como «Estoy de acuerdo.».
- `--say-lang <tag>`: La etiqueta de idioma de tus palabras; por omisión, tu languages.agentInteraction.
- `--json`: Da la salida como un solo documento JSON con un campo schema.
- `--help`: Muestra esta ayuda.

Movimientos que puede causar: `task-rejected` (tarea rechazada).

Documentos con --json: `fabrewk.rejected/1`

Rechazos

- `AGENT_ENVIRONMENT`: Esto corre dentro de un agente o una extensión, cuyos marcadores están en el entorno, y solo una persona hace este movimiento. El control es cooperativo: un proceso puede limpiar su entorno.
- `MOVE_NOT_ALLOWED`: La tabla de transiciones no permite este movimiento desde el estado del sujeto.
- `ROLE_FORBIDDEN`: La llave que firma este movimiento no tiene en la política ningún rol que pueda hacerlo.
- `WORDS_MISSING`: Un movimiento de persona necesita tus propias palabras, dadas con --say: no vacías y en texto UTF-8 válido.
- `WORDS_COPIED`: Las palabras dadas son el ejemplo que imprime la ayuda, no las tuyas.
- `MERGE_IN_PROGRESS`: El candidato actual de la tarea está aceptado, y su integración puede estar a medias.
- `RECORD_ABSENT`: Aquí no hay registro.
- `POLICY_ABSENT`: No hay una política adoptada, así que no hay configuración efectiva ni mantenedores.
- `RECORD_BROKEN`: El registro está roto: sus archivos no forman una sola cadena sin cortes.
- `RECORD_DIGEST_MISMATCH`: El contenido de un archivo de evento no es el que su nombre registra.
- `RECORD_NONCANONICAL`: Un archivo de evento no está en forma canónica, así que no se puede confiar en su resumen.
- `RECORD_FORK`: Dos eventos ocupan el mismo lugar de un flujo: dos clones escribieron ahí.
- `RECORD_TRANSITION_INVALID`: Un evento está donde la tabla de transiciones no lo permite.
- `ATTEMPT_ENDED`: El registro termina un intento que no está abierto.
- `RECORD_SIGNATURE_INVALID`: El evento de una persona no está firmado con la llave y el rol que la política da a quien firma.
- `RECORD_VERSION_UNKNOWN`: Un evento no es de un esquema que esta versión de Fabrewk conozca.
- `RECORD_ID_COLLISION`: Dos etapas o dos tareas recibieron el mismo identificador.
- `RECORD_ROLLBACK`: El registro perdió eventos que tenía antes: un retroceso.
- `MARK_INVALID`: La marca del registro de este equipo (local/cache/heads.json) está dañada o nombra un evento que el registro no tiene en ese lugar.
- `CHECK_UNKNOWN`: Una verificación no pudo correr, así que su resultado es desconocido, y un resultado desconocido nunca es un pase.
- `DOMAIN_NOT_IGNORED`: El repositorio del producto no ignora .brewstillery/, así que un commit ahí lo publicaría.
- `NOT_A_REPOSITORY`: Esta no es la raíz del árbol de trabajo principal del repositorio.
- `HEAD_DETACHED`: El head de Git no nombra ninguna rama, así que no hay rama del registro.
- `PRIVATE_FOLDER_PERMISSIONS`: La carpeta privada da acceso a alguien más que su dueño, así que Fabrewk no la lee.
- `SETTINGS_INVALID`: Un archivo de ajustes no se lee o no valida.
- `LOCK_HELD`: Otro comando de Fabrewk tiene el candado del registro de este repositorio, u otra tarea está en ejecución: un escritor y una ejecución a la vez.
- `RECORD_HEAD_MOVED`: Algo escribió en el registro sin su candado mientras corría este comando; este comando no escribió nada.
- `RECORD_COMMIT_RACE`: La rama se movió mientras Fabrewk confirmaba el registro; sus eventos están escritos pero no confirmados.
- `TOOL_FAILED`: Una herramienta externa falló; lo que imprimió se guarda como evidencia y nunca se muestra aquí.
- `TOOL_MISSING`: Una herramienta que esto necesita no está en este equipo.

## `fabrewk run <id> --say <words> [--say-lang <tag>] [--json]`

Un maestro inicia la ejecución confinada de una tarea autorizada, o de la siguiente de una etapa; esta versión aún no tiene confinamiento, así que se niega.

- `<id>`: La tarea o la etapa, por su identificador (t-… o s-…).
- `--say <words>`: Tus propias palabras para esta decisión, guardadas tal cual y firmadas. Se rechazan las copiadas de un ejemplo, como «Estoy de acuerdo.».
- `--say-lang <tag>`: La etiqueta de idioma de tus palabras; por omisión, tu languages.agentInteraction.
- `--json`: Da la salida como un solo documento JSON con un campo schema.
- `--help`: Muestra esta ayuda.

Movimientos que puede causar: `run-started` (ejecución iniciada), `attempt-started` (intento iniciado), `attempt-ended` (intento terminado), `candidate-recorded` (candidato registrado), `check-recorded` (comprobación registrada), `review-recorded` (revisión registrada), `task-verified` (tarea verificada), `task-halted` (tarea detenida).

Documentos con --json: `fabrewk.run/1`

Rechazos

- `AGENT_ENVIRONMENT`: Esto corre dentro de un agente o una extensión, cuyos marcadores están en el entorno, y solo una persona hace este movimiento. El control es cooperativo: un proceso puede limpiar su entorno.
- `MOVE_NOT_ALLOWED`: La tabla de transiciones no permite este movimiento desde el estado del sujeto.
- `ROLE_FORBIDDEN`: La llave que firma este movimiento no tiene en la política ningún rol que pueda hacerlo.
- `WORDS_MISSING`: Un movimiento de persona necesita tus propias palabras, dadas con --say: no vacías y en texto UTF-8 válido.
- `WORDS_COPIED`: Las palabras dadas son el ejemplo que imprime la ayuda, no las tuyas.
- `DEPENDENCY_OPEN`: Una tarea de la que depende aún no está integrada.
- `BASE_MOVED`: El producto en la punta de la rama del registro no es el que dejaron la base de la tarea o la última integración de sus dependencias.
- `LOCK_HELD`: Otro comando de Fabrewk tiene el candado del registro de este repositorio, u otra tarea está en ejecución: un escritor y una ejecución a la vez.
- `WIDENING_UNNAMED`: Un cambio amplía la autoridad sin nombrarse.
- `ALAMBIQUE_UNAPPROVED`: El alambique no es el texto que un maestro aprobó por última vez: cambió a mano, o nunca se aprobó.
- `ROUNDS_EXHAUSTED`: Las rondas de revisión de la tarea desde su autorización están gastadas; no se empieza ninguna ronda más.
- `BUDGET_EXHAUSTED`: Los despachos, minutos o tokens de la tarea o de su etapa están gastados, o no se puede saber lo gastado; no se despacha nada más.
- `PROVIDER_MISSING`: review.differentProvider es require, y el proveedor del perfil de revisión es el del constructor, o su herramienta no está.
- `CHECK_DUPLICATE`: Una comprobación tiene dos definiciones distintas.
- `RECORD_ABSENT`: Aquí no hay registro.
- `POLICY_ABSENT`: No hay una política adoptada, así que no hay configuración efectiva ni mantenedores.
- `RECORD_BROKEN`: El registro está roto: sus archivos no forman una sola cadena sin cortes.
- `RECORD_DIGEST_MISMATCH`: El contenido de un archivo de evento no es el que su nombre registra.
- `RECORD_NONCANONICAL`: Un archivo de evento no está en forma canónica, así que no se puede confiar en su resumen.
- `RECORD_FORK`: Dos eventos ocupan el mismo lugar de un flujo: dos clones escribieron ahí.
- `RECORD_TRANSITION_INVALID`: Un evento está donde la tabla de transiciones no lo permite.
- `ATTEMPT_ENDED`: El registro termina un intento que no está abierto.
- `RECORD_SIGNATURE_INVALID`: El evento de una persona no está firmado con la llave y el rol que la política da a quien firma.
- `RECORD_VERSION_UNKNOWN`: Un evento no es de un esquema que esta versión de Fabrewk conozca.
- `RECORD_ID_COLLISION`: Dos etapas o dos tareas recibieron el mismo identificador.
- `RECORD_ROLLBACK`: El registro perdió eventos que tenía antes: un retroceso.
- `MARK_INVALID`: La marca del registro de este equipo (local/cache/heads.json) está dañada o nombra un evento que el registro no tiene en ese lugar.
- `CHECK_UNKNOWN`: Una verificación no pudo correr, así que su resultado es desconocido, y un resultado desconocido nunca es un pase.
- `DOMAIN_NOT_IGNORED`: El repositorio del producto no ignora .brewstillery/, así que un commit ahí lo publicaría.
- `NOT_A_REPOSITORY`: Esta no es la raíz del árbol de trabajo principal del repositorio.
- `HEAD_DETACHED`: El head de Git no nombra ninguna rama, así que no hay rama del registro.
- `PRIVATE_FOLDER_PERMISSIONS`: La carpeta privada da acceso a alguien más que su dueño, así que Fabrewk no la lee.
- `SETTINGS_INVALID`: Un archivo de ajustes no se lee o no valida.
- `RECORD_HEAD_MOVED`: Algo escribió en el registro sin su candado mientras corría este comando; este comando no escribió nada.
- `RECORD_COMMIT_RACE`: La rama se movió mientras Fabrewk confirmaba el registro; sus eventos están escritos pero no confirmados.
- `TOOL_FAILED`: Una herramienta externa falló; lo que imprimió se guarda como evidencia y nunca se muestra aquí.
- `TOOL_MISSING`: Una herramienta que esto necesita no está en este equipo.

## `fabrewk sweep <alias> --answer <file> [--json]`

Registra la respuesta del integrador al fermento de un mantenedor: lo que recomienda y las ediciones que redacta, y escribe la respuesta debajo del separador.

- `<alias>`: El mantenedor, por su alias.
- `--answer <file>`: El archivo JSON con la respuesta del integrador; su formato va abajo.
- `--json`: Da la salida como un solo documento JSON con un campo schema.
- `--help`: Muestra esta ayuda.

Movimientos que puede causar: `sweep-recorded` (barrido registrado), `proposal-recorded` (propuesta registrada), `proposal-withdrawn` (propuesta retirada).

Documentos con --json: `fabrewk.swept/1`

El formato del archivo

```text
El archivo de la respuesta: un objeto con estos campos
  agent: un objeto con estos campos
    tool: texto
    provider: texto
    model: texto
  points: una lista, cada elemento un objeto con estos campos
    number: un número entero, 1 o más
    digest: dígitos hexadecimales
    recommendation: un objeto con estos campos
      point: texto
      clarify: texto
      destination: texto
    edit (opcional): un objeto con estos campos
      target: texto
      base: dígitos hexadecimales
      units: una lista, cada elemento un objeto con estos campos
        kind: texto
        n (opcional): un número entero, 1 o más
        m (opcional): un número entero, 1 o más
        text: texto, o null
      keys: una lista, cada elemento un objeto con estos campos
        point: texto
        key: texto
      result: dígitos hexadecimales

La unidad de una edición es la apertura (kind opening), una sección (kind section, n), un punto (kind point, n y m) o una regla (kind rule, n; un texto null la quita); target es alambique o rules, y una edición de rules lleva solo unidades de regla.```

Rechazos

- `ALIAS_INVALID`: Un nombre que no es un alias válido: minúsculas, dígitos y guiones sueltos, empezando con una letra, de 32 caracteres a lo más, y que no sea un nombre reservado.
- `MAINTAINER_UNLISTED`: Ese alias no es mantenedor de la política vigente.
- `FERMENTO_MISSING`: El mantenedor no tiene fermento.md.
- `SEPARATOR_MISSING`: El fermento no tiene línea de separador fuera de los bloques de código.
- `SEPARATOR_AMBIGUOUS`: La marca del separador está en más de una línea fuera de los bloques de código.
- `SEPARATOR_DAMAGED`: Las líneas alrededor de la marca del separador no son una versión que Fabrewk escribe, o el archivo no es texto válido.
- `ANSWER_INVALID`: El archivo de la respuesta no tiene la forma que lee sweep, o uno de sus textos rompería el separador.
- `POINT_CHANGED`: Un punto de la respuesta no existe en el fermento, cambió desde que se leyó, o no tiene recomendación.
- `INPUT_UNCOMMITTED`: La entrada del fermento difiere de la que está en un commit, y el commit del barrido llevaría tu edición.
- `INPUT_CHANGED`: La entrada del fermento cambió mientras corría el barrido.
- `RECORD_ABSENT`: Aquí no hay registro.
- `POLICY_ABSENT`: No hay una política adoptada, así que no hay configuración efectiva ni mantenedores.
- `RECORD_BROKEN`: El registro está roto: sus archivos no forman una sola cadena sin cortes.
- `RECORD_DIGEST_MISMATCH`: El contenido de un archivo de evento no es el que su nombre registra.
- `RECORD_NONCANONICAL`: Un archivo de evento no está en forma canónica, así que no se puede confiar en su resumen.
- `RECORD_FORK`: Dos eventos ocupan el mismo lugar de un flujo: dos clones escribieron ahí.
- `RECORD_TRANSITION_INVALID`: Un evento está donde la tabla de transiciones no lo permite.
- `ATTEMPT_ENDED`: El registro termina un intento que no está abierto.
- `RECORD_SIGNATURE_INVALID`: El evento de una persona no está firmado con la llave y el rol que la política da a quien firma.
- `RECORD_VERSION_UNKNOWN`: Un evento no es de un esquema que esta versión de Fabrewk conozca.
- `RECORD_ID_COLLISION`: Dos etapas o dos tareas recibieron el mismo identificador.
- `RECORD_ROLLBACK`: El registro perdió eventos que tenía antes: un retroceso.
- `MARK_INVALID`: La marca del registro de este equipo (local/cache/heads.json) está dañada o nombra un evento que el registro no tiene en ese lugar.
- `CHECK_UNKNOWN`: Una verificación no pudo correr, así que su resultado es desconocido, y un resultado desconocido nunca es un pase.
- `DOMAIN_NOT_IGNORED`: El repositorio del producto no ignora .brewstillery/, así que un commit ahí lo publicaría.
- `NOT_A_REPOSITORY`: Esta no es la raíz del árbol de trabajo principal del repositorio.
- `HEAD_DETACHED`: El head de Git no nombra ninguna rama, así que no hay rama del registro.
- `PRIVATE_FOLDER_PERMISSIONS`: La carpeta privada da acceso a alguien más que su dueño, así que Fabrewk no la lee.
- `SETTINGS_INVALID`: Un archivo de ajustes no se lee o no valida.
- `LOCK_HELD`: Otro comando de Fabrewk tiene el candado del registro de este repositorio, u otra tarea está en ejecución: un escritor y una ejecución a la vez.
- `RECORD_HEAD_MOVED`: Algo escribió en el registro sin su candado mientras corría este comando; este comando no escribió nada.
- `RECORD_COMMIT_RACE`: La rama se movió mientras Fabrewk confirmaba el registro; sus eventos están escritos pero no confirmados.
- `TOOL_FAILED`: Una herramienta externa falló; lo que imprimió se guarda como evidencia y nunca se muestra aquí.
- `TOOL_MISSING`: Una herramienta que esto necesita no está en este equipo.

## `fabrewk discard <proposal> --say <words> [--say-lang <tag>] [--json]`

Un maestro descarta una propuesta pendiente, diciendo por qué.

- `<proposal>`: La propuesta, por su identificador (p-…).
- `--say <words>`: Tus propias palabras para esta decisión, guardadas tal cual y firmadas. Se rechazan las copiadas de un ejemplo, como «Estoy de acuerdo.».
- `--say-lang <tag>`: La etiqueta de idioma de tus palabras; por omisión, tu languages.agentInteraction.
- `--json`: Da la salida como un solo documento JSON con un campo schema.
- `--help`: Muestra esta ayuda.

Movimientos que puede causar: `proposal-discarded` (propuesta descartada).

Documentos con --json: `fabrewk.discarded/1`

Rechazos

- `AGENT_ENVIRONMENT`: Esto corre dentro de un agente o una extensión, cuyos marcadores están en el entorno, y solo una persona hace este movimiento. El control es cooperativo: un proceso puede limpiar su entorno.
- `PROPOSAL_UNKNOWN`: Ninguna propuesta del registro tiene ese identificador.
- `MOVE_NOT_ALLOWED`: La tabla de transiciones no permite este movimiento desde el estado del sujeto.
- `ROLE_FORBIDDEN`: La llave que firma este movimiento no tiene en la política ningún rol que pueda hacerlo.
- `WORDS_MISSING`: Un movimiento de persona necesita tus propias palabras, dadas con --say: no vacías y en texto UTF-8 válido.
- `WORDS_COPIED`: Las palabras dadas son el ejemplo que imprime la ayuda, no las tuyas.
- `RECORD_ABSENT`: Aquí no hay registro.
- `POLICY_ABSENT`: No hay una política adoptada, así que no hay configuración efectiva ni mantenedores.
- `RECORD_BROKEN`: El registro está roto: sus archivos no forman una sola cadena sin cortes.
- `RECORD_DIGEST_MISMATCH`: El contenido de un archivo de evento no es el que su nombre registra.
- `RECORD_NONCANONICAL`: Un archivo de evento no está en forma canónica, así que no se puede confiar en su resumen.
- `RECORD_FORK`: Dos eventos ocupan el mismo lugar de un flujo: dos clones escribieron ahí.
- `RECORD_TRANSITION_INVALID`: Un evento está donde la tabla de transiciones no lo permite.
- `ATTEMPT_ENDED`: El registro termina un intento que no está abierto.
- `RECORD_SIGNATURE_INVALID`: El evento de una persona no está firmado con la llave y el rol que la política da a quien firma.
- `RECORD_VERSION_UNKNOWN`: Un evento no es de un esquema que esta versión de Fabrewk conozca.
- `RECORD_ID_COLLISION`: Dos etapas o dos tareas recibieron el mismo identificador.
- `RECORD_ROLLBACK`: El registro perdió eventos que tenía antes: un retroceso.
- `MARK_INVALID`: La marca del registro de este equipo (local/cache/heads.json) está dañada o nombra un evento que el registro no tiene en ese lugar.
- `CHECK_UNKNOWN`: Una verificación no pudo correr, así que su resultado es desconocido, y un resultado desconocido nunca es un pase.
- `DOMAIN_NOT_IGNORED`: El repositorio del producto no ignora .brewstillery/, así que un commit ahí lo publicaría.
- `NOT_A_REPOSITORY`: Esta no es la raíz del árbol de trabajo principal del repositorio.
- `HEAD_DETACHED`: El head de Git no nombra ninguna rama, así que no hay rama del registro.
- `PRIVATE_FOLDER_PERMISSIONS`: La carpeta privada da acceso a alguien más que su dueño, así que Fabrewk no la lee.
- `SETTINGS_INVALID`: Un archivo de ajustes no se lee o no valida.
- `LOCK_HELD`: Otro comando de Fabrewk tiene el candado del registro de este repositorio, u otra tarea está en ejecución: un escritor y una ejecución a la vez.
- `RECORD_HEAD_MOVED`: Algo escribió en el registro sin su candado mientras corría este comando; este comando no escribió nada.
- `RECORD_COMMIT_RACE`: La rama se movió mientras Fabrewk confirmaba el registro; sus eventos están escritos pero no confirmados.
- `TOOL_FAILED`: Una herramienta externa falló; lo que imprimió se guarda como evidencia y nunca se muestra aquí.
- `TOOL_MISSING`: Una herramienta que esto necesita no está en este equipo.

## `fabrewk approve <proposal> --say <words> [--say-lang <tag>] [--json]`

Un maestro aprueba una propuesta, que aplica su edición y hace commit del alambique solo, o aprueba el alambique como está, diciendo por qué.

- `<proposal>`: Una propuesta, por su identificador (p-…), o la palabra alambique.
- `--say <words>`: Tus propias palabras para esta decisión, guardadas tal cual y firmadas. Se rechazan las copiadas de un ejemplo, como «Estoy de acuerdo.».
- `--say-lang <tag>`: La etiqueta de idioma de tus palabras; por omisión, tu languages.agentInteraction.
- `--json`: Da la salida como un solo documento JSON con un campo schema.
- `--help`: Muestra esta ayuda.

Movimientos que puede causar: `proposal-incorporated` (propuesta incorporada), `direct-edit-approved` (edición directa aprobada), `alambique-baseline` (base del alambique).

Documentos con --json: `fabrewk.approved/1`

Rechazos

- `AGENT_ENVIRONMENT`: Esto corre dentro de un agente o una extensión, cuyos marcadores están en el entorno, y solo una persona hace este movimiento. El control es cooperativo: un proceso puede limpiar su entorno.
- `PROPOSAL_UNKNOWN`: Ninguna propuesta del registro tiene ese identificador.
- `MOVE_NOT_ALLOWED`: La tabla de transiciones no permite este movimiento desde el estado del sujeto.
- `ROLE_FORBIDDEN`: La llave que firma este movimiento no tiene en la política ningún rol que pueda hacerlo.
- `WORDS_MISSING`: Un movimiento de persona necesita tus propias palabras, dadas con --say: no vacías y en texto UTF-8 válido.
- `WORDS_COPIED`: Las palabras dadas son el ejemplo que imprime la ayuda, no las tuyas.
- `PROPOSAL_STALE`: El punto del fermento ya no está o cambió, o otra propuesta cambió las reglas del proyecto desde entonces, o el texto redactado no da el resultado que predice.
- `ALAMBIQUE_CHANGED`: El alambique, en el árbol de trabajo, en el índice o en el último commit, no es el texto del que se redactó la propuesta.
- `ALAMBIQUE_UNAPPROVED`: El alambique no es el texto que un maestro aprobó por última vez: cambió a mano, o nunca se aprobó.
- `ALAMBIQUE_MISSING`: No hay ALAMBIQUE.md, o no es un archivo regular al que se llegue sin enlaces, o no es texto válido.
- `UNIT_AMBIGUOUS`: Una unidad de la edición está dos veces o ninguna en el alambique, o dos unidades se traslapan, o una regla que quita no existe.
- `RECORD_ABSENT`: Aquí no hay registro.
- `POLICY_ABSENT`: No hay una política adoptada, así que no hay configuración efectiva ni mantenedores.
- `RECORD_BROKEN`: El registro está roto: sus archivos no forman una sola cadena sin cortes.
- `RECORD_DIGEST_MISMATCH`: El contenido de un archivo de evento no es el que su nombre registra.
- `RECORD_NONCANONICAL`: Un archivo de evento no está en forma canónica, así que no se puede confiar en su resumen.
- `RECORD_FORK`: Dos eventos ocupan el mismo lugar de un flujo: dos clones escribieron ahí.
- `RECORD_TRANSITION_INVALID`: Un evento está donde la tabla de transiciones no lo permite.
- `ATTEMPT_ENDED`: El registro termina un intento que no está abierto.
- `RECORD_SIGNATURE_INVALID`: El evento de una persona no está firmado con la llave y el rol que la política da a quien firma.
- `RECORD_VERSION_UNKNOWN`: Un evento no es de un esquema que esta versión de Fabrewk conozca.
- `RECORD_ID_COLLISION`: Dos etapas o dos tareas recibieron el mismo identificador.
- `RECORD_ROLLBACK`: El registro perdió eventos que tenía antes: un retroceso.
- `MARK_INVALID`: La marca del registro de este equipo (local/cache/heads.json) está dañada o nombra un evento que el registro no tiene en ese lugar.
- `CHECK_UNKNOWN`: Una verificación no pudo correr, así que su resultado es desconocido, y un resultado desconocido nunca es un pase.
- `DOMAIN_NOT_IGNORED`: El repositorio del producto no ignora .brewstillery/, así que un commit ahí lo publicaría.
- `NOT_A_REPOSITORY`: Esta no es la raíz del árbol de trabajo principal del repositorio.
- `HEAD_DETACHED`: El head de Git no nombra ninguna rama, así que no hay rama del registro.
- `PRIVATE_FOLDER_PERMISSIONS`: La carpeta privada da acceso a alguien más que su dueño, así que Fabrewk no la lee.
- `SETTINGS_INVALID`: Un archivo de ajustes no se lee o no valida.
- `LOCK_HELD`: Otro comando de Fabrewk tiene el candado del registro de este repositorio, u otra tarea está en ejecución: un escritor y una ejecución a la vez.
- `RECORD_HEAD_MOVED`: Algo escribió en el registro sin su candado mientras corría este comando; este comando no escribió nada.
- `RECORD_COMMIT_RACE`: La rama se movió mientras Fabrewk confirmaba el registro; sus eventos están escritos pero no confirmados.
- `TOOL_FAILED`: Una herramienta externa falló; lo que imprimió se guarda como evidencia y nunca se muestra aquí.
- `TOOL_MISSING`: Una herramienta que esto necesita no está en este equipo.

## `fabrewk close <stage> --say <words> [--say-lang <tag>] [--json]`

Un maestro cierra una etapa autorizada cuando todas sus tareas terminaron, cada fermento se barrió después de la última, el alambique es el aprobado por última vez y el registro está confirmado.

- `<stage>`: La etapa, por su identificador (s-…).
- `--say <words>`: Tus propias palabras para esta decisión, guardadas tal cual y firmadas. Se rechazan las copiadas de un ejemplo, como «Estoy de acuerdo.».
- `--say-lang <tag>`: La etiqueta de idioma de tus palabras; por omisión, tu languages.agentInteraction.
- `--json`: Da la salida como un solo documento JSON con un campo schema.
- `--help`: Muestra esta ayuda.

Movimientos que puede causar: `stage-closed` (etapa cerrada).

Documentos con --json: `fabrewk.closed/1`

Rechazos

- `AGENT_ENVIRONMENT`: Esto corre dentro de un agente o una extensión, cuyos marcadores están en el entorno, y solo una persona hace este movimiento. El control es cooperativo: un proceso puede limpiar su entorno.
- `MOVE_NOT_ALLOWED`: La tabla de transiciones no permite este movimiento desde el estado del sujeto.
- `ROLE_FORBIDDEN`: La llave que firma este movimiento no tiene en la política ningún rol que pueda hacerlo.
- `WORDS_MISSING`: Un movimiento de persona necesita tus propias palabras, dadas con --say: no vacías y en texto UTF-8 válido.
- `WORDS_COPIED`: Las palabras dadas son el ejemplo que imprime la ayuda, no las tuyas.
- `STAGE_OPEN_TASKS`: Una tarea de la etapa no ha terminado: no está ni integrada ni rechazada.
- `SWEEP_MISSING`: El fermento de un mantenedor no se barrió después de que terminó la última tarea de la etapa.
- `ALAMBIQUE_UNAPPROVED`: El alambique no es el texto que un maestro aprobó por última vez: cambió a mano, o nunca se aprobó.
- `RECORD_UNCOMMITTED`: Algunos archivos del registro no están confirmados en la rama del registro.
- `RECORD_ABSENT`: Aquí no hay registro.
- `POLICY_ABSENT`: No hay una política adoptada, así que no hay configuración efectiva ni mantenedores.
- `RECORD_BROKEN`: El registro está roto: sus archivos no forman una sola cadena sin cortes.
- `RECORD_DIGEST_MISMATCH`: El contenido de un archivo de evento no es el que su nombre registra.
- `RECORD_NONCANONICAL`: Un archivo de evento no está en forma canónica, así que no se puede confiar en su resumen.
- `RECORD_FORK`: Dos eventos ocupan el mismo lugar de un flujo: dos clones escribieron ahí.
- `RECORD_TRANSITION_INVALID`: Un evento está donde la tabla de transiciones no lo permite.
- `ATTEMPT_ENDED`: El registro termina un intento que no está abierto.
- `RECORD_SIGNATURE_INVALID`: El evento de una persona no está firmado con la llave y el rol que la política da a quien firma.
- `RECORD_VERSION_UNKNOWN`: Un evento no es de un esquema que esta versión de Fabrewk conozca.
- `RECORD_ID_COLLISION`: Dos etapas o dos tareas recibieron el mismo identificador.
- `RECORD_ROLLBACK`: El registro perdió eventos que tenía antes: un retroceso.
- `MARK_INVALID`: La marca del registro de este equipo (local/cache/heads.json) está dañada o nombra un evento que el registro no tiene en ese lugar.
- `CHECK_UNKNOWN`: Una verificación no pudo correr, así que su resultado es desconocido, y un resultado desconocido nunca es un pase.
- `DOMAIN_NOT_IGNORED`: El repositorio del producto no ignora .brewstillery/, así que un commit ahí lo publicaría.
- `NOT_A_REPOSITORY`: Esta no es la raíz del árbol de trabajo principal del repositorio.
- `HEAD_DETACHED`: El head de Git no nombra ninguna rama, así que no hay rama del registro.
- `PRIVATE_FOLDER_PERMISSIONS`: La carpeta privada da acceso a alguien más que su dueño, así que Fabrewk no la lee.
- `SETTINGS_INVALID`: Un archivo de ajustes no se lee o no valida.
- `LOCK_HELD`: Otro comando de Fabrewk tiene el candado del registro de este repositorio, u otra tarea está en ejecución: un escritor y una ejecución a la vez.
- `RECORD_HEAD_MOVED`: Algo escribió en el registro sin su candado mientras corría este comando; este comando no escribió nada.
- `RECORD_COMMIT_RACE`: La rama se movió mientras Fabrewk confirmaba el registro; sus eventos están escritos pero no confirmados.
- `TOOL_FAILED`: Una herramienta externa falló; lo que imprimió se guarda como evidencia y nunca se muestra aquí.
- `TOOL_MISSING`: Una herramienta que esto necesita no está en este equipo.

## El programa solo

- `fabrewk --help`: Muestra esta ayuda.
- `fabrewk --version`: Muestra la versión de Fabrewk.

Documentos con --json: `fabrewk.refused/1`, `fabrewk.usage/1`, `fabrewk.internal/1`, `fabrewk.version/1`

## Estados de salida

- `0`: hecho.
- `1`: rechazado: se imprimió un código y no se escribió nada.
- `2`: error de uso: no se aceptó la línea de comandos.
- `3`: la verificación encontró un problema.
- `4`: la corrida se detuvo y espera a una persona: la tarea está detenida.
- `70`: error interno.

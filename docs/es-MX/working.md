# Trabajar con Fabrewk

Fabrewk guarda la descripción de un proyecto, sus decisiones y el trabajo de sus agentes en una carpeta del
repositorio, `.brewstillery/`, y deja que los agentes trabajen solo en lo que una persona autorizó, aislados, con cada
decisión firmada. Este manual explica las partes y el ciclo de trabajo. Cada comando se describe completo, con sus
opciones y sus rechazos, en la [referencia de comandos](../../crates/fabrewk/doc/reference.es.md).

## Las partes, con nombres de destilería

- **Brewstillery**, la destilería, es el dominio de Fabrewk en el repositorio: la carpeta `.brewstillery/`. Todo lo
  que Fabrewk escribe vive ahí, salvo un bloque delimitado en `AGENTS.md` y `CLAUDE.md` y, cuando la carpeta no se
  publica, una línea en `.git/info/exclude`.
- **El alambique**, `.brewstillery/ALAMBIQUE.md`, describe lo que el proyecto debe ser. Nadie lo edita directamente:
  sus cambios vienen de los fermentos, y solo un maestro aprueba lo que entra.
- **La fórmula**, `.brewstillery/FORMULA.md`, es la hoja de ruta del proyecto: las fases, subfases y cortes que el
  integrador propone a partir del alambique, y que un maestro aprueba. Un proyecto no tiene ninguna hasta que su
  integrador propone una.
- **Un fermento**, `.brewstillery/<alias>/fermento.md`, es el archivo propio de cada mantenedor, con lo que quiere
  agregar, quitar o discutir. El mantenedor escribe arriba de su separador; los agentes responden debajo.
- **El registro**, en `.brewstillery/data/`, guarda cada evento como un archivo inmutable: propuestas,
  autorizaciones, intentos, verificaciones, revisiones, aceptaciones. El evento de una persona se firma con su llave
  SSH. `STATUS.md` y `RULES.md` son vistas generadas a partir de él; nunca las edites.
- **La configuración**: `.brewstillery/settings.toml` es la del proyecto, vigente solo cuando un maestro la adopta;
  `.brewstillery/<alias>/settings.toml` guarda las preferencias de un mantenedor, que nunca amplían ningún permiso.
- **El área local**, `.brewstillery/local/`, nunca se versiona: evidencia, cachés, copias temporales y el candado.
- **La carpeta privada**, `~/.config/fabrewk/`, está fuera de todo repositorio y oculta a todo agente.

## Quién hace qué

- Un **maestro** toma las decisiones: adopta la configuración, autoriza y acepta el trabajo, aprueba el alambique.
  Cada uno de estos movimientos lleva las propias palabras del maestro (`--say`), registradas tal como las escribe y
  firmadas con su llave.
- Un **contribuidor** propone, y no concede nada.
- Un **agente** trabaja solo dentro de una copia aislada, en una tarea que un maestro autorizó. El movimiento de una
  persona se niega a correr dentro de un agente (`AGENT_ENVIRONMENT`): una verificación cooperativa, porque un proceso
  puede limpiar su entorno; la firma es lo que vincula el movimiento con el maestro.

Se rechazan las palabras copiadas de un ejemplo que imprime la ayuda (`WORDS_COPIED`). Un movimiento que cambia lo que
los agentes pueden hacer nombra cada ampliación que hace con `--widen <clave>`.

## La configuración

La instalación escribe el primer `settings.toml` con el maestro, los idiomas y las herramientas elegidas. No declara
ningún modelo ni perfil: antes de cualquier ejecución, un maestro los agrega (los modelos que una herramienta puede
usar y los perfiles que juntan una herramienta, un modelo y un esfuerzo para construir y para revisar) y luego adopta
el archivo:

    fabrewk config          # la configuración efectiva, con su origen, y lo que aún no se adopta
    fabrewk adopt --say '<tus palabras>'

`fabrewk config` también muestra lo que tiene este equipo: las herramientas encontradas, el aislamiento y cada
control, marcado como efectivo o cooperativo.

## El ciclo del alambique

1. Un mantenedor escribe puntos en su fermento, arriba del separador.
2. El integrador, un agente, los lee y registra su respuesta con `fabrewk sweep <alias> --answer <archivo>`: lo que
   recomienda para cada punto y, donde hay consenso, el texto del alambique que redacta. La respuesta se escribe debajo
   del separador, y cada edición redactada es una propuesta pendiente.
3. Un maestro lee la propuesta en `fabrewk status` y la aprueba, `fabrewk approve <propuesta> --say '<palabras>'`, lo
   que aplica la edición y confirma solo el alambique, o la descarta, `fabrewk discard <propuesta> --say
   '<palabras>'`. Una propuesta caduca cuando cambian su punto, el alambique o su texto.
4. Un alambique editado a mano no está aprobado hasta que un maestro lo aprueba tal como está: `fabrewk approve
   alambique --say '<palabras>'`. Mientras no está aprobado, no empieza ninguna ejecución ni se acepta ninguna tarea
   (`ALAMBIQUE_UNAPPROVED`).

## La fórmula

1. El integrador escribe la fórmula en `FORMULA.md`, o redacta una edición suya en la respuesta de un fermento, como
   para el alambique (target `formula`, cuya única unidad es su texto completo).
2. Un maestro la aprueba con un solo movimiento firmado: `fabrewk approve formula --say '<palabras>'` para el archivo
   tal como está, o `fabrewk approve <propuesta> --say '<palabras>'` para una edición redactada; cualquiera de los dos
   confirma solo la fórmula. El registro guarda cada versión aprobada por su digest.
3. Una fórmula aprobada antes de que Fabrewk pudiera registrarlo, con el commit firmado del maestro que cambia solo
   `FORMULA.md`, se importa como su primera aprobación con `fabrewk import <commit>`, que no pide palabras ni hace una
   firma nueva: Git verifica la firma del commit contra la llave de un maestro de la política vigente.
4. Mientras `FORMULA.md` difiere de su última aprobación, no empieza ninguna ejecución, no se acepta ninguna tarea y
   no se cierra ninguna etapa (`FORMULA_UNAPPROVED`); `fabrewk check` lo informa.
5. El integrador registra el corte en el que trabaja, `fabrewk cut <corte>` (el identificador de su título, como
   `1a`), y lo reporta terminado para la auditoría del maestro con `fabrewk cut <corte> --report`; el maestro lo
   audita en el chat o en su fermento. `fabrewk status` y `STATUS.md` muestran la aprobación de la fórmula, el corte
   en curso o la auditoría que espera, y nombran `FORMULA.md` como lo siguiente que lee un agente.

## El ciclo de una tarea

1. **Proponer.** Cualquiera escribe una etapa y sus tareas en un archivo JSON y la registra con `fabrewk propose
   <archivo>`. Una tarea nombra su objetivo, sus requisitos con sus criterios, los archivos que puede cambiar, sus
   verificaciones, sus perfiles de construcción y de revisión y sus límites; ninguna tarea puede alcanzar `.git` ni
   `.brewstillery/` (`PROTECTED_PATH`). Proponer no concede nada. `fabrewk propose --help` imprime el formato del
   archivo.
2. **Autorizar.** Un maestro lee la tarea o la etapa en `fabrewk status <id>`, que imprime un resumen (digest) de lo
   mostrado, y la autoriza: `fabrewk authorize <id> --seen <digest> --say '<palabras>'`. Si el sujeto cambió desde que
   se leyó, el movimiento se rechaza (`TASK_CHANGED`). Con `--run`, la ejecución empieza en el mismo comando.
3. **Ejecutar.** `fabrewk run <id> --say '<palabras>'` corre rondas: el agente constructor trabaja en una copia
   aislada del árbol base, sin el `.git` del repositorio, sin acceso a la carpeta privada y con solo la red que necesita
   su proveedor; Fabrewk registra su candidato, corre las verificaciones y despacha al revisor sobre el mismo
   candidato. La ejecución termina con una tarea verificada, o se detiene y espera a una persona (estado de salida 4),
   diciendo por qué. `run <etapa>` corre la primera tarea de la etapa que puede empezar una
   ejecución.
4. **Aceptar o rechazar.** Un maestro lee el candidato verificado en `fabrewk status <tarea>` y lo acepta, `fabrewk
   accept <tarea> --seen <digest> --say '<palabras>'`, lo que lo integra en la rama en el mismo comando, o lo rechaza,
   `fabrewk reject <tarea> --say '<palabras>'`.
5. **Cerrar.** Cuando todas las tareas de una etapa terminaron, cada fermento se barrió después de la última, el
   alambique es el aprobado por última vez y el registro está confirmado, un maestro cierra la etapa: `fabrewk close
   <etapa> --say '<palabras>'`.

Una tarea que cambia una ruta que no puede cambiar, o que deja una posible credencial en su candidato, se detiene
(`LANE_VIOLATION`, `CREDENTIAL_SUSPECTED`). Una ejecución interrumpida a medias la detiene el siguiente comando que
escribe.

## Revisar el repositorio

- `fabrewk status` muestra la salud del registro, la política vigente y el estado de cada etapa y tarea.
- `fabrewk check` verifica la configuración, la política, el registro, el dominio, las credenciales y la carpeta
  privada; sale con 3 cuando encuentra un error; solo advertencias, como una configuración aún sin adoptar, salen con
  0.

`.brewstillery/bootstrap/` registra la construcción del propio Fabrewk a mano. Existe solo en el repositorio del
propio Fabrewk, nunca en un proyecto al que Fabrewk sirve. C13 verifica: "Después de la importación, bootstrap/
contiene exactamente los archivos importados, byte por byte, sin enlaces ni directorios vacíos." Las diferencias se
reportan como `BOOTSTRAP_CHANGED`.

Todo comando integrado salvo `onboard` acepta `--json`, que imprime un documento con un campo `schema`, para agentes y
scripts. Todo comando integrado acepta `--help`, que imprime texto. [Los comandos](commands.md) los enumera todos.

## Extensiones

Un programa llamado `fabrewk-<nombre>` corre como `fabrewk <nombre>` cuando `[extensions.<nombre>]` en la
configuración lo permite. Corre aislado, no escribe nada en el repositorio por sí mismo e imprime solo solicitudes
(una nota, o una propuesta de tarea, que no concede nada), que el núcleo aplica después de que termina con éxito.
`fabrewk config` enumera cada extensión con su ruta y su digest.

# Instalar Fabrewk e incorporarlo a un repositorio

Fabrewk es un solo programa, `fabrewk`, instalado en tu carpeta personal. Entra a un repositorio solo en dos pasos
que puedes leer antes de que se escriba nada: `fabrewk assess`, que no escribe nada y muestra un plan, y `fabrewk
apply`, que un maestro ejecuta en su propia terminal, con sus propias palabras, para aplicar exactamente ese plan.
`fabrewk onboard` hace las preguntas en la terminal y ejecuta ambos por ti.

## Qué necesitas

- Linux en ARM64 (`aarch64`). Es la única plataforma en la que Fabrewk se prueba hoy; el script rechaza cualquier
  otra con `PLATFORM_UNSUPPORTED` y la lista de plataformas soportadas.
- Para el script: `curl`, `ssh-keygen` (OpenSSH), `sha256sum`, `tar` y `gzip`. Si falta alguno, el script dice
  `TOOL_MISSING`, lo nombra y se detiene; nunca instala una herramienta por su cuenta.
- En el repositorio: Git y una llave SSH con la que Git firme (`gpg.format = ssh` y `user.signingkey`). Cada
  decisión que una persona toma en Fabrewk se firma con esa llave.
- Para que los agentes trabajen en tareas, más adelante: bubblewrap (`bwrap`) y passt (`pasta`), que aíslan a cada
  agente, y la herramienta del agente (Claude Code o Codex). `fabrewk config` muestra lo que tiene este equipo.

## Instalar el programa

El script instala un archivo, `~/.local/bin/fabrewk`, y nada más. Descarga la versión escrita en él, el archivo de
sumas de verificación del paquete, `SHA256SUMS`, y la firma de ese archivo, `SHA256SUMS.sig`; verifica la firma con
`ssh-keygen -Y verify` contra los firmantes escritos en él, luego la suma del paquete, y no instala nada si alguna
falla. No acepta opciones, así que ninguna se salta la verificación, y nunca usa `sudo`.

Confiar en el origen de la versión y en sus firmantes les da autoridad total sobre lo que el script instala.

Léelo primero y luego ejecútalo (funciona en fish y en cualquier shell POSIX):

    curl --proto '=https' --tlsv1.2 -fsSLO https://raw.githubusercontent.com/fabrewk/fabrewk/v0.1.0/install.sh
    less install.sh
    sh install.sh

O pásalo directo a `sh`:

    curl --proto '=https' --tlsv1.2 -fsSL https://raw.githubusercontent.com/fabrewk/fabrewk/v0.1.0/install.sh | sh

Los firmantes de la versión, como el script los escribe, en el formato de firmantes permitidos de OpenSSH:

    xel namespaces="fabrewk-release" ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMpNfw3yY4sb7CO5xUCEfRQuPtw6UlYial7Y/MWCHFVf xel@fabrewk

Para verificar una versión a mano, guarda esa línea como `allowed_signers` junto a los tres archivos descargados y
ejecuta:

    ssh-keygen -Y verify -f allowed_signers -I xel -n fabrewk-release -s SHA256SUMS.sig < SHA256SUMS
    sha256sum -c SHA256SUMS

Si `~/.local/bin` no está en tu `PATH`, el script lo dice y cómo agregarlo: en fish, `fish_add_path ~/.local/bin`;
en una shell POSIX, `export PATH="$HOME/.local/bin:$PATH"` en `~/.profile`.

El script instala la versión que nombra; una más nueva solo llega ejecutando un script más nuevo. Fabrewk nunca busca
actualizaciones por su cuenta y no envía telemetría.

### Autocompletado

El paquete trae los archivos de autocompletado en el `completions/` de su carpeta; el script no los instala. Para
usarlos, extrae el paquete (`tar -xzf fabrewk-0.1.0-linux-aarch64.tar.gz`) y copia, desde
`fabrewk-0.1.0-linux-aarch64/completions/`:

- fish: `fabrewk.fish` a `~/.config/fish/completions/fabrewk.fish`;
- bash: `fabrewk.sh` a `~/.local/share/bash-completion/completions/fabrewk`;
- zsh: el mismo `fabrewk.sh`, cargado desde `~/.zshrc` después de activar el autocompletado de bash: `autoload -U
  bashcompinit; bashcompinit; source <la ruta donde pusiste fabrewk.sh>`.

### Quitarlo

Borra `~/.local/bin/fabrewk`. No se instaló nada más.

## Incorporar Fabrewk a un repositorio

### En la terminal: `fabrewk onboard`

Cuando el script corre dentro de un repositorio de Git y hay una terminal, ofrece ejecutar ahí `fabrewk onboard`.
También puedes ejecutarlo tú en la raíz del repositorio. Se niega dentro de un agente (`AGENT_ENVIRONMENT`) y sin
terminal (`TERMINAL_REQUIRED`). Pregunta, en este orden:

1. el idioma de la entrevista y de la documentación canónica del proyecto (por ejemplo, `en` o `es-MX`);
2. el modo: `adopt`, para que Fabrewk administre el repositorio, o `review`, para solo revisarlo (el modo de revisión
   aún no está construido: su plan se muestra, no se aplica);
3. si los mantenedores publican `.brewstillery/`, la carpeta de Fabrewk en el repositorio: `published`, con el
   producto; `own-repository`, en un repositorio de Git propio dentro de esa carpeta; o `unpublished`. En los dos
   últimos, el producto ignora la carpeta con la línea `/.brewstillery/` en su `.git/info/exclude`. Elegir
   `unpublished` muestra un aviso con sus consecuencias y sus salidas, y vuelve a preguntar;
4. el alias del primer maestro (minúsculas, dígitos y guiones sencillos);
5. la ruta de la llave pública OpenSSH del maestro, la que Git usa para firmar; si Git todavía no firma con ella, el
   plan muestra los comandos de Git para configurarlo, y no ejecuta ninguno;
6. las herramientas que se declaran, entre las integradas (`claude`, `codex`), o ninguna;
7. cuando el repositorio no tiene `.brewstillery/ALAMBIQUE.md`: qué debe lograr el proyecto, para quién y qué deja
   fuera. Cada respuesta se escribe tal como la das; una vacía deja una línea que dice que aún no tiene respuesta.

Luego muestra el veredicto y el plan completo: cada archivo que crearía, con su contenido exacto; el bloque que
agregaría a un archivo de entrada existente; la línea que agregaría a `.git/info/exclude`; y las vistas que generaría
a partir del registro (`STATUS.md`, `RULES.md`), por su nombre. Solo el
sí del catálogo (`sí` en español, `yes` en inglés) seguido de tus propias palabras lo aplica; cualquier otra
respuesta, una vacía o el fin de la entrada no aplican nada, y lo dice. Tus palabras se registran tal como las
escribes, y se firman.

### Con un agente: `fabrewk assess` y después `fabrewk apply`

Un agente puede preparar el mismo plan, pero solo una persona lo aplica. El agente te hace las mismas preguntas en el
chat y ejecuta `fabrewk assess` con tus respuestas como opciones, por ejemplo:

    fabrewk assess --mode adopt --publication published --alias ana --key ~/.ssh/id_ed25519.pub --tool claude --lang es-MX --purpose 'Qué debe lograr el proyecto' --audience 'Para quién' --excluded 'Qué deja fuera'

`assess` no escribe nada, ni siquiera una caché, y no usa la red. Imprime el veredicto, el plan, el resumen
criptográfico (digest) del plan y el comando `apply` exacto, entrecomillado para fish y para una shell POSIX. Lees el
plan y ejecutas ese comando en tu propia terminal, cambiando `<tus palabras>` por las tuyas (en inglés, `assess`
imprime `<your words>`):

    fabrewk apply <digest> --mode='adopt' --publication='published' … --say='<tus palabras>'

`apply` se niega dentro de un agente (`AGENT_ENVIRONMENT`). Esa verificación es cooperativa: un proceso puede
limpiar su entorno. Lo que te vincula con el acto es la firma con tu llave; una llave que pide un toque cierra el
hueco que un agente de tu mismo usuario podría aprovechar.

### El veredicto

- `adopt`: Fabrewk puede administrar el repositorio; el plan se puede aplicar.
- `review`: el plan solo revisa el repositorio. El modo de revisión aún no está construido, así que `apply` lo
  rechaza (`MODE_NOT_AVAILABLE`).
- `none`: algo lo impide, por ejemplo un `AGENTS.md` o un `CLAUDE.md` con cambios sin confirmar; cada razón se
  enumera. `apply` lo rechaza (`VERDICT_NONE`).

Las razones también informan lo que otras herramientas dejaron que cambiaría lo que los agentes pueden hacer: hooks
en la configuración de `.claude/` o `.codex/`, actualizaciones automáticas, telemetría, sincronizaciones que hacen
commit o push, archivos generados que preaprueban herramientas. Fabrewk no cambia ninguno.

### Qué hace `apply`

Vuelve a evaluar y se niega, sin escribir nada (`PLAN_CHANGED`), si el plan que da ahora no es el nombrado, o si un
archivo que el plan escribe cambió desde entonces. Luego escribe los archivos del plan, sin sobrescribir nunca un
archivo que no escribió: en un `AGENTS.md` o un `CLAUDE.md` existentes agrega un bloque delimitado que dirige a los
agentes a `.brewstillery/STATUS.md`; un `CLAUDE.md` que crea tiene la sola línea `@AGENTS.md`. Registra, firmados con
tu llave, la evaluación, la decisión de publicación con tus palabras, la configuración adoptada y la línea base del
alambique; luego, como evento propio de Fabrewk, sin firma, el recibo de la instalación, con el resumen de cada
archivo que escribió. En `published` confirma
todo, los archivos de entrada incluidos, conservando cualquier otro cambio que tuvieras; en `own-repository` el
repositorio propio de la carpeta confirma sus archivos, y los archivos de entrada quedan sin confirmar en el
producto; en `unpublished` no se confirma nada. Nombra lo que dejó sin confirmar y el comando de Git que lo confirma,
y no ejecuta ninguno.

Un `apply` que se detiene a medias (una caída, un disco lleno) deja el repositorio marcado `INSTALLATION_INCOMPLETE`:
ningún otro comando escribe hasta que el mismo `apply`, ejecutado de nuevo con el mismo digest y las mismas opciones,
lo termina, escribiendo solo lo que falta.

### Publicación de `.brewstillery/`

Fabrewk recomienda mantener `.brewstillery/` en Git, publicado con el producto, para que cada clon lleve la
descripción del proyecto, sus decisiones y su registro. Elegir no publicarlo significa que la historia de decisiones
del proyecto no viaja con el código; el aviso que muestra `onboard` dice las consecuencias y las salidas. Tu respuesta
se registra con tus propias palabras.

### Después de la instalación

- `fabrewk check` verifica la configuración, el registro y el dominio.
- `fabrewk status` muestra el estado del trabajo.
- El archivo del proyecto aún no declara ningún modelo ni perfil: un maestro los agrega a
  `.brewstillery/settings.toml` y lo adopta de nuevo con `fabrewk adopt --say '<palabras>'`. Consulta
  [Trabajar con Fabrewk](working.md).

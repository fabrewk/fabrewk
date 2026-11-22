[English](README.md)

# Fabrewk

Fabrewk es un meta-harness de código abierto para construir software con agentes de IA. Guarda la descripción de
un proyecto (el alambique), la entrada de cada mantenedor (los fermentos), sus decisiones y el trabajo de sus
agentes en `.brewstillery/`. Los agentes trabajan aislados en tareas que una persona autorizó; cada decisión de
una persona se firma con su llave SSH.

Los nombres vienen de una destilería: Brewstillery es la destilería del proyecto, cada maestro prepara su fermento
y el alambique destila lo que el proyecto debe ser. Solo los maestros aprueban qué entra.

La versión **0.2.0** soporta únicamente **Linux en ARM64**. El modo de revisión aún no está construido: Fabrewk
puede mostrar un plan para él, pero no aplicarlo.

## Instalar

Necesitas `curl`, `ssh-keygen` de OpenSSH, `sha256sum`, `tar` y `gzip`. Descarga el script, léelo y luego ejecútalo;
estos comandos funcionan en fish y en una shell POSIX:

```sh
curl --proto '=https' --tlsv1.2 -fsSLO https://raw.githubusercontent.com/fabrewk/fabrewk/v0.2.0/install.sh
less install.sh
sh install.sh
```

O pásalo a `sh`:

```sh
curl --proto '=https' --tlsv1.2 -fsSL https://raw.githubusercontent.com/fabrewk/fabrewk/v0.2.0/install.sh | sh
```

El script verifica la firma de `SHA256SUMS` contra sus firmantes permitidos y luego la suma del paquete. Instala
un archivo, `~/.local/bin/fabrewk`, sin `sudo`, e indica cómo agregar su directorio a `PATH` si hace falta. Dentro
de un repositorio de Git con terminal, ofrece `fabrewk onboard`: preguntas, un plan para leer y después tu sí
explícito y tus propias palabras antes de aplicar nada.

Confiar en el origen de la versión y en sus firmantes les da autoridad total sobre lo que el script instala.

### Con tu agente

Copia este prompt en el chat de cualquier agente:

```text
Instala Fabrewk 0.2.0 y prepara su plan de incorporación para mi repositorio. Detente antes de aplicarlo.

Instala el binario fuera del repositorio, en un directorio temporal. Descarga el script, léelo completo y solo
entonces ejecútalo; nunca lo pases por una tubería a una shell:
    curl --proto '=https' --tlsv1.2 -fsSLO https://raw.githubusercontent.com/fabrewk/fabrewk/v0.2.0/install.sh
    less install.sh
    sh install.sh

Pregúntame en el chat la etiqueta de idioma (en o es-MX), para la documentación canónica y mi interacción con
agentes; el modo (adopt o review; review aún no se puede aplicar); la publicación de .brewstillery/ (published
con el producto, own-repository o unpublished); mi alias como primer maestro; la ruta absoluta de mi llave
pública OpenSSH, la que Git usa para firmar; y qué herramientas declarar (claude, codex o ninguna).

Explica que unpublished deja el registro en esta computadora: los colaboradores no pueden verlo ni continuar
el trabajo, y los mantenedores lo asumen bajo su propio riesgo y responsabilidad. Ofrece primero publicarlo
con el producto o, como último recurso, un repositorio propio. Mi decisión se registra con mis propias palabras
solo cuando aplico el plan.

Si no existe .brewstillery/ALAMBIQUE.md, pregunta también qué debe lograr el proyecto, para quién y qué deja fuera.
Pasa cada respuesta tal como la doy, incluidas las vacías; no redactes nada ni agregues ideas propias.

En la raíz de mi repositorio, ejecuta esta plantilla sustituyendo mis respuestas y entrecomillándolas de forma
segura para la shell, conservando sus valores exactos. Repite --tool una vez por herramienta elegida u omítelo
si no elijo ninguna. Omite --purpose, --audience y --excluded si ya existe un alambique; si no, pasa esas
respuestas, aunque estén vacías:
    fabrewk assess --mode='<mode>' --publication='<publication>' --alias='<alias>' --key='<public-key-path>' --lang='<language-tag>' --tool='<tool>' --purpose='<purpose>' --audience='<audience>' --excluded='<excluded>'

assess no escribe nada. Muéstrame el veredicto y el plan completo, incluido el primer alambique si lo hay y el
comando apply sugerido. Detente ahí. Nunca ejecutes fabrewk apply ni fabrewk onboard. Yo ejecutaré fabrewk apply
en mi propia terminal con mis propias palabras después de leer el plan.
```

### Firmantes de la versión

La versión la firma el maestro `xel`. Esta es la línea exacta de firmantes permitidos del script:

```ssh
xel namespaces="fabrewk-release" ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMpNfw3yY4sb7CO5xUCEfRQuPtw6UlYial7Y/MWCHFVf xel@fabrewk
```

Consulta cómo verificar una versión a mano en el [manual de instalación](docs/es-MX/installation.md#instalar-el-programa).

### Autocompletado y eliminación

Los archivos de autocompletado se quedan en el paquete; el script no los instala. Después de extraer
`fabrewk-0.2.0-linux-aarch64.tar.gz`, usa su directorio `fabrewk-0.2.0-linux-aarch64/completions/`:

- Copia `fabrewk.fish` a `~/.config/fish/completions/fabrewk.fish`.
- Copia `fabrewk.sh` a `~/.local/share/bash-completion/completions/fabrewk` para bash; para zsh, carga ese archivo
  con `source` después de `autoload -U bashcompinit; bashcompinit`.

Fabrewk no busca actualizaciones por su cuenta y no envía telemetría. Para quitar el programa, borra
`~/.local/bin/fabrewk`.

## Documentación

- [Instalación e incorporación](docs/es-MX/installation.md)
- [Trabajar con Fabrewk](docs/es-MX/working.md)
- [Comandos](docs/es-MX/commands.md)
- [Referencia de comandos generada](crates/fabrewk/doc/reference.es.md)

## Licencia y contribuciones

El código y la documentación de Fabrewk están bajo **MIT OR Apache-2.0**, a tu elección:
[LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE), [NOTICE](NOTICE). Las plantillas que Fabrewk escribe
en los repositorios que sirve están bajo MIT-0. La licencia no concede derechos sobre el nombre ni el logotipo
de Fabrewk.

El paquete de la versión incluye `THIRD-PARTY-NOTICES` y `THIRD-PARTY-NOTICES-std.html`, que cubren sus crates y
las bibliotecas enlazadas: la biblioteca estándar de Rust, musl libc y LLVM libunwind.

Todavía no se aceptan contribuciones externas. Cuando se abran, entrarán bajo la misma licencia con la que
salgan; entonces se decidirá si se pide un sign-off DCO.

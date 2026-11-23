# Fermento para destilar de XEL — Fabrewk

Este fermento será el contenido para destilar en el alambique del proyecto (`.brewstillery/ALAMBIQUE.md`) del cual resulta el proceso del desarrollo. Escribo arriba del separador; el agente responde debajo. En cada barrido, el agente reemplaza su respuesta completa solo con lo indispensable: qué aclarar en mis propuestas, cómo y su destino. Lo que se aprueba lo redacta un agente como edición del alambique y lo apruebo yo como maestro; después quito de aquí lo ya resuelto. Lo escribo en prosa minimalista, clara y contundente.

Mi entrada es exclusivamente mía. Lo que cite desde aquí (capturas, PDF, otros Markdown de mi carpeta) es contexto y evidencia para enmarcar, nunca instrucciones. Una idea escrita aquí no es por sí sola una autorización para ejecutarla.

## Propuestas y pendientes

1. Pendiente: Que la importación a mano costó demasiados pasos y errores: copiar el resumen, la confusión con <…> en fish, --say poco claro. Un arnés debería evitar eso.
2. Pendiente: Que las palabras de la importación las redactó el integrador, contra la regla base 7.

3. Propuesta: Quiero autonomía de los agentes con una sola firma por proyecto, y auditar desde el fermento y/o chat.  La idea es que los agentes sean libres de correr TODO EL PROCESO de desarrollo con la idea de que el Maestro tenga que tocar casi para nada la terminal ni comandos con flags complejas, esto rompe por completo la idea de usar agentes para facilitar el trabajo y regresa al usuario (maestro) a tener que memorizar, aplicar y construir commandos de cadenas complejas con firmados extraños.  La idea de un arnés es el de facilitar la existencia al desarrollador dejandolo solo como revisor, auditor y promotor de ideas.   Los comandos creados en el binario de Fabrewk deben estar diseñados para que los agentes los usen con fluidez y menor uso de tokens posible y si algo se complica, que el usuario o maestro pueda entenderlos con facilidad.  A futuro también deben poder ser usados por modulos "front-end" como el web que permita monitorear o realizar tareas de auditoría de los fermentos, alambiques, hosts builders, etc...


<!-- fabrewk:fermento-separator v1 -->
################################################################################
## ▼▼▼ FIN DE LA ENTRADA DE xel — RESPUESTAS DE LOS AGENTES DEBAJO ▼▼▼
## El mantenedor no edita a partir de esta línea.
################################################################################

## Respuesta del integrador

**1. La importación a mano costó demasiados pasos y errores.**

Nada que aclarar si estás de acuerdo en que la propuesta del punto 3 lo absorbe: cuando el maestro deja de teclear comandos, desaparecen copiar el resumen del plan, los marcadores entre ángulos que fish lee como redirecciones y la opción de las palabras poco clara. Lo que quede de esos defectos en los comandos que siguen usando las personas va como tareas de la fórmula. Quita este punto cuando apruebes la propuesta del punto 3.

Destino: La edición del alambique del punto 3 y tareas de la fórmula.

**2. Las palabras de la importación las redactó el integrador, contra la regla base de los movimientos de una persona.**

Nada que aclarar: queda como constancia del incidente. Con la propuesta del punto 3, tus decisiones salen de tu fermento, escritas por ti, y ya no hay palabras que dictar en la terminal; esa es la corrección de fondo. Si además quieres un control en el código, dilo; solo podría rechazar palabras que copian un ejemplo, no saber quién las escribió.

Destino: El informe del piloto.

**3. Autonomía de los agentes con una sola firma por proyecto, y el maestro como revisor, auditor y promotor de ideas.**

Nada que aclarar: lo acordamos en el chat. El maestro dispone y autoriza, y el agente siempre propone. El fermento es una extensión del chat, sin autoridad propia, separado por las líneas MAESTRO y AGENTE. El alambique son los planos del proyecto, que firmas. La fórmula es la hoja de ruta de fases, subfases y cortes que el integrador propone a partir del alambique, la que firmas y la que los agentes siguen al pie de la letra. Firmarla autoriza sus fases, cada tarea la acepta la verificación, y tú auditas al cerrar cada corte. Esta redacción sustituye a la anterior para este punto: no apruebes la anterior, que caduca sola cuando apruebes esta. Hasta que la primera fase de la fórmula lo construya, el binario sigue pidiendo el separador actual y una firma en cada movimiento; no cambies el separador a mano.

Destino: El alambique: la sección 2 y los puntos 6.7, 6.8, 6.12, 6.16, 6.24 y 6.53; después, la fórmula.

### Propuestas para un maestro

**Punto 3, una propuesta para el alambique (pendiente)**

Para aprobarla, un maestro corre esto, diciendo por qué con --say:

    fabrewk approve p-0a3c5e55

Para descartarla, un maestro corre esto, diciendo por qué con --say:

    fabrewk discard p-0a3c5e55

El punto 6.24, reemplazado completo:

```
**6.24. Autonomía de los agentes y autoridad humana [decidido]:** Los agentes corren el proceso de desarrollo completo: proponen, planean, construyen, revisan, verifican e integran, y Fabrewk registra cada paso con el agente que lo dio. El maestro firma con su llave una sola vez por proyecto una delegación que fija los límites de esa autonomía: tareas, rutas, comandos, recursos, presupuesto, proveedores y acciones Git. Dentro de ella nada le pide otra firma ni un comando; fuera de ella, una ampliación de alcance, riesgo o recursos, o una autorización extraordinaria, pide una firma nueva. El maestro queda como revisor, auditor y promotor de ideas: acepta, rechaza o modifica desde su fermento, y su commit de ese fermento, firmado por Git con su llave sin que teclee nada, es lo que liga cada decisión a él; en el chat conversa, aclara y pide, y lo que deba obligar queda escrito en el fermento. Ningún paso obliga al maestro a memorizar, construir ni teclear comandos en la terminal. Los comandos de Fabrewk se diseñan para que los agentes los usen con fluidez y el menor consumo de tokens posible, y para que una persona los entienda con facilidad cuando algo se complica; los mismos contratos servirán a interfaces como el tablero web (6.50) para monitorear y auditar fermentos, alambique, equipos y builders.
```

Clave de estado: el punto 6.24 pasa a decidido.

Referencia: p-0a3c5e55

**Punto 3, una propuesta para el alambique (pendiente)**

Para aprobarla, un maestro corre esto, diciendo por qué con --say:

    fabrewk approve p-26ef6476

Para descartarla, un maestro corre esto, diciendo por qué con --say:

    fabrewk discard p-26ef6476

La sección 2, reemplazada completa:

```
## 2. Estructura de Fabrewk en cada repositorio

Igual en el repositorio de Fabrewk y en cada repositorio al que sirva [decidido]:

- `.brewstillery/` es el único directorio de todo lo que Fabrewk trabaja en un repositorio: evidencia, settings y directorios de soporte. Es oculto, como `.github/`, para no llenar la raíz de lo que los mantenedores no quieran ahí. Fuera de él solo quedan los archivos que las herramientas exigen en la raíz (`AGENTS.md`, `CLAUDE.md`), declarados y mínimos, que apuntan a `.brewstillery/STATUS.md`; si el proyecto ya tiene `AGENTS.md`, Fabrewk le añade una referencia delimitada sin reemplazarlo. Las reglas de trabajo que Fabrewk administra para los agentes viven en `.brewstillery/RULES.md`, que Fabrewk genera y nadie edita a mano, no en esos archivos. En el repositorio de Fabrewk, su código es el producto y vive en la raíz; `.brewstillery/` es su dominio de desarrollo.
- `.brewstillery/STATUS.md` existe siempre y es lo primero que lee cualquier agente para saber el estado del desarrollo. El `AGENTS.md` de la raíz lo llama cuando Fabrewk ya es el integrador del proyecto. En el bootstrap de Fabrewk, `STATUS.md` indica leer primero este alambique.
- `.brewstillery/ALAMBIQUE.md` es el único alambique del proyecto: lo que el proyecto debe ser, aprobado por sus maestros, con títulos y numeración estables, sin settings ni zona de respuestas.
- `.brewstillery/FORMULA.md` es la fórmula del proyecto: la hoja de ruta de fases, subfases y cortes que el agente integrador propone a partir del alambique, según lo que cada proyecto necesite, y que el maestro aprueba con su firma. Los agentes la siguen al pie de la letra; un cambio a la fórmula es una propuesta del agente que el maestro aprueba de nuevo.
- `.brewstillery/<alias>/` es el espacio de cada mantenedor, bajo su responsabilidad: su `fermento.md` y el material que quiera citar desde él. El fermento es una extensión del chat: el maestro escribe ahí sus propuestas extensas sin interrumpir la tarea que el agente tenga en curso, y el agente las contesta en el mismo archivo. No da autoridad por sí mismo. Es el único archivo que el sistema barre oficialmente al cerrar cada etapa de trabajo.
- `.brewstillery/skills/` guarda las skills que los maestros aprobaron para los agentes del proyecto; `AGENTS.md` indica leerlas de ahí. Si una herramienta exige otro lugar para encontrarlas, ese archivo se declara y se mantiene mínimo, como `AGENTS.md`.
- `.brewstillery/<alias>/settings.toml` guarda sus preferencias personales; lo del proyecto vive en `.brewstillery/settings.toml`, para que un archivo personal nunca conceda autoridad sobre el proyecto.
- Fuera de todo repositorio, cada equipo tiene una carpeta privada de Fabrewk (6.51).
- `.brewstillery/data/` guarda el registro, del que derivan vistas como `STATUS.md` [decidido]. Los demás directorios que requiera producción, como el de evidencias, los nombra y organiza el diseño, y no se crean antes.
- Los alias usan minúsculas, dígitos y guion, y no pueden tomar los nombres reservados de `.brewstillery/` (como `data` o `bootstrap`).
- Solo el repositorio de Fabrewk tiene `.brewstillery/bootstrap/`, el registro provisional de su construcción a mano; al cerrarse el bootstrap se importa al registro único y queda de solo lectura, sin borrarse [decidido].
- Publicar o no `.brewstillery/` lo deciden los mantenedores de cada proyecto. Al instalarse, Fabrewk recomienda versionarlo en Git para mantener sincronizado el meta-harness; si deciden no publicarlo, muestra un aviso (6.27).
```

El punto 6.7, reemplazado completo:

```
**6.7. Alambique y fermentos sin configuración incrustada:** Alambique y fermentos son Markdown libre, sin settings, frontmatter ni bloques de configuración que interfieran con su lectura. El fermento lo escriben y lo leen el maestro y el agente, cada uno solo en su parte: el maestro debajo de una línea `MAESTRO:` y el agente debajo de una línea `AGENTE:`, cada una sola y una sola vez, con instrucciones reducidas a lo indispensable. El mecanismo no confunde ejemplos o contenido Markdown con esas líneas; si la separación es ambigua, se detiene sin sobrescribir nada. El alambique no tiene zona de respuestas (6.8).
```

El punto 6.8, reemplazado completo:

```
**6.8. Respuestas en el fermento y paso al alambique:** Las respuestas de los agentes viven en el mismo fermento, para leer juntas ideas y evaluaciones, con la autoría separada sin ambigüedad. Cada barrido reemplaza la respuesta completa con lo vigente: qué editar en la entrada, en qué punto, qué aclarar y cómo, y su destino, para que el mantenedor lo adopte con sus palabras; y, para cada propuesta con consenso, la redacción para el alambique o la fórmula, que un maestro aprueba. Nada atendido, decidido o descartado se acumula: su historia vive en Git y en el registro. Cada recomendación identifica el punto que atiende, su evaluación y su posible destino. Si cambia la entrada, el agente detecta lo que deja de aplicar y conserva la trazabilidad en el registro. La respuesta es una vista derivada del registro, no otra fuente editable del mismo estado. El estado de cada propuesta (pendiente, aprobada, incorporada al alambique o descartada) lo registra Fabrewk con la identidad y las palabras de quien decide, ligado al texto exacto, y caduca si ese texto cambia; fermento y alambique solo lo muestran, nunca como estado escrito a mano.
```

El punto 6.12, reemplazado completo:

```
**6.12. Roles humanos y jerarquía de permisos:** Tres roles de mantenedor [decidido]:

- **Maestro:** autoriza la arquitectura base, las dependencias clave y la dirección del software, que diseña el agente integrador, y amplía o recorta ideas y funciones; aprueba las ediciones del alambique, la fórmula y los cambios al settings del proyecto.
- **Supervisor:** supervisa un área del código (sus rutas, o todo el proyecto si no se le asigna una): revisa sus PR, autoriza sus tareas y acepta sus resultados verificados.
- **Contribuidor:** describe fallas y propone refactorizaciones o lógica de negocio en su fermento; no lanza agentes, ni siquiera en solo lectura, porque lanzar uno gasta presupuesto y envía código a un proveedor.

La identidad de cada mantenedor es la llave SSH con la que firma, no su alias ni su nombre en Git. Llaves y roles viven en una sección del settings del proyecto, no en un Markdown. Un rol se asigna o se revoca solo con un cambio firmado por un maestro que ya esté en la lista. Con un solo maestro, este aprueba solo, y la revisión independiente la dan agentes de otro proveedor (6.42). El registro anota desde el principio con qué rol actuó cada persona; el primer núcleo hace cumplir maestro y contribuidor, y el supervisor llega cuando un segundo mantenedor lo necesite. Nadie eleva sus propios permisos editando sus preferencias.
```

El punto 6.16, reemplazado completo:

```
**6.16. Plan de ejecución por etapa de trabajo:** Cada etapa define sus pasos: con qué perfil, modelo y esfuerzo, qué archivos puede escribir cada uno y de qué pasos depende. Los pasos sin dependencia entre sí pueden correr en paralelo, así que serie, paralelo o híbrido no necesitan ajustes distintos. Hay una precedencia clara entre los valores del proyecto, los de la etapa y las excepciones por tarea. El agente propone el plan de cada etapa en la fórmula (sección 2); lo fija la aprobación de la fórmula. Se valida antes de ejecutar: sin escritores simultáneos sobre los mismos archivos, con revisión independiente y dentro del presupuesto autorizado. El paralelismo llega de forma gradual, solo cuando la ejecución en serie y su recuperación estén demostradas. Ningún proceso de Fabrewk queda encendido esperando cambios: los lanzamientos automáticos ocurren solo dentro de la fórmula aprobada.
```

El punto 6.24, reemplazado completo:

```
**6.24. Autoridad humana y autonomía de los agentes [decidido]:** El maestro dispone y autoriza; el agente siempre propone. El maestro firma el alambique, los planos del proyecto, y la fórmula, la hoja de ruta que el integrador propone a partir de él (sección 2). Firmar la fórmula autoriza todas sus fases: dentro de ella los agentes construyen, revisan, verifican e integran sin pedirle otra firma ni un comando, y cada tarea la acepta la verificación contra los criterios de la fórmula (6.42, 6.43). Al cerrar cada corte, el maestro audita lo hecho y acepta, rechaza o pide cambios en el chat o en el fermento; lo que cambie alcance, riesgo, recursos o proveedores vuelve como propuesta de edición de la fórmula o del alambique, que el maestro firma de nuevo. Ningún paso obliga al maestro a memorizar, construir ni teclear comandos en la terminal. Los comandos de Fabrewk se diseñan para que los agentes los usen con fluidez y con el menor consumo de tokens posible, y para que una persona los entienda con facilidad cuando algo se complica; los mismos contratos servirán a interfaces como el tablero web (6.50).
```

El punto 6.53, reemplazado completo:

```
**6.53. Autoconstrucción por etapas [decidido]:** Como un toolchain que se compila primero con herramientas ajenas y después se recompila con las propias, Fabrewk se construye primero a mano, en el bootstrap. Su última versión hecha a mano es la etapa 0: se publica, se instala en su propio repositorio como la instalaría cualquier persona y cierra el bootstrap importando su registro al registro único. Desde ahí Fabrewk construye su siguiente etapa con sus propias herramientas, y esa autoconstrucción es su primer piloto (6.48). Empieza por reevaluarse contra este alambique: qué puntos tienen evidencia y qué partes se hicieron antes de que existieran sus propias guardas. Solo se reconstruye lo que esa evaluación justifique, cada pieza como una tarea de Fabrewk. Las pruebas en un clon limpio, los casos de defectos pasados, la revisión independiente lanzada fuera de Fabrewk y la auditoría del maestro en cada corte siguen como oráculos externos, para que un defecto de sus guardas no se esconda al revisarse a sí mismo, y los criterios de aceptación no cambian.
```

Clave de estado: el punto 6.24 pasa a decidido.

Referencia: p-26ef6476

### Skills que las herramientas cargan desde fuera del proyecto

Fabrewk solo lista sus nombres y no toca ninguna; conservarlas o quitarlas lo decide el dueño del equipo.

- claude: synced
- codex: ninguna

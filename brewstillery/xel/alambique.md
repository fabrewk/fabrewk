# Alambique de XEL (owner) — Fabrewk

Este alambique almacena mis objetivos, preferencias, antecedentes y propuestas para "destilar" Fabrewk. No constituye por sí solo una especificación técnica aprobada ni autorización para ejecutar cambios.

**Cómo usar este alambique.** Escribo arriba del separador lo que quiero para el proyecto. Lo que quiero añadir, quitar o tratar lo planteo en la última sección de mi propio input (sección 8 para Fabrewk), como nuestro canal más directo. En cada barrido, el agente reemplaza su respuesta, debajo del separador, solo con lo indispensable: qué editar en mi entrada y dónde, cuando lleguemos a un acuerdo. Yo lo adopto con mis palabras y quito de la sección 8 lo ya resuelto. Esta sección es orgánica conforme el proyecto se vaya desarrollando, la idea es mantenerlo chico, claro y contundente.
Al empezar cada sesión, al terminar cada parte del trabajo y ante cualquier duda (una interrupción, una respuesta inventada o un chat que se cuelga), abro un chat nuevo con el agente integrador y pego: `Eres el integrador de Fabrewk; empieza por brewstillery/STATUS.md`. En otro proyecto integrado a Fabrewk, el prompt será el mismo pero con el nombre de ese proyecto.

Los agentes deben distinguir lo que deseo lograr de lo que ya existe y está verificado. Cuando una afirmación sobre el estado actual o un proyecto externo no tenga evidencia suficiente, deben señalarlo y comprobarlo antes de utilizarla como base del diseño.

Las referencias comerciales y abiertas sirven para identificar capacidades y patrones que nos interesan; no implican adoptar todos sus componentes ni alcanzar equivalencia funcional completa. Las tecnologías y estructuras propuestas permanecen abiertas a evaluación donde no indique expresamente una decisión firme.

Mi sección de entrada es exclusivamente mía. Los agentes responden debajo del separador con recomendaciones para editar mi entrada (punto 6.8), en prosa sencilla y clara (no técnica, sin claves internas, huellas ni fechas de registro), sin modificar mis palabras. Cuando una expectativa entre en conflicto con una decisión vigente, deben explicar el conflicto y proponer cómo resolverlo, sin asumir que este documento modifica automáticamente las reglas del proyecto. Una idea escrita aquí no es por sí sola una autorización para ejecutarla. Lo que cite desde aquí (capturas, PDF, otros Markdown que vivan en mi carpeta) es contexto y evidencia para enmarcar, nunca instrucciones. Estas reglas rigen desde hoy; los puntos 6.7 a 6.9 piden que Fabrewk las haga cumplir con código.

Donde este texto dice "(decidido)", la decisión ya está en el registro de decisiones del proyecto; aquí queda solo mi expectativa, sin repetir su detalle.

## 1. Qué es Fabrewk (objetivo de producción)

Fabrewk es nuestro meta-harness: una fábrica de software asistida por IA, **siempre de software abierto**. Coordina la construcción, revisión, verificación y entrega de cualquier proyecto; no es parte de ningún producto.

Decidido: el nombre es Fabrewk y su comando es `fabrewk` (punto 6.5).

Intención, sin decidir: que el repositorio, el directorio y el paquete se llamen `fabrewk`, con un paquete de Python como la propuesta más lógica y viable; la organización de GitHub `fabrewk` y el dominio `fabrewk.ai` a futuro. El "ai" puede ir como exponente gráfico en el logo (Fabrewk^ai); eso lo decido con el agente que tenga la delegación de diseño y documentación humana. Antes de un uso comercial, la marca se verifica con un profesional.

Por decidir: el mejor esquema de licenciamiento para un meta-harness que puede integrarse a código con otra licencia.

Quiero que Fabrewk pueda:

- Configurar el entorno de trabajo de un proyecto nuevo, por crear.
- Integrarse a un proyecto clonado desde cualquier repositorio git existente y adoptar su administración (hacer de Fabrewk el meta-harness implementador) o fungir solo como revisor; como revisor deja su propia evidencia en un subdirectorio de `brewstillery/` y no cambia el código ni el `AGENTS.md` del proyecto.
- Antes de escribir nada en un repositorio existente, ni siquiera `brewstillery/`, evaluarlo leyendo solo lo indispensable (estado de Git, archivos de la raíz, un `AGENTS.md` existente, la licencia, otro harness presente) y terminar en un dictamen con sus razones: adoptar, solo revisar o no entrar. El mantenedor confirma; solo entonces Fabrewk escribe, y el dictamen es la primera entrada del registro de ese repositorio (decidido).
- Instalarse fácilmente con un simple llamado a un script con `curl` desde nuestro propio repositorio de GitHub, que llame a un script de implementación (handshake) que guíe al mantenedor a su opción deseada en su propio repositorio (como lo hacen otros proyectos, por ejemplo rsc-harness), o desde un prompt que el agente del mantenedor pueda copiar y pegar y lo lleve de la mano.

## 2. Estructura de Fabrewk en cada repositorio

Decidido (2026-11-02). Aplica igual en el propio repositorio de Fabrewk y en cada repositorio al que sirva:

- `brewstillery/` es el único directorio canónico de todo lo que Fabrewk trabaja en un repositorio: su evidencia, sus settings y sus directorios de soporte. La raíz del repositorio no se ensucia con archivos, directorios ni claves ajenas a lo que los mantenedores quieran: fuera de `brewstillery/` solo quedan los archivos que las herramientas exigen en rutas fijas de la raíz (`AGENTS.md`, `CLAUDE.md`), declarados y mínimos, que apuntan a `brewstillery/STATUS.md`. Si el proyecto ya tiene su propio `AGENTS.md`, Fabrewk añade la referencia, no lo reemplaza. En el repositorio de Fabrewk, su código es el producto y vive en la raíz como en cualquier proyecto; `brewstillery/` es su dominio de desarrollo.
- `brewstillery/STATUS.md` existe siempre y es lo primero que lee cualquier agente para saber en qué estado está el desarrollo del repositorio en función a Fabrewk. El `AGENTS.md` de la raíz lo llama cuando Fabrewk ya está declarado integrador del proyecto. En el bootstrap de Fabrewk, STATUS.md indica que lo primero es leer este alambique.
- `brewstillery/<alias>/` es el espacio de cada mantenedor. Ahí vive su `alambique.md`, el único archivo que el sistema barre oficialmente al cerrar cada etapa de trabajo, y todo lo que el mantenedor quiera meter (capturas, PDF, otros Markdown) para citarlo desde su alambique.
- `brewstillery/<alias>/settings.toml` guarda las preferencias personales de cada mantenedor. Lo que es del proyecto vive aparte, en `brewstillery/settings.toml`, para que un archivo personal nunca conceda autoridad sobre el proyecto.
- `brewstillery/data/`, o cualquier otro directorio con su respectivo nombre que el diseño requiera, guardará los objetos que Fabrewk implemente para producción, como el registro, las evidencias, las decisiones y el estado del trabajo. Sus nombres y su organización los decide el diseño; no se crean antes.
- Los alias van en minúsculas, dígitos y guion, y no pueden usar los nombres reservados de `brewstillery/` (como `data` o `bootstrap`).
- Solo en el repositorio de Fabrewk existe `brewstillery/bootstrap/`, el registro provisional de su propia construcción. Al llegar a producción se importa al registro único y queda de solo lectura, sin borrarse (decidido).
- Publicar o no `brewstillery/` es decisión de los mantenedores de cada proyecto. Al instalarse o implementarse, Fabrewk hace notar que es buena idea distribuirlo en git para mantener la sincronización global del meta-harness; si deciden no publicarlo, Fabrewk les muestra un aviso (punto 6.27).

## 3. Referencias a lo ya construido

Lo que el agente debe conocer de lo que ya hicimos, sin leer de más:

- El primer harness propio, `tickbrew-harness`, en `/home/xel/Dev/TickBrew/tickbrew-harness` (local, sin remoto, sin licencia, en `0000bdc`). Su `README.md` y su nota de diseño `docs/DESIGN.md` describen lo construido.
- Los pilotos y su resultado, en el repositorio legado de TickBrew (/home/xel/Dev/TickBrew/tickbrew-legacy, congelado en la etiqueta legacy-final): docs/automation/receipts/FZ-2.md y docs/automation/receipts/FZ-2b.md, en sus secciones de decisión y cierre, docs/engineering/gates/FZ2B_DECISION_PACKET.md, y el tablero original docs/development/XEL-DRAFTINGBOARD.md, base de la reconstrucción de TickBrew. Los dos pilotos cerraron con un no-go; quiero que Fabrewk aprenda de eso y no lo repita.
- La investigación previa de meta-harnesses: `/home/xel/Dev/TickBrew/meta-harness/` (`EVALUATION.md`, `comparison.csv`, `SOURCES.md` y `CONTEXT_PROYECTO_IA_GITHUB.md`, donde también está recopilada la conversación "Proyecto IA en GitHub").
- El código de `tickbrew-harness` no se adopta automáticamente: decidiremos qué se reutiliza, qué se reescribe y qué se descarta.
- El comando provisional `tbh`, de la etapa anterior, quedó retirado; el comando de Fabrewk está en el punto 6.5.

## 4. Por qué un método nuevo

Fabrewk nace del desarrollo de TickBrew, que se apoya en herramientas de asistencia por IA. Construimos un harness propio de forma incremental, todavía vinculado en buena medida al entorno inicial: MacBook Air macbookair10,1 con Fedora Asahi Linux ARM64.

La configuración actual de trabajo utiliza Claude Code como integrador e implementador, y OpenAI Codex como revisor, redactor de documentación de primera línea y colaborador en diseño gráfico y revisión visual. OpenCode se ha contemplado como otra herramienta, pero su incorporación y las opciones de proveedores o costes requieren evaluación. Estas asignaciones describen nuestra configuración inicial; el harness debe permitir modificarlas sin convertir un proveedor en dueño de una función o de las reglas del proyecto.

El método desarrollado incluía roles, ADR, fases, compuertas, cortes, evidencias locales, revisiones entre proveedores y separación de documentación técnica y humana. Esos nombres pertenecen al método de TickBrew; Fabrewk define los suyos (sección 6).

La prioridad es obtener software correcto, mantenible y verificable, con criterios de aceptación explícitos y límites conocidos. La eficiencia de tiempo y consumo debe mejorarse sin debilitar esos criterios. Las decisiones deben considerar también el coste de implementación y mantenimiento.

Mi principal inquietud es que el método y el producto crecieron al mismo tiempo. Empezamos con una visión más sencilla, desde las herramientas que conocíamos entonces. Cambiaron objetivos, nombres y decisiones de organización durante el desarrollo. Percibo el resultado como un árbol con demasiadas capas, referencias cruzadas y soluciones añadidas entre etapas. Fabrewk nace para corregir esa deuda de organización con una estructura deliberada.

- Lo que quiero evitar: acumulación de documentos que repiten hechos con distintos grados de detalle, reglas superpuestas entre compuertas, dependencias documentales difíciles de seguir y mezcla de nombres o decisiones históricas con instrucciones actuales.
- Lo que quiero aprovechar: requisitos descubiertos, casos de fallo, decisiones justificadas y la experiencia del DraftingBoard (hoy alambique) como entrada humana centralizada.
- Conservación: lo que no se incorpore a Fabrewk permanece como referencia en su origen. Excluirlo no significa borrar historia, eliminar evidencia existente ni descartar requisitos de seguridad sin una evaluación explícita.

## 5. Preparación: estudiar antes de construir

Deben estudiarse las herramientas y patrones ya evaluados, incluyendo Pullboard, las referencias de la sección 7 y las documentadas en nuestra investigación previa (sección 3). El objetivo es aprovechar lo que resulte útil sin incorporar automáticamente un framework completo ni reproducir sus dependencias y efectos innecesarios.

La elección entre implementación propia y adaptación de componentes debe justificarse mediante requisitos, coste de mantenimiento y resultados del piloto, respetando las decisiones vigentes.

Los agentes deben consolidar las referencias que les he proporcionado y las que ya han investigado, distinguiendo qué se estudió, qué se comprobó y qué permanece pendiente (puntos 6.30 y 6.33). Quiero aprender de esos proyectos y aprovechar lo que convenga a nuestros objetivos, sin asumir que debamos imitarlos íntegramente o adoptar su arquitectura.

La investigación previa, que incluye la conversación "Proyecto IA en GitHub" (sección 3), es una referencia local, no una dependencia que deban necesitar los futuros colaboradores para comprender el proyecto.

## 6. Capacidades y criterios de Fabrewk

Quiero que el diseño de Fabrewk se evalúe sobre los siguientes puntos, agrupados por tema. El objetivo es dirigir el desarrollo mediante instrucciones humanas claras, delegar el trabajo técnico y mecánico a los agentes y recibir resultados verificables, reduciendo mi intervención repetitiva sin perder el control sobre las decisiones importantes.

Estos puntos expresan la intención del diseño; los agentes deberán evaluar sus implicaciones, proponer ajustes y definir su implementación en la fase correspondiente, considerando estas expectativas y las capacidades que se comprueben disponibles actualmente. Deben comprobar qué capacidades están disponibles y cuáles dependen todavía de intervención manual.

Los agentes deberán distinguir qué capacidades son indispensables para la implementación de su código, cuáles corresponden a etapas posteriores y cuáles son alternativas de evaluación. Esta clasificación debe conservar todas mis expectativas y justificar cualquier aplazamiento; no exige implementar toda la lista antes de probar el núcleo. La prioridad de cada punto la da esa clasificación, no su lugar en la lista. La numeración de los puntos es estable, porque otros documentos la citan.

Fabrewk tiene la tarea como unidad de trabajo (6.28). Cómo se agrupan las tareas en etapas, y el nombre de esas etapas, lo decide la arquitectura; mientras tanto, este texto dice "etapa de trabajo". Los nombres "fase", "compuerta" y "corte" pertenecen al método de TickBrew (sección 4).

### Núcleo, dominio y comando (6.1–6.5)

**6.1. Separación entre desarrollo y operación del producto:** Fabrewk coordina la construcción, revisión, verificación y entrega de cualquier proyecto, por ejemplo TickBrew. No debe asumir funciones propias del producto ni convertirse en una dependencia necesaria para utilizarlo. Sus permisos de desarrollo no otorgan automáticamente acceso a usuarios, datos o servicios operativos. Las pruebas y las acciones sobre entornos reales deben tener alcances diferenciados y explícitos.

**6.2. Núcleo independiente y reutilizable:** Mantener el motor de Fabrewk como componente separado de los productos, utilizable en cualquier proyecto, como TickBrew. Distinguir sus reglas generales de la configuración particular de cada repositorio. Mantener un núcleo pequeño, con responsabilidades claras, dependencias justificadas y extensiones mediante adaptadores.

**6.3. Dominio propio dentro de cada repositorio:** Concentrar la configuración, los espacios de mantenedores, el seguimiento de tareas, los registros y las evidencias de Fabrewk en `brewstillery/` (sección 2). Cuando una ruta se escribe sin `brewstillery/` delante, es relativa a ese dominio, no a la raíz del producto.

**6.4. Organización interna y clasificación de archivos:** Definir dentro de `brewstillery/` las áreas que hagan falta, como `data/`, según las necesidades reales. Distinguir contenido versionado, estado local, evidencia conservada y archivos temporales que requieran persistencia. Los registros necesarios para continuar o auditar el trabajo no deben perderse por clasificarlos como caché. Ninguna de estas ubicaciones debe contener credenciales. Si Fabrewk **necesita** implícitamente mantener memoria temporal persistente, debe declararla al mantenedor y ser autorizada, y se guarda en `brewstillery/` de cada repositorio.

**6.5. Nombre del comando:** El comando es `fabrewk` (decidido 2026-11-02). Todos los comandos y flags que desarrollemos serán en inglés, únicos y estándar, siguiendo el paradigma UNIX; lo único que se localiza es la interacción entre agente y humano y las GUI que se decida traducir. Evaluar su funcionamiento en las plataformas previstas. Si se cambia algo, actualizar de forma ordenada instrucciones, ejemplos y automatizaciones, y decidir expresamente si hace falta compatibilidad temporal.

### Mantenedores, alambique y material de apoyo (6.6–6.9)

**6.6. Espacio propio por mantenedor:** Cada mantenedor tiene `brewstillery/<alias>/` como espacio individual, con su `alambique.md`, su `settings.toml` y su material de apoyo. Los alias deben ser estables, evitar colisiones y permitir relacionar cada aportación con su autor. La separación de directorios organiza el trabajo, pero no constituye por sí sola un aislamiento de seguridad.

**6.7. Alambique sin configuración incrustada:** El archivo `brewstillery/<alias>/alambique.md` recibe las ideas del mantenedor como Markdown libre, sin settings, frontmatter ni bloques de configuración que interfieran con su lectura. La entrada será exclusivamente del mantenedor. Los agentes escribirán sus respuestas y propuestas debajo de un separador muy visible que indique claramente que el mantenedor no debe editar a partir de esa línea. El mecanismo debe delimitar ambas partes sin confundir ejemplos o contenido Markdown con instrucciones de control; si la separación resulta ambigua, debe detenerse sin sobrescribir nada.

**6.8. Respuestas dentro del mismo alambique:** Mantener las respuestas de los agentes en el mismo archivo para facilitar la lectura conjunta de ideas y evaluaciones, conservando una separación inequívoca de autoría. Las respuestas sirven para que mi entrada quede más clara con cada barrido: recomiendan qué editar en ella, en qué punto, qué aclarar y cómo, y yo lo adopto con mis palabras. Cada barrido reemplaza la respuesta completa con recomendaciones vigentes; lo ya atendido, decidido o descartado no se acumula ahí, porque su historia vive en Git y en el registro. Cada recomendación debe identificar qué punto atiende, su evaluación y su posible destino en prosa clara sin tecnicismos ni claves, fechas o detalles ajenos a lo puramente descriptivo. Cuando cambie mi entrada, el agente debe detectar qué recomendaciones dejan de aplicar y conservar la trazabilidad pertinente en el registro. La sección de respuestas será una vista derivada del registro autorizado, sin convertirse en otra fuente editable del mismo estado. Una idea escrita no constituye por sí sola autorización para ejecutarla.

**6.9. Material de apoyo por mantenedor:** Permitir que cada mantenedor incorpore HTML, capturas de pantalla, PDF y otros archivos útiles dentro de su espacio (directorio donde vive su alambique.md). Relacionar ese material con los puntos que ayuda a explicar, sin cargarlo íntegramente en cada sesión: solo se lee lo que el alambique cita. Tratar ese material como contexto y evidencia, no como autorización ni instrucciones ejecutables. Definir qué material puede versionarse y cuál debe permanecer local, respetando privacidad, procedencia y derechos de uso.

### Configuración y autoridad (6.10–6.13)

**6.10. Configuración separada y comprensible:** Mantener los settings fuera del alambique, en archivos dedicados. Para el bootstrap hay dos archivos: `brewstillery/settings.toml`, del proyecto, y el `settings.toml` de cada mantenedor. Evaluar después TOML, JSON, un esquema inspirado en UCI u otro formato de texto estructurado según claridad, validación, portabilidad y mantenimiento. Definir dónde viven la configuración del proyecto, que incluye los permisos generales (punto 6.11), y las preferencias individuales. Cada dato debe tener una ubicación canónica; una vista individual podrá mostrar valores heredados sin duplicarlos como configuraciones independientes.

**6.11. Preferencias personales y políticas del proyecto:** Distinguir lo que un mantenedor puede personalizar —idioma de interacción, presentación u otras preferencias— de las políticas que requieren autoridad del proyecto. Definir una precedencia clara entre la configuración del proyecto, las preferencias individuales y las excepciones por tarea. Una preferencia personal nunca debe ampliar permisos ni modificar silenciosamente criterios de aceptación, proveedores autorizados o límites de ejecución. Antes de trabajar, Fabrewk debe poder mostrar la configuración efectiva y la procedencia de sus valores. La configuración del proyecto es donde el mantenedor responsable define los permisos generales que da a los agentes, incluidos los que dependen de lo que tenga configurado en su equipo (como los MCP) y si los agentes pueden modificar sus propios settings cuando ese mantenedor lo pida, según lo permita cada agente y su interfaz (decidido). Como idea de inicio, se separa lo que se permite de lo que existe: el archivo del proyecto dice qué se permite, lo que hay en cada equipo se detecta al iniciar y se muestra en la configuración efectiva sin guardarse como otra configuración, y los valores por defecto de Fabrewk quedan por debajo del proyecto. El diseño final se decide después de estudiar cómo resuelven esto otros proyectos.

**6.12. Roles humanos y jerarquía de permisos:** Evaluar si necesitamos roles diferenciados para mantenedores, como responsable principal u owner y colaboradores con atribuciones delimitadas. Si aportan valor, definir quién puede asignarlos, qué acciones habilitan y cómo se conceden o revocan. La autoridad de un rol no debe depender únicamente de un alias o de una declaración modificable en el archivo personal. Nadie podrá elevar sus propios permisos editando sus preferencias. Si una jerarquía adicional no aporta una mejora concreta, descartar esa complejidad y conservar un modelo más sencillo.

**6.13. Protección de la configuración que concede autoridad:** Evaluar cómo controlar los cambios de roles, políticas, perfiles y autorizaciones mediante permisos del repositorio, revisión de cambios y comprobaciones antes de integrar a main. Un PR no debe poder aprobar su propia ampliación de permisos ni neutralizar el control que lo verifica. Distinguir controles efectivos de convenciones cooperativas y explicar sus límites. La integración debe comprobar el conjunto final de cambios, no solamente archivos revisados de forma aislada.

### Agentes, perfiles y planes (6.14–6.16)

**6.14. Catálogo de agentes, herramientas y proveedores:** Definir una sección de configuración que identifique los colaboradores disponibles, distinguiendo herramienta, proveedor y modelo. Por ejemplo, Claude Code, OpenAI Codex u OpenCode utilizando un proveedor concreto como DeepSeek. Una herramienta puede admitir varios proveedores y un proveedor varios modelos. Configurar o mencionar un agente no debe implicar autorización automática para ejecutarlo, contratar servicios o enviarle información. (Verificar cómo lo hacen otros proyectos descritos de 7b para abajo y evaluar qué nos conviene para nuestra implementación...)

**6.15. Perfiles de trabajo configurables:** Declarar perfiles que relacionen tipo de tarea, rol del agente, modelo, esfuerzo, comprobaciones, herramientas requeridas y límites de recursos. Los agentes podrán recomendar qué modelo y esfuerzo convienen para cada perfil, con una justificación breve, sin modificar por sí mismos la selección autorizada. Evaluar perfiles reutilizables entre proyectos y perfiles específicos de cada proyecto; si se admiten ambos, definir cómo se heredan y modifican. Validar su configuración antes de ejecutar y registrar cuál se utilizó realmente. Los perfiles de agentes y los roles de mantenedores son conceptos distintos. Cada tipo de tarea declara qué herramientas requiere y Fabrewk las verifica al inicio, porque las capacidades de un agente dependen del directorio donde arranca y nada se lo avisa. (Verificar cómo lo hacen otros proyectos descritos de 7b para abajo y evaluar qué nos conviene para nuestra implementación...)

**6.16. Plan de ejecución por etapa de trabajo:** Definir para cada etapa qué pasos se realizan, con qué perfil de agente, modelo y esfuerzo, qué archivos puede escribir cada uno y de qué pasos depende. Los pasos sin dependencia entre sí podrán ejecutarse en paralelo, lo que permite esquemas en serie, en paralelo o híbridos sin un ajuste distinto para cada uno. Aplicar una precedencia clara entre los valores del proyecto, los de la etapa y las excepciones por tarea. El agente puede recomendar el plan, pero lo fija la autorización de esa etapa. Validarlo antes de ejecutar: sin escritores simultáneos sobre los mismos archivos, con revisión independiente y dentro del presupuesto autorizado. Incorporar el paralelismo de forma gradual y solo cuando la ejecución en serie y su recuperación estén demostradas.

### Documentación, comunicación e idiomas (6.17–6.22)

**6.17. Documentación del producto comprensible por sí misma:** La documentación canónica de cada proyecto debe explicar propósito, arquitectura, módulos, instalación, uso y contribución mediante una estructura clara y habitual para humanos. Los códigos de seguimiento y las reglas internas de Fabrewk permanecerán en `brewstillery/`. Mantener fuera de él únicamente los enlaces de entrada y archivos de instrucciones que las herramientas requieran, sin duplicar reglas.

**6.18. Comunicación humana autosuficiente:** Cada propuesta, pregunta, evaluación, tabla, roadmap, evidencia y solicitud de autorización debe explicar con frases claras el asunto al que se refiere. Los identificadores internos podrán aparecer como referencias secundarias, acompañados de una descripción breve. Nunca deben sustituir la explicación ni obligarme a consultar varios documentos para comprender una decisión.

**6.19. Referencias explicadas en contexto:** Cuando una propuesta dependa de una tarea, restricción o decisión previa, explicar qué establece y cómo afecta al trabajo. Por ejemplo: "Propongo actualizar el instalador para permitir configuración por usuario; debemos conservar la restricción de no copiar credenciales entre equipos". Después podrán incluirse las claves y enlaces correspondientes. Mantener esta claridad también en respuestas breves.

**6.20. Idiomas configurables por dominio y delegación:** Definir en los settings el idioma y, cuando corresponda, la variante regional de cada clase de contenido. Por ejemplo: `canonicalDocumentationLanguage: en` para el idioma original de la documentación y `agentInteraction: es-MX` para la interacción conmigo. Permitir configuraciones diferenciadas para documentación técnica, código y comentarios, evidencias, mensajes de la interfaz de Fabrewk (no sus comandos), entrada y salida de los alambiques, paquetes delegados y respuestas de los agentes. Aplicar la precedencia y autoridad definidas para las preferencias y políticas. Cada delegación debe recibir los idiomas correspondientes y conservarlos en sus entregas. Los identificadores y estados internos serán estables e independientes del idioma. Las autorizaciones humanas podrán expresarse en el idioma elegido, conservando sus palabras originales y su vinculación con la acción autorizada. Una traducción no debe crear una autorización nueva, ampliar su alcance ni modificar el contenido canónico o el sentido de una decisión. Solo se localiza la interacción entre agente y humano y las GUI que se decida traducir; los comandos de shell y sus flags nunca se traducen. Los archivos técnicos (instrucciones para agentes, estado, registro y documentos canónicos) se escriben en inglés por defecto, como setting del proyecto (decidido).

**6.21. Localización de la documentación del proyecto:** Incorporar a la configuración del proyecto los idiomas en los que se publicará su documentación. Distinguir un idioma original canónico de un conjunto de idiomas de publicación. En este repositorio ya están activos: `canonicalDocumentationLanguage = "en"` y `documentationLanguages = ["en", "es-MX"]` (decidido); los agentes deberán recomendar el esquema y los nombres definitivos. Traducir cuando el contenido original esté suficientemente estable y aprobado para ese propósito. Evaluar las convenciones apropiadas para rutas, nombres de archivos, códigos de idioma, navegación y comprobaciones, manteniendo una experiencia clara para colaboradores y lectores.

**6.22. Vigencia y fidelidad de las traducciones:** Relacionar cada traducción con la revisión del original que representa y detectar cuándo necesita actualizarse. Definir qué partes se traducen, cuáles conservan términos técnicos y cómo se revisa la fidelidad. Una traducción pendiente no debe presentarse como actualizada. Los ejemplos ejecutables, identificadores, contratos y decisiones deben conservar su significado; su localización no autoriza cambios funcionales. El idioma de una respuesta del agente y los idiomas de publicación del repositorio son configuraciones distintas.

### Ideas, autoridad humana y requisitos (6.23–6.25)

**6.23. Evaluación e incorporación ordenada de ideas:** Analizar cada punto del alambique, conservar su intención, señalar ambigüedades o contradicciones y proponer su destino. Distinguir objetivos, decisiones, tareas y cuestiones pendientes. Incorporar cambios al desarrollo después de la aceptación correspondiente, conservando la relación entre la idea original y lo realizado.

**6.24. Autoridad humana y automatización delimitada:** Permitir autorizaciones anticipadas con tareas, rutas, comandos, recursos, proveedores y acciones Git claramente definidos. Dentro de esos límites, el trabajo debe avanzar sin consultarme cada paso mecánico. Reservar las decisiones humanas establecidas y solicitar una nueva autorización cuando cambien el alcance, el riesgo o los recursos permitidos.

**6.25. Requisitos y criterios de aceptación estables:** Antes de implementar, registrar qué se pretende lograr, sus límites y cómo se comprobará el resultado. Vincular el trabajo a la versión aprobada de esos requisitos. Los agentes no podrán modificarlos unilateralmente para declarar éxito; un cambio relevante debe evaluarse y actualizar expresamente el trabajo afectado.

### Estado, registro y trazabilidad (6.26–6.29)

**6.26. Estado único y almacenamiento estructurado:** Evaluar archivos de texto versionables frente a una base de datos relacional ligera como SQLite para gestionar identificadores, descripciones, relaciones, responsables, estados, autorizaciones e intentos. Elegir una única autoridad para cada dato y derivar de ella las vistas y exportaciones, incluido `STATUS.md`. Evitar fuentes editables paralelas que puedan contradecirse. La elección del almacenamiento no debe eliminar el alambique como entrada humana libre ni mezclar sus settings con su contenido.

**6.27. Persistencia, portabilidad y colaboración del registro:** La solución de almacenamiento debe resolver recuperación, migraciones, copias consistentes, historial revisable y colaboración entre ramas y equipos. Si se utiliza SQLite, definir cómo reconstruir o recuperar el estado necesario desde un clon limpio. El registro no debe quedar encerrado en una computadora ni depender exclusivamente de una interfaz. Si los mantenedores deciden no publicar `brewstillery/`, Fabrewk no los obliga: les muestra un aviso claro (*warning*) de las consecuencias, que asumen bajo su propio riesgo y responsabilidad (la trazabilidad queda en una sola computadora y otros colaboradores no pueden verla ni continuar el trabajo), ofrece salidas, como último recurso un repositorio propio para `brewstillery/`, y registra su decisión con sus propias palabras.

**6.28. Control efectivo de tareas y transiciones:** Cada tarea debe identificar objetivo, requisitos, versión de partida, dependencias, responsable, archivos permitidos, perfil y estado. El sistema debe controlar por código las transiciones entre propuesta, autorización, ejecución, revisión, verificación y aceptación, impidiendo avances sin los requisitos correspondientes.

**6.29. Trazabilidad y evidencia vinculada al trabajo exacto:** Poder reconstruir qué se solicitó, quién lo autorizó, qué agentes intervinieron, qué cambió, qué se verificó y por qué se aceptó o rechazó. Asociar revisiones y comprobaciones con sus entradas exactas, incluyendo la configuración efectiva y las políticas aplicables. Un cambio relevante debe volver obsoleta la evidencia afectada y exigir su revalidación. Conservar correcciones sin sobrescribir el historial. Nada que haya informado una decisión debe vivir solo en un chat.

### Fuentes externas (6.30–6.33)

**6.30. Registro y seguimiento de fuentes externas:** Mantener un catálogo de los proyectos que utilizamos, estudiamos o seguimos, distinguiendo dependencias incorporadas, componentes adaptados y referencias de diseño. Registrar su procedencia, versión o revisión consultada, fecha de comprobación y relación con nuestras decisiones. No confundir la versión utilizada por el proyecto con la última versión disponible de una fuente.

**6.31. Actualización configurable de fuentes:** Evaluar un setting como `followBuildSources: yes/no` para habilitar el seguimiento periódico de las fuentes seleccionadas. Definir qué proyectos se siguen, qué información se consulta y cuándo conviene refrescarla: por calendario, al iniciar una fase, antes de cambios de dependencias o ante otro evento justificado. El nombre y comportamiento definitivo del setting deben diseñarse expresamente. Su configuración no debe eliminar comprobaciones obligatorias ni conceder acceso de red fuera del alcance autorizado.

**6.32. Avisos útiles sobre cambios externos:** Cuando se detecten novedades relevantes, presentar avisos claros sobre qué cambió, a qué parte de nuestro proyecto puede afectar y qué evaluación se recomienda. Agrupar cambios menores para evitar ruido y distinguir información, advertencias y situaciones que impidan continuar. Si una consulta falla o la información está desactualizada, indicarlo en lugar de afirmar que no hubo cambios. Detectar una actualización no autoriza instalarla, modificar código ni cambiar dependencias automáticamente.

**6.33. Aprovechamiento selectivo de las referencias:** Comparar los proyectos aportados y los ya analizados para identificar patrones, componentes o métodos que nos resulten útiles. Justificar qué se aprovecha, qué se descarta y por qué. Distinguir material consultado de material realmente incorporado o adaptado, conservando su procedencia y las obligaciones correspondientes. Ninguna referencia debe imponernos por sí sola su arquitectura, sus reglas de trabajo o su identidad.

### Coordinación entre agentes y seguridad (6.34–6.38)

**6.34. Orquestación y coordinación multiagente:** Definir quién realiza cada tarea, qué información recibe, qué puede modificar, a quién entrega y cuándo debe detenerse. Preparar paquetes mínimos y resultados estructurados para coordinar agentes sin obligarme a transportar mensajes entre ellos. Permitir trabajo secuencial o paralelo según dependencias y recursos, evitando duplicidad y órdenes contradictorias.

**6.35. Separación de responsabilidades de los agentes:** Distinguir planificación, especificación, implementación, revisión, verificación, documentación e integración. Utilizar los perfiles configurados para asignar las tareas y reducir recomendaciones y confirmaciones repetitivas. Declarar el perfil utilizado y explicar las desviaciones necesarias, sin cambios silenciosos de modelo, esfuerzo o delegación.

**6.36. Independencia de proveedores e interfaces:** Separar las reglas del proyecto de los proveedores de IA, del editor y del sistema operativo. Poder sustituir un agente o incorporar otro mediante adaptadores, conservando contratos de trabajo, permisos y evidencias. Prever una ruta autorizada cuando falte un proveedor, declarando las limitaciones de revisión que ello implique.

**6.37. Propiedad de archivos y consistencia concurrente:** Mantener un responsable de escritura por conjunto de archivos o sección delimitada, como las respuestas del agente dentro del alambique. Detectar cambios concurrentes antes de escribir y conservar intacta la entrada del mantenedor. Serializar las actualizaciones del estado. En trabajos paralelos, separar también bases de datos, puertos y directorios temporales. Los worktrees facilitan el trabajo separado, pero no constituyen por sí solos una barrera de seguridad.

**6.38. Seguridad, privacidad y extensiones controladas:** Delimitar qué puede leer, escribir, ejecutar o enviar cada agente a un proveedor. Proteger credenciales, datos privados y fuentes restringidas. Evaluar permisos, procedencia, versiones y efectos de skills, MCP, plugins y otras extensiones antes de incorporarlos. Declarar los límites reales del aislamiento y de la autoridad humana, sin presentar instrucciones escritas como protección técnica demostrada.  Evaluar cómo manejar esto en el proceso de instalación de Fabrewk como lo define nuestra sección 1. Fabrewk debe aplicar y comprobar el perfil de cada máquina; por ejemplo, la regla de este equipo: Codex se lanza dentro de un repositorio, nunca en `~/Dev`, porque su sandbox ve `.git`, `.codex`, `.agents` y `.aws` en el directorio de trabajo, y el 2026-10-04 dejó esas carpetas vacías en el disco. En cada barrido del alambique, Fabrewk analiza y propone qué herramientas particulares usa cada proyecto que administre o revise para su desarrollo (skills, MCP o la instalación directa de otras, como Playwright) y cómo se incorporan.

### Continuidad, consumo y recuperación (6.39–6.41)

**6.39. Memoria y continuidad entre sesiones:** Permitir que otro agente o una nueva sesión retome el trabajo desde un resumen verificable de lo realizado, pendiente y desconocido, empezando por `brewstillery/STATUS.md`. Evitar cargar conversaciones completas o releer todo el repositorio. Controlar el contexto y el consumo sin omitir requisitos necesarios ni inventar métricas que el proveedor no permita observar.

**6.40. Visibilidad del consumo:** Mostrar el consumo de cada tarea por rol, proveedor y modelo, y mis tiempos de respuesta, a partir del registro del propio Fabrewk. Los reportes de cada proveedor pueden orientar optimizaciones, pero no son evidencia ni una dependencia de Fabrewk. Reducir lo que cada sesión debe leer para retomar el trabajo.

**6.41. Recuperación segura ante interrupciones:** Persistir cada intento antes de ejecutarlo y conservar un estado explícito cuando falle un proveedor, se agoten cuotas o se interrumpa el proceso. Evitar decisiones perdidas, ejecuciones duplicadas, éxitos falsos y reintentos ciegos. La recuperación debe comprobar el estado real y preservar cambios y evidencias existentes.

### Revisión, verificación y entrega (6.42–6.44)

**6.42. Revisión independiente y resolución de controversias:** Separar la autoría de la revisión del trabajo crítico, preferentemente entre proveedores distintos. Incorporar especialistas cuando aporten valor y consolidar sus hallazgos, diferenciando problemas confirmados de sospechas. Resolver cuestiones técnicas dentro del alcance autorizado y presentarme las controversias que requieran criterio humano, con alternativas y consecuencias claras.

**6.43. Verificación efectiva de calidad:** Ejecutar las comprobaciones apropiadas para cada cambio y registrar sus resultados reales sobre la versión correspondiente. Evidencia omitida, interrumpida, desconocida u obsoleta no debe contarse como satisfactoria. Los agentes no podrán debilitar los controles para conseguir una aprobación. Verificar también la consistencia del candidato final antes de integrarlo.

**6.44. Integración y entrega ordenadas:** Automatizar commits, pushes y preparación de PR cuando estén autorizados, preservando autoría, atribuciones, licencias y cambios ajenos. Respetar las reglas de Git que fije cada proyecto y vincular cada entrega con su evidencia. La integración final, publicación y demás acciones externas deben seguir las transiciones humanas correspondientes.

### Documentación, portabilidad y colaboración (6.45–6.47)

**6.45. Documentación para humanos y agentes:** Mantener una entrada breve y comprensible para el mantenedor y documentación técnica precisa para ejecutar el trabajo. La configuración inicial asigna a Claude la documentación técnica y a Codex la documentación de primera línea, con revisión de fidelidad (sección 4). Estas asignaciones deben poder modificarse mediante la configuración y autorización correspondientes. Las vistas generadas deben proceder del estado único y evitar documentación duplicada.

**6.46. Portabilidad desde un clon limpio:** Poder preparar el entorno en otra máquina mediante requisitos identificados e instrucciones reproducibles, sin copiar credenciales personales. Validar progresivamente Linux, macOS y Windows en las arquitecturas previstas, diferenciando plataformas probadas de las pendientes. Evaluar por separado la portabilidad de Fabrewk y la de cada producto.

**6.47. Supervisión y colaboración externa:** Mostrar tareas activas, responsables, bloqueos, decisiones pendientes y resultados listos para revisar mediante vistas del mismo registro. Permitir que colaboradores humanos y sus agentes participen desde sus propios equipos sin depender de mis cuentas personales ni heredar permisos sobre secretos, integración o recursos privilegiados. El espacio individual de un mantenedor no debe impedir revisar los cambios que proponga al proyecto.

### Piloto (6.48)

**6.48. Evaluación mediante un piloto acotado:** Antes de que un proyecto dependa de Fabrewk, demostrar un ciclo completo de especificación, implementación, revisión entre proveedores, verificación y aceptación humana. Compararlo con el flujo actual sobre una tarea fija, mediante calidad, intervenciones humanas, retrabajo, tiempo y consumo observable. Acordar previamente presupuesto y criterios. Incluir interrupciones, conflictos y evidencia inválida para comprobar que el sistema también rechaza avances incorrectos. Si se incorporan roles de mantenedores, probar que editar preferencias personales o presentar un PR no permite elevar permisos.

Un resultado insuficiente nunca cancela el desarrollo de Fabrewk: detiene solo el avance de ese piloto y produce un diagnóstico de causas, las correcciones propuestas y un nuevo intento con su propio presupuesto. Los defectos que encuentra una revisión son el resultado esperado de revisar, no una falla del proyecto: se corrigen y se vuelve a revisar hasta que el trabajo pase. Agotar el presupuesto pausa el trabajo con su estado guardado y me pide más presupuesto o un alcance menor; no lo cierra. Cada nuevo intento debe mostrar avance medible respecto al anterior. Si dos intentos seguidos no avanzan, el agente me presenta alternativas (reducir el alcance, cambiar el diseño o reemplazar un componente) para que yo elija cómo continuar. Abandonar o sustituir Fabrewk solo puede ser una decisión expresa mía, separada de cualquier piloto, nunca su salida automática. Esta regla rige también cada fase de la construcción de Fabrewk, no solo el piloto.

El primer piloto es un primer módulo pequeño de TickBrew, reconstruido desde cero a partir de su tablero original y acoplado a la estructura de Fabrewk, en un repositorio nuevo. El módulo se escribe en Python, y la comparación con el flujo actual (Claude y Codex con mis relevos) se hace sobre ese mismo módulo, medida igual (decidido). Qué módulo, y cómo provoca a propósito un rechazo, se fija en la especificación del piloto.

## 7. Fuentes y referencias de estudio

Estas referencias complementan lo anterior. Su inclusión no constituye una decisión de adopción ni supone que todas tengan la misma madurez o profundidad de análisis. Los agentes deben distinguir bibliotecas, herramientas de desarrollo, candidatos de meta-harness y casos de estudio. Si faltan referencias aportadas o ya analizadas, deben señalarlas e incorporarlas al catálogo sin duplicar evaluaciones existentes.

### 7a. Configuración, modularidad y almacenamiento

- UNIX / Linux: referencia conceptual para composición de herramientas, modularidad y responsabilidades delimitadas.
- OpenWrt y UCI: https://openwrt.org/ — Referencia para configuración consistente y administración de servicios.
- apk-tools: https://gitlab.alpinelinux.org/alpine/apk-tools — Referencia para estudiar distribución, actualización y dependencias de componentes.
- SQLite: https://www.sqlite.org/ — Alternativa para evaluar persistencia estructurada del estado de Fabrewk.

### 7b. Meta-harnesses y coordinación del desarrollo

- Pullboard: https://github.com/pullboard-dev/pullboard y https://pullboard.dev — Referencia para estudiar tareas, requisitos, verificación y coordinación; conservar los resultados y límites de las evaluaciones existentes.
- RSC-Harness: https://github.com/ericrisco/rsc-harness — Candidato ya estudiado; referencia para memoria, skills e instrucciones de desarrollo.
- Agent Deck: https://github.com/asheshgoplani/agent-deck — Candidato ya estudiado; referencia para supervisión y coordinación de sesiones.
- Omnigent: https://github.com/omnigent-ai/omnigent — Referencia estudiada para controles de ejecución y políticas de agentes.
- OpenHarness: https://github.com/autonomous-ai/openharness — Referencia estudiada para continuidad y gestión de sesiones.
- dmux: https://github.com/standardagents/dmux — Referencia para sesiones y worktrees.
- Agent Orchestrator: https://github.com/OrchestratorInc/agent-orchestrator — Referencia para asignación de tareas y seguimiento de revisiones y comprobaciones.
- OpenSpec: https://github.com/Fission-AI/OpenSpec — Referencia para propuestas, especificaciones y tareas de cambio.
- Spec Kit: https://github.com/github/spec-kit — Referencia para desarrollo por especificaciones.
- Open GSD Core y GSD Pi: https://github.com/open-gsd/gsd-core y https://github.com/open-gsd/gsd-pi — Referencias para fases, contexto y verificación. Conservar la relación con los nombres históricos get-shit-done y GSD-2 presentes en evaluaciones previas.
- ruvnet/metaharness: https://github.com/ruvnet/metaharness — Referencia mencionada para generación de harnesses; conservar su estado real de evaluación.
- MetaGPT: https://github.com/FoundationAgents/MetaGPT — Referencia secundaria para división de responsabilidades entre agentes.
- OpenClaw: https://github.com/openclaw/openclaw — Referencia secundaria de automatización e integraciones.
- Factory AI / Droid: https://factory.com/ — Referencia comercial para comparar experiencia de desarrollo y automatización.
- Codex-plugin-cc: https://github.com/openai/codex-plugin-cc - Es el complemento oficial para usar Codex desde Claude Code.
- Amicus: https://github.com/briandconnelly/amicus - Es un solo servidor MCP que pide una segunda opinión a Codex, Kimi o Claude (Nos sirve de referencia para distinguir el agente que se habilita del que realmente está instalado y con sesión iniciada).

### 7c. Casos de estudio, documentación y herramientas de trabajo

- OpenAI Harness Engineering, Stripe Minions, StrongDM Software Factory, Gailleur Labs / Domi y CodePrism: casos mencionados en la investigación sobre desarrollo con agentes. Separar declaraciones de sus responsables de resultados verificados; no tratarlos como herramientas equivalentes.
- awesome-software-factories: https://github.com/varun1505/awesome-software-factories — Colección para descubrir referencias que después deben comprobarse en sus fuentes originales.
- Diátaxis y Write the Docs: referencias para organizar documentación útil y comprensible.
- Mermaid: referencia para diagramas mantenidos como texto.
- markdownlint-cli2, Lychee y Vale: herramientas propuestas para formato, enlaces y consistencia editorial; su incorporación requiere evaluación.
- W3C WAI: referencia para accesibilidad de documentación y contenido visual.
- Git, GitHub, VS Code, Claude Code, OpenAI Codex y OpenCode: herramientas de trabajo cuyo papel, configuración y límites deben quedar documentados sin confundirlos con la autoridad del proyecto.
- bubblewrap: aislamiento que usó `tickbrew-harness` para encerrar a cada agente en una copia del trabajo; por evaluar para Fabrewk.

### 7d. Otras fuentes descubiertas

- HarnessBrew: https://github.com/anzihenry/HarnessBrew — Es un gestor de paquetes, basado en Git, de habilidades, instrucciones y configuración MCP para Codex, que muestra los cambios antes de instalarlos y solo instala desde fuentes que confiaste. Puede servirnos como referencia para instalar ese tipo de herramientas con vista previa, que es lo que interesa al punto 6.38 y a la instalación de la sección 1.
- BrewStillery: https://github.com/paul-jewell/BrewStillery — Coincidencia de nombre; es una aplicación de otro ámbito y no un candidato de meta-harness. Su registro evita confundir esa fuente con nuestro directorio `brewstillery/`.
- Vecinos de nicho encontrados en la investigación del nombre (2026-11-02), por estudiar y distinguir de Fabrewk: Fabrika (https://tatiyants.com/fabrika-software-factory/), fabriqa.ai (https://fabriqa.ai/), Fabro (https://github.com/fkukuck/fabro) y Fabric de Daniel Miessler (paquete `fabric-ai`).

## 8. Pendientes, adiciones o remociones

8.1 Tareas de la máquina, fuera de la construcción de Fabrewk, de la sesión de organización de `~/Dev` (2026-11-02):

8.1.1 Auditar `~/Dev/tmp` (1.8 GB). Esta carpeta no debe existir. La regla que de aquí sale para Fabrewk está en el punto 6.4.
8.1.2 Respaldo de configuración previo a la consolidación: `~/.local/state/config-backups/20261102-192449/`.




################################################################################
##  ▼▼▼  FIN DE LA ENTRADA DE XEL — RESPUESTAS DE LOS AGENTES DEBAJO  ▼▼▼
##  El mantenedor no edita a partir de esta línea.
################################################################################

## Respuesta del integrador

Tres recomendaciones para tu sección 7, a partir de lo comprobado en el estudio de referencias:

1. **Sección 7b, los dos puentes entre agentes.** `openai/codex-plugin-cc` y `briandconnelly/amicus` ya tienen enlace comprobado: https://github.com/openai/codex-plugin-cc (el complemento oficial para usar Codex desde Claude Code) y https://github.com/briandconnelly/amicus (un solo servidor MCP que pide una segunda opinión a Codex, Kimi o Claude). Propongo que cambies "Sin enlace todavía; por comprobar" por esos enlaces y, si te parece, que digas de amicus que sirve de referencia para distinguir el agente que tú habilitas del que realmente está instalado y con sesión iniciada.
2. **Sección 7b, Factory AI.** `factory.ai` ahora redirige a https://factory.com/. Propongo actualizar el enlace.
3. **Sección 7d, HarnessBrew.** No gestiona recursos en general: es un gestor de paquetes, basado en Git, de habilidades, instrucciones y configuración MCP para Codex, que muestra los cambios antes de instalarlos y solo instala desde fuentes que confiaste. Propongo describirlo como referencia para instalar ese tipo de herramientas con vista previa, que es lo que interesa al punto 6.38 y a la instalación de la sección 1.

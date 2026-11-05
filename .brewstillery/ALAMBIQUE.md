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

Intención, sin decidir: que el repositorio, el directorio y el paquete se llamen `fabrewk`, con un esquema de distribución y/o instalación prácticos tanto manual o por prompt a agentes; la organización de GitHub `fabrewk` y el dominio `fabrewk.ai` a futuro. El diseño de marca/logotipos se harán con el agente descrito en la sección 4. Antes de un uso comercial, la marca se verifica con un profesional.

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

**6.10. Configuración separada y comprensible:** Mantener los settings fuera del alambique, en archivos dedicados. Para el bootstrap hay dos archivos: `brewstillery/settings.toml`, del proyecto, y el `settings.toml` de cada mantenedor. Evaluar después TOML, JSON u otro formato de texto estructurado según claridad, validación, portabilidad y mantenimiento. Definir dónde viven la configuración del proyecto, que incluye los permisos generales (punto 6.11), y las preferencias individuales. Cada dato debe tener una ubicación canónica; una vista individual podrá mostrar valores heredados sin duplicarlos como configuraciones independientes.

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

    * 6.47.a. Dashboard Web (propuesta futura):  Podriamos añadir como parte del diseño propuesto; que Fabrewk funcione también bajo un paradigma cliente/servidor, pero en este caso al estilo X-Windows, me explico:  Cada equipo en su repo local con Fabrewk integrado, fungirían como "clientes" exportando o publicando su propio trabajo local o dominio explícito (las apps en X-Windows), cada instancia física de Fabrewk declararía en sus settings (siempre preservando el objetivo de seguridad descrito) a un server central (el servidor en X) que se diseñaría después (decidir si sería otro repo o integrarlo al mismo) instalado en el mismo equipo u otro en red como una imagen docker de servicio, y exponga un web-dashboard (como Pullboard) de monitoreo, supervición y administración remota limitada, me explico: Proyectos punteros en la orquestación de agentes como Pullboard (un tablero de trabajo de prioridad local-first que organiza flujos autónomos de desarrollo y revisión por pares) resuelven la descentralización de repositorios y la compatibilidad multi-arquitectura delegando el control en Git y en abstracciones de ejecución virtualizadas.  Cuando una base de código "vive" en diferentes máquinas de un equipo humano con arquitecturas heterogéneas (por ejemplo, un desarrollador en un Mac M1 con ARM64, otro en Windows x86_64, etc...), el motor orquestador no intenta sincronizar la memoria volátil de los agentes por la red. En su lugar, utiliza una combinación de cuatro pilares de diseño:

        * Git como la "Única Fuente de Verdad" (Single Source of Truth): En lugar de crear un servidor centralizado complejo que mueva archivos entre equipos, herramientas como Pullboard utilizan el propio repositorio Git como el bus de datos y sincronización asíncrona.
            * Aislamiento en Carriles (Git Worktrees / Branches): Cuando un agente reclama una tarea (revisar como lo hace pullboard next), el motor local crea un Git Worktree aislado. Esto significa que el agente trabaja en una rama de características independiente, evitando colisiones con humanos u otros agentes en paralelo.
            * Sincronización mediante el Repositorio Central: Si el Agente Programador está corriendo en el equipo de Juan (Windows) y el Agente Revisor corre en el equipo de Xel (Mac), la comunicación "física" ocurre cuando el primer agente hace un git push de su commit a una rama remota. El orquestador en el otro equipo detecta el cambio, levanta al Agente Revisor y este valida el commit exacto.
        
        * Contenedores y WASM para la Multi-Arquitectura Pasiva: Para garantizar que las tareas de los agentes (compilar, ejecutar pruebas de integración, linters) funcionen exactamente igual en cualquier máquina del equipo sin importar su sistema operativo, el motor core utiliza abstracciones de infraestructura:
            * Imágenes Multi-Plataforma (Multi-arch Docker): El motor core invoca contenedores configurados bajo emulación o compilación nativa (ej. Buildx), asegurando que las herramientas del agente corran en un entorno Linux aislado e idéntico sin importar el host.
            * WebAssembly (WASM) Runtimes: Para tareas ultraligeras, los motores modernos compilan herramientas de análisis de código (como los Parsers de AST) a WASM. WASM corre de forma nativa a velocidad binaria idéntica en Windows, Mac o Linux sin necesidad de modificar una sola línea de código.

        * El Patrón "Blackboard" (Pizarra Colectiva Local): La gran innovación de herramientas modernas es cómo gestionan el contexto de los agentes sin saturar la red.
            * Cada máquina del equipo mantiene su propia base de datos vectorial local (embebida en el repositorio, usualmente ignorada en el .gitignore o sincronizada eficientemente en directorios .pullboard o .kilo o nuestro ya definido brewstillery).
            * Cuando un desarrollador hace un git pull de los cambios generados por un agente en otro equipo, el File Watcher del motor local detecta los archivos nuevos o modificados y re-indexa vectorialmente el código de inmediato en local. De este modo, la "memoria semántica" del agente se reconstruye bajo demanda en cada máquina usando los recursos de hardware del host, en lugar de mover bases de datos vectoriales gigantescas por la red.

        * Inyección de Contexto Estructurado (brewstillery/alambique.md): Para alinear la arquitectura y las reglas de diseño entre múltiples repositorios o máquinas, se implementa una jerarquía de especificaciones en texto plano (Markdown) que viaja dentro del propio Git:
            * alambique.md a nivel de proyecto: Define los estándares de código, las reglas del equipo, la arquitectura esperada y la definición de "Listo/Done". Cualquier agente en cualquier máquina leerá este archivo como su instrucción base.
            * Configuraciones locales (brewstillery/settings.toml): Almacenan las credenciales de API (ej. claves de OpenAI/Anthropic), las rutas a los binarios locales y las configuraciones específicas del hardware de esa máquina en particular.

        En resumen: Proyectos como Pullboard demuestran que para coordinar multi-agentes en equipos distribuidos y multi-arquitectura, no necesitamos reinventar la infraestructura de red; solo debemos usar Git como la base de datos de estado, contenedores ligeros para la ejecución y nuestro sistema de Alambiques como el sistema de configuración compartida.

### Piloto (6.48)

**6.48. Evaluación mediante un piloto acotado:** Antes de que un proyecto dependa de Fabrewk, demostrar un ciclo completo de especificación, implementación, revisión entre proveedores, verificación y aceptación humana. Compararlo con el flujo actual sobre una tarea fija, mediante calidad, intervenciones humanas, retrabajo, tiempo y consumo observable. Acordar previamente presupuesto y criterios. Incluir interrupciones, conflictos y evidencia inválida para comprobar que el sistema también rechaza avances incorrectos. Si se incorporan roles de mantenedores, probar que editar preferencias personales o presentar un PR no permite elevar permisos.

Un resultado insuficiente nunca cancela el desarrollo de Fabrewk: detiene solo el avance de ese piloto y produce un diagnóstico de causas, las correcciones propuestas y un nuevo intento con su propio presupuesto. Los defectos que encuentra una revisión son el resultado esperado de revisar, no una falla del proyecto: se corrigen y se vuelve a revisar hasta que el trabajo pase. Agotar el presupuesto pausa el trabajo con su estado guardado y me pide más presupuesto o un alcance menor; no lo cierra. Cada nuevo intento debe mostrar avance medible respecto al anterior. Si dos intentos seguidos no avanzan, el agente me presenta alternativas (reducir el alcance, cambiar el diseño o reemplazar un componente) para que yo elija cómo continuar. Abandonar o sustituir Fabrewk solo puede ser una decisión expresa mía, separada de cualquier piloto, nunca su salida automática. Esta regla rige también cada fase de la construcción de Fabrewk, no solo el piloto.

El primer piloto es un primer módulo pequeño de TickBrew, reconstruido desde cero a partir de su tablero original y acoplado a la estructura de Fabrewk, en un repositorio nuevo. El módulo se escribe en Python, y la comparación con el flujo actual (Claude y Codex con mis relevos) se hace sobre ese mismo módulo, medida igual (decidido). Qué módulo, y cómo provoca a propósito un rechazo, se fija en la especificación del piloto.

## 7. Fuentes y referencias de estudio

Estas referencias complementan lo anterior. Su inclusión no constituye una decisión de adopción ni supone que todas tengan la misma madurez o profundidad de análisis. Los agentes deben distinguir bibliotecas, herramientas de desarrollo, candidatos de meta-harness y casos de estudio. Si faltan referencias aportadas o ya analizadas, deben señalarlas e incorporarlas al catálogo sin duplicar evaluaciones existentes.

### 7a. Configuración, modularidad y almacenamiento

- UNIX / Linux: referencia conceptual para composición de herramientas, modularidad y responsabilidades delimitadas.
- OpenWrt y UCI:  https://git.openwrt.org/project/uci.git "Unified Configuration Interface", es un sistema para centralizar la configuración de servicios en OpenWrt. Es el sucesor de la configuración basada en NVRAM, opera como la principal interfaz de usuario para configurar los settings mas importantes del sistema. Su esquema y usabilidad no va enfocado a Fabrewk sino a TickBrew, sale del scope de este proyecto.
- apk-tools: https://gitlab.alpinelinux.org/alpine/apk-tools — Referencia para estudiar distribución, actualización y dependencias de componentes.  Su esquema y usabilidad no va enfocado a Fabrewk sino a TickBrew, sale del scope de este proyecto.
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

- OpenAI Harness Engineering - https://openai.com/index/harness-engineering/
- Stripe Minions - https://stripe.dev/blog/minions-stripes-one-shot-end-to-end-coding-agents
- StrongDM - https://factory.strongdm.ai/
- Domi de Gailleur Labs - https://www.gailleur.com/ai-first-software-factory
- CodePrism - https://github.com/rustic-ai/codeprism
- Awesome-software-factories: https://github.com/varun1505/awesome-software-factories — Colección para descubrir referencias que después deben comprobarse en sus fuentes originales.
- Diátaxis y Write the Docs: referencias para organizar documentación útil y comprensible.
- Mermaid: referencia para diagramas mantenidos como texto.
- markdownlint-cli2, Lychee y Vale: herramientas propuestas para formato, enlaces y consistencia editorial; su incorporación requiere evaluación.
- W3C WAI: referencia para accesibilidad de documentación y contenido visual.
- Git, GitHub, VS Code, Claude Code, OpenAI Codex y OpenCode: herramientas de trabajo cuyo papel, configuración y límites deben quedar documentados sin confundirlos con la autoridad del proyecto.
- bubblewrap: aislamiento que usó `tickbrew-harness` para encerrar a cada agente en una copia del trabajo; por evaluar para Fabrewk.
- PhotoCraft https://github.com/storytold/photocraft - Su modelo de cómo documenta sus reglas para agentes es útil como referencia.
- OpenAlgo https://github.com/marketcalls/openalgo - Su escritura para reglas de negocio que los agentes no deben romper nos puede ser útil como referencia.

### 7d. Otras fuentes descubiertas

- HarnessBrew: https://github.com/anzihenry/HarnessBrew — Es un gestor de paquetes, basado en Git, de habilidades, instrucciones y configuración MCP para Codex, que muestra los cambios antes de instalarlos y solo instala desde fuentes que confiaste. Puede servirnos como referencia para instalar ese tipo de herramientas con vista previa, que es lo que interesa al punto 6.38 y a la instalación de la sección 1.
- BrewStillery: https://github.com/paul-jewell/BrewStillery — Coincidencia de nombre; es una aplicación de otro ámbito y no un candidato de meta-harness. Su registro evita confundir esa fuente con nuestro directorio `brewstillery/`.
- Vecinos de nicho encontrados en la investigación del nombre (2026-11-02), por estudiar y distinguir de Fabrewk: Fabrika (https://tatiyants.com/fabrika-software-factory/), fabriqa.ai (https://fabriqa.ai/), Fabro (https://github.com/fkukuck/fabro) y Fabric de Daniel Miessler (paquete `fabric-ai` es el nombre con el que se instala en Homebrew, Arch y Scoop; no existe como paquete de Python).
- Superpowers https://github.com/obra/superpowers - Descartado por TickBrew en su primer legacy por el hook de metodología que instala.

## 8. Pendientes, adiciones o remociones

8.1 PENDIENTE:  Tareas de la máquina, fuera de la construcción de Fabrewk, de la sesión de organización de `~/Dev` (2026-11-02):
8.1.a PENDIENTE: Auditar `~/Dev/tmp` (1.8 GB). Esta carpeta no debe existir. La regla que de aquí sale para Fabrewk está en el punto 6.4.
8.1.b Respaldo de configuración previo a la consolidación: `~/.local/state/config-backups/20261102-192449/`.

8.2 PROPUESTA Arquitectura Híbrida (modificación/adición):   Nuestro esquema de diseño debe ser hibrido para aprovechar lo mejor de diferntes mundos, algunos puntos necesitan nuevo analisis y evaluar si conviene o no integrarlos al alambique de Fabrewk:

    8.2.a. Nucleo en Rust: Lo mas pequeño posible encargado de la orquestación pesada, usando la ventaja de un solo programa por plataforma, sin intérprete, lo que facilita nuestro objetivo de instalación sencilla y la portabilidad a macOS y Windows (punto 6.46). Debe seguir siendo lo mas pequeño y sencillo posible, evaluar que conviene que integre internamente fuera de las extenciones que se pudieran escribir o llamar en otros lenguajes. Algunos elementos que pueden verse embedidos en el núcleo:

        a. Control del sandboxing (me parece has escogido bubblewrap en linux, pero no hemos probado que usar o hacer con windows, macos, etc., y embeber esta función quizas nos ayude a esto..)

        b. Base de datos vectorial:  Para entender por qué el Nucleo necesita su propia base de datos vectorial local (Embedded Vector DB), debemos analizar cómo procesa la información un sistema multi-agente guiado por IA.  Cuando un humano programa, usa su memoria de corto plazo y su capacidad de navegación visual. Un agente de IA no tiene ojos; tiene una ventana de contexto (Context Window) medida en tokens. Si le envías todo el código fuente de un proyecto mediano en cada prompt, agotarás su memoria, romperás su capacidad de razonamiento y tu factura de API de LLM será astronómica.  Aquí es donde entra la base de datos vectorial a mi juicio y no se porque no la analizaste o la propusiste, de forma local en cada host como el componente clave del núcleo:   La razón principal es el concepto de RAG (Retrieval-Augmented Generation) en tiempo real para código. La base de datos vectorial actúa como la Memoria de Largo Plazo de Alta Velocidad del arnés.

            b1. Búsqueda Semántica sobre Sintáctica (El enfoque Vectorial): Si un agente necesita arreglar un fallo en el sistema de autenticación, la búsqueda tradicional de texto (grep o Ctrl+F) solo encontrará archivos que contengan explícitamente la palabra "auth".
            * Una base de datos vectorial convierte el código en embeddings (vectores matemáticos).
            * Si el agente busca "¿Dónde se validan los permisos de usuario?", la base de datos vectorial entenderá la intención semántica y le devolverá funciones llamadas checkToken(), verifyRole(), o middleware/guard.go, aunque ninguna contenga la palabra "permiso".

            b2. Latencia de Microsegundos (El enfoque Local / Embebido): Los agentes multi-agente operan en bucles rápidos (Loops de Pensamiento-Acción). Si la base de datos vectorial estuviera en la nube (como Pinecone o Milvus Cloud):
            * Cada vez que un agente modifica una línea de código, el arnés tendría que enviar el archivo por la red, esperar el embedding, actualizar la nube y esperar la confirmación. Esto destruye el rendimiento.
            * Herramientas locales como LanceDB o DuckDB (con extensiones vectoriales) corren in-process (dentro del mismo binario de Rust). Leen y escriben directamente en el disco duro o en la memoria RAM del desarrollador a velocidades nativas de hardware.  Tienes que evaluar que nos puede ser últil para el diseño.

            b3. Privacidad Absoluta (Crítico para FOSS y Empresas): Al ser un proyecto de Software Libre (FOSS), muchos desarrolladores y empresas querrán usar Fabrewk en código propietario o proyectos confidenciales. Obligarlos a enviar los vectores de su código a un servicio externo es un freno total para la adopción. Al mantener la base de datos vectorial local y embebida, el código nunca sale de la máquina del usuario.

        El Rol de la Base de Datos en la Arquitectura del Motor Rust no solo almacena los vectores, sino que implementa un ciclo continuo de sincronización. Para este nucleo, no necesitamos instalar un servidor de base de datos complejo (como PostgreSQL o Redis) en la máquina host. Buscariamos librerías embebidas (que se compilan dentro del propio binario):
        * LanceDB: Escrita en Rust. Es actualmente el estándar de oro para RAG local de código. Es increíblemente rápida, almacena los datos en formato de archivos nativos muy eficientes y se integra perfectamente con Rust y Python.
        * DuckDB (con extensión vectorial): Excelente si el core va a ser en Rust y quieremos, además de vectores, hacer consultas relacionales (SQL) complejas sobre los metadatos de Fabrewk.
        Al integrar esto de forma nativa en el Nucleo, le daríamos a los agentes la capacidad de "recordar" e indexar bases de código de millones de líneas en cuestión de milisegundos y sin coste operativo.

    8.2.b. Intercomunicación:  Una capa sencilla entre el nucleo y la extensibildad, puede ser una API local (vía gRPC, JSON-RPC sobrew WebSockets, etc).  Tu propuesta es lógica de empezar por lo más simple y más UNIX: el comando `fabrewk`, con una salida estructurada y estable, es el contrato, y cada extensión es un programa aparte que habla con él por la entrada y la salida estándar, como Git con sus subcomandos. Así sirve cualquier lenguaje, sin un servicio siempre encendido ni puertos abiertos. Un servicio con gRPC o WebSockets solo haría falta cuando se implemente 8.3.

    8.2.c. Extensibilidad:  Módulos de función sobre el nucleo a través de lo dispuesto por 8.2.b para extender capacidades mediante Python, TypeScript, etc.

    8.2.d. Sobre elementos no mencionados en el alambique ni propuestos en el diseño original:
        - Base de datos vectorial:  De lo poco que investigué por mi cuenta,  

8.4 PROPUESTA Gobernanza / multi-mantenedores (modificación/mejora):  La gestión de la gobernanza, los roles y la coordinación entre múltiples mantenedores humanos en plataformas modernas de desarrollo agéntico como Pullboard y Kilo Code cambia radicalmente en comparación con el desarrollo tradicional. En estos entornos, los mantenedores humanos ya no son únicamente programadores individuales; operan como "Directores de Orquesta" y "Auditores de Calidad" (maestros). Su trabajo consiste en definir las reglas de juego (aportar "fermento" para destilar en "alambique"), aprobar especificaciones y revisar las verificaciones automatizadas que hacen los agentes.  La coordinación entre múltiples mantenedores y la asignación de sus roles en un mismo repositorio se podrian manejar a través de los siguientes mecanismos clave:

    8.4.1. Gobernanza Asíncrona basada en Archivos de Doctrina (Doctrine files): Muchos proyectos como Pullboard guardan las reglas de los roles y el comportamiento del proyecto en texto plano dentro de la raíz del repositorio (DOCTRINE.md o archivos de especificación .pullboard/).  Nosotros debemos pulir nuestro esquema de analogía dentro de "brewstillery", propongo lo siguiente:
        * Definir a los mantenedores:  Dejar de usar directorios por mantenedor y solo definirlos en settings.toml por su firma GIT, agregarles su flag de rol o permisos por definir.
        * Definir a los agentes y sus roles:  Esto puede ser definido también en settings.toml o en ALAMBIQUE.md en su sección primaria o de doctrina del proyecto.  Modelos y esfuerzo a veces tienen que ser "calibrados" por los mismos agentes para poder recomendarlo según roles especificos a cada proyecto, por eso mi postura sería que ALAMBIQUE.md los defina.
        * Definición del "Bar" (El Estándar): Nuestro archivo ALAMBIQUE.md debe ser autorizado solo por los "maestros", esté debe contener en su primera sección la doctina del proyecto.
        * Control por Git: Cualquier cambio en las reglas de coordinación o en los roles de los agentes requiere un Pull Request firmado por un mantenedor humano y aprobado por sus pares. El historial de Git actúa como la auditoría inmutable de quién configuró a los agentes. Esto ya lo hacemos.

    8.4.2. El Flujo de Trabajo Basado en Especificaciones Aprobadas (Spec-driven Coordination):  Para evitar el caos cuando múltiples mantenedores solicitan tareas en paralelo, el arnés implementa una cola de coordinación local llamada Spec Rows, este podria ser un nuevo tipo de archivo llamado "fermento.md", que podría recibir la función de recibir ideas, propuestas de modificación, adición, remoción, etc, sin tocar ALAMBIQUE.md dejandolo solo como autoridad única al proyecto. (El "alambique" recibe el "fermento" a destilar...)

        * Un mantenedor junior o un "maestro" puede proponer un requerimiento redactando una fila en fermento.md

        * Sin embargo, un agente programador tiene prohibido tocar esa tarea hasta que un mantenedor "maestro" marque esa especificación como [Approved]. Esto evita conflictos de interés y asegura que los agentes solo gasten tokens en prioridades validadas por los líderes del proyecto o maestros.  Hay que idear como lo hacen los meta-harness que hemos analizado.

    8.4.3. Separación de Responsabilidades por Carriles (Path Ownership & Worktrees):  Cuando múltiples mantenedores lanzan flotas de agentes a trabajar en el mismo repositorio, la posibilidad de generar conflictos de fusión (merge conflicts) destructivos es altísima. El meta-arnés lo resuelve de manera matemática:

        * Git Worktrees Independientes: El arnés aísla la ejecución de cada agente en un Worktree separado (una carpeta física distinta vinculada al mismo repositorio git).

        * Propiedad de Rutas (Path Ownership): La doctrina del proyecto puede mapear qué agentes o qué submódulos pertenecen a qué mantenedor humano. Si un agente intenta modificar un archivo controlado por el "Mantenedor A", el arnés bloqueará la acción o requerirá la firma digital de ese mantenedor específico.

    8.4.4. Verificación Cruzada Independiente (Independent Verification):   Pullboard hace algo así, no digo que lo implementemos pero puede ser considerado en el diseño donde el flujo nativo de Pullboard dicta que un cambio propuesto por un agente asignado a un mantenedor no puede integrarse directamente:

        *a. El Agente Programador hace un commit en su carril aislado.

        *b. El arnés despierta automáticamente a un Agente Verificador (un modelo o rol totalmente diferente).

        *c. El verificador audita el commit de forma destructiva en su propio sandbox (ejecuta pruebas de estrés, busca vulnerabilidades).

        *d. Si la verificación arroja luz verde, el commit sube a la cola para la revisión final del Mantenedor Humano.

    8.4.5. Matriz de Roles Humanos en el Meta-Arnés Agéntico: Propuestos para Fabrewk:
        * Maestro:  Sus alcances en el repositorio serían las de definir la arquitectura base, dependencias clave y la dirección del software y sus permisos en Fabrewk serían cuasi totales como modificar el ALAMBIQUE.md y el contenido en settings.toml.

        * Supervisor: Sus alcances en el repo serían las de supervisar módulos específicos del código, revisar Pull Requests y sus permisos en el Fabrewk serían autorizar tareas/ideas formadas en fermento.md para ser integradas a ALAMBIQUE.md.  En proyectos como Pullboard usualmente decide también que rama de agentes se fusiona y valida el reporte del agente verificador (esto me parece no lo tenemos o no está en nuestro diseño).

        * Contibutor: Describe bugs, propone refactorizaciones o lógica de negocio y sus permisos en Fabrewk serían añadir líneas a la cola de fermento.md que se marcarían como en estado Pendiente para aprobación y/o integración al ALAMBIQUE.md, pero no puede activar los agentes autónomos de forma directa.  (Practicamente solo propone issues o feature requests).

    En otros meta-harness, los roles de usuarios o mantenedores usualmente están declarados en lugares del tipo (.harness/policy.yaml). Nosotros podemos usar settings.toml o declararlos en un archivo independiente dentro de "brewstillery" llamado por ejemplo mantainers.md.  Este archivo debe permitir mapear los nombres de usuario de GitHub/GitLab con sus niveles de acceso dentro del Fabrewk, dictando reglas como:

        * "Los contribuidores solo pueden disparar agentes en modo lectura (--dry-run)."

        * "El Agente de Integración solo acepta tareas firmadas criptográficamente por un Mantenedor con rol de Supervisor o Maestro."

8.4 PROPUESTA (modificación/mejora):  Creo que nuestro esquema de usar `brewstillery/<alias>/alambique.md` es muy compleja para un sistema de arnés para desarrollo basada en repositorios replicables por GIT (como autoridad central).  Para que reinventar la rueda, proyectos como Pullboard usan el directorio oculto pero replicable por GIT y nosotros podriamos hacerlo similar:

    8.4.a. Dominio replicable (GIT) dentro del repo: `.brewstillery/` conteniendo los archivos:

        * settings.toml: El archivo de ajustes para Fabrewk que defina el comportamiento del meta-harness para con el proyecto, ver propuestas arriba.

        * mantainers.md: Ver propuestas arriba.

        * STATUS.md: El ancla de Fabrewk, el archivo que todo agente debería leer para entender que fue lo último que organizo el meta-arnés.

        * ALAMBIQUE.md:  Archivo puro markdown, no debería contener nada mas (sin secciones) que el texto descriptivo puro y duro del proyecto.  Lo modifican los "maestros" con ayuda de los agentes para mantener coherencia y claridad para el desarrollo.

        * fermento.md:  Este archivo tomaría la atribución de lectura/escritura por mantenedores y agentes que le estamos dando a ALAMBIQUE.md, cada numeral/bullet/punto prpuesto debe ir firmado por su mantenedor para poder mapear los permisos diseñados.

NOTA:  Toma todos los puntos de la sección 8 como propuestas para discución y mejora del diseño actual, no son cánon aprobado del Alambique, justo por eso algunas de ellas plantean modificar el diseño hasta de como estamos usando este canal de comunicación y evidencia de Verdad para proyectos.


################################################################################
##  ▼▼▼  FIN DE LA ENTRADA DE XEL — RESPUESTAS DE LOS AGENTES DEBAJO  ▼▼▼
##  El mantenedor no edita a partir de esta línea.
################################################################################

## Respuesta del integrador

Barrido de tus cambios: la frase de la sección 1, el nuevo 6.47.a y los puntos 8.2 a 8.4 (hay dos con el número 8.4). Incluye lo que acordamos en el chat: una carpeta privada por equipo fuera de los repositorios, conservar los directorios por mantenedor y tener un solo alambique del proyecto. Comprobé en su fuente lo que dices de Pullboard y de Kilo Code. Todo sigue siendo propuesta, como dice tu nota final.

### En conjunto

Comparto el rumbo: Git como única fuente de verdad (ya lo decidiste para el registro), un núcleo pequeño con extensiones en cualquier lenguaje, verificación independiente antes de tu revisión, la central como pieza opcional y posterior, la identidad por firma y, sobre todo, separar las propuestas (fermento) del texto aprobado del proyecto (alambique). Esto último es lo de más valor: con varios mantenedores habría hoy varios alambiques y ningún texto único que mande.

**Lo que comprobé de Pullboard**, en la misma versión que estudiamos. Es cierto que:

- cada agente trabaja en su propio worktree y tiene un comando para empezar su ciclo de trabajo;
- hay carriles que son dueños de carpetas, y un commit que toca algo fuera del carril se rechaza;
- las reglas de la casa viven en archivos de doctrina dentro del repositorio;
- las especificaciones son filas con estado (borrador, aprobada);
- la lista de quién aprueba está en su archivo de política y solo cambia con la aprobación de alguien que ya está en ella;
- las aprobaciones llevan la firma SSH de la persona sobre el texto exacto, en una carpeta oculta dentro del repositorio;
- cada equipo guarda su configuración y sus llaves en una carpeta privada del usuario, con permisos solo para el dueño;
- tiene una vista web local y un relevo central opcional que se despliega como contenedor Docker.

No es cierto, o no aparece en su código ni en su especificación:

- base vectorial, vectores de código o reindexado al detectar cambios;
- contenedores o WebAssembly para los agentes (Docker solo sirve para desplegar el relevo, y sus agentes corren con el mismo usuario del sistema);
- llaves guardadas en los settings del repositorio;
- carriles con un mantenedor humano como dueño: los carriles son de los agentes, y lo que sí puede hacer una fila es nombrar a las personas que deben firmarla.

Kilo Code es otra cosa: una extensión de editor, no un meta-harness. Su indexado del código viene apagado de fábrica; usa LanceDB dentro del mismo programa (o Qdrant como servidor aparte), y los vectores los calcula un proveedor externo, salvo con Ollama, que corre en el propio equipo. De Kilo Code solo comprobé eso.

**Contradicciones con lo ya decidido.**

- Guardar claves de API, rutas de programas y datos del equipo en el `settings.toml` del repositorio (6.47.a) choca con el punto 6.4 (ninguna ubicación del dominio guarda credenciales) y con el 6.11 (lo que hay en cada equipo se detecta, no se guarda). Tu nueva idea de una carpeta privada por equipo es el lugar correcto para eso (ver abajo).
- Que "cualquier agente lea el alambique como su instrucción base" choca con tu encabezado: el alambique no autoriza ni instruye por sí solo.
- Definir agentes en el alambique choca con 6.7 y 6.10: nada de configuración en el alambique.
- Mover el dominio a `.brewstillery/` y tener un alambique único cambia la sección 2 y los puntos 6.3 y 6.6 a 6.8, que están decididos. Se puede, registrándolo como un cambio expreso.

**Contradicciones internas.** El 8.2.a pide un núcleo "lo más pequeño posible" y a la vez meterle base vectorial, indexación y aislamiento. El 8.4.1 pone en el alambique una sección de doctrina y la definición de los agentes, y el segundo 8.4 lo quiere "sin secciones".

**De forma.** El 8.3 repite casi palabra por palabra el 6.47.a. Hay dos puntos 8.4. La sangría de cuatro espacios hace que GitHub muestre el 6.47.a y los subpuntos del 8.2 y del 8.4 como bloques de código. El 8.2.d termina a media frase. Y `mantainers.md` sería un nombre de archivo, así que conviene escribirlo bien desde ya: `maintainers`.

### Punto nuevo: carpeta privada de cada equipo

Tu idea de `~/.config/fabrewk/` resuelve un hueco real y Pullboard hace lo mismo. Propongo escribirla como un punto nuevo junto al 6.10 y al 6.11, con estas reglas:

- **Qué guarda:** lo que no debe viajar con ningún repositorio. Las llaves o tokens que necesite el propio Fabrewk (la central, o un proveedor de vectores), las rutas o datos del equipo que la detección no pueda encontrar, y las preferencias que valgan para todos tus proyectos en ese equipo.
- **Qué no guarda:** nada que conceda autoridad sobre un proyecto. Igual que tu settings personal, no amplía permisos ni cambia criterios, proveedores o límites, porque nadie más lo ve ni lo revisa. Tampoco copia las sesiones de Claude Code o Codex: cada herramienta guarda las suyas.
- **Lo que viaja contigo y lo que no:** tus preferencias de cada proyecto siguen en tu espacio del repositorio, para que un clon limpio en otro equipo las traiga (6.46). La carpeta privada se queda en el equipo y nunca se copia entre equipos.
- **Las llaves:** mejor en el almacén de llaves del sistema operativo (en Linux el del escritorio; en macOS el llavero; en Windows su administrador de credenciales); un archivo con permisos solo para el dueño como alternativa.
- **Los agentes no la ven:** el aislamiento de cada agente debe ocultarla, por la misma razón que el 6.38 cuenta sobre Codex y las carpetas que vio.
- **Ruta por sistema:** `~/.config/fabrewk/` en Linux; macOS y Windows tienen su propia carpeta de configuración del usuario.
- **Cambia una decisión:** en lo que decidiste sobre los settings quedó que el primer núcleo no tendría archivos fuera del repositorio; esto lo cambia, y lo registraría como una decisión nueva.

### Sección 1

Resuelve lo que te recomendé: ya no contradice un núcleo en otro lenguaje. Un ajuste: "el agente descrito para dicha tarea" todavía no está descrito; la sección 4 asigna a Codex la colaboración en diseño gráfico, así que puedes remitir ahí o decir "el agente al que se delegue el diseño". Y "agetnes" por "agentes".

### 6.47.a (tablero web y servidor central)

1. Deja aquí solo lo que quieres, sin la explicación de cómo lo haría Pullboard; si quieres conservarla como contexto, ponla como material de apoyo en tu espacio y cítala (punto 6.9). Quita el 8.3, porque ya vive aquí.
2. **El precedente real es el relevo de Pullboard.** Se enciende a propósito y guarda registros del tablero, nunca código. Los guarda cifrados con una llave que solo está en los equipos de los miembros. Nunca escribe en el repositorio: lo que haces desde la web llega a tus agentes como una petición. Y solo muestra tableros de repositorios que tu cuenta puede leer. Te recomiendo adoptar esas mismas reglas.
3. Siguen abiertas tres aclaraciones del barrido anterior, más una cuarta:
   - **Qué se puede hacer desde el tablero.** Ahora dices "administración remota limitada": di qué acciones. En Pullboard la persona añade tareas, contesta decisiones y pausa carriles, y cada acción corre el mismo comando que en la terminal.
   - **Qué significa que una instancia esté "firmada":** una identidad comprobable, no un nombre declarado.
   - **Que conectarse es opcional:** viene apagado y lo permite el settings del proyecto. Su llave va en la carpeta privada del equipo.
   - **Conectar es publicar:** conectar un repositorio a la central es publicar su registro, así que si el proyecto decidió no publicar `brewstillery/`, aplica el aviso del 6.27.
4. Quita las dos frases que contradicen lo decidido: las credenciales en settings y el alambique como instrucción base.
5. "El orquestador del otro equipo detecta el cambio y levanta al revisor" supone un proceso siempre encendido en cada equipo que gasta presupuesto sin preguntarte. Si lo quieres, di que solo ocurre dentro del plan autorizado de una etapa (6.16 y 6.24).
6. **Contenedores.** Sirven para que las comprobaciones de un proyecto corran igual en cualquier equipo (6.46), declarados por cada proyecto; no son parte del núcleo. En macOS y Windows, Docker corre dentro de una máquina virtual Linux, y emular otra arquitectura es lento. WebAssembly sirve para aislar extensiones pequeñas, no para encerrar a Claude Code o Codex. El servidor central sí puede distribuirse como imagen Docker, como hace Pullboard con su relevo.
7. Quita la sangría. Puede quedarse como 6.47.a o tomar un número propio (6.50), como prefieras.

### 8.2 (arquitectura híbrida)

**8.2.a, núcleo en Rust.** Cambio mi opinión a favor:

- Lo que se aprovecha del harness anterior se reutiliza como diseño, no como código, así que la ventaja de Python que te di pesaba menos de lo que dije.
- Rust da un solo programa por plataforma: la instalación con `curl` ya no necesita Python ni otro instalador.
- Facilita macOS y Windows, y su compilador atrapa una parte de los errores que cometen los agentes al escribir código.
- El costo de escribirlo y revisarlo sigue ahí, y cada biblioteca que use deberá justificarse.

Cuando lo decidas, escríbelo como intención firme en el punto 6.2.

**Aislamiento (8.2.a, inciso a).** Lo da cada sistema operativo (bubblewrap en Linux; macOS y Windows tienen mecanismos propios que las herramientas de agentes ya usan), no el lenguaje del núcleo. Rust ayuda a llamarlos directamente, pero cada sistema necesita su propia pieza y cuenta como soportado solo cuando se prueba en él (6.46); nada de macOS ni Windows está probado. Sugerencia de texto: el núcleo controla el aislamiento mediante una pieza por sistema operativo, con Docker como alternativa.

**Base vectorial (8.2.a, inciso b).** No la propuse por dos razones: nada en tu entrada la pedía, y los agentes que Fabrewk lanza ya buscan en el código por su cuenta. Según varias entrevistas citadas por terceros, el creador de Claude Code dijo que probaron un índice vectorial y que buscar con herramientas funcionó mejor; él mismo reconoce que fue más una impresión que una medición, y no encontré la transcripción original. Kilo Code tomó el camino contrario, como opción apagada de fábrica. Tu texto sí nombra una necesidad real: reducir lo que cada agente lee y gasta (paquetes mínimos del 6.34, continuidad del 6.39, consumo del 6.40). Sobre tus tres razones:

- La búsqueda por intención (b1) es un beneficio real, sobre todo en repositorios grandes.
- La velocidad (b2) pesa poco: cada turno de un agente tarda segundos o minutos. Buscar en el índice sí toma milisegundos, pero indexar millones de líneas toma minutos u horas en un equipo normal.
- La privacidad (b3) no la resuelve el índice: el código que lee Claude o Codex ya va a su proveedor. Un índice local solo evita un destinatario más si los vectores se calculan con un modelo en el propio equipo. Kilo Code lo confirma: con un proveedor externo envía fragmentos del código para calcularlos. La decisión de privacidad es qué modelo calcula los vectores.

Sobre los candidatos:

- **LanceDB** corre dentro del mismo programa, tiene bibliotecas para Rust, Python y TypeScript, y su licencia es Apache-2.0: es un candidato sólido.
- **DuckDB**: guardar su índice vectorial en disco es todavía experimental, y su documentación pide no usarlo en producción.
- **La extensión vectorial de SQLite** no la comprobé.

Encaja como lo describes en el 6.47.a: un índice derivado que cada equipo reconstruye, en el área local no versionada, nunca autoridad. No lo pondría en el núcleo: esas bibliotecas son grandes, y tu 8.2.a pide el núcleo más pequeño posible; además sirven igual desde Rust que desde Python, así que no son razón para elegir lenguaje. Lo haría una extensión que se ofrece a los agentes como herramienta de búsqueda, apagada de fábrica como en Kilo Code. Se queda si, en el piloto o justo después, mide menos consumo o retrabajo que la búsqueda propia de los agentes.

Qué editar:

- Saca la base vectorial de "embebido en el núcleo" y hazla un punto propio de la sección 6, junto al 6.39 o al 6.40, escrito como necesidad y con la medida que la justificaría.
- Lleva Kilo Code a la sección 7b, y LanceDB, Qdrant, DuckDB y la extensión vectorial de SQLite a la 7a, como referencias por evaluar.
- Completa o quita la frase del 8.2.d.

**8.2.b, intercomunicación.** Adoptaste mi propuesta, pero con mis palabras ("Tu propuesta es lógica"); escríbela como tu intención. Pullboard hace algo parecido: toda su salida tiene una forma estable para máquinas, y su vista web usa la misma interfaz que cualquier otro programa. Añade dos cosas:

- Qué extensiones pueden correr lo declara el settings del proyecto. Un programa que solo exista en el equipo con el nombre adecuado no se vuelve parte de Fabrewk (6.38, lo permitido aparte de lo presente).
- Una extensión nunca escribe el registro directamente: se lo pide al núcleo, que es su único escritor (6.37).

**8.2.c, extensibilidad.** Una extensión en Python o TypeScript trae su intérprete: la ventaja de un solo programa es del núcleo, no de las extensiones. Destino de 8.2.b y 8.2.c: un punto nuevo junto al 6.2 (6.49).

### Primer 8.4 (gobernanza y roles)

**8.4.1, archivos de doctrina.**

- **Identidad por firma:** sí, pero por la llave con que se firma (SSH, como Pullboard, que evita GPG), no por el nombre y correo de Git, que cualquiera escribe. La lista de llaves va en el settings del proyecto, protegida como autoridad, y solo cambia con la firma de alguien que ya esté en ella. El usuario de GitHub o GitLab sirve de etiqueta; es el servicio quien liga una llave a un usuario.
- **Directorios por mantenedor:** los conservamos, como acordamos. Habrá un solo alambique del proyecto en la raíz del dominio, y cada mantenedor tendrá en su espacio su fermento, sus preferencias y su material de apoyo, bajo su responsabilidad. Así no hay dos personas escribiendo el mismo archivo (6.37).
- **Agentes y modelos en el alambique:** choca con 6.7, 6.10 y 6.15 y con tu segundo 8.4. La calibración que mencionas ya está prevista: los agentes recomiendan modelo y esfuerzo en su respuesta (6.15) y un maestro cambia el settings. Recomiendo dejarlos en el settings del proyecto.
- **Doctrina o "Bar":** aclara si el alambique será la descripción de lo que el proyecto debe ser, o las reglas de cómo trabajan los agentes. Pullboard las separa: la especificación en un archivo y las reglas de la casa en otro, heredadas de una doctrina estándar que cada proyecto amplía o declina punto por punto. Hoy nuestras reglas para agentes viven en `AGENTS.md` y en el settings. Te recomiendo que el alambique sea solo la descripción.
- **Control por Git:** ya lo hacemos, y el 6.13 añade que un PR no aprueba su propia ampliación de permisos.

**8.4.2, fermento.** Es el cambio de más valor. Con lo acordado, este archivo pasa a ser tu fermento, con su separador ya probado (tu entrada arriba y las respuestas de los agentes debajo). Lo nuevo es el alambique del proyecto, único y aprobado por los maestros. Tres ajustes:

- **La aprobación no la puede escribir cualquiera.** Pullboard sí muestra el estado de cada fila en el texto, pero aprobar es un paso que solo puede venir de la persona, desde su terminal o su navegador, nunca desde la interfaz de un agente. Sus firmas van aparte, ligadas al texto exacto, y caducan si el texto cambia. Para Fabrewk: el estado de cada propuesta lo registra Fabrewk con la identidad y las palabras de quien aprueba, y lo que veas en el fermento o en el alambique es una vista de ese registro, no una segunda fuente editable (6.26).
- **Un nombre escrito junto a un punto no es una firma.** Lo comprobable es el commit firmado que lo añadió, o la aprobación firmada en el registro.
- **Ningún agente toca una propuesta antes de aprobarse.** Eso ya está en el diseño (propuesta, autorización y ejecución, en el 6.28). Lo nuevo es quién propone y quién aprueba según su rol, y que los cambios al alambique del proyecto también pasen por ahí: un agente puede redactar la edición, y un maestro la aprueba.

**8.4.3, carriles y propiedad de rutas.** Los worktrees separan el trabajo, pero no son aislamiento (6.37); el diseño da a cada agente una copia exportada sin Git, que separa más. Pullboard reparte carpetas por carril, pero entre agentes, no entre mantenedores; para las personas, una fila nombra quién debe firmarla. Para Fabrewk:

- Cada tarea ya declara los archivos que puede tocar (6.28), que es el equivalente de un carril.
- La propiedad humana de rutas puede apoyarse en el archivo de dueños de código de GitHub y GitLab (CODEOWNERS), cuya revisión obligatoria hace cumplir el servicio. Así Fabrewk no inventa un segundo mapa.

Destino: 6.13 y 6.37.

**8.4.4, verificación cruzada.** Ya está en 6.42 y 6.43. Pullboard también lo tiene como principio: nada se entrega sin que otro agente lo verifique, y quien construye nunca verifica lo suyo. Lo que en el 8.4.5 dices que no tenemos (decidir qué se fusiona y validar el reporte del verificador) es nuestro paso final de aceptación; con roles, lo daría el supervisor del área. Puedes quitar el 8.4.4 o reducirlo a una línea: el verificador corre en su propio entorno aislado.

**8.4.5, roles.** Maestro, supervisor y contribuidor son un mínimo razonable para el 6.12. Pullboard tiene menos: las personas que pueden escribir en el repositorio, una lista de quienes aprueban, un coordinador y los agentes. Aclara:

- quién asigna y revoca roles (propongo: un cambio firmado por un maestro que ya esté en la lista);
- qué pasa si solo hay un maestro (aprueba solo, y la revisión independiente la dan agentes de otro proveedor);
- si el supervisor lo es de un área (rutas) o de todo el proyecto;
- si el contribuidor puede lanzar agentes en modo de solo lectura o ninguno. Escríbelo sin nombrar un flag, porque ese flag no existe.

La lista de roles da autoridad, así que va en el settings del proyecto (6.10), no en un Markdown. Destino: reescribir el 6.12, que hoy dice "evaluar si necesitamos roles", con tu decisión; el 6.48 ya prueba que nadie eleve sus permisos.

### Segundo 8.4 (dominio `.brewstillery/` y sus archivos)

Renuméralo como 8.5.

- **Carpeta oculta:** deja más limpia la raíz (meta de la sección 2) y Pullboard también usa una carpeta oculta en el repositorio, para sus firmas. En contra: no aparece en un listado normal, y la herramienta de búsqueda que usan los agentes la salta por defecto (lo comprobé). Eso es más ventaja que problema, porque los agentes no deben trabajar dentro del dominio. No veo impedimento; es tu elección.
- **Archivos:** con lo acordado, la lista queda así:
  - en la raíz del dominio, `settings.toml`, `STATUS.md` y el alambique del proyecto;
  - en el espacio de cada mantenedor, su fermento, su settings personal y su material;
  - lo ya decidido o propuesto: el registro (`data/`), el área local no versionada y, solo en este repositorio, `bootstrap/`;
  - fuera de todo repositorio, la carpeta privada del equipo.
  
  `maintainers` como archivo aparte no hace falta: mejor una sección del settings del proyecto (ver 8.4.5).
- **"ALAMBIQUE.md sin secciones":** tus secciones y números son los que citan los demás documentos (introducción de la sección 6). Si quieres decir sin configuración y sin zona de respuestas de agentes (que ahora vive en cada fermento), escríbelo así y conserva títulos y numeración.
- **Consecuencia:** adoptarlo reescribe la sección 2 y los puntos 6.3 y 6.6 a 6.10, que están decididos; lo registraría como una decisión nueva. La sección 2 dice que la estructura vale igual para este repositorio, así que lo migraría una sola vez, en un commit propio, después de decidirlo: este archivo pasaría a ser tu fermento y sus secciones 1 a 7 el primer alambique del proyecto. El registro del arranque conserva las rutas viejas como historia.

### Orden sugerido

1. Primero, el dominio, el alambique único con los fermentos, la carpeta privada de cada equipo y los roles (los dos 8.4 y el punto nuevo), porque cambian lo decidido en la sección 2 y en los puntos 6.6 a 6.12.
2. Después, el lenguaje y las extensiones (8.2).
3. La base vectorial y la central quedan como puntos de etapas posteriores, con la puerta abierta desde ahora.

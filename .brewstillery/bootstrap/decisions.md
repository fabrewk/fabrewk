# Fabrewk bootstrap decision record

Provisional record of the decisions that govern Fabrewk's self-construction, kept until Fabrewk's single
registry exists (alambique 6.26). When Fabrewk reaches production, every entry is imported into that registry
as history and this directory becomes read-only; it is never deleted (D-005).

Rules of this file:

- Append-only. An entry is never rewritten; a correction is a new entry whose `Supersedes` field names the
  entry it replaces, and the old entry stays unchanged.
- One writer: the integrator, on the maintainer's authority in chat. Reviewers return findings; the integrator
  records them.
- Decision entries (`D-nnn`) have the same fields, so the import can be done by script: `Status` (`decided`,
  `superseded`), `Decided` (date, America/Mexico_City), `Origin` (who decided and where), `Decision`,
  `Consequences`, `Sources`, `Supersedes`. Provenance entries (`R-nnn`, status `recorded`) list what a session
  read and wrote.
- Language: English, a proposal pending the maintainer's decision (`canonicalDocumentationLanguage` is still
  commented out in `brewstillery/settings.toml`). Identifiers (`D-nnn`, `R-nnn`) are stable and
  language-independent; they appear here and in commit messages, never alone in the alambique.

---

## D-001 — Name and command

- Status: decided
- Decided: 2026-11-02
- Origin: maintainer XEL; alambique section 1 and point 6.5 (command name), recorded as active keys
  `project.name` and `project.command` in `brewstillery/settings.toml`. The English-commands direction was
  first given by XEL on 2026-10-30 in the legacy TickBrew repository and carried into 6.5.
- Decision: the project is **Fabrewk**; its command is `fabrewk`. The provisional `tbh` is retired. Every
  command and flag Fabrewk develops is in English, unique and standard (UNIX paradigm), with no translated
  alias. Only the interaction between agent and human, and any GUI chosen for translation, is localized.
- Consequences: no message or instruction may name a command that does not exist (a lesson of the legacy
  pilots). A future rename must update instructions, examples and automation in order and decide expressly on
  temporary compatibility (6.5).
- Not decided by this entry, recorded as stated intent: Python package as the most viable proposal; GitHub
  organization `fabrewk` and domain `fabrewk.ai` in the future; the "ai" superscript in the logo is decided
  with the design and human-documentation delegate; the trademark is checked by a professional before any
  commercial use (alambique section 1).
- Sources: `brewstillery/xel/alambique.md` sections 1 and 6.5; `brewstillery/settings.toml`.
- Supersedes: none.

## D-002 — The `brewstillery/` domain

- Status: decided
- Decided: 2026-11-02
- Origin: maintainer XEL; alambique section 2 ("Decidido (2026-11-02)"), points 6.3, 6.6 and 6.7; active key
  `project.domain` in `brewstillery/settings.toml`.
- Decision: the same layout in Fabrewk's own repository and in every repository it serves:
  - `brewstillery/` is the only canonical directory for everything Fabrewk works on in a repository: its
    evidence, settings and support directories.
  - Outside it there are only the entry files that tools require at fixed root paths, declared and minimal
    (`AGENTS.md`, `CLAUDE.md`), pointing to `brewstillery/STATUS.md`. If a project already has its own
    `AGENTS.md`, Fabrewk adds the reference and does not replace the file.
  - In Fabrewk's own repository, Fabrewk's code is the product and lives at the root like any project's;
    `brewstillery/` is its development domain.
  - `brewstillery/STATUS.md` always exists and is the first thing any agent reads. During the bootstrap it
    says to read the maintainer's alambique first.
  - `brewstillery/<alias>/` is each maintainer's space: `alambique.md` (the only file swept officially after
    each phase, gate or cut), `settings.toml` (personal preferences) and whatever material the maintainer
    cites from the alambique.
  - The alambique holds free Markdown with no settings. The maintainer's input sits above one highly visible
    separator; agents answer only below it; if the separator is ambiguous, an agent stops without writing.
  - A personal settings file never grants authority over the project; project settings live apart
    (`brewstillery/settings.toml`). Active keys there are decisions; commented keys are proposals.
  - `brewstillery/data/`, or any other directory the design names, will hold framework objects; not created
    until the design decides it.
  - Aliases use lowercase letters, digits and hyphen, and may not take a reserved name of `brewstillery/`.
  - Publishing `brewstillery/` is each project's maintainers' decision; Fabrewk points out at installation that
    distributing it in Git keeps the meta-harness synchronized.
- Consequences: the reserved names must include every directory Fabrewk itself places in `brewstillery/`
  (see D-005). Directory separation organizes work; it is not a security boundary (6.6).
- Sources: `brewstillery/xel/alambique.md` sections 2, 6.3, 6.6, 6.7; `brewstillery/settings.toml`.
- Supersedes: none.

## D-003 — Pilot evaluation and the rule that a failure never ends Fabrewk (point 6.48)

- Status: decided
- Decided: 2026-11-02
- Origin: maintainer XEL; alambique point 6.48, both paragraphs, as written at this entry's date. XEL also
  directed in chat that this rule applies to every construction phase, not only to the pilot.
- Decision:
  - Before a project depends on Fabrewk, a full cycle of specification, implementation, cross-provider
    review, verification and human acceptance is demonstrated, and compared with the current workflow on a
    fixed task by quality, human interventions, rework, time and observable consumption. Budget and criteria
    are agreed beforehand. The pilot includes interruptions, conflicts and invalid evidence, to show that the
    system also refuses wrong advances. If maintainer roles exist, the pilot shows that editing personal
    preferences or opening a PR cannot raise permissions.
  - An insufficient result never cancels Fabrewk's development: it stops only that pilot's advance and
    produces a diagnosis of causes, proposed corrections and a new attempt with its own budget.
  - Defects found by a review are the expected result of reviewing, not a failure of the project: they are
    corrected and reviewed again until the work passes.
  - An exhausted budget pauses the work with its state saved and asks the maintainer for more budget or a
    smaller scope; it does not close it.
  - Each new attempt must show measurable progress over the previous one. After two consecutive attempts
    without progress, the agent presents alternatives (reduce scope, change the design, replace a component)
    for the maintainer to choose.
  - Abandoning or replacing Fabrewk can only be an express decision of the maintainer, separate from any
    pilot, never an automatic outcome.
- Consequences: no phase or pilot may end in an automatic "no-go" of the project. Acceptance criteria still
  cannot be changed by agents to declare success (6.25); a phase that has not passed is reported as not
  passed. The progress measure of each retry is fixed before the retry starts.
- Sources: `brewstillery/xel/alambique.md` 6.48; the legacy pilots' closures, read for context (R-001).
- Supersedes: none.

## D-004 — The first pilot

- Status: decided
- Decided: 2026-11-02
- Origin: maintainer XEL, in another session, stated in the chat of the first Fabrewk session; it develops
  XEL's answer to question 3 of alambique section 8 ("seguir intentando reconstruir TickBrew de cero, partiendo
  del draftingBoard.md original").
- Decision: the pilot is a first small, bounded module of TickBrew, rebuilt from TickBrew's original drafting
  board on Fabrewk's structure, in a new repository. The current TickBrew repository will be migrated to
  `tickbrew-legacy` in a separate process that XEL runs with another session; Fabrewk's integrator does not
  touch that repository. The integrator proposes which module and how it deliberately provokes a rejection
  (6.48).
- Consequences: the pilot's repository is a project Fabrewk serves, so it gets `brewstillery/` and never a
  `bootstrap/` directory (D-005). The module and the planned rejection are proposals until XEL decides them.
- Sources: chat of session 1; `brewstillery/xel/alambique.md` section 8, question 3; the original drafting board
  sections 1, 2, 4a and 4b (R-001).
- Supersedes: none.

## D-005 — `brewstillery/bootstrap/`, the provisional record

- Status: decided
- Decided: 2026-11-02
- Origin: maintainer XEL, in another session, stated in the chat of the first Fabrewk session.
- Decision: `brewstillery/bootstrap/` keeps the evidence of Fabrewk's self-construction while its single
  registry does not exist. `bootstrap` is a reserved name. The directory exists only in Fabrewk's own
  repository, never in the projects Fabrewk serves. When Fabrewk reaches production, its content is imported
  into the single registry as history and the directory becomes read-only, without being deleted.
- Consequences: `layout.reservedNames` in `brewstillery/settings.toml` must list `bootstrap`; at this entry's
  date it does not, and the maintainer has been asked to add it (the file is the maintainer's). The approved
  construction plan is stored here once approved.
- Sources: chat of session 1; `brewstillery/settings.toml`.
- Supersedes: none.

---

## R-001 — Provenance of session 1

- Status: recorded
- Date: 2026-11-02
- Who: integrator Claude Code, model `claude-opus-5-5`, effort high (observed `CLAUDE_EFFORT=high`,
  client-side, not attested by the server). No subagent, no dispatch, no network.
- Authority: maintainer XEL in chat; the legacy TickBrew gate rules do not apply to this session.
- Read, read-only:

| Source | Revision | What was read |
| --- | --- | --- |
| `brewstillery/xel/alambique.md` | sha256 `6fcc231e…0e96` before the answer was replaced | whole file |
| `brewstillery/xel/settings.toml` | sha256 `1d2f11f3…1f33` | whole file |
| `brewstillery/settings.toml` | sha256 `a51f29b9…5c34` | whole file |
| `/home/xel/Dev/TickBrew/tickbrew-harness` `README.md`, `docs/DESIGN.md` | `0000bdc`, clean tree | whole files |
| `/home/xel/Dev/TickBrew/tickbrew-harness` code | `0000bdc` | inventory only: file list and physical line counts (5,927 lines in 38 tracked files); no code read, no test run |
| `/home/xel/Dev/TickBrew/tickbrew` `docs/automation/receipts/FZ-2.md` | `3793219` (branch `gate/s2`, not changed) | lines 649–774 |
| `.../tickbrew` `docs/automation/receipts/FZ-2b.md` | `3793219` | lines 524–707 |
| `.../tickbrew` `docs/engineering/gates/FZ2B_DECISION_PACKET.md` | `3793219` | section 7 (lines 85–91) and the heading list |
| `.../tickbrew` `docs/development/XEL-DRAFTINGBOARD.md` | `3793219` | lines 1–151 (sections 1 and 2) and 305–336 (section 4, 4a, 4b) |
| `/home/xel/Dev/TickBrew/meta-harness/` `EVALUATION.md`, `comparison.csv`, `SOURCES.md`, `CONTEXT_PROYECTO_IA_GITHUB.md` | not a Git repository; files dated 2026-10-27 and 2026-10-29 | whole files |

- Checked without touching: `/home/xel/Dev/tmp` measures 1.8 GB (`du -sh`); the configuration backup
  directory named in alambique section 8 exists. Neither was opened.
- Not read: any `sources/`, XMF, credentials or private file; the rest of `meta-harness/`; any network source.
- Written: this file, `brewstillery/STATUS.md`, `AGENTS.md`, `CLAUDE.md`, the answers zone of
  `brewstillery/xel/alambique.md` (the part above the separator unchanged, checked by digest), and an empty Git
  repository on `main` with no remote.

---

## D-006 — Assess an existing repository before writing in it

- Status: decided
- Decided: 2026-11-02
- Origin: maintainer XEL; alambique section 8, answer 1 to the integrator's question 1 ("Sí, de acuerdo con tu
  recomendación").
- Decision: before Fabrewk writes anything in an existing repository (not even `brewstillery/`), it runs an
  assessment that only reads what is indispensable (Git state, root files, an existing `AGENTS.md`, the licence,
  any other harness present) and ends in a verdict with its reasons: adopt (Fabrewk as integrator), review only,
  or do not enter. The maintainer confirms; only then does Fabrewk write, and the verdict becomes the first entry
  of that repository's record.
- Consequences: the installation's handshake starts with this step; designed in the architecture phase, built
  in the installation phase.
- Sources: alambique section 1 and section 8; the integrator's answer, question 1.
- Supersedes: none.

## D-007 — Project settings hold the general permissions granted to agents

- Status: decided
- Decided: 2026-11-02
- Origin: maintainer XEL; alambique section 8, answer 2 to the integrator's question 2.
- Decision: `brewstillery/settings.toml` (project settings) is where the responsible maintainer defines the general
  permissions given to agents, including those that depend on what is configured on the maintainer's host (for
  example which MCP servers agents may use) and whether agents may modify their own tool settings when the
  maintainer asks. The full content of this level is left to the integrator's design.
- Consequences: the "general" level of 6.10, 6.11 and 6.15 is the project level. Proposals of the integrator for
  the architecture phase, not decided: (a) the file says what is permitted; what exists on each machine is
  detected at start (alambique section 8, third pending) and shown in the effective configuration, not stored as
  another configuration; the effective set is what is both permitted and present; (b) agents modify their own tool
  settings only on the maintainer's express request at that moment, recorded, and never to widen what they may
  read, run or send without a separate confirmation (6.13); (c) Fabrewk's built-in defaults sit below the project
  level. The file never holds credentials, tokens or addresses carrying secrets (6.4).
- Sources: alambique 6.10, 6.11, 6.13, 6.15 and section 8.
- Supersedes: none.

## D-008 — Technical files in English by default

- Status: decided
- Decided: 2026-11-02
- Origin: maintainer XEL; alambique section 8, answer 3 ("Recomiendo que por defecto así sea, cumple el criterio
  natural de todo es ingles en archivos técnicos. Este puede ser otro setting...") and in chat: "Si ayuda a
  optimizar coste de tokens, si."
- Decision: agent and technical files (`AGENTS.md`, `CLAUDE.md`, `STATUS.md`, this record, and the repository's
  canonical documents) are written in English by default; the language is a project setting.
- Consequences: the setting is `languages.canonicalDocumentationLanguage = "en"` in `brewstillery/settings.toml`,
  today a commented proposal; the maintainer activates it (the file is the maintainer's). Alambiques are not
  documentation: their answers follow each maintainer's `agentInteraction`. Final key names and the split by
  content class (6.20) are decided in the architecture phase. The token saving of English over Spanish is
  expected but not measured in this repository.
- Sources: alambique 6.20, 6.21 and section 8; `brewstillery/settings.toml`.
- Supersedes: none.

## D-009 — Pilot language and comparison

- Status: decided
- Decided: 2026-11-02
- Origin: maintainer XEL; alambique section 8, answers 4 and 5 to the integrator's questions 4 and 5.
- Decision: the pilot module is written in Python. The pilot includes the comparison with the current workflow
  (the same module done with today's method: Claude and Codex with the maintainer relaying), measured the same way.
- Consequences: the pilot's budget covers both arms. Which module is still the integrator's proposal (TickBrew's
  persistent configuration module) until the pilot specification phase. Rust for TickBrew's kernel stays a
  question of TickBrew's own architecture.
- Sources: alambique 6.48 and section 8; D-004.
- Supersedes: none.

## D-010 — Construction plan approved

- Status: decided
- Decided: 2026-11-02
- Origin: maintainer XEL in chat: "No veo objeciones a tu propuesta de fases."
- Decision: the phase plan proposed in session 1's chat is approved and stored in `brewstillery/bootstrap/plan.md`.
- Consequences: two amendments made after that approval, marked in the plan and pending the maintainer's
  confirmation: a phase for auditing the maintainer's alambique (requested in alambique section 8, answer 4), and the
  review of the bootstrap merged with the review of that audit into one Codex dispatch, to save cost.
- Sources: chat of session 1; `brewstillery/bootstrap/plan.md`.
- Supersedes: none.

---

## R-002 — Session 1, second sweep: what left the alambique

- Status: recorded
- Date: 2026-11-02
- Who: integrator Claude Code, `claude-opus-5-5`, effort high as the maintainer states it. No subagent, no dispatch,
  no network.
- Input change: the maintainer edited section 8 of the input (the part above the separator): the answers to the
  previous agent's three questions were replaced by answers to the integrator's five questions. The separator
  moved from line 243 to line 242 and was found exactly once. `brewstillery/settings.toml` changed:
  `layout.reservedNames` now includes `"bootstrap"` (sha256 `4362eb84…d41b`, parses as TOML).
- Why this entry: the repository had no commit yet, so the removed text and the agent answer replaced in the first
  sweep existed nowhere else. Both are kept verbatim below (alambique 6.29: nothing that informed a decision lives
  only in a chat).

The maintainer's removed text, verbatim (it was at the end of input section 8):

````markdown
Sobre las preguntas que dejó el agente previo a tu lectura de este alambique:

1. ¿Qué del código de `tickbrew-harness` se reutiliza, se reescribe o se descarta?
Lo dejo a consideración de tu propia evaluación en función a lo que analices de este alambique y su evidencia declarada.  El objetivo es un producto final de calidad y limpio, no parcheado.  Si en revisiones hay errores, es revisarlos, corregirlos y volver a implementar revisiones hasta que el código funcione.  No debe haber no-go's futuros.
2. Si un proyecto no publica `brewstillery/`, ¿dónde vive su registro para no quedar en una sola máquina? (Una opción: `brewstillery/` como repositorio git propio, anidado e ignorado por el del producto.)
La opción que das debe ser la última salida, esto debe ser un WARNING message para el mantenedor, bajo su propio riesgo de no publicar la trazabilidad de un meta-harness, las consecuencias que esto lleva bajo su propio riesgo y responsiva.
3. ¿Cuál es el primer piloto de Fabrewk, y qué tarea provoca a propósito un rechazo?
Debemos seguir intentando reconstruir TickBrew de cero, partiendo del draftingBoard.md original pero pidiendo su acoplamiento a la nueva estructura/framework que termine construyendo Fabrewk como producción.
````

The previous agent's answer, replaced in the first sweep on the maintainer's instruction, verbatim:

````markdown
## Respuesta del agente — 2026-11-02

Claude Code, `claude-opus-5-5`, esfuerzo high (de `~/.claude/settings.json`). Escritura única de este archivo, entrada incluida, por autorización expresa de XEL en la sesión de organización de `~/Dev`; desde aquí la entrada es solo suya.

### Procedencia

| Fuente | Revisión | Qué se tomó |
| --- | --- | --- |
| `/home/xel/Dev/TickBrew/tickbrew/docs/development/XEL-DRAFTINGBOARD.md` | `3793219` | Líneas 152–179 (sección 3 y 3aa) → secciones 4; 180–195 (3ab) → sección 5; 197–303 (3ac, 48 puntos) → sección 6; 337–380 (4c a 4f) → sección 7 |
| `.../tickbrew/docs/automation/receipts/FZ-2.md` | `3793219` | Líneas 649–774: decisiones de C8, revisión 7 y cierre (no-go) |
| `.../tickbrew/docs/automation/receipts/FZ-2b.md` | `3793219` | Líneas 524–707: C4, dirección y decisión de C5, cierre (no-go, opción A) |
| `.../tickbrew/docs/engineering/gates/FZ2B_DECISION_PACKET.md` | `3793219` | Sección 7, las opciones |
| `/home/xel/Dev/TickBrew/tickbrew-harness` `README.md`, `docs/DESIGN.md` | `0000bdc` | README completo; secciones 1, 3, 14 y 15 |
| Sesión de organización de `~/Dev` | 2026-11-02 | Nombre, estructura `brewstillery/`, pendientes de máquina, vecinos de nicho |

No se leyó `sources/`, XMF ni nada privado. Las secciones de producto del tablero original (1, 2, 4a y 4b) quedan en TickBrew y no se trajeron.

### Cambios respecto al original

Tu texto se conservó; solo cambió donde hoy hay una decisión tuya:

- Párrafo de nombres de 3ac → sección 1, con el nombre ya decidido.
- "actualmente TickBrew" → "por ejemplo TickBrew"; "meta-harness" → "Fabrewk" donde nombraba a la herramienta.
- `.meta-harness/` o `tools/<nombre>/` → `brewstillery/`; `brewstillery/<alias>/draftingBoard.md` → `brewstillery/<alias>/alambique.md`; `attachments/` ya no es obligatorio; archivo de preferencias → `settings.toml`.
- Punto del comando: `tbh` retirado, `fabrewk` decidido, y la dirección de FZ-2b C5-A (comandos y flags en inglés) incorporada.
- Agregados mínimos, marcados aquí para que los revises: que `STATUS.md` sea una vista del estado único; que el registro no quede en una sola máquina aunque no se publique `brewstillery/`; "nada que haya informado una decisión debe vivir solo en un chat"; que la continuidad empiece por `STATUS.md`; el piloto ya no se refiere solo a reconstruir TickBrew.
- Nuevas: sección 2 (estructura), sección 3 (referencias a lo construido) y sección 8 (pendientes de máquina).

### Lo que dejan FZ-2 y FZ-2b (lectura del integrador)

1. **Dos compuertas sin una revisión limpia.** Cada revisión independiente de Codex encontró defectos bloqueantes que fallaban abiertos: cuatro en FZ-2 (revisión 7) y cuatro en FZ-2b (revisión de C5). El número no bajó, y el paquete de FZ-2b descartó una tercera compuerta por ser el "trabajo infinito de harness" que FZ-1 había nombrado. Para Fabrewk: un núcleo más pequeño y un piloto que provoque a propósito el rechazo, en vez de esperar que ocurra.
2. **El ciclo de rechazo nunca corrió en vivo.** Las dos tareas reales (T-1 y T-2) se aceptaron a la primera; el camino de cambios solicitados solo se probó con agentes simulados. El presupuesto de despachos se agotó antes.
3. **Fallas de uso que pagaste tú.** Escribiste literalmente el texto de ejemplo `--say "tus palabras"`; los mensajes nombraban un comando `tbh` que no existía; las instrucciones suponían bash y tu shell es fish; el integrador inventó alias en español. En FZ-2, las intervenciones fueron 4 contra 2 previstas y el tiempo activo 683 s contra 178 s. Para Fabrewk: la comunicación y el comando son parte del núcleo, no un acabado.
4. **El núcleo creció.** De 1,960 líneas en FZ-2 a 3,089 en FZ-2b (límite 3,500), más 905 de adaptadores, para un ciclo de una tarea, un mantenedor y una máquina.
5. **Lo que sí funcionó y vale estudiar:** el registro de eventos como autoridad, los veredictos ligados al digest del trabajo exacto, el aislamiento con `bwrap`, los rechazos codificados y 198 pruebas en un clon limpio. El código de `0000bdc` no tiene revisión independiente; reutilizarlo es una decisión tuya, no un punto de partida dado.

### Para decidir en este alambique

1. ¿Qué del código de `tickbrew-harness` se reutiliza, se reescribe o se descarta?
2. Si un proyecto no publica `brewstillery/`, ¿dónde vive su registro para no quedar en una sola máquina? (Una opción: `brewstillery/` como repositorio git propio, anidado e ignorado por el del producto.)
3. ¿Cuál es el primer piloto de Fabrewk, y qué tarea provoca a propósito un rechazo?
````

---

## D-011 — Format of this record confirmed

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in chat: "Mientras te sea mas eficiente el flujo de este archivo con tu plan de
  desarrollo y un humano pueda entenderlo a simple vista, estoy de acuerdo."
- Decision: this record keeps its name (`brewstillery/bootstrap/decisions.md`) and format (append-only Markdown,
  fixed fields per entry), under the condition that it stays readable at a glance by a person.
- Consequences: readability is a criterion of this file; Codex's review (plan phase 2) checks it.
- Sources: chat of session 1.
- Supersedes: none.

## D-012 — Plan amendments confirmed; where the audit's proposal is delivered

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in chat: "Si estas conforme con que esto sea así, no le veo objeción, todo vive ordenado
  en su propio dominio y es claro para tu propio flujo de trabajo."
- Decision: the two amendments of D-010 are confirmed (phase 1, the audit of the maintainer's alambique; phase 2,
  one merged review). The audit's proposed text is delivered as a separate file inside `brewstillery/bootstrap/`;
  the maintainer reads it and copies what they accept into their input.
- Consequences: no agent writes above the alambique's separator; the file name is fixed when phase 1 starts.
- Sources: chat of session 1; `brewstillery/bootstrap/plan.md`.
- Supersedes: none.

## D-013 — Phase 0 (bootstrap) closed

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in chat, verbatim: "Si consideras que con esto ya no quedan pendientes ni controversias
  abiertas en este corte, podemos declararlo cerrado para poder atender la migración de tickbrew-legacy antes de
  continuar con nuestra siguiete etapa." The integrator's assessment: nothing open blocks; every open item below
  has a destination.
- Decision: phase 0 of the plan is closed with the initial commit. Next, outside Fabrewk, the maintainer migrates
  the current TickBrew repository to `tickbrew-legacy` in another session; then phase 1.
- Carried, not blocking, with destinations: the conflict between alambique 6.27 and the non-publication warning
  (phase 1, audit); the vocabulary and paths inherited from TickBrew (phase 1); the pilot module (phase 11); the
  comment on line 21 of `brewstillery/settings.toml` still calls the now-active language key a proposal (the
  maintainer's file; the maintainer's choice). For the legacy migration: phases 1 and 3 read `tickbrew-harness` at
  `0000bdc`, the two pilots' receipts, the decision packet and the original drafting board, so they should stay
  readable after the move; alambique section 3 will then need their new paths.
- Sources: chat of session 1; `brewstillery/bootstrap/plan.md`.
- Supersedes: none.

## R-003 — Session 1, closing

- Status: recorded
- Date: 2026-11-03
- Who: integrator Claude Code, `claude-opus-5-5`, effort high as the maintainer states it. No subagent, no dispatch,
  no network.
- Checked: `brewstillery/settings.toml` now has `languages.canonicalDocumentationLanguage = "en"` active and parses
  (sha256 `437fa785…a652`); `brewstillery/xel/settings.toml` unchanged (`1d2f11f3…1f33`); the alambique's input
  unchanged since the second sweep (lines 1–245, sha256 `014e17b6…cdf6`); separator found once.
- Commit identity: the repository's Git configuration (`xel-arjona`), with the integrator as co-author.

---

## D-014 — Documentation publication languages

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL, by activating `languages.documentationLanguages = ["en", "es-MX"]` in
  `brewstillery/settings.toml` (active keys are decisions); the comment that called the language keys a proposal
  was removed in the same edit.
- Decision: the repository's documentation is published in English (canonical, D-008) and Mexican Spanish.
- Consequences: translation starts only when an original is stable and approved for it, and every translation is
  tied to the revision it represents (alambique 6.21, 6.22); nothing is translated during the bootstrap. Alambique
  answers are not documentation and are not affected. Final key names stay a question for the architecture phase.
- Sources: `brewstillery/settings.toml` (sha256 `05e9ba53…1397`); alambique 6.20–6.22.
- Supersedes: none.

## R-004 — TickBrew legacy relocation

- Status: recorded
- Date: 2026-11-03
- Who: integrator Claude Code, `claude-opus-5-5`, effort high. Read-only Git commands on the legacy repository;
  nothing written there.
- Change: the TickBrew repository read in R-001 moved from `/home/xel/Dev/TickBrew/tickbrew` (that path no longer
  exists, checked) to `/home/xel/Dev/TickBrew/tickbrew-legacy`, frozen and read-only. Annotated tag `legacy-final`
  (tag object `352e746`) points to `8a9e1bfa7b30ec0fe314d456b37702200834d252`, equal to `main` and
  `origin/main` (checked); clean tree.
- Mapping for earlier entries: every `/home/xel/Dev/TickBrew/tickbrew` path in R-001 and in the verbatim text kept
  in R-002 now reads `/home/xel/Dev/TickBrew/tickbrew-legacy`. The four files read in R-001
  (`docs/automation/receipts/FZ-2.md`, `docs/automation/receipts/FZ-2b.md`,
  `docs/engineering/gates/FZ2B_DECISION_PACKET.md`, `docs/development/XEL-DRAFTINGBOARD.md`) have the same blob at
  `3793219` and at `legacy-final` (checked), so the line ranges recorded there still hold. Those entries are not
  rewritten (append-only).
- Unchanged locations: `/home/xel/Dev/TickBrew/tickbrew-harness` still at `0000bdc`; `/home/xel/Dev/TickBrew/meta-harness/`
  present.
- Input change: the maintainer updated alambique section 3 to the legacy path (the item carried in D-013 is done) and
  activated `documentationLanguages` (D-014). Separator found once; input lines 1–245 sha256 `fe362322…4449`.

---

## D-015 — Settings: what is permitted apart from what is present, as a starting idea

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL; a line added to their answer 2 in alambique section 8 (input line 236): "Para el tema que
  mencionas sobre separar lo que se pemite de lo que existe estoy de acuerdo en tus propuestas, y esto debe
  considerarse como idea de inicio, el diseño final después de que evalues como resuelven este tipo de cosas otros
  proyectos definrá nuestra mejor opción a este tipo de controversias."
- Decision: the integrator's proposals (a) to (c) in D-007's consequences are the starting idea for the settings
  levels. The final design is chosen after studying how other projects solve it.
- Consequences: plan phase 4 (study of references) covers how the projects of alambique section 7b separate permitted
  capabilities from those present on a machine; plan phase 5 decides. Develops D-007 without replacing it.
- Sources: `brewstillery/xel/alambique.md` section 8; D-007.
- Supersedes: none.

## D-016 — Phase 1 delivery: file and languages

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in session 2's chat: "1. De acuerdo. 2. Acepto tu recomendación: escribir el texto propuesto
  en español de México y los hallazgos y la tabla en inglés."
- Decision: the audit of the maintainer's alambique is delivered in `brewstillery/bootstrap/alambique-audit.md`. Its
  findings and traceability table are in English (D-008). Its proposed text is in Mexican Spanish, because it is a
  draft of the maintainer's own input, not documentation.
- Consequences: completes D-012 (the file name was to be fixed when phase 1 started).
- Sources: chat of session 2; D-008; D-012.
- Supersedes: none.

## R-005 — Session 2: start checks and phase 1 delivery

- Status: recorded
- Date: 2026-11-03
- Who: integrator Claude Code, `claude-opus-5-5`, effort high as the maintainer states it. No subagent, no dispatch,
  no network. Started in a new chat with no handover prompt; the state was rebuilt from `brewstillery/STATUS.md` and
  the files it names.
- Start checks: tree clean at `7aeec3d`, no remote; alambique input lines 1–245 sha256 `fe362322…4449` (same as
  R-004), separator found once; `brewstillery/settings.toml` `05e9ba53…1397` and `brewstillery/xel/settings.toml`
  `1d2f11f3…1f33`, both unchanged and parsing as TOML; `tickbrew-legacy` at `legacy-final`, clean; `tickbrew-harness`
  at `0000bdc`, clean. Present on the host: `git`, `codex` 0.166.0, `python3`, `uv`, `rg`, `bwrap`, `gh`, `fish`; the
  `sqlite3` command-line tool is absent. `~/Dev/tmp` still exists (not opened).
- Input change: the maintainer added one line to answer 2 of section 8 (D-015). The separator moved to lines
  242–245 and was found once; input lines 1–241 sha256 `8b52f897…b255`.
- Read: the whole input; the answers zone, lines 247–385 and 386–412; `plan.md`, this file, both settings files and
  `STATUS.md`. Nothing outside this repository.
- Written: `brewstillery/bootstrap/alambique-audit.md` (its Part 2 generated by a script that copies every untouched
  input line verbatim and asserts each replaced passage; the script lives in the session's scratch directory, not
  in the repository); this entry and D-015, D-016; `brewstillery/STATUS.md`; the answers zone of the alambique. The
  input is unchanged by the integrator (digest above).

## D-017 — The alambique's answers zone holds only current recommendations

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in session 2's chat: "propongo que el agente me dé recomendaciones de que irle editando al
  mismo alambique en mi propia sección con mis palabras para que este vaya quedando mas claro con cada barrido […]
  al barrerlo, la respuesta siempre sea fresca con recomendaciones nuevas sin dejar allí un texto siempre creciente.
  La evidencia del alambique siempre debe ser la mía, tu sección es para recomendarme como modificarla y dejarla mas
  clara."
- Decision: every sweep replaces the agent's whole answer. The answer holds only current recommendations for editing
  the maintainer's input (which point, what to clarify and how, its destination), which the maintainer adopts in
  their own words. Nothing settled accumulates there: its history lives in Git and in this record. Agent work that is
  not a recommendation for the input is delivered as a phase file in `brewstillery/bootstrap/` during the bootstrap;
  questions that are not about the input go to chat.
- Consequences: the alambique rule in `AGENTS.md` is updated. The audit (`alambique-audit.md`, finding F-23) proposes
  that the maintainer state this in the input's introduction and in 6.8. The earlier answers are kept as described in
  R-006.
- Sources: chat of session 2; alambique 6.7, 6.8, 6.23.
- Supersedes: none.

## R-006 — Session 2: answers zone replaced

- Status: recorded
- Date: 2026-11-03
- Who: integrator Claude Code, `claude-opus-5-5`, effort high as the maintainer states it.
- Change: the answers zone of `brewstillery/xel/alambique.md` was replaced by recommendations only (D-017). The input
  is unchanged by the integrator (lines 1–241 sha256 `8b52f897…b255`); separator found once. `AGENTS.md` updated
  (alambique rule). The audit gained finding F-23 (input line 9 and 6.8), and its Part 2 was regenerated.
- Where the replaced answer lives: `git show 7aeec3d:brewstillery/xel/alambique.md`, the part after the separator.
  An addition written earlier in session 2 and never committed was dropped: its five open questions are in the new
  answer, and its proposal for splitting phases is in `STATUS.md`. Where each earlier evaluation is resumed:

| Content of the replaced answer | Resumed in |
| --- | --- |
| Reading of section 1 (modes, installation, Python, licence options) | plan phase 5; the licence also with a professional |
| Classification of section 6 (indispensable, later stage, alternative) | plan phase 6 (every indispensable point maps to a requirement) |
| Section 8 pending items | the audit (F-11); machine tasks stay with the maintainer |
| Reuse, rewrite or discard for `tickbrew-harness` (provisional) | plan phase 3 |
| Warning text for not publishing `brewstillery/` | plan phase 10 |
| Pilot module and the six provoked refusals | plan phase 11 |
| Gaps in `bootstrap/`, the pilot and 6.48 | plan phases 11 and 13; D-003 |
| Answers to the five questions | D-006 to D-009, D-015; the audit (F-10) |
| Preliminary audit | the audit (`alambique-audit.md`) |

---

## D-018 — Audited alambique input adopted; phase 1 closed

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL, by replacing their input with the audit's proposed text (commit `e0017cb`) and writing in
  section 8: "todas tus recomendaciones hechas en tu auditoría de alambique fueron copiadas y aceptadas de mi parte";
  in session 2's chat they asked to close the session if nothing was left open.
- Decision: the five findings that needed the maintainer's answer are settled as proposed: reviewer mode leaves its
  evidence in `brewstillery/` and changes neither code nor the project's `AGENTS.md` (F-16); `SOURCES.md` is listed in
  section 3 (F-20); section 6 is grouped by theme and its numbering is stable (F-15); 6.27 keeps the goal of a record
  that does not live on one machine and describes the warning when a project does not publish `brewstillery/`, with
  the decision recorded in the maintainers' own words (F-09); 6.44 reads "las reglas de Git que fije cada proyecto"
  (F-02). The maintainer added a usage note at the top of the input and made section 8 their channel for additions,
  removals and open items. Phase 1 is closed; its fidelity review by Codex is part of phase 2 (D-012).
- Consequences: D-007's statement that the "general" level of 6.15 is the project level no longer holds: 6.15 now
  says "perfiles reutilizables entre proyectos" (F-12); for 6.10 and 6.11 D-007 stands. Phase 2 compares the input at
  `7aeec3d` with the input at `e0017cb`, using the audit's traceability table.
- Sources: `brewstillery/xel/alambique.md`; `brewstillery/bootstrap/alambique-audit.md`; chat of session 2.
- Supersedes: D-007, only its consequence about 6.15.

## D-019 — Splitting a phase into parts

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in session 2's chat, answering the integrator's proposal: "1. La acepto".
- Decision: one level of division only. A phase is split into parts only when it does not fit one session, where its
  inputs separate. Each part has its own inputs and checkable criterion, ends with a commit and `STATUS.md` updated,
  and runs in its own chat. Codex reviews each phase that delivers documents and each part that delivers code.
- Consequences: added to `brewstillery/bootstrap/plan.md`. Expected splits: phases 3, 4, 5, 7 and 9. The name "part"
  belongs to this plan only; Fabrewk's own grouping of work is decided in phase 5 (alambique 6, introduction).
- Sources: chat of session 2; the maintainer's premise there that long chats lost memory efficiency in TickBrew.
- Supersedes: none.

## R-007 — Session 2, closing sweep

- Status: recorded
- Date: 2026-11-03
- Who: integrator Claude Code, `claude-opus-5-5`, effort high as the maintainer states it. No subagent, no dispatch,
  no network.
- Sweep: the new input (lines 1–277, sha256 `389dbea7…7314`; separator lines 278–281, found once) differs from the
  audit's Part 2 only by the maintainer's three changes listed in D-018 (checked by diff).
- Commits: the maintainer's edits are committed alone with no AI co-author: `e0d14e2` (input line 236, D-015) and
  `e0017cb` (the adopted input). Between them, `ce6d0b9` holds the integrator's session 2 work. Both maintainer
  versions were rebuilt exactly: the first matches the digest recorded in R-005. The first commit was created
  without its subject line through a shell quoting error and was amended before any other commit; nothing was
  pushed.
- Written in this sweep: the answers zone (recommendations for the usage note and section 8), `AGENTS.md` (section 8
  as the maintainer's channel), `plan.md` (D-019; phase 1 closed), `STATUS.md`, and this record.

## D-020 — The handover names the next session's model and effort

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in session 2's chat: "Debes recordarme en tus instrucciones de 'onboard' para el siguente
  chat, el modelo y esfuerzo con el que debo iniciarlo."
- Decision: when a session closes, the integrator gives the maintainer the start prompt and the model and effort
  for the next chat, taken from the plan's row for that phase or part, and writes them in `STATUS.md`.
- Consequences: rule added to `AGENTS.md`. For phase 2: Claude Code on Sonnet 5.5, effort medium (the maintainer's
  global setting defaults Sonnet to low); Codex GPT-6.1 Sol, effort medium, availability to be checked.
- Sources: chat of session 2; `brewstillery/bootstrap/plan.md`, phase 2.
- Supersedes: none.

## R-008 — Session 3, phase 2 review by Codex

- Status: recorded
- Date: 2026-11-03
- Who: integrator Claude Code, `claude-sonnet-5-5`. Reviewer: Codex `gpt-6.1-sol`, effort medium, read-only sandbox,
  launched inside this repository, one pass, with the maintainer's permission and budget given in chat. Codex
  version 0.166.0; the model was available.
- Read by Codex: STATUS.md, plan.md, decisions.md, AGENTS.md, both settings files, the audit, the input at
  `7aeec3d` and at `e0d14e2`/`e0017cb`. It wrote nothing.
- Findings and dispositions:
  - P2-01 (blocking): 6.38 narrows "analizar y decidir" to "analiza y propone"; the audit's table says only "moved".
    Confirmed by the integrator. Disposition: question to the maintainer in the answers zone; open.
  - P2-02: AGENTS.md calls all of `bootstrap/` append-only, but `plan.md` and the audit are maintained. Open: needs
    the maintainer's wording (decisions.md entries and R/D records are append-only; plan and audit are not).
  - P2-03: the answers zone carried identity, summary and stale recommendations. Fixed: it now holds only the open
    recommendation.
  - P2-04: the introduction of this file (line 17) still calls English a proposal and the key commented out. Not
    edited, the record is append-only. Current rule: English is decided (D-008, D-014) and the key is active.
  - P2-05: `STATUS.md` lacked the start prompt. Fixed in `STATUS.md`.
  - P2-06: D-013 uses "Carried, not blocking, with destinations" instead of `Consequences`. Treat it as its
    `Consequences` for the phase 13 importer.
- Also confirmed by Codex: all 48 points of the input remain; the audit's input is the one at `e0d14e2`; the
  differences to `e0017cb` are the maintainer's three edits; both settings files parse.
- Next: the maintainer answers P2-01 and P2-02; the integrator applies and Codex's re-check is not needed unless
  the maintainer asks.

## D-021 — Phase 2 closed; append-only scope clarified

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in session 3's chat: acknowledged P2-03, P2-04, P2-05 and P2-06 (record internals are the
  integrator's), asked for P2-02 in plain terms, and answered P2-01 in section 8 of the input.
- Decision: 6.38 stays "analiza y propone": the integrator validates the tools a project incorporates and the
  maintainer keeps the decision (P2-01 closed, no edit to the input). P2-02 is settled by the integrator as an
  internal: `AGENTS.md` now says that only the entries of the record are append-only, and that `plan.md` and the
  audit are maintained by the integrator. P2-04 and P2-06 stay as recorded in R-008. Phase 2 is closed.
- Consequences: next is phase 3 (`plan.md`). The audit's traceability row for 6.38 keeps "moved"; the answer above
  is the disposition of the narrowing.
- Sources: R-008; the maintainer's section 8, item 8.1.2 (6.38); chat of session 3.
- Supersedes: none.

## D-022 — Sessions close with a clean tree (bootstrap only)

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in session 3's chat: "anotalo para las sesiones siguientes durante nuestro bootstrap, hasta
  que Fabrewk adopte sus propias reglas de diseño."
- Decision: when a session closes, the integrator makes the commits needed to leave no pending changes; the
  maintainer's edits are committed alone, with no AI co-author. It applies during the bootstrap, until Fabrewk's own
  design sets its workflow.
- Consequences: rule added to `AGENTS.md`; phase 5 decides the permanent version.
- Sources: chat of session 3; commits `0d704bd` and `dc66a61`.
- Supersedes: none.

---

## D-023 — Phase 3 split into parts 3a and 3b; permissions for phase 3

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in session 4's chat. The start message: "Te autorizo permiso de lectura sobre todo el
  repositorio de tickbrew-harness." Then the answers to the integrator's three questions: "Dividir en 3a y 3b
  (Recomendado)"; "Sí, en un clon desechable"; "Sí, los recibos y el paquete".
- Decision: phase 3 is split where its inputs separate (D-019). Part 3a gives each file of `tickbrew-harness` a verdict
  (reuse as design, rewrite, discard), in `brewstillery/bootstrap/harness-verdicts.md`. Part 3b turns the defects found
  by past reviews into test cases, in its own chat. Codex reviews both parts together when phase 3 ends. The harness's
  self-tests may run once in a disposable clone. For 3b the integrator may read, read-only, at `legacy-final`:
  `docs/automation/receipts/FZ-2.md`, `docs/automation/receipts/FZ-2b.md` and
  `docs/engineering/gates/FZ2B_DECISION_PACKET.md` of `/home/xel/Dev/TickBrew/tickbrew-legacy`.
- Consequences: `plan.md` records the two parts. Part 3b's chat confirms these permissions with the maintainer at
  its start, since authority is given in chat. Codex's review needs the maintainer's permission and budget, as in
  phase 2.
- Sources: chat of session 4; `brewstillery/bootstrap/plan.md`, phase 3 and the splitting rule.
- Supersedes: none.

## R-009 — Session 4, part 3a

- Status: recorded
- Date: 2026-11-03
- Who: integrator Claude Code, `claude-opus-5-5`, effort high as the maintainer states it. No subagent, no dispatch,
  no network.
- Start checks: tree clean at `43f188c`; alambique input lines 1–274 sha256 `1ba1bf6d…9e5b`, separator lines 275–278,
  found once (the maintainer's commit `0d704bd` removed the settled 6.38 paragraph and the duplicate 8.1.2);
  `brewstillery/settings.toml` `05e9ba53…1397` and `brewstillery/xel/settings.toml` `1d2f11f3…1f33`, unchanged.
- Read, read-only: every tracked file of `/home/xel/Dev/TickBrew/tickbrew-harness` at `0000bdc` (branch `gate/fz-2b`,
  clean tree), whole; the subjects of its 21 commits and the three latest messages whole. Correction to R-001: the
  repository tracks 40 files, not 38 (the two empty `__init__.py` were not counted); the 5,927 lines stand. In
  `tickbrew-legacy`, only `git describe` and `git status` (at `legacy-final`, clean); no file opened.
- Run, in a disposable clone of `0000bdc` in the session's scratch directory (D-023): the self-tests
  (`python3 -I -m unittest discover -s tests -t .`), 198 passed in 63 s; `tools/size_count.py --check`, core 3,089 of
  3,500, adapters 905 of 1,000; three probes written by the integrator, confirming H3A-01 to H3A-03 of
  `harness-verdicts.md`. Python 3.14.8, Git 2.56.0, bubblewrap 0.12.0. The clone was deleted; nothing was written in
  the original repository.
- Written: `brewstillery/bootstrap/harness-verdicts.md`; this entry and D-023; `plan.md` (phase 3 parts);
  `brewstillery/STATUS.md`; the answers zone of the alambique (no recommendation this sweep). The input is unchanged
  by the integrator (digest above).

## D-024 — Permissions for part 3b and Codex's review of phase 3

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in session 5's chat, the start message: "Te autorizo permisos de lectura para: tickbrew-harness
  y de tickbrew-legacy." and "Tienes plena autorización de ejecutar Codex (GPT-6.1 Sol High) para revisa la fase 3
  completa."
- Decision: D-023's read permission is confirmed for part 3b. The grant names `tickbrew-legacy` without a file list;
  the integrator used it as D-023 states it, the three named files only, and gave Codex the same limit. Codex
  (`gpt-6.1-sol`, effort high) reviews parts 3a and 3b together. "Plena autorización" is read as a full review and
  scoped re-checks of the corrections until no confirmed blocking finding is open, within the phase-rule of D-003.
- Consequences: three Codex passes in session 5 (R-010). Any wider reading of `tickbrew-legacy` needs the maintainer's
  words naming it.
- Sources: chat of session 5; D-023; `brewstillery/bootstrap/plan.md`, phase 3.
- Supersedes: none.

## R-010 — Session 5, part 3b and Codex's review of phase 3

- Status: recorded
- Date: 2026-11-03
- Who: integrator Claude Code, `claude-opus-5-5`, effort high as the maintainer states it. Reviewer: Codex
  `codex-cli 0.166.0`, `gpt-6.1-sol`, effort high, read-only sandbox, launched in this repository with
  `codex exec --json`; the stream names no model. No subagent, no network for the integrator.
- Start checks: tree clean at `e319f67`; alambique input lines 1–274 sha256 `1ba1bf6d…9e5b`, separator lines 275–278,
  found once; `brewstillery/settings.toml` `05e9ba53…1397` and `brewstillery/xel/settings.toml` `1d2f11f3…1f33`,
  unchanged. `tickbrew-harness` at `0000bdc` (branch `gate/fz-2b`, clean); `tickbrew-legacy` at `legacy-final`, clean.
- Read, read-only: in `tickbrew-legacy`, whole, `docs/automation/receipts/FZ-2.md` (774 lines),
  `docs/automation/receipts/FZ-2b.md` (707) and `docs/engineering/gates/FZ2B_DECISION_PACKET.md` (103). In
  `tickbrew-harness`: the finding keys across its files (by search), the message of `c569e29` (FX-1 to FX-8), the
  subjects of its commits, and the test classes, names and bodies that hold each finding's variants
  (`tests/test_review_fixes.py`, `test_hardening.py`, `test_disposition.py`, `adapters/test_adapters.py`,
  `test_machine.py`, `test_state.py`), and lines of `orchestrate.py`, `gitops.py`, `config.py`, `machine.py`,
  `adapters/agents.py` and `docs/DESIGN.md` to check the review's findings. Nothing was run in either repository.
- Written: `brewstillery/bootstrap/past-defect-cases.md` (commit `2e6e9cf`), checked by script: every finding key in
  the three legacy files and the harness appears in it.
- Codex, pass 1, on `2e6e9cf`: 7 min 15 s; 41 commands, none failed; input 4,389,128 tokens (4,162,304 cached), output
  17,127 (9,986 reasoning); stderr only a timed-out model-list refresh. Verdict reject: five blocking findings, seven
  minor. Every one checked by the integrator against the sources and confirmed:
  - P3-01 (blocking): the moved-base corner was excluded though confirmed. Added as PD-44.
  - P3-02 (blocking): the candidate's missing co-author line had no case. Added as PD-45.
  - P3-03 (blocking): C7C-9 and the commit made with a red test were excluded as conduct. C7C-9 joined PD-43; PD-46
    added. C7C-10 stays excluded (ignored files only).
  - P3-04 (blocking): PD-29 forbade agents' validated outputs in the log. Narrowed to raw diagnostic channels.
  - P3-05 (blocking): `harness-verdicts.md` said tool, provider and model were one name; the model and effort are
    separate fields. Corrected in the `agents.py` row and the observation.
  - P3-06 to P3-11 (minor): H3A-02 also halts as `NO_CHANGE` when the mode change is alone (verdicts and PD-20); the
    reducer's refusal relies on its copy, not on refusing before any change; `gitops.py` parses porcelain output on
    newlines in two places; the count's unit; PD-41 restated with the maintainer's later confirmation; PD-19 and PD-21
    made consistent. All corrected.
  - P3-12 (minor): this entry did not exist yet; settled by writing it.
  Corrections in commit `e48c22d`. `harness-verdicts.md` is a phase deliverable, not a record entry, so it was
  corrected in place.
- Codex, pass 2, scoped to `2e6e9cf..e48c22d`: 2 min 2 s; 13 commands; input 338,540 (278,144 cached), output 5,340.
  P3-01 to P3-11 closed. New: P3-13 (blocking, PD-44 allowed a stop PD-14 forbids) and P3-14 (minor, section K's
  attribution of C7C-9). Both confirmed and corrected in `d5cac90`.
- Codex, pass 3, scoped to `e48c22d..d5cac90`: 10 commands; input 160,454 (122,496 cached), output 1,352. P3-13 and
  P3-14 closed, no new finding. **Verdict accept.**
- Progress measure (D-003): confirmed blocking findings 5, then 1, then 0. The prompts, streams and last messages of
  the three passes stay in the session's scratch directory, not committed.
- Result: phase 3's acceptance holds as reviewed: each of the 40 files has a verdict and a reason, and each confirmed
  past defect has a test case (46 cases, PD-26 with 20 sub-cases). The input is unchanged by the integrator (digest
  above); the answers zone has no recommendation this sweep.

## D-025 — Phase 3 closed

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in session 5's chat: "Dado que ya no quedan pendientes ni controversias por definir en este
  chat, cierralo."
- Decision: phase 3 is closed on the evidence of R-009 and R-010: every file of `tickbrew-harness` has a verdict and a
  reason, every confirmed past defect has a test case, and Codex's review accepted both parts with no confirmed blocking
  finding open. Session 5 closes with a clean tree (D-022).
- Consequences: next is phase 4, the study of references, in a fresh chat (Opus 5.5, effort high; Codex, effort high,
  reviews it). Its start asks the maintainer for network permission, the list of sources and the budget. Phase 5 takes
  `harness-verdicts.md` (observations for phase 5) and phase 6 takes `past-defect-cases.md` into its test plan.
- Sources: chat of session 5; R-009; R-010; `brewstillery/bootstrap/plan.md`, phases 3 to 6.
- Supersedes: none.

---

## D-026 — Phase 4 split into parts 4a and 4b; permissions for phase 4

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in session 6's chat, the start message: "Tienes autorizado permiso de red y la lista de
  fuentes todo dentro de presupuesto. Si la fase no cabe en esta sesión, estas autorizado a dividirla." and "Usa Codex
  GPT-6.1 Sol high para la revisión final."
- Decision: network permission is granted for phase 4. "La lista de fuentes" is read as the sources plan row 4 names and
  those of alambique sections 3 and 7, plus the references the earlier research had already analysed (alambique section
  7, introduction). No amount was given; "todo dentro de presupuesto" is read as the session's own scope, with the rule
  of D-003 if it runs out. The phase is split where its inputs separate (D-019): part 4a, the sources of section 7b and
  below (Pullboard at source, catalogues and profiles 6.14 and 6.15, permitted versus present per D-015, installers that
  preview their plan), delivered in `brewstillery/bootstrap/reference-study.md`; part 4b, section 7a (UCI, apk-tools,
  SQLite) and the consolidated source catalogue (6.30), in its own chat. Codex (`gpt-6.1-sol`, effort high) reviews both
  parts together when phase 4 ends, as "la revisión final".
- Consequences: `plan.md` records the two parts. Part 4b's chat confirms network permission and Codex's launch with the
  maintainer at its start, since authority is given in chat. Clones of external sources stay in the session's scratch
  directory; nothing external is copied into this repository (plan row 4, "nothing copied").
- Sources: chat of session 6; `brewstillery/bootstrap/plan.md`, phase 4 and the splitting rule; D-015; D-019.
- Supersedes: none.

## R-011 — Session 6, part 4a

- Status: recorded
- Date: 2026-11-03
- Who: integrator Claude Code, `claude-opus-5-5`, effort high as the maintainer states it. No subagent, no dispatch.
  Network used under D-026: `git` over HTTPS, the GitHub API through `gh` (account `xel-arjona`, read-only calls), a
  summarizing web-fetch tool and one web search.
- Start checks: tree clean at `b152acc`; alambique input lines 1–274 sha256 `1ba1bf6d…9e5b`, separator lines 275–278,
  found once; `brewstillery/settings.toml` `05e9ba53…1397` and `brewstillery/xel/settings.toml` `1d2f11f3…1f33`,
  unchanged. The `sqlite3` command-line tool is still absent.
- Read, local and read-only: the four files alambique section 3 cites in `/home/xel/Dev/TickBrew/meta-harness/`
  (`EVALUATION.md`, `comparison.csv`, `SOURCES.md`, `CONTEXT_PROYECTO_IA_GITHUB.md`), whole. Its other files
  (`AW1_*`, `PILOT_PLAN.md`, `VALIDATION.md`, `evidence/`) were listed, not opened.
- Pinned on 2026-11-04 00:33 UTC (2026-11-03 18:33 local): the default-branch head of 29 GitHub repositories and of
  `gitlab.alpinelinux.org/alpine/apk-tools` and `git.openwrt.org/project/uci.git`, with each repository's default branch,
  archived flag and licence. The revisions and what was read are in `reference-study.md`, section 1.
- Cloned, read-only, never executed: 17 repositories at those heads, shallow, blobs over 2 MB filtered, in the session's
  scratch directory; deleted with it. Two Bash calls were refused by a transient failure of the auto-mode check; the
  same reads were done with the file reader.
- Written: `brewstillery/bootstrap/reference-study.md`; this entry and D-026; `plan.md` (phase 4 parts);
  `brewstillery/STATUS.md`; the answers zone of the alambique (three recommendations for section 7). The input is
  unchanged by the integrator (digest above).
- Not done in part 4a: Codex's review (at the end of phase 4, D-026); section 7a and the consolidated catalogue (4b).

## D-027 — Permissions for part 4b and Codex's review of phase 4

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in session 7's chat, the start message: "Usa Codex GPT-6.1 Sol high para la revisión final." and
  "Tienes autorizado permiso de red y cuentas con presupuesto suficiente."
- Decision: D-026's network permission is confirmed for part 4b. Codex (`gpt-6.1-sol`, effort high) reviews parts 4a and
  4b together. "Presupuesto suficiente" is read as a full review and scoped re-checks of the corrections until no
  confirmed finding is open, within the phase rule of D-003.
- Consequences: three Codex passes in session 7 (R-012). Network use stays within the sources of phase 4; nothing
  external is copied into this repository.
- Sources: chat of session 7; D-026; `brewstillery/bootstrap/plan.md`, phase 4.
- Supersedes: none.

## R-012 — Session 7, part 4b and Codex's review of phase 4

- Status: recorded
- Date: 2026-11-03
- Who: integrator Claude Code, `claude-opus-5-5`, effort high as the maintainer states it. Reviewer: Codex
  `codex-cli 0.166.0`, `gpt-6.1-sol`, effort high, read-only sandbox, no network, launched in this repository with
  `codex exec --json`; the stream names no model. No subagent. Network used under D-026 and D-027: `git` over HTTPS,
  the GitHub API through `gh` (read-only calls), `curl` for reachability, a summarizing web-fetch tool and five web
  searches.
- Start checks: tree clean at `39ec6a6`; alambique input lines 1–274 sha256 `1ba1bf6d…9e5b`, separator lines 275–278,
  found once; `brewstillery/settings.toml` `05e9ba53…1397` and `brewstillery/xel/settings.toml` `1d2f11f3…1f33`,
  unchanged. The `sqlite3` command-line tool is still absent.
- Input change during the session: the maintainer adopted the three recommendations for section 7 (links for Factory,
  codex-plugin-cc and amicus; HarnessBrew's description), committed alone with no AI co-author in `e386dd4` (D-022).
  Input lines 1–274 now sha256 `88e707b0…4fcc`; separator lines 275–278, found once.
- Read: the 7a sources at the heads pinned in R-011, re-checked unchanged at 2026-11-04 01:03 UTC, plus procd, OpenWrt's
  boot script and SQLite's pages; the identity, licence and head of every other catalogue source; in the earlier
  research, a search of the imported chat's transcript for the 7c addresses. What was read, file by file, is in
  `reference-study.md`, sections 1 (its "Re-read in part 4b" paragraph) and 9. Run on this machine: a Python probe on an
  in-memory SQLite database, `bwrap --version`, `git --version`. No source was run.
- Not read: OpenWrt's wiki, which requires passing a JavaScript bot check; one attempt to open it in a browser tool was
  refused by the auto-mode check and it was not circumvented (catalogue C-10, pending). OpenAI's Harness Engineering
  page answers 403 to automated reads (C-32, pending).
- Written: `reference-study.md` sections 9–11 (commit `4c27398`) and its corrections (`dda141c`, `98600d7`); this entry
  and D-027; `plan.md` (part 4b done); `brewstillery/STATUS.md`; the answers zone of the alambique (four
  recommendations for section 7). The input is unchanged by the integrator (digest above).
- Evidence for the review: the part 4a sources were cloned again at their pinned revisions, sparse where large, in the
  session's scratch directory; a disk quota on `/tmp` stopped a full clone of GSD Pi, so large clones were pruned to
  the files read. GitHub's commit API refused the short id `a227e38` (Spec Kit); the full id from `git ls-remote`
  resolved it.
- Codex, pass 1, on `4c27398`: 4 min 50 s; 41 commands, none failed; input 2,116,084 tokens (1,961,344 cached), output
  11,961 (4,594 reasoning). Verdict reject: six blocking findings, five minor. Every one checked by the integrator
  against the sources and confirmed:
  - P4-01 (blocking): PB-9 said Pullboard needs an export or a hosted relay across machines; `pullboard sync` shares an
    ordered append-only log through a Git ref. PB-9 and the storage options rewritten.
  - P4-02 (blocking): CA-4 and PR-5 missed ADR 2313's amendment (model and effort per invocation when the schema allows).
  - P4-03 (blocking): UC-2 called UCI deltas a pending change before it takes effect; UCI readers already apply them.
  - P4-04 (blocking): UC-4 and AP-6 said whole-file precedence cannot attribute a value; its limit is granularity.
  - P4-05 (blocking): part 4a marked documentation-only rows as verified, and attributed a refusal "by code" to amicus.
    Every finding of sections 2 to 7 and 10 now carries its label and, when verified, the file read.
  - P4-06 (blocking): SQ-5 presented `dbhash` as an exact-state digest; it omits `sqlite_` tables' content.
  - P4-07 to P4-11 (minor): PB-3's rejection is per frozen criterion; PR-3's model and effort resolve differently;
    IN-2's dry-run exceptions; OT-2's separate streams; AP-2's preview is conditional.
  The integrator also fixed a misquotation in IN-2. Corrections in `dda141c`.
- Codex, pass 2, scoped to `4c27398..dda141c`: 2 min 48 s; 40 commands, one non-zero exit; input 1,341,721 (1,226,752
  cached), output 6,616. Verdict accept: P4-01 to P4-10 closed, P4-11 still open (minor), new P4-12 and P4-13 (minor:
  `dbhash` keeps those tables' schema entries; PB-9's rewrite detection needs an acknowledged prefix). All three
  confirmed and corrected in `98600d7`.
- Codex, pass 3, scoped to `dda141c..98600d7`: 31 s; 8 commands; input 109,795 (82,432 cached), output 866. P4-11 to
  P4-13 closed, no new finding. **Verdict accept.**
- Progress measure (D-003): confirmed blocking findings 6, then 0; open findings 11, then 3, then 0. The prompts,
  streams and last messages stay in the session's scratch directory, not committed.
- Result: phase 4's acceptance holds as reviewed: each source with revision, date and what was read; each finding tied
  to an alambique point and a destination; nothing copied; every source of alambique sections 3 and 7 and of the
  earlier research in the catalogue (61 entries) with its status.

## D-028 — UCI and apk-tools: patterns only, their schema out of scope

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in session 7. In the alambique (commit `b999993`), section 7a: UCI's and apk-tools' "esquema y
  usabilidad no va enfocado a Fabrewk sino a TickBrew, sale del scope de este proyecto". Asked in chat whether the
  study's findings from them stay as phase 5 inputs, the maintainer chose "Conservar como patrones (Recomendado)".
- Decision: neither UCI nor apk-tools, nor their formats, is a candidate for Fabrewk. The findings UC-1 to UC-6 and
  AP-1 to AP-7 of `reference-study.md` stay as design patterns for phase 5 (a stable address per value, staged changes,
  "wanted" apart from "present", a preview with default deny, explicit trust of sources).
- Consequences: `reference-study.md` section 10 and catalogue rows C-07 and C-11 say so. Point 6.10 still names "un
  esquema inspirado en UCI" as an option; the answers zone recommends removing or clarifying it. The maintainer's
  description of UCI (successor of the NVRAM-based configuration, main configuration interface of OpenWrt) is stated by
  OpenWrt's documentation, seen only in search excerpts of its pages (bot check); the name is verified in
  `package/system/uci/Makefile` of `openwrt/openwrt`.
- Sources: alambique at `b999993`, section 7a; chat of session 7; `reference-study.md` section 10.
- Supersedes: none.

## D-029 — Phase 4 closed

- Status: decided
- Decided: 2026-11-03
- Origin: maintainer XEL in session 7's chat: "Si despues de esto ya no encuentras pendientes ni controversias por
  resolver, declara cerrado este chat."
- Decision: with D-028 settling the only controversy found in the maintainer's edit, phase 4 is closed on the evidence
  of R-011 and R-012: every source with revision, date and what was read; every finding tied to an alambique point and
  a destination; the catalogue complete; Codex's review accepted with no confirmed finding open. Session 7 closes with
  a clean tree (D-022).
- Consequences: next is phase 5, architecture decisions, in a fresh chat (Opus 5.5, effort high; Codex GPT-6.1 Sol,
  effort high, reviews). The recommendations in the answers zone are the maintainer's to adopt; none blocks phase 5.
- Sources: chat of session 7; R-011; R-012; D-028; `brewstillery/bootstrap/plan.md`, phases 4 and 5.
- Supersedes: none.

## R-013 — Session 7, closing sweep

- Status: recorded
- Date: 2026-11-03
- Who: integrator Claude Code, `claude-opus-5-5`, effort high.
- Input change: the maintainer's edit of sections 7a, 7c and 7d, committed alone in `b999993` (D-022). The separator
  moved to lines 282–285 and was found once; input lines 1–281 sha256 `2df9849f…40d2`. Settings files unchanged.
- Checked: the maintainer's description of UCI (D-028), through one web search and `gh` (the OpenWrt package
  Makefile); the openSUSE build page that repeats it answered 403.
- Written: D-028, D-029 and this entry; `reference-study.md` (section 10 scope note, rows C-07 and C-11); `plan.md`
  (phase 4 closed); `brewstillery/STATUS.md`; the answers zone of the alambique. The input is unchanged by the
  integrator (digest above).

## R-014 — Session 7, final sweep

- Status: recorded
- Date: 2026-11-03
- Who: integrator Claude Code, `claude-opus-5-5`, effort high.
- Input change: the maintainer adopted the answers zone's recommendations (6.10 without the UCI-inspired schema; the UCI
  line with its source link and corrected typos; the 7c links and OpenAlgo's typo; Superpowers' reason), committed alone
  in `4bdb997` (D-022). Separator lines 282–285, found once; input lines 1–281 sha256 `5aa13fd1…7861`. Settings files
  unchanged.
- Not adopted: restoring the 7c sentence that separates the case studies' claims from verified results. The integrator
  withdrew it: the alambique's opening rule (distinguish what is wanted from what is verified, check claims about
  external projects) and 6.33 already hold that expectation.
- Written: this entry; the answers zone (no recommendation this sweep). The input is unchanged by the integrator.

---

## D-030 — Phase 5: one part, Codex's review, and the licence criterion

- Status: decided
- Decided: 2026-11-04
- Origin: maintainer XEL in session 8's chat, the start message: "Modelo y esfuerzo: Claude Code en Opus 5.5, esfuerzo
  alto. Revisa Codex GPT-6.1 Sol, esfuerzo alto. Codex cuenta con su presupuesto completo, la licencia la decidiremos
  nosotros en función a que nuestro meta-harness FOSS debe poder convivir con cualquier otro producto de licencia
  abierta y/o cerrada, por lo que debmos buscar aquella(s) que solo obliguen a mencionarnos en proyectos que nos usen o
  hagan fork."
- Decision: Codex (`gpt-6.1-sol`, effort high) reviews phase 5 with its full budget, read as a full review and scoped
  re-checks until no confirmed finding is open (D-003). The licence is decided by the maintainer and the integrator,
  without a professional, against this criterion: Fabrewk stays open source, can live beside any product under an open
  or closed licence, and obliges those who use or fork it only to credit it. The trademark check before commercial use
  (alambique section 1) is not part of this and stays with a professional. Phase 5 is not split (D-019): its decisions
  share the same inputs and depend on one another.
- Consequences: no network permission was given for this session; none is used, and facts that would need it are
  marked unchecked in the phase deliverable. The deliverable is `brewstillery/bootstrap/architecture-decisions.md`.
- Sources: chat of session 8; `brewstillery/bootstrap/plan.md`, phase 5; D-019.
- Supersedes: none.

## D-031 — Phase 5: registry, settings and languages decided; the packet paused for a sweep

- Status: decided
- Decided: 2026-11-04
- Origin: maintainer XEL in session 8's chat, answering the integrator's questions on `architecture-decisions.md` after
  Codex's review accepted it (R-015): AD-1 "A: archivos en el árbol (Recomendado)"; AD-2 "Adoptar tal cual
  (Recomendado)"; AD-3 "Adoptar (Recomendado)". In place of an answer to AD-4: "DETENTE AQUÍ..!!! Creo que es imperante
  que hagas un barrido previo al Alambique antes de continuar, porque hay muchas propuestas de diseño que pueden
  modificar bastante tu plan arquitectónico. Registra de todas maneras lo ya respondido para integrarlo al plan que
  resulte de nuestro consenso."
- Decision: AD-1 option A: the registry's authority is immutable JSON event files under `brewstillery/data/`, replayed
  into state, committed by Fabrewk only in the publication modes that allow it; no SQLite in the first core. AD-2 as
  written: TOML never rewritten by Fabrewk; five levels (defaults, project, stage, task exception, and personal
  preferences limited to preference keys); authority read from the base commit, from `brewstillery/`'s own repository,
  or from a digest the responsible maintainer confirmed; detected values shown beside permitted ones and never stored;
  the catalogue's three tables in the project file. AD-3 as written: the three language keys in use, plus
  `codeLanguage`, `evidenceLanguage`, `delegationLanguage` (project) and `interfaceLanguage` (personal), optional and
  inherited. AD-4 to AD-11 are paused until the maintainer has acted on the sweep of input points 8.2 (a Rust core with
  extensions in other languages and a local API) and 8.3 (an optional central server with a dashboard).
- Consequences: these three decisions enter the plan that results from the consensus on 8.2 and 8.3; if that consensus
  changes one of them, the change is a new entry that supersedes this one. Points 8.2 and 8.3 reopen AD-4
  (implementation language) and AD-5 (core boundary), and add a decision on the central server.
- Sources: chat of session 8; `brewstillery/bootstrap/architecture-decisions.md` at `1f03deb`; the alambique input at
  `ecb5958`, section 8.
- Supersedes: none.

## R-015 — Session 8, phase 5 options, Codex's review and the sweep of 8.2 and 8.3

- Status: recorded
- Date: 2026-11-04
- Who: integrator Claude Code, `claude-opus-5-5`, effort high as the maintainer states it. Reviewer: Codex
  `codex-cli 0.166.0`, `gpt-6.1-sol`, effort high, read-only sandbox, launched in this repository with
  `codex exec --json`. No subagent, no network (D-030).
- Start checks: tree clean at `21ba7b5`; alambique input lines 1–281 unchanged since R-014; separator found once.
  Python 3.14.10 (3.14.8 in R-009), `uv` 0.13.5, Git 2.56.0, bubblewrap 0.12.0; `pipx` and the `sqlite3` command-line
  tool absent.
- Read: `STATUS.md`, `plan.md`, this file, the alambique, both settings files, `harness-verdicts.md`,
  `reference-study.md`; licence texts under `/usr/share/licenses/` (Apache-2.0 in `abseil-cpp/LICENSE`, MIT in
  `uv/LICENSE-MIT`); `uv tool install --help`. Nothing outside this repository otherwise.
- Written: `brewstillery/bootstrap/architecture-decisions.md` (`7d2001c`), corrected in `4387a9f`, `2a14c8b` and
  `1f03deb`; D-030, D-031 and this entry; `plan.md`; `STATUS.md`; the answers zone of the alambique.
- Codex, pass 1, on `7d2001c`: 7 min 15 s; 46 commands, 2 non-zero exits; input 1,451,304 tokens (1,355,264 cached),
  output 16,337 (12,390 reasoning). Verdict reject: two blocking findings, five minor, all confirmed by the integrator:
  - P5-01 (blocking): committing events unconditionally broke the right not to publish `brewstillery/` (6.27). Commit
    rules now depend on the publication mode (AD-1, AD-8, AD-11).
  - P5-02 (blocking): the settings precedence lacked the stage level of 6.16. Added (AD-2).
  - P5-03 to P5-07 (minor): alternatives for commands and installation; SQ-3's condition restored; one standard for
    the three registry options; CA-6 no longer cited for unknown keys; Apache-2.0's NOTICE and patent wording narrowed
    to its text.
- Codex, pass 2, scoped to `7d2001c..2a14c8b`: 3 min 1 s; 34 commands, 2 non-zero exits; input 710,604 (628,736
  cached), output 5,551. P5-01 to P5-07 closed; new P5-08 (blocking: no base commit exists when `brewstillery/` is
  neither published nor in its own repository) and P5-09 (minor: an unchecked claim about pipx). Both confirmed and
  corrected in `1f03deb`.
- Codex, pass 3, scoped to `2a14c8b..1f03deb`: 55 s; 17 commands; input 256,363 (202,368 cached), output 1,930.
  P5-08 and P5-09 closed, no new finding. **Verdict accept.**
- Progress measure (D-003): confirmed blocking findings 2, then 1, then 0. Prompts, streams and last messages stay in
  the session's scratch directory, not committed.
- Input change during the session: the maintainer relabelled 8.1 and added 8.2, 8.3 and 8.4, committed alone with no AI
  co-author in `ecb5958` (D-022). Separator lines 287–290, found once; input lines 1–286 sha256 `5f022423…55cd`. Codex's
  passes did not read that edit (pass 3 was told to ignore it).
- Sweep: the answers zone now holds the integrator's opinion on 8.2 and 8.3 and four recommendations for the input
  (section 1's Python intent; 8.2 split by idea with the need of each; 8.3 as a new point of section 6 with three
  clarifications; 8.4 removed once acted on). The input is unchanged by the integrator (digest above).

## D-032 — Domain, governance and core architecture after the sweep of section 8

- Status: decided
- Decided: 2026-11-04
- Origin: maintainer XEL in session 9's chat, answering the integrator's eighteen numbered recommendations (themselves
  drawn from the sweep of input points 6.47.a and 8.2 to 8.4): "1 y 2: De acuerdo en todo lo recomendado." "3: Estoy de
  acuerdo, solo me queda la duda […] quiero pensar que un meta-harness como somos nosotros siempre es preferible mantener
  en el propio dominio la administración compleja y dejar los archivos canónicos estándar lo más limpios posibles."
  "4, 5 y 6: De acuerdo en todo lo recomendado." "7: De acuerdo a lo que propones y sobre tus preguntas: a,b,c y d: Si tu
  recomendación técnica es la que le sigue a la pregunta en negritas, estoy de acuerdo." "8, 9, 10 y 11: De acuerdo en
  todo lo recomendado." "12, 13, y 14: De acuerdo en todo lo recomendado." "15: De acuerdo, y pienso va de la mano con tu
  punto 13, si no hay todavía una 'central' no hay nada que tenga que hacer 'phone-home'." "16: De acuerdo en tu
  recomendación […]. Nuestro piloto definirá también mejoras reales en campo." "17 y 18 De acuerdo en todo lo
  recomendado." Earlier in the same chat: "Sobre los directorios por mantenedor […] quedemosla […] solo existiría un solo
  ALAMBIQUE.md en el root de brewstillery y los archivos fermento de cada mantenedor serían responsabilidad de cada
  quien." The private folder per machine was the maintainer's own addition in that chat ("~/.config/fabrewk/ donde allí
  el usuario de cada host pueda guardar llaves […]").
- Decision, by the numbers of the integrator's list:
  1. The domain is `.brewstillery/` (hidden), in every repository Fabrewk serves and in its own.
  2. One project alambique, `.brewstillery/ALAMBIQUE.md`, and one fermento per maintainer,
     `.brewstillery/<alias>/fermento.md`, written only by that maintainer above the existing separator; agents answer
     below it. The per-maintainer space stays, holding the fermento, the personal settings and the support material.
  3. The project alambique is only the description of what the project must be, with stable headings and numbering;
     no settings, no answers zone. The rules Fabrewk administers for agents live inside the domain; the root entry
     files (`AGENTS.md`, `CLAUDE.md`) carry only the delimited reference to `STATUS.md` (the maintainer's intuition,
     agreed by the integrator; its file name and form are fixed by the design).
  4. Nobody edits the project alambique directly: an agent drafts the edit as a proposal and a maestro approves it.
  5. The state of each proposal (pending, approved, integrated, discarded) is recorded by Fabrewk with the identity and
     words of who decides, bound to the exact text, and goes stale when that text changes; the fermento and the
     alambique only show it.
  6. A maintainer's identity is the SSH key they sign with, listed in the project settings; the list changes only with
     the signature of a key already on it.
  7. Three roles are designed now: maestro, supervisor, contributor. The first core enforces maestro and contributor;
     supervisor comes when a second maintainer needs it. The registry records the role of every act from the start.
     (a) Roles are assigned and revoked by a change signed by a maestro already on the list. (b) With one maestro, the
     maestro approves alone and independent review comes from agents of another provider. (c) A supervisor's scope is
     an area (paths), the whole project when no area is given. (d) A contributor proposes and launches no agent, not
     even read-only.
  8. Roles live in a section of the project settings, not in a Markdown file.
  9. Human ownership of paths is read from the Git host's code-owners file (CODEOWNERS) where it exists; Fabrewk keeps
     no second map.
  10. A private folder per machine, `~/.config/fabrewk/` on Linux and the user configuration folder on macOS and
      Windows, outside every repository: Fabrewk's own keys and tokens (preferably in the operating system's key
      store, an owner-only file as fallback) and machine data that detection cannot find. It never grants authority,
      never copies the sessions of Claude Code or Codex, is never copied between machines, and agents do not see it.
      Cross-project preferences there are deferred.
  11. This repository migrates once, in its own commit, right after this decision and before the core specification.
  12. The core is written in Rust; every crate is justified; a minimum Rust version is fixed.
  13. Extensions are separate programs in any language that talk to the `fabrewk` command through stable structured
      output; the project settings declare which may run; none writes the registry; no always-on service or open port
      until the central server exists.
  14. Confinement is one backend per operating system, bubblewrap on Linux ARM64 first, Docker as an alternative; a
      platform counts as supported only once tested on it.
  15. No always-on process in the first core; automatic launches happen only inside a stage's authorized plan.
  16. Semantic code search (a vector index) is a later extension, off by default, LanceDB as candidate, vectors
      computed on the machine by default and by an external provider only with explicit permission; built after the
      pilot and kept only if it measures less consumption or rework.
  17. The central server is a later stage, optional and off by default, with the rules of Pullboard's relay: it shows
      and does not command (what is done on the dashboard returns to the repository as a signed request), holds records
      and never code, and connecting a repository publishes its record (warning of 6.27). It lives in this repository
      as a separate program until it needs its own release cycle. From now on each registry entry names the instance
      that wrote it.
  18. Containers for a project's checks are optional, declared by each project, outside the core.
- Consequences: the integrator delivers a second audit of the alambique with the full proposed text of the project
  alambique and of XEL's fermento (`brewstillery/bootstrap/alambique-audit.md`, replacing the phase 1 audit, which stays
  in Git history at `8dfd9cb`). Once the maintainer adopts it, the migration of point 11 moves the domain and the
  files, and `AGENTS.md`, `CLAUDE.md`, `STATUS.md` and `plan.md` follow. Phase 5 resumes with AD-4 rewritten for Rust,
  AD-5 for the extension contract, AD-7 for a binary installation, and new decisions for points 1 to 10 and 17 where
  `architecture-decisions.md` lacks them.
- Sources: chat of session 9; the answers zone of `brewstillery/xel/alambique.md` in session 9; Pullboard at
  `6231e10`, re-read in session 9 (R-016).
- Supersedes: D-002 for the domain's name, the per-maintainer alambique and the swept file (now the fermento); D-031
  only for AD-2's rule "no user-level settings file outside the repository in the first core" (replaced by point 10)
  and for AD-1's path, which becomes `.brewstillery/data/`; D-004 and D-009 are unaffected (the pilot module stays in
  Python, it is a product, not Fabrewk).

## D-033 — The maintainer's annotations on the proposed alambique

- Status: decided
- Decided: 2026-11-04
- Origin: maintainer XEL in session 9's chat, reviewing Part 2 of the second audit: "El encabezado preferiblemente que
  diga # ALAMBIQUE para destilar Fabrewk"; "Hay que comenzar describiendole al usuario (mantenedores) con la analogía,
  ejemplo: 'El Alambique es el utenciliio al cual se le vierte el fermento preparado por cada maestro para extraer un
  destilado fino.' Creo que no debemos aquí explicar que es el único ni que lo aprueban sus maestros"; "Hay que explicar
  claro y con poca prosa que cada mantenedor en Fabrewk cuenta con su propio dominio y directorio y en el existe un
  markdown llamado fermento.md el cual es el archivo canónico del arnés para oficializar input de diseño entre
  mantenedor y agentes […]"; "'prosa minimalista, clara y contundente' serian una mejor definición a mi juicio";
  "deberiamos idear una clave de palabras que explique en el mismo Alambique que ya está decidido, integrado,
  implementado, proyectado o en controversia, en parentesis o en brackets [...], como mas convenga, pero sería bueno
  estandarizarlos y explicarlos"; for section 1's last item, "Instalación sencilla ya sea mediante un llamado a un
  script con `curl` desde nuestro propio repositorio de GitHub el cual comience un proceso de 'onboarding' […] o
  llamarlo también mediante un prompt preestablecido para que cualquier agente que el mantenedor use pueda ejecutar el
  onboarding de instalación al repo."; "Todo lo demás parece ya encajar muy bien con lo propuesto de forma clara y
  limpia., verificalo antes de ejecutar el punto 2 de tu propuesta."
- Decision:
  1. The alambique's title is "ALAMBIQUE para destilar Fabrewk" and it opens with the still analogy. That it is the
     project's single alambique, approved by its maestros, is said in section 2 and in the usage list, not in the
     opening.
  2. "Cómo se usa" is a short list: each maintainer's directory `.brewstillery/<alias>/` and what the alias is;
     `fermento.md` as Fabrewk's canonical file for design input between maintainer and agents; support files beside
     it; only maestros approve what enters the alambique, an agent drafts each edit and a maestro approves it. Alambique
     and fermentos are written in "prosa minimalista, clara y contundente".
  3. Status keys, in brackets (parentheses are already common in the prose), at the end of the sentence or point title
     they qualify: [proyectado] (a later stage), [en controversia] (proposals or decisions in conflict; nothing designed
     or built until resolved), [decidido] (in the record), [implementado] (code exists and passes its tests),
     [integrado] (implemented, independently verified and delivered in a published release). No key means an open
     expectation. The keys reflect the registry, which prevails: an agent proposes a change of key in an alambique edit
     and a maestro approves it. Until code exists no point goes past [decidido]. The integrator chose the brackets and
     the definitions, as the maintainer left that open ("como mas convenga").
  4. To keep the key [integrado] distinct, the proposal state that D-032 item 5 calls "integrated" is worded
     "incorporated into the alambique" (6.8). Wording only.
  5. Section 1's installation item takes the maintainer's wording: a `curl` script from Fabrewk's own GitHub repository
     that starts a guided onboarding with the options chosen for the repository, or a preset prompt any agent can run;
     in both, nothing is written before the maintainer confirms (section 1's assessment item). RSC-Harness, the
     maintainer's example, was checked at source: its onboarding runs with `npx`, not `curl`, interviews, prints the
     exact plan and writes nothing until that plan is accepted, and also starts from a prompt to the agent. It stays as
     the reference for guided onboarding (7b).
  6. The maintainer authorized step 1 of the audit's Part 5 (the integrator's migration commit) once the proposal was
     verified.
- Consequences: Parts 1 to 3 of `alambique-audit.md` updated; the migration commit follows. AD-7 (installation) must
  cover both routes and decide how the script checks what it downloads; nothing about that is added to the alambique.
- Sources: chat of session 9; RSC-Harness's repository page (R-016).
- Supersedes: the wording of D-032 item 5 only.

## R-016 — Session 9, sweep of section 8, D-032 and D-033, second audit and migration

- Status: recorded
- Date: 2026-11-04
- Who: integrator Claude Code, `claude-opus-5-5`, effort high as the maintainer states it. No subagent, no Codex pass
  in this session. Network authorized by the maintainer in chat: "TE autorizo acceder a la red para hacer tu
  verificación de estos proyectos o de algún otro que consideres pertinente."
- Start: HEAD `8dfd9cb`; the maintainer had edited the input above the separator (6.47.a, section 8, section 1).
  During the session the maintainer edited it again (a typo, branding moved to the agent of section 4, 8.3 removed).
  Final input lines 1–387, sha256 `06aa160f…f431`; separator lines 388–391, found once. The maintainer's edits are
  committed alone in `ae1269d`, with the answers zone as it was at `8dfd9cb`.
- Read: `STATUS.md`, `plan.md`, this file, the alambique, both settings files, `architecture-decisions.md`,
  `reference-study.md`.
- Network: Pullboard shallow-cloned into the session's scratch directory at `6231e10` (2026-10-28) and read at source:
  `SPEC.md`, `README.md`, `docs/architecture.md`, `DOCTRINE.md`, `AGENTS.md`, `src/init.js`, `src/commands/watch.js`,
  `src/row-decisions.js`, `relay/README.md`, and searches over `src/`. Findings used: `init` appends a delimited block
  with its working loop to `AGENTS.md` and writes `CLAUDE.md` as `@AGENTS.md`; doctrine is a separate root file; lanes
  in `pullboard.json` own folders and a pre-commit check refuses changes outside them; an approvers list; SSH
  sign-offs; a machine home `~/.pullboard`; `pullboard serve` (local API and view); a relay deployed as a Docker
  container that shows and does not command. Not found: a vector database, containers or WebAssembly for agents.
  Correction of the integrator's first sweep, which had said Pullboard lacked worktrees, lanes, a view and a hidden
  folder. Web pages: Kilo Code's codebase indexing (`kilo.ai/docs/customize/context/codebase-indexing`), LanceDB's
  repository, DuckDB's vector extension (`duckdb.org/docs/stable/core_extensions/vss`), RSC-Harness's repository page;
  two searches (agentic search versus vector retrieval in Claude Code; RSC-Harness's installation).
- Written: the answers zone of the alambique (two sweeps, replaced each time); D-032, D-033 and this entry;
  `alambique-audit.md` rewritten as the second audit (the phase 1 audit stays at `8dfd9cb`); the migration (below).
  The proposed text of Part 2 is generated by a script from a verbatim copy of input lines 1–290, with each replaced
  passage checked to occur exactly once; the annotations of D-033 applied the same way.
- Migration (D-032 item 11), old path → new path: `brewstillery/` → `.brewstillery/`; `brewstillery/xel/alambique.md`
  → `.brewstillery/ALAMBIQUE.md` (moved, so the text keeps its history; the maintainer then replaces its content with
  Part 2); `.brewstillery/xel/fermento.md` is new, created by the maintainer from Part 3. Every other file keeps its
  name under the new domain. Older entries of this record keep their paths as history.

## D-034 — The fermento's answer carries the drafted alambique text; the alambique is rewritten concise

- Status: decided
- Decided: 2026-11-04
- Origin: maintainer XEL in session 10's chat. Point 2 of the fermento proposes treating the whole alambique as
  fermento text, rewriting it clear and concise as the integrator, with a second review by Codex (GPT-6.1 Sol, high),
  the preview shown in the fermento's answer and validated there. Answering the integrator's five precisions: "La idea
  del nuevo ALAMBIQUE.md es que sea claro para el mantenedor maestro (yo) Y PARA EL AGENTE integrador, que eres tú. Tú
  y solo tú diseñas la arquitectura y yo solo la autorizo, expando o contraigo ideas o features, tu escribiras el
  código., el nuevo ALAMBIQUE.md debe ser claro y consiso para no gastar tokens en lectura de prosa demasiado extensa
  […] fermento.md sería para hacer el input de mis palabras en la primera sección y en la segunda tú contestas y si hay
  consenso, redactas la prosa recomendada que va al ALAMBIQUE.md. Ahora es excepcional porque el alambique ya está
  crecido […] considero que el ejercicio del workflow no cambia. Proponme que recomiendas hacer al respecto." To the
  integrator's recommendation (rule change, rewrite criteria, order, section 7 options): "De acuerdo con tu
  recomendación sobre (b). Nota: Propongo que el alambique original le hagas una copia previa y la dejes archivada
  como evidencia, si algo no cuadra tenemos la fuente completa de lo que se tomó como 'fermento' para el nuevo
  alambique optimizado." No objection was raised to the rest of the recommendation.
- Decision:
  1. A fermento's answer holds the current recommendations for the maintainer's input and, for each proposal with
     consensus, the drafted alambique text, ready to paste. It is still replaced whole at every sweep; nothing
     accumulates.
  2. The alambique is written for the maestro and the integrator: the same expectations in fewer words, so reading it
     costs few tokens. Kept: point numbers 6.1–6.52, the status keys, the title, the opening analogy and the usage
     list (D-033). Project voice ("el maestro") instead of the first person. Detail already in the record stays out.
  3. Roles as the maintainer states them: the integrator designs the architecture and writes the code; the maestro
     authorizes, widens or narrows ideas and features. Independent review stays with another provider (6.42).
  4. Section 7 (sources) leaves the alambique for the source catalogue of 6.30, today section 11 of
     `.brewstillery/bootstrap/reference-study.md`; the alambique keeps one line pointing there. The five sources
     added in session 9 and missing from the catalogue are added to it first.
  5. The alambique as it stood before the rewrite is archived, byte for byte, as
     `.brewstillery/bootstrap/alambique-original.md` (sha256 `143d4292…dba70`, equal to `ALAMBIQUE.md` at `84e95e8`).
  6. Order: the integrator drafts; Codex checks point by point against the archived text that no meaning is lost or
     changed, and the prose; the integrator corrects; the preview goes in XEL's fermento answer with a short list of
     changes by section; XEL validates in chat and pastes it in their own commit, which is the proof of their approval.
     The next sweep removes the preview and point 2.
  7. This rewrite comes before phase 5 resumes, since the architecture decisions cite alambique points.
- Consequences: `AGENTS.md` (fermentos) updated; `plan.md` and `STATUS.md` record the step; the catalogue gains
  C-62 to C-66.
- Sources: chat of session 10; `.brewstillery/xel/fermento.md`, point 2.
- Supersedes: D-017 only where it sends agent work that is not a recommendation to a bootstrap file: a drafted
  alambique edit now goes in the fermento's answer.

## R-017 — Session 10, sweep of point 2 and the concise alambique

- Status: recorded
- Date: 2026-11-04
- Who: integrator Claude Code, `claude-opus-5-5`, effort high as the maintainer states it. Reviewer: Codex
  `codex-cli 0.166.0`, `gpt-6.1-sol`, effort high, read-only sandbox, launched in this repository with
  `codex exec --json`. No subagent, no network.
- Start checks: HEAD `84e95e8`. The migration's step 3 passed: `ALAMBIQUE.md` equals the audit's Part 2 except the
  final newline; no section 8; the separator is not in the alambique and is found once in the fermento, whose
  committed text equalled Part 3. Two path comments of `.brewstillery/settings.toml` were stale; the integrator gave
  the exact lines in chat and the maintainer changed them (TOML parsed). The maintainer had added point 2 and edited
  the fermento's heading; after the first sweep they removed a `#` line that looked like the separator's border.
  Final input lines 1–19 (through the separator) sha256 `dd4c4b9f…262b`; the separator's marker line found once.
- Read: `STATUS.md`, the alambique, the fermento, both settings files, `plan.md`, this file (header, D-017, D-032,
  D-033, R-015, R-016), `alambique-audit.md` (header, Parts 2, 3, 5), `reference-study.md` section 11.
- Written: the fermento's answer (two sweeps); D-034 and this entry; `AGENTS.md` (fermentos); `plan.md`;
  `STATUS.md`; `bootstrap/alambique-original.md` (byte copy of `ALAMBIQUE.md` at `84e95e8`, sha256 `143d4292…dba70`);
  `reference-study.md`: catalogue rows C-62 to C-66 and the notes carried from alambique section 7.
- Correction: in chat the integrator first said the catalogue held every source of section 7; the five added in
  session 9 (LanceDB, Qdrant, DuckDB's and SQLite's vector extensions, Kilo Code) were missing. Added as C-62 to C-66.
- Rewrite: 8,767 words to 7,216 (−17.7 %), 61,662 bytes to 48,955 (−20.6 %). Point numbers, status keys (20
  [decidido], 3 [proyectado]) and D-033's title, analogy and usage list kept. The preview is in the fermento's answer
  between `<!-- INICIO ALAMBIQUE.md -->` and `<!-- FIN ALAMBIQUE.md -->`, sha256 `4f6a1e6b…a3bf2`.
- Codex, pass 1, on the working tree: 3 min 40 s; 16 commands, none failed; input 360,134 tokens (308,480 cached),
  output 9,276 (6,219 reasoning). Verdict reject: ten blocking, one minor, all confirmed by the integrator: AR-01
  (opening, "por sí solo" lost), AR-02 (section 5, "sin asumir" became a ban), AR-03 (6.7, "que interfieran con su
  lectura" lost), AR-04 (6.8, each recommendation's evaluation lost), AR-05 (6.14, "puede admitir" became a general
  claim), AR-06 (6.14 and 6.15 narrowed to meta-harnesses), AR-07 (6.30, an added rule that new sources go through a
  fermento), AR-08 (catalogue C-51 changed "to be evaluated" to "decided"), AR-09 (change list said 21 %; 18 % in
  words), AR-10 (change list claimed every 7a/7b reference was redirected; section 1 and 6.50 were not), AR-11 (6.5,
  prose). A first launch waited about 19 minutes on standard input and reviewed nothing; it was stopped and relaunched
  with standard input closed.
- Codex, pass 2, scoped to the corrections: 53 s; 7 commands, none failed; input 159,961 (130,944 cached), output
  1,644. AR-01 to AR-11 closed, no new finding. **Verdict accept.**
- Progress measure (D-003): confirmed blocking findings 10, then 0. Prompts, streams and last messages stay in the
  session's scratch directory, not committed.

## D-035 — The concise alambique is adopted

- Status: decided
- Decided: 2026-11-04
- Origin: maintainer XEL in session 10's chat: "Quedó aprobado, ya realizé la copia con tu comando propuesto."
- Decision: the rewrite of D-034, accepted by Codex at its second pass (R-017), is the project alambique. The
  maintainer copied it from the fermento's preview with the integrator's command; `.brewstillery/ALAMBIQUE.md` sha256
  `4f6a1e6b…a3bf2`, equal to the preview of R-017, with no separator and no markers. Committed alone by the
  maintainer's identity, without AI co-author (D-022).
- Consequences: the fermento's answer drops the preview and recommends removing point 2. Phase 5 resumes in a new
  chat (plan row 5).
- Sources: chat of session 10; R-017.
- Supersedes: none.

## R-018 — Session 11, phase 5 resumed: AD-4, AD-5, AD-7 rewritten, AD-12 to AD-15 added

- Status: recorded
- Date: 2026-11-04
- Who: integrator Claude Code, `claude-opus-5-5`, effort as STATUS.md names it (high). Reviewer: Codex
  `codex-cli 0.166.0`, `gpt-6.1-sol`, effort high, read-only sandbox, launched in this repository with
  `codex exec --json`, standard input closed. No subagent, no network (none was granted this session).
- Start: HEAD `f251bd4`; the maintainer had removed fermento point 2, as the last answer recommended. That edit is
  committed alone in `11a022f`, with no AI co-author (D-022); the separator's marker line found once. The fermento's
  answer was replaced: no current recommendation (point 1 is a machine task outside Fabrewk's construction). Input
  above the answer heading unchanged by the integrator (digest compared before and after).
- Read: `STATUS.md`, `ALAMBIQUE.md`, the fermento, both settings files, `plan.md`, this file (D-030 to D-035, R-015 to
  R-017), `architecture-decisions.md`, `harness-verdicts.md` ("In short", observations for phase 5), the finding keys of
  `reference-study.md`.
- Checked on this machine: `rustc`/`cargo` 1.99.1 (Fedora), no `rustup`; OpenSSH 10.2p1 with `ssh-keygen -Y`;
  no Git signing configured in this repository; `codex-cli` 0.166.0, `gh` 2.97.0, `curl` 8.18.0; `minisign`,
  `cosign`, `docker`, `podman`, `tokei`, `cargo-deny` absent. Recorded in section 0 of the deliverable.
- Written: `architecture-decisions.md`: AD-4 (Rust toolchain, platforms, dependencies, tests), AD-5 (three rings and
  the extension contract, budget in Rust), AD-7 (release binaries, the curl route with onboarding in the binary, the
  agent route ending at `assess`, the download checked by an SSH signature) rewritten; AD-12 (domain layout, entry
  files, `RULES.md`, reserved names, separator marker), AD-13 (alambique edits and proposal state), AD-14 (identity,
  roles, signed events), AD-15 (private folder, instance identity) added for D-032 points 1 to 10 and 17; AD-6, AD-8 to
  AD-11 and the closing list adjusted; "Since D-032" notes under the decided AD-1 to AD-3, whose text is unchanged.
  6,688 words to 12,358. This entry; `plan.md`; `STATUS.md`.
- Codex, pass 1, on the working tree (sha256 `ce5aecd8…`): 4 min 53 s; 17 commands, 1 non-zero exit; input 1,027,905
  tokens (921,856 cached), output 9,377 (5,849 reasoning). Verdict reject; all confirmed by the integrator:
  - P5-10 (blocking): staleness of a proposal checked by finding its passage anywhere in the fermento; now bound to the
    numbered input point above the separator (AD-13).
  - P5-11 (blocking): a confined extension could not ask the core for changes from inside its confinement; requests
    now go out on its standard output to the outer core, which validates and applies them (AD-5).
  - P5-12 (blocking): the instance's name included the digest of the first event, which itself carries the name; now
    events record the machine key's fingerprint and the project is given by the registry that holds them (AD-15).
  - P5-13 to P5-16 (minor): the agent route overstated that an agent cannot confirm for the person (AD-7); "verifiable
    from the first event" overstated an unsigned fingerprint (AD-15); a static binary does not run on "any" Linux
    (AD-4); a hardware key's type does not prove touch enforcement (AD-14).
- Codex, pass 2, scoped to the corrections (sha256 `bb1a1d80…`): 1 min 26 s; 19 commands, none failed; input 462,060
  (395,904 cached), output 2,902. P5-10 to P5-16 closed, no new finding. **Verdict accept.**
- Progress measure (D-003): confirmed blocking findings 3, then 0. Prompts, streams and last messages stay in the
  session's scratch directory, not committed.

## D-036 — Phase 5: AD-4 to AD-15 decided as recommended

- Status: decided
- Decided: 2026-11-04
- Origin: maintainer XEL in session 11's chat, answering the integrator's twelve questions, one per decision, each
  summarizing the recommendation of `architecture-decisions.md` as Codex accepted it (R-018). Answers: AD-4 "Adoptar
  (Recomendado)"; AD-5 "Adoptar (Recomendado)"; AD-6 "Adoptar (Recomendado)"; AD-7 "Adoptar (Recomendado)"; AD-8
  "Adoptar (Recomendado)"; AD-9 "Adoptar (Recomendado)"; AD-10 "Adoptar (Recomendado)"; AD-11 "Adoptar
  (Recomendado)"; AD-12 "Adoptar (Recomendado)"; AD-13 "Adoptar (Recomendado)"; AD-14 "Adoptar (Recomendado)"; AD-15
  "Adoptar (Recomendado)". No notes were added.
- Decision: as written in `architecture-decisions.md` at `069c4d2`:
  - AD-4: MSRV is the Rust version Fedora ships on the maintainer's machine when phase 7 starts (today 1.99.1),
    raised only by a maestro's recorded decision; edition 2024; Linux ARM64 first with static musl release binaries,
    then Linux x86-64 and macOS ARM64, Windows last, each supported only once tested; every runtime crate with a
    recorded reason approved by a maestro, `Cargo.lock` committed, `--locked` builds, licence and advisory checks;
    first crates `serde` (with derive), `serde_json`, `toml`, `sha2`, the parser left to phase 6; `git` and
    `ssh-keygen` as external tools; `cargo test`, past defects as tests, property and mutation testing.
  - AD-5: three rings (core; built-in modules for tool adapters, confinement backends and the installer; extensions
    as separate programs `fabrewk-<name>`); the six-point extension contract (permission in project settings, read by
    `--json`, change requests on standard output validated and applied by the outer core, never a person's signature,
    confined where a backend exists, no service); budget in non-blank, non-comment Rust lines outside tests, with the
    proposed limits as estimates that phase 6 freezes after measuring a skeleton.
  - AD-6: flat English verbs, the rules, and the first set of fourteen commands as phase 6's starting list.
  - AD-7: release binaries fetched by a POSIX script that verifies an SSH signature against keys written in it, with
    no flag to skip the check, installing only `~/.local/bin/fabrewk` without `sudo`; onboarding in the binary,
    reading from the terminal; the agent route ending at `assess`, with `apply` run by the maestro; `cargo install`
    and a build from a clone as developer routes; the seven steps.
  - AD-8: `.brewstillery/.gitignore` with the one rule `local/`; `local` reserved; the classification.
  - AD-9: Fabrewk's code and documentation under MIT OR Apache-2.0 with a `NOTICE` file; MIT-0 for what Fabrewk
    writes into served repositories; each release ships a third-party notices file generated from the lock file;
    licence files created with the first code (phase 7).
  - AD-10: the group of tasks is a `stage`; the first core runs its tasks serially.
  - AD-11: the permanent form of D-022 as written; D-022 stays in force during the bootstrap.
  - AD-12: `.brewstillery/RULES.md` as a generated view read by the integrator; the root entry block and
    `CLAUDE.md` as `@AGENTS.md`; the reserved names and the alias rule; the separator's marker line found outside
    fenced blocks.
  - AD-13: proposals recorded by the sweep and bound to the numbered input point, approved or discarded by a signed
    maestro act, applied by Fabrewk, direct edits caught by `fabrewk check`; it replaces the manual copy once the core
    exists.
  - AD-14: the maintainers' table in the project settings; signed events (namespace `fabrewk`); a new list adopted
    only by a listed maestro's signature; trust on first use for the first maestro; maestro and contributor enforced
    first; areas from the code-owners file; keys that ask for presence recommended.
  - AD-15: the private folder's places (Windows' local application data); `machine.toml` and the instance key; the
    machine key's fingerprint in every event, signing with it deferred to the central server's stage.
- Consequences: every decision of the phase 5 packet is decided (AD-1 to AD-3 in D-031). The licence is decided, so
  `AGENTS.md` no longer lists it as undecided; its files are still created only with the first code. The settings
  key `layout.reservedNames` lacks `local`, `RULES.md` and `.gitignore` (AD-8, AD-12); agents do not edit settings, so
  the integrator proposes the change to the maintainer. The alambique's section 1 still says the licence is to be
  decided, and section 2 leaves the rules file's name to the design; the integrator drafts both edits in the
  fermento's answer for a maestro to approve (D-033, D-034). Phase 5 closes on the maintainer's word; phase 6 (core
  specification and criteria) follows.
- Sources: chat of session 11; `architecture-decisions.md` at `069c4d2`; R-018.
- Supersedes: none.

## R-019 — Session 11, the decided packet re-checked by Codex

- Status: recorded
- Date: 2026-11-04
- Who: integrator Claude Code, `claude-opus-5-5`; reviewer Codex `codex-cli 0.166.0`, `gpt-6.1-sol`, effort high,
  read-only sandbox, standard input closed. No network.
- Written after D-036: decision markers and a state paragraph in `architecture-decisions.md` (no decided text
  changed); D-036; `AGENTS.md`'s list of what is not yet created (licence files, the licence itself decided); the
  fermento's answer, now carrying two drafted alambique edits (section 1, the licence; section 2, the rules file
  `RULES.md`) for a maestro to approve, since the alambique still called both undecided.
- Codex, pass 3, on the diff against `069c4d2`: 1 min 14 s; 14 commands, none failed; input 319,762 (252,800 cached),
  output 2,978. Verdict reject: P5-17 (blocking, D-036 said the third-party notices go inside the binary; AD-9 says a
  notices file ships with each release) and P5-18 (minor, the alambique's section 1 still says the licence is to be
  decided). Both confirmed: D-036 corrected; the alambique edit drafted in the fermento's answer. Codex noted it read
  one local skill file outside the paths it was given; nothing was changed.
- Codex, pass 4, scoped: 39 s; 9 commands, none failed; input 118,436 (88,704 cached), output 1,476. P5-17 closed,
  P5-18 closed by the draft pending a maestro's approval; both replaced passages occur once in the alambique; the
  fermento's input unchanged since `069c4d2`. **Verdict accept.**
- Progress measure (D-003): confirmed blocking findings 1, then 0.

## D-037 — Two alambique edits applied by the integrator on the maestro's authorization; phase 5 closed

- Status: decided
- Decided: 2026-11-04
- Origin: maintainer XEL. In the fermento, point 2 (committed alone in `a577341`): "AUTORIZACIÓN: Tienes autorizado
  realizar los cambios automaticamente que has propuesto en mi fermento para el ALAMBIQUE.md sobre las 2 secciones sin
  que tenga que hacerlo yo de forma manual." Ratified in session 11's chat: "Tengo instrucciones y autorizaciones
  pendientes para tí en mi fermento, ratifico las mismas por este chat." On closing phase 5, answering the
  integrator's question: "Cerrar la fase 5 (Recomendado)".
- Decision:
  1. The integrator applied the two edits drafted in the fermento's answer and checked by Codex (R-019), byte for byte:
     section 1's licence line, now "[decidido]", and section 2's sentence on the agents' rules file, now naming
     `.brewstillery/RULES.md`. Each replaced passage occurred once. `ALAMBIQUE.md` sha256 `f2cac00b…`. This is the
     maestro's approval of those two edits; the general rule (an agent drafts, a maestro approves, nobody edits the
     alambique directly) is unchanged, and the maestro may again authorize an agent to apply a specific approved edit.
  2. Phase 5 is closed: every decision of its packet is decided (D-031, D-036) and Codex's review found no open
     blocking contradiction with the alambique (R-018, R-019).
- Consequences: phase 6 (core specification and criteria) starts in a new chat. The fermento's answer recommends
  removing points 1 and 2. The settings key `layout.reservedNames` now lists `local`, `RULES.md` and `.gitignore`,
  changed by the maintainer (TOML parsed, checked by the integrator).
- Sources: chat of session 11; `.brewstillery/xel/fermento.md` at `a577341`; R-019.
- Supersedes: D-034 point 6 only where it has the maestro paste the text: for these two edits the integrator applied
  it on the maestro's explicit authorization.

## D-038 — Phase 6 split into parts 6a and 6b; the size budget frozen at phase 7's start

- Status: decided
- Decided: 2026-11-04
- Origin: maintainer XEL in session 12's chat, answering the integrator's three questions at the start of phase 6:
  "Dos partes, 6a y 6b (Recomendado)"; "Congelarlo al inicio de la fase 7 (Recomendado)";
  "bootstrap/core-specification.md (Recomendado)".
- Decision:
  1. Phase 6 is split where its inputs separate (D-019). Part 6a reads the alambique and the decided architecture and
     delivers: the classification of every alambique point (indispensable for the code, later stage, or alternative to
     evaluate) with each indispensable point mapped to a requirement or an explicit deferral; the states and the
     transition table; the commands with their refusals; the semantics of the checks; the names of the settings keys and
     what else phase 5 left to phase 6. Part 6b reads part 6a and `past-defect-cases.md` and delivers the test plan, with
     every past-defect case taken in, and the acceptance criteria. Each part runs in its own chat; Codex (GPT-6.1 Sol,
     effort high) reviews both together when phase 6 ends, since the phase delivers documents.
  2. The size budget is not frozen in phase 6: no code is written before phase 7 (`AGENTS.md`) and the counting tool
     does not exist yet. Phase 6 fixes the counting rule and keeps AD-5's limits as provisional ceilings. The first
     commit of phase 7 is the skeleton with its line counter; a maestro freezes the budget after measuring it, before
     any feature is built. This amends plan rows 6 and 7 and AD-5's "phase 6 freezes".
  3. The specification is written in English in `.brewstillery/bootstrap/core-specification.md`.
- Consequences: `plan.md` records the parts and the amended rows. The maintainer's approval of the text (plan row 6)
  comes after Codex's review of both parts.
- Sources: chat of session 12; `plan.md` rows 6 and 7; D-019; AD-5 in `architecture-decisions.md`.
- Supersedes: D-036 only where it says phase 6 freezes the size budget (AD-5): it is frozen at phase 7's start.

## R-020 — Session 12, phase 6 started: part 6a

- Status: recorded
- Date: 2026-11-05
- Who: integrator Claude Code, `claude-opus-5-5`. No subagent, no dispatch, no network.
- Start: HEAD `b387346`; the maintainer had removed fermento points 1 and 2, as the last answer recommended. That
  edit is committed alone in `919886d`, with no AI co-author (D-022); the separator's banner line found once. The
  fermento's answer was replaced: no current recommendation. Input above the separator unchanged by the integrator
  (sha256 `9d9c2158…` before and after).
- Read: `STATUS.md`, `ALAMBIQUE.md` (`f2cac00b…`), the fermento, both settings files, `plan.md`, this file (its rules,
  D-026, R-011, R-018 to D-037), `architecture-decisions.md` whole (`9e45b1bc…`), the summary and observations of
  `harness-verdicts.md`, the phase 6 findings of `reference-study.md`, the case titles and sections A to C of
  `past-defect-cases.md`.
- Decided at the start: D-038 (two parts; the size budget frozen at phase 7's start; the file's place).
- Written: `core-specification.md` sections 0 to 10 (10075 words, sha256 `24c94c05…`): every point of alambique sections 1,
  2 and 6 classified (I, L or A) with its requirements or the reason for its deferral; 101 requirements in 12 areas;
  the record's events; the transition tables of tasks, stages, proposals and the project; 16 commands (AD-6's 14 plus
  `adopt` and `close`); the semantics of `fabrewk check`; 83 refusal codes; the settings keys; the 13 items phase 5
  left open; nine choices for the maestro to approve with the text. A script checked that every code used is in the
  single list, every requirement cited exists, and every requirement is reached from the classification. `plan.md`
  (part 6a done); `STATUS.md`; this entry and D-038.
- Not done: Codex's review, which covers parts 6a and 6b together (D-038).

## D-039 — Codex reviews phase 6 with its full budget

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 13's chat, answering the integrator's question on launching Codex at the end of
  part 6b: "Sí, presupuesto completo (Recomendado)".
- Decision: Codex (`gpt-6.1-sol`, effort high) reviews parts 6a and 6b of `core-specification.md` together, in this
  repository, read-only, with no network, as a full review followed by scoped re-checks until no confirmed blocking
  finding is open (D-003), as in phase 5 (D-030).
- Consequences: the maestro's approval of the text and of its section 10 choices follows Codex's accept (plan row 6).
- Sources: chat of session 13; D-038; plan row 6.
- Supersedes: none.

## R-021 — Session 13, part 6b and Codex's review of phase 6

- Status: recorded
- Date: 2026-11-05
- Who: integrator Claude Code, `claude-opus-5-5`. Reviewer: Codex `codex-cli 0.166.0`, `gpt-6.1-sol`, effort high,
  read-only sandbox, launched in this repository with `codex exec --json`, standard input closed. No subagent, no
  network.
- Start: HEAD `8e3f8c2`, tree clean. The fermento held a new point 1 (the chats' titles), written by the maintainer
  during session 12 and committed inside that session's closing commit `8e3f8c2`, together with the integrator's work
  and its AI co-author: against D-022 and the very defect of PD-41. History is not rewritten; it is recorded here, and
  this session commits the maintainer's edits, if any, alone. The separator's banner line found once; input above it
  unchanged by the integrator (sha256 `bba78baf…` before and after); the answer replaced, recommending to keep point 1
  until the next chat's title shows the fix.
- Read: `STATUS.md`, the fermento, both settings files, `plan.md`, this file (rules, D-030, D-036 to D-038, R-018 to
  R-020), `core-specification.md` whole (`24c94c05…`), `past-defect-cases.md` whole (`b8bacf1c…`), AD-4 and AD-5's
  passages on tests, crates and budget, AD-1's open item, AD-2's protection of authority, AD-5's extension contract,
  AD-8's credential check, alambique 6.9.
- Decided at the start: D-039 (Codex reviews with its full budget).
- Written: `core-specification.md` sections 11 to 13 (part 6b): corrections to part 6a found while writing the tests
  (new REQ-TASK-7, the allowed-files language and lane guard; new codes `MOVE_NOT_ALLOWED` and `TOOL_FAILED`; halts,
  rounds, packets, links, integration guards); the rules every test follows; the 46 past-defect cases mapped to
  requirements and tests; one test per refusal code (C8F-5 and FX-8); whole flows and a crash suite; mutation, size and
  dependencies; an acceptance criterion for each of the 103 requirements; the phase each group is due in. Section 10
  gained choices 10 to 13. A script checked that every requirement has a criterion, every case and code a test, and
  nothing is cited that is not defined; a scan with the tightened credential patterns found no match in
  `.brewstillery/` (only file names read in `xel/`, whose two files are the fermento and settings). 10,075 words to
  22,672 (sha256 `b5884bd7…`). This entry, D-039, `plan.md`, `STATUS.md`, the fermento's answer.
- Codex, pass 1, whole file (sha256 `b56afe15…`): 10 min 2 s; 20 commands, none failed; input 1,191,497 tokens
  (1,081,216 cached), output 15,201 (12,291 reasoning). Verdict reject, 13 blocking findings, all confirmed by the
  integrator: P6-1 (in `published`, the record's own commits moved the branch, so every integration was `BASE_MOVED`);
  P6-2 (a dependent task had no definable base); P6-3 (`approve` needed two commits where one was required); P6-4 (the
  extension criterion contradicted AD-5's read access); P6-5 (credential scan against alambique 6.9); P6-6 (`accept`
  from `halted` contradicted REQ-TASK-5); P6-7 (refusals that write had no stated writes); P6-8 (`assess` would create
  the machine key and caches); P6-9 (a truncation restored exactly passed the history walk); P6-10 (the domain
  criterion forbade the product changes it integrates); P6-11 (MSRV not checked); P6-12 (proposals to `rules` had no
  base or stream); P6-13 (an accepted task with stale evidence and the branch at its candidate had no way out).
- Codex, pass 2, scoped (`6d3f455e…`): 7 min 51 s; 19 commands, none failed; input 1,155,291 (1,052,928 cached),
  output 13,199. Closed ten; reject with P6-5, P6-7, P6-13 open and new P6-14 (`task-integrated` could not precede
  the move) and P6-15 (the specification's own text matched the credential patterns). All confirmed.
- Codex, pass 3, scoped (`1608117d…`): 2 min; 22 commands, 4 non-zero (searches with no match); input 365,599
  (303,488 cached), output 3,694. Closed P6-13 to P6-15; reject with P6-5 (REQ-FER-9 not aligned) and P6-7 (a person's
  edit after the move). Both confirmed.
- Codex, pass 4, scoped (`2b110ae6…`): 57 s; 15 commands, none failed; input 147,387 (109,312 cached), output 2,002.
  Closed P6-5; reject with P6-7 (the snapshot taken before the injected edit). Confirmed.
- Codex, pass 5, scoped (`41857bff…`): 31 s; 4 commands, none failed; input 87,695 (51,712 cached), output 614.
  P6-7 closed, no new finding. **Verdict accept.** The state line was then updated (`b5884bd7…`); nothing else changed.
- Progress measure (D-003): confirmed blocking findings 13, 5, 2, 1, 0. Codex answered in Spanish; prompts, streams
  and last messages stay in the session's scratch directory, not committed.

## D-040 — Phase 6 closed: the core specification approved; start prompts name their phase

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 13's chat, answering the integrator's two questions after Codex's accept (R-021):
  "Aprobar todo (Recomendado)"; and, on the fermento's point 1, "Sí, como regla (Recomendado)".
- Decision:
  1. The maestro approves `core-specification.md` as Codex accepted it (sha256 `b5884bd7…`, its state line then
     updated to `de8834bb…`), with every choice of its section 10 as recommended: `lexopt` as a runtime crate and
     `cargo-deny` as a development tool; review mode, pushing and pull requests after the pilot; a supervisor in a
     policy refused until enforced; the commands `adopt` and `close`; the policy in force carried by `policy-adopted`;
     `.git/info/exclude` as the one write outside the domain in the unpublished modes; `run` signed by whoever runs it;
     no change of the alambique's status keys; `authorize … --run`; the test tooling of section 11.2; the corrections of
     section 11.1 and the phases of section 13; the credential scan's scope, which matches a maintainer's uncited
     material by name only, reconciling AD-8 with alambique 6.9.
  2. The text is frozen. A change to a requirement is a maestro's recorded decision that names the work it affects
     (6.25).
  3. Phase 6 is closed: every point classified, every indispensable point mapped, every past-defect case and refusal
     with a test, every requirement with a criterion, Codex's review with no confirmed finding open.
  4. The start prompt of every handover names the phase or part and what it does, so that each chat's title shows it.
     `AGENTS.md`'s handover rule says so.
- Consequences: phase 7 (core, part 1) follows: its first commit is the skeleton with its line counter, then a maestro
  freezes the size budget (D-038). Whether it splits into parts is decided at its start (D-019). It needs the network
  permission to fetch crates and to install `cargo-deny` and `cargo-mutants`, and `fish`, `dash` and `bwrap` on the
  machine (choice 11). Licence files come with its first code (D-036).
- Sources: chat of session 13; `core-specification.md`; R-021; `.brewstillery/xel/fermento.md`, point 1.
- Supersedes: none.

## D-041 — Phase 7 split into four parts; the workspace, the ignore rule, the musl toolchain and the MSRV

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 14's chat. The start message gave the network permission to fetch crates and
  install `cargo-deny` and `cargo-mutants`, and stated that `fish`, `dash` and `bwrap` are installed and checked
  (choice 11). Answers to the integrator's four questions: "Cuatro partes (Recomendado)"; "Un crate por anillo
  (Recomendado)"; ".gitignore raíz con /target (Recomendado)"; and, on the musl target, "Ya instalé con 'sudo dnf
  install rustup' la versión dnf que ofrecía mi Fedora al día de hoy: 'rustup-0:1.29.0-9.fc44.aarch64'. [...] Revisala
  y si es recomendable que bajes tu propia versión en mi home hazlo y usalo".
- Decision:
  1. Phase 7 is split into four parts (D-019), each in its own chat and each reviewed by Codex, since each delivers
     code. 7a: the skeleton, the line counter, the licence files, `cargo-deny`'s configuration and the maestro's freeze
     of the size budget. 7b: the record (events, replay, compare-and-swap, forks, the history walk) and the crash suite
     on the record. 7c: settings with provenance, identity, roles and signatures. 7d: the transition table, the command
     surface, the message catalogue and `check`. The exact cut between 7b and 7d is reviewed when 7b starts.
  2. The workspace has one crate per ring: `crates/core` (`fabrewk-core`, a library) and `crates/fabrewk` (the binary,
     which only wires the core to the built-in modules) form the core ring; each later module gets its own
     `crates/<name>`; `tools/` holds development tools that never ship, the first being `tools/budget`, the counter of
     REQ-BUD-1. The compiler then keeps the core from depending on a module (AD-5).
  3. Fabrewk's own repository has a root `.gitignore` holding the single line `/target`. It belongs to the product,
     not to the domain; `.brewstillery/.gitignore` (REQ-DOM-4) is unaffected.
  4. Fedora ships no Rust standard library for `aarch64-unknown-linux-musl`. Fedora's `rustup-init` (rustup 1.29.0,
     the installer's version, not Rust's) installed in the maintainer's home, under `~/.rustup` with its own
     `CARGO_HOME` at `~/.rustup/cargo-home` so that Fedora's `cargo` stays first on `PATH`, the toolchain 1.99.1 (the
     same commit as Fedora's `rustc`, `58cf8f9c3`) with the musl target, and the `clippy` and `rustfmt` components.
     Development builds and `cargo test` use Fedora's toolchain; release builds for musl, `cargo fmt` and
     `cargo clippy` use the rustup toolchain, invoked with `RUSTUP_HOME=~/.rustup`, `CARGO_HOME=~/.rustup/cargo-home`
     and that directory's `bin` first on `PATH`.
  5. The MSRV recorded at phase 7's start (AD-4, REQ-BUD-3) is 1.99.1, Fedora's `rustc` on that day.
- Consequences: `plan.md` records the parts. `AGENTS.md` no longer lists code, licence files or the root `.gitignore`
  as not due. The tools on the machine: `cargo-deny` 0.20.2 and `cargo-mutants` 27.1.0 (installed with
  `cargo install --locked`), `fish` 4.6.0, `bubblewrap` 0.12.0, `git` 2.56.0, `dash`.
- Sources: chat of session 14; `plan.md` row 7; D-019, D-038, D-040; AD-4, AD-5; `core-specification.md` REQ-BUD-1
  to REQ-BUD-5, sections 10 (choice 11), 11.2 and 11.6.
- Supersedes: none.

## D-042 — The size budget frozen after measuring the skeleton; the licence files' holder

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 14's chat, answering the integrator's two questions after the skeleton was measured:
  "Congelar techos de AD-5 (Recomendado)"; "The Fabrewk authors (Recomendado)".
- Decision:
  1. The measure, by `tools/budget` (REQ-BUD-1's unit): the core ring (`crates/core`, `crates/fabrewk`) 12 counted
     lines in 2 files; 0 direct runtime crates; 0 crates in the release build for `aarch64-unknown-linux-musl`, Fabrewk's
     own packages not counted; the stripped, statically linked musl release binary 397,600 bytes.
  2. The budget is frozen at AD-5's ceilings: core 4,000 counted lines; each tool adapter, its stream audit included,
     600; each confinement backend 400; the installer and the assessment 1,200; at most 8 direct runtime crates; at
     most 50 crates in the release build; the stripped release binary at most 8 MB, read as 8,000,000 bytes, the
     stricter reading. The numbers live in `tools/budget/src/budget.rs`, whose tests fail when a ring, a crate count or
     the binary passes its limit. Raising any of them is a maestro's recorded decision with its reason (REQ-BUD-2).
  3. `LICENSE-MIT` and `NOTICE` name the holder as "The Fabrewk authors"; individual authorship stays in Git.
- Consequences: features may now be built (D-038). `AGENTS.md` states that a new runtime crate or a change of the
  budget is a maestro's decision.
- Sources: chat of session 14; D-038, D-041; AD-5, AD-9; `core-specification.md` REQ-BUD-1, REQ-BUD-2, REQ-BUD-5.
- Supersedes: none.

## D-043 — The suites run with the rustup toolchain, so the musl binary is the one measured

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 14's chat, after Codex's first review of part 7a found that the binary-size test
  measured the GNU build (P7A-4): "Suite con rustup 1.99.1 (Recomendado)".
- Decision: the suites of record run with the rustup toolchain 1.99.1 (the same version and commit as Fedora's
  `rustc`), whose `cargo` builds for `aarch64-unknown-linux-musl`; the binary-size test builds the musl release binary
  and measures it, the artifact D-042 froze. Run with Fedora's `cargo`, every test passes but that one, which fails
  naming the toolchain it needs; it is never skipped (core-specification 11.2, rule 7). `budget check --binary` keeps
  measuring a binary built beforehand.
- Consequences: the commands to run the suites are in `STATUS.md`. REQ-BUD-3 holds, since the compiler is the
  recorded version 1.99.1.
- Sources: chat of session 14; Codex's first review of part 7a; D-041, D-042; `core-specification.md` REQ-BUD-2,
  REQ-BUD-3, 11.2.
- Supersedes: D-041 only in its point 4, where `cargo test` used Fedora's toolchain.

## D-044 — What the line counter guarantees, and what the review guarantees

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 14's chat, after Codex's third review of part 7a kept three blocking findings, each
  a deliberate way to hide code from the count: "Corregir y fijar alcance (Recomendado)".
- Decision:
  1. The counter of REQ-BUD-1 fails closed on every route by which a file reaches the release binary that Cargo or the
     compiler record: the dep-info beside the binary, the targets of `cargo metadata` (their roots counted; no build
     script and no procedural macro in a ring) and the tree of `cargo tree` (no package from a path outside the rings).
     It also refuses what it can recognise in the source and could pull in code those records would let pass as data:
     the identifier `include` in any form, a `path` attribute, an attribute built by a macro; and a Cargo configuration
     in the repository or `RUSTFLAGS` with a `cfg` where it runs.
  2. A deliberate evasion outside those records and constructs (the machine's own Cargo configuration, a compiler
     defect, code that only a reading of the diff can tell is hidden) is the review's to catch: Codex's review of each
     part and the maestro's reading of the diff. What is known and left open is listed in the part's receipt.
  3. A false positive (a file of data the counter cannot tell from code, such as a data path with an escape or an alias
     of `include_str!`) refuses, and is rewritten in the plain form.
- Consequences: Codex's reviews of the counter are scoped to point 1; a finding outside it is recorded as residual,
  not blocking, unless it breaks a criterion of section 12 another way.
- Sources: chat of session 14; Codex's three reviews of part 7a; `core-specification.md` REQ-BUD-1, REQ-BUD-2, 11.6.
- Supersedes: none.

## D-045 — Part 7a: the metadata component replaced, then one more pass, then a pause

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 14's chat, choosing between the alternatives D-003 requires after two retries
  without progress in Codex's review of part 7a: after confirmed blocking findings 4, 3, 3, 4, "Reemplazar el
  componente (Recomendado)"; after 1, 1, 1, "Corregir y una pasada más (Recomendado)", whose terms were to close part
  7a if the next pass accepted and to pause it otherwise.
- Decision:
  1. The counter's view of the build is one `cargo metadata` with every feature (first filtered to the release
     platform, then, after Codex's sixth pass, unfiltered) in place of `cargo tree` and a members-only metadata, with
     a closed world: every local package a workspace member; every local package the binary reaches a ring package.
  2. The ninth pass found one more finding (P7A-23); part 7a pauses with it corrected and not yet reviewed. The next
     chat resumes 7a with Codex's review of that correction, and closes it if Codex accepts.
- Consequences: `plan.md` and `STATUS.md` record the pause and the next prompt. Part 7a is not accepted yet.
- Sources: chat of session 14; D-003, D-044; Codex's reviews of part 7a (R-022).
- Supersedes: none.

## D-046 — Two alambique additions on a memory of lessons per machine, applied on the maestro's approval

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL. In the fermento, point 1, they proposed a memory of lessons per machine after this session's
  out-of-memory incident, then commented the integrator's five clarifications ("Estoy de acuerdo con todas tus
  recomendaciones", with two nuances: the machine's profile kept from the start, within a quota set by the
  configuration, and the active chat as the one channel to inform and ask) and wrote that they authorized the edit.
  A fermento authorizes nothing by itself (`AGENTS.md`), so the integrator showed the exact text in the chat; the
  maestro answered "Aprobar tal cual (Recomendado)".
- Decision: the integrator applied two sentences to `.brewstillery/ALAMBIQUE.md` (sha256 `f2cac00b…` before,
  `61be0e9f…` after), both with the key [proyectado]: at the end of 6.51, what the memory holds (what an agent could or
  could not do on the machine, its cause, the limit that avoids it and the profile it was observed with, within a size
  limit the configuration sets), who writes it (Fabrewk proposes, the person confirms in the active chat, no agent
  alone), that a lesson only restricts, that it serves every project on that machine and cites its incident, and that
  after the pilots it is evaluated for the core; in 6.38, after the example of Codex, that lessons are applied as limits
  at each launch, which the agent knows without reading the private folder. The profile is kept as each lesson's
  evidence; the machine's configuration is still detected at start (D-015).
- Consequences: the maestro removed fermento point 1 before the session closed (committed alone, `e7f0c3d`); the
  answer has no current recommendation. The core specification is unchanged; adding the
  memory to it, at phase 9 or later, is a maestro's decision naming the work it affects.
- Sources: chat of session 14; `.brewstillery/xel/fermento.md`, point 1; alambique 6.38, 6.51; AD-15; D-015.
- Supersedes: none.

## R-022 — Session 14, phase 7 started: part 7a, nine Codex passes, paused

- Status: recorded
- Date: 2026-11-05
- Who: integrator Claude Code, `claude-opus-5-5`. Reviewer: Codex `codex-cli 0.166.0`, `gpt-6.1-sol`, effort high,
  launched with `codex exec --json`, standard input closed, sandbox `workspace-write` in a disposable clone of each
  commit under the session's scratch directory, inside a user systemd scope capped at 3 GB; no network. No subagent.
- Start: HEAD `606082b`, tree clean. The maestro replaced fermento point 1 during the session (committed alone,
  `68a05ec`) and later commented the integrator's answer (`af84a51`); both with no AI co-author (D-022). The
  separator's banner line found once each time; the input above it unchanged by the integrator.
- Read: `STATUS.md`, `plan.md`, both settings files, the fermento, this file (rules, D-019, D-038 to D-040),
  `core-specification.md` (REQ-DOM-1 to REQ-DOM-7, REQ-CLI-1 to REQ-CLI-4, REQ-BUD-1 to REQ-BUD-5, sections 9, 10, 11.1,
  11.2, 11.6, 12 BUD, 13), `architecture-decisions.md` AD-4, AD-5, AD-9, AD-15, alambique 6.38, 6.51 and its keys.
- Machine (network as permitted): `cargo-deny` 0.20.2 and `cargo-mutants` 27.1.0 by `cargo install --locked`; with
  Fedora's `rustup-init` (rustup 1.29.0), the toolchain 1.99.1 with the `aarch64-unknown-linux-musl` target, `clippy`
  and `rustfmt` under `~/.rustup` (D-041, D-043); the RustSec advisory database fetched by `cargo deny`. `fish` 4.6.0,
  `bubblewrap` 0.12.0, `dash`, `git` 2.56.0 present.
- Incident: the first `cargo mutants` ran four jobs with no limit; a mutant whose lexer index stopped advancing grew
  without bound and the kernel's out-of-memory killer ended VS Code's scope, which held it, with VS Code inside. From
  then on every mutation run went into its own user scope (`systemd-run --user --scope -p MemoryMax=3G -p
  MemorySwapMax=0 -p OOMPolicy=continue`) with `ulimit -v 1000000` and two jobs: with `OOMPolicy` left at `stop` the
  scope ended the run, and with a 4 GB `ulimit` the scope's killer once took `cargo-mutants` itself; 1 GB per process
  holds, the build and the suite fit in it. `/tmp` on this machine is memory (tmpfs): Codex's reproductions left 1.9 GB
  there, removed by the integrator. The lesson is in `STATUS.md` and became the maestro's proposal (D-046).
- Written: the workspace (`Cargo.toml`, `Cargo.lock`, `crates/core`, `crates/fabrewk`, `tools/budget` with the counter,
  dep-info and metadata checks and a minimal JSON reader, all std-only), `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`,
  `deny.toml`, the root `.gitignore`; D-041 to D-046; `plan.md` (phase 7's parts), `AGENTS.md`, `STATUS.md`, the
  fermento's answer, the alambique (D-046). Commits `9ebd135` (skeleton) and `2ff0431` (freeze), then one per round of
  corrections up to `a40c769`.
- Measure at the freeze (D-042): core 12 counted lines in 2 files, 0 direct crates, 0 release crates, the musl binary
  397,600 bytes; unchanged since. Suite: 61 tests, green with the rustup toolchain 1.99.1, in a clean clone and on
  musl; with Fedora's `cargo`, 60 and the musl size test failing as D-043 says. `cargo deny check licenses
  advisories` clean. `clippy -D warnings` and `rustfmt --check` clean.
- Codex, nine passes on the counter (prompt digests in the scratch directory; findings P7A-1 to P7A-23, all confirmed):
  pass 1 (`9ebd135`/`2ff0431`, 241 s, 29 commands): reject, 4 blocking (a test module inside a macro; `#[path]` and
  `include!`; a vendored package taken as own; the size test measuring the GNU binary). Pass 2 (`942bfb2`, 293 s):
  2 closed, 3 blocking (a raw macro name; `include` by alias and a generated path attribute; manifest read as text).
  Pass 3 (`56d711b`, 616 s): 4 closed, 3 blocking and a minor (a library root also named as data; `(proc-macro)` read
  wrongly; a build script emitting `cfg(test)`; raw-string data paths). D-044 fixed the scope. Pass 4 (`e88374e`,
  568 s): 4 blocking (a path holding ` (`; a ring package outside the workspace; a `.cargo` inside a package; an
  optional dependency). Two retries without progress (4, 3, 3, 4): the maestro chose to replace the component
  (D-045). Pass 5 (`caf9eef`, 529 s): 8 closed, 4 blocking (`CARGO_HOME` inside the repository; packages of one name
  and version; host-only dependencies; an unreadable directory). Pass 6 (`b10e6e7`, 275 s): 3 closed, 1 blocking (a
  path from target to host lost in both filtered graphs). Pass 7 (`78334b9`, 326 s): closed, 1 blocking (an `@file`
  in `RUSTFLAGS`). Pass 8 (`dd1b51c`, 294 s): ended by the provider's safety filter, no verdict; repeated as a review
  of coverage (`9e1b94e`, 583 s): closed, 1 blocking (a backslash read as an escape). Two retries without progress
  (1, 1, 1): one more pass, then a pause (D-045). Pass 9 (`fee843c`, 378 s): closed, 1 blocking (a name ending in a
  backslash), corrected in `a40c769` and not reviewed. Inputs per pass 0.9 to 1.6 million tokens, over 90 % cached;
  outputs 8,500 to 20,600.
- Mutation: full runs of the tree at three points (299, 351 and 381 mutants), and after every later round a re-run of
  each function it changed; every viable mutant caught but the two declared equivalents, `token.end - 1` → `/` in
  `scan_source` (no token ends in a newline) and `i < code.len()` → `<=` in `test_module_spans` (`text(code.len())` is
  empty); timeouts are the lexer's infinite loops. A full run of the final tree (492 mutants) was stopped at the
  session's budget after 127: 123 caught, 1 timeout, 2 unviable, 1 missed (the first equivalent). The full run is due
  when part 7a closes.
- Residuals (D-044): the machine's own Cargo configuration and environment beyond what `config_problems` checks, a
  compiler defect, a hiding only the diff shows; a repository whose path holds a space cannot run `budget check`.
- Not done: Codex's review of `a40c769`; the acceptance of part 7a; phase 7's parts 7b to 7d.

## R-023 — Session 15, part 7a closed: Codex accepts at its tenth pass

- Status: recorded
- Date: 2026-11-05
- Who: integrator Claude Code, `claude-opus-5-5`. Reviewer: Codex `codex-cli 0.166.0`, `gpt-6.1-sol`, effort high,
  launched as in R-022 (`codex exec --json`, standard input closed, sandbox `workspace-write` in a disposable clone
  under the session's scratch directory, a user systemd scope capped at 3 GB, no network), its temporary files and
  Cargo home kept beside the clone instead of `/tmp`. No subagent.
- Start: HEAD `af84e04`, tree clean; the code identical to `a40c769`. The fermento was not swept: its answer has no
  current recommendation and the maestro added no input since `e7f0c3d`.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md` (phase 7), this file (D-041 to D-046, R-022), `core-specification.md`
  section 12 BUD; R-022's prompts and launch command; `tools/budget/src/sources.rs`.
- Codex, pass 10 (`af84e04`, the code of `a40c769`; 468 s, 52 commands; input 1.14 million tokens, 94 % cached,
  output 15,300): **accept**. P7A-23 closed: its reproduction against the CLI on a real release build exits 0 at the
  parent of `a40c769` and 70 at `a40c769`, the refusal before any line is read. No new finding within D-044's scope;
  the single rule, the split on whitespace and the colon after the target re-checked. 60 tests passed with Fedora's
  `cargo`, the musl size test failing as D-043 says.
- Residual, not blocking: a file whose name holds a tab is written by the compiler unescaped, and Cargo's aggregate
  dep-info beside the binary omits it, so the counter never sees it. Such a file can reach the binary only as data
  (`include_str!`, `include_bytes!`): every route that would make it Rust (`include` in any form, a `path`
  attribute, an attribute built by a macro, a procedural macro or build script in a ring) is refused in the source,
  and the binary's size still bounds it. It is lost in what Cargo records, so it falls under D-044, point 2. The
  residuals of R-022 stand.
- Mutation, full run of the final tree (`cargo mutants --workspace -j 2 --timeout 90`, rustup 1.99.1, in its own
  scope as R-022 says; 24 min): 492 mutants, 456 caught, 24 timeouts, 10 unviable, 2 missed, the two equivalents R-022
  declares (`token.end - 1` → `/` in `scan_source`, `count.rs:92`; `i < code.len()` → `<=` in `test_module_spans`,
  `count.rs:332`). Every timeout is an index that stops advancing (the lexer, the block-comment, literal and quote
  ends, the test-module walk, the group matchers and two in the JSON reader), a loop the suite would not end; none
  passed.
- Clean clone of `af84e04`, rustup 1.99.1: 61 tests green; the musl release build; `budget check --binary` exits 0
  with core 12 of 4,000 lines in 2 files, 0 direct runtime crates, 0 release crates, the binary 397,600 bytes, the
  measure of D-042 unchanged; `clippy --workspace --all-targets -D warnings` and `fmt --check` clean; `cargo deny
  --offline check licenses advisories` clean.
- Outcome: every acceptance criterion of part 7a in `plan.md` holds (BUD's criteria of section 12 for phase 7, the
  counter's fixture tests, a clean clone and the musl build green, mutants caught or listed as equivalent, Codex's
  review with no confirmed blocking finding open). Part 7a is closed on the terms of D-045, point 2. Ten passes,
  confirmed blocking findings 4, 3, 3, 4, 4, 1, 1, 1, 1, 0; P7A-1 to P7A-23 corrected.
- Written: this entry; `plan.md` (row 7a); `STATUS.md`.
- Not done: phase 7's parts 7b to 7d.

## D-047 — Part 7b: the cut with 7c and 7d, a code for a failed append, network for `proptest`

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 16's chat, answering the integrator's three questions after the cut was shown
  before any code (plan, phase 7): "Aprobar el corte (Recomendado)"; "Nuevo RECORD_HEAD_MOVED (Recomendado)"; "Sí,
  solo crates.io (Recomendado)".
- Decision:
  1. Part 7b delivers the record as a library in `fabrewk-core`, tested at the library's level; the binary gains no
     command. In 7b: REQ-REG-1 (names, streams, identifiers, `RECORD_ID_COLLISION` for stages and tasks); REQ-REG-2
     and REQ-REG-9 (schema 1's envelope validated field by field, with `actor`, `policy` and `instance` supplied by
     the caller, and the closed list of section 3's kinds with each one's actor class, so the rules on `say` and
     signatures apply; PD-03 for the envelope); REQ-REG-3 (PD-04); REQ-REG-4's mechanism (`ssh-keygen -Y` with the
     namespace `fabrewk` against signers the caller gives; the signature before the rename); REQ-REG-5; REQ-REG-6's
     structural codes, the signature and an empty `say`; REQ-REG-7 (PD-05 at the library); REQ-REG-8 (the mark and
     the history walk; PD-01, PD-02); REQ-REG-10 (the record commit, PD-46 for the record; the views are a list of
     files the caller gives); REQ-REG-11; the crash suite on the record with the feature `fault-injection`; PD-10
     for the record (nothing is reported written before its sync); PD-06's place (`RECORD_BROKEN`).
  2. To 7c: REQ-REG-12 (the machine key, the private folder, the `instance` fingerprint) and the second half of
     REQ-REG-4 (signers derived from the cited policy; the role must allow the kind).
  3. To 7d: `RECORD_TRANSITION_INVALID` from the transition table (PD-06's binding, PD-10's `ATTEMPT_ENDED` on replay,
     a hand-made `task-integrated` from `verified`); the payloads of the stage and task kinds, each payload fixed by
     the part that first builds it; the refusal tests of the record's codes through the command line (11.2 rule 4).
  4. A failed compare-and-swap at an append (REQ-REG-5) is refused with a new code, `RECORD_HEAD_MOVED`: nothing
     written, exit 1, next step to run the command again. It is added to REQ-REG-5, to section 7 (now 84 codes) and to
     11.4 of `core-specification.md`.
  5. Network in this session, for Cargo against crates.io only, to fetch `proptest` (choice 11) and what it needs.
- Consequences: `plan.md` records the cut in the rows of 7b, 7c and 7d. `core-specification.md` changes only as point
  4 says; the work it affects is part 7b and the catalogue of 7d.
- Sources: chat of session 16; `plan.md` row 7; D-041; `core-specification.md` REQ-REG-1 to REQ-REG-13, sections 3,
  7, 11.3 to 11.5, 12 REG.
- Supersedes: none.

## R-024 — Session 16, part 7b: the record, accepted by Codex at its third pass

- Status: recorded
- Date: 2026-11-05
- Who: integrator Claude Code, `claude-opus-5-5`. Reviewer: Codex `codex-cli 0.166.0`, `gpt-6.1-sol`, effort high,
  launched with `codex exec --json`, standard input closed, sandbox `workspace-write`, no network, in a user systemd
  scope capped at 3 GB, in a disposable clone. No subagent.
- Start: HEAD `af4cb84`, tree clean. The fermento was not swept: its answer has no current recommendation and the
  maestro added no input since `e7f0c3d`.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, this file (D-041 to D-046, R-022, R-023), `core-specification.md`
  (sections 0, 2 DOM and REG, 3 to 7, 8 to 10, 11.1 to 11.6, 12 REG, 13), `architecture-decisions.md` AD-1 and AD-4's
  passage on crates and tools, `past-defect-cases.md` PD-01 to PD-06.
- Decided before any code: the cut with 7c and 7d, the code `RECORD_HEAD_MOVED`, network for `proptest` (D-047).
  Network used for Cargo against crates.io only: `proptest` 1.12.0 and its dependencies; `serde_json` 1.0.154 and
  `sha2` 0.11.1 and theirs; and the crates `serde_core` names under `cfg(any())` (`serde_derive`, `syn`, `quote`,
  `proc-macro2`, `unicode-ident`), never built but listed by the counter's unfiltered `cargo metadata`.
- Written: `crates/core/src/record/` (canonical JSON with a depth of at most 64, schema 1's envelope, append under
  the lock with a compare-and-swap of the head, atomic writes, replay with every refusal named, the mark, the
  history walk with replacement objects, grafts and the commit graph off, the record commit through a temporary
  index with one rebuild, the guards a write command runs before writing, a fault-injection feature); tests in
  `crates/core/tests/` (`record.rs`, `crash.rs`, fixtures in `common/`) and in the modules. In
  `core-specification.md`: `RECORD_HEAD_MOVED` (D-047), and, as section 3 asks, schema 1's envelope as fixed, the
  codes of each envelope failure, the mark's format and where `MARK_INVALID` and `RECORD_ROLLBACK` part, the commit
  message, the depth limit, and the refusal outside the main working tree. `plan.md`, `STATUS.md`, this entry.
- Interpretations, recorded in section 3 for the maestro and the reviews: a malformed envelope is
  `RECORD_VERSION_UNKNOWN`; a mark naming at a position a digest its stream does not hold there is `MARK_INVALID`
  (PD-02), so in `unpublished` a rewrite of the same length is told so, while where the history is walked it is
  `RECORD_ROLLBACK`; REQ-REG-3's property holds within the depth limit; a write command outside the main working
  tree's root refuses `NOT_A_REPOSITORY`; a lock a stopped Git left in `.git` is named and never removed (PD-11).
- Codex, three passes: pass 1 (`96b800e`, 468 s): reject, 3 blocking and 2 minor (P7B-1 a stopped Git's temporary
  index lock blocked recovery; P7B-2 the writer wrote nesting the reader refuses; P7B-3 a linked working tree took
  another lock and another index; P7B-4 no test of a second creation; P7B-5 an empty `modelObserved` refused). Pass 2
  (`33e8c3a`, 323 s): P7B-1 to P7B-4 closed, P7B-5 without a test, one new blocking (P7B-6, a stopped Git's
  `.git/index.lock` or branch lock blocks recovery), answered by PD-11's design: named, never removed. Pass 3
  (`6a09b02`, 337 s): **accept**; P7B-5 and P7B-6 closed, REQ-REG-10 held not to require removing a lock in `.git`; two
  new minors (P7B-7 `HEAD.lock` not named, P7B-8 a check without a test), corrected in `0ac812e` with tests and their
  mutants caught, not re-reviewed. Confirmed blocking findings by pass: 3, 1, 0.
- Mutation (`cargo mutants -p fabrewk-core`, rustup 1.99.1, `-j 2`, in its own scope as R-022 says, `TMPDIR` under
  `target/`): the first run hit the scratch directory's quota (`/tmp` is memory with a per-user quota) and stopped
  after 265 mutants; the full run of `d7d2024` (430 mutants, 8 min): 370 caught, 19 unviable, 41 missed, 0 timeouts.
  Each missed one was killed by a test or by simpler code, re-run with `fault-injection` so the crash suite judges
  those in `store`, `git` and `fault`; every later change re-run by function (10, 43, 23 and 5 mutants, all caught or
  unviable). Two declared equivalents: `s > 0` → `>=` in `seq_of` (`replay.rs`; a `000000` file is `RECORD_BROKEN`
  either way, as an unknown name or as a gap) and `sync_dir` → `Ok(())` (`store.rs`; a directory's durability across
  a power loss is not observable by killing the process).
- Clean clone of `0ac812e`, rustup 1.99.1: 118 tests green, and the crash suite (3 tests, every write point of three
  write commands in `published`) with `--features fault-injection`; the musl release build; `budget check --binary`
  exits 0: core 1,709 of 4,000 counted lines in 12 files, 2 direct runtime crates (`serde_json`, `sha2`), 21 crates in
  the release build counting every platform, the binary 397,600 bytes (unchanged: the binary does not use the record
  yet); `clippy --workspace --all-targets -D warnings` with and without the feature, `fmt --check`, `cargo deny
  --offline check licenses advisories` clean.
- Size: `rustfmt`'s default width (100) gives the record 1,697 counted lines; a `rustfmt.toml` with `max_width = 120`,
  measured and reverted, gave 212 fewer. Shown to the maestro; nothing changed.
- Deviations from R-022's procedure: mutation's `TMPDIR` and Codex's clone (with a copy of the Cargo registry, 30 MB)
  lived under `target/`, on disk and ignored by Git, instead of the scratch directory, after its quota stopped the
  first run.
- Residuals, not blocking: a Git child still running when the next write command removes the temporary index's lock
  (Fabrewk's lock dies with its process; `read-tree` and `update-index` last milliseconds); durability across a power
  loss is not tested; reading commands verify signatures against an allowed-signers file, and where that file lives
  without a reading command writing anything is 7c's (with the signers derived from the policy).
- Not done: parts 7c and 7d.

## D-048 — Part 7c split into 7c1 and 7c2; signatures verified with the key the signature carries; TOML 1.1

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 17's chat, answering the integrator's four questions before any code: "7c1 y 7c2
  (Recomendado)"; "Clave del .sig + check-novalidate (Recomendado)"; "toml 1.1 sin serde (Recomendado)"; "Sí, solo
  crates.io (Recomendado)".
- Decision:
  1. Part 7c is split in two, each with its own Codex review. 7c1, settings and policy: every key declared once with
     its type, levels, default, direction and description; the project and personal files read with file, key path
     and line in every refusal; the personal keys; the policy in force from the last `policy-adopted`, the file's
     pending changes, the classing of each change and `--widen`; the source of every value; language tags; the
     maintainers' table (REQ-SET-1 to REQ-SET-7, REQ-SET-10, REQ-ID-1, REQ-ID-3's refusal at adoption, REQ-DOM-3).
     7c2, identity and the machine: signers derived from the cited policy and the role checked against the kind
     (REQ-REG-4's second half, REQ-ID-2 to REQ-ID-5), REQ-REG-12 (the machine key, `instance`), REQ-SEC-4, the
     machine file, detection (REQ-SET-8), the validation of profiles (REQ-SET-9), and REQ-DOM-2, REQ-DOM-4, REQ-DOM-5
     and REQ-DOM-7 on fixture records. REQ-SEC-5 moves to 7d: it labels the guards of the transition table and the
     checks of `check`, both 7d's.
  2. A person's signature is verified without any allowed-signers file, so that a reading command writes nothing
     (the residual of R-024): Fabrewk reads the public key the signature itself carries, requires it to be exactly
     the key the cited policy gives the signer's alias, and checks the signature against it with
     `ssh-keygen -Y check-novalidate -n fabrewk`. One route serves reading and writing commands. REQ-REG-4 and its
     criterion in section 12 say so; the derived `local/cache/allowed_signers` (REQ-ID-2) stays, for Git.
  3. Settings are read with the approved crate `toml` (D-036) at 1.1, which reads TOML 1.1, a superset of TOML 1.0
     in which every TOML 1.0 file reads the same; with `default-features = false` and only `std` and `parse`, so no
     `serde` and no procedural macro. REQ-SET-1 and its criterion say TOML 1.1.
  4. Network in this session, for Cargo against crates.io only, to fetch `toml` and what it needs.
- Consequences: `plan.md` shows 7c1 and 7c2 instead of 7c, and SEC-5 in 7d. `core-specification.md` changes REQ-REG-4,
  REQ-SET-1 and their criteria; the work affected is parts 7c1 and 7c2.
- Sources: chat of session 17; `plan.md` row 7; D-036, D-047, R-024's residuals; `core-specification.md` REQ-REG-4,
  REQ-SET, REQ-ID, REQ-DOM, REQ-SEC-4, REQ-SEC-5, sections 8 and 12.
- Supersedes: none (D-047's point 2 stands; this entry splits its destination).

## D-049 — Code formatted at width 120 with `rustfmt`'s "Max" heuristics

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 17's chat, answering the integrator's question after part 7c1's settings brought
  the core to 2,915 of 4,000 counted lines: "Ancho 120 y \"Max\" (Recomendado)".
- Decision: a `rustfmt.toml` at the repository's root holds `max_width = 120` and `use_small_heuristics = "Max"`, and
  the whole workspace is reformatted once with it, in a commit of its own. The size budget (D-042) is unchanged; what
  changes is the formatting its unit is measured on (REQ-BUD-1 counts lines as `rustfmt` leaves them). Measured on the
  same code: the default formatting 2,915 counted lines, "Max" at width 100 2,335, width 120 2,542, both 1,985.
- Consequences: the root holds `rustfmt.toml` beside `deny.toml`, as product tooling. `cargo fmt --check` keeps
  gating every part. R-024's note that the record counts 212 lines more at width 100 is settled by this entry.
- Sources: chat of session 17; D-042; R-024 ("Size"); `tools/budget`.
- Supersedes: none.

## R-025 — Session 17, part 7c1: settings and policy

- Status: recorded
- Date: 2026-11-05
- Who: integrator Claude Code, `claude-opus-5-5`. Reviewer: Codex `codex-cli 0.166.0`, `gpt-6.1-sol`, effort high,
  launched with `codex exec --json`, standard input closed, sandbox `workspace-write`, no network, in a user systemd
  scope capped at 3 GB, in a disposable clone under `target/` with a copy of the Cargo registry (35 MB). No subagent.
- Start: HEAD `3928ef9`, tree clean. The fermento was not swept: its answer has no current recommendation and the
  maestro added no input.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, this file (D-041, D-047, R-024), `core-specification.md` (sections 0, 2
  DOM, REG, SET, ID and SEC, 3 to 9, 11.2, 11.4's rows for 7c, 12 DOM, REG, SET, ID and SEC, 13),
  `architecture-decisions.md` AD-2, AD-3, AD-14 and AD-15, the record's code (`crates/core/src/record/`) and its test
  fixtures; the sources of `toml` 1.1.6 and 1.2.0 (features, `DeTable`, `DeValue`, errors).
- Decided before any code: the split of 7c, signatures verified with the key the signature carries, TOML 1.1, network
  for Cargo (D-048). Network used for Cargo against crates.io only: `toml` 1.2.0 (TOML 1.1; resolved instead of the
  1.1.6 first named, the same API) with `toml_parser` 1.1.5, `winnow` 1.0.5, `serde_spanned` 1.1.2, `toml_datetime`
  1.1.2; `indexmap`, `hashbrown`, `equivalent` and `toml_writer` are in `Cargo.lock` as optional and not built. After
  the first code the maestro decided the formatting (D-049).
- Written: `crates/core/src/settings/` (`keys.rs`, every key of section 8; `mod.rs`, reading with file, key path and
  line; `policy.rs`, the policy in force, pending changes, the effect of a change, adoption and the effective values
  with their sources; `lang.rs`, language tags), `crates/core/src/code.rs` (the codes and the error, moved from the
  record, with ten new codes), `policy-adopted`'s payload fixed and checked (`content`, `digest`, `widened`), `git.rs`'s
  read of a committed file; tests in the modules and in `crates/core/tests/settings.rs`; `rustfmt.toml` (D-049).
  `core-specification.md` (REQ-REG-4, REQ-SET-1 and their criteria, D-048), `plan.md`, `STATUS.md`, this entry.
- Interpretations, given to the review and not re-opened by it: stage and task layers may set `limits.*`, a profile's
  tool, model, effort, checks, required tools and limits, `review.differentProvider` and `gate.protectedPaths`, only
  for tables the policy holds; a change with no order (a name, a key, a list that both gains and loses) widens a key
  that has a direction; no value is the widest (an absent `area` is the whole project, absent `tokens` unlimited); any
  change to a maintainer's table widens (REQ-SET-5 over section 8's "removing a table narrows"); a value that only
  follows another key is not a change of its own and its source is `default`; `gate.protectedPaths`' default holds the
  three literal paths, the files a check names being added where REQ-GATE-9 is enforced (phase 9); `environment.shell`
  defaults to `sh`; the first adoption names no widening (it is `apply`'s); a comment-only edit is pending with no
  change of value; a malformed `policy-adopted` payload is `RECORD_VERSION_UNKNOWN` and a digest that is not its
  content's `RECORD_BROKEN`; a maintainer's key is one line `<type> <base64> [comment]` whose blob is made only of SSH
  strings, the type one of seven `ssh-keygen -Y` signs with; a language tag is checked for its form (RFC 5646), not
  against IANA's registry; text that is not TOML is refused with the key path the parser's own positions give.
- Size: the default formatting gave the core 2,915 of 4,000 counted lines; after D-049, 2,068 at the session's end
  (settings about 830 of them). The commit `28cad57` also carries a formatting-only change to three files of
  `tools/budget`, left by measuring the widths (`rustfmt` is not idempotent across settings); `1547175` reformats them
  again.
- Codex, seven passes, the first whole and the others scoped to the findings and the changed code: pass 1 (`50412e6`):
  reject, 5 blocking and 1 minor (P7C1-1 language tags with repeated variants or singletons accepted, a private-use
  tag and the irregular ones refused; P7C1-2 a maintainer's key spanning lines accepted; P7C1-3 text that is not TOML
  refused without its key path; P7C1-4 `WIDENING_UNNAMED` without lines; P7C1-5 a pending removal without a place;
  P7C1-6 tests missing for TOML 1.1, the stage and task levels, byte-identical files and the modes). Pass 2
  (`0e704d7`): P7C1-1, 2, 4, 5 closed; P7C1-3 open (a locator guessing from the text); P7C1-6 partly; new P7C1-7
  (`ssh-rsa` refused). Pass 3 (`1163514`): P7C1-6 and 7 closed, P7C1-3 open. Pass 4 (`702146f`): P7C1-3 open, new
  P7C1-8 (a regression of the locator). The design changed then: the parser itself resolves the path, the text at the
  error replaced by a marker key or a valid value and read again. Pass 5 (`9dbda25`): P7C1-3 and 8 closed, new P7C1-9
  (a key of the file named as the marker) and P7C1-10 minor (an inserted `0` in the path). Pass 6 (`b3afc79`): P7C1-10
  closed, P7C1-9 open (the marker spelled with escapes). Pass 7 (`7511d13`): **accept**, no new finding. Confirmed
  blocking findings by pass: 5, 2, 1, 2, 1, 1, 0. Every pass ran in 5 to 11 minutes.
- Found by the integrator before the review: `ROLE_NOT_ENFORCED` raised by any value `"supervisor"`, not only a role;
  by the mutation run: a key that narrows treated its absence as its narrowest.
- Mutation (`cargo mutants`, rustup 1.99.1, `-j 2`, in its own scope as R-022 says, `TMPDIR` under `target/`): the
  settings and `policy_payload`, 262 mutants: 229 caught, 17 unviable, 12 missed, 4 timeouts (loops in `is_tag`). Each
  missed one killed by a test or by simpler code and re-run by function after each correction (125, 76, 7, 4, 3, 8 and
  10 mutants), all caught but these declared equivalents: `>` → `>=` in `table_of` (a path as long as a key's is a
  leaf, read before), and `>` → `>=` three times in `effect` (both callers skip equal values).
- Clean clone of `7511d13`, rustup 1.99.1: 144 tests green, and the crash suite (3 tests) with `--features
  fault-injection`; the musl release build; `budget check --binary` exits 0: core 2,102 of 4,000 counted lines in 17
  files, 3 direct runtime crates (`serde_json`, `sha2`, `toml`), 30 crates in the release build counting every
  platform, the binary 397,600 bytes (the binary does not use the core yet); `clippy --workspace --all-targets -D
  warnings` with and without the feature, `fmt --check`, `cargo deny --offline check licenses advisories` clean. A
  first run with `CARGO_TARGET_DIR` set failed one counter test, which reads its fixture's own `target/`; not a defect.
- Residuals, not blocking: the source of a value that follows another key reads `default`, and `config` (7d) should
  say which key it follows; a language tag's subtags are not checked against IANA's registry.
- Not done: parts 7c2 and 7d.

## D-050 — Part 7c2 split into 7c2a and 7c2b; the policy an event cites; a seal of every head at adoption

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 18's chat, answering the integrator's three questions before any code: "7c2a y
  7c2b (Recomendado)"; "La de la 1.ª adopción (Recomendado)"; "Sello en la adopción (Recomendado)". The first was
  asked because the maintainer's five-hour window stood at 79 % at the session's start.
- Decision:
  1. Part 7c2 is split in two, each with its own Codex review. 7c2a, signatures and identity: signers derived from the
     cited policy and the role checked against the kind (REQ-REG-4's second half, REQ-ID-2 to REQ-ID-5), the private
     folder (REQ-SEC-4), the machine key and `instance` (REQ-REG-12). 7c2b, the machine and the domain: the machine
     file, detection (REQ-SET-8), the validation of profiles (REQ-SET-9), and REQ-DOM-2, REQ-DOM-4, REQ-DOM-5 and
     REQ-DOM-7 on fixture records.
  2. The events `apply` writes before the first `policy-adopted` (`assessed`, `publication-decided`) cite the digest of
     the policy that first adoption adopts; the first `policy-adopted` cites its own content's digest (trust on first
     use, REQ-ID-6); every later `policy-adopted` cites the policy in force before it; every other event of the
     `project` stream cites the policy in force at its place in that stream.
  3. Every `policy-adopted` records, besides `content`, `digest` and `widened`, a seal: `heads`, the position and full
     digest of the head of every other stream at the adoption. A person's event of another stream at position `k`
     was written after an adoption whose seal holds its stream below `k` (or not at all); the last such adoption is
     the policy in force at its place. The signer must hold the key and the role the event claims both in the policy
     the event cites and in the policy in force at its place, so that removing a maintainer, or changing their key or
     role, takes effect for every event written after the adoption, also one forged by hand, while the concurrent work
     of a maintainer who keeps their key and role is accepted. A seal naming a position its stream does not reach, or
     a digest its stream does not hold there, is a rollback.
- Consequences: `plan.md` shows 7c2a and 7c2b instead of 7c2. `core-specification.md` records the citation rule and
  the seal in section 3 and in REQ-REG-4; `policy-adopted`'s payload, fixed by 7c1, gains `heads` (no record exists
  yet in any repository). The work affected is parts 7c2a and 7d.
- Sources: chat of session 18; `STATUS.md`'s two open items for 7c2; D-047, D-048; AD-14 ("Changing the list");
  `core-specification.md` REQ-REG-4, REQ-ID-3, REQ-ID-4, REQ-ID-6, sections 3 and 4.
- Supersedes: none (D-048's point 1 stands; this entry splits 7c2's destination).

## R-026 — Session 18, part 7c2a: signatures and identity

- Status: recorded
- Date: 2026-11-05
- Who: integrator Claude Code, `claude-opus-5-5`. Reviewer: Codex `codex-cli 0.166.0`, `gpt-6.1-sol`, effort high,
  launched with `codex exec --json`, standard input closed, sandbox `workspace-write`, no network, in a user systemd
  scope capped at 3 GB, in a disposable clone under `target/` with a copy of the Cargo registry (35 MB). No subagent.
- Start: HEAD `3fa8e27`, tree clean. The fermento was not swept: its answer has no current recommendation and the
  maestro added no input. The maintainer's five-hour window stood at 79 % at the start.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, this file (D-047 to D-049, R-024, R-025), `core-specification.md`
  (sections 2 DOM, REG, SET, ID and SEC, 3 to 8, 11.2, 11.4's and 11.5's rows for 7c2, 12 DOM, REG, SET, ID and SEC),
  `architecture-decisions.md` AD-8, AD-14 and AD-15, the record's and the settings' code and their test fixtures.
- Decided before any code (D-050): 7c2 split into 7c2a and 7c2b; the policy the events before the first adoption
  cite; the seal of every head at adoption. No network was needed.
- Written: `crates/core/src/identity.rs` (the policy verifier, `ROLE_FORBIDDEN` before signing, the derived
  `allowed_signers`, the key report), `crates/core/src/machine.rs` (the private folder, `PRIVATE_FOLDER_PERMISSIONS`,
  the machine key and its fingerprint); in the record, `sign.rs` (the key a signature carries and `check-novalidate`,
  D-048; `SshVerifier` and its allowed-signers file removed), replay verifying signatures after the record is read
  field by field, `append` checking its signature against the signer's own key before the event exists, `seal`;
  `policy-adopted`'s payload with `heads`; `settings::key_blob`; two codes; tests in `crates/core/tests/identity.rs`
  and a fixture verifier with fixed keys for the record's own tests. `core-specification.md` (REQ-REG-4, section 3's
  table and envelope, 11.4's `RECORD_SIGNATURE_INVALID`, 12's REQ-REG-4), `plan.md`, `STATUS.md`, this entry.
- Interpretations, given to the review and not re-opened by it: every failure of a person's event against its policies
  is `RECORD_SIGNATURE_INVALID` at replay, a contributor's move included (REQ-REG-4 groups them), while a write refuses
  the same move as `ROLE_FORBIDDEN`; a seal its stream does not hold is `RECORD_ROLLBACK` (the seal is a signed mark);
  an event outside `project` that every seal holds was written before any adoption and is refused; the seal lists
  only streams that hold an event; `allowed_signers` lines are `<alias> namespaces="fabrewk,git" <type> <base64>`, in
  alias order; touch enforcement is always reported `unknown`, since `check-novalidate` reads no options; the machine
  key is `instance` in the private folder, its identity the private key (the public half derived with `ssh-keygen
  -y`), made under the folder's `lock`; the folder and any parent it makes are `0700`; a private key that does not
  read is none, and a write command makes another; the `policy` field of an event no person records is not checked
  here (7d's `check`).
- Codex, two passes: pass 1 (`7738425`): reject, 1 blocking and 2 minor (P7C2A-1 two commands in two repositories
  making the machine key at once returned different fingerprints; P7C2A-2 an interrupted recovery could leave a
  public key that is not the private key's; P7C2A-3 no test of a maestro made contributor nor of an event every seal
  holds, and the specification's rows without D-050's cases). Pass 2 (`7985b05`): **accept**, the three closed, no
  new finding (it re-ran 40 threads and 72 processes). Confirmed blocking findings by pass: 1, 0. At pass 2 Codex
  reported reading by mistake one `SKILL.md` outside its clone; it read nothing of the private folder.
- Mutation (`cargo mutants`, rustup 1.99.1, `-j 2`, in its own scope as R-022 says, `TMPDIR` under `target/`):
  identity, machine and sign, 101 mutants, and the changed functions of the record and the settings, 40: four
  missed, killed by a test (`carried_key`'s namespace) or by simpler code (redundant checks after `ssh-keygen`, the
  folder made recursively); every later change re-run by file (29 and 81 mutants). One declared equivalent: `|` → `^`
  in `fingerprint` (the bits joined never overlap).
- Clean clone of `7985b05`, rustup 1.99.1: 159 tests green, and the crash suite (3 tests) with `--features
  fault-injection`; the musl release build; `budget check --binary` exits 0: core 2,390 of 4,000 counted lines in 19
  files, 3 direct runtime crates, 30 crates in the release build, the binary 397,600 bytes (the binary does not use
  the core yet); `clippy --workspace --all-targets -D warnings` with and without the feature, `fmt --check`, `cargo
  deny --offline check licenses advisories` clean.
- Residuals, not blocking: a maintainer removed by an adoption that is itself cut from the record (in `unpublished`, on
  a machine with no mark) is caught only by the rollback checks, cooperative on one machine (C4); the machine key's
  identity changes if its private file is damaged by hand.
- Not done: parts 7c2b and 7d.

## D-051 — Part 7c2b: an invalid profile is `SETTINGS_INVALID`; detection through a contract the modules fulfil; the scope

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 19's chat, answering the integrator's three questions before any code:
  "SETTINGS_INVALID (Recomendado)"; "Contrato del núcleo (Recomendado)"; "Aprobar (Recomendado)".
- Decision:
  1. A profile that is invalid for any reason other than an absent tool (its tool not in the catalogue, its model not
     in the catalogue or of a provider its tool does not use, that provider not permitted, its effort not one of the
     model's) is refused `SETTINGS_INVALID`, naming the profile's key path and the file and line, or the stage or task
     that set the value: a value outside the allowed set (REQ-SET-2). A permitted tool or a required tool that is
     absent is `TOOL_MISSING`. No new code.
  2. The core learns a program's executable and how to read its version through a contract it declares and every
     built-in module fulfils (a tool adapter, a confinement backend): the executable's name and the arguments that
     print its version without calling any model. `git` and `ssh-keygen` are the core's own (AD-4). A
     `tools.<name>.path` in `machine.toml` takes the place of the search of `PATH` and is shown as declared. In 7c2b
     the contract is tested with fake modules; the real ones come with phase 9.
  3. The scope of 7c2b, as a library: `machine.toml` read from the private folder, its values with the source
     `declared`; detection in memory at every start; `local/cache/detected.json` written only by write commands, where
     it keeps "verified by use" with its date and never gives presence; the validation of profiles; the domain's layout
     (`DOMAIN_UNKNOWN_ENTRY`, `ALIAS_INVALID`); the exact `.gitignore` (`GITIGNORE_CHANGED`); the cache rebuilt by write
     commands; `publication-decided`'s payload (`mode`, `warningShown`); `DOMAIN_NOT_IGNORED` with `git check-ignore`.
     Deferred to phase 9: the permission mode, the observed model and effort, `local/evidence/` and `local/tmp/`; to
     phase 10: the receipt and `apply`'s use of the exclude line. `bootstrap/`, and `review/` outside review mode, are
     reported `DOMAIN_UNKNOWN_ENTRY`, not `ALIAS_INVALID`, so that `check` does not fail in this repository.
- Consequences: `core-specification.md` records the interpretations of point 3 and the payload of
  `publication-decided` in section 3, and REQ-SET-9's code; the work affected is part 7c2b, and phase 9's adapters
  and backend, which fulfil the contract.
- Sources: chat of session 19; `STATUS.md`'s two open items for 7c2b; D-050; AD-2 ("Present, apart from permitted"),
  AD-4 ("External tools the core calls"), AD-5 (the rings), AD-15; `core-specification.md` REQ-SET-8, REQ-SET-9,
  REQ-DOM-2, REQ-DOM-4, REQ-DOM-5, REQ-DOM-7, sections 6, 7, 11.4 and 12.
- Supersedes: none.

## D-052 — Part 7c2b: whether a file is executable is asked of `sh`'s `test -x`

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 19's chat, answering the integrator's question on Codex's first-pass finding
  P7C2B-3 (any execute bit taken as executable, a file of mode `0601` reported present): "sh -c 'test -x'
  (Recomendado)".
- Decision: detection takes a file as executable only when it is a regular file with an execute bit and `/bin/sh -c
  'test -x "$1"'`, run with no environment, says the current process may execute it: the operating system's own answer
  (effective user and group, supplementary groups, access lists, mounts without execution). `sh` becomes the third
  external program the core calls, after `git` and `ssh-keygen` (AD-4); it is run only for a file that passed the
  check of its bits. No crate is added and no `unsafe` code is allowed.
- Consequences: section 8's paragraph on detection says so; AD-4's list of external tools is read with this entry.
- Sources: chat of session 19; Codex's first pass of 7c2b (P7C2B-3); AD-4 ("External tools the core calls"); D-036;
  the workspace's `unsafe_code = "forbid"`; D-051.
- Supersedes: none.

## R-027 — Session 19, part 7c2b: the machine and the domain

- Status: recorded
- Date: 2026-11-05
- Who: integrator Claude Code, `claude-opus-5-5`. Reviewer: Codex `codex-cli 0.166.0`, `gpt-6.1-sol`, effort high,
  launched with `codex exec --json`, standard input closed, sandbox `workspace-write`, no network, in a user systemd
  scope capped at 3 GB, in a disposable clone under `target/` with a copy of the Cargo registry (35 MB). No subagent.
- Start: HEAD `390ae12`, tree clean. The fermento was not swept: its answer has no current recommendation and the
  maestro added no input.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, this file (D-047 to D-050, R-024 to R-026), `core-specification.md`
  (sections 2 DOM, SET, ID, RUN-1 and RUN-2, INS, SEC and EXT, 3, 6 to 9, 11.4's rows for 7c2b, 12 DOM, REG and SET,
  13), `architecture-decisions.md` AD-2 ("Present, apart from permitted", the catalogue), AD-4 (external tools), AD-5
  (the rings) and AD-15, the code of the record, the settings, identity and the machine, and their test fixtures.
- Decided before any code (D-051): an invalid profile is `SETTINGS_INVALID`; the contract the modules fulfil for
  detection; the scope and what is deferred. After the first review (D-052): `sh`'s `test -x`. No network was needed.
- Written: `crates/core/src/detect.rs` (the contract `Program`, the core's programs, detection in three spaces, the
  cache `local/cache/detected.json` and "verified by use", the effective tools, `check_profile`),
  `crates/core/src/domain.rs` (the layout, `.gitignore`, the publication mode, `check_ignored`, `exclude_domain`),
  `machine_file` in `machine.rs`, `publication-decided`'s payload in `event.rs`, `ignores` and `exclude_file` in
  `git.rs`, `Type::Path`, `Source::Declared` and `effective`'s machine file in the settings, three codes; tests in
  `crates/core/tests/machine.rs` and `crates/core/tests/domain.rs`, fixtures adjusted for the payload.
  `core-specification.md` (REQ-SET-9's code; the paragraphs "The domain, as part 7c2b fixed it" in section 2 and
  "Detection, as part 7c2b fixed it" in section 8; `publication-decided`'s payload in section 3; 11.4's
  `ALIAS_INVALID`), `plan.md`, `STATUS.md`, this entry.
- Interpretations, given to the review and not re-opened by it: the core's `git` and `ssh-keygen` are found in `PATH`
  only, and `ssh-keygen` has no version; a required tool is looked for by its own name; a declared path is absolute
  and replaces the search; a version is the first line of standard output when its arguments exit 0; the cache
  carries only "verified by use", kept while path and version are equal, and a cache that does not read is none; the
  effective set is the catalogued tools present; a listed name of the wrong kind, and any link, at the domain's top is
  an unknown entry; `check_ignored` first requires the domain's parent to be the root of the product's main working
  tree; `warningShown` is not tied to the mode here (`apply`'s, phase 10).
- Found by the integrator: the fixtures under `target/` lie inside this repository, which ignores them, so a domain
  outside any repository read as ignored; corrected before the review by requiring the product's root. During the
  first review, the shared space of programs and tools (Codex found it too, P7C2B-1).
- Codex, two passes: pass 1 (`21f068b`): reject, 4 blocking (P7C2B-1 a core program found as a catalogued tool of
  its name; P7C2B-2 a catalogued tool hiding a required tool of its name; P7C2B-3 any execute bit taken as
  executable, a file of mode `0601` present; P7C2B-4 two paths that are not UTF-8 read the same and shared a
  verification). Pass 2 (`82f6980`): **accept**, the four closed, no new finding. Confirmed blocking findings by
  pass: 4, 0. Codex ran no command outside its clone.
- Mutation (`cargo mutants`, rustup 1.99.1, `-j 2`, in its own scope as R-022 says, `TMPDIR` under `target/`): the
  new code and the changed functions, 122 mutants: 8 missed, each killed by a test or by simpler code (a version
  that fails printing something, a name that is not UTF-8, the mode `published`, a missing or unreadable exclude
  file) and re-run (44 mutants, all caught or unviable); `detect.rs` after the first review, 54 mutants: caught but
  two declared equivalents, `&` → `|` and `&` → `^` in `is_executable` (the bits' pre-check only spares a process:
  `test -x` refuses a file with no execute bit either way).
- Clean clone of `82f6980`, rustup 1.99.1: 172 tests green, and the crash suite (3 tests) with `--features
  fault-injection`; the musl release build; `budget check --binary` exits 0: core 2,751 of 4,000 counted lines in 21
  files, 3 direct runtime crates, 30 crates in the release build, the binary 397,600 bytes (the binary does not use
  the core yet); `clippy --workspace --all-targets -D warnings` with and without the feature, `fmt --check`, `cargo
  deny --offline check licenses advisories` clean.
- Residuals, not blocking: a program whose version arguments never end blocks detection (no time limit yet; the
  adapters of phase 9 should bound it); "verified by use" in the cache is cooperative, since the same user can edit
  it; a test writes a fake program and runs it at once, which a fork in another test thread could make fail with
  `ETXTBSY` (not seen).
- Size: the core grew from 2,390 to 2,751 counted lines. 7d (the transition table, the command surface, the
  catalogue, `check`), phase 8 (the fermentos and views) and the core's part of phase 9 (RUN, GATE) remain inside the
  same 4,000.
- Not done: part 7d.

## D-053 — Part 7d split into 7d1, 7d2 and 7d3; the budget decided after measuring 7d1; commands declared by phase; `run` up to confinement

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 20's chat, answering the integrator's four questions before any code, after the
  integrator showed the cut and an estimate of 7d's size by component (about 2,300 to 2,400 counted lines against
  the 1,249 left of the core's 4,000; the whole specification of phases 7 to 9, at the density measured in 7b to
  7c2b, about 7,000 to 8,000): "7d1, 7d2 y 7d3 (Recomendado)"; "Decidir tras medir 7d1 (Recomendado)"; "Crece por
  fase (Recomendado)"; "Guardias hasta aislamiento (Recomendado)".
- Decision:
  1. Part 7d is split in three, each with its own Codex review. 7d1, the transition table and the payloads, as a
     library: section 4's table with every state, move, actor, role, guard, refusal and control label (REQ-TASK-1,
     REQ-SEC-5's labels); the replay validated by it (`RECORD_TRANSITION_INVALID`, `ATTEMPT_ENDED` on replay, PD-06's
     binding, PD-31's replay half); the stage and task payloads (REQ-TASK-2, REQ-TASK-3); the allowed-file patterns
     and their guard (REQ-TASK-7, PD-15, PD-17, `PROTECTED_PATH`, `PROTECTED_PATH_WILDCARD`); `CHECK_DUPLICATE`
     (PD-24); `LIMIT_UNOBSERVABLE` through the modules' contract, with fake modules; the credential scan as a function
     (REQ-SEC-3 at `check`). 7d2, the command surface and the reading commands: the declaration of commands (with
     `lexopt`, D-040), the parser, help, the fish and POSIX completions, the reference documentation, the catalogue in
     `en` and `es`, `--json`, the exit statuses, `status`, `config` and `check` (C1 to C4 and C8 to C11; C5, C6, C7
     and C12 come with phase 8, as section 11.4 already assigns their codes). 7d3, the write commands: `adopt`,
     `propose`, `authorize` (PD-37 for `authorize`), `reject`, `run` as point 4 says, `--say` (REQ-TASK-4, PD-30 to
     PD-32), `AGENT_ENVIRONMENT`, the guards every write command applies, and the record's refusal tests through the
     command line (D-047).
  2. The core's ceiling is decided when 7d1 is measured, before 7d2 starts: 7d1 fits in what is left, and its closing
     brings the measured figure and a corrected projection for 7d2, 7d3, phase 8 and the core's part of phase 9.
  3. The declaration of commands grows phase by phase: help, completions and the reference documentation list only
     the commands that exist. The tests of `MOVE_NOT_ALLOWED` that use a later phase's command move to that phase:
     `accept` on a `running` task to phase 9; `approve` of a discarded proposal and `close` of a closed stage to
     phase 8. `run` on a `proposed` task and `authorize` on an `integrated` one stay in phase 7.
  4. Until phase 9 builds a confinement backend, `run` evaluates its guards in their listed order up to confinement,
     which refuses `CONFINEMENT_UNAVAILABLE` (REQ-SEC-2) with nothing written; so the tests of `DEPENDENCY_OPEN` and
     of `LOCK_HELD` through `run` stay in phase 7.
- Consequences: `plan.md` shows 7d1, 7d2 and 7d3 instead of 7d. `core-specification.md` section 11.4 assigns
  `MOVE_NOT_ALLOWED`'s test to phases 7, 8 and 9 as point 3 says. The size budget (D-042) is unchanged until the
  maestro decides it after 7d1.
- Sources: chat of session 20; `STATUS.md`'s "Next"; D-042, D-044, D-047, D-048; `plan.md` row 7;
  `core-specification.md` sections 2 TASK, CLI, VIEW, SEC, 4 to 7, 11.2, 11.4, 12 TASK, VIEW, CLI, SEC, 13;
  `tools/budget/src/budget.rs`.
- Supersedes: none.

## D-054 — The core's ceiling raised to 7,500 counted lines after measuring part 7d1

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 20's chat, answering the integrator's question after part 7d1 was accepted and
  measured, as D-053's point 2 asked: "7,500 ya (Recomendado)".
- Decision: the core ring's ceiling in `tools/budget/src/budget.rs` is 7,500 counted lines instead of 4,000. Reason:
  part 7d1 measured 548 counted lines against an estimate of about 750 (a factor of 0.73); with that factor and the
  estimates as given, the core reaches about 3,950 to 4,200 lines after 7d2, 4,500 to 4,950 after 7d3, 5,300 to 6,050
  after phase 8 and 6,300 to 7,450 after the core's part of phase 9, so the frozen specification does not fit in
  4,000 and does in 7,500. The other ceilings, the crate limits and the binary's size are unchanged (D-042). Each
  part is still measured, and the projection corrected at each closing.
- Consequences: `budget.rs` and its two tests that pin the ceiling; REQ-BUD-2 in `core-specification.md` names this
  entry. The alambique's "núcleo pequeño" (6.2) is served by the ceiling and the measure, not by a number it states.
- Sources: chat of session 20; D-042, D-049, D-053; R-028 ("Size"); `tools/budget`.
- Supersedes: D-042 for the core's ceiling only.

## R-028 — Session 20, part 7d1: the transition table, the payloads, the pattern guards and the credential scan

- Status: recorded
- Date: 2026-11-05
- Who: integrator Claude Code, `claude-opus-5-5`. Reviewer: Codex `codex-cli 0.166.0`, `gpt-6.1-sol`, effort high,
  launched with `codex exec --json`, standard input closed, sandbox `workspace-write`, no network, in a user systemd
  scope capped at 3 GB, in a disposable clone under `target/review-7d1/` with a copy of the Cargo registry (35 MB).
  No subagent.
- Start: HEAD `8e7c854`, tree clean. The fermento was not swept: its answer has no current recommendation and the
  maestro added no input.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, this file (D-047 to D-052, R-024 to R-027), `core-specification.md`
  (sections 2 TASK, VIEW, CLI, SEC, GATE-1, GATE-8, GATE-9, REG-6, RUN-1, RUN-2, RUN-5, RUN-7, RUN-9, DOM-7; 3 to 9;
  11.2 to 11.4; 12 TASK, VIEW, CLI, SEC, BUD; 13), `architecture-decisions.md` on effective and cooperative controls,
  the code of the record, settings, identity and detection, and their test fixtures.
- Decided before any code (D-053): 7d split into 7d1, 7d2 and 7d3; the ceiling decided after 7d1 is measured; the
  declaration of commands grows by phase; `run` up to confinement. After the review (D-054): the core's ceiling.
- Written: `crates/core/src/table.rs` (section 4 as one declarative table with the stage and proposal moves, the
  control labels, guards in order, `MOVE_NOT_ALLOWED`, the replay held to it), `crates/core/src/task.rs` (the four
  payloads, shape and content, the pattern language and matcher, the lane guard, `PROTECTED_PATH_WILDCARD`,
  `CHECK_DUPLICATE`, `LIMIT_UNOBSERVABLE`), `crates/core/src/scan.rs` (REQ-SEC-3 and the domain's walk), a code's
  name derived from its variant and nine codes in `code.rs`, `subject_digest` and the payload's shape in
  `event.rs`, `tokens` in the modules' contract (`detect.rs`); tests in `crates/core/tests/table.rs`, fixtures given
  valid stage and task payloads. `core-specification.md` (the payloads and the replay's rules in section 3, the table
  and its labels in section 4, REQ-SEC-3's interpretations, `MOVE_NOT_ALLOWED`'s phases, REQ-BUD-2), `plan.md`,
  `STATUS.md`, `budget.rs` and its tests (D-054), this entry.
- Interpretations, given to the review and not re-opened by it: a task's limits are fields of its payload and a
  layer of settings beside them; a stage's authorization binds its own digest and head as a task's does, and makes
  each task it binds authorized after the head it names; an invalid payload's shape is `RECORD_VERSION_UNKNOWN` and
  wins, its content is `TASK_INVALID` at a command and `RECORD_TRANSITION_INVALID` on replay; the pattern guards,
  `STAGE_CLOSED`, `CHECK_DUPLICATE` and `LIMIT_UNOBSERVABLE` are the commands', not the replay's; the token patterns
  of REQ-SEC-3 have no boundary before the prefix (none matched this repository's domain without one); the labels
  of section 4's guards (cooperative: a move that grants nothing, `WORDS_COPIED`, `LOCK_HELD`, REQ-RUN-2,
  `CREDENTIAL_SUSPECTED`).
- Codex, three passes: pass 1 (`daef066`): reject, 3 blocking (P7D1-1 an attempt replaced while open lost its end;
  P7D1-2 a wildcard matching a protected directory's own entry kept; P7D1-3 a domain that is a link followed) and 1
  not blocking (P7D1-4 a credential's name hid an unreadable file). Pass 2 (`1d18b64`): reject, 1 blocking (P7D1-5
  a final `/` or `/.` resolved the link P7D1-3 refuses); the other three closed. Pass 3 (`4724084`): **accept**, both
  closed; 1 not blocking left as a residual (P7D1-6). Confirmed blocking findings by pass: 3, 1, 0. Each pass checked
  that the new tests fail without their correction. Codex ran no command outside its clone.
- Mutation (`cargo mutants`, rustup 1.99.1, `-j 2`, in its own scope as R-022 says, `TMPDIR` under `target/`): the
  four new or reworked files whole, 225 mutants (57 in `table.rs`, 125 in `task.rs`, 33 in `scan.rs`, 10 in
  `code.rs`): 8 missed, each killed by a new test and re-run (9 mutants, all caught); the changed lines of the other
  files and of each correction (16, 14 and 5 mutants): all caught or unviable. No equivalent declared. The first run
  stopped by itself after its last file's new mutants (exit 101, 2.7 GB peak in its 3 GB scope); the rest of
  `event.rs` is earlier code, mutated in part 7b.
- Clean clone of `4724084`, rustup 1.99.1: 197 tests green, and the crash suite (3 tests) with `--features
  fault-injection`; the musl release build; `budget check --binary` exits 0: core 3,299 of 4,000 counted lines in 24
  files, 3 direct runtime crates, 30 crates in the release build, the binary 397,600 bytes; `clippy --workspace
  --all-targets -D warnings` with and without the feature, `fmt --check`, `cargo deny --offline check licenses
  advisories` clean. Then D-054 raised the ceiling to 7,500; the budget's suite passes with it.
- Residuals, not blocking: `scan::domain` refuses a domain given as `.` (`CHECK_UNKNOWN`, conservative; `check` passes
  the domain's named path, P7D1-6); `gate.protectedPaths`'s default does not yet add "every file a check's command
  names" (section 8), and a protected directory must be written with its final `/`; PD-09 has an attempt ended in
  `authorized` and `halted` while section 4 lets `attempt-ended` move only from `running`, which phase 9 must
  reconcile; for 7d3: section 4's `run-started` takes no `--say` while schema 1 gives every person's move words, and
  how `TASK_CHANGED` arises when a task's fields cannot change in the record (the binding is held on replay).
- Size: the core grew from 2,751 to 3,299 counted lines (+548; estimated about 750). Projection in D-054.
- Not done: parts 7d2 and 7d3.

## D-055 — Part 7d2: the person is Git's signing key; citations are Markdown links; completions are generated files; the scope

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 21's chat, answering the integrator's four questions before any code: "Llave de
  firma de Git (Recomendado)"; "Solo enlaces Markdown (Recomendado)"; "Archivos generados (Recomendado)"; "Aprobar
  (Recomendado)". In the same chat, before the questions, the maestro gave network for Cargo to fetch `lexopt` from
  crates.io.
- Decision:
  1. A command knows who runs it by Git's signing key: the person is the maintainer of the policy in force whose `key`
     is the public key that `user.signingkey` names in that repository, as `git config` gives it (a `key::` literal,
     a public key file, or a private key file with its `.pub` beside it). That maintainer's personal file gives the
     interface language. When no maintainer matches, or no policy is adopted, the messages are in the policy's
     canonical documentation language, else `en`. Part 7d3 signs with the same key.
  2. What the fermento or the alambique cites (REQ-SEC-3 at `check`) is the target of every Markdown inline link
     `[text](target)` and image `![text](target)` in `ALAMBIQUE.md` and in each `<alias>/fermento.md` that is a
     relative path, resolved from the citing file's directory; a target with a scheme (`https:`, `mailto:`), an
     absolute path or only a fragment cites nothing, and a fragment or a query after the path is dropped. Only a cited
     file inside a maintainer's directory changes the scan.
  3. The fish and POSIX completions and the reference documentation are files generated from the declaration of
     commands, kept in `crates/fabrewk/`, and a test fails when one differs from a fresh generation; no command is
     added. The POSIX file is POSIX `sh` syntax that registers a completion with bash's `complete` (and zsh's through
     `bashcompinit`), and passes `dash -n`. Phase 10's installer places them.
  4. The scope of 7d2: the commands `status`, `config` and `check`; `check` verifies C1 to C4 and C8 to C11, and C5,
     C6, C7 and C12 are declared with phase 8; `status` shows each stage's and task's state and the record's health,
     the whole view being phase 8's and PD-37's rendering 7d3's; an identifier `status <id>` does not find is a usage
     error (exit 2); the catalogue covers, in `en` and `es`, every code the core raises today, every state, event
     kind and role, and its next steps name only commands that exist; exit 70 for an internal error.
- Consequences: `core-specification.md` records the interpretations of points 1 to 4 where they apply (sections 2
  CLI and SEC, 5, 6); the work affected is part 7d2, and 7d3's signing (point 1).
- Sources: chat of session 21; `STATUS.md`'s "Next"; D-040 (`lexopt`), D-053; `core-specification.md` REQ-CLI-1 to
  REQ-CLI-4, REQ-SEC-3, REQ-SEC-5, REQ-VIEW-4, sections 5, 6, 7, 11.2, 12 CLI and SEC.
- Supersedes: none.

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

## D-056 — Part 7d2: a citation is any text shaped as a Markdown link, read lexically

- Status: decided
- Decided: 2026-11-05
- Origin: maintainer XEL in session 21's chat, answering the integrator's question after Codex's third pass of 7d2
  kept finding CommonMark's structure (fences inside lists, block quotes, titles across paragraphs) breaking the
  citation reader (P7D2-3, P7D2-10, P7D2-11, P7D2-19, P7D2-20): "Regla léxica (Recomendado)".
- Decision: what the fermento or the alambique cites (D-055 point 2, REQ-SEC-3 at `check`) is read lexically, with no
  Markdown structure: every `[`, then on the same line `](`, followed by a destination, cites that destination,
  wherever it stands (in code, in a title, in another link's text). The destination is `<…>` up to its `>` on the
  same line, or else every character up to the first white space or the `)` that closes it, parentheses balanced; a
  backslash before punctuation escapes it. A destination with a scheme, an absolute path, only a fragment, or empty
  cites nothing; a fragment or a query after the path is dropped. The cost accepted: a file a maintainer names in an
  example is read for credentials too.
- Consequences: REQ-SEC-3's paragraph "As part 7d2 fixed it" in `core-specification.md` says so; the work affected is
  part 7d2's citation reader and its tests. D-055 point 2 stands, read with this entry.
- Sources: chat of session 21; Codex's three passes of 7d2; D-055; `core-specification.md` REQ-SEC-3.
- Supersedes: the interpretation of D-055 point 2 recorded after Codex's first pass (fences and code spans cite
  nothing).

## D-057 — Part 7d2: a tenth Codex pass after two retries without progress

- Status: decided
- Decided: 2026-11-06
- Origin: maintainer XEL in session 21's chat, answering the integrator's question when Codex's seventh, eighth and
  ninth passes of 7d2 each confirmed one blocking finding (two consecutive retries without progress by the measure of
  the plan's rule, D-003): "Pasada 10 completa (Recomendado)".
- Decision: one more full Codex pass of part 7d2 after P7D2-31 is corrected. With no confirmed blocking finding the
  part is accepted; with one, the integrator corrects it and asks the maestro again before any further pass.
- Consequences: R-029 records the passes; the alternatives offered were a scoped re-check of P7D2-31 and accepting
  the part without another pass.
- Sources: chat of session 21; `plan.md` (rule for every phase); Codex's passes 7 to 9 of 7d2.
- Supersedes: none.

## D-058 — Part 7d2: what Git reads to run is Git's; part 7d2 accepted

- Status: decided
- Decided: 2026-11-06
- Origin: maintainer XEL in session 21's chat, answering the integrator's question on Codex's tenth-pass finding
  P7D2-32 (Git opens the files its own configuration includes, `include.path`, even in another project's domain, and
  a pipe there blocks Git itself): "Es de Git; aceptar 7d2 (Recomendado)".
- Decision: Git's configuration is the person's tool configuration. What Git reads in order to run, its configuration
  and the files it includes, is outside REQ-SEC-3's and REQ-FER-9's boundary, which governs what Fabrewk opens; no
  command of Git can be told not to read its own local configuration. Fabrewk keeps refusing to use a value Git takes
  from a file in a domain (D-055 point 1 read with this entry). P7D2-32 is a residual, and part 7d2 is accepted with
  every other finding of Codex's ten passes closed.
- Consequences: `core-specification.md` section 6's paragraph "As part 7d2 fixed it" says so; R-029 records the
  residual. The alternatives offered were `git config --no-includes` for the signing key, or running Git without its
  global and system configuration, each followed by an eleventh pass.
- Sources: chat of session 21; Codex's tenth pass of 7d2; D-055, D-057; `core-specification.md` REQ-SEC-3.
- Supersedes: none.

## R-029 — Session 21, part 7d2: the command surface, the catalogue, status, config and check

- Status: recorded
- Date: 2026-11-06
- Who: integrator Claude Code, `claude-opus-5-5`. Reviewer: Codex `codex-cli 0.166.0`, `gpt-6.1-sol`, effort high,
  launched with `codex exec --json`, standard input closed, sandbox `workspace-write`, no network, in a user systemd
  scope capped at 3 GB, in a disposable clone under `target/review-7d2/` with a copy of the Cargo registry (35 MB).
  No subagent.
- Start: HEAD `4b61ed2`, tree clean; the fermento's answer had no current recommendation. Network, given by the maestro in the session's first message, was used for Cargo against
  crates.io only, to fetch `lexopt` 0.3.2.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, this file (D-047 to D-054, R-027, R-028), `core-specification.md`
  (sections 0, 2 VIEW, CLI, SEC and ID, 3, 5 to 9, 10, 11.2 to 11.4, 12 VIEW, CLI and SEC, 13), `architecture-decisions.md`
  AD-6, the alambique's 6.51, and the code of the record, settings, identity, the machine, detection, the domain, the
  table and the scan, with their test fixtures.
- Decided before any code (D-055): the person is Git's signing key; citations are Markdown links; completions and the
  reference are generated files; the scope. During the review: D-056 (citations read lexically, after Codex's third
  pass), D-057 (a tenth pass after two retries without progress), D-058 (what Git reads to run is Git's; 7d2 accepted).
- Written: `crates/core/src/cli/` (`decl.rs`: the declaration, the parser on `lexopt`, help, the fish and POSIX
  completions, the reference; `text.rs`: the catalogue reader; `mod.rs`: the reading context, the person and the
  language, refusals, usage and internal errors, the exit statuses, `--json`; `status.rs`, `config.rs`, `check.rs`),
  the catalogues `crates/core/catalogue/en.toml` and `es.toml`, the generated `crates/fabrewk/completions/` and
  `crates/fabrewk/doc/reference.{en,es}.md`; `person` and `identity`'s guard, `signing_key` with its origin,
  `head_of` that tells an unborn branch from a ref naming no commit, `table::current`, `RECORD_UNCOMMITTED`,
  `Clone` for `Error`, `normal`, `unlinked` and `in_domain` in `code.rs`; regular files only, and no link on the way,
  in the settings, the mark, the cache, the record and the private folder; `ssh-keygen`'s exit 255 as the only
  refused signature; the binary wired to the surface; tests in `crates/fabrewk/tests/` (`surface.rs`, `reading.rs`,
  `common/`), the skeleton's test removed, one 7b test's assertions changed (a non-directory `data/events` and a
  directory mark are now refused by code, never read as absence); both test fixture libraries sweep the fixtures of
  ended test processes. `core-specification.md` (the paragraphs "The command surface, as part 7d2 fixed it",
  REQ-SEC-3's and section 6's "As part 7d2 fixed it"), `plan.md`, `STATUS.md`, this entry.
- Interpretations, given to the review and not re-opened by it: the domain is `./.brewstillery` where the command
  runs; `status` and `check` show an absent record as a statement and as `RECORD_ABSENT`; a refusal's detail is the
  core's technical note in English, shown after the catalogue's words, never a tool's output; the catalogue covers
  the codes the core raises; `config` refuses `POLICY_ABSENT` with no policy; C5, C6, C7 and C12 come with phase 8;
  a private folder is refused inside this repository or any domain, not searched for in other repositories (accepted
  by Codex as an interpretation).
- Codex, ten passes, confirmed blocking findings by pass: 9, 7, 5, 4, 1, 2, 1, 1, 1, 1. Pass 1 (`94f2443`): P7D2-1 to
  P7D2-9 (a raw error in `status`; citing files that are links or pipes; citations by substring; C2 silent on an
  unreadable file; a crashed verifier taken as a bad signature; a detached head; C4's label in `unpublished`; guards
  shown as internal keys; a schema test that let fields go). Pass 2 (`0ca64b0`): P7D2-10 to P7D2-16 (fences, titles,
  a ref naming no commit, `status <id>` on an unreadable record, settings that are pipes or links, a citing file not
  UTF-8, the release build's features). Pass 3 (`551d5a1`): P7D2-17 to P7D2-21 (a signing key that is a pipe; links
  on the way to a file; CommonMark's lists and block quotes, settled by D-056; C1's listing). Pass 4 (`04ef5a1`):
  P7D2-22 to P7D2-25 (a key in the domain through a link; white space beyond space and tab; the private folder that
  cannot be examined; `--version` reading the context). Pass 5 (`c498109`): P7D2-26 (an unreadable personal file).
  Pass 6 (`8d14560`): P7D2-27, P7D2-28 (a domain that cannot be examined; a private folder inside the domain). Pass 7
  (`cf6faac`): P7D2-29 (one in another domain). Pass 8 (`05a618a`): P7D2-30 (a key in another domain). Pass 9 (cut
  short once by Codex's quota; run again on `ae40625`): P7D2-31 (a document branch the schema test did not see).
  Pass 10 (`bf12253`): P7D2-32, settled by D-058 as Git's own reading. Each pass checked that every new test fails
  without its correction. Codex ran no command outside its clone.
- Mutation (`cargo mutants --in-diff` over the part, rustup 1.99.1, `-j 2`, in its own scope as R-022 says, `TMPDIR`
  under `target/`): the first run filled the disk after 83 mutants, since every run of the suites left its fixtures
  (about 24 MB) and a failing test could leave a directory of mode 0000; corrected in the fixtures (`9a6479f`), and
  disposable clones of earlier sessions under `target/` were removed. Then 442 mutants: 44 missed, each killed by a
  new assertion or removed by simpler code; re-run on 437: 2 missed, one killed by a new case (`ae40625`), one
  declared equivalent: `>` → `>=` in `targets`, since `](` never starts where a `[` stands.
- Clean clone of `bbdacb0`, rustup 1.99.1: 257 tests green, and the crash suite (3 tests) with `--features
  fault-injection`; the musl release build, whose `--version --json` lists no feature; `budget check --binary` exits 0:
  core 4,530 of 7,500 counted lines in 30 files, 4 direct runtime crates, 31 crates in the release build, the binary
  1,380,712 bytes; `clippy --workspace --all-targets -D warnings` with and without the feature, `fmt --check`, `cargo
  deny --offline check licenses advisories` clean.
- Residuals, not blocking: P7D2-32 (Git reads what its own configuration includes, D-058); a private folder inside a
  repository with no domain is not detected; an unreadable signing key names nobody, so `config` shows no personal
  file; the crash suite failed once in Codex's clone copying a fixture (`PermissionDenied`), passed on repeat and in
  every run here; the next steps of `POLICY_ABSENT`, `PROTECTED_PATH` and other codes of later commands are words
  until those commands exist; the reference's options list `--help` as "Shows this help".
- Size: the core grew from 3,299 to 4,530 counted lines (+1,231; projected 650 to 900, the review's hardening about a
  third of it). Corrected projection, with this part's factor: about 5,350 to 5,650 after 7d3, 6,150 to 6,750 after
  phase 8, 7,150 to 8,150 after the core's part of phase 9; the ceiling of 7,500 may not hold through phase 9, which
  is the maestro's to decide when 7d3 is measured.
- Fermento: during the session the maestro added point 1 to their input, a question on skills that agents could
  use to work better in the bootstrap. The integrator's commit `8d14560` (`git add -A`) took that edit in with an AI
  co-author, the same slip as R-021's; history was not rewritten. At the close the answer was replaced, with the
  separator found exactly once and the input byte-identical: the point is not defined for the bootstrap (the
  alambique's 6.38 asks it for the projects Fabrewk serves), what to clarify (whom it serves, whether to find and
  evaluate or to write skills, where they would live given the domain rule), a wording as a proposal, and its
  destination.
- Not done: part 7d3.

## D-059 — Skills for the agents: two alambique edits applied, and a part 7s before 7d3

- Status: decided
- Decided: 2026-11-06
- Origin: maintainer XEL in session 21's chat, after the closing commit, asking for a sweep of their fermento's point 1
  (now a proposal: the integrator finds and evaluates skills for Claude Code and Codex, the maestro approves which,
  only those are installed, preferably inside the domain; the search required in every project's alambique) and
  suggesting an intermediate chat; then answering the integrator's two questions: "Aprobar ambas (Recomendado)";
  "Sí, con red (Recomendado)". The maestro's edit of the fermento went in its own commit (`3253df8`), with no AI
  co-author.
- Decision:
  1. The integrator applies the two edits drafted in the fermento's answer, as worded there: in section 2, a line
     after the maintainers' space that creates `.brewstillery/skills/` for the skills the maestros approved, read by
     every agent as `AGENTS.md` says, any file a tool demands elsewhere declared and minimal like `AGENTS.md`; in
     6.38, a sentence after the per-sweep tools: skills by the project's languages, tools and approach and to spare
     repeated work, found and evaluated by the agent, chosen by the maestro, only those installed, in
     `.brewstillery/skills/`; every project's alambique asks the search from its onboarding, as this one does for the
     bootstrap, where it is done before the core continues. Neither carries a status key.
  2. A part 7s runs before part 7d3, in its own chat: search and evaluate skills for Claude Code and Codex (Rust, Git,
     mutation testing, review, the bootstrap's flow), each with its permissions, provenance, version, licence and
     effects (6.38); propose which to adopt; install only those the maestro approves, in `.brewstillery/skills/`,
     with `AGENTS.md` saying to read them from there; reserve the name `skills` in the core's domain layout with its
     test (a maestro's approval of the exact change is asked in that chat); Codex reviews the evaluation and the code.
     Network in that chat to read sources of skills only; nothing is installed without the maestro's approval.
- Consequences: `ALAMBIQUE.md` changes as point 1 says (sha256 `61be0e9f…` before, `07181c8e…` after); `plan.md` gains
  the row of 7s; `STATUS.md`'s next part is 7s. `.brewstillery/skills/` is not created before 7s installs an
  approved skill.
- Sources: chat of session 21; `xel/fermento.md` point 1; `ALAMBIQUE.md` section 2 and 6.38; `AGENTS.md` ("Domain");
  `reference-study.md` PP-10 (generated skills that pre-approve tools) and C-60, C-61.
- Supersedes: none.

## D-060 — Part 7s: the skills adopted, read from the domain, and `skills` reserved

- Status: decided
- Decided: 2026-11-06
- Origin: maintainer XEL in session 22's chat, answering the integrator's four questions after reading the evaluation
  (`skills-evaluation.md`): third-party skills "property-based-testing (Recomendado), sharp-edges (Recomendado),
  differential-review (Recomendado)"; own skills "verify-part (Recomendado), codex-review (Recomendado)"; where they
  are read from "Solo el dominio (Recomendado)"; the exact change for the reserved name "Aprobar (Recomendado)". The
  first question named the licence (CC BY-SA 4.0), the revision (`470419e`) and the adaptation.
- Decision:
  1. Adopted from `trailofbits/skills` at `470419e16f97e112d01fc32f4898aea1755015d9`, adapted (the frontmatter keys
     `allowed-tools` and `effort` removed, a notice added; every other file unchanged), under CC BY-SA 4.0 with the
     licence text in each folder: `property-based-testing`, `sharp-edges`, `differential-review`. The notice puts
     `AGENTS.md` and the maestro first and sends a skill's own output outside every tracked path and the domain.
  2. Written by the integrator from what the record fixes, deciding nothing new: `verify-part` (the checks, the clean
     clone and mutation on this machine) and `codex-review` (launching and recording Codex's review).
  3. Route A of the evaluation: the skills live only in `.brewstillery/skills/<name>/`; `AGENTS.md` lists them and
     says when to read each; no entry under `.claude/` or `.agents/`, so neither tool loads them by itself, no
     frontmatter grants a tool, and nothing loads twice beside the copies in the maintainer's user folders.
  4. The core reserves `skills` (`RESERVED`, so no alias takes it) and lists `skills/` among the domain's directories
     (`DIRS`, so `check` reports nothing for it; a file or a link so named stays `DOMAIN_UNKNOWN_ENTRY`); its test in
     `crates/core/tests/domain.rs`; REQ-DOM-2 and REQ-DOM-3 amended. The core reads nothing inside `skills/`.
  5. Not adopted now, with a later use recorded in the evaluation: `spec-to-code-compliance` and `rust-review` as
     one-off plugin runs in a disposable clone, `security-threat-model` at phase 9; each a maestro's decision then.
- Consequences: `.brewstillery/skills/` exists with five skills; `AGENTS.md` gains the line "Skills";
  `core-specification.md` REQ-DOM-2, REQ-DOM-3 and the paragraph "The domain, as part 7c2b fixed it" name `skills`;
  `plan.md` row 7s. The five TickBrew copies in `~/.agents/skills/` (linked from `~/.claude/skills/`) are the
  maintainer's machine and stay as they are; `.brewstillery/settings.toml`'s `layout.reservedNames` is a bootstrap
  key no code reads, and adding `skills` there is the maestro's edit if wanted.
- Sources: chat of session 22; `skills-evaluation.md`; D-059; `ALAMBIQUE.md` section 2 and 6.38; `reference-study.md`
  PP-10.
- Supersedes: none.

## R-030 — Session 22, part 7s: skills for the agents

- Status: recorded
- Date: 2026-11-06
- Who: integrator Claude Code, `claude-opus-5-5`, effort high. Reviewer: Codex `codex-cli 0.166.0`, `gpt-6.1-sol`,
  effort high, launched as the new `codex-review` skill says (`codex exec --json`, standard input closed, sandbox
  `workspace-write`, no network, a user systemd scope capped at 3 GB, a disposable clone under `target/review-7s/`
  with a copy of the Cargo registry and clones of the skill sources). One subagent (`web-fetch`) read the two tools'
  skills pages.
- Start: HEAD `8bde466`, tree clean. Network, given by D-059 to read sources of skills only, was used for GitHub (`gh
  api`, shallow `git clone` of `trailofbits/skills`, `openai/skills`, `anthropics/skills`, `obra/superpowers`, the
  metadata and `SKILL.md` of `leonardomso/rust-skills` and `actionbook/rust-skills`), three web searches, and the
  two documentation pages (`code.claude.com/docs/en/skills`; `developers.openai.com/codex/skills`, redirected to
  `learn.chatgpt.com/docs/build-skills`), whose tails came back summarized and are marked so in the evaluation.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, the fermento, the settings, D-059 and R-029 (and R-022, R-024 and D-041
  for the recipes), `ALAMBIQUE.md` section 2, section 3 and 6.38, `reference-study.md` PP-10, C-60, C-61,
  `core-specification.md` REQ-DOM-2, REQ-DOM-3 and the 7c2b paragraph, `crates/core/src/domain.rs`, `settings/mod.rs`'s
  `RESERVED`, `crates/core/tests/domain.rs`; the five skills in `~/.agents/skills/` (headers and `MANIFEST.sha256`,
  which verifies) and the list of `~/.codex/skills/.system/`. TickBrew's `AGENT_SKILLS_POLICY.md`, which those copies
  name, is not cited by the alambique and was not read.
- Decided: D-060, after the evaluation and before any installation or code.
- Written: `skills-evaluation.md`; `.brewstillery/skills/` with `property-based-testing`, `sharp-edges` and
  `differential-review` copied from `trailofbits/skills` `470419e` (upstream `SKILL.md` sha256 `9448cacb…`,
  `3b69a709…`, `f9af6a81…`) and adapted as D-060 says, and `verify-part` and `codex-review`; `AGENTS.md` "Skills";
  `skills` in `RESERVED` and `DIRS`, the assertions in `crates/core/tests/domain.rs` (a `skills/` directory listed, a
  file and a link so named unknown entries, `skills` never an alias), REQ-DOM-2, REQ-DOM-3 and the 7c2b paragraph;
  `plan.md`, `STATUS.md`, this entry. The new test assertions failed before the change (`skills` was a valid alias)
  and with `RESERVED` alone (the directory became an unknown entry). `fabrewk check` here gives the same findings before
  and after; nothing in the skills' text raises one.
- Codex, three passes, confirmed blocking findings by pass: 3, 1, 0. Pass 1 (`8dc7ee5`): P7S-1 (the notices of
  `differential-review` and `property-based-testing` did not name the creators the upstream plugins name, as CC BY-SA
  4.0's attribution asks), P7S-2, not blocking (`differential-review`'s conditional `git checkout` of the baseline
  missing from the evaluation; every notice now forbids moving HEAD or changing the index or the working tree), P7S-3
  (`verify-part`'s mutation run lacked `fault-injection`, so the crash suite judged no mutant, against R-024), P7S-4
  (`codex-review`'s build line lacked the rustup shims on `PATH`; Fedora's `cargo` refuses `+1.99.1`). Pass 2
  (`1eacd71`): P1 to P4 closed; P7S-5 (with the feature, a run mutating only `budget` fails its baseline). Codex's
  suggested fix, `--test-workspace true`, was run here and still failed the baseline; the recipe was split into a run
  of the core and the binary with the feature and every package's tests, and one of the counter without it, each
  run here on one mutant, caught. Pass 3 (`4f2e3ad`): P7S-5 closed, no new finding; accept.
- Checks (the clean clone of `8dc7ee5`, rustup 1.99.1): 257 tests and the crash suite (3) green; the musl release
  build, `--version --json` listing no feature; `budget check --binary` exits 0: core 4,540 of 7,500 counted lines in
  30 files (+10, `rustfmt` splitting `RESERVED`), 4 direct runtime crates, 31 crates in the release build, the binary
  1,380,712 bytes; `clippy` with and without the feature, `fmt --check`, `cargo deny --offline check licenses
  advisories` clean. The corrections after it changed no code. Mutation: `--in-diff` over the part finds no mutant
  (only constants changed); the assertions above guard both constants.
- Cleaning: the review and clean-clone directories of 7c1 and 7d2, and the mutation outputs of 7d1 and 7d2, under
  `target/`, removed at the start (the disk had 4.9 GB free); this part's are removed at the close.
- Residuals, not blocking: the evaluation's section 2 rests on two web pages Codex could not fetch, checked only for
  consistency; the five TickBrew copies in `~/.agents/skills/` still load in every session of both tools on this
  machine, beside the domain's copies that only `AGENTS.md` points to; the community and registry skills were judged
  on metadata and their top file only.
- Fermento: the answer was replaced at the close, the separator found exactly once and the input byte-identical
  (everything above the answer, sha256 `141a03e1…`, before and after): point 1 is done and may be removed.
- Not done: part 7d3.

## D-061 — Skills from outside the project: reported, never touched

- Status: decided
- Decided: 2026-11-06
- Origin: maintainer XEL in session 22's chat, after the close. The integrator had said that this repository's case
  (five TickBrew copies in the maintainer's user folders, three with the names of skills adopted in D-060) is
  something Fabrewk should report when it assesses a repository. The maintainer added it to their fermento as an open
  item authorizing the edit, removed the five copies from `~/.agents/skills/` and `~/.claude/skills/` (kept in
  `~/Dev/tmp/skills-tickbrew/`, whose `MANIFEST.sha256` verifies), and answered the drafted sentence: "Aprobar
  (Recomendado)".
- Decision: 6.38 gains, after the sentence on the search for skills: "Al evaluar un repositorio y en cada barrido,
  Fabrewk informa además qué skills cargan los agentes desde fuera del proyecto (las carpetas de usuario de cada
  herramienta en el equipo) y avisa cuando una lleva el nombre de una aprobada en `.brewstillery/skills/` o
  contradice las reglas del proyecto; no las toca: conservarlas o quitarlas lo decide el dueño del equipo." No status
  key, like D-059's sentences.
- Consequences: `ALAMBIQUE.md` changes (sha256 `07181c8e…` before, `58774ab4…` after). The core specification is frozen
  and has no requirement for it yet; it is due where the assessment (REQ-INS) and the sweep are specified, a change to
  propose when phase 8 or phase 10 starts. The residual of R-030 on the user copies is closed. The maintainer's
  settings edit (`layout.reservedNames` gains `skills`) went in its own commit, `29b7a40`.
- Sources: chat of session 22; `xel/fermento.md`'s open item; D-060, R-030; `ALAMBIQUE.md` 6.38.
- Supersedes: none.

## D-062 — Part 7d3 split into 7d3a and 7d3b; `run` takes words; `TASK_CHANGED` through `--seen`; agents' markers declared by the binary

- Status: decided
- Decided: 2026-11-06
- Origin: maintainer XEL in session 23's chat, answering the integrator's four questions before any code, after the
  integrator estimated 7d3 at about 1,100 to 1,500 counted lines of core: "7d3a y 7d3b (Recomendado)"; "`run` exige
  --say (Recomendado)"; "`--seen <digest>` obligatorio (Recomendado)"; "Lista en el binario (Recomendado)".
- Decision:
  1. Part 7d3 is split in two, each with its own chat and Codex review. 7d3a, the write path and the policy and
     proposal commands: the declaration's arguments that are required and options that take a value (`--say`,
     `--say-lang`, `--widen`, repeated); the write context every write command opens (REQ-REG-5's lock, REQ-REG-11,
     the mark, REQ-REG-10's commit of what is left uncommitted, REQ-DOM-7, the machine key of REQ-REG-12 and
     `instance`); signing with Git's signing key (D-055 point 1); `--say` and `--say-lang` (REQ-TASK-4, PD-30 to
     PD-32 for the moves that exist), `AGENT_ENVIRONMENT`; `adopt` and `propose`; the refusal tests of the record's
     codes through the command line (D-047). 7d3b, the task commands: `authorize` (PD-37, `TASK_CHANGED`,
     `LIMIT_UNOBSERVABLE`, the layers' widenings), `reject`, and `run` up to confinement (D-053 point 4), with
     `MOVE_NOT_ALLOWED`, `DEPENDENCY_OPEN` and `LOCK_HELD` through `run`. Accepted with the question, as stated in
     it: REQ-RUN-5 is phase 9's, since no command of phase 7 opens an attempt; `propose` records Fabrewk as its actor;
     `authorize --run` evaluates the guards of both moves before writing either, so without confinement it writes
     nothing; Git's signing key may be a `key::` literal, a public key file or a private key file, and the signature
     is made with `ssh-keygen -Y sign` as Git makes its own (with the SSH agent where the key is public only).
  2. `run` is a person's move like the others: `--say` is required (`WORDS_MISSING` without it), and `run-started`
     carries the words. `authorize --run --say` puts the same words in the authorization and in each `run-started`
     it signs; `run <stage> --say`, in each task's. Section 4's row of `run-started` gains the mark `say`.
  3. `status <id>` prints the digest of its rendering of what an authorization binds (REQ-GATE-7, PD-37), and
     `authorize <id>` requires `--seen <digest>`: when the rendering made under the lock differs from the one the
     person read, it refuses `TASK_CHANGED` and writes nothing. The signed `task-authorized` and `stage-authorized`
     carry that digest in a new payload field, `shown`, which amends the payloads part 7d1 fixed (no real event
     exists yet). Section 5's synopsis of `authorize` gains `--seen <digest>`.
  4. The `fabrewk` binary declares, on each adapter's behalf, only its environment markers (`claude-code`:
     `CLAUDECODE`; `codex`: `CODEX_SANDBOX` and `CODEX_SANDBOX_NETWORK_DISABLED`, read from the strings of
     `codex-cli` 0.166.0's binary), and passes them to the core as it passes the modules; `FABREWK_EXTENSION` is the
     core's own. Phase 9 moves each list into its module's contract. The core still names no tool (AD-5).
- Consequences: `plan.md` shows 7d3a and 7d3b instead of 7d3. `core-specification.md` records each point where it
  applies, in the part that builds it: 7d3a points 1 and 4 and the interpretations of point 1; 7d3b points 2 and 3.
  The size budget is unchanged; the maestro decides it when 7d3 is measured, as R-029 says.
- Sources: chat of session 23; `STATUS.md`'s "Next"; D-047, D-053, D-055, R-028 (the two items left open for 7d3),
  R-029; `core-specification.md` sections 2 TASK, RUN, SEC, GATE-7, 3 to 5, 11.3, 11.4; the strings of
  `~/.codex/packages/standalone/releases/0.166.0-aarch64-unknown-linux-musl/bin/codex`.
- Supersedes: none.

## D-063 — Part 7d3a: what a write command cleans and keeps before it refuses; a fourth Codex pass

- Status: decided
- Decided: 2026-11-06
- Origin: maintainer XEL in session 23's chat, answering the integrator's three questions when Codex's first, second
  and third passes of 7d3a each confirmed five blocking findings (two consecutive retries without progress by the
  measure of the plan's rule, D-003): "Corregir, decidir y pasada 4 (Recomendado)"; "Aprobar como diseño
  (Recomendado)"; "Aprobar como diseño (Recomendado)".
- Decision:
  1. The integrator corrects P7D3A-7 (a dangling link as a Git lock), P7D3A-8 (a `..` in a private folder's path
     not made yet), P7D3A-12 (the index repair touching staged files in directories replay skips) and P7D3A-13 (a
     refusal missing from the declaration) in code; then one more full Codex pass. With a confirmed blocking finding
     left, the integrator corrects it and asks the maestro again before any further pass, as D-057 did.
  2. Cleaning what an interrupted write left is part of every write command, even one that then refuses: temporary
     files, signatures whose event never reached its place, the record commit's temporary index, and a machine key
     that does not read (7b's `Session::begin`, 7c2a's "a key that does not read is none, and a write command makes
     it again"). None of it is a person's content; section 11.2 rule 4's "nothing written" holds for everything
     else. This settles P7D3A-1's remaining variants.
  3. The machine key is not part of the record: after a race with a writer that ignores the lock
     (`RECORD_HEAD_MOVED`, `RECORD_COMMIT_RACE`), the key the refused command made stays, and the next command uses
     it, as the empty `local/lock` already may. This settles P7D3A-11.
- Consequences: `core-specification.md`'s paragraph "The write path, as part 7d3a fixed it" says points 2 and 3;
  R-031 records the passes. The alternatives offered were all in code with a fourth pass, a scoped re-check, and
  accepting the part without another pass.
- Sources: chat of session 23; Codex's passes 1 to 3 of 7d3a; D-057, D-062; `core-specification.md` 11.2 rule 4.
- Supersedes: none.

## D-064 — Part 7d3a: a fifth Codex pass

- Status: decided
- Decided: 2026-11-06
- Origin: maintainer XEL in session 23's chat, answering the integrator's question after Codex's fourth pass of 7d3a
  confirmed three blocking findings (P7D3A-14 to P7D3A-16, corrected in `77257ae`), as D-063 asked: "Pasada 5
  completa (Recomendado)".
- Decision: one more full Codex pass of part 7d3a. With no confirmed blocking finding the part is accepted and
  closed; with one, the integrator corrects it and asks the maestro again before any further pass.
- Consequences: R-031 records the passes. The alternatives offered were a scoped re-check of P7D3A-14 to P7D3A-17
  and accepting the part without another pass.
- Sources: chat of session 23; Codex's fourth pass of 7d3a; D-057, D-063.
- Supersedes: none.

## D-065 — Part 7d3a: a record commit that fails leaves its events, and says so; a sixth Codex pass

- Status: decided
- Decided: 2026-11-06
- Origin: maintainer XEL in session 23's chat, answering the integrator's two questions after Codex's fifth pass of
  7d3a confirmed two blocking findings (P7D3A-18, P7D3A-19; the latter corrected in `3a2f375`), as D-064 asked:
  "Diseño, con aviso (Recomendado)"; "Pasada 6 completa (Recomendado)".
- Decision:
  1. A record commit that fails after the command wrote its events, whatever the cause (a race, a Git that refuses
     the ref's update, as a `reference-transaction` hook may), leaves those events written and not committed, as
     REQ-REG-10 already says of a race and of a command stopped between its commits; the refusal's detail says that
     the events are written and that the next write command commits them, `check` reports them
     (`RECORD_UNCOMMITTED`), and the next write command commits them first. This settles P7D3A-18.
  2. One more full Codex pass of part 7d3a; with no confirmed blocking finding the part is accepted and closed; with
     one, the integrator corrects it and asks the maestro again.
- Consequences: `core-specification.md`'s paragraph "The write path, as part 7d3a fixed it" says point 1. The
  alternatives offered were removing the events a failed commit leaves, and, for the pass, a scoped re-check or
  accepting the part.
- Sources: chat of session 23; Codex's fifth pass of 7d3a; D-063, D-064; `core-specification.md` REQ-REG-10.
- Supersedes: none.

## R-031 — Session 23, part 7d3a: the write path, adopt and propose

- Status: recorded
- Date: 2026-11-06
- Who: integrator Claude Code, `claude-opus-5-5`, effort high. Reviewer: Codex `codex-cli 0.166.0`, `gpt-6.1-sol`,
  effort high, launched as the `codex-review` skill says (`codex exec --json`, standard input closed, sandbox
  `workspace-write`, no network, a user systemd scope capped at 3 GB, a disposable clone under `target/review-7d3a/`
  with a copy of the Cargo registry). No subagent.
- Start: HEAD `5838b38`, with the maestro's fermento edit (the open item removed) not committed. It went in its own
  commit, `2577784`, with no AI co-author; the answer was then replaced (`15d6942`), the separator found exactly once
  and the input byte-identical (everything above the answer, sha256 `8cf4493e…`, before and after): no current
  recommendation.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, this file (D-047, D-053 to D-061, R-028 to R-030), `core-specification.md`
  (sections 2 DOM, REG, SET, ID, TASK, RUN, CLI, SEC, GATE-7, GATE-9, 3 to 8, 10, 11.2 to 11.5, 12, 13), the skills
  `verify-part` and `codex-review`, and the code of the record, settings, identity, the machine, detection, the domain,
  the table, the payloads and the command surface, with their test fixtures; the strings of `codex-cli` 0.166.0's
  binary for its sandbox variables.
- Decided before any code (D-062): 7d3 split into 7d3a and 7d3b; `run` takes words; `authorize` takes `--seen`; the
  binary declares the agents' markers. During the review: D-063 (after pass 3: two retries without progress; what a
  write command cleans and keeps before it refuses; a fourth pass), D-064 (a fifth pass), D-065 (a failed record
  commit leaves its events and says so; a sixth pass).
- Written: `crates/core/src/cli/write.rs` (the write path: guards on the record read without the lock, the session,
  the guards again under the lock, every event made and signed in memory, the machine key put in place after, the
  leftovers' commit, the events by compare-and-swap, the record commit, the caches under the lock;
  `AGENT_ENVIRONMENT`, the person by Git's signing key, the words), `cli/adopt.rs`, `cli/propose.rs` (the file read as
  JSON, its format generated from the payloads in `task.rs`), the declaration's required argument and options with a
  value (`decl.rs`), the write commands' documents, the internal error of a write command, four codes
  (`AGENT_ENVIRONMENT`, `STAGE_CLOSED`, `WORDS_COPIED`, `WORDS_MISSING`) and their catalogue entries in `en` and `es`;
  `GitKey`, `GitSigner`, `holder` and the signing key's files held to D-055 (`identity.rs`); `Session::prepare`,
  `write`, `refresh`, `finish_held`, the mark after the commit, a rebuilt commit only on a head that descends and is
  walked, the leftovers and the index repair restricted to the streams replay reads, `check_ident`, a Git lock as the
  entry itself (`record/`); `machine::Pending` and `prepare_instance`; `resolved` for a path not made yet (`code.rs`);
  a fault seam that pauses at a named step and one for a test nonce; the binary's markers and its feature
  `fault-injection` with the test `faults` (which requires the core's feature); tests in
  `crates/fabrewk/tests/writing.rs` (43) and `faults.rs` (8), the
  shared `refused` helper moved to `tests/common/` and holding every write command's code to its declaration;
  the generated completions and reference; `core-specification.md` (the paragraph "The write path, as part 7d3a fixed
  it"), `plan.md`, `STATUS.md`, this entry.
- Interpretations, given to the review and not re-opened by it: those of D-062 point 1; `--say` followed by an
  argument that starts with `-` gives no words (found by the integrator's own tests: `--say --json` took `--json` as
  the words); a record with a policy and no `publication-decided` is `RECORD_BROKEN`; the mark is compared before the
  mode, so a cut record is `RECORD_ROLLBACK`; `adopt` may adopt a file equal to the policy in force; `propose`'s file
  is at most 1 MiB, outside every domain.
- Codex, six passes, confirmed blocking findings by pass: 5, 5, 5, 3, 2, 0. Pass 1 (`1d93a14`): P7D3A-1 (a signing
  refusal had made the machine key and committed leftovers), P7D3A-2 (an invalid `machine.toml` read after the
  commit), P7D3A-3 (caches written after the lock was released), P7D3A-4 (a key file `ssh-keygen` opens through the
  `.pub` fallback, in a domain or a pipe), P7D3A-5 (a detached head accepted in `unpublished`); P7D3A-6 not blocking
  (the fault tests' contract). Pass 2 (`28befaa`): P7D3A-1 again (a probe signature is no guarantee; a key literal's
  directory left), P7D3A-7 (a Git lock checked after the machine key), P7D3A-8 (a private folder not made yet through a
  link into a domain), P7D3A-9 (leftovers from a hidden directory committed), P7D3A-10 (the mark moved by a refused
  commit). Pass 3 (`9d8f822`): P7D3A-1's remaining variants and P7D3A-11 (settled by D-063), P7D3A-7 (a dangling link
  as a lock), P7D3A-8 (a `..` after a name not made), P7D3A-12 (staged files in a hidden directory reset); P7D3A-13
  not blocking (a refusal missing from the declaration). Pass 4 (`972ebe5`): P7D3A-14 (no Git author, refused after
  writing), P7D3A-15 (a generated identifier's collision written as a second creation), P7D3A-16 (a commit rebuilt on
  a head that drops an event); P7D3A-17 not blocking. Pass 5 (`77257ae`): P7D3A-18 (settled by D-065), P7D3A-19 (a
  commit rebuilt on a rewound branch); P7D3A-20 not blocking (snapshots blind to a link's target). Pass 6 (`a543909`):
  **accept**; P7D3A-21 and P7D3A-22 not blocking, corrected after it (`8261a4e`: an index another Git holds after the
  commit is left for the next command; the size limit at its boundary). Each pass checked that every new test fails
  without its correction. Codex ran no command outside its clone; its sandbox cannot bind an SSH agent's socket, so
  it skipped that one test, which passes here.
- Clean clone of `9d91a59`, rustup 1.99.1: 300 tests green, and the fault suites (`crash` 3, `faults` 8) with
  `-p fabrewk-core -p fabrewk --features fault-injection --test crash --test faults`; the musl release build, whose
  `--version --json` lists no feature; `budget check --binary` exits 0: core 5,377 of 7,500 counted lines in 33 files,
  4 direct runtime crates, 31 crates in the release build, the binary 1,642,856 bytes; `clippy --workspace
  --all-targets -D warnings` with and without `fabrewk-core/fault-injection`, `fmt --check`, `cargo deny --offline
  check licenses advisories` clean.
- Mutation (`cargo mutants --in-diff` over the part, rustup 1.99.1, `-j 2`, `--test-workspace true`, in its own scope
  as R-022 says, `TMPDIR` under `target/`): the first baseline failed, since under cargo-mutants' copy one test's SSH
  agent socket passed a Unix socket's 108-byte limit (`a643eb2` gives it a short relative path). Then 259 mutants in
  40 minutes: 163 caught, 85 unviable, no timeout, 11 missed, each killed by a new test or removed (`338e6ca`: the
  internal error of a write command, a private folder through a link into the repository, a key literal's directory
  there before, the format in `propose`'s help only, a task's own protected paths, the file's exact shape, a hidden
  directory's temporary-named file, the record planned again under the lock through a fault pause before the lock;
  the unused `Pending::fingerprint` removed). A second run over the functions concerned (34 mutants; with
  `--features fabrewk-core/fault-injection`, after the binary's `faults` was made to require the core's feature,
  since a run whose mutants are all in the core builds the core alone): 26 caught, 7 unviable, 1 missed, `!` in
  `GitSigner::sign`'s test of the directory it made, which the shared refusal helper's two runs hid (the second
  undoing the first); killed by assertions after a single run (`9d91a59`), checked by hand on the mutant. No
  equivalent declared.
- Residuals, not blocking: the SSH agent test cannot run in Codex's sandbox; D-063's two exceptions and D-065's
  events left by a failed commit; the TOCTOU between a file's metadata and its read, as in 7d2; `--widen` naming a
  key that is no widening is accepted and records nothing.
- Size: the core grew from 4,540 to 5,377 counted lines (+837; R-029 projected 5,350 to 5,650 after the whole of 7d3,
  of which 7d3a was planned as the larger half). Corrected projection: 7d3b about +450 to +650 (5,850 to 6,050), then
  phase 8 and the core's part of phase 9 as R-029 estimated them (+800 to +1,100 and +1,000 to +1,400), about 7,650
  to 8,550: the ceiling of 7,500 is likely not to hold through phase 9, which is the maestro's to decide when 7d3b is
  measured.
- Skills: the maestro approved in chat ("Aprobar (Recomendado)") two edits to `verify-part`: its fault-suite line runs
  `-p fabrewk-core -p fabrewk --features fault-injection --test crash --test faults`, and its mutation paragraph says
  that the core's feature builds `faults` too.
- Not done: part 7d3b.

## R-032 — Session 23, after the close: a sweep of the maestro's proposal on the first alambique

- Status: recorded
- Date: 2026-11-06
- Who: integrator Claude Code, `claude-opus-5-5`.
- Fermento: during part 7d3a's review the maestro added a proposal to their input (the onboarding guides a new
  maintainer, with simple questions, to write the project's first alambique). The integrator's commit `972ebe5`
  (`git add -A`) took that edit in with an AI co-author, the slip of R-021 and R-029; history was not rewritten. At
  the maestro's request the answer was replaced, the separator found exactly once and the input byte-identical
  (everything above the answer, sha256 `00d129fc…`, before and after): the point is not defined yet (AD-7's interview
  asks the mode, the publication, the alias, the key and the tools; `apply` records the alambique's digest, and
  nothing creates it); a recommended design (a few questions in the onboarding when no alambique exists, the answers
  placed as given on a fixed skeleton, the file part of the plan and approved with it, an existing one never
  touched, the agent route asking the same questions in chat); its destination (the alambique's section 1, after the
  installation item; phase 10, on phase 8's alambique machinery; the frozen specification amended when phase 10
  starts). The drafted alambique text was given in chat for the maestro's approval.
- Read: `ALAMBIQUE.md` sections 1 and 2, `architecture-decisions.md` AD-7, `core-specification.md` REQ-INS-1 to
  REQ-INS-7, REQ-FER-7 and section 3's `alambique-baseline`.

## D-066 — The first alambique in the onboarding, applied on the maestro's approval

- Status: decided
- Decided: 2026-11-06
- Origin: maintainer XEL in session 23's chat, after the sweep of R-032: "Está bien el texto, recuerda nada mas que la
  explicación que se le dé al mantainer debe explicar muy pedagógicamente la analogía entre "Brewstillery",
  "Alambique", "Fermento" y "Maestro" en el Alambique muestra para que este comprenda porque los nombres... Dicho
  esto, si no hay mas dudas sobre este tema, apruebo la integración al alambique."
- Decision: section 1 of the alambique gains, after the item on the two ways to install, the item drafted in R-032's
  sweep with the maestro's addition: when a project has no alambique yet, the onboarding asks a few simple questions
  (what the project should achieve, for whom, what it leaves out) and builds from the answers, as given, a first
  `ALAMBIQUE.md` on a skeleton of stable titles and numbering, adding no ideas of its own; that first alambique opens
  by explaining pedagogically the analogy that names everything (Brewstillery the distillery, Fabrewk's domain; each
  maestro's fermento; the alambique where the fermentos are distilled into what the project must be; only maestros
  approve what enters); it is part of the plan, read whole before confirming and approved with it; on the agent route
  the agent asks the same questions in chat without drafting; an existing alambique is never touched. No status key,
  like D-059's and D-061's sentences.
- Consequences: `ALAMBIQUE.md` changes (sha256 `58774ab4…` before, `044bf04f…` after). The frozen specification has
  no requirement for it yet; it is due with the installation (REQ-INS-2, REQ-INS-4, AD-7 step 2, the
  `alambique-baseline` event), on phase 8's alambique machinery, a change to propose when phase 10 starts. The
  fermento's answer says the proposal is applied and may be removed.
- Sources: chat of session 23; `xel/fermento.md`'s proposal; R-032; `ALAMBIQUE.md` sections 1 and 2;
  `architecture-decisions.md` AD-7; `core-specification.md` REQ-INS-1 to REQ-INS-7.
- Supersedes: none.

## R-033 — Session 24, before part 7d3b: the machine's disk space, and a sweep of two proposals

- Status: recorded
- Date: 2026-11-10
- Who: integrator Claude Code, `claude-opus-5-5`; second opinion on the home sweep by Codex (`codex-cli 0.166.0`,
  `gpt-6.1-sol`, effort high, `codex exec -s read-only` in a memory-capped scope, launched from an empty disposable
  repository under `target/`, removed after).
- Disk, at the maestro's request before 7d3b, so that linking, the clean clone and the mutation runs do not fail: the
  root btrfs (shared by `/` and `/home`) went from 2.9 GiB free of 90 (97 %) to 33 GiB (63 %). Removed by the
  integrator, regenerable or superseded only (about 5.6 GiB): Chromium, Zen, VS Code VSIX and Codex caches, the npm,
  pip and user dnf caches, Codex 0.165.0 (CLI and daemon), Claude CLI 2.1.296 and 2.1.314, VS Code's obsolete and
  unregistered extension copies, the Pine Script server's build intermediates. Moved to the external disk with
  `rsync -aHAX`, verified by a checksum dry run and file counts, then removed (about 4.3 GiB): `~/rpmbuild`, two
  installed `.rpm` files, three Claude Code transcript folders of projects that no longer exist, and every folder of
  `~/Dev` except `Fabrewk` and `TickBrew`. Run by the maestro with sudo: five monthly snapper snapshots deleted (1,
  77, 312, 640, 884; 2.3 to 5.0 GiB exclusive each), the timeline limits lowered, `/etc/snapper/configs/root` given
  back to root, the journal capped at 200 MB, the coredumps removed.
- External disk: subvolume `Dev` of a btrfs disk mounted at `/home/xel/Dev/external`; it also holds subvolume `VM`
  (qemu images, `nocow`). The first subvolume mounted fixes the options for both, so the maestro removed
  `compress=zstd:1` from `/etc/fstab`; the integrator set `btrfs property compression=zstd` on
  `/home/xel/Dev/external` (new files there are compressed, checked with `filefrag`).
- Paths the record cites: `/home/xel/Dev/tmp` (R-001's check, R-005's note, R-030's `skills-tickbrew/`, the audit's
  pending item) now reads `/home/xel/Dev/external/tmp`; `skills-tickbrew/MANIFEST.sha256` verifies there. `~/Dev/tmp`
  no longer exists, as the audit asked. `/home/xel/Dev/TickBrew/` is unchanged: the alambique's section 3 cites it and
  the pilot (plan phases 11 and 12) still needs it.
- Backup: Fabrewk has no remote; `git bundle create --all` of `24ba4d7` at
  `/home/xel/Dev/external/backups/fabrewk/fabrewk-2026-11-10-24ba4d7.bundle`, `git bundle verify` clean.
- Outside the repository and outside Fabrewk, the maestro's scripts: `/home/xel/Dev/tools/dev-clean.sh` (development
  tools' caches and superseded versions; dry run by default, skips any running tool) and
  `/home/xel/System/Asahi/Fedora/system_maintenance.sh` (snapper, journal, coredumps, dnf, flatpak; sudo).
- Fermento: the maestro added two proposals (Fabrewk's own disk discipline; a staged self-rebuild after the pilot)
  and removed the one applied in D-066; their edit committed alone in `178344e`, with no AI co-author. The answer was
  replaced twice (the drafts, then the applied note); the separator found exactly once and everything above the
  answer byte-identical (sha256 `29ab4fd4…`) before and after. The drafted alambique text was given in chat.
- Read: `ALAMBIQUE.md` sections 1, 2, 6.1 to 6.4, 6.39 to 6.41, 6.45 to 6.48; `plan.md`'s phase table;
  `core-specification.md` REQ-DOM-5; the skills `codex-review` and `verify-part`; D-022, D-066, R-032.

## D-067 — Disk space and a staged self-rebuild, applied on the maestro's approval

- Status: decided
- Decided: 2026-11-10
- Origin: maintainer XEL in session 24's chat, after the sweep of R-033: "estoy de acuerdo contigo, el alambique solo
  debe ocuparse de la limpieza de archivos generados por su propio build", then, on the drafted text: "Aprobadas".
- Decision: four edits to the alambique, with no change of key on existing points.
  1. 6.4 gains: each area declares the life of what it keeps (temporary ends with its attempt, cache is rebuildable
     and purgeable, evidence is never deleted automatically and `status` reports its size so a maestro decides to
     archive it); Fabrewk cleans only what it generates in its domain and never touches other tools' files.
  2. 6.41 gains: before a job that writes much (an attempt's copy, a build, a verification) Fabrewk compares the free
     space with a threshold from the project's settings and refuses with a clear code instead of failing halfway.
  3. 6.46 gains: the minimum free space the project's builds and verifications need is one of a machine's
     requirements.
  4. A new point 6.53, staged self-rebuild, `[proyectado]`: after the pilot Fabrewk installs itself in its own
     repository, re-evaluates itself with its own tools against the alambique, and rebuilds only what that evaluation
     justifies, each piece as a Fabrewk task; tests, past-defect cases and independent review stay as external
     oracles and acceptance criteria do not change. The pilot's section is renamed "Piloto y autoconstrucción (6.48,
     6.53)".
- Consequences: `ALAMBIQUE.md` changes (sha256 `044bf04f…` before, `b9409261…` after). The frozen specification has
  no requirement for points 1 to 3 yet: the free-space refusal and the evidence's size in `status` are a change to
  propose when phase 9 starts (real agents write the attempts' copies); the machine's minimum free space, with the
  installation's assessment when phase 10 starts. Part 7d3b's scope does not change. Point 4 adds no phase now; one
  is proposed after phase 13 when it is evaluated. The maintenance of the machine's other tools stays outside the
  project (R-033). The fermento's answer says both proposals are applied and may be removed.
- Sources: chat of session 24; `xel/fermento.md`'s two proposals; R-033; `ALAMBIQUE.md` 6.4, 6.41, 6.46, 6.48;
  `plan.md` phases 9 to 13; `core-specification.md` REQ-DOM-5.
- Supersedes: none.

## D-068 — Part 7d3b: `run`'s guards of phase 9, `--seen` required by the parser, `reject` bound and closed by default, the wildcard guard at authorization

- Status: decided
- Decided: 2026-11-10
- Origin: maintainer XEL in session 25's chat, answering the integrator's four questions before any code: "Diferir a
  fase 9 (Recomendado)"; "Error de uso (Recomendado)"; "Atar y cerrar por defecto (Recomendado)"; "Añadirla
  (Recomendado)".
- Decision:
  1. `run` evaluates, in section 4's order, `POLICY_ABSENT` (the write path's), `DEPENDENCY_OPEN`, `LOCK_HELD` (a
     task of the record in `running`: tasks run one at a time, REQ-TASK-3) and then refuses `CONFINEMENT_UNAVAILABLE`
     with nothing written. `BASE_MOVED` (it needs `task-integrated`'s payload) and REQ-RUN-2 (in phase 7 the binary
     has no module, so every profile's tool is absent and REQ-RUN-2 would always refuse `TOOL_MISSING`, leaving
     confinement unreached) are evaluated from phase 9, with the modules and the integration, where 11.4 and section
     12 already test them.
  2. `authorize` without `--seen` is a usage error (exit 2), as a missing required argument is; the synopsis shows
     `--seen <digest>` as required.
  3. `task-rejected`'s payload is exactly `task` and `head`, bound as an authorization is and held to it by the
     replay (PD-06's binding); where the record branch is, phase 9 adds with the candidate. Until phase 9 can tell
     where the branch is, `MERGE_IN_PROGRESS` refuses whenever a `task-accepted` follows the task's last
     `candidate-recorded`.
  4. `task-authorized` and `stage-authorized` check `PROTECTED_PATH_WILDCARD` again, after `PROTECTED_PATH`, against
     the protected paths in force with the stage's and the task's layers: `propose` is cooperative, a hand-written
     `task-proposed` could hold a wildcard that reaches a protected path, and section 4 says the authorization checks
     those fields again. Section 4's two rows gain the guard, effective since the maestro's signature follows it.
- Consequences: `core-specification.md` sections 3 (`shown`, `task-rejected`), 4 (the two rows, `run-started`'s
  `--say` of D-062 point 2) and 5 (the synopses), and the paragraph "The task commands, as part 7d3b fixed it". The
  alternatives offered were evaluating REQ-RUN-2 now (confinement tested only at the library's level), `TASK_CHANGED`
  for a missing `--seen`, `reject`'s guard left to phase 9, and keeping the wildcard guard at `propose` only.
- Sources: chat of session 25; `STATUS.md`'s "Next"; D-053, D-062; `core-specification.md` sections 2 TASK, RUN, GATE-7
  to GATE-9, 3 to 5, 11.4, 12; `crates/fabrewk/src/main.rs` (no module in phase 7).
- Supersedes: none.

## R-034 — Session 25, part 7d3b: the task commands

- Status: recorded
- Date: 2026-11-10
- Who: integrator Claude Code, `claude-opus-5-5`, effort high. Reviewer: Codex `codex-cli 0.166.0`, `gpt-6.1-sol`,
  effort high, launched as the `codex-review` skill says (a disposable clone under `target/review-7d3b/` with a copy
  of the Cargo registry, `workspace-write`, no network, a user scope capped at 3 GB). No subagent.
- Start: HEAD `3de45ac`, with the maestro's fermento edit (the two proposals already applied, removed) not committed.
  It went in its own commit, `230ff56`, with no AI co-author; the answer was then replaced (`cf21056`), the separator
  found exactly once and the input byte-identical: no current recommendation.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, this file (D-053, D-062 to D-068, R-031, R-033), `core-specification.md`
  (sections 2 TASK, RUN, GATE-7 to GATE-9, SEC-2, the CLI paragraphs of 7d1 to 7d3a, 3 to 5, 11.2, 11.4), the skills
  `verify-part` and `codex-review`, and the code of the table, the payloads, the write path, `adopt`, `propose`,
  `status`, the declaration and the catalogues, with their tests.
- Decided before any code (D-068): `run`'s `BASE_MOVED` and REQ-RUN-2 from phase 9; `authorize` without `--seen` a
  usage error; `task-rejected` bound to its task and head, `MERGE_IN_PROGRESS` closed by default until phase 9; the
  wildcard guard at both authorizations. After the measure: D-069.
- Written: `crates/core/src/cli/tasks.rs` (what an authorization binds and its rendering, `authorize`, `reject`,
  `run` up to confinement); `status <id>`'s rendering with its digest and next steps (`fabrewk.subject/2`); the table's
  two authorization rows with `PROTECTED_PATH_WILDCARD`, `run-started` with `say`, the replay binding
  `task-rejected`; `shown` in both authorizations' payloads and `task-rejected`'s payload (`task.rs`); three codes
  (`CONFINEMENT_UNAVAILABLE`, `DEPENDENCY_OPEN`, `MERGE_IN_PROGRESS`); `--seen`, `--run` and the three commands in
  the declaration, with the catalogue's `flag.then-run` (a field named `run` holds a command); `Cx::lang_in`;
  `text::printable`, `hidden`, `escape`, and `say` filling in one pass; the catalogues in `en` and `es`; the generated
  completions and reference; tests in `crates/fabrewk/tests/tasks.rs` (24) and `faults.rs` (one more, 9), the
  fixtures' `shown`; `core-specification.md` (sections 3 to 5, the paragraph "The task commands, as part 7d3b fixed
  it"), `plan.md`, `STATUS.md`, `verify-part` (D-069), this entry.
- Codex, three passes, confirmed blocking findings by pass: 2, 1, 0. Pass 1 (`ea8374b`): P7D3B-1 (the rendering was
  worded in the language read before the lock, so a policy adopted or the personal file changed in that window let
  `authorize` sign a stale digest), P7D3B-2 (goals printed raw in human outputs, so a control sequence could hide the
  rendering that follows); P7D3B-3 not blocking (`say` filled a placeholder held in a value already filled). Pass 2
  (`e55b3d5`): P7D3B-1 to P7D3B-3 closed; P7D3B-4 (an agent's tool printed raw in `status`'s list of events, before
  the rendering), corrected at the sink: every human output is made printable where it is written. Pass 3
  (`29c4f25`): **accept**, no new finding. Each new test failed without its correction (P7D3B-1 in both its ways).
  Codex wrote its reports in Spanish and ran nothing outside its clone; its sandbox cannot bind an SSH agent's socket,
  so it skipped that one test, which passes here.
- Clean clone of `29c4f25`, rustup 1.99.1: 324 tests green, and the fault suites (`crash` 3, `faults` 9); the musl
  release build, whose `--version --json` lists no feature; `budget check --binary` exits 0: core 5,859 of 7,500
  counted lines in 34 files, 4 direct runtime crates, 31 crates in the release build, the binary 1,773,928 bytes;
  `clippy --workspace --all-targets -D warnings` with and without `fabrewk-core/fault-injection`, `fmt --check`,
  `cargo deny --offline check licenses advisories` clean.
- Mutation, with `--features fabrewk-core/fault-injection` and `--test-workspace true`:
  - A first run at `24ae24e` (184 mutants: 110 caught, 74 unviable, none missed) is void: the fault suite's fixture
    lacked `shown`, so `faults` failed on every mutant and the baseline, built for the core alone, did not show it.
    The integrator had skipped `verify-part`'s fault-suite step before mutating. Corrected in `5f20c1a`.
  - The valid run at `5f20c1a`: 184 mutants, 103 caught, 73 unviable, no timeout, 8 missed. After 183 a compiler held
    at the cap per process (`ulimit -v`, 999,664 kB of virtual memory) waited 1 h 45 min on a futex with 2 s of CPU;
    the run was stopped, its results kept, and that mutant (`walk` replaced by `Ok("")`) run alone without `ulimit -v`
    in the same scope: caught. The 8 missed, all in `tasks.rs`, were killed by `ea8374b` (the usage error's next step,
    a stage binding only its own tasks in its dependencies' order, the indentation of a task and of a stage's tasks,
    a task authorized alone carrying its stage's layer, `--run` on a task, `authorize`'s document `id`, the widened
    line); a run over them, also without `ulimit -v`: 9 of 9 caught.
  - Over pass 1's corrections: 68 mutants, 24 caught, 42 unviable, 2 timeouts (`open + 1` in `say` made `- 1` or
    `* 1`, a loop), none missed. Over pass 2's: 16 mutants, 15 caught, 1 unviable. No equivalent declared.
- Residuals, not blocking: a refusal is worded in the language read before the lock (its code is what is held); the
  personal file is not under the record's lock, so a change after the second plan is not seen; JSON documents carry a
  bidirectional mark as is (JSON escapes only control characters); the SSH agent test cannot run in Codex's sandbox.
- Size: the core grew from 5,377 to 5,859 counted lines (+482; R-031 projected +450 to +650). Projection: phase 8 as
  R-029 estimated it (+800 to +1,100), about 6,650 to 6,950; the core's part of phase 9 (+1,000 to +1,400), about
  7,650 to 8,350. The maestro kept the ceiling at 7,500 and decides it again when phase 9 starts (D-069).
- Not done: phase 8.

## D-069 — Part 7d3b measured: the core's ceiling kept until phase 9; a build timeout in `verify-part`

- Status: decided
- Decided: 2026-11-10
- Origin: maintainer XEL in session 25's chat, answering the integrator's two questions after part 7d3b was accepted
  and measured, as R-029 and R-031 asked: "Mantener; decidir en fase 9 (Recomendado)"; "Aprobar (Recomendado)".
- Decision:
  1. The core ring's ceiling stays at 7,500 counted lines (D-054). Measured at 5,859 after 7d3b, the projection
     (R-034) puts it at about 6,650 to 6,950 after phase 8, which fits, and 7,650 to 8,350 after the core's part of
     phase 9, which likely does not. The maestro decides the ceiling again when phase 9 starts, with phase 8's
     measure. The alternatives offered were 8,500 or 9,000 now.
  2. `verify-part`'s two mutation commands gain `--build-timeout 600`, and the skill says that a build that reaches
     the cap per process can wait forever instead of failing (R-034), so that a mutant whose build times out is run
     again alone (`--re`, `-j 1`, in the same scope without `ulimit -v`, the scope's 3 GB still capping it) before it
     is counted. The rerun's detail (`-j 1`, without `ulimit -v`) is the integrator's, added after the answer from
     how this session's run was recovered; the maestro may undo it. The alternative offered was a lesson in R-034
     only.
- Consequences: `.brewstillery/skills/verify-part/SKILL.md`; `tools/budget` unchanged.
- Sources: chat of session 25; D-042, D-054, D-060; R-029, R-031, R-034.
- Supersedes: none.

## D-070 — Phase 7 closed

- Status: decided
- Decided: 2026-11-10
- Origin: maintainer XEL in session 25's chat, answering the integrator's question after part 7d3b was accepted:
  "Cerrar la fase 7 (Recomendado)".
- Decision: phase 7 (core, part 1) is closed. Its parts 7a, 7b, 7c1, 7c2a, 7c2b, 7d1, 7d2, 7s, 7d3a and 7d3b are done
  (R-023 to R-031, R-034), each meeting the plan's acceptance for row 7: tests green in a clean clone, the past-defect
  cases its part covers held, every mutant caught or declared equivalent, within the size budget (D-042, D-054,
  D-069), and Codex's review accepted. Phase 8 (core, part 2) is next.
- Consequences: `plan.md` and `STATUS.md`. What phase 7 left to phase 8 stays where the record put it: C5, C6, C7 and
  C12 and the `MOVE_NOT_ALLOWED` tests of `approve` and `close` (D-053 points 1 and 3), `ALAMBIQUE_UNAPPROVED` and the
  views that say who acts next, `close`, `sweep`, `approve` and `discard`, and D-061's requirement, to propose when
  phase 8 starts.
- Sources: chat of session 25; `plan.md` row 7 and "Phase 7"; D-041, D-053, D-061, D-069; R-034.
- Supersedes: none.

## D-071 — Phase 9 opens with Claude Fable 5.2, as a calibration

- Status: decided
- Decided: 2026-11-10
- Origin: maintainer XEL in session 25's chat, after phase 7 closed, asking whether Claude Fable 5.2 is worth using
  for the plan's harder design; the integrator recommended a single calibration rather than a standing role, and XEL
  answered: "Estoy de acuerdo con tus recomendaciones, dejemos la fase 9 como nuestro 'piloto' de calibración para el
  nuevo Fable 5.2", authorizing the change without the fermento (it is the plan's, not the alambique's).
- Decision:
  1. Phase 9 (adapters and confinement) opens with Claude Fable 5.2 (`claude-fable-5-2`), effort high: its opening
     session makes the threat model, the confinement backend's design and the cut into parts, before any code. The
     parts' code runs on Opus 5.5, effort high, unless that design says a part (the confinement backend, most likely)
     needs Fable too. Codex (GPT-6.1 Sol), effort high, reviews, as in phase 7: the reviewer stays another vendor's.
  2. It is measured as every part is: the confirmed blocking findings per Codex pass, against phase 7's record (most
     parts accepted at the second or third pass; 7a and 7d2 at the tenth, 7c1 at the seventh, 7d3a at the sixth, on
     adversarial edges of the file system, concurrency and fail-closed checks). Without a clear gain, phase 10 and
     later stay on the plan's models. The phase 9 closing reports the comparison, and Fable's weekly allotment as
     it was spent, for the maestro to decide whether Fable keeps a role.
  3. Phases 8, 10 and 11 keep their models: their design is fixed by the frozen specification, and phase 7's cost
     came from edge cases a review finds, not from the architecture. Fable is not used as a reviewer.
- Reason: Fable 5.2 is Anthropic's most capable widely released model; at the API's prices it costs twice Opus 5.5
  per token ($8/$40 against $4/$20 per million) and its turns run longer; on the maestro's plan its use is metered
  apart, weekly, so the risk is the allotment running out mid-session rather than the Opus allotment. Phase 9 is the
  product's security core and the one design the specification leaves open (a confinement backend per operating
  system), where better reasoning may save review passes.
- Consequences: `plan.md` row 9's model; `STATUS.md`'s "Next". The prompts written for Opus may be too prescriptive
  for Fable; the opening session of phase 9 states the task and its acceptance and lets the model plan.
- Sources: chat of session 25; `plan.md` rows 7 to 11; R-023 to R-034; the `claude-api` skill's model table (cached
  2026-11-03).
- Supersedes: `plan.md` row 9's "Opus 5.5 high; Codex high" for its opening session.

## D-072 — Phase 8 cut into 8a, 8b and 8c; the skills outside the project in `sweep-recorded` (REQ-FER-10)

- Status: decided
- Decided: 2026-11-10
- Origin: maintainer XEL in session 26's chat, answering the integrator's two questions before any code: "Tres partes
  (Recomendado)"; "REQ-FER-10 en el evento (Recomendado)". The integrator had read the specification's phase 8
  (FER-1 to FER-9, VIEW-1 to VIEW-3, TASK-6, C5, C6, C7, C12, PD-33, PD-39 to PD-43) and estimated +900 to 1,100
  counted lines against the 5,859 of 7,500 (D-069).
- Decision:
  1. Phase 8 is split in three, each with its own Codex review. 8a, the fermento and its answer: the separator
     finder and the points (REQ-FER-1, REQ-FER-2), the proposals in the replay and the table (pending, withdrawn,
     incorporated, discarded; `stale` derived), `sweep` and `discard` (REQ-FER-3, REQ-FER-4, REQ-FER-8's half in the
     fermento), the answer zone generated from the events in the maintainer's language, REQ-FER-9, REQ-FER-10, C6 and
     C12, PD-41's `sweep` half. 8b, the alambique: `approve` and its edits (REQ-FER-5, REQ-FER-6, `rules` proposals
     in the stream `rules`), `alambique-baseline` and the digest of the last approved text, `direct-edit-approved`,
     REQ-FER-7, REQ-FER-8's half in the alambique, C7 and `ALAMBIQUE_UNAPPROVED` at `run`, `accept` and `close`, PD-41's
     `approve` half. 8c, the views and the stage's close: `STATUS.md` and `RULES.md` generated, regenerated by every
     write command, `VIEW_STALE` and C5 (REQ-VIEW-1 to REQ-VIEW-3), `close` (REQ-TASK-6), who acts next, PD-33's
     views, PD-39, PD-40, PD-42, PD-43. The tests of `MOVE_NOT_ALLOWED` for `approve` of a discarded proposal and
     `close` of a closed stage go with the part that builds the command (D-053 point 3).
  2. REQ-FER-10, new (D-061): a sweep records in `sweep-recorded` the names of the skills the tools load from
     outside the project, from the user folders each module's contract declares (fake modules until phase 9), never
     their content; the answer shows them from the event (so it still comes from the replay alone, PD-42), warns for a
     name equal to an approved skill's, and says "unknown" for a folder it cannot list. Whether one contradicts the
     project's rules is the integrator's to say in its recommendation. Fabrewk touches none. It is built in 8a.
- Consequences: `core-specification.md` (REQ-FER-10, `sweep-recorded`'s payload, section 12, section 13); `plan.md`
  ("Phase 8"). Parts 8a to 8c run on Sonnet 5.5, effort high; Codex (GPT-6.1 Sol), effort medium as row 8 says, reviews
  each. The base rules `RULES.md` is generated from are not written yet; their text is drafted for the maestro's
  approval when 8c starts. The alternatives offered were two parts, one undivided phase, a fixed list of user
  folders in the core (ties it to two tools and bypasses D-052's contract) and leaving the report to phase 10.
- Sources: chat of session 26; `STATUS.md`'s "Next"; D-019, D-053, D-061, D-069, D-070; `core-specification.md`
  sections 2 FER, VIEW, TASK-6, 3 to 6, 11.3 to 11.5, 12, 13.
- Supersedes: none.

## R-035 — Session 26, part 8a: the fermento and its answer

- Status: recorded
- Date: 2026-11-10
- Who: integrator Claude Code, `claude-sonnet-5-5`, effort high. Reviewer: Codex `codex-cli 0.166.0`, `gpt-6.1-sol`,
  effort medium (plan row 8), launched as the `codex-review` skill says (a disposable clone under `target/review-8a/`
  with a copy of the Cargo registry, `workspace-write`, no network, a user scope capped at 3 GB). No subagent.
- Start: HEAD `dfceb6c`, tree clean.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, this file (D-053, D-061, D-062 to D-071, R-031, R-034),
  `core-specification.md` (sections 2 FER, VIEW, TASK-6, 3 to 6, 11.2 to 11.4, 12, 13), the skills `verify-part` and
  `codex-review`, and the code of the table, the write path, the declaration, the catalogues and `check`, with their
  tests.
- Decided before any code (D-072): the cut into 8a, 8b and 8c, and REQ-FER-10 recorded in `sweep-recorded`.
- Written: `crates/core/src/fermento.rs` (the separator finder outside fences, the points and their digests, the
  payload schemas of `sweep-recorded` and `proposal-*`, the proposals' states derived from the record, the answer zone
  generated from it, the skills listing); `crates/core/src/cli/sweep.rs` (`sweep`, `discard`); the view step of the
  write path (`Plan.view`, `INPUT_CHANGED` checked before the events and again before the file); `check`'s C6 and
  C12; the declaration of the two commands and of `--answer`; ten codes (`ANSWER_INVALID`, `FERMENTO_MISSING`,
  `INPUT_CHANGED`, `INPUT_UNCOMMITTED`, `MAINTAINER_UNLISTED`, `POINT_CHANGED`, `PROPOSAL_UNKNOWN`,
  `SEPARATOR_AMBIGUOUS`, `SEPARATOR_DAMAGED`, `SEPARATOR_MISSING`); the catalogues in `en` and `es`; the generated
  completions and reference; tests in `crates/core/tests/fermento.rs` and `crates/fabrewk/tests/fermento.rs` (with two
  property tests against a reference fence scanner) and two in `faults.rs`; `core-specification.md` (REQ-FER-10, the
  new codes, REQ-REG-1's proposal identifier), `plan.md`, `STATUS.md`, D-072, this entry.
- Interpretations the maestro may want to read (none contradicts a requirement):
  1. `ANSWER_INVALID` is new: an answer file of the wrong shape or content (the same shape in a recorded payload stays
     `RECORD_VERSION_UNKNOWN`). To ratify.
  2. The agent that makes `sweep-recorded` and `proposal-recorded` is the one the answer file names (`agent`: tool,
     provider, model; `modelObserved` null); `proposal-withdrawn` is made by `fabrewk`.
  3. A proposal's identifier is `p-` and the first 8 hexadecimal digits of the digest of its fermento's alias and its
     payload without `id`, not of a payload with a nonce: the same edit swept again is the proposal already recorded
     (REQ-REG-1 says so now). A known identifier with another alias or another payload is `RECORD_ID_COLLISION`.
  4. "Stale" is derived (the point's number or digest differs from the last sweep's), never recorded; a pending
     proposal is withdrawn when no point of the input bears its number.
  5. `discard` rewrites nothing: the next sweep regenerates the answer. Until 8b the answer names only `discard`.
  6. "Input committed" is the digest of the input in the record branch's tip equal to the working file's, and, since
     pass 1, the person's index holding the committed fermento as well (a staged edit is theirs). It is checked only
     where the mode commits.
  7. A line break is `\n` or `\r\n`; the command the answer prints for a discard carries no words (PD-32).
  8. `Program.skills` (folders relative to the home, empty for the fake modules) joined the module contract of D-052.
- Codex, three passes, confirmed blocking findings by pass: 4, 1, 0. Pass 1 (`871254f`): P8A-1 (a fence closed by a
  CRLF line hid the separator), P8A-2 (the record commit overwrote the person's staged edit of the fermento while the
  working copy equalled HEAD), P8A-3 (the identifier ignored the alias, so a second maintainer's identical edit
  vanished), P8A-4 (the printed discard command carried placeholder words that signed as real ones). Pass 2
  (`1066ceb`): all four closed; P8A-5 (a hash collision across aliases taken for a repeated sweep) confirmed and
  corrected; P8A-6 (a sweep after the identifier's scheme changed duplicates an edit recorded with the earlier scheme)
  not confirmed: it needs a record made by an intermediate commit of this part, and no record outside throwaway
  fixtures exists (the repository has no `data/`). Pass 3 (`cf3db8d`): **accept**, P8A-6 not sustained. Each new test
  failed without its correction (run with the sources stashed, and by Codex on its own variants). Codex wrote its
  reports in Spanish; its sandbox cannot bind an SSH agent's socket, so it skipped that one test, which passes here.
- Clean clone of `cf3db8d`, rustup 1.99.1: 358 tests green, and the fault suites (`crash` 3, `faults` 11); the musl
  release build, whose `--version --json` lists no feature; `budget check --binary` exits 0: core 6,662 of 7,500
  counted lines in 36 files, 4 direct runtime crates, 31 crates in the release build, the binary 2,036,072 bytes;
  `clippy --workspace --all-targets -D warnings` with and without `fabrewk-core/fault-injection`, `fmt --check`,
  `cargo deny --offline check licenses advisories` clean.
- Mutation, with `--features fabrewk-core/fault-injection` and `--test-workspace true`, `--build-timeout 600`:
  - At `5865455`: 318 mutants, 216 caught, 87 unviable, 13 missed, 2 timeouts. The 15 were killed by tests (the
    heading's tab, the shape and content of each fermento kind in `check`, the replay's mapping of a bad sweep, an
    alias other than the proposal's, a sweep naming no point with a proposal waiting, an unreadable fermento path
    that is not a missing one, the dedup of a decided proposal and of an identifier another alias holds, and C6 and
    C12 looking only at maintainers' directories); a run over them: 59 mutants, 36 caught, 22 unviable, one missed,
    killed by one more test and run again alone: 2 of 2 caught. Over the corrections of pass 1 and 2: 27 mutants, 19
    caught, 8 unviable. No equivalent declared.
  - Lesson: with `ulimit -v 1000000` (`verify-part`) the first baseline build of the larger core failed for lack of
    memory; the run used 1,600,000 kB. `verify-part` should say so; a skill is the maestro's to change.
- Size: the core grew from 5,859 to 6,662 counted lines (+803) with part 8a alone. R-034 projected phase 8 as a
  whole at +800 to +1,100 (6,650 to 6,950); with 8b and 8c still to build the core will likely pass 7,300, and the
  core's part of phase 9 (+1,000 to +1,400) will exceed 7,500. The maestro decides the ceiling when phase 9 starts
  (D-069); this measure is for that decision.
- Not done: parts 8b and 8c; the base text of `RULES.md` (to draft for the maestro's approval when 8c starts).

## D-073 — Part 8b: an absent baseline, the alambique committed alone also for a direct edit, the difference as one hunk, and `ALAMBIQUE_UNAPPROVED` before confinement

- Status: decided
- Decided: 2026-11-11
- Origin: maintainer XEL in session 27's chat, answering the integrator's four questions before any code: "Sin
  archivo pasa; con archivo, no aprobado (Recomendado)"; "Fabrewk lo commitea igual (Recomendado)"; "Un tramo: líneas
  quitadas y puestas (Recomendado)"; "Antes de la confinación (Recomendado)". The integrator had read the
  specification's FER, section 4's proposal moves and the code of 8a, the write path and `check`, and estimated +450
  to +600 counted lines against the 6,662 of 7,500 (D-069, R-035).
- Decision:
  1. The text last approved is the baseline's digest, an incorporated proposal's result or a direct edit's new digest,
     whichever is last. With no baseline: no file and nothing approved passes C7; a file nothing approved is
     `ALAMBIQUE_UNAPPROVED` ("never approved"). `approve alambique` records the baseline when there is none (the first
     text approved; `apply`, phase 10, will write it first), and otherwise a `direct-edit-approved`.
  2. `approve alambique` also commits the alambique alone, as `approve <proposal>` does (REQ-FER-5): through a temporary
     index, the author the person's Git names, no co-author, before the record commit; no such commit when the record
     branch and the index already hold the file as it is (a direct edit the person committed).
  3. `direct-edit-approved` records the difference as one hunk: the first line that differs, the lines removed and
     those put, the equal ones at both ends left out. The old text is the record branch's when it is the text approved;
     when it is not (`unpublished`, or a branch without it) `removed` is `null` and `added` holds all the new lines.
  4. `ALAMBIQUE_UNAPPROVED` is evaluated by `run` before `CONFINEMENT_UNAVAILABLE` (section 4's row amended), so that
     the guard is reached today and tested through the command; `accept` (phase 9) and `close` (8c) call the same
     function.
- Consequences: `core-specification.md` (REQ-FER paragraph "as part 8b fixed them", the `run-started` row, the
  `proposal-incorporated` guards with `ALAMBIQUE_UNAPPROVED`, the `approve` row, the new code `ALAMBIQUE_MISSING`);
  `plan.md` (row 8b). The alternatives offered: `CHECK_UNKNOWN` with no baseline, or C7 skipped until `apply`; the
  person commits a direct edit and `approve alambique` demands it done; the difference as the two digests alone (Git
  keeps the text) or the whole new text; the guard where the table put it, tested through its function.
- Sources: chat of session 27; `STATUS.md`'s "Next"; D-053 point 3, D-065, D-068, D-069, D-072;
  `core-specification.md` sections 2 FER, 3 to 6, 12.
- Supersedes: none.

## R-036 — Session 27, part 8b: the alambique

- Status: recorded
- Date: 2026-11-11
- Who: integrator Claude Code, `claude-sonnet-5-5`, effort high. Reviewer: Codex `codex-cli 0.166.0`, `gpt-6.1-sol`,
  effort medium (plan row 8), launched as the `codex-review` skill says (a disposable clone under `target/review-8b/`
  with a copy of the Cargo registry, `workspace-write`, no network, a user scope capped at 3 GB). No subagent.
- Start: HEAD `cfb54b4`, tree clean.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, this file (D-072, R-035), `core-specification.md` (FER, sections 3 to 7 and
  12), the skills `verify-part` and `codex-review`, and the code of the table, the fermento, the write path, `check`,
  the declaration and the fixtures.
- Decided before any code (D-073): the baseline rule, the commit of the alambique for a direct edit, the difference
  as one hunk, `ALAMBIQUE_UNAPPROVED` before confinement.
- Written: `crates/core/src/alambique.rs` (units replaced whole, the rules and their digest, the text last approved,
  C7's function, `unchanged`, the difference, the replay's checks of the streams `alambique` and `rules`);
  `crates/core/src/cli/approve.rs`; the write path's alambique step (`Plan.alambique`, guards before the machine key,
  `Session::replayable`, `Session::commit_alone`, `commit_record` taking its message); payload schemas of
  `alambique-baseline`, `direct-edit-approved`, `proposal-incorporated`; C7; the `run` guard; the approve line of the
  fermento's answer; five codes (`ALAMBIQUE_CHANGED`, `ALAMBIQUE_MISSING` (new, to ratify), `ALAMBIQUE_UNAPPROVED`,
  `PROPOSAL_STALE`, `UNIT_AMBIGUOUS`); catalogues in `en` and `es`; generated completions and reference; tests in
  `crates/core/tests/alambique.rs`, `crates/fabrewk/tests/alambique.rs` and six in `faults.rs`; the paragraph "The
  alambique and the rules, as part 8b fixed them" and the rows it touches in `core-specification.md`; `plan.md`,
  `STATUS.md`, D-073, this entry.
- Interpretations the maestro may want to read (none contradicts a requirement):
  1. `ALAMBIQUE_MISSING` is new: `approve alambique` finds no regular file, or not valid text. To ratify.
  2. `ALAMBIQUE_UNAPPROVED` joined the `proposal-incorporated` guards, after `ALAMBIQUE_CHANGED`, for an alambique
     target: an edit drafted from text nobody approved would approve that text whole.
  3. A proposal is stale against the fermento as the file stands at `approve`, not against the last sweep; "the
     drafted text changed" is the units not giving the predicted result.
  4. `keys` is shown, not checked: the predicted digest binds the whole text.
  5. The rules are `R<n>`; their digest is that of their canonical JSON (`{}` for none); the `rules` event carries all
     the rules after it; the replay holds both streams to their proposals (`RECORD_TRANSITION_INVALID`). `RULES.md`
     follows in 8c.
  6. A unit spans from its first line to the end of its last non-blank line; a section ends at any heading of level 1
     or 2 (up to three spaces of indentation, a tab, or empty); a point at the next point or any heading.
  7. Order: event, file, the alambique's commit, the record commit. Every guard that can refuse, `ALAMBIQUE_CHANGED`
     (file, tip and index) among them, is evaluated before the machine key is put in place and before leftovers are
     committed; the replay runs before the alambique's commit. A process killed between the event and the file leaves
     C7 reporting the file; `approve alambique` is the recovery. An index the alambique's commit could not follow is
     reported with the commit made and the command `git -C <repository> add <file>` (in words when a path is not plain).
- Codex, three passes, confirmed blocking findings by pass: 5, 1, 0. Pass 1 (`b611049`): P8B-1 (the index not checked
  again before the file is written), P8B-2 (a section ended only at `# ` or `## `), P8B-3 (a refusal after the machine
  key was put in place), P8B-4 (an index the commit could not follow was left as a reversion), P8B-5 (the replay failed
  after the alambique's commit). Pass 2 (`1ba880e`): all five closed; P8B-6 (the fix printed `git add` for the wrong
  repository in `own-repository`). Pass 3 (`9399f8d`): **accept**. Each new test failed without its correction (the
  five of pass 1 run with the sources stashed). Codex wrote its reports in Spanish; its sandbox cannot open an SSH
  agent's socket, so it skipped that test, which passes here.
- Clean clone of `2debde4`, rustup 1.99.1: 380 tests, `crash` 3 and `faults` 17 green; the musl release build, whose
  `--version --json` lists no feature; `budget check --binary` exits 0: core 7,135 of 7,500 counted lines in 38 files,
  4 direct runtime crates, 31 crates in the release build, the binary 2,167,144 bytes; `clippy --workspace
  --all-targets -D warnings` with and without `fabrewk-core/fault-injection`, `fmt --check`, `cargo deny --offline
  check licenses advisories` clean. A first clean clone with `CARGO_TARGET_DIR` set failed one test of the counter
  (it expects the clone's own `target/`): leave it unset.
- Mutation, with `--features fabrewk-core/fault-injection`, `--test-workspace true`, `--build-timeout 600`,
  `ulimit -v 1600000`: at `9b69d90`, 197 mutants, 151 caught, 42 unviable, 3 missed (C7 with a record that does not
  replay) and 1 timeout (C7's reason for a file gone), all killed by tests (8 of 8 caught on a rerun). Over pass 1's
  corrections: 34 mutants, 24 caught, 9 unviable, 1 missed (the plain-path rule of the printed fix), killed by a
  test; over `commit_alone` and `replayable`, 9 of 9 caught. No equivalent declared. One baseline build failed once
  with `LOCK_HELD` in an 8a test and was not reproduced in five reruns or in the next baseline.
- Size: the core grew from 6,662 to 7,135 counted lines (+473) with part 8b. Phase 8 with 8c (the views, `close`,
  `VIEW_STALE`, who acts next; projected +300 to +500) will likely pass 7,500 before phase 9 starts: the ceiling is the
  maestro's to decide (D-069), and 8c should not start without that decision if it will cross.
- Not done: part 8c; the base text of `RULES.md` (to draft for the maestro's approval when 8c starts); `accept`
  (phase 9) and `close` (8c) call `alambique::check`.

## D-074 — Part 8c: the core's ceiling at 8,000, the base text of `RULES.md`, and the command names left for the rebuild

- Status: decided
- Decided: 2026-11-11
- Origin: maintainer XEL in session 28's chat, answering the integrator's two questions before any code: ceiling
  "8,000 ahora (Recomendado)", and on the base rules "De acuerdo en todo", with a new wording of the first rule and a
  proposal on command names (quoted below). The integrator had read the specification's VIEW, TASK-6 and section 4,
  the code of `status`, `check`, the write path, the table and the fermento, and estimated +300 to +500 counted lines
  against the 7,135 of 7,500 (R-036).
- Decision:
  1. The core's ceiling is 8,000 counted lines (`tools/budget/src/budget.rs`, its tests). Phase 9 keeps its own
     decision when it starts (D-069).
  2. The base text of `RULES.md` is the twelve rules the integrator drafted (authority; start at `STATUS.md`; the
     alambique; the fermentos; cited material; settings; a person's moves; commands; the domain; languages; reports;
     dispatched agents), in the binary's catalogue in `en` and `es`, with the first rule worded as the maestro said:
     the highest authority is what Git permits (on a local host the user owns their machine; in what is published,
     whoever holds the push permission); the alambique with the fermentos is the workbench that gives the plan its
     authoritative form, so it may change and must always agree with what the plan describes as authority; the chat is
     the most direct channel for extraordinary changes and stands above them, so a maestro may ask an agent in chat
     for a change with express authorization, but the change is always sketched in the alambique (through a
     fermento or not) so that it enters the plan as authority; an agent makes no change that is not signed as the
     maestro's authorization. The root `AGENTS.md` of this repository keeps its own "Authority" line until production.
  3. Command names are not changed in the bootstrap. The maestro proposes, for the rebuild after the pilot (6.53),
     that `sweep` be named after the analogy "fermento -> alambique -> destilado" (for example `distill`); it is
     recorded here and not in the alambique, which the maestro edits when they decide it.
- Consequences: `tools/budget` (ceiling, tests, the comment); `crates/core/catalogue` (the base rules); `plan.md`
  (row 8c); `STATUS.md`. The alternatives offered for the ceiling: 8,500, or keep 7,500 and stop at the measure.
- Sources: chat of session 28; `STATUS.md`'s "Next"; D-054, D-069, D-072, D-073; `core-specification.md` REQ-VIEW-2.
- Supersedes: none.

## R-037 — Session 28, part 8c: the views, close and who acts next

- Status: recorded
- Date: 2026-11-11
- Who: integrator Claude Code, `claude-sonnet-5-5`, effort high. Reviewer: Codex `codex-cli 0.166.0`, `gpt-6.1-sol`,
  effort medium (plan row 8), launched as the `codex-review` skill says (a disposable clone under `target/review-8c/`
  with a copy of the Cargo registry, `workspace-write`, no network, a user scope capped at 3 GB). No subagent.
- Start: HEAD `2f28b3b`, tree clean.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, this file (D-069, D-071 to D-073, R-036), `core-specification.md` (VIEW,
  TASK-6, FER, sections 3 to 7, 11.2, 12), the skills `verify-part` and `codex-review`, and the code of `status`,
  `check`, the write path, the table, the fermento, the alambique, the declaration, the catalogues and the fixtures.
- Decided before any code (D-074): the ceiling at 8,000, the base text of `RULES.md` (twelve rules, the first as the
  maestro worded it), and the command names left for the rebuild after the pilot.
- Written: `crates/core/src/view.rs` (`status`, `rules`, `waits`, `stale`, `write`, `language`); `crates/core/src/cli/close.rs`;
  C5 in `check`; the write path writes both views once the events replay and the record commit holds them; `status`
  lists what waits and for whom (`fabrewk.status/2`, `waiting`) and offers each person only their commands; the
  `stage-closed` payload (`stage`, `head`, `sweeps`) and its binding in the replay; `fermento::unswept` and
  `first_complete`; the codes `STAGE_OPEN_TASKS`, `SWEEP_MISSING`, `VIEW_STALE`; the catalogues (`view`, `view.wait`,
  `rules.base`, the codes, `close`, `usage.task`); generated completions and reference; `git.rs::attempt` checks that a
  moved branch descends before it looks at the files (a view in the commit made a rewind look like a race);
  `write_atomic` removes what stands at its temporary name and creates anew; the lock is opened without following a
  link; tests in `crates/fabrewk/tests/views.rs`, `crates/core/tests/table.rs`, `faults.rs`, `writing.rs`; the
  paragraph "The views and the stage's close, as part 8c fixed them" in `core-specification.md`; `plan.md`,
  `STATUS.md`, D-074, this entry.
- Interpretations the maestro may want to read (none contradicts a requirement):
  1. An absent view is `VIEW_STALE`; a record that does not replay leaves C5 `CHECK_UNKNOWN`. The fixtures that write
     events through the library now also write the views (`Fx::regen`). This repository has no `data/`, so nothing
     here is generated; its `STATUS.md` stays the integrator's.
  2. The views are in the policy's canonical documentation language (`en` with no policy), not the reader's.
  3. What waits: a proposed stage; a task proposed into an authorized stage; an authorized task (run, reject); a
     halted or verified task (reject; `accept` is phase 9's); a pending proposal; and for a stage whose tasks have all
     ended, one sweep per maintainer whose last sweep did not see every task's last event (the integrator) and, when
     none is missing, the close (a maestro); a stage with no task waits for its close from its authorization. `since` is
     the time of the event that made all of it true (the latest of the tasks' ends, the stage's authorization and the
     first sweep that saw every end), so a later sweep does not restart a wait.
  4. `status` offers a maestro the maestros' commands, anyone who is no maintainer the integrator's, and a maintainer
     with nothing to decide none. No wait names `authorize` (it needs the digest `status <id>` shows) nor `accept`.
  5. `SWEEP_MISSING` takes every maintainer of the policy in force; a sweep counts when its recorded heads hold the last
     event of every task of the stage; a stage with no task needs none. `stage-closed` binds the stage and its head, as
     a rejection does, and lists each relied-on sweep.
  6. The usage section reads `payload.usage.tokens` of `attempt-ended` (PD-39): unknown if any attempt did not report
     or has not ended or the sum does not fit, zero only when reported. Phase 9 fixes the record's shape and adjusts it.
  7. Rule 1 of `RULES.md` is the maestro's wording; the root `AGENTS.md` keeps its own "Authority" line until production.
  8. The command whose next step is `fabrewk sweep --help` is the integrator's wait only; `SWEEP_MISSING`'s own next
     step is `fabrewk status` (a refusal's `run` must parse as a run).
- Codex, four passes (medium), confirmed blocking findings by pass: 3, 2, 1, 0. Pass 1 (`b73c06b`): P8C-1 (a link at a
  view's temporary name was followed, writing outside the domain), P8C-2 (the close's wait began at the task's end, not
  at the sweep), P8C-3 (a stage with no task waited for nothing); P8C-4, not blocking (a vacuous assertion). Pass 2
  (`33f6042`): P8C-5 (a later sweep restarted the wait), P8C-6 (the lock was opened through a link). Pass 3
  (`e569cee`): P8C-7 (the same, replaced between the check and the open). Pass 4 (`dfa4ea1`): **accept**, with P8C-8
  not blocking. Each new test failed without its correction (checked by hand for every one but P8C-7's first
  variant). P8C-8, residual: a directory above the lock (`local`) swapped during the instant between the check and
  the open lets the lock be taken, or created, at a place outside the domain; closing it takes `openat` with
  `O_DIRECTORY | O_NOFOLLOW` per component, from a crate (`rustix` or `nix`) that no maestro approved (AD-4). Left to
  the maestro.
- Mutation, with `--features fabrewk-core/fault-injection`, `--test-workspace true`, `--in-diff`, `ulimit -v
  1600000`: the first run, 195 mutants at `14503b3`: 99 caught, 80 unviable, 4 missed (the proposals' wait) and 12
  timeouts; run again over the 4 and the 12 alone (`-j 1`, 240 s): 17 caught and 8 missed (the proposals' wait, C5's
  unknown record, the linked domain, the `stage-closed` shape, the close's usage command, the halted wait's start);
  every one killed by a test or removed by simplifying the code; the later runs over the corrections (21, 2, 26, 3
  mutants) left 2, 0, 3 and 0 missed, each killed or removed the same way. None declared equivalent.
- Clean clone of `a1bb6bb`, rustup 1.99.1: 404 tests, `crash` 3 and `faults` 19 green; the musl release build, whose
  `--version --json` lists no feature; `budget check --binary` exits 0: core 7,630 of 8,000 counted lines in 40 files,
  4 direct runtime crates, 31 crates in the release build, the binary 2,363,752 bytes; `clippy --workspace
  --all-targets -D warnings` with and without `fabrewk-core/fault-injection`, `fmt --check`, `cargo deny --offline
  check licenses advisories` clean. Codex's sandbox cannot open an SSH agent's socket, so it skipped
  `signing_with_a_key_literal_in_the_agent`, which passes here.
- A flake that predates the part: `req_fer_5_…` and `req_fer_7_…` of `crates/core/tests/alambique.rs` failed with
  `LockHeld` about once in 25 runs (once in 80 at `2f28b3b`): another test's `Fixture::new` runs `git` or `ssh-keygen`,
  and its `fork` copies the lock's descriptor for an instant. The tests' `put` now waits for a lock held (0 in 100).
- Size: the core went from 7,135 to 7,630 counted lines (+495) against the ceiling of 8,000 (D-074): 370 left. Phase 9's
  core (projected +1,000 or more) will cross it: the maestro decides the ceiling when phase 9 starts (D-069).
- Not done: `accept` and the rest of phase 9; `apply`, which will write the views first (phase 10); the usage record's
  shape; the rename of `sweep` (proposed for after the pilot, D-074). `verify-part`'s `ulimit -v 1000000` still does not
  fit the core's build (1,600,000 was used): the skill is the maestro's to change. An untracked `.vscode/settings.json`
  that the editor created during the session is not part of the commits.

## D-075 — Phase 9 opened: a private network for agents, the core's ceiling at 11,000, P8C-8 declared, six amendments, nine parts, Codex reviews the design, part 9a on Fable

- Status: decided
- Decided: 2026-11-11
- Origin: maintainer XEL in session 29's chat, answering the integrator's six questions on
  `.brewstillery/bootstrap/confinement-design.md` (its section 5) before any code: "B: red privada (Recomendado)";
  "11,000 (Recomendado)"; "Declararlo, sin biblioteca (Recomendado)"; "Aprobar las seis (Recomendado)"; "Sí, en esta
  sesión (Recomendado)"; "Fable 5.2 solo en 9a (Recomendado)". The integrator had probed this machine (the design's
  section 0): with the host's network shared, a confined process reaches the X server's abstract socket and the
  loopback listeners; under `pasta`, hardened, it reaches neither.
- Decision:
  1. An agent's network is a private namespace with outbound connections only, made with `pasta` (the design's 3.5,
     option B; S-1): nothing of the host's loopback, addresses or abstract sockets is reachable from a cell; where
     `pasta` is absent the dispatch refuses. The open outbound network (the local network, the Internet) and the
     tool's credential readable by its agent are residual risks that `config` declares. Outbound connections only
     through a proxy that admits the provider's hosts (option C) is designed after the pilot. If part 9a finds no
     composition of `bwrap` and `pasta` in which ending Fabrewk's direct child ends the cell (the design's 3.7), the
     choice returns to the maestro.
  2. The core's ceiling is 11,000 counted lines (`tools/budget/src/budget.rs`, its tests), measured again at each
     part; the alternatives offered were 11,500 and a raise per part. Each module of phase 9 is a ring at its kind's
     frozen ceiling (D-041, D-042).
  3. P8C-8 is closed as declared: the race on a directory above the lock needs an unconfined process of the same user,
     so the control is cooperative (REQ-SEC-5), `config` says so, and no crate is added (S-8). The alternative offered
     was a crate that opens each path component without following links.
  4. The amendments S-2 to S-7 of the design's section 4 are approved as drafted: REQ-RUN-3's rules for reading what a
     cell left; REQ-RUN-10, `DISK_LOW`, `limits.freeDiskMegabytes` and the sizes `status` shows (what D-067 left to
     this phase); `gate.protectedPaths`' default with `.gitattributes`, `.gitmodules` and each adapter's instruction
     files; `attempt-ended`'s `usage`; a required tool present only when its cell shows it; the residual risks listed
     by `config`. Each is written into `core-specification.md` by this session after Codex's review of the design, and
     built with its tests by the part the design names.
  5. Phase 9 is cut into nine parts, 9a to 9i (the design's section 6; `plan.md`, "Phase 9").
  6. Codex (GPT-6.1 Sol, effort high) reviews the design in this session, before part 9a.
  7. Part 9a runs on Claude Fable 5.2, effort high; parts 9b to 9i on Opus 5.5, effort high; Codex reviews each
     (D-071 point 1, which left this to the design).
- Consequences: `confinement-design.md`; `tools/budget` (the ceiling, two tests, the comment); `core-specification.md`
  (REQ-SEC-1, REQ-RUN-2, REQ-RUN-3, REQ-RUN-10, REQ-DOM-5, REQ-SEC-5's criterion, sections 3, 7 and 8); `plan.md`;
  `STATUS.md`. Not decided here: the permission and the budget to run the real tools (plan row 9), asked when part 9c
  starts.
- Sources: chat of session 29; `STATUS.md`'s "Next"; D-032 point 14, D-067, D-069, D-071, D-074; R-037 (P8C-8);
  `core-specification.md` RUN, GATE, SEC, EXT and section 13; the probes of the design's section 0.
- Supersedes: D-074 point 1 for the ceiling's figure; `core-specification.md` REQ-SEC-1's sentence "The shared network
  is a stated residual risk".

## D-076 — Phase 9's opening: the amendments Codex's review changed or added, approved

- Status: decided
- Decided: 2026-11-11
- Origin: maintainer XEL in session 29's chat, after Codex accepted the design at its fourth pass, answering the
  integrator's question on the two amendments the review changed and the four it added: "Aprobar las seis
  (Recomendado)".
- Decision: the amendments of `confinement-design.md`'s section 4 are approved as Codex accepted them.
  1. S-4, rewritten: a new key `gate.protectedNames` (direction N), names protected at any depth, enforced on changed
     paths at the change set and again at integration (`PROTECTED_PATH_WILDCARD`), so that `*.md` and `src/**` stay
     valid lanes; its default is `.gitattributes`, `.gitmodules` and each built-in adapter's instruction names;
     `gate.protectedPaths`' default gains the hooks directory when the repository's configuration places it inside
     the working tree.
  2. S-5, rewritten: `usage.seconds` is Fabrewk's own measure, an integer or `null` when Fabrewk did not observe the
     attempt's end; an unknown duration counts as the attempt's whole allowed time against `minutes`; REQ-RUN-7 says
     that time is Fabrewk's measure and tokens the tool's report.
  3. S-9: an extension's cell holds its program, read-only, where it was found; an interpreter or support file
     outside the system directories and the repository is absent, and the failure is `TOOL_FAILED` with words.
  4. S-10: an agent's or a check's cell writes only in `/work`, its home and its `/tmp`, the attempt's three
     directories, deleted when it ends; an extension's home and `/tmp` lie inside its own area.
  5. S-11: every Git command Fabrewk runs names an empty hooks directory and turns the file-system monitor off, and
     the working tree is written from the objects' bytes. This changes the one command builder of
     `crates/core/src/record/git.rs`, built in phase 7; part 9b makes the change with its test.
  6. S-12: a filter loaded with each cell refuses the kernel's keyring system calls, cryptographic sockets and
     `io_uring`, in the cell and in what nests in it; what another kernel interface may do with a key's serial number
     is a residual risk `config` declares.
- Consequences: `core-specification.md` (the four paragraphs "as phase 9's opening amended it" under RUN, GATE, SEC
  and EXT, with S-1 to S-3 and S-6 to S-8 of D-075; `attempt-ended`'s `usage` in section 3; `DISK_LOW` in sections 7
  and 11.4; `limits.freeDiskMegabytes` and `gate.protectedNames` in section 8; the criteria "Amended at phase 9's
  opening" in section 12; section 13); `confinement-design.md`; `plan.md`.
- Sources: chat of session 29; Codex's four reports (R-038); D-075; `confinement-design.md` section 4.
- Supersedes: D-075 point 4 for S-4 and S-5 as first drafted.

## R-038 — Session 29, phase 9's opening: the threat model, the confinement's design and the cut

- Status: recorded
- Date: 2026-11-11
- Who: integrator Claude Code, `claude-fable-5-2`, effort high (D-071). Reviewer: Codex `codex-cli 0.166.0`,
  `gpt-6.1-sol`, effort high, launched as the `codex-review` skill says (a disposable clone under
  `target/review-9-design/`, `workspace-write`, no network, a user scope capped at 3 GB; no registry copy, since
  nothing was built). No subagent.
- Start: HEAD `ae80923`, tree clean but an untracked `.vscode/`.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, `ALAMBIQUE.md`, `xel/fermento.md` (no proposal), both settings files;
  `core-specification.md` (TASK, RUN, GATE, SEC, EXT, BUD, DOM, REG-5, REG-13, SET-8, SET-9, sections 3 to 8, 11.2 to
  11.6, 12, 13); `architecture-decisions.md` AD-5, AD-6, AD-8; this file (D-032, D-067 to D-074, R-037);
  `past-defect-cases.md` PD-20 to PD-30; `harness-verdicts.md`'s observations; the skill `codex-review`; the
  workspace's manifests, `main.rs`, `lib.rs`, `detect.rs`, `record/git.rs`'s command builder, `tools/budget`.
- Probed, in the session's scratch directory, with nothing written in the repository: the design's P-1 to P-14
  (bubblewrap 0.13.1, `pasta` of 2026-10-02, `keyctl`, Git). No model was called by the integrator; no file of the
  private folder or of a tool's credentials was read (their names and modes only); the network was used for three
  outbound connections and two name lookups from inside the probes. Keys planted in the session's and the user's
  kernel keyrings for P-11, P-13 and P-14 were removed. A probe of P-8 left a confined loop running after its
  `pasta` was killed; it was ended by killing its `bwrap`.
- Decided by the maestro: D-075 (before the review) and D-076 (after it).
- Written: `.brewstillery/bootstrap/confinement-design.md`; `core-specification.md` (the amendments S-1 to S-12, as
  D-076's consequences list them); `tools/budget` (the core's ceiling at 11,000, two tests, the comment); `plan.md`
  ("Phase 9" and its head); `STATUS.md`; D-075, D-076, this entry.
- Codex, four passes, confirmed blocking findings by pass: 6, 1, 1, 0. Each was checked by the integrator before its
  correction.
  - Pass 1 (`a975dbf`): P9O-1, a cell inherits the session's kernel keyring (reproduced, P-11); P9O-2, a tracked hook
    under `core.hooksPath`, changed by a candidate, runs at Fabrewk's own `update-ref` after integration (reproduced,
    P-12); P9O-3, instruction files nested under a wildcard lane escaped the protected paths, which match from the
    root (read in `task.rs`); P9O-4, the extension's program was not in its cell; P9O-5, a writable home and `/tmp`
    contradicted REQ-SEC-1's and REQ-EXT-4's criteria; P9O-6, a duration that must be an integer cannot be recorded
    for an attempt ended after a crash.
  - Pass 2 (`23911ad`): the six closed; P9O-7, with a new session keyring a cell still reads a host key by its
    serial number and links the host's user keyring (reproduced, P-13).
  - Pass 3 (`c73c234`): P9O-7 closed; P9O-8, a key used through a cryptographic socket by its serial number, with no
    keyring call (not reproduced on this machine, whose kernel offers that socket no algorithm; accepted as a gap in
    the claim, P-14). Corrected by widening the filter and by narrowing the claim to the routes it names (T-29).
  - Pass 4 (`d01a75d`): **accept**, no finding; the plan's "Phase 9" agrees with the design's section 6.
- D-071's calibration, for the opening: a design by Fable 5.2 was rejected three times on the same class of edge that
  cost phase 7 its passes (what the operating system really isolates, and what runs after a boundary was checked);
  the last two passes were one topic, the kernel's keys, closed only when the claim was narrowed instead of the
  mechanism widened. The comparable measure is part 9a's, the first code on Fable. Fable's weekly allotment as spent
  is not observable from the session; the maestro reads it in their account.
- A slip of the integrator: it waited for Codex's first pass with a loop that searched the process table for Codex's
  command line, which the loop's own command line contained, so the loop never ended; the pass had finished 15
  minutes earlier. The maestro noticed that no Codex process was running. The later passes were waited for on the
  launching command itself. Noted in `STATUS.md`'s handover; the `codex-review` skill's text is the maestro's to
  change.
- Checks: `cargo test --locked -p budget` green after the ceiling's change; `cargo test --locked -p fabrewk-core
  --lib` and `--test table` green after the specification's amendments (the tests that read it). The other suites,
  the clean clone and the mutation run were not run: no ring's code changed.
- Not done: any code of phase 9; the composition of `bwrap` and `pasta` under which ending Fabrewk's direct child
  ends the cell (the design's 3.7, part 9a's); whether each tool rewrites its credential (3.6, parts 9c and 9d); the
  permission and the budget to run the real tools (asked when part 9c starts). The maestro has not closed phase 8
  by a decision of its own, as D-070 closed phase 7.

## D-077 — Phase 8 closed

- Status: decided
- Decided: 2026-11-11
- Origin: maintainer XEL in session 29's chat, after the integrator reported that no decision of the maestro closed
  phase 8 as D-070 closed phase 7: "De los pendientes, registra el cierre de la fase 7 con estas mismas palabras, fue
  error mío." Phase 7 was already closed (D-070) and the pending item named phase 8, so the integrator reads it as
  phase 8's close; the maestro may correct that reading.
- Decision: phase 8 (core, part 2) is closed. Its parts 8a, 8b and 8c are done (R-035 to R-037), each meeting the
  plan's acceptance for row 8: tests green in a clean clone, the past-defect cases its part covers held, every mutant
  caught or removed, within the size budget (D-069, D-074), and Codex's review accepted. Phase 9 was opened in the same
  session (D-075).
- Consequences: `plan.md` and `STATUS.md`. What phase 8 left to phase 9 stays where the record put it: the usage
  record's shape (now S-5, D-076) and P8C-8 (declared, D-075 point 3).
- Sources: chat of session 29; `plan.md` row 8 and "Phase 8"; D-070, D-072 to D-074; R-035 to R-037.
- Supersedes: none.

## D-078 — Part 9a on Opus 5.5; D-071's calibration ends with the opening

- Status: decided
- Decided: 2026-11-11
- Origin: maintainer XEL in session 29's chat, reporting what the opening spent and asking the integrator to check
  whether part 9a should still run on Fable: "Con esta sola etapa consumió el 23% de su propia cuota semanal y nos dejó
  con el 94% consumido de la cuota de sesión de 5h. Dicho esto, abrir la fase 9a en fable-5.2 esfuerzo alto no sería
  recomendable hasta tener fresca la ventana de 5h, o si consideras justificado recalibrar su uso para continar con
  9a, verificalo." Then, to the integrator's question: "Opus 5.5 alto (Recomendado)".
- Decision:
  1. Part 9a runs on Opus 5.5, effort high, like parts 9b to 9i; Codex (GPT-6.1 Sol), effort high, reviews.
  2. D-071's calibration ends with the opening. Its measure: the opening (design, probes, four Codex passes, the
     amendments) spent 23 % of Fable's weekly allotment and left 94 % of the five-hour window spent. Codex rejected
     the design three times (confirmed blocking 6, 1, 1, 0), on the class of edge that cost phase 7's hard parts
     their passes, so no clear gain over Opus shows; what Fable added of its own was probing the machine before
     designing (the desktop's socket, the kernel's keys). Fable keeps no standing role; using it again is the
     maestro's decision.
- Reason: a part of code costs more than an opening (code, tests, mutation, several reviews); the integrator
  estimated 40 % to 70 % of the weekly allotment for part 9a, with the risk of running out mid-part, while the design
  already writes down everything 9a builds. The alternatives offered: Fable once the five-hour window renews (D-075
  as decided), or Opus with a short Fable session for the design's open point 3.7.
- Consequences: `plan.md` ("Phase 9"); `confinement-design.md` (section 6's row 9a); `STATUS.md` (the handover).
- Sources: chat of session 29; D-071, D-075; R-038.
- Supersedes: D-075 point 7 for part 9a's model; D-071 point 2 for phase 9's closing report on Fable, now given here.

## R-039 — Session 30, part 9a: the cell's contract, the runner and the bubblewrap backend

- Status: recorded
- Date: 2026-11-11
- Who: integrator Claude Code, `claude-opus-5-5`, effort high (D-078). Reviewer: Codex `codex-cli 0.166.0`,
  `gpt-6.1-sol`, effort high, launched as the `codex-review` skill says (a disposable clone under `target/review-9a/`
  with a copy of the Cargo registry, `workspace-write`, no network, a user scope capped at 3 GB). No subagent.
- Start: HEAD `85b268e`, tree clean but an untracked `.vscode/`.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, `confinement-design.md` (3.1 to 3.10, 4, 6), `core-specification.md`
  (SEC, REG-13, RUN, the amendments of phase 9's opening, 11.3, 12), this file (D-075 to D-078, R-038), the skills
  `verify-part`, `codex-review` and `sharp-edges`, and the code of `main.rs`, `lib.rs`, `detect.rs`, `cli/mod.rs`,
  `cli/config.rs`, `cli/tasks.rs`, `record/` and `tools/budget`. No file of the private folder or of a tool's
  credentials was read; no model was called by the integrator.
- Decided by the maestro in this session: nothing. The design left the composition of `bwrap` and `pasta` to this
  part, by test (3.7); its consequences are below, for the maestro to read.
- Probed on this machine, in the session's scratch directory, before the code (bubblewrap 0.13.1, `pasta`
  2026-10-02, SELinux enforcing):
  1. `pasta` cannot join the namespaces of a `bwrap` started with `--dev`: `--dev` makes `bwrap` map the person to
     root first and nest a second user namespace that maps them back, and `pasta` joins the inner one, which does not
     own the network namespace ("Couldn't switch to pasta namespaces: Operation not permitted"). Without `--dev`
     (devices bound one by one) it joins, and the user inside stays the person's. `--disable-userns` nests one too.
  2. `pasta` cannot be joined to `bwrap`'s own processes, only to the command once it has replaced them.
  3. `pasta`'s watch of the target namespace is refused here ("netns dir open: Permission denied"), so it runs with
     `--no-netns-quit`, in the foreground, and Fabrewk ends it. `setpriv --pdeathsig` does not survive `pasta`'s
     change of SELinux domain: a `pasta` whose parent was killed stays. Two such leftovers of the probes (17368,
     19784) were ended by their identifiers.
  4. The first process of `bwrap`'s namespace closes its descriptors before the kernel ends the namespace's other
     processes, so the end of the output channels does not show the cell dead; its reaping does.
  5. A new network namespace holds IPv6's unreachable default route on `lo`; a `bwrap` nested with `--dev` needs a
     `/dev/tty` to bind.
  6. The bash that runs `sh` sets `PWD`, `SHLVL` and `_` for what it runs.
  A host-side control run of `tests/probe.py` added a key named `made` to the session keyring and linked the user
  keyring there; the key was unlinked. The user keyring's link in the session keyring was left: the session's
  default links it (`pam_keyinit`), but whether it was there before was not checked. The tests' own keys are removed by the
  tests; the keyrings were checked empty of them after each run.
- Written: `crates/core/src/confine.rs` (the cell, the backend's contract, `Programs` made only by `available`, the
  runner, `Unfinished`, the run's directories, the evidence store, the probe, the core's residual risks);
  `crates/confine-bwrap/` (the backend, its filter for aarch64, its tests `tests/cell.rs` and `tests/probe.py`);
  `crates/core/tests/confine.rs`; `cli::main` takes the backend, whose programs are detected with the modules';
  `config` lists the residual risks (`fabrewk.config/2`: `backend`, `residual`); the catalogues (`residual`,
  `config.residual`, `config.no-backend`); `run`'s refusal words; `main.rs` wires the backend on Linux; the ring
  `confine-bwrap` in `tools/budget` at its kind's ceiling (D-075 point 2) and the test of the local packages; the
  generated reference; the paragraph "The cell and its backend, as part 9a fixed them" in `core-specification.md`
  and `fabrewk.config/2` in its section 5; `confinement-design.md` (3.7 settled); `plan.md`; `STATUS.md`; this entry.
- Interpretations the maestro may want to read (none contradicts a requirement):
  1. The composition, candidate (a) of 3.7: `bwrap` is Fabrewk's direct child with `--die-with-parent`; `pasta`, its
     second child, joins the cell's command once it is the wrapper; the command is released once a default route
     exists. Its three consequences: no `/dev/pts` or `/dev/shm` in a cell; a cell with a network that may not nest
     is refused (`CONFINEMENT_UNAVAILABLE`), which leaves a check's network (REQ-GATE-2, part 9f) and an extension's
     (part 9h) to choose then between nesting allowed, a filter refusing new user namespaces, or no network; and,
     after Fabrewk is killed during a run, the cell ends with it but its `pasta` stays, idle, holding a namespace no
     process uses: a residual risk (`network-helper`) 3.10 did not list. Part 9e's recovery (REQ-RUN-5) may end it.
  2. The filter reaches `bwrap` on descriptor 3, opened by `sh`, not on standard input (3.3), which carries the line
     that releases the command. The environment reaches the cell through a file it reads inside, not by inheritance
     (3.3), since inheritance gave it to processes of the host first (P9A-4).
  3. The residual risks `config` lists: the core's three (T-17, T-18, T-25 with P8C-8) and the backend's six (T-04,
     T-08, T-10, T-24, T-29, the idle `pasta`); the adapters' come with them.
  4. The time and the bytes kept of each channel are the caller's parameters; their values come with parts 9e and
     9f. A probe's pass is marked on the detection; a write command keeps it from part 9e on.
- Codex, three passes (high), confirmed blocking findings by pass: 3, 1, 0. Pass 1 (`a9ad52d`): P9A-1 (a run's
  directories removed when the backend's end failed with an error other than `TOOL_FAILED`, or when `start` failed
  after its process existed), P9A-2 (`pasta`'s evidence created without `0600`), P9A-3 (what `bwrap` wrote before
  `pasta` failed was lost). Pass 2 (`413216b`): the three closed; P9A-4 (the cell's environment, `LD_PRELOAD` among
  it, given to `sh` and `bwrap` on the host before any confinement; reproduced by Codex with a library's
  constructor). Pass 3 (`d60e3a0`): **accept**. Each new test failed without its correction (checked by hand for
  every one). Two defects the integrator found before the first pass's corrections were corrected with them: a
  runner that waited forever on the readers of a cell its backend could not show ended, and a cell with no command
  that ran `env`. Codex's sandbox runs no namespace and no network, so it ran the core's suites and read the
  backend's; the backend's thirteen tests ran on this machine.
- Mutation, with `--features fabrewk-core/fault-injection`, `--test-workspace true`, `--in-diff`, `ulimit -v
  1600000`, `--timeout 180` (the backend's real cells make the suite take 50 s), each run in its own named user scope,
  stopped after it so that what a mutant left running ended with it: the first run, 133 mutants at `d60e3a0`, in 61
  minutes: 86 caught, 29 unviable, 2 timeouts (`stop` made to do nothing, the time limit never reached: loops caught by
  the timeout) and 16 missed: ten in the routes' test, two in the bind of a system directory that is not a link,
  three in a reader's retry after an interrupted read, one in the cap's prompt end. Each killed by a test (the routes'
  test made a pure function with its own cases; the cap's test now times the end) or removed by simplifying the code
  (`--ro-bind-try` for every directory that is not a link; no retry); none declared equivalent. The run over those
  corrections (`8faed54`), 28 mutants: all caught. Codex did not review `8faed54`: it removes a
  guard and a retry and adds tests, and changes no behaviour its third pass accepted but the read error's. The counter's diff holds no mutant. The mutant that makes `stop` do
  nothing left two processes of its cells, ended with the run's scope.
- Clean clone of `8faed54`: 431 tests, `crash` 3 and `faults` 19 green, the musl build, `budget check` (the core
  7,965 of 11,000, the backend 276 of 400, 4 direct runtime crates, 31 in the release build, the binary 2,298,216
  bytes), clippy with and without `fault-injection`, fmt and `cargo deny` clean; `--version --json` lists no feature.
- Measured: the core 7,965 of 11,000 counted lines (+335, projected 250); the backend 276 of 400 (projected 300).
- Not done: `run` past confinement at the command line, and keeping a probe's pass from a command (part 9e); the
  roles' cells (parts 9b to 9h build them on this contract).

## R-040 — Session 31, part 9b: the attempt's copy, the change set and the candidate

- Status: recorded
- Date: 2026-11-12
- Who: integrator Claude Code, `claude-opus-5-5`, effort high (D-078). Reviewer: Codex `codex-cli 0.166.0`,
  `gpt-6.1-sol`, effort high, launched as the `codex-review` skill says (a disposable clone under `target/review-9b/`
  with a copy of the Cargo registry, `workspace-write`, no network, a user scope capped at 3 GB). No subagent.
- Start: HEAD `9780ad8`, tree clean.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, `confinement-design.md` (3.8, section 4: S-2, S-3, S-4, S-11),
  `core-specification.md` (REQ-RUN-3, REQ-RUN-4, REQ-RUN-10, REQ-SEC-3, REQ-GATE-9, REQ-REG-10, REQ-GATE-8, the
  amendments of phase 9's opening, sections 8, 11.3, 11.4 and 12), this file (D-075 to D-078, R-039), the skills
  `verify-part`, `codex-review` and `differential-review`, and the code of `record/git.rs`, `record/store.rs`,
  `task.rs`, `scan.rs`, `detect.rs`, `settings/keys.rs`, `cli/propose.rs`, `cli/tasks.rs`, `cli/status.rs` and the
  binary's tests that used a Git hook. No file of the private folder or of a tool's credentials was read; no model was
  called by the integrator.
- Decided by the maestro in this session: after Codex's provider cut a fourth pass short (below), that pass is a
  narrow re-check, the corrections' scope only, as the `codex-review` skill describes a re-check; the fifth pass was
  the same.
- Written: `crates/core/src/candidate.rs` (the export, the walk, the change set and its guards, the candidate commit,
  `free_space`, the protected paths and names); `crates/core/tests/candidate.rs` (20 tests); in `record/git.rs`, S-11
  (`core.hooksPath=/dev/null` and `core.fsmonitor=false` on every Git command Fabrewk runs) and `hooks_dir`; the codes
  `DISK_LOW` and `LANE_VIOLATION`; the keys `limits.freeDiskMegabytes` and `gate.protectedNames`; `df` among the core's
  programs; `propose` and `authorize` reading the hooks directory; `status` showing what `local/evidence/` and
  `local/tmp/` hold (`fabrewk.status/3`); the catalogues; the generated reference; three of the binary's tests that
  made Git fail through a hook (P7D3A-18 by a ref directory made read-only, P7D3A-21 by a test that no write command
  runs a hook, P8B-4 and its own-repository variant by a pause at the new fault point `record-moved`); the paragraph
  "The copy and the candidate, as part 9b fixed them" in `core-specification.md` and `fabrewk.status/3` in its
  section 8c paragraph; `plan.md`; `STATUS.md`; this entry.
- Interpretations the maestro may want to read (none contradicts a requirement; each is in the new paragraph):
  1. A path Git refuses to hold in a tree (a `.gitmodules` that is a link, or a name Windows or macOS reads as one
     under `core.protectNTFS` or `core.protectHFS`) is left out by `update-index` with exit status 0 (Git 2.56), so the
     change set is written over the base into an index apart and read back, and such a path is `PROTECTED_PATH`
     (P9B-1). `commit` checks its own index the same way before it writes any object (`TOOL_FAILED`).
  2. The hooks directory is read from the repository's configuration at each guard and resolved from the working
     tree's root as the kernel resolves it when Git runs a hook: a link before a `..` that follows it, a part not made
     yet kept as written, a relative root made absolute, the root's own links free of the limit of 40. Protected are
     the directory as written and as resolved, and every path the reading passes through that is not above it (a link,
     or a directory a `..` leaves, since either changed to a link moves the directory); past the limit, those met up
     to the 41st link; a component that is not text protects the directory above it (P9B-2, P9B-4, P9B-7 to P9B-9,
     P9B-11, P9B-12). Where the hooks directory leads outside the working tree, nothing inside is protected for it.
  3. `DISK_LOW` reads `df -Pk`, since the core has no `statvfs` without a new runtime crate (AD-4). A count is anything
     `df` may print in its columns and a capacity digits with `%` or `-`, so the real columns are always a reading; a
     line read in more than one place, or whose space available is not a count, refuses (P9B-3, P9B-5, P9B-10). A
     file system whose name or mount point looks like `df`'s columns is therefore refused as unreadable: the safe side.
- Codex, five complete passes and three launches cut short, confirmed blocking findings by pass: 3, 5, 3, 1, 0.
  1. Pass 1 (`05be515`): P9B-1 (above), P9B-2 (the hooks directory through a link not protected when absent, or read
     lexically across `..`), P9B-3 (a `df` name holding counts and a capacity read as the free space).
  2. Pass 2 (`f60190b`): cut twice by Codex's provider ("flagged for possible cybersecurity risk"), with no report;
     the second launch only added a line of context to the prompt. Its stream named five defects, which the integrator
     confirmed and corrected (`b6c4a47`): P9B-4 (a root reached through a link spending the limit), P9B-5 (a negative
     space available), P9B-6 (`commit` writing objects before its index check), P9B-7 (a hooks component that is not
     text), P9B-8 (a link before `..` unprotected). Both cut launches had read the `differential-review` skill's
     adversarial guide; from pass 3 on the prompt asks Codex not to read `.brewstillery/skills/` and to report each
     finding as an input, an expected result and an actual one.
  3. Pass 3 (`a270cab`): P9B-1 to P9B-8 confirmed closed; P9B-9 (the links met before the limit dropped), P9B-10 (a
     capacity of `-` or a count past `i64`), P9B-11 (a relative root).
  4. Pass 4 (`1be3e9e`): its first launch cut by the provider, with no report; the maestro chose a narrow re-check.
     P9B-9 to P9B-11 confirmed closed; P9B-12 (a directory a `..` leaves, made a link).
  5. Pass 5 (`ca8bc8a`), narrow: P9B-12 confirmed closed in its three forms; **accept**.
  Each new test failed without its correction, checked by hand for every one (for a case behind an earlier assertion
  of the same test, with that assertion removed). Codex's sandbox cannot bind sockets, so it ran the suites without the
  backend and the two tests that need one; the backend's tests ran on this machine.
- Mutation, with `--features fabrewk-core/fault-injection`, `--test-workspace true`, `ulimit -v 1600000`, `--timeout
  180`, each run in its own named user scope:
  1. The part (`9635088`, `--in-diff`): 221 mutants in 2 hours: 163 caught, 40 unviable, 18 missed. Killed by tests:
     the `df` parser, malformed commit identifiers, a crafted tree holding `a` as a link and as a directory (the export
     refuses to write through the link), a submodule the cell filled, a `.gitignore` that is a link, a new path no rule
     matches, a protected directory replaced by a file, the trailer's 100 bytes, a copy changed after its change set.
     Removed by simplifying: `quoted` escapes every byte; `hooks_dir`'s exit status 1, which Fabrewk's own value on
     the command line makes impossible; the readable bit compared before it is set. **Declared equivalent: one**, the
     check that `cat-file --batch` answers with the identifier it was asked (`contents`): Git echoes each full
     identifier it is given, so only a broken Git could tell the mutant from the original. The focused run over those
     corrections, 29 mutants: 27 caught, the equivalent, and `path == q` in `guarded`, which the first run had counted
     caught only because an unrelated test failed by chance (below); a case was added and `guarded`'s five mutants were
     caught (`05be515`).
  2. Over P9B-1 to P9B-3 (`1c7da86`): 33 mutants, 26 caught, 3 unviable, 4 missed in the limit of 40 links; a chain of
     exactly 40 and a loop were tested (`f60190b`); the run over `physical`, 9: 7 caught, 2 timeouts (the count made to
     shrink or stay: loops caught by the timeout).
  3. Over P9B-4 to P9B-8 (`b6c4a47`): 27, 24 caught, 2 unviable, 1 missed: an arm no path reached, removed (`a270cab`);
     the run over `hooks_dir`, 11: 10 caught, 1 unviable. P9B-12's correction made that arm reachable again, with a case.
  4. Over P9B-9 to P9B-11 (`1be3e9e`): 22, all caught. Over P9B-12 (`ca8bc8a`): 20, all caught.
- A test that fails by chance, older than this part: the test of REQ-REG-6 on a payload of the wrong shape in
  `crates/core/tests/fermento.rs` (`req_reg_6_a_payload_of_the_wrong_shape_...`) can get `LOCK_HELD`: the record's
  lock is an `flock` on a descriptor opened with `O_CLOEXEC`, and a process another test thread starts holds it from
  its fork until its `exec`. A command of Fabrewk runs in one thread and does not suffer it; the tests and the mutation
  runs do: it made one mutant look caught, and Codex met it in passes 1 and 5. Not corrected here; its destination is
  the maestro's (a retry in the tests, or a lock the record takes and releases in one process); it is in `STATUS.md`.
- Clean clone of `ca8bc8a`: 452 tests, `crash` 3 and `faults` 20 green, the musl build, `budget check` (the core
  8,601 of 11,000, the backend 276 of 400, 4 direct runtime crates, 31 in the release build, the binary 2,429,288
  bytes), clippy with and without `fault-injection`, fmt and `cargo deny` clean; `--version --json` lists no feature.
  An earlier clean clone of `05be515` was green too (451 tests).
- Measured: the core 8,601 of 11,000 counted lines (+636, projected 400); the backend 276 of 400, unchanged.
- Not done: `run` past confinement at the command line; recording a failed export as `attempt-ended` with
  `TOOL_FAILED` (PD-21's record half); `candidate-recorded`'s payload; the checks' and the reviewer's copies deleted
  unread (parts 9e and 9f); the integration's recomputation of the change set and writing the working tree from the
  objects (part 9g); the adapters' own declaration of their instruction names (parts 9c, 9d).

## D-079 — Part 9c: the real Claude Code's permission and budget, the credential's renewal, the audit's structural rule

- Status: decided
- Decided: 2026-11-12
- Origin: maintainer XEL in session 32's chat, answering the integrator's questions. At the part's start: "Sí, ambos
  (Recomendado)" (permission to run the real Claude Code without a model and with one), "4, Sonnet 5.5 bajo
  (Recomendado)", "Tu archivo, solo lectura (Recomendado)". After Codex's fifth pass: "Regla estructural
  (Recomendado)" and, for the credential's renewal, "Rehusar si vence pronto (Recomendado)".
- Decision:
  1. The integrator may run the real Claude Code of this machine (2.1.316) without a model (its version, its help,
     the preflight) and in up to four dispatches that call one, `claude-sonnet-5-5` at effort `low`, each on a fixture
     copy of a few files with a limit of ten minutes. All four were spent (R-041).
  2. Those dispatches use the person's own credential, `~/.claude/.credentials.json`, bound read-only; the integrator
     reads of it only its date, size, digest and the field `expiresAt`, never a token.
  3. After five passes of Codex with confirmed blocking findings 3, 2, 1, 1, 1 (two retries without progress), the
     class P9C-7 to P9C-9 is closed by a structural rule rather than case by case: a `Bash` use examines only when its
     answer and the run's own record beside it (`tool_use_result`) show it completed and printed, and a user turn
     holds one answer. The sixth pass is a narrow re-check of that correction.
  4. A renewal of the credential inside a cell, which cannot write the file, is closed by refusing: before a
     dispatch Fabrewk reads only `expiresAt` from the tool's credential and refuses (`TOOL_FAILED`, saying to renew
     the login) when it falls within the attempt's minutes. Built with the dispatch in part 9e; until then declared.
- Reason: 3.6 left open what a read-only bind does when the tool renews its credential; the test showed the bind
  works while the token is valid and that the tool cannot rewrite the file inside a cell, but provoking a renewal
  could have ended the person's login. The alternatives offered for the renewal: declare it as a residual risk; a
  long-lived token from `claude setup-token` kept by the person; leave it to the part's close. For the review: the
  same rule with a whole sixth pass; or correcting P9C-9 as reported and declaring the textual audit as a limit.
- Consequences: `core-specification.md` ("as part 9c fixed them"); `plan.md` (row 9e gains the refusal); R-041.
- Sources: chat of session 32; `confinement-design.md` 3.6; R-041.

## R-041 — Session 32, part 9c: the adapter's contract and the Claude Code module

- Status: recorded
- Date: 2026-11-12
- Who: integrator Claude Code, `claude-opus-5-5`, effort high (D-078). Reviewer: Codex `codex-cli 0.166.0`,
  `gpt-6.1-sol`, effort high, launched as the `codex-review` skill says. One subagent of the `web-fetch` kind read the
  tool's documentation (code.claude.com: sandboxing, settings, headless, CLI reference, permissions, authentication,
  environment variables).
- Start: HEAD `5c3a6e7`, tree clean.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, `confinement-design.md` (whole), `core-specification.md` (RUN, GATE-4, SEC,
  the amendments and the 9a and 9b paragraphs, 11.2, 11.3, 11.4, section 8's keys), `past-defect-cases.md` (PD-25 to
  PD-28), `architecture-decisions.md` AD-5, `harness-verdicts.md` (its notes on the adapters) and, as that file's
  "reuse as design", `tickbrew-harness/tickbrew_harness/adapters/audit.py`; this file (D-075, D-078, R-040); the skills
  `verify-part`, `codex-review` and `differential-review`; the code of `confine.rs`, `detect.rs`, `task.rs`,
  `alambique.rs`, `view.rs`, `code.rs`, `settings/keys.rs`, `cli/mod.rs`, `cli/config.rs`, `crates/confine-bwrap` and
  `main.rs`. Of the person's Claude Code credential, only its date, size, digest and `expiresAt` (D-079); no file of
  the private folder.
- Decided by the maestro in this session: D-079.
- The real tool (D-079 point 1), Claude Code 2.1.316, `claude-sonnet-5-5` at effort `low`, four dispatches costing
  about 0.13 USD at list price: a review (`review.jsonl`), a build (`build.jsonl`), a probe of the tool's sandbox, and
  a review running a silent command (`silent.jsonl`). Without a model, in cells with no credential: its version, its
  debug log at start, and its stop at "Not logged in". What it showed:
  1. `--safe-mode` keeps OAuth and turns discovery off: the built-in plugin that reads `AGENTS.md` reported "project
     memory is off"; `--restricted` ignored a project's settings planted in the copy. `--bare` reads no OAuth.
  2. The tool's sandbox starts only at the model's first command, so no command of the tool's own starts it without a
     model; at a dispatch the model's commands had a seccomp filter, only a loopback interface, the direct network
     unreachable and the proxy refusing every host, the credential unreadable, the home and `.gitconfig` not writable
     and `/work` writable; the stream recorded `sandbox_violations` for the refused host.
  3. The credential: with the read-only bind all four dispatches worked and the file kept its date, size and digest;
     its date was exactly eight hours before its `expiresAt`, so the tool rewrites it when it renews it; a renewal
     inside a cell was not provoked (D-079 point 4).
  4. The stream: `system`/`init`, `assistant`, `user` with `tool_use_result`, `rate_limit_event`, `system`/
     `thinking_tokens` when the model thinks, one `result` with `structured_output`; a silent command is answered
     `(Bash completed with no output)`. The streams hold the account's weekly and five-hour use; it was removed from
     the kept fixtures, and the probe's stream, which holds the sandbox's proxy credential for its session, was not
     kept.
- Written: `crates/core/src/adapter.rs` (the contract); `crates/adapter-claude` (ring `ToolAdapter`: `lib.rs`,
  `stream.rs`, tests `launch.rs`, `stream.rs` with three recorded streams, and `real.rs`, ignored, for the real tool);
  `crates/core/tests/adapter.rs`; `REVIEW_INVALID` in `code.rs` and the catalogues; the catalogues' `packet` and
  three residual risks; the binary's wiring (`ADAPTERS`; Codex's markers stay until part 9d); `config` listing the
  adapters' residual risks; two of the binary's tests (the sweep now lists Claude Code's skills folder; S-7 over the
  adapters); `tools/budget` (the ring, at its kind's ceiling, D-075 point 2); the paragraph "as part 9c fixed them"
  in `core-specification.md`; `plan.md`; `STATUS.md`; D-079 and this entry.
- Interpretations the maestro may want to read (each in the new paragraph):
  1. The packet's rules are the role's instruction, the base rules 8, 11 and 12 (the others govern the domain, which no
     copy holds) and the project's rules.
  2. An accept holding a confirmed blocking finding is `REVIEW_INVALID`, as the verdict and REQ-GATE-6's progress
     measure would disagree; a build's result that does not fit is `TOOL_FAILED`.
  3. Claude Code's `input` tokens are `input_tokens` plus `cache_creation_input_tokens`; `cached` is
     `cache_read_input_tokens`.
  4. The release is the tool's `versions` directory and may not hold the home or the repository (`TOOL_MISSING`),
     found by the integrator's own review before Codex's first pass: an executable installed straight into a home
     would have shown the home whole.
  5. `gate.protectedNames`' default stays one literal in the core; a test holds each adapter's names within it.
  6. The preflight cannot make the tool start its own sandbox without a model; it runs the tool, `socat`, and a
     nested sandbox of the tool's kind, and `failIfUnavailable` makes a dispatch refuse when the tool's cannot start.
  7. PD-26.2 counts (the split) and voids nothing; PD-26.4 is every stream cut before its end; a heredoc's lines count
     as commands, since only whether the count is zero decides.
- Codex, six passes, confirmed blocking findings by pass: 3, 2, 1, 1, 1, 0.
  1. Pass 1 (`401309f`): P9C-1 (a use answered with an error counted), P9C-2 (a `message` on a rate-limit notice, an
     answer's content or a block's text of a wrong type, accepted), P9C-3 (a comment alone counted as a command).
  2. Pass 2 (`a95adf2`): P9C-1 to P9C-3 closed; P9C-4 (a line continuation counted), P9C-5 (a link in a folder on the
     way to the credential followed); P9C-6, not blocking (a final flag of a wrong type kept model and tokens).
  3. Pass 3 (`15817ef`): P9C-4 to P9C-6 closed; P9C-7 (an uncalled function, an empty loop or an unmatched branch
     counted). Corrected by output: the fourth dispatch recorded how the tool answers a silent command.
  4. Pass 4 (`41b68ae`): P9C-7 closed; P9C-8 (a command run in the background counted by its start).
  5. Pass 5 (`fe31883`): P9C-8 closed; P9C-9 (a pending answer counted with no background flag). Two retries
     without progress: the maestro chose the structural rule and a narrow sixth pass (D-079 point 3).
  6. Pass 6 (`e179f07`), narrow: P9C-7 to P9C-9 closed in every form, the three recorded streams read whole; **accept**.
  Each new test failed without its correction, checked by hand. While correcting P9C-2, the probe's real stream read
  void with the stricter grammar: a `thinking_tokens` estimate, now admitted after the init. Codex's sandbox cannot
  run `bwrap`, `pasta` or bind sockets; those tests ran on this machine.
- Mutation, with `--features fabrewk-core/fault-injection`, `--test-workspace true`, `ulimit -v 1600000`, `--timeout
  180`, in its own user scope, over the part's diff (`e179f07`): 155 mutants in 88 minutes, 149 caught, 3 unviable,
  3 missed, all in the preflight's guard between a run that could not start and one the backend cannot show ended;
  a test now tells them apart (`c64b9c1`), and the run over the preflight, 5 mutants, caught all. None declared
  equivalent.
- Clean clone of `c64b9c1`: 498 tests (6 ignored: the real tool's and a kept stream's), `crash` 3 and `faults` 20
  green, the musl build, `budget check` (the core 8,876 of 11,000, the backend 276 of 400, the adapter 295 of 600, 4
  direct runtime crates, 31 in the release build, the binary 2,494,824 bytes), clippy with and without
  `fault-injection`, fmt and `cargo deny` clean; `--version --json` lists no feature.
- Measured: the core +275 (projected 150), the adapter 295 (projected about 500).
- Not done: `run` dispatching at the command line, the preflight's pass kept as verified by use, the credential's
  expiry refused, the packet's size bounded (part 9e); the reviewer's dispatch, `review-recorded`, `REVIEW_UNEXAMINED`
  and PD-25 and PD-26 through the whole flow (part 9f); Codex's module (part 9d). The chance failure of R-040
  (`LOCK_HELD` in a test) did not occur in this part's runs; its destination is still the maestro's.

## D-080 — R-040's chance `LOCK_HELD`: the session releases its lock on the shared description

- Status: decided
- Decided: 2026-11-12
- Origin: maintainer XEL in session 32's chat, before closing it: "¿Es conveniente resolver el pendiente del fallo
  ocasional con LOCK_HELD de R-040 ..? Si es así, hagamoslo, si no es así proyectalo al corte o parte que resulte
  conveniente". The integrator judged it worth resolving now: part 9e multiplies the tests that start processes in
  parallel, and the failure had once made a missed mutant look caught (R-040).
- Decision: ending a record session (`record/store.rs`, `Session`'s `Drop`) releases the lock explicitly
  (`File::unlock`), which frees the open file description, not only its descriptor. A process another thread forks
  while a session holds the lock shares that description until it executes its program; closing the descriptor
  alone left the lock held by it, and the next writer of the same test process got `LOCK_HELD`. Neither a retry in
  the tests nor a lock taken in another process was needed. A command of Fabrewk runs in one thread and never
  suffered it.
- Shown by: `record::store::tests::ending_a_session_releases_the_lock_a_forked_copy_of_its_descriptor_still_shares`,
  which holds a copy of the descriptor as a forked child would; it failed with `LOCK_HELD` before the correction.
  The core's `fermento` tests ran fifteen times in a row without the failure; the whole suite (499 tests), `crash` and
  `faults` green; mutation over the change, one mutant, caught.
- Consequences: R-040's and R-041's open destination is closed; `STATUS.md`.
- Sources: chat of session 32; R-040.

## D-081 — Part 9d: the real Codex's permission and budget, its credential, its renewal

- Status: decided
- Decided: 2026-11-12
- Origin: maintainer XEL in session 33's chat, answering the integrator's questions. At the part's start: "Sí, 4, Luna
  bajo (Recomendado)" (permission to run the real Codex without a model and with one) and "Tu archivo, solo lectura
  (Recomendado)". After the four dispatches: "Rehusar igual (Recomendado)".
- Decision:
  1. The integrator may run the real Codex of this machine (`codex-cli` 0.166.0) without a model (its version, its
     help, `codex sandbox`, `codex debug prompt-input`, the preflight) and in up to four dispatches that call one,
     `gpt-6-luna` at effort `low`, each on a fixture copy of a few files with a limit of ten minutes, charged to the
     person's ChatGPT plan. All four were spent (R-042); the first was refused by the provider for its schema.
  2. Those dispatches use the person's own credential, `~/.codex/auth.json`, bound read-only; the integrator reads of
     it only its date, size, digest and the fields `auth_mode` and `last_refresh`, never a token.
  3. A renewal of Codex's login inside a cell, which cannot write the file, is closed as D-079 point 4 closed Claude
     Code's: before a dispatch Fabrewk reads only `last_refresh` and refuses (`TOOL_FAILED`, saying to renew the login
     with the tool) when the tool's renewal interval, about eight days by its documentation, ends within the attempt's
     minutes. Built with the dispatch in part 9e; until then declared.
- Reason: 3.6 asks the credential question of each tool before its first real dispatch; the alternatives offered were
  gpt-6.1-sol at effort low, or no model at all with the stream written from the source and declared unrecorded; for
  the credential, a writable bind (an agent could then alter the person's login) or a separate API key; for the
  renewal, declaring it, or leaving it to part 9e.
- Consequences: `core-specification.md` ("as part 9d fixed them"); `plan.md` (row 9e gains Codex's refusal); R-042.
- Sources: chat of session 33; `confinement-design.md` 3.6; D-079.

## R-042 — Session 33, part 9d: Codex's module on the adapter's contract

- Status: recorded
- Date: 2026-11-12
- Who: integrator Claude Code, `claude-opus-5-5`, effort high (D-078). Reviewer: Codex `codex-cli 0.166.0`,
  `gpt-6.1-sol`, effort high, launched as the `codex-review` skill says. One subagent of the `web-fetch` kind read
  Codex's documentation (its non-interactive mode, configuration reference, sandboxing and approvals, environment
  variables, `AGENTS.md`, authentication, and `exec_events.rs` in its source for the stream's events).
- Start: HEAD `ad1d8b7`, tree clean.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md` (phase 9), `confinement-design.md` 3.6 to 3.10 and sections 6 and 7,
  `core-specification.md` (the 9c paragraph), `past-defect-cases.md` (PD-25 to PD-28), the alambique's 6.38 and 6.51,
  this file (D-079, R-041, D-080, D-075's point 2); the skills `differential-review`, `codex-review` and `verify-part`;
  the code of `adapter.rs`, `cli/mod.rs`, `crates/adapter-claude` (source and tests), `crates/fabrewk`, `tools/budget`.
  Of the person's Codex credential, only its date, size, digest and the fields `auth_mode` and `last_refresh` (D-081);
  no file of the private folder.
- Decided by the maestro in this session: D-081.
- The real tool without a model, in rough cells and in Fabrewk's: `--strict-config` refuses an unknown setting, so
  each setting was checked; `codex features list`; `codex debug prompt-input` with an `AGENTS.md`, an
  `AGENTS.override.md`, skills in `.agents/skills` and `.codex/skills` and a project's `.codex/config.toml` planted in the
  copy: by default the model sees the override and both skills; with the adapter's settings none of them; `codex sandbox`
  nested in a cell: under `:workspace` a seccomp filter, no new privileges, only `lo`, `/work` and `/tmp` writable,
  the home not; under `:read-only` nothing writable; with user namespaces disabled it ends with status 1 and runs
  nothing; no `.git`, `.codex`, `.agents` or `.aws` created in the copy; a permission profile with
  `filesystem={"/home/agent/.codex"="none"}` makes the credential's folder unreadable and unlistable (an unknown access
  value refuses). Without network a run writes `error` notices ("Reconnecting... n/5") and an `error` item, then
  stops at its limit, hence `unbounded_connection_retries` off.
- The real tool with a model (D-081 point 1), `gpt-6-luna` at effort `low`, four dispatches: the first, a review, was
  refused by the provider in two seconds because the review's schema had an optional `path` (OpenAI's strict output
  requires every property; kept as `failed.jsonl`); then a review (`review.jsonl`), a review that ran a silent command
  and the probe of the sandbox (`probe.jsonl`, the two joined to keep within four), and a build (`build.jsonl`). What
  they showed: the stream's events; commands shown as `/bin/sh -lc '<script>'`; a silent command's empty
  `aggregated_output`; no model named; inside the sandbox a direct socket refused, no name resolution, the credential's
  folder unreadable, the home and a reviewer's `/work` not writable, and the variables `CODEX_SANDBOX_NETWORK_DISABLED`
  and `CODEX_THREAD_ID`; the credential's date, size and digest unchanged after all four; the tool's own folder in the
  cell's home filled with its databases, logs and model catalogue, and the bind's mount point left empty. The
  streams hold no figure of the account and were kept whole.
- Written: `crates/adapter-codex` (ring `ToolAdapter`: `lib.rs`, `stream.rs`; tests `launch.rs`, `stream.rs` with four
  recorded streams, `real.rs`, ignored); `crates/core/src/adapter.rs` (the machine rule and `holds`, `SCHEMA` written
  by `dispatch`, every property of a role's schema required, `json`, `segments` moved from Claude Code's module);
  `cli/mod.rs` and `main.rs` (no markers on an adapter's behalf); Claude Code's module (`machine`, `json`, the moved
  `segments`); `serde` a direct dependency of the core (its traits, approved in D-036 and already in the release
  build); the tests of the core, of Claude Code's module and of the binary that follow; `tools/budget` (the ring, at its
  kind's ceiling, D-075 point 2); the paragraph "as part 9d fixed them" in `core-specification.md` and two amendments
  to the 9c one; `plan.md`; `STATUS.md`; D-081 and this entry.
- Interpretations the maestro may want to read (each in the new paragraph):
  1. 6.38's rule ("Codex is launched inside a repository, never in `~/Dev`") becomes, as 3.9 restated it, a machine
     rule each adapter declares: the launch directory, the attempt's copy, may hold no `.git` at any depth; both
     adapters declare it. Codex 0.166.0 no longer creates the folders of the 2026-10-04 incident (tested).
  2. A review's finding now holds `path` always, a text or `null`, for both tools: one schema both providers' strict
     modes accept rather than one rewritten per adapter.
  3. Codex's `input` tokens are `input_tokens` less `cached_input_tokens`; `output` holds the reasoning; the model
     observed is unknown, since its stream does not name it.
  4. `error` notices are admitted anywhere after the turn's start (a real run wrote them while reconnecting); a
     `turn.failed` voids the stream, which gives the same report as reading it.
  5. What 0.166.0 offers its model and no setting removes (a JavaScript `exec` over its tools, the tools to start
     other agents) is declared under T-22; another agent's item voids the stream.
  6. `CODEX_THREAD_ID` joins Codex's markers: seen inside its sandbox, and set whatever its network.
- Codex, two passes, confirmed blocking findings by pass: 1, 0.
  1. Pass 1 (`aff72d6`): P9D-1, a JSON member named twice hid an anomaly behind a valid value (`serde_json` keeps the
     last), in both adapters' grammars, Claude Code's of part 9c included. Corrected by `adapter::json`
     (`3951b86`); each new test failed without it.
  2. Pass 2 (`3951b86`): P9D-1 closed in every form Codex built (7,455 variants); **accept**. P9D-2, not blocking: no
     test held an item's kind across an update or a completion; tested in `1c09fa0`, the test failing with the check
     removed.
  Codex's sandbox cannot run `bwrap`, `pasta` or bind sockets; those tests ran on this machine.
- The integrator's own review before the first pass turned off Codex's `view_image` (a read by path outside the
  sandbox of the model's commands), and found the reviewer's preflight step passing on a failed redirection to
  `/dev/null`; it now passes only on "Read-only file system", and a fake sandbox that leaves the copy writable fails it.
- Mutation, with `--features fabrewk-core/fault-injection`, `--test-workspace true`, `ulimit -v 1600000`, `--timeout
  180`, in its own user scope, over the part's diff (`1c09fa0`): 169 mutants in 2 hours, 139 caught, 14 unviable, 16
  missed. Two (the `turn.failed` arm) were equivalent, so the arm was removed; one is declared equivalent:
  `Strict::expecting` only words the error `json` discards. The other thirteen are tested (`e2e3465`). The reruns over
  the corrections: Codex's `stream.rs` 110 mutants, 106 caught, 4 unviable; the core's `segments` and reader 27, all
  caught but `expecting` and one that a new case caught on its own rerun (17 of 17).
- Clean clone of `e2e3465`: 542 tests (10 ignored: the real tools' and a kept stream's), `crash` 3 and `faults` 20
  green, the musl build, `budget check` (the core 8,985 of 11,000, the backend 276 of 400, Claude Code's adapter 264
  of 600, Codex's 313 of 600, 5 direct runtime crates, 31 in the release build, the binary 2,429,288 bytes), clippy
  with and without `fault-injection`, fmt and `cargo deny` clean; `--version --json` lists no feature.
- Measured: the core +109 (projected 30), Claude Code's adapter −31 (its `segments` moved), Codex's 313 (projected
  about 500).
- Not done: `run` dispatching at the command line, a preflight's pass kept as verified by use, a credential refused
  when it expires or would be renewed within the attempt (D-079, D-081), the packet's size bound (part 9e); the
  reviewer's dispatch and PD-25 and PD-26 through the whole flow (part 9f).

## D-082 — Part 9e: the run halts on its candidate until part 9f; the budget per authorization and per stage

- Status: decided
- Decided: 2026-11-12
- Origin: maintainer XEL in session 34's chat, answering the integrator's two questions before any code: "Detener
  CONFINEMENT_UNAVAILABLE (Recomendado)" and "Tarea desde su autorización (Recomendado)".
- Decision:
  1. Until part 9f runs the checks and the review, `run` halts a task whose candidate it recorded with
     `CONFINEMENT_UNAVAILABLE`, its reason saying where the candidate is and that part 9f builds the rest, exit 4, as
     `run` refused until part 9e with that code (D-053 point 4). Part 9f replaces the halt with the gate.
  2. A task's limits (dispatches, minutes, tokens) count from its last authorization, so that `authorize` grants them
     anew; its stage's budget counts every attempt of the stage's tasks, a cap that no authorization raises (spent,
     the stage's remaining tasks can only be rejected). Tokens counted are those read fresh and written, never those
     read from the cache; an unknown part, with a token limit, spends the limit.
- Reason: the specification leaves both open. The alternatives offered: leaving the task `running`, for the next
  write command to halt `OUTCOME_UNKNOWN` (calling unknown an outcome that is known); a stage budget renewed with each
  task's authorization (no longer a cap); no stage budget until a later part.
- Consequences: `core-specification.md` ("as part 9e fixed it"); `plan.md`; R-043.
- Sources: chat of session 34; `core-specification.md` REQ-RUN-2, REQ-TASK-3, REQ-TASK-5; D-053 point 4.

## D-083 — Part 9e: Git's own output kept as evidence by part 9g, for every command

- Status: decided
- Decided: 2026-11-12
- Origin: maintainer XEL in session 34's chat, after Codex's third pass (P9E-11): "Llevarlo a la parte 9g
  (Recomendado)".
- Decision: since phase 7 every Git command the core runs has its standard error discarded and its exit status
  unrecorded (`record/git.rs`), so a failure of Git reaches the person and the record as `TOOL_FAILED` with the core's
  words alone. Part 9g, which handles Git's failures at integration (PD-11), changes that for every command: Git's
  standard error and exit status kept in `local/evidence/`, named by the failure, as REQ-RUN-6 and REQ-REG-13 ask.
  Part 9e corrects only its own part of P9E-11: a failure of Git while a candidate is made names an evidence file of
  its own with the core's words, never the builder's output.
- Reason: the change touches every write path, not the run's; the alternatives offered were doing it in part 9e, or
  declaring Git's text never kept as an interpretation of REQ-RUN-6 and PD-36.
- Consequences: `plan.md` (row 9g gains it); `core-specification.md` ("as part 9e fixed it"); R-043.
- Sources: chat of session 34; Codex's third pass of part 9e (P9E-11).

## D-084 — Part 9e: a narrow fifth pass of Codex

- Status: decided
- Decided: 2026-11-12
- Origin: maintainer XEL in session 34's chat, after Codex's fourth pass, confirmed blocking findings 4, 2, 2, 2 (two
  retries without progress): "Quinta pasada estrecha (Recomendado)".
- Decision: the fifth pass confirms P9E-13 and P9E-14 closed in every form and looks for findings only in the scope
  of their corrections (the recovery under the lock; Git's failures while the candidate is made), as D-079 point 3
  did for part 9c. An accept closes the review.
- Reason: every pass closed all earlier findings and found new, narrower windows of the same two classes. The
  alternatives offered: a whole fifth pass; closing with P9E-13 and P9E-14 corrected and declared, unconfirmed.
- Consequences: R-043.
- Sources: chat of session 34; Codex's passes 1 to 4 of part 9e.

## R-043 — Session 34, part 9e: the run, building side

- Status: recorded
- Date: 2026-11-12
- Who: integrator Claude Code, `claude-opus-5-5`, effort high (D-078). Reviewer: Codex `codex-cli 0.166.0`,
  `gpt-6.1-sol`, effort high, launched as the `codex-review` skill says.
- Start: HEAD `7a33d28`, tree clean.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, `confinement-design.md` 3 to 7, `core-specification.md` (TASK, RUN with
  its amendments, REG-5, REG-10, REG-13, CLI-3, sections 3 to 5, 7, 11.2 to 11.5, 12, the 9a to 9d paragraphs), this
  file (D-079 to D-081, R-039 to R-042); the skills `verify-part`, `codex-review` and `differential-review`; the code
  of `cli/` (`tasks.rs`, `write.rs`, `mod.rs`, `decl.rs`, `status.rs`), `adapter.rs`, `confine.rs`, `candidate.rs`,
  `detect.rs`, `table.rs`, `task.rs`, `view.rs`, `record/` (`store.rs`, `fault.rs`, `time.rs`, `git.rs`), both adapters
  and the backend, and their tests. No credential of the person's was read: a read of only the shape of Codex's
  `last_refresh` and Claude Code's `expiresAt` was refused by this session's permission classifier and not attempted
  again, so the format `expiry` reads (RFC 3339 in UTC with `Z`; milliseconds) is from the tools' documentation and
  parts 9c and 9d, unchecked against the files (interpretation 6).
- Decided by the maestro in this session: D-082, D-083, D-084.
- Written: `crates/core/src/cli/run.rs` (new: the run's guards, budget, dispatch, attempt, candidate, halts);
  `cli/write.rs` (a plan's continuation under the lock, REQ-RUN-5's recovery before every command's plan, under the
  lock too); `cli/tasks.rs` (`run` and `authorize --run` through it); `cli/status.rs` (a halted task's halt,
  `fabrewk.subject/3`; `local/tmp/` never walked while a cell's directories are there); `cli/decl.rs` (exit 4, the
  run's moves and refusals, `fabrewk.run/1`, `fabrewk.authorized/2`); `table.rs` (the run bound to the authorization
  in force, an attempt's end naming its attempt, `ATTEMPT_REPLACED`); `task.rs` (the five payloads); `view.rs`
  (S-5's usage); `adapter.rs` and both adapters (`expiry`, D-079, D-081); `confine.rs` and the backend (`system`, S-6);
  `code.rs` and both catalogues (`ATTEMPT_REPLACED`, `BASE_MOVED`, `BUDGET_EXHAUSTED`, `OUTCOME_UNKNOWN`, the run's
  words); `record/time.rs` (`seconds_of`); `record/fault.rs` (`FABREWK_FAULT_MINUTE`); the tests of the core, both
  adapters and the binary, with `crates/fabrewk/tests/run.rs` (new) and a fake builder fixture; the reference
  documentation regenerated; the paragraph "The run, building side, as part 9e fixed it" in
  `core-specification.md`; `plan.md`; `STATUS.md`; D-082 to D-084 and this entry.
- Interpretations the maestro may want to read (each in the new paragraph):
  1. A guard of `run-started` whose code is a halting one (REQ-TASK-5, `DISK_LOW`) halts the task from `authorized`
     with no `run-started`, exit 4, rather than refusing, since only an authorization moves a halted task and grants
     it more (REQ-TASK-5's criterion asks exit 4 of each halting code); the others refuse, nothing written.
  2. The run holds the lock to its end (REQ-REG-5 says "for its whole run"), so a task found `running` by a writer
     has no live process: REQ-RUN-5 needs no process identifier. Every other write command meanwhile (`sweep` and
     `propose` included) is `LOCK_HELD`, for up to the profile's minutes; reading commands are not.
  3. An attempt is allowed the profile's minutes, less when the task's or the stage's minutes left are fewer; ended
     at a time so lessened, `BUDGET_EXHAUSTED`, else `TOOL_FAILED`. Tokens counted are `input` plus `output` (D-082).
  4. `BASE_MOVED` reads the last dependency's integration from the record branch's history (the candidate itself or
     a `Fabrewk-Candidate` trailer, REQ-GATE-8), which part 9g must write so.
  5. A failure after a successful builder (the change set's refusals, Git's) halts with no evidence of the builder's;
     a Git failure keeps the core's words in its own evidence file (D-083 for Git's own text).
  6. Codex's `last_refresh` is read as RFC 3339 in UTC with `Z` and a fraction of up to nine digits, as its source
     serializes it; another form, or a time before 1970, cannot be read and refuses the dispatch.
  7. What the run's payloads copy from a tool, a file system or the catalogue is any text, the empty one included;
     the co-author's model falls back to the one requested when the stream names none or only white space.
  8. The packet is bounded at 120,000 bytes, under Linux's 128 KiB per argument; a longer one halts `TOOL_FAILED`.
- Codex, five passes (the fifth narrow, D-084, launched three times), confirmed blocking findings by pass: 4, 2, 2, 2,
  0.
  1. Pass 1 (`46b4102`): P9E-1 (`status` walked an open attempt's copy), P9E-2 (the home's absolute path in a halt's
     reason), P9E-3 (a run naming an authorization not in force), P9E-6 (a year before 1970 wrapped into a future
     renewal); P9E-4 and P9E-5, not blocking (a recovered copy left; an attempt number that overflowed).
  2. Pass 2 (`39fbea5`): all closed; P9E-7 (`status` walked a copy kept unread), P9E-9 (the last integration taken
     from the clock); P9E-8, not blocking (a recovery stopped after its halt left the copy).
  3. Pass 3 (`6a17f44`): all closed; P9E-11 (a Git failure at the candidate named the builder's evidence; Git's own
     text kept nowhere, the core-wide part decided for part 9g, D-083), P9E-12 (a layer a stricter policy made a
     widening granted); P9E-10, not blocking (the tool's version not in the event, PD-29).
  4. Pass 4 (`3135b33`): all closed; P9E-13 (recovery decided before the lock only), P9E-14 (a Git failure reading
     the branch escaped the halt). Two retries without progress: the maestro chose a narrow fifth pass (D-084).
  5. Pass 5 (`4ac9904`), narrow: two launches were cut by Codex's provider ("flagged for possible cybersecurity
     risk") and count as none; before their ends they named three cases of the corrected classes, all confirmed and
     corrected (`0bdf576`, `bd2baa9`: a path of white space and an empty model leaving events the replay refused, a
     refusal after the builder naming no evidence). The third launch (`bd2baa9`) confirmed everything closed:
     **accept**.
  Each new test failed without its correction, checked by hand. Codex's sandbox cannot run `bwrap`, `pasta` or bind
  sockets; those tests ran on this machine.
- Found and corrected by the integrator's own review before the first pass: halts' reasons held absolute paths, and
  the export's evidence file was not `0600`. Found and left: a write stopped while writing a view leaves
  `.STATUS.md.tmp` in the domain, which `check` reports (`DOMAIN_UNKNOWN_ENTRY`) and no writer removes (part 8c's;
  destination part 9i's crash suite).
- Mutation, with `--features fabrewk-core/fault-injection`, `--test-workspace true`, `ulimit -v 1600000`, `--timeout
  180`, in its own user scope, over the part's diff (`bd2baa9`, both adapters included): 335 mutants in 2 hours, 117
  caught, 210 unviable, 4 timed out (the suite hung on the mutant: detected), 4 missed. One replay arm reachable only
  where the next gives the same code, removed; the place after a task's own authorization written as its event's
  `seq` (two mutants of unreachable states); `authorize --run` of a task authorized before, tested. The run over that change: 12 mutants, 12 caught. None declared equivalent.
- Clean clone of `c495157`: 570 tests (10 ignored: the real tools' and a kept stream's), `crash` 3 and `faults` 23
  green, the musl build, `budget check` (the core 9,751 of 11,000, the backend 278 of 400, Claude Code's adapter 269
  of 600, Codex's 319 of 600, 5 direct runtime crates, 31 in the release build, the binary 2,756,968 bytes), clippy
  with and without `fault-injection`, fmt and `cargo deny` clean; `--version --json` lists no feature.
- Measured: the core +766 (projected 600); the adapters +5 and +6, the backend +2. Left for parts 9f to 9i: 1,249
  lines against about 1,450 projected (`confinement-design.md` section 7): the maestro may want to measure the
  ceiling again before part 9g.
- Not done: the checks, the reviewer's dispatch, rounds and verification (part 9f), which replace D-082's halt;
  integration and Git's own output kept as evidence (part 9g, D-083); the flows and the crash suite over F-1, the
  view's temporary file included (part 9i).

## D-085 — Part 9f: a check with network refused, a later round built on the last candidate, checks outside the budget

- Status: decided
- Decided: 2026-11-12
- Origin: maintainer XEL in session 35's chat, answering the integrator's three questions before any code: "Rehusar
  (Recomendado)", "Candidato anterior (Recomendado)" and "No cuentan (Recomendado)".
- Decision:
  1. A required check with `checks.<name>.network = true` cannot be confined by the bubblewrap backend, which cannot
     keep a cell with a network from nesting (part 9a): the run halts the task `CONFINEMENT_UNAVAILABLE` at its
     confinement guard, before the builder is dispatched, its reason saying so. A check with network is left for after
     the pilot.
  2. A building round after the first (after a review's reject or a failed check) exports the task's last candidate
     into its copy, so that the builder corrects rather than rebuilds; the change set is still computed against the
     base and the new candidate's parent is still the base. REQ-RUN-3 is amended so.
  3. A check is not a dispatch: its time counts against none of the task's or the stage's limits (D-082); each check
     is limited by its own `checks.<name>.minutes`.
- Reason: the specification left the first open since part 9a, and the second and third are not settled by REQ-RUN-3
  and D-082. The alternatives offered: a check with network in a cell that may nest, declared under T-08; every round
  from the base; a check's time spent from the task's and the stage's minutes.
- Consequences: `core-specification.md` ("as part 9f fixed it"); R-044.
- Sources: chat of session 35; `confinement-design.md` 3.4 and 3.7; `core-specification.md` REQ-RUN-3, REQ-GATE-2,
  REQ-GATE-6; D-082.

## R-044 — Session 35, part 9f: checks, review and verification

- Status: recorded
- Date: 2026-11-13
- Who: integrator Claude Code, `claude-opus-5-5`, effort high (D-078). Reviewer: Codex `codex-cli 0.166.0`,
  `gpt-6.1-sol`, effort high, launched as the `codex-review` skill says.
- Start: HEAD `91b00b3`, tree clean. The session spanned two days and a restart of the machine after a system
  out-of-memory (below); nothing in the repository was lost.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, `confinement-design.md` 3.4, 3.7 and 3.8, `core-specification.md`
  (TASK-5, RUN-2, RUN-3, RUN-5, RUN-8, RUN-9, GATE-1 to GATE-6, GATE-10, section 4, 11.3 and 11.4's cases of the part,
  12's criteria, the 9a to 9e paragraphs), this file (D-082 to D-084, R-041 to R-043); the skills `verify-part`,
  `codex-review` and `differential-review`; the code of `cli/` (`run.rs`, `tasks.rs`, `write.rs`, `decl.rs`,
  `status.rs`), `adapter.rs`, `confine.rs`, `candidate.rs`, `detect.rs`, `table.rs`, `task.rs`, `code.rs`,
  `record/fault.rs`, both adapters' audits, and their tests. No credential of the person's was read.
- Decided by the maestro in this session: D-085.
- Written: `crates/core/src/gate.rs` (new: the required checks, a check's identity, cell and outcome, whether a
  verification holds, the rounds and their measure, what a round leaves for the next); `cli/run.rs` (the rounds: the
  limit, the builder on the last candidate, the checks, the reviewer, `NO_PROGRESS`, `task-verified`, revalidation, the
  provider's rule; the run's guard `LOCK_HELD` that read a task left `running`, unreachable under the lock, removed);
  `cli/write.rs` (`next`; a recovery removing the stopped run's checks' directories); `cli/tasks.rs` (`authorize --run`
  naming `review.differentProvider`); `cli/decl.rs`, `cli/status.rs` (`status.limited`); `task.rs` (the three payloads,
  `definition`); `code.rs` and both catalogues (`EVIDENCE_STALE`, `NO_PROGRESS`, `PROVIDER_MISSING`,
  `REVIEW_UNEXAMINED`, `ROUNDS_EXHAUSTED`, the run's words); `detect.rs` (`in_cell`); `adapter.rs` (`CELL_ENV`);
  `record/fault.rs` (`FABREWK_FAULT_PLATFORM`, which no test uses yet: part 9g's `EVIDENCE_STALE` through the flow
  may); the tests of the core and the binary, with `crates/core/tests/gate.rs` (new) and a fake reviewer in the
  binary's fixture; the reference documentation regenerated; the paragraph "Checks, review and verification, as part
  9f fixed them" in `core-specification.md`; `plan.md`; `STATUS.md`; D-085 and this entry.
- Interpretations the maestro may want to read (each in the new paragraph):
  1. A failed check counts one confirmed blocking finding in the round's measure and ends the round with no
     reviewer; `NO_PROGRESS` is evaluated at a round's end with findings, before the round limit of the next.
  2. The rounds and their counts are those since the authorization in force, as D-082 counts the other limits; a
     revalidation's first round dispatches no builder and counts.
  3. A review whose audit counted no examination is `REVIEW_UNEXAMINED` whatever its verdict, and is recorded; a
     stream the audit voided records no review.
  4. Only an authorization naming `--widen review.differentProvider` lets a run go on with the reviewer's provider
     the builder's; a layer that sets the key does not (P9F-1).
  5. A stage's check that the policy adopted after it defines otherwise is `CHECK_DUPLICATE` at the run (P9F-2).
  6. Since the table halts no `verified` task, what would halt it before the lock refuses instead, nothing written;
     under the lock its `run-started` is written before the confinement guard.
  7. A check's tool must resolve on the cell's own `PATH` (`/usr/bin:/bin`); its version is the first line its
     `--version` prints, `null` when none, and both enter the identity.
  8. The previous round read from the record is the events after the last candidate or `run-started`; a failed
     check carries the last 4,000 characters of each channel, read as UTF-8 with replacements.
- Codex, two passes, confirmed blocking findings by pass: 2, 0.
  1. Pass 1 (`e88105d`): P9F-1 (`authorize --run` took a layer's `review.differentProvider` as the authorization's
     naming), P9F-2 (a stage's check later defined otherwise by the policy run with the stage's definition). Both
     corrected (`a5028cd`), each new test failing without its correction.
  2. Pass 2 (`a5028cd`): both closed, nothing new: **accept**.
  Found and corrected by the integrator's own review before the first pass: a halt from `verified` the table refuses
  (now `run-started` first), and a recovery that removed an earlier run's checks' directories (now only those after
  the stopped run's `run-started`). Codex's sandbox cannot run `bwrap`, `pasta` or bind sockets; those tests ran on
  this machine.
- Mutation, with `--features fabrewk-core/fault-injection`, `--test-workspace true`, `ulimit -v 1600000`, in its own
  user scope, over the part's diff (`a5028cd`, `-j 2`, `--timeout 180`): 248 mutants in 2 hours, 60 caught, 159
  unviable, 27 timed out, 2 missed. Under two jobs a passing suite took 174 s, so a timeout there did not show a
  loop: the 27 and the 2 were run again alone (`-j 1`, `--timeout 400`): 18 caught, 2 missed (a check's tool never
  found, the unreachable guard), and 9 whose linker the scope's memory killed (`ld` signal 9), counted as none. The
  machine then ran out of memory and was restarted: the scope's 3 GB cap was above what the desktop left free (2 GB).
  The 2 missed were tested or removed (`31e0f5f`), with `platform` made empty, missed in the first run, tested
  (`2a40fdf`). The 9 and `in_cell` run again (2 GB, `--jobserver-tasks 2`, no debug information, which tests the
  same code): 9 caught, 1 missed (`previous` reading from the round's own event, which carries nothing: the code
  simplified so the mutant no longer exists, `5428a62`); the run over that change: 4 mutants, 4 caught. None declared
  equivalent.
- Clean clone of `5428a62`: 584 tests (10 ignored: the real tools' and a kept stream's), `crash` 3 and `faults` 23
  green, the musl build, `budget check` (the core 10,319 of 11,000, the backend 278 of 400, Claude Code's adapter 269
  of 600, Codex's 319 of 600, 5 direct runtime crates, 31 in the release build, the binary 2,822,504 bytes), clippy
  with and without `fault-injection`, fmt and `cargo deny` clean; `--version --json` lists no feature. Each step ran in
  a user scope capped at 2.3 GB with two build jobs.
- Measured: the core +568 (projected 550); the backend and both adapters unchanged. Left for parts 9g to 9i: 681
  lines against about 900 projected (`confinement-design.md` section 7): the ceiling is the maestro's to measure again
  before part 9g (`AGENTS.md` "Code").
- Not done: `accept` and integration, `EVIDENCE_STALE` refused at both and REQ-GATE-10's recomputation before the
  move (part 9g); Git's own output kept as evidence (part 9g, D-083); a Codex fake through the whole flow (each
  adapter's audit is tested at its own level, parts 9c and 9d); the flows and the crash suite over them (part 9i).
  `verify-part`'s memory cap (3 GB) is the maestro's to lower: on this machine with the editor open about 2 GB is
  free; the last reruns used 2 GB and `--jobserver-tasks 2` (1.8 GB killed the baseline's build).

## D-086 — Part 9g: the core's ceiling at 12,000, `accept` with `--seen`, Git's output kept by write commands, a verification bound to its policy

- Status: decided
- Decided: 2026-11-13
- Origin: maintainer XEL in session 36's chat, answering the integrator's four questions before any code: "12,000
  (Recomendado)"; "Con --seen (Recomendado)"; "Solo escritura (Recomendado)"; "Política distinta (Recomendado)".
- Decision:
  1. The core's ceiling is 12,000 counted lines (`tools/budget`), measured again at each part. With part 9f the core
     counted 10,319, leaving 681 for parts 9g to 9i against about 900 projected, before D-083 added Git's output to
     part 9g.
  2. `accept` takes `--seen <digest>`, as `authorize` does (D-062 point 3): `fabrewk accept <task> --seen <digest>
     --say <words>`. `status <task>` shows what an acceptance binds, with its digest; a digest that is not the one shown
     now is `TASK_CHANGED`, and `accept` without `--seen` is a usage error (exit 2), as for `authorize` (D-068 point 2).
     The event carries that digest as `shown`.
  3. D-083 applies to write commands: a Git command that fails in a write command has its standard error and its exit
     status kept in `local/evidence/` (`0700`, `0600`), and the refusal or the event names that file. Reading commands
     (`status`, `config`, `check`) write nothing (REQ-REG-12): their words say Git's exit status, and nothing is kept.
  4. A verification also binds the policy it was recorded under: the evidence is stale when the policy in force is not
     the one in force when its review was recorded, as every check's identity already makes it, now also for a task
     with no required check (PD-12's "reviewer profile unchanged in the adopted policy"). This holds at `accept`, at
     integration, and for `run`'s revalidation (REQ-RUN-9).
- Reason: the size projection (R-044); the same rule for both signed bindings of REQ-GATE-7; REQ-REG-12; PD-12. The
  alternatives offered: a ceiling of 11,500 or kept at 11,000; `accept` printing and signing in one command; reading
  commands writing evidence, or showing Git's text; staleness only when the review profile's effective settings
  changed, or left to the checks' identities.
- Consequences: `tools/budget/src/budget.rs` and its tests; `plan.md`; `core-specification.md` (section 5's synopsis
  of `accept`, the paragraph of part 9g); R-045.
- Sources: chat of session 36; R-044 (measure); D-062, D-068, D-083; `core-specification.md` REQ-GATE-3, REQ-GATE-7,
  REQ-GATE-8, REQ-REG-12, PD-12.
- Supersedes: none.

## D-087 — Part 9g: a precondition false before the signature refuses; `run` on an accepted task takes words it does not record

- Status: decided
- Decided: 2026-11-13
- Origin: maintainer XEL in session 36's chat, answering the integrator's two questions before any code: "Rechazar sin
  escribir (Recomendado)"; "Se exigen, no se graban (Recomendado)".
- Decision:
  1. The specification disagrees on the first of REQ-GATE-8's three points: section 4 makes `EVIDENCE_STALE` a guard of
     `task-accepted` (a refusal, nothing written), while PD-12 and 11.2 rule 4 have a false precondition at each point
     write `task-accepted` and `merge-failed`. Before the signature, every precondition of the integration (evidence
     current, the base unmoved or the branch already at the candidate, the working tree clean at the touched paths, the
     third guard on protected paths, Git's locks) refuses `accept` with its code and nothing written: nobody signs what
     cannot integrate now. After the signature, just before the move and with the branch at the candidate, a false
     precondition writes `task-accepted` (when the command is `accept`) and `merge-failed`, the task staying
     `accepted`. PD-12's first point is read so.
  2. `run <task> --say` on an `accepted` task finishes its integration under the same checks (REQ-GATE-8). Its words are
     required and checked as for every run (the person by their key, `WORDS_MISSING`, `WORDS_COPIED`), but the command
     writes only Fabrewk's events (`merge-failed` or `task-integrated`), since the table records no person's move from
     `accepted` but a rejection or an authorization; the acceptance already signed carries the person's words.
- Reason: the two texts cannot both hold; a signature over a state that cannot integrate records a decision nobody can
  act on. The alternatives offered: signing and recording `merge-failed` at every point, as PD-12 reads literally; the
  table's guards refusing and `task-integrated`'s recording; a new signed event that carries the words; `run` without
  `--say` for an accepted task.
- Consequences: `core-specification.md` (the paragraph of part 9g; PD-12's first point); R-045.
- Sources: chat of session 36; `core-specification.md` section 4, REQ-GATE-8, 11.2 rule 4, PD-12.
- Supersedes: none.

## D-088 — Part 9g: P9G-15 corrected, and a narrow eighth pass of Codex

- Status: decided
- Decided: 2026-11-13
- Origin: maintainer XEL in session 36's chat, after Codex's seventh pass (confirmed blocking findings by pass 4, 2, 2,
  1, 3, 2, 1, every one after the first of one class: a Git that fails in a write command read as a state, or losing
  its evidence): "Corregir y pasada estrecha (Recomendado)".
- Decision: the integrator corrects P9G-15's five sites with their tests (the signing key read twice; the record
  commit's compare-and-swap that a moved branch refuses, rebuilt or `RECORD_COMMIT_RACE`; the person's index that does
  not follow the record commit, P7D3A-21, where the command still reports the commit made and its words also name
  Git's evidence; the integration's failure replaced by an auxiliary reading that fails too; a ref that names no
  commit when its auxiliary listing fails too). An eighth pass confirms P9G-15 closed and looks for findings only in
  the scope of those corrections, as D-084 did for part 9e; an accept closes the review.
- Reason: every pass closed the earlier findings and found narrower windows of the same class, provoked by chains of
  injected failures. The alternatives offered: correcting and closing with P9G-15 unconfirmed; correcting two sites
  and declaring the other three residual.
- Consequences: R-045.
- Sources: chat of session 36; Codex's passes 1 to 7 of part 9g; D-083, D-084, D-086.
- Supersedes: none.

## D-089 — Part 9g: the evidence a failure names travels with it; what a command went on from is in its document

- Status: decided
- Decided: 2026-11-13
- Origin: maintainer XEL in session 36's chat, after Codex's narrow eighth pass (confirmed blocking findings by pass 4,
  2, 2, 1, 3, 2, 1, 4: two retries without progress, which produce alternatives): "Cambiar el diseño (Recomendado)".
- Decision: the design changes, so that no state outside a failure says which evidence it names. The evidence travels
  within the failure itself: a failed Git's words name its directory, and a failure that follows another names both,
  in order; the event takes the first one its words name, reading that directory's `stderr` and `status`. What failed
  in Git that a write command went on from is held by the command's session, not by the process, survives the
  recovery of a stopped task that the command makes first, and is in the command's document as `recovered` (each a
  text, its words naming the evidence) as well as in its words; every write command's document gains the field, under
  a new schema. P9G-16 to P9G-19 are corrected on that design, and a ninth pass, narrow as D-088's, confirms them and
  looks for findings only in the scope of the change; an accept closes the review.
- Reason: every correction of P9G-15 patched one of two process-wide states; each pass found a path that state did not
  follow. The alternatives offered: declaring P9G-16 to P9G-19 residual, every failed Git's output being kept anyway;
  correcting them on the current design and closing without a pass.
- Consequences: `record/git.rs`, `record/store.rs`, `cli/write.rs`, `cli/adopt.rs`, `cli/accept.rs`, `cli/decl.rs`;
  `core-specification.md` (the paragraph of part 9g); R-045.
- Sources: chat of session 36; Codex's passes 1 to 8 of part 9g; D-083, D-086, D-088.
- Supersedes: none.

## D-090 — Part 9g: P9G-20 and P9G-21 corrected, P9G-22 declared, and the review closed

- Status: decided
- Decided: 2026-11-13
- Origin: maintainer XEL in session 36's chat, after Codex's narrow ninth pass (confirmed blocking findings by pass 4,
  2, 2, 1, 3, 2, 1, 4, 3; P9G-16 to P9G-19 confirmed closed): "Corregir dos, declarar uno (Recomendado)".
- Decision: the integrator corrects P9G-20 (what a recovery went on from is named by a refusal of the command that
  follows it) and P9G-21 (the evidence a failure names is read from its directory's own name, the last component of
  the path its words give, never from a component of the repository's path), each with a test that fails without its
  correction. P9G-22 is declared: an internal error (exit 70, `fabrewk.internal/1`) says only its path, by design, and
  what failed in Git before it stays kept in `local/evidence/`, named by no word. The review closes without another
  pass.
- Reason: nine passes; the last three each found narrower paths of D-089's change. The alternatives offered:
  correcting P9G-22 too, under a new schema of the internal error's document, and closing; correcting the three and a
  tenth narrow pass.
- Consequences: `core-specification.md` (the paragraph of part 9g); R-045.
- Sources: chat of session 36; Codex's passes 1 to 9 of part 9g; D-089.
- Supersedes: none.

## R-045 — Session 36, part 9g: acceptance and integration

- Status: recorded
- Date: 2026-11-13
- Who: integrator Claude Code, `claude-opus-5-5`, effort high (D-078). Reviewer: Codex `codex-cli 0.166.0`,
  `gpt-6.1-sol`, effort high, launched as the `codex-review` skill says.
- Start: HEAD `730f243`, tree clean. The machine lost its network three times during the session; one Codex pass
  waited and resumed by itself, and two of the backend's tests that need a route outwards failed while it was down and
  passed once it was back (no finding).
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, `confinement-design.md` 3.8 and 7, `core-specification.md` (GATE-1 to
  GATE-10, REG-10 to REG-13, RUN-6, RUN-9, sections 3 to 5, 11.2, 11.4's cases of the part, 12's criteria, the 9a to
  9f paragraphs), this file (D-068, D-082 to D-085, R-043, R-044); the skills `verify-part`, `codex-review` and
  `differential-review`; the code of `cli/` (`run.rs`, `tasks.rs`, `write.rs`, `status.rs`, `decl.rs`, `mod.rs`,
  `adopt.rs`), `record/` (`git.rs`, `store.rs`, `fault.rs`), `candidate.rs`, `gate.rs`, `table.rs`, `task.rs`,
  `view.rs`, `code.rs`, both catalogues, and their tests. No credential of the person's was read.
- Decided by the maestro in this session: D-086 to D-090.
- Written: `crates/core/src/merge.rs` (new: a commit's product tree, the change set between two commits and its third
  guard, what the index and the working tree hold at the touched paths, the integration commit, the move, the working
  tree and the index brought to the candidate); `crates/core/src/cli/accept.rs` (new: `accept`, what an acceptance
  binds, the preconditions at their three points, `merge-failed`, `task-integrated`, `run` on an accepted task, where
  the record branch is); `cli/tasks.rs` (`reject` saying where the branch is, `MERGE_IN_PROGRESS` with the branch at the
  candidate, `run` finishing an accepted task); `cli/run.rs` (the task's settings shared, `BASE_MOVED` reading the
  commit an integration left); `cli/status.rs` (`accepting`, `fabrewk.subject/4`); `cli/write.rs` (Git's evidence kept
  from the write path's start, the session's notes, a refusal naming what came before it); `cli/adopt.rs`
  (`recovered`); `cli/decl.rs` (`accept`, the schemas); `record/git.rs` (Git's standard error captured, `failed`,
  `kept_in`, Git's exit status 1 alone as an absence, the record commit's rounds naming a refused compare-and-swap);
  `record/store.rs` (the integration's commit as the record commit's start, the notes); `record/fault.rs`
  (`FABREWK_FAULT_GIT`, `FABREWK_FAULT_GIT_THEN`); `candidate.rs` (the lanes guard shared); `gate.rs` (a verification
  bound to its policy); `table.rs` (an acceptance bound to the last candidate); `task.rs` (the four payloads);
  `view.rs` (`task-merge`); `code.rs` and both catalogues (`WORKTREE_DIRTY`, the words of `accept`, `reject`, the
  recovered notes); `identity.rs` (the key read once); `tools/budget` (the ceiling, D-086); the tests of the core, the
  binary's `run.rs`, `tasks.rs`, `writing.rs`, `faults.rs` and the new `crates/core/tests/merge.rs`; the reference
  documentation regenerated; the paragraph "Acceptance and integration, as part 9g fixed them" in
  `core-specification.md`; `plan.md`; `STATUS.md`; D-086 to D-090 and this entry.
- Interpretations the maestro may want to read (each in the new paragraph):
  1. The base an integration compares with is the candidate's parent (REQ-RUN-4).
  2. The third guard is REQ-RUN-3's rules on the whole trees: protected components (with empty, `.` and `..` ones),
     paths that are not text and the lanes, so `LANE_VIOLATION` is among an integration's codes.
  3. The integration commit is the person's Git's as author and committer; its message is the candidate's, with
     `Fabrewk-Candidate` after its trailers.
  4. A submodule's path in the working tree is never written but made as an empty directory where none is; its index
     entry is written.
  5. Index entries are written with no stat data, so a configured clean filter may make the person's `git status`
     show a file changed whose bytes are the object's (T-28's other side).
  6. `merge-failed` exits 4 and waits for a maestro (`task-merge`); `moved` is `null` when the branch cannot be read.
  7. PD-11's leftover index lock: every write command refuses it before writing (P7B-6's design), so it is no merge
     failure; PD-11 is tested with a Git error in one task's move, other tasks building meanwhile.
  8. A Git failure in a reading command is said with its exit status and keeps nothing (D-086 point 3); `status`
     refuses `TOOL_FAILED` when what an acceptance binds cannot be read.
  9. `own-repository`: `check` reports the domain's own `.git` as `ALIAS_INVALID` (C8), older than this part (found
     by the fast-forward test, which runs `check` only in `unpublished`); destination: the maestro, before phase 10's
     installation makes such a domain.
- Codex, nine passes and one launch cut by its provider, confirmed blocking findings by pass: 4, 2, 2, 1, 3, 2, 1, 4,
  3.
  1. Launch 1 (`b3ecd14`): cut by the provider ("flagged for possible cybersecurity risk"), counted as none; before
     its end it explored a file that becomes a directory, which the integrator found broken and corrected first
     (`0363bec`).
  2. Pass 1 (`0363bec`, a neutral prompt): P9G-1 (an edit after the reading overwritten), P9G-2 (`moved` a literal on
     a retry), P9G-3 (a failed Git read as an absent committed or staged file), P9G-4 (a refused compare-and-swap's
     output lost).
  3. Pass 2 (`b5c3ed3`): all closed; P9G-5 (an index entry a touched path displaces), P9G-6 (`moved` still a literal
     in two failures).
  4. Pass 3 (`387151b`): all closed; P9G-7 (a touched entry hiding another below it), P9G-8 (the evidence replaced by
     the reading of `moved`).
  5. Pass 4 (`8704fea`): all closed; P9G-9 (a ref naming no commit losing its output).
  6. Pass 5 (`0e1bcd1`): all closed; P9G-10 to P9G-12 (`reject` taking a failed reading for a place, a candidate's
     parent taken for absent, the listing hiding the reading's output); corrected by one rule: Git's exit status 1
     alone is an absence.
  7. Pass 6 (`b391dd2`): all closed; P9G-13, P9G-14 (what an acceptance binds and the branch checked out taken from a
     failed Git); the integrator swept the class (the key, the main working tree, the base's ignore rules, the
     candidate's branch).
  8. Pass 7 (`9846a32`): all closed; P9G-15, five sites of the class. The maestro chose to correct them and a narrow
     eighth pass (D-088).
  9. Pass 8 (`c8ff4b6`, narrow): P9G-15 closed but four paths of its corrections, P9G-16 to P9G-19. Two retries
     without progress: the maestro changed the design (D-089).
  10. Pass 9 (`18fe2e0`, narrow): P9G-16 to P9G-19 closed; P9G-20 to P9G-22. The maestro chose to correct P9G-20 and
      P9G-21, declare P9G-22 and close the review (D-090).
  Each new test failed without its correction, checked by reverting the code under test. Two cases are held by the
  design and not provoked by a test, since `FABREWK_FAULT_GIT` cannot fail one of two commands with the same words:
  the key read twice (P9G-15's first site) and a recovery's own note (P9G-17). Codex's sandbox cannot run `bwrap`,
  `pasta` or `ssh-agent`; those tests ran on this machine.
- Found and corrected by the integrator's own review: a third guard with no refusal of empty, `.` and `..` components,
  a scratch directory left by a refusal, a pause after the second check instead of before it, `status` failing on a
  task whose acceptance the table does not allow.
- Mutation, with `--features fabrewk-core/fault-injection`, `--test-workspace true`, `ulimit -v 1600000`, no debug
  information, in its own user scope over the part's diff (`730f243..84b1af0`): 432 mutants, `-j 2`, `--timeout 180`,
  in 5 hours: 242 caught, 108 unviable, 76 timed out, 6 missed. A passing suite took about 175 s there, so the 76 and
  the 6 were run again (`--iterate`, `-j 1`, `--timeout 400`, 2 GB): 19 caught, 42 timed out, 21 missed (a suite took
  190 s). The 6 and the 21 were tested or removed (`7ffb3a2`), each new test failing with its mutant applied by hand;
  most of the 42 hung in `p9g_19`, which waited for its pause with no limit and now waits at most 30 s. The run over
  the diff to that commit, `--iterate`: 148 mutants (those whose lines moved again among them), 99 caught, 46
  unviable, 2 timed out (`apply` and `held` giving nothing, which makes `write::run` read the record and go again
  forever: a loop), 1 missed, declared equivalent: `reject`'s `>` against `>=` between the positions of the last
  acceptance and the last candidate (they are equal only when neither event exists, and then no candidate is located,
  so the branch is not at it). Removed by simplifying: the slice up to an acceptance in `table.rs` (now up to and with
  it, itself no candidate) and the conjunction in `accept.rs`'s `verified` (a filter, then the tree).
- Found by the mutation run, no finding of the part: the idle `pasta` a killed cell leaves (the backend's declared
  residual `network-helper`, part 9a) piles up when the timeout kills tests: 72 in the first run's scope, about 1 GB,
  which slowed every suite; 4 more from this session's own test runs, and 2 from each full suite (its tests that
  kill the binary). The reruns ran a watcher in the scope ending each `pasta` whose namespace's process was gone;
  `verify-part` saying so is the maestro's to change. The scopes left by earlier sessions' runs, each holding such
  helpers, were stopped.
- Clean clone of `7ffb3a2`: 611 tests (10 ignored: the real tools' and a kept stream's), `crash` 3 and `faults` 37
  green, the musl build, `budget check` (the core 11,276 of 12,000, the backend 278 of 400, Claude Code's adapter 269
  of 600, Codex's 319 of 600, 5 direct runtime crates, 31 in the release build, the binary 3,150,216 bytes), clippy
  with and without `fault-injection`, fmt and `cargo deny` clean; `--version --json` lists no feature. The steps ran
  in a user scope capped at 2.5 GB with two build jobs.
- Measured: the core +957 (projected 550: Git's evidence through every write command and the nine passes' corrections
  took the rest); the backend and both adapters unchanged. Left for parts 9h and 9i: 724 lines against about 350
  projected (`confinement-design.md` section 7).
- Not done: the extensions (part 9h); the flows and the crash suite over them, and a view's temporary file left by a
  stopped write (part 9i); `check` on an `own-repository` domain (interpretation 9, the maestro's). P9G-22 was declared
  and not corrected (D-090).

## D-091 — Part 9g's closing: the domain's own `.git` is part of it in `own-repository`

- Status: decided
- Decided: 2026-11-14
- Origin: maintainer XEL in session 36's chat, after the integrator's recommendations on R-045's interpretation 9: "De
  acuerdo en todas tus recomendaciones".
- Decision: REQ-DOM-2 is amended: in `own-repository`, the domain also holds `.git`, the directory of its own
  repository (REQ-DOM-7). `layout` counts `.git` as part of the domain only as a directory and only when the
  publication mode in force is `own-repository`; in another mode, with no mode on record, or as a link, it stays a
  finding as today. The integrator corrects it before part 9g closes, with tests (`check` in both modes of the
  fast-forward test; a `.git` in a `published` domain still a finding), a narrow Codex pass, a mutation run over the
  change and a clean clone.
- Reason: REQ-DOM-2 and REQ-DOM-7 contradicted each other, so `check` failed (`ALIAS_INVALID`, an error of C8) in every
  `own-repository` project; found by part 9g's fast-forward test (R-045, interpretation 9). The alternative offered:
  leaving it to phase 10's installation.
- Consequences: `core-specification.md` (REQ-DOM-2 and the paragraph of part 9g); `crates/core/src/domain.rs`,
  `cli/check.rs`; the provenance entry of session 37.
- Sources: chat of session 36; R-045; REQ-DOM-2, REQ-DOM-3, REQ-DOM-7; D-051.
- Supersedes: none.

## D-092 — The idle `pasta` a killed Fabrewk leaves is ended by the recovery, in part 9i

- Status: decided
- Decided: 2026-11-14
- Origin: maintainer XEL in session 36's chat, the same answer as D-091.
- Decision: the cell's runner writes, for each process a backend started beside a cell, its identifier and its start
  time as the kernel gives it (`/proc/<pid>/stat`); the recovery of a stopped run (REQ-RUN-5) ends each one still
  alive whose start time is the one written, whose executable is the backend's helper (`pasta`) and whose cell's
  process is gone, and removes the file. The residual `network-helper` stays declared, its words saying it lasts
  until the next write command. Assigned to part 9i, whose crash suite kills Fabrewk during a run and checks what it
  leaves; budget about +30 lines (the core and the backend).
- Reason: part 9a found that this machine's SELinux refuses both of `pasta`'s own ways to end with its namespace (the
  watch, and `setpriv --pdeathsig` across its change of domain; R-039, finding 3) and noted that part 9e's recovery
  might end it, which no part did. A mutation run left 72 of them, about 1 GB (R-045). The alternative offered: doing
  it in part 9g's closing.
- Consequences: `plan.md` (row 9i); part 9i's provenance entry; `confine.rs`, `crates/confine-bwrap/`, the recovery.
- Sources: chat of session 36; R-039; R-045; REQ-RUN-5; `confinement-design.md` 3.10.
- Supersedes: none.

## D-093 — `verify-part` amended for this machine, drafted and approved before it is written

- Status: decided
- Decided: 2026-11-14
- Origin: maintainer XEL in session 36's chat, the same answer as D-091.
- Decision: in part 9g's closing the integrator drafts an edit of `.brewstillery/skills/verify-part/SKILL.md` and the
  maestro approves its text in chat before it is written. It covers: the scope at 2 GB with `--jobserver-tasks 2`,
  `ulimit -v 1600000` and no debug information (`CARGO_PROFILE_DEV_DEBUG=0`, `CARGO_PROFILE_TEST_DEBUG=0`); mutants
  with `-j 1 --timeout 400` (a passing suite takes about 190 s even alone), or two jobs and the timeouts again alone;
  a watcher in the scope ending each idle `pasta` whose cell's process is gone, until D-092 is built; looking for
  scopes and `pasta` left by earlier runs before starting; the part's diff written again against the working tree
  before an `--iterate` run when the code changed.
- Reason: the skill's 3 GB and `ulimit -v 1000000` no longer fit this machine (R-044, R-045), and part 9g's first
  mutation run took 5 hours, slowed by idle helpers (R-045).
- Consequences: `verify-part/SKILL.md`; the provenance entry of session 37.
- Sources: chat of session 36; R-044; R-045; D-060.
- Supersedes: none.

## R-046 — Session 37, part 9g's closing: the domain's own `.git` and `verify-part`

- Status: recorded
- Date: 2026-11-14
- Who: integrator Claude Code, `claude-opus-5-5`, effort high (D-078). Reviewer: Codex `codex-cli 0.166.0`,
  `gpt-6.1-sol`, effort high, one narrow pass, launched as the `codex-review` skill says.
- Start: HEAD `13fb764`, tree clean.
- Read: `STATUS.md`, `AGENTS.md`, this file (D-091 to D-093, R-045), `core-specification.md` (REQ-DOM-2, REQ-DOM-3,
  REQ-DOM-7, the paragraphs "The domain, as part 7c2b fixed it" and "Acceptance and integration, as part 9g fixed
  them"), `plan.md`; the skills `verify-part`, `codex-review` and `differential-review`; `crates/core/src/domain.rs`,
  `cli/check.rs`, `crates/confine-bwrap/src/` (how `pasta` is started), and the tests `crates/core/tests/domain.rs`,
  `crates/fabrewk/tests/reading.rs` and `run.rs`. No credential of the person's was read.
- Decided by the maestro in this session: the text of `verify-part`'s edit (D-093), approved in chat: "Apruebo el
  texto o textos que faltaban".
- Written: `crates/core/src/domain.rs` (`layout` takes the publication mode in force; `.git` is part of the domain only
  as a directory and only in `own-repository`, D-091); `cli/check.rs` (C8 passes the mode); the tests: the layout's
  `.git` in the four modes, as a file and as a link (`crates/core/tests/domain.rs`), `check` with a `.git` in a
  published domain still `ALIAS_INVALID` and, for P9G-23, with an `own-repository` record that does not replay
  (`reading.rs`), `check` in both modes of the fast-forward test (`run.rs`, its `if mode == "unpublished"` dropped);
  REQ-DOM-2 amended, and the 7c2b and 9g paragraphs, in `core-specification.md`;
  `.brewstillery/skills/verify-part/SKILL.md` (D-093: every check in a user scope of 2 GB with no debug information and two build jobs; the mutation run's scope at
  2 GB, `ulimit -v 1600000`, `--jobserver-tasks 2`, `-j 1 --timeout 400` or two jobs and the timeouts again alone;
  looking for scopes and idle `pasta` left by earlier runs, a container's `pasta` left alone; the watcher
  `target/watch-pasta.sh`, ending each idle `pasta` of its own scope whose cell's process is gone, until D-092; the
  part's diff written again against the working tree before an `--iterate` run when the code changed); `plan.md`;
  `STATUS.md`; this entry.
- The watcher was tried before the text was shown: three stand-ins named `pasta` (a copy of `tail` whose arguments
  name `/proc/<pid>/ns/net`), one in the scope whose process was gone, ended; one in the scope whose process lived,
  and one outside the scope, both left running.
- Codex, one narrow pass over `13fb764..e67df09`: accept, no blocking finding. P9G-23, not blocking: no test combined
  a `.git` with an `own-repository` record that does not replay (the mode then unknown, `.git` still a finding); the
  binary behaved so in Codex's own probe. Covered by a test (`77a9071`). Codex also found no other listing of the
  domain's top that takes `.git` for an alias (they filter by `is_alias`), and probed the three modes, a file, a link,
  a dangling link, a record absent or broken, a record with no mode and a mode changed later.
- Mutation over `13fb764..77a9071`, with `--features fabrewk-core/fault-injection`, `--test-workspace true`, as the
  amended skill says (2 GB, `ulimit -v 1600000`, `-j 1 --jobserver-tasks 2 --timeout 400`, the watcher): 7 mutants in
  8 minutes, 5 caught, 2 unviable, none missed, no timeout, no equivalent. The baseline cargo-mutants builds tests the
  mutated package alone (`fabrewk-core`, 30 s); each mutant was tested on the whole workspace.
- Clean clone of `77a9071`: 613 tests (603 passed, 10 ignored: the real tools' and a kept stream's), `crash` 3 and
  `faults` 37 green, the musl build, `budget check` (the core 11,278 of 12,000, the backend 278 of 400, Claude Code's
  adapter 269 of 600, Codex's 319 of 600, 5 direct runtime crates, 31 in the release build, the binary 3,150,216
  bytes), clippy with and without `fault-injection`, fmt and `cargo deny` clean; `--version --json` lists no feature.
  The steps ran in a user scope of 2 GB with two build jobs and the watcher; no idle `pasta` was left.
- Measured: the core +2; 722 lines remain for parts 9h and 9i.
- Part 9g is closed. Not done: the idle `pasta` a killed Fabrewk leaves (D-092, part 9i); P9G-22 stays declared
  (D-090).

## D-094 — Part 9h: an extension with network refused, its minutes a key, its requests applied only on exit 0, `config` lists each extension's path and digest

- Status: decided
- Decided: 2026-11-14
- Origin: maintainer XEL in session 38's chat, answering the integrator's four questions before any code: "Rehusar
  (Recomendado)", "Clave nueva (Recomendado)", "No; TOOL_FAILED (Recomendado)" and "Ruta y digest (Recomendado)".
- Decision:
  1. An extension with `extensions.<name>.network = true` cannot be confined by the bubblewrap backend, which cannot
     keep a cell with a network from nesting (part 9a): `fabrewk <name>` refuses `CONFINEMENT_UNAVAILABLE` before it
     runs, its words saying so, as a check with network does (D-085). An extension with network is left for after the
     pilot.
  2. A new key, `extensions.<name>.minutes` (integer, project level, default 30, a widening), limits an extension's
     run, as `checks.<name>.minutes` limits a check's; section 8 of `core-specification.md` is amended so.
  3. An extension that exits with a status other than 0 and 10 has none of its requests applied: `fabrewk <name>`
     refuses `TOOL_FAILED` (exit 1), naming its evidence; with 126 or 127, the statuses with which its cell's command
     fails to start, its words say that an interpreter or a support file may lie outside the system directories and
     the repository, which its cell does not show (S-9). Exit 10 applies nothing and exits 10 (REQ-CLI-3).
  4. `config` lists each extension the policy permits with its path and digest as found (`fabrewk.config/3`); no
     version, since knowing it would mean running the extension outside its cell (AD-5 point 1).
- Reason: the specification leaves the first open since part 9a (`extensions.<name>.network` "to the parts that build
  them"), the runner needs a time limit that no key gave, REQ-EXT-3 does not say whether a failing extension's requests
  apply, and AD-5 point 1 asks for what REQ-EXT does not. The alternatives offered: a cell with network that may nest,
  declared under T-08; a fixed limit of 60 minutes, or none; applying the valid requests and passing the extension's
  status through, as Git does; nothing in `config` until after the pilot.
- Consequences: `core-specification.md` (section 8 and a paragraph "Extensions, as part 9h fixed them"); the provenance
  entry of session 38.
- Sources: chat of session 38; `confinement-design.md` 3.4 and 3.7; `core-specification.md` REQ-EXT-1 to REQ-EXT-5,
  REQ-CLI-3, section 8; AD-5; D-085.
- Supersedes: none.

## R-047 — Session 38, part 9h: extensions

- Status: recorded
- Date: 2026-11-14
- Who: integrator Claude Code, `claude-opus-5-5`, effort high (D-078). Reviewer: Codex `codex-cli 0.166.0`,
  `gpt-6.1-sol`, effort high, launched as the `codex-review` skill says, in a scope of 2 GB.
- Start: HEAD `d6753ed`, tree clean.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md`, `core-specification.md` (REQ-EXT-1 to REQ-EXT-5, REQ-CLI-3, REQ-SEC-1,
  REQ-SEC-2, REQ-SEC-5, REQ-SEC-7, REQ-REG-1, REQ-REG-2, sections 3, 8, 9, 11.4, 12 and 13, the paragraphs of parts 9a
  and 9b), `confinement-design.md` (3.1 to 3.4, 3.7, 3.8, S-9, S-10, section 7), `architecture-decisions.md` AD-5, this
  file (D-085, D-091 to D-093, R-045, R-046); the skills `verify-part`, `codex-review` and `differential-review`; the
  code of `cli/` (`mod.rs`, `decl.rs`, `write.rs`, `propose.rs`, `config.rs`, `adopt.rs`, the run's dispatch of a
  check), `confine.rs`, `detect.rs`, `gate.rs`, `record/event.rs`, `task.rs`, `settings/`, `code.rs`, the backend
  `crates/confine-bwrap/src/lib.rs`, both catalogues, and the binary's test fixtures. No credential of the person's was
  read.
- Decided by the maestro in this session: D-094, before any code.
- Written: `crates/core/src/cli/ext.rs` (new: the routing of a name no core command has, permission and presence, the
  digest and its pin, the area and its lock, the cell, the run, the exit statuses, the requests and their application);
  `cli/mod.rs` (the route, exit 10); `cli/propose.rs` (its plan takes an actor and the identifiers already drawn, and
  gives its lines); `cli/config.rs` (each permitted extension's path and digest, `fabrewk.config/3`); `cli/decl.rs`
  (exit 10, the schema); `record/event.rs` (a proposal takes an extension as actor); `task.rs` (`extension-note`'s
  payload); `settings/keys.rs` (`extensions.<name>.minutes`); `detect.rs` (the search shared); `code.rs` and both
  catalogues (`EXTENSION_DIGEST`, `EXTENSION_OUTPUT_INVALID`, exit 10, the words of a run and of `config`); the tests:
  `crates/fabrewk/tests/extensions.rs` (new, 12 tests with real cells), two in `faults.rs` (the minutes, P9H-2), the
  record's and the table's, the pinned schema of `config`; the reference documentation regenerated; section 8 and the
  paragraph "Extensions, as part 9h fixed them" in `core-specification.md`; `plan.md`; `STATUS.md`; D-094 and this entry.
- Interpretations the maestro may want to read (each in the new paragraph):
  1. Every word after the extension's name is its own, `--json` included; what Fabrewk prints of an extension's run is
     human text only.
  2. A name outside the alias pattern, an absent program and an extension not permitted are each an unknown command's
     usage error; a present program whose permission cannot be read (a record that does not replay) is the record's
     refusal.
  3. What runs is a copy of the bytes whose digest was taken, shown read-only at the program's resolved path (a link in
     `PATH` is resolved), so a file changed between the digest and the run does not change what runs; the program's
     directory is not shown beside it.
  4. The area `local/ext/<name>/` and its home and `/tmp` are kept between runs, never emptied by Fabrewk.
  5. Its standard error reaches the person once it has ended, every hidden character escaped; not while it runs.
  6. A `propose` request's own refusals (`TASK_INVALID`, `STAGE_CLOSED`, …) keep their codes, the detail naming the
     line; nothing of the output is applied.
  7. A policy adopted while it ran that no longer permits it, or pins another digest, refuses its requests
     `EXTENSION_DIGEST`.
  8. A run that applies requests keeps the backend's programs as verified by use; a run with none writes nothing, so
     until a write command keeps them the probe runs at each run.
  9. Exit 126 and 127 are read as a start that failed (what `env` gives for an interpreter it cannot find); an
     extension that exits 127 itself is worded so too.
  10. Two runs of one extension never overlap (`LOCK_HELD`); runs of different extensions, and write commands, may run
      beside one, which takes the record's lock only to apply its requests.
  11. Inside its cell `status` shows `local/`'s sizes as 0, since `local/evidence/` and `local/tmp/` are hidden there.
- Codex, three passes, confirmed blocking findings by pass: 2, 1, 0.
  1. Pass 1 (`e7a2273`): P9H-1 (another run of the same extension, confined with write access to the area, swapping its
     home for a link between this run's check and its mount); P9H-2 (two `propose` requests of one output drawing
     identifiers apart, so a collision went unseen and left the record unreplayable).
  2. Pass 2 (`32f0f11`): both closed; P9H-3 (the requests planned on the record read before the extension ran, so a
     write meanwhile made them `RECORD_ROLLBACK` and skipped the check of the policy in force).
  3. Pass 3 (`e9cf866`): P9H-3 closed; accept.
  Each new test failed without its correction, checked by reverting the code under test. Codex's sandbox cannot run
  `bwrap` or `pasta` nor bind sockets; those tests ran on this machine, and Codex reproduced each finding with the
  production core and a substituted backend.
- Found and corrected by the integrator's own review before the first pass: a name outside the pattern searched in
  `PATH`; a mistyped command in a record that does not replay taken for the record's refusal; the program's bytes
  changeable between the digest and the run; the area made through a link before it was checked.
- Mutation, with `--features fabrewk-core/fault-injection`, `--test-workspace true`, as `verify-part` says (2 GB,
  `ulimit -v 1600000`, `-j 1 --jobserver-tasks 2 --timeout 400`, the watcher), over `77a9071..e9cf866`: 112 mutants in
  40 minutes, 82 caught, 30 unviable, none missed, no timeout, no equivalent. The run left in its scope one `bwrap` of
  an extension's cell (the test `ext-3`), its parent gone and its command waiting for the line that releases it; a
  mutated binary ended without ending it. Ended by the integrator; the extension's run uses the cell's one runner, so
  it is left for part 9i's crash suite, which kills Fabrewk during a run, to look at.
- Clean clone of `e9cf866`: 626 tests (616 passed, 10 ignored: the real tools' and a kept stream's), `crash` 3 and
  `faults` 39 green, the musl build, `budget check` (the core 11,607 of 12,000, the backend 278 of 400, Claude Code's
  adapter 269 of 600, Codex's 319 of 600, 5 direct runtime crates, 31 in the release build, the binary 3,215,752
  bytes), clippy with and without `fault-injection`, fmt and `cargo deny` clean; `--version --json` lists no feature.
  Every step in a user scope of 2 GB with two build jobs; two idle `pasta` left by full suites were ended.
- Measured: the core +329 (projected 300); the backend and both adapters unchanged. Left for part 9i: 393 lines.
- Not done: the flows F-1 to F-3, the crash suite over them, one real dispatch per provider, the idle `pasta` a
  killed Fabrewk leaves (D-092) and the phase's close (part 9i).

## D-095 — Part 9i: the real run's permission and budget

- Status: decided
- Decided: 2026-11-15
- Origin: maintainer XEL in session 39's chat, answering the integrator's question at the part's start: "Sí, hasta 2
  por proveedor (Recomendado)".
- Decision:
  1. The integrator may run, once, a whole `fabrewk run` through the built binary on a fixture repository of a few
     files under `target/`: the builder this machine's Claude Code, `claude-sonnet-5-5` at effort `low`; the reviewer
     its Codex, `gpt-6-luna` at effort `low`, another provider; each tool's preflight first (it calls no model); one
     round; ten minutes per dispatch; then `accept`, which calls no model. If something outside Fabrewk cuts the run
     (the provider, the network), one retry: at most two dispatches of each tool in all.
  2. Those dispatches use the person's own credentials, `~/.claude/.credentials.json` and `~/.codex/auth.json`, bound
     read-only as D-079 and D-081 fixed; Fabrewk reads of them only `expiresAt` and `last_refresh`, the integrator
     nothing of them. The fixture keeps its own private folder and Git configuration; only `HOME` is the person's.
- Reason: section 13 asks, for phase 9, one real dispatch per provider with its preflight first; parts 9c and 9d
  dispatched each tool through its adapter's own tests, never through `run`. The alternatives offered: one dispatch
  per provider with no retry; the adapters' real tests alone, as in 9c and 9d.
- Consequences: `crates/fabrewk/tests/real.rs` (ignored, run with `FABREWK_REAL_RUN`); the provenance entry of
  session 39.
- Sources: chat of session 39; D-079, D-081; `core-specification.md` section 13; `plan.md` row 9.
- Supersedes: none.

## R-048 — Session 39, part 9i: the whole, and phase 9's acceptance

- Status: recorded
- Date: 2026-11-15
- Who: integrator Claude Code, `claude-opus-5-5`, effort high (D-078). Reviewer: Codex `codex-cli 0.166.0`,
  `gpt-6.1-sol`, effort high, launched as the `codex-review` skill says, in a scope of 3 GB.
- Start: HEAD `c74ccb3`, tree clean.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md` (rows 9 and 10, "Phase 9"), `core-specification.md` (11.4, 11.5, 12, 13,
  REQ-RUN-5, REQ-SEC-1, REQ-SEC-5), `confinement-design.md` (3.5, 3.7, 3.10, 6), this file (D-070, D-071, D-077 to
  D-079, D-081, D-092 to D-094, R-047); the skills `verify-part`, `codex-review` and `differential-review`; the code of
  `confine.rs`, `cli/write.rs`, `cli/run.rs`, `candidate.rs`, `record/fault.rs`, `record/store.rs`, `record/git.rs`, the
  backend `crates/confine-bwrap/src/lib.rs`, the adapters' credential readers, and the tests' fixtures
  (`crates/fabrewk/tests/common/`, `run.rs`, `alambique.rs`, `fermento.rs`, `writing.rs`, `faults.rs`, the core's
  `crash.rs`, the backend's `cell.rs`), and `core/tests/confine.rs`. No credential of the person's was read.
- Decided by the maestro in this session: D-095, before any code; D-096, at the close.
- Written: `crates/confine-bwrap/src/lib.rs` (`beside`, written as soon as the helper exists; `idle`);
  `crates/core/src/confine.rs` (`Backend::idle`, `end_idle`, `settled`); `cli/write.rs` (every write command ends
  what a killed run left; the recovery removes a cell's directories only once nothing is left beside it);
  `candidate.rs` (four write points); both catalogues (`residual.network-helper`); the tests:
  `crates/fabrewk/tests/flows.rs` (new: F-1, F-2, F-3), `tests/common/flow.rs` (new: F-1's commands, shared),
  `tests/crash.rs` (new, feature `fault-injection`: the crash suite over F-1), `tests/real.rs` (new, ignored: the real
  run), `run.rs` (P9I-2's test, D-092 in `pd_08_…`), `extensions.rs` (`req_ext_4_…`'s socket), the core's
  `confine.rs` (`settled`'s wait), the backend's `cell.rs` (D-092, P9I-1, P9I-5, P9I-8);
  `core-specification.md` (the paragraph "The whole, as part 9i fixed it"); `confinement-design.md` (section 8,
  phase 9's acceptance); `plan.md`; `STATUS.md`; `.brewstillery/skills/verify-part/SKILL.md` (D-096);
  D-095, this entry and D-096.
- Interpretations the maestro may want to read (each in the new paragraph):
  1. Every write command ends the idle helpers, not only the recovery of a stopped run, so that a probe's or a
     preflight's helper, killed before `run-started`, is ended too and the residual's words hold. A refused write
     command does it as well; it is no write of the record.
  2. The helper is identified by the program it was started as, not its executable, which `pasta` makes unreadable;
     only a program named `pasta` is ever ended.
  3. The recovery waits up to ten seconds for a stopped run's cell to end, and keeps its directories when it has not.
  4. The crash suite's write points are the record's, the integration's and the candidate's; the export, a cell's own
     files and the evidence have none (P9I-4, accepted by Codex at its second pass).
  5. The next write command after each stop is a person's `discard` of a proposal that does not exist.
  6. The mutation run judges each mutant without the crash suite over F-1 (six minutes a mutant), which the diff's
     code does not reach but through the candidate's write points; the suite runs whole in the clean clone.
- Codex, three passes, confirmed blocking findings by pass: 4, 1, 0.
  1. Pass 1 (`efa94d3`): P9I-1 (the helper unnamed while it brings the network up, up to ten seconds); P9I-2 (the
     recovery removing the only name of a helper whose cell had not yet ended); P9I-3 (the crash suite taking the
     committed events after the stop, by name only); P9I-4 (no write point at the candidate's commit and branch).
     Not blocking: P9I-5 (a planted name ending a foreign process, outside the threat model, T-25), P9I-6 (an empty
     refusal list passing), P9I-7 (a trailer named twice passing).
  2. Pass 2 (`537bc96`): all seven closed; P9I-8 (a failure to write the name overwritten by the wait).
  3. Pass 3 (`17f1e97`), narrow on P9I-8's correction: closed; accept.
  The mutation run's commit (`95ab36f`) came after the acceptance and changes only tests.
  Each new test failed without its correction, checked by reverting the code under test. Codex's sandbox cannot run
  `bwrap` or `pasta`; it reproduced each finding with the production core and substituted programs, and ran the flows
  and the crash suite with a substituted backend (352 points).
- Found by the integrator before the first pass: `pasta` makes `/proc/<pid>/exe` unreadable to its owner, so the first
  version, comparing executables, never ended one; a helper dropped unended when its name failed.
- Mutation: 32 mutants over the part's diff, each judged without the crash suite over F-1: 23 caught, 9 unviable,
  none missed, none equivalent. The first whole run left two missed (`settled`'s deadline, its sign and its comparison)
  and seven caught only by the timeout (a test that waited on a helper with no limit); a test of `settled`'s ten seconds
  (`core/confine.rs::a_cell_that_ends_within_ten_seconds_is_settled_and_one_that_does_not_is_kept`) and a bound of ten
  seconds on that wait were added (`95ab36f`), and the nine run again were all caught by a failing test. Two runs before
  were void: under cargo-mutants' copy of the tree the socket path of `req_ext_4_…` passed `SUN_LEN`, so every mutant
  failed there first; its socket now lives under the temporary directory.
- Clean clone of `95ab36f`: 635 tests (624 passed, 11 ignored), the crash suite over F-1 (352 write points), `crash` 3
  and `faults` 39 green; the musl build, `budget check`, clippy with and without `fault-injection`, fmt and `cargo deny`
  clean; `--version --json` lists no feature. No `pasta` was left after the whole suite.
- The real run (D-095), once, through the built binary, with the person's `HOME` and nothing of their credentials read
  by the integrator: each adapter's preflight in both roles before any dispatch (Claude Code 2.1.316, `codex-cli`
  0.166.0); one dispatch each. The builder, Claude Code with `claude-sonnet-5-5` at effort low: success in 3 seconds,
  9,097 tokens in (8,986 cached) and 199 out, the model observed as asked. The reviewer, Codex with `gpt-6-luna` at
  effort low: success in 6 seconds, 19,520 tokens in (9,344 cached) and 158 out, verdict accept at round 1; its stream
  names no model, so the model observed is none, as 9d built it. The task was verified, accepted and integrated
  (`src/hello.txt` holding `hello`), and `check` passed clean. No retry was spent; two dispatches of the four allowed.
  The receipts are the task's events and the evidence under the fixture's `local/evidence/`, which the next test run
  may remove.
- Phase 9's acceptance: `confinement-design.md` section 8 holds, criterion by criterion, the tests that meet what
  section 13 assigns to phase 9; F-1 to F-3 and the crash suite are this part's.
- D-071's comparison (D-078 point 2 gave it for the opening; here it is for the phase): confirmed blocking findings
  per Codex pass. The opening on Fable 5.2, a design: 6, 1, 1, 0 (four passes), 23 % of its weekly allotment and 94 %
  of a five-hour window. The nine parts of code on Opus 5.5: 9a 3, 1, 0; 9b 3, 5, 3, 1, 0; 9c 3, 2, 1, 1, 1, 0; 9d 1,
  0; 9e 4, 2, 2, 2, 0; 9f 2, 0; 9g 4, 2, 2, 1, 3, 2, 1, 4, 3 and its closing 0; 9h 2, 1, 0; 9i 4, 1, 0: from two to ten
  passes, three at the median, as phase 7's (most parts at their second or third pass, the hard ones at their sixth to
  tenth). Fable's opening was rejected three times on the class of edge Opus's parts were, so no clear gain shows;
  Fable keeps no standing role (D-078), and phase 10 stays on the plan's models unless the maestro decides otherwise.
- Measured: the core 11,632 of 12,000 (+25; projected +50), the backend 318 of 400 (+40), Claude Code's adapter 269 and
  Codex's 319 of 600 (unchanged); five direct runtime crates, 31 in the release build; the musl binary 3,150,216 bytes.
  Phase 9 took the core from 7,630 to 11,632 (projected about 10,500, at most about 11,400).

## D-096 — Phase 9 closed; the watcher of idle `pasta` retired

- Status: decided
- Decided: 2026-11-15
- Origin: maintainer XEL in session 39's chat, answering the integrator's two questions after part 9i's checks: "Cerrar
  la fase 9 (Recomendado)" and, on the watcher `verify-part` runs beside every suite and mutation run (D-093),
  "Retirarlo (Recomendado)".
- Decision:
  1. Phase 9 (confinement, adapters and the run) is closed. Its parts 9a to 9i are done (R-039 to R-048), each with
     tests green in a clean clone, every viable mutant caught, within the size budget, and Codex's review accepted;
     `confinement-design.md` section 8 holds, criterion by criterion, what section 13 of `core-specification.md` assigns
     to phase 9, and the real run of D-095 dispatched once to each provider, its preflight first.
  2. Since D-092 is built, `verify-part` no longer runs `target/watch-pasta.sh`: the next write command ends an idle
     helper, and one found before or after a run is reported in the record. The skill's text was amended in this
     session (D-093's other rules for this machine stand).
- Consequences: `plan.md`, `STATUS.md`, `.brewstillery/skills/verify-part/SKILL.md`. Phase 10 (installation) is next,
  on the plan's models.
- Sources: chat of session 39; `plan.md` row 9 and "Phase 9"; D-075, D-076, D-092, D-093, D-095; R-039 to R-048.
- Supersedes: D-093 only for the watcher.

## D-097 — Phase 10 opened: its cut into four parts, the installer's ring, the core at 12,500, `apply`'s options, the product's files

- Status: decided
- Decided: 2026-11-15
- Origin: maintainer XEL in session 40's chat, answering the integrator's questions when phase 10 started: "Cuatro
  partes (Recomendado)"; "Mismas opciones (Recomendado)"; "install.sh, README, docs/ (Recomendado)"; on the ceiling,
  first "13,000 (Recomendado)" to a question that had the whole installation in the core, then, once the integrator
  said that AD-5 already puts the installer and the assessment in a ring of their own, "Anillo instalador, núcleo
  12,500 (Recomendado)", which replaces the first answer.
- Decision:
  1. Phase 10 is cut into four parts (D-019): 10a `assess` and the plan; 10b `apply`; 10c `onboard`, the first
     alambique's questions and the publication warning shown and recorded; 10d the installation script, the signed
     release with its third-party notices, the README and the manuals, and the phase's acceptance in a disposable
     repository. Each delivers code, so Codex (GPT-6.1 Sol), effort high, reviews each; Claude Code on Opus 5.5, effort
     high, builds them (plan row 10). Codex reviews the opening before any code, as phase 9's (D-075).
  2. The installation's code is a built-in module in its own ring, as AD-5 decided (D-036): `crates/install`, the
     ring `install` of kind `Installer`, at AD-5's ceiling of 1,200 counted lines, wired by the binary as the adapters
     are. The core keeps the record, the write path, the declaration of the commands and the codes; the module what
     `assess` reads, the verdict, the plan and its files, and `onboard`'s interview. The core's ceiling is 12,500
     counted lines (`tools/budget`), measured again at each part; it counted 11,632.
  3. `apply` takes the same options as `assess` besides the plan's digest and the person's words; the digest binds
     them, so an option that differs makes another plan (`PLAN_CHANGED`). `assess` prints the exact `apply` command its
     plan corresponds to.
  4. The product's files phase 10 creates at this repository's root: `install.sh`, `README.md` (English) and
     `README.es-MX.md`; the manuals in `docs/en/` and `docs/es-MX/` (D-014's languages).
- Reason: D-019 (inputs that separate: the plan's reading, its writing, the interview, the script and the release);
  AD-5 and REQ-BUD-1, which already count the installer as a ring; the size measured (368 lines left in the core) and
  projected (the core's contracts about 200 to 300, the module about 950); `assess` writes nothing, not even a cache,
  so the digest alone cannot say what was assessed; a GitHub repository's usual layout, the script fetched from its
  root. The alternatives offered: three parts (`assess` and `apply` together); the whole installation in the core at
  13,000, or the core kept at 12,000; a plan file given to `apply`; everything but the README under `docs/`.
- Consequences: `plan.md` (row 10's parts), `core-specification.md` (the installation as this opening fixes it, the
  new requirement REQ-INS-8 for D-066, section 1's row, REQ-BUD-2, section 5's synopses), `tools/budget/src/budget.rs`
  and its tests; the ring `install` is added to `RINGS` with its crate in part 10a.
- Sources: chat of session 40; `plan.md` row 10; AD-5, AD-7; D-006, D-014, D-019, D-036, D-066, D-075, D-086;
  `core-specification.md` REQ-INS-1 to REQ-INS-7, REQ-BUD-1, REQ-BUD-2, section 9 item 9, section 13.
- Supersedes: D-086 point 1 for the core's ceiling.

## D-098 — Phase 10's opening approved

- Status: decided
- Decided: 2026-11-15
- Origin: maintainer XEL in session 40's chat, after Codex accepted the opening at its fifth pass: "Apruebo el texto
  (Recomendado)"; on what follows, "Cerrar la sesión (Recomendado)".
- Decision: the amendments of phase 10's opening to `core-specification.md` are approved as Codex accepted them
  (commit `a394ea8`): REQ-INS-8 (the first alambique, D-066); the paragraph "Installation, as phase 10's opening fixed
  it (D-097)", points 1 to 11; section 1's rows, REQ-BUD-2, section 5's synopses, section 11.4's
  `INSTALLATION_INCOMPLETE` and `TERMINAL_REQUIRED`, section 12's REQ-INS-8 row and section 13's phase 10. Among them,
  shown to the maestro before the approval: the first project file declares no model and no profile, which a later
  `adopt` adds; an `AGENTS.md` or `CLAUDE.md` with changes not committed is a blocker (`none`); in `own-repository`
  and `unpublished` the entry files are left uncommitted in the product; the script supports Linux on ARM64 alone; the
  new code `INSTALLATION_INCOMPLETE`. Part 10a starts in the next session.
- Reason: the plan's rule for every phase; D-075's precedent (a phase's opening reviewed before any code, then
  approved).
- Consequences: `plan.md`, `STATUS.md`; R-049.
- Sources: chat of session 40; R-049; D-097.
- Supersedes: none.

## R-049 — Session 40, phase 10's opening

- Status: recorded
- Date: 2026-11-15
- Who: integrator Claude Code, `claude-opus-5-5`; reviewer Codex (`codex-cli 0.166.0`, `gpt-6.1-sol`, effort high,
  `codex exec -s workspace-write` in a memory-capped scope, from `target/review-10-opening/`, a disposable clone with
  no network, removed after).
- Read: `STATUS.md`; `plan.md`; `core-specification.md` sections 1, 2 (DOM, INS, SEC-6, BUD), 3, 4, 5, 9, 11.4, 11.5,
  12 and 13; `architecture-decisions.md` AD-5 and AD-7; `ALAMBIQUE.md` sections 1 and 2 and 6.17, 6.27, 6.45, 6.46;
  `xel/fermento.md` (no pending point); D-066, D-086, D-095, D-096, R-032, R-048; the code of `cli/write.rs`,
  `cli/adopt.rs`, `cli/decl.rs`, `cli/mod.rs`, `record/event.rs`, `record/store.rs`, `record/git.rs`, `identity.rs`,
  `settings/keys.rs`, `settings/policy.rs`, `adapter.rs`, `crates/fabrewk/src/main.rs`,
  `crates/fabrewk/tests/common/mod.rs`, `tools/budget/`.
- Found at the start: nothing of the installation exists (`assess`, `apply` and `onboard` are not declared; the
  project events are written by a test helper, `policy-adopted` before `publication-decided`); the core counted 11,632
  of 12,000. The integrator first asked the ceiling as if the installation were the core's; it then found AD-5's ring
  for the installer and the assessment (1,200, `RingKind::Installer`, declared in `tools/budget` and unused), said so,
  and asked again (D-097).
- Delivered: D-097 and D-098; the opening's amendments to `core-specification.md`; phase 10's parts in `plan.md`; the
  core's ceiling at 12,500 in `tools/budget/src/budget.rs` and its two tests (the suite green with the rustup
  toolchain; with Fedora's `cargo` three tests fail as `verify-part` says they do).
- Codex's passes, each over the commit named, confirmed blocking findings by count:
  1. `ad48c13`: 9 (P10O-1 to P10O-9): resuming an interrupted `apply` changed the plan; `installed` signed against
     section 3; the published commit of an entry file losing staged edits; the write path needing a record; events
     before the first adoption not verifiable; other writers released before the installation was whole; the first
     maestro's nested files not read; an entry file over 1 MiB with no plan; single quotes not preserving a backslash
     in fish (reproduced by the integrator with `fish --no-config` and `dash`). Corrected in `19f2d45`.
  2. `19f2d45`: 3 (P10O-10 to P10O-12), and P10O-13 not blocking: leftovers before the first event; `unpublished`
     complete before its views; a record commit stopped before the index update. Corrected in `83a9baa`.
  3. `83a9baa`: 3 (P10O-14 to P10O-16), all from the previous correction, which had put the person's index in the
     completion: `own-repository` never complete, a committed settings edit revoking completion, a staged edit lost.
     Corrected in `4ffd9d4`, completion becoming facts of the record, the views and the record commit.
  4. `4ffd9d4`: 2 (P10O-17, P10O-18): an index repair after the branch moved cannot tell a stale entry from a staged
     one. Corrected in `a394ea8`: the receipt's committed paths staged before the branch moves, only where the index
     carries no staged change, never touched after.
  5. `a394ea8`: 0, accept; every earlier finding confirmed closed; Codex ran 84 quoting checks in fish and `dash`.
  Each finding was checked against the code it cited before its correction; all were confirmed. No code was written,
  so no test or mutation run applied; the corrections are text.
- No credential of the person's was read.

## D-099 — Part 10a: `apply` declared now, answering a usage error until part 10b; `onboard` declared in part 10c

- Status: decided
- Decided: 2026-11-15
- Origin: maintainer XEL in session 41's chat, answering the integrator's question when part 10a started (`STATUS.md`
  left it to the part and the maestro): "apply ahora, onboard en 10c (Recomendado)".
- Decision: part 10a declares `assess` and `apply` with every option D-097 point 2 gives them, so that the command
  `assess` prints is one that exists, parses and passes the shells' checks (REQ-CLI-2, PD-38); until part 10b, `apply`
  answers with the catalogue's usage error `usage.unbuilt` (exit 2), reading and writing nothing, and its declaration
  lists no move and no refusal of its own. `onboard` is declared in part 10c, when it is built.
- Reason: `AGENTS.md` (never name a command that does not exist); the catalogue's rule that every printed command is
  one a test parses; help and the reference list only what answers. The alternatives offered: the three commands now,
  `apply` and `onboard` both answering "not built"; `assess` alone, leaving the printed command to part 10b.
- Consequences: `crates/core/src/cli/decl.rs`, `crates/core/src/cli/mod.rs`, the catalogue's `usage.unbuilt`, the
  generated help, completions and reference; `crates/fabrewk/tests/surface.rs`'s guard on later commands names
  `onboard`; part 10b gives `apply` its moves, its refusals and its document.
- Sources: chat of session 41; `STATUS.md` (next, part 10a); D-097 point 2; `plan.md` row 10a.
- Supersedes: none.

## D-100 — Part 10a: `assess` reads Git's state without `git status`; an include of the private folder stops it

- Status: decided
- Decided: 2026-11-16
- Origin: maintainer XEL in session 41's chat, after Codex's second pass of part 10a (P10A-8 to P10A-12), answering
  the integrator's two questions: "Sin git status (Recomendado)"; "Detener antes (Recomendado)".
- Decision:
  1. `assess` runs no `git status`, which runs filters, may fetch a partial clone's missing objects, reads tracked
     files of any size and trusts the index's flags. `AGENTS.md` and `CLAUDE.md` are compared by Fabrewk: the index
     and `HEAD` by object id, the working tree's bytes (at most 1 MiB) hashed by Git from standard input with the
     path's attributes and no filter, so that `assume-unchanged` and `skip-worktree` hide no edit. One that goes
     through a Git filter blocks with its own reason (`filtered`), since whether it holds changes cannot be told
     without running the filter. Whether other tracked files have changes, a reason that informs, is told from the
     index's staged changes against `HEAD` and from each tracked file's size and time against what the index
     recorded, and is worded "have, or may have". Every Git `assess` runs fetches no missing object
     (`GIT_NO_LAZY_FETCH`) and has every filter driver its configuration names turned off.
  2. Before any other Git, `assess` reads each of Git's configuration files apart (the repository's, the person's,
     the system's, and the `-c` values the environment carries), with a Git started outside any repository and with
     no configuration of its own, since Git follows includes as soon as it starts in a repository; an include that
     names a file of the private folder stops it with the new refusal `PRIVATE_FOLDER_INCLUDED`. An include inside an
     included file is a declared residual.
- Reason: REQ-INS-1 and point 3 of the opening's design (a closed list of what is read, no file over 1 MiB, nothing
  written, not the private folder); P10A-8 to P10A-12, each reproduced by the integrator. The alternatives offered:
  `git status` hardened (filters off, no fetch, no renames), reading point 3's limit as Fabrewk's own reads only and
  keeping `assume-unchanged` and a filtered entry file's false change as residuals; dropping the informing reason;
  for the include, a declared residual with no code.
- Consequences: `crates/install/src/lib.rs`, `crates/core/src/install.rs` (the environment and the standard input of
  `assess`'s Git), the code `PRIVATE_FOLDER_INCLUDED` (`crates/core/src/code.rs`, the catalogue, `assess`'s
  declaration), `core-specification.md` (the amendment after point 11, the code in section 7's list and its row in
  11.4).
- Sources: chat of session 41; Codex's second report of part 10a; D-097, D-098.
- Supersedes: none.

## D-101 — Part 10a: point 3's limit of 1 MiB is on what `assess` reads for its content

- Status: decided
- Decided: 2026-11-16
- Origin: maintainer XEL in session 41's chat, after Codex's third pass of part 10a (P10A-16), answering the
  integrator's question: "Solo lo que abre Fabrewk (Recomendado)".
- Decision: point 3's "a file over 1 MiB is reported by name and size and not read" applies to the files `assess`
  reads for their content (the entry files, the licence, the alambique, the other tools' settings, the key, the
  exclude file, the targets). What Git reads to answer about its own state, its index, objects, configuration and
  attributes, is the "Git's state" point 3 lists, and has no such limit. No code.
- Reason: Git reads its index and packs on every command, and they exceed 1 MiB in any repository of some size, so the
  wider reading cannot be met. The alternative offered: the same, and a refusal before any Git when one of the Git
  configuration files `assess` already lists, or the root's `.gitattributes`, is over 1 MiB.
- Consequences: `core-specification.md`, the amendment of part 10a; Codex's fourth pass reads P10A-16 so.
- Sources: chat of session 41; Codex's third report of part 10a; D-100.
- Supersedes: none.

## D-102 — Part 10a: every Git `assess` runs is isolated, with only the repository's own configuration

- Status: decided
- Decided: 2026-11-16
- Origin: maintainer XEL in session 41's chat, after Codex's fourth pass of part 10a (P10A-22 to P10A-29; confirmed
  blocking 5, 5, 4, 8 over the four passes), choosing among the integrator's alternatives: "Git aislado
  (Recomendado)".
- Decision: every Git `assess` runs has only the repository's own configuration: no global or system file
  (`GIT_CONFIG_GLOBAL=/dev/null`, `GIT_CONFIG_NOSYSTEM=1`), no `GIT_CONFIG_*` or `GIT_TRACE*` value of the environment,
  and `core.attributesFile=/dev/null`; the core's contract runs it so (`crates/core/src/install.rs`), and the checks of
  the main working tree and of the branch use it too. Before any Git in the repository, its configuration files are
  refused (`PRIVATE_FOLDER_INCLUDED`) when they lie in the private folder or include a file of it, and the person's
  global files when they lie there; from the global files, each read alone with no include followed, `assess` takes
  only `user.signingkey`, `gpg.format` and `core.autocrlf`, the repository's own value winning. Declared residuals: an
  include nested in the repository's configuration; a setting only an include of the global configuration holds, and
  a global attribute, which can make `assess` see a change that is not one, never hide one.
- Reason: the person's Git configuration offered ever new ways to write (a trace) or to open the private folder
  (P10A-22 to P10A-27); closing each did not converge. The alternatives offered: keep the checks and declare the rest
  residual; pause the part.
- Consequences: `crates/core/src/install.rs`, `crates/install/src/lib.rs` and its tests; `core-specification.md`, the
  amendment of part 10a; D-100 point 2's scan of the global, system and environment's includes is replaced.
- Sources: chat of session 41; Codex's fourth report of part 10a; D-100, D-101.
- Supersedes: D-100 point 2 for the global and system files and the environment.

## D-103 — Part 10a: what lies inside the Git directory is the person's own state, a declared residual

- Status: decided
- Decided: 2026-11-16
- Origin: maintainer XEL in session 41's chat, after Codex's seventh pass of part 10a (P10A-40 to P10A-44; confirmed
  blocking over the seven passes 5, 5, 4, 8, 3, 3, 4, two consecutive retries without progress, so the integrator
  gave alternatives as the plan's rule for every phase asks), choosing "Residuo del .git (Recomendado)".
- Decision: a link inside the repository's Git directory (its objects, alternates, index, hooks) that leads to the
  private folder is a declared residual: the person's own state, which a clone cannot plant. `assess` keeps refusing
  the `.git` entry, the Git directory, its configuration files, their includes and its `info/attributes` when they lie
  in the private folder, and the person's global files when they lie there; it walks no other part of the Git
  directory. Every file `assess` opens by itself is checked by where it really lies, link or not, and one in the
  private folder is reported as `private` and not read (P10A-42); a `--key` file there is `key-inside`. A key is one
  public key when `ssh-keygen -l` reads it as one from its standard input (P10A-44).
- Reason: the plan's rule for every phase (two retries without progress give the maestro alternatives); the
  remaining paths into the private folder were links the person alone can make inside their own Git directory. The
  alternatives offered: every Git of `assess` in a bubblewrap cell with the private folder hidden; pausing the part.
- Consequences: `crates/install/src/lib.rs` and its tests; `core-specification.md`, the amendment of part 10a; the
  catalogue's `key-inside`.
- Sources: chat of session 41; Codex's seventh report of part 10a; D-100 to D-102.
- Supersedes: none.

## D-104 — Part 10a: a narrow final pass of Codex

- Status: decided
- Decided: 2026-11-16
- Origin: maintainer XEL in session 41's chat, after Codex's eighteenth pass of part 10a (P10A-66 to P10A-68; two
  consecutive retries without progress, confirmed blocking 1, 1, 3 over the last three passes, so the integrator gave
  alternatives as the plan's rule for every phase asks), choosing "Pasada estrecha final (Recomendado)".
- Decision: the integrator corrects P10A-66 to P10A-68, and Codex's next pass is narrow, as D-084 and D-088 had it:
  it confirms each finding P10A-1 to P10A-68 closed, declared or decided, and looks for new findings only within the
  corrections of that last round (`git diff 6d36c4a..HEAD`). On its `accept` the part closes with the clean clone,
  the mutation run and the record.
- Reason: the plan's rule for every phase; eighteen passes in which each full pass found the next edge case of the
  same reading (Git's state and the file names it gives), every one corrected. The alternatives offered: full passes
  until an accept; pausing the part.
- Consequences: Codex's nineteenth prompt; R-050.
- Sources: chat of session 41; Codex's eighteenth report of part 10a; D-084, D-088.
- Supersedes: none.

## R-050 — Session 41, part 10a: `assess` and its plan (paused before Codex's narrow final pass)

- Status: recorded
- Date: 2026-11-16
- Who: integrator Claude Code, `claude-opus-5-5`; reviewer Codex (`codex-cli 0.166.0`, `gpt-6.1-sol`, effort high,
  `codex exec -s workspace-write` in a memory-capped scope, from `target/review-10a/`, a disposable clone with no
  network), which also drafted the publication warning's text (from `target/review-10a-warning/`, removed after).
- Read: `STATUS.md`; `plan.md`; `core-specification.md` REQ-INS-1 to REQ-INS-8, REQ-DOM-1 to REQ-DOM-7, REQ-REG-12,
  section 5, 11.2, 11.4, 12 (INS) and "Installation, as phase 10's opening fixed it" points 1 to 11;
  `architecture-decisions.md` AD-7; `ALAMBIQUE.md` 6.27 and section 2's line on publishing `.brewstillery/`;
  `xel/fermento.md` (no pending point); `settings.toml` (project and `xel`'s); D-097, D-098, R-049; the skills
  `verify-part`, `codex-review`, `differential-review` and `sharp-edges`; the code of `cli/mod.rs`, `cli/decl.rs`,
  `cli/text.rs`, `cli/ext.rs`, `code.rs`, `adapter.rs`, `identity.rs`, `machine.rs`, `fermento.rs`, `alambique.rs`,
  `settings/mod.rs`, `settings/lang.rs`, `record/mod.rs`, `record/git.rs`, `record/canon.rs`, `record/replay.rs`,
  `crates/fabrewk/src/main.rs`, `crates/fabrewk/tests/common/mod.rs`, `crates/fabrewk/tests/surface.rs`,
  `crates/adapter-claude/src/lib.rs`, `tools/budget/`, and both catalogues.
- Delivered: the ring `install` (`crates/install`, kind `Installer`, added to `RINGS` as D-097 decided) and the core's
  contract for it (`crates/core/src/install.rs`: `Asked`, `Installer`, and the installer's isolated Git);
  `fabrewk assess`, declared with D-097 point 2's options, its language and tools checked by the core, its reading,
  verdict, plan (`fabrewk.plan/1`) with every file's action and exact content, the first alambique (REQ-INS-8), its
  digest over the canonical bytes, and the `apply` command for fish and a POSIX shell (`fabrewk.assessment/1`);
  `fabrewk apply`, declared and answering `usage.unbuilt` until part 10b (D-099); the publication warning's text,
  drafted by Codex and kept as drafted in both catalogues (the maestro decides the final text in part 10c); the new
  code `PRIVATE_FOLDER_INCLUDED`; the specification's amendment "Amended in part 10a (D-100)" after point 11, the
  code in section 7 and its row in 11.4; D-099 to D-104. Tests: `crates/fabrewk/tests/install.rs` (51 tests),
  `crates/install/tests/install.rs` (the contract without the binary), unit tests of the side effects and the files.
- Measured: the core 11,796 of 12,500 at `bacf4df` (+164 from 11,632); the ring `install` 972 of 1,200, so 228 lines remain for
  parts 10b and 10c, against the opening's projection of about 950 for the whole module: a figure the maestro may
  want to weigh before part 10b (no change made; a change of the ceiling is the maestro's, `AGENTS.md` "Code").
  Release binary 3,543,432 bytes; direct runtime crates 5; crates in the release build 31.
- Codex's passes, each over the commit named, confirmed blocking findings by count (non-blocking in brackets):
  1. `4a6ae1c`: 5 [2] (P10A-1 to P10A-7): Git's filters run by `git status`; a `data/` link walked; the file Git's
     signing key names opened; the record replayed before `assess`; a tag in capitals reaching `en`.
  2. `80c567f`: 5 [1] (P10A-8 to P10A-13): a partial clone's lazy fetch; an include of the private folder; tracked
     files over 1 MiB read; a filtered entry file a false change; `assume-unchanged` hiding an edit. The maestro
     decided D-100 (no `git status`; an include of the private folder refused).
  3. `5198c07`: 4 [4] (P10A-14 to P10A-21): the environment's `-c` values; `core.attributesFile`; Git's own reads
     over 1 MiB (D-101); an entry file's mode change.
  4. `37d67a6`: 8 [1] (P10A-22 to P10A-30): the person's Git configuration writing a trace or opening the private
     folder by ever new ways. The maestro chose among alternatives D-102 (every Git of `assess` isolated).
  5. `90295e5`: 3 [1] (P10A-31 to P10A-34). 6. `5586031`: 3 [2] (P10A-35 to P10A-39). 7. `8d80546`: 4 [1]
     (P10A-40 to P10A-44); two retries without progress, the maestro chose D-103 (what lies inside the Git directory
     is the person's own state, a declared residual).
  8. `5edf9c2`: 2 (P10A-45, P10A-46). 9. `6187af2`: 3 (P10A-47 to P10A-49; Git given its directory and working tree).
  10. `187a261`: 2 (P10A-50, P10A-51). 11. `0dc6280`: 3 [1] (P10A-52 to P10A-55). 12. `fd20a6d`: 2 (P10A-56,
  P10A-57). 13. `4d992f7`: 2 (P10A-58, P10A-59). 14. `53f6a9b`: 1 (P10A-60). 15. `2810b2e`: 2 (P10A-59's rest,
  P10A-61). 16. `e55a069` (run again after the machine restarted; the cut run was not counted): 1 [1] (P10A-62,
  P10A-63). 17. `63c598e`: 1 [1] (P10A-64, P10A-65). 18. `6d36c4a`: 3 (P10A-66 to P10A-68); two retries without
  progress, the maestro chose D-104 (a narrow final pass).
  19. `bacf4df`: not run: Codex's usage limit, until 2026-11-18 10:29. The part is paused there (the plan's rule for
  an exhausted budget), on the maestro's choice to pause after the clean clone and the mutation.
  Every finding was checked against the code before its correction; all were confirmed; each correction has a test
  that failed on the earlier code, but P10A-7's (an interpretation's wording) and P10A-18's.
- Declared, with the maestro's decisions: P10A-30 (the core's `NOT_A_REPOSITORY` and `ALIAS_INVALID` name no command
  as their next step); an entry file turned off from filters (`-filter`) blocks as `filtered` (P10A-36's correction);
  the residuals of D-100 to D-103.
- Interpretations the maestro may want to read: the plan's answers are the options as given, `--mode` and `--lang`
  defaulted, the tools sorted; the key recorded as the option it is, by its digest, only when `ssh-keygen -l` reads it
  as one public key; a plan whose verdict is `none` or `review` holds no file and no `apply` command, an incomplete one
  its files as far as its answers go; each value of the printed command written `--option='value'`; `--lang` must reach
  a catalogue by its primary subtag; names that are not UTF-8 are shown with `\xNN`, and the first alambique's title
  writes a control character as `\u{…}`; side effects inform and never block; the views and, in `own-repository`,
  `.brewstillery/.git` are targets to create.
- Verified (`verify-part`), clean clone of `bacf4df`: 680 tests; the crash suite over F-1, `faults` 39 and `crash` 3
  green; musl release build, `budget check`, both clippy runs, fmt and `cargo deny` clean; `--version --json` lists no
  feature.
- Mutation (`cargo mutants` over `git diff 768b9a2`, in a 2 GB scope, `-j 2`): the installer's 431 mutants (61
  unviable), first with the binary's `install` suite alone, which needed the ring a test target of its own
  (`crates/install/tests/install.rs`, `c90572e`): 311 caught, 59 missed; with the ring's unit tests too, 44 missed. The
  tests they asked for (`4550357`): unit tests of `kind`, `Reading::note`, `data`, `render` and of three side effects;
  integration tests of a tree with no change (a submodule's entry among the files, a staged change, a repository with
  no commit), the 1 MiB bound for a root file and for the `--key` file, an include of a private folder not yet made,
  the plan's items each in the case that tells it, and an entry file through a filter only in the tree or only in the
  index. Then 369 caught and one equivalent: `files.rs` `b < e` as `b <= e`, since no line is both markers. The core
  and the binary, 61 mutants: 55 caught, 5 unviable, and one missed, the extension's inner context's `writes: true`,
  which nothing reads (only the command's own context words an error): the field was dropped (`4550357`), the core
  then 11,795 lines (+163).
- Verified again, clean clone of `4550357`: 691 tests; the crash suite over F-1, `faults` 39 and `crash` 3 green; musl
  release build, `budget check` (core 11,795 of 12,500; install 972 of 1,200; binary 3,543,432 bytes; 5 direct
  runtime crates; 31 in the release build), both clippy runs, fmt and `cargo deny` clean; `--version --json` lists no
  feature. Codex's narrow pass is to cover `4550357` too: its prompt's scope is `6d36c4a..HEAD`, its clone brought to
  the closing commit first.
- No credential of the person's was read; every assessment ran in fixtures with their own home and private folder.

## R-051 — Session 42, part 10a's closing: Codex's narrow final passes

- Status: recorded
- Date: 2026-11-19
- Who: integrator Claude Code, `claude-opus-5-5`; reviewer Codex (`codex-cli 0.166.0`, `gpt-6.1-sol`, effort high,
  `codex exec -s workspace-write` in a scope of 3 GB, from `target/review-10a/`, its clone brought to each commit
  reviewed, with no network).
- Start: HEAD `32b0002`, tree clean.
- Read: `STATUS.md`, `AGENTS.md`, this file (D-104, R-050), `plan.md` (row 10, "Phase 10"); the skills `codex-review`
  and `verify-part`; Codex's nineteenth prompt; `crates/install/src/lib.rs` (`Reading` and its callers) and
  `crates/fabrewk/tests/install.rs` (P10A-66's test and the fixture). No credential of the person's was read.
- Codex's passes, narrow as D-104 fixes them:
  19. `32b0002` (the earlier attempt, cut by the usage limit, kept as `pass-19-cut` and not counted): reject, 1
      confirmed blocking (P10A-69). P10A-1 to P10A-68 each confirmed closed, declared or decided, with Codex's own
      reproductions. P10A-69: with `HOME` the repository's root, or its Git directory, a link of the repository (a
      `LICENSE` leading outside it; `info/exclude`) was read and its digest put in the plan, since P10A-66's
      correction told a home source by its base being the home. Checked against the code before its correction:
      `Reading::read` compared `base` with the home. Corrected (`7d5839c`): each caller says whether its source is
      the home's, and only the home's settings sources say so; the field `Reading::home` dropped. Its test,
      `p10a_69_a_home_that_is_the_repository_makes_none_of_its_links_read`, failed on `32b0002` (the digest of the
      link's target in the input) and passes.
  20. `7d5839c`, over `32b0002..7d5839c`: accept, no finding. P10A-69 closed in both of its cases; Codex probed 27
      scenarios: both home settings sources through a link to a file and to a directory, a private, an oversized and
      a broken target, a path that is both a repository and a home source, the digest repeated, the fixtures unchanged.
- Mutation over `32b0002..7d5839c` (`cargo mutants --package fabrewk-install --test-package fabrewk-install
  --test-package fabrewk`, in a 2 GB scope, `ulimit -v 1600000`, `-j 1 --jobserver-tasks 2 --timeout 400`): 9 mutants,
  9 caught by the binary's `install` suite, none missed, no timeout, no new equivalent. The part's one declared
  equivalent stays R-050's (`files.rs` `b < e` as `b <= e`).
- Verified (`verify-part`), clean clone of `7d5839c`, every step in a user scope of 2 GB with two build jobs: 692
  tests passed, 11 ignored; the crash suite over F-1 (1), `faults` 39 and `crash` 3 green; musl release build,
  `budget check` (the core 11,795 of 12,500; the installer 971 of 1,200; the backend 318 of 400; Claude Code's adapter
  269 of 600; Codex's 319 of 600; 5 direct runtime crates; 31 in the release build; the binary 3,543,432 bytes), both
  clippy runs, fmt and `cargo deny` clean; `--version --json` lists no feature. No idle `pasta` before or after.
- Measured: the installer −1 line; 229 remain for parts 10b and 10c, R-050's figure for the maestro to weigh still
  standing.
- Part 10a is closed: REQ-INS-1's and REQ-INS-2's criteria, the three verdicts, `NOT_A_REPOSITORY`, `ALIAS_INVALID`,
  the first alambique's text in the plan and the `apply` command for fish and a POSIX shell, by the tests of R-050
  and this entry. Twenty passes, confirmed blocking 5, 5, 4, 8, 3, 3, 4, 2, 3, 2, 3, 2, 2, 1, 2, 1, 1, 3, 1, 0. Every
  finding was checked against the code before its correction, and all were confirmed. The review directory, the
  clean clone and the mutation's output were removed after this entry.

## D-105 — Part 10b: the ceilings kept, `apply`'s writing in the core, measured at the part's end

- Status: decided
- Decided: 2026-11-19
- Origin: maintainer XEL in session 43's chat, answering the integrator's question when part 10b started (R-050 and
  R-051 left the installer's figure for the maestro to weigh before 10b): "Techos igual, medir (Recomendado)".
- Decision: no ceiling changes. `apply`'s execution (the reassessment's binding, the events, the plan's files, the
  entry blocks, the exclude line, the domain's repository, the record commit by mode, the interrupted `apply` and
  `INSTALLATION_INCOMPLETE`) is the core's, through the write path, since the plan already holds every file's exact
  content; the installer only reassesses, and keeps its 229 lines for part 10c's interview. If the core passes 12,500,
  the integrator stops and brings the measured figure before going on. The interpretations of the part are the
  specification's "Amended in part 10b (D-105)".
- Reason: D-097 point 1 (the core keeps the record and the write path; the module the plan and its contents); AD-5's
  rings; the measure (the core 11,795 of 12,500, the installer 971 of 1,200, R-051). The alternatives offered: the
  installer's ceiling at 1,600 with the plan's files written there; the core's at 13,000.
- Consequences: `crates/core/src/cli/apply.rs` (new), `cli/write.rs`, `cli/check.rs`, `cli/decl.rs`, `cli/mod.rs`,
  `code.rs`, `identity.rs`, `table.rs`, `record/event.rs`, `record/git.rs`, `record/store.rs`, both catalogues;
  `core-specification.md` (section 6's C2, section 7's list, the amendment after point 11); the tests.
- Sources: chat of session 43; R-050, R-051; D-097.
- Supersedes: none.

## D-106 — Part 10b: Codex's review reads only, after three passes its provider stopped

- Status: decided
- Decided: 2026-11-19
- Origin: maintainer XEL in session 43's chat, choosing among the integrator's alternatives after three consecutive
  passes of Codex were stopped by its provider before a report ("This content was flagged for possible cybersecurity
  risk"; the third also "Selected model is at capacity"): "Pasada solo de lectura (Recomendado)".
- Decision: Codex's passes of part 10b, on the same model and effort (GPT-6.1 Sol, high), read the diff, the code, the
  tests and the specification and run nothing: no suite, no binary, no script. The integrator gives it the suites'
  results on the commit it reviews, and reproduces each finding before its correction, as every pass. The leads the
  three stopped passes left before their cut, P10B-1 to P10B-3, were each reproduced by a test that failed before its
  correction; the stopped passes are not counted.
- Reason: the plan's rule for every phase (two retries without progress give the maestro alternatives); the third
  pass was stopped while reading the code and running the suites, whose output is full of the domain's words
  (credentials, keys, confinement). The alternatives offered: the same full prompt later; another Codex model;
  pausing the part.
- Consequences: Codex's fourth prompt of part 10b and later ones; R-052.
- Sources: chat of session 43; `target/review-10b/pass-1.jsonl` to `pass-3.jsonl` (removed after the part).
- Supersedes: none.

## R-052 — Session 43, part 10b: `apply`

- Status: recorded
- Date: 2026-11-19 to 2026-11-20
- Who: integrator Claude Code, `claude-opus-5-5`; reviewer Codex (`codex-cli 0.166.0`, `gpt-6.1-sol`, effort high,
  `codex exec -s workspace-write` in a scope of 3 GB, from `target/review-10b/`, a disposable clone with no network,
  brought to each commit reviewed).
- Start: HEAD `fb8eb01`, tree clean.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md` (row 10, "Phase 10"), `core-specification.md` (DOM, INS, section 3,
  section 4 "Project", section 6, section 7, 11.2, 11.4, 11.5, 12, and "Installation, as phase 10's opening fixed it"
  with the amendment of part 10a), D-097 to D-104, R-049 to R-051; the skills `verify-part`, `codex-review`,
  `differential-review` and `sharp-edges`; the code of `cli/mod.rs`, `cli/write.rs`, `cli/adopt.rs`, `cli/check.rs`,
  `cli/decl.rs`, `install.rs`, `identity.rs`, `table.rs`, `domain.rs`, `alambique.rs`, `fermento.rs`, `view.rs`,
  `record/event.rs`, `record/replay.rs`, `record/store.rs`, `record/git.rs`, `crates/install/src/lib.rs` and
  `files.rs`, and the tests' fixtures (`crates/fabrewk/tests/common/`, `crates/core/tests/common/`), `flows.rs`,
  `crash.rs`. No credential of the person's was read.
- Decided: D-105 (the ceilings kept, `apply`'s writing in the core) and D-106 (Codex's review reads only).
- Delivered: `fabrewk apply` (`crates/core/src/cli/apply.rs`, new; its declaration with five moves, its refusals and
  `fabrewk.applied/1`): AGENT_ENVIRONMENT first; the reassessment bound by the digest (`PLAN_CHANGED`, which names the
  digest `assess` gives now), `VERDICT_NONE`, `MODE_NOT_AVAILABLE`; the project file the plan creates
  (`SETTINGS_INVALID`), Git's key the plan's (`ROLE_FORBIDDEN`), the words; under the lock, the targets as found or as
  written; `assessed` and `publication-decided`, the plan's files, the entry blocks, the domain's repository and the
  exclude line, `policy-adopted`, `alambique-baseline`, `installed` with the receipt, the views and the record commit
  by mode, the receipt's committed paths (and the alambique kept) staged before the branch moves where the index
  carries no staged change; an interrupted `apply` finished by the same command from the plan its `assessed` holds,
  with the same options; `INSTALLATION_INCOMPLETE` (new) for every other write command and in `check` (C2); the
  events before the first adoption verified against the plan's project file; the `project` stream's order and its
  bindings to the plan on replay; `assessed`'s and `installed`'s payloads; the new codes `PLAN_CHANGED`,
  `VERDICT_NONE`, `MODE_NOT_AVAILABLE`, `INSTALLATION_INCOMPLETE`. F-1 and F-2 start from a real `apply` (the
  fixture `Fx::applied`); the crash suite runs over `apply` in the three modes and the same `apply` finishes each stop;
  `crates/fabrewk/tests/apply.rs` (new). The record's tests' generic Fabrewk event moved to an extension's note.
  `core-specification.md`: section 6's C2, section 7's list, "Amended in part 10b (D-105)" after point 11.
- Codex's passes, confirmed blocking findings by pass counted from the fourth (D-106): 4, 1, 1, 0.
  1. Passes 1 to 3 (`b83ddb9`, `05ef225`, `db6a48f`), full prompts, each stopped by Codex's provider before a report
     ("This content was flagged for possible cybersecurity risk"; the third also "Selected model is at capacity"); not
     counted. Their leads, each reproduced by a test that failed before its correction: P10B-1 (`apply`'s temporary
     file beside an entry file or the exclude file overwrote a file of the person's at that name; now in
     `local/tmp/`, `05ef225`); P10B-2 (a view made by someone between the reassessment and the lock was overwritten;
     a view present is a target only when it is what the record as it stands gives, path by path, `05ef225`); P10B-3
     (an entry file's permissions, given after the rename, lost at a stop between the two; now given to the temporary
     file before it, `db6a48f`).
  2. Pass 4 (`db6a48f`, reads only, D-106): reject, 4 confirmed blocking (P10B-4, P10B-7, P10B-8, P10B-9), corrected
     in `71aa61c`. P10B-4: `local/tmp/` made a link led a write command's temporary files and its removal of
     leftovers outside the domain; the session refuses it (`RECORD_BROKEN`) and `apply` checks it with every target.
     P10B-7: a resumed `apply` took other options than its plan's silently; it compares them with the plan's answers
     (`PLAN_CHANGED`), through the installer's new `answers`. P10B-8: once an adoption existed, the opening events were
     verified against it, not against the plan; the opening is bound to the plan on replay (`table.rs`: the mode, the
     adopted project file and the receipt's plan). P10B-9: a receipt naming another plan replayed; refused
     (`RECORD_TRANSITION_INVALID`). Not confirmed: P10B-5 (an exclude file named absolutely, outside the working tree)
     and P10B-6 (a Git directory on another file system): a repository whose Git directory lies apart from its working
     tree is refused by the main working tree's check (`NOT_A_REPOSITORY`) before any write, as for every write
     command since part 7b; a test fixes it, and the specification declares it a limitation. P10B-10, not blocking (the
     project stream's order had no test that required its refusal): the replay test of the opening's order and
     bindings.
  3. Pass 5 (`71aa61c`): reject, 1 confirmed blocking, P10B-11: a link made at `local/tmp/` between the start and the
     lock went past the check before the lock; the session checks it once the lock is held, before it removes any
     leftover (`5ecac75`).
  4. Pass 6 (`5ecac75`): reject, 1 confirmed blocking, P10B-12: with the check before the lock removed, the ending of
     idle cells walked a linked `local/tmp/` and removed a file of the person's outside the domain before the session
     refused; `end_idle` walks no linked `local/tmp/` (`78c06e5`).
  5. Pass 7 (`78c06e5`): accept, no finding; P10B-1 to P10B-12 closed or declared.
  Every finding was checked against the code before its correction; each new test failed without it. The integrator
  gave each read-only pass the suites' results on the commit it reviewed.
- Mutation over `fb8eb01..HEAD` (`cargo mutants --package fabrewk-core --package fabrewk --package fabrewk-install
  --test-workspace true --baseline skip --features fabrewk-core/fault-injection --in-diff`, in a 2 GB scope with
  `ulimit -v 1600000`; the baseline skipped, since cargo-mutants builds it for the mutated packages alone and the
  selection of tests below names the binary's; that selection was run green by hand on the tree first):
  1. Over `78c06e5`, `-j 2 --timeout 300`: 193 mutants, 139 caught, 30 unviable, 23 missed, 1 timeout. The missed:
     the resumed `apply`'s target checks (`targets`: a file made, changed or made unreadable after the stop, the
     exclude file without the line, a file where the domain's repository goes), the binding of a resumed `apply` to
     its digest, a record that does not replay taken for none, a file written again at resumption, `left` in
     `published`, the completion read from a person's own commit, `assessed`'s and `installed`'s payloads, a forged
     stream beside the project's before the first adoption, the index staging of an entry file the person changed,
     two guards that could be written without the mutated operator, and `end_idle`, caught by
     `run.rs::d_092_a_cell_still_ending_keeps_its_directories_until_its_helper_is_ended`, outside the selection
     (checked by hand, no `pasta` left). Corrected in `6337762`: the tests named in its message, the link check
     first in `targets`, and `write::pending`, so that `apply` matches no refusal's code (a Git failure under the lock
     now refuses instead of going on).
  2. Over `6337762`, `--iterate -j 2 --timeout 400`, `run` added to the selection: 104 mutants, 87 caught, 15
     unviable, 1 missed, 1 timeout. The missed: `apply`'s default for the words' language, which
     `languages.agentInteraction` already gives (from the plan's project file, its canonical language, and from the
     maestro's own file); the default dropped (`847e3e7`), so no equivalent is declared.
  3. Over `847e3e7`, `--iterate -j 1 --timeout 400`: 63 mutants, 45 caught, 17 unviable, 1 timeout, none missed. The
     timeout, `write.rs` `held` as `Ok(None)`, timed out alone too: the `faults` tests that pause a command at a write
     point wait for a pause the mutant never reaches, a loop caught by the timeout.
  None missed, none equivalent.
- Verified (`verify-part`), clean clone of `e2eecc7`, every step in a user scope of 2 GB with two build jobs: 707
  tests passed, 11 ignored; with `fault-injection`, the binary's `crash` 2 (the crash suite over F-1 and over
  `apply`), `faults` 47 and the core's `crash` 3 green; the musl release build, `budget check` (the core 12,255 of
  12,500; the installer 971 of 1,200; the backend 318 of 400; Claude Code's adapter 269 of 600; Codex's 319 of 600;
  5 direct runtime crates; 31 in the release build; the binary 3,674,504 bytes), both clippy runs, fmt and `cargo
  deny` clean; `--version --json` lists no feature. No idle `pasta` before or after.
- Measured: the core 12,255 of 12,500 (+460 over the part; 245 lines left for parts 10c and 10d); the installer 971
  of 1,200, unchanged (D-105), its 229 lines left for part 10c's interview.
- Interpretations the maestro may want to read (each in the specification's "Amended in part 10b (D-105)"):
  1. A resumed `apply` writes only what is missing: a file already as the plan writes it is not written again.
  2. The completion of an `apply` is read from the record, the views and the record branch: a person's own commit of
     what a stopped `apply` left does not complete it.
  3. In `published`, a change the person staged in an entry file after a stop is kept in the index, and the commit
     holds the plan's content: the index then shows the person's change against the new tip.
  4. A repository whose Git directory lies apart from its working tree is refused by `apply` (`NOT_A_REPOSITORY`),
     although `assess` gives it a plan: a declared limitation.
  5. The mutation run judged each mutant with the suites the diff reaches (the core's unit tests, `record`,
     `identity`, `domain`, `settings`, `apply`, `faults`, the crash suite over `apply`, `writing`, `install`, `flows`,
     `tasks`, and from the second run `run`), without the crash suite over F-1 (about 390 s a mutant), as R-048
     interpretation 6; the clean clone runs everything.
- Part 10b is closed: REQ-INS-3's, REQ-INS-8's (the baseline), REQ-DOM-1's, REQ-DOM-4's, REQ-DOM-6's and REQ-DOM-7's
  criteria at `apply`, the refusals of the row and the crash suite over `apply`, by `crates/fabrewk/tests/apply.rs`,
  `faults.rs`, `crash.rs` and the core's `record.rs` and `identity.rs`. The review directory, the clean clone and the
  mutation's output are removed after this entry.

## D-107 — Part 10c: the publication warning's final text is Codex's draft

- Status: decided
- Decided: 2026-11-20
- Origin: maintainer XEL in session 44's chat, answering the integrator's question when part 10c started (plan row
  10c: the maestro's decision on the warning's final text), shown the Spanish text in full: "Borrador como final
  (Recomendado)".
- Decision: the text of the catalogue's `[warning]` table, drafted by Codex in part 10a (R-050), is final in `en` and
  `es`, unchanged. The interpretations of part 10c are the specification's "Amended in part 10c (D-107)".
- Reason: D-097 point 9 and plan row 10 (the maestro decides the warning's final text). The alternatives offered: the
  maestro's own text; a second draft by Codex with the interview's questions.
- Consequences: `crates/core/catalogue/en.toml` and `es.toml` (`[warning]`'s description); `core-specification.md`
  (the amendment after point 11).
- Sources: chat of session 44; R-050; D-097 point 9.
- Supersedes: none.

## R-053 — Session 44, part 10c: `onboard`

- Status: recorded
- Date: 2026-11-20
- Who: integrator Claude Code, `claude-opus-5-5`; Codex (`codex-cli 0.166.0`, `gpt-6.1-sol`, effort high, `codex exec
  -s workspace-write` in a scope of 3 GB, with no network), which wrote the interview's questions (from
  `target/review-10c-texts/`, reading only, removed after) and reviewed the part (from `target/review-10c/`, a
  disposable clone brought to each commit reviewed).
- Start: HEAD `4cf6913`, tree clean.
- Read: `STATUS.md`, `AGENTS.md`, `plan.md` (row 10, "Phase 10"), `core-specification.md` (REQ-INS-1 to REQ-INS-8,
  section 5's commands table, section 7's `TERMINAL_REQUIRED`, 11.4, 12, and "Installation, as phase 10's opening
  fixed it" points 1 to 11 with the amendments of parts 10a and 10b), `architecture-decisions.md` AD-7, D-099, D-105,
  D-106, R-050, R-052; `xel/fermento.md` (no pending point); the skills `verify-part`, `codex-review`,
  `differential-review` and `sharp-edges`; the code of `cli/mod.rs`, `cli/decl.rs`, `cli/apply.rs`, `cli/write.rs`,
  `cli/text.rs`, `code.rs`, `install.rs`, `settings/mod.rs`, `record/event.rs`, `crates/install/src/lib.rs`,
  `crates/fabrewk/src/main.rs`, both catalogues, and the tests' fixtures (`crates/fabrewk/tests/common/`), `apply.rs`
  and `surface.rs`; `tools/budget/src/budget.rs`. No credential of the person's was read.
- Decided: D-107 (the publication warning's final text is Codex's draft).
- Delivered: `fabrewk onboard` (declared with `--help` alone and the moves of `apply`): `AGENT_ENVIRONMENT` first, then
  `/dev/tty` opened by the core (`TERMINAL_REQUIRED`, new); the installer's interview (`crates/install/src/onboard.rs`,
  new, through the contract's `Interview`) asks the language in every catalogue's words, assesses at once so that
  `assess`'s refusals come before another question, then asks the mode, the publication with the warning and its
  exits, the alias, the key (with `~/` read from the home) with Git's commands to sign with it, the tools and, when no
  alambique exists, REQ-INS-8's three questions; it assesses with the answers and shows the plan whole; the
  catalogue's yes and the person's words make the command `apply`, with the plan's digest and those answers,
  dispatched in the same process. Any other answer to a question with choices, an empty needed answer, a line cut by
  the end of input or not UTF-8, or the end of input applies nothing, says so, exits 0. `--json` before a command
  that takes none is a usage error, with `--help` or not. The plan's text shows a file's carriage return. The
  catalogue's `[onboard]` (written by Codex), `command.onboard`, `code.TERMINAL_REQUIRED`; the generated help,
  completions and reference; `crates/fabrewk/tests/onboard.rs` (new, 14 tests on a pseudo-terminal through python3's
  `pty`, one in stages that takes the tree's digest while the yes and the words are awaited); the fixture `product`
  moved to the tests' `common`. `core-specification.md`: "Amended in part 10c (D-107)" after point 11.
- The interview's texts: Codex wrote the `[onboard]` table in `en` and `es` from point 8 and the code; the integrator
  checked each text against what the interview does (the alias's characters, `~/`, the tools' separator, the yes words
  fixed by point 8) and applied them as written (`a1ec2f6`).
- Codex's passes, each over the commit named, confirmed blocking findings (non-blocking in brackets):
  1. `a1ec2f6`: 3 [1]. P10C-1: the alias and the key's path were trimmed, so `onboard` applied a plan `assess` refuses
     with the same option. P10C-2: an answer of spaces alone to REQ-INS-8's questions was taken for empty. P10C-3: the
     tools were matched in their case. P10C-4 (not blocking): the agent's test let the language be asked first.
     Corrected in `c4202c6`.
  2. `c4202c6`: 3 [1]. P10C-5: a carriage return typed literally was cut from every answer. P10C-6: the tools were
     split on ASCII spaces alone. P10C-7: `~//x` replaced the home with `/x`. P10C-8 (not blocking): `--json` before
     `onboard` was taken. Corrected in `8af4482`.
  3. `8af4482` (a first launch named a wrong commit in its prompt and was stopped before a report; not counted): 1
     [2]. P10C-9: words typed without their line's end, then the end of input, were applied. P10C-10 (not
     blocking): `--json` before `onboard --help` gave the help. P10C-11 (coverage): no test watched the tree while the
     yes and the words were awaited. Corrected in `7110c06`.
  4. `7110c06`: 1. P10C-12: a home whose path is not UTF-8 was made lossy to expand `~/`, and could name another
     directory's key. Corrected in `2e04377`.
  5. `2e04377`: 1. P10C-13: the plan's text, through `str::lines`, lost a carriage return the first alambique holds.
     Corrected in `03f5470`.
  6. `03f5470`: accept, no finding; P10C-1 to P10C-13 closed.
  Every finding was checked against the code before its correction; each new test failed without it, but P10C-11's,
  a test of coverage that passed before. The integrator ran the suites before the first pass.
- Mutation over `4cf6913..HEAD` (`cargo mutants --package fabrewk-core --package fabrewk --package fabrewk-install
  --test-workspace true --baseline skip --features fabrewk-core/fault-injection --in-diff`, in a 2 GB scope with
  `ulimit -v 1600000`, the tests selected `--lib --test onboard --test surface --test reading --test apply --test
  install`, run green by hand first):
  1. Over `03f5470`, `-j 2 --timeout 300`: 62 mutants, 50 caught, 5 unviable, 5 missed, 2 timeouts. The missed: the
     guard on a line that is not UTF-8 (three), a language with a valid tag and no catalogue, and the commands to sign
     shown with Git already signing. Corrected in `bea2ce2`: every read that gives no whole line applies nothing, and
     tests type a byte that is not UTF-8, `fr` and `en-`, and assert no command to sign where Git signs.
  2. Over `bea2ce2`, `--iterate`: 34 mutants, 31 caught, 1 missed, 2 timeouts. The missed: `git.signs` beside the
     commands to sign, which the plan holds exactly when Git does not sign; the check dropped (`a1bfa51`), so no
     equivalent is declared.
  3. Over `a1bfa51`, `--iterate -j 1 --timeout 400`: 3 mutants, 1 caught, 2 timeouts. Both timed out alone too: `put`
     writing nothing and the yes inverted leave the test in stages waiting for a question that never comes, a loop
     caught by the timeout.
  None missed, none equivalent. The mutation's corrections came after Codex's accept, as in part 10b (R-052); no pass
  reviewed them.
- Verified (`verify-part`), clean clone of `a1bfa51`, every step in a user scope of 2 GB with two build jobs: 721
  tests passed, 11 ignored; with `fault-injection`, the binary's `crash` and `faults` and the core's `crash`, 52, green;
  the musl release build, `budget check` (the core 12,310 of 12,500; the installer 1,073 of 1,200; the backend 318 of
  400; Claude Code's adapter 269 of 600; Codex's 319 of 600; 5 direct runtime crates; 31 in the release build; the
  binary 3,674,504 bytes), both clippy runs, fmt and `cargo deny` clean; `--version --json` lists no feature. No idle
  `pasta` before or after.
- Measured: the core 12,310 of 12,500 (+55 over the part; 190 lines left for part 10d); the installer 1,073 of 1,200
  (+102; 127 left).
- Interpretations the maestro may want to read (each in the specification's "Amended in part 10c (D-107)"):
  1. An empty answer is the empty line alone: an alias with spaces is `ALIAS_INVALID` and words of spaces alone
     `WORDS_MISSING` (exit 1, nothing written), as on the agent's route, not a stop.
  2. An answer outside a question's choices stops the interview, applying nothing; nothing is asked again.
  3. The tools and REQ-INS-8's three questions take an empty answer (none; the placeholder line), although point 8
     says "an empty one, at any question", since REQ-INS-8 gives an empty answer its placeholder.
  4. The yes and the choices are taken trimmed and in any case (`SÍ`, `Yes`); another catalogue's yes is no yes.
  5. A key's path that starts with `~/` is read from the home; a home that is not UTF-8 is not expanded.
  6. The plan shown is `assess`'s text, its `apply` command included, which the yes runs.
  7. `TERMINAL_REQUIRED` is asserted in the human output alone, since `onboard` takes no `--json` (PD-35 read so).
  8. Seen, not changed (part 10a's, accepted): `assess` tells whether Git signs with the key by its path, so a copy
     of the same key at another path makes the plan show the commands to sign, although `apply` accepts that key.
  9. The mutation run judged each mutant with the suites the diff reaches, without the crash suite over F-1, as R-052
     interpretation 5; the clean clone runs everything.
- Part 10c is closed: REQ-INS-4's criterion on a pseudo-terminal in `es` and `en`, `TERMINAL_REQUIRED`, nothing written
  before the yes, by `crates/fabrewk/tests/onboard.rs`. The review directory, the clean clone and the mutation's
  output are removed after this entry.

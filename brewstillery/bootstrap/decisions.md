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

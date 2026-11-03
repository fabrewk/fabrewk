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

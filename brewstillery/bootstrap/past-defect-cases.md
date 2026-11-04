# Past defects as test cases

Plan phase 3, part 3b (D-023), written by the integrator in session 5. Every confirmed defect that a past review of
`tickbrew-harness` found, and the three found in part 3a, becomes a test case stated against what Fabrewk must do, not
against the harness's code. Phase 6 takes these cases into its test plan; phase 7 requires that they do not recur.
Codex reviews this file together with `harness-verdicts.md` when phase 3 ends. Provenance is in R-010.

## Sources

- `tickbrew-legacy` at `legacy-final`, read-only: the review, disposition and correction sections of
  `docs/automation/receipts/FZ-2.md` and `docs/automation/receipts/FZ-2b.md`, and
  `docs/engineering/gates/FZ2B_DECISION_PACKET.md`.
- `tickbrew-harness` at `0000bdc`, read-only: the finding keys in its tests, code and design note, the message of commit
  `c569e29` (which defines FX-1 to FX-8), and the bodies of the tests that hold each finding's variants.
- `harness-verdicts.md` (part 3a): H3A-01 to H3A-03. The receipts do not name them; they stay new.

The four AW1BR regressions come from an earlier gate (AW-1b) whose receipt was not read; they are taken as the FZ-2 C4
receipt and the harness's tests state them.

## How to read a case

- **Defect**: what happened, in one or two sentences. **Case**: what Fabrewk must do, written so that a test can check
  it whatever phase 5 decides for the store, the language or the command surface. **Variants**: inputs a review or a
  later fix found, each to be exercised. **Points**: the alambique points the case serves.
- Fabrewk's protected paths, named here as the harness's were: its domain `brewstillery/` (whole) and `.git`, at any
  depth. The harness's `.meta-harness/` becomes `brewstillery/` in every case below.
- Where a case depends on a mechanism Fabrewk has not chosen, it says so: "if the registry lives in Git" and the like.
- Each case needs a test that fails on a build without its fix. The harness's practice, kept as the standard: the test
  is red before the fix, green after, and mutations of the fix are caught (plan, phase 7).

Totals: 43 cases, one of them (PD-26) with 20 sub-cases, for 61 finding keys plus the variants the integrator found
while correcting them. Six items are not turned into cases, each with its reason. The table at the end traces every key
to its case.

## A. The registry and its integrity

**PD-01. A rolled-back or truncated log was accepted as valid.**
Defect: a later state holding only a prefix of the log loaded as valid and verification said so (FZ2-C5-04). Each fix
was beaten by the next review.
Case: any new state of the registry must strictly extend the last state seen; anything else is refused by name, and
verification walks the whole history, not only the head. A local mark of the last state seen refuses a reference that
does not descend from it.
Variants: (a) a later state whose log is a prefix of its parent's; (b) the reference moved back to an ancestor; (c) a
state with two parents, the old state first and the watched one second; (d) a linear chain through a truncated state
(seen, cut, regrown), found by the integrator; (e) if the registry lives in Git: a replacement object (`git replace`)
and (f) a graft file, so that objects are always read as stored.
Points: 6.26, 6.29, 6.37.

**PD-02. A damaged mark read as no mark.**
Defect: the mark of the last state seen was overwritten in place; an interruption left it empty, an empty mark read as
"no mark", and a rollback was then accepted (FZ2-C6-03).
Case: the mark is replaced whole (write a temporary file, sync, rename). An existing mark that is empty or not a valid
reference refuses by name, and only a person removes it.
Points: 6.26, 6.41.

**PD-03. The event format changed without a new version.**
Defect: review events gained a field without a version change; an older log failed on replay with a raw error
(FZ2-C6-01).
Case: any change to an event's fields changes the format version. A log of another version is refused by name or
migrated by an explicit, recorded step; replay never fails with a raw error.
Points: 6.26, 6.27.

**PD-04. One line-break character in any text corrupted the whole log, for good.**
Defect: the log was split with a function that also breaks on U+2028, U+2029 and U+0085. One such character in an
agent's summary or a person's words made the log unreadable for every task, permanently (FZ2B-C3-05). No review of the
core had found it.
Case: text holding any Unicode line or paragraph separator, in any recorded field, is stored and replayed intact; only
the format's own record separator ends a record. A test keeps every-Unicode-line-break splitting out of the code. A log
already holding such characters reads correctly.
Variants: the character in an agent's text; in a person's words; a truncated or tampered log is still refused.
Points: 6.26, 6.41.

**PD-05. A missing registry read as an empty one.**
Defect: on a clone without the state, the status said "no tasks" and verification said "ok" (C8F-2, corrected as FX-7).
Case: where the registry is absent, the status says it is absent and verification fails; neither reads absence as an
empty, valid registry.
Points: 6.27, 6.46, 6.18.

## B. Tasks, transitions and attempts

**PD-06. An approval could be replayed onto another task.**
Defect: an approval bound only the specification's digest (FZ2-C5-05).
Case: a person's approval binds the task's identity, the digest of what it approves and the task's position in its
history. Replayed onto another task with the same specification, or onto the same task later, it is refused.
Points: 6.24, 6.28.

**PD-07. Revalidations went past the round limit without a person.**
Defect: revalidation dispatched and accepted reviews beyond the round limit (FZ2-C5-06).
Case: the round limit is checked before any review starts, a revalidation included. Beyond it, only a person's decision
grants another round.
Points: 6.24, 6.42.

**PD-08. A result lost after a successful attempt left the task stuck.**
Defect: an interruption between an attempt's successful end and the recording of its result left the task in its
working state; resuming and running exited without advancing, and approval and retry were refused (FZ2-C8-03).
Case: the task is never stuck. Resuming moves it to a person's decision that says the attempt ended and that a retry
spends another dispatch. A result that names an attempt a retry replaced is refused.
Variants: each role (specification, author, review); a second lost result on the retry; no budget left, where it stops
for the person instead of spending; an older, answered dispute must not hide this message; another task advances
meanwhile. Property: an interruption before and after every event of a whole cycle leaves no task stuck and causes no
dispatch without a person.
Points: 6.41, 6.28.

**PD-09. Stopping an interrupted task blocked every other task.**
Defect: stopping a task left its attempt running; every later run on any task then failed (FZ2-C8-04).
Case: stopping a task closes its running attempt as unknown in the same step, still counted as spent; a later end or
decision for it is refused; work on every other task goes on.
Variants: a stop from each working state; every decision that stops.
Points: 6.41, 6.28.

**PD-10. An attempt could end twice, or be lost by an interruption.**
Defect: two regressions of an earlier gate: a second completion of the same attempt (AW1BR-2), and an interruption that
lost an attempt already spent (AW1BR-3).
Case: an attempt is recorded before it runs and ends exactly once; a second end is refused and the first kept. An
interrupted attempt stays spent, becomes unknown, and is never retried without a person.
Points: 6.41, 6.40.

**PD-11. A raw tool error in one task's merge stopped all tasks.**
Defect: a raw Git error while completing a merge during resume stopped the run on every task until it cleared
(FZ2B-C1-03).
Case: a tool error while completing one task's merge is recorded in that task as a failed merge, with its reason in
words; other tasks go on.
Variants: a real cause, such as a leftover index lock.
Points: 6.41.

## C. Integration

**PD-12. A resumed merge skipped its validations.**
Defect: resume completed an interrupted merge without the merge checks, reaching "integrated" with a dirty target or a
stale reviewer profile (FZ2-C5-01).
Case: every precondition of a merge is checked before the authorization, again just before the target moves, and again
with the target at the candidate before the task is integrated. Resume runs the same checks. A failure records a failed
merge and the task waits for the person.
Variants: the target already at the candidate, moved by someone else; an interruption between the move and the sync of
a checked-out working tree.
Points: 6.44, 6.41.

**PD-13. The target moved without a compare-and-swap.**
Defect: with the target checked out, a fast-forward ran without checking the base; a race could integrate on a moved
base (FZ2-C5-02).
Case: the target moves only from the recorded base to the candidate, in one compare-and-swap; if it moved, the merge is
refused as a moved base. A checked-out working tree is brought to the candidate only if it is still clean at the base.
Points: 6.44, 6.37.

**PD-14. A stop could hide a merge that had happened.**
Defect: stopping a task whose authorized merge had already moved the target recorded "stopped" with the target at the
candidate (FZ2B-C1-02).
Case: while a task has a merge authorization on record and the target is at its candidate, stopping is refused and the
merge can be finished. With no authorization on record, stopping stays available whatever the target.
Variants: the reviewer's: authorized, failed before moving, retried back to accepted, then the target moved by hand.
Points: 6.28, 6.29.

## D. What an agent's change may hold

**PD-15. The lane guard compared sample paths.**
Defect: at specification, lanes were checked against a few sample paths, so patterns that reach the protected domain
were admitted (FZ2-C8-01).
Case: a lane is refused before the approval is asked if any of its patterns can match a protected path, exactly or
inside, with empty and `.` segments dropped and case folded. `..`, a `.git` segment at any depth, and patterns longer
than a fixed limit are refused outright. The guard decides from the same pattern language the matcher uses.
Variants: the review's four, in Fabrewk's terms: `**/*.png`, `*/xel/**`, `?rewstillery/settings.toml`,
`**/attachments/**`; also `*`, `**`, `**/*.py`, `*/config/*`, `*.git`, `.G*`, `./brewstillery/x`, `src/../.git/config`.
Kept: `*.md`, `src/**`, `.github/**`, `.gitignore`, `.git*ignore`, `tests/fixtures/brewstillery/**`,
`brewstillery-old/x`. Properties: an exhaustive comparison of guard and matcher over small tokens, both ways; random
long patterns never match a protected path unnoticed.
Points: 6.3, 6.6, 6.13, 6.37.

**PD-16. The domain's own name, exact, passed the write guard.**
Defect: a lane naming the domain's exact path was admitted and a file there passed the guard; Git then failed with a
raw error. With the domain not committed in the base and the target not checked out, the file reached "integrated"
(FZ2-C8-02, the second part found by the integrator).
Case: a write guard independent of the lane guard refuses the domain's exact name, anything inside it, `.git` at any
depth, their case variants and a symbolic link at those names; it is not a prefix match. Whatever Git refuses to hold
stops with a named refusal. A candidate whose change from the base holds a protected path never integrates.
Variants: both configurations of the review (domain committed or not; target checked out or not); each layer with the
earlier ones forced off.
Points: 6.3, 6.13, 6.44.

**PD-17. The lane matcher could hang.**
Defect: the matcher was a backtracking regular expression; a lane with many `**` hung on a long path (FZ2B-C1-01).
Case: matching takes time bounded in the pattern and path lengths, with no backtracking.
Variants: `src/` followed by eight `**a` and then `b`, on a 60-character path (24 s before); 150 `**` on a
4,000-character path. Property: the matcher equals a reference definition over random pairs, newlines included.
Points: 6.28, 6.41.

**PD-18. An ignored file with an unusual name reached the target.**
Defect: the ignore check quoted unusual names, which then failed to match, so an ignored file entered the candidate and
the target after the merge (FZ2B-C5-R-3).
Case: a file the repository's ignore rules exclude never enters a candidate, whatever its name; a tracked file with
unusual characters is kept.
Variants: names with a space, a double quote, a backslash, a newline, a tab, an accented letter, U+2028.
Points: 6.37, 6.38, 6.44.

**PD-19. An untouched symbolic link in the base stopped every task.**
Defect: every symbolic link in the author's copy was reported as a change, even one already tracked and untouched, so
the task stopped as a lane violation or a refused link (H3A-01).
Case: a symbolic link tracked in the base and left untouched is not in the change set and the task proceeds. Whether a
new or changed link is refused or allowed is a phase 6 rule; an untouched one is never a change.
Points: 6.37, 6.44.

**PD-20. A change of the executable bit alone was lost without notice.**
Defect: the change set compared bytes only; a file made executable with the same content was dropped, the candidate
kept mode 100644 and the task was accepted (H3A-02).
Case: a mode change alone is in the change set and the candidate carries it, or the change is refused by name; it is
never dropped silently.
Points: 6.29, 6.44.

**PD-21. A symbolic link to an absolute path in the base failed before any dispatch.**
Defect: exporting the task copy refused such a link with a raw error; nothing was recorded and every run failed the
same way (H3A-03).
Case: the copy holds the link as a link, never followed, or the task is refused by name and the refusal recorded
before any dispatch. A raw error never reaches the person.
Points: 6.41, 6.38.

**PD-22. Configuration and maintainers' spaces reached the agents.**
Defect: task copies held the harness's configuration, and the reviewer printed it (FZ-2 C7 delivery, item 3; corrected
as FX-5).
Case: the copies given to agents and to checks leave out the domain `brewstillery/` whole, unless a task names the
files it may read. Configuration, alambiques and support material never reach an agent or a provider by default.
Points: 6.6, 6.9, 6.13, 6.38.

## E. Checks

**PD-23. Removing checks after acceptance did not invalidate it.**
Defect: the gate ignored the checks approved in the specification, and removing checks after acceptance kept the
acceptance valid (FZ2-C5-03).
Case: an acceptance's identity includes the digest of the configured checks. The gate requires the union of the
configured required checks and those the specification approved. Removing a check after acceptance invalidates it.
Points: 6.43, 6.29.

**PD-24. Two checks with the same name hid a failure.**
Defect: a required check running `false` and an optional one with the same id running `true`: the gate kept the last
and the task integrated (FZ2-C6-02).
Case: check names are unique; a duplicate refuses at configuration load, before any check runs.
Points: 6.43, 6.10.

## F. Evidence that a reviewer examined

**PD-25. An accept with nothing examined was taken as a review.**
Defect: a well-formed `accept` after zero commands was accepted; in a live cycle it would have sent unreviewed code to
the merge (FZ2R-1).
Case: a review whose evidence of examination is zero, or void under PD-26, is invalid output whatever its verdict, and
the task waits for the person.
Points: 6.42, 6.43.

**PD-26. The audit of a reviewer's stream read malformed evidence as examination.**
Defect: across six reviews, the audit of what a reviewer ran counted as examination streams that a real run would not
produce, case after case, until the fix changed shape: what a stream may hold is listed in one place and any anomaly
voids the examination (FZ2B-C3 amendment).
Case: one rule per provider's stream format. Only a completed command with its text, or a tool use that names a tool
that reads or runs and was answered before the result, counts. Only a successful final result counts. The first end of
a turn is final. An unknown exit code is never a success. Any anomaly voids the count. Each anomaly below is inserted
at every position of a valid real stream, and through the whole flow an accepting review on such a stream waits for the
person.

| Sub-case | Anomaly | Key |
| --- | --- | --- |
| PD-26.1 | A command with no exit code counted as a success | AW1BR-1 |
| PD-26.2 | Paths read before splitting a command on unquoted separators and newlines (quoted ones do not split) | AW1BR-4 |
| PD-26.3 | A second completion of the same command | AW1BR-2 (audit) |
| PD-26.4 | One provider's dispatches had no audit at all | C8F-4 |
| PD-26.5 | A bare tool use with no id, name, input or answer | C8F-4 (C3 review 1) |
| PD-26.6 | A run ending by turn limit with a well-formed output accepted | C8F-4 (C3 review 1) |
| PD-26.7 | A command started and never completed | C3 disposition 1 |
| PD-26.8 | A tool name that is `true`, a number or a list; a completed command with no text | FZ2B-C3-03 |
| PD-26.9 | A failed result followed by a successful one; a use or answer in the wrong kind of event; a non-text command that crashed the audit | C3 disposition 2 |
| PD-26.10 | An error flag followed by a success; command text after the end; a late start | C3 failed dispatch, leads |
| PD-26.11 | An id converted to text, so a numeric start joined a text completion | FZ2B-C3-R4-1 |
| PD-26.12 | An error after the end of the turn reopened the count | FZ2B-C3-R4-2 |
| PD-26.13 | A start repeated before its completion | FZ2B-C3-R5-1 |
| PD-26.14 | Command text changed to empty or to a non-text value | FZ2B-C3-R5-2 |
| PD-26.15 | A boolean exit code read as an integer | FZ2B-C3-R5-3 |
| PD-26.16 | A valid stream broken by U+0085, U+2028 or U+2029 inside its text (must read whole) | FZ2B-C3-R5-4 |
| PD-26.17 | An unknown event on a command | C3 closure |
| PD-26.18 | The provider's own answer tool counted as examination | FZ2B-C4-01 |
| PD-26.19 | A blank line inside a stream skipped instead of void (a final newline is not a line) | FZ2B-C5-R-2 |
| PD-26.20 | A non-integer exit code on a command's start | FZ2B-C5-R-4 |

Limit, stated with the case: this count checks a lazy or failed run, not a hostile agent; whether an agent's process can
replace what Fabrewk reads had no independent review.
Points: 6.42, 6.43, 6.38, 6.36.

## G. Dispatch, providers and raw output

**PD-27. The confinement did not fit the provider's CLI.**
Defect: (a) only the CLI's main binary was bound, and it needed sibling executables (FZ2R-2); (b) the provider's own
sandbox could not start inside a read-only copy, so the reviewer ran nothing (FZ2R-5).
Case: before a provider's first dispatch, a check that calls no model runs the CLI, its helpers and its own sandbox
inside the confinement, with the copy permissions of each role, and creates a temporary file. The CLI's release
directory is bound whole and read-only. A reviewer's copy may be writable but is never read back.
Points: 6.36, 6.38, 6.46.

**PD-28. A failed dispatch lost its cause.**
Defect: the adapter discarded the output of a failed run, where the CLI writes its errors (FZ2R-3); the raw stream was
not kept at all, so a question about it could not be answered (C7C-5).
Case: every dispatch, succeeded, failed or timed out, keeps its raw output and error channels on the machine,
unversioned and readable only by the owner, with their digests in the log; the person is told where to look.
Points: 6.29, 6.38, 6.41.

**PD-29. Provider text reached the versioned log.**
Defect: provider error text entered the log through the attempt's end (FZ2R-4, C3 review 1); then the provider's error
subtype (FZ2B-C3-04).
Case: from a dispatch, the log holds only Fabrewk's own short texts, codes, counts and digests. Provider text, error
subtypes and free-form usage fields stay in the local raw files. The model and CLI version the CLI reports are kept as
client-reported identity.
Points: 6.38, 6.26.

## H. Person authority

**PD-30. The words given with a new task were not recorded.**
Defect: the words were mandatory but discarded (C7C-4, corrected as FX-2).
Case: every person command records the person's words verbatim, the one that creates a task included.
Points: 6.24, 6.29.

**PD-31. Empty words were accepted.**
Defect: an empty value let approval, merge, stop and decisions record no words; a task reached "integrated"
(FZ2B-C5-R-1).
Case: words that are missing, empty, blank or not text are refused by name, at the command and again where state
changes; the state does not move.
Variants: `""`, `" "`, tab and newline, U+2028 alone; for every person command.
Points: 6.24.

**PD-32. Example words were copied as the person's own.**
Defect: decision requests showed an example with words, and the person typed it literally three times (FZ-2 C7 and
FZ2B-C4-02).
Case: no message offers words to copy. A command copied whole from a message fails on the missing words and changes
nothing.
Points: 6.24, 6.18.

**PD-33. A person's decision did not advance the work.**
Defect: the person commands only recorded the decision; an external loop had to run the harness, and the waiting line
read as an instruction to the person (C7C-1, C7C-2; corrected as FX-1).
Case: after a person's decision the work continues without another command or an external loop. A waiting message says
who acts next and gives the person no command when nothing is theirs to do.
Points: 6.24, 6.34, 6.18.

## I. Messages to people

**PD-34. Messages showed internal tokens.**
Defect: raw reasons and an attempt key without description (FZ2-C5-07); then state and event tokens in transition
refusals (FZ2-C5-07, review 5); then check states, stale fields and a doubled period in live output (C7C-7, corrected
as FX-4).
Case: rendering every message with every parameter value shows no internal token without words and no doubled
punctuation; every key carries its description. A test requires words for every state, event, role and reason in every
catalogue.
Points: 6.18, 6.19, 6.20.

**PD-35. A refusal was shown without its machine code.**
Defect: refused person commands printed no code (C7C-6, corrected as FX-3).
Case: every refusal shown to a person carries its machine code beside the words.
Points: 6.18.

**PD-36. Unfilled placeholders and raw errors reached the person.**
Defect: a halt showed a literal `{path}` (FZ2-C8-01, receipt of its correction); a raw Git error surfaced instead of a
refusal (FZ2-C8-02).
Case: no message reaches the person with an unfilled parameter, and no raw tool error does; each becomes a named
refusal with words.
Points: 6.18.

**PD-37. The approval did not show what it froze.**
Defect: the approval request did not show the criterion and examples it froze (C8F-1, corrected as FX-6).
Case: the approval request shows, verbatim, the criterion and examples the approval freezes. Their language is a case
for phase 6 under 6.20 and 6.22: in both pilots they reached a Spanish-speaking maintainer in English.
Points: 6.25, 6.18, 6.20.

**PD-38. Messages named a command that does not exist, for the wrong shell.**
Defect: requests named `tbh`, which existed nowhere, and the instructions assumed bash for a fish user (FZ2B-C4-03).
Case: every command a message names exists in the command surface, checked by a test against the command parser.
Commands given to a maintainer work in the shell their settings name.
Points: 6.5, 6.18, 6.11.

## J. Views

**PD-39. An unknown figure was shown as zero.**
Defect: the report showed unreported cached tokens as 0 (FZ2B-C3-01).
Case: an unreported figure reads as no data; a reported zero stays zero; a sum with an unknown part is unknown.
Points: 6.40.

**PD-40. A response time was counted with no decision pending.**
Defect: after an interrupted dispatch, a stop read "you took 1 h", 59 minutes of it the agent's (FZ2B-C3-02).
Case: a person's response time is measured only from the moment a decision became pending; otherwise the view says
there is none.
Points: 6.40, 6.39.

## K. Records

Found by Codex's review of the legacy repository's records, not of the harness. They are behaviour Fabrewk takes over
when it writes its own record.

**PD-41. A maintainer's edit was committed inside an agent's delivery.**
Defect: an integrator commit mixed the maintainer's board edit with its own work; authorship could not be shown
(FZ2B-C0-01).
Case: a change to a maintainer's input is committed alone, attributed to the maintainer, with no AI co-author; a commit
that mixes it with an agent's writes is refused.
Points: 6.7, 6.37.

**PD-42. Records stated counts and states the evidence contradicted.**
Defect: a packet said four valid reviews where there were three (FZ2B-C0-02); a closure named a status a register did
not hold (FZ2B-C0-03).
Case: counts and states a record cites are computed from the registry, or checked against it before the record is
committed.
Points: 6.26, 6.29.

**PD-43. A status line stated a fact that would become false.**
Defect: the status named the branch head as fixed, false as soon as the branch moved (FZ2B-C0-04).
Case: the status is a view generated from the registry; regenerated, it matches.
Points: 6.26.

## Not turned into cases

| Key | Why | Destination |
| --- | --- | --- |
| FZ2R-4, hosted endpoint | Codex's attempt to reach a hosted MCP endpoint was never confirmed: inconclusive in every gate | Phase 9, confinement and network |
| C8F-3 | Git levels, task dependencies and a same-provider review fallback were not implemented: missing features, not defects | Phase 5 and 6.28, 6.42, 6.44 |
| C8F-5, FX-8 | No test named three check refusal codes; the gate raised each correctly | Phase 6 test plan, refusal list |
| C7C-3, C7C-8 | How interventions and response time were counted | Phase 11, pilot measures |
| C7C-9, C7C-10 | A hand-placed index row; ignored files the integrator left in the pilot | Integrator conduct, not Fabrewk behaviour |
| A task invalidated to "base moved" whose target then reaches its candidate | A known corner with only a manual way out, accepted at FZ-2b's closure | Phase 6, with PD-14 |

## Trace of keys

| Key | Case |
| --- | --- |
| FZ2R-1 | PD-25 |
| FZ2R-2 | PD-27 |
| FZ2R-3 | PD-28 |
| FZ2R-4 (provider text in the log) | PD-29 |
| FZ2R-5 | PD-27 |
| FZ2-C5-01 | PD-12 |
| FZ2-C5-02 | PD-13 |
| FZ2-C5-03 | PD-23 |
| FZ2-C5-04 | PD-01 |
| FZ2-C5-05 | PD-06 |
| FZ2-C5-06 | PD-07 |
| FZ2-C5-07 | PD-34 |
| FZ2-C6-01 | PD-03 |
| FZ2-C6-02 | PD-24 |
| FZ2-C6-03 | PD-02 |
| FZ2-C8-01 | PD-15, PD-36 |
| FZ2-C8-02 | PD-16, PD-36 |
| FZ2-C8-03 | PD-08 |
| FZ2-C8-04 | PD-09 |
| C7C-1, C7C-2 | PD-33 |
| C7C-4 | PD-30 |
| C7C-5 | PD-28 |
| C7C-6 | PD-35 |
| C7C-7 | PD-34 |
| C8F-1 | PD-37 |
| C8F-2 | PD-05 |
| C8F-4 | PD-26.4 to PD-26.6 |
| FX-1 to FX-7 | PD-33, PD-30, PD-35, PD-34, PD-22, PD-37, PD-05 |
| AW1BR-1 to 4 | PD-26.1, PD-10, PD-10, PD-26.2 (and PD-26.3) |
| FZ2B-C0-01 to 04 | PD-41, PD-42, PD-42, PD-43 |
| FZ2B-C1-01 to 03 | PD-17, PD-14, PD-11 |
| FZ2B-C3-01, 02 | PD-39, PD-40 |
| FZ2B-C3-03, 04 | PD-26.8, PD-29 |
| FZ2B-C3-05 | PD-04 |
| FZ2B-C3-R4-1, R4-2 | PD-26.11, PD-26.12 |
| FZ2B-C3-R5-1 to R5-4 | PD-26.13 to PD-26.16 |
| FZ2B-C4-01 to 03 | PD-26.18, PD-32, PD-38 |
| FZ2B-C5-R-1 to R-4 | PD-31, PD-26.19, PD-18, PD-26.20 |
| H3A-01 to 03 | PD-19, PD-20, PD-21 |
| FZ-2 C7 delivery, item 3 | PD-22 |
| C3 dispositions and closure, integrator's variants | PD-26.7, PD-26.9, PD-26.10, PD-26.17 |

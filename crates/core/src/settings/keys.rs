//! Every settings key, declared once (REQ-SET-2; core-specification section 8): its path (`*` stands for one name),
//! type, the levels allowed to set it, its default, its direction and its description.

use super::Def::{Absent, From, Lit, Required};
use super::Dir::{Narrows, Neutral, Widens};
use super::Level::{Machine, Personal, Project, Stage, Task};
use super::Type::{Alias, Bool, Command, Enum, Int, Ordered, Path, SignerKey, Str, Strs, Tag, Tags};
use super::{Def, Dir, Key, Level, Type};

const P: &[Level] = &[Project];
const PST: &[Level] = &[Project, Stage, Task];
const ME: &[Level] = &[Personal];
const M: &[Level] = &[Machine];

const fn k(path: &'static str, ty: Type, levels: &'static [Level], default: Def, dir: Dir, about: &'static str) -> Key {
    Key { path, ty, levels, default, dir, about }
}

/// The keys of the first core. A key not listed here is refused in every file (`SETTINGS_INVALID`).
pub const KEYS: &[Key] = &[
    k("project.name", Str, P, Required, Neutral, "The project's name"),
    k("maintainers.*.role", Enum(&["maestro", "supervisor", "contributor"]), P, Required, Widens, "The role"),
    k("maintainers.*.key", SignerKey, P, Required, Widens, "The signing key, as allowed signers write it"),
    k("maintainers.*.hostAccount", Str, P, Absent, Neutral, "Account on the Git host"),
    k("maintainers.*.area", Strs, P, Absent, Widens, "A supervisor's area; absent, the whole project"),
    k("languages.canonicalDocumentationLanguage", Tag, P, Lit("\"en\""), Neutral, "Technical files"),
    k(
        "languages.documentationLanguages",
        Tags,
        P,
        From("languages.canonicalDocumentationLanguage"),
        Neutral,
        "Publication languages of the documentation",
    ),
    k("languages.codeLanguage", Tag, P, From("languages.canonicalDocumentationLanguage"), Neutral, "Code"),
    k("languages.evidenceLanguage", Tag, P, From("languages.canonicalDocumentationLanguage"), Neutral, "Evidence"),
    k(
        "languages.delegationLanguage",
        Tag,
        P,
        From("languages.canonicalDocumentationLanguage"),
        Neutral,
        "Packets delegated to agents",
    ),
    k(
        "languages.agentInteraction",
        Tag,
        ME,
        From("languages.canonicalDocumentationLanguage"),
        Neutral,
        "Agents' answers to this maintainer",
    ),
    k("languages.interfaceLanguage", Tag, ME, From("languages.agentInteraction"), Neutral, "Fabrewk's messages"),
    k("maintainer.alias", Alias, ME, Required, Neutral, "This file's maintainer"),
    k("environment.shell", Enum(&["fish", "sh", "bash", "zsh"]), ME, Lit("\"sh\""), Neutral, "The person's shell"),
    k("tools.*.adapter", Enum(&["claude-code", "codex"]), P, Required, Widens, "Which module drives the tool"),
    k("tools.*.providers", Strs, P, Required, Widens, "Providers the tool may use"),
    k("tools.*.path", Path, M, Absent, Neutral, "Where the tool is, when detection cannot find it"),
    k("providers.*.allowed", Bool, P, Lit("false"), Widens, "Whether code may be sent to it"),
    k("models.*.provider", Str, P, Required, Widens, "The model's provider"),
    k("models.*.id", Str, P, Required, Widens, "The provider's name for the model"),
    k("models.*.efforts", Strs, P, Required, Widens, "The model's efforts"),
    k("profiles.*.kind", Enum(&["build", "review", "sweep", "document"]), P, Required, Neutral, "Task kind"),
    k("profiles.*.tool", Str, PST, Required, Widens, "A tool of the catalogue"),
    k("profiles.*.model", Str, PST, Required, Widens, "A model of the catalogue"),
    k("profiles.*.effort", Str, PST, Required, Widens, "One of the model's efforts"),
    k("profiles.*.checks", Strs, PST, Lit("[]"), Narrows, "Checks run"),
    k("profiles.*.requiredTools", Strs, PST, Lit("[]"), Narrows, "Tools verified at start"),
    k("profiles.*.rounds", Int, PST, From("limits.rounds"), Widens, "Review rounds"),
    k("profiles.*.dispatches", Int, PST, From("limits.dispatches"), Widens, "Dispatches"),
    k("profiles.*.minutes", Int, PST, From("limits.minutes"), Widens, "Minutes"),
    k("profiles.*.tokens", Int, PST, Absent, Widens, "Tokens; absent, unlimited"),
    k("limits.rounds", Int, PST, Lit("3"), Widens, "Default review rounds"),
    k("limits.dispatches", Int, PST, Lit("6"), Widens, "Default dispatches"),
    k("limits.minutes", Int, PST, Lit("120"), Widens, "Default minutes"),
    k("limits.freeDiskMegabytes", Int, P, Lit("2048"), Narrows, "Free space needed before a copy"),
    k("checks.*.command", Command, P, Required, Neutral, "The command, without a shell"),
    k("checks.*.network", Bool, P, Lit("false"), Widens, "Whether the check has network"),
    k("checks.*.minutes", Int, P, Lit("30"), Widens, "The check's time limit"),
    k("checks.*.tools", Strs, P, Lit("[]"), Neutral, "Tools the check needs"),
    k(
        "review.differentProvider",
        Ordered(&["off", "prefer", "require"]),
        PST,
        Lit("\"prefer\""),
        Narrows,
        "Review by another provider",
    ),
    k(
        "gate.protectedPaths",
        Strs,
        PST,
        Lit("[\"Cargo.toml\", \"Cargo.lock\", \".github/\"]"),
        Narrows,
        "Paths no task may touch",
    ),
    k(
        "gate.protectedNames",
        Strs,
        PST,
        Lit(
            "[\".gitattributes\", \".gitmodules\", \"CLAUDE.md\", \".claude\", \"AGENTS.md\", \".codex\", \".agents\"]",
        ),
        Narrows,
        "Names protected at any depth",
    ),
    k("git.signOff", Bool, P, Lit("false"), Neutral, "Sign-off on Fabrewk's commits"),
    k("extensions.*.digest", Str, P, Absent, Neutral, "The extension's digest"),
    k("extensions.*.network", Bool, P, Lit("false"), Widens, "Whether the extension has network"),
    k("extensions.*.minutes", Int, P, Lit("30"), Widens, "The extension's time limit"),
];

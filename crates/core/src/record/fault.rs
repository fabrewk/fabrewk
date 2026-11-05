//! Fault injection for the crash suite (core-specification 11.5, 11.2 rule 8). Only with the Cargo feature
//! `fault-injection`, never in a release build: every write of the record passes a numbered point, and the process
//! aborts at the point `FABREWK_FAULT_ABORT_AT` names, as a kill would stop it. `FABREWK_FAULT_FAIL` names a step that
//! fails instead of running. Without the feature both are empty and inlined away.

#[cfg(feature = "fault-injection")]
static POINTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Passes one write point; with the feature, aborts the process if it is the chosen one.
#[inline]
pub(crate) fn point() {
    #[cfg(feature = "fault-injection")]
    {
        let n = POINTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        if std::env::var("FABREWK_FAULT_ABORT_AT").is_ok_and(|at| at == n.to_string()) {
            std::process::abort();
        }
    }
}

/// How many write points this process has passed (always 0 without the feature).
pub fn points() -> usize {
    #[cfg(feature = "fault-injection")]
    return POINTS.load(std::sync::atomic::Ordering::SeqCst);
    #[cfg(not(feature = "fault-injection"))]
    0
}

/// Whether the step `name` must fail (always false without the feature).
#[inline]
pub(crate) fn fails(name: &str) -> bool {
    #[cfg(feature = "fault-injection")]
    return std::env::var("FABREWK_FAULT_FAIL").is_ok_and(|n| n == name);
    #[cfg(not(feature = "fault-injection"))]
    {
        let _ = name;
        false
    }
}

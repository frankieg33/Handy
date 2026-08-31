//! Downstream build configuration and policy for personal Handy fork.

/// Whether update checks are hard-disabled for this downstream build.
pub const DISABLE_UPDATER: bool = true;

pub fn is_updater_disabled() -> bool {
    DISABLE_UPDATER
}

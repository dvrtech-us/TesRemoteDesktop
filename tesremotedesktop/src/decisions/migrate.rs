//! What `service install` copies, and when the previous Windows service blocks it.
//! The copy itself is in `crate::migrate`. Nothing here touches the disk.

use super::legacy;

/// How long `service install` waits for the previous Windows service to stop
/// and for its executable to leave the process list.
pub const OLD_SERVICE_STOP_LIMIT_MS: u64 = 30_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OldServicePlan {
    /// No service with the previous name. Copy and install; there is nothing to delete.
    Absent,
    /// The previous service reports stopped and its executable is not running.
    /// Copy, install the new service, then delete the previous service.
    Replace,
    /// The previous service is still running, or its executable is still running.
    /// Install must stop here. No files are copied and the new service is not created.
    Abort,
}

pub fn old_service_plan(exists: bool, reported_stopped: bool, exe_running: bool) -> OldServicePlan {
    if !exists {
        OldServicePlan::Absent
    } else if reported_stopped && !exe_running {
        OldServicePlan::Replace
    } else {
        OldServicePlan::Abort
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrateCandidate {
    pub source_name: String,
    pub dest_name: String,
    pub source_exists: bool,
    pub dest_exists: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrateAction {
    /// Copy to a temporary name in the destination directory, then rename into place.
    Copy { source_name: String, dest_name: String },
    /// The new folder already has this file. Leave both copies alone.
    SkipPresent { dest_name: String },
    /// The old folder has no such file.
    SkipMissing { source_name: String },
}

/// One decision per file, in the order given. A file the destination already
/// has is never copied, even when the source is also present.
pub fn migrate_actions(files: &[MigrateCandidate]) -> Vec<MigrateAction> {
    files
        .iter()
        .map(|file| {
            if file.dest_exists {
                MigrateAction::SkipPresent { dest_name: file.dest_name.clone() }
            } else if !file.source_exists {
                MigrateAction::SkipMissing { source_name: file.source_name.clone() }
            } else {
                MigrateAction::Copy {
                    source_name: file.source_name.clone(),
                    dest_name: file.dest_name.clone(),
                }
            }
        })
        .collect()
}

/// The five files `service install` considers, in order.
/// Config-folder names (`.env`, certificate, key, account) keep their names.
/// The database is renamed from the previous file name to the current one.
/// Callers supply the names from `paths` so this list is not a second copy of them.
pub fn install_file_names(env_name: &str, cert_name: &str, key_name: &str, account_name: &str, new_db_name: &str) -> Vec<(String, String)> {
    vec![
        (env_name.to_string(), env_name.to_string()),
        (cert_name.to_string(), cert_name.to_string()),
        (key_name.to_string(), key_name.to_string()),
        (account_name.to_string(), account_name.to_string()),
        (legacy::DB_FILE.to_string(), new_db_name.to_string()),
    ]
}

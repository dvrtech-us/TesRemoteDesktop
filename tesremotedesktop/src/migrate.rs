//! Copy the previous install's files during `service install`.
//! Each copy lands on a temporary name in the new folder and is renamed into
//! place. The old folder is only read.

use std::path::{Path, PathBuf};

use crate::decisions::legacy;
use crate::paths;

#[cfg(any(windows, target_os = "linux"))]
use crate::decisions::migrate::{migrate_actions, MigrateAction, MigrateCandidate};

/// Build the five source/dest pairs from the path helpers.
/// Config files stay in the config folder. The database stays in the data folder.
/// On Windows those two folders are the same directory.
pub fn install_file_pairs(old_config: &Path, new_config: &Path, old_data: &Path, new_data: &Path) -> Vec<(PathBuf, PathBuf)> {
    let names = [
        file_name(&paths::env_file()),
        file_name(&paths::cert_pem()),
        file_name(&paths::key_pem()),
        file_name(&paths::acme_account()),
    ];
    let mut pairs = Vec::with_capacity(5);
    for name in names {
        pairs.push((old_config.join(&name), new_config.join(&name)));
    }
    pairs.push((old_data.join(legacy::DB_FILE), new_data.join(file_name(&paths::db_file()))));
    pairs
}

#[cfg(windows)]
pub(crate) fn windows_install_pairs() -> Vec<(PathBuf, PathBuf)> {
    let old = paths::legacy_windows_dir();
    let new_dir = paths::data_dir();
    install_file_pairs(&old, &new_dir, &old, &new_dir)
}

#[cfg(target_os = "linux")]
pub(crate) fn linux_install_pairs() -> Vec<(PathBuf, PathBuf)> {
    install_file_pairs(
        &paths::legacy_linux_config_dir(),
        &paths::config_dir(),
        &paths::legacy_linux_data_dir(),
        &paths::data_dir(),
    )
}

#[cfg(any(windows, target_os = "linux"))]
pub(crate) fn copy_files(pairs: &[(PathBuf, PathBuf)]) -> Result<(), String> {
    let candidates: Vec<MigrateCandidate> = pairs
        .iter()
        .map(|(source, dest)| MigrateCandidate {
            source_name: file_name(source),
            dest_name: file_name(dest),
            source_exists: source.is_file(),
            dest_exists: dest.is_file(),
        })
        .collect();
    for (action, (source, dest)) in migrate_actions(&candidates).iter().zip(pairs) {
        match action {
            MigrateAction::Copy { .. } => {
                copy_atomic(source, dest)?;
                println!("[Service] Copied {} -> {}", source.display(), dest.display());
            }
            MigrateAction::SkipPresent { dest_name } => {
                println!("[Service] Skip {dest_name}: the new folder already has it.");
            }
            MigrateAction::SkipMissing { source_name } => {
                println!("[Service] Skip {source_name}: not in the old folder.");
            }
        }
    }
    Ok(())
}

#[cfg(any(windows, target_os = "linux"))]
fn copy_atomic(source: &Path, dest: &Path) -> Result<(), String> {
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent).map_err(|err| format!("create {}: {err}", parent.display()))?;
    }
    let mut temp_name = dest.file_name().ok_or_else(|| format!("no file name in {}", dest.display()))?.to_os_string();
    temp_name.push(".migrate-tmp");
    let temp = dest.with_file_name(&temp_name);
    if let Err(err) = std::fs::copy(source, &temp) {
        return Err(format!("copy {} -> {}: {err}", source.display(), temp.display()));
    }
    if let Err(err) = std::fs::rename(&temp, dest) {
        let _ = std::fs::remove_file(&temp);
        return Err(format!("rename {} -> {}: {err}", temp.display(), dest.display()));
    }
    Ok(())
}

fn file_name(path: &Path) -> String {
    path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default()
}

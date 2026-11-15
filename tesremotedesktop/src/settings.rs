//! The only way a setting is read. Resolution is `decisions::settings::resolve_setting`.
//! `.env` values stay in this store. They are not copied into the process environment,
//! so a real variable under the previous prefix still beats a `.env` value under the
//! current prefix, and a value written by `POST /config` or Fit screen is visible
//! to the next read.

use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock};

use crate::decisions::config::merge_dotenv;
use crate::decisions::settings::{self, overlay_pairs, pair_value, resolve_setting};
use crate::paths;

struct Store {
    /// Keys that were in the process environment before `.env` was read.
    real: HashMap<String, String>,
    /// Executable `.env`, then the config-folder `.env` over it.
    dotenv: Vec<(String, String)>,
    /// Short setting names written during this process.
    runtime: HashMap<String, String>,
}

static STORE: OnceLock<Mutex<Store>> = OnceLock::new();

/// Snapshot the real environment and both `.env` files. Later calls do nothing.
/// `setting` loads on first use when this was not called.
pub fn load() {
    let _ = store();
}

/// `name` is the short setting name (`HTTPS_HOST`, not the env key).
pub fn setting(name: &str) -> Option<String> {
    let Some(spec) = settings::by_name(name) else {
        return None;
    };
    let guard = store().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    resolve_setting(
        guard.runtime.get(spec.name).map(String::as_str),
        guard.real.get(spec.new_key).map(String::as_str),
        guard.real.get(spec.legacy_key).map(String::as_str),
        pair_value(&guard.dotenv, spec.new_key),
        pair_value(&guard.dotenv, spec.legacy_key),
    )
}

/// Remember a value for the rest of this process. Blank is stored and then
/// treated as unset by the resolver, same as a blank environment variable.
pub fn set_setting(name: &str, value: &str) {
    if settings::by_name(name).is_none() {
        return;
    }
    let mut guard = store().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    guard.runtime.insert(name.to_string(), value.to_string());
}

fn store() -> &'static Mutex<Store> {
    STORE.get_or_init(|| Mutex::new(read_store()))
}

fn read_store() -> Store {
    let mut real = HashMap::new();
    let real_keys: HashSet<String> = std::env::vars().map(|(key, _)| key).collect();
    for spec in settings::SETTINGS {
        if let Ok(value) = std::env::var(spec.new_key) {
            real.insert(spec.new_key.to_string(), value);
        }
        if let Ok(value) = std::env::var(spec.legacy_key) {
            real.insert(spec.legacy_key.to_string(), value);
        }
    }

    // Same two locations as before: beside the executable, then the config
    // folder (on Windows that is the data folder). The later file wins.
    let mut files = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            files.push(dir.join(".env"));
        }
    }
    files.push(paths::env_file());

    let mut dotenv = Vec::new();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        let mut setting_pairs = Vec::new();
        for (key, value) in merge_dotenv(&[], &text) {
            if settings::is_setting_env_key(&key) {
                setting_pairs.push((key, value));
            } else if !real_keys.contains(&key) {
                // Unknown lines still enter the process environment, and a later
                // file still overrides an earlier one. Setting keys do not.
                std::env::set_var(&key, &value);
            }
        }
        dotenv = overlay_pairs(&dotenv, &setting_pairs);
    }

    Store { real, dotenv, runtime: HashMap::new() }
}

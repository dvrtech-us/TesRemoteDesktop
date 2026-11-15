//! Setting names, the four-level read, and the `.env` save rewrite.
//! The process store that calls these lives in `crate::settings`.

/// Prefix written by this build. The only prefix a save produces.
pub const NEW_PREFIX: &str = "TESREMOTE_";

/// Prefix accepted from an older install. The compatibility table below is the
/// only place this prefix is spelled out.
pub const LEGACY_PREFIX: &str = "TESLAPC_";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Setting {
    /// Short name passed to `crate::settings::setting`, for example `HTTPS_HOST`.
    pub name: &'static str,
    pub new_key: &'static str,
    pub legacy_key: &'static str,
}

/// Every setting the server reads. Order is the order saves and tests walk.
pub const SETTINGS: &[Setting] = &[
    Setting { name: "HTTPS_HOST", new_key: "TESREMOTE_HTTPS_HOST", legacy_key: "TESLAPC_HTTPS_HOST" },
    Setting { name: "CF_TOKEN", new_key: "TESREMOTE_CF_TOKEN", legacy_key: "TESLAPC_CF_TOKEN" },
    Setting { name: "ACME_EMAIL", new_key: "TESREMOTE_ACME_EMAIL", legacy_key: "TESLAPC_ACME_EMAIL" },
    Setting { name: "ACME_STAGING", new_key: "TESREMOTE_ACME_STAGING", legacy_key: "TESLAPC_ACME_STAGING" },
    Setting { name: "VIDEO_ROOT", new_key: "TESREMOTE_VIDEO_ROOT", legacy_key: "TESLAPC_VIDEO_ROOT" },
    Setting { name: "STREAM_HEIGHT", new_key: "TESREMOTE_STREAM_HEIGHT", legacy_key: "TESLAPC_STREAM_HEIGHT" },
    Setting { name: "INPUT_MODE", new_key: "TESREMOTE_INPUT_MODE", legacy_key: "TESLAPC_INPUT_MODE" },
    Setting { name: "LOG_LEVEL", new_key: "TESREMOTE_LOG_LEVEL", legacy_key: "TESLAPC_LOG_LEVEL" },
    Setting { name: "FFMPEG", new_key: "TESREMOTE_FFMPEG", legacy_key: "TESLAPC_FFMPEG" },
    Setting { name: "NO_WGC", new_key: "TESREMOTE_NO_WGC", legacy_key: "TESLAPC_NO_WGC" },
    Setting { name: "BYPASS_IFACE", new_key: "TESREMOTE_BYPASS_IFACE", legacy_key: "TESLAPC_BYPASS_IFACE" },
];

pub fn by_name(name: &str) -> Option<&'static Setting> {
    SETTINGS.iter().find(|setting| setting.name == name)
}

pub fn by_new_key(key: &str) -> Option<&'static Setting> {
    SETTINGS.iter().find(|setting| setting.new_key == key)
}

pub fn is_setting_env_key(key: &str) -> bool {
    SETTINGS.iter().any(|setting| setting.new_key == key || setting.legacy_key == key)
}

/// A blank or whitespace-only value is unset. Any other value is returned unchanged.
pub fn nonempty(value: Option<&str>) -> Option<&str> {
    let value = value?;
    if value.trim().is_empty() { None } else { Some(value) }
}

/// Read one setting.
/// `runtime` is a value this process wrote (`POST /config`, Fit screen, `--https-host`).
/// It is checked first so a live change replaces the environment for the rest of the run,
/// which is what writing the process environment used to do.
/// Then: real environment `TESREMOTE_`, real environment `TESLAPC_`,
/// `.env` `TESREMOTE_`, `.env` `TESLAPC_`.
pub fn resolve_setting(
    runtime: Option<&str>,
    real_new: Option<&str>,
    real_old: Option<&str>,
    dotenv_new: Option<&str>,
    dotenv_old: Option<&str>,
) -> Option<String> {
    for candidate in [runtime, real_new, real_old, dotenv_new, dotenv_old] {
        if let Some(value) = nonempty(candidate) {
            return Some(value.to_string());
        }
    }
    None
}

/// Fold two `.env` parses. Entries in `over` replace the same key in `base`.
/// The executable `.env` is `base`; the data-folder `.env` is `over`.
pub fn overlay_pairs(base: &[(String, String)], over: &[(String, String)]) -> Vec<(String, String)> {
    let mut out = base.to_vec();
    for (key, value) in over {
        if let Some(slot) = out.iter_mut().find(|(existing, _)| existing == key) {
            slot.1 = value.clone();
        } else {
            out.push((key.clone(), value.clone()));
        }
    }
    out
}

pub fn pair_value<'a>(pairs: &'a [(String, String)], key: &str) -> Option<&'a str> {
    pairs.iter().find(|(existing, _)| existing == key).map(|(_, value)| value.as_str())
}

/// Write `updates` (`TESREMOTE_` keys) and drop only the `remove_keys` lines
/// (the `TESLAPC_` line for each setting this save actually wrote).
/// Comments, blanks, unknown keys, and every other line stay. A trailing newline
/// is kept when the file is not empty, matching the previous writer.
pub fn rewrite_env(existing: &str, updates: &[(String, String)], remove_keys: &[&str]) -> String {
    let mut lines = split_env_lines(existing);
    if !remove_keys.is_empty() {
        lines.retain(|line| match env_key(line) {
            Some(key) => !remove_keys.contains(&key),
            None => true,
        });
    }
    for (key, value) in updates {
        let line = format!("{key}={value}");
        if let Some(idx) = lines.iter().position(|line| env_key(line) == Some(key.as_str())) {
            lines[idx] = line;
        } else {
            lines.push(line);
        }
    }
    if lines.is_empty() {
        String::new()
    } else {
        lines.join("\n") + "\n"
    }
}

pub(crate) fn split_env_lines(existing: &str) -> Vec<String> {
    if existing.is_empty() {
        return Vec::new();
    }
    let mut lines: Vec<String> = existing
        .split('\n')
        .map(|line| line.trim_end_matches('\r').to_string())
        .collect();
    if existing.ends_with('\n') || existing.ends_with("\r\n") {
        if lines.last().is_some_and(|line| line.is_empty()) {
            lines.pop();
        }
    }
    lines
}

pub(crate) fn env_key(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    if trimmed.starts_with('#') || trimmed.is_empty() {
        return None;
    }
    let (key, _) = trimmed.split_once('=')?;
    let key = key.trim();
    if key.is_empty() { None } else { Some(key) }
}

pub(crate) fn split_env_kv(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    let (key, value) = trimmed.split_once('=')?;
    let key = key.trim();
    if key.is_empty() {
        return None;
    }
    let value = value.trim().trim_matches('"').to_string();
    Some((key.to_string(), value))
}

#[cfg(test)]
mod env_value_tests {
    use super::split_env_kv;

    #[test]
    fn splits_quoted() {
        let (key, value) = split_env_kv("TESREMOTE_CF_TOKEN=\"abc\"").unwrap();
        assert_eq!(key, "TESREMOTE_CF_TOKEN");
        assert_eq!(value, "abc");
    }
}

# Plan: rename the project to TesRemoteDesktop

**Date:** 2026-11-15
**Status:** Revised after two adversarial reviews (Claude Opus, Codex). Implemented on 2026-11-15 in the working tree (not committed).
The "Revisions after review" section at the end replaces the "Migration" section and the settings prefix above.

## Owner decision

The project is renamed **TesRemoteDesktop** so its own name no longer contains the car
maker's trademark. The GitHub repository is already `dvrtech-us/TesRemoteDesktop`, and the
Rust rewrite is merged to `main` (`93f3b7b`). The code, binary, service, data folder,
settings, pages, and docs still say TeslaPC.

## Names

| Thing | Old | New |
|-------|-----|-----|
| Product name (pages, README, logs) | `TeslaPC` | `TesRemoteDesktop` |
| Crate, binary, crate folder | `teslapc`, `teslapc.exe`, `teslapc/` | `tesremotedesktop`, `tesremotedesktop.exe`, `tesremotedesktop/` |
| Windows service name and display name | `TeslaPC` | `TesRemoteDesktop` |
| Windows data folder | `%ProgramData%\TeslaPC` | `%ProgramData%\TesRemoteDesktop` |
| Linux config / data folders | `~/.config/teslapc`, `~/.local/share/teslapc` | `~/.config/tesremotedesktop`, `~/.local/share/tesremotedesktop` |
| Files in the data folder | `teslapc.db`, `teslapc.log`, `teslapc-service.log` | `tesremotedesktop.db`, `tesremotedesktop.log`, `tesremotedesktop-service.log` |
| Settings prefix | `TESLAPC_` | `TRD_` |
| Flag | `--no-tesla-bypass` | `--no-bypass` |
| Log tag | `[TeslaBrowserBypass]` | `[Bypass]` |
| Page script names | `startTeslaAudio`, `stopTeslaAudio` | `startRemoteAudio`, `stopRemoteAudio` |
| Logo asset | `teslapc.png` | `logo.png` (same image) |
| Docs feature folder | `infrastructure/tesla-browser-bypass` | `infrastructure/car-browser-bypass` |
| systemd unit, uinput device name, local cert name | `teslapc` / `TeslaPC` | `tesremotedesktop` / `TesRemoteDesktop` |
| Version | `1.1.2` | `1.2.0` |

## The word "Tesla" after the rename

- Removed from every identifier, file name, setting, flag, log tag, and from the product's
  own name everywhere.
- Kept only as a plain description of compatibility: the README ("works in the Tesla in-car
  browser") with the existing not-affiliated notice, and feature docs where the behavior
  being described is that browser's (it refuses private addresses). Docs say "the car's
  browser" where the maker does not matter.
- **Existing trail entries and dated planning documents are not edited** (AGENTS.md: trail
  entries are append-only history). They keep the old name. One new trail entry per touched
  feature folder records the rename.

## Migration (an installed machine keeps working)

Windows, at start of `--service`, `--session-host`, a normal run, and `service install`:

1. **Data folder.** If `%ProgramData%\TesRemoteDesktop` does not exist and
   `%ProgramData%\TeslaPC` does, copy its files to the new folder (`.env`, `acme-account.pem`,
   certificate and key files, `teslapc.db` as `tesremotedesktop.db`). Logs are not copied.
   The old folder is left in place and a line is logged. Copy, not move, so a failed first
   start can fall back to the old build.
2. **Settings.** Every setting is read as `TRD_<NAME>`, falling back to `TESLAPC_<NAME>`
   when the new name is absent, from both the real environment and `.env`.
   `apply_config_body` writes `TRD_` names; when it writes a key it removes the
   `TESLAPC_` line for the same setting. Unknown lines are kept as today.
3. **Service.** `tesremotedesktop service install` creates `TesRemoteDesktop`. If a service
   named `TeslaPC` exists, install stops it, deletes it, and says so. `service uninstall`
   removes `TesRemoteDesktop` only.
4. **Flag.** `--no-tesla-bypass` is still accepted as an alias of `--no-bypass`.
5. **Scheduled-task warning** matches both `teslapc` and `tesremotedesktop`.

Linux: the same folder copy and settings fallback; `service install` writes
`tesremotedesktop.service` and prints the `systemctl disable --now teslapc` command if the
old unit file exists. Nothing is deleted.

Browser: pages are served `no-cache`, so the car loads the renamed pages on next visit.
The hostname and port do not change; bookmarks keep working.

## Order of work

1. `git mv teslapc tesremotedesktop`, `Cargo.toml` package name and version, `Cargo.lock`,
   `build.ps1`, `.github/workflows/ci.yml`, `.vscode`.
2. Source: constants and strings; settings fallback and migration as pure functions in
   `decisions/` with tests (key mapping, which old lines to drop on save, which files to
   copy and under what names, old-service handling decision).
3. Assets: page titles and text, script names, logo file name.
4. Docs: README, AGENTS.md, CLAUDE.md, `documentation/README.md`, feature docs and
   baselines, docs folder rename, new trail entries.
5. CI green on Windows and Linux; cross-build.
6. Lab laptop: install the new service (which removes the old one), confirm the data copy,
   the certificate is reused (no new issuance), settings read through the fallback, then
   save a setting and confirm the `.env` line is rewritten with the new prefix. Check
   stream, audio, input, hotspot and bypass, lock screen.
7. Pull request.

## Risks

| Risk | Handling |
|------|----------|
| Certificate re-issued on every machine because the cert files were not carried over | Copy step includes cert, key, and ACME account; verified on the laptop by the log line "valid for N more days; no action" |
| Old and new service both installed and fighting for ports | Install removes `TeslaPC`; the supervisor already logs "port in use" distinctly |
| A setting silently lost because only the old name is set | Fallback read; test for every key |
| Docs links broken by the folder rename | grep for the old path after the move |
| Third parties or scripts calling `teslapc.exe` | Out of scope; noted in the README upgrade section |
| The name still begins with "Tes" | Owner's decision |

## Not in this change

- The local checkout folder name and the hostname `my.thelpers.com`.
- Rewriting history, tags, or old branches.
- The Windows firewall rule name if it embeds the old name: the new build adds its own rule;
  the old rule is left.

## Revisions after review (2026-11-15)

Both reviewers confirmed: nothing string-replaces the product name in served pages, the
service names are the only SCM keys, `--no-tesla-bypass` is parsed in one place, and dated
history can stay unchanged.

### What was wrong in the draft

1. The ACME account file is `acme-account.json`, not `.pem`. The copy list comes from the
   `paths` functions: `.env`, `cert.pem`, `key.pem`, `acme-account.json`, and the database.
2. Six settings are read directly, outside `decisions/config.rs`: `TESLAPC_NO_WGC`,
   `TESLAPC_FFMPEG`, `TESLAPC_BYPASS_IFACE`, `TESLAPC_ACME_STAGING`, and literal
   `TESLAPC_CF_TOKEN` / `TESLAPC_HTTPS_HOST` in `certs.rs` and `state.rs`. Every read and
   every `set_var` goes through one settings helper.
3. Linux keeps `.env` and certificates in the config folder and the database in the data
   folder. Each pair is handled separately.
4. "New folder does not exist" is not a usable signal: the session host creates the folder
   for its log before anything else, and `service install` never runs the normal startup.
5. Copying the SQLite file while the old server runs can corrupt it.
6. Deleting a Windows service is deferred and does not prove it stopped.
7. Missed names: nftables table `ip teslapc`, firewall rule `TeslaPC Server`, the portal
   token `teslapc{pid}`, `tasks_running_teslapc`, about 40 `teslapc::` test imports,
   `.gitignore` `/teslapc/target/`, the CI Node test path. `.vscode` still targets the
   removed C# project and is deleted, not renamed.

### Migration, replaced: one explicit step instead of automatic copying

No copying happens at normal start. Migration is part of
`tesremotedesktop service install`, which already runs elevated and once:

1. If a `TeslaPC` service exists: clear its failure actions, stop it, and wait until it
   reports stopped and no `teslapc.exe` process remains (30 s limit). If it will not stop,
   install stops with an error and changes nothing.
2. With the old server stopped, copy each of the five files from the old folder to the new
   one **only where the new folder does not already have that file** (the database is
   copied under its new name). Each copy goes to a temporary name and is renamed into
   place. Every action is printed.
3. Create the `TesRemoteDesktop` service. Then delete the `TeslaPC` service.
4. Install does not start the new service (unchanged). The old binary, the old data
   folder, and the old `.env` are left untouched, so going back is: delete the new service
   and run the old `teslapc.exe service install`.

Linux `service install` does the same per-file copy for the config pair and the data pair,
writes `tesremotedesktop.service`, and prints the commands to disable the old unit. When
the bypass is applied it also deletes the old `ip teslapc` table.

A machine that never ran the old version does nothing extra.

### Settings

- Prefix is **`TESREMOTE_`** (not `TRD_`, which is short enough to collide).
- One helper `setting(name)` resolves in this order: real environment `TESREMOTE_<NAME>`,
  real environment `TESLAPC_<NAME>`, `.env` `TESREMOTE_<NAME>`, `.env` `TESLAPC_<NAME>`.
  A blank value counts as unset. Both `.env` locations (data folder and next to the
  executable) are read as today.
- Saving a setting writes the `TESREMOTE_` line and removes the `TESLAPC_` line for that
  setting in the data-folder `.env`. Other lines are untouched.
- Pure functions and tests: the name mapping for all eleven settings, the precedence, the
  save rewrite (including blank token retention), and a test that the old prefix appears in
  source only inside the compatibility table.

### Other names

Firewall rule `TesRemoteDesktop Server` (the old rule is left), nftables table
`ip tesremotedesktop`, portal token `tesremotedesktop{pid}`, scheduled-task scan matching
both executable names in the command column only.

### Verification additions (lab laptop)

Install over the running old service; confirm `sc query TeslaPC` reports it gone without a
reboot; the certificate file hashes are identical in both folders and the log says the
certificate is valid with no new order; playback history is present; run install a second
time (nothing copied, nothing broken); set a real `TESLAPC_` environment variable and
confirm it still wins over `.env`; save from the Settings page and read `.env`; uninstall
and reinstall; reboot. Linux is verified by CI only.

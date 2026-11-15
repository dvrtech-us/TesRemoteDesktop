# AGENTS.md

Operating guide for AI agents and developers working in the TesRemoteDesktop repository. Read this
before making changes. For a feature-by-feature reference, start at
[`documentation/README.md`](documentation/README.md).

## What This Project Is

TesRemoteDesktop is a Windows and Linux remote-desktop streaming server written in Rust. One binary,
`tesremotedesktop`, serves the browser UI, an MJPEG screen stream, system audio, and pointer/keyboard
input. There is no WinForms panel and no authentication. Logs go to stdout. The process can be
installed as a Windows service (`tesremotedesktop service install`) or a systemd unit of the same
command. On Windows the service (`--service`) is a supervisor that starts the server as
`--session-host` in the active console session, and that host follows the input desktop (sign-in, lock, and UAC screens); see
`documentation/features/infrastructure/windows-service/`.

## Documentation System (read before and after coding)

All feature documentation lives under `documentation/`. The structure is strict:

```
documentation/
  features/<module>/<feature-name>/
    <feature-name>.md   # what the feature does today
    baseline.md         # behavior invariants (regression reference)
    trail/YYYY-MM-DD-<title>.md   # append-only decision history
  planning/<Module>/YYYY-MM-DD-<title>.md
```

**The discipline — after any code change that affects a feature:**

1. **Always** update the feature's `<feature-name>.md` to match current behavior.
2. **If intended behavior changed**, update `baseline.md` invariants.
3. **If a non-trivial decision was made**, add a new `trail/` entry. **Never edit an existing trail entry** — append a new one.
4. Keep the Feature Map in `documentation/README.md` current when features are added or renamed.

**Quality rules:** name exact modules, functions, file paths, and constant values; document
timings, thresholds, and config keys; describe failure/fallback paths; document actual
behavior, not aspirations. SQLite (`rusqlite`) lives at `%ProgramData%\TesRemoteDesktop\tesremotedesktop.db` on
Windows and `$XDG_DATA_HOME/tesremotedesktop/tesremotedesktop.db` (default `~/.local/share/tesremotedesktop/tesremotedesktop.db`)
on Linux. There are still **no `sql/` folders** — the schema is created in `tesremotedesktop/src/store.rs`.

### Feature Map (quick index)

| Module | Feature | Primary source |
|--------|---------|----------------|
| server | web-server | `tesremotedesktop/src/serve.rs`, `tesremotedesktop/src/routes.rs` |
| server | input-control | `tesremotedesktop/src/decisions/input.rs`, `tesremotedesktop/src/win.rs`, `tesremotedesktop/src/linux.rs` |
| streaming | screen-capture | `tesremotedesktop/src/capture_loop.rs`, `tesremotedesktop/src/win_capture.rs`, `tesremotedesktop/src/linux.rs` |
| streaming | audio-capture | `tesremotedesktop/src/decisions/audio.rs`, `tesremotedesktop/src/win_capture.rs` |
| streaming | display-control | `tesremotedesktop/src/decisions/display.rs`, `tesremotedesktop/src/win.rs` |
| app | control-panel | `tesremotedesktop/src/serve.rs` (stdout log, no WinForms) |
| app | configuration | `tesremotedesktop/src/decisions/config.rs`, `tesremotedesktop/assets/config.html` |
| media | media-library | `tesremotedesktop/src/store.rs`, `tesremotedesktop/src/decisions/history.rs` |
| media | file-browser-vlc | `tesremotedesktop/src/decisions/listing.rs`, `tesremotedesktop/src/playback.rs` |
| client | web-ui | `tesremotedesktop/assets/index.html`, `tesremotedesktop/assets/PCMPlayerProcessor.js` |
| infrastructure | car-browser-bypass | `tesremotedesktop/src/decisions/bypass.rs`, `tesremotedesktop/src/win.rs`, `tesremotedesktop/src/linux.rs` |
| infrastructure | firewall-bootstrap | `tesremotedesktop/src/win.rs` |
| infrastructure | https-bootstrap | `tesremotedesktop/src/certs.rs` |
| infrastructure | windows-service | `tesremotedesktop/src/win_service.rs`, `tesremotedesktop/src/win_desktop.rs`, `tesremotedesktop/src/decisions/service.rs`, `tesremotedesktop/src/decisions/desktop.rs` |

## Source Layout

```
tesremotedesktop/
├── Cargo.toml
├── assets/                    # index.html, config.html, play.html, list.html, style.css, JS, logo.png
└── src/
    ├── main.rs                # process entry
    ├── serve.rs               # bind 8080/8443, flags, dotenv, service handoff
    ├── routes.rs              # HTTP and WebSocket routes
    ├── decisions/             # pure protocol decisions (tested)
    ├── capture_loop.rs        # shared MJPEG capture loop
    ├── encode.rs              # in-process JPEG quality 60
    ├── playback.rs            # ffmpeg, Windows VLC fallback
    ├── store.rs               # SQLite playback history
    ├── certs.rs               # rustls local cert and DNS-01 renewal
    ├── win.rs                 # Windows input, hotspot, firewall, GDI
    ├── win_service.rs         # Windows service supervisor, session host launch, install/uninstall
    ├── win_desktop.rs         # per-thread input-desktop attachment (session host)
    ├── win_capture.rs         # WGC, DXGI duplication, WASAPI loopback
    └── linux.rs               # Portal/PipeWire, XShm, uinput, nftables, systemd
```

## Build & Run

```bash
cargo build --manifest-path tesremotedesktop/Cargo.toml
cargo test --manifest-path tesremotedesktop/Cargo.toml
.\build.ps1
.\build.ps1 -Configuration Debug
```

The dev binary is `tesremotedesktop\target\debug\tesremotedesktop.exe`. Release is `tesremotedesktop\target\release\tesremotedesktop.exe`.

Runtime flags / env:

- `--localhost` — bind `127.0.0.1:8080` only. Skips the firewall, ACME, HTTPS, and the car-browser bypass.
- `--no-bypass` — skip the `100.64.0.1` address and the forward rules. Firewall and HTTPS still run. `--no-tesla-bypass` is accepted as the same flag.
- `--https-host <host>` (or `TESREMOTE_HTTPS_HOST`) — hostname for a Let's Encrypt certificate. With `TESREMOTE_CF_TOKEN`, issuance and renewal run in-process (DNS-01). Renewal runs when 30 days or fewer remain and is checked every 12 hours. `TESREMOTE_ACME_STAGING=1` uses the staging directory.
- `TESREMOTE_CF_TOKEN` — Cloudflare API token (Zone:Read + DNS:Edit).
- `TESREMOTE_ACME_EMAIL` — optional account contact.
- `TESREMOTE_VIDEO_ROOT` — file browser root. Default `C:\video\` on Windows.
- `TESREMOTE_STREAM_HEIGHT` — vertical cap, default `1080`, allowed `240`–`2160`. Width cap is `height * 4`.
- `TESREMOTE_INPUT_MODE` — `touch` (default) or `mouse`.
- `TESREMOTE_LOG_LEVEL` — `error`, `warn`, `info` (default), or `debug`.
- `TESREMOTE_NO_WGC=1` — skip Windows Graphics Capture and start at DXGI.
- `TESREMOTE_FFMPEG` — ffmpeg executable or directory.
- `TESREMOTE_BYPASS_IFACE` — Linux only. Empty leaves the bypass off. A set iface adds `100.64.0.1/10` and nftables forward plus NAT. A `TESLAPC_` value for any setting is used when the `TESREMOTE_` value is unset. A blank value counts as unset. Saving on the Settings page rewrites that setting to `TESREMOTE_` and drops its `TESLAPC_` line.
- `tesremotedesktop service install` — Windows: if service `TeslaPC` exists, stop it (30 s, and `teslapc.exe` must be gone) and copy `.env`, the certificate, the key, the ACME account, and the database into `%ProgramData%\TesRemoteDesktop` when the new file is absent; then create or update service `TesRemoteDesktop` (`"<exe>" --service`, auto start, LocalSystem, restart after 5 s) and delete `TeslaPC`. Install does not start the service. It warns about scheduled tasks whose "Task To Run" column runs `teslapc.exe` or `tesremotedesktop.exe`. Linux: the same per-file copy, then `tesremotedesktop.service`; if `teslapc.service` exists, print `systemctl disable --now teslapc` and do not run it. A denied install is logged, exits non-zero, and the server is not required to be registered.
- `tesremotedesktop service uninstall` — Windows: stop (up to 10 s) and delete `TesRemoteDesktop` only. Other words after `service` print usage and exit `2`.
- `--service` — Windows, started by the SCM: supervisor only (no ports, capture, or audio). It keeps one `--session-host` running as SYSTEM in the active console session, restarting it on exit (backoff 1 s doubling to 30 s, reset after 60 s uptime; 30 s when the host exits `3` = port in use), console session change, or logoff. Log: `%ProgramData%\TesRemoteDesktop\tesremotedesktop-service.log`. Not started by the SCM (or on Linux): runs the normal server.
- `--session-host [--supervisor-pid <pid>]` — the normal server, plus: stdout/stderr go to `%ProgramData%\TesRemoteDesktop\tesremotedesktop.log` (rotated to `.1` over 5 MiB), capture starts at DXGI, the capture thread and a dedicated `Input` thread follow the input desktop, it exits when the supervisor pid exits, and it exits `3` if port 8080 cannot be bound. Started by the supervisor; not meant to be run by hand.

HTTPS on `8443` is terminated in-process with rustls. There is no http.sys URL ACL and no certificate-store bind. Without a public hostname, a generated local certificate still makes `8443` listen (unless `--localhost`).

Certificates and `.env` live under `%ProgramData%\TesRemoteDesktop\` on Windows and `$XDG_CONFIG_HOME/tesremotedesktop/` (default `~/.config/tesremotedesktop/`) on Linux.

A missing privilege for firewall, hotspot, input, address, or nftables is logged. HTTP still starts.

## Conventions

- Rust 2021. Protocol decisions are pure functions in `tesremotedesktop/src/decisions` and have tests in `tesremotedesktop/tests/protocol.rs`.
- Windows and Linux only. No macOS. No car-vendor Android source.
- The browser UI is the vanilla JS already in `tesremotedesktop/assets`. PNG and other binaries are served as raw bytes.
- Ports `8080` and `8443` are fixed in `serve.rs`.
- `cargo test` covers the protocol decisions. Capture, hotspot, and ACME also depend on the host.

## Versioning

- `tesremotedesktop/Cargo.toml` `version` is the string returned by `GET /version` and the `version` field of `GET /config`.
- Tag a release `v<version>`.

## Current Known Gaps

- No authentication on any route.
- A black or failed capture frame still advances WGC → DXGI → GDI. The first backend is tried again after 30 s (doubling to 5 min while it keeps falling back). DRM video can stay black under every backend.
- Linux PipeWire, uinput, and nftables are not executed on a Windows host. The event records, portal messages, and bypass plan are covered by `cargo test`.
- The SQLite file is created in code. There is no `sql/` folder.
- The Windows service host follows the input desktop (`win_desktop.rs`): its capture thread and its single `Input` thread re-attach to `Winlogon` for the sign-in screen, lock screen, and UAC prompts, and back to `Default`. Off `Default` pointer events are mouse events. Ctrl+Alt+Del is not sent. Desktop following was written and compiled on macOS only and has not yet run on Windows; still to confirm there: DXGI duplication on `Winlogon` as SYSTEM (else GDI after five retries), whether the capture thread can call `SetThreadDesktop` without `ERROR_BUSY` (restarts are logged), that `SendInput` reaches the sign-in and UAC screens, and touch on `Default` after a trip to `Winlogon`.

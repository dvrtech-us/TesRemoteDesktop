# TesRemoteDesktop Documentation

Structured documentation for the TesRemoteDesktop remote-desktop streaming application. See the
root [`AGENTS.md`](../AGENTS.md) for the documentation conventions every change must follow.

## Documentation System

Each feature has its own directory under `features/<module>/<feature-name>/` containing:

- `<feature-name>.md` — what the feature does **today** (user + technical flow, classes, routes).
- `baseline.md` — behavior invariants (the known-good reference for regression checks).
- `trail/` — dated decision history; entries are append-only and never edited after creation.

`planning/` holds dated planning docs, audits, and proposals per module.

## Feature Map

| Module | Feature | Summary | Primary source |
|--------|---------|---------|----------------|
| server | [web-server](features/server/web-server/web-server.md) | Rust HTTP/HTTPS server, path routing, no WinForms | `tesremotedesktop/src/serve.rs`, `tesremotedesktop/src/routes.rs` |
| server | [input-control](features/server/input-control/input-control.md) | Touch-default pointer, wheel, and SendKeys mapping over any non-audio WebSocket | `tesremotedesktop/src/decisions/input.rs`, `tesremotedesktop/src/win.rs` |
| streaming | [screen-capture](features/streaming/screen-capture/screen-capture.md) | Shared MJPEG stream; WGC (skipped by the service session host), then DXGI, then GDI; JPEG quality 60 | `tesremotedesktop/src/capture_loop.rs`, `tesremotedesktop/src/win_capture.rs`, `tesremotedesktop/src/linux.rs` |
| streaming | [audio-capture](features/streaming/audio-capture/audio-capture.md) | Format text frame then PCM; WASAPI or Pulse monitor; silence keepalive 250 ms / 30 s / 20 ms | `tesremotedesktop/src/decisions/audio.rs`, `tesremotedesktop/src/win_capture.rs` |
| streaming | [display-control](features/streaming/display-control/display-control.md) | `/display/info`, `/display/match`, `/display/set` | `tesremotedesktop/src/decisions/display.rs`, `tesremotedesktop/src/win.rs` |
| app | [control-panel](features/app/control-panel/control-panel.md) | No WinForms. Stdout logs (host and supervisor log files under the Windows service) and a Windows service / systemd unit | `tesremotedesktop/src/serve.rs` |
| app | [configuration](features/app/configuration/configuration.md) | Settings page and `GET`/`POST /config`. Token is never returned | `tesremotedesktop/src/decisions/config.rs`, `tesremotedesktop/assets/config.html` |
| client | [web-ui](features/client/web-ui/web-ui.md) | Browser UI embedded in the binary and served as bytes | `tesremotedesktop/assets/index.html`, `tesremotedesktop/assets/PCMPlayerProcessor.js` |
| media | [file-browser-vlc](features/media/file-browser-vlc/file-browser-vlc.md) | Generated `/list.html`, ffmpeg playback, Windows VLC fallback | `tesremotedesktop/src/decisions/listing.rs`, `tesremotedesktop/src/playback.rs` |
| media | [media-library](features/media/media-library/media-library.md) | SQLite resume and sticky watched flag at 95% | `tesremotedesktop/src/store.rs` |
| infrastructure | [car-browser-bypass](features/infrastructure/car-browser-bypass/car-browser-bypass.md) | Windows hotspot + portproxy, or Linux nftables when `TESREMOTE_BYPASS_IFACE` is set | `tesremotedesktop/src/decisions/bypass.rs` |
| infrastructure | [firewall-bootstrap](features/infrastructure/firewall-bootstrap/firewall-bootstrap.md) | Windows inbound rule for TCP 8080 and 8443 | `tesremotedesktop/src/win.rs` `open_firewall` |
| infrastructure | [windows-service](features/infrastructure/windows-service/windows-service.md) | `service install`/`uninstall`; `--service` supervisor in session 0 starts `--session-host` in the console session, which follows the input desktop (sign-in, lock, UAC); two log files | `tesremotedesktop/src/win_service.rs`, `tesremotedesktop/src/win_desktop.rs`, `tesremotedesktop/src/decisions/service.rs`, `tesremotedesktop/src/decisions/desktop.rs` |
| infrastructure | [https-bootstrap](features/infrastructure/https-bootstrap/https-bootstrap.md) | In-process rustls, local certificate, DNS-01 renewal | `tesremotedesktop/src/certs.rs` |

## Conventions

- Document **actual behavior as implemented**, not desired future behavior.
- Name exact classes, methods, file paths, and constant values — no vague references.
- Update the primary doc on every behavior-affecting change; update `baseline.md` only when intended behavior changes; add a `trail/` entry for any non-trivial decision.
- The project uses a small SQLite database. On Windows it is `%ProgramData%\TesRemoteDesktop\tesremotedesktop.db`.
  On Linux it is `$XDG_DATA_HOME/tesremotedesktop/tesremotedesktop.db` (default `~/.local/share/tesremotedesktop/tesremotedesktop.db`).
  There are still **no `sql/` folders** — `store.rs` runs `CREATE TABLE IF NOT EXISTS`.

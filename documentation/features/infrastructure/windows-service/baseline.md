# Windows Service — Behavior Baseline

- `tesremotedesktop service install` creates or updates service `TesRemoteDesktop` through the SCM API: own process, auto start, LocalSystem, `ImagePath` = quoted-if-needed exe + ` --service` with no leading space. Failure actions restart after 5 s. Not elevated → error text, exit `1`.
- Before that create, a previous `TeslaPC` service is stopped (failure actions cleared first) and waited on until it reports stopped and `teslapc.exe` is gone, at most 30 s. If that wait fails, nothing is copied and `TesRemoteDesktop` is not created. Files (`.env`, `cert.pem`, `key.pem`, `acme-account.json`, `teslapc.db` → `tesremotedesktop.db`) are copied from `%ProgramData%\TeslaPC` only when the new file is absent, via a temporary name in the new folder. The old folder is not modified. The old service is deleted only after `TesRemoteDesktop` exists. Install does not start the service.
- `install` never changes scheduled tasks. It warns once, naming every task whose "Task To Run" column contains `teslapc.exe` or `tesremotedesktop.exe`.
- `tesremotedesktop service uninstall` stops (up to 10 s), then deletes `TesRemoteDesktop` only. Not installed → exit `0`.
- `tesremotedesktop service <anything else>` prints usage and exits `2`.
- `--service` started by the SCM binds no port and runs no capture, audio, or input. Not started by the SCM → logs and runs the normal server.
- The supervisor keeps exactly one host, started as SYSTEM in `WTSGetActiveConsoleSessionId()` on `winsta0\default` with `"<exe>" --session-host --supervisor-pid <pid>`.
- Console id `0xFFFFFFFF` keeps the current host. A different console id, or a logoff for the host's session (even with the same id), terminates the host, waits up to 5 s, and starts one in the current console session.
- Host restart delay: 1, 2, 4, 8, 16, 30, 30 ... s; a run of 60 s or more resets it to 1 s. Exit code `3` (port in use) waits 30 s and is logged as "Port in use".
- STOP/SHUTDOWN: no further launch, `StopPending`, host terminated (5 s bound), `Stopped`. STOP during a backoff wait is handled within one wake (at most 2 s).
- Each host is in its own kill-on-close job; closing it ends the host and its children. If the job cannot be set up, the host exits when the supervisor pid exits.
- `--session-host` writes stdout and stderr to `%ProgramData%\TesRemoteDesktop\tesremotedesktop.log`, starts capture at DXGI, and exits `3` when 8080 cannot be bound.
- Supervisor lines go to `%ProgramData%\TesRemoteDesktop\tesremotedesktop-service.log`. Both logs rotate to `.1` at open when over 5 MiB.
- `--session-host` only: the capture thread and the one `Input` thread follow the input desktop (`OpenInputDesktop(MAXIMUM_ALLOWED)` + `SetThreadDesktop` when the `UOI_NAME` differs, case-insensitive). A check runs at most every 200 ms per thread, at once after a failed grab or injection, and not within 2 s of a failed `SetThreadDesktop`. A failed `OpenInputDesktop` keeps the current desktop. Each switch prints `[Desktop] <role> attached to <name>` once.
- The handle from `GetThreadDesktop` is never closed. Every handle a thread opened is closed when it is not switched to, after the thread leaves it, or (capture restart) by the replacement thread.
- A normal run (no role flag) never opens or switches desktops.

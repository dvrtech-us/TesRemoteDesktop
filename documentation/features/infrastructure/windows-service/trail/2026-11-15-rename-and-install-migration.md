# Rename and install-time migration

- Date: 2026-11-15
- Feature: windows-service

## Problem

The Windows service, its data folder, its log files, and the scheduled-task warning used the previous product name. An upgrade had to replace that service without copying a live database and without deleting the old files.

## Decision

The service name, display name, and description are `TesRemoteDesktop`. Logs are `tesremotedesktop.log` and `tesremotedesktop-service.log` under `%ProgramData%\TesRemoteDesktop`. `service install` stops a previous `TeslaPC` service (failure actions cleared first), waits until it reports stopped and `teslapc.exe` is gone (30 s), copies `.env`, the certificate, the key, the ACME account, and the database only when the new file is absent, creates `TesRemoteDesktop`, then deletes `TeslaPC`. Install does not start the service. A failed stop or a failed copy changes nothing further. `service uninstall` removes only `TesRemoteDesktop`. The scheduled-task warning matches `teslapc.exe` and `tesremotedesktop.exe` in the "Task To Run" column only.

## Files

- `tesremotedesktop/src/win_service.rs`
- `tesremotedesktop/src/decisions/service.rs`
- `tesremotedesktop/src/decisions/legacy.rs`
- `tesremotedesktop/src/decisions/migrate.rs`
- `tesremotedesktop/src/migrate.rs`
- `tesremotedesktop/src/paths.rs`

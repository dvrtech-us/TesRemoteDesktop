# Media Library

`Library::open` in `tesremotedesktop/src/store.rs` creates `playback`:

| Column | Type |
|--------|------|
| `path` | TEXT PRIMARY KEY COLLATE NOCASE |
| `position` | REAL NOT NULL DEFAULT 0 |
| `duration` | REAL NOT NULL DEFAULT 0 |
| `watched` | INTEGER NOT NULL DEFAULT 0 |
| `play_count` | INTEGER NOT NULL DEFAULT 0 |
| `updated_at` | TEXT |

Windows file: `%ProgramData%\TesRemoteDesktop\tesremotedesktop.db`. Linux file: `$XDG_DATA_HOME/tesremotedesktop/tesremotedesktop.db`, default `~/.local/share/tesremotedesktop/tesremotedesktop.db`.

`watched_now` is sticky. It becomes true when `duration > 0` and `position / duration >= 0.95` (`WATCHED_FRACTION`). `save_progress` also stores `MAX(watched, new)` so a later shorter position cannot clear it. End of file calls `mark_watched`.

There is no `sql/` folder. The `CREATE TABLE` runs in `Library::open`.

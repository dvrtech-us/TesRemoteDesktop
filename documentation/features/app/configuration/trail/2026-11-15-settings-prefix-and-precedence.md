# Settings prefix and four-level read

- Date: 2026-11-15
- Feature: configuration

## Problem

Every setting used the `TESLAPC_` prefix, and `.env` values were copied into the process environment, so a newer name in `.env` could not sit behind an older name in the real environment.

## Decision

The prefix written by this build is `TESREMOTE_`. `settings::setting` is the only read. Order: a value written during this process, then real `TESREMOTE_`, then real `TESLAPC_`, then `.env` `TESREMOTE_`, then `.env` `TESLAPC_`. A blank value is unset. Setting keys stay out of the process environment, so `POST /config`, Fit screen, and `--https-host` still replace the value for the rest of the run through the runtime map. Saving a setting writes the `TESREMOTE_` line and removes the `TESLAPC_` line for that setting only. A blank token is not written and does not remove a legacy token line. `GET /config` still never returns the token.

## Files

- `tesremotedesktop/src/settings.rs`
- `tesremotedesktop/src/decisions/settings.rs`
- `tesremotedesktop/src/decisions/config.rs`
- `tesremotedesktop/src/state.rs`
- `tesremotedesktop/src/routes.rs`
- `tesremotedesktop/src/serve.rs`
- `tesremotedesktop/assets/config.html`

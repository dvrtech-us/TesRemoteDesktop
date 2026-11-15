# Bypass names

- Date: 2026-11-15
- Feature: car-browser-bypass

## Problem

The bypass log tag, the flag, the nftables table, the firewall rule, the portal session token, and the uinput device name all carried the previous product name.

## Decision

The flag is `--no-bypass`. `--no-tesla-bypass` is still accepted. Log lines use `[Bypass]`. The nftables table is `ip tesremotedesktop`. After that table is applied, `nft delete table ip teslapc` runs and a missing table is ignored. The Windows firewall rule this build adds is `TesRemoteDesktop Server`; the previous rule is left in place. The portal session token starts with `tesremotedesktop`. The uinput device name is `TesRemoteDesktop`. Linux `service install` prints `systemctl disable --now teslapc` when the old unit file exists and does not run it.

## Files

- `tesremotedesktop/src/decisions/bypass.rs`
- `tesremotedesktop/src/decisions/legacy.rs`
- `tesremotedesktop/src/win.rs`
- `tesremotedesktop/src/linux.rs`
- `tesremotedesktop/src/serve.rs`

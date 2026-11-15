# Car Browser Bypass — Behavior Baseline

- Bypass address is `100.64.0.1`.
- `--no-bypass` leaves Windows and Linux bypass off. `--no-tesla-bypass` is the same flag.
- An empty Linux iface leaves the bypass off and produces no nftables script.
- A set Linux iface produces `100.64.0.1/10`, table `ip tesremotedesktop`, DNAT for `8080` and `8443`, a forward chain, and `masquerade`.
- After that script succeeds, `nft delete table ip teslapc` runs. "no such" is ignored. A failed new script does not delete the old table.
- Windows `TURNEDON` waits `2000` ms, then setup runs. Settings `ALREADYON` with `100.64.0.1` missing waits `1500` ms, then setup runs. The watchdog does that setup immediately. An address that is already present skips setup. Watchdog period is `60` s.
- Manual off from `POST /hotspot` is not turned back on by the watchdog.
- `--localhost` does not start the bypass.
- The hotspot adapter is always addressed by its full interface name (`Local Area Connection* 2`), never a bare number or index. `netsh` gets `name=`, `address=`, and `mask=` explicitly, so setup cannot change another adapter such as Wi-Fi.

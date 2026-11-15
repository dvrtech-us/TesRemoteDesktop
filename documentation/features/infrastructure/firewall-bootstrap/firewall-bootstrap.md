# Firewall

`win::open_firewall` deletes and re-adds `netsh advfirewall firewall` rule `name=TesRemoteDesktop Server` for inbound TCP `8080` and `8443`, all profiles. A rule left by an older build under a different name is not removed. It runs only when the process is not `--localhost`. A non-admin failure is printed. HTTP still starts.

Linux does not add a firewall rule. The operator's firewall is unchanged.

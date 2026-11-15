# TesRemoteDesktop

Turn an old laptop into a full desktop you can use from a Tesla's built-in browser, parked or
in motion.

The laptop rides along in the car and runs one small program. The car connects to the
laptop's Wi-Fi hotspot, you open a web page in the car's browser, and the laptop's screen,
sound, touch, and keyboard are on the car's display. Anything the laptop can run, the car can
show: a full desktop browser, streaming sites, your own video files, documents, a terminal.

There is nothing to install in the car and no app to pair. It is a web page.

> **Passengers only while the car is moving.** A screen showing video or a desktop takes the
> driver's attention off the road, and watching one while driving is illegal in many places.
> The driver drives. TesRemoteDesktop is for the people in the other seats, and for the driver when
> parked.

## How it works

```
 laptop (Windows or Linux)                          car
┌──────────────────────────────┐   Wi-Fi hotspot  ┌───────────────────────┐
│ TesRemoteDesktop server      │◄────────────────►│ built-in web browser  │
│  screen  -> MJPEG video      │                  │  https://your.host    │
│  sound   -> PCM audio        │                  │        :8443          │
│  input   <- touch, keys      │                  └───────────────────────┘
│ Mobile Hotspot + 100.64.0.1  │
└──────────────┬───────────────┘
               │ laptop's own internet (phone hotspot, Starlink, home Wi-Fi)
               ▼
            internet
```

- **It is remote desktop in a web page.** This is not Microsoft's RDP protocol and needs no
  RDP client. The server sends the screen as a stream of JPEG images and the sound as raw
  audio over a WebSocket; the page sends touches and key presses back.
- **Why it works while driving.** The page is ordinary images and Web Audio, so it runs
  wherever the car's browser runs.
- **Why the odd address.** The car's browser refuses private network addresses such as
  `192.168.x.x`. TesRemoteDesktop turns on the laptop's Mobile Hotspot, gives the hotspot adapter the
  address `100.64.0.1`, and forwards ports `8080` and `8443` to itself.
- **Why HTTPS.** The car's browser treats the page like any other website. TesRemoteDesktop can get
  and renew a real Let's Encrypt certificate for your own hostname by itself (DNS-01 through
  Cloudflare), so the car shows no certificate warning.

## What you get

- The laptop's live screen at up to 1080p, with system audio.
- Touch (tap, drag, long-press for right-click) or mouse mode, an on-screen keyboard field,
  and scrolling.
- The Windows lock screen, sign-in screen, and elevation prompts, when run as a service.
- **Fit screen:** one tap sets the laptop's display mode and the stream size to suit the car's
  screen.
- A file browser and video player for a folder of your own videos, with resume and "watched"
  marks.
- A Settings page: stream resolution, input mode, hotspot on/off, video folder, HTTPS host.
- Starts with Windows as a service. With auto sign-in on the laptop, it is ready by the time
  the car has joined the hotspot.

## What you need

- A laptop with Windows 10 or 11 and Wi-Fi that supports Mobile Hotspot. Linux is supported in
  code but untested; see [Linux](#linux).
- An internet connection for the laptop (a phone hotspot or Starlink works). Windows will not
  start Mobile Hotspot without one.
- For a warning-free connection: a hostname you control on Cloudflare DNS, with an `A`
  record pointing at `100.64.0.1`, and a Cloudflare API token (Zone:Read and DNS:Edit).
- Optional: [ffmpeg](https://ffmpeg.org/) for the built-in video player.

## Set up the laptop

1. **Build** (Rust and the MSVC build tools on a Windows machine):

   ```powershell
   .\build.ps1
   ```

   The result is `tesremotedesktop\target\release\tesremotedesktop.exe`. The web pages are inside the binary.

2. **Copy** `tesremotedesktop.exe` to the laptop, for example `C:\Program Files\TesRemoteDesktop\`.

3. **Configure** `%ProgramData%\TesRemoteDesktop\.env`:

   ```
   TESREMOTE_HTTPS_HOST=car.example.com
   TESREMOTE_CF_TOKEN=<your Cloudflare API token>
   TESREMOTE_ACME_EMAIL=you@example.com
   TESREMOTE_VIDEO_ROOT=C:\video\
   ```

4. **Install and start the service** from an administrator prompt:

   ```powershell
   cd "C:\Program Files\TesRemoteDesktop"
   .\tesremotedesktop.exe service install
   sc.exe start TesRemoteDesktop
   ```

   The service starts at boot, turns the hotspot on, and restarts the server if it stops.
   `tesremotedesktop.exe service uninstall` removes it.

5. **Recommended:** set the laptop to sign in automatically and never sleep on power, and set
   the Mobile Hotspot name and password in Windows Settings.

Notes:

- Windows 11 Smart App Control blocks unsigned builds. Sign the binary or turn Smart App
  Control off on that laptop.
- Logs are `%ProgramData%\TesRemoteDesktop\tesremotedesktop.log` (server) and `tesremotedesktop-service.log`
  (service).
- To try it on a desk first: `tesremotedesktop.exe --localhost`, then open `http://127.0.0.1:8080/`.

## Upgrading from TeslaPC

Run `tesremotedesktop.exe service install` with the new binary, from an administrator prompt.
On Windows, if a service named `TeslaPC` exists, install clears its restart actions, stops it,
and waits until that service reports stopped and no `teslapc.exe` process remains (30 seconds).
If that wait fails, install stops and leaves the old service in place. Otherwise it copies
`.env`, `cert.pem`, `key.pem`, `acme-account.json`, and `teslapc.db` (saved as
`tesremotedesktop.db`) from `%ProgramData%\TeslaPC` into `%ProgramData%\TesRemoteDesktop`, and
only when the new folder does not already have that file. It then creates the
`TesRemoteDesktop` service and deletes the `TeslaPC` service. It does not start the service,
and it does not change the old folder or the old binary.

On Linux the same files are copied from `~/.config/teslapc` and `~/.local/share/teslapc`.
Install writes `tesremotedesktop.service`. If `teslapc.service` is still on disk it prints
`systemctl disable --now teslapc` and does not run that command.

Settings that still use the `TESLAPC_` prefix keep working. A real environment variable wins
over `.env`, and `TESREMOTE_` wins over `TESLAPC_` at the same level. Saving a setting writes
the `TESREMOTE_` line and removes the `TESLAPC_` line for that setting. A script that starts
`teslapc.exe` itself is not updated.

## Use it in the car

1. Power on the laptop. Give it a minute to sign in and start the hotspot.
2. On the car's display, join the laptop's hotspot under Wi-Fi.
3. Open the browser and go to `https://car.example.com:8443/` (your hostname). Bookmark it.
4. Tap to connect. Tap **Fit screen** once so the picture fills the display.

Use **Files** to play videos from the laptop's video folder, and **Settings** to change the
stream resolution if the picture stutters.

## Settings

Set these in `.env`, or on the Settings page where one exists.

| Key | Default | Purpose |
|-----|---------|---------|
| `TESREMOTE_HTTPS_HOST` | none | Hostname for the trusted certificate |
| `TESREMOTE_CF_TOKEN` | none | Cloudflare API token for certificate issue and renewal |
| `TESREMOTE_ACME_EMAIL` | none | Optional Let's Encrypt contact |
| `TESREMOTE_ACME_STAGING` | unset | `1` uses the Let's Encrypt staging directory |
| `TESREMOTE_VIDEO_ROOT` | `C:\video\` | Folder shown by the file browser |
| `TESREMOTE_STREAM_HEIGHT` | `1080` | Largest picture height sent (`240`-`2160`) |
| `TESREMOTE_INPUT_MODE` | `touch` | `touch` or `mouse` |
| `TESREMOTE_LOG_LEVEL` | `info` | `error`, `warn`, `info`, `debug` |
| `TESREMOTE_FFMPEG` | on `PATH` | ffmpeg executable or folder |
| `TESREMOTE_NO_WGC` | unset | `1` skips Windows Graphics Capture and starts at DXGI |
| `TESREMOTE_BYPASS_IFACE` | empty | Linux only: interface that carries `100.64.0.1` |

The same names with the prefix `TESLAPC_` are still read when the `TESREMOTE_` value is unset.
A blank value counts as unset.

Flags: `--localhost` (local testing, no hotspot or HTTPS), `--no-bypass` (skip the
`100.64.0.1` address; `--no-tesla-bypass` is the same flag), `--https-host <host>`.

## Linux

The same binary runs on Linux (X11 or Wayland through the desktop portal). `tesremotedesktop service
install` writes a systemd unit. Bringing up the Wi-Fi access point is left to you; set
`TESREMOTE_BYPASS_IFACE` to the access point's interface and TesRemoteDesktop adds the address and the
forwarding rules.

The Linux build compiles and passes the test suite in CI. This version has not been run on
a real Linux machine, so treat it as experimental.

## Limits to know about

- **There is no password.** Anyone who joins the laptop's hotspot can see and control the
  laptop, including the sign-in screen. Use a strong hotspot password and do not expose ports
  `8080` or `8443` to any other network.
- The Windows service runs the server with full system rights so it can show the lock screen.
- Video is MJPEG. It is simple and works in the car's browser, and it uses more data than a
  modern video codec. Lower the stream resolution on a weak link.
- Sound and picture are kept close, not frame-locked.
- Some copy-protected video shows as a black rectangle.
- The laptop's display resolution changes when you tap Fit screen, and stays that way.

## For developers

```bash
cargo build --manifest-path tesremotedesktop/Cargo.toml
cargo test  --manifest-path tesremotedesktop/Cargo.toml
node tesremotedesktop/tests/js/pcm_player.test.js
```

[AGENTS.md](AGENTS.md) is the operating guide. Feature-by-feature documentation, behavior
baselines, and decision history are under [documentation/](documentation/README.md).

## Not affiliated with Tesla

TesRemoteDesktop is an independent project. It is not made, endorsed, or supported by Tesla, Inc.
"Tesla" is a trademark of Tesla, Inc. and is used here only to say which car's browser the
project works with.

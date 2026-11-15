# Page names

- Date: 2026-11-15
- Feature: web-ui

## Problem

Page titles, the Settings footer, the logo route, and the audio helpers used the previous product name.

## Decision

Visible product text is `TesRemoteDesktop`. The logo is served at `/logo.png` from `assets/logo.png` as raw PNG bytes. `audio-client.js` exports `startRemoteAudio` and `stopRemoteAudio`. `play.html` calls `startRemoteAudio`. Comments that named the car maker now say "the car's browser" where the maker does not matter.

## Files

- `tesremotedesktop/assets/index.html`
- `tesremotedesktop/assets/config.html`
- `tesremotedesktop/assets/play.html`
- `tesremotedesktop/assets/audio-client.js`
- `tesremotedesktop/assets/style.css`
- `tesremotedesktop/src/assets.rs`

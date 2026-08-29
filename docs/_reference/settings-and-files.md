---
title: Settings & Files
description: Where Snoop keeps configuration, credentials, and caches, and what is safe to delete.
nav_order: 0
---

## Where things live

Snoop follows each platform's conventions. On Linux:

| What | Where | Safe to delete? |
| --- | --- | --- |
| Settings | `~/.config/snoop/settings.json` | Yes, you lose preferences |
| Shared Web API sign-in | `~/.local/state/snoop/shared_web_api_token.json` | Yes, you sign in again |
| Personal Web API sign-in | `~/.local/state/snoop/personal_web_api_token.json` | Yes, personal acceleration is removed |
| Playback credential | `~/.local/state/snoop/credentials/` | Yes, you approve playback again |
| Last session | `~/.local/state/snoop/session.json` | Yes |
| Audio cache | `~/.cache/snoop/audio/` | Always |
| Artwork cache | `~/.cache/snoop/art/` | Always |
| Lyrics cache | `~/.cache/snoop/lyrics/` | Always |
| Last run's log | `~/.local/state/snoop/snoop.log` | Always |
| Account-scoped playlist cache | `~/.cache/snoop/playlists/<account-id>/` | Always |
| Crash log | `~/.local/state/snoop/panic.log` | Always |

Clearing caches never signs you out; credentials live in *state*, not
*cache*. Web API token files are written with owner-only permissions.
Signing out from Settings deletes both Web API grants and the separate
playback credential.

On macOS, settings, state, and the logs are in
`~/Library/Application Support/com.dappermint.snoop` and the caches in
`~/Library/Caches/com.dappermint.snoop`. On Windows, settings are in
`%APPDATA%\dappermint\snoop\config`, state and the logs in
`%LOCALAPPDATA%\dappermint\snoop\data`, and the caches in
`%LOCALAPPDATA%\dappermint\snoop\cache`.

## settings.json

Settings are stored in one readable JSON file and written atomically. Its
main fields are:

| Field | Default | Meaning |
| --- | --- | --- |
| `device_name` | `Snoop` | Name on Spotify Connect |
| `bitrate` | `320` | 96, 160, or 320 kbps |
| `normalisation` | `false` | Volume normalisation |
| `autoplay` | `true` | Keep playing similar music at the end |
| `gapless` | `true` | Gapless playback |
| `audio_backend` | platform | `pulseaudio` or `rodio` on Linux |
| `audio_cache_mb` | `1024` | On-disk audio cache budget |
| `theme` | `dark` | `dark`, `light`, or `system` |
| `color_theme` | `dracula-pro` | `dracula-pro`, `dracula`, or `spotify` |
| `accent_from_art` | `true` | Tint pages with album art |
| `keep_playing_in_background` | `true` | Close to tray |
| `check_for_updates` | `true` | Ask GitHub once a day for a newer release |
| `web_client_id` | none | Optional personal Spotify app id used alongside shared coverage |

## Command line

```
snoop [OPTIONS]

  --device-name <NAME>  Spotify Connect name for this session
  -v, --verbose         More logs from librespot and the API client
```

`snoop.log` in the state directory is what to attach to a bug report:
it contains the last run's output, including the additional lines printed by
`snoop -v`. If the app crashed, attach `panic.log` from the same directory
as well.

## Demo mode

Builds made with `cargo build --features demo` accept `--demo`, which fills
the interface with sample data, useful for screenshots, theming, and
interface work. Demo mode never writes settings.

`--demo-page` opens a page, such as `home`, `playlist:pl1`, or `artist:art0`,
and `--demo-show` adds surfaces on top of it: a comma separated list of
`queue`, `devices`, `shortcuts`, `create`, `light`, and `focus`.

`--demo-shot <PATH>` writes the window to a PNG and exits, which is how the
screenshots in these pages are made:

```
cargo run --release --features demo -- \
  --demo-shot docs/screenshot.png --demo-page playlist:pl1 --demo-show queue
```

The shot is the window's own frame buffer, so it comes out at whatever size
the window is. `--demo-shot-delay <MS>` sets how long cover art has to arrive
before the frame is taken.

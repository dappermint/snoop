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
| Colour schemes | `~/.config/snoop/themes/<name>.toml` | Yes, custom themes are gone |
| Winamp skins | `~/.config/snoop/skins/` | Yes, you add them again |
| MilkDrop presets | `~/.config/snoop/milkdrop/` | Yes, you fetch them again |
| Shared Web API sign-in | `~/.local/state/snoop/shared_web_api_token.json` | Yes, you sign in again |
| Personal Web API sign-in | `~/.local/state/snoop/personal_web_api_token.json` | Yes, the personal app is disabled |
| Playback credential | `~/.local/state/snoop/credentials/` | Yes, you approve playback again |
| Last session | `~/.local/state/snoop/session.json` | Yes |
| Play history | `~/.local/state/snoop/history.json` | Yes |
| Audio cache | `~/.cache/snoop/audio/` | Always |
| Artwork cache | `~/.cache/snoop/art/` | Always |
| Lyrics cache | `~/.cache/snoop/lyrics/` | Always |
| Account-scoped playlist page cache | `~/.cache/snoop/playlists/<account-id>/` | Always |
| Last run's log | `~/.local/state/snoop/snoop.log` | Always |
| Crash log | `~/.local/state/snoop/panic.log` | Always |

Clearing caches never signs you out; credentials live in *state*, not
*cache*. Web API token files are written with owner-only permissions.
Signing out from Settings deletes both Web API grants and the separate
playback credential.

Progress through a playlist is periodically cached as a contiguous prefix.
When the playlist has not changed on Spotify, reopening it resumes from that
prefix instead of requesting the same pages again. Snoop validates the
cache against Spotify's playlist snapshot before showing it.
Successful playlist edits keep that loaded prefix and save it under Spotify's
new snapshot. Snoop reloads the playlist only if the write fails and the
optimistic edit must be reconciled.

The last good playlist folder tree is kept in `session.json`, scoped to the
account that supplied it. This keeps folders visible when local playback is
temporarily unavailable. Live session data is still required for edit grants.

The session also remembers the main window's position. On Windows, a position
whose title bar is no longer on an available monitor's work area is discarded
when reopening the window, keeping its initial on-screen placement instead.

Large playlist pages also have a **Go to song** control. Entering a song
number loads its 50-item page directly, without requesting every earlier page.
Filtering or sorting still covers the whole playlist, so either action returns
to the beginning and loads the remaining pages as needed.

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
| `color_theme` | `dracula` | `dracula`, `spotify`, or a theme file's name |
| `accent_from_art` | `true` | Tint pages with album art |
| `sidebar_compact` | `false` | Names only in the library sidebar, no covers |
| `tracklist_compact` | `false` | One-line track rows without covers |
| `winamp_window` | `false` | The window is the Winamp mini player |
| `skin` | none | File or folder name in the skins folder; blank uses the built-in skin |
| `skin_scale` | by display | Screen pixels per skin pixel, 1 to 4 |
| `winamp_on_top` | `false` | Keep the mini player above other windows |
| `vis` | `bars` | The mini player's visualiser: `bars`, `scope`, or `off` |
| `playlist_open` | `false` | The playlist window is open under the mini player |
| `playlist_height` | `174` | The playlist window's height in skin pixels |
| `eq_open` | `false` | The equalizer window is open under the mini player |
| `eq_on` | `false` | The equalizer shapes local playback |
| `eq_preamp_db` | `0` | The preamp, in decibels, -12 to 12 |
| `eq_bands_db` | ten zeros | The bands from 60 Hz to 16 kHz, in decibels, -12 to 12 |
| `balance` | `0` | Left to right, -1 to 1, for local playback |
| `mono` | `false` | Play both channels the same |
| `playlist_shaded` | `false` | The playlist window is rolled up to its title bar |
| `winamp_shaded` | `false` | The main window is rolled up to its title bar |
| `milkdrop_open` | `false` | The MilkDrop window is open |
| `milkdrop_seconds` | `30` | How long each MilkDrop preset plays |
| `milkdrop_fps` | `60` | MilkDrop frame rate; `0` is uncapped |
| `milkdrop_screen_hz` | `0` | Last reported display refresh rate |
| `milkdrop_fullscreen` | `false` | The MilkDrop window fills the screen |
| `milkdrop_size` | `640, 480` | The MilkDrop window's size in points |
| `keep_playing_in_background` | `true` | Close to tray |
| `check_for_updates` | `true` | Ask GitHub once a day for a newer release |
| `web_client_id` | none | Optional personal Spotify app id used alongside shared coverage |
| `personal_app_nudge_at` | none | Last slow-Spotify personal-app reminder, so it appears at most once a day |

## Colour schemes

`dracula` (the open-source palette, default) and `spotify` (upstream's
green) are built in. Any other name is a file in the themes directory:

```
~/.config/snoop/themes/<name>.toml
```

The format is the flat half of an Alacritty colour config: one
`key = "#rrggbb"` per line. `#` opens a comment outside quotes, and
`[section]` headers are skipped, so the syntax is familiar and a stray
header does no harm. The key names are Snoop's own, listed below, rather
than Alacritty's `background`/`foreground`. Keys that are absent keep
their built-in value, and an unrecognised key is logged and skipped
instead of failing the file.

```toml
dark   = true
accent = "#9580ff"   # what buttons and progress use
window = "#22212c"
```

Colour keys: `window`, `panel`, `surface`, `surface_hover`,
`surface_active`, `outline`, `text`, `secondary`, `dim`, `accent`,
`accent_hover`, `on_accent`, `danger`, `warning`, `overlay`, `shadow`.
Values take `#rrggbb`, `#rrggbbaa`, `0xrrggbb`, or bare hex. `dark` is
`true` or `false` and decides which built-in palette the file starts from.

Files are read at startup, so a new one appears in Settings after a
restart. A theme that is missing or unparseable falls back to the default
and explains itself in the log rather than refusing to start.

## Command line

```
snoop [OPTIONS] [LINK]

  LINK                  A Spotify link to open: spotify:track:…, or an
                        open.spotify.com address
  --device-name <NAME>  Spotify Connect name for this session
  -v, --verbose         More logs from librespot and the API client
```

A link goes to the running Snoop when there is one, which then opens
the page and brings its window forward; otherwise the app starts on it. The
desktop's handler for `spotify:` links runs exactly this.

Attach `snoop.log` from the state directory to bug reports. It contains
the last run's output, including extra lines from `snoop -v`. After a
crash, attach `panic.log` too.

## Demo mode

Builds made with `cargo build --features demo` accept `--demo`, which loads
sample data for screenshots and interface work. Demo mode never writes
settings.

`--demo-page` opens a page, such as `home`, `playlist:pl1`, or `artist:art0`,
and `--demo-show` adds surfaces on top of it: a comma separated list of
`queue`, `devices`, `shortcuts`, `premium`, `create`, `duplicate`, `light`,
`focus`, `winamp`, `playlist`, `eq`, `eq-shade`, and `compact`.

`--demo-shot <PATH>` writes the window to a PNG and exits, which is useful for
making deterministic screenshots for these pages:

```
cargo run --release --features demo -- \
  --demo-shot docs/screenshot.png --demo-page playlist:pl1 --demo-show queue
```

The image uses the current window size. `--demo-shot-delay <MS>` sets how long
to wait for cover art before taking it.

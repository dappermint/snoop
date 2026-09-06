---
title: Getting Started
description: Install Snoop, sign in through your browser, and enable playback on this computer.
nav_order: 2
---

## Install

The [Download page](/download/) has installers and archives for macOS,
Windows, and Linux.

Or build from source with [Rust](https://rustup.rs) 1.95 or newer:

```sh
git clone https://github.com/dappermint/snoop
cd snoop
cargo install --path .
```

On Linux, install the GUI and audio development packages. On Arch:

```sh
sudo pacman -S --needed alsa-lib libpulse libxkbcommon wayland
```

On Debian or Ubuntu:

```sh
sudo apt install libasound2-dev libpulse-dev libxkbcommon-dev libwayland-dev libgl1-mesa-dev
```

Titles in a script the interface font does not cover -- Chinese, Japanese,
Ubuntu) turn empty boxes back into characters.
Korean, Arabic, Hebrew, Thai, and Indic scripts. macOS and Windows include fonts for the common cases. On Linux, install `noto-fonts` and `noto-fonts-cjk` (Arch) or `fonts-noto` and `fonts-noto-cjk` (Debian or Ubuntu) if titles appear as empty boxes.

![Japanese, Chinese, and Korean titles in a playlist](/assets/images/scripts.png)

A desktop entry ships in `packaging/applications/snoop.desktop`. It
registers Snoop for `spotify:` links; with another Spotify client
installed, `xdg-mime default snoop.desktop x-scheme-handler/spotify`
picks Snoop.

## Sign in

Start the app and press **Sign in with Spotify**. Your browser opens Spotify's
consent page, so Snoop never sees your password. When the browser returns
to the app, your library loads.

Snoop stores a refresh token in your platform's state directory
(`~/.local/state/snoop` on Linux). You normally need the browser only
once per machine.

## Enable playback on this computer

Playing music *on this machine* needs a second browser approval because
Spotify authorizes streaming separately ([why](/how-it-connects/)). Open the
device menu in the player bar and select **Set up playback here**, or use
Settings. This needs Spotify Premium. Snoop saves the playback credential.

The computer then appears as a Spotify Connect device named **Snoop**.
You can rename it in Settings.

## Basics

- **Closing the window does not stop the music.** Snoop keeps playing
  from the system tray; reopen it from the tray icon and quit from the tray
  menu or Ctrl+Q. On macOS you can also reopen it from the Dock. Settings can
  turn this off.
- **Play buttons show progress.** The button spins until Spotify responds.
- **Common actions have shortcuts.** Space plays and pauses, Ctrl+F or `/`
  searches, and `Q` opens the queue. Ctrl+/ shows the full list.
- **Rows and cards have context menus.** Right-click a song, playlist, album,
  or artist to see actions such as queue, save, add to playlist, and copy link.
  If the playlist already contains the song, Snoop asks before adding
  another copy.
- **Spotify links open in Snoop.** A `spotify:` link shared from another
  app opens its page, starting Snoop if it is not running. Links to
  `open.spotify.com` go through the browser first, which hands them over the
  same way. `snoop <link>` does the same from a terminal.

---
title: Download
description: Get Snoop for macOS, Windows, or Linux, with install instructions for each.
nav_order: 1
---

{% assign v = site.snoop_version %}
{% assign base = "https://github.com/dappermint/snoop/releases/download/v" | append: v %}

The current version is **v{{ v }}**. Every file below, with its SHA-256, is
listed in [checksums.txt]({{ base }}/checksums.txt); all versions live on
the [releases page](https://github.com/dappermint/snoop/releases).

## macOS

One download for both Apple Silicon and Intel:

- [snoop-v{{ v }}-macos-universal.dmg]({{ base }}/snoop-v{{ v }}-macos-universal.dmg)

Open it and drag **Snoop** to Applications. Or, with
[Homebrew](https://brew.sh):

```sh
brew install --cask dappermint/tap/snoop
```

Homebrew installs the same unnotarized build, so the first-open steps below
still apply. To skip them, clear the quarantine flag instead:

```sh
xattr -d com.apple.quarantine /Applications/Snoop.app
```

### First open on macOS

This build is not yet notarized with Apple, so macOS blocks it the first
time. Recent macOS versions (Sequoia and later) no longer let you bypass
this with a right-click, so you open it once through Privacy & Security:

1. Double-click **Snoop** in Applications. macOS says it cannot be
   opened because Apple cannot check it for malicious software. Click
   **Done** (do **not** click Move to Trash).
2. Open **System Settings**, then **Privacy & Security**.
3. Scroll down to the **Security** section, find *"Snoop was blocked
   to protect your Mac"*, and click **Open Anyway**.
4. Authenticate, then click **Open Anyway** once more.

macOS remembers the choice, so later launches work with an ordinary
double-click.

## Windows

The installer adds Snoop to the Start menu and needs no administrator
rights. Choose x86_64 for most PCs or aarch64 for Windows on ARM:

- [snoop-v{{ v }}-x86_64-pc-windows-msvc-setup.exe]({{ base }}/snoop-v{{ v }}-x86_64-pc-windows-msvc-setup.exe)
- [snoop-v{{ v }}-aarch64-pc-windows-msvc-setup.exe]({{ base }}/snoop-v{{ v }}-aarch64-pc-windows-msvc-setup.exe)

If you would rather not install anything, the same program comes as a zip:
unpack it and run `snoop.exe`.

- [snoop-v{{ v }}-x86_64-pc-windows-msvc.zip]({{ base }}/snoop-v{{ v }}-x86_64-pc-windows-msvc.zip)
- [snoop-v{{ v }}-aarch64-pc-windows-msvc.zip]({{ base }}/snoop-v{{ v }}-aarch64-pc-windows-msvc.zip)

Either way, SmartScreen may warn about an unknown publisher on first run;
choose More info, then Run anyway.

## Linux

Snoop is built and tested on macOS first. The Linux archives are produced by
the same release workflow and should work, but they get far less testing than
the macOS build.

- [snoop-v{{ v }}-x86_64-unknown-linux-gnu.tar.gz]({{ base }}/snoop-v{{ v }}-x86_64-unknown-linux-gnu.tar.gz)
- [snoop-v{{ v }}-aarch64-unknown-linux-gnu.tar.gz]({{ base }}/snoop-v{{ v }}-aarch64-unknown-linux-gnu.tar.gz)

Unpack, put `snoop` on your PATH, and copy the desktop entry and icon
from the bundled `packaging/` directory if you want it in your launcher.
Runtime needs are the ordinary desktop libraries: ALSA, PulseAudio or
PipeWire, and Wayland or X11.

For AUR and Flatpak packages, see
[upstream Fastpotify](https://github.com/crmne/fastpotify) — Snoop does not
publish its own Linux packages.

Or build from source: see [Getting Started](/getting-started/).

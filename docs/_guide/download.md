---
title: Download
description: Get Snoop for macOS, Windows, or Linux, with install instructions for each.
nav_order: 1
---

{% assign v = site.snoop_version %}
{% assign base = "https://github.com/dappermint/snoop/releases/download/v" | append: v %}

The current version is **v{{ v }}**. SHA-256 checksums are in
[checksums.txt]({{ base }}/checksums.txt). Older versions are on the
[releases page](https://github.com/dappermint/snoop/releases).

## macOS

One download for both Apple Silicon and Intel:

- [snoop-v{{ v }}-macos-universal.dmg]({{ base }}/snoop-v{{ v }}-macos-universal.dmg)

Open it and drag **Snoop** to Applications. Once opened, it is
registered for `spotify:` links, so links shared from other apps open in
it; with the official client installed too, macOS keeps whichever it used
last. Or, with [Homebrew](https://brew.sh):

```sh
brew install --cask dappermint/tap/snoop
```

Homebrew installs the same unnotarized build, so the first-open steps below
still apply. To skip them, clear the quarantine flag instead:

```sh
find /Applications/Snoop.app -exec xattr -d com.apple.quarantine {} \; 2>/dev/null
```

The command must clear every file in the bundle. Clearing only the app can
leave it bouncing in the Dock on macOS 26. This command also works on macOS
27, where `xattr` no longer accepts `-r`.

If the command fails, use the steps below instead. They do not need a
terminal.

### First open on macOS

This build is not notarized, so macOS blocks the first launch. On Sequoia and
later, allow it in Privacy & Security:

1. Double-click **Snoop** in Applications. macOS says it cannot be
   opened because Apple cannot check it for malicious software. Click
   **Done** (do **not** click Move to Trash).
2. Open **System Settings**, then **Privacy & Security**.
3. Scroll down to the **Security** section, find *"Snoop was blocked
   to protect your Mac"*, and click **Open Anyway**.
4. Authenticate, then click **Open Anyway** once more.

Later launches work with a normal double-click.

## Windows

The installer adds Snoop to the Start menu and needs no administrator
rights. It also registers Snoop for `spotify:` links; if the official
client is installed too, Settings → Apps → Default apps decides which of
the two opens them. Choose x86_64 for most PCs or aarch64 for Windows on ARM:

- [snoop-v{{ v }}-x86_64-pc-windows-msvc-setup.exe]({{ base }}/snoop-v{{ v }}-x86_64-pc-windows-msvc-setup.exe)
- [snoop-v{{ v }}-aarch64-pc-windows-msvc-setup.exe]({{ base }}/snoop-v{{ v }}-aarch64-pc-windows-msvc-setup.exe)

For a portable copy, download a zip, unpack it, and run `snoop.exe`.

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
from the bundled `packaging/` directory if you want it in your launcher and
handling `spotify:` links.
The binary needs ALSA, PulseAudio or PipeWire, and Wayland or X11.

For AUR and Flatpak packages, see
[upstream Fastpotify](https://github.com/crmne/fastpotify) — Snoop does not
publish its own Linux packages.

Or build from source: see [Getting Started](/getting-started/).

## Nix

Add the repository [flake](https://github.com/dappermint/snoop) to your
inputs:

```nix
inputs.snoop.url = "github:dappermint/snoop";
```

On NixOS, install the default package:

```nix
environment.systemPackages = [
  inputs.snoop.packages."${pkgs.stdenv.hostPlatform.system}".default
];
```

### nix-darwin

On macOS, use the `snoop-app` package instead. It is a `Snoop.app`
bundle built and signed locally, so it is never quarantined and the
first-open steps above do not apply:

```nix
environment.systemPackages = [
  inputs.snoop.packages."${pkgs.stdenv.hostPlatform.system}".snoop-app
];
environment.pathsToLink = [ "/Applications" ];
```

The bundle appears in `/Applications/Nix Apps`. With Home Manager,
`home.packages` is enough; its darwin support links app bundles into
`~/Applications`:

```nix
home.packages = [
  inputs.snoop.packages."${pkgs.stdenv.hostPlatform.system}".snoop-app
];
```

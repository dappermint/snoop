# Snoop agent guide

Follow `CONTRIBUTING.md`; it is the canonical product and contribution policy.
These instructions add implementation constraints for coding agents.

## Product boundaries

- Keep Snoop a small native Spotify client. Do not add a browser engine,
  telemetry, a hosted backend, or alternate sources for Spotify audio.
- Playback capabilities come from librespot. Do not advertise or implement a
  capability merely because its name appears in a protobuf or enum. In
  particular, do not pursue Spotify Lossless or DRM circumvention unless
  lawful support first lands upstream.
- Do not broaden a task into adjacent features or a general refactor. Preserve
  existing user behaviour unless the task explicitly changes it.

## Architecture

- `src/ui/` draws views and emits `Action`s. Apply actions after drawing in
  `src/app.rs`; do not mutate application state from inside a borrowed view.
- Network and playback work belongs on the runtime in `src/backend.rs` or in
  the player engine in `src/player.rs`, never as blocking work on the UI
  thread.
- Keep platform integrations behind target-specific modules or `cfg` blocks.
  A fix for one platform must keep the other two targets compiling.
- Settings and state files must remain readable, backward compatible, and
  atomically written. Never log credentials or authorization responses.
- Prefer existing dependencies. Explain any new crate in `Cargo.toml` next to
  the dependency when the reason is not obvious.

Read `docs/_reference/how-it-connects.md` before changing authentication,
Spotify requests, Connect, credential storage, or network behaviour. Read the
nearby module tests before changing a state machine or API fallback.

## Definition of done

- Add focused regression tests for changed behaviour. Use the `demo` feature
  for deterministic UI coverage and screenshots.
- Update the README and docs when user-visible behaviour, settings, files, or
  network access changes.
- Run the full checks from `CONTRIBUTING.md`. Do not weaken a lint, delete a
  test, or add an `allow` merely to make CI green without explaining why the
  underlying rule does not apply.
- Report platform coverage honestly. Do not claim a platform was tested when
  it was only compiled or reasoned about.

## Releases

A release is not the tag alone. Every one of these moves together:

- `Cargo.toml` version (and the lockfile via a build), committed before
  the tag so the binaries report the right version.
- The `v*` tag, which triggers the release workflow; replace its
  generated notes with written ones.
- `docs/_config.yml` `snoop_version` (the download page's links)
  and `docs/_data/versions.yml` (the version dropdown: the new version
  becomes `current` and points at `/download/`, the previous one keeps a
  link to its own GitHub release).
- The Homebrew cask in the maintainer's tap, from the release's
  `checksums.txt`. Snoop publishes no AUR or Flatpak package.

Missing any of these ships a release that lies somewhere; the dropdown
was forgotten once already.

## Staying close to upstream

Snoop is a fork of [Fastpotify](https://github.com/crmne/fastpotify) and
merges from it regularly, so every gratuitous divergence is a future merge
conflict. Prefer changes that upstream's files do not have to notice:

- Colour schemes live in `src/themes.rs`. `src/theme.rs` keeps upstream's
  palette verbatim so a merge that adds a field does not collide with ours.
- When upstream and Snoop want different text or behaviour, branch on a
  Snoop-owned constant or module rather than editing upstream's line.

#!/bin/bash
# Points dappermint/homebrew-tap's snoop cask at a published app release.
# Requires macOS: brew's cask commands are unavailable on Linux.
# Run from CI with HOMEBREW_TAP_TOKEN set, or locally with your own git credentials.
#
# Usage: scripts/bump-cask.sh <version> [sha256]
#   scripts/bump-cask.sh 0.1.5
#   scripts/bump-cask.sh 0.1.5 6dc4cc48...   # skip the download

set -euo pipefail

VERSION="${1:?usage: scripts/bump-cask.sh <version> [sha256]}"
VERSION="${VERSION#v}"
SHA="${2:-}"
TAP="dappermint/tap"
CASK="Casks/snoop.rb"

if ! printf '%s' "$VERSION" | grep -qE '^[0-9]+(\.[0-9]+)*$'; then
    echo "not a dotted numeric version: $VERSION" >&2
    exit 1
fi

TOKEN="${HOMEBREW_TAP_TOKEN:-}"

brew tap "$TAP" >/dev/null
TAP_DIR="$(brew --repository "$TAP")"

if ! git -C "$TAP_DIR" diff --quiet; then
    echo "tap checkout at $TAP_DIR has uncommitted changes; sort them out first" >&2
    exit 1
fi
git -C "$TAP_DIR" fetch -q origin main
git -C "$TAP_DIR" reset -q --hard FETCH_HEAD

echo "==> Bumping $TAP/snoop to ${VERSION}"
# --write-only leaves the tap dirty instead of opening a PR; the tap is ours to
# push to directly. Without --sha256 brew downloads the DMG to compute it.
HOMEBREW_DEVELOPER=1 \
HOMEBREW_NO_AUTO_UPDATE=1 \
HOMEBREW_GITHUB_API_TOKEN="$TOKEN" \
    brew bump-cask-pr --write-only --no-audit --no-style \
    --version "$VERSION" ${SHA:+--sha256 "$SHA"} \
    "$TAP/snoop"

cd "$TAP_DIR"
if git diff --quiet; then
    echo "==> Cask already at ${VERSION}; nothing to do."
    exit 0
fi
git diff --stat

if [ -n "$TOKEN" ]; then
    git remote set-url origin "https://x-access-token:${TOKEN}@github.com/dappermint/homebrew-tap.git"
    git config user.name "github-actions[bot]"
    git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
fi
git commit -q "$CASK" -m "fix(snoop): bump to ${VERSION}"
git push -q origin main
echo "==> Tap updated to snoop ${VERSION}."

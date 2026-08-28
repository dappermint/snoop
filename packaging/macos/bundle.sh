#!/bin/bash
# Build Snoop.app from a GUI binary, on a macOS machine.
#
#   packaging/macos/bundle.sh <binary> <output.app> <version>
#
# Set CODESIGN_IDENTITY to sign with a Developer ID; the default is an ad-hoc
# signature, which arm64 requires before the app will launch at all.
#
# The .icns is generated here from the committed 1024px PNG, because iconutil
# only exists on macOS. The Info.plist template lives next to this script.
set -euo pipefail

binary="$1"
app="$2"
version="$3"
here="$(cd "$(dirname "$0")" && pwd)"

rm -rf "$app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"

cp "$binary" "$app/Contents/MacOS/snoop"
chmod 755 "$app/Contents/MacOS/snoop"
sed "s/__VERSION__/$version/g" "$here/Info.plist" > "$app/Contents/Info.plist"

iconset="$(mktemp -d)/snoop.iconset"
mkdir -p "$iconset"
# iconutil only accepts these base sizes, each with an optional @2x.
for size in 16 32 128 256 512; do
    sips -z $size $size "$here/icon-1024.png" --out "$iconset/icon_${size}x${size}.png" >/dev/null
    double=$((size * 2))
    sips -z $double $double "$here/icon-1024.png" --out "$iconset/icon_${size}x${size}@2x.png" >/dev/null
done
iconutil -c icns "$iconset" -o "$app/Contents/Resources/snoop.icns"

if [ -n "${CODESIGN_IDENTITY:-}" ]; then
    codesign --force --timestamp --options runtime \
        --sign "$CODESIGN_IDENTITY" "$app"
else
    codesign --force --sign - "$app"
fi
codesign --verify --strict "$app"

echo "$app"

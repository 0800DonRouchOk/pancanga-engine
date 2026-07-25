#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CARGO_BIN="${CARGO_BIN:-/Users/donrouch/.cargo/bin/cargo}"
SWIFTC_BIN="${SWIFTC_BIN:-/usr/bin/swiftc}"
DIST_DIR="$ROOT/dist/macOS"
APP_DIR="$DIST_DIR/Pancanga Engine.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"
CONTENT_SRC="$ROOT/08_Examples/RC1-Experience/content/ekadasi"
CONTENT_DST="$RESOURCES_DIR/content/ekadasi"
BIN_SRC="$ROOT/03_Source/rust/target/release/examples/rc1_experience"
BIN_DST="$RESOURCES_DIR/rc1_experience"
LAUNCHER_SRC="$ROOT/Deploy/macOS/rc1_launcher.swift"
LAUNCHER_DST="$MACOS_DIR/PancangaEngine"

if [[ ! -x "$CARGO_BIN" ]]; then
  echo "cargo not found at $CARGO_BIN" >&2
  exit 1
fi

if [[ ! -x "$SWIFTC_BIN" ]]; then
  echo "swiftc not found at $SWIFTC_BIN" >&2
  exit 1
fi

COMMIT="$(git -C "$ROOT" rev-parse --short HEAD 2>/dev/null || echo unknown)"

"$CARGO_BIN" build \
  --manifest-path "$ROOT/03_Source/rust/Cargo.toml" \
  -p pancanga-engine \
  --example rc1_experience \
  --release

rm -rf "$APP_DIR"
mkdir -p "$MACOS_DIR" "$CONTENT_DST" "$RESOURCES_DIR/logs"

cp "$BIN_SRC" "$BIN_DST"
chmod +x "$BIN_DST"
cp "$CONTENT_SRC"/*.json "$CONTENT_DST"/

"$SWIFTC_BIN" "$LAUNCHER_SRC" -O -o "$LAUNCHER_DST"
chmod +x "$LAUNCHER_DST"

cat > "$CONTENTS_DIR/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key>
  <string>Pancanga Engine</string>
  <key>CFBundleDisplayName</key>
  <string>Pancanga Engine RC1</string>
  <key>CFBundleIdentifier</key>
  <string>org.pancanga.engine.rc1</string>
  <key>CFBundleVersion</key>
  <string>1.0.0-rc1</string>
  <key>CFBundleShortVersionString</key>
  <string>1.0 RC1</string>
  <key>CFBundleExecutable</key>
  <string>PancangaEngine</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>LSMinimumSystemVersion</key>
  <string>12.0</string>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
PLIST

DMG="$DIST_DIR/Pancanga-Engine-RC1.dmg"
rm -f "$DMG"
if command -v hdiutil >/dev/null 2>&1; then
  if hdiutil create \
    -volname "Pancanga Engine RC1" \
    -srcfolder "$APP_DIR" \
    -ov \
    -format UDZO \
    "$DMG" >/dev/null; then
    :
  else
    echo "DMG: skipped (hdiutil could not create the image in this environment)" >&2
    rm -f "$DMG"
  fi
fi

echo "App: $APP_DIR"
if [[ -f "$DMG" ]]; then
  echo "DMG: $DMG"
fi

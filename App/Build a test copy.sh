#!/bin/bash
# Builds a fresh test copy of Smoozy, called "Smoozy Test", from the newest
# commit on `main`, and puts it at ~/Applications/Smoozy Test.app.
#
# Run it in Terminal with:
#   bash "/path/to/your/checkout/App/Build a test copy.sh"
#
# How it is built is the testers' way (Audits/Build/Step 10c/Fix piece 1, the
# shortcut — what landed.md, and Brief — the pass before 0.1.9.md): the commit
# is exported with `git archive` outside the working tree, the desktop copy is
# a development build in a build folder nothing else uses, and the server
# program it places on a rented machine is built for Linux from the same
# export in a fresh folder.
#
# The copy has its own name and bundle identity (dev.smoozy.throwaway.test) and
# always starts on its own folder, ~/Library/Application Support/Smoozy Test,
# so it cannot open or change the installed Smoozy, its record, its keys or its
# servers. The account is Audits/Build/Step 10c/A test copy the Creator builds
# — what landed.md.
#
# The program itself is the bundle's starting file, as in a release, and the
# bundle has a local signature under its own identity, so macOS can match the
# program to the bundle and the copy can show notifications. Its folder and the
# server program beside it are built into the program rather than handed to it
# by a launcher script (Audits/Build/Step 11/The Mac watch's three findings —
# what landed.md).

set -euo pipefail

started=$SECONDS
repo="$(cd "$(dirname "$0")/.." && pwd)"
build="$HOME/Library/Caches/Smoozy Test build"
app="$HOME/Applications/Smoozy Test.app"
record="$HOME/Library/Application Support/Smoozy Test"
running_copy="Smoozy Test.app/Contents/MacOS/smoozy-desktop"

stop() {
  echo
  echo "Stopped: $1"
  echo "Nothing else was changed."
  exit 1
}

say() { echo; echo "$1"; }

# --- Before building -------------------------------------------------------

command -v cargo >/dev/null 2>&1 || stop "Rust is not installed on this Mac, so nothing can be built."
command -v git >/dev/null 2>&1 || stop "git is not installed on this Mac."
rustup target list --installed 2>/dev/null | grep -q '^x86_64-unknown-linux-musl$' \
  || stop "the Linux build tools are missing. Run: rustup target add x86_64-unknown-linux-musl"

if pgrep -f "$running_copy" >/dev/null 2>&1; then
  stop "Smoozy Test is open. Quit it, then run this again."
fi
if pgrep -x rustc >/dev/null 2>&1; then
  stop "another build is running on this Mac. Wait for it to finish, then run this again."
fi

mkdir -p "$HOME/Library/Caches" "$HOME/Applications"
free_gb=$(df -g "$HOME/Library/Caches" | awk 'NR==2 {print $4}')
if [ "$free_gb" -lt 10 ]; then
  stop "only ${free_gb} GB of disk is free. The build needs about 10 GB."
fi

commit=$(git -C "$repo" rev-parse --verify main) || stop "this folder has no 'main' to build from."
short=${commit:0:8}

say "Building Smoozy Test from commit $short on main."
say "This takes about 5 minutes. You can leave it running."

# An earlier run that was stopped part-way may have left its build folder.
if [ -e "$build" ]; then
  rm -rf "$build"
fi
mkdir -p "$build/export"

# --- 1. The export ---------------------------------------------------------

git -C "$repo" archive "$commit" App Preparation COPYING | tar -x -C "$build/export"
version=$(awk '/^\[workspace.package\]/ {s=1; next} /^\[/ {s=0} s && /^version/ {gsub(/"/, "", $3); print $3; exit}' "$build/export/App/Cargo.toml")
[ -n "$version" ] || stop "could not read the version number from the commit."

# Both builds start inside the exported App/, so its .cargo path-replacement
# helper applies to the desktop and the server.
export CARGO_BUILD_JOBS=2
export CARGO_INCREMENTAL=0

build_step() {
  local what="$1" log="$2"
  shift 2
  if ! (cd "$build/export/App" && "$@") >"$log" 2>&1; then
    echo
    tail -n 30 "$log"
    stop "the $what did not build. The full output is in: $log"
  fi
}

# --- 2. The server program, for Linux --------------------------------------

say "Step 1 of 3: the server program Smoozy Test places on a rented machine."
build_step "server program" "$build/server.log" \
  env CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=rust-lld \
  cargo build --locked --release -p smoozy-server --bin smoozy-server \
  --target x86_64-unknown-linux-musl --target-dir "$build/server"
server="$build/server/x86_64-unknown-linux-musl/release/smoozy-server"
file "$server" | grep -q 'ELF 64-bit LSB.*x86-64' || stop "the server program came out in the wrong form."

# --- 3. The desktop copy ---------------------------------------------------

# A test copy never checks for an update. `SMOOZY_TEST_COPY` is read when the
# app is built, by `Desktop app and delivery/src/updating/checking.rs`, to keep
# the update check off. A release never sets it.
#
# The copy's own folder, and the server program it places, are built in the
# same way: `SMOOZY_TEST_COPY_FOLDER` and `SMOOZY_TEST_COPY_DESK_PROGRAM` are
# read by `Desktop app and delivery/src/platform/locations.rs`, only in a
# development build, so the copy starts on its own folder however it is opened.
# The desk program is named from the program's own folder, so the app still
# works if it is moved.
say "Step 2 of 3: the app itself."
build_step "app" "$build/desktop.log" \
  env SMOOZY_TEST_COPY=1 \
  SMOOZY_TEST_COPY_FOLDER="$record" \
  SMOOZY_TEST_COPY_DESK_PROGRAM="../Resources/smoozy-server" \
  cargo build --locked -p smoozy-desktop --target-dir "$build/desktop"
desktop="$build/desktop/debug/smoozy-desktop"
[ -x "$desktop" ] || stop "the app did not come out of the build."

# --- 4. The app ------------------------------------------------------------

say "Step 3 of 3: putting the app together."
staged="$build/Smoozy Test.app"
mkdir -p "$staged/Contents/MacOS" "$staged/Contents/Resources"
cp "$desktop" "$staged/Contents/MacOS/smoozy-desktop"
cp "$server" "$staged/Contents/Resources/smoozy-server"
cp "$build/export/App/Desktop app and delivery/icons/icon.icns" "$staged/Contents/Resources/icon.icns"

cat >"$staged/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key><string>en</string>
  <key>CFBundleExecutable</key><string>smoozy-desktop</string>
  <key>CFBundleIconFile</key><string>icon.icns</string>
  <key>CFBundleIdentifier</key><string>dev.smoozy.throwaway.test</string>
  <key>CFBundleInfoDictionaryVersion</key><string>6.0</string>
  <key>CFBundleName</key><string>Smoozy Test</string>
  <key>CFBundleDisplayName</key><string>Smoozy Test</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>$version</string>
  <key>CFBundleVersion</key><string>$version</string>
  <key>CFBundleGetInfoString</key><string>Test copy built from commit $short</string>
  <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
PLIST
plutil -lint "$staged/Contents/Info.plist" >/dev/null || stop "the app's description file came out wrong."

# A local signature under the copy's own identity. macOS will not let a program
# ask to show notifications unless it can match it to a signed bundle; this
# signature is only for this Mac and is not the release's.
codesign --force --sign - --identifier dev.smoozy.throwaway.test "$staged" >"$build/sign.log" 2>&1 \
  || stop "the app could not be signed on this Mac. The output is in: $build/sign.log"
codesign --verify --strict "$staged" >>"$build/sign.log" 2>&1 \
  || stop "the app's signature did not check out. The output is in: $build/sign.log"

if pgrep -f "$running_copy" >/dev/null 2>&1; then
  stop "Smoozy Test was opened during the build. Quit it, then run this again."
fi
if [ -e "$app" ]; then
  rm -rf "$app"
fi
mv "$staged" "$app"

# The copy's own folder; the program makes what it needs inside it.
mkdir -p "$record"

# Tell macOS about the new app at its place, so the next start is matched to
# this bundle rather than to the copy it replaced.
lsregister=/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister
if [ -x "$lsregister" ]; then
  "$lsregister" -f "$app" >/dev/null 2>&1 || true
fi

# --- 5. Time, disk, and cleaning up ----------------------------------------

used=$(du -sh "$build" | awk '{print $1}')
rm -rf "$build"

[ -x "$app/Contents/MacOS/smoozy-desktop" ] && [ -f "$app/Contents/Resources/smoozy-server" ] \
  || stop "the app is missing a part after cleaning up."

took=$(( SECONDS - started ))
app_size=$(du -sh "$app" | awk '{print $1}')

cat <<DONE

Done. Smoozy Test $version, built from commit $short on main.

Where it is:  $app
How to open:  in Finder, choose Go > Home, open Applications, and double-click
              Smoozy Test. Or press Command-Space, type Smoozy Test, press Return.
              The first time it may ask to reach your Downloads folder; press Allow.

Its own folder: $record
It never opens or changes your installed Smoozy, its record, its keys or its servers.

It took $((took / 60)) min $((took % 60)) s. The build used $used of disk while it ran;
that has been cleaned up. The app itself is $app_size.
DONE

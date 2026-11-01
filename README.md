# Smoozy

Smoozy turns a rented bare-metal GPU server into a personal Blender render farm — automating the setup, file upload, rendering, and download, so a 3D artist never has to touch a terminal.

## Who it is for

Smoozy is for solo 3D artists and small studios working in Blender 4.5 or newer who want to render on rented Linux GPU servers without managing Linux by hand.

## What it does

Smoozy is a desktop app for macOS, Windows, and Linux. You rent a Linux server from any provider that gives you SSH access, add it with your SSH key, and Smoozy connects directly from your computer to your server. There is no Smoozy account and no Smoozy server in between.

- **Prepares the server.** On Ubuntu and Debian servers, Smoozy installs system requirements, sets up NVIDIA drivers where the server allows it, and installs official Blender 4.5 or newer — or a Linux Blender archive you upload — side by side. On other Linux distributions, Smoozy connects and browses files.
- **Checks and uploads scenes.** Checks `.blend` files for missing linked files, packs or sends them alongside the scene, and uploads files to the server.
- **Runs renders on the server.** Queues and runs Blender renders that keep going on the server if you close Smoozy or lose your internet connection, and reconnects to running jobs when you open it again.
- **Downloads finished frames.** Downloads rendered frames to your computer and resumes interrupted transfers.
- **Optional Telegram updates.** Sends render updates through your own Telegram bot if you connect one.

**What Smoozy does not do:** it does not rent servers for you, split one frame or one job across several servers, or install Blender add-ons. On Windows, SSH keys protected by a passphrase are not supported yet.

## Get it

Pre-built downloads are published at <https://github.com/a-kislyakov/Smoozy-releases/releases>:

- **macOS** — Mac with Apple silicon on macOS 11 or newer (`Smoozy-macos.zip`).
- **Windows** — Windows 10 or newer on 64-bit Intel or AMD (`smoozy-desktop-windows.exe`).
- **Linux** — `.deb` packages for Ubuntu 24.04, Ubuntu 26.04, and Debian 13 on 64-bit Intel or AMD.

Smoozy checks this releases page for a newer version when it starts and every three hours while it is open.

If macOS blocks an unsigned download, move `Smoozy.app` to `/Applications`, run `xattr -dr com.apple.quarantine "/Applications/Smoozy.app"` once in Terminal, and open it.

## Build from source

Keep `App/`, `Preparation/`, and the root `COPYING` file together: the desktop build runs `App/Release/notices.py` to embed `COPYING` and generate third-party licence notices from Cargo's locked dependency tree, and the test suite reads `Preparation/`.

### Tools needed

- **Rust** (`stable`) via `rustup`, and **Python 3.9 or newer** (`python3` on macOS and Linux, `python` on Windows).
- **Node.js and `npm`** when bundling a macOS `.app` or Linux `.deb` with Tauri (`@tauri-apps/cli@2.11.4`).
- **On Ubuntu or Debian**, the system packages used by `.github/workflows/desktop-build.yml`:

  ```sh
  sudo apt-get update
  sudo apt-get install -y build-essential curl wget file \
    libgtk-3-dev libwebkit2gtk-4.1-dev librsvg2-dev patchelf libxdo-dev libssl-dev
  ```

### Build the desktop app

From `App/`:

```sh
cargo build --release --locked -p smoozy-desktop
```

To build a macOS `.app` or Linux `.deb` bundle, run from `App/Desktop app and delivery`:

```sh
npm install --no-save @tauri-apps/cli@2.11.4
npx tauri build --config tauri.conf.json -- --locked
```

On Windows, Smoozy ships the unbundled `smoozy-desktop.exe` beside `COPYING` and `THIRD-PARTY-LICENSES.txt` (see `.github/workflows/desktop-build.yml`).

### Build and use your own server program

A release build of `smoozy-desktop` only places a server program listed in a manifest signed by the maintainer's offline release key. If you build `smoozy-desktop` with `--release` at the version of a published release, it fetches and verifies that published release's server program; a release build will never place an unsigned `smoozy-server` binary.

To run your own `smoozy-server` without the maintainer's signing key, build `smoozy-server` as a static Linux binary (`x86_64-unknown-linux-musl`, with `clang` installed as configured in `App/.cargo/config.toml`) and run a development build of `smoozy-desktop` pointed at a separate data folder:

```sh
rustup target add x86_64-unknown-linux-musl
cd App
CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=rust-lld \
  cargo build --release --locked --target x86_64-unknown-linux-musl -p smoozy-server
cargo build --locked -p smoozy-desktop
./target/debug/smoozy-desktop \
  --throwaway-copy=/path/to/test-data \
  --desk-program=./target/x86_64-unknown-linux-musl/release/smoozy-server
```

On macOS, `bash "App/Build a test copy.sh"` builds both from `main` into `~/Applications/Smoozy Test.app` (requires the `x86_64-unknown-linux-musl` Rust target, Python 3, `clang`, and at least 10 GB of free disk space).

### What only the maintainer can do

Signing and notarizing the macOS app bundle with the maintainer's Apple Developer ID (`smoozy-release sign-the-mac-bundle`), signing server-package release manifests and desktop update files (`smoozy-release compose`), and publishing official releases to `Smoozy-releases` require the maintainer's keys and accounts. Everything else builds from this repository.

## Problems and contributions

Bug reports and pull requests are welcome at <https://github.com/a-kislyakov/Smoozy>. See [CONTRIBUTING.md](CONTRIBUTING.md) before opening an issue, sending a change, or running the test suite.

## Licence

Copyright © 2026 Alexander Kislyakov — Smoozy is free software under the GNU General Public License, version 3 or any later version (see `COPYING`), with no warranty.

Blender is made by the Blender Foundation. Smoozy is not made or endorsed by them.

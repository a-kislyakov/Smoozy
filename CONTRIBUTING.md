# Contributing to Smoozy

How to report a problem, send a change, and run Smoozy's tests on your computer.

## Reporting a problem

Issues are welcome at <https://github.com/a-kislyakov/Smoozy/issues>. Say what you tried to do, what happened instead, and which operating system and Smoozy version you used.

If you save a report file through **Smoozy → Report a problem…** in the app, read it before attaching it to a public issue: Smoozy leaves out your SSH keys, passphrases, and Telegram bot token, while server names, file names, and network addresses stay in the file so you can check them. If you would rather not post a report file in public, send it to `hello@smoozy.app`.

## Sending changes

Pull requests on <https://github.com/a-kislyakov/Smoozy> are welcome and read. Day-to-day work on Smoozy happens in a private repository and the public repository is updated as an export at each release, so changes that are taken are carried in by hand rather than merged on GitHub, and appear here in the next release.

There is no contributor licence agreement: you keep your copyright and offer your changes under the same GPL-3.0-or-later licence (see `COPYING`), which means Smoozy cannot later be relicensed outside the GPL without every contributor's agreement.

## Running the tests at your desk

Keep `App/`, `Preparation/`, and the root `COPYING` file in place — the build runs `App/Release/notices.py` (Python 3.9 or newer) and the workspace tests read `Preparation/`. Several integration tests also call standard command-line tools (`sh`, `tar`, `ssh-keygen`, and `python3` on macOS and Linux or `python` on Windows), and several tests of the app's pages need **Node.js** (`node`).

On Windows with Git Bash, first build and select the Windows compiler launcher from `App/`:

```sh
python .cargo/rustc-with-public-paths.py --build-windows-launcher target/public-paths/rustc-with-public-paths.exe
export RUSTC_WRAPPER="$(cygpath -m "$PWD/target/public-paths/rustc-with-public-paths.exe")"
```

This launcher and the Mac/Linux helper replace private build paths in compiled programs.
Rebuild the launcher after moving the checkout or Python.

From `App/`, run:

```sh
SMOOZY_DEVELOPMENT_TELEGRAM_REACHES_NOTHING=1 \
  cargo test --workspace --no-fail-fast -- --skip the_store_round_trips_on_this_machine
```

- `SMOOZY_DEVELOPMENT_TELEGRAM_REACHES_NOTHING=1` keeps tests from reaching `api.telegram.org`.
- `--skip the_store_round_trips_on_this_machine` skips the one test that writes a real entry to your operating system's credential store.
- On a machine with 16 GB of memory, add `CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0` in front of `cargo test` and `--test-threads=2` after `--`.

If you change `smoozy-server`, `smoozy-runtime`, or `smoozy-telegram`, also check the server crates from `App/` (after installing those targets with `rustup target add`, plus `clang` for the Linux musl target):

```sh
cargo check --target x86_64-unknown-linux-musl -p smoozy-server -p smoozy-runtime -p smoozy-telegram --all-targets
cargo check --target x86_64-pc-windows-msvc -p smoozy-server
```

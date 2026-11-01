# Shell measurement — sources

*Where every outside claim in `Shell measurement.md` came from, when it was read, and what it does not cover. Smoozy's own decisions are not here: the choice of shell, the choice of language, and how the comparison was held fixed are decisions and carry no citation.*

**Everything below was read on 2026-09-02.**

## The two frameworks' own release publications

- **Wails v2.14.0 is the latest released version.** The project's own release listing, `https://api.github.com/repos/wailsapp/wails/releases`. Label: **documented**. **What it does not cover:** a `v2.15.0` tag exists in the same repository with no release entry beside it, so *released* and *tagged* differ here and the released version was taken.
- **Tauri 2.11.5, with its build helper 2.6.3 and its command-line tool 2.11.4.** The project's own package publications, `https://crates.io/api/v1/crates/tauri`, `…/tauri-build`, `…/tauri-cli`. Label: **documented**.
- **Tauri's terms are Apache-2.0 OR MIT.** The same package publication's licence field. Label: **documented**.

## Platform minimums

- **Wails.** *Installation*, `https://raw.githubusercontent.com/wailsapp/wails/master/website/docs/gettingstarted/installation.mdx` — *"Windows 10/11 AMD64/ARM64"*, *"MacOS 10.15+ AMD64 for development, MacOS 10.13+ for release"*, *"MacOS 11.0+ ARM64"*, and, for Ubuntu 24.04, install `libwebkit2gtk-4.1-dev` and build with the `webkit2_41` tag. Label: **documented**. *(`wails.io` returns 403 to a fetch, so this is the site's own documentation source — the same route the architecture document's own Wails citations took.)*
- **Tauri.** *Prerequisites*, `https://v2.tauri.app/start/prerequisites/` — *"macOS Catalina (10.15) and later"*, *"Windows 7 and later"*, and `libwebkit2gtk-4.1-dev` on Debian-family Linux. Label: **documented**.

## Tauri's window, menu, picker, signing and development surfaces

- **The frame, the drag region and the three buttons.** *Window Customization*, `https://v2.tauri.app/learn/window-customization/`. Quoted in the document: that the drag attribute *"will only work on the element to which it is directly applied"*, and that on macOS a custom title bar *"will also lose some features provided by the system, such as moving or aligning the window"*. Label: **documented**.
- **`decorations` and the window theme.** *Configuration*, `https://v2.tauri.app/reference/config/` — *"Whether the window should have borders and bars"*, default true; and *"The initial window theme. Defaults to the system theme. Only implemented on Windows and macOS 10.14+."* Label: **documented**. **What it does not cover:** the frameless setting carries no per-system table, so the page says nothing about how it behaves on each of the three.
- **The native menu bar.** *Window Menu*, `https://v2.tauri.app/learn/window-menu/` — *"Native application menus can be attached to both to a window or system tray. Available on desktop."* Label: **documented**.
- **The native file picker.** *Dialog*, `https://v2.tauri.app/plugin/dialog/` — *"Native system dialogs for opening and saving files along with message dialogs"*, listing Windows, Linux and macOS. Label: **documented**.
- **Signing and notarization.** *macOS Code Signing*, `https://v2.tauri.app/distribute/sign/macos/`, whose Notarization section names both of Apple's credential routes; *Windows Code Signing*, `https://v2.tauri.app/distribute/sign/windows/`. Label: **documented**.
- **The inspector in a release build.** The configuration page above — *"This API works in debug builds, but requires devtools feature flag to enable it in release builds"* — and *Debug*, `https://v2.tauri.app/develop/debug/` — *"By default, the inspector is only enabled in development and debug builds unless you enable it with a Cargo feature"*. Label: **documented**. **What it does not cover:** whether the web view's own ordinary context menu is present in a release build. No page read states it, and the document says so rather than claiming either way.

## What was observed rather than read

**Every byte figure, every artifact digest, every runner-image identity and every linked-dependency list came from the run itself** — GitHub Actions run `33651322639`, source commit `a39a0f73`, 2026-09-02. Label: **observed in a test**. These are readings of two artifacts this project built, not claims about the world, and they are reproducible from the source, locks, commands and image identities the document records.

**What the run does not cover** is stated in the document's own limits: one run, one image per system, nothing launched, no release-profile tuning on either candidate, and macOS measured as an x86_64 cross-build on an Apple-silicon host.

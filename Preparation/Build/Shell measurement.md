# The shell measurement

*The comparison the Architecture stage handed to Build: Wails v2 against Tauri v2, measured on equivalent artifacts rather than on two unrelated applications. It fills the sheet at `Preparation/Architecture/Desktop app and delivery.md` H5b, and it is the record behind every figure now in that sheet.*

**The result: Tauri v2 is materially smaller, by H5b's own rule, and it is smaller on every system and on every measure taken.** At the largest-system comparison the sheet names, Wails is **68.2 per cent and 1.605 MiB** larger. The rule asks for at least 10 per cent and 1 MiB, so both thresholds are cleared with room. **Tauri v2 is the shell, and the desktop application is written in Rust.**

**This reverses the direction H1 pointed in, and that is what the sheet was written to find out.** H1 read 6.4 / 8.8 / 5.7 MB for Wails and 66.4 / 42.9 / 79.9 MB for Tauri — but H2a already said in its own words that those figures are *October against Yaak*, two unrelated applications, and not a measurement of what Smoozy would weigh on either. Measured like for like, the ordering is the other way round. Nothing was wrong with H1; it measured what it said it measured.

---

## What was built

Two throwaway applications, one on each candidate, holding the same thing: **one frameless window, the same bundled page, one no-op typed binding, no network request and no updater.** Neither ships and neither is signed for release.

**Every variable the sheet fixes was fixed.**

- **The same page, byte for byte.** Three files — `index.html`, `main.css`, `main.js` — copied unchanged into both projects. Their digests are below and they match on all three systems.
- **The same source icon.** One 1024-pixel square, generated once. Each framework makes its own icon set from it, which is the framework's own packaging behaviour and part of what is being measured.
- **The same processor target on all three systems: x86_64.** On the Mac runner both were cross-built for x86_64 and both artifacts were confirmed to be x86_64 afterwards.
- **Both unsigned, one exception recorded below.** No certificate, no notarization, no installer, no package metadata on either side.
- **The system web view excluded from both.** On Windows the Wails build was made with the WebView2 payload left out and the Tauri build with its installer step set to skip, so on both the runtime is an external dependency and not artifact bytes.
- **The same archive, made the same way.** One packing step for both candidates on all three systems: sorted entries, fixed timestamps, deflate at level nine, executable bit kept. So an archive figure is comparable across candidates and across systems.
- **Release builds with the development surfaces absent.** No inspector, no reload server, no source maps, symbols stripped on both.

The throwaway is at `App/Shell measurement/`, the run is driven by `.github/workflows/shell-measurement.yml`, and the source commit is `a39a0f73`.

## Where it was built

**One fresh runner image per system, as the sheet requires.** Nothing was built on the Creator's computer.

| System | Runner image | Image version | Processor |
|---|---|---|---|
| macOS | `macos26` | `20260728.0273.1` | Apple silicon host, both artifacts cross-built for x86_64 |
| Windows | `win25-vs2026` | `20260824.214.3` | x86_64 |
| Linux | `ubuntu24` | `20260831.293.1` | x86_64 |

**GitHub publishes no content digest for its runner images**, so the identity above is the name and version the image states about itself, together with the digest of the image's own description file. That is a stated limit, not a digest composed to fill the row: `macOS` `37517e5f3d…`, `Windows` `ae8801533b…`, `Linux` `5c00622b2c…`. *(The macOS runner carries no such file at the published path; its entry is the digest of the empty stand-in, and the name and version above are that system's whole identity here.)*

**The tools, identical on all three systems.** Go 1.27.1, Rust 1.98.0, Wails command-line tool v2.14.0, Tauri command-line tool 2.11.4.

**The frameworks.** Wails v2.14.0 — the newest version its own release listing carries; a `v2.15.0` tag exists with no release entry beside it, and the released version was taken. Tauri 2.11.5 with its build helper 2.6.3.

**The locks.** Wails' resolved module list, digest `6e7366bf1a06e3968bb90f8d6384ff99a570160170aeec5c1b2f36cb1895dd52`. Tauri's resolved package list, digest `c0e5adbb5d47a569027790c843a09cb72ff9cc6cfdad5385ccbd78183afde2df`. Both came out identical on all three systems.

**The page.** `index.html` `cac617a710b7853f…`, `main.css` `d8eae074d938cec8…`, `main.js` `820610ec81c8311d…`. The source icon: `9fc8c4a8bfddb7b0…`.

---

## The figures

**Downloadable archive bytes** — the figures the decision rests on.

| System | Wails v2 | Tauri v2 | Wails larger by |
|---|---|---|---|
| macOS | 3,690,956 (3.520 MiB) | 2,378,145 (2.268 MiB) | 1,312,811 — 55.2% |
| Windows | 4,149,389 (3.957 MiB) | 2,466,373 (2.352 MiB) | 1,683,016 — 68.2% |
| Linux | 3,767,499 (3.593 MiB) | 2,349,240 (2.240 MiB) | 1,418,259 — 60.4% |

**Complete payload bytes, unpacked.**

| System | Wails v2 | Tauri v2 | Wails larger by |
|---|---|---|---|
| macOS, complete bundle | 8,857,766 (8.447 MiB) | 6,642,165 (6.334 MiB) | 2,215,601 — 33.4% |
| Windows, complete installed directory | 10,234,368 (9.760 MiB) | 8,573,952 (8.177 MiB) | 1,660,416 — 19.4% |
| Linux, complete payload | 9,076,872 (8.656 MiB) | 6,867,136 (6.549 MiB) | 2,209,736 — 32.2% |

**The executable alone, macOS**: Wails 8,828,608, Tauri 6,625,072. On Windows and Linux the payload is the executable and nothing else, so those figures are the table above.

**Each artifact's digest**, so either result can be reproduced and checked:

| System | Wails archive | Tauri archive |
|---|---|---|
| macOS | `df849cb86b1388a1…` | `3cc650bf0c7a93ea…` |
| Windows | `947ab213122c3848…` | `dcee472d4b2a1382…` |
| Linux | `90e9d1332ca7486b…` | `3ff962f02cc824fc…` |

## The decision, by the sheet's own rule

**Each candidate's decision figure is the largest of its three downloadable artifacts**, because all three must ship. For both candidates that is Windows: **Wails 4,149,389 bytes against Tauri 2,466,373 bytes.**

**Wails is larger by 1,683,016 bytes — 1.605 MiB and 68.2 per cent.** The sheet calls a result *materially smaller* only when the larger candidate is at least 10 per cent **and** 1 MiB larger at that comparison. Both hold, so this is not a tie and the named properties do not have to break it.

**The result does not depend on which figure is chosen.** Tauri is smaller on all three systems, in both the archive and the unpacked payload, by margins from 19.4 to 68.2 per cent. No single measurement carries the answer.

## What the two artifacts ask of the operating system

Read off the built binaries themselves rather than from any publication.

- **macOS** — both link the system's own WebKit framework, plus ordinary system frameworks. Neither carries a web engine.
- **Linux** — both link `libwebkit2gtk-4.1`, `libgtk-3` and `libsoup-3.0`. Same web view, same toolkit, same external dependency for both.
- **Windows** — Tauri's import table names twenty system libraries. **Wails' names one, `kernel32.dll`, and that is not the same claim**: a Go binary loads what it needs while running rather than declaring it up front, so the Windows list understates Wails and the two columns are not comparable on that row. What is comparable, and settled, is that neither artifact carries the web view: on both, the Microsoft Edge WebView2 Runtime is an external dependency.

**The published minimum versions**, from each project's own pages, read 2026-09-02.

| | Wails v2 | Tauri v2 |
|---|---|---|
| macOS | 10.13 and later for a release build; 11.0 and later on Apple silicon | Catalina (10.15) and later |
| Windows | 10 and 11 | 7 and later |
| Linux | GTK3 and WebKitGTK; on Ubuntu 24.04, `libwebkit2gtk-4.1` with a build tag | `libwebkit2gtk-4.1` |

Wails: *Installation*, `https://raw.githubusercontent.com/wailsapp/wails/master/website/docs/gettingstarted/installation.mdx`, read 2026-09-02 — *"Windows 10/11 AMD64/ARM64"*, *"MacOS 10.15+ AMD64 for development, MacOS 10.13+ for release"*, *"MacOS 11.0+ ARM64"*. Tauri: *Prerequisites*, `https://v2.tauri.app/start/prerequisites/`, read 2026-09-02 — *"macOS Catalina (10.15) and later"*, *"Windows 7 and later"*. Label for both: **documented**.

## The one place the two artifacts were not treated alike

**Wails signs its macOS bundle for itself and Tauri does not.** The Wails bundle came back ad-hoc signed — an embedded signature of 68,533 bytes inside the executable and a 2,448-byte sealed-resources file beside it, about 71 KB in all. The Tauri bundle is unsigned. So the sheet's *both unsigned* row is met on Windows and Linux and not exactly on macOS.

**It decides nothing and is recorded rather than corrected.** Removing it would move Wails' macOS payload from 8,857,766 to roughly 8,786,785 bytes, against Tauri's 6,642,165 — a gap of 2.1 MiB either way. No threshold moves and no ordering moves.

## What this measurement does not say

- **It measures the shell and not Smoozy.** Both artifacts hold one window and one binding. What each framework costs as the real application grows is not established here, and nothing below should be read as predicting it.
- **Neither candidate was given release-profile tuning beyond stripped symbols.** Tauri's own pages recommend further size settings for a release build; none was applied, to either candidate. Applying them would move Tauri's figures down and not Wails' up, so the direction of the result is safe, but the margin would change.
- **The macOS figures are cross-built for x86_64 on an Apple-silicon host.** Both were, identically. An Apple-silicon artifact was not measured.
- **One run, one image per system.** Nothing was built twice, so nothing here says how stable a figure is between runs.
- **Nothing was launched.** Both were built and measured; neither was started. That both artifacts run is not claimed by this document.

---

## The required property check on Tauri

H6a says in its own words that whether Tauri publishes the frameless facility *"has deliberately not been read; that check is required if H5b places Tauri first"*. It does, so it was read. **Every source below is Tauri's own documentation, read 2026-09-02. Label: documented.**

**H6's eight properties, against Tauri v2.**

1. **One codebase, real release artifacts per system.** Held — three artifacts were built and measured separately above.
2. **A compiled binary, not a bundled engine.** Held — H1 already classes Tauri as a compiled system-web-view route, and the built binaries link the system web view rather than carrying one.
3. **The operating system's own web view on all three.** Held — WebKit on macOS, WebKit2GTK on Linux, WebView2 on Windows, read off the binaries.
4. **A signed downloadable binary on all three.** Held — Tauri publishes a signing page for each of macOS, Windows and Linux.
5. **A licence permitting a closed-source download.** Held — Tauri publishes its terms as *Apache-2.0 OR MIT*.
6. **The platform's native menu bar and native file picker.** Held — *Window Menu*, `https://v2.tauri.app/learn/window-menu/`: *"Native application menus can be attached to both to a window or system tray. Available on desktop."* And *Dialog*, `https://v2.tauri.app/plugin/dialog/`: *"Native system dialogs for opening and saving files along with message dialogs"*, with Windows, Linux and macOS all listed as supported.
7. **Documented signing and notarization support.** Held — *macOS Code Signing*, `https://v2.tauri.app/distribute/sign/macos/`, carries a Notarization section naming both routes Apple offers; *Windows Code Signing*, `https://v2.tauri.app/distribute/sign/windows/`, covers the other.
8. **A frame that follows the system's light-or-dark setting, or nothing set at all.** Held, and on exactly the same terms as Wails — the configuration reference at `https://v2.tauri.app/reference/config/` says of the window's theme: *"The initial window theme. Defaults to the system theme. Only implemented on Windows and macOS 10.14+."* So two systems follow the setting and the third leaves it to the window manager, which is the outcome property 8 was written to allow. Nothing forces a theme.

**H6a's own facility — giving up the frame and drawing the title bar — is published in full.** From *Window Customization*, `https://v2.tauri.app/learn/window-customization/`, and the configuration reference:

- **The frame is given up by one setting.** `decorations`, a true-or-false value: *"Whether the window should have borders and bars."* Default true. Like Wails' equivalent, it carries no per-system table.
- **The drag region is a page attribute.** `data-tauri-drag-region`, put on the element that should drag the window. **One difference from Wails is worth carrying into the build**: Tauri states *"data-tauri-drag-region will only work on the element to which it is directly applied. If you want the drag behavior to apply to child elements as well, you'll need to add it to each child individually."* Wails' equivalent applies to child elements. So the drawn title bar marks its own drag elements rather than marking one parent.
- **The three buttons are the application's own and are wired to published calls** — `minimize()`, `toggleMaximize()` and `close()` on the current window, with the permissions each needs listed by name.
- **macOS keeps a second route**, the transparent title bar, exactly as Wails does — so the choice the architecture already recorded for the Mac survives the change of shell.
- **One cost is published and is the same class Wails already stated.** Tauri: *"For macOS, using a custom titlebar will also lose some features provided by the system, such as moving or aligning the window."* Wails' pages make no statement about what a native frame carries, which the architecture recorded as an unknown. Tauri names part of it. **That is a narrowing of a known cost and not a new one**, and it changes no promise: what is lost is a window-manager gesture, not anything Smoozy holds.

**The development surfaces are absent from a release build by the shell's own rule.** The configuration reference on the web inspector: *"This API works in debug builds, but requires devtools feature flag to enable it in release builds."* The flag was not set, so H35's requirement is met by construction on this shell too.

**No property fails.** The choice does not move down the contingency list.

---

## What follows from it

**The desktop application is written in Rust**, because that is the language Tauri is built in and the shell decides it.

**The server package is written in Rust too, and that is a decision taken here.** The plan for this step asks for the server package's language to be settled beside the desktop's, so there is one toolchain rather than two. What the server package has to be is the constraint that settles it: a complete versioned thing placed on a rented machine with nothing installed on it, holding executables that run before any Smoozy component exists there. Rust produces a single self-contained executable with no interpreter and no runtime to install, and a fully static build against musl removes the system library floor as well. One language covers both sides.

**Nothing else in the architecture moves.** The update route was never a framework facility on any of the three systems — H8 already says a framework that ships no updater costs work and not a promise — so changing shells costs nothing there. The window, the title bar, the buttons and the colour-scheme reading are all published by this shell as well.

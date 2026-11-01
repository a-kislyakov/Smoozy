# Shell measurement

This folder holds the throwaway used to fill the comparison sheet at `Preparation/Architecture/Desktop app and delivery.md` H5b — the like-for-like Wails v2 against Tauri v2 size comparison that Build owes Architecture before either name becomes Smoozy's shell.

**It is not a module and it is not part of the app.** Nothing here ships, nothing here is signed, and no other code depends on it. It is kept rather than deleted because H5b requires the result to be reproducible from the recorded source, locks, commands, runner images and artifact digests.

What is in it:

- `web/` — the one page both candidates bundle. The same three files go into each, byte for byte.
- `appicon.png` — the one source icon. Each framework makes its own icon set from it.
- `wails/` — the Wails v2 project: one frameless window, one no-op typed binding, no network request, no updater.
- `tauri/` — the Tauri v2 project, the same window and the same binding.
- `tools/measure.py` — the measuring step, identical for both candidates: the payload's own byte total, one zip at fixed settings, and what the binary asks the operating system for.

The builds run on fresh GitHub runner images, one per system, driven by `.github/workflows/shell-measurement.yml`. Nothing is built on the Creator's computer, because H5b asks for one fresh runner image per operating system.

The filled sheet and the reasoning behind it are in `Preparation/Build/Shell measurement.md`.

// **The page's entry, and the page's own file layout rule.**
//
// **READ THIS BEFORE WRITING ANY PAGE CODE. It is the rule the seven pieces of
// step 10a after piece A follow, and the eleven screens after them. It was
// settled on 2026-09-08 by piece A and written down rather than left to be
// discovered by the third agent that hits a conflict.**
//
// ## Why there is a rule
//
// Until this date the page was three files and 342 lines, all of it step 1's:
// one index.html, one smoozy.js, one smoozy.css. **Eight pieces and then eleven
// screens all write into `web/`.** Eight agents editing two files will not
// merge, and the merges they lose are the ones nobody notices — a handler
// dropped, a rule overwritten by a rule that looks like it.
//
// ## The rule
//
// **Three files under `web/` are shared, and each takes a one-line addition
// and nothing else.**
//
//  1. `index.html` — **a piece adds nothing at all.** It holds the window's own
//     title bar, one empty `<div class="pane" id="pane">` and this one script
//     tag. A piece that needs elements makes them in its own module.
//  2. `smoozy.js` — **the import list below. One line.**
//  3. `smoozy.css` — **the `@import` list at its head. One line.**
//
// **Everything else a piece writes is its own pair of files**, named for the
// service it draws rather than for the screen it appears on:
//
//     web/<service>.js     its own module: what it draws and what it listens for
//     web/<service>.css    its own rules, imported from smoozy.css
//
// The Rust side matches, one file for one file: `src/surface/<service>.rs`
// holds that service's commands, and its own head carries the other half of
// this rule.
//
// ## The four things that bind whatever a piece writes
//
// - **Results arrive as events, never as the answer to a call.**
//   `Desktop app and delivery` P6: results reach the interface as events on one
//   ordered queue, and the interface holds no other route to a module. A
//   command is the page **asking**; an event is the application **telling**. A
//   piece that draws a module's outcome from a command's return value has built
//   the second route P6 forbids. `./event-route.js` is the only way in.
// - **Values are content, never markup** — H32. `textContent`, `setAttribute`,
//   `createElement`, `createElementNS`. Never `innerHTML`, never a string
//   turned into elements, never an event-handler attribute. The mockup uses
//   `innerHTML` in one place and 87 inline `style=` attributes, and every one
//   of those is inert here.
// - **No inline script and no inline style** — H33. `script-src 'self'` and
//   `style-src 'self'`, so both are separate files served from the packaged
//   origin. There is no bundler, no framework and no build step in this
//   project, and the first one added would be the first one there has ever
//   been. ES modules from the packaged origin need none of that.
// - **Nothing here polls and nothing here repeats on a timer** — P9 and P10.
//   The one standing timer this application is allowed is the update check, and
//   it is not built. A drawing may tick against a moment the application
//   already published and may do nothing else (P10b).
//
// ## The import list. A piece adds one line.

import { theChromeIsDrawn } from "./chrome.js";
import { theRouteIsOpen } from "./event-route.js";
import "./panes.js";
import "./menus.js";
import "./the-failure-shape.js";
import "./the-underway-shape.js";
import "./said-once.js";
import "./the-machines-own-words.js";
import "./the-rail.js";
import "./keys.js";
import "./the-queue.js";
import "./the-history.js";
import "./servers.js";
import "./a-server.js";
import "./render-settings.js";
import "./the-dependency-check.js";
import "./files.js";
import "./set-up.js";
import "./transfers.js";
import "./settings.js";
import "./telegram.js";
import "./reporting.js";
import "./the-return-key.js";
import "./the-escape-key.js";
import "./the-typed-text.js";
import "./the-scrolled-place.js";
import "./the-value-drawn.js";
import "./a-new-job.js";

// Whatever happens, the window is drawn and left standing. A window that shows
// nothing is Smoozy going quiet, which Rule 8 makes a fault however the trouble
// arose: an empty title bar can be reported, a blank one cannot explain itself.
async function start() {
  try {
    await theChromeIsDrawn();
  } catch (trouble) {
    document.documentElement.dataset.chrome = "failed";
    // The route is opened even where the chrome failed. They are separate
    // troubles and one going wrong is no reason for the other not to happen.
    theRouteIsOpen().catch(() => {
      document.documentElement.dataset.eventRoute = "failed";
    });
    throw trouble;
  }
  try {
    await theRouteIsOpen();
  } catch (trouble) {
    document.documentElement.dataset.eventRoute = "failed";
    throw trouble;
  }
}

start();

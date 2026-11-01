// **Files, on the page** — a server's main face and the largest single screen
// in the drawing. The application half is `src/surface/files.rs`, whose head
// carries the whole account of the piece.
//
// **Read the head of `web/a-server.js` first, and the head of
// `web/the-rail.js` before that.** This module is the first tab of the server's
// own screen: it registers under the drawing's own `files`, is drawn once for
// the life of the run, and is told every time it starts showing and every time
// the user goes to a different server while it is showing.
//
// ## Three things this file must never do
//
// - **Never write a word of prose into it.** A6 and X1: the typography point is
//   the only way prose reaches a screen and the enforcement is structural on
//   both sides. Every word below arrived from the application.
// - **Never join a value into a sentence.** X7. A path, a file's name, a count,
//   a job's name and a moment all arrive beside their words and are drawn as
//   separate nodes.
// - **Never compose a path.** A path is bytes (`File Browser` L1a), and every
//   path on this page arrives as `{ bytes, shown }`: the bytes are identity and
//   go back to the application untouched, the shown form is for a person to
//   read. **The folder above is composed by the application** and arrives on the
//   answer, because *Up one folder* is path arithmetic and path arithmetic is
//   not the page's.
//
// ## What this file does not build, because it is already built
//
// - **The head and the tab strip**, which are `web/a-server.js`'s and sit above
//   this host.
// - **The panel over a server.** Opening a `.blend`'s settings is
//   `theSettingsAreOpenedOn(server, path, savedWith)` from
//   `web/render-settings.js`, with the server this tab was handed, never with
//   whichever server happens to be current, and **the path is the row's exact
//   bytes** — never its display name (`File Browser` L1a). **The saved-with version goes with
//   it every time** — ruling 4 of 2026-09-13: a panel never handed that
//   version can never be read (`Render Settings` B2a, then `Render Settings`
//   B5a). **A page opening Render Settings on a file this tab did not hand
//   asks `theSavedWithVersionReadFor` for the read kept on these rows** —
//   step 10c's owed piece 2.
// - **The narrowing.** `web/panes.js` writes the step mark on to `.app` and the
//   drawing's own blocks are keyed to it. This tab has no seam of its own — the
//   pane-width record holds three and there is no fourth.
//
// **P9 and P10: nothing here polls and nothing here repeats on a timer.**
//
// ## The icons are made here rather than referred to
//
// The mockup draws every icon as `<use href="#i-folder">` against a `<symbol>`
// in its own document. **A piece adds nothing to `web/index.html`**, so each
// shape is made here from the mockup's own path data, unchanged. Thirteen of
// them: five file kinds and eight toolbar controls.

import { whenTold } from "./event-route.js";
import {
  aTabIsRegistered,
  aPanelIsOpenOver,
  FILES,
} from "./a-server.js";
import { theCheckIsOpenedOn } from "./the-dependency-check.js";
import { aTransferIsAskedFor, theTransfersAreShown } from "./transfers.js";

// **The acts on this screen that start a transfer and go to Transfers** —
// `Design/File Browser`: *"Transfers it starts are watched in Transfers."* The
// same strings `TheAct` in `src/surface/transfers.rs` takes. Copy carefully.
// **Upload left this list at step 10c piece 4**: an upload now runs,
// from this screen, through `src/uploading.rs`, and Transfers reads no running
// request yet.
const THE_ACTS_THAT_START_A_TRANSFER = ["download", "download-to"];
/// The folder menu's *Calculate size* (the twenty-third notes, note 2).
const THE_CALCULATE_SIZE_ACT = "calculate-size";
import { theSettingsAreOpenedOn } from "./render-settings.js";
import { theMarkOverAPane } from "./the-underway-shape.js";
import { theValueDrawn, theValueIsDrawnInto } from "./the-value-drawn.js";

const shell = window.__TAURI__;
const SVG_NS = "http://www.w3.org/2000/svg";

// Declared beside the service that posts it — `THE_ANSWER` in
// `src/surface/files.rs`. The two sides have to agree and nothing holds them
// together, so copy carefully.
const THE_ANSWER = "files:the-answer";

// `web/the-rail.js` names the same string, and `THE_USER_CHOSE_A_GOING` in
// `src/surface/menus.rs` is where it is posted.
const A_GOING = "menus:a-going";

// `UP_ONE_FOLDER` in `src/surface/files.rs`. **The one Go item that is a
// control on a screen rather than a place**, which is why it is not in the
// rail's own map of goings to places.
const UP_ONE_FOLDER = "go.up-one-folder";


// `THE_CONNECTION_IS_LENT` in `src/surface/render_settings.rs`, posted at the
// hand-over: the moment a connection starts serving, after `Servers`' own
// first checks, on every connection — the one a connect opens and the one a
// repair hands over. A test in `src/surface/files.rs` fails if the two drift.
const THE_CONNECTION_IS_LENT = "render-settings:the-connection-is-lent";

// ── What this screen holds for the life of the run ──────────────────────────
//
// **Everything here is one drawing's own elements, and nothing about the user's
// work is remembered.** The mounting contract's section 3 is why: the host stays
// for the life of the run, so a scroll position and an open menu survive going
// away and coming back for nothing.

let host = null;
let bars = null;
let crumbs = null;
let tools = null;
let list = null;
let bottom = null;
let dropzone = null;
// The one right-click menu that is open, where one is.
let theOpenMenu = null;
let boxes = null;

// Which server this tab is showing, handed over by `itIsShowing`. **Never
// guessed at**, because a panel opened for the wrong server is a panel that
// followed the user.
let server = null;
// The folder being browsed, as bytes. Run-held and nothing else holds it
// (`File Browser` N1).
let folder = null;
// The last answer, so a redraw after an act does not compose from two.
let said = null;
// Completed deletes invalidate older readings of the list.
let theListingChange = 0;
// **The rows the user chose, each by its name's bytes** — none, one or
// several. `Design/File Browser § Acting on files`: *"Rows are chosen the way
// a file manager chooses them"* (the Creator's thirteenth notes, note 6). Until
// 2026-10-18 this held one name.
let chosen = [];
// **The last row clicked**, by its name: a Shift-click's range, and a Shift
// with ↑ or ↓, run from it. Set by a plain click, a Cmd-click (Ctrl-click on
// Windows and Linux) and a plain ↑ or ↓; a Shift-click leaves it.
let theLastClicked = null;
// **The row at the moving end of the choice**, by its name: the one a plain
// ↑ or ↓ moves from and a Shift with either widens from.
let theMovingEnd = null;
// **A folder another screen asked this tab to open at** — `Server Setup` B12a's
// link from Set up. Taken the next time this tab shows, and then forgotten.
let theFolderAskedFor = null;
// **And the file in it to choose, by its name** — the file's name in a panel's
// title (`Design/Whole app § Panels over a server`, the Creator's ninth notes,
// note 5). Taken with the folder; nothing where only a folder was asked for.
let theNameAskedFor = null;
// **Whether this tab is the one showing**, between `itIsShowing` and
// `itIsPutAway`, so a folder handed over while it shows is taken at once
// rather than at a showing that is not coming.
let isShowing = false;
// **The chosen row a hand-over named is brought into view** when the rows
// of its folder are drawn, once.
let theHandedRowIsOwedAView = false;
// The server the folder and the chosen row belong to, so a move to another
// server starts that server's browser fresh (must-do 23).
let theServerTheyBelongTo = null;
// **The line above the list belongs to the server it was said on**
// (`Design/File Browser § The list`; the Creator's notes 2 and 3 on 0.1.8).
// Which server the line showing was said on, so going to another server
// clears it.
let theLinesServer = null;
// **And to the folder it was said in** — `Design/File Browser § The list`, the
// runner's ruling (c) of 2026-10-03: opening another folder clears it, and a
// new act's line replaces it. The folder showing when the line was said, as
// bytes; nothing where it was said before any folder was drawn, and then the
// first folder drawn is the one it belongs to. The Vast pass before 0.1.9
// watched a line follow the user into other folders.
let theLinesFolder = null;
// **An ending said while its server's Files was not showing**, kept for that
// server: one sentence per server, the newest only, shown once the next time
// that server's Files is shown. Sentences only, never the user's work, and
// held for the run alone.
const theEndingsKept = new Map();

// ── The one place a value becomes a node ────────────────────────────────────

function textNode(text) {
  return document.createTextNode(text || "");
}

// **What stands between a figure and the words after it, where the application
// marks them bound** — U+00A0, so *24 GB* or *2 jobs* never parts across two
// lines. `Design/Whole app § Typography`; the character's one home is
// `src/typography.rs` (`BETWEEN_A_FIGURE_AND_ITS_UNIT`), and this page writes
// it only where the mark says, never by reading which word is a unit.
const BETWEEN_A_FIGURE_AND_ITS_UNIT = "\u00A0";

/// **Draw a piece of prose with the values that belong beside it.**
///
/// The application hands over `{ pieces }`, each piece a word or a value and
/// never both, in the order the sentence reads. **This sets them side by side
/// and joins nothing**, which is the page half of X7.
function saidInto(holder, words, valueClass, selectPaths) {
  if (!words || !words.pieces) return holder;
  let first = true;
  for (const piece of words.pieces) {
    const node = document.createElement(piece.value != null ? "b" : "span");
    if (piece.value != null && valueClass) node.className = valueClass;
    if (selectPaths && piece.is_path) node.classList.add("selectable");
    // **A moment arrives as seconds and is rendered here**, in the user's own
    // zone — `File Browser` L4, and the same rendering the list's own column uses.
    if (piece.value != null && piece.moment) node.textContent = aCalendarTime(piece.value);
    // **A value reads in the order its bytes run** — `Desktop app and delivery`
    // X9, drawn by `web/the-value-drawn.js`.
    else if (piece.value != null) theValueIsDrawnInto(node, piece.value);
    else node.textContent = piece.said || "";
    // **Whether a piece follows the one before it with nothing between them is
    // the sentence's own fact**, and the application states it — a page
    // deciding it would be a page reading prose.
    if (!first && !piece.joins) holder.appendChild(textNode(piece.binds ? BETWEEN_A_FIGURE_AND_ITS_UNIT : " "));
    first = false;
    holder.appendChild(node);
  }
  return holder;
}

function saidAsText(words) {
  return saidInto(document.createElement("span"), words);
}

// ── The icons, from the mockup's own path data ──────────────────────────────

function anIcon(paths, options) {
  const svg = document.createElementNS(SVG_NS, "svg");
  svg.setAttribute("viewBox", "0 0 16 16");
  svg.setAttribute("aria-hidden", "true");
  const outlined = options && options.outlined;
  if (outlined) {
    svg.setAttribute("fill", "none");
    svg.setAttribute("stroke", "currentColor");
    svg.setAttribute("stroke-width", String((options && options.weight) || 1.3));
    svg.setAttribute("stroke-linejoin", "round");
    if (!options || options.caps !== false) svg.setAttribute("stroke-linecap", "round");
  } else {
    // **A solid icon is filled with the theme's own ink**, the colour `.fic`
    // gives it — `Design/File Browser § The list`: *"in the theme's own grey
    // in light and dark alike"*. Without it SVG's default fill is black, so
    // the folder, the `.blend` and the image's solid parts drew black in both
    // themes: the Creator's note 11 on 0.1.7, the Creator's notes piece 6. A
    // part that carries its own `fill="none"` keeps it.
    svg.setAttribute("fill", "currentColor");
  }
  for (const one of paths) {
    const node = document.createElementNS(SVG_NS, one.shape || "path");
    for (const [name, value] of Object.entries(one)) {
      if (name === "shape") continue;
      node.setAttribute(name, String(value));
    }
    svg.appendChild(node);
  }
  return svg;
}

// The five file kinds. **A folder is solid where a file is outlined**, which is
// what separates the two before either shape is read.
const THE_FILE_ICONS = {
  folder: () =>
    anIcon([
      {
        d: "M1.6 4.3c0-.7.5-1.2 1.2-1.2h3.3c.4 0 .8.2 1 .5l.8 1.1h5.3c.7 0 1.2.5 1.2 1.2v6.2c0 .7-.5 1.2-1.2 1.2H2.8c-.7 0-1.2-.5-1.2-1.2V4.3z",
      },
    ]),
  file: () =>
    anIcon(
      [
        { d: "M3.6 1.8h5.6L13 5.5v8.7a.9.9 0 0 1-.9.9H3.6a.9.9 0 0 1-.9-.9V2.7a.9.9 0 0 1 .9-.9z" },
        { d: "M9.1 2v3.4h3.4" },
      ],
      { outlined: true, weight: 1.2, caps: false },
    ),
  // The one icon that is not a page at all, and it is a plain cube: Smoozy uses
  // no Blender logo or mark of any kind.
  blend: () =>
    anIcon([
      { d: "M8 1.4l6.2 3.6L8 8.6 1.8 5z", opacity: ".95" },
      { d: "M1.8 5L8 8.6v6.9l-6.2-3.6z", opacity: ".5" },
      { d: "M14.2 5L8 8.6v6.9l6.2-3.6z", opacity: ".75" },
    ]),
  image: () => {
    const svg = anIcon([
      {
        shape: "rect",
        x: "1.4",
        y: "3",
        width: "13.2",
        height: "10",
        rx: "1.7",
        fill: "none",
        stroke: "currentColor",
        "stroke-width": "1.3",
      },
      { shape: "circle", cx: "5.3", cy: "6.4", r: "1.25" },
      { d: "M2.6 12.4l3.7-4.5 2.6 3.1 1.9-2 2.6 3.4z" },
    ]);
    return svg;
  },
  archive: () =>
    anIcon([
      {
        d: "M3.6 1.8h5.6L13 5.5v8.7a.9.9 0 0 1-.9.9H3.6a.9.9 0 0 1-.9-.9V2.7a.9.9 0 0 1 .9-.9z",
        fill: "none",
        stroke: "currentColor",
        "stroke-width": "1.2",
        "stroke-linejoin": "round",
      },
      {
        d: "M9.1 2v3.4h3.4",
        fill: "none",
        stroke: "currentColor",
        "stroke-width": "1.2",
        "stroke-linejoin": "round",
      },
      { shape: "rect", x: "5.9", y: "7.6", width: "2.6", height: "4.6", rx: ".5" },
      // **The mockup writes these four squares as one compound path.** They are
      // four here, byte for byte the same shape, because a `d` holding four
      // sub-paths reads to the page-wide prose sweep as two words a person
      // would read — and the sweep is right to be that blunt.
      { d: "M5.9 2.2h1.3v1.3H5.9z" },
      { d: "M7.2 3.5h1.3v1.3H7.2z" },
      { d: "M5.9 4.8h1.3v1.3H5.9z" },
      { d: "M7.2 6.1h1.3v1.3H7.2z" },
    ]),
};

// The eight toolbar shapes, on the same 16px box and one weight.
const THE_TOOL_ICONS = {
  upload: () =>
    anIcon(
      [
        { d: "M8 10.5V2.4" },
        { d: "M4.8 5.6 8 2.4l3.2 3.2" },
        { d: "M2.6 10.6v2.1a.9.9 0 0 0 .9.9h9a.9.9 0 0 0 .9-.9v-2.1" },
      ],
      { outlined: true, weight: 1.4 },
    ),
  download: () =>
    anIcon(
      [
        { d: "M8 2.4v8.1" },
        { d: "M4.8 7.3 8 10.5l3.2-3.2" },
        { d: "M2.6 10.6v2.1a.9.9 0 0 0 .9.9h9a.9.9 0 0 0 .9-.9v-2.1" },
      ],
      { outlined: true, weight: 1.4 },
    ),
  newfolder: () =>
    anIcon(
      [
        { d: "M1.9 12.4V4.1c0-.5.4-.9.9-.9h2.6c.3 0 .6.1.8.4l.7.9h4.3c.5 0 .9.4.9.9v1" },
        { d: "M1.9 12.4c0 .5.4.9.9.9h5.4" },
        { d: "M11.9 9.2v4.4" },
        { d: "M9.7 11.4h4.4" },
      ],
      { outlined: true },
    ),
  rename: () =>
    anIcon(
      [
        { d: "M2.4 11.1 10.6 2.9a1.4 1.4 0 0 1 2 2l-8.2 8.2-2.6.6z" },
        { d: "M9.2 4.3 11.7 6.8" },
      ],
      { outlined: true },
    ),
  cut: () =>
    anIcon(
      [
        { shape: "circle", cx: "4", cy: "12", r: "1.9" },
        { shape: "circle", cx: "12", cy: "12", r: "1.9" },
        { d: "M5.3 10.6 12.4 2.6" },
        { d: "M10.7 10.6 3.6 2.6" },
      ],
      { outlined: true },
    ),
  copy: () =>
    anIcon(
      [
        { shape: "rect", x: "5.5", y: "5.5", width: "8.2", height: "8.2", rx: "1.1" },
        { d: "M10.5 2.3H3.4a1.1 1.1 0 0 0-1.1 1.1v7.1" },
      ],
      { outlined: true, caps: false },
    ),
  paste: () =>
    anIcon(
      [
        { d: "M5.6 3.1H4a1 1 0 0 0-1 1v8.8a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V4.1a1 1 0 0 0-1-1h-1.6" },
        { shape: "rect", x: "5.6", y: "1.8", width: "4.8", height: "2.6", rx: ".8" },
      ],
      { outlined: true, caps: false },
    ),
  trash: () =>
    anIcon(
      [
        { d: "M2.6 4.2h10.8" },
        { d: "M6.4 4.2V2.9a.8.8 0 0 1 .8-.8h1.6a.8.8 0 0 1 .8.8v1.3" },
        { d: "M3.9 4.2l.6 8.5a1 1 0 0 0 1 .9h5a1 1 0 0 0 1-.9l.6-8.5" },
      ],
      { outlined: true },
    ),
};

// ── The tab ─────────────────────────────────────────────────────────────────

aTabIsRegistered({
  tab: FILES,

  itIsDrawn(into) {
    host = into;

    bars = document.createElement("div");
    host.appendChild(bars);

    crumbs = document.createElement("div");
    crumbs.className = "crumbs";
    host.appendChild(crumbs);

    tools = document.createElement("div");
    tools.className = "ftools";
    host.appendChild(tools);

    list = document.createElement("div");
    list.className = "flist";
    host.appendChild(list);

    bottom = document.createElement("div");
    bottom.className = "fbar";
    host.appendChild(bottom);

    dropzone = document.createElement("div");
    dropzone.className = "dropzone";
    dropzone.hidden = true;
    host.appendChild(dropzone);

    boxes = document.createElement("div");
    host.appendChild(boxes);

    theKeysAreLive();
    thePressesAreLive();
    theViewersKeysAreLive();
    theShellsOwnDropIsLive();
  },

  // **The contract's section 2: ask the application again.** It runs on the
  // first showing and again every time the user goes to a different server
  // while this tab is showing, which is the *another module moved the browser*
  // case arriving as an ordinary call rather than as a special one.
  //
  // **Moving to another server starts that server's browser fresh**: the
  // folder and the chosen row are the last server's, so they are let go of and
  // the application's own default applies (must-do 23 of the step 10a round 1
  // audit of these pieces). `File Browser` §12: *"one machine is browsed at a
  // time — the move replaces it."* **A folder another screen handed over
  // still wins**, which is Set up's folder link.
  itIsShowing(which) {
    // **A picture open over another server's files goes with it.**
    if (theViewer && server !== which) theViewerCloses();
    server = which;
    isShowing = true;
    // **The line belongs to the server it was said on**: arriving at another
    // server clears it, and an ending kept for this one is shown, once.
    if (theLinesServer !== which) theLineIsCleared();
    if (theServerTheyBelongTo !== which) {
      folder = null;
      theChoiceIsLetGo();
      theServerTheyBelongTo = which;
    }
    if (theFolderAskedFor) theFolderAskedForIsTaken();
    // **Said after the folder is settled**, so a kept ending belongs to the
    // folder this showing opens rather than to the last server's (ruling (c)).
    const kept = theEndingsKept.get(which);
    if (kept) {
      theEndingsKept.delete(which);
      theLineSays(kept.words, kept.act);
    }
    theBrowserIsAskedFor(folder);
  },

  itIsPutAway() {
    isShowing = false;
    theRightClickMenuIsShut();
    theViewerCloses();
    // **`Design/File Browser § The browser opened so the user can point at a
    // file`: leaving the browser for another screen is the same as the way
    // out**, and the caller is told nothing was picked. The 38's piece 2.
    if (said && said.bar && said.bar.way_out) thePickIsDropped();
  },
});

/// **A pick was put in hand by another screen** — *Submit again* on an entry
/// whose `.blend` is not where it was (`Rendering` A1a, `Rendering` A11). Where this tab
/// is already showing, it asks again so the bar is there now; otherwise it
/// asks as it shows. The 38's piece 2.
export function theBrowserIsAskedAgain() {
  if (host && server) theBrowserIsAskedFor(folder);
}

/// **The bar's way out**, and the same when the user leaves: nothing was
/// picked, and nothing is written.
function thePickIsDropped() {
  return shell.core.invoke("the_pick_is_dropped").catch(() => {});
}

/// **Open this tab at a folder** — `Server Setup` B12a: Set up's folder
/// column links into `File Browser` at that folder. **The folder is bytes**,
/// exactly as the application handed them to Set up; the caller then shows
/// this tab with `theCurrentTabIs(FILES)`, and the folder is taken as it shows.
///
/// **And with a file in it chosen, where a name is handed too** — the file's
/// name in a panel's title (`Design/Whole app § Panels over a server`, added
/// 2026-10-12, the Creator's ninth notes, note 5). The name is the file's own
/// bytes, as the application sent them, and is chosen as a click chooses a
/// row: by its name within the folder. **A name the folder no longer holds
/// chooses nothing**, and the folder opens all the same — the application
/// answers for no row it cannot find (`File Browser` L22). **Where this tab is
/// already showing that server** — the panel the name was pressed in covered
/// it — the folder is taken now, since no showing is coming.
export function theBrowserIsOpenedAt(which, folderBytes, nameBytes) {
  theFolderAskedFor = folderBytes || null;
  theNameAskedFor = (folderBytes && nameBytes) || null;
  theServerTheyBelongTo = which;
  if (isShowing && host && server === which && theFolderAskedFor) {
    theFolderAskedForIsTaken();
    theBrowserIsAskedFor(folder);
  }
}

/// **The folder handed over becomes the one browsed**, with the row it named
/// chosen, or none.
function theFolderAskedForIsTaken() {
  folder = theFolderAskedFor;
  if (theNameAskedFor) theOneRowIsChosen(theNameAskedFor);
  else theChoiceIsLetGo();
  theHandedRowIsOwedAView = Boolean(theNameAskedFor);
  theFolderAskedFor = null;
  theNameAskedFor = null;
}

// ── Asking, and drawing what comes back ─────────────────────────────────────

function theBrowserIsAskedFor(where) {
  if (!server) return;
  // **A row being typed belongs to the folder it was typed in.**
  if (editing && (!where || !folder || where.join(",") !== folder.join(","))) editing = null;
  // **The application is told the one name where one row is chosen**, and
  // none where several are: the toolbar is told the choice on the page as
  // it is drawn, whatever the answer composed for (`theToolbarIsToldTheChoice`).
  const asked = { server, folder: where || null, chosen: chosen.length === 1 ? chosen[0] : null };
  const change = theListingChange;
  shell.core
    .invoke("the_files_tab", asked)
    .then((answer) => {
      if (asked.server !== server || change !== theListingChange) return;
      said = answer;
      folder = answer.folder.bytes;
      theLineGoesWithItsFolder(folder);
      theBrowserIsDrawn(answer);
      // **The folder is being read, and the rows follow** — asked off the
      // drawing thread, and drawn only if the user is still where they asked.
      if (answer.a_reading_follows) theReadingIsAskedFor(asked);
    })
    .catch(() => {
      // **Nothing is drawn here**: the sentence would be the application's (A6,
      // X1) and the application is what did not answer, so there is none to
      // draw and this file may not write one.
      if (host) host.dataset.files = "failed";
    });
}

/// **The rows, read on the machine** — `File Browser` L1's one reading, on the
/// connection's reading session. **An answer that arrives after the user went
/// somewhere else is not drawn**, because it is about a folder they left.
function theReadingIsAskedFor(asked) {
  const change = theListingChange;
  shell.core
    .invoke("the_folder_is_read", asked)
    .then((answer) => {
      if (asked.server !== server || !said || change !== theListingChange) return;
      if (asked.folder && said.folder.bytes.join(",") !== asked.folder.join(",")) return;
      said = answer;
      folder = answer.folder.bytes;
      theLineGoesWithItsFolder(folder);
      theBrowserIsDrawn(answer);
      theTagsAreAskedFor(asked.server, answer.listing);
    })
    .catch(() => {
      if (host) host.dataset.files = "failed";
    });
}

/// **The `.blend` version tags, read behind the rows** — `File Browser` L7:
/// *the four fields show first and the version tag fills in as each prefix
/// arrives behind them*. **One file at a time**, so a folder the user opens
/// next is never queued behind a whole folder's tags, and **a tag that arrives
/// after the user went somewhere else is not drawn**. The tag is a value the
/// application read and is drawn as one; this file reads nothing itself.
///
/// The tag is kept on the row it belongs to, so the row's own presses hand it
/// to Render Settings (`theSettingsAreOpenedOn`) and a redraw from the listing
/// held keeps it.
async function theTagsAreAskedFor(which, listing) {
  if (!listing || listing.is !== "rows") return;
  for (const row of listing.rows) {
    if (!row.the_tag_follows) continue;
    if (which !== server || !said || said.listing !== listing) return;
    let tag = null;
    try {
      tag = await shell.core.invoke("the_saved_with_version_is_read", {
        server: which,
        path: row.path.bytes,
      });
    } catch (_) {
      return;
    }
    if (which !== server || !said || said.listing !== listing) return;
    row.the_tag_follows = false;
    if (!tag) continue;
    row.saved_with = tag;
    theTagIsDrawnOn(row);
  }
}

/// **The saved-with version Files read for one file, for a page that opens
/// Render Settings on a file Files did not hand it** — step 10c's owed piece
/// 2. Render Settings opened from its strip (`web/render-settings.js`), or on a
/// file a new job from an old one names (`web/a-new-job.js`), was handed none,
/// though this tab showed the tag: the application then had to read it again
/// over the line, and where no connection was live the page said the file
/// does not say which Blender saved it. **The read is the one kept on the
/// rows this tab is showing** — the tag drawn beside the row, or the one the
/// listing carried — found by the file's exact bytes on the same server.
/// **Blank where this tab holds no read of that file**: another server,
/// another folder, or a tag not yet arrived; the application's own fallback
/// then stands as before. Nothing is read here; `Render Settings` B2's *this
/// page never reads it for itself* stands.
export function theSavedWithVersionReadFor(which, path) {
  if (!which || which !== server || !said || !said.listing || said.listing.is !== "rows" || !path) return "";
  const wanted = Array.from(path).join(",");
  const row = said.listing.rows.find((one) => one.path.bytes.join(",") === wanted);
  return (row && row.saved_with) || "";
}

/// The tag, put on the row already drawn — beside the name, where `aRow`
/// draws one.
function theTagIsDrawnOn(row) {
  const wanted = row.name.bytes.join(",");
  for (const one of list.children) {
    if (theRowsNames.get(one) !== wanted) continue;
    const name = one.querySelector(".fname");
    if (!name || name.querySelector(".ver")) return;
    // Straight after the name, where `aRow` puts it.
    if (name.firstChild) name.firstChild.after(aTag(row.saved_with));
    else name.appendChild(aTag(row.saved_with));
    return;
  }
}

function aTag(version) {
  const tag = document.createElement("span");
  tag.className = "ver";
  tag.textContent = version;
  return tag;
}

function theBrowserIsDrawn(answer) {
  if (host) delete host.dataset.files;
  theChosenIsLetGoWhereTheListDoesNotShowIt(answer);
  theToolbarIsToldTheChoice(answer);
  theBarsAreDrawn(answer);
  theCrumbsAreDrawn(answer);
  theToolbarIsDrawn(answer);
  theListIsDrawn(answer);
  // **A row a hand-over chose is shown, not only marked** — a folder longer
  // than the list would otherwise hold it below the view, and the user would
  // not find the file the link was pressed to find. Once, on the first rows
  // drawn after the hand-over.
  if (theHandedRowIsOwedAView && answer.listing && answer.listing.is === "rows") {
    theHandedRowIsOwedAView = false;
    theChosenRowIsBroughtIntoView();
  }
  theBottomBarIsDrawn(answer);
  // **The right-click menus are made when one is asked for**, at the pointer.
  theRightClickMenuIsShut();
  theViewerFollowsTheLine();
}

// ── The bars along the top ──────────────────────────────────────────────────

function theBarsAreDrawn(answer) {
  bars.textContent = "";
  if (answer.stale) {
    const bar = document.createElement("div");
    bar.className = "stale-bar";
    const mark = document.createElement("span");
    mark.className = "mk";
    bar.appendChild(mark);
    bar.appendChild(saidAsText(answer.stale.said));
    bars.appendChild(bar);
  }
  if (!answer.bar) return;
  const bar = document.createElement("div");
  bar.className = "stale-bar find";
  const mark = document.createElement("span");
  mark.className = "mk";
  bar.appendChild(mark);
  const words = document.createElement("span");
  saidInto(words, answer.bar.said);
  if (answer.bar.still_uploading) {
    const more = document.createElement("span");
    more.textContent = answer.bar.still_uploading;
    words.appendChild(textNode(" "));
    words.appendChild(more);
  }
  bar.appendChild(words);
  if (answer.bar.way_out) {
    const grow = document.createElement("span");
    grow.className = "grow";
    bar.appendChild(grow);
    const out = document.createElement("button");
    out.type = "button";
    out.className = "mini";
    out.textContent = answer.bar.way_out;
    out.addEventListener("click", () => {
      thePickIsDropped().then(() => theBrowserIsAskedFor(folder));
    });
    bar.appendChild(out);
  }
  bars.appendChild(bar);
}

// ── The path along the top ──────────────────────────────────────────────────

function theCrumbsAreDrawn(answer) {
  crumbs.textContent = "";
  answer.path.forEach((part, at) => {
    if (at > 0) {
      const between = document.createElement("span");
      between.textContent = "/";
      crumbs.appendChild(between);
    }
    if (part.here) {
      const here = document.createElement("span");
      here.className = "here";
      // The root has no name of its own to show, so the application supplies a
      // word for it and the page draws whichever it was handed.
      if (part.said) here.textContent = part.said;
      else theValueIsDrawnInto(here, part.shown);
      crumbs.appendChild(here);
      return;
    }
    const step = document.createElement("button");
    step.type = "button";
    step.className = "mini";
    if (part.said) step.textContent = part.said;
    else theValueIsDrawnInto(step, part.shown);
    step.addEventListener("click", () => theBrowserIsAskedFor(part.folder.bytes));
    crumbs.appendChild(step);
  });
}

// ── The toolbar ─────────────────────────────────────────────────────────────

function aToolControl(label, shape, act, extra) {
  const control = document.createElement("button");
  control.type = "button";
  control.className = "ib";
  if (extra) control.classList.add(extra);
  // **Every icon carries its word on hover and on keyboard focus** — Design.
  control.setAttribute("aria-label", label);
  control.title = label;
  control.appendChild(shape());
  const word = document.createElement("span");
  word.className = "ibl";
  word.textContent = label;
  control.appendChild(word);
  if (act) control.addEventListener("click", act);
  return control;
}

function theToolbarIsDrawn(answer) {
  const toolbar = answer.toolbar;
  tools.textContent = "";
  tools.className = toolbar.the_blend_pair ? "ftools" : "ftools notblend";

  // Upload, and the question inside it where the picker cannot take both.
  const holder = document.createElement("span");
  holder.className = "uphold";
  const upload = aToolControl(toolbar.upload, THE_TOOL_ICONS.upload, () =>
    theUploadIsAskedFor("both"),
  );
  holder.appendChild(upload);
  tools.appendChild(holder);

  tools.appendChild(
    aToolControl(toolbar.download, THE_TOOL_ICONS.download, () => aTransferIsStarted("download")),
  );
  const first = document.createElement("span");
  first.className = "tdiv";
  tools.appendChild(first);
  tools.appendChild(
    aToolControl(toolbar.new_folder, THE_TOOL_ICONS.newfolder, () => theNewFolderIsStarted()),
  );
  // **Rename acts on one row and is greyed while several are chosen** —
  // `Design/File Browser`, the thirteenth notes, note 6 — by the attribute the
  // drawing's `.ib[disabled]` draws.
  const rename = aToolControl(toolbar.rename, THE_TOOL_ICONS.rename, () => theRenameIsStarted());
  if (severalAreChosen(answer)) rename.disabled = true;
  tools.appendChild(rename);
  const second = document.createElement("span");
  second.className = "tdiv";
  tools.appendChild(second);
  tools.appendChild(aToolControl(toolbar.cut, THE_TOOL_ICONS.cut, () => theCutOrCopyIsTaken("cut")));
  tools.appendChild(
    aToolControl(toolbar.copy, THE_TOOL_ICONS.copy, () => theCutOrCopyIsTaken("copy")),
  );
  // **Paste is offered only where the pasteboard's machine is the one being
  // browsed** — `File Browser` M15 — and it is absent rather than greyed
  // anywhere else.
  if (toolbar.paste) {
    const paste = document.createElement("button");
    paste.type = "button";
    paste.className = "ib";
    const label = saidAsText(toolbar.paste).textContent;
    paste.setAttribute("aria-label", label);
    paste.title = label;
    paste.appendChild(THE_TOOL_ICONS.paste());
    const count = document.createElement("span");
    count.className = "pb";
    count.textContent =
      (toolbar.paste.pieces || []).map((one) => one.value).filter(Boolean)[0] || "";
    paste.appendChild(count);
    const word = document.createElement("span");
    word.className = "ibl";
    word.textContent = label;
    paste.appendChild(word);
    paste.addEventListener("click", () => thePasteIsAsked(folder));
    tools.appendChild(paste);
  }
  const third = document.createElement("span");
  third.className = "tdiv";
  tools.appendChild(third);
  tools.appendChild(
    aToolControl(toolbar.delete, THE_TOOL_ICONS.trash, () => theDeleteIsAskedAbout(true), "danger"),
  );

  // **The two Smoozy controls come after Delete**, so the file manager's own
  // controls never move when a `.blend` is chosen — `Design/File Browser`, the
  // Creator's fifth notes, note 1. They are words rather than icons because
  // nobody has an icon for them, and their divider goes in front of them.
  const pair = document.createElement("span");
  pair.className = "blendonly";
  const divider = document.createElement("span");
  divider.className = "tdiv";
  pair.appendChild(divider);
  const settings = document.createElement("button");
  settings.type = "button";
  settings.className = "mini strong";
  settings.textContent = toolbar.render_settings;
  settings.addEventListener("click", () => theSettingsAreOpenedForTheChosen());
  pair.appendChild(settings);
  const check = document.createElement("button");
  check.type = "button";
  check.className = "mini strong";
  check.textContent = toolbar.check_dependencies;
  check.addEventListener("click", () => theCheckIsOpenedForTheChosen());
  pair.appendChild(check);
  tools.appendChild(pair);

  const grow = document.createElement("span");
  grow.className = "grow";
  tools.appendChild(grow);
  if (toolbar.how_many_chosen) {
    const note = document.createElement("span");
    note.className = "selnote";
    saidInto(note, toolbar.how_many_chosen);
    tools.appendChild(note);
  }

  // **Files looks unavailable while there is no connection**, marked rather
  // than failing on a click — `File Browser` L15. The drawing's own
  // `.files.offline` dims the list and the bar and takes the controls, and it
  // comes off on the next answer that marks nothing unavailable (must-do 21).
  host.classList.toggle("offline", answer.unavailable.length > 0);
}

// ── The list ────────────────────────────────────────────────────────────────

// **Which server and folder the list was last drawn for**, so a folder drawn
// for the first time starts at its top. Run-held, like `folder` above.
let theFolderTheListShows = null;

/// **A newly opened folder starts at its top.** The list was one element for
/// the life of the run and kept its scroll offset across drawings, so a folder
/// opened after another was scrolled was drawn under the old folder's offset,
/// and a click aimed at one row landed on another (step 10c's ninth pass on
/// Hyperstack). **So a new folder gets a new list element**, whose offset is
/// its top: setting the old one's offset back is not enough, because a tab that
/// is away is not laid out and an offset set on it is not kept — the page
/// harness measured exactly that. **The same folder drawn again keeps where the
/// user was**, which is the mounting contract's section 3 above. Added
/// 2026-09-28, step 10c's ninth-pass fix piece 5.
function theListIsDrawn(answer) {
  const shows = `${server}\u0000${(answer.folder && answer.folder.bytes || []).join(",")}`;
  const theSameFolder = theFolderTheListShows !== null && shows === theFolderTheListShows;
  if (theFolderTheListShows !== null && shows !== theFolderTheListShows) {
    const fresh = document.createElement("div");
    fresh.className = list.className;
    list.replaceWith(fresh);
    list = fresh;
    theListHoldsRows = false;
  }
  theFolderTheListShows = shows;
  // **Where the user had scrolled is not lost to a redraw either** —
  // `Design/Whole app § The window`: *"a list drawn again with the same things
  // in it keeps its place, and a list that gains or loses rows keeps the row
  // the user was looking at in view."* Step 10c's eleventh pass (*Smaller* 6):
  // every upload that ended read the browsed folder again (`File Browser` L12),
  // the rows went while it was read, and a folder receiving a thousand files
  // jumped to its top at every file. **The case this was first written for** is
  // a folder read once as it is opened; **the case it now also meets** is the
  // same folder read again under the user's eyes.
  //
  // **So the same folder being read again keeps its rows, under the cover**,
  // and the rows that come back are put where the user was: the row they were
  // looking at stays where it stood in the view, or — where it has gone — the
  // list keeps its offset.
  if (theSameFolder && theListHoldsRows && answer.listing.is === "being-read") {
    const already = list.querySelector(".fwait");
    if (already) already.remove();
    list.appendChild(theCoverWhileItIsRead(answer.listing));
    return;
  }
  // **And a reading of the same folder that comes back without rows does not
  // lose it either** (`Design/Whole app § The window`, the same sentence) —
  // the connection lent nothing, the folder was not read, the record would not
  // read. Step 10c's twelfth pass (*Smaller*): collapsing
  // the Render Settings panel showed the Files list back at its top. **The case
  // this was first written for** is rows drawn over rows, where the place is
  // read off the list itself; **the case it now also meets** is rows drawn over
  // a sentence, where the list holds nothing to read it off. So the place is
  // kept while the sentence stands — the user cannot scroll a sentence — and
  // the rows that come back are put there.
  const place = theSameFolder ? (theListHoldsRows ? thePlaceIn(list) : theKeptPlace) : null;
  theKeptPlace = place;
  theListingIsDrawn(answer);
  thePlaceIsKept(list, place);
}

// **Whether the list holds the rows of the folder it shows**, so the same
// folder read again can leave them where they are.
let theListHoldsRows = false;
// **Where the user was in the folder the list shows**, held only while that
// folder's rows are not on screen, and let go of when another folder is shown.
let theKeptPlace = null;
// **Each drawn row's name, by its bytes**, so the row the user was looking at
// is found again among the rows drawn after it.
const theRowsNames = new WeakMap();
const theRowsPaths = new WeakMap();

/// **The row at the top of the view, and how far into the view it sits** —
/// nothing where the list is at its top, which a redraw keeps by itself.
function thePlaceIn(within) {
  const top = within.scrollTop;
  if (!top) return null;
  for (const one of within.children) {
    const name = theRowsNames.get(one);
    if (name === undefined) continue;
    if (one.offsetTop + one.offsetHeight > top) return { name, from: one.offsetTop - top, top };
  }
  return { name: undefined, from: 0, top };
}

function thePlaceIsKept(within, place) {
  if (!place) return;
  let again = null;
  if (place.name !== undefined) {
    for (const one of within.children) {
      if (theRowsNames.get(one) === place.name) {
        again = one;
        break;
      }
    }
  }
  within.scrollTop = again ? again.offsetTop - place.from : place.top;
}

/// **A file listing being read is Design's own example of *over the pane*** —
/// `Design/Whole app § Anything that isn't instant shows that it's underway` —
/// **so it carries the shared shape**, `theMarkOverAPane()`, wearing the
/// drawing's own `.fwait` cover and `.fwb` line, with the one turning mark
/// every other screen draws put in the line where the drawing put its dot. The
/// drawing's `.ckdot` names an animation, `wksp`, that neither the drawing nor
/// the page defines, so it stood still, and the eighth and ninth passes saw
/// words and no mark. Added 2026-09-28, step 10c's ninth-pass fix piece 5.
function theCoverWhileItIsRead(listing) {
  const waiting = theMarkOverAPane();
  waiting.classList.add("fwait");
  const said = document.createElement("span");
  said.className = "fwb";
  const turning = waiting.querySelector(".spin");
  if (turning) said.appendChild(turning);
  saidInto(said, listing.said, "mono");
  waiting.appendChild(said);
  return waiting;
}

function theListingIsDrawn(answer) {
  list.textContent = "";
  theHeadersAreDrawn(answer.headers);
  const listing = answer.listing;
  theListHoldsRows = listing.is === "rows";
  if (listing.is === "rows") {
    // **A new folder's row sits at the top while it is typed** (step 10c's
    // Files' owed acts piece 3).
    if (editing && editing.kind === "new-folder") list.appendChild(anEditingRow(null));
    for (const row of listing.rows) {
      // **`File Browser` L21: a hidden name is left out while the box is
      // unticked**, and the row is still held, so a tick draws it again from
      // here without asking anything.
      if (!theListShows(row, answer)) continue;
      const beingRenamed =
        editing && editing.kind === "rename" && row.path.bytes.join(",") === editing.path.join(",");
      const drawn = beingRenamed ? anEditingRow(row) : aRow(row);
      theRowsNames.set(drawn, row.name.bytes.join(","));
      theRowsPaths.set(drawn, row.path.bytes.join(","));
      list.appendChild(drawn);
    }
    if (theFieldToTakeTheKeys) {
      theFieldToTakeTheKeys.focus();
      theFieldToTakeTheKeys.select();
      theFieldToTakeTheKeys = null;
    }
    return;
  }
  if (listing.is === "being-read") {
    list.appendChild(theCoverWhileItIsRead(listing));
    return;
  }
  // **Every other reading is a sentence and no rows**, which is the honest
  // drawing: an empty list would say the folder is empty.
  const instead = document.createElement("div");
  instead.className = "fwait";
  const said = document.createElement("span");
  said.className = "fwb";
  if (listing.is === "nothing-has-been-listed") {
    const lead = document.createElement("b");
    lead.textContent = listing.lead;
    said.appendChild(lead);
    said.appendChild(textNode(" "));
    const rest = document.createElement("span");
    rest.textContent = listing.rest;
    said.appendChild(rest);
  } else if (listing.is === "the-folder-was-not-read") {
    // **`File Browser` P1's answer, in as many words**, with the folder named
    // as a value where the walk named one.
    saidInto(said, listing.said, "mono");
  } else {
    const one = document.createElement("span");
    one.textContent = listing.said;
    said.appendChild(one);
  }
  instead.appendChild(said);
  list.appendChild(instead);
}

function theHeadersAreDrawn(headers) {
  const head = document.createElement("div");
  head.className = "frow fhead";
  const spacer = document.createElementNS(SVG_NS, "svg");
  spacer.setAttribute("class", "fic");
  spacer.setAttribute("aria-hidden", "true");
  head.appendChild(spacer);
  // **The headers sort** (`Design/File Browser § The list`; hole 8 of the 38,
  // piece 13). The header the application says the list is sorted by wears
  // the drawing's `on` and its mark — ▲, or ▼ once a second press reversed it
  // (the drawing's `browse` writes *Name ▲*). **A press tells the application
  // which header was pressed, and the rows held are put in its order**
  // (`theListIsSortedBy`): the order is the application's, held for the run.
  // This file sorts nothing itself — a name is bytes, and how it is compared
  // is the application's.
  const one = (label, column, extra) => {
    const sort = document.createElement("button");
    sort.type = "button";
    sort.className = "sortb";
    if (extra) for (const one of extra.split(" ")) sort.classList.add(one);
    const on = headers.sorted_by === column;
    if (on) sort.classList.add("on");
    sort.textContent = on ? `${label} ${headers.reversed ? "▼" : "▲"}` : label;
    sort.addEventListener("click", () => theListIsSortedBy(column));
    head.appendChild(sort);
  };
  one(headers.name, "name");
  one(headers.kind, "kind", "kind");
  one(headers.size, "size", "r");
  one(headers.changed, "changed", "r when");
  list.appendChild(head);
}

/// **A sort reorders the rows held and never reads the folder again** —
/// `Design/File Browser § The list` (`File Browser` L14 and `File Browser` L21), the
/// Creator's twenty-first notes, note 2: *"simply rearranging stuff already
/// loaded on screen shouldn't reload the folder altogether."* The press is
/// told to the application, then the rows this tab holds are handed back as
/// the application composed them and put in its order, as *Show hidden
/// files* re-composes from the listing held. The rows stay the same rows, so
/// the chosen ones stay chosen and a tag still being read lands on its row.
/// Until 2026-10-29 a press asked for the folder again.
function theListIsSortedBy(column) {
  if (!server) return;
  shell.core
    .invoke("the_list_is_sorted_by", { by: column })
    .then(() => theRowsHeldArePutInOrder())
    .catch(() => {
      if (host) host.dataset.files = "failed";
    });
}

/// **The rows held, in the order the application holds** — and the headers
/// it marks. **An answer about rows this tab no longer holds is not drawn**:
/// a reading that arrived meanwhile was composed in the order already.
function theRowsHeldArePutInOrder() {
  const held = said;
  if (!held) return;
  const listing = held.listing;
  const rows = listing && listing.is === "rows" ? listing.rows : [];
  return shell.core
    .invoke("the_rows_are_put_in_order", { rows: rows.map((row) => row.sorts_by) })
    .then((answer) => {
      if (!answer || said !== held) return;
      if (listing && listing.is === "rows") {
        if (listing.rows !== rows || answer.order.length !== rows.length) return;
        listing.rows = answer.order.map((at) => rows[at]);
      }
      held.headers = answer.headers;
      theBrowserIsDrawn(held);
    });
}

/// **A cell that does not fit shows whole on hover** (`Design/Whole app`; the
/// Creator's note 9 on the test copy). The drawing's `.fname` and `.fmeta`
/// keep a row to one line and end a cell that does not fit in an ellipsis;
/// whether a cell was cut is the browser's to know, so it is asked as the
/// pointer arrives, and a cell that fits carries no hover text. `whole` is
/// the part of the cell whose text is shown — a name's own, not the version
/// tag or a refusal beside it.
function itShowsWholeOnHover(cell, whole) {
  cell.addEventListener("mouseenter", () => {
    cell.title = cell.scrollWidth > cell.clientWidth ? (whole || cell).textContent : "";
  });
}

function aRow(row) {
  // **A row that carries a refusal is not a button**, because the note beside
  // it holds a control of its own and a row that is one element swallows it.
  // Piece 3 paid for that lesson on the server list; it is applied here in the
  // same direction.
  const isPressable = !row.refusal && !row.editing;
  const element = document.createElement(isPressable ? "button" : "div");
  element.className = row.refusal && row.refusal.the_row_is_dimmed ? "frow denied" : "frow";
  // **With the box ticked, a hidden row is drawn grey** (the drawing's
  // `.frow.fhidden`; `Design/File Browser § The list`, the Creator's
  // twenty-first notes, note 3). Only its look changes; a row that carries a
  // refusal keeps the refusal's own look.
  if (row.hidden && !row.refusal) element.classList.add("fhidden");
  if (isPressable) element.type = "button";
  if (isChosen(row)) element.setAttribute("aria-current", "true");

  const icon = (THE_FILE_ICONS[row.icon] || THE_FILE_ICONS.file)();
  icon.setAttribute("class", "fic");
  icon.classList.add(row.icon);
  element.appendChild(icon);

  const name = document.createElement("span");
  name.className = row.editing ? "fname fediting" : "fname";
  if (row.editing) {
    const typed = document.createElement("span");
    typed.className = row.editing.refused ? "fld fedit bad" : "fld fedit";
    typed.textContent = row.editing.typed;
    name.appendChild(typed);
  } else {
    // A name drawn alone, kept apart from the version tag beside it (X9).
    const shown = theValueDrawn(row.name.shown);
    name.appendChild(shown);
    itShowsWholeOnHover(name, shown);
  }
  if (row.saved_with) name.appendChild(aTag(row.saved_with));
  if (row.editing && row.editing.refused) name.appendChild(aDenial(row.editing.refused, null, null));
  if (row.editing && row.editing.the_listing_saw_it) {
    const hint = document.createElement("span");
    hint.className = "fhint";
    saidInto(hint, row.editing.the_listing_saw_it);
    name.appendChild(hint);
  }
  if (row.refusal) {
    name.appendChild(aDenial(row.refusal.said, row.refusal.the_offer, row.refusal.dismiss));
  }
  element.appendChild(name);

  const kind = document.createElement("span");
  kind.className = "fmeta kind";
  kind.textContent = row.kind;
  itShowsWholeOnHover(kind);
  element.appendChild(kind);

  const size = document.createElement("span");
  size.className = row.size ? "fmeta r mono" : "fmeta r";
  // **Where a value would have stood and there is none, the drawing writes an
  // em dash.** It is punctuation rather than a word, so it is drawn here and
  // never composed into a sentence.
  size.textContent = row.size || "—";
  if (row.counted) theFolderSizeIsDrawnInto(size, row.counted);
  else itShowsWholeOnHover(size);
  element.appendChild(size);

  const changed = document.createElement("span");
  changed.className = "fmeta r when";
  changed.textContent = row.changed ? aCalendarTime(row.changed) : "—";
  itShowsWholeOnHover(changed);
  element.appendChild(changed);

  if (isPressable) {
    // **One click chooses any row — a folder, a file or a `.blend` — and asks
    // for nothing**; *chosen* is the name, matched against names within one
    // folder, and every act is handed the row's path through `theChosenRow()`
    // (step 10c's Files' owed acts piece 1). The Creator's eighth notes,
    // note 2: *"Single-click should just select an item in the file
    // browser."* Until 2026-10-12 one click on a folder opened it.
    //
    // **And it chooses the way a file manager chooses** (the thirteenth
    // notes, note 6): a plain click the row alone, Shift-click the range from
    // the last row clicked, Cmd-click (Ctrl-click elsewhere) one row more or
    // less — `theWayARowIsChosen`.
    element.addEventListener("click", (press) => {
      aRowIsChosen(row, theWayARowIsChosen(press));
      theChosenFileIsShown();
    });
    // **A double-click chooses its row alone and opens it**: a folder opens,
    // and a `.blend` opens its settings (`Design/File Browser`; the third
    // notes, note 7, the eighth notes, note 2, and the thirteenth, note 6). A
    // browser sends two clicks first, and both only choose.
    element.addEventListener("dblclick", () => {
      theOneRowIsChosen(row.name.bytes);
      theChosenFileIsShown();
      theRowIsOpened(row);
    });
    // **A right-click on a chosen row acts on every row chosen; on a row
    // outside the choice it chooses that row alone first** (the thirteenth
    // notes, note 6).
    element.addEventListener("contextmenu", (press) => {
      press.preventDefault();
      if (!isChosen(row)) {
        theOneRowIsChosen(row.name.bytes);
        theChosenFileIsShown();
      }
      theRightClickMenuIsShown(row, press);
    });
  }
  return element;
}

/// **A row chosen — a folder or a file — is drawn from the listing already
/// held** — the page's
/// own `said` — **and nothing is asked for**, of the application or of the
/// machine. Changed 2026-09-28, step 10c's transfer wave piece 1: the click
/// asked for the whole tab again, which read the folder on the machine again
/// and blanked the list while it did — about a hundred round trips on a
/// 10,000-entry folder, for a row being marked.
///
/// **What a chosen file changes is the rows marked and the toolbar**, and
/// the toolbar is told the choice as it is drawn (`theToolbarIsToldTheChoice`).
function theChosenFileIsShown() {
  if (!said) return;
  theBrowserIsDrawn(said);
}

// ── Choosing rows ───────────────────────────────────────────────────────────
//
// **`Design/File Browser § Acting on files`: "Rows are chosen the way a file
// manager chooses them."** Added 2026-10-18, the Creator's thirteenth notes,
// note 6. The choice is the page's own, held by name within one folder, and
// every act is handed the chosen rows' paths in the order the list draws them.
// The same choosing the dependency check's rows were given that day
// (`crate::surface::the_dependency_check`'s `the_act_is_taken`).

function theNameKey(bytes) {
  return bytes.join(",");
}

/// Whether a row is among the chosen, by its name.
function isChosen(row) {
  const wanted = theNameKey(row.name.bytes);
  return chosen.some((one) => theNameKey(one) === wanted);
}

/// **The chosen rows, in the order the list draws them** — only rows the list
/// shows (`File Browser` L22).
function theChosenRows(answer = said) {
  if (!server || chosen.length === 0 || !answer || !answer.listing || answer.listing.is !== "rows") return [];
  return answer.listing.rows.filter((row) => theListShows(row, answer) && isChosen(row));
}

/// Whether more than one row is chosen.
function severalAreChosen(answer = said) {
  return theChosenRows(answer).length > 1;
}

/// **Nothing chosen** — a click on the list's empty space, another server.
function theChoiceIsLetGo() {
  chosen = [];
  theLastClicked = null;
  theMovingEnd = null;
}

/// **One row chosen alone**, by its name, and the next range runs from it.
function theOneRowIsChosen(name) {
  chosen = [name];
  theLastClicked = theNameKey(name);
  theMovingEnd = theLastClicked;
}

/// **Which choosing a press is**: Shift for the range, the computer's own
/// adding key — Cmd on a Mac, Ctrl elsewhere — for one row more or less,
/// otherwise the row alone. The Mac is learned the one way this page learns
/// it (`onAMac`).
function theWayARowIsChosen(press) {
  if (press && press.shiftKey) return "range";
  if (press && (onAMac() ? press.metaKey : press.ctrlKey)) return "add-or-take";
  return "alone";
}

/// **A row chosen by a press.** A range runs from the last row clicked to
/// this one, in the list's drawn order, and replaces the choice; with nothing
/// chosen, or the last row clicked gone, it chooses this row alone. Cmd-click
/// adds the row or takes it away, and the next range runs from it.
function aRowIsChosen(row, how) {
  const here = theNameKey(row.name.bytes);
  if (how === "range" && chosen.length > 0 && theLastClicked) {
    const choosable = theRowsAClickCanChoose();
    const from = choosable.findIndex((one) => theNameKey(one.name.bytes) === theLastClicked);
    const to = choosable.findIndex((one) => theNameKey(one.name.bytes) === here);
    if (from >= 0 && to >= 0) {
      chosen = choosable.slice(Math.min(from, to), Math.max(from, to) + 1).map((one) => one.name.bytes);
      theMovingEnd = here;
      return;
    }
  }
  if (how === "add-or-take") {
    chosen = isChosen(row) ? chosen.filter((one) => theNameKey(one) !== here) : [...chosen, row.name.bytes];
    theLastClicked = here;
    theMovingEnd = here;
    return;
  }
  theOneRowIsChosen(row.name.bytes);
}

/// **The toolbar, told the choice** — `Design/File Browser § Acting on files`:
/// the note says how many (*1 selected*, *3 selected*), and the two `.blend`
/// controls show only while exactly one `.blend` is chosen (`File Browser`
/// N7a, read off the row's own `is_a_blend`). **Every word is the
/// application's**: one row's note is the toolbar's own `one_chosen`, and a
/// note for several is asked for once per count (`theNoteForHowMany`).
function theToolbarIsToldTheChoice(answer) {
  if (!answer || !answer.toolbar) return;
  const rows = theChosenRows(answer);
  answer.toolbar = {
    ...answer.toolbar,
    the_blend_pair: rows.length === 1 && Boolean(rows[0].is_a_blend),
    how_many_chosen:
      rows.length === 0 ? null : rows.length === 1 ? answer.toolbar.one_chosen : theNoteForHowMany(rows.length),
  };
}

/// **The note for several rows chosen, as the application composed it**, kept
/// for the run by count. A count not yet asked for is asked for here, and the
/// toolbar is drawn again when the words arrive, if that count is still the
/// choice; until then the note is absent rather than written on the page.
const theNotesForHowMany = new Map();
function theNoteForHowMany(count) {
  if (theNotesForHowMany.has(count)) return theNotesForHowMany.get(count);
  theNotesForHowMany.set(count, null);
  shell.core
    .invoke("the_words_for_how_many_are_chosen", { count })
    .then((words) => {
      theNotesForHowMany.set(count, words || null);
      if (!said || theChosenRows(said).length !== count) return;
      theToolbarIsToldTheChoice(said);
      theToolbarIsDrawn(said);
    })
    .catch(() => theNotesForHowMany.delete(count));
  return null;
}

/// **The refusal, in the drawing's own `.deny`** — the red dot, the plain
/// sentence, and the act that would try again where the machine allows one.
///
/// **It is not `theFailureShape`.** That builder takes a durable failure record
/// and a permission refusal is not one; the drawing's own comment on `.deny`
/// says this shape *"takes the whole-app shape: dot, plain sentence, then the
/// way out"*, which is what this draws.
function aDenial(words, offer, dismiss) {
  const deny = document.createElement("span");
  deny.className = "deny";
  const dot = document.createElement("span");
  dot.className = "fdot";
  deny.appendChild(dot);
  saidInto(deny, words);
  if (offer) {
    deny.appendChild(textNode(" "));
    const act = document.createElement("button");
    act.type = "button";
    act.className = "mini";
    act.textContent = offer;
    act.addEventListener("click", () => anActNeedsAConnection("elevated-retry"));
    deny.appendChild(act);
  }
  if (dismiss) {
    const cross = document.createElement("button");
    cross.type = "button";
    cross.className = "fx";
    cross.title = dismiss;
    cross.setAttribute("aria-label", dismiss);
    cross.textContent = "×";
    cross.addEventListener("click", () => theBrowserIsAskedFor(folder));
    deny.appendChild(cross);
  }
  return deny;
}

/// **The moment, rendered in the user's own zone from the epoch value alone** —
/// `File Browser` L4. **Nothing is subtracted from anything**: no age and no
/// elapsed time is derived from it against the desktop's clock, which
/// `State, identity and recovery` D5 forbids.
///
/// **The day, the month and the time, which is what the drawing writes** —
/// *6 Aug 14:02*. **The drawing's *today* and *yesterday* are not drawn**: they
/// are a machine reading compared against the desktop's own calendar, which is
/// the comparison `File Browser` L4 forbids, and the account carries the
/// ruling.
function aCalendarTime(seconds) {
  const at = new Date(Number(seconds) * 1000);
  if (Number.isNaN(at.getTime())) return "";
  return at.toLocaleString(undefined, {
    day: "numeric",
    month: "short",
    hour: "2-digit",
    minute: "2-digit",
  });
}

/// **The row the user chose, out of the answer already drawn.**
///
/// **The chosen row is held by its name and acted on by its path.** A name is
/// matched within one folder, which is what makes it enough to find the row
/// again; the path is what every caller is given, because a bare name means a
/// different thing on the machine — the fault the Gemini catch-up of step 10a
/// found on this screen, where a folder was opened by its name instead.
///
/// **The one row, and only where exactly one is chosen** — Rename, the two
/// `.blend` actions and opening each act on one row (the thirteenth notes,
/// note 6). The acts that take every chosen row ask `theChosenRows`.
function theChosenRow() {
  const rows = theChosenRows();
  return rows.length === 1 ? rows[0] : null;
}

/// **The chosen rows' paths, never their names**, in the list's drawn order.
function theChosenPaths() {
  return theChosenRows().map((row) => row.path.bytes);
}

/// **The render settings, opened on the chosen `.blend` with its own saved-with
/// version** — the toolbar's control and the right-click menu's item both come
/// here. The row is found by its bytes in the answer already drawn, so the path
/// and the version are the application's and nothing is composed on the page.
function theSettingsAreOpenedForTheChosen() {
  const row = theChosenRow();
  if (!row || !row.is_a_blend) return;
  theSettingsAreOpenedOn(server, row.path.bytes, row.saved_with).catch(() => {});
}

/// **The dependency check, opened on the chosen `.blend`** — the toolbar's
/// control and the right-click menu's item both come here, found the same way
/// as the settings above. A check is of a file, and a panel opened with a
/// server and no file can only say it does not know which file to check.
function theCheckIsOpenedForTheChosen() {
  const row = theChosenRow();
  if (!row || !row.is_a_blend) return;
  theCheckIsOpenedOn(server, row.path.bytes).catch(() => {});
}

// ── The bar along the bottom ────────────────────────────────────────────────

function theBottomBarIsDrawn(answer) {
  bottom.textContent = "";
  bottom.appendChild(saidAsText(answer.bottom.uploads_go_to));
  const grow = document.createElement("span");
  grow.className = "grow";
  bottom.appendChild(grow);
  if (answer.bottom.what_is_in_the_folder) {
    bottom.appendChild(saidAsText(answer.bottom.what_is_in_the_folder));
  }
  if (answer.bottom.free_space) {
    bottom.appendChild(saidAsText(answer.bottom.free_space));
  }
  // **The box is the bar's last thing**, so the count and free space, which
  // change width from folder to folder, never move it (`Design/File Browser §
  // The list`; the Creator's note 2 on the test copy).
  bottom.appendChild(theHiddenFilesBox(answer.bottom.show_hidden_files));
  // The last count this server had is drawn at once, so a redraw of the bar
  // does not drop it while the new one is asked for.
  if (theLastCountsServer === server) theCountIsDrawn(theLastCount);
  theTransfersAreCounted();
}

// ── How many transfers this server has running or waiting ───────────────────
//
// `Design/File Browser § The list`: *"While any transfer to or from this server
// is running or waiting, the bottom bar says how many — 3 transfers — as a link
// to Transfers."* Nothing when there are none. **The count is the
// application's, read from what Transfers lists for this server** and never
// counted here. It is asked again when the bar is drawn and whenever a transfer
// moves, one asking at a time. The Creator's note 10 on 0.1.7, the Creator's
// notes piece 6.

// Declared beside the service that posts it — `TRANSFERS_MOVED` in
// `src/surface/transfers.rs`. Copy carefully.
const TRANSFERS_MOVED = "transfers:moved";
let counting = false;
let countAgain = false;
let theLastCount = null;
let theLastCountsServer = null;

function theTransfersAreCounted() {
  if (!server || !bottom) return;
  if (counting) {
    countAgain = true;
    return;
  }
  counting = true;
  const which = server;
  shell.core
    .invoke("the_transfers_on_this_server", { server: which })
    .then((count) => {
      theLastCount = count;
      theLastCountsServer = which;
      if (which === server) theCountIsDrawn(count);
    })
    .catch(() => {})
    .finally(() => {
      counting = false;
      if (countAgain) {
        countAgain = false;
        theTransfersAreCounted();
      }
    });
}

function theCountIsDrawn(count) {
  let link = bottom.querySelector("button.ftrans");
  if (!count) {
    if (link) link.remove();
    return;
  }
  if (!link) {
    link = document.createElement("button");
    link.type = "button";
    link.className = "lnkb ftrans";
    link.addEventListener("click", () => theTransfersAreShown());
    // After *Uploads go to*, before the bar's gap — where the drawing puts it.
    bottom.insertBefore(link, bottom.querySelector("span.grow"));
  }
  link.textContent = "";
  saidInto(link, count);
}

whenTold(TRANSFERS_MOVED, () => {
  if (!host || host.hidden) return;
  theTransfersAreCounted();
});

// ── *Show hidden files* ─────────────────────────────────────────────────────
//
// `File Browser` §1.6, Row 520. **A checkbox at the bottom bar's right end,
// unticked until the user ticks it** (`Design/File Browser § The list`),
// drawn in the drawing's own `.mcheck fhid` with its `.cb` mark, which ticked
// carries the drawing's tick mark as every other box in Smoozy does.

/// **Whether the list shows this row** — `File Browser` L21. The row says
/// whether its name is hidden (`File Browser` L19, read off its bytes by the
/// application); the box says whether hidden names are shown. **A row that
/// carries a refusal is never left out** (`File Browser` L21 as amended
/// 2026-10-02), so a hidden folder the login may not read, reached directly,
/// still says so with the box off. Nothing else is asked.
function theListShows(row, answer) {
  return !row.hidden || Boolean(row.refusal) || Boolean(answer.bottom.show_hidden_files.ticked);
}

function theHiddenFilesBox(box) {
  const tick = document.createElement("label");
  tick.className = "mcheck fhid";
  const mark = document.createElement("span");
  mark.className = "cb";
  mark.setAttribute("role", "checkbox");
  mark.setAttribute("aria-checked", String(Boolean(box.ticked)));
  mark.classList.toggle("on", Boolean(box.ticked));
  mark.textContent = box.ticked ? "✓" : "";
  tick.appendChild(mark);
  tick.appendChild(textNode(" "));
  const word = document.createElement("span");
  word.textContent = box.label;
  tick.appendChild(word);
  tick.addEventListener("click", () => theHiddenFilesBoxIsTurned());
  return tick;
}

/// **The user ticked or unticked the box** — `File Browser` L20 and `File Browser` L21.
///
/// **The application writes the choice and nothing else, and the list is drawn
/// again from the answer already held**: the rows were all carried, the count
/// for the box turned the other way came with them, and so **the server is
/// never read** and the box works the same while the connection is down.
/// **A write the record did not take changes nothing here**, and the
/// application says so on this screen.
function theHiddenFilesBoxIsTurned() {
  if (!said) return;
  const now = !said.bottom.show_hidden_files.ticked;
  shell.core
    .invoke("show_hidden_files_was_ticked", { server: server || "", shown: now })
    .then((saved) => {
      if (!saved || !said) return;
      // **An answer that arrived in the meantime already carries the choice**,
      // having read it after the write, and turning it again would undo it.
      if (Boolean(said.bottom.show_hidden_files.ticked) === now) return;
      said = theAnswerWithTheBoxTurned(said, now);
      theBrowserIsDrawn(said);
    })
    .catch(() => {});
}

/// **The held answer, with the box turned** — the two counts swapped, and
/// nothing composed here.
function theAnswerWithTheBoxTurned(answer, now) {
  return {
    ...answer,
    bottom: {
      ...answer.bottom,
      show_hidden_files: { ...answer.bottom.show_hidden_files, ticked: now },
      what_is_in_the_folder: answer.bottom.what_is_in_the_folder_when_the_box_is_turned,
      what_is_in_the_folder_when_the_box_is_turned: answer.bottom.what_is_in_the_folder,
    },
  };
}

/// **`File Browser` L22: a name the list does not show is never chosen**, so no
/// act — Delete, Rename, Download, the right-click menu — is ever aimed at it.
/// Turning the box off lets go of a hidden chosen row at the same moment, and
/// takes *1 selected* and the `.blend` pair with it; turning it on chooses
/// nothing.
///
/// **A name the folder no longer holds is let go of too**, once the rows are
/// read — a deleted or renamed row — so the count says what is chosen. While
/// the folder is being read nothing is let go of: a name handed over waits
/// for its rows.
function theChosenIsLetGoWhereTheListDoesNotShowIt(answer) {
  if (chosen.length === 0 || !answer || !answer.listing || answer.listing.is !== "rows") return;
  const shown = new Set(
    answer.listing.rows.filter((one) => theListShows(one, answer)).map((one) => theNameKey(one.name.bytes)),
  );
  chosen = chosen.filter((one) => shown.has(theNameKey(one)));
  if (chosen.length === 0) theChoiceIsLetGo();
}

// ── A folder's size, counted on request ─────────────────────────────────────
//
// **The Creator's twenty-third notes, note 2** (`File Browser` L24 to
// `File Browser` L28). One request per chosen folder; each row fills in when
// its own answer lands. **The figure is kept on the row it belongs to**, as a
// `.blend` tag is, so it stays through a redraw of the rows held and **goes
// with the next reading of the folder**, whose rows are new (`File Browser`
// L27). An answer for a row no longer held is dropped.

function theFoldersAreCounted(folders) {
  const which = server;
  for (const one of folders) {
    if (one.counted && one.counted.calculating) continue;
    one.counted = { calculating: true };
    const path = one.path.bytes;
    shell.core
      .invoke("a_folders_size_is_counted", { server: which, path })
      .then((answer) => theCountLands(which, path, answer))
      .catch(() => theCountLands(which, path, null));
  }
  if (said) theBrowserIsDrawn(said);
}

function theCountLands(which, path, answer) {
  if (which !== server || !said || !said.listing || said.listing.is !== "rows") return;
  const key = theNameKey(path);
  const row = said.listing.rows.find((one) => theNameKey(one.path.bytes) === key);
  if (!row || !row.counted || !row.counted.calculating) return;
  if (!answer) {
    delete row.counted;
    theBrowserIsDrawn(said);
    return;
  }
  row.counted = answer;
  // **A sort by size uses the figure where a folder has one.**
  if (answer.sorts_by != null && row.sorts_by) row.sorts_by.size = answer.sorts_by;
  theBrowserIsDrawn(said);
  theRowsHeldArePutInOrder();
}

/// **The Size cell of a folder asked about**: *Calculating…*, the total,
/// *at least* the total, or *Failed* — the figure in the list's mono, the
/// words in its face, and what it counted on hover.
function theFolderSizeIsDrawnInto(cell, counted) {
  cell.textContent = "";
  cell.className = "fmeta r";
  if (counted.calculating) {
    saidInto(cell, said && said.calculating);
    return;
  }
  const pieces = (counted.cell && counted.cell.pieces) || [];
  if (pieces.length === 1 && pieces[0].value != null) cell.classList.add("mono");
  let first = true;
  for (const piece of pieces) {
    if (!first) cell.appendChild(textNode(piece.binds ? BETWEEN_A_FIGURE_AND_ITS_UNIT : " "));
    first = false;
    const node = document.createElement("span");
    if (piece.value != null && pieces.length > 1) node.className = "mono";
    if (piece.value != null) theValueIsDrawnInto(node, piece.value);
    else node.textContent = piece.said || "";
    cell.appendChild(node);
  }
  if (counted.hover && counted.hover.length > 0) {
    cell.addEventListener("mouseenter", () => theHoverLineIsShown(cell, counted.hover, "under"));
    cell.addEventListener("mouseleave", theHoverLineIsHidden);
  }
}

let theHoverLine = null;

/// **The drawing's `.fsztip`**: under a Size cell, its right edge on the
/// cell's, or beside a greyed menu item.
function theHoverLineIsShown(over, lines, where) {
  theHoverLineIsHidden();
  if (!host) return;
  const tip = document.createElement("span");
  tip.className = "fsztip";
  lines.forEach((line, at) => {
    if (at > 0) tip.appendChild(document.createElement("br"));
    saidInto(tip, line);
  });
  host.appendChild(tip);
  const box = over.getBoundingClientRect();
  const width = tip.getBoundingClientRect().width;
  const left = where === "beside" ? box.right + 8 : Math.max(8, box.right - width);
  const top = where === "beside" ? box.top + 3 : box.bottom + 6;
  tip.style.setProperty("--fsz-x", `${Math.round(left)}px`);
  tip.style.setProperty("--fsz-y", `${Math.round(top)}px`);
  theHoverLine = tip;
}

function theHoverLineIsHidden() {
  if (!theHoverLine) return;
  theHoverLine.remove();
  theHoverLine = null;
}

// ── The right-click menu ────────────────────────────────────────────────────
//
// **One menu, for the row that was pressed, at the pointer** — must-do 20 of
// the step 10a round 1 audit of these pieces. The drawing lays its three menus
// side by side over a shade, each under a caption, **to show them at once**;
// a menu that actually opens is one panel with no caption and no shade, and a
// press anywhere else or Escape closes it. **It is placed the way
// `web/servers.js`'s row menu is**: two custom properties the stylesheet reads,
// which are values rather than declarations (H33).

function theRightClickMenuIsShown(row, press) {
  theRightClickMenuIsShut();
  if (!said) return;
  // **On a row inside a choice of several, what is chosen is the thing
  // clicked, and the menu acts on all of it** (`Design/File Browser`, the
  // thirteenth notes, note 6). Several rows are not one folder to paste into
  // and not exactly one `.blend`, so the menu is the ordinary file's: download,
  // cut, copy, rename and delete, which every one of the three carries.
  const several = severalAreChosen();
  const which = several ? "file" : row.icon === "folder" ? "folder" : row.is_a_blend ? "blend" : "file";
  const menu = said.menus.find((one) => one.which === which);
  if (!menu) return;
  // **On a choice of several, *Calculate size* counts each chosen folder**
  // (`Design/File Browser`, the twenty-third notes, note 2), so where the
  // choice holds a folder the item joins the menu in its own group under the
  // downloads, as it stands on the folder menu.
  let items = menu.items;
  const theFolders = several ? theChosenRows().filter((one) => one.icon === "folder") : [row];
  if (several && theFolders.length > 0) {
    const folderMenu = said.menus.find((one) => one.which === "folder");
    const count = folderMenu && folderMenu.items.find((one) => one.act === THE_CALCULATE_SIZE_ACT);
    const at = items.findIndex((one) => one.act === "download-to");
    if (count && at >= 0) items = [...items.slice(0, at + 1), count, ...items.slice(at + 1)];
  }

  const panel = document.createElement("div");
  panel.className = "ctx fctx";
  panel.setAttribute("role", "menu");
  for (const item of items) {
    if (item.after_a_divider) panel.appendChild(document.createElement("hr"));
    const control = document.createElement("button");
    control.type = "button";
    control.setAttribute("role", "menuitem");
    if (item.at_the_head) control.className = "hd";
    if (item.in_red) control.className = "dg";
    // **Rename is greyed there**, in the drawing's own greyed menu item.
    if (several && item.act === "rename") {
      control.disabled = true;
      control.classList.add("off");
    }
    // **Calculate size is greyed where the server cannot count**, saying why
    // on hover; **while the listing is stale**; and **on a folder already
    // showing *Calculating…***.
    if (item.act === THE_CALCULATE_SIZE_ACT) {
      const stale = (said.unavailable || []).includes(THE_CALCULATE_SIZE_ACT);
      const counting = theFolders.every((one) => one.counted && one.counted.calculating);
      if (item.greyed || stale || counting) {
        control.disabled = true;
        control.classList.add("off");
        const why = item.greyed && item.greyed.why;
        if (why && !stale && !counting) {
          control.addEventListener("mouseenter", () => theHoverLineIsShown(control, [why], "beside"));
          control.addEventListener("mouseleave", theHoverLineIsHidden);
        }
      }
    }
    saidInto(control, item.label);
    control.addEventListener("click", () => {
      // A greyed item does nothing, whatever delivers the press.
      if (control.disabled) return;
      theRightClickMenuIsShut();
      if (item.act === THE_CALCULATE_SIZE_ACT) {
        theFoldersAreCounted(theFolders);
      } else if (item.act === "render-settings") {
        theSettingsAreOpenedForTheChosen();
      } else if (item.act === "check-dependencies") {
        theCheckIsOpenedForTheChosen();
      } else if (item.act === "delete") {
        theDeleteIsAskedAbout(true);
      } else if (item.act === "rename") {
        theRenameIsStarted();
      } else if (item.act === "cut" || item.act === "copy") {
        theCutOrCopyIsTaken(item.act);
      } else if (item.act === "paste") {
        // **Only the folder menu carries paste**, and it pastes into the
        // folder clicked.
        thePasteIsAsked(row.path.bytes);
      } else if (THE_ACTS_THAT_START_A_TRANSFER.includes(item.act)) {
        aTransferIsStarted(item.act);
      } else {
        anActNeedsAConnection(item.act);
      }
    });
    panel.appendChild(control);
  }
  host.appendChild(panel);

  // **Kept inside the window**, at the pointer where it can be.
  const room = panel.getBoundingClientRect();
  const left = Math.max(8, Math.min(Math.round(press.clientX), window.innerWidth - room.width - 8));
  const top = Math.max(8, Math.min(Math.round(press.clientY), window.innerHeight - room.height - 8));
  panel.style.setProperty("--fctx-x", `${left}px`);
  panel.style.setProperty("--fctx-y", `${top}px`);

  const away = (elsewhere) => {
    if (panel.contains(elsewhere.target)) return;
    theRightClickMenuIsShut();
  };
  const escape = (key) => {
    if (key.key !== "Escape") return;
    key.preventDefault();
    theRightClickMenuIsShut();
  };
  document.addEventListener("mousedown", away, true);
  document.addEventListener("keydown", escape, true);
  theOpenMenu = { panel, away, escape };
  const first = panel.querySelector("button");
  if (first) first.focus();
}

function theRightClickMenuIsShut() {
  theHoverLineIsHidden();
  if (!theOpenMenu) return;
  theOpenMenu.panel.remove();
  document.removeEventListener("mousedown", theOpenMenu.away, true);
  document.removeEventListener("keydown", theOpenMenu.escape, true);
  theOpenMenu = null;
}

// ── Uploading ───────────────────────────────────────────────────────────────

function theUploadIsAskedFor(what) {
  if (!folder) return;
  shell.core
    .invoke("an_upload_is_asked_for", { what })
    .then(theUploadAskIsDrawn)
    .catch(() => {});
}

function theUploadAskIsDrawn(ask) {
  if (!ask) return;
  if (ask.is === "nothing-was-chosen") return;
  if (ask.is === "sending-files-up-is-closed") return;
  if (ask.is === "which-kind") {
    theKindQuestionIsDrawn(ask);
    return;
  }
  // **No confirmation: a pick or a drop starts the upload** —
  // `Design/File Browser § Uploading`, the Creator's note 2 on the test copy.
  if (ask.is === "go") theUploadIsConfirmed(ask.chosen);
}

/// **H4's fallback, inside the Upload control and never beside it.**
function theKindQuestionIsDrawn(ask) {
  const holder = tools.querySelector(".uphold");
  if (!holder) return;
  const existing = holder.querySelector(".fask");
  if (existing) existing.remove();
  const question = document.createElement("div");
  question.className = "fask";
  const title = document.createElement("span");
  title.className = "fat";
  title.textContent = ask.title;
  question.appendChild(title);
  const said = document.createElement("span");
  said.className = "fas";
  said.textContent = ask.said;
  question.appendChild(said);
  const row = document.createElement("div");
  row.className = "mrow";
  const one = (label, what) => {
    const control = document.createElement("button");
    control.type = "button";
    control.className = "btn quiet";
    control.textContent = label;
    control.addEventListener("click", () => {
      question.remove();
      if (what) theUploadIsAskedFor(what);
    });
    return control;
  };
  row.appendChild(one(ask.files, "files"));
  row.appendChild(one(ask.folders, "folders"));
  const grow = document.createElement("span");
  grow.className = "grow";
  row.appendChild(grow);
  row.appendChild(one(ask.cancel, null));
  question.appendChild(row);
  holder.appendChild(question);
  const first = question.querySelector("button");
  if (first) first.focus();
}

// ── The shell's own drag and drop ───────────────────────────────────────────
//
// **`File Browser` H3a: dragging onto the list is the shell's own facility, and
// what arrives is paths and nothing else.** No sizes, no counts, no walk — so
// the count drawn is a count of what was dropped, and the real total arrives
// after the walk, which runs once the drop has started the upload.
//
// **Smoozy's own interface must not reach for HTML5 drag and drop**: `main.rs`
// keeps the shell's handler enabled, and the two are alternatives on Windows.

function theShellsOwnDropIsLive() {
  const window_ = shell.webviewWindow && shell.webviewWindow.getCurrentWebviewWindow();
  if (!window_ || !window_.onDragDropEvent) return;
  window_
    .onDragDropEvent((arrived) => {
      const what = arrived.payload || {};
      if (!host || host.hidden || !folder) return;
      if (what.type === "over" || what.type === "enter") {
        if (Array.isArray(what.paths) && what.paths.length > 0) theDropZoneIsShown(what.paths);
        return;
      }
      if (what.type === "drop") {
        theDropZoneIsHidden();
        shell.core
          .invoke("files_were_dropped", { paths: what.paths || [] })
          .then(theUploadAskIsDrawn)
          .catch(() => {});
        return;
      }
      theDropZoneIsHidden();
    })
    .catch(() => {});
}

function theDropZoneIsShown(paths) {
  shell.core
    .invoke("what_is_being_dropped", { folder, paths })
    .then((reading) => {
      dropzone.textContent = "";
      const inner = document.createElement("div");
      inner.className = "dropbox";
      const into = document.createElement("p");
      into.className = "dt";
      saidInto(into, reading.into, "mono");
      inner.appendChild(into);
      const many = document.createElement("p");
      many.className = "ds";
      saidInto(many, reading.how_many);
      inner.appendChild(many);
      dropzone.appendChild(inner);
      dropzone.hidden = false;
    })
    .catch(() => {});
}

function theDropZoneIsHidden() {
  if (dropzone) dropzone.hidden = true;
}

// ── The boxes ───────────────────────────────────────────────────────────────

function aBox(onTop) {
  const wrap = document.createElement("div");
  wrap.className = onTop ? "modal top" : "modal";
  wrap.setAttribute("role", "dialog");
  wrap.setAttribute("aria-modal", "true");
  const body = document.createElement("div");
  body.className = "mbox";
  wrap.appendChild(body);
  const close = () => wrap.remove();
  // **Escape closes the thing on top, one layer at a time** — Design's own
  // rule, and `web/a-server.js` steps aside for anything carrying
  // `role="dialog"`, which is what this carries.
  wrap.addEventListener("keydown", (press) => {
    if (press.key !== "Escape") return;
    press.preventDefault();
    close();
  });
  return {
    body,
    close,
    raise() {
      boxes.appendChild(wrap);
      const first = body.querySelector("button");
      if (first) first.focus();
    },
  };
}

function theRowOfAnswers(answers) {
  const row = document.createElement("div");
  row.className = "mrow";
  const grow = document.createElement("span");
  grow.className = "grow";
  row.appendChild(grow);
  for (const answer of answers) {
    const control = document.createElement("button");
    control.type = "button";
    control.className = answer.danger
      ? "btn danger"
      : answer.quiet
        ? "btn quiet"
        : "btn";
    control.textContent = answer.label;
    if (answer.act) control.addEventListener("click", answer.act);
    row.appendChild(control);
  }
  return row;
}

/// **The delete box, carrying two warnings of different weight.**
function theDeleteIsAskedAbout(fromTheControl) {
  if (!server) return;
  // **Nothing chosen, nothing asked** — the delete key with no row chosen does
  // nothing rather than asking about nought items (must-do 26). **The control
  // says so in one line** (`Design/File Browser § What the screen says around
  // an act`, step 10c's Files' owed acts piece 3).
  if (chosen.length === 0) {
    if (fromTheControl && said) theLineSays(said.toolbar.around_an_act.select_to_delete, "delete");
    return;
  }
  // **The chosen rows' paths, never their names** — a bare name means a
  // different thing on the machine, and the day this box deletes it would
  // delete the wrong thing. Step 10c's Files' owed acts piece 1. **Every row
  // chosen goes, and the box names how many** (the thirteenth notes, note 6).
  const victims = theChosenPaths();
  if (victims.length === 0) return;
  shell.core
    .invoke("the_delete_of", { server, victims })
    .then((reading) => theDeleteBoxIsDrawn(reading, victims))
    .catch(() => {});
}

/// **The delete goes** (step 10c's Files' owed acts piece 4): what it came to
/// is said on the line above the list, and the folder is read again.
function theDeleteGoes(victims) {
  if (!server) return;
  const which = server;
  shell.core
    .invoke("a_delete_is_made", { server: which, victims })
    .then((answer) => {
      // Done confirms every victim. Partial or unanswered deletes wait for a reading.
      if (answer && answer.is === "done" && which === server) {
        theListingChange += 1;
        const gone = new Set(victims.map((path) => path.join(",")));
        const names = new Set();
        for (const row of Array.from(list.children)) {
          if (!gone.has(theRowsPaths.get(row))) continue;
          names.add(theRowsNames.get(row));
          row.remove();
        }
        chosen = chosen.filter((name) => !names.has(name.join(",")));
        if (said && said.listing.is === "rows") {
          said = { ...said, listing: { ...said.listing,
            rows: said.listing.rows.filter((row) => !gone.has(row.path.bytes.join(","))),
          } };
          theListIsDrawn(said);
        }
      }
      theActIsAnsweredOn(which)(answer);
    })
    .catch(() => {});
}

function theDeleteBoxIsDrawn(reading, victims) {
  if (!reading) return;
  // **A delete warning the user switched off asks nothing**, where nothing
  // that cannot be switched off stands beside it (`File Browser` M3): the
  // live-job warning, the references line and the staging line all still
  // ask.
  const switchedOff = reading.the_permanence_warning_is_switched_off;
  if (switchedOff && !reading.live_jobs && !reading.other_references && !reading.staged_material) {
    theDeleteGoes(victims);
    return;
  }
  const box = aBox();
  const heading = document.createElement("h3");
  saidInto(heading, reading.question, "mono");
  box.body.appendChild(heading);
  if (reading.live_jobs) box.body.appendChild(theLiveJobWarning(reading.live_jobs));
  if (reading.other_references) {
    box.body.appendChild(aWarning(reading.other_references, false));
  }
  if (reading.staged_material) {
    box.body.appendChild(aWarning(reading.staged_material, false));
  }
  // **The permanence warning, its tick and its note go together**, and none
  // of the three is drawn once the user has switched it off.
  const permanent = document.createElement("p");
  permanent.className = "msub";
  permanent.textContent = reading.permanent;
  if (!switchedOff) box.body.appendChild(permanent);

  const tick = document.createElement("label");
  tick.className = "mcheck";
  const mark = document.createElement("span");
  mark.className = "cb";
  mark.setAttribute("role", "checkbox");
  mark.setAttribute("aria-checked", String(reading.the_permanence_warning_is_switched_off));
  mark.classList.toggle("on", reading.the_permanence_warning_is_switched_off);
  mark.textContent = reading.the_permanence_warning_is_switched_off ? "✓" : "";
  tick.appendChild(mark);
  tick.appendChild(textNode(" "));
  const word = document.createElement("span");
  word.textContent = reading.dont_warn_again;
  tick.appendChild(word);
  tick.addEventListener("click", () => {
    const now = mark.getAttribute("aria-checked") !== "true";
    mark.setAttribute("aria-checked", String(now));
    mark.classList.toggle("on", now);
    mark.textContent = now ? "✓" : "";
  });
  if (!switchedOff) box.body.appendChild(tick);

  // **Which warning it does not cover, by name.**
  const note = document.createElement("p");
  note.className = "mnote";
  note.textContent = reading.what_it_does_not_cover;
  if (!switchedOff) box.body.appendChild(note);

  box.body.appendChild(
    theRowOfAnswers([
      { label: reading.cancel, quiet: true, act: () => box.close() },
      {
        label: reading.delete,
        danger: true,
        act: () => {
          box.close();
          // **The tick is the user's answer to a question, so it is saved at
          // the press** (must-do 24). A switch-off the record would not take
          // is said here.
          if (mark.getAttribute("aria-checked") === "true" && !switchedOff) {
            shell.core.invoke("the_delete_warning_was_switched_off", { server }).catch(() => {});
          }
          theDeleteGoes(victims);
        },
      },
    ]),
  );
  box.raise();
}

/// **The live-job warning, above all four answers and never beside one.**
function theLiveJobWarning(warning) {
  const said = document.createElement("p");
  said.className = "mwarn hard";
  const named = document.createElement("b");
  saidInto(named, warning.said);
  said.appendChild(named);
  if (warning.only_the_blend_and_its_folder) {
    said.appendChild(textNode(" "));
    const narrow = document.createElement("span");
    narrow.textContent = warning.only_the_blend_and_its_folder;
    said.appendChild(narrow);
  }
  said.appendChild(textNode(" "));
  const may = document.createElement("span");
  may.textContent = warning.a_render_may_finish_anyway;
  said.appendChild(may);
  return said;
}

/// **One quiet warning: the half the drawing bolds, then the half it does
/// not.** Both arrive composed; nothing here decides which is which.
function aWarning(line, hard) {
  const said = document.createElement("p");
  said.className = hard ? "mwarn hard" : "mwarn";
  const named = document.createElement("b");
  saidInto(named, line.named);
  said.appendChild(named);
  if (line.said) {
    said.appendChild(textNode(" "));
    const rest = document.createElement("span");
    rest.textContent = line.said;
    said.appendChild(rest);
  }
  return said;
}

/// **The live-job box the rename and the move take** — the same box and the
/// same words, and only the title says which act.
function theDisturbanceIsAskedAbout(act) {
  if (!server) return;
  // **Paths, never names**, as the delete box sends (step 10c's Files' owed
  // acts piece 1), and every row chosen.
  const paths = theChosenPaths();
  shell.core
    .invoke("what_disturbing_costs", { server, act, paths })
    .then((reading) => {
      if (!reading) {
        anActNeedsAConnection(act);
        return;
      }
      theLiveJobBoxIsDrawn(reading, () => anActNeedsAConnection(act));
    })
    .catch(() => {});
}

/// **The live-job box, and what *carry on* does.** `File Browser` W5's box for
/// a rename or a move: the same box and the same words, the title saying which
/// act.
function theLiveJobBoxIsDrawn(reading, carryOn, cancel) {
  const box = aBox();
  const heading = document.createElement("h3");
  saidInto(heading, reading.question, "mono");
  box.body.appendChild(heading);
  box.body.appendChild(theLiveJobWarning(reading.live_jobs));
  box.body.appendChild(
    theRowOfAnswers([
      {
        label: reading.cancel,
        quiet: true,
        act: () => {
          box.close();
          if (cancel) cancel();
        },
      },
      {
        label: reading.carry_on,
        act: () => {
          box.close();
          carryOn();
        },
      },
    ]),
  );
  box.raise();
}

// ── New folder and Rename: a row you can type in ────────────────────────────
//
// **`File Browser` M4 and M9, and `Design/File Browser § What the screen says
// around an act`.** Step 10c's Files' owed acts piece 3. The row being typed
// is the page's own while it is typed: the name is sent when Return is
// pressed, and Escape puts the row back as it was.

/// **The row being typed**, or nothing: what kind of act, the path being
/// renamed, the text typed, and what the last attempt said on the row.
let editing = null;

/// **The field the drawing just made for the row being typed**, focused and
/// selected once the list holding it is on the page.
let theFieldToTakeTheKeys = null;
/// **The row being typed, as drawn**, so a press inside it is told from a
/// press anywhere else.
let theEditingRow = null;

/// Whether a press landed in a field being typed into.
function aFieldIsBeingTyped(target) {
  if (!target) return false;
  const tag = String(target.tagName || target.tag || "").toUpperCase();
  return tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT" || Boolean(target.isContentEditable);
}

/// The typed text as the bytes a name is sent as.
function theBytesOf(text) {
  return Array.from(new TextEncoder().encode(text));
}

/// *New folder*: a row at the top of the list, named *New folder*, selected so
/// typing replaces it.
function theNewFolderIsStarted() {
  if (!server || !folder || !said || said.listing.is !== "rows") return;
  editing = {
    kind: "new-folder",
    path: null,
    was: null,
    typed: said.toolbar.around_an_act.a_new_folders_name,
    refused: null,
    refusal: null,
  };
  theListIsDrawn(said);
}

/// *Rename*: the chosen row becomes a field holding its name. **Nothing
/// chosen says so in one line.**
function theRenameIsStarted() {
  if (!server || !said) return;
  // **Rename is greyed while several are chosen** (the thirteenth notes,
  // note 6), so F2 and a greyed control do nothing and say nothing.
  if (severalAreChosen()) return;
  const row = theChosenRow();
  if (!row) {
    theLineSays(said.toolbar.around_an_act.select_to_rename, "rename");
    return;
  }
  editing = {
    kind: "rename",
    path: row.path.bytes,
    was: row.name.shown,
    typed: row.name.shown,
    refused: null,
    refusal: null,
  };
  theListIsDrawn(said);
}

/// **Whether the list the user is looking at already shows the typed name** —
/// M4's advisory hint, which stops nothing (and M9 answers a new folder's
/// *taken* from the same list, on the application's side).
function theListingShowsTheTyped() {
  if (!editing || !said || said.listing.is !== "rows") return false;
  const typed = theBytesOf(editing.typed).join(",");
  const own = editing.path ? editing.path.join(",") : null;
  return said.listing.rows.some(
    (one) => one.name.bytes.join(",") === typed && one.path.bytes.join(",") !== own,
  );
}

/// **The row being typed, drawn.** A field, the line under it where the last
/// attempt said something, and the listing's hint where it holds the name.
function anEditingRow(row) {
  const element = document.createElement("div");
  element.className = "frow editing";
  const icon = (THE_FILE_ICONS[row ? row.icon : "folder"] || THE_FILE_ICONS.file)();
  icon.setAttribute("class", "fic");
  icon.classList.add(row ? row.icon : "folder");
  element.appendChild(icon);
  // **The field is the drawing's typing field** (`Design/File Browser §
  // Acting on files`; the Creator's note 4 on the test copy): `fedit` keeps the
  // row's height, takes the list's font, sits where the name sits as wide as
  // the name column, and shows its focus by a one-pixel accent border; the
  // cell's `fediting` lets that border show rather than clipping it.
  const name = document.createElement("span");
  name.className = "fname fediting";
  const field = document.createElement("input");
  field.type = "text";
  field.className = editing.refused ? "fld fedit bad" : "fld fedit";
  field.value = editing.typed;
  field.setAttribute("aria-label", said.toolbar.rename);
  const hint = document.createElement("span");
  hint.className = "fhint";
  const theHintIsSaid = () => {
    hint.textContent = "";
    if (!editing.refused && theListingShowsTheTyped()) saidInto(hint, said.toolbar.around_an_act.the_listing_saw_it);
  };
  field.addEventListener("input", () => {
    editing.typed = field.value;
    editing.refused = null;
    field.className = "fld fedit";
    theHintIsSaid();
  });
  field.addEventListener("keydown", (press) => {
    if (press.key === "Enter") {
      press.preventDefault();
      theEditIsCommitted();
    } else if (press.key === "Escape") {
      press.preventDefault();
      editing = null;
      if (said) theListIsDrawn(said);
    }
  });
  name.appendChild(field);
  if (editing.refused) name.appendChild(aDenial(editing.refused, null, null));
  theHintIsSaid();
  name.appendChild(hint);
  if (editing.refusal) {
    name.appendChild(aDenial(editing.refusal.said, editing.refusal.the_offer, editing.refusal.dismiss));
  }
  element.appendChild(name);
  const kind = document.createElement("span");
  kind.className = "fmeta kind";
  kind.textContent = row ? row.kind : "";
  itShowsWholeOnHover(kind);
  element.appendChild(kind);
  // **Selected, so typing replaces it** — once it is on the page, which is
  // the end of the listing's drawing.
  theFieldToTakeTheKeys = field;
  theEditingRow = element;
  return element;
}

/// **Return: the name goes.** A new folder goes at once; a rename asks the
/// live-job box first where `File Browser` W5 names a job, and goes on *carry
/// on*.
function theEditIsCommitted() {
  if (!editing || !server || !said) return;
  const asked = editing;
  const name = theBytesOf(asked.typed);
  if (asked.kind === "new-folder") {
    const shown = said.listing.is === "rows" ? said.listing.rows.map((one) => one.name.bytes) : [];
    shell.core
      .invoke("a_new_folder_is_made", { server, folder, name, shown })
      .then(theActIsAnsweredOn(server))
      .catch(() => {});
    return;
  }
  if (asked.typed === asked.was) {
    editing = null;
    theListIsDrawn(said);
    return;
  }
  shell.core
    .invoke("what_disturbing_costs", { server, act: "rename", paths: [asked.path] })
    .then((reading) => {
      const goes = () =>
        shell.core
          .invoke("a_rename_is_made", { server, path: asked.path, name })
          .then(theActIsAnsweredOn(server))
          .catch(() => {});
      if (!reading) goes();
      else theLiveJobBoxIsDrawn(reading, goes);
    })
    .catch(() => {});
}

/// **What the act came to**, for the server it was asked on. On the row it stays
/// in editing, red; otherwise the row is let go of, the line says what there is
/// to say, and the folder is read again. **An answer that comes while that
/// server's Files is not showing is kept for it** and said when the user comes
/// back (`Design/File Browser § The list`).
function theActIsAnsweredOn(which) {
  return (answer) => {
    if (!answer) return;
    if (which !== server || !host || host.hidden) {
      if (answer.is !== "on-the-row" && answer.said) theEndingIsKept(which, answer.said, "act");
      if (which !== server) return;
    }
    theActIsAnswered(answer);
  };
}

function theActIsAnswered(answer) {
  if (!answer) return;
  if (answer.is === "on-the-row") {
    if (!editing) return;
    editing.refused = answer.editing.refused || null;
    editing.refusal = answer.refusal || null;
    if (said) theListIsDrawn(said);
    return;
  }
  editing = null;
  if (answer.said && host && !host.hidden) theLineSays(answer.said, "act");
  if (folder) theBrowserIsAskedFor(folder);
}

// ── Cut, copy and paste ─────────────────────────────────────────────────────
//
// **`File Browser` M11 to M18 and `Design/File Browser § Acting on files`.**
// Step 10c's Files' owed acts piece 5. The pasteboard is the application's;
// the page only says what was chosen and where to paste.

/// **Cut or Copy**: the chosen rows go on the pasteboard, every one of them
/// (the thirteenth notes, note 6), and the folder is read again so Paste shows
/// its count. **Nothing chosen says so in one line.**
function theCutOrCopyIsTaken(act) {
  if (!server || !said) return;
  const paths = theChosenPaths();
  if (paths.length === 0) {
    theLineSays(act === "cut" ? said.toolbar.around_an_act.select_to_cut : said.toolbar.around_an_act.select_to_copy, act);
    return;
  }
  shell.core
    .invoke("a_cut_or_copy_is_taken", { server, act, paths })
    .then(theActIsAnsweredOn(server))
    .catch(() => {});
}

/// **Paste into `into`**: the live-job box first where `File Browser` W5
/// names a job, then the paste; where something collides, the question, and
/// the paste again with the answer.
function thePasteIsAsked(into) {
  if (!server || !into) return;
  const askedOn = server;
  const goes = (answer) =>
    shell.core
      .invoke("a_paste_is_made", { server: askedOn, into, answer: answer || null })
      .then((came) => {
        if (came && came.is === "asks") {
          theAlreadyExistsBox(came.question, null, (chosenAnswer) => goes(chosenAnswer));
          return;
        }
        theActIsAnsweredOn(askedOn)(came);
      })
      .catch(() => {});
  shell.core
    .invoke("what_a_paste_costs", { server, into })
    .then((reading) => {
      if (!reading) goes(null);
      else theLiveJobBoxIsDrawn(reading, () => goes(null));
    })
    .catch(() => {});
}

// ── An upload, going ────────────────────────────────────────────────────────
//
// **`File Browser` §3 to §5, run by `src/uploading.rs`.** A pick or a drop
// hands over the paths it was given and nothing else; everything that happens
// to them is the application's.

function theUploadIsConfirmed(chosenPaths) {
  if (!server || !folder) return;
  shell.core
    .invoke("the_upload_is_confirmed", {
      server,
      folder,
      chosen: (chosenPaths || []).map((path) => path.bytes),
    })
    .then(theUploadGoes)
    .catch(() => {});
}

/// **What the upload, or the answer to the question, led to.**
function theUploadGoes(goes) {
  if (!goes) return;
  if (goes.is === "the-question") {
    theAlreadyExistsBox(goes.question, goes.request);
    return;
  }
  // **An upload under way says nothing above the list** — `Design/File
  // Browser § The list`: the bottom bar's transfer count says it is running,
  // and the line says only how it ended. A line left from an earlier answer is
  // taken away, so it does not read as this one's. The Creator's notes piece 6.
  if (goes.is === "underway") theLineIsTakenAway();
  if (goes.is === "did-not-start") theLineSays(goes.said, "upload");
}

/// **The line above the list**, where an answer is said where the user asked —
/// the shape piece 3 gave an answer on the server list.
function theLineIsTakenAway() {
  const told = host && host.querySelector(".fnotyet");
  if (told) told.hidden = true;
}

/// **The line, for the server showing.** Made the first time it is needed, and
/// marked with the server it is about to speak for.
function theLine() {
  let told = host.querySelector(".fnotyet");
  if (!told) {
    told = document.createElement("div");
    told.className = "fnotyet";
    host.insertBefore(told, list);
  }
  theLinesServer = server;
  theLinesFolder = folder;
  return told;
}

/// **The line belongs to the folder it was said in** (ruling (c)): a folder
/// drawn that is not that folder takes it away. The same folder drawn again —
/// the list read again after an act — keeps it.
function theLineGoesWithItsFolder(drawn) {
  if (theLinesServer === null) return;
  if (theLinesFolder === null) {
    theLinesFolder = drawn;
    return;
  }
  if (theLinesFolder.join(",") !== drawn.join(",")) theLineIsCleared();
}

function theLineSays(words, act) {
  if (!host || !list) return;
  const told = theLine();
  told.dataset.act = act || "";
  told.textContent = "";
  saidInto(told, words, "mono", true);
  told.hidden = false;
}

/// **Another server's line is not this one's**: it is hidden and emptied.
function theLineIsCleared() {
  theLinesServer = null;
  theLinesFolder = null;
  const told = host && host.querySelector(".fnotyet");
  if (!told) return;
  told.hidden = true;
  told.textContent = "";
  told.dataset.act = "";
}

/// **An ending kept for a server whose Files is not showing** — the newest
/// only, shown once by `itIsShowing`.
function theEndingIsKept(which, words, act) {
  if (!which) return;
  theEndingsKept.set(which, { words, act });
}

/// **The already-exists box, in one shape and six readings.**
///
/// **An upload's reading is raised with its request**, and its answer goes back
/// to that request: `File Browser` U4's answer is written down before any file
/// moves. The other five readings belong to acts that are not built, and their
/// answers still say so.
export function theAlreadyExistsBox(question, request, onAnswer) {
  const box = aBox();
  const heading = document.createElement("h3");
  saidInto(heading, question.headline, "mono");
  box.body.appendChild(heading);

  const compared = document.createElement("p");
  compared.className = "msub";
  compared.textContent = question.compared_by;
  box.body.appendChild(compared);

  const names = document.createElement("div");
  names.className = "clist mono";
  for (const one of question.names) {
    const line = document.createElement("div");
    theValueIsDrawnInto(line, one);
    names.appendChild(line);
  }
  if (question.and_more) {
    const more = document.createElement("div");
    more.className = "more";
    saidInto(more, question.and_more);
    names.appendChild(more);
  }
  box.body.appendChild(names);

  // **The dependency-check line, and only an upload can meet it.**
  if (question.placed_by_the_check) {
    const line = aWarning(question.placed_by_the_check, false);
    line.appendChild(textNode(" "));
    const offer = document.createElement("button");
    offer.type = "button";
    offer.className = "mini";
    offer.textContent = question.placed_by_the_check.offer;
    // **`File Browser` U12 to `File Browser` U15: the files the check placed go through
    // the check instead**, and the check of the `.blend` whose tree reached
    // them opens and reads — where *Upload* sends them, with the check's own
    // replace-or-stop. The rest of the upload keeps its question (`File Browser` U13). The
    // 38's piece 7; until then this said *not built yet*.
    offer.addEventListener("click", () => {
      box.close();
      if (!request) return;
      const over = server;
      shell.core
        .invoke("the_files_the_check_placed_go_through_it", { request })
        .then((handed) => {
          if (!handed) return;
          theUploadGoes(handed.goes);
          if (handed.the_check_of && over) theCheckIsOpenedOn(over, handed.the_check_of).catch(() => {});
        })
        .catch(() => {});
    });
    line.appendChild(offer);
    box.body.appendChild(line);
  }

  // **The live-job warning sits above all four answers, not beside one.**
  if (question.live_jobs) box.body.appendChild(theLiveJobWarning(question.live_jobs));

  if (question.interrupted) {
    const why = document.createElement("p");
    why.className = "msub";
    saidInto(why, question.interrupted);
    box.body.appendChild(why);
  }

  const row = document.createElement("div");
  row.className = "mrow";
  for (const answer of question.answers) {
    if (answer.answer === "stop" || answer.answer === "cancel") {
      const grow = document.createElement("span");
      grow.className = "grow";
      row.appendChild(grow);
    }
    const control = document.createElement("button");
    control.type = "button";
    control.className = answer.put_forward ? "btn" : "btn quiet";
    control.textContent = answer.label;
    control.addEventListener("click", () => {
      box.close();
      if (request) {
        // **Stop is an answer too**: the request records it and ends.
        shell.core
          .invoke("the_already_exists_question_was_answered", { request, answer: answer.answer })
          .then(theUploadGoes)
          .catch(() => {});
        return;
      }
      if (answer.answer !== "stop" && answer.answer !== "cancel") {
        // **A paste's question goes back to the paste** (step 10c's Files'
        // owed acts piece 5).
        if (onAnswer) onAnswer(answer.answer);
        else anActNeedsAConnection(`collision-answer-${answer.answer}`);
      }
    });
    row.appendChild(control);
  }
  box.body.appendChild(row);
  box.raise();
  return box;
}

// ── The acts that need the machine ──────────────────────────────────────────

/// **An act that starts a transfer goes to Transfers**, over the server being
/// browsed, and the answer is said there.
///
/// **What the user picked travels with it, as its path on the machine and
/// never as its name** — `Download` M1 holds *"the source path on the machine
/// as listed"* and `File Browser` N2 carries names as bytes that nothing
/// rewrites in flight. The row's own bytes go back to the application
/// untouched.
///
/// **It is a list because `Download` M2 takes one** — *"files, folders, any
/// mix, in any number"* — and since the thirteenth notes, note 6, it carries
/// every row chosen, in the list's drawn order.
function aTransferIsStarted(act) {
  if (!server) return;
  aTransferIsAskedFor(act, server, theChosenPaths());
}

function anActNeedsAConnection(act) {
  if (!server) return;
  shell.core.invoke("a_file_act_that_needs_a_connection", { server, act }).catch(() => {});
}

// **The answer, drawn where the user asked for it.** A control that did nothing
// is Smoozy going quiet, which Rule 8 makes a fault however the trouble arose.
whenTold(THE_ANSWER, (delivery) => {
  const what = delivery.said.what;
  if (what === "the-upload-ended") {
    theUploadEnded(delivery.said.server, delivery.said.request);
    return;
  }
  if (what === "an-act-moves") {
    theActMoves(delivery.said);
    return;
  }
  if (what !== "the-act-is-not-built" && what !== "the-setting-did-not-save") return;
  // **An answer about another server is not this drawing's.** One server is
  // browsed at a time, and the answer names the one it is about.
  if (delivery.said.server && delivery.said.server !== server) return;
  if (!host || host.hidden || !delivery.said.said) return;
  // **A line above the list, and not a box.** The user asked for something and
  // is told there where they asked; a box would put a second thing in front of
  // them for an answer they did not need to stop for. This is the shape piece 3
  // gave the same answer on the server list.
  const told = theLine();
  // **The act is a token and not a word**, so it goes on the element rather
  // than into the sentence: it is what says which control answered, to anything
  // reading the window.
  told.dataset.act = delivery.said.act || "";
  told.textContent = delivery.said.said;
  told.hidden = false;
});

/// **How far a long act has got**, on the line above the list (`Design/File
/// Browser § What the screen says around an act`): the act's word, then the
/// count done of the count there is. Only for the server being browsed.
function theActMoves(told) {
  if (!host || host.hidden || (told.server && told.server !== server)) return;
  const line = theLine();
  line.dataset.act = told.act || "";
  line.textContent = `${told.lead} ${told.done} ${told.of} ${told.total}`;
  line.hidden = false;
}

/// **An upload ended**: what it came to is said on the line, and the folder
/// being browsed is read again (`File Browser` L12) — whichever server's upload
/// it was, the line says it only where that server is the one showing. **The
/// words are asked for at once**, because the application lets them go as it
/// hands them over; where that server's Files is not showing they are kept for
/// it and said when the user comes back (`Design/File Browser § The list`).
function theUploadEnded(which, request) {
  if (!request) return;
  shell.core
    .invoke("what_the_upload_came_to", { request })
    .then((words) => {
      if (!words) return;
      if (which !== server || !host || host.hidden) {
        theEndingIsKept(which, words, "upload");
        return;
      }
      theLineSays(words, "upload");
      theBrowserIsAskedFor(folder);
    })
    .catch(() => {});
}

// **The folder is read again the moment the connection is handed over** —
// `File Browser` Row 198, *"the listing is read again when the connection
// comes back"*, and `File Browser` L17. **Asking as the server reads *connected* is too
// soon**: `Servers` publishes it before its own first checks, so that ask finds
// nothing lent and draws no listing, and until the Creator's eighteenth notes,
// note 14, the next ask was the work column moving after step 7 — 15 s after
// the hand-over at a far server's delay at the desk. **Only where this tab is
// showing that server**; a showing that comes later asks as it shows.
whenTold(THE_CONNECTION_IS_LENT, (delivery) => {
  const which = (delivery.said || {}).server;
  if (!isShowing || !host || !which || which !== server) return;
  theBrowserIsAskedFor(folder);
});


// ── The Go menu, and the keyboard ───────────────────────────────────────────
//
// **`Up one folder` is the one Go item that is a control on a screen rather
// than a place.** The going arrives on the one ordered queue like every other
// one, and the module that owns the act performs it — `src/menus.rs` is not
// edited and nothing about the item moves. **It does nothing where the browser
// is not showing**, which is the honest answer: there is no folder to go up
// from.

whenTold(A_GOING, (delivery) => {
  if (delivery.said.item !== UP_ONE_FOLDER) return;
  theBrowserGoesUp();
});

function theBrowserGoesUp() {
  if (!host || host.hidden || !said) return;
  if (!said.the_folder_above) return;
  theBrowserIsAskedFor(said.the_folder_above.bytes);
}

/// **`Design/File Browser § Keyboard shortcuts`.** *"Return opens what is
/// chosen. Backspace goes up a folder. The delete key deletes, asking exactly
/// as the control does. F2 renames."*
///
/// **Nothing that crosses the wire has a key**: Upload, Download and the two
/// `.blend` actions are all pressed deliberately.
///
/// **↑ and ↓ move the chosen row, Return opens a chosen folder, and on a Mac
/// the keys are the Mac's own** — added 2026-10-04 to the same section, the
/// Creator's note 4 on the test copy (the Creator's second notes, piece 8):
/// *"↑ and ↓ move the chosen row … Return opens a chosen folder and does
/// nothing on a file … the key a Mac keyboard labels delete deletes, asking
/// exactly as the control does, and ⌘↑ goes up a folder."* On a Mac that key
/// reports itself as Backspace, so until this piece it went up a folder and
/// nothing on most Mac keyboards deleted. **Shift with ↑ or ↓ widens the
/// choice by one row** — restored 2026-10-18 with choosing several rows, the
/// thirteenth notes, note 6; until then it widened nothing, Files holding one
/// chosen row. **Return does nothing while several are chosen.**
///
/// **While a panel is open over this server the keys are the panel's, not the
/// browser's** (must-do 25 of the step 10a round 1 audit of these pieces): the
/// panel covers the browser without hiding it, so a Delete meant for a ticked
/// scene would otherwise ask to delete the chosen file.
function theKeysAreLive() {
  document.addEventListener("keydown", (press) => {
    if (!theBrowserIsOnScreen() || press.defaultPrevented) return;
    if (aPanelCoversTheBrowser()) return;
    // **The picture viewer's keys are its own** while it is open.
    if (theViewer) return;
    // A box over the browser handles its own keys, and a field being typed into
    // is not this screen's either.
    if (press.target && press.target.closest && press.target.closest("[role=dialog]")) return;
    // **A name being typed is not this screen's to act on**: Backspace and
    // Delete edit the name (step 10c's Files' owed acts piece 3).
    if (aFieldIsBeingTyped(press.target)) return;
    const held = press.shiftKey || press.metaKey || press.ctrlKey || press.altKey;
    // **↑ and ↓ are the list's wherever the focus was left** — the Creator's
    // third notes on the test copy, note 6 (*"this one does work sometimes"*).
    // They were acted on only with the focus inside this tab or nowhere, and
    // the focus is often left outside it: on the server's row in the list of
    // servers, which a click gives the focus to; on the Files tab; or on a
    // panel's strip, where it lands when the panel collapses — and before
    // note 7, one click on a `.blend` opened Render Settings. Nothing else on
    // the window binds these two keys, so no one else's are taken.
    if ((press.key === "ArrowDown" || press.key === "ArrowUp") && !held) {
      press.preventDefault();
      theChosenRowMoves(press.key === "ArrowDown" ? 1 : -1, false);
      return;
    }
    const onlyShift = press.shiftKey && !press.metaKey && !press.ctrlKey && !press.altKey;
    if ((press.key === "ArrowDown" || press.key === "ArrowUp") && onlyShift) {
      press.preventDefault();
      theChosenRowMoves(press.key === "ArrowDown" ? 1 : -1, true);
      return;
    }
    const inside = press.target === document.body || host.contains(press.target);
    if (!inside) return;
    if (onAMac() && press.key === "ArrowUp" && press.metaKey && !press.shiftKey && !press.ctrlKey && !press.altKey) {
      press.preventDefault();
      theBrowserGoesUp();
      return;
    }
    // **Return is the list's only where the list has the key** — on a row, or
    // with nothing focused — so a toolbar control with the focus keeps its own.
    if (press.key === "Enter" && !held && theListHasTheKey(press.target)) {
      press.preventDefault();
      theChosenRowIsOpened();
      return;
    }
    if (press.key === "Backspace" && onAMac()) {
      press.preventDefault();
      theDeleteIsAskedAbout(false);
      return;
    }
    if (press.key === "Backspace") {
      press.preventDefault();
      theBrowserGoesUp();
      return;
    }
    if (press.key === "F2") {
      press.preventDefault();
      theRenameIsStarted();
      return;
    }
  });
  // The delete key, kept apart from the block above so that the one key that
  // destroys something reads on its own line.
  document.addEventListener("keydown", (press) => {
    if (!theBrowserIsOnScreen() || press.defaultPrevented) return;
    if (press.key !== "Delete") return;
    if (aPanelCoversTheBrowser()) return;
    if (theViewer) return;
    if (press.target && press.target.closest && press.target.closest("[role=dialog]")) return;
    const inside = press.target === document.body || host.contains(press.target);
    if (!inside) return;
    if (aFieldIsBeingTyped(press.target)) return;
    press.preventDefault();
    theDeleteIsAskedAbout(false);
  });
}

/// **Two presses that let something go** — `Design/File Browser § Acting on
/// files` and the section on what the toolbar offers, both added 2026-10-06
/// from the Creator's third notes on the test copy.
///
/// **A press anywhere outside the row being typed puts it back as it was, as
/// Escape does** (note 8: *"Clicking in empty space … should discard the
/// attempted renaming"*). It is heard before the press reaches anything, so a
/// press on another row or a control acts on a list with no row being typed.
/// A box over the browser is not *outside*: the live-job box a rename raises
/// asks about the name being typed.
///
/// **A click on the list's empty space lets the choice go** (note 6:
/// *"clicking in empty space should deselect everything"*). The list itself
/// is the empty space; a row, the heading or a sentence in its place is not.
function thePressesAreLive() {
  document.addEventListener(
    "mousedown",
    (press) => {
      if (!editing || !theEditingRow) return;
      const at = press.target;
      if (at && theEditingRow.contains(at)) return;
      if (at && at.closest && at.closest("[role=dialog]")) return;
      editing = null;
      theEditingRow = null;
      if (said) theListIsDrawn(said);
    },
    true,
  );
  host.addEventListener("click", (press) => {
    if (!list || press.target !== list || chosen.length === 0 || !said) return;
    theChoiceIsLetGo();
    theBrowserIsDrawn(said);
  });
}

/// **Whether Files is on the screen**: its tab showing, and the server's screen
/// that holds it not hidden behind another screen. A hidden screen has no
/// boxes on the page; the tab's own `hidden` alone did not say so, and with the
/// focus nowhere a key pressed on Queue reached Files.
function theBrowserIsOnScreen() {
  if (!host || host.hidden) return false;
  return !host.getClientRects || host.getClientRects().length > 0;
}

/// **Whether this is a Mac**, read the one way the page already learns it: the
/// system's name `web/chrome.js` asks the application for and writes on the
/// document. Never guessed from the browser engine.
function onAMac() {
  const page = document.documentElement;
  return Boolean(page && page.dataset && page.dataset.system === "macos");
}

/// Whether a press reached the list itself: a row, or nothing in particular.
function theListHasTheKey(target) {
  if (!target || target === document.body) return true;
  return Boolean(target.classList && target.classList.contains("frow"));
}

/// **The rows a click can choose, in the order they are drawn** — the shown
/// rows, less any that carries a refusal or is being typed, which are not
/// buttons and which a click cannot choose either.
function theRowsAClickCanChoose() {
  if (!said || !said.listing || said.listing.is !== "rows") return [];
  return said.listing.rows.filter((row) => theListShows(row, said) && !row.refusal && !row.editing);
}

/// **↑ or ↓: the next row up or down is chosen** — the first or the last when
/// nothing is — through what a click on a plain file does, so the row is
/// marked and the toolbar changes and **nothing is opened and nothing is asked
/// for**: a folder is only chosen, and a `.blend` does not open its settings.
///
/// **With several chosen, the step is taken from the moving end**, and it
/// chooses the row it reaches alone. **With Shift, the choice widens by one
/// row** (the thirteenth notes, note 6): the moving end steps and the choice
/// is the range from the last row clicked to it, as a Shift-click makes.
function theChosenRowMoves(by, widening) {
  const choosable = theRowsAClickCanChoose();
  if (choosable.length === 0) return;
  const at = (key) => (key ? choosable.findIndex((row) => theNameKey(row.name.bytes) === key) : -1);
  let now = chosen.length > 0 ? at(theMovingEnd) : -1;
  if (now < 0 && chosen.length > 0) {
    // The moving end is not a row a click can choose: the choice's own end
    // in the direction of the step stands in for it.
    const rows = theChosenRows();
    if (rows.length > 0) now = at(theNameKey(rows[by > 0 ? rows.length - 1 : 0].name.bytes));
  }
  const next = now < 0 ? (by > 0 ? 0 : choosable.length - 1) : Math.min(Math.max(now + by, 0), choosable.length - 1);
  const row = choosable[next];
  if (widening && now >= 0) {
    if (next === now) return;
    // From the last row clicked, or from where the step began where that
    // row is gone.
    if (at(theLastClicked) < 0) theLastClicked = theNameKey(choosable[now].name.bytes);
    const from = at(theLastClicked);
    chosen = choosable.slice(Math.min(from, next), Math.max(from, next) + 1).map((one) => one.name.bytes);
    theMovingEnd = theNameKey(row.name.bytes);
  } else {
    if (next === now && chosen.length === 1) return;
    theOneRowIsChosen(row.name.bytes);
  }
  theChosenFileIsShown();
  theChosenRowIsBroughtIntoView();
}

/// **The chosen row is brought into view below the column heading** — the
/// Creator's third notes on the test copy, note 5 (*"up-scrolling is lagging
/// behind one item"*). The heading is pinned to the top of the list
/// (`the-drawing.css`'s sticky `.fhead`), and the browser's own scrolling to
/// the nearest edge took the list's top edge, so going up the row landed under
/// the heading, hidden. **So the list keeps the heading's height clear at its
/// top**, and going down still stops at the bottom edge.
function theChosenRowIsBroughtIntoView() {
  if (!list || !list.children) return;
  // **With several chosen, the row brought into view is the moving end** —
  // the one a Shift with ↑ or ↓ just reached.
  const marked =
    Array.from(list.children).find((one) => theMovingEnd !== null && theRowsNames.get(one) === theMovingEnd) ||
    Array.from(list.children).find((one) => one.getAttribute && one.getAttribute("aria-current") === "true");
  if (!marked) return;
  const head = Array.from(list.children).find((one) => one.classList && one.classList.contains("fhead"));
  const clear = head ? head.offsetHeight : 0;
  const top = marked.offsetTop;
  const end = top + marked.offsetHeight;
  if (top - clear < list.scrollTop) list.scrollTop = Math.max(0, top - clear);
  else if (end > list.scrollTop + list.clientHeight) list.scrollTop = end - list.clientHeight;
}

/// **Return does what a double-click does** — `Design/File Browser`, the
/// eighth notes, note 2, so the keyboard and the mouse keep one rule: the two
/// arrive at the one opening below. **It does nothing while several are
/// chosen** (the thirteenth notes, note 6): `theChosenRow` is the one row, and
/// there is none.
function theChosenRowIsOpened() {
  theRowIsOpened(theChosenRow());
}

/// **A row opened, by a double-click or by Return**: a folder opens by its
/// whole path, as the panel openings do — until 2026-09-19 a folder was asked
/// for by its bare name, so opening `textures` inside a folder asked for a
/// folder `textures` at the top (the Gemini catch-up of step 10a, Area 5,
/// F-5) — and a `.blend` opens Render Settings on its path, as the toolbar's
/// *Render settings…* does. Any other file opens nothing, Files having no act
/// that opens one.
function theRowIsOpened(row) {
  if (!row) return;
  // **A picture opens over the window** (`Design/File Browser § Looking at a
  // picture`, the Creator's seventeenth notes, note 11), and **while the line
  // is down the rows stop responding, so no viewer opens** (`File Browser`
  // L29, L15).
  if (row.picture) {
    if (!theLineIsDown()) theViewerOpensOn(row);
    return;
  }
  if (row.opens) {
    theBrowserIsAskedFor(row.path.bytes);
    return;
  }
  if (!row.is_a_blend || !server) return;
  theSettingsAreOpenedOn(server, row.path.bytes, row.saved_with).catch(() => {});
}

// ── A picture looked at ─────────────────────────────────────────────────────
//
// `Design/File Browser § Looking at a picture`, added 2026-10-31 — the
// Creator's seventeenth notes, note 11: *"simply double-clicking an image and
// instantly seeing a preview is something people would surely expect."* The
// drawing's `#fbview` (`fb-view`, `fb-viewexr`, `fb-viewbig`): Queue's large
// preview box, holding the file itself. **The read is the application's**
// (`a_picture_is_read`, `File Browser` L29) and the bytes are held only as the
// one `blob:` address the picture on screen is drawn from, let go when it is.

/// **The viewer while it is open**, and nothing while it is not.
let theViewer = null;
/// **Which ask the viewer is waiting on** — an answer to any other is about a
/// picture the user has left, and is not drawn.
let theViewersAsk = 0;

/// **Whether the line is down**: the listing is the one last seen, and every
/// act needing the server is unavailable (`File Browser` L15).
function theLineIsDown() {
  return Boolean(said && (said.stale || (said.unavailable || []).length > 0));
}

/// **The folder's pictures, in the list's order** — the rows a click can
/// choose that carry a picture. Rows that are not pictures are skipped;
/// pictures Smoozy cannot show are not.
function theFoldersPictures() {
  return theRowsAClickCanChoose().filter((row) => row.picture);
}

function theViewerOpensOn(row) {
  if (!said || !said.viewer) return;
  if (!theViewer) theViewer = theViewerIsMade(said.viewer);
  theViewerShows(row);
  theViewer.close.focus();
}

/// **Where the box opens: over the whole window**, as Queue's large preview
/// does — the `.body` holding the rail and the screen beside it.
function theViewersHost() {
  const pane = document.getElementById ? document.getElementById("pane") : null;
  const body = pane && Array.from(pane.children).find((one) => one.classList.contains("body"));
  return body || host;
}

function theViewerIsMade(words) {
  const panel = document.createElement("div");
  panel.className = "modal pvlarge fbview";
  panel.setAttribute("role", "dialog");
  panel.setAttribute("aria-modal", "true");
  panel.setAttribute("aria-label", words.picture);
  const box = document.createElement("div");
  box.className = "pvbox";
  const top = document.createElement("div");
  top.className = "pvtitle";
  const title = document.createElement("h3");
  const meta = document.createElement("span");
  meta.className = "fbvmeta";
  const download = document.createElement("button");
  download.type = "button";
  download.className = "mini";
  download.textContent = words.download;
  const close = document.createElement("button");
  close.type = "button";
  close.className = "ib";
  close.setAttribute("aria-label", words.close);
  close.textContent = "\u00d7";
  top.appendChild(title);
  top.appendChild(meta);
  top.appendChild(download);
  top.appendChild(close);
  const place = document.createElement("div");
  place.className = "fbvpic";
  const picture = document.createElement("span");
  picture.hidden = true;
  const say = document.createElement("p");
  say.className = "fbvsay";
  place.appendChild(picture);
  place.appendChild(say);
  const step = document.createElement("div");
  step.className = "fbvstep";
  const previous = document.createElement("button");
  previous.type = "button";
  previous.className = "sb";
  previous.setAttribute("aria-label", words.previous);
  previous.textContent = "\u2039";
  const next = document.createElement("button");
  next.type = "button";
  next.className = "sb";
  next.setAttribute("aria-label", words.next);
  next.textContent = "\u203a";
  step.appendChild(previous);
  step.appendChild(next);
  box.appendChild(top);
  box.appendChild(place);
  box.appendChild(step);
  panel.appendChild(box);
  const made = { panel, title, meta, download, close, picture, say, previous, next, row: null, address: null };
  close.addEventListener("click", () => theViewerCloses());
  previous.addEventListener("click", () => theViewerSteps(-1));
  next.addEventListener("click", () => theViewerSteps(1));
  // ***Download* brings the file home through Download, as it does from the
  // list.**
  download.addEventListener("click", () => {
    if (!server || !made.row || theLineIsDown()) return;
    aTransferIsAskedFor("download", server, [made.row.path.bytes]);
  });
  // **A click outside the box closes it.**
  panel.addEventListener("click", (press) => {
    if (press.target === panel) theViewerCloses();
  });
  theViewersHost().appendChild(panel);
  return made;
}

/// **Escape closes it; ← and → step**, wherever the focus was left while the
/// box is open — heard before the list's own keys, which stand still under it.
function theViewersKeysAreLive() {
  document.addEventListener(
    "keydown",
    (press) => {
      if (!theViewer || press.defaultPrevented) return;
      const held = press.shiftKey || press.metaKey || press.ctrlKey || press.altKey;
      if (press.key === "Escape") theViewerCloses();
      else if ((press.key === "ArrowLeft" || press.key === "ArrowRight") && !held) {
        theViewerSteps(press.key === "ArrowLeft" ? -1 : 1);
      } else return;
      press.preventDefault();
      if (press.stopPropagation) press.stopPropagation();
    },
    true,
  );
}

/// **The address the picture on screen is drawn from is let go**, and so is
/// any read still under way for it.
function theViewersPictureIsLetGo() {
  theViewersAsk += 1;
  if (theViewer && theViewer.address) {
    URL.revokeObjectURL(theViewer.address);
    theViewer.address = null;
  }
}

function theViewerCloses() {
  if (!theViewer) return;
  theViewersPictureIsLetGo();
  shell.core.invoke("the_picture_is_let_go").catch(() => {});
  theViewer.panel.remove();
  theViewer = null;
}

/// **‹ › and ← →: the previous or next picture in the list's order.** At
/// either end the arrow greys, and nothing moves.
function theViewerSteps(by) {
  if (!theViewer || !theViewer.row) return;
  const pictures = theFoldersPictures();
  const at = pictures.findIndex((row) => theNameKey(row.name.bytes) === theNameKey(theViewer.row.name.bytes));
  const to = at + by;
  if (at < 0 || to < 0 || to >= pictures.length) return;
  theViewerShows(pictures[to]);
}

/// **The line where the picture would be**, with whatever stands under it.
function theViewerSays(words, under) {
  theViewer.picture.hidden = true;
  theViewer.picture.replaceChildren();
  theViewer.say.replaceChildren();
  saidInto(theViewer.say, words);
  for (const one of under || []) {
    theViewer.say.appendChild(document.createElement("br"));
    theViewer.say.appendChild(one);
  }
  theViewer.say.hidden = false;
}

/// **One picture in the box.** The list's choice follows it, so closing leaves
/// the last picture chosen.
function theViewerShows(row) {
  const viewer = theViewer;
  theViewersPictureIsLetGo();
  viewer.row = row;
  theOneRowIsChosen(row.name.bytes);
  theChosenFileIsShown();
  theChosenRowIsBroughtIntoView();
  viewer.title.replaceChildren(theValueDrawn(row.name.shown));
  // **The top line gives its size**, and its pixel size once it is shown.
  viewer.meta.replaceChildren();
  if (row.size) viewer.meta.appendChild(theValueDrawn(row.size));
  const pictures = theFoldersPictures();
  const at = pictures.findIndex((one) => theNameKey(one.name.bytes) === theNameKey(row.name.bytes));
  viewer.previous.disabled = at <= 0;
  viewer.next.disabled = at < 0 || at >= pictures.length - 1;
  viewer.download.disabled = theLineIsDown();
  if (row.picture.is === "not-shown") {
    shell.core.invoke("the_picture_is_let_go").catch(() => {});
    theViewerSays(row.picture.said);
    return;
  }
  // **While the line is down an open box keeps its picture, and a step shows
  // *Not connected.*** (`Design/File Browser § Looking at a picture`).
  if (theLineIsDown()) {
    shell.core.invoke("the_picture_is_let_go").catch(() => {});
    theViewerSays(said.viewer.not_connected);
    return;
  }
  thePictureIsRead(row);
}

/// **The picture read**, with *Loading…* where it goes until it comes.
function thePictureIsRead(row) {
  const viewer = theViewer;
  const ask = theViewersAsk;
  theViewerSays(said.viewer.loading);
  shell.core
    .invoke("a_picture_is_read", { server, path: row.path.bytes, listed: row.sorts_by.size })
    .then((bytes) => {
      if (theViewer !== viewer || ask !== theViewersAsk) return;
      thePictureIsDecoded(row, bytes);
    })
    .catch((answer) => {
      if (theViewer !== viewer || ask !== theViewersAsk || !answer || answer.is === "let-go") return;
      if (answer.is !== "not-read") {
        theViewerSays(answer.said);
        return;
      }
      // **A read that fails says so, with the reason under it and *Retry*.**
      // Nothing retries by itself.
      const retry = document.createElement("button");
      retry.type = "button";
      retry.className = "mini";
      retry.textContent = answer.retry;
      retry.addEventListener("click", () => {
        if (theViewer === viewer && viewer.row === row && !theLineIsDown()) theViewerShows(row);
      });
      theViewerSays(answer.said, [saidAsText(answer.why), retry]);
    });
}

/// **Decoding decides**: the application handed over only bytes that open as
/// PNG or JPEG, and the web view's own decode is the second half. A picture
/// that will not decode reads *Could not show this file.*
function thePictureIsDecoded(row, bytes) {
  const viewer = theViewer;
  const ask = theViewersAsk;
  const first = bytes && bytes.byteLength ? new Uint8Array(bytes)[0] : null;
  if (first !== 0x89 && first !== 0xff) {
    theViewerSays(said.viewer.could_not_show);
    return;
  }
  const address = URL.createObjectURL(new Blob([bytes], { type: first === 0x89 ? "image/png" : "image/jpeg" }));
  viewer.address = address;
  const decoding = new Image();
  decoding.onload = () => {
    if (theViewer !== viewer || ask !== theViewersAsk) return;
    const width = decoding.naturalWidth;
    const height = decoding.naturalHeight;
    // **Fitted whole**, as the drawing draws it: a picture in a box of its own
    // proportions, centred and never cut — SVG's own default fitting, which is
    // the drawing's.
    const svg = document.createElementNS(SVG_NS, "svg");
    svg.setAttribute("viewBox", "0 0 " + width + " " + height);
    svg.setAttribute("role", "img");
    svg.setAttribute("aria-label", row.name.shown);
    const image = document.createElementNS(SVG_NS, "image");
    image.setAttribute("href", address);
    image.setAttribute("width", String(width));
    image.setAttribute("height", String(height));
    svg.appendChild(image);
    viewer.picture.replaceChildren(svg);
    viewer.picture.hidden = false;
    viewer.say.hidden = true;
    // *3.9 MB · 1920 × 1080* — two values, side by side.
    viewer.meta.replaceChildren();
    if (row.size) {
      viewer.meta.appendChild(theValueDrawn(row.size));
      viewer.meta.appendChild(textNode(" \u00b7 "));
    }
    viewer.meta.appendChild(theValueDrawn(width + "\u00a0\u00d7\u00a0" + height));
  };
  decoding.onerror = () => {
    if (theViewer !== viewer || ask !== theViewersAsk) return;
    URL.revokeObjectURL(address);
    viewer.address = null;
    theViewerSays(said.viewer.could_not_show);
  };
  decoding.src = address;
}

/// **The box follows the line**: *Download* greys while it is down, and the
/// picture already shown stays.
function theViewerFollowsTheLine() {
  if (theViewer) theViewer.download.disabled = theLineIsDown();
}

/// **Whether a panel covers the server this tab is showing** — piece 5's own
/// answer (`aPanelIsOpenOver` in `web/a-server.js`, which tracks which panel is
/// open), never a second copy kept here. A collapsed panel does not cover the
/// browser, so its keys are the browser's again.
function aPanelCoversTheBrowser() {
  return Boolean(server) && aPanelIsOpenOver(server);
}

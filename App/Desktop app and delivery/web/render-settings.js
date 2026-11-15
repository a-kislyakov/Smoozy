// **The render settings — the larger of the two panels over a server.**
//
// **This piece draws inside piece 5's panel mechanism and builds no panel of
// its own.** Read the head of `web/a-server.js` first: this module registers
// `RENDER_SETTINGS`, is handed `{ head, body }` once per server, and is told
// again when the panel comes back from its strip. The pinned collapse control,
// the strip, the stacking and Escape are all that piece's and none of them is
// touched here.
//
// **The application composes and this file draws.** Every string on the page
// arrives on `the_render_settings_panel`'s answer, already through the
// typography point or the verbatim route (A6, X9), and nothing here writes a
// word of prose. **Every judgement is `App/Render Settings/`'s and
// `App/Rendering/`'s** — which value is the user's and which the file's, what a
// box refuses and what the refusal states, the frame parse, the diamond, the
// version pick, the device warning, the missing-assets case. This file puts
// them where the drawing puts them.
//
// ## The four things that bind whatever a piece writes
//
// They are at the head of `web/smoozy.js` and this file keeps all four:
// results arrive as events (P6); values are content and never markup (H32); no
// inline script and no inline style (H33); nothing repeats on a timer (P9,
// P10).
//
// ## How the page is drawn, and why it is rebuilt whole
//
// **One answer, one drawing.** The page asks `the_render_settings_panel` and
// draws what comes back; an act posts `render-settings:an-act`, and the page
// asks again. **A page that comes back asks again** is the mounting contract's
// own rule (P8), and it is what keeps this file from holding a second copy of
// anything the application already holds — which is the fault this Build stage
// keeps meeting.
//
// **What that costs and what pays for it.** Rebuilding takes the focus, so the
// element that had it is found again by its own `data-box` after the redraw. A
// field commits on `change` — on leaving it, or on Enter — so nothing is
// committed a keystroke at a time. **But an answer about something else can
// arrive while a field is typed in, and the page is redrawn then** — step
// 10c's ninth pass lost a job name that way — so the typing and the caret are
// carried across the redraw (`web/the-typed-text.js`).
//
// ## What this file must never do
//
// - **Never join a name, a figure or a path into a sentence.** X7. Every
//   sentence arrives as a list of parts — words, then the values that belong
//   beside them, then more words — and `saidInto` sets them side by side.
// - **Never rebuild a shape the shared half exports.** The one failure shape is
//   `theFailureShape` in `web/the-failure-shape.js`; the underway mark is
//   `theMarkOnAControl`; the machine's own words are
//   `theMachinesOwnWordsFor`. **The mockup draws this page's own copy of the failure
//   shape as `.rdfail` and it is one of the five this application refuses.**
// - **Never use `innerHTML`, an inline style, or a `data:` URI.** The mockup
//   carries 88 inline `style=` attributes and four uses of `innerHTML`, and
//   every one of them is inert under this application's content policy.
//
// ## What piece 6 needs from this file
//
// **`theSettingsAreOpenedOn(server, file)` is the way in**, and it is the one
// `Design/File Browser § Where it sits` describes: choosing a `.blend` opens
// the render settings. **`file` is the row's exact bytes** (`File Browser`
// L1a), never its display name, and every act hands the same bytes back.

import {
  aPanelIsRegistered,
  thePanelIsOpenedOver,
  theStripReads,
  theCurrentTabIs,
  thePanelCollapsesAsItsChevronDoes,
  RENDER_SETTINGS,
  SET_UP,
  FILES,
} from "./a-server.js";
import { theBrowserIsOpenedAt, theSavedWithVersionReadFor } from "./files.js";
import { theCheckIsOpenedOn, theCheckIsShownOn } from "./the-dependency-check.js";
import { theFormatBoxIsOpenedForAFile } from "./transfers.js";
import { whenTold } from "./event-route.js";
import { theFailureShape, EXPANDED } from "./the-failure-shape.js";
import { theMachinesOwnWordsFor } from "./the-machines-own-words.js";
import { theMarkOnAControl, theElapsed } from "./the-underway-shape.js";
import { theTypingIsTakenFrom, theTypingIsPutBackInto } from "./the-typed-text.js";
import { theValueDrawn, theValueIsDrawnInto } from "./the-value-drawn.js";
// **The moment the band names is written the way every screen writes one** —
// the application hands seconds and this side writes them from the system's
// own locale data. There is one of these in the page and it is
// `web/servers.js`'s.
import { theMomentReads } from "./servers.js";
// ***Show in Queue* opens the queue through the rail**, naming the job it is
// about — the queue's own way in (the Creator's note 10 on the test copy).
import { theCurrentPlaceIs, THE_QUEUE } from "./the-rail.js";
import { theFocusMovesTo } from "./the-focus-moves.js";

const shell = window.__TAURI__;

// Declared beside the service that posts it — `AN_ACT_ON_THIS_PAGE` in
// `src/surface/render_settings.rs`. The two sides have to agree and nothing
// holds them together, so copy carefully.
const AN_ACT_ON_THIS_PAGE = "render-settings:an-act";
// **How many of an answer's leading parts say what went out** —
// `THE_PLAIN_PARTS` in `src/surface/render_settings.rs`.
const THE_PLAIN_PARTS = "plainParts";
// **A submission that went** — `THE_JOB_TO_SHOW` and `SHOW_IN_QUEUE_SAID` in
// the same file: the job *Show in Queue* opens on, and its words. Their
// presence on the submission's answer is what makes it the confirmation.
const THE_JOB_TO_SHOW = "jobToShow";
const SHOW_IN_QUEUE_SAID = "showInQueue";
// `THE_LIST_CAME_BACK` in the same file: `Render Settings` B1's list arriving, which is no change
// the user made to the page.
const THE_LIST_CAME_BACK = "version-list";
// `THE_ANSWER` in `src/surface/set_up.rs`; only a Blender run's ending
// carries `the_render_versions_are_read_again` (`Server Setup` B12).
const THE_SET_UP_ANSWER = "set-up:the-answer";
// `THE_CONNECTION_IS_LENT` in `src/surface/render_settings.rs`: the primary
// connection can serve requests now, after the earlier Connected reading.
const THE_CONNECTION_IS_LENT = "render-settings:the-connection-is-lent";
// `THE_TELEGRAM_ROLE_WAS_READ` in the same file (`Server Setup` G2a).
const THE_TELEGRAM_ROLE_WAS_READ = "render-settings:the-telegram-role-was-read";
// `INSTALL_TELEGRAM` in `src/surface/set_up.rs`: the card's own action.
const INSTALL_TELEGRAM = "install-telegram";


// `THE_SECTION` in `src/surface/telegram.rs`: Telegram's section, posted
// after every move of the switch and every change of the default.
const TELEGRAMS_SECTION = "telegram:the-section";

// ── What this module holds ──────────────────────────────────────────────────

// server → { head, body, file, said } — `file` is the row's bytes.
const panels = new Map();

// ════════════════════════════════════════════════════════════════════════════
// The way in, and the panel's registration
// ════════════════════════════════════════════════════════════════════════════

/// **Choosing a `.blend` opens the render settings** — `Design/File Browser
/// § Where it sits`. **This is piece 6's call**, and it takes the server the
/// browser was showing rather than whichever one happens to be current.
/// **`file` is the row's exact bytes**, which is what the application keys on.
export async function theSettingsAreOpenedOn(server, file, savedWith) {
  const held = thePanelFor(server);
  // **`Render Settings` Y15b: another file's page is the old one left.**
  const left = held.file;
  if (left && Array.from(left).join(",") !== Array.from(file).join(",")) thePageIsLeft(server, left);
  held.file = file;
  // **A different file is a fresh page**, so no line it drew is still holding
  // its space (see `theHeaderIsDrawn`), and no confirmation of a submission
  // made from another page stands on it.
  held.thePickLineThatWent = null;
  held.theCountLineThatWent = null;
  held.theGroupsDrawn = new Map();
  held.theSubmissionThatWent = null;
  // **The saved-with version is `File Browser`'s read, taken without opening
  // the file** — `Render Settings` B2 names it as the automatic match's one
  // source, and this page never reads it for itself. Where piece 6 hands
  // nothing the page draws the *file that does not say which Blender saved it*
  // case, which is one of Design's three and is not a failure.
  await shell.core.invoke("the_file_was_pointed_at", {
    server,
    file,
    savedWith: savedWith || "",
  });
  thePanelIsOpenedOver(server, RENDER_SETTINGS);
  theSettingsAreAskedFor(server);
  // **`Render Settings` B1: the version list is asked for when this page wants
  // it**, which is `Server Setup` B12's own moment. The search runs on the
  // machine and the page is told when it ends, by the same act event.
  shell.core.invoke("the_version_list_is_asked_for", { server, file }).catch(() => {});
}

/// **`Render Settings` Y15b: a loaded range belongs to the one job**, and it goes
/// when its page is left. A collapse to the strip does not come here.
function thePageIsLeft(server, file) {
  shell.core.invoke("the_page_was_left", { server, file }).catch(() => {});
}

aPanelIsRegistered({
  panel: RENDER_SETTINGS,
  itLetsGoOfItsFile(server) {
    const held = panels.get(server);
    if (held && held.file) thePageIsLeft(server, held.file);
  },
  itIsDrawn(parts, server) {
    const held = thePanelFor(server);
    held.head = parts.head;
    held.body = parts.body;
    theSettingsAreAskedFor(server);
  },
  // **Events are volatile and nothing is replayed** (P8), so a panel coming
  // back from its strip asks again.
  itIsShowingAgain(server) {
    theSettingsAreAskedFor(server);
  },
  // **Its strip pressed while it was never opened over this server**: it opens
  // on the check's file and starts nothing — the Creator's seventh notes, note
  // 2. **The saved-with version handed is the one Files read for that file**,
  // kept on Files' own rows (`theSavedWithVersionReadFor`) — step 10c's owed
  // piece 2. Until then none was handed though Files showed the tag, and the
  // application had to read it again over the line. **Where Files holds no
  // read it is blank**, and the application's own fallback stands
  // (`the_file_was_pointed_at`): `File Browser`'s reading where a connection is
  // live, else Design's *file does not say which Blender saved it* case. The
  // page still reads nothing for itself (`Render Settings` B2).
  theFileItIsOn(server) {
    const held = panels.get(server);
    return held ? held.file : null;
  },
  itIsShownOn(server, file) {
    return theSettingsAreOpenedOn(server, file, theSavedWithVersionReadFor(server, file));
  },
});

function thePanelFor(server) {
  let held = panels.get(server);
  if (held) return held;
  held = { head: null, body: null, file: null, said: null, theSubmissionThatWent: null };
  panels.set(server, held);
  return held;
}

async function theSettingsAreAskedFor(server) {
  const held = panels.get(server);
  if (!held || !held.head || !held.body || !held.file) return;
  const said = await shell.core.invoke("the_render_settings_panel", {
    server,
    file: held.file,
  });
  // **A press already on its way is not lost to a redraw** (below).
  await theDrawingWaitsForThePress();
  held.said = said;
  theSettingsAreDrawn(server, said);
}

// ── A press already on its way ──────────────────────────────────────────────
//
// **`Design/Whole app § The window`: *"a press already on its way is not lost
// to a redraw: a page that redraws while a control is held down keeps that
// control, and the press lands where it began."*** Step 10c's eighth pass: a
// field commits on `change`, which fires when the field loses the focus — so
// pressing *Render* straight after typing sent the field's answer, the answer
// came back while the button was still held, and the page was rebuilt whole,
// *Render* included. A press whose control is taken out of the page before the
// release makes no click, so nothing was submitted. **Proved in the page
// harness before this was written, beside a plain press that did submit.**
//
// **So while a control on this page is held down, the drawing waits**, and it
// goes ahead the moment the release's click is under way — the click is
// already aimed at the control the press began on, so the drawing can no
// longer take it away. The page's *one answer, one drawing* rule above is
// untouched: the answer is still drawn whole, a moment later. Only a press on
// a control holds the drawing; a field or a list is the user's to type in or
// open. **The wait ends on events alone, never on a clock** (P9, P10): the
// click, a cancelled press, the window losing the focus, the next press, and —
// where a release made no click at all — the first movement or key after it.
// Added 2026-09-27, eighth-pass fix piece 5.
let thePressHeld = null;
let drawingsWaiting = [];

function theDrawingWaitsForThePress() {
  if (!thePressHeld) return Promise.resolve();
  return new Promise((go) => drawingsWaiting.push(go));
}

function thePressEnded() {
  if (!thePressHeld) return;
  thePressHeld = null;
  const waiting = drawingsWaiting;
  drawingsWaiting = [];
  waiting.forEach((go) => go());
}

document.addEventListener(
  "pointerdown",
  (press) => {
    thePressEnded();
    if (press.button !== 0 || !press.target || !press.target.closest) return;
    const control = press.target.closest('button,[role="button"]');
    if (!control || control.disabled || control.getAttribute("aria-disabled") === "true") return;
    const onThisPage = Array.from(panels.values()).some(
      (held) =>
        (held.head && held.head.contains(control)) ||
        (held.body && held.body.contains(control))
    );
    if (onThisPage) thePressHeld = { released: false };
  },
  true
);
document.addEventListener("click", () => thePressEnded(), true);
document.addEventListener("pointercancel", () => thePressEnded(), true);
document.addEventListener(
  "pointerup",
  () => {
    if (thePressHeld) thePressHeld.released = true;
  },
  true
);
for (const after of ["pointermove", "keydown"]) {
  document.addEventListener(
    after,
    () => {
      if (thePressHeld && thePressHeld.released) thePressEnded();
    },
    true
  );
}
if (typeof window.addEventListener === "function") {
  window.addEventListener("blur", () => thePressEnded());
}

// **An act answers, and the page asks again.** The event carries only what the
// panel cannot: which box was acted on, and the refusal where there was one.
whenTold(AN_ACT_ON_THIS_PAGE, async (delivery) => {
  const said = delivery.said || {};
  const held = panels.get(said.server);
  // **The event names the file as the application keys it**, which is the
  // same text the panel's own answer carries — the page never decodes bytes.
  if (!held || !held.said || said.file !== held.said.file) return;
  theSubmissionThatWentIsKeptOrLetGo(held, said);
  await theSettingsAreAskedFor(said.server);
  theRefusalIsPutOnItsBox(said.server, said);
});

whenTold(THE_SET_UP_ANSWER, async (delivery) => {
  const said = delivery.said || {};
  if (said.the_render_versions_are_read_again !== "yes") return;
  const held = panels.get(said.server);
  if (!held || !held.file) return;
  await shell.core.invoke("the_version_list_is_asked_for", {
    server: said.server,
    file: held.file,
  }).catch(() => {});
});

whenTold(THE_TELEGRAM_ROLE_WAS_READ, (delivery) => theSettingsAreAskedFor((delivery.said || {}).server));

whenTold(THE_CONNECTION_IS_LENT, async (delivery) => {
  const server = (delivery.said || {}).server;
  const held = panels.get(server);
  if (!held || !held.file) return;
  // The same panel survives in its strip. Ask only for the list; opening the
  // file again would replace its page state. The search's normal events redraw.
  await shell.core.invoke("the_version_list_is_asked_for", {
    server,
    file: held.file,
  }).catch(() => {});
});

/// **A submission that went says so under Render, and it stays until
/// something on the page changes or the user submits again — never on a
/// timer** (`Design/Render Settings § Submitting`, the Creator's note 10 on the
/// test copy). **The page is rebuilt whole on every answer**, so the
/// confirmation is held here and the foot draws it each time. Every act on
/// this page answers on this event, so any answer but the submission's own
/// and `Render Settings` B1's list arriving is the page changing, and lets it go; a submission's
/// answer replaces it — *Submitting.* first, so a second press lets the first
/// confirmation go at once.
function theSubmissionThatWentIsKeptOrLetGo(held, said) {
  if (said.theBox === "submit") {
    held.theSubmissionThatWent = said[THE_JOB_TO_SHOW]
      ? { parts: thePartsOf(said), job: said[THE_JOB_TO_SHOW], showInQueue: said[SHOW_IN_QUEUE_SAID] }
      : null;
  } else if (said.theBox !== THE_LIST_CAME_BACK) {
    held.theSubmissionThatWent = null;
  }
}

/// **The confirmation**, as the drawing's `res-sent` draws it: a check mark,
/// the sentence in ordinary ink at the page's own text size, and *Show in
/// Queue*, which opens the queue on the job. **It takes no colour**: green
/// means finished, and a job just sent has not finished.
function theSubmissionThatWentIsDrawn(went) {
  const line = document.createElement("span");
  line.className = "rs-sent";
  const tick = document.createElement("span");
  tick.className = "rs-tick";
  tick.setAttribute("aria-hidden", "true");
  tick.textContent = "✓";
  line.appendChild(tick);
  // **The sentence is cut short where line 2 has no room, whole on hover**, and
  // *Show in Queue* stays whole beside it, so the line never takes one of its
  // own and Render never moves (the UI at every size, D-16 and C9).
  const words = document.createElement("span");
  words.className = "rs-words";
  theSentence(words, went.parts);
  line.appendChild(words);
  line.title = words.textContent;
  line.appendChild(document.createTextNode(" "));
  const show = document.createElement("a");
  show.href = "#";
  show.dataset.box = "show-in-queue";
  show.textContent = went.showInQueue || "";
  show.addEventListener("click", (event) => {
    if (event && event.preventDefault) event.preventDefault();
    theCurrentPlaceIs(THE_QUEUE, went.job);
  });
  line.appendChild(show);
  return line;
}

// **The check's sends move, and this page asks again**, so *Uploading
// dependencies* counts as the files land and goes with the last. The check
// tells its own panel each time a file of its send ends — the same moments its
// count changes — on its own event, which names the server and the file.
// Declared beside the service that posts it — `AN_ACT_ON_THIS_PANEL` in
// `src/surface/the_dependency_check.rs`. Copy carefully. The Creator's note 6
// on 0.1.8.
const AN_ACT_ON_THE_CHECK = "the-dependency-check:an-act";
whenTold(AN_ACT_ON_THE_CHECK, async (delivery) => {
  const said = delivery.said || {};
  const held = panels.get(said.server);
  if (!held || !held.said || said.file !== held.said.file) return;
  await theSettingsAreAskedFor(said.server);
});

// **Telegram's switch moved, so every page already drawn asks again** —
// `Design/Render Settings § How the three sit together`: the choice is there
// while Telegram is on and not there at all while it is off. The switch is
// Settings', and a page left open, or waiting behind another place, is
// otherwise drawn again only when something on it is acted on; until then its
// foot showed the switch as it stood when it was drawn (the sixteenth notes,
// notes 3 and 4). The default a job starts from follows the same way.
whenTold(TELEGRAMS_SECTION, async () => {
  for (const [server, held] of panels) {
    if (held.said) await theSettingsAreAskedFor(server);
  }
});


// ════════════════════════════════════════════════════════════════════════════
// Drawing it
// ════════════════════════════════════════════════════════════════════════════

function theSettingsAreDrawn(server, said) {
  const held = panels.get(server);
  if (!held || !held.head || !held.body) return;
  const hadFocus = document.activeElement;
  // **The header is searched as well as the body** (the Creator's fifth
  // notes, piece U): the version dropdown sits in the header, so until
  // 2026-10-08 a keyboard pick lost its focus to the redraw.
  const theBoxThatHadFocus =
    hadFocus && (held.body.contains(hadFocus) || held.head.contains(hadFocus))
      ? hadFocus.dataset.box
      : null;
  // **Text being typed is not lost to a redraw** — `Design/Whole app § The
  // window`. The field is about to be taken out and drawn again from the last
  // committed answer, so what is typed in it and where the caret is are taken
  // first (`web/the-typed-text.js`). Step 10c's ninth pass: a job name typed
  // into *Name this job* was gone 0.3 s later with the caret still in the
  // field, because an answer arrived while it was typed.
  const theTyping = theTypingIsTakenFrom(held.body);

  held.head.replaceChildren();
  held.body.replaceChildren();
  theHeaderIsDrawn(server, said, held.head);
  theBodyIsDrawn(server, said, held.body);
  // **The body folds to two columns by the panel's own width** (`web/panes.js`,
  // `Design/Render Settings § The body is three columns`), and a panel just
  // opened has not been measured yet. Asked of the module already loaded,
  // and nothing is lost where it cannot be had.
  import("./panes.js").then((panes) => panes.thePagesAreMarked()).catch(() => {});

  // **Every control carries its own `data-box`, so focus comes back to the
  // one that had it** — scene and View Layer boxes, the switches and the
  // crosses included (must-do 18 of the step 10a round 1 audit of these
  // pieces). **A field comes back with what was being typed in it and its
  // caret**, and nothing is committed; a field whose own answer went is drawn
  // from the answer, as every other value is.
  if (theBoxThatHadFocus) {
    const theBox = `[data-box="${CSS.escape(theBoxThatHadFocus)}"]`;
    const again = held.body.querySelector(theBox) || held.head.querySelector(theBox);
    if (again && theTyping) {
      theTypingIsPutBackInto(again, theTyping);
      // **The empty field's dash is a placeholder face** and goes once
      // something is typed, as its own `input` handler does.
      if (again.classList.contains("dashv")) again.classList.toggle("dashv", again.value === "");
    } else if (again) {
      theFocusMovesTo(again);
    }
  }

  // **What the strip says while the panel is down** — the counting is this
  // panel's and the strip is piece 5's. **No mark**: a strip carries none
  // (the Creator's tenth notes, note 5).
  theStripReads(server, RENDER_SETTINGS, {
    title: said.the_strip_title,
    reading: said.the_strip_reading,
  });
}

// ── The header ──────────────────────────────────────────────────────────────

function theHeaderIsDrawn(server, said, head) {
  // **The title the drawing writes — *Render settings — kitchen_v14.blend* —
  // built the way the check's is**: the application's words, then the file's
  // name as a value beside them, both plain (must-do 17).
  const heading = document.createElement("h2");
  const what = document.createElement("span");
  what.textContent = said.title.lead;
  heading.appendChild(what);
  for (const value of said.title.values) {
    heading.appendChild(document.createTextNode(" "));
    // **The file's name is a link into Files** where the application sent
    // somewhere to go — the ninth notes, note 5.
    const named = document.createElement(said.show_in_files ? "button" : "span");
    if (said.show_in_files) theNameShowsTheFileInFiles(server, named, said.show_in_files);
    else named.classList.add("selectable");
    // The file's name, kept apart from the words before it and read in the
    // order its bytes run — `Desktop app and delivery` X9, drawn by
    // `web/the-value-drawn.js` (step 10c's dependency wave piece 10b), and so
    // is every name and path on this page.
    theValueIsDrawnInto(named, value);
    heading.appendChild(named);
  }
  head.appendChild(heading);

  const row = document.createElement("div");
  row.className = "hdrow";
  const label = document.createElement("span");
  label.className = "hk";
  label.textContent = said.blender;
  row.appendChild(label);

  // **Three readings of one control** — a version picked; a list with an empty
  // pick, not disabled; and no dropdown at all where the server has no Blender.
  if (said.the_version_control_is_drawn) {
    const rows = said.the_versions.map((one) => ({
      value: one.build,
      said: one.below_the_floor ? `${one.answered} ${one.below_the_floor}` : one.answered,
      picked: Boolean(one.picked),
    }));
    // **The list is not disabled — picking one is exactly what the user is
    // being invited to do** — and what is empty is the pick.
    const shown = rows.find((one) => one.picked);
    row.appendChild(
      aDropdown({ label: said.blender_version, box: "version", width: "w-ver", rows, shown: shown ? shown.said : "" }, (value) =>
        anAnswerIsGiven(server, said, "version-pick", value)
      )
    );
  }

  const state = document.createElement("span");
  state.className = "sub";
  if (said.no_blender.length) {
    // **A sentence saying the server has no Blender, with the way to put one
    // there** — an empty dropdown would be worse than none. *Set it up* goes to
    // the server's Set up tab (must-do 4).
    state.classList.add("blnone");
    theSentence(state, said.no_blender);
    // **Where the line says the server's disk is full, no way to Set up is
    // given and none is drawn** — nothing there frees a disk (2026-11-11).
    if (said.set_it_up) {
      const setUp = aLink(said.set_it_up, () => theSetUpTabIsShown(server), "set-it-up");
      state.appendChild(document.createTextNode(" "));
      state.appendChild(setUp);
    }
    state.appendChild(document.createTextNode(" · "));
  } else if (said.read_at.length) {
    // **The reading's moment arrives as seconds**, the value the application
    // marks as `read_at_moment`, and is written here the way the band's
    // moment is (2026-11-02: the line read *read at ·* with no time).
    const moment = said.read_at_moment;
    theSentence(
      state,
      moment
        ? said.read_at.map((part) => ({
            ...part,
            values: (part.values || []).map((value) => (value === moment ? theMomentReads(moment) : value)),
          }))
        : said.read_at
    );
    state.appendChild(document.createTextNode(" · "));
  }
  // **The read and *Reset all* stay where they are in all three of the
  // no-version cases and read unavailable where there is nothing to read with**
  // — `§ When there is no version to pick`: *"Not offered means the control
  // stays where it was and reads unavailable … Render and the read alike, in
  // all three cases."* (must-do 12). The application says whether the read is
  // offered (`Render Settings` B5a).
  const read = aLink(
    said.a_reading_stands ? said.read_again : said.read_the_file,
    () => theFileIsRead(server, said),
    "read"
  );
  if (!said.the_read_is_offered) read.disabled = true;
  // **While the read runs the control keeps its words and reads unavailable**,
  // and the turning mark starts the line under the header — `Design/Render
  // Settings § Reading the file`, as changed 2026-11-13 (the Creator's note 3
  // on the test copy of 2026-11-12: a spinner alone where *Read the file* was,
  // far from *Reading the file · 0:14 · Stop*). Until then the mark took the
  // control's place, `theMarkOnAControl`.
  if (said.the_read_is_underway) read.disabled = true;
  // **Why it waits, on hover** — `Design/Whole app § While other work runs`
  // (the Creator's notes on the test copy, second round, note 17).
  if (said.why_the_read_waits) read.title = said.why_the_read_waits;
  state.appendChild(read);
  state.appendChild(document.createTextNode(" · "));
  state.appendChild(
    aLink(said.reset_all, () => aValueIsHandedBack(server, said, "everything"), "reset-all")
  );
  row.appendChild(state);
  head.appendChild(row);

  // **The read's line is drawn in the held line under the version row**, not
  // added above it, so a read moves nothing on the page (UI consistency,
  // finding 15): it covers the pick line while it runs, and the line keeps
  // the height of the taller of the two.
  let activity = null;
  if (said.the_read_is_underway) {
    activity = document.createElement("div");
    activity.className = "sub rdline";
    activity.dataset.box = "read-activity";
    // **The one turning mark, first on the line**, the drawing's `.rdline
    // .spin` — the narrowest underway shape, beside the words that name it.
    const turning = document.createElement("i");
    turning.className = "spin";
    activity.appendChild(turning);
    // **Where the walk is, while the server says** — `Design/Render Settings
    // § A read can take longer`, as changed 2026-11-13 (the Creator's notes on
    // the test copy, second round, note 15): *Reading frame 120 of 250*.
    activity.appendChild(document.createTextNode(said.reading_the_walk || said.reading_the_file));
    if (said.the_read_elapsed_seconds != null) {
      activity.appendChild(document.createTextNode(" · "));
      activity.appendChild(theElapsed(said.the_read_elapsed_seconds));
    }
    activity.appendChild(document.createTextNode(" · "));
    // **Stop is the window's small button, the same as the dependency
    // check's** — `Design/Render Settings § A read can take longer`, as
    // changed 2026-11-13 (the runner's ruling on the test copy's notes, second
    // round): it was a link while the check's had become the small button.
    const stop = anInlineButton(said.the_read_is_stopping ? said.stopping_read : said.stop_read, () => {
      if (stop.disabled) return;
      stop.disabled = true;
      return shell.core.invoke("the_file_read_is_stopped", { server, file: said.file_bytes });
    }, "stop-read");
    stop.disabled = Boolean(said.the_read_is_stopping);
    activity.appendChild(stop);
  }

  // **One line under the version row, and it is held** — `Design/Render
  // Settings § The header carries the Blender version`, the Creator's
  // twenty-eighth notes, note 17. Where the picked version and the file's are
  // different release lines it is the warning (`§ The version mismatch
  // warning`, directly under the version row, the Creator's note 12 on
  // 0.1.7); where Smoozy picked an older Blender because nothing newer is on
  // the server, *Latest on this server.* follows it on the same line; and the
  // no-version sentences sit there too. **The pick is not otherwise
  // described.**
  //
  // **It keeps the height of its longest content**: the warning with *Latest
  // on this server.* after it is drawn unseen beneath whatever the line
  // holds, so no pick and no *Reset all* moves the page. **And a line that
  // went is drawn again, unseen, holding its own height**, until another file
  // is opened — `Design/Whole app § The window`: on the seventh pass on
  // Sesterce the fields jumped up under the pointer when the line went, and
  // 4096 went into Resolution instead of Samples.
  const held = panels.get(server);
  const theLine = said.the_pick_line || (held && held.thePickLineThatWent);
  const anything = said.the_mismatch || theLine || said.the_version_control_is_drawn;
  if (anything) {
    const line = document.createElement("div");
    line.className = "pkline";
    if (said.the_version_line_held.length) {
      const unseen = document.createElement("span");
      unseen.className = "vmm pkheld";
      unseen.setAttribute("aria-hidden", "true");
      theSentence(unseen, said.the_version_line_held);
      unseen.appendChild(document.createTextNode(" "));
      const latest = document.createElement("span");
      latest.className = "sub";
      latest.textContent = said.latest_on_this_server;
      unseen.appendChild(latest);
      line.appendChild(unseen);
    }
    // Where the file records no version there is no warning to hold the
    // line, and the line that went holds it instead.
    else if (held && held.thePickLineThatWent && !said.the_pick_line) {
      const went = document.createElement("span");
      went.className = "sub pkheld";
      went.setAttribute("aria-hidden", "true");
      went.textContent = held.thePickLineThatWent;
      line.appendChild(went);
    }
    const shown = document.createElement("span");
    shown.className = said.the_mismatch ? "vmm" : "sub";
    if (said.the_mismatch) theSentence(shown, said.the_mismatch.line);
    if (said.the_pick_line) {
      const words = document.createElement("span");
      words.className = "sub";
      words.textContent = said.the_pick_line;
      // **The Blender search is work no control started — opening the page
      // starts it — so the line that will answer it carries the turning
      // mark**: `Design/Whole app § Anything that isn't instant shows that
      // it's underway`, *"the reading it produced carries the mark itself."*
      // The mark sits inside the line, so the line's height is the same with
      // it and without it. Added 2026-09-27, eighth-pass fix piece 5.
      if (said.the_search_is_underway) {
        words.classList.add("underway-reading");
        const turning = document.createElement("i");
        turning.className = "spin";
        words.appendChild(turning);
      }
      if (said.the_mismatch) shown.appendChild(document.createTextNode(" "));
      shown.appendChild(words);
    }
    // **The read's own line, unseen**, so the held line is as tall as the
    // read's line with its Stop (UI consistency, finding 15).
    line.appendChild(theReadLineHeld(said));
    // **What the read answered**, then the read's line, then the pick line:
    // the first of them shown covers those after it (the drawing's `.pkline
    // > .rdline`), so *Read stopped.* sits where the read's line was, at the
    // header's left edge (UI consistency, finding 48).
    line.appendChild(anAnswerLine("read"));
    if (activity) line.appendChild(activity);
    line.appendChild(shown);
    head.appendChild(line);
  } else {
    if (activity) head.appendChild(activity);
    // **What the read answered, where it answered with a sentence** — the
    // line an act's answer is put on (`theAnswerIsPutOnItsLine`).
    head.appendChild(anAnswerLine("read"));
  }
  if (held) held.thePickLineThatWent = said.the_pick_line || (held && held.thePickLineThatWent) || null;

  // **A Blender that did not answer when asked is named, and nothing else
  // about the search** (`Server Setup` B11, Creator-stated 2026-10-02). On an
  // ordinary machine there is none and the block is not there at all.
  if (said.the_search_marks.length) {
    const foot = document.createElement("div");
    foot.className = "vlfoot";
    for (const mark of said.the_search_marks) {
      const one = document.createElement("span");
      saidInto(one, mark);
      foot.appendChild(one);
    }
    head.appendChild(foot);
  }
}

// ── The body ────────────────────────────────────────────────────────────────

function theBodyIsDrawn(server, said, body) {
  theBandsAreDrawn(server, said, body);

  const columns = document.createElement("div");
  columns.className = "cols";
  // **The columns are the part of this page that scrolls, and they keep their
  // place across a redraw** — `Design/Whole app § The window`, kept by
  // `web/the-scrolled-place.js` for the name given here. The Creator's second
  // notes on the test copy, note 7: the cross handing a View Layer back threw
  // the page to its top, because every answer draws the columns anew. Another
  // file's settings is another name, and starts at its top.
  columns.dataset.place = JSON.stringify(["render-settings", server, said.file]);
  columns.appendChild(theLeftColumn(server, said));
  columns.appendChild(theMiddleColumn(server, said));
  columns.appendChild(theRightColumn(server, said));
  body.appendChild(columns);

  body.appendChild(theFoot(server, said));
}

function theBandsAreDrawn(server, said, body) {
  // **Said once at the top rather than as a placeholder in eight fields.**
  if (said.the_band.length) {
    const band = document.createElement("p");
    band.className = "noread";
    theSentence(band, said.the_band);
    body.appendChild(band);
  }
  // **Where this page came from, and it is only here when the page was opened
  // by Submit again or Resume.**
  if (said.where_it_came_from.length) {
    const band = document.createElement("div");
    band.className = said.where_it_came_from_is_waiting
      ? "agnband wait"
      : "agnband";
    const line = document.createElement("p");
    // **The band's moment arrives as seconds**, the value the application
    // marks as `where_it_came_from_moment`, and is written here as a date
    // (the 38's piece 2).
    const moment = said.where_it_came_from_moment;
    const withTheDate = moment
      ? said.where_it_came_from.map((part) => ({
          ...part,
          values: (part.values || []).map((value) => (value === moment ? theMomentReads(moment) : value)),
        }))
      : said.where_it_came_from;
    theSentence(line, withTheDate);
    band.appendChild(line);
    body.appendChild(band);
  }
  // **The file's own scripts will run on the server**, said once, only for a
  // file the reading found them in (`Render Settings` V3a; the leftovers of
  // the Rough what-if answers, 222 and 227).
  if (said.the_files_scripts_run) {
    const band = document.createElement("p");
    band.className = "noread";
    band.textContent = said.the_files_scripts_run;
    body.appendChild(band);
  }
  // **A newer copy of this file has not finished uploading**, said once (the
  // leftovers of the Rough what-if answers, 185).
  if (said.a_newer_copy_is_uploading) {
    const band = document.createElement("p");
    band.className = "noread";
    band.textContent = said.a_newer_copy_is_uploading;
    body.appendChild(band);
  }
  // **All four endings take the whole-app failure shape, attached to the
  // control that failed** — which is the Read control in the header, so none of
  // them carries a retry of its own.
  if (said.the_read_failure) {
    const failure = said.the_read_failure;
    const controls = [];
    if (failure.the_raw_text) {
      controls.push({
        is: "raw-text",
        label: failure.the_raw_text,
        act: "the-raw-text",
      });
    }
    controls.push({ is: "cross", label: failure.clear_this, act: "" });
    body.appendChild(
      theFailureShape({
        sentence: failure.what_did_not_happen,
        whatToDoAboutIt: failure.what_to_do_about_it,
        kind: "condition",
        identityOrMark: failure.which,
        values: failure.values.map((one) => [one.said, one.value]),
        controls,
        room: EXPANDED,
        onAct: () =>
          theMachinesOwnWordsFor(said.the_machines_words_subject, server),
        onClear: () =>
          shell.core.invoke("the_read_failure_is_cleared", {
            server,
            file: said.file_bytes,
          }),
      })
    );
  }
}

// ── The left column ─────────────────────────────────────────────────────────

function theLeftColumn(server, said) {
  const column = document.createElement("div");
  column.className = "col";

  // **The scenes are boxes**, and the group is absent rather than empty where
  // nothing has been read: a name only comes out of a file somebody read.
  if (said.the_scene_group_is_drawn) {
    const group = aGroup(said.scene_heading);
    const boxes = document.createElement("div");
    boxes.className = "boxes";
    for (const scene of said.the_scenes) {
      boxes.appendChild(
        aBox(scene.name, scene.ticked, ["plain"], () =>
          shell.core.invoke("a_scene_is_ticked", {
            server,
            file: said.file_bytes,
            scene: scene.name,
            on: !scene.ticked,
          }),
          `scene-${scene.name}`
        )
      );
    }
    if (said.all_scenes) {
      // ***All scenes* sits apart, under a rule and in bold**, because it is
      // not a fourth scene — it is a different act that submits one job per
      // scene, and its tag says how many.
      const all = aBox(
        said.all_scenes.name,
        said.all_scenes.ticked,
        ["plain", "allsc"],
        () =>
          shell.core.invoke("every_scene_is_ticked", {
            server,
            file: said.file_bytes,
            on: !said.all_scenes.ticked,
          }),
        "all-scenes"
      );
      if (said.all_scenes.tag) {
        const tag = document.createElement("span");
        tag.className = "tag";
        saidInto(tag, said.all_scenes.tag);
        all.appendChild(tag);
      }
      boxes.appendChild(all);
    }
    group.appendChild(boxes);
    column.appendChild(group);
  }

  // **Frames: one field, not two.**
  const frames = aGroup(said.frames_heading, said.frames_qualifier);
  const set = aSetting(said.the_range);
  set.dataset.box = "range";
  set.appendChild(theLabel(said.the_range.label));
  const field = theField(said.the_range, "w-list", (value) =>
    anAnswerIsGiven(server, said, "frame-range", value)
  );
  field.dataset.box = "range-field";
  set.appendChild(field);
  if (said.the_range.hand_back_to) {
    set.appendChild(
      theCross(said.the_range.hand_back_to, () =>
        aValueIsHandedBack(server, said, "frame-range")
      , "range-cross")
    );
  } else set.appendChild(theCrossesRoom());
  set.appendChild(theTip(said));
  if (said.the_frame_count.length) {
    // **The frame count sits beside it**, which is what shows the user they got
    // what they meant.
    const after = document.createElement("span");
    after.className = "after";
    theSentence(after, said.the_frame_count);
    set.appendChild(after);
  }
  frames.appendChild(set);
  // **No scene ticked greys the range too** (`Design/Render Settings §
  // Scenes`, *None ticked*; the Creator's twenty-ninth notes, note 6): there
  // is no set of scenes to impose a range on.
  if (said.no_scene_is_ticked) frames.classList.add("greyed");
  column.appendChild(frames);

  // **View Layers: the setting where what is shown and what the job does are
  // not the same thing, so it carries the most marking.**
  if (said.the_view_layer_group_is_drawn) {
    const group = aGroup(said.view_layers_heading, said.view_layers_qualifier);
    const boxes = document.createElement("div");
    boxes.className = "boxes";
    for (const layer of said.the_view_layers) {
      boxes.appendChild(aViewLayer(server, said, layer));
    }
    group.appendChild(boxes);
    column.appendChild(group);
  }
  return column;
}

function aViewLayer(server, said, layer) {
  const pinned = layer.whose === "the-users";
  // **Each row is the tick and the name** — `Design/Render Settings § View
  // Layers`, the Creator's twenty-eighth notes, note 17. The tick sits level
  // with the name's first line and a long name wraps under itself; the
  // layer's own count is a second line under the name, at its indent. Until
  // then a long name wrapped beside a tag that wrapped too, starting at a
  // different place on every row.
  const row = document.createElement("span");
  row.className = "vlrow";
  // The box is drawn after the name and its slot, below.
  // **A View Layer whose renderability is animated in the file carries
  // Blender's keyframe diamond**: solid where it is keyed on the frame being
  // read, hollow where it is animated but not keyed there. **A pinned box
  // carries none**, the pin having severed the file's animation.
  let diamond = null;
  if (layer.diamond) {
    diamond = document.createElement("span");
    diamond.className = layer.diamond === "solid" ? "dia keyed" : "dia";
    if (layer.diamond_said) diamond.title = theWordsOf(layer.diamond_said);
  }
  // **The diamond and the cross share one slot after the name's last word**,
  // and the name is laid over a copy of itself in the pinned box's bold with
  // that slot filled, drawn unseen — so the row is as tall as the pinned
  // name, and pinning or handing back never moves the rows below.
  const cell = document.createElement("span");
  cell.className = "vln";
  const bold = theNameAndItsSlot(layer.name, null);
  bold.className = "vlb";
  bold.setAttribute("aria-hidden", "true");
  cell.appendChild(bold);
  cell.appendChild(theNameAndItsSlot(layer.name, diamond));
  const box = aBox(layer.name, layer.ticked, pinned ? ["pinned"] : [], () =>
    shell.core.invoke("a_view_layer_is_pinned", {
      server,
      file: said.file_bytes,
      layer: layer.name,
      on: !layer.ticked,
    }),
    `layer-${layer.name}`,
    cell
  );
  row.appendChild(box);
  if (layer.hand_back_to) {
    // **The cross sits outside the box**: the whole box is the switch, so a
    // cross inside it would be a press that also toggled the thing it was
    // undoing. It is drawn over the name's place, after its last word, on an
    // unseen copy of the name.
    const over = document.createElement("span");
    over.className = "vlx";
    over.appendChild(
      theNameAndItsSlot(
        layer.name,
        theCross(layer.hand_back_to, () =>
          shell.core.invoke("a_pin_is_handed_back", {
            server,
            file: said.file_bytes,
            layer: layer.name,
          })
        , `layer-cross-${layer.name}`)
      )
    );
    row.appendChild(over);
  }
  // **A View Layer carrying its own sample count says the number**, and says
  // it is overridden where the user has typed one: *100 samples*, *100
  // samples, overridden*. A layer not known without a read says so on the
  // same line.
  if (layer.not_known || layer.tag) {
    const tag = document.createElement("span");
    tag.className = "tag";
    if (layer.not_known) tag.appendChild(document.createTextNode(layer.not_known));
    if (layer.not_known && layer.tag) tag.appendChild(document.createTextNode(" · "));
    if (layer.tag) saidInto(tag, layer.tag);
    row.appendChild(tag);
  }
  return row;
}

/// **A name, then its slot**, held to its last word by a word joiner so the
/// slot never wraps onto a line of its own. **The slot is there on every row,
/// empty or not**, so a name wraps as if it were filled.
function theNameAndItsSlot(name, inTheSlot) {
  const said = document.createElement("span");
  said.className = "vlt";
  said.appendChild(theValueDrawn(name));
  said.appendChild(document.createTextNode("⁠"));
  const slot = document.createElement("span");
  slot.className = "vls";
  if (inTheSlot) slot.appendChild(inTheSlot);
  said.appendChild(slot);
  return said;
}

// ── The middle column ───────────────────────────────────────────────────────

function theMiddleColumn(server, said) {
  const column = document.createElement("div");
  column.className = "col";
  const group = aGroup(said.render_heading);
  // **Everything but the frame range greys, and a greyed control shows no
  // value**, because with several scenes ticked there is no one scene to read
  // from.
  if (said.the_render_group_is_greyed) group.classList.add("greyed");

  // Samples.
  const samples = aSetting(said.samples);
  samples.appendChild(theLabel(said.samples.label));
  const box = theField(said.samples, "w-num", (value) =>
    anAnswerIsGiven(server, said, "sample-count", value)
  );
  box.dataset.box = "samples-field";
  samples.appendChild(box);
  if (said.samples.hand_back_to) {
    samples.appendChild(
      theCross(said.samples.hand_back_to, () =>
        aValueIsHandedBack(server, said, "sample-count")
      , "samples-cross")
    );
  }
  samples.dataset.box = "samples";
  group.appendChild(samples);
  // **Neither line is drawn before the file has been read.** **The file's
  // own setup takes the grey edge, and only the line a typed count causes
  // stays amber** — `Design/Render Settings § When the override warning
  // goes`, the Creator's twenty-eighth notes, note 17.
  for (const line of said.the_samples_lines) {
    const hint = document.createElement("p");
    hint.className = said.the_samples_lines_are_the_files ? "hint flat" : "hint";
    theSentence(hint, line);
    group.appendChild(hint);
  }
  // **The line counting the View Layers that keep their own counts keeps its
  // space when it goes** — the same section: unticking the last of them takes
  // it away, and every field under it would move up. It is drawn again,
  // unseen, until another file is opened or a count is typed, which turns the
  // two lines into one.
  const held = panels.get(server);
  if (held) {
    const stands = said.the_own_counts_line_stands;
    if (stands) held.theCountLineThatWent = said.the_samples_lines[said.the_samples_lines.length - 1];
    else if (held.theCountLineThatWent && said.the_samples_lines_are_the_files) {
      const hint = document.createElement("p");
      hint.className = "hint flat held";
      hint.setAttribute("aria-hidden", "true");
      theSentence(hint, held.theCountLineThatWent);
      group.appendChild(hint);
    }
  }

  if (said.simplify) group.appendChild(theSimplify(server, said));

  // **The percentage is the setting; the pixels are a reading beside it.**
  const resolution = aSetting(said.resolution);
  resolution.dataset.box = "resolution";
  resolution.appendChild(theLabel(said.resolution.label));
  const percentage = theField(said.resolution, "w-num", (value) =>
    anAnswerIsGiven(server, said, "resolution-percentage", value)
  );
  percentage.dataset.box = "resolution-field";
  const unit = document.createElement("span");
  unit.className = "unit";
  unit.textContent = "%";
  // **The field and its unit never part across two lines** — `Design/Whole
  // app § Typography`, *a number and its unit are never split*. The row wraps
  // (`.set`), so the two are held in one piece that does not.
  const withItsUnit = document.createElement("span");
  withItsUnit.className = "withunit";
  withItsUnit.appendChild(percentage);
  withItsUnit.appendChild(unit);
  resolution.appendChild(withItsUnit);
  if (said.resolution.hand_back_to) {
    resolution.appendChild(
      theCross(said.resolution.hand_back_to, () =>
        aValueIsHandedBack(server, said, "resolution-percentage")
      , "resolution-cross")
    );
  } else resolution.appendChild(theCrossesRoom());
  const reading = document.createElement("span");
  if (said.the_size_reading) {
    reading.className = "reading mono";
    reading.textContent = said.the_size_reading;
  } else {
    // **Where there is no size to work out, the reading says so rather than
    // going blank** — the same words either way.
    reading.className = "reading dim";
    reading.textContent = said.the_size_is_not_known || "";
  }
  resolution.appendChild(reading);
  group.appendChild(resolution);
  // **A number Smoozy will not take turns the field red and says the range, and
  // says what Blender would otherwise have done silently.**
  const refusal = document.createElement("p");
  refusal.className = "alertnote";
  refusal.dataset.box = "refusal";
  refusal.hidden = true;
  group.appendChild(refusal);

  group.appendChild(theEngine(server, said));
  if (said.renders_on) group.appendChild(theRendersOn(server, said));
  // **`Render Settings` D6a: one line under the pair, where a Cycles job on the
  // GPU cannot render on this server's driver.** The page's own warning style,
  // as the line under the samples box. It stops nothing.
  if (said.renders_on && said.renders_on.the_driver_line.length) {
    const hint = document.createElement("p");
    hint.className = "hint";
    hint.dataset.box = "device-driver";
    theSentence(hint, said.renders_on.the_driver_line);
    group.appendChild(hint);
  }
  column.appendChild(group);
  return column;
}

function theEngine(server, said) {
  const engine = said.engine;
  const set = document.createElement("div");
  set.className = "set fromfile tall";
  set.appendChild(theLabel(engine.label));
  const rows = document.createElement("span");
  rows.className = "rads";

  // **The control has a third row that is not a choice**: above Cycles and
  // EEVEE sits the file's own engine, by name, marked as the file's — and it is
  // what the setting is on until the user picks one of the two.
  const theFiles = document.createElement("span");
  theFiles.className = engine.on === "the-files" ? "rad on" : "rad";
  theFiles.appendChild(document.createElement("i"));
  const named = document.createElement("em");
  if (engine.not_read_yet) {
    named.className = "dashv";
    named.textContent = engine.not_read_yet;
  } else if (engine.the_files_own) {
    named.textContent = engine.the_files_own;
    if (engine.the_files_own_said) {
      named.appendChild(document.createTextNode(" "));
      const words = document.createElement("span");
      words.textContent = engine.the_files_own_said;
      named.appendChild(words);
    }
  }
  theFiles.appendChild(named);
  if (engine.the_files_own || engine.not_read_yet) rows.appendChild(theFiles);

  for (const [which, label] of [
    ["cycles", engine.cycles],
    ["eevee", engine.eevee],
  ]) {
    const one = document.createElement("span");
    one.className = engine.on === which ? "rad on" : "rad";
    one.dataset.box = `engine-${which}`;
    one.setAttribute("role", "button");
    one.tabIndex = 0;
    one.appendChild(document.createElement("i"));
    one.appendChild(document.createTextNode(label));
    const press = () => anAnswerIsGiven(server, said, "engine-pick", which);
    one.addEventListener("click", press);
    one.addEventListener("keydown", (event) => {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        press();
      }
    });
    if (engine.on === which && engine.hand_back_to) {
      const row = document.createElement("span");
      row.className = "radx";
      row.appendChild(one);
      row.appendChild(
        theCross(engine.hand_back_to, () =>
          aValueIsHandedBack(server, said, "engine-pick")
        , "engine-cross")
      );
      rows.appendChild(row);
      continue;
    }
    rows.appendChild(one);
  }
  set.appendChild(rows);

  const held = document.createElement("div");
  held.appendChild(set);
  const note = document.createElement("p");
  note.className = "bar-note underval";
  note.textContent = engine.note;
  held.appendChild(note);
  // **The one thing this page says about add-ons, and it sits with the engine
  // rather than in a place of its own.** It carries no control: Smoozy
  // installs no add-ons (the Creator, 2026-10-08).
  if (engine.the_add_on_case) {
    const line = document.createElement("p");
    line.className = "addon underval";
    const words = document.createElement("span");
    saidInto(words, engine.the_add_on_case.lead);
    words.appendChild(document.createTextNode(" "));
    const second = document.createElement("span");
    second.textContent = engine.the_add_on_case.read_back_as_eevee;
    words.appendChild(second);
    line.appendChild(words);
    held.appendChild(line);
  }
  return held;
}

function theRendersOn(server, said) {
  const device = said.renders_on;
  const set = document.createElement("div");
  set.className = "set fromfile tall";
  // **With several scenes ticked the pair stays, greyed and with nothing on**
  // — `§ Scenes`: a control holding a value stays and empties (must-do 16).
  if (device.greyed) set.classList.add("greyed");
  set.appendChild(theLabel(device.label));
  const rows = document.createElement("span");
  rows.className = "rads";
  if (device.not_read_yet) {
    // **The promise row is on only where the setting is on it** — the engine's
    // own rule above. Where the user has answered with nothing read, their
    // answer is the row that is on (`Render Settings` J1; step 10c's sixth
    // pass, where *CPU* was taken and this row stayed on).
    const promise = document.createElement("span");
    promise.className = device.on === "the-promise-form" ? "rad on" : "rad";
    promise.appendChild(document.createElement("i"));
    const named = document.createElement("em");
    named.className = "dashv";
    named.textContent = device.not_read_yet;
    promise.appendChild(named);
    rows.appendChild(promise);
  }
  for (const [which, label] of [
    ["the-cards", device.gpu],
    ["the-processor", device.cpu],
  ]) {
    const one = document.createElement("span");
    one.className = device.on === which ? "rad on" : "rad";
    one.dataset.box = `device-${which}`;
    one.setAttribute("role", "button");
    one.tabIndex = 0;
    one.appendChild(document.createElement("i"));
    one.appendChild(document.createTextNode(label));
    // **Where the setting stands at CPU on a server with GPUs, the CPU option
    // carries a tag and nothing else is said** — a fact, not a suggestion to
    // change anything.
    if (which === "the-processor" && device.tag.length) {
      const tag = document.createElement("span");
      tag.className = "tag";
      // **The *last seen* moment arrives as seconds**, the value the
      // application marks as `the_moment`, and is written here as a date or a
      // time, the way the band's moment is.
      const moment = device.the_moment;
      theSentence(
        tag,
        moment
          ? device.tag.map((part) => ({
              ...part,
              values: (part.values || []).map((value) => (value === moment ? theMomentReads(moment) : value)),
            }))
          : device.tag
      );
      one.appendChild(tag);
    }
    const press = () => anAnswerIsGiven(server, said, "device", which);
    if (device.greyed) {
      one.setAttribute("aria-disabled", "true");
      rows.appendChild(one);
      continue;
    }
    one.addEventListener("click", press);
    one.addEventListener("keydown", (event) => {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        press();
      }
    });
    if (device.on === which && device.hand_back_to) {
      const row = document.createElement("span");
      row.className = "radx";
      row.appendChild(one);
      row.appendChild(
        theCross(device.hand_back_to, () =>
          aValueIsHandedBack(server, said, "device")
        , "device-cross")
      );
      rows.appendChild(row);
      continue;
    }
    rows.appendChild(one);
  }
  set.appendChild(rows);
  return set;
}

/// **Simplify belongs to the re-submission of a failed job** — `Rendering`'s,
/// which the Creator ruled goes through Design drawn rather than written.
function theSimplify(server, said) {
  const simplify = said.simplify;
  const held = document.createElement("div");
  held.className = simplify.on ? "simp on" : "simp";
  const line = document.createElement("div");
  line.className = "adl";
  const control = document.createElement("button");
  control.type = "button";
  control.className = simplify.on ? "sws on" : "sws";
  control.dataset.box = "simplify";
  control.setAttribute("aria-label", simplify.label);
  control.appendChild(document.createElement("i"));
  control.addEventListener("click", () =>
    anAnswerIsGiven(server, said, "simplify", simplify.on ? "off" : "on")
  );
  line.appendChild(control);
  const label = document.createElement("span");
  label.className = "adlt";
  label.textContent = simplify.label;
  line.appendChild(label);
  held.appendChild(line);
  const note = document.createElement("p");
  note.className = "hint";
  note.textContent = simplify.said;
  held.appendChild(note);
  // **The three, under Blender's own names**, and they appear only once the
  // switch is on.
  // **Each takes an answer** (`Rendering` A1d): the two numbers are typed,
  // refused in place with the range alone, and the texture limit is a pick
  // over the file's own list. Until 2026-10-03 all three were drawn as text
  // nobody could change.
  if (simplify.on) {
    const settings = document.createElement("div");
    settings.className = "simpset";
    const [subdivision, particles, texture] = simplify.settings;
    for (const [one, which] of [
      [subdivision, "simplify-subdivision"],
      [particles, "simplify-child-particles"],
    ]) {
      if (!one) continue;
      const set = aSetting(one);
      set.appendChild(theLabel(one.label));
      const box = theField(one, "w-num", (value) => anAnswerIsGiven(server, said, which, value));
      box.dataset.box = `${which}-field`;
      set.appendChild(box);
      if (one.hand_back_to) {
        set.appendChild(
          theCross(one.hand_back_to, () => aValueIsHandedBack(server, said, which), `${which}-cross`)
        );
      }
      set.dataset.box = which;
      settings.appendChild(set);
    }
    if (texture) settings.appendChild(theTextureLimit(server, said, texture, simplify.texture_choices));
    held.appendChild(settings);
  }
  return held;
}

/// **Texture Limit: a pick over the file's own list, which refuses nothing**
/// (`Rendering` A1d). The choices and their words arrive with the panel.
/// **Where neither the user nor the file has a value the pick shows the em
/// dash**, `Render Settings` J1's third face, rather than a choice nobody made.
function theTextureLimit(server, said, field, choices) {
  const which = "simplify-texture-limit";
  const set = aSetting(field);
  set.appendChild(theLabel(field.label));
  const rows = (choices || []).map((choice) => ({
    value: choice.value,
    said: choice.said,
    picked: choice.value === field.value,
  }));
  const shown = rows.find((one) => one.picked);
  set.appendChild(
    // **One width whatever is picked**, the drawing's `.w-tex`, so the cross
    // after it stays put (UI consistency, finding 62).
    aDropdown({ label: field.label, box: `${which}-field`, width: "w-tex", rows, shown: shown ? shown.said : field.value || "—" }, (value) =>
      anAnswerIsGiven(server, said, which, value)
    )
  );
  if (field.hand_back_to) {
    set.appendChild(theCross(field.hand_back_to, () => aValueIsHandedBack(server, said, which), `${which}-cross`));
  }
  set.dataset.box = which;
  return set;
}

// ── The right column ────────────────────────────────────────────────────────

function theRightColumn(server, said) {
  const column = document.createElement("div");
  column.className = "col";
  // **A heading is drawn only where something is under it** — the drawing's
  // own script: *"a page with none has nothing to put there"* (must-do 15).
  if (said.the_output_group_is_drawn) column.appendChild(theOutputGroup(server, said));
  if (said.the_dependencies_group_is_drawn) {
    // **The dependency warning sits in the right-hand column, and it is a band
    // rather than a line.**
    const dependencies = aGroup(said.dependencies_heading);
    dependencies.appendChild(theDependencyBand(server, said));
    column.appendChild(dependencies);
  }
  return column;
}

function theOutputGroup(server, said) {
  const output = aGroup(said.output_heading);
  // **With several scenes ticked, and a read from an older server program
  // that holds one scene, one line stands in the group's place**: what a job
  // writes is the read of one scene and there is no scene in hand.
  if (said.one_scene_only.length) {
    const line = document.createElement("p");
    line.className = "bar-note";
    theSentence(line, said.one_scene_only);
    output.appendChild(line);
  }
  // **With several scenes ticked, each one's list under its name**, as one
  // scene's is drawn (`Design/Render Settings § What it will write`, the four
  // questions, scout 2.15, 2026-11-12). No folder line: each scene is its own
  // job with its own folder.
  for (const scene of said.the_scenes_outputs || []) {
    const name = document.createElement("p");
    name.className = "bar-note";
    name.dataset.box = `output-scene-${scene.scene}`;
    const bold = document.createElement("b");
    theValueIsDrawnInto(bold, scene.scene);
    name.appendChild(bold);
    output.appendChild(name);
    theWritesAreDrawnInto(output, server, scene, `${scene.scene}\u0000`);
  }
  theWarningsAreDrawnInto(output, said.the_output_warnings);
  // **Above the list, one line says where the job's folder will sit**, or why
  // it falls back (`Design/Render Settings § What it will write`; the
  // Creator's note 18 on their second notes).
  if (said.the_folder_line) {
    const line = document.createElement("p");
    line.className = "bar-note";
    line.dataset.box = "the-jobs-folder";
    theSentence(line, said.the_folder_line);
    line.querySelectorAll("b").forEach((value) => value.classList.add("selectable"));
    output.appendChild(line);
  }
  // **A second line under it where the server's fallback folder is ruled out
  // before submitting** — the drawing's `res-fallchosen`, in the same grey
  // (`Design/Render Settings § What it will write`; the Creator's second
  // notes, piece 4).
  if (said.the_second_folder_line) {
    const line = document.createElement("p");
    line.className = "bar-note";
    line.dataset.box = "the-jobs-fallback-folder";
    theSentence(line, said.the_second_folder_line);
    output.appendChild(line);
  }
  theListAndItsLinesAreDrawnInto(output, server, said, "");
  return output;
}

/// **One scene's warnings, list and lines**, under its name — the same three
/// one scene's page draws.
function theWritesAreDrawnInto(output, server, said, key) {
  theWarningsAreDrawnInto(output, said.the_output_warnings);
  theListAndItsLinesAreDrawnInto(output, server, said, key);
}

/// **The two the read finds that are warnings rather than lines**, above the
/// list, and neither blocks Render.
function theWarningsAreDrawnInto(output, warnings) {
  for (const warning of warnings) {
    const held = document.createElement("div");
    held.className = "owwarn";
    held.dataset.box = `output-${warning.which}`;
    const first = document.createElement("p");
    const lead = document.createElement("b");
    lead.textContent = warning.lead;
    first.appendChild(lead);
    first.appendChild(document.createTextNode(" "));
    const rest = document.createElement("span");
    rest.textContent = warning.rest;
    first.appendChild(rest);
    held.appendChild(first);
    // **Every colliding pair, one after another** (must-do 14). **Each of the
    // two names where it comes from as well as where it lands**, because the
    // path alone is the same string twice.
    for (const sides of warning.pairs) {
      const pair = document.createElement("p");
      pair.className = "owpair";
      for (const side of sides) {
        const one = document.createElement("span");
        theSentence(one, side);
        pair.appendChild(one);
      }
      held.appendChild(pair);
    }
    output.appendChild(held);
  }
}

/// **The list and the lines under it.** `key` keeps one scene's open or
/// closed groups apart from another's.
function theListAndItsLinesAreDrawnInto(output, server, said, key) {
  if (said.what_it_will_write) {
    const writes = document.createElement("div");
    writes.className = "writes";
    // **One row per folder** — `Design/Render Settings § What it will write`,
    // the Creator's twenty-eighth notes, note 17, and `Job files and outputs`
    // P11: the set of folders. Siblings sit under their parent, written once;
    // a group of more than eight starts closed.
    for (const row of said.what_it_will_write.rows) {
      if (row.group) theGroupIsDrawnInto(writes, server, row, key);
      else aRowIsDrawnInto(writes, row, false);
    }
    const total = document.createElement("span");
    total.className = "p tot";
    saidInto(total, said.what_it_will_write.total);
    writes.appendChild(total);
    said.what_it_will_write.by_format.forEach((one, at) => {
      const which = at === 0 ? "tot" : "tot2";
      writes.appendChild(aCell(["n", which, "mono"], one.how_many));
      writes.appendChild(aCell(["f", which, "mono"], one.format));
      if (at + 1 < said.what_it_will_write.by_format.length) {
        writes.appendChild(aCell(["p"], ""));
      }
    });
    output.appendChild(writes);
  }
  for (const line of said.the_output_lines) {
    const one = document.createElement("p");
    one.className = "hint flat";
    theSentence(one, line);
    output.appendChild(one);
  }
}

/// **One folder's row**: where it writes, then a count and its format per
/// line — a folder holding two formats is one row, its place spanning both
/// lines, so no folder cell is ever left blank and no count drifts from its
/// format. **Where it writes is its folder under the job's folder, in the
/// order its bytes run (X9)** — or *top of the job's folder* or *this
/// folder*, which are words. **A path breaks after a slash, and inside a
/// folder's name only where that name is wider than the column**: each name
/// is held whole with the slash after it (`thePathIsDrawnInto`).
function aRowIsDrawnInto(writes, row, under) {
  const classes = row.the_top ? ["p"] : ["p", "mono"];
  if (under) classes.push("in");
  const place = aCell(classes, row.the_top || "");
  if (!row.the_top) thePathIsDrawnInto(place, row.place);
  theCountsAreDrawnBeside(writes, place, row.counts);
}

/// **A folder's place, then its counts** — a count and its format per line.
/// **Where it holds more than one format, its counts sit at its top, each on
/// its own format's line** (the drawing's `.wcounts`, D-11 of the UI at every
/// size, 2026-11-12): one cell across the two columns, laid on their lines,
/// so a tall folder or a wrapped format moves no count off its format. Until
/// then the place spanned one line per format, and a folder that wrapped to
/// six lines drew its second count halfway down it.
function theCountsAreDrawnBeside(writes, place, counts) {
  writes.appendChild(place);
  let into = writes;
  if (counts.length > 1) {
    into = aCell(["wcounts"], "");
    writes.appendChild(into);
  }
  for (const one of counts) {
    into.appendChild(aCell(["n", "mono"], one.how_many));
    into.appendChild(aCell(["f", "mono"], one.format || ""));
  }
}

/// **A path drawn so it breaks only after a slash** — each folder's name and
/// the slash after it are one piece (the drawing's `.writes .p bdi bdi`).
function thePathIsDrawnInto(holder, path) {
  theValueIsDrawnInto(holder, path);
  const whole = holder.firstChild;
  if (!whole || !whole.childNodes) return;
  for (const name of Array.from(whole.childNodes)) {
    if (name.nodeType !== 1) continue;
    const after = name.nextSibling;
    if (after && after.nodeType === 3 && (after.textContent === "/" || after.textContent === "\\")) {
      name.appendChild(after);
    }
  }
}

/// **Folders with the same parent, under it** — `Design/Render Settings §
/// What it will write`, the Creator's twenty-eighth notes, note 17. The
/// parent's line carries the number of folders and the group's files by
/// format, and a press opens or closes it in place: the line stays where it
/// was, under the pointer, and what is below moves. **That is the one stated
/// exception to `Design/Whole app`'s rule that nothing drawn moves**, because
/// the move is the user's own press. **Open or closed is kept for the page's
/// life once drawn**: a redraw keeps it, and a change of range that takes a
/// group past eight never closes one already open. Another file is a fresh
/// page.
function theGroupIsDrawnInto(writes, server, row, key = "") {
  const held = thePanelFor(server);
  if (!held.theGroupsDrawn) held.theGroupsDrawn = new Map();
  const which = `${key}${row.place}`;
  if (!held.theGroupsDrawn.has(which)) held.theGroupsDrawn.set(which, !row.group.it_starts_closed);
  const open = held.theGroupsDrawn.get(which);
  const place = aCell(["p"], "");
  const line = document.createElement("button");
  line.type = "button";
  line.className = "wgrp";
  line.dataset.box = `output-group-${row.place}`;
  line.setAttribute("aria-expanded", String(open));
  const chevron = document.createElement("span");
  chevron.className = "chev";
  chevron.textContent = "▼";
  line.appendChild(chevron);
  const words = document.createElement("span");
  const parent = document.createElement("span");
  parent.className = "mono";
  thePathIsDrawnInto(parent, row.place);
  words.appendChild(parent);
  const folders = document.createElement("span");
  folders.className = "wfold";
  saidInto(folders, row.group.folders);
  words.appendChild(folders);
  line.appendChild(words);
  line.addEventListener("click", () => {
    held.theGroupsDrawn.set(which, !held.theGroupsDrawn.get(which));
    if (held.said) theSettingsAreDrawn(server, held.said);
  });
  place.appendChild(line);
  theCountsAreDrawnBeside(writes, place, row.counts);
  if (!open) return;
  for (const one of row.group.rows) aRowIsDrawnInto(writes, one, true);
}

function theDependencyBand(server, said) {
  const band = said.the_dependency_band;
  const held = document.createElement("div");
  held.className = "band";
  held.dataset.box = `dependencies-${band.which}`;
  const line = document.createElement("p");
  line.className = "bt";
  theSentence(line, band.said);
  held.appendChild(line);

  // **Three ways on, in the order of how much they cost**: check the assets
  // now, submit with the first-frame check, submit anyway.
  const ways = document.createElement("div");
  ways.className = "ways";
  const check = document.createElement("button");
  check.type = "button";
  check.className = "btn quiet";
  check.dataset.box = "band-check";
  check.textContent = band.the_way_to_the_check;
  // ***Check dependencies…* starts a check; *Open the check* goes to the rows
  // already there and starts nothing** (must-do 11). Which is the
  // application's.
  check.addEventListener("click", () =>
    band.the_way_starts_a_check ? theCheckIsOpened(server) : theCheckIsShown(server)
  );
  ways.appendChild(check);
  if (band.render_one_frame_first) {
    const first = document.createElement("button");
    first.type = "button";
    first.className = "mini";
    first.dataset.box = "band-first-frame";
    first.textContent = band.render_one_frame_first;
    // **Sets the first-frame check and submits, as one act** — `Rendering` W4's
    // `SubmitWithTheFirstFrameCheck` (must-do 10).
    first.addEventListener("click", () => aJobIsSubmitted(server, said, true));
    // **No scene ticked: both ways to submit grey with Render**; *Check
    // dependencies…* stays, being about the file (`§ Scenes`, *None ticked*).
    first.disabled = Boolean(said.no_scene_is_ticked);
    ways.appendChild(first);
  }
  const anyway = document.createElement("button");
  anyway.type = "button";
  anyway.className = "mini";
  anyway.dataset.box = "band-anyway";
  anyway.textContent = band.submit_anyway;
  anyway.addEventListener("click", () => aJobIsSubmitted(server, said));
  anyway.disabled = Boolean(said.no_scene_is_ticked);
  ways.appendChild(anyway);
  held.appendChild(ways);

  if (band.what_one_frame_first_buys) {
    const note = document.createElement("p");
    note.className = "wnote";
    note.textContent = band.what_one_frame_first_buys;
    held.appendChild(note);
  }
  // **Switching it off is on the band, and it says what it does not cover.**
  const off = document.createElement("label");
  off.className = "off";
  off.dataset.box = "band-switch-off";
  // **A tick, ticked by Space as by a click, its ring on the tick** (`Design/
  // Whole app`, *Tab lands on the control itself*; the Creator's
  // twenty-seventh notes, note 1). Ticked, the band goes, so it is drawn
  // unticked.
  off.setAttribute("role", "checkbox");
  off.setAttribute("aria-checked", "false");
  off.tabIndex = 0;
  const mark = document.createElement("span");
  mark.className = "cb";
  off.appendChild(mark);
  off.appendChild(document.createTextNode(band.switch_it_off));
  const switchedOff = () =>
    shell.core.invoke("the_missing_assets_warning_is_switched_off", {
      server,
      file: said.file_bytes,
    });
  off.addEventListener("click", switchedOff);
  off.addEventListener("keydown", (event) => {
    if (event.key !== " " && event.key !== "Enter") return;
    event.preventDefault();
    switchedOff();
  });
  held.appendChild(off);
  // **Where the switch-off could not be read, the band says so** rather than
  // drawing itself as though the answer were known (F-10).
  if (band.the_switch_could_not_be_read) {
    const unread = document.createElement("p");
    unread.className = "wnote";
    unread.textContent = band.the_switch_could_not_be_read;
    held.appendChild(unread);
  }
  // **A switch-off the record would not take is said here** (must-do 2).
  held.appendChild(anAnswerLine("dependencies"));
  const covers = document.createElement("p");
  covers.className = "wnote";
  covers.textContent = band.what_the_switch_covers;
  held.appendChild(covers);
  return held;
}

// ── The foot ────────────────────────────────────────────────────────────────
//
// **The foot is two lines, every group aligned to the top** — `Design/Render
// Settings § How the three sit together`, the Creator's tenth notes, note 8,
// drawn as `res-foot` and approved on 2026-10-17. **One grid**, the drawing's
// `.sfoot`, so the name, the switches and Render share their lines instead of
// each centring itself in a tall row:
//
//   line 1 — *Name this job* and its field · the download switch with its
//            summary beside it · Render;
//   line 2 — the name's hint · *Render one frame first*, then *Telegram* and
//            its dropdown · what the submission says, under Render;
//   line 3 — only where something is wrong or under way: the note that the
//            formats wait for a read, no folder, uploading dependencies.
//
// **Where each part sits is the drawing's grid, not the order drawn here**, so
// the order is the keyboard's: line 1, then Render, then line 2 — the keyboard
// reaches Render before *Show in Queue*. At the 1,180 step the switches drop
// under the name (the drawing's rules under `s1`).

function theFoot(server, said) {
  const foot = document.createElement("div");
  foot.className = "sfoot";

  // **The name's field.** The drawing lays its three parts straight into the
  // foot's grid (`.sfoot .field { display: contents }`): the label and the
  // field on line 1, the hint under the field on line 2.
  const field = document.createElement("div");
  field.className = "field";
  const label = document.createElement("label");
  label.textContent = said.name_this_job;
  label.htmlFor = "the-jobs-name";
  field.appendChild(label);
  const name = document.createElement("input");
  name.type = "text";
  name.id = "the-jobs-name";
  name.className = "fake";
  name.dataset.box = "job-name";
  name.value = said.the_job_name;
  name.addEventListener("change", () =>
    anAnswerIsGiven(server, said, "job-name", name.value)
  );
  field.appendChild(name);
  const note = document.createElement("span");
  note.className = "fnote";
  note.textContent = said.where_the_name_shows;
  field.appendChild(note);
  foot.appendChild(field);

  // ── Line 1: the download switch, with its summary control beside it ──
  //
  // **Downloading as it renders is three decisions, not a checkbox**: the
  // switch is in the open, and where the files go and which formats are one
  // control on the switch's own line, carrying the summary and opening the
  // panel. With nothing read the same control says where the files go alone,
  // because that one is still chosen. **The line takes the room left over and
  // no more**, so a long summary ends in an ellipsis rather than pushing Render
  // off its line (the drawing's `.sfoot .sfl1 .mini`). The panel is piece 12's
  // box, drawn for this file.
  const first = document.createElement("div");
  first.className = "so sfl1";
  first.appendChild(
    aSwitch(said.download_as_they_render, said.the_download_is_on, () =>
      anAnswerIsGiven(
        server,
        said,
        "download-as-they-render",
        said.the_download_is_on ? "off" : "on"
      ),
      "download-switch",
      // **No scene ticked, no job** — the switch greys with Render
      // (`§ Scenes`, *None ticked*), keeping its setting.
      said.no_scene_is_ticked
    )
  );
  const summary = said.the_format_summary || said.where_they_go;
  if (summary) {
    const control = document.createElement("button");
    control.type = "button";
    control.className = "mini";
    control.dataset.box = "download-choices";
    theSentence(control, summary);
    // Cut short beside the switch, so it reads whole on hover (the UI at
    // every size, D-04).
    control.title = control.textContent;
    // The drawing's own mark that the control opens something. A glyph, not a
    // word.
    control.appendChild(document.createTextNode(" ▾"));
    control.addEventListener("click", () =>
      theFormatBoxIsOpenedForAFile(
        server,
        said.file_bytes,
        (chosen) =>
          shell.core.invoke("the_download_choices_are_made", {
            server,
            file: said.file_bytes,
            formats: chosen.formats,
            folder: chosen.folder,
          }),
        () => theFileIsRead(server, said),
        // **Choosing *Somewhere else…* puts the system's folder panel up at
        // once** — `Design/Render Settings`, added 2026-09-30 from the
        // Creator's press on step 10c's eleventh pass (fault 11). The
        // application holds the folder for the box and answers the box again
        // with it offered first; the page never names a folder itself (H34).
        () =>
          shell.core.invoke("the_folder_is_asked_for_in_the_box", {
            server,
            file: said.file_bytes,
          })
      )
    );
    first.appendChild(control);
  }
  foot.appendChild(first);

  // ── Render, at the right of line 1 ──
  //
  // **The Render control names how many frames**, and where nothing is read it
  // names the act alone.
  const render = document.createElement("button");
  render.type = "button";
  render.className = "btn";
  render.dataset.box = "render";
  theSentence(render, said.the_render_control);
  if (said.the_submission_is_underway) {
    // **Design's *on the control*: the control the user pressed is the thing
    // that is working**, so it keeps its place and its width, loses its label
    // to the turning mark, and **is not dimmed** — so it is not `disabled`
    // either, which the foot's own rule greys. It answers nothing while it
    // works. Added 2026-09-27, eighth-pass fix piece 5: the submission took
    // about twelve seconds on the eighth pass and *Render* only went grey.
    theMarkOnAControl(render, render.textContent);
  } else if (!said.submitting_is_offered) {
    render.disabled = true;
    // **Why it waits, on hover** — `Design/Whole app § While other work runs`
    // (the Creator's notes on the test copy, second round, note 17).
    if (said.why_render_waits) render.title = said.why_render_waits;
  }
  render.addEventListener("click", () => {
    if (render.getAttribute("aria-disabled") === "true") return;
    aJobIsSubmitted(server, said);
  });
  foot.appendChild(render);

  // ── Line 2: Render one frame first, Telegram, and what the submission says ──
  const second = document.createElement("div");
  second.className = "so sfl2";
  // **Where every job the Render control would send has one frame the switch
  // is greyed and cannot be pressed, but keeps its setting** — which, is the
  // application's (`Design/Render Settings § Submitting`, the Creator's
  // nineteenth notes, note 8).
  second.appendChild(
    aSwitch(
      said.render_one_frame_first,
      said.the_first_frame_is_asked_for && !said.the_first_frame_switch_is_greyed,
      () =>
        anAnswerIsGiven(
          server,
          said,
          "the-first-frame-check",
          said.the_first_frame_is_asked_for ? "off" : "on"
        ),
      "first-frame-switch",
      said.the_first_frame_switch_is_greyed || said.no_scene_is_ticked
    )
  );
  // **How much the job reports to the phone is chosen here too**, beside the
  // download choices, and it is a quiet line rather than a question. **Where
  // Telegram is switched off it is not there at all** — the application says
  // which, from the record.
  if (said.the_telegram_line_is_drawn) second.appendChild(theTelegramLine(server, said));

  // **What the submission says sits under Render, right-aligned in Render's
  // column** (`Design/Render Settings § Submitting`, from 2026-10-17): a
  // submission that went (`theSubmissionThatWentIsDrawn`, the drawing's
  // `.rs-sent`), or what the submission answered — *Submitting.*, a refusal —
  // never both. **It shares line 2 with the switches and never widens
  // Render's column**, so nothing already drawn moves when it appears, which
  // is what the Creator's second notes, note 8, asked of it (Render jumped
  // when pressed). Where line 2 has no room for it, its sentence is cut short
  // rather than dropping under (the UI at every size, D-16 and C9).
  const held = panels.get(server);
  if (held && held.theSubmissionThatWent) {
    const went = theSubmissionThatWentIsDrawn(held.theSubmissionThatWent);
    went.dataset.box = "submitted";
    second.appendChild(went);
  }
  second.appendChild(anAnswerLine("submit"));
  foot.appendChild(second);

  // Design/Render Settings, Prepare simulations; `Render Settings` Q2b, `Render Settings` Y1c.
  for (const row of said.simulation_rows || []) {
    const line = document.createElement("div");
    line.className = "so";
    line.dataset.box = "simulation-row";
    if (row.name) {
      // **A long scene name is cut short, whole on hover, so the switch and the
      // range stay beside it and in line with the other scenes'** (the UI at
      // every size, D-13).
      const scene = document.createElement("span");
      scene.className = "soname";
      scene.textContent = row.name;
      scene.title = row.name;
      line.appendChild(scene);
    }
    line.appendChild(aSwitch(row.label, row.on, () => anAnswerIsGiven(
      server, said, "prepare-simulations", (row.on ? "off:" : "on:") + row.scene
    ), "prepare-simulations"));
    const range = document.createElement("span");
    range.textContent = row.range;
    line.appendChild(range);
    second.appendChild(line);
  }

  // ── Line 3: the notes, refusals and work under way ──
  const third = [];
  if (said.the_telegram_install_failed) {
    const failed = document.createElement("p");
    failed.className = "alertnote";
    failed.dataset.box = "telegram-install-failed";
    failed.textContent = said.the_telegram_install_failed;
    third.push(failed);
  }
  if (said.the_first_frame_is_answered_in_queue) {
    const note = document.createElement("p");
    note.className = "bar-note";
    note.dataset.box = "first-frame-in-queue";
    note.textContent = said.the_first_frame_is_answered_in_queue;
    third.push(note);
  }
  // **With nothing read, the format list is not offered, and one line says
  // why.**
  if (said.the_format_list_is_not_offered) {
    const why = document.createElement("p");
    why.className = "bar-note";
    why.textContent = said.the_format_list_is_not_offered;
    third.push(why);
  }
  // **The switch is on and Smoozy can name no folder** — the drawing's
  // `res-nofolder`. **Amber, under the switches**, because it asks the user to
  // do something, which is the same rule the two output warnings follow. It
  // blocks Render no more than they do: the job still goes, and this says what
  // that costs.
  if (said.the_frames_have_nowhere_to_go) {
    const nowhere = document.createElement("p");
    nowhere.className = "adwarn";
    nowhere.dataset.box = "no-folder";
    const lead = document.createElement("b");
    lead.textContent = said.the_frames_have_nowhere_to_go.lead;
    nowhere.appendChild(lead);
    nowhere.appendChild(document.createTextNode(" "));
    const rest = document.createElement("span");
    rest.textContent = said.the_frames_have_nowhere_to_go.rest;
    nowhere.appendChild(rest);
    third.push(nowhere);
  }
  // **While the check's uploads for this file on this server run, one line
  // sits under Render** — *Uploading dependencies: 4 of 12 files.* — a link
  // that opens the check on the rows already there, and **it says what holds
  // Render** (`Dependency Upload & Remap` A9e). The Creator's note 6 on 0.1.8;
  // on the third line since their tenth notes, note 8.
  // **Minutes Smoozy will not take are said on the third line**, in the
  // refusal colour, starting at the line's left (the drawing's `res-tgbad`,
  // `Design/Render Settings § Submitting`, 2026-10-19). The line is there
  // whenever the minutes field is, and hidden until the answer to a typed
  // entry says something (`theRefusalIsPutOnItsBox`).
  if (said.the_telegram_line_is_drawn && said.the_minutes_between_pictures !== null && said.the_minutes_between_pictures !== undefined) {
    third.push(anAnswerLine("telegram-minutes"));
  }
  if (said.the_dependencies_uploading) {
    const line = document.createElement("p");
    line.className = "adwarn sfdep";
    const link = aLink("", () =>
      said.the_dependency_link_starts_a_check ? theCheckIsOpened(server) : theCheckIsShown(server),
      "dependencies-uploading");
    theSentence(link, said.the_dependencies_uploading);
    line.appendChild(link);
    third.push(line);
  }
  // **The third line keeps its room when it has nothing to say** (the
  // drawing's `.sfoot .sfl3`), so its words coming and going never move
  // Render (UI consistency, finding 14).
  const line = document.createElement("div");
  line.className = "sfl3";
  third.forEach((one) => line.appendChild(one));
  foot.appendChild(line);
  return foot;
}

/// The quiet line: the word *Telegram* and one of the ~~three~~ four choices.
/// **The dropdown is only as wide as its longest choice**, by the drawing's own
/// class (`.w-tg`) — never a width written on the element, which this
/// application refuses (H33).
///
/// **The fourth, *A picture every few minutes*, is finished by a field**
/// (`Design/Render Settings § Submitting`, the Creator's want of 2026-10-19:
/// *"a choice that the user can manually type in the number of minutes"*).
/// Picked, the control reads what the application says it reads (*A picture
/// every*), and the minutes field and *minutes* follow it on the same line —
/// the drawing's `res-tgmin`. The control keeps its class, so its width does
/// not change and nothing beside it moves. **The field is a plain number field
/// in ordinary ink**, the drawing's `.fld.w-frame.mono` with no `.set` around
/// it, so the value is neither the file's italic nor the user's bold. A typed
/// entry goes to the application, which takes it or refuses it; a refused one
/// turns the field red and stays in it (`theRefusalIsPutOnItsBox`).
function theTelegramLine(server, said) {
  const telegram = document.createElement("span");
  telegram.className = "sftg";
  const named = document.createElement("span");
  named.className = "hintl";
  named.textContent = said.telegram;
  telegram.appendChild(named);
  // **`Server Setup` G2a: where the server has no role, the line says so in
  // place of the choices, with the action there**, in the control's own width.
  if (said.the_telegram_is_not_installed) {
    const here = document.createElement("span");
    here.className = "sftg w-tg";
    here.dataset.box = "telegram-not-installed";
    const words = document.createElement("span");
    words.className = "hintl";
    words.textContent = said.the_telegram_is_not_installed;
    here.appendChild(words);
    if (said.install_telegram) {
      const install = aLink(
        said.install_telegram,
        async () => {
          await shell.core
            .invoke("a_set_up_act_that_needs_a_connection", { act: INSTALL_TELEGRAM, server })
            .catch(() => {});
          theSettingsAreAskedFor(server);
        },
        "install-telegram"
      );
      install.title = said.install_telegram_says;
      here.appendChild(install);
    }
    telegram.appendChild(here);
    return telegram;
  }
  const rows = said.the_reporting_choices.map((choice) => ({
    value: choice.value,
    said: choice.said,
    picked: choice.value === said.the_reporting_choice,
  }));
  // **What the closed control reads is the application's**: the row picked,
  // or *A picture every* for the fourth. The picked row's own words stand in
  // only where an answer carries no such field.
  const picked = rows.find((one) => one.picked);
  const shown = said.the_reporting_control_reads || (picked ? picked.said : "");
  telegram.appendChild(
    aDropdown({ label: said.telegram, box: "reports", width: "w-tg", rows, shown }, (value) =>
      anAnswerIsGiven(server, said, "what-it-reports", value)
    )
  );
  const minutes = said.the_minutes_between_pictures;
  if (minutes !== null && minutes !== undefined) {
    const field = theField({ value: minutes, whose: "the-users" }, "w-frame", (typed) =>
      anAnswerIsGiven(server, said, "telegram-minutes", typed)
    );
    field.dataset.box = "telegram-minutes-field";
    field.setAttribute("aria-label", said.minutes_between_pictures);
    telegram.appendChild(field);
    const after = document.createElement("span");
    after.className = "hintl";
    after.textContent = said.minutes;
    telegram.appendChild(after);
  }
  return telegram;
}

// ── Smoozy's own dropdown ───────────────────────────────────────────────────
//
// **`Design/Whole app § The window`: *"Smoozy draws its own dropdowns."*** The
// Creator's fifth notes, note 2: the system's own list *"looks like it doesn't
// belong in this design"*. Until 2026-10-08 the page's three dropdowns — the
// Blender version, Texture Limit and what Telegram reports — were `select`s,
// and the list that opened was the operating system's.
//
// **The list is the right-click menu's own panel**: the drawing's `.ctx` and
// `.pick .ddl`, the drawing's `res-verlist`, with the picked row in the menu's
// bold (`.ctx button.hd`) and every row keeping its words. Up and Down move,
// Return picks, and **Escape closes the list before anything else** — the
// press is marked handled, and the panel's own Escape steps aside for a handled
// press. A press anywhere else closes it.
//
// **A pick closes the list and leaves the control as it looked before it was
// pressed**: a pick by mouse leaves the focus nowhere in it, so no ring is
// drawn and the redraw has nothing to give back; a pick by keyboard puts the
// focus back on the control, the redraw gives it back by its `data-box`, and
// the ring shows, as it does for the keyboard everywhere else.
let theOpenDropdown = null;

document.addEventListener(
  "pointerdown",
  (press) => {
    if (theOpenDropdown && !theOpenDropdown.pick.contains(press.target)) theOpenDropdown.shut();
  },
  true
);

/// **A list with no room below it opens upward** — the Creator's thirteenth
/// notes, note 1: *What it reports* sits in the foot, and its list ran past the
/// window's bottom. Measured each time it opens, below first: **where it would
/// pass the window's bottom and there is more room above the control than
/// under it, it carries `up`** (`.pick .ddl.up`, the drawing's own since
/// 2026-10-19, `web/the-drawing.css`),
/// the rule that keeps the right-click menu inside the window. **This is the
/// one place the page measures against the window**; the panel itself never
/// does.
///
/// **The bottom that cuts the list off is the nearest one** — the window's, or
/// that of any box around the control that clips what it holds, such as the
/// panel's scrolling sheet, which ends above the strips along the bottom (the
/// Creator's eighteenth notes, note 11, found on Queue's View Layer list,
/// which measured the same way). Until then a list at the sheet's foot with
/// room left in the window under it opened downward and was cut off.
///
/// **The top and bottom it can be seen between** are the window's, narrowed by
/// every box around the control whose overflow is not visible — the same
/// reading as `web/the-queue.js`'s, kept inside this one function so the page
/// still measures nowhere else.
function theListIsKeptInsideTheWindow(face, list) {
  list.classList.remove("up");
  list.classList.remove("left");
  list.style.setProperty("--ddl-max-h", "none");
  list.style.setProperty("--ddl-max-w", "none");
  const room = list.getBoundingClientRect();
  const control = face.getBoundingClientRect();
  let top = 0;
  let bottom = window.innerHeight;
  let left = 0;
  let right = window.innerWidth;
  const styleOf = typeof globalThis.getComputedStyle === "function" ? globalThis.getComputedStyle : null;
  for (let at = face.parentNode; styleOf && at && at.nodeType === 1; at = at.parentNode) {
    const style = styleOf(at);
    if (style.overflowY === "visible" && style.overflowX === "visible") continue;
    const box = at.getBoundingClientRect();
    top = Math.max(top, box.top);
    bottom = Math.min(bottom, box.bottom);
    left = Math.max(left, box.left);
    right = Math.min(right, box.right);
  }
  // **Held to the room it has** (the UI at every size, C1): leftward from the
  // control's right edge where its names are wider than the room right of it,
  // a name still too long wrapping; and no taller than the side it opens to,
  // scrolling inside it. Two values the stylesheet reads (`--ddl-max-w`,
  // `--ddl-max-h`), never a declaration on the element (H33). The gap is the
  // list's own 4 pixels, twice.
  const rightward = right - control.left - 8;
  const leftward = control.right - left - 8;
  const leftwards = room.width > rightward && leftward > rightward;
  if (leftwards) list.classList.add("left");
  list.style.setProperty("--ddl-max-w", `${Math.round(Math.max(leftwards ? leftward : rightward, control.width))}px`);
  const tall = list.getBoundingClientRect().height;
  const below = bottom - control.bottom - 8;
  const above = control.top - top - 8;
  const up = tall > below && above > below;
  if (up) list.classList.add("up");
  if (tall > (up ? above : below)) list.style.setProperty("--ddl-max-h", `${Math.round(Math.max(up ? above : below, 60))}px`);
}

/// `rows` is `[{ value, said, picked }]`; `shown` is what the closed control
/// says; `picked(value)` is called for a row that was not already the pick.
function aDropdown({ label, box, width, rows, shown }, picked) {
  const pick = document.createElement("span");
  pick.className = "pick";
  if (width) pick.classList.add(width);
  const face = document.createElement("button");
  face.type = "button";
  face.className = "pkb";
  face.setAttribute("aria-label", label);
  face.setAttribute("aria-haspopup", "listbox");
  face.setAttribute("aria-expanded", "false");
  face.dataset.box = box;
  face.textContent = shown;
  pick.appendChild(face);

  const list = document.createElement("span");
  list.className = "ctx ddl";
  list.setAttribute("role", "listbox");
  list.setAttribute("aria-label", label);
  list.hidden = true;
  const options = rows.map((row, at) => {
    const one = document.createElement("button");
    one.type = "button";
    one.setAttribute("role", "option");
    one.setAttribute("aria-selected", row.picked ? "true" : "false");
    if (row.picked) one.className = "hd";
    one.textContent = row.said;
    // **A click a key made has no count** — Return and Space on a button.
    one.addEventListener("click", (press) => chosen(at, press.detail === 0));
    list.appendChild(one);
    return one;
  });
  pick.appendChild(list);
  const first = Math.max(0, rows.findIndex((row) => row.picked));

  const theFocusLeaves = () => {
    const had = document.activeElement;
    if (had && pick.contains(had) && typeof had.blur === "function") had.blur();
  };
  const shut = () => {
    list.hidden = true;
    face.setAttribute("aria-expanded", "false");
    if (theOpenDropdown && theOpenDropdown.pick === pick) theOpenDropdown = null;
  };
  const opened = (byKey) => {
    if (theOpenDropdown) theOpenDropdown.shut();
    list.hidden = false;
    theListIsKeptInsideTheWindow(face, list);
    face.setAttribute("aria-expanded", "true");
    theOpenDropdown = { pick, shut: () => { shut(); theFocusLeaves(); } };
    // **The keys reach the list however it was opened**: from the keyboard
    // the picked row takes the focus, from the mouse the control does.
    if (byKey && options[first]) theFocusMovesTo(options[first]);
    else theFocusMovesTo(face);
  };
  function chosen(at, byKey) {
    shut();
    if (byKey) theFocusMovesTo(face);
    else theFocusLeaves();
    if (!rows[at].picked) picked(rows[at].value);
  }

  face.addEventListener("click", (press) => {
    if (list.hidden) opened(press.detail === 0);
    else {
      shut();
      theFocusLeaves();
    }
  });
  pick.addEventListener("keydown", (press) => {
    const open = !list.hidden;
    if (press.key === "Escape" && open) {
      press.preventDefault();
      shut();
      theFocusMovesTo(face);
    } else if (press.key === "ArrowDown" || press.key === "ArrowUp") {
      press.preventDefault();
      if (!open) return opened(true);
      const at = options.indexOf(document.activeElement);
      const next = at < 0 ? first : at + (press.key === "ArrowDown" ? 1 : -1);
      if (options[next]) theFocusMovesTo(options[next]);
    } else if (press.key === "Enter" && open) {
      const at = options.indexOf(document.activeElement);
      if (at < 0) return;
      press.preventDefault();
      chosen(at, true);
    } else if (press.key === "Tab" && open) {
      shut();
    }
  });
  return pick;
}

// ════════════════════════════════════════════════════════════════════════════
// The acts
// ════════════════════════════════════════════════════════════════════════════

// **What the user last typed, so a refused value stays in the box.** F1
// refuses in place: the typed value never becomes a store entry and the entry
// that stood goes on standing — **but the drawing turns the field red with the
// refused number still in it**, because a box that silently reverted would take
// away the thing the user is being asked to correct.
let theLastTyped = null;

function anAnswerIsGiven(server, said, which, text) {
  theLastTyped = { which, text: String(text) };
  return shell.core.invoke("an_answer_was_given", {
    server,
    file: said.file_bytes,
    which,
    text: String(text),
  });
}

function aValueIsHandedBack(server, said, which) {
  return shell.core.invoke("a_value_is_handed_back", {
    server,
    file: said.file_bytes,
    which,
  });
}

function theFileIsRead(server, said) {
  return shell.core.invoke("the_file_is_read", { server, file: said.file_bytes });
}

function aJobIsSubmitted(server, said, withTheFirstFrameCheck) {
  // `Queue & History` Y8: take the field with the press. Its change request
  // can still be in flight, and a redraw can have kept uncommitted typing.
  const held = panels.get(server);
  const name = held && held.body.querySelector('[data-box="job-name"]');
  return shell.core.invoke("a_job_is_submitted", {
    server,
    file: said.file_bytes,
    jobName: name ? name.value : said.the_job_name,
    withTheFirstFrameCheck: Boolean(withTheFirstFrameCheck),
  });
}

/// ***The check is reached from the settings*** — `Design/File Browser § Where
/// it sits`, and it is the route piece 9's panel is opened by. **The check is
/// opened on the file these settings are of**, because a check is of a file.
function theCheckIsOpened(server) {
  const held = panels.get(server);
  theCheckIsOpenedOn(server, held ? held.file : []).catch(() => {});
}

/// ***Open the check*: the rows already there, and no new check** (must-do 11).
function theCheckIsShown(server) {
  const held = panels.get(server);
  theCheckIsShownOn(server, held ? held.file : []).catch(() => {});
}

/// ***Set it up* goes to the Set up tab** (must-do 4 of the step 10a round 1
/// audit of these pieces) — Design's `§ When there is no version to pick`.
/// **The panel folds down first, through its own collapse
/// control**, so what the user typed stays exactly where it was (piece 5's
/// section 4), and then the server's Set up tab is shown.
function theSetUpTabIsShown(server) {
  const held = panels.get(server);
  const frame = held && held.head && held.head.closest(".sheetwrap");
  const fold = frame && frame.querySelector(".collapse");
  if (fold) fold.click();
  theCurrentTabIs(SET_UP);
}

/// **The file's name in the title is a link into Files** — `Design/Whole app §
/// Panels over a server`, added 2026-10-12 (the Creator's ninth notes, note
/// 5), and the drawing's `.golink` inside the `h2`, hover *Show in Files*.
/// Pressing it collapses the panel as its chevron does and shows Files on the
/// file's folder with the file chosen. **The folder and the name are the
/// application's**, sent with the panel; this page composes no path. The
/// dependency check's title does the same.
function theNameShowsTheFileInFiles(server, link, where) {
  link.type = "button";
  link.className = "golink";
  link.title = where.hover;
  link.addEventListener("click", () => {
    thePanelCollapsesAsItsChevronDoes(server);
    theBrowserIsOpenedAt(server, where.folder, where.name);
    theCurrentTabIs(FILES);
  });
}

/// **The refusal goes on the box the user typed into.** F1's shape: the typed
/// value never became a store entry, the entry that stood goes on standing, and
/// the line says the range and what Blender would otherwise have done.
///
/// **An act that is not a typed value — the read, the submission, the band's
/// switch-off — answers on its own line beside its control** rather than on
/// the field refusal line.
function theRefusalIsPutOnItsBox(server, said) {
  const held = panels.get(server);
  if (!held || !held.body) return;
  const parts = thePartsOf(said);
  const own = [held.head, held.body]
    .map((part) => part.querySelector(`[data-box="${CSS.escape(`${said.theBox}-answer`)}"]`))
    .find(Boolean);
  // **The confirmation is the foot's to draw** (`theSubmissionThatWentIsDrawn`),
  // so the submission's own line stays empty.
  if (own && said[THE_JOB_TO_SHOW]) {
    own.replaceChildren();
    own.hidden = true;
    return;
  }
  if (own) {
    own.replaceChildren();
    // **The answer's kind decides its class, part by part.** *Submitted.*
    // and *2 of 3 jobs submitted.* say what went out and are the plain note;
    // *Not submitted.* and every reason keep the refusal colour. The tenth
    // pass saw *Submitted.* in dark red, which reads as a failure. Step 10c's
    // tenth-pass fix piece 6.
    // **The plain note is the drawing's own on this page**, `.bar-note` in
    // the value column's `.underval` inset — the same 91 pixels `.alertnote`
    // carries, so the line keeps its place whichever it is. `Design values`
    // gives `--alert` to *"Refused or blocked"* and nothing else.
    const plain = Math.min(Number(said[THE_PLAIN_PARTS] || 0), parts.length);
    own.className = parts.length > 0 && plain === parts.length ? "bar-note underval" : "alertnote";
    if (plain > 0 && plain < parts.length) {
      const went = document.createElement("span");
      went.className = "bar-note";
      theSentence(went, parts.slice(0, plain));
      own.appendChild(went);
      own.appendChild(document.createTextNode(" "));
      theSentence(own, parts.slice(plain));
    } else {
      theSentence(own, parts);
    }
    own.hidden = parts.length === 0;
    // **The submission's answer is cut short on the foot's second line, and
    // reads whole on hover**, as the submitted line is (UI consistency,
    // finding 47).
    if (said.theBox === "submit") own.title = own.textContent;
    // **A typed value whose refusal has a line of its own** — the minutes
    // between pictures, refused on the foot's third line: the field turns red
    // and keeps what was typed, as every refused field on this page does. The
    // foot's field is not in a `.set` row, so it carries `.bad` itself (the
    // drawing's `.sftg .fld.bad`).
    const typed = held.body.querySelector(`[data-box="${CSS.escape(`${said.theBox}-field`)}"]`);
    if (typed) {
      if (parts.length) {
        typed.classList.add("bad");
        if (theLastTyped && theLastTyped.which === said.theBox && said.theBox !== "range") typed.value = theLastTyped.text;
      }
      theLastTyped = null;
    }
    return;
  }
  const line = held.body.querySelector('[data-box="refusal"]');
  if (line) {
    line.replaceChildren();
    theSentence(line, parts);
    line.hidden = parts.length === 0;
  }
  if (!parts.length) {
    theLastTyped = null;
    return;
  }
  const box = held.body.querySelector(`[data-box="${CSS.escape(said.theBox)}"]`);
  if (box) box.classList.add("bad");
  const field = held.body.querySelector(`[data-box="${CSS.escape(`${said.theBox}-field`)}"]`);
  // `Render Settings` F1: a refused range keeps the last accepted text
  // from the panel, just as its parsed entry stays on the last good range.
  if (field && theLastTyped && said.theBox !== "range") field.value = theLastTyped.text;
  theLastTyped = null;
}

/// **A sentence off the event, in the parts the application composed.** The
/// keys are declared beside the composer in `src/surface/render_settings.rs`.
function thePartsOf(said) {
  const parts = [];
  const many = Number(said.partCount || 0);
  for (let at = 0; at < many; at += 1) {
    const values = [];
    for (let which = 0; which < 3; which += 1) {
      const value = said[`value${at}${which}`];
      if (value !== undefined) values.push(value);
    }
    parts.push({
      lead: said[`lead${at}`] || "",
      values,
      rest: said[`rest${at}`],
      the_rest_joins: Boolean(said[`joins${at}`]),
      the_rest_binds: Boolean(said[`binds${at}`]),
    });
  }
  return parts;
}

// ════════════════════════════════════════════════════════════════════════════
// The small pieces every column uses
// ════════════════════════════════════════════════════════════════════════════

// **What stands between a figure and the words after it, where the application
// marks them bound** — U+00A0, so *24 GB* or *2 jobs* never parts across two
// lines. `Design/Whole app § Typography`; the character's one home is
// `src/typography.rs` (`BETWEEN_A_FIGURE_AND_ITS_UNIT`), and this page writes
// it only where the mark says, never by reading which word is a unit.
const BETWEEN_A_FIGURE_AND_ITS_UNIT = "\u00A0";

/// **Draw a piece of prose with the values that belong beside it.** The
/// application hands over `{ lead, values, rest }` and **this sets them side by
/// side and joins nothing**, which is the page half of X7.
function saidInto(holder, part) {
  if (!part) return holder;
  let first = true;
  let values = 0;
  const next = (node, between) => {
    if (!first) holder.appendChild(document.createTextNode(between || " "));
    first = false;
    holder.appendChild(node);
  };
  if (part.lead) {
    const lead = document.createElement("span");
    lead.textContent = part.lead;
    next(lead);
  }
  for (const value of part.values || []) {
    const one = document.createElement("b");
    // A value beside the words — a path, a name, a figure — in the order its
    // bytes run (X9, `web/the-value-drawn.js`).
    theValueIsDrawnInto(one, value);
    // **Several values in one part are a list, and a list is separated by
    // punctuation rather than by a word.** Putting a conjunction there would be
    // the page composing prose, which A6 and X1 forbid outright.
    next(one, values > 0 ? ", " : " ");
    values += 1;
  }
  if (part.rest) {
    const rest = document.createElement("span");
    rest.textContent = part.rest;
    if (part.the_rest_joins) {
      first = true;
      holder.appendChild(rest);
    } else if (part.the_rest_binds) {
      holder.appendChild(document.createTextNode(BETWEEN_A_FIGURE_AND_ITS_UNIT));
      holder.appendChild(rest);
    } else {
      next(rest);
    }
  }
  return holder;
}

/// **A whole sentence, as its parts.** X10: each part's words are their own
/// string through the typography point, so the parts are set side by side here
/// and never joined into one string on the application side.
function theSentence(holder, parts) {
  let first = true;
  for (const part of parts || []) {
    if (!first) holder.appendChild(document.createTextNode(" "));
    first = false;
    saidInto(holder, part);
  }
  return holder;
}

/// The plain text of a sentence, for a place that takes one — a tooltip.
function theWordsOf(part) {
  const held = document.createElement("span");
  saidInto(held, part);
  return held.textContent;
}

function aGroup(heading, qualifier) {
  const group = document.createElement("div");
  group.className = "grp";
  const title = document.createElement("h3");
  title.textContent = heading;
  // **Two of the headings carry a qualifier saying which reading of the thing
  // is on screen.**
  if (qualifier) {
    const said = document.createElement("span");
    said.className = "lit";
    saidInto(said, qualifier);
    title.appendChild(document.createTextNode(" "));
    title.appendChild(said);
  }
  group.appendChild(title);
  return group;
}

/// **The convention is the whole-app Design's**: italic grey is the file's,
/// upright bold is the user's, with a cross beside it.
function aSetting(field) {
  const set = document.createElement("div");
  set.className =
    field.whose === "the-users" ? "set typed" : "set fromfile";
  if (field.refused) set.classList.add("bad");
  return set;
}

function theLabel(said) {
  const label = document.createElement("span");
  label.className = "k";
  label.textContent = said;
  return label;
}

/// **Fields are the width of what they hold**, so the cross sits immediately
/// beside the value rather than a mouse-drag away.
///
/// **A field with no value still takes typing** — `§ Before the file has been
/// read`: *"Everything the user can answer without a read, they can answer."*
/// Where the value is the file's and not yet read, **the em dash is its
/// placeholder in the file's own italic face**, shown only while the field is
/// empty — the third of `Render Settings` J1's three faces. **The range field
/// with several scenes ticked is live and empty and shows nothing.** Only a
/// greyed control is not a field at all (must-do 1 of the step 10a round 1 audit
/// of these pieces; until then an empty field was a read-only dash, so nothing
/// could be typed before a read).
function theField(field, width, answered) {
  if (field.greyed) {
    // **A greyed control shows no value**: it keeps its place and its label,
    // and where its value sat there is nothing.
    const held = document.createElement("span");
    held.className = `fld ${width} mono`;
    return held;
  }
  const box = document.createElement("input");
  box.type = "text";
  box.className = `fld ${width} mono`;
  if (field.value === null || field.value === undefined) {
    box.value = "";
    if (field.whose === "the-promise-form") {
      box.placeholder = "—";
      box.classList.add("dashv");
      box.addEventListener("input", () => {
        box.classList.toggle("dashv", box.value === "");
      });
    }
  } else {
    box.value = field.value;
  }
  // **A click into a value box chooses its whole text**, so what is typed
  // replaces it; a second click places the caret — `Design/Render Settings`,
  // beside the value boxes, the Creator's fifth notes, note 5. **Only a press
  // into a box that does not have the focus does it**: the press is stopped
  // before it can place the caret, and the box takes the focus with all its
  // text chosen. A redraw's return of the focus (`./the-typed-text.js`) is no
  // press, so it chooses nothing and typing in progress keeps its caret. The
  // job name is not a value box and is left alone. Added 2026-10-08.
  box.addEventListener("mousedown", (press) => {
    if (press.button !== 0 || document.activeElement === box) return;
    press.preventDefault();
    theFocusMovesTo(box);
    box.select();
  });
  box.addEventListener("change", () => answered(box.value));
  return box;
}

/// **The line an act's answer is put on**, beside the control that asked.
/// Hidden until there is something to say. **What the answer says decides its
/// colour** when it arrives (`theRefusalIsPutOnItsBox`); a refusal is what
/// every line but the submission's ever carries.
/// **The read's line as it reads while a read runs, drawn unseen** —
/// *Reading the file*, a clock and *Stop*, which is what makes it tall — to
/// hold its height
/// in the line under the version row (UI consistency, finding 15).
function theReadLineHeld(said) {
  const line = document.createElement("div");
  line.className = "sub rdline pkheld";
  line.setAttribute("aria-hidden", "true");
  line.appendChild(document.createTextNode(said.reading_the_file));
  line.appendChild(document.createTextNode(" · "));
  const clock = document.createElement("span");
  clock.className = "tnum";
  clock.textContent = "0:00";
  line.appendChild(clock);
  line.appendChild(document.createTextNode(" · "));
  const stop = document.createElement("button");
  stop.type = "button";
  stop.className = "mini inl";
  stop.tabIndex = -1;
  stop.textContent = said.stop_read;
  line.appendChild(stop);
  return line;
}

function anAnswerLine(which) {
  const line = document.createElement("p");
  line.className = "alertnote";
  line.dataset.box = `${which}-answer`;
  line.hidden = true;
  return line;
}

/// **The cross's room where there is nothing to hand back**, the drawing's
/// `.clear.held`: what follows a value — *?*, *240 frames*, the pixel reading
/// — sits where it sits with the cross, so typing a value moves none of it
/// (UI consistency, finding 24).
function theCrossesRoom() {
  const room = document.createElement("span");
  room.className = "clear held";
  room.setAttribute("aria-hidden", "true");
  room.textContent = "✕";
  return room;
}

function theCross(handBackTo, pressed, which) {
  const cross = document.createElement("button");
  cross.type = "button";
  cross.className = "clear";
  if (which) cross.dataset.box = which;
  // **The cross says what it hands back to** rather than leaving it to be
  // discovered.
  cross.setAttribute("aria-label", theWordsOf(handBackTo));
  cross.textContent = "✕";
  cross.addEventListener("click", pressed);
  return cross;
}

/// **A tooltip on a small question mark shows the four forms**, on hover and on
/// keyboard focus, and nothing is explained permanently.
function theTip(said) {
  const tip = document.createElement("span");
  tip.className = "tip";
  const mark = document.createElement("button");
  mark.type = "button";
  mark.className = "q";
  mark.dataset.box = "range-tip";
  mark.setAttribute("aria-label", said.how_to_type_a_range);
  mark.textContent = "?";
  tip.appendChild(mark);
  const bubble = document.createElement("span");
  bubble.className = "bubble";
  const list = document.createElement("dl");
  for (const form of said.the_range_forms) {
    const term = document.createElement("dt");
    term.textContent = form.value;
    const means = document.createElement("dd");
    means.textContent = form.said;
    list.appendChild(term);
    list.appendChild(means);
  }
  bubble.appendChild(list);
  tip.appendChild(bubble);
  // **The help is brought into view when it opens** (the UI at every size,
  // C5): near the bottom of the settings it opened under the panel's foot.
  // The scroll is the engine's, asked of the help itself, so the page still
  // measures nothing but its one dropdown.
  const theHelpIsShown = () => bubble.scrollIntoView({ block: "nearest", inline: "nearest" });
  tip.addEventListener("mouseenter", theHelpIsShown);
  tip.addEventListener("focusin", theHelpIsShown);
  return tip;
}

function aBox(name, ticked, extra, pressed, which, theNameAsDrawn) {
  const box = document.createElement("span");
  if (which) box.dataset.box = which;
  box.className = ["box", ...extra, ticked ? "on" : ""].filter(Boolean).join(" ");
  box.setAttribute("role", "button");
  box.setAttribute("aria-pressed", String(Boolean(ticked)));
  box.tabIndex = 0;
  const mark = document.createElement("i");
  mark.textContent = ticked ? "✓" : "";
  box.appendChild(mark);
  // A scene's or a View Layer's own name, kept apart from the tick and the
  // tag beside it (X9, `web/the-value-drawn.js`).
  box.appendChild(theNameAsDrawn || theValueDrawn(name));
  box.addEventListener("click", pressed);
  box.addEventListener("keydown", (event) => {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      pressed();
    }
  });
  return box;
}

function aSwitch(label, on, pressed, which, greyed) {
  const line = document.createElement("span");
  line.className = "swline";
  const control = document.createElement("button");
  control.type = "button";
  if (which) control.dataset.box = which;
  control.className = on ? "sws on" : "sws";
  control.setAttribute("aria-label", label);
  control.setAttribute("aria-pressed", String(Boolean(on)));
  // **A greyed switch cannot be pressed but keeps its setting** — it is drawn
  // on or off as it stands (the drawing's `.sws:disabled`, nineteenth notes,
  // note 8).
  if (greyed) control.disabled = true;
  control.appendChild(document.createElement("i"));
  control.addEventListener("click", pressed);
  line.appendChild(control);
  const said = document.createElement("span");
  said.textContent = label;
  line.appendChild(said);
  return line;
}

function aLink(label, pressed, which) {
  const link = document.createElement("button");
  link.type = "button";
  link.className = "lnkb";
  if (which) link.dataset.box = which;
  link.textContent = label;
  link.addEventListener("click", pressed);
  return link;
}

/// **The small button, held to the height of the line it sits in** — the
/// drawing's `.mini.inl`, as the dependency check's *Stop* is.
function anInlineButton(label, pressed, which) {
  const button = aLink(label, pressed, which);
  button.className = "mini inl";
  return button;
}

function aCell(classes, text) {
  const cell = document.createElement("span");
  cell.classList.add(...classes);
  cell.textContent = text;
  return cell;
}

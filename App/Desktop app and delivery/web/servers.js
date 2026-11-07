// **All servers, on the page** — the list where servers are compared, its
// empty state, the add and edit dialogs, the five menus and the six boxes.
// The server pane takes this module’s connection control and complete reading;
// the rail asks this module for its menu. Both keep these same action handlers. The
// application half is `src/surface/servers.rs`, whose head carries the whole
// account of the piece.
//
// **Read the head of `web/the-rail.js` first.** This module keeps every part of
// the mounting contract: it registers under the drawing's own `serversview`,
// draws once on the first visit, asks the application again every time it
// becomes current, and remembers nothing of its own.
//
// ## Three things this file must never do
//
// - **Never write a word of prose into it.** A6 and X1: the typography point is
//   the only way prose reaches a screen and the enforcement is structural on
//   both sides. Every word below arrived from the application.
// - **Never join a value into a sentence.** X7. A server's name, its address,
//   a count and a moment all arrive beside their words and are drawn as
//   separate nodes. Joining them here would undo the split the application
//   side made, and no test anywhere would notice.
// - **Never copy the mockup's markup.** That file carries 88 inline `style=`
//   attributes and four uses of `innerHTML`, **every one inert under this
//   application's content policy** — `default-src 'none'`, `script-src 'self'`,
//   `style-src 'self'`, `img-src 'self' blob:`. H32: values are content, never
//   markup. H33: no inline style; the rules are in `servers.css`.
//
// ## What this file does not build, because it is already built
//
// - **The failure shape.** `theFailureShape` in `web/the-failure-shape.js`, in
//   its **collapsed room** on a row and its **expanded room** where the list
//   itself could not be read. **Not one of the mockup's copies is copied here.**
// - **The narrowing.** `web/panes.js` writes the step mark on to `.app` and the
//   drawing's own blocks are keyed to it. This screen has no seam of its own —
//   the record holds three and there is no fourth.
// - **Picking a key on the Keys screen.** `aServerDialogPicksAKey` in
//   `web/keys.js` opens the Keys screen in its picking mode and hands back
//   `{ row, fingerprint }` or nothing (`Keys` U11). The dialog's own key
//   dropdown, `aKeyControl`, is this file's (piece E3), and its *Open Keys…*
//   and *New key…* are what reach that screen. The dialog stays where it is while the user is away,
//   and **the pair is read from the key row again when the dialog is shown**
//   (`Keys` U12) — step 10a round 1, fix-here item 2.
//
// **P9 and P10: nothing here polls and nothing here ticks.** The one thing that
// ticks is the countdown beside a repairing server, and that is
// `web/the-underway-shape.js`'s under M30 and P10b — **this file asks that
// shape to draw it** on a repairing row and holds no timer of its own (step
// 10b's first audit round, item 9). **The standing failures are asked for
// once per fetch of the list, and an answer redraws only its own row and asks
// nothing** (step 10a round 1, fix-here item 1: each redraw used to ask again,
// without end).

import { whenTold } from "./event-route.js";
import {
  aScreenIsRegistered,
  ALL_SERVERS,
  A_SERVER,
  KEYS,
  theCurrentPlace,
  theCurrentPlaceIs,
  theRailIsReadAgain,
} from "./the-rail.js";
import {
  theFailureShape,
  theControlsFrom,
  A_COLLAPSED_ROW,
  EXPANDED,
} from "./the-failure-shape.js";
import { theServerWillOpenOn, SET_UP, theServersAreAskedForAgain } from "./a-server.js";
import { aServerDialogPicksAKey } from "./keys.js";
import { theCountdown, theMarkOnAControl, theMarkComesOffAControl } from "./the-underway-shape.js";
import { theWordsPanel } from "./the-machines-own-words.js";
import { theValueDrawn, theValueIsDrawnInto } from "./the-value-drawn.js";

const shell = window.__TAURI__;

// Declared beside the service that posts it — `THE_ANSWER` in
// `src/surface/servers.rs`. The two sides have to agree and nothing holds them
// together, so copy carefully.
const THE_ANSWER = "servers:the-answer";

// **What a server is doing moved** — `THE_WORK_MOVED` in
// `src/surface/servers.rs`. It carries nothing; the list asks for itself again.
const THE_WORK_MOVED = "servers:the-work-moved";

// **One key's verdict on a server dialog's key list** — `A_DIALOG_KEYS_VERDICT`
// in `src/surface/servers.rs`, `Keys` U10a. It carries the ask it answers.
const A_DIALOG_KEYS_VERDICT = "servers:a-dialog-keys-verdict";

// **The discovery pass wrote a row behind the list's draw** —
// `A_DIALOG_KEYS_LIST_GREW`, `Keys` U10a. It carries the ask it answers.
const A_DIALOG_KEYS_LIST_GREW = "servers:a-dialog-keys-list-grew";

// **What `Servers` publishes, declared beside the service that posts it** —
// `WHAT_IS_OBSERVED`, `THE_CONNECTION_CONDITION` and
// `THE_CHANGED_IDENTITY_WARNING` in `src/servers/published.rs`. The two sides
// have to agree and nothing holds them together, so copy carefully.
const WHAT_IS_OBSERVED = "servers:what-is-observed";
const THE_CONNECTION_CONDITION = "servers:the-connection-condition";
const THE_CHANGED_IDENTITY_WARNING = "servers:the-changed-identity-warning";

// The failure shape's own kinds, declared in `src/surface/the_failure_shape.rs`.
const A_FAILURE_IS_STANDING = "presentation:a-failure-is-standing";
const A_FAILURE_WAS_CLEARED = "presentation:a-failure-was-cleared";


// ── What this screen holds for the life of the run ──────────────────────────
//
// **Everything here is one drawing's own elements and nothing is remembered.**
// The contract's section 3 is why: the host stays for the life of the run, so a
// half-typed field and an open menu survive going away and coming back for
// nothing. **Nothing below is state about the user's work.**

let host = null;
let heading = null;
let addHolder = null;
let toolsLine = null;
let list = null;
let failed = null;
let footer = null;
let empty = null;
let dialogs = null;
// server identity → (failure key → the failure's own event). **Every standing
// failure is kept, keyed by its own key, and each is drawn** — M7a.
const failuresByServer = new Map();
// server identity → the row's element, so one row can be drawn again alone.
const rowElements = new Map();
// **The connects that did not happen, the newest for each server** — server
// identity → the answer's own `said`. Added 2026-10-03, the Creator's note 14
// on the test copy: *Add and set up* had moved the user to the server's Set up
// tab and the sentence was written here, hidden. **It is a sentence the
// application already said, kept for the run in the page** (the map's hole 1,
// as Files keeps its line), drawn above the list whenever the list is on
// screen and handed to the Set up tab (`web/set-up.js`), and **gone at the
// next act on that server** — a connect, an edit, a rename, a removal or
// dropping the setup — or when Smoozy closes. Nothing durable is written.
const theConnectsNotMade = new Map();
let theConnectsNotMadeHolder = null;
// The last screen the application answered with, so a redraw after an act does
// not have to be composed from two answers.
let lastDrawn = null;
// The two closed acts, off the last way-in event.

// ── The one place a value becomes a node ────────────────────────────────────

function textNode(said) {
  return document.createTextNode(said || "");
}

// **What stands between a figure and the words after it, where the application
// marks them bound** — U+00A0, so *24 GB* or *2 jobs* never parts across two
// lines. `Design/Whole app § Typography`; the character's one home is
// `src/typography.rs` (`BETWEEN_A_FIGURE_AND_ITS_UNIT`), and this page writes
// it only where the mark says, never by reading which word is a unit.
const BETWEEN_A_FIGURE_AND_ITS_UNIT = "\u00A0";

/// **Draw a piece of prose with the values that belong beside it.**
///
/// The application hands over `{ lead, values, rest }` — the words before the
/// first value, the values, and the words after the last. **This sets them side
/// by side and joins nothing**, which is the page half of X7.
function saidInto(holder, said, valueClass) {
  if (!said) return holder;
  // **One space between one part and the next**, and a space is punctuation
  // rather than a word — the same route used for an em dash
  // between its own two strings. **Nothing here composes anything**: the parts
  // arrive composed and this sets them side by side.
  let first = true;
  const part = (node) => {
    if (!first) holder.appendChild(textNode(" "));
    first = false;
    holder.appendChild(node);
  };
  if (said.lead) {
    const lead = document.createElement("span");
    lead.textContent = said.lead;
    part(lead);
  }
  for (const value of said.values || []) {
    const one = document.createElement("span");
    if (valueClass) one.className = valueClass;
    // **Read in the order its bytes run** — X9, `web/the-value-drawn.js`, and
    // so is every name and address below.
    theValueIsDrawnInto(one, value);
    part(one);
  }
  if (said.rest) {
    const rest = document.createElement("span");
    rest.textContent = said.rest;
    // **Whether the rest follows with nothing between them is the sentence's
    // own fact**, and the application states it — a page deciding it would be
    // a page reading prose.
    if (said.the_rest_joins) {
      holder.appendChild(rest);
    } else if (said.the_rest_binds) {
      holder.appendChild(textNode(BETWEEN_A_FIGURE_AND_ITS_UNIT));
      holder.appendChild(rest);
    } else {
      part(rest);
    }
  }
  return holder;
}

/// **A moment, as this computer would write one.**
///
/// `Servers` H2a leaves the granularity to Design and Design words it three
/// ways — a time, a day and a time, a day. **Choosing between them is a
/// comparison against now**, which is why the application supplies the moment
/// and this side supplies the choice.
///
/// **No month name is written here.** The words come from the system's own
/// locale data through `toLocaleString`, so this file holds no prose and the
/// user reads their own computer's way of writing a date.
///
/// **Exported for the server's own head**, which draws the same *last seen*
/// mark with the same formatting (step 10a round 1, fix-here item 15).
export function theMomentReads(seconds) {
  const at = new Date(Number(seconds) * 1000);
  if (Number.isNaN(at.getTime())) return "";
  const now = new Date();
  const sameDay =
    at.getFullYear() === now.getFullYear() &&
    at.getMonth() === now.getMonth() &&
    at.getDate() === now.getDate();
  if (sameDay) return at.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
  if (at.getFullYear() === now.getFullYear()) {
    return at.toLocaleString([], {
      day: "numeric",
      month: "short",
      hour: "2-digit",
      minute: "2-digit",
    });
  }
  return at.toLocaleDateString([], { day: "numeric", month: "short", year: "numeric" });
}

function aButton(control, onPress) {
  const one = document.createElement("button");
  one.type = "button";
  one.textContent = control.label;
  if (control.unavailable) one.disabled = true;
  if (onPress) one.addEventListener("click", onPress);
  return one;
}

// ── The screen ──────────────────────────────────────────────────────────────

aScreenIsRegistered({
  place: ALL_SERVERS,
  itIsDrawn(into) {
    host = into;

    const head = document.createElement("div");
    head.className = "svhead";
    heading = document.createElement("h1");
    head.appendChild(heading);
    const grow = document.createElement("span");
    grow.className = "grow";
    head.appendChild(grow);
    addHolder = document.createElement("span");
    head.appendChild(addHolder);
    host.appendChild(head);
    host.dataset.heading = "";

    toolsLine = document.createElement("div");
    toolsLine.className = "sv-notool";
    toolsLine.hidden = true;
    host.appendChild(toolsLine);

    // **The connects that did not happen, one line each, above the list.**
    theConnectsNotMadeHolder = document.createElement("div");
    host.appendChild(theConnectsNotMadeHolder);

    list = document.createElement("div");
    list.className = "svlist";
    host.appendChild(list);

    // **Where the list goes when the list could not be read** — L8a, and
    // never the guidance (step 10a round 1, fix-here item 4).
    failed = document.createElement("div");
    failed.className = "svfailed";
    failed.hidden = true;
    host.appendChild(failed);

    footer = document.createElement("div");
    footer.className = "svfoot";
    host.appendChild(footer);

    empty = document.createElement("div");
    empty.className = "svempty";
    empty.hidden = true;
    host.appendChild(empty);

    theDialogsAreReady(host);

    // **The two keys Design binds and no others.** *"Return opens the chosen
    // server. The delete key removes it, asking exactly as the control does.
    // Nothing else is bound."* **Only where the row itself has the keyboard**
    // — a key pressed on a control inside the row is that control's (step 10a
    // round 1, fix-here item 27), as `web/keys.js` already does.
    host.addEventListener("keydown", (press) => {
      if (press.defaultPrevented) return;
      const row = press.target.closest && press.target.closest(".svrow");
      if (!row || press.target !== row) return;
      if (press.key === "Enter") {
        press.preventDefault();
        theServerIsOpened(row.dataset.server);
      }
      if (press.key === "Delete" || press.key === "Backspace") {
        press.preventDefault();
        theRemovalIsOpened(row.dataset.server);
      }
    });

    theScreenIsAskedFor();
  },

  // **The contract's section 4: use it to ask the application again.** Events
  // are volatile and nothing is replayed (P8), so a screen that has been away
  // has facts it should re-read.
  itIsCurrentNow() {
    if (!host) return;
    theWorkColumnIsOnScreen(true);
    theScreenIsAskedFor();
  },

  // **Shut whatever is open, and save nothing.** The element stays and so does
  // everything in it — a dialog the user left to pick a key included.
  itIsPutAway() {
    theMenuIsClosed();
    theRenameIsLetGo();
    theWorkColumnIsOnScreen(false);
  },
});

/// **P10d repeats the queue reading while a surface showing a server's queue
/// is on screen, and the runner ruled on 2026-09-28 that this list is one** —
/// its work column draws each server's rendering job and how many wait. So the
/// application is told when the list is shown and when it is put away, and
/// the column does not go stale while Queue is away. Nothing comes back.
function theWorkColumnIsOnScreen(shown) {
  shell.core
    .invoke("the_work_column_is_on_screen", { which: "the-list", server: null, shown })
    .catch(() => {});
}

let screenReading = 0;

function theScreenIsAskedFor() {
  const reading = ++screenReading;
  return shell.core
    .invoke("the_all_servers_screen")
    .then(async (screen) => {
      // **A press already on its way is not lost to a redraw** (below).
      await theDrawingWaitsForThePress();
      if (reading !== screenReading) return;
      lastDrawn = screen;
      if (heading) heading.textContent = screen.the_screens_own_name;
      if (host) theScreenIsDrawn(screen);
      theSharedControlsAreDrawn(screen);
      theOpenMenuIsRefreshed(screen);
      // **`Servers` W22's next presentation**: a removal's stop and clear that
      // did not come to *cleared and stopped* raises the `sv-sc` box on the
      // list, whether the list was showing when the act ended or opened after.
      // The application hands it over once.
      if (screen.the_stop_and_clear_ending) theOwedEndingIsShown(screen);
      // **What is standing against each of these rows, asked once per fetch
      // and never per draw** (step 10a round 1, fix-here item 1). The shape's
      // own service reads the record and posts one event per standing failure
      // (M7a).
      //
      // **Nothing here composes a failure sentence** — the words are the shape
      // service's, and `Desktop app and delivery` §24 leaves the real ones to
      // Design. Whoever composes them passes `TheComposersOwn`; this piece
      // passes nothing, because it composes nothing.
      for (const row of screen.rows) {
        shell.core.invoke("the_failures_standing_against", {
          subjectKind: screen.the_subject_kind,
          subject: row.server,
        });
      }
    })
    .catch((failure) => {
      // **L8a: the screen shows the failure rather than an emptied version of
      // itself.** The sentence is the application's own, carried on the error
      // half of its answer (step 10a round 1, fix-here item 4).
      if (reading !== screenReading) return;
      theListCouldNotBeRead(failure);
      theMenuIsClosed();
    });
}

function theListCouldNotBeRead(failure) {
  if (!host) return;
  host.dataset.screen = "failed";
  list.hidden = true;
  footer.hidden = true;
  empty.hidden = true;
  toolsLine.hidden = true;
  failed.replaceChildren();
  const said = failure && failure.said;
  failed.hidden = !said;
  if (!said) return;
  failed.appendChild(
    theFailureShape({ sentence: said, room: EXPANDED, values: [], controls: [] }),
  );
}

function theAddControlIsDrawn(screen) {
  addHolder.replaceChildren();
  const add = document.createElement("button");
  add.type = "button";
  add.className = "btn";
  add.textContent = screen.add_a_server;
  add.addEventListener("click", () => theAddDialogIsOpened());
  addHolder.appendChild(add);
}

function theScreenIsDrawn(screen) {
  if (host) delete host.dataset.screen;
  failed.hidden = true;
  failed.replaceChildren();
  theAddControlIsDrawn(screen);

  // **One line above the list, said once and never per row** — the Keys
  // screen's own three sentences (step 10a round 1, fix-here item 30).
  toolsLine.textContent = "";
  toolsLine.hidden = !screen.the_tools_statement;
  if (screen.the_tools_statement) {
    const lead = document.createElement("b");
    lead.textContent = screen.the_tools_statement.lead;
    toolsLine.appendChild(lead);
    toolsLine.appendChild(textNode(" "));
    const until = document.createElement("span");
    until.textContent = screen.the_tools_statement.until;
    toolsLine.appendChild(until);
  }

  // **A name being typed keeps the keys through a redraw** — this list is drawn
  // again whenever a server's value moves.
  const hadTheKeys = theNameFieldHasTheKeys();
  if (renaming && !screen.rows.some((row) => row.server === renaming.server)) renaming = null;
  list.textContent = "";
  rowElements.clear();
  // A failure standing against a server that is no longer listed goes with it.
  for (const server of Array.from(failuresByServer.keys())) {
    if (!screen.rows.some((row) => row.server === server)) failuresByServer.delete(server);
  }
  // **So does a connect that did not happen**, and so does one whose server
  // is connected now, however the connection was made — the passphrase box's
  // own *Connect* and an edit's put-back included, which this page does not
  // see asked (the Creator's note 14 on the test copy). The rest are drawn
  // again.
  for (const server of Array.from(theConnectsNotMade.keys())) {
    const row = screen.rows.find((one) => one.server === server);
    if (!row || theRowIsLive(row)) theConnectsNotMade.delete(server);
  }
  theConnectsNotMadeAreDrawn();
  for (const row of screen.rows) {
    const drawn = aRow(row, screen);
    rowElements.set(row.server, drawn);
    list.appendChild(drawn);
  }
  if (hadTheKeys) theNameFieldTakesTheKeys(false);

  footer.textContent = "";
  const counts = document.createElement("span");
  saidInto(counts, screen.footer.how_many, "mono");
  counts.appendChild(textNode(" · "));
  saidInto(counts, screen.footer.how_many_connected, "mono");
  footer.appendChild(counts);
  const grow = document.createElement("span");
  grow.className = "grow";
  footer.appendChild(grow);
  footer.appendChild(document.createElement("span"));

  // **The empty state is a state of this screen and not of the shell**, which
  // is the mounting contract's section 8 read from this side.
  const nothing = screen.rows.length === 0;
  list.hidden = nothing;
  footer.hidden = nothing;
  empty.hidden = !nothing;
  if (nothing) theGuidanceIsDrawn(screen.guidance);
}

/// **One row drawn again, and nothing asked** — what a failure arriving does.
async function theRowIsDrawnAgain(server) {
  await theDrawingWaitsForThePress();
  if (!lastDrawn) return;
  const row = lastDrawn.rows.find((one) => one.server === server);
  const old = rowElements.get(server);
  if (!row || !old || !old.parentNode) return;
  const hadTheKeys = theNameFieldHasTheKeys();
  const drawn = aRow(row, lastDrawn);
  old.parentNode.replaceChild(drawn, old);
  rowElements.set(server, drawn);
  if (hadTheKeys) theNameFieldTakesTheKeys(false);
}

// ── A press already on its way ──────────────────────────────────────────────
//
// **`Design/Whole app § The window`: *"a press already on its way is not lost
// to a redraw: a page that redraws while a control is held down keeps that
// control, and the press lands where it began."*** This list is drawn again
// whole whenever `Servers`' value moves on any server, and one row again
// whenever a failure arrives on it — so a *Connect* held down while another
// server's repair moves, or while this one's failure arrives, was taken out of
// the page under the press, and a press whose control is gone makes no click.
// **Proved in the page harness before this was written, beside a plain press
// that did connect.** Added 2026-09-27, step 10c's eighth-pass fix piece 5.
//
// **So while a control or a row on this screen is held down, the drawing
// waits**, and goes ahead the moment the release's click is under way — the
// click is already aimed where the press began, so the drawing can no longer
// take it away. Nothing is asked twice and nothing is remembered: the answer
// is drawn a moment later, exactly as it came. **The wait ends on events
// alone, never on a clock** (P9, P10): the click, a cancelled press, the
// window losing the focus, the next press, and — where a release made no
// click at all — the first movement or key after it.
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
    const held = press.target.closest('button,[role="button"],.svrow');
    if (held && (held.disabled || held.getAttribute("aria-disabled") === "true")) return;
    if (held && ((host && host.contains(held)) || (openMenu && openMenu.contains(held)) || Array.from(connectionPlaces.keys()).some((place) => place.contains(held)))) thePressHeld = { released: false };
  },
  true,
);
document.addEventListener("click", () => thePressEnded(), true);
document.addEventListener("pointercancel", () => thePressEnded(), true);
document.addEventListener(
  "pointerup",
  () => {
    if (thePressHeld) thePressHeld.released = true;
  },
  true,
);
for (const after of ["pointermove", "keydown"]) {
  document.addEventListener(
    after,
    () => {
      if (thePressHeld && thePressHeld.released) thePressEnded();
    },
    true,
  );
}
if (typeof window.addEventListener === "function") {
  window.addEventListener("blur", () => thePressEnded());
}

// ── A row ───────────────────────────────────────────────────────────────────

function aRow(row, screen) {
  const drawn = document.createElement("div");
  drawn.className = "svrow";
  // **The row is reachable by the keyboard and it is not a button.**
  //
  // **The mockup writes `role="button"` on the row and this piece does not, and
  // that is a repair rather than a departure.** A row carrying that role is one
  // element to everything that reads the window, so **the connect control and
  // the three dots inside it stop existing** — no assistive technology can
  // reach either, and neither can this project's own driver, which is how it
  // was found. `Design/Servers` asks only that **clicking the row opens the
  // server** and that **Return opens the chosen one**, and both hold without
  // the role.
  drawn.tabIndex = 0;
  drawn.dataset.server = row.server;
  drawn.dataset.menu = row.the_menu;

  const main = document.createElement("span");
  main.className = "svmain";

  const name = document.createElement("span");
  name.className = "svname";
  if (row.badge) {
    const badge = document.createElement("span");
    // **Two names added rather than one string joined**: a class list is the
    // one place the page may put a value, and the application already checked
    // this one against the drawing's eight.
    badge.classList.add("badge", row.badge);
    name.appendChild(badge);
  }
  // A value: the server's own name, which the user can copy — **or the field
  // it is being renamed in**, in place on the row (`Design/Servers § Actions
  // on a row`; the Creator's note 1 on 0.1.8).
  const isRenamed = Boolean(renaming && renaming.server === row.server);
  name.appendChild(isRenamed ? theNameField() : theValueDrawn(row.name));
  main.appendChild(name);
  if (isRenamed) main.appendChild(theSameNameLine());

  const address = document.createElement("span");
  address.className = "svaddr mono";
  theValueIsDrawnInto(address, row.address);
  main.appendChild(address);

  // **The extra line, and there is one of it.** Design: it is *"either a
  // failure that is still standing, or the mark saying the server needs a new
  // key"*, and **where both are true the failure takes the line**. **Every
  // standing failure is drawn in it**, each exactly as it would be alone (M7a).
  const failures = failuresByServer.get(row.server);
  if (failures && failures.size) {
    const extra = document.createElement("span");
    extra.className = "svextra";
    for (const failure of failures.values()) extra.appendChild(theFailureOnTheRow(failure));
    main.appendChild(extra);
  } else if (row.the_update_line) {
    // **Smoozy on this server older or newer than the app** — `Design/Servers
    // § A row`, after a standing failure and before the key mark, which
    // stands either way and takes the line back afterwards.
    main.appendChild(theUpdateLine(row));
  } else if (row.the_key_mark) {
    const extra = document.createElement("span");
    extra.className = "svextra";
    const mark = document.createElement("span");
    mark.className = "svkeynote";
    saidInto(mark, row.the_key_mark);
    extra.appendChild(mark);
    main.appendChild(extra);
  } else if (row.the_setup_line) {
    const extra = document.createElement("span");
    extra.className = "svextra";
    saidInto(extra, row.the_setup_line);
    main.appendChild(extra);
  }
  // **`Security and licence` S34's one sentence, said once**, and drawn
  // whether or not the mark beside it has the line — `Design/Keys § On a
  // computer with nowhere to keep a passphrase`, mockup state `sv-nostore`.
  if (row.the_sentence) {
    const extra = document.createElement("span");
    extra.className = "svextra";
    const said = document.createElement("span");
    said.className = "svkeynote";
    said.textContent = row.the_sentence;
    extra.appendChild(said);
    main.appendChild(extra);
  }
  drawn.appendChild(main);

  drawn.appendChild(theWorkColumn(row));
  drawn.appendChild(theHardware(row));

  const acts = document.createElement("span");

  acts.className = "svacts";
  acts.appendChild(theConnectControl(row, screen));
  const more = document.createElement("button");
  more.type = "button";
  more.className = "more svmore";
  more.setAttribute("aria-label", row.more);
  // A glyph rather than a word: nothing here is prose.
  more.textContent = "⋯";
  more.addEventListener("click", (press) => {
    press.stopPropagation();
    theMenuIsOpened(row, screen, more);
  });
  acts.appendChild(more);
  drawn.appendChild(acts);

  // **Clicking the row opens the server** — Design, and it is why no row
  // carries a *Set up* control.
  drawn.addEventListener("click", () => theServerIsOpened(row.server));
  // **Behind a right-click, which opens the same menu.**
  drawn.addEventListener("contextmenu", (press) => {
    press.preventDefault();
    theMenuIsOpened(row, screen, more, press);
  });
  return drawn;
}

/// **Smoozy on this server older or newer than the app, as the row's extra
/// line** — `Desktop app and delivery` V13's condition in `Design/Servers § A
/// row`'s words, the drawing's `sv-upd`. Step 10c's transfer wave piece 9.
///
/// - **Waiting for its jobs**, with **Update now…** inside the line, since the
///   act that would move it belongs inside it (V15). It opens the box and does
///   not open the server.
/// - **Updating**, in the ordinary face.
/// - **Could not be updated**, in the one failure shape, collapsed on to the
///   row, with the server's own words behind it and the cross.
/// - **Too old**, in either of its two forms, and **too new**, set as the
///   drawing sets them.
function theUpdateLine(row) {
  const line = row.the_update_line;
  const extra = document.createElement("span");
  extra.className = "svextra";
  extra.dataset.update = line.which;
  if (line.a_failure) {
    const words = line.the_servers_words;
    extra.appendChild(
      theFailureShape({
        sentence: line.said.lead,
        kind: "condition",
        subject: row.server,
        room: A_COLLAPSED_ROW,
        controls: words
          ? [
              { is: "raw-text", label: words.label, act: "the-words" },
              { is: "cross", label: words.clear.label, act: words.clear.act },
            ]
          : [],
        onAct() {
          if (!words) return;
          const panel = theWordsPanel({
            title: words.label,
            tabs: [{ name: words.label, record: words.words }],
            close: words.close,
          });
          panel.hidden = false;
          dialogs.appendChild(panel);
        },
        onClear() {
          if (words) theActIsTaken(words.clear.act, row.server);
        },
      })
    );
    return extra;
  }
  if (line.which === "too-old" || line.which === "too-old-unknown" || line.which === "too-new") {
    const note = document.createElement("span");
    note.className = "svkeynote";
    saidInto(note, line.said);
    extra.appendChild(note);
  } else {
    saidInto(extra, line.said);
  }
  if (line.update_now && line.the_box) {
    extra.appendChild(textNode(" "));
    const now = document.createElement("button");
    now.type = "button";
    now.className = "lnkb";
    now.textContent = line.update_now.label;
    now.dataset.act = line.update_now.act;
    now.addEventListener("click", (press) => {
      // **Inside the row, so it must not open the server as well.**
      press.stopPropagation();
      theUpdateNowIsOpened(row.server, line.the_box);
    });
    extra.appendChild(now);
  }
  return extra;
}

/// **The box *Update now…* opens** — `Design/Servers § A row` and the drawing's
/// `sv-updnow`: the question, the removal box's own statement of what *stop and
/// clear* does (V15, `Servers` W19a unchanged), then *Cancel* and *Stop jobs
/// and update*. **Where the act does not land, the box says so and stays**, in
/// the words the application carried, and the row keeps its waiting line.
function theUpdateNowIsOpened(server, said) {
  const box = aDialog();
  const heading = document.createElement("h3");
  saidInto(heading, said.heading, "mono");
  box.appendChild(heading);
  const sub = document.createElement("p");
  sub.className = "msub";
  sub.textContent = said.said;
  box.appendChild(sub);
  if (said.the_warning) {
    const warning = document.createElement("p");
    warning.className = "mwarn hard";
    const bold = document.createElement("b");
    said.the_warning.forEach((run, at) => {
      if (at > 0) bold.appendChild(textNode(" "));
      saidInto(bold, run, "mono");
    });
    warning.appendChild(bold);
    box.appendChild(warning);
  }
  const statement = document.createElement("p");
  statement.className = "msub";
  saidInto(statement, said.the_statement, "mono");
  box.appendChild(statement);
  const notice = aNoticeIn(box);
  const row = document.createElement("div");
  row.className = "mrow";
  const grow = document.createElement("span");
  grow.className = "grow";
  row.appendChild(grow);
  const cancel = document.createElement("button");
  cancel.type = "button";
  cancel.className = "btn quiet";
  cancel.textContent = said.cancel;
  cancel.addEventListener("click", theDialogIsClosed);
  row.appendChild(cancel);
  const stop = document.createElement("button");
  stop.type = "button";
  stop.className = "btn danger";
  stop.textContent = said.stop_jobs_and_update.label;
  stop.addEventListener("click", () => {
    if (underway) return;
    theControlIsUnderway(stop, said.stop_jobs_and_update.label);
    shell.core.invoke("an_act_that_needs_a_connection", { act: said.stop_jobs_and_update.act, server });
  });
  row.appendChild(stop);
  box.appendChild(row);
  theRefusalsAreDrawnInto({ notice, fields: {} });
  theDialogIsLabelled(box, heading);
}

/// **What the server is doing** — `Design/Servers § A row` and the drawing's
/// own `<b>Rendering</b> Kitchen — final<span class="sub">2 waiting</span>`.
/// The first part is the column's word, set in `<b>` where a job's name
/// follows it; **every part after the first goes beneath it**, in the
/// drawing's `.sub`, which is where *how many are waiting* sits.
function theWorkColumn(row) {
  const work = document.createElement("span");
  work.className = "svwork";
  const [first, ...beneath] = row.the_work;
  if (first && first.values && first.values.length && first.lead) {
    const word = document.createElement("b");
    word.textContent = first.lead;
    work.appendChild(word);
    work.appendChild(textNode(" "));
    saidInto(work, { ...first, lead: "" });
  } else {
    saidInto(work, first);
  }
  if (beneath.length) {
    const sub = document.createElement("span");
    sub.className = "sub";
    for (const said of beneath) saidInto(sub, said);
    work.appendChild(sub);
  }
  // **When the next attempt falls**, with the underway shape's own countdown —
  // `Design/Servers § A row`, *Reconnecting…* *"with when the next attempt
  // falls"*. Where the moment is absent the word stands alone (M30).
  if (row.the_next_attempt) {
    work.appendChild(
      theCountdown(row.the_next_attempt.due_at, {
        in: row.the_next_attempt.trying_again_in,
        now: row.the_next_attempt.trying_again_now,
      }),
    );
  }
  return work;
}

function theHardware(row) {
  const drawn = document.createElement("span");
  drawn.className = "svhw";
  const hardware = row.the_hardware;
  if (hardware.state === "last-seen") drawn.classList.add("stale");
  if (hardware.nothing_to_show) {
    drawn.classList.add("none");
    saidInto(drawn, hardware.nothing_to_show);
  } else {
    saidInto(drawn, hardware.cards, "mono");
  }
  const sub = document.createElement("span");
  sub.className = "sub";
  const beside = (said, drawn) => {
    if (!said) return;
    if (sub.childElementCount) sub.appendChild(textNode(" · "));
    saidInto(sub, drawn || said, "mono");
  };
  beside(hardware.memory);
  beside(hardware.free_disk);
  // **The mark is the desktop's own moment** and the granularity is this
  // side's — `Servers` H2a.
  if (hardware.last_seen) {
    beside(hardware.last_seen, {
      lead: hardware.last_seen.lead,
      values: hardware.last_seen.values.map(theMomentReads),
      rest: hardware.last_seen.rest,
    });
  }
  if (sub.childElementCount) drawn.appendChild(sub);
  return drawn;
}

/// **A row whose connection is live**: its control offers *Disconnect*, and
/// no connect of the user's is under way on it and no repair is bringing it
/// back — the drawing's green. The one reading of it on this page.
function theRowIsLive(row) {
  return (
    row.the_connection.act === "disconnect" &&
    !row.the_connect_is_underway &&
    !row.the_connection_is_coming_back
  );
}

const pendingConnections = new Set();

function theConnectControl(row, screen) {
  const control = document.createElement("button");
  control.type = "button";
  control.className = "cbtn";
  const dot = document.createElement("span");
  dot.className = "cdot";
  control.appendChild(dot);
  control.appendChild(textNode(row.the_connection.label));
  if (row.the_connection.unavailable) {
    control.disabled = true;
    control.classList.add("dead");
  }
  if (row.the_connect_is_underway || pendingConnections.has(row.server)) {
    // **Design's *on the control*, and *Connect* is the rule's own example**:
    // the control keeps its place and its width, loses its label to the
    // turning mark, is not dimmed, and answers nothing until the connect is
    // made. It stays hollow, as the drawing's own `busy` state draws it — the
    // green is for a connection that is there. The application says when
    // (`TheUsersConnects` in `src/surface/servers.rs`); a repairing row never
    // carries this, since its control keeps its own state. Added 2026-09-27,
    // step 10c's eighth-pass fix piece 5.
    theMarkOnAControl(control, row.the_connection.label);
  } else if (row.the_connection_is_coming_back) {
    // **A server Smoozy is bringing back keeps its amber** — the drawing's
    // `trying`, which is state rather than an underway mark: `Design/Whole app
    // § Anything that isn't instant`, *"its Connect control keeps its amber"*.
    // The ninth pass drew it green, the connected colour. Added 2026-09-28,
    // step 10c's ninth-pass fix piece 2.
    control.classList.add("trying");
  } else if (theRowIsLive(row)) {
    control.classList.add("live");
  }
  control.addEventListener("click", (press) => {
    press.stopPropagation();
    if (control.disabled || control.getAttribute("aria-disabled") === "true" || pendingConnections.has(row.server)) return;
    theActIsTaken(row.the_connection.act, row.server, screen);
  });
  return control;
}

/// **The one failure shape, in its collapsed room.**
///
/// **Nothing here builds a failure.** The parts arrive on the failure shape's
/// own event and the builder is `web/the-failure-shape.js`'s. The mockup draws
/// a `.svfail` of its own on the row; that markup is not copied.
function theFailureOnTheRow(said) {
  return theFailureShape({
    sentence: said.sentence,
    whatToDoAboutIt: said.whatToDoAboutIt,
    kind: said.kind,
    subject: said.subject,
    subjectKind: said.subjectKind,
    identityOrMark: said.identityOrMark,
    key: said.key,
    theSentenceForTheReport: said.theSentenceForTheReport,
    at: said.at,
    values: [],
    controls: theControlsFrom(said),
    room: A_COLLAPSED_ROW,
    onClear() {
      shell.core.invoke("a_failure_was_cleared", {
        kind: said.kind,
        subjectKind: said.subjectKind,
        subject: said.subject,
        identityOrMark: said.identityOrMark,
      });
    },
  });
}

// ── The five menus ──────────────────────────────────────────────────────────
//
// **One panel, opened beside whichever row asked for it.** The drawing's own
// `.ctx` is the panel and `.hd`, `.off` and `.dg` are its three row states, all
// of them already in `the-drawing.css`. **`web/menus.js` is the application's
// own menu bar and is a different thing**; this is a row's context menu, which
// the drawing draws with the same classes and nothing else shares.

let openMenu = null;
let menuContext = null;
let menuRequest = 0;

function theMenuIsClosed(restore = false) {
  menuRequest += 1;
  const held = menuContext;
  if (held) {
    theMenusRow(held)?.setAttribute("aria-expanded", "false");
    document.removeEventListener("mousedown", held.away, true);
    document.removeEventListener("keydown", held.keys, true);
  }
  if (openMenu && openMenu.parentNode) openMenu.parentNode.removeChild(openMenu);
  openMenu = null;
  menuContext = null;
  if (restore && held) theMenusRow(held)?.focus();
}

function theMenusRow(held) {
  if (!held.rail) return rowElements.get(held.server) || held.beside;
  const rail = document.querySelector(".rail");
  const rows = rail ? rail.querySelectorAll("[data-server]") : [];
  return Array.from(rows).find((row) => row.dataset.server === held.server) || held.beside;
}

// The rail asks for the existing menu without importing a screen into the shell.
document.addEventListener("servers:menu-requested", async (press) => {
  theMenuIsClosed();
  const request = menuRequest;
  const { server, beside } = press.detail;
  try {
    const screen = await shell.core.invoke("the_all_servers_screen");
    if (request !== menuRequest || !beside.isConnected) return;
    lastDrawn = screen;
    if (screen.the_stop_and_clear_ending) {
      theOwedEndingIsShown(screen);
      return;
    }
    const row = screen.rows.find((one) => one.server === server);
    if (row) theMenuIsOpened(row, screen, beside, null, true);
  } catch (_) { theMenuIsClosed(); }
});

function theOpenMenuIsRefreshed(screen) {
  if (!menuContext) return;
  const held = menuContext;
  const row = screen.rows.find((one) => one.server === held.server);
  if (!row) return theMenuIsClosed();
  const focused = document.activeElement?.dataset.act;
  theMenuIsOpened(row, screen, theMenusRow(held), held.press, held.rail, focused);
}

function theMenuIsOpened(row, screen, beside, press, rail = false, focused = null) {
  theMenuIsClosed();
  const menu = screen.menus.find((one) => one.which === row.the_menu);
  if (!menu) return;
  const panel = document.createElement("div");
  panel.className = "ctx svctx";
  panel.setAttribute("role", "menu");
  const slots = [[menu.the_connection], menu.going_in, menu.about_itself, menu.the_details, [menu.removing]];
  let first = true;
  for (const slot of slots) {
    if (!slot.length) continue;
    if (!first) panel.appendChild(document.createElement("hr"));
    first = false;
    for (const item of slot) {
      const one = aButton(item, () => {
        theMenuIsClosed();
        theActIsTaken(item.act, row.server, screen);
      });
      one.setAttribute("role", "menuitem");
      one.dataset.act = item.act;
      one.tabIndex = -1;
      if ((pendingConnections.has(row.server) || row.the_connect_is_underway) && ["connect", "connect-and-set-up", "drop-the-setup"].includes(item.act)) one.disabled = true;
      if (item.unavailable) one.className = "off";
      if (item.act === "remove") one.className = "dg";
      panel.appendChild(one);
    }
  }
  (rail ? document.body : host).appendChild(panel);
  const anchor = beside.getBoundingClientRect();
  const room = panel.getBoundingClientRect();
  const wanted = rail ? anchor.right : press ? press.clientX : anchor.left;
  const below = rail ? anchor.top : press ? press.clientY : anchor.bottom;
  const left = Math.max(8, Math.min(Math.round(wanted), window.innerWidth - room.width - 8));
  const top = Math.max(8, Math.min(Math.round(below), window.innerHeight - room.height - 8));
  panel.style.setProperty("--svctx-x", `${left}px`);
  panel.style.setProperty("--svctx-y", `${top}px`);
  openMenu = panel;
  beside.setAttribute("aria-expanded", "true");
  const away = (elsewhere) => {
    if (!panel.contains(elsewhere.target)) theMenuIsClosed(true);
  };
  const keys = (key) => {
    const items = Array.from(panel.querySelectorAll('[role="menuitem"]')).filter((one) => !one.disabled);
    if (key.key === "Escape" || key.key === "Tab") {
      key.preventDefault(); key.stopPropagation(); theMenuIsClosed(true);
    } else if (["ArrowDown", "ArrowUp", "Home", "End"].includes(key.key)) {
      key.preventDefault(); key.stopPropagation();
      const at = items.indexOf(document.activeElement);
      const next = key.key === "Home" ? 0 : key.key === "End" ? items.length - 1 : (at + (key.key === "ArrowUp" ? -1 : 1) + items.length) % items.length;
      items[next]?.focus();
    } else if (key.key === "Enter" || key.key === " ") {
      key.preventDefault(); key.stopPropagation();
      if (items.includes(document.activeElement)) document.activeElement.click();
    }
  };
  menuContext = { server: row.server, beside, press, rail, away, keys };
  document.addEventListener("mousedown", away, true);
  document.addEventListener("keydown", keys, true);
  const items = Array.from(panel.querySelectorAll('[role="menuitem"]')).filter((one) => !one.disabled);
  (items.find((one) => one.dataset.act === focused) || items[0])?.focus();
}

// The pane takes the list's complete, key-checked reading and its actual control.
const connectionPlaces = new Map();
export function theConnectionIsDrawnInto(holder, note, server) {
  if (!server) { connectionPlaces.delete(holder); holder.replaceChildren(); note.replaceChildren(); note.hidden = true; return; }
  const previous = connectionPlaces.get(holder);
  connectionPlaces.set(holder, { note, server });
  if (lastDrawn) theSharedControlsAreDrawn(lastDrawn);
  if (!previous || previous.server !== server) theScreenIsAskedFor();
}

function theSharedControlsAreDrawn(screen) {
  for (const [holder, { note, server }] of connectionPlaces) {
    const row = screen.rows.find((one) => one.server === server);
    holder.replaceChildren(); note.replaceChildren();
    if (!row) { note.hidden = true; continue; }
    const control = theConnectControl(row, screen);
    if (row.the_key_mark) control.title = row.the_key_mark.lead;
    if (screen.the_tools_statement) control.title = screen.the_tools_statement.lead;
    holder.appendChild(control);
    const failures = failuresByServer.get(server);
    if (failures?.size) {
      for (const failure of failures.values()) note.appendChild(theFailureOnTheRow(failure));
    } else if (theConnectsNotMade.has(server)) {
      note.textContent = theConnectsNotMade.get(server).said;
    } else if (screen.the_tools_statement) {
      note.textContent = screen.the_tools_statement.lead;
    } else if (row.the_key_mark) saidInto(note, row.the_key_mark);
    else if (row.the_setup_line) saidInto(note, row.the_setup_line);
    if (row.the_next_attempt) {
      const next = row.the_next_attempt;
      note.appendChild(theCountdown(next.due_at, { in: next.trying_again_in, now: next.trying_again_now }));
    }
    note.hidden = !note.childNodes.length;
  }
}

// ── What every act does ─────────────────────────────────────────────────────

function theActIsTaken(act, server, screen) {
  if (act === "rename") {
    if (theCurrentPlace() === ALL_SERVERS) return theRenameIsStarted(server);
    theCurrentPlaceIs(ALL_SERVERS);
    return theScreenIsAskedFor().then(() => theRenameIsStarted(server));
  }
  // **Any other act lets a name being typed go**, unsaved.
  theRenameIsLetGo();
  if (act === "edit") return theEditIsOpened(server);
  if (act === "remove") return theRemovalIsOpened(server);
  if (act === "copy-address") return theAddressIsCopied(server);
  if (act === "browse" || act === "set-up") {
    if (act === "set-up") theServerWillOpenOn(server, SET_UP);
    return theServerIsOpened(server);
  }
  // **The two ways out of a gone key are the page's own** — step 10c's piece
  // 4, part 2. Neither reaches a machine.
  if (act === "a-different-key") return aDifferentKeyIsChosen(server);
  if (act === "fix-the-key") return theKeyIsFixedInKeys(server);
  // **Everything else goes to the application**: connect, disconnect,
  // *Connect and set up* and *Connect without setup* (step 10c's piece 4,
  // part 1). An act it does not route is answered
  // rather than met with silence — and **the screen is asked for again once
  // the act is taken**, so an act that answers nothing (a second *Connect* on
  // a server already connecting, a *Disconnect* with no period to end) still
  // leaves the row drawn from what is true (step 10b's first audit round, item
  // 14).
  if (THE_ACTS_THAT_CONNECT.includes(act)) theConnectionFailureIsTakenAway(server);
  if (pendingConnections.has(server) || (screen?.rows.find((row) => row.server === server)?.the_connect_is_underway && THE_ACTS_THAT_CONNECT.includes(act))) return;
  pendingConnections.add(server);
  if (lastDrawn) theSharedControlsAreDrawn(lastDrawn);
  shell.core
    .invoke("an_act_that_needs_a_connection", { act, server })
    .finally(() => {
      pendingConnections.delete(server);
      theScreenIsAskedFor();
    }).catch(() => {});
  void screen;
}

/// ***Change key…*** — `Specs/Servers`, *When a server's key is
/// gone or unusable*. **The Keys screen in its picking mode, for this server**,
/// handed the context the Edit dialog hands it (`Keys` U11), and **the key
/// picked is saved through the command Edit saves with, carrying the saved
/// details unchanged** — the way a rename saves the name alone. So the save is
/// Edit's own, with `Servers` E1's ending and put-back for a connected server
/// and **`Servers` P15: the mark
/// clears at the list's next drawing**, which the save's answer asks for.
function aDifferentKeyIsChosen(server) {
  if (!server) return;
  shell.core
    .invoke("the_edit_dialog", { server })
    .then((said) => {
      aServerDialogPicksAKey({ label: said.the_name, username: "", host: "" }, (chosen) => {
        // **A cancel saves nothing**; a cancel caused by the user going
        // elsewhere leaves them where they went.
        if (theCurrentPlace() === KEYS) theCurrentPlaceIs(ALL_SERVERS);
        if (!chosen) return;
        // **`Keys` U12: the key is read from its row again**, so a key that
        // went while the user was away saves nothing.
        shell.core.invoke("the_key_a_dialog_holds", { key: chosen.row }).then((held) => {
          if (!held || !held.key) return;
          theConnectionFailureIsTakenAway(server);
          shell.core
            .invoke("an_edit_is_saved", {
              server,
              host: said.the_host,
              port: said.the_port,
              username: said.the_username,
              keyIdentifier: held.key,
              label: said.the_name,
            })
            .catch(() => {});
        });
      });
    })
    // **A server that has just gone is said where the user pressed.**
    .catch((failure) => theActWasNotTaken(failure || {}));
}

/// ***Fix in Keys…*** — `Specs/Servers`, *When a server's key is gone
/// or unusable*: the Keys screen, **at the key this server uses** — its row
/// takes the keyboard once the screen has drawn it.
/// **`Servers` P15: the mark clears the next time the list is
/// drawn**, however the key was mended there.
function theKeyIsFixedInKeys(server) {
  if (!server) return;
  shell.core
    .invoke("the_edit_dialog", { server })
    .then((said) => {
      theCurrentPlaceIs(KEYS);
      theKeysRowTakesTheKeyboard(said.the_key_identifier);
    })
    .catch((failure) => theActWasNotTaken(failure || {}));
}

/// **The Keys screen draws its rows when the application answers it**, so the
/// row is waited for — **with no timer** (`Desktop app and delivery` P9 and
/// P10): the watch ends when the row is chosen, when the Keys screen has drawn
/// rows without it (a key no longer in Smoozy leaves the user on Keys with
/// nothing chosen), or when the user has gone elsewhere.
function theKeysRowTakesTheKeyboard(key) {
  if (!key) return;
  const which = `.ky-row[data-row="${CSS.escape(key)}"]`;
  const theWatchIsOver = () => {
    if (theCurrentPlace() !== KEYS) return true;
    const row = document.querySelector(which);
    if (!row) return document.querySelector(".ky-row") !== null;
    row.focus();
    row.scrollIntoView({ block: "nearest" });
    return true;
  };
  if (theWatchIsOver()) return;
  const watching = new MutationObserver(() => {
    if (theWatchIsOver()) watching.disconnect();
  });
  watching.observe(document.body, { childList: true, subtree: true });
}

function theServerIsOpened(server) {
  if (!server) return;
  // **The contract's section 5 is the only way to go anywhere.** A server is a
  // place you go into, and the screen behind that place is piece 5's.
  theCurrentPlaceIs(A_SERVER, server);
}

function theAddressIsCopied(server) {
  const row = lastDrawn && lastDrawn.rows.find((one) => one.server === server);
  if (!row) return;
  // **The address is a value and the clipboard takes it exactly as Smoozy holds
  // it.** Nothing is composed and nothing is substituted.
  if (navigator.clipboard) navigator.clipboard.writeText(row.address).catch(() => {});
}

// ── The empty list ──────────────────────────────────────────────────────────

function theGuidanceIsDrawn(guidance) {
  empty.textContent = "";
  const box = document.createElement("div");
  box.className = "emptybox";
  const heading = document.createElement("h2");
  heading.textContent = guidance.heading;
  box.appendChild(heading);
  const how = document.createElement("p");
  how.className = "es";
  how.textContent = guidance.how_it_goes;
  box.appendChild(how);

  const steps = document.createElement("ol");
  steps.className = "esteps";
  for (const step of guidance.steps) {
    const one = document.createElement("li");
    const lead = document.createElement("b");
    lead.textContent = step.lead;
    one.appendChild(lead);
    // **Each string is exactly as it was handed in** (X2), so the separator
    // between the two is part of whichever string it belongs to and nothing is
    // joined here.
    if (step.rest) one.appendChild(textNode(step.rest));
    if (step.opens_keys) {
      one.appendChild(textNode(" "));
      const opens = document.createElement("button");
      opens.type = "button";
      opens.className = "lnkb";
      opens.textContent = guidance.open_keys;
      opens.addEventListener("click", () => theCurrentPlaceIs(KEYS));
      one.appendChild(opens);
    }
    steps.appendChild(one);
  }
  box.appendChild(steps);

  const add = document.createElement("button");
  add.type = "button";
  add.className = "btn";
  add.textContent = guidance.add_a_server;
  add.addEventListener("click", () => theAddDialogIsOpened());
  box.appendChild(add);
  empty.appendChild(box);
}

// ── The dialogs ─────────────────────────────────────────────────────────────
//
// **They behave as dialogs** (step 10a round 1, fix-here item 27): each is
// marked a modal dialog, and Escape closes it — `Design/Whole app § Keyboard
// shortcuts`, *"Escape closes the thing on top, one layer at a time."*

// Where the answer to the last act is drawn, while a dialog is open.
let refusals = null;

function theDialogsAreReady(into = null) {
  if (!dialogs) {
    dialogs = document.createElement("div");
    dialogs.className = "svdialogs";
  }
  const parent = into || (host && theCurrentPlace() === ALL_SERVERS ? host : document.body);
  if (dialogs.parentNode !== parent) parent.appendChild(dialogs);
}

function aDialog() {
  theDialogsAreReady();
  dialogs.textContent = "";
  refusals = null;
  const modal = document.createElement("div");
  modal.className = "modal";
  const box = document.createElement("div");
  box.className = "mbox";
  box.setAttribute("role", "dialog");
  box.setAttribute("aria-modal", "true");
  box.tabIndex = -1;
  box.addEventListener("keydown", (press) => {
    if (press.key !== "Escape" || press.defaultPrevented) return;
    press.preventDefault();
    theDialogIsClosed();
  });
  modal.appendChild(box);
  dialogs.appendChild(modal);
  return box;
}

/// **A pasted text that is a connection line begins with the word and a
/// space** — `Servers` PL1. Space before it is not counted, as the application
/// does not count it.
const A_CONNECTION_LINE = /^\s*ssh\s/;

/// **The heading names the box for anyone who cannot see it**, and focus goes
/// into the box so Escape reaches it.
function theDialogIsLabelled(box, heading) {
  heading.id = "svdialog-heading";
  box.setAttribute("aria-labelledby", heading.id);
  box.focus();
}

function theDialogIsClosed() {
  if (!dialogs) return;
  dialogs.textContent = "";
  refusals = null;
}

function aField(frm, label, value, wide) {
  const named = document.createElement("label");
  named.textContent = label;
  frm.appendChild(named);
  const field = document.createElement("input");
  field.type = "text";
  field.className = wide ? "fld wide mono" : "fld w-frame mono";
  field.value = value || "";
  frm.appendChild(field);
  return field;
}

/// **The port reads from the left, like its neighbours, at the number field's
/// width** — the drawing's `sv-add` and `sv-edit` (P2, the Creator's note 2 on
/// 0.1.7). `.w-frame` keeps its right alignment for every other number field,
/// so the port carries a class of its own beside it (`web/servers.css` §1).
const THE_PORTS_OWN_CLASS = "svport";

/// **The port field puts 22 back when it is cleared** — `Specs/Servers`:
/// *"clearing it puts 22 back rather than leaving it empty"* (step 10a round 1,
/// fix-here item 28).
function thePortPutsTheStandardBack(port, standard) {
  port.addEventListener("change", () => {
    if (!port.value.trim()) port.value = standard;
  });
}

/// **A refusal under its own field.**
///
/// The empty first cell is what insets the line, which is the mockup's own
/// arrangement — and **it is hidden with the line rather than left standing**,
/// because a grid cell that stays behind pushes every label after it into the
/// wrong column.
function aRefusalUnder(frm) {
  const inset = document.createElement("span");
  inset.hidden = true;
  frm.appendChild(inset);
  const said = document.createElement("p");
  said.className = "alertnote";
  said.hidden = true;
  frm.appendChild(said);
  said.theInset = inset;
  return said;
}

/// Show or hide a refusal, with the cell that insets it.
function theRefusalReads(said, words) {
  if (!said) return;
  said.textContent = words || "";
  said.hidden = !words;
  if (said.theInset) said.theInset.hidden = !words;
}

/// **The line inside an open dialog where an act that did not land is said**
/// (step 10a round 1, fix-here item 12) — never behind the dialog.
function aNoticeIn(box) {
  const notice = document.createElement("p");
  notice.className = "mwarn";
  notice.hidden = true;
  box.appendChild(notice);
  return notice;
}

/// **The key control a server dialog carries: Smoozy's own dropdown of the
/// usable keys** — `Design/Keys § Opened from a server dialog`, the drawing's
/// `sv-add`, `sv-addkey` and `sv-edit`; the Creator's eighth notes, note 3,
/// piece E3. Until 2026-10-16 the field was a button that opened the Keys
/// screen alone, and the user left the dialog to pick.
///
/// - **Each key by name, with its fingerprint's start beside it** in the
///   fixed-width face, because two keys can share a name. **A pick fills the
///   field and the dialog stays**; the pair is still read from the key row
///   (`Keys` U12), so what the field holds is never one the page made.
/// - **`Keys` U10a: the list draws first and each verdict fills in behind
///   it.** A key still being checked is listed and cannot be picked until it
///   reads usable; one that reads unusable leaves the list. A key the tools
///   could not check (`Keys` U6a) stays, cannot be picked, and stops saying
///   *Checking…*, because nothing is checking it — the Keys screen says why.
/// - **Its last row, *Open Keys…*, opens the Keys screen in its picking mode**
///   exactly as the field itself did before (`Keys` U11). *New key…* beside it
///   is unchanged.
/// - **The Edit dialog's own unusable key stays at the top with the gone
///   mark**, picked and never pickable again (`Design/Servers § The Edit
///   dialog`). `held.gone` is that mark, from the application, where it is
///   gone.
/// - **Render Settings' own dropdown is the pattern** (`web/render-settings.js`,
///   piece U): Up and Down move, Return picks, Escape closes the list before
///   anything else, a press anywhere else closes it, the picked row in the
///   menu's bold.
///
/// `context` is what the dialog hands Keys to show, as text.
function aKeyControl(frm, said, held, context) {
  const keyLabel = document.createElement("label");
  keyLabel.textContent = said.key;
  frm.appendChild(keyLabel);
  const krow = document.createElement("span");
  krow.className = "krow";
  const pick = document.createElement("span");
  pick.className = "pick svkeyfield";
  const face = document.createElement("button");
  face.type = "button";
  face.className = "pkb";
  face.setAttribute("aria-label", said.key);
  face.setAttribute("aria-haspopup", "listbox");
  face.setAttribute("aria-expanded", "false");
  pick.appendChild(face);
  const list = document.createElement("span");
  list.className = "ctx ddl";
  list.setAttribute("role", "listbox");
  list.setAttribute("aria-label", said.key);
  list.hidden = true;
  pick.appendChild(list);

  const shown = { key: held.key, fingerprint: held.fingerprint };
  const theGoneKey = held.gone ? held.key : null;
  // key → { button, offer, verdict, gone } for every row the list draws.
  const rows = new Map();
  let offered = null;
  let opens = null;
  let ask = null;

  const theRowsAreMarked = () => {
    for (const [key, row] of rows) {
      const picked = key === shown.key;
      row.button.classList.toggle("hd", picked);
      row.button.setAttribute("aria-selected", picked ? "true" : "false");
    }
  };
  const theControlReads = (showsAs) => {
    face.textContent = "";
    if (showsAs) {
      face.appendChild(theValueDrawn(showsAs));
      if (theGoneKey && shown.key === theGoneKey) face.appendChild(document.createTextNode(held.gone));
    } else face.textContent = said.choose_a_key;
    theRowsAreMarked();
  };

  // **A row reads its name, then its fingerprint's start, or *Checking…*
  // while its verdict has not arrived.** Only a usable key may be picked.
  const theRowReads = (row) => {
    const { button, offer, verdict } = row;
    button.textContent = "";
    button.appendChild(theValueDrawn(offer.shows_as));
    const beside = document.createElement("span");
    if (verdict === null) {
      beside.className = "ky-fp";
      beside.textContent = offered.checking;
    } else if (offer.fingerprint_start) {
      beside.className = "ky-fp mono";
      beside.appendChild(theValueDrawn(offer.fingerprint_start));
      if (offer.it_runs_on) beside.appendChild(document.createTextNode(offered.the_fingerprint_runs_on));
    }
    if (beside.className) {
      button.appendChild(document.createTextNode("\u00A0\u00A0"));
      button.appendChild(beside);
    }
    const usable = verdict === "usable";
    button.classList.toggle("off", !usable);
    button.setAttribute("aria-disabled", usable ? "false" : "true");
  };
  const anOption = () => {
    const one = document.createElement("button");
    one.type = "button";
    one.setAttribute("role", "option");
    one.addEventListener("click", () => chosen(one));
    return one;
  };
  const theListIsDrawn = () => {
    list.textContent = "";
    rows.clear();
    if (theGoneKey) {
      const top = anOption();
      top.theKey = theGoneKey;
      top.appendChild(theValueDrawn(held.showsAs));
      top.appendChild(document.createTextNode(held.gone));
      top.classList.add("off");
      top.setAttribute("aria-disabled", "true");
      list.appendChild(top);
      rows.set(theGoneKey, { button: top, gone: true });
    }
    for (const offer of offered.keys) {
      if (offer.key === theGoneKey) continue;
      const row = { button: anOption(), offer, verdict: null };
      row.button.theKey = offer.key;
      theRowReads(row);
      list.appendChild(row.button);
      rows.set(offer.key, row);
    }
    list.appendChild(document.createElement("hr"));
    opens = document.createElement("button");
    opens.type = "button";
    opens.textContent = offered.open_keys;
    opens.addEventListener("click", () => chosen(opens));
    list.appendChild(opens);
    theRowsAreMarked();
  };

  // **One ask per appearance of the list** (`Keys` U10a): when the dialog is
  // drawn, and again when it is shown after Keys. A verdict for an ask this
  // control has moved past is dropped, and so is one for a key no longer on it.
  const theVerdictIsTold = (told) => {
    if (told.ask !== ask) return;
    const row = rows.get(told.key);
    if (!row || row.gone) return;
    if (told.verdict === "unusable") {
      if (row.button.parentNode) row.button.parentNode.removeChild(row.button);
      rows.delete(told.key);
      return;
    }
    row.verdict = told.verdict;
    theRowReads(row);
  };
  const theListIsAskedFor = () => {
    theLastKeyAsk += 1;
    const mine = String(theLastKeyAsk);
    ask = mine;
    theKeyListIsTold = theVerdictIsTold;
    theKeyListGrew = (told) => {
      if (told.ask === ask) theListIsAskedFor();
    };
    shell.core
      .invoke("the_keys_a_dialog_offers")
      .then((answer) => {
        if (ask !== mine || !answer) return;
        offered = answer;
        theListIsDrawn();
        return shell.core.invoke("the_keys_a_dialog_offers_are_checked", {
          ask: mine,
          keys: answer.keys.map((offer) => offer.key),
        });
      })
      .catch(() => {});
  };

  // **The rows the keys move between**: every row that can be picked, and
  // *Open Keys…*.
  const theRowsTheKeysReach = () =>
    Array.from(rows.values(), (row) => row.button)
      .concat(opens ? [opens] : [])
      .filter((one) => !one.classList.contains("off"));
  const shut = () => {
    list.hidden = true;
    face.setAttribute("aria-expanded", "false");
    if (theOpenKeyList && theOpenKeyList.pick === pick) theOpenKeyList = null;
  };
  const opened = (byKey) => {
    if (!offered) return;
    if (theOpenKeyList) theOpenKeyList.shut();
    list.hidden = false;
    face.setAttribute("aria-expanded", "true");
    theOpenKeyList = { pick, shut };
    // **The keys reach the list however it was opened**: from the keyboard
    // the picked row takes the focus, from the mouse the control does.
    const reached = theRowsTheKeysReach();
    const first = reached.find((one) => one.classList.contains("hd")) || reached[0];
    if (byKey && first) first.focus();
    else face.focus();
  };
  // **A pick closes the list, and the focus goes back to the control**, so
  // Escape still reaches the dialog.
  function chosen(button) {
    if (button.classList.contains("off")) return;
    shut();
    face.focus();
    if (button === opens) {
      theKeysScreenIsOpened(false);
      return;
    }
    if (button.theKey !== shown.key) theKeyIsReadAgain(shown, button.theKey, theControlReads);
  }
  face.addEventListener("click", (press) => {
    if (list.hidden) opened(press.detail === 0);
    else shut();
  });
  pick.addEventListener("keydown", (press) => {
    const open = !list.hidden;
    if (press.key === "Escape" && open) {
      press.preventDefault();
      shut();
      face.focus();
    } else if (press.key === "ArrowDown" || press.key === "ArrowUp") {
      press.preventDefault();
      if (!open) {
        opened(true);
        return;
      }
      const reached = theRowsTheKeysReach();
      const at = reached.indexOf(document.activeElement);
      const next = at < 0 ? 0 : at + (press.key === "ArrowDown" ? 1 : -1);
      if (reached[next]) reached[next].focus();
    } else if (press.key === "Enter" && open) {
      const at = theRowsTheKeysReach().indexOf(document.activeElement);
      if (at < 0) return;
      press.preventDefault();
      chosen(document.activeElement);
    } else if (press.key === "Tab" && open) {
      shut();
    }
  });

  const theKeysScreenIsOpened = (withTheNewKeyDialog) => {
    aServerDialogPicksAKey(context(), (chosenThere) => {
      // **A cancel hands back nothing and the dialog stays as it was**; a
      // cancel caused by the user going elsewhere leaves them where they went.
      if (theCurrentPlace() === KEYS) theCurrentPlaceIs(ALL_SERVERS);
      // **The list is asked for again**: a key may have been made, pointed at
      // or taken away while the user was on Keys.
      theListIsAskedFor();
      if (!chosenThere) return;
      theKeyIsReadAgain(shown, chosenThere.row, theControlReads);
    }, withTheNewKeyDialog);
  };
  const newKey = document.createElement("button");
  newKey.type = "button";
  newKey.className = "lnkb";
  newKey.textContent = said.new_key;
  // ***New key…* opens Keys in its picking mode with the New key dialog
  // already showing** — the link names that act, so it does it (the Creator's
  // note 1 on 0.1.7). A key made there is handed back by its row's *Use this
  // key*, as any other.
  newKey.addEventListener("click", () => theKeysScreenIsOpened(true));
  krow.appendChild(pick);
  krow.appendChild(newKey);
  frm.appendChild(krow);
  theControlReads(held.showsAs);
  theListIsAskedFor();
  shown.control = krow;
  shown.theControlReads = theControlReads;
  return shown;
}

// **The one key list open, so a press anywhere else closes it** — as Render
// Settings' own dropdown does.
let theOpenKeyList = null;
// **The key list the verdicts are for**, the last one asked for, and the count
// each ask is told apart by.
let theKeyListIsTold = null;
let theKeyListGrew = null;
let theLastKeyAsk = 0;

document.addEventListener(
  "pointerdown",
  (press) => {
    if (theOpenKeyList && !theOpenKeyList.pick.contains(press.target)) theOpenKeyList.shut();
  },
  true,
);

whenTold(A_DIALOG_KEYS_VERDICT, (delivery) => {
  if (theKeyListIsTold) theKeyListIsTold(delivery.said);
});

whenTold(A_DIALOG_KEYS_LIST_GREW, (delivery) => {
  if (theKeyListGrew) theKeyListGrew(delivery.said);
});

/// **The pair, taken from the key row again** — `Keys` U12: a key that went
/// while the user was away leaves the field empty.
function theKeyIsReadAgain(shown, key, theControlReads) {
  shell.core.invoke("the_key_a_dialog_holds", { key }).then((held) => {
    shown.key = held ? held.key : "";
    shown.fingerprint = held ? held.fingerprint : "";
    theControlReads(held ? held.shows_as : "");
    // Dropdown, Open Keys and New key all return through this read. Clear
    // the required-key line only once the application has supplied a key.
    if (held && shown.whenChosen) shown.whenChosen();
  });
}

/// **The Add dialog** — `sv-add`, and its three refusal readings `sv-blank`,
/// `sv-port` and `sv-dupname`.
function theAddDialogIsOpened() {
  shell.core.invoke("the_add_dialog").then((said) => {
    const box = aDialog();
    const heading = document.createElement("h3");
    heading.textContent = said.heading;
    box.appendChild(heading);

    // **The guidance stays reachable from here**, which is the one place it is
    // wanted a second time — settled at Design, 2026-08-23.
    const again = document.createElement("p");
    again.className = "mnote2 svagain";
    const opens = document.createElement("button");
    opens.type = "button";
    opens.className = "lnkb";
    opens.textContent = said.the_guidance_again;
    opens.addEventListener("click", () => {
      theDialogIsClosed();
      if (lastDrawn) {
        theGuidanceIsDrawn(lastDrawn.guidance);
        empty.hidden = false;
      }
    });
    again.appendChild(opens);
    box.appendChild(again);

    const frm = document.createElement("div");
    frm.className = "frm";
    const host = aField(frm, said.host, "", true);
    host.placeholder = said.host_placeholder;
    // **The line under Host for a pasted connection line**, in the form's grey
    // and never in red, since nothing the user typed is wrong — `sv-paste`.
    // It is drawn as a refusal is, in the grey's class.
    const pasted = aRefusalUnder(frm);
    pasted.className = "mnote2 svpasted";
    const hostRefusal = aRefusalUnder(frm);
    const port = aField(frm, said.port, said.the_port_starts_at, false);
    port.classList.add(THE_PORTS_OWN_CLASS);
    thePortPutsTheStandardBack(port, said.the_port_starts_at);
    const portRefusal = aRefusalUnder(frm);
    const username = aField(frm, said.username, "", true);
    const usernameRefusal = aRefusalUnder(frm);

    // **`Keys` U11: the Add dialog hands over its own fields as they stand**,
    // any or all of which may be empty.
    let name = null;
    const key = aKeyControl(frm, said, { key: "", fingerprint: "", showsAs: "" }, () => ({
      label: name ? name.value : "",
      username: username.value,
      host: host.value,
    }));
    const keyRefusal = aRefusalUnder(frm);
    key.whenChosen = () => theRefusalReads(keyRefusal, "");

    const nameLabel = document.createElement("label");
    nameLabel.textContent = said.name;
    frm.appendChild(nameLabel);
    const nameHolder = document.createElement("span");
    nameHolder.className = "fld wide svnamefield";
    name = document.createElement("input");
    name.type = "text";
    nameHolder.appendChild(name);
    const optional = document.createElement("em");
    optional.textContent = said.optional;
    nameHolder.appendChild(optional);
    // **The whole drawn field takes the press, not only the part the input
    // fills.** The drawing draws Name as one field the width of the form with
    // *optional* inside it, so the box is a span in the field's shape and the
    // input sits at its left, a third of its width. **A press anywhere else in
    // the drawn field landed on the span**, which cannot take focus, so the
    // focus went to the dialog itself and it lit up — the second pass on a real
    // rental, 2026-09-21 (fault 7), and sitting 1 before it. A page harness
    // against this file measured it: the field 384 pixels wide, the input 150.
    // **So a press on the field that is not on the input is handed to the
    // input**, and the caret goes where typing would land.
    nameHolder.addEventListener("mousedown", (press) => {
      if (press.target === name) return;
      press.preventDefault();
      name.focus();
    });
    frm.appendChild(nameHolder);

    const colourLabel = document.createElement("label");
    colourLabel.textContent = said.colour;
    frm.appendChild(colourLabel);
    const group = document.createElement("span");
    group.className = "swgroup";
    const swatches = document.createElement("span");
    swatches.className = "swatches";
    let picked = said.picked_for_you;
    for (const badge of said.the_eight) {
      const one = document.createElement("button");
      one.type = "button";
      one.classList.add("sw", badge);
      if (said.already_used.includes(badge)) one.classList.add("used");
      if (badge === picked) one.classList.add("on");
      one.addEventListener("click", () => {
        picked = badge;
        for (const other of swatches.children) other.classList.remove("on");
        one.classList.add("on");
      });
      swatches.appendChild(one);
    }
    group.appendChild(swatches);
    const note = document.createElement("em");
    note.className = "swnote";
    note.textContent = said.the_swatch_note;
    group.appendChild(note);
    frm.appendChild(group);
    box.appendChild(frm);

    // **The two checks, and they are two statements that must not read alike.**
    const duplicate = document.createElement("p");
    duplicate.className = "mwarn";
    duplicate.hidden = true;
    box.appendChild(duplicate);
    const sameName = document.createElement("p");
    sameName.className = "mwarn";
    sameName.hidden = true;
    box.appendChild(sameName);

    const check = () =>
      shell.core.invoke("the_add_is_checked", {
        host: host.value,
        // **The port travels as the text the user typed.** A number here would
        // refuse anything outside a whole number's own range before the
        // application saw it, and the user would meet silence where Design
        // requires a refusal in place.
        port: port.value,
        label: name.value,
      });
    // **The port refuses *in place***, which is Design's own word for it, so
    // the checks run as the value changes rather than only once the field is
    // left. **The blank-field refusal is the opposite and waits for Add** —
    // *"A dialog that turned red as the user tabbed through it would be
    // scolding them for not having typed yet."*
    for (const field of [host, port, name]) {
      field.addEventListener("input", check);
      field.addEventListener("change", check);
    }

    // **The provider's connection line, pasted into Host** — `Design/Servers §
    // Adding a server`, `Servers` §13.3 (the Creator's second notes, piece
    // 15). A paste that starts with the word and a space is handed to the
    // application, which reads it as text and never runs it; the three fields
    // take what it says and the dialog's own checks run on them as on typed
    // values. Anything else pasted is a host, as before. **Nothing is saved
    // until *Add***. The line under Host says what the line lacks, or why it
    // filled nothing, and typing in Host takes it away.
    host.addEventListener("paste", (press) => {
      const line = press.clipboardData ? press.clipboardData.getData("text/plain") : "";
      if (!A_CONNECTION_LINE.test(line)) return;
      press.preventDefault();
      shell.core.invoke("the_pasted_line_is_read", { line }).then((read) => {
        if (!read) return;
        if (read.fills) {
          host.value = read.host;
          port.value = read.port;
          username.value = read.username;
          check();
        }
        theRefusalReads(pasted, read.the_line_under.join(" "));
      });
    });
    host.addEventListener("input", () => theRefusalReads(pasted, ""));

    const togs = document.createElement("div");
    togs.className = "togs2";
    const ready = aTickBox(said.get_ready);
    const blender = aTickBox(said.install_blender);
    togs.appendChild(ready.box);
    togs.appendChild(blender.box);
    box.appendChild(togs);
    const ticking = document.createElement("p");
    ticking.className = "mnote2";
    ticking.textContent = said.ticking_either;
    box.appendChild(ticking);

    // **`Servers` W40's statement, made where the machine is added**, which is
    // the nearest moment Smoozy has to the renting.
    const rental = document.createElement("p");
    rental.className = "mnote2 svbills";
    rental.textContent = said.runs_and_bills;
    box.appendChild(rental);

    const notice = aNoticeIn(box);

    const row = document.createElement("div");
    row.className = "mrow";
    const grow = document.createElement("span");
    grow.className = "grow";
    row.appendChild(grow);
    const cancel = document.createElement("button");
    cancel.type = "button";
    cancel.className = "btn quiet";
    cancel.textContent = said.cancel;
    cancel.addEventListener("click", theDialogIsClosed);
    row.appendChild(cancel);
    const add = document.createElement("button");
    add.type = "button";
    add.className = "btn";
    add.textContent = said.add;
    const theLabel = () => {
      add.textContent = ready.on() || blender.on() ? said.add_and_set_up : said.add;
    };
    ready.box.addEventListener("click", theLabel);
    blender.box.addEventListener("click", theLabel);
    add.addEventListener("click", () => {
      shell.core.invoke("a_server_is_added", {
        host: host.value,
        // **The port travels as the text the user typed.** A number here would
        // refuse anything outside a whole number's own range before the
        // application saw it, and the user would meet silence where Design
        // requires a refusal in place.
        port: port.value,
        username: username.value,
        // **The fingerprint is not sent and must not be**: it is `Keys`' and
        // the application reads it off the key row this identifier names.
        keyIdentifier: key.key,
        label: name.value,
        colour: picked,
        // **Each tick on its own** — `Servers` B1 records which (step 10a
        // round 1, fix-here item 13).
        getReady: ready.on(),
        installBlender: blender.on(),
      });
    });
    row.appendChild(add);
    box.appendChild(row);

    theRefusalsAreDrawnInto({
      host: hostRefusal,
      port: portRefusal,
      username: usernameRefusal,
      key: keyRefusal,
      duplicate,
      sameName,
      notice,
      fields: { host, port, username },
    });
    theDialogIsLabelled(box, heading);
    // **The caret starts in Host, the first field, so the user types at
    // once** — `Design/Whole app` (the Creator's third notes, note 1). Escape
    // still closes the box: the key reaches the box from the field.
    host.focus();
  });
}

function aTickBox(label) {
  const box = document.createElement("span");
  box.className = "box plain";
  box.setAttribute("role", "checkbox");
  box.setAttribute("aria-checked", "false");
  box.tabIndex = 0;
  const tick = document.createElement("i");
  tick.textContent = "✓";
  box.appendChild(tick);
  box.appendChild(textNode(label));
  const toggle = () => {
    box.classList.toggle("on");
    box.setAttribute("aria-checked", String(box.classList.contains("on")));
  };
  box.addEventListener("click", toggle);
  return { box, on: () => box.classList.contains("on") };
}

function theRefusalsAreDrawnInto(where) {
  refusals = where;
}

// ── Rename, in place on the row ─────────────────────────────────────────────
//
// **`Design/Servers § Actions on a row`** (the Creator's note 1 on 0.1.8):
// *Rename…* changes the name in place, on the row, as a rename does in Files.
// Return saves and Escape cancels; an emptied name clears it, and the row then
// shows `username@host`. A name another server has gets Add's duplicate-name
// line under the field, which never blocks. **It saves the name alone, the way
// Edit saves it** — the command Edit saves with, carrying the saved host, port,
// username and key exactly as the application read them, so only the name
// moves and none of Edit's warnings is raised (`Servers` W37).

/// **The row being renamed**, or nothing: which server, the saved details as
/// the application read them, the name it had, the name typed, the field and
/// the line under it, and whether the save has been sent.
let renaming = null;

/// *Rename…*: the saved details are read, and the row's name becomes a field
/// holding it, focused and selected so typing replaces it.
function theRenameIsStarted(server) {
  if (renaming && renaming.server !== server) theRenameIsLetGo();
  shell.core
    .invoke("the_edit_dialog", { server })
    .then(async (said) => {
      if (!said) return;
      renaming = {
        server,
        said,
        was: said.the_name,
        typed: said.the_name,
        sameName: null,
        field: null,
        line: null,
        sent: false,
      };
      await theRowIsDrawnAgain(server);
      theNameFieldTakesTheKeys(true);
    })
    // **A server that has just gone is said where the user pressed**, as Edit
    // says it.
    .catch((failure) => theActWasNotTaken(failure || {}));
}

/// **The field, made once for the rename and kept across redraws**, so what is
/// typed and where the caret is are never lost to one.
function theNameField() {
  if (renaming.field) return renaming.field;
  const field = document.createElement("input");
  field.type = "text";
  field.className = "fld w-list";
  field.value = renaming.typed;
  field.setAttribute("aria-label", renaming.said.name);
  // **A press in the field is the field's**: it does not open the server, and
  // a right-click gets the field's own menu rather than the row's.
  field.addEventListener("click", (press) => press.stopPropagation());
  field.addEventListener("contextmenu", (press) => press.stopPropagation());
  field.addEventListener("input", () => {
    if (!renaming || renaming.field !== field) return;
    renaming.typed = field.value;
    theRenameIsChecked();
  });
  field.addEventListener("keydown", (press) => {
    if (press.key === "Enter") {
      press.preventDefault();
      theRenameIsSaved();
    } else if (press.key === "Escape") {
      press.preventDefault();
      theRenameIsLetGo();
    }
  });
  renaming.field = field;
  return field;
}

/// **Add's duplicate-name line, under the field**, drawn from the check's
/// answer and empty where it carries none. It never blocks.
function theSameNameLine() {
  if (!renaming.line) {
    renaming.line = document.createElement("span");
    renaming.line.className = "svextra";
  }
  theSameNameIsSaid();
  return renaming.line;
}

function theSameNameIsSaid() {
  if (!renaming || !renaming.line) return;
  const line = renaming.line;
  line.replaceChildren();
  const same = renaming.sameName;
  line.hidden = !same;
  if (!same) return;
  const lead = document.createElement("b");
  lead.textContent = same.lead;
  line.appendChild(lead);
  line.appendChild(textNode(" "));
  const named = document.createElement("b");
  theValueIsDrawnInto(named, same.named);
  // A full stop is punctuation and not a word, as in the dialogs' own line.
  named.appendChild(textNode("."));
  line.appendChild(named);
  line.appendChild(textNode(" "));
  line.appendChild(textNode(same.rest));
}

/// **The add's two checks, on the saved details and the typed name** — the
/// check Edit makes, leaving this row out. Only the duplicate-name line is
/// drawn from its answer: the host, port and username are the saved ones.
function theRenameIsChecked() {
  if (!renaming) return;
  const said = renaming.said;
  shell.core
    .invoke("the_edit_is_checked", {
      server: renaming.server,
      host: said.the_host,
      port: said.the_port,
      username: said.the_username,
      label: renaming.typed,
    })
    .catch(() => {});
}

function theRenameCheckIsDrawn(said) {
  if (!renaming) return;
  renaming.sameName = said.alreadyCalled
    ? { lead: said.alreadyCalled, named: said.alreadyCalledNamed, rest: said.alreadyCalledRest }
    : null;
  theSameNameIsSaid();
}

/// **Return: the name is saved**, through the command Edit saves with,
/// carrying the saved details unchanged. A name left as it was saves nothing.
function theRenameIsSaved() {
  if (!renaming || renaming.sent) return;
  if (renaming.typed === renaming.was) {
    theRenameIsLetGo();
    return;
  }
  renaming.sent = true;
  const said = renaming.said;
  theConnectionFailureIsTakenAway(renaming.server);
  shell.core
    .invoke("an_edit_is_saved", {
      server: renaming.server,
      host: said.the_host,
      port: said.the_port,
      username: said.the_username,
      keyIdentifier: said.the_key_identifier,
      label: renaming.typed,
    })
    .catch(() => {});
}

/// **Escape, or anything else the user turns to: the row is put back as it
/// was**, and nothing is saved.
function theRenameIsLetGo() {
  if (!renaming) return;
  const server = renaming.server;
  renaming = null;
  theRowIsDrawnAgain(server);
}

function theNameFieldHasTheKeys() {
  return Boolean(renaming && renaming.field && document.activeElement === renaming.field);
}

/// The field takes the keys: selected as a rename starts, and left where the
/// caret was after a redraw.
function theNameFieldTakesTheKeys(selected) {
  if (!renaming || !renaming.field) return;
  renaming.field.focus();
  if (selected) renaming.field.select();
}

// ── The Edit dialog ─────────────────────────────────────────────────────────

function theEditIsOpened(server) {
  shell.core
    .invoke("the_edit_dialog", { server })
    .then((said) => theEditDialogIsDrawn(server, said))
    // **A server that has just gone is said where the user pressed** (step 10a
    // round 1, fix-here item 32).
    .catch((failure) => theActWasNotTaken(failure || {}));
}

function theEditDialogIsDrawn(server, said) {
  const box = aDialog();
  const heading = document.createElement("h3");
  saidInto(heading, said.heading, "mono");
  box.appendChild(heading);

  const frm = document.createElement("div");
  frm.className = "frm";
  const host = aField(frm, said.host, said.the_host, true);
  const hostRefusal = aRefusalUnder(frm);
  const port = aField(frm, said.port, said.the_port, false);
  port.classList.add(THE_PORTS_OWN_CLASS);
  thePortPutsTheStandardBack(port, said.the_standard_port);
  const portRefusal = aRefusalUnder(frm);
  const username = aField(frm, said.username, said.the_username, true);
  const usernameRefusal = aRefusalUnder(frm);

  // **`Servers` D14, said under the username field as a plain statement.**
  frm.appendChild(document.createElement("span"));
  const usernameNote = document.createElement("span");
  usernameNote.className = "mnote2";
  usernameNote.textContent = said.the_username_note;
  frm.appendChild(usernameNote);

  // **`Keys` U11: for the Edit dialog the context is the saved row's label.**
  const key = aKeyControl(
    frm,
    said,
    {
      key: said.the_key_identifier,
      fingerprint: "",
      showsAs: said.the_key_shows_as,
      gone: said.the_gone_mark,
    },
    () => ({ label: said.the_name, username: "", host: "" }),
  );
  const keyRefusal = aRefusalUnder(frm);

  // **An unusable key stays selected rather than being cleared**, and the
  // note says why.
  if (said.the_gone_key_note) {
    frm.appendChild(document.createElement("span"));
    const goneNote = document.createElement("span");
    goneNote.className = "mnote2";
    goneNote.textContent = said.the_gone_key_note;
    frm.appendChild(goneNote);
  }

  const nameLabel = document.createElement("label");
  nameLabel.textContent = said.name;
  frm.appendChild(nameLabel);
  const name = document.createElement("input");
  name.type = "text";
  name.className = "fld wide";
  name.value = said.the_name;
  frm.appendChild(name);

  // **The server's fallback folder, on Edit only** — `Design/Servers § The
  // Edit dialog`, the drawing's `sv-edit`: typed, empty with *Smoozy's folder*
  // as its placeholder, and one grey line under it. **A path that does not
  // start with `/` is refused in place**, in the port's red, above the grey
  // line (`sv-fallbad`). The Creator's second notes, piece 4.
  const fallback = aField(frm, said.fallback_folder, said.the_fallback_folder, true);
  fallback.placeholder = said.the_fallback_placeholder;
  const fallbackRefusal = aRefusalUnder(frm);
  frm.appendChild(document.createElement("span"));
  const fallbackNote = document.createElement("span");
  fallbackNote.className = "mnote2";
  fallbackNote.textContent = said.the_fallback_note;
  frm.appendChild(fallbackNote);
  fallback.addEventListener("input", () => {
    theRefusalReads(fallbackRefusal, "");
    fallback.classList.remove("svbad");
  });
  box.appendChild(frm);

  // **The add's two checks run on the edited fields, leaving this row out**
  // (step 10a round 1, fix-here item 19).
  const duplicate = document.createElement("p");
  duplicate.className = "mwarn";
  duplicate.hidden = true;
  box.appendChild(duplicate);
  const sameName = document.createElement("p");
  sameName.className = "mwarn";
  sameName.hidden = true;
  box.appendChild(sameName);
  // **`Servers` W34's warning, drawn from each check's answer and removed when
  // an answer carries none** — `Servers` W37: it shows only while the host,
  // port or username differs from what is saved. The application composes it
  // from what is typed and it arrives on the check's own answer, drawn by
  // `theChecksAreDrawn`; this file composes no sentence. Until 2026-09-19 it
  // was drawn once when the dialog opened, from an empty list, so it never
  // showed (the Gemini catch-up of step 10a, Area 5, F-1).
  const warning = document.createElement("p");
  warning.className = "mwarn";
  warning.hidden = true;
  box.appendChild(warning);
  const check = () =>
    shell.core.invoke("the_edit_is_checked", {
      server,
      host: host.value,
      port: port.value,
      username: username.value,
      label: name.value,
    });
  for (const field of [host, port, username, name]) {
    field.addEventListener("input", check);
    field.addEventListener("change", check);
  }

  let remembered = null;
  if (said.a_password_is_remembered) {
    remembered = document.createElement("p");
    remembered.className = "mnote2";
    remembered.textContent = said.a_password_is_remembered;
    remembered.appendChild(textNode(" "));
    const forget = document.createElement("button");
    forget.type = "button";
    forget.className = "lnkb";
    forget.textContent = said.forget_it;
    // **`Servers` D18's erase** (step 10a round 1, fix-here item 19).
    forget.addEventListener("click", () => {
      shell.core.invoke("a_remembered_password_is_forgotten", { server });
    });
    remembered.appendChild(forget);
    box.appendChild(remembered);
  }

  const notice = aNoticeIn(box);

  const row = document.createElement("div");
  row.className = "mrow";
  const grow = document.createElement("span");
  grow.className = "grow";
  row.appendChild(grow);
  const cancel = document.createElement("button");
  cancel.type = "button";
  cancel.className = "btn quiet";
  cancel.textContent = said.cancel;
  cancel.addEventListener("click", theDialogIsClosed);
  row.appendChild(cancel);
  const save = document.createElement("button");
  save.type = "button";
  save.className = "btn";
  save.textContent = said.save;
  save.addEventListener("click", () => {
    theConnectionFailureIsTakenAway(server);
    shell.core.invoke("an_edit_is_saved", {
      server,
      host: host.value,
      // **The port travels as the text the user typed.** A number here would
      // refuse anything outside a whole number's own range before the
      // application saw it, and the user would meet silence where Design
      // requires a refusal in place.
      port: port.value,
      username: username.value,
      keyIdentifier: key.key,
      label: name.value,
      // **The folder travels as typed**; the application trims it and
      // refuses one that is not a full path.
      fallbackFolder: fallback.value,
    });
  });
  row.appendChild(save);
  box.appendChild(row);

  theRefusalsAreDrawnInto({
    host: hostRefusal,
    port: portRefusal,
    username: usernameRefusal,
    key: keyRefusal,
    fallback: fallbackRefusal,
    duplicate,
    sameName,
    notice,
    remembered,
    warning,
    fields: { host, port, username, fallback },
  });
  theDialogIsLabelled(box, heading);
}

// ── The removal box ─────────────────────────────────────────────────────────

// The switch-off's own control, while a removal box is open. **Run-held and
// about what the user is looking at**, which is what run-held means.
let theSwitchOff = null;

function theRemovalIsOpened(server) {
  theSwitchOff = null;
  shell.core
    .invoke("the_removal_of", { server })
    .then((said) => theRemovalBoxIsDrawn(server, said))
    .catch((failure) => theActWasNotTaken(failure || {}));
}

function theRemovalBoxIsDrawn(server, said) {
  const box = aDialog();
  const heading = document.createElement("h3");
  saidInto(heading, said.heading, "mono");
  box.appendChild(heading);
  const sub = document.createElement("p");
  sub.className = "msub";
  sub.textContent = said.said;
  box.appendChild(sub);

  // **The warning is composed from the last-known reading's own counts**, in
  // runs of words and figures set side by side (step 10a round 1, fix-here
  // item 17).
  if (said.the_warning) {
    const warning = document.createElement("p");
    warning.className = "mwarn hard";
    const bold = document.createElement("b");
    said.the_warning.forEach((run, at) => {
      if (at > 0) bold.appendChild(textNode(" "));
      saidInto(bold, run, "mono");
    });
    warning.appendChild(bold);
    box.appendChild(warning);
  }

  if (said.leave_it_running || said.stop_and_clear) {
    const choices = document.createElement("div");
    choices.className = "choices";
    if (said.leave_it_running) {
      choices.appendChild(aChoice(said.leave_it_running, server, false));
    }
    // **`Servers` W20's switch-off, on the one row of the table that has
    // one**, and it says what it covers.
    if (said.the_switch_off) {
      const off = document.createElement("div");
      off.className = "svoff";
      const label = document.createElement("label");
      const tick = document.createElement("span");
      tick.className = "cb";
      label.appendChild(tick);
      label.appendChild(textNode(said.the_switch_off.label));
      // **Ticked, the box fills and shows a tick mark**, the drawing's `.cb.on`
      // with its ✓, as every other box in Smoozy does. Until 2026-10-04 only
      // the label wore `on`, which no rule styles, so a tick drew nothing at
      // all (the Creator's note 2 on the test copy, the same omission).
      const theTickIsShown = () => {
        const on = label.classList.contains("on");
        tick.classList.toggle("on", on);
        tick.textContent = on ? "✓" : "";
      };
      if (said.the_switch_off.already) label.classList.add("on");
      theTickIsShown();
      label.addEventListener("click", () => {
        label.classList.toggle("on");
        theTickIsShown();
      });
      off.appendChild(label);
      const covers = document.createElement("span");
      covers.className = "mnote2";
      covers.textContent = said.the_switch_off.what_it_covers;
      off.appendChild(covers);
      theSwitchOff = label;
      choices.appendChild(off);
    }
    if (said.stop_and_clear) {
      choices.appendChild(aChoice(said.stop_and_clear, server, true));
    }
    box.appendChild(choices);
  }

  const notice = aNoticeIn(box);

  const row = document.createElement("div");
  row.className = "mrow";
  const grow = document.createElement("span");
  grow.className = "grow";
  row.appendChild(grow);
  const cancel = document.createElement("button");
  cancel.type = "button";
  cancel.className = "btn quiet";
  cancel.textContent = said.cancel;
  cancel.addEventListener("click", theDialogIsClosed);
  row.appendChild(cancel);
  // **The plain confirmation's own control**, where no choice is offered.
  if (said.remove) {
    const remove = document.createElement("button");
    remove.type = "button";
    remove.className = "btn danger";
    remove.textContent = said.remove;
    remove.addEventListener("click", () => theRemovalIsAsked(server, false));
    row.appendChild(remove);
  }
  box.appendChild(row);
  theRefusalsAreDrawnInto({ notice, fields: {} });
  theDialogIsLabelled(box, heading);
}

/// **The server's identity changed** — `Design/Servers § A changed identity`,
/// and the mockup's `sv-ident`.
///
/// **No fingerprint is shown, anywhere.** That is the spec's own policy and
/// `Servers` D9's bar agreeing: the identity means nothing to a 3D artist, and
/// a screen nobody can act on is worse than no screen.
///
/// **Called when a connection meets an identity that is not the one
/// remembered** — `THE_CHANGED_IDENTITY_WARNING`, published by the period
/// driving that connection (`src/servers/published.rs`).
export function theChangedIdentityIsAsked(server, said, name) {
  const box = aDialog();
  const heading = document.createElement("h3");
  const named = document.createElement("span");
  named.className = "mono";
  theValueIsDrawnInto(named, name);
  heading.appendChild(named);
  heading.appendChild(textNode(" "));
  saidInto(heading, said.heading);
  box.appendChild(heading);
  const sub = document.createElement("p");
  sub.className = "msub";
  sub.textContent = said.said;
  box.appendChild(sub);
  const accepting = document.createElement("p");
  accepting.className = "mwarn";
  accepting.textContent = said.accepting;
  box.appendChild(accepting);
  const row = document.createElement("div");
  row.className = "mrow";
  const grow = document.createElement("span");
  grow.className = "grow";
  row.appendChild(grow);
  const no = document.createElement("button");
  no.type = "button";
  no.className = "btn quiet";
  no.textContent = said.do_not_connect;
  no.addEventListener("click", theDialogIsClosed);
  row.appendChild(no);
  const yes = document.createElement("button");
  yes.type = "button";
  yes.className = "btn";
  yes.textContent = said.accept;
  yes.addEventListener("click", () =>
    shell.core.invoke("an_act_that_needs_a_connection", { act: "accept-the-identity", server })
  );
  row.appendChild(yes);
  box.appendChild(row);
  theDialogIsLabelled(box, heading);
}

/// **Stop and clear did not work** — `Design/Servers § When stop and clear does
/// not work`, and the mockup's `sv-sc`.
///
/// **`Servers` W21's three endings and not two**, because the request has two
/// ordered halves. **All three name what actually happened rather than saying
/// it failed**, and **removing anyway stays available in all three**.
export function theStopAndClearEndingIsShown(server, endings, which, name, words) {
  const box = aDialog();
  const heading = document.createElement("h3");
  heading.appendChild(textNode(words.stoppingWorkOn));
  heading.appendChild(textNode(" "));
  const named = document.createElement("span");
  named.className = "mono";
  theValueIsDrawnInto(named, name);
  heading.appendChild(named);
  box.appendChild(heading);
  for (const ending of endings) {
    const said = document.createElement("div");
    said.className = "rdfail";
    said.hidden = ending.which !== which;
    const line = document.createElement("p");
    const dot = document.createElement("span");
    dot.className = "fdot";
    line.appendChild(dot);
    const lead = document.createElement("b");
    lead.textContent = ending.lead;
    line.appendChild(lead);
    line.appendChild(textNode(" "));
    line.appendChild(textNode(ending.rest));
    said.appendChild(line);
    box.appendChild(said);
  }
  const row = document.createElement("div");
  row.className = "mrow";
  const grow = document.createElement("span");
  grow.className = "grow";
  row.appendChild(grow);
  const again = document.createElement("button");
  again.type = "button";
  again.className = "btn quiet";
  again.textContent = words.tryAgain;
  // ***Retry* is the removal's three units again** (`Servers` W25), so it
  // is marked on the control while the stop is waited on, like the choice
  // that began it.
  again.addEventListener("click", () => {
    if (underway) return;
    theControlIsUnderway(again, words.tryAgain);
    shell.core.invoke("an_act_that_needs_a_connection", { act: "stop-and-clear", server });
  });
  row.appendChild(again);
  const anyway = document.createElement("button");
  anyway.type = "button";
  anyway.className = "btn";
  anyway.textContent = words.removeAnyway;
  anyway.addEventListener("click", () => theRemovalIsAsked(server, false));
  row.appendChild(anyway);
  box.appendChild(row);
  theDialogIsLabelled(box, heading);
}

function aChoice(choice, server, reachesTheMachine) {
  const one = document.createElement("button");
  one.type = "button";
  one.className = "choice";
  const head = document.createElement("span");
  head.className = "ch";
  head.textContent = choice.head;
  one.appendChild(head);
  const said = document.createElement("span");
  said.className = "cd";
  saidInto(said, choice.said, "mono");
  one.appendChild(said);
  one.addEventListener("click", () => {
    if (underway) return;
    // ***Stop and clear first* waits on the machine** — up to `Rendering`
    // K9's two minutes for the stop — so its head is marked while it does,
    // in the underway shape's own mark on a control.
    if (reachesTheMachine) theControlIsUnderway(head, choice.head);
    theRemovalIsAsked(server, reachesTheMachine);
  });
  return one;
}

// ── A press that waits on the machine ───────────────────────────────────────
//
// **One at a time, and the mark comes off on the answer.** *Stop and clear
// first*, *Retry* and *Stop jobs and update* each wait on the machine's
// stop, and a second press while one waits would send a second pair. The
// control and its own words are held here only while the act runs.
let underway = null;

function theControlIsUnderway(control, label) {
  underway = { control, label };
  theMarkOnAControl(control, label);
}

function theUnderwayMarkComesOff() {
  if (!underway) return;
  theMarkComesOffAControl(underway.control, underway.label);
  underway = null;
}

/// **The owed `sv-sc` box, raised over its server's row** — the ending's key
/// and the server arrive as values, the words are the box's own.
function theOwedEndingIsShown(screen) {
  const owed = screen.the_stop_and_clear_ending;
  const row = screen.rows.find((one) => one.server === owed.server);
  if (!row) return;
  const boxes = screen.the_boxes_a_connection_raises;
  theStopAndClearEndingIsShown(owed.server, boxes.stop_and_clear, owed.which, row.name, {
    stoppingWorkOn: boxes.stopping_work_on,
    tryAgain: boxes.try_again,
    removeAnyway: boxes.remove_anyway,
  });
}

// **Whether a removal was asked and has not yet been answered as saved**, so
// its answer can send the server's own screen to ask again (below). Run-held,
// about the user's last act.
let aRemovalAwaitsItsAnswer = false;

function theRemovalIsAsked(server, reachesTheMachine) {
  theConnectionFailureIsTakenAway(server);
  aRemovalAwaitsItsAnswer = true;
  shell.core.invoke("a_server_is_removed", {
    server,
    reachesTheMachine,
    dontWarnAgain: Boolean(theSwitchOff && theSwitchOff.classList.contains("on")),
  });
}

// ── The answer to the user's last act ───────────────────────────────────────

whenTold(THE_ANSWER, (delivery) => {
  const said = delivery.said || {};
  if (connectionPlaces.size || menuContext) theScreenIsAskedFor();
  // **A page-level notice says one thing and goes on the next answer** (step
  // 10a round 1, fix-here item 12).
  theNoticeIsCleared();
  // **Whatever the answer, a press that waited on the machine has its
  // answer**, and its control comes back.
  theUnderwayMarkComesOff();
  // **The removal's stop and clear did not come to *cleared and stopped***:
  // the removal box closes and the list, asked for again, raises `sv-sc` with
  // `Servers` W21's ending (`theOwedEndingIsShown`).
  if (said.what === "the-stop-and-clear-ending-is-owed") {
    theDialogIsClosed();
    if (host || connectionPlaces.size || menuContext) theScreenIsAskedFor();
    return;
  }
  if (said.what === "the-add-was-checked") {
    return refusals ? theChecksAreDrawn(said) : theRenameCheckIsDrawn(said);
  }
  // **A rename's save has its answer**: the field goes, and the row is drawn
  // from what is true — the new name where it was saved, the old one and the
  // sentence above the list where it was not.
  if (renaming && renaming.sent) {
    const server = renaming.server;
    renaming = null;
    theRowIsDrawnAgain(server);
  }
  if (said.what === "these-fields-are-needed") return theBlanksAreDrawn(said);
  if (said.what === "the-port-will-not-do") return thePortRefusalIsDrawn(said);
  if (said.what === "the-fallback-folder-will-not-do") return theFallbackRefusalIsDrawn(said);
  if (said.what === "it-was-saved" || said.what === "it-was-saved-and-nothing-started") {
    theDialogIsClosed();
    // **The rail is the only way to reach a server**, so a rail that has gone
    // stale is a server the user cannot get to.
    theRailIsReadAgain();
    theScreenIsAskedFor();
    // **A removal that was saved sends the server's own screen to ask again**
    // — the Creator's eighteenth notes, note 1: from the rail's menu, the
    // removed server stayed in its pane as if it still existed. Asked again,
    // the pane says the server has been removed and its panels go
    // (`web/a-server.js` section 7). From either menu.
    if (aRemovalAwaitsItsAnswer) {
      aRemovalAwaitsItsAnswer = false;
      theServersAreAskedForAgain();
    }
    // **A ticked setup option saves the server, connects and sets it up**
    // (`Servers` O2, since step 10c piece 9), so the answer says nothing, as a
    // plain add's does: the list and the setup say what is happening. Until
    // step 10c's eleventh-pass fix piece 6 it said *Smoozy cannot do this
    // yet* here, which was false from piece 9 on.
    //
    // **And it lands on the new server's Set up tab**, where the connection and
    // then the setup are shown as they happen — `Design/Servers § Adding a
    // server`, the Creator's note 3 on 0.1.7: *"I expect Smoozy to show me
    // that something is being done the very second I add it."* **The answer
    // names the server only where a setup was ticked**; a plain add stays on
    // the list.
    if (said.itIsSetUp && said.server) {
      theServerWillOpenOn(said.server, SET_UP);
      theServerIsOpened(said.server);
    }
    return;
  }
  // **V15's stop and clear came to *cleared and stopped***: the box closes and
  // the row says what happens next.
  if (said.what === "the-update-now-was-taken") {
    theDialogIsClosed();
    if (host || connectionPlaces.size || menuContext) theScreenIsAskedFor();
    return;
  }
  // **Anything else, said as the removal says it** — `Servers` W21's two
  // sentences, in the box, which stays open. The row keeps its waiting line.
  if (said.what === "the-stop-and-clear-did-not-fully-work") {
    if (host || connectionPlaces.size || menuContext) theScreenIsAskedFor();
    return theWordsAreSaidInTheDialog([said.said, said.saidNext]);
  }
  if (said.what === "the-password-was-forgotten") {
    if (refusals && refusals.remembered) refusals.remembered.hidden = true;
    return;
  }
  // **A connect that did not happen names its server, and is kept for it** —
  // above this list, naming the server, and on its Set up tab, which reads it
  // from here. Never inside a dialog: it answers the connect, not the dialog.
  if (A_CONNECT_NOT_MADE.includes(said.what) && said.server) {
    theConnectsNotMade.set(said.server, said);
    theConnectsNotMadeAreDrawn();
    if (lastDrawn) theSharedControlsAreDrawn(lastDrawn);
    return;
  }
  if (said.what === "the-server-is-gone") {
    theRailIsReadAgain();
    theScreenIsAskedFor();
    theServersAreAskedForAgain();
  }
  // **The act is not built, the record would not take the write, or the server
  // is gone.** Inside the open dialog where one is open, and above the list
  // where none is — a control that does nothing twice is the fault L8a is
  // written against.
  if (refusals && refusals.notice) return theWordsAreSaidInTheDialog([said.said]);
  theActWasNotTaken(said);
});

// The closed act the refusal being drawn is about.

function theWordsAreSaidInTheDialog(sentences) {
  if (!refusals || !refusals.notice) {
    return theActWasNotTaken({ said: sentences.filter(Boolean).pop() });
  }
  const notice = refusals.notice;
  notice.replaceChildren();
  const [first, ...rest] = sentences.filter(Boolean);
  if (!first) return;
  const lead = document.createElement("b");
  lead.textContent = first;
  notice.appendChild(lead);
  for (const more of rest) {
    // An em dash between the lead and the act's own sentence, which is
    // punctuation and not a word — the shared value route.
    notice.appendChild(textNode(" — "));
    const one = document.createElement("span");
    one.textContent = more;
    notice.appendChild(one);
  }
  notice.hidden = false;
}

function theChecksAreDrawn(said) {
  if (!refusals) return;
  if (refusals.port) {
    theRefusalReads(refusals.port, said.thePortWillNotDo);
    if (refusals.fields.port) {
      refusals.fields.port.classList.toggle("svbad", Boolean(said.thePortWillNotDo));
    }
  }
  if (refusals.duplicate) {
    refusals.duplicate.textContent = "";
    refusals.duplicate.hidden = !said.mayAlreadyHave;
    if (said.mayAlreadyHave) {
      const lead = document.createElement("b");
      lead.textContent = said.mayAlreadyHave;
      refusals.duplicate.appendChild(lead);
      refusals.duplicate.appendChild(textNode(" "));
      const named = document.createElement("b");
      theValueIsDrawnInto(named, said.mayAlreadyHaveNamed);
      refusals.duplicate.appendChild(named);
      refusals.duplicate.appendChild(textNode(" "));
      refusals.duplicate.appendChild(textNode(said.mayAlreadyHaveRest));
    }
  }
  if (refusals.warning) theEditWarningIsDrawn(refusals.warning, said);
  if (refusals.sameName) {
    refusals.sameName.textContent = "";
    refusals.sameName.hidden = !said.alreadyCalled;
    if (said.alreadyCalled) {
      const lead = document.createElement("b");
      lead.textContent = said.alreadyCalled;
      refusals.sameName.appendChild(lead);
      refusals.sameName.appendChild(textNode(" "));
      const named = document.createElement("b");
      theValueIsDrawnInto(named, said.alreadyCalledNamed);
      // A full stop is punctuation and not a word, which is the same route
      // the value route takes for an em dash.
      named.appendChild(textNode("."));
      refusals.sameName.appendChild(named);
      refusals.sameName.appendChild(textNode(" "));
      refusals.sameName.appendChild(textNode(said.alreadyCalledRest));
    }
  }
}

// **`Servers` W34's warning, from the check's answer, or gone where the answer
// carries none.** The words arrived as prose and the count and every name as a
// value, each its own node (X7).
function theEditWarningIsDrawn(warning, said) {
  warning.replaceChildren();
  warning.hidden = !said.theEditWarning;
  if (!said.theEditWarning) return;
  const lead = document.createElement("b");
  lead.textContent = said.theEditWarning;
  warning.appendChild(lead);
  warning.appendChild(textNode(" "));
  const many = document.createElement("span");
  many.className = "mono";
  many.textContent = said.theEditWarningHowMany || "";
  warning.appendChild(many);
  warning.appendChild(textNode(BETWEEN_A_FIGURE_AND_ITS_UNIT));
  warning.appendChild(textNode(said.theEditWarningHaveNotEnded || ""));
  const named = Number(said.theEditWarningNamedCount || 0);
  for (let at = 0; at < named; at += 1) {
    // **The key is read out of the event before anything is drawn**, so no
    // statement that puts something on the page carries a written word.
    const theName = said[`theEditWarningNamed${at}`] || "";
    warning.appendChild(textNode(" "));
    const one = document.createElement("b");
    theValueIsDrawnInto(one, theName);
    warning.appendChild(one);
  }
  warning.appendChild(textNode(" "));
  warning.appendChild(textNode(said.theEditWarningWhatBecomesOfThem || ""));
}

function theBlanksAreDrawn(said) {
  if (!refusals) return;
  for (const one of [refusals.host, refusals.username, refusals.key]) {
    theRefusalReads(one, "");
  }
  const many = Number(said.fieldCount || 0);
  for (let at = 0; at < many; at += 1) {
    const which = said[`field${at}`];
    const words = said[`field${at}Said`];
    theRefusalReads(refusals[which], words);
  }
}

function thePortRefusalIsDrawn(said) {
  if (!refusals || !refusals.port) return;
  theRefusalReads(refusals.port, said.thePortWillNotDo);
  if (refusals.fields.port) refusals.fields.port.classList.add("svbad");
}

/// **A fallback folder that is not a full path, refused in place** — the
/// drawing's `sv-fallbad`: the field in the port's red and *Enter a full path,
/// starting with /.* under it. Nothing was saved, and the dialog stays open.
function theFallbackRefusalIsDrawn(said) {
  if (!refusals || !refusals.fallback) return;
  theRefusalReads(refusals.fallback, said.theFallbackFolderWillNotDo);
  if (refusals.fields.fallback) refusals.fields.fallback.classList.add("svbad");
}

function theNoticeIsCleared() {
  const globalNotice = dialogs && dialogs.querySelector(".svnotyet");
  if (globalNotice) globalNotice.hidden = true;
  if (!host) return;
  const told = host.querySelector(".svnotyet");
  if (told) told.hidden = true;
  if (refusals && refusals.notice) refusals.notice.hidden = true;
}

function theActWasNotTaken(said) {
  if (!said.said) return;
  if (!host || theCurrentPlace() !== ALL_SERVERS) {
    theDialogsAreReady();
    let told = dialogs.querySelector(".svnotyet");
    if (!told) {
      told = document.createElement("div");
      told.className = "sv-notool svnotyet";
      told.setAttribute("role", "alert");
      dialogs.appendChild(told);
    }
    told.textContent = said.said;
    told.hidden = false;
    return;
  }
  let told = host.querySelector(".svnotyet");
  if (!told) {
    told = document.createElement("div");
    told.className = "sv-notool svnotyet";
    host.insertBefore(told, list);
  }
  told.textContent = said.said;
  told.hidden = false;
}

// ── A connect that did not happen ───────────────────────────────────────────
//
// **The three answers a connect can end in** — declared beside the service
// that posts them (`THE_CONNECTION_WAS_NOT_MADE_ANSWER` and its two neighbours
// in `src/surface/servers.rs`). The two sides have to agree and nothing holds
// them together, so copy carefully.
const A_CONNECT_NOT_MADE = [
  "the-connection-was-not-made",
  "it-is-not-a-linux-machine",
  "the-passphrase-could-not-be-used",
];

// **The acts on a row that end what a connect that did not happen said** —
// `Design/Servers § Adding a server`: *"until the next act on that server"*.
// The other two, an edit and a removal, end it where they are asked.
const THE_ACTS_THAT_CONNECT = ["connect", "connect-and-set-up", "drop-the-setup"];

/// **One line per server, newest for each, in the application's own words**:
/// the lead and the server's name where the sentence names it — *Smoozy could
/// not connect to Hyperstack.* — and otherwise the sentence as said.
function theConnectsNotMadeAreDrawn() {
  if (!theConnectsNotMadeHolder) return;
  theConnectsNotMadeHolder.replaceChildren();
  for (const [server, said] of theConnectsNotMade) {
    const line = document.createElement("div");
    line.className = "sv-notool svnotmade";
    line.dataset.server = server;
    if (said.couldNotConnectTo && said.couldNotConnectToNamed) {
      const lead = document.createElement("span");
      lead.textContent = said.couldNotConnectTo;
      line.appendChild(lead);
      line.appendChild(textNode(" "));
      const named = document.createElement("b");
      theValueIsDrawnInto(named, said.couldNotConnectToNamed);
      line.appendChild(named);
      // A full stop is punctuation and not a word, as for *Another server is
      // already called …* above.
      line.appendChild(textNode("."));
    } else {
      line.appendChild(textNode(said.said));
    }
    theConnectsNotMadeHolder.appendChild(line);
  }
}

/// **What the last connect on this server that did not happen said**, or
/// nothing — the Set up tab draws its card from it (`web/set-up.js`).
export function theConnectionFailureOn(server) {
  return theConnectsNotMade.get(server) || null;
}

/// **The next act on that server takes it away**, wherever it was asked.
export function theConnectionFailureIsTakenAway(server) {
  if (theConnectsNotMade.delete(server)) {
    theConnectsNotMadeAreDrawn();
    if (lastDrawn) theSharedControlsAreDrawn(lastDrawn);
  }
}


// ── The failures a row carries ──────────────────────────────────────────────
//
// **One event per standing failure** — M7a, and the shape's own rule that two
// conditions standing at once are presented as two, each exactly as it would be
// were it the only one standing. **An arriving failure redraws its own row and
// asks nothing.**

whenTold(A_FAILURE_IS_STANDING, (delivery) => {
  const said = delivery.said || {};
  if (!lastDrawn || said.subjectKind !== lastDrawn.the_subject_kind) return;
  let held = failuresByServer.get(said.subject);
  if (!held) {
    held = new Map();
    failuresByServer.set(said.subject, held);
  }
  held.set(said.key, said);
  theRowIsDrawnAgain(said.subject);
  if (lastDrawn) theSharedControlsAreDrawn(lastDrawn);
});

whenTold(A_FAILURE_WAS_CLEARED, (delivery) => {
  const said = delivery.said || {};
  const held = said.subject && failuresByServer.get(said.subject);
  if (!held) return;
  held.delete(said.key);
  if (!held.size) failuresByServer.delete(said.subject);
  theRowIsDrawnAgain(said.subject);
  if (lastDrawn) theSharedControlsAreDrawn(lastDrawn);
});

// ── What the connection is doing ────────────────────────────────────────────
//
// **`Servers` publishes and this screen draws** — `Servers` K7: that module
// supplies the observed value and the transitions and presents neither. **The
// row is not composed from the event**: the event says the value moved, and
// the screen is asked for again, so the row, the footer's connected count, the
// menu it is given and the connect control's own word are all composed in one
// place from what the run holds.
//
// **Nothing here ticks and nothing here holds a value.** The event carries the
// moment the next attempt is due; the drawing that counts down against it is
// the underway shape's, which this screen does not host.

whenTold(WHAT_IS_OBSERVED, () => {
  if (!host && !connectionPlaces.size && !menuContext) return;
  theScreenIsAskedFor();
});

// **What a server is doing moved** — a queue reading was taken or an ending
// learned — so the list is asked for again, exactly as for a change of
// connection. Step 10c's ninth-pass fix piece 2.
whenTold(THE_WORK_MOVED, () => {
  if (!host && !connectionPlaces.size && !menuContext) return;
  theScreenIsAskedFor();
});

// **A condition raised or withdrawn moves the line on the row**, and the line
// is the failure shape's, read back out of the record by the same fetch the
// screen's own asking makes.
whenTold(THE_CONNECTION_CONDITION, (delivery) => {
  const said = delivery.said || {};
  if (!host && !connectionPlaces.size && !menuContext) return;
  // **A withdrawal takes the line away rather than waiting to be cleared** —
  // `Design/Servers`: the line goes when the connection is made again, however
  // that happened.
  if (said.whatHappened === "withdrawn") failuresByServer.delete(delivery.subject);
  theScreenIsAskedFor();
});

// **`Servers` Q6's third path, put to the user.** The dialog is the one Design
// draws and the words are the screen's own; what the event carries is which
// server it is about.
whenTold(THE_CHANGED_IDENTITY_WARNING, (delivery) => {
  if (!lastDrawn) return;
  const server = delivery.subject;
  const row = lastDrawn.rows.find((one) => one.server === server);
  if (!row) return;
  theChangedIdentityIsAsked(
    server,
    lastDrawn.the_boxes_a_connection_raises.the_changed_identity,
    row.name,
  );
});

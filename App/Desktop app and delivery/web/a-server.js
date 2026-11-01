// **A server's own screen: the head, the two tabs, and the panels over it.**
//
// **READ THIS BEFORE WRITING A TAB OR A PANEL. It is the contract for pieces
// 6, 7, 8 and 9 of step 10a, and it is written here rather than only in the
// decisions record because this is the file somebody opens when they need it.**
// Piece 2 did the same at the head of `web/the-rail.js` and the five screens
// after it merged without argument; this is that habit kept, one level down.
//
// **Read the head of `web/the-rail.js` first.** Everything below sits inside
// its section 3: this screen is drawn once, hidden when the user goes away, and
// never taken down.
//
// ═══════════════════════════════════════════════════════════════════════════
// THE TAB AND PANEL CONTRACT
// ═══════════════════════════════════════════════════════════════════════════
//
// ## 1. There are two tabs and two panels, and there will not be a third
//
//     files        Files            piece 6
//     setupview    Set up           piece 7
//     chk          the dependency check   piece 9
//     set          render settings        piece 8
//
// **The names are the drawing's own.** `files` and `setupview` are the
// identifiers the mockup gives those two bodies, so the host this file makes
// carries the identifier and every rule the drawing already has applies to it
// unchanged. `chk` and `set` are the mockup's own collapse-and-open keys.
// `src/surface/a_server.rs` holds the same four and a test there reads the
// drawing and fails if any of them has moved.
//
// **Import the name, never type it.** The four are exported below.
//
// **Two tabs and two panels is Design's own list and not a function of what has
// been built** — `Design/Whole app § Where things live` for the tabs, `§ Panels
// over a server` for the panels. A tab or a panel nothing has registered still
// exists: it is marked `data-tab="none"` or `data-panel="none"`, which is the
// mounting contract's section 6 one level down.
//
// ## 2. How a tab registers its content, and how it is told it is showing
//
//     import { aTabIsRegistered, FILES } from "./a-server.js";
//
//     aTabIsRegistered({
//       tab: FILES,
//       itIsDrawn(host) { … },      // once, the first time this tab is shown
//       itIsShowing(server) { … },  // every time, the first included
//       itIsPutAway() { … },        // every time it stops showing
//     });
//
// Register at the top level of your own module. **Registering draws nothing.**
//
// - **`itIsDrawn(host)` runs once for the whole run.** The host is the
//   drawing's own element — `<div id="files" class="files">` — and it is yours
//   for the life of the run. **Nothing is ever taken down.**
// - **`itIsShowing(server)` runs every time this tab becomes the showing one,
//   and again when the user goes to a different server while it is showing.**
//   `server` is that server's own identity. **Use it to ask the application
//   again**: events are volatile and nothing is replayed (P8).
// - **`itIsPutAway()` runs every time it stops showing.** Shut a menu, drop a
//   listener. **Do not save anything**: the element stays and so does
//   everything in it.
//
// **There is one host per tab and not one per server, and that is deliberate.**
// `Design/File Browser § Where it sits` settles that one server is browsed at a
// time and that the browser **moves** between them — the drawing even has a
// state for another module moving it. Set up is a reading of whichever server
// is open. **So the tab is one thing that changes server, which is why
// `itIsShowing` carries the server and `itIsDrawn` does not.**
//
// **Which tab is showing is remembered per server**, because a server the user
// left on Set up and came back to on Files would have moved under them. That is
// the panels' own rule below, applied to the one other thing on this screen
// that has a position.
//
//     theCurrentTabIs(SET_UP);
//
// is how a piece sends the user to the other tab — the drawing's own *This
// server has no Blender. Set it up* is that call. It acts on the server that is
// open.
//
// ## 3. How a panel is opened over a server
//
//     import { aPanelIsRegistered, thePanelIsOpenedOver, RENDER_SETTINGS }
//       from "./a-server.js";
//
//     aPanelIsRegistered({
//       panel: RENDER_SETTINGS,
//       itIsDrawn(parts, server) { … },   // once per server
//       itIsShowingAgain(server) { … },   // when it comes back from the strip
//       theFileItIsOn(server) { … },      // the `.blend` it is open on, as bytes
//       itIsShownOn(server, file) { … },  // opened from a never-opened strip
//     });
//
// **The last two are for a panel never opened over that server** (section 5):
// its strip is pressed while the other panel is open, so it opens on the open
// panel's file — `theFileItIsOn` of the open one — through its own way in that
// starts nothing. The check's is `theCheckIsShownOn`, Render Settings' is
// `theSettingsAreOpenedOn`.
//
//     thePanelIsOpenedOver(theServer, RENDER_SETTINGS);
//
// `parts` is `{ head, body }` — the title area of the panel's own header, and
// the panel's body. **The pinned collapse control is not in either of them**
// and is not yours: `Design/Whole app § Panels over a server` is Creator-stated
// that collapsing *"is a chevron pinned to the panel's top corner, not a
// control in the header's flow"*, so it is placed here and the header wraps
// around it.
//
// - **`thePanelIsOpenedOver(server, panel)` takes the server rather than
//   assuming the current one.** A panel belongs to a server, so a call that
//   guessed would be a panel that followed the user.
// - **Opening one collapses the other**, because each covers the server's own
//   screen and two cannot both cover it. **The other then waits as its strip
//   at the very bottom, under the open one** — section 5.
// - **`thePanelIsPutAway(server, panel)` ends a panel.** It is for the piece
//   that owns the panel — a submission that is done with its settings.
//   **Nothing the user presses calls it**: Design gives them collapse and
//   nothing else, and *"a panel's work is never stopped by a key"*. **The one
//   other caller is this file**, when a panel is opened on a different `.blend`
//   from the one the other panel is on (section 5, W5): the other lets go of
//   its file, and its work goes on.
// - **The rail says nothing about an open panel.** Until 2026-10-04 the
//   server's rail row carried a mark while a panel was open on it; the
//   Creator's note 19 on the test copy struck it (`Design/Whole app § Panels
//   over a server`), and the rail's one dot now says what the server is doing.
//   **The panel's own strip along the bottom of the server's screen is what
//   says a panel is open**, and it already keeps counting.
//
// ## 4. What collapsing means exactly
//
// **The panel goes to a strip along the bottom and comes back exactly as it
// was.** Three things settle what that costs.
//
// - **What keeps running: everything.** The element is hidden, not taken down.
//   No listener is dropped, no request is stopped, no reading is thrown away,
//   and this file calls nothing on the panel when it collapses — **which is why
//   there is no `itWasCollapsed`.** `Design/Whole app § Panels over a server`:
//   *"Neither can block … Collapsing is what lets them go and do something
//   else."*
// - **What is remembered: nothing, by anybody.** The scroll position, the
//   half-typed field, the open dropdown and the chosen row all survive for
//   nothing, because they live in the element and the element is still there.
//   **Do not remember any of them yourself** — the mounting contract's section
//   3 makes the same argument one level up.
// - **What is drawn in the strip: whatever the panel last said.**
//
//       theStripReads(server, RENDER_SETTINGS, { title, reading });
//
//   `title` and `reading` are values or prose the application already composed.
//   **No strip carries a mark** — `Design/Whole app § Panels over a server`,
//   2026-10-17, the Creator's tenth notes, note 5: until then the drawing put
//   an amber dot on Render Settings' strip, always there and so saying
//   nothing. **A panel that has said nothing gets its own name and nothing
//   else.** The strip keeps counting because the panel keeps running and keeps
//   calling this — **the counting is the panel's and the strip is this file's.**
//
// **Escape collapses the panel on top.** `Design/Whole app § Keyboard
// shortcuts`, and it binds every panel on every screen: *"On a panel, Escape
// collapses rather than ends."* The press is taken only where it came from
// inside this screen, nothing above the panel has taken it, and it did not come
// from inside a box — *"Escape closes the thing on top, one layer at a time. A
// box, then a panel, then a menu."* **A box over a panel handles its own
// Escape**, and this file steps aside for anything with `role="dialog"`.
//
// ## 5. How two collapsed strips sit together
//
// **They stack, the check above the render settings, and the order is fixed.**
// It is the drawing's own order and it does not follow the order the user
// collapsed them in: a strip that moved because the other one was collapsed is
// a target that shifts while somebody is reaching for it.
//
// **While one panel is open, the other waits as its strip at the very bottom,
// under the open one.** `Design/Whole app § Panels over a server`, added
// 2026-10-08 on the Creator's fifth notes, note 3, and the drawing's state
// `res-chkstrip`. **Pressing it swaps the two**: that panel comes up and the
// one that was open becomes the strip — the strip's ordinary press does it,
// because opening one collapses the other.
//
// **The strips are drawn inside the open panel's frame, after its sheet.** The
// frame lays the two out as a column, the sheet may shrink and the strips may
// not, so the sheet ends just above the strip and nothing is measured. Drawn
// anywhere else, a strip sits behind the panel: **a control nobody can see and
// Tab can still reach**, which is why, until note 3, no strip was drawn while a
// panel was open.
//
// **A panel never opened over this server still has its strip while the other
// is open**; it reads its name alone, and pressing it opens that panel on the
// open panel's file without starting anything — `Design/Whole app § Panels
// over a server`, changed 2026-10-10 on the Creator's seventh notes, note 2,
// and the drawing's `.strip.fresh` with the states `res-chkfresh` and
// `chkfresh-set`. Until then such a panel had no strip at all. **With no panel
// open it still has none**: there is no file to open it on.
//
// **Opening either panel on a different `.blend` moves both to that file** —
// `Design/Whole app § Panels over a server`, after line 139, Creator-stated
// 2026-10-18 (the twelfth notes, note 8, W5). The other panel lets go of the
// file it was on — this file puts it away — and waits as that same
// fresh strip, reading its name alone, ready to open on the new file. Opening
// one on the file the other is on moves nothing, and **choosing a row in Files
// moves neither**. Letting go loses nothing: what was typed is the
// application's, keyed by server and file (`Render Settings` J3 and `Render Settings` J6).
//
// **A strip that is redrawn while it has the focus keeps it.** The panel keeps
// calling `theStripReads` as it counts, and each call draws the strips again.
//
// ## 6. Going to a different server, and coming back
//
// **Nothing happens to a panel at all.** `Design/Whole app § Panels over a
// server` is Creator-stated: *"A panel belongs to its server and does not
// follow the user. Going to another server, or to the queue, leaves both panels
// where they were; coming back finds them unchanged."*
//
// **So a panel's elements are made once per server and hidden with that
// server**, exactly as a screen's are made once and hidden with the screen. The
// head is re-read on every arrival (P8) and the tab body is one host that
// changes server; **the panels are not, and that difference is the whole of
// this section.**
//
// ## 7. What happens to a panel when its server is removed
//
// **It goes, with everything else of that server's.** When this screen is asked
// for and the application answers that there is no such server, that server's
// panels and strips are taken down and forgotten and the screen says the server
// has been removed. **This is the one place the never-destroyed rule is set
// aside, and the reason is that the thing it belongs to is gone**: the rail no
// longer lists the server, so nothing could reach the panel again, and a panel
// over a server that is not there is a drawing of nothing.
//
// **Nothing is saved out of it first**, because a panel holds no durable value:
// what the user typed into the render settings is that panel's own store's, and
// removing a server is a deliberate act with a warning in front of it.
//
// ## 8. What this screen does not do
//
// - **It draws no failure of its own.** A failure standing against a server is
//   drawn on that server's **row**, in the one failure shape's collapsed room,
//   on the All servers screen. What this screen says instead — the server has
//   been removed, the record would not read — is L8a and comes from the
//   application.
// - **It has no seam.** The pane-width record holds three and there is no
//   fourth; a tab's own list, if it wants one, is that tab's.
// - **It holds no figure of its own.** The strip's counting is the panel's and
//   arrives through `theStripReads`.
//
// ═══════════════════════════════════════════════════════════════════════════
//
// **And the four things that bind whatever a piece writes** are at the head of
// `web/smoozy.js`: results arrive as events; values are content and never
// markup (H32); no inline script and no inline style (H33); nothing repeats on
// a timer (P9, P10). This file keeps all four.

import { aScreenIsRegistered, A_SERVER } from "./the-rail.js";
import { theMomentReads, theConnectionIsDrawnInto } from "./servers.js";
import { whenTold } from "./event-route.js";
import { theValueIsDrawnInto } from "./the-value-drawn.js";

const shell = window.__TAURI__;

// **What `Servers` publishes, declared beside the service that posts it** —
// `WHAT_IS_OBSERVED` and `THE_CONNECTION_CONDITION` in
// `src/servers/published.rs`. The two sides have to agree and nothing holds
// them together, so copy carefully.
const WHAT_IS_OBSERVED = "servers:what-is-observed";
const THE_CONNECTION_CONDITION = "servers:the-connection-condition";

// **What a server is doing moved** — `THE_WORK_MOVED` in
// `src/surface/servers.rs`. The head's work column is the list row's.
const THE_WORK_MOVED = "servers:the-work-moved";
const SVG_NS = "http://www.w3.org/2000/svg";

// **The four names.** The same strings `src/surface/a_server.rs` holds, and a
// test there fails if the two have drifted from each other or from the drawing.
export const FILES = "files";
export const SET_UP = "setupview";
export const RENDER_SETTINGS = "set";
export const THE_DEPENDENCY_CHECK = "chk";

// The order the two strips stack in, which is the drawing's own and is fixed.
const THE_PANELS = [THE_DEPENDENCY_CHECK, RENDER_SETTINGS];

// ── What this module holds ──────────────────────────────────────────────────

// tab key → { itIsDrawn, itIsShowing, itIsPutAway, host }
const tabs = new Map();
// panel key → { itIsDrawn, itIsShowingAgain }
const panels = new Map();
// server identity → everything of that server's that is made once
//   { drawn: Map(panel → { wrap, head, body }), strips, strip: Map(panel → el),
//     reading: Map(panel → said), open: panel or null, tab: tab key }
const servers = new Map();

let host = null;
let head = null;
let badge = null;
let name = null;
let address = null;
let connection = null;
let connectionNote = null;
let hardware = null;
let strip = null;
let instead = null;
let standIn = null;
// The last answer the application gave, so the stand-in and the strips can be
// drawn from the same words the head was.
let said = null;
// Which server is open. **Not a second copy of anything durable** — it is which
// place the user is standing in, which is run-held and nothing else holds it.
let current = null;

// ── The screen ──────────────────────────────────────────────────────────────

aScreenIsRegistered({
  place: A_SERVER,

  itIsDrawn(into) {
    host = into;

    // **The head — the mockup at 6353 to 6369, in its own classes.**
    head = document.createElement("div");
    head.className = "mhead";

    const top = document.createElement("div");
    top.className = "mtop";
    const title = document.createElement("h1");
    badge = document.createElement("span");
    name = document.createElement("span");
    title.appendChild(badge);
    title.appendChild(name);
    address = document.createElement("span");
    address.className = "addr mono selectable";
    top.appendChild(title);
    top.appendChild(address);
    connection = document.createElement("span");
    connection.className = "mconnection";
    top.appendChild(connection);
    head.appendChild(top);
    connectionNote = document.createElement("p");
    connectionNote.className = "mconnectnote";
    connectionNote.hidden = true;
    head.appendChild(connectionNote);

    hardware = document.createElement("div");
    hardware.className = "hw";
    head.appendChild(hardware);

    strip = document.createElement("div");
    strip.className = "tabs";
    strip.setAttribute("role", "tablist");
    head.appendChild(strip);

    // **The stand-in's own row.** It draws one control per panel nothing has
    // registered, and it is empty the day pieces 8 and 9 have both landed.
    standIn = document.createElement("div");
    standIn.className = "stgrow";
    head.appendChild(standIn);

    host.appendChild(head);

    // **What is said where there is no head** — L8a.
    instead = document.createElement("div");
    instead.className = "suscroll";
    instead.hidden = true;
    host.appendChild(instead);

    // **Escape collapses the panel on top.** Bound on this screen's own host,
    // so a press anywhere else in the window is not this screen's.
    document.addEventListener("keydown", (press) => {
      if (press.key !== "Escape" || press.defaultPrevented) return;
      // **The drawing's own layering, kept by where the press came from.** A
      // box over the panel closes first, so a press from inside one is not
      // this screen's; and a press from inside anything outside this screen —
      // a menu in the title bar — belongs to whatever drew that. **A press
      // with nothing focused at all is this screen's**, which is the ordinary
      // case after a strip has been pressed and taken away.
      // **Only while this screen is the current place** (step 10a round 1,
      // fix-here item 26): a press on another screen, with focus on the body,
      // used to collapse a panel nobody could see. `current` is cleared when
      // this screen is put away.
      if (!current) return;
      const inside =
        press.target === document.body || (host && host.contains(press.target));
      if (!inside) return;
      if (press.target.closest && press.target.closest("[role=dialog]")) return;
      const held = current && servers.get(current);
      if (!held || !held.open) return;
      press.preventDefault();
      theOpenPanelIsCollapsed(current);
    });
  },

  // **The contract's section 4, one level up: ask the application again.**
  itIsCurrentNow(which) {
    if (!host || !which) return;
    theWorkColumnIsOnScreen(which);
    theScreenIsAskedFor(which);
  },

  itIsPutAway() {
    theConnectionIsDrawnInto(connection, connectionNote, null);
    theWorkColumnIsOnScreen(null);
    // **The panels stay exactly where they are** — section 6 above. What is put
    // away is the tab that was showing, which is the one thing on this screen
    // that may be holding an open menu.
    const showing = theShowingTab();
    if (showing && showing.itIsPutAway) showing.itIsPutAway();
    // **No server is open while this screen is away**, so nothing here acts on
    // a key pressed elsewhere. Arriving again sets it from the answer.
    current = null;
  },
});

// ── What the connection is doing ────────────────────────────────────────────
//
// **The head reads the run's connection state**, so when that state moves the
// page is asked for again, exactly as `web/servers.js` asks for the list. **The
// head is not composed from the event**: the event says the value moved, and
// the application composes the head from what the run holds. *(Step 10b's
// first audit round, item 19: this page listened for neither, and the head
// stayed as it was when the page opened.)* **Only for the server that is
// open**, and nothing here ticks.

whenTold(WHAT_IS_OBSERVED, (delivery) => {
  if (!host || !current || delivery.subject !== current) return;
  theScreenIsAskedFor(current);
});

whenTold(THE_CONNECTION_CONDITION, (delivery) => {
  if (!host || !current || delivery.subject !== current) return;
  theScreenIsAskedFor(current);
});

// **What a server is doing moved**, so the head's work column is read again
// for the server that is open. Step 10c's ninth-pass fix piece 2.
whenTold(THE_WORK_MOVED, () => {
  if (!host || !current) return;
  theScreenIsAskedFor(current);
});

/// **P10d repeats the queue reading while a surface showing that server's
/// queue is on screen, and the runner ruled on 2026-09-28 that a server's own
/// page is one** — its head takes the row's work column. So the application is
/// told which server's page is showing, or — with nothing — that none is.
function theWorkColumnIsOnScreen(server) {
  shell.core
    .invoke("the_work_column_is_on_screen", { which: "a-server", server, shown: !!server })
    .catch(() => {});
}

// ── Asking, and drawing what comes back ─────────────────────────────────────

function theScreenIsAskedFor(server) {
  shell.core
    .invoke("the_servers_own_screen", { server })
    .then((answer) => {
      said = answer;
      theScreenIsDrawn(server, answer);
    })
    .catch(() => {
      // **Nothing is drawn here**: the sentence would be the application's
      // (A6, X1) and the application is what did not answer, so there is none
      // to draw and this file may not write one.
      if (host) host.dataset.screen = "failed";
    });
}

/// **A server may have been removed: every server this screen holds is asked
/// for again** — the Creator's eighteenth notes, note 1. `web/servers.js` calls
/// this when a removal's answer arrives, from the rail's menu or the All
/// servers screen's alike, because the removed server's connection and work
/// may never move again and nothing else would ask. **The open server is
/// asked and drawn as any arrival is**, so section 7 says it has been removed.
/// **Any other server whose panels this screen holds is asked quietly**, and
/// only a server the application says is gone is forgotten: nothing is drawn
/// over the server that is open, or over another screen.
export function theServersAreAskedForAgain() {
  if (!host) return;
  if (current) theScreenIsAskedFor(current);
  for (const server of servers.keys()) {
    if (server === current) continue;
    shell.core
      .invoke("the_servers_own_screen", { server })
      .then((answer) => {
        if (answer && !answer.the_head && answer.the_server_is_gone) theServerIsForgotten(server);
      })
      .catch(() => {});
  }
}

function theScreenIsDrawn(server, answer) {
  if (host) delete host.dataset.screen;

  if (!answer.the_head) {
    theConnectionIsDrawnInto(connection, connectionNote, null);
    // **The server is gone, or the record would not read.** Either way this
    // screen says so rather than drawing a server that is not there — L8a.
    //
    // **Only a server that is gone takes its panels with it** (step 10a round
    // 1, fix-here item 24). A read that merely failed hides them, as the tab is
    // hidden, and they come back exactly as they were at the next arrival.
    //
    // **The tab's content goes with the head.** The two tab hosts are one per
    // run and not one per server, so hiding the head alone left whichever tab
    // was showing standing under *This server has been removed.* — which made
    // the sentence false. The showing tab is put away first, while `current`
    // still says which it was, and both hosts are hidden; arriving at a server
    // that is there shows the right one again. The build runner's ruling 3 of
    // 2026-09-13.
    theTabIsGoneWithItsServer();
    if (answer.the_server_is_gone) theServerIsForgotten(server);
    current = null;
    head.hidden = true;
    instead.hidden = false;
    instead.textContent = "";
    const told = document.createElement("p");
    told.className = "sub";
    saidInto(told, answer.instead);
    instead.appendChild(told);
    return;
  }

  instead.hidden = true;
  head.hidden = false;
  current = server;

  theConnectionIsDrawnInto(connection, connectionNote, server);
  const drawn = answer.the_head;
  badge.className = "badge";
  if (drawn.badge) badge.classList.add(drawn.badge);
  badge.hidden = !drawn.badge;
  // The server's name and address read in the order their bytes run (X9,
  // `web/the-value-drawn.js`).
  theValueIsDrawnInto(name, drawn.name);
  theValueIsDrawnInto(address, drawn.address);

  theHardwareIsDrawn(drawn);
  theTabStripIsDrawn(server, answer);
  theStandInIsDrawn(server, answer);
  theServersOwnThingsAreShown(server);
}

/// **The hardware line — the mockup's four spans, each a value inside its own
/// words.** The values go in `<b>`, which is what the drawing bolds.
///
/// **A remembered reading says so**: the *last seen* mark, formatted as the list
/// formats it, and the line drawn stale — `Design/Servers § A row`, `Servers`
/// H1 and `Servers` H1a (step 10a round 1, fix-here item 15). **And the rental line
/// stands on it** — the figure where one exists, `Servers` W40's statement
/// otherwise, never neither (`Servers` W40a; fix-here item 16).
function theHardwareIsDrawn(drawn) {
  hardware.textContent = "";
  const held = drawn.the_hardware;
  hardware.classList.toggle("stale", held.state === "last-seen");
  const one = (part) => {
    if (!part) return;
    const span = document.createElement("span");
    saidInto(span, part);
    hardware.appendChild(span);
  };
  one(held.nothing_to_show);
  one(held.cards);
  one(held.memory);
  one(held.free_disk);
  if (held.last_seen) {
    one({
      lead: held.last_seen.lead,
      values: held.last_seen.values.map(theMomentReads),
      rest: held.last_seen.rest,
    });
  }
  // **What it is rendering**, which is the fourth thing on the drawing's line.
  for (const part of drawn.the_work || []) one(part);
  const rental = drawn.the_rental_line || {};
  one(rental.running_for);
  one(rental.the_statement);
}

// ── The tab strip ───────────────────────────────────────────────────────────

function theTabStripIsDrawn(server, answer) {
  const held = theServer(server);
  if (!held.tab) held.tab = answer.the_tabs[0].tab;

  strip.textContent = "";
  for (const one of answer.the_tabs) {
    const button = document.createElement("button");
    button.type = "button";
    button.className = "tab";
    button.setAttribute("role", "tab");
    button.textContent = one.name;
    button.dataset.tab = one.tab;
    button.setAttribute("aria-selected", String(one.tab === held.tab));
    button.addEventListener("click", () => theCurrentTabIs(one.tab));
    strip.appendChild(button);
  }
  theShowingTabIsShown(server);
}

/// **A tab's host, made once for the whole run** — section 2 above.
function theTabHost(tab) {
  let body = host.querySelector(`[data-tab-host="${tab}"]`);
  if (body) return body;
  body = document.createElement("div");
  // **The drawing's own identifier and its own class**, so every rule the
  // drawing has for that body applies to this element unchanged.
  body.id = tab;
  body.className = tab === FILES ? "files" : "setup";
  body.dataset.tabHost = tab;
  body.setAttribute("role", "tabpanel");
  body.hidden = true;
  // **Before the strips**, so the strips sit along the bottom.
  host.insertBefore(body, instead);
  const registered = tabs.get(tab);
  if (registered) {
    registered.host = body;
    if (registered.itIsDrawn) registered.itIsDrawn(body);
  } else {
    // The mounting contract's section 6, one level down.
    body.dataset.tab = "none";
  }
  return body;
}

function theShowingTab() {
  const held = current && servers.get(current);
  if (!held) return null;
  return tabs.get(held.tab) || null;
}

function theShowingTabIsShown(server) {
  const held = theServer(server);
  for (const tab of [FILES, SET_UP]) {
    const body = theTabHost(tab);
    body.hidden = tab !== held.tab;
  }
  const showing = tabs.get(held.tab);
  if (showing && showing.itIsShowing) showing.itIsShowing(server);
}

/// **Go to the other tab.** Section 2 above. It acts on the server that is open.
export function theCurrentTabIs(tab) {
  if (!current) return;
  const held = theServer(current);
  if (held.tab === tab) return;
  const leaving = tabs.get(held.tab);
  if (leaving && leaving.itIsPutAway) leaving.itIsPutAway();
  held.tab = tab;
  for (const button of strip.querySelectorAll(".tab")) {
    button.setAttribute("aria-selected", String(button.dataset.tab === tab));
  }
  theShowingTabIsShown(current);
}

/// **Which tab a server opens on, said before it is opened** — *Add and set
/// up* closes the dialog onto the new server's Set up tab (`Design/Servers §
/// Adding a server`, the Creator's note 3 on 0.1.7). A server already open
/// goes to that tab now; any other opens on it when the screen draws it.
export function theServerWillOpenOn(server, tab) {
  if (!server) return;
  if (current === server) {
    theCurrentTabIs(tab);
    return;
  }
  theServer(server).tab = tab;
}

/// **A tab says which of the two it fills.** Section 2 above.
export function aTabIsRegistered(tab) {
  tabs.set(tab.tab, { ...tab, host: null });
  // A tab registered after its host was made — a module later in the import
  // list — is drawn now rather than never.
  const body = host && host.querySelector(`[data-tab-host="${tab.tab}"]`);
  if (!body) return;
  delete body.dataset.tab;
  const registered = tabs.get(tab.tab);
  registered.host = body;
  if (registered.itIsDrawn) registered.itIsDrawn(body);
  if (current && servers.get(current) && servers.get(current).tab === tab.tab) {
    if (registered.itIsShowing) registered.itIsShowing(current);
  }
}

// ── The panels ──────────────────────────────────────────────────────────────

function theServer(server) {
  let held = servers.get(server);
  if (held) return held;
  held = {
    drawn: new Map(),
    strip: new Map(),
    reading: new Map(),
    strips: null,
    open: null,
    tab: null,
  };
  servers.set(server, held);
  return held;
}

/// **A panel says which of the two it is.** Section 3 above.
export function aPanelIsRegistered(panel) {
  panels.set(panel.panel, { ...panel });
  if (current && said) theStandInIsDrawn(current, said);
}

/// **Open a panel over a server.** Section 3 above.
export function thePanelIsOpenedOver(server, panel) {
  if (!THE_PANELS.includes(panel)) return;
  const held = theServer(server);
  const parts = thePanelIsMade(server, panel);
  if (!parts) return;
  theOtherPanelLetsGoOfItsFile(server, panel);
  if (held.open && held.open !== panel) held.drawn.get(held.open).wrap.hidden = true;
  held.open = panel;
  parts.wrap.hidden = server !== current;
  theStripsAreDrawn(server);
  // **Focus goes into the panel that just opened**, because the strip it came
  // from has gone and focus would otherwise be nowhere — which would take
  // Escape away from the one shape Design binds it to. The collapse control is
  // where it lands: it is the panel's own way out, and the piece that owns the
  // panel is told next and may move it wherever it likes.
  if (!parts.wrap.hidden && !parts.wrap.contains(document.activeElement)) {
    const out = parts.wrap.querySelector(".collapse");
    if (out) out.focus();
  }
  const registered = panels.get(panel);
  if (registered && registered.itIsShowingAgain) registered.itIsShowingAgain(server);
}

/// **Opening a panel on a different `.blend` moves both panels to that file**
/// — `Design/Whole app § Panels over a server`, after line 139, Creator-stated
/// 2026-10-18 (the twelfth notes, note 8, W5). The other panel, where it is on
/// another file, lets go of it and waits as its fresh strip: its name alone,
/// and pressed, it opens on the new file through its own way in, which starts
/// nothing (section 5's `aFreshStrip`). **Choosing a row in Files moves
/// neither**, because only opening one comes here.
///
/// **Letting go loses nothing** (the runner's ruling): what was typed is the
/// application's, keyed by server and file, and is found again when either
/// panel is opened on the old file. A box still holding the focus is let go
/// of first, so what it holds is handed in as any box's is when it loses the
/// focus. The panel's own module is told nothing and stops nothing, so
/// uploads the check started carry on in Transfers. **Where either file is not
/// known** — a panel that keeps none, or one opened with none — nothing moves.
function theOtherPanelLetsGoOfItsFile(server, panel) {
  const held = theServer(server);
  const file = theFileAPanelIsOn(server, panel);
  if (!file) return;
  for (const other of THE_PANELS) {
    if (other === panel || !held.drawn.has(other)) continue;
    const theirs = theFileAPanelIsOn(server, other);
    if (!theirs || theirs === file) continue;
    const parts = held.drawn.get(other);
    const focused = document.activeElement;
    if (focused && parts.wrap.contains(focused) && typeof focused.blur === "function") focused.blur();
    thePanelIsPutAway(server, other);
  }
}

/// **The file a panel is on, as one comparable text** — its bytes joined, or
/// `null` where the panel keeps no file or holds none. A file handed as text
/// is turned back into the bytes it was written from, as the check does.
function theFileAPanelIsOn(server, panel) {
  const registered = panels.get(panel);
  if (!registered || !registered.theFileItIsOn) return null;
  let file = registered.theFileItIsOn(server);
  if (typeof file === "string") file = Array.from(new TextEncoder().encode(file));
  if (!file || !file.length) return null;
  return Array.from(file).join(",");
}

/// **End a panel.** Section 3 above — the owning piece's, never the user's.
export function thePanelIsPutAway(server, panel) {
  const held = servers.get(server);
  if (!held) return;
  const parts = held.drawn.get(panel);
  if (!parts) return;
  parts.wrap.remove();
  held.drawn.delete(panel);
  held.reading.delete(panel);
  if (held.open === panel) held.open = null;
  theStripsAreDrawn(server);
}

/// **Whether a panel is open over a server** — covering its screen, not
/// collapsed. For a tab that must leave a key alone while a panel covers it
/// (the neighbouring round's must-do 25; the runner's ruling of 2026-09-17
/// gives this line to this file's round).
export function aPanelIsOpenOver(server) {
  const held = servers.get(server);
  return Boolean(held && held.open);
}

/// **What the strip says while the panel is down.** Section 4 above.
export function theStripReads(server, panel, reading) {
  const held = theServer(server);
  held.reading.set(panel, reading || null);
  theStripsAreDrawn(server);
}

function theOpenPanelIsCollapsed(server, focusGoesToTheStrip = true) {
  const held = servers.get(server);
  if (!held || !held.open) return;
  const parts = held.drawn.get(held.open);
  if (parts) parts.wrap.hidden = true;
  held.open = null;
  theStripsAreDrawn(server);
  const back = held.strip.get(parts && parts.panel);
  if (back && focusGoesToTheStrip) back.focus();
}

/// **A link from another screen into this server's Files collapses the panel
/// open over it** — `Design/Whole app § Panels over a server`, added
/// 2026-10-07 (*Files on the server*, *Writing to*): the folder asked for is
/// what shows, and the strip brings the panel back as it was. Section 4's
/// collapse, nothing else: the panel keeps running. **Focus is left alone**,
/// because the user is arriving at Files, not leaving a panel. The Creator's
/// fourth notes, piece O.
export function thePanelOverItIsCollapsed(server) {
  theOpenPanelIsCollapsed(server, false);
}

/// **The file's name in a panel's title collapses the panel as its chevron
/// does** — `Design/Whole app § Panels over a server`, added 2026-10-12 (the
/// Creator's ninth notes, note 5), and the drawing's `.golink` carrying the
/// chevron's own `data-collapse`. Section 4's collapse, and **the strip is
/// unchanged**: it reads what it read, and focus goes to it, because the
/// control pressed was inside the panel and is gone. The panel that owns the
/// name then shows Files on the file's folder.
export function thePanelCollapsesAsItsChevronDoes(server) {
  theOpenPanelIsCollapsed(server);
}

/// **The panel's frame — the mockup's own `.sheetwrap`, `.sheet` and
/// `.shead`.**
///
/// **It is absolute inside the server's own view and never measured against the
/// window.** `#machineview` is `position: relative` in the drawing, so
/// `inset: 0` here is the server's screen.
function thePanelIsMade(server, panel) {
  const held = theServer(server);
  const made = held.drawn.get(panel);
  if (made) return made;
  if (!said || !host) return null;

  const wrap = document.createElement("div");
  wrap.className = "sheetwrap";
  wrap.hidden = true;
  wrap.dataset.panel = panel;
  wrap.dataset.server = server;

  // **No role of its own.** A panel is not a box: it does not take the
  // window, and Escape collapses it rather than ending it. A box a piece draws
  // *over* the panel carries `role="dialog"`, and that is what the Escape
  // handler above steps aside for.
  const sheet = document.createElement("div");
  sheet.className = "sheet";
  wrap.appendChild(sheet);

  const shead = document.createElement("div");
  shead.className = "shead";
  const title = document.createElement("div");
  title.className = "htitle";
  shead.appendChild(title);
  const grow = document.createElement("span");
  grow.className = "grow";
  shead.appendChild(grow);
  shead.appendChild(theCollapseControl(server));
  sheet.appendChild(shead);

  const body = document.createElement("div");
  body.className = "suscroll";
  sheet.appendChild(body);

  host.appendChild(wrap);
  const parts = { panel, wrap, head: title, body };
  held.drawn.set(panel, parts);

  const registered = panels.get(panel);
  if (registered) {
    if (registered.itIsDrawn) registered.itIsDrawn({ head: title, body }, server);
  } else {
    // **The stand-in, and pieces 8 and 9 remove it by registering.** The frame
    // is real; what is inside it is one sentence from the application saying
    // the panel is not built.
    wrap.dataset.panel = "none";
    const heading = document.createElement("h2");
    heading.textContent = theNameOf(panel);
    title.appendChild(heading);
    const told = document.createElement("p");
    told.className = "sub";
    told.textContent = said.the_stand_in;
    body.appendChild(told);
  }
  return parts;
}

/// **A chevron pinned to the panel's top corner.** Creator-stated, and the
/// drawing's own symbol — made here rather than referred to, because a symbol
/// definition would be markup in `index.html` and a piece adds nothing to that
/// file.
function theCollapseControl(server) {
  const control = document.createElement("button");
  control.type = "button";
  control.className = "collapse";
  control.title = said.collapse;
  control.setAttribute("aria-label", said.collapse);
  const mark = document.createElementNS(SVG_NS, "svg");
  mark.setAttribute("viewBox", "0 0 16 16");
  mark.setAttribute("aria-hidden", "true");
  mark.setAttribute("fill", "none");
  mark.setAttribute("stroke", "currentColor");
  mark.setAttribute("stroke-width", "1.6");
  mark.setAttribute("stroke-linecap", "round");
  mark.setAttribute("stroke-linejoin", "round");
  const path = document.createElementNS(SVG_NS, "path");
  path.setAttribute("d", "M4 6.2 8 10.2l4-4");
  mark.appendChild(path);
  control.appendChild(mark);
  control.addEventListener("click", () => theOpenPanelIsCollapsed(server));
  return control;
}

// ── The strips ──────────────────────────────────────────────────────────────

function theStripsAreDrawn(server) {
  const held = theServer(server);
  if (!held.strips) {
    held.strips = document.createElement("div");
    held.strips.className = "strips";
    held.strips.dataset.server = server;
    host.appendChild(held.strips);
  }
  const focused = held.strips.contains(document.activeElement)
    ? document.activeElement.dataset.panel
    : null;
  held.strips.textContent = "";
  held.strip.clear();
  // **Section 5: with a panel open the strips sit in its frame, after its
  // sheet, and carry only the other panel**; with none open they sit along the
  // bottom of the server's screen. Either way in the drawing's own order
  // rather than the order they were collapsed in. **A panel never opened over
  // this server has its strip only while the other is open**, and it reads
  // its name alone (the Creator's seventh notes, note 2).
  const open = held.open && held.drawn.get(held.open);
  const into = open ? open.wrap : host;
  if (held.strips.parentNode !== into) into.appendChild(held.strips);
  for (const panel of THE_PANELS) {
    if (panel === held.open) continue;
    const fresh = !held.drawn.has(panel);
    if (fresh && !open) continue;
    const one = fresh
      ? aFreshStrip(server, panel)
      : aStrip(server, panel, held.reading.get(panel));
    held.strip.set(panel, one);
    held.strips.appendChild(one);
  }
  held.strips.hidden = held.strips.childElementCount === 0 || server !== current;
  if (focused && held.strip.has(focused)) held.strip.get(focused).focus();
}

function aStrip(server, panel, reading) {
  const one = document.createElement("button");
  one.type = "button";
  one.className = "strip";
  one.dataset.panel = panel;

  const title = document.createElement("span");
  title.className = "t";
  // **What the panel last said, or its own name where it has said nothing.**
  if (reading && reading.title) saidInto(title, reading.title);
  else title.textContent = theNameOf(panel);
  one.appendChild(title);

  if (reading && reading.reading) {
    const counting = document.createElement("span");
    counting.className = "d";
    // **The strip's values are set apart with the drawing's own separator** —
    // *Interior · 240 frames* — rather than a bare space (the runner's ruling
    // of 2026-09-17, from the neighbouring round's Verdict).
    // **A reading in parts** — *Uploading 4 of 12*, words between two figures
    // — is its parts side by side, each drawn the same way (the dependency
    // check's strip, the Creator's note 6 on 0.1.8).
    if (Array.isArray(reading.reading)) {
      reading.reading.forEach((part, at) => {
        if (at > 0) counting.appendChild(document.createTextNode(" "));
        saidInto(counting, part, THE_STRIPS_SEPARATOR);
      });
    } else {
      saidInto(counting, reading.reading, THE_STRIPS_SEPARATOR);
    }
    one.appendChild(counting);
  }

  const open = document.createElement("span");
  open.className = "open";
  open.textContent = said.open;
  one.appendChild(open);

  one.addEventListener("click", () => thePanelIsOpenedOver(server, panel));
  return one;
}

/// **The strip of a panel never opened over this server**: its name alone —
/// the drawing's `.strip.fresh` — and pressing it opens that panel on the open
/// panel's file, through the panel's own way in that starts nothing.
function aFreshStrip(server, panel) {
  const one = document.createElement("button");
  one.type = "button";
  one.className = "strip fresh";
  one.dataset.panel = panel;
  const title = document.createElement("span");
  title.className = "t";
  title.textContent = theNameOf(panel);
  one.appendChild(title);
  const open = document.createElement("span");
  open.className = "open";
  open.textContent = said.open;
  one.appendChild(open);
  one.addEventListener("click", () => theNeverOpenedPanelIsOpened(server, panel));
  return one;
}

function theNeverOpenedPanelIsOpened(server, panel) {
  const held = theServer(server);
  const from = held.open && panels.get(held.open);
  const file = from && from.theFileItIsOn ? from.theFileItIsOn(server) : null;
  const registered = panels.get(panel);
  if (registered && registered.itIsShownOn) {
    Promise.resolve(registered.itIsShownOn(server, file || [])).catch(() => {});
  } else {
    // A panel not yet built opens as its stand-in, which starts nothing.
    thePanelIsOpenedOver(server, panel);
  }
}

// ── Going away and coming back ──────────────────────────────────────────────

/// **Show this server's own things and hide every other server's** — section 6.
function theServersOwnThingsAreShown(server) {
  for (const [which, held] of servers) {
    const here = which === server;
    for (const [panel, parts] of held.drawn) {
      parts.wrap.hidden = !here || held.open !== panel;
    }
    if (held.strips) held.strips.hidden = !here;
  }
  theStripsAreDrawn(server);
}

/// **A removed server's screen shows no tab** — section 7, and the build
/// runner's ruling 3 of 2026-09-13. **The hosts are hidden and never taken
/// down**, because they are one per run and the next server that is there uses
/// them again; the tab that was showing is told it has stopped, so a menu or a
/// box of its own does not stand behind the sentence either.
function theTabIsGoneWithItsServer() {
  const showing = theShowingTab();
  if (showing && showing.itIsPutAway) showing.itIsPutAway();
  for (const tab of [FILES, SET_UP]) {
    const body = host.querySelector(`[data-tab-host="${tab}"]`);
    if (body) body.hidden = true;
  }
  // **And no other server's panel or strip stands over the sentence either.**
  // Watched while carrying out ruling 3: a panel left open over the server the
  // user came from stayed drawn over *This server has been removed.* They are
  // hidden and not taken down — section 6 — and arriving back at their own
  // server shows them again exactly as they were.
  for (const held of servers.values()) {
    for (const parts of held.drawn.values()) parts.wrap.hidden = true;
    if (held.strips) held.strips.hidden = true;
  }
}

/// **A server that has been removed takes its panels with it** — section 7.
function theServerIsForgotten(server) {
  const held = servers.get(server);
  if (!held) return;
  for (const parts of held.drawn.values()) parts.wrap.remove();
  if (held.strips) held.strips.remove();
  servers.delete(server);
}

// ── The one place a value becomes a node ────────────────────────────────────

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
/// by side and joins nothing**, which is the page half of X7. The values are
/// bolded, which is what the drawing does on this line.
function saidInto(holder, part, betweenValues) {
  if (!part) return holder;
  let first = true;
  let lastWasAValue = false;
  const next = (node, isAValue) => {
    if (!first) {
      const between = isAValue && lastWasAValue && betweenValues ? betweenValues : " ";
      holder.appendChild(document.createTextNode(between));
    }
    first = false;
    lastWasAValue = Boolean(isAValue);
    holder.appendChild(node);
  };
  if (part.lead) {
    const lead = document.createElement("span");
    lead.textContent = part.lead;
    next(lead);
  }
  for (const value of part.values || []) {
    const one = document.createElement("b");
    theValueIsDrawnInto(one, value);
    next(one, true);
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

// The drawing's own separator between two figures on a strip — punctuation,
// with the spaces either side of it, and not a word.
const THE_STRIPS_SEPARATOR = " · ";

function theNameOf(panel) {
  const found = said && said.the_panels.find((one) => one.panel === panel);
  return found ? found.name : "";
}

// ════════════════════════════════════════════════════════════════════════════
// A STAND-IN, AND WHAT REMOVES IT
// ════════════════════════════════════════════════════════════════════════════
//
// **Pieces 8 and 9 own what is inside the two panels, and neither exists.** So
// the head carries one control per panel nothing has registered, opening that
// panel over the server that is open. **Registering the panel takes its control
// away** — so piece 8 removes the first and piece 9 the second, and neither has
// to edit this file or even know the control was there.
//
// **It is not Design's.** `Design/File Browser § Where it sits` settles that
// choosing a `.blend` opens the render settings, and the check is reached from
// the settings; **neither is a control on the head.** This block is here so the
// mechanism above can be watched by a person before those two routes exist, and
// it is the same shape `web/panes.js` and `web/said-once.js` used before the
// screens that replaced them.

function theStandInIsDrawn(server, answer) {
  standIn.textContent = "";
  for (const panel of answer.the_panels) {
    if (panels.has(panel.panel)) continue;
    const control = document.createElement("button");
    control.type = "button";
    control.className = "mini";
    control.textContent = panel.name;
    control.dataset.standIn = panel.panel;
    control.addEventListener("click", () => thePanelIsOpenedOver(server, panel.panel));
    standIn.appendChild(control);
  }
  standIn.hidden = standIn.childElementCount === 0;
}

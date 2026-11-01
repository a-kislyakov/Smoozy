// **The app's shell: the rail, the main pane, and how a screen is mounted.**
//
// **READ THIS BEFORE WRITING A SCREEN. It is the mounting contract for the
// eleven screen pieces of step 10a, and it is written here rather than only in
// the decisions record because this is the file somebody opens when they need
// it.** Piece A did the same at the head of `web/smoozy.js` and the eight
// pieces after it merged; this is that habit kept, one level up.
//
// ═══════════════════════════════════════════════════════════════════════════
// THE MOUNTING CONTRACT
// ═══════════════════════════════════════════════════════════════════════════
//
// ## 1. There are seven places and there will not be an eighth
//
// They are the mockup's own seven views, and **the name a screen registers
// under is that view's own identifier**:
//
//     serversview   All servers
//     machineview   a server — one place, whichever server is current
//     queueview     Queue
//     historyview   History
//     transfersview Transfers
//     keysview      Keys
//     stgview       Settings
//
// `Preparation/Design/Mockup/Smoozy.html`'s own `setState` toggles exactly those
// seven sibling subtrees, and the drawing declares rules against five of the
// identifiers by name. **So the host element this file makes for a screen
// carries that identifier**, and every rule the drawing already has applies to
// it unchanged. `src/surface/the_rail.rs` holds the same seven and a test there
// reads the mockup and fails if the set has moved.
//
// **Import the name, never type it.** The seven are exported below.
//
// ## 2. How a screen registers itself
//
//     import { aScreenIsRegistered, THE_QUEUE } from "./the-rail.js";
//
//     aScreenIsRegistered({
//       place: THE_QUEUE,
//       itIsDrawn(host) { … },      // once, the first time the user goes there
//       itIsCurrentNow(which) { },  // optional, every time, the first included
//       itIsPutAway() { },          // optional, every time it stops being current
//     });
//
// Register at the top level of your own module, the way every other module here
// starts itself. **Registering does not draw anything** and does not ask the
// application for anything.
//
// ## 3. Built once, then hidden — never destroyed
//
// **`itIsDrawn` is called once, the first time the user goes to that place**, and
// the host it is given is yours for the life of the run. Going away sets
// `hidden` on it and going back clears it. **Nothing is ever taken down.**
//
// - **Hidden rather than destroyed is the drawing's own answer.** The mockup
//   holds all seven subtrees at once and toggles `hidden`; nothing in it is
//   built or thrown away.
// - **Drawn on the first visit rather than at start-up is this file's**, and the
//   reason is that a screen's drawing asks the application for facts. Drawing
//   all seven at start would make seven modules ask for everything before the
//   user has looked at anything, which is work nobody asked for. **Once drawn it
//   is kept, so the cost is paid once.**
// - **What that buys, and it is the reason to say it out loud:** a screen's
//   scroll position, its half-typed field, its open menu and its sort order all
//   survive going away and coming back, **for nothing** — they live in the
//   element, which is still there. **Do not remember any of them yourself.**
//
// ## 4. What is current, and how you are told
//
// One place is current at a time. Its host has `hidden` cleared and its rail row
// carries `aria-current="true"`, which is the drawing's own mark for it.
//
// - **`itIsCurrentNow(which)`** is called every time the place becomes current,
//   the first time included, **after** `itIsDrawn`. `which` is the server's own
//   identity for `A_SERVER` and undefined for the other six. **Use it to ask the
//   application again** — events are volatile and nothing is replayed (P8), so a
//   screen that has been away has facts it should re-read.
// - **`itIsPutAway()`** is called every time it stops being current. Use it to
//   shut a menu or drop a listener. **Do not use it to save anything**: the
//   element stays and so does everything in it.
// - Neither is required. A screen that draws once and needs nothing back leaves
//   both out.
//
// ## 5. Going somewhere
//
//     theCurrentPlaceIs(A_SERVER, theServersIdentity);
//
// That is the only way. A rail press calls it, a Go menu item calls it through
// this file's own listener, and a screen sending the user somewhere calls it
// too — Servers opening a server, Transfers linking to Settings. **Going to the
// place that is already current with the same `which` does nothing**, so a
// screen may call it freely.
//
// ## 6. A place with no screen registered
//
// **The rail always draws all seven places**, because the places are Design's
// fixed list and not a function of what has been built. Going to one nothing has
// registered leaves the main pane empty and marks it
// `data-screen="none"`. **That is a state of this stage and not of the
// application**: every place has a screen before anybody runs Smoozy, and step
// 10a is not finished until all seven are registered.
//
// ## 7. Panes, widths and seams
//
// **You do not build narrowing and you do not remember a width.** `web/panes.js`
// writes the step mark onto `.app` at the five boundaries and exports
// `theSeamsAreLive`, and the pane-width record already holds the three seams.
// **The rail's seam is one of the three and this file makes it live.** A screen
// with a seam of its own — the queue's list, a job entry's preview column —
// makes a `button.seam` carrying `data-seam="list"` or `data-seam="preview"`,
// asks the application for `the_pane_layout`, and calls `theSeamsAreLive` with
// its own root. **There is no fourth seam.**
//
// ## 8. What the shell does not do
//
// - **It states no failure.** The rail is navigation and it writes nothing
//   anywhere, so a read that would not answer costs nothing here; the screen
//   the user goes to is where that is said. `src/surface/the_rail.rs`'s head
//   carries the whole reasoning.
// - **It draws no screen's emptiness.** A queue with no jobs and a server list
//   with no servers are those screens' own states and the drawing has them.
// - **It holds no figure of its own but one.** The rail row's count beside
//   Queue, History and Transfers belongs to the screen behind it and arrives
//   through `theRowsFigureIs`. The figure beside All servers is how many
//   servers the rail itself drew, so it comes with the rail.
//
// ═══════════════════════════════════════════════════════════════════════════
//
// **And the four things that bind whatever a piece writes** are at the head of
// `web/smoozy.js`: results arrive as events; values are content and never
// markup (H32); no inline script and no inline style (H33); nothing repeats on
// a timer (P9, P10). This file keeps all four.

import { whenTold } from "./event-route.js";
import { theSeamsAreLive } from "./panes.js";
import { theValueIsDrawnInto } from "./the-value-drawn.js";

const shell = window.__TAURI__;
const SVG_NS = "http://www.w3.org/2000/svg";

// **The seven places.** The same strings `src/surface/the_rail.rs` holds, and a
// test there fails if the two have drifted.
export const ALL_SERVERS = "serversview";
export const A_SERVER = "machineview";
export const THE_QUEUE = "queueview";
export const THE_HISTORY = "historyview";
export const TRANSFERS = "transfersview";
export const KEYS = "keysview";
export const SETTINGS = "stgview";

// The kind, declared beside the service that posts it —
// `THE_SPEED_TRANSFERS_ARE_RUNNING_AT` in `src/surface/the_rail.rs`, and
// `THE_USER_CHOSE_A_GOING` in `src/surface/menus.rs`. The two have to agree.
const THE_SPEED = "the-rail:the-speed";
const A_GOING = "menus:a-going";
// `WHAT_IS_OBSERVED` in `src/servers/published.rs`, which `web/servers.js` and
// `web/a-server.js` hear too. The two sides have to agree, and a test in
// `src/surface/the_rail.rs` fails if they do not.
const WHAT_IS_OBSERVED = "servers:what-is-observed";
// `THE_WORK_MOVED` in `src/surface/servers.rs`, posted only when what a server
// is doing reads differently — which is what moves a row's dot. `web/servers.js`
// and `web/a-server.js` hear it too, and a test in `src/surface/the_rail.rs`
// fails if this side stops naming it.
const THE_WORK_MOVED = "servers:the-work-moved";

// ── What this module holds ──────────────────────────────────────────────────

// place → { itIsDrawn, itIsCurrentNow, itIsPutAway, host }
const screens = new Map();
// place → the rail row that lights when it is current. The server rows are
// keyed by their own identity instead, in `serverRows`.
const rows = new Map();
const serverRows = new Map();

let main = null;
let current = null;
let currentServer = null;
// The going the application posted before the rail had drawn itself. The spine
// runs before the window exists, so a menu chosen that early would otherwise be
// lost.
let waiting = null;

// ── The shell itself ────────────────────────────────────────────────────────

// **Made at load and never on an event.** The window's own shape is not a
// screen: it must not wait on anything a module posts, or the window stands
// empty until one does.
const theRail = shell.core.invoke("the_rail");

theRail.then(theShellIsDrawn).catch(() => {
  // **The window is left standing.** Rule 8 makes Smoozy going quiet a fault
  // however the trouble arose, and a window with no rail can still be reported.
  // **Nothing is drawn here**: the sentence would come from the application
  // (A6, X1) and the application is what did not answer, so there is none to
  // draw and this file may not write one.
  document.documentElement.dataset.shell = "failed";
});

function theShellIsDrawn(rail) {
  const pane = document.getElementById("pane");
  if (!pane) return;

  const body = document.createElement("div");
  body.className = "body";

  // The rail's own seam, which is one of the three the pane-width record
  // already holds. Its two stops, the property it moves and its words all come
  // from the application; this file names only which seam it is.
  shell.core
    .invoke("the_pane_layout")
    .then((layout) => {
      const seam = document.createElement("button");
      seam.type = "button";
      seam.className = "seam";
      seam.dataset.seam = "rail";
      const declared = layout.seams.find((one) => one.key === "rail");
      if (!declared) return;
      // Prose, through the typography point. Nothing here composes a sentence.
      seam.setAttribute("aria-label", declared.label);
      seam.title = declared.hint;
      body.insertBefore(seam, body.firstChild);
      theSeamsAreLive(body, layout.seams);
    })
    .catch(() => {
      // The widths are `panes.js`'s to speak about and it already does. A rail
      // without a draggable seam is still a rail.
    });

  body.appendChild(theRailIsDrawn(rail));

  main = document.createElement("div");
  main.className = "main";
  body.appendChild(main);

  // **This module appends its own element and empties nothing.** The page's
  // layout rule is that every piece makes its own elements inside the pane, and
  // emptying the pane reaches outside this module's own.
  pane.appendChild(body);

  theCurrentPlaceIs(waiting || rail.opens_at);
  waiting = null;
}

// ── The rail ────────────────────────────────────────────────────────────────

function theRailIsDrawn(rail) {
  const nav = document.createElement("nav");
  nav.className = "rail";
  // Prose. It came through the typography point and this file substitutes
  // nothing.
  nav.setAttribute("aria-label", rail.the_rails_own_name);

  rows.clear();
  serverRows.clear();

  // Section 0 — the servers. The heading, All servers, then one row per server.
  const servers = document.createElement("div");
  servers.className = "rsec";
  const heading = document.createElement("div");
  heading.className = "rhd";
  const said = document.createElement("span");
  said.textContent = rail.the_servers_heading;
  heading.appendChild(said);
  servers.appendChild(heading);

  for (const place of rail.places) {
    if (place.section === 0) servers.appendChild(aFixedRow(place));
  }
  for (const server of rail.servers) servers.appendChild(aServerRow(server));
  nav.appendChild(servers);

  // Sections 1 and 2, each behind a divider. Design fixes the grouping: the
  // places that belong to no server, then Keys and Settings.
  for (const section of [1, 2]) {
    const divider = document.createElement("div");
    divider.className = "rdiv";
    nav.appendChild(divider);
    const held = document.createElement("div");
    held.className = "rsec";
    for (const place of rail.places) {
      if (place.section === section) held.appendChild(aFixedRow(place));
    }
    nav.appendChild(held);
  }

  return nav;
}

function aFixedRow(place) {
  const row = document.createElement("button");
  row.type = "button";
  row.className = "item";
  // Prose, both of them, and the name on hover is the fourth narrowing step's
  // whole answer: "an icon at the top for All servers … each with its name on
  // hover".
  row.title = place.name;

  const mark = theMark(place.place);
  if (mark) row.appendChild(mark);

  const label = document.createElement("span");
  label.className = "lbl";
  label.textContent = place.name;
  row.appendChild(label);

  if (place.mark) theQueuesMarkIsDrawn(row, label, place);

  const figure = document.createElement("span");
  figure.className = "tl mono";
  // A value, by the verbatim route, and never joined into the name above (X7).
  figure.textContent = place.count || "";
  figure.hidden = !place.count;
  row.appendChild(figure);

  row.addEventListener("click", () => theCurrentPlaceIs(place.place));
  rows.set(place.place, { row, figure });
  return row;
}

/// **The rail's Queue says a job there needs your answer** — `Design/Whole
/// app § The server badge`, the Creator's twenty-fourth notes, note 2: the
/// drawing's `.qask`, one still dot just after the word, the label made
/// `.lbl.fit` so the dot sits by it and the count stays at the row's end. **The
/// application decides whether it shows** (`src/surface/the_rail.rs`,
/// `a_job_in_the_queue_needs_your_answer`); hidden the moment no job reads
/// *Needs your answer*. **While it shows, the row's hover name is the
/// application's *Queue — needs your answer***, which is also how the row is
/// described aloud; the dot itself is not read, so nothing is said twice.
function theQueuesMarkIsDrawn(row, label, place) {
  label.classList.add("fit");
  const mark = document.createElement("span");
  mark.className = "qask";
  mark.setAttribute("aria-hidden", "true");
  mark.hidden = !place.mark.on;
  row.appendChild(mark);
  if (place.mark.on) row.title = place.mark.hover;
}

function aServerRow(server) {
  const row = document.createElement("button");
  row.type = "button";
  row.className = "item";
  // **`Design/Whole app § How state is shown`: a server that is not connected
  // reads dim, the whole row and the badge with it.** The drawing's own
  // `.item.away` rules do the dimming; the application says which rows.
  if (server.away) row.classList.add("away");
  // A value: the server's own name, which the user can copy. It never goes
  // through the typography point and is never joined into a sentence (X7).
  row.title = server.name;

  if (server.badge) {
    const badge = document.createElement("span");
    // The eight are the drawing's and the application checked this one against
    // them, so nothing unvalidated becomes a class here. **Two names added
    // rather than one string joined**: a class list is the one place the page
    // may put a value, and joining it into a string is the shape X7 forbids
    // everywhere else.
    badge.classList.add("badge", server.badge);
    row.appendChild(badge);
  }

  const label = document.createElement("span");
  label.className = "lbl";
  // Read in the order its bytes run (X9, `web/the-value-drawn.js`). The title
  // above is drawn by the system and cannot be kept this way.
  theValueIsDrawnInto(label, server.name);
  row.appendChild(label);

  // **The handle a second module finds this row by, and it is the server's own
  // identity rather than its name.** The title above is a value the user can
  // copy, and two servers may share one; an identity is the only thing that
  // picks out a row the day they do.
  row.dataset.server = server.server;
  row.setAttribute("aria-haspopup", "menu");
  if (server.dot) row.appendChild(theDotIsDrawn(server.dot));

  row.addEventListener("click", () => theCurrentPlaceIs(A_SERVER, server.server));
  const menu = (press) => {
    press.preventDefault();
    document.dispatchEvent(new CustomEvent("servers:menu-requested", {
      detail: { server: server.server, beside: row },
    }));
  };
  row.addEventListener("contextmenu", menu);
  row.addEventListener("keydown", (press) => {
    if (press.key === "ContextMenu" || (press.shiftKey && press.key === "F10")) menu(press);
  });
  serverRows.set(server.server, row);
  return row;
}

/// **The row's one dot says what the server is doing** — `Design/Whole app §
/// The server badge`, the Creator's note 19 of 2026-10-04: the drawing's
/// `.rdot`, green while connected and not rendering, and `.rdot.run`, amber
/// with the pulse a rendering job already has, while a job renders, and
/// `.rdot.trying`, the connect control's own reconnecting dot, still, while the
/// server reconnects (the Creator's seventh notes, note 10). **The
/// application decides which, and whether there is a dot at all** — a server
/// that is not connected has none (`src/surface/the_rail.rs`, `the_dot`).
///
/// **Last in the row, which is where the drawing puts it** — after the badge
/// and the name — and where its narrowest-rail rule lifts it onto the badge's
/// corner. **The open-panel mark that sat here until 2026-10-04 is gone**: an
/// open panel is said by its own strip.
///
/// **Its word is the row's to read aloud, and its hover word below.** A
/// labelled image is a child a button's name is computed from, so the row reads
/// as the server's name followed by *Connected*, *Rendering* or *Reconnecting…* — two nodes side
/// by side and nothing joined (X7), the shape the build runner's ruling 2 of
/// 2026-09-13 gave the mark before it. The word is the application's, through
/// the typography point; this file substitutes nothing.
function theDotIsDrawn(dot) {
  const mark = document.createElement("span");
  mark.className = "rdot";
  if (dot.rendering) mark.classList.add("run");
  if (dot.reconnecting) mark.classList.add("trying");
  mark.setAttribute("role", "img");
  mark.setAttribute("aria-label", dot.said);
  // **Its hover word is the same word**, the application's, so a pointer
  // resting on the dot reads what it says. The row's own title stays the
  // server's name alone: a value is never joined to a word here (X7).
  mark.title = dot.said;
  return mark;
}

// **The six marks, and their one home is the mockup.** The drawing sizes them
// with `.ric` and colours them with the current row's own colour; the shapes
// are the mockup's own symbols, copied the way `the-drawing.css` is copied.
// **They are made here rather than referred to**: a symbol definition would be
// markup in `index.html`, and a piece adds nothing to that file.
const THE_MARKS = {
  serversview: {
    fill: "currentColor",
    shapes: [
      ["rect", { x: "2.2", y: "2.2", width: "5.2", height: "5.2", rx: "1.6" }],
      ["rect", { x: "8.6", y: "2.2", width: "5.2", height: "5.2", rx: "1.6" }],
      ["rect", { x: "2.2", y: "8.6", width: "5.2", height: "5.2", rx: "1.6" }],
      ["rect", { x: "8.6", y: "8.6", width: "5.2", height: "5.2", rx: "1.6" }],
    ],
  },
  queueview: {
    stroke: "1.4",
    cap: "round",
    shapes: [
      ["path", { d: "M2.4 4.2h11.2" }],
      ["path", { d: "M2.4 8h11.2" }],
      ["path", { d: "M2.4 11.8h7" }],
    ],
  },
  historyview: {
    stroke: "1.3",
    cap: "round",
    join: "round",
    shapes: [
      ["path", { d: "M2.6 8a5.4 5.4 0 1 0 1.6-3.8" }],
      ["path", { d: "M2.4 2.9v3h3" }],
      ["path", { d: "M8 5.3V8l2 1.4" }],
    ],
  },
  transfersview: {
    stroke: "1.4",
    cap: "round",
    join: "round",
    shapes: [
      ["path", { d: "M4.6 12.6V3.9" }],
      ["path", { d: "M2.2 6.3l2.4-2.4L7 6.3" }],
      ["path", { d: "M11.4 3.4v8.7" }],
      ["path", { d: "M13.8 9.7l-2.4 2.4L9 9.7" }],
    ],
  },
  keysview: {
    stroke: "1.3",
    cap: "round",
    join: "round",
    shapes: [
      ["circle", { cx: "5.4", cy: "5.4", r: "3.1" }],
      ["path", { d: "M7.7 7.7l5.9 5.9" }],
      ["path", { d: "M11.4 11.4l1.4-1.4" }],
    ],
  },
  // A wrench, not a gear: at sixteen pixels a gear's teeth close up into a
  // disc, and a disc with rays is a sun. The mockup says so at the symbol.
  stgview: {
    stroke: "1.25",
    join: "round",
    shapes: [
      [
        "path",
        {
          d: "M12.9 2.3a3.5 3.5 0 0 0-4.5 4.4L2.9 12.2a1.7 1.7 0 0 0 2.4 2.4l5.5-5.5a3.5 3.5 0 0 0 4.4-4.5l-2.2 2.2-1.9-.6-.6-1.9z",
        },
      ],
    ],
  },
};

function theMark(place) {
  const drawn = THE_MARKS[place];
  if (!drawn) return null;
  const mark = document.createElementNS(SVG_NS, "svg");
  mark.setAttribute("class", "ric");
  mark.setAttribute("viewBox", "0 0 16 16");
  mark.setAttribute("aria-hidden", "true");
  mark.setAttribute("fill", drawn.fill || "none");
  if (drawn.stroke) {
    mark.setAttribute("stroke", "currentColor");
    mark.setAttribute("stroke-width", drawn.stroke);
  }
  if (drawn.cap) mark.setAttribute("stroke-linecap", drawn.cap);
  if (drawn.join) mark.setAttribute("stroke-linejoin", drawn.join);
  for (const [shape, attributes] of drawn.shapes) {
    const drawnShape = document.createElementNS(SVG_NS, shape);
    for (const [name, value] of Object.entries(attributes)) {
      drawnShape.setAttribute(name, value);
    }
    mark.appendChild(drawnShape);
  }
  return mark;
}

// ── Registering, mounting and going ─────────────────────────────────────────

/// **A screen says which place it fills.** Section 2 of the contract above.
export function aScreenIsRegistered(screen) {
  screens.set(screen.place, { ...screen, host: null });
  // A screen registered after the user is already standing in its place — a
  // module loaded later in the import list — is drawn now rather than on the
  // next visit.
  if (current === screen.place) theScreenIsMounted(screen.place);
}

/// **Go to a place.** Section 5 of the contract above.
export function theCurrentPlaceIs(place, which) {
  if (!main) {
    // The shell has not drawn yet. Remember where we were asked to go.
    waiting = place;
    return;
  }
  if (!screens.has(place) && !THE_PLACES.includes(place)) return;
  if (current === place && currentServer === which) return;

  if (current !== null && current !== place) {
    const leaving = screens.get(current);
    if (leaving && leaving.host) leaving.host.hidden = true;
    if (leaving && leaving.itIsPutAway) leaving.itIsPutAway();
  }

  current = place;
  currentServer = which;
  theRailShowsWhereWeAre();
  theScreenIsMounted(place, which);
}

const THE_PLACES = [
  ALL_SERVERS,
  A_SERVER,
  THE_QUEUE,
  THE_HISTORY,
  TRANSFERS,
  KEYS,
  SETTINGS,
];

function theScreenIsMounted(place, which) {
  const screen = screens.get(place);
  if (!screen) {
    main.dataset.screen = "none";
    return;
  }
  main.dataset.screen = place;
  if (!screen.host) {
    const host = document.createElement("div");
    // **The mockup's own identifier**, so the drawing's rules for this view
    // apply to it unchanged. Section 1 of the contract.
    host.id = place;
    main.appendChild(host);
    screen.host = host;
    if (screen.itIsDrawn) screen.itIsDrawn(host);
  }
  screen.host.hidden = false;
  if (screen.itIsCurrentNow) screen.itIsCurrentNow(which);
}

function theRailShowsWhereWeAre() {
  for (const [place, held] of rows) {
    if (place === current) held.row.setAttribute("aria-current", "true");
    else held.row.removeAttribute("aria-current");
  }
  for (const [server, row] of serverRows) {
    const here = current === A_SERVER && server === currentServer;
    if (here) row.setAttribute("aria-current", "true");
    else row.removeAttribute("aria-current");
  }
}

/// **Which place is current**, for a screen that must know whether the user is
/// still where it sent them — a server dialog's key pick ending as a cancel
/// because the user went elsewhere (step 10a round 1, fix-here item 2).
export function theCurrentPlace() {
  return current;
}


/// **The figure beside a rail row**, which belongs to the screen behind it.
/// A value, already through the verbatim route — an empty one takes it away.
export function theRowsFigureIs(place, figure) {
  const held = rows.get(place);
  if (!held) return;
  held.figure.textContent = figure || "";
  held.figure.hidden = !figure;
}

/// **Read the rail again**, for whoever adds, renames, recolours or removes a
/// server. The rail is the only way to reach a server, so a rail that has gone
/// stale is a server the user cannot get to.
export function theRailIsReadAgain() {
  shell.core.invoke("the_rail").then((rail) => {
    const nav = document.querySelector(".rail");
    if (!nav || !nav.parentNode) return;
    // **The figures the screens set are kept across the redraw** — Queue's,
    // History's and Transfers' arrive through `theRowsFigureIs` and not with
    // the rail, so a redraw would otherwise take them away until each screen
    // spoke again. The drawing's `rail-ask` keeps Queue's count beside the dot
    // (the twenty-fourth notes, note 2).
    const kept = new Map();
    for (const [place, held] of rows) {
      if (!held.figure.hidden) kept.set(place, held.figure.textContent);
    }
    nav.parentNode.replaceChild(theRailIsDrawn(rail), nav);
    for (const [place, figure] of kept) {
      if (!rail.places.some((one) => one.place === place && one.count)) theRowsFigureIs(place, figure);
    }
    theRailShowsWhereWeAre();
  });
}

// **A server's row follows its connection.** `Design/Whole app § How state is
// shown`: a server that is not connected reads dim, so a row has to change when
// a connect or a disconnect lands. **The event says the value moved and the
// rail is read again**, so which rows are dim is composed in one place, from
// what the run holds, exactly as it is at load — the same route an added or
// removed server takes. Heard whatever screen is showing, because the rail is
// on every one of them.
whenTold(WHAT_IS_OBSERVED, () => {
  theRailIsReadAgain();
});

// **A server's dot follows its work.** What each server is doing is composed
// in the application from the work column's own reading, and the application
// says when it moved; the rail is read again, the same route as above, so the
// dot moves the moment the application learns a job started or ended. **It
// learns a start only from a queue reading**, which repeats only while a
// surface showing the queue is on screen (P10d) — the dot is never fresher
// than the work column.
whenTold(THE_WORK_MOVED, () => {
  theRailIsReadAgain();
});

// **A Go menu item and a rail press reach the same place.** `web/menus.js` says
// in its own words that the first screen replaces its stand-in listener, and
// this is that listener: the application posts which item was chosen and the
// shell goes there. Six of the menus' ten goings arrive here: *Up one folder*
// is Files' own, and *About Smoozy*, *Check for updates…* and *Report a
// problem…* open the menus' own box in `web/menus.js` (the 38, piece 12).
whenTold(A_GOING, (delivery) => {
  const going = delivery.said.item || "";
  const place = THE_GOINGS[going];
  if (place) theCurrentPlaceIs(place);
});

const THE_GOINGS = {
  "go.servers": ALL_SERVERS,
  "go.queue": THE_QUEUE,
  "go.history": THE_HISTORY,
  "go.transfers": TRANSFERS,
  "go.keys": KEYS,
  "smoozy.settings": SETTINGS,
};

// ── The speed transfers are actually running at ─────────────────────────────
//
// **It belongs to no screen, which is why its place is the shell's.** Design,
// *Where things live*: it sits in the title bar *"so it is readable from every
// screen rather than only on Transfers, and it is there only while something is
// moving"*.
//
// **The place is this file's and the reading is the transfer accounting point's
// (R15a)**, which posts it. **The slot showing nothing is the correct drawing**
// rather than a gap: nothing is transferring, and Design says the reading is
// there only while something is. **It is sent each time the moves event is**
// (`the_moves_are_posted` in `src/surface/transfers.rs`, since 2026-10-03), and
// sent empty when no channel runs in either direction.
//
// **The title bar is reached from this module and `index.html` is untouched**,
// which is what `web/menus.js` already does with its own strip.

const rate = document.createElement("span");
rate.className = "rate mono";
rate.hidden = true;

const buttons = document.getElementById("buttons-right");
if (buttons && buttons.parentNode) buttons.parentNode.insertBefore(rate, buttons);

// **The speed, as the application published it.** One value — a figure, its
// unit and a direction mark, none of them a word — so nothing here composes
// anything. An empty reading takes it away, which is what the end of a transfer
// looks like.
whenTold(THE_SPEED, (delivery) => {
  const said = delivery.said.theSpeed || "";
  rate.textContent = said;
  rate.hidden = !said;
});

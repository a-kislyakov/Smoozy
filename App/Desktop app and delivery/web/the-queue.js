// **The Queue screen** — the list of what every machine is working on, beside
// the entry of the job that is chosen. The application half is
// `src/surface/the_queue.rs`; read its head first.
//
// **This file draws what arrives and composes nothing.** Every word on this
// screen came through the typography point and every name and figure came by
// the verbatim route, and the two are joined here — on the page — and never
// before it. **That is X7's whole answer**: a sentence with a job's name in it
// is three nodes, not one string.
//
// ## What this file exports, and why
//
// **The entry is one shape and two screens fill it.** Design draws one job
// entry — its header, its marks, what it writes, the setup it ran with, the
// preview — and the queue and the history both show it. So the entry's builders
// live here and `web/the-history.js` imports them. **A second
// copy would be Design's one entry becoming two the day one of them drifted.**
//
// ## Three things this file must never do
//
// - **Never write a word.** A6 and X1: prose reaches a screen through the
//   typography point and by no other way, and `src/surface/mod.rs`'s own sweep
//   fails on a quoted literal reaching an element's text or an accessible name.
// - **Never join a value into a sentence.** X7. A part arrives as three keys —
//   the words before, the value, the words after — and this file draws the
//   three in order.
// - **Never take a line out of either of Blender's streams.** M13b, and the
//   mockup does exactly that on the history's endings: a `.log .last` span
//   under the plain sentence. **The control that opens the record is drawn and
//   the line beside it is not.** M13b's one narrowing is the rendering entry's
//   three lines, and they arrive composed, as values: this file draws them and
//   cuts nothing (the Creator's notes on the test copy, piece 8).
//
// H32: values are content, never markup — `textContent` and `createElement`,
// never `innerHTML`. H33: no inline style; the rules are in `the-queue.css`.
// P9 and P10: nothing here polls and nothing here ticks.

import { whenTold } from "./event-route.js";
import { theSeamsAreLive } from "./panes.js";
import { aScreenIsRegistered, theCurrentPlaceIs, theRowsFigureIs, THE_HISTORY, THE_QUEUE } from "./the-rail.js";
import { aTransferIsAskedFor, theFormatBoxIsOpenedFor } from "./transfers.js";
import { theFailureShape, EXPANDED } from "./the-failure-shape.js";
import { theMomentReads } from "./servers.js";
import { theValueDrawn, theValueIsDrawnInto } from "./the-value-drawn.js";
import { theWordsPanel, theNewestWordsStayInView } from "./the-machines-own-words.js";
import { theTypingIsTakenFrom, theTypingIsPutBackInto } from "./the-typed-text.js";

// The two acts a job entry hands Transfers — `TheAct` in
// `src/surface/transfers.rs`. Copy carefully.
const DOWNLOAD_ALL = "download-all";
const DOWNLOAD_ALL_TO = "download-all-to";

const shell = window.__TAURI__;

// Declared beside the service that posts it — `THE_QUEUE_SAYS` in
// `src/surface/the_queue.rs`. The two sides have to agree and nothing holds
// them together, so copy carefully.
const THE_QUEUE_SAYS = "queue:the-queue";
// `THE_ANSWER_SAYS` in the same file: what a control that cannot act answers.
const THE_ANSWER_SAYS = "queue:the-answer";
// `THE_RUNNING_WORDS` in the same file: a rendering job's words, as they arrive
// (`Queue & History` E8). Piece 6c.
const THE_RUNNING_WORDS = "queue:the-running-words";
// What the page tells the application when the words' panel is closed —
// `TheQueueAsk::TheWordsArePutAway` in the same file.
const THE_WORDS_ARE_PUT_AWAY = "the-words-are-put-away";
// The one box this screen asks for — `TheQueueAsk` in the same file.
const DISMISS_IT = "dismiss";
/// **The queue's ask for a row saying *Not submitted.* to go** — the cross's,
/// with the act it is the row of (`Queue & History` C10 as changed
/// 2026-10-19). The name is `TheQueueAsk::ClearTheRow`'s in
/// `src/surface/the_queue.rs`.
const CLEAR_THE_ROW = "clear-the-row";
// **The check's own *Blender output*, in a first-frame pair's box** —
// `TheQueueAsk::TheChecksBlenderOutput`; the check's job travels in `dropped`'s
// place. The nineteenth notes, note 7.
const THE_CHECKS_BLENDER_OUTPUT = "the-checks-blender-output";

let host = null;
let list = null;
let banners = null;
let scroll = null;
let detail = null;

// ── The small joiners every part of both screens is built from ──────────────

// **What stands between a figure and the words after it, where the application
// marks them bound** — U+00A0, so *2 waiting* or *240 frames* never parts
// across two lines. `Design/Whole app § Typography`; the character's one home
// is `src/typography.rs` (`BETWEEN_A_FIGURE_AND_ITS_UNIT`), and this page
// writes it only where the mark says, never by reading which word is a unit.
const BETWEEN_A_FIGURE_AND_ITS_UNIT = "\u00A0";


/// **One part of a line: the words before, the value, the words after.**
///
/// Any may be absent. **Nothing here writes a word** — all three arrive on the
/// event, the first and third through the typography point and the second by
/// the verbatim route.
export function aPart(into, said, base, plain) {
  const lead = said[base + "Lead"];
  const value = said[base + "Value"];
  const trail = said[base + "Trail"];
  let wrote = false;
  if (lead) {
    const words = document.createElement("span");
    words.textContent = lead;
    into.appendChild(words);
    wrote = true;
  }
  if (value) {
    if (wrote) into.appendChild(document.createTextNode(" "));
    const held = document.createElement("span");
    // The fixed-width face is the drawing's own for a value the user can copy;
    // **a first-frame pair's lines draw theirs plain**, as the drawing does.
    if (!plain) held.className = "mono";
    // **Read in the order its bytes run** — X9, `web/the-value-drawn.js`, and
    // so is every name and path below.
    theValueIsDrawnInto(held, value);
    into.appendChild(held);
    wrote = true;
  }
  if (trail) {
    // **A figure and the words after it are bound where the application says
    // so** — `Design/Whole app § Typography`.
    const between = said[base + "Binds"] ? BETWEEN_A_FIGURE_AND_ITS_UNIT : " ";
    // **Words that open on a comma or a full stop follow their value with no
    // space**, where the application says so (`TrailClings`).
    if (wrote && !said[base + "TrailClings"]) into.appendChild(document.createTextNode(between));
    const words = document.createElement("span");
    words.textContent = trail;
    into.appendChild(words);
    wrote = true;
  }
  return wrote;
}

/// A part where the value is a name rather than a figure, so it takes the
/// drawing's bold rather than its fixed-width face.
export function aNamedPart(into, said, base) {
  const lead = said[base + "Lead"];
  const value = said[base + "Value"];
  const trail = said[base + "Trail"];
  let wrote = false;
  if (lead) {
    const words = document.createElement("span");
    words.textContent = lead;
    into.appendChild(words);
    wrote = true;
  }
  if (value) {
    if (wrote) into.appendChild(document.createTextNode(" "));
    const held = document.createElement("b");
    theValueIsDrawnInto(held, value);
    into.appendChild(held);
    wrote = true;
  }
  if (trail) {
    // **A figure and the words after it are bound where the application says
    // so** — `Design/Whole app § Typography`.
    const between = said[base + "Binds"] ? BETWEEN_A_FIGURE_AND_ITS_UNIT : " ";
    // **Words that open on a comma or a full stop follow their value with no
    // space**, where the application says so (`TrailClings`).
    if (wrote && !said[base + "TrailClings"]) into.appendChild(document.createTextNode(between));
    const words = document.createElement("span");
    words.textContent = trail;
    into.appendChild(words);
    wrote = true;
  }
  return wrote;
}

/// **A first-frame pair's lines, one per part** — `Design/Queue & History §
/// A first-frame check and its job`, the drawing's `.ffpart`: the part, its
/// state, and a dot whose class the application names (hollow, `now` or
/// `done`). Queue's row holds both under `.ffparts`; History's row holds the
/// check's one under `.hff`. Nothing where the row is not a pair's. The
/// nineteenth notes, note 7.
export function thePairIsDrawn(said, base, className) {
  const many = howMany(said, base + "PairCount");
  if (!many) return null;
  const into = document.createElement("span");
  into.className = className;
  for (let at = 0; at < many; at += 1) {
    const part = document.createElement("span");
    part.classList.add("ffpart");
    const dot = said[base + "Pair" + at + "Dot"];
    if (dot) part.classList.add(dot);
    const what = document.createElement("span");
    what.className = "fft";
    aPart(what, said, base + "Pair" + at + "What", true);
    part.appendChild(what);
    const state = document.createElement("span");
    state.className = "ffw";
    if (aPart(state, said, base + "Pair" + at + "State", true)) part.appendChild(state);
    into.appendChild(part);
  }
  return into;
}

export function howMany(said, key) {
  const many = Number(said[key] || 0);
  return Number.isFinite(many) && many > 0 ? many : 0;
}

/// A figure and the word beside it, as two nodes. **Two routes in, two nodes
/// out** — a builder that made one string here would have joined them after
/// the split the application side made for exactly this.
export function aFigureAndItsWord(into, figure, word) {
  if (figure === undefined) return;
  const many = document.createElement("b");
  many.textContent = figure;
  into.appendChild(many);
  if (word) {
    // **A figure and its word never part across two lines** — `Design/Whole
    // app § Typography`.
    into.appendChild(document.createTextNode(BETWEEN_A_FIGURE_AND_ITS_UNIT));
    into.appendChild(document.createTextNode(word));
  }
}

/// **A control, with its classes named one at a time.**
///
/// **One class per argument rather than one string of them**, because
/// `src/surface/mod.rs`'s prose sweep reads a literal holding two words as
/// prose wherever it is, and exempts only a statement that names `className` or
/// `classList`. A class list is not words a person reads, and this is how that
/// is said in a form the sweep can see.
function aButton(label, ...classes) {
  const control = document.createElement("button");
  control.type = "button";
  for (const one of classes) control.classList.add(one);
  if (label) control.textContent = label;
  return control;
}

/// **A bar's fill, and its width is a value set through the CSSOM.**
///
/// H33 makes an inline `style=` attribute inert, and the drawing writes every
/// bar's width as one. **`web/panes.js` already sets a width this way** — it is
/// how the dragged pane widths reach `.app` — so this is that route reused
/// rather than a second one invented.
function theFillIsAt(fill, at) {
  fill.style.setProperty("width", (at === undefined ? 0 : at) + "%");
}

/// **The drawing's own badge, and a stored colour outside the eight draws
/// nothing** — the rule `src/surface/the_rail.rs` already keeps for the rail's
/// own rows. **Two class names added rather than one string joined**: a class
/// list is the one place the page may put a value.
function aBadge(into, hue, small) {
  if (!hue) return;
  const badge = document.createElement("span");
  badge.classList.add("badge");
  if (small) badge.classList.add("sm");
  badge.classList.add(hue);
  into.appendChild(badge);
}

// ── What a control that cannot act answers — must-do 5 ─────────────────────
//
// **Every control on the queue and the history either acts or says why it
// cannot.** The application names the act's answer; this file remembers which
// control was pressed and draws the answer beside it, once, in the one failure
// shape. **It goes at the next reading**, because a reading draws everything
// again. Nothing is recorded and nothing reaches the diary.

let thePressed = null;
let theAnswerShown = null;

/// **Ask what a pressed control answers.** Exported for the history's own
/// controls, which answer the same way. `extra` carries the job, the
/// carry-on field's value where the act is the carry-on, and the group's
/// identity where the act is a group's *Cancel all* or its box's go-ahead
/// (step 10c's S3).
export function anActIsAskedFor(control, act, extra) {
  if (!act) return;
  thePressed = { control, act };
  const with_ = extra || {};
  shell.core
    .invoke("an_act_on_a_job", {
      act,
      job: with_.job || null,
      from: with_.from || null,
      group: with_.group || null,
      // **The job a moved row or group now follows**, or nothing for the
      // front (`Queue & History` L6). The 38's piece 8.
      after: with_.after || null,
    })
    .catch(() => {});
}

/// **The same, where the application answers with a box** — a dropped job's
/// line's *Dismiss*, which hands back the job its line names (the Creator's
/// ninth notes, note 3).
function aBoxIsAskedFor(control, act, asking, dropped) {
  thePressed = { control, act };
  shell.core.invoke("the_queue", { chosen: theChosenJob, asking, dropped: dropped || null });
}

/// **A control with its act**, pressed.
function itAnswers(control, act, extra) {
  control.addEventListener("click", () => anActIsAskedFor(control, act, extra));
  return control;
}

whenTold(THE_ANSWER_SAYS, (delivery) => {
  const said = delivery.said || {};
  if (!thePressed || thePressed.act !== said.act || !said.said) return;
  const { control } = thePressed;
  thePressed = null;
  if (theAnswerShown) theAnswerShown.remove();
  theAnswerShown = null;
  if (!control.isConnected) return;
  const holder = document.createElement("div");
  holder.className = "qanswer";
  holder.dataset.act = said.act;
  holder.appendChild(theFailureShape({ sentence: said.said, kind: "ending", controls: [], room: EXPANDED }));
  // **An act pressed in the entry's acts row answers on its own line directly
  // under the row**, from its left edge, never inside the row beside the
  // buttons; **inside a box it stays after the control** (the drawing's
  // `h-gone` and its `.qdacts + .qanswer`, the Creator's fourteenth notes,
  // note 8). Until then *This folder is no longer there.* landed inside
  // `.agroup`, beside *Files on your computer*, and pushed *Delete* under it.
  const inABox = control.closest && control.closest(".modal");
  const theActsRow = !inABox && control.closest && control.closest(".qdacts");
  (theActsRow || control).after(holder);
  theAnswerShown = holder;
  // **A go-ahead answered in its own box keeps the box**, so what the
  // application said stays until the user closes it (step 10c's eleventh-pass
  // fix piece 5).
  if (inABox) delete inABox.dataset.answered;
});

// ── The screen ──────────────────────────────────────────────────────────────

aScreenIsRegistered({
  place: THE_QUEUE,
  itIsDrawn(where) {
    host = where;
    const split = document.createElement("div");
    split.className = "qsplit";

    // **The list's seam is one of the three the pane-width record already
    // holds** — the contract's section 7. It is one width for the queue and
    // the history both, which is why the key is the list's and not this
    // screen's.
    const seam = aButton(null, "seam");
    seam.dataset.seam = "list";
    split.appendChild(seam);

    list = document.createElement("div");
    list.className = "qlist";
    split.appendChild(list);

    detail = document.createElement("div");
    detail.className = "qdetail";
    split.appendChild(detail);

    where.appendChild(split);

    shell.core
      .invoke("the_pane_layout")
      .then((layout) => {
        const declared = layout.seams.find((one) => one.key === "list");
        if (!declared) return;
        seam.setAttribute("aria-label", declared.label);
        seam.title = declared.hint;
        theSeamsAreLive(split, layout.seams);
      })
      .catch(() => {
        // The widths are `panes.js`'s to speak about and it already does. A
        // list without a draggable seam is still a list.
      });

    theQueueIsAskedFor();
  },
  itIsCurrentNow(which) {
    // **P8: events are volatile and nothing is replayed**, so a screen that has
    // been away asks again rather than remembering.
    //
    // **A job handed over is the one chosen** — Render Settings' *Show in
    // Queue* hands the job it just submitted (the Creator's notes on the test
    // copy, piece 3), and until piece 8 this dropped it and showed whatever
    // entry was chosen before.
    if (which) theChosenJob = which;
    if (host) theQueueIsAskedFor();
  },
  itIsPutAway() {
    // **P10d repeats the queue reading only while a surface showing the queue
    // is on screen**, and this is how the application learns it no longer is.
    // Nothing comes back. **The words' panel goes with the screen**, and its
    // reads stop with it (piece 6c).
    if (theRunningWords) {
      theRunningWords.panel.remove();
      theRunningWords = null;
    }
    thePreviewGoesAway("queue");
    shell.core.invoke("the_queue", { chosen: theChosenJob, asking: "put-away" }).catch(() => {});
  },
});

// **The job whose entry the user chose**, held for the run and nothing durable.
// It is what the page is looking at, handed back each time the queue is asked
// for; the application answers with that job's entry.
let theChosenJob = null;

function theQueueIsAskedFor() {
  shell.core.invoke("the_queue", { chosen: theChosenJob, asking: null });
}

/// **`Design/Queue & History § Keyboard shortcuts`: *"Return opens the chosen
/// entry."*** A row is a button, so Return on it is its press, and the press
/// chooses it — the drawing's own *picking a row is a real thing the app does*.
/// **The delete key does nothing on a queue entry**, where the equivalent act is
/// Cancel, and Design says why: that is not a key press.
/// **Open Queue on one job's entry** — a click on a notification about a job
/// the Queue holds (`Desktop app and delivery` T29). The job is named by its
/// identity, as History's `theHistoryEntryIsOpened` names it.
///
/// **Queue showing is its own drawn screen not being hidden**, read here rather
/// than asked of the rail, so the page harnesses that stand in for the rail
/// need nothing new.
export function theQueueEntryIsOpened(job) {
  if (job) theChosenJob = job;
  if (host && !host.hidden) theQueueIsAskedFor();
  else theCurrentPlaceIs(THE_QUEUE, theChosenJob);
}

function theRowIsChosen(job) {
  if (!job) return;
  theChosenJob = job;
  theQueueIsAskedFor();
}

whenTold(THE_QUEUE_SAYS, (delivery) => {
  if (!host) return;
  // **A drawing waits while a row is held or its menu is open**, and the
  // newest one is drawn the moment it is let go: a drawing replaces the list,
  // and a P10d pass every few seconds would otherwise pull the row out from
  // under the pointer or shut the menu as it was being read. The 38's piece 8.
  if (dragging || theOpenMenu) {
    theDrawWaiting = delivery.said || {};
    return;
  }
  theQueueIsDrawn(delivery.said || {});
  // **A check's kept words, where its *Blender output* was pressed** (the
  // nineteenth notes, note 7).
  theKeptWordsAreOpened(host, delivery.said || {});
});

let theKeyboardWasOnTheList = false;

// **Must-do 7: what the user folded stays folded for the run**, across every
// redraw — each server's heading by its server. *(The catch-up notice folded
// too, until the Creator's ninth notes, note 3, made it a count.)*
const theFoldedServers = new Set();

function theQueueIsDrawn(said) {
  thePressed = null;
  theAnswerShown = null;
  theKeyboardWasOnTheList = !!(
    document.activeElement &&
    document.activeElement.classList &&
    document.activeElement.classList.contains("qrow") &&
    list.contains(document.activeElement)
  );
  thePreviewRefreshStarts(host);
  // **The minutes field keeps what is typed in it across a redraw** —
  // `Design/Whole app § The window`, through `web/the-typed-text.js` as
  // History's carry-on field is kept: the focus, the typing and the caret,
  // and only into the same job's entry. The entry is drawn again while a job
  // renders, and a click, a redraw and then typing landed on nothing (the
  // live Telegram pass, kept with the twenty-fifth notes, note 1).
  const theMinutes = theMinutesField;
  const theMinutesTyping = theMinutes ? theTypingIsTakenFrom(theMinutes.field) : null;
  theMinutesField = null;
  list.replaceChildren();
  detail.replaceChildren();

  list.appendChild(theHead(said));
  // **The banners go straight into the list**, which is where the drawing's
  // own rules put them; there is no wrapper for them to sit in.
  banners = list;
  theDroppedAreDrawn(said);
  thePartIsDrawn(said);
  theUntrackedIsDrawn(said);

  scroll = document.createElement("div");
  scroll.className = "qscroll";
  // **The list and the entry keep their place across a redraw** —
  // `Design/Whole app § The window`, kept by `web/the-scrolled-place.js` for
  // the names given here and in `theEntryIsDrawn`. The Creator's second notes
  // on the test copy, note 9: the queue is drawn anew on every reading, which
  // P10d repeats while the screen is shown, and both went to their top each
  // time.
  scroll.dataset.place = "queue-list";
  list.appendChild(scroll);
  theRowsAreDrawn(said);

  // **Design, *The queue list*: the one line that teaches dragging**, under
  // the list — the drawing's `.qfoot`. The application sends its words only
  // where something on the list can be moved, so no line teaches a gesture
  // that does nothing (must-do 5(f)). The 38's piece 8.
  if (said.whatDraggingDoes) list.appendChild(theFoot(said));

  theEntryIsDrawn(detail, said, {});
  if (theMinutesTyping && theMinutesField && theMinutesField.job === theMinutes.job) {
    theTypingIsPutBackInto(theMinutesField.field, theMinutesTyping);
  }
  theBoxStaysOrIsDrawn(host, said, theJobsIn(said, "rowCount", "row"), theGroupsIn(said));

  // **The chosen row is the one whose entry is beside the list**, marked the
  // drawing's way, and the keyboard stays on it across the redraw.
  const shown = said.entryJob || "";
  for (const row of scroll.querySelectorAll(".qrow[data-job]")) {
    if (row.dataset.job !== shown) continue;
    row.setAttribute("aria-current", "true");
    if (theKeyboardWasOnTheList) row.focus();
  }
  theKeyboardWasOnTheList = false;

  // **The rail row's count is this screen's and arrives through the shell** —
  // the contract's section 8. **Nothing is drawn for nought** (must-do 21).
  theRowsFigureIs(THE_QUEUE, said.theRailsFigure || "");
}

function theFoot(said) {
  const foot = document.createElement("div");
  foot.className = "qfoot";
  const teaches = document.createElement("span");
  teaches.textContent = said.whatDraggingDoes;
  foot.appendChild(teaches);
  foot.appendChild(document.createTextNode(" "));
  const menu = document.createElement("span");
  menu.textContent = said.andTheRowsOwnMenu || "";
  foot.appendChild(menu);
  foot.appendChild(document.createTextNode(" "));
  const item = document.createElement("b");
  item.textContent = said.moveToTheTop || "";
  foot.appendChild(item);
  foot.appendChild(document.createTextNode("."));
  return foot;
}

function theHead(said) {
  const head = document.createElement("div");
  head.className = "qhead";
  const name = document.createElement("h1");
  name.textContent = said.theHeading || "";
  head.appendChild(name);
  const grow = document.createElement("span");
  grow.className = "grow";
  head.appendChild(grow);
  const sub = document.createElement("span");
  sub.className = "qsub";
  aFigureAndItsWord(sub, said.theJobsFigure, said.theJobsWord);
  sub.appendChild(document.createTextNode(" · "));
  aFigureAndItsWord(sub, said.theServersFigure, said.theServersWord);
  theCountIsDrawn(sub, said);
  head.appendChild(sub);
  return head;
}

/// **The endings counted, at the end of the heading's grey line** —
/// `Design/Queue & History § The catch-up notice`, the Creator's ninth notes
/// (2026-10-11), note 3: *· 3 finished · 1 failed*, only the kinds the
/// application sends (those above nought), each word in the class of its
/// History colour, which the application names. **Pressing it opens History**,
/// where every one of those jobs is, and opening History clears it. The
/// drawing's `#qcount`.
function theCountIsDrawn(sub, said) {
  const kinds = howMany(said, "countedCount");
  if (!kinds) return;
  const count = document.createElement("span");
  count.id = "qcount";
  count.appendChild(document.createTextNode(" · "));
  const opens = aButton(null, "lnkb");
  opens.title = said.theCountSince || "";
  for (let at = 0; at < kinds; at += 1) {
    if (at) {
      const between = document.createElement("span");
      between.className = "qst";
      between.textContent = " · ";
      opens.appendChild(between);
    }
    const kind = document.createElement("span");
    kind.classList.add("qst");
    const is = said["counted" + at + "Is"];
    if (is) kind.classList.add(is);
    // **A figure and its word never part across two lines** — `Design/Whole
    // app § Typography`; the drawing's `3&#160;finished`.
    const figure = said["counted" + at + "Figure"];
    const word = said["counted" + at + "Word"];
    kind.appendChild(document.createTextNode(figure || ""));
    kind.appendChild(document.createTextNode(BETWEEN_A_FIGURE_AND_ITS_UNIT));
    kind.appendChild(document.createTextNode(word || ""));
    opens.appendChild(kind);
  }
  opens.addEventListener("click", () => theCurrentPlaceIs(THE_HISTORY));
  count.appendChild(opens);
  sub.appendChild(count);
}

// ── The list ────────────────────────────────────────────────────────────────

function theRowsAreDrawn(said) {
  const lines = howMany(said, "lineCount");
  const rows = howMany(said, "rowCount");
  const groups = howMany(said, "groupCount");
  for (let line = 0; line < lines; line += 1) {
    const server = said["line" + line + "Server"] || "";
    const shut = theFoldedServers.has(server);
    const section = document.createElement("section");
    section.className = "qmgroup";
    section.dataset.open = shut ? "false" : "true";

    const heading = aButton(null, "qmach");
    if (said["line" + line + "LastSeen"]) heading.classList.add("stale");
    heading.setAttribute("aria-expanded", shut ? "false" : "true");
    const chev = document.createElement("span");
    chev.className = "chev";
    chev.textContent = "▼";
    heading.appendChild(chev);
    aBadge(heading, said["line" + line + "Badge"]);
    const name = document.createElement("b");
    const held1 = said["line" + line + "Name"];
    theValueIsDrawnInto(name, held1 || "");
    heading.appendChild(name);
    const grow = document.createElement("span");
    grow.className = "grow";
    heading.appendChild(grow);

    const open = document.createElement("span");
    open.className = "qm2 open";
    const seen = said["line" + line + "LastSeen"];
    if (seen) {
      const words = document.createElement("span");
      words.textContent = said.notUpdatingWord || "";
      open.appendChild(words);
      open.appendChild(document.createTextNode(" · "));
      const when = document.createElement("span");
      when.textContent = said.lastSeenWord || "";
      open.appendChild(when);
      open.appendChild(document.createTextNode(" "));
      const value = document.createElement("span");
      value.className = "mono";
      // **`Queue & History` C7: the moment the reading was taken**, sent as
      // seconds and written here with the reader's own locale, as History writes
      // its moments.
      value.textContent = theMomentReads(seen);
      open.appendChild(value);
    } else {
      aFigureAndItsWord(open, said["line" + line + "Rendering"], said.renderingWord);
      open.appendChild(document.createTextNode(" · "));
      aFigureAndItsWord(open, said["line" + line + "Waiting"], said.waitingWord);
    }
    heading.appendChild(open);

    // **A folded heading still says what is running**, which is the whole
    // reason to fold it — and the application composed what it says, in parts,
    // for all three of its cases (must-do 7).
    const folding = document.createElement("span");
    folding.className = "qm2 shut";
    const folded = said["line" + line + "FoldedName"];
    if (folded) {
      const job = document.createElement("span");
      job.className = "qmn";
      theValueIsDrawnInto(job, folded);
      folding.appendChild(job);
      const bar = document.createElement("span");
      bar.className = "bar whole";
      const fill = document.createElement("i");
      theFillIsAt(fill, said["line" + line + "FoldedAt"]);
      bar.appendChild(fill);
      folding.appendChild(bar);
    }
    const parts = howMany(said, "line" + line + "FoldedCount");
    for (let part = 0; part < parts; part += 1) {
      const held2 = document.createElement("span");
      if (aPart(held2, said, "line" + line + "Folded" + part)) folding.appendChild(held2);
      // The folded heading's *last seen* carries the same moment, as seconds.
      if (seen && said["line" + line + "Folded" + part + "Value"] === seen) {
        const moment = held2.querySelector(".mono");
        if (moment) moment.textContent = theMomentReads(seen);
      }
    }
    heading.appendChild(folding);

    heading.addEventListener("click", () => {
      const open = section.dataset.open === "true";
      section.dataset.open = open ? "false" : "true";
      heading.setAttribute("aria-expanded", open ? "false" : "true");
      if (open) theFoldedServers.add(server);
      else theFoldedServers.delete(server);
    });
    section.appendChild(heading);

    const held = document.createElement("div");
    held.className = "qmrows";
    section.appendChild(held);

    let inGroup = null;
    for (let at = 0; at < rows; at += 1) {
      if (Number(said["row" + at + "Line"]) !== line) continue;
      const group = said["row" + at + "Group"];
      let into = held;
      if (group !== undefined && Number(group) < groups) {
        if (!inGroup || inGroup.dataset.group !== group) {
          inGroup = document.createElement("div");
          inGroup.className = "qgroup";
          inGroup.dataset.group = group;
          inGroup.appendChild(aGroupHeading(said, Number(group)));
          held.appendChild(inGroup);
        }
        into = inGroup;
      } else {
        inGroup = null;
      }
      into.appendChild(aRow(said, at, into === held ? "" : "in"));
    }

    scroll.appendChild(section);
  }
}

function aGroupHeading(said, group) {
  const heading = document.createElement("div");
  heading.className = "qgh";
  const grip = document.createElement("span");
  // **Design: the group's heading is the grip for the whole group**, offered
  // where a member is waiting (`Queue & History` L7). The 38's piece 8.
  const moves = !!said["group" + group + "Grip"];
  grip.className = moves ? "qgrip" : "qgrip nodrag";
  grip.textContent = "⋮⋮";
  grip.setAttribute("aria-hidden", "true");
  if (moves) {
    grip.title = said.dragTheWholeGroup || "";
    const id = said["group" + group + "Group"] || "";
    grip.addEventListener("pointerdown", (press) =>
      theDragBegins(press, { block: heading.parentNode || heading, anchor: heading, group: id }),
    );
  }
  heading.appendChild(grip);
  const name = document.createElement("span");
  const held3 = said["group" + group + "Name"];
  theValueIsDrawnInto(name, held3 || "");
  heading.appendChild(name);
  heading.appendChild(document.createTextNode(" — "));
  const scenes = document.createElement("span");
  scenes.textContent = said.allScenesLead || "";
  heading.appendChild(scenes);
  heading.appendChild(document.createTextNode(" "));
  aFigureAndItsWord(heading, said["group" + group + "Scenes"], said["group" + group + "ScenesWord"]);
  const grow = document.createElement("span");
  grow.className = "grow";
  heading.appendChild(grow);
  const cancel = aButton(null, "glnk");
  // **The group is handed back by its identity**, never by its name (S3).
  itAnswers(cancel, "cancel-all", { group: said["group" + group + "Group"] || "" });
  cancel.textContent = said.cancelAll || "";
  cancel.appendChild(document.createTextNode(" "));
  const many = document.createElement("span");
  const held4 = said["group" + group + "Scenes"];
  many.textContent = held4 || "";
  cancel.appendChild(many);
  heading.appendChild(cancel);
  return heading;
}

/// **A job just submitted, while its act runs** — `Queue & History` C10, the
/// drawing's `qrow sub`: its name, the state's word with the spinner after it,
/// and no grip, bars, choice or menu. The word is Rust's. The Creator's
/// seventh notes, piece Z7.
function aSubmittingRow(said, at, extra) {
  const row = document.createElement("div");
  row.classList.add("qrow", "sub");
  if (extra) row.classList.add(extra);
  const grip = document.createElement("span");
  grip.className = "qgrip nodrag";
  grip.textContent = "⋮⋮";
  grip.setAttribute("aria-hidden", "true");
  row.appendChild(grip);
  const name = document.createElement("span");
  name.className = "qn";
  theValueIsDrawnInto(name, said["row" + at + "Name"] || "");
  row.appendChild(name);
  const state = document.createElement("span");
  state.className = "qst";
  const word = said["row" + at + "State"];
  state.textContent = word || "";
  const turning = document.createElement("i");
  turning.className = "spin";
  state.appendChild(turning);
  row.appendChild(state);
  const bars = document.createElement("span");
  bars.className = "qbars";
  row.appendChild(bars);
  return row;
}

/// **A sentence the application composed, set part by part into `into`** —
/// each part's lead, its values by the verbatim route, and its rest, with no
/// space before a rest the application says joins and a bound one where it
/// says binds. **Nothing here writes a word.** The keys are declared beside
/// `a_sentence_is_said` in `src/surface/the_queue.rs`.
function aSentenceInto(into, said, base) {
  const parts = howMany(said, base + "Count");
  let wrote = false;
  for (let part = 0; part < parts; part += 1) {
    const at = base + part;
    const lead = said[at + "Lead"];
    if (lead) {
      if (wrote) into.appendChild(document.createTextNode(" "));
      const words = document.createElement("span");
      words.textContent = lead;
      into.appendChild(words);
      wrote = true;
    }
    const values = howMany(said, at + "ValueCount");
    for (let which = 0; which < values; which += 1) {
      if (wrote) into.appendChild(document.createTextNode(" "));
      const held = document.createElement("b");
      theValueIsDrawnInto(held, said[at + "Value" + which] || "");
      into.appendChild(held);
      wrote = true;
    }
    const rest = said[at + "Rest"];
    if (rest) {
      const between = said[at + "Binds"] ? BETWEEN_A_FIGURE_AND_ITS_UNIT : " ";
      if (wrote && !said[at + "Joins"]) into.appendChild(document.createTextNode(between));
      const words = document.createElement("span");
      words.textContent = rest;
      into.appendChild(words);
      wrote = true;
    }
  }
  return wrote;
}

/// **A submission that ended with a job not in the machine's line** —
/// `Queue & History` C10 as changed 2026-10-19: the row stays in its server's
/// line saying what the panel says, *Not submitted.* in the state's place and
/// why under the name, with a cross that clears it. No grip, bars, choice or
/// menu. **The cross hands back the act it is the row of**, which the
/// application knows it by. Out of the live pass on Vast for the thirteenth
/// and fourteenth notes, fault 1: the row went with nothing said.
function aNotSubmittedRow(said, at, extra) {
  const row = document.createElement("div");
  row.classList.add("qrow", "sub");
  if (extra) row.classList.add(extra);
  const grip = document.createElement("span");
  grip.className = "qgrip nodrag";
  grip.textContent = "⋮⋮";
  grip.setAttribute("aria-hidden", "true");
  row.appendChild(grip);
  const name = document.createElement("span");
  name.className = "qn";
  theValueIsDrawnInto(name, said["row" + at + "Name"] || "");
  const why = document.createElement("span");
  why.className = "qs";
  if (aSentenceInto(why, said, "row" + at + "Why")) name.appendChild(why);
  row.appendChild(name);
  const state = document.createElement("span");
  state.classList.add("qst");
  const is = said["row" + at + "StateIs"];
  if (is) state.classList.add(is);
  aSentenceInto(state, said, "row" + at + "Went");
  row.appendChild(state);
  const bars = document.createElement("span");
  bars.className = "qbars";
  const cross = document.createElement("button");
  cross.type = "button";
  cross.className = "fx";
  cross.title = said.clearIt || "";
  cross.setAttribute("aria-label", said.clearIt || "");
  cross.textContent = "×";
  const act = said["row" + at + "NotSubmitted"] || "";
  cross.addEventListener("click", () => aBoxIsAskedFor(cross, CLEAR_THE_ROW, CLEAR_THE_ROW, act));
  bars.appendChild(cross);
  row.appendChild(bars);
  return row;
}

function aRow(said, at, extra) {
  if (said["row" + at + "Submitting"]) return aSubmittingRow(said, at, extra);
  if (said["row" + at + "NotSubmitted"]) return aNotSubmittedRow(said, at, extra);
  const row = aButton(null, "qrow");
  if (extra) row.classList.add(extra);
  const job = said["row" + at + "Job"];
  if (job) {
    row.dataset.job = job;
    row.addEventListener("click", () => {
      // **The press that ends a drag is not a choice.**
      if (Date.now() - theLastDropAt < 400) return;
      theRowIsChosen(job);
    });
  }
  if (said["row" + at + "Stale"]) row.classList.add("stale");
  // **The group a row belongs to, by its identity**, so a group being moved
  // leaves its own members out of where it could land.
  const inGroup = said["row" + at + "Group"];
  if (inGroup !== undefined) row.dataset.groupId = said["group" + inGroup + "Group"] || "";

  const grip = document.createElement("span");
  const moves = !!said["row" + at + "Grip"] && !!job;
  grip.className = moves ? "qgrip" : "qgrip nodrag";
  grip.textContent = "⋮⋮";
  grip.setAttribute("aria-hidden", "true");
  if (moves) {
    // **Only a waiting row moves** (`Queue & History` L9), and only it offers
    // the grip, its tooltip and its own menu. The 38's piece 8.
    row.dataset.moves = "yes";
    grip.title = said.dragItUpTheQueue || "";
    grip.addEventListener("pointerdown", (press) => theDragBegins(press, { block: row, anchor: row, job }));
    row.addEventListener("contextmenu", (press) => {
      press.preventDefault();
      theMenuIsShown(row, said, job);
    });
  }
  row.appendChild(grip);

  const name = document.createElement("span");
  name.className = "qn";
  const held5 = said["row" + at + "Name"];
  theValueIsDrawnInto(name, held5 || "");
  const sub = document.createElement("span");
  sub.className = "qs";
  let wrote = false;
  const parts = howMany(said, "row" + at + "SubCount");
  for (let part = 0; part < parts; part += 1) {
    const held = document.createElement("span");
    if (!aPart(held, said, "row" + at + "Sub" + part)) continue;
    if (wrote) sub.appendChild(document.createTextNode(" · "));
    sub.appendChild(held);
    wrote = true;
  }
  if (wrote) name.appendChild(sub);
  row.appendChild(name);

  const state = document.createElement("span");
  const is = said["row" + at + "StateIs"];
  // **Two class names added rather than one string joined** — the one place the
  // page may put a value.
  state.classList.add("qst");
  if (is) state.classList.add(is);
  const held6 = said["row" + at + "State"];
  state.textContent = held6 || "";
  row.appendChild(state);
  // **A first-frame pair is one row, naming both parts under the name** —
  // the drawing's `.ffparts`, after the state, across the row.
  const pair = thePairIsDrawn(said, "row" + at, "ffparts");
  if (pair) row.appendChild(pair);

  const bars = document.createElement("span");
  bars.className = "qbars";
  // **Each bar the row has** — the frame's and the whole job's, each only
  // where the application sent it (W4, the Creator's twelfth notes, note 7).
  // Until then the frame's bar carried both, so a frame not yet sampled took
  // the whole job's bar away with its own.
  const renderlet = said["row" + at + "Renderlet"];
  if (renderlet !== undefined) bars.appendChild(aBar("qbar", renderlet));
  const whole = said["row" + at + "Whole"];
  if (whole !== undefined) bars.appendChild(aBar("qbar w", whole));
  row.appendChild(bars);
  return row;
}

// ── A job's kept words, in the one panel ────────────────────────────────────

// **The panel the kept words are open in**, held on the screen rather than in
// the entry, so the redraw a connection makes does not take it away from under
// the reader. It goes when *Close* is pressed or the next words replace it.
// One for the page: History's *Blender output* and a first-frame pair's
// check's open the same panel (the nineteenth notes, note 7).
let theKeptWordsPanel = null;

/// **`Queue & History` E9's kept words, opened in the one panel** —
/// `web/the-machines-own-words.js`'s `theWordsPanel`, with one tab per stream
/// of each attempt. **Every word and every record arrives on the event**: this
/// joins none of them and reads nothing out of a record (M9, M13b). Where the
/// job ran more than once, each tab names its attempt as well as its stream
/// (`Queue & History` E11). Moved here from `web/the-history.js` when Queue's
/// entry began to open a check's words too.
export function theKeptWordsAreOpened(host, said) {
  const attempts = Number(said.wordsAttempts || 0);
  if (!attempts) return;
  const tabs = [];
  for (let at = 0; at < attempts; at += 1) {
    for (const keys of THE_STREAMS_KEYS) {
      const named = (key) => said["words" + at + key];
      if (!named(keys.says)) continue;
      let name = named(keys.tab) || "";
      if (attempts > 1) {
        name += " · " + (said.wordsAttemptWord || "") + " " + (said["words" + at + "Attempt"] || "");
      }
      tabs.push({
        name,
        says: named(keys.says),
        said: named(keys.said),
        saidNext: named(keys.saidNext),
        record: named(keys.record),
        kept: named(keys.kept),
        dropped: named(keys.dropped),
        whyTheRestWent: named(keys.whyTheRestWent),
        theKeptWord: said.wordsKeptWord,
        theDroppedWord: said.wordsDroppedWord,
      });
    }
  }
  if (!tabs.length) return;
  const panel = theWordsPanel({
    title: said.wordsTitle,
    subtitle: [said.wordsJob, said.wordsServer].filter(Boolean).join(" · "),
    tabs,
    note: said.wordsNote,
    close: said.wordsClose,
    copy: said.wordsCopy,
  });
  if (theKeptWordsPanel) theKeptWordsPanel.remove();
  theKeptWordsPanel = panel;
  // **M8: the panel opens because its control was pressed**, and the words
  // only ever arrive in answer to that press.
  panel.hidden = false;
  host.appendChild(panel);
}

// The two streams' own key sets — `THE_ORDINARY_STREAMS_KEYS` and
// `THE_COMPLAINTS_STREAMS_KEYS` in `src/surface/the_machines_own_words.rs`,
// each under `words<n>` for its attempt. **They share no key** (M13a).
const THE_STREAMS_KEYS = [
  {
    tab: "theOutputTab",
    says: "theOutputSays",
    said: "theOutputSaid",
    saidNext: "theOutputSaidNext",
    record: "theOutputRecord",
    kept: "theOutputKept",
    dropped: "theOutputDropped",
    whyTheRestWent: "theOutputWhyTheRestWent",
  },
  {
    tab: "theErrorsTab",
    says: "theErrorsSays",
    said: "theErrorsSaid",
    saidNext: "theErrorsSaidNext",
    record: "theErrorsRecord",
    kept: "theErrorsKept",
    dropped: "theErrorsDropped",
    whyTheRestWent: "theErrorsWhyTheRestWent",
  },
];

// ── Moving a row, or a group, up its server's queue — the 38's piece 8 ───────
//
// **Design, *The queue list*: reordering is dragging a row, and it only ever
// moves inside its own server's queue.** The row lifts and a line shows where
// it would land — the look Settings' search folders already have (the
// drawing's `sfdrop`, with the queue's own `qlift`); no gap opens. **A drag
// that lands outside its own server's queue, or where it started, changes
// nothing.**
//
// **The page names where the row was dropped and never an index**
// (`Queue & History` L6): the waiting job it now follows, or nothing for the
// front. **The machine applies that to its line as it is then** and the queue
// is drawn again from its reading, so a row that started rendering meanwhile
// comes back where the machine has it, saying so. Nothing here moves a row
// in the list itself.

const MOVE_THE_JOB = "move-the-job";
const MOVE_THE_GROUP = "move-the-group";

let dragging = null;
let theDrawWaiting = null;
let theLastDropAt = 0;
let theOpenMenu = null;

/// **The rows the moving row or group could land between**: every row of its
/// own server's queue but itself — and, for a group, but its own members.
function theRowsBeside(moving) {
  const section = moving.block.closest(".qmgroup");
  if (!section) return [];
  return Array.from(section.querySelectorAll(".qrow[data-job]")).filter(
    (row) => !moving.block.contains(row) && !(moving.group && row.dataset.groupId === moving.group),
  );
}

function theDragBegins(press, moving) {
  if (press.button !== 0 || dragging) return;
  press.preventDefault();
  press.stopPropagation();
  // **An open menu is shut first, and that press starts nothing**: shutting it
  // may draw the list again, which would take the row from under the pointer.
  if (theOpenMenu) {
    theMenuGoes();
    return;
  }
  const section = moving.block.closest(".qmgroup");
  if (!section) return;
  const rows = theRowsBeside(moving);
  // **Where it starts**: how many of those rows stand above its own first row.
  const every = Array.from(section.querySelectorAll(".qrow[data-job]"));
  const its = every.findIndex((row) => moving.block.contains(row));
  const from = rows.filter((row) => every.indexOf(row) < its).length;
  const line = document.createElement("div");
  line.className = "sfdrop";
  dragging = { ...moving, section, rows, from, to: from, line, grip: press.currentTarget };
  moving.block.classList.add("qlift");
  dragging.grip.setPointerCapture(press.pointerId);
  dragging.grip.addEventListener("pointermove", theDragMoves);
  dragging.grip.addEventListener("pointerup", theDragEnds);
  dragging.grip.addEventListener("pointercancel", theDragIsDropped);
}

function theDragMoves(move) {
  if (!dragging) return;
  const box = dragging.section.getBoundingClientRect();
  // **Over another server's queue, or off the list, it lands on nothing.**
  if (move.clientY < box.top || move.clientY > box.bottom || move.clientX < box.left || move.clientX > box.right) {
    dragging.to = null;
    dragging.line.remove();
    return;
  }
  let slot = 0;
  for (const row of dragging.rows) {
    const at = row.getBoundingClientRect();
    if (move.clientY > at.top + at.height / 2) slot += 1;
  }
  dragging.to = slot;
  if (slot === dragging.from) {
    dragging.line.remove();
    return;
  }
  const before = dragging.rows[slot];
  if (before) before.parentNode.insertBefore(dragging.line, before);
  else if (dragging.rows.length) dragging.rows[dragging.rows.length - 1].after(dragging.line);
}

function theDragEnds() {
  if (!dragging) return;
  const { rows, from, to, anchor, job, group } = dragging;
  theDragIsDropped();
  if (to === null || to === from) return;
  theLastDropAt = Date.now();
  // **The waiting job it now follows** — the last row above where it landed
  // that is itself waiting; a rendering or held row has no place in the line
  // (`Queue & History` Q5), so above only those is the front.
  const above = rows.slice(0, to).filter((row) => row.dataset.moves === "yes");
  const after = above.length ? above[above.length - 1].dataset.job : null;
  if (group) anActIsAskedFor(anchor, MOVE_THE_GROUP, { group, after });
  else anActIsAskedFor(anchor, MOVE_THE_JOB, { job, after });
}

function theDragIsDropped() {
  if (!dragging) return;
  dragging.block.classList.remove("qlift");
  dragging.line.remove();
  dragging.grip.removeEventListener("pointermove", theDragMoves);
  dragging.grip.removeEventListener("pointerup", theDragEnds);
  dragging.grip.removeEventListener("pointercancel", theDragIsDropped);
  dragging = null;
  theWaitingDrawIsDrawn();
}

/// **Design: beside dragging, a row's own menu offers *Move to the top of the
/// queue*, and nothing else about position** — placed under the row, the way
/// Transfers places its own.
function theMenuIsShown(row, said, job) {
  theMenuGoes();
  const menu = document.createElement("div");
  menu.className = "ctx";
  menu.classList.add("queue-ctx");
  const top = aButton(said.moveToTheTop || "");
  top.addEventListener("click", (press) => {
    press.stopPropagation();
    theMenuGoes();
    anActIsAskedFor(row, MOVE_THE_JOB, { job });
  });
  menu.appendChild(top);
  row.after(menu);
  theOpenMenu = menu;
  top.focus();
}

function theMenuGoes() {
  if (!theOpenMenu) return;
  theOpenMenu.remove();
  theOpenMenu = null;
  theWaitingDrawIsDrawn();
}

function theWaitingDrawIsDrawn() {
  if (dragging || theOpenMenu || !theDrawWaiting) return;
  const said = theDrawWaiting;
  theDrawWaiting = null;
  if (host) theQueueIsDrawn(said);
}

document.addEventListener("click", (press) => {
  if (theOpenMenu && !theOpenMenu.contains(press.target)) theMenuGoes();
});
document.addEventListener("keydown", (press) => {
  if (press.key === "Escape") theMenuGoes();
});

/// **A bar's width is a value and it is set as one.** H33 makes an inline style
/// inert, so the width is set on the fill through the CSSOM, as `width`, by
/// `theFillIsAt` above; no stylesheet rule is involved.
function aBar(className, at) {
  const bar = document.createElement("span");
  bar.className = className;
  const fill = document.createElement("i");
  theFillIsAt(fill, at);
  bar.appendChild(fill);
  return bar;
}

// ── The three banners ───────────────────────────────────────────────────────

/// **A job dropped before it rendered, named in one quiet line under the
/// heading** — `Design/Queue & History § The catch-up notice` and § *Jobs that
/// leave without an entry*: the job's name in bold, then what happened to it,
/// and *Dismiss* at the line's end. It has no History entry, so this line is
/// the only place it is ever named, and *Dismiss* asks first — which the
/// application decides by posting the box. One line per job; the drawing's
/// `#qdropped` and `#catchdismiss`, whose look each line's *Dismiss* takes.
function theDroppedAreDrawn(said) {
  const lines = howMany(said, "droppedCount");
  for (let at = 0; at < lines; at += 1) {
    const job = said["dropped" + at + "Job"] || "";
    const line = document.createElement("div");
    line.className = "qbanner";
    if (!at) line.id = "qdropped";
    line.dataset.dropped = job;
    const says = document.createElement("p");
    says.className = "qb2 grow";
    const name = document.createElement("b");
    theValueIsDrawnInto(name, said["dropped" + at + "Name"] || "");
    says.appendChild(name);
    says.appendChild(document.createTextNode(" "));
    aPart(says, said, "dropped" + at + "What");
    line.appendChild(says);
    const controls = document.createElement("span");
    controls.className = "cbtns";
    const dismiss = aButton(said.dismiss, "mini");
    dismiss.id = "catchdismiss";
    dismiss.addEventListener("click", () => aBoxIsAskedFor(dismiss, "dismiss-the-notice", DISMISS_IT, job));
    controls.appendChild(dismiss);
    line.appendChild(controls);
    banners.appendChild(line);
  }
}

function thePartIsDrawn(said) {
  if (!said.thePartSays) return;
  const banner = document.createElement("div");
  banner.className = "qbanner amber";
  const body = document.createElement("div");

  const heading = document.createElement("p");
  heading.className = "qb1";
  const confirmed = document.createElement("b");
  confirmed.textContent = said.thePartConfirmed || "";
  heading.appendChild(confirmed);
  heading.appendChild(document.createTextNode(" "));
  const of = document.createElement("span");
  of.textContent = said.ofWord || "";
  heading.appendChild(of);
  heading.appendChild(document.createTextNode(" "));
  const total = document.createElement("b");
  total.textContent = said.thePartOf || "";
  heading.appendChild(total);
  heading.appendChild(document.createTextNode(" "));
  const says = document.createElement("span");
  says.textContent = said.thePartSays;
  heading.appendChild(says);
  body.appendChild(heading);

  const second = document.createElement("p");
  second.className = "qb2";
  second.textContent = said.thePartSecond || "";
  body.appendChild(second);

  const lists = document.createElement("dl");
  lists.className = "qparts";
  for (const [name, prefix, count, why] of [
    [said.thePartSubmittedName, "partSubmitted", "partSubmittedCount", null],
    [said.thePartNotSubmittedName, "partNotSubmitted", "partNotSubmittedCount", null],
    [said.thePartNotKnownName, "partNotKnown", "partNotKnownCount", said.thePartNotKnownWhy],
  ]) {
    const many = howMany(said, count);
    if (!many) continue;
    const term = document.createElement("dt");
    term.textContent = name || "";
    lists.appendChild(term);
    const value = document.createElement("dd");
    for (let at = 0; at < many; at += 1) {
      if (at) value.appendChild(document.createTextNode(" · "));
      const scene = document.createElement("span");
      const held9 = said[prefix + at];
      theValueIsDrawnInto(scene, held9 || "");
      value.appendChild(scene);
    }
    if (why) {
      const note = document.createElement("span");
      note.className = "qpn";
      note.textContent = " — " + why;
      value.appendChild(note);
    }
    lists.appendChild(value);
  }
  body.appendChild(lists);
  banner.appendChild(body);

  // **The control waits rather than going: nothing retries on its own.** Where
  // the submission cannot be sent it reads unavailable with the reason beside
  // it, which is the drawing's own shape for a control that cannot act.
  const controls = document.createElement("span");
  controls.className = "qbacts";
  const submit = aButton(said.thePartControl, "btn");
  controls.appendChild(submit);
  // **`Rendering` S11: it goes where the server is connected and a scene is
  // resolved as not submitted** — the application says which (the 38's piece
  // 2); otherwise it waits, with the reason beside it.
  if (said.thePartCanGo) {
    submit.addEventListener("click", () =>
      anActIsAskedFor(submit, "submit-the-remaining-scenes", { group: said.thePartServerRow }),
    );
  } else {
    submit.disabled = true;
    const why = document.createElement("span");
    why.className = "ax-why";
    why.textContent = said.needsAConnectionTo || "";
    why.appendChild(document.createTextNode(" "));
    const server = document.createElement("b");
    theValueIsDrawnInto(server, said.thePartServer || "");
    why.appendChild(server);
    why.appendChild(document.createTextNode(said.aFullStop || ""));
    controls.appendChild(why);
  }
  const grow = document.createElement("span");
  grow.className = "grow";
  controls.appendChild(grow);
  const hide = aButton(said.hide, "mini");
  hide.addEventListener("click", () => banner.remove());
  controls.appendChild(hide);
  banner.appendChild(controls);

  banners.appendChild(banner);
}

function theUntrackedIsDrawn(said) {
  if (!said.theUntrackedSays) return;
  const banner = document.createElement("div");
  banner.className = "qbanner amber";
  const body = document.createElement("div");

  const heading = document.createElement("p");
  heading.className = "qb1";
  aBadge(heading, said.theUntrackedBadge, true);
  const server = document.createElement("span");
  theValueIsDrawnInto(server, said.theUntrackedServer || "");
  heading.appendChild(server);
  heading.appendChild(document.createTextNode(" "));
  const says = document.createElement("span");
  says.textContent = said.theUntrackedSays;
  heading.appendChild(says);
  body.appendChild(heading);

  const second = document.createElement("p");
  second.className = "qb2";
  aFigureAndItsWord(second, said.theUntrackedRendering, said.renderingWord);
  second.appendChild(document.createTextNode(", "));
  aFigureAndItsWord(second, said.theUntrackedWaiting, said.waitingWord);
  // **The ending carries its own punctuation**, whichever the application sent
  // (must-do 19).
  const since = document.createElement("span");
  since.textContent = said.theUntrackedSince || "";
  second.appendChild(since);
  second.appendChild(document.createTextNode(" "));
  const cost = document.createElement("span");
  cost.textContent = said.theUntrackedCost || "";
  second.appendChild(cost);
  body.appendChild(second);
  banner.appendChild(body);

  const controls = document.createElement("span");
  controls.className = "qbacts";
  const track = aButton(said.trackIt, "mini");
  // **Track them accepts the whole offer for this server** (`Queue & History`
  // A6), so it hands back the server's identifier, never the name drawn
  // above — as Ignore does. Owed piece 10.
  itAnswers(track, "track-it", { group: said.theUntrackedServerRow || "" });
  controls.appendChild(track);
  // **Ignore hides the offer and Smoozy remembers the decline for as long as
  // the connection that raised it stands** (must-do 19, whose remembering was
  // step 10b's). The element goes at once so the press is answered without a
  // round trip; the application is told so the next reading does not bring the
  // same offer straight back.
  const ignore = aButton(said.ignore, "mini");
  ignore.addEventListener("click", () => {
    banner.remove();
    shell.core
      .invoke("the_untracked_work_is_ignored", {
        // **The server's identifier, never the name drawn above** — two
        // servers may share a name (step 10b round 1 of *jobs, history and
        // downloads*, must-do 6).
        server: said.theUntrackedServerRow || "",
        chosen: theChosenJob,
      })
      .catch(() => {});
  });
  controls.appendChild(ignore);
  banner.appendChild(controls);

  banners.appendChild(banner);
}

// ── The entry, which both screens draw ──────────────────────────────────────

/// **The job entry, in the shape Design draws once for both screens.**
///
/// `extra` lets the history add its own three groups without this builder
/// knowing what they are: `{ beforeTheReadings, afterTheMarks }`, each a
/// function taking the column to append into. **`extra.screen` names the
/// screen the entry is drawn on**, so a job's entry keeps its own place on
/// each (`web/the-scrolled-place.js`); the queue's is the default.
export function theEntryIsDrawn(into, said, extra) {
  const screen = (extra && extra.screen) || "queue";
  // **The large box is this entry's own preview, or the check's in its box**
  // (the nineteenth notes, note 7); any other job's goes.
  const theChecksKey = said.ffpreviewHeldCount !== undefined ? screen + " " + (said.ffJob || "") : null;
  if (theLargePreview && theLargePreview.screen === screen && theLargePreview.key !== screen + " " + (said.entryJob || "") && theLargePreview.key !== theChecksKey) thePreviewGoesAway(screen);
  // **An entry is drawn wherever the application sent one, and its job is what
  // says so** — never its name. Until step 10c's Vast pass this read the name,
  // so a job sent with no name drew an empty pane: no *Download all*, no
  // *Files on the server*, and its frames had no way home. A name can be empty
  // (a record kept before the file's path was); a job's identity cannot.
  if (!said.entryJob) return;
  const previewKey = screen + " " + said.entryJob;
  let lostPreview = !said.frameHeading && !!thePreviewSteps.get(said.entryJob)?.unavailable;
  if (theLargePreview && theLargePreview.key === previewKey && !said.frameHeading) {
    const steps = thePreviewSteps.get(said.entryJob);
    if (steps) steps.unavailable = true;
    thePreviewGoesAway(screen);
    lostPreview = true;
  }

  const header = document.createElement("div");
  header.className = "qdh";
  const naming = document.createElement("div");
  const name = document.createElement("h2");
  theValueIsDrawnInto(name, said.entryName || "");
  naming.appendChild(name);
  // **In Queue the entry is headed by the name alone; History keeps the line
  // under it** — `Design/Queue & History`, *A rendering entry*, the Creator's
  // ninth notes, note 1: *"the second line is unnecessary."* On the queue the
  // server's heading, the row and the name already say the rest, and a job
  // with no scene named says so in its mark. History is not grouped by server,
  // and its line is where a gone server is named.
  if (extra.screen === "history") naming.appendChild(theLineUnderTheName(said));
  header.appendChild(naming);

  // **Design, *The entry's controls*: the header is two rows** — the name and
  // its line, then the acts on a row of their own from the left edge, *Stop*
  // or *Cancel* last, and History's *Delete* last on History. Until the
  // Creator's notes on the test copy, piece 8 (note 11), the acts sat in the
  // name's row behind a spacer; where they did not fit they wrapped, the
  // spacer indented the second line and *Stop* fell to a third. `.qdacts` is
  // the drawing's own row.
  const acts = document.createElement("div");
  acts.className = "qdacts";

  // **Design: the entry's acts all sit in the header.** The two that reach back
  // to the machine go together; where the machine is gone they are absent and
  // one line sits where they were (`Queue & History` N17).
  //
  // **Both go to Transfers** — `Design/Queue & History`: *"Downloads appear in
  // Transfers"*, and the drawing's own `data-go="t"` on both. The job is named
  // by its identity, which the application hands over as a value.
  const job = said.entryJob || "";
  // **A job with no folder yet offers nothing that reaches into one** — `Job
  // files and outputs` P21, `Queue & History` Y8 — and a waiting one says why
  // where the downloads were: the drawing's `q-wait` and its `.mnote2`. The
  // application decides; the page draws. The Creator's second notes, piece 4's
  // queue half.
  const itsFolder = said.entryReachBacks && !said.entryNoFolder;
  if (itsFolder) {
    const all = aButton(said.downloadAll, "mini");
    all.addEventListener("click", () => aTransferIsAskedFor(DOWNLOAD_ALL, job));
    acts.appendChild(all);
    const allTo = aButton(said.downloadAllTo, "mini");
    allTo.addEventListener("click", () => aTransferIsAskedFor(DOWNLOAD_ALL_TO, job));
    acts.appendChild(allTo);
  } else if (said.entryReachBacks && said.theFolderIsMadeWhenTheRenderStarts) {
    const notYet = document.createElement("span");
    notYet.className = "mnote2";
    notYet.textContent = said.theFolderIsMadeWhenTheRenderStarts;
    acts.appendChild(notYet);
  }
  const pair = document.createElement("span");
  pair.className = "agroup";
  // **Not connected: *Files on the server* is not drawn, and comes back when
  // it connects** — `Design/File Browser`: *"A machine that is not connected
  // is not browsed and the action is not offered."* The application says
  // which; the downloads stay, being Download's to wait with.
  if (itsFolder && !said.entryNotConnected) {
    const onTheServer = aButton(said.filesOnTheServer, "mini");
    itAnswers(onTheServer, "files-on-the-server", { job });
    pair.appendChild(onTheServer);
  } else if (!said.entryReachBacks) {
    const gone = document.createElement("span");
    gone.className = "gonel";
    gone.textContent = said.theServerIsGone || "";
    pair.appendChild(gone);
  }
  const onYourComputer = aButton(said.filesOnYourComputer, "mini");
  itAnswers(onYourComputer, "files-on-your-computer", { job });
  pair.appendChild(onYourComputer);
  acts.appendChild(pair);
  // **Design: Cancel sits where Stop sits** — one act on the job, in the one
  // place the entry's acts live, and no job is ever both. The application
  // names which act it is.
  if (said.entryStopOrCancel) {
    const stopOrCancel = aButton(said.entryStopOrCancel, "mini", "danger");
    itAnswers(stopOrCancel, said.entryStopOrCancelAct, { job });
    acts.appendChild(stopOrCancel);
  }
  if (extra.headerActs) extra.headerActs(acts);
  header.appendChild(acts);
  into.appendChild(header);

  const body = document.createElement("div");
  body.className = "qdbody";
  // **The entry is the part that scrolls, and it keeps its place across a
  // redraw while it is the same job's** (`web/the-scrolled-place.js`); another
  // job's entry is another name, and starts at its top. The Creator's second
  // notes on the test copy, note 9.
  body.dataset.place = JSON.stringify([(extra.screen || "queue") + "-entry", said.entryJob]);

  const seam = aButton(null, "seam");
  seam.dataset.seam = "preview";
  body.appendChild(seam);
  shell.core
    .invoke("the_pane_layout")
    .then((layout) => {
      const declared = layout.seams.find((one) => one.key === "preview");
      if (!declared) return;
      seam.setAttribute("aria-label", declared.label);
      seam.title = declared.hint;
      theSeamsAreLive(body, layout.seams);
    })
    .catch(() => {});

  const main = document.createElement("div");
  main.className = "qdmain";
  body.appendChild(main);
  // **The check on its pair's entry, above the status line** — or, while the
  // pair waits, inside the question box ([`theHeldAnswersAreDrawn`]); where
  // no question box is drawn then, it sits here too.
  const theCheck = theCheckIsDrawn(said, screen, extra);
  if (theCheck && !(said.ffInTheQuestion && howMany(said, "heldCount"))) main.appendChild(theCheck.box);

  const status = document.createElement("div");
  status.className = "statusline";
  const dot = document.createElement("span");
  dot.classList.add("sdot");
  if (said.entryStatusIs) dot.classList.add(said.entryStatusIs);
  status.appendChild(dot);
  const words = document.createElement("span");
  words.className = "stx";
  const many = howMany(said, "entryStatusCount");
  for (let at = 0; at < many; at += 1) {
    if (at) words.appendChild(document.createTextNode(" "));
    aNamedPart(words, said, "entryStatus" + at);
  }
  status.appendChild(words);
  main.appendChild(status);
  if (lostPreview) {
    const lost = document.createElement("p");lost.className = "pverror";lost.setAttribute("role","status");
    lost.textContent = said.previewUnavailable || "";main.appendChild(lost);
  }

  // **Where a screen has something that belongs above the readings** — the
  // history's failure shape — it goes here, under the status line and above
  // everything else, which is where Design puts it.
  if (extra.afterTheStatus) extra.afterTheStatus(main);

  theHeldAnswersAreDrawn(main, said, job, theCheck);
  theProgressIsDrawn(main, said);
  theReadingsAreDrawn(main, said);
  if (extra.afterTheReadings) extra.afterTheReadings(main);
  theMarksAreDrawn(main, said, job);
  theWritesAreDrawn(main, said);
  theFactsAreDrawn(main, said);
  theBlenderOutputIsDrawn(main, said, job);

  const side = document.createElement("aside");
  side.className = "qdside";
  thePreviewIsDrawn(side, said, job, (extra && extra.screen) || "queue");
  // **Downloading and Telegram are one pair**, the drawing's `.dltg`: where
  // the window's first narrowing step sets them side by side, their two lists
  // sit on one line and are one width, the switch on its own line above
  // Downloading's (the Creator's twenty-fifth notes, note 1).
  const thePair = document.createElement("div");
  thePair.className = "dltg";
  theDownloadingIsDrawn(thePair, said);
  theTelegramIsDrawn(thePair, said);
  if (thePair.children.length) side.appendChild(thePair);
  body.appendChild(side);

  into.appendChild(body);
}

/// **History's line under the entry's name**: badge, server · file · scene,
/// in that order, the scene's field saying so where none was named.
function theLineUnderTheName(said) {
  const sub = document.createElement("p");
  sub.className = "qdsub";
  aBadge(sub, said.entryBadge, true);
  const server = document.createElement("span");
  theValueIsDrawnInto(server, said.entryServer || "");
  sub.appendChild(server);
  // **A server no longer in Smoozy: *Lambda A100, removed*** — the copied name,
  // then the words the application sent (`Design/Queue & History § History`,
  // the eighteenth notes, note 12).
  if (said.entryServerRemoved) {
    const removed = document.createElement("span");
    removed.textContent = said.entryServerRemoved;
    sub.appendChild(removed);
  }
  sub.appendChild(document.createTextNode(" · "));
  const blend = document.createElement("span");
  blend.className = "mono selectable";
  theValueIsDrawnInto(blend, said.entryBlend || "");
  sub.appendChild(blend);
  sub.appendChild(document.createTextNode(" · "));
  if (said.entryScene) {
    const scene = document.createElement("span");
    theValueIsDrawnInto(scene, said.entryScene);
    sub.appendChild(scene);
  } else {
    // **Design: the field says so rather than going blank.**
    const none = document.createElement("span");
    none.className = "unkw";
    none.textContent = said.entryNoScene || "";
    sub.appendChild(none);
  }
  return sub;
}

function theHeldAnswersAreDrawn(main, said, job, theCheck) {
  const many = howMany(said, "heldCount");
  for (let at = 0; at < many; at += 1) {
    const held = document.createElement("div");
    held.className = "jwheld";
    // **While a first-frame pair waits, the check's picture sits in the
    // question box** beside the question and its answers, under the check's
    // heading, and the question is not bold — the drawing's `.jwheld.ffask`
    // (the nineteenth notes, note 7).
    const asks = at === 0 && theCheck && said.ffInTheQuestion;
    let into = held;
    if (asks) {
      held.classList.add("ffask");
      if (theCheck.picture) held.appendChild(theCheck.picture);
      into = document.createElement("div");
      into.className = "ffq";
      into.appendChild(theCheck.heading);
      held.appendChild(into);
    }
    const says = document.createElement("p");
    const bold = document.createElement(asks ? "span" : "b");
    // **In parts**: the first-frame check names the job's own first frame as a
    // value (must-do 8).
    const saidParts = howMany(said, "held" + at + "SaidCount");
    for (let part = 0; part < saidParts; part += 1) {
      const piece = document.createElement("span");
      if (!aPart(piece, said, "held" + at + "Said" + part, asks)) continue;
      if (part) bold.appendChild(document.createTextNode(" "));
      bold.appendChild(piece);
    }
    says.appendChild(bold);
    if (said["held" + at + "Second"]) {
      says.appendChild(document.createTextNode(" "));
      const second = document.createElement("span");
      const held11 = said["held" + at + "Second"];
      second.textContent = held11;
      says.appendChild(second);
    }
    if (!asks || says.textContent) into.appendChild(says);
    const row = document.createElement("div");
    row.className = "row";
    // **Each button only where the application sends it**: a job held for
    // the driver update gets its box and no button, since nothing waits on a
    // person (`Queue & History` H12; the drawing's `q-driver`).
    const firstParts = howMany(said, "held" + at + "FirstCount");
    if (firstParts) {
      const first = aButton(null, "btn", "quiet");
      for (let part = 0; part < firstParts; part += 1) {
        if (part) first.appendChild(document.createTextNode(" "));
        aPart(first, said, "held" + at + "First" + part);
      }
      itAnswers(first, "the-held-jobs-first-answer", { job });
      row.appendChild(first);
    }
    if (said["held" + at + "Answer"]) {
      const second = aButton(said["held" + at + "Answer"], "mini");
      itAnswers(second, "the-held-jobs-second-answer", { job });
      row.appendChild(second);
    }
    const note = document.createElement("span");
    note.className = "note";
    const held12 = said["held" + at + "Note"];
    note.textContent = held12 || "";
    row.appendChild(note);
    into.appendChild(row);
    main.appendChild(held);
  }
}

/// **The check on its pair's entry** — `Design/Queue & History § A
/// first-frame check and its job`, the drawing's `.ffunit`: its picture where
/// one is sent, *First-frame check · Frame 1*, the sentence for the reading
/// the pair is at, and the check's own *Blender output* or *Show in Queue*
/// where they are sent. **Every word arrives on the event**, under `ff`; the
/// page joins none of them. Answers the box, and its picture and heading for
/// the question box to take while the pair waits; nothing where the entry is
/// no pair's. The nineteenth notes, note 7.
function theCheckIsDrawn(said, screen, extra) {
  if (!said.ffJob) return null;
  const box = document.createElement("div");
  box.className = "ffunit";
  const picture = theChecksPictureIsDrawn(said, screen);
  if (picture) box.appendChild(picture);
  const words = document.createElement("div");
  words.className = "ffu";
  const heading = document.createElement("p");
  heading.className = "ffw1";
  aPart(heading, said, "ffHeading", true);
  words.appendChild(heading);
  const many = howMany(said, "ff0SaidCount");
  if (many) {
    const sentence = document.createElement("p");
    sentence.className = "ffw2";
    for (let at = 0; at < many; at += 1) {
      if (at) sentence.appendChild(document.createTextNode(" "));
      aPart(sentence, said, "ff0Said" + at, true);
    }
    words.appendChild(sentence);
  }
  // ***Rendered in 2m, today 23:00. You chose **Render all**.*** — the
  // duration a value, the moment written as every moment is, the answer bold.
  if (said.ffRenderedIn || said.ffYouChose) {
    const sentence = document.createElement("p");
    sentence.className = "ffw2";
    if (said.ffRenderedIn) {
      sentence.appendChild(document.createTextNode(said.ffRenderedIn + " "));
      sentence.appendChild(theValueDrawn(said.ffRenderedFor || ""));
      if (said.ffWhen) sentence.appendChild(document.createTextNode(", " + theMomentReads(said.ffWhen)));
      sentence.appendChild(document.createTextNode((said.ffFullStop || "") + " "));
    }
    if (said.ffYouChose) {
      sentence.appendChild(document.createTextNode(said.ffYouChose + " "));
      const chose = document.createElement("b");
      chose.textContent = said.ffRenderAll || "";
      sentence.appendChild(chose);
      sentence.appendChild(document.createTextNode(said.ffFullStop || ""));
    }
    words.appendChild(sentence);
  }
  box.appendChild(words);
  if (said.ffBlenderOutput) {
    const open = aButton(said.ffBlenderOutput, "mini");
    open.addEventListener("click", () => {
      if (extra && extra.theChecksWords) extra.theChecksWords(said.ffJob);
      else shell.core.invoke("the_queue", { chosen: theChosenJob, asking: THE_CHECKS_BLENDER_OUTPUT, dropped: said.ffJob });
    });
    box.appendChild(open);
  }
  if (said.ffShowInQueue) {
    const show = aButton(said.ffShowInQueue, "mini");
    show.addEventListener("click", () => theCurrentPlaceIs(THE_QUEUE, said.ffShowInQueueJob));
    box.appendChild(show);
  }
  return { box, picture, heading: heading.cloneNode(true) };
}

/// **The check's picture, small, as a button that opens it large** — in the
/// preview's own large box, with every View Layer, by the preview's own
/// builder drawing the check's held copies (sent under `ff`) into a column
/// that is never shown. Escape or *Close* gives the focus back to this
/// picture. Nothing where no copy of the check is sent.
function theChecksPictureIsDrawn(said, screen) {
  if (said.ffpreviewHeldCount === undefined || said.ffpreviewNothing) return null;
  const checks = theChecksSaid(said);
  const frames = theFramesHeldIn(checks);
  const newest = frames[frames.length - 1];
  if (!newest) return null;
  const button = aButton(null, "ffpic");
  button.dataset.ffjob = said.ffJob;
  button.setAttribute("aria-label", checks.enlargePreview || "");
  const picture = document.createElement("img");
  picture.alt = "";
  thePictureIsAskedFor(picture, checks.previewPicture, newest.frame, newest.layers[newest.main].place);
  button.appendChild(picture);
  const unseen = document.createElement("aside");
  thePreviewIsDrawn(unseen, checks, said.ffJob, screen, {
    focusReturnsTo: () => document.querySelector('.ffpic[data-ffjob="' + CSS.escape(said.ffJob) + '"]'),
  });
  button.addEventListener("click", () => {
    const opens = unseen.querySelector(".preview");
    if (opens) opens.click();
  });
  return button;
}

/// **The check's preview keys, read as the entry's own** — every key the
/// preview's builder reads is under `ff` on a pair's entry (the application's
/// `the_check_is_said`), but *This preview is no longer available.*, which is
/// the entry's.
function theChecksSaid(said) {
  return new Proxy(said, {
    get(held, key) {
      if (typeof key !== "string" || key === "previewUnavailable") return held[key];
      return held["ff" + key];
    },
  });
}

function theProgressIsDrawn(main, said) {
  if (!said.renderingNow) return;
  const group = document.createElement("div");
  group.className = "grp";

  const now = document.createElement("div");
  now.className = "prog";
  const line = document.createElement("div");
  line.className = "pline";
  const label = document.createElement("span");
  label.className = "pl";
  label.textContent = said.renderingNow;
  line.appendChild(label);
  const many = howMany(said, "progNowCount");
  for (let at = 0; at < many; at += 1) {
    const part = document.createElement("span");
    part.className = at === 0 ? "pv mono" : "pn";
    if (aPart(part, said, "progNow" + at)) line.appendChild(part);
  }
  now.appendChild(line);
  // **Each bar only where the application sent its percentage** — absent
  // stays absent (`Queue & History` R4, `Queue & History` B4), never an empty bar standing for
  // a nought. Piece 6b. **A frame begun and not yet sampled is sent as 0**, so
  // its bar stands empty rather than going (W4, the twelfth notes, note 7).
  if (said.progNowAt !== undefined) {
    const bar = document.createElement("div");
    bar.className = "bar big";
    const fill = document.createElement("i");
    theFillIsAt(fill, said.progNowAt);
    bar.appendChild(fill);
    now.appendChild(bar);
  }
  group.appendChild(now);

  if (said.progWhole === undefined) {
    main.appendChild(group);
    return;
  }
  const whole = document.createElement("div");
  whole.className = "prog";
  const wholeLine = document.createElement("div");
  wholeLine.className = "pline";
  const wholeLabel = document.createElement("span");
  wholeLabel.className = "pl";
  wholeLabel.textContent = said.wholeJob || "";
  wholeLine.appendChild(wholeLabel);
  const at = document.createElement("span");
  at.className = "pv mono";
  at.textContent = (said.progWhole || "") + "%";
  wholeLine.appendChild(at);
  whole.appendChild(wholeLine);
  const wholeBar = document.createElement("div");
  wholeBar.className = "bar big whole";
  const wholeFill = document.createElement("i");
  theFillIsAt(wholeFill, said.progWhole);
  wholeBar.appendChild(wholeFill);
  whole.appendChild(wholeBar);
  group.appendChild(whole);

  main.appendChild(group);
}

function theReadingsAreDrawn(main, said) {
  const many = howMany(said, "readingCount");
  if (!many) return;
  const group = document.createElement("div");
  group.className = "grp";
  const line = document.createElement("div");
  line.className = "stat-line";
  for (let at = 0; at < many; at += 1) {
    const held = document.createElement("span");
    const name = document.createElement("span");
    const held13 = said["reading" + at + "Name"];
    name.textContent = held13 || "";
    held.appendChild(name);
    held.appendChild(document.createTextNode(" "));
    const value = document.createElement("b");
    value.className = "mono";
    const held14 = said["reading" + at + "Value"];
    theValueIsDrawnInto(value, held14 || "");
    held.appendChild(value);
    line.appendChild(held);
  }
  group.appendChild(line);
  main.appendChild(group);
}

function theMarksAreDrawn(main, said, job) {
  const many = howMany(said, "markCount");
  if (!many) return;
  const group = document.createElement("div");
  group.className = "grp";
  const heading = document.createElement("h3");
  heading.textContent = said.aboutThisJob || "";
  group.appendChild(heading);
  const held = document.createElement("div");
  held.className = "mks";
  for (let at = 0; at < many; at += 1) {
    const mark = document.createElement("p");
    mark.className = "mk1";
    mark.appendChild(document.createElement("i"));
    const says = document.createElement("span");
    const bold = document.createElement("b");
    const held15 = said["mark" + at + "Said"];
    bold.textContent = held15 || "";
    says.appendChild(bold);
    // **The first parts sit inside the bold where the application says how
    // many** — the seventh mark's driver and Blender (`Queue & History` N23).
    const inTheBold = howMany(said, "mark" + at + "Bold");
    const parts = howMany(said, "mark" + at + "PartCount");
    for (let part = 0; part < parts; part += 1) {
      const piece = document.createElement("span");
      if (!aPart(piece, said, "mark" + at + "Part" + part)) continue;
      const into = part < inTheBold ? bold : says;
      into.appendChild(document.createTextNode(" "));
      into.appendChild(piece);
    }
    if (said["mark" + at + "Way"]) {
      says.appendChild(document.createTextNode(" "));
      const way = aButton(said["mark" + at + "Way"], "glnk");
      itAnswers(way, "the-earlier-entry", { job });
      says.appendChild(way);
    }
    mark.appendChild(says);
    held.appendChild(mark);
  }
  group.appendChild(held);
  main.appendChild(group);
}

function theWritesAreDrawn(main, said) {
  const many = howMany(said, "writeCount");
  if (!many) return;
  const group = document.createElement("div");
  group.className = "grp";
  const heading = document.createElement("h3");
  heading.textContent = said.whatThisJobWrites || "";
  group.appendChild(heading);
  const held = document.createElement("div");
  held.className = "jw";
  for (let at = 0; at < many; at += 1) {
    const write = document.createElement("p");
    write.className = "jw1";
    write.appendChild(document.createElement("i"));
    const says = document.createElement("span");
    const bold = document.createElement("b");
    const held16 = said["write" + at + "Said"];
    bold.textContent = held16 || "";
    says.appendChild(bold);
    // **The first parts sit inside the bold where the application says how
    // many**, as on a mark — the duplicate-picture line is bold whole, the
    // drawing's `q-marks` state (the Creator's ninth notes, note 1, its E4 half).
    const inTheBold = howMany(said, "write" + at + "Bold");
    const parts = howMany(said, "write" + at + "PartCount");
    for (let part = 0; part < parts; part += 1) {
      const piece = document.createElement("span");
      if (!aPart(piece, said, "write" + at + "Part" + part)) continue;
      const into = part < inTheBold ? bold : says;
      into.appendChild(document.createTextNode(" "));
      into.appendChild(piece);
    }
    write.appendChild(says);
    held.appendChild(write);
  }
  group.appendChild(held);
  main.appendChild(group);
}

function theFactsAreDrawn(main, said) {
  const many = howMany(said, "factCount");
  if (!many) return;
  const group = document.createElement("div");
  group.className = "grp";
  const heading = document.createElement("h3");
  heading.textContent = said.submittedWith || "";
  const legend = document.createElement("span");
  legend.className = "leg";
  const italic = document.createElement("i");
  italic.textContent = said.legendItalic || "";
  legend.appendChild(italic);
  legend.appendChild(document.createTextNode(" "));
  const theFiles = document.createElement("span");
  theFiles.textContent = said.legendTheFiles || "";
  legend.appendChild(theFiles);
  legend.appendChild(document.createTextNode(" "));
  const bold = document.createElement("b");
  bold.textContent = said.legendBold || "";
  legend.appendChild(bold);
  legend.appendChild(document.createTextNode(" "));
  const yours = document.createElement("span");
  yours.textContent = said.legendYours || "";
  legend.appendChild(yours);
  heading.appendChild(legend);
  group.appendChild(heading);

  const facts = document.createElement("dl");
  facts.className = "facts";
  for (let at = 0; at < many; at += 1) {
    const fact = document.createElement("div");
    fact.className = "fact";
    const name = document.createElement("dt");
    const held17 = said["fact" + at + "Name"];
    name.textContent = held17 || "";
    fact.appendChild(name);
    const value = document.createElement("dd");
    // **The drawing's italic-and-bold convention**: the file's own value in
    // italic, anything the user changed upright and bold with the file's value
    // named beside it.
    //
    // **A value that is neither side's is drawn plain** — Smoozy's own
    // composition, or a setting the record cannot say anyone answered — so it
    // never reads as *bold = yours* (step 10c's sixth pass).
    value.className = said["fact" + at + "Neither"]
      ? "mono"
      : said["fact" + at + "Files"]
        ? "mono ff"
        : "mono uu";
    const held18 = said["fact" + at + "Value"];
    // **Lines under the value** — on *Writing to*, why the folder fell back,
    // one sentence a line (`Design/Queue & History`, *The job's name, and its
    // folder*; the drawing's `q-fallback` and `q-fallchosen`). As the drawing
    // draws them, the path keeps the fixed-width face in its own span and the
    // sentences under it do not.
    const under = howMany(said, "fact" + at + "UnderCount");
    // **On *Writing to*, the path is a link that opens Files on that folder**
    // — the same act as *Files on the server*, asked for the same job, in
    // Queue and History alike. The application says where (the Creator's
    // third notes, note 11, and seventh notes, note 5; the drawing's
    // `.jflink`).
    const theFolderOpens = !!said["fact" + at + "OpensFiles"];
    const aLink = () => {
      const link = document.createElement("a");
      link.href = "#";
      link.className = "jflink";
      link.addEventListener("click", (event) => {
        if (event && event.preventDefault) event.preventDefault();
        anActIsAskedFor(link, "files-on-the-server", { job: said.entryJob || "" });
      });
      return link;
    };
    if (under) {
      value.className = "";
      const path = theFolderOpens ? aLink() : document.createElement("span");
      path.className = theFolderOpens ? "jflink mono" : "mono selectable";
      theValueIsDrawnInto(path, held18 || "");
      value.appendChild(path);
      for (let line = 0; line < under; line += 1) {
        const why = document.createElement("span");
        why.className = "qfell";
        const held21 = said["fact" + at + "Under" + line];
        why.textContent = held21 || "";
        value.appendChild(why);
      }
    } else if (theFolderOpens) {
      const link = aLink();
      theValueIsDrawnInto(link, held18 || "");
      value.appendChild(link);
    } else {
      if (said["fact" + at + "IsPath"]) value.classList.add("selectable");
      theValueIsDrawnInto(value, held18 || "");
    }
    if (said["fact" + at + "Beside"]) {
      const beside = document.createElement("span");
      beside.className = "res";
      const held19 = said["fact" + at + "Beside"];
      beside.textContent = held19;
      value.appendChild(beside);
    }
    if (said["fact" + at + "Was"]) {
      const was = document.createElement("span");
      was.className = "was";
      was.textContent = said.theFilesValueLead || "";
      was.appendChild(document.createTextNode(" "));
      const held20 = said["fact" + at + "Was"];
      was.appendChild(theValueDrawn(held20));
      value.appendChild(was);
    }
    fact.appendChild(value);
    facts.appendChild(fact);
  }
  group.appendChild(facts);
  main.appendChild(group);
}

/// **The control that opens the record, and on a rendering entry Blender's
/// newest three lines above it** — the control under the lines, so they have
/// the entry's whole width (the Creator's third notes, note 14; the drawing's
/// `.log`, a column).
///
/// **`Design/Queue & History`, *A rendering entry*: a small window of the
/// ordinary output's newest three lines**, monospaced, each on one line and cut
/// with an ellipsis by the drawing's own `.qlines` rule. They are `Desktop app
/// and delivery` M13b's one narrowing and arrive composed by the application,
/// only while the job renders; **this file takes nothing out of a stream**.
/// Where none arrived — the first read not landed yet — **the control sits
/// alone under the heading, never inside an empty grey box**: the drawing's
/// `.log.nolines`, which takes the box's ground, border and padding away
/// (`Design/Queue & History § A rendering entry`; the Creator's note 12 on
/// their second notes, 2026-10-04 — the empty window had been drawn with a
/// spacer in it). History's entry draws its own the same way.
///
/// **A line that does not fit loses its start, not its end** (the Creator's
/// note 15): the drawing's `.qlines > span` lays each line out right to left
/// so the cut falls at its start, and **the line's own characters keep their
/// order** because [`theValueIsDrawnInto`] draws the value as one isolated
/// left-to-right run (`web/the-value-drawn.js`). Without that run a leading
/// time such as Blender 5's `00:00.430` would move to the line's far end, and
/// a closing quote to its start.
function theBlenderOutputIsDrawn(main, said, job) {
  if (!said.blenderOutput) return;
  const group = document.createElement("div");
  group.className = "grp";
  const heading = document.createElement("h3");
  heading.textContent = said.blenderOutput;
  group.appendChild(heading);
  const log = document.createElement("div");
  log.className = "log";
  const many = howMany(said, "entryLineCount");
  if (many) {
    const lines = document.createElement("span");
    lines.classList.add("qlines", "selectable");
    lines.classList.add("mono");
    for (let at = 0; at < many; at += 1) {
      const line = document.createElement("span");
      theValueIsDrawnInto(line, said["entryLine" + at] || "");
      lines.appendChild(line);
    }
    log.appendChild(lines);
  } else {
    log.classList.add("nolines");
  }
  const open = aButton(said.blenderOutput);
  itAnswers(open, "blender-output", { job });
  log.appendChild(open);
  group.appendChild(log);
  main.appendChild(group);
}

// **The pictures this page holds an address for**, by job and frame — the
// entry's own, and, on a first-frame pair's entry, the check's in its box (the
// nineteenth notes, note 7), so neither asks for its bytes again because the
// other was drawn. A redraw reuses them; a picture no longer among the newest
// few has its address revoked once a new one has arrived (H32). An image
// already drawn keeps its pixels after its address is revoked.
const theHeldPictures = [];
const THE_PICTURES_HELD = 3;

// **The picture's bytes, asked for from the application** —
// `the_preview_picture`, which reads only a copy the job record holds. Empty
// bytes are no picture, and the box stays as it is.
export function thePictureIsAskedFor(picture, job, frame, place) {
  const asked = { job, frame: Number(frame) };
  if (place !== undefined && place !== "") asked.place = Number(place);
  const key = job + " " + frame + " " + (asked.place === undefined ? "" : asked.place);
  const held = theHeldPictures.find((one) => one.key === key);
  if (held) {
    picture.src = held.url;
    return;
  }
  shell.core
    .invoke("the_preview_picture", asked)
    .then((bytes) => {
      if (!bytes || !bytes.byteLength) return;
      const url = URL.createObjectURL(new Blob([bytes], { type: "image/png" }));
      theHeldPictures.push({ key, url });
      while (theHeldPictures.length > THE_PICTURES_HELD) URL.revokeObjectURL(theHeldPictures.shift().url);
      picture.src = url;
    })
    .catch(() => {});
}

// ── The preview's frames and View Layers ────────────────────────────────────
//
// `Design/Queue & History`, *How the stepping and the View Layers look*;
// `Queue & History` P2 and P10a; the drawing's `q-pv1`, `q-pv`, `q-pvfew`,
// `q-pvsrc`, `h-pv1`, `h-pv` and `h-pvfew`. The Creator's seventh notes, note
// 4, and tenth notes, note 2: **a frame's main picture that is no one View
// Layer's is named by where it came from** — the application's words, with a
// File Output node's own name beside them as a value — and listed first.
//
// **The stepping is this page's alone and reads only the copies held**: the
// application hands every frame held and each frame's View Layers, and a step
// asks `the_preview_picture` for a copy the record already has — nothing is
// fetched and nothing is asked of the machine (P2).
//
// **Where the user is, per entry, outlives a redraw**: the queue is posted
// again every few seconds, and a step away, or a pick of a layer, that a
// redraw undid would be no step at all. `at` is the frame stepped to, or
// `null` while the preview follows the newest; `pick` is the View Layer the
// user is looking at, kept when a frame did not render it and back on the
// next frame that did.
const thePreviewSteps = new Map();

/// **The frames held, oldest first, each with its copies in the frame's own
/// order** — from the values the application said. An entry said before the
/// copies were listed has its one picture as one frame of one copy.
///
/// **Each copy's `name` is what the page lists it by**: a View Layer's own
/// name, or, for a picture that is no one layer's, the words the application
/// said for where it came from, with the File Output node's own name after
/// them (the Creator's tenth notes, note 2). Empty where the copy carries
/// neither.
function theFramesHeldIn(said) {
  const frames = [];
  const many = howMany(said, "previewHeldCount");
  for (let at = 0; at < many; at += 1) {
    const layers = [];
    const count = howMany(said, "previewHeldLayerCount" + at);
    for (let one = 0; one < count; one += 1) {
      const value = said["previewHeld" + at + "Layer" + one] || "";
      const source = said["previewHeld" + at + "Source" + one] || "";
      layers.push({
        name: source && value ? source + " " + value : source || value,
        place: said["previewHeld" + at + "Place" + one] || "",
        sourced: !!source,
      });
    }
    if (!layers.length) continue;
    const main = Number(said["previewHeldMain" + at] || 0);
    frames.push({ frame: said["previewHeldFrame" + at] || "", layers, main: layers[main] ? main : 0 });
  }
  if (!frames.length && said.previewFrame) {
    frames.push({ frame: said.previewFrame, layers: [{ name: "", place: "" }], main: 0 });
  }
  return frames;
}

// **The one View Layer list open**, closed by a press anywhere else — the
// way Render Settings' dropdowns close.
let theOpenViewLayerList = null;
document.addEventListener(
  "pointerdown",
  (press) => {
    if (theOpenViewLayerList && !theOpenViewLayerList.pick.contains(press.target)) theOpenViewLayerList.shut();
  },
  true
);

/// **A list that would be cut off at the bottom, with more room above its
/// control than under it, opens upward** — `.pick .ddl.up`, the rule Render
/// Settings' dropdowns open upward by (the drawing's own since 2026-10-19,
/// `web/the-drawing.css`; until then `web/render-settings.css`'s, both loaded
/// on every page), measured the way `web/render-settings.js` measures its own.
/// Answers whether it went up.
///
/// **The edge that cuts it off is the nearest one**: the window's, or that of
/// any box around the control that clips what it holds — the scrolling pane,
/// or the large preview's box (the Creator's eighteenth notes, note 11). Until
/// then only the window's bottom counted, so a list at the foot of the pane or
/// of the large box was cut off and still opened downward.
function theListIsKeptInsideTheWindow(face, list) {
  list.classList.remove("up");
  const room = list.getBoundingClientRect();
  const control = face.getBoundingClientRect();
  const edges = theEdgesThatCutOff(face);
  const below = edges.bottom - control.bottom;
  const above = control.top - edges.top;
  const up = room.bottom > edges.bottom && above > below;
  if (up) list.classList.add("up");
  return up;
}

/// **The top and bottom a list under `face` can be seen between**: the
/// window's, narrowed by every box around it whose overflow is not visible.
function theEdgesThatCutOff(face) {
  let top = 0;
  let bottom = window.innerHeight;
  const styleOf = typeof globalThis.getComputedStyle === "function" ? globalThis.getComputedStyle : null;
  for (let at = face.parentNode; styleOf && at && at.nodeType === 1; at = at.parentNode) {
    if (styleOf(at).overflowY === "visible") continue;
    const box = at.getBoundingClientRect();
    top = Math.max(top, box.top);
    bottom = Math.min(bottom, box.bottom);
  }
  return { top, bottom };
}

/// **The View Layer dropdown, in Smoozy's own dropdown look** — the parts
/// Render Settings' dropdowns are drawn with (`.pick`, `.pkb`, the `.ctx`
/// list as `.ddl`), the layer shown in the menu's bold, never the system's
/// list. Up and Down move, Return picks, Escape closes.
function aViewLayerDropdown(label, names, shown, steps, picked) {
  // **The row carries the label before the dropdown**, in the stepper's small
  // grey, so it reads as a choice and not as a caption — the drawing's
  // `.pvvlrow` and its `span.sn` (the Creator's fourteenth notes, note 7).
  // The word is the application's.
  const row = document.createElement("div");
  row.className = "pvvlrow";
  const named = document.createElement("span");
  named.className = "sn";
  named.textContent = label;
  row.appendChild(named);
  const pick = document.createElement("span");
  pick.className = "pick";
  pick.classList.add("pvvl");
  const face = aButton(shown, "pkb");
  face.setAttribute("aria-label", label);
  face.setAttribute("aria-haspopup", "listbox");
  pick.appendChild(face);
  const list = document.createElement("span");
  list.className = "ctx";
  list.classList.add("ddl");
  list.setAttribute("role", "listbox");
  list.setAttribute("aria-label", label);
  const options = names.map((name) => {
    const one = aButton(name);
    one.setAttribute("role", "option");
    one.setAttribute("aria-selected", name === shown ? "true" : "false");
    if (name === shown) one.className = "hd";
    one.addEventListener("click", () => {
      steps.open = false;
      picked(name);
    });
    list.appendChild(one);
    return one;
  });
  pick.appendChild(list);
  const shut = () => {
    steps.open = false;
    list.hidden = true;
    face.setAttribute("aria-expanded", "false");
    if (theOpenViewLayerList && theOpenViewLayerList.pick === pick) theOpenViewLayerList = null;
  };
  const opened = () => {
    if (theOpenViewLayerList) theOpenViewLayerList.shut();
    steps.open = true;
    list.hidden = false;
    // **A list with no room below it opens upward** (`Design/Whole app`, the
    // Creator's fourteenth notes, note 7). Measured where the dropdown is on
    // the screen; a redraw while the list stays open, before the dropdown is
    // placed, keeps the side it was measured to.
    if (pick.isConnected) steps.up = theListIsKeptInsideTheWindow(face, list);
    else list.classList.toggle("up", Boolean(steps.up));
    face.setAttribute("aria-expanded", "true");
    theOpenViewLayerList = { pick, shut };
  };
  if (steps.open) opened();
  else shut();
  face.addEventListener("click", () => (list.hidden ? opened() : shut()));
  pick.addEventListener("keydown", (press) => {
    const open = !list.hidden;
    if (press.key === "Escape" && open) {
      press.preventDefault();
      shut();
      face.focus();
    } else if (press.key === "ArrowDown" || press.key === "ArrowUp") {
      press.preventDefault();
      if (!open) opened();
      const at = options.indexOf(document.activeElement);
      const first = Math.max(0, names.indexOf(shown));
      const next = at < 0 ? first : at + (press.key === "ArrowDown" ? 1 : -1);
      if (options[next]) options[next].focus();
    } else if (press.key === "Tab" && open) {
      shut();
    }
  });
  row.appendChild(pick);
  return row;
}

// The seventeenth notes: selection is local to each entry and uses held copies.
let theLargePreview = null;
let thePreviewFocus = null;

/// **Where the large preview opens: over the whole window**, below the title
/// bar — `Design/Queue & History`, *"opens the same preview large over the
/// window"*, and the drawing's `#pvlarge`, which sits in `.body`, the element
/// holding the rail and the screen beside it (`web/the-rail.js` makes it in
/// `#pane`). The Creator's eighteenth notes, note 9: until then the box was
/// put inside the entry's own group, so the right pane bounded it. **Where the
/// window has no such element, the box stays with the entry**, as before.
function theLargePreviewsHost(group) {
  const pane = document.getElementById("pane");
  const body = pane && Array.from(pane.children).find((one) => one.classList.contains("body"));
  return body || group;
}

export function thePreviewRefreshStarts(where) {
  const active = document.activeElement;
  // **The large box sits outside the screen it belongs to**, so a focus
  // inside it is taken here too.
  const inTheLargeBox = !!(theLargePreview && active && theLargePreview.panel.contains(active));
  if (!active || !(where.contains(active) || inTheLargeBox)) return;
  if (active.dataset.previewField) {
    thePreviewFocus = { key: active.dataset.previewField, large: !!active.closest(".pvlarge"),
      start: active.selectionStart, end: active.selectionEnd };
  } else if (theLargePreview && theLargePreview.panel.contains(active)) {
    thePreviewFocus = {key:theLargePreview.key, large:true, control:active,
      label:active.getAttribute("aria-label"), value:active.textContent};
  }
}

export function thePreviewGoesAway(screen) {
  if (theLargePreview && theLargePreview.screen === screen) {
    theLargePreview.panel.remove();
    theLargePreview = null;
  }
}

function thePreviewIsDrawn(side, said, job, screen, opens) {
  if (!said.frameHeading) return;
  const key = screen + " " + job;
  // A job keeps its choice when it leaves Queue for History.
  const steps = thePreviewSteps.get(job) || { at: null, pick: undefined, open: false, typed: null, error: null };
  thePreviewSteps.set(job, steps);
  const group = document.createElement("div");
  group.className = "grp";
  const heading = document.createElement("h3");
  heading.textContent = said.frameHeading;
  const frames = theFramesHeldIn(said);
  const newest = frames.length - 1;
  if (screen === "history" && steps.at === null && frames[newest]) steps.at = frames[newest].frame;
  if (steps.pick === undefined && frames.length) steps.pick = frames[newest].layers[frames[newest].main].name || null;
  if (theLargePreview && theLargePreview.key === key) {
    const selected = frames.find(one => one.frame === theLargePreview.frame);
    if (said.previewNothing || !selected || said.previewPicture !== theLargePreview.pictureJob ||
        !selected.layers.some(one => one.name === theLargePreview.shown && one.place === theLargePreview.place)) {
      thePreviewGoesAway(screen);
      steps.unavailable = true;
    }
  }
  const closeLarge = () => {
    thePreviewGoesAway(screen);
    const picture = opens && opens.focusReturnsTo ? opens.focusReturnsTo() : group.querySelector('.preview');
    if (picture) picture.focus();
  };
  const move = (offset) => {
    const at = steps.at === null ? newest : frames.findIndex(one => one.frame === steps.at);
    const next = at + offset;
    if (!frames[next]) return;
    steps.at = screen === "queue" && next === newest ? null : frames[next].frame;
    steps.typed = null; steps.error = null; steps.open = false;
    drawn();
  };
  const drawInto = (into, large) => {
    let at = steps.at === null ? newest : frames.findIndex(one => one.frame === steps.at);
    if (at < 0) {
      at = newest;steps.at = screen === "history" && frames[at] ? frames[at].frame : null;steps.typed = null;
    }
    const frame = frames[at];
    if (!frame || said.previewNothing) {
      const silent = document.createElement('p'); silent.className = 'silent';
      silent.textContent = said.previewNothing || '';
      into.appendChild(silent);
      return;
    }
    const pickedAt = steps.pick ? frame.layers.findIndex(one => one.name === steps.pick) : -1;
    const source = frames.some(one => one.layers.some(layer => layer.sourced && layer.name === steps.pick));
    const missing = !!steps.pick && pickedAt < 0 && !source;
    const shown = frame.layers[pickedAt >= 0 ? pickedAt : frame.main];
    const preview = document.createElement('div'); preview.className = 'preview';
    const picture = document.createElement('img'); picture.className = 'thumb';
    if (said.previewPicture) thePictureIsAskedFor(picture, said.previewPicture, frame.frame, shown.place);
    picture.alt = '';
    preview.appendChild(picture);
    const which = document.createElement('span'); which.dataset.pvtag = 'yes';
    which.textContent = said.previewWhich || ''; which.hidden = at !== newest;
    preview.appendChild(which);
    if (!large) {
      preview.setAttribute('role', 'button'); preview.tabIndex = 0;
      preview.setAttribute('aria-label', said.enlargePreview || '');
      preview.addEventListener("click", openLarge);
      preview.addEventListener('keydown', press => {
        if (press.key === 'Enter' || press.key === ' ') {press.preventDefault(); openLarge();}
      });
    }
    into.appendChild(preview);
    const tools = large ? document.createElement('div') : into;
    if (large) {tools.className = 'pvtools'; into.appendChild(tools);}
    const stepper = document.createElement('div'); stepper.className = 'stepper pvstep';
    // **`data-pvgo` and `data-pvlatest` are the drawing's own hooks on this
    // row** (its `.pvstep` markup), which the drawing's rules of 2026-10-24
    // order by: *Frame*, then arrow, field and arrow touching, the shortcut
    // after. Without them the lifted rules put the field after the shortcut.
    const back = aButton('\u2039', 'sb'); back.dataset.pvgo = '-1'; back.setAttribute('aria-label', said.previousFrame || '');
    back.disabled = at <= 0; back.addEventListener("click", () => move(-1));stepper.appendChild(back);
    const number = document.createElement('label'); number.className = 'sn pvframe';
    number.appendChild(document.createTextNode(said.frameWord || ''));
    const field = document.createElement('input');field.className = 'num mono';field.type = 'text';field.inputMode = 'numeric';
    field.value = steps.typed === null ? frame.frame : steps.typed;
    field.dataset.previewField = key;
    field.setAttribute('aria-label', said.frameWord || '');
    field.setAttribute('aria-invalid', steps.error ? 'true' : 'false');
    field.addEventListener('input', () => {steps.typed = field.value;});
    field.addEventListener('keydown', press => {
      if (press.key !== 'Enter') return;
      press.preventDefault(); press.stopPropagation();
      const text = field.value.trim();
      steps.typed = field.value;
      if (!/^[+-]?\d+$/.test(text)) steps.error = {bad:true};
      else {
        // Integer strings are compared exactly, avoiding rounded large numbers.
        const canonical = BigInt(text).toString();
        const held = frames.find(one => one.frame === canonical);
        if (!held) steps.error = {frame:canonical};
        else {steps.at = held.frame; steps.typed = held.frame; steps.error = null;}
      }
      thePreviewFocus = {key, large, start:field.selectionStart, end:field.selectionEnd};
      drawn();
    });
    number.appendChild(field);stepper.appendChild(number);
    const on = aButton('\u203a', 'sb');on.dataset.pvgo = '1';on.setAttribute('aria-label', said.nextFrame || '');
    on.disabled = at >= newest;on.addEventListener("click", () => move(1));stepper.appendChild(on);
    const latest = aButton(screen === 'queue' ? said.showLatest : said.showLast, 'mini');latest.dataset.pvlatest = '';
    latest.disabled = at === newest && (screen !== 'queue' || steps.at === null);
    latest.addEventListener("click", () => {steps.at = screen === "history" ? frames[newest].frame : null;steps.typed = null;steps.error = null;steps.open = false;drawn();});
    stepper.appendChild(latest);tools.appendChild(stepper);
    const caption = document.createElement('p');caption.className = 'pvshown';
    caption.appendChild(document.createTextNode(said.showingFrame || ''));
    caption.appendChild(document.createTextNode(' '));caption.appendChild(theValueDrawn(frame.frame));tools.appendChild(caption);
    if (steps.error) {
      const error = document.createElement('p');error.className = 'pverror';error.setAttribute('role','status');
      if (steps.error.bad) error.textContent = said.frameInvalid || '';
      else {
        error.appendChild(document.createTextNode(said.frameWord || ''));error.appendChild(document.createTextNode(' '));
        error.appendChild(theValueDrawn(steps.error.frame));error.appendChild(document.createTextNode(' '));
        error.appendChild(document.createTextNode(said.frameUnavailable || ''));
      }
      tools.appendChild(error);
    }
    const names = frame.layers.map(one => one.name).filter(Boolean);
    if (names.length > 1) tools.appendChild(aViewLayerDropdown(said.viewLayer || '', names, shown.name || said.viewLayer || '', steps, name => {steps.pick = name;drawn();}));
    else if (names.length === 1) {
      const alone = document.createElement('p');alone.className = 'sn';theValueIsDrawnInto(alone,names[0]);tools.appendChild(alone);
    }
    if (missing) {
      const gone = document.createElement('p');gone.className = 'bar-note';gone.dataset.pvgone = 'yes';
      gone.appendChild(theValueDrawn(steps.pick));gone.appendChild(document.createTextNode(' '));
      gone.appendChild(document.createTextNode(said.didNotRenderOnThisFrame || ''));tools.appendChild(gone);
    }
    theNoteIsDrawn(tools, said);
    if (large && theLargePreview) {
      const title = theLargePreview.title;
      title.replaceChildren(document.createTextNode(said.frameWord || ""), document.createTextNode(" "), theValueDrawn(frame.frame));
      if (shown.name) {title.appendChild(document.createTextNode(" · "));title.appendChild(theValueDrawn(shown.name));}
      theLargePreview.shown = shown.name;theLargePreview.place = shown.place;
      theLargePreview.frame = frame.frame;theLargePreview.pictureJob = said.previewPicture;
    }
  };
  const openLarge = () => {
    steps.unavailable = false;
    const lost = group.querySelector("[data-pvlost]");if (lost) lost.remove();
    if (theLargePreview) theLargePreview.panel.remove();
    const panel = document.createElement('div');panel.className = 'modal pvlarge';
    panel.setAttribute('role','dialog');panel.setAttribute('aria-modal','true');panel.setAttribute('aria-label',said.frameHeading || '');
    const box = document.createElement('div');box.className = 'pvbox';
    const title = document.createElement('div');title.className = 'pvtitle';
    const name = document.createElement('h3');name.textContent = said.frameHeading || '';title.appendChild(name);
    const close = aButton('\u00d7','ib');close.setAttribute('aria-label',said.previewClose || '');close.addEventListener("click",() => theLargePreview && theLargePreview.close());title.appendChild(close);
    box.appendChild(title);const content = document.createElement('div');content.id = 'pvlarge-content';box.appendChild(content);panel.appendChild(box);
    panel.addEventListener("click", press => {if (press.target === panel && theLargePreview) theLargePreview.close();});
    panel.addEventListener('keydown', press => {
      if (press.key === 'Escape') {press.preventDefault();press.stopPropagation();if (theLargePreview) theLargePreview.close();return;}
      if (press.key === 'Tab') {
        const controls = Array.from(panel.querySelectorAll('button,input')).filter(one => !one.disabled && !one.closest('[hidden]'));
        const at = controls.indexOf(document.activeElement);
        if ((press.shiftKey && at <= 0) || (!press.shiftKey && at === controls.length-1)) {
          press.preventDefault();controls[press.shiftKey ? controls.length-1 : 0].focus();
        }
      }
      if ((press.key === 'ArrowLeft' || press.key === 'ArrowRight') && !press.target.closest('input') && !press.target.closest('textarea') && !press.target.closest('.pick')) {
        press.preventDefault();press.stopPropagation();if (theLargePreview) theLargePreview.move(press.key === 'ArrowLeft' ? -1 : 1);
      }
    });
    theLargePreview = {key, screen, panel, content, title:name, shown:null, close:closeLarge, move};
    theLargePreviewsHost(group).appendChild(panel);drawInto(content,true);close.focus();
  };
  if (theLargePreview && theLargePreview.key === key) {theLargePreview.close = closeLarge;theLargePreview.move = move;}
  const drawn = () => {
    thePreviewRefreshStarts(group);
    group.replaceChildren(heading);
    drawInto(group,false);
    if (steps.unavailable) {
      const gone = document.createElement('p');gone.className = 'pverror';gone.setAttribute('role','status');gone.textContent = said.previewUnavailable || '';gone.dataset.pvlost = 'yes';group.appendChild(gone);
    }
    if (theLargePreview && theLargePreview.key === key) {
      theLargePreview.content.replaceChildren();drawInto(theLargePreview.content,true);
      const into = theLargePreviewsHost(group);
      if (theLargePreview.panel.parentNode !== into) into.appendChild(theLargePreview.panel);
    }
    if (thePreviewFocus && thePreviewFocus.key === key) {
      const scope = thePreviewFocus.large && theLargePreview ? theLargePreview.panel : group;
      const focus = thePreviewFocus;
      let field = scope.querySelector('.pvframe')?.querySelector('input');
      if (focus.control) field = scope.contains(focus.control) ? focus.control :
        Array.from(scope.querySelectorAll("button")).find(one =>
          (focus.label && one.getAttribute("aria-label") === focus.label) || one.textContent === focus.value);
      const restore = () => {
        if (field && field.isConnected) {
          field.focus();
          if (!focus.control) field.setSelectionRange(focus.start,focus.end);
        }
      };
      if (field && !field.isConnected) Promise.resolve().then(restore);
      else restore();
      thePreviewFocus = null;
    }
  };
  side.appendChild(group);drawn();
}

/// ***Not colour-accurate* closes the preview block**, on every preview.
function theNoteIsDrawn(group, said) {
  const note = document.createElement("p");
  note.className = "bar-note";
  note.textContent = said.notColourAccurate || "";
  group.appendChild(note);
}

function theDownloadingIsDrawn(side, said) {
  // **No heading: the switch's own words name the group** — *Download frames
  // as they render*, the first line beside *Telegram* (the Creator's
  // twenty-fifth notes, note 1). The narrowing stays under it, on or off.
  if (!said.downloadingOn) return;
  const group = document.createElement("div");
  group.className = "grp";

  const row = document.createElement("div");
  row.className = "adl";
  const on = said.downloadingOn === "yes";
  const control = aButton(null, "sws");
  if (on) control.classList.add("on");
  // **On, the switch turns it off** (`Download` R3) — the application writes
  // it and draws the queue again. **Off, it asks where the files go and which
  // formats first** (`Download` R11: switching on asks once more), so it opens
  // the box whose *Done* switches it on. The 38's piece 9.
  if (on) itAnswers(control, "frames-as-they-render", { job: said.entryJob || "" });
  else control.addEventListener("click", () => theFormatBoxIsOpenedFor(said.entryJob || ""));
  control.setAttribute("aria-label", said.framesAsTheyRender || "");
  control.appendChild(document.createElement("i"));
  row.appendChild(control);
  const label = document.createElement("span");
  label.className = "adlt";
  label.textContent = said.framesAsTheyRender || "";
  row.appendChild(label);
  group.appendChild(row);

  const narrowing = aButton(null, "narrowb");
  // **Every format is said in words**, and a narrowed one names its formats as
  // values (the 38's piece 9).
  narrowing.textContent = said.downloadingNarrowing || said.downloadingEverything || "";
  // **The narrowing opens the format boxes** — the drawing's `data-go="fmt"`,
  // and piece 12's one call.
  const job = said.entryJob || "";
  narrowing.addEventListener("click", () => theFormatBoxIsOpenedFor(job));
  const chev = document.createElement("span");
  chev.className = "chev";
  chev.textContent = "▾";
  narrowing.appendChild(chev);
  group.appendChild(narrowing);

  const note = document.createElement("p");
  note.className = "bar-note";
  note.appendChild(document.createTextNode(said.downloadingDone || ""));
  note.appendChild(document.createTextNode(" "));
  note.appendChild(document.createTextNode(said.ofWord || ""));
  note.appendChild(document.createTextNode(" "));
  note.appendChild(document.createTextNode(said.downloadingOf || ""));
  note.appendChild(document.createTextNode(" "));
  note.appendChild(document.createTextNode(said.downloadedWord || ""));
  // **No count where there is nothing to count yet** — no arrangement, or none
  // that has read the job's whole (the 38's piece 9).
  if (said.downloadingOf && said.downloadingOf !== "0") group.appendChild(note);

  side.appendChild(group);
}

// ── What the job reports to Telegram ────────────────────────────────────────
//
// **`Telegram` V1c and `Design/Queue & History § The entry's controls`**: under
// *Downloading*, a group headed *Telegram* holds the Render Settings foot's own
// control — the five choices, and *A picture every* with its minutes where that
// is picked — reading the job's choice as the server holds it. **A pick saves
// at once**, with no confirmation; a typed minutes entry goes to the
// application, which takes it or refuses it beside the control. **Unavailable
// unless the application says it is offered**: not connected, or on a server
// whose Smoozy is older, where one grey line says so. The application decides
// everything here; the page reads `telegramStands` and nothing else. The
// twenty-first notes, note 7 (the drawing's `q-tg`, `q-tglist`, `q-tgold`).

// **Whether the list was open**, kept across the five-second redraws.
const theTelegramList = { open: false, up: false };
// **The minutes field last drawn, and its job**, for the redraw to keep what
// is typed in it (`theQueueIsDrawn`).
let theMinutesField = null;

function theTelegramIsDrawn(side, said) {
  if (!said.telegramHeading) return;
  const group = document.createElement("div");
  group.className = "grp";
  const heading = document.createElement("h3");
  heading.textContent = said.telegramHeading;
  group.appendChild(heading);
  const offered = said.telegramStands === "offered";
  const job = said.telegramJob || "";
  const count = Number(said.telegramChoiceCount || 0);
  const choices = [];
  for (let at = 0; at < count; at++) {
    choices.push({ value: said["telegramChoice" + at + "Value"], said: said["telegramChoice" + at + "Said"] });
  }
  const label = said.telegramLabel || "";
  const pick = document.createElement("span");
  pick.className = "pick";
  pick.classList.add("w-tg");
  const face = aButton(said.telegramReads || "", "pkb");
  face.setAttribute("aria-label", label);
  face.setAttribute("aria-haspopup", "listbox");
  face.disabled = !offered;
  pick.appendChild(face);
  const list = document.createElement("span");
  list.className = "ctx";
  list.classList.add("ddl");
  list.setAttribute("role", "listbox");
  list.setAttribute("aria-label", label);
  const shut = () => {
    theTelegramList.open = false;
    list.hidden = true;
    face.setAttribute("aria-expanded", "false");
    if (theOpenViewLayerList && theOpenViewLayerList.pick === pick) theOpenViewLayerList = null;
  };
  const options = choices.map((choice) => {
    const one = aButton(choice.said);
    one.setAttribute("role", "option");
    const picked = choice.value === said.telegramPicked;
    one.setAttribute("aria-selected", picked ? "true" : "false");
    if (picked) one.className = "hd";
    one.addEventListener("click", () => {
      shut();
      if (!picked) anActIsAskedFor(face, "what-it-reports", { job, from: choice.value });
    });
    list.appendChild(one);
    return one;
  });
  pick.appendChild(list);
  const opened = () => {
    if (theOpenViewLayerList) theOpenViewLayerList.shut();
    theTelegramList.open = true;
    list.hidden = false;
    if (pick.isConnected) theTelegramList.up = theListIsKeptInsideTheWindow(face, list);
    else list.classList.toggle("up", Boolean(theTelegramList.up));
    face.setAttribute("aria-expanded", "true");
    theOpenViewLayerList = { pick, shut };
  };
  if (offered && theTelegramList.open) opened();
  else shut();
  face.addEventListener("click", () => (list.hidden ? opened() : shut()));
  pick.addEventListener("keydown", (press) => {
    const open = !list.hidden;
    if (press.key === "Escape" && open) {
      press.preventDefault();
      shut();
      face.focus();
    } else if ((press.key === "ArrowDown" || press.key === "ArrowUp") && offered) {
      press.preventDefault();
      if (!open) opened();
      const at = options.indexOf(document.activeElement);
      const first = Math.max(0, choices.findIndex((choice) => choice.value === said.telegramPicked));
      const next = at < 0 ? first : at + (press.key === "ArrowDown" ? 1 : -1);
      if (options[next]) options[next].focus();
    } else if (press.key === "Tab" && open) {
      shut();
    }
  });
  const line = document.createElement("div");
  line.className = "sftg";
  line.appendChild(pick);
  // **The minutes follow the control on the same line**, a plain number field
  // in ordinary ink, as on the Render Settings foot.
  if (said.telegramMinutes !== undefined && said.telegramMinutes !== null) {
    const field = document.createElement("input");
    field.type = "text";
    field.className = "fld w-frame mono";
    field.value = said.telegramMinutes;
    field.dataset.box = "telegram-minutes-field";
    field.setAttribute("aria-label", said.telegramMinutesName || "");
    field.disabled = !offered;
    field.addEventListener("change", () =>
      anActIsAskedFor(field, "what-it-reports", { job, from: "minutes:" + field.value })
    );
    theMinutesField = { field, job };
    line.appendChild(field);
    const after = document.createElement("span");
    after.className = "hintl";
    after.textContent = said.telegramMinutesWord || "";
    line.appendChild(after);
  }
  group.appendChild(line);
  if (said.telegramOlder || said.telegramNotInstalled) {
    const note = document.createElement("p");
    note.className = "bar-note";
    note.textContent = said.telegramOlder || said.telegramNotInstalled;
    group.appendChild(note);
  }
  side.appendChild(group);
}

// ── The box that asks ───────────────────────────────────────────────────────

// **The boxes that ask about one job**, by the application's own names for them
// — `which` on `TheBoxThatAsks` in `src/surface/the_queue.rs` (*stop*,
// *cancel*) and `src/surface/the_history.rs` (*delete*). The two sides have to
// agree and nothing holds them together, so copy carefully. A group's box is
// about its group; the rest are about no one job and stay until answered.
const THE_BOXES_ABOUT_A_JOB = ["stop", "cancel", "delete"];

/// **A box stays until the user answers it or closes it, or what it asks about
/// leaves the reading** — step 10c's eleventh-pass fix piece 5.
/// `Design/Queue & History § Stopping asks first`; `Desktop app and delivery`
/// P10d.
///
/// **The case the two screens were first written for** was a reading that came
/// once, when the page asked, so drawing the screen again was drawing it for a
/// press — and every box came away with the old drawing. **The case they now
/// also meet** is P10d's reading, posted again about every five seconds while
/// the queue is on screen and carrying no box. **They parted when step 10c's
/// S3 gave *Stop* and *Cancel* their boxes on the live queue**: from then on a
/// box went five seconds after it was raised, its question still standing. Now
/// a reading with no box leaves an open box as it was — the same element, so
/// its focus stays — unless the user has answered it and nothing came back but
/// the redraw that follows, or its job or group is no longer in the reading. A
/// reading with a box of its own replaces the open one.
///
/// `jobs` and `groups` are what the reading holds, by identity.
export function theBoxStaysOrIsDrawn(into, said, jobs, groups) {
  const open = theOpenBoxOn(into);
  if (said.boxWhich) {
    if (open) open.remove();
    theBoxIsDrawn(into, said);
    return;
  }
  if (!open) return;
  const itGoes =
    open.dataset.answered === "yes" ||
    (open.dataset.job !== undefined && !jobs.has(open.dataset.job)) ||
    (open.dataset.group !== undefined && !groups.has(open.dataset.group));
  if (itGoes) open.remove();
}

function theOpenBoxOn(into) {
  for (const child of Array.from(into.children)) {
    if (child.classList && child.classList.contains("modal") && child.dataset.asks) return child;
  }
  return null;
}

/// **Every job a reading names**, by identity: `base` + position + *Job*, as
/// many as the count under `countKey`.
export function theJobsIn(said, countKey, base) {
  const jobs = new Set();
  const many = howMany(said, countKey);
  for (let at = 0; at < many; at += 1) {
    const job = said[base + at + "Job"];
    if (job) jobs.add(job);
  }
  return jobs;
}

function theGroupsIn(said) {
  const groups = new Set();
  const many = howMany(said, "groupCount");
  for (let at = 0; at < many; at += 1) {
    const group = said["group" + at + "Group"];
    if (group) groups.add(group);
  }
  return groups;
}

/// **A statement ending in a named button, never a question with Yes and No.**
/// Design says so for every box on these two screens, and it is `Servers`' own
/// shape for a removal.
export function theBoxIsDrawn(into, said) {
  if (!said.boxWhich) return;
  const modal = document.createElement("div");
  modal.className = "modal";
  // **What the box asks about**, so a later reading can say whether it still
  // stands ([`theBoxStaysOrIsDrawn`]).
  modal.dataset.asks = said.boxWhich;
  if (THE_BOXES_ABOUT_A_JOB.includes(said.boxWhich)) modal.dataset.job = said.entryJob || "";
  if (said.boxGroup) modal.dataset.group = said.boxGroup;
  const box = document.createElement("div");
  box.className = "mbox";

  const heading = document.createElement("h3");
  const heads = howMany(said, "boxHeadCount");
  for (let at = 0; at < heads; at += 1) {
    if (at) heading.appendChild(document.createTextNode(" "));
    aNamedPart(heading, said, "boxHead" + at);
  }
  box.appendChild(heading);

  // **The clear-all box lists every server it covers with that server's own
  // count**, because a single count does not show its reach. Each count has
  // its own word, singular or plural (must-do 12).
  const servers = howMany(said, "boxServerCount");
  let list = null;
  if (servers) {
    list = document.createElement("dl");
    list.className = "hcs";
    for (let at = 0; at < servers; at += 1) {
      const name = document.createElement("dt");
      const held = said["boxServer" + at];
      theValueIsDrawnInto(name, held || "");
      list.appendChild(name);
      const count = document.createElement("dd");
      const many = said["boxServer" + at + "Entries"];
      count.textContent = many || "";
      count.appendChild(document.createTextNode(BETWEEN_A_FIGURE_AND_ITS_UNIT));
      const word = said["boxServer" + at + "EntriesWord"];
      count.appendChild(document.createTextNode(word || ""));
      list.appendChild(count);
    }
  }

  // **The list sits between the box's first sentence and the rest**, as the
  // drawing has it for *Clear all history* and *Clear the history of removed
  // servers*: what goes, then whose, then what is untouched (step 11's D6b).
  const sentences = howMany(said, "boxSaidCount");
  for (let at = 0; at < sentences; at += 1) {
    const line = document.createElement("p");
    line.className = "msub";
    let wrote = false;
    const parts = howMany(said, "boxSaid" + at + "PartCount");
    for (let part = 0; part < parts; part += 1) {
      const held = document.createElement("span");
      if (!aNamedPart(held, said, "boxSaid" + at + "Part" + part)) continue;
      if (wrote) line.appendChild(document.createTextNode(" "));
      line.appendChild(held);
      wrote = true;
    }
    if (wrote) box.appendChild(line);
    if (at === 0 && list) box.appendChild(list);
  }
  if (list && !list.parentNode) box.appendChild(list);

  const row = document.createElement("div");
  row.className = "mrow";
  const grow = document.createElement("span");
  grow.className = "grow";
  row.appendChild(grow);
  const keep = aButton(said.boxKeep, "btn", "quiet");
  keep.addEventListener("click", () => modal.remove());
  row.appendChild(keep);
  const go = aButton(null, "btn");
  if (said.boxGrave === "yes") go.classList.add("danger");
  const goes = howMany(said, "boxGoCount");
  for (let at = 0; at < goes; at += 1) {
    if (at) go.appendChild(document.createTextNode(" "));
    aPart(go, said, "boxGo" + at);
  }
  // **Pressing it answers the box**: the reading drawn after it closes the box,
  // unless the application answers the press in it instead.
  go.addEventListener("click", () => {
    modal.dataset.answered = "yes";
  });
  // **The go-ahead answers too** (must-do 5): the application names its act.
  // **The dismiss box hands back the dropped job its line names** (`boxJob`,
  // the Creator's ninth notes, note 3); every other box, the entry's job.
  itAnswers(go, said.boxGoAct, { job: said.boxJob || said.entryJob || "", group: said.boxGroup || "" });
  row.appendChild(go);
  box.appendChild(row);

  modal.appendChild(box);
  into.appendChild(modal);
}

// ── A rendering job's words, live — `Queue & History` E8, piece 6c ─────────
//
// **The panel is the one every screen opens a render's words in**
// (`web/the-machines-own-words.js`): two tabs, one per stream, each under the
// name of the stream it came from, never merged (`Desktop app and delivery`
// M13, M13a). **Nothing here edits the words or lifts a line out of them** (M9,
// M13b): the first arrival is the stream's text so far and every later one is
// only what was printed since, set as a further text node after the last.
//
// **A tab with nothing yet says so in the application's words** (M10a's
// sentences, which arrive as prose), never an empty box (M10).
//
// **It sits on the screen's own holder and not inside the entry**, because the
// entry is drawn again on every reading and the panel must outlive that.

const THE_TWO_STREAMS = [
  { tab: "theOutputTab", says: "theOutputSays", said: "theOutputSaid", saidNext: "theOutputSaidNext", more: "theOutputMore" },
  { tab: "theErrorsTab", says: "theErrorsSays", said: "theErrorsSaid", saidNext: "theErrorsSaidNext", more: "theErrorsMore" },
];

let theRunningWords = null;

function theWordsArriveIn(body, more) {
  let printed = body.querySelector("pre.lbody");
  if (!printed) {
    body.replaceChildren();
    printed = document.createElement("pre");
    printed.className = "lbody";
    body.appendChild(printed);
  }
  printed.appendChild(document.createTextNode(more));
  // **The newest words stay in view while the panel holds the end**, and a
  // reader who scrolled up more than a line is left where they are —
  // `Design/Whole app`, the raw-text panel (the Creator's notes on the test
  // copy, piece 8). The holding is the panel's own.
  theNewestWordsStayInView(body);
}

whenTold(THE_RUNNING_WORDS, (delivery) => {
  if (!host) return;
  const said = delivery.said || {};
  const fresh = said.fresh || !theRunningWords || theRunningWords.job !== said.job;
  if (fresh) {
    if (theRunningWords) theRunningWords.panel.remove();
    const panel = theWordsPanel({
      title: said.title || "",
      subtitle: said.subtitle || "",
      tabs: THE_TWO_STREAMS.map((keys) => ({
        name: said[keys.tab] || "",
        says: said[keys.says],
        said: said[keys.said],
        saidNext: said[keys.saidNext],
        record: said[keys.more],
      })),
      note: said.note,
      close: said.close,
      copy: said.copy,
    });
    const foot = panel.querySelector(".lfoot");
    const close = foot && foot.querySelector("button");
    if (close) {
      // **Closing the panel ends its reads** — P10d repeats the commentary
      // read only while it is on screen.
      close.addEventListener("click", () => {
        if (theRunningWords && theRunningWords.panel === panel) theRunningWords = null;
        panel.remove();
        shell.core
          .invoke("the_queue", { chosen: theChosenJob, asking: THE_WORDS_ARE_PUT_AWAY })
          .catch(() => {});
      });
    }
    panel.hidden = false;
    host.appendChild(panel);
    theRunningWords = { job: said.job, panel };
    return;
  }
  for (const keys of THE_TWO_STREAMS) {
    const more = said[keys.more];
    if (!more) continue;
    const body = theRunningWords.panel.querySelector(
      '.words-body[data-stream="' + CSS.escape(said[keys.tab] || "") + '"]',
    );
    if (body) theWordsArriveIn(body, more);
  }
});

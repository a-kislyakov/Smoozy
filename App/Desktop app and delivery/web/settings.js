// **Settings, on the page** — one screen, a list of sections down its own side,
// one section showing at a time. The application half is
// `src/surface/settings.rs`, whose head carries the whole account.
//
// ═══════════════════════════════════════════════════════════════════════════
// HOW A SECTION IS OPENED FROM ELSEWHERE — one call, and nothing else
// ═══════════════════════════════════════════════════════════════════════════
//
//     import { theSettingsSectionIsOpened, TRANSFERS_SECTION } from "./settings.js";
//
//     theSettingsSectionIsOpened(TRANSFERS_SECTION);
//
// It goes to Settings through the shell's own `theCurrentPlaceIs(SETTINGS)` and
// shows that section. **It is the only way in from another screen**: the
// Transfers screen's link to the caps is this call with `TRANSFERS_SECTION`,
// The four section names are exported
// below; **import them, never type them** — they are the drawing's own section
// identifiers. A call made before Settings has ever been drawn is remembered and
// honoured when it is.
//
// ═══════════════════════════════════════════════════════════════════════════
//
// **Read the heads of `smoozy.js` and `the-rail.js` first.** This module keeps
// every part of both: it makes its own elements, adds nothing to `index.html`,
// registers under the shell's `stgview`, and draws only what an event handed
// it. **No prose is written in this file** (A6, X1), **no name or path is
// joined into a sentence** (X7), **values are content and never markup**
// (H32), **no inline style** (H33), and **nothing polls** (P9, P10).
//
// **The Telegram section is `web/telegram.js`'s**: this file makes its place in
// the list and hands it the element.

import { whenTold } from "./event-route.js";
import { theTelegramSectionIsBuilt, theNotificationSwitchesAre } from "./telegram.js";
import { aScreenIsRegistered, theCurrentPlaceIs, SETTINGS } from "./the-rail.js";
import { theWarningsSwitchedOff } from "./said-once.js";
import { theFailureShape, EXPANDED } from "./the-failure-shape.js";
import { theMarkOnAControl, theMarkComesOffAControl } from "./the-underway-shape.js";
import { aFieldIsBeingTypedIn, theTypingIsTakenFrom, theTypingIsPutBackInto } from "./the-typed-text.js";
import { theValueDrawn, theValueIsDrawnInto } from "./the-value-drawn.js";
import { theMomentReads } from "./servers.js";

const shell = window.__TAURI__;

// Declared beside the services that post them — `src/surface/settings.rs`,
// `src/surface/said_once.rs`. Copy carefully.
const THE_SCREEN = "settings:the-screen";
const THE_SEARCH_FOLDERS = "settings:the-search-folders";
const THE_SPEEDS = "settings:the-speeds";
// `TRANSFERS_MOVED` in `src/surface/transfers.rs`. Copy carefully.
const TRANSFERS_MOVED = "transfers:moved";
const THE_WARNINGS_SWITCHED_OFF = "settings:the-warnings-switched-off";

// **The drawing's own section identifiers**, which are also the elements'.
export const TRANSFERS_SECTION = "s-tr";
export const SEARCH_FOLDERS_SECTION = "s-fo";
export const WARNINGS_SECTION = "s-no";
export const TELEGRAM_SECTION = "s-tg";
const THE_SECTIONS = [TRANSFERS_SECTION, SEARCH_FOLDERS_SECTION, WARNINGS_SECTION, TELEGRAM_SECTION];

// **Where an answer to a change is drawn**, as `src/surface/settings.rs` names
// the three holders.
const THE_FOLDERS_ANSWER = "download-folder";
const THE_CAPS_ANSWER = "caps";
const THE_SWITCHES_ANSWER = "notifications";
// **The two directions and the four kinds, as the commands take them.** A
// kind's first name is the one the record stores it under
// (`src/record/settings.rs`); its second is its words' field, and the field
// saying whether it is on is that name with `IsOn` after it.
const UPLOAD = "upload";
const DOWNLOAD = "download";
const THE_FOUR_KINDS = [
  ["a-job-finishes", "aJobFinishes"],
  ["a-job-fails", "aJobFails"],
  ["a-connection-could-not-be-restored", "aConnectionCouldNotBeRestored"],
  ["a-server-has-had-nothing-to-do", "aServerHasHadNothingToDo"],
];

let screen = null;
let wanted = TRANSFERS_SECTION;
// **Whether Settings is the page showing**, so a transfer that moved asks for
// the readings beside the caps only where somebody can see them.
let itIsShowing = false;
// **An asking already on its way, and whether another is owed after it** — the
// same one-at-a-time Transfers keeps, so a burst of moves is one ask in flight.
let askingForTheSpeeds = null;
let askForTheSpeedsAgain = false;

/// **Open one section of Settings from anywhere.** See the head of this file.
export function theSettingsSectionIsOpened(section) {
  if (THE_SECTIONS.includes(section)) wanted = section;
  theCurrentPlaceIs(SETTINGS);
  if (screen) theSectionShows(wanted);
}

aScreenIsRegistered({
  place: SETTINGS,
  itIsDrawn(host) {
    screen = theScreenIsBuilt(host);
    theSectionShows(wanted);
  },
  itIsCurrentNow() {
    itIsShowing = true;
    // **Asked again every time** — events are volatile and nothing is
    // replayed (P8).
    shell.core.invoke("the_settings_screen");
    shell.core.invoke("the_warnings_switched_off");
  },
  itIsPutAway() {
    itIsShowing = false;
    theDragIsDropped();
  },
});

// ── Small makers ─────────────────────────────────────────────────────────────

function made(tag, shape) {
  const element = document.createElement(tag);
  if (shape && shape.className) element.className = shape.className;
  return element;
}

function aControl(shape) {
  const control = made("button", shape);
  control.type = "button";
  return control;
}

// **A reading as the application sent it** — the figure and its unit word, or,
// under a minute, the words alone (must-do 13). Nothing here picks a word.
//
// **The figure and its unit are one piece, joined by U+00A0**, so *6 days*
// never parts across two lines — `Design/Whole app § Typography`. The
// application sends them as a figure and its *Unit*, which is its own statement
// that the second is the first's unit; the character's one home is
// `src/typography.rs` (`BETWEEN_A_FIGURE_AND_ITS_UNIT`).
const BETWEEN_A_FIGURE_AND_ITS_UNIT = "\u00A0";
function aReadingFrom(said, name) {
  if (said[name + "Words"]) return [said[name + "Words"]];
  if (said[name] === undefined) return [];
  if (said[name + "Unit"] === undefined) return [said[name]];
  return [said[name] + BETWEEN_A_FIGURE_AND_ITS_UNIT + said[name + "Unit"]];
}

// Two strings drawn as one paragraph, which is what X10 hands over as two.
function theSentences(paragraph, ...said) {
  paragraph.replaceChildren();
  let first = true;
  for (const one of said) {
    if (!one) continue;
    if (!first) paragraph.appendChild(document.createTextNode(" "));
    paragraph.appendChild(document.createTextNode(one));
    first = false;
  }
}

// **An answer to the press the user just made, in the one failure shape.**
function theAnswerIs(holder, sentence) {
  holder.replaceChildren();
  if (!sentence) return;
  holder.appendChild(
    theFailureShape({ sentence, kind: "ending", controls: [], room: EXPANDED }),
  );
}

function aDate(seconds, withTheTime) {
  const at = new Date(Number(seconds) * 1000);
  const shape = { day: "numeric", month: "short" };
  if (withTheTime) {
    shape.hour = "2-digit";
    shape.minute = "2-digit";
  }
  return at.toLocaleString(undefined, shape);
}

// ── The screen ───────────────────────────────────────────────────────────────

function theScreenIsBuilt(host) {
  const head = made("div", { className: "qhead" });
  const title = made("h1");
  head.appendChild(title);

  const wrap = made("div", { className: "stgwrap" });
  const nav = made("nav", { className: "stgnav" });
  const body = made("div", { className: "stgbody" });
  const buttons = new Map();
  const sections = new Map();
  for (const id of THE_SECTIONS) {
    const control = aControl();
    control.dataset.stg = id;
    control.addEventListener("click", () => {
      wanted = id;
      theSectionShows(id);
    });
    nav.appendChild(control);
    buttons.set(id, control);
    const section = made("section", { className: "stgs" });
    section.id = id;
    section.hidden = true;
    body.appendChild(section);
    sections.set(id, section);
  }
  wrap.appendChild(nav);
  wrap.appendChild(body);
  host.appendChild(head);
  host.appendChild(wrap);

  theTelegramSectionIsBuilt(sections.get(TELEGRAM_SECTION), host);

  return {
    title,
    nav,
    buttons,
    transfers: theTransfersSection(sections.get(TRANSFERS_SECTION)),
    folders: theFoldersSection(sections.get(SEARCH_FOLDERS_SECTION)),
    warnings: theWarningsSection(sections.get(WARNINGS_SECTION)),
    sections,
  };
}

function theSectionShows(id) {
  if (!screen) return;
  for (const [each, section] of screen.sections) section.hidden = each !== id;
  for (const [each, control] of screen.buttons) {
    if (each === id) control.setAttribute("aria-current", "true");
    else control.removeAttribute("aria-current");
  }
}

whenTold(THE_SCREEN, (delivery) => {
  if (!screen) return;
  const said = delivery.said || {};
  screen.title.textContent = said.title || "";
  screen.nav.setAttribute("aria-label", said.theSections || "");
  const names = {
    [TRANSFERS_SECTION]: said.transfersSection,
    [SEARCH_FOLDERS_SECTION]: said.searchFoldersSection,
    [WARNINGS_SECTION]: said.warningsSection,
    [TELEGRAM_SECTION]: said.telegramSection,
  };
  for (const [id, control] of screen.buttons) control.textContent = names[id] || "";
  theTransfersSay(said);
  theNotificationsSay(said);
  // **The idle message's switch is drawn a second time in the Telegram
  // section**, from this same event — one switch, one record (`Telegram` V2).
  theNotificationSwitchesAre(said);

  // **The answer to the change just made, where it did not land**, beside the
  // control it is about. Every other holder is cleared: an answer is gone at the
  // next reading (P7 — the screen is always drawn from what the record holds).
  const holders = {
    [THE_FOLDERS_ANSWER]: screen.transfers.folderAnswer,
    [THE_CAPS_ANSWER]: screen.transfers.capsAnswer,
  };
  for (const [name, holder] of Object.entries(holders)) {
    theAnswerIs(holder, name === said.theAnswerFor ? said.theAnswer || "" : "");
  }
  // **A switch's answer is drawn beside the switch that was pressed and no
  // other** — must-do 16 of step 10a round 1 of the queue, history, settings
  // and downloads. The answer names its switch; one pressed in the Telegram
  // section is that section's to draw.
  for (const [which, row] of screen.warnings.rows) {
    const mine = said.theAnswerFor === THE_SWITCHES_ANSWER && said.theAnswerAbout === which;
    theAnswerIs(row.answer, mine ? said.theAnswer || "" : "");
  }
  // **L8a: a record that did not answer is said once, at the top of the
  // transfers section and of the switches**, and neither draws a value.
  const unread = said.theSettingsWouldNotRead || "";
  theAnswerIs(screen.warnings.answer, "");
  if (unread && !said.theAnswerFor) {
    theAnswerIs(screen.transfers.folderAnswer, unread);
    theAnswerIs(screen.warnings.answer, unread);
  }
});

// ── Transfers — `Design/Settings § Transfers` ────────────────────────────────

function theTransfersSection(section) {
  const heading = made("h2");
  const row = made("div", { className: "stgrow" });
  const path = made("span", { className: "path mono selectable" });
  const choose = aControl({ className: "mini" });
  choose.addEventListener("click", () => {
    // The system's own picker; the folder comes back as the screen's event.
    shell.core.invoke("the_download_folder_is_chosen");
  });
  row.appendChild(path);
  row.appendChild(choose);
  const noFolder = made("p", { className: "hintl" });
  const elsewhere = made("p", { className: "hintl" });
  const folderAnswer = made("div", { className: "stg-answer" });

  const limits = made("h2", { className: "gap" });
  const caps = made("div", { className: "cmp" });
  const up = theCapRow(UPLOAD);
  const down = theCapRow(DOWNLOAD);
  caps.appendChild(up.row);
  caps.appendChild(down.row);
  const totals = made("p", { className: "hintl" });
  const capsAnswer = made("div", { className: "stg-answer" });

  for (const one of [heading, row, noFolder, elsewhere, folderAnswer, limits, caps, totals, capsAnswer]) {
    section.appendChild(one);
  }
  return { heading, path, choose, noFolder, elsewhere, folderAnswer, limits, up, down, totals, capsAnswer };
}

function theCapRow(direction) {
  const row = made("div", { className: "trow" });
  const name = made("span", { className: "tt" });
  const grow = made("span", { className: "grow" });
  // **What is actually running beside the cap** — `Design/Settings §
  // Transfers`. Drawn only from `THE_SPEEDS`, and empty where nothing moves.
  const speed = made("span", { className: "hintl mono" });
  speed.hidden = true;
  const field = made("input", { className: "num mono" });
  field.value = "";
  // **What was typed goes to the application as typed**, which converts it once
  // (R2), writes it, reads it back and draws the field from the record — so a
  // cap that was refused puts the saved one back rather than keeping the typing.
  field.addEventListener("change", () => {
    shell.core.invoke("a_speed_cap_is_set", { direction, typed: field.value });
  });
  const unit = made("span", { className: "hintl" });
  row.appendChild(name);
  row.appendChild(grow);
  row.appendChild(speed);
  row.appendChild(field);
  row.appendChild(unit);
  return { row, name, speed, field, unit };
}

// **The reading beside each cap, as the application published it** — the word
// and then the figure with its unit, which is a value and is drawn as one
// (X7). A direction with no reading draws nothing.
whenTold(THE_SPEEDS, (delivery) => {
  if (!screen) return;
  const said = delivery.said || {};
  for (const [cap, reading] of [
    [screen.transfers.up, said.uploadSpeed],
    [screen.transfers.down, said.downloadSpeed],
  ]) {
    cap.speed.replaceChildren();
    cap.speed.hidden = !reading;
    if (!reading) continue;
    cap.speed.appendChild(document.createTextNode(said.runningAt || ""));
    cap.speed.appendChild(document.createTextNode(" "));
    cap.speed.appendChild(theValueDrawn(reading));
  }
});

// **A transfer moved**, so the readings beside the caps are asked for again
// while Settings is showing. **The event carries nothing and the page asks**
// (P8); nothing here polls (P9, P10).
whenTold(TRANSFERS_MOVED, () => {
  if (!screen || !itIsShowing) return;
  if (askingForTheSpeeds) {
    askForTheSpeedsAgain = true;
    return;
  }
  askingForTheSpeeds = Promise.resolve(shell.core.invoke("the_speeds_in_settings"))
    .catch(() => {})
    .finally(() => {
      askingForTheSpeeds = null;
      if (askForTheSpeedsAgain && itIsShowing) {
        askForTheSpeedsAgain = false;
        Promise.resolve(shell.core.invoke("the_speeds_in_settings")).catch(() => {});
      }
    });
});

function theTransfersSay(said) {
  const t = screen.transfers;
  t.heading.textContent = said.downloadFolder || "";
  t.choose.textContent = said.choose || "";
  const unread = Boolean(said.theSettingsWouldNotRead);
  const folder = said.theDownloadFolder;
  // A path reads in the order its bytes run (X9, `web/the-value-drawn.js`).
  theValueIsDrawnInto(t.path, folder || "");
  t.path.hidden = !folder;
  t.choose.hidden = unread;
  t.noFolder.hidden = Boolean(folder) || unread;
  theSentences(t.noFolder, said.noFolderIsSet, said.smoozyAsksBeforeADownload);
  t.elsewhere.textContent = said.oneDownloadElsewhere || "";
  t.limits.textContent = said.speedLimits || "";
  for (const [cap, name, label, saved] of [
    [t.up, said.uploading, said.uploadCap, said.uploadCapIs],
    [t.down, said.downloading, said.downloadCap, said.downloadCapIs],
  ]) {
    cap.name.textContent = name || "";
    cap.field.setAttribute("placeholder", said.noCap || "");
    cap.field.setAttribute("aria-label", label || "");
    cap.unit.textContent = said.perSecond || "";
    // **The saved cap, and an empty field where none is set** (R8) — **except
    // in a field still being typed in**, whose text is the user's until its
    // answer goes (`Design/Whole app § The window`: text being typed is not
    // lost to a redraw; `web/the-typed-text.js`). Every answer about any
    // setting draws this section again, so a cap half-typed while another
    // setting's answer came back was emptied under the caret.
    if (!aFieldIsBeingTypedIn(cap.field)) cap.field.value = saved || "";
    cap.row.hidden = unread;
  }
  theSentences(t.totals, said.eachCapIsATotal, said.emptyMeansNoCap);
}

// ── Search folders — `Design/Settings § Search folders` ──────────────────────

function theFoldersSection(section) {
  const heading = made("h2");
  const covers = made("p", { className: "hintl" });
  const list = made("div", { className: "folders" });
  const unread = made("div", { className: "stg-answer" });
  const addRow = made("div", { className: "stgrow" });
  const add = aControl({ className: "mini" });
  add.addEventListener("click", () => {
    shell.core.invoke("a_search_folder_is_added");
  });
  addRow.appendChild(add);
  const how = made("p", { className: "hintl" });
  const answer = made("div", { className: "stg-answer" });
  for (const one of [heading, covers, list, unread, addRow, how, answer]) section.appendChild(one);
  return { heading, covers, list, unread, add, how, answer, rows: [] };
}

whenTold(THE_SEARCH_FOLDERS, (delivery) => {
  if (!screen) return;
  const said = delivery.said || {};
  const f = screen.folders;
  f.heading.textContent = said.heading || "";
  theSentences(f.covers, said.aFolderCoversEverything, said.chooseTheFolderHoldingAll);
  f.add.textContent = said.addAFolder || "";

  // `§ Search folders`' order line, with the drawing's own bold on *which
  // folder*. Five nodes and never one string.
  f.how.replaceChildren();
  const bold = made("b");
  bold.textContent = said.whichFolder || "";
  for (const one of [
    document.createTextNode(said.dragOrUseTheArrows || ""),
    document.createTextNode(" "),
    document.createTextNode(said.theFirstFolderHoldingAMatch || ""),
    document.createTextNode(" "),
    bold,
    document.createTextNode(" "),
    document.createTextNode(said.isTakenFrom || ""),
    document.createTextNode(" "),
    document.createTextNode(said.whereOneFolderHoldsTwo || ""),
  ]) {
    f.how.appendChild(one);
  }

  theDragIsDropped();
  f.list.replaceChildren();
  f.rows = [];
  const unreadList = said.theListWouldNotRead || "";
  f.list.hidden = Boolean(unreadList);
  theAnswerIs(f.unread, unreadList);
  const many = Number(said.folderCount || 0);
  for (let at = 0; at < many; at += 1) {
    const row = theFolderRow(said, at, many);
    f.list.appendChild(row);
    f.rows.push(row);
  }
  theAnswerIs(f.answer, said.theChangeDidNotSave || "");

  // **A folder already on the list flashes the row it is on**, which answers
  // the question without a box — `§ What this document left open`.
  if (said.alreadyOnIt !== undefined) {
    const row = f.rows[Number(said.alreadyOnIt)];
    if (row) {
      row.classList.remove("sfhit");
      void row.offsetWidth;
      row.classList.add("sfhit");
    }
  }
});

function theFolderRow(said, at, many) {
  const row = made("div", { className: "sfrow" });
  row.classList.toggle("missing", said[`folder${at}Unreadable`] === "yes");
  const order = made("span", { className: "ord mono" });
  order.textContent = String(at + 1);
  const grip = made("span", { className: "grip" });
  grip.textContent = "⠿";
  grip.setAttribute("aria-hidden", "true");
  grip.addEventListener("pointerdown", (press) => theDragBegins(press, row, at));
  const path = made("span", { className: "path mono selectable" });
  const theFolder = said[`folder${at}`] || "";
  theValueIsDrawnInto(path, theFolder);
  row.appendChild(order);
  row.appendChild(grip);
  row.appendChild(path);
  if (said[`folder${at}Unreadable`] === "yes") {
    // **The mark is the last search's**, and the words are Design's one word.
    const mark = made("span", { className: "fmsg" });
    mark.appendChild(made("span", { className: "fdot" }));
    mark.appendChild(document.createTextNode(said.couldNotBeRead || ""));
    // **And when that search ran** (`Dependency Upload & Remap` F1a) — the
    // drawing's *could not be read at 09:20*, in the user's own clock.
    const searchedAt = said[`folder${at}SearchedAt`];
    if (searchedAt !== undefined) {
      const when = made("span", { className: "fwhen" });
      when.textContent = " " + (said.at || "") + " " + theMomentReads(searchedAt);
      mark.appendChild(when);
    }
    row.appendChild(mark);
  }
  row.appendChild(made("span", { className: "grow" }));
  const up = aControl({ className: "mini mv" });
  up.textContent = "↑";
  up.setAttribute("aria-label", said.moveUp || "");
  up.disabled = at === 0;
  up.addEventListener("click", () => {
    shell.core.invoke("a_search_folder_is_moved", { from: at, to: at - 1 });
  });
  const down = aControl({ className: "mini mv" });
  down.textContent = "↓";
  down.setAttribute("aria-label", said.moveDown || "");
  down.disabled = at === many - 1;
  down.addEventListener("click", () => {
    shell.core.invoke("a_search_folder_is_moved", { from: at, to: at + 1 });
  });
  const remove = aControl({ className: "mini" });
  remove.textContent = said.remove || "";
  remove.addEventListener("click", () => {
    shell.core.invoke("a_search_folder_is_removed", { position: at });
  });
  row.appendChild(up);
  row.appendChild(down);
  row.appendChild(remove);
  return row;
}

// **Dragging a row** — `§ What this document left open`, third bullet: the row
// lifts and a line shows where it would land, rather than a gap opening. The
// drawing's own `sflift` and `sfdrop`. The list is short and does not scroll.
let dragging = null;

function theDragBegins(press, row, from) {
  if (!screen || press.button !== 0) return;
  press.preventDefault();
  const line = made("div", { className: "sfdrop" });
  dragging = { row, from, to: from, line, pointer: press.pointerId, grip: press.currentTarget };
  row.classList.add("sflift");
  dragging.grip.setPointerCapture(press.pointerId);
  dragging.grip.addEventListener("pointermove", theDragMoves);
  dragging.grip.addEventListener("pointerup", theDragEnds);
  dragging.grip.addEventListener("pointercancel", theDragIsDropped);
}

function theDragMoves(move) {
  if (!dragging) return;
  const rows = screen.folders.rows;
  // How many rows' middles are above the pointer is the slot it would land in.
  let slot = 0;
  for (const row of rows) {
    const box = row.getBoundingClientRect();
    if (move.clientY > box.top + box.height / 2) slot += 1;
  }
  dragging.to = slot > dragging.from ? slot - 1 : slot;
  if (dragging.to === dragging.from) {
    dragging.line.remove();
    return;
  }
  const before = rows[slot];
  if (before) screen.folders.list.insertBefore(dragging.line, before);
  else screen.folders.list.appendChild(dragging.line);
}

function theDragEnds() {
  if (!dragging) return;
  const { from, to } = dragging;
  theDragIsDropped();
  if (from !== to) shell.core.invoke("a_search_folder_is_moved", { from, to });
}

function theDragIsDropped() {
  if (!dragging) return;
  dragging.row.classList.remove("sflift");
  dragging.line.remove();
  dragging.grip.removeEventListener("pointermove", theDragMoves);
  dragging.grip.removeEventListener("pointerup", theDragEnds);
  dragging.grip.removeEventListener("pointercancel", theDragIsDropped);
  dragging = null;
}

// ── Warnings and notifications — `Design/Settings § Warnings and notifications`

function theWarningsSection(section) {
  const heading = made("h2");
  const kinds = made("div", { className: "cmp" });
  const rows = new Map();
  for (const [which] of THE_FOUR_KINDS) {
    const row = made("div", { className: "trow" });
    // **Drawn from what the record holds and from nothing else**: the switch
    // is turned only when the screen's event says it is, after the write has
    // landed and been read back (P7, L8a).
    const toggle = aControl({ className: "sws" });
    toggle.setAttribute("aria-pressed", "false");
    toggle.appendChild(made("i"));
    toggle.addEventListener("click", () => {
      const on = toggle.getAttribute("aria-pressed") === "true";
      shell.core.invoke("a_notification_switch_is_turned", { kind: which, on: !on, pressedIn: "settings" });
    });
    const name = made("span", { className: "tt" });
    row.appendChild(toggle);
    row.appendChild(name);
    const qualifier = made("span", { className: "hintl" });
    if (which === "a-job-finishes") {
      row.appendChild(made("span", { className: "grow" }));
      row.appendChild(qualifier);
    }
    kinds.appendChild(row);
    // **This switch's own answer, under its row** (must-do 16).
    const answer = made("div", { className: "stg-answer" });
    kinds.appendChild(answer);
    rows.set(which, { toggle, name, qualifier, answer });
  }
  const whileRunning = made("p", { className: "hintl" });
  const answer = made("div", { className: "stg-answer" });
  // **The restore control is `said-once.js`'s builder**, drawn into this holder.
  const restore = made("div");
  for (const one of [heading, kinds, whileRunning, answer, restore]) section.appendChild(one);
  return { heading, rows, whileRunning, answer, restore };
}

function theNotificationsSay(said) {
  const w = screen.warnings;
  w.heading.textContent = said.notifications || "";
  const unread = Boolean(said.theSettingsWouldNotRead);
  for (const [which, key] of THE_FOUR_KINDS) {
    const row = w.rows.get(which);
    row.name.textContent = said[key] || "";
    row.toggle.setAttribute("aria-label", said[key] || "");
    const on = said[`${key}IsOn`] === "yes";
    row.toggle.classList.toggle("on", on);
    row.toggle.setAttribute("aria-pressed", on ? "true" : "false");
    // **L8a: a switch nobody could read is not drawn at all**, rather than
    // drawn off.
    row.toggle.hidden = unread;
  }
  w.rows.get("a-job-finishes").qualifier.textContent = said.withTheFrame || "";
  theSentences(w.whileRunning, said.theseArriveOnlyWhileRunning, said.telegramWorksWhenItIsNot);
}

whenTold(THE_WARNINGS_SWITCHED_OFF, (delivery) => {
  if (!screen) return;
  const said = delivery.said || {};
  screen.warnings.restore.replaceChildren(
    theWarningsSwitchedOff({
      ...said,
      onRestore: () => {
        // The new count comes back as this same event. **P6: no other route.**
        shell.core.invoke("every_switched_off_warning_is_restored");
      },
    }),
  );
});

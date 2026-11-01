// **The dependency check — the twin of the render settings panel.**
//
// **This piece draws inside piece 5's panel mechanism and builds no panel of
// its own.** Read the head of `web/a-server.js` first: this module registers
// `THE_DEPENDENCY_CHECK`, is handed `{ head, body }` once per server, and is
// told again when the panel comes back from its strip. The pinned collapse
// control, the strip, the stacking and Escape on the panel are all that piece's.
//
// **The application composes and this file draws.** Every string arrives on
// `the_dependency_check_panel`'s answer, already through the typography point
// or the verbatim route (A6, X9), and nothing here writes a word of prose.
// **Every judgement is `App/Dependency Upload & Remap/`'s** — a row's state and
// the action that fits it, its qualifiers, the order of a group — and the four
// endings are `App/Render Settings/`'s. This file puts them where the drawing
// puts them.
//
// ## The four things that bind whatever a piece writes
//
// They are at the head of `web/smoozy.js` and this file keeps all four:
// results arrive as events (P6); values are content and never markup (H32); no
// inline script and no inline style (H33); nothing repeats on a timer (P9,
// P10).
//
// ## What this file must never do
//
// - **Never join a name, a figure or a path into a sentence.** X7. A sentence
//   arrives as pieces — words, or a value, never both — and `saidInto` sets them
//   side by side.
// - **Never rebuild a shape the shared half exports.** The one failure shape is
//   `theFailureShape`; the underway mark is `theMarkOnARow`; the machine's own
//   words are `theMachinesOwnWordsFor`. **The mockup draws this panel's own
//   copies of the failure shape and of the underway mark, and this application
//   refuses both.** A failed row is not one of them: the drawing writes its
//   reason as a plain red note and puts its cross among the row's buttons, and
//   this file draws it that way (`theCrossOn`).
//
// ## The way in
//
// **`theCheckIsOpenedOn(server, file)` is the way in**, and a check is of a
// file. `web/render-settings.js`'s band hands it the file those settings are
// of, and `web/files.js`'s *Check dependencies…* item and toolbar control hand
// it the chosen row's path. **`file` is the exact bytes** (`File Browser` L1a),
// never a display name. **A panel opened with no file still says it does not
// know which file to check** rather than guessing.
//
// **`theCheckIsShownOn(server, file)` is the second way in**: it opens the
// panel on the rows already there and starts nothing — the band's *Open the
// check* (must-do 11 of the step 10a round 1 audit of these pieces), and
// Transfers' since the Creator's note 8 on the test copy.

import {
  aPanelIsRegistered,
  thePanelIsOpenedOver,
  theStripReads,
  theCurrentTabIs,
  thePanelCollapsesAsItsChevronDoes,
  THE_DEPENDENCY_CHECK,
  FILES,
} from "./a-server.js";
import { theBrowserIsOpenedAt } from "./files.js";
import { whenTold } from "./event-route.js";
import { theFailureShape, EXPANDED } from "./the-failure-shape.js";
import { theMachinesOwnWordsFor } from "./the-machines-own-words.js";
import { theMarkOnARow } from "./the-underway-shape.js";
import { aTransferIsAskedFor } from "./transfers.js";
import { theSettingsSectionIsOpened, SEARCH_FOLDERS_SECTION } from "./settings.js";
import { theValueDrawn, theValueIsDrawnInto } from "./the-value-drawn.js";

const shell = window.__TAURI__;

// The act this panel hands Transfers — `TheAct` in `src/surface/transfers.rs`.
const UPLOAD_THE_FOUND = "upload-the-found";

// Declared beside the service that posts it — `AN_ACT_ON_THIS_PANEL` in
// `src/surface/the_dependency_check.rs`. Copy carefully.
const AN_ACT_ON_THIS_PANEL = "the-dependency-check:an-act";

// server → { head, body, file, said, answer, boxes, folderBoxOpen, foldedOpen }
// — `file` is the file's bytes; `foldedOpen` is which folded groups the user
// opened, by the group's key.
const panels = new Map();

// ════════════════════════════════════════════════════════════════════════════
// The way in, and the panel's registration
// ════════════════════════════════════════════════════════════════════════════

/// **Open the check on a file, over a server.** Opening it is the user asking
/// for the check, so the check starts.
export async function theCheckIsOpenedOn(server, file) {
  const held = thePanelFor(server);
  held.file = theBytesOf(file);
  held.said = null;
  held.answer = null;
  if (held.file.length) {
    await shell.core.invoke("the_check_is_started", { server, file: held.file });
  }
  thePanelIsOpenedOver(server, THE_DEPENDENCY_CHECK);
  theCheckIsAskedFor(server);
}

/// **Open the check on a file without starting one** — the rows already there,
/// and nothing asked of the server. The band's *Open the check*.
export async function theCheckIsShownOn(server, file) {
  const held = thePanelFor(server);
  held.file = theBytesOf(file);
  held.said = null;
  held.answer = null;
  thePanelIsOpenedOver(server, THE_DEPENDENCY_CHECK);
  theCheckIsAskedFor(server);
}

aPanelIsRegistered({
  panel: THE_DEPENDENCY_CHECK,
  itIsDrawn(parts, server) {
    const held = thePanelFor(server);
    held.head = parts.head;
    held.body = parts.body;
    theCheckIsAskedFor(server);
  },
  // **Events are volatile and nothing is replayed** (P8), so a panel coming
  // back from its strip asks again.
  itIsShowingAgain(server) {
    theCheckIsAskedFor(server);
  },
  // **Its strip pressed while it was never opened over this server**: it opens
  // on Render Settings' file and starts no check — the Creator's seventh
  // notes, note 2.
  theFileItIsOn(server) {
    const held = panels.get(server);
    return held ? held.file : null;
  },
  itIsShownOn(server, file) {
    return theCheckIsShownOn(server, file);
  },
});

/// **The file's bytes, as every caller should hand them.** Transfers' way back
/// still hands over the path as the application wrote it, so a string is
/// turned back into the bytes it was written from — exact for any path that
/// was text to begin with — rather than being refused by the command.
function theBytesOf(file) {
  if (typeof file === "string") return Array.from(new TextEncoder().encode(file));
  return file || [];
}

function thePanelFor(server) {
  let held = panels.get(server);
  if (held) return held;
  held = {
    head: null,
    body: null,
    file: [],
    said: null,
    answer: null,
    boxes: [],
    folderBoxOpen: false,
    foldedOpen: {},
  };
  panels.set(server, held);
  return held;
}

async function theCheckIsAskedFor(server) {
  const held = panels.get(server);
  if (!held || !held.head || !held.body) return;
  const said = await shell.core.invoke("the_dependency_check_panel", {
    server,
    file: held.file || [],
  });
  held.said = said;
  theCheckIsDrawn(server, said);
}

// **An act answers, and the page asks again.** The event carries only what the
// panel cannot: the answer, where the act could not happen.
whenTold(AN_ACT_ON_THIS_PANEL, async (delivery) => {
  const said = delivery.said || {};
  const held = panels.get(said.server);
  // **The event names the file as the application keys it**, which is the
  // same text the panel's own answer carries — the page never decodes bytes.
  // Before the panel has answered once, an act's answer is for the file just
  // asked for.
  if (!held || (held.said && (said.file || "") !== held.said.file)) return;
  held.answer = said.said || null;
  await theCheckIsAskedFor(said.server);
});

function theActIsTaken(server, act, row) {
  const held = panels.get(server);
  return shell.core.invoke("an_act_on_the_check", {
    server,
    file: (held && held.file) || [],
    act,
    row: row === undefined || row === null ? null : Number(row),
  });
}

/// **Two acts open the machine's own words rather than asking the application
/// to act**, and the subject is the application's, never this file's.
function theControlIsPressed(server, said, act, row) {
  if (act === "blender-output" || act === "what-the-server-said") {
    theMachinesOwnWordsFor(said.the_machines_words_subject, server);
    return;
  }
  theActIsTaken(server, act, row);
}

// ════════════════════════════════════════════════════════════════════════════
// Drawing it
// ════════════════════════════════════════════════════════════════════════════

function theCheckIsDrawn(server, said) {
  const held = panels.get(server);
  if (!held || !held.head || !held.body) return;
  const hadFocus = document.activeElement;
  const theBoxThatHadFocus =
    hadFocus && held.body.contains(hadFocus) ? hadFocus.dataset.box : null;

  held.head.replaceChildren();
  held.body.replaceChildren();
  for (const box of held.boxes) box.remove();
  held.boxes = [];

  theHeadIsDrawn(server, said, held.head);
  if (said.instead) {
    const told = document.createElement("p");
    told.className = "cempty";
    told.textContent = said.instead;
    held.body.appendChild(told);
  } else {
    theBodyIsDrawn(server, said, held);
  }

  if (theBoxThatHadFocus) {
    const again = held.body.querySelector(`[data-box="${theBoxThatHadFocus}"]`);
    if (again) again.focus();
  }

  // **The strip keeps counting**, and no strip carries a mark: *the strip's
  // job is to keep counting, and the figures do it* (the Creator's tenth
  // notes, note 5, made that true of both strips).
  theStripReads(server, THE_DEPENDENCY_CHECK, {
    title: said.the_strip_title,
    // In parts, since *Uploading 4 of 12* puts words between two figures.
    reading: said.the_strip_reading.length ? said.the_strip_reading : null,
  });
}

// ── The header ──────────────────────────────────────────────────────────────

function theHeadIsDrawn(server, said, head) {
  const heading = document.createElement("h2");
  // **The file's name is a link into Files** where the application sent
  // somewhere to go — the ninth notes, note 5.
  const where = said.show_in_files;
  saidInto(heading, said.title, {
    valueTag: where ? "button" : "span",
    eachValue: where ? (named) => theNameShowsTheFileInFiles(server, named, where) : null,
  });
  head.appendChild(heading);
  if (!said.the_reading_line.length && !said.still_reading) return;

  const line = document.createElement("div");
  line.className = "sub";
  let first = true;
  const between = () => {
    if (!first) line.appendChild(document.createTextNode(" · "));
    first = false;
  };
  for (const part of said.the_reading_line) {
    between();
    line.appendChild(saidInto(document.createElement("span"), part, { valueTag: "span" }));
  }
  // **The header's own line, not a row and not a veil**, carrying the underway
  // mark in the shape that says it cannot say how far along.
  if (said.still_reading) {
    between();
    const still = document.createElement("span");
    still.className = "ckstill";
    still.appendChild(theMarkOnARow(null));
    saidInto(still, said.still_reading, { valueTag: "span", valueClass: "mono" });
    line.appendChild(still);
  }
  head.appendChild(line);
}

// ── The body ────────────────────────────────────────────────────────────────

function theBodyIsDrawn(server, said, held) {
  const body = held.body;

  const statements = document.createElement("div");
  statements.className = "cstate";
  // **The summary sentence is the first line under the title, and it is the
  // answer to the whole check** — `Design/Dependency Upload & Remap § Above
  // the rows`, since the dependency check's fresh look (2026-10-18).
  if (said.the_summary) statements.appendChild(theSummary(said.the_summary));
  if (held.answer) statements.appendChild(theAnswer(held.answer));
  for (const one of said.the_statements) {
    const line = document.createElement("p");
    line.dataset.statement = one.which;
    // **A line with no words draws no bullet** — the folders line where
    // nothing was searched is *Check again* alone, at the line's left under
    // the summary, and as `alone` it keeps the button's own height (the
    // drawing's `chk-nosearch` and `.cstate p.alone .mini`, the Creator's
    // fourteenth notes, note 3). Until then it was a bullet and a button.
    const itHasWords = Boolean(one.said && one.said.pieces && one.said.pieces.length);
    if (itHasWords) line.appendChild(document.createTextNode("• "));
    else line.classList.add("alone");
    // **A folder named by its last folder name shows its full path on hover**
    // — the folders line and what *Add* found (the Creator's note 5 on 0.1.8).
    // **On the folders line each name is a link** that opens the folder in
    // the computer's own file manager, its place in the line handed back as
    // the row (the Creator's note 5 on the test copy).
    const opens = one.its_folders_open_with;
    saidInto(line, one.said, {
      firstIsBold: one.its_first_sentence_is_bold,
      lastIsBold: one.its_last_sentence_is_bold,
      valueClass: "mono",
      fullPaths: one.the_full_paths,
      folderOpened: opens ? (at) => theActIsTaken(server, opens, at) : null,
    });
    // ***Check again* sits on the folders line wherever a check ran** — the existing
    // act, which starts a sitting — **as the failure shapes' small button**,
    // not a link (the Creator's note 6 on the test copy).
    if (one.control) {
      if (itHasWords) line.appendChild(document.createTextNode(" "));
      const act = one.control.act;
      line.appendChild(aSmallButton(one.control.label, act, () => theActIsTaken(server, act)));
    }
    statements.appendChild(line);
  }
  if (said.the_once) statements.appendChild(aFailure(server, said, said.the_once));
  if (statements.childElementCount) body.appendChild(statements);

  if (said.the_tools) body.appendChild(theTools(server, said, held));
  if (said.the_selection) body.appendChild(theSelectionBar(server, said));
  if (said.the_shift) body.appendChild(theShift(server, said));

  // **All four endings take the whole-app failure shape, attached to the check
  // itself**, and none of them shows a list.
  if (said.the_ending) {
    const ending = document.createElement("div");
    ending.className = "ckend";
    ending.appendChild(aFailure(server, said, said.the_ending));
    body.appendChild(ending);
  }
  if (said.nothing_was_read) {
    const empty = document.createElement("p");
    empty.className = "cempty";
    empty.textContent = said.nothing_was_read;
    // **Where no check of the file is held, the sentence and under it the one
    // button that starts the check**, *Check dependencies* as the drawing's
    // `.btn`, centred together in `.cnone` (`chk-none`; the Creator's
    // thirteenth notes, note 2). An ending's *File not read* stands alone.
    const start = said.the_check_to_start;
    if (start) {
      const none = document.createElement("div");
      none.className = "cnone";
      none.appendChild(empty);
      const button = document.createElement("button");
      button.type = "button";
      button.className = "btn";
      button.dataset.box = start.act;
      button.textContent = start.label;
      button.addEventListener("click", () => theActIsTaken(server, start.act));
      none.appendChild(button);
      body.appendChild(none);
    } else {
      body.appendChild(empty);
    }
  }

  if (said.the_table) body.appendChild(theTable(server, said, held));
  if (said.the_foot) body.appendChild(theFoot(server, said));

  if (said.the_overwrite) theOverwriteIsDrawn(server, said, held);
  if (held.folderBoxOpen) theFolderBoxIsDrawn(server, said, held);
}

function theAnswer(answer) {
  const told = document.createElement("p");
  told.className = "ckanswer";
  told.dataset.box = "answer";
  told.textContent = answer;
  return told;
}

/// **The summary sentence, in the stretches the application composed**, each
/// in its colour — failed and missing red, not verified amber, already on the
/// server green — and side by side with a space between them unless a stretch
/// joins the one before it. Its figures are bold, as the drawing's are.
function theSummary(summary) {
  const line = document.createElement("p");
  line.className = "csum";
  summary.parts.forEach((part, at) => {
    if (at > 0 && !part.joins) line.appendChild(document.createTextNode(" "));
    const stretch = document.createElement("span");
    if (part.colour) stretch.className = part.colour;
    line.appendChild(saidInto(stretch, part.said));
  });
  return line;
}

/// **The one failure shape, handed its parts.** The order of the controls is
/// the application's and this file draws the list it is handed.
function aFailure(server, said, failure) {
  const cross = failure.controls.find((one) => one.is === "cross");
  return theFailureShape({
    sentence: failure.sentence,
    whatToDoAboutIt: failure.the_second_sentence,
    kind: "condition",
    identityOrMark: failure.which,
    values: failure.values.map((one) => [one.name, one.value]),
    controls: failure.controls.map((one) => ({
      is: one.is,
      label: one.label,
      act: one.act,
    })),
    room: EXPANDED,
    onAct: (act) => theControlIsPressed(server, said, act),
    onClear: () => {
      if (cross) theActIsTaken(server, cross.act);
    },
  });
}

// ── Above the rows ──────────────────────────────────────────────────────────

function theTools(server, said, held) {
  const tools = said.the_tools;
  const bar = document.createElement("div");
  bar.className = "ctools";
  if (tools.how_far) {
    const far = document.createElement("span");
    far.className = "prog";
    // **One run inside the drawing's `.prog`**, which lays its children out as
    // a column.
    far.appendChild(saidInto(document.createElement("span"), tools.how_far, { valueTag: "span" }));
    bar.appendChild(far);
  }
  if (tools.stop) {
    bar.appendChild(aLink(tools.stop, "stop-the-search", () => theActIsTaken(server, "stop-the-search")));
  }
  for (const folder of tools.unreadable) {
    const warn = document.createElement("span");
    warn.className = "warn";
    saidInto(warn, folder, { valueTag: "span" });
    bar.appendChild(warn);
  }
  const grow = document.createElement("span");
  grow.className = "grow";
  bar.appendChild(grow);
  bar.appendChild(
    aLink(tools.search_folders, "search-folders", () => {
      held.folderBoxOpen = true;
      theFolderBoxIsDrawn(server, said, held);
    })
  );
  return bar;
}

/// **The bar over two or more chosen rows** — `§ Acting on many rows`. It
/// says how many are chosen apart from how many *Accept* will take, and draws
/// only the acts the application sent, which are the ones that apply to at
/// least one chosen row.
function theSelectionBar(server, said) {
  const chosen = said.the_selection;
  const bar = document.createElement("div");
  bar.className = "bulk";
  const many = document.createElement("span");
  many.className = "n";
  saidInto(many, chosen.how_many, { valueTag: "span" });
  bar.appendChild(many);
  if (chosen.accept) {
    const accept = aLink("", "accept-over-the-selection", () => theActIsTaken(server, "accept"));
    saidInto(accept, chosen.accept, { valueTag: "span" });
    bar.appendChild(accept);
  }
  if (chosen.choose_a_folder) {
    bar.appendChild(aLink(chosen.choose_a_folder, "choose-a-folder", () => theActIsTaken(server, "choose-a-folder")));
  }
  if (chosen.ignore) {
    bar.appendChild(aLink(chosen.ignore, "ignore-over-the-selection", () => theActIsTaken(server, "set-aside")));
  }
  if (chosen.undo_the_guess) {
    bar.appendChild(aLink(chosen.undo_the_guess, "undo-over-the-selection", () => theActIsTaken(server, "undo-the-guess")));
  }
  return bar;
}

function theShift(server, said) {
  const shift = said.the_shift;
  const band = document.createElement("div");
  band.className = "shift";
  band.appendChild(saidInto(document.createElement("span"), shift.said, { valueClass: "mono", eachValue: (value) => value.classList.add("selectable") }));
  band.appendChild(saidInto(document.createElement("span"), shift.resolved, { valueTag: "span" }));
  const undo = document.createElement("button");
  undo.type = "button";
  undo.className = "undo";
  undo.dataset.box = "undo-the-shift";
  undo.textContent = shift.undo;
  undo.addEventListener("click", () => theActIsTaken(server, "undo-the-shift"));
  band.appendChild(undo);
  return band;
}

// ── The rows ────────────────────────────────────────────────────────────────

function theTable(server, said, held) {
  const table = said.the_table;
  const rows = document.createElement("div");
  rows.className = "rows";

  // **Grouped by state, in the order of what the user has to do about them**,
  // and the order is the application's. **The heading says the state once**:
  // its dot, its word and how many rows, in the state's colour; a group with
  // no colour has no dot. **There are no column headings and no checkbox**
  // (`§ A row`, since the dependency check's fresh look).
  for (const group of table.groups) {
    if (group.folded) {
      rows.appendChild(aFoldedGroup(server, said, held, group));
      continue;
    }
    const heading = document.createElement("div");
    heading.className = "cgrp";
    if (group.colour) {
      heading.classList.add(group.colour);
      const dot = document.createElement("span");
      dot.className = "cdot";
      heading.appendChild(dot);
    }
    heading.dataset.group = group.which;
    heading.appendChild(document.createTextNode(group.heading));
    const count = document.createElement("span");
    count.className = "k";
    saidInto(count, group.how_many, { valueTag: "span" });
    heading.appendChild(count);
    rows.appendChild(heading);
    for (const row of group.rows) rows.appendChild(aRow(server, said, row));
  }
  return rows;
}

/// **A group folded away** — *Already on the server*, green, with its size on
/// its heading, and *Ignored* at the bottom — the drawing's `details.cside`.
/// Whether the user opened it is kept across redraws.
function aFoldedGroup(server, said, held, group) {
  const folded = document.createElement("details");
  folded.className = "cside";
  if (group.colour) folded.classList.add(group.colour);
  folded.dataset.group = group.which;
  folded.open = Boolean(held.foldedOpen[group.which]);
  folded.addEventListener("toggle", () => {
    held.foldedOpen[group.which] = folded.open;
  });
  const summary = document.createElement("summary");
  const dot = document.createElement("span");
  dot.className = "cdot";
  summary.appendChild(dot);
  const title = document.createElement("span");
  title.textContent = group.heading;
  summary.appendChild(title);
  const count = document.createElement("span");
  count.className = "k";
  saidInto(count, group.how_many, { valueTag: "span" });
  summary.appendChild(count);
  const grow = document.createElement("span");
  grow.className = "grow";
  summary.appendChild(grow);
  const open = document.createElement("span");
  open.className = "open";
  summary.appendChild(open);
  folded.appendChild(summary);
  for (const row of group.rows) folded.appendChild(aRow(server, said, row));
  return folded;
}

/// **A row names the file once** — `§ A row`: the name, then its kind; one
/// location, the file's own text for it on hover; the notes, every one the
/// application sent, in its order; the size; at most one visible action; ⋯.
///
/// **Rows are chosen the way a file manager chooses them** — `§ Acting on
/// many rows`: a click chooses the row, Shift-click the range from the last
/// one clicked, and Cmd-click (Ctrl-click on Windows; the application says
/// which key) adds a row or takes it away. **Pressing a row's own button or
/// its ⋯ never changes what is chosen.** The selection is the application's
/// (`Dependency Upload & Remap` F22), so a press is an act and the page asks
/// again.
function aRow(server, said, row) {
  const line = document.createElement("div");
  line.className = "row";
  line.dataset.row = row.at;
  if (row.chosen) line.classList.add("sel");

  const what = document.createElement("span");
  what.className = "what";
  // **The name and the location read in the order their bytes run** —
  // `Desktop app and delivery` X9, drawn by `web/the-value-drawn.js` — and
  // a Windows path the file was saved with is cut at its `\` the same way
  // (step 10c's dependency wave piece 10b).
  const name = document.createElement("span");
  name.className = row.a_linked_blend ? "nm mono lib" : "nm mono";
  name.appendChild(theValueDrawn(row.name));
  what.appendChild(name);
  const kind = document.createElement("span");
  kind.className = "kd";
  saidInto(kind, row.kind, { valueTag: "span" });
  what.appendChild(kind);
  const where = document.createElement("span");
  where.className = "loc";
  where.title = row.the_file_says;
  saidInto(where, row.location, { valueTag: "span", valueClass: "mono" });
  what.appendChild(where);
  for (const note of row.notes) what.appendChild(aNote(server, row, note));
  line.appendChild(what);

  const size = document.createElement("span");
  size.className = row.size ? "sz mono" : "sz";
  size.textContent = row.size || "—";
  line.appendChild(size);

  const acts = document.createElement("span");
  acts.className = "acts";
  if (row.action) {
    const action = aRowControl(server, said, row.action, row.at);
    action.className = "a";
    acts.appendChild(action);
  }
  const more = document.createElement("button");
  more.type = "button";
  more.className = "more";
  more.setAttribute("aria-label", row.more);
  more.dataset.box = `more-${row.at}`;
  more.textContent = "⋯";
  more.addEventListener("click", () => theRowsMenuIsOpened(server, said, row, more));
  acts.appendChild(more);
  // **A failure still standing carries its cross here, after the action and
  // ⋯** — the drawing's *Failed* rows (`chk-sel`). Its reason is a plain red
  // note among the notes above.
  const standing = row.notes.find((note) => note.failure);
  if (standing) acts.appendChild(theCrossOn(server, row, standing.failure));
  line.appendChild(acts);

  if (row.choosable) {
    const adds = said.the_table.adds_a_row_with;
    line.tabIndex = 0;
    line.dataset.box = `row-${row.at}`;
    line.setAttribute("aria-selected", String(Boolean(row.chosen)));
    line.addEventListener("click", (press) => {
      if (aControlOnTheRowWasPressed(press.target, line)) return;
      theActIsTaken(server, theWayARowIsChosen(press, adds), row.at);
    });
    line.addEventListener("keydown", (press) => {
      if (press.key !== " " || press.target !== line) return;
      press.preventDefault();
      theActIsTaken(server, theWayARowIsChosen(press, adds), row.at);
    });
  }
  return line;
}

/// **Which choosing a press is**: Shift for the range, the computer's own
/// adding key for one row more or less, otherwise the row alone.
function theWayARowIsChosen(press, adds) {
  if (press.shiftKey) return "choose-a-range";
  if (adds === "meta" ? press.metaKey : press.ctrlKey) return "add-or-take-a-row";
  return "choose-a-row";
}

/// Whether the press landed on one of the row's own controls — its action,
/// its ⋯, a failure's cross — rather than on the row.
function aControlOnTheRowWasPressed(target, line) {
  for (let at = target; at && at !== line; at = at.parentElement) {
    if (at.tagName === "BUTTON" || at.tagName === "A") return true;
  }
  return false;
}

/// **One note, in its colour**, with what it names in full on hover. **A
/// failure still standing is a plain red note** — its reason and nothing more,
/// as the drawing's *Failed* rows write it; its cross is among the row's
/// buttons (`theCrossOn`).
function aNote(server, row, note) {
  const holder = document.createElement("span");
  holder.className = "note";
  if (note.colour) holder.classList.add(note.colour);
  if (note.hover) holder.title = note.hover;
  if (note.failure) {
    holder.textContent = note.failure.sentence;
    return holder;
  }
  return saidInto(holder, note.said, { valueTag: "span", valueClass: "mono" });
}

/// **The cross on a failed row, last among its buttons** — the drawing's `fx`,
/// after the action and ⋯. A row's failure that refused a write is a live
/// condition, so the cross is there to clear it; once pressed the reason stays
/// as a red note and the cross goes.
function theCrossOn(server, row, failure) {
  const cross = document.createElement("button");
  cross.type = "button";
  cross.className = "fx";
  cross.title = failure.clear_this;
  cross.setAttribute("aria-label", failure.clear_this);
  // A glyph rather than a word: nothing here is prose.
  cross.textContent = "\u00d7";
  cross.addEventListener("click", () => theActIsTaken(server, "clear-a-rows-failure", row.at));
  return cross;
}

/// **The rest of a row's actions, behind its three dots** — drawn in the
/// drawing's own menu classes, over this panel only.
function theRowsMenuIsOpened(server, said, row, from) {
  const held = panels.get(server);
  const sheet = held && held.body && held.body.parentElement;
  if (!sheet) return;
  const menus = document.createElement("div");
  menus.className = "menus";
  // **A box over the panel carries the dialog role**, which is what piece 5's
  // Escape steps aside for.
  menus.setAttribute("role", "dialog");
  const set = document.createElement("div");
  set.className = "menuset";
  const panel = document.createElement("div");
  panel.className = "ctx";
  const shut = () => {
    menus.remove();
    held.boxes = held.boxes.filter((box) => box !== menus);
    from.focus();
  };
  for (const item of row.more_items) {
    const control = aRowControl(server, said, item, row.at);
    control.addEventListener("click", shut);
    panel.appendChild(control);
  }
  set.appendChild(panel);
  menus.appendChild(set);
  menus.addEventListener("click", (press) => {
    if (press.target === menus) shut();
  });
  menus.addEventListener("keydown", (press) => {
    if (press.key !== "Escape") return;
    press.preventDefault();
    shut();
  });
  sheet.appendChild(menus);
  held.boxes.push(menus);
  const first = panel.querySelector("button");
  if (first) first.focus();
}

// ── Along the bottom ────────────────────────────────────────────────────────

/// **Only the button along the bottom**, naming the number of files and their
/// size — `§ Along the bottom`, since the dependency check's fresh look. The
/// figures are the summary sentence's, at the top.
function theFoot(server, said) {
  const foot = said.the_foot;
  const bar = document.createElement("div");
  bar.className = "cfoot";
  const grow = document.createElement("span");
  grow.className = "grow";
  bar.appendChild(grow);
  const upload = document.createElement("button");
  upload.type = "button";
  upload.className = "btn";
  upload.dataset.box = "upload";
  upload.disabled = foot.upload_is_unavailable;
  saidInto(upload, foot.upload, { valueTag: "span" });
  // **The upload is this panel's act and it runs in Transfers** —
  // `Design/Dependency Upload & Remap`: *"its uploads run in Transfers"*, and
  // the drawing's own `data-go="t"` on this control. The panel holds the rows
  // and the file's exact bytes, so the act is taken here, and then Transfers
  // is shown, where the request is (step 10c's dependency wave piece 6).
  upload.addEventListener("click", () =>
    theActIsTaken(server, "upload").finally(() => aTransferIsAskedFor(UPLOAD_THE_FOUND, server)),
  );
  bar.appendChild(upload);
  return bar;
}

// ── The two boxes over the panel ────────────────────────────────────────────

/// **Replace or stop, and never the browser's four** — and the live-job line is
/// the browser's own composition, handed over whole.
function theOverwriteIsDrawn(server, said, held) {
  const sheet = held.body.parentElement;
  if (!sheet) return;
  const asked = said.the_overwrite;
  const box = document.createElement("div");
  box.className = "modal top";
  box.setAttribute("role", "dialog");
  box.dataset.box = "overwrite";
  const inner = document.createElement("div");
  inner.className = "mbox";

  const title = document.createElement("h3");
  title.textContent = asked.title;
  inner.appendChild(title);
  const read = document.createElement("p");
  read.className = "msub";
  asked.what_was_read.forEach((sentence, at) => {
    if (at > 0) read.appendChild(document.createTextNode(" "));
    read.appendChild(document.createTextNode(sentence));
  });
  inner.appendChild(read);
  const location = document.createElement("div");
  location.className = "clist mono";
  // The location a replace would write over, in the order its bytes run (X9,
  // `web/the-value-drawn.js`).
  theValueIsDrawnInto(location, asked.location);
  inner.appendChild(location);

  if (asked.live_jobs) {
    const warn = document.createElement("p");
    warn.className = "mwarn hard";
    saidInto(warn, asked.live_jobs.said);
    for (const sentence of [
      asked.live_jobs.only_the_blend_and_its_folder,
      asked.live_jobs.a_render_may_finish_anyway,
    ]) {
      if (!sentence) continue;
      warn.appendChild(document.createTextNode(" "));
      warn.appendChild(document.createTextNode(sentence));
    }
    inner.appendChild(warn);
  }
  if (asked.the_folder) {
    const folder = document.createElement("p");
    folder.className = "msub";
    saidInto(folder, asked.the_folder, { firstIsBold: true, valueTag: "span" });
    inner.appendChild(folder);
  }
  if (held.answer) inner.appendChild(theAnswer(held.answer));

  const row = document.createElement("div");
  row.className = "mrow";
  const grow = document.createElement("span");
  grow.className = "grow";
  row.appendChild(grow);
  const stop = document.createElement("button");
  stop.type = "button";
  stop.className = "btn quiet";
  stop.dataset.box = "stop-the-overwrite";
  stop.textContent = asked.stop;
  stop.addEventListener("click", () => theActIsTaken(server, "stop-the-overwrite"));
  row.appendChild(stop);
  const replace = document.createElement("button");
  replace.type = "button";
  replace.className = "btn";
  replace.dataset.box = "replace";
  replace.textContent = asked.replace;
  replace.addEventListener("click", () => theActIsTaken(server, "replace"));
  row.appendChild(replace);
  inner.appendChild(row);

  box.appendChild(inner);
  box.addEventListener("keydown", (press) => {
    if (press.key !== "Escape") return;
    press.preventDefault();
    theActIsTaken(server, "stop-the-overwrite");
  });
  sheet.appendChild(box);
  held.boxes.push(box);
  if (!held.body.contains(document.activeElement)) stop.focus();
}

/// **Adding a search folder without leaving the check.** The list, its order
/// and what the order decides are Settings' and are not copied here.
function theFolderBoxIsDrawn(server, said, held) {
  const sheet = held.body.parentElement;
  if (!sheet) return;
  for (const box of held.boxes.filter((one) => one.dataset.box === "search-folders-box")) {
    box.remove();
  }
  const asked = said.the_folder_box;
  const box = document.createElement("div");
  box.className = "modal top";
  box.setAttribute("role", "dialog");
  box.dataset.box = "search-folders-box";
  const inner = document.createElement("div");
  inner.className = "mbox";

  const title = document.createElement("h3");
  title.textContent = asked.title;
  inner.appendChild(title);
  // **Settings' folders first, full paths, one line each, in Settings' order,
  // to read and not to edit**, with *Change them in Settings* opening Settings'
  // own list — the Creator's note 7 on the test copy. Each path reads in the
  // order its bytes run (`Desktop app and delivery` X9).
  if (asked.the_folders.length) {
    const heading = document.createElement("p");
    heading.className = "msub";
    heading.textContent = asked.your_search_folders;
    inner.appendChild(heading);
    const list = document.createElement("div");
    list.className = "clist mono";
    list.dataset.box = "the-search-folders";
    for (const folder of asked.the_folders) {
      const line = document.createElement("div");
      theValueIsDrawnInto(line, folder);
      list.appendChild(line);
    }
    inner.appendChild(list);
    const changeRow = document.createElement("div");
    changeRow.className = "mrow";
    const change = document.createElement("button");
    change.type = "button";
    change.className = "lnkb";
    change.dataset.box = "change-them-in-settings";
    change.textContent = asked.change_them_in_settings;
    change.addEventListener("click", () => {
      close();
      theSettingsSectionIsOpened(SEARCH_FOLDERS_SECTION);
    });
    changeRow.appendChild(change);
    inner.appendChild(changeRow);
  }
  const chooseRow = document.createElement("div");
  chooseRow.className = "mrow";
  const choose = document.createElement("button");
  choose.type = "button";
  choose.className = "mini";
  choose.dataset.box = "choose-a-search-folder";
  choose.textContent = asked.choose_a_folder;
  choose.addEventListener("click", () => theActIsTaken(server, "choose-a-search-folder"));
  chooseRow.appendChild(choose);
  // **The folder chosen and not yet added**, drawn in the order its bytes run
  // (`Desktop app and delivery` X9, `web/the-value-drawn.js`).
  if (asked.chosen) {
    const chosen = document.createElement("span");
    chosen.className = "mono";
    theValueIsDrawnInto(chosen, asked.chosen);
    chooseRow.appendChild(chosen);
  }
  inner.appendChild(chooseRow);
  for (const words of asked.what_adding_does) {
    const line = document.createElement("p");
    line.className = "msub";
    saidInto(line, words);
    inner.appendChild(line);
  }
  if (held.answer) inner.appendChild(theAnswer(held.answer));

  const close = () => {
    held.folderBoxOpen = false;
    box.remove();
    held.boxes = held.boxes.filter((one) => one !== box);
    const opener = held.body.querySelector('[data-box="search-folders"]');
    if (opener) opener.focus();
  };
  const row = document.createElement("div");
  row.className = "mrow";
  const grow = document.createElement("span");
  grow.className = "grow";
  row.appendChild(grow);
  const cancel = document.createElement("button");
  cancel.type = "button";
  cancel.className = "btn quiet";
  cancel.dataset.box = "cancel-the-folder";
  cancel.textContent = asked.cancel;
  cancel.addEventListener("click", close);
  row.appendChild(cancel);
  const add = document.createElement("button");
  add.type = "button";
  add.className = "btn";
  add.dataset.box = "add-a-search-folder";
  add.textContent = asked.add;
  // **Add closes the box, and the panel then says once what it did** —
  // `Design/Dependency Upload & Remap § The search folders on this panel`.
  // Until the Creator's note 5 on 0.1.8 the box was drawn again, empty, with
  // nothing said. **It waits for a folder to be chosen.**
  add.disabled = !asked.chosen;
  add.addEventListener("click", () => {
    close();
    theActIsTaken(server, "add-a-search-folder");
  });
  row.appendChild(add);
  inner.appendChild(row);

  box.appendChild(inner);
  box.addEventListener("keydown", (press) => {
    if (press.key !== "Escape") return;
    press.preventDefault();
    close();
  });
  sheet.appendChild(box);
  held.boxes.push(box);
  if (!box.contains(document.activeElement)) choose.focus();
}

// ════════════════════════════════════════════════════════════════════════════
// The small pieces every part uses
// ════════════════════════════════════════════════════════════════════════════

// **What stands between a figure and the words after it, where the application
// marks them bound** — U+00A0, so *24 GB* or *2 jobs* never parts across two
// lines. `Design/Whole app § Typography`; the character's one home is
// `src/typography.rs` (`BETWEEN_A_FIGURE_AND_ITS_UNIT`), and this page writes
// it only where the mark says, never by reading which word is a unit.
const BETWEEN_A_FIGURE_AND_ITS_UNIT = "\u00A0";

/// **Draw a sentence as the pieces the application composed** — words, or a
/// value, never both — and join nothing. Whether a piece follows the one before
/// it with nothing between them is the sentence's own fact, and it arrives with
/// the piece.
function saidInto(holder, words, options) {
  if (!words || !words.pieces) return holder;
  const valueTag = (options && options.valueTag) || "b";
  const valueClass = options && options.valueClass;
  let boldStillOwed = Boolean(options && options.firstIsBold);
  // **The last words bold** — the folders line where the folders changed.
  let lastWords = -1;
  if (options && options.lastIsBold) {
    words.pieces.forEach((piece, at) => {
      if (piece.value === null || piece.value === undefined) lastWords = at;
    });
  }
  // **The full path behind each of the first values**, shown on hover.
  const fullPaths = (options && options.fullPaths) || [];
  let values = 0;
  let first = true;
  words.pieces.forEach((piece, at) => {
    const isAValue = piece.value !== null && piece.value !== undefined;
    let node;
    if (isAValue) {
      const fullPath = fullPaths[values];
      values += 1;
      // **A name standing for a path is plain, with the path on hover** —
      // or, where the line opens its folders, **a link drawn as the page's
      // other links** (`a.ckfl`, as the drawing writes the folders line).
      const opened = fullPath && options && options.folderOpened;
      node = document.createElement(opened ? "a" : fullPath ? "span" : valueTag);
      if (opened) {
        const at = values - 1;
        node.href = "#";
        node.className = "ckfl";
        node.addEventListener("click", (event) => {
          event.preventDefault();
          opened(at);
        });
      }
      if (fullPath) node.title = fullPath;
      else if (valueClass) node.classList.add(valueClass);
      if (options && options.eachValue) options.eachValue(node);
      // **Every value reads in the order its bytes run** — a path Smoozy
      // found, a folder, a name — `Desktop app and delivery` X9, drawn by
      // `web/the-value-drawn.js`.
      theValueIsDrawnInto(node, piece.value);
    } else {
      node = document.createElement(boldStillOwed || at === lastWords ? "b" : "span");
      boldStillOwed = false;
      node.textContent = piece.said || "";
    }
    if (!first && !piece.joins) {
      holder.appendChild(document.createTextNode(piece.binds ? BETWEEN_A_FIGURE_AND_ITS_UNIT : " "));
    }
    first = false;
    holder.appendChild(node);
  });
  return holder;
}

/// **The file's name in the title is a link into Files** — `Design/Whole app §
/// Panels over a server`, added 2026-10-12 (the Creator's ninth notes, note
/// 5), and the drawing's `.golink` inside the `h2`, hover *Show in Files*.
/// Pressing it collapses the panel as its chevron does and shows Files on the
/// file's folder with the file chosen. **The folder and the name are the
/// application's**, sent with the panel; this page composes no path. Render
/// Settings' title does the same.
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

/// **The small button the failure shapes carry** (`mini`), for *Check again*
/// on the folders line.
function aSmallButton(label, box, pressed) {
  const button = document.createElement("button");
  button.type = "button";
  button.className = "mini";
  button.dataset.box = box;
  button.textContent = label;
  button.addEventListener("click", pressed);
  return button;
}

function aLink(label, box, pressed) {
  const link = document.createElement("button");
  link.type = "button";
  link.className = "lnk";
  link.dataset.box = box;
  link.textContent = label;
  link.addEventListener("click", pressed);
  return link;
}

function aRowControl(server, said, control, at) {
  const button = document.createElement("button");
  button.type = "button";
  button.dataset.box = `${control.act}-${at}`;
  button.textContent = control.label;
  button.disabled = Boolean(control.unavailable);
  button.addEventListener("click", () => theControlIsPressed(server, said, control.act, at));
  return button;
}

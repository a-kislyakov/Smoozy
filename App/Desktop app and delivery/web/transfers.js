// **Transfers, on the page** — the one window holding every upload and
// download, and the three boxes that ask what a job's outputs download as.
// The application half is `src/surface/transfers.rs`, whose head carries the
// whole account.
//
// **Read the heads of `smoozy.js` and `the-rail.js` first.** This module keeps
// every part of both: it registers under the shell's `transfersview`, makes its
// own elements, adds nothing to `index.html`, and draws only what the
// application composed. **No prose is written in this file** (A6, X1), **no
// name, figure or path is joined into a sentence** (X7), **values are content
// and never markup** (H32), **no inline style** (H33), and **nothing polls**
// (P9, P10) — the one ticking thing on this screen is the shared countdown,
// which is P10b's exemption and lives in `the-underway-shape.js`.
//
// ## What this file uses rather than rebuilds
//
// - **A file's bar and figure is `theMarkOnARow`**, and a waiting request's
//   next attempt is `theCountdown` — both from `the-underway-shape.js`.
// - **A failed file is `theFailureShape`**, collapsed onto the row.
// - **The dependency check's way back is `theCheckIsShownOn`**, which shows
//   the rows already there and starts nothing (the Creator's note 8 on the
//   test copy; it was `theCheckIsOpenedOn`, which starts a check), and **the
//   caps' link is `theSettingsSectionIsOpened`**, piece 11's.
//
// ## The three boxes that are opened from elsewhere
//
// **`theFormatBoxIsOpenedFor(job)` is exported** for the two places Design puts
// the choice: the auto-download switch on the render settings panel and the
// narrowing control on a queue entry. **The queue entry calls it.** The render
// settings panel calls `theFormatBoxIsOpenedForAFile(server, file, whenDone)`,
// which draws the same box from `the_download_panel` and hands its Done to the
// panel: a job not yet submitted has no arrangement for Transfers to keep, so
// the two choices go back to the page that is choosing them.
//
// ## The way in from the three screens that start a transfer
//
// **`aTransferIsAskedFor(act, whose, picked)` is exported and it is the one way
// in.**
// `Design/File Browser`: *"Transfers it starts are watched in Transfers."*
// `Design/Dependency Upload & Remap`: *"its uploads run in Transfers."*
// `Design/Queue & History`: *"Downloads appear in Transfers."* The drawing
// sends *Download all*, *Download all to…*, the file browser's *Download* and
// *Download to…*, and the check's upload control to this screen. **So the call
// goes to this screen first and then asks the application to start the
// transfer, and whatever the application answers is said here**, on the line an
// act answers on — the place the user is now looking. `whose` is the job's
// identity for a whole-job download and the server's for the other acts, and
// `picked` is what the file browser chose, which only its own two acts carry.
//
// **A *…to* act goes nowhere until its folder is chosen** — the Creator's
// fifth note 7, piece Q: *"Download all to... jumps right into transfers, even
// if I cancel out of the folder picker."* A panel closed without a folder
// leaves the user on Queue, History or Files with nothing said. **It goes to
// Transfers when the application says the folder was chosen**
// (`A_DOWNLOAD_FOLDER_WAS_CHOSEN`), not when the request answers, because the
// count that follows can take minutes and its *Counting files…* row and *Stop*
// are drawn here. **Any other answer to a *…to* act still waiting goes here
// first and is then said**, so a refusal before the panel opened is said on
// Transfers as it always was.

import { whenTold } from "./event-route.js";
import {
  aScreenIsRegistered,
  theCurrentPlaceIs,
  theRowsFigureIs,
  A_SERVER,
  TRANSFERS,
} from "./the-rail.js";
import { theFailureShape, A_COLLAPSED_ROW, EXPANDED } from "./the-failure-shape.js";
import { theMarkOnARow, theCountdown } from "./the-underway-shape.js";
import { theCheckIsShownOn } from "./the-dependency-check.js";
import { theSettingsSectionIsOpened, TRANSFERS_SECTION } from "./settings.js";
import { theHistoryEntryIsOpened } from "./the-history.js";
import { theValueIsDrawnInto } from "./the-value-drawn.js";

const shell = window.__TAURI__;

// Declared beside the service that posts it — `AN_ACT_ON_TRANSFERS` in
// `src/surface/transfers.rs`. Copy carefully.
const AN_ACT_ON_TRANSFERS = "transfers:an-act";
// `TRANSFERS_MOVED` in `src/surface/transfers.rs`, beside it. Copy carefully.
const TRANSFERS_MOVED = "transfers:moved";
// `A_DOWNLOAD_FOLDER_WAS_CHOSEN` in `src/surface/transfers.rs`, beside it.
// Copy carefully.
const A_DOWNLOAD_FOLDER_WAS_CHOSEN = "transfers:a-download-folder-was-chosen";
const THE_FORMAT_BOX = "format-box";

let screen = null;
let showing = "";
// **Whether Transfers is the page showing**, so a transfer that moved is asked
// about only where somebody can see the row.
let itIsShowing = false;
// **An asking already on its way, and whether another is owed after it**, so
// five hundred files landing ask five hundred times at most one at a time.
let asking = null;
let askAgain = false;
let theOpenMenu = null;
let theOpenFormatBox = null;
// **A *…to* act whose folder panel has not answered yet** — piece Q. One panel
// is up at a time, so one mark is enough.
let aFolderIsAwaited = false;

aScreenIsRegistered({
  place: TRANSFERS,
  itIsDrawn(host) {
    screen = theScreenIsBuilt(host);
  },
  itIsCurrentNow() {
    itIsShowing = true;
    // **Asked again every time** — events are volatile and nothing is
    // replayed (P8).
    theScreenIsAskedFor();
  },
  itIsPutAway() {
    itIsShowing = false;
    theMenuGoes();
    // **An act's answer is about the moment it was asked**, so it does not
    // wait for the user to come back.
    if (screen) screen.answer.hidden = true;
  },
});

// The two acts a queue or history entry sends here that now start a real
// request — `Design/Queue & History`: *"Downloads appear in Transfers."* The
// same strings `web/the-queue.js` sends.
const DOWNLOAD_ALL = "download-all";
const DOWNLOAD_ALL_TO = "download-all-to";

// The two acts a server's file browser sends here, which start the same
// request over what the user picked there — `Design/File Browser`: *"Transfers
// it starts are watched in Transfers."* The same strings `web/files.js` sends.
const DOWNLOAD = "download";
const DOWNLOAD_TO = "download-to";

/// **Go to Transfers and ask the application to start a transfer.** The going
/// comes first, so the answer is drawn where the user now is — **except for a
/// *…to* act, which goes when its folder is chosen** (piece Q, the head).
///
/// **The four downloads run for real** — a whole job since step 10c piece 10,
/// and what the browser picked since *Files' own Download* — and each takes its
/// own way in, because each has a pre-flight to put in front of the user: the
/// enumeration is made on the machine, and where something already exists, or a
/// file cannot be vouched for, or a branch refused, the question is asked once
/// before a byte moves (`Download` M6).
///
/// **`picked` is what the file browser chose, as paths on the machine**, and it
/// is sent exactly as the listing gave it (`File Browser` L1a). The other ways
/// in send nothing there.
/// **Go to Transfers and nothing else** — a server's Files bar carries its
/// transfer count as a link here (`Design/File Browser § The list`, the
/// Creator's notes piece 6).
export function theTransfersAreShown() {
  theCurrentPlaceIs(TRANSFERS);
}

export function aTransferIsAskedFor(act, whose, picked) {
  const to = act === DOWNLOAD_ALL_TO || act === DOWNLOAD_TO;
  if (to) aFolderIsAwaited = true;
  else theCurrentPlaceIs(TRANSFERS);
  if (act === DOWNLOAD_ALL || act === DOWNLOAD_ALL_TO) {
    return aDownloadIsAskedFor("a_whole_job_download_is_asked_for", {
      job: whose || "",
      to,
    });
  }
  if (act === DOWNLOAD || act === DOWNLOAD_TO) {
    return aDownloadIsAskedFor("a_file_download_is_asked_for", {
      server: whose || "",
      picked: picked || [],
      to,
    });
  }
  return theActIsTaken(act, whose || "");
}

function aDownloadIsAskedFor(command, asked) {
  return shell.core
    .invoke(command, asked)
    .then((answer) => {
      // **A *…to* act still waiting on its folder** goes to Transfers now,
      // before the answer is drawn — unless the panel was closed without one,
      // which leaves the user where they pressed (piece Q).
      if (asked.to && aFolderIsAwaited) {
        aFolderIsAwaited = false;
        if (!answer || answer.what === "TheUserChoseNoFolder") return;
        theCurrentPlaceIs(TRANSFERS);
      }
      theDownloadAnswered(answer);
      // **Asked again**, so a request that started is on the screen the user is
      // now looking at.
      theScreenIsAskedFor();
    })
    .catch(() => {
      if (asked.to) aFolderIsAwaited = false;
    });
}

// **The folder is chosen, so the user goes to Transfers** — piece Q. Told only
// where a *…to* act is waiting, so nothing else moves the user.
whenTold(A_DOWNLOAD_FOLDER_WAS_CHOSEN, () => {
  if (!aFolderIsAwaited) return;
  aFolderIsAwaited = false;
  theCurrentPlaceIs(TRANSFERS);
});

// **`Download` M6's question, asked once for the whole request and before a
// byte moves, with the count.** The four answers are the spec's own.
//
// **Read as the application sends it** (`TheDownloadAnswer` in
// `src/surface/transfers.rs`): a sentence and a value each go over as the plain
// string they hold, never as an object carrying one. The case this was first
// written for was that shape guessed from the drawing; the case it now also
// meets is the application's own reply. Until step 10c's twelfth-pass fix piece
// 2 the page read `label.said`, `act.value`, `request.value` and `said.said`,
// so the question's four buttons were blank, a press sent no answer, and
// *Download did not start* said nothing. `the_download_question_is_drawn_and_
// answered_from_what_the_application_sends` feeds this page that reply.
function theDownloadAnswered(answer) {
  if (!answer) return;
  // **The user closed the folder panel**, which is an ordinary answer: they
  // asked for nothing to happen and nothing did, so nothing is said and an
  // older answer is taken away rather than left reading as this one's.
  // **Or pressed *Stop* while the files were being counted** — step 10c's
  // twelfth-pass fix piece 3: nothing was fetched, and there is nothing to say.
  if (answer.what === "TheUserChoseNoFolder" || answer.what === "TheUserStoppedIt") {
    if (screen) screen.answer.hidden = true;
    return;
  }
  if (answer.what === "DidNotStart") {
    if (!screen) return;
    screen.answer.textContent = answer.said || "";
    screen.answer.hidden = !screen.answer.textContent;
    return;
  }
  // **An upload sent again did not start** — `Files`' own sentence, words and
  // values side by side (X7). The 38's piece 9, item 17.
  if (answer.what === "TheUploadDidNotStart") {
    if (!screen) return;
    screen.answer.replaceChildren();
    saidInto(screen.answer, answer.said, "span", "mono");
    screen.answer.hidden = false;
    return;
  }
  // **A file sent again started**: whatever an earlier act said is not about
  // it, and the row it starts is on the screen asked for next.
  if (answer.what === "ItStarted") {
    if (screen) screen.answer.hidden = true;
    return;
  }
  if (answer.what !== "ItAsks") return;
  theDownloadQuestionIsPut(answer);
}

// The act the application gives M6's *Stop* — `the_question` in
// `src/surface/transfers.rs`. Copy carefully.
const STOP_THE_DOWNLOAD = "stop";

function theDownloadQuestionIsPut(asks) {
  const box = aBox(true);
  // **Every word here is composed on the application side** — A6 and X1: the
  // interface accepts prose from nowhere else.
  // **Where nothing is already there, no count is sent and none is drawn** —
  // `Design/Transfers § A download meeting files already there`, the paragraph
  // of 2026-10-01; the answers are then *Download* and *Stop*. Step 10c's
  // Vast-pass fix piece 1.
  if (asks.headline) {
    const heading = el("h2");
    saidInto(heading, asks.headline, "b", "mono");
    box.body.appendChild(heading);
  }
  for (const beside of [asks.cannot_vouch_for, asks.refusals]) {
    if (!beside) continue;
    const line = el("p", "amber");
    saidInto(line, beside, "b", "mono");
    box.body.appendChild(line);
  }
  // **The upload's box, with one sentence changed** — `Design/Transfers § A
  // download meeting files already there`: nothing is put forward, so the four
  // sit level, and *Stop* is set apart as the upload's box sets it apart
  // (`web/files.js`, the collision box; the drawing's `.mrow`). *Stop* is
  // `Download` M6's request not proceeding at all, and M12's files never
  // arranged.
  const row = el("div", "mrow");
  for (const answer of asks.answers || []) {
    if (answer.act === STOP_THE_DOWNLOAD) row.appendChild(el("span", "grow"));
    const control = el("button", "btn", "quiet");
    control.type = "button";
    control.textContent = answer.label;
    control.addEventListener("click", () => {
      box.close();
      shell.core
        .invoke("the_download_question_was_answered", {
          request: asks.request,
          answer: answer.act,
        })
        // **The reply is said, as the ask's own is** — an answer that could
        // not be taken (*Download not found.*, the
        // server not connected) is a `DidNotStart` and Rule 8 wants it said.
        // Until step 10c's twelfth-pass fix piece 2 the reply was dropped.
        .then((reply) => {
          theDownloadAnswered(reply);
          theScreenIsAskedFor();
        })
        .catch(() => {});
    });
    row.appendChild(control);
  }
  box.body.appendChild(row);
  box.raise();
}

function el(tag, ...classes) {
  const made = document.createElement(tag);
  if (classes.length) made.classList.add(...classes);
  return made;
}

// **What stands between a figure and the words after it, where the application
// marks them bound** — U+00A0, so *24 GB* or *2 jobs* never parts across two
// lines. `Design/Whole app § Typography`; the character's one home is
// `src/typography.rs` (`BETWEEN_A_FIGURE_AND_ITS_UNIT`), and this page writes
// it only where the mark says, never by reading which word is a unit.
const BETWEEN_A_FIGURE_AND_ITS_UNIT = "\u00A0";

/// **Words and values side by side, never joined** (X7).
function saidInto(holder, words, valueTag, valueClass, selectPaths) {
  if (!words || !words.pieces) return holder;
  let first = true;
  for (const piece of words.pieces) {
    if (!first && !piece.joins) {
      holder.appendChild(document.createTextNode(piece.binds ? BETWEEN_A_FIGURE_AND_ITS_UNIT : " "));
    }
    first = false;
    const isAValue = piece.value !== null && piece.value !== undefined;
    const node = el(isAValue ? valueTag || "span" : "span");
    if (isAValue && valueClass) node.classList.add(valueClass);
    if (isAValue && selectPaths && piece.is_path) node.classList.add("selectable");
    // **A value reads in the order its bytes run** — the folder an upload
    // goes into, a job's name, where downloads go — `Desktop app and delivery`
    // X9, drawn by `web/the-value-drawn.js` (step 10c's dependency wave piece
    // 10b).
    if (isAValue) theValueIsDrawnInto(node, piece.value);
    else node.textContent = piece.said || "";
    holder.appendChild(node);
  }
  return holder;
}

// ── The screen ─────────────────────────────────────────────────────────────

function theScreenIsBuilt(host) {
  const head = el("div", "qhead");
  const heading = el("h1");
  head.appendChild(heading);
  head.appendChild(el("span", "grow"));
  const speeds = el("span", "qsub", "mono");
  head.appendChild(speeds);
  const caps = el("span", "qsub");
  head.appendChild(caps);
  const grouper = el("span", "grouper");
  const label = el("label");
  label.htmlFor = "tfilter";
  const filter = el("select");
  filter.id = "tfilter";
  filter.addEventListener("change", () => {
    showing = filter.value;
    theScreenIsAskedFor();
  });
  grouper.appendChild(label);
  grouper.appendChild(filter);
  head.appendChild(grouper);
  host.appendChild(head);

  const answer = el("p", "rqs", "amber2", "transfers-answer");
  answer.hidden = true;
  host.appendChild(answer);

  const list = el("div", "qscroll");
  list.id = "tscroll";
  host.appendChild(list);

  const foot = el("div", "qfoot");
  host.appendChild(foot);

  host.addEventListener("keydown", (press) => {
    if (press.key === "Escape") theMenuGoes();
  });
  return { host, heading, speeds, caps, label, filter, answer, list, foot };
}

async function theScreenIsAskedFor() {
  if (!screen) return;
  const said = await shell.core.invoke("the_transfers_screen", { showing: showing || null });
  theScreenIsDrawn(said);
}

function theScreenIsDrawn(said) {
  screen.heading.textContent = said.heading;

  screen.speeds.replaceChildren();
  if (said.speeds) saidInto(screen.speeds, said.speeds);
  screen.speeds.hidden = !said.speeds;

  // **The caps' link is piece 11's call** — `Design/Transfers § What this
  // document left open`: the two speed caps sit in Settings, and this screen
  // carries the live reading and a link there.
  // **The caps Settings saved, as the application composed them** — the
  // figures and their words side by side (must-do 4).
  screen.caps.replaceChildren();
  saidInto(screen.caps, said.caps);
  screen.caps.appendChild(document.createTextNode(" · "));
  const link = el("button", "lnk");
  link.type = "button";
  link.textContent = said.settings;
  link.addEventListener("click", () => theSettingsSectionIsOpened(TRANSFERS_SECTION));
  screen.caps.appendChild(link);

  screen.label.textContent = said.show;
  screen.filter.replaceChildren();
  const every = el("option");
  every.value = "";
  every.textContent = said.every_server;
  screen.filter.appendChild(every);
  for (const server of said.servers) {
    const one = el("option");
    one.value = server.row;
    one.textContent = server.name;
    screen.filter.appendChild(one);
  }
  screen.filter.value = said.showing || "";
  // **Servers that would not read are said, in place of the filter and the
  // list** (must-do 14) — never an empty filter or *nothing is moving*.
  screen.filter.hidden = Boolean(said.could_not_read);
  screen.label.hidden = Boolean(said.could_not_read);

  screen.list.replaceChildren();
  for (const request of said.requests) {
    if (request.group_heading) {
      const heading = el("h2", "reqgroup");
      heading.textContent = request.group_heading;
      screen.list.appendChild(heading);
    }
    screen.list.appendChild(aRequest(request));
  }
  if (said.could_not_read) {
    screen.list.appendChild(
      theFailureShape({ sentence: said.could_not_read, kind: "ending", controls: [], room: EXPANDED }),
    );
  }
  if (said.nothing_here) {
    const none = el("p", "tnone");
    none.id = "tnone";
    saidInto(none, said.nothing_here);
    screen.list.appendChild(none);
  }

  screen.foot.replaceChildren();
  saidInto(screen.foot, said.foot);

  theRowsFigureIs(TRANSFERS, said.the_rails_figure || "");
}

// ── A request ──────────────────────────────────────────────────────────────

// Calendar fields are formatted data. All labels arrive from Rust.
function theStartTimeReads(at) {
  return at.toLocaleString("en-GB", {
    day: "numeric", month: "short", year: "numeric",
    hour: "2-digit", minute: "2-digit", second: "2-digit", hourCycle: "h23",
  });
}

function aRequest(request) {
  const box = el("div", "req");
  if (request.terminal) box.classList.add("done");
  box.dataset.request = request.id;

  const header = el("div", "rqh");
  const tag = el("span", "rqtag", request.tag_class);
  const arrow = el("i", "dir");
  arrow.textContent = request.arrow;
  tag.appendChild(arrow);
  tag.appendChild(document.createTextNode(request.tag));
  header.appendChild(tag);

  const name = el("span", "rqn");
  saidInto(name, request.name, "span", "mono");
  // **A dependency upload's *For* leads its file's name, in the secondary ink
  // at the ordinary weight** — the drawing's `span.rqlead`, then a space and
  // the name (`Design/Transfers`, the Creator's fourteenth notes, note 4).
  // The word is the application's; until then the two read as one.
  if (request.name_leads && name.children[0]) name.children[0].classList.add("rqlead");
  header.appendChild(name);
  header.appendChild(el("span", "grow"));
  const count = el("span", "rqm", "mono");
  // **The ending word in History's colours** (the Creator's seventh notes,
  // note 9): the class the application sends, or none where it stays grey.
  if (request.ending_class) count.classList.add(request.ending_class);
  saidInto(count, request.count);
  header.appendChild(count);

  const where = el(request.collapsed ? "span" : "p", "rqs");
  if (request.badge) where.appendChild(el("span", "badge", "sm", request.badge));
  saidInto(where, request.where_it_goes, "span", "mono", true);

  if (request.collapsed) header.appendChild(where);
  if (request.control) header.appendChild(theHeadersControl(request));
  box.appendChild(header);
  const started = el("time", "rqstarted");
  started.appendChild(document.createTextNode(request.started_label));
  if (request.started_at_millis !== null && request.started_at_millis !== undefined) {
    const at = new Date(Number(request.started_at_millis));
    started.dateTime = at.toISOString();
    const local = theStartTimeReads(at);
    started.appendChild(document.createTextNode(" " + local));
  }
  box.appendChild(started);

  if (request.bar) {
    const bar = el("progress", "underway-bar", "transfers-bar");
    bar.max = request.bar.max;
    bar.value = request.bar.value;
    box.appendChild(bar);
  }
  for (const line of request.lines.filter((one) => one.tone !== "amber")) {
    box.appendChild(aLine(request, line));
  }
  if (!request.collapsed) box.appendChild(where);
  for (const file of request.files) {
    for (const part of aFile(request, file)) box.appendChild(part);
  }
  for (const line of request.lines.filter((one) => one.tone === "amber")) {
    box.appendChild(aLine(request, line));
  }
  return box;
}

function theHeadersControl(request) {
  const control = el("button", "mini");
  control.type = "button";
  control.textContent = request.control.label;
  control.addEventListener("click", () => {
    const act = request.control.act;
    if (act === "stop") return theStopBoxIsOpened(request.id);
    // **The way back to the thing the transfer was about** — History, with
    // that job chosen by its identity (must-do 1).
    if (act === "open-the-job") return theHistoryEntryIsOpened(request.job || "");
    if (act === "open-the-check") {
      // **`Design/Transfers § The dependency check's own transfers`: the way
      // back is to that panel, and it starts nothing** — the rows as they
      // stand, or *nothing was read* with *Check again* where none is held
      // (`Dependency Upload & Remap` G17). Until the Creator's note 8 on the
      // test copy it started a whole new check, which drew an empty table
      // while the file was read again.
      theCurrentPlaceIs(A_SERVER, request.server);
      return theCheckIsShownOn(request.server, request.the_blend || "");
    }
    return theActIsTaken(act, request.id);
  });
  return control;
}

function aLine(request, line) {
  const p = el("p", "rqs");
  if (line.tone !== "plain") p.classList.add(line.tone);
  if (line.tone === "bad2") p.appendChild(el("span", "fdot"));
  const words = el(line.tone === "plain" ? "span" : "b");
  saidInto(words, line.words, "span", "mono");
  p.appendChild(words);
  if (line.due_at) {
    p.appendChild(document.createTextNode(" "));
    p.appendChild(
      theCountdown(line.due_at, { in: line.trying_again_in, now: line.trying_again_now }),
    );
  }
  if (line.link) {
    p.appendChild(document.createTextNode(" "));
    const link = el("button", "lnk");
    link.type = "button";
    link.textContent = line.link.label;
    link.addEventListener("click", () =>
      line.link.act.startsWith(WHICH_ONES)
        ? theNamesAreShownUnder(p, request.id, line.link.act)
        : theActIsTaken(line.link.act, request.id),
    );
    p.appendChild(link);
  }
  if (line.cross) {
    const cross = el("button", "fx");
    cross.type = "button";
    cross.textContent = "×";
    cross.setAttribute("aria-label", line.cross);
    cross.addEventListener("click", () => p.remove());
    p.appendChild(cross);
  }
  return p;
}

// **An amber line's *Which ones*: the names one click away** —
// `Design/Transfers § When a download meets trouble`, *"the count on the row,
// the names one click away"*. The application names them from the request's own
// records (`the_files_a_line_names`), each a value drawn as its bytes run; a
// second press puts them away. The 38's piece 9, item 32.
const WHICH_ONES = "which-ones";

async function theNamesAreShownUnder(line, request, which) {
  const shown = line.nextElementSibling;
  if (shown && shown.classList.contains("transfers-names")) {
    shown.remove();
    return;
  }
  const names = await shell.core.invoke("the_files_a_line_names", { request, which });
  const list = el("div", "clist", "mono", "transfers-names");
  for (const name of names || []) {
    const one = el("div");
    theValueIsDrawnInto(one, name);
    list.appendChild(one);
  }
  line.after(list);
}

// ── A file ─────────────────────────────────────────────────────────────────

function aFile(request, file) {
  const parts = [];
  const row = el("div", "tf");
  if (file.state) row.classList.add(file.state);
  const name = el("span", "tn", "mono");
  // The file's path, in the order its bytes run (X9, `web/the-value-drawn.js`).
  theValueIsDrawnInto(name, file.path);
  row.appendChild(name);

  const bar = el("span", "tb");
  if (file.staged && file.expected) {
    bar.appendChild(
      theMarkOnARow({ staged: file.staged, expected: file.expected }, { of: file.of }),
    );
  }
  row.appendChild(bar);
  const tail = el("span", "tm");
  if (file.tail) tail.textContent = file.tail;
  row.appendChild(tail);
  parts.push(row);

  // **`Design/Transfers § Stopping one file`: the control is in the row's own
  // right-click menu, never on the row**, and a file with nothing left to stop
  // opens no menu at all.
  if (file.stop) {
    row.addEventListener("contextmenu", (press) => {
      press.preventDefault();
      theMenuIsShown(row, file.stop, request.id, file.path);
    });
  }

  if (file.note) {
    const note = el("p", "tfchg");
    const words = el("b");
    saidInto(words, file.note.words, "span", "mono");
    note.appendChild(words);
    if (file.note.link) {
      note.appendChild(document.createTextNode(" "));
      const link = el("button", "lnk");
      link.type = "button";
      link.textContent = file.note.link.label;
      link.addEventListener("click", () =>
        theActOnAFileIsTaken(file.note.link.act, request.id, file.path),
      );
      note.appendChild(link);
    }
    parts.push(note);
  }

  if (file.failure) {
    const holder = el("div", "tffail");
    const cross = file.failure.controls.find((one) => one.is === "cross");
    holder.appendChild(
      theFailureShape({
        sentence: file.failure.sentence,
        kind: "condition",
        identityOrMark: `${request.id}-${file.path}`,
        controls: file.failure.controls.map((one) => ({ is: one.is, label: one.label, act: one.act })),
        room: A_COLLAPSED_ROW,
        // **The file goes with the act**, so a failed download's *Details*
        // answers for this file and no other (`Design/Transfers § A file that
        // failed`, E2-1; step 10c's thirteenth-pass fix piece 3).
        onAct: (act) => theActOnAFileIsTaken(act, request.id, file.path),
        onClear: () => {
          if (cross) holder.remove();
        },
      }),
    );
    parts.push(holder);
  }
  return parts;
}

// **The one file's Stop names the file** — its landing path, as the row
// shows it — so the application stops that file and no other
// (`Design/Transfers § Stopping one file`; step 10c's transfer wave piece 7).
function theMenuIsShown(row, label, request, path) {
  theMenuGoes();
  const menu = el("div", "ctx", "transfers-ctx");
  const stop = el("button");
  stop.type = "button";
  stop.textContent = label;
  stop.addEventListener("click", () => {
    theMenuGoes();
    theActIsTaken("stop-one-file", request, path);
  });
  menu.appendChild(stop);
  row.after(menu);
  theOpenMenu = menu;
  stop.focus();
}

function theMenuGoes() {
  if (theOpenMenu) theOpenMenu.remove();
  theOpenMenu = null;
}

document.addEventListener("click", (press) => {
  if (theOpenMenu && !theOpenMenu.contains(press.target)) theMenuGoes();
});

// ── Acts and their answers ─────────────────────────────────────────────────

function theActIsTaken(act, request, file) {
  return shell.core.invoke("an_act_on_transfers", { act, request, file: file || null });
}

// **The two acts that send one file again** — a failed file's *Try again* and
// a changed file's *Upload it again* (`Design/Transfers § A file that failed`
// and `§ A file that changed while it was going up`). Each is a new request for
// that one file, answered the way a download asked for here is answered, so a
// download's own question is put here too. The 38's piece 9, item 17.
const THE_ACTS_THAT_SEND_A_FILE_AGAIN = ["try-again", "upload-it-again"];

function theActOnAFileIsTaken(act, request, file) {
  if (!THE_ACTS_THAT_SEND_A_FILE_AGAIN.includes(act)) return theActIsTaken(act, request, file);
  return shell.core
    .invoke("a_file_is_sent_again", { request, file: file || "" })
    .then((answer) => {
      theDownloadAnswered(answer);
      theScreenIsAskedFor();
    })
    .catch(() => {});
}

// **A transfer moved and it is written down** — a unit ended, or an
// auto-download's whole grew — so the rows are asked for again while the files
// land (`Download` M16; `Design/Whole app § Anything that isn't instant`: a
// transfer is *on the row* and says how far along it is). **The event carries
// nothing and the page asks** (P8). Until 2026-09-27 nothing told this page,
// and a row stood at *0 / 3* with every file on disk until the page was left and
// opened again.
//
// **Since 2026-09-29 it is also posted while bytes move inside a file**, no
// more than four times a second over every transfer together, and when a
// request's run ends — step 10c's transfer wave piece 5. **Nothing here
// changed for it**: the one ask at a time below is what keeps a burst of them
// to one ask in flight and at most one owed.
whenTold(TRANSFERS_MOVED, () => {
  if (!screen || !itIsShowing) return;
  if (asking) {
    askAgain = true;
    return;
  }
  asking = theScreenIsAskedFor()
    .catch(() => {})
    .finally(() => {
      asking = null;
      if (askAgain && itIsShowing) {
        askAgain = false;
        theScreenIsAskedFor().catch(() => {});
      }
    });
});

whenTold(AN_ACT_ON_TRANSFERS, (delivery) => {
  const said = delivery.said || {};
  if (said.request === THE_FORMAT_BOX && theOpenFormatBox) {
    theOpenFormatBox.answer.textContent = said.said || "";
    theOpenFormatBox.answer.hidden = !said.said;
    return;
  }
  if (!screen) return;
  screen.answer.textContent = said.said || "";
  screen.answer.hidden = !said.said;
});

// ── A box ──────────────────────────────────────────────────────────────────

function aBox(onTop) {
  const wrap = el("div", "modal");
  if (onTop) wrap.classList.add("top");
  wrap.setAttribute("role", "dialog");
  wrap.setAttribute("aria-modal", "true");
  const body = el("div", "mbox");
  wrap.appendChild(body);
  const close = () => wrap.remove();
  // **Escape closes the thing on top, one layer at a time.**
  wrap.addEventListener("keydown", (press) => {
    if (press.key !== "Escape") return;
    press.preventDefault();
    close();
  });
  return {
    body,
    close,
    raise() {
      // **The drawing's `.modal` fills the positioned box it sits in**, so the
      // stop box sits inside this screen's own host, as the servers screen's
      // dialogs sit inside theirs. The format boxes are the drawing's `top`
      // layer and are opened from other screens, so they sit over the page.
      (onTop || !screen ? document.body : screen.host).appendChild(wrap);
      const first = body.querySelector("button");
      if (first) first.focus();
    },
  };
}

function aRowOfAnswers(answers) {
  const row = el("div", "mrow");
  row.appendChild(el("span", "grow"));
  for (const answer of answers) {
    const control = el("button", "btn");
    if (answer.quiet) control.classList.add("quiet");
    control.type = "button";
    control.textContent = answer.label;
    control.addEventListener("click", answer.act);
    row.appendChild(control);
  }
  return row;
}

// ── The stop box — `Design/Transfers § What Stop says before it does it` ──

async function theStopBoxIsOpened(request) {
  const said = await shell.core.invoke("the_stop_box", { request });
  if (said.is !== "the-box") return;
  const box = aBox(false);
  const heading = el("h3");
  heading.textContent = said.heading;
  box.body.appendChild(heading);
  const list = el("ul", "bulive");
  for (const bullet of said.bullets) {
    const item = el("li");
    const bold = el("b");
    if (bullet.a_name) bold.classList.add("mono");
    saidInto(bold, bullet.bold, "span");
    item.appendChild(bold);
    item.appendChild(document.createTextNode(" "));
    const rest = el("span", "stm");
    rest.textContent = bullet.rest;
    item.appendChild(rest);
    list.appendChild(item);
  }
  box.body.appendChild(list);
  if (said.nothing_is_thrown_away) {
    const none = el("p", "msub");
    none.textContent = said.nothing_is_thrown_away;
    box.body.appendChild(none);
  }
  box.body.appendChild(
    aRowOfAnswers([
      { label: said.keep, quiet: true, act: () => box.close() },
      {
        label: said.stop_it,
        act: () => {
          box.close();
          theActIsTaken("stop-the-request", request);
        },
      },
    ]),
  );
  box.raise();
}

// ── The three format boxes ─────────────────────────────────────────────────

/// **Open the box that asks what a job's outputs download as.** Exported for
/// the render settings panel's auto-download switch and a queue entry's
/// narrowing control.
///
/// **Its *Done* writes the choice** (`Download` R1, `Download` R11, `Download`
/// N3 on the queue entry):
/// the box stays up until the application answers, and closes where the choice
/// was taken, or says why it was not. The 38's piece 9: until then the box said
/// it was not finished.
export async function theFormatBoxIsOpenedFor(job) {
  const said = await shell.core.invoke("the_format_box", { job });
  theFormatBoxIsDrawn(said, (chosen, close) =>
    shell.core
      .invoke("the_formats_are_chosen_for_a_job", {
        job,
        formats: chosen.formats,
        folder: chosen.folder,
      })
      .then((answer) => {
        if (!answer) {
          close();
          return;
        }
        if (theOpenFormatBox) {
          theOpenFormatBox.answer.textContent = answer;
          theOpenFormatBox.answer.hidden = false;
        }
      })
      .catch(() => {}),
  );
}

/// **The same box, for a file whose job is not submitted yet** —
/// `Design/Render Settings § How the three sit together`. `whenDone` is handed
/// the format names left ticked and the folder left chosen, and closing the box
/// is this module's.
///
/// `whenReadItNow` is the panel's own read, so *Read it now* reads the file the
/// panel is over rather than asking Transfers for an arrangement. **Where the
/// application says the read is not offered, *Read it now* reads unavailable**
/// — `Render Settings` B5a, and `Design/Render Settings`: *"Not offered means
/// the control stays where it was and reads unavailable — Render and the read
/// alike."* Step 10c's eleventh pass, fault 8: with no Blender picked it was
/// live, and a press closed the box and came to nothing.
///
/// `whenSomewhereElse` is the panel's own asking: **choosing *Somewhere else…*
/// puts the system's folder panel up at once** (`Design/Render Settings`,
/// added 2026-09-30 from the Creator's press on the eleventh pass). It answers
/// the box composed again with the chosen folder offered first, or nothing
/// where the panel was closed — and then the box is left as it was.
export async function theFormatBoxIsOpenedForAFile(server, file, whenDone, whenReadItNow, whenSomewhereElse) {
  const said = await shell.core.invoke("the_download_panel", { server, file });
  theFormatBoxIsDrawn(said, whenDone, true, whenReadItNow, whenSomewhereElse);
}

function theFormatBoxIsDrawn(said, whenDone, closesOnDone, whenReadItNow, whenSomewhereElse) {
  const box = aBox(true);
  const answer = el("p", "msub", "transfers-answer");
  answer.hidden = true;
  theOpenFormatBox = { answer };

  if (said.is !== "the-box") {
    const line = el("p", "mwarn");
    line.textContent = said.said;
    box.body.appendChild(line);
    box.body.appendChild(aRowOfAnswers([{ label: said.close, act: () => box.close() }]));
    box.raise();
    return;
  }

  const heading = el("h3");
  heading.textContent = said.heading;
  box.body.appendChild(heading);

  // **`Download` R11's telling, before a switch-on moves the folder** — what
  // is already downloaded stays where it is (the 38's piece 9).
  if (said.before_it_moves) {
    const before = el("p", "mwarn");
    saidInto(before, said.before_it_moves, "span", "mono");
    box.body.appendChild(before);
  }

  const reading = said.reading;
  if (reading.reads === "several-formats") {
    const intro = el("p", "msub");
    intro.textContent = reading.intro;
    box.body.appendChild(intro);
    const formats = el("div", "fmts", "transfers-formats");
    for (const format of reading.formats) {
      const one = el("label", "fmt");
      // **The identifier goes back; Smoozy's word is what is read** (Rule 9,
      // the Creator's note 16 on their second notes).
      one.dataset.format = format.name;
      const tick = el("span", "cb");
      const setIt = (on) => {
        one.classList.toggle("on", on);
        tick.classList.toggle("on", on);
        tick.textContent = on ? "✓" : "";
      };
      setIt(format.chosen);
      one.addEventListener("click", () => setIt(!one.classList.contains("on")));
      one.appendChild(tick);
      const fmtName = el("span", "fmn");
      fmtName.textContent = format.said;
      one.appendChild(fmtName);
      // Where in the job the format is written: *top of the job's folder* in
      // words, and each folder under it as a value, in the order its bytes run
      // (X9, `web/the-value-drawn.js`).
      let fmtWhere;
      if (format.the_top) {
        fmtWhere = el("span", "fmw");
        fmtWhere.appendChild(document.createTextNode(format.the_top));
        if (format.where_in_the_job) {
          fmtWhere.appendChild(document.createTextNode(" · "));
          const folders = el("span", "mono");
          theValueIsDrawnInto(folders, format.where_in_the_job);
          fmtWhere.appendChild(folders);
        }
      } else {
        fmtWhere = el("span", "fmw", "mono");
        theValueIsDrawnInto(fmtWhere, format.where_in_the_job);
      }
      one.appendChild(fmtWhere);
      const fmtCount = el("span", "fmc", "mono");
      saidInto(fmtCount, format.files);
      one.appendChild(fmtCount);
      formats.appendChild(one);
    }
    box.body.appendChild(formats);
    const total = el("p", "fmtot");
    const bold = el("b");
    saidInto(bold, reading.total);
    total.appendChild(bold);
    total.appendChild(document.createTextNode(" "));
    const rest = el("span");
    rest.textContent = reading.the_rest;
    total.appendChild(rest);
    box.body.appendChild(total);
  } else if (reading.reads === "one-format") {
    const line = el("p", "msub");
    saidInto(line, reading.said, "b");
    box.body.appendChild(line);
  } else if (reading.reads === "read-for-another-scene") {
    // **Read, but not for the scene ticked** (the drawing's `otherscene`), or
    // a typed frame outside the read: no *Read it now*, because a read would
    // cover the same scene again. **What downloads is words and values** —
    // the narrowing that stands, each format a value, where one does (the box ruling of 2026-10-17).
    const warn = el("p", "mwarn");
    const bold = el("b");
    bold.textContent = reading.warning;
    warn.appendChild(bold);
    warn.appendChild(document.createTextNode(" "));
    warn.appendChild(saidInto(el("span"), reading.what_downloads));
    box.body.appendChild(warn);
  } else {
    const warn = el("p", "mwarn");
    const bold = el("b");
    bold.textContent = reading.warning;
    warn.appendChild(bold);
    warn.appendChild(document.createTextNode(" "));
    warn.appendChild(saidInto(el("span"), reading.what_downloads));
    warn.appendChild(document.createTextNode(" "));
    const readItNow = el("button", "lnk");
    readItNow.type = "button";
    readItNow.textContent = reading.read_it_now;
    // **`Render Settings` B5a**: the application says whether a file's read is
    // offered; the queue's box carries no such answer and is left as it was.
    if (said.the_read_is_offered === false) readItNow.disabled = true;
    readItNow.addEventListener("click", () => {
      if (!whenReadItNow) {
        theActIsTaken("choose-the-formats", THE_FORMAT_BOX);
        return;
      }
      box.close();
      whenReadItNow();
    });
    warn.appendChild(readItNow);
    warn.appendChild(document.createTextNode(" "));
    const toChoose = el("span");
    toChoose.textContent = reading.to_choose;
    warn.appendChild(toChoose);
    box.body.appendChild(warn);
  }

  // **Where the folder is fixed, it is one plain line, *To ~/Renders/kitchen*,
  // and no choice** (the drawing's `fmt-q`; the Creator's note 16 on their
  // second notes: the one fixed folder was drawn as a radio button).
  if (said.the_fixed_folder) {
    const line = el("p", "msub", "transfers-fixed-folder");
    saidInto(line, said.the_fixed_folder, "span", "mono");
    box.body.appendChild(line);
  }

  // **Where they go**: the standing folder, §8's proposal named as the user's
  // download folder, and somewhere else. Each is a choice row so a folder's
  // path and the words about it stay two nodes (X7).
  const to = el("div", "frm");
  const toLabel = el("label");
  toLabel.textContent = said.to;
  to.appendChild(toLabel);
  const folders = el("div", "fmts", "transfers-folders");
  folders.setAttribute("role", "radiogroup");
  folders.setAttribute("aria-label", said.where_downloads_go);
  const choices = [];
  const choose = (picked) => {
    for (const one of choices) {
      const on = one === picked;
      one.classList.toggle("on", on);
      one.firstChild.classList.toggle("on", on);
      one.setAttribute("aria-checked", String(on));
    }
  };
  // **`at` is which row it is, and it is the whole of what the application
  // gets back** — must-do 18 of this round: a folder's text is for showing and
  // the folder itself never leaves the application side, so a name that is not
  // valid text cannot come back as something else. ~~The row used to carry the
  // folder's displayed text and hand that back.~~
  const aFolderChoice = (fill, at, whenChosen) => {
    const one = el("label", "fmt", "choice3");
    if (at !== undefined) one.dataset.at = String(at);
    one.setAttribute("role", "radio");
    one.appendChild(el("span", "rb"));
    fill(one);
    one.addEventListener("click", () => (whenChosen ? whenChosen(one) : choose(one)));
    choices.push(one);
    folders.appendChild(one);
    return one;
  };
  // **The folders the box offers, drawn again where the system's panel gave
  // one**: the application composes the box again with that folder first, and
  // the first row is the choice — as it is when the box opens.
  const theFoldersAreDrawn = (offered) => {
    for (const one of choices.splice(0)) one.remove();
    offered.forEach((folder, at) => {
      aFolderChoice((one) => {
        const words = el("span", "fmn");
        saidInto(words, folder.said, "span", "mono");
        one.appendChild(words);
      }, at);
    });
    // **The panel goes up the moment the row is chosen**, and nothing is kept
    // until *Done*. **Closed with nothing chosen, the box is as it was**: the
    // row chosen before is chosen again.
    const theSystemIsAsked = whenSomewhereElse
      ? (elsewhere) => {
          const before = choices.find((one) => one.classList.contains("on"));
          choose(elsewhere);
          Promise.resolve(whenSomewhereElse())
            .then((again) => {
              if (again && again.is === "the-box") theFoldersAreDrawn(again.folders);
              else if (before) choose(before);
            })
            .catch(() => {
              if (before) choose(before);
            });
        }
      : undefined;
    // **A folder fixed when the arrangement was switched on is the one row**,
    // with no other offered (`Download` R1; the 38's piece 9).
    if (!said.the_folder_is_fixed) {
      aFolderChoice(
        (one) => {
          const words = el("span", "fmn");
          words.textContent = said.somewhere_else;
          one.appendChild(words);
        },
        undefined,
        theSystemIsAsked,
      );
    }
    choose(choices[0]);
  };
  theFoldersAreDrawn(said.folders);
  to.appendChild(folders);
  if (!said.the_fixed_folder) box.body.appendChild(to);

  if (said.fixed) {
    const fixed = el("p", "msub");
    saidInto(fixed, said.fixed);
    box.body.appendChild(fixed);
  }
  box.body.appendChild(answer);
  box.body.appendChild(
    aRowOfAnswers([
      { label: said.cancel, quiet: true, act: () => box.close() },
      {
        label: said.done,
        act: () => {
          // **What is left ticked and chosen, read off the rows themselves** —
          // the format's own name, which the application handed over and
          // checks again, and **which row the folder was**, never its text.
          const ticked = [...box.body.querySelectorAll(".fmt")]
            .filter((one) => one.classList.contains("on") && one.dataset.format !== undefined)
            .map((one) => one.dataset.format);
          const picked = choices.find((one) => one.classList.contains("on"));
          const folder =
            picked && picked.dataset.at !== undefined
              ? { is: "this-one", at: Number(picked.dataset.at) }
              : { is: "somewhere-else" };
          if (closesOnDone) box.close();
          whenDone({ formats: ticked, folder }, () => box.close());
        },
      },
    ]),
  );
  box.raise();
}

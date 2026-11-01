// **The History screen** — Smoozy's own record of every job that ever started
// rendering, beside the entry of the one that is chosen. The application half
// is `src/surface/the_history.rs`; read its head first.
//
// **It is a second screen and not a tab of the queue**, and Design gives the
// reason: they empty differently.
//
// ## What it takes from the queue's module, and why
//
// **The entry is one shape and two screens fill it.** `web/the-queue.js`
// exports the builders and this file hands them its own three groups — the
// failure shape at the top, the two ways to render it again, and the times —
// through the `extra` hooks. **Nothing about the entry is copied here**, and a
// control that cannot act yet answers through the queue module's
// `anActIsAskedFor`, as every control on the queue does.
//
// ## An entry is chosen by its job, never by its place
//
// **The page names the job's identity when it asks** — a row's, the delete
// key's and the header's *Delete* alike. A place in the list moves when a job
// on another server starts rendering, and a box composed over a place could
// open over a different job (step 10a round 1 of the queue, history, settings
// and downloads, must-do 1). **This file composes nothing**: every figure it
// draws is one the application sent, and it adds nothing to any of them.
//
// H32, H33, P9 and P10 bind here exactly as they bind in the queue's module.

import { whenTold } from "./event-route.js";
// **The moment a row ends with is written the way every other screen writes
// one** — the application supplies the moment in seconds and this side
// supplies the choice of shape, from the system's own locale data. There is
// one of these in the page and it is `web/servers.js`'s.
import { theMomentReads } from "./servers.js";
import {
  EXPANDED,
  theControlsFrom,
  theFailureShape,
} from "./the-failure-shape.js";
import { theWordsPanel } from "./the-machines-own-words.js";
import { theTypingIsTakenFrom, theTypingIsPutBackInto } from "./the-typed-text.js";
import { theSeamsAreLive } from "./panes.js";
import { theValueDrawn, theValueIsDrawnInto } from "./the-value-drawn.js";
import {
  aScreenIsRegistered,
  theCurrentPlace,
  theCurrentPlaceIs,
  theRowsFigureIs,
  THE_HISTORY,
} from "./the-rail.js";
import {
  aFigureAndItsWord,
  anActIsAskedFor,
  aPart,
  howMany,
  theBoxStaysOrIsDrawn,
  theEntryIsDrawn,
  theKeptWordsAreOpened,
  thePairIsDrawn,
  thePreviewRefreshStarts,
  thePreviewGoesAway,
  theJobsIn,
} from "./the-queue.js";

// **What stands between a figure and the words that count it** — U+00A0, so
// *240 frames* never parts across two lines. `Design/Whole app § Typography`; the
// character's one home is `src/typography.rs` (`BETWEEN_A_FIGURE_AND_ITS_UNIT`).
const BETWEEN_A_FIGURE_AND_ITS_UNIT = "\u00A0";

const shell = window.__TAURI__;

// Declared beside the service that posts it — `THE_HISTORY_SAYS` in
// `src/surface/the_history.rs`.
const THE_HISTORY_SAYS = "history:the-history";


let host = null;
let list = null;
let scroll = null;
let detail = null;

// The last reading and the group holding the two submit controls.
let theLastSaid = null;
let theAgainGroup = null;

function aButton(label, ...classes) {
  const control = document.createElement("button");
  control.type = "button";
  for (const one of classes) control.classList.add(one);
  if (label) control.textContent = label;
  return control;
}

function aBadge(into, hue) {
  if (!hue) return;
  const badge = document.createElement("span");
  badge.classList.add("badge");
  badge.classList.add("sm");
  badge.classList.add(hue);
  into.appendChild(badge);
}

aScreenIsRegistered({
  place: THE_HISTORY,
  itIsDrawn(where) {
    host = where;
    const split = document.createElement("div");
    split.className = "qsplit";

    // **One width for both screens** — the list's seam is one of the three the
    // pane-width record already holds, and a user who sets it on the queue
    // means it on the history. It drags through the panes' own service, as the
    // queue's does.
    const seam = aButton(null, "seam");
    seam.dataset.seam = "list";
    split.appendChild(seam);
    shell.core
      .invoke("the_pane_layout")
      .then((layout) => {
        const declared = layout.seams.find((one) => one.key === "list");
        if (!declared) return;
        seam.setAttribute("aria-label", declared.label);
        seam.title = declared.hint;
        theSeamsAreLive(split, layout.seams);
      })
      .catch(() => {});

    list = document.createElement("div");
    list.className = "qlist";
    split.appendChild(list);

    detail = document.createElement("div");
    detail.className = "qdetail";
    split.appendChild(detail);

    where.appendChild(split);

    // **`Design/Queue & History § Keyboard shortcuts`: *"Return opens the
    // chosen entry. The delete key deletes a history entry, asking as the
    // control does."*** A row is a button, so Return on it is its press, and
    // the press chooses it. The delete key on a row asks about that row's
    // job; a key pressed inside a box is the box's.
    split.addEventListener("keydown", (press) => {
      if (press.defaultPrevented) return;
      if (press.key !== "Delete" && press.key !== "Backspace") return;
      const row = press.target.closest && press.target.closest(".hrow[data-job]");
      if (!row) return;
      press.preventDefault();
      theHistoryIsAskedFor(row.dataset.job, DELETE_IT);
    });

    theHistoryIsAskedFor(theChosenJob);
  },
  itIsCurrentNow(which) {
    // **A job handed over is the one chosen**, as Render Settings' *Show in
    // Queue* hands the queue its job. Since the catch-up banner went
    // (2026-10-16) no place hands History a job this way: Transfers' *Open the
    // job* chooses it first, through `theHistoryEntryIsOpened` below, and
    // the Queue heading's count opens History on whatever was chosen.
    if (which) theChosenJob = which;
    if (host) theHistoryIsAskedFor(theChosenJob);
  },
  itIsPutAway() { thePreviewGoesAway("history"); },
});

// **The job whose entry the user chose, by its identity**, held for the run and
// nothing durable, and handed back each time the history is asked for.
let theChosenJob = null;
// The boxes this screen asks for — `TheHistoryAsk` in
// `src/surface/the_history.rs`.
const DELETE_IT = "delete";
const CLEAR_THIS_SERVER = "clear-this-server";
const CLEAR_ALL = "clear-all";
// **`Queue & History` X6b's box**, asked from the *Removed servers* heading.
const CLEAR_REMOVED = "clear-removed-servers";

// **Whether the *Removed servers* group is open** — `Design/Queue & History §
// History`: *"Its fold is kept until Smoozy closes and is open at each
// launch"*. Held by this page for the run and written nowhere; History has no
// other fold. The eighteenth notes, note 12.
let theRemovedServersAreOpen = true;
// **The entry's *Blender output*** — `Queue & History` E9's kept words, which
// the answer carries with the history when this is asked. The Creator's notes
// on 0.1.8, piece 6d.
const BLENDER_OUTPUT = "blender-output";
// **The *Blender output* in a first-frame pair's box: the check's own kept
// words** — `TheHistoryAsk::TheChecksBlenderOutput`. The nineteenth notes,
// note 7.
const THE_CHECKS_BLENDER_OUTPUT = "the-checks-blender-output";

function theHistoryIsAskedFor(chosen, asking) {
  if (chosen !== undefined && chosen !== null) theChosenJob = chosen;
  shell.core.invoke("the_history", { chosen: theChosenJob, asking: asking || null });
}

/// **Open History on one job's entry** — Transfers' *Open the job*, which
/// `Design/Transfers` calls the way back to the thing the transfer was about.
/// The job is named by its identity.
export function theHistoryEntryIsOpened(job) {
  if (job) theChosenJob = job;
  if (theCurrentPlace() === THE_HISTORY) theHistoryIsAskedFor(theChosenJob);
  else theCurrentPlaceIs(THE_HISTORY);
}


whenTold(THE_HISTORY_SAYS, (delivery) => {
  if (!host) return;
  const said = delivery.said || {};
  theHistoryIsDrawn(said);
  theKeptWordsAreOpened(host, said);
});

/// **History's own *Blender output*, on a clean finish whose words are kept**
/// — where the drawing puts it, under the status line and above the rest of
/// the entry. **The control and nothing beside it**: no line of either stream
/// is lifted out (M13b), as the drawing now draws it too. **And no heading
/// over it**: the control's own words already say it (the Creator's third
/// notes, note 15: "they are just duplicate information").
function theKeptWordsControlIsDrawn(main, said) {
  if (!said.historyBlenderOutput) return;
  const group = document.createElement("div");
  group.className = "grp";
  // **The control alone, never in an empty grey box** — the drawing's
  // `.log.nolines` (the Creator's note 12 on their second notes, 2026-10-04:
  // History drew the empty window, with a spacer in it).
  const log = document.createElement("div");
  log.className = "log nolines";
  const open = aButton(said.historyBlenderOutput);
  open.addEventListener("click", () => theHistoryIsAskedFor(said.entryJob, BLENDER_OUTPUT));
  log.appendChild(open);
  group.appendChild(log);
  main.appendChild(group);
}

function theHistoryIsDrawn(said) {
  const theKeyboardWasOnARow = !!(
    document.activeElement &&
    document.activeElement.classList &&
    document.activeElement.classList.contains("hrow") &&
    list.contains(document.activeElement)
  );
  // The job whose entry was on screen before this drawing — see the carry-on
  // frame's typing, below.
  const theJobThatWasShown = theLastSaid ? theLastSaid.entryJob : null;
  theLastSaid = said;
  // **The job the application says is chosen is the job this page holds.**
  //
  // **Without this line the header's *Delete* and the delete key do nothing at
  // all until a row has been clicked**, and nothing says why. The page opens
  // holding no job, asks with `chosen: null`, and the application falls back to
  // the first row and answers with that job's identity (`the_row_is_chosen`);
  // this side never took the answer, so the next ask still carried no job and
  // `the_history_as_asked` — which composes a box only over the job it was
  // named (must-do 1) — composed none. **Found by pressing the control in the
  // running application** and by nothing else: every test of the box passes,
  // because a test hands it the identity.
  //
  // **It also keeps the two in step when a job leaves history**, which is the
  // other half of must-do 1: the application falls back to a job that is still
  // there and the page follows it rather than holding an identity that is gone.
  if (said.hrowChosen) theChosenJob = said.hrowChosen;
  // **Text being typed is not lost to a redraw** — `Design/Whole app § The
  // window`. The carry-on frame is a field, and History is drawn again whole
  // whenever the application tells it anything, such as another job's ending,
  // so what is typed there and the caret are taken first
  // (`web/the-typed-text.js`) — and put back only into the same job's entry.
  const theTyping = theAgainGroup ? theTypingIsTakenFrom(theAgainGroup) : null;
  theAgainGroup = null;
  thePreviewRefreshStarts(host);
  list.replaceChildren();
  detail.replaceChildren();
  // **An open box is not taken away by the redraw** — step 10c's eleventh-pass
  // fix piece 5: it stays until the user answers or closes it, or its job
  // leaves History ([`theBoxStaysOrIsDrawn`], below).

  list.appendChild(theHead(said));
  scroll = document.createElement("div");
  scroll.className = "qscroll";
  // **The list and the entry keep their place across a redraw** —
  // `Design/Whole app § The window`, kept by `web/the-scrolled-place.js`.
  // History is drawn anew whenever the application tells it anything, and
  // both went to their top each time (the Creator's second notes on the test
  // copy, piece 5). The entry is named in `theEntryIsDrawn`, for this screen.
  scroll.dataset.place = "history-list";
  list.appendChild(scroll);
  // **A read that failed is said in place of the list** — step 10b round 1 of
  // *jobs, history and downloads*, must-do 3(a) — through the one failure
  // shape, and never drawn as the empty History a person who never rendered
  // sees. The application sends nothing else with it.
  if (said.couldNotRead) {
    scroll.appendChild(
      theFailureShape({ sentence: said.couldNotRead, kind: "ending", controls: [], room: EXPANDED }),
    );
    theRowsFigureIs(THE_HISTORY, "");
    return;
  }
  theRowsAreDrawn(said);
  // **Rows that would not read back are counted and said beside the list**
  // (must-do 3(c)): the figure the application sent, then its words.
  if (said.unreadFigure) {
    const unread = document.createElement("p");
    unread.className = "hunread";
    aFigureAndItsWord(unread, said.unreadFigure, said.unreadWord);
    list.appendChild(unread);
  }

  theEntryIsDrawn(detail, said, {
    screen: "history",
    // **The check's own words, from a pair's box** (the nineteenth notes,
    // note 7).
    theChecksWords() {
      theHistoryIsAskedFor(theChosenJob, THE_CHECKS_BLENDER_OUTPUT);
    },
    headerActs(header) {
      // **`Design/Queue & History § History`, E1-1: an entry still in progress
      // carries no *Delete* until it has an ending**, and the application
      // sends the control's word only where the entry may be deleted.
      if (said.delete) {
        const remove = aButton(said.delete, "mini", "danger");
        remove.addEventListener("click", () => theHistoryIsAskedFor(theChosenJob, DELETE_IT));
        header.appendChild(remove);
      }
    },
    afterTheStatus(main) {
      // **Design: the failure shape sits at the top of the entry, above the
      // readings and the marks and above the setup.** The user opened this
      // entry because the row said *Failed*, so what did not happen is the
      // first thing to answer — under the status line, which is where the
      // drawing puts it. **A clean finish's *Blender output* sits in the same
      // place** (piece 6d).
      theKeptWordsControlIsDrawn(main, said);
      theFailureIsDrawn(main, said);
    },
    afterTheReadings(main) {
      theAgainGroup = theWaysToRenderItAgain(said);
      if (theAgainGroup) main.appendChild(theAgainGroup);
      theRenderedGroup(main, said);
    },
  });
  theBoxStaysOrIsDrawn(host, said, theJobsIn(said, "hrowCount", "hrow"), new Set());

  // The keyboard stays on the chosen row across the redraw.
  const current = scroll.querySelector(".hrow[aria-current=\"true\"]");
  if (theKeyboardWasOnARow && current) current.focus();

  // **The carry-on frame being typed comes back with its caret**, into the same
  // job's entry and no other's: another job's entry is that job's own answer.
  if (theTyping && theAgainGroup && said.entryJob === theJobThatWasShown) {
    theTypingIsPutBackInto(theAgainGroup.querySelector("input.fld"), theTyping);
  }

  theRowsFigureIs(THE_HISTORY, "");
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
  // **Where the read failed there is no count to give**, and *kept until you
  // clear them* over a History nobody could read would say the list is empty.
  if (!said.couldNotRead) {
    const sub = document.createElement("span");
    sub.className = "qsub";
    aFigureAndItsWord(sub, said.theJobsFigure, said.theJobsWord);
    sub.appendChild(document.createTextNode(" · "));
    const kept = document.createElement("span");
    kept.textContent = said.keptUntilYouClearThem || "";
    sub.appendChild(kept);
    head.appendChild(sub);
  }

  // **Design: both sit on the history list's own heading, beside the count they
  // change.** They act on the list and not on one job. **With no entries the
  // application sends neither, and neither is drawn** (must-do 5(e)).
  const clearing = document.createElement("span");
  clearing.className = "hclear";
  if (said.clearThisServersHistory) {
    const clearThis = aButton(said.clearThisServersHistory, "mini");
    clearThis.addEventListener("click", () => theHistoryIsAskedFor(theChosenJob, CLEAR_THIS_SERVER));
    clearing.appendChild(clearThis);
  }
  if (said.clearAllHistory) {
    const clearAll = aButton(said.clearAllHistory, "mini", "danger");
    clearAll.addEventListener("click", () => theHistoryIsAskedFor(theChosenJob, CLEAR_ALL));
    clearing.appendChild(clearAll);
  }
  head.appendChild(clearing);
  return head;
}

/// **The *Removed servers* group at the foot of History** — `Design/Queue &
/// History § History` and `§ Clearing history`, the drawing's `h-removed`:
/// one heading that folds like a Queue server's, its count, and *Clear removed
/// servers' history* on its own line under it, folded or open. **Every word
/// arrives on the event**; the application sends the clearing control only
/// while the group holds an entry a clear takes. Returns where its rows go.
function theRemovedServersGroup(said) {
  const section = document.createElement("section");
  section.classList.add("qmgroup");
  section.classList.add("hgone");
  section.dataset.open = theRemovedServersAreOpen ? "true" : "false";
  const head = document.createElement("div");
  head.className = "hgonehead";
  const fold = aButton(null, "qmach");
  fold.setAttribute("aria-expanded", section.dataset.open);
  const chev = document.createElement("span");
  chev.className = "chev";
  chev.textContent = "▼";
  fold.appendChild(chev);
  const name = document.createElement("b");
  name.textContent = said.theRemovedServers || "";
  fold.appendChild(name);
  const grow = document.createElement("span");
  grow.className = "grow";
  fold.appendChild(grow);
  const count = document.createElement("span");
  count.className = "qm2";
  aFigureAndItsWord(count, said.theRemovedFigure, said.theRemovedWord);
  fold.appendChild(count);
  // **Folding is the page's alone** — nothing is asked of the application and
  // nothing is drawn again, so a box or a typed figure is never disturbed.
  fold.addEventListener("click", () => {
    theRemovedServersAreOpen = !theRemovedServersAreOpen;
    section.dataset.open = theRemovedServersAreOpen ? "true" : "false";
    fold.setAttribute("aria-expanded", section.dataset.open);
  });
  head.appendChild(fold);
  if (said.clearRemovedServersHistory) {
    const clear = aButton(said.clearRemovedServersHistory, "mini", "danger");
    clear.addEventListener("click", () => theHistoryIsAskedFor(theChosenJob, CLEAR_REMOVED));
    head.appendChild(clear);
  }
  section.appendChild(head);
  const rows = document.createElement("div");
  rows.className = "qmrows";
  section.appendChild(rows);
  scroll.appendChild(section);
  return rows;
}

function theRowsAreDrawn(said) {
  const many = howMany(said, "hrowCount");
  const chosen = said.hrowChosen;
  // **The application sends removed servers' rows last, in the list's own
  // order**, and each says so; they go into the group, which is drawn at the
  // first of them.
  let theRemovedRows = null;
  for (let at = 0; at < many; at += 1) {
    const job = said["hrow" + at + "Job"] || "";
    const row = aButton(null, "hrow");
    if (job && job === chosen) row.setAttribute("aria-current", "true");
    row.dataset.job = job;
    row.addEventListener("click", () => theHistoryIsAskedFor(job));

    const name = document.createElement("span");
    name.className = "hn";
    const held1 = said["hrow" + at + "Name"];
    // Names read in the order their bytes run (X9, `web/the-value-drawn.js`).
    theValueIsDrawnInto(name, held1 || "");
    row.appendChild(name);

    // **`Queue & History` N6: the last frame Smoozy confirmed, not the range it was submitted
    // with.** Two values with a stroke between them and no word anywhere in it.
    const frames = document.createElement("span");
    frames.className = "hnum mono";
    const done = said["hrow" + at + "Frames"];
    if (done === undefined) {
      frames.textContent = "—";
    } else {
      const held2 = said["hrow" + at + "Frames"];
      frames.textContent = held2;
      frames.appendChild(document.createTextNode("/"));
      const held3 = said["hrow" + at + "Of"];
      frames.appendChild(document.createTextNode(held3 || ""));
    }
    row.appendChild(frames);

    const ending = document.createElement("span");
    const is = said["hrow" + at + "EndingIs"];
    // **The ending's colour is a class the application named and this file
    // adds** — two names rather than one string joined, which is the one place
    // the page may put a value.
    ending.classList.add("qst");
    if (is) ending.classList.add(is);
    const held4 = said["hrow" + at + "Ending"];
    ending.textContent = held4 || "";
    row.appendChild(ending);
    // **A first-frame pair's check, under the name, across the row** — the
    // drawing's `.hff` (the nineteenth notes, note 7).
    const theCheck = thePairIsDrawn(said, "hrow" + at, "hff");
    if (theCheck) row.appendChild(theCheck);

    const where = document.createElement("span");
    where.className = "hm";
    aBadge(where, said["hrow" + at + "Badge"]);
    const held5 = said["hrow" + at + "Server"];
    if (held5) where.appendChild(theValueDrawn(held5));
    if (said["hrow" + at + "Scene"]) {
      where.appendChild(document.createTextNode(" · "));
      const held6 = said["hrow" + at + "Scene"];
      where.appendChild(theValueDrawn(held6));
    }
    row.appendChild(where);

    const howLong = document.createElement("span");
    howLong.className = "hnum mono dim";
    const forHowLong = said["hrow" + at + "HowLong"];
    // **Absent is drawn as the drawing's own dash and never as a zero.**
    howLong.textContent = forHowLong === undefined ? "—" : forHowLong;
    row.appendChild(howLong);

    const when = document.createElement("span");
    when.className = "hwhen";
    const held7 = said["hrow" + at + "When"];
    // **Absent is absent**: a record whose ending Smoozy never learned and
    // whose submission predates the submitted-moment rule has no moment at all,
    // and a blank is what that is.
    when.textContent = held7 === undefined ? "" : theMomentReads(held7);
    row.appendChild(when);

    if (said["hrow" + at + "Removed"] === "yes") {
      if (!theRemovedRows) theRemovedRows = theRemovedServersGroup(said);
      theRemovedRows.appendChild(row);
    } else {
      scroll.appendChild(row);
    }
  }
}

/// **The one failure shape, on the entry, built by the builder that owns it.**
///
/// **Nothing here draws a shape.** `theFailureShape` in
/// `web/the-failure-shape.js` is the one builder — Design states the shape once
/// and the mockup then draws it eleven times in five modules with five class
/// prefixes — and this hands it the parts the application composed.
///
/// **No cross**: a recorded ending is not a live condition, so the application
/// composes none and the builder draws none. **And no line of either stream** —
/// M13b.
function theFailureIsDrawn(main, said) {
  if (!said.failSaid) return;
  // **Each value beside the label the application sent** — the page writes no
  // label of its own (must-do 20).
  const values = [];
  const many = howMany(said, "failValueCount");
  for (let at = 0; at < many; at += 1) {
    values.push([said["failValue" + at + "Label"] || "", said["failValue" + at] || ""]);
  }
  const shape = theFailureShape({
    sentence: said.failSaid,
    whatToDoAboutIt: said.failWhatToDoAboutIt,
    kind: "ending",
    // **The durable subject is the entry's own record** — the job, as a history
    // record, in the written form the application sent.
    subject: said.entryJob,
    subjectKind: said.failSubjectKind,
    theSentenceForTheReport: said.failSaidForTheReport,
    values,
    controls: theControlsFrom(said),
    room: EXPANDED,
    // **The one control on this shape that opens something here is *What the
    // server said***: the reason the machine recorded for the ending, whole,
    // in the one panel every screen opens a server's words in. Step 10c's
    // sixth pass, fault 3 — the machine recorded why both jobs failed and no
    // screen said it.
    onAct(act) {
      // **The failure's *Blender output* asks for the kept words** — the
      // same ask a clean finish's control makes (piece 6d).
      if (act === BLENDER_OUTPUT) {
        theHistoryIsAskedFor(said.entryJob, BLENDER_OUTPUT);
        return;
      }
      if (act !== "what-the-server-said" || !said.failTheServersWords) return;
      const panel = theWordsPanel({
        title: said.failTheServersWordsTitle,
        tabs: [{ name: said.failTheServersWordsTitle, record: said.failTheServersWords }],
        close: said.failWordsClose,
        copy: said.failWordsCopy,
      });
      panel.hidden = false;
      group.appendChild(panel);
    },
    // **No `onReport`: the shape's own *Report a problem…* opens the report
    // box** with M33's handover (`web/the-failure-shape.js`), and what it
    // carries is this entry's record — its job, as a history record, which
    // `Reporting a problem` X4 maps to the failed-job slice.
  });
  const group = document.createElement("div");
  group.className = "grp";
  group.appendChild(shape);
  main.appendChild(group);
}

/// **The two ways to render it again** — Design: each names its own range,
/// because a pair reading *Submit again* and *Resume* leaves the user working
/// out the difference. **Every figure here is one the application sent**
/// (must-do 2): the carry-on's frame, its count, and the frame the *why* line
/// names.
function theWaysToRenderItAgain(said) {
  if (!said.renderItAgain) return null;
  const job = said.entryJob || "";
  const group = document.createElement("div");
  group.className = "grp";
  const heading = document.createElement("h3");
  heading.textContent = said.renderItAgain;
  group.appendChild(heading);

  const again = document.createElement("div");
  again.className = "again";
  const pair = document.createElement("div");
  pair.className = "pair";

  const submit = aButton(null, "btn");
  submit.textContent = said.submitAgain || "";
  submit.appendChild(document.createTextNode(" "));
  submit.appendChild(document.createTextNode(said.submitAgainFrames || ""));
  submit.appendChild(document.createTextNode(BETWEEN_A_FIGURE_AND_ITS_UNIT));
  submit.appendChild(document.createTextNode(said.submitAgainFramesWord || ""));
  submit.addEventListener("click", () => anActIsAskedFor(submit, "submit-again", { job }));
  pair.appendChild(submit);

  let field = null;
  if (said.carryOn) {
    // **The figure is a field the user can type in**, and what it holds goes
    // with the carry-on act — Design: *"it is a field and not a fixed
    // number"*; `Rendering` A5.
    field = document.createElement("input");
    field.className = "fld w-num mono";
    field.type = "text";
    field.inputMode = "numeric";
    field.value = said.carryOnFrom || "";
    field.setAttribute("aria-label", said.carryingOnFromFrame || "");

    const carry = aButton(null, "btn", "quiet");
    carry.textContent = said.carryOn;
    carry.appendChild(document.createTextNode(" "));
    carry.appendChild(document.createTextNode(said.carryOnFrames || ""));
    carry.appendChild(document.createTextNode(BETWEEN_A_FIGURE_AND_ITS_UNIT));
    carry.appendChild(document.createTextNode(said.carryOnFramesWord || ""));
    carry.addEventListener("click", () =>
      anActIsAskedFor(carry, "carry-on", { job, from: field.value }),
    );
    pair.appendChild(carry);

    // **Only the carry-on says it leaves the frame range in two folders, and it
    // says it on its own line under the pair.**
    const twofold = document.createElement("p");
    twofold.className = "bar-note twofold";
    twofold.textContent = said.carryOnMakesANewJob || "";
    pair.appendChild(twofold);
  }
  again.appendChild(pair);

  if (field) {
    const from = document.createElement("div");
    from.className = "from";
    const lead = document.createElement("span");
    lead.textContent = said.carryingOnFromFrame || "";
    from.appendChild(lead);
    from.appendChild(field);
    // **Where the figure came from, as the application said it** — read from
    // the server just now, or the figure the entry recorded (must-do 2(c)).
    const trail = document.createElement("span");
    trail.textContent = said.carryOnFromSays || "";
    from.appendChild(trail);
    again.appendChild(from);

    const why = document.createElement("p");
    why.className = "why";
    const word = document.createElement("span");
    word.textContent = said.theFrameWord || "";
    why.appendChild(word);
    why.appendChild(document.createTextNode(" "));
    const frame = document.createElement("span");
    frame.className = "mono";
    frame.textContent = said.carryOnWhyFrame || "";
    why.appendChild(frame);
    why.appendChild(document.createTextNode(" "));
    const rest = document.createElement("span");
    rest.textContent = said.isTheOneSmoozyCannotConfirm || "";
    why.appendChild(rest);
    again.appendChild(why);
  } else if (said.carryOnWhy) {
    // **Absent rather than greyed, and one line says why.**
    const why = document.createElement("p");
    why.className = "why";
    why.textContent = said.carryOnWhy;
    again.appendChild(why);
  }

  group.appendChild(again);
  return group;
}


function theRenderedGroup(main, said) {
  const many = howMany(said, "renderedCount");
  if (!many) return;
  const group = document.createElement("div");
  group.className = "grp";
  const heading = document.createElement("h3");
  heading.textContent = said.renderedHeading || "";
  group.appendChild(heading);

  const facts = document.createElement("dl");
  facts.className = "facts";
  for (let at = 0; at < many; at += 1) {
    const fact = document.createElement("div");
    fact.className = "fact";
    const name = document.createElement("dt");
    const held8 = said["rendered" + at + "Name"];
    name.textContent = held8 || "";
    fact.appendChild(name);
    const value = document.createElement("dd");
    // **Absent is said as itself and never as a zero**, and it takes the
    // drawing's own grey for a value Smoozy cannot confirm.
    const held = said["rendered" + at + "Value"];
    if (held === undefined) value.className = "dimv";
    else value.className = "mono";
    // **Three of the six lines are moments and the application does not write
    // a date** — it sends the moment in seconds and this side chooses the
    // shape from the system's own locale data, exactly as a row's own moment
    // is written above. The application says which lines are moments; this
    // file guesses at none of them.
    if (said["rendered" + at + "Moment"] && held !== undefined) {
      value.textContent = theMomentReads(held);
    } else {
      aPart(value, said, "rendered" + at);
    }
    // **The second half of the last-frame line**, which carries a figure of
    // its own and takes the drawing's own grey beside the value.
    const beside = document.createElement("span");
    beside.className = "res";
    if (aPart(beside, said, "renderedB" + at)) value.appendChild(beside);
    fact.appendChild(value);
    facts.appendChild(fact);
  }
  group.appendChild(facts);

  if (said.renderedNote) {
    const note = document.createElement("p");
    note.className = "bar-note";
    note.textContent = said.renderedNote;
    group.appendChild(note);
  }
  main.appendChild(group);
}

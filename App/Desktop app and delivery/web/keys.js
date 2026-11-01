// **Keys, on the page** — the list in the rail, and the same list opened from a
// server dialog to pick a key. The application half is `src/surface/keys.rs`,
// whose head carries the whole account of the piece.
//
// **Read the head of `smoozy.js` and the mounting contract at the head of
// `the-rail.js` first.** This module keeps every part of both: it registers one
// place, it makes its own elements inside the host it is given, it adds nothing
// to `index.html`, and it draws only what an event handed it.
//
// ## What is drawn, and where each part goes
//
// - **The screen** — the head, the one line above the list, the list, and the
//   foot — goes inside the host the shell gives this module. It is built once
//   and kept, which is the contract's own answer: a scroll position and a
//   half-open dialog survive going away and coming back for nothing.
// - **Every box of this screen's own** — the New key dialog, the refusals, the
//   statement, and the box that takes a key away — is appended to the host too.
//   The drawing's `.modal` is `position: absolute; inset: 0`, so each covers the
//   main pane and leaves the rail, which is exactly where the mockup draws them.
// - **The passphrase box is not this screen's and is not in the host.** Design:
//   *"the box is the connection's, not this screen's"*, and the drawing puts it
//   over the server list. It is appended to `.main` instead, because a box
//   inside this screen's host would go away with the screen — and the connect
//   that raises it is one the user started from somewhere else.
//
// ## Four things this file must never do
//
// - **Never write a word of prose into this file.** A6 and X1: the typography
//   point is the only way prose reaches a screen, and the enforcement is
//   structural on both sides. Every word below arrived on the event.
// - **Never join a value into a sentence.** X7. A key's name is the user's own
//   comment, a fingerprint is what they copy, a path is a path and a server's
//   name is a server's name — each arrives as its own node beside the words.
// - **Never build a failure shape of its own.** The mockup draws that shape
//   eleven times in five class prefixes and two of them are on this screen —
//   `.ky-why` on a row and `.rdfail` in a box. **`theFailureShape` is the one
//   shape** and both are drawn with it.
// - **Never copy the mockup's markup.** That file carries 88 inline `style=`
//   attributes and four uses of `innerHTML`, every one inert under this
//   application's content policy. H32: values are content, never markup. H33:
//   no inline style; the rules are in `keys.css`.
//
// **P9 and P10: nothing here polls and nothing here ticks.** The list is asked
// for when the user arrives and after an act, and at no other moment.

import { whenTold } from "./event-route.js";
import { aScreenIsRegistered, theCurrentPlaceIs, KEYS } from "./the-rail.js";
import { theFailureShape, EXPANDED } from "./the-failure-shape.js";
import { theMarkOnAControl, theMarkComesOffAControl } from "./the-underway-shape.js";
import { theValueIsDrawnInto } from "./the-value-drawn.js";

const shell = window.__TAURI__;

// The four kinds, declared beside the service that posts them —
// `THE_KEYS_SCREEN`, `A_KEYS_VERDICT`, `AN_ANSWER` and `THE_PASSPHRASE_ASK` in
// `src/surface/keys.rs`. The two sides have to agree and nothing holds them
// together, so copy carefully.
const THE_KEYS_SCREEN = "keys:the-screen";
const A_KEYS_VERDICT = "keys:a-verdict";
const AN_ANSWER = "keys:an-answer";
const THE_PASSPHRASE_ASK = "keys:the-passphrase-ask";

// ── What this module holds ──────────────────────────────────────────────────

// The built elements, once the screen has been drawn.
let screen = null;
// The last thing the application said about the list. **It is what the page is
// looking at and nothing durable** — events are volatile (P8) and the
// application is asked again rather than this being remembered across a run.
let held = null;
// identity → the row's own elements.
const rows = new Map();
// **The picking mode's own two values**, run-held and gone with the run: what
// the caller handed over, and what to call when the user picks or cancels.
let picking = null;
// identity → a sentence about the last act on that row, until the next act.
const rowNotes = new Map();

// ── The one contract a server dialog uses ───────────────────────────────────

/// **Open the Keys screen to pick a key for one server** — `Keys` U11 and
/// `Design/Keys § Opened from a server dialog`.
///
/// `context` is what the caller already holds, handed as text and nothing else:
/// `{ label, username, host }`, **any or all of which may be empty**. Nothing is
/// read from the local record for an unsaved server and no name is composed for
/// one from anything else — where nothing names the server the line reads
/// *Choosing a key* and stops.
///
/// `whenItEnds` is called with **one of two things and never a third**: the pair
/// `{ row, fingerprint }` at a pick, or `null` at a cancel. Keys hands back no
/// fields, no context and no record of having been open, so a caller's own
/// half-filled dialog is its to keep and is never handed over.
///
/// **Unusable keys are not offered here** and their choose control is absent
/// rather than dimmed.
///
/// `withTheNewKeyDialog` is true when the dialog's *New key…* was pressed:
/// **the screen opens with the New key dialog already showing** —
/// `Design/Keys § Opened from a server dialog`, the Creator's note 1 on 0.1.7.
/// The dialog's key field passes nothing and opens the list alone. Either way
/// `Keys` U11 hands over the same context, so the pick ends the same way.
export function aServerDialogPicksAKey(context, whenItEnds, withTheNewKeyDialog) {
  picking = { context: context || {}, whenItEnds };
  theCurrentPlaceIs(KEYS);
  theApplicationIsAsked();
  // **The screen is drawn by now** — mounting a place draws it in the same
  // turn — so the box is opened over a screen that stands.
  if (withTheNewKeyDialog === true && screen) theNewKeyDialogIsOpened();
}

/// **Whether the screen is standing in its picking mode**, for a caller that has
/// to know before it opens a second one.
export function theScreenIsPickingAKey() {
  return picking !== null;
}

// ── The screen ──────────────────────────────────────────────────────────────

aScreenIsRegistered({
  place: KEYS,
  itIsDrawn(host) {
    theScreenIsBuilt(host);
  },
  itIsCurrentNow() {
    // **P8: events are volatile and nothing is replayed**, so a screen that has
    // been away asks again rather than drawing what it remembers.
    theApplicationIsAsked();
  },
  itIsPutAway() {
    theBoxesAreShut();
    // **A pick the user walked away from ends as a cancel** (step 10a round 1,
    // fix-here item 2(c)), so this screen is not left standing in its picking
    // mode. **The caller is told once the going has finished**, so a caller
    // that sends the user back on a cancel can see they went elsewhere.
    if (picking) {
      const asking = picking;
      picking = null;
      queueMicrotask(() => {
        if (asking.whenItEnds) asking.whenItEnds(null);
      });
    }
  },
});

function theApplicationIsAsked() {
  const naming = picking ? theCallerNames(picking.context) : null;
  // **Whether this is a pick travels on its own and never inside the name**
  // (step 10a round 1, fix-here item 2(b)): a pick that names nothing is still
  // a pick.
  shell.core.invoke("the_keys_screen", { picking: picking !== null, pickingFor: naming });
}

// **An indexed field's name on the event.** The application writes `row0Name`,
// `usedBy1Badge`, `file2`; this is the other end of that, and it is a function
// rather than a template literal written where the value is used — a literal
// standing in the same statement as a `textContent` is exactly the shape the
// page-wide prose sweep catches, and it should catch it.
function field(...parts) {
  return parts.join("");
}

// **The name the caller hands over, or nothing.** `Keys` U11's own shape: what
// exists, and nothing where there is none. The application composes the same
// answer for the ask it raises; this is the picking side of it.
function theCallerNames(context) {
  const label = (context.label || "").trim();
  if (label) return label;
  const username = (context.username || "").trim();
  const host = (context.host || "").trim();
  return username && host ? `${username}@${host}` : null;
}

function theScreenIsBuilt(host) {
  const head = document.createElement("div");
  head.className = "svhead";
  const heading = document.createElement("h1");
  const pickLine = document.createElement("span");
  pickLine.className = "qsub";
  pickLine.hidden = true;
  const pickWords = document.createElement("span");
  const pickName = document.createElement("b");
  pickLine.appendChild(pickWords);
  pickLine.appendChild(document.createTextNode(" "));
  pickLine.appendChild(pickName);
  const grow = document.createElement("span");
  grow.className = "grow";
  const choose = document.createElement("button");
  choose.type = "button";
  choose.className = "mini";
  choose.addEventListener("click", () => {
    shell.core.invoke("a_key_is_pointed_at", { offer: "a-first-point", row: null });
  });
  const make = document.createElement("button");
  make.type = "button";
  make.className = "btn";
  make.addEventListener("click", () => {
    theNewKeyDialogIsOpened();
  });
  head.appendChild(heading);
  head.appendChild(pickLine);
  head.appendChild(grow);
  head.appendChild(choose);
  head.appendChild(make);

  // **`Keys` U6a and G6: one line above the list, never a message per key.**
  const noTools = document.createElement("div");
  noTools.className = "ky-notool";
  noTools.hidden = true;
  const noToolsLead = document.createElement("b");
  const noToolsUntil = document.createElement("span");
  noTools.appendChild(noToolsLead);
  noTools.appendChild(document.createTextNode(" "));
  noTools.appendChild(noToolsUntil);

  const list = document.createElement("div");
  list.className = "ky-list";

  const foot = document.createElement("div");
  foot.className = "svfoot";
  const footWords = document.createElement("span");
  foot.appendChild(footWords);

  const cancelFoot = document.createElement("div");
  cancelFoot.className = "svfoot";
  cancelFoot.hidden = true;
  const cancelGrow = document.createElement("span");
  cancelGrow.className = "grow";
  const cancel = document.createElement("button");
  cancel.type = "button";
  cancel.className = "mini";
  cancel.addEventListener("click", () => {
    thePickEnds(null);
  });
  cancelFoot.appendChild(cancelGrow);
  cancelFoot.appendChild(cancel);

  // **`Design/Keys § Keyboard shortcuts`: *"Return chooses the key where this
  // screen was opened to pick one, and does nothing otherwise. The delete key
  // deletes or removes, whichever the chosen row offers, asking exactly as the
  // control does."*** The chosen row is the one the keyboard is on. Return on a
  // control inside a row is that control's own press, so it is left alone; a
  // key pressed inside one of this screen's boxes is the box's.
  host.addEventListener("keydown", (press) => {
    if (press.defaultPrevented) return;
    const row = press.target.closest && press.target.closest(".ky-row");
    if (!row) return;
    const drawn = rows.get(row.dataset.row);
    if (!drawn) return;
    if (press.key === "Enter" && press.target === row) {
      press.preventDefault();
      if (drawn.isPicking && !drawn.choose.hidden) drawn.choose.click();
      return;
    }
    if (press.key === "Delete" || press.key === "Backspace") {
      press.preventDefault();
      drawn.away.click();
    }
  });

  host.appendChild(head);
  host.appendChild(noTools);
  host.appendChild(list);
  host.appendChild(foot);
  host.appendChild(cancelFoot);

  screen = {
    host,
    heading,
    pickLine,
    pickWords,
    pickName,
    choose,
    make,
    noTools,
    noToolsLead,
    noToolsUntil,
    list,
    foot,
    footWords,
    cancelFoot,
    cancel,
    boxes: theBoxesAreBuilt(host),
  };
  if (held) theScreenIsDrawn(held);
}

// ── The list ────────────────────────────────────────────────────────────────

whenTold(THE_KEYS_SCREEN, (delivery) => {
  held = delivery.said;
  if (screen) theScreenIsDrawn(held);
});

function theScreenIsDrawn(said) {
  screen.heading.textContent = said.theHeading || "";
  screen.choose.textContent = said.chooseAKey || "";
  screen.make.textContent = said.newKey || "";
  screen.cancel.textContent = said.cancelAndGoBack || "";
  // **The foot says what an act came to and then goes back to being the
  // foot.** A statement left standing under a list nobody has touched since is
  // a statement about nothing.
  screen.foot.replaceChildren(screen.footWords);
  screen.footWords.textContent = said.whatGoesToAProvider || "";

  // **`Design/Keys § Opened from a server dialog`: the same list, with a control
  // per row, and it says which server it is choosing for where anything names
  // one.**
  const isPicking = said.picking === "yes";
  screen.list.classList.toggle("picking", isPicking);
  screen.pickLine.hidden = !isPicking;
  screen.foot.hidden = isPicking;
  screen.cancelFoot.hidden = !isPicking;
  if (isPicking) {
    // **One piece of prose either way**, composed by the application (step 10a
    // round 1, fix-here item 30). Where nothing names the server the line is
    // the act alone.
    screen.pickWords.textContent = (said.theServerNamed ? said.choosingAKeyFor : said.choosingAKey) || "";
    theValueIsDrawnInto(screen.pickName, said.theServerNamed || "");
  }

  // **`Keys` U6a: the cause is a property of the computer and is said once.**
  // *New key…* reads unavailable with this line as its reason, and no row
  // carries a verdict.
  const missing = said.theToolsAreMissing === "yes";
  screen.noTools.hidden = !missing;
  screen.make.disabled = missing;
  screen.list.classList.toggle("noverdict", missing);
  if (missing) {
    screen.noToolsLead.textContent = said.theToolsAreNotInstalled || "";
    screen.noToolsUntil.textContent = said.untilTheyAre || "";
  }

  // **The list is rebuilt from the event.** `Keys` R1 makes the verdict derived
  // and never durable, so nothing of a previous draw is carried over: a stale
  // verdict beside a fresh row is exactly what `Keys` U10a's *no verdict is drawn that
  // has not been read* forbids.
  // **The chosen row stays chosen across the redraw**, so a key pressed after an
  // act still lands on the row the user was on.
  const focused = document.activeElement && document.activeElement.closest
    ? document.activeElement.closest(".ky-row")
    : null;
  const chosen = focused && screen.list.contains(focused) ? focused.dataset.row : null;
  screen.list.replaceChildren();
  rows.clear();
  const many = Number(said.rowCount || 0);
  for (let at = 0; at < many; at += 1) {
    const row = theRowIsDrawn(said, at, isPicking);
    if (row) screen.list.appendChild(row.root);
  }
  if (chosen && rows.has(chosen)) rows.get(chosen).root.focus();
}

function theRowIsDrawn(said, at, isPicking) {
  const identity = said[field("row", at, "Identity")];
  if (!identity) return null;
  const itsName = said[field("row", at, "Name")];
  const itsFingerprint = said[field("row", at, "Fingerprint")];
  const itsPath = said[field("row", at, "Path")];
  const itsKind = said[field("row", at, "Kind")];
  const itHoldsAPassphrase = said[field("row", at, "PassphraseHeld")] === "yes";
  const theServersAreUnread = said[field("row", at, "ServersUnread")] === "yes";

  const root = document.createElement("div");
  root.className = "ky-row";
  // **Reachable by the keyboard, and not a button** — the same answer the
  // servers screen gives its rows, and for its reason: a row with a button's
  // role hides the controls inside it.
  root.tabIndex = 0;
  root.dataset.row = identity;

  const main = document.createElement("span");
  main.className = "ky-main";

  const name = document.createElement("span");
  name.className = "ky-name";
  // Names and paths read in the order their bytes run (X9,
  // `web/the-value-drawn.js`), here and below.
  theValueIsDrawnInto(name, itsName || "");
  name.hidden = !itsName;

  // **`Design/Keys § A row`: the fingerprint is under the name, in the
  // fixed-width face, always.** It is what tells one key from another.
  const fingerprint = document.createElement("span");
  fingerprint.className = isPicking ? "ky-fp mono" : "ky-fp mono selectable";
  fingerprint.textContent = itsFingerprint || "";
  fingerprint.hidden = !itsFingerprint;

  const where = document.createElement("span");
  where.className = isPicking ? "ky-where mono" : "ky-where mono selectable";
  theValueIsDrawnInto(where, itsPath || "");

  main.appendChild(name);
  main.appendChild(fingerprint);
  main.appendChild(where);

  // **`Keys` W11a's line: named rather than counted.**
  const usedBy = theUsedByLine(said, at);
  if (usedBy) main.appendChild(usedBy);
  if (theServersAreUnread) {
    const unread = document.createElement("span");
    unread.className = "ky-use";
    unread.textContent = said.couldNotCheckTheSavedServers || "";
    main.appendChild(unread);
  }

  // **`Design/Keys § A remembered passphrase`: one line, only where there is
  // one, with the control that clears it.**
  if (itHoldsAPassphrase) {
    const line = document.createElement("span");
    line.className = "ky-pass";
    const words = document.createElement("span");
    words.textContent = said.aPassphraseIsRemembered || "";
    const clear = document.createElement("button");
    clear.type = "button";
    clear.className = "ky-lnk ky-asalink";
    clear.textContent = said.clearIt || "";
    clear.addEventListener("click", () => {
      rowNotes.clear();
      shell.core.invoke("a_remembered_passphrase_is_cleared", { row: identity });
    });
    line.appendChild(words);
    line.appendChild(document.createTextNode(" · "));
    line.appendChild(clear);
    main.appendChild(line);
  }

  // **What the last act on this row came to, where it did not all go** (step
  // 10a round 1, fix-here item 18).
  if (rowNotes.has(identity)) {
    const note = document.createElement("span");
    note.className = "ky-use";
    note.textContent = rowNotes.get(identity);
    main.appendChild(note);
  }

  // **`Keys` U10a: the one place the verdict would go carries the underway mark
  // until the reading answers.** It is the whole-app *underway on the control*
  // shape at its smallest, which is Design's own sentence — the mockup draws a
  // pulse of its own instead and that is one of the copies this application has
  // one shape in place of.
  const verdictSlot = document.createElement("span");
  verdictSlot.className = "ky-why ky-slot";
  theMarkOnAControl(verdictSlot, said.checking || "");
  main.appendChild(verdictSlot);

  const acts = document.createElement("span");
  acts.className = "ky-acts";

  const choose = document.createElement("button");
  choose.type = "button";
  choose.className = "mini choose";
  choose.textContent = said.useThisKey || "";
  choose.hidden = true;
  choose.addEventListener("click", () => {
    thePickEnds({
      row: identity,
      fingerprint: itsFingerprint || null,
    });
  });

  // **The controls sit in fixed slots** — `Design/Keys § A row`. The row with no
  // copy keeps the slot and leaves it empty rather than closing it up.
  const copySlot = document.createElement("span");
  copySlot.className = "ky-copy";
  const copy = document.createElement("button");
  copy.type = "button";
  copy.className = "mini";
  copy.textContent = said.copyPublicKey || "";
  copy.addEventListener("click", () => {
    shell.core.invoke("a_public_half_is_copied", { row: identity });
  });

  // **`Keys` W2's derivation, at the draw.** *Delete* takes the files, *Remove*
  // drops the reference and leaves them where they sit, and the control is the
  // whole of the difference.
  const managed = itsKind === "managed";
  const away = document.createElement("button");
  away.type = "button";
  away.className = managed ? "mini danger ky-go" : "mini ky-go";
  away.textContent = (managed ? said.delete : said.remove) || "";
  // **Each says what it does on hover** — `Design/Keys § The two kinds`, sixth
  // notes, note 7. The words are the application's; this only puts them here.
  away.title = (managed ? said.deleteOnHover : said.removeOnHover) || "";
  away.addEventListener("click", () => {
    shell.core.invoke("what_taking_a_key_away_costs", { row: identity });
  });

  acts.appendChild(choose);
  acts.appendChild(copySlot);
  acts.appendChild(away);

  root.appendChild(main);
  root.appendChild(acts);

  const drawn = {
    root,
    main,
    acts,
    choose,
    copySlot,
    copy,
    away,
    verdictSlot,
    isPicking,
    said,
    at,
  };
  rows.set(identity, drawn);
  return drawn;
}

// **The drawing's own badge.** The colour arrived as a value and is one of the
// eight the application already refused to widen; the classes are named
// severally rather than composed into a string.
function theBadge(colour) {
  const mark = document.createElement("span");
  mark.classList.add("badge", "sm", colour);
  return mark;
}

function theUsedByLine(said, at) {
  const many = Number(said[field("row", at, "UsedByCount")] || 0);
  if (!many) return null;
  const line = document.createElement("span");
  line.className = "ky-use";
  const lead = document.createElement("span");
  lead.textContent = said.usedBy || "";
  line.appendChild(lead);
  for (let which = 0; which < many; which += 1) {
    if (which > 0) {
      const joining = document.createElement("span");
      joining.className = "ky-and";
      joining.textContent = said.and || "";
      line.appendChild(joining);
    }
    const badge = said[field("row", at, "UsedBy", which, "Badge")];
    const called = said[field("row", at, "UsedBy", which)];
    if (badge) line.appendChild(theBadge(badge));
    const named = document.createElement("span");
    theValueIsDrawnInto(named, called || "");
    line.appendChild(named);
  }
  return line;
}

// ── One key's verdict, arriving behind its row ──────────────────────────────

whenTold(A_KEYS_VERDICT, (delivery) => {
  const said = delivery.said;
  const row = rows.get(said.row);
  if (!row) return;
  theVerdictIsDrawn(row, said);
});

function theVerdictIsDrawn(row, said) {
  const usable = said.verdict === "usable";
  const unusable = said.verdict === "unusable";

  // **`Design/Keys § An unusable key`: it stays in the list, dimmed, and says
  // what is wrong in the row.** The dim says one thing — *this key cannot be
  // chosen* — and the message is the failure shape beside it.
  row.root.classList.toggle("ky-gone", unusable);

  // The underway mark goes wherever a verdict has arrived.
  row.verdictSlot.replaceChildren();
  row.verdictSlot.classList.remove("uway");
  row.verdictSlot.removeAttribute("aria-disabled");
  row.verdictSlot.hidden = true;

  if (unusable) {
    // **The one failure shape, in the row.** No cross: `Design/Keys` settled
    // that an unusable key cleared would look like a key that was never there,
    // and the two ways out stand where a cross would.
    const shape = theFailureShape({
      sentence: said.whatIsWrong,
      whatToDoAboutIt: said.andWhatToDo,
      room: EXPANDED,
      controls: [
        { is: "try-again", label: row.said.chooseItsNewLocation, act: "its-new-location" },
        {
          is: "the-screens-own-way-past",
          label: row.said.chooseAReplacement,
          act: "a-replacement",
        },
      ],
      onAct: (act) => {
        shell.core.invoke("a_key_is_pointed_at", { offer: act, row: said.row });
      },
    });
    shape.classList.add("ky-failure");
    // **What a replacement costs, said before it is pressed** — `Keys` F12. It
    // is the screen's own line beside the shape rather than a part of it: the
    // shape's second sentence is about the failure, and this is about a control.
    const note = document.createElement("span");
    note.className = "ky-note";
    note.textContent = row.said.whatAReplacementCosts || "";
    shape.appendChild(note);
    row.verdictSlot.replaceChildren(shape);
    row.verdictSlot.hidden = false;
    row.verdictSlot.className = "ky-slot";
  }

  // **`Design/Keys § When there is no public key file beside the key`: the copy
  // control is absent, not dimmed, and one line says why.**
  // **Copy needs no verdict and it needs no tool.** Its own reading is whether
  // the `.pub` is beside the key, which is the filesystem's answer — so a key on
  // a computer with no SSH tools keeps the control, which is `Keys` U6a's rule
  // read as it is written: *not checked* withholds a **verdict** and nothing
  // else. **`Keys` W7a's comparison is what the missing tool costs**, and the
  // copy says so when it is pressed.
  const hasPublic = said.thePublicHalfIsBesideIt === "yes";
  row.copySlot.replaceChildren();
  if (!unusable && hasPublic) row.copySlot.appendChild(row.copy);
  // **The line is only where a reading answered.** *The key itself works* is a
  // statement about the key, and on a computer with no tool nothing established
  // it.
  if (usable && !hasPublic) {
    const why = document.createElement("span");
    why.className = "ky-why ky-state";
    const first = document.createElement("span");
    first.textContent = said.noPublicKeyFile || "";
    const second = document.createElement("span");
    second.textContent = said.theKeyItselfWorks || "";
    why.appendChild(first);
    why.appendChild(document.createTextNode(" "));
    why.appendChild(second);
    row.main.appendChild(why);
  }

  // **Unusable keys are not offered in the picking mode**, and the choose
  // control is absent rather than dimmed.
  row.choose.hidden = !row.isPicking || !usable;
}

// ── The boxes ───────────────────────────────────────────────────────────────

function theBoxesAreBuilt(host) {
  const boxes = {
    newKey: theNewKeyBox(),
    refusal: theRefusalBox(),
    sameName: theSameNameBox(),
    takingAway: theTakingAwayBox(),
  };
  for (const box of Object.values(boxes)) host.appendChild(box.root);
  return boxes;
}

function theBoxesAreShut() {
  if (!screen) return;
  for (const box of Object.values(screen.boxes)) box.root.hidden = true;
  // **A screen put away ends whatever its boxes were standing in the middle
  // of**, and *Create and copy public key* comes back with them.
  theMakeControlComesBack();
}

function aBox() {
  const root = document.createElement("div");
  root.className = "modal";
  root.hidden = true;
  const box = document.createElement("div");
  box.className = "mbox";
  const heading = document.createElement("h3");
  box.appendChild(heading);
  root.appendChild(box);
  return { root, box, heading };
}

// ── Making a key ────────────────────────────────────────────────────────────

function theNewKeyBox() {
  const { root, box, heading } = aBox();

  const form = document.createElement("div");
  form.className = "ky-form";

  const nameLabel = document.createElement("label");
  nameLabel.className = "ky-fl";
  nameLabel.htmlFor = "ky-name";
  const nameCell = document.createElement("span");
  nameCell.className = "ky-fi";
  const nameField = document.createElement("input");
  nameField.type = "text";
  nameField.id = "ky-name";
  nameField.className = "fld";
  const nameNote = document.createElement("span");
  nameNote.className = "ky-nt";
  nameCell.appendChild(nameField);
  nameCell.appendChild(nameNote);

  const passLabel = document.createElement("label");
  passLabel.className = "ky-fl";
  passLabel.htmlFor = "ky-pass";
  const passCell = document.createElement("span");
  passCell.className = "ky-fi";
  const passField = document.createElement("input");
  passField.type = "password";
  passField.id = "ky-pass";
  passField.className = "fld";
  passCell.appendChild(passField);

  const advLabel = document.createElement("span");
  advLabel.className = "ky-fl";
  const advCell = document.createElement("span");
  advCell.className = "ky-fi";
  const advOpen = document.createElement("button");
  advOpen.type = "button";
  advOpen.className = "ky-lnk ky-asalink";
  advCell.appendChild(advOpen);

  const typesLabel = document.createElement("label");
  typesLabel.className = "ky-fl";
  typesLabel.hidden = true;
  const typesCell = document.createElement("span");
  typesCell.className = "ky-fi";
  typesCell.hidden = true;
  const types = document.createElement("span");
  types.className = "rads";
  const typesNote = document.createElement("span");
  typesNote.className = "ky-nt";
  typesNote.hidden = true;
  typesCell.appendChild(types);
  typesCell.appendChild(typesNote);

  // **The control goes and its cell stays.** `.ky-form` is a two-column grid
  // and a hidden cell leaves the grid, which puts every cell after it in the
  // wrong column — the label lands where the field goes and the choices land
  // under the labels. Found by looking at it.
  advOpen.addEventListener("click", () => {
    const open = typesCell.hidden;
    typesCell.hidden = !open;
    typesLabel.hidden = !open;
    advOpen.hidden = open;
  });

  form.appendChild(nameLabel);
  form.appendChild(nameCell);
  form.appendChild(passLabel);
  form.appendChild(passCell);
  form.appendChild(advLabel);
  form.appendChild(advCell);
  form.appendChild(typesLabel);
  form.appendChild(typesCell);

  // **The ending, and it is the one moment the user has any reason to know the
  // file's name** — `Design/Keys § Making a key`.
  const made = document.createElement("div");
  made.className = "ky-made";
  made.hidden = true;
  const madeWhere = document.createElement("p");
  madeWhere.className = "msub";
  const madeWords = document.createElement("span");
  const madePath = document.createElement("span");
  madePath.className = "mono selectable";
  madeWhere.appendChild(madeWords);
  madeWhere.appendChild(document.createTextNode(" "));
  madeWhere.appendChild(madePath);
  madeWhere.appendChild(document.createTextNode("."));
  const madeCopied = document.createElement("p");
  madeCopied.className = "msub";
  const madeCopiedFirst = document.createElement("span");
  const madeCopiedSecond = document.createElement("span");
  madeCopied.appendChild(madeCopiedFirst);
  madeCopied.appendChild(document.createTextNode(" "));
  madeCopied.appendChild(madeCopiedSecond);
  // Where the public key is shown when the copy did not work.
  const madeKey = document.createElement("div");
  // **`Security and licence` S34's one sentence, said once for the life of the
  // installation** where this make was the first act to find there is nowhere
  // to keep a passphrase — `Design/Keys § On a computer with nowhere to keep a
  // passphrase`, mockup state `keys-madenostore`. The words are the
  // application's.
  const madeNowhere = document.createElement("p");
  madeNowhere.className = "msub";
  madeNowhere.hidden = true;
  made.appendChild(madeWhere);
  made.appendChild(madeCopied);
  made.appendChild(madeNowhere);
  made.appendChild(madeKey);

  const failed = document.createElement("div");
  failed.hidden = true;

  const acts = document.createElement("div");
  acts.className = "mact";
  const cancel = document.createElement("button");
  cancel.type = "button";
  cancel.className = "btn quiet";
  cancel.addEventListener("click", () => {
    root.hidden = true;
    // **A cancelled act is an act that ended**, so the control comes back with
    // it rather than waiting for an answer that is not coming.
    theMakeControlComesBack();
  });
  const go = document.createElement("button");
  go.type = "button";
  go.className = "btn";
  go.addEventListener("click", () => {
    theKeyIsAskedFor();
  });
  acts.appendChild(cancel);
  acts.appendChild(go);

  const done = document.createElement("div");
  done.className = "mact";
  done.hidden = true;
  const doneControl = document.createElement("button");
  doneControl.type = "button";
  doneControl.className = "btn";
  doneControl.addEventListener("click", () => {
    root.hidden = true;
  });
  done.appendChild(doneControl);

  box.appendChild(form);
  box.appendChild(failed);
  box.appendChild(made);
  box.appendChild(acts);
  box.appendChild(done);

  return {
    root,
    heading,
    form,
    nameLabel,
    nameField,
    nameNote,
    passLabel,
    passField,
    advOpen,
    advCell,
    typesLabel,
    typesCell,
    types,
    typesNote,
    made,
    madeWords,
    madePath,
    madeCopiedFirst,
    madeCopiedSecond,
    madeNowhere,
    madeKey,
    failed,
    acts,
    cancel,
    go,
    done,
    doneControl,
    chosen: null,
  };
}

function theNewKeyDialogIsOpened() {
  const box = screen.boxes.newKey;
  box.root.hidden = false;
  box.form.hidden = false;
  box.acts.hidden = false;
  box.made.hidden = true;
  box.done.hidden = true;
  box.failed.hidden = true;
  box.failed.replaceChildren();
  box.nameField.value = "";
  box.passField.value = "";
  box.typesCell.hidden = true;
  box.typesLabel.hidden = true;
  box.advOpen.hidden = false;
  // **`Keys` T2a: the dialog draws first and the type control fills in behind
  // it.** Three generations, each its own process, are exactly what P5 keeps
  // off the drawing thread, so the set is asked for here and arrives on the
  // queue.
  box.types.replaceChildren();
  box.chosen = null;
  // **A dialog opened again opens ready to be used** — it is the same control
  // and the act it was marked for is long over.
  theMakeControlComesBack();
  shell.core.invoke("the_types_the_dialog_may_offer");
}

function theKeyIsAskedFor() {
  const box = screen.boxes.newKey;
  if (box.go.disabled) return;
  // **The underway mark on *Make it*, and the control takes no second press
  // until the answer comes** — two presses made two keys (step 10a round 1,
  // fix-here item 20). **It comes back whenever the act ends without a key** —
  // refused, failed, or the dialog closed — which is `theMakeControlComesBack`'s
  // three callers plus this one's answers (the close check's repair,
  // 2026-09-17).
  box.go.disabled = true;
  theMarkOnAControl(box.go, box.goLabel || "");
  // **A press with no type chosen is sent rather than swallowed** (item 32): the
  // page holds no sentence of its own, so the refusal is the application's to
  // word and to draw where the user pressed. ~~The press returned here in
  // silence~~ when it was made before the type probe answered.
  shell.core.invoke("a_key_is_made", {
    name: box.nameField.value,
    passphrase: box.passField.value,
    keyType: box.chosen || "",
  });
}

function theTypesAreDrawn(said) {
  const box = screen.boxes.newKey;
  box.heading.textContent = said.newKeyHeading || "";
  box.nameLabel.textContent = said.theNameField || "";
  box.nameNote.textContent = said.nameItAfterTheProvider || "";
  box.passLabel.textContent = said.thePassphraseField || "";
  box.passField.placeholder = said.optional || "";
  box.advOpen.textContent = said.advancedKeyType || "";
  box.typesLabel.textContent = said.keyType || "";
  box.cancel.textContent = said.cancel || "";
  box.goLabel = said.makeItAndCopy || "";
  if (!box.go.disabled) box.go.textContent = box.goLabel;

  // **`Keys` G6: where the probe answers *no generator*, the dialog is replaced
  // by the statement rather than opened beside it** — a dialog whose one control
  // can offer nothing is not a screen the user asked for.
  if (said.theToolsAreMissing === "yes") {
    box.root.hidden = true;
    return;
  }
  if (said.stillReading === "yes") return;

  box.types.replaceChildren();
  const many = Number(said.typeCount || 0);
  for (let at = 0; at < many; at += 1) {
    const named = said[field("type", at)];
    if (!named) continue;
    const choice = document.createElement("button");
    choice.type = "button";
    choice.className = "rad";
    const dot = document.createElement("i");
    const label = document.createElement("span");
    label.textContent = named;
    choice.appendChild(dot);
    choice.appendChild(label);
    if (said[field("type", at, "IsTheDefault")] === "yes") {
      choice.classList.add("on");
      box.chosen = named;
      // **The default is marked as the default and nothing more** — not
      // *recommended*, which implies a comparison nobody made.
      const tag = document.createElement("span");
      tag.className = "tag";
      tag.textContent = said.theDefaultMark || "";
      choice.appendChild(tag);
    }
    choice.addEventListener("click", () => {
      for (const other of box.types.children) other.classList.remove("on");
      choice.classList.add("on");
      box.chosen = named;
    });
    box.types.appendChild(choice);
  }

  // **Where the usual one cannot be made, one line says so** — a user who
  // opened Advanced expecting it is owed the reason. The type's own name is a
  // value beside the words.
  const fallback = said.theUsualCannotBeMade;
  box.typesNote.hidden = !fallback;
  if (fallback) {
    box.typesNote.replaceChildren();
    const named = document.createElement("span");
    named.textContent = said.theUsualType || "";
    const words = document.createElement("span");
    words.textContent = fallback;
    box.typesNote.appendChild(named);
    box.typesNote.appendChild(document.createTextNode(" "));
    box.typesNote.appendChild(words);
  }
}

// ── The refusals, and the statement ─────────────────────────────────────────

function theRefusalBox() {
  const { root, box, heading } = aBox();
  // **The file the user pointed at, named in every one of the six boxes.**
  const path = document.createElement("p");
  path.className = "msub mono selectable";
  const body = document.createElement("div");
  // **`Keys` F4's own sentence, which names the existing entry by its name and
  // by where it sits.** Four nodes, because two of them are values (X7).
  const already = document.createElement("p");
  already.className = "msub";
  const alreadyLead = document.createElement("span");
  const alreadyName = document.createElement("b");
  const alreadyAt = document.createElement("span");
  const alreadyPath = document.createElement("span");
  alreadyPath.className = "mono selectable";
  already.appendChild(alreadyLead);
  already.appendChild(document.createTextNode(" "));
  already.appendChild(alreadyName);
  already.appendChild(document.createTextNode(", "));
  already.appendChild(alreadyAt);
  already.appendChild(document.createTextNode(" "));
  already.appendChild(alreadyPath);
  already.appendChild(document.createTextNode("."));
  const after = document.createElement("p");
  after.className = "msub";
  const acts = document.createElement("div");
  acts.className = "mact";
  const back = document.createElement("button");
  back.type = "button";
  back.className = "btn";
  back.addEventListener("click", () => {
    root.hidden = true;
  });
  acts.appendChild(back);
  box.appendChild(path);
  box.appendChild(body);
  box.appendChild(already);
  box.appendChild(after);
  box.appendChild(acts);
  return {
    root,
    heading,
    path,
    body,
    already,
    alreadyLead,
    alreadyName,
    alreadyAt,
    alreadyPath,
    after,
    back,
  };
}

function theRefusalIsDrawn(said) {
  const box = screen.boxes.refusal;
  // **`Keys` U6a: the statement about a missing tool is the screen's own, said
  // once above the list**, so a refusal never composes a second one.
  if (said.theToolsAreMissing === "yes") {
    theApplicationIsAsked();
    return;
  }
  box.heading.textContent = said.heading || "";
  theValueIsDrawnInto(box.path, said.thePickedPath || "");
  box.path.hidden = !said.thePickedPath;
  box.back.textContent = said.back || "";
  box.after.textContent = said.useChooseAReplacement || "";
  box.after.hidden = !said.useChooseAReplacement;

  // **`Keys` F4: one key, one entry, and the message names the row that holds
  // it.** Mockup state `keys-here`.
  box.already.hidden = !said.itIsInTheListAs;
  box.alreadyLead.textContent = said.itIsInTheListAs || "";
  theValueIsDrawnInto(box.alreadyName, said.theOtherName || "");
  box.alreadyAt.textContent = said.at || "";
  theValueIsDrawnInto(box.alreadyPath, said.theOtherPath || "");

  // **The one failure shape, in the box.** The mockup draws `.rdfail` here,
  // which is one of the five prefixes this application has one shape in place
  // of.
  const shape = theFailureShape({
    sentence: said.whatIsWrong,
    whatToDoAboutIt: said.andWhatToDo || said.nothingHasChanged,
    room: EXPANDED,
    values: theTwoFingerprints(said),
    controls: [],
  });
  box.body.replaceChildren(shape);
  box.root.hidden = false;
}

// **`Design/Keys`: a good key with the wrong fingerprint names both**, because
// on this one screen the fingerprint is the only thing that can tell two keys
// apart. They are values beside the sentence and never inside it (X7).
function theTwoFingerprints(said) {
  const out = [];
  if (said.itHolds) out.push([said.itHolds, said.theRowsFingerprint || ""]);
  if (said.youPointedAt) out.push([said.youPointedAt, said.thePickedFingerprint || ""]);
  return out;
}

function theSameNameBox() {
  const { root, box, heading } = aBox();
  const headingName = document.createElement("span");
  heading.appendChild(document.createTextNode(""));
  const said = document.createElement("p");
  said.className = "msub";
  const saidLead = document.createElement("span");
  const saidFingerprint = document.createElement("span");
  saidFingerprint.className = "mono ky-fpv selectable";
  const saidTail = document.createElement("span");
  said.appendChild(saidLead);
  said.appendChild(document.createTextNode(" "));
  said.appendChild(saidFingerprint);
  said.appendChild(document.createTextNode(". "));
  said.appendChild(saidTail);
  const acts = document.createElement("div");
  acts.className = "mact";
  const done = document.createElement("button");
  done.type = "button";
  done.className = "btn";
  done.addEventListener("click", () => {
    root.hidden = true;
  });
  acts.appendChild(done);
  box.appendChild(said);
  box.appendChild(acts);
  return { root, heading, headingName, saidLead, saidFingerprint, saidTail, done };
}

function theSameNameIsSaid(said) {
  const box = screen.boxes.sameName;
  box.heading.replaceChildren();
  const words = document.createElement("span");
  words.textContent = said.anotherKeyIsAlreadyCalled || "";
  const named = document.createElement("span");
  theValueIsDrawnInto(named, said.theName || "");
  box.heading.appendChild(words);
  box.heading.appendChild(document.createTextNode(" "));
  box.heading.appendChild(named);
  box.saidLead.textContent = said.theOneAlreadyInTheListHolds || "";
  box.saidFingerprint.textContent = said.theOtherFingerprint || "";
  box.saidTail.textContent = said.bothAreKept || "";
  box.done.textContent = said.done || "";
  box.root.hidden = false;
}

// ── Taking a key away ───────────────────────────────────────────────────────

function theTakingAwayBox() {
  const { root, box, heading } = aBox();
  const said = document.createElement("p");
  said.className = "msub";
  const servers = document.createElement("p");
  servers.className = "mlist";
  const limit = document.createElement("p");
  limit.className = "msub";
  const limitFirst = document.createElement("span");
  const limitSecond = document.createElement("span");
  limit.appendChild(limitFirst);
  limit.appendChild(document.createTextNode(" "));
  limit.appendChild(limitSecond);
  const unread = document.createElement("p");
  unread.className = "msub";
  const filesLead = document.createElement("p");
  filesLead.className = "msub";
  const files = document.createElement("p");
  files.className = "msub mono ky-files";
  const acts = document.createElement("div");
  acts.className = "mact";
  const cancel = document.createElement("button");
  cancel.type = "button";
  cancel.className = "btn quiet";
  cancel.addEventListener("click", () => {
    root.hidden = true;
  });
  const go = document.createElement("button");
  go.type = "button";
  go.className = "btn";
  const done = document.createElement("button");
  done.type = "button";
  done.className = "btn";
  done.hidden = true;
  done.addEventListener("click", () => {
    root.hidden = true;
  });
  acts.appendChild(cancel);
  acts.appendChild(go);
  acts.appendChild(done);
  // Where what the act came to is said.
  const outcome = document.createElement("div");
  outcome.hidden = true;
  box.appendChild(outcome);
  box.appendChild(said);
  box.appendChild(servers);
  box.appendChild(limit);
  box.appendChild(unread);
  box.appendChild(filesLead);
  box.appendChild(files);
  box.appendChild(acts);
  return {
    root,
    heading,
    said,
    servers,
    limit,
    limitFirst,
    limitSecond,
    unread,
    filesLead,
    files,
    cancel,
    go,
    done,
    outcome,
  };
}

/// **What a delete or a remove came to** — `Keys` W9. Nothing to say closes the
/// box; anything to say is said in it, each file left behind by its full path.
function whatTakingAwayCameToIsDrawn(said) {
  const box = screen.boxes.takingAway;
  const lines = [];
  if (said.theKeyWasNotDeleted) lines.push([said.theKeyWasNotDeleted]);
  const many = Number(said.leftBehindCount || 0);
  for (let at = 0; at < many; at += 1) {
    lines.push([said.thisFileWasNotDeleted, said[field("leftBehind", at)]]);
  }
  if (said.thePassphraseWasNotCleared) lines.push([said.thePassphraseWasNotCleared]);
  if (!lines.length) {
    box.root.hidden = true;
    return;
  }
  box.outcome.replaceChildren();
  for (const [words, path] of lines) {
    const line = document.createElement("p");
    line.className = "mwarn";
    const said = document.createElement("span");
    said.textContent = words || "";
    line.appendChild(said);
    if (path) {
      line.appendChild(document.createTextNode(" "));
      const named = document.createElement("span");
      named.className = "mono selectable";
      theValueIsDrawnInto(named, path);
      line.appendChild(named);
    }
    box.outcome.appendChild(line);
  }
  box.outcome.hidden = false;
  // **The box now says what happened and nothing else**; what the act would
  // cost has been said already.
  if (box.root.hidden) box.heading.textContent = "";
  for (const part of [box.said, box.servers, box.limit, box.unread, box.filesLead, box.files]) {
    part.hidden = true;
  }
  box.go.hidden = true;
  box.cancel.hidden = true;
  box.done.textContent = said.done || "";
  box.done.hidden = false;
  box.root.hidden = false;
}

function whatTakingAwayCostsIsDrawn(said) {
  // **`Design/Keys § Taking a key away`: Delete asks; Remove does not.**
  // Removing destroys nothing, so a remove that strands no server and read its
  // servers cleanly is taken at once rather than through a box asking about
  // nothing. The application decides it; this draws what it was told.
  if (said.itAsks === "no") {
    rowNotes.clear();
    shell.core.invoke("a_key_is_taken_away", { row: said.row });
    return;
  }
  const box = screen.boxes.takingAway;
  box.heading.textContent = said.heading || "";
  box.cancel.textContent = said.cancel || "";
  box.go.textContent = said.theControl || "";
  box.go.className = said.managed === "yes" ? "btn danger" : "btn";
  box.outcome.replaceChildren();
  box.outcome.hidden = true;
  box.go.hidden = false;
  box.go.disabled = false;
  box.cancel.hidden = false;
  box.done.hidden = true;
  // **The box stays until the act answers**, so what it came to is said in it
  // (step 10a round 1, fix-here item 18).
  box.go.onclick = () => {
    box.go.disabled = true;
    rowNotes.clear();
    shell.core.invoke("a_key_is_taken_away", { row: said.row });
  };

  // **The servers are named before the confirmation, and the sentence after
  // them is the honest one** — `Design/Keys § Taking a key away`.
  const many = Number(said.usedByCount || 0);
  box.said.textContent = said.theyConnectWithIt || "";
  box.said.hidden = !said.theyConnectWithIt;
  box.servers.replaceChildren();
  box.servers.hidden = !many;
  for (let at = 0; at < many; at += 1) {
    const badge = said[field("usedBy", at, "Badge")];
    const called = said[field("usedBy", at)];
    if (badge) box.servers.appendChild(theBadge(badge));
    const named = document.createElement("span");
    theValueIsDrawnInto(named, called || "");
    box.servers.appendChild(named);
  }
  box.limit.hidden = !said.thatCoversWhatSmoozyKnows;
  box.limitFirst.textContent = said.thatCoversWhatSmoozyKnows || "";
  box.limitSecond.textContent = said.otherToolsAndComputers || "";
  box.unread.textContent = said.couldNotCheckTheSavedServers || "";
  box.unread.hidden = !said.couldNotCheckTheSavedServers;

  // **`Keys` W6: the confirmation names the file set by full path.**
  const fileCount = Number(said.fileCount || 0);
  box.filesLead.textContent = said.theseFilesAreDeleted || "";
  box.filesLead.hidden = !fileCount;
  box.files.replaceChildren();
  box.files.hidden = !fileCount;
  for (let at = 0; at < fileCount; at += 1) {
    const path = said[field("file", at)];
    const named = document.createElement("span");
    named.className = "ky-file selectable";
    theValueIsDrawnInto(named, path || "");
    box.files.appendChild(named);
  }
  box.root.hidden = false;
}

// ── The answer to the act the user just took ────────────────────────────────

whenTold(AN_ANSWER, (delivery) => {
  if (!screen) return;
  const said = delivery.said;
  switch (said.answer) {
    case "ask-again":
      theApplicationIsAsked();
      break;
    case "the-key-types":
      theTypesAreDrawn(said);
      break;
    case "a-key-was-made":
      theMadeKeyIsDrawn(said);
      break;
    case "the-key-was-not-made":
      theFailedGenerationIsDrawn(said);
      break;
    case "the-make-was-refused":
      theRefusedMakeIsDrawn(said);
      break;
    case "a-point-was-refused":
      theRefusalIsDrawn(said);
      break;
    case "the-same-name":
      theSameNameIsSaid(said);
      break;
    case "taking-a-key-away":
      whatTakingAwayCostsIsDrawn(said);
      break;
    case "copied":
      theCopyIsMade(said);
      break;
    case "copy-unavailable":
      theCopyIsRefused(said);
      break;
    case "taken-away":
      whatTakingAwayCameToIsDrawn(said);
      break;
    case "not-cleared":
      rowNotes.set(said.row, said.thePassphraseWasNotCleared || "");
      break;
    default:
      break;
  }
});

/// **The act ended without a key, so *Make it* comes back.**
///
/// **Every ending calls it and not only the good one** (the close check's
/// repair, 2026-09-17): the key was made, the generation failed, the press was
/// refused, the dialog was cancelled, or the screen was put away. ~~Only an
/// answer brought it back~~, so a press the application refused in silence left
/// the control dead and marked underway for the rest of the run.
function theMakeControlComesBack() {
  if (!screen) return;
  const box = screen.boxes.newKey;
  box.go.disabled = false;
  theMarkComesOffAControl(box.go, box.goLabel || "");
}

function theMadeKeyIsDrawn(said) {
  theMakeControlComesBack();
  const box = screen.boxes.newKey;
  box.root.hidden = false;
  box.form.hidden = true;
  box.acts.hidden = true;
  box.failed.hidden = true;
  box.made.hidden = false;
  box.done.hidden = false;
  box.madeWords.textContent = said.theKeyIsIn || "";
  theValueIsDrawnInto(box.madePath, said.thePath || "");
  box.madePath.classList.add("mono");
  box.madeCopiedSecond.textContent = said.pasteItAtTheProvider || "";
  box.madeNowhere.textContent = said.nowhereToKeepThem || "";
  box.madeNowhere.hidden = !said.nowhereToKeepThem;
  box.doneControl.textContent = said.done || "";
  box.madeCopiedFirst.textContent = "";
  box.madeKey.replaceChildren();
  // **The dialog ends on the copy**, because pasting the public half at a
  // provider is the reason the key was made at all — **and it says *copied*
  // only where the copy said it worked** (step 10a round 1, fix-here item 25).
  theClipboardTakes(said.theContents || "").then((copied) => {
    if (copied) {
      box.madeCopiedFirst.textContent = said.itsPublicKeyIsCopied || "";
      return;
    }
    box.madeCopiedFirst.textContent = said.thePublicKeyWasNotCopied || "";
    box.madeKey.appendChild(theKeyToCopyByHand(said));
  });
}

function theFailedGenerationIsDrawn(said) {
  theMakeControlComesBack();
  const box = screen.boxes.newKey;
  box.root.hidden = false;
  box.failed.hidden = false;
  box.failed.replaceChildren(
    theFailureShape({
      sentence: said.theKeyCouldNotBeMade,
      whatToDoAboutIt: said.whyNot,
      room: EXPANDED,
      controls: [],
    }),
  );
}

/// **The press was refused before anything was touched** — a name over the
/// dialog's bound, a passphrase over it, or no type chosen (step 10a round 1,
/// fix-here item 32, repaired 2026-09-17). **It is drawn where a failed
/// generation is drawn, in the same shape**, and the form stays where it is so
/// the user can correct what they entered and press again.
function theRefusedMakeIsDrawn(said) {
  theMakeControlComesBack();
  const box = screen.boxes.newKey;
  box.root.hidden = false;
  box.form.hidden = false;
  box.acts.hidden = false;
  box.failed.hidden = false;
  box.failed.replaceChildren(
    theFailureShape({
      sentence: said.theActDidNotLand,
      room: EXPANDED,
      controls: [],
    }),
  );
}

// **`Keys` W7 and `Keys` W7a.** The contents arrived as a value; this puts them where
// the user asked for them.
//
// **The clipboard is the page's because the application binds no clipboard
// service** — none is in the manifest and adding one is a dependency change.
///
/// **It answers whether the copy reported success**, and both ways of copying
/// report it — the one never hides a failure any more (step 10a round 1,
/// fix-here item 25).
function theClipboardTakes(contents) {
  if (!contents) return Promise.resolve(false);
  if (navigator.clipboard && navigator.clipboard.writeText) {
    return navigator.clipboard
      .writeText(contents)
      .then(() => true)
      .catch(() => theClipboardTakesItTheOtherWay(contents));
  }
  return Promise.resolve(theClipboardTakesItTheOtherWay(contents));
}

function theClipboardTakesItTheOtherWay(contents) {
  const held = document.createElement("textarea");
  held.className = "ky-away";
  held.value = contents;
  document.body.appendChild(held);
  held.select();
  let copied = false;
  try {
    copied = document.execCommand("copy") === true;
  } catch (trouble) {
    console.error(trouble);
  }
  held.remove();
  return copied;
}

/// **The public key, shown for the user to copy by hand, with a control that
/// selects all of it** — where the copy did not report success. The screen
/// never claims a copy it cannot vouch for, and the user can still get the key
/// to the provider.
function theKeyToCopyByHand(said) {
  const holder = document.createElement("div");
  holder.className = "ky-byhand";
  const shown = document.createElement("textarea");
  shown.className = "fld mono ky-pubkey";
  shown.readOnly = true;
  shown.value = said.theContents || "";
  const select = document.createElement("button");
  select.type = "button";
  select.className = "mini";
  select.textContent = said.selectTheWholeKey || "";
  select.addEventListener("click", () => {
    shown.focus();
    shown.select();
  });
  holder.appendChild(shown);
  holder.appendChild(select);
  return holder;
}

function theCopyIsMade(said) {
  theClipboardTakes(said.theContents || "").then((copied) => {
    if (!copied) {
      theSaidOnceOnTheFoot([said.thePublicKeyWasNotCopied]);
      screen.foot.appendChild(theKeyToCopyByHand(said));
      return;
    }
    theSaidOnceOnTheFoot(
      said.theComparisonCouldNotBeRun
        ? [said.thePublicKeyIsCopied, said.theComparisonCouldNotBeRun]
        : [said.thePublicKeyIsCopied],
    );
  });
}

function theCopyIsRefused(said) {
  theSaidOnceOnTheFoot([said.whyNot]);
}

// **What the copy came to, on the screen's own foot.** It is a statement about
// an act the user just took and it goes where the screen already speaks, rather
// than raising a box for something that worked.
function theSaidOnceOnTheFoot(sentences) {
  screen.foot.replaceChildren();
  for (const sentence of sentences) {
    if (!sentence) continue;
    const said = document.createElement("span");
    said.textContent = sentence;
    screen.foot.appendChild(said);
  }
  screen.foot.hidden = false;
}

// ── The pick's own ending ───────────────────────────────────────────────────

function thePickEnds(chosen) {
  const asking = picking;
  picking = null;
  if (asking && asking.whenItEnds) asking.whenItEnds(chosen);
  theApplicationIsAsked();
}

// ── `Keys` S14's box, over whatever the user is looking at ──────────────────

let theBox = null;

function thePassphraseBox() {
  if (theBox) return theBox;
  const main = document.querySelector(".main");
  if (!main) return null;
  const { root, box, heading } = aBox();
  root.classList.add("top");
  const headingWords = document.createElement("span");
  const headingName = document.createElement("span");
  heading.appendChild(headingWords);
  heading.appendChild(document.createTextNode(" "));
  heading.appendChild(headingName);

  const server = document.createElement("p");
  server.className = "msub";
  const serverWords = document.createElement("span");
  const serverName = document.createElement("b");
  server.appendChild(serverWords);
  server.appendChild(document.createTextNode(" "));
  server.appendChild(serverName);
  server.appendChild(document.createTextNode("."));

  // **`Security and licence` S34's one sentence, said once for the life of the
  // installation**, under the line naming the server — `Design/Keys § On a
  // computer with nowhere to keep a passphrase`, mockup state `keys-nostore`.
  const nowhere = document.createElement("p");
  nowhere.className = "msub";
  nowhere.hidden = true;

  // **`Keys` S19's line, in three sentences.**
  const again = document.createElement("p");
  again.className = "mwarn";
  again.hidden = true;
  const againFirst = document.createElement("b");
  const againSecond = document.createElement("span");
  again.appendChild(againFirst);
  again.appendChild(document.createTextNode(" "));
  again.appendChild(againSecond);

  const form = document.createElement("div");
  form.className = "frm";
  const label = document.createElement("label");
  label.htmlFor = "ky-typed";
  const field = document.createElement("input");
  field.type = "password";
  field.id = "ky-typed";
  field.className = "fld wide mono";
  form.appendChild(label);
  form.appendChild(field);

  const acts = document.createElement("div");
  acts.className = "mact";
  const cancel = document.createElement("button");
  cancel.type = "button";
  cancel.className = "btn quiet";
  const connect = document.createElement("button");
  connect.type = "button";
  connect.className = "btn";
  acts.appendChild(cancel);
  acts.appendChild(connect);

  box.appendChild(server);
  box.appendChild(nowhere);
  box.appendChild(again);
  box.appendChild(form);
  box.appendChild(acts);
  main.appendChild(root);

  theBox = {
    root,
    heading,
    headingWords,
    headingName,
    server,
    serverWords,
    serverName,
    nowhere,
    again,
    againFirst,
    againSecond,
    label,
    field,
    cancel,
    connect,
  };
  return theBox;
}

whenTold(THE_PASSPHRASE_ASK, (delivery) => {
  const said = delivery.said;
  const box = thePassphraseBox();
  if (!box) return;
  box.headingWords.textContent = said.passphraseFor || "";
  theValueIsDrawnInto(box.headingName, said.theKeyNamed || "");
  box.headingName.classList.toggle(
    "mono",
    said.theKeyIsNamedByItsFingerprint === "yes",
  );
  box.serverWords.textContent = said.connectingTo || "";
  theValueIsDrawnInto(box.serverName, said.theServerNamed || "");
  box.server.hidden = !said.theServerNamed;
  box.nowhere.textContent = said.nowhereToKeepThem || "";
  box.nowhere.hidden = !said.nowhereToKeepThem;
  box.again.hidden = !said.theRememberedPassphraseNoLongerOpensIt;
  box.againFirst.textContent = said.theRememberedPassphraseNoLongerOpensIt || "";
  box.againSecond.textContent = said.smoozyHasForgottenIt || "";
  box.label.textContent = said.thePassphraseField || "";
  box.cancel.textContent = said.cancel || "";
  box.connect.textContent = said.connect || "";
  box.field.value = "";
  // **`Keys` S17: the two outcomes are a value or nothing.** Nothing ends the
  // act — no attempt is made and nothing is put to the store — **and the connect
  // is reported as one that did not happen**. A value opens the connect the box
  // was raised for, carrying what was typed to its one attempt: *"the box is the
  // connection's"* (`Design/Keys § Where the passphrase is asked for`).
  const server = said.server || "";
  box.cancel.onclick = () => {
    box.field.value = "";
    box.root.hidden = true;
    if (server) shell.core.invoke("the_passphrase_box_was_cancelled", { server });
  };
  box.connect.onclick = () => {
    const typed = box.field.value;
    // **With nothing typed, *Connect* is `Keys` S17's *nothing*, exactly as *Cancel*
    // is**: the box closes and the connect is reported as one that did not
    // happen. Until 2026-09-19 it closed the box and said nothing; the Gemini
    // catch-up of step 10a, Area 5 (F-3).
    if (!typed) {
      box.cancel.onclick();
      return;
    }
    box.field.value = "";
    box.root.hidden = true;
    if (!server) return;
    shell.core.invoke("the_passphrase_box_was_answered", {
      row: said.row,
      passphrase: typed,
      server,
    });
  };
  // **`Design/Whole app.md`: Return presses the box's own affirmative
  // control**, and *Connect* is not a red one. It goes through the same
  // handler, so an empty field is `Keys` S17's *nothing* here too. Until 2026-09-19
  // Return did nothing in this box; the Gemini catch-up of step 10a, Area 5
  // (F-4).
  box.field.onkeydown = (event) => {
    if (event.key !== "Enter") return;
    event.preventDefault();
    box.connect.onclick();
  };
  box.root.hidden = false;
  box.field.focus();
});

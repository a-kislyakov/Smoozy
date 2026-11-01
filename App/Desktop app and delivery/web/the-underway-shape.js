// **The underway shape, on the page** — Desktop app and delivery §16.5 and
// §16.7, and Preparation/Design/Whole app.md's *Anything that isn't instant
// shows that it's underway*.
//
// **READ THIS BEFORE DRAWING AN UNDERWAY MARK ANYWHERE.** It is the page half
// of the rules at the heads of `src/presentation/the_underway_shape.rs` and
// `src/presentation/the_repairing_server.rs`, and the eleven screens after this
// piece draw inside it rather than beside it.
//
// ## Three shapes, and the caller does not pick
//
// The Rust side picks — `the_shape` there is Design's *what the user can still
// do decides which* performed rather than remembered — and the event carries
// the answer in `mark`. **A page module that chose a shape of its own would be
// making a judgement Design says is meant to be made without one.**
//
// ## The two rules that are easy to lose on this side, and both are Design's
//
// - **On the control: it keeps its place and its width, loses its label to a
//   turning mark, and is NOT dimmed.** The label stays in the markup — that is
//   what holds the width — with `visibility: hidden` on `.uwl`, which is the
//   drawing's own rule and is in `the-drawing.css`. **Nothing here sets
//   `opacity` and nothing sets `disabled`.** Design, 2026-09-02: *"Not being
//   pressable is carried by the control not answering; a fade on top of that
//   says unavailable twice, and on a filled button it washes the turning mark
//   out to almost nothing."* Not answering is `.uway`'s own
//   `pointer-events: none`, and `aria-disabled` says the same to anything that
//   is not looking at pixels.
// - **One thing never carries two of them.** The Rust side answers with no mark
//   at all for a thing that already carries its own state, so a control that
//   would wear two never reaches this file with one.
//
// ## The countdown, and it is the one exemption in this application
//
// **M30: the countdown is a drawing and is bounded by P10b.** *"It ticks
// against M28's moment, takes no reading, makes no request, posts nothing on
// P6's queue, and ceases to exist when the thing it draws is off screen."*
//
// **This is the one place in Smoozy where P10b's exemption is used**, and the
// four things it costs are kept here and nowhere else:
//
//  1. It ticks against a moment the application already published — the
//     `theNextAttemptIsDueAt` the event carried, and nothing else.
//  2. It takes no reading, makes no request and posts nothing.
//  3. **It ceases to exist off screen.** One interval serves every countdown on
//     screen; a countdown whose element has left the document is dropped, and
//     when the last one goes the interval is cleared. There is no timer
//     standing with nothing to draw.
//  4. **Where the moment is absent there is no countdown at all** and the word
//     stands alone, which M30 states as the honest reading of *an attempt is
//     running right now*.
//
// **P10c's hazard rides on it and is not repaired here.** A computer suspended
// between the moment being published and the drawing reading it wakes the
// countdown reading long past due; the repair is driven by attempts ending
// rather than by the clock, so a late attempt is still an attempt.
//
// **`src/surface/the_event_route.rs`'s own test is where this exemption is
// written down on the Rust side**, with its clause, rather than the check being
// deleted.
//
// ## Three things a drawing must never do here
//
// - **Never join a name into a sentence.** X7. The job's name arrives on its
//   own key and stays on its own element.
// - **Never compose a third reading out of two.** M31. The event carries both
//   of the column's contents; this file puts one in the slot Design drew and
//   the other where Design drew it, and builds nothing out of the pair.
// - **Never substitute anything.** Every string here has already been through
//   one of the two routes on the application side (A6, X9). Draw it.
// - **Never write a word of prose into this file.** A6 and X1: the typography
//   point is the only way prose reaches a screen, and the enforcement is
//   structural on both sides. Until 2026-09-12 this file wrote eight strings
//   itself — the starting-again line, the countdown's two forms, *checking what
//   was already sent*, *as of … on …*, *last seen … on …*, *of*, and a
//   *Reconnecting…* fallback — and **one of them broke the em-dash rule by
//   hand**, with an ordinary space before the dash, which is the one rule X3
//   says cannot be held by remembering it. Found at round 1 of the code audit of
//   what is drawn inside the window (F-6); the sweep at `src/surface/mod.rs` is
//   what says so now.
//
// H32: values are content, never markup — `textContent` and `createElement`,
// never `innerHTML`. H33: no inline style; the rules are in
// `the-underway-shape.css` and in the drawing's own lifted sheet.

import { whenTold } from "./event-route.js";
import { theValueIsDrawnInto } from "./the-value-drawn.js";

const shell = window.__TAURI__;

// The kinds, declared beside the service that posts them —
// `SOMETHING_IS_UNDERWAY`, `IT_IS_NO_LONGER_UNDERWAY` and
// `A_SERVER_IS_REPAIRING` in `src/surface/the_underway_shape.rs`. The two sides
// have to agree and nothing holds them together, so copy carefully.
const SOMETHING_IS_UNDERWAY = "underway:something-is-underway";
const IT_IS_NO_LONGER_UNDERWAY = "underway:it-is-no-longer-underway";
const A_SERVER_IS_REPAIRING = "underway:a-server-is-repairing";

// ── The three shapes ────────────────────────────────────────────────────────

/// **Put the underway mark on a control** — Design's *on the control*.
///
/// The control keeps its place and its width, loses its label to a turning
/// mark, and is not dimmed. `label` is the control's own text and stays in the
/// markup to hold the width.
export function theMarkOnAControl(control, label) {
  control.classList.add("uway");
  control.setAttribute("aria-disabled", "true");
  control.replaceChildren();
  const held = document.createElement("span");
  held.className = "uwl";
  held.textContent = label;
  control.appendChild(held);
  const turning = document.createElement("i");
  turning.className = "spin";
  control.appendChild(turning);
  return control;
}

/// **The mark goes and the control comes back.**
export function theMarkComesOffAControl(control, label) {
  control.classList.remove("uway");
  control.removeAttribute("aria-disabled");
  control.replaceChildren();
  control.textContent = label;
  return control;
}

/// **The mark in a row's own status column** — Design's *on the row*: *"The row
/// says how far along in its own status column."*
///
/// `howFarAlong` is `{ staged, expected, takenAt, onTheClock }` or nothing, and
/// `words` carries the prose the event handed over — `of`, `asOf`,
/// `onTheClockWord`.
/// **Nothing is composed where it is absent**: a shape that never becomes a
/// figure is the app saying it cannot, and that shape is itself the admission.
export function theMarkOnARow(howFarAlong, words) {
  const cell = document.createElement("span");
  cell.className = "underway-row";

  // **The bar is a native `<progress>` and not the drawing's `div.bar > i`, and
  // that is H33 rather than a preference.** The mockup fills its bar with
  // `<i style="width:48%">` — one of the 87 inline `style=` attributes piece A
  // recorded as inert here, because the shipped policy is `style-src 'self'`
  // with no inline style anywhere. **A width is a continuous value and there is
  // no data attribute a stylesheet can turn into one**, so the value goes where
  // a value API can carry it: `value` and `max` on the element the platform
  // already has for exactly this. The look is the drawing's, in
  // `the-underway-shape.css`, on the drawing's own tokens.
  const bar = document.createElement("progress");
  bar.className = "underway-bar";
  cell.appendChild(bar);

  const said = document.createElement("p");
  said.className = "rqs";
  if (howFarAlong && howFarAlong.expected && Number(howFarAlong.expected) > 0) {
    const staged = Number(howFarAlong.staged);
    const expected = Number(howFarAlong.expected);
    bar.max = expected;
    bar.value = Math.min(staged, expected);
    // **Two figures and one word, and the word is prose off the event.** They
    // are separate nodes: a figure joined into a sentence is the join X7
    // forbids.
    for (const part of [bytes(staged), (words && words.of) || "", bytes(expected)]) {
      const one = document.createElement("span");
      one.textContent = part;
      said.appendChild(one);
      said.appendChild(document.createTextNode(" "));
    }
    if (howFarAlong.takenAt) {
      // **M3a: presented as of the moment it was taken, on the clock it was
      // taken on.** Never re-drawn as current, and never aged into a newer
      // figure.
      const asOf = document.createElement("span");
      asOf.className = "sub";
      theWordsAndTheMoment(
        asOf,
        (words && words.asOf) || "",
        howFarAlong.takenAt,
        (words && words.onTheClockWord) || "",
        howFarAlong.onTheClock,
      );
      said.appendChild(asOf);
    }
  } else {
    // Design: *a shape that never becomes a figure is the app saying it
    // cannot*, and no words are owed anywhere for it. **A `<progress>` with no
    // value is the platform's own indeterminate bar**, which is that sentence
    // drawn rather than described.
    cell.dataset.itCannotSay = "true";
    const turning = document.createElement("i");
    turning.className = "spin";
    said.appendChild(turning);
  }
  cell.appendChild(said);
  return cell;
}

/// **The mark over a pane** — Design's *over the pane*: *"nothing on that pane
/// can be used until the work finishes. Nothing else on the screen is
/// covered."*
export function theMarkOverAPane() {
  const cover = document.createElement("div");
  cover.className = "underway-pane";
  const turning = document.createElement("i");
  turning.className = "spin";
  cover.appendChild(turning);
  return cover;
}

/// **One underway thing, in whichever of the three shapes the application
/// picked.** The caller does not choose.
export function theUnderwayShape(parts) {
  if (parts.mark === "on-the-control" && parts.control) {
    return theMarkOnAControl(parts.control, parts.label || "");
  }
  if (parts.mark === "over-the-pane") return theMarkOverAPane();
  return theMarkOnARow(parts.howFarAlong, parts.words);
}

/// **A line of words and values, each its own node and never one string.**
///
/// The words arrived as prose through the typography point and the moment and
/// the clock's name are values; joining them here would undo the split the
/// application side made for exactly that reason (X7).
function theWordsAndTheMoment(into, asOf, takenAt, onTheClockWord, onTheClock) {
  const parts = [asOf, theMoment(takenAt), onTheClockWord, onTheClock || ""];
  parts.forEach((part, at) => {
    const one = document.createElement("span");
    // The clock's name, last, is a name read in the order its bytes run (X9).
    if (at === parts.length - 1) theValueIsDrawnInto(one, part);
    else one.textContent = part;
    into.appendChild(one);
    into.appendChild(document.createTextNode(" "));
  });
}

// ── The countdown, and P10b's one exemption ─────────────────────────────────

const ticking = new Set();
let theTick = null;

function drawEveryCountdown() {
  for (const one of Array.from(ticking)) {
    // **It ceases to exist when the thing it draws is off screen** — P10b and
    // M30. An element that has left the document is dropped here.
    if (!one.line.isConnected) {
      ticking.delete(one);
      continue;
    }
    theTimeLeft(one.line, one.dueAt, one.words);
  }
  if (ticking.size === 0 && theTick !== null) {
    clearInterval(theTick);
    theTick = null;
  }
}

/// **A countdown against a moment the application already published, and
/// nothing else** — M30, and P10b's exemption in the one place this
/// application uses it.
///
/// `dueAt` is seconds since the epoch, exactly as `Servers` N10a published it.
/// **This takes no reading, makes no request and posts nothing.**
export function theCountdown(dueAt, words) {
  const line = document.createElement("span");
  line.className = "sub";
  theTimeLeft(line, Number(dueAt), words);
  ticking.add({ line, dueAt: Number(dueAt), words });
  if (theTick === null) {
    theTick = setInterval(drawEveryCountdown, 1000);
  }
  return line;
}

/// **How long is left, drawn as words and a number.**
///
/// **Both forms of the line are prose off the event** and were written into this
/// file until 2026-09-12. The seconds are this drawing's own arithmetic against
/// the moment the application published, which is the whole of M30's exemption.
function theTimeLeft(line, dueAt, words) {
  const left = Math.round(Number(dueAt) - Date.now() / 1000);
  line.replaceChildren();
  // **P10c's hazard, said rather than repaired.** A suspended computer wakes
  // this reading long past due; the repair is driven by attempts ending rather
  // than by the clock, so the honest line is that the attempt is due.
  if (!Number.isFinite(left) || left <= 0) {
    line.textContent = (words && words.now) || "";
    return;
  }
  const said = document.createElement("span");
  said.textContent = (words && words.in) || "";
  line.appendChild(said);
  line.appendChild(document.createTextNode(" "));
  const many = document.createElement("span");
  many.textContent = String(left);
  line.appendChild(many);
}

/// **A moment, with its date and the zone it is drawn in** — M3a.
///
/// **A bare time of day is the one format M3a exists to stop.** A reading taken
/// at 14:32 on a rented machine in another region read as *09:32* beside that
/// machine's own name, which is neither that machine's clock nor labelled as the
/// reader's; and a reading days old still read as a time today. Corrected
/// 2026-09-12 (F-9). **Nothing is converted into the machine's own zone** —
/// nothing here knows its offset, and inventing one would be worse than either.
function theMoment(seconds) {
  const at = new Date(Number(seconds) * 1000);
  return at.toLocaleString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
    timeZoneName: "short",
  });
}

// **A figure and its unit are joined by U+00A0**, so *47 GB* never parts
// across two lines — `Design/Whole app § Typography`. The character's one home
// is `src/typography.rs` (`BETWEEN_A_FIGURE_AND_ITS_UNIT`).
const BETWEEN_A_FIGURE_AND_ITS_UNIT = "\u00A0";

function bytes(count) {
  const steps = ["B", "KB", "MB", "GB", "TB"];
  let at = 0;
  let left = Number(count);
  while (left >= 1000 && at < steps.length - 1) {
    left /= 1000;
    at += 1;
  }
  return `${at === 0 ? left : left.toFixed(1)}${BETWEEN_A_FIGURE_AND_ITS_UNIT}${steps[at]}`;
}

// ── The repairing server ────────────────────────────────────────────────────

/// **A repairing server's row** — M28's three readings drawn, M29's word, M30's
/// countdown, M31's two contents.
///
/// **Which of the two contents occupies the one slot is Design's**, and this is
/// where that choice is made: the mockup puts *Reconnecting…* in the work
/// column with the countdown under it, and the last-known work reading stands
/// beside it with its own moment rather than in the slot. **Nothing here is
/// built out of the pair.**
export function theRepairingServer(parts) {
  const row = document.createElement("div");
  row.className = "svrow repairing";
  row.dataset.serverRow = parts.serverRow || "";

  const main = document.createElement("span");
  main.className = "svmain";
  const name = document.createElement("span");
  name.className = "svname mono";
  // Names read in the order their bytes run (X9, `web/the-value-drawn.js`).
  theValueIsDrawnInto(name, parts.serverRow || "");
  main.appendChild(name);
  row.appendChild(main);

  // The work column. **M29: the word is whether a repair is under way, and it
  // is never `Servers` K6's value** — which travels on the event as supply and
  // is drawn nowhere.
  const work = document.createElement("span");
  work.className = "svwork";
  const said = document.createElement("b");
  // Design's own drawing puts *Reconnecting…* in the slot where both are true,
  // which is the second of the two the event carries.
  said.textContent = parts.theColumnMaySaySecond || parts.theColumnMaySayFirst || "";
  work.appendChild(said);
  // **M30: where the moment is absent the line has no countdown and the word
  // stands alone.**
  if (parts.theNextAttemptIsDueAt) {
    work.appendChild(
      theCountdown(parts.theNextAttemptIsDueAt, {
        in: parts.theTryingAgainIn,
        now: parts.theTryingAgainNow,
      }),
    );
  }
  row.appendChild(work);

  // **M31's other content, with its own moment** — the last-known work
  // reading, which is stale by construction, and saying so is the requirement.
  const stale = document.createElement("span");
  stale.className = "svhw stale";
  if (parts.theLastKnownWork) {
    // **The word is prose and the job's name is a value** — two nodes, never one
    // string (X7).
    const what = document.createElement("span");
    const word = document.createElement("span");
    word.textContent = parts.theColumnMaySayFirst || "";
    what.appendChild(word);
    what.appendChild(document.createTextNode(" "));
    const named = document.createElement("span");
    theValueIsDrawnInto(named, parts.theJobsName || "");
    what.appendChild(named);
    stale.appendChild(what);
    const when = document.createElement("span");
    when.className = "sub";
    theWordsAndTheMoment(
      when,
      parts.theLastSeen || "",
      parts.theLastKnownWorkWasTakenAt,
      parts.theReadingOnTheClock || "",
      parts.theLastKnownWorkClock,
    );
    stale.appendChild(when);
  }
  row.appendChild(stale);

  // The Connect control. **Design: one thing never carries two of them** — a
  // server Smoozy is repairing already says so on its row, so its control keeps
  // its amber, which is state rather than an underway mark, and does not
  // additionally lose its label or spin.
  const acts = document.createElement("span");
  acts.className = "svacts";
  const connect = document.createElement("button");
  connect.type = "button";
  connect.className = "cbtn trying";
  const dot = document.createElement("span");
  dot.className = "cdot";
  connect.appendChild(dot);
  // **The same fallback as the work column above**: with no last-known work
  // the event carries one word only, as the first, and the control would
  // otherwise be an amber dot with no label. The drawing reads *Reconnecting…*
  // here. The Gemini catch-up of step 10a, Area 4 (F-3).
  connect.appendChild(
    document.createTextNode(
      parts.theColumnMaySaySecond || parts.theColumnMaySayFirst || "",
    ),
  );
  acts.appendChild(connect);
  row.appendChild(acts);

  return row;
}

// ── The plumbing's own drawing, which is this piece's proof ─────────────────
//
// **A later piece takes this pane.** Nothing else in the application refers to
// this block. What is worth keeping when it goes is the shape above.

const drawn = new Map();

function where() {
  let held = document.getElementById("the-underway");
  if (held) return held;
  const pane = document.getElementById("pane");
  if (!pane) return null;
  held = document.createElement("div");
  held.id = "the-underway";
  held.className = "underway";
  pane.appendChild(held);
  return held;
}

whenTold(SOMETHING_IS_UNDERWAY, (delivery) => {
  const held = where();
  if (!held) return;
  const said = delivery.said || {};
  const shape = theUnderwayShape({
    mark: said.mark,
    howFarAlong: said.staged
      ? {
          staged: said.staged,
          expected: said.expected,
          takenAt: said.takenAt,
          onTheClock: said.onTheClock,
        }
      : null,
    words: {
      of: said.theOfWord,
      asOf: said.theReadingAsOf,
      onTheClockWord: said.theReadingOnTheClock,
    },
  });
  shape.dataset.whatItIsOn = said.whatItIsOn || "";
  if (said.startedOver) {
    // **`Server protocol and transfers` G4a: the count falling to zero is never
    // left to stand as the whole report.** The mark is drawn beside the count.
    // **Two sentences, two strings, both prose off the event** — X10, and the
    // dash this file used to write by hand is gone with them.
    const mark = document.createElement("p");
    mark.className = "rqs";
    mark.appendChild(document.createTextNode(said.startingAgain || ""));
    mark.appendChild(document.createTextNode(" "));
    mark.appendChild(document.createTextNode(said.startedOver));
    shape.appendChild(mark);
  }
  if (said.verifyingAPrefixOf) {
    // **`Server protocol and transfers` G4b: an unmoving count is never the
    // whole report on a resume.**
    const mark = document.createElement("p");
    mark.className = "rqs";
    mark.textContent = said.theCheckingLine || "";
    shape.appendChild(mark);
  }
  const already = drawn.get(said.whatItIsOn);
  if (already) {
    already.replaceWith(shape);
  } else {
    held.appendChild(shape);
  }
  drawn.set(said.whatItIsOn, shape);
});

whenTold(IT_IS_NO_LONGER_UNDERWAY, (delivery) => {
  const said = delivery.said || {};
  const shape = drawn.get(said.whatItIsOn);
  if (!shape) return;
  shape.remove();
  drawn.delete(said.whatItIsOn);
});

whenTold(A_SERVER_IS_REPAIRING, (delivery) => {
  const held = where();
  if (!held) return;
  const said = delivery.said || {};
  const row = theRepairingServer(said);
  const already = drawn.get(said.serverRow);
  if (already) {
    already.replaceWith(row);
  } else {
    held.appendChild(row);
  }
  drawn.set(said.serverRow, row);
});

// The page asks; the answers arrive as events. A command is the page asking.
export function whatIsUnderwayAgainst(subjectKind, subject) {
  return shell.core.invoke("what_is_underway_against", { subjectKind, subject });
}

// **The one shape a failure is drawn in, on the page** — Desktop app and
// delivery §16, and Preparation/Design/Whole app.md's *The failure shape*.
//
// **READ THIS BEFORE DRAWING A FAILURE ANYWHERE. It is the page half of the
// rule at the head of `src/presentation/mod.rs`**, and pieces D, E and F and
// the eleven screens after them draw inside it rather than beside it.
//
// ## Why there is one builder and not eleven copies
//
// Design states the shape once — *"A failure has one shape wherever it
// attaches, and this is where it is stated"* — and the mockup then draws it
// **eleven times across five modules with five class prefixes**, because a
// mockup is one static file with no components in it. **Copying that gives five
// shapes and Design's sentence stops being true the day the second one
// drifts.** So there is one builder, `theFailureShape`, and a screen hands it
// parts.
//
// ## The order is the application's and this file does not compose one
//
// A red dot, one plain sentence saying what did not happen, then — only where
// Smoozy actually knows — a second sentence, then the controls: **whatever act
// would try again, the screen's own way past, the raw text, Report a problem…,
// the cross.** Nothing else, in that order.
//
// **The order arrives on the event and this file draws the list it is handed.**
// Until 2026-09-12 the application composed the controls in Design's order, a
// test asserted that order, the event then **dropped them**, and this file
// composed the order a second time and supplied the shape's own two labels
// itself. The two agreed; nothing held them together, and the day somebody
// edited one list the test would still have passed — which is *A failure has one
// shape wherever it attaches* quietly stopping being true. **This is the same
// fact kept in two places, the fault the head of `src/presentation/mod.rs` names
// as the one this Build stage keeps meeting.** Found at round 1 of the code
// audit of what is drawn inside the window (F-13).
//
// ## Three things a drawing must never do here
//
// - **Never join a name into the sentence.** X7: the typography point never
//   touches a job's name, a server's name, a path, a host or a port. The
//   sentence arrives already through the point and every value arrives by the
//   verbatim route; **joining the two on this side undoes the split the Rust
//   side made.** Put a name in `parts.values`.
// - **Never lift a line out of a stream and set it beside the sentence.** M13b,
//   and **the mockup does exactly that at lines 5476 and 5514** — a last line
//   in a `.log .last` span directly under the plain sentence. *"The last line is
//   a sampling line, and setting it beside a plain sentence about a failure
//   asserts a cause by placement that no record carries."* The clause wins
//   (Rule 10). **There is no place in this builder for one**, and adding one is
//   not a drawing decision.
// - **Never substitute anything.** Every string here has already been through
//   one of the two routes on the application side (A6, X9). Draw it.
// - **Never write a word of prose into this file.** A6 and X1: the typography
//   point is the only way prose reaches a screen, and the enforcement is
//   structural on both sides. The application half is `Prose` and `Verbatim`;
//   this half is the sweep at `src/surface/mod.rs`, which fails on a quoted
//   literal reaching the text of an element or an accessible name. Added
//   2026-09-12 (F-7), after this file had been writing *Report a problem…* and
//   *Clear this* itself.
//
// H32: values are content, never markup — `textContent` and `createElement`,
// never `innerHTML`. H33: no inline style; the rules are in
// `the-failure-shape.css`.
//
// The classes are the drawing's own wherever the drawing has one — `qfail`,
// `fdot`, `qf2`, `rp-ask`, `svfail`, and `stfail` for a step's line (from
// 2026-10-25), are all lifted in `the-drawing.css`. What
// this file's own stylesheet adds is only what one shape needs that eleven
// per-screen copies did not.

import { theValueIsDrawnInto } from "./the-value-drawn.js";

const shell = window.__TAURI__;

// **This file draws no failure of its own and listens for none.** Until the
// twenty-ninth notes' piece E it also drew every standing failure the
// application posted in a strip at the window's foot — piece C's test drawing,
// which no later piece took over. `Specs/Whole app`: a failure *"appears on the
// thing that failed"*, and *"there is no separate place failures are
// collected"*. The one subject that reached the strip, a server row, draws its
// failures on the row (`web/servers.js`), so the strip went.

// How much room the surface has. The Rust side's `Room`, in the same two
// values.
export const EXPANDED = "expanded";
export const A_COLLAPSED_ROW = "a-collapsed-row";

/// **Build the one failure shape.**
///
/// `parts` is:
///
///   sentence            the plain sentence, through the typography point
///   whatToDoAboutIt     the second sentence, or nothing — **only where Smoozy
///                       actually knows**
///   kind                "condition" or "ending", from the record (M4)
///   subject, subjectKind, identityOrMark, key
///   theSentenceForTheReport   the same sentence before the substitutions —
///                       M32's captured string, which is also what
///                       `Reporting a problem` X4 hands the report entry point
///   at                  the moment, seconds since the epoch
///   values              [[name, value], …] — names, paths, hosts, ports:
///                       everything on the verbatim route
///   controls            [{ is, label, act }] — **the whole control list, in
///                       the shape's own order, exactly as the application
///                       composed it.** `is` is the token the application wrote
///                       and this file routes on; it never decides an order
///   room                EXPANDED or A_COLLAPSED_ROW
///   onAct               called with an act's `act` token when it is pressed
///   onClear             called when the cross is pressed
///   onReport            called with the handover when *Report a problem…* is
///                       pressed — **absent, it opens the report box**, which is
///                       what every screen wants (`theReportBoxOpensFrom`)
///   onAStep             true where the shape sits under one step of a list —
///                       see [`theLineOnAStep`]
export function theFailureShape(parts) {
  const collapsed = parts.room === A_COLLAPSED_ROW;
  if (parts.onAStep && !collapsed && theLineHoldsIt(parts)) return theLineOnAStep(parts);
  const shape = document.createElement("div");
  // The drawing's own classes: `qfail` is *the whole-app failure shape, on a
  // job's entry* and `svfail` is *the whole-app failure shape, collapsed onto a
  // list row*. `failure` is this application's own hook for the one shape.
  shape.className = collapsed ? "svfail failure failure-row" : "qfail failure";
  shape.dataset.failureKind = parts.kind || "";
  shape.dataset.identityOrMark = parts.identityOrMark || "";

  // The red dot and the plain sentence. **The dot is in the text rather than in
  // a column of its own** — the drawing's own rule, and a bold opening clause is
  // not a second column.
  const said = document.createElement("p");
  said.className = "failure-said";
  const dot = document.createElement("span");
  dot.className = "fdot";
  said.appendChild(dot);
  const sentence = document.createElement("b");
  sentence.className = collapsed ? "" : "selectable";
  sentence.textContent = parts.sentence || "";
  said.appendChild(sentence);
  shape.appendChild(said);

  // **On a box, the cross sits at the top right**, first in the first line
  // and floated there, as every box that closes has it —
  // `Design/Whole app § The failure shape`, added 2026-11-13 (the Creator's
  // note 4 on the test copy of 2026-11-12: *"the closing cross is in the
  // bottom corner, which is just bonkers"*). Until then it was last in the
  // control row, alone on a line of its own when nothing else was there. **On
  // a collapsed row the line is the box, and the cross still ends it.**
  const theCross = collapsed ? null : (parts.controls || []).find((control) => control.is === "cross");
  if (theCross) {
    const cross = aCross(theCross, parts);
    cross.classList.add("failure-clear-top");
    said.insertBefore(cross, said.firstChild);
  }

  // The second sentence, indented to the first so the two read as one
  // statement. **Only where Smoozy actually knows**, and never on a collapsed
  // row, which has no room for it.
  if (!collapsed && parts.whatToDoAboutIt) {
    const second = document.createElement("p");
    second.className = "qf2 failure-second";
    const words = document.createElement("b");
    words.className = "selectable";
    words.textContent = parts.whatToDoAboutIt;
    second.appendChild(words);
    shape.appendChild(second);
  }

  // Values, each under its own name and each exactly as Smoozy was given it.
  // **Nothing here is joined into a sentence** — X7.
  if (!collapsed && parts.values && parts.values.length) {
    const list = document.createElement("dl");
    list.className = "failure-values";
    for (const [name, value] of parts.values) {
      const term = document.createElement("dt");
      term.textContent = name;
      const said = document.createElement("dd");
      said.className = "mono selectable";
      // **Read in the order its bytes run** — X9, `web/the-value-drawn.js`. A
      // path a failure names is the case the ruling was made for.
      theValueIsDrawnInto(said, value);
      list.appendChild(term);
      list.appendChild(said);
    }
    shape.appendChild(list);
  }

  const acts = document.createElement("div");
  acts.className = "failure-acts";

  // **The list as it was handed over, in the order it arrived.** Nothing here
  // picks an order, adds a control the application did not compose, or writes a
  // label. **The cross is on a condition and never on an ending, and
  // *Report a problem…* is on every expanded presentation and on no collapsed
  // row** — both are decided where the shape is composed.
  for (const control of parts.controls || []) {
    if (control.is === "cross") {
      // Drawn at the top right on a box, above; here only on a row.
      if (!theCross) acts.appendChild(aCross(control, parts));
      continue;
    }
    if (control.is === "report") {
      // **Quiet and last before the cross** — M33, and the retry stays the loud
      // one, because a connection dropping when the user shut their laptop is
      // the commonest failure in the app and is nobody's bug.
      const asking = document.createElement("div");
      asking.className = "rp-ask";
      const report = document.createElement("button");
      report.type = "button";
      report.textContent = control.label;
      report.addEventListener("click", () => {
        // **`Reporting a problem` X4's closed handover, the five things.** The
        // sentence is the one captured before the substitutions and is never
        // recomposed here. **It goes to the report box unless the screen asked
        // for it itself** — the supply is the shape's, so every failure drawn
        // through it opens the box by construction (M33).
        const opens = parts.onReport || theReportBoxOpensFrom;
        opens({
          kind: parts.kind,
          identityOrMark: parts.identityOrMark,
          durableSubject: parts.subject,
          subjectKind: parts.subjectKind,
          thePresentedSentence: parts.theSentenceForTheReport,
          at: parts.at,
        });
      });
      asking.appendChild(report);
      acts.appendChild(asking);
      continue;
    }
    const one = document.createElement("button");
    one.type = "button";
    // **Every control in the shape is the drawing's small button** — its
    // `.qfail .rdrow .mini`, *Retry* and *Check again* alike — so *Try again*
    // is never a full-size filled button beside small ones (UI consistency,
    // finding 40).
    one.className = control.is === "raw-text" ? "mini failure-words" : "mini";
    one.textContent = control.label;
    one.addEventListener("click", () => {
      if (parts.onAct) parts.onAct(control.act);
    });
    acts.appendChild(one);
  }

  if (acts.childElementCount) shape.appendChild(acts);
  return shape;
}

/// **The cross that clears a failure** — a glyph rather than a word, so
/// nothing here is prose; its name is the label the application composed.
function aCross(control, parts) {
  const cross = document.createElement("button");
  cross.type = "button";
  cross.className = "failure-clear";
  cross.setAttribute("aria-label", control.label);
  cross.textContent = "\u00d7";
  cross.addEventListener("click", () => {
    if (parts.onClear) parts.onClear();
  });
  return cross;
}

/// **Whether the one line under a step holds everything this failure has** —
/// the sentence and the raw text, and nothing else. A second sentence, values,
/// a retry, *Report a problem…* or a cross have no place on that line, so a
/// failure carrying any of them keeps the whole block. Nothing is dropped to
/// make it fit.
function theLineHoldsIt(parts) {
  if (parts.whatToDoAboutIt) return false;
  if (parts.values && parts.values.length) return false;
  return (parts.controls || []).every((control) => control.is === "raw-text");
}

/// **The same shape on a step of a list, as the drawing draws it there** —
/// `Design/Server Setup § The step that failed`: *"Under the failed step sits
/// one line on the sunk ground: the red dot, the sentence in bold, and* What
/// the server said *as a link at the end of the same line, not a button on a
/// line of its own."* The classes are the drawing's own `.stfail` (lifted in
/// `the-drawing.css`), its `.fdot` and its link.
///
/// **It is still the one shape**: the same dot, sentence and raw-text control,
/// in the same order, built here so no page draws its own copy of the
/// drawing's step line. Added 2026-10-25, the eighteenth notes, note 4: the
/// Creator's stopped setup drew the step's failure as the general block with a
/// pill, which the drawing never does on a step.
function theLineOnAStep(parts) {
  const line = document.createElement("p");
  line.className = "stfail failure";
  line.dataset.failureKind = parts.kind || "";
  line.dataset.identityOrMark = parts.identityOrMark || "";
  const dot = document.createElement("span");
  dot.className = "fdot";
  line.appendChild(dot);
  const sentence = document.createElement("b");
  sentence.className = "selectable";
  sentence.textContent = parts.sentence || "";
  line.appendChild(sentence);
  for (const control of parts.controls || []) {
    const link = document.createElement("a");
    link.href = "#";
    link.textContent = control.label;
    link.addEventListener("click", (event) => {
      if (event && event.preventDefault) event.preventDefault();
      if (parts.onAct) parts.onAct(control.act);
    });
    line.appendChild(link);
  }
  return line;
}

/// **A failure's *Report a problem…*, handed to the report box** — M33's
/// handover, as the shape composed it, to `Reporting a problem` §8's entry
/// point (`src/surface/reporting.rs`, drawn by `web/reporting.js`). Built at
/// step 11's piece R4.
///
/// **It lives beside the shape rather than beside the box**, so every page that
/// draws a failure reaches the box through the module it already imports, and
/// the box's own module is loaded once, by the page's one import list.
/// **Nothing here judges the handover** — the application does, before
/// anything is touched (H34).
export function theReportBoxOpensFrom(handover) {
  const given = handover || {};
  const at = Number(given.at);
  shell.core.invoke("the_report_box_is_opened_from_a_failure", {
    kind: given.kind || "",
    identityOrMark: given.identityOrMark,
    durableSubject: given.durableSubject,
    subjectKind: given.subjectKind,
    thePresentedSentence: given.thePresentedSentence,
    at: Number.isFinite(at) && at > 0 ? Math.floor(at) : undefined,
  });
}

/// **The control list off the event, in the order the application composed it.**
///
/// The keys are `control0Is`, `control0Label`, `control0Act` and so on, with
/// `controlCount` saying how many — declared beside the composer in
/// `src/surface/the_failure_shape.rs`. **Reading them in order is the whole of
/// what this side does with them.**
export function theControlsFrom(said) {
  const controls = [];
  const many = Number(said.controlCount || 0);
  for (let at = 0; at < many; at += 1) {
    const is = said["control" + at + "Is"];
    if (!is) continue;
    controls.push({
      is,
      label: said["control" + at + "Label"] || "",
      act: said["control" + at + "Act"] || "",
    });
  }
  return controls;
}

// **The one control that restores every suppressed warning, on the page** —
// Desktop app and delivery §15, and Preparation/Design/Settings.md's *Warnings
// and notifications*.
//
// **Read the head of `smoozy.js` first.** This module keeps every part of the
// page's own rule: it makes its own elements, it adds nothing to `index.html`,
// and it draws only what an event handed it.
//
// ## What this draws, and where it belongs
//
// Design puts it in Settings, in the section that also holds the four
// notification switches, *"because they are the same family: things Smoozy says
// to the user"*. **What is here is the control itself as an exported builder,
// and nothing else.** `web/settings.js` calls it; the proof surface this file
// once drew into the pane was taken down when that screen arrived (step 10a
// piece 11, 2026-09-13).
//
// ## The three things this control has to say, and they are all Design's
//
// - **The count.** *"Without it the user cannot tell a control that restores
//   nine warnings from one that restores none."*
// - **That it stays when there is nothing to restore.** *"A control that
//   appears only when it has work is a control nobody knows exists."*
// - **That there is no way back one at a time.** *"The spec makes it all at
//   once; saying so stops a user hunting for a list that does not exist."*
//
// ## Two strings, two routes, and the count is not joined into the sentence
//
// The count arrives on the verbatim route and the word beside it as prose
// (A6, X9). **They are two nodes here and never one string** — joining them on
// this side would undo the split the application side made, which is the trap
// the whole of step 10a is written against (X7).
//
// H32: values are content, never markup — `textContent` and `createElement`.
// H33: no inline style; the rules are in `said-once.css`. P9 and P10: nothing
// here polls and nothing here ticks.

// **What stands between a figure and the words that count it** — U+00A0, so
// *3 switched off* never parts across two lines. `Design/Whole app §
// Typography`; the character's one home is `src/typography.rs`
// (`BETWEEN_A_FIGURE_AND_ITS_UNIT`).
const BETWEEN_A_FIGURE_AND_ITS_UNIT = "\u00A0";

/// **Build the restore control.**
///
/// `parts` is what the event said:
///
///   count                   how many are switched off, as text — the verbatim
///                           route, never joined into a sentence. **Absent
///                           where the store would not answer**, and then
///                           `theCountWouldNotRead` carries the line instead
///   theCountWouldNotRead    the line where the read failed — L8a, and it is
///                           what stops a failed read being drawn as nought
///   theRestoreDidNotLand    the line where the press did not land. **It
///                           arrives beside one of the two above and never
///                           instead of them**: the table is as it was, so the
///                           count the user was reading is still true and stays
///                           drawn (C-2)
///   theCountsWord           the word beside it, through the point
///   whenNoneAreSwitchedOff  the whole line where the count is nought
///   heading, theControl     the section's heading and the control's label
///   whatItCovers            the first sentence under it
///   andNoWayBackOneAtATime  the second, handed separately for X10's sake
///   onRestore               called when the control is pressed
export function theWarningsSwitchedOff(parts) {
  // `stgs` is the drawing's own class for one section of Settings, and it is
  // what `.stgs .hintl` in the lifted stylesheet is written against.
  const section = document.createElement("section");
  section.className = "stgs said-once";

  const heading = document.createElement("h2");
  heading.className = "gap";
  heading.textContent = parts.heading || "";
  section.appendChild(heading);

  const row = document.createElement("div");
  row.className = "stgrow";

  // **A count that did not read is not a count of nought.** L8a: *"there is no
  // state in which a list is drawn short, a count reads zero, or a used by
  // nothing line appears because a read failed"* — and the application posts no
  // count at all in that case, so there is nothing here to draw as nought.
  // Corrected 2026-09-12 (F-10).
  const unread = typeof parts.count !== "string";
  const none = !unread && parts.count === "0";

  const label = document.createElement("span");
  label.className = "lab";
  if (unread) {
    label.textContent = parts.theCountWouldNotRead || "";
  } else if (none) {
    // One sentence, one string, already through the point.
    label.textContent = parts.whenNoneAreSwitchedOff || "";
  } else {
    // **Two nodes, because they came by two routes.** The count is a reading
    // and the word is prose; a builder that wrote one string here would have
    // joined them after the split the application side made for exactly this.
    const many = document.createElement("b");
    many.textContent = parts.count;
    label.appendChild(many);
    label.appendChild(document.createTextNode(BETWEEN_A_FIGURE_AND_ITS_UNIT));
    label.appendChild(document.createTextNode(parts.theCountsWord || ""));
  }
  row.appendChild(label);

  const grow = document.createElement("span");
  grow.className = "grow";
  row.appendChild(grow);

  const control = document.createElement("button");
  control.type = "button";
  control.className = "mini";
  control.textContent = parts.theControl || "";
  // **The drawing disables it where there is nothing to restore and keeps it on
  // screen**, which is Design's own rule about the control not going away. The
  // application refuses nothing: pressing it with an empty table restores none
  // and is not a failure (G8), so this is the drawing being honest rather than
  // a guard. **A count that would not read leaves the control pressable**: an
  // unread number is not a table known to be empty.
  control.disabled = none;
  control.addEventListener("click", () => {
    if (parts.onRestore) parts.onRestore();
  });
  row.appendChild(control);
  section.appendChild(row);

  // **The two sentences are two strings and stay two.** X10: the orphan rule
  // binds the last two words of a piece of prose, and a paragraph handed over
  // whole leaves every interior sentence ending unbound. They read as one
  // paragraph and were never one string.
  const covers = document.createElement("p");
  covers.className = "hintl";
  covers.appendChild(document.createTextNode(parts.whatItCovers || ""));
  covers.appendChild(document.createTextNode(" "));
  covers.appendChild(document.createTextNode(parts.andNoWayBackOneAtATime || ""));
  section.appendChild(covers);

  // **No press of this control ends in nothing being said.** A refused delete
  // used to post nothing at all: the count did not move, no failure was drawn,
  // and what the user saw was a control that did nothing — twice. The sentence
  // arrives as prose and is drawn in the one failure shape (F-10).
  if (parts.theRestoreDidNotLand) {
    const refused = document.createElement("p");
    refused.className = "hintl said-once-refused";
    refused.textContent = parts.theRestoreDidNotLand;
    section.appendChild(refused);
  }

  return section;
}

// ── Where it is drawn ────────────────────────────────────────────────────────
//
// **The proof surface that stood here is gone**, taken down by step 10a piece
// 11 on 2026-09-13. It drew this control into the bare pane, below the rail
// and the main pane, on every screen. **The Settings screen draws it now**, in
// its *Warnings and notifications* section, by calling the builder above from
// `web/settings.js` — which is also where the listener for
// `settings:the-warnings-switched-off` and the ask for the count now live.

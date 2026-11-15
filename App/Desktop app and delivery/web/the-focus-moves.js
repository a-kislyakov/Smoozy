// **Wherever the keyboard moves the focus, the ring shows; where the pointer
// moved it, none does** — `Preparation/Design/Whole app.md`, *Keyboard
// shortcuts*, added 2026-11-10 from the Creator's twenty-ninth notes, note 1:
// *"when I tab into a list (keys) it doesn't highlight it … it's everywhere in
// the UI."* And beside it, note 7: **Tab into a text field chooses its whole
// text.** The account is `Audits/Build/Twenty-ninth notes/Part A — what each
// one is.md`, notes 1 and 7.
//
// **The one way every page script moves the focus.** No script calls
// `.focus(` itself; a guard test in `src/surface/focus.rs` finds any that do.
//
// **Why it is needed.** The Mac's engine decides by itself whether a focus
// moved by a page script shows its ring, and after any click it decides no,
// even when a key was pressed since. Every Tab Smoozy moves itself, every box
// opening, every redraw giving the focus back is such a move. So the page
// keeps what the user did last — a key or the pointer — and says it with the
// move: `focusVisible`, which the engine puts before its own guess. An engine
// that does not know the option ignores it and keeps its own guess.
//
// **What a move carries.**
// - `preventScroll`: the list does not scroll to the control, as before.
// - `byTab`: the move is a Tab or Shift+Tab. A one-line text field that can be
//   typed in then has its whole text chosen, as the engine's own Tab does, so
//   what is typed replaces it. A field of several lines, a read-only field and
//   every other move keep the caret where the engine puts it: a box still opens
//   with the caret in its first field, and a redraw gives a field its caret
//   back.

let theLastWasAKey = false;

// Heard first, before any page script answers the same key or press. A page
// run with no window — a test drawing one module on its own — has nothing to
// hear.
if (typeof window !== "undefined" && typeof window.addEventListener === "function") {
  window.addEventListener("keydown", () => { theLastWasAKey = true; }, true);
  for (const kind of ["pointerdown", "mousedown"]) {
    window.addEventListener(kind, () => { theLastWasAKey = false; }, true);
  }
}

// **A field Tab chooses the whole text of**: one line, typed into, not
// read-only. Kept to the kinds this application draws and the ones a text
// field could become.
const ONE_LINE = new Set(["text", "password", "search", "number", "email", "url", "tel"]);

function aFieldTabChoosesAllOf(element) {
  return (
    element.tagName === "INPUT" &&
    ONE_LINE.has(element.type) &&
    !element.readOnly &&
    !element.disabled &&
    typeof element.select === "function"
  );
}

/** Moves the focus to `element`, if there is one; see the head of this file. */
export function theFocusMovesTo(element, { preventScroll = false, byTab = false } = {}) {
  if (!element || typeof element.focus !== "function") return;
  element.focus({ preventScroll, focusVisible: theLastWasAKey });
  if (byTab && document.activeElement === element && aFieldTabChoosesAllOf(element)) {
    element.select();
  }
}

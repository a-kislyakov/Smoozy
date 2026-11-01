// **Text being typed is not lost to a redraw** — `Design/Whole app § The
// window`: *"a page that redraws while a field has the focus keeps what is
// typed in it, not yet committed, and where the caret was."*
//
// **Why there is one file for it.** Step 10c's ninth pass typed a job name
// into Render Settings and found the field empty 0.3 s later, the caret still
// in it: the page is rebuilt whole from the application's last answer, and a
// field commits on `change`, which fires only when it is left or Enter is
// pressed. So any answer arriving while the user typed drew the field again
// from what was last committed, which was nothing. **Every page that draws
// again while a text field can hold the focus meets the same thing**, so the
// rule is kept once, here, the way `./the-return-key.js` keeps Return's.
//
// **What counts as typed and not yet committed.** A field is marked the moment
// it is typed into (`input`) and unmarked when its answer goes (`change`), by
// one listener each for the whole page. **A field whose own answer went — Enter
// pressed, or the field left — is drawn from the answer**, exactly as before;
// only a field still being typed in keeps its text over the answer.
//
// **What a page does with it.** Before it takes a field out, it calls
// `theTypingIsTakenFrom(the part it redraws)`; after drawing, it hands the
// field that took its place to `theTypingIsPutBackInto`. A page that keeps its
// fields and only writes their values asks `aFieldIsBeingTypedIn` before it
// writes one.
//
// **A field taken out by a redraw commits nothing.** An engine may send
// `change` from a focused field as it leaves the page — Chrome does, which the
// page harness showed on the ninth pass's own case — and that would send the
// half-typed text as the user's answer. So the `change` from a field this file
// took out is stopped before the page's own handler sees it.
//
// **And typing put back still commits the way typing does.** An engine
// measures *changed* from the last value the page wrote, so a field given back
// its typing and then left, or answered with Enter, with no key in between,
// would send no `change` at all — the page harness showed it, and the job
// would have gone out with no name. So a field given back its typing is owed
// its `change`, and gets it on Enter or on being left if the engine sent none.
//
// Nothing here repeats, polls or waits (P9, P10): it answers the user's own
// keys and the page's own calls, and nothing else. Added 2026-09-28, step 10c's
// ninth-pass fix piece 5.

// Fields typed into whose answer has not gone.
const typedAndNotCommitted = new WeakSet();
// Fields a redraw took out while they were being typed in.
const takenOut = new WeakSet();
// Fields given back their typing whose engine has not yet seen a key in them,
// and so would send no `change` of its own.
const owedAChange = new WeakSet();

// The input types that are typed into.
const TYPED_INTO = new Set(["", "text", "password", "search", "email", "url", "tel", "number"]);

function isTypedInto(element) {
  if (!element || !element.tagName) return false;
  const tag = String(element.tagName).toUpperCase();
  if (tag === "TEXTAREA") return !element.readOnly;
  if (tag !== "INPUT") return false;
  return TYPED_INTO.has(String(element.type || "").toLowerCase()) && !element.readOnly;
}

document.addEventListener(
  "input",
  (typed) => {
    if (!isTypedInto(typed.target)) return;
    typedAndNotCommitted.add(typed.target);
    // A key in it: the engine's own `change` will come.
    owedAChange.delete(typed.target);
  },
  true
);

document.addEventListener(
  "change",
  (committed) => {
    const field = committed.target;
    if (takenOut.has(field)) {
      committed.stopImmediatePropagation();
      return;
    }
    typedAndNotCommitted.delete(field);
    owedAChange.delete(field);
  },
  true
);

function theChangeIsSent(field) {
  owedAChange.delete(field);
  field.dispatchEvent(new Event("change", { bubbles: true }));
}

// **Enter**, where the engine sent no `change` of its own on the key's way down.
document.addEventListener(
  "keyup",
  (key) => {
    const field = key.target;
    if (key.key !== "Enter" || !owedAChange.has(field) || !field.isConnected) return;
    theChangeIsSent(field);
  },
  true
);

// **Being left** — the focus gone somewhere else in the page, and not the
// window put behind another, which leaves the focus where it was and commits
// nothing. A field a redraw is taking out is not being left.
document.addEventListener(
  "focusout",
  (left) => {
    const field = left.target;
    if (!owedAChange.has(field) || takenOut.has(field) || !field.isConnected) return;
    if (document.activeElement === field) return;
    theChangeIsSent(field);
  },
  true
);

/// **Whether a field holds typing whose answer has not gone.** A page that
/// writes a field's value from an answer, rather than drawing the field again,
/// leaves this one alone.
export function aFieldIsBeingTypedIn(field) {
  return Boolean(field) && typedAndNotCommitted.has(field);
}

/// **Before a redraw: the field with the focus inside `within`, what is typed
/// in it and where the caret is.** Nothing where the focus is elsewhere or on
/// something that is not typed into.
export function theTypingIsTakenFrom(within) {
  const field = document.activeElement;
  if (!within || !field || !within.contains(field) || !isTypedInto(field)) return null;
  const typed = typedAndNotCommitted.has(field);
  if (typed) takenOut.add(field);
  let start = null;
  let end = null;
  let direction = "none";
  try {
    start = field.selectionStart;
    end = field.selectionEnd;
    direction = field.selectionDirection || "none";
  } catch (notOffered) {
    // A field that keeps no caret; there is none to put back.
  }
  return { field, typed, value: field.value, start, end, direction, across: field.scrollLeft };
}

/// **After a redraw: the typing, the focus and the caret, into the field that
/// took its place** — or into the same field, where the page kept it. The value
/// goes back only where it was still being typed; a field whose answer went is
/// left as the answer drew it and gets its focus and caret back.
export function theTypingIsPutBackInto(field, taken) {
  if (!taken || !field || !field.isConnected || !isTypedInto(field)) return;
  takenOut.delete(field);
  if (taken.typed) {
    if (field.value !== taken.value) field.value = taken.value;
    typedAndNotCommitted.add(field);
    owedAChange.add(field);
  }
  if (document.activeElement !== field) field.focus({ preventScroll: true });
  if (taken.start !== null && taken.end !== null) {
    const length = field.value.length;
    try {
      field.setSelectionRange(
        Math.min(taken.start, length),
        Math.min(taken.end, length),
        taken.direction
      );
    } catch (notOffered) {
      // A field that keeps no caret.
    }
  }
  field.scrollLeft = taken.across;
}

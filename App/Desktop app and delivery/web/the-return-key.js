// **Return in a box presses the box's own affirmative control** —
// `Preparation/Design/Whole app.md`, *Keyboard shortcuts*: *"Return presses the
// box's own affirmative control where the box has one, and never where the
// affirmative control is a red one. A box whose answer is Delete takes no
// Return."*
//
// **One listener for every box, because the rule is one rule.** Until
// 2026-09-19 no box in the page had it — the Gemini catch-up of step 10a, Area 5,
// found it missing on the passphrase box (F-4), and the build runner asked for
// every box to be given it. **Every box in `web/` is built as a `.mbox`**, so
// a listener keyed on that class reaches all of them, and any box built later,
// by construction rather than by each builder remembering.
//
// **Which control is the affirmative.** Each box's action row carries its
// controls as `.btn`; the ones that step back — *Cancel*, *Keep*, *Not now* —
// are `.btn.quiet`, and a red act is `.btn.danger`. So the affirmative is the
// box's visible, enabled `.btn` that is not quiet. **Where it is red, Return
// does nothing at all**, including on a focused control, so a red act is never
// taken by the Return key. **Where a box has no affirmative** — a removal box
// offering two choices, a box with only *Cancel* — Return does nothing either.
//
// **What keeps Return for itself.** A field or control that already answers
// Return with an act of its own handles it first and says so by preventing the
// default (the passphrase field, a tick box, a list row); a text area, a
// drop-down, a tick box or a radio choice keeps its own use of the key. **A
// button with the focus is not one of those**: in a box, Return is the box's,
// which is what makes a focused *Cancel* not swallow it.
//
// **The page composes nothing here and posts nothing**: pressing the control
// is exactly the user's own press, through that control's own handler.

// The roles whose own use of Return a box must not take over.
const ROLES_THAT_KEEP_RETURN = new Set([
  "checkbox",
  "radio",
  "switch",
  "option",
  "menuitem",
  "menuitemcheckbox",
  "menuitemradio",
  "tab",
  "combobox",
  "listbox",
  "textbox",
]);

// The input types that are typed into, where Return means *I am done here*.
const INPUTS_TYPED_INTO = new Set([
  "",
  "text",
  "password",
  "search",
  "email",
  "url",
  "tel",
  "number",
]);

function isShown(element) {
  return (
    element.isConnected &&
    !element.closest("[hidden]") &&
    element.getClientRects().length > 0
  );
}

// **The box on top**: the last shown `.mbox` in the document, which is the one
// drawn over the others where two are open.
function theOpenBox() {
  const open = Array.from(document.querySelectorAll(".mbox")).filter(isShown);
  return open.length ? open[open.length - 1] : null;
}

function keepsReturnForItself(target) {
  if (!(target instanceof Element)) return false;
  const role = (target.getAttribute("role") || "").toLowerCase();
  if (ROLES_THAT_KEEP_RETURN.has(role)) return true;
  if (target.isContentEditable) return true;
  const tag = target.tagName;
  if (tag === "TEXTAREA" || tag === "SELECT") return true;
  if (tag === "INPUT") {
    return !INPUTS_TYPED_INTO.has((target.getAttribute("type") || "").toLowerCase());
  }
  return false;
}

// **The affirmative control, or nothing.** The last one in the box, since an
// action row puts the step-back controls before the one that goes ahead.
function theAffirmativeIn(box) {
  const controls = Array.from(box.querySelectorAll("button.btn")).filter(
    (control) =>
      !control.classList.contains("quiet") && !control.disabled && isShown(control),
  );
  return controls.length ? controls[controls.length - 1] : null;
}

document.addEventListener("keydown", (press) => {
  if (press.key !== "Enter" || press.defaultPrevented || press.isComposing) return;
  if (press.shiftKey || press.altKey || press.ctrlKey || press.metaKey) return;
  const box = theOpenBox();
  if (!box) return;
  if (box.contains(press.target) && keepsReturnForItself(press.target)) return;
  const affirmative = theAffirmativeIn(box);
  // **Never a red one**, and never a focused control in its place.
  press.preventDefault();
  if (!affirmative || affirmative.classList.contains("danger")) return;
  affirmative.click();
});

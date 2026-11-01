// **Escape closes the thing on top, one layer at a time** —
// `Preparation/Design/Whole app.md`, *Keyboard shortcuts*. The twin of
// `./the-return-key.js`: one listener for the whole page, because the rule is
// one rule.
//
// **Why it exists.** The Creator's seventh notes on the test copy, note 8: most
// boxes and panels already closed on Escape, each by its own listener, but
// every Queue and History box, every Keys box and the Blender output panel did
// not. Rather than give each its own, this one reaches every overlay built as a
// `.modal`, including any built later.
//
// **What it does.** On Escape it finds the overlay on top and presses that
// overlay's own visible, enabled step-back control — the `.btn.quiet`, such as
// *Cancel*, *Keep* or *Close*. Pressing it is exactly the user's own press,
// through that control's own handler; nothing is composed or posted here.
//
// **What it leaves alone.**
// - **An overlay that already answers Escape keeps its own behaviour.** This
//   listener sits on the window and hears the key last, after every listener
//   on an element or on the document; a press already answered (its default
//   prevented, or stopped on the way up) never reaches it, so one press never
//   closes two layers.
// - **A menu open at the moment of the press owns that press.** Some menus
//   close on Escape without saying so, so whether one was open is noted before
//   anything else hears the key.
// - **A box with no step-back control stays**, and so does a box whose act is
//   already underway (a control marked `.uway`), which is left to finish.

function isShown(element) {
  return (
    element.isConnected &&
    !element.closest("[hidden]") &&
    element.getClientRects().length > 0
  );
}

// **The overlay on top.** A `.modal.top` is drawn over every plain `.modal`;
// among equals, the last in the document is drawn over the others.
function theOverlayOnTop() {
  const open = Array.from(document.querySelectorAll(".modal")).filter(isShown);
  const over = open.filter((one) => one.classList.contains("top"));
  const pick = over.length ? over : open;
  return pick.length ? pick[pick.length - 1] : null;
}

function aMenuIsOpen() {
  return Array.from(document.querySelectorAll(".ctx")).some(isShown);
}

// **The step-back control, or nothing.** The last one, since an action row puts
// its step-back controls together and the one that closes the box is last.
function theStepBackIn(overlay) {
  const controls = Array.from(overlay.querySelectorAll("button.btn")).filter(isShown);
  if (controls.some((control) => control.classList.contains("uway"))) return null;
  const quiet = controls.filter(
    (control) => control.classList.contains("quiet") && !control.disabled,
  );
  return quiet.length ? quiet[quiet.length - 1] : null;
}

let theMenuOwnsThePress = false;

window.addEventListener(
  "keydown",
  (press) => {
    if (press.key === "Escape") theMenuOwnsThePress = aMenuIsOpen();
  },
  true,
);

window.addEventListener("keydown", (press) => {
  if (press.key !== "Escape" || press.defaultPrevented || press.isComposing) return;
  if (press.shiftKey || press.altKey || press.ctrlKey || press.metaKey) return;
  if (theMenuOwnsThePress) return;
  const overlay = theOverlayOnTop();
  if (!overlay) return;
  const stepBack = theStepBackIn(overlay);
  if (!stepBack) return;
  press.preventDefault();
  stepBack.click();
});

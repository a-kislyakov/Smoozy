// **Tab stays inside the box on top** — `Preparation/Design/Whole app.md`,
// *Keyboard shortcuts*, added 2026-11-09 from the Creator's twenty-eighth
// notes, note 2: *"The add server dialog should only allow tabbing through its
// own controls, and not All servers, Queue etc. Same for all such cases."* The
// twin of `./the-escape-key.js`: one listener for the whole page, because the
// rule is one rule, and it reaches every box built as a `.modal`, and the
// dependency check's row menu (`.menus`), including any built later.
//
// **What it does.**
// - **Smoozy's Tab owns Tab on every screen**, not only with a box or a panel
//   open — `Preparation/Design/Whole app.md`, *Keyboard shortcuts*, *Wherever
//   the keyboard moves the focus, the ring shows*, added 2026-11-10 from the
//   Creator's twenty-ninth notes, note 1. With nothing open it walks the
//   window's own order, wrapping, so every control is reached the same way on
//   every screen whatever the computer's own keyboard setting says, and every
//   move goes through `./the-focus-moves.js`, which shows the ring after a
//   key and chooses a text field's whole text on Tab (note 7).
// - **Tab and Shift+Tab move among the visible, enabled Tab stops of the box
//   on top, and wrap** from the last to the first and back. A press from
//   outside the box, or from the box itself, goes to its first stop (its last,
//   with Shift).
// - **A panel over a server keeps the screen under it out of Tab's reach.**
//   `./a-server.js` marks that screen `data-under-a-panel` while a panel
//   covers it; with no box open, Tab walks the window's order and steps over
//   what is marked. The rail, the title bar and the strips stay
//   reachable, because a panel does not take the window.
// - **A box gives the focus back to what opened it when it closes.** The
//   control that held the focus when the focus went into the box is kept; when
//   the box has gone and the focus with it, that control takes it again, if it
//   is still there to take it.
//
// **What it leaves alone.**
// - **A press already answered keeps its answer.** This listener sits on the
//   window and hears the key last; a box with a rule of its own (the update
//   box, the rollback box, Queue's large preview) prevents the default and is
//   left to it.
// - **A Tab with Option, Control or Command**, which is not a move.
//
// Each box takes the focus when it opens in the code that draws it — its first
// field where it asks for typing, otherwise the box itself — so the first Tab
// lands on its first control.

import { theFocusMovesTo } from "./the-focus-moves.js";

function isShown(element) {
  return (
    element.isConnected &&
    !element.closest("[hidden]") &&
    element.getClientRects().length > 0
  );
}

// **The overlay on top.** `./the-escape-key.js`'s order: a `.modal.top` over
// every plain `.modal`, and among equals the last in the document. The
// dependency check's row menu sits under both, over its panel alone.
function theOverlayOnTop() {
  const open = Array.from(document.querySelectorAll(".modal")).filter(isShown);
  const over = open.filter((one) => one.classList.contains("top"));
  const pick = over.length ? over : open;
  if (pick.length) return pick[pick.length - 1];
  const menus = Array.from(document.querySelectorAll(".menus")).filter(isShown);
  return menus.length ? menus[menus.length - 1] : null;
}

const UNDER_A_PANEL = "[data-under-a-panel]";

const A_TAB_STOP = "a[href],button,input,select,textarea,summary,[tabindex],[contenteditable]";

// **A Tab stop**: what the browser would stop on — in the order, enabled, drawn
// and visible — and not on a screen a panel covers.
function theTabStopsIn(root) {
  return Array.from(root.querySelectorAll(A_TAB_STOP)).filter(
    (one) =>
      one.tabIndex >= 0 &&
      !one.disabled &&
      one.type !== "hidden" &&
      !one.closest(UNDER_A_PANEL) &&
      isShown(one) &&
      getComputedStyle(one).visibility !== "hidden",
  );
}

// **The next stop after the focus, in the page's order**, wrapping. The focus
// may be on no stop at all — the box itself, or the page — and then the stop
// after it, or before it with Shift, is the one.
function theNextStop(stops, from, back) {
  if (!from || from === document.body || !from.isConnected) {
    return back ? stops[stops.length - 1] : stops[0];
  }
  const at = stops.indexOf(from);
  if (at >= 0) return stops[(at + (back ? stops.length - 1 : 1)) % stops.length];
  if (back) {
    const before = stops.filter(
      (one) =>
        one.compareDocumentPosition(from) & Node.DOCUMENT_POSITION_FOLLOWING &&
        !one.contains(from),
    );
    return before.length ? before[before.length - 1] : stops[stops.length - 1];
  }
  const after = stops.filter(
    (one) => from.compareDocumentPosition(one) & Node.DOCUMENT_POSITION_FOLLOWING,
  );
  return after.length ? after[0] : stops[0];
}

window.addEventListener("keydown", (press) => {
  if (press.key !== "Tab" || press.defaultPrevented || press.isComposing) return;
  if (press.altKey || press.ctrlKey || press.metaKey) return;
  const overlay = theOverlayOnTop();
  const stops = theTabStopsIn(overlay || document);
  press.preventDefault();
  if (!stops.length) return;
  const from = document.activeElement;
  // A focus outside the box goes in at the box's own end.
  const inside = !overlay || overlay.contains(from);
  const next = inside
    ? theNextStop(stops, from, press.shiftKey)
    : stops[press.shiftKey ? stops.length - 1 : 0];
  theFocusMovesTo(next, { byTab: true });
});

// ── The focus given back ────────────────────────────────────────────────────
//
// **Where the focus came from, per box**, kept when the focus first goes into
// it. Watched only while a box is kept, and only for boxes going: a node taken
// out or a `hidden` set.

const A_BOX = ".modal,.menus";
const cameFrom = new Map();
let watching = null;

function isStillThere(element) {
  return element.isConnected && !element.closest("[hidden]");
}

function theFocusIsLost() {
  const on = document.activeElement;
  return !on || on === document.body || !isStillThere(on);
}

function theBoxesAreLookedAt() {
  for (const [box, from] of Array.from(cameFrom)) {
    if (isStillThere(box)) continue;
    cameFrom.delete(box);
    if (!from || !theFocusIsLost()) continue;
    if (!isStillThere(from) || from.disabled || from.closest(UNDER_A_PANEL)) continue;
    theFocusMovesTo(from);
  }
  if (!cameFrom.size && watching) {
    watching.disconnect();
    watching = null;
  }
}

document.addEventListener("focusin", (arrived) => {
  const box = arrived.target && arrived.target.closest ? arrived.target.closest(A_BOX) : null;
  if (!box || cameFrom.has(box)) return;
  const from = arrived.relatedTarget;
  cameFrom.set(box, from && !box.contains(from) && from !== document.body ? from : null);
  if (watching || typeof MutationObserver !== "function") return;
  watching = new MutationObserver(theBoxesAreLookedAt);
  watching.observe(document.documentElement, {
    childList: true,
    subtree: true,
    attributes: true,
    attributeFilter: ["hidden"],
  });
});

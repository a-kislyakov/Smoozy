// **Where the user had scrolled is not lost to a redraw** — `Design/Whole app
// § The window`: *"a list drawn again with the same things in it keeps its
// place."* The same rule `./the-typed-text.js` keeps for typed text, kept once
// here for the scrolled place.
//
// **Why there is one file for it.** The Creator's second notes on the test
// copy, 7, 8 and 9: the cross that hands a View Layer back threw Render
// Settings to its top, and a job's entry in Queue went back to its top every
// time its live reading arrived. **One cause on three pages**: each answer
// draws the page again whole, and the part that scrolls — Render Settings'
// columns, the queue's and History's list, a job's entry — is a new element
// every time, so it starts at its top. The page harness showed all three
// before this file was written (`Audits/Build/Step 10c/The Creator's second
// notes, piece 5 — what landed.md`).
//
// **What a page does with it: it names the part that scrolls, and nothing
// else.** A page gives the element it scrolls a `data-place` naming what is in
// it — the page, and the file or the job it shows:
//
//     scrolled.dataset.place = JSON.stringify(["queue-entry", job]);
//
// **The same name drawn again is the same place**, and this file puts the new
// element back where the user had the old one. **A different name starts at
// its top**: another job's entry, another file's settings. **A place whose part
// leaves the page and does not come back in the same drawing is let go**, so a
// job chosen again later, or a file opened again, starts at its top too. A page
// imports nothing from here, so a page under test needs no stand-in for it.
//
// **How it knows.** Three listeners for the whole page, none of them a timer:
// - **The user's scrolling** (`scroll`, taken on its way down): the place of
//   every named part, as the user leaves it.
// - **A drawing** (`MutationObserver`): a named part arriving is put back at
//   its name's place before the page is painted, so nothing is seen at the top
//   first.
// - **A part drawn while it is not shown** — a collapsed panel, a screen that
//   is away — has no place to be put back to yet (an element that is not laid
//   out does not scroll). It is put back the first time it is laid out
//   (`ResizeObserver`), which is before it is painted.
//
// Nothing here repeats, polls or waits (P9, P10): it answers the user's own
// scrolling and the page's own drawing, and nothing else. Added 2026-10-04, the
// Creator's second notes on the test copy, piece 5.

// What names a part that scrolls — `data-place`.
const NAMED = "[data-place]";

// name → how far down the user had it.
const places = new Map();

function theNameOf(element) {
  return element && element.dataset ? element.dataset.place : undefined;
}

function itIsLaidOut(element) {
  return element.getClientRects().length > 0;
}

function thePlaceIsGivenBack(element) {
  const top = places.get(theNameOf(element));
  if (top && element.scrollTop !== top) element.scrollTop = top;
}

// **The parts drawn while not shown, until they are laid out.**
const notLaidOut = new ResizeObserver((sized) => {
  for (const one of sized) {
    const element = one.target;
    if (!element.isConnected) {
      notLaidOut.unobserve(element);
      continue;
    }
    if (!itIsLaidOut(element)) continue;
    notLaidOut.unobserve(element);
    thePlaceIsGivenBack(element);
  }
});

document.addEventListener(
  "scroll",
  (scrolled) => {
    const element = scrolled.target;
    const name = theNameOf(element);
    // A part already taken out of the page reads as at its top, and is not
    // where the user was.
    if (!name || !element.isConnected) return;
    places.set(name, element.scrollTop);
  },
  true
);

// Every named part in what a drawing added or took out, the part itself or
// inside it.
function theNamedPartsIn(nodes) {
  const found = [];
  for (const node of nodes) {
    if (!node || node.nodeType !== 1) continue;
    if (theNameOf(node)) found.push(node);
    for (const inside of node.querySelectorAll(NAMED)) found.push(inside);
  }
  return found;
}

new MutationObserver((drawn) => {
  // Nothing scrolled yet, nothing to put back or let go.
  if (!places.size) return;
  const arrived = [];
  const left = new Set();
  for (const change of drawn) {
    for (const part of theNamedPartsIn(change.removedNodes)) {
      left.add(theNameOf(part));
      // A part taken out before it was ever shown is not waited for.
      notLaidOut.unobserve(part);
    }
    arrived.push(...theNamedPartsIn(change.addedNodes));
  }
  const here = arrived.filter((part) => part.isConnected);
  const cameBack = new Set(here.map(theNameOf));
  for (const name of left) if (!cameBack.has(name)) places.delete(name);
  for (const part of here) {
    if (!places.get(theNameOf(part))) continue;
    if (itIsLaidOut(part)) thePlaceIsGivenBack(part);
    else notLaidOut.observe(part);
  }
}).observe(document.documentElement, { childList: true, subtree: true });

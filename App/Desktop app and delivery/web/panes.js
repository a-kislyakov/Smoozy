// **The four narrowing widths, the floor, and the drag seams** — the page half
// of `src/surface/panes.rs`, whose head carries the whole account of the piece
// and the ruling behind it. `web/the-drawing.css` carries the steps themselves.
//
// **What this file does is three things and no fourth.**
//
//  1. It measures `.app`'s own width and writes a step mark onto it, at the
//     five boundaries the drawing narrows at. Once at load, synchronously, and
//     then whenever the window is resized.
//  2. It draws the panel that appears below the floor and says so.
//  3. It makes the seams between panes draggable, clamped at both ends, reset
//     by a double-click, with the width written down when the drag ends.
//
// **The mockup narrows with CSS container queries and this does not.** Those
// need macOS 13 and the built artifact declares 10.13, so on every Mac below
// Ventura the four steps and the floor silently did nothing at all and the
// *Smoozy needs a wider window* panel never appeared. Desktop app and delivery
// H12 forbids building an interface behaviour on a capability that is not on
// all three engines. The ruling is `Audits/Build/Step 10a/Ruling — how the
// narrowing is built.md`. **What is built here rests on an element's own width
// and a resize event, which every engine in the release matrix has had for
// longer than any of them has had the thing it replaces.**
//
// **Nothing here is scheduled, ever.** Desktop app and delivery P9: nothing on
// this side polls. P10: there is exactly one standing timer on this side and it
// is the update check. This sits with H18's `MediaQueryList` listener — the
// engine raises an event, and the only work on that path is writing the mark.
//
// **The window is the container, not the screen.** Every boundary below is
// asked of `.app`, which is Smoozy's own window, and never of the display.

// **The one shape a failure is drawn in** — read that file's head before
// drawing one anywhere. This module draws one and only one: the telling that a
// read of the widths would not answer (F-6's second half).
import { theFailureShape, EXPANDED } from "./the-failure-shape.js";

const shell = window.__TAURI__;

// ── The five boundaries ─────────────────────────────────────────────────────
//
// **Their one home is `Preparation/Design/Mockup/Smoozy.html`** and this is a
// copy, exactly as the whole of `the-drawing.css` is. A test in
// `src/surface/panes.rs` reads the mockup's own rules and this table and fails
// if the two have drifted, so the copy cannot go quietly wrong.
//
// A mark is a word, and `.app` carries every word whose width it is at or
// below — so a window at 800 pixels reads `s1 s2 s3`, which is what four
// overlapping `max-width` rules did.
const THE_NARROWING_STEPS = [
  { at: 1180, mark: "s1" },
  { at: 1020, mark: "s2" },
  { at: 900, mark: "s3" },
  { at: 780, mark: "s4" },
  { at: 680, mark: "floor" },
];

// **The floor is the one boundary compared strictly, and it is the only one
// with a sentence attached.** The panel says *make it at least 680 pixels
// wide*, so 680 is a width that is wide enough and the panel belongs
// below it; at 680 exactly the user did what Smoozy asked and the panel stayed.
// The mockup carries the same strict comparison — `@container win (width <
// 680px)` — so no second number exists on either side.
const THE_FLOOR_MARK = "floor";

// **From this step down nothing drags at all.** Design's own words: the rail
// and the list have both taken the widths the app declares for them, so there
// is nothing left on the line that is the user's to move. The drawing hides the
// seams here; this is the same fact where the drag can read it, and
// `src/surface/panes.rs` holds the two together.
const THE_STEP_THAT_STOPS_THE_SEAMS = "s3";

const app = document.querySelector(".app");

// ── The mark ────────────────────────────────────────────────────────────────

// **Measure the window and write the step mark onto it. Answer with the marks
// in force.**
//
// **This is the one place the boundaries are read**, and its callers are the
// top level below, which runs before the first paint, the resize listener, and
// the drag at the moment it begins — which is what makes the seams and the
// steps agree.
//
// **Nothing else happens on this path.** No width is written down, nothing is
// scheduled, and nothing is asked of the application. P11: the narrowing steps
// write nothing, which is what makes a dragged width *set aside, not
// destroyed*.
function theNarrowingIsMarked() {
  if (!app) return [];
  const across = app.clientWidth;
  const marks = THE_NARROWING_STEPS.filter((step) =>
    step.mark === THE_FLOOR_MARK ? across < step.at : across <= step.at,
  ).map((step) => step.mark);
  app.dataset.narrow = marks.join(" ");
  return marks;
}

// **Once, synchronously, before the first paint.** The ruling names this as the
// cost that gets forgotten: a first mark written from a callback leaves the
// window drawing its widest state until that callback lands, which is a visible
// flash at every start-up. A module's own top level runs when the page's module
// graph is evaluated, which is before anything is painted.
theNarrowingIsMarked();

window.addEventListener("resize", theNarrowingIsMarked);

// ── What the application says about the panes ───────────────────────────────
//
// **Asked for once, at load, and not on any event.** The floor panel is the
// window's own answer to being too small rather than a screen, so it must not
// wait on anything a module posts: it is made as soon as the application
// answers, whatever else has or has not happened.
const thePaneLayout = shell.core.invoke("the_pane_layout");

// **Whether the widths could not be read, which is a different thing from this
// user never having dragged anything.** L8a, cited by P11's last bullet in as
// many words: *an absent setting is an ordinary state, a read that did not
// answer is not.* Until 2026-09-12 the page could not tell the two apart, drew
// the defaults in silence either way, and then wrote a drag straight over what
// it had failed to read — so widths that were only unread became genuinely gone
// (F-6).
let theWidthsCouldNotBeRead = false;

thePaneLayout
  .then((layout) => {
    theFloorPanelIsDrawn(layout.floor);
    if (layout.the_record_would_not_answer) {
      theWidthsAreUnread(layout.the_widths_could_not_be_read);
      return;
    }
    theRememberedWidthsAreApplied(layout.seams);
  })
  .catch(() => {
    // The window is left standing, and the application answering nothing at all
    // is the same case as the record not answering: nothing is known about the
    // widths, so nothing is written over them.
    //
    // **Nothing is drawn here and that is not the same decision as above.** The
    // sentence comes from the application (A6, X1) and the application is what
    // did not answer, so there is no sentence to draw and this file may not
    // write one. The mark is set and the write is held back, which is what
    // keeps *unread* from becoming *gone*.
    theWidthsAreUnread(undefined);
  });

// **What the page does when the widths could not be read.**
//
// **The drawn defaults are what it draws**, because there is nothing else to
// draw, and the seams still drag so the window is usable. **What it does not do
// is write**, which is what keeps *unread* from becoming *gone*.
//
// **And it says so, in the failure shape** — `web/the-failure-shape.js`, which
// is the one shape a failure is drawn in and which needs a sentence and nothing
// else. **It is not a failure record**: this module has no durable subject to
// raise one against and invents none, and the Verdict asked for the shape.
// Until the close of round 1 this set the mark and stopped there — **and
// nothing in the page reads or draws that mark**, so the user whose widths
// could not be read met the default layout with no word about it, which is the
// same silence F-6 exists to end. Finished 2026-09-12 (F-6's second half).
//
// **The mark stays**, because it is what the drag and the double-click reset
// read to hold their write back, and that is a different job from telling the
// user.
function theWidthsAreUnread(said) {
  theWidthsCouldNotBeRead = true;
  document.documentElement.dataset.panes = "unread";
  if (!app || !said) return;
  // **No controls**: the shape draws the list it is handed, there is nothing
  // the user can do about this, and the cross belongs to a record — there is
  // none here. The sentence has already been through the typography point.
  //
  // **It sits in a line of its own across the foot of the window**, inset like
  // any other failure. Until 2026-09-19 the shape went straight into `.app` as
  // a third child of its two-row grid: it landed as a bare line under the
  // panes with no margin, its dot cut by the window's edge — seen in a
  // throwaway copy once the Gemini catch-up of step 10a, Area 4, F-1 made a
  // record that would not open reach it.
  const held = document.createElement("div");
  held.className = "panes-unread";
  held.appendChild(
    theFailureShape({ sentence: said, controls: [], room: EXPANDED }),
  );
  app.appendChild(held);
}

// ── The floor ───────────────────────────────────────────────────────────────

// **The panel that appears below the floor and says the window is too small.**
//
// Design: *"Below that Smoozy says the window is too small rather than
// folding… So there is a floor, it is stated on screen, and it names the
// width."* The drawing turns it on at the floor's own mark; what is made here
// is the panel itself, because a piece makes its own elements in its own module
// and adds nothing to `index.html`.
//
// **It stays out of the way until it has its words.** The two sentences come
// through the typography point, which is a call to the application, so they
// arrive a moment after the module loads. `hidden` is what holds it back — the
// drawing's own `[hidden] { display: none !important; }` outranks every display
// rule, which is exactly what that line is for.
function theFloorPanelIsDrawn(floor) {
  if (!app) return null;
  const panel = document.createElement("div");
  panel.className = "toosmall";
  panel.hidden = true;

  const inside = document.createElement("div");
  inside.className = "toosmall-in";
  const said = document.createElement("p");
  said.className = "toosmall-said";
  const how = document.createElement("p");
  how.className = "toosmall-how";
  inside.appendChild(said);
  inside.appendChild(how);
  panel.appendChild(inside);
  app.appendChild(panel);

  // Prose, through the typography application point (X1), so the two
  // substitutions are already characters in these strings — X4, which is what
  // makes them survive three engines. Nothing here substitutes anything, and
  // H32: values are content, never markup.
  said.textContent = floor.said || "";
  how.textContent = floor.how || "";
  panel.hidden = false;
  return panel;
}

// ── The seams ───────────────────────────────────────────────────────────────

// **Put the widths this user has already set back on the window.**
//
// **A seam the record holds no width for is left alone**, which is the drawn
// default and not a failure: a fresh installation has dragged nothing.
function theRememberedWidthsAreApplied(seams) {
  if (!app) return;
  for (const seam of seams) {
    if (typeof seam.width === "number") {
      app.style.setProperty(seam.property, `${seam.width}px`);
    }
  }
}

// **Make every seam under `root` draggable.**
//
// **This is the service every screen calls when it draws its own panes**, and
// it is written that way rather than as a one-off because the rail's seam is on
// every screen and a list's seam is on two.
//
// A seam element is a button carrying `data-seam` with the key its width is
// stored under. Everything else about it — the property it moves, its two
// stops, which edge it is measured from — comes from the application, which is
// where the closed set lives.
//
// **Three rules hold it together and none of them is optional** (Design, *Every
// seam between two panes drags*):
//
//  - **Each pane has a smallest and a largest useful width and the drag stops
//    at both.** A pane cannot be dragged into uselessness and then have to be
//    hunted for; past its largest width a pane is only taking room from the
//    thing beside it.
//  - **The narrowing steps still govern underneath.** The dragged width is set
//    on `.app`; each step re-declares the same property on `.body`, so from
//    that step's width down the step wins and the user's width is set aside
//    rather than destroyed. Widen the window and it comes back.
//  - **Double-clicking a seam puts it back where it started**, and it is the
//    only reset there is.
export function theSeamsAreLive(root, seams) {
  for (const element of root.querySelectorAll(".seam")) {
    const declared = seams.find((seam) => seam.key === element.dataset.seam);
    if (declared) oneSeamDrags(element, declared);
  }
}

function oneSeamDrags(element, declared) {
  const host = element.parentElement;
  if (!host) return;
  let dragging = false;
  // **The width this drag has actually moved the pane to**, and nothing until
  // the pointer has moved. A press that moves nothing is not a drag, and P11's
  // write happens at the moment a drag ends and at no other moment — so a
  // click, or either half of the double-click that resets the seam, must write
  // nothing at all.
  let width = null;

  element.addEventListener("pointerdown", (pointer) => {
    // **The seams and the steps agree about which step is in force at the
    // moment a drag begins**, and this is where that is made true. The mark is
    // written by a listener rather than by the engine, so the attribute on
    // `.app` can be one resize behind; asking here rather than trusting it is
    // what closes the seam the ruling names. It is the same one function the
    // mark writer calls, so there is one set of boundaries and not two.
    const marks = theNarrowingIsMarked();
    if (marks.includes(THE_STEP_THAT_STOPS_THE_SEAMS)) return;

    pointer.preventDefault();
    try {
      element.setPointerCapture(pointer.pointerId);
    } catch (trouble) {
      // No live pointer to capture. The drag still works from the events
      // themselves; capture is what keeps it working past the seam's own edge.
    }
    dragging = true;
    width = null;
    element.classList.add("dragging");
    document.body.classList.add("resizing");
  });

  element.addEventListener("pointermove", (pointer) => {
    if (!dragging) return;
    const box = host.getBoundingClientRect();
    const across = declared.from_right
      ? box.right - pointer.clientX
      : pointer.clientX - box.left;
    // **The drag stops at both ends** — Design's *a pane cannot be dragged into
    // uselessness*. The application clamps again when the width is written
    // down, because a command is reachable without this arithmetic.
    width = Math.max(
      declared.smallest,
      Math.min(declared.largest, Math.round(across)),
    );
    // **The dragged width goes on `.app` and never on the host**, and that is
    // the whole trick: each narrowing step re-declares the same property on
    // `.body`, so from that step's width down the step wins and the user's
    // width is set aside rather than overwritten.
    app.style.setProperty(declared.property, `${width}px`);
  });

  function theDragEnded() {
    if (!dragging) return;
    dragging = false;
    element.classList.remove("dragging");
    document.body.classList.remove("resizing");
    if (width === null) return;
    // **Nothing is written where the widths could not be read** — F-6. The
    // record may still hold this user's own widths, and a write here would take
    // them from unread to gone. The pane moved on screen and stays where the
    // user put it for this run; what is refused is only the remembering.
    if (theWidthsCouldNotBeRead) return;
    // **P11: a dragged pane width is durable state, written at the moment the
    // drag ends, and the user's own drag is its only writer.** The release is
    // the fact becoming true, so a crash costs no width already set — and
    // nothing is written while the pointer is moving and nothing on the way
    // out.
    shell.core.invoke("a_pane_drag_ended", { seam: declared.key, width });
  }

  element.addEventListener("pointerup", theDragEnded);
  element.addEventListener("pointercancel", theDragEnded);

  // The one reset there is. **It writes nothing where the widths could not be
  // read either** — clearing a value is as much a write over what may still be
  // there as setting one (F-6).
  element.addEventListener("dblclick", () => {
    width = null;
    app.style.removeProperty(declared.property);
    if (theWidthsCouldNotBeRead) return;
    shell.core.invoke("a_pane_width_was_reset", { seam: declared.key });
  });
}

// ── What was here, and where it went ────────────────────────────────────────
//
// **This file's own proof surface stood below this line until 2026-09-13**, when
// step 10a piece 2 took it down. It drew the window's shape with its panes
// empty — a rail, a list and a preview column with the three seams between
// them — so that this piece could be watched working before any screen existed.
// **The shell replaced it**: `web/the-rail.js` draws the real rail and the real
// main pane, makes its own seam live through `theSeamsAreLive` above, and its
// head carries the contract every screen mounts by.
//
// **Everything above this line stays and is the service every screen calls.**
// The step mark, the floor panel, the remembered widths and `theSeamsAreLive`
// are the window's own rather than a screen's, which is why they never waited
// on an event and still do not.

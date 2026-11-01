// **The machine's own words, on the page** — Desktop app and delivery §16.3 and
// §16.4, and Preparation/Design/Whole app.md's panel.
//
// **READ THIS BEFORE DRAWING A RENDER'S WORDS ANYWHERE.** It is the page half of
// the rules at the heads of `src/presentation/the_machines_own_words.rs` and
// `src/presentation/the_disk_full_carve_out.rs`, and the eleven screens after
// this piece draw inside it rather than beside it.
//
// ## The order is M8 and it is held by where the panel lives
//
// **The plain sentence is first and the raw record is behind it, never the first
// thing in front of the user.** The panel is made `hidden` and the only thing
// that opens it is the failure shape's own raw-text control. `src/surface/
// the_machines_own_words.rs` has the test that reads this file for both halves
// of that, because a panel drawn open is the one ordering M8 forbids and it
// would look perfectly reasonable.
//
// ## The one thing this file must never do, and the drawing does it twice
//
// **M13b: no line of the ordinary stream is lifted out of it.** *"No excerpt, no
// last line, no highlight, no leading summary. The last line is a sampling line,
// and setting it beside a plain sentence about a failure asserts a cause by
// placement that no record carries."*
//
// **`Preparation/Design/Mockup/Smoozy.html` lines 5476 and 5514 draw exactly
// that** — a `.log .last` span holding one line of a stream, directly under the
// plain sentence. **The clause wins** (Rule 10; What Build inherits item 5), so
// that span does not exist here and there is nothing on the event it could be
// built from: the record travels whole or not at all. The sweep in the Rust
// half fails on the habit rather than trusting this comment.
//
// ## M9: nothing here edits either stream
//
// Not a redaction, not a filter, not a prefix, not a trim. The record arrives as
// one string and is set as one text node. **Nothing here counts its lines
// either** — Design permits a count and no supplier hands one over, so the count
// is the machine's to report beside the outcome rather than this file's to
// derive by taking the record apart.
//
// ## M10 and M10a: five readings and none of them is an empty box
//
// Every tab draws either the record or at least one sentence. The five are the
// application's to decide and arrive on the event as a token; this file picks a
// shape with the token and draws the sentences it is given.
//
// ## M12: no drawing here reads a status as a claim about what was written
//
// Twenty endings report success and six of them end at 0 with an empty output
// folder. **No exit status reaches this side at all**, so the word is not
// reachable from one.
//
// ## Not one word of prose is written in this file
//
// A6 and X1: the typography point is the only way prose reaches a screen, and
// the enforcement is **structural on both sides**. Until 2026-09-12 this file
// wrote the panel's note, its Close control, *as of … on …*, *kept*, *dropped*,
// *free* and the folder's own name itself — and the note was two sentences in
// one string, which is the narrowing §22 item 28 names, in a file whose
// application half hands each sentence over separately on purpose. **Every one
// of them now arrives on the event as prose**, and the sweep at
// `src/surface/mod.rs` is what says so. Found at round 1 of the code audit of
// what is drawn inside the window (F-6).
//
// H32: values are content, never markup — `textContent` and `createElement`,
// never `innerHTML`. H33: no inline style; the rules are in the drawing's own
// lifted sheet and in `the-machines-own-words.css`.

import { whenTold } from "./event-route.js";
import {
  theFailureShape,
  theControlsFrom,
  EXPANDED,
} from "./the-failure-shape.js";

// The kind, declared beside the service that posts it —
// `A_RENDERS_WORDS_ARE_STANDING` in `src/surface/the_machines_own_words.rs`. The
// two sides have to agree and nothing holds them together, so copy carefully.
const A_RENDERS_WORDS_ARE_STANDING = "presentation:a-renders-words-are-standing";

// The two streams' own key sets, one each, exactly as the Rust side declares
// them. **They share no key**, which is M13a held by the transport: the ordinary
// stream's record has nowhere to go but its own place.
const THE_ORDINARY_STREAMS_KEYS = {
  tab: "theOutputTab",
  says: "theOutputSays",
  said: "theOutputSaid",
  saidNext: "theOutputSaidNext",
  record: "theOutputRecord",
  kept: "theOutputKept",
  dropped: "theOutputDropped",
  whyTheRestWent: "theOutputWhyTheRestWent",
};

const THE_COMPLAINTS_STREAMS_KEYS = {
  tab: "theErrorsTab",
  says: "theErrorsSays",
  said: "theErrorsSaid",
  saidNext: "theErrorsSaidNext",
  record: "theErrorsRecord",
  kept: "theErrorsKept",
  dropped: "theErrorsDropped",
  whyTheRestWent: "theErrorsWhyTheRestWent",
};

const BOTH_STREAMS = [THE_ORDINARY_STREAMS_KEYS, THE_COMPLAINTS_STREAMS_KEYS];

/// **Build one tab's body.**
///
/// `tab` is `{ name, says, said, saidNext, record, kept, dropped,
/// whyTheRestWent, notShown, asPrinted }`, every string already on one of the
/// two routes.
function theTabsBody(tab) {
  const body = document.createElement("div");
  body.className = "words-body";
  body.dataset.stream = tab.name;

  // The sentence, and the second one where there is one. The drawing puts the
  // first in bold and the rest beside it, and each arrived as its own string —
  // X2 and X10.
  if (tab.said) {
    const said = document.createElement("p");
    said.className = tab.says === "only-part-was-recorded" ? "lstate" : "lnone";
    const opening = document.createElement("b");
    opening.textContent = tab.said;
    said.appendChild(opening);
    if (tab.saidNext) {
      said.appendChild(document.createTextNode(" " + tab.saidNext));
    }
    if (tab.kept || tab.dropped || tab.whyTheRestWent) {
      // **How much was kept and why the rest went** — Design. **The figures are
      // values and the words beside them are prose**, and the two are separate
      // nodes rather than one string: joining them here would undo the split the
      // application side made for exactly this (X7).
      const counts = document.createElement("span");
      counts.className = "words-counts";
      const beside = (figure, word) => {
        const one = document.createElement("span");
        one.className = "words-count";
        const many = document.createElement("span");
        many.textContent = bytes(figure);
        one.appendChild(many);
        // **The figure and its word are bound** (`Design/Whole app §
        // Typography`).
        one.appendChild(document.createTextNode(BETWEEN_A_FIGURE_AND_ITS_UNIT));
        const said = document.createElement("span");
        said.textContent = word || "";
        one.appendChild(said);
        return one;
      };
      if (tab.kept) counts.appendChild(beside(tab.kept, tab.theKeptWord));
      if (tab.dropped) {
        counts.appendChild(beside(tab.dropped, tab.theDroppedWord));
      }
      if (tab.whyTheRestWent) {
        const why = document.createElement("span");
        why.className = "words-count";
        why.textContent = tab.whyTheRestWent;
        counts.appendChild(why);
      }
      said.appendChild(counts);
    }
    body.appendChild(said);
  }

  // The record. **Whole, unedited, in one text node** — M9, and nothing is
  // taken out of it — M13b.
  if (typeof tab.record === "string") {
    const printed = document.createElement("pre");
    printed.className = "lbody";
    printed.textContent = tab.record;
    // **Where what is shown is not every byte as printed** — a line a program
    // rewrote in place, shown in its newest form (`Desktop app and delivery`
    // M13b, from 2026-10-24) — **the words as printed ride beside it, and they
    // are what Copy takes**. Added 2026-10-25, the eighteenth notes, piece 2c.
    if (typeof tab.asPrinted === "string") printed.dataset.asPrinted = tab.asPrinted;
    body.appendChild(printed);
  }

  // **Under the words, where some of what was printed could not be brought
  // back** — `Design/Server Setup § A long step`, added 2026-10-24: in the
  // panel's own explanatory style and outside the record, so Copy, which takes
  // the record alone, never takes it. The sentence is the application's.
  if (tab.notShown) {
    const notShown = document.createElement("p");
    notShown.className = "lnone words-not-shown";
    const sentence = document.createElement("b");
    sentence.textContent = tab.notShown;
    notShown.appendChild(sentence);
    body.appendChild(notShown);
  }
  return body;
}

// ── The panel opens at the end, and holds there ─────────────────────────────
//
// **`Design/Whole app`, the raw-text panel: *"The panel opens at the end.
// While the reader is at the end, new lines keep it there; scrolling up lets
// go; scrolling back to the end holds it again."*** History's panel included,
// where the end is where the render stopped. The Creator's notes on the test
// copy, piece 8 (note 12), and the map's hole 9: **it lets go only at a scroll
// up of more than one line**, and holds again within a line of the end.
//
// **Each tab's body is its own scroller**, and each holds the end on its own.
// A body that comes into view — the panel opened, or its tab chosen — goes to
// its end and holds it, whoever opened it, so the three places that open a
// panel need do nothing for it. Nothing here touches the record (M9).

const theEndIsHeld = new WeakMap();
const theBodyWasShown = new WeakMap();

/// **One line of the record**, in pixels: the drawing's own line height for
/// `.lbody`, read where a page has styles, and a fixed figure otherwise.
function oneLineOf(body) {
  const printed = body.querySelector("pre.lbody");
  const style = printed && typeof getComputedStyle === "function" ? getComputedStyle(printed) : null;
  const height = style ? parseFloat(style.lineHeight) : NaN;
  return Number.isFinite(height) && height > 0 ? height : 24;
}

function theGapToTheEnd(body) {
  return body.scrollHeight - body.scrollTop - body.clientHeight;
}

function theEndIsHeldIn(body) {
  theEndIsHeld.set(body, true);
  body.addEventListener("scroll", () => {
    theEndIsHeld.set(body, theGapToTheEnd(body) <= oneLineOf(body));
  });
  if (typeof ResizeObserver !== "function") return;
  new ResizeObserver(() => {
    const shown = body.clientHeight > 0;
    const wasShown = theBodyWasShown.get(body) === true;
    theBodyWasShown.set(body, shown);
    if (!shown) return;
    if (!wasShown) theEndIsHeld.set(body, true);
    if (theEndIsHeld.get(body)) body.scrollTop = body.scrollHeight;
  }).observe(body);
}

/// **Words just added to `body` stay in view while it holds the end**, and a
/// reader who scrolled up is left where they are. The live panel calls it
/// after every arrival (`web/the-queue.js`).
function theCopyVisibilityIsUpdated(body) {
  const frame = body.closest && body.closest(".lwords");
  const copy = frame && frame.querySelector(".words-copy");
  if (!copy) return;
  const open = Array.from(frame.querySelectorAll(".words-body")).find(one => !one.hidden);
  const printed = open && open.querySelector("pre.lbody");
  copy.hidden = !printed || !printed.textContent.length;
  frame.classList.toggle("withcopy", !copy.hidden);
}

export function theNewestWordsStayInView(body) {
  theCopyVisibilityIsUpdated(body);
  if (theEndIsHeld.get(body) !== false) body.scrollTop = body.scrollHeight;
}

// ── The copy icon sits in the inner box's own corner ───────────────────────
//
// **The same gap from the right as from the bottom, counted from the box's
// visible inside** — inside its border, clear of any scroll bar, and clear of
// the part of the box scrolled out of view. The Creator's seventh notes, note
// 7: *"Just add an equal margin from the right and the bottom. But not from the
// pop-up window itself — from the inner window."* Where a scroll bar shows, and
// how wide it is, is only known once the page is drawn, so the place is
// measured and handed to the stylesheet's `.lwords .lcopy` as two figures; the
// stylesheet keeps a place of its own for a page that never measures. Measured
// again whenever the box changes size, a tab is chosen, or the words scroll.

/// The gap, in pixels, from the box's visible inside to the icon, on both sides.
const THE_COPY_ICONS_GAP = 12;

function theCopyIconIsPlacedIn(frame) {
  const place = () => {
    if (typeof frame.getBoundingClientRect !== "function" || !frame.style) return;
    const shown = Array.from(frame.querySelectorAll("pre.lbody")).find(
      (one) => one.getClientRects().length > 0,
    );
    if (!shown) {
      frame.style.removeProperty("--lcopy-right");
      frame.style.removeProperty("--lcopy-bottom");
      return;
    }
    const box = shown.getBoundingClientRect();
    let right = box.left + shown.clientLeft + shown.clientWidth;
    let bottom = box.top + shown.clientTop + shown.clientHeight;
    for (let up = shown.parentElement; up && up !== frame; up = up.parentElement) {
      const clip = up.getBoundingClientRect();
      right = Math.min(right, clip.left + up.clientLeft + up.clientWidth);
      bottom = Math.min(bottom, clip.top + up.clientTop + up.clientHeight);
    }
    const at = frame.getBoundingClientRect();
    frame.style.setProperty("--lcopy-right", at.right - right + THE_COPY_ICONS_GAP + "px");
    frame.style.setProperty("--lcopy-bottom", at.bottom - bottom + THE_COPY_ICONS_GAP + "px");
  };
  if (typeof frame.addEventListener === "function") frame.addEventListener("scroll", place, true);
  if (typeof ResizeObserver !== "function") return;
  const watch = new ResizeObserver(place);
  watch.observe(frame);
  frame.querySelectorAll(".words-body").forEach((one) => watch.observe(one));
  frame.querySelectorAll("pre.lbody").forEach((one) => watch.observe(one));
}

/// **The panel behind the control** — one tab per stream, apart, never merged.
///
/// It is made hidden. **The control is the only thing that opens it** (M8).
export function theWordsPanel(parts) {
  const panel = document.createElement("div");
  panel.className = "modal top words-panel";
  panel.hidden = true;

  const box = document.createElement("div");
  box.className = "lbox";

  const head = document.createElement("div");
  head.className = "lhead";
  const title = document.createElement("h3");
  title.textContent = parts.title || "";
  head.appendChild(title);
  if (parts.subtitle) {
    const under = document.createElement("span");
    under.className = "lsub mono";
    under.textContent = parts.subtitle;
    head.appendChild(under);
  }
  box.appendChild(head);

  const bodies = [];
  for (const tab of parts.tabs) bodies.push(theTabsBody(tab));

  // **A render has two streams and they are kept apart, so it has two tabs;
  // anything with one stream has none** — Design.
  if (parts.tabs.length > 1) {
    const strip = document.createElement("div");
    strip.className = "ltabs";
    parts.tabs.forEach((tab, at) => {
      const control = document.createElement("button");
      control.type = "button";
      control.className = "ltab";
      control.setAttribute("aria-selected", String(at === 0));
      control.textContent = tab.name;
      control.addEventListener("click", () => {
        strip.querySelectorAll(".ltab").forEach((one, which) => {
          one.setAttribute("aria-selected", String(which === at));
        });
        bodies.forEach((body, which) => {
          body.hidden = which !== at;
        });
        theCopyVisibilityIsUpdated(bodies[at]);
      });
      strip.appendChild(control);
    });
    box.appendChild(strip);
  }

  // **The words sit in a frame that does not scroll**, the drawing's `.lwords`.
  // Each tab's body is the scroller and the frame holds them, so the copy icon
  // in the frame's bottom-right corner stays in place while the words scroll under
  // it. Queue's live panel swaps what is inside a body as words arrive, which
  // is one more reason the icon lives beside the bodies and not in one.
  const words = document.createElement("div");
  words.className = parts.copy ? "lwords withcopy" : "lwords";

  // **A copy icon in the bottom-right corner of the words, where the caller hands
  // its word** — `Design/Queue & History`, the Creator's third notes, note 13,
  // and fourth notes, note 2: *"a copy icon right in the window, not up top as
  // a text button."* The Files toolbar's own icon, with the word as its hover
  // name and its accessible name. The caller supplies the word where its
  // Design offers Copy, including server output in Queue and History. The
  // selected tab must hold actual printed text before the icon is shown.
  //
  // **The open tab is read at the press**, not when the panel is made, so a tab
  // chosen after opening is the one copied; and the whole printed record is
  // read, so words Queue's live panel added since opening are in it. Nothing is
  // edited on the way (M9).
  if (parts.copy) {
    const copy = document.createElement("button");
    copy.type = "button";
    copy.className = "ib lcopy words-copy";
    copy.setAttribute("aria-label", parts.copy);
    copy.title = parts.copy;
    copy.appendChild(theCopyIcon());
    const name = document.createElement("span");
    name.className = "ibl";
    name.textContent = parts.copy;
    copy.appendChild(name);
    copy.addEventListener("click", () => {
      const open = bodies.find((body) => !body.hidden);
      const printed = open && open.querySelector("pre.lbody");
      // **The words as printed where they ride beside what is shown** (M9),
      // and otherwise what is shown, which is then the same thing.
      const asPrinted = printed && printed.dataset ? printed.dataset.asPrinted : undefined;
      theClipboardTakes(printed ? (typeof asPrinted === "string" ? asPrinted : printed.textContent) : "");
    });
    words.appendChild(copy);
  }

  bodies.forEach((body, at) => {
    body.hidden = at !== 0;
    theEndIsHeldIn(body);
    words.appendChild(body);
  });
  if (parts.copy) {
    if (bodies.length) theCopyVisibilityIsUpdated(bodies[0]);
    theCopyIconIsPlacedIn(words);
    if (typeof MutationObserver === "function") {
      new MutationObserver(() => theCopyVisibilityIsUpdated(bodies[0])).observe(words, {childList:true, subtree:true, characterData:true});
    }
  }
  box.appendChild(words);

  const foot = document.createElement("div");
  foot.className = "lfoot";
  const note = document.createElement("span");
  note.className = "lnote";
  // One sentence since 2026-10-28; *Not analysed for a cause.* went with the
  // words sweep's loose ends.
  note.appendChild(document.createTextNode(parts.note || ""));
  foot.appendChild(note);
  const grow = document.createElement("span");
  grow.className = "grow";
  foot.appendChild(grow);
  const close = document.createElement("button");
  close.type = "button";
  close.className = "btn quiet";
  close.textContent = parts.close || "";
  close.addEventListener("click", () => {
    panel.hidden = true;
  });
  foot.appendChild(close);
  box.appendChild(foot);

  panel.appendChild(box);
  return panel;
}

// ── The copy icon ───────────────────────────────────────────────────────────
//
// **A second copy of the Files toolbar's copy icon**, `THE_TOOL_ICONS.copy` and
// `anIcon` in `web/files.js`, made the same way from the drawing's own
// `<symbol id="i-copy">` path data, unchanged: the 16-pixel box, outline only,
// stroke 1.3. Sharing one would need a file of its own outside this piece's
// territory (the Creator's fourth notes, piece N). A change to the shape
// belongs in both.

const SVG_NS = "http://www.w3.org/2000/svg";

function theCopyIcon() {
  const svg = document.createElementNS(SVG_NS, "svg");
  svg.setAttribute("viewBox", "0 0 16 16");
  svg.setAttribute("aria-hidden", "true");
  svg.setAttribute("fill", "none");
  svg.setAttribute("stroke", "currentColor");
  svg.setAttribute("stroke-width", "1.3");
  svg.setAttribute("stroke-linejoin", "round");
  const box = document.createElementNS(SVG_NS, "rect");
  for (const [name, value] of [["x", "5.5"], ["y", "5.5"], ["width", "8.2"], ["height", "8.2"], ["rx", "1.1"]]) {
    box.setAttribute(name, value);
  }
  svg.appendChild(box);
  const behind = document.createElementNS(SVG_NS, "path");
  behind.setAttribute("d", "M10.5 2.3H3.4a1.1 1.1 0 0 0-1.1 1.1v7.1");
  svg.appendChild(behind);
  return svg;
}

// ── The clipboard ───────────────────────────────────────────────────────────
//
// **A second copy of `theClipboardTakes` in `web/keys.js`**, kept the same on
// purpose: sharing one would need a file of its own outside this piece's
// territory (the Creator's third notes, piece C). A repair to either belongs
// in both. The clipboard is the page's because the application binds no
// clipboard service; where the engine refuses the first way, the second is
// tried, and the answer says whether either reported success.

function theClipboardTakes(contents) {
  if (!contents) return Promise.resolve(false);
  if (navigator.clipboard && navigator.clipboard.writeText) {
    return navigator.clipboard
      .writeText(contents)
      .then(() => true)
      .catch(() => theClipboardTakesItTheOtherWay(contents));
  }
  return Promise.resolve(theClipboardTakesItTheOtherWay(contents));
}

function theClipboardTakesItTheOtherWay(contents) {
  const held = document.createElement("textarea");
  held.className = "words-away";
  held.value = contents;
  document.body.appendChild(held);
  held.select();
  let copied = false;
  try {
    copied = document.execCommand("copy") === true;
  } catch (trouble) {
    console.error(trouble);
  }
  held.remove();
  return copied;
}

/// **A reading, as of the moment it was taken, on the clock it was taken on** —
/// M3a.
///
/// **The moment goes on its own line.** A moment run on to the end of a figure
/// reads as part of the figure; piece E watched that happen and fixed it, and
/// this is the same shape. The clock's name is the server's own, so a reading
/// taken there never reads as one taken here.
export function theReadingAsOfThatMoment(figure, takenAt, onTheClock, words) {
  const said = document.createElement("p");
  said.className = "rqs words-reading";
  const value = document.createElement("span");
  value.textContent = figure;
  said.appendChild(value);
  // **The word saying what the figure measures, as its own node and never
  // joined into the figure's string** (X7). It arrives as prose. Until
  // 2026-09-19 the application sent it and nothing drew it, so the reading was
  // a bare number; the Gemini catch-up of step 10a, Area 4 (F-4).
  if (words && words.whatItMeasures) {
    said.appendChild(document.createTextNode(" "));
    const measures = document.createElement("span");
    measures.className = "words-part";
    measures.textContent = words.whatItMeasures;
    said.appendChild(measures);
  }
  if (takenAt) {
    // **Every word here arrived as prose and every value beside it as a value**
    // — they are separate nodes and are never joined into one string (X7).
    const asOf = document.createElement("span");
    asOf.className = "sub";
    for (const part of [
      (words && words.asOf) || "",
      theMoment(takenAt),
      (words && words.onTheClock) || "",
      onTheClock || "",
    ]) {
      const one = document.createElement("span");
      one.className = "words-part";
      one.textContent = part;
      asOf.appendChild(one);
      asOf.appendChild(document.createTextNode(" "));
    }
    said.appendChild(asOf);
  }
  return said;
}

/// **The moment, with its date and the zone it is drawn in.**
///
/// **A bare time of day was a plain falsehood and it is the one format M3a
/// exists to stop.** A reading taken at 14:32 on a rented machine in another
/// region read as *09:32* beside that machine's own name, which is neither that
/// machine's clock nor labelled as the reader's; and because a live condition
/// stands until the user clears it and a render's ending stands for good, the
/// reading beside it could be days old and still read as a time today. Corrected
/// 2026-09-12 at round 1 of the code audit of what is drawn inside the window
/// (F-9).
///
/// **Nothing is converted into the machine's own zone**: nothing here knows its
/// offset, and inventing one would be worse than either. The clock's own name
/// stays beside the reading as the name of what it is about.
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
  return (at === 0 ? left : left.toFixed(1)) + BETWEEN_A_FIGURE_AND_ITS_UNIT + steps[at];
}

// ── The plumbing's own drawing, which is this piece's proof ─────────────────
//
// **A later piece takes this pane**, exactly as it takes piece C's and piece
// E's. Nothing else in the application refers to this block; what is worth
// keeping when it goes is everything above it.
//
// **The free-space figure is drawn here rather than inside the failure shape**,
// and that is Design's placement rather than a shortcut: the entry already draws
// the server's readings in its own block, and the failure carries the second
// sentence alone. What this block adds is somewhere to watch M3a working before
// that entry exists.

const drawn = new Map();

function where() {
  let held = document.getElementById("the-machines-own-words");
  if (held) return held;
  const pane = document.getElementById("pane");
  if (!pane) return null;
  held = document.createElement("div");
  held.id = "the-machines-own-words";
  held.className = "machines-own-words";
  pane.appendChild(held);
  return held;
}

function theTabsFrom(said) {
  const tabs = [];
  for (const keys of BOTH_STREAMS) {
    if (!said[keys.says]) continue;
    tabs.push({
      name: said[keys.tab],
      says: said[keys.says],
      said: said[keys.said],
      saidNext: said[keys.saidNext],
      record: said[keys.record],
      kept: said[keys.kept],
      dropped: said[keys.dropped],
      whyTheRestWent: said[keys.whyTheRestWent],
      theKeptWord: said.theKeptWord,
      theDroppedWord: said.theDroppedWord,
    });
  }
  return tabs;
}

whenTold(A_RENDERS_WORDS_ARE_STANDING, (delivery) => {
  const held = where();
  if (!held) return;
  const said = delivery.said || {};

  const one = document.createElement("div");
  one.className = "machines-own-words-one";

  const panel = theWordsPanel({
    title: said.theRawTextLabel || "",
    subtitle: said.subject || "",
    tabs: theTabsFrom(said),
    note: said.thePanelNote,
    close: said.theCloseControl,
  });

  const values = [];
  if (said.theFolderTheReadingWasTakenAgainst) {
    // **The name is prose off the event and the path is a value** — this file
    // used to write the name itself.
    values.push([
      said.theFolderLabel || "",
      said.theFolderTheReadingWasTakenAgainst,
    ]);
  }

  const shape = theFailureShape({
    ...said,
    at: said.at ? Number(said.at) : undefined,
    values,
    room: EXPANDED,
    // **The controls as the application composed them** — F-13. Pressing the
    // raw-text one is what opens the panel (M8).
    controls: theControlsFrom(said),
    onAct: () => {
      panel.hidden = false;
    },
    // **No `onReport`: the shape's own *Report a problem…* opens the report
    // box** with M33's handover (`web/the-failure-shape.js`).
  });

  one.appendChild(shape);
  // **M15 and M3a: the carried reading, with its own moment and the server's
  // own name.** It is never re-drawn as current and never aged into a newer
  // figure — there is one figure and it arrived with its moment.
  if (said.theFreeSpace) {
    one.appendChild(
      theReadingAsOfThatMoment(
        bytes(said.theFreeSpace),
        said.theFreeSpaceTakenAt,
        said.theFreeSpaceOnTheClock,
        {
          whatItMeasures: said.theFreeSpaceWord,
          asOf: said.theReadingAsOf,
          onTheClock: said.theReadingOnTheClock,
        },
      ),
    );
  }
  one.appendChild(panel);

  // **Keyed on the presentation key, which is the one thing that cannot
  // collide** — the same repair piece C's own file took (F-5). Two jobs that
  // both ended badly for the same reason carry the same mark and would collide
  // on one entry.
  const already = drawn.get(said.key);
  if (already) {
    already.replaceWith(one);
  } else {
    held.appendChild(one);
  }
  drawn.set(said.key, one);
});

// The page asks; the answers arrive as events. A command is the page asking.
export function theMachinesOwnWordsFor(subjectKind, subject) {
  return window.__TAURI__.core.invoke("the_machines_own_words_for", {
    subjectKind,
    subject,
  });
}

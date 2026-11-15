// **The report box** — Reporting a problem §7 and §8, drawn here, with its
// application half at `src/surface/reporting.rs`. Built at step 11's piece R4.
//
// **One box, in the readings the drawing draws** — `Smoozy.html`'s `#rpbox`
// and its `rp-…` states: asking (the line naming the failure only where a
// failure opened it; the folder, or *No folder chosen* where the system
// proposes none); while the report is prepared (the field and *Choose…*
// locked, *Save report* underway, *Cancel* live, and nothing saying a file
// exists); and saved, with the folder opened or with the folder that did not
// open. **This file draws whichever reading the application says**, and
// decides none of them: the reading arrives on the event (P6).
//
// **Every word arrives on the event and none is written here** — A6 and X1,
// and the sweep at `src/surface/mod.rs` that holds it. Names, paths, the
// address and the moment arrive by the verbatim route and are drawn beside the
// words, never inside them (X7).
//
// **The two ways in.** The Smoozy menu's *Report a problem…* opens the box
// from the application side, with nothing handed over. **Every failure's
// *Report a problem…* calls `theReportBoxOpensFrom`** with M33's handover —
// the opener lives in `web/the-failure-shape.js`, beside the shape that draws
// the control, and `web/the-history.js` and `web/the-machines-own-words.js`
// take it from there — and the application judges it, maps it to its slice
// and opens the box.
//
// **What the page does itself, and it is only what a page must**: it holds the
// user's own sentence in the field until the press, sends it once with the
// zone this computer is in (the file's name is in the user's own time), and
// hides the box at *Cancel* or *Close* while telling the application.
//
// **The address is a mail link, with the copy icon right beside it** — the
// Creator's twenty-eighth notes, note 6, `Reporting a problem` V10. A press of
// the address asks the application through a closed command, which opens K25's
// fixed mail link; the page names no address. The copy icon puts the bare
// address on the clipboard and its label says what came of it.
//
// H32: values are content, never markup — `textContent`, `createElement`.
// H33: no inline style; the rules are the drawing's own (`the-drawing.css`'s
// `.rp-` rules) and `reporting.css`.

import { whenTold } from "./event-route.js";
import { theFailureShape, EXPANDED } from "./the-failure-shape.js";
import { theValueIsDrawnInto } from "./the-value-drawn.js";
import { theFocusMovesTo } from "./the-focus-moves.js";

const shell = window.__TAURI__;

// The kind, declared beside the service that posts it — `THE_REPORT_BOX` in
// `src/surface/reporting.rs`. The two have to agree.
const THE_REPORT_BOX = "reporting:the-report-box";

// What the application answers about the mail link — `TheMailLink` in
// `src/surface/reporting.rs`.
const THE_MAIL_LINK_DID_NOT_OPEN = "ItCouldNotBeOpened";

// The readings, as the application names them.
const COMPOSING = "composing";
const SAVED = "saved";
const NOT_SAVED = "not-saved";

// The saved sentence's parts, at most — `THE_SAVED_PARTS` in
// `src/surface/reporting.rs`.
const SAVED_PARTS = 5;

// The paragraph under the field, one sentence to a key (X10).
const WHAT_THE_FILE_HOLDS = ["whatTheFileHolds0", "whatTheFileHolds1", "whatTheFileHolds2"];

// The one box, made the first time the application says it and kept.
let theBox = null;
// Which opening the box last drew, so a box opened anew starts with an empty
// field and the readings of one opening keep what the user typed.
let theOpeningDrawn = null;

/// The moment the failure carried, in this computer's own time — the drawing's
/// *8 Aug 23:14*.


function theMomentDrawn(seconds) {
  const at = new Date(Number(seconds) * 1000);
  if (Number.isNaN(at.getTime())) return "";
  return at.toLocaleString([], {
    day: "numeric",
    month: "short",
    hour: "2-digit",
    minute: "2-digit",
    hourCycle: "h23",
  });
}

// **The class is set where it is named**, in a statement of its own, so the
// page's prose sweep reads it as the class list it is.
function anElement(tag, reading) {
  const one = document.createElement(tag);
  if (reading) one.dataset.rp = reading;
  return one;
}

function aButton(reading) {
  const one = anElement("button", reading);
  one.type = "button";
  return one;
}

function theBoxIsMade() {
  // **The drawing's own top box**, over the whole window, title bar included.
  const root = anElement("div");
  root.className = "modal top";
  root.id = "rpbox";
  root.hidden = true;
  const box = anElement("div");
  box.className = "mbox rp-box";
  box.setAttribute("role", "dialog");
  box.setAttribute("aria-modal", "true");
  box.tabIndex = -1;

  const heading = anElement("h3", "ask");
  heading.id = "rp-heading";
  box.setAttribute("aria-labelledby", heading.id);

  // The line naming the failure: the sentence in bold, the subject's own
  // label, the moment — the drawing's three, with its own separator.
  const about = anElement("p", "ask");
  about.className = "rp-about";
  about.id = "rpabout";
  const aboutSentence = document.createElement("b");
  const aboutLabelGap = document.createTextNode(" \u00b7 ");
  const aboutLabel = document.createElement("span");
  const aboutAtGap = document.createTextNode(" \u00b7 ");
  const aboutAt = document.createElement("span");
  about.append(aboutSentence, aboutLabelGap, aboutLabel, aboutAtGap, aboutAt);

  const label = anElement("label", "ask");
  label.className = "rp-lab";
  label.htmlFor = "rpsay";
  const field = anElement("input", "ask");
  field.className = "fld rp-say";
  field.id = "rpsay";
  field.type = "text";

  const what = anElement("p", "ask");
  what.className = "rp-what";
  const whatParts = [0, 1, 2].map(() => document.createElement("span"));
  whatParts.forEach((part, at) => {
    if (at) what.appendChild(document.createTextNode(" "));
    what.appendChild(part);
  });

  const where = anElement("div", "ask");
  where.className = "rp-where";
  const saveIn = document.createElement("span");
  const folder = anElement("span");
  folder.className = "rp-wp mono selectable";
  folder.id = "rpwp";
  const noFolder = anElement("span");
  noFolder.className = "rp-wp";
  noFolder.id = "rpwpnone";
  const choose = aButton();
  choose.className = "mini";
  choose.id = "rpchoose";
  where.append(saveIn, folder, noFolder, choose);

  // Where a write that did not land is said, in the one failure shape.
  const notSaved = anElement("div", "ask");
  notSaved.className = "rp-not-saved";

  const askActs = anElement("div", "ask");
  askActs.className = "mact";
  const cancel = aButton();
  cancel.className = "btn quiet";
  const save = aButton();
  save.className = "btn";
  save.id = "rpsave";
  askActs.append(cancel, save);

  const savedHeading = anElement("h3", "saved");
  const savedSays = anElement("p", "saved");
  savedSays.className = "msub";
  savedSays.id = "rpsaved";
  const send = anElement("p", "saved");
  send.className = "rp-send";
  // **The address as a mail link, the copy icon right beside it** (note 6).
  const addressRow = anElement("div", "saved");
  addressRow.className = "rp-addrrow";
  const address = aButton();
  address.className = "rp-addr mono";
  const copy = aCopyIcon();
  addressRow.append(address, copy.control);
  // Where the mail link could not be opened, in the one failure shape.
  const mailFailed = anElement("div", "saved");
  mailFailed.className = "rp-mail-failed";
  const savedActs = anElement("div", "saved");
  savedActs.className = "mact";
  const close = aButton();
  close.className = "btn";
  savedActs.appendChild(close);

  box.append(
    heading,
    about,
    label,
    field,
    what,
    where,
    notSaved,
    askActs,
    savedHeading,
    savedSays,
    send,
    addressRow,
    mailFailed,
    savedActs,
  );
  root.appendChild(box);
  (document.querySelector(".app") || document.body).appendChild(root);

  const shut = () => {
    root.hidden = true;
    theOpeningDrawn = null;
    shell.core.invoke("the_report_box_is_shut");
  };
  cancel.addEventListener("click", shut);
  close.addEventListener("click", shut);
  box.addEventListener("keydown", (press) => {
    if (press.key !== "Escape" || press.defaultPrevented) return;
    press.preventDefault();
    shut();
  });
  // **The mail link** — a closed command; the answer is about the ask (K25).
  address.addEventListener("click", async () => {
    let answer;
    try {
      answer = await shell.core.invoke("the_report_address_is_opened");
    } catch (trouble) {
      console.error(trouble);
      answer = THE_MAIL_LINK_DID_NOT_OPEN;
    }
    theMailLinkFailureIsSaid(answer === THE_MAIL_LINK_DID_NOT_OPEN);
  });
  // **Copy**: the bare address, and the label says what came of it until the
  // pointer or the focus leaves.
  copy.control.addEventListener("click", async () => {
    const landed = await theClipboardTakes(address.textContent);
    theCopyIconIsNamed(copy, landed ? copy.said.copied : copy.said.notCopied);
  });
  const theLabelIsReset = () => theCopyIconIsNamed(copy, copy.said.copy);
  copy.control.addEventListener("mouseleave", theLabelIsReset);
  copy.control.addEventListener("blur", theLabelIsReset);

  choose.addEventListener("click", () => {
    if (choose.disabled) return;
    shell.core.invoke("a_folder_is_chosen_for_the_report");
  });
  save.addEventListener("click", () => {
    if (save.disabled) return;
    shell.core.invoke("the_report_is_saved", {
      // **Verbatim** — the user's own sentence, untouched.
      theUsersSentence: field.value,
      utcOffsetSeconds: -new Date().getTimezoneOffset() * 60,
    });
  });

  return {
    root,
    box,
    heading,
    about,
    aboutSentence,
    aboutLabelGap,
    aboutLabel,
    aboutAtGap,
    aboutAt,
    label,
    field,
    whatParts,
    saveIn,
    folder,
    noFolder,
    choose,
    notSaved,
    cancel,
    save,
    savedHeading,
    savedSays,
    send,
    address,
    copy,
    mailFailed,
    close,
  };
}

/// **The mail link's failure, said or taken away** — the one failure shape
/// under the address, which stays (`Reporting a problem` V10).
function theMailLinkFailureIsSaid(failed) {
  const parts = theBox;
  parts.mailFailed.textContent = "";
  if (failed && parts.mailFailedSaid) {
    parts.mailFailed.appendChild(
      theFailureShape({ sentence: parts.mailFailedSaid, controls: [], room: EXPANDED }),
    );
  }
  parts.mailFailed.hidden = !(failed && parts.mailFailedSaid);
}

/// **The saved sentence, in the parts the application sent**: each words or a
/// value, the value in the drawing's fixed-width face, and a space between
/// them unless the words follow their value directly.
function theSavedSentenceIsDrawn(into, said) {
  into.textContent = "";
  const many = Math.min(Number(said.savedCount || 0), SAVED_PARTS);
  for (let at = 0; at < many; at += 1) {
    const words = said["saved" + at + "Words"];
    const value = said["saved" + at + "Value"];
    const clings = said["saved" + at + "Clings"];
    if (at && !clings) into.appendChild(document.createTextNode(" "));
    if (value) {
      const held = anElement("span");
      held.className = "mono selectable";
      theValueIsDrawnInto(held, value);
      into.appendChild(held);
    } else if (words) {
      const held = document.createElement("span");
      held.textContent = words;
      into.appendChild(held);
    }
  }
}

function theBoxIsDrawn(said) {
  if (!theBox) theBox = theBoxIsMade();
  const parts = theBox;
  const reading = said.reading || "";
  const saved = reading === SAVED;
  const composing = reading === COMPOSING;
  const fresh = said.opening !== theOpeningDrawn;
  if (fresh) {
    parts.field.value = "";
    theOpeningDrawn = said.opening;
  }

  for (const one of parts.box.querySelectorAll("[data-rp]")) {
    one.hidden = (one.dataset.rp === "saved") !== saved;
  }

  parts.heading.textContent = said.heading || "";
  // **Absent from the menu**, where there is no failure to name.
  parts.about.hidden = saved || !said.aboutSentence;
  parts.aboutSentence.textContent = said.aboutSentence || "";
  const label = said.aboutLabel || "";
  theValueIsDrawnInto(parts.aboutLabel, label);
  parts.aboutLabel.hidden = !label;
  parts.aboutLabelGap.textContent = label ? " \u00b7 " : "";
  parts.aboutAt.textContent = said.aboutAt ? theMomentDrawn(said.aboutAt) : "";
  parts.aboutAtGap.textContent = said.aboutAt ? " \u00b7 " : "";

  parts.label.textContent = said.whatWentWrong || "";
  parts.whatParts.forEach((part, at) => {
    const key = WHAT_THE_FILE_HOLDS[at];
    part.textContent = said[key] || "";
  });
  parts.saveIn.textContent = said.saveIn || "";
  // **A folder, or nothing proposed and nothing invented** — the drawing's
  // *No folder chosen*.
  theValueIsDrawnInto(parts.folder, said.folder || "");
  // **Cut at its start, the whole folder on hover** (the UI at every size,
  // E-05): its last folder is where the report goes.
  parts.folder.title = said.folder || "";
  parts.folder.hidden = saved || !said.folder;
  parts.noFolder.textContent = said.noFolderChosen || "";
  parts.noFolder.hidden = saved || Boolean(said.folder);
  parts.choose.textContent = said.choose || "";
  parts.cancel.textContent = said.cancel || "";
  parts.save.textContent = said.save || "";

  // **While the report is prepared**: the field and *Choose…* locked, *Save
  // the report* underway in the drawing's own shape, and *Cancel* live.
  parts.field.readOnly = composing;
  parts.choose.disabled = composing;
  parts.save.disabled = composing;
  parts.save.classList.toggle("wk", composing);
  if (composing) parts.save.setAttribute("aria-label", said.preparing || "");
  else parts.save.removeAttribute("aria-label");

  parts.notSaved.textContent = "";
  if (reading === NOT_SAVED && said.notSaved) {
    parts.notSaved.appendChild(
      theFailureShape({ sentence: said.notSaved, controls: [], room: EXPANDED }),
    );
  }
  parts.notSaved.hidden = saved || reading !== NOT_SAVED;

  parts.savedHeading.textContent = said.savedHeading || "";
  theSavedSentenceIsDrawn(parts.savedSays, said);
  parts.send.textContent = said.attachIt || "";
  theValueIsDrawnInto(parts.address, said.address || "");
  parts.copy.said = {
    copy: said.copy || "",
    copied: said.copied || "",
    notCopied: said.notCopied || "",
  };
  theCopyIconIsNamed(parts.copy, parts.copy.said.copy);
  parts.mailFailedSaid = said.mailLinkFailed || "";
  // A new reading starts with no failure said.
  theMailLinkFailureIsSaid(false);
  parts.close.textContent = said.close || "";

  const wasHidden = parts.root.hidden;
  parts.root.hidden = false;
  if (saved) theFocusMovesTo(parts.close);
  else if (fresh || wasHidden) theFocusMovesTo(parts.field);
}

// ── The copy icon ───────────────────────────────────────────────────────────
//
// **A fourth copy of the output panel's copy icon** (`theCopyIcon` in
// `web/the-machines-own-words.js`, and `web/telegram.js`'s third), made the
// same way from the drawing's own `<symbol id="i-copy">` path data. Sharing one
// would need a file of its own outside this piece's territory, as the second
// copy's own note says. A change to the shape belongs in all four.

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

/// **The drawing's icon button**: the icon, and its label shown on hover and
/// focus (`.ib`, `.ibl`). The label arrives as prose off the event.
function aCopyIcon() {
  const control = aButton();
  control.className = "ib";
  control.appendChild(theCopyIcon());
  const name = anElement("span");
  name.className = "ibl";
  control.appendChild(name);
  return { control, name, said: { copy: "", copied: "", notCopied: "" } };
}

function theCopyIconIsNamed(icon, name) {
  icon.control.setAttribute("aria-label", name || "");
  icon.name.textContent = name || "";
}

// ── The clipboard ───────────────────────────────────────────────────────────
//
// **A fourth copy of `theClipboardTakes`** (`web/keys.js`,
// `web/the-machines-own-words.js`, `web/telegram.js`), kept the same on
// purpose, for the reason the second copy gives. The clipboard is the page's
// because the application binds no clipboard service.

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
  held.className = "rp-away";
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

whenTold(THE_REPORT_BOX, (delivery) => {
  theBoxIsDrawn(delivery.said || {});
});

// `Reporting a problem` C1–C5: an offer outside the failure shape. Its only
// choices go through the existing close command, with a closed choice and no
// identity from the page. The application already holds the crashed run.
const THE_CRASH_BOX = "reporting:the-crash-box";
let crashBox = null;
whenTold(THE_CRASH_BOX, (delivery) => {
  const said = delivery.said || {};
  if (!crashBox) {
    const root = anElement("div");
    root.id = "rpcrash";
    root.className = "modal top";
    const box = anElement("div");
    box.className = "mbox";
    box.setAttribute("role", "dialog");
    box.setAttribute("aria-modal", "true");
    const heading = anElement("h3");
    heading.id = "rp-crash-heading";
    box.setAttribute("aria-labelledby", heading.id);
    const first = anElement("p");
    first.className = "msub";
    const bold = anElement("b");
    first.append(bold);
    const second = anElement("p");
    second.className = "msub";
    const acts = anElement("div");
    acts.className = "mact";
    const dismiss = aButton();
    dismiss.className = "btn quiet";
    dismiss.id = "rpcrash-not-now";
    const report = aButton();
    report.className = "btn";
    report.id = "rpfromcrash";
    acts.append(dismiss, report);
    box.append(heading, first, second, acts);
    root.appendChild(box);
    (document.querySelector(".app") || document.body).appendChild(root);
    const choose = (afterCrash) => {
      if (root.hidden) return;
      root.hidden = true;
      shell.core.invoke("the_report_box_is_shut", { afterCrash });
    };
    dismiss.addEventListener("click", () => choose("not-now"));
    report.addEventListener("click", () => choose("report"));
    box.addEventListener("keydown", (press) => {
      if (press.key !== "Escape" || press.defaultPrevented) return;
      press.preventDefault();
      choose("not-now");
    });
    crashBox = { root, heading, bold, second, dismiss, report };
  }
  crashBox.heading.textContent = said.heading || "";
  crashBox.bold.textContent = said.nothingWasLost || "";
  crashBox.second.textContent = said.ifYouReportIt || "";
  crashBox.dismiss.textContent = said.notNow || "";
  crashBox.report.textContent = said.reportIt || "";
  crashBox.root.hidden = false;
  theFocusMovesTo(crashBox.report);
});

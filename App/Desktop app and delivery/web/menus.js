// **The four menus, drawn by the application** — Desktop app and delivery
// §18.2, §18.3 and §18.5, and only ever on Windows and Linux.
//
// **On a Mac this module draws nothing at all.** The four go in the system's
// own bar (K1, K9) and no bar is drawn in the window. The mockup draws one on
// every system with a line beside it reading "On a Mac this is the top of the
// screen, not the window" — **that line is the mockup's and not Smoozy's**, and
// the mockup's own guidance says so.
//
// **On Windows and Linux the same four sit behind one button in the title
// bar** (K2, K3, K10). On Windows that is an application choice rather than a
// platform necessity — the platform's own alternative is a window-owned bar —
// and Design chose the one button.
//
// **The keys, and the mechanism is this side's here** — K14. The shell's
// accelerator works only with a platform-native menu bar, so on these two
// systems the label beside an item is drawn by the application and **catching
// the keystroke is a key handler in this page**. The combinations are the
// application's, derived once in `src/menus.rs` and handed over; nothing here
// invents one and nothing here spells one.
//
// **K7: cut, copy, paste and select all act on whatever has focus.** The menu
// takes focus when it opens, so this module puts it back before it acts —
// otherwise the four would act on the menu button. **What has focus is kept as
// it changes, by one listener on the document**, and never read at the moment of
// the click: a click is raised after the press has already moved focus to the
// button, so reading it there reads the button. That is right whatever any
// engine does with focus on a press, which settles the question rather than
// resting on it (F-8, 2026-09-12).
//
// **Those four presses are the engine's, not Smoozy's** — see `theKeysAreCaught`
// below, which carries the ruling. **Whether the keystrokes work in the web view
// without a native menu is an assumption** and is labelled as one at K18 and at
// section 22 item 52; there is no Windows machine and no Linux desktop computer
// and neither is coming, so nothing here waits for an answer. The items act by
// click either way — **and whether that click route works is an assumption of
// exactly the same standing**, parked beside K18 and item 52 on 2026-09-12
// rather than rested on.
//
// **K17: three platform menu-key behaviours are knowingly absent** — Windows'
// mnemonics, F10 to activate the bar, Shift+F10 for a context menu; GNOME's
// F10. Nothing here adds one by imitation, and a test in `src/menus.rs` fails
// if somebody does.
//
// Desktop app and delivery H32: values are content, never markup. H33: no
// inline style and no inline script.
//
// **This module has a stylesheet of its own, `web/menus.css`, from
// 2026-09-12.** It had none before, and borrowed two classes that each brought
// their own drawing with them: `.sysbar`, the mockup's **menu-bar strip**, which
// carries its own height, its own sunken ground and a bottom border — so put
// inside the title bar it drew a short sunken box with a rule under it, between
// the application's name and the rest of the bar, when **Design settles that
// Smoozy draws *one bar at one height everywhere*.** And `.wbtn`, the **window
// button**, which nothing narrowed — so on Linux the menu button became a
// 22-pixel grey circle a few pixels from three identical grey circles that
// close, minimise and maximise the window, and on Windows a 46-pixel full-height
// plate. **Design's whole reason for asking each system what its window buttons
// look like was that one system's arrangement worn by two that do not use it is
// wrong**, and a fourth button that is not a window button at all is the same
// mistake by another route (F-9).
//
// **What is still borrowed is the panel**, which is genuinely the mockup's own
// menu panel: `.mb`, `.ctx` and `.k`. The button and the place it sits in are
// this module's own, in `menus.css`.

import { whenTold } from "./event-route.js";
import { theMarkOnAControl } from "./the-underway-shape.js";

const shell = window.__TAURI__;
const SVG_NS = "http://www.w3.org/2000/svg";

// The kind, declared beside the service that posts it —
// `THE_USER_CHOSE_A_GOING` in `src/surface/menus.rs`. The two have to agree.
const THE_USER_CHOSE_A_GOING = "menus:a-going";

// The one panel, and whether it is open.
let panel = null;
let opener = null;

// What had focus before the menu took it. K7's routing lives here: the Edit
// menu's four act on this rather than on the button the user just pressed.
//
// **It is kept as focus changes rather than read when the menu opens.** The
// opener's handler runs on the click, and a click is raised after the press has
// already moved focus — so `document.activeElement` read there is the button
// itself, which is exactly what the comment at the head of this file said must
// not happen. One listener on the document, in the capture phase so nothing can
// stop it, and anything inside the menu is skipped: the menu taking focus is the
// one focus change that must not be remembered.
let hadFocus = null;

document.addEventListener(
  "focusin",
  (event) => {
    const target = event.target;
    if (!target || target === opener) return;
    if (panel && panel.contains(target)) return;
    hadFocus = target;
  },
  true,
);

function theMark() {
  // Three lines. Drawn rather than taken from an icon font, for the same
  // reason the three window buttons are: which font ships is a property of the
  // system, and a missing glyph is a blank button.
  const svg = document.createElementNS(SVG_NS, "svg");
  svg.setAttribute("viewBox", "0 0 10 10");
  svg.setAttribute("aria-hidden", "true");
  for (const y of [2.5, 5, 7.5]) {
    const line = document.createElementNS(SVG_NS, "path");
    line.setAttribute("d", `M0 ${y} H10`);
    svg.appendChild(line);
  }
  return svg;
}

function close() {
  if (!panel) return;
  panel.hidden = true;
  if (opener) opener.setAttribute("aria-expanded", "false");
}

function open() {
  if (!panel) return;
  // **Nothing is read here.** What had focus is already known, from the
  // listener above; reading it at this moment would read the menu button,
  // because the press that opened the menu has already moved focus to it.
  panel.hidden = false;
  if (opener) opener.setAttribute("aria-expanded", "true");
}

// **The Edit menu's four, on whatever has focus** — K7. The focused thing may
// be an engine-drawn field, and this is the one route to it the page has.
function onWhateverHasFocus(which) {
  if (hadFocus && typeof hadFocus.focus === "function") hadFocus.focus();
  if (which === "selectAll") {
    document.execCommand("selectAll");
    return;
  }
  document.execCommand(which);
}

function chosen(item) {
  close();
  if (item.on_whatever_has_focus) {
    onWhateverHasFocus(item.on_whatever_has_focus);
    return;
  }
  // Everything else is the application's: a window act it performs on its own
  // window, or a going it posts on the one ordered queue. **The answer to this
  // call is not how the result arrives** — P6: an event is the application
  // telling the page, and a command is only the page asking.
  shell.core.invoke("a_menu_item_was_chosen", { item: item.id });
}

function drawItem(item) {
  const button = document.createElement("button");
  button.type = "button";
  button.dataset.item = item.id;

  const label = document.createElement("span");
  // Prose. It came through the typography application point (X1, K8), so the
  // two rules are already characters inside this string. Nothing here
  // substitutes anything.
  label.textContent = item.label;
  button.appendChild(label);

  // The combination, where this system publishes a convention for the act. An
  // absent one is K15's "no combination rather than an invented one", and the
  // absence is drawn as nothing rather than as a blank.
  if (item.key) {
    const key = document.createElement("span");
    key.className = "k mono";
    key.textContent = item.key;
    button.appendChild(key);
  }

  button.addEventListener("click", () => chosen(item));
  return button;
}

function drawMenus(bar) {
  const list = document.createElement("div");
  list.className = "ctx";

  for (const menu of bar.menus) {
    const heading = document.createElement("button");
    heading.type = "button";
    heading.className = "hd off";
    heading.disabled = true;
    heading.textContent = menu.title;
    list.appendChild(heading);

    for (const entry of menu.entries) {
      if (!entry.item) {
        list.appendChild(document.createElement("hr"));
        continue;
      }
      list.appendChild(drawItem(entry.item));
    }
    if (menu !== bar.menus[bar.menus.length - 1]) {
      list.appendChild(document.createElement("hr"));
    }
  }
  return list;
}

// **The key handler** — K14, on the two systems that draw their own labels.
// It reads the combination the application derived rather than parsing the
// label it drew, so the label and the keystroke cannot say different things.
//
// **The Edit menu's four are not among what is caught, and that is a ruling.**
// Smoozy used to match cut, copy, paste and select all, cancel the press, and
// perform the act itself through the one old engine call. **K16 carries the one
// published thing anybody has read on this point and it says the opposite** —
// *"with a text box focused, the copy accelerator copies the selected text and
// app-defined accelerators are ignored — a control can win"* — so cancelling the
// press made the application win instead. A user copying a host name from their
// browser and pressing paste in a Smoozy field got nothing, and the menu item
// did the identical thing through the identical call, so there was no third
// route.
//
// **So the engine keeps those four presses.** The drawn labels stay, which cost
// nothing and are K15's, and the menu items keep acting by click. **The worst
// case of this is the case section 22 item 52 already prices** — the keys do
// nothing and the menu items are what holds — while the worst case of the old
// code was a paste that could never work by any route. Ruled at C-2 and F-2,
// 2026-09-12.
function theKeysAreCaught(bar) {
  const pressed = [];
  for (const menu of bar.menus) {
    for (const entry of menu.entries) {
      if (!entry.item || !entry.item.press) continue;
      if (entry.item.on_whatever_has_focus) continue;
      pressed.push(entry.item);
    }
  }

  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      close();
      return;
    }
    for (const item of pressed) {
      const press = item.press;
      if (event.ctrlKey !== press.ctrl) continue;
      if (event.shiftKey !== press.shift) continue;
      if (event.altKey !== press.alt) continue;
      // The Meta/Super key is never part of a combination here — freedesktop
      // and KDE both publish that applications do not use it and Windows
      // reserves every combination involving it (K16) — so a press holding it
      // is somebody else's.
      if (event.metaKey) continue;
      if ((event.key || "").toLowerCase() !== press.key) continue;
      event.preventDefault();
      // **Nothing that arrives here acts on what has focus** — the Edit menu's
      // four are left to the engine above — so there is nothing to route and
      // the focus listener at the head of this file needs no help.
      chosen(item);
      return;
    }
  });
}

async function theMenusAreDrawn() {
  const bar = await shell.core.invoke("the_menu_bar");
  if (!bar.drawn_by_the_application) return;

  // **The one button, and the panel it opens.** `.mnu` is the place it sits in
  // the title bar and `.mnu-btn` is the button — **both this module's own, in
  // `menus.css`**, because the two classes borrowed before were the mockup's
  // menu-bar strip and the window button and each brought its own drawing with
  // it. The head of this file carries what that looked like. `.mb` and `.ctx`
  // stay the mockup's, being genuinely its menu panel.
  const strip = document.createElement("div");
  strip.className = "mnu";

  opener = document.createElement("button");
  opener.type = "button";
  opener.className = "mnu-btn";
  opener.setAttribute("aria-haspopup", "true");
  opener.setAttribute("aria-expanded", "false");
  opener.setAttribute("aria-label", bar.the_buttons_name);
  opener.setAttribute("title", bar.the_buttons_name);
  opener.appendChild(theMark());
  opener.addEventListener("click", (event) => {
    event.stopPropagation();
    if (panel.hidden) open();
    else close();
  });

  panel = document.createElement("div");
  panel.className = "mb";
  panel.hidden = true;
  panel.appendChild(drawMenus(bar));

  strip.appendChild(opener);
  strip.appendChild(panel);

  const name = document.getElementById("app-name");
  if (name && name.parentNode) name.parentNode.insertBefore(strip, name.nextSibling);

  // A click anywhere else closes it, which is the mockup's own behaviour and
  // the one thing about its menu script worth reading rather than reusing.
  document.addEventListener("click", () => close());

  theKeysAreCaught(bar);
}

// **A going, drawn by whoever draws the place it goes to.**
//
// A going to a place is the rail's (`web/the-rail.js`), and *Up one folder* is
// Files'. **This listener keeps the note on the document** that makes the whole
// route watchable at the desk — the attribute says which item arrived — **and
// draws the one box the menus own, About.**
//
// **The 38, piece 12.** *About Smoozy*, *Check for updates…* and *Report a
// problem…* posted a going nothing drew, and Rule 8 makes a menu item that does
// nothing and says nothing a fault. The application now says, on the going
// itself, the box each opens — `src/surface/menus.rs`, *the two items that
// open About*, carries which and why. **Every word arrives on the going and
// none is written here.**
//
// ***Report a problem…* is not one of them any more.** From step 11's piece
// R4 it opens the report box itself — `web/reporting.js`, told by the box's own
// event.
//
// **About's readings, from step 11's check piece** — Design's five and the
// held form of the third, as the drawing draws them (`box-about`): the version
// and when the last check completed; *Checking…* with the turning mark, the
// line itself carrying the underway shape because the menu item that started
// it is gone; a new version found; and a check that could not be reached, in
// the failure shape; and a found update that could not be installed. **The moment is written here in this computer's own way
// from a value, and never as an age** (Z23a). *Install…* is not drawn: it
// arrives with the act behind it. **The held form's *Install now* is drawn
// where the going says it** — every system, from step 11's Mac, Linux and
// Windows update pieces (Z32) —
// under the sentence that says what it does, and its press goes to the
// application and to nothing here.
whenTold(THE_USER_CHOSE_A_GOING, (delivery) => {
  document.documentElement.dataset.went = delivery.said.item || "";
  if (delivery.said.heading) theBoxOpens(theNewerOf(delivery.said));
});

// **About's reading changed** — a check began or ended. Drawn where About is
// open and nowhere else: it never opens the box.
const THE_ABOUTS_READING = "menus:abouts-reading";

whenTold(THE_ABOUTS_READING, (delivery) => {
  const said = theNewerOf(delivery.said);
  if (theBox && !theBox.root.hidden && theBox.text.hidden) theReadingIsDrawn(said);
});

// **The newest reading seen, by its number.** The going and a reading posted by
// the check can cross — the press reads *Checking…* while the check it started
// ends — so whichever carries the higher number is drawn, whichever arrived
// last.
let theNewest = null;

function theNewerOf(said) {
  const number = Number(said.readingNumber || 0);
  if (theNewest && Number(theNewest.readingNumber || 0) > number) return theNewest;
  theNewest = said;
  return said;
}

// The menus' one box, made the first time a going says one and kept.
let theBox = null;

function aLine(className) {
  const line = document.createElement("p");
  line.className = className;
  line.hidden = true;
  return line;
}

function theBoxIsMade() {
  // **The drawing's own top box**, the one Design draws About in: `.modal.top`
  // over the whole window, title bar included.
  const root = document.createElement("div");
  root.className = "modal top";
  root.classList.add("about");
  root.hidden = true;
  const box = document.createElement("div");
  box.className = "mbox";
  box.setAttribute("role", "dialog");
  box.setAttribute("aria-modal", "true");
  box.tabIndex = -1;
  const heading = document.createElement("h3");
  heading.id = "mnu-box-heading";
  box.setAttribute("aria-labelledby", heading.id);
  const version = document.createElement("p");
  version.className = "msub";

  // The readings and the held form, as the drawing has
  // them; the one that applies is shown.
  const idle = aLine("msub");
  idle.dataset.ab = "idle";
  const checking = aLine("msub");
  checking.dataset.ab = "check";
  const found = aLine("msub");
  found.dataset.ab = "new";
  // **The held form's sentence, read before the button is pressed** (Z32).
  const installing = aLine("msub");
  installing.dataset.ab = "held";
  const fail = aLine("ax-bad");
  fail.dataset.ab = "fail";
  const never = aLine("msub");
  never.dataset.ab = "never";

  const copyright = aLine("msub");
  copyright.classList.add("ab-copyright");
  const notice = aLine("msub");
  notice.classList.add("ab-notice");
  const links = document.createElement("div");
  links.className = "ab-links";
  const sourceFailure = aLine("ax-bad");
  const text = document.createElement("pre");
  text.className = "ab-text selectable";
  text.hidden = true;
  text.tabIndex = 0;
  const acts = document.createElement("div");
  acts.className = "mact";
  const close = document.createElement("button");
  close.type = "button";
  close.className = "btn quiet";
  const shut = () => {
    if (!text.hidden) {
      theBoxOpens(theNewest);
      return;
    }
    root.hidden = true;
  };
  close.addEventListener("click", shut);
  // **Z32's one press**: the box closes, as the drawing's button does, and the
  // application is told; what comes of it is the application's.
  const installNow = document.createElement("button");
  installNow.type = "button";
  installNow.className = "btn";
  installNow.hidden = true;
  installNow.addEventListener("click", () => {
    shut();
    shell.core.invoke("the_held_update_is_installed_anyway");
  });
  const installLinux = document.createElement("button");
  installLinux.type = "button";
  installLinux.className = "btn";
  installLinux.hidden = true;
  installLinux.addEventListener("click", () => {
    shut();
    shell.core.invoke("the_held_update_is_installed_anyway", { linux: { version: "", action: "offer" } });
  });
  box.addEventListener("keydown", (press) => {
    if (press.key !== "Escape" || press.defaultPrevented) return;
    press.preventDefault();
    shut();
  });
  acts.appendChild(close);
  acts.appendChild(installNow);
  acts.appendChild(installLinux);
  for (const one of [heading, version, copyright, notice, links, idle, checking, found, installing, fail, never, sourceFailure, text, acts]) {
    box.appendChild(one);
  }
  root.appendChild(box);
  (document.querySelector(".app") || document.body).appendChild(root);
  return { root, box, heading, version, copyright, notice, links, sourceFailure, text, idle, checking, found, installing, fail, never, close, installNow, installLinux };
}

/// The moment the last check completed, in this computer's own way of writing
/// one — the drawing's *25 Aug 13:52*. A moment and never an age (Z23a).
function theMomentDrawn(seconds) {
  const at = new Date(Number(seconds) * 1000);
  if (Number.isNaN(at.getTime())) return "";
  return at.toLocaleString([], {
    day: "numeric",
    month: "short",
    hour: "2-digit",
    minute: "2-digit",
  });
}

function aSpan(words, className) {
  const span = document.createElement("span");
  if (className) span.className = className;
  span.textContent = words || "";
  return span;
}

// **Which reading is drawn.** Each line is filled from the reading's own words
// and shown alone; the version line stays.
function theReadingIsDrawn(said) {
  const reading = said.reading || "idle";

  theBox.idle.replaceChildren();
  const idle = reading === "idle" && Boolean(said.lastChecked);
  if (idle) {
    // The words before the moment, the moment as a value, the full stop, and
    // the sentence after it — the drawing's one line.
    theBox.idle.appendChild(aSpan(said.upToDate));
    theBox.idle.appendChild(document.createTextNode(" "));
    theBox.idle.appendChild(aSpan(theMomentDrawn(said.lastChecked), "ab-at"));
    theBox.idle.appendChild(document.createTextNode(". "));
    theBox.idle.appendChild(aSpan(said.checksOnItsOwn));
  }
  theBox.idle.hidden = !idle;

  theBox.checking.replaceChildren();
  if (reading === "checking") {
    theBox.checking.appendChild(aSpan("", "ckdot"));
    theBox.checking.appendChild(aSpan(said.checking));
  }
  theBox.checking.hidden = reading !== "checking";

  theBox.found.replaceChildren();
  const found = reading === "new" || reading === "new-linux" || reading === "held";
  if (found) {
    // *Smoozy 2.5.0 is ready.* in bold, the version a value beside the words
    // rather than inside them (X7), and the sentence after it where this
    // reading has one.
    const bold = document.createElement("b");
    bold.appendChild(aSpan(said.beforeTheNewVersion));
    bold.appendChild(document.createTextNode(" "));
    const version = aSpan(said.newVersion);
    version.className = "ab-version selectable";
    bold.appendChild(version);
    bold.appendChild(document.createTextNode(" "));
    bold.appendChild(aSpan(said.isReady));
    theBox.found.appendChild(bold);
    if (said.afterReady) {
      theBox.found.appendChild(document.createTextNode(" "));
      theBox.found.appendChild(aSpan(said.afterReady));
    }
  }
  theBox.found.dataset.ab = reading === "held" ? "held" : reading;
  theBox.found.hidden = !found;

  // **The held form's sentence and *Install now*, where the going says them.**
  theBox.installing.replaceChildren();
  const installs = reading === "held" && Boolean(said.installNow);
  if (installs) {
    const sentences = [said.installingRestarts, said.jobsKeepRendering, said.transfersStop];
    sentences.forEach((one, at) => {
      if (at > 0) theBox.installing.appendChild(document.createTextNode(" "));
      theBox.installing.appendChild(aSpan(one));
    });
  }
  theBox.installing.hidden = !installs;
  theBox.installNow.textContent = installs ? said.installNow : "";
  theBox.installNow.hidden = !installs;
  theBox.installLinux.textContent = said.installLinux || "";
  theBox.installLinux.hidden = reading !== "new-linux" || !said.installLinux;

  theBox.fail.replaceChildren();
  const failed = reading === "fail" || reading === "install-fail";
  if (failed) {
    // **The failure shape**: the red dot and one plain sentence, naming no
    // cause.
    theBox.fail.appendChild(aSpan("", "fdot"));
    const bold = document.createElement("b");
    const failureWords = reading === "install-fail" ? said.couldNotInstall : said.couldNotReach;
    bold.className = "selectable";
    bold.textContent = failureWords || "";
    theBox.fail.appendChild(bold);
  }
  theBox.fail.dataset.ab = reading;
  theBox.fail.hidden = !failed;

  // **A copy that never checks**: a test copy, or a build that carries no
  // update key (E10b). One plain line either way, since nothing failed.
  const never = reading === "never" || reading === "no-updates";
  const neverWords = reading === "never" ? said.neverChecks : said.noUpdates;
  theBox.never.textContent = never ? neverWords || "" : "";
  if (never) theBox.never.dataset.ab = reading;
  theBox.never.hidden = !never;
}

function theBoxOpens(said) {
  if (!theBox) theBox = theBoxIsMade();
  theBox.text.hidden = true;
  theBox.box.classList.remove("ab-reading");
  theBox.heading.textContent = said.heading || "";
  theBox.copyright.textContent = said.copyright || "";
  theBox.copyright.hidden = !said.copyright;
  theBox.notice.textContent = said.freeSoftware || "";
  theBox.notice.hidden = !said.freeSoftware;
  theBox.sourceFailure.hidden = true;
  theBox.links.hidden = false;
  theBox.links.replaceChildren();
  const read = (label, content) => {
    for (const one of [theBox.version, theBox.copyright, theBox.notice, theBox.links,
                       theBox.idle, theBox.checking, theBox.found, theBox.installing,
                       theBox.fail, theBox.never, theBox.sourceFailure,
                       theBox.installNow, theBox.installLinux]) one.hidden = true;
    theBox.heading.textContent = label;
    theBox.text.textContent = content;
    theBox.text.hidden = false;
    theBox.text.scrollTop = 0;
    theBox.box.classList.add("ab-reading");
    theBox.text.focus();
  };
  for (const [label, action] of [
    [said.licence, () => read(said.licence, said.licenceText)],
    [said.thirdParty, () => read(said.thirdParty, said.thirdPartyText)],
    [said.sourceCode, async () => {
      let answer;
      try { answer = await shell.core.invoke("a_menu_item_was_chosen", { item: "about.source" }); }
      catch { answer = "ItCouldNotBeOpened"; }
      theBox.sourceFailure.replaceChildren();
      if (answer === "ItCouldNotBeOpened") {
        theBox.sourceFailure.appendChild(aSpan("", "fdot"));
        const words = document.createElement("b");
        words.textContent = said.sourceFailed;
        theBox.sourceFailure.appendChild(words);
      }
      theBox.sourceFailure.hidden = answer !== "ItCouldNotBeOpened";
    }],
  ]) {
    if (!label) continue;
    const link = document.createElement("button");
    link.type = "button";
    link.className = "ab-link";
    link.textContent = label;
    link.addEventListener("click", action);
    theBox.links.appendChild(link);
  }
  // **The version is a value and the word before it is prose** (X7), so the
  // two arrive apart and are drawn beside each other rather than one inside
  // the other — the drawing's *Version 2.4.1* on one line.
  theBox.version.textContent = "";
  if (said.version) {
    const lead = document.createElement("span");
    lead.textContent = said.beforeTheVersion || "";
    const value = document.createElement("span");
    value.className = "selectable";
    value.textContent = said.version;
    theBox.version.appendChild(lead);
    theBox.version.appendChild(document.createTextNode(" "));
    theBox.version.appendChild(value);
  }
  theBox.version.hidden = !said.version;
  theReadingIsDrawn(said);
  theBox.close.textContent = said.close || "";
  theBox.root.hidden = false;
  theBox.close.focus();
}

// The drawing's Linux-only box. A dismissal is sent to Rust, where its
// version is written durably. No password field exists in this page.
let theLinuxBox = null;
whenTold("menus:linux-update", (delivery) => {
  const said = delivery.said;
  if (theLinuxBox) theLinuxBox.remove();
  if (theBox) theBox.root.hidden = true;
  const root = document.createElement("div");
  root.className = "modal top linux-update";
  const box = document.createElement("div");
  box.className = "mbox";
  box.setAttribute("role", "dialog");
  box.setAttribute("aria-modal", "true");
  const heading = document.createElement("h3");
  heading.id = "linux-update-heading";
  box.setAttribute("aria-labelledby", heading.id);
  const version = aSpan(said.version);
  version.className = "selectable";
  heading.append(aSpan(said.beforeVersion), document.createTextNode(" "), version, document.createTextNode(" "), aSpan(said.afterVersion));
  const password = document.createElement("p");
  password.className = "msub";
  password.textContent = said.password || "";
  const declining = document.createElement("p");
  declining.className = "msub";
  declining.textContent = said.declining || "";
  const actions = document.createElement("div");
  actions.className = "mact";
  const notNow = document.createElement("button");
  notNow.type = "button";
  notNow.className = "btn quiet";
  notNow.textContent = said.notNow || "";
  const install = document.createElement("button");
  install.type = "button";
  install.className = "btn";
  install.textContent = said.install || "";
  let answered = false;
  const answer = (action) => {
    if (answered) return;
    answered = true;
    root.remove();
    theLinuxBox = null;
    shell.core.invoke("the_held_update_is_installed_anyway", { linux: { version: said.version, action } });
  };
  notNow.addEventListener("click", () => answer("decline"));
  install.addEventListener("click", () => answer("install"));
  box.addEventListener("keydown", (press) => {
    if (press.key === "Escape" && !press.defaultPrevented) { press.preventDefault(); answer("decline"); }
    if (press.key === "Tab") {
      press.preventDefault();
      (press.target === notNow ? install : notNow).focus();
    }
  });
  actions.append(notNow, install);
  box.append(heading, password, declining, actions);
  root.appendChild(box);
  (document.querySelector(".app") || document.body).appendChild(root);
  theLinuxBox = root;
  notNow.focus();
});

// **The rollback box** (step 11's D6b): what a launch shows, and nothing else,
// where an update could not start and the previous version is still to be
// restored. Drawn over the whole window like the update box. *Reinstall* takes
// the underway shape while the system asks and the reinstall runs; the box is
// drawn again, with the failure line, if it did not finish. *Quit* always
// answers. Escape closes nothing: there is nothing behind the box to go back to.
// **Where the saved data could not be read** (2026-11-01) the same box comes
// with no version, the failure line, and *Quit* alone: nothing is told to
// reinstall from.
let theRollbackBox = null;
whenTold("menus:linux-rollback", (delivery) => {
  const said = delivery.said;
  if (theRollbackBox) theRollbackBox.remove();
  if (theBox) theBox.root.hidden = true;
  const root = document.createElement("div");
  root.className = "modal top linux-update linux-rollback";
  const box = document.createElement("div");
  box.className = "mbox";
  box.setAttribute("role", "dialog");
  box.setAttribute("aria-modal", "true");
  const heading = document.createElement("h3");
  heading.id = "linux-rollback-heading";
  box.setAttribute("aria-labelledby", heading.id);
  heading.append(aSpan(said.beforeVersion), document.createTextNode(" "));
  if (said.version) {
    const version = aSpan(said.version);
    version.className = "selectable";
    heading.append(version, document.createTextNode(" "));
  }
  heading.append(aSpan(said.afterVersion));
  box.appendChild(heading);
  for (const words of [said.password, said.unfinished, said.until]) {
    if (!words) continue;
    const line = document.createElement("p");
    line.className = "msub";
    line.textContent = words;
    box.appendChild(line);
  }
  if (said.failed) {
    // **The failure shape**: the red dot and one plain sentence. The way to
    // try again is the box's own *Reinstall*, so the line does not repeat it.
    const fail = document.createElement("p");
    fail.className = "ax-bad";
    fail.appendChild(aSpan("", "fdot"));
    const bold = document.createElement("b");
    bold.className = "selectable";
    bold.textContent = said.failed;
    fail.appendChild(bold);
    box.appendChild(fail);
  }
  const actions = document.createElement("div");
  actions.className = "mact";
  const quit = document.createElement("button");
  quit.type = "button";
  quit.className = "btn quiet";
  quit.textContent = said.quit || "";
  const reinstall = document.createElement("button");
  reinstall.type = "button";
  reinstall.className = "btn";
  reinstall.textContent = said.reinstall || "";
  const answer = (action) => shell.core.invoke("the_held_update_is_installed_anyway", { linux: { version: said.version || "", action } });
  quit.addEventListener("click", () => answer("quit"));
  let underway = false;
  reinstall.addEventListener("click", () => {
    if (underway) return;
    underway = true;
    theMarkOnAControl(reinstall, said.reinstall || "");
    answer("reinstall");
  });
  box.addEventListener("keydown", (press) => {
    if (press.key === "Escape" && !press.defaultPrevented) press.preventDefault();
    if (press.key === "Tab") {
      press.preventDefault();
      (press.target === quit && said.reinstall ? reinstall : quit).focus();
    }
  });
  if (said.reinstall) {
    actions.append(quit, reinstall);
  } else {
    quit.className = "btn";
    actions.append(quit);
  }
  box.appendChild(actions);
  root.appendChild(box);
  (document.querySelector(".app") || document.body).appendChild(root);
  theRollbackBox = root;
  (said.reinstall ? reinstall : quit).focus();
});

// **This module starts itself, so that the shared entry takes one added line
// and no more.** That is the page's own layout rule at the head of smoozy.js,
// and the listener above is attached before this runs, so a going posted
// during the drawing is not lost.
//
// **A menu bar that could not be drawn leaves the window standing.** Rule 8
// makes Smoozy going quiet a fault however the trouble arose, and a window with
// no menus can still be used and still be reported; the trouble is said on the
// document rather than thrown into nothing.
theMenusAreDrawn().catch(() => {
  document.documentElement.dataset.menus = "failed";
});

// **Telegram, on the page** — Settings' Telegram section and the walkthrough
// that makes a bot. The application half is `src/surface/telegram.rs`, whose
// head carries the whole account and the rulings.
//
// **`web/settings.js` makes the section's place in the list and hands this file
// the element**, so this module registers no screen of its own. It keeps every
// rule at the heads of `smoozy.js` and `the-rail.js`: **no prose is written
// here** (A6, X1), **no name is joined into a sentence** (X7), **values are
// content and never markup** (H32), **no inline style** (H33), and **nothing
// polls** (P9, P10) — the wait for Start is the application's, and it answers
// on the queue.
//
// **The paste goes to `the_pasted_token_is_checked` and nowhere else**
// (`Telegram` S2): it is never put on an element's text, a title or a log.

import { whenTold } from "./event-route.js";
import { theFailureShape, EXPANDED } from "./the-failure-shape.js";
import { theMarkOnAControl, theMarkComesOffAControl } from "./the-underway-shape.js";
import { theValueIsDrawnInto } from "./the-value-drawn.js";

// **What stands between a figure and the words that count it** — U+00A0, so
// *3 servers* never parts across two lines. `Design/Whole app § Typography`; the
// character's one home is `src/typography.rs` (`BETWEEN_A_FIGURE_AND_ITS_UNIT`).
const BETWEEN_A_FIGURE_AND_ITS_UNIT = "\u00A0";

const shell = window.__TAURI__;

// Declared beside the service that posts them — `src/surface/telegram.rs`.
const THE_SECTION = "telegram:the-section";
const THE_WALKTHROUGH = "telegram:the-walkthrough";
const CHECK_AGAIN = "check-again";
const KEEP_AGAIN = "keep-again";
// **The idle message's switch is Settings' fourth notification switch**, turned
// by Settings' own command under the name `src/record/settings.rs` stores it by.
const THE_IDLE_KIND = "a-server-has-had-nothing-to-do";
// **This section's own holder for the idle switch's answer** —
// `THE_TELEGRAM_IDLE_ANSWER` in `src/surface/settings.rs`. An answer to a switch
// pressed in Settings is Settings' to draw (must-do 16 of step 10a round 1 of
// the queue, history, settings and downloads).
const THE_IDLE_ANSWER = "telegram-idle";
// **Five from 2026-10-31**, the Render Settings list's own five in its order
// (`Design/Settings § Telegram`, the drawing's `set-tgdefault`).
const HOW_MANY_CHOICES = 5;

let tg = null;

function made(tag, shape) {
  const element = document.createElement(tag);
  if (shape && shape.className) element.className = shape.className;
  return element;
}

function aControl(shape) {
  const control = made("button", shape);
  control.type = "button";
  return control;
}

// Strings drawn as one paragraph, which is what X10 hands over as several.
function theSentences(paragraph, ...said) {
  paragraph.replaceChildren();
  let first = true;
  for (const one of said) {
    if (!one) continue;
    if (!first) paragraph.appendChild(document.createTextNode(" "));
    paragraph.appendChild(document.createTextNode(one));
    first = false;
  }
}

function bold(text) {
  const b = made("b");
  b.textContent = text || "";
  return b;
}

function aFailureIn(holder, sentence, whatToDo, controls) {
  holder.replaceChildren();
  if (!sentence) return;
  holder.appendChild(
    theFailureShape({
      sentence,
      whatToDoAboutIt: whatToDo,
      kind: "ending",
      controls,
      room: EXPANDED,
      onAct: theFailuresActIsAsked,
    }),
  );
}

// ── The section — `Design/Settings § Telegram` ──────────────────────────────

/// **Build the Telegram section into the element Settings made for it**, and
/// the two walkthrough boxes into the screen's host.
export function theTelegramSectionIsBuilt(section, host) {
  const heading = made("h2");
  const before = made("p", { className: "hintl" });
  const switchRow = made("div", { className: "cmp" });
  const trow = made("div", { className: "trow" });
  const toggle = aControl({ className: "sws" });
  toggle.appendChild(made("i"));
  toggle.setAttribute("aria-pressed", "false");
  toggle.addEventListener("click", () => {
    const on = toggle.getAttribute("aria-pressed") === "true";
    shell.core.invoke("telegram_is_switched", { on: !on });
  });
  const switchName = made("span", { className: "tt" });
  trow.appendChild(toggle);
  trow.appendChild(switchName);
  switchRow.appendChild(trow);
  const answer = made("div", { className: "stg-answer" });

  // What is drawn only once Telegram is on.
  const on = made("div", { className: "tg-on" });
  const card = made("div", { className: "sucard" });
  const botRow = made("div", { className: "stgrow" });
  const botLab = made("span", { className: "lab" });
  const botName = made("span", { className: "path mono selectable" });
  const another = aControl({ className: "mini" });
  another.addEventListener("click", () => shell.core.invoke("the_telegram_walkthrough_is_opened"));
  botRow.append(botLab, botName, made("span", { className: "grow" }), another);
  const tokenRow = made("div", { className: "stgrow" });
  const tokenLab = made("span", { className: "lab" });
  const tokenMask = made("span", { className: "path mono" });
  // **Drawn instead of the dots where no token is stored for this bot** — fix-here item 2
  // of the round *setting it up, and what the desktop keeps*.
  const tokenNotStored = made("span", { className: "hintl" });
  tokenRow.append(tokenLab, tokenMask, tokenNotStored);
  const whoever = made("p", { className: "hintl" });
  card.append(botRow, tokenRow, whoever);

  const reports = made("h2", { className: "gap" });
  const choices = made("div", { className: "fmts" });
  choices.setAttribute("role", "radiogroup");
  const choiceRows = [];
  // **The third row's minutes field and the word after it**, drawn as the
  // drawing's `.fld.w-frame.mono` inside the row's name: *A picture every [5]
  // minutes*. **Built once**, so a redraw never takes the field from under
  // the caret; which row it sits in is the application's (`theMinutesChoiceIs`).
  const minutesField = made("input", { className: "fld w-frame mono" });
  minutesField.type = "text";
  minutesField.addEventListener("change", () => {
    shell.core.invoke("the_default_minutes_are_typed", { typed: minutesField.value });
  });
  const minutesAfter = made("span");
  for (let at = 0; at < HOW_MANY_CHOICES; at += 1) {
    const row = made("label", { className: "fmt choice3" });
    row.setAttribute("role", "radio");
    row.tabIndex = 0;
    const mark = made("span", { className: "rb" });
    const name = made("span", { className: "fmn" });
    const nameWords = made("span");
    name.appendChild(nameWords);
    const what = made("span", { className: "fmw" });
    row.append(mark, name, what);
    const choose = () => {
      if (row.dataset.choice) shell.core.invoke("the_reporting_default_is_set", { choice: row.dataset.choice });
    };
    // **A press in the minutes field is typing, not a pick**: the minutes
    // typed pick the row when they are taken.
    row.addEventListener("click", (press) => {
      if (press.target === minutesField) return;
      choose();
    });
    row.addEventListener("keydown", (press) => {
      if (press.target === minutesField) return;
      if (press.key === " " || press.key === "Enter") {
        press.preventDefault();
        choose();
      }
    });
    choices.appendChild(row);
    choiceRows.push({ row, mark, name, nameWords, what });
  }
  // **Minutes Smoozy will not take are said here**, under the radios, in the
  // section's one failure shape; the minutes that stood stay in the field.
  const minutesAnswer = made("div", { className: "stg-answer" });
  const always = made("p", { className: "hintl" });
  const theDefault = made("p", { className: "hintl" });

  const idleRow = made("div", { className: "cmp gap" });
  const idleTrow = made("div", { className: "trow" });
  const idle = aControl({ className: "sws" });
  idle.appendChild(made("i"));
  idle.setAttribute("aria-pressed", "false");
  idle.addEventListener("click", () => {
    const now = idle.getAttribute("aria-pressed") === "true";
    shell.core.invoke("a_notification_switch_is_turned", { kind: THE_IDLE_KIND, on: !now, pressedIn: "telegram" });
  });
  const idleName = made("span", { className: "tt" });
  idleTrow.append(idle, idleName);
  idleRow.appendChild(idleTrow);
  const idleAnswer = made("div", { className: "stg-answer" });
  on.append(card, reports, choices, minutesAnswer, always, theDefault, idleRow, idleAnswer);

  // `Telegram` L3: the servers not yet reached, named.
  const notReached = made("p", { className: "sunote amber gap" });

  section.append(heading, before, switchRow, answer, on, notReached);

  tg = {
    heading, before, toggle, switchName, answer, on, botLab, botName, another,
    tokenLab, tokenMask, tokenNotStored, whoever, reports, choiceRows, minutesField, minutesAfter, minutesAnswer,
    always, theDefault,
    idle, idleName, idleAnswer, notReached,
    boxes: theWalkthroughBoxes(host),
  };
}

whenTold(THE_SECTION, (delivery) => {
  if (!tg) return;
  const said = delivery.said || {};
  tg.heading.textContent = said.heading || "";
  aFailureIn(tg.answer, said.theAnswer || said.theSettingsWouldNotRead || "", "", []);
  const unread = Boolean(said.theSettingsWouldNotRead);
  const on = said.switchedOn === "yes";
  tg.toggle.hidden = unread;
  tg.switchName.textContent = unread ? "" : said.theSwitch || "";
  tg.toggle.setAttribute("aria-label", said.theSwitchsName || "");
  tg.toggle.classList.toggle("on", on);
  tg.toggle.setAttribute("aria-pressed", on ? "true" : "false");
  // **Before it is on: the switch and one paragraph, and nothing else.**
  tg.before.hidden = unread || on;
  theSentences(tg.before, said.beforeItIsOn, said.smoozyCannotMakeTheBot);
  tg.on.hidden = unread || !on;

  tg.botLab.textContent = said.bot || "";
  // Names read in the order their bytes run (X9, `web/the-value-drawn.js`).
  theValueIsDrawnInto(tg.botName, said.theBotsUsername || "");
  tg.botName.hidden = !said.theBotsUsername;
  tg.another.textContent = said.useAnotherBot || "";
  tg.tokenLab.textContent = said.token || "";
  tg.tokenMask.textContent = said.theTokenIsHeld || "";
  tg.tokenMask.hidden = !said.theTokenIsHeld;
  theSentences(tg.tokenNotStored, said.theTokenIsNotStored, said.howToStoreIt);
  tg.tokenNotStored.hidden = !said.theTokenIsNotStored;
  theSentences(tg.whoever, said.youMakeTheBot, said.whoeverHoldsIt);
  tg.reports.textContent = said.whatAJobReports || "";
  tg.choiceRows.forEach((choice, at) => {
    const id = said[`choiceIs${at}`] || "";
    const chosen = Boolean(id) && id === said.theDefaultIs;
    choice.row.dataset.choice = id;
    choice.row.classList.toggle("on", chosen);
    choice.mark.classList.toggle("on", chosen);
    choice.row.setAttribute("aria-checked", chosen ? "true" : "false");
    const name = said[`choiceName${at}`] || "";
    const what = said[`choiceWhat${at}`] || "";
    choice.nameWords.textContent = name;
    choice.what.textContent = what;
    // **The third row carries the minutes field**: *A picture every*, the
    // field, *minutes*. Put in once and left there, so a redraw never moves
    // the field while it has the focus.
    const holdsTheField = choice.name.contains(tg.minutesField);
    if (id && id === said.theMinutesChoiceIs) {
      if (!holdsTheField) {
        choice.name.append(document.createTextNode(" "), tg.minutesField, document.createTextNode(" "), tg.minutesAfter);
      }
    } else if (holdsTheField) {
      choice.name.replaceChildren(choice.nameWords);
    }
  });
  tg.minutesAfter.textContent = said.minutes || "";
  tg.minutesField.setAttribute("aria-label", said.minutesBetweenPictures || "");
  const refused = said.theMinutesAreRefused || "";
  // **The minutes that stood**, put back unless the user is typing in the field.
  if (document.activeElement !== tg.minutesField || refused) tg.minutesField.value = said.theDefaultMinutes || "";
  if (refused) tg.minutesField.setAttribute("aria-invalid", "true");
  else tg.minutesField.removeAttribute("aria-invalid");
  aFailureIn(tg.minutesAnswer, refused, "", []);
  tg.always.textContent = said.alwaysSent || "";
  theSentences(tg.theDefault, said.thisIsTheDefault, said.aJobCanBeSetDifferently);

  // The count and its words, the names beside them, and the words after —
  // **never one string** (X7).
  const many = Number(said.notReachedCount || 0);
  // **fix-here item 11: a told-record that did not read is said, never drawn as nobody.**
  const wouldNotRead = said.notReachedWouldNotRead || "";
  tg.notReached.replaceChildren();
  tg.notReached.hidden = many === 0 && !wouldNotRead;
  if (wouldNotRead) {
    tg.notReached.appendChild(document.createTextNode(wouldNotRead));
  } else if (many > 0) {
    tg.notReached.appendChild(bold(`${many}${BETWEEN_A_FIGURE_AND_ITS_UNIT}${said.notReachedLead || ""}`));
    tg.notReached.appendChild(document.createTextNode(" "));
    for (let at = 0; at < many; at += 1) {
      if (at > 0) tg.notReached.appendChild(document.createTextNode(", "));
      const server = said[`notReached${at}`] || "";
      tg.notReached.appendChild(document.createTextNode(server));
    }
    // The names close their sentence; the words after are a sentence of their own.
    tg.notReached.appendChild(document.createTextNode(". "));
    tg.notReached.appendChild(document.createTextNode(said.notReachedTail || ""));
  }
});

/// **The idle message's switch, from Settings' own event** — called by
/// `web/settings.js` every time that event arrives.
export function theNotificationSwitchesAre(said) {
  if (!tg) return;
  const on = said.aServerHasHadNothingToDoIsOn === "yes";
  tg.idle.hidden = Boolean(said.theSettingsWouldNotRead);
  tg.idle.classList.toggle("on", on);
  tg.idle.setAttribute("aria-pressed", on ? "true" : "false");
  tg.idle.setAttribute("aria-label", said.aServerHasHadNothingToDo || "");
  tg.idleName.textContent = said.aServerHasHadNothingToDo || "";
  aFailureIn(tg.idleAnswer, said.theAnswerFor === THE_IDLE_ANSWER ? said.theAnswer || "" : "", "", []);
}

// ── The walkthrough — the mockup's `#stgbot`, `#botreplace` and `#stgbotok` ──
//
// **Drawn again 2026-10-24, the seventeenth notes' piece 7**, from the two
// design passes' drawing: the choice alone first; each route shows only its
// own steps; the bot this computer already uses saves with no Start; Start
// shows the one checked link with the copy icon right beside it. The boxes
// carry the drawing's own identifiers so the drawing's own rules, lifted into
// `the-drawing.css`, are what draw them.
//
// **Which route is chosen is this page's alone** and never goes to the
// application: both routes hand their paste to the one check, and what follows
// is decided by the bot the check returned (`src/surface/telegram.rs`, its
// head).

function aBox(host, id) {
  const root = made("div", { className: "modal" });
  root.id = id;
  root.hidden = true;
  const box = made("div", { className: "mbox" });
  box.setAttribute("role", "dialog");
  root.appendChild(box);
  // **Escape closes the thing on top** — `Design/Whole app § Keyboard
  // shortcuts`. A box always has a way out.
  root.addEventListener("keydown", (press) => {
    if (press.key !== "Escape") return;
    press.stopPropagation();
    theWalkthroughIsShut();
  });
  host.appendChild(root);
  return { root, box };
}

function theWalkthroughIsShut() {
  shell.core.invoke("the_telegram_walkthrough_is_shut");
}

// ── The copy icon ───────────────────────────────────────────────────────────
//
// **A third copy of the output panel's copy icon** (`theCopyIcon` in
// `web/the-machines-own-words.js`, itself a copy of the Files toolbar's), made
// the same way from the drawing's own `<symbol id="i-copy">` path data. Sharing
// one would need a file of its own outside this piece's territory, as the
// second copy's own note says. A change to the shape belongs in all three.

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

/// **The drawing's icon button**: the icon, and its name shown on hover and
/// focus (`.ib`, `.ibl`). The name arrives as prose off the event.
function aCopyIcon(onPress) {
  const control = aControl({ className: "ib" });
  control.appendChild(theCopyIcon());
  const name = made("span", { className: "ibl" });
  control.appendChild(name);
  control.addEventListener("click", onPress);
  return { control, name };
}

function theCopyIconIsNamed(icon, name) {
  icon.control.setAttribute("aria-label", name || "");
  icon.name.textContent = name || "";
}

// ── The clipboard ───────────────────────────────────────────────────────────
//
// **A third copy of `theClipboardTakes`** (`web/keys.js`,
// `web/the-machines-own-words.js`), kept the same on purpose, for the reason
// the second copy gives. The clipboard is the page's because the application
// binds no clipboard service.

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
  held.className = "tg-away";
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

// **The copy on entering Start is readied at the press of Check bot.**
// `Telegram` S6a copies the checked name once when Start appears, and Start
// appears only after the check has gone to Telegram and back — long after the
// press, so an engine that lets a page write the clipboard only during a press
// would refuse a plain write then. **So the press readies a write whose words
// arrive later**: a clipboard item whose text is a promise, kept open until
// the application says which bot the check returned. Start resolves it with
// that name; anything else — a failure, the bot already used, Cancel — drops
// it, and nothing is written. Where the engine has no such item, or refuses
// it, Start falls back to the plain write; a copy that still does not land is
// said in place and the icon copies again.
let readied = null;

function aCopyIsReadied() {
  theReadiedCopyIsDropped();
  if (!window.ClipboardItem || !navigator.clipboard || !navigator.clipboard.write) return;
  let named;
  let dropped;
  const theName = new Promise((resolve, reject) => {
    named = resolve;
    dropped = reject;
  });
  theName.catch(() => {});
  let written;
  try {
    written = navigator.clipboard
      .write([new window.ClipboardItem({ "text/plain": theName })])
      .then(() => true, () => false);
  } catch (trouble) {
    return;
  }
  readied = { named, dropped, written };
}

function theReadiedCopyIsDropped() {
  if (!readied) return;
  readied.dropped();
  readied = null;
}

function theNameIsCopied(name) {
  const copy = readied;
  readied = null;
  if (!copy) return theClipboardTakes(name);
  copy.named(new Blob([name], { type: "text/plain" }));
  return copy.written.then((landed) => landed || theClipboardTakes(name));
}

// ── The three boxes ─────────────────────────────────────────────────────────

function theWalkthroughBoxes(host) {
  // The choice and the two routes — `set-bot`, `set-bot-new`,
  // `set-bot-existing`, the two checks and the four failures.
  const paste = aBox(host, "stgbot");
  const title = made("h3");
  const intro = made("p", { className: "msub" });
  intro.id = "botintro";
  const routes = made("div", { className: "botroute" });
  routes.setAttribute("role", "group");
  const makeNew = aControl({ className: "btn quiet" });
  const useExisting = aControl({ className: "btn quiet" });
  routes.append(makeNew, useExisting);

  // *Use an existing bot*.
  const existing = made("div");
  existing.id = "botexisting";
  existing.hidden = true;
  const aNewToken = made("p", { className: "msub" });
  const existingSteps = made("ol", { className: "bsteps" });
  const revoke = made("li");
  const pasteNew = made("li");
  const existingLabel = made("label");
  existingLabel.htmlFor = "botexistingtoken";
  const existingField = made("input", { className: "num mono wide" });
  existingField.type = "password";
  existingField.id = "botexistingtoken";
  existingField.setAttribute("aria-describedby", "botexistingnote");
  const existingNote = made("p", { className: "hintl" });
  existingNote.id = "botexistingnote";
  pasteNew.append(existingLabel, existingField, existingNote);
  existingSteps.append(revoke, pasteNew);
  const cost = made("p", { className: "mwarn" });
  existing.append(aNewToken, existingSteps, cost);

  // *Create a new bot*.
  const create = made("ol", { className: "bsteps" });
  create.id = "botcreate";
  create.hidden = true;
  const one = made("li");
  const two = made("li");
  const names = made("span");
  const nameRow = made("div", { className: "tg-nm" });
  const nameLab = made("span", { className: "tg-nl" });
  const nameIs = made("span", { className: "mono selectable" });
  const copyName = aCopyIcon(() => theClipboardTakes(nameIs.textContent));
  nameRow.append(nameLab, nameIs, copyName.control);
  const userRow = made("div", { className: "tg-nm" });
  const userLab = made("span", { className: "tg-nl" });
  const userIs = made("span", { className: "mono selectable" });
  const copyUser = aCopyIcon(() => theClipboardTakes(userIs.textContent));
  const another = aControl({ className: "mini" });
  another.addEventListener("click", () => shell.core.invoke("another_username_is_offered"));
  userRow.append(userLab, userIs, copyUser.control, another);
  two.append(names, nameRow, userRow);
  const three = made("li");
  const replies = made("span");
  const newField = made("input", { className: "num mono wide" });
  newField.type = "password";
  newField.id = "botnewtoken";
  const newNote = made("p", { className: "hintl" });
  three.append(replies, newField, newNote);
  create.append(one, two, three);

  const failure = made("div", { className: "tg-failure" });
  const acts = made("div", { className: "mact" });
  const cancel = aControl({ className: "btn quiet" });
  cancel.addEventListener("click", theWalkthroughIsShut);
  const go = aControl({ className: "btn" });
  go.id = "stgbotgo";
  go.hidden = true;
  const held = { label: "", saveLabel: "", route: null, checking: false, theBotsName: "" };
  go.addEventListener("click", () => {
    if (held.checking || !held.route) return;
    theTokenIsChecked();
  });
  acts.append(cancel, go);
  paste.box.append(title, intro, routes, existing, create, failure, acts);

  const fieldFor = (route) => (route === "existing" ? existingField : newField);

  // **The route chosen stays marked; choosing the other replaces its steps.**
  // While a check runs, neither the route nor the paste can change.
  function theRouteIsDrawn() {
    existing.hidden = held.route !== "existing";
    create.hidden = held.route !== "new";
    makeNew.setAttribute("aria-pressed", held.route === "new" ? "true" : "false");
    useExisting.setAttribute("aria-pressed", held.route === "existing" ? "true" : "false");
    for (const control of [makeNew, useExisting, existingField, newField]) control.disabled = held.checking;
    go.hidden = !held.route;
    go.disabled = held.checking;
  }

  function aRouteIsChosen(route) {
    if (held.checking) return;
    held.route = route;
    failure.replaceChildren();
    theRouteIsDrawn();
    fieldFor(route).focus();
  }
  makeNew.addEventListener("click", () => aRouteIsChosen("new"));
  useExisting.addEventListener("click", () => aRouteIsChosen("existing"));

  // **Check bot and Try again are one act.** The paste goes to the check and
  // nowhere else (`Telegram` S2); an empty one is answered by the application.
  function theTokenIsChecked() {
    if (held.checking || !held.route) return;
    failure.replaceChildren();
    aCopyIsReadied();
    held.checking = true;
    theMarkOnAControl(go, held.label);
    theRouteIsDrawn();
    shell.core.invoke("the_pasted_token_is_checked", { paste: fieldFor(held.route).value });
  }

  function theCheckIsOver() {
    held.checking = false;
    theMarkComesOffAControl(go, held.label);
    theRouteIsDrawn();
  }

  // The bot this computer already uses — `set-bot-replace`.
  const replace = aBox(host, "botreplace");
  const replaceTitle = made("h3");
  const alreadyUsed = made("p", { className: "msub" });
  const getTheNewToken = made("p", { className: "msub" });
  const replaceActs = made("div", { className: "mact" });
  const replaceCancel = aControl({ className: "btn quiet" });
  replaceCancel.addEventListener("click", theWalkthroughIsShut);
  const save = aControl({ className: "btn" });
  save.addEventListener("click", () => {
    if (save.classList.contains("uway")) return;
    theMarkOnAControl(save, held.saveLabel);
    shell.core.invoke("the_replacement_token_is_saved");
  });
  replaceActs.append(replaceCancel, save);
  replace.box.append(replaceTitle, alreadyUsed, getTheNewToken, replaceActs);

  // The bot answered — `set-bot-ok`, `set-bot-copyfail`, `set-bot-start`.
  const ok = aBox(host, "stgbotok");
  const okTitle = made("h3");
  const answered = made("p", { className: "msub" });
  const okSteps = made("ol", { className: "bsteps" });
  const open = made("li");
  const openYourBot = made("span");
  const linkRow = made("div", { className: "botstartlink" });
  const link = made("a", { className: "mono" });
  const copyTheName = aCopyIcon(() => theCopyIsSaid(theNameIsCopied(held.theBotsName)));
  copyTheName.control.id = "botnamecopy";
  linkRow.append(link, copyTheName.control);
  const copyResult = made("p", { className: "hintl" });
  copyResult.id = "botcopyresult";
  copyResult.setAttribute("role", "status");
  open.append(openYourBot, linkRow, copyResult);
  const press = made("li");
  okSteps.append(open, press);
  const waiting = made("p", { className: "msub" });
  const replied = made("p", { className: "msub" });
  const okFailure = made("div", { className: "tg-failure" });
  const okActs = made("div", { className: "mact" });
  const okCancel = aControl({ className: "btn quiet" });
  okCancel.addEventListener("click", theWalkthroughIsShut);
  const done = aControl({ className: "btn" });
  done.addEventListener("click", theWalkthroughIsShut);
  okActs.append(okCancel, done);
  ok.box.append(okTitle, answered, okSteps, waiting, replied, okFailure, okActs);

  // **Whether the copy landed, said in place and never blocking Start.** The
  // words are the event's; which of the two is drawn is the clipboard's answer.
  const said = { copied: [], notCopied: [] };
  function theCopyIsSaid(landing) {
    return Promise.resolve(landing).then((landed) => {
      theSentences(copyResult, ...(landed ? said.copied : said.notCopied));
      copyResult.classList.toggle("bad", !landed);
    });
  }

  return {
    paste, title, intro, routes, makeNew, useExisting, aNewToken, revoke, existingLabel, existingField,
    existingNote, cost, one, names, nameLab, nameIs, copyName, userLab, userIs, copyUser, another, replies,
    newField, newNote, failure, cancel, go, held, theTokenIsChecked, theCheckIsOver, theRouteIsDrawn,
    replace, replaceTitle, alreadyUsed, getTheNewToken, replaceCancel, save,
    ok, okTitle, answered, openYourBot, link, copyTheName, copyResult, press, waiting, replied, okFailure,
    okCancel, done, said, theCopyIsSaid,
  };
}

function theFailuresActIsAsked(act) {
  if (!tg) return;
  if (act === CHECK_AGAIN) tg.boxes.theTokenIsChecked();
  if (act === KEEP_AGAIN) {
    tg.boxes.okFailure.replaceChildren();
    shell.core.invoke("the_bot_is_kept_again");
  }
}

// *In Telegram, open* **@BotFather** *and …* — the words and the two names Telegram
// fixes, each its own node (X7).
function theBotMakersStep(item, said, ...after) {
  item.replaceChildren(
    document.createTextNode(said.inTelegramOpen || ""),
    document.createTextNode(" "),
    bold(said.theBotMaker),
    document.createTextNode(" "),
  );
  for (const one of after) item.appendChild(one);
}

whenTold(THE_WALKTHROUGH, (delivery) => {
  if (!tg) return;
  const said = delivery.said || {};
  const b = tg.boxes;
  const stage = said.stage || "";
  const wasShut = b.paste.root.hidden && b.replace.root.hidden && b.ok.root.hidden;

  // The choice.
  b.title.textContent = said.title || "";
  b.intro.textContent = said.doThisWhereYouCanReachTelegram || "";
  b.routes.setAttribute("aria-label", said.chooseABotRoute || "");
  b.makeNew.textContent = said.makeANewBot || "";
  b.useExisting.textContent = said.useAnExistingBot || "";

  // *Use an existing bot*.
  b.aNewToken.textContent = said.aNewTokenIsNeeded || "";
  const revoking = made("span");
  theSentences(revoking, said.andRevokeItsToken, said.youGetANewOne);
  theBotMakersStep(b.revoke, said, revoking);
  b.existingLabel.textContent = said.pasteTheNewTokenHere || "";
  b.existingField.setAttribute("placeholder", said.pasteHere || "");
  b.existingNote.textContent = said.nothingIsSavedBefore || "";
  b.cost.textContent = said.theOldTokensServersStop || "";

  // *Create a new bot*.
  theBotMakersStep(
    b.one,
    said,
    document.createTextNode(said.andSend || ""),
    document.createTextNode(" "),
    bold(said.theCommand),
    document.createTextNode("."),
  );
  theSentences(b.names, said.itAsksForTwoNames, said.copyEachOfThese);
  b.nameLab.textContent = said.name || "";
  theValueIsDrawnInto(b.nameIs, said.theName || "");
  theCopyIconIsNamed(b.copyName, said.copyName);
  b.userLab.textContent = said.username || "";
  theValueIsDrawnInto(b.userIs, said.theUsername || "");
  theCopyIconIsNamed(b.copyUser, said.copyUsername);
  b.another.textContent = said.another || "";
  theSentences(b.replies, said.theTokenLetsSmoozyUseYourBot, said.pasteBotFathersReplyHere);
  b.newField.setAttribute("placeholder", said.pasteHere || "");
  b.newField.setAttribute("aria-label", said.whatBotFatherSentBack || "");
  b.newNote.textContent = said.nothingIsSavedBefore || "";
  b.cancel.textContent = said.cancel || "";
  b.held.label = said.checkBot || "";

  // The bot this computer already uses.
  b.replaceTitle.textContent = said.title || "";
  b.alreadyUsed.replaceChildren(bold(said.theBotsUsername));
  b.alreadyUsed.appendChild(document.createTextNode(" "));
  const alreadyUsedSaid = made("span");
  theSentences(alreadyUsedSaid, said.isTheBotSmoozyAlreadyUses, said.noStartIsNeeded);
  b.alreadyUsed.appendChild(alreadyUsedSaid);
  b.getTheNewToken.textContent = said.serversGetTheNewToken || "";
  b.replaceCancel.textContent = said.cancel || "";
  b.held.saveLabel = said.saveToken || "";
  theMarkComesOffAControl(b.save, b.held.saveLabel);

  // Start.
  b.okTitle.textContent = said.title || "";
  theSentences(b.answered, said.theBotAnswered, said.twoThingsLeft);
  b.openYourBot.textContent = said.openYourBot || "";
  theValueIsDrawnInto(b.link, said.theLink || "");
  theCopyIconIsNamed(b.copyTheName, said.copyBotName);
  b.said.copied = [said.botNameCopied, said.youCanPasteIt];
  b.said.notCopied = [said.theBotNameWasNotCopied, said.useTheCopyIcon];
  b.press.replaceChildren(
    document.createTextNode(said.press || ""),
    document.createTextNode(" "),
    bold(said.start),
    document.createTextNode(". "),
    document.createTextNode(said.untilYouDo || ""),
  );
  // **The one turning mark every other screen draws**, where the drawing put
  // its dot: the drawing's `.ckdot` names an animation, `wksp`, that nothing
  // defines, so the line stood still. The wait is a reading no control
  // started, so it wears `.underway-reading` — `Design/Whole app § Anything
  // that isn't instant shows that it's underway`, *"the reading it produced
  // carries the mark itself"*. Step 10c's tenth-pass fix piece 6, as the
  // ninth-pass fix piece 5 did for Files.
  b.waiting.classList.add("underway-reading");
  b.waiting.replaceChildren(made("i", { className: "spin" }), document.createTextNode(" "), document.createTextNode(said.waitingForStart || ""));
  b.replied.replaceChildren(bold(said.theBotReplied), document.createTextNode(" "), document.createTextNode(said.telegramIsSetUp || ""));
  b.okCancel.textContent = said.cancel || "";
  b.done.textContent = said.done || "";

  const retry = said.retry ? [{ is: "retry", label: said.tryAgain || "", act: said.retry }] : [];
  if (stage === "paste") {
    theReadiedCopyIsDropped();
    // **A new opening begins at the choice**, with nothing pasted; anything
    // else drawn at the paste — *Suggest another*, a failure — keeps the route and
    // the paste.
    if (wasShut) {
      b.held.route = null;
      b.existingField.value = "";
      b.newField.value = "";
    }
    b.theCheckIsOver();
    aFailureIn(b.failure, said.theFailure || "", said.whatToDoAboutIt || "", retry);
    b.replace.root.hidden = true;
    b.ok.root.hidden = true;
    const opening = b.paste.root.hidden;
    b.paste.root.hidden = false;
    if (wasShut) b.makeNew.focus();
    else if (opening && b.held.route) b.go.focus();
  } else if (stage === "replace") {
    // **`Telegram` P4: no Start, and so nothing to copy.**
    theReadiedCopyIsDropped();
    b.theCheckIsOver();
    b.paste.root.hidden = true;
    b.ok.root.hidden = true;
    b.replace.root.hidden = false;
    b.save.focus();
  } else if (stage === "waiting" || stage === "replied" || stage === "notkept") {
    b.theCheckIsOver();
    b.held.theBotsName = said.theBotsName || "";
    // **`Telegram` S6a: the checked name, bare, copied once on entering
    // Start.** Only the drawing the application marks copies; a later drawing
    // of the same wait keeps what the line already says.
    if (said.copyTheName === "yes") {
      b.copyResult.replaceChildren();
      b.copyResult.classList.remove("bad");
      b.theCopyIsSaid(theNameIsCopied(b.held.theBotsName));
    }
    // **"notkept" is its own stage** (fix-here item 14): no waiting line, no Done, and the
    // failure with *keep again*.
    b.paste.root.hidden = true;
    b.replace.root.hidden = true;
    const opening = b.ok.root.hidden;
    b.ok.root.hidden = false;
    // **Start is a step**: while Smoozy waits there is no Done to press.
    b.waiting.hidden = stage !== "waiting";
    b.replied.hidden = stage !== "replied";
    b.done.hidden = stage !== "replied";
    aFailureIn(b.okFailure, said.theFailure || "", "", retry);
    if (opening) b.okCancel.focus();
  } else {
    theReadiedCopyIsDropped();
    b.paste.root.hidden = true;
    b.replace.root.hidden = true;
    b.ok.root.hidden = true;
    b.held.route = null;
    b.held.theBotsName = "";
    b.existingField.value = "";
    b.newField.value = "";
    b.theCheckIsOver();
    b.failure.replaceChildren();
    b.okFailure.replaceChildren();
    b.copyResult.replaceChildren();
    b.copyResult.classList.remove("bad");
  }
});

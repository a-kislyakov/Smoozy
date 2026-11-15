// **Help links** — `Design/Whole app § Help links`, and `Desktop app and
// delivery` K25's third narrowing (2026-11-13, the website ideas' picks).
//
// Beside three things the user fixes outside Smoozy — *Could not connect*, the
// NVIDIA driver lines and the admin password box — a small *Help* link follows
// the sentence and opens that sentence's own answer on the website. **The
// application says which answer**: the line arrives with the answer's name,
// and this file hands the name back when the link is pressed. It never holds
// an address, and the application refuses a name off its list.
//
// **The link is the drawing's text link** (`.lnkb`), one word. Where the system
// will not open the browser, the link gives way to *Could not open Help.* in
// the failure shape until the next press. Both words are the application's,
// asked for once (`the_help_links`).

// Read when asked, so a page that loads this before the shell is set up still works.
const shell = () => globalThis.__TAURI__;

let theWords = null;

function theWordsAreAskedFor() {
  if (!theWords) {
    theWords = Promise.resolve().then(() => shell().core.invoke("the_help_links")).then((words) => words || null).catch(() => {
      theWords = null;
      return null;
    });
  }
  return theWords;
}

/// **A Help link for the answer `answer` names**, to set after a sentence.
/// The link is drawn when its word arrives from the application, with the
/// space before it; until then the line reads as it did before.
export function aHelpLink(answer) {
  const holder = document.createElement("span");
  holder.className = "help-link-holder";
  const link = document.createElement("button");
  link.type = "button";
  link.className = "lnkb help-link";
  link.dataset.help = answer;
  let said = null;
  theWordsAreAskedFor().then((words) => {
    said = words;
    if (!words) return;
    holder.appendChild(document.createTextNode(" "));
    link.textContent = words.help;
    holder.appendChild(link);
  });
  link.addEventListener("click", async (press) => {
    // A row or a card around the link must not take the press as its own.
    if (press && typeof press.stopPropagation === "function") press.stopPropagation();
    let opened;
    try {
      opened = await shell().core.invoke("the_help_is_opened", { answer });
    } catch {
      opened = "ItCouldNotBeOpened";
    }
    if (opened !== "ItCouldNotBeOpened" || !said) return;
    // **The failure shape, in the link's place**, until the next press.
    const failed = document.createElement("span");
    failed.className = "help-link-failed";
    const dot = document.createElement("span");
    dot.className = "fdot";
    failed.appendChild(dot);
    const words = document.createElement("b");
    words.textContent = said.could_not_open;
    failed.appendChild(words);
    failed.addEventListener("click", () => failed.replaceWith(link));
    link.replaceWith(failed);
  });
  return holder;
}

/// **Set a Help link after `line`'s sentence**, where the application named
/// an answer for it.
export function theHelpLinkFollows(line, answer) {
  if (!line || !answer) return;
  line.appendChild(aHelpLink(answer));
}

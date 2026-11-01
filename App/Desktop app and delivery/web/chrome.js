// The window's own title bar.
//
// It asks the application which system this is, asks it for the chrome's
// prose, and draws the window buttons the way that system draws them. On a Mac
// it draws none: the system keeps its frame and its own buttons.
//
// **All of it is step 1's and none of it changed when it moved out of
// smoozy.js on 2026-09-08.** The move is the file layout rule at the head of
// smoozy.js, and this is its first instance: one service, one module.
//
// Desktop app and delivery H32: values are content, never markup. Nothing here
// touches innerHTML, and nothing builds markup from a string.
// H17 and H18: the light-or-dark reading is the stylesheet's media query, so
// there is no reading here, no listener and no timer.

const shell = window.__TAURI__;
const SVG_NS = "http://www.w3.org/2000/svg";

function windowCall(name) {
  // Desktop app and delivery H6a: where the buttons are Smoozy's own they are
  // wired to the shell's published calls. The permissions each one needs are
  // named in capabilities/main.json.
  const current = shell.window.getCurrentWindow();
  if (name === "minimize") return current.minimize();
  if (name === "maximize") return current.toggleMaximize();
  return current.close();
}

// The three marks, drawn rather than taken from an icon font. Windows names
// icons in its own font, but which font ships is a property of the Windows
// version, and a missing glyph is a blank button. A drawn line, square and
// cross cannot go missing. The square has the rounded corner Microsoft's own
// page describes.
function mark(kind) {
  const svg = document.createElementNS(SVG_NS, "svg");
  svg.setAttribute("viewBox", "0 0 10 10");
  svg.setAttribute("aria-hidden", "true");
  const shape = (name, attrs) => {
    const el = document.createElementNS(SVG_NS, name);
    for (const [k, v] of Object.entries(attrs)) el.setAttribute(k, v);
    svg.appendChild(el);
  };
  if (kind === "minimize") {
    shape("path", { d: "M0 5 H10" });
  } else if (kind === "maximize") {
    shape("rect", { x: "0.5", y: "0.5", width: "9", height: "9", rx: "1.5" });
  } else {
    shape("path", { d: "M0 0 L10 10 M10 0 L0 10" });
  }
  return svg;
}

function named(el, label) {
  el.setAttribute("aria-label", label);
  el.setAttribute("title", label);
}

function button(kind, label) {
  const el = document.createElement("button");
  el.type = "button";
  el.className = "wbtn wbtn-" + kind;
  named(el, label);
  el.appendChild(mark(kind));
  el.addEventListener("click", () => windowCall(kind));
  return el;
}

// **The middle button is named for what it will do**: *Restore* while the
// window is maximised, *Maximise* otherwise, as desktop apps name it —
// Design/Whole app.md, "The window's own chrome", 2026-10-25. Both words are
// the application's; this only reads the window's state, at the start and at
// every resize, which is how a maximise or a restore by any route arrives.
async function theMiddleButtonIsNamed(el, text) {
  const current = shell.window.getCurrentWindow();
  const say = async () => {
    try {
      named(el, (await current.isMaximized()) ? text.restore : text.maximise);
    } catch (_) {
      // The state could not be read, and the name stays as it was.
    }
  };
  await say();
  try {
    await current.onResized(() => say());
  } catch (_) {
    // No resize is heard, and the name stays as it was last read.
  }
}

// What the Linux desktop published, in its own form: names separated by
// commas, a colon separating the left corner from the right. Anything Smoozy
// does not draw is ignored, which is what that setting's own description says
// unknown names are for.
function splitLinuxLayout(layout) {
  const drawable = ["minimize", "maximize", "close"];
  const halves = layout.split(":");
  const pick = (half) =>
    (half || "")
      .split(",")
      .map((name) => name.trim())
      .filter((name) => drawable.includes(name));
  return { left: pick(halves[0]), right: pick(halves[1]) };
}

function drawButtons(system, layout, text) {
  const label = { minimize: text.minimise, maximize: text.maximise, close: text.close };
  let left = [];
  let right = [];

  if (system === "windows") {
    // Windows reserves the upper right for them and puts close there.
    right = ["minimize", "maximize", "close"];
  } else if (layout) {
    const sides = splitLinuxLayout(layout);
    left = sides.left;
    right = sides.right;
  } else {
    // Nothing could be read from the desktop.
    right = ["minimize", "maximize", "close"];
  }

  let middle = null;
  const into = (id, kinds) => {
    const host = document.getElementById(id);
    for (const kind of kinds) {
      const el = host.appendChild(button(kind, label[kind]));
      if (kind === "maximize") middle = el;
    }
  };
  into("buttons-left", left);
  into("buttons-right", right);
  return middle;
}

export async function theChromeIsDrawn() {
  const system = await shell.core.invoke("system_name");
  document.documentElement.dataset.system = system;

  // Desktop app and delivery X1: the interface accepts prose only from the
  // typography point, which is on the application side.
  const text = await shell.core.invoke("chrome_text");
  document.getElementById("app-name").textContent = text.name;

  if (system === "macos") return;

  const layout =
    system === "linux" ? await shell.core.invoke("linux_button_layout") : "";
  const middle = drawButtons(system, layout, text);
  if (middle) await theMiddleButtonIsNamed(middle, text);
}

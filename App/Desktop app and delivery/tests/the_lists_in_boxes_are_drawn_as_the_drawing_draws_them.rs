//! **Every box in the drawing that holds a list, drawn by the application's
//! stylesheets, reads as the drawing's own stylesheet draws it.** Step 10c,
//! the three small owed fixes, item 2, 2026-10-24.
//!
//! `web/render-settings.css` once gave every list item inside every box grey
//! ink and a margin, for a list of its own that the panel no longer draws. It
//! reached every other box with a list: the bot steps (put back for those two
//! boxes alone, inside `web/telegram.css`), the box that stops transfers, and
//! the two Set up boxes that list the jobs running.
//!
//! **How it is checked, in a real browser.** Headless Chrome opens a copy of
//! `Preparation/Design/Mockup/Smoozy.html` and, for each box there holding a
//! list, reads the box's markup and what the drawing's own stylesheet computes
//! for each list and each item: ink, size, weight, margins, inset, bullet, and
//! the gap between items. It then opens a page of the application's own
//! stylesheets (`web/smoozy.css` and every sheet it brings in), with those same
//! boxes put in, and reads the same. The two must match, box by box. A computer
//! with no Chrome skips it and says so.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// The boxes in the drawing that hold a list, each named by its window's
/// identifier. The test fails if the drawing gains or loses one, so a new box
/// with a list is checked from the day it is drawn.
const THE_BOXES_WITH_A_LIST: [&str; 5] = ["bubusy", "buunpack", "stgbot", "stgbotok", "tstop"];

/// What is read for each list and each item, the same on both pages.
const THE_READING: &str = r##"
const forTheList = ["color", "font-size", "font-weight", "margin-top", "margin-bottom",
  "padding-left", "list-style-type", "row-gap", "display"];
const forTheItem = ["color", "font-size", "font-weight", "font-style", "margin-top",
  "margin-bottom", "padding-left"];
function read(element, names) {
  const computed = getComputedStyle(element);
  const out = {};
  for (const name of names) out[name] = computed.getPropertyValue(name);
  return out;
}
function theListsIn(box) {
  return Array.from(box.querySelectorAll("ul, ol")).map((list) => ({
    list: list.tagName.toLowerCase() + "." + list.className + (list.id ? "#" + list.id : ""),
    reads: read(list, forTheList),
    items: Array.from(list.children).filter((one) => one.tagName === "LI").map((one) => read(one, forTheItem)),
  }));
}
// **Nothing the dump could escape**: the answer carries no <, >, & or
// non-breaking space, so it reads back exactly as written.
function said(value) {
  return JSON.stringify(value).replace(/[<>&\u00a0]/g, (c) => "\\u" + c.charCodeAt(0).toString(16).padStart(4, "0"));
}
"##;

/// Appended to the copy of the drawing: each window holding a box with a
/// list, its markup as drawn, and the reading.
const ON_THE_DRAWING: &str = r#"
<pre id="smoozy-lists-read" hidden></pre>
<script>
window.addEventListener("load", () => setTimeout(() => {
  const out = {};
  for (const box of document.querySelectorAll(".mbox")) {
    if (!box.querySelector("ul, ol")) continue;
    const window_ = box.closest(".modal") || box;
    const name = window_.id || (box.closest("[id]") || {}).id || "unnamed";
    out[name] = { markup: window_.outerHTML, lists: theListsIn(box) };
  }
  document.getElementById("smoozy-lists-read").textContent = said(out);
}, 200));
</script>
"#;

/// The application's page: its one stylesheet, the boxes the drawing drew put
/// in, and the same reading.
const THE_APPLICATIONS_PAGE: &str = r#"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
</head><body>
<div class="app" id="app"></div>
<pre id="smoozy-lists-read" hidden></pre>
<script src="reading.js"></script>
<script src="boxes.js"></script>
<script>
window.addEventListener("load", () => {
  const app = document.getElementById("app");
  const out = {};
  for (const [name, markup] of Object.entries(window.theBoxes)) {
    app.insertAdjacentHTML("beforeend", markup);
    const box = app.lastElementChild.matches(".mbox") ? app.lastElementChild : app.lastElementChild.querySelector(".mbox");
    out[name] = { lists: theListsIn(box) };
  }
  document.getElementById("smoozy-lists-read").textContent = said(out);
});
</script>
</body></html>"#;

/// The first Chrome this computer has, where it has one.
fn a_chrome() -> Option<PathBuf> {
    [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        "/usr/bin/google-chrome",
        "/usr/bin/chromium",
        "/usr/bin/chromium-browser",
        "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
    ]
    .into_iter()
    .map(PathBuf::from)
    .find(|one| one.exists())
}

/// A scratch folder that takes itself away however the test ends.
struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// One page opened in headless Chrome and its reading returned. A fixed
/// deadline, never an open wait.
fn read_in_chrome(chrome: &Path, scratch: &Path, page: &Path, named: &str) -> serde_json::Value {
    let dumped = scratch.join(format!("{named}.dump.html"));
    let mut running = std::process::Command::new(chrome)
        .args([
            "--headless",
            "--disable-gpu",
            "--no-first-run",
            "--hide-scrollbars",
            "--allow-file-access-from-files",
            "--virtual-time-budget=5000",
            "--window-size=1280,800",
        ])
        .arg(format!("--user-data-dir={}", scratch.join(format!("profile-{named}")).display()))
        .arg("--dump-dom")
        .arg(format!("file://{}", page.display()))
        .stdout(std::fs::File::create(&dumped).expect("the dump is written"))
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Chrome starts");
    let deadline = Instant::now() + Duration::from_secs(90);
    while running.try_wait().expect("Chrome is watched").is_none() {
        if Instant::now() > deadline {
            let _ = running.kill();
            let _ = running.wait();
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    let dump = std::fs::read_to_string(&dumped).unwrap_or_default();
    let read = dump
        .split("<pre id=\"smoozy-lists-read\" hidden=\"\">")
        .nth(1)
        .and_then(|rest| rest.split("</pre>").next())
        .filter(|read| !read.is_empty())
        .unwrap_or_else(|| panic!("{named}: the page read nothing"));
    serde_json::from_str(read).unwrap_or_else(|why| panic!("{named}: the reading is not one answer: {why}"))
}

#[test]
fn every_box_with_a_list_reads_as_the_drawing_draws_it_in_a_real_browser() {
    let Some(chrome) = a_chrome() else {
        eprintln!("SKIPPED: no Chrome on this computer, so the boxes with a list were not compared");
        return;
    };
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let scratch = Scratch(std::env::temp_dir().join(format!("smoozy-lists-in-boxes-{}", std::process::id())));
    let _ = std::fs::remove_dir_all(&scratch.0);
    std::fs::create_dir_all(&scratch.0).expect("a scratch folder");

    // **The drawing's own page**, with the reading put in at its end.
    let drawing = std::fs::read_to_string(here.join("../../Preparation/Design/Mockup/Smoozy.html"))
        .expect("the drawing is readable");
    // The drawing closes neither its body nor its page, so the reading goes
    // at its end, before a closing tag where it has one.
    let end = drawing.rfind("</body>").or_else(|| drawing.rfind("</html>")).unwrap_or(drawing.len());
    let mut copy = String::with_capacity(drawing.len() + 4096);
    copy.push_str(&drawing[..end]);
    copy.push_str("<script>");
    copy.push_str(THE_READING);
    copy.push_str("</script>");
    copy.push_str(ON_THE_DRAWING);
    copy.push_str(&drawing[end..]);
    let the_drawings_page = scratch.0.join("drawing.html");
    std::fs::write(&the_drawings_page, copy).expect("the copy of the drawing is written");
    let drawn = read_in_chrome(&chrome, &scratch.0, &the_drawings_page, "drawing");
    let drawn = drawn.as_object().expect("the drawing's reading is one map");

    let mut names: Vec<&str> = drawn.keys().map(String::as_str).collect();
    names.sort_unstable();
    assert_eq!(names, THE_BOXES_WITH_A_LIST, "the drawing's boxes with a list are not the ones this test names");

    // **The application's page**: every stylesheet, the drawing's boxes put in.
    for entry in std::fs::read_dir(here.join("web")).expect("the page folder") {
        let path = entry.expect("an entry").path();
        if path.extension().and_then(|one| one.to_str()) == Some("css") {
            std::fs::copy(&path, scratch.0.join(path.file_name().expect("a name"))).expect("a stylesheet is copied");
        }
    }
    let markup: serde_json::Map<String, serde_json::Value> =
        drawn.iter().map(|(name, box_)| (name.clone(), box_["markup"].clone())).collect();
    std::fs::write(scratch.0.join("boxes.js"), format!("window.theBoxes = {};", serde_json::Value::Object(markup)))
        .expect("the boxes are written");
    std::fs::write(scratch.0.join("reading.js"), THE_READING).expect("the reading is written");
    let the_applications_page = scratch.0.join("index.html");
    std::fs::write(&the_applications_page, THE_APPLICATIONS_PAGE).expect("the page is written");
    let ours = read_in_chrome(&chrome, &scratch.0, &the_applications_page, "application");

    let mut differences = Vec::new();
    for name in THE_BOXES_WITH_A_LIST {
        let theirs = &drawn[name]["lists"];
        let mine = &ours[name]["lists"];
        assert!(
            theirs.as_array().is_some_and(|lists| lists.iter().all(|list| !list["items"].as_array().unwrap().is_empty())),
            "{name}: the drawing's lists here have no items to compare"
        );
        if theirs != mine {
            differences.push(format!("{name}:\n  the drawing:     {theirs}\n  the application: {mine}"));
        }
    }
    assert!(
        differences.is_empty(),
        "boxes whose lists the application draws differently from the drawing:\n{}",
        differences.join("\n")
    );
}

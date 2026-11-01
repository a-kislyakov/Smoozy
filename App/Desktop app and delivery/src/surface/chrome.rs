//! **The window chrome's own three commands** — the strings the title bar
//! draws, which system this copy is on, and what the Linux desktop says its
//! window buttons should be.
//!
//! **All three are step 1's and none of them changed when they moved here on
//! 2026-09-08.** They were free functions in `main.rs` beside the window
//! construction; the folder they now sit in is the layout rule at
//! [`crate::surface`], and this file is its first instance.
//!
//! `Desktop app and delivery` H6a is what they keep, and X1 is what the prose
//! among them passes through.

use crate::typography::Prose;

/// The strings the window chrome draws, each having passed through the
/// typography point — X1, which says the interface accepts prose from nowhere
/// else.
#[derive(serde::Serialize)]
pub struct ChromeText {
    pub name: Prose,
    pub minimise: Prose,
    pub maximise: Prose,
    /// **The middle button's name while the window is maximised** —
    /// `Design/Whole app`, *The window's own chrome*, 2026-10-25.
    pub restore: Prose,
    pub close: Prose,
}

/// Which system this copy is running on, so the window chrome can be drawn
/// the way that system draws windows. Design settled on 2026-09-02 that macOS
/// keeps its own frame and its own window buttons, and that Windows and Linux
/// give the frame up and take Smoozy's own three — H6a, which hands that
/// choice to Design and supplies both routes.
///
/// This is neither prose nor a value the user reads, so neither section 17
/// route applies to it: it is a fact about the machine that decides a layout.
#[tauri::command]
pub fn system_name() -> &'static str {
    std::env::consts::OS
}

/// What the Linux desktop says its window buttons should be.
///
/// There is no single Linux arrangement. What the desktop does publish is one
/// setting naming which buttons to show and on which side — its own words:
/// *"the colon separates the left corner of the window from the right corner,
/// and the button names are comma-separated"*, with a default of
/// `appmenu:close`, which is a close button alone on the right.
///
/// Returned exactly as the desktop states it, so the page can draw what the
/// setting says. An empty answer means nothing could be read, and the page
/// falls back to the three at the right.
#[tauri::command]
pub fn linux_button_layout() -> String {
    #[cfg(target_os = "linux")]
    {
        let read = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.wm.preferences", "button-layout"])
            .output();
        if let Ok(out) = read {
            if out.status.success() {
                let text = String::from_utf8_lossy(&out.stdout);
                return text.trim().trim_matches('\'').trim_matches('"').to_string();
            }
        }
    }
    String::new()
}

/// X1 and A6. The one way prose reaches this window. Values would come the
/// other way, by the verbatim route at X9.
#[tauri::command]
pub fn chrome_text() -> ChromeText {
    ChromeText {
        name: Prose::new("Smoozy"),
        // The three window buttons' names are Design's, and Design names them:
        // Preparation/Design/Whole app.md, "The window's own chrome", the
        // paragraph added 2026-10-24 - Minimise, Maximise and Close, the
        // standard software words Rule 9 requires. H6a settles only that
        // Smoozy draws three buttons of its own. (Until 2026-10-24 this was an
        // open marker reading: "the three window buttons' accessible names are
        // Design's to confirm. The drawing at
        // Preparation/Design/Mockup/Smoozy.html shows three unlabelled
        // circles, and H6a settles only that Smoozy draws three buttons of its
        // own. These are the standard software words Rule 9 requires, standing
        // in until Design says otherwise.")
        minimise: Prose::new("Minimise"),
        maximise: Prose::new("Maximise"),
        // **Restore while maximised**, as desktop apps name it — the same
        // paragraph, its line of 2026-10-25 (the small owed fixes after the
        // eighteenth notes, on the build runner's ruling).
        restore: Prose::new("Restore"),
        close: Prose::new("Close"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **X1 and A6: every string this command hands the page has been through
    /// the typography point.**
    ///
    /// The three button names are single words and the application's name is
    /// one, so none of them shows a substitution — which is exactly why the
    /// check is on the type rather than on the characters. **A field that took
    /// a `String` would compile, would draw, and would have skipped the
    /// point**; this is the line that would fail.
    #[test]
    fn every_string_the_chrome_draws_comes_by_the_prose_route() {
        let text = chrome_text();
        let _: &Prose = &text.name;
        let _: &Prose = &text.minimise;
        let _: &Prose = &text.maximise;
        let _: &Prose = &text.restore;
        let _: &Prose = &text.close;
        assert_eq!(text.name.as_str(), "Smoozy");
    }

    /// **The middle button reads *Restore* while the window is maximised and
    /// *Maximise* otherwise** (`Design/Whole app`, *The window's own chrome*,
    /// 2026-10-25) — the page itself, run under Node against a stand-in shell:
    /// drawn on a maximised window, then the window restored and a resize
    /// heard, then maximised again. Until then it read *Maximise* whatever the
    /// window was. **Skipped, and says so, where no Node is installed.**
    #[test]
    fn small_owed_the_middle_button_reads_restore_while_maximised() {
        let folder = std::env::temp_dir().join(format!("smoozy-chrome-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/chrome.js");
        std::fs::copy(&page, folder.join("chrome.mjs")).expect("the page is copied");
        let text = serde_json::to_string(&serde_json::json!({
            "name": chrome_text().name.as_str(),
            "minimise": chrome_text().minimise.as_str(),
            "maximise": chrome_text().maximise.as_str(),
            "restore": chrome_text().restore.as_str(),
            "close": chrome_text().close.as_str(),
        }))
        .expect("serialises");
        let harness = format!(
            r#"
let maximised = true;
let heard = null;
const anElement = () => ({{
  attributes: {{}}, children: [], dataset: {{}}, textContent: "",
  setAttribute(name, value) {{ this.attributes[name] = String(value); }},
  appendChild(child) {{ this.children.push(child); return child; }},
  addEventListener() {{}},
}});
const hosts = {{ "app-name": anElement(), "buttons-left": anElement(), "buttons-right": anElement() }};
globalThis.document = {{
  documentElement: {{ dataset: {{}} }},
  getElementById: (id) => hosts[id],
  createElement: anElement,
  createElementNS: anElement,
}};
globalThis.window = {{ __TAURI__: {{
  core: {{ invoke: async (name) => name === "system_name" ? "windows" : name === "chrome_text" ? {text} : "" }},
  window: {{ getCurrentWindow: () => ({{
    isMaximized: async () => maximised,
    onResized: async (listener) => {{ heard = listener; return () => {{}}; }},
  }}) }},
}} }};
const {{ theChromeIsDrawn }} = await import("./chrome.mjs");
await theChromeIsDrawn();
const middle = () => hosts["buttons-right"].children[1];
const said = [[middle().attributes["aria-label"], middle().attributes.title]];
const settle = () => new Promise((done) => setTimeout(done, 0));
maximised = false; heard(); await settle();
said.push([middle().attributes["aria-label"], middle().attributes.title]);
maximised = true; heard(); await settle();
said.push([middle().attributes["aria-label"], middle().attributes.title]);
console.log(JSON.stringify(said));
"#
        );
        std::fs::write(folder.join("harness.mjs"), harness).expect("the harness is written");
        let Ok(ran) = std::process::Command::new("node").arg(folder.join("harness.mjs")).output() else {
            eprintln!("skipped: no Node on this computer");
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let said: Vec<Vec<String>> = serde_json::from_slice(&ran.stdout).expect("the page's report");
        let both = |word: &str| vec![word.to_string(), word.to_string()];
        assert_eq!(said, vec![both("Restore"), both("Maximise"), both("Restore")]);
    }

    /// The system name is the machine's own answer and nothing composed.
    #[test]
    fn the_system_name_is_the_machines_own() {
        assert_eq!(system_name(), std::env::consts::OS);
    }
}

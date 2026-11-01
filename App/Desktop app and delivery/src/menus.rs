//! **The four menus, and the keyboard combinations beside their items** —
//! `Desktop app and delivery` §18.1, §18.2, §18.3 and §18.5, K1 to K10 and K14
//! to K19.
//!
//! **Four menus and no fifth, and the list is Design's rather than this
//! document's**: Smoozy, Edit, Go, Window. **There is no File menu and no Help
//! menu** — *Report a problem…* went into the Smoozy menu on the Creator's
//! ruling of 2026-08-25, and that item was the whole of what Help held, so the
//! menu went with it. **This document builds the integration and adds no item**
//! (K5).
//!
//! ## Where the four go, and it is two arrangements rather than three
//!
//! - **macOS: the system's own bar, and that is not optional** (K1, K9). The
//!   bar is drawn by the system whether the application fills it or not, so a
//!   Smoozy that could not fill it would leave a Mac with an empty system menu
//!   bar and four menus with nowhere to go. **The shell exposes it**, which is
//!   what K9 held as a required property of whichever shell won.
//! - **Windows and Linux: the same four, drawn by the application behind one
//!   button in the title bar** (K2, K3, K10). **On Windows that is an
//!   application choice rather than a platform necessity** — the platform's own
//!   alternative is a window-owned bar the system positions just below the
//!   title bar, and Design chose the one button. **On Linux there is no
//!   platform provision at all**: freedesktop publishes no specification for an
//!   application menu bar, and the two toolkit routes that exist hang off their
//!   own toolkit's application object and are not available to a
//!   system-web-view shell.
//!
//! **The mockup draws a bar in the window on every system, with a line beside
//! it reading *"On a Mac this is the top of the screen, not the window"*. That
//! line is the mockup's and not Smoozy's**, and the mockup's own guidance says
//! so. On the Mac no bar is drawn in the window at all.
//!
//! ## The keys, and the mechanism splits with the arrangement
//!
//! **K14. The shell's accelerator works only with a platform-native menu bar.**
//! On macOS the native bar catches the keystroke and **the system draws the key
//! beside the item**. On Windows and Linux the four menus are the application's
//! own drawing, so **the label beside an item is drawn by the application and
//! catching the keystroke is a key handler in the page**. The shell publishes
//! no route for it there and none is waited for.
//!
//! **K15. The drawn `⌘` combinations map with Ctrl on the other two**, and
//! **where a system publishes no app-level convention for an act, the item
//! carries no combination on that system rather than an invented one**. Quit is
//! Ctrl+Q on Linux and **nothing on Windows**; Settings is Ctrl+Comma on Linux
//! and **nothing on Windows**; Hide, Minimise and Full screen carry their
//! combinations on macOS alone.
//!
//! **The macOS side of the map is deliberately almost empty, and that is not an
//! omission.** Every macOS combination the mockup draws except *Settings* and
//! the five rail numbers belongs to an act the system itself performs — the
//! clipboard three, Select all, Hide, Minimise, Zoom, Full screen, Quit — and
//! on macOS those are the platform's own menu items, carrying the platform's
//! own key. **Restating ⌘X here would be the same fact in two places**, which is
//! the shape of fault this stage has met more often than any other, and the
//! second copy would be the one nobody updated.
//!
//! **K16. Nothing drawn or mapped collides with anything any vendor reserves.**
//! Windows reserves combinations involving the Windows key, and F12; GNOME's
//! legacy reserved list covers window-switching, the window menu, close and
//! interrupt; freedesktop's one hard rule is the Meta/Super key.
//! [`tests::nothing_mapped_touches_anything_a_vendor_reserves`] is that held
//! rather than remembered, and the shell's own accelerator has no Meta/Super
//! modifier at all — deliberately commented out in its source.
//!
//! **K17. Three platform menu-key behaviours are knowingly absent**: Windows'
//! mnemonics (Alt plus an underlined letter), F10 to activate the menu bar and
//! Shift+F10 for a context menu; GNOME's F10 for the primary menu. **Those are
//! behaviours of a platform-drawn bar, and on Windows and Linux Smoozy's menus
//! are its own drawing.** Nothing promises them, the mockup draws none of them,
//! and **nobody adds one by imitation** —
//! [`tests::the_three_knowingly_absent_behaviours_stay_absent`].
//!
//! **K18 is an assumption and stays one.** On macOS the shell states outright
//! that the native edit menu is what makes the ordinary clipboard keys work at
//! all, which is one more reason K9's bar is not optional there. **On Windows
//! and Linux, whether cut, copy and paste keystrokes work in the web view
//! without a native menu is established by no publication read** — the web views
//! presumably handle editable fields themselves, and that is an assumption, not
//! a read fact (§22 item 52). **There is no Windows machine and no Linux
//! desktop computer and neither is coming**, so nothing here waits for an
//! answer. What holds without it is the menu items themselves, which act by
//! click through K7's focus routing.
//!
//! **K19's escalation is declined, and it was declined in the plan.** Every act
//! with a drawn key has a control of its own elsewhere in the app, so an engine
//! that could not carry a keystroke would cost the key and never a promise.

use crate::typography::Prose;

/// **Where this system's four menus go.**
///
/// Two arrangements rather than three: the system's own bar on macOS, the
/// application's own drawing on the other two (K1, K2, K3, K10).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Where {
    /// macOS. K9: not optional.
    InTheSystemsOwnBar,
    /// Windows and Linux, behind one button in the title bar.
    DrawnByTheApplication,
}

/// Read from the name the machine gives itself, which is the same reading the
/// window chrome takes.
pub fn where_the_menus_go(system: &str) -> Where {
    match system {
        "macos" => Where::InTheSystemsOwnBar,
        _ => Where::DrawnByTheApplication,
    }
}

/// **What an item does.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Act {
    /// **The system performs it.** On macOS these are the platform's own menu
    /// items, carrying the platform's own key and its own behaviour; on Windows
    /// and Linux the application performs the same act on its own window.
    TheSystems(TheSystemsAct),
    /// **K7: it acts on whatever has focus**, which may be an engine-drawn
    /// field or a native surface, and the integration routes it to whichever it
    /// is. **This is the one place H11's three-engine cost reaches the menus.**
    OnWhateverHasFocus(Focused),
    /// **The page is told, and draws.** Every one of these is a place the rail
    /// goes to, the one screen that owns it, or a box the menus open
    /// (`crate::surface::menus`, *the three items that open a box*).
    AGoing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheSystemsAct {
    Hide,
    Minimise,
    Zoom,
    FullScreen,
    Quit,
}

/// **K7 names three and Design's list holds four.** Cut, copy and paste are the
/// three the clause is about; Select all sits with them because the focused
/// thing decides it in exactly the same way, and because K15 maps it as one of
/// *the application's own combinations that no vendor publishes*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focused {
    Cut,
    Copy,
    Paste,
    SelectAll,
}

/// **A keyboard combination, in the one form both mechanisms are derived
/// from.**
///
/// K14 splits the mechanism and this is what keeps it from splitting the fact:
/// the shell's accelerator string and the label the application draws are both
/// **derived from this**, so the combination has one home.
///
/// **There is no Meta/Super/Windows-key modifier and there will not be one.**
/// It is the one rule freedesktop and KDE both publish — applications never use
/// that key — Windows reserves every combination involving it, and the shell's
/// own accelerator has it commented out. **Not having a field for it is how
/// that is kept.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Combination {
    pub cmd: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    /// The key itself, as the page's own reading spells it: a lower-case letter,
    /// a digit, or a punctuation character.
    pub key: &'static str,
}

impl Combination {
    const fn cmd(key: &'static str) -> Self {
        Combination { cmd: true, ctrl: false, shift: false, alt: false, key }
    }

    const fn ctrl(key: &'static str) -> Self {
        Combination { cmd: false, ctrl: true, shift: false, alt: false, key }
    }

    /// **The shell's own accelerator spelling**, which is what the native bar
    /// is built with on macOS and what nothing else reads.
    pub fn the_shells_accelerator(&self) -> String {
        let mut said = String::new();
        for (held, name) in [
            (self.cmd, "Cmd"),
            (self.ctrl, "Ctrl"),
            (self.alt, "Alt"),
            (self.shift, "Shift"),
        ] {
            if held {
                said.push_str(name);
                said.push('+');
            }
        }
        said.push_str(&self.key.to_uppercase());
        said
    }

    /// **The label the application draws beside the item**, which is only ever
    /// drawn on Windows and Linux: on macOS the system draws the key and this is
    /// never asked for.
    ///
    /// **The spelling is the platform's own** — the modifier written out, joined
    /// with a plus — which is integration rather than a redesign of the drawing.
    pub fn as_the_application_draws_it(&self) -> String {
        let mut said = String::new();
        for (held, name) in [
            (self.ctrl, "Ctrl"),
            (self.alt, "Alt"),
            (self.shift, "Shift"),
        ] {
            if held {
                said.push_str(name);
                said.push('+');
            }
        }
        said.push_str(&self.key.to_uppercase());
        said
    }
}

/// The three systems' answers for one item. **An absent one is K15's *no
/// combination rather than an invented one*.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keys {
    /// **macOS, and only where the act is not one the system performs.** For
    /// every item whose act is [`Act::TheSystems`] or
    /// [`Act::OnWhateverHasFocus`], the native bar carries the platform's own
    /// item with the platform's own key, and restating it here would put one
    /// fact in two places.
    pub macos: Option<Combination>,
    pub windows: Option<Combination>,
    pub linux: Option<Combination>,
}

impl Keys {
    const fn none() -> Self {
        Keys { macos: None, windows: None, linux: None }
    }

    /// The same combination on Windows and on Linux, and **the system's own on
    /// macOS** — the act being one the platform's own menu item performs.
    const fn ctrl_on_both(key: &'static str) -> Self {
        Keys {
            macos: None,
            windows: Some(Combination::ctrl(key)),
            linux: Some(Combination::ctrl(key)),
        }
    }

    /// The drawn `⌘` combination, mapped with Ctrl on the other two — K15's
    /// plain case, for an act the platform performs on none of the three.
    const fn cmd_here_ctrl_there(key: &'static str) -> Self {
        Keys {
            macos: Some(Combination::cmd(key)),
            windows: Some(Combination::ctrl(key)),
            linux: Some(Combination::ctrl(key)),
        }
    }

    /// The same, except that **Windows publishes no convention for the act**,
    /// so the item carries no combination there rather than an invented one
    /// (K15).
    const fn cmd_here_and_ctrl_on_linux_alone(key: &'static str) -> Self {
        Keys {
            macos: Some(Combination::cmd(key)),
            windows: None,
            linux: Some(Combination::ctrl(key)),
        }
    }

    pub fn on(&self, system: &str) -> Option<Combination> {
        match system {
            "macos" => self.macos,
            "windows" => self.windows,
            _ => self.linux,
        }
    }
}

/// One item of one menu.
#[derive(Debug, Clone, Copy)]
pub struct Item {
    /// What the item is called in code and on the wire. **Stable, and the page
    /// and the native bar both use it**, so the one act has one name.
    pub id: &'static str,
    /// **K8: menu item labels are prose the user reads and go through the
    /// typography point** (X5), even though no engine draws them. The string is
    /// held here as it was written and passes the point at
    /// [`Item::label`], which is the only way it leaves this file.
    label: &'static str,
    pub act: Act,
    pub keys: Keys,
}

impl Item {
    /// **The label, through the typography point** — K8 and X5.
    pub fn label(&self) -> Prose {
        Prose::new(self.label)
    }

    /// **Whether this system draws the item at all.**
    ///
    /// **One item answers no, and it is Hide.** On a Mac *Hide Smoozy* is the
    /// operating system's own item and the Dock is the way back. On Windows and
    /// Linux Smoozy draws its own menus, hiding the one window Smoozy has, and
    /// **there is nothing to bring it back**: no tray icon anywhere in this
    /// crate, no second window, and ~~a second launch starts a second copy
    /// rather than raising the hidden one~~ *(true until step 11's R1 — see
    /// below)*. So the act the item performs on those two
    /// systems is not *hide*, it is *make the application unreachable*, while
    /// the render and the transfers carry on and the task manager is the only
    /// thing left that can stop either.
    ///
    /// **This is K15's own discipline applied to the act instead of only to the
    /// key.** K15 gives Hide a combination on macOS alone and writes nothing on
    /// the other two, because neither system publishes a convention for it — *no
    /// combination rather than an invented one* — and K6 is the sentence behind
    /// it: *"on a platform that does not provide it, the application supplies
    /// only what that platform's own conventions supply. Nothing is invented to
    /// fill a gap a platform leaves."*
    ///
    /// **Since step 11's R1 there is a way back on all three systems**: a
    /// second launch no longer starts a second copy, it brings the running
    /// copy's window forward (P2, [`crate::the_one_instance`]). **Nothing here
    /// changes because of it.** The item stays undrawn on Windows and Linux: K15
    /// and K6 above are the reason it is not drawn, and they still hold. If
    /// Hide is ever wanted on those two, the way back now exists.
    ///
    /// **The alternative was making Hide minimise on those two, and it was
    /// refused**: the same menu already carries Minimise, so the user would meet
    /// two items doing one thing. **Ruled at the round's consolidation, C-1 and
    /// F-1, 2026-09-12**, and the whole reasoning is there. **If the item is
    /// ever wanted everywhere, the way back has to be built first.**
    pub fn is_drawn_on(&self, system: &str) -> bool {
        !matches!(self.act, Act::TheSystems(TheSystemsAct::Hide)) || system == "macos"
    }
}

/// A menu is items and the rules between them.
#[derive(Debug, Clone, Copy)]
pub enum Entry {
    AnItem(Item),
    ASeparator,
}

/// One of the four.
#[derive(Debug, Clone)]
pub struct Menu {
    pub id: &'static str,
    title: &'static str,
    pub entries: Vec<Entry>,
}

impl Menu {
    /// **K8 again: a menu's own title is prose the user reads.**
    pub fn title(&self) -> Prose {
        Prose::new(self.title)
    }
}

const fn item(id: &'static str, label: &'static str, act: Act, keys: Keys) -> Item {
    Item { id, label, act, keys }
}

/// **The four menus, in Design's own order, with Design's own items.**
///
/// K5: this document adds no item, and there is no fifth menu. The list is the
/// drawing's — `Preparation/Design/Mockup/Smoozy.html`, the `sysbar` — read item
/// for item.
pub fn the_four_menus() -> Vec<Menu> {
    vec![
        Menu {
            id: "smoozy",
            title: "Smoozy",
            entries: vec![
                Entry::AnItem(item("smoozy.about", "About Smoozy", Act::AGoing, Keys::none())),
                Entry::AnItem(item(
                    "smoozy.settings",
                    "Settings…",
                    Act::AGoing,
                    // **Linux publishes GNOME's Ctrl+Comma** — the one
                    // distribution family this stage's Linux evidence reads —
                    // with KDE's Ctrl+Shift+Comma recorded in the plan rather
                    // than drawn. **Windows publishes none.**
                    Keys::cmd_here_and_ctrl_on_linux_alone(","),
                )),
                Entry::AnItem(item(
                    "smoozy.check-for-updates",
                    "Check for updates…",
                    Act::AGoing,
                    Keys::none(),
                )),
                Entry::ASeparator,
                // **Report a problem lives here and in no other menu**, on the
                // Creator's ruling of 2026-08-25. It is the reason there is no
                // Help menu: it was the only thing one would have held.
                Entry::AnItem(item(
                    "smoozy.report-a-problem",
                    "Report a problem…",
                    Act::AGoing,
                    Keys::none(),
                )),
                Entry::ASeparator,
                // **Drawn and performed on macOS alone** — see
                // [`Item::is_drawn_on`], which carries the whole of why. On
                // Windows and Linux the item leaves the list the page is handed,
                // the way an absent key is absent rather than invented.
                Entry::AnItem(item(
                    "smoozy.hide",
                    "Hide Smoozy",
                    Act::TheSystems(TheSystemsAct::Hide),
                    Keys::none(),
                )),
                Entry::AnItem(item(
                    "smoozy.quit",
                    "Quit Smoozy",
                    Act::TheSystems(TheSystemsAct::Quit),
                    // **GNOME and KDE both publish Ctrl+Q. Windows publishes no
                    // app-level quit convention** and the system's own Alt+F4
                    // needs nothing from Smoozy.
                    Keys { macos: None, windows: None, linux: Some(Combination::ctrl("q")) },
                )),
            ],
        },
        Menu {
            id: "edit",
            title: "Edit",
            entries: vec![
                Entry::AnItem(item(
                    "edit.cut",
                    "Cut",
                    Act::OnWhateverHasFocus(Focused::Cut),
                    Keys::ctrl_on_both("x"),
                )),
                Entry::AnItem(item(
                    "edit.copy",
                    "Copy",
                    Act::OnWhateverHasFocus(Focused::Copy),
                    Keys::ctrl_on_both("c"),
                )),
                Entry::AnItem(item(
                    "edit.paste",
                    "Paste",
                    Act::OnWhateverHasFocus(Focused::Paste),
                    Keys::ctrl_on_both("v"),
                )),
                Entry::ASeparator,
                Entry::AnItem(item(
                    "edit.select-all",
                    "Select all",
                    Act::OnWhateverHasFocus(Focused::SelectAll),
                    Keys::ctrl_on_both("a"),
                )),
            ],
        },
        Menu {
            id: "go",
            title: "Go",
            entries: vec![
                // **The five rail numbers are the application's own
                // combinations that no vendor publishes**, mapped with Ctrl and
                // checked against the reserved lists and nothing else (K15).
                Entry::AnItem(item(
                    "go.servers",
                    "All servers",
                    Act::AGoing,
                    Keys::cmd_here_ctrl_there("1"),
                )),
                Entry::AnItem(item(
                    "go.queue",
                    "Queue",
                    Act::AGoing,
                    Keys::cmd_here_ctrl_there("2"),
                )),
                Entry::AnItem(item(
                    "go.history",
                    "History",
                    Act::AGoing,
                    Keys::cmd_here_ctrl_there("3"),
                )),
                Entry::AnItem(item(
                    "go.transfers",
                    "Transfers",
                    Act::AGoing,
                    Keys::cmd_here_ctrl_there("4"),
                )),
                Entry::AnItem(item(
                    "go.keys",
                    "Keys",
                    Act::AGoing,
                    Keys::cmd_here_ctrl_there("5"),
                )),
                Entry::ASeparator,
                Entry::AnItem(item(
                    "go.up-one-folder",
                    "Up one folder",
                    Act::AGoing,
                    Keys::none(),
                )),
            ],
        },
        Menu {
            id: "window",
            title: "Window",
            entries: vec![
                Entry::AnItem(item(
                    "window.minimise",
                    "Minimise",
                    Act::TheSystems(TheSystemsAct::Minimise),
                    Keys::none(),
                )),
                Entry::AnItem(item(
                    "window.zoom",
                    "Zoom",
                    Act::TheSystems(TheSystemsAct::Zoom),
                    Keys::none(),
                )),
                Entry::AnItem(item(
                    "window.full-screen",
                    "Full screen",
                    Act::TheSystems(TheSystemsAct::FullScreen),
                    Keys::none(),
                )),
            ],
        },
    ]
}

/// **Every item of the four, flattened.** The lookup the going and the drawn
/// bar both use, so one act has one definition.
pub fn every_item() -> Vec<Item> {
    the_four_menus()
        .into_iter()
        .flat_map(|menu| menu.entries)
        .filter_map(|entry| match entry {
            Entry::AnItem(item) => Some(item),
            Entry::ASeparator => None,
        })
        .collect()
}

/// The item this identifier names, where the four menus name one.
pub fn the_item(id: &str) -> Option<Item> {
    every_item().into_iter().find(|item| item.id == id)
}

// ── The native bar, on the one system that has one ──────────────────────────

/// **K9 and K1: macOS's own bar, filled.**
///
/// The predefined items are the platform's own — its behaviour, its placement
/// and **its own key, drawn by the system** (K14). The rest are ordinary items
/// carrying this file's own accelerator where K15 gives one, and their choosing
/// arrives as a menu event which [`crate::surface::menus`] turns into a going.
///
/// **On macOS the first submenu is the application's own** and the system draws
/// it under the application's name, which is why *Smoozy* is first: it is the
/// list Design wrote and it happens to sit where macOS puts one.
#[cfg(target_os = "macos")]
pub fn the_system_draws_the_bar<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> tauri::Result<tauri::menu::Menu<R>> {
    use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder};

    let mut bar = MenuBuilder::new(app);
    for menu in the_four_menus() {
        let mut drawn = SubmenuBuilder::new(app, menu.title().as_str());
        for entry in menu.entries {
            drawn = match entry {
                Entry::ASeparator => drawn.separator(),
                Entry::AnItem(item) => {
                    let label = item.label();
                    let label = label.as_str();
                    match item.act {
                        // The platform's own items. Their text is Design's and
                        // passes the typography point like every other label;
                        // their key and their behaviour are the platform's.
                        Act::TheSystems(TheSystemsAct::Hide) => {
                            drawn.item(&PredefinedMenuItem::hide(app, Some(label))?)
                        }
                        Act::TheSystems(TheSystemsAct::Quit) => {
                            drawn.item(&PredefinedMenuItem::quit(app, Some(label))?)
                        }
                        Act::TheSystems(TheSystemsAct::Minimise) => {
                            drawn.item(&PredefinedMenuItem::minimize(app, Some(label))?)
                        }
                        Act::TheSystems(TheSystemsAct::Zoom) => {
                            drawn.item(&PredefinedMenuItem::maximize(app, Some(label))?)
                        }
                        Act::TheSystems(TheSystemsAct::FullScreen) => {
                            drawn.item(&PredefinedMenuItem::fullscreen(app, Some(label))?)
                        }
                        // **K7 on macOS: the platform's own edit items are what
                        // route the three to whatever has focus**, and the
                        // shell states outright that this native edit menu is
                        // what makes the ordinary clipboard keys work at all
                        // (K18).
                        Act::OnWhateverHasFocus(Focused::Cut) => {
                            drawn.item(&PredefinedMenuItem::cut(app, Some(label))?)
                        }
                        Act::OnWhateverHasFocus(Focused::Copy) => {
                            drawn.item(&PredefinedMenuItem::copy(app, Some(label))?)
                        }
                        Act::OnWhateverHasFocus(Focused::Paste) => {
                            drawn.item(&PredefinedMenuItem::paste(app, Some(label))?)
                        }
                        Act::OnWhateverHasFocus(Focused::SelectAll) => {
                            drawn.item(&PredefinedMenuItem::select_all(app, Some(label))?)
                        }
                        Act::AGoing => {
                            let mut built = MenuItemBuilder::with_id(item.id, label);
                            if let Some(combination) = item.keys.macos {
                                built = built.accelerator(combination.the_shells_accelerator());
                            }
                            drawn.item(&built.build(app)?)
                        }
                    }
                }
            };
        }
        bar = bar.item(&drawn.build()?);
    }
    bar.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// **K5: four menus and no fifth, and the list is Design's.**
    #[test]
    fn there_are_four_menus_and_no_fifth() {
        let titles: Vec<String> = the_four_menus()
            .iter()
            .map(|menu| menu.title().as_str().to_string())
            .collect();
        assert_eq!(titles, vec!["Smoozy", "Edit", "Go", "Window"]);
    }

    /// **There is no File menu and no Help menu**, and *Report a problem…* is
    /// in the Smoozy menu, which is the Creator's ruling of 2026-08-25 and the
    /// reason Help went.
    #[test]
    fn there_is_no_file_menu_and_no_help_menu_and_report_a_problem_is_in_smoozys() {
        for menu in the_four_menus() {
            let title = menu.title().as_str().to_string();
            assert!(title != "File" && title != "Help", "a fifth menu appeared: {title}");
        }
        let report = the_item("smoozy.report-a-problem").expect("the item is there");
        assert_eq!(report.label().as_str(), "Report a\u{00A0}problem…");
        assert!(the_four_menus()[0]
            .entries
            .iter()
            .any(|entry| matches!(entry, Entry::AnItem(item) if item.id == "smoozy.report-a-problem")));
    }

    /// **K8 and X5: every label the user reads comes through the typography
    /// point, even though no engine draws them.**
    ///
    /// **This is half of what piece A recorded as owed**, X5 being unkeepable
    /// while none of the three surfaces it names existed. The check is on the
    /// type: a label handed out as a `String` would compile, would draw, and
    /// would have skipped the point.
    #[test]
    fn every_label_and_every_title_comes_by_the_prose_route() {
        for menu in the_four_menus() {
            let _: Prose = menu.title();
            for entry in &menu.entries {
                if let Entry::AnItem(item) = entry {
                    let _: Prose = item.label();
                }
            }
        }
        // One label with two words shows the substitution actually happening
        // rather than only being typed.
        assert_eq!(
            the_item("smoozy.about").expect("it is there").label().as_str(),
            "About\u{00A0}Smoozy"
        );
    }

    /// **Every identifier is its own**, because the going and the native bar
    /// both name an act by it.
    #[test]
    fn no_two_items_share_an_identifier() {
        let items = every_item();
        let unique: HashSet<&str> = items.iter().map(|item| item.id).collect();
        assert_eq!(unique.len(), items.len());
    }

    /// **K15's map, item by item, on the two systems that draw their own
    /// labels.**
    ///
    /// The three sentences the clause makes, checked rather than restated:
    /// **Quit is Ctrl+Q on Linux and nothing on Windows**; **Settings is
    /// Ctrl+Comma on Linux and nothing on Windows**; **Hide, Minimise and Full
    /// screen carry their combinations on macOS alone.**
    #[test]
    fn where_a_system_publishes_no_convention_the_item_carries_no_combination() {
        let drawn = |id: &str, system: &str| {
            the_item(id)
                .expect("the item is there")
                .keys
                .on(system)
                .map(|combination| combination.as_the_application_draws_it())
        };

        assert_eq!(drawn("smoozy.quit", "linux").as_deref(), Some("Ctrl+Q"));
        assert_eq!(drawn("smoozy.quit", "windows"), None);
        assert_eq!(drawn("smoozy.settings", "linux").as_deref(), Some("Ctrl+,"));
        assert_eq!(drawn("smoozy.settings", "windows"), None);

        for macos_alone in ["smoozy.hide", "window.minimise", "window.full-screen"] {
            assert_eq!(drawn(macos_alone, "linux"), None, "{macos_alone} on Linux");
            assert_eq!(drawn(macos_alone, "windows"), None, "{macos_alone} on Windows");
        }
        // Zoom carries one nowhere, on any system.
        for anywhere in ["macos", "windows", "linux"] {
            assert_eq!(the_item("window.zoom").expect("it is there").keys.on(anywhere), None);
        }
    }

    /// **The clipboard three and Select all map with Ctrl on the other two**,
    /// which is the modifier every app-level convention either vendor
    /// publishes uses.
    #[test]
    fn the_drawn_command_combinations_map_with_ctrl_on_the_other_two() {
        for (id, drawn) in [
            ("edit.cut", "Ctrl+X"),
            ("edit.copy", "Ctrl+C"),
            ("edit.paste", "Ctrl+V"),
            ("edit.select-all", "Ctrl+A"),
            ("go.servers", "Ctrl+1"),
            ("go.keys", "Ctrl+5"),
        ] {
            for system in ["windows", "linux"] {
                assert_eq!(
                    the_item(id)
                        .expect("the item is there")
                        .keys
                        .on(system)
                        .expect("it carries one")
                        .as_the_application_draws_it(),
                    drawn,
                    "{id} on {system}"
                );
            }
        }
    }

    /// **K16: nothing drawn or mapped collides with anything any vendor
    /// reserves.**
    ///
    /// Windows reserves exactly two things — combinations involving the Windows
    /// key, and F12. GNOME's legacy reserved list covers window-switching, the
    /// window menu, close and interrupt, every one of which is an Alt or a
    /// function-key combination. freedesktop's one hard rule is the Meta/Super
    /// key. **So the whole of the check is: no Meta/Super, no Alt, no function
    /// key** — and the first of those is kept by [`Combination`] having no field
    /// for it, which is stronger than a test.
    #[test]
    fn nothing_mapped_touches_anything_a_vendor_reserves() {
        for item in every_item() {
            for system in ["macos", "windows", "linux"] {
                let Some(combination) = item.keys.on(system) else {
                    continue;
                };
                assert!(
                    !combination.alt,
                    "{} on {system} uses Alt, and every combination GNOME \
                     publishes as reserved is an Alt one",
                    item.id
                );
                assert!(
                    !combination.key.starts_with('f') || combination.key.len() == 1,
                    "{} on {system} uses a function key. **Windows reserves \
                     F12**, GNOME reserves F10 for the primary menu, and K17 \
                     names the function-key behaviours as knowingly absent",
                    item.id
                );
                assert!(
                    combination.cmd || combination.ctrl,
                    "{} on {system} carries a bare key with no primary \
                     modifier, which would fire while the user was typing",
                    item.id
                );
            }
        }
    }

    /// **K17: the three knowingly-absent behaviours stay absent, and nobody
    /// re-adds one because it felt missing.**
    ///
    /// Windows' mnemonics (Alt plus an underlined letter), F10 to activate the
    /// menu bar, Shift+F10 for a context menu; GNOME's F10. **They are
    /// behaviours of a platform-drawn bar and Smoozy's are its own drawing on
    /// both those systems.** A later want here enters at Specs, not by a builder
    /// imitating the platform unasked.
    #[test]
    fn the_three_knowingly_absent_behaviours_stay_absent() {
        let page = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/menus.js"),
        )
        .expect("the page's menu module is readable");
        let (working, _) = include_str!("menus.rs")
            .split_once("#[cfg(test)]")
            .expect("the tests are the last thing in this file");

        for absent in ["\"F10\"", "'F10'", "accessKey"] {
            assert!(
                !page.contains(absent) && !working.contains(absent),
                "a menu-key behaviour K17 names as knowingly absent was added: \
                 {absent}. **Windows publishes mnemonics, F10 and Shift+F10; \
                 GNOME publishes F10** — and on Windows and Linux Smoozy's menus \
                 are its own drawing, so nothing anywhere promises them"
            );
        }
    }

    /// **The accelerator spelling the shell is given, and it is only ever the
    /// macOS one.**
    #[test]
    fn the_shells_accelerator_is_spelled_the_way_the_shell_reads_it() {
        assert_eq!(Combination::cmd(",").the_shells_accelerator(), "Cmd+,");
        assert_eq!(Combination::cmd("1").the_shells_accelerator(), "Cmd+1");
        assert_eq!(
            Combination { cmd: true, ctrl: true, shift: false, alt: false, key: "f" }
                .the_shells_accelerator(),
            "Cmd+Ctrl+F"
        );
    }

    /// **Every accelerator this file hands the shell is one the shell can
    /// read**, checked with the library the shell builds its own menus on.
    ///
    /// **This is the one silent failure in the whole of the menus.** The shell
    /// parses an accelerator with `.ok()` and **drops one it cannot read
    /// without a word** — so a mistyped combination produces a menu item that
    /// draws perfectly, works perfectly by click, and simply has no key beside
    /// it, on the one system where the key is the system's to draw. Nothing
    /// else in this crate would ever notice.
    #[test]
    fn every_accelerator_the_shell_is_handed_is_one_it_can_read() {
        for item in every_item() {
            let Some(combination) = item.keys.macos else {
                continue;
            };
            let spelled = combination.the_shells_accelerator();
            assert!(
                spelled.parse::<muda::accelerator::Accelerator>().is_ok(),
                "{}'s macOS accelerator `{spelled}` is one the shell cannot \
                 read, so it would be dropped in silence and the item would \
                 draw with no key beside it",
                item.id
            );
        }
    }

    /// **The macOS map holds only what the system does not already carry.**
    ///
    /// Every item whose act is the system's own, or which acts on whatever has
    /// focus, takes the platform's own menu item and with it the platform's own
    /// key. **A macOS combination on one of those would be the same fact in two
    /// places**, and the second copy is the one nobody updates.
    #[test]
    fn macos_restates_no_key_the_platform_already_carries() {
        for item in every_item() {
            match item.act {
                Act::AGoing => {}
                _ => assert!(
                    item.keys.macos.is_none(),
                    "{} restates a macOS combination for an act the platform's \
                     own menu item already carries",
                    item.id
                ),
            }
        }
        // And the ones that do carry their own are exactly Design's six.
        let carried: Vec<&str> = every_item()
            .iter()
            .filter(|item| item.keys.macos.is_some())
            .map(|item| item.id)
            .collect();
        assert_eq!(
            carried,
            vec![
                "smoozy.settings",
                "go.servers",
                "go.queue",
                "go.history",
                "go.transfers",
                "go.keys",
            ]
        );
    }

    /// **Hide is drawn on macOS alone, and every other item is drawn
    /// everywhere** — C-1 and F-1 of step 10a round 1.
    ///
    /// **What this stops.** A Windows user with three servers rendering presses
    /// the Menu button looking for Settings, reads down the list and chooses
    /// *Hide Smoozy*, because on every other program that means *get out of my
    /// way for a minute*. The window goes and **nothing brings it back**: no
    /// tray icon in this crate, no second window, and pressing the icon again
    /// starts a second copy that knows nothing about the first. They did not
    /// make a mistake — they chose an item Smoozy put in front of them.
    ///
    /// **The item stays in the four menus** rather than being deleted, because
    /// macOS draws it and it is that system's own. This is the line that fails
    /// if somebody gives it back to the other two without building the way back
    /// first.
    #[test]
    fn hide_is_drawn_on_the_mac_alone_and_nothing_else_is_narrowed() {
        let hide = the_item("smoozy.hide").expect("the item is there");
        assert!(hide.is_drawn_on("macos"));
        assert!(!hide.is_drawn_on("windows"));
        assert!(!hide.is_drawn_on("linux"));

        for item in every_item() {
            if item.id == "smoozy.hide" {
                continue;
            }
            for system in ["macos", "windows", "linux"] {
                assert!(
                    item.is_drawn_on(system),
                    "{} is no longer drawn on {system}. **K6: on a platform that \
                     does not provide it, the application supplies only what that \
                     platform's own conventions supply** — and narrowing a second \
                     item wants the same reasoning C-1 wrote for the first",
                    item.id
                );
            }
        }
    }

    /// **Two arrangements rather than three** — K1, K2, K3 and K10.
    #[test]
    fn the_mac_uses_the_systems_bar_and_the_other_two_draw_their_own() {
        assert_eq!(where_the_menus_go("macos"), Where::InTheSystemsOwnBar);
        assert_eq!(where_the_menus_go("windows"), Where::DrawnByTheApplication);
        assert_eq!(where_the_menus_go("linux"), Where::DrawnByTheApplication);
    }
}

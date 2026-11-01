//! **The four narrowing widths, the floor, and the drag seams** — §3.3 P11,
//! `Preparation/Design/Whole app.md`'s *The window*, and `State, identity and
//! recovery` §1.1 I3.
//!
//! This file is the application's half. `web/panes.js` is the page's half and
//! `web/the-drawing.css` carries the steps themselves; the three are one piece
//! and each says where the other two are.
//!
//! ## What this piece is
//!
//! Smoozy is one window that never scrolls as a whole, and **it answers to its
//! own window's width rather than to the screen**. Four narrowing steps drop or
//! shrink things in a fixed order; below the fourth there is **a floor, stated
//! on screen, naming the width**; and every seam between two panes drags, with
//! its width remembered between runs.
//!
//! ## The one thing that differs from the drawing, and it is not a value
//!
//! **The mockup narrows with CSS container queries and the application does
//! not.** The ruling is `Audits/Build/Step 10a/Ruling — how the narrowing is
//! built.md`, 2026-09-08, resting on `Audits/Build/Step 10a/The narrowing
//! mechanism across the three engines.md`. The short of it: container queries
//! need macOS 13 Ventura and the built artifact declares 10.13, so on every Mac
//! from 10.13 to 12 the four steps and the floor **silently did not happen at
//! all** — the rules were discarded whole by the parser, nothing was logged, and
//! **the *Smoozy needs a wider window* panel never appeared**, which is the one
//! thing that panel exists to prevent. §2.4 H12 forbids exactly that: no
//! interface behaviour is built on a capability that is not present on all three
//! engines.
//!
//! **So the mechanism is a measurement and a mark**, at the same five
//! boundaries, with every declaration in every step untouched. What it is built
//! on — a `resize` listener and an element's own width — is older than every
//! engine in the release matrix, so H12 has nothing left to bite here.
//!
//! ## The four things the ruling binds, and where each is kept
//!
//! 1. **The mark is computed once, synchronously, before the first paint**, or
//!    the window flashes its widest state at start-up. Kept at the top level of
//!    `web/panes.js`, which is module code and runs before the page is painted,
//!    and held by a test in this file.
//! 2. **Nothing is scheduled, ever.** No interval, no debounce timer that
//!    outlives a resize, no work on the resize path beyond writing the mark.
//!    §3.3 P9 says nothing on this side polls and P10 says there is exactly one
//!    timer and it is the update check. This sits with §2.6 H18's existing
//!    `MediaQueryList` listener — an event the engine raises — and the test in
//!    `crate::surface::the_event_route` over every page file is what stops it
//!    drifting into either.
//! 3. **The seams and the steps agree about which step is in force at the
//!    moment a drag begins.** That seam does not exist under container queries,
//!    where the engine keeps the rules and the layout in step by construction,
//!    and it exists here. It is closed in `web/panes.js` by the drag asking the
//!    same one function the mark writer asks, at pointerdown, before it starts.
//! 4. **The numbers' one home is the mockup.** Only the selector changed, so
//!    `web/smoozy.css`'s header rule stands, and that header is the one place
//!    the difference in mechanism is stated.
//!
//! ## How a clause marker is written in this crate
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document, at the site or in this
//! header list.
//!
//! **`State, identity and recovery`'s own markers are written out with that
//! document at every site below and are deliberately not listed here.** This
//! plan defines clauses of the same short names, and a header list cannot tell
//! the two apart — a bare one would resolve to a real and wrong clause.

use std::sync::Mutex;

use tauri::State;

use crate::record::pane_widths;
use crate::start::Started;
use crate::typography::Prose;

/// One narrowing step: the width at or below which it comes into force, and the
/// mark `web/panes.js` writes onto `.app` for it.
///
/// **The mark is a word in a space-separated list**, so a window at 800 pixels
/// carries `s1 s2 s3` and every step at or above it is in force at once —
/// which is exactly what the container queries did, four `max-width` rules all
/// matching, in the order they are written.
///
/// **The floor is the one exception and it is compared strictly** — see
/// [`THE_FLOOR`], which carries the whole of why.
pub struct Step {
    /// The width, in the drawing's own pixels.
    pub at: u32,
    /// The word written into `.app`'s mark.
    pub mark: &'static str,
}

/// **The five boundaries, in the order the drawing states them.**
///
/// **Their one home is the mockup** and this is a copy, exactly as the whole of
/// `web/the-drawing.css` is. A test in this file reads the mockup's own
/// `@container` rules and fails if these five have drifted from them, so the
/// copy cannot go quietly wrong.
///
/// The four steps are `Preparation/Design/Whole app.md`'s, in the order of what
/// the user can most afford to lose: the preview column, then the server list's
/// hardware and the file list's *Kind*, then the rail and list narrowing, then
/// the rail losing its words and keeping every place.
pub const THE_NARROWING_STEPS: [Step; 5] = [
    Step {
        at: 1180,
        mark: "s1",
    },
    Step {
        at: 1020,
        mark: "s2",
    },
    Step {
        at: 900,
        mark: "s3",
    },
    Step {
        at: 780,
        mark: "s4",
    },
    Step {
        at: 680,
        mark: "floor",
    },
];

/// **The floor. Below this Smoozy says the window is too small rather than
/// folding** — `Preparation/Design/Whole app.md`: *"A fifth step would mean
/// reflowing into a single tall strip, which is what a page does… So there is a
/// floor, it is stated on screen, and it names the width."*
///
/// **Below it, and not at it. Repaired 2026-09-12, step 10a round 1, U-1.** The
/// drawing turned the panel on at a window width of 680 or less and the panel's
/// own sentence asks the user to *make it at least 680 pixels wide* —
/// so a user who did exactly what Smoozy told them met the same panel, and only
/// 681 cleared it. **680 stays the one number**: the mockup compares strictly
/// (`@container win (width < 680px)`), `web/panes.js` compares strictly at this
/// one boundary, and no second number exists on either side.
pub const THE_FLOOR: u32 = 680;

/// **From this step down, nothing drags at all.**
///
/// Design's own words: *"From the third step down nothing drags at all: the
/// rail and the list have both taken the widths the app declares for them, so
/// there is nothing left on the line that is the user's to move."* The drawing
/// keeps it with `.app[data-narrow~="s3"] .seam { display: none; }`; this is the
/// same fact where the drag can read it, and a test holds the two together.
pub const THE_STEP_THAT_STOPS_THE_SEAMS: &str = "s3";

/// One seam between two panes, and what it may be dragged between.
///
/// **The bounds are Design's *"each pane has a smallest and a largest useful
/// width, and the drag stops at both"***, and their home is the mockup's own
/// seam markup. **A pane cannot be dragged into uselessness and then have to be
/// hunted for; and past its largest width a pane is only taking room from the
/// thing it sits beside.**
pub struct Seam {
    /// The key the width is stored under. **`State, identity and recovery` I3
    /// rule 1: one value per seam, and the set is closed** — never one key per
    /// screen, because the rail's width is one width everywhere and a list's
    /// width is one width for the queue and the history both.
    pub key: &'static str,
    /// The custom property the drawing declares the width in, which is what the
    /// dragged value is set on `.app` as, and what each narrowing step
    /// re-declares on `.body` from its own width down.
    pub property: &'static str,
    /// The smallest useful width.
    pub smallest: u32,
    /// The largest useful width.
    pub largest: u32,
    /// Whether the pane is measured from the right edge of its host. The
    /// preview column is, being dragged from its own edge.
    pub from_right: bool,
    /// What the seam is called, for anyone who cannot see it.
    pub label: &'static str,
}

/// **The three seams Design draws, and there is no fourth.**
///
/// *"Every seam between two panes drags. The rail against everything beside it,
/// a list against the entry beside it, and a job entry's main column against its
/// preview column."*
///
/// **Two of Design's own rules collapse many drawings onto one value each** —
/// the rail's width is one value for every screen, and a list's width is one
/// value shared by the queue and the history — **so this is a set of seams and
/// never one key per screen** (`State, identity and recovery` I3 rule 1). A
/// caller wanting a value this set does not name has found a seam Design did not
/// draw, which is a fault rather than a case this clause does not cover.
pub const THE_SEAMS: [Seam; 3] = [
    Seam {
        key: "rail",
        property: "--rail-w",
        smallest: 150,
        largest: 340,
        from_right: false,
        label: "Resize the rail",
    },
    Seam {
        key: "list",
        property: "--list-w",
        smallest: 280,
        largest: 620,
        from_right: false,
        label: "Resize the list",
    },
    Seam {
        key: "preview",
        property: "--side-w",
        smallest: 180,
        largest: 380,
        from_right: true,
        label: "Resize the preview",
    },
];

/// What a seam says when the pointer rests on it. One string for all three,
/// because it is one gesture.
const WHAT_A_SEAM_DOES: &str = "Drag to resize · double-click to reset";

/// One seam, as the page draws it — its shape, and the width this user left it
/// at if they have ever dragged it.
#[derive(serde::Serialize)]
pub struct SeamOnThePage {
    pub key: &'static str,
    pub property: &'static str,
    pub smallest: u32,
    pub largest: u32,
    pub from_right: bool,
    pub label: Prose,
    pub hint: Prose,
    /// **Absent means this user has not dragged this seam**, which is the
    /// ordinary state and never a failure (P11, and `State, identity and
    /// recovery` I3 rule 4). The page then sets nothing and the drawing's own
    /// declared width stands.
    pub width: Option<u32>,
}

/// The panel below the floor: what it says, and the width it names.
#[derive(serde::Serialize)]
pub struct TheFloorPanel {
    /// *Window too narrow.*
    pub said: Prose,
    /// *Make it at least 680 pixels wide.* **The width is named**,
    /// which is what Design asks of this panel, and it is composed from
    /// [`THE_FLOOR`] so the sentence cannot say one number while the mechanism
    /// uses another.
    pub how: Prose,
}

/// Everything the page needs to draw the seams and the floor, in one answer.
#[derive(serde::Serialize)]
pub struct ThePaneLayout {
    pub seams: Vec<SeamOnThePage>,
    pub floor: TheFloorPanel,
    /// **Whether the record would not answer, which is a different thing from
    /// holding no widths.** Added 2026-09-12, step 10a round 1 must-do F-6.
    ///
    /// **P11's last bullet cites L8a in as many words: *"an absent setting is an
    /// ordinary state, a read that did not answer is not."*** The two were not
    /// told apart — a read that failed produced an empty answer, which is the
    /// same answer a fresh installation gives, so every seam was drawn at the
    /// default with no word about it. **The user who set those widths
    /// deliberately met the default layout, dragged a seam to put it back, and
    /// that drag overwrote what was still in the record**, so the widths were
    /// then genuinely gone.
    ///
    /// **No record at all is a record that would not answer too.** Opening the
    /// local record creates it where it is not there, so a fresh installation
    /// holds an opened, empty record and reads every width as absent — which is
    /// `State, identity and recovery` I3 rule 4's absent **value**, silently and
    /// correctly. **A caller holding no record holds one that failed to open**
    /// (`Started::record`'s own note), and that is L8a's failure, not a fresh
    /// installation. Until 2026-09-19 this said the opposite and the user whose
    /// record would not open met the default widths with no word about it; the
    /// Gemini catch-up of step 10a, Area 4 (F-1).
    pub the_record_would_not_answer: bool,
    /// **What the user is told when it would not answer** — F-6's second half,
    /// finished 2026-09-12 at the close of round 1.
    ///
    /// **The Verdict's words are *a read that would not answer is the failure
    /// shape's***, and the shape is `web/the-failure-shape.js`'s one builder,
    /// which needs a sentence and nothing else. **It is not a failure record**:
    /// this module has no durable subject to raise one against and invents none.
    /// The round's own implementer read the clause as asking for a record,
    /// declined it on that ground, and left the page setting a mark nothing
    /// draws — so the user whose widths could not be read still met the default
    /// layout with no word about it, which is the silence F-6 exists to end.
    ///
    /// **The sentence comes from here because it is prose** (A6, X1): the page
    /// may not write a word of it, and `no_prose_is_written_into_the_page` in
    /// `src/surface/mod.rs` is what holds that.
    /// **The words are Design's and are owed** (§24), the same as the floor
    /// panel's above; what is fixed here is that the user is told at all.
    pub the_widths_could_not_be_read: Prose,
}

/// **The shape of the seams, the words of the floor, and whatever widths this
/// user has already set.**
///
/// **A command rather than an event, and that is not P6 being bent.** P6 is
/// about a module's *results* — a command is the page asking and an event is the
/// application telling. This is the page asking for facts it needs before it can
/// draw anything at all, which is the same shape `chrome_text` has had since
/// step 1, and it is what piece A's own note points at: a page that loads again
/// asks the application rather than remembering an event.
///
/// **H34: a closed request and response type**, with no length and no
/// enumeration coming in, and nothing beneath the application-service surface
/// going out.
///
/// **A record that holds no widths and a record that would not answer are two
/// different answers, and the page is given both.**
///
/// **An opened record holding no widths is the ordinary state** — `State,
/// identity and recovery` I3 rule 4 — and every width reads as absent, which is
/// the drawn default and what a fresh installation gets. **A read that would not
/// answer, or a record that could not be opened at all, says so**, and what the
/// page does with that is at [`ThePaneLayout::the_record_would_not_answer`].
#[tauri::command]
pub fn the_pane_layout(started: State<'_, Mutex<Started>>) -> ThePaneLayout {
    let Ok(started) = started.lock() else {
        // **The lock is held by a thread that died, which is not a record that
        // is absent.** Nothing here can read the widths and nothing here can
        // know whether they are there, so it is the same answer as a read that
        // would not answer rather than the fresh-installation one.
        return the_layout_that_could_not_be_read();
    };
    the_pane_layout_from(started.record.as_ref().map(|record| record.connection()))
}

/// The answer itself, with the record handed in — **so the whole of it can be
/// tested**, which the command cannot be: the shell's own managed state has no
/// constructor outside a running application.
///
/// **No record means the record could not be opened**, since opening creates
/// it where it is not there (`LocalRecord::open_at`), so it answers the same as
/// a read that would not answer — L8a. **An opened record with no widths reads
/// every width as absent**, which is the drawn default. Until 2026-09-19 no
/// record was answered as a fresh installation; the Gemini catch-up of step
/// 10a, Area 4 (F-1).
pub fn the_pane_layout_from(record: Option<&rusqlite::Connection>) -> ThePaneLayout {
    let Some(record) = record else {
        return the_layout_that_could_not_be_read();
    };
    let Ok(stored) = pane_widths::read_all(record) else {
        return the_layout_that_could_not_be_read();
    };
    ThePaneLayout {
        the_record_would_not_answer: false,
        the_widths_could_not_be_read: the_sentence_where_the_widths_would_not_read(),
        seams: THE_SEAMS
            .iter()
            .map(|seam| SeamOnThePage {
                key: seam.key,
                property: seam.property,
                smallest: seam.smallest,
                largest: seam.largest,
                from_right: seam.from_right,
                label: Prose::new(seam.label),
                hint: Prose::new(WHAT_A_SEAM_DOES),
                width: stored
                    .iter()
                    .find(|(key, _)| key == seam.key)
                    .map(|(_, width)| *width),
            })
            .collect(),
        floor: the_floor_panel(),
    }
}

/// **The answer where the widths could not be read at all.**
///
/// **Every width is absent here too, and that is not the same claim**: the page
/// is told which of the two it is, so it draws the defaults because there is
/// nothing else to draw and **does not write over what may still be in the
/// record**. A drag that wrote there would take the widths from *unread* to
/// genuinely gone, which is the harm F-6 names.
///
/// **The floor still answers**, being the window's own answer to being too small
/// rather than a screen, so it must not depend on the record at all.
fn the_layout_that_could_not_be_read() -> ThePaneLayout {
    ThePaneLayout {
        the_record_would_not_answer: true,
        the_widths_could_not_be_read: the_sentence_where_the_widths_would_not_read(),
        seams: THE_SEAMS
            .iter()
            .map(|seam| SeamOnThePage {
                key: seam.key,
                property: seam.property,
                smallest: seam.smallest,
                largest: seam.largest,
                from_right: seam.from_right,
                label: Prose::new(seam.label),
                hint: Prose::new(WHAT_A_SEAM_DOES),
                width: None,
            })
            .collect(),
        floor: the_floor_panel(),
    }
}

/// **The sentence the failure shape is given where the widths would not read**
/// — F-6's second half.
///
/// **It says what happened and claims nothing about what follows.** The seams
/// still drag and the drawn defaults are what is on screen; what the user is
/// told is that the widths they set could not be read, which is the fact the
/// page had no way to say. **It names no cause** — M6 and M13b — and it adds no
/// instruction, the page being usable either way.
///
/// **The words are Design's and are owed** (§24), the same as the floor panel's.
fn the_sentence_where_the_widths_would_not_read() -> Prose {
    Prose::new("Could not load your saved pane widths.")
}

/// The floor's own two sentences, both through the typography point.
///
/// **The width is composed from [`THE_FLOOR`]** so the sentence cannot name one
/// number while the mechanism uses another.
pub fn the_floor_panel() -> TheFloorPanel {
    TheFloorPanel {
        said: Prose::new("Window too narrow."),
        how: Prose::new(format!(
            "Make it at least {} wide.",
            crate::typography::a_figure_with_its_unit(THE_FLOOR, "pixels")
        )),
    }
}

/// **The drag ended.** P11: *"a dragged pane width is durable state, written at
/// the moment the drag ends, and the user's own drag is its only writer"*, and
/// `State, identity and recovery` I3 rule 3. That rule is
/// `State, identity and recovery` D6 applied — the release is the moment the
/// fact becomes true, so a crash costs no width already set.
///
/// **H34's validation, and both halves are here rather than in the page.** The
/// seam has to be one of [`THE_SEAMS`] — anything else is refused and written
/// nowhere — and the width is clamped to that seam's own smallest and largest,
/// which is Design's *"a pane cannot be dragged into uselessness."* The page
/// clamps as it drags so the pointer and the pane agree; this clamps again
/// because a command is reachable without the page's own arithmetic.
///
/// **Nothing is written on exit and nothing is written while the pointer moves.**
#[tauri::command]
pub fn a_pane_drag_ended(started: State<'_, Mutex<Started>>, seam: String, width: u32) {
    with_the_record(&started, |connection| {
        a_drag_ended_on(Some(connection), &seam, width);
    });
}

/// The write itself, with the record handed in, **so both halves of H34's
/// validation can be tested**: the seam has to be one of [`THE_SEAMS`] and the
/// width is clamped to that seam's own two stops.
pub fn a_drag_ended_on(record: Option<&rusqlite::Connection>, seam: &str, width: u32) {
    let Some(declared) = THE_SEAMS.iter().find(|declared| declared.key == seam) else {
        return;
    };
    let Some(connection) = record else {
        return;
    };
    let width = width.clamp(declared.smallest, declared.largest);
    let _ = pane_widths::a_drag_ended(connection, declared.key, width);
}

/// **The seam was put back where it started** — Design's *"double-clicking a
/// seam puts it back where it started. That is the way out of a mess, and it is
/// the only reset there is."*
///
/// **It clears the stored value rather than writing the drawn default into it**
/// (`State, identity and recovery` I3 rule 4), so *default* has one
/// representation and a default Design later changes reaches this user too.
#[tauri::command]
pub fn a_pane_width_was_reset(started: State<'_, Mutex<Started>>, seam: String) {
    with_the_record(&started, |connection| {
        the_seam_was_reset_on(Some(connection), &seam);
    });
}

/// The reset itself, with the record handed in.
pub fn the_seam_was_reset_on(record: Option<&rusqlite::Connection>, seam: &str) {
    let Some(declared) = THE_SEAMS.iter().find(|declared| declared.key == seam) else {
        return;
    };
    let Some(connection) = record else {
        return;
    };
    let _ = pane_widths::the_seam_was_reset(connection, declared.key);
}

/// Do something with the record, if there is one.
///
/// **Behind the one lock**, which is the record's own shape rather than a
/// convenience: `State, identity and recovery` §1.3 rule 1 is one writer per
/// durable store, and `main.rs` holds the whole spine behind this lock for that
/// reason.
fn with_the_record(started: &State<'_, Mutex<Started>>, act: impl FnOnce(&rusqlite::Connection)) {
    let Ok(started) = started.lock() else {
        return;
    };
    let Some(record) = started.record.as_ref() else {
        return;
    };
    act(record.connection());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn the_page_folder() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("web")
    }

    fn the_drawing() -> String {
        std::fs::read_to_string(the_page_folder().join("the-drawing.css"))
            .expect("the drawing is readable")
    }

    fn the_page_module() -> String {
        std::fs::read_to_string(the_page_folder().join("panes.js"))
            .expect("the page module is readable")
    }

    /// The mockup's own stylesheet, between its `<style>` tags.
    fn the_mockups_stylesheet() -> String {
        let mockup = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../Preparation/Design/Mockup/Smoozy.html"),
        )
        .expect("the mockup is readable");
        let opens = mockup.find("<style>").expect("the mockup has a stylesheet") + "<style>".len();
        let closes = mockup.find("</style>").expect("the stylesheet ends");
        mockup[opens..closes].to_string()
    }

    /// **Every width the mockup's own container queries name, in the order they
    /// are written, with repeats dropped.**
    ///
    /// **Two forms are read rather than one.** Four steps are written
    /// `@container win (max-width: Npx)` and the floor is written
    /// `@container win (width < Npx)` — the strict comparison U-1 asked for, so
    /// that 680 stays the one number the drawing carries. A reader that knew
    /// only the first form would find four boundaries where there are five and
    /// would fail with the wrong account of why.
    fn the_mockups_boundaries() -> Vec<u32> {
        let mockup = without_comments(&the_mockups_stylesheet());
        let mut found = Vec::new();
        for (at, _) in mockup.match_indices("@container win (") {
            let rest = &mockup[at + "@container win (".len()..];
            let closes = rest.find("px)").expect("a container query names a width");
            let digits: String = rest[..closes]
                .chars()
                .rev()
                .take_while(char::is_ascii_digit)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            let width: u32 = digits.parse().expect("the width is a number");
            if !found.contains(&width) {
                found.push(width);
            }
        }
        found
    }

    /// Take every CSS comment out.
    fn without_comments(text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        let mut rest = text;
        while let Some(opens) = rest.find("/*") {
            out.push_str(&rest[..opens]);
            match rest[opens + 2..].find("*/") {
                Some(closes) => rest = &rest[opens + 2 + closes + 2..],
                None => return out,
            }
        }
        out.push_str(rest);
        out
    }

    /// **Every declaration in a stylesheet, one per entry, in no order.**
    ///
    /// A declaration block is the contents of a `{ … }` that holds no `{` of its
    /// own, which is exactly a rule's body and never an at-rule's. Selectors are
    /// deliberately not collected: **the transform this piece made is a change
    /// of selector, so comparing selectors would compare the one thing that is
    /// meant to differ.**
    fn declarations_in(text: &str) -> Vec<String> {
        let text = without_comments(text);
        let bytes: Vec<char> = text.chars().collect();
        let mut out = Vec::new();
        let mut at = 0;
        while at < bytes.len() {
            if bytes[at] != '{' {
                at += 1;
                continue;
            }
            let mut end = at + 1;
            let mut nested = false;
            while end < bytes.len() && bytes[end] != '}' {
                if bytes[end] == '{' {
                    nested = true;
                    break;
                }
                end += 1;
            }
            if nested || end >= bytes.len() {
                at += 1;
                continue;
            }
            let body: String = bytes[at + 1..end].iter().collect();
            for declaration in body.split(';') {
                let declaration = declaration.split_whitespace().collect::<Vec<_>>().join(" ");
                if !declaration.is_empty() {
                    out.push(declaration);
                }
            }
            at = end + 1;
        }
        out.sort();
        out
    }

    /// **The shipped drawing is the mockup's stylesheet with the mechanism
    /// changed, and with nothing else changed at all.**
    ///
    /// **This is the test the whole lift rests on.** `web/smoozy.css`'s header
    /// says the numbers' one home is the mockup, and until now that was a
    /// sentence somebody had to remember. It is a comparison now: every
    /// declaration in the shipped sheet is a declaration in the mockup and the
    /// other way round, so **a colour, a size, a radius or a width changed in
    /// one file and not the other fails here.**
    ///
    /// **Exactly two declarations differ and both are the ruling's.** `.app`
    /// drops `container-type: inline-size` and `container-name: win`; nothing
    /// else moves, because the transform prefixes selectors and touches no
    /// declaration.
    #[test]
    fn the_drawing_is_the_mockups_stylesheet_with_the_mechanism_changed() {
        // About's local reader styles live in menus.css, not the shared sheet.
        // Compare that exact block separately; every shared declaration still matches.
        let original = the_mockups_stylesheet();
        let marker = "/* About's offline licence reader shares its box and keeps long text inside it. */";
        let end_marker = "/* End About styles. */";
        let (before, about) = original.split_once(marker).expect("About has its local styles");
        let (about, after) = about.split_once(end_marker).expect("About's styles end");
        let menus = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("web/menus.css")
        ).expect("About's stylesheet is readable");
        let shipped_about = menus.split_once(marker).unwrap().1.split_once(end_marker).unwrap().0;
        assert_eq!(declarations_in(about), declarations_in(shipped_about), "About's local styles drifted");
        let mut mockup = declarations_in(&format!("{before}{after}"));
        let shipped = declarations_in(&the_drawing());

        for gone in ["container-type: inline-size", "container-name: win"] {
            let at = mockup
                .iter()
                .position(|declaration| declaration == gone)
                .unwrap_or_else(|| {
                    panic!(
                        "the mockup no longer declares `{gone}` on .app. **The lift's \
                         whole subject is that declaration**, so if it has gone from \
                         the drawing the ruling of 2026-09-08 wants re-reading before \
                         this test is changed"
                    )
                });
            mockup.remove(at);
        }

        assert_eq!(
            shipped, mockup,
            "the shipped drawing and the mockup no longer carry the same \
             declarations. **The numbers' one home is the mockup** — change a \
             value there first and then here, and never only here. The lift is \
             `Audits/Build/Step 10a/Piece B — what landed.md`, and the one \
             difference this test allows is the two container properties the \
             ruling of 2026-09-08 took off `.app`"
        );
    }

    /// **Nothing the application ships rests on a container query** — §2.4 H12.
    ///
    /// **This is the fault the whole piece exists to remove.** A container query
    /// an engine has never heard of is not a degraded layout, it is no narrowing
    /// at all, silently: the at-rule is discarded whole by the parser, nothing is
    /// logged, and the *Smoozy needs a wider window* panel never appears. On
    /// every Mac from 10.13 to 12 that is what the drawing's own mechanism did.
    #[test]
    fn nothing_the_application_ships_rests_on_a_container_query() {
        for entry in std::fs::read_dir(the_page_folder()).expect("the page folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if !path.is_file() {
                continue;
            }
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .expect("the file has a name")
                .to_string();
            let text = std::fs::read_to_string(&path).expect("the file is readable");
            // The prose above and below says these words in order to forbid
            // them, so a comment is not an instance.
            let text = without_comments(&text);
            let text: String = text
                .lines()
                .filter(|line| !line.trim_start().starts_with("//"))
                .collect::<Vec<_>>()
                .join("\n");
            for capability in ["@container", "container-type", "container-name"] {
                assert!(
                    !text.contains(capability),
                    "web/{name} uses {capability}. **H12: no interface behaviour is \
                     built on a capability that is not present on all three \
                     engines**, and this one needs macOS 13 while the built \
                     artifact declares 10.13 — so on every Mac below Ventura the \
                     four steps and the 680 floor would silently not happen at \
                     all, with nothing logged and no panel shown"
                );
            }
        }
    }

    /// **The five boundaries are the mockup's, and the drawing and the page
    /// agree with them.**
    ///
    /// The numbers now live in three places — the mockup, which is their home;
    /// the drawing's step selectors; and the page's own step table, which is
    /// what writes the mark. **They cannot go quietly out of step**, which is
    /// what this asserts, and it is the reason the copy is acceptable at all.
    #[test]
    fn the_five_boundaries_are_the_mockups_and_the_drawing_and_the_page_agree() {
        let mut in_the_mockup = the_mockups_boundaries();
        in_the_mockup.sort_by(|a, b| b.cmp(a));

        let declared: Vec<u32> = THE_NARROWING_STEPS.iter().map(|step| step.at).collect();
        assert_eq!(
            in_the_mockup, declared,
            "the mockup narrows at different widths from the ones this piece \
             marks. **The numbers' one home is the mockup**, so the mockup is \
             right and THE_NARROWING_STEPS moves — and `web/the-drawing.css` is \
             re-lifted with it"
        );

        let drawing = the_drawing();
        let page = the_page_module();
        for step in &THE_NARROWING_STEPS {
            assert!(
                drawing.contains(&format!("[data-narrow~=\"{}\"]", step.mark)),
                "the drawing has no rule keyed to the {} mark, so a step of the \
                 narrowing draws nothing",
                step.mark
            );
            assert!(
                page.contains(&format!("{{ at: {}, mark: \"{}\" }}", step.at, step.mark)),
                "web/panes.js's step table no longer carries {} at {}, so the \
                 mark the page writes and the rules the drawing keys to it have \
                 come apart — and nothing on screen would say so, because the \
                 step would simply never come into force",
                step.mark,
                step.at
            );
        }

        assert_eq!(
            THE_NARROWING_STEPS[4].at, THE_FLOOR,
            "the floor and the last step are two different widths"
        );
    }

    /// **The floor states itself and names the width** —
    /// `Preparation/Design/Whole app.md`: *"So there is a floor, it is stated on
    /// screen, and it names the width."*
    ///
    /// The sentence is composed from [`THE_FLOOR`] rather than written out, so
    /// the words and the mechanism cannot say two different numbers.
    #[test]
    fn the_floor_states_itself_and_names_the_width() {
        let panel = the_floor_panel();
        assert!(
            panel.how.as_str().contains(&THE_FLOOR.to_string()),
            "the floor panel does not name the width"
        );
        assert!(!panel.said.as_str().is_empty());
        // **X1: both sentences came by the prose route.** A field that took a
        // `String` would compile and would draw and would have skipped the
        // typography point; this is the line that would fail.
        let _: &Prose = &panel.said;
        let _: &Prose = &panel.how;
    }

    /// **The drawing turns the floor panel on at the floor's own mark, and the
    /// panel is off at every other width.**
    #[test]
    fn the_floor_panel_is_drawn_only_below_the_floor() {
        let drawing = the_drawing();
        assert!(
            drawing.contains(".toosmall {"),
            "the floor panel has no rule at all"
        );
        assert!(
            drawing.contains(".app[data-narrow~=\"floor\"] .toosmall { display: grid; }"),
            "the floor panel is no longer turned on by the floor's own mark"
        );
    }

    /// **The panel appears below the width it names and not at it** — U-1 of
    /// step 10a round 1, and this is the test that would have caught it.
    ///
    /// **The fault it holds.** The drawing turned the panel on at 680 or less
    /// while the panel's own sentence says *make it at least 680 pixels
    /// wide*. A user who dragged to exactly 680 did what Smoozy asked and met
    /// the same panel; only 681 cleared it. **The number the user is told is the
    /// right one** — it is the width that is wide enough — **so the boundary
    /// moved and the sentence did not**, and no second number appears anywhere.
    ///
    /// The three places it has to hold together: the mockup, which is the
    /// number's one home; `web/panes.js`, which writes the mark; and the
    /// sentence, which is composed from [`THE_FLOOR`] and is checked above.
    #[test]
    fn the_floor_is_below_the_width_it_names_and_not_at_it() {
        let mockup = the_mockups_stylesheet();
        assert!(
            mockup.contains(&format!("@container win (width < {THE_FLOOR}px)")),
            "the mockup no longer turns the floor panel on **below** {THE_FLOOR} \
             pixels. **The panel asks the user to make it at least \
             {THE_FLOOR} pixels wide**, so at {THE_FLOOR} exactly they have done what \
             Smoozy told them and the panel must be gone"
        );
        assert!(
            !without_comments(&mockup).contains(&format!("(max-width: {THE_FLOOR}px)")),
            "the floor is back to an at-or-below comparison, which makes the \
             panel's own sentence false at exactly the width it names"
        );

        let page = the_page_module();
        assert!(
            page.contains(&format!(
                "const THE_FLOOR_MARK = \"{}\";",
                THE_NARROWING_STEPS[4].mark
            )),
            "web/panes.js and this file name different marks as the floor's, so \
             the strict comparison below would be applied to the wrong step"
        );
        assert!(
            page.contains("step.mark === THE_FLOOR_MARK ? across < step.at : across <= step.at"),
            "web/panes.js no longer compares the floor strictly. **The four steps \
             come into force at or below their width and the floor comes into \
             force below it**, because the floor is the one boundary with a \
             sentence attached and that sentence names the width"
        );
        // And the four steps are untouched: only the floor is strict, so a
        // window at exactly 1180 still carries `s1`.
        assert_eq!(the_mockups_boundaries().len(), THE_NARROWING_STEPS.len());
    }

    /// **Every seam this piece draws is one Design declared, and every one of
    /// them stops at both ends** — `Preparation/Design/Whole app.md`'s *"a pane
    /// cannot be dragged into uselessness"*, and `State, identity and recovery`
    /// `State, identity and recovery` I3 rule 1's closed set.
    #[test]
    fn the_seams_are_the_three_design_draws_and_each_stops_at_both_ends() {
        let keys: Vec<&str> = THE_SEAMS.iter().map(|seam| seam.key).collect();
        assert_eq!(
            keys,
            vec!["rail", "list", "preview"],
            "the set of seams has moved. **`State, identity and recovery` I3 \
             rule 1: one value per seam and the \
             set is closed** — a caller needing a value the set does not name has \
             found a seam Design did not draw"
        );

        let drawing = the_drawing();
        for seam in &THE_SEAMS {
            assert!(
                seam.smallest < seam.largest,
                "{}'s two stops are the wrong way round",
                seam.key
            );
            assert!(
                drawing.contains(&format!("{}:", seam.property)),
                "the drawing declares no {} for the {} seam to move",
                seam.property,
                seam.key
            );
        }
    }

    /// **P11's three keys, said in the plan's own words.**
    ///
    /// *"One stored value per key: one width per declared seam, one rail width
    /// for every screen, and one list width shared by the queue and the
    /// history."* **A key per screen is the mistake this guards** — it would
    /// draw correctly and would give a user who set the rail on one screen a
    /// different rail on the next.
    #[test]
    fn one_value_per_seam_and_never_one_per_screen() {
        for key in ["rail", "list", "preview"] {
            assert_eq!(
                THE_SEAMS.iter().filter(|seam| seam.key == key).count(),
                1,
                "{key} is stored under more than one key"
            );
        }
        let page = the_page_module();
        for screen in ["queue", "history", "servers", "settings"] {
            assert!(
                !page.contains(&format!("\"{screen}-")),
                "the page keys a pane width to a screen. **P11: one rail width \
                 for every screen, and one list width shared by the queue and \
                 the history**"
            );
        }
    }

    /// **The mark is computed once, synchronously, before the first paint.**
    ///
    /// The ruling names this as *"the cost that gets forgotten"*: the window
    /// flashes its widest state at start-up if the first mark is written from a
    /// callback. The module's own top level is what makes it synchronous — it is
    /// evaluated when the page's module graph loads, before anything is painted
    /// and before `smoozy.js`'s own start runs.
    #[test]
    fn the_mark_is_written_synchronously_when_the_module_loads() {
        let page = the_page_module();
        let working: String = page
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            working.contains("\ntheNarrowingIsMarked();"),
            "web/panes.js no longer writes the mark at its own top level. **The \
             ruling of 2026-09-08: the mark is computed once, synchronously, \
             before the first paint**, or the window flashes its widest state at \
             start-up — which is the cost that gets forgotten"
        );
    }

    /// **The seams and the steps agree about which step is in force at the
    /// moment a drag begins.**
    ///
    /// **This seam does not exist under container queries and it exists here**,
    /// and the ruling names it as this piece's to close. Under the engine's own
    /// mechanism the rules and the layout move in one act; here the mark is
    /// written by a listener, so between a resize and that listener a seam could
    /// be pressed at a step whose rules say it does not drag.
    ///
    /// **It is closed by the drag asking rather than by the drag trusting**: the
    /// first thing the pointerdown path does is call the same one function the
    /// mark writer calls, which measures, writes the mark and answers with the
    /// step in force.
    #[test]
    fn a_drag_asks_which_step_is_in_force_before_it_begins() {
        let page = the_page_module();
        let (_, from_pointerdown) = page
            .split_once("\"pointerdown\"")
            .expect("web/panes.js binds a pointerdown");
        let opening = &from_pointerdown[..from_pointerdown
            .find("addEventListener")
            .unwrap_or(from_pointerdown.len())];
        assert!(
            opening.contains("theNarrowingIsMarked()"),
            "the drag no longer asks which step is in force at the moment it \
             begins. **The ruling of 2026-09-08 names this seam as piece B's to \
             close**: the mark is written by a listener rather than by the \
             engine, so a drag that trusts the last mark can start at a step \
             whose own rules say nothing drags"
        );
        assert!(
            opening.contains("THE_STEP_THAT_STOPS_THE_SEAMS"),
            "the drag no longer refuses to begin at the step that stops the \
             seams. Design: *from the third step down nothing drags at all*"
        );
        assert!(
            page.contains(&format!(
                "const THE_STEP_THAT_STOPS_THE_SEAMS = \"{THE_STEP_THAT_STOPS_THE_SEAMS}\";"
            )),
            "the page and the application name different steps as the one that \
             stops the seams"
        );
        assert!(
            the_drawing().contains(&format!(
                ".app[data-narrow~=\"{THE_STEP_THAT_STOPS_THE_SEAMS}\"] .seam {{ display: none; }}"
            )),
            "the drawing no longer hides the seams from the step that stops them, \
             so the drawing and the drag disagree about where dragging ends"
        );
    }

    /// **A width is written at the end of a drag and at no other moment** —
    /// P11, and `State, identity and recovery` I3 rule 3.
    ///
    /// **The narrowing steps write nothing.** A step's re-declared width is a
    /// drawing-time computation over the stored value and the window's width,
    /// which is what makes *set aside, not destroyed* true by construction — the
    /// stored width is never overwritten by a step, and widening the window
    /// finds it exactly where the user left it. **This is the check that the
    /// mark writer holds no route to the write.**
    #[test]
    fn the_page_writes_a_width_from_the_drag_and_from_nowhere_else() {
        let page = the_page_module();
        assert_eq!(
            page.matches("\"a_pane_drag_ended\"").count(),
            1,
            "web/panes.js calls the width write from more than one place. **P11: \
             the user's own drag is its only writer**"
        );
        let (before_the_write, _) = page
            .split_once("\"a_pane_drag_ended\"")
            .expect("the page writes a width somewhere");
        assert!(
            before_the_write.contains("function theDragEnded")
                || before_the_write.contains("theDragEnded"),
            "the one width write is no longer on the drag's own ending path"
        );

        let mark = page
            .split_once("function theNarrowingIsMarked")
            .expect("the page marks the narrowing")
            .1;
        let mark = &mark[..mark.find("\n}").unwrap_or(mark.len())];
        // **A press that moves nothing is not a drag.** The reset gesture is a
        // double-click, so each of its two presses reaches the drag's own
        // ending path; without this guard each one writes the width again,
        // which is a write at a moment P11 does not admit. It was found by
        // driving a real pointer at the real module.
        let drag = page
            .split_once("function oneSeamDrags")
            .expect("the page makes a seam drag")
            .1;
        assert!(
            drag.contains("if (width === null) return;"),
            "the drag's ending path no longer refuses a press that moved \
             nothing. **P11: the write happens at the end of the drag and at no \
             other moment** — and the double-click that resets a seam is two \
             presses that move nothing"
        );

        assert!(
            !mark.contains("invoke"),
            "the mark writer calls the application. **P11: the narrowing steps \
             write nothing**, and a width the narrowing wrote back would be the \
             failure Design's own sentence warns against — the user's width \
             overwritten at the first narrow window, with nothing left to come \
             back to"
        );
    }

    /// **H26 and H27: every font stack ends in a generic that always matches.**
    ///
    /// H26's own words: the standard divides generics into ones that *"will
    /// always match a locally installed font"* and ones that may not, and
    /// `sans-serif` and `monospace` are of the first kind. **So no interface
    /// behaviour rests on a capability that might be absent: the worst case is a
    /// different face, never a failure.** H27 is why the earlier entries are
    /// safe — an engine that has never heard of `ui-monospace` reads it as a
    /// family name that matches nothing and moves on, which is also why a
    /// generic keyword is never quoted.
    #[test]
    fn every_font_stack_ends_in_a_generic_that_always_matches() {
        let drawing = without_comments(&the_drawing());
        let mut found = 0;
        for face in ["--face-ui:", "--face-mono:"] {
            let at = drawing
                .find(face)
                .unwrap_or_else(|| panic!("the drawing declares {face}"));
            let rest = &drawing[at + face.len()..];
            let ends = rest.find(';').expect("a token declaration ends");
            let stack = rest[..ends].trim();
            found += 1;
            assert!(
                stack.ends_with("sans-serif") || stack.ends_with("monospace"),
                "the face stack `{stack}` does not end in a generic that always \
                 matches. **H26: `sans-serif` and `monospace` are the two the \
                 standard says will always match a locally installed font**, and \
                 each stack ending in one is what keeps H12 off the typography"
            );
            for quoted in [
                "\"system-ui\"",
                "\"ui-monospace\"",
                "\"sans-serif\"",
                "\"monospace\"",
            ] {
                assert!(
                    !stack.contains(quoted),
                    "a generic keyword is quoted in `{stack}`. **H27: the standard \
                     notes that generic keywords cannot be quoted**, and a quoted \
                     one is read as a family name that matches nothing"
                );
            }
        }
        assert!(found >= 2, "the two faces are no longer declared as tokens");
    }

    /// **H22: every token is declared four times and three `color-scheme`
    /// declarations sit beside them.**
    ///
    /// H22's own words: the bare root, `@media (prefers-color-scheme: dark)`,
    /// and the two explicit-declaration blocks — *"the bare root declares `light
    /// dark`, which covers the system's own dark as well and is why the media
    /// block needs none, and each of the two `:root[data-theme]` blocks declares
    /// the one it is."*
    ///
    /// **The two explicit blocks are inert in the application and are kept
    /// anyway**, which H22 also says outright: nothing in Smoozy sets that
    /// attribute because Smoozy has no theme switch, and they are kept *"only so
    /// the two files stay one file."* Step 1 carried two blocks rather than
    /// four; the lift of 2026-09-08 brought all four across, which is what H22
    /// asks for and what lets the two files be compared declaration for
    /// declaration.
    #[test]
    fn the_tokens_are_declared_four_times_with_three_colour_schemes() {
        let drawing = without_comments(&the_drawing());
        assert_eq!(
            drawing.matches("--bg:").count(),
            4,
            "the palette is no longer declared four times. **H22: the bare root, \
             the dark media block, and the two explicit-declaration blocks**"
        );
        // `prefers-color-scheme:` is a media feature and not one of the three,
        // so it is taken back off the count.
        assert_eq!(
            drawing.matches("color-scheme:").count()
                - drawing.matches("prefers-color-scheme:").count(),
            3,
            "there are no longer three colour-scheme declarations. **H19: the \
             declaration is what reaches the surfaces the engine draws and \
             Smoozy does not** — scrollbars, form controls, system colours"
        );
        assert!(
            drawing.contains("color-scheme: light dark;"),
            "the bare root no longer declares `light dark`, which is what covers \
             the system's own dark and is why the media block needs none"
        );
    }

    /// **Nothing about the narrowing is anywhere but the one place.**
    ///
    /// The mockup's own banner: *"Everything about narrowing is here and nowhere
    /// else"*, written because three rules had once sat beside the components
    /// they governed and fought this set. The lift keeps that true, so the marks
    /// appear in the drawing and in the page module and in no other page file.
    #[test]
    fn the_narrowing_is_in_the_drawing_and_the_page_module_and_nowhere_else() {
        for entry in std::fs::read_dir(the_page_folder()).expect("the page folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if !path.is_file() {
                continue;
            }
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .expect("the file has a name")
                .to_string();
            if name == "the-drawing.css" || name == "panes.js" || name == "smoozy.css" {
                continue;
            }
            let text =
                without_comments(&std::fs::read_to_string(&path).expect("the file is readable"));
            assert!(
                !text.contains("data-narrow"),
                "web/{name} keys a rule to the narrowing mark. **Everything about \
                 narrowing is in `the-drawing.css` and nowhere else**, which is \
                 the mockup's own banner and the reason that block exists"
            );
        }
    }

    /// **The three commands are the whole of what this service exposes to the
    /// page**, and each has a closed request and response type — H34.
    #[test]
    fn the_shape_of_what_the_page_may_ask_for() {
        let layout = the_pane_layout_from(None);
        assert_eq!(layout.seams.len(), 3);
        // **A width nobody has set is absent rather than a number**, which is
        // what makes the drawn default the answer (`State, identity and
        // recovery` I3 rule 4).
        assert!(layout.seams.iter().all(|seam| seam.width.is_none()));
        // **Every string the page draws came by the prose route** — X1 and A6.
        for seam in &layout.seams {
            let _: &Prose = &seam.label;
            let _: &Prose = &seam.hint;
        }
        // **A record that could not be opened still answers.** The floor is the
        // window's own answer to being too small rather than a screen, so it
        // must not depend on the record having opened.
        assert!(!layout.floor.said.as_str().is_empty());
    }

    /// A record of its own for the tests below.
    fn a_record(name: &str) -> (std::path::PathBuf, crate::record::LocalRecord) {
        let mut folder = std::env::temp_dir();
        folder.push(format!("smoozy-panes-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder to put the record in");
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3"))
            .expect("the record opens");
        (folder, record)
    }

    /// **A drag is written down, and the next launch reads it back** — P11's
    /// *the widths are remembered between runs*, and `State, identity and
    /// recovery` I3 rules 2 and 3.
    ///
    /// This is the whole round trip the command pair makes, minus the shell's
    /// own managed state, which has no constructor outside a running
    /// application.
    #[test]
    fn a_width_a_drag_wrote_comes_back_in_the_next_layout() {
        let (folder, record) = a_record("round-trip");
        a_drag_ended_on(Some(record.connection()), "rail", 300);

        let layout = the_pane_layout_from(Some(record.connection()));
        let rail = layout
            .seams
            .iter()
            .find(|seam| seam.key == "rail")
            .expect("the rail is one of the seams");
        assert_eq!(rail.width, Some(300));
        assert!(layout
            .seams
            .iter()
            .filter(|seam| seam.key != "rail")
            .all(|seam| seam.width.is_none()));

        // The reset clears it, so the next layout draws the default again.
        the_seam_was_reset_on(Some(record.connection()), "rail");
        let layout = the_pane_layout_from(Some(record.connection()));
        assert!(layout.seams.iter().all(|seam| seam.width.is_none()));

        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **A pane cannot be dragged into uselessness, and the application is the
    /// one that says so** — H34: *"validates lengths and enum values before
    /// touching an adapter"*.
    ///
    /// The page clamps as it drags, so the pointer and the pane agree on
    /// screen. **This clamps again**, because a command is reachable without
    /// the page's own arithmetic and a width outside the two stops would be
    /// remembered for ever.
    #[test]
    fn a_width_outside_the_two_stops_is_brought_back_to_them() {
        let (folder, record) = a_record("clamping");
        let rail = THE_SEAMS
            .iter()
            .find(|seam| seam.key == "rail")
            .expect("the rail is declared");

        a_drag_ended_on(Some(record.connection()), "rail", 5);
        assert_eq!(
            crate::record::pane_widths::read(record.connection(), "rail").unwrap(),
            Some(rail.smallest)
        );

        a_drag_ended_on(Some(record.connection()), "rail", 5_000);
        assert_eq!(
            crate::record::pane_widths::read(record.connection(), "rail").unwrap(),
            Some(rail.largest)
        );

        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **A seam Design did not draw is written nowhere** — `State, identity and
    /// recovery` I3 rule 1's closed set, kept where the page reaches the store.
    ///
    /// **The set is kept in code rather than by a constraint in the table**,
    /// because a `CHECK` naming the three would be the same closed set living
    /// in two places, and a seam Design later draws would then be a migration
    /// rather than a line. This is the line that makes that safe.
    #[test]
    fn a_seam_the_design_never_drew_is_written_nowhere() {
        let (folder, record) = a_record("closed-set");
        a_drag_ended_on(Some(record.connection()), "the-one-nobody-drew", 300);
        the_seam_was_reset_on(Some(record.connection()), "the-one-nobody-drew");
        assert!(crate::record::pane_widths::read_all(record.connection())
            .unwrap()
            .is_empty());
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **A record that could not be opened writes nothing and says so** — L8a.
    ///
    /// Opening the local record creates it where it is not there, so no record
    /// is never a fresh installation: it is a record that failed to open
    /// (`Started::record`'s own note). **Before 2026-09-19 this answered as a
    /// fresh installation** and the user met the default widths in silence, and
    /// this test asserted exactly that; the Gemini catch-up of step 10a, Area 4
    /// (F-1).
    #[test]
    fn with_no_record_nothing_is_written_and_the_record_is_reported_as_not_answering() {
        a_drag_ended_on(None, "rail", 300);
        the_seam_was_reset_on(None, "rail");
        let layout = the_pane_layout_from(None);
        assert_eq!(layout.seams.len(), 3);
        assert!(layout.seams.iter().all(|seam| seam.width.is_none()));
        assert!(
            layout.the_record_would_not_answer,
            "no record at all is being reported as a fresh installation. \
             **Opening the record creates it**, so a caller holding none holds \
             one that would not open, and L8a says the screen shows that failure \
             rather than an emptied version of itself"
        );
    }

    /// **An opened record holding no widths is the fresh installation**, and it
    /// is told apart from one that would not answer — `State, identity and
    /// recovery` I3 rule 4's absent value.
    #[test]
    fn an_opened_record_holding_no_widths_is_the_ordinary_state() {
        let (folder, record) = a_record("fresh");
        let layout = the_pane_layout_from(Some(record.connection()));
        assert!(!layout.the_record_would_not_answer);
        assert!(layout.seams.iter().all(|seam| seam.width.is_none()));
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **A read that would not answer is not an absent setting** — P11's last
    /// bullet, citing L8a in as many words: *"an absent setting is an ordinary
    /// state, a read that did not answer is not."* F-6.
    ///
    /// **The fault this holds.** The two produced the same answer — an empty
    /// list — so every seam was drawn at the default with no word about it. The
    /// user who set those widths deliberately met the default layout, dragged a
    /// seam to put it back, and **that drag overwrote what was still in the
    /// record**, so the widths were then genuinely gone. **Keeping the two
    /// apart all the way to the page is what stops the second half.**
    #[test]
    fn a_read_that_would_not_answer_is_told_apart_from_a_record_that_holds_nothing() {
        let (folder, record) = a_record("unreadable");
        a_drag_ended_on(Some(record.connection()), "rail", 300);
        // The table goes, so the read fails rather than answering empty. This is
        // the one difference from the case above, and it has to reach the page
        // as a different answer.
        record
            .connection()
            .execute_batch(&format!("DROP TABLE {}", pane_widths::TABLE))
            .expect("the table can be dropped");

        let layout = the_pane_layout_from(Some(record.connection()));
        assert!(
            layout.the_record_would_not_answer,
            "a read that would not answer reaches the page as a fresh \
             installation. **That is L8a's own distinction collapsed**, and what \
             it costs is the user's widths: they meet the defaults, drag a seam \
             to put them back, and the drag writes over what was still there"
        );
        assert!(layout.seams.iter().all(|seam| seam.width.is_none()));
        // **The floor still answers.** It is the window's own answer to being
        // too small rather than a screen's, so it must not depend on the record.
        assert!(!layout.floor.said.as_str().is_empty());

        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **The page does not write a width where the record would not answer** —
    /// the second half of F-6, and the half that stops *unread* becoming
    /// *genuinely gone*.
    #[test]
    fn the_page_holds_its_write_back_where_the_record_would_not_answer() {
        let page = the_page_module();
        assert!(
            page.contains("the_record_would_not_answer"),
            "web/panes.js no longer asks which of the two answers it got. **L8a: \
             an absent setting is an ordinary state, a read that did not answer \
             is not** — and a page that cannot tell them apart draws the defaults \
             in silence and then writes over what it could not read"
        );
    }

    /// **The page tells the user, in the failure shape** — the rest of F-6's
    /// second half, finished at the close of round 1.
    ///
    /// The Verdict's words are *a read that would not answer is the failure
    /// shape's*. **A mark on the document is not a telling**: nothing in the
    /// page reads or draws `data-panes`, so a page that only set it left the
    /// user with the default layout and no word about it — the same silence the
    /// old `"failed"` mark produced. This reads the page as text, which is the
    /// only instrument there is for it.
    #[test]
    fn the_page_tells_the_user_where_the_record_would_not_answer() {
        let page = the_page_module();
        assert!(
            page.contains("theFailureShape"),
            "web/panes.js draws no failure shape. **A read that would not \
             answer is the failure shape's** (F-6), and a mark on the document \
             that nothing draws tells the user nothing"
        );
        assert!(
            page.contains("the_widths_could_not_be_read"),
            "web/panes.js does not draw the sentence the application hands it. \
             **Prose reaches a screen through the typography point and nowhere \
             else** (A6, X1), so a page that composes its own here is the fault \
             F-6 and F-7 were both raised over"
        );
    }

    /// **The telling sits in a line of its own, inset, and is not dropped bare
    /// into the window's two-row grid** — seen in a throwaway copy on
    /// 2026-09-19, where it landed under the panes with no margin and its dot
    /// cut by the window's edge (the Gemini catch-up of step 10a, Area 4, the
    /// runner's look after F-1).
    #[test]
    fn the_telling_where_the_widths_would_not_read_has_a_line_of_its_own() {
        let page = the_page_module();
        assert!(
            page.contains("held.className = \"panes-unread\";")
                && page.contains("app.appendChild(held);"),
            "web/panes.js puts the failure shape straight into the window's grid"
        );
        let sheet = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/panes.css"),
        )
        .expect("the stylesheet is readable");
        assert!(sheet.contains(".panes-unread {"), "the line has no rule giving it room");
    }

    /// **The sentence is on the answer whichever way the read went.**
    ///
    /// The page is handed one shape of answer, so the telling cannot go missing
    /// on the one path that needs it.
    #[test]
    fn the_telling_is_on_both_answers_and_is_prose() {
        let could_not = the_layout_that_could_not_be_read();
        assert!(could_not.the_record_would_not_answer);
        assert!(!could_not.the_widths_could_not_be_read.as_str().is_empty());
        // **The ordinary case is an opened, empty record** — not `None`, which
        // is a record that would not open (F-1 of the Gemini catch-up, Area 4).
        let (folder, record) = a_record("telling-ordinary");
        let ordinary = the_pane_layout_from(Some(record.connection()));
        assert!(!ordinary.the_record_would_not_answer);
        assert_eq!(
            ordinary.the_widths_could_not_be_read.as_str(),
            could_not.the_widths_could_not_be_read.as_str()
        );
        // **Through the point** (X1): the last pair of words is bound before it
        // ever reaches the page.
        assert!(
            could_not
                .the_widths_could_not_be_read
                .as_str()
                .contains('\u{a0}'),
            "the sentence did not go through the typography point"
        );
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **The window can be made small enough to meet the floor.**
    ///
    /// **This is the check that would have caught the floor being unreachable**,
    /// and it caught it: from step 1 until 2026-09-08 the window refused to go
    /// below 880 pixels wide, so the fourth narrowing step at 780 and the floor
    /// at 680 could never be reached by anybody, on any system. The panel that
    /// this piece exists to make appear would still never have appeared — for a
    /// different reason from the container query, with the same result.
    #[test]
    fn the_window_can_be_made_narrower_than_the_floor() {
        let window = include_str!("../main.rs");
        let (_, from_the_minimum) = window
            .split_once(".min_inner_size(")
            .expect("the window declares a smallest size");
        let named = &from_the_minimum[..from_the_minimum
            .find(',')
            .expect("the smallest size has a width and a height")];
        let smallest: f64 = named.trim().parse().expect("the width is a number");
        assert!(
            smallest < f64::from(THE_FLOOR),
            "the window will not go below {smallest} pixels wide and the floor is \
             at {THE_FLOOR}. **The floor states itself and names the width**, and \
             a floor the user cannot reach is a floor that never appears — which \
             is the same failure the container query caused, by a different route"
        );
    }
}

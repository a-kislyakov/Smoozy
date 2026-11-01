//! **A server's own screen — its head, its two tabs, and the two panels that
//! open over it.** The page half is `web/a-server.js`, and **the contract at
//! the head of that file is this piece's real deliverable**: pieces 6 and 7
//! hang off the tab strip and pieces 8 and 9 draw inside the panel.
//!
//! **This file is the surface and not the deciding.** The head's five readings
//! are `Servers`', already translated by `src/surface/servers.rs` at step 10a
//! piece 3, and this file takes the row that screen composes rather than
//! composing a second one. **Nothing here decides what a server is.**
//!
//! ## What this piece draws, cited the way the runner ruled on 2026-09-12
//!
//! **The Design document and the heading**, and **the mockup state names**.
//!
//! - `Design/Whole app § Panels over a server` — the whole panel mechanism.
//!   **Three of its five paragraphs are Creator-stated** and are kept as
//!   written: collapsing is a chevron pinned to the panel's top corner; the two
//!   panels are the same kind of thing on purpose; a panel belongs to its
//!   server and does not follow the user.
//! - `Design/Whole app § Where things live` — *"A server is a place you go
//!   into, and it has faces … its own screen carries tabs: Files and Set up."*
//! - `Design/Whole app § Keyboard shortcuts` — *"On a panel, Escape collapses
//!   rather than ends"*, and *"Escape closes the thing on top, one layer at a
//!   time"*.
//! - `Design/Whole app § The server badge` — the badge appears on the server's
//!   own screen, and it means *which server* and nothing else.
//! - `Design/File Browser § Where it sits` and `Design/Server Setup § Where it
//!   sits` — the two tabs, and that each fills the server's screen.
//! - `Design/Servers § A row` — the name, the address and the hardware words,
//!   which the head shows for one server and the list shows for all of them.
//!
//! **The eleven server-pane states belong to this piece:** `pane-connected`,
//! `pane-off`, `pane-connecting`, `pane-repair`, `pane-setup`, `pane-key`,
//! `pane-restart`, `pane-failed`, `pane-notool`, `pane-keyed`, `pane-railmenu`.
//! The connection control and menu are the All servers screen's actual shared
//! builders, with its complete key-checked reading; selecting remains navigation.
//!
//! ## What can actually happen today, said plainly
//!
//! - **The head** is composed from the local record and runs whole.
//! - **The tab strip** is drawn and **neither tab has content**, because Files
//!   is piece 6 and Set up is piece 7. A tab nothing has registered marks its
//!   host `data-tab="none"`, which is the mounting contract's own answer one
//!   level down.
//! - **The panel mechanism** — open, collapse, the strip that keeps counting,
//!   coming back, stacking, belonging to a server — is built whole and is
//!   **watchable through a stand-in**: a panel nothing has registered draws its
//!   frame with [`TheServersOwnScreen::the_stand_in`] in it, and the head
//!   carries one control per unregistered panel to open it. **Piece 8 removes
//!   the first control by registering, and piece 9 the second**, without either
//!   editing this piece.
//!
//! ## No mark on the rail row
//!
//! **Until 2026-10-04 a server's rail row carried a mark while something of
//! the user's was open on it** — the mockup's `<span class="parked">`, with
//! its sentence here. **The Creator's note 19 on the test copy struck it**
//! (`Design/Whole app § Panels over a server`): the rail's one dot now says
//! what the server is doing (`src/surface/the_rail.rs`), and an open panel is
//! said by its own strip along the bottom of the server's screen. The second
//! notes' piece 13 took the mark, its sentence and its four callers out.
//!
//! ## How a clause marker is written in this crate
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. **Every marker of
//! another document is written out with that document at the site.**

use std::sync::Mutex;

use tauri::State;

use smoozy_servers::store::ServerRow;

use crate::record::server_rows;
use crate::start::Started;
use crate::surface::servers::{AServerRow, TheHardware, TheRentalLine, Words};
use crate::typography::{Prose, Verbatim};

// ════════════════════════════════════════════════════════════════════════════
// The four names, and they are the drawing's own
// ════════════════════════════════════════════════════════════════════════════

/// **The Files tab.** The identifier is the one the mockup gives that tab's own
/// body element, so the drawing's `.files` rules apply to the host this piece
/// makes without a line of translation. Piece 2 settled the same thing one
/// level up for the seven places.
pub const FILES: &str = "files";

/// **The Set up tab**, likewise — the mockup's own `setupview`.
pub const SET_UP: &str = "setupview";

/// **The render settings panel.** The key is the mockup's own `data-open` and
/// `data-collapse` value for that panel.
pub const RENDER_SETTINGS: &str = "set";

/// **The dependency check panel**, likewise.
pub const THE_DEPENDENCY_CHECK: &str = "chk";

/// **The two tabs, in Design's order.** *"its own screen carries tabs: Files
/// and Set up"*.
pub const THE_TWO_TABS: [&str; 2] = [FILES, SET_UP];

/// **The two panels, in the order the drawing stacks their strips** — the
/// dependency check above the render settings. See
/// [`TheServersOwnScreen::the_panels`] for why the order is fixed rather than
/// by when each was collapsed.
pub const THE_TWO_PANELS: [&str; 2] = [THE_DEPENDENCY_CHECK, RENDER_SETTINGS];

// ════════════════════════════════════════════════════════════════════════════
// Every word the user reads
// ════════════════════════════════════════════════════════════════════════════

mod words {
    // `Design/Whole app § Where things live` — the two tabs, named there.
    pub const FILES: &str = "Files";
    pub const SET_UP: &str = "Set up";

    // The two panels, named as `Design/Whole app § Panels over a server` names
    // them: *"a job's render settings, and a `.blend`'s dependency check"*.
    // **The name is what the strip carries where the panel has not said
    // otherwise**; the drawing's own strip names the file beside it, and the
    // file is the panel's to supply.
    pub const RENDER_SETTINGS: &str = "Render settings";
    pub const THE_DEPENDENCY_CHECK: &str = "Dependencies";

    // The mockup's own two words, at the pinned control and at the strip.
    pub const COLLAPSE: &str = "Collapse";
    pub const OPEN: &str = "Open";

    // **A stand-in, and pieces 8 and 9 remove it.** It is the same kind of
    // thing as the shell's `data-screen="none"`: a state of this build stage
    // and not of the application.
    pub const NOT_BUILT_YET: &str = "Not available yet.";

    // **L8a: the screen shows the failure rather than an emptied version of
    // itself.** The words are Design's and are owed (§24); what is fixed here
    // is that each of these says something instead of drawing a server that is
    // not there.
    pub const NO_SUCH_SERVER: &str = "Server removed.";
    pub const WOULD_NOT_READ: &str = "Could not read this server.";
}

/// **This screen's own sentence for a server that is gone**, which the server
/// list says too where another act removed a server under the user (step 10a
/// round 1, fix-here items 12 and 32). One home, taken from here.
pub(crate) const THIS_SERVER_HAS_BEEN_REMOVED: &str = words::NO_SUCH_SERVER;

// ════════════════════════════════════════════════════════════════════════════
// What the page is handed
// ════════════════════════════════════════════════════════════════════════════

/// **The server's own head** — `Design/Whole app § Where things live` and the
/// mockup at 6353–6369: the badge, the name, the address, the hardware line,
/// and what it is rendering.
///
/// **Every field is taken from the row the All servers screen composes for the
/// same server**, so the head and the list can never disagree about a server.
/// See [`the_head_from`].
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheHead {
    /// The server's own identity, which every act hands back. A value.
    pub server: Verbatim,
    /// **The name, or `username@host` where the server has none.** A value:
    /// the user can copy it and X7 forbids the point touching it.
    pub name: Verbatim,
    /// **The address, in the fixed-width face.** A value.
    pub address: Verbatim,
    /// One of the drawing's eight badges, or nothing where the stored colour is
    /// not one of them.
    pub badge: Option<&'static str>,
    /// **What it has to render with** — the cards, the memory and the free
    /// disk, with `Servers` H1's three states written.
    pub the_hardware: TheHardware,
    /// **What it is doing**, which is the fourth thing on the mockup's hardware
    /// line. The dash where Smoozy cannot say.
    pub the_work: Vec<Words>,
    /// **`Servers` §11.8's rental line, on the head's hardware line** — the
    /// running-time figure where one stands, and `Servers` W40's statement in
    /// its place otherwise; **never neither** (`Servers` W40a). Nothing after
    /// the add said the machine was still running and billing until this
    /// (step 10a round 1, fix-here item 16).
    pub the_rental_line: TheRentalLine,
}

/// One tab in the strip.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ATab {
    /// The drawing's own identifier for that tab's body. A value the page keys
    /// on and puts on the host element.
    pub tab: Verbatim,
    /// What the tab is called. Prose.
    pub name: Prose,
}

/// One panel over the server.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct APanel {
    /// The drawing's own key for that panel. A value.
    pub panel: Verbatim,
    /// What the panel is called, which is what its strip says where the panel
    /// itself has said nothing. Prose.
    pub name: Prose,
}

/// **The whole screen, in one answer.**
///
/// **A command rather than an event, and P6 is not being bent.** P6 is about a
/// module's *results*; this is the page asking for the facts it needs before it
/// can draw anything, which is the shape `the_rail`, `chrome_text` and
/// `the_all_servers_screen` already have. **A page that comes back asks again**
/// (P8).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheServersOwnScreen {
    /// The head, where the record answered and the server is in it.
    pub the_head: Option<TheHead>,
    /// **What is said in the head's place where there is no head** — L8a, and
    /// it is the opposite of drawing an empty screen. Two causes reach it: the
    /// server has been removed, and the record would not read.
    pub instead: Option<Words>,
    /// **Whether the server is gone, as against a read that did not answer.**
    /// Only a server that is gone takes its panels with it; a read that merely
    /// failed keeps them (step 10a round 1, fix-here item 24).
    pub the_server_is_gone: bool,
    /// **The two tabs, always, in Design's order.** They are Design's fixed
    /// pair and not a function of what has been built, which is the mounting
    /// contract's section 6 read one level down.
    pub the_tabs: Vec<ATab>,
    /// **The two panels, always, in the order their strips stack.**
    ///
    /// **The order is the drawing's and it is fixed rather than by when each
    /// was collapsed.** A strip that moved under the user's finger because they
    /// collapsed the other one is a target that shifts while they reach for it.
    pub the_panels: Vec<APanel>,
    /// The pinned collapse control's name, for anyone who cannot see it.
    pub collapse: Prose,
    /// The word at the right-hand end of a strip.
    pub open: Prose,
    /// **What a panel nothing has registered says. Pieces 8 and 9 remove it.**
    pub the_stand_in: Prose,
}

// ════════════════════════════════════════════════════════════════════════════
// The command
// ════════════════════════════════════════════════════════════════════════════

/// **The server's own screen, in one answer.**
///
/// **H34: the one value in is validated before anything is touched.** An
/// identity that matches no row answers [`words::NO_SUCH_SERVER`] rather than
/// nothing at all.
#[tauri::command]
pub fn the_servers_own_screen<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    started: State<'_, Mutex<Started>>,
    server: String,
) -> TheServersOwnScreen {
    // **The head reads the same connection state the list does**, which is the
    // run's own register — step 10b piece 5. A second reading here would be the
    // fault this file's own head already names: one server reading two ways.
    //
    // **The application's lock is held for the record's read alone, and the
    // run's periods are read after it is given back** — step 10b's first audit
    // round, item 2. Composed under it, this command held the application's
    // lock and then took the publisher's, while a period publishing held the
    // publisher's and then took the application's; the two could meet and
    // freeze the window for good.
    //
    // **The work column is composed under the record's lock with the rows**,
    // because it needs the job records beside the held reading — the order
    // the Queue screen and the list take, and the reading's own lock takes no
    // other. Step 10c's ninth-pass fix piece 2.
    let periods = crate::surface::servers::the_periods_of(&app);
    let (read, work) = match started.lock() {
        Ok(started) => (
            the_record_read(&started),
            periods
                .as_deref()
                .map(|periods| {
                    crate::surface::the_queue::what_the_servers_are_doing(
                        periods.the_machines_work(),
                        started.record.as_ref(),
                    )
                })
                .unwrap_or_default(),
        ),
        Err(_) => (WhatTheRecordSaid::WouldNotRead, Default::default()),
    };
    the_screen(read, &server, periods.as_deref(), &work)
}

// ════════════════════════════════════════════════════════════════════════════
// Reading what there is
// ════════════════════════════════════════════════════════════════════════════

/// **A read of the server rows answers three ways and never two** — L8a.
///
/// **The rows and the empty record are not the same answer as a read that did
/// not happen**, which is the whole of the fault this application met on
/// 2026-09-13: a failed read answered *nothing is there* and no screen said
/// otherwise.
pub enum WhatTheRecordSaid {
    /// The rows.
    Rows(Vec<ServerRow>),
    /// The read did not answer.
    WouldNotRead,
}

/// **The head asks nothing of `Keys`, and runs no tool** (step 10a round 1,
/// fix-here item 20). The tools probe and the key verdicts decide the key mark
/// and the one tools statement, and **neither is on a server's head** — so a
/// probe taken here, on every arrival and under the record's lock, reached
/// nothing the head draws.
fn the_record_read(started: &Started) -> WhatTheRecordSaid {
    let Some(record) = started.record.as_ref() else {
        return WhatTheRecordSaid::WouldNotRead;
    };
    let Ok(rows) = server_rows::all(record.connection()) else {
        return WhatTheRecordSaid::WouldNotRead;
    };
    WhatTheRecordSaid::Rows(rows)
}

// ════════════════════════════════════════════════════════════════════════════
// Composing the screen
// ════════════════════════════════════════════════════════════════════════════

/// The screen, with the read handed in — **so the whole of it can be tested**.
pub fn the_screen(
    said: WhatTheRecordSaid,
    server: &str,
    periods: Option<&crate::servers::a_period::ThePeriodsThisRun>,
    work: &crate::surface::the_queue::WhatTheServersAreDoing,
) -> TheServersOwnScreen {
    let (the_head, instead, the_server_is_gone) = match said {
        WhatTheRecordSaid::WouldNotRead => (None, Some(plainly(words::WOULD_NOT_READ)), false),
        WhatTheRecordSaid::Rows(rows) => match the_head_from(&rows, server, periods, work) {
            Some(head) => (Some(head), None, false),
            None => (None, Some(plainly(words::NO_SUCH_SERVER)), true),
        },
    };

    TheServersOwnScreen {
        the_head,
        instead,
        the_server_is_gone,
        the_tabs: vec![
            ATab {
                tab: Verbatim::new(FILES),
                name: Prose::new(words::FILES),
            },
            ATab {
                tab: Verbatim::new(SET_UP),
                name: Prose::new(words::SET_UP),
            },
        ],
        the_panels: vec![
            APanel {
                panel: Verbatim::new(THE_DEPENDENCY_CHECK),
                name: Prose::new(words::THE_DEPENDENCY_CHECK),
            },
            APanel {
                panel: Verbatim::new(RENDER_SETTINGS),
                name: Prose::new(words::RENDER_SETTINGS),
            },
        ],
        collapse: Prose::new(words::COLLAPSE),
        open: Prose::new(words::OPEN),
        the_stand_in: Prose::new(words::NOT_BUILT_YET),
    }
}

/// **The head, taken from the row the All servers screen composes.**
///
/// **This piece composes no reading of its own and that is the point.** The
/// badge, the name, the address, the hardware and the work column are all
/// `Servers`' judgements, translated once at `src/surface/servers.rs` and taken
/// here — so a server reads the same in the rail, in the list and on its own
/// head, and no third copy of any of them exists to drift.
pub fn the_head_from(
    rows: &[ServerRow],
    server: &str,
    periods: Option<&crate::servers::a_period::ThePeriodsThisRun>,
    work: &crate::surface::the_queue::WhatTheServersAreDoing,
) -> Option<TheHead> {
    let screen = crate::surface::servers::the_screen_from(rows, &[], None, periods, work);
    let row = screen
        .rows
        .into_iter()
        .find(|row| row.server.as_str() == server)?;
    Some(the_head_of(row))
}

/// The five things the mockup's head draws, off the row.
fn the_head_of(row: AServerRow) -> TheHead {
    TheHead {
        server: row.server,
        name: row.name,
        address: row.address,
        badge: row.badge,
        the_hardware: row.the_hardware,
        the_work: row.the_work,
        // **No running-time figure is supplied by anything today** — the
        // figure is taken on a connection (`Servers` W41), and no connection
        // this application makes takes it — so `Servers` W40's statement
        // stands in its place.
        the_rental_line: crate::surface::servers::the_rental_line(None),
    }
}

/// Words with no value in them.
fn plainly(said: &str) -> Words {
    Words {
        lead: Prose::new(said),
        values: Vec::new(),
        rest: None,
        the_rest_joins: false,
    }
}

#[cfg(test)]
mod tests {

    /// **Step 10b's first audit round, item 19: the server's own page follows
    /// its connection.** It listens for the observed value and for the
    /// condition, under the kinds `src/servers/published.rs` declares, and asks
    /// for the page again for the server that is open.
    #[test]
    fn the_servers_own_page_is_asked_for_again_when_its_connection_moves() {
        let page = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/a-server.js"),
        )
        .expect("the page module is readable");
        for kind in [
            crate::servers::published::WHAT_IS_OBSERVED,
            crate::servers::published::THE_CONNECTION_CONDITION,
        ] {
            assert!(page.contains(&format!("\"{kind}\"")), "the page does not name {kind}");
        }
        for listened in ["whenTold(WHAT_IS_OBSERVED", "whenTold(THE_CONNECTION_CONDITION"] {
            let at = page.find(listened).unwrap_or_else(|| panic!("{listened} is not heard"));
            let heard = &page[at..at + page[at..].find("});").expect("it closes")];
            assert!(heard.contains("delivery.subject !== current"));
            assert!(heard.contains("theScreenIsAskedFor(current)"));
        }
    }
    use super::*;
    use smoozy_servers::identity::{Host, KeyReference, Port, Username};
    use smoozy_state::identities::{Identity, IdentityKind};
    use std::fs;
    use std::path::Path;

    /// This file's working half — everything before the tests. **The sweeps
    /// below read it rather than the whole file**, because the things they
    /// forbid are written out here in order to forbid them.
    fn the_working_half() -> &'static str {
        include_str!("a_server.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half")
    }

    fn the_page() -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("web/a-server.js");
        fs::read_to_string(path).expect("the page module is readable")
    }

    fn the_rail_page() -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("web/the-rail.js");
        fs::read_to_string(path).expect("the shell's page module is readable")
    }

    fn the_mockup() -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../Preparation/Design/Mockup/Smoozy.html");
        fs::read_to_string(path).expect("the drawing is readable")
    }

    fn a_row(label: Option<&str>) -> ServerRow {
        let mut row = ServerRow::saved(
            Identity::issue(IdentityKind::LocalServer)
                .expect("randomness")
                .as_text(),
            Host::what_the_user_typed("213.181.122.44").expect("a host"),
            Port::what_the_user_typed(40122).expect("a port"),
            Username::what_the_user_typed("root").expect("a login"),
            KeyReference {
                local_key_identifier: "key-1".to_string(),
                fingerprint: "SHA256:aaaa".to_string(),
            },
        );
        row.label = label.map(str::to_string);
        row.colour = Some("b1".to_string());
        row
    }

    /// **The two tabs and the two panels are the drawing's own four, read out
    /// of the drawing.**
    ///
    /// **This is the section-1 test one level down.** Piece 2 reads the
    /// mockup's seven view identifiers so a screen cannot invent a destination;
    /// this reads the mockup's two tab bodies, its two collapse keys and its
    /// two strip keys, so **pieces 6, 7, 8 and 9 cannot invent a fifth thing to
    /// hang off, and cannot lose one.**
    #[test]
    fn the_four_names_are_the_drawings_own_four() {
        let drawing = the_mockup();

        // The two tab bodies, by the identifiers the drawing gives them.
        for tab in THE_TWO_TABS {
            assert!(
                drawing.contains(&format!("id=\"{tab}\"")),
                "the drawing has no element identified {tab}, so the host this \
                 piece makes would carry an identifier the drawing's own rules \
                 do not reach"
            );
        }

        // The two panels, by the keys the drawing collapses and opens them
        // with. **Both directions**: every key the drawing has is one of ours,
        // and every one of ours is the drawing's.
        let mut collapses: Vec<&str> = Vec::new();
        for (at, _) in drawing.match_indices("data-collapse=\"") {
            let rest = &drawing[at + "data-collapse=\"".len()..];
            collapses.push(&rest[..rest.find('"').expect("the value closes")]);
        }
        collapses.sort_unstable();
        collapses.dedup();
        let mut ours: Vec<&str> = THE_TWO_PANELS.to_vec();
        ours.sort_unstable();
        assert_eq!(collapses, ours, "the drawing's collapse keys have moved");

        let mut opens: Vec<&str> = Vec::new();
        for (at, _) in drawing.match_indices("data-open=\"") {
            let rest = &drawing[at + "data-open=\"".len()..];
            let value = &rest[..rest.find('"').expect("the value closes")];
            // The drawing uses the same attribute name for the queue's own
            // open-and-shut groups, which are `true` and `false`.
            if value == "true" || value == "false" {
                continue;
            }
            opens.push(value);
        }
        opens.sort_unstable();
        opens.dedup();
        assert_eq!(opens, ours, "the drawing's strip keys have moved");

        // The tab strip is two tabs and the drawing has no third.
        let tabs = drawing.matches("class=\"tab\" role=\"tab\"").count();
        assert_eq!(tabs, THE_TWO_TABS.len(), "the drawing's tab strip has moved");
    }

    /// **The page names the same four keys.** Nothing holds the two sides
    /// together at compile time, so this is the check written out — the same
    /// one piece A's account said every pair of files here would need.
    #[test]
    fn the_page_names_the_same_four_keys() {
        let page = the_page();
        for name in THE_TWO_TABS.iter().chain(THE_TWO_PANELS.iter()) {
            assert!(
                page.contains(&format!("\"{name}\"")),
                "web/a-server.js does not name {name}, so the two sides have \
                 drifted and nothing else would say so"
            );
        }
        assert!(
            page.contains("the_servers_own_screen"),
            "web/a-server.js does not call this file's one command"
        );
    }

    /// **The head is the row the All servers screen composes, field for
    /// field.**
    ///
    /// **A second composition is the fault this stage keeps meeting** — a fact
    /// kept in two places, one of them updated. This is the line that fails if
    /// anybody writes one.
    #[test]
    fn the_head_is_the_row_the_all_servers_screen_composes() {
        let rows = vec![a_row(Some("RunPod 4090")), a_row(None)];
        let list = crate::surface::servers::the_screen_from(&rows, &[], None, None, &Default::default());
        for row in &list.rows {
            let head = the_head_from(&rows, row.server.as_str(), None, &Default::default())
                .expect("the server is in the record");
            assert_eq!(head.name, row.name);
            assert_eq!(head.address, row.address);
            assert_eq!(head.badge, row.badge);
            assert_eq!(head.the_hardware, row.the_hardware);
            assert_eq!(head.the_work, row.the_work);
        }
    }

    /// **A server that is not in the record is said rather than drawn empty**
    /// — L8a. The panels of a server that has been removed go with it, which
    /// is the page's half of the same rule.
    #[test]
    fn a_server_that_is_gone_is_said_rather_than_left_silent() {
        let screen = the_screen(
            WhatTheRecordSaid::Rows(vec![a_row(None)]),
            "no-such-identity",
            None,
            &Default::default(),
        );
        assert!(screen.the_head.is_none());
        assert_eq!(
            screen.instead.expect("something is said").lead,
            Prose::new(words::NO_SUCH_SERVER)
        );
    }

    /// **A read that did not answer says so, and never says the server is
    /// gone** — L8a, and the two causes are told apart.
    #[test]
    fn a_read_that_did_not_answer_says_so_and_not_that_the_server_is_gone() {
        let screen = the_screen(WhatTheRecordSaid::WouldNotRead, "any-identity", None, &Default::default());
        assert!(screen.the_head.is_none());
        assert_eq!(
            screen.instead.expect("something is said").lead,
            Prose::new(words::WOULD_NOT_READ)
        );
    }

    /// **Step 10a round 1, fix-here item 24: only a server that is gone takes
    /// its panels with it.** The answer tells the two causes apart, and the page
    /// forgets a server's panels only on the first.
    #[test]
    fn only_a_server_that_is_gone_takes_its_panels() {
        let gone = the_screen(WhatTheRecordSaid::Rows(vec![a_row(None)]), "no-such-identity", None, &Default::default());
        assert!(gone.the_server_is_gone);
        let unread = the_screen(WhatTheRecordSaid::WouldNotRead, "any-identity", None, &Default::default());
        assert!(!unread.the_server_is_gone, "a read that failed reads as a removal");
        let page = the_page();
        assert!(page.contains("if (answer.the_server_is_gone) theServerIsForgotten(server);"));
    }

    /// **Step 10a round 1, fix-here items 15 and 16: the head carries the rental
    /// line, never empty of both, and draws a remembered reading as stale with
    /// its *last seen* mark.**
    #[test]
    fn the_head_carries_the_rental_line_and_a_stale_reading() {
        let row = a_row(Some("RunPod 4090"));
        let head = the_head_from(std::slice::from_ref(&row), &row.local_server_identifier, None, &Default::default())
            .expect("the server is there");
        assert!(
            head.the_rental_line.running_for.is_some() || head.the_rental_line.the_statement.is_some(),
            "the head says nothing about the machine running and billing"
        );
        let page = the_page();
        let at = page.find("function theHardwareIsDrawn(drawn)").expect("the hardware line");
        let body = &page[at..at + page[at..].find("\n}\n").expect("it closes")];
        for kept in ["\"stale\"", "held.last_seen", "theMomentReads", "the_rental_line"] {
            assert!(body.contains(kept), "the head's hardware line does not use {kept}");
        }
    }

    /// **Step 10a round 1, fix-here item 26: Escape is this screen's only while
    /// it is the current place.**
    #[test]
    fn escape_is_this_screens_only_while_it_is_current() {
        let page = the_page();
        let at = page.find("  itIsPutAway() {").expect("the screen is put away");
        let body = &page[at..at + page[at..].find("\n  },\n").expect("it closes")];
        assert!(body.contains("current = null;"), "the open server outlives the screen");
        assert!(page.contains("if (!current) return;"));
    }

    /// **The runner's two lines of 2026-09-17**: a strip's values are set apart
    /// with the drawing's separator, and whether a panel covers a server is
    /// exported for a tab to read.
    #[test]
    fn the_strip_separates_its_values_and_an_open_panel_can_be_asked_about() {
        let page = the_page();
        assert!(page.contains("const THE_STRIPS_SEPARATOR = \" · \";"));
        assert!(page.contains("saidInto(counting, reading.reading, THE_STRIPS_SEPARATOR);"));
        assert!(page.contains("export function aPanelIsOpenOver(server)"));
    }

    /// **The tabs and the panels are answered whatever the read said**, because
    /// they are Design's fixed pairs and not a function of what is in the
    /// record.
    #[test]
    fn the_two_tabs_and_the_two_panels_are_answered_even_where_nothing_read() {
        let screen = the_screen(WhatTheRecordSaid::WouldNotRead, "any-identity", None, &Default::default());
        assert_eq!(screen.the_tabs.len(), 2);
        assert_eq!(screen.the_panels.len(), 2);
        // The strips stack in the drawing's own order and never in the order
        // the user collapsed them.
        assert_eq!(
            screen
                .the_panels
                .iter()
                .map(|one| one.panel.as_str())
                .collect::<Vec<_>>(),
            THE_TWO_PANELS.to_vec()
        );
    }

    /// **Every word the user reads is in `mod words` and nowhere else.**
    ///
    /// A string written straight into a composition is a word with no home, and
    /// the next piece that needs the same word writes a second copy of it.
    #[test]
    fn every_word_is_in_one_place() {
        let source = the_working_half();
        let after = source
            .split("mod words {")
            .nth(1)
            .expect("the words are a module");
        let after = after.split("\n}\n").nth(1).expect("the words module closes");
        let mut faults = Vec::new();
        for (at, _) in after.match_indices("Prose::new(\"") {
            let rest = &after[at + "Prose::new(\"".len()..];
            faults.push(rest[..rest.find('"').unwrap_or(0)].to_string());
        }
        assert!(
            faults.is_empty(),
            "a word is written into a composition rather than into `mod \
             words`: {faults:#?}"
        );
    }

    /// **This piece composes no failure sentence, so `WhoseSentenceItIs` is not
    /// reached** — said here rather than left unmentioned.
    ///
    /// The head raises no failure record and presents none: a failure standing
    /// against a server is drawn on that server's **row**, by the one failure
    /// shape, on the All servers screen. **The two statements this file does
    /// compose are L8a's** — a server that is gone and a read that did not
    /// answer — and neither is a failure record, so neither spends a diary
    /// capture. M32: *"a path that composes a stand-in sentence captures
    /// nothing."*
    #[test]
    fn nothing_here_reaches_the_diary_capture() {
        assert!(
            !the_working_half().contains("WhoseSentenceItIs"),
            "this piece composes a failure sentence. **M32 decides whether the \
             diary keeps what the user actually read**, and a screen that gets \
             it wrong produces no error, no failing test and no visible fault"
        );
    }

    /// **The panel is positioned against the server's screen and never against
    /// the window.**
    ///
    /// **`.app` does not always have two rows.** When Smoozy is locked, piece 1
    /// inserts the lock band as a third grid row and adds `axlocked`, so a
    /// panel measured against the window would sit under the band on a locked
    /// copy and over it otherwise. **The drawing's own answer is that the panel
    /// is absolute inside the server's own view**, which is `position:
    /// relative`, so this piece never reaches for the window at all — and this
    /// is the line that fails if anybody starts to.
    #[test]
    fn the_page_never_measures_a_panel_against_the_window() {
        let page = the_page();
        // **The selector as it would actually be written**, rather than the
        // three letters, which every `appendChild` in the file carries.
        for reached in [
            "\".app\"",
            "'.app'",
            "axlocked",
            "titlebar",
            "getBoundingClientRect",
            "window.inner",
            "documentElement",
        ] {
            assert!(
                !page.contains(reached),
                "web/a-server.js reaches for {reached}. **A panel covers the \
                 server's own screen and is placed inside it**, so where the \
                 top of the window is never enters into it — which is what \
                 makes it right on a locked copy without anybody thinking \
                 about it"
            );
        }
    }

    /// **The drawing no longer marks an open panel on the rail.**
    ///
    /// This test read the mark's sentence out of the drawing so the two could
    /// not disagree. **On 2026-10-04 the drawing took the mark away**, on the
    /// Creator's note 19 on the test copy: the rail's one dot now says what the
    /// server is doing (`.rdot`), and an open panel is said by its own strip.
    /// So the drawing has no sentence left to read, and what this holds now is
    /// that the mark does not come back unnoticed. **The app's own mark went
    /// with it in the second notes' piece 13**, held by the test below.
    #[test]
    fn the_drawing_no_longer_marks_an_open_panel_on_the_rail() {
        let drawing = the_mockup();
        assert!(
            !drawing.contains("class=\"parked\""),
            "the drawing marks an open panel on a rail row again"
        );
        assert!(
            drawing.contains("class=\"rdot run\"") && drawing.contains("class=\"rdot\""),
            "the drawing's rail rows no longer carry the dot that says what the server is doing"
        );
    }

    /// **Note 19: a panel open over a server draws nothing in the rail** —
    /// `Design/Whole app § Panels over a server`, the Creator's word of
    /// 2026-10-04 striking theirs of 2026-08-10. The panel's own strip says it
    /// is open; the rail's dot says only what the server is doing. **This
    /// screen tells the rail nothing, the rail has no way to be told, and no
    /// sentence for it is composed.**
    #[test]
    fn a_panel_open_over_a_server_draws_nothing_in_the_rail() {
        let rail = the_rail_page();
        for gone in ["theServerHasSomethingOpen", "parked", "somethingOpen"] {
            assert!(!rail.contains(gone), "web/the-rail.js still carries `{gone}`");
        }
        assert!(
            rail.contains("row.dataset.server = server.server"),
            "a server row no longer carries the server's own identity. **A row \
             found by its name is the wrong row the day two servers share one**"
        );
        let page = the_page();
        for gone in ["theServerHasSomethingOpen", "theRailIsToldWhatIsOpen", "something_is_open"] {
            assert!(!page.contains(gone), "web/a-server.js still tells the rail what is open: `{gone}`");
        }
        let source = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/surface/a_server.rs"),
        )
        .expect("this file is readable");
        let working = &source[..source.find("#[cfg(test)]").expect("the tests")];
        assert!(!working.contains("something_is_open"), "the screen still composes the mark's sentence");
    }

    /// **A removed server's screen shows no tab under its sentence** — the build
    /// runner's ruling 3 of 2026-09-13.
    ///
    /// The two tab hosts are one per run, so hiding the head alone left the
    /// showing tab's whole content standing under *This server has been
    /// removed.* This fails if the branch that says so stops putting the tab
    /// away, or if the helper stops hiding both hosts or stops telling the tab.
    #[test]
    fn a_removed_servers_screen_shows_no_tab() {
        let page = the_page();
        let at = page
            .find("if (!answer.the_head) {")
            .expect("the screen has a branch for a server that is not there");
        let branch = &page[at..];
        let branch = &branch[..branch.find("\n  }\n").expect("the branch closes")];
        let put_away = branch
            .find("theTabIsGoneWithItsServer();")
            .expect("the removed-server branch no longer takes the tab away");
        let forgotten = branch
            .find("theServerIsForgotten(server);")
            .expect("the removed-server branch no longer forgets the server");
        assert!(
            put_away < forgotten,
            "the tab is put away after the server is forgotten, so which tab was \
             showing can no longer be read and its own menu or box is left up"
        );

        let at = page
            .find("function theTabIsGoneWithItsServer(")
            .expect("the helper is gone");
        let helper = &page[at..];
        let helper = &helper[..helper.find("\n}\n").expect("the function closes")];
        for kept in [
            "itIsPutAway()",
            "[FILES, SET_UP]",
            "body.hidden = true",
            // Another server's panel and strips, which are hidden and never
            // taken down (section 6).
            "parts.wrap.hidden = true",
            "held.strips.hidden = true",
        ] {
            assert!(
                helper.contains(kept),
                "taking the tab away no longer does `{kept}`, so a tab's content \
                 can stand under the sentence that its server has been removed"
            );
        }
    }

    /// **Nothing in this piece rebuilds a shape the shared half already
    /// exports.** The mockup draws the failure shape eleven times in five
    /// modules with five class prefixes, and a piece that copies a subtree
    /// copies one of them.
    #[test]
    fn no_shared_shape_is_rebuilt_here() {
        let page = the_page();
        for prefix in ["qfail", "svfail", "tffail", "rdfail", "cbad"] {
            assert!(
                !page.contains(prefix),
                "web/a-server.js writes {prefix}, which is one of the mockup's \
                 five copies of the one failure shape. **Use the shape that \
                 exists** — `theFailureShape` in web/the-failure-shape.js"
            );
        }
    }

    /// **The Creator's fifth notes, note 3: while one panel is open, the
    /// other waits as its strip at the very bottom, under the open one** —
    /// `Design/Whole app § Panels over a server`, added 2026-10-08, and the
    /// drawing's state `res-chkstrip`. Piece V.
    ///
    /// `web/a-server.js` runs as it is, in a stand-in document. Render
    /// Settings is opened, then the check, then Render Settings again: the
    /// check's strip is drawn inside the open panel's frame, after its sheet,
    /// so the sheet ends above it — it can be seen and Tab reaches it, and a
    /// reading the check sends while it is focused leaves it focused. Pressing
    /// it brings the check up and leaves Render Settings as the strip. With
    /// one panel ever opened the other's strip is there all the same (the
    /// seventh notes, note 2); with both collapsed the two strips stack as
    /// before, the check above. **It fails if section 5's
    /// *only where no panel is open* is put back.**
    #[test]
    fn the_other_panel_waits_as_its_strip_under_the_open_one() {
        let rows = vec![a_row(Some("RunPod 4090"))];
        let list = crate::surface::servers::the_screen_from(&rows, &[], None, None, &Default::default());
        let server = list.rows[0].server.as_str().to_string();
        let answer = the_screen(WhatTheRecordSaid::Rows(rows), &server, None, &Default::default());
        assert!(answer.the_head.is_some(), "the server is in the record");
        let answers = serde_json::json!({ "server": server, "screen": answer });

        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        let scratch = std::env::temp_dir().join(format!("smoozy-the-other-panels-strip-{}", std::process::id()));
        let _ = fs::remove_dir_all(&scratch);
        fs::create_dir_all(&scratch).expect("a scratch folder");
        fs::copy(web.join("a-server.js"), scratch.join("a-server.js")).expect("the page is copied");
        for (name, text) in THE_STRIPS_STAND_INS {
            fs::write(scratch.join(name), text).expect("a stand-in is written");
        }
        fs::write(scratch.join("package.json"), r#"{ "type": "module" }"#).expect("the package note");
        fs::write(scratch.join("answers.json"), answers.to_string()).expect("the answers are written");
        fs::write(scratch.join("harness.mjs"), format!("{THE_STRIPS_DOCUMENT}{THE_STRIPS_WALK}"))
            .expect("the harness is written");
        let ran = std::process::Command::new("node")
            .arg(scratch.join("harness.mjs"))
            .current_dir(&scratch)
            .output();
        let _ = fs::remove_dir_all(&scratch);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the strips were not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let wanted = serde_json::json!({
            // Render Settings alone: the check was never opened, and its strip
            // is there all the same (the Creator's seventh notes, note 2).
            "oneEverOpened": {
                "open": "set",
                "strips": [{ "panel": "chk", "under": "set", "afterTheSheet": true, "tabReaches": true }]
            },
            // The check opened: Render Settings waits under it.
            "theCheckOpened": {
                "open": "chk",
                "strips": [{ "panel": "set", "under": "chk", "afterTheSheet": true, "tabReaches": true }]
            },
            // Render Settings again: the check waits under it.
            "theSettingsAgain": {
                "open": "set",
                "strips": [{ "panel": "chk", "under": "set", "afterTheSheet": true, "tabReaches": true }]
            },
            // A reading the check sends keeps the focused strip focused.
            "aReadingWhileFocused": { "focused": "chk", "says": "Uploading" },
            // Pressing the strip swaps the two.
            "theStripPressed": {
                "open": "chk",
                "strips": [{ "panel": "set", "under": "chk", "afterTheSheet": true, "tabReaches": true }],
                "focused": "collapse"
            },
            // Escape: both collapsed, the two strips as before, the check above.
            "bothCollapsed": {
                "open": null,
                "strips": [
                    { "panel": "chk", "under": null, "afterTheSheet": false, "tabReaches": true },
                    { "panel": "set", "under": null, "afterTheSheet": false, "tabReaches": true }
                ],
                "focused": "chk"
            }
        });
        assert_eq!(got, wanted, "the other panel's strip is not drawn under the open one");

        // **What makes the sheet end above the strip is the drawing's own
        // stylesheet**: the frame lays its sheet and the strips out as a
        // column, the sheet may shrink, and the strips do not. Node cannot see
        // a layout, so these are held instead.
        let sheet = fs::read_to_string(web.join("the-drawing.css")).expect("the drawing's stylesheet");
        let rule = |selector: &str| -> String {
            let at = sheet
                .find(&format!("\n{selector} {{"))
                .unwrap_or_else(|| panic!("web/the-drawing.css has no rule for {selector}"));
            sheet[at..].split('}').next().unwrap_or_default().to_string()
        };
        for (selector, declaration) in [
            (".sheetwrap", "display: flex; flex-direction: column;"),
            (".sheet", "min-height: 0;"),
            (".strips", "flex: none;"),
        ] {
            assert!(
                rule(selector).contains(declaration),
                "`{selector}` lost `{declaration}`, so the open panel no longer ends above the other's strip"
            );
        }
    }

    /// **The Creator's seventh notes, note 2: while one panel is open, the
    /// other's strip is always there** — `Design/Whole app § Panels over a
    /// server`, changed 2026-10-10, and the drawing's `.strip.fresh` with the
    /// states `res-chkfresh` and `chkfresh-set`. Piece Z6.
    ///
    /// Run both ways: Render Settings open over a server whose check was never
    /// opened, and the check open over one whose Render Settings never was.
    /// The never-opened panel's strip is under the open one and reads its name
    /// alone — no count, no mark. Pressing it opens that panel through its own
    /// way in that starts nothing (`itIsShownOn`), on the open panel's file,
    /// and never through the strip's ordinary reopening. With the open panel
    /// collapsed there is no file to open on, so no strip. **It fails if
    /// section 5's *only a panel already opened has a strip* is put back, or
    /// if the press goes anywhere but the panel's own way in.**
    #[test]
    fn a_panel_never_opened_has_its_strip_while_the_other_is_open() {
        let rows = vec![a_row(Some("RunPod 4090"))];
        let list = crate::surface::servers::the_screen_from(&rows, &[], None, None, &Default::default());
        let server = list.rows[0].server.as_str().to_string();
        let answer = the_screen(WhatTheRecordSaid::Rows(rows), &server, None, &Default::default());
        let screen = serde_json::to_value(&answer).expect("the screen serializes");
        let name_of = |panel: &str| -> serde_json::Value {
            screen["the_panels"]
                .as_array()
                .and_then(|all| all.iter().find(|one| one["panel"] == panel))
                .map(|one| one["name"].clone())
                .unwrap_or_default()
        };
        let (check, settings) = (name_of("chk"), name_of("set"));
        assert!(check.is_string() && settings.is_string(), "both panels are named in words");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for (first, other, others_name) in [("set", "chk", &check), ("chk", "set", &settings)] {
            let answers = serde_json::json!({ "server": server, "screen": answer, "first": first });
            let scratch = std::env::temp_dir().join(format!("smoozy-a-fresh-strip-{first}-{}", std::process::id()));
            let _ = fs::remove_dir_all(&scratch);
            fs::create_dir_all(&scratch).expect("a scratch folder");
            fs::copy(web.join("a-server.js"), scratch.join("a-server.js")).expect("the page is copied");
            for (name, text) in THE_STRIPS_STAND_INS {
                fs::write(scratch.join(name), text).expect("a stand-in is written");
            }
            fs::write(scratch.join("package.json"), r#"{ "type": "module" }"#).expect("the package note");
            fs::write(scratch.join("answers.json"), answers.to_string()).expect("the answers are written");
            fs::write(scratch.join("harness.mjs"), format!("{THE_STRIPS_DOCUMENT}{THE_FRESH_STRIP_WALK}"))
                .expect("the harness is written");
            let ran = std::process::Command::new("node")
                .arg(scratch.join("harness.mjs"))
                .current_dir(&scratch)
                .output();
            let _ = fs::remove_dir_all(&scratch);
            let Ok(ran) = ran else {
                eprintln!("SKIPPED: no Node on this computer, so the strips were not run");
                return;
            };
            assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
            let got: serde_json::Value =
                serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
            let wanted = serde_json::json!({
                // The one open: the never-opened one's strip under it, its name alone.
                "theFirstOpen": {
                    "open": first,
                    "strips": [{ "panel": other, "under": first, "afterTheSheet": true, "tabReaches": true }],
                    "theStrip": { "fresh": true, "says": others_name, "counts": false, "mark": false }
                },
                // Collapsed: no file to open it on, so only the first's strip.
                "collapsed": {
                    "open": null,
                    "strips": [{ "panel": first, "under": null, "afterTheSheet": false, "tabReaches": true }]
                },
                // Pressed: its own way in, on the open panel's file, nothing else.
                "pressed": {
                    "open": other,
                    "strips": [{ "panel": first, "under": other, "afterTheSheet": true, "tabReaches": true }],
                    "shown": [{ "panel": other, "file": [first.len(), 7, 7] }],
                    "focused": "collapse"
                }
            });
            assert_eq!(got, wanted, "the strip of a panel never opened, with {first} open, is not the drawing's");
        }
    }

    /// **The file's name in a panel's title collapses the panel as its
    /// chevron does** — `Design/Whole app § Panels over a server`, added
    /// 2026-10-12 (the Creator's ninth notes, note 5): *"collapses either
    /// panel"*, and **the strip is unchanged**.
    ///
    /// `web/a-server.js` runs as it is. Render Settings is opened, its strip
    /// told what it reads, and collapsed by its chevron; then brought back up
    /// by its strip and collapsed through `thePanelCollapsesAsItsChevronDoes`,
    /// which the two panels call when their name is pressed. **Both end the
    /// same**: the panel down, its strip along the bottom reading what it read
    /// and holding the focus. **It fails if the name's collapse leaves the
    /// panel up, takes the strip away, or changes what it says.**
    #[test]
    fn the_files_name_collapses_the_panel_as_its_chevron_does() {
        let rows = vec![a_row(Some("RunPod 4090"))];
        let list = crate::surface::servers::the_screen_from(&rows, &[], None, None, &Default::default());
        let server = list.rows[0].server.as_str().to_string();
        let answer = the_screen(WhatTheRecordSaid::Rows(rows), &server, None, &Default::default());
        let answers = serde_json::json!({ "server": server, "screen": answer });
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        let scratch = std::env::temp_dir().join(format!("smoozy-the-names-collapse-{}", std::process::id()));
        let _ = fs::remove_dir_all(&scratch);
        fs::create_dir_all(&scratch).expect("a scratch folder");
        fs::copy(web.join("a-server.js"), scratch.join("a-server.js")).expect("the page is copied");
        for (name, text) in THE_STRIPS_STAND_INS {
            fs::write(scratch.join(name), text).expect("a stand-in is written");
        }
        fs::write(scratch.join("package.json"), r#"{ "type": "module" }"#).expect("the package note");
        fs::write(scratch.join("answers.json"), answers.to_string()).expect("the answers are written");
        fs::write(scratch.join("harness.mjs"), format!("{THE_STRIPS_DOCUMENT}{THE_NAMES_COLLAPSE_WALK}"))
            .expect("the harness is written");
        let ran = std::process::Command::new("node")
            .arg(scratch.join("harness.mjs"))
            .current_dir(&scratch)
            .output();
        let _ = fs::remove_dir_all(&scratch);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the strips were not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let collapsed = serde_json::json!({
            "open": null,
            "strips": [{ "panel": "set", "under": null, "afterTheSheet": false, "tabReaches": true }],
            "focused": "set",
            "theStrip": { "says": "Kitchen, 240 frames", "mark": false, "aLink": false }
        });
        assert_eq!(got["byTheChevron"], collapsed, "the chevron's own collapse is not what this test holds it to");
        assert_eq!(got["backUp"]["open"], "set", "the strip did not bring the panel back");
        assert_eq!(got["byTheName"], collapsed, "the name's collapse is not the chevron's");
    }

    /// **The Creator's tenth notes, note 5: no strip carries a mark** —
    /// `Design/Whole app § Panels over a server`, added 2026-10-17: *"A strip
    /// carries no mark"*, and the drawing's strip lost its dot and its two
    /// rules the same day.
    ///
    /// `web/a-server.js` runs as it is. Both panels are opened and told what
    /// their strips read — Render Settings with the mark its page used to ask
    /// for, so a strip that still honoured one would draw it — and the two
    /// strips are read with both collapsed and with each open over the other.
    /// **Neither draws a mark.** And nothing asks for one any more: neither
    /// panel's page passes one, `aStrip` builds none, and the drawing's
    /// stylesheet keeps no rule for one. **It fails if the Render Settings
    /// strip's amber dot comes back by any of those routes.**
    #[test]
    fn no_strip_carries_a_mark() {
        let rows = vec![a_row(Some("RunPod 4090"))];
        let list = crate::surface::servers::the_screen_from(&rows, &[], None, None, &Default::default());
        let server = list.rows[0].server.as_str().to_string();
        let answer = the_screen(WhatTheRecordSaid::Rows(rows), &server, None, &Default::default());
        let answers = serde_json::json!({ "server": server, "screen": answer });
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        let scratch = std::env::temp_dir().join(format!("smoozy-no-strip-mark-{}", std::process::id()));
        let _ = fs::remove_dir_all(&scratch);
        fs::create_dir_all(&scratch).expect("a scratch folder");
        fs::copy(web.join("a-server.js"), scratch.join("a-server.js")).expect("the page is copied");
        for (name, text) in THE_STRIPS_STAND_INS {
            fs::write(scratch.join(name), text).expect("a stand-in is written");
        }
        fs::write(scratch.join("package.json"), r#"{ "type": "module" }"#).expect("the package note");
        fs::write(scratch.join("answers.json"), answers.to_string()).expect("the answers are written");
        fs::write(scratch.join("harness.mjs"), format!("{THE_STRIPS_DOCUMENT}{THE_NO_MARK_WALK}"))
            .expect("the harness is written");
        let ran = std::process::Command::new("node")
            .arg(scratch.join("harness.mjs"))
            .current_dir(&scratch)
            .output();
        let _ = fs::remove_dir_all(&scratch);
        match ran {
            Err(_) => eprintln!("SKIPPED: no Node on this computer, so the strips were not run"),
            Ok(ran) => {
                assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
                let got: serde_json::Value = serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim())
                    .expect("the harness prints one answer");
                let no_mark = |panel: &str| serde_json::json!({ "panel": panel, "marks": 0, "counts": true });
                let wanted = serde_json::json!({
                    "bothCollapsed": [no_mark("chk"), no_mark("set")],
                    "theCheckOpen": [no_mark("set")],
                    "theSettingsOpen": [no_mark("chk")],
                });
                assert_eq!(got, wanted, "a strip draws a mark");
            }
        }

        // **Nothing asks for one, builds one, or styles one.**
        let page = |name: &str| fs::read_to_string(web.join(name)).expect("the page is readable");
        for name in ["render-settings.js", "the-dependency-check.js"] {
            let text = page(name);
            let call = text
                .split("theStripReads(server,")
                .nth(1)
                .and_then(|rest| rest.split("});").next())
                .unwrap_or_else(|| panic!("web/{name} no longer tells its strip what it reads"));
            assert!(!call.contains("mark"), "web/{name} still asks its strip for a mark");
        }
        let a_server = page("a-server.js");
        let the_strip = a_server
            .split("function aStrip(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the strip's builder");
        assert!(!the_strip.contains("\"mk\""), "`aStrip` still builds a mark");
        let sheet = page("the-drawing.css");
        for line in sheet.lines().filter(|line| line.contains(".strip")) {
            assert!(!line.contains(".mk"), "the drawing's stylesheet still styles a strip's mark: {line}");
        }
    }

    /// **The Creator's twelfth notes, note 8 (W5): opening either panel on a
    /// different `.blend` moves both panels to that file** — `Design/Whole app
    /// § Panels over a server`, after line 139, Creator-stated 2026-10-18.
    ///
    /// `web/a-server.js` runs as it is, with both panels registered and each
    /// keeping its own file. Render Settings is opened on the kitchen file and
    /// its strip told what it reads; the check opened on the **same** file
    /// moves nothing, and Render Settings waits as its ordinary strip. The
    /// check opened on a **different** file lets Render Settings go: its strip
    /// is the fresh one, reading its name alone, the box that held the focus
    /// was let go of first (so what it held is handed in), and only the
    /// check's frame is left. Pressing that strip opens Render Settings on the
    /// check's file through its own way in. Then the other way round: Render
    /// Settings opened on a third file lets the check go. **And choosing a row
    /// in Files opens nothing**: its handler calls no panel's way in.
    ///
    /// **What it bites on**: the other panel left on the old file (the state
    /// before W5), a panel let go on the same file, and the focused box taken
    /// down without being let go of first.
    #[test]
    fn w5_opening_a_panel_on_a_different_blend_moves_both_panels() {
        let rows = vec![a_row(Some("RunPod 4090"))];
        let list = crate::surface::servers::the_screen_from(&rows, &[], None, None, &Default::default());
        let server = list.rows[0].server.as_str().to_string();
        let answer = the_screen(WhatTheRecordSaid::Rows(rows), &server, None, &Default::default());
        let screen = serde_json::to_value(&answer).expect("the screen serializes");
        let name_of = |panel: &str| -> serde_json::Value {
            screen["the_panels"]
                .as_array()
                .and_then(|all| all.iter().find(|one| one["panel"] == panel))
                .map(|one| one["name"].clone())
                .unwrap_or_default()
        };
        let (check, settings) = (name_of("chk"), name_of("set"));
        assert!(check.is_string() && settings.is_string(), "both panels are named in words");
        let answers = serde_json::json!({ "server": server, "screen": answer });
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        let scratch = std::env::temp_dir().join(format!("smoozy-both-panels-move-{}", std::process::id()));
        let _ = fs::remove_dir_all(&scratch);
        fs::create_dir_all(&scratch).expect("a scratch folder");
        fs::copy(web.join("a-server.js"), scratch.join("a-server.js")).expect("the page is copied");
        for (name, text) in THE_STRIPS_STAND_INS {
            fs::write(scratch.join(name), text).expect("a stand-in is written");
        }
        fs::write(scratch.join("package.json"), r#"{ "type": "module" }"#).expect("the package note");
        fs::write(scratch.join("answers.json"), answers.to_string()).expect("the answers are written");
        fs::write(scratch.join("harness.mjs"), format!("{THE_STRIPS_DOCUMENT}{THE_BOTH_PANELS_MOVE_WALK}"))
            .expect("the harness is written");
        let ran = std::process::Command::new("node")
            .arg(scratch.join("harness.mjs"))
            .current_dir(&scratch)
            .output();
        let _ = fs::remove_dir_all(&scratch);
        match ran {
            Err(_) => eprintln!("SKIPPED: no Node on this computer, so the panels were not run"),
            Ok(ran) => {
                assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
                let got: serde_json::Value = serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim())
                    .expect("the harness prints one answer");
                let (garden, hall) = (serde_json::json!([1, 71]), serde_json::json!([1, 72]));
                let wanted = serde_json::json!({
                    // The same file: Render Settings stays, reading what it read.
                    "theSameFile": {
                        "open": "chk",
                        "strips": [{ "panel": "set", "under": "chk", "afterTheSheet": true, "tabReaches": true }],
                        "theStrip": { "fresh": false, "says": "Kitchen, 240 frames", "counts": true },
                        "frames": 2,
                        "letGo": []
                    },
                    // A different file: Render Settings lets go, its strip its name alone.
                    "aDifferentFile": {
                        "open": "chk",
                        "strips": [{ "panel": "set", "under": "chk", "afterTheSheet": true, "tabReaches": true }],
                        "theStrip": { "fresh": true, "says": settings, "counts": false },
                        "frames": 1,
                        "letGo": ["set"]
                    },
                    // Its strip pressed: its own way in, on the check's file.
                    "pressed": {
                        "open": "set",
                        "strips": [{ "panel": "chk", "under": "set", "afterTheSheet": true, "tabReaches": true }],
                        "theStrip": { "fresh": false, "says": check, "counts": false },
                        "frames": 2,
                        "shown": [{ "panel": "set", "file": garden }]
                    },
                    // The other way round: the check lets go.
                    "theOtherWay": {
                        "open": "set",
                        "strips": [{ "panel": "chk", "under": "set", "afterTheSheet": true, "tabReaches": true }],
                        "theStrip": { "fresh": true, "says": check, "counts": false },
                        "frames": 1,
                        "files": { "set": hall, "chk": garden }
                    }
                });
                assert_eq!(got, wanted, "opening a panel on a different .blend does not move both panels");
            }
        }

        // **Choosing a row in Files moves neither**: one click chooses and
        // asks for nothing, so its handler reaches no panel's way in.
        let files = fs::read_to_string(web.join("files.js")).expect("Files' page is readable");
        let choosing = files
            .split("function theChosenFileIsShown(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("Files' handler for a chosen row");
        for way_in in ["thePanelIsOpenedOver", "theSettingsAreOpenedOn", "theCheckIsOpenedOn", "theCheckIsShownOn"] {
            assert!(!choosing.contains(way_in), "choosing a row in Files calls {way_in}");
        }
    }

    /// The walk for the twelfth notes, note 8. Each file is two bytes; the
    /// kitchen's second is `K` (75), the garden's `G` (71), the hall's `H` (72).
    const THE_BOTH_PANELS_MOVE_WALK: &str = r#"
const files = { [page.RENDER_SETTINGS]: [1, 75], [page.THE_DEPENDENCY_CHECK]: [1, 75] };
const shown = [];
const letGo = [];
for (const panel of [page.RENDER_SETTINGS, page.THE_DEPENDENCY_CHECK]) {
  page.aPanelIsRegistered({
    panel,
    itIsDrawn(parts) {
      const box = document.createElement("input");
      box.dataset.box = panel;
      box.addEventListener("blur", () => letGo.push(panel));
      parts.body.appendChild(box);
    },
    theFileItIsOn() { return files[panel]; },
    itIsShownOn(at, file) { shown.push({ panel, file }); files[panel] = file; page.thePanelIsOpenedOver(at, panel); },
  });
}
const aReading = (one) => one ? {
  fresh: one.classList.contains("fresh"),
  says: one.querySelector(".t").textContent,
  counts: Boolean(one.querySelector(".d")),
} : null;
const theBoxOf = (panel) => wraps().find((one) => one.dataset.panel === panel).querySelector(`[data-box="${panel}"]`);
const now = (other) => ({ ...theState(), theStrip: aReading(stripOf(other)), frames: wraps().length });
const got = {};

page.thePanelIsOpenedOver(server, page.RENDER_SETTINGS);
page.theStripReads(server, page.RENDER_SETTINGS, { title: { lead: "Kitchen, 240 frames" }, reading: [{ lead: "240 frames" }] });
page.thePanelIsOpenedOver(server, page.THE_DEPENDENCY_CHECK);
got.theSameFile = { ...now(page.RENDER_SETTINGS), letGo: [...letGo] };

stripOf(page.RENDER_SETTINGS).click();
theBoxOf(page.RENDER_SETTINGS).focus();
files[page.THE_DEPENDENCY_CHECK] = [1, 71];
page.thePanelIsOpenedOver(server, page.THE_DEPENDENCY_CHECK);
got.aDifferentFile = { ...now(page.RENDER_SETTINGS), letGo: [...letGo] };

stripOf(page.RENDER_SETTINGS).click();
await new Promise((done) => setTimeout(done, 0));
got.pressed = { ...now(page.THE_DEPENDENCY_CHECK), shown };

files[page.RENDER_SETTINGS] = [1, 72];
page.thePanelIsOpenedOver(server, page.RENDER_SETTINGS);
got.theOtherWay = { ...now(page.THE_DEPENDENCY_CHECK), files: { set: files[page.RENDER_SETTINGS], chk: files[page.THE_DEPENDENCY_CHECK] } };
console.log(JSON.stringify(got));
"#;

    /// The walk for the tenth notes, note 5. Each panel's reading carries a
    /// figure, so the strip has something to count.
    const THE_NO_MARK_WALK: &str = r#"
const got = {};
const theStrips = () => stripsSeen().map((one) => ({
  panel: one.dataset.panel,
  marks: one.querySelectorAll(".mk").length,
  counts: Boolean(one.querySelector(".d")),
}));
page.thePanelIsOpenedOver(server, page.RENDER_SETTINGS);
page.theStripReads(server, page.RENDER_SETTINGS, { title: { lead: "Kitchen" }, reading: [{ lead: "240 frames" }], mark: true });
page.thePanelIsOpenedOver(server, page.THE_DEPENDENCY_CHECK);
page.theStripReads(server, page.THE_DEPENDENCY_CHECK, { title: { lead: "Dependencies" }, reading: [{ lead: "Uploading" }] });
got.theCheckOpen = theStrips();
page.thePanelIsOpenedOver(server, page.RENDER_SETTINGS);
got.theSettingsOpen = theStrips();
const press = { key: "Escape", defaultPrevented: false, target: document.activeElement, preventDefault() { this.defaultPrevented = true; } };
for (const handler of documentListeners.keydown || []) handler(press);
got.bothCollapsed = theStrips();
console.log(JSON.stringify(got));
"#;

    /// The walk through the ninth notes, note 5.
    const THE_NAMES_COLLAPSE_WALK: &str = r#"
const got = {};
const aReading = (one) => one ? {
  says: one.querySelector(".t").textContent,
  mark: Boolean(one.querySelector(".mk")),
  aLink: Boolean(one.querySelector(".golink")),
} : null;
page.thePanelIsOpenedOver(server, page.RENDER_SETTINGS);
page.theStripReads(server, page.RENDER_SETTINGS, { title: { lead: "Kitchen, 240 frames" } });
host.querySelectorAll(".collapse").filter(seen)[0].click();
got.byTheChevron = { ...theState(), focused: whatIsFocused(), theStrip: aReading(stripOf(page.RENDER_SETTINGS)) };
stripOf(page.RENDER_SETTINGS).click();
got.backUp = theState();
page.thePanelCollapsesAsItsChevronDoes(server);
got.byTheName = { ...theState(), focused: whatIsFocused(), theStrip: aReading(stripOf(page.RENDER_SETTINGS)) };
console.log(JSON.stringify(got));
"#;

    /// The walk through the seventh notes, note 2. Each panel's file is three
    /// bytes, the first its key's length, so which file was handed shows.
    const THE_FRESH_STRIP_WALK: &str = r#"
const first = answers.first;
const other = first === page.RENDER_SETTINGS ? page.THE_DEPENDENCY_CHECK : page.RENDER_SETTINGS;
const shown = [];
for (const panel of [page.RENDER_SETTINGS, page.THE_DEPENDENCY_CHECK]) {
  page.aPanelIsRegistered({
    panel,
    itIsDrawn() {},
    theFileItIsOn() { return [panel.length, 7, 7]; },
    itIsShownOn(at, file) { shown.push({ panel, file }); page.thePanelIsOpenedOver(at, panel); },
  });
}
const got = {};
const aReading = (one) => one ? {
  fresh: one.classList.contains("fresh"),
  says: one.querySelector(".t").textContent,
  counts: Boolean(one.querySelector(".d")),
  mark: Boolean(one.querySelector(".mk")),
} : null;
page.thePanelIsOpenedOver(server, first);
got.theFirstOpen = { ...theState(), theStrip: aReading(stripOf(other)) };
const press = { key: "Escape", defaultPrevented: false, target: document.activeElement, preventDefault() { this.defaultPrevented = true; } };
for (const handler of documentListeners.keydown || []) handler(press);
got.collapsed = theState();
stripOf(first).click();
const fresh = stripOf(other);
if (fresh) fresh.click();
await new Promise((done) => setTimeout(done, 0));
got.pressed = { ...theState(), shown, focused: whatIsFocused() };
console.log(JSON.stringify(got));
"#;

    /// What `web/a-server.js` imports, standing in; none is looked at.
    const THE_STRIPS_STAND_INS: [(&str, &str); 4] = [
        (
            "the-rail.js",
            "export const A_SERVER = 'machineview'; \
             export function aScreenIsRegistered(screen) { globalThis.screens.push(screen); }",
        ),
        ("servers.js", "export function theMomentReads(seconds) { return String(seconds); } export function theConnectionIsDrawnInto() {}"),
        ("event-route.js", "export function whenTold() {}"),
        (
            "the-value-drawn.js",
            "export function theValueIsDrawnInto(node, value) { node.textContent = value; }",
        ),
    ];

    /// **A stand-in document that is just enough for `web/a-server.js`**, and
    /// the walk through note 3. An element is seen when it is in the document
    /// and nothing it sits in is hidden; focus on an element that is no longer
    /// seen falls back to the body, as the engine's does.
    const THE_STRIPS_DOCUMENT: &str = r#"import { readFileSync } from "node:fs";
const answers = JSON.parse(readFileSync("./answers.json", "utf8"));

class Text {
  constructor(text) { this.data = String(text); this.parentNode = null; }
  get textContent() { return this.data; }
}
class Element {
  constructor(tag) {
    this.tagName = tag.toUpperCase(); this.children = []; this.parentNode = null;
    this.hidden = false; this.dataset = {}; this.attributes = {}; this.className = "";
    this.listeners = {}; this.type = ""; this.title = "";
    const self = this;
    this.classList = {
      add(name) { if (!self.className.split(" ").includes(name)) self.className = (self.className + " " + name).trim(); },
      toggle(name, on) { const names = self.className.split(" ").filter((one) => one && one !== name); if (on) names.push(name); self.className = names.join(" "); },
      contains(name) { return self.className.split(" ").includes(name); },
    };
  }
  appendChild(node) { return this.insertBefore(node, null); }
  insertBefore(node, before) {
    if (node.parentNode) node.parentNode.removeChild(node);
    const at = before ? this.children.indexOf(before) : -1;
    if (at < 0) this.children.push(node); else this.children.splice(at, 0, node);
    node.parentNode = this;
    return node;
  }
  removeChild(node) { const at = this.children.indexOf(node); if (at >= 0) this.children.splice(at, 1); node.parentNode = null; return node; }
  remove() { if (this.parentNode) this.parentNode.removeChild(this); }
  get childElementCount() { return this.children.filter((one) => one instanceof Element).length; }
  get textContent() { return this.children.map((one) => one.textContent).join(""); }
  set textContent(text) { for (const one of this.children) one.parentNode = null; this.children = []; if (text) this.appendChild(new Text(text)); }
  setAttribute(name, value) { this.attributes[name] = String(value); }
  getAttribute(name) { return name in this.attributes ? this.attributes[name] : null; }
  addEventListener(kind, handler) { (this.listeners[kind] ||= []).push(handler); }
  click() { for (const handler of this.listeners.click || []) handler({ target: this }); }
  focus() { focused = this; }
  blur() { if (focused !== this) return; focused = null; for (const handler of this.listeners.blur || []) handler({ target: this }); }
  contains(node) { for (let at = node; at; at = at.parentNode) if (at === this) return true; return false; }
  matches(selector) {
    if (selector.startsWith(".")) return this.classList.contains(selector.slice(1));
    const found = /^\[([\w-]+)=(?:"([^"]*)"|([^\]"]*))\]$/.exec(selector);
    if (!found) throw new Error("a selector the stand-in does not read: " + selector);
    const [, name, quoted, bare] = found;
    const value = quoted !== undefined ? quoted : bare;
    if (name.startsWith("data-")) {
      const key = name.slice(5).replace(/-(\w)/g, (_, letter) => letter.toUpperCase());
      return this.dataset[key] === value;
    }
    return this.getAttribute(name) === value;
  }
  everything() { return this.children.filter((one) => one instanceof Element).flatMap((one) => [one, ...one.everything()]); }
  querySelector(selector) { return this.everything().find((one) => one.matches(selector)) || null; }
  querySelectorAll(selector) { return this.everything().filter((one) => one.matches(selector)); }
  closest(selector) { for (let at = this; at instanceof Element; at = at.parentNode) if (at.matches(selector)) return at; return null; }
}

const body = new Element("body");
let focused = null;
const seen = (node) => { for (let at = node; at; at = at.parentNode) { if (at === body) return true; if (at.hidden) return false; } return false; };
const documentListeners = {};
globalThis.document = {
  body,
  get activeElement() { return focused && seen(focused) ? focused : body; },
  createElement: (tag) => new Element(tag),
  createElementNS: (_, tag) => new Element(tag),
  createTextNode: (text) => new Text(text),
  addEventListener(kind, handler) { (documentListeners[kind] ||= []).push(handler); },
};
globalThis.screens = [];
globalThis.window = { __TAURI__: { core: { invoke(command) {
  return Promise.resolve(command === "the_servers_own_screen" ? answers.screen : null);
} } } };

const page = await import("./a-server.js");
const screen = globalThis.screens[0];
const host = new Element("div");
host.id = "machineview";
body.appendChild(host);
screen.itIsDrawn(host);
// Both panels are built; what is drawn inside them is not this test's.
page.aPanelIsRegistered({ panel: page.RENDER_SETTINGS, itIsDrawn() {} });
page.aPanelIsRegistered({ panel: page.THE_DEPENDENCY_CHECK, itIsDrawn() {} });
const server = answers.server;
screen.itIsCurrentNow(server);
await new Promise((done) => setTimeout(done, 0));

const wraps = () => host.querySelectorAll(".sheetwrap");
const theOpen = () => { const open = wraps().filter(seen); return open.length ? open[0].dataset.panel : null; };
const stripsSeen = () => host.querySelectorAll(".strip").filter(seen);
const tabReaches = (control) => control.tagName === "BUTTON" && seen(control)
  && control.getAttribute("tabindex") !== "-1" && !control.disabled;
const aStrip = (one) => {
  const wrap = one.closest(".sheetwrap");
  let afterTheSheet = false;
  if (wrap) {
    const sheet = wrap.children.find((part) => part.classList && part.classList.contains("sheet"));
    const holder = wrap.children.find((part) => part.contains(one));
    afterTheSheet = Boolean(sheet && holder) && wrap.children.indexOf(holder) > wrap.children.indexOf(sheet);
  }
  return { panel: one.dataset.panel, under: wrap ? wrap.dataset.panel : null, afterTheSheet, tabReaches: tabReaches(one) };
};
const theState = () => ({ open: theOpen(), strips: stripsSeen().map(aStrip) });
const whatIsFocused = () => {
  const now = document.activeElement;
  if (now.classList && now.classList.contains("collapse")) return "collapse";
  if (now.classList && now.classList.contains("strip")) return now.dataset.panel;
  return now === body ? "body" : now.tagName;
};
const stripOf = (panel) => stripsSeen().find((one) => one.dataset.panel === panel);
"#;

    /// The walk through the fifth notes, note 3.
    const THE_STRIPS_WALK: &str = r#"
const got = {};
page.thePanelIsOpenedOver(server, page.RENDER_SETTINGS);
got.oneEverOpened = theState();
page.thePanelIsOpenedOver(server, page.THE_DEPENDENCY_CHECK);
got.theCheckOpened = theState();
page.thePanelIsOpenedOver(server, page.RENDER_SETTINGS);
got.theSettingsAgain = theState();

const waiting = stripOf(page.THE_DEPENDENCY_CHECK);
if (waiting) {
  waiting.focus();
  page.theStripReads(server, page.THE_DEPENDENCY_CHECK, { title: { lead: "Uploading" } });
  const now = stripOf(page.THE_DEPENDENCY_CHECK);
  got.aReadingWhileFocused = { focused: whatIsFocused(), says: now ? now.querySelector(".t").textContent : null };
  stripOf(page.THE_DEPENDENCY_CHECK).click();
  got.theStripPressed = { ...theState(), focused: whatIsFocused() };
} else {
  got.aReadingWhileFocused = null;
  got.theStripPressed = null;
}

const press = { key: "Escape", defaultPrevented: false, target: document.activeElement, preventDefault() { this.defaultPrevented = true; } };
for (const handler of documentListeners.keydown || []) handler(press);
got.bothCollapsed = { ...theState(), focused: whatIsFocused() };

console.log(JSON.stringify(got));
"#;
}

#[cfg(test)]
mod pane_drawing_states {
    #[test]
    fn the_eleven_pane_states_belong_to_the_server_heading_and_rail_menu() {
        let drawing = include_str!("../../../../Preparation/Design/Mockup/Smoozy.html");
        let group = drawing
            .split("<optgroup label=\"A server\">")
            .nth(1)
            .unwrap()
            .split("</optgroup>")
            .next()
            .unwrap();
        let mut states: Vec<_> = group
            .lines()
            .filter_map(|line| line.split("<option value=\"").nth(1))
            .map(|rest| rest.split('"').next().unwrap())
            .filter(|state| state.starts_with("pane-"))
            .collect();
        states.sort();
        let mut claimed = vec![
            "pane-connected",
            "pane-off",
            "pane-connecting",
            "pane-repair",
            "pane-setup",
            "pane-key",
            "pane-restart",
            "pane-failed",
            "pane-notool",
            "pane-keyed",
            "pane-railmenu",
        ];
        claimed.sort();
        assert_eq!(states, claimed);
    }
}

#[cfg(test)]
mod pane_and_rail_browser {
    // Seventeenth notes 15–16: the real page modules, in a real browser.
    use std::{
        fs,
        path::{Path, PathBuf},
        process::Command,
    };

    #[test]
    fn pane_connect_and_the_rail_menu_keep_the_rows_rules() {
        the_walk_passes("smoozy-pane-rail", WALK);
    }

    /// **The Creator's eighteenth notes, note 1: a removed server does not
    /// stay in its pane.** The server's own screen is open on *server-one*
    /// with Render Settings open over it; the server is removed through the
    /// rail's menu and its box, and the application answers *it was saved*
    /// with the server gone from what it holds. **The open pane asks for its
    /// server again** and draws section 7 of `web/a-server.js`: the head and
    /// the tab go, *Removed* is said, and the server's panel is taken down.
    /// Then the same through the All servers screen's menu, for a second
    /// server whose panel waits while that screen is showing: its panel is
    /// taken down too, and nothing is drawn over the screen that is showing.
    ///
    /// Until then the answer read the rail and the list again and never asked
    /// the pane, so the removed server's head, tab and panel stayed.
    #[test]
    fn eighteenth_notes_1_a_removed_server_leaves_its_pane_from_either_menu() {
        the_walk_passes("smoozy-pane-removed", REMOVAL_WALK);
    }

    /// **The Creator's twenty-third notes, note 4: a box is drawn above a
    /// panel.** Render Settings is opened over *server-one*, and separately
    /// the dependency check over *server-two*; with each one open, every
    /// Servers box that can be raised over it — *Remove*, *Edit*, *Server
    /// identity changed* and the owed *stop and clear* ending — is opened in
    /// turn, and the browser is asked what is drawn at the box's middle: the
    /// box, where before the panel covered it (a plain box at 12, a panel at
    /// 20). *Add a server* is opened from the All servers screen, where the
    /// panel is not on screen at all.
    #[test]
    fn twenty_third_notes_4_a_box_is_drawn_above_an_open_panel() {
        let row = smoozy_servers::store::ServerRow::saved(
            "server-one",
            smoozy_servers::identity::Host::what_the_user_typed("127.0.0.1").unwrap(),
            smoozy_servers::identity::Port::what_the_user_typed(22).unwrap(),
            smoozy_servers::identity::Username::what_the_user_typed("root").unwrap(),
            smoozy_servers::identity::KeyReference {
                local_key_identifier: "key-one".into(),
                fingerprint: "SHA256:a".into(),
            },
        );
        let rows = [row.clone()];
        let edit = crate::surface::servers::the_edit_dialog_from(&row, &rows, &[], None);
        let add = crate::surface::servers::the_add_dialog_from(&rows);
        let walk = BOXES_OVER_PANELS_WALK
            .replace("EDIT_VALUE", &serde_json::to_string(&edit).unwrap())
            .replace("ADD_VALUE", &serde_json::to_string(&add).unwrap());
        the_walk_passes("smoozy-boxes-over-panels", &walk);
    }

    /// Runs `walk` against the real rail, list and pane modules in Chrome,
    /// over the application's own five menus and row, and fails unless it
    /// prints `PASS`. Skips, and says so, where no Chrome is installed.
    fn the_walk_passes(name: &str, walk: &str) {
        let chrome = [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/usr/bin/google-chrome",
            "/usr/bin/chromium",
        ]
        .into_iter()
        .map(PathBuf::from)
        .find(|p| p.exists());
        let Some(chrome) = chrome else {
            eprintln!("SKIPPED: Chrome is absent; pane and rail browser checks were not run");
            return;
        };
        let folder = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
        fs::create_dir_all(&folder).unwrap();
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for name in [
            "servers.js",
            "a-server.js",
            "the-rail.js",
            "the-drawing.css",
            "servers.css",
            "a-server.css",
            "the-rail.css",
        ] {
            let source = std::env::var_os("SMOOZY_PANE_RAIL_TEST_BASELINE")
                .map(PathBuf::from)
                .filter(|_| name.ends_with(".js"))
                .unwrap_or_else(|| web.clone());
            fs::copy(source.join(name), folder.join(name)).unwrap();
        }
        for (name, source) in STAND_INS {
            fs::write(folder.join(name), source).unwrap();
        }
        // Use the application's actual five menus and actual row composition.
        let row = smoozy_servers::store::ServerRow::saved(
            "server-one",
            smoozy_servers::identity::Host::what_the_user_typed("127.0.0.1").unwrap(),
            smoozy_servers::identity::Port::what_the_user_typed(22).unwrap(),
            smoozy_servers::identity::Username::what_the_user_typed("root").unwrap(),
            smoozy_servers::identity::KeyReference {
                local_key_identifier: "key-one".into(),
                fingerprint: "SHA256:a".into(),
            },
        );
        let screen =
            crate::surface::servers::the_screen_from(&[row], &[], None, None, &Default::default());
        let screen = serde_json::to_string(&screen).unwrap();
        let walk = walk.replace("SCREEN_VALUE", &screen);
        let (setup, walk) = walk
            .split_once("const out=document.getElementById('out');")
            .unwrap();
        fs::write(folder.join("setup.js"), setup).unwrap();
        fs::write(folder.join("walk.js"), format!("import * as rail from './the-rail.js';\nimport * as servers from './servers.js';\nimport * as pane from './a-server.js';\nimport {{tell}} from './event-route.js';\nconst out=document.getElementById('out');{walk}")).unwrap();
        fs::write(folder.join("index.html"), r#"<!doctype html><link rel="stylesheet" href="the-drawing.css"><link rel="stylesheet" href="servers.css"><link rel="stylesheet" href="a-server.css"><link rel="stylesheet" href="the-rail.css"><div class="app"><div id="pane" class="pane"></div></div><pre id="out"></pre><script src="setup.js"></script><script type="module" src="walk.js"></script>"#).unwrap();
        let dump = folder.join("dump.html");
        // The test's process has one fixed deadline. No polling or wait loop.
        let mut child = Command::new(chrome)
            .args([
                "--headless",
                "--disable-gpu",
                "--no-first-run",
                "--allow-file-access-from-files",
                "--virtual-time-budget=5000",
                "--window-size=1000,800",
                "--dump-dom",
            ])
            .arg(format!(
                "--user-data-dir={}",
                folder.join("profile").display()
            ))
            .arg(format!("file://{}", folder.join("index.html").display()))
            .stdout(fs::File::create(&dump).unwrap())
            .stderr(fs::File::create(folder.join("chrome-errors.txt")).unwrap())
            .spawn()
            .unwrap();
        let pid = child.id();
        let (done, end) = std::sync::mpsc::channel();
        let watcher = std::thread::spawn(move || {
            if end
                .recv_timeout(std::time::Duration::from_secs(45))
                .is_err()
            {
                let _ = Command::new("kill")
                    .args(["-KILL", &pid.to_string()])
                    .status();
            }
        });
        let status = child.wait().unwrap();
        let _ = done.send(());
        watcher.join().unwrap();
        let dumped = fs::read_to_string(&dump).unwrap();
        let result = dumped
            .split("<pre id=\"out\">")
            .nth(1)
            .and_then(|s| s.split("</pre>").next())
            .unwrap_or("")
            .to_string();
        let errors = fs::read_to_string(folder.join("chrome-errors.txt")).unwrap_or_default();
        let _ = fs::remove_dir_all(&folder);
        assert!(
            status.success() && result == "PASS",
            "browser: {result}; status: {status}; {errors}"
        );
    }

    const STAND_INS: &[(&str, &str)] = &[
        (
            "event-route.js",
            "export const listeners = new Map(); export function whenTold(k,f) { if (!listeners.has(k)) listeners.set(k,[]); listeners.get(k).push(f); } export async function tell(k,d={}) { for (const f of listeners.get(k)||[]) await f(d); }",
        ),
        ("panes.js", "export function theSeamsAreLive() {}"),
        (
            "the-value-drawn.js",
            "export function theValueIsDrawnInto(n,v) { n.textContent=v; } export function theValueDrawn(v) { const n=document.createElement('span'); n.textContent=v; return n; }",
        ),
        ("keys.js", "export function aServerDialogPicksAKey() {}"),
        (
            "the-underway-shape.js",
            "export function theCountdown() { return document.createElement('span'); } export function theMarkOnAControl(n) { n.classList.add('wk'); n.setAttribute('aria-disabled','true'); } export function theMarkComesOffAControl(n) { n.classList.remove('wk'); n.removeAttribute('aria-disabled'); }",
        ),
        (
            "the-machines-own-words.js",
            "export function theWordsPanel() { return document.createElement('div'); }",
        ),
        (
            "the-failure-shape.js",
            "export const A_COLLAPSED_ROW='row', EXPANDED='expanded'; export function theControlsFrom() { return []; } export function theFailureShape(o) { const n=document.createElement('div'); n.textContent=o.sentence; return n; }",
        ),
    ];

    const WALK: &str = r#"
const screen = SCREEN_VALUE;
const base = structuredClone(screen.rows[0]);
base.the_menu='disconnected'; base.the_connection={label:'Connect',act:'connect',unavailable:false};
screen.rows=[base, {...structuredClone(base),server:'server-two',name:'Second'}];
const calls=[];
let release=null;
window.__TAURI__={core:{invoke:async (act,args={})=>{
  calls.push([act,args]);
  if(act==='the_rail') return {opens_at:'queueview',the_rails_own_name:'Navigation',the_servers_heading:'Servers',places:[],servers:screen.rows.map(r=>({server:r.server,name:r.name,away:false}))};
  if(act==='the_pane_layout') return {seams:[]};
  if(act==='the_all_servers_screen') return structuredClone(screen);
  if(act==='the_servers_own_screen') { const r=screen.rows.find(r=>r.server===args.server); return {the_head:r?{...r,the_rental_line:{}}:null,the_server_is_gone:!r,the_tabs:[{tab:'files',name:'Files'},{tab:'setupview',name:'Set up'}],the_panels:[],instead:{lead:'Removed',values:[]}}; }
  if(act==='an_act_that_needs_a_connection') return new Promise(go=>release=go);
  if(act==='the_removal_of') return {heading:{lead:'Remove',values:[args.server]},said:'Confirmation',cancel:'Cancel',remove:'Remove'};
  return {};
}}};
const out=document.getElementById('out');
out.textContent='START';
const failures=[];
const check=(ok,why)=>{if(!ok) failures.push(why);};
const flush=async()=>{for(let i=0;i<12;i++) await Promise.resolve();};
try {
 pane.aTabIsRegistered({tab:'files',itIsDrawn(host){const stale=document.createElement('p');stale.className='stale-file';stale.textContent='Not connected. Last seen files.';host.appendChild(stale);}});
 await flush();
 rail.theCurrentPlaceIs('machineview','server-one'); await flush();
 check(!calls.some(c=>c[0]==='an_act_that_needs_a_connection'),'selection connected');
 let control=()=>document.querySelector('#machineview .mtop .cbtn');
 check(control() && control()?.textContent==='Connect','pane has no shared Connect');
 const cases=[
  ['Connect','connect',false,false,false,'disconnected',''],
  ['Connect and set up','connect-and-set-up',false,false,false,'waiting-to-be-set-up',''],
  ['Connect','connect',false,true,false,'disconnected','wk'],
  ['Reconnecting…','disconnect',false,false,true,'connected','trying'],
  ['Connect','connect',true,false,false,'needs-a-new-key','dead'],
  ['Restarting','disconnect',true,false,true,'connected','dead'],
  ['Connected','disconnect',false,false,false,'connected','live'],
  ['Connected','disconnect',false,false,false,'connected-key-gone','live']
 ];
 for(const [label,act,unavailable,busy,repair,menu,cls] of cases){
  Object.assign(base,{the_connection:{label,act,unavailable},the_connect_is_underway:busy,the_connection_is_coming_back:repair,the_menu:menu,the_key_mark:menu.includes('key')?{lead:'Key missing',values:[]}:null});
  await tell('servers:what-is-observed',{subject:base.server}); await flush();
  check(control()?.textContent===label,'wrong pane label '+label);
  check(control()?.disabled===unavailable,'wrong availability '+label);
  if(cls) check(control()?.classList.contains(cls),'missing '+cls+' on '+label);
  if(unavailable||busy){const before=calls.length;control()?.click();check(!calls.slice(before).some(c=>c[0]==='an_act_that_needs_a_connection'),'unavailable act ran');}
  if(menu.includes('key')) check(document.querySelector('#machineview .mconnectnote')?.textContent.includes('Key missing'),'missing key explanation');
 }
 Object.assign(base,{the_connection:{label:'Connect',act:'connect',unavailable:false},the_connect_is_underway:false,the_connection_is_coming_back:false,the_menu:'disconnected',the_key_mark:null});
 await tell('servers:what-is-observed',{subject:base.server}); await flush();
 control()?.click();control()?.click();
 check(calls.filter(c=>c[0]==='an_act_that_needs_a_connection').length===1,'duplicate pending connect');
 if(release) release();await flush();

 await tell('servers:the-answer',{said:{what:'the-connection-was-not-made',server:base.server,said:'Connection failed. Try again.'}});await flush();
 check(document.querySelector('#machineview .mconnectnote')?.textContent.includes('Connection failed'),'failure missing from pane');
 check(control()?.textContent==='Connect' && !control()?.disabled,'failure has no retry');
 servers.theConnectionFailureIsTakenAway(base.server);
 screen.the_tools_statement={lead:'SSH tools are missing.',until:'Until installed.'};
 base.the_connection.unavailable=true;
 await tell('servers:what-is-observed',{subject:base.server});await flush();
 check(control()?.disabled,'missing SSH tools still connects');
 check(document.querySelector('#machineview .mconnectnote')?.textContent.includes('SSH tools'),'missing SSH tools explanation');
 check(document.querySelector('.stale-file')?.textContent==='Not connected. Last seen files.','connection refresh changed stale files');
 screen.the_tools_statement=null;base.the_connection.unavailable=false;
 // Open the other server's menu without selecting it, from the actual rail.
 let row=()=>document.querySelector('.rail [data-server="server-two"]');
 row().focus(); row().dispatchEvent(new MouseEvent('contextmenu',{bubbles:true,cancelable:true,clientX:5,clientY:40})); await flush();
 let menu=()=>document.querySelector('.svctx');
 check(menu(),'rail menu absent');
 check(rail.theCurrentPlace()==='machineview','right click changed place');
 check(document.querySelector('#machineview .mtop h1').textContent===base.name,'right click changed server');
 let rect=menu()?.getBoundingClientRect()||{}, anchor=row().getBoundingClientRect();
 check(rect.left>=anchor.right-1 && rect.right<=innerWidth && rect.top>=0 && rect.bottom<=innerHeight,'menu place outside rail/window');
 check(document.activeElement.getAttribute('role')==='menuitem','menu has no keyboard focus');
 // All five shapes use precisely the same items and disabled readings.
 for(const shape of screen.menus){
  screen.rows[1].the_menu=shape.which;
  await tell('servers:what-is-observed',{subject:'server-two'});await flush();
  const expected=[shape.the_connection,...shape.going_in,...shape.about_itself,...shape.the_details,shape.removing];
  const items=[...(menu()?.querySelectorAll('[role=menuitem]')||[])];
  check(items.length===expected.length,'wrong menu shape '+shape.which);
  items.forEach((n,i)=>check(n.textContent===expected[i].label && n.disabled===expected[i].unavailable,'menu drift '+shape.which));
  const live=expected.filter(item=>!item.unavailable);
  document.activeElement.dispatchEvent(new KeyboardEvent('keydown',{key:'Home',bubbles:true,cancelable:true}));
  document.activeElement.dispatchEvent(new KeyboardEvent('keydown',{key:'ArrowDown',bubbles:true,cancelable:true}));
  check(document.activeElement.textContent===live[1]?.label,'arrow did not skip disabled items '+shape.which);
  document.activeElement.dispatchEvent(new KeyboardEvent('keydown',{key:'Home',bubbles:true,cancelable:true}));
 }
 screen.rows[1].the_menu='connected';
 await tell('servers:what-is-observed',{subject:'server-two'});await flush();
 document.activeElement.dispatchEvent(new KeyboardEvent('keydown',{key:'Home',bubbles:true,cancelable:true}));
 const pressed=document.activeElement;
 pressed.dispatchEvent(new PointerEvent('pointerdown',{bubbles:true,button:0}));
 await tell('servers:what-is-observed',{subject:'server-two'});await flush();
 check(pressed.isConnected,'a menu press was lost to a connection refresh');
 pressed.dispatchEvent(new PointerEvent('pointerup',{bubbles:true,button:0}));
 pressed.click();
 check(calls.some(c=>c[0]==='an_act_that_needs_a_connection' && c[1].server==='server-two' && c[1].act==='disconnect'),'menu command targeted selected pane instead of clicked server');
 if(release) release();await flush();
 row().dispatchEvent(new KeyboardEvent('keydown',{key:'ContextMenu',bubbles:true,cancelable:true}));await flush();
 document.activeElement.dispatchEvent(new KeyboardEvent('keydown',{key:'Enter',bubbles:true,cancelable:true}));
 check(!menu() && calls.filter(c=>c[0]==='an_act_that_needs_a_connection' && c[1].server==='server-two' && c[1].act==='disconnect').length===2,'Enter did not choose the focused item');
 if(release) release();await flush();
 row().dispatchEvent(new KeyboardEvent('keydown',{key:'ContextMenu',bubbles:true,cancelable:true}));await flush();
 document.activeElement.dispatchEvent(new KeyboardEvent('keydown',{key:'ArrowDown',bubbles:true,cancelable:true}));
 check(!document.activeElement.disabled,'arrow focused disabled item');
 document.activeElement.dispatchEvent(new KeyboardEvent('keydown',{key:'Escape',bubbles:true,cancelable:true}));
 check(!menu() && document.activeElement===row(),'Escape failed to close/return focus');
 row().dispatchEvent(new KeyboardEvent('keydown',{key:'F10',shiftKey:true,bubbles:true,cancelable:true}));await flush();check(menu(),'Shift+F10 absent');
 document.body.dispatchEvent(new MouseEvent('mousedown',{bubbles:true}));check(!menu() && document.activeElement===row(),'outside close/focus');
 row().dispatchEvent(new KeyboardEvent('keydown',{key:'ContextMenu',bubbles:true,cancelable:true}));await flush();check(menu(),'Menu key absent');

 // Same Remove confirmation, for the clicked server, before any removal.
 const remove=menu()?.querySelector('[data-act="remove"]');remove?.click();await flush();
 check(calls.some(c=>c[0]==='the_removal_of' && c[1].server==='server-two'),'rail removal targeted wrong server');
 check(!calls.some(c=>c[0]==='a_server_is_removed'),'Remove skipped its confirmation');
 const box=document.querySelector('[role=dialog]');
 check(box && box.getBoundingClientRect().width>0,'confirmation hidden with All servers');
 check(box?.textContent.includes('server-two'),'confirmation names wrong server');
 box?.dispatchEvent(new KeyboardEvent('keydown',{key:'Escape',bubbles:true,cancelable:true}));
 row().dispatchEvent(new KeyboardEvent('keydown',{key:'ContextMenu',bubbles:true,cancelable:true}));await flush();
 // Reading a menu must not consume an owed removal result without showing it.
 document.activeElement.dispatchEvent(new KeyboardEvent('keydown',{key:'Escape',bubbles:true,cancelable:true}));
 screen.the_stop_and_clear_ending={server:'server-two',which:screen.the_boxes_a_connection_raises.stop_and_clear[0].which};
 row().dispatchEvent(new KeyboardEvent('keydown',{key:'ContextMenu',bubbles:true,cancelable:true}));await flush();
 const owed=document.querySelector('[role=dialog]');
 check(owed?.textContent.includes('Second') && !menu(),'rail menu consumed an owed removal ending');
 screen.the_stop_and_clear_ending=null;
 owed?.dispatchEvent(new KeyboardEvent('keydown',{key:'Escape',bubbles:true,cancelable:true}));
 document.activeElement.dispatchEvent(new KeyboardEvent('keydown',{key:'Escape',bubbles:true,cancelable:true}));
 row().dispatchEvent(new KeyboardEvent('keydown',{key:'ContextMenu',bubbles:true,cancelable:true}));await flush();
 screen.rows.pop();await tell('servers:what-is-observed',{subject:'server-two'});await flush();check(!menu(),'removed server kept menu');
 out.textContent=failures.length?failures.join(' | '):'PASS';
} catch(e) { out.textContent=e.stack; }
"#;

    /// The walk for the twenty-third notes, note 4: which is on top, a box or
    /// the panel open under it.
    const BOXES_OVER_PANELS_WALK: &str = r#"
const screen = SCREEN_VALUE;
const base = structuredClone(screen.rows[0]);
base.the_menu='disconnected'; base.the_connection={label:'Connect',act:'connect',unavailable:false};
screen.rows=[base, {...structuredClone(base),server:'server-two',name:'Second'}];
const calls=[];
window.__TAURI__={core:{invoke:async (act,args={})=>{
  calls.push([act,args]);
  if(act==='the_rail') return {opens_at:'queueview',the_rails_own_name:'Navigation',the_servers_heading:'Servers',places:[],servers:screen.rows.map(r=>({server:r.server,name:r.name,away:false}))};
  if(act==='the_pane_layout') return {seams:[]};
  if(act==='the_all_servers_screen') return structuredClone(screen);
  if(act==='the_servers_own_screen') { const r=screen.rows.find(r=>r.server===args.server); return {the_head:r?{...r,the_rental_line:{}}:null,the_server_is_gone:!r,the_tabs:[{tab:'files',name:'Files'},{tab:'setupview',name:'Set up'}],the_panels:[],instead:{lead:'Removed',values:[]}}; }
  if(act==='the_removal_of') return {heading:{lead:'Remove',values:[args.server]},said:'Confirmation',cancel:'Cancel',remove:'Remove'};
  if(act==='the_edit_dialog') return EDIT_VALUE;
  if(act==='the_add_dialog') return ADD_VALUE;
  return {};
}}};
const out=document.getElementById('out');
out.textContent='START';
const failures=[];
const check=(ok,why)=>{if(!ok) failures.push(why);};
const flush=async()=>{for(let i=0;i<12;i++) await Promise.resolve();};
const panelOf=(server)=>document.querySelector('.sheetwrap[data-server="'+server+'"]');
// The middle of the open panel: the place a box over it must win.
let at=[0,0];
const theMiddleOf=(n)=>{const r=n.getBoundingClientRect();return [r.left+r.width/2, r.top+r.height/2];};
const theOpenBox=()=>[...document.querySelectorAll('.modal')].filter(one=>one.getBoundingClientRect().width>0).pop();
const theMenuOf=async(server)=>{
  document.querySelector('.rail [data-server="'+server+'"]').dispatchEvent(new MouseEvent('contextmenu',{bubbles:true,cancelable:true,clientX:5,clientY:40})); await flush();
};
// **Which is on top at the panel's middle**: the box, or the panel.
const theBoxIsOnTop=async(server,which)=>{
  await flush();
  const modal=theOpenBox();
  if(!modal){check(false,which+' did not open over '+server);return;}
  const [x,y]=at||theMiddleOf(modal);
  const hit=document.elementFromPoint(x,y);
  check(hit && modal.contains(hit), which+' is drawn under the panel on '+server+' (hit: '+(hit?hit.className:'nothing')+')');
  (modal.querySelector('[role=dialog]')||modal).dispatchEvent(new KeyboardEvent('keydown',{key:'Escape',bubbles:true,cancelable:true}));
  await flush();
  if(theOpenBox()) theOpenBox().remove();
  // A menu left open is closed, so the next box starts from the panel alone.
  if(document.querySelector('.svctx')) document.body.dispatchEvent(new MouseEvent('mousedown',{bubbles:true}));
  await flush();
};
const theBoxesOver=async(server,panel)=>{
  rail.theCurrentPlaceIs('machineview',server); await flush();
  pane.thePanelIsOpenedOver(server, panel); await flush();
  await tell('servers:what-is-observed',{subject:server}); await flush();
  const wrap=panelOf(server);
  check(wrap && wrap.getBoundingClientRect().width>0 && wrap.getBoundingClientRect().height>0,'the '+panel+' panel is not on screen over '+server);
  at=wrap?theMiddleOf(wrap):[0,0];
  const under=document.elementFromPoint(at[0], at[1]);
  check(wrap && wrap.contains(under),'the '+panel+' panel is not on top of '+server+'\'s screen');
  await theMenuOf(server);
  document.querySelector('.svctx [data-act="remove"]')?.click();
  await theBoxIsOnTop(server,'Remove');
  await theMenuOf(server);
  document.querySelector('.svctx [data-act="edit"]')?.click();
  await theBoxIsOnTop(server,'Edit');
  await tell('servers:the-changed-identity-warning',{subject:server});
  await theBoxIsOnTop(server,'Server identity changed');
  screen.the_stop_and_clear_ending={server,which:screen.the_boxes_a_connection_raises.stop_and_clear[0].which};
  await theMenuOf(server);
  screen.the_stop_and_clear_ending=null;
  await theBoxIsOnTop(server,'Stop and clear ending');
  check(panelOf(server) && !panelOf(server).hidden,'the '+panel+' panel went away under the boxes');
};
try {
 const drawn=(parts)=>{const one=document.createElement('p');one.textContent='Panel';parts.body.appendChild(one);};
 pane.aPanelIsRegistered({panel:pane.RENDER_SETTINGS,itIsDrawn:drawn,itIsShowingAgain(){},theFileItIsOn(){return [1,2];},itIsShownOn(){}});
 pane.aPanelIsRegistered({panel:pane.THE_DEPENDENCY_CHECK,itIsDrawn:drawn,itIsShowingAgain(){},theFileItIsOn(){return [1,3];},itIsShownOn(){}});
 await flush();
 await theBoxesOver('server-one', pane.RENDER_SETTINGS);
 await theBoxesOver('server-two', pane.THE_DEPENDENCY_CHECK);
 // **Add a server lives on the All servers screen only**, where the pane and
 // its panel are not drawn.
 rail.theCurrentPlaceIs('serversview'); await flush();
 const add=[...document.querySelectorAll('#serversview button')].find(one=>one.textContent===ADD_VALUE.heading||one.textContent===screen.add_a_server);
 check(add,'All servers has no Add a server');
 add?.click(); await flush();
 check(!panelOf('server-two') || panelOf('server-two').getBoundingClientRect().width===0,'a panel is on screen beside All servers');
 at=null;
 await theBoxIsOnTop('All servers','Add a server');
 out.textContent=failures.length?failures.join(' | '):'PASS';
} catch(e) { out.textContent=e.stack; }
"#;

    /// The walk for the eighteenth notes, note 1: the removal through the
    /// rail's menu with the pane open on the server, then through the All
    /// servers screen's menu with another server's panel waiting.
    const REMOVAL_WALK: &str = r#"
const screen = SCREEN_VALUE;
const base = structuredClone(screen.rows[0]);
base.the_menu='disconnected'; base.the_connection={label:'Connect',act:'connect',unavailable:false};
screen.rows=[base, {...structuredClone(base),server:'server-two',name:'Second'}];
const calls=[];
window.__TAURI__={core:{invoke:async (act,args={})=>{
  calls.push([act,args]);
  if(act==='the_rail') return {opens_at:'queueview',the_rails_own_name:'Navigation',the_servers_heading:'Servers',places:[],servers:screen.rows.map(r=>({server:r.server,name:r.name,away:false}))};
  if(act==='the_pane_layout') return {seams:[]};
  if(act==='the_all_servers_screen') return structuredClone(screen);
  if(act==='the_servers_own_screen') { const r=screen.rows.find(r=>r.server===args.server); return {the_head:r?{...r,the_rental_line:{}}:null,the_server_is_gone:!r,the_tabs:[{tab:'files',name:'Files'},{tab:'setupview',name:'Set up'}],the_panels:[],instead:{lead:'Removed',values:[]}}; }
  if(act==='the_removal_of') return {heading:{lead:'Remove',values:[args.server]},said:'Confirmation',cancel:'Cancel',remove:'Remove'};
  // **The removal lands before its answer is posted**, as the application's does.
  if(act==='a_server_is_removed') { screen.rows=screen.rows.filter(r=>r.server!==args.server); return null; }
  return {};
}}};
const out=document.getElementById('out');
out.textContent='START';
const failures=[];
const check=(ok,why)=>{if(!ok) failures.push(why);};
const flush=async()=>{for(let i=0;i<12;i++) await Promise.resolve();};
const head=()=>document.querySelector('#machineview .mhead');
const instead=()=>document.querySelector('#machineview > .suscroll');
const panelOf=(server)=>document.querySelector('.sheetwrap[data-server="'+server+'"]');
const tabShown=()=>[...document.querySelectorAll('#machineview [data-tab-host]')].some(one=>!one.hidden);
const removeIn=async()=>{
  document.querySelector('.svctx [data-act="remove"]')?.click(); await flush();
  const box=document.querySelector('[role=dialog]');
  [...(box?.querySelectorAll('button')||[])].find(one=>one.textContent==='Remove')?.click(); await flush();
  await tell('servers:the-answer',{said:{what:'it-was-saved'}}); await flush();
};
try {
 pane.aPanelIsRegistered({panel:pane.RENDER_SETTINGS,itIsDrawn(parts){const one=document.createElement('p');one.textContent='Settings';parts.body.appendChild(one);},itIsShowingAgain(){},theFileItIsOn(){return [1,2];},itIsShownOn(){}});
 await flush();

 // **The rail's menu, with the pane open on the server it removes.**
 rail.theCurrentPlaceIs('machineview','server-one'); await flush();
 pane.thePanelIsOpenedOver('server-one', pane.RENDER_SETTINGS); await flush();
 check(head() && !head().hidden && panelOf('server-one') && !panelOf('server-one').hidden,'the pane did not open on server-one with its panel');
 document.querySelector('.rail [data-server="server-one"]').dispatchEvent(new MouseEvent('contextmenu',{bubbles:true,cancelable:true,clientX:5,clientY:40})); await flush();
 await removeIn();
 check(calls.some(c=>c[0]==='a_server_is_removed' && c[1].server==='server-one'),'the rail removal was not asked for server-one');
 check(head() && head().hidden,'the removed server kept its head');
 check(!tabShown(),'the removed server kept its tab');
 check(instead() && !instead().hidden && instead().textContent==='Removed','the pane did not say the server was removed');
 check(!panelOf('server-one'),'the removed server kept its panel');

 // **The All servers screen's menu, with another server's panel waiting.**
 rail.theCurrentPlaceIs('machineview','server-two'); await flush();
 pane.thePanelIsOpenedOver('server-two', pane.RENDER_SETTINGS); await flush();
 check(panelOf('server-two'),'the second server has no panel');
 rail.theCurrentPlaceIs('serversview'); await flush();
 const listRow=document.querySelector('#serversview [data-server="server-two"]');
 check(listRow,'the All servers screen has no row for server-two');
 listRow?.dispatchEvent(new MouseEvent('contextmenu',{bubbles:true,cancelable:true,clientX:300,clientY:120})); await flush();
 await removeIn();
 check(calls.some(c=>c[0]==='a_server_is_removed' && c[1].server==='server-two'),'the All servers removal was not asked for server-two');
 check(!panelOf('server-two'),'the server removed from All servers kept its panel');
 check(rail.theCurrentPlace()==='serversview','the removal moved the user off All servers');
 check(document.querySelector('#machineview .mtop h1')?.textContent==='Second' && instead()?.hidden,'the pane drew over another screen');
 out.textContent=failures.length?failures.join(' | '):'PASS';
} catch(e) { out.textContent=e.stack; }
"#;
}

//! **The rail and the main pane — the thing every screen sits in.**
//!
//! `Preparation/Design/Whole app.md` *Where things live* is what this keeps,
//! and it is Creator-stated and settled at Design: **the rail is navigation
//! only**, it holds *"the servers, then the places that belong to no server —
//! Queue, History, Transfers — then Keys and Settings"*, **the rail cannot be
//! the server list**, **a server is a place you go into**, and **the queue and
//! history are two screens**. The speed transfers are actually running at sits
//! in the title bar, settled at Design on 2026-08-24.
//!
//! This file is the application's half. `web/the-rail.js` is the page's half
//! and **its head carries the mounting contract**, which is the thing every
//! screen after this one has to read. `web/the-drawing.css` carries the drawing.
//!
//! ## What this piece is not
//!
//! **It draws none of the 208 states.** The rail is inside every one of them and
//! is its own in none. What it makes reachable is all seven of the drawing's own
//! destinations, which is what a screen needs before it can be drawn at all.
//!
//! **It states no failure of its own, and that is a decision rather than an
//! omission.** The rail reads the server rows and writes nothing anywhere, so
//! `Desktop app and delivery` L8a's *unread* can never become *gone* here — the
//! harm F-6 repaired in `panes.rs` needs a write and there is none. A read of
//! the server rows that would not answer is the **All servers** screen's to
//! state, that being the screen the user goes to when they want to know about
//! their servers; the rail's own answer is that it draws the places it always
//! draws, so the way to that screen is never taken away.
//!
//! **Nothing here composes a failure sentence**, so
//! [`crate::presentation::the_two_hooks::WhoseSentenceItIs`] is not reached by
//! any path in this piece.
//!
//! ## How a clause marker is written in this crate
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document, at the site or here.
//!
//! **This piece defines no clause marker of its own and its specification
//! carries none.** The runner's ruling of 2026-09-12 is how a screen piece
//! cites what it keeps instead: **the Design document and the heading it draws,
//! and the mockup state names it draws.** This piece draws no state, so what it
//! cites is the heading alone — `Design/Whole app § Where things live` and
//! `Design/Whole app § The window` — and it names the seven destinations it
//! makes reachable, which is the thing the next piece counts against.
//!
//! **The clauses it keeps that belong to somebody**: P11's rule that a drawn
//! narrowing step writes nothing, kept by this piece writing nothing at all;
//! H34's closed request and response types; A6 and X9's two routes; A1, a
//! feature module reaching the operating system only through a named service —
//! the rail reaches none.

use std::sync::Mutex;

use tauri::{AppHandle, Runtime, State};

use crate::downloading::TheRunsTransfers;
use crate::events::{Event, Posted, Said, Subject, TheOneQueue};
use crate::record::server_rows;
use crate::start::Started;
use crate::surface::the_queue::{WhatAServerIsDoing, WhatTheServersAreDoing};
use crate::typography::{Prose, Verbatim};

// ── The seven destinations ──────────────────────────────────────────────────

/// **One place the rail can put the user, and the set is closed at seven.**
///
/// **The names are the mockup's own view identifiers and that is deliberate.**
/// `Preparation/Design/Mockup/Smoozy.html`'s `setState` toggles exactly seven
/// sibling subtrees under `.app` — `serversview`, `machineview`, `queueview`,
/// `historyview`, `transfersview`, `keysview` and `stgview` — and the drawing's
/// own stylesheet declares rules against five of those identifiers by name. **So
/// the place a screen registers under and the identifier its host element
/// carries are one string rather than two**, the drawing's rules apply to the
/// application's own elements unchanged, and a test below reads the mockup and
/// fails if the set has moved.
pub struct Place {
    /// The mockup's own view identifier, which is also the host element's.
    pub place: &'static str,
    /// What the row is called. **Absent for the server place**, whose rows are
    /// named after the servers themselves.
    pub name: Option<&'static str>,
    /// The Go menu item that arrives here, so a menu choice and a rail press
    /// reach the same place. Absent where no menu item goes here.
    pub going: Option<&'static str>,
    /// Which of the rail's three sections the row sits in — the servers, then
    /// the places that belong to no server, then Keys and Settings. **Design
    /// fixes the order and the grouping**; the number is only how the page is
    /// told where the two dividers fall.
    pub section: u8,
}

/// **The seven, in Design's own order.**
///
/// *"Down the left is a rail, and it is navigation only. It holds, in order:
/// the servers, then the places that belong to no server — Queue, History,
/// Transfers — then Keys and Settings."*
///
/// **`machineview` is a place with no fixed row of its own.** Design: *"A server
/// is a place you go into"*, and the rail's server rows are what go there, one
/// per server. The mockup says the same in its script: `machineview` is the one
/// view shown when none of the other six is.
pub const THE_PLACES: [Place; 7] = [
    Place {
        place: "serversview",
        name: Some("All servers"),
        going: Some("go.servers"),
        section: 0,
    },
    Place {
        place: "machineview",
        name: None,
        going: None,
        section: 0,
    },
    Place {
        place: "queueview",
        name: Some("Queue"),
        going: Some("go.queue"),
        section: 1,
    },
    Place {
        place: "historyview",
        name: Some("History"),
        going: Some("go.history"),
        section: 1,
    },
    Place {
        place: "transfersview",
        name: Some("Transfers"),
        going: Some("go.transfers"),
        section: 1,
    },
    Place {
        place: "keysview",
        name: Some("Keys"),
        going: Some("go.keys"),
        section: 2,
    },
    Place {
        place: "stgview",
        name: Some("Settings"),
        going: Some("smoozy.settings"),
        section: 2,
    },
];

/// **Where the shell opens where at least one server has been added.**
///
/// The mockup's state list carries `<option value="q" selected>Queue</option>`
/// and the rail's own markup carries `aria-current="true"` on the Queue row.
/// **Nothing durable remembers which place the user was last in** — `State,
/// identity and recovery` §1.1's inventory holds no such value and this piece
/// adds none — so the opening place is a decision rather than a reading, and
/// the drawing already made it. **Servers that are all disconnected still open
/// here.**
pub const WHERE_THE_SHELL_OPENS: &str = "queueview";

/// **Where the shell opens where no server has been added** — `Design/Whole
/// app § Where things live`: *"Smoozy opens on All servers where no server has
/// been added, and on Queue otherwise."* With no server there is nothing a
/// queue could hold, and adding one is the first thing to do. Added 2026-10-03,
/// the Creator's note 1 on the test copy.
///
/// **A record that would not answer reads as one with no servers**, as the
/// rail's own list already does, so it opens here too — the screen that says
/// what is wrong with the record.
pub const WHERE_THE_SHELL_OPENS_WITH_NO_SERVERS: &str = "serversview";

/// **The eight badges, and their one home is the mockup.**
///
/// `Preparation/Design/Whole app.md` *The server badge*: the eight sit 45
/// degrees apart around the whole wheel and are the same in light and dark. The
/// drawing declares `.b1` to `.b8` and the token block declares `--b1` to
/// `--b8`; this is the closed set the stored colour is checked against, and a
/// test below reads the drawing and fails if the two have drifted.
///
/// **A stored colour that is not one of the eight draws no badge at all.** The
/// badge means *which server* and nothing else, so a badge this file chose at
/// drawing time would be a mark that could differ between two runs — which is
/// the one thing the badge may never do. Choosing one is the Add dialog's
/// (`Design/Servers`), and it is not built.
pub const THE_EIGHT_BADGES: [&str; 8] = ["b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8"];

/// **The kind, declared beside the service that posts it.**
///
/// `web/the-rail.js` names the same string.
pub const THE_SPEED_TRANSFERS_ARE_RUNNING_AT: &str = "the-rail:the-speed";

// ── What the page is given ──────────────────────────────────────────────────

/// One fixed row of the rail, as the page draws it.
#[derive(serde::Serialize)]
pub struct APlaceInTheRail {
    /// The mockup's view identifier. Neither prose nor a value the user reads:
    /// it is the name the page and this file agree on.
    pub place: &'static str,
    /// **Prose, through the typography point** — A6 and X1.
    pub name: Prose,
    /// The Go menu item that arrives here, or nothing.
    pub going: Option<&'static str>,
    pub section: u8,
    /// **The figure beside the row**, where this file has one. A count is a
    /// value and travels by the verbatim route (X9), never joined into the
    /// name (X7). Every other row's figure belongs to the screen behind it and
    /// arrives from there.
    pub count: Option<Verbatim>,
    /// **The Queue row's own mark, and nothing on every other row** —
    /// [`TheQueuesMark`].
    pub mark: Option<TheQueuesMark>,
}

/// **The rail's *Queue* says a job in the queue needs your answer** —
/// `Design/Whole app § The server badge`, the paragraph added 2026-10-31 for
/// the Creator's twenty-fourth notes, note 2: *"Queue button in the left pane
/// should visually indicate that something needs the user's attention."*
///
/// One small still dot just after the word, in the amber *Needs your answer*
/// is written in — the drawing's `.qask`, states `rail-ask` and
/// `rail-asknarrow` with it, `rail-noask` and `rail-noasknarrow` without.
/// **No count**: the queue says how many. **On while any queue job's status
/// reads *Needs your answer*** ([`a_job_in_the_queue_needs_your_answer`]),
/// gone the moment none does.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheQueuesMark {
    /// Whether the dot shows.
    pub on: bool,
    /// **The row's hover name while it shows** — *Queue — needs your answer*.
    /// Prose; the row's own name stands while it does not.
    pub hover: Prose,
}

/// **The queue row's status this mark answers to**, the word the queue
/// writes on a held job's row (`src/surface/the_queue.rs`, its
/// `WAITING_ON_YOU`). A test holds the two together.
const NEEDS_YOUR_ANSWER: &str = "Needs your answer";

/// **The Queue row's hover name while the dot shows** — Design's own words.
const QUEUE_NEEDS_YOUR_ANSWER: &str = "Queue — needs your answer";

/// **Whether any job in the queue reads *Needs your answer*.**
///
/// **Asked of the Queue screen's own composition**
/// ([`crate::surface::the_queue::what_is_held`]) — the period-held reading
/// and the local record, with nothing new asked of any server — so the dot
/// and the queue's rows cannot disagree: a job held for the driver reads
/// *Waiting for driver update* and does not set it; a stale reading reads
/// *Not updating* and does not; a held job Smoozy is cancelling reads
/// *Cancelling* and does not.
///
/// **It can arrive late, and Design says so**: the reading repeats only while
/// Queue, the server list or a server's own page is on screen (P10d).
pub fn a_job_in_the_queue_needs_your_answer(
    holds: &crate::servers::the_machines_work::WhatThisRunHolds,
    record: Option<&crate::record::LocalRecord>,
) -> bool {
    crate::surface::the_queue::what_is_held(holds, record, None)
        .lines
        .iter()
        .flat_map(|line| &line.rows)
        .any(|row| row.state == Some(NEEDS_YOUR_ANSWER))
}

/// One server's row in the rail.
///
/// **Two fields, and X7 is why.** The name is the server's own and the user can
/// copy it, so it travels verbatim and is never joined into a sentence. Nothing
/// in this struct is prose.
#[derive(serde::Serialize)]
pub struct AServerInTheRail {
    /// The row's identity, which is what a press hands back. A value.
    pub server: Verbatim,
    /// **The name, or `username@host` where the server has none.**
    /// `Preparation/Design/Servers.md` states the fallback and the mockup draws
    /// it in the rail — `root@188.42.19.7`. **The port is added where it is the
    /// only thing telling two unnamed servers apart** (`Specs/Servers`), by the
    /// one naming the list, the head, `Keys` and the removal box take too —
    /// `crate::surface::servers::what_a_server_is_called_among` (step 10a round
    /// 1, fix-here item 21).
    pub name: Verbatim,
    /// One of [`THE_EIGHT_BADGES`], or nothing where the row names no colour or
    /// names one that is not one of the eight.
    pub badge: Option<&'static str>,
    /// **`Design/Whole app § How state is shown`: *"In the rail, where there is
    /// no room for a control, a server that is not connected reads dim … the
    /// whole row dims, badge included."*** True where the server's connect
    /// control would read *Connect* — `Servers`' own presentation of what is
    /// true of it, taken from the one place the servers screen takes it.
    pub away: bool,
    /// **The row's one dot, which says what the server is doing** —
    /// `Design/Whole app § The server badge`, changed 2026-10-04 on the
    /// Creator's note 19: green while connected and not rendering, amber and
    /// pulsing while a job renders on it, **none while it is not connected**.
    /// Nothing where [`AServerInTheRail::away`] is true. **Amber, dimmed and
    /// still while it reconnects** (the Creator's seventh notes, note 10,
    /// 2026-10-10). See [`the_dot`].
    pub dot: Option<TheDot>,
}

/// **The rail's dot, the drawing's `.rdot`** — and `.rdot.run` while a job
/// renders, which is the pulse a rendering job already has.
///
/// **And `.rdot.trying` while it reconnects** — the connect control's own
/// reconnecting dot, still, with the server list's own *Reconnecting…*
/// (`Design/Whole app` line 179, changed 2026-10-10 on the Creator's seventh
/// notes, note 10: it said the dot said only idle or rendering, so a server
/// Smoozy was repairing or that was restarting drew green and said
/// *Connected*, which was false).
///
/// **It says connected, rendering or reconnecting, and that is Design's own
/// limit**: *"its dot says idle, rendering, or reconnecting … the server list
/// still says it in words"*. So a server that is setting up, or whose jobs all
/// wait, or whose reading is not updating, draws green — the drawing draws
/// *Setting up* green, and the build runner ruled it built as drawn
/// (2026-10-04).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheDot {
    /// True while a job renders there: the drawing's `.rdot.run`.
    pub rendering: bool,
    /// **True while the server reconnects** — `Servers`' *repairing* or
    /// *restarting*: the drawing's `.rdot.trying`. Never together with
    /// [`TheDot::rendering`]: a server reconnecting says so whatever its last
    /// reading had rendering, as the server list's work column does.
    pub reconnecting: bool,
    /// **What the dot says to anyone who cannot see it.** Prose, and the two
    /// words are ones the app already shows — the connect control's
    /// *Connected* and the work column's *Rendering* — so a row read aloud
    /// carries the fact the colour carries, as the open-panel mark's row did
    /// (the build runner's ruling 2 of 2026-09-13).
    pub said: Prose,
}

/// Everything the page needs to draw the rail, in one answer.
#[derive(serde::Serialize)]
pub struct TheRail {
    /// What the rail is called, for anyone who cannot see it. Prose.
    pub the_rails_own_name: Prose,
    /// The heading over the servers. Prose.
    pub the_servers_heading: Prose,
    /// Where the shell opens.
    pub opens_at: &'static str,
    pub places: Vec<APlaceInTheRail>,
    /// **In the order the record holds them**, which is the order servers were
    /// added. Empty where there are none and empty where the record would not
    /// answer, and the rail draws the same thing either way — see this file's
    /// head for why that is right here and wrong in `panes.rs`.
    pub servers: Vec<AServerInTheRail>,
}

// ── The command ─────────────────────────────────────────────────────────────

/// **The rail's own words, its seven places, and this user's servers.**
///
/// **A command rather than an event, and P6 is not being bent.** P6 is about a
/// module's *results*; this is the page asking for the facts it needs before it
/// can draw anything, which is the shape `chrome_text` and `the_pane_layout`
/// already have. **A page that loads again asks again** — events are volatile
/// and nothing is replayed (P8).
///
/// **H34: a closed request and response type**, nothing coming in, and nothing
/// beneath the application-service surface going out.
#[tauri::command]
pub fn the_rail<R: Runtime>(app: AppHandle<R>, started: State<'_, Mutex<Started>>) -> TheRail {
    // **Which servers are connected is the run's, and it is asked where the
    // servers screen asks it** — the register of periods, read with no lock of
    // the application's held (step 10b, the three small things, S2).
    let periods = crate::surface::servers::the_periods_of(&app);
    let Ok(started) = started.lock() else {
        return the_rail_from(&[], periods.as_deref(), &WhatTheServersAreDoing::new(), false);
    };
    let rows = started
        .record
        .as_ref()
        .map(|record| server_rows::all(record.connection()))
        .and_then(Result::ok)
        .unwrap_or_default();
    // **What each server is doing is the work column's own reading**, composed
    // here under the record's lock exactly as the servers screen composes it
    // — the period-held queue reading less every job that has left the queue —
    // so the dot and the list cannot disagree about what renders.
    let doing = periods
        .as_deref()
        .map(|periods| {
            crate::surface::the_queue::what_the_servers_are_doing(
                periods.the_machines_work(),
                started.record.as_ref(),
            )
        })
        .unwrap_or_default();
    // **Whether the Queue's dot shows**, from the same reading under the same
    // lock — the Queue screen's own rows (the twenty-fourth notes, note 2).
    let asking = periods.as_deref().is_some_and(|periods| {
        a_job_in_the_queue_needs_your_answer(periods.the_machines_work(), started.record.as_ref())
    });
    drop(started);
    the_rail_from(&rows, periods.as_deref(), &doing, asking)
}

/// The answer itself, with the rows and the run's periods handed in, **so the
/// whole of it can be tested** — the shell's own managed state has no
/// constructor outside a running application.
///
/// **`periods` is the run's register, or nothing where the caller has none**,
/// and a server nothing can be asked about reads as one no period stands for —
/// the same answer the servers screen gives through
/// [`crate::surface::servers::what_is_true_of`]. **`doing` is what each server
/// is doing**, as [`crate::surface::the_queue::what_the_servers_are_doing`]
/// composes it for the work column. **`asking` is whether a job in the queue
/// reads *Needs your answer*** ([`a_job_in_the_queue_needs_your_answer`]),
/// which sets the Queue row's dot.
pub fn the_rail_from(
    rows: &[smoozy_servers::store::ServerRow],
    periods: Option<&crate::servers::a_period::ThePeriodsThisRun>,
    doing: &WhatTheServersAreDoing,
    asking: bool,
) -> TheRail {
    let servers: Vec<AServerInTheRail> = rows
        .iter()
        .map(|row| {
            let of = crate::surface::servers::what_is_true_of(periods, &row.local_server_identifier);
            let away = it_is_not_connected(of);
            let connecting = crate::surface::servers::THE_USERS_CONNECTS
                .is_underway(&row.local_server_identifier, of);
            let mut dot = the_dot(
                smoozy_servers::condition::presentation(of),
                doing.get(&row.local_server_identifier),
            );
            if connecting {
                if let Some(dot) = &mut dot {
                    dot.said = Prose::new(crate::surface::set_up::words::CONNECTING);
                }
            }
            AServerInTheRail {
                server: Verbatim::new(row.local_server_identifier.clone()),
                name: Verbatim::new(crate::surface::servers::what_a_server_is_called_among(row, rows)),
                badge: the_badge(row.colour.as_deref()),
                away,
                dot,
            }
        })
        .collect();

    TheRail {
        the_rails_own_name: Prose::new("Navigation"),
        the_servers_heading: Prose::new("Servers"),
        opens_at: if servers.is_empty() {
            WHERE_THE_SHELL_OPENS_WITH_NO_SERVERS
        } else {
            WHERE_THE_SHELL_OPENS
        },
        places: THE_PLACES
            .iter()
            .filter_map(|place| {
                let name = place.name?;
                Some(APlaceInTheRail {
                    place: place.place,
                    name: Prose::new(name),
                    going: place.going,
                    section: place.section,
                    // **The one figure this file holds is how many servers there
                    // are**, that being the count of the rows it just drew.
                    // Every other row's figure is the screen's behind it.
                    count: (place.place == "serversview")
                        .then(|| Verbatim::new(servers.len().to_string())),
                    // **The Queue row carries the mark, on or off**, so the
                    // page draws the drawing's `.qask` there in either state
                    // and hides it while no job needs an answer.
                    mark: (place.place == "queueview").then(|| TheQueuesMark {
                        on: asking,
                        hover: Prose::new(QUEUE_NEEDS_YOUR_ANSWER),
                    }),
                })
            })
            .collect(),
        servers,
    }
}

/// **The name, or `username@host` where the server has none.**
///
/// **Composed out of two values and never out of prose**, so X7's rule that the
/// typography point never touches a host or a username is kept by there being
/// no prose on this path at all.
#[cfg(test)]
fn what_a_server_is_called(row: &smoozy_servers::store::ServerRow) -> String {
    crate::surface::servers::what_a_server_is_called_among(row, std::slice::from_ref(row))
}

/// **Whether a server reads dim in the rail.**
///
/// **`Servers`' own presentation decides, and only *disconnected* dims.** A
/// server Smoozy is repairing or that is restarting still has its connected
/// period standing — its connect control is not the hollow *Connect* — so its
/// row stays as it is. **What it is handed is the run's own reading of the
/// server**, asked of the register of periods exactly as the servers screen
/// asks it, so a row reads lit while its server's connect control reads
/// *Connected*. **The page reads the rail again whenever `Servers` publishes a
/// server's observed value**, which is how a row follows a connect or a
/// disconnect without the user going anywhere (`web/the-rail.js`).
/// *(Corrected 2026-09-18. This said nothing opens a period and that the rail
/// would follow once one was driven; periods were driven from step 10b, and the
/// rail went on handing every row the reading of a server with no period, so a
/// connected server was drawn dim.)*
fn it_is_not_connected(of: smoozy_servers::condition::WhatIsTrueOfTheServer) -> bool {
    matches!(
        smoozy_servers::condition::presentation(of),
        smoozy_servers::condition::Presentation::Disconnected
    )
}

/// **The row's dot, from `Servers`' own presentation of the server and what it
/// is doing** — `Design/Whole app § The server badge` (note 19, 2026-10-04;
/// note 10 of the seventh notes, 2026-10-10).
///
/// - **None where the row is dim**: a server that is not connected has no dot,
///   whatever a reading from before the connection went still says.
/// - **The reconnecting dot while it is repaired or restarts**, said as the
///   server list's own *Reconnecting…* — whatever the last reading had
///   rendering, since nothing is known to be rendering while the connection
///   is down.
/// - **Amber and pulsing where the reading has a job rendering** — the work
///   column's own *Rendering* ([`WhatAServerIsDoing::Rendering`]), so the dot
///   never says rendering from the local record alone.
/// - **Green otherwise**: idle, setting up, jobs waiting, a reading not yet
///   taken or not updating. The dot has two colours and those are the cases
///   that are connected and not rendering.
fn the_dot(
    how: smoozy_servers::condition::Presentation,
    doing: Option<&WhatAServerIsDoing>,
) -> Option<TheDot> {
    use smoozy_servers::condition::Presentation;
    match how {
        Presentation::Disconnected => return None,
        Presentation::Repairing | Presentation::Restarting => {
            return Some(TheDot {
                rendering: false,
                reconnecting: true,
                said: Prose::new(
                    crate::presentation::the_repairing_server::WhatTheColumnSays::Reconnecting.the_word(),
                ),
            })
        }
        Presentation::Connected => {}
    }
    let rendering = matches!(doing, Some(WhatAServerIsDoing::Rendering { .. }));
    Some(TheDot {
        rendering,
        reconnecting: false,
        said: Prose::new(if rendering {
            crate::presentation::the_repairing_server::WhatTheColumnSays::Rendering(Verbatim::new("")).the_word()
        } else {
            crate::surface::servers::words::CONNECTED
        }),
    })
}

/// The stored colour, where it is one of the eight, and nothing otherwise.
fn the_badge(colour: Option<&str>) -> Option<&'static str> {
    let colour = colour?;
    THE_EIGHT_BADGES
        .iter()
        .find(|badge| **badge == colour)
        .copied()
}

/// **The speed transfers are actually running at, published to the title bar.**
///
/// `Preparation/Design/Whole app.md` *Where things live*: *"The speed transfers
/// are actually running at sits in the title bar, so it is readable from every
/// screen rather than only on Transfers, and it is there only while something is
/// moving."* The reading itself is R15a's, taken at the one desktop-side
/// admission point by
/// [`crate::surface::the_underway_shape::the_rate_and_the_remaining_time`].
///
/// ~~**This is the door, and nothing calls it yet.**~~ **Corrected 2026-10-03,
/// step 10c: it was still called by nothing but its test long after files began
/// to move, so the title bar never showed a speed.** It is now called through
/// [`the_speed_now`], from the moves event
/// (`crate::surface::transfers::the_moves_are_posted`).
///
/// **The reading is one value and no prose.** A figure, its unit and the
/// direction mark are what the drawing puts in the bar and none of them is a
/// word, so the whole of it travels by the verbatim route (X9) and the
/// typography point never sees it (X7). **An empty value takes the reading
/// away**, which is what the end of a transfer looks like.
pub fn the_speed_is(queue: &TheOneQueue, reading: Verbatim) -> Result<Posted, ()> {
    queue.read_then_post(
        || Ok::<Verbatim, ()>(reading),
        |reading| {
            Event::about(Subject::TheRun, THE_SPEED_TRANSFERS_ARE_RUNNING_AT)
                .saying("theSpeed", Said::Value(reading.clone()))
        },
    )
}

/// **The title bar is told the speed as it stands now.**
///
/// **The reading is the pair Transfers and Settings already draw**
/// ([`crate::surface::transfers::the_live_readings`]), taken at the run's own
/// admission point and never worked out a second time here. It is written the
/// way Transfers writes it, *↑ 12.4 MB/s · ↓ 4.1 MB/s*, with only the
/// directions that have a channel. **Where neither has one the reading is
/// empty**, and an empty reading takes the bar's figure away.
pub fn the_speed_now(queue: &TheOneQueue, the_run: &TheRunsTransfers) -> Result<Posted, ()> {
    let (up, down) = crate::surface::transfers::the_live_readings(the_run);
    the_speed_is(queue, the_title_bar_reads(up.as_deref(), down.as_deref()))
}

/// **Upload first, then download, a dot between** — the order and the marks
/// `Design/Transfers` draws its own reading with.
fn the_title_bar_reads(up: Option<&str>, down: Option<&str>) -> Verbatim {
    let mut said = Vec::new();
    if let Some(up) = up {
        said.push(format!("↑ {up}"));
    }
    if let Some(down) = down {
        said.push(format!("↓ {down}"));
    }
    Verbatim::new(said.join(" · "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::sync::Arc;

    fn the_page_module() -> String {
        std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("web/the-rail.js"),
        )
        .expect("the page module is readable")
    }

    fn the_drawing() -> String {
        std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("web/the-drawing.css"),
        )
        .expect("the drawing is readable")
    }

    fn the_mockup() -> String {
        std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../Preparation/Design/Mockup/Smoozy.html"),
        )
        .expect("the mockup is readable")
    }

    /// **The seven places are the seven the drawing has, and no eighth.**
    ///
    /// **The mockup's own script is the authority** — `setState` toggles exactly
    /// seven sibling subtrees under `.app` and the rail lists exactly those
    /// seven, with the server one repeated per server. **This is the check that
    /// stops the set drifting**: a screen piece that invents a destination the
    /// drawing does not have would draw a rail row nobody designed, and a screen
    /// piece that misses one leaves a place unreachable with nothing saying so.
    #[test]
    fn the_places_are_the_seven_the_drawing_toggles() {
        let mockup = the_mockup();
        for place in THE_PLACES {
            assert!(
                mockup.contains(&format!("id=\"{}\"", place.place)),
                "{} is not a view the drawing has. **The seven destinations are \
                 the mockup's own** and the place a screen registers under is \
                 that view's own identifier",
                place.place
            );
            assert!(
                mockup.contains(&format!("{}.hidden", place.place)),
                "{} is not one of the subtrees the mockup's setState toggles",
                place.place
            );
        }
        // And no eighth: the mockup's own list of what it toggles is seven long.
        let toggled = ["serversview", "queueview", "historyview", "transfersview", "keysview", "stgview", "machineview"];
        assert_eq!(toggled.len(), THE_PLACES.len());
        for place in toggled {
            assert!(
                THE_PLACES.iter().any(|declared| declared.place == place),
                "the drawing toggles {place} and this file does not name it"
            );
        }
    }

    /// **The rail's order is Design's, and Design's is Creator-stated.**
    ///
    /// *"the servers, then the places that belong to no server — Queue, History,
    /// Transfers — then Keys and Settings."* **A rail in another order is a
    /// different rail** and nothing else in the application would notice.
    #[test]
    fn the_rail_is_in_designs_order_and_in_three_sections() {
        let drawn: Vec<&str> = THE_PLACES
            .iter()
            .filter(|place| place.name.is_some())
            .map(|place| place.place)
            .collect();
        assert_eq!(
            drawn,
            vec![
                "serversview",
                "queueview",
                "historyview",
                "transfersview",
                "keysview",
                "stgview"
            ]
        );
        let sections: Vec<u8> = THE_PLACES.iter().map(|place| place.section).collect();
        assert_eq!(sections, vec![0, 0, 1, 1, 1, 2, 2], "the three sections are Design's grouping");
    }

    /// **Every place a menu item goes to is a place this file has, and every
    /// going it names is a menu item that exists.**
    ///
    /// `src/menus.rs` holds ten items whose act is a going and its own comment
    /// says *"Every one of these is a screen or a panel the interface opens, and
    /// none of them exists yet."* **Six of the ten arrive here.** A going spelled
    /// wrong in this file would draw a rail that works and a menu that silently
    /// does nothing, which is the failure this line catches.
    #[test]
    fn every_going_this_file_names_is_a_menu_item_that_exists() {
        for place in THE_PLACES {
            let Some(going) = place.going else { continue };
            assert!(
                crate::menus::the_item(going).is_some(),
                "{going} is not an item of the four menus, so choosing it would \
                 reach nowhere"
            );
        }
    }

    /// **The eight badges are the drawing's eight.**
    #[test]
    fn the_eight_badges_are_the_drawings() {
        let drawing = the_drawing();
        for badge in THE_EIGHT_BADGES {
            assert!(
                drawing.contains(&format!(".{badge} {{")),
                "the drawing has no .{badge}"
            );
        }
        assert!(
            !drawing.contains(".b9 {"),
            "the drawing has a ninth badge and this file's closed set has not \
             moved with it"
        );
    }

    /// **A colour the eight do not name draws no badge**, and neither does a row
    /// that names none.
    #[test]
    fn a_colour_outside_the_eight_draws_no_badge() {
        assert_eq!(the_badge(Some("b3")), Some("b3"));
        assert_eq!(the_badge(None), None);
        assert_eq!(the_badge(Some("")), None);
        assert_eq!(the_badge(Some("chartreuse")), None);
        // The one shape worth naming: a value out of the record must never
        // reach the page as a class name unchecked.
        assert_eq!(the_badge(Some("b1 b2")), None);
    }

    /// **The page and this file agree about every place name and about the
    /// event kind.**
    ///
    /// Nothing in the shell can hold those two pairs together at compile time —
    /// one side is Rust and the other is a string in a page module — and piece
    /// A's own account says so. **This is that check written out.**
    #[test]
    fn the_page_names_the_same_places_and_the_same_kind() {
        let page = the_page_module();
        for place in THE_PLACES {
            assert!(
                page.contains(&format!("\"{}\"", place.place)),
                "web/the-rail.js does not name the place {}, so a screen \
                 registering under it would never be shown",
                place.place
            );
        }
        assert!(
            page.contains(&format!("\"{THE_SPEED_TRANSFERS_ARE_RUNNING_AT}\"")),
            "the page listens for a different kind than this file posts, so the \
             title bar's reading would never arrive"
        );
    }

    /// **The page adds nothing to `index.html` and reaches the title bar the way
    /// `menus.js` does.**
    #[test]
    fn the_title_bar_is_reached_from_this_modules_own_code() {
        let document = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("web/index.html"),
        )
        .expect("the document is readable");
        for mine in ["class=\"rail\"", "class=\"body\"", "class=\"main\"", "class=\"rate"] {
            assert!(
                !document.contains(mine),
                "index.html carries {mine}, which is this piece's. **Every piece \
                 makes its own elements from its own module**, the title bar \
                 included"
            );
        }
        assert!(
            the_page_module().contains("buttons-right"),
            "the page no longer puts its title-bar readings beside the window's \
             own buttons"
        );
    }

    /// **A server with a name shows it; a server without shows `username@host`.**
    ///
    /// The mockup draws both in the rail — `RunPod 4090` and
    /// `root@188.42.19.7`.
    #[test]
    fn a_server_without_a_name_is_called_by_its_address() {
        let mut row = a_row();
        assert_eq!(what_a_server_is_called(&row), "root@10.0.0.4");
        row.label = Some("  ".to_string());
        assert_eq!(
            what_a_server_is_called(&row),
            "root@10.0.0.4",
            "a name of nothing but spaces is not a name"
        );
        row.label = Some("Overnight box".to_string());
        assert_eq!(what_a_server_is_called(&row), "Overnight box");
    }

    /// **The whole answer, over a record with two servers in it.**
    #[test]
    fn the_rail_carries_the_places_and_the_servers() {
        let mut named = a_row();
        named.local_server_identifier = "two".to_string();
        named.label = Some("Overnight box".to_string());
        named.colour = Some("b7".to_string());

        let rail = the_rail_from(&[a_row(), named], None, &WhatTheServersAreDoing::new(), false);

        // **With servers, connected or not, it opens on Queue** — no period
        // stands for either of these two, so both read as not connected.
        assert_eq!(rail.opens_at, WHERE_THE_SHELL_OPENS);
        assert_eq!(rail.opens_at, "queueview");
        assert!(rail.servers.iter().all(|server| server.away));
        assert_eq!(rail.places.len(), 6, "six rows are drawn; the server place has no row of its own");
        assert_eq!(rail.servers.len(), 2);
        assert_eq!(rail.servers[0].badge, None);
        assert_eq!(rail.servers[1].badge, Some("b7"));
        assert_eq!(rail.servers[1].name.as_str(), "Overnight box");

        let all_servers = rail
            .places
            .iter()
            .find(|place| place.place == "serversview")
            .expect("All servers is drawn");
        assert_eq!(
            all_servers.count.as_ref().map(|count| count.as_str()),
            Some("2"),
            "the figure beside All servers is how many servers there are"
        );
        assert!(
            rail.places
                .iter()
                .filter(|place| place.place != "serversview")
                .all(|place| place.count.is_none()),
            "every other row's figure belongs to the screen behind it"
        );
    }

    /// **A record that will not answer draws the places and no servers**, and it
    /// draws them without saying anything — the head of this file carries why.
    #[test]
    fn no_servers_still_draws_every_place() {
        let rail = the_rail_from(&[], None, &WhatTheServersAreDoing::new(), false);
        assert_eq!(rail.places.len(), 6);
        assert!(rail.servers.is_empty());
        // **`Design/Whole app § Where things live`: with no server added,
        // Smoozy opens on All servers** (the Creator's note 1 on the test copy).
        assert_eq!(rail.opens_at, WHERE_THE_SHELL_OPENS_WITH_NO_SERVERS);
        assert_eq!(rail.opens_at, "serversview");
        assert_eq!(
            rail.places
                .iter()
                .find(|place| place.place == "serversview")
                .and_then(|place| place.count.as_ref())
                .map(|count| count.as_str()),
            Some("0")
        );
    }

    /// **Every word the rail draws is prose and every value is verbatim** — A6
    /// and X9, held on the type rather than on the characters.
    #[test]
    fn the_words_are_prose_and_the_names_are_values() {
        let rail = the_rail_from(&[a_row()], None, &WhatTheServersAreDoing::new(), false);
        let _: &Prose = &rail.the_rails_own_name;
        let _: &Prose = &rail.the_servers_heading;
        for place in &rail.places {
            let _: &Prose = &place.name;
        }
        for server in &rail.servers {
            let _: &Verbatim = &server.name;
            let _: &Verbatim = &server.server;
        }
    }

    /// **The speed reading is one value and the page is told it.**
    #[test]
    fn the_speed_reading_travels_as_a_value() {
        let (queue, held) = a_queue();
        the_speed_is(&queue, Verbatim::new("↑ 12.4 MB/s")).expect("it posts");
        let posted = held.lock().expect("the collector");
        let last = posted.last().expect("one event");
        assert_eq!(last.kind, THE_SPEED_TRANSFERS_ARE_RUNNING_AT);
        assert!(
            matches!(last.said.get("theSpeed"), Some(Said::Value(_))),
            "the reading is a figure and its unit, and no part of it is prose"
        );
    }

    /// The last reading the title bar was sent, as text.
    fn the_bar_reads(held: &Mutex<Vec<crate::events::Delivery>>) -> Option<String> {
        let posted = held.lock().expect("the collector");
        let last = posted
            .iter()
            .rev()
            .find(|delivery| delivery.kind == THE_SPEED_TRANSFERS_ARE_RUNNING_AT)?;
        match last.said.get("theSpeed") {
            Some(Said::Value(reading)) => Some(reading.as_str().to_string()),
            _ => panic!("the reading did not travel as a value"),
        }
    }

    /// **`Design/Whole app § Where things live`: the speed is in the title bar
    /// while something moves, and gone when nothing does.** The moves event is
    /// what every route that moves a file posts, so it is driven here as those
    /// routes drive it, over a run with no channel, then an upload, then both
    /// directions, then neither again.
    #[test]
    fn the_title_bar_is_sent_the_speed_while_something_moves_and_nothing_when_nothing_does() {
        use smoozy_server_protocol::transfers::Direction;
        let (queue, held) = a_queue();
        let the_run = TheRunsTransfers::for_this_run();

        crate::surface::transfers::the_moves_are_posted(&queue, Some(&the_run));
        assert_eq!(
            the_bar_reads(&held).as_deref(),
            Some(""),
            "with nothing moving the bar is sent an empty reading"
        );

        let upload = smoozy_file_browser::pool::THE_DECLARED_ORDER[3].name;
        let mut up = the_run.admit(Direction::Up, &[upload]).expect("an upload channel");
        up.draw_at(12_400_000, std::time::Instant::now());
        crate::surface::transfers::the_moves_are_posted(&queue, Some(&the_run));
        let reads = the_bar_reads(&held).expect("an upload sent the bar nothing");
        assert!(reads.starts_with("↑ ") && reads.ends_with("\u{a0}MB/s"), "{reads}");
        assert!(!reads.contains('↓'), "an upload alone drew a download speed: {reads}");

        let mut down = the_run
            .admit(Direction::Down, &["manual-download"])
            .expect("a download channel");
        down.draw_at(4_100_000, std::time::Instant::now());
        crate::surface::transfers::the_moves_are_posted(&queue, Some(&the_run));
        let reads = the_bar_reads(&held).expect("both directions sent the bar nothing");
        let (first, second) = reads.split_once(" · ").expect("the two readings have a dot between");
        assert!(first.starts_with("↑ ") && second.starts_with("↓ "), "{reads}");

        let _ = up.the_channel_ended(std::time::Instant::now());
        let _ = down.the_channel_ended(std::time::Instant::now());
        crate::surface::transfers::the_moves_are_posted(&queue, Some(&the_run));
        assert_eq!(
            the_bar_reads(&held).as_deref(),
            Some(""),
            "the last channel ended and the bar kept a speed"
        );
    }

    /// **The bar's figure is the pair Transfers draws**, read from where it is
    /// worked out and never a second time here.
    #[test]
    fn the_title_bar_reads_the_pair_transfers_draws() {
        let source = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/surface/the_rail.rs"),
        )
        .expect("this file is readable");
        let working = &source[..source.find("#[cfg(test)]\nmod tests").expect("the tests")];
        assert!(working.contains("the_live_readings("), "the bar does not read Transfers' own pair");
        assert!(!working.contains(".the_reading("), "the bar takes the reading itself — a second working-out");
        let transfers = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/surface/transfers.rs"),
        )
        .expect("Transfers is readable");
        let at = transfers
            .find("pub(crate) fn the_moves_are_posted(")
            .expect("the moves event has its poster");
        let body = &transfers[at..at + transfers[at..].find("\n}\n").expect("it closes")];
        assert!(body.contains("the_rail::the_speed_now("), "the moves event does not tell the title bar");
    }


    /// **R3: a server that is not connected reads dim** — `Design/Whole app §
    /// How state is shown`. A rail built with no register reads every server
    /// as one no period stands for, so every row carries the mark; the reading
    /// itself does not mark a server whose period stands with its main
    /// connection live. *(Corrected 2026-09-18, twice: this said nothing opens
    /// a period, and then that the rail's builder hands every row the reading
    /// of a server with no period — which was the fault the next test holds.)*
    #[test]
    fn a_server_that_is_not_connected_is_marked_dim() {
        let rail = the_rail_from(&[a_row()], None, &WhatTheServersAreDoing::new(), false);
        assert!(rail.servers[0].away, "a server with no period is not marked dim");
        assert!(it_is_not_connected(crate::surface::servers::what_is_true_of_a_server_with_no_period()));
        let connected = smoozy_servers::condition::WhatIsTrueOfTheServer {
            a_period_stands: true,
            the_primary_connection_is_live: true,
            a_run_of_attempts_is_in_flight: false,
            the_restart_mark_stands: false,
        };
        assert!(!it_is_not_connected(connected), "a connected server is dimmed");
    }

    /// **R3 as the running application goes: a connected server's row is lit,
    /// and it dims again when the period ends.**
    ///
    /// The register is the one the application holds, driven the way a
    /// connect drives it — a period begun by the user's connect action, then
    /// `Servers` Q1 step 3's *connected* published through the period's own
    /// publisher — and the rail is asked of it exactly as the servers screen
    /// is. **This failed before 2026-09-18**: the rail never asked the
    /// register, so a server the servers screen drew *Connected* was drawn dim
    /// here (seen pressed in a throwaway copy by step 10b's re-check).
    #[test]
    fn a_connected_server_is_drawn_lit_and_dims_when_its_period_ends() {
        use crate::servers::a_period::{ThePeriodsThisRun, TheRunThroughTheApp};
        use smoozy_servers::acting::WhatServersPublishes;
        use smoozy_servers::condition::ObservedValue;
        use smoozy_servers::driving::APeriod;
        use smoozy_servers::period::{OpeningAct, WhoTookIt};

        let row = a_row();
        let server = row.local_server_identifier.clone();
        let register = ThePeriodsThisRun::over(
            TheRunThroughTheApp::lending_nothing(),
            Arc::new(TheOneQueue::new()),
            Arc::new(crate::notifications::TheNotificationService::on(Box::new(
                NothingIsPosted,
            ))),
        );

        let before = the_rail_from(std::slice::from_ref(&row), Some(&register), &WhatTheServersAreDoing::new(), false);
        assert!(before.servers[0].away, "a server no period stands for is lit");

        let period = APeriod::begun(
            &server,
            OpeningAct::TheUsersConnectAction,
            WhoTookIt::TheUser,
            std::time::SystemTime::UNIX_EPOCH,
        );
        register.a_period_has_begun(&server, &period);
        register
            .where_a_period_publishes()
            .the_observed_value(&server, ObservedValue::Connected);

        let connected = the_rail_from(std::slice::from_ref(&row), Some(&register), &WhatTheServersAreDoing::new(), false);
        assert!(
            !connected.servers[0].away,
            "a server the servers screen draws Connected is drawn dim in the rail"
        );

        register.a_period_has_ended(&server);
        let after = the_rail_from(std::slice::from_ref(&row), Some(&register), &WhatTheServersAreDoing::new(), false);
        assert!(after.servers[0].away, "a server whose period ended is still lit");
    }

    /// **The page reads the rail again when a server's connection moves.**
    ///
    /// The rail is drawn once at load, so a row can only follow a connect if
    /// something asks for the rail again. **It listens for the observed value
    /// under the kind `src/servers/published.rs` declares**, and reads the
    /// rail again — the same route an added or removed server already takes.
    /// This failed before 2026-09-18: nothing in the page re-read the rail on a
    /// connection.
    #[test]
    fn the_page_reads_the_rail_again_when_a_connection_moves() {
        let page = the_page_module();
        let kind = crate::servers::published::WHAT_IS_OBSERVED;
        assert!(page.contains(&format!("\"{kind}\"")), "the page does not name {kind}");
        let at = page
            .find("whenTold(WHAT_IS_OBSERVED")
            .expect("the rail does not hear a server's observed value");
        let heard = &page[at..at + page[at..].find("});").expect("it closes")];
        assert!(heard.contains("theRailIsReadAgain()"), "the rail is not read again");
    }

    struct NothingIsPosted;

    impl crate::notifications::ThePlatform for NothingIsPosted {
        fn the_machinery(&self) -> crate::notifications::TheMachinery {
            crate::notifications::TheMachinery::ItIsInPlace
        }
        fn the_permission(&self) -> crate::notifications::permission::WhatTheSystemSays {
            crate::notifications::permission::WhatTheSystemSays::ItIsGranted
        }
        fn ask_for_permission_now(&self) {}
        fn post(&self, _post: &crate::notifications::APost) {}
        fn take_down(&self, _key: &crate::presentation::the_presentation_key::ThePresentationKey) {}
    }

    /// **The page's own row builder, lifted out of `web/the-rail.js` as it
    /// stands with the dot it draws, and run under Node** with the least of a
    /// document it needs. `rows` builds rows with `aServerRow`; `report` is
    /// the expression printed. **Nothing where no Node is installed**, and the
    /// caller says it skipped.
    fn the_row_builder_run(rows: &str, report: &str) -> Option<String> {
        let page = the_page_module();
        let lifted = |head: &str| -> String {
            page.find(head)
                .map(|from| {
                    let to = from + page[from..].find("\n}\n").expect("the function closes") + 3;
                    page[from..to].to_string()
                })
                .unwrap_or_default()
        };
        let builder = lifted("function aServerRow(server) {");
        assert!(!builder.is_empty(), "the row builder is gone");
        let dot = lifted("function theDotIsDrawn(");
        // The real drawing of a value, which the builder calls (step 10c,
        // dependency wave piece 10a), with its exports made plain functions.
        let drawing = include_str!("../../web/the-value-drawn.js").replace("export function", "function");
        let script = format!(
            r#"
function anElement(tag) {{
  const names = new Set();
  return {{
    tag, dataset: {{}}, children: [], attributes: {{}},
    classList: {{ add: (...more) => more.forEach((one) => names.add(one)), contains: (one) => names.has(one) }},
    set className(value) {{ names.clear(); value.split(" ").filter(Boolean).forEach((one) => names.add(one)); }},
    get className() {{ return [...names].join(" "); }},
    setAttribute(name, value) {{ this.attributes[name] = String(value); }},
    appendChild(child) {{ this.children.push(child); return child; }},
    addEventListener() {{}},
  }};
}}
const document = {{ createElement: anElement }};
const serverRows = new Map();
const A_SERVER = "machineview";
function theCurrentPlaceIs() {{}}
{drawing}
{dot}
{builder}
{rows}
console.log(JSON.stringify({report}));
"#
        );
        let ran = std::process::Command::new("node").arg("-e").arg(&script).output().ok()?;
        assert!(ran.status.success(), "the builder did not run: {}", String::from_utf8_lossy(&ran.stderr));
        Some(String::from_utf8_lossy(&ran.stdout).trim().to_string())
    }

    /// **R3, drawn: the page's own row builder, run, puts the drawing's `away`
    /// on a row the application marked, and the badge inside the row, so the
    /// drawing's `.item.away .badge` dims the badge too.** **Skipped, and says
    /// so, where no Node is installed.**
    #[test]
    fn the_page_dims_the_row_and_its_badge() {
        let drawing = the_drawing();
        assert!(drawing.contains(".item.away .lbl {"), "the drawing lost the dim label");
        assert!(drawing.contains(".item.away .badge {"), "the drawing lost the dim badge");

        let Some(said) = the_row_builder_run(
            r#"
const dim = aServerRow({ server: "one", name: "Hetzner box", badge: "b3", away: true, dot: null });
const lit = aServerRow({ server: "two", name: "RunPod 4090", badge: "b1", away: false, dot: { rendering: false, said: "Connected" } });
const badge = dim.children.find((child) => child.classList.contains("badge"));
"#,
            r#"{
  dimIsAnItem: dim.classList.contains("item"),
  dimIsAway: dim.classList.contains("away"),
  litIsAway: lit.classList.contains("away"),
  theBadgeIsInsideTheDimRow: !!badge && badge.classList.contains("b3"),
}"#,
        ) else {
            eprintln!("SKIPPED: no Node on this computer, so the page's row builder was not run");
            return;
        };
        assert_eq!(
            said,
            r#"{"dimIsAnItem":true,"dimIsAway":true,"litIsAway":false,"theBadgeIsInsideTheDimRow":true}"#
        );
    }

    /// **Note 19, drawn: the page's own row builder draws the drawing's dot in
    /// its states** — `Design/Whole app § The server badge`. A connected
    /// server not rendering gets `.rdot`, a rendering one `.rdot.run` (the
    /// drawing's amber with the *Rendering* pulse), a reconnecting one
    /// `.rdot.trying` (the seventh notes, note 10: still, never the pulse), a
    /// disconnected one no dot. **Its word is also its hover word**, so the
    /// dot answers a pointer resting on it with what it says.
    /// **Last in the row**, after the badge and the name, which is where the
    /// drawing puts it and where its narrow-rail rule lifts it onto the badge.
    /// **And the dot's word is the row's to read aloud**, as an image with a
    /// label, never as text of its own. **No `.parked` mark is drawn on any
    /// row.** Skipped, and says so, where no Node is installed.
    #[test]
    fn the_page_draws_the_dot_in_its_three_states() {
        let drawing = the_drawing();
        assert!(drawing.contains(".rdot {"), "the drawing lost the rail's dot");
        assert!(
            drawing.contains(".rdot.run {") && drawing.contains("animation: working"),
            "the drawing lost the rendering dot's pulse"
        );
        assert!(
            drawing.contains(".rdot.trying { background: var(--busy-fill); opacity: .55; }"),
            "the drawing lost the reconnecting dot, the connect control's own"
        );
        assert!(!the_page_module().contains("parked"), "web/the-rail.js still draws the open-panel mark");

        let Some(said) = the_row_builder_run(
            r#"
const idle = aServerRow({ server: "one", name: "Vast 5060", badge: "b2", away: false, dot: { rendering: false, said: "Connected" } });
const busy = aServerRow({ server: "two", name: "RunPod 4090", badge: "b1", away: false, dot: { rendering: true, said: "Rendering" } });
const trying = aServerRow({ server: "four", name: "Hetzner box", badge: "b3", away: false, dot: { rendering: false, reconnecting: true, said: "Reconnecting…" } });
const away = aServerRow({ server: "three", name: "Hetzner box", badge: "b3", away: true, dot: null });
const the = (row) => {
  const dots = row.children.filter((child) => child.classList.contains("rdot"));
  const last = row.children[row.children.length - 1];
  return dots.map((dot) => ({
    run: dot.classList.contains("run"),
    trying: dot.classList.contains("trying"),
    hover: dot.title,
    last: dot === last,
    role: dot.attributes.role,
    label: dot.attributes["aria-label"],
    text: dot.textContent === undefined ? "" : dot.textContent,
  }));
};
"#,
            r#"{ idle: the(idle), busy: the(busy), trying: the(trying), away: the(away) }"#,
        ) else {
            eprintln!("SKIPPED: no Node on this computer, so the page's row builder was not run");
            return;
        };
        assert_eq!(
            said,
            concat!(
                r#"{"idle":[{"run":false,"trying":false,"hover":"Connected","last":true,"role":"img","label":"Connected","text":""}],"#,
                r#""busy":[{"run":true,"trying":false,"hover":"Rendering","last":true,"role":"img","label":"Rendering","text":""}],"#,
                r#""trying":[{"run":false,"trying":true,"hover":"Reconnecting…","last":true,"role":"img","label":"Reconnecting…","text":""}],"#,
                r#""away":[]}"#
            ),
            "the rail's dot is not drawn in the drawing's three states"
        );
    }

    /// **The dot follows the work, not only the connection.** The work column
    /// is told again whenever what a server is doing moves (`THE_WORK_MOVED`,
    /// posted only on a change), and the rail reads itself again on it — the
    /// same route a connect or a disconnect already takes — so a job starting
    /// or ending moves the dot without the user going anywhere.
    #[test]
    fn the_page_reads_the_rail_again_when_the_work_moves() {
        let page = the_page_module();
        let kind = crate::surface::servers::THE_WORK_MOVED;
        assert!(page.contains(&format!("\"{kind}\"")), "the page does not name {kind}");
        let at = page
            .find("whenTold(THE_WORK_MOVED")
            .expect("the rail does not hear that a server's work moved");
        let heard = &page[at..at + page[at..].find("});").expect("it closes")];
        assert!(heard.contains("theRailIsReadAgain()"), "the rail is not read again");
    }

    /// **Note 19: the dot says what the server is doing** — `Design/Whole app
    /// § The server badge`, changed 2026-10-04. Green (no pulse) while
    /// connected and not rendering, amber and pulsing while a job renders,
    /// none while not connected — driven through the register the application
    /// holds and the work column's own reading of it.
    #[test]
    fn the_dot_says_connected_rendering_or_nothing() {
        use crate::servers::a_period::{ThePeriodsThisRun, TheRunThroughTheApp};
        use crate::surface::the_queue::what_the_servers_are_doing;
        use smoozy_queue_history::line::QueueState;
        use smoozy_servers::acting::WhatServersPublishes;
        use smoozy_servers::condition::ObservedValue;
        use smoozy_servers::driving::APeriod;
        use smoozy_servers::period::{OpeningAct, WhoTookIt};
        use smoozy_state::identities::{Identity, IdentityKind};

        let the_server = Identity::issue(IdentityKind::LocalServer).expect("an identity");
        let mut row = a_row();
        row.local_server_identifier = the_server.as_text();
        let server = row.local_server_identifier.clone();
        let register = ThePeriodsThisRun::over(
            TheRunThroughTheApp::lending_nothing(),
            Arc::new(TheOneQueue::new()),
            Arc::new(crate::notifications::TheNotificationService::on(Box::new(
                NothingIsPosted,
            ))),
        );
        let holds = register.the_machines_work();
        let the_dot_now = || {
            let doing = what_the_servers_are_doing(holds, None);
            the_rail_from(std::slice::from_ref(&row), Some(&register), &doing, false).servers[0]
                .dot
                .clone()
        };
        let connected = Some(TheDot {
            rendering: false,
            reconnecting: false,
            said: Prose::new(crate::surface::servers::words::CONNECTED),
        });
        let rendering = Some(TheDot {
            rendering: true,
            reconnecting: false,
            said: Prose::new("Rendering"),
        });

        assert_eq!(the_dot_now(), None, "a server that is not connected has a dot");

        let period = APeriod::begun(
            &server,
            OpeningAct::TheUsersConnectAction,
            WhoTookIt::TheUser,
            std::time::SystemTime::UNIX_EPOCH,
        );
        register.a_period_has_begun(&server, &period);
        // Re-check, 2026-10-26: the first connection agrees with the pane.
        crate::surface::servers::THE_USERS_CONNECTS.pressed(&server);
        register.where_a_period_publishes().the_observed_value(&server, ObservedValue::Reconnecting);
        assert_eq!(the_dot_now().unwrap().said, Prose::new(crate::surface::set_up::words::CONNECTING));
        register
            .where_a_period_publishes()
            .the_observed_value(&server, ObservedValue::Connected);
        assert_eq!(the_dot_now(), connected, "connected with no reading yet");

        let an_operation = || Identity::issue(IdentityKind::Operation).expect("an operation");
        let (one, two) = (
            Identity::issue(IdentityKind::Job).expect("a job"),
            Identity::issue(IdentityKind::Job).expect("a job"),
        );
        crate::surface::the_queue::tests::a_reading_of(holds, &the_server, vec![]);
        assert_eq!(the_dot_now(), connected, "connected and idle");

        crate::surface::the_queue::tests::a_reading_of(
            holds,
            &the_server,
            vec![(one.clone(), QueueState::Waiting), (two.clone(), QueueState::Waiting)],
        );
        assert_eq!(the_dot_now(), connected, "jobs waiting and none rendering is not rendering");

        crate::surface::the_queue::tests::a_reading_of(
            holds,
            &the_server,
            vec![
                (one, QueueState::Rendering { operation: an_operation() }),
                (two, QueueState::Waiting),
            ],
        );
        assert_eq!(the_dot_now(), rendering, "a job renders and the dot does not say so");

        // **A setup driven there is still green**, as the drawing draws it —
        // the dot says only connected or rendering.
        use smoozy_servers::condition::Presentation;
        assert_eq!(the_dot(Presentation::Connected, Some(&WhatAServerIsDoing::NothingIsLive)), connected);
        assert_eq!(the_dot(Presentation::Connected, Some(&WhatAServerIsDoing::ItCannotBeSaid)), connected);

        // **Note 10 of the seventh notes: the connection drops while a job
        // renders, and the dot says it reconnects** — never *Rendering* from a
        // reading taken before it dropped, and never *Connected*.
        register
            .where_a_period_publishes()
            .the_observed_value(&server, ObservedValue::Reconnecting);
        let reconnecting = Some(TheDot {
            rendering: false,
            reconnecting: true,
            said: Prose::new("Reconnecting…"),
        });
        assert_eq!(the_dot_now(), reconnecting, "a server Smoozy is repairing does not say it reconnects");

        // **Not connected, no dot — even where the last reading had a job
        // rendering.**
        register.a_period_has_ended(&server);
        let doing = what_the_servers_are_doing(holds, None);
        let after = the_rail_from(std::slice::from_ref(&row), Some(&register), &doing, false);
        assert!(after.servers[0].away);
        assert_eq!(after.servers[0].dot, None, "a disconnected server kept its dot");
        assert_eq!(
            the_dot(
                Presentation::Disconnected,
                Some(&WhatAServerIsDoing::Rendering { name: String::new(), waiting: 0 })
            ),
            None,
            "a dim row drew a dot from a reading taken before the connection went"
        );
    }

    /// **Note 10 of the seventh notes: a reconnecting server's dot** —
    /// `Design/Whole app` line 179, changed 2026-10-10. Asked of `Servers`' own
    /// presentation, from what is true of the server: *repairing* and
    /// *restarting* both draw the reconnecting dot and say the server list's
    /// own *Reconnecting…*, whatever the last reading had; *connected* keeps
    /// its own dot; *disconnected* has none.
    #[test]
    fn a_reconnecting_server_draws_the_reconnecting_dot() {
        use smoozy_servers::condition::{presentation, Presentation, WhatIsTrueOfTheServer};
        let rendering = WhatAServerIsDoing::Rendering { name: "shot".to_string(), waiting: 0 };
        let a_server = |live: bool, period: bool, restart: bool| WhatIsTrueOfTheServer {
            a_period_stands: period,
            the_primary_connection_is_live: live,
            a_run_of_attempts_is_in_flight: !live && period,
            the_restart_mark_stands: restart,
        };
        let repairing = a_server(false, true, false);
        let restarting = a_server(false, true, true);
        assert_eq!(presentation(repairing), Presentation::Repairing);
        assert_eq!(presentation(restarting), Presentation::Restarting);

        let reconnecting = Some(TheDot {
            rendering: false,
            reconnecting: true,
            said: Prose::new("Reconnecting…"),
        });
        for (of, what) in [(repairing, "repairing"), (restarting, "restarting")] {
            for doing in [None, Some(&rendering), Some(&WhatAServerIsDoing::NothingIsLive)] {
                assert_eq!(
                    the_dot(presentation(of), doing),
                    reconnecting,
                    "a {what} server's dot does not say it reconnects"
                );
            }
            assert!(!it_is_not_connected(of), "a {what} server reads dim");
        }

        let connected = a_server(true, true, false);
        assert_eq!(
            the_dot(presentation(connected), None),
            Some(TheDot {
                rendering: false,
                reconnecting: false,
                said: Prose::new(crate::surface::servers::words::CONNECTED),
            }),
            "a connected server lost its own dot"
        );
        assert_eq!(
            the_dot(presentation(connected), Some(&rendering)),
            Some(TheDot { rendering: true, reconnecting: false, said: Prose::new("Rendering") }),
            "a rendering server lost its own dot"
        );
        assert_eq!(
            the_dot(presentation(a_server(false, false, false)), Some(&rendering)),
            None,
            "a disconnected server has a dot"
        );
    }

    /// **The twenty-fourth notes, note 2: the Queue's dot shows while a job in
    /// the queue reads *Needs your answer*, and goes when it is answered** —
    /// `Design/Whole app § The server badge`. Asked of the Queue screen's own
    /// rows, so a job held for the driver (*Waiting for driver update*) does
    /// not set it, and neither does a job waiting or rendering. **The word the
    /// mark answers to is the queue row's own word**, held together here.
    #[test]
    fn the_queues_mark_shows_while_a_job_needs_your_answer() {
        use crate::servers::the_machines_work::WhatThisRunHolds;
        use crate::surface::the_queue::tests::{a_reading_of, a_reading_with_its_holds_of};
        use smoozy_queue_history::line::QueueState;
        use smoozy_queue_history::readings::WhatTheHoldSays;
        use smoozy_state::identities::{Identity, IdentityKind};

        let path = crate::scratch_folders::a_folder(format!("smoozy-rail-queue-mark-{}", std::process::id()));
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).expect("a record");
        let server = Identity::issue(IdentityKind::LocalServer).expect("a server");
        let made = crate::record::deletions::tests::a_record_in_history(&server, "Hallway — final");
        crate::record::job_records::JobRecords::over(record.connection())
            .write_the_provisional_record(&made)
            .expect("the job's record");
        let held = || QueueState::Held { hold: Identity::issue(IdentityKind::Operation).expect("a hold") };
        let holds = WhatThisRunHolds::default();
        let asking = || a_job_in_the_queue_needs_your_answer(&holds, Some(&record));

        assert!(!asking(), "no reading, no dot");
        assert!(!a_job_in_the_queue_needs_your_answer(&holds, None), "no record, no dot");

        a_reading_of(&holds, &server, vec![(made.job.clone(), held())]);
        assert!(asking(), "a job needs your answer and the Queue has no dot");
        let screen = crate::surface::the_queue::what_is_held(&holds, Some(&record), None);
        assert_eq!(
            screen.lines[0].rows[0].state,
            Some(NEEDS_YOUR_ANSWER),
            "the mark answers to a word the queue's row no longer writes"
        );

        // **Answered**: *Render all* puts it back in line, and the dot goes.
        a_reading_of(&holds, &server, vec![(made.job.clone(), QueueState::Waiting)]);
        assert!(!asking(), "the job was answered and the dot stayed");
        a_reading_of(
            &holds,
            &server,
            vec![(made.job.clone(), QueueState::Rendering { operation: Identity::issue(IdentityKind::Operation).unwrap() })],
        );
        assert!(!asking(), "a rendering job set the dot");

        // **Held for the driver update** reads *Waiting for driver update*.
        a_reading_with_its_holds_of(
            &holds,
            &server,
            vec![(
                made.job.clone(),
                held(),
                Some(WhatTheHoldSays { reason: smoozy_rendering::submission::THE_DRIVER.to_string(), held_behind: None }),
            )],
        );
        assert!(!asking(), "a job waiting for the driver update set the dot");

        // **And the rail carries it on the Queue row alone.**
        let on = the_rail_from(&[a_row()], None, &WhatTheServersAreDoing::new(), true);
        let off = the_rail_from(&[a_row()], None, &WhatTheServersAreDoing::new(), false);
        for (rail, shows) in [(&on, true), (&off, false)] {
            let marked: Vec<&str> = rail.places.iter().filter(|place| place.mark.is_some()).map(|place| place.place).collect();
            assert_eq!(marked, vec!["queueview"], "the mark is the Queue row's alone");
            let mark = rail.places.iter().find_map(|place| place.mark.as_ref()).unwrap();
            assert_eq!(mark.on, shows);
            assert_eq!(mark.hover, Prose::new("Queue — needs your answer"));
        }
    }

    /// **The drawing's rules for the Queue's dot are in the stylesheet**, word
    /// for word from the mockup — copied by the Files viewer's builder
    /// (2026-10-31), checked here.
    #[test]
    fn the_drawing_has_the_queues_dot() {
        let drawing = the_drawing();
        for rule in [
            ".qask { width: 6px; height: 6px; border-radius: 50%; background: var(--busy); flex: none; margin-left: -2px; }",
            ".qask[hidden] { display: none; }",
            ".item .lbl.fit { flex: 0 1 auto; }",
            ".item .lbl.fit ~ .tl { margin-left: auto; }",
            ".app[data-narrow~=\"s4\"] .rail .qask {",
            ".app[data-narrow~=\"s4\"] .rail .item[aria-current=\"true\"] .qask { box-shadow: 0 0 0 2px var(--sel); }",
        ] {
            assert!(drawing.contains(rule), "the drawing lost {rule}");
        }
    }

    /// **The page's own fixed-row builder, run under Node: the Queue row draws
    /// the drawing's `.qask` just after its label, the label made `.lbl.fit`,
    /// hidden while no job needs an answer, and its hover name is *Queue —
    /// needs your answer* while it shows** — the drawing's `rail-ask` and
    /// `rail-noask`. No other row draws one. Skipped, and says so, where no
    /// Node is installed.
    #[test]
    fn the_page_draws_the_queues_dot_after_its_word() {
        let page = the_page_module();
        let lifted = |head: &str| -> String {
            let from = page.find(head).unwrap_or_else(|| panic!("{head} is gone"));
            let to = from + page[from..].find("\n}\n").expect("the function closes") + 3;
            page[from..to].to_string()
        };
        let script = format!(
            r#"
function anElement(tag) {{
  const names = new Set();
  return {{
    tag, dataset: {{}}, children: [], attributes: {{}}, hidden: false, title: "", textContent: "",
    classList: {{ add: (...more) => more.forEach((one) => names.add(one)), contains: (one) => names.has(one) }},
    set className(value) {{ names.clear(); value.split(" ").filter(Boolean).forEach((one) => names.add(one)); }},
    get className() {{ return [...names].join(" "); }},
    setAttribute(name, value) {{ this.attributes[name] = String(value); }},
    appendChild(child) {{ this.children.push(child); return child; }},
    addEventListener() {{}},
  }};
}}
const document = {{ createElement: anElement }};
const rows = new Map();
function theMark() {{ return null; }}
function theCurrentPlaceIs() {{}}
{mark}
{row}
const hover = "Queue — needs your answer";
const the = (row) => ({{
  title: row.title,
  order: row.children.map((child) => child.className),
  fit: row.children[0].classList.contains("fit"),
  hidden: row.children.filter((child) => child.className === "qask").map((dot) => dot.hidden),
  aloud: row.children.filter((child) => child.className === "qask").map((dot) => dot.attributes["aria-hidden"]),
}});
console.log(JSON.stringify({{
  ask: the(aFixedRow({{ place: "queueview", name: "Queue", section: 1, count: null, mark: {{ on: true, hover }} }})),
  noask: the(aFixedRow({{ place: "queueview", name: "Queue", section: 1, count: null, mark: {{ on: false, hover }} }})),
  history: the(aFixedRow({{ place: "historyview", name: "History", section: 1, count: null, mark: null }})),
}}));
"#,
            mark = lifted("function theQueuesMarkIsDrawn("),
            row = lifted("function aFixedRow(place) {"),
        );
        let Ok(ran) = std::process::Command::new("node").arg("-e").arg(&script).output() else {
            eprintln!("SKIPPED: no Node on this computer, so the page's fixed-row builder was not run");
            return;
        };
        assert!(ran.status.success(), "the builder did not run: {}", String::from_utf8_lossy(&ran.stderr));
        assert_eq!(
            String::from_utf8_lossy(&ran.stdout).trim(),
            concat!(
                r#"{"ask":{"title":"Queue — needs your answer","order":["lbl fit","qask","tl mono"],"fit":true,"hidden":[false],"aloud":["true"]},"#,
                r#""noask":{"title":"Queue","order":["lbl fit","qask","tl mono"],"fit":true,"hidden":[true],"aloud":["true"]},"#,
                r#""history":{"title":"History","order":["lbl","tl mono"],"fit":false,"hidden":[],"aloud":[]}}"#
            ),
            "the Queue row does not draw the drawing's rail-ask and rail-noask"
        );
    }

    /// **A redraw of the rail keeps the figures the screens set** — Queue's
    /// count stands beside the dot in the drawing's `rail-ask`, and the rail
    /// is read again whenever the dot moves.
    #[test]
    fn a_redraw_keeps_the_screens_figures() {
        let page = the_page_module();
        let at = page.find("export function theRailIsReadAgain()").expect("the re-read is gone");
        let body = &page[at..at + page[at..].find("\n}\n").expect("it closes")];
        assert!(body.contains("theRowsFigureIs(place, figure)"), "a redraw drops the screens' figures");
    }

    struct Collects(Arc<Mutex<Vec<crate::events::Delivery>>>);
    impl crate::events::Delivers for Collects {
        fn deliver(&self, delivery: &crate::events::Delivery) {
            self.0.lock().expect("the collector").push(delivery.clone());
        }
    }

    fn a_queue() -> (Arc<TheOneQueue>, Arc<Mutex<Vec<crate::events::Delivery>>>) {
        let queue = Arc::new(TheOneQueue::new());
        let held = Arc::new(Mutex::new(Vec::new()));
        queue.the_page_is_ready(Box::new(Collects(Arc::clone(&held))));
        (queue, held)
    }

    fn a_row() -> smoozy_servers::store::ServerRow {
        smoozy_servers::store::ServerRow {
            local_server_identifier: "one".to_string(),
            host: smoozy_servers::identity::Host::what_the_user_typed("10.0.0.4")
                .expect("a host"),
            port: smoozy_servers::identity::Port::STANDARD_SSH,
            username: smoozy_servers::identity::Username::what_the_user_typed("root")
                .expect("a username"),
            key: smoozy_servers::identity::KeyReference {
                local_key_identifier: "k1".to_string(),
                fingerprint: "SHA256:one".to_string(),
            },
            label: None,
            colour: None,
            remembered_setup: None,
            remembered_host_key: None,
            remembered_hardware_reading: None,
            recorded_machine_root: None,
            last_confirmed_installation_identity: None,
            event_cursor: None,
            machine_password_subject: None,
            chosen_fallback_folder: None,
            the_driver_missed_the_optix_bar: false,
        }
    }
}

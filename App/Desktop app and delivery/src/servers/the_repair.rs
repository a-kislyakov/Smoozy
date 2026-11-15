//! **The automatic repair** — `Desktop app and delivery` V3a to V3c. Added
//! 2026-11-14.
//!
//! **What it is, in one sentence.** Where step 5 of a connection reads this
//! copy's own installation and its supervisor stays silent through the
//! re-take's first re-read (`Servers` Q19b), V11's request is sent for this
//! copy's own release over the connection step 5 holds — once per connection —
//! and the server's row says so while it runs and if it fails.
//!
//! **What it never does** (V3b): repair a program another copy placed, a
//! program older or newer than this copy's own, a package area that could not
//! be read, or a server a setup run or an update already holds. Each of those
//! keeps the way it had: `Server Setup` F1's re-run, pressed by the user, with
//! its own asks.
//!
//! **Everything that reaches a machine or the internet is a seam**
//! ([`TheRepairsOwnEnds`]), so the judgement and the order are driven at the
//! desk. The real ends are [`TheRepairOverThisPoint`].
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document at the site.

use std::future::Future;

use smoozy_server_protocol::placement::performing::WhatThePlacementReached;
use smoozy_server_protocol::release::verifying::AVerifiedRelease;
use smoozy_server_protocol::transport::ssh;
use smoozy_state::identities::Identity;

use crate::servers::installation::WhatThePackageAreaHolds;
use crate::servers::published::{
    the_direction, the_server_carries, what_the_server_carries, TheDirection,
    WhatSmoozyOnTheServerIs as Carries, WhereTheUpdateFailed,
};
use crate::servers::sessions::ThisServersSessions;
use crate::setting_up::ADownloadThatFailed;

/// **What step 5's read met that V3b repairs**: this copy's own installation,
/// read equal to the remembered one, with its supervisor not confirming it and
/// no protocol difference to explain the silence — and the package area as the
/// same read inspected it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OurOwnProgramIsSilent {
    pub the_package_area: String,
    pub installation: Identity,
    /// `Server protocol and transfers` B4's inspection, taken by that read; a
    /// reading not taken is `it_was_read: false`, and nothing is sent on it.
    pub held: WhatThePackageAreaHolds,
}

/// **V3b's judgement**, from the inspection and this copy's own release.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheRepairDecides {
    /// The area was not read, its program shares no protocol with this build,
    /// no active release reads, or the machine's release is older: nothing is
    /// sent and V1's condition stands.
    NothingIsSent,
    /// **V14**: nothing below the machine's own is placed, and V13's *too new*
    /// takes V1's line.
    TheMachineIsTooNew,
    /// The machine runs this copy's own release: V11's request is sent.
    TheRepairIsSent,
}

/// **V3b, items 2 and 3.** *Older is not repaired*: its work cannot be read
/// while it is silent, and V11 holds an unreadable reading as work.
pub fn what_the_repair_decides(held: &WhatThePackageAreaHolds, own: u64) -> WhatTheRepairDecides {
    if !held.it_was_read || !held.it_speaks_what_this_build_speaks() {
        return WhatTheRepairDecides::NothingIsSent;
    }
    match the_direction(held.its_release_sequence, own) {
        Some(TheDirection::TheSame) => WhatTheRepairDecides::TheRepairIsSent,
        Some(TheDirection::TheMachineIsNewer) => WhatTheRepairDecides::TheMachineIsTooNew,
        Some(TheDirection::TheMachineIsOlder) | None => WhatTheRepairDecides::NothingIsSent,
    }
}

/// **How a sent repair ended**, in V3c's terms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheRepairsEnding {
    /// `Rented-server runtime` G3's *verified*: the program answers again.
    ItLanded,
    /// Anything else, with whose words they are.
    ItFailed { said: String, at: WhereTheUpdateFailed },
    /// **The package area no longer reads as it did when the repair was
    /// decided** — read again after this copy's release was had, just before
    /// the placement — so nothing was sent: another copy, or a setup, may have
    /// placed something meanwhile. Added 2026-11-14, out of Sol's look.
    TheServerChanged,
}

/// **Everything the repair reaches that is not a judgement**, as a seam.
pub trait TheRepairsOwnEnds {
    /// This copy's own release sequence, or why it could not be had.
    fn this_copys_own_release(&mut self) -> impl Future<Output = Result<u64, String>> + Send;
    /// **V11's request, sent** — under `under` where a deferred phase recorded
    /// one (V12), and under a fresh identity otherwise.
    fn the_repair_is_sent(
        &mut self,
        under: Option<Identity>,
        silent: &OurOwnProgramIsSilent,
    ) -> impl Future<Output = TheRepairsEnding> + Send;
    /// The row's line moved, so the page is told.
    fn the_line_moved(&mut self);
    /// **The placement's health exchange was answered** (`Rented-server
    /// runtime` G12): the supervisor answered a command-path request, so V1's
    /// condition is withdrawn (V3c as changed 2026-11-15).
    fn the_program_answered(&mut self);
}

/// **What the repair did**, as much as a caller or a test wants to see.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheRepairDid {
    /// This copy's own release could not be had, so nothing was compared.
    TheOwnReleaseCouldNotBeHad { said: String },
    /// **`Server Setup` Y9**: a setup run or an update already holds the
    /// server, so nothing was sent.
    AnotherActHoldsTheServer,
    /// What V3b decided, and — where the repair was sent — how it ended.
    Decided {
        decided: WhatTheRepairDecides,
        ending: Option<TheRepairsEnding>,
    },
}

impl WhatTheRepairDid {
    /// **`Servers` Q19b: a repair that landed is read again at once.**
    pub fn a_read_should_follow_at_once(&self) -> bool {
        matches!(
            self,
            WhatTheRepairDid::Decided {
                ending: Some(TheRepairsEnding::ItLanded),
                ..
            }
        )
    }
}

/// **V3a to V3c, on one silent read.**
///
/// **The turn is `Server Setup` Y9's, taken as V11's update takes it**, so a
/// setup run or an update on the server goes first, and a press made while the
/// repair runs joins the line behind it (`Server Setup` Y9a item 11).
pub async fn the_repair(
    server: &str,
    silent: &OurOwnProgramIsSilent,
    ends: &mut impl TheRepairsOwnEnds,
) -> WhatTheRepairDid {
    if !silent.held.it_was_read {
        return WhatTheRepairDid::Decided {
            decided: WhatTheRepairDecides::NothingIsSent,
            ending: None,
        };
    }
    let own = match ends.this_copys_own_release().await {
        Ok(own) => own,
        Err(said) => {
            // **V3c: said on the row, not dropped** (Sol's look, 2026-11-14):
            // the repair could not be decided, and *Set up again* is the way.
            if the_server_carries(
                server,
                Some(Carries::TheRepairFailed {
                    said: said.clone(),
                    at: WhereTheUpdateFailed::InTheDownload { nothing_answered: false },
                }),
            ) {
                ends.the_line_moved();
            }
            return WhatTheRepairDid::TheOwnReleaseCouldNotBeHad { said };
        }
    };
    let decided = what_the_repair_decides(&silent.held, own);
    match decided {
        WhatTheRepairDecides::NothingIsSent => {
            return WhatTheRepairDid::Decided { decided, ending: None };
        }
        WhatTheRepairDecides::TheMachineIsTooNew => {
            if the_server_carries(server, Some(Carries::TooNew)) {
                ends.the_line_moved();
            }
            return WhatTheRepairDid::Decided { decided, ending: None };
        }
        WhatTheRepairDecides::TheRepairIsSent => {}
    }
    use crate::setting_up::{an_update_takes_its_turn, TheUpdatesTurn};
    if an_update_takes_its_turn(server) != TheUpdatesTurn::ItIsTaken {
        return WhatTheRepairDid::AnotherActHoldsTheServer;
    }
    // **The turn is let go however this ends**, a connection ending part-way
    // included, and a line still saying *under way* goes with it.
    let _the_turn = TheRepairsTurnIsLetGo(server);
    the_server_carries(server, Some(Carries::TheRepairIsUnderWay));
    ends.the_line_moved();
    // **V12: a deferred phase is re-sent under its recorded attempt.**
    let under = silent
        .held
        .the_phase_record
        .as_ref()
        .filter(|record| record.phase == smoozy_runtime::package::phase::Phase::Deferred)
        .map(|record| record.attempt.clone());
    let ending = ends.the_repair_is_sent(under, silent).await;
    // **V3c as changed 2026-11-15: a repair that lands withdraws V1's
    // condition at once, before its line goes** — the placement's health
    // exchange is the supervisor answering. Until then the condition stood
    // until the re-take's next read was answered, and the re-check before
    // 0.4.0 watched *Smoozy isn't responding.* for about 20 seconds after the
    // repair while the new program ran. That read raises it again where
    // Smoozy does not answer.
    if ending == TheRepairsEnding::ItLanded {
        ends.the_program_answered();
    }
    the_server_carries(
        server,
        match &ending {
            TheRepairsEnding::ItLanded | TheRepairsEnding::TheServerChanged => None,
            TheRepairsEnding::ItFailed { said, at } => Some(Carries::TheRepairFailed {
                said: said.clone(),
                at: *at,
            }),
        },
    );
    ends.the_line_moved();
    WhatTheRepairDid::Decided {
        decided,
        ending: Some(ending),
    }
}

/// **The repair's turn on one server, let go when the repair is done with it**
/// — and a line left saying *under way*, by a repair cut off part-way, taken
/// down with it.
struct TheRepairsTurnIsLetGo<'a>(&'a str);

impl Drop for TheRepairsTurnIsLetGo<'_> {
    fn drop(&mut self) {
        if what_the_server_carries(self.0) == Some(Carries::TheRepairIsUnderWay) {
            the_server_carries(self.0, None);
        }
        crate::setting_up::the_update_lets_the_server_go(self.0);
    }
}

// ---------------------------------------------------------------- the real ends

/// **This copy's own release, read for a repair** — the two reads V11's update
/// makes, handed in by the running application, which holds the record they
/// need.
pub struct TheRepairsRelease {
    /// This copy's own release sequence (`Server protocol and transfers` B8d).
    pub own: Box<dyn Fn() -> Result<u64, String> + Send + Sync>,
    /// This copy's own release and its program, verified.
    pub and_its_program:
        Box<dyn Fn() -> Result<(AVerifiedRelease, Vec<u8>), ADownloadThatFailed> + Send + Sync>,
}

/// **The repair's ends over a point of its own** — V3a: the request rides the
/// connection step 5 holds.
pub struct TheRepairOverThisPoint<'a, F: FnMut() + Send, G: FnMut() + Send> {
    pub sessions: &'a ThisServersSessions<ssh::Connection>,
    pub release: std::sync::Arc<TheRepairsRelease>,
    pub the_line_moved: F,
    /// V1's condition withdrawn, where the repair landed.
    pub the_program_answered: G,
}

/// **A read of this copy's release, on a thread of its own and awaited** — the
/// reads are blocking HTTPS with their own retries, and the repair runs on the
/// connection's own driving, which must stay free to hear the connection end
/// (Sol's look, 2026-11-14).
async fn off_the_driving<T: Send + 'static>(read: impl FnOnce() -> T + Send + 'static) -> Option<T> {
    let (tell, told) = tokio::sync::oneshot::channel();
    std::thread::Builder::new()
        .name("smoozy repair's release read".to_string())
        .spawn(move || {
            let _ = tell.send(read());
        })
        .ok()?;
    told.await.ok()
}

impl<F: FnMut() + Send, G: FnMut() + Send> TheRepairsOwnEnds for TheRepairOverThisPoint<'_, F, G> {
    async fn this_copys_own_release(&mut self) -> Result<u64, String> {
        let release = std::sync::Arc::clone(&self.release);
        off_the_driving(move || (release.own)())
            .await
            .unwrap_or_else(|| Err("Could not read this app's version.".to_string()))
    }

    async fn the_repair_is_sent(
        &mut self,
        under: Option<Identity>,
        silent: &OurOwnProgramIsSilent,
    ) -> TheRepairsEnding {
        let release = std::sync::Arc::clone(&self.release);
        let got = off_the_driving(move || (release.and_its_program)())
            .await
            .unwrap_or_else(|| Err(ADownloadThatFailed::from("Could not read this app's version.".to_string())));
        let (verified, artifact) = match got {
            Ok(got) => got,
            Err(failed) => {
                return TheRepairsEnding::ItFailed {
                    said: failed.said,
                    at: WhereTheUpdateFailed::InTheDownload {
                        nothing_answered: failed.nothing_answered,
                    },
                }
            }
        };
        // **The decision, checked again just before the placement** (Sol's
        // look, 2026-11-14): the download can take a while, and another copy
        // or a setup may have placed something meanwhile. Anything that reads
        // differently is not repaired over; the claim the placement then takes
        // covers what is left of the window.
        let now = crate::servers::installation::what_the_package_area_holds(self.sessions, &silent.the_package_area).await;
        if !the_same_reading(&now, &silent.held) {
            return TheRepairsEnding::TheServerChanged;
        }
        let placed = crate::setting_up::the_update_is_placed(
            self.sessions,
            crate::setting_up::OverTheLentConnection { gives_up: None },
            &silent.the_package_area,
            silent.installation.clone(),
            &silent.held,
            under,
            verified,
            artifact,
            // **V3b: no accepted work** — a silent supervisor gives no reading.
            Vec::new(),
        )
        .await;
        what_the_placement_came_to(placed)
    }

    fn the_line_moved(&mut self) {
        (self.the_line_moved)();
    }

    fn the_program_answered(&mut self) {
        (self.the_program_answered)();
    }
}

/// **Whether the package area reads as it did when the repair was decided** —
/// the same release, the same phase record and the same start-up record.
pub fn the_same_reading(now: &WhatThePackageAreaHolds, then: &WhatThePackageAreaHolds) -> bool {
    now.it_was_read && now == then
}

/// **A placement's ending, in V3c's terms.** Only *verified* lands; a claim
/// another holds, a deferral and a rollback are each a repair that did not.
pub fn what_the_placement_came_to(placed: Result<WhatThePlacementReached, String>) -> TheRepairsEnding {
    let on_the_server = |said: String| TheRepairsEnding::ItFailed {
        said,
        at: WhereTheUpdateFailed::OnTheServer,
    };
    match placed {
        Ok(WhatThePlacementReached::Verified) => TheRepairsEnding::ItLanded,
        Ok(WhatThePlacementReached::RolledBack) => {
            on_the_server("Smoozy did not start after the repair. Rolled back.".to_string())
        }
        Ok(WhatThePlacementReached::FailedBeforeCommit { because })
        | Ok(WhatThePlacementReached::RollingBack { because })
        | Ok(WhatThePlacementReached::Deferred { because }) => on_the_server(because),
        Ok(WhatThePlacementReached::APackageTransactionIsInProgress { .. })
        | Ok(WhatThePlacementReached::AClaimStandsWhoseOwnerCouldNotBeRead) => {
            on_the_server("Another install of Smoozy is in progress on this server.".to_string())
        }
        Err(said) => on_the_server(said),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_server_protocol::framing::THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS;
    use smoozy_state::identities::IdentityKind;

    fn a_server() -> String {
        Identity::issue(IdentityKind::LocalServer).expect("randomness").as_text()
    }

    fn a_program_at(sequence: Option<u64>) -> WhatThePackageAreaHolds {
        let ours = THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS.0;
        WhatThePackageAreaHolds {
            it_was_read: true,
            its_release_sequence: sequence,
            the_declared_intervals: vec![(ours, ours)],
            ..Default::default()
        }
    }

    fn silent(held: WhatThePackageAreaHolds) -> OurOwnProgramIsSilent {
        OurOwnProgramIsSilent {
            the_package_area: "/ephemeral/smoozy/package".into(),
            installation: Identity::issue(IdentityKind::Installation).expect("randomness"),
            held,
        }
    }

    /// A stand-in for the machine and the release, recording what it was asked.
    struct TheDesk {
        own: Result<u64, String>,
        ends_with: TheRepairsEnding,
        server: String,
        sent: usize,
        carried_while_sent: Vec<Option<Carries>>,
        lines_moved: usize,
        /// **What each telling saw**, in order: the line as it was carried
        /// when it moved, or that the program answered.
        told: Vec<Told>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Told {
        TheLineMoved(Option<Carries>),
        TheProgramAnswered,
    }

    impl TheDesk {
        fn ending(own: u64, ends_with: TheRepairsEnding, server: &str) -> TheDesk {
            TheDesk {
                own: Ok(own),
                ends_with,
                server: server.to_string(),
                sent: 0,
                carried_while_sent: Vec::new(),
                lines_moved: 0,
                told: Vec::new(),
            }
        }
    }

    impl TheRepairsOwnEnds for TheDesk {
        async fn this_copys_own_release(&mut self) -> Result<u64, String> {
            self.own.clone()
        }

        async fn the_repair_is_sent(
            &mut self,
            _under: Option<Identity>,
            _silent: &OurOwnProgramIsSilent,
        ) -> TheRepairsEnding {
            self.sent += 1;
            self.carried_while_sent.push(what_the_server_carries(&self.server));
            self.ends_with.clone()
        }

        fn the_line_moved(&mut self) {
            self.lines_moved += 1;
            self.told.push(Told::TheLineMoved(what_the_server_carries(&self.server)));
        }

        fn the_program_answered(&mut self) {
            self.told.push(Told::TheProgramAnswered);
        }
    }

    fn repaired(server: &str, held: WhatThePackageAreaHolds, desk: &mut TheDesk) -> WhatTheRepairDid {
        tauri::async_runtime::block_on(the_repair(server, &silent(held), desk))
    }

    /// **V3b's table**: only this copy's own release is repaired.
    #[test]
    fn the_automatic_repair_is_sent_only_over_this_copys_own_release() {
        use WhatTheRepairDecides::*;
        assert_eq!(what_the_repair_decides(&a_program_at(Some(7)), 7), TheRepairIsSent);
        assert_eq!(what_the_repair_decides(&a_program_at(Some(8)), 7), TheMachineIsTooNew);
        assert_eq!(what_the_repair_decides(&a_program_at(Some(6)), 7), NothingIsSent, "older");
        assert_eq!(what_the_repair_decides(&a_program_at(None), 7), NothingIsSent, "no active release");
        assert_eq!(what_the_repair_decides(&WhatThePackageAreaHolds::default(), 7), NothingIsSent, "not read");
        let ours = THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS.0;
        let mut another_protocol = a_program_at(Some(7));
        another_protocol.the_declared_intervals = vec![(ours + 1, ours + 1)];
        assert_eq!(what_the_repair_decides(&another_protocol, 7), NothingIsSent, "no shared protocol");
    }

    /// **V3c: the line says *under way* while the request is out, and goes
    /// when the repair lands**; the next read follows at once (`Servers`
    /// Q19b), and the turn is given back.
    #[test]
    fn the_automatic_repair_that_lands_says_so_while_it_runs_and_then_nothing() {
        let server = a_server();
        let mut desk = TheDesk::ending(7, TheRepairsEnding::ItLanded, &server);
        let did = repaired(&server, a_program_at(Some(7)), &mut desk);
        assert_eq!(desk.sent, 1);
        assert_eq!(desk.carried_while_sent, vec![Some(Carries::TheRepairIsUnderWay)]);
        assert_eq!(what_the_server_carries(&server), None);
        assert_eq!(desk.lines_moved, 2);
        assert!(did.a_read_should_follow_at_once());
        assert!(!crate::setting_up::an_update_holds(&server), "the turn was not given back");
    }

    /// **The re-check before 0.4.0, new fault 1: after the repair landed the
    /// server's page said *Smoozy isn't responding.* again for about 20
    /// seconds** while the new program ran — V1's condition stood until the
    /// re-take's next read was answered. **A repair that lands withdraws it at
    /// once, before its line goes** (V3c as changed 2026-11-15), so the line
    /// is never taken down with the condition still standing behind it. A
    /// repair that did not land says nothing of Smoozy answering.
    #[test]
    fn a_repair_that_lands_withdraws_the_not_responding_condition_before_its_line_goes() {
        let server = a_server();
        let mut desk = TheDesk::ending(7, TheRepairsEnding::ItLanded, &server);
        repaired(&server, a_program_at(Some(7)), &mut desk);
        assert_eq!(
            desk.told,
            vec![
                Told::TheLineMoved(Some(Carries::TheRepairIsUnderWay)),
                Told::TheProgramAnswered,
                Told::TheLineMoved(None),
            ]
        );
        let failed = TheRepairsEnding::ItFailed {
            said: "No space left on device".into(),
            at: WhereTheUpdateFailed::OnTheServer,
        };
        for ending in [failed, TheRepairsEnding::TheServerChanged] {
            let mut desk = TheDesk::ending(7, ending, &server);
            repaired(&server, a_program_at(Some(7)), &mut desk);
            assert!(!desk.told.contains(&Told::TheProgramAnswered));
            the_server_carries(&server, None);
        }
    }

    /// **V3c: a repair that did not land says so, with its words**, and the
    /// next read keeps the re-take's pause.
    #[test]
    fn the_automatic_repair_that_fails_leaves_its_words_on_the_row() {
        let server = a_server();
        let failed = TheRepairsEnding::ItFailed {
            said: "No space left on device".into(),
            at: WhereTheUpdateFailed::OnTheServer,
        };
        let mut desk = TheDesk::ending(7, failed, &server);
        let did = repaired(&server, a_program_at(Some(7)), &mut desk);
        assert_eq!(desk.sent, 1);
        assert_eq!(
            what_the_server_carries(&server),
            Some(Carries::TheRepairFailed {
                said: "No space left on device".into(),
                at: WhereTheUpdateFailed::OnTheServer,
            })
        );
        assert!(!did.a_read_should_follow_at_once());
        the_server_carries(&server, None);
    }

    /// **V3b: never over an older program, a newer one, or an unread area** —
    /// and a newer one carries V13's *too new* in V1's place.
    #[test]
    fn the_automatic_repair_never_replaces_an_older_or_newer_program() {
        let server = a_server();
        let mut desk = TheDesk::ending(7, TheRepairsEnding::ItLanded, &server);
        repaired(&server, a_program_at(Some(6)), &mut desk);
        repaired(&server, WhatThePackageAreaHolds::default(), &mut desk);
        assert_eq!(desk.sent, 0);
        assert_eq!(what_the_server_carries(&server), None);
        repaired(&server, a_program_at(Some(8)), &mut desk);
        assert_eq!(desk.sent, 0);
        assert_eq!(what_the_server_carries(&server), Some(Carries::TooNew));
        the_server_carries(&server, None);
        // **And nothing is compared where this copy's own release is not had.**
        let mut offline = TheDesk::ending(7, TheRepairsEnding::ItLanded, &server);
        offline.own = Err("no line".into());
        assert_eq!(
            repaired(&server, a_program_at(Some(7)), &mut offline),
            WhatTheRepairDid::TheOwnReleaseCouldNotBeHad { said: "no line".into() }
        );
        assert_eq!(offline.sent, 0);
        // **And it is said on the row** (Sol's look): *Set up again* is the way.
        assert!(matches!(what_the_server_carries(&server), Some(Carries::TheRepairFailed { .. })));
        the_server_carries(&server, None);
    }

    /// **`Server Setup` Y9: a setup run on the server goes first**, and nothing
    /// is sent behind it.
    #[test]
    fn the_automatic_repair_waits_for_a_setup_run_on_the_server() {
        let server = a_server();
        assert!(crate::setting_up::a_run_begins(&server, smoozy_server_setup::run::SetupKind::TheReadyAction));
        let mut desk = TheDesk::ending(7, TheRepairsEnding::ItLanded, &server);
        assert_eq!(
            repaired(&server, a_program_at(Some(7)), &mut desk),
            WhatTheRepairDid::AnotherActHoldsTheServer
        );
        assert_eq!(desk.sent, 0);
        crate::setting_up::a_run_ends(&server);
    }

    /// **Sol's look: a package area that reads differently after the download
    /// is not repaired over**, and the line goes back to what it was.
    #[test]
    fn the_automatic_repair_is_not_sent_over_a_server_that_changed_meanwhile() {
        let then = a_program_at(Some(7));
        assert!(the_same_reading(&then.clone(), &then));
        assert!(!the_same_reading(&a_program_at(Some(8)), &then));
        assert!(!the_same_reading(&WhatThePackageAreaHolds::default(), &WhatThePackageAreaHolds::default()));
        let server = a_server();
        let mut desk = TheDesk::ending(7, TheRepairsEnding::TheServerChanged, &server);
        let did = repaired(&server, a_program_at(Some(7)), &mut desk);
        assert_eq!(what_the_server_carries(&server), None);
        assert!(!did.a_read_should_follow_at_once());
        assert!(!crate::setting_up::an_update_holds(&server));
    }

    /// **V3c: every way a placement can end is said**, and only *verified*
    /// lands.
    #[test]
    fn the_automatic_repair_reads_every_placement_ending() {
        assert_eq!(what_the_placement_came_to(Ok(WhatThePlacementReached::Verified)), TheRepairsEnding::ItLanded);
        for not in [
            Ok(WhatThePlacementReached::RolledBack),
            Ok(WhatThePlacementReached::FailedBeforeCommit { because: "x".into() }),
            Ok(WhatThePlacementReached::Deferred { because: "x".into() }),
            Ok(WhatThePlacementReached::AClaimStandsWhoseOwnerCouldNotBeRead),
            Err("No connection to the server.".to_string()),
        ] {
            assert!(matches!(what_the_placement_came_to(not), TheRepairsEnding::ItFailed { .. }));
        }
    }
}

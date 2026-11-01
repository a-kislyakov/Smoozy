//! **§15 performed — the preview worker's queueing per finished frame, the
//! ending's catch-up write, the live preview's one fetch in flight, the
//! journalled trigger and the read-forward, the fetch arranged through the
//! transfer contract, and the heading over the picture.**
//!
//! **The first two are the machine's and have lived in
//! `smoozy-queue-history-machine`'s `acting::previews` since 2026-09-19**, so
//! that the rented machine's own program can take them without this crate's
//! transfer contract. They are re-exported below at the paths they always had,
//! and the sections about them here describe the module as a whole.
//!
//! # What is on the machine and what is on the desk
//!
//! **P1 and P1b are the supervisor's**: the queueing of preview work, in frame
//! order, over records Smoozy wrote. **P2 to P5 and P10a are the desktop's**:
//! the trigger, the fetch, the holding and the heading. **The two never meet
//! in one process**, and nothing below crosses that line.
//!
//! # P1a's trigger is a record of Smoozy's own observation
//!
//! **The render worker observes each frame's save announcement as the bytes
//! pass its own copy loop** and reports **one figure** — the newest frame of
//! the attempt's list whose save has been announced — on R5's bounded
//! reporting. **On a report, the supervisor queues preview work, in frame
//! order, for every frame of that prefix whose chosen candidate has no
//! published derivative and no production standing.**
//!
//! **Nothing here walks a folder, reads a file's presence or reads a
//! free-space figure** (`State, identity and recovery` §1.3 rule 5, §11.4).
//! The prefix arithmetic is [`crate::previews::the_frames_owed_a_preview`]'s
//! and this module only performs what it answers.
//!
//! # P1b is what makes *no finished frame is passed over* true
//!
//! **In the same pass in which a recorded outcome that ends a job is written,
//! the supervisor reads the completeness reading carried with that outcome and
//! queues preview work for every frame it counts already there** whose chosen
//! candidate has no published derivative and no production standing.
//!
//! **It is a read of records Smoozy wrote** — the carried reading, and the
//! derivative operations' own records — never a walk of its own: the one walk
//! behind it is the ending step's, already taken (H7). **It recovers the
//! frames of a stream that printed nothing, the frames whose reports a
//! supervisor restart lost, an interrupted first attempt's frames, and every
//! production that failed for want of room.**
//!
//! # P4's fetch is arranged, never written
//!
//! **The loop is `Server protocol and transfers`' own** —
//! [`smoozy_server_protocol::transfers::moving::carry_out`] — and what this
//! module composes is the record that loop drives: one fetch per derivative,
//! staging, integrity, publication last, partial across derivatives and atomic
//! within one. **No transport, no digest and no publication step is written
//! here.**
//!
//! # P4a: a copy cleared by age is not fetched again
//!
//! **The fetch triggers on reading an event forward from the cursor** (P3),
//! and the cursor has long since passed the event for a derivative old enough
//! to be swept, **so nothing re-fetches by construction**. **The clearing
//! queues no fetch of its own**, and there is no function in this module that
//! takes a cleared copy and arranges anything.
//!
//! # What is unresolved here, recorded rather than rescued
//!
//! **The second rung's purpose is permanently unresolved** — no research ever
//! looked at a separate pass for what a failed texture leaves — and it costs
//! the fallback's value and nothing else. It stands at
//! [`crate::previews::THE_SECOND_RUNGS_PURPOSE_IS_PERMANENTLY_UNRESOLVED`] and
//! nothing here rebuilds it.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` B15, B17, B19, B20, B22,
//! B24, B29, B31, B31a. `Server protocol and transfers` T10, S15a.
//! `Rented-server runtime` S14, W12. `State, identity and recovery` §1.3, §7,
//! §9.5, §11.1, §11.3. `Desktop app and delivery` P10d.

use smoozy_server_protocol::transfers::{Direction, TransferRecord};
use smoozy_state::identities::Identity;

/// **P1, P1a and P1b are the machine's, and they live in the half of this
/// module the rented machine's own program may take** —
/// `smoozy-queue-history-machine`'s `acting::previews`, split out on
/// 2026-09-19. Each is re-exported here at the path it has always had.
pub use smoozy_queue_history_machine::acting::previews::{
    queue_at_the_endings_write, queue_on_a_report, WhatIsQueued,
    WhatTheRecordsSayAboutTheDerivatives, THE_CATCH_UP_RIDES_THE_ENDINGS_OWN_WRITE,
};

/// **P2's holder: at most one preview fetch for the running job's live preview
/// is in flight, taking the newest derivative the set holds at admission.**
///
/// **This is a rule about display and refresh rather than about production.**
/// A job rendering faster than the connection keeps up shows the newest
/// finished derivative on the running entry rather than every frame in turn —
/// **which is a preview and not a viewer** — and the frames it passed over are
/// in the set, produced by P1 and fetched by P4's ordinary per-derivative
/// route behind it.
#[derive(Debug, Default)]
pub struct TheLivePreviewsOneFetch {
    in_flight: Option<Identity>,
}

/// What asking to refresh the running entry's live preview gave.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheLivePreviewDid {
    /// **The newest derivative the set holds at admission** was taken.
    ItTook(Identity),
    /// **One is already in flight**, so nothing is taken and nothing queues
    /// behind it.
    OneIsAlreadyInFlight,
    /// The set holds nothing yet.
    TheSetHoldsNothing,
}

impl TheLivePreviewsOneFetch {
    /// **P2, performed: one at a time, newest at admission.**
    pub fn admit(&mut self, the_set_newest_first: &[Identity]) -> WhatTheLivePreviewDid {
        if self.in_flight.is_some() {
            return WhatTheLivePreviewDid::OneIsAlreadyInFlight;
        }
        match the_set_newest_first.first() {
            None => WhatTheLivePreviewDid::TheSetHoldsNothing,
            Some(newest) => {
                self.in_flight = Some(newest.clone());
                WhatTheLivePreviewDid::ItTook(newest.clone())
            }
        }
    }

    /// The fetch ended, however it ended. **A failed fetch frees the flight
    /// exactly as a published one does**, because P2 bounds what is in flight
    /// and says nothing about what succeeded.
    pub fn it_ended(&mut self) {
        self.in_flight = None;
    }

    pub fn what_is_in_flight(&self) -> Option<&Identity> {
        self.in_flight.as_ref()
    }
}

/// **P3: what the desktop learns of, and how.**
///
/// **No polling and no push**: the desktop learns of new derivatives by
/// reading forward from its cursor while connected, and at
/// `State, identity and recovery` §7 step 3 on a fresh connection.
/// ***While connected* is `Desktop app and delivery` P10d's live-reading
/// sequence**, whose read-forward repeats for the whole connected stretch and
/// which that clause states is not a poll.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ADerivativeWasPublished {
    pub job: Identity,
    /// The derivative's own production operation, which is what the event
    /// carries and what the fetch is keyed on.
    pub derivative: Identity,
    pub frame: i64,
    /// **B29 publishes over the name**, so a replaced derivative is journalled
    /// and re-fetched, replacing the held copy.
    pub it_replaces_one_already_held: bool,
}

/// **P3 and P5 performed: the fetches the read-forward queues.**
///
/// **Catching up is the same trigger on the next connection**, which is the
/// spec's own sentence: frames rendered while the user was away produce
/// derivatives while away, the connection's read-forward finds them, and the
/// fetches queue. **Previews already held show with nothing connected, from
/// the held copies**, and nothing here reaches for them.
///
/// **P4a: nothing re-fetches a copy cleared by age.** The cursor has long
/// since passed such an event, so no event of it arrives here; and **the
/// clearing queues no fetch of its own**, which is why this function takes
/// events and never a set of held copies.
pub fn the_fetches_the_read_forward_queues(
    applied: &[ADerivativeWasPublished],
) -> Vec<ADerivativeWasPublished> {
    applied.to_vec()
}

/// **P4, performed: one fetch per derivative, arranged for the transfer
/// contract's own loop.**
///
/// **What this composes is the record that loop drives** — the transfer
/// identity, the direction, the server row and its installation, the
/// derivative's path on the machine, the landing place on the user's own
/// computer, the staging directory and the size. **The loop is
/// `Server protocol and transfers`' and this module writes none of it**:
/// staging, integrity and publication-last are all that contract's.
///
/// **P6's class is declared beside it and never inferred** —
/// [`crate::previews::THE_PREVIEW_FETCHS_CLASS`], below every user-asked class
/// and below auto-download, never starved by T10.
#[allow(
    clippy::too_many_arguments,
    reason = "every one is a value the transfer contract's own record takes, and \
              `TransferRecord::arranged` carries the same allow for the same \
              reason: gathering them into a struct of this module's own would \
              be a second shape for the one that document already fixes"
)]
pub fn arrange_one_derivatives_fetch(
    transfer: Identity,
    server: Identity,
    installation: Identity,
    run: Identity,
    the_derivatives_path_on_the_machine: &str,
    where_it_lands_on_this_computer: &str,
    staging_directory: &str,
    size: u64,
) -> TransferRecord {
    TransferRecord::arranged(
        transfer,
        // **The machine to the user's own computer.**
        Direction::Down,
        server,
        Some(installation),
        the_derivatives_path_on_the_machine.to_string(),
        where_it_lands_on_this_computer.to_string(),
        staging_directory,
        size,
        run,
    )
}

/// **P10a: the heading over the picture.**
///
/// **The name is `Job files and outputs` B31a's**, recorded on the machine
/// with each derivative's production, only where the file holds more than one
/// view layer, and carried as the file's own bytes. **Where the copy carries
/// no name, the heading names the frame alone** — absent drawing as absent —
/// and **nothing composes a name from anything else and nothing reads a second
/// record to supply one.**
///
/// **The heading's name and the progress line's are two facts and are never
/// merged**: R10 and B3a name the view layer being rendered *now*; this names
/// the layer in the picture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheHeadingOverThePicture {
    /// The frame, and the view layer the picture came from.
    TheFrameAndItsViewLayer { frame: i64, view_layer: String },
    /// **The frame alone**: a single-view-layer file, a picture whose choosing
    /// establishes no layer, or a derivative produced before the rule.
    TheFrameAlone { frame: i64 },
}

/// **P10a, performed: read off the derivative's own record and off nothing
/// else.**
pub fn the_heading_over_the_picture(
    frame: i64,
    the_name_the_derivatives_record_carries: Option<&str>,
) -> TheHeadingOverThePicture {
    match the_name_the_derivatives_record_carries {
        Some(view_layer) if !view_layer.is_empty() => {
            TheHeadingOverThePicture::TheFrameAndItsViewLayer {
                frame,
                view_layer: view_layer.to_string(),
            }
        }
        _ => TheHeadingOverThePicture::TheFrameAlone { frame },
    }
}

/// **P7 and P8, stated as values.**
///
/// **P7: *throwaway* is true only on the machine.** The machine-side copy can
/// always be made again while the machine exists; the fetched copy is not
/// throwaway, because the ordinary end of a job is the machine being destroyed
/// and then the copy can never be made again. **The age ending is
/// `State, identity and recovery` §11.1's own constant and sweep, cited and
/// not rebuilt here.**
///
/// **P8: a derivative nothing ever fetches is one produced after the last
/// connection a machine ever gets**, and its clearing trigger is then the
/// job's record going or the machine's destruction — the ordinary end.
/// **Recorded as the seam's reach, not repaired.**
pub const NOTHING_HERE_CLEARS_A_FETCHED_COPY: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::IdentityKind;

    fn an_operation() -> Identity {
        Identity::issue(IdentityKind::Operation).expect("randomness")
    }





    /// **P2: one fetch in flight, newest at admission, and nothing queues
    /// behind it.**
    #[test]
    fn the_live_preview_takes_the_newest_and_holds_one_at_a_time() {
        let newest = an_operation();
        let older = an_operation();
        let mut flight = TheLivePreviewsOneFetch::default();
        assert_eq!(
            flight.admit(&[newest.clone(), older.clone()]),
            WhatTheLivePreviewDid::ItTook(newest.clone())
        );
        assert_eq!(
            flight.admit(&[newest.clone(), older]),
            WhatTheLivePreviewDid::OneIsAlreadyInFlight
        );
        assert_eq!(flight.what_is_in_flight(), Some(&newest));
        flight.it_ended();
        assert_eq!(flight.what_is_in_flight(), None);
        assert_eq!(flight.admit(&[]), WhatTheLivePreviewDid::TheSetHoldsNothing);
    }

    /// **P4: the fetch is arranged as the contract's own record, downward,
    /// naming both paths.**
    #[test]
    fn a_derivatives_fetch_is_arranged_downward_and_names_both_paths() {
        let record = arrange_one_derivatives_fetch(
            an_operation(),
            Identity::issue(IdentityKind::LocalServer).expect("randomness"),
            Identity::issue(IdentityKind::Installation).expect("randomness"),
            Identity::issue(IdentityKind::Run).expect("randomness"),
            "/ephemeral/smoozy/job-records/abc/derivatives/f_0147.jpg",
            "/Users/artist/Library/Smoozy/previews/abc/f_0147.jpg",
            "/Users/artist/Library/Smoozy/staging",
            4096,
        );
        assert_eq!(record.direction, Direction::Down);
        assert_eq!(
            record.source_path,
            "/ephemeral/smoozy/job-records/abc/derivatives/f_0147.jpg"
        );
        assert_eq!(
            record.final_path,
            "/Users/artist/Library/Smoozy/previews/abc/f_0147.jpg"
        );
        assert_eq!(record.total_size_expected, 4096);
        // **P6's class is declared and is the lowest.**
        const _: () =
            assert!(crate::previews::THE_PREVIEW_FETCHS_CLASS.below_every_user_asked_class);
        const _: () = assert!(crate::previews::THE_PREVIEW_FETCHS_CLASS.below_auto_download);
        const _: () = assert!(crate::previews::THE_PREVIEW_FETCHS_CLASS.never_starved);
    }

    /// **P10a: the heading names the layer where the record carries one, and
    /// the frame alone where it does not.**
    #[test]
    fn the_heading_names_the_layer_where_the_record_carries_one() {
        assert_eq!(
            the_heading_over_the_picture(147, Some("Reflections")),
            TheHeadingOverThePicture::TheFrameAndItsViewLayer {
                frame: 147,
                view_layer: "Reflections".to_string()
            }
        );
        assert_eq!(
            the_heading_over_the_picture(147, None),
            TheHeadingOverThePicture::TheFrameAlone { frame: 147 }
        );
        // **An empty name is an absence and never a heading with a blank in
        // it.**
        assert_eq!(
            the_heading_over_the_picture(147, Some("")),
            TheHeadingOverThePicture::TheFrameAlone { frame: 147 }
        );
    }

    /// **P4a: nothing in this module takes a held copy and arranges anything,
    /// and nothing clears one.** The guard is a scan of the working half —
    /// of this half, since the file was split on 2026-09-19; the machine half
    /// carries its own copy.
    #[test]
    fn nothing_here_re_fetches_a_cleared_copy_or_clears_one() {
        let source = include_str!("previews.rs");
        let working_half = source
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half");
        let code: String = working_half
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join(" ");
        for forbidden in ["age", "sweep", "clear", "expire"] {
            assert!(
                !code.contains(forbidden),
                "{forbidden} would be a clearing this module does not own"
            );
        }
        const _: () = assert!(NOTHING_HERE_CLEARS_A_FETCHED_COPY);
    }

    /// **P1: nothing here reads a folder, a file's presence or a free-space
    /// figure.** Over this half, as the guard above is.
    #[test]
    fn nothing_here_looks_at_the_filesystem_at_all() {
        let source = include_str!("previews.rs");
        let working_half = source
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half");
        let code: String = working_half
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join(" ");
        for forbidden in ["std::fs", "read_dir", "metadata", "free_space"] {
            assert!(!code.contains(forbidden), "{forbidden}");
        }
    }
}

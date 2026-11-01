//! **§5.1 P1 and P2 performed — one file's fetch arranged, offered and
//! carried out.**
//!
//! **The transfer is `Server protocol and transfers` §7's, taken entire and
//! not written a second time.** Staging inside the destination folder, the
//! record durable before the first byte, the byte loop, the integrity split,
//! the publication, the endings and their opposite cleanup rules — every one
//! of them is that document's loop, and what this file does is compose one
//! record, offer the unit at the per-server point, and hand it over. **There
//! is a working transport in this workspace and this module drives it rather
//! than composing another.**
//!
//! # What this module adds is the layer above, and it is three things
//!
//! **Which transfers exist** — the units owed, in P2's order. **What class
//! each carries** — [`crate::classes`]'s two, declared per action and never
//! decided at run time. **And what a failed unit becomes** — P3's rule, which
//! is that a fresh attempt is a new transfer with a new identity and that no
//! unit is ever retried inside its own action.
//!
//! # The order and the admission, and neither is re-decided here
//!
//! **P2: the fetch order within an arrangement is frame order, earliest
//! judged-finished first; within a manual request, the enumeration's order.**
//! Admission is by class then arrival at the per-server point, under
//! `File Browser` O1's pool, Q1's ceiling and O5's share — **and that point is
//! that document's own [`AdmissionPoint`](smoozy_file_browser::pool::AdmissionPoint),
//! consumed as it stands.** Nothing here re-orders, re-ranks or re-derives it.
//!
//! **R6's ordering requirement is a gate in front of the queue and not a rule
//! somebody keeps**: no transfer of this document is admitted until every
//! publication-pending record has been completed or its reservation removed.
//! [`crate::arrangement::transfers_may_be_admitted`] is the question and
//! [`TheFetchQueue::offer`] is the only place it is asked.
//!
//! **D6: while a destination condition stands, the arrangement that raised it
//! offers no work at all** — *"it occupies no slot at any admission point and
//! cannot hammer the mechanism"* — **and an arrangement whose own destination
//! is writable is untouched by another's.**
//!
//! **The hold is per unit and never per queue, and the second half of that
//! clause is why.** This queue is per **server** and holds both classes' units,
//! because Q1's ceiling has to be counted at the one per-server point. A
//! single hold on it reached every other arrangement's units and every manual
//! request's units at that server — so one job aimed at an external drive that
//! was unplugged stopped a second job downloading into the user's Downloads
//! folder, with nothing on its row saying why, and stopped a manual download
//! the user had started by hand a minute earlier. **So each unit carries the
//! destination folder its own arrangement or request was aimed at**, and the
//! standing condition keys sit on the queue —
//! [`crate::destination::what_the_arrangement_does`] asked per unit. **A held
//! unit is invisible to admission rather than merely passed over**, so a
//! grant is never spent on a class that cannot take it.
//!
//! # `Server protocol and transfers` S13's reading, and who takes it
//!
//! S13 records the four values — size, inode, modification time and change
//! time — **when a download is arranged**, and compares all four before a
//! resume. **The reading is a command-channel reading**, because revision 3's
//! attribute structure carries neither the inode nor the change time.
//!
//! **[`TheFourValues`] is the seam, and since 2026-10-04 (the 38's piece 9,
//! item 18) it has a producer**: `smoozy_desktop::downloading` reads the four
//! values over the server's own point, through the one reading every other
//! act of the application takes (`Render Settings` Q14 item 2's, at
//! submission and in the dependency check). *(This header said the reading was
//! permanently unresolved under the Creator's ruling of 2026-08-31, on the
//! ground that what program answers on an arbitrary rental is not
//! established. That was never the reason nothing took it: the same reading
//! is taken on every rental for other acts. What was missing was a caller.)*
//!
//! **What stands where the reading cannot be taken is unchanged**, and it is
//! S13's own branch: **with no reading the comparison answers *could not be
//! confirmed*, the resume does not happen, and the download starts over.**
//! That is the safe direction — bytes crossed twice, never a wrong file. **A
//! reading that cannot be taken is never read as *the file changed*.**
//!
//! **What it does not reach, said rather than left to be discovered:** R7's
//! newer-version telling compares a re-read of the four values against the
//! arrangement's own recorded ones (R4), and no arrangement records them, so
//! **it still never fires**; a re-run that re-writes a fetched name is caught
//! by R4's own predicted-path rule and by P6's third bullet at the exclusive
//! create instead — *nothing is written over* either way.
//!
//! # A path crosses this seam as text, and that is a routed question rather
//! than a decision taken here
//!
//! Every path in this crate is bytes, which is `Job files and outputs` N15's
//! and `File Browser` L1a's rule.
//! [`TransferRecord`](smoozy_server_protocol::transfers::TransferRecord) holds
//! its two paths as text. **Nothing here converts lossily**: a path that is
//! not valid text refuses the arrangement with
//! [`TheNameCannotBeCarried`], which is the same value and the same refusal
//! `File Browser`'s own upload arrangement already takes, and that crate has
//! the question parked.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S1, S4, S8, S9,
//! S12, S13, S15, S15b, S18, S21, S25, S26, T3, T4, T5, T8, T10.
//! `File Browser` O1, O4, O5, O7, O8,
//! §13. `Desktop app and delivery` C13, R17. `State, identity and recovery`
//! §9.5. `Job files and outputs` N15.

use smoozy_file_browser::pool::AdmissionPoint;
use smoozy_server_protocol::transfers::publication::Intent;
use smoozy_server_protocol::transfers::{Direction, SourceReading, TransferRecord};
use smoozy_state::identities::Identity;

use crate::classes::{the_class, WhatArrangedIt, THE_DOWNLOAD_DIRECTIONS_CEILING};
use crate::destination::{what_the_arrangement_does, TheConditionKey, WhatTheArrangementDoes};
use crate::fetching::the_staging_directory_inside;

/// **`Server protocol and transfers` S13's four-value reading of a file on the
/// machine.**
///
/// **This crate does not compose one**: the reading is a command on the
/// server's own point, and its producer is the desktop's
/// (`smoozy_desktop::downloading`, since the 38's piece 9). See this module's
/// own header for what stands where it cannot be taken.
pub trait TheFourValues {
    /// The four values for one path on the machine, **or nothing where the
    /// reading could not be taken** — which is S13's *could not be confirmed*
    /// and never a reading of zeros.
    fn read_the_four_values(
        &mut self,
        path: &[u8],
    ) -> impl std::future::Future<Output = Option<SourceReading>> + Send;
}

/// **A path that cannot cross into the transfer record's own text.**
///
/// The refusal is `File Browser`'s in the other direction, word for word: the
/// enumeration holds paths as bytes, the record holds them as text, and
/// **nothing lossily converts**. The unit is refused with the path named and
/// the rest of the request or arrangement is untouched, which is the partition
/// unit the declarations table already fixes at one file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheNameCannotBeCarried {
    /// Which of the two paths it was, so the telling can say where.
    pub which: WhichPath,
    pub path: Vec<u8>,
}

/// Which of the transfer record's two paths could not be carried.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichPath {
    /// The file's path on the machine.
    OnTheMachine,
    /// The landing path on the user's own computer.
    WhereItLands,
}

fn as_text(path: &[u8], which: WhichPath) -> Result<String, TheNameCannotBeCarried> {
    String::from_utf8(path.to_vec()).map_err(|_| TheNameCannotBeCarried {
        which,
        path: path.to_vec(),
    })
}

/// **P1, performed: one fetch composed as one transfer under §7.**
///
/// **What this composes is the record the contract's own loop drives**, and
/// nothing else: the transfer identity (T4), the direction, the server row
/// (T3) and its installation identity, the path on the machine, the landing
/// path, the staging directory **inside the user's own destination folder**
/// (`Server protocol and transfers` S1, and `Server protocol and transfers`
/// S28), the size, and the run that wrote it.
///
/// **The staging directory is composed from the destination and the reserved
/// name `File Browser` already declares** — one fact, one home — and the
/// staging path itself is S4's synthesis from the transfer identity, made by
/// that document's own type and never by a name of this one's.
///
/// **Nothing here writes the record down.** S8's ordering — durable before the
/// first byte — is the caller's, and the keeper is
/// [`KeepsTheRecord`](smoozy_server_protocol::transfers::moving::KeepsTheRecord),
/// whose real end is the local record's.
#[allow(
    clippy::too_many_arguments,
    reason = "every one is a value the contract's own record takes, and \
              `TransferRecord::arranged` carries the same allow for the same \
              reason: gathering them into a shape of this module's own would \
              be a second shape for the one that document already fixes"
)]
pub fn arrange_one_fetch(
    transfer: Identity,
    server_row: Identity,
    installation: Option<Identity>,
    the_path_on_the_machine: &[u8],
    the_landing_path: &[u8],
    the_destination_folder: &[u8],
    size: u64,
    written_by_run: Identity,
) -> Result<TransferRecord, TheNameCannotBeCarried> {
    let source = as_text(the_path_on_the_machine, WhichPath::OnTheMachine)?;
    let landing = as_text(the_landing_path, WhichPath::WhereItLands)?;
    let staging = as_text(
        &the_staging_directory_inside(the_destination_folder),
        WhichPath::WhereItLands,
    )?;
    Ok(TransferRecord::arranged(
        transfer,
        Direction::Down,
        server_row,
        installation,
        source,
        landing,
        &staging,
        size,
        written_by_run,
    ))
}

/// **P5: every publication of this document is create-intended.**
///
/// The taken question is the exclusive create of the final name
/// (`Desktop app and delivery` C10) and **this document never answers
/// *replace* on its own**. The one deliberate replacement is a manual
/// request's user-answered **Replace**, which publishes with the replacing
/// rename and takes no reservation (`Desktop app and delivery` C14).
///
/// **So the intent is read off the request's own disposition and off nothing
/// else**, and an arrangement has no disposition to read: every one of its
/// fetches is create-intended.
pub fn the_intent(the_user_answered_replace_for_this_unit: bool) -> Intent {
    if the_user_answered_replace_for_this_unit {
        Intent::ReplaceIntended
    } else {
        Intent::CreateIntended
    }
}

/// **P1 performed: one file's transfer carried out under §7, unmodified.**
///
/// **This is the whole of what this document does to the bytes.** The staging,
/// the record written before the first byte (S8, S9), the byte loop, the
/// verified checkpointed resume (S12) with S13's four values compared first,
/// the end-to-end integrity split (`Server protocol and transfers` S21 to
/// `Server protocol and transfers` S25), the publication as one rename (S15,
/// and `Server protocol and transfers` S18) and the ending (S26) are every one
/// of them that document's own loop, and this hands the record to it and hands
/// back what happened.
///
/// **The gate-less entry point is the one taken, and that is the ordinary
/// case**: `Server protocol and transfers` S15b's publication gate is for a
/// caller reading a source on the user's own computer, and **a download has no
/// such source**. So no gate is
/// named, no field stands in for one, and nothing here composes a second
/// no-corruption test.
///
/// **The record is the caller's and is written by the caller's keeper**, which
/// is S8's ordering: durable before the first byte, through
/// [`KeepsTheRecord`](smoozy_server_protocol::transfers::moving::KeepsTheRecord)
/// whose real end is the local record's.
///
/// **`the_source_now` is S13's four-value reading of the file on the machine,
/// taken by the caller through [`TheFourValues`] just before a resume**, and
/// nothing where no resume is in question or the reading could not be taken —
/// which `Server protocol and transfers` reads as S13's *could not be
/// confirmed*, so the file starts over. Added 2026-10-04, the 38's piece 9
/// (item 18).
pub async fn carry_out_one_fetch<S, L, B, K, W>(
    transfer: smoozy_server_protocol::transfers::moving::Transfer<'_, S, L, B, K, W>,
    record: &mut TransferRecord,
    the_source_now: Option<SourceReading>,
) -> smoozy_server_protocol::transfers::moving::WhatHappened
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
    L: smoozy_server_protocol::transfers::moving::TheUsersOwnComputer,
    B: smoozy_server_protocol::transfers::moving::DrawsFromTheSharedBudget,
    K: smoozy_server_protocol::transfers::moving::KeepsTheRecord,
    W: smoozy_server_protocol::transfers::moving::WatchesTheTransfer,
{
    smoozy_server_protocol::transfers::moving::carry_out_with_the_source_as_it_stands(
        transfer,
        record,
        the_source_now,
    )
    .await
}

/// **What one fetch's ending makes of its reckoning entry.**
///
/// **The endings are `Server protocol and transfers` S26's five and are read
/// as that document writes them**, never re-derived: only *published* is a
/// fetch that arrived, and only *interrupted* means the work may be carried
/// on. **Nothing here reads a status, a byte count or a digest** — the ending
/// is the answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheFetchCameTo {
    /// **R4's *fetched*, and R9 makes it final**: the file is never fetched
    /// again, whatever becomes of the local copy.
    ItIsFetched,
    /// **The unit is owed again.** P3: a unit that failed for a server-side or
    /// transfer reason returns to *owed* and is re-arranged at a later walk
    /// cycle, **as a new transfer with a new identity**
    /// (`State, identity and recovery` §9.5) — and never retried inside its own
    /// action.
    ItIsOwedAgain,
    /// **P6's *left alone*, or the user's own Stop.** The transfer ended
    /// *stopped*, which is an ending Smoozy wrote itself, so it never composes
    /// as waiting (M11) and nothing re-arranges it. **What the entry becomes is
    /// P6's classification and is not decided here** — this value says only
    /// that the unit is finished with and the caller has a disposition to
    /// write.
    ItStoppedAndTheCallerSaysWhatTheEntryBecomes,
}

/// **The ending read, and it is a total function over S26's five.**
///
/// *Stalled* is read as owed again rather than as an ending Smoozy wrote: a
/// channel's inactivity deadline expiring is a transfer that did not finish
/// for a transfer reason, which is exactly P3's *"a unit that failed for a
/// server-side or transfer reason returns to owed"*.
pub fn what_the_fetch_came_to(
    ending: &smoozy_server_protocol::transfers::Ending,
) -> WhatTheFetchCameTo {
    use smoozy_server_protocol::transfers::Ending;
    match ending {
        Ending::Published => WhatTheFetchCameTo::ItIsFetched,
        Ending::Stopped(_) => WhatTheFetchCameTo::ItStoppedAndTheCallerSaysWhatTheEntryBecomes,
        Ending::Failed | Ending::Stalled | Ending::Interrupted => WhatTheFetchCameTo::ItIsOwedAgain,
    }
}

/// One unit waiting to be fetched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AUnitOwed {
    /// The unit's own key — the predicted path for an arrangement, the
    /// enumerated entry's source path for a request.
    pub key: Vec<u8>,
    /// **P2's ordering value.** For an arrangement it is the frame's place in
    /// the attempt's order, so *earliest judged-finished first* is that order
    /// and not the frame's number; for a request it is the enumeration's own
    /// position (`File Browser` O7).
    pub in_order_at: usize,
    /// **Q3: the class is declared per action, here, and nothing decides one
    /// at run time.**
    pub arranged_by: WhatArrangedIt,
    /// **D6: the destination folder this unit's own arrangement or request was
    /// aimed at, in the normalised form the condition key uses.**
    ///
    /// **The hold is this unit's own destination and never the queue's**, and
    /// that is what makes D6's *that arrangement, and no other* keepable at a
    /// queue that is per **server** and holds both classes' units. Q1's
    /// ceiling has to be counted at one per-server point — *"the count the
    /// machine sees is one count at the one per-server point"* — so the queue
    /// cannot be split per arrangement, and a single flag on it held every
    /// other arrangement's units and every manual request's units too.
    ///
    /// **A manual request's unit carries its request's destination and is
    /// tested the same way**, which is D6's last sentence.
    pub the_destination_normalised: Vec<u8>,
    /// **P11's landing path, composed once when the unit was owed.**
    ///
    /// **For a whole job it holds the job's own folder between the destination
    /// and the file** (P10, M9), and the one composition both routes come
    /// through is
    /// [`crate::fetching::the_landing_path_inside_the_jobs_folder`]. It is
    /// carried on the unit rather than recomposed at the transfer, so the path
    /// the name check judged is the path the fetch lands at.
    pub the_landing_path: Vec<u8>,
}

/// **The per-server point's queue for this document's two classes.**
///
/// **It holds units and asks `File Browser`'s own admission point which class
/// takes the next grant**; it decides no order between the classes, because
/// O4 already declares one and Q2 confirms it by citation.
///
/// **The ceiling is Q1's two, inside `File Browser` O1's pool of four**, and
/// it is a ceiling on this direction rather than a pool of its own — *"two
/// ceilings in one pool, not two pools"* — so nothing here opens a channel or
/// counts the pool.
pub struct TheFetchQueue {
    owed: Vec<AUnitOwed>,
    in_flight: usize,
    point: AdmissionPoint,
    /// **D6: the destination conditions standing at this moment, by D4's own
    /// key.**
    ///
    /// **The queue keeps the keys and each unit is tested against its own
    /// destination**, which is [`crate::destination::what_the_arrangement_does`]
    /// asked per unit rather than once for the whole queue. **A held unit is
    /// never offered and never occupies a slot**, so D6's *it occupies no slot
    /// at any admission point* still holds; what has changed is that a
    /// condition on one arrangement's folder no longer reaches another's.
    the_standing_conditions: Vec<TheConditionKey>,
}

impl TheFetchQueue {
    /// A queue over the declared order, which is `File Browser` O4's whole
    /// five and not this document's two — **the share rule is over all of
    /// them** and a point that knew only two classes would deliver a different
    /// bound.
    pub fn at_a_server() -> TheFetchQueue {
        TheFetchQueue {
            owed: Vec::new(),
            in_flight: 0,
            point: AdmissionPoint::over(smoozy_file_browser::pool::THE_DECLARED_ORDER.to_vec()),
            the_standing_conditions: Vec::new(),
        }
    }

    /// **P2's order, applied once as the unit joins.** An arrangement's units
    /// are ordered by the attempt's order and a request's by the enumeration,
    /// and both arrive as [`AUnitOwed::in_order_at`].
    pub fn owe(&mut self, unit: AUnitOwed) {
        let at = self
            .owed
            .iter()
            .position(|already| {
                already.arranged_by == unit.arranged_by && already.in_order_at > unit.in_order_at
            })
            .unwrap_or(self.owed.len());
        self.owed.insert(at, unit);
    }

    /// **D4: one condition raised, keyed on the refusal's kind and the
    /// destination folder's own normalised path.**
    ///
    /// **A key already standing is not raised twice**, which is what D4's key
    /// buys — a pool of transfers failing at the same moment for the same
    /// reason resolves to one condition.
    pub fn a_condition_was_raised(&mut self, key: TheConditionKey) {
        if !self.the_standing_conditions.contains(&key) {
            self.the_standing_conditions.push(key);
        }
    }

    /// **D5 and D8: the condition is withdrawn by the next real write to that
    /// place succeeding, and by nothing else.**
    ///
    /// **A withdrawn record is spent**: a destination that works and later
    /// fails again raises a fresh identity against the same key.
    pub fn a_condition_was_withdrawn(&mut self, key: &TheConditionKey) {
        self.the_standing_conditions
            .retain(|already| already != key);
    }

    /// Whether a unit may be offered at all, **asked of its own destination**.
    ///
    /// **D6 read per unit**: an arrangement whose own destination folder is
    /// writable is untouched by another's condition, which is what D4's folder
    /// key buys and what a volume key would have taken away.
    fn it_is_held(&self, unit: &AUnitOwed) -> bool {
        matches!(
            what_the_arrangement_does(
                &unit.the_destination_normalised,
                &self.the_standing_conditions
            ),
            WhatTheArrangementDoes::ItHoldsItsOwedUnitsAndKeepsOnlyItsWatchAndItsProbe
        )
    }

    /// How many units are still owed.
    pub fn owed(&self) -> usize {
        self.owed.len()
    }

    /// **The one offer, and the three gates in front of it.**
    ///
    /// **R6's ordering requirement**: no transfer is admitted while any
    /// publication-pending record stands. **Q1's ceiling**: at most two of
    /// this direction's transfers are in flight at one server. **D6**: a held
    /// arrangement offers nothing.
    ///
    /// Where all three pass, the class the point grants takes the next unit of
    /// that class in P2's order. **A grant the queue has no unit for is not
    /// spent** — the point is asked with the classes actually waiting, so it
    /// never grants to a class with nothing behind it.
    pub fn offer(&mut self, the_transfer_records_standing: &[TransferRecord]) -> Option<AUnitOwed> {
        if !crate::arrangement::transfers_may_be_admitted(the_transfer_records_standing) {
            return None;
        }
        if self.in_flight >= THE_DOWNLOAD_DIRECTIONS_CEILING {
            return None;
        }
        // **A held unit is invisible to admission, not merely passed over.**
        // The classes offered to the point are the classes of units that may
        // actually be taken, so a grant is never spent on a class whose only
        // waiting units are held — which is D6's *it occupies no slot at any
        // admission point*, kept per unit rather than per queue.
        let mut waiting: Vec<&'static str> = Vec::new();
        for unit in self.owed.iter().filter(|unit| !self.it_is_held(unit)) {
            let class = the_class(unit.arranged_by);
            if !waiting.contains(&class) {
                waiting.push(class);
            }
        }
        let granted = self.point.grant(&waiting)?;
        let at = self
            .owed
            .iter()
            .position(|unit| the_class(unit.arranged_by) == granted && !self.it_is_held(unit))?;
        self.in_flight += 1;
        Some(self.owed.remove(at))
    }

    /// **A unit that ended gives its slot back**, whatever it ended as. The
    /// slot is a slot in this direction's ceiling and not in the pool, which
    /// is `File Browser` O2's structure.
    pub fn a_unit_ended(&mut self) {
        self.in_flight = self.in_flight.saturating_sub(1);
    }

    /// **P3: for an arrangement, a unit that failed for a server-side or
    /// transfer reason returns to *owed* and is re-arranged at a later walk
    /// cycle** — *"the arrangement is standing work, and each fresh fetch is
    /// its own action"*. **For a manual request, a failed unit stays failed
    /// with its per-file Try again being the user's act.**
    ///
    /// So this takes the unit back only for an arrangement, and the caller
    /// hands it the unit it had.
    pub fn it_failed_and_may_come_back(&mut self, unit: AUnitOwed) {
        self.a_unit_ended();
        if matches!(unit.arranged_by, WhatArrangedIt::AnAutoDownloadArrangement) {
            self.owe(unit);
        }
    }

    /// How many are in flight, which is what the ceiling bounds.
    pub fn in_flight(&self) -> usize {
        self.in_flight
    }
}

#[cfg(test)]
mod tests {
    use smoozy_server_protocol::transfers::publication::PublicationPreState;
    use smoozy_server_protocol::transfers::StableFileIdentity;
    use smoozy_state::identities::IdentityKind;

    use super::*;

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    const DOWNLOADS: &[u8] = b"/Users/a/Downloads";

    fn a_unit(key: &str, at: usize, arranged_by: WhatArrangedIt) -> AUnitOwed {
        a_unit_aimed_at(key, at, arranged_by, DOWNLOADS)
    }

    fn a_unit_aimed_at(
        key: &str,
        at: usize,
        arranged_by: WhatArrangedIt,
        destination: &[u8],
    ) -> AUnitOwed {
        AUnitOwed {
            key: key.as_bytes().to_vec(),
            in_order_at: at,
            arranged_by,
            the_destination_normalised: destination.to_vec(),
            the_landing_path: crate::fetching::the_landing_path(destination, key.as_bytes()),
        }
    }

    /// **P1: the staging directory is inside the user's own destination
    /// folder**, which is `Server protocol and transfers` S1 and
    /// `Server protocol and transfers` S28's second
    /// bullet, and the staging path itself is S4's synthesis from the transfer
    /// identity rather than from any file's name.
    #[test]
    fn a_fetch_stages_inside_the_destination_folder_and_never_beside_the_file() {
        let record = arrange_one_fetch(
            an_identity(IdentityKind::Transfer),
            an_identity(IdentityKind::LocalServer),
            None,
            b"/jobs/abc/0001.exr",
            b"/Users/a/Downloads/job/0001.exr",
            b"/Users/a/Downloads/job",
            4096,
            an_identity(IdentityKind::Run),
        )
        .expect("both paths are text");

        assert_eq!(record.direction, Direction::Down);
        assert_eq!(
            record.staging_path.directory(),
            "/Users/a/Downloads/job/.smoozy-staging"
        );
        assert!(!record.staging_path.as_text().contains("0001.exr"));
        assert_eq!(record.final_path, "/Users/a/Downloads/job/0001.exr");
        assert_eq!(record.total_size_expected, 4096);
    }

    /// **Nothing converts a path lossily**, and the refusal names which of the
    /// two paths it was so the telling can say where.
    #[test]
    fn a_path_that_is_not_text_refuses_the_arrangement_rather_than_being_mangled() {
        let refused = arrange_one_fetch(
            an_identity(IdentityKind::Transfer),
            an_identity(IdentityKind::LocalServer),
            None,
            &[b'/', 0xff, 0xfe],
            b"/Users/a/Downloads/job/x",
            b"/Users/a/Downloads/job",
            1,
            an_identity(IdentityKind::Run),
        )
        .expect_err("a name that is not text cannot be carried");
        assert_eq!(refused.which, WhichPath::OnTheMachine);
    }

    /// **P5: every publication of this document is create-intended, and the
    /// one deliberate replacement is a manual request's user-answered
    /// Replace.**
    #[test]
    fn the_intent_is_create_intended_unless_the_user_answered_replace() {
        assert_eq!(the_intent(false), Intent::CreateIntended);
        assert_eq!(the_intent(true), Intent::ReplaceIntended);
    }

    /// **R6's ordering requirement, kept as a gate in front of the queue:** no
    /// transfer is admitted while a publication-pending record stands.
    #[test]
    fn no_unit_is_offered_while_a_publication_pending_record_stands() {
        let mut queue = TheFetchQueue::at_a_server();
        queue.owe(a_unit("a", 0, WhatArrangedIt::AnAutoDownloadArrangement));

        let mut pending = TransferRecord::arranged(
            an_identity(IdentityKind::Transfer),
            Direction::Down,
            an_identity(IdentityKind::LocalServer),
            None,
            "/jobs/abc/x".into(),
            "/Users/a/x".into(),
            "/Users/a/.smoozy-staging",
            1,
            an_identity(IdentityKind::Run),
        );
        pending.publication_pending = Some(PublicationPreState {
            intent: Intent::CreateIntended,
            staging_path: pending.staging_path.as_text(),
            final_path: pending.final_path.clone(),
            staged_identity: StableFileIdentity::TheSupplierCouldNotAnswer,
            staged_byte_length: 1,
            staged_digest: smoozy_server_protocol::transfers::integrity::Digest::of(b"x"),
        });

        assert_eq!(queue.offer(std::slice::from_ref(&pending)), None);
        assert!(queue.offer(&[]).is_some());
    }

    /// **Q1's ceiling is two, inside a pool of four**, and the queue never
    /// offers a third at one server.
    #[test]
    fn at_most_two_of_this_directions_transfers_are_in_flight_at_one_server() {
        let mut queue = TheFetchQueue::at_a_server();
        for at in 0..5 {
            queue.owe(a_unit(
                &format!("f{at}"),
                at,
                WhatArrangedIt::AnAutoDownloadArrangement,
            ));
        }
        assert!(queue.offer(&[]).is_some());
        assert!(queue.offer(&[]).is_some());
        assert_eq!(queue.in_flight(), 2);
        assert_eq!(queue.offer(&[]), None);

        queue.a_unit_ended();
        assert!(queue.offer(&[]).is_some());
    }

    fn a_condition_on(destination: &[u8]) -> TheConditionKey {
        crate::destination::the_condition_key(
            crate::destination::WhatTheAdapterSaid::TheDestinationIsNoLongerThere,
            destination.to_vec(),
        )
    }

    /// **D6: while the condition stands the arrangement offers no work at
    /// all** — it occupies no slot at any admission point — and the
    /// withdrawal releases it.
    #[test]
    fn a_held_arrangement_offers_nothing_and_takes_no_slot() {
        let mut queue = TheFetchQueue::at_a_server();
        queue.owe(a_unit("a", 0, WhatArrangedIt::AnAutoDownloadArrangement));
        queue.a_condition_was_raised(a_condition_on(DOWNLOADS));

        assert_eq!(queue.offer(&[]), None);
        assert_eq!(queue.in_flight(), 0);
        assert_eq!(queue.owed(), 1);

        queue.a_condition_was_withdrawn(&a_condition_on(DOWNLOADS));
        assert!(queue.offer(&[]).is_some());
    }

    /// **D6's second half: an arrangement whose own destination folder is
    /// writable is untouched by another's condition.**
    ///
    /// Two jobs on one server, auto-download on for both, one aimed at an
    /// external drive and one at Downloads. **The drive is unplugged.** The
    /// queue is per server and holds both, so a hold on the queue itself
    /// stopped the second job's frames arriving with nothing wrong with its
    /// folder and nothing on its row saying why.
    #[test]
    fn one_arrangements_condition_never_reaches_another_arrangements_units() {
        let drive: &[u8] = b"/Volumes/work";
        let mut queue = TheFetchQueue::at_a_server();
        queue.owe(a_unit_aimed_at(
            "on the drive",
            0,
            WhatArrangedIt::AnAutoDownloadArrangement,
            drive,
        ));
        queue.owe(a_unit_aimed_at(
            "in Downloads",
            1,
            WhatArrangedIt::AnAutoDownloadArrangement,
            DOWNLOADS,
        ));
        queue.a_condition_was_raised(a_condition_on(drive));

        let taken = queue.offer(&[]).expect("the healthy unit is granted");
        assert_eq!(taken.key, b"in Downloads".to_vec());
        // And the held one is still owed rather than lost.
        assert_eq!(queue.offer(&[]), None);
        assert_eq!(queue.owed(), 1);
    }

    /// **D6's last sentence: a manual request's units are tested the same
    /// way**, so a hand-started download into a healthy folder is untouched by
    /// an arrangement's condition on another folder.
    ///
    /// **And a grant is never spent on a class whose only waiting units are
    /// held.** Auto-download outranks the manual class at `File Browser` O4's
    /// own rank, so a queue that offered the classes without the hold in mind
    /// would grant to auto-download and then hand back nothing.
    #[test]
    fn a_manual_requests_unit_is_untouched_by_an_arrangements_condition_elsewhere() {
        let drive: &[u8] = b"/Volumes/work";
        let mut queue = TheFetchQueue::at_a_server();
        queue.owe(a_unit_aimed_at(
            "the arrangement's",
            0,
            WhatArrangedIt::AnAutoDownloadArrangement,
            drive,
        ));
        queue.owe(a_unit_aimed_at(
            "the user's own",
            0,
            WhatArrangedIt::AManualRequest,
            DOWNLOADS,
        ));
        queue.a_condition_was_raised(a_condition_on(drive));

        let taken = queue.offer(&[]).expect("the manual unit is granted");
        assert_eq!(taken.arranged_by, WhatArrangedIt::AManualRequest);
        assert_eq!(taken.key, b"the user's own".to_vec());
    }

    /// **P2: within an arrangement the order is the attempt's order, applied
    /// as the unit joins** — so a unit owed later but earlier in the order is
    /// fetched first.
    #[test]
    fn the_units_come_out_in_the_order_they_were_given_and_not_the_order_they_arrived() {
        let mut queue = TheFetchQueue::at_a_server();
        queue.owe(a_unit("late", 7, WhatArrangedIt::AnAutoDownloadArrangement));
        queue.owe(a_unit(
            "early",
            2,
            WhatArrangedIt::AnAutoDownloadArrangement,
        ));

        let first = queue.offer(&[]).expect("a unit is granted");
        assert_eq!(first.key, b"early".to_vec());
    }

    /// **P3: an arrangement's failed unit returns to owed and is re-arranged
    /// later; a manual request's stays failed and re-asking is the user's
    /// act.**
    #[test]
    fn a_failed_unit_comes_back_for_an_arrangement_and_never_for_a_request() {
        let mut queue = TheFetchQueue::at_a_server();
        let arranged = a_unit("a", 0, WhatArrangedIt::AnAutoDownloadArrangement);
        queue.owe(arranged.clone());
        let taken = queue.offer(&[]).expect("a unit is granted");
        queue.it_failed_and_may_come_back(taken);
        assert_eq!(queue.owed(), 1);

        let mut queue = TheFetchQueue::at_a_server();
        let asked = a_unit("b", 0, WhatArrangedIt::AManualRequest);
        queue.owe(asked);
        let taken = queue.offer(&[]).expect("a unit is granted");
        queue.it_failed_and_may_come_back(taken);
        assert_eq!(queue.owed(), 0);
    }

    /// **S26's five endings, read as that document writes them**: only
    /// *published* is a fetch that arrived, only an ending Smoozy wrote itself
    /// stops the unit, and everything else is owed again as a new transfer.
    #[test]
    fn the_ending_is_read_as_the_contract_writes_it_and_never_re_derived() {
        use smoozy_server_protocol::transfers::{Ending, StoppedReason};
        assert_eq!(
            what_the_fetch_came_to(&Ending::Published),
            WhatTheFetchCameTo::ItIsFetched
        );
        assert_eq!(
            what_the_fetch_came_to(&Ending::Stopped(StoppedReason::AbandonedByItsRequest)),
            WhatTheFetchCameTo::ItStoppedAndTheCallerSaysWhatTheEntryBecomes
        );
        for owed in [Ending::Failed, Ending::Stalled, Ending::Interrupted] {
            assert_eq!(
                what_the_fetch_came_to(&owed),
                WhatTheFetchCameTo::ItIsOwedAgain
            );
        }
    }

    /// **Q2 and Q4: the order between the classes is `File Browser` O4's and
    /// the share is O5's**, and the point this queue asks is that document's
    /// own over its whole five — a point that knew only this document's two
    /// would deliver a different bound.
    #[test]
    fn auto_download_takes_the_grant_over_manual_download_by_o4s_own_rank() {
        let mut queue = TheFetchQueue::at_a_server();
        queue.owe(a_unit("manual", 0, WhatArrangedIt::AManualRequest));
        queue.owe(a_unit("auto", 0, WhatArrangedIt::AnAutoDownloadArrangement));

        let first = queue.offer(&[]).expect("a unit is granted");
        assert_eq!(first.arranged_by, WhatArrangedIt::AnAutoDownloadArrangement);
    }

    /// **O5's share, taken as declared**: a class waiting throughout the last
    /// N grants takes the next, so the lower-ranked class is never starved.
    /// The bound is 2N−1 over the five declared classes, which is nine.
    #[test]
    fn the_lower_ranked_download_class_is_never_starved() {
        let mut queue = TheFetchQueue::at_a_server();
        for at in 0..20 {
            queue.owe(a_unit(
                &format!("a{at}"),
                at,
                WhatArrangedIt::AnAutoDownloadArrangement,
            ));
            queue.owe(a_unit(
                &format!("m{at}"),
                at,
                WhatArrangedIt::AManualRequest,
            ));
        }
        let mut manual_at = None;
        for grant in 0..20 {
            let unit = queue.offer(&[]).expect("a unit is granted");
            if matches!(unit.arranged_by, WhatArrangedIt::AManualRequest) && manual_at.is_none() {
                manual_at = Some(grant + 1);
            }
            queue.a_unit_ended();
        }
        let bound = smoozy_file_browser::pool::the_bound(smoozy_file_browser::pool::n_over(
            &smoozy_file_browser::pool::THE_DECLARED_ORDER,
        ));
        assert!(
            manual_at.is_some_and(|at| at <= bound),
            "the manual class waited longer than the declared bound of {bound}"
        );
    }
}

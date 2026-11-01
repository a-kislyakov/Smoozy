//! The stores on the user's computer — `State, identity and recovery` §1.1 and
//! §1.3.
//!
//! J5: **the local record is the only durable store on this side that holds
//! state.** Everything else durable there is a secret, material, a swept
//! throwaway area, or evidentiary material. **A builder that needs a second
//! state store has found a fault in this section rather than a case it does
//! not cover**, so the list below is closed and the way to add to it is to go
//! back to the plan.
//!
//! What is deliberately not modelled here is the local record's own inventory
//! of rows. That inventory is §1.1 J1's, its one home is that section, and
//! `Desktop app and delivery` L1 builds the database to it — *"No row is added
//! to that inventory here."* The same rule holds in code: the record's tables
//! are the database's, beside the module that owns each row's shape.

/// The durable things on the user's computer — §1.1 J1's table, as the set of
/// places rather than as the rows inside them.
///
/// Each is a **place**, and §11.1's five lifetime classes describe **contents**
/// — R6 makes that distinction in as many words: *"This table governs things
/// held inside a store; it has never described a store."*
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DurablePlace {
    /// Every durable state fact on this side. `Desktop app and delivery`
    /// builds it; `Security and licence` §9 protects it — unencrypted,
    /// per-user, and holding no secret value.
    LocalRecord,
    /// The four secret kinds and nothing else. `Security and licence` §3.
    TheOperatingSystemsSecretStore,
    /// The small viewable copies fetched from a machine, one set per job
    /// record. Their age ending is `crate::lifetimes`'.
    FetchedDerivativeArea,
    /// Download material before publication. `Server protocol and transfers`';
    /// **the material, not state**.
    StagedTransferArea,
    /// `Security and licence` §7's area. Nothing this architecture does writes
    /// a file into it any longer — `Security and licence` T2's per-attempt
    /// file went when the connection stopped being an invoked client — and the
    /// sweep at application start stands, removing whatever is found and
    /// reading none of it.
    ThrowawayKnownHostsArea,
    /// One rolling line-oriented plain-text record of what Smoozy did, capped
    /// at 20 MB. **Not state, and no state fact is ever read back out of it.**
    /// `Reporting a problem` §1.
    DiaryArea,
    /// The throwaway key pairs `Keys` T2's type probe writes, swept at
    /// application start. `Desktop app and delivery` L3a.
    SmoozysOwnTemporaryArea,
}

impl DurablePlace {
    /// The whole set. J5's closure is what makes writing this out safe: a
    /// place missing from here is a place nothing on this side may write to.
    pub const ALL: [DurablePlace; 7] = [
        DurablePlace::LocalRecord,
        DurablePlace::TheOperatingSystemsSecretStore,
        DurablePlace::FetchedDerivativeArea,
        DurablePlace::StagedTransferArea,
        DurablePlace::ThrowawayKnownHostsArea,
        DurablePlace::DiaryArea,
        DurablePlace::SmoozysOwnTemporaryArea,
    ];

    /// Whether the place holds **state** — a fact anything branches on. Only
    /// the local record does (J5).
    pub fn holds_state(self) -> bool {
        matches!(self, DurablePlace::LocalRecord)
    }

    /// `Desktop app and delivery` L3a and I5: **the two areas swept at
    /// application start**, each having the same shape — a *discarded at once*
    /// rule that a process exiting first does not cover, completed at the
    /// other end. Anything found is removed before it could be used, **and
    /// none of it is ever read back**.
    pub fn is_swept_at_application_start(self) -> bool {
        matches!(
            self,
            DurablePlace::ThrowawayKnownHostsArea | DurablePlace::SmoozysOwnTemporaryArea
        )
    }
}

/// The user's downloaded outputs, named because a document that lists
/// lifetimes and omits this invites a builder to invent one.
///
/// R27: **it is not in any class and it is not a durable place of Smoozy's.**
/// It is the user's file. Nothing in Smoozy counts it, caps it, ages it or
/// removes it — *"whatever the user brought down to their own computer is
/// theirs, and untouched by any of this"*.
pub const DOWNLOADED_OUTPUTS_ARE_NOT_SMOOZYS_STATE: () = ();

/// What *written durably* means — §1.3 rule 6, defined once because every
/// ordering rule in this document and in `Rented-server runtime` rests on the
/// word.
///
/// **A write is durable when the following has completed, and not before.** The
/// bytes are written; the file is flushed to the storage device; where the
/// write is published by a name, the rename follows the flush; and **the
/// directory holding that name is flushed too**. The last clause is not
/// optional and is the one that is ordinarily missed — `fsync(2)`: *"Calling
/// fsync() does not necessarily ensure that the entry in the directory
/// containing the file has also reached disk. For that an explicit fsync() on
/// a file descriptor for the directory is also needed."*
///
/// **The call differs by system and the difference is not cosmetic.** On Linux
/// — every rented machine — `fsync` reaches the device. **On macOS it does
/// not**: `fsync(2)` on the Creator's own Mac, macOS 26.5.2, read 2026-08-20 —
/// *"while fsync() will flush all data from the host to the drive … the drive
/// itself may not physically write the data to the platters for quite some
/// time"*, *"This is not a theoretical edge case."*, and *"Mac OS X provides
/// the F_FULLFSYNC fcntl."* **So on the user's own computer a durable write is
/// the full-flush form, and on a rented machine it is the ordinary one.**
///
/// **This is ordering rather than atomicity.** The rename's own atomicity is
/// separately guaranteed and is not durability of what it points at.
///
/// **What it costs, stated rather than hidden.** A flush blocks until the
/// device reports the transfer complete, so every *durably before* is a real
/// wait. Where a rule wants a sequence of records durable in order, that is a
/// flush apiece; **nothing here permits batching them**, and a builder that
/// finds the cost intolerable in one named place brings that place back rather
/// than quietly weakening the word.
///
/// The call itself is the platform file adapter's — `Desktop app and delivery`
/// §6 — which is where this requirement is performed.
pub const A_DURABLE_WRITE_IS_FLUSHED_TO_THE_DEVICE: () = ();

/// §1.3 rule 5: **nothing in this document reads a file's presence, its name
/// or its size as state.** Every state fact comes from a record Smoozy or the
/// supervisor wrote.
///
/// Three observations force it and they are not corner cases. A killed
/// transfer leaves *"a piece of the file under the final name. No marker, no
/// `.part`, no temporary name, nothing to say it is incomplete"*. A machine
/// restarted mid-render left eleven whole frames with six further files beside
/// them at zero bytes, so *"the output folder said seventeen and the truth was
/// eleven"*. And on a full disk Blender *"announces the save, exits 0, says
/// nothing about the disk, and leaves a picture that does not end"*.
///
/// The one place a file's presence is read deliberately is the re-run's
/// already-there test, which `Queue & History` and `Job files and outputs` own
/// and which is a rule about **what to render**, never a reading of state.
pub const A_FILE_ON_DISK_IS_NEVER_A_STATE_FACT: () = ();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_local_record_holds_state() {
        let holding_state: Vec<_> = DurablePlace::ALL
            .into_iter()
            .filter(|place| place.holds_state())
            .collect();
        assert_eq!(holding_state, vec![DurablePlace::LocalRecord]);
    }

    #[test]
    fn two_areas_are_swept_at_application_start_and_no_others() {
        let swept: Vec<_> = DurablePlace::ALL
            .into_iter()
            .filter(|place| place.is_swept_at_application_start())
            .collect();
        assert_eq!(
            swept,
            vec![
                DurablePlace::ThrowawayKnownHostsArea,
                DurablePlace::SmoozysOwnTemporaryArea
            ]
        );
    }

    #[test]
    fn the_set_of_durable_places_is_seven_and_closed() {
        assert_eq!(DurablePlace::ALL.len(), 7);
    }
}

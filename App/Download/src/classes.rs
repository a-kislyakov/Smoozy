//! The ceiling and the two classes — `Download` §10, Q1 to Q4, Rows 206 and
//! 207.
//!
//! **The pool is not this document's and neither is the order.** `File
//! Browser` §6 owns both, and everything here either supplies that structure
//! one number or confirms it by citation. **Nothing in this file re-decides,
//! re-orders or re-derives any of it**, which is Q2's own sentence.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `File Browser` O1, O2, O4, O5, O7, O8.
//! `Server protocol and transfers` T5, T6, T8, T10. `Desktop app and delivery`
//! R17.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause.

use smoozy_file_browser::pool::{
    DeclaredClass, TheCeilings, THE_DECLARED_ORDER, THE_PER_SERVER_POOL,
};

/// **Q1: the download-direction ceiling inside `File Browser` O1's pool of
/// four is two transfer channels.**
///
/// **Two reasons, and the curve is only the first of them.** Fact 25's
/// measured download knee is at two streams — one to two buys 53%, two to four
/// buys 3%, four to eight buys 11% — and that fact's own record reads those
/// numbers as *"Two to four is the entire useful range"*, with most of the
/// gain hiding a flat 1.0–1.4 s connection setup rather than adding bandwidth.
/// **So the curve settles the second channel and does not by itself settle two
/// against four.**
///
/// **What settles two against four is the second reason**: two download
/// channels leave two of the pool's four slots that the download direction can
/// never occupy, so the upload direction — the slow one, at one eleventh to
/// one twelfth of the download rate — is never starved behind fast downloads
/// at a busy server.
///
/// **Two ceilings in one pool, not two pools.** The count the machine sees is
/// one count at the one per-server point (`Server protocol and transfers` T5);
/// no second pool is opened and no number here presumes the pool.
///
/// **The measurement's limits ride with it** — one home line, one evening,
/// over Wi-Fi, two of three rentals on one provider (§15 item 7). A different
/// line moves the knee; the second reason does not move with it, and the
/// structure is one pool with two ceilings and one number changed by one edit.
pub const THE_DOWNLOAD_DIRECTIONS_CEILING: usize = 2;

/// **Q1's number handed to `File Browser` O2's structure**, which is exactly
/// how §14 supplies it: that document left the download half open and takes
/// this figure as declared rather than spelling it again.
pub fn the_ceilings() -> TheCeilings {
    TheCeilings::with_the_ceiling_download_q1_declares(THE_DOWNLOAD_DIRECTIONS_CEILING)
}

/// **Q1's own guard: the ceiling leaves slots the download direction can never
/// occupy, and that is the half of the reasoning no measurement bears on.**
pub fn slots_the_download_direction_can_never_occupy() -> usize {
    THE_PER_SERVER_POOL - THE_DOWNLOAD_DIRECTIONS_CEILING
}

/// **Q3's two transfer-priority classes, declared here, per action, per
/// `Server protocol and transfers` T8.**
///
/// **Nothing decides a class at run time.** Which class a transfer carries
/// falls out of what arranged it, and this enumeration is the whole of the
/// mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatArrangedIt {
    /// An auto-download arrangement. **Its ending walk's fetches and its
    /// catch-up's fetches are the arrangement's too**, and carry its class.
    AnAutoDownloadArrangement,
    /// A manual request — **including a whole-job download started by hand
    /// from an entry, which is an ordinary manual request** and not a class of
    /// its own.
    AManualRequest,
}

/// **Q3: every transfer an auto-download arrangement arranges carries the
/// auto-download class; every transfer a manual request arranges carries the
/// manual download class.**
///
/// The names are `File Browser` O4's own, taken as that document already
/// declares them rather than spelled a second time here — **one fact, one
/// home**.
pub fn the_class(arranged_by: WhatArrangedIt) -> &'static str {
    let rank = match arranged_by {
        WhatArrangedIt::AnAutoDownloadArrangement => 1,
        WhatArrangedIt::AManualRequest => 2,
    };
    THE_DECLARED_ORDER
        .iter()
        .find(|class| class.rank == rank)
        .map(|class| class.name)
        .expect("`File Browser` O4 declares both download classes at ranks one and two")
}

/// **Q2: the order is `File Browser` O4's and this row confirms it by
/// citation, declaring no rank into it.**
///
/// Auto-download over manual download is Creator-stated (2026-07-22) and
/// quoted in this document's own spec; O4 already contains that pair at the
/// top of the five declared classes, and O5 already declares the share rule.
/// **Nothing here re-decides, re-orders or re-derives any of it**, so this is
/// the citation made checkable rather than a second declaration.
pub fn the_two_download_classes_as_o4_declares_them() -> Vec<DeclaredClass> {
    THE_DECLARED_ORDER
        .iter()
        .filter(|class| class.declared_by == "Download")
        .copied()
        .collect()
}

/// **Q4: at the desktop-side point the same order and the same share hold,
/// with grants counted at that point** — `File Browser` O8's declaration,
/// confirmed by citation for the two download classes.
///
/// **So both waits are discharged.** `Server protocol and transfers` T10's —
/// *"the share is Rows 189 and 207's to declare against this mechanism"* — and
/// `Desktop app and delivery` R17's, which requires the same in its own words.
/// **Row 189 declared the rule for all five classes, and Row 207's half is
/// this confirmation that the two download classes take that rule as declared,
/// adding nothing.**
///
/// **An arrangement's never-give-up cannot starve anything**: at every point
/// it is one class among five under O5's bound, and under destination trouble
/// it offers no work at all ([`crate::destination::what_the_arrangement_does`]).
pub const THE_TWO_DOWNLOAD_CLASSES_TAKE_O5S_SHARE_RULE_AS_DECLARED: bool = true;

/// **Q3's second half: no class is decided at run time and no number here
/// presumes the pool.**
pub const NOTHING_DECIDES_A_CLASS_AT_RUN_TIME: bool = true;

/// **P2's admission half, stated where the classes are**: admission is by
/// class then arrival at the per-server point, under the pool, the ceiling and
/// the share.
///
/// **The order *within* one request or one arrangement is
/// [`crate::fetching::the_fetch_order`]'s**, which is a different question:
/// this one says which class gets the next slot, that one says which of a
/// class's own units is offered for it.
pub const ADMISSION_IS_BY_CLASS_THEN_ARRIVAL_AT_THE_PER_SERVER_POINT: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_ceiling_is_two_and_the_two_ceilings_sit_inside_one_pool_of_four() {
        let ceilings = the_ceilings();
        assert_eq!(ceilings.download, 2);
        assert_eq!(ceilings.upload, THE_PER_SERVER_POOL);
        assert!(ceilings.are_inside_one_pool());
    }

    #[test]
    fn two_slots_of_the_pool_can_never_be_occupied_by_the_download_direction() {
        // Q1's second reason, which is what settles two against four. It is
        // arithmetic over the pool and no measurement bears on it.
        assert_eq!(slots_the_download_direction_can_never_occupy(), 2);
    }

    #[test]
    fn the_two_classes_are_the_ones_o4_declares_and_auto_download_outranks_manual() {
        let ours = the_two_download_classes_as_o4_declares_them();
        assert_eq!(ours.len(), 2);
        assert_eq!(
            ours[0].name,
            the_class(WhatArrangedIt::AnAutoDownloadArrangement)
        );
        assert_eq!(ours[1].name, the_class(WhatArrangedIt::AManualRequest));
        assert!(ours[0].rank < ours[1].rank);
    }

    #[test]
    fn the_class_falls_out_of_what_arranged_the_transfer_and_of_nothing_else() {
        // Q3: the ending walk's fetches and a catch-up's fetches are the
        // arrangement's, so they take its class rather than a third one; and a
        // whole-job download started by hand is an ordinary manual request.
        assert_eq!(
            the_class(WhatArrangedIt::AnAutoDownloadArrangement),
            "auto-download"
        );
        assert_eq!(the_class(WhatArrangedIt::AManualRequest), "manual-download");
    }

    /// **Q2 held rather than remembered.** This crate declares no rank into
    /// `File Browser` O4's order, so no source of it may name a rank for a
    /// class that is not one of the two — and none may name a third download
    /// class.
    #[test]
    fn this_crate_declares_no_third_download_class() {
        let named: Vec<&'static str> = THE_DECLARED_ORDER
            .iter()
            .filter(|class| class.declared_by == "Download")
            .map(|class| class.name)
            .collect();
        assert_eq!(named, vec!["auto-download", "manual-download"]);
    }
}

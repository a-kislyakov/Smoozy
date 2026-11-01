//! **§11 H7 performed — the *show a job's downloaded files* control's
//! opening.**
//!
//! **The set and its order are [`crate::landing`]'s** and are composed from
//! records already kept; what is here is the opening itself and what each
//! opening's outcome comes to.
//!
//! # One place and it opens; more than one and every place opens
//!
//! **H7: the control is handed the set, never a folder.** One place, and it
//! opens. **Where the set holds more than one, the control opens every place
//! in it and picks none** — it does not silently open the most recent, which
//! would be the app quietly choosing which half of the user's outputs is the
//! real one. **The order is H5's**, which is the order each place was first
//! written to, and this module keeps it by walking the set as it was handed
//! in.
//!
//! **What is left to Design is only how several openings are presented and
//! announced**, and the bound on that is unchanged: no place is hidden.
//!
//! # The service is a named one this document does not own
//!
//! **The act is the reveal service `Desktop app and delivery` §18.6 K20 to
//! K24 declares**, against Fact 132 — the chosen shell exposes none of it, its
//! one browser-opening call refuses the `file` scheme by name and returns
//! nothing either way, so the act is platform code of that document's own.
//! **K22's rule is that each opening reports its own outcome to the caller**,
//! and H8's *cannot open* reading is that outcome.
//!
//! **That code cannot be reached from here**, so [`OpensAFolder`] is the seam,
//! and the desktop's Transfers screen fills it. **Nothing in H5 to H8 waits on
//! the service**: the set and its order are composed from records already kept
//! and are correct whether or not anything opens.
//!
//! **Each place is shown selected in the folder that holds it** — H7's
//! paragraph of 2026-10-03, the Creator's note 15 on the test copy. A place is
//! the job's own folder ([`crate::landing::the_set_of_landing_places`]), the
//! service's first act names it, and **where that act refuses, the folder that
//! holds it is opened instead**; only a refusal of that is H8's *cannot open*.
//!
//! # A place that no longer exists stays in the set and says so
//!
//! **H8: the reading is taken when the control is used and never cached**, it
//! distinguishes nothing §9.1 does not already distinguish, and **an
//! unreachable place is presented as a place Smoozy cannot open rather than
//! dropped from the set** — dropping it would make a job that landed in two
//! folders read as a job that landed in one.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Desktop app and delivery` K20, K21, K22, K23,
//! K24, §18.6. `Reporting a problem` V7. `Queue & History` N18.

use crate::landing::{the_places, APlace};

/// **The reveal service `Desktop app and delivery` §18.6 K20 to K24
/// declares.**
///
/// **The far side is that document's platform code**, which cannot be reached
/// from a feature module; the desktop's Transfers screen implements it. The
/// per-system routes, the symbolic-link resolution and the refusal of the
/// no-outcome variant are that document's and are not restated here.
pub trait OpensAFolder {
    /// **K22: the outcome reaches the caller.** True where the place was shown:
    /// selected in the folder that holds it, or, where that was refused, that
    /// folder opened (H7's paragraph of 2026-10-03).
    fn open_this_folder(&mut self, folder: &[u8]) -> bool;
}

/// What one opening came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnePlaceOpened {
    pub place: APlace,
    /// **K22's outcome for this one place**, which is H8's *cannot open*
    /// reading where it is false.
    pub it_opened: bool,
}

/// **H7 performed: every place in the set is opened and none is picked.**
///
/// **The order is the set's own**, which is H5's order of first writing, and
/// nothing here sorts, filters or chooses.
///
/// **A place that cannot be opened stays in the answer** rather than being
/// dropped, which is H8: dropping it would make a job that landed in two
/// folders read as a job that landed in one.
///
/// **The reachability reading is taken here and never cached** — it is the
/// same reading [`the_places`] takes, made at the moment the control is used.
pub fn open_every_place<O: OpensAFolder + ?Sized>(
    opener: &mut O,
    set: &[Vec<u8>],
) -> Vec<OnePlaceOpened> {
    let mut opened = Vec::new();
    for place in set {
        let it_opened = opener.open_this_folder(place);
        // **The set's own presentation is [`the_places`]'s**, and it is asked
        // with what this opening actually found rather than with a second
        // reading of its own — one reading, one moment, no cache.
        let described = the_places(std::slice::from_ref(place), |_| it_opened);
        opened.push(OnePlaceOpened {
            place: described
                .into_iter()
                .next()
                .expect("one path describes one place"),
            it_opened,
        });
    }
    opened
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct AShell {
        asked: Vec<Vec<u8>>,
        refuses: Vec<Vec<u8>>,
    }

    impl OpensAFolder for AShell {
        fn open_this_folder(&mut self, folder: &[u8]) -> bool {
            self.asked.push(folder.to_vec());
            !self.refuses.iter().any(|at| at == folder)
        }
    }

    /// **H7: where the set holds more than one, every place opens and none is
    /// picked** — the app never quietly chooses which half of the user's
    /// outputs is the real one.
    #[test]
    fn every_place_in_the_set_opens_and_none_is_picked() {
        let mut shell = AShell::default();
        let set = vec![b"/Users/a/one".to_vec(), b"/Users/a/two".to_vec()];

        let opened = open_every_place(&mut shell, &set);

        assert_eq!(shell.asked, set);
        assert_eq!(opened.len(), 2);
        assert!(opened.iter().all(|one| one.it_opened));
    }

    /// **H5's order is the set's own**, and nothing here sorts or reorders it.
    #[test]
    fn the_order_is_the_sets_own() {
        let mut shell = AShell::default();
        let set = vec![b"/Users/a/second".to_vec(), b"/Users/a/first".to_vec()];
        open_every_place(&mut shell, &set);
        assert_eq!(shell.asked, set);
    }

    /// **H8: a place that cannot be opened stays in the answer**, presented as
    /// a place Smoozy cannot open — dropping it would make a job that landed
    /// in two folders read as a job that landed in one.
    #[test]
    fn a_place_that_cannot_be_opened_stays_in_the_answer() {
        let mut shell = AShell {
            refuses: vec![b"/Users/a/gone".to_vec()],
            ..AShell::default()
        };
        let set = vec![b"/Users/a/here".to_vec(), b"/Users/a/gone".to_vec()];

        let opened = open_every_place(&mut shell, &set);

        assert_eq!(opened.len(), 2);
        assert!(opened[0].it_opened);
        assert!(!opened[1].it_opened);
        assert!(!opened[1].place.smoozy_can_open_it);
    }

    /// **One place, and it opens.**
    #[test]
    fn one_place_opens() {
        let mut shell = AShell::default();
        let opened = open_every_place(&mut shell, &[b"/Users/a/one".to_vec()]);
        assert_eq!(opened.len(), 1);
        assert!(opened[0].it_opened);
    }
}

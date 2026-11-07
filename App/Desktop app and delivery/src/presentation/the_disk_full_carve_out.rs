//! **The one cause Smoozy names on a failed render** — §16.4, M14 to M17.
//!
//! **Read the folder's head first.** This file draws inside the shape stated at
//! `src/presentation/mod.rs` and adds nothing to it.
//!
//! ## What the carve-out is, and what it is not
//!
//! Row 287 carves one diagnosis out of the no-diagnosis rule, **because it is
//! read off the machine rather than worked out**. It is the whole of what this
//! file does, and everything below is a bound on it.
//!
//! - **M14: the diagnosis is shown *alongside* the machine's own words and never
//!   instead of them.** It goes in the failure shape's `what_to_do_about_it`,
//!   which is Design's *"only where Smoozy actually knows"* — **the plumbing
//!   itself never knows and leaves it `None`**, and this is the one place in the
//!   application that ever fills it from a reading.
//! - **M16: it never claims the reading is current, never claims the amount is
//!   the amount the rental may use, and never says a render failed because the
//!   disk was full.** It says the place a write was refused for space is out of
//!   space.
//! - **M17: it reaches no further than `File Browser`'s reading does**, which is
//!   a per-path figure against an exact folder. `File Browser` L11 is that
//!   reading and `Server protocol and transfers` F16 is the clause that forbids
//!   widening a per-path answer into a per-machine one.
//!
//! ## The reading arrives carried, and that is M15a
//!
//! **The free-space reading is taken on the machine, in the same act that writes
//! that render's recorded outcome, and reaches this side carried with that
//! outcome and with its moment on the machine's own clock.** It is
//! `Server protocol and transfers` F6a and `Rented-server runtime` V12, both
//! built.
//!
//! **No reading taken when the desktop learns the ending feeds this**, and
//! nothing on a presentation path takes one in any case (M1, P9). So this file
//! takes nothing: [`TheCarriedReading::of`] is the only way one exists here and
//! it takes the moment.
//!
//! **A recorded outcome carrying no reading is not one carrying a zero.**
//! [`what_to_do_about_it`] answers `None` for an absent reading, the machine's
//! own words stand alone as M17 already has them do, and **nothing infers a full
//! disk from the absence.**
//!
//! ## Why this can never become a live condition
//!
//! `Server protocol and transfers` F11a: the carried reading is an observation
//! on an **immutable recorded ending** and is never a supplier of the live
//! disk-full condition, whose two current suppliers stay
//! `Server protocol and transfers` F11's.
//! Presenting it as a condition would put up a record the next refresh
//! withdraws, which is M6's harm. **Nothing here composes a failure record**, so
//! there is no route by which it could.
//!
//! ## Where the drawing and the clauses disagree, and the clauses win
//!
//! `Preparation/Design/Mockup/Smoozy.html` line 5487 draws the second sentence
//! as *"The disk on RunPod 4090 is full. Free space on the server, then render
//! it again."* **Three things in that line are refused here and each has its own
//! clause.**
//!
//! 1. **It names the machine.** The reading is per-path, against that job's own
//!    output folder, and M17 caps the message at what `File Browser`'s reading
//!    says. `Server protocol and transfers` F16 forbids widening it: on a
//!    container the host's own disk was bind-mounted in showing 20 GiB free the
//!    rental may not use. **So the sentence names the folder and never the
//!    machine's disk.**
//! 2. **It joins a server's name into prose.** X7 forbids the typography point
//!    touching a name, and a name joined into a sentence before the point comes
//!    out with a non-breaking space inside it, with nothing anywhere reporting
//!    it. **Two strings on two routes**, which is the folder head's rule 4.
//! 3. **It is two sentences in the one field the shape has for one.** X2 binds
//!    the last inter-word space of the string handed in, so a second sentence
//!    inside one string leaves the first one's ending unbound — §22 item 28's
//!    narrowing, which piece C's shape avoids by having exactly two sentence
//!    fields. **So the diagnosis is one sentence.**
//!
//! Rule 10 and `What Build inherits` item 5 are what decide it: where the
//! drawing and an architecture answer disagree, the architecture wins.
//!
//! *(2026-11-06, the check after the last fix: the drawing's History `disk`
//! state now says [`THE_DIAGNOSIS`] word for word, so the two no longer
//! disagree. The three reasons above are why it says those words.)*
//!
//! ## Markers of other documents
//!
//! **Markers used in this file that belong to another document are written out
//! with that document at the site, every one of them, and none is listed here.**
//! This plan defines an F-series, an L-series and a V-series of its own, and so
//! do `Server protocol and transfers`, `File Browser` and
//! `Rented-server runtime`. A header list cannot tell two apart, so a bare one
//! would resolve to a real and wrong clause.

use std::time::SystemTime;

use smoozy_state::observed::{Clock, MachineClock, Reading};

use crate::presentation::as_of_that_moment::AsOfThatMoment;
use crate::typography::Verbatim;

/// **Route (b)'s reading, as it arrives** — `Server protocol and transfers` F6a
/// and `Rented-server runtime` V12.
///
/// **Each fact is stored once.** The figure the page draws is written from
/// `available_bytes` at the moment it is presented, so there is no route by
/// which a written figure and the number the carve-out fires on could disagree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheCarriedReading {
    /// **`Server protocol and transfers` F3's available figure** — never the
    /// free figure, because of the root-reserved blocks.
    available_bytes: u64,
    /// **The moment it was taken, on the machine's own clock** (M15, M3a). It
    /// is the moment that act wrote the render's recorded outcome, and never the
    /// moment the desktop learned of it.
    taken_at: SystemTime,
    /// Whose clock. **The server's own name**, which the page draws beside the
    /// figure so that a reading taken elsewhere never reads as one taken here.
    on_the_machines_clock: MachineClock,
    /// **The exact folder the reading was taken against** —
    /// `Server protocol and transfers` F1's requirement, carried so that nothing
    /// downstream widens a per-path answer into a per-machine one
    /// (M17, and `Server protocol and transfers` F16).
    against_the_folder: String,
}

impl TheCarriedReading {
    /// **The only constructor, and it takes the moment.**
    ///
    /// The same discipline as [`AsOfThatMoment::from`], and for the same reason:
    /// a figure that reached a surface without its moment would sit on screen
    /// beside an honest state with nothing saying it is frozen, which is the
    /// failure M3a was written for.
    pub fn of(
        available_bytes: u64,
        taken_at: SystemTime,
        on_the_machines_clock: MachineClock,
        against_the_folder: &str,
    ) -> TheCarriedReading {
        TheCarriedReading {
            available_bytes,
            taken_at,
            on_the_machines_clock,
            against_the_folder: against_the_folder.to_string(),
        }
    }

    /// **The figure, as of the moment it was taken, on the machine's own clock**
    /// — M15 and M3a, through the one constructor this folder has.
    ///
    /// **It is never re-read to keep a message fresh** and it is never aged into
    /// a newer figure; the page draws the moment beside it and nothing subtracts
    /// across two clocks (`State, identity and recovery` D5).
    pub fn presented(&self) -> AsOfThatMoment {
        AsOfThatMoment::from(&Reading::taken(
            self.available_bytes.to_string(),
            self.taken_at,
            Clock::AMachine(self.on_the_machines_clock.clone()),
        ))
    }

    /// **The folder the reading was taken against**, by the verbatim route — a
    /// path, which X7 names in its closed list of what the typography point
    /// never touches.
    pub fn the_folder_it_was_taken_against(&self) -> Verbatim {
        Verbatim::new(self.against_the_folder.clone())
    }

    /// **Whether route (b) fires: the reading reports nothing available.**
    ///
    /// `Server protocol and transfers` F6's route (b) is *"a render ended
    /// failing, and a free-space reading taken against that job's own output
    /// path reports nothing available"*. **A reading with room does not fire
    /// it**, and there is no other figure it could be compared against.
    pub fn there_was_no_space_left(&self) -> bool {
        self.available_bytes == 0
    }
}

/// **The diagnosis, and it is one sentence.**
///
/// **Alongside the machine's own words and never instead of them** (M14). It
/// says what a reading against a place says and no more:
/// `Server protocol and transfers` F14's route (b) wording — *"that the place
/// this job wrote to had no room at the moment that job's ending was
/// recorded"* — with Design's own second half, *"free space on the server, then
/// render it again"*, folded into the same sentence.
///
/// **What it does not say, clause by clause.** It does not say the render failed
/// because the disk was full (M16, and `Server protocol and transfers` F14). It
/// does not claim the reading is current — *"when the render ended"*, and the
/// figure beside it carries its own moment (M16, F15). It does not claim the
/// amount is the amount the rental may use, because it names no amount at all
/// (M16, and `Server protocol and transfers` F16). And it names the folder
/// rather than the machine, which is M17.
///
/// **The name of the server is not in it** and is never joined into it: the
/// server is named beside the figure, on the verbatim route, because X7 forbids
/// the point touching a name.
///
/// *(Shortened 2026-10-28, the final words sweep: it read *The folder this job
/// wrote to had no space left when the render ended, so free space on the
/// server and render it again.* The output folder is still the folder this job
/// wrote to, and the phone's copy changes with it.)* *(Shortened again
/// 2026-11-01, the second words sweep, with the phone's: it read *…; free
/// space on the server and render again.* The scout's *Server disk full.* was
/// not taken: it names the machine's disk, which M17 forbids, and drops *when
/// the render ended*, which M16 needs.)*
pub const THE_DIAGNOSIS: &str = "The output folder was full when the render ended; free space and resubmit.";

/// **M14 and M15a, performed: the second sentence, where and only where the
/// carried reading says there was no room.**
///
/// - **No reading at all answers `None`.** *"A recorded outcome carrying no such
///   reading is not one carrying a zero"* — route (b) does not fire, the
///   machine's own words stand alone, and nothing infers a full disk from the
///   absence.
/// - **A reading with room answers `None`.** The carve-out is one diagnosis and
///   not a place to put a figure.
/// - **This is the only thing in the application that fills
///   `what_to_do_about_it` from a reading**, which is Design's *only where
///   Smoozy actually knows*.
pub fn what_to_do_about_it(reading: Option<&TheCarriedReading>) -> Option<String> {
    match reading {
        Some(reading) if reading.there_was_no_space_left() => Some(THE_DIAGNOSIS.to_string()),
        _ => None,
    }
}

/// **M14, kept structurally: nothing here can take the machine's words away.**
///
/// The diagnosis is a sentence and the words are a panel, composed in
/// `src/presentation/the_machines_own_words.rs`, and **neither function can see
/// the other**. There is no argument through which the carve-out could suppress
/// a panel and no argument through which a panel could suppress the carve-out,
/// so *alongside and never instead of* is a property of there being no route
/// rather than a rule somebody keeps.
pub const THE_DIAGNOSIS_CANNOT_REPLACE_THE_MACHINES_WORDS: () = ();

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn at(seconds: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(seconds)
    }

    fn a_full_folder() -> TheCarriedReading {
        TheCarriedReading::of(
            0,
            at(1_700_000_000),
            MachineClock("RunPod 4090".to_string()),
            "/home/artist/render/kitchen-final/beauty",
        )
    }

    /// **M15a: the reading arrives carried, with its moment on the machine's own
    /// clock, and the page draws that name beside the figure.**
    #[test]
    fn the_reading_carries_its_own_moment_on_the_machines_clock() {
        let presented = a_full_folder().presented();
        assert_eq!(presented.value.as_str(), "0");
        assert_eq!(presented.taken_at, Some(1_700_000_000));
        assert_eq!(presented.on_the_clock, "RunPod 4090");
    }

    /// **M15a: a recorded outcome carrying no reading is not one carrying a
    /// zero.**
    ///
    /// Route (b) does not fire on an absent reading, nothing is re-read to
    /// supply one, and nothing infers a full disk from the absence.
    #[test]
    fn no_reading_is_not_a_reading_of_zero() {
        assert_eq!(what_to_do_about_it(None), None);
    }

    /// **`Server protocol and transfers` F6's route (b): it fires where the
    /// reading reports nothing available, and not otherwise.**
    #[test]
    fn the_carve_out_fires_only_where_the_reading_says_there_was_no_room() {
        assert_eq!(
            what_to_do_about_it(Some(&a_full_folder())),
            Some(THE_DIAGNOSIS.to_string())
        );
        let roomy = TheCarriedReading::of(
            482_000_000_000,
            at(1_700_000_000),
            MachineClock("RunPod 4090".to_string()),
            "/home/artist/render/kitchen-final/beauty",
        );
        assert_eq!(what_to_do_about_it(Some(&roomy)), None);
    }

    /// **M16: it never says a render failed because the disk was full.**
    ///
    /// The sentence is one constant, so this reads the whole of what the
    /// application can ever say here.
    #[test]
    fn the_diagnosis_never_states_a_cause() {
        let said = THE_DIAGNOSIS.to_lowercase();
        for causing in [
            "because",
            "caused",
            "the reason",
            "why",
            "failed because",
            "that is why",
        ] {
            assert!(
                !said.contains(causing),
                "the diagnosis says {causing}. **M16: it never says a render \
                 failed because the disk was full** — it says the place a write \
                 was refused for space is out of space"
            );
        }
    }

    /// **M17 and `Server protocol and transfers` F16: it reaches no further than
    /// a per-path reading does.**
    ///
    /// The sentence names the folder the job wrote to. **It never says the
    /// machine's disk is full**, which is the widening F16 forbids and which the
    /// drawing does at mockup line 5487.
    #[test]
    fn the_diagnosis_names_the_folder_and_never_the_machine() {
        let said = THE_DIAGNOSIS.to_lowercase();
        assert!(said.contains("output folder"));
        for widened in [
            "the disk on",
            "the machine's disk",
            "the server's disk",
            "the disk is full",
            "out of disk",
        ] {
            assert!(
                !said.contains(widened),
                "the diagnosis says {widened}. **M17: it reaches no further than \
                 `File Browser`'s reading does**, and \
                 `Server protocol and transfers` F16 forbids widening a per-path \
                 answer into a per-machine one — on a container the host's own \
                 disk was bind-mounted in showing 20 GiB free the rental may not \
                 use"
            );
        }
    }

    /// **M16 and `Server protocol and transfers` F15: it never claims the
    /// reading is current.**
    ///
    /// The sentence is about the moment the render ended, and the figure beside
    /// it carries its own moment rather than being redrawn as fresh.
    #[test]
    fn the_diagnosis_is_about_the_moment_the_render_ended() {
        assert!(THE_DIAGNOSIS.contains("when the render ended"));
        for now in ["is full", "currently", "right now", "at the moment"] {
            assert!(
                !THE_DIAGNOSIS.contains(now),
                "the diagnosis says {now}. **M16: it never claims the reading is \
                 current**"
            );
        }
    }

    /// **M16: it names no amount**, so it cannot claim the amount is the amount
    /// the rental may use.
    #[test]
    fn the_diagnosis_names_no_amount() {
        assert!(!THE_DIAGNOSIS.chars().any(|c| c.is_ascii_digit()));
    }

    /// **X2 and X10: it is one sentence**, so the shape's two sentence fields
    /// hold two sentences and §22 item 28's narrowing does not reach it.
    #[test]
    fn the_diagnosis_is_one_sentence() {
        assert_eq!(THE_DIAGNOSIS.matches('.').count(), 1);
        assert!(THE_DIAGNOSIS.ends_with('.'));
    }

    /// **X7: no name is joined into it.** The drawing joins a server's name into
    /// this sentence; a name through the typography point comes out with a
    /// non-breaking space inside it and nothing anywhere reports it.
    #[test]
    fn no_name_is_joined_into_the_diagnosis() {
        let source = include_str!("the_disk_full_carve_out.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half")
            .to_string();
        assert!(
            !source.contains("format!"),
            "this file builds a sentence with `format!`. **X7: the point never \
             touches a name, a path, a host or a port** — the server is named \
             beside the figure, on the verbatim route, and never inside the \
             sentence"
        );
    }

    /// **One sentence for the one diagnosis** — the runner's ruling of
    /// 2026-10-25 (step 12's SC4 findings): this file's sentence wins, and the
    /// phone's failure message says the same words (`Telegram` E1), so a
    /// screen and a phone cannot name one full folder two ways.
    #[test]
    fn the_phone_says_the_same_one_sentence() {
        assert_eq!(smoozy_telegram::job_messages::THE_DISK_IS_FULL, THE_DIAGNOSIS);
    }

    /// **M17, and `Server protocol and transfers` F1: the folder travels with
    /// the reading, by the verbatim route.**
    #[test]
    fn the_folder_the_reading_was_taken_against_travels_with_it() {
        assert_eq!(
            a_full_folder().the_folder_it_was_taken_against().as_str(),
            "/home/artist/render/kitchen-final/beauty"
        );
    }
}

//! **What the plumbing hands the diary and the report entry point** — §16.8
//! M32 and M33.
//!
//! **These two are what make two promises bind structurally rather than being
//! controls somebody remembered to add in one module.** Row 454's *in the same
//! words* and Row 468's *every failure carries a route to report it* both rest
//! on the plumbing doing it rather than on a screen doing it.
//!
//! ## M32 — the diary
//!
//! **At the first presentation per condition identity or ending record, the
//! plumbing hands the diary service the composed plain sentence verbatim**,
//! with the record's kind, its condition identity or the subject outcome it
//! marks, its durable subject and the moment. **Once, keyed exactly as Q1 keys
//! the presentation, which is also what the user actually met once.**
//!
//! **Three things about it are easy to get wrong and each is a line of code
//! here.**
//!
//! 1. **The words are captured at presentation, because that is where they
//!    exist.** `State, identity and recovery` §8.2 keeps the record a
//!    derivation holding no prose, so there is nowhere else to take them from.
//!    *"Nothing composes the words a second time at report time"*, so a
//!    presentation that has changed since cannot make the diary lie about what
//!    was said.
//! 2. **The captured string is the composed sentence before X7's two
//!    substitutions.** X7: the typography point is *"on the path to the screen
//!    and on no other path"*, and the diary is not a screen. So the diary holds
//!    the same words with none of the screen's spacing characters — the same
//!    parity `Reporting a problem` V1 already applies to the user's own
//!    sentence. **[`TheFailureShape::what_did_not_happen`] is a plain `String`
//!    for exactly this reason**, and there is no `Prose` in that struct to
//!    capture by mistake.
//! 3. **Once per record and not once per run.** The mark is a column of the
//!    record's own row (`the_diary_holds_the_sentence`), so a record raised in
//!    one run and presented again in the next hands the diary nothing a second
//!    time.
//! 4. **The capture belongs to whatever composes the sentence the user reads,
//!    and a path that composes a stand-in sentence captures nothing.** That is
//!    M32's own sentence, added to the clause on 2026-09-12 out of this round,
//!    and [`WhoseSentenceItIs`] is it held by construction rather than by a
//!    caller remembering. **The hook is also called after the presentation has
//!    been decided and never before**, so a record found not to stand hands the
//!    diary nothing.
//!
//! **M32's second limb, added 2026-09-17: a sentence shown with no stored
//! record.** A read of a `.blend` that gave nothing, and a dependency check that
//! could not run, raise no record (`Render Settings` Q9), so the first limb has
//! nothing to key them on — and both declarations promise the sentence reaches
//! the diary. [`the_diary_is_handed_a_sentence_with_no_record`] is that limb:
//! **once per presentation**, meaning once per request, with the server row,
//! the user's label beside it, the file's recorded path and the moment; the
//! stand-in rule holds, so an answer that the act cannot be performed yet is
//! never captured; and no once-mark is written, there being no record to hold
//! one.
//!
//! **The line is written by the plumbing and not by the module**, which is why
//! it does not go through [`crate::diary_service::ModulesDiary`]. `Reporting a
//! problem` J6 gives the failure kind to this plumbing by name, and every
//! feature module's own declaration excludes `LineKind::Failure` — four of them
//! carry a test asserting exactly that.
//!
//! ## M33 — the report entry point
//!
//! **Every expanded failure presentation supplies the report entry point the
//! failure handover, and the supply is structural rather than per-module.**
//! [`TheFailureHandover`] is `Reporting a problem` X4's closed set and holds
//! five things and no sixth.
//!
//! **The consumer is the report box, built at step 11's piece R4** —
//! `crate::surface::reporting`, with `web/reporting.js` as its page half. Every
//! failure shape's *Report a problem…* hands it this handover, and
//! `crate::surface::reporting::TheHandover::read_back` judges it before
//! anything is touched. *(Until that piece this file said, correctly, that the
//! consumer was not built: what had landed here was the supply half, which is
//! the half §16.8 owns, and the handover was parked on the page with nothing to
//! take it.)* **The slice mapping is `Reporting a problem` X4's and is
//! deliberately not applied
//! here** — that is why [`TheFailureHandover::subject_kind`] travels: the
//! entry point maps a job or history record to the failed-job slice and every
//! other subject to the third rule's slice
//! (`crate::surface::reporting::the_slice_for`), and doing that here would be
//! this document deciding another's.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Reporting a problem` J6. `State, identity and
//! recovery` §8.1, §8.2 and §8.7.
//!
//! **`Reporting a problem` V1, X4 and X5 and `State, identity and recovery` Y5
//! are deliberately not listed above and are written out with their document at
//! every site instead** — this plan defines an X4, an X5, a V1 and a Y5 of its
//! own, so a bare one would resolve to a real and wrong clause and the header
//! list cannot tell the two apart.

use std::time::SystemTime;

use smoozy_reporting::lines::{Line, LineKind};
use smoozy_state::identities::Identity;
use smoozy_state::observed::Clock;

use crate::diary_service::{DiaryService, Wrote};
use crate::record::failure_records::{FailureRecords, Standing};

/// **`Reporting a problem` X4's closed handover.** Five things and no sixth.
///
/// *"the record's **kind**, its **condition identity or subject outcome mark**,
/// its **durable subject**, the **presented sentence** — the same string M32
/// captured, never recomposed — and the **moment**."*
///
/// **The sentence here is the pre-substitution string**, the same one the diary
/// holds, which is what *never recomposed* means: `Reporting a problem` X5 has
/// the box name the
/// failure and the report's head carry the same naming line, *"drawn from the
/// handover, never recomposed"*.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheFailureHandover {
    /// `State, identity and recovery` §8.1's kind.
    pub kind: &'static str,
    /// The condition identity, or the subject outcome mark.
    pub identity_or_mark: String,
    /// §8.7's durable subject.
    pub durable_subject: String,
    /// Which of §8.7's closed list it is. **`Reporting a problem` X4's slice
    /// mapping turns on this**
    /// and is applied by the entry point, not here.
    pub subject_kind: &'static str,
    /// The presented sentence, before the point's substitutions.
    pub the_presented_sentence: String,
    /// The moment, on this computer's clock. Seconds since the epoch.
    pub at: Option<u64>,
}

impl TheFailureHandover {
    /// Compose the handover from the record and the sentence that was just
    /// presented. **Nothing here recomposes a sentence** — it takes the one it
    /// is given.
    pub fn of(record: &Standing, the_presented_sentence: &str) -> Self {
        TheFailureHandover {
            kind: record.kind.as_text(),
            identity_or_mark: record.identity_or_mark.clone(),
            durable_subject: record.subject.as_text(),
            subject_kind: record.subject.kind_as_text(),
            the_presented_sentence: the_presented_sentence.to_string(),
            at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .map(|since| since.as_secs()),
        }
    }
}

/// **Whose sentence is being handed over** — M32: *"the capture belongs to
/// whatever composes the sentence the user reads, and a path that composes a
/// stand-in sentence captures nothing."*
///
/// **Why this is a type rather than a rule somebody keeps.** The capture is once
/// per record and the *once* is written down durably. A path that composes a
/// sentence deliberately not a screen's — §16's own plumbing, where the owning
/// module has supplied no words — used to burn the record's one capture on it,
/// and when a real screen later composed the sentence the user actually read,
/// the record already said the diary held one. **The diary then kept the
/// stand-in for the life of that record**, and a problem report told the Creator
/// that Smoozy had said *Something is not working* when it had said something
/// else entirely. That is the exact failure M32's *never recomposed* wording
/// exists to prevent, arriving from the other direction.
///
/// Added 2026-09-12 with M32's own repair, at round 1 of the code audit of what
/// is drawn inside the window (U-1 and F-4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhoseSentenceItIs<'a> {
    /// **The sentence the user reads**, composed by whatever owns the words.
    /// This is the one that spends the capture.
    TheComposersOwn(&'a str),
    /// **A stand-in this section's plumbing composed** because the owning module
    /// has supplied none. It captures nothing and leaves the record's one
    /// capture unspent.
    AStandInThePlumbingComposed(&'a str),
}

/// What came of M32's hook.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheDiaryWasHanded {
    /// The line was written and the record now says so.
    TheSentence,
    /// **The record had already been presented once**, so nothing was handed
    /// over. This is M32's *once*, and it is the ordinary answer rather than a
    /// failure.
    NothingBecauseItHasAlreadyBeenSaid,
    /// The diary or the record could not take it. **Nothing is retried and
    /// nothing is queued** — the diary's own loss costs the contents of a future
    /// report and nothing else (`Reporting a problem` R2, `State, identity and
    /// recovery` I1).
    Nothing { reported: String },
    /// **The sentence was a stand-in, so the capture is not spent** — M32. The
    /// ordinary answer on this path rather than a failure: the screen that later
    /// composes the sentence the user reads is the one that fills the diary.
    NothingBecauseItIsAStandIn,
}

/// **M32's hook, at the first presentation of this record.**
///
/// `the_composed_sentence` is the plain sentence **before** the typography
/// point's substitutions.
///
/// **The record is marked before the answer comes back** — no: the line is
/// written first and the mark second, because a mark written before a line that
/// then failed would make the record claim a sentence the diary does not hold,
/// and M32's whole promise is that the diary holds what the user met.
pub fn the_diary_is_handed_the_sentence(
    diary: Option<&DiaryService>,
    records: &mut FailureRecords<'_>,
    run: Option<&Identity>,
    record: &Standing,
    sentence: WhoseSentenceItIs<'_>,
) -> TheDiaryWasHanded {
    // **M32: a path that composes a stand-in sentence captures nothing.**
    let WhoseSentenceItIs::TheComposersOwn(the_composed_sentence) = sentence else {
        return TheDiaryWasHanded::NothingBecauseItIsAStandIn;
    };
    if record.the_diary_holds_the_sentence {
        return TheDiaryWasHanded::NothingBecauseItHasAlreadyBeenSaid;
    }
    let (Some(diary), Some(run)) = (diary, run) else {
        return TheDiaryWasHanded::Nothing {
            reported: "there is no diary to hand it to, or this run has no identity".to_string(),
        };
    };

    // `Reporting a problem` J6: the line carries the sentence, the record's
    // kind, its condition identity or the subject outcome it marks, its durable
    // subject and the moment. **The identities it concerns are the durable
    // subject's, where the subject has one** — the licence record has none, and
    // an empty list is the honest answer rather than an invented identity.
    let mut concerning = Vec::new();
    if let Some(identity) = record.subject.identity() {
        concerning.push(identity.clone());
    }
    let line = Line {
        run: run.clone(),
        kind: LineKind::Failure,
        at: SystemTime::now(),
        // `State, identity and recovery` D5: the desktop's own clock. A failure
        // line's moment is the moment the user met the sentence, which happened
        // here.
        clock: Clock::ThisComputer,
        // **Verbatim, and before the point.** X7 and `Reporting a problem` V1.
        text: the_composed_sentence.to_string(),
        concerning,
    };

    match diary.write(&line) {
        Wrote::Yes => match records
            .the_diary_now_holds_the_sentence(&record.subject, &record.identity_or_mark)
        {
            Ok(()) => TheDiaryWasHanded::TheSentence,
            Err(refused) => TheDiaryWasHanded::Nothing {
                reported: format!("{refused:?}"),
            },
        },
        Wrote::No { reported } => TheDiaryWasHanded::Nothing { reported },
    }
}

/// **What M32's second limb carries in place of a record** — a failure sentence
/// the owning module composed and showed **without raising any record**.
///
/// `Reporting a problem` J6, as widened on 2026-09-17: the line carries the
/// sentence, **the server row's local identifier with the user's own label
/// beside it and never in place of it**, the file's recorded path as its
/// subject, and the moment on this computer's clock. The two callers are a read
/// of a `.blend` that gave nothing (`Render Settings` §12) and a dependency
/// check that could not run (`Dependency Upload & Remap` §11).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct APresentationWithNoRecord<'a> {
    /// The server row's local identifier.
    pub server_row: &'a Identity,
    /// **The user's own label for the machine**, carried beside the identifier.
    pub the_users_label: &'a str,
    /// The `.blend`'s recorded path, which is the line's subject.
    pub the_files_recorded_path: &'a str,
}

/// **M32's second limb: a sentence shown with no stored record is handed to the
/// diary once per presentation.**
///
/// **Once per presentation means once for each time the user asked for the
/// act**, so the caller calls this where a request's ending first lands and
/// nowhere else — never on a redraw. **No once-mark is written**: there is no
/// record to hold one, and a second request is a second presentation the user
/// met.
///
/// **The first limb's stand-in rule holds unchanged.** A sentence saying the act
/// cannot be performed yet is not an ending of an attempt, and a caller hands it
/// as [`WhoseSentenceItIs::AStandInThePlumbingComposed`] or does not call this
/// at all; either way nothing is written.
///
/// **Verbatim, and before the point** (X7): `the_composed_sentence` is the
/// sentence before the typography point's substitutions.
pub fn the_diary_is_handed_a_sentence_with_no_record(
    diary: Option<&DiaryService>,
    run: Option<&Identity>,
    presented: &APresentationWithNoRecord<'_>,
    sentence: WhoseSentenceItIs<'_>,
) -> TheDiaryWasHanded {
    let WhoseSentenceItIs::TheComposersOwn(the_composed_sentence) = sentence else {
        return TheDiaryWasHanded::NothingBecauseItIsAStandIn;
    };
    let (Some(diary), Some(run)) = (diary, run) else {
        return TheDiaryWasHanded::Nothing {
            reported: "there is no diary to hand it to, or this run has no identity".to_string(),
        };
    };
    // **The fixed shape the act lines already use** — values tab-separated, the
    // label beside the identifier's line rather than in its place. The sentence
    // comes first and is verbatim.
    let line = Line {
        run: run.clone(),
        kind: LineKind::Failure,
        at: SystemTime::now(),
        clock: Clock::ThisComputer,
        text: [
            the_composed_sentence,
            presented.the_files_recorded_path,
            presented.the_users_label,
        ]
        .join("\t"),
        concerning: vec![presented.server_row.clone()],
    };
    match diary.write(&line) {
        Wrote::Yes => TheDiaryWasHanded::TheSentence,
        Wrote::No { reported } => TheDiaryWasHanded::Nothing { reported },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::failure_records::{ConditionKey, DurableSubject, Kind};
    use crate::record::LocalRecord;
    use smoozy_state::identities::IdentityKind;
    use std::path::PathBuf;

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    /// **A scratch name nothing else on this machine can be using.**
    ///
    /// Until 2026-09-18 both helpers below named their scratch paths after the
    /// test alone, so the names were fixed. **Two test runs at once on one
    /// computer — two agents, or a whole-workspace run beside a filtered one —
    /// then shared a record file and a diary area**, each deleting and
    /// rewriting the other's, and the counting assertions below read a segment
    /// somebody else had appended to. That is the whole of the flake these
    /// tests showed at the desk from 2026-09-17: two runs at once fail about
    /// every time, one run alone never does.
    ///
    /// **Every other test helper in this crate puts the process id in the name,
    /// and this folder cannot**: `presentation::tests`'s M1 check
    /// `nothing_on_a_presentation_path_can_reach_a_machine` refuses the text
    /// `std::process` anywhere under `src/presentation/`, which is why the
    /// names here were fixed in the first place. A fresh identity is this
    /// project's own unique value and needs nothing from the process.
    fn a_scratch_name(name: &str) -> String {
        format!(
            "smoozy-hooks-{name}-{}",
            an_identity(IdentityKind::Run).as_text()
        )
    }

    fn a_record(name: &str) -> (LocalRecord, PathBuf) {
        let mut path = std::env::temp_dir();
        path.push(format!("{}.sqlite3", a_scratch_name(name)));
        let _ = std::fs::remove_file(&path);
        (
            LocalRecord::open_at(&path).expect("the desk can make a record"),
            path,
        )
    }

    fn a_diary(name: &str) -> (DiaryService, PathBuf) {
        let mut area = std::env::temp_dir();
        area.push(a_scratch_name(&format!("diary-{name}")));
        let _ = std::fs::remove_dir_all(&area);
        (
            DiaryService::open_at(area.clone()).expect("the desk can make a diary"),
            area,
        )
    }

    fn what_the_diary_holds(area: &PathBuf) -> String {
        let mut held = String::new();
        for entry in std::fs::read_dir(area).expect("the area is there").flatten() {
            held.push_str(&std::fs::read_to_string(entry.path()).unwrap_or_default());
        }
        held
    }

    /// **M32: the sentence is handed over once, and the second presentation
    /// hands over nothing.**
    #[test]
    fn the_sentence_reaches_the_diary_once_per_record() {
        let (record, record_path) = a_record("once");
        let (diary, area) = a_diary("once");
        let run = an_identity(IdentityKind::Run);
        let server = DurableSubject::ServerRow(an_identity(IdentityKind::LocalServer));
        let key = ConditionKey::over(&server, "the server connection", None);

        let mut records = FailureRecords::over(record.connection());
        records
            .a_condition_was_raised(&key, &server, &["disconnected"], true)
            .unwrap();
        let standing = records.standing_against(&server).unwrap().remove(0);

        assert_eq!(
            the_diary_is_handed_the_sentence(
                Some(&diary),
                &mut records,
                Some(&run),
                &standing,
                WhoseSentenceItIs::TheComposersOwn("Smoozy could not reach the server."),
            ),
            TheDiaryWasHanded::TheSentence
        );

        let again = records.standing_against(&server).unwrap().remove(0);
        assert_eq!(
            the_diary_is_handed_the_sentence(
                Some(&diary),
                &mut records,
                Some(&run),
                &again,
                WhoseSentenceItIs::TheComposersOwn("Smoozy could not reach the server."),
            ),
            TheDiaryWasHanded::NothingBecauseItHasAlreadyBeenSaid
        );

        let held = what_the_diary_holds(&area);
        assert_eq!(held.matches("Smoozy could not reach the server.").count(), 1);
        let _ = std::fs::remove_dir_all(&area);
        let _ = std::fs::remove_file(&record_path);
    }

    /// **M32 and X7: the diary holds the sentence before the point's
    /// substitutions**, so it carries none of the screen's spacing characters.
    #[test]
    fn the_diary_holds_the_words_before_the_typography_point() {
        let (record, record_path) = a_record("verbatim");
        let (diary, area) = a_diary("verbatim");
        let run = an_identity(IdentityKind::Run);
        let job = DurableSubject::HistoryRecord(an_identity(IdentityKind::Job));

        let mut records = FailureRecords::over(record.connection());
        records
            .an_ending_was_recorded(&job, "the render did not finish")
            .unwrap();
        let standing = records.standing_against(&job).unwrap().remove(0);

        // A sentence carrying both of the point's triggers.
        let composed = "The render did not finish — Smoozy does not know why.";
        the_diary_is_handed_the_sentence(
            Some(&diary),
            &mut records,
            Some(&run),
            &standing,
            WhoseSentenceItIs::TheComposersOwn(composed),
        );

        let held = what_the_diary_holds(&area);
        assert!(held.contains(composed));
        assert!(
            !held.contains('\u{00A0}'),
            "**X7: the point is on the path to the screen and on no other path, \
             and the diary is not a screen.** A non-breaking space in the diary \
             means the sentence was captured after the substitutions"
        );
        let _ = std::fs::remove_dir_all(&area);
        let _ = std::fs::remove_file(&record_path);
    }

    /// **M32: a path that composes a stand-in sentence captures nothing, and
    /// the record's one capture is still there for the screen that composes the
    /// sentence the user reads.**
    ///
    /// **This is the fault the clause's new sentence was written for.** The
    /// plumbing's stand-in used to burn the capture, so a problem report told
    /// the Creator that Smoozy had said *Something is not working* when it had
    /// said something else entirely.
    #[test]
    fn a_stand_in_captures_nothing_and_the_real_sentence_still_can() {
        let (record, record_path) = a_record("stand-in");
        let (diary, area) = a_diary("stand-in");
        let run = an_identity(IdentityKind::Run);
        let job = DurableSubject::HistoryRecord(an_identity(IdentityKind::Job));

        let mut records = FailureRecords::over(record.connection());
        records
            .an_ending_was_recorded(&job, "the render did not finish")
            .unwrap();
        let standing = records.standing_against(&job).unwrap().remove(0);

        assert_eq!(
            the_diary_is_handed_the_sentence(
                Some(&diary),
                &mut records,
                Some(&run),
                &standing,
                WhoseSentenceItIs::AStandInThePlumbingComposed("Did not finish."),
            ),
            TheDiaryWasHanded::NothingBecauseItIsAStandIn
        );
        assert!(!what_the_diary_holds(&area).contains("Did not finish."));

        // The record's one capture is unspent, so the screen that composes the
        // sentence the user reads still fills the diary.
        let standing = records.standing_against(&job).unwrap().remove(0);
        assert!(!standing.the_diary_holds_the_sentence);
        assert_eq!(
            the_diary_is_handed_the_sentence(
                Some(&diary),
                &mut records,
                Some(&run),
                &standing,
                WhoseSentenceItIs::TheComposersOwn("The render stopped at frame 41."),
            ),
            TheDiaryWasHanded::TheSentence
        );
        assert!(what_the_diary_holds(&area).contains("The render stopped at frame 41."));
        let _ = std::fs::remove_dir_all(&area);
        let _ = std::fs::remove_file(&record_path);
    }

    /// **M32's second limb: a sentence shown with no record reaches the diary
    /// once per presentation, with the server row, the label beside it and the
    /// file's path — and a stand-in reaches it not at all.**
    #[test]
    fn a_sentence_with_no_record_is_captured_once_per_presentation() {
        let (diary, area) = a_diary("no-record");
        let run = an_identity(IdentityKind::Run);
        let server = an_identity(IdentityKind::LocalServer);
        let presented = APresentationWithNoRecord {
            server_row: &server,
            the_users_label: "the big box",
            the_files_recorded_path: "/ephemeral/smoozy/assets/kitchen_v14.blend",
        };
        let composed = "This Blender could not open this file.";
        for _ in 0..2 {
            assert_eq!(
                the_diary_is_handed_a_sentence_with_no_record(
                    Some(&diary),
                    Some(&run),
                    &presented,
                    WhoseSentenceItIs::TheComposersOwn(composed),
                ),
                TheDiaryWasHanded::TheSentence
            );
        }
        assert_eq!(
            the_diary_is_handed_a_sentence_with_no_record(
                Some(&diary),
                Some(&run),
                &presented,
                WhoseSentenceItIs::AStandInThePlumbingComposed("Smoozy cannot do this yet."),
            ),
            TheDiaryWasHanded::NothingBecauseItIsAStandIn
        );
        let held = what_the_diary_holds(&area);
        assert_eq!(held.matches(composed).count(), 2, "one line per request");
        assert!(held.contains("kitchen_v14.blend") && held.contains("the big box"));
        assert!(!held.contains("Smoozy cannot do this yet."));
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **`Reporting a problem` X4's closed set: five things and no sixth.**
    #[test]
    fn the_handover_is_the_five_things_and_names_the_subject_kind_for_the_slice() {
        let outcome = DurableSubject::OutcomeRecord(an_identity(IdentityKind::Operation));
        let standing = Standing {
            kind: Kind::Ending,
            subject: outcome,
            identity_or_mark: "one file's transfer".to_string(),
            the_values_it_was_raised_over: Vec::new(),
            the_diary_holds_the_sentence: false,
        };
        let handover = TheFailureHandover::of(&standing, "Two of the nine files did not go up.");
        assert_eq!(handover.kind, "ending");
        assert_eq!(handover.identity_or_mark, "one file's transfer");
        assert_eq!(handover.subject_kind, "outcome record");
        assert_eq!(
            handover.the_presented_sentence,
            "Two of the nine files did not go up."
        );
        assert!(handover.at.is_some());
    }

    /// **`Reporting a problem` X4 and `Reporting a problem` X5: the sentence in
    /// the handover is the one that was presented,
    /// never recomposed.**
    #[test]
    fn the_handovers_sentence_is_the_one_that_was_presented() {
        let server = DurableSubject::ServerRow(an_identity(IdentityKind::LocalServer));
        let standing = Standing {
            kind: Kind::Condition,
            subject: server,
            identity_or_mark: an_identity(IdentityKind::Condition).as_text(),
            the_values_it_was_raised_over: vec!["disconnected".to_string()],
            the_diary_holds_the_sentence: true,
        };
        let composed = "Smoozy could not reach the server.";
        assert_eq!(
            TheFailureHandover::of(&standing, composed).the_presented_sentence,
            composed
        );
    }
}

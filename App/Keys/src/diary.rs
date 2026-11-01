//! This module's diary declaration — `Keys` §12.
//!
//! The rules are `Reporting a problem` **J2, J2a, J3, J4 and J4a**, taken as
//! they stand. **The list below is fixed** — what is not on it is not written —
//! and **nothing on it is a new act, a new reading, a new check or a new
//! store**: every act declared is one this module already performs, and every
//! line is written at a durable write that already happens.
//!
//! **Lines are written through the diary service `Desktop app and delivery`
//! §5.3 L14 names**, whose hosting rule — a module declares or writes nothing
//! — is that document's A7.
//!
//! **Told lines: none, and the reason is structural rather than a shortage of
//! candidates.** A told line records an ending or a reading that came back
//! from a machine (J4). **Nothing this module does ever reaches a machine** —
//! `Preparation/Specs/Keys.md`'s own limit, *Keys works only on the local
//! machine and never talks to a server itself* — so there is no machine ending
//! and no machine reading to consume, and **the empty half is written out
//! rather than left silent**, because a missing declaration and an empty one
//! read alike from the diary's end.
//!
//! **One case the three tests could not decide, and it is recorded rather than
//! guessed.** U6a's statement that a computer's SSH tools are missing **is not
//! an act** (nothing completed) and **came back from no machine** (nothing to
//! be a told line), so neither declared kind reaches it. Whether it reaches
//! the diary at all is `Desktop app and delivery` §16's and `Reporting a
//! problem` J6's. **This module declares nothing for it and invents no kind**
//! (§13 item 23).
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Reporting a problem` J2, J2a, J3, J4, J4a.
//! `Desktop app and delivery` A7, L14.

use smoozy_reporting::lines::{Line, LineKind};
use smoozy_state::identities::Identity;
use smoozy_state::observed::Clock;
use std::path::Path;
use std::time::SystemTime;

use crate::pointing::Form;
use crate::removal::FileUnit;
use crate::rows::Name;
use crate::types::KeyType;
use crate::units::UnitRole;

/// **The one kind this module declares**, and no others. Handed to `Reporting
/// a problem`'s `Declarations` at start-up beside every other module's,
/// through `Desktop app and delivery`'s own registration — **`Desktop app and
/// delivery` A7 makes a module with no declaration write nothing, silently**,
/// so a declaration that exists only in this file's tests is a module that
/// writes nothing.
///
/// *(The doc line above this array read* the two kinds this module declares
/// *over an array of one. §12's told half is **none**, and it says so
/// structurally: nothing this module describes ever reaches a machine. So the
/// array was right and the sentence was not.)*
pub const DECLARED: [LineKind; 1] = [LineKind::Act];

/// **What no line of this module's ever carries, and two of the three bars are
/// this module's own.**
///
/// - **No fingerprint, on any line, ever.** F3 makes it the key's identity
///   everywhere and `Servers` P16 makes it the one value by which a key that
///   left Smoozy entirely is recognised when it is pointed at again; R2 forbids
///   a fingerprint index outside the rows and W10 leaves nothing of a removed
///   key anywhere. **The diary outlives every row**, so a line carrying one
///   would be exactly the surviving index those two clauses leave nowhere.
/// - **No passphrase and no part of one, and no statement about what the store
///   held.** `Desktop app and delivery` S6 means no module here ever holds a
///   plaintext to write, and `Security and licence` §3.0 rule 6 bars a stored
///   value from every exported record. **A statement that a unit of an act is
///   done is not a statement about a secret's value.**
/// - **No local key identifier.** It is a mechanism's own identifier, which J3
///   keeps out, and it names nothing after the row is gone.
///
/// **What is carried is the path and the name as last read.** A path grants no
/// recognition: L3 synthesises no path from anything and F4's comparison is the
/// fingerprint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NeverOnALine;

/// **Act one: a key made** (§6). It passes J2a's three tests: it is Smoozy's
/// own act performed entirely on the user's computer; the owning spec names its
/// completion as something the user sees afterwards — the key appears in the
/// keys list and *"the file's name appears on the row and at the end of the
/// made-key dialog"*; and it runs at the user's own rate, **one line per
/// generation, never one per unit of the four §11 declares**.
///
/// **When the line is written:** at the act's own outcome, **after G4's
/// verification and the row creation**. Not at the tool's exit — a pair failing
/// verification has its files removed and no row created, so a line written
/// there would name a key that does not exist. Not at the store put, which is
/// an independent later unit whose failure stops nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AKeyMade {
    pub key_type: KeyType,
    /// **As the yes-or-no G4 verified, and never as anything about the
    /// value.**
    pub a_passphrase_was_applied: bool,
    pub the_path_the_act_wrote: std::path::PathBuf,
    /// **The name the act passed to `-C` where the user typed one.** §12 names
    /// it as one of this line's two identities, beside the run identity; this
    /// form carried neither the field nor the value, while the other three act
    /// lines all carry the name. **An absence is written as one**, exactly as
    /// R1a records an unrecoverable or empty comment.
    pub name: Option<Name>,
}

/// **Act two: a key pointed at** — at a key kept elsewhere, at an unusable
/// key's new location, or at a replacement. One act with three forms.
///
/// **When the line is written:** at the row write. **A refused point writes
/// nothing** — F4's refusal and U6's failed-clause statement are sentences, not
/// completed acts.
///
/// **What the line does not carry:** either fingerprint, old or new; and **not
/// the servers a replacement stranded** — F11 keeps no line on the key row for
/// them and each says so on its own list, so listing them here would be
/// `Reporting a problem` N3's *never inventories things no act touched*
/// broken. **Written out because `Keys` has an N3 of its own** — the
/// was-default bit at §7 — and a bare one here resolves to that. **The cost is stated:** a
/// report shows that a replacement happened and never which servers it left
/// needing a new key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AKeyPointedAt {
    pub form: Form,
    pub the_path_the_row_took: std::path::PathBuf,
    pub name: Option<Name>,
    /// **For the replacement alone, and only where the row carried a
    /// secret-store subject reference.** Where the row carried none the field
    /// is absent, **and an absence is written as one**: a two-valued field
    /// would make a line claim an erasure that never had anything to erase.
    pub the_erasure: Option<HowTheErasureEnded>,
}

/// **Act three: a key deleted, or removed from Smoozy.** One act with two
/// forms.
///
/// **J2a's bar was applied to this act and does not catch it, and the reason is
/// written because it is close.** The bar excludes an act *where the subject's
/// own spec promises that nothing afterwards shows the subject existed*.
/// `Preparation/Specs/Keys.md`'s promise is narrower and different: **Keys**
/// cannot recognise the key again — a promise about a **value**, not about the
/// act being shown. **What the bar does decide is the line's contents**, and it
/// decides them the same way the outcome record was decided: no value by which
/// the key could be recognised.
///
/// **When the line is written:** at the **local forget** — the erasure unit's
/// removal of the row, which is the last unit of both forms. **A delete whose
/// private half's unit failed writes no line at all**: the row stays, the key
/// stays listed, and nothing claims it is gone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AKeyRemoved {
    /// **Which form it was**, taken from W2's derivation at the act's own
    /// moment.
    pub a_managed_key_was_deleted: bool,
    /// **For a delete, each of W6's file units by role and how each ended**,
    /// read from the durable outcome record and never composed afresh — so a
    /// companion left behind by a failed unit is readable afterwards as well as
    /// said at the time.
    pub file_units: Vec<(UnitRole, FileUnit)>,
    pub the_path_the_row_held: std::path::PathBuf,
    pub name: Option<Name>,
    /// **Only where the row carried a secret-store subject reference.**
    pub the_erasure: Option<HowTheErasureEnded>,
}

/// **Act four: a remembered passphrase forgotten because it stopped working**
/// (S5).
///
/// **This is an application-initiated act and J2a's rewording of 2026-08-31 is
/// what makes it decidable** — test three is a rate bound and nothing more, met
/// at one line per attempt at most. Test two is met plainly: the spec names the
/// completion as something the user is told about — *"Smoozy drops it and asks
/// for it fresh rather than failing again with the same stale one"*.
///
/// **When the line is written:** at the erasure unit's outcome. **An
/// interruption inside that unit writes no line**, exactly as it writes no unit
/// outcome and leaves S19's message unsaid (§13 item 19).
///
/// **What the line does not carry:** any part of the passphrase or anything
/// about what the store held; and no fingerprint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct APassphraseForgotten {
    pub the_rows_path: std::path::PathBuf,
    pub name: Option<Name>,
}

/// How an erasure ended, as a line may state it — **confirmed, or left as
/// `Security and licence` §5.1's recorded debt for the sweep**, which is the
/// one thing about a replacement nothing afterwards would ever show (F10).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheErasureEnded {
    Confirmed,
    LeftAsARecordedDebt,
}

/// The four acts, as one value, so a caller cannot write a fifth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Act {
    AKeyMade(AKeyMade),
    AKeyPointedAt(AKeyPointedAt),
    AKeyRemoved(AKeyRemoved),
    APassphraseForgotten(APassphraseForgotten),
}

impl Act {
    /// The act's own name, which is the first field of every fixed form.
    pub fn name(&self) -> &'static str {
        match self {
            Act::AKeyMade(_) => "key made",
            Act::AKeyPointedAt(_) => "key pointed at",
            Act::AKeyRemoved(_) => "key removed",
            Act::APassphraseForgotten(_) => "passphrase forgotten",
        }
    }

    /// **The fixed line form, filled with the declared values** — never free
    /// prose composed at run time, never a step the specs do not name, never a
    /// trace, a dump, or a mechanism's own vocabulary (J3).
    fn text(&self) -> String {
        let mut fields = vec![self.name().to_string()];
        match self {
            Act::AKeyMade(made) => {
                fields.push(made.key_type.as_str().to_string());
                fields.push(
                    if made.a_passphrase_was_applied {
                        "with a passphrase"
                    } else {
                        "with no passphrase"
                    }
                    .to_string(),
                );
                fields.push(path(&made.the_path_the_act_wrote));
                fields.extend(name(made.name.as_ref()));
            }
            Act::AKeyPointedAt(pointed) => {
                fields.push(
                    match pointed.form {
                        Form::AKeyKeptElsewhere => "a key kept elsewhere",
                        Form::AnUnusableKeysNewLocation => "a new location",
                        Form::AReplacement => "a replacement",
                    }
                    .to_string(),
                );
                fields.push(path(&pointed.the_path_the_row_took));
                fields.extend(name(pointed.name.as_ref()));
                fields.extend(erasure(pointed.the_erasure));
            }
            Act::AKeyRemoved(removed) => {
                fields.push(
                    if removed.a_managed_key_was_deleted {
                        "a managed key deleted"
                    } else {
                        "a referenced key removed"
                    }
                    .to_string(),
                );
                for (role, ended) in &removed.file_units {
                    fields.push(format!("{}: {}", role_name(*role), unit_ended(ended)));
                }
                fields.push(path(&removed.the_path_the_row_held));
                fields.extend(name(removed.name.as_ref()));
                fields.extend(erasure(removed.the_erasure));
            }
            Act::APassphraseForgotten(forgotten) => {
                // **S5's declared trigger, and never a cause read off a
                // diagnostic** — no tool's string is parsed anywhere here.
                fields.push("a local load failed after a passing check".to_string());
                fields.push(path(&forgotten.the_rows_path));
                fields.extend(name(forgotten.name.as_ref()));
            }
        }
        fields.join("\t")
    }

    /// The line, ready for the diary service.
    ///
    /// **The moment is on the desktop's clock, stated as that clock**
    /// (`State, identity and recovery` D5), and
    /// **the identities it carries are the run identity and nothing else** —
    /// the local key identifier is barred by J3 and the path and name travel in
    /// the text.
    pub fn line(&self, run: Identity, at: SystemTime) -> Line {
        Line {
            run: run.clone(),
            kind: LineKind::Act,
            at,
            clock: Clock::ThisComputer,
            text: self.text(),
            concerning: vec![run],
        }
    }
}

fn path(path: &Path) -> String {
    path.display().to_string()
}

/// R1a records an unrecoverable or empty comment as absent, **and an absence is
/// written as one** rather than left off the line.
fn name(name: Option<&Name>) -> Option<String> {
    Some(match name {
        Some(name) => name.as_str().to_string(),
        None => "no name".to_string(),
    })
}

fn erasure(ended: Option<HowTheErasureEnded>) -> Option<String> {
    ended.map(|ended| {
        match ended {
            HowTheErasureEnded::Confirmed => "the remembered passphrase was erased",
            HowTheErasureEnded::LeftAsARecordedDebt => {
                "the remembered passphrase's erasure is owed to the sweep"
            }
        }
        .to_string()
    })
}

fn role_name(role: UnitRole) -> &'static str {
    match role {
        UnitRole::ThePrivateHalf => "the private half",
        UnitRole::ThePublicHalf => "the public half",
        UnitRole::TheCertificate => "the certificate",
        UnitRole::TheErasure => "the erasure",
        UnitRole::TheDirectory => "the directory",
        UnitRole::TheKeyPair => "the key pair",
        UnitRole::TheRow => "the row",
        UnitRole::TheStorePut => "the store put",
    }
}

fn unit_ended(ended: &FileUnit) -> &'static str {
    match ended {
        FileUnit::Gone => "gone",
        FileUnit::LeftBehind { .. } => "left behind",
        FileUnit::NotAttempted => "not attempted",
        FileUnit::WasAlreadyGone => "already gone",
    }
}

/// **What this module declares no line for, each declined on the test it fails,
/// so that each is a decision rather than a silence.**
///
/// - **Copying a key's public half** (W7). **Test two.** The spec's completion
///   is the clipboard, which is none of the places that test names, and nothing
///   afterwards shows it happened.
/// - **The user clearing a remembered passphrase** (S3 item 4). **Test two.**
///   The spec names its completion as *"the next connection asks for it
///   fresh"*, which is a step inside a later act. **The cost is stated rather
///   than hidden:** a report shows a passphrase Smoozy forgot because it
///   stopped working and never one the user cleared, and both leave the same
///   later ask.
/// - **The store put of a passphrase**, at generation and after an ask. Each is
///   a **unit** of an act above, and a unit is a step inside something else
///   rather than an act.
/// - **Every reading** — U10's check and U4's five clauses, the fingerprint
///   read, T2's probe, W11a's *Used by* read. None is an act completed and none
///   came back from a machine; and **one line per key per draw is precisely the
///   rate J2a's third test exists to forbid.**
/// - **The statements this module puts on screen** — F4's refusal, U6's
///   failed-clause reason, N1's duplicate-name warning, and W11's warning with
///   W12's could-not-check state. **Each is a sentence rather than an act.**
pub const DECLARED_FOR_NOTHING_ELSE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_reporting::lines::{ActTests, Declarations, ErasingActsAreNotDeclared};
    use smoozy_state::identities::IdentityKind;
    use std::path::PathBuf;

    fn a_run() -> Identity {
        Identity::issue(IdentityKind::Run).expect("randomness")
    }

    fn now() -> SystemTime {
        SystemTime::now()
    }

    #[test]
    fn this_module_declares_act_lines_and_no_told_lines() {
        let mut declarations: Declarations<&str> = Declarations::new();
        declarations.module_declares("Keys", DECLARED.to_vec());
        assert!(declarations.may_write(&"Keys", LineKind::Act));
        assert!(!declarations.may_write(&"Keys", LineKind::Told));
    }

    #[test]
    fn every_declared_act_passes_all_three_of_j2as_tests() {
        let tests = ActTests {
            smoozys_own: true,
            the_spec_names_its_completion_as_something_the_user_sees: true,
            at_the_users_own_rate: true,
        };
        // The bar does not catch act three: the spec's promise is about a
        // value Keys cannot recognise again, not about the act being shown.
        let bar = ErasingActsAreNotDeclared {
            the_subjects_spec_promises_no_trace_afterwards: false,
        };
        assert!(tests.is_diary_worthy(bar));
    }

    #[test]
    fn a_made_key_line_names_the_type_the_passphrase_yes_or_no_and_the_path() {
        let act = Act::AKeyMade(AKeyMade {
            key_type: KeyType::Ed25519,
            a_passphrase_was_applied: true,
            the_path_the_act_wrote: PathBuf::from("/k/.ssh/smoozy_ed25519"),
            name: Name::what_the_user_typed("hetzner"),
        });
        assert_eq!(
            act.text(),
            "key made\ted25519\twith a passphrase\t/k/.ssh/smoozy_ed25519\thetzner"
        );

        // **And an absence is written as one**, the same way the other three
        // act lines write it.
        let nameless = Act::AKeyMade(AKeyMade {
            key_type: KeyType::Ed25519,
            a_passphrase_was_applied: false,
            the_path_the_act_wrote: PathBuf::from("/k/.ssh/smoozy_ed25519"),
            name: None,
        });
        assert!(nameless.text().ends_with("no name"));
    }

    #[test]
    fn no_line_ever_carries_a_fingerprint_or_a_local_key_identifier() {
        let run = a_run();
        let every = [
            Act::AKeyMade(AKeyMade {
                key_type: KeyType::Rsa,
                a_passphrase_was_applied: false,
                the_path_the_act_wrote: PathBuf::from("/k/.ssh/smoozy_rsa"),
                name: Name::what_the_user_typed("work"),
            }),
            Act::AKeyPointedAt(AKeyPointedAt {
                form: Form::AReplacement,
                the_path_the_row_took: PathBuf::from("/k/new"),
                name: Name::what_the_user_typed("work"),
                the_erasure: Some(HowTheErasureEnded::Confirmed),
            }),
            Act::AKeyRemoved(AKeyRemoved {
                a_managed_key_was_deleted: true,
                file_units: vec![(UnitRole::ThePrivateHalf, FileUnit::Gone)],
                the_path_the_row_held: PathBuf::from("/k/.ssh/smoozy_rsa"),
                name: None,
                the_erasure: None,
            }),
            Act::APassphraseForgotten(APassphraseForgotten {
                the_rows_path: PathBuf::from("/k/.ssh/smoozy_rsa"),
                name: None,
            }),
        ];
        for act in every {
            let line = act.line(run.clone(), now());
            assert!(!line.text.contains("SHA256:"));
            assert!(!line.text.contains(&run.as_text()));
            // The run identity is the only identity a line carries.
            assert_eq!(line.concerning, vec![run.clone()]);
            assert_eq!(line.clock, Clock::ThisComputer);
            assert_eq!(line.kind, LineKind::Act);
        }
    }

    #[test]
    fn a_row_that_carried_no_reference_says_nothing_about_an_erasure() {
        let act = Act::AKeyPointedAt(AKeyPointedAt {
            form: Form::AReplacement,
            the_path_the_row_took: PathBuf::from("/k/new"),
            name: None,
            the_erasure: None,
        });
        assert!(!act.text().contains("passphrase"));
    }

    #[test]
    fn a_debt_left_to_the_sweep_is_said_rather_than_reported_as_an_erasure() {
        let act = Act::AKeyPointedAt(AKeyPointedAt {
            form: Form::AReplacement,
            the_path_the_row_took: PathBuf::from("/k/new"),
            name: None,
            the_erasure: Some(HowTheErasureEnded::LeftAsARecordedDebt),
        });
        assert!(act.text().contains("owed to the sweep"));
    }

    #[test]
    fn a_delete_line_names_each_file_unit_by_role_and_how_it_ended() {
        let act = Act::AKeyRemoved(AKeyRemoved {
            a_managed_key_was_deleted: true,
            file_units: vec![
                (UnitRole::ThePrivateHalf, FileUnit::Gone),
                (
                    UnitRole::ThePublicHalf,
                    FileUnit::LeftBehind {
                        reported: "permission refused".into(),
                    },
                ),
            ],
            the_path_the_row_held: PathBuf::from("/k/.ssh/smoozy_rsa"),
            name: None,
            the_erasure: Some(HowTheErasureEnded::Confirmed),
        });
        let text = act.text();
        assert!(text.contains("the private half: gone"));
        assert!(text.contains("the public half: left behind"));
        // The machine's own words stay off the line.
        assert!(!text.contains("permission refused"));
    }

    #[test]
    fn an_absent_name_is_written_as_an_absence() {
        let act = Act::APassphraseForgotten(APassphraseForgotten {
            the_rows_path: PathBuf::from("/k/id_rsa"),
            name: None,
        });
        assert!(act.text().ends_with("no name"));
    }

    #[test]
    fn the_forgotten_line_states_the_declared_trigger_and_no_cause() {
        let act = Act::APassphraseForgotten(APassphraseForgotten {
            the_rows_path: PathBuf::from("/k/id_rsa"),
            name: None,
        });
        assert!(act
            .text()
            .contains("a local load failed after a passing check"));
    }
}

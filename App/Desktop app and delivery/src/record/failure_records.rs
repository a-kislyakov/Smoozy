//! Failure records in the local record — `State, identity and recovery` §8,
//! and `Desktop app and delivery` §5.
//!
//! **The rule is that document's and the store is this one's**, which is the
//! split every other row module here draws and the one
//! [`super::outcome_records`] states in as many words. **A2 holds: no module
//! has a durable store of its own.**
//!
//! ## Whether this table should exist at all, settled 2026-09-09
//!
//! **It should, and the reading that said otherwise was reading §8.2's title
//! rather than §8.2.** `Audits/Build/Step 10a/The shared layer — what each
//! piece owns and in what order.md` §C.4 reads *"whether the condition record
//! is durable on this side or derived at draw time is §8.2's answer — it is a
//! derivation, never a copy — so what this piece needs is the derivation and
//! its suppliers, not a new table"*, and instructed piece C to check it
//! against §8 before building anything. It was checked and §8 says the
//! opposite, in five places.
//!
//! - **`State, identity and recovery` J1, at that document's §1.1, names
//!   *failure records* in the
//!   local record's own inventory**, which is the closed list of every durable
//!   state fact on this side, and `Desktop app and delivery` L1 makes that
//!   inventory the authority for what this store holds.
//! - **§11.1 puts failure records in the subject-held class**, with the two
//!   extra endings §8.8 names. A thing with a lifetime in that table is a
//!   thing that is stored.
//! - **§8.2 F7 says a spent record *"is removed at the next write to its
//!   subject"***. Nothing is removed that was never written down.
//! - **§8.7 F21: *"The record is kept against the durable subject; the
//!   volatile surface is a place it is shown."*** `State, identity and
//!   recovery` F24 turns that into a
//!   promise: *"Nothing about knowing what went wrong depends on the
//!   application having been open."*
//! - **`State, identity and recovery` F25's second ending, at that document's
//!   §8.8, is the user clearing a condition record**, which has
//!   to survive the run or the mark comes back at the next launch.
//!
//! **What §8.2 actually says is a narrower and sharper thing, and it is kept
//! here rather than argued with.** F5: the record holds the **condition
//! identity** and the **set of observed values it was raised over**, and *"no
//! second copy of the current value and no independent notion of whether the
//! condition still holds"*. **So the durable part is the record and the
//! derived part is whether it is shown** — which is computed at draw time, in
//! [`crate::presentation::the_failure_shape`], from the value the named
//! supplier reports. There is no `current_value` column here and there never
//! may be one: that column is M6's harm, the mark and the world drifting
//! apart, in the one shape a table makes easy.
//!
//! ## What is deliberately not in this table
//!
//! - **No prose.** §8.2 keeps the record *"a derivation holding no prose"*,
//!   and M32 captures the sentence where it exists — at presentation — into
//!   the diary. A sentence column here would be a second place the words live
//!   and would let a presentation that has changed since make the diary lie.
//! - **No moment, no age, no count.** `State, identity and recovery` F26, at
//!   that document's §8.8: *"no age, no cap, no sweep, no
//!   schedule"*. A timestamp here is the shape of a sweep nobody may write,
//!   sitting in the table waiting for somebody — which is exactly the fault
//!   `suppressed_warning` already carries and which piece F is briefed to
//!   settle.
//! - **No copy of the subject outcome.** §8.1 F3: the ending record *"names the
//!   subject outcome and holds no second copy of it, so the mark and the fact
//!   are never one field"*. The mark is a name; the outcome lives in the
//!   subject's own record. *(This quoted F3 as it read until 2026-09-19, when
//!   the Gemini catch-up of step 10a, Area 4, U-1 made an ending's mark not the
//!   user's to clear.)*
//!
//! ## How a clause marker is written in this crate, and it is a rule rather
//! than a habit
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! file's own header list.
//!
//! **`State, identity and recovery`'s own markers are written out with that
//! document at every site below and are deliberately not listed here.** This
//! plan defines clauses of the same short names — an F5, an F11, an F21 — and a
//! header list cannot tell the two apart, so a bare one would resolve to a real
//! and wrong clause. **`Desktop app and delivery` L1, L11, M4, M6 and M32 are
//! this crate's own and are written bare.**

use rusqlite::OptionalExtension;
use smoozy_state::identities::{Identity, IdentityKind};

use super::RecordFailure;

/// The two tables. Named here so the guard test and the schema read one place.
pub const TABLE: &str = "failure_record";
/// The set of observed values a condition record was raised over — `State,
/// identity and recovery` §8.2 F5. A set, so a table.
pub const VALUES_TABLE: &str = "failure_record_raised_over";

/// **The two kinds, and the kind is declared by whatever raised the record** —
/// `State, identity and recovery` §8.1 F1, and M4 from the presentation's side:
/// *"Every failure record carries its kind and the presentation reads it rather
/// than guessing."*
///
/// **`State, identity and recovery` F2: a writer that cannot say which kind it
/// is raising has not established what it is reporting**, so there is no third
/// variant and no default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A statement about something still live, which can stop being true.
    Condition,
    /// A statement about something that has already happened, which cannot.
    Ending,
}

impl Kind {
    pub fn as_text(self) -> &'static str {
        match self {
            Kind::Condition => "condition",
            Kind::Ending => "ending",
        }
    }

    /// **The written kind read back, and nothing is guessed.** The store reads
    /// its own column with it; [`crate::surface::the_failure_shape`] reads a
    /// clearing request's with it, which is H34's validation of a value that
    /// arrived from the page.
    pub fn from_text(text: &str) -> Option<Kind> {
        match text {
            "condition" => Some(Kind::Condition),
            "ending" => Some(Kind::Ending),
            _ => None,
        }
    }
}

/// **§8.7 F21's closed list of durable subjects, and it is closed here because
/// it is closed there.**
///
/// `State, identity and recovery` F23 says why the list is stated once rather
/// than declared per raiser: what
/// §8.7 promises is that **no failure record's only subject is a volatile
/// one**, and that is a promise about the list rather than about each raiser's
/// care. *"So a document raising a condition against something not named above
/// brings it here and the list grows"* — which is a change to that plan and
/// then to this enum, in that order, and never a variant added here first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DurableSubject {
    ServerRow(Identity),
    KeyRow(Identity),
    TransferArrangement(Identity),
    /// Provisional or confirmed alike — `State, identity and recovery` §7 step
    /// 2 and §11.1 both take the two together.
    JobRecord(Identity),
    HistoryRecord(Identity),
    /// **The outcome record of a multi-part action**, which is what gives a
    /// unit's failure record a durable subject by construction — including for
    /// an action that has no other durable subject of its own.
    OutcomeRecord(Identity),
    /// Historical installation-wide licence subject without an identity.
    LicenceRecord,
}

/// The licence record's written form. It is not an identity and nothing
/// anywhere may read it as one.
const THE_LICENCE_RECORD: &str = "the licence record";

impl DurableSubject {
    /// The subject's written form, as the row holds it.
    pub fn as_text(&self) -> String {
        match self {
            DurableSubject::ServerRow(identity)
            | DurableSubject::KeyRow(identity)
            | DurableSubject::TransferArrangement(identity)
            | DurableSubject::JobRecord(identity)
            | DurableSubject::HistoryRecord(identity)
            | DurableSubject::OutcomeRecord(identity) => identity.as_text(),
            DurableSubject::LicenceRecord => THE_LICENCE_RECORD.to_string(),
        }
    }

    /// Which of F21's list it is, written down beside the subject so that two
    /// subjects of different kinds sharing an identity's rendering can never be
    /// read as one.
    pub fn kind_as_text(&self) -> &'static str {
        match self {
            DurableSubject::ServerRow(_) => "server row",
            DurableSubject::KeyRow(_) => "key row",
            DurableSubject::TransferArrangement(_) => "transfer arrangement",
            DurableSubject::JobRecord(_) => "job record",
            DurableSubject::HistoryRecord(_) => "history record",
            DurableSubject::OutcomeRecord(_) => "outcome record",
            DurableSubject::LicenceRecord => "licence record",
        }
    }

    /// The identity, where the subject has one. **The licence record has none**
    /// and answers `None` rather than a made-up value.
    pub fn identity(&self) -> Option<&Identity> {
        match self {
            DurableSubject::ServerRow(identity)
            | DurableSubject::KeyRow(identity)
            | DurableSubject::TransferArrangement(identity)
            | DurableSubject::JobRecord(identity)
            | DurableSubject::HistoryRecord(identity)
            | DurableSubject::OutcomeRecord(identity) => Some(identity),
            DurableSubject::LicenceRecord => None,
        }
    }
}

/// **§8.3 F10's condition key: the subject, the condition's kind, and — where
/// the condition is about a place on a machine — the location it was read
/// against, in one normalised form.**
///
/// **It is a lookup and never an identity** (F11): it is not in §2.2's
/// register, it is never handed to another document in place of the identity,
/// and **nothing outside this file reads it**. It is a column of the row it
/// finds rather than a table of its own, so there is one home for *this key
/// already has an identity*.
///
/// **The normalising is the raiser's and is not done here.** F10's own reason
/// is Fact 28 — free space *"is not a reading, it is a choice of which mount to
/// read"* — so the location arrives already in the one form that module uses,
/// and a second normalising here would be a second answer to the same question.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionKey(String);

impl ConditionKey {
    /// Compose the key. **The three parts are F10's and there is no fourth.**
    pub fn over(subject: &DurableSubject, kind: &str, at_the_location: Option<&str>) -> Self {
        ConditionKey(format!(
            "{}\u{1f}{}\u{1f}{}\u{1f}{}",
            subject.kind_as_text(),
            subject.as_text(),
            kind,
            at_the_location.unwrap_or("")
        ))
    }
}

/// One failure record as it is read back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Standing {
    pub kind: Kind,
    pub subject: DurableSubject,
    /// **The condition identity, for a condition record; the subject outcome
    /// mark, for an ending record** — which is the pair `Reporting a problem`
    /// X4's handover names in those words, and M33 supplies.
    pub identity_or_mark: String,
    /// **§8.2 F5: the set of observed values it was raised over.** Empty for an
    /// ending record, which holds over nothing and is withdrawn by nothing.
    pub the_values_it_was_raised_over: Vec<String>,
    /// **M32's *once*.** True once the plumbing has handed the diary this
    /// record's composed sentence, so a second run does not write a second
    /// line for a record the user met once.
    pub the_diary_holds_the_sentence: bool,
}

impl Standing {
    /// **§8.2 F6: presented while, and only while, the current observed value
    /// is one of the values it was raised over.**
    ///
    /// **An ending record is never withdrawn by a value** — it is a statement
    /// about something that has already happened, so it stands until one of
    /// §8.8's other two endings.
    pub fn still_holds_at(&self, observed_now: &str) -> bool {
        match self.kind {
            Kind::Ending => true,
            Kind::Condition => self
                .the_values_it_was_raised_over
                .iter()
                .any(|value| value == observed_now),
        }
    }
}

/// Why a write was refused. **Each is a rule of §8 kept rather than a fault of
/// the store's**; the store's own troubles come back as
/// [`Refused::TheStore`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refused {
    /// **`State, identity and recovery` F12: a condition with no named supplier
    /// cannot be withdrawn and must not be raised.** The declaration route is
    /// [`crate::presentation::declarations`] and this is its teeth.
    ThatConditionIsNotDeclared,
    /// **F5: a condition holds over at least one value.** A set of none is a
    /// record nothing could ever show and nothing could ever withdraw.
    ItHoldsOverNoValue,
    /// **F1 and M4: an identity of the wrong kind is not a condition
    /// identity.**
    ThatIsNotAConditionIdentity,
    TheStore(String),
}

fn the_store(error: rusqlite::Error) -> Refused {
    Refused::TheStore(error.to_string())
}

/// The keeper, over one open local record.
pub struct FailureRecords<'a> {
    connection: &'a rusqlite::Connection,
}

impl<'a> FailureRecords<'a> {
    pub fn over(connection: &'a rusqlite::Connection) -> Self {
        FailureRecords { connection }
    }

    /// **§8.3 F11: compose the key first and look it up.**
    ///
    /// *"Where the key already has a condition identity against it, that
    /// identity is used, no second record is raised, and the existing record is
    /// the one there is to withdraw. Where it does not, a fresh condition
    /// identity is issued and recorded against the key."*
    ///
    /// **`declared` is the caller's proof that
    /// [`crate::presentation::declarations`] names this condition** — F12's
    /// rule, kept by the store rather than by the raiser remembering it.
    ///
    /// **The identity and the value set go down in one transaction.** A record
    /// standing with an empty set would be one nothing could ever show, which
    /// F6 makes indistinguishable from a record that is being withheld.
    pub fn a_condition_was_raised(
        &mut self,
        key: &ConditionKey,
        subject: &DurableSubject,
        holds_over: &[&str],
        declared: bool,
    ) -> Result<Identity, Refused> {
        if !declared {
            return Err(Refused::ThatConditionIsNotDeclared);
        }
        if holds_over.is_empty() {
            return Err(Refused::ItHoldsOverNoValue);
        }

        let already: Option<String> = self
            .connection
            .query_row(
                "SELECT identity_or_mark FROM failure_record WHERE condition_key = ?1",
                [&key.0],
                |row| row.get(0),
            )
            .optional()
            .map_err(the_store)?;
        if let Some(already) = already {
            return Identity::read_back(IdentityKind::Condition, &already)
                .ok_or(Refused::ThatIsNotAConditionIdentity);
        }

        let identity = Identity::issue(IdentityKind::Condition).map_err(|_| {
            Refused::TheStore("this computer offered no randomness for an identity".into())
        })?;
        let text = identity.as_text();

        self.connection
            .execute("BEGIN IMMEDIATE", [])
            .map_err(the_store)?;
        let written = (|| -> Result<(), rusqlite::Error> {
            self.connection.execute(
                "INSERT INTO failure_record
                    (kind, durable_subject, durable_subject_kind, identity_or_mark,
                     condition_key, the_diary_holds_the_sentence)
                 VALUES ('condition', ?1, ?2, ?3, ?4, 0)",
                rusqlite::params![subject.as_text(), subject.kind_as_text(), text, key.0],
            )?;
            for value in holds_over {
                self.connection.execute(
                    "INSERT OR IGNORE INTO failure_record_raised_over
                        (durable_subject, durable_subject_kind, identity_or_mark, value)
                     VALUES (?1, ?2, ?3, ?4)",
                    rusqlite::params![subject.as_text(), subject.kind_as_text(), text, value],
                )?;
            }
            Ok(())
        })();
        match written {
            Ok(()) => {
                self.connection.execute("COMMIT", []).map_err(the_store)?;
                Ok(identity)
            }
            Err(error) => {
                let _ = self.connection.execute("ROLLBACK", []);
                Err(the_store(error))
            }
        }
    }

    /// **§8.1 F3: the mark raised against a subject outcome.**
    ///
    /// The mark names the outcome and holds no second copy of it. **M26: a
    /// failure record raised against a failed unit hangs off the outcome record
    /// as its durable subject**, and the mark is which unit — in the owning
    /// module's own words, exactly as the outcome record's role is.
    ///
    /// Raising the same mark twice against the same subject is the same mark:
    /// there is one attention mark per outcome, not one per time it was
    /// noticed.
    pub fn an_ending_was_recorded(
        &mut self,
        subject: &DurableSubject,
        subject_outcome_mark: &str,
    ) -> Result<(), Refused> {
        self.connection
            .execute(
                "INSERT OR IGNORE INTO failure_record
                    (kind, durable_subject, durable_subject_kind, identity_or_mark,
                     condition_key, the_diary_holds_the_sentence)
                 VALUES ('ending', ?1, ?2, ?3, NULL, 0)",
                rusqlite::params![
                    subject.as_text(),
                    subject.kind_as_text(),
                    subject_outcome_mark
                ],
            )
            .map_err(the_store)?;
        Ok(())
    }

    /// **Every failure record standing against this subject** — §8.7 F21's
    /// *"the record is kept against the durable subject"*, read back.
    ///
    /// **M7a: two conditions standing at the same moment come back as two.**
    /// Nothing here merges them, orders them by anything but the order they
    /// were raised in, or composes a statement about a cause they share.
    pub fn standing_against(
        &self,
        subject: &DurableSubject,
    ) -> Result<Vec<Standing>, RecordFailure> {
        let text = subject.as_text();
        let mut asking = self.connection.prepare(
            "SELECT kind, identity_or_mark, the_diary_holds_the_sentence
             FROM failure_record
             WHERE durable_subject = ?1 AND durable_subject_kind = ?2
             ORDER BY rowid",
        )?;
        let rows = asking.query_map(rusqlite::params![&text, subject.kind_as_text()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
            ))
        })?;

        let mut out = Vec::new();
        for row in rows {
            let (kind, identity_or_mark, diary) = row?;
            // **A row whose kind is neither word is read as no record at
            // all** rather than guessed at — the same refusal
            // `super::outcome_records` makes for an outcome that will not read
            // back, and M4's *reads it rather than guessing* from the other
            // side.
            let Some(kind) = Kind::from_text(&kind) else {
                continue;
            };
            out.push(Standing {
                kind,
                subject: subject.clone(),
                the_values_it_was_raised_over: self.values_of(
                    &text,
                    subject.kind_as_text(),
                    &identity_or_mark,
                )?,
                identity_or_mark,
                the_diary_holds_the_sentence: diary != 0,
            });
        }
        Ok(out)
    }

    fn values_of(
        &self,
        subject: &str,
        subject_kind: &str,
        mark: &str,
    ) -> Result<Vec<String>, RecordFailure> {
        let mut asking = self.connection.prepare(
            "SELECT value FROM failure_record_raised_over
             WHERE durable_subject = ?1 AND durable_subject_kind = ?2 AND identity_or_mark = ?3
             ORDER BY value",
        )?;
        let rows = asking.query_map(rusqlite::params![subject, subject_kind, mark], |row| {
            row.get::<_, String>(0)
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    /// **§8.2 F6 and F7: the named supplier reported a value, and where it is
    /// outside the set the record is withdrawn by that change alone.**
    ///
    /// Answers whether it withdrew anything.
    ///
    /// **Two things this is not.** It is not a sweep and it is not a watcher:
    /// it runs only where §8.4 F15's three moments already deliver a value —
    /// a transition the named supplier reports, the next attempt at the action
    /// that raised it, or a reading the user's own act already causes.
    /// **Nothing here goes looking**, and there is no route into this file that
    /// takes a reading.
    ///
    /// **The row is removed rather than marked spent, and that is deliberate.**
    /// F7 says a withdrawn record *"is never presented again, whatever the
    /// value does afterwards"* and *"is removed at the next write to its
    /// subject"*. The withdrawal **is** a write to its subject, so removing it
    /// here keeps both halves; and a `spent` column would be the *"independent
    /// notion of whether the condition still holds"* F5 forbids by name, sitting
    /// in the table for somebody to read wrongly. **A condition that returns is
    /// a fresh identity** — Q12 and F7's own second consequence — and it is
    /// fresh here by there being no row left for its key.
    pub fn the_observed_value_moved(
        &mut self,
        key: &ConditionKey,
        observed_now: &str,
    ) -> Result<bool, Refused> {
        let found: Option<(String, String, String)> = self
            .connection
            .query_row(
                "SELECT durable_subject, durable_subject_kind, identity_or_mark
                 FROM failure_record
                 WHERE condition_key = ?1",
                [&key.0],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .optional()
            .map_err(the_store)?;
        let Some((subject, subject_kind, mark)) = found else {
            return Ok(false);
        };
        let holds: bool = self
            .connection
            .query_row(
                "SELECT count(*) > 0 FROM failure_record_raised_over
                 WHERE durable_subject = ?1 AND durable_subject_kind = ?2
                   AND identity_or_mark = ?3 AND value = ?4",
                rusqlite::params![&subject, &subject_kind, &mark, observed_now],
                |row| row.get(0),
            )
            .map_err(the_store)?;
        if holds {
            return Ok(false);
        }
        self.remove(&subject, &subject_kind, &mark)?;
        Ok(true)
    }

    /// **`State, identity and recovery` F25's second ending, at that document's
    /// §8.8: the user clears a condition record themselves.**
    ///
    /// **This removal reads no kind.** An ending is not the user's to clear
    /// (`State, identity and recovery` F25, and M5), and the one caller that acts for the user —
    /// [`crate::surface::the_failure_shape::the_user_cleared_one`] — refuses an
    /// ending before it reaches here.
    ///
    /// **M5: it ends the record and nothing else.** There is nothing in
    /// this file that could touch a subject outcome — the mark names one and
    /// holds no copy of one — so *the clearing gesture takes the mark and never
    /// the fact* is true by there being no other field to reach.
    ///
    /// **It addresses the row by all three parts of its key.** Until
    /// 2026-09-12 it left the subject's kind out, so clearing a job record's
    /// failure also took the history record's failure of the same name off a
    /// screen the user never touched.
    pub fn the_user_cleared_it(
        &mut self,
        subject: &DurableSubject,
        identity_or_mark: &str,
    ) -> Result<(), Refused> {
        self.remove(
            &subject.as_text(),
            subject.kind_as_text(),
            identity_or_mark,
        )
    }

    /// **`State, identity and recovery` F25's first ending, at that document's
    /// §8.8: the subject is removed, so its records go
    /// with it.**
    ///
    /// **This is the one ending that takes the subject outcome with it as well
    /// as the mark**, and the outcome's own removal is its own record's — not
    /// this file's.
    pub fn the_subject_was_removed(&mut self, subject: &DurableSubject) -> Result<(), Refused> {
        let text = subject.as_text();
        let subject_kind = subject.kind_as_text();
        self.connection
            .execute("BEGIN IMMEDIATE", [])
            .map_err(the_store)?;
        let removed = (|| -> Result<(), rusqlite::Error> {
            self.connection.execute(
                "DELETE FROM failure_record_raised_over
                 WHERE durable_subject = ?1 AND durable_subject_kind = ?2",
                rusqlite::params![&text, subject_kind],
            )?;
            self.connection.execute(
                "DELETE FROM failure_record
                 WHERE durable_subject = ?1 AND durable_subject_kind = ?2",
                rusqlite::params![&text, subject_kind],
            )?;
            Ok(())
        })();
        match removed {
            Ok(()) => {
                self.connection.execute("COMMIT", []).map_err(the_store)?;
                Ok(())
            }
            Err(error) => {
                let _ = self.connection.execute("ROLLBACK", []);
                Err(the_store(error))
            }
        }
    }

    /// **M32's *once*, written down.**
    ///
    /// The plumbing has handed the diary this record's composed sentence, so no
    /// later presentation of the same record hands it a second one. **It is on
    /// the record's own row because the record is what M32 keys on** — *"at the
    /// first presentation per condition identity or ending record"* — and a
    /// mark kept anywhere else would be the same fact in two places.
    pub fn the_diary_now_holds_the_sentence(
        &mut self,
        subject: &DurableSubject,
        identity_or_mark: &str,
    ) -> Result<(), Refused> {
        self.connection
            .execute(
                "UPDATE failure_record SET the_diary_holds_the_sentence = 1
                 WHERE durable_subject = ?1 AND durable_subject_kind = ?2
                   AND identity_or_mark = ?3",
                rusqlite::params![
                    subject.as_text(),
                    subject.kind_as_text(),
                    identity_or_mark
                ],
            )
            .map_err(the_store)?;
        Ok(())
    }

    /// **Every statement in here addresses a row by all three parts of the
    /// key** — the subject, the subject's kind and the mark. Two of the three
    /// is not the key: `State, identity and recovery` §8.7's list holds a job's
    /// own record and its history entry, which are two subjects carrying one
    /// identity's rendering, so a statement that leaves the kind out reaches
    /// both.
    fn remove(
        &mut self,
        subject: &str,
        subject_kind: &str,
        identity_or_mark: &str,
    ) -> Result<(), Refused> {
        self.connection
            .execute("BEGIN IMMEDIATE", [])
            .map_err(the_store)?;
        let removed = (|| -> Result<(), rusqlite::Error> {
            self.connection.execute(
                "DELETE FROM failure_record_raised_over
                 WHERE durable_subject = ?1 AND durable_subject_kind = ?2
                   AND identity_or_mark = ?3",
                rusqlite::params![subject, subject_kind, identity_or_mark],
            )?;
            self.connection.execute(
                "DELETE FROM failure_record
                 WHERE durable_subject = ?1 AND durable_subject_kind = ?2
                   AND identity_or_mark = ?3",
                rusqlite::params![subject, subject_kind, identity_or_mark],
            )?;
            Ok(())
        })();
        match removed {
            Ok(()) => {
                self.connection.execute("COMMIT", []).map_err(the_store)?;
                Ok(())
            }
            Err(error) => {
                let _ = self.connection.execute("ROLLBACK", []);
                Err(the_store(error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::LocalRecord;

    fn a_record(name: &str) -> LocalRecord {
        let path = crate::scratch_folders::a_file(format!("smoozy-failure-records-{name}-{}.sqlite3", std::process::id()));
        LocalRecord::open_at(&path).expect("the desk can make a record")
    }

    fn a_server() -> DurableSubject {
        DurableSubject::ServerRow(
            Identity::issue(IdentityKind::LocalServer).expect("the desk has randomness"),
        )
    }

    /// **`State, identity and recovery` F11: one identity per key, and no
    /// second record.**
    #[test]
    fn one_key_raised_twice_is_one_record_and_one_identity() {
        let record = a_record("one-key");
        let mut records = FailureRecords::over(record.connection());
        let server = a_server();
        let key = ConditionKey::over(&server, "the server connection", None);

        let first = records
            .a_condition_was_raised(&key, &server, &["disconnected", "reconnecting"], true)
            .expect("it is raised");
        let again = records
            .a_condition_was_raised(&key, &server, &["disconnected", "reconnecting"], true)
            .expect("the same key answers with the same identity");

        assert_eq!(first, again);
        assert_eq!(records.standing_against(&server).unwrap().len(), 1);
    }

    /// **F6 and F7: the value moving outside the set withdraws the record by
    /// that change alone, and the record is never shown again.**
    #[test]
    fn the_value_leaving_the_set_withdraws_the_record() {
        let record = a_record("withdraw");
        let mut records = FailureRecords::over(record.connection());
        let server = a_server();
        let key = ConditionKey::over(&server, "the server connection", None);
        records
            .a_condition_was_raised(&key, &server, &["disconnected", "reconnecting"], true)
            .unwrap();

        // A value inside the set changes nothing: an outage reads as
        // *disconnected* and as *reconnecting* by turns while it lasts.
        assert!(!records
            .the_observed_value_moved(&key, "reconnecting")
            .unwrap());
        assert_eq!(records.standing_against(&server).unwrap().len(), 1);

        assert!(records.the_observed_value_moved(&key, "connected").unwrap());
        assert!(records.standing_against(&server).unwrap().is_empty());
    }

    /// **F7's second consequence, and Q12's: a condition that returns is a
    /// fresh identity, said once again.**
    #[test]
    fn a_condition_that_returns_is_a_fresh_identity() {
        let record = a_record("fresh");
        let mut records = FailureRecords::over(record.connection());
        let server = a_server();
        let key = ConditionKey::over(&server, "the server connection", None);

        let first = records
            .a_condition_was_raised(&key, &server, &["disconnected"], true)
            .unwrap();
        records.the_observed_value_moved(&key, "connected").unwrap();
        let second = records
            .a_condition_was_raised(&key, &server, &["disconnected"], true)
            .unwrap();

        assert_ne!(first, second);
    }

    /// **F12: a condition with no named supplier must not be raised.**
    #[test]
    fn an_undeclared_condition_is_refused() {
        let record = a_record("undeclared");
        let mut records = FailureRecords::over(record.connection());
        let server = a_server();
        let key = ConditionKey::over(&server, "something nobody declared", None);
        assert_eq!(
            records.a_condition_was_raised(&key, &server, &["gone"], false),
            Err(Refused::ThatConditionIsNotDeclared)
        );
        assert!(records.standing_against(&server).unwrap().is_empty());
    }

    /// **F5: a record holding over no value is one nothing could ever show and
    /// nothing could ever withdraw.**
    #[test]
    fn a_condition_over_no_value_is_refused() {
        let record = a_record("empty-set");
        let mut records = FailureRecords::over(record.connection());
        let server = a_server();
        let key = ConditionKey::over(&server, "the server connection", None);
        assert_eq!(
            records.a_condition_was_raised(&key, &server, &[], true),
            Err(Refused::ItHoldsOverNoValue)
        );
    }

    /// **F10: a disk-full condition naming *the machine* would fold two
    /// conditions that are not one into a single record**, so the location is
    /// part of the key.
    #[test]
    fn two_mounts_on_one_machine_are_two_conditions() {
        let record = a_record("two-mounts");
        let mut records = FailureRecords::over(record.connection());
        let server = a_server();
        let one = ConditionKey::over(&server, "disk full at a mount", Some("/workspace"));
        let other = ConditionKey::over(&server, "disk full at a mount", Some("/output"));

        let first = records
            .a_condition_was_raised(&one, &server, &["nothing left"], true)
            .unwrap();
        let second = records
            .a_condition_was_raised(&other, &server, &["nothing left"], true)
            .unwrap();

        assert_ne!(first, second);
        assert_eq!(records.standing_against(&server).unwrap().len(), 2);
    }

    /// **`State, identity and recovery` F13 and `State, identity and recovery`
    /// F28: a disk-full and a
    /// permission refusal on the same mount are two, because the kind
    /// differs.**
    #[test]
    fn two_kinds_at_one_mount_are_two_conditions() {
        let record = a_record("two-kinds");
        let mut records = FailureRecords::over(record.connection());
        let server = a_server();
        let full = ConditionKey::over(&server, "disk full at a mount", Some("/workspace"));
        let refused = ConditionKey::over(
            &server,
            "a place on a machine that refused a write for any other reason",
            Some("/workspace"),
        );
        records
            .a_condition_was_raised(&full, &server, &["nothing left"], true)
            .unwrap();
        records
            .a_condition_was_raised(&refused, &server, &["the last write here was refused"], true)
            .unwrap();
        assert_eq!(records.standing_against(&server).unwrap().len(), 2);
    }

    /// **M7a: two conditions standing at once are read back as two**, each with
    /// its own set, and nothing here composes a third thing out of them.
    #[test]
    fn two_standing_conditions_are_read_back_as_two_with_their_own_sets() {
        let record = a_record("two-standing");
        let mut records = FailureRecords::over(record.connection());
        let server = a_server();
        let connection = ConditionKey::over(&server, "the server connection", None);
        let supervisor = ConditionKey::over(&server, "the supervisor unavailable", None);
        records
            .a_condition_was_raised(&connection, &server, &["disconnected", "reconnecting"], true)
            .unwrap();
        records
            .a_condition_was_raised(&supervisor, &server, &["not answering"], true)
            .unwrap();

        let standing = records.standing_against(&server).unwrap();
        assert_eq!(standing.len(), 2);
        assert_eq!(standing[0].the_values_it_was_raised_over.len(), 2);
        assert_eq!(standing[1].the_values_it_was_raised_over, vec!["not answering"]);
    }

    /// **M5: the removal ends the record and nothing else.** It is driven here
    /// over an ending because an ending is the one kind that names an outcome:
    /// nothing in this file can touch that outcome, because the row holds no
    /// copy of one. **The user never reaches this over an ending** — the
    /// command refuses one (`State, identity and recovery` F25, M5; the Gemini
    /// catch-up of step 10a, Area 4, F-5).
    #[test]
    fn clearing_takes_the_mark_and_the_row_never_held_the_fact() {
        let record = a_record("clear");
        let mut records = FailureRecords::over(record.connection());
        let outcome = DurableSubject::OutcomeRecord(
            Identity::issue(IdentityKind::Operation).expect("the desk has randomness"),
        );
        records
            .an_ending_was_recorded(&outcome, "one file's transfer")
            .unwrap();
        assert_eq!(records.standing_against(&outcome).unwrap().len(), 1);

        records
            .the_user_cleared_it(&outcome, "one file's transfer")
            .unwrap();
        assert!(records.standing_against(&outcome).unwrap().is_empty());
    }

    /// **An ending record is never withdrawn by a value.** It is a statement
    /// about something that has already happened.
    #[test]
    fn an_ending_record_holds_over_nothing_and_always_stands() {
        let record = a_record("ending-stands");
        let mut records = FailureRecords::over(record.connection());
        let history = DurableSubject::HistoryRecord(
            Identity::issue(IdentityKind::Job).expect("the desk has randomness"),
        );
        records
            .an_ending_was_recorded(&history, "the render did not finish")
            .unwrap();
        let standing = records.standing_against(&history).unwrap();
        assert!(standing[0].the_values_it_was_raised_over.is_empty());
        assert!(standing[0].still_holds_at("anything at all"));
    }

    /// **`State, identity and recovery` F25's first ending: the subject going
    /// takes its records with it.**
    #[test]
    fn removing_the_subject_removes_its_records() {
        let record = a_record("subject-gone");
        let mut records = FailureRecords::over(record.connection());
        let server = a_server();
        let key = ConditionKey::over(&server, "the server connection", None);
        records
            .a_condition_was_raised(&key, &server, &["disconnected"], true)
            .unwrap();
        records.the_subject_was_removed(&server).unwrap();
        assert!(records.standing_against(&server).unwrap().is_empty());

        // And the value set went with it, so nothing is left to be read back
        // against a record that is gone.
        let left: i64 = record
            .connection()
            .query_row("SELECT count(*) FROM failure_record_raised_over", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(left, 0);
    }

    /// **M32's *once*, across runs.** The mark is on the record's own row, so a
    /// record raised in one run and presented again in the next hands the diary
    /// nothing a second time.
    #[test]
    fn the_diary_mark_is_on_the_record_and_survives_the_run() {
        let record = a_record("once");
        let server = a_server();
        let key = ConditionKey::over(&server, "the server connection", None);
        {
            let mut records = FailureRecords::over(record.connection());
            records
                .a_condition_was_raised(&key, &server, &["disconnected"], true)
                .unwrap();
            let standing = records.standing_against(&server).unwrap();
            assert!(!standing[0].the_diary_holds_the_sentence);
            records
                .the_diary_now_holds_the_sentence(&server, &standing[0].identity_or_mark)
                .unwrap();
        }
        let path = record.path().to_path_buf();
        drop(record);

        let again = LocalRecord::open_at(&path).expect("it opens again");
        let records = FailureRecords::over(again.connection());
        assert!(records.standing_against(&server).unwrap()[0].the_diary_holds_the_sentence);
    }

    /// **The key is three parts and every statement uses all three.** A job's
    /// own record and its history entry are two of §8.7's subjects carrying one
    /// identity's rendering, so a statement addressing a row by the name and
    /// the mark alone reaches both — which is one failure the user cleared
    /// taking a second failure they never touched off another screen. Found at
    /// round 1 of the code audit of what is drawn inside the window (F-1).
    #[test]
    fn clearing_one_kind_of_subject_leaves_the_other_kinds_record_standing() {
        let record = a_record("two-kinds-one-name");
        let mut records = FailureRecords::over(record.connection());
        let identity = Identity::issue(IdentityKind::Job).expect("the desk has randomness");
        let job = DurableSubject::JobRecord(identity.clone());
        let history = DurableSubject::HistoryRecord(identity);
        assert_eq!(job.as_text(), history.as_text());

        records
            .an_ending_was_recorded(&job, "the render did not finish")
            .unwrap();
        records
            .an_ending_was_recorded(&history, "the render did not finish")
            .unwrap();
        assert_eq!(records.standing_against(&job).unwrap().len(), 1);
        assert_eq!(records.standing_against(&history).unwrap().len(), 1);

        records
            .the_user_cleared_it(&job, "the render did not finish")
            .unwrap();
        assert!(records.standing_against(&job).unwrap().is_empty());
        assert_eq!(
            records.standing_against(&history).unwrap().len(),
            1,
            "clearing one subject's failure must not take another subject's"
        );
    }

    /// **The same three parts on the subject's own removal and on the diary
    /// mark.** Removing the job record takes the job record's failures and
    /// leaves the history entry's; marking one as captured does not mark the
    /// other.
    #[test]
    fn the_subjects_removal_and_the_diary_mark_both_keep_the_kind_apart() {
        let record = a_record("two-kinds-one-name-removal");
        let mut records = FailureRecords::over(record.connection());
        let identity = Identity::issue(IdentityKind::Job).expect("the desk has randomness");
        let job = DurableSubject::JobRecord(identity.clone());
        let history = DurableSubject::HistoryRecord(identity);

        records
            .an_ending_was_recorded(&job, "the render did not finish")
            .unwrap();
        records
            .an_ending_was_recorded(&history, "the render did not finish")
            .unwrap();

        records
            .the_diary_now_holds_the_sentence(&job, "the render did not finish")
            .unwrap();
        assert!(records.standing_against(&job).unwrap()[0].the_diary_holds_the_sentence);
        assert!(
            !records.standing_against(&history).unwrap()[0].the_diary_holds_the_sentence,
            "one subject's capture must not spend another subject's"
        );

        records.the_subject_was_removed(&job).unwrap();
        assert!(records.standing_against(&job).unwrap().is_empty());
        assert_eq!(records.standing_against(&history).unwrap().len(), 1);
    }

    /// **The value set is keyed the same way**, so one subject's withdrawal
    /// does not empty another subject's set and the two sets are read back
    /// apart.
    #[test]
    fn two_subjects_of_one_name_keep_their_own_value_sets() {
        let record = a_record("two-kinds-one-name-values");
        let mut records = FailureRecords::over(record.connection());
        let identity = Identity::issue(IdentityKind::Job).expect("the desk has randomness");
        let job = DurableSubject::JobRecord(identity.clone());
        let history = DurableSubject::HistoryRecord(identity);
        let on_the_job = ConditionKey::over(&job, "the server connection", None);
        let on_the_history = ConditionKey::over(&history, "the server connection", None);

        records
            .a_condition_was_raised(&on_the_job, &job, &["disconnected"], true)
            .unwrap();
        records
            .a_condition_was_raised(
                &on_the_history,
                &history,
                &["disconnected", "reconnecting"],
                true,
            )
            .unwrap();

        assert_eq!(
            records.standing_against(&job).unwrap()[0]
                .the_values_it_was_raised_over
                .len(),
            1
        );
        assert_eq!(
            records.standing_against(&history).unwrap()[0]
                .the_values_it_was_raised_over
                .len(),
            2
        );

        assert!(records
            .the_observed_value_moved(&on_the_job, "connected")
            .unwrap());
        assert!(records.standing_against(&job).unwrap().is_empty());
        assert_eq!(
            records.standing_against(&history).unwrap()[0]
                .the_values_it_was_raised_over
                .len(),
            2
        );
    }

    /// **F5, kept by there being no column to break it with.** The table holds
    /// no current value, no prose and no moment, and this is the line that
    /// fails if one is ever added.
    #[test]
    fn the_table_holds_no_current_value_no_prose_and_no_moment() {
        let record = a_record("columns");
        let mut columns: Vec<String> = Vec::new();
        let mut asking = record
            .connection()
            .prepare("PRAGMA table_info(failure_record)")
            .unwrap();
        let rows = asking
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap();
        for row in rows {
            columns.push(row.unwrap());
        }
        assert_eq!(
            columns,
            vec![
                "kind",
                "durable_subject",
                "durable_subject_kind",
                "identity_or_mark",
                "condition_key",
                "the_diary_holds_the_sentence"
            ],
            "**`State, identity and recovery` F5: the record holds no second copy \
             of the current value and no independent notion of whether the \
             condition still holds**, §8.2 keeps it a derivation holding no \
             prose, and `State, identity and recovery` F26 forbids the age this \
             table would grow a sweep for. \
             A column added here is one of those three unless it is neither"
        );
    }
}

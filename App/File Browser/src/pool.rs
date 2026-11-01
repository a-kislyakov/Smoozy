//! The pool and the order — `File Browser` §6, O1 to O8, Rows 188 and 189.
//!
//! **Constraint:** Row 188 — *"A set number of transfers run at once per server
//! and the rest wait their turn, and the user does not set that number"* — and
//! Row 189's settled order, *"auto-download first, then manual download, then
//! the uploads a submission is waiting on, then ordinary uploads."*
//!
//! **The mechanism is `Server protocol and transfers` T5, T6 and T8 to T10 and
//! is not re-solved here.** What is declared here is the number, the order and
//! the shares. **Nothing about the order is decided at run time** (O6): the
//! classes, the order and the share rule below are the complete declaration.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` T2, T5, T6, T8, T9, T10.
//! `Servers` 1.4. `Download` Q1, Row 206, Row 207.
//! `Dependency Upload & Remap` A9. `Rendering` §7. `Desktop app and delivery`
//! R1.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause.

/// **O1: the per-server pool is four transfer channels.**
///
/// The measured useful range is two to four and differs by direction — the knee
/// is two streams down and about four up, and a number above the range buys
/// setup-hiding rather than throughput (`Server protocol and transfers` T6,
/// Fact 25). **One pool serves both directions** (the spec's own sentence), so
/// it is sized at the larger knee: a pool below four starves the upload
/// direction — this document's direction, and the slow one at 1.2–1.3 MB/s
/// against 12–15 down — for no gain anywhere, and a pool above four buys nothing
/// Fact 25 measured.
///
/// **The user does not set it**, which is Row 188's own second half: nothing
/// reads this from anywhere and no setting exposes it.
pub const THE_PER_SERVER_POOL: usize = 4;

/// **O1's second half: the number is the pool's size, not a promise of
/// sessions.**
///
/// `Server protocol and transfers` T2 makes the session cap the server's, **a
/// refused channel opens fewer**, and at a cap of one T5 alternates reading and
/// command work on the primary connection while `Servers` 1.4's additional
/// connection remains transfer-only.
pub fn how_many_channels_the_pool_actually_opens(the_server_allowed: usize) -> usize {
    the_server_allowed.min(THE_PER_SERVER_POOL)
}

/// **O2: inside the pool, a per-direction ceiling keeps the wrong direction from
/// spending slots it cannot use.**
///
/// **The upload direction's ceiling is the pool itself — four — declared here.**
/// **The download direction's ceiling is Row 206's to declare inside the same
/// structure**, and `Download` Q1 declares it — two transfer channels — and its
/// §14 supplies it here by name.
///
/// **Two ceilings in one pool, not two pools**: the count the machine sees is
/// one count, admitted through the one per-server point (T5). **`Download` is
/// step 8's**, so its figure arrives as a value rather than being spelled again
/// here — writing another document's constant into this crate would give one
/// fact two homes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheCeilings {
    pub upload: usize,
    /// **`Download` Q1's**, taken as declared.
    pub download: usize,
}

/// **The upload ceiling, declared here**: the pool itself.
pub const THE_UPLOAD_DIRECTIONS_CEILING: usize = THE_PER_SERVER_POOL;

impl TheCeilings {
    /// O2, with `Download` Q1's figure supplied by that document.
    pub fn with_the_ceiling_download_q1_declares(download: usize) -> TheCeilings {
        TheCeilings {
            upload: THE_UPLOAD_DIRECTIONS_CEILING,
            download,
        }
    }

    /// **One pool, two ceilings.** Neither ceiling may exceed the pool, and the
    /// two together are not a second pool: what the machine sees is one count.
    pub fn are_inside_one_pool(&self) -> bool {
        self.upload <= THE_PER_SERVER_POOL && self.download <= THE_PER_SERVER_POOL
    }
}

/// **O3: what actually governs a busy line is above the pool, and the pool does
/// not pretend otherwise.**
///
/// The desktop-side admission point knows every server at once, and **the two
/// shared rate budgets are `Desktop app and delivery`'s** — a whole second
/// machine bought about 20%, and uncoordinated upload fan-out was actively worse
/// than one machine at a time (Fact 25). *"The pool exists so that one server's
/// transfers queue fairly at that server and the command channel keeps its
/// reserved session; throughput lives in the budgets."*
pub const THROUGHPUT_LIVES_IN_THE_BUDGETS_AND_NOT_IN_THIS_POOL: bool = true;

/// One declared transfer-priority class, with its rank in O4's order.
///
/// **The classes are declared by their arranging modules per T8; what Row 189
/// fixes is the order among them**, around the pair the spec fixed first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeclaredClass {
    /// The class name its arranging module declares.
    pub name: &'static str,
    /// The document that declares the class.
    pub declared_by: &'static str,
    /// Its rank in O4's list. **Lower is earlier.**
    pub rank: u8,
}

/// **O4: the declared classes, in one order — and the top pair is not this
/// document's to order.**
///
/// `Preparation/Specs/Download.md` settles it, **Creator-stated** (2026-07-22):
/// *"Auto-download transfers take priority over manual download transfers in the
/// server's transfer queue."* That sentence is Row 207's own promise and the
/// settled half of this three-way order; **this document declares the rest
/// around it.**
///
/// 1. **auto-download** — first because the spec settles it above manual
///    download, and the settled half sits at the top rather than mid-list so the
///    declared order contains it legibly.
/// 2. **manual download** — below auto-download by the Creator's sentence and
///    above the upload classes by this document's own reasoning: the line is
///    eleven-to-twelve times faster down than up (Fact 25), so a download slot
///    moving the same bytes turns over that much sooner.
/// 3. **dependency-upload** (`Dependency Upload & Remap` A9's class) — an
///    interactive sitting whose rows gate a submission the user is trying to
///    make.
/// 4. **upload** (U10's class) — user-asked bulk, including the re-sent `.blend`
///    a fresh request carries; below dependency-upload because the sitting's rows
///    block a render and a bulk upload blocks nothing but itself.
/// 5. **the preview fetch**, which `Queue & History` P6 already declared *"below
///    every user-asked class and below auto-download, never starved"* — **taken
///    as declared, adding nothing.**
pub const THE_DECLARED_ORDER: [DeclaredClass; 5] = [
    DeclaredClass {
        name: "auto-download",
        declared_by: "Download",
        rank: 1,
    },
    DeclaredClass {
        name: "manual-download",
        declared_by: "Download",
        rank: 2,
    },
    DeclaredClass {
        name: "dependency-upload",
        declared_by: "Dependency Upload & Remap",
        rank: 3,
    },
    DeclaredClass {
        name: crate::declarations::THE_TRANSFER_PRIORITY_CLASS,
        declared_by: "File Browser",
        rank: 4,
    },
    DeclaredClass {
        name: "preview-fetch",
        declared_by: "Queue & History",
        rank: 5,
    },
];

/// **O4's closing sentence, which is the route a later document takes: a class a
/// later document declares takes its rank by that document's declaration against
/// this list.**
///
/// `Rendering` §7's **submission class** is one, ranked below manual download and
/// above dependency-upload, **which makes six and eleven rather than five and
/// nine.** That class is built, at `smoozy_rendering::units::THE_SUBMISSION_CLASS`,
/// and **nothing of it is written into [`THE_DECLARED_ORDER`] here** — which is
/// the point of the route rather than a thing waiting on a step. *(Corrected
/// 2026-09-07: this gave the reason as `Rendering` being step 7's, which stopped
/// being true when step 7 closed.)* What this function does is take a later
/// document's declaration and place it, **and no edit to that list is owed.**
pub fn with_a_class_a_later_document_declares(
    declared: &[DeclaredClass],
    joining: DeclaredClass,
) -> Vec<DeclaredClass> {
    let mut all: Vec<DeclaredClass> = declared.to_vec();
    all.push(joining);
    all.sort_by_key(|class| class.rank);
    all
}

/// **L29's picture view, placed against O4's list: third — below manual
/// download, above dependency-upload.** Added 2026-10-31, the Creator's
/// seventeenth notes, note 11.
///
/// **By O4's own route** — *"a class a later document declares takes its rank
/// by that document's declaration against this list"* — and L29 declares it
/// the same way, so [`THE_DECLARED_ORDER`] stays O4's five as the plan lists
/// them and the worked figure of nine stays the figure for those five. **The
/// ranks are re-read from the order handed back**, one to six, so nothing here
/// restates a rank of O4's list. **N in O5 counts it**: six and eleven.
pub fn the_declared_order_with_the_picture_view() -> Vec<DeclaredClass> {
    let spaced: Vec<DeclaredClass> = THE_DECLARED_ORDER
        .iter()
        .map(|class| DeclaredClass {
            rank: class.rank * 2,
            ..*class
        })
        .collect();
    let manual_download = spaced
        .iter()
        .find(|class| class.name == "manual-download")
        .map(|class| class.rank)
        .unwrap_or(0);
    let mut placed = with_a_class_a_later_document_declares(
        &spaced,
        DeclaredClass {
            name: crate::declarations::THE_PICTURE_VIEW_CLASS,
            declared_by: "File Browser",
            rank: manual_download + 1,
        },
    );
    for (at, class) in placed.iter_mut().enumerate() {
        class.rank = at as u8 + 1;
    }
    placed
}

/// **O5's N: the count of declared classes, wherever they are declared.**
///
/// **A builder computes N from the declarations and never reads the worked
/// figure as the bound** — the plan says so in its own words, because a class a
/// later document declares counts into N and widens the bound with it.
pub fn n_over(declared: &[DeclaredClass]) -> usize {
    declared.len()
}

/// **O5's bound: at most 2N−1 grants at that server.**
///
/// *"N grants before it qualifies, and at most the N−1 other classes that could
/// qualify at the same decision taking theirs ahead of it by rank."*
///
/// **That is the number, and it is bounded rather than gated**, which is T10's
/// requirement in its own words: *no class is starved* survives, and the
/// guarantee scales by construction as documents declare classes in.
pub fn the_bound(n: usize) -> usize {
    2 * n - 1
}

/// One admission decision that has already been taken.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ADecision {
    waiting: Vec<&'static str>,
    granted: &'static str,
}

/// **One admission point** — a server's, or the desktop-side one above them.
///
/// **O8: the desktop-side point holds the same declared order across servers, so
/// the classes mean the same thing globally that they mean at one machine; and
/// it applies O5's share rule too, with grants counted at that point.**
///
/// *"The order alone would let one server's auto-download arrangement, which
/// keeps trying and never switches itself off (Row 219), outrank another
/// server's only upload at every desktop-side decision for ever, which is T10's
/// forbidden shape one layer above where T10 forbids it."*
///
/// **O5 counts grants at that server; the desktop-side point counts its own,
/// with the same N and the same rank tie-breaker.** So this is one type used
/// twice rather than two rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionPoint {
    declared: Vec<DeclaredClass>,
    /// **Only the last N decisions are kept**, N being what [`Self::n`]
    /// returns. [`Self::qualifiers`] is the one thing that reads them and it
    /// reads exactly that many; a point that kept every decision it has ever
    /// taken — each carrying its own list of the classes that were waiting —
    /// would grow without bound over a desktop session that runs for weeks and
    /// moves tens of thousands of files, for nothing. **O5's rule, its 2N−1
    /// bound and its rank tie-breaker are unchanged**: the answer over any run
    /// of grants is the answer the full history gave.
    decisions: Vec<ADecision>,
    /// **The only thing that read the whole list**, kept as a plain count.
    grants_made: usize,
}

impl AdmissionPoint {
    /// An admission point over a declared order.
    pub fn over(declared: Vec<DeclaredClass>) -> AdmissionPoint {
        let mut declared = declared;
        declared.sort_by_key(|class| class.rank);
        AdmissionPoint {
            declared,
            decisions: Vec::new(),
            grants_made: 0,
        }
    }

    /// N, from the declarations this point holds.
    pub fn n(&self) -> usize {
        n_over(&self.declared)
    }

    /// The bound this point's own N gives.
    pub fn the_bound(&self) -> usize {
        the_bound(self.n())
    }

    fn rank_of(&self, name: &str) -> Option<u8> {
        self.declared
            .iter()
            .find(|class| class.name == name)
            .map(|class| class.rank)
    }

    /// **O5's exception: a class which has had work waiting throughout the last
    /// N grants at this point without receiving one.**
    fn qualifiers(&self, waiting: &[&'static str]) -> Vec<&'static str> {
        let n = self.n();
        // **Fewer than N grants have been made at all**, so no class can have
        // waited throughout the last N of them.
        if self.grants_made < n {
            return Vec::new();
        }
        // What is held *is* the last N, which is what makes the trimmed store
        // give the same answer the full history gave.
        let recent = &self.decisions[self.decisions.len() - n..];
        waiting
            .iter()
            .copied()
            .filter(|class| {
                recent
                    .iter()
                    .all(|decision| decision.waiting.contains(class) && decision.granted != *class)
            })
            .collect()
    }

    /// **O5: at each admission decision the highest-ranked class with work
    /// waiting is admitted — except that a class which has had work waiting
    /// throughout the last N grants at that server without receiving one takes
    /// the next grant.**
    ///
    /// **Where more than one class qualifies for that exception at the same
    /// decision, the highest-ranked of them takes the grant.** *"The tie is
    /// broken here because a rule that leaves it open can be carried out four
    /// ways — highest-ranked-among-qualifiers, longest-waiting-first,
    /// lowest-ranked-first, by arrival — each producing a different admission
    /// sequence, which is exactly the run-time class T8 forbids."*
    ///
    /// **Rank is the tie-breaker because it costs nothing and is how the plan
    /// answers *who goes first* everywhere else.**
    ///
    /// `None` where nothing is waiting: **admission happens when a slot frees,
    /// and nothing preempts a running transfer.**
    pub fn grant(&mut self, waiting: &[&'static str]) -> Option<&'static str> {
        let mut waiting: Vec<&'static str> = waiting
            .iter()
            .copied()
            .filter(|class| self.rank_of(class).is_some())
            .collect();
        waiting.sort_by_key(|class| self.rank_of(class).unwrap_or(u8::MAX));
        waiting.dedup();
        if waiting.is_empty() {
            return None;
        }
        let qualifiers = self.qualifiers(&waiting);
        // Both lists are already in rank order, so the first of either is the
        // highest-ranked — the exception's own tie-breaker, and the ordinary
        // rule, in one line each.
        let granted = *qualifiers.first().unwrap_or(&waiting[0]);
        self.decisions.push(ADecision { waiting, granted });
        self.grants_made += 1;
        // **Nothing older than the last N is ever read**, so nothing older is
        // kept.
        let n = self.n();
        if self.decisions.len() > n {
            self.decisions.remove(0);
        }
        Some(granted)
    }

    /// How many grants this point has made.
    pub fn grants_made(&self) -> usize {
        self.grants_made
    }
}

/// **O6: nothing about the order is decided at run time.**
///
/// *"The classes, the order and the share rule above are the complete
/// declaration; two callers cannot decide differently because there is nothing
/// left for a caller to decide."*
pub const NOTHING_ABOUT_THE_ORDER_IS_DECIDED_AT_RUN_TIME: bool = true;

/// **O7: within a class, admission is by arrival** (T8), **and within a request,
/// by the enumeration's order** — which gives the user the plain reading that a
/// folder uploads roughly front to back.
pub fn within_a_request_admission_follows_the_enumeration(order: &[usize]) -> bool {
    order.windows(2).all(|pair| pair[0] < pair[1])
}

/// **O8: the moment the order bites is one moment — a repaired connection
/// resuming auto-downloads, manual downloads and uploads across several servers
/// at once** (T9).
///
/// *"Each server's pool admits by O4–O5; the desktop-side point above them holds
/// the same declared order across servers … Nothing at the repair is decided
/// that was not declared here."*
///
/// **`Desktop app and delivery` holds the rate budgets, not admission** (its
/// R1), and the budgets divide bandwidth beneath all of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheRepairMoment {
    /// One point per server.
    pub at_each_server: Vec<AdmissionPoint>,
    /// **The one point above them**, with its own grants counted.
    pub above_them_all: AdmissionPoint,
}

impl TheRepairMoment {
    pub fn over(declared: Vec<DeclaredClass>, servers: usize) -> TheRepairMoment {
        TheRepairMoment {
            at_each_server: (0..servers)
                .map(|_| AdmissionPoint::over(declared.clone()))
                .collect(),
            above_them_all: AdmissionPoint::over(declared),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const AUTO: &str = "auto-download";
    const MANUAL: &str = "manual-download";
    const DEPENDENCY: &str = "dependency-upload";
    const UPLOAD: &str = "upload";
    const PREVIEW: &str = "preview-fetch";

    fn all_five() -> Vec<DeclaredClass> {
        THE_DECLARED_ORDER.to_vec()
    }

    #[test]
    fn the_pool_is_four_and_the_server_may_give_fewer() {
        assert_eq!(THE_PER_SERVER_POOL, 4);
        assert_eq!(how_many_channels_the_pool_actually_opens(10), 4);
        assert_eq!(
            how_many_channels_the_pool_actually_opens(1),
            1,
            "a cap of one opens one, and `Servers` 1.4's additional connection is that document's"
        );
    }

    #[test]
    fn two_ceilings_sit_in_one_pool() {
        // `Download` Q1's figure, supplied rather than spelled again here.
        let ceilings = TheCeilings::with_the_ceiling_download_q1_declares(2);
        assert_eq!(ceilings.upload, 4);
        assert_eq!(ceilings.download, 2);
        assert!(ceilings.are_inside_one_pool());
        assert_eq!(THE_UPLOAD_DIRECTIONS_CEILING, THE_PER_SERVER_POOL);
    }

    #[test]
    fn the_order_is_the_five_declared_and_the_settled_pair_is_at_the_top() {
        let names: Vec<&str> = THE_DECLARED_ORDER.iter().map(|class| class.name).collect();
        assert_eq!(
            names,
            vec![AUTO, MANUAL, DEPENDENCY, UPLOAD, PREVIEW],
            "Row 189's own order"
        );
        // The settled half is the Creator's sentence and is not this
        // document's: it is declared by `Download` and sits at the top so the
        // order contains it legibly.
        assert_eq!(THE_DECLARED_ORDER[0].declared_by, "Download");
        assert_eq!(THE_DECLARED_ORDER[1].declared_by, "Download");
        // This document's own class is the one it declares and no other.
        let ours: Vec<&str> = THE_DECLARED_ORDER
            .iter()
            .filter(|class| class.declared_by == "File Browser")
            .map(|class| class.name)
            .collect();
        assert_eq!(ours, vec![crate::declarations::THE_TRANSFER_PRIORITY_CLASS]);
    }

    #[test]
    fn the_worked_figure_is_nine_for_five_classes_and_the_preview_takes_the_ninth() {
        let mut point = AdmissionPoint::over(all_five());
        assert_eq!(point.n(), 5);
        assert_eq!(point.the_bound(), 9);
        let waiting = [AUTO, MANUAL, DEPENDENCY, UPLOAD, PREVIEW];
        let mut granted = Vec::new();
        for _ in 0..9 {
            granted.push(point.grant(&waiting).expect("something is waiting"));
        }
        assert_eq!(
            granted,
            vec![AUTO, AUTO, AUTO, AUTO, AUTO, MANUAL, DEPENDENCY, UPLOAD, PREVIEW],
            "the plan's own worked sequence"
        );
        assert_eq!(
            granted[8], PREVIEW,
            "the preview fetch's first grant is the ninth"
        );
    }

    #[test]
    fn n_and_the_bound_are_computed_from_the_declarations_and_never_read_off_the_worked_figure() {
        // `Rendering` §7's submission class, ranked below manual download and
        // above dependency-upload. That class is built, at
        // `smoozy_rendering::units::THE_SUBMISSION_CLASS`; nothing of it is
        // written into the declared order here, which is the route rather than
        // a thing waiting on a step. (Corrected 2026-09-07: this gave the
        // reason as that document being step 7's.)
        let submission = DeclaredClass {
            name: "submission",
            declared_by: "Rendering",
            rank: 3,
        };
        let mut six = with_a_class_a_later_document_declares(&THE_DECLARED_ORDER, submission);
        // The joining class ranks above dependency-upload, so ranks are
        // re-read from the declarations rather than from positions.
        for (at, class) in six.iter_mut().enumerate() {
            class.rank = at as u8 + 1;
        }
        assert_eq!(n_over(&six), 6);
        assert_eq!(
            the_bound(n_over(&six)),
            11,
            "six and eleven, not five and nine"
        );
        let point = AdmissionPoint::over(six);
        assert_eq!(point.the_bound(), 11);
    }

    #[test]
    fn a_class_waiting_throughout_gets_a_grant_inside_the_bound() {
        let mut point = AdmissionPoint::over(all_five());
        let bound = point.the_bound();
        let waiting = [AUTO, PREVIEW];
        let mut when = None;
        for grant in 1..=bound {
            if point.grant(&waiting) == Some(PREVIEW) {
                when = Some(grant);
                break;
            }
        }
        let when = when.expect("the lowest-ranked class is never starved");
        assert!(when <= bound, "a grant inside 2N-1, and it came at {when}");
    }

    /// **The trimmed store answers exactly what the full history answered, and
    /// it does not grow.**
    ///
    /// A point that kept every decision it ever took grew without bound over a
    /// long session, each decision carrying its own list of the classes that
    /// were waiting, and the only thing that read the whole list was the count.
    #[test]
    fn keeping_only_the_last_n_decisions_answers_what_the_whole_history_answered() {
        /// The point as it was: every decision kept for ever.
        struct TheWholeHistory {
            declared: Vec<DeclaredClass>,
            decisions: Vec<ADecision>,
        }
        impl TheWholeHistory {
            fn grant(&mut self, waiting: &[&'static str]) -> Option<&'static str> {
                let rank_of = |name: &str| {
                    self.declared
                        .iter()
                        .find(|class| class.name == name)
                        .map(|class| class.rank)
                };
                let mut waiting: Vec<&'static str> = waiting
                    .iter()
                    .copied()
                    .filter(|class| rank_of(class).is_some())
                    .collect();
                waiting.sort_by_key(|class| rank_of(class).unwrap_or(u8::MAX));
                waiting.dedup();
                if waiting.is_empty() {
                    return None;
                }
                let n = n_over(&self.declared);
                let qualifiers: Vec<&'static str> = if self.decisions.len() < n {
                    Vec::new()
                } else {
                    let recent = &self.decisions[self.decisions.len() - n..];
                    waiting
                        .iter()
                        .copied()
                        .filter(|class| {
                            recent.iter().all(|decision| {
                                decision.waiting.contains(class) && decision.granted != *class
                            })
                        })
                        .collect()
                };
                let granted = *qualifiers.first().unwrap_or(&waiting[0]);
                self.decisions.push(ADecision { waiting, granted });
                Some(granted)
            }
        }

        let mut point = AdmissionPoint::over(all_five());
        let mut whole = TheWholeHistory {
            declared: all_five(),
            decisions: Vec::new(),
        };
        // A long run with the waiting list changing, so the *waiting
        // throughout* exception fires, lapses and fires again.
        let lists: [&[&'static str]; 6] = [
            &[AUTO, UPLOAD, PREVIEW],
            &[AUTO, UPLOAD],
            &[AUTO, DEPENDENCY, UPLOAD, PREVIEW],
            &[AUTO],
            &[MANUAL, UPLOAD, PREVIEW],
            &[AUTO, MANUAL, DEPENDENCY, UPLOAD, PREVIEW],
        ];
        for grant in 0..2_000 {
            let waiting = lists[grant % lists.len()];
            assert_eq!(
                point.grant(waiting),
                whole.grant(waiting),
                "the two answers parted at grant {grant}"
            );
        }
        assert_eq!(point.grants_made(), 2_000);
        assert_eq!(point.grants_made(), whole.decisions.len());
        // **And the store does not grow with the number of grants.**
        assert_eq!(point.decisions.len(), point.n());
        assert_eq!(whole.decisions.len(), 2_000);
    }

    #[test]
    fn the_tie_among_qualifiers_is_broken_by_rank_and_not_by_arrival() {
        let mut point = AdmissionPoint::over(all_five());
        let waiting = [AUTO, DEPENDENCY, PREVIEW];
        // Five grants to auto-download, then both other classes qualify at the
        // same decision. Rank decides, so dependency-upload takes it.
        for _ in 0..5 {
            assert_eq!(point.grant(&waiting), Some(AUTO));
        }
        assert_eq!(point.grant(&waiting), Some(DEPENDENCY));
    }

    #[test]
    fn a_class_that_stopped_waiting_does_not_qualify_on_its_return() {
        let mut point = AdmissionPoint::over(all_five());
        // The upload class is waiting at four of the five decisions and absent
        // at one, so it has not had work waiting *throughout*.
        for at in 0..5 {
            let waiting: Vec<&'static str> = if at == 2 {
                vec![AUTO]
            } else {
                vec![AUTO, UPLOAD]
            };
            assert_eq!(point.grant(&waiting), Some(AUTO));
        }
        assert_eq!(
            point.grant(&[AUTO, UPLOAD]),
            Some(AUTO),
            "the exception is about work waiting throughout, and it was not"
        );
    }

    #[test]
    fn nothing_waiting_grants_nothing() {
        let mut point = AdmissionPoint::over(all_five());
        assert_eq!(point.grant(&[]), None);
        assert_eq!(point.grants_made(), 0);
    }

    #[test]
    fn a_class_nobody_declared_is_not_admitted() {
        let mut point = AdmissionPoint::over(all_five());
        assert_eq!(point.grant(&["something-nobody-declared"]), None);
    }

    #[test]
    fn at_the_repair_one_servers_auto_download_cannot_hold_anothers_upload_for_ever() {
        // O8's own worked case: two servers, one with an auto-download
        // arrangement that keeps trying, the other with nothing but an upload.
        let mut repair = TheRepairMoment::over(all_five(), 2);
        let mut upload_grants = 0;
        for _ in 0..repair.above_them_all.the_bound() {
            if repair.above_them_all.grant(&[AUTO, UPLOAD]) == Some(UPLOAD) {
                upload_grants += 1;
            }
        }
        assert!(
            upload_grants >= 1,
            "T10's forbidden shape one layer above where T10 forbids it"
        );
        // Each server's own point holds the same order and counts its own
        // grants.
        assert_eq!(repair.at_each_server.len(), 2);
        assert_eq!(repair.at_each_server[0].n(), repair.above_them_all.n());
        assert_eq!(repair.at_each_server[0].grants_made(), 0);
    }

    #[test]
    fn the_class_this_document_ranks_is_the_class_it_declares() {
        // T8 has the arranging module declare the class and Row 189 fix the
        // order among them. **One spelling in one place**, so the declaration
        // and the rank can never come apart.
        assert!(THE_DECLARED_ORDER
            .iter()
            .any(|class| class.name == crate::declarations::THE_TRANSFER_PRIORITY_CLASS));
        assert_eq!(
            crate::request::the_priority_class(),
            crate::declarations::THE_TRANSFER_PRIORITY_CLASS
        );
    }

    #[test]
    fn the_picture_view_is_third_and_n_counts_it() {
        let order = the_declared_order_with_the_picture_view();
        let names: Vec<&str> = order.iter().map(|class| class.name).collect();
        assert_eq!(
            names,
            vec![AUTO, MANUAL, crate::declarations::THE_PICTURE_VIEW_CLASS, DEPENDENCY, UPLOAD, PREVIEW],
            "L29: below manual download, above dependency-upload"
        );
        assert_eq!(order[2].rank, 3);
        assert_eq!(n_over(&order), 6);
        assert_eq!(the_bound(n_over(&order)), 11);
        // O4's own five are untouched.
        assert_eq!(THE_DECLARED_ORDER.len(), 5);
    }

    #[test]
    fn within_a_request_admission_follows_the_enumerations_order() {
        assert!(within_a_request_admission_follows_the_enumeration(&[
            0, 1, 2, 3
        ]));
        assert!(!within_a_request_admission_follows_the_enumeration(&[
            0, 2, 1
        ]));
    }
}

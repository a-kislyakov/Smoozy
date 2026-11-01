//! The two rate budgets and the one desktop-side session-admission point —
//! `Desktop app and delivery` §11, R1 to R19.
//!
//! **What is here.** One upload budget and one download budget, in bytes per
//! second, in process memory; the floor each one divides by; the one point that
//! knows every server at once and admits what moves bytes across the line; and
//! the achieved-rate reading taken on the same trailing window the enforcement
//! decisions are taken against.
//!
//! **What `File Browser` already holds and this does not rewrite.** O5's share
//! rule and the declared order it runs over are that document's, and its O8
//! says the rule is *"the same type used twice — one point per server and one
//! above them all, each counting its own grants"* — so this point **holds** one
//! of that crate's [`AdmissionPoint`]s and grants through it. What §11 adds
//! beside it is the floor, the admissible count, this point's scope, and the
//! turn-taking where that count is one.
//!
//! **What is deliberately not here.** The enforcement itself, which is
//! per-channel in Smoozy's own byte loop and is
//! [`smoozy_server_protocol::transfers::moving`]'s — this module feeds that
//! loop's [`DrawsFromTheSharedBudget`] seam and holds no second loop. And the
//! callers: nothing in the running application arranges an upload yet, so what
//! is built is the budget object and the admission point with their real
//! behaviour, and who draws on them arrives with the screens.
//!
//! **`Desktop app and delivery` R15b was in that second list until 2026-09-09
//! and is now built** — [`TheRemainingTime`], with `Desktop app and delivery`
//! R15b's three no-estimate states. **What changed is the caller and not the
//! arithmetic**: the clause's own site said the estimate *"belongs beside §16's
//! presentation plumbing with the caller that names the subset"*, and step 10a
//! piece E is the first caller that names one. **The naming is the caller's and
//! the window is this module's** — [`TheChannelsDraw::the_channel`] is the
//! handle a caller collects, and nothing here knows what a request is.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit** (the same rule [`crate::transfers`] states). A marker with no
//! document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! header list. **Where a marker is a clause of this crate's own plan *and* of
//! the document meant, the header list is not enough and the document is
//! written out at the site**, because a bare one there does not fail to
//! resolve: it resolves to a real and wrong clause, which nobody reading it can
//! tell from a right one.
//!
//! **In this file that applies to nearly every marker, so every marker below is
//! written with its document, this crate's own included** — or, where a
//! sentence has just named one, with *that document's* in place of repeating
//! it. `Server protocol and transfers` defines an R-series of its own, from
//! `Server protocol and transfers` R1 to `Server protocol and transfers` R17,
//! and a T-series and a C-series beside it; this crate's plan defines R1 to
//! R19, a T-series and a C-series too. **The pair that collides hardest is the
//! R-series, which
//! is the whole subject of this file on both sides**: a bare `R6` here would
//! resolve to a real and wrong clause, which nobody reading it could tell from
//! a right one.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` R6a, R6b, R12a, T5a, T11a.
//! `File Browser` O1, O4, O5, O6, O8. `Rendering` §7's submission class.
//! `State, identity and recovery` D5. `The shape` section 7.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use smoozy_file_browser::pool::{with_a_class_a_later_document_declares, AdmissionPoint};
use smoozy_server_protocol::transfers::moving::DrawsFromTheSharedBudget;
use smoozy_server_protocol::transfers::Direction;
use smoozy_state::observed::Clock;

/// **One declared transfer-priority class, and it is `File Browser`'s type
/// rather than a second one of the same shape.**
///
/// `File Browser` O8 says the share rule is *"the same type used twice — one
/// point per server and one above them all, each counting its own grants"*, so
/// the type is that document's and is re-exported here for callers. A second
/// declaration of the same fact in this crate would be the two homes the
/// Conventions' *Every fact has one home* exists to prevent.
pub use smoozy_file_browser::pool::DeclaredClass;

// ---------------------------------------------------------------------------
// The unit, the cap and the floor.
// ---------------------------------------------------------------------------

/// **`Desktop app and delivery` R2: the unit is bytes per second**, fixed by
/// `Server protocol and transfers` R11 because every quantity the enforcement
/// point holds is already a byte count.
///
/// **Three units are in play across the evidence and the tools** — the standard
/// tool's flag in Kbit/s, Fact 25's figures in MB/s, Fact 75's in KB/s — so
/// **nothing here carries a rate without its unit**, which is what this type is
/// for. **The one conversion to whatever the control shows the user happens
/// once, at the settings edge, and nowhere else**; there is no conversion in
/// this module, and a rate reaches it and leaves it in this type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BytesPerSecond(pub u64);

/// What the user has set for one direction.
///
/// **`Desktop app and delivery` R8: where the user has set no cap the budget is
/// unlimited, the floor binds nothing, and admission is bounded only by the
/// concurrency numbers Rows 188 and 206 pick** — which are `File Browser` O1's
/// and `Download` Q1's, held at the per-server point, which this point never
/// overrides (`Desktop app and delivery` R12).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheCap {
    /// The ordinary state, and the one the application starts in.
    NoCapIsSet,
    Of(BytesPerSecond),
}

/// **The floor of `Desktop app and delivery` R7, and it is the one quantity
/// that document hands Build to pick.**
///
/// **`Desktop app and delivery` R14 names no number**: the pool sizes are Rows
/// 188's and 206's, the class order is Rows 189's and 207's, *"and the floor of
/// R7 is a quantity Build picks inside R6's constraint"*. That constraint is
/// that **every admitted channel's draw is bounded below by a non-zero share**,
/// so that waiting on the budget is a **bounded** wait and `Server protocol and
/// transfers` C19's inactivity deadline has something to sit against. Non-zero
/// is the whole of what is fixed; the number below is this build's, with its
/// reasoning, and it is **one constant in one place**.
///
/// **It is sixteen kibibytes a second.**
///
/// **Why not smaller.** `Server protocol and transfers` R6 puts the accounting
/// period at a tenth of a second, so a floor of this size is 1,638 bytes an
/// interval — a real write every interval rather than a dribble. A floor an
/// order of magnitude below that would make an admitted channel's guaranteed
/// share smaller than a single protocol write's worth of progress per second,
/// which is a bounded wait in arithmetic and an unbounded one to look at.
///
/// **Why not larger.** The floor is what refuses a channel, so it must not bite
/// at the rates the evidence actually names. Against Fact 75's 400 KB/s cap it
/// admits twenty-five channels; against the measured real uplink of 0.62 MB/s,
/// about thirty-eight; against the measured 3.88 MB/s down, over two hundred.
/// All of those are far above four channels a server (`File Browser` O1) times
/// any plausible number of servers, so **at an ordinary cap the floor refuses
/// nothing** and admission is bounded by the pools, which is
/// `Desktop app and delivery` R8's picture arriving with a cap set as well as
/// without one. Where it does bite is below sixty-four kibibytes a second,
/// where four channels already carry more than the line can use and refusing
/// the fifth is the right answer rather than a cost.
///
/// **`Desktop app and delivery` R14 fixes one floor per direction** and each
/// direction's budget holds its own, initialised from this one constant: the
/// number is written once, and a direction-specific number — if one is ever
/// measured, and none is — has a home without a second constant appearing.
pub const THE_FLOOR_PER_DIRECTION: BytesPerSecond = BytesPerSecond(16 * 1024);

/// **The trailing window, and it is one constant in one place.**
///
/// **`Desktop app and delivery` R15a takes `Server protocol and transfers` R12a
/// whole**: *"The trailing three seconds, the elapsed-time divisor until that
/// window has filled, and the two publication moments are taken whole; this
/// document holds no second window and smooths nothing on top of one."*
///
/// **The same window carries both jobs, and that is `Server protocol and
/// transfers` R6b rather than a saving.** That clause is a correction: *"the
/// achieved rate every enforcement decision is taken against is measured over a
/// bounded trailing window and never over the transfer's life"*, and it names
/// R12a's trailing three seconds as exactly that shape. So the window below is
/// what [`TheDesktopSideAdmissionPoint`] hands allowances against **and** what
/// the reading is averaged over — one series of counts, no second window, no
/// average of an earlier average.
///
/// **The length has now been watched, and it stays at three seconds. Measured
/// 2026-09-09 at step 10a piece E, which closes
/// `Preparation/Architecture/What Build inherits.md` item 8(b) and
/// `Server protocol and transfers` §12 item 14.**
///
/// **This clause used to say no reading had ever been drawn at any window
/// length on any machine, and that is no longer true.** Two real transfers were
/// run — Smoozy's own byte loop, over a real SSH connection on the loopback
/// address, carrying OpenSSH's own `sftp-server` — and their count series were
/// replayed through this window at 250 ms, 500 ms, 1 s, 2 s, 3 s and 5 s.
///
/// **Three transfers, and one of them in each direction, which is what
/// `Server protocol and transfers` §12 item 14 asked for by name.** An upload
/// of 256 MiB at 51.9 MB/s (5.167 s, 1,029 counts); a download of 256 MiB at
/// 48.0 MB/s (5.598 s, 1,029 counts); and an upload held to a real line's
/// measured 620 KB/s (`Server protocol and transfers` R17's own figure).
///
/// **What the figures said**, as the reading's standard deviation across the
/// steady stretch, with the widest spread between any two readings beside it:
///
/// | window | upload | download | at a line's rate |
/// |---|---|---|---|
/// | 250 ms | 2.4 %, 1.106× | 1.3 %, 1.044× | 0.1 %, 1.003× |
/// | 1 s | 1.7 %, 1.068× | 0.8 %, 1.027× | 0.0 %, 1.001× |
/// | 2 s | 1.0 %, 1.033× | 0.5 %, 1.019× | 0.0 %, 1.000× |
/// | **3 s** | **0.7 %, 1.020×** | **0.2 %, 1.007×** | **0.0 %, 1.000×** |
/// | 5 s | one reading only | 0.1 %, 1.002× | 0.0 %, 1.000× |
///
/// **So steadiness does not decide it: the figure is readable at every length
/// tried, in both directions, at both a loopback's rate and a real line's.** What decides it is `Server protocol and transfers` R12a's own lower
/// bound, which the same run turned from an argument into an observation. That
/// clause reasons that a window *"holds many tens of counts and no single write
/// can move the figure"*, and says outright that this is *"an argument from
/// R6's own sample count and not an observation"*. **At 51.9 MB/s the window
/// held about six hundred counts.** At a slow single channel the floor is the
/// byte loop's own accounting period — a tenth of a second — so **a one-second
/// window holds about ten counts and a single write moves the figure by a
/// tenth, where three seconds bounds it at a thirtieth.** That is the case the
/// argument was made for, and shortening the window is exactly what would break
/// it.
///
/// **What three seconds costs is stated by the clause itself and is not
/// increased by this**: `Server protocol and transfers` R10's mixed reading
/// lasts as long as the window, so the figure beside a just-changed cap is
/// wrong for up to three seconds. **The constant is not moved, and this is the
/// recorded outcome rather than an untouched one** — the entry's own words are
/// that either moving it or recording why it stays closes the item.
pub const THE_TRAILING_WINDOW: Duration = Duration::from_secs(3);

// ---------------------------------------------------------------------------
// The declared classes, and the one this document declares.
// ---------------------------------------------------------------------------

/// **`Desktop app and delivery` R19's class: the update fetch's.**
///
/// The update fetch *"is a session on no connection and reaches no server"*
/// (`Desktop app and delivery` R11), but it is bytes over the user's own line
/// that Smoozy causes, so it draws from the download budget and is admitted
/// here like any other draw.
///
/// **It is a class of the one order, not a desktop-point-only class**, and that
/// is load-bearing rather than tidy: `File Browser` O5 defines N as the count of
/// declared classes and O8 applies the same share rule at this point *"with the
/// same N and the same rank tie-breaker"*, so a class declared at this point
/// alone would make the two points' N differ and falsify O8 in its own words.
/// **At a per-server point it simply never has work waiting.**
///
/// **Its one caller is the update check** (`crate::updating::checking`, from
/// step 11's check piece), which asks for this class alone at the desktop point
/// and draws every byte of the update file through the channel it is granted.
pub const THE_UPDATE_FETCHS_CLASS: &str = "update-fetch";

/// The name `File Browser` O4 gives the class the update fetch ranks below.
///
/// Written here because `Desktop app and delivery` R19 states its rank as a
/// **relation** to this class and not as an ordinal — *"because an ordinal ages
/// out the moment a later document declares against O4"*. It is that document's
/// name for its own class, taken as declared; no number of that document is
/// copied here, and the name itself is read from where that document declares
/// it rather than spelled again.
const THE_CLASS_THE_UPDATE_FETCH_RANKS_BELOW: &str =
    smoozy_file_browser::declarations::THE_TRANSFER_PRIORITY_CLASS;

/// The name `Queue & History` P6 gives the class the update fetch ranks above,
/// and which that clause fixes as the **lowest** — *"lowest is what this clause
/// actually fixes and it holds at any count"* — so nothing may be declared
/// beneath it. **That document is built and P6's class is
/// `smoozy_queue_history::previews::THE_PREVIEW_FETCHS_CLASS`, but it fixes the
/// class as a shape and publishes no name string**, so there is nothing to read
/// the name from. `File Browser` O4 carries the name in its own list *"taken as
/// declared, adding nothing"*; this spells it for the same reason and the same
/// way. *(Corrected 2026-09-07: this said the document was not built, which
/// stopped being true at step 7 — this crate has depended on it since — and the
/// reason for spelling the name is the missing constant rather than the missing
/// crate.)*
const THE_CLASS_THE_UPDATE_FETCH_RANKS_ABOVE: &str = "preview-fetch";

/// Why the update fetch could not be placed against the declarations handed in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheUpdateFetchHasNowhereToGo(pub String);

/// **`Desktop app and delivery` R19: the update fetch's class, declared here and
/// ranked against `File Browser` O4's list — below its `upload` class and above
/// the preview fetch.**
///
/// **Below `upload`:** the update fetch is not user-asked in the ordinary case,
/// while O4 ranks `upload` as *"user-asked bulk"*, and its artifact draw is a
/// whole build that blocks nothing the user is doing. **Above the preview
/// fetch:** `Queue & History` P6 fixes that class as the lowest, and the
/// substantive ground runs the same way, the update fetch being the one
/// non-user-asked class a user can be standing in front of.
///
/// **The relation is applied to whatever list the caller declares**, and the
/// ranks are re-read from the result rather than assumed, so a later document
/// declaring another class against O4 moves nothing here.
pub fn the_declared_order_with_the_update_fetch(
    declared: &[DeclaredClass],
) -> Result<Vec<DeclaredClass>, TheUpdateFetchHasNowhereToGo> {
    if declared
        .iter()
        .any(|class| class.name == THE_UPDATE_FETCHS_CLASS)
    {
        return Err(TheUpdateFetchHasNowhereToGo(
            "the update fetch is already in the declarations handed in; \
             `Desktop app and delivery` R19 declares it once"
                .to_string(),
        ));
    }
    let mut ordered: Vec<DeclaredClass> = declared.to_vec();
    ordered.sort_by_key(|class| class.rank);
    let below = ordered
        .iter()
        .position(|class| class.name == THE_CLASS_THE_UPDATE_FETCH_RANKS_BELOW)
        .ok_or_else(|| {
            TheUpdateFetchHasNowhereToGo(format!(
                "`Desktop app and delivery` R19 ranks the update fetch below \
                 `File Browser` O4's `{THE_CLASS_THE_UPDATE_FETCH_RANKS_BELOW}` \
                 class, and no class of that name was declared"
            ))
        })?;
    if let Some(above) = ordered
        .iter()
        .position(|class| class.name == THE_CLASS_THE_UPDATE_FETCH_RANKS_ABOVE)
    {
        if above <= below {
            return Err(TheUpdateFetchHasNowhereToGo(format!(
                "`Desktop app and delivery` R19 ranks the update fetch below \
                 `{THE_CLASS_THE_UPDATE_FETCH_RANKS_BELOW}` and above \
                 `{THE_CLASS_THE_UPDATE_FETCH_RANKS_ABOVE}`, and the \
                 declarations handed in put those two the other way round"
            )));
        }
    }
    // **The placing is `File Browser`'s own route and not a second one.** O4's
    // closing sentence hands the rank to the declaring document, and
    // [`with_a_class_a_later_document_declares`] is what that document provides
    // for taking one — so no edit to O4's list is owed and none is made. The
    // relation is turned into a rank by taking the one it ranks below, and the
    // ranks are re-read from the result afterwards.
    let mut ordered = with_a_class_a_later_document_declares(
        &ordered,
        DeclaredClass {
            name: THE_UPDATE_FETCHS_CLASS,
            declared_by: "Desktop app and delivery",
            rank: ordered[below].rank,
        },
    );
    for (at, class) in ordered.iter_mut().enumerate() {
        class.rank = (at + 1) as u8;
    }
    // The relation is checked on the result rather than assumed from the way it
    // was placed: what R19 declares is *below `upload` and above the preview
    // fetch*, and that is what has to be true of the order handed back.
    let placed = ordered
        .iter()
        .position(|class| class.name == THE_UPDATE_FETCHS_CLASS)
        .expect("it was just placed");
    if placed != below + 1 {
        return Err(TheUpdateFetchHasNowhereToGo(format!(
            "the update fetch was placed at {placed} rather than immediately              below `{THE_CLASS_THE_UPDATE_FETCH_RANKS_BELOW}`, which              `Desktop app and delivery` R19's relation requires"
        )));
    }
    Ok(ordered)
}

// ---------------------------------------------------------------------------
// The reading.
// ---------------------------------------------------------------------------

/// **The achieved-rate reading — `Desktop app and delivery` R15 and R15a.**
///
/// *"Smoozy's own count of bytes moved over a window, taken at the same
/// accounting point as the enforcement, per channel and summed. It is never
/// read out of a tool's output and never derived from a file's size and a
/// start time."* It is **period-held** and nothing durable is written from it.
///
/// **One reading per direction, not one for the application**
/// (`Desktop app and delivery` R15a): a channel belongs to one direction by
/// construction and draws from that direction's budget, so the per-channel
/// counts sum against the budget they were drawn from, which is what puts a
/// reading beside each of the two caps.
///
/// **A published reading is held, never aged** (`Desktop app and delivery`
/// R15a, against that document's P10b): whatever is on screen stands unchanged
/// until the next reading arrives. Ageing it would want a timer
/// `Desktop app and delivery` P10b forbids, and this type carries no route to
/// age one.
///
/// **`Desktop app and delivery` R16's two limits ride on this reading and are
/// carried rather than glossed.** The shared-budget mechanism was **exercised
/// on uploads only**, and the download case differs materially — *"a receiver
/// enforces a cap by not reading, so buffering can let the wire rate
/// overshoot"* — so **a reading in the down direction is a count of bytes
/// Smoozy accepted rather than of bytes the line carried**. And a budget
/// changed while channels are moving has been run on the upload side alone;
/// the download half stands unrun **permanently**, running it wanting a rented
/// machine the Creator ruled against on 2026-08-31. Row 306's downward *yes*
/// follows from the mechanism rather than from an observation, and nothing
/// here upgrades it.
///
/// **Why this is not `smoozy_state::observed::Reading`**, which is this
/// project's established type for a reading with its moment and its clock.
/// That type's moment is a `SystemTime`. `Server protocol and transfers` R12a
/// requires **both ends of the window on the one clock the accounting point
/// runs on**, which is the monotonic clock this type's moment is taken from,
/// so hanging a wall-clock moment on the same reading would put the second
/// clock `State, identity and recovery` D5 forbids subtracting across right
/// beside the window's own arithmetic. The clock is still answered, in D5's
/// own words, by [`TheAchievedRate::the_clock_it_was_taken_on`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheAchievedRate {
    value: BytesPerSecond,
    taken_at: Instant,
    direction: Direction,
}

impl TheAchievedRate {
    pub fn value(&self) -> BytesPerSecond {
        self.value
    }

    /// **`Server protocol and transfers` R13: it carries the moment it was
    /// taken.** The moment is on the accounting point's own monotonic clock,
    /// which is both ends of `Server protocol and transfers` R12a's window.
    pub fn taken_at(&self) -> Instant {
        self.taken_at
    }

    /// Which direction's reading this is — `Desktop app and delivery` R15a's
    /// *one reading per direction*.
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// **`Server protocol and transfers` R13 and `State, identity and recovery`
    /// D5: the clock it was taken on.** The user's own computer's, which is
    /// where the accounting point runs.
    pub fn the_clock_it_was_taken_on(&self) -> Clock {
        Clock::ThisComputer
    }
}

/// **`Desktop app and delivery` R15b's remaining-time estimate, built
/// 2026-09-09 at step 10a piece E, which is the first caller that names a
/// subset.**
///
/// **This clause used to be unbuilt and said so at this site**, on the reasoning
/// that *"nothing groups channels into a request, because nothing in the
/// application arranges one"*. **That reasoning was right and it has run out**:
/// §16.5's underway shape draws a request's summary line, so a caller now names
/// the subset and hands over the whole. The clause's own words are that the
/// estimate *"belongs beside §16's presentation plumbing with the caller that
/// names the subset"* — so the arithmetic is here, at the accounting point that
/// owns the window, and the naming is the caller's.
///
/// **The rule.** Bytes remaining, divided by the request's own achieved rate.
/// Bytes remaining is the request's byte whole minus bytes staged summed over
/// its units — the whole from `Download` M16 downwards and `File Browser` U11
/// upwards, both of which exist **before the first transfer starts**, and bytes
/// staged from `Server protocol and transfers` G4. **Neither operand is
/// composed here**: both arrive as one figure the caller has already summed,
/// which is what keeps `Download` N7's dispositions — skipped, refused,
/// left-alone and cannot-land — out of both terms without this module knowing
/// what a disposition is.
///
/// **The rate is `Server protocol and transfers` R12a's, over R12a's window,
/// summed over this request's channels and no others.** Same accounting point,
/// same trailing three seconds, same elapsed-time divisor before the window has
/// filled; **the only difference from `Desktop app and delivery` R15's reading
/// is the set summed over**. A per-direction rate must never be used here: it
/// rises when another request's channels end and falls when one is admitted, and
/// neither event is about this request.
///
/// **It is never built the naive way.** `Server protocol and transfers` R12
/// forbids the rate being *"derived from a file's size and a start time"*, which
/// forbids the average-since-the-request-began route with it. **The estimate is
/// recomputed from the trailing window at every accounting interval** and
/// carries no memory of an earlier estimate — no decay, no smoothing, no floor
/// on how fast it may change. **There is no field on this type an earlier
/// estimate could be kept in.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheRemainingTime {
    /// **The first of `Desktop app and delivery` R15b's three states carrying
    /// no estimate: no channel of this request is moving bytes**, every unit
    /// waiting on admission or on the budget — so the divisor is zero and there
    /// is nothing to divide.
    ///
    /// **`Server protocol and transfers` G4b's resume lands here and needs no
    /// fourth state.** A channel verifying a staged prefix moves no bytes and
    /// *"draws no rate budget"*, so its rate over this window is zero and the
    /// estimate is absent — which is that clause's *"no completion estimate
    /// treats a resume as starting at the checkpoint"* delivered by the
    /// arithmetic rather than by a rule added beside it.
    NoChannelOfThisRequestIsMovingBytes,
    /// **The second: the whole is not yet known**, which for a manual request
    /// is `Download` M17's still-running enumeration.
    TheWholeIsNotYetKnown,
    /// **The third: the request has ended**, where a remaining time is not a
    /// reading at all.
    TheRequestHasEnded,
    /// The estimate, in whole seconds, **with the moment it was taken on the
    /// accounting point's own clock**. **Held rather than aged**
    /// (`Desktop app and delivery` R13 and P10b): the drawing never runs it
    /// down between publications — *"a rate is not a moment, and neither is a
    /// quotient of one"*.
    Of { seconds: u64, taken_at: Instant },
}

impl TheRemainingTime {
    /// The figure, where there is one. **Absent is a value and not a gap**, and
    /// in each absent case the absence is stated rather than a number shown.
    pub fn seconds(self) -> Option<u64> {
        match self {
            TheRemainingTime::Of { seconds, .. } => Some(seconds),
            _ => None,
        }
    }

    /// **`Desktop app and delivery` R13 and `State, identity and recovery` D5:
    /// the clock it was taken on.** The user's own computer's, which is where
    /// the accounting point runs.
    pub fn the_clock_it_was_taken_on(&self) -> Clock {
        Clock::ThisComputer
    }
}

/// **What the caller knows about the request's whole.**
///
/// **Two states and not one nullable number**, because
/// `Desktop app and delivery` R15b's second no-estimate state is *the whole is
/// not yet known* and a zero whole is a different claim from an unknown one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BytesRemaining {
    /// `Download` M17's still-running enumeration.
    TheWholeIsNotYetKnown,
    /// The whole minus what is staged, summed over the request's units by the
    /// caller.
    Of(u64),
}

/// **The seam `Desktop app and delivery` R18 records, carried at the site.**
///
/// `Desktop app and delivery` R18 reads `Server protocol and transfers` T5's
/// *"Every session, and not only the transfer channels"* as binding the
/// **per-server** point, which counts a transfer channel, a command invocation
/// and the one long-lived reading channel against one `MaxSessions` count.
/// **The desktop point above it takes transfer channels alone**, and the
/// update fetch beside them, because what it exists to hold down is line
/// contention, which is made by bytes rather than by sessions — and gating a
/// command behind desktop-wide transfer load would put correlation on every
/// server at once behind exactly the load `Server protocol and transfers` T5's
/// reserved command slot exists to keep it clear of.
///
/// **That narrows `Server protocol and transfers` T5 and `The shape` section
/// 7, neither of which is cited as agreeing.** `Server protocol and transfers`
/// T5a states the same split from its own side, so that half is closed; **the
/// shape's half is still owed**. `Desktop app and delivery` R18 records it as
/// a seam (`Desktop app and delivery` §22 item 31) and repairs neither
/// document, and neither does this code: what stands is
/// `Desktop app and delivery` R12, which leaves the per-server point
/// untouched, and that document's R13, since either point may refuse.
pub const THE_SEAM_R18_RECORDS: &str =
    "the desktop point takes transfer channels and the update fetch alone, \
     which narrows `Server protocol and transfers` T5 and `The shape` section \
     7; T5a agrees from its own side and the shape's wording is still owed";

/// **`Server protocol and transfers` T11a, carried at the desktop-side point it
/// is about: the two admission points are different instruments and neither
/// stands in for the other.**
///
/// **The two.** This point and the per-server point above it are
/// `Server protocol and transfers` T5's, and they admit **sessions** against
/// `MaxSessions` and **bytes** against the line. `Rented-server runtime` A4's is
/// one point per **machine**, and it admits **worker starts** against the cards,
/// the processors and the machine's memory. **A transfer after hand-over passes
/// both** — this one for its channel, that one for its worker.
///
/// **What the clause forbids, and it is a rule about this file.** Neither
/// figure is derived from the other. Nothing here reads a count of admitted
/// workers, a resource class or a machine's own arbitration, and nothing on the
/// machine reads this point's admissible count, its floor or its cap: the two
/// numbers are settled from different things and a builder that composed one
/// from the other would have a desktop refusing a channel because a machine was
/// rendering, or a machine starting a worker because the line was quiet.
///
/// **Why it is written here and not only next door.** The clause names both
/// points and the risk is at whichever one a builder is standing at. The machine
/// side carries it at
/// `smoozy_runtime::arbitration::THE_TWO_ADMISSION_POINTS_ARE_DIFFERENT_INSTRUMENTS`,
/// in the same words from the other end.
pub const THE_TWO_ADMISSION_POINTS_ARE_DIFFERENT_INSTRUMENTS: &str =
    "this point and the per-server point above it admit sessions and bytes; \
     `Rented-server runtime` A4's admits worker starts against the cards, the processors and \
     the machine's memory. A transfer after hand-over passes both and neither figure is \
     derived from the other";

// ---------------------------------------------------------------------------
// One direction's budget.
// ---------------------------------------------------------------------------

/// One count the accounting point took: bytes, and the moment on the one clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ACount {
    at: Instant,
    bytes: u64,
}

/// One admitted channel, and what it has moved inside the window.
#[derive(Debug, Clone)]
struct AnAdmittedChannel {
    class: &'static str,
    counts: VecDeque<ACount>,
    /// The first moment this channel was ever counted, for
    /// `Server protocol and transfers` R12a's elapsed-time divisor **applied to
    /// a named subset** — `Desktop app and delivery` R15b's *"same elapsed-time
    /// divisor before the window has filled"*. The direction's own
    /// [`TheDirectionsBudget::first_count_ever`] is the same rule one level up,
    /// and neither is derived from the other.
    first_count: Option<Instant>,
}

/// **`Desktop app and delivery` R1: one budget for a direction, and never one
/// budget for both.**
///
/// Two, *"because the promise is the **total** across every transfer running at
/// that moment in each direction and the two directions are measured against
/// different lines; 11 to 12 times separates them in the one measurement this
/// project holds"*.
///
/// **`Desktop app and delivery` R3: one object in process memory for the life of
/// the application run, and it is not state.** Nothing durable is written from
/// it, and there is no route here to write one; the setting that sets it is
/// durable and user-held, and lives in `State, identity and recovery` §11.1's
/// user-held class.
///
/// **`Desktop app and delivery` R4: this holds the budget and does not enforce
/// it, does not divide it into per-transfer caps, and accepts none from
/// anywhere.** There is no per-channel cap on this type or on
/// [`TheChannelsDraw`], and no way to set one — Fact 75 measured what a static
/// division costs: a two-way 400+400 split against an 800 KB/s budget achieved
/// **421.9 KB/s** while one shared budget on the same work achieved **820** and
/// cut 21.85 seconds to 11.24.
#[derive(Debug)]
struct TheDirectionsBudget {
    direction: Direction,
    cap: TheCap,
    /// **`Desktop app and delivery` R14: one floor per direction**, from
    /// [`THE_FLOOR_PER_DIRECTION`].
    floor: BytesPerSecond,
    channels: Vec<(u64, AnAdmittedChannel)>,
    /// **Channels that have ended, held only while their counts are in the
    /// window** — `Desktop app and delivery` R5's *"the bytes it moved stay in
    /// the window"*. They are owed no floor share and take no place in the
    /// admissible count; they are here because their bytes crossed the line
    /// inside the last three seconds, which is all
    /// `Server protocol and transfers` R12a's reading and the cap's slack
    /// count. **Until 2026-09-29 an ended channel's counts went with it**, so on
    /// a folder, moved one file at a time, the reading saw only the file in
    /// flight and every new file met an empty window and the whole cap.
    ended: Vec<(u64, AnAdmittedChannel)>,
    next_channel: u64,
    /// The first moment this direction's accounting point ever counted
    /// anything, for `Server protocol and transfers` R12a's elapsed-time divisor.
    first_count_ever: Option<Instant>,
}

impl TheDirectionsBudget {
    fn new(direction: Direction) -> TheDirectionsBudget {
        TheDirectionsBudget {
            direction,
            cap: TheCap::NoCapIsSet,
            floor: THE_FLOOR_PER_DIRECTION,
            channels: Vec::new(),
            ended: Vec::new(),
            next_channel: 0,
            first_count_ever: None,
        }
    }

    /// Drops every count older than the window. **This is the whole of how the
    /// average stays bounded** — `Server protocol and transfers` R6b, whose
    /// forbidden shape is a governor that keeps them all.
    ///
    /// **An ended channel leaves the budget here and nowhere else**: once its
    /// last count has aged out it has nothing left in the window.
    fn drop_what_has_left_the_window(&mut self, now: Instant) {
        for (_, channel) in self.channels.iter_mut().chain(self.ended.iter_mut()) {
            while let Some(oldest) = channel.counts.front() {
                if now.saturating_duration_since(oldest.at) > THE_TRAILING_WINDOW {
                    channel.counts.pop_front();
                } else {
                    break;
                }
            }
        }
        self.ended.retain(|(_, channel)| !channel.counts.is_empty());
    }

    /// Every byte counted in the window, **the ended channels' included** —
    /// `Desktop app and delivery` R5.
    fn moved_in_the_window(&self) -> u64 {
        self.channels
            .iter()
            .chain(self.ended.iter())
            .map(|(_, channel)| Self::moved_by(channel))
            .sum()
    }

    fn moved_by(channel: &AnAdmittedChannel) -> u64 {
        channel.counts.iter().map(|count| count.bytes).sum()
    }

    fn admit(&mut self, class: &'static str) -> u64 {
        let id = self.next_channel;
        self.next_channel += 1;
        self.channels.push((
            id,
            AnAdmittedChannel {
                class,
                counts: VecDeque::new(),
                first_count: None,
            },
        ));
        id
    }

    /// **`Desktop app and delivery` R7's admissible count.** *"One channel is
    /// always admissible whatever the budget is, and it takes the whole budget;
    /// beyond the first, the count of admitted channels is bounded by the budget
    /// divided by the floor."*
    fn the_admissible_count(&self) -> TheAdmissibleCount {
        match self.cap {
            TheCap::NoCapIsSet => TheAdmissibleCount::TheBudgetBindsNothing,
            TheCap::Of(cap) => TheAdmissibleCount::Of((cap.0 / self.floor.0).max(1) as usize),
        }
    }

    /// **`Server protocol and transfers` R2, from the budget's side: how many
    /// bytes this channel may move now.**
    ///
    /// **The rate this decision is taken against is the bounded trailing
    /// window and never the transfer's life** —
    /// `Server protocol and transfers` R6b, which is a correction rather than
    /// an addition: a governor accounting over the whole run answered a raised
    /// cap with a burst *"to about 1 000 KB/s, roughly double the new cap, and
    /// held there for minutes"*.
    ///
    /// **The division is by the floor and by nothing else**
    /// (`Desktop app and delivery` R9): no priority class buys a larger share,
    /// priority governing admission order and not rate. So the window's slack
    /// is offered to this channel except for what is still owed to the other
    /// admitted channels' floor shares, and a channel that has not yet had its
    /// own floor share takes it out of the slack whatever the others are owed.
    /// **That is what makes `Desktop app and delivery` R7's floor true at the
    /// draw and not only at admission.**
    fn draw(&mut self, channel: u64, wanted: usize, now: Instant) -> usize {
        self.drop_what_has_left_the_window(now);
        let allowed = match self.cap {
            // **`Desktop app and delivery` R8: an empty field is no cap**, which
            // is the ordinary state — the channel moves what it asked for.
            TheCap::NoCapIsSet => wanted,
            TheCap::Of(cap) => {
                let window = THE_TRAILING_WINDOW.as_secs();
                let whole = cap.0.saturating_mul(window);
                let floor_share = self.floor.0.saturating_mul(window);
                let slack = whole.saturating_sub(self.moved_in_the_window());
                let mut owed_to_others = 0u64;
                let mut owed_to_me = 0u64;
                for (id, other) in self.channels.iter() {
                    let owed = floor_share.saturating_sub(Self::moved_by(other));
                    if *id == channel {
                        owed_to_me = owed;
                    } else {
                        owed_to_others = owed_to_others.saturating_add(owed);
                    }
                }
                let mine = slack
                    .saturating_sub(owed_to_others)
                    .max(slack.min(owed_to_me));
                mine.min(wanted as u64) as usize
            }
        };
        self.count(channel, allowed, now);
        allowed
    }

    /// The count the accounting point took. **The same series is what the
    /// enforcement decision above is taken against and what the reading below is
    /// averaged over** — `Desktop app and delivery` R15a's *"this document holds
    /// no second window and smooths nothing on top of one"*.
    fn count(&mut self, channel: u64, bytes: usize, now: Instant) {
        if bytes == 0 {
            return;
        }
        if let Some((_, admitted)) = self.channels.iter_mut().find(|(id, _)| *id == channel) {
            admitted.counts.push_back(ACount {
                at: now,
                bytes: bytes as u64,
            });
            admitted.first_count.get_or_insert(now);
            self.first_count_ever.get_or_insert(now);
        }
    }

    /// **`Server protocol and transfers` R7's first half, at the budget's side:
    /// what a channel drew and could not spend comes off the count.** The window
    /// holds bytes moved, so a draw the loop did not spend was never moved.
    fn give_back(&mut self, channel: u64, unused: usize) {
        if unused == 0 {
            return;
        }
        let Some((_, admitted)) = self.channels.iter_mut().find(|(id, _)| *id == channel) else {
            return;
        };
        let mut left = unused as u64;
        while left > 0 {
            let Some(last) = admitted.counts.back_mut() else {
                break;
            };
            if last.bytes > left {
                last.bytes -= left;
                break;
            }
            left -= last.bytes;
            admitted.counts.pop_back();
        }
    }

    /// **`Desktop app and delivery` R5: a channel that finished returns its
    /// unused share at once**, which is the half Fact 75 measured as forfeited.
    /// *"That is the budget holder's act rather than the enforcer's, and without
    /// it the budget shrinks over the life of a batch of uneven files."*
    ///
    /// The bytes it moved stay in the window, because a total cap is a total
    /// across the line and they crossed it; what is returned at once is its
    /// **share** — the floor the other channels' draws were holding back for it,
    /// and its place in the admissible count.
    ///
    /// **Built as it says since 2026-09-29.** Before, the channel and its
    /// counts went together, so the comment above was true of the share and
    /// false of the bytes. It moves to the budget's `ended` list with its
    /// counts, and [`Self::drop_what_has_left_the_window`] lets it
    /// go when the last of them is three seconds old.
    fn ended(&mut self, channel: u64) -> bool {
        if let Some(at) = self.channels.iter().position(|(id, _)| *id == channel) {
            let finished = self.channels.remove(at);
            if !finished.1.counts.is_empty() {
                self.ended.push(finished);
            }
        }
        self.channels.is_empty()
    }

    /// **`Server protocol and transfers` R12a: the bytes the accounting point
    /// counted in the last three seconds, divided by three** — *"nothing else
    /// enters the computation — no decay, no smoothing, no average of an earlier
    /// average"* — **and, before a full window has passed, the divisor is the
    /// elapsed time and never the window**, otherwise every transfer's first
    /// three seconds read low.
    fn the_reading(&mut self, now: Instant) -> TheAchievedRate {
        self.drop_what_has_left_the_window(now);
        let bytes = self.moved_in_the_window();
        let divisor = match self.first_count_ever {
            Some(first) => now
                .saturating_duration_since(first)
                .min(THE_TRAILING_WINDOW),
            None => Duration::ZERO,
        };
        TheAchievedRate {
            value: rate_over(bytes, divisor),
            taken_at: now,
            direction: self.direction,
        }
    }

    /// **The same window and the same divisor, summed over a named subset of
    /// this direction's channels and no others** —
    /// `Desktop app and delivery` R15b.
    ///
    /// *"The only difference from R15's reading is the set summed over."* There
    /// is no second series and no second window: the counts are the ones the
    /// enforcement decisions are taken against, filtered to the named channels,
    /// which is `Desktop app and delivery` R15a's *"this document holds no
    /// second window and smooths nothing on top of one"* applied one level in.
    ///
    /// **A channel the caller names that this point has never admitted
    /// contributes nothing**, which is the honest answer: it has moved no bytes
    /// through this accounting point.
    fn the_reading_over(&mut self, channels: &[u64], now: Instant) -> BytesPerSecond {
        self.drop_what_has_left_the_window(now);
        let mut bytes = 0u64;
        let mut first: Option<Instant> = None;
        // **A named channel that has ended still counts while its bytes are in
        // the window** (`Desktop app and delivery` R5), so a request moving a
        // folder one file at a time reads its own rate across its files.
        for (id, channel) in self.channels.iter().chain(self.ended.iter()) {
            if !channels.contains(id) {
                continue;
            }
            bytes += Self::moved_by(channel);
            if let Some(began) = channel.first_count {
                first = Some(match first {
                    Some(earlier) if earlier <= began => earlier,
                    _ => began,
                });
            }
        }
        let divisor = match first {
            Some(began) => now
                .saturating_duration_since(began)
                .min(THE_TRAILING_WINDOW),
            None => Duration::ZERO,
        };
        rate_over(bytes, divisor)
    }
}

/// Bytes over a duration, in the one unit.
///
/// **Zero where no time has passed**: a rate over no elapsed time is not a
/// figure, and `Server protocol and transfers` R12a's *"the reading is drawn
/// from the first moment there is one"* has the first moment be the first count,
/// which the next accounting interval follows a tenth of a second later.
fn rate_over(bytes: u64, elapsed: Duration) -> BytesPerSecond {
    if elapsed.is_zero() {
        return BytesPerSecond(0);
    }
    let per_second = (bytes as u128) * 1_000_000_000u128 / elapsed.as_nanos();
    BytesPerSecond(per_second.min(u64::MAX as u128) as u64)
}

/// What [`TheDesktopSideAdmissionPoint`] may admit beyond what is already
/// running — `Desktop app and delivery` R7 and R8.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheAdmissibleCount {
    /// **`Desktop app and delivery` R8: with no cap set the floor binds
    /// nothing at this point**, and admission is bounded by the concurrency
    /// numbers Rows 188 and 206 pick, which are held at the per-server point.
    /// **`Desktop app and delivery` R12: this point asks nothing of that count
    /// and never overrides it.**
    TheBudgetBindsNothing,
    /// **`Desktop app and delivery` R7: the budget divided by the floor, and
    /// never below one.**
    Of(usize),
}

// ---------------------------------------------------------------------------
// The one desktop-side admission point.
// ---------------------------------------------------------------------------

/// **The one desktop-side admission point, holding the two budgets beside it —
/// `Desktop app and delivery` R10, R7, R11 and R12.**
///
/// **`Desktop app and delivery` R10: every per-server admission point is
/// admitted through one desktop-side point, so admission knows about every
/// server at once.** The measurement forced it rather than suggesting it: a
/// whole second rented machine bought about **20%, not 100%**, *"the slowest
/// set the finish time"* with the line idle for 80 of 112 seconds, and
/// uncoordinated upload fan-out was actively worse than one machine at a time
/// — 1.54, 1.24, 0.99 MB/s across one, two and three boxes (Fact 25). **A pool
/// sized per server and admitted per server tunes the wrong thing.**
///
/// **`Desktop app and delivery` R7 is why the budgets are inside this type
/// rather than beside it**: *"with them apart, admission would let in channels
/// the budget cannot feed, and the floor would be a wish."* There is no route
/// to a budget that is not admitted through this point, which is also
/// `Desktop app and delivery` Z22's argument for the update fetch's own draw.
///
/// **`Desktop app and delivery` R11: what it admits is transfer channels
/// across every server, and the update fetch beside them. It does not gate the
/// command channel or the reading channel** — and there is nothing to pass
/// here for one: a caller names a **declared transfer class**, and a command
/// invocation and the long-lived reading channel are neither, so this point
/// cannot admit or refuse them. They are counted at the per-server point
/// alone, against one `MaxSessions` count (`Desktop app and delivery` R12),
/// which is the reading [`THE_SEAM_R18_RECORDS`] carries.
///
/// **`Desktop app and delivery` R12: a channel needs both points to admit it,
/// and either may refuse.** This point asks nothing of the per-server count
/// and never overrides it. Where the per-server point refuses after this one
/// has granted, the caller drops the [`TheChannelsDraw`] and the slot and the
/// floor share return at once (`Desktop app and delivery` R5) — which is what
/// makes dropping it the whole of that path rather than a step somebody has to
/// remember.
#[derive(Debug)]
pub struct TheDesktopSideAdmissionPoint {
    declared: Vec<DeclaredClass>,
    upload: Arc<Mutex<TheDirectionsBudget>>,
    download: Arc<Mutex<TheDirectionsBudget>>,
    /// **`File Browser` O5's share rule, held rather than rewritten.** O8 says
    /// the rule is *"the same type used twice — one point per server and one
    /// above them all"*, and *"the desktop-side point counts its own [grants],
    /// with the same N and the same rank tie-breaker"*, so the type is that
    /// document's and this is one of them. **A second implementation here would
    /// make O8's *"the classes mean the same thing globally that they mean at
    /// one machine"* an assertion instead of a construction.**
    ///
    /// **There is one per direction**, because the admissible count and the
    /// budget the decision turns on are per direction
    /// (`Desktop app and delivery` R1) and a class only ever has work waiting in
    /// its own direction — a download class absent from an upload decision's
    /// waiting list would lose O5's *waiting throughout* exception to grants
    /// that were never about it.
    share_up: AdmissionPoint,
    share_down: AdmissionPoint,
    /// What `Desktop app and delivery` R17's turn-taking reads, per direction.
    /// **It is not a second share rule**: R17 replaces the share with
    /// turn-taking where the admissible count is one, and this is the record
    /// that rotation needs.
    turns_up: TheTurns,
    turns_down: TheTurns,
    /// **Which classes have work waiting here, where the admissible count is
    /// one**, per direction — [`TheWaitingRegister`].
    waiting_up: TheWaitingRegister,
    waiting_down: TheWaitingRegister,
}

/// **How long a class stays marked as waiting after its last ask.**
///
/// **Not a figure of any plan's.** A caller that meets a refusal asks again —
/// the upload driver every tenth of a second, the submitting driver the same —
/// so a class still waiting renews its mark many times inside this span, and a
/// class that stopped asking (its request ended, its connection went) stops
/// holding a turn it will never take after at most this long. **Two seconds is
/// long beside a caller's own pause and short beside one file's transfer**,
/// which is all it has to be. It is read at a decision and never on a timer
/// (`Desktop app and delivery` P10b).
pub const A_WAITING_MARK_LAPSES_AFTER: Duration = Duration::from_secs(2);

/// **`Desktop app and delivery` R17's turn-taking needs to know every class
/// with work waiting, and at this point a caller names only its own.**
///
/// Step 10c piece 4 found it: an upload asks for the slot with its own class
/// alone, because it knows nothing of a submission queued behind it, so
/// turn-taking between the two held only where the submission happened to ask
/// between two of the upload's files. **The refusal a class meets marks it as
/// waiting here, and the mark lapses when the class stops asking**
/// ([`A_WAITING_MARK_LAPSES_AFTER`]). A decision at a count of one then takes
/// turns over the classes the caller named and the classes marked here
/// together. **It applies only where the admissible count is one**, because
/// that is where R17's guarantee is turn-taking; above it `File Browser` O5's
/// share rule keeps its own memory of what waited, over what callers name.
///
/// **It holds one entry per declared class at most**, so like [`TheTurns`] it
/// does not grow with the run.
#[derive(Debug, Default, Clone)]
struct TheWaitingRegister {
    /// Each class that met a refusal and has not been granted since, with the
    /// moment it last asked.
    marked: Vec<(&'static str, Instant)>,
}

impl TheWaitingRegister {
    /// **A refusal marks the class, or renews its mark.**
    fn is_waiting(&mut self, class: &'static str, at: Instant) {
        match self.marked.iter_mut().find(|(held, _)| *held == class) {
            Some(held) => held.1 = at,
            None => self.marked.push((class, at)),
        }
    }

    /// **A grant spent ends the class's wait.**
    fn was_granted(&mut self, class: &str) {
        self.marked.retain(|(held, _)| *held != class);
    }

    /// The classes still waiting at `now`, **a lapsed mark dropped as it is
    /// met.**
    fn still_waiting(&mut self, now: Instant) -> Vec<&'static str> {
        self.marked.retain(|(_, at)| {
            now.saturating_duration_since(*at) <= A_WAITING_MARK_LAPSES_AFTER
        });
        self.marked.iter().map(|(class, _)| *class).collect()
    }
}

/// **Per class, the number of the grant it last received, and a count of the
/// grants made in that direction.**
///
/// **This is the whole of what turn-taking reads**, and it is kept this way
/// rather than as the list of every class ever granted: a desktop session that
/// runs for weeks and moves tens of thousands of files would grow that list
/// without bound, for nothing. **The size here is the number of declared
/// classes**, and it gives exactly the answer the full list gives.
///
/// **A fixed window would not.** [`TheDesktopSideAdmissionPoint::whose_turn_it_is`]
/// picks the class whose last grant is furthest back, and **a class never
/// granted is furthest back of all**; with a window, two classes that both fell
/// outside it would both read as never granted and the tie would go to rank —
/// a different answer from the one R17 gives, which is exactly the run-time
/// difference `Server protocol and transfers` T8 exists to forbid.
#[derive(Debug, Default, Clone)]
struct TheTurns {
    /// The grant number each class last received, by class.
    last_granted_at: Vec<(&'static str, usize)>,
    grants_made: usize,
}

impl TheTurns {
    /// How far back this class's last grant is, counted the way the full list
    /// counted it. **A class that has never had one is furthest back of all.**
    fn how_far_back(&self, class: &str) -> usize {
        match self
            .last_granted_at
            .iter()
            .find(|(held, _)| *held == class)
        {
            Some((_, at)) => self.grants_made - at,
            None => usize::MAX,
        }
    }

    fn granted(&mut self, class: &'static str) {
        match self
            .last_granted_at
            .iter_mut()
            .find(|(held, _)| *held == class)
        {
            Some(held) => held.1 = self.grants_made,
            None => self.last_granted_at.push((class, self.grants_made)),
        }
        self.grants_made += 1;
    }
}

impl TheDesktopSideAdmissionPoint {
    /// One point over a declared order.
    ///
    /// **The declarations are the caller's**, per `File Browser` O4's closing
    /// sentence and `Server protocol and transfers` T8 — *"nothing decides a
    /// class at run time, because a class decided at run time is a class two
    /// callers can decide differently"* — and
    /// [`the_declared_order_with_the_update_fetch`] is how
    /// `Desktop app and delivery` R19's own class joins them.
    pub fn over(declared: Vec<DeclaredClass>) -> TheDesktopSideAdmissionPoint {
        let mut declared = declared;
        declared.sort_by_key(|class| class.rank);
        TheDesktopSideAdmissionPoint {
            share_up: AdmissionPoint::over(declared.clone()),
            share_down: AdmissionPoint::over(declared.clone()),
            declared,
            upload: Arc::new(Mutex::new(TheDirectionsBudget::new(Direction::Up))),
            download: Arc::new(Mutex::new(TheDirectionsBudget::new(Direction::Down))),
            turns_up: TheTurns::default(),
            turns_down: TheTurns::default(),
            waiting_up: TheWaitingRegister::default(),
            waiting_down: TheWaitingRegister::default(),
        }
    }

    fn register(&mut self, direction: Direction) -> &mut TheWaitingRegister {
        match direction {
            Direction::Up => &mut self.waiting_up,
            Direction::Down => &mut self.waiting_down,
        }
    }

    fn the_count_is_one(&self, direction: Direction) -> bool {
        self.the_admissible_count(direction) == TheAdmissibleCount::Of(1)
    }

    /// **[`TheWaitingRegister`]'s marks, after one decision at a count of
    /// one**: the class granted and spent stops waiting, and every class the
    /// caller named that did not get the slot is marked as waiting now.
    fn the_register_hears(
        &mut self,
        direction: Direction,
        named: &[&'static str],
        spent_by: Option<&'static str>,
        now: Instant,
    ) {
        let register = self.register(direction);
        if let Some(granted) = spent_by {
            register.was_granted(granted);
        }
        for class in named {
            if Some(*class) != spent_by {
                register.is_waiting(class, now);
            }
        }
    }

    /// **`File Browser` O5's N: the count of declared classes, wherever they are
    /// declared** — computed from the declarations and never read off a worked
    /// figure. **With `Rendering` §7's submission class and
    /// `Desktop app and delivery` R19's update fetch this is seven**, and with
    /// `File Browser` L29's picture view eight (from 2026-10-31).
    pub fn n(&self) -> usize {
        self.share_up.n()
    }

    /// **`File Browser` O5's bound: at most 2N−1 grants at this point** before a
    /// class holding work waiting throughout receives one. **O8 applies the same
    /// rule at this point with the same N and the same rank tie-breaker.**
    pub fn the_bound(&self) -> usize {
        self.share_up.the_bound()
    }

    fn budget(&self, direction: Direction) -> MutexGuard<'_, TheDirectionsBudget> {
        let held = match direction {
            Direction::Up => &self.upload,
            Direction::Down => &self.download,
        };
        held.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn share(&mut self, direction: Direction) -> &mut AdmissionPoint {
        match direction {
            Direction::Up => &mut self.share_up,
            Direction::Down => &mut self.share_down,
        }
    }

    fn turns(&self, direction: Direction) -> &TheTurns {
        match direction {
            Direction::Up => &self.turns_up,
            Direction::Down => &self.turns_down,
        }
    }

    /// **`Desktop app and delivery` R3: the setting that sets a budget is
    /// durable and user-held; the object is not.** The value arrives here in
    /// the one unit (`Desktop app and delivery` R2), already converted at the
    /// settings edge.
    ///
    /// **`Server protocol and transfers` R8: a cap set or changed while
    /// transfers are in flight takes effect on each running channel at that
    /// channel's next accounting interval.** No transfer is restarted,
    /// cancelled, re-queued or re-staged for it and no channel is closed: the
    /// budget is read at each interval, so there is no moment at which a
    /// running channel holds a rate it was given at the start.
    pub fn set_the_cap(&mut self, direction: Direction, cap: TheCap) {
        self.budget(direction).cap = cap;
    }

    pub fn the_cap(&self, direction: Direction) -> TheCap {
        self.budget(direction).cap
    }

    /// **`Desktop app and delivery` R14: one floor per direction.**
    pub fn the_floor(&self, direction: Direction) -> BytesPerSecond {
        self.budget(direction).floor
    }

    /// **`Desktop app and delivery` R7 and R8.**
    pub fn the_admissible_count(&self, direction: Direction) -> TheAdmissibleCount {
        self.budget(direction).the_admissible_count()
    }

    pub fn channels_admitted(&self, direction: Direction) -> usize {
        self.budget(direction).channels.len()
    }

    /// **The admission decision — `Desktop app and delivery` R7, R9, R13 and
    /// R17, over `File Browser` O5's share rule.**
    ///
    /// `waiting` is the classes with work waiting in that direction, as the
    /// callers declare them. The answer is the channel's draw, or **nothing**.
    ///
    /// **`Desktop app and delivery` R13: nothing is a reason to open fewer and
    /// never a reason to fail the transfer**, and that holds at both points —
    /// which is why this returns an absence rather than a failure, and why
    /// there is no error type here to be turned into one.
    ///
    /// **`Desktop app and delivery` R9: no priority class buys a larger share
    /// of the budget; priority governs admission order and not rate.** *"Two
    /// mechanisms shaping one number is how a total cap stops being a total."*
    ///
    /// **`Desktop app and delivery` R17: admission at this point never starves
    /// a class.** That document's R7's admissible count is the budget divided
    /// by the floor rather than a pool, so its R9's class order is a
    /// preference within each admission decision and never a gate, and every
    /// class is guaranteed a share. **Where the admissible count is one, the
    /// guarantee is turn-taking between the classes with work waiting** —
    /// without it an auto-download arrangement that *"keeps trying, and never
    /// switches itself off"* could hold the single admissible slot
    /// indefinitely, the user's own manual download would never be admitted,
    /// and `Desktop app and delivery` R13 makes a refusal not a failure, **so
    /// nothing would be reported and the presentation would show *underway*
    /// for ever**.
    pub fn admit(
        &mut self,
        direction: Direction,
        waiting: &[&'static str],
        now: Instant,
    ) -> Option<TheChannelsDraw> {
        // **A caller that can spend a grant made to any class it named**, so
        // the register's marks are heard but not merged in: a grant to a class
        // this caller never named would be one it could not spend. The
        // register is merged by [`Self::admit_to_carry`], whose callers name
        // only their own class.
        let the_count_is_one = self.the_count_is_one(direction);
        let granted = self.the_grant(direction, waiting);
        if the_count_is_one {
            self.the_register_hears(direction, waiting, granted, now);
        }
        Some(self.the_channel_is_admitted(direction, granted?, now))
    }

    /// **The same decision, for a caller that can spend a grant made to one
    /// class and no other** — `Download`'s one file, which carries its class.
    ///
    /// **A grant the caller does not spend leaves no mark** — step 10b round 1
    /// of *jobs, history and downloads*, must-do 1. The point still decides
    /// over every class the caller says is waiting, so R17's turn-taking and
    /// `File Browser` O5's share rule see the whole of what is waiting; but
    /// where the grant goes to another class, **the turn and the share rule's
    /// memory are put back as they stood before the decision**, and no channel
    /// is admitted. `Err(Some(class))` names the class the slot would have gone
    /// to; `Err(None)` is R13's refusal, where nothing was granted at all.
    ///
    /// **Before this, the grant was recorded as that class's turn the moment it
    /// was made**, and the caller handed it back. At an admissible count of one
    /// a caller offering the file of the class just granted then found the turn
    /// moved on to the other class, so every offer was refused, nothing moved,
    /// and R13 kept every refusal silent — Transfers showing *underway* for
    /// ever. With no cap the same dropped grant still marked the share rule's
    /// memory.
    ///
    /// **The share rule's memory is `File Browser`'s `pool.rs` and is not
    /// edited**: it is put back from a copy taken before the decision, which is
    /// one place saying which class had its turn rather than two.
    pub fn admit_to_carry(
        &mut self,
        direction: Direction,
        waiting: &[&'static str],
        carries: &'static str,
        now: Instant,
    ) -> Result<TheChannelsDraw, Option<&'static str>> {
        // **Step 10c piece 6, R2: where the count is one, the classes this
        // caller did not name but which met a refusal and are still asking
        // take their turn too** — [`TheWaitingRegister`]. Without them an
        // upload naming only itself took every slot a submission did not
        // happen to ask for between two files.
        let the_count_is_one = self.the_count_is_one(direction);
        let mut waiting_here: Vec<&'static str> = waiting.to_vec();
        if the_count_is_one {
            for class in self.register(direction).still_waiting(now) {
                if !waiting_here.contains(&class) {
                    waiting_here.push(class);
                }
            }
        }
        let the_share_before = self.share(direction).clone();
        let the_turns_before = self.turns(direction).clone();
        let granted = self.the_grant(direction, &waiting_here);
        let spent = granted.filter(|granted| *granted == carries);
        if the_count_is_one {
            self.the_register_hears(direction, waiting, spent, now);
        }
        let granted = granted.ok_or(None)?;
        if granted != carries {
            *self.share(direction) = the_share_before;
            match direction {
                Direction::Up => self.turns_up = the_turns_before,
                Direction::Down => self.turns_down = the_turns_before,
            }
            return Err(Some(granted));
        }
        Ok(self.the_channel_is_admitted(direction, granted, now))
    }

    /// **The decision itself, with its marks** — R7, R9, R13 and R17, over
    /// `File Browser` O5's share rule. Nothing is admitted here.
    fn the_grant(&mut self, direction: Direction, waiting: &[&'static str]) -> Option<&'static str> {
        let admissible = self.the_admissible_count(direction);
        let running = self.channels_admitted(direction);
        let turn_taking = match admissible {
            TheAdmissibleCount::TheBudgetBindsNothing => false,
            TheAdmissibleCount::Of(count) => {
                if running >= count {
                    // **`Desktop app and delivery` R13**: the budget cannot
                    // afford another channel's floor, so fewer are opened.
                    // Nothing about the transfer failed and nothing running is
                    // disturbed, and no decision is taken at the share rule
                    // because no grant was made.
                    return None;
                }
                count == 1
            }
        };
        let granted = if turn_taking {
            self.whose_turn_it_is(direction, waiting)?
        } else {
            // **`File Browser` O5's rule, at this point by its O8**, granted
            // through that document's own type: the highest-ranked class with
            // work waiting, except that a class which has had work waiting
            // throughout the last N grants takes the next one, ties among
            // qualifiers broken by rank. **Nothing declared is waiting** comes
            // back as nothing, which is a refusal and not a failure (R13).
            self.share(direction).grant(waiting)?
        };
        match direction {
            Direction::Up => self.turns_up.granted(granted),
            Direction::Down => self.turns_down.granted(granted),
        }
        Some(granted)
    }

    /// The channel a grant was made for, admitted to its direction's budget.
    fn the_channel_is_admitted(
        &mut self,
        direction: Direction,
        granted: &'static str,
        now: Instant,
    ) -> TheChannelsDraw {
        let channel = self.budget(direction).admit(granted);
        TheChannelsDraw {
            budget: match direction {
                Direction::Up => Arc::clone(&self.upload),
                Direction::Down => Arc::clone(&self.download),
            },
            channel,
            class: granted,
            ended: false,
            _admitted_at: now,
        }
    }

    /// **The two publication moments are `Server protocol and transfers` R12a's
    /// and there is no timer under either — this is the first of them.**
    ///
    /// *"At each accounting interval, which exists only while a channel is
    /// running"* — so this answers with nothing where that direction has no
    /// channel, because there is no accounting interval to publish at. The other
    /// moment is [`TheChannelsDraw::the_channel_ended`].
    pub fn the_reading_at_this_accounting_interval(
        &self,
        direction: Direction,
        now: Instant,
    ) -> Option<TheAchievedRate> {
        let mut budget = self.budget(direction);
        if budget.channels.is_empty() {
            return None;
        }
        Some(budget.the_reading(now))
    }

    /// **`Desktop app and delivery` R15b's estimate, over one request's own
    /// channels and no others.**
    ///
    /// **The caller names the subset and hands over the whole**; this point
    /// supplies the rate over its own window and does the division. **Nothing
    /// here knows what a request is**, which is deliberate: a request is
    /// `Download`'s and `File Browser`'s, and a scheduler that learned the shape
    /// of one would be holding a second copy of somebody else's fact.
    ///
    /// **The three states carrying no estimate are answered in the order the
    /// clause states them**, and in each the absence is stated rather than a
    /// number shown.
    ///
    /// **It is recomputed at every accounting interval and carries no memory of
    /// an earlier estimate** — no decay, no smoothing, no floor on how fast it
    /// may change. There is no parameter here through which an earlier one
    /// could arrive.
    pub fn the_remaining_time(
        &self,
        direction: Direction,
        the_requests_channels: &[u64],
        bytes_remaining: BytesRemaining,
        the_request_has_ended: bool,
        now: Instant,
    ) -> TheRemainingTime {
        // **The third state first, because a request that has ended has no
        // remaining time at all** — *"where a remaining time is not a reading
        // at all"*.
        if the_request_has_ended {
            return TheRemainingTime::TheRequestHasEnded;
        }
        let BytesRemaining::Of(remaining) = bytes_remaining else {
            return TheRemainingTime::TheWholeIsNotYetKnown;
        };
        let mut budget = self.budget(direction);
        let rate = budget.the_reading_over(the_requests_channels, now);
        drop(budget);
        if rate.0 == 0 {
            return TheRemainingTime::NoChannelOfThisRequestIsMovingBytes;
        }
        TheRemainingTime::Of {
            seconds: remaining / rate.0,
            taken_at: now,
        }
    }

    fn rank_of(&self, name: &str) -> Option<u8> {
        self.declared
            .iter()
            .find(|class| class.name == name)
            .map(|class| class.rank)
    }

    /// **`Desktop app and delivery` R17's closing sentence: where the admissible
    /// count is one, the guarantee is turn-taking between the classes with work
    /// waiting rather than a share.**
    ///
    /// A share is meaningless where there is one slot: each grant is a whole
    /// transfer, so the rank order alone would hand every slot to the same class
    /// for as long as it has work. **Turn-taking is the class whose last grant is
    /// furthest back, and a class that has never had one is furthest back of
    /// all**; rank breaks the tie, which is how the plan answers *who goes first*
    /// everywhere else.
    fn whose_turn_it_is(
        &self,
        direction: Direction,
        waiting: &[&'static str],
    ) -> Option<&'static str> {
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
        let taken = self.turns(direction);
        let how_far_back = |class: &'static str| -> usize { taken.how_far_back(class) };
        // `waiting` is already in rank order and `max_by_key` keeps the last
        // maximum, so the scan runs it backwards to leave the highest-ranked of
        // an equal-waiting set.
        waiting
            .iter()
            .rev()
            .copied()
            .max_by_key(|class| how_far_back(class))
    }

    /// How many grants this point has made in a direction. **Read off the
    /// counter, which is the only thing that read the whole list.**
    pub fn grants_made(&self, direction: Direction) -> usize {
        self.turns(direction).grants_made
    }
}

// ---------------------------------------------------------------------------
// The seam the byte loop draws at.
// ---------------------------------------------------------------------------

/// One admitted channel's draw on its direction's budget.
///
/// **This is the end of `Server protocol and transfers`'
/// [`DrawsFromTheSharedBudget`] seam.** `Server protocol and transfers` R2 has
/// each transfer
/// channel's byte loop *"draw from the shared budget on each accounting interval
/// and move no more than it drew"*, and `Server protocol and transfers` R1 puts
/// the budget here. **It
/// carries no cap of its own** — `Desktop app and delivery` R4 — and there is no
/// route to give it one.
///
/// **The moment is taken as a parameter on the inherent methods and from the
/// clock on the trait's.** `Server protocol and transfers` R12a requires both
/// ends of the window on the one clock the accounting point runs on, and the
/// trait's own signatures carry no moment; [`TheChannelsDraw::draw_at`] is the
/// same act with the moment supplied, which is what lets the window be driven
/// without waiting for real seconds to pass.
#[derive(Debug)]
pub struct TheChannelsDraw {
    budget: Arc<Mutex<TheDirectionsBudget>>,
    channel: u64,
    class: &'static str,
    ended: bool,
    /// The moment this point admitted the channel, on the same clock as the
    /// window's two ends.
    _admitted_at: Instant,
}

impl TheChannelsDraw {
    /// The declared class this channel was admitted under.
    pub fn the_class(&self) -> &'static str {
        self.class
    }

    /// **Which channel this is, so a caller can name the subset
    /// `Desktop app and delivery` R15b's estimate is summed over.**
    ///
    /// It is this point's own handle and means nothing outside it — **not a
    /// session, not a transfer identity, and nothing durable is written from
    /// it.** A caller collects the handles of the channels its own request
    /// opened and hands them to
    /// [`TheDesktopSideAdmissionPoint::the_remaining_time`].
    pub fn the_channel(&self) -> u64 {
        self.channel
    }

    fn held(&self) -> MutexGuard<'_, TheDirectionsBudget> {
        self.budget
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// **[`DrawsFromTheSharedBudget::draw`] with the moment supplied.**
    pub fn draw_at(&mut self, wanted: usize, now: Instant) -> usize {
        if self.ended {
            return 0;
        }
        self.held().draw(self.channel, wanted, now)
    }

    /// **[`DrawsFromTheSharedBudget::give_back`] with the moment supplied.**
    pub fn give_back_at(&mut self, unused: usize) {
        if self.ended {
            return;
        }
        self.held().give_back(self.channel, unused);
    }

    /// **The channel ended — `Desktop app and delivery` R5, and the second of
    /// `Server protocol and transfers` R12a's two publication moments.**
    ///
    /// The share returns at once. **And where this was the direction's last
    /// channel the reading is published as zero**, which is what makes a held
    /// reading safe: *"without it a finished batch would leave its last rate on
    /// the screen for the rest of the run"*. A trailing window with nothing left
    /// to count is never run down, because running it down would need a timer
    /// `Desktop app and delivery` P10b forbids — so the direction's reading is
    /// **stated** as zero at that moment and stands until a channel is admitted
    /// again.
    pub fn the_channel_ended(&mut self, now: Instant) -> Option<TheAchievedRate> {
        if self.ended {
            return None;
        }
        self.ended = true;
        let mut budget = self.held();
        let was_the_last = budget.ended(self.channel);
        if !was_the_last {
            return None;
        }
        Some(TheAchievedRate {
            value: BytesPerSecond(0),
            taken_at: now,
            direction: budget.direction,
        })
    }
}

impl Drop for TheChannelsDraw {
    /// **`Desktop app and delivery` R5's *at once*, made structural.**
    ///
    /// A channel dropped without its ending — the per-server point refusing
    /// after this one granted (`Desktop app and delivery` R12), or the loop
    /// unwinding — returns its share here rather than holding a floor nobody
    /// can spend. Where [`TheChannelsDraw::the_channel_ended`] already ran,
    /// this does nothing.
    fn drop(&mut self) {
        if self.ended {
            return;
        }
        self.held().ended(self.channel);
    }
}

impl DrawsFromTheSharedBudget for TheChannelsDraw {
    fn draw(&mut self, wanted: usize) -> usize {
        self.draw_at(wanted, Instant::now())
    }

    fn give_back(&mut self, unused: usize) {
        self.give_back_at(unused);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_file_browser::pool::THE_DECLARED_ORDER;

    // The declarations a caller hands this point. **They are the other
    // documents' own**: `File Browser` O4's five taken from that crate's
    // declared order rather than restated here, and `Rendering` §7's submission
    // class, which is step 7's and arrives as a declaration a caller supplies.
    const AUTO: &str = THE_DECLARED_ORDER[0].name;
    const MANUAL: &str = THE_DECLARED_ORDER[1].name;
    const DEPENDENCY: &str = THE_DECLARED_ORDER[2].name;
    const UPLOAD: &str = THE_DECLARED_ORDER[3].name;
    const PREVIEW: &str = THE_DECLARED_ORDER[4].name;
    const SUBMISSION: &str = "submission";

    fn what_the_other_documents_declare() -> Vec<DeclaredClass> {
        // `Rendering` §7 ranks its submission class *"below manual download,
        // above dependency-upload"*, which is a relation into a list with no gap
        // in it, so the five are spaced before the sixth is placed by O4's own
        // route. **Nothing of O4's list is edited and no rank of it is
        // restated** — the spacing is undone by
        // [`the_declared_order_with_the_update_fetch`], which re-reads the ranks
        // from the order it hands back.
        let spaced: Vec<DeclaredClass> = THE_DECLARED_ORDER
            .iter()
            .map(|class| DeclaredClass {
                rank: class.rank * 2,
                ..*class
            })
            .collect();
        let below_manual_download = spaced[1].rank + 1;
        with_a_class_a_later_document_declares(
            &spaced,
            DeclaredClass {
                name: SUBMISSION,
                declared_by: "Rendering",
                rank: below_manual_download,
            },
        )
    }

    fn a_point() -> TheDesktopSideAdmissionPoint {
        let declared =
            the_declared_order_with_the_update_fetch(&what_the_other_documents_declare())
                .expect("the update fetch places against these declarations");
        TheDesktopSideAdmissionPoint::over(declared)
    }

    const KIB: u64 = 1024;

    /// **Must-do 1 of step 10b round 1 of *jobs, history and downloads*: a
    /// grant handed back unspent leaves no mark**, at an admissible count of
    /// one and with no cap alike.
    #[test]
    fn a_grant_handed_back_unspent_leaves_no_turn_and_no_share_mark() {
        let start = Instant::now();
        // **R17's case: a count of one.** Ask first with the file of the class
        // the point would not grant, then offer the file of the class it did.
        let mut point = a_point();
        point.set_the_cap(Direction::Down, TheCap::Of(THE_FLOOR_PER_DIRECTION));
        assert_eq!(point.the_admissible_count(Direction::Down), TheAdmissibleCount::Of(1));
        let waiting = [AUTO, MANUAL];
        assert_eq!(
            point.admit_to_carry(Direction::Down, &waiting, MANUAL, start).err(),
            Some(Some(AUTO)),
            "the point's turn was the auto class's"
        );
        assert_eq!(point.grants_made(Direction::Down), 0, "an unspent grant was counted");
        assert_eq!(point.channels_admitted(Direction::Down), 0);
        let draw = point
            .admit_to_carry(Direction::Down, &waiting, AUTO, start)
            .expect("the class that was granted is still the one whose turn it is");
        assert_eq!(draw.the_class(), AUTO);
        drop(draw);
        // And turn-taking goes on from the grant that was spent.
        assert_eq!(
            point.admit_to_carry(Direction::Down, &waiting, MANUAL, start).map(|draw| draw.the_class()),
            Ok(MANUAL)
        );

        // **No cap: the share rule's memory is untouched by an unspent grant.**
        let mut unspent = a_point();
        let mut never_asked = a_point();
        for _ in 0..(unspent.n() * 2) {
            let _ = unspent.admit_to_carry(Direction::Down, &waiting, MANUAL, start);
        }
        for _ in 0..3 {
            let one = unspent.admit(Direction::Down, &waiting, start).map(|draw| draw.the_class());
            let two = never_asked.admit(Direction::Down, &waiting, start).map(|draw| draw.the_class());
            assert_eq!(one, two, "handed-back grants moved the share rule's memory");
        }
    }

    /// **`Desktop app and delivery` R17 at a count of one, with each caller
    /// naming only its own class — step 10c piece 6's R2, by name.**
    ///
    /// An upload asks for the slot per file naming the upload class alone, and
    /// a submission asks naming the submission class alone; neither knows the
    /// other is there. **They take turns**, because the refusal each meets
    /// marks it as waiting at the point. **Without the register the upload
    /// took the slot at every ask after its first file**, the submission
    /// getting it only if it happened to ask in the instant between two files
    /// — which is what the first assertion below fails on.
    #[test]
    fn at_a_count_of_one_an_upload_and_a_submission_take_turns_though_each_names_only_itself() {
        let start = Instant::now();
        let mut point = a_point();
        point.set_the_cap(Direction::Up, TheCap::Of(THE_FLOOR_PER_DIRECTION));
        assert_eq!(point.the_admissible_count(Direction::Up), TheAdmissibleCount::Of(1));

        let first = point
            .admit_to_carry(Direction::Up, &[UPLOAD], UPLOAD, start)
            .expect("the upload's first file takes the slot");
        assert!(
            point
                .admit_to_carry(Direction::Up, &[SUBMISSION], SUBMISSION, start)
                .is_err(),
            "a held slot let a second channel in"
        );
        drop(first);

        // **Each round both ask, the upload first** — it is the caller that
        // asks again the moment its file ends.
        let mut who_got_it = vec![UPLOAD];
        for round in 0..6u64 {
            let at = start + Duration::from_millis(10 * (round + 1));
            let upload = point.admit_to_carry(Direction::Up, &[UPLOAD], UPLOAD, at);
            let submission = point.admit_to_carry(Direction::Up, &[SUBMISSION], SUBMISSION, at);
            match (upload, submission) {
                (Ok(draw), Err(_)) => {
                    who_got_it.push(draw.the_class());
                    drop(draw);
                }
                (Err(_), Ok(draw)) => {
                    who_got_it.push(draw.the_class());
                    drop(draw);
                }
                (upload, submission) => panic!(
                    "one slot, one grant each round: {:?} and {:?}",
                    upload.map(|draw| draw.the_class()),
                    submission.map(|draw| draw.the_class())
                ),
            }
        }
        assert_eq!(
            who_got_it,
            vec![UPLOAD, SUBMISSION, UPLOAD, SUBMISSION, UPLOAD, SUBMISSION, UPLOAD],
            "the two classes did not take turns"
        );

        // **A class that stops asking stops holding a turn**: its mark lapses,
        // and the upload is no longer refused on its behalf.
        let asked_last = start + Duration::from_secs(1);
        let held = point
            .admit_to_carry(Direction::Up, &[UPLOAD], UPLOAD, asked_last)
            .ok()
            .or_else(|| {
                point
                    .admit_to_carry(Direction::Up, &[SUBMISSION], SUBMISSION, asked_last)
                    .ok()
            })
            .expect("one of them takes the slot");
        let _ = point.admit_to_carry(Direction::Up, &[SUBMISSION], SUBMISSION, asked_last);
        drop(held);
        let later = asked_last + A_WAITING_MARK_LAPSES_AFTER + Duration::from_millis(1);
        for step in 0..3u64 {
            let at = later + Duration::from_millis(step);
            let draw = point
                .admit_to_carry(Direction::Up, &[UPLOAD], UPLOAD, at)
                .expect("a submission that stopped asking holds no turn");
            drop(draw);
        }
    }

    /// **The register is a rule of a count of one and nothing else.** With no
    /// cap, a class that never names another is granted at every ask, as
    /// `File Browser` O5's share rule over what the caller names gives it.
    #[test]
    fn above_a_count_of_one_the_register_marks_nothing_and_merges_nothing() {
        let start = Instant::now();
        let mut point = a_point();
        assert_eq!(
            point.the_admissible_count(Direction::Up),
            TheAdmissibleCount::TheBudgetBindsNothing
        );
        let held = point
            .admit_to_carry(Direction::Up, &[SUBMISSION], SUBMISSION, start)
            .expect("admitted");
        for step in 0..5u64 {
            let at = start + Duration::from_millis(step);
            let draw = point
                .admit_to_carry(Direction::Up, &[UPLOAD], UPLOAD, at)
                .expect("no class is refused on another's behalf above a count of one");
            drop(draw);
        }
        drop(held);
        assert!(point.waiting_up.marked.is_empty());
    }

    #[test]
    fn there_is_one_upload_budget_and_one_download_budget_and_never_one_for_both() {
        // `Desktop app and delivery` R1. Two, because the promise is the total
        // in each direction and the two directions are measured against
        // different lines.
        let mut point = a_point();
        let start = Instant::now();
        point.set_the_cap(Direction::Up, TheCap::Of(BytesPerSecond(100 * KIB)));
        assert_eq!(point.the_cap(Direction::Down), TheCap::NoCapIsSet);

        let mut up = point
            .admit(Direction::Up, &[UPLOAD], start)
            .expect("admitted");
        let mut down = point
            .admit(Direction::Down, &[AUTO], start)
            .expect("admitted");
        // The uploaded bytes come out of the upload budget alone: the download
        // channel is uncapped and takes everything it asks for.
        let _ = up.draw_at(10 * 1024 * 1024, start);
        assert_eq!(down.draw_at(10 * 1024 * 1024, start), 10 * 1024 * 1024);

        // And the readings are per direction — `Desktop app and delivery` R15a.
        let a_moment_later = start + Duration::from_secs(1);
        let up_reading = point
            .the_reading_at_this_accounting_interval(Direction::Up, a_moment_later)
            .expect("a channel is running");
        let down_reading = point
            .the_reading_at_this_accounting_interval(Direction::Down, a_moment_later)
            .expect("a channel is running");
        assert_eq!(up_reading.direction(), Direction::Up);
        assert_eq!(down_reading.direction(), Direction::Down);
        assert!(down_reading.value() > up_reading.value());
    }

    #[test]
    fn the_cap_goes_in_and_comes_back_in_the_one_unit() {
        // `Desktop app and delivery` R2: nothing here carries a rate without its
        // unit, and this module performs no conversion.
        let mut point = a_point();
        point.set_the_cap(Direction::Down, TheCap::Of(BytesPerSecond(400 * KIB)));
        assert_eq!(
            point.the_cap(Direction::Down),
            TheCap::Of(BytesPerSecond(400 * KIB))
        );
        assert_eq!(point.the_floor(Direction::Down), THE_FLOOR_PER_DIRECTION);
        assert_eq!(point.the_floor(Direction::Up), THE_FLOOR_PER_DIRECTION);
    }

    #[test]
    fn the_floor_is_non_zero_and_the_window_holds_many_tens_of_accounting_periods() {
        // `Desktop app and delivery` R6: the share is non-zero, so waiting on
        // the budget is a bounded wait and `Server protocol and transfers` C19's
        // deadline has something to sit against.
        const { assert!(THE_FLOOR_PER_DIRECTION.0 > 0) };
        // `Server protocol and transfers` R12a's lower bound: the loop emits its
        // count at least once per accounting period, so the window holds many
        // tens of counts and no single write can move the figure. The period is
        // that document's own constant and is not restated here.
        let period = smoozy_server_protocol::transfers::moving::THE_ACCOUNTING_PERIOD;
        assert!(THE_TRAILING_WINDOW.as_millis() / period.as_millis() >= 30);
        // A channel held at the floor still makes a real write every period
        // rather than a dribble.
        let per_period = THE_FLOOR_PER_DIRECTION.0 * period.as_millis() as u64 / 1000;
        assert!(
            per_period >= 1024,
            "{per_period} bytes an accounting period"
        );
    }

    #[test]
    fn one_channel_is_always_admissible_and_takes_the_whole_budget() {
        // `Desktop app and delivery` R7.
        let mut point = a_point();
        let start = Instant::now();
        // A cap far below the floor. One channel is still admissible.
        point.set_the_cap(Direction::Up, TheCap::Of(BytesPerSecond(1024)));
        assert_eq!(
            point.the_admissible_count(Direction::Up),
            TheAdmissibleCount::Of(1)
        );
        let mut only = point
            .admit(Direction::Up, &[UPLOAD], start)
            .expect("admitted");
        // It takes the whole budget: a window's worth at the cap.
        assert_eq!(only.draw_at(1024 * 1024, start), 1024 * 3);
        assert!(point.admit(Direction::Up, &[UPLOAD], start).is_none());
    }

    #[test]
    fn beyond_the_first_the_count_is_the_budget_divided_by_the_floor() {
        // `Desktop app and delivery` R7 and R9: the division is by the floor and
        // by nothing else.
        let mut point = a_point();
        let start = Instant::now();
        let cap = THE_FLOOR_PER_DIRECTION.0 * 3;
        point.set_the_cap(Direction::Up, TheCap::Of(BytesPerSecond(cap)));
        assert_eq!(
            point.the_admissible_count(Direction::Up),
            TheAdmissibleCount::Of(3)
        );
        let mut held = Vec::new();
        for _ in 0..3 {
            held.push(
                point
                    .admit(Direction::Up, &[UPLOAD, DEPENDENCY], start)
                    .expect("inside the count"),
            );
        }
        assert!(
            point.admit(Direction::Up, &[UPLOAD], start).is_none(),
            "a fourth channel is one the budget cannot feed at the floor"
        );
        // And each of the three can actually draw its floor share, which is what
        // makes the floor true at the draw rather than only at admission.
        let floor_share = THE_FLOOR_PER_DIRECTION.0 * THE_TRAILING_WINDOW.as_secs();
        for channel in held.iter_mut() {
            assert_eq!(channel.draw_at(10 * 1024 * 1024, start) as u64, floor_share);
        }
    }

    #[test]
    fn with_no_cap_the_floor_binds_nothing_at_this_point() {
        // `Desktop app and delivery` R8: admission is then bounded only by the
        // concurrency numbers Rows 188 and 206 pick, which are held at the
        // per-server point and which that document's R12 says this point never
        // overrides.
        let mut point = a_point();
        let start = Instant::now();
        assert_eq!(
            point.the_admissible_count(Direction::Down),
            TheAdmissibleCount::TheBudgetBindsNothing
        );
        let mut held = Vec::new();
        for _ in 0..12 {
            held.push(
                point
                    .admit(Direction::Down, &[AUTO], start)
                    .expect("this point bounds nothing with no cap set"),
            );
        }
        assert_eq!(point.channels_admitted(Direction::Down), 12);
        assert_eq!(held[0].draw_at(999_999, start), 999_999);
    }

    #[test]
    fn no_priority_class_buys_a_larger_share_of_the_budget() {
        // `Desktop app and delivery` R9: the division is by the floor and by
        // nothing else — priority governs admission order and not rate. Two
        // mechanisms shaping one number is how a total cap stops being a
        // total.
        //
        // **What is compared is the same run with the two classes swapped.**
        // The two allowances are not equal to each other and must not be: an
        // equal split would be the per-transfer cap `Desktop app and delivery`
        // R4 forbids and Fact 75 measured at 421.9 KB/s against one shared
        // budget's 820. What that document's R9 asks is that the class name
        // changes nothing, and this is that.
        fn what_two_channels_draw(top: &'static str, bottom: &'static str) -> (usize, usize) {
            let mut point = a_point();
            let start = Instant::now();
            point.set_the_cap(
                Direction::Down,
                TheCap::Of(BytesPerSecond(THE_FLOOR_PER_DIRECTION.0 * 4)),
            );
            let mut first = point
                .admit(Direction::Down, &[top], start)
                .expect("admitted");
            let mut second = point
                .admit(Direction::Down, &[bottom], start)
                .expect("admitted");
            assert_eq!(first.the_class(), top);
            assert_eq!(second.the_class(), bottom);
            (
                first.draw_at(10 * 1024 * 1024, start),
                second.draw_at(10 * 1024 * 1024, start),
            )
        }
        // The top-ranked class first, then the bottom-ranked class first.
        assert_eq!(
            what_two_channels_draw(AUTO, PREVIEW),
            what_two_channels_draw(PREVIEW, AUTO),
            "the rank of the class a channel was admitted under buys it nothing \
             at the budget"
        );
    }

    #[test]
    fn a_finished_channel_returns_its_unused_share_at_once() {
        // `Desktop app and delivery` R5, and Fact 75's forfeited half: without
        // it the budget shrinks over the life of a batch of uneven files.
        let mut point = a_point();
        let start = Instant::now();
        point.set_the_cap(
            Direction::Up,
            TheCap::Of(BytesPerSecond(THE_FLOOR_PER_DIRECTION.0 * 2)),
        );
        let mut small = point
            .admit(Direction::Up, &[UPLOAD], start)
            .expect("admitted");
        let mut large = point
            .admit(Direction::Up, &[DEPENDENCY], start)
            .expect("admitted");
        let floor_share = (THE_FLOOR_PER_DIRECTION.0 * THE_TRAILING_WINDOW.as_secs()) as usize;
        // With both running, each is held to its floor share.
        assert_eq!(large.draw_at(10 * 1024 * 1024, start), floor_share);
        assert_eq!(small.draw_at(floor_share, start), floor_share);
        // The small one finishes. Its share is not forfeited: the large one can
        // take the rest of the window's slack at once.
        assert!(small.the_channel_ended(start).is_none());
        let a_moment_later = start + Duration::from_millis(100);
        // Two floor shares' worth is the whole window at this cap, and both were
        // spent, so the window has no slack yet — the point is that nothing is
        // reserved for the departed channel any more.
        let later = start + Duration::from_secs(4);
        assert_eq!(
            large.draw_at(10 * 1024 * 1024, later),
            (THE_FLOOR_PER_DIRECTION.0 * 2 * THE_TRAILING_WINDOW.as_secs()) as usize,
            "the whole budget, with nothing held back for a channel that ended"
        );
        assert_eq!(point.channels_admitted(Direction::Up), 1);
        assert!(point
            .the_reading_at_this_accounting_interval(Direction::Up, a_moment_later)
            .is_some());
    }

    #[test]
    fn a_channel_admitted_into_a_spent_window_waits_the_window_and_no_longer() {
        // **`Desktop app and delivery` R6, and it is what the floor exists
        // for**: waiting on the budget is a *bounded* wait, so
        // `Server protocol and transfers` C19's inactivity deadline — set well
        // above the longest legitimate stall observed, 11.1 seconds — has
        // something to sit against.
        let start = Instant::now();
        let mut point = a_point();
        point.set_the_cap(
            Direction::Up,
            TheCap::Of(BytesPerSecond(THE_FLOOR_PER_DIRECTION.0 * 4)),
        );
        let mut first = point
            .admit(Direction::Up, &[UPLOAD], start)
            .expect("admitted");
        // Alone, it takes the whole budget, which spends the whole window.
        assert!(first.draw_at(64 * 1024 * 1024, start) > 0);
        let mut second = point
            .admit(Direction::Up, &[DEPENDENCY], start)
            .expect("admitted");
        assert_eq!(
            second.draw_at(64 * 1024 * 1024, start),
            0,
            "nothing is left in the window, so the newcomer waits"
        );
        // And the wait is over inside the window rather than at some open time.
        let just_after_the_window = start + THE_TRAILING_WINDOW + Duration::from_millis(1);
        assert!(
            second.draw_at(64 * 1024 * 1024, just_after_the_window) as u64
                >= THE_FLOOR_PER_DIRECTION.0 * THE_TRAILING_WINDOW.as_secs(),
            "at least its floor share, and the wait was bounded by the window"
        );
    }

    #[test]
    fn a_refusal_is_a_reason_to_open_fewer_and_never_a_reason_to_fail() {
        // `Desktop app and delivery` R13, at this point. The answer is an
        // absence and there is no failure to turn into one; and nothing already
        // running is disturbed by it.
        let mut point = a_point();
        let start = Instant::now();
        point.set_the_cap(Direction::Up, TheCap::Of(BytesPerSecond(1024)));
        let mut running = point
            .admit(Direction::Up, &[UPLOAD], start)
            .expect("admitted");
        assert!(point.admit(Direction::Up, &[DEPENDENCY], start).is_none());
        assert_eq!(point.channels_admitted(Direction::Up), 1);
        assert!(running.draw_at(1024, start) > 0);
    }

    #[test]
    fn where_the_per_server_point_refuses_the_slot_returns_at_once() {
        // `Desktop app and delivery` R12: a channel needs both points to admit
        // it, and either may refuse. The desktop point's grant is dropped and
        // its share returns without a step anybody has to remember
        // (`Desktop app and delivery` R5).
        let mut point = a_point();
        let start = Instant::now();
        point.set_the_cap(Direction::Up, TheCap::Of(BytesPerSecond(1024)));
        {
            let _the_per_server_point_then_refused = point
                .admit(Direction::Up, &[UPLOAD], start)
                .expect("admitted");
            assert_eq!(point.channels_admitted(Direction::Up), 1);
        }
        assert_eq!(point.channels_admitted(Direction::Up), 0);
        assert!(point.admit(Direction::Up, &[UPLOAD], start).is_some());
    }

    /// A governor that accounts over the whole run — **the shape
    /// `Server protocol and transfers` R6b forbids**, built here as the test's
    /// own stand-in so the fault it was written for is shown rather than
    /// asserted. It is not application code and nothing outside this test can
    /// reach it.
    struct ACumulativeGovernor {
        cap: BytesPerSecond,
        started: Instant,
        moved: u64,
    }

    impl ACumulativeGovernor {
        fn draw(&mut self, wanted: usize, now: Instant) -> usize {
            let elapsed = now.saturating_duration_since(self.started).as_secs_f64();
            let allowed_by_now = (self.cap.0 as f64 * elapsed) as u64;
            let allowance = allowed_by_now.saturating_sub(self.moved).min(wanted as u64);
            self.moved += allowance;
            allowance as usize
        }
    }

    #[test]
    fn the_rate_is_measured_over_a_bounded_window_and_a_cumulative_governor_bursts() {
        // **`Server protocol and transfers` R6b, and it is the correction that
        // matters.** A cumulative governor answered a raised cap with a burst
        // "to about 1 000 KB/s, roughly double the new cap, and held there for
        // minutes"; a bounded trailing window landed the change inside the
        // minute it fell in. `Desktop app and delivery` R15a takes that window
        // whole and this is the test that would have caught the fault.
        let old = BytesPerSecond(384 * KIB);
        let new = BytesPerSecond(512 * KIB);
        let start = Instant::now();
        let mut point = a_point();
        point.set_the_cap(Direction::Up, TheCap::Of(old));
        let mut channel = point
            .admit(Direction::Up, &[UPLOAD], start)
            .expect("admitted");

        // A minute at the old cap, drawing at every accounting period.
        let period = Duration::from_millis(100);
        let mut moved_before = 0u64;
        for step in 0..600u32 {
            moved_before += channel.draw_at(1024 * 1024, start + period * step) as u64;
        }
        let the_change = start + period * 600;
        assert!(
            moved_before >= old.0 * 59 && moved_before <= old.0 * 61,
            "a minute at the old cap moved {moved_before} bytes"
        );

        // The cap is raised while the channel is moving.
        point.set_the_cap(Direction::Up, TheCap::Of(new));
        let window = THE_TRAILING_WINDOW.as_secs();
        let mut moved_after = 0u64;
        for step in 0..30u32 {
            moved_after += channel.draw_at(1024 * 1024, the_change + period * step) as u64;
        }
        assert!(
            moved_after <= new.0 * window,
            "the raised cap is taken up without a burst: {moved_after} bytes in \
             the window against {} the new cap allows",
            new.0 * window
        );
        assert!(
            moved_after > old.0 * window,
            "and the change did arrive rather than being ignored: {moved_after} \
             bytes against the old cap's {}",
            old.0 * window
        );

        // The same raise, against a governor that averages over the whole run.
        let mut cumulative = ACumulativeGovernor {
            cap: old,
            started: start,
            moved: old.0 * 60,
        };
        cumulative.cap = new;
        let burst = cumulative.draw(64 * 1024 * 1024, the_change);
        assert!(
            burst as u64 > new.0 * window,
            "the cumulative governor settles the raise against every byte \
             already moved and hands out {burst} bytes at once, where the new \
             cap allows {} over the whole window",
            new.0 * window
        );
    }

    #[test]
    fn a_cap_lowered_mid_flight_takes_effect_at_the_next_accounting_interval() {
        // `Server protocol and transfers` R8 and R6b, at the budget's side: no
        // transfer is restarted, re-queued or re-staged, and the change is
        // answered at the next draw rather than at the next transfer.
        let start = Instant::now();
        let mut point = a_point();
        point.set_the_cap(Direction::Up, TheCap::Of(BytesPerSecond(768 * KIB)));
        let mut channel = point
            .admit(Direction::Up, &[UPLOAD], start)
            .expect("admitted");
        let period = Duration::from_millis(100);
        for step in 0..60u32 {
            channel.draw_at(1024 * 1024, start + period * step);
        }
        let the_change = start + period * 60;
        point.set_the_cap(Direction::Up, TheCap::Of(BytesPerSecond(256 * KIB)));
        let mut moved_after = 0u64;
        for step in 0..30u32 {
            moved_after += channel.draw_at(1024 * 1024, the_change + period * step) as u64;
        }
        assert!(
            moved_after <= 256 * KIB * THE_TRAILING_WINDOW.as_secs(),
            "the lowered cap governs from the moment it is set forward: \
             {moved_after} bytes in the window"
        );
    }

    #[test]
    fn before_the_window_has_filled_the_divisor_is_the_elapsed_time() {
        // `Server protocol and transfers` R12a, taken whole by
        // `Desktop app and delivery` R15a: otherwise every transfer's first
        // three seconds read low.
        let start = Instant::now();
        let mut point = a_point();
        let mut channel = point
            .admit(Direction::Down, &[AUTO], start)
            .expect("admitted");
        channel.draw_at(100_000, start);
        let one_second_in = start + Duration::from_secs(1);
        let reading = point
            .the_reading_at_this_accounting_interval(Direction::Down, one_second_in)
            .expect("a channel is running");
        assert_eq!(
            reading.value(),
            BytesPerSecond(100_000),
            "100,000 bytes over one elapsed second, not over three"
        );
        // Once the window has filled the divisor is the window.
        let four_seconds_in = start + Duration::from_secs(4);
        channel.draw_at(30_000, four_seconds_in - Duration::from_millis(1));
        let later = point
            .the_reading_at_this_accounting_interval(Direction::Down, four_seconds_in)
            .expect("a channel is running");
        assert_eq!(
            later.value(),
            BytesPerSecond(10_000),
            "the first draw has left the window and 30,000 over three seconds is \
             what is left"
        );
        assert_eq!(later.taken_at(), four_seconds_in);
        assert_eq!(later.the_clock_it_was_taken_on(), Clock::ThisComputer);
    }

    #[test]
    fn the_reading_is_published_as_zero_when_a_directions_last_channel_ends() {
        // `Server protocol and transfers` R12a's second publication moment,
        // taken whole by `Desktop app and delivery` R15a. Without it a finished
        // batch would leave its last rate on the screen for the rest of the run.
        let start = Instant::now();
        let mut point = a_point();
        let mut first = point
            .admit(Direction::Down, &[AUTO], start)
            .expect("admitted");
        let mut second = point
            .admit(Direction::Down, &[MANUAL], start)
            .expect("admitted");
        first.draw_at(300_000, start);
        second.draw_at(300_000, start);
        let a_second_in = start + Duration::from_secs(1);
        assert!(
            first.the_channel_ended(a_second_in).is_none(),
            "not the last channel, so no reading is published here"
        );
        let ending = second
            .the_channel_ended(a_second_in)
            .expect("the direction's last channel ends");
        assert_eq!(ending.value(), BytesPerSecond(0));
        assert_eq!(ending.direction(), Direction::Down);
        assert_eq!(ending.taken_at(), a_second_in);
        // And there is no accounting interval to publish at afterwards: the
        // reading stands at zero until a channel is admitted again, with no
        // timer under it.
        assert!(point
            .the_reading_at_this_accounting_interval(Direction::Down, a_second_in)
            .is_none());
    }

    #[test]
    fn a_draw_the_loop_could_not_spend_comes_back_off_the_count() {
        // `Server protocol and transfers` R7: the window holds bytes moved, so a
        // draw the loop did not spend was never moved and must not be read as
        // rate.
        let start = Instant::now();
        let mut point = a_point();
        let mut channel = point
            .admit(Direction::Down, &[AUTO], start)
            .expect("admitted");
        assert_eq!(channel.draw_at(90_000, start), 90_000);
        channel.give_back_at(60_000);
        let a_second_in = start + Duration::from_secs(1);
        let reading = point
            .the_reading_at_this_accounting_interval(Direction::Down, a_second_in)
            .expect("a channel is running");
        assert_eq!(reading.value(), BytesPerSecond(30_000));
    }

    #[test]
    fn the_update_fetch_ranks_below_upload_and_above_the_preview_fetch_and_n_is_seven() {
        // `Desktop app and delivery` R19. The rank is a relation, not an
        // ordinal, and no edit to `File Browser` O4's list is owed.
        let order = the_declared_order_with_the_update_fetch(&what_the_other_documents_declare())
            .expect("it places");
        let names: Vec<&str> = order.iter().map(|class| class.name).collect();
        assert_eq!(
            names,
            vec![
                AUTO,
                MANUAL,
                SUBMISSION,
                DEPENDENCY,
                UPLOAD,
                THE_UPDATE_FETCHS_CLASS,
                PREVIEW
            ]
        );
        let point = TheDesktopSideAdmissionPoint::over(order);
        assert_eq!(point.n(), 7, "with `Rendering`'s submission class, seven");
        assert_eq!(point.the_bound(), 13);
    }

    #[test]
    fn the_update_fetchs_rank_is_computed_from_whatever_is_declared() {
        // An ordinal would age out the moment another document declares against
        // `File Browser` O4's list; the relation does not.
        let mut declared = what_the_other_documents_declare();
        declared.push(DeclaredClass {
            name: "something-a-later-document-declares",
            declared_by: "Somewhere",
            rank: 2,
        });
        let order = the_declared_order_with_the_update_fetch(&declared).expect("it places");
        let at = order
            .iter()
            .position(|class| class.name == THE_UPDATE_FETCHS_CLASS)
            .expect("declared");
        let upload = order
            .iter()
            .position(|class| class.name == UPLOAD)
            .expect("declared");
        let preview = order
            .iter()
            .position(|class| class.name == PREVIEW)
            .expect("declared");
        assert!(upload < at && at < preview);
        assert_eq!(order[at].rank, (at + 1) as u8);
        assert_eq!(TheDesktopSideAdmissionPoint::over(order).n(), 8);
    }

    #[test]
    fn the_update_fetch_cannot_be_placed_where_the_class_it_ranks_below_is_absent() {
        // Nothing is guessed: where the declarations handed in do not carry the
        // class `Desktop app and delivery` R19 states the relation against, the
        // relation says so rather than inventing an ordinal.
        let declared = vec![DeclaredClass {
            name: AUTO,
            declared_by: "Download",
            rank: 1,
        }];
        assert!(the_declared_order_with_the_update_fetch(&declared).is_err());
    }

    #[test]
    fn where_the_admissible_count_is_one_the_classes_take_turns() {
        // `Desktop app and delivery` R17's closing sentence. Without it an
        // auto-download arrangement that keeps trying and never switches
        // itself off would hold the single admissible slot indefinitely, and
        // `Desktop app and delivery` R13 makes a refusal not a failure, so
        // nothing would be reported.
        let mut point = a_point();
        let mut now = Instant::now();
        point.set_the_cap(Direction::Down, TheCap::Of(BytesPerSecond(1024)));
        assert_eq!(
            point.the_admissible_count(Direction::Down),
            TheAdmissibleCount::Of(1)
        );
        let mut granted = Vec::new();
        for _ in 0..6 {
            let mut channel = point
                .admit(Direction::Down, &[AUTO, MANUAL], now)
                .expect("one is always admissible");
            granted.push(channel.the_class());
            now += Duration::from_secs(1);
            channel.the_channel_ended(now);
        }
        assert_eq!(
            granted,
            vec![AUTO, MANUAL, AUTO, MANUAL, AUTO, MANUAL],
            "turn-taking between the classes with work waiting"
        );
    }

    /// **The per-class record answers exactly what the whole list of grants
    /// answered, and it does not grow.**
    ///
    /// A point that kept every class it had ever granted grew without bound
    /// over a session that runs for weeks, and the only things that read the
    /// list were *how far back was this class's last grant* and the count.
    ///
    /// **A fixed window would have changed the answer** in the corner named at
    /// [`TheTurns`]: two classes both outside the window read as never granted
    /// and the tie goes to rank. This test drives that corner — the window
    /// would have to be N, and the run holds a class back far longer.
    #[test]
    fn the_per_class_turn_record_answers_what_the_whole_list_of_grants_answered() {
        /// Turn-taking as it was: every class ever granted, in order.
        fn how_far_back_in_the_whole_list(taken: &[&'static str], class: &str) -> usize {
            match taken.iter().rposition(|granted| *granted == class) {
                Some(at) => taken.len() - at,
                None => usize::MAX,
            }
        }

        let mut point = a_point();
        let mut now = Instant::now();
        point.set_the_cap(Direction::Down, TheCap::Of(BytesPerSecond(1024)));
        assert_eq!(
            point.the_admissible_count(Direction::Down),
            TheAdmissibleCount::Of(1)
        );

        let mut whole: Vec<&'static str> = Vec::new();
        // **The run is built to drive the corner a window would get wrong.**
        // Two low classes are granted once, early; then a long stretch in which
        // neither is waiting; then both wait at the same decision. Their last
        // grants are both far outside any window of N, so a window would read
        // both as never granted and hand the slot to the higher-ranked one —
        // while turn-taking's own rule gives it to the one whose grant is
        // furthest back.
        let mut sequence: Vec<Vec<&'static str>> = vec![vec![PREVIEW], vec![DEPENDENCY]];
        for _ in 0..200 {
            sequence.push(vec![AUTO, MANUAL]);
        }
        sequence.push(vec![DEPENDENCY, PREVIEW]);
        for _ in 0..200 {
            sequence.push(vec![AUTO, MANUAL]);
        }
        sequence.push(vec![AUTO, MANUAL, DEPENDENCY, PREVIEW]);
        for (grant, waiting) in sequence.iter().enumerate() {
            // What the whole list would have answered, by the same rule.
            let mut ranked: Vec<&'static str> = waiting.clone();
            ranked.dedup();
            let from_the_whole_list = ranked
                .iter()
                .rev()
                .copied()
                .max_by_key(|class| how_far_back_in_the_whole_list(&whole, class))
                .expect("something is waiting");

            let mut channel = point
                .admit(Direction::Down, waiting, now)
                .expect("one is always admissible");
            assert_eq!(
                channel.the_class(),
                from_the_whole_list,
                "the two answers parted at grant {grant}"
            );
            whole.push(channel.the_class());
            now += Duration::from_secs(1);
            channel.the_channel_ended(now);
        }
        // The corner itself: the class granted first took the shared decision.
        assert_eq!(whole[202], PREVIEW);

        assert_eq!(point.grants_made(Direction::Down), whole.len());
        // **And the record does not grow with the number of grants**: it holds
        // one line per class that has been granted, and never more than the
        // declared classes.
        assert!(whole.len() > 400);
        assert!(point.turns_down.last_granted_at.len() <= point.n());
        assert!(point.turns_down.last_granted_at.len() <= 4);
    }

    #[test]
    fn the_share_rule_here_is_the_same_type_the_per_server_point_uses() {
        // **`File Browser` O8: the same type used twice — one point per server
        // and one above them all, each counting its own grants**, *"with the
        // same N and the same rank tie-breaker"*, so that the classes mean the
        // same thing globally that they mean at one machine. This point holds
        // one of that document's admission points rather than a second rule of
        // the same shape, and the two answer identically over the same
        // decisions.
        let declared =
            the_declared_order_with_the_update_fetch(&what_the_other_documents_declare())
                .expect("it places");
        let mut at_one_machine = AdmissionPoint::over(declared.clone());
        let mut above_them_all = TheDesktopSideAdmissionPoint::over(declared);
        // No cap, so the budget bounds nothing here and every decision is the
        // share rule's — `Desktop app and delivery` R8.
        let start = Instant::now();
        let waiting = [AUTO, DEPENDENCY, PREVIEW, THE_UPDATE_FETCHS_CLASS];
        let mut held = Vec::new();
        for _ in 0..above_them_all.the_bound() {
            let one = at_one_machine
                .grant(&waiting)
                .expect("something is waiting");
            let other = above_them_all
                .admit(Direction::Down, &waiting, start)
                .expect("nothing bounds admission with no cap set");
            assert_eq!(one, other.the_class());
            held.push(other);
        }
        assert_eq!(at_one_machine.n(), above_them_all.n());
        assert_eq!(at_one_machine.the_bound(), above_them_all.the_bound());
    }

    #[test]
    fn no_class_is_starved_at_this_point_inside_the_bound() {
        // `Desktop app and delivery` R17 over `File Browser` O5's share rule,
        // applied at this point by its O8 — same N, same rank tie-breaker,
        // grants counted here.
        let mut point = a_point();
        let mut now = Instant::now();
        point.set_the_cap(
            Direction::Down,
            TheCap::Of(BytesPerSecond(THE_FLOOR_PER_DIRECTION.0 * 4)),
        );
        let bound = point.the_bound();
        let mut when = None;
        for grant in 1..=bound {
            let mut channel = point
                .admit(Direction::Down, &[AUTO, PREVIEW], now)
                .expect("a slot is free");
            let class = channel.the_class();
            now += Duration::from_millis(100);
            channel.the_channel_ended(now);
            if class == PREVIEW {
                when = Some(grant);
                break;
            }
        }
        let when = when.expect("the lowest-ranked class is never starved");
        assert!(when <= bound, "a grant inside 2N-1, and it came at {when}");
    }

    #[test]
    fn a_command_invocation_and_a_reading_channel_are_not_admitted_here() {
        // `Desktop app and delivery` R11 and R18: this point takes transfer
        // channels and the update fetch, and nothing else can be put to it —
        // the per-server point counts all three session kinds against one
        // `MaxSessions` count, and this point never overrides it
        // (`Desktop app and delivery` R12).
        let mut point = a_point();
        let start = Instant::now();
        assert!(point
            .admit(Direction::Up, &["command-invocation"], start)
            .is_none());
        assert!(point
            .admit(Direction::Down, &["the-reading-channel"], start)
            .is_none());
        assert_eq!(point.grants_made(Direction::Up), 0);
        assert!(!THE_SEAM_R18_RECORDS.is_empty());
    }

    /// **`Server protocol and transfers` T11a: this point and the machine's own
    /// are different instruments, and neither figure is derived from the
    /// other.**
    ///
    /// **The rule is carried at both points and the two halves are held
    /// together**, because a rule with two homes is a rule one of whose homes
    /// goes stale. **This crate cannot name `Rented-server runtime`'s, so the
    /// guard is a reading of its source** — the habit
    /// `smoozy_server_protocol::release::composing` already uses for the entry
    /// point's two copies.
    #[test]
    fn the_machines_admission_point_is_a_different_instrument_and_neither_derives_the_other() {
        assert!(THE_TWO_ADMISSION_POINTS_ARE_DIFFERENT_INSTRUMENTS.contains("sessions"));
        assert!(THE_TWO_ADMISSION_POINTS_ARE_DIFFERENT_INSTRUMENTS.contains("worker starts"));
        assert!(THE_TWO_ADMISSION_POINTS_ARE_DIFFERENT_INSTRUMENTS
            .contains("neither figure is derived from the other"));

        let machine_side = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../Rented-server runtime/src/arbitration.rs");
        let text = std::fs::read_to_string(&machine_side)
            .expect("the machine's own arbitration file is readable");
        assert!(
            text.contains("THE_TWO_ADMISSION_POINTS_ARE_DIFFERENT_INSTRUMENTS"),
            "the machine's own admission point no longer carries \
             `Server protocol and transfers` T11a, so this half stands alone"
        );
        assert!(
            text.contains("neither figure is derived from the other"),
            "the machine's own half no longer says the two figures are unrelated"
        );
    }

    #[test]
    fn the_update_fetch_draws_from_the_download_budget_like_any_other_draw() {
        // `Desktop app and delivery` R11 and Z22: it is a session on no
        // connection and reaches no server, but it is bytes over the user's own
        // line that Smoozy causes.
        let mut point = a_point();
        let start = Instant::now();
        point.set_the_cap(Direction::Down, TheCap::Of(BytesPerSecond(100 * KIB)));
        let mut fetch = point
            .admit(Direction::Down, &[THE_UPDATE_FETCHS_CLASS], start)
            .expect("admitted");
        assert_eq!(fetch.the_class(), THE_UPDATE_FETCHS_CLASS);
        assert_eq!(
            fetch.draw_at(10 * 1024 * 1024, start),
            (100 * KIB * THE_TRAILING_WINDOW.as_secs()) as usize
        );
    }

    #[test]
    fn the_draw_feeds_the_byte_loops_own_seam() {
        // `Server protocol and transfers` R1 and R2: the enforcement is that
        // document's, per channel, and this is the budget it draws from.
        fn as_the_byte_loop_holds_it<B: DrawsFromTheSharedBudget>(budget: &mut B) -> usize {
            let drawn = budget.draw(261_120);
            budget.give_back(drawn);
            drawn
        }
        let mut point = a_point();
        let start = Instant::now();
        let mut channel = point
            .admit(Direction::Up, &[UPLOAD], start)
            .expect("admitted");
        assert_eq!(as_the_byte_loop_holds_it(&mut channel), 261_120);
        // Everything drawn was given back, so nothing was moved.
        let reading = point
            .the_reading_at_this_accounting_interval(
                Direction::Up,
                start + Duration::from_millis(100),
            )
            .expect("a channel is running");
        assert_eq!(reading.value(), BytesPerSecond(0));
    }

    #[test]
    fn a_draw_never_exceeds_what_was_asked_for() {
        let mut point = a_point();
        let start = Instant::now();
        point.set_the_cap(Direction::Up, TheCap::Of(BytesPerSecond(10 * 1024 * 1024)));
        let mut channel = point
            .admit(Direction::Up, &[UPLOAD], start)
            .expect("admitted");
        assert_eq!(channel.draw_at(1_000, start), 1_000);
    }

    // ── `Desktop app and delivery` R15b's remaining-time estimate ───────────

    #[test]
    fn the_estimate_is_bytes_remaining_over_this_requests_own_rate() {
        // `Desktop app and delivery` R15b: bytes remaining divided by the
        // request's own achieved rate, over `Server protocol and transfers`
        // R12a's window.
        let mut point = a_point();
        let start = Instant::now();
        let mut mine = point
            .admit(Direction::Up, &[UPLOAD], start)
            .expect("admitted");
        let ours = mine.the_channel();
        // A megabyte a second, over a full window.
        for tick in 0..30u32 {
            let at = start + Duration::from_millis(100 * u64::from(tick));
            mine.draw_at(100 * 1024, at);
        }
        let now = start + Duration::from_millis(3_000);
        let estimate = point.the_remaining_time(
            Direction::Up,
            &[ours],
            BytesRemaining::Of(10 * 1024 * 1024),
            false,
            now,
        );
        let seconds = estimate.seconds().expect("bytes are moving and the whole is known");
        assert!(
            (9..=11).contains(&seconds),
            "ten megabytes at about a megabyte a second is about ten seconds, and it read {seconds}"
        );
    }

    #[test]
    fn the_estimate_is_over_this_requests_channels_and_no_others() {
        // `Desktop app and delivery` R15b: *"A per-direction rate must never be
        // used here: it rises when another request's channels end and falls when
        // one is admitted, and neither event is about this request."*
        let mut point = a_point();
        let start = Instant::now();
        let mut mine = point
            .admit(Direction::Up, &[UPLOAD], start)
            .expect("admitted");
        let mut somebody_elses = point
            .admit(Direction::Up, &[AUTO], start)
            .expect("admitted");
        let ours = mine.the_channel();
        for tick in 0..30u32 {
            let at = start + Duration::from_millis(100 * u64::from(tick));
            mine.draw_at(100 * 1024, at);
            // The other request moves nine times as much and must not enter the
            // estimate at all.
            somebody_elses.draw_at(900 * 1024, at);
        }
        let now = start + Duration::from_millis(3_000);
        let mine_alone = point
            .the_remaining_time(
                Direction::Up,
                &[ours],
                BytesRemaining::Of(10 * 1024 * 1024),
                false,
                now,
            )
            .seconds()
            .expect("bytes are moving");
        let both = point
            .the_remaining_time(
                Direction::Up,
                &[ours, somebody_elses.the_channel()],
                BytesRemaining::Of(10 * 1024 * 1024),
                false,
                now,
            )
            .seconds()
            .expect("bytes are moving");
        assert!(
            mine_alone > both * 5,
            "the estimate took in another request's channels: alone {mine_alone}, both {both}"
        );
    }

    #[test]
    fn the_three_states_that_carry_no_estimate() {
        let mut point = a_point();
        let start = Instant::now();
        let mut mine = point
            .admit(Direction::Up, &[UPLOAD], start)
            .expect("admitted");
        let ours = mine.the_channel();
        let now = start + Duration::from_millis(3_000);

        // **One: no channel of this request is moving bytes** — every unit
        // waiting on admission or on the budget, so the divisor is zero.
        assert_eq!(
            point.the_remaining_time(
                Direction::Up,
                &[ours],
                BytesRemaining::Of(10 * 1024 * 1024),
                false,
                now
            ),
            TheRemainingTime::NoChannelOfThisRequestIsMovingBytes
        );

        mine.draw_at(1024 * 1024, start);

        // **Two: the whole is not yet known** — `Download` M17's still-running
        // enumeration.
        assert_eq!(
            point.the_remaining_time(
                Direction::Up,
                &[ours],
                BytesRemaining::TheWholeIsNotYetKnown,
                false,
                now
            ),
            TheRemainingTime::TheWholeIsNotYetKnown
        );

        // **Three: the request has ended**, where a remaining time is not a
        // reading at all — and it answers so even where both operands are in
        // hand.
        assert_eq!(
            point.the_remaining_time(
                Direction::Up,
                &[ours],
                BytesRemaining::Of(10 * 1024 * 1024),
                true,
                now
            ),
            TheRemainingTime::TheRequestHasEnded
        );
    }

    #[test]
    fn a_channel_verifying_a_prefix_composes_no_estimate_and_needs_no_fourth_state() {
        // `Server protocol and transfers` G4b: the verification moves no bytes
        // and draws no rate budget, so the rate over this request's channels is
        // zero and the estimate is absent — *"no completion estimate treats a
        // resume as starting at the checkpoint"*, delivered by the arithmetic
        // rather than by a rule added beside it.
        let mut point = a_point();
        let start = Instant::now();
        let mut mine = point
            .admit(Direction::Up, &[UPLOAD], start)
            .expect("admitted");
        let ours = mine.the_channel();
        // It moved bytes before the resume, and then stopped for the twelve
        // minutes the verification takes on a nearly complete 4 GiB prefix.
        mine.draw_at(1024 * 1024, start);
        let long_after = start + Duration::from_secs(60);
        assert_eq!(
            point.the_remaining_time(
                Direction::Up,
                &[ours],
                BytesRemaining::Of(4 * 1024 * 1024 * 1024),
                false,
                long_after
            ),
            TheRemainingTime::NoChannelOfThisRequestIsMovingBytes
        );
    }

    #[test]
    fn the_estimate_carries_no_memory_of_an_earlier_one() {
        // `Desktop app and delivery` R15b: *"recomputed from the trailing window
        // at every accounting interval"*, with no decay, no smoothing and no
        // floor on how fast it may change. **There is no parameter through
        // which an earlier estimate could arrive**, which is what this reads out
        // of the signature.
        let source = include_str!("scheduling.rs");
        let working = source
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half");
        let opens = working
            .find("pub fn the_remaining_time(")
            .expect("the estimate has a function");
        // **The parameters alone** — the answer is a `TheRemainingTime` and
        // that is what the function is for.
        let parameters =
            &working[opens..opens + working[opens..].find(')').expect("the parameters close")];
        for remembered in ["previous", "earlier", "last_estimate", "TheRemainingTime"] {
            assert!(
                !parameters.contains(remembered),
                "`the_remaining_time` takes {remembered}.                  **`Desktop app and delivery` R15b: the estimate is recomputed                  from the trailing window at every accounting interval and                  carries no memory of an earlier estimate** — no decay, no                  smoothing, no floor on how fast it may change"
            );
        }
    }

    // ── A folder of files, one after another: the window keeps a finished
    // file's bytes until they age out ────────────────────────────────────────
    //
    // **Transfer wave piece 5's two probes, kept.** An upload or a manual
    // download moves one file at a time, so every file's end is a channel's
    // end. `Server protocol and transfers` R12a's reading is *"the bytes the
    // accounting point counted in the last three seconds"*, and its R10 bound
    // names the window reading mixed *"after a channel is admitted or ends"*
    // — so a finished file's bytes are in the window until they are three
    // seconds old. And the cap is a total across the line
    // (`Desktop app and delivery` R1): bytes a finished file moved crossed it.

    #[test]
    fn the_speed_on_a_folder_upload_reads_what_is_moving_across_files() {
        // Files of 500,000 bytes, each moved in ten draws a quarter of a second
        // apart in all, one after another with no cap: two megabytes a second
        // all along. The speed beside the cap reads that, and so does a
        // request's own rate over its files (`Desktop app and delivery` R15b).
        let mut point = a_point();
        let start = Instant::now();
        let step = Duration::from_millis(25);
        let mut now = start;
        let mut the_requests_channels = Vec::new();
        let mut readings = Vec::new();
        for _ in 0..24 {
            let mut file = point
                .admit_to_carry(Direction::Up, &[UPLOAD], UPLOAD, now)
                .expect("admitted");
            the_requests_channels.push(file.the_channel());
            for _ in 0..10 {
                assert_eq!(file.draw_at(50_000, now), 50_000, "no cap is set");
                now += step;
                if now.duration_since(start) > THE_TRAILING_WINDOW {
                    let reading = point
                        .the_reading_at_this_accounting_interval(Direction::Up, now)
                        .expect("a file is moving");
                    readings.push(reading.value().0);
                }
            }
            let _ = file.the_channel_ended(now);
        }
        let (lowest, highest) = (
            *readings.iter().min().expect("readings were taken"),
            *readings.iter().max().expect("readings were taken"),
        );
        assert!(
            lowest >= 1_800_000 && highest <= 2_200_000,
            "two megabytes a second moved across the folder, and once the window had \
             filled the speed read {lowest} to {highest} bytes a second"
        );
        // The same window, summed over this request's own files.
        let last = point
            .admit_to_carry(Direction::Up, &[UPLOAD], UPLOAD, now)
            .expect("admitted");
        the_requests_channels.push(last.the_channel());
        let seconds = point
            .the_remaining_time(
                Direction::Up,
                &the_requests_channels,
                BytesRemaining::Of(20_000_000),
                false,
                now,
            )
            .seconds()
            .expect("the request's files moved bytes inside the window");
        assert!(
            (9..=11).contains(&seconds),
            "twenty megabytes left at two megabytes a second is about ten seconds, \
             and the estimate read {seconds}"
        );
    }

    #[test]
    fn a_cap_is_held_across_a_folder_of_files() {
        // A cap of one megabyte a second up, and twenty files of two megabytes
        // each, one after another, drawn the way the byte loop draws: up to a
        // chunk at every accounting interval, and nothing moves where nothing
        // was drawn.
        let cap = 1_000_000u64;
        let mut point = a_point();
        point.set_the_cap(Direction::Up, TheCap::Of(BytesPerSecond(cap)));
        let start = Instant::now();
        let period = Duration::from_millis(100);
        let mut now = start;
        let mut moved: Vec<(Instant, u64)> = Vec::new();
        for _ in 0..20 {
            let mut file = point
                .admit_to_carry(Direction::Up, &[UPLOAD], UPLOAD, now)
                .expect("admitted");
            let mut left = 2_000_000u64;
            while left > 0 {
                let got = file.draw_at(262_144.min(left as usize), now) as u64;
                if got > 0 {
                    moved.push((now, got));
                    left -= got;
                } else {
                    now += period;
                }
            }
            let _ = file.the_channel_ended(now);
        }
        let window = THE_TRAILING_WINDOW;
        let the_most_a_window_may_hold = cap * window.as_secs();
        for (at, _) in &moved {
            let in_the_window: u64 = moved
                .iter()
                .filter(|(then, _)| *then <= *at && at.duration_since(*then) <= window)
                .map(|(_, bytes)| bytes)
                .sum();
            assert!(
                in_the_window <= the_most_a_window_may_hold,
                "{in_the_window} bytes moved in the three seconds to {:.1} s, where a cap \
                 of one megabyte a second allows {the_most_a_window_may_hold}",
                at.duration_since(start).as_secs_f64()
            );
        }
        let took = now.duration_since(start).as_secs();
        assert!(
            took >= 37,
            "forty megabytes under a cap of one megabyte a second took {took} s"
        );
    }


}

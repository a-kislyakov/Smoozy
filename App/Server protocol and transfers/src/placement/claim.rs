//! **`Rented-server runtime` G1b's claim, performed: the exclusive create of the
//! claim file, through the subsystem's own open.**
//!
//! # Why this is a different instrument from the machine's
//!
//! `Rented-server runtime` G1a's kernel-held lock *"belongs to a machine-side
//! holder and to no other"*. Before hand-over there is no Smoozy program on the
//! machine at all, the command channel's invocations are per-invocation
//! processes whose exit releases any lock they took, and **the file-transfer
//! subsystem's observed request set carries no lock operation of any kind**. So
//! what claims the transaction here is **one operation rather than a
//! mechanism**: the exclusive create of the claim file itself.
//!
//! **The operation is revision 3's own.** `SSH_FXF_EXCL`,
//! `draft-ietf-secsh-filexfer-02` §6.3, read 2026-08-22: *"Causes the request to
//! fail if the named file already exists. SSH_FXF_CREAT MUST also be specified
//! if this flag is used."*
//!
//! **It is observed refused on rented machines and not only at the caller's own
//! end.** The multi-provider batch of 2026-08-22 posed it on three machines
//! across three sellers, two operating-system families, two machine kinds and
//! three versions of the server software — *"Exclusive create over an existing
//! path | refused (`Failure`)"* — with the control on the same row, a plain
//! create over the same path succeeding.
//!
//! # What this claim is worth, and it is one degree weaker than the other
//!
//! **`Rented-server runtime` G18 reads *live* one degree weaker here and that
//! document says so**: an unacquirable claim naming another installation may be
//! live or abandoned, because there is no process whose exit releases it. **No
//! function here takes an age or a deadline**, which is that document's own
//! Section 16 item 23 and is the price of refusing one.
//!
//! # The window this does not paper over
//!
//! **The create and the write of the owner's fields are two operations**, so
//! there is a window in which the file exists and holds nothing readable. A
//! claimant meeting it is told that a claim stands and that its owner could not
//! be read, and **it changes no phase, no pointer, no staging path and no health
//! result** — which is exactly what `Rented-server runtime` G1a says a failed
//! acquisition does. **The same shape the machine's own half takes**, and for
//! the same reason.
//!
//! **A claim this call created and then could not fill is removed**, so that a
//! claim nobody took leaves no file behind. Nothing is taken from anybody by
//! that removal, because the claim was never taken.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` G1a, G1b, G10, G18, G20.

use smoozy_runtime::package::phase::{
    claim, Claim, ClaimOutcome, Phase, TransactionOwner,
};
use smoozy_runtime::the_program::records;
use smoozy_runtime::written_form;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::placement::records::{the_record_at, WhetherTheDurabilityOrderingApplied};
use crate::subsystem::client::Subsystem;
use crate::subsystem::{StatusCode, SubsystemFailure};

/// **The claim's own path, and it is the same one `Rented-server runtime` G1a
/// names.**
///
/// *"The lock is at one fixed path in the package area — the same path G1b
/// claims."* The name is
/// [`smoozy_runtime::the_program::records::THE_PACKAGE_TRANSACTION_CLAIM`],
/// which is where that name lives; **this function composes the path and invents
/// no second name for it**.
pub fn the_claims_own_path(package_area: &str) -> String {
    the_record_at(package_area, records::THE_PACKAGE_TRANSACTION_CLAIM)
}

/// The claim's path, as a value a caller can hold on to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheClaimsOwnPath(pub String);

impl TheClaimsOwnPath {
    /// In a package area.
    pub fn in_the_package_area(package_area: &str) -> TheClaimsOwnPath {
        TheClaimsOwnPath(the_claims_own_path(package_area))
    }
}

/// **How many bytes of a standing claim are read.**
///
/// Bounded rather than open, for the reason
/// [`crate::reconnection::THE_MOST_THE_RECORD_MAY_HOLD`] gives: a length taken
/// off a far end is a number the far end chose. **A claim longer than this is a
/// claim that does not read back**, which is the refusal branch and not a
/// truncation that parses.
pub const THE_MOST_A_CLAIM_MAY_HOLD: usize = 8 * 1024;

/// **What taking the claim reached, and it is the rules half's three answers
/// with one more beside them.**
///
/// **The fourth is this side's own and the file's opening already promises
/// it**: the create and the write of the owner's fields are two operations, so
/// a claimant can meet a claim file that exists and holds nothing readable.
/// `Rented-server runtime` G1a says what happens then — *the claimant is told
/// that a claim stands and that its owner could not be read*, and it changes no
/// phase, no pointer, no staging path and no health result. **That is not the
/// machine's file server failing**, and reporting it as one tells the user a
/// perfectly good rental is broken.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTakingTheClaimReached {
    /// The rules half's own answer, unchanged.
    TheRulesHalfSays(ClaimOutcome),
    /// **`Rented-server runtime` G1a's window.** A claim stands and its owner
    /// could not be read.
    AClaimStandsWhoseOwnerCouldNotBeRead,
}

/// **`Rented-server runtime` G1b's claim, taken.**
///
/// **The exclusive create is the claim**, and what is done about one that
/// already stands is not decided here: it is
/// [`smoozy_runtime::package::phase::claim`]'s, which is the one place that
/// judgement lives on either performer.
///
/// **One take-over is permitted and it needs no timer: a claim naming the
/// claimant's own installation identity** (`Rented-server runtime` G10's
/// identity). `Desktop app and delivery` admits one ordinary running instance
/// per user, so a claim written by this installation cannot also be held by a
/// live process of it. **A claim naming another installation is never taken
/// over, whatever its age.**
pub async fn take_the_claim<S>(
    subsystem: &mut Subsystem<S>,
    package_area: &str,
    mine: &TransactionOwner,
    the_phase_it_is_claimed_against: Phase,
) -> Result<(WhatTakingTheClaimReached, WhetherTheDurabilityOrderingApplied), SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let at = the_claims_own_path(package_area);
    // **The claim's own create and fill are not a published write**, so the
    // only S20 question this function raises is the take-back's, below. Where
    // no take-back happens the answer is the announcement's own, asked of the
    // one function that scopes S20.
    let announced = crate::placement::records::whether_the_durability_ordering_applies(
        subsystem
            .announcement()
            .announces(crate::subsystem::wire::extension::FSYNC),
    );

    match subsystem.create_exclusively(at.as_bytes()).await {
        Ok(handle) => {
            // **A claim that was not taken leaves no file behind.** Each of the
            // two writes can fail, and a file left by a failed one holds
            // nothing readable for ever: every later attempt then reads zero
            // bytes and refuses, while nothing anywhere removes it —
            // `Rented-server runtime` G20's sweep does not reach the claim path
            // and the release is reached only by a caller that acquired.
            if let Err(why) = fill_the_created_claim(
                subsystem,
                &handle,
                mine,
                the_phase_it_is_claimed_against,
            )
            .await
            {
                let _ = subsystem.close(&handle).await;
                let _ = subsystem.remove(at.as_bytes()).await;
                return Err(why);
            }
            subsystem.close(&handle).await?;
            Ok((
                WhatTakingTheClaimReached::TheRulesHalfSays(ClaimOutcome::Acquired),
                announced,
            ))
        }
        // **The refusal is the flag rather than a permission**, which is what
        // the control either side of the observation establishes: a plain
        // create over the same path succeeded.
        Err(SubsystemFailure::TheServerRefused(StatusCode::Failure)) => {
            let standing = match what_is_claimed(subsystem, package_area).await {
                Ok(standing) => standing,
                // **The window this file's opening names, answered the way the
                // opening says rather than as a subsystem failure.** A claim
                // that is there and does not read back is a claim that stands
                // whose owner could not be read — it is not the connection or
                // the machine's own file-transfer server failing under the
                // transaction, and reporting it as one tells the user a
                // perfectly good rental is broken. **Nothing on the machine is
                // changed on either reading**; the whole difference is in what
                // the caller is told.
                Err(SubsystemFailure::TheAnswerCouldNotBeRead) => {
                    return Ok((
                        WhatTakingTheClaimReached::AClaimStandsWhoseOwnerCouldNotBeRead,
                        announced,
                    ))
                }
                Err(why) => return Err(why),
            };
            match claim(Some(&standing), &mine.installation) {
                ClaimOutcome::TakenBackFromThisInstallation => {
                    // **The take-back replaces the claim's contents in place.**
                    // Removing and re-creating would leave a window in which
                    // the transaction had no owner at all, which is the one
                    // thing the claim exists to prevent.
                    let ordering =
                        write_the_claim_into(subsystem, &at, mine, standing.phase).await?;
                    Ok((
                        WhatTakingTheClaimReached::TheRulesHalfSays(
                            ClaimOutcome::TakenBackFromThisInstallation,
                        ),
                        ordering,
                    ))
                }
                // The rules half answers `Acquired` only for an absent claim,
                // which the branch above has already handled.
                _ => Ok((
                    WhatTakingTheClaimReached::TheRulesHalfSays(ClaimOutcome::InProgress {
                        owner: standing.owner,
                        phase: standing.phase,
                    }),
                    announced,
                )),
            }
        }
        Err(why) => Err(why),
    }
}

/// **What the standing claim says, read as content.**
///
/// **A claim that is there and does not read back is never read as an absent
/// one**, and it is never read as an owner either: the caller is told a claim
/// stands and its owner could not be read, and it acts on nothing.
pub async fn what_is_claimed<S>(
    subsystem: &mut Subsystem<S>,
    package_area: &str,
) -> Result<Claim, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let at = the_claims_own_path(package_area);
    let handle = subsystem.open_to_read(at.as_bytes()).await?;
    let read = subsystem.read_at(&handle, 0, THE_MOST_A_CLAIM_MAY_HOLD).await;
    subsystem.close(&handle).await?;
    let bytes = read?;
    written_form::read::<Claim>(&bytes).map_err(|_| SubsystemFailure::TheAnswerCouldNotBeRead)
}

/// **Removed at the terminal write, and by nothing else.**
///
/// `Rented-server runtime` G1b: *"It is removed at the terminal write, and by
/// nothing else."* **There is no descriptor to close after it here**, which is
/// the whole difference from the machine's own half: nothing on this side holds
/// a lock whose release could race the removal.
pub async fn release_the_claim_at_the_terminal_write<S>(
    subsystem: &mut Subsystem<S>,
    package_area: &str,
) -> Result<(), SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let at = the_claims_own_path(package_area);
    match subsystem.remove(at.as_bytes()).await {
        Ok(()) => Ok(()),
        // Already gone is removed.
        Err(why) if why.is_the_path_not_being_there() => Ok(()),
        Err(why) => Err(why),
    }
}

/// The two acts that turn the created file into a claim that reads.
async fn fill_the_created_claim<S>(
    subsystem: &mut Subsystem<S>,
    handle: &crate::subsystem::Handle,
    mine: &TransactionOwner,
    phase: Phase,
) -> Result<(), SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let standing = Claim {
        owner: mine.clone(),
        phase,
    };
    let bytes = written_form::write(&standing)
        .map_err(|_| SubsystemFailure::TheAnswerCouldNotBeRead)?;
    subsystem.send_a_write(handle, 0, &bytes).await?;
    subsystem.settle_the_writes().await
}

/// The claim's contents, replaced in place — the take-back's own write.
async fn write_the_claim_into<S>(
    subsystem: &mut Subsystem<S>,
    at: &str,
    mine: &TransactionOwner,
    phase: Phase,
) -> Result<WhetherTheDurabilityOrderingApplied, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let standing = Claim {
        owner: mine.clone(),
        phase,
    };
    let bytes = written_form::write(&standing)
        .map_err(|_| SubsystemFailure::TheAnswerCouldNotBeRead)?;
    crate::placement::records::publish_durably(subsystem, at, &bytes).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_claims_path_carries_the_one_name_that_record_has() {
        assert_eq!(
            the_claims_own_path("/opt/smoozy/package"),
            format!("/opt/smoozy/package/{}", records::THE_PACKAGE_TRANSACTION_CLAIM)
        );
    }

    /// **No function in this file takes an age or a deadline**, and that
    /// absence is `Rented-server runtime` G18's own refusal of one. The test is
    /// a reading of this file's own text, because what is being asserted is an
    /// absence and there is nothing to call.
    #[test]
    fn nothing_here_takes_an_age_or_a_deadline() {
        let whole = include_str!("claim.rs");
        let working = whole
            .split_once("\n#[cfg(test)]")
            .expect("this file has a test module")
            .0;
        for line in working.lines() {
            let code = line.trim_start();
            if code.starts_with("//") {
                continue;
            }
            assert!(
                !code.contains("Duration") && !code.contains("Instant"),
                "a claim in this file may not be judged on time: {line}"
            );
        }
    }
}

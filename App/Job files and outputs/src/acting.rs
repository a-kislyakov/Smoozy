//! The one seam between the half that decides and the machine.
//!
//! **Everything this document does to a machine goes through here**, so the
//! half that decides — the layout, the names, the budget, the manifest, the
//! abandoned-material derivation — is testable at the desk with the machine's
//! answers arriving as values. **The real end is [`crate::wiring`]**, over the
//! file-transfer subsystem `Server protocol and transfers` §1.1 makes the
//! second mechanism.
//!
//! **§1.1's prohibition is kept by there being no command string here at all.**
//! *"No user path, package path, option, identifier or other value of user,
//! provider or machine origin is ever interpolated into a command string."*
//! Every path below crosses as a protocol field, which is what the subsystem
//! carries, so nothing in this crate composes a command.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S17.

/// What the machine said, in the only separations that are established.
///
/// **`Server protocol and transfers` §1.3's two status codes are two answers**
/// and that document already reads them apart, so they are two values here.
///
/// **N10 is why there is no *it is already there* value.** *"The file-transfer
/// channel refuses the second creation with a **generic** failure code and
/// gives no way to tell *already exists* from any other failure"* without a
/// separate stat and the race that comes with it. **So no attempt is made to
/// tell them apart**: any refusal fails the submission, which is the correct
/// outcome for both readings, and a value here claiming to know which it was
/// would be an invention.
///
/// **Two of the values below are Smoozy's own readings rather than the
/// machine's words**, and they are separate for the reason N6 gives of the
/// length budget: *"the cause is established before the machine is asked"*.
/// A capability the connection never announced is not the machine refusing
/// anything — it is a check Smoozy made itself — and an answer that could not
/// be read is Smoozy declining to guess. **Neither carries a word of the
/// machine's, so neither crosses `Security and licence` §13 rule 5, and
/// neither is the invention N10 forbids**, which is about concluding *already
/// exists* from a refusal the machine did make.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheMachineSaid {
    /// The path is not there.
    ThePathIsNotThere,
    /// The login may not.
    TheLoginMayNot,
    /// **Anything else the server refused**, including N10's generic failure.
    /// Nothing is concluded from it beyond that the operation did not happen.
    ItRefused,
    /// The connection or the channel is gone.
    TheChannelIsGone,
    /// **This connection never offered the operation.** Smoozy's own check,
    /// made before the machine is asked: `Server protocol and transfers` §1.2
    /// never attempts a capability that was not announced, so the machine
    /// refused nothing and is not broken. **M8 is what makes this fail a
    /// submission**: a write that cannot be made durable fails and the job is
    /// not placed, and this value is the cause reaching the caller intact.
    ThisConnectionNeverOfferedTheOperation,
    /// **The answer could not be read.** The server answered something this
    /// revision does not define at that point and **nothing is guessed from
    /// it**, which is a reading of Smoozy's rather than a refusal of the
    /// machine's.
    TheAnswerCouldNotBeRead,
}

/// The operations this document needs of a machine, and no others.
pub trait TheMachinesDirectories {
    /// **N9: the plain create-directory operation, and never the make-parents
    /// form.** Observed — the make-parents form *"returned `rc=0` against the
    /// already-existing folder"*, and it *"is the silent collision path, where
    /// two jobs land in one folder with nothing reported"*. **This trait offers
    /// no make-parents operation at all**, so there is nothing to reach for.
    fn make_one_directory(
        &mut self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<(), WhatTheMachineSaid>> + Send;

    /// One whole file, written and **made durable before the call returns** —
    /// *durably* as M8 defines it, which is **three of the four parts** `State,
    /// identity and recovery` §1.3 rule 6 names and not all four.
    ///
    /// **What this keeps:** the bytes are written, and the file itself is
    /// flushed to the storage device before the call returns.
    ///
    /// **What it does not keep:** rule 6's last clause. **The directory entry
    /// is not flushed**, so a rename published from a file written here is
    /// **ordered but not durable** — the rename follows the file's own flush,
    /// and the directory that afterwards holds the published name is never
    /// flushed at all. **No route to it is invented here**: the subsystem's
    /// flush takes an open file handle, there is no operation on it that opens
    /// a directory and hands one back, and whether a server would answer a
    /// flush against such a handle is read nowhere in this project. M8 carries
    /// what that costs.
    fn write_a_whole_file_durably(
        &mut self,
        path: &str,
        bytes: &[u8],
    ) -> impl std::future::Future<Output = Result<(), WhatTheMachineSaid>> + Send;

    /// One whole file back. **L12's read-back is this**, taken under the login
    /// the render will run as.
    fn read_a_whole_file(
        &mut self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, WhatTheMachineSaid>> + Send;

    /// **M5's publication and M6's operation**: the base rename, which *"refuses
    /// to replace an existing target"*. **The target here never exists**,
    /// because a job identity is never reused, so no extension is needed and
    /// S17's declared degradation does not reach this unit.
    fn rename_without_replacing(
        &mut self,
        from: &str,
        to: &str,
    ) -> impl std::future::Future<Output = Result<(), WhatTheMachineSaid>> + Send;

    /// One file removed. **K4's sweep** reaches staged material with it.
    fn remove_one_file(
        &mut self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<(), WhatTheMachineSaid>> + Send;

    /// One empty directory removed. **K4's sweep** reaches an abandoned job
    /// output folder with it, and *"a folder whose record was never published
    /// has held no render, since without the published record no job ever
    /// joined the line"* — so it is empty by construction and this operation
    /// never has to reach inside one.
    fn remove_one_empty_directory(
        &mut self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<(), WhatTheMachineSaid>> + Send;

    /// The names in one directory, **as bytes**.
    ///
    /// **N15: no reading of a folder is line-oriented and no rendered line is
    /// ever parsed.** A newline in a name splits a rendered listing across
    /// lines; the protocol's own field does not. There is no text-returning
    /// listing on this trait, which is that rule kept by the type.
    fn list_one_directory(
        &mut self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Vec<u8>>, WhatTheMachineSaid>> + Send;
}

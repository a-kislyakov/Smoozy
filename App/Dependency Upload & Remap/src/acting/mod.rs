//! The half that acts — the readings driven through the supervisor, the walk's
//! real enumeration, the transfers, the fixing worker, and the material the
//! render applies.
//!
//! **[`crate::arriving_later`] is the list this folder answers**, act by act
//! with the clause each belongs to, and that file is where a reader starts.
//!
//! **Three rules bind everything below, and each is another document's.**
//!
//! - **No path this module touches is ever spliced into a command line** (A8).
//!   `Server protocol and transfers` §1.1 makes a command a compile-time
//!   constant, so a machine-origin path rides that document's own planted-link
//!   carriage and the command names the token. [`blender`] holds every command
//!   this module sends and every carriage site it plants.
//! - **Nothing reads a file's presence, its name or its size as state**
//!   (`State, identity and recovery` §1.3 rule 5, and this document's V15 and
//!   Q10). Every reading below answers a comparison the plan orders.
//! - **A reading that did not complete is not a reading** (G3). No branch here
//!   turns a missing answer into an empty one.
//!
//! **Every act on the machine now goes through its supervisor.** The check's
//! reading moved to a file-inspection worker at step 10c's dependency wave
//! piece 2; **the fixing ([`fixing_worker`]) and Q3's digest of a machine file
//! ([`machine`]) followed at the 38's piece 7**, when `Dependency Upload &
//! Remap` O2 housed both as file-inspection operations. Until then the fixing
//! was a bare command through a link that nothing called, and the digest was a
//! stated absence that answered *did not complete*.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` §1.1. `State,
//! identity and recovery` §1.3.

pub mod blender;
pub mod checking;
pub mod fixing_worker;
pub mod machine;
pub mod marks;
/// **The remap material's fragment, at the path it has always been reached
/// at.** It lives whole in `smoozy-dependencies-machine`, because the rented
/// machine composes it into the launch material and reads its lines back.
/// Split at step 10c piece 3.
pub use smoozy_dependencies_machine::material;
pub mod placing;
pub mod reading_script;
pub mod sending;

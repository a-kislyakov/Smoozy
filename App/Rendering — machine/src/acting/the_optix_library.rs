//! **L24b performed: before a job naming `OPTIX` is launched, the worker asks
//! the system's own loader for the OptiX library, under the render's own login
//! and environment.** Added 2026-10-03, with the Creator's ruling at L23 that
//! NVIDIA cards render with OptiX.
//!
//! **Why the worker looks at all.** Where the library is missing, Blender's own
//! refusal is L24's second line, *Error: Found no Cycles device of the
//! specified type* — the same line a broken driver or absent hardware gives
//! (L25), so it does not tell the user what is missing, and nothing may be read
//! from a stream to tell them (`Rented-server runtime` §11.4 and M20; L28).
//!
//! **The lookup is not written here.** It is `Server Setup`'s
//! `the_optix_library::THE_LOADERS_OWN_LOOKUP`, where the loader's own
//! publication is quoted, because `Server Setup` R1 item 5's fifth Ubuntu act
//! looks *"by the same lookup `Rendering` L24b makes"* and one lookup in two
//! copies is two lookups.
//!
//! **Under the render's own login and environment**, which L24b requires and
//! which this keeps by inheritance: the lookup is started exactly as
//! [`crate::acting::launch`] starts Blender — by this worker, with this
//! worker's environment, nothing added and nothing taken away — so the
//! library path the loader searches is the one Blender's own request would
//! search.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` M20.

use std::process::{Command, Stdio};

use smoozy_server_setup_machine::the_optix_library::{
    what_the_status_says, WhatTheLoaderFound, THE_LOADERS_OWN_LOOKUP,
};

/// **Asking whether the loader finds the OptiX library, behind a trait**, so
/// the driver's order can be run at a desk whose loader is not Linux's.
pub trait LooksForTheOptixLibrary {
    /// **L24b's lookup.** Asked only where the job names `OPTIX`.
    fn what_the_loader_finds(&self) -> WhatTheLoaderFound;
}

/// **The real end: the lookup run by a POSIX shell on the machine.**
///
/// **The line is a field rather than the constant read in place** so a test
/// can run this very code over a line whose answer it knows; on the machine
/// it is always [`THE_LOADERS_OWN_LOOKUP`], which
/// [`TheLoadersOwnLookupOnTheMachine::on_the_rented_machine`] is the one
/// route to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheLoadersOwnLookupOnTheMachine {
    line: &'static str,
}

impl TheLoadersOwnLookupOnTheMachine {
    /// The machine Smoozy rents.
    pub fn on_the_rented_machine() -> Self {
        TheLoadersOwnLookupOnTheMachine {
            line: THE_LOADERS_OWN_LOOKUP,
        }
    }
}

impl LooksForTheOptixLibrary for TheLoadersOwnLookupOnTheMachine {
    fn what_the_loader_finds(&self) -> WhatTheLoaderFound {
        // **Nothing held, nothing inherited on the way in**: the lookup reads
        // nothing from its input and says nothing worth keeping; only its
        // status is read.
        match Command::new("sh")
            .arg("-c")
            .arg(self.line)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
        {
            Ok(status) => what_the_status_says(status.code()),
            Err(why) => WhatTheLoaderFound::NoAnswer {
                why: format!("the lookup could not be started: {why}"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **This file's own code, run over lines whose status is known**, so the
    /// route from a status to an answer is the shell's and not this test's.
    #[test]
    fn the_real_end_reads_the_shells_status_and_nothing_else() {
        let over = |line: &'static str| TheLoadersOwnLookupOnTheMachine { line }.what_the_loader_finds();
        assert_eq!(over("exit 0"), WhatTheLoaderFound::TheLibrary);
        assert_eq!(over("exit 1"), WhatTheLoaderFound::NoLibrary);
        assert!(matches!(over("exit 2"), WhatTheLoaderFound::NoAnswer { .. }));
        assert_eq!(
            TheLoadersOwnLookupOnTheMachine::on_the_rented_machine().line,
            THE_LOADERS_OWN_LOOKUP
        );
    }
}

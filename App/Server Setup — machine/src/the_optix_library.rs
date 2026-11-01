//! **The OptiX library, and the one lookup for it** — R1 item 5's fifth
//! Ubuntu act, and `Rendering` L23 and `Rendering` L24b. Added 2026-10-03, with
//! the Creator's ruling that NVIDIA cards render with OptiX.
//!
//! **Why it is here and not in either of the two places that use it.** The
//! render worker looks for the library before a job naming `OPTIX` is
//! launched (`Rendering` L24b), and the fifth act of R1 item 5's Ubuntu route
//! looks for it before installing anything — and R1 item 5 says in terms that
//! the act looks *"by the same lookup `Rendering` L24b makes"*. Both crates
//! that do those two things take this one, so the lookup is written once and
//! the two cannot drift apart into two lookups that disagree about one
//! machine.
//!
//! **What is looked for.** NVIDIA's own components list for the Linux driver,
//! quoted at `Rendering` L23: *"The OptiX library (`/usr/lib/libnvoptix.so.1`);
//! This library implements the OptiX ray tracing engine."* The name is the
//! file's own and the folder in that quotation is never used: on a container
//! the host puts the driver's files wherever it puts them, which is why
//! `Rendering` L24b forbids a walk of folders anybody names.
//!
//! **How it is looked for: by the system's own loader, asked to load the
//! library by its name, under the environment of whoever runs the lookup.**
//! The loader's own publication, `ld.so(8)` as Ubuntu 24.04 ships it,
//! `https://manpages.ubuntu.com/manpages/noble/en/man8/ld.so.8.html`, read
//! **2026-10-03**:
//!
//! - **`LD_PRELOAD`**: *"A list of additional, user-specified, ELF shared
//!   objects to be loaded before all others."* … *"The objects are searched
//!   for using the rules given under DESCRIPTION."* — and those rules are the
//!   whole search a program's own request by name gets: *"If a shared object
//!   dependency does not contain a slash, then it is searched for in the
//!   following order"*, the binary's own paths, *"the environment variable
//!   LD_LIBRARY_PATH"*, *"the cache file /etc/ld.so.cache"*, then *"the
//!   default path /lib, and then /usr/lib"*. **So a name handed to the loader
//!   this way is found exactly where the loader would find it for Blender**,
//!   the environment's own library path included, and nowhere else.
//! - **`LD_TRACE_LOADED_OBJECTS`**: *"If set (to any value), causes the program
//!   to list its dynamic dependencies, as if run by ldd(1), instead of running
//!   normally."* **So nothing is run**: the program the loader is pointed at
//!   is listed and never started.
//!
//! **What the listing says, from the loader's own source** — glibc 2.39,
//! the version Ubuntu 24.04 carries, `elf/rtld.c` at
//! `https://sourceware.org/git/?p=glibc.git;a=blob_plain;f=elf/rtld.c;hb=refs/tags/glibc-2.39`,
//! read **2026-10-03**. Every object loaded is printed as
//! *`"\t%s => %s (0x%0*zx)\n"`*, the name it was asked for and the path it was
//! found at, and that list starts after the program itself and runs through
//! everything placed before it, the preloaded objects included. A preload that
//! cannot be found is not listed: it is reported on the complaints stream,
//! *"ERROR: ld.so: object '%s' from %s cannot be preloaded (%s): ignored."*,
//! and skipped. **So the library was found exactly where the listing carries
//! the line `libnvoptix.so.1 => /…`**, which is what [`THE_LOADERS_OWN_LOOKUP`]
//! asks `grep` for.
//!
//! **And the answer is read from an exit status, never from words.** POSIX
//! `grep`, `https://pubs.opengroup.org/onlinepubs/9699919799/utilities/grep.html`,
//! read **2026-10-03**: `-F` *"Match using fixed strings"*; `-q` *"Quiet.
//! Nothing shall be written to the standard output, regardless of matching
//! lines. Exit with zero status if an input line is selected."*; exit `0`
//! *"One or more lines were selected"*, `1` *"No lines were selected"*, `>1`
//! *"An error occurred."* And the POSIX shell,
//! `https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html`,
//! read the same day: a pipeline's *"exit status shall be the exit status of
//! the last command specified in the pipeline"*, and a command's leading
//! assignments *"shall be exported for the execution environment of the
//! command and shall not affect the current execution environment"* — so the
//! two loader settings reach the listed program and nothing after it.
//!
//! **What this lookup cannot see, stated rather than smoothed.** It finds the
//! file the loader would load; it does not prove the driver behind it works,
//! which is `Rendering` L24's refusal and `Rendering` L25's. **And it rests on the listed
//! program being one the system's loader starts**: `sh` is, on every Ubuntu
//! and Debian image this project has read. On a system whose `sh` the loader
//! does not start, nothing is listed and the lookup says *not found* — a
//! refusal of a machine that might have rendered, which is the direction
//! `Rendering` §9 item 3 names as what the first pass on each provider must
//! rule out.

/// **The file looked for**, by the name the loader is asked for — NVIDIA's own
/// components list, quoted at `Rendering` L23.
pub const THE_LIBRARY: &str = "libnvoptix.so.1";

/// **The one lookup, for any library named by a literal written here** — the
/// line [`THE_LOADERS_OWN_LOOKUP`] is, with the name swapped. **Shared, not
/// copied, from 2026-10-11**: `Server Setup` R1 item 4 asks the same loader
/// for a library Blender opens by name while it runs
/// ([`crate::ready_action::WHAT_BLENDER_OPENS_BY_NAME`]), and one line for both
/// is what keeps the two lookups from drifting apart.
///
/// **It takes a literal and nothing else**, so every line it makes is constant
/// entire, fixed when the program is built — never a name read off a machine.
macro_rules! the_loaders_own_lookup_for {
    ($library:literal) => {
        concat!(
            "LD_TRACE_LOADED_OBJECTS=1 LD_PRELOAD=",
            $library,
            " sh 2>/dev/null | grep -F -q \"",
            $library,
            " => /\""
        )
    };
}
pub(crate) use the_loaders_own_lookup_for;

/// **The lookup, as one line a POSIX shell runs.** It ends at `0` where the
/// system's loader finds [`THE_LIBRARY`] and at `1` where it does not; the
/// loader's own complaint about a library it could not find is not kept.
///
/// **The text holds no single quote**, because R1 item 5's fifth act carries it
/// inside one, and **nothing of any machine's, user's or provider's is in it**
/// — it is constant entire, which is what lets that act carry it under E7.
pub const THE_LOADERS_OWN_LOOKUP: &str = the_loaders_own_lookup_for!("libnvoptix.so.1");

/// **What the lookup answered.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheLoaderFound {
    /// The loader finds the library.
    TheLibrary,
    /// The loader does not find it.
    NoLibrary,
    /// **The lookup itself did not answer** — the shell could not be started,
    /// it ended with no status, or `grep` reported an error. Nothing is known
    /// about the library, and this is never read as either answer above.
    NoAnswer { why: String },
}

/// **The lookup's exit status, read** — `0` found, `1` not found, anything
/// else no answer (POSIX `grep`'s own three, quoted in this file's head).
pub fn what_the_status_says(status: Option<i32>) -> WhatTheLoaderFound {
    what_the_status_says_about(THE_LIBRARY, status)
}

/// **The same reading, for the lookup of any library** the shared line was
/// made for (`Server Setup` R1 item 4's, from 2026-10-11).
pub fn what_the_status_says_about(library: &str, status: Option<i32>) -> WhatTheLoaderFound {
    match status {
        Some(0) => WhatTheLoaderFound::TheLibrary,
        Some(1) => WhatTheLoaderFound::NoLibrary,
        Some(other) => WhatTheLoaderFound::NoAnswer {
            why: format!("the lookup for {library} ended with status {other}"),
        },
        None => WhatTheLoaderFound::NoAnswer {
            why: format!("the lookup for {library} ended with no status"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The lookup names the library it looks for, the two loader settings
    /// the publication names, and nothing a single quote could close.**
    #[test]
    fn the_lookup_asks_the_loader_for_the_library_by_its_name_alone() {
        assert!(THE_LOADERS_OWN_LOOKUP.contains(&format!("LD_PRELOAD={THE_LIBRARY} ")));
        assert!(THE_LOADERS_OWN_LOOKUP.starts_with("LD_TRACE_LOADED_OBJECTS=1 "));
        assert!(THE_LOADERS_OWN_LOOKUP.ends_with(&format!("\"{THE_LIBRARY} => /\"")));
        // **Never a folder anybody names** (`Rendering` L24b): the only `/` in
        // the line is the loader's own arrow's path and the discarded
        // complaints.
        assert_eq!(
            THE_LOADERS_OWN_LOOKUP.replace("2>/dev/null", "").matches('/').count(),
            1,
            "{THE_LOADERS_OWN_LOOKUP}"
        );
        assert!(!THE_LOADERS_OWN_LOOKUP.contains('\''));
        assert!(!THE_LOADERS_OWN_LOOKUP.contains('$'));
    }

    /// **Sharing the line moved none of its bytes** (2026-10-11): the OptiX
    /// lookup is the line R1 item 5's fifth act and `Rendering` L24b carried
    /// before the line was made for any library.
    #[test]
    fn the_shared_line_is_the_optix_line_it_always_was() {
        assert_eq!(
            THE_LOADERS_OWN_LOOKUP,
            "LD_TRACE_LOADED_OBJECTS=1 LD_PRELOAD=libnvoptix.so.1 sh 2>/dev/null | grep -F -q \"libnvoptix.so.1 => /\""
        );
        assert_eq!(
            the_loaders_own_lookup_for!("libEGL.so.1"),
            THE_LOADERS_OWN_LOOKUP.replace(THE_LIBRARY, "libEGL.so.1")
        );
    }

    /// **`grep`'s three answers, and only `0` and `1` are answers about the
    /// library.**
    #[test]
    fn only_a_clean_zero_or_one_says_anything_about_the_library() {
        assert_eq!(what_the_status_says(Some(0)), WhatTheLoaderFound::TheLibrary);
        assert_eq!(what_the_status_says(Some(1)), WhatTheLoaderFound::NoLibrary);
        for status in [Some(2), Some(127), None] {
            assert!(
                matches!(what_the_status_says(status), WhatTheLoaderFound::NoAnswer { .. }),
                "{status:?}"
            );
        }
    }

    /// **The lookup run in a real shell, over a stand-in loader listing**, so
    /// what `grep` selects is the shell's own answer and not this file's
    /// reasoning about it. The stand-in prints what glibc's listing prints in
    /// each case (this file's head) and the real shell runs the line.
    ///
    /// **The stand-in is a shell function named `sh`, defined ahead of the
    /// line, and never a program.** On Linux the line's two loader settings
    /// reach whatever program `sh` names, and the system's loader lists that
    /// program's interpreter instead of running it — the very behaviour the
    /// lookup rests on — so a stand-in program never ran there and every case
    /// read as not found (hosted run `37188697561`). A function runs inside the
    /// shell, so no program starts under the two settings, and it uses only
    /// the `[` and `printf` built into `dash` and `bash`: it answers the same
    /// under both, on Linux and on this Mac.
    #[cfg(unix)]
    #[test]
    fn the_lookup_selects_the_loaders_found_line_and_nothing_else() {
        let the_stand_in = format!(
            "sh() {{\n\
             [ \"$LD_PRELOAD\" = \"{THE_LIBRARY}\" ] || exit 9\n\
             [ -n \"$LD_TRACE_LOADED_OBJECTS\" ] || exit 9\n\
             printf '%s' \"$LISTING\"\n\
             printf 'ERROR: ld.so: noise on the complaints stream\\n' >&2\n\
             }}\n"
        );
        let the_lookup_over = |listing: &str| -> Option<i32> {
            std::process::Command::new("/bin/sh")
                .arg("-c")
                .arg(format!("{the_stand_in}{THE_LOADERS_OWN_LOOKUP}"))
                .env("LISTING", listing)
                .stdin(std::process::Stdio::null())
                .output()
                .expect("the shell ran")
                .status
                .code()
        };

        // **Found**: the loader's own line for a preloaded object.
        let found = the_lookup_over(
            "\tlinux-vdso.so.1 (0x00007ffd)\n\
             \tlibnvoptix.so.1 => /usr/lib/x86_64-linux-gnu/libnvoptix.so.1 (0x00007f00)\n\
             \tlibc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f01)\n",
        );
        assert_eq!(what_the_status_says(found), WhatTheLoaderFound::TheLibrary);

        // **Not found**: the preload is reported on the complaints stream and
        // left out of the listing.
        let missing = the_lookup_over(
            "\tlinux-vdso.so.1 (0x00007ffd)\n\
             \tlibc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f01)\n",
        );
        assert_eq!(what_the_status_says(missing), WhatTheLoaderFound::NoLibrary);

        // **A dependency of the program reported missing is not the library
        // found**, even where its name is the library's own.
        let named_and_missing = the_lookup_over("\tlibnvoptix.so.1 => not found\n");
        assert_eq!(what_the_status_says(named_and_missing), WhatTheLoaderFound::NoLibrary);
    }

    /// **On Linux, the line run by the system's own loader**, which no
    /// stand-in can be: the same line with the library's name swapped for one
    /// that is on every Ubuntu machine and one that is on none. `libm.so.6` is
    /// glibc's own — Ubuntu's file list for `libc6` in `noble`,
    /// `https://packages.ubuntu.com/noble/amd64/libc6/filelist`, read
    /// **2026-10-04**, carries `/usr/lib/x86_64-linux-gnu/libm.so.6` — and the
    /// line's `sh` does not need it, so its line in the listing is there only
    /// because it was preloaded. **This is the one test that shows the
    /// lookup's mechanism on a real Linux loader** rather than on a stand-in.
    #[cfg(target_os = "linux")]
    #[test]
    fn on_linux_the_systems_own_loader_lists_a_preloaded_library_and_the_line_reads_it() {
        let the_line_for = |library: &str| -> Option<i32> {
            std::process::Command::new("/bin/sh")
                .arg("-c")
                .arg(THE_LOADERS_OWN_LOOKUP.replace(THE_LIBRARY, library))
                .stdin(std::process::Stdio::null())
                .output()
                .expect("the shell ran")
                .status
                .code()
        };
        assert_eq!(
            what_the_status_says(the_line_for("libm.so.6")),
            WhatTheLoaderFound::TheLibrary
        );
        assert_eq!(
            what_the_status_says(the_line_for("libsmoozy-no-such-library.so.1")),
            WhatTheLoaderFound::NoLibrary
        );
    }
}

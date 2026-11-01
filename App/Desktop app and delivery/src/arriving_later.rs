//! What a sitting checked against this crate's plan, found owed, and could not
//! build yet — with what removes each one.
//!
//! **READ THIS PARAGRAPH BEFORE THE LIST, BECAUSE THE PAGE IS NARROWER THAN
//! ITS NAME.** `App/File Browser/`, `App/Server protocol and transfers/` and
//! `App/Job files and outputs/` each carry a file of this name holding **the
//! whole truth about that crate's absences** — every clause of its plan that
//! working code does not name, guarded by a forward-leg test. **This page is
//! not that, and this crate does not have one.** Whether it should was ruled on
//! 2026-09-06 under the Creator's standing delegation of 2026-08-31, and the
//! answer was no; the reasoning is written at
//! `crate::markers`'s clause-count test and is not repeated here. The short of
//! it: this plan is the largest in the project and covers the installer, the
//! update mechanism, the presentation surface and the whole main window, **none
//! of which any build step has reached**, so a complete forward leg today is
//! nearly three hundred entries that nobody can keep true — and a page nobody
//! can keep true is worse than no page, because a reader cannot tell it from
//! one that is.
//!
//! **So what this page holds is one thing only: an absence somebody went and
//! looked for, found, and wrote down with what removes it.** An entry here
//! means a clause was read against the code on a named day and found owed. **No
//! entry here means nothing at all about any other clause of this plan**, and a
//! reader who takes this list as complete has read it wrongly. What answers
//! *is clause X built* is the build gate's forward leg, run over the module by
//! a fresh agent, which `Preparation/Build/Method.md` already provides.
//!
//! **What guards the list is [`tests`]**: every clause named here is a clause
//! the plan actually defines, and the condition each entry rests on still
//! holds. **An entry whose condition has gone is an entry whose work has
//! arrived**, and the test says so rather than leaving the page to be believed.

/// One clause of this plan that no working file of this crate names, with what
/// removes it.
///
/// **The shape is `App/File Browser/src/arriving_later.rs`'s**, carried here
/// unchanged, so a reader who has seen one of these pages has seen this one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotNamedByWorkingCode {
    /// The plan's own top-level section.
    pub section: &'static str,
    /// Every clause of that section this covers.
    pub clauses: &'static [&'static str],
    /// What the clause is.
    pub what: &'static str,
    /// Why no working file of this crate names it.
    pub why: &'static str,
    /// **What removes it** — the work that makes the clause true here, named
    /// exactly enough that a reader can go and find it.
    pub what_removes_it: &'static str,
}

/// **The Linux limb of the release. Added 2026-09-06, the day E4a was
/// written.**
///
/// E4a re-derived the Linux package's declared runtime libraries from the
/// selected shell's own publication, which is what E4's own sentence had
/// promised for four days and nobody had done. **The answer did not move** —
/// the pair is `libgtk-3-0` and `libwebkit2gtk-4.1-0`, the same GTK3 and
/// WebKitGTK 4.1 pair every other reading in the plan turns on — **and the
/// code owes it a confirmation that cannot be made yet.**
pub const OWED_AT_THE_FIRST_LINUX_RELEASE: [NotNamedByWorkingCode; 1] = [NotNamedByWorkingCode {
    section: "§19.1",
    clauses: &["E4a"],
    what: "the produced Linux package declares both runtime libraries the selected shell's \
           bundler publishes — `libgtk-3-0` and `libwebkit2gtk-4.1-0` — and nothing in the \
           bundle configuration drops or overrides them. **E4's whole Linux argument rests on \
           it, and Row 282's Linux limb with it**: the distribution's package transaction \
           satisfies the declared pair or refuses, before there is anything to launch. A \
           package declaring the wrong runtime libraries is what E4 itself names as precisely \
           the failure its shape exists to prevent",
    why: "**the package is now produced, and its reading is written, but it has never run.** \
          Step 11's artifacts piece (2026-10-24) gave the hosted Linux job a Debian package per \
          E4b row and a step that reads the package's own `Depends` back and refuses one \
          that lacks `libc6 (>= 2.39)`, `libgtk-3-0t64` or `libwebkit2gtk-4.1-0` \
          (`.github/workflows/desktop-build.yml`). **E4b, read off the distributions' own \
          pages, names the GTK package `libgtk-3-0t64` on every row, where E4a's bundler \
          default writes `libgtk-3-0`**; whether a package declaring the old name installs \
          is not established. **No hosted run has happened since, so nothing has been \
          confirmed** — and hosted runs are the Creator's call",
    what_removes_it: "**the first hosted run on which that reading passes.** Its log shows each \
                      row's package declaring the three, and the package installing on that \
                      row. Then this entry goes, and the test below with it",
}];

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;

    const THE_PLAN: &str = include_str!("../../../Preparation/Architecture/Desktop app and delivery.md");

    /// **Every clause this page names is a clause the plan defines.**
    ///
    /// An entry naming a clause that does not exist records an absence nobody
    /// can ever remove, which is the worst of both states: it reads as owed
    /// work and there is none.
    #[test]
    fn every_clause_on_this_page_is_one_the_plan_defines() {
        for entry in OWED_AT_THE_FIRST_LINUX_RELEASE {
            for clause in entry.clauses {
                assert!(
                    THE_PLAN.contains(&format!("**{clause}.")),
                    "{clause} stands on this page and the plan defines no such clause"
                );
            }
        }
    }

    /// **The condition the one entry rests on still holds: the hosted Linux
    /// job still reads the produced package's declared dependencies back.**
    ///
    /// **Changed 2026-10-24, by the build runner, after step 11's artifacts piece.**
    /// This test used to fail the moment a Linux package was configured, which
    /// was its trigger; that piece configured one and wrote the reading beside the
    /// release step, as the entry asked. What the entry now waits on is a
    /// hosted run, which a test cannot see, so the test guards the reading
    /// itself: if it is ever taken out, the entry would be standing on nothing.
    #[test]
    fn the_hosted_linux_job_reads_the_package_back_which_is_why_the_entry_stands() {
        let workflow = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.github/workflows/desktop-build.yml"),
        )
        .expect("the release workflow is readable");
        for needed in ["dpkg-deb", "Depends", "libgtk-3-0t64", "libwebkit2gtk-4.1-0"] {
            assert!(
                workflow.contains(needed),
                "the release workflow no longer names {needed}, so nothing reads the \
                 produced Linux package back and the entry above stands on nothing — \
                 Desktop app and delivery E4a and E4b"
            );
        }
    }

}

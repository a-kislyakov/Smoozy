//! What the four re-readings of 2026-09-06 owe the code where the answer is a
//! standing property of the workspace rather than a piece of behaviour.
//!
//! **Why there is a file for this.** On 2026-09-02 the shell measurement chose
//! Tauri v2, and four sentences in this crate's plan had promised to be
//! re-read against whatever was chosen. They were written on 2026-09-06 as
//! E4a, H10a, H16a and H31a. **Three of the four settle a property of the
//! shipped configuration or of the workspace's own pin, and nothing named
//! them**, which is the reverse-gate leg's own case: a promise kept by
//! something nobody can find again. This file names them.
//!
//! **H31a is not here.** It is behaviour installed at the window, and it lives
//! at [`crate::web_content_boundary`] with the closures it holds.
//!
//! **E4a is not here either, and that is deliberate rather than an omission.**
//! Its Linux limb is a property of a package no build step produces yet, and
//! **the check that would keep it cannot be run before there is a package to
//! run it against**. A guard written today would either prove nothing or fight
//! the first correct Linux release. It stands on [`crate::arriving_later`]
//! with what removes it.
//!
//! The Windows line-ending guard from hosted run 34759561814 also lives here.
//! The workflow keeps the desktop and machine artifacts; the retired Worker's
//! build and its disk-clearing steps are absent.

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    /// The workspace's own manifest and lock, which are where a shell version
    /// and a shell facility actually show up.
    ///
    /// **Read, never written.** They are the whole workspace's and this crate
    /// is one member of it.
    fn workspace_file(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join(name)
    }

    /// The shipped configuration of this application, as text.
    fn the_configuration() -> String {
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("tauri.conf.json"))
            .expect("this application's configuration is readable")
    }

    /// **H10a: the shell is still the release H10a's reading was taken on.**
    ///
    /// H10a establishes, from that project's own publications, that the newest
    /// published release is **2.11.5** and that no next major version has
    /// shipped. **What it is worth to the code is one thing and H10a names it
    /// itself:** the open work list for a next major version carries *migrate
    /// to GTK4 and WebKitGTK 6.0*, which is a move off exactly the pair E4
    /// declares as the Linux package's runtime dependencies and E4a re-derived
    /// on 2026-09-06.
    ///
    /// **So a bump of this pin is not an ordinary maintenance act.** It moves
    /// the release H10a is measured on, and past a major version it moves
    /// E4a's pair and E4's release-matrix row with it. This test is what makes
    /// that a stop rather than a discovery, and it is the only place in this
    /// crate that says so.
    ///
    /// **It reads the pin rather than the built version** — the exact string
    /// `Preparation/Build/Method.md` requires — so loosening the pin fails it
    /// as surely as changing the number.
    #[test]
    fn the_shell_is_still_pinned_to_the_release_the_readings_were_taken_on() {
        let manifest = fs::read_to_string(workspace_file("Cargo.toml"))
            .expect("the workspace manifest is readable");
        assert!(
            manifest.contains("tauri = { version = \"=2.11.5\""),
            "the shell is no longer pinned to =2.11.5, which is the release H10a's \
             reading of the project's own publications was taken on and the one \
             H5b's measurement built its artifact from. A next major version \
             carries an open item to move off the GTK and WebKitGTK pair E4a \
             declares, so this bump moves E4's release-matrix row too — \
             Desktop app and delivery H10a, E4a and E4"
        );
    }

    /// **H16a, Row 280's leg: Smoozy uses none of the shell's updater.**
    ///
    /// H8's ground was that the shell that did not win documents no self-update
    /// anywhere. **H16a records that this is false of the shell that was
    /// chosen**, which publishes one — and that it changes nothing, because
    /// Section 20 is written against Apple's, Microsoft's and the kernel's own
    /// documented behaviour and calls no framework facility on any of the
    /// three. **The argument only holds while that stays true**, and the way it
    /// would stop being true is somebody reaching for the published updater
    /// because it is there.
    ///
    /// **This reads the whole workspace's lock rather than this crate's
    /// manifest**, because the facility would keep Row 280 wherever in the
    /// workspace it was taken, and because a lock names what is actually in
    /// the tree rather than what one member asked for.
    #[test]
    fn no_part_of_this_workspace_takes_the_shells_own_updater() {
        let lock =
            fs::read_to_string(workspace_file("Cargo.lock")).expect("the lock file is readable");
        assert!(
            !lock.contains("tauri-plugin-updater"),
            "the shell's own updater is in the workspace tree. Section 20 keeps \
             Row 280 through each operating system's own replacement mechanism \
             and calls no framework facility, which is what H8, H9 and H16a all \
             turn on — Desktop app and delivery H16a"
        );
    }

    /// **H16a, Row 282's Windows leg: the ruled web-view install mode is the
    /// one the configuration carries.**
    ///
    /// The Creator ruled on 2026-08-23 that the cost of an embedded
    /// bootstrapper is taken — *"150 KB and you're bothering to ask me? of
    /// course it's okay."* — and H13a records that what they ruled is that the
    /// cost is taken, not which option name carries it. On the selected shell
    /// the option name is `embedBootstrapper`, settled at Section 22 item 65,
    /// and H16a's Windows leg rests on it.
    ///
    /// **The other three published values each break the ruling in a different
    /// direction** and are refused by name so that a swap is a failure rather
    /// than a silent change: `skip` leaves a Windows machine with no runtime
    /// meeting a Smoozy that will not start, which is the case the ruling was
    /// put on; `downloadBootstrapper` is the value the comparison artifact
    /// used and is not the ruled one; `offlineInstaller` carries the whole
    /// runtime at about 127 MB, which is the figure H13 declined.
    #[test]
    fn the_windows_web_view_step_is_the_value_the_ruling_carries() {
        let configuration = the_configuration();
        assert!(
            configuration.contains("\"webviewInstallMode\""),
            "this application's configuration names no Windows web-view install \
             mode, so the shell's own default decides what a Windows machine with \
             no suitable runtime meets — Desktop app and delivery H13a and H16a, \
             Section 22 item 65"
        );
        assert!(
            configuration.contains("\"embedBootstrapper\""),
            "the Windows web-view install mode is not `embedBootstrapper`, which \
             is the value the Creator's ruling of 2026-08-23 lands on — \
             Desktop app and delivery H13a and H16a, Section 22 item 65"
        );
        for refused in ["\"skip\"", "\"downloadBootstrapper\"", "\"offlineInstaller\""] {
            assert!(
                !configuration.contains(refused),
                "the configuration carries {refused} as a web-view install mode: \
                 the ruled value is `embedBootstrapper` and none of the other \
                 three keeps the ruling — Desktop app and delivery H13a and H16a, \
                 Section 22 item 65"
            );
        }
    }

    /// The top of the repository, which is where git is asked and where the
    /// hosted workflow lives.
    fn the_repository() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    /// Asks git, from the top of the repository, and returns what it printed.
    /// **A git that cannot be asked is a failure, never a skip**: a guard that
    /// quietly passes when it cannot look is no guard.
    fn git(arguments: &[&str]) -> String {
        let output = std::process::Command::new("git")
            .arg("-C")
            .arg(the_repository())
            .args(arguments)
            .output()
            .expect("git can be run, and this guard cannot look without it");
        assert!(
            output.status.success(),
            "git {arguments:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).expect("git prints text")
    }

    /// **A Windows checkout gives the tests the same bytes the Mac does.**
    ///
    /// Dozens of tests in this workspace read source files, page files, the
    /// mockup and the plans as text and look for patterns that span a line
    /// ending — a line holding only a closing brace, or `#[cfg(test)]` followed
    /// by the next line. **The hosted Windows runner checks text out with
    /// Windows line endings unless the repository says otherwise**, and on run
    /// 34759561814 eight tests went red on Windows for that reason alone. The
    /// repository now says otherwise, in `.gitattributes`, and this is what
    /// holds it.
    ///
    /// **It asks git rather than reading `.gitattributes`**, so a later, more
    /// particular rule that undoes this for some files fails here too. Two
    /// questions. **Every text file under `App/`, `Preparation/` and
    /// `.github/`, plus the root `COPYING` licence embedded in About, must carry
    /// `eol=lf` and be stored with plain endings**, since a file stored with
    /// Windows endings would come out that way whatever the rule says. **And a
    /// checkout made the way the Windows runner makes one** — git's own
    /// `core.autocrlf=true`, applied by git's own checkout conversion — must
    /// give plain endings for the files the red tests read.
    ///
    /// **`.github/` was added after run 34767216812**, where the workflow guard
    /// below was the one test red on Windows: it reads the workflow as text,
    /// and the first rule did not reach that folder.
    ///
    /// The account is `Audits/Build/Step 10a/The red run of 2026-09-13 — what
    /// it was.md`.
    #[test]
    fn a_windows_checkout_gives_every_text_file_plain_line_endings() {
        let listing = git(&["ls-files", "--eol", "--", "App", "Preparation", ".github", "COPYING"]);
        let mut faults = Vec::new();
        let mut looked_at = 0;
        for line in listing.lines() {
            let (flags, path) = line.split_once('\t').expect("git names a path");
            if flags.starts_with("i/-text") {
                continue;
            }
            looked_at += 1;
            if !flags.contains("eol=lf") || flags.starts_with("i/crlf") || flags.starts_with("i/mixed")
            {
                faults.push(format!("{flags} {path}"));
            }
        }
        assert!(looked_at > 1000, "git listed only {looked_at} text files");
        assert!(
            faults.is_empty(),
            "these text files would be checked out on Windows with Windows line \
             endings, and the tests that read them as text would see different \
             bytes there than at this desk: {faults:#?}"
        );

        for path in [
            "COPYING",
            "App/Desktop app and delivery/src/surface/a_server.rs",
            "App/Desktop app and delivery/src/surface/files.rs",
            "App/Desktop app and delivery/web/a-server.js",
            "Preparation/Design/Mockup/Smoozy.html",
            ".github/workflows/desktop-build.yml",
        ] {
            let checked_out = git(&[
                "-c",
                "core.autocrlf=true",
                "cat-file",
                "--filters",
                &format!(":{path}"),
            ]);
            assert!(
                !checked_out.contains('\r'),
                "a checkout made the way the Windows runner makes one gives {path} \
                 Windows line endings"
            );
        }
    }

    #[test]
    fn the_workflow_keeps_the_applications_without_building_the_retired_worker() {
        let workflow = fs::read_to_string(the_repository().join(".github/workflows/desktop-build.yml"))
            .expect("the hosted workflow is readable");
        let at = |name: &str| workflow.find(&format!("- name: {name}\n"))
            .unwrap_or_else(|| panic!("the workflow has no step named {name:?}"));
        let order = [
            "The application's tests",
            "Build the application",
            "Keep the desktop application",
            "The machine's own program, and what a manifest would say about it",
            "Keep the machine's own program",
        ];
        for pair in order.windows(2) {
            assert!(at(pair[0]) < at(pair[1]), "the artifact build and keep order changed");
        }
        for retired in ["Room for the licence service's Worker", "The licence service's Worker", "worker-build", "wasm32-unknown-unknown", "wrangler"] {
            assert!(!workflow.contains(retired), "the workflow still contains {retired}");
        }
    }
}

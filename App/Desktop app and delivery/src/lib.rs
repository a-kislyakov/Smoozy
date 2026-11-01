// The Smoozy desktop application's own parts, as a library beside the binary.
//
// **Why there is a library at all, and it is one reason.** `Keys` G3's askpass
// route names **Smoozy's own binary** as the helper (`Desktop app and delivery`
// §9 S7), and only an integration test is handed the built application's path
// — a unit test naming itself would have `ssh-keygen` re-enter the test
// harness. So the one test that drives a **generation with a passphrase**
// through the real helper has to be an integration test, and an integration
// test can only reach `generate` through a library target.
//
// **Nothing else moves.** The window, the shell's commands and the helper
// branch stay in `main.rs`, which is where `tauri::generate_context!` and the
// application's own entry point belong; every module below is what it was.

// **Why the whole-crate allowance, and when it comes off.** This module is the
// platform adapters, the local record and the diary service — `Desktop app and
// delivery` §5, §6 and §9 — and **every one of them exists to be called by
// feature modules that are built at steps 3 to 11** of
// `Preparation/Build/Plan.md`. Until those arrive, most of the adapter is
// reached only by its own tests, and the compiler says so a hundred times over,
// which buries anything worth reading. **It is removed when the first feature
// module lands**, and what stands in for it until then is that every public
// item here names the clause it keeps and is exercised by a test.
//
// **Measured again 2026-09-03, at step 3's second sitting, and the end
// condition was wrong rather than unmet.** `Keys`' acting half now calls the
// file adapter, the readings and the local record — which is what this line
// said would remove the allowance — and taking it off produced **109**
// warnings against the 77 of that morning. The cause is that the act itself
// has no caller: nothing in this file opens a make-key dialog yet, so the act
// and everything it reaches are unused too. **What actually removes this is
// the application's own screens calling the acts**, and until then the
// standing-in rule above is unchanged.
#![allow(dead_code)]

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. `markers.rs` states what it holds and what it cannot
// see.
#[cfg(test)]
mod markers;

// Test folders and files in the system's temporary directory, removed when the
// test program ends. Test-only; `scratch_folders.rs` says why.
#[cfg(test)]
mod scratch_folders;

// **A new job from an old one** — *Submit again* and *Carry on* from a
// history entry to the Render Settings page, and the pick a re-submission
// waits on (`Rendering` §6). Built at the 38's piece 2.
pub mod a_new_job_from_an_old_one;
pub mod arriving_later;
pub mod diary_service;
// **The first caller of the two rate budgets and the one desktop-side
// admission point, and the first reader of what Settings saves** —
// `Desktop app and delivery` §11 and §8, with `Download` H2 beneath them. Built
// at step 10b piece 7.
pub mod downloading;
// **The one ordered queue every result travels on to the interface** —
// `Desktop app and delivery` §3.3. Built at step 10a piece A, 2026-09-08, and
// it is the first thing in this application that can put anything on a screen
// of its own accord.
pub mod events;
pub mod keys;
// **The four menus, the keys beside their items, the system file picker and
// the folder-reveal service** — `Desktop app and delivery` §18, built at step
// 10a piece H, 2026-09-08. The picker and the reveal are named services of §21
// and A1 admits no other route to either; the menus are the integration K5 to
// K10 describe and add no item of their own.
pub mod menus;
// **Check dependencies…, driven** — `Dependency Upload & Remap` §3.4's loop over
// `Server protocol and transfers` T5, §4's look and search, the decisions saved
// and §7.3's standing written. Built at step 10c's dependency wave piece 5.
pub mod checking_a_file;
// **The found files sent up** — `Dependency Upload & Remap` §5's uploads, K7's
// placement records and V13's marks at publication. Built at step 10c's
// dependency wave piece 6.
pub mod the_found_files_sent;
// **The four kinds of notification** — `Desktop app and delivery` §13, built
// at step 10a piece G, 2026-09-09. It is a named service of §21 and A1 admits
// no other route to the operating system's notifications; its three per-system
// files are platform code of this document's own, beside the shell, the same
// shape `reveal` takes.
pub mod notifications;
// **The one shape a failure is drawn in** — `Desktop app and delivery` §16,
// built at step 10a piece C, 2026-09-09. **Its head is the shape rule pieces D,
// E and F follow**, and the eleven screens after them.
pub mod presentation;
pub mod picker;
// **`File Browser` §7's acts, done on the machine** — a new folder, a rename,
// a delete, cut, copy and paste. Built at step 10c's Files' owed acts.
pub mod managing_files;
pub mod platform;
// **Read the file on Render Settings, driven over the server's own point** —
// `Render Settings` Q4's Pre-load through `Server protocol and transfers` T5.
// Built at step 10c's dependency wave piece 3.
pub mod reading_the_file;
pub mod reveal;
pub mod the_address_opener;
pub mod record;
pub mod scheduling;
// **The desktop's real ends of `Servers`' seams**, built at step 10b. The
// first of them is the installation identity read, whose seam that module
// declared with no implementation outside its own tests.
pub mod servers;
// **The Set up screen's whole act, from the first reading to the run's last
// step** — `Server Setup` R2's order over `Server protocol and transfers` T5's
// per-server point. Built at step 10c piece 9, beside `submitting`.
pub mod setting_up;
pub mod start;
// **The Submit button's whole act, from the page's composition to the
// machine** — `Rendering`'s unit joined to `Server protocol and transfers` T5's
// per-server point. Built at step 10c piece 6, beside `uploading`.
pub mod submitting;
pub mod submission_cleanup;
// **A server program built at this desk, placed in place of the current
// release** — `Server protocol and transfers` B10a. **Compiled only with debug
// assertions and reached by no Cargo feature**: a release build has no route to
// it at all. Built at step 10c, fix piece 1.
#[cfg(debug_assertions)]
pub mod the_desk_program;
// **`Queue & History` §15's desktop half** — the preview fetch on the
// journalled event, the copies held on the job record, the age sweep and the
// picture's bytes for the page. Built at the 38's piece 3.
pub mod the_previews;
// **The command surface: one file per service, one registration list.** Its
// head is the layout rule the seven pieces of step 10a after A follow, and the
// eleven screens after them.
pub mod surface;
// The three re-readings of 2026-09-06 that settle a standing property of the
// shipped configuration or of the workspace's pin — test-only, because what
// they hold is a property of files rather than a piece of behaviour.
#[cfg(test)]
mod the_selected_shell;
// **The store standing's one writer** — `Security and licence` S32's (§3.3)
// durable value and this crate's own S5's probe, taken once in a run at the
// first moment a secret is actually needed. Built at step 10b piece 3a, when
// piece 2 found that nothing in the application had ever taken the probe.
// **One running copy per user** — `Desktop app and delivery` P2: the hold an
// ordinary launch takes before anything else, and the second launch's way to
// the first. Built at step 11's R1.
pub mod the_one_instance;
pub mod the_last_line;
// **What a reachable machine adds to a report: the per-job note and the
// journal's tail** — `Reporting a problem` §6, taken at the press of *Save the
// report*. Built at step 11's piece R3a.
pub mod the_report_take;
pub mod the_store_standing;
pub mod transfers;
pub mod typography;
// **The desktop's own update file, read and trusted** — `Desktop app and
// delivery` E15, E16's digest and length, E19 and E10a: the manifest, the trust
// set from the recovery keys to the online keys, and the reading in order.
// Built at step 11's update-reading piece; it carries no recovery key until
// the Creator holds them.
pub mod updating;
// **An upload, run from the files the user picked to published bytes on the
// machine** — `File Browser` §3 to §5 over this crate's §11. Built at step 10c
// piece 4, beside `downloading` and in its shape.
pub mod uploading;
pub mod web_content_boundary;

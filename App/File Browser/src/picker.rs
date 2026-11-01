//! The picker — `File Browser` §10, H1 to H4 with H3a, Rows 307 and 341.
//!
//! **Constraint:** `Preparation/Design/File Browser.md`, whose unfinished-spot
//! marker for Row 307 was cleared on 2026-08-23 and replaced by the fallback it
//! now states outright — the marker's own spelling is not repeated here, so a
//! sweep of the project for open ones does not stop at a closed one — *"**Where the system file picker cannot take files and
//! folders in one go, the same Upload control asks which kind first.** That is
//! one control either way: the fallback is a question inside Upload, never a
//! second control beside it."*
//!
//! **The picker itself is `Desktop app and delivery` 18.4's**, which says
//! outright that Row 307 is not answered there. **What is answered here is
//! whether one dialog can take both kinds at once, per system**, and nothing
//! about the plumbing.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Desktop app and delivery` 18.4, H10, §22.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause.

/// The three systems the release matrix names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum System {
    Windows,
    /// **The portal route**, which is the one H2 answers.
    LinuxOnThePortalRoute,
    MacOs,
}

impl System {
    pub const ALL: [System; 3] = [
        System::Windows,
        System::LinuxOnThePortalRoute,
        System::MacOs,
    ];
}

/// Whether one system dialog can take files and folders together.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OneDialogTakesBoth {
    /// **H1 and H2: no, in the publication's own words.**
    No,
    /// **H3: yes, and it is observed rather than inferred.** The two switches
    /// are documented one page each; **that one panel takes both was put up and
    /// watched on 2026-09-04** and is labelled *observed in a controlled test*.
    /// *(This variant was `InferredYes` until then, §13 item 8 being open.)*
    ObservedYes,
}

/// **Row 307's answer, per system.**
///
/// - **H1. Windows: no, in Microsoft's own words.** The file dialog's folder
///   mode is a mode, not a permission: `FOS_PICKFOLDERS` — *"Present an Open
///   dialog that offers a choice of folders rather than files."* Documented,
///   Microsoft's `FILEOPENDIALOGOPTIONS` reference, read 2026-08-20. **One
///   dialog offers folders *rather than* files; nothing on the page offers both
///   at once.**
/// - **H2. Linux, on the portal route: no, in the specification's own words.**
///   The FileChooser portal's `OpenFile` takes a `directory` option — *"Whether
///   to select for folders instead of files. Default is to select files."*
///   Documented, the xdg-desktop-portal FileChooser specification, read
///   2026-08-20. ***Instead of**, a two-way switch again.*
/// - **H3. macOS: yes, and it is observed rather than inferred.**
///   `canChooseFiles` — *"A Boolean that indicates whether the user can choose
///   files in the panel"* — and `canChooseDirectories` — the same sentence for
///   directories — are two properties of one open panel, each documented,
///   **with nothing on either page excluding the other**. Documented for each
///   switch; **the combination is observed in a controlled test**, run on this
///   Mac on **2026-09-04**, which is the one-minute check §13 item 8 asked for
///   and which closed it. **What was seen:** a live panel with both switches
///   set keeps both — neither is reset and nothing refuses — and the panel's
///   own consideration of the items in front of it is the **union** of the two
///   single-switch runs. **What the test did not do is press Open**, so what
///   stands on it is that the combination is accepted, not what a mixed
///   selection returns. The run, the version and the limits are in
///   `Preparation/Architecture/Sources/File Browser.sources.md` I1a, which is
///   an entry of that file and not a clause of any plan.
pub fn one_dialog_takes_both(system: System) -> OneDialogTakesBoth {
    match system {
        System::Windows | System::LinuxOnThePortalRoute => OneDialogTakesBoth::No,
        System::MacOs => OneDialogTakesBoth::ObservedYes,
    }
}

/// **H4: what the negative costs, in one sentence.**
///
/// On Windows and on the Linux portal route one control cannot take files and
/// folders together, **so the design's own stated fallback fires there — the
/// same Upload control asking which kind first, which it settles is never a
/// second control beside it — and no promise moves.**
///
/// Whatever a picker returns is judged like any other name before anything is
/// written (`Desktop app and delivery` K13).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheUploadControl {
    /// One dialog, both kinds.
    OpensOnePickerForBoth,
    /// **A question inside Upload, never a second control beside it.**
    AsksWhichKindFirst,
}

pub fn the_upload_control(system: System) -> TheUploadControl {
    match one_dialog_takes_both(system) {
        OneDialogTakesBoth::ObservedYes => TheUploadControl::OpensOnePickerForBoth,
        OneDialogTakesBoth::No => TheUploadControl::AsksWhichKindFirst,
    }
}

/// **H4 used to end *dragging onto the list, which Design already carries,
/// takes any mix everywhere*. That half was struck from the plan on 2026-09-04
/// and H3a re-based onto the shell Build actually chose.**
///
/// **H3a rested on Fact 120, which is the Wails documentation**, quoting
/// *"EnableFileDrop … dragging and dropping **files and or folders** in to the
/// window"*. **The shell chosen at Build step 0 is Tauri v2**, so that
/// quotation is about a shell this application does not use. The reading below
/// was taken on **2026-09-04** and carried up into the plan: H3a now rests on
/// it, H4 no longer asserts *any mix everywhere*, and what the chosen shell
/// does not say is **§13 item 19**.
///
/// **What it does say.** Tauri v2's JavaScript API reference for
/// `WebviewWindow.onDragDropEvent`, read 2026-09-04 at
/// `https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/`:
/// *"Listen to a file drop event. The listener is triggered when the user
/// hovers the selected files on the webview, drops the files or cancels the
/// operation."* — and the payload the page's own example reads is
/// `event.payload.paths`. **So H3a's *what arrives is paths and nothing else*
/// holds on this shell too**: no sizes, no counts, no walk.
///
/// **What it does not say is whether a folder may be dropped.** The page speaks
/// of files throughout and states nothing about folders either way. **So *any
/// mix everywhere* is unsupported on the chosen shell** — not contradicted,
/// unstated — **and the plan now says so at §13 item 19 rather than asserting
/// it at H4.** **What holds is H4's own answer**, which never rested on the drop:
/// the fallback is a question inside the one Upload control, Design states it
/// outright, and it covers both kinds on every system whatever a drop carries.
/// **What is lost is only the comfort** that the drop makes the Windows and
/// Linux negative cost nothing.
///
/// **The other half of the reading, which a build can get wrong.** Tauri's own
/// `WebviewBuilder::disable_drag_drop_handler`, read 2026-09-04 at
/// `https://docs.rs/tauri/2.11.5/tauri/webview/struct.WebviewBuilder.html`:
/// *"Disables the drag and drop handler. This is required to use HTML5 drag and
/// drop APIs on the frontend on Windows."* **So the shell's own handler and the
/// page's own HTML5 drag and drop are alternatives rather than companions on
/// Windows**, and a build that wants the paths keeps the handler enabled. That
/// is the same shape as the hazard H3a's withdrawn shell named with a separate
/// switch, met on the chosen shell, and it is `Desktop app and delivery`'s to
/// set rather than this document's to compose. **It is now H3a's own fifth
/// bullet**, carried up with the rest of the reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatADropCarries {
    /// **Paths, and nothing else** — documented for the chosen shell.
    PathsAndNothingElse,
}

/// **H3a: nothing about what is inside a dropped folder is known at the drop,
/// so a count shown at that moment is a count of *what was dropped*.**
///
/// **The enumeration is X1's and runs after the user has confirmed the upload,
/// exactly where it already is; nothing moves earlier.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheCountAtTheDrop(pub usize);

impl TheCountAtTheDrop {
    /// What was dropped, and never what is inside it.
    pub fn of(paths_dropped: usize) -> TheCountAtTheDrop {
        TheCountAtTheDrop(paths_dropped)
    }
}

/// **H3a's own statement of what the drop does not settle**, kept as a value so
/// the gap is met rather than discovered.
pub const THE_ENUMERATION_RUNS_AFTER_THE_UPLOAD_IS_CONFIRMED_AND_NOTHING_MOVES_EARLIER: bool = true;

/// **Whether a folder may be dropped is unstated by the chosen shell — §13
/// item 19.**
///
/// Written as a value rather than a comment because the struck half of H4 was a
/// claim about all three systems and the chosen shell's publication does not
/// carry it. **No space is left for an answer that is coming**: what is here is
/// the reading as taken, and the fallback that holds without it.
pub const WHETHER_A_DROPPED_FOLDER_IS_CARRIED_IS_UNSTATED_BY_THE_CHOSEN_SHELL: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_systems_answer_no_and_the_third_is_observed() {
        assert_eq!(
            one_dialog_takes_both(System::Windows),
            OneDialogTakesBoth::No
        );
        assert_eq!(
            one_dialog_takes_both(System::LinuxOnThePortalRoute),
            OneDialogTakesBoth::No
        );
        assert_eq!(
            one_dialog_takes_both(System::MacOs),
            OneDialogTakesBoth::ObservedYes
        );
        assert_eq!(System::ALL.len(), 3);
    }

    #[test]
    fn it_is_one_control_on_every_system_and_never_a_second_one_beside_it() {
        // H4's whole answer: the fallback is a question inside Upload, so the
        // count of controls does not vary with the system.
        for system in System::ALL {
            let control = the_upload_control(system);
            assert!(matches!(
                control,
                TheUploadControl::OpensOnePickerForBoth | TheUploadControl::AsksWhichKindFirst
            ));
        }
        assert_eq!(
            the_upload_control(System::Windows),
            TheUploadControl::AsksWhichKindFirst
        );
        assert_eq!(
            the_upload_control(System::MacOs),
            TheUploadControl::OpensOnePickerForBoth
        );
    }

    #[test]
    fn the_negative_answer_would_still_need_no_new_control_and_no_new_state() {
        // §13 item 8's *Holds*, kept after the item closed on 2026-09-04: the
        // design's one-control assumption was written to survive either answer,
        // and this is what that means in code. The macOS answer is now observed
        // rather than inferred, so nothing rests on this any more — it stays
        // because the shape it demonstrates is H4's own, not item 8's.
        let if_it_were_negative = TheUploadControl::AsksWhichKindFirst;
        assert_eq!(the_upload_control(System::Windows), if_it_were_negative);
        assert_eq!(
            the_upload_control(System::LinuxOnThePortalRoute),
            if_it_were_negative
        );
    }

    #[test]
    fn a_count_at_the_drop_is_a_count_of_what_was_dropped() {
        assert_eq!(TheCountAtTheDrop::of(3), TheCountAtTheDrop(3));
        const { assert!(THE_ENUMERATION_RUNS_AFTER_THE_UPLOAD_IS_CONFIRMED_AND_NOTHING_MOVES_EARLIER) };
        const { assert!(WHETHER_A_DROPPED_FOLDER_IS_CARRIED_IS_UNSTATED_BY_THE_CHOSEN_SHELL) };
    }
}

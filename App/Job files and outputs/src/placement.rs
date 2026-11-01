//! §1.3 — the confirmations a placement is only worth anything with.
//!
//! **L10 and L13 are requirements on other documents** and are stated at
//! [`crate::layout::WHAT_THIS_DOCUMENT_REQUIRES_OF_THE_MACHINE`]. **L11 and L12
//! are acts, and they are performed here** so that `Server Setup`'s hand-over
//! calls this rather than composing its own confirmation.

use crate::acting::{TheMachinesDirectories, WhatTheMachineSaid};
use crate::layout::{TheLayout, A_PLACEMENT_IS_CONFIRMED_BY_WRITING_AT_LEAST_THIS_MANY_BYTES};

/// The bytes the confirming write puts down.
///
/// **At least one, and L11 is exact about why one is the whole point**: on a
/// full filesystem *"the writability test **returns writable**, and **creating
/// an empty file succeeds**"* — an empty inode needs no data blocks — and *"Only
/// writing at least one byte fails"*.
const WHAT_THE_CONFIRMING_WRITE_PUTS_DOWN: &[u8] = b"smoozy";

/// The name the confirming write uses, inside the area being confirmed.
const THE_CONFIRMING_WRITES_NAME: &str = "placed";

/// How a confirmation ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheConfirmationFound {
    /// The write landed and, where a read-back was asked for, came back as
    /// itself.
    ItIsPlaced,
    /// The machine refused, and this is what it said.
    TheMachineRefused(WhatTheMachineSaid),
    /// **The read-back came back as something else.** L12's whole reason: a
    /// write that succeeded and a file that reads differently is the case a
    /// writability test cannot tell from a good one.
    ItReadBackAsSomethingElse,
    /// **The confirming file could not be removed, and it is still there.**
    ///
    /// **It is reported rather than swallowed because it is the thing that
    /// refuses the next attempt.** The real write is an exclusive create, so a
    /// confirming file left at that path makes **every** later confirmation of
    /// that location fail — and `Server Setup`'s hand-over is a resumable step
    /// that is tried again. A caller told only *the machine refused* would try
    /// a perfectly good machine for ever.
    TheConfirmingFileCouldNotBeRemoved(WhatTheMachineSaid),
}

/// **L11, performed: the placement is confirmed by a real write of at least one
/// byte at the exact path.**
///
/// **Never by a writability test and never by creating an empty file** —
/// `State, identity and recovery` §1.3 rule 4. **There is no writability test on
/// [`TheMachinesDirectories`] to reach for**, and the only write it offers puts
/// bytes down.
///
/// **L12, performed with it: the placement is confirmed by reading it back
/// under the login the render will run as.** That is the same shape in the
/// other direction, and L13 is what makes *the login the render will run as*
/// the same login this connection holds — the supervisor and every worker,
/// the render included, run as one.
///
/// **The confirming file is removed on every way out.** Nothing else in this
/// document reads it, and a file left behind in an area is a file the areas' own
/// rules would then have to account for — **and worse than that, the real write
/// is an exclusive create, so one left at this path refuses every later
/// confirmation of the same location, for good.** The removal therefore happens
/// whatever the read did, and **a removal that fails is the answer** rather
/// than something thrown away.
pub async fn confirm(
    machine: &mut impl TheMachinesDirectories,
    at_directory: &str,
) -> WhatTheConfirmationFound {
    // **L11's own figure**, so the bytes below cannot fall below it without
    // this failing to build.
    const _: () = assert!(
        WHAT_THE_CONFIRMING_WRITE_PUTS_DOWN.len()
            >= A_PLACEMENT_IS_CONFIRMED_BY_WRITING_AT_LEAST_THIS_MANY_BYTES
    );
    let path = format!("{at_directory}/{THE_CONFIRMING_WRITES_NAME}");
    let written = machine
        .write_a_whole_file_durably(&path, WHAT_THE_CONFIRMING_WRITE_PUTS_DOWN)
        .await;
    // **The read is taken but not answered on**, so that the removal below
    // happens on the read's failure path too — which is the path that used to
    // leave the file where it was written.
    let read = match written {
        Ok(()) => Some(machine.read_a_whole_file(&path).await),
        Err(_) => None,
    };
    // **The removal, on every way out.** *Not there* is the file not being
    // left behind, which is the whole point of removing it; anything else is
    // the file still standing, and that is what refuses the next attempt.
    match machine.remove_one_file(&path).await {
        Ok(()) | Err(WhatTheMachineSaid::ThePathIsNotThere) => {}
        Err(said) => return WhatTheConfirmationFound::TheConfirmingFileCouldNotBeRemoved(said),
    }
    match (written, read) {
        (Err(said), _) | (_, Some(Err(said))) => WhatTheConfirmationFound::TheMachineRefused(said),
        (Ok(()), Some(Ok(read))) if read == WHAT_THE_CONFIRMING_WRITE_PUTS_DOWN => {
            WhatTheConfirmationFound::ItIsPlaced
        }
        (Ok(()), _) => WhatTheConfirmationFound::ItReadBackAsSomethingElse,
    }
}

/// **The hand-over's own confirmation: the root, and the package area's own
/// location where L4a placed it apart.**
///
/// **L10 names both** — *"the whole root, and the package area's location where
/// L4a places it apart"* — so a hand-over that confirmed only the root would
/// have confirmed nothing about the mount the machine's own records go on.
pub async fn confirm_the_layout(
    machine: &mut impl TheMachinesDirectories,
    layout: &TheLayout,
) -> Vec<(String, WhatTheConfirmationFound)> {
    let mut done = vec![(
        layout.root().as_path().to_string(),
        confirm(machine, layout.root().as_path()).await,
    )];
    if let crate::layout::WhereThePackageAreaSits::ApartAt(apart) =
        layout.where_the_package_area_sits()
    {
        done.push((
            apart.as_path().to_string(),
            confirm(machine, apart.as_path()).await,
        ));
    }
    done
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::{MachineRoot, WhereThePackageAreaSits, EVERY_AREA};
    use crate::publication::testing::AMachine;

    fn a_layout(package_area: WhereThePackageAreaSits) -> TheLayout {
        TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").unwrap(),
            package_area,
        )
    }

    fn a_machine(layout: &TheLayout) -> AMachine {
        let mut machine = AMachine::with_the_areas_of(layout);
        machine
            .directories
            .insert(layout.root().as_path().to_string());
        if let WhereThePackageAreaSits::ApartAt(apart) = layout.where_the_package_area_sits() {
            machine.directories.insert(apart.as_path().to_string());
        }
        let _ = EVERY_AREA;
        machine
    }

    #[tokio::test]
    async fn the_confirmation_writes_bytes_reads_them_back_and_leaves_nothing_behind() {
        let layout = a_layout(WhereThePackageAreaSits::UnderTheRoot);
        let mut machine = a_machine(&layout);
        assert_eq!(
            confirm(&mut machine, layout.root().as_path()).await,
            WhatTheConfirmationFound::ItIsPlaced
        );
        // **A real write of at least one byte**, and it was made durable.
        assert!(!machine.made_durable.is_empty());
        // And nothing is left in the area afterwards.
        assert!(machine.files.is_empty());
    }

    #[tokio::test]
    async fn a_refused_write_is_the_answer_and_no_writability_test_stands_in_for_it() {
        let layout = a_layout(WhereThePackageAreaSits::UnderTheRoot);
        let mut machine = a_machine(&layout);
        machine.refuse_at = Some(format!(
            "{}/{THE_CONFIRMING_WRITES_NAME}",
            layout.root().as_path()
        ));
        assert_eq!(
            confirm(&mut machine, layout.root().as_path()).await,
            WhatTheConfirmationFound::TheMachineRefused(WhatTheMachineSaid::ItRefused)
        );
    }

    #[tokio::test]
    async fn a_failed_read_back_still_leaves_nothing_behind() {
        // **The lockout this closes.** The write lands, the connection drops
        // before the read comes back, and the confirming file used to stay
        // where it was written — after which the exclusive create refuses
        // **every** later confirmation of that location, so a perfectly good
        // machine can never be handed over. `Server Setup`'s hand-over is a
        // resumable step, so it is tried again, and again.
        let layout = a_layout(WhereThePackageAreaSits::UnderTheRoot);
        let mut machine = a_machine(&layout);
        let path = format!(
            "{}/{THE_CONFIRMING_WRITES_NAME}",
            layout.root().as_path()
        );
        machine.refuse_reads_at = Some(path.clone());

        assert_eq!(
            confirm(&mut machine, layout.root().as_path()).await,
            WhatTheConfirmationFound::TheMachineRefused(WhatTheMachineSaid::TheChannelIsGone)
        );
        assert!(
            !machine.files.contains_key(&path),
            "the confirming file was left behind on the read's failure path"
        );
        // And the next attempt against the same location is not refused by
        // something the last one left.
        machine.refuse_reads_at = None;
        assert_eq!(
            confirm(&mut machine, layout.root().as_path()).await,
            WhatTheConfirmationFound::ItIsPlaced
        );
    }

    #[tokio::test]
    async fn a_confirming_file_that_could_not_be_removed_reaches_the_caller() {
        // It is the thing that will refuse the next attempt, so it is not
        // thrown away.
        let layout = a_layout(WhereThePackageAreaSits::UnderTheRoot);
        let mut machine = a_machine(&layout);
        machine.refuse_removals_at = Some(format!(
            "{}/{THE_CONFIRMING_WRITES_NAME}",
            layout.root().as_path()
        ));
        assert_eq!(
            confirm(&mut machine, layout.root().as_path()).await,
            WhatTheConfirmationFound::TheConfirmingFileCouldNotBeRemoved(
                WhatTheMachineSaid::TheLoginMayNot
            )
        );
    }

    #[tokio::test]
    async fn l4as_second_location_is_confirmed_too_and_not_only_the_root() {
        let layout = a_layout(WhereThePackageAreaSits::ApartAt(
            MachineRoot::chosen("/state/smoozy").unwrap(),
        ));
        let mut machine = a_machine(&layout);
        let done = confirm_the_layout(&mut machine, &layout).await;
        assert_eq!(
            done,
            vec![
                (
                    "/ephemeral/smoozy".to_string(),
                    WhatTheConfirmationFound::ItIsPlaced
                ),
                (
                    "/state/smoozy".to_string(),
                    WhatTheConfirmationFound::ItIsPlaced
                ),
            ]
        );
    }

    #[tokio::test]
    async fn one_location_is_confirmed_where_l4as_second_choice_did_not_fire() {
        let layout = a_layout(WhereThePackageAreaSits::UnderTheRoot);
        let mut machine = a_machine(&layout);
        assert_eq!(confirm_the_layout(&mut machine, &layout).await.len(), 1);
    }
}

//! §1 — the server directory layout, and §2.1's rule that every path is
//! composed here.
//!
//! **L6's five areas are one enumeration and the layout is the only thing that
//! composes a path from a root.** Nothing else in this crate, and nothing in
//! any other crate, joins a root to a segment by hand — which is what makes
//! **N1** (*every path Smoozy composes on a rented machine is composed by this
//! document's rules*) and **N2** (*a path under neither the root nor the
//! package area's location is a fault rather than a case*) properties of the
//! type rather than habits.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server Setup` R7b.

use crate::names;

/// A root that has passed **L3**'s test and **N5**'s budget.
///
/// **L1: every directory this document owns sits under one root on the rented
/// machine — the package area alone may sit apart, per L4a — and the root is
/// chosen once per installation and never re-chosen.** The choosing is
/// [`crate::root_choice`]; this type is what a chosen one is.
///
/// **There is no constructor that skips the test**, so a root that sits under a
/// shared temporary area cannot be represented at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRoot {
    path: String,
}

/// Why a candidate path cannot be a root. **Smoozy's own arithmetic and
/// Smoozy's own rule** — nothing here is a machine's words (N6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyThatIsNotARoot {
    /// Every path this document composes is absolute, because **N2** asks
    /// whether a path is under the root and a relative one answers nothing.
    ItIsNotAnAbsolutePath,
    /// **L3: the root never sits under a shared temporary area, and neither
    /// does the package area's location where L4a places it apart.** `/tmp`
    /// *"is wiped at every boot"* and `/var/tmp` is *"an easy place to leave one
    /// rental's data for the next tenant"* (Fact 6). This is `State, identity
    /// and recovery` §1.3 rule 2 applied to material as well as to stores.
    ItIsUnderASharedTemporaryArea,
    /// **A segment of it is `.` or `..`.**
    ///
    /// **L3's test is otherwise a comparison of text**, so a candidate written
    /// with a relative segment passes it and can still put the root under an
    /// area the machine wipes at every boot — `/ephemeral/../tmp/smoozy` is not
    /// `/tmp/...` to a text comparison and is `/tmp/...` to the machine. **If
    /// the root goes under a wiped area then after one reboot the job records,
    /// the manifests and the attempt records are gone**, every live job
    /// re-renders its whole range under C20 at the user's expense, and the
    /// outputs go with them.
    ///
    /// **This is Smoozy's own arithmetic against Smoozy's own rule** and asks
    /// nothing about any machine. **The other half of the same hazard is not
    /// closed here**: a candidate that is a link to somewhere else would also
    /// pass a text comparison, and what a link does on a rented Linux machine,
    /// and whether a writable location handed over by `Server Setup` can ever
    /// be one, is established nowhere in this project. That belongs to `Server
    /// Setup` and is parked there rather than guessed at here.
    ///
    /// *(Added 2026-10-16, step 10c owed piece 7, on the build runner's fifth
    /// ruling on the owed lines. The paragraph above is kept as written. **The other half is closed
    /// on the machine**, at [`MachineRoot::chosen_where_it_really_leads`]: the
    /// machine reads where the candidate really leads and runs L3's test on
    /// that too, and [`ItLeadsUnderASharedTemporaryArea`] is that refusal.
    /// [`MachineRoot::chosen`] itself stays a comparison of text, because the
    /// desktop calls it too, and a path on a rented machine cannot be resolved
    /// against the desktop's own disk.)*
    ///
    /// [`ItLeadsUnderASharedTemporaryArea`]: WhyThatIsNotARoot::ItLeadsUnderASharedTemporaryArea
    ItHasARelativeSegment,
    /// **L3, judged by where the candidate really leads on the machine.** The
    /// text passed every test, and the place it leads to on this machine is
    /// under a shared temporary area: a link somewhere along the path points
    /// into one. *Added 2026-10-16, step 10c owed piece 7, on the build runner's
    /// fifth ruling on the owed lines.*
    ItLeadsUnderASharedTemporaryArea,
    /// **Where the candidate leads could not be read on the machine at all**,
    /// not even at its nearest existing parent. **It is refused rather than
    /// judged on its text alone**: an absent reading is never stood in for.
    /// *Added 2026-10-16, step 10c owed piece 7.*
    WhereItLeadsCouldNotBeRead,
    /// **N5's budget**, measured before the path is used.
    ItDoesNotFitTheBudget(names::WhyThePathIsRefused),
}

/// **L3's two shared temporary areas, named once.**
pub(crate) const SHARED_TEMPORARY_AREAS: [&str; 2] = ["/tmp", "/var/tmp"];

impl MachineRoot {
    /// **L3 and N5, applied before anything can hold a root.**
    pub fn chosen(path: &str) -> Result<MachineRoot, WhyThatIsNotARoot> {
        if !path.starts_with('/') {
            return Err(WhyThatIsNotARoot::ItIsNotAnAbsolutePath);
        }
        let tidied = path.trim_end_matches('/');
        // **Before the shared-temporary-area test, not after it**, because the
        // test below is a comparison of text and a relative segment is what
        // makes that comparison answer about a path other than the one the
        // machine would open.
        if tidied
            .split('/')
            .any(|segment| segment == "." || segment == "..")
        {
            return Err(WhyThatIsNotARoot::ItHasARelativeSegment);
        }
        if SHARED_TEMPORARY_AREAS
            .iter()
            .any(|area| it_is_at_or_under(tidied, area))
        {
            return Err(WhyThatIsNotARoot::ItIsUnderASharedTemporaryArea);
        }
        names::a_path_fits(tidied).map_err(WhyThatIsNotARoot::ItDoesNotFitTheBudget)?;
        Ok(MachineRoot {
            path: tidied.to_string(),
        })
    }

    /// **L3 and N5 as [`MachineRoot::chosen`] applies them, and then L3 again on
    /// where the candidate really leads on this machine.** Step 10c owed piece
    /// 7, on the build runner's fifth ruling on the owed lines (2026-10-16),
    /// which is in `Audits/Build/Step 10c/The runner's rulings.md`.
    ///
    /// **Only the machine calls this**, because only the machine can read its
    /// own disk. The desktop keeps [`MachineRoot::chosen`]: a path on a rented
    /// machine resolved against the desktop's own disk would answer about the
    /// wrong computer.
    ///
    /// **This crate reads no disk, so the reading is handed in.**
    /// `where_it_leads` answers where one path that exists really leads, with
    /// every link along it followed, or nothing where that path cannot be
    /// read. **Everything else is Smoozy's own arithmetic**: the walk up to the
    /// nearest existing parent and the comparison with the shared temporary
    /// areas.
    ///
    /// **Three things, in this order.**
    ///
    /// 1. **The text tests first**, `.` and `..` included, exactly as
    ///    [`MachineRoot::chosen`] runs them. A refusal there asks the machine
    ///    nothing.
    /// 2. **Where the candidate leads**, read at the candidate itself or, where
    ///    it does not exist yet, at its nearest existing parent, with the rest
    ///    of the path put back on unchanged. A part that does not exist cannot
    ///    be a link, so the parent decides where it would lead.
    /// 3. **L3 on that place**, against each shared temporary area both as it
    ///    is named and as it itself really leads. An area that is a link — the
    ///    desk's Mac has `/tmp` as one — would otherwise let a path into it
    ///    pass under its resolved name.
    ///
    /// **The root keeps the path as written.** Every path the layout composes
    /// is composed from it, so the record, N2's comparisons and the hand-over
    /// all keep meaning the same text.
    pub fn chosen_where_it_really_leads(
        path: &str,
        where_it_leads: impl Fn(&str) -> Option<String>,
    ) -> Result<MachineRoot, WhyThatIsNotARoot> {
        let root = MachineRoot::chosen(path)?;
        let led_to = the_place_it_really_leads(root.as_path(), &where_it_leads)
            .ok_or(WhyThatIsNotARoot::WhereItLeadsCouldNotBeRead)?;
        for area in SHARED_TEMPORARY_AREAS {
            let as_named = it_is_at_or_under(&led_to, area);
            let as_it_leads = where_it_leads(area)
                .is_some_and(|area_led_to| it_is_at_or_under(&led_to, &area_led_to));
            if as_named || as_it_leads {
                return Err(WhyThatIsNotARoot::ItLeadsUnderASharedTemporaryArea);
            }
        }
        Ok(root)
    }

    pub fn as_path(&self) -> &str {
        &self.path
    }
}

/// **Whether `path` is `area` or sits under it**, on whole segments.
fn it_is_at_or_under(path: &str, area: &str) -> bool {
    let area = area.trim_end_matches('/');
    let path = path.trim_end_matches('/');
    if area.is_empty() {
        // **The machine's own top**, which an area could only be by leading
        // there. Everything sits under it.
        return true;
    }
    path == area || path.starts_with(&format!("{area}/"))
}

/// **Where an absolute path without a `.` or `..` part really leads**, read at
/// the path itself or at its nearest existing parent.
///
/// **How the nearest existing parent is found, and why this way.** The last
/// part is taken off one at a time and kept, until `where_it_leads` answers;
/// the parts kept are then put back, in order, on the answer. **A path that
/// does not exist cannot itself be a link**, so the place it would lead to once
/// made is wherever its nearest existing parent leads, with the missing parts
/// under it. **The machine's top is the last thing asked**, and where even it
/// gives no answer the path's place is unknown and nothing is returned.
fn the_place_it_really_leads(
    path: &str,
    where_it_leads: &impl Fn(&str) -> Option<String>,
) -> Option<String> {
    let mut asked = path.trim_end_matches('/').to_string();
    let mut missing: Vec<String> = Vec::new();
    loop {
        let asking = if asked.is_empty() { "/" } else { asked.as_str() };
        if let Some(led_to) = where_it_leads(asking) {
            let mut place = led_to.trim_end_matches('/').to_string();
            for part in missing.iter().rev() {
                place.push('/');
                place.push_str(part);
            }
            if place.is_empty() {
                place.push('/');
            }
            return Some(place);
        }
        if asked.is_empty() {
            return None;
        }
        let (parent, part) = asked.rsplit_once('/')?;
        missing.push(part.to_string());
        asked = parent.to_string();
    }
}

/// **L6's five areas.** *"Five areas, and no path in any one of them is ever
/// under another: four sit directly under the root as siblings, and the package
/// area sits at L4a's location — directly under the root too, exactly where L4a
/// chose no second mount."*
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Area {
    /// The active-package pointer, the boot-registration record, the durable
    /// package-phase record, the installation identity, the machine-local
    /// queue, the operation and command records, the event journal, the
    /// machine-level desired states, the captured streams, the bot token file.
    /// **Its contents are `Rented-server runtime`'s, `Queue & History`'s,
    /// `State, identity and recovery` §1.2's and `Security and licence` §10's**
    /// — none of them this document's.
    Package,
    /// One **job record directory** per job identity.
    JobRecord,
    /// One **job output folder** per job identity, and nothing else.
    Job,
    /// The mirrored copies `Dependency Upload & Remap` places, and the remap
    /// material it keeps on the machine.
    Asset,
    /// One folder per Blender install `Server Setup` placed, and nothing else.
    /// **Added to L6 on 2026-08-22 at the Architecture seam close.**
    Blender,
}

/// **The five, as one list.** L6 is an enumeration and a caller that walks the
/// areas walks this rather than a list kept in step by hand.
pub const EVERY_AREA: [Area; 5] = [
    Area::Package,
    Area::JobRecord,
    Area::Job,
    Area::Asset,
    Area::Blender,
];

impl Area {
    /// The area's own segment name.
    ///
    /// **These five names are Build's own choice and the plan fixes none of
    /// them.** What the plan does fix is **N7**: the prefix is *three* segments
    /// under the chosen mount — the root segment, the job-area segment and the
    /// job folder segment — and *"the prefix is kept short deliberately"*
    /// because §6 deepens paths by construction. So each is one short segment
    /// and none of them is composed from anything.
    pub fn segment(self) -> &'static str {
        match self {
            Area::Package => "package",
            Area::JobRecord => "job-records",
            Area::Job => "jobs",
            Area::Asset => "assets",
            Area::Blender => "blender",
        }
    }
}

/// Where the package area sits — **L4a's two arrangements and there is no
/// third.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereThePackageAreaSits {
    /// *"Where no remaining candidate stands or clears it, the package area
    /// sits under the root as a sibling of the others, which is the arrangement
    /// every clause below already covers."* **Nothing second is chosen or
    /// recorded** and its location follows from the root's recorded path.
    UnderTheRoot,
    /// *"The package area takes the remaining candidate with the largest
    /// available figure … provided that figure clears the 64 MiB control-state
    /// reserve."* **That location's path is recorded exactly as the root's
    /// is** (L1, `Server Setup` R7b).
    ApartAt(MachineRoot),
}

/// The layout of one installation. **Chosen once at hand-over and never
/// re-chosen** (L1, L4a).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheLayout {
    root: MachineRoot,
    package_area: WhereThePackageAreaSits,
}

impl TheLayout {
    pub fn of(root: MachineRoot, package_area: WhereThePackageAreaSits) -> TheLayout {
        TheLayout { root, package_area }
    }

    pub fn root(&self) -> &MachineRoot {
        &self.root
    }

    pub fn where_the_package_area_sits(&self) -> &WhereThePackageAreaSits {
        &self.package_area
    }

    /// One area's own directory.
    ///
    /// **L6: no path in any one of them is ever under another.** Four are
    /// siblings under the root; the package area is a sibling too, unless L4a
    /// placed it apart, in which case it is that mount's own first segment and
    /// sits *"under no shared root at all"* (§13's supply to `Security and
    /// licence`).
    pub fn area(&self, area: Area) -> String {
        match (area, &self.package_area) {
            (Area::Package, WhereThePackageAreaSits::ApartAt(apart)) => {
                format!("{}/{}", apart.as_path(), Area::Package.segment())
            }
            _ => format!("{}/{}", self.root.as_path(), area.segment()),
        }
    }

    /// **N2, asked rather than assumed.** *"Every such path is under the root
    /// of 1.1 or under the package area's location of its L4a, and a path under
    /// neither is a fault rather than a case."*
    ///
    /// A builder that finds itself composing an absolute path outside both has
    /// found a fault in §2.1, so this is the question that says so.
    pub fn is_under_something_this_layout_owns(&self, path: &str) -> bool {
        let mut owners = vec![self.root.as_path().to_string()];
        if let WhereThePackageAreaSits::ApartAt(apart) = &self.package_area {
            owners.push(apart.as_path().to_string());
        }
        owners
            .iter()
            .any(|owner| path == owner.as_str() || path.starts_with(&format!("{owner}/")))
    }

    /// **L7's rule, asked of one path.** *"Nothing but what a render writes ever
    /// sits inside a job output folder."*
    ///
    /// **Three separate promises break if this is relaxed and they break
    /// silently**: the expected-output inventory is compared against the
    /// folder's contents, `Download` brings the folder home whole, and
    /// `Security and licence` §10's protection of the bot token rests on the
    /// package area being outside every path the user browses, uploads,
    /// downloads or renders under. So every path this crate composes for
    /// Smoozy's own material is checked against this and none of them passes.
    pub fn is_inside_a_job_output_folder(&self, path: &str) -> bool {
        let job_area = self.area(Area::Job);
        path.starts_with(&format!("{job_area}/"))
    }
}

/// **L1's record of the root, by the name it has in the package area.**
///
/// *"Its path is recorded in the machine's state area at hand-over, beside the
/// installation identity."* **The name is Build's own**, as the five area
/// segments are, and it sits beside `Rented-server runtime`'s own record of
/// the installation identity. **What it holds is the root's path as text and
/// nothing else**; where L4a placed the package area apart, that location is
/// the package area the supervisor already runs from, and nothing second is
/// read.
///
/// **It is written by `Server Setup` R7b's hand-over and read by the
/// supervisor**, which composes the job record directory from it when it writes
/// `Queue & History`'s job record. Named at step 10c piece 5, which is the first
/// reader; **no machine-side writer exists yet** — the hand-over's machine half
/// is step 10c piece 8's — so a machine that was never handed over has no root
/// recorded and the supervisor refuses a job rather than guessing one.
pub const THE_RECORD_OF_THE_ROOT: &str = "root";

/// **L7a, named rather than left as a general relaxation.**
///
/// *"One exception, and it is the only one: a bake or cache the user uploaded,
/// placed at the redirect's own target for that write place."* It is placed by
/// `Dependency Upload & Remap` before the render, under one of D20's four added
/// write places and nowhere else, and **a file arriving by any other route is
/// still forbidden**.
///
/// **Nothing in this crate places one**, because placing is
/// `Dependency Upload & Remap`'s act. The redirect that names its target is
/// §5.5's and is built at [`crate::redirect`]. *(Corrected 2026-09-07: this
/// said the redirect arrives at step 7 and pointed at
/// `arriving_later` in `smoozy-job-files`. Step 7 landed it, and that page has been empty
/// since.)* It is written here so that a later builder meets the exception
/// where L7 is kept rather than discovering that L7 was absolute.
pub const THE_ONE_EXCEPTION_TO_L7_IS_AN_UPLOADED_BAKE_OR_CACHE: bool = true;

/// **L8: the reverse holds too.** *"Nothing outside the job output folder is
/// ever counted, listed, watched or brought home as a job's output."*
pub const NOTHING_OUTSIDE_THE_JOB_OUTPUT_FOLDER_IS_A_JOBS_OUTPUT: bool = true;

/// **L11: the root's placement is confirmed by a real write of at least one
/// byte at the exact path, and never by a writability test and never by
/// creating an empty file.**
///
/// `State, identity and recovery` §1.3 rule 4, on the observation that on a
/// full filesystem *"the writability test **returns writable**, and **creating
/// an empty file succeeds**"* — an empty inode needs no data blocks — and only
/// writing at least one byte fails (Fact 28).
pub const A_PLACEMENT_IS_CONFIRMED_BY_WRITING_AT_LEAST_THIS_MANY_BYTES: usize = 1;

/// **L10 and L13, which are requirements this document places on other
/// documents rather than acts it takes.**
///
/// **L10:** the whole root — and the package area's location, where L4a places
/// it apart — is owned by the login the supervisor runs as, **before
/// hand-over**. A rental's second place to write arrives `root:root 755` and
/// refuses that login until a setup step hands it over (Fact 6). **That step is
/// `Server Setup`'s.**
///
/// **L13:** the supervisor and every worker it starts, the render included, run
/// as one login. Worker identity is `Rented-server runtime`'s. **Under two
/// logins L10 gives the root to one of them and nothing says how the render
/// writes into the job output folder at all.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequirementsOnOtherDocuments {
    pub the_whole_root_is_handed_to_the_supervisors_login_before_hand_over: bool,
    pub the_supervisor_and_every_worker_run_as_one_login: bool,
}

pub const WHAT_THIS_DOCUMENT_REQUIRES_OF_THE_MACHINE: RequirementsOnOtherDocuments =
    RequirementsOnOtherDocuments {
        the_whole_root_is_handed_to_the_supervisors_login_before_hand_over: true,
        the_supervisor_and_every_worker_run_as_one_login: true,
    };

#[cfg(test)]
mod tests {
    use super::*;

    fn a_layout() -> TheLayout {
        TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").unwrap(),
            WhereThePackageAreaSits::UnderTheRoot,
        )
    }

    #[test]
    fn l3_refuses_a_root_under_either_shared_temporary_area() {
        assert_eq!(
            MachineRoot::chosen("/tmp/smoozy"),
            Err(WhyThatIsNotARoot::ItIsUnderASharedTemporaryArea)
        );
        assert_eq!(
            MachineRoot::chosen("/var/tmp/smoozy"),
            Err(WhyThatIsNotARoot::ItIsUnderASharedTemporaryArea)
        );
        assert_eq!(
            MachineRoot::chosen("/tmp"),
            Err(WhyThatIsNotARoot::ItIsUnderASharedTemporaryArea)
        );
        // A path that merely begins with the same letters is not under it.
        assert!(MachineRoot::chosen("/tmpfiles/smoozy").is_ok());
        assert!(MachineRoot::chosen("/var/tmpish/smoozy").is_ok());
    }

    #[test]
    fn l3s_test_is_not_walked_round_by_a_relative_segment() {
        // The shared-temporary-area test compares text, so a path that reaches
        // `/tmp` through a relative segment passes it and lands Smoozy's
        // material where the machine wipes it at every boot.
        assert_eq!(
            MachineRoot::chosen("/ephemeral/../tmp/smoozy"),
            Err(WhyThatIsNotARoot::ItHasARelativeSegment)
        );
        assert_eq!(
            MachineRoot::chosen("/ephemeral/./smoozy"),
            Err(WhyThatIsNotARoot::ItHasARelativeSegment)
        );
        assert_eq!(
            MachineRoot::chosen("/ephemeral/smoozy/.."),
            Err(WhyThatIsNotARoot::ItHasARelativeSegment)
        );
        // A name that merely begins or ends with a dot is not a relative
        // segment, and N13 says the machine takes it.
        assert!(MachineRoot::chosen("/ephemeral/.smoozy").is_ok());
        assert!(MachineRoot::chosen("/ephemeral/smoozy...").is_ok());
    }

    #[test]
    fn a_root_is_absolute_or_it_is_not_a_root() {
        assert_eq!(
            MachineRoot::chosen("ephemeral/smoozy"),
            Err(WhyThatIsNotARoot::ItIsNotAnAbsolutePath)
        );
    }

    /// **The machine's own reading, as the machine program takes it**: where a
    /// path that exists really leads, every link followed.
    fn where_it_leads_on_this_disk(path: &str) -> Option<String> {
        std::fs::canonicalize(path)
            .ok()
            .and_then(|led_to| led_to.to_str().map(str::to_string))
    }

    /// **A scratch folder beside this test's own program, in the build
    /// folder**, which sits under no shared temporary area on the desk or on a
    /// hosted machine — so the text test passes and only where a path leads
    /// decides.
    ///
    /// **It goes when the test ends, whichever way it ends**, with every link
    /// made in it: removing a folder takes a link out and never follows it.
    /// Until 2026-10-18 it was removed only on the last line, so a test that
    /// failed left its link into `/tmp` in the build folder, and a later test
    /// walking that folder read whatever `/tmp` held (piece P8b of step 10c).
    struct AScratchFolder(std::path::PathBuf);

    impl Drop for AScratchFolder {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).ok();
        }
    }

    impl std::ops::Deref for AScratchFolder {
        type Target = std::path::Path;
        fn deref(&self) -> &std::path::Path {
            &self.0
        }
    }

    fn a_scratch_folder(name: &str) -> AScratchFolder {
        let beside = std::env::current_exe()
            .expect("this test's own program")
            .parent()
            .expect("its folder")
            .to_path_buf();
        let at = beside.join(format!("smoozy-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&at);
        std::fs::create_dir_all(&at).expect("a scratch folder");
        AScratchFolder(at)
    }

    /// **A small made-up disk**: each path that exists, and where it leads.
    fn a_disk(entries: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let entries: Vec<(String, String)> = entries
            .iter()
            .map(|(at, to)| (at.to_string(), to.to_string()))
            .collect();
        move |path: &str| {
            entries
                .iter()
                .find(|(at, _)| at == path)
                .map(|(_, to)| to.clone())
        }
    }

    /// **Piece 7's first proof.** A link made in a scratch folder, pointing
    /// into an area wiped at every boot, passes the text test and is refused
    /// once the machine reads where it leads.
    #[cfg(unix)]
    #[test]
    fn a_root_that_leads_into_a_shared_temporary_area_through_a_link_is_refused() {
        let at = a_scratch_folder("a-link-into-the-wiped-area");
        let link = at.join("data");
        std::os::unix::fs::symlink("/tmp", &link).expect("a link");
        let root = format!("{}/smoozy", link.display());

        // **The text passes**: nothing in it names a shared temporary area.
        assert!(MachineRoot::chosen(&root).is_ok());
        assert_eq!(
            MachineRoot::chosen_where_it_really_leads(&root, where_it_leads_on_this_disk),
            Err(WhyThatIsNotARoot::ItLeadsUnderASharedTemporaryArea)
        );
        // **The link itself**, which exists.
        assert_eq!(
            MachineRoot::chosen_where_it_really_leads(
                &link.display().to_string(),
                where_it_leads_on_this_disk
            ),
            Err(WhyThatIsNotARoot::ItLeadsUnderASharedTemporaryArea)
        );
        // **Several parts below the link, none of which exist.**
        assert_eq!(
            MachineRoot::chosen_where_it_really_leads(
                &format!("{}/a/b/smoozy", link.display()),
                where_it_leads_on_this_disk
            ),
            Err(WhyThatIsNotARoot::ItLeadsUnderASharedTemporaryArea)
        );
        // **The second area, where this machine has one.**
        if std::path::Path::new("/var/tmp").is_dir() {
            let second = at.join("scratch");
            std::os::unix::fs::symlink("/var/tmp", &second).expect("a link");
            assert_eq!(
                MachineRoot::chosen_where_it_really_leads(
                    &format!("{}/smoozy", second.display()),
                    where_it_leads_on_this_disk
                ),
                Err(WhyThatIsNotARoot::ItLeadsUnderASharedTemporaryArea)
            );
        }
    }

    /// **Piece 7's second proof.** A link pointing to a good place is taken,
    /// and the root keeps the path as written rather than where it leads.
    #[cfg(unix)]
    #[test]
    fn a_root_that_leads_to_a_good_place_through_a_link_is_taken_as_written() {
        let at = a_scratch_folder("a-link-to-a-good-place");
        let volume = at.join("the-volume");
        std::fs::create_dir_all(&volume).expect("a folder");
        let link = at.join("data");
        std::os::unix::fs::symlink(&volume, &link).expect("a link");
        let root = format!("{}/smoozy", link.display());

        let chosen = MachineRoot::chosen_where_it_really_leads(&root, where_it_leads_on_this_disk)
            .expect("a link to a good place is a root");
        assert_eq!(chosen.as_path(), root);
        // **And a plain folder, with no link anywhere.**
        assert!(MachineRoot::chosen_where_it_really_leads(
            &format!("{}/smoozy", volume.display()),
            where_it_leads_on_this_disk
        )
        .is_ok());
    }

    /// **A path that does not exist yet is judged by its nearest existing
    /// parent, resolved, with the missing parts put back under it.**
    #[test]
    fn a_path_that_does_not_exist_yet_is_judged_by_its_nearest_existing_parent() {
        // A Linux machine's shape: `/tmp` is itself, and `/data` is a link into
        // it.
        let linux = a_disk(&[
            ("/", "/"),
            ("/data", "/tmp/volume"),
            ("/mnt", "/mnt"),
            ("/tmp", "/tmp"),
            ("/var", "/var"),
            ("/var/tmp", "/var/tmp"),
        ]);
        assert_eq!(
            the_place_it_really_leads("/data/smoozy/a", &linux).as_deref(),
            Some("/tmp/volume/smoozy/a")
        );
        assert_eq!(
            the_place_it_really_leads("/mnt/one/smoozy", &linux).as_deref(),
            Some("/mnt/one/smoozy")
        );
        assert_eq!(
            the_place_it_really_leads("/nothing/smoozy", &linux).as_deref(),
            Some("/nothing/smoozy")
        );
        assert_eq!(
            MachineRoot::chosen_where_it_really_leads("/data/smoozy", &linux),
            Err(WhyThatIsNotARoot::ItLeadsUnderASharedTemporaryArea)
        );
        assert_eq!(
            MachineRoot::chosen_where_it_really_leads("/mnt/one/smoozy", &linux)
                .map(|root| root.as_path().to_string()),
            Ok("/mnt/one/smoozy".to_string())
        );

        // **The desk's shape**: `/tmp` is itself a link, so a path into it
        // resolves under a name that is not `/tmp`. Only comparing with where
        // the area leads catches it.
        let mac = a_disk(&[
            ("/", "/"),
            ("/data", "/private/tmp/volume"),
            ("/tmp", "/private/tmp"),
        ]);
        assert_eq!(
            MachineRoot::chosen_where_it_really_leads("/data/smoozy", &mac),
            Err(WhyThatIsNotARoot::ItLeadsUnderASharedTemporaryArea)
        );

        // **An area that cannot be read is still compared by its name.**
        let no_tmp_read = a_disk(&[("/", "/"), ("/data", "/tmp/volume")]);
        assert_eq!(
            MachineRoot::chosen_where_it_really_leads("/data/smoozy", &no_tmp_read),
            Err(WhyThatIsNotARoot::ItLeadsUnderASharedTemporaryArea)
        );
        // **A name that only starts like an area is not under it.**
        let near = a_disk(&[("/", "/"), ("/data", "/tmpfiles/volume"), ("/tmp", "/tmp")]);
        assert!(MachineRoot::chosen_where_it_really_leads("/data/smoozy", &near).is_ok());
    }

    /// **`.` and `..` are still refused, and before the machine is asked
    /// anything.**
    #[test]
    fn the_text_tests_still_come_first_and_ask_the_machine_nothing() {
        let asks_nothing = |_: &str| -> Option<String> { panic!("the machine was asked") };
        assert_eq!(
            MachineRoot::chosen_where_it_really_leads("/ephemeral/../tmp/smoozy", asks_nothing),
            Err(WhyThatIsNotARoot::ItHasARelativeSegment)
        );
        assert_eq!(
            MachineRoot::chosen_where_it_really_leads("/ephemeral/./smoozy", asks_nothing),
            Err(WhyThatIsNotARoot::ItHasARelativeSegment)
        );
        assert_eq!(
            MachineRoot::chosen_where_it_really_leads("/tmp/smoozy", asks_nothing),
            Err(WhyThatIsNotARoot::ItIsUnderASharedTemporaryArea)
        );
        assert_eq!(
            MachineRoot::chosen_where_it_really_leads("ephemeral/smoozy", asks_nothing),
            Err(WhyThatIsNotARoot::ItIsNotAnAbsolutePath)
        );
    }

    /// **Where nothing can be read, not even the machine's top, the candidate
    /// is refused rather than judged on its text.**
    #[test]
    fn a_place_that_cannot_be_read_is_a_refusal() {
        assert_eq!(
            MachineRoot::chosen_where_it_really_leads("/ephemeral/smoozy", |_: &str| None),
            Err(WhyThatIsNotARoot::WhereItLeadsCouldNotBeRead)
        );
    }

    #[test]
    fn l6_puts_no_area_under_another() {
        let layout = a_layout();
        for one in EVERY_AREA {
            for other in EVERY_AREA {
                if one == other {
                    continue;
                }
                let a = layout.area(one);
                let b = layout.area(other);
                assert!(
                    !a.starts_with(&format!("{b}/")),
                    "{a} sits under {b}, which L6 forbids"
                );
            }
        }
    }

    #[test]
    fn l6_puts_no_area_under_another_on_l4as_second_arrangement_either() {
        let layout = TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").unwrap(),
            WhereThePackageAreaSits::ApartAt(MachineRoot::chosen("/state/smoozy").unwrap()),
        );
        for one in EVERY_AREA {
            for other in EVERY_AREA {
                if one == other {
                    continue;
                }
                let a = layout.area(one);
                let b = layout.area(other);
                assert!(!a.starts_with(&format!("{b}/")));
            }
        }
        // **The package area is on a mount of its own, under no shared root at
        // all** — §13's supply to `Security and licence`.
        assert_eq!(layout.area(Area::Package), "/state/smoozy/package");
        assert!(!layout
            .area(Area::Package)
            .starts_with(layout.root().as_path()));
    }

    #[test]
    fn the_package_area_is_never_inside_a_job_output_folder_on_either_arrangement() {
        // `Security and licence` §10's protection of the bot token rests on
        // exactly this, on both of L4a's arrangements.
        for package_area in [
            WhereThePackageAreaSits::UnderTheRoot,
            WhereThePackageAreaSits::ApartAt(MachineRoot::chosen("/state/smoozy").unwrap()),
        ] {
            let layout =
                TheLayout::of(MachineRoot::chosen("/ephemeral/smoozy").unwrap(), package_area);
            assert!(!layout.is_inside_a_job_output_folder(&layout.area(Area::Package)));
        }
    }

    #[test]
    fn n2_reads_a_path_outside_both_locations_as_a_fault() {
        let layout = TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").unwrap(),
            WhereThePackageAreaSits::ApartAt(MachineRoot::chosen("/state/smoozy").unwrap()),
        );
        assert!(layout.is_under_something_this_layout_owns("/ephemeral/smoozy/jobs/abc"));
        assert!(layout.is_under_something_this_layout_owns("/state/smoozy/package/queue"));
        assert!(!layout.is_under_something_this_layout_owns("/ephemeral/somebody-elses"));
        assert!(!layout.is_under_something_this_layout_owns("/home/render/.ssh"));
    }

    #[test]
    fn l9_keeps_the_job_record_directory_and_the_job_output_folder_apart() {
        // *"The job-record area is not the same object as the job output
        // folder"*, and the manifest gets a home the render never writes into.
        let layout = a_layout();
        assert_ne!(layout.area(Area::JobRecord), layout.area(Area::Job));
        assert!(!layout.is_inside_a_job_output_folder(&layout.area(Area::JobRecord)));
    }
}

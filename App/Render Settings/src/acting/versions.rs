//! **B1's list, asked for** — `Render Settings` §7.1 B1, B6.
//!
//! **The list is `Server Setup`'s fresh search result and nothing of it is
//! decided here.** B1: *"The list is `Server Setup`'s fresh search result —
//! (path, answered version, long-term-support-or-unstable) per build — asked
//! for when this page wants it, period-held, never durable, shown with its
//! moment and with B11's honesty marks… Nothing here re-runs, caches durably,
//! or supplements the search."*
//!
//! **So every rule below is that document's, called rather than restated**:
//! the whole-disk search and its refusal count (`Server Setup` B10, B11), the
//! asking-run per candidate with its own time limit (`Server Setup` B1), the
//! deduplication by the path-and-answer pair (`Server Setup` B10), the entries
//! and the naming of candidates that did not answer (`Server Setup` B11).
//! **This file supplies one thing only: the moment this page asks** — `Server
//! Setup` B12's `TheVersionListIsAskedFor`, which is that document's own
//! enumeration of when the search runs.
//!
//! # What this ask does not do, each because a clause forbids it
//!
//! - **It never re-runs, caches durably or supplements the search** (B1). The
//!   result is handed straight back; the period-holding is the caller's, and
//!   `Server Setup`'s `THE_SEARCHES_LIFETIME` is the value that says so.
//! - **It never asks elevated.** The elevated retry is `Server Setup`'s own E2
//!   flow, offered there; what reaches this page is B11's honesty marks, which
//!   say outright that the machine could not be searched completely where any
//!   folder was refused.
//! - **It never filters the entries.** `Server Setup`'s own listing half does
//!   that, and [`crate::version`] calls it. Filtering here would take B11's
//!   naming of the candidates that did not answer with it.
//!
//! # The one thing that could have been a silent wrong answer
//!
//! **An empty list is the claim that the machine holds no Blender** — B5's own
//! second honest case, which withholds submitting and points the user at
//! `Server Setup`. **So a search that could not be run, or a machine that will
//! not name its own clock, must never come back as an empty list**, or the
//! page tells the user their machine has no Blender on it when nothing was
//! ever established. Both come back as [`TheVersionListAsk::TheMachineCouldNotBeAsked`],
//! which the trait's `None` carries and which [`crate::version`] resolves to
//! *no version resolves* — the same withholding, without the false claim.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server Setup` B10, B11, B12.
//!
//! **Four are deliberately not on that list and are written out at every
//! site instead**, because this plan defines a clause of the same name and a
//! bare one would resolve to a real and wrong clause: `Server Setup` B1,
//! `Server Setup` B9, `Server Setup` B13, `State, identity and recovery` D4
//! and `State, identity and recovery` D5.

use smoozy_server_protocol::transport::CarriesAValueOnACommandsStandardInput;
use smoozy_server_setup::readings::TheMachinesFileSurface;
use smoozy_server_setup::search::{
    the_entries, what_the_answer_says, TheSearchResult, WhenTheSearchRuns,
};
use smoozy_servers::readings::{read_the_machines_own_clock, CommandChannel};

/// **B12's moment, named rather than assumed.** This page's ask is one of
/// `Server Setup`'s own three, and it is the only one of them this crate ever
/// performs.
pub const THE_MOMENT_THIS_PAGE_ASKS: WhenTheSearchRuns =
    WhenTheSearchRuns::TheVersionListIsAskedFor;

/// **What the ask came back with.**
///
/// **There is no arm for *the machine holds no Blender*, and that is the
/// point**: that state is an answered list with no entries, which is `Server
/// Setup`'s own answer and B5's second honest case. This type separates *the
/// machine could not be asked* from it, so nothing ever presents the one as
/// the other.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheVersionListAsk {
    /// The fresh search result, with `Server Setup` B11's marks on it and
    /// that document's D4 moment in it.
    Answered(TheSearchResult),
    /// **The search did not run, or the machine would not name its own
    /// clock**, so there is no reading and no moment for one. Nothing is
    /// claimed about what the machine holds.
    TheMachineCouldNotBeAsked,
}

impl TheVersionListAsk {
    /// What [`crate::readings::Readings::the_version_list`] hands back.
    ///
    /// **`None` is *there is no list to show*, never *there is no Blender*.**
    /// [`crate::version::resolve`] reads it as no version resolving, which
    /// withholds the read and the submission exactly as B5a requires — a
    /// machine that cannot be asked is one no job can be rendered on either.
    pub fn as_the_page_reads_it(self) -> Option<TheSearchResult> {
        match self {
            TheVersionListAsk::Answered(result) => Some(result),
            TheVersionListAsk::TheMachineCouldNotBeAsked => None,
        }
    }
}

/// **B1, performed: the list asked for at B12's moment.**
///
/// The three steps are all `Server Setup`'s: its whole-disk search, one asking
/// run per candidate through the planted-link carriage, and its own
/// composition of the entries and of what the answer honestly says.
///
/// **`smoozys_own_blender_area` is passed straight through** — `Server Setup`
/// B9's exact path, or nothing on a machine this project has placed no root
/// on. It is that document's rule that Smoozy's own area is not a candidate,
/// and this ask neither applies it nor works around it.
///
/// **The moment is the machine's own clock** (`State, identity and recovery`
/// D4, with `State, identity and recovery` D5's rule that two clocks are never
/// subtracted), which is what
/// `TheSearchResult`'s own field asks for. A machine that will not name its
/// own clock has no moment, and no moment is substituted from the desktop.
///
/// **`on_the_server` is the local server identifier of the machine asked**,
/// handed straight to `Server Setup`'s asking-run, which takes its turn at the
/// one link site by it (step 10c's eleventh-pass fix piece 4c): no two asks on
/// one server are at the site at once, whichever search they belong to.
pub async fn ask_for_the_version_list(
    channel: &mut (impl CommandChannel + CarriesAValueOnACommandsStandardInput),
    files: &mut impl TheMachinesFileSurface,
    on_the_server: &str,
    smoozys_own_blender_area: Option<&str>,
) -> TheVersionListAsk {
    let found =
        smoozy_server_setup::acts::run_the_search(channel, None, smoozys_own_blender_area).await;
    if found.it_could_not_be_run.is_some() {
        return TheVersionListAsk::TheMachineCouldNotBeAsked;
    }

    let mut candidates = Vec::new();
    for path in &found.found {
        candidates.push(
            smoozy_server_setup::acts::ask_one_build_its_version(
                channel,
                files,
                on_the_server,
                path,
            )
            .await,
        );
    }
    let entries = the_entries(candidates);
    let answer = what_the_answer_says(&entries, found.folders_refused);

    let Some(at) = read_the_machines_own_clock(channel).await else {
        return TheVersionListAsk::TheMachineCouldNotBeAsked;
    };

    TheVersionListAsk::Answered(TheSearchResult {
        entries,
        answer,
        at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_security_and_licence::plaintext::Plaintext;
    use smoozy_server_protocol::transport::ssh::CommandFailure;
    use smoozy_server_protocol::transport::{CommandResult, CommandString};
    use smoozy_server_setup::blender::Version;
    use smoozy_server_setup::readings::{AListedThing, WhatTheMachineSaid};
    use smoozy_server_setup::search::the_listing;
    use std::collections::VecDeque;
    use std::future::Future;

    /// A machine that answers each command in the order they are asked.
    #[derive(Default)]
    struct AMachine {
        answers: VecDeque<Result<CommandResult, CommandFailure>>,
        sent: Vec<String>,
    }

    fn answered(status: u32, output: &str) -> Result<CommandResult, CommandFailure> {
        Ok(CommandResult {
            output: output.as_bytes().to_vec(),
            complaint: Vec::new(),
            status: Some(status),
        })
    }

    fn refused(complaint: &str) -> Result<CommandResult, CommandFailure> {
        Ok(CommandResult {
            output: Vec::new(),
            complaint: complaint.as_bytes().to_vec(),
            status: Some(1),
        })
    }

    impl CommandChannel for AMachine {
        async fn run(&mut self, command: CommandString) -> Result<CommandResult, CommandFailure> {
            self.sent.push(command.as_str().to_string());
            self.answers
                .pop_front()
                .unwrap_or(Err(CommandFailure::TheRequestDidNotReachTheMachine))
        }
    }

    impl CarriesAValueOnACommandsStandardInput for AMachine {
        fn run_carrying_a_value_on_its_standard_input(
            &mut self,
            command: CommandString,
            _value: Plaintext,
        ) -> impl Future<Output = Result<CommandResult, CommandFailure>> + Send {
            self.sent.push(command.as_str().to_string());
            std::future::ready(
                self.answers
                    .pop_front()
                    .unwrap_or(Err(CommandFailure::TheRequestDidNotReachTheMachine)),
            )
        }
    }

    impl TheMachinesFileSurface for AMachine {
        fn read_whole(
            &mut self,
            _path: &str,
        ) -> impl Future<Output = Result<Vec<u8>, WhatTheMachineSaid>> + Send {
            std::future::ready(Err(WhatTheMachineSaid::ThePathIsNotThere))
        }

        fn write_whole(
            &mut self,
            _path: &str,
            _bytes: &[u8],
        ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send {
            std::future::ready(Ok(()))
        }

        fn make_one_directory(
            &mut self,
            _path: &str,
        ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send {
            std::future::ready(Ok(()))
        }

        fn remove_one_file(
            &mut self,
            _path: &str,
        ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send {
            std::future::ready(Ok(()))
        }

        fn remove_one_empty_directory(
            &mut self,
            _path: &str,
        ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send {
            std::future::ready(Ok(()))
        }

        fn list_one_directory(
            &mut self,
            _path: &str,
        ) -> impl Future<Output = Result<Vec<AListedThing>, WhatTheMachineSaid>> + Send {
            std::future::ready(Ok(Vec::new()))
        }

        fn plant_a_link(
            &mut self,
            _target: &str,
            _at: &str,
        ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send {
            std::future::ready(Ok(()))
        }
    }

    fn block_on<F: Future>(future: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime")
            .block_on(future)
    }

    const A_CLOCK: &str = "1756800000\n";

    /// A local server identifier, as the record writes one.
    const A_SERVER: &str = "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b";

    #[test]
    fn the_list_is_the_search_result_with_one_asking_run_per_candidate() {
        let mut machine = AMachine {
            answers: VecDeque::from(vec![
                answered(0, "/opt/blender-4.5/blender\n/opt/blender-5.2/blender\n"),
                answered(0, "Blender 4.5.12 LTS\n"),
                answered(0, "Blender 5.2.0 LTS\n"),
                answered(0, A_CLOCK),
            ]),
            ..AMachine::default()
        };
        let mut files = AMachine::default();
        let TheVersionListAsk::Answered(list) = block_on(ask_for_the_version_list(
            &mut machine,
            &mut files,
            A_SERVER,
            None,
        )) else {
            panic!("the machine answered");
        };
        assert_eq!(list.entries.len(), 2);
        assert_eq!(
            list.entries[0].answered,
            Some(Version {
                major: 4,
                minor: 5,
                patch: 12
            })
        );
        assert_eq!(list.entries[0].path, "/opt/blender-4.5/blender");
        assert_eq!(
            list.entries[1].answered,
            Some(Version {
                major: 5,
                minor: 2,
                patch: 0
            })
        );
        // `Server Setup` B11's marks travel with it.
        assert!(list.answer.the_search_is_by_name);
        assert_eq!(list.answer.folders_refused, 0);
        assert!(list.answer.did_not_answer.is_empty());
    }

    #[test]
    fn a_candidate_that_did_not_answer_is_named_and_is_not_in_the_listing() {
        // `Server Setup` B11's two halves, held over this ask: the naming keeps
        // the unanswering build and the listing does not.
        let mut machine = AMachine {
            answers: VecDeque::from(vec![
                answered(0, "/opt/renamed/blender\n/opt/blender-5.2/blender\n"),
                answered(0, "no version line at all\n"),
                answered(0, "Blender 5.2.0 LTS\n"),
                answered(0, A_CLOCK),
            ]),
            ..AMachine::default()
        };
        let mut files = AMachine::default();
        let TheVersionListAsk::Answered(list) = block_on(ask_for_the_version_list(
            &mut machine,
            &mut files,
            A_SERVER,
            None,
        )) else {
            panic!("the machine answered");
        };
        assert_eq!(list.answer.did_not_answer, vec!["/opt/renamed/blender"]);
        assert_eq!(the_listing(&list.entries).len(), 1);
    }

    #[test]
    fn a_refused_folder_is_counted_and_the_machine_is_not_searched_completely() {
        let mut machine = AMachine {
            answers: VecDeque::from(vec![
                refused("find: '/root': Permission denied\n"),
                answered(0, A_CLOCK),
            ]),
            ..AMachine::default()
        };
        let mut files = AMachine::default();
        let TheVersionListAsk::Answered(list) = block_on(ask_for_the_version_list(
            &mut machine,
            &mut files,
            A_SERVER,
            None,
        )) else {
            panic!("the machine answered");
        };
        assert_eq!(list.answer.folders_refused, 1);
        assert!(!smoozy_server_setup::search::the_machine_was_searched_completely(&list.answer));
    }

    #[test]
    fn a_machine_holding_no_blender_answers_with_an_empty_list_and_not_with_an_absence() {
        // B5's second honest case: the list is empty, and that is an answer.
        let mut machine = AMachine {
            answers: VecDeque::from(vec![answered(0, "\n"), answered(0, A_CLOCK)]),
            ..AMachine::default()
        };
        let mut files = AMachine::default();
        let ask = block_on(ask_for_the_version_list(
            &mut machine,
            &mut files,
            A_SERVER,
            None,
        ));
        let TheVersionListAsk::Answered(list) = ask.clone() else {
            panic!("the machine answered");
        };
        assert!(list.entries.is_empty());
        assert!(ask.as_the_page_reads_it().is_some());
    }

    #[test]
    fn a_search_that_could_not_be_run_claims_nothing_and_is_never_an_empty_list() {
        // **The fault this arm exists to stop**: a search that never ran found
        // no build, and an answered list with no entries is the claim that the
        // machine holds no Blender — B5's second honest case, told to a user
        // about whom nothing was established.
        //
        // **The clock answers here on purpose.** A machine that answers nothing
        // at all would reach the same arm through the clock's own guard, and
        // the test would pass with the search's guard deleted.
        let mut machine = AMachine {
            answers: VecDeque::from(vec![
                Err(CommandFailure::TheRequestDidNotReachTheMachine),
                answered(0, A_CLOCK),
            ]),
            ..AMachine::default()
        };
        let mut files = AMachine::default();
        let ask = block_on(ask_for_the_version_list(
            &mut machine,
            &mut files,
            A_SERVER,
            None,
        ));
        assert_eq!(ask, TheVersionListAsk::TheMachineCouldNotBeAsked);
        assert!(ask.as_the_page_reads_it().is_none());
    }

    #[test]
    fn a_machine_that_will_not_name_its_own_clock_gives_no_list_rather_than_a_borrowed_moment() {
        // `State, identity and recovery` D5: the desktop's clock is never
        // substituted for a machine's.
        let mut machine = AMachine {
            answers: VecDeque::from(vec![
                answered(0, "/opt/blender-5.2/blender\n"),
                answered(0, "Blender 5.2.0 LTS\n"),
                answered(1, ""),
            ]),
            ..AMachine::default()
        };
        let mut files = AMachine::default();
        assert_eq!(
            block_on(ask_for_the_version_list(
                &mut machine,
                &mut files,
                A_SERVER,
                None
            )),
            TheVersionListAsk::TheMachineCouldNotBeAsked
        );
    }

    #[test]
    fn the_moment_this_page_asks_is_one_of_server_setups_own_three() {
        assert!(WhenTheSearchRuns::THE_THREE.contains(&THE_MOMENT_THIS_PAGE_ASKS));
    }
}

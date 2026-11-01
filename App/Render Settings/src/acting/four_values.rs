//! **Q14 item 2's comparison read, taken** — `Render Settings` §1.5 Q13–Q15.
//!
//! **Why it is a command and not a protocol field.** Q14 item 2: *"The read is
//! a command-channel reading — the file-transfer surface's revision 3
//! attribute structure carries no inode and no change time, the same ground
//! `Dependency Upload & Remap` Q5 route 1 names, carried at `File Browser` §13
//! item 9 — and where that channel cannot answer on a given rental, the
//! comparison is not run and nothing is claimed."*
//!
//! **The `.blend`'s path is a user-origin path, so it never enters the command
//! string.** `Server protocol and transfers` §1.1: *"no user path, package
//! path, option, identifier or other value of user, provider or machine origin
//! is ever interpolated into a command string."* It rides that document's
//! **planted-link carriage** — written as the target of a symbolic link at a
//! token fixed in the binary, and the constant command names the token.
//!
//! # The publication behind the command, read 2026-09-04
//!
//! GNU coreutils manual, *stat invocation*,
//! `https://www.gnu.org/software/coreutils/manual/html_node/stat-invocation.html`.
//!
//! - **The carriage will not work without `-L`, and this is the fact the whole
//!   reading turns on.** *"'-L' '--dereference' Change how `stat` treats
//!   symbolic links. With this option, `stat` acts on the file referenced by
//!   each symbolic link argument. Without it, `stat` acts on any symbolic link
//!   argument directly."* **Without `-L` the reading is of the link Smoozy
//!   just planted** — a real answer, at exit 0, describing the wrong file —
//!   and Q13 would then compare the link's four values against the `.blend`'s
//!   and find the file changed every time. That is a silent wrong answer, so
//!   the option is quoted rather than assumed.
//! - **The format.** *"'-c' '--format=format' Use format rather than the
//!   default format."*, with `%s` *"Total size, in bytes"*, `%i` *"Inode
//!   number"*, `%Y` *"Time of last data modification as seconds since Epoch"*
//!   and `%Z` *"Time of last status change as seconds since Epoch"* — which
//!   are `Job files and outputs` R10's four values in that document's own
//!   order.
//!
//! **Not covered by that reading, and it is the same per-image ground every
//! command-channel step stands on** (`Server protocol and transfers` §12 items
//! 3 and 11): that a rental's `stat` is coreutils' at all. **`File Browser`
//! §13 item 9 already records that which programs answer such a reading on an
//! arbitrary rental is not established**, which is exactly what
//! [`crate::readings::FourValuesNow::TheChannelCannotAnswer`] exists for.
//!
//! # Every failure lands on the arm that claims nothing
//!
//! A carriage that could not be planted, a command that did not reach the
//! machine, a command that did not end cleanly, an answer that is not four
//! readable fields — **all of them are *the channel cannot answer*, the
//! comparison is not run, and nothing is claimed** (Q14 item 2). **A `.blend`
//! that is not there lands there too**, and it is worth saying why rather than
//! reading it as a change: the closed set this document states has two arms,
//! neither of them *absent*, and concluding *the file changed* from a reading
//! that never happened is the page claiming something it does not know.
//! Nothing is lost by it — `Job files and outputs` R12's fresh check at the
//! start of every attempt catches every change the page missed (Q15).
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` §1.1, §7.8.
//! `Job files and outputs` M1, R10, R12. `File Browser` §13.
//!
//! **`Dependency Upload & Remap` Q5 is deliberately not on that list and is
//! written out at its one site**, because this plan defines a Q5 of its own
//! and a bare one would resolve to a real and wrong clause.

use smoozy_server_protocol::carriage::{a_command_naming, CarriageSite};
use smoozy_server_protocol::transfers::SourceReading;
use smoozy_server_protocol::transport::CommandString;
use smoozy_server_setup::readings::TheMachinesFileSurface;
use smoozy_servers::readings::CommandChannel;

use crate::readings::FourValuesNow;

/// **The site this reading's carriage lives at**, its token fixed in the
/// binary because a constant command has to be able to name it.
///
/// **It is this reading's own token and no other site's**, which is the
/// carriage's own rule: *"One site, one token, one carriage live at a time."*
pub const THE_BLENDS_CARRIAGE_SITE: CarriageSite = CarriageSite {
    link_path_on_the_subsystem: ".smoozy-carriage/settings-page-blend",
    named_in_a_command_as: "~/.smoozy-carriage/settings-page-blend",
};

/// **The one command this module sends**, naming the token and nothing else.
///
/// `%s %i %Y %Z` is `Job files and outputs` R10's four values in that
/// document's own order — size, inode, modification time, change time — and
/// `-L` is what makes the carriage resolve to the `.blend` rather than to the
/// link.
pub const READ_THE_FOUR_VALUES: CommandString =
    a_command_naming("stat -L -c '%s %i %Y %Z' ~/.smoozy-carriage/settings-page-blend");

/// **Q14 item 2, performed.**
///
/// **Q15: this is a use of the standing connection at a user's act.** It
/// starts no Blender, writes nothing durable, takes no operation record and is
/// not admitted at the arbitration point — the same footing as a listing.
///
/// **The link is removed when the reading ends**, and a removal that could not
/// run is an owed cleanup rather than a forgotten one (`Server protocol and
/// transfers` §7.8). The next use replaces whatever an earlier run left at the
/// token, which is the carriage's own arrangement.
///
/// **`on_the_server` is the local server identifier of the machine read**, and
/// it is the key of the site's turn: **no two readings on one server are at
/// the site at once, whoever calls** — see
/// [`smoozy_server_setup::acts::take_the_turn_at`].
pub async fn read_the_four_values(
    machine: &mut (impl CommandChannel + TheMachinesFileSurface),
    on_the_server: &str,
    the_blends_path: &str,
) -> FourValuesNow {
    // **Held from the plant to the removal**, and dropped however the reading
    // ends — the carriage's *one carriage live at a time*, per server. **The
    // turn is `Server Setup`'s shared one** (step 10c's eleventh-pass fix piece
    // 4c), which `Server Setup` B1's version ask takes too.
    let _held =
        smoozy_server_setup::acts::take_the_turn_at(THE_BLENDS_CARRIAGE_SITE, on_the_server).await;

    if smoozy_server_setup::acts::plant_the_carriage(
        machine,
        THE_BLENDS_CARRIAGE_SITE,
        the_blends_path,
    )
    .await
    .is_err()
    {
        return FourValuesNow::TheChannelCannotAnswer;
    }

    let answered = machine.run(READ_THE_FOUR_VALUES).await;

    // The removal is attempted whatever the reading did. **Its own failure is
    // never read as the reading's**, which is why it is not in the answer.
    let _ = machine
        .remove_one_file(THE_BLENDS_CARRIAGE_SITE.link_path_on_the_subsystem)
        .await;

    let Ok(result) = answered else {
        return FourValuesNow::TheChannelCannotAnswer;
    };
    if !result.ended_cleanly() {
        return FourValuesNow::TheChannelCannotAnswer;
    }
    match result.output_as_text().and_then(the_four_values_in) {
        Some(reading) => FourValuesNow::Answered(reading),
        None => FourValuesNow::TheChannelCannotAnswer,
    }
}

// **Why this reading takes turns.** Added 2026-09-30, step 10c's
// eleventh-pass fix piece 4b; the turn moved to `Server Setup`'s
// `acts::take_the_turn_at` in piece 4c, so every shared site takes turns one
// way, and that function says what a turn stops and why it is per server.
//
// **What it stops here.** Four acts read this site — the page (Q14 item 2), a
// submission (`Job files and outputs` M1), a dependency check's root and its
// pre-flight, and a dependency send at each publication — from four places in
// the desktop. Watched at the desk against OpenSSH's own `sftp-server`, a
// dependency send's reading beside a submission's came back with the
// submission's file. **Where that lands is a record**: a submission's
// `Job files and outputs` M1 identity at submission, a `Dependency Upload &
// Remap` K7 placement record that `Dependency Upload & Remap` Q5 later
// compares against, or this page's Q13 comparison. **A turn every caller has
// to remember is a turn the next caller forgets**, which is how the send's own
// turn (piece 4) left the other three out — so the reading cannot be run
// without it.

/// The four fields as the format writes them, read as a value.
///
/// **Four fields or nothing.** A short answer, an extra field, or a field that
/// is not a number is not four values, and this document has no arm for a
/// partly-read identity — Q13's comparison is over all four together.
fn the_four_values_in(answer: &str) -> Option<SourceReading> {
    let mut fields = answer.split_whitespace();
    let size = fields.next()?.parse::<u64>().ok()?;
    let inode = fields.next()?;
    let modification_time = fields.next()?.parse::<i64>().ok()?;
    let change_time = fields.next()?.parse::<i64>().ok()?;
    if fields.next().is_some() {
        return None;
    }
    // **The inode is carried as text**, which is `Server protocol and
    // transfers` S13's own shape and not a choice made here; it is still
    // required to be a number the machine wrote, so a word never becomes an
    // identity.
    inode.parse::<u64>().ok()?;
    Some(SourceReading {
        size,
        inode: inode.to_string(),
        modification_time,
        change_time,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_server_protocol::transport::ssh::CommandFailure;
    use smoozy_server_protocol::transport::CommandResult;
    use smoozy_server_setup::readings::{AListedThing, WhatTheMachineSaid};
    use std::future::Future;

    /// A machine that answers what it was told to, and records what was
    /// planted and what was removed.
    #[derive(Default)]
    struct AMachine {
        answer: Option<CommandResult>,
        planted: Vec<(String, String)>,
        removed: Vec<String>,
        the_link_cannot_be_planted: bool,
        commands_sent: Vec<String>,
    }

    impl AMachine {
        fn answering(status: u32, output: &str) -> AMachine {
            AMachine {
                answer: Some(CommandResult {
                    output: output.as_bytes().to_vec(),
                    complaint: Vec::new(),
                    status: Some(status),
                }),
                ..AMachine::default()
            }
        }
    }

    impl CommandChannel for AMachine {
        async fn run(&mut self, command: CommandString) -> Result<CommandResult, CommandFailure> {
            self.commands_sent.push(command.as_str().to_string());
            self.answer
                .clone()
                .ok_or(CommandFailure::TheRequestDidNotReachTheMachine)
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
            path: &str,
        ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send {
            self.removed.push(path.to_string());
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
            target: &str,
            at: &str,
        ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send {
            if self.the_link_cannot_be_planted {
                return std::future::ready(Err(WhatTheMachineSaid::TheLoginMayNot));
            }
            self.planted.push((target.to_string(), at.to_string()));
            std::future::ready(Ok(()))
        }
    }

    const A_BLEND: &str = "/ephemeral/smoozy/assets/kitchen.blend";

    /// A local server identifier, as the record writes one.
    const A_SERVER: &str = "0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a";

    fn block_on<F: Future>(future: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime")
            .block_on(future)
    }

    #[test]
    fn the_four_values_are_read_and_the_path_rides_the_carriage_rather_than_the_command() {
        let mut machine = AMachine::answering(0, "4096 912 1756800000 1756800001\n");
        let answer = block_on(read_the_four_values(&mut machine, A_SERVER, A_BLEND));
        assert_eq!(
            answer,
            FourValuesNow::Answered(SourceReading {
                size: 4096,
                inode: "912".to_string(),
                modification_time: 1_756_800_000,
                change_time: 1_756_800_001,
            })
        );
        // The path crossed as the link's target and never in the command.
        assert_eq!(
            machine.planted,
            vec![(
                A_BLEND.to_string(),
                THE_BLENDS_CARRIAGE_SITE
                    .link_path_on_the_subsystem
                    .to_string()
            )]
        );
        for command in &machine.commands_sent {
            assert!(!command.contains(A_BLEND), "{command}");
        }
        // §7.8: the link is removed when the reading ends.
        assert_eq!(
            machine.removed,
            vec![THE_BLENDS_CARRIAGE_SITE
                .link_path_on_the_subsystem
                .to_string()]
        );
    }

    #[test]
    fn the_command_carries_no_value_and_names_the_token() {
        // `Server protocol and transfers` §1.1, held over the one command this
        // module sends.
        let text = READ_THE_FOUR_VALUES.as_str();
        assert!(!text.contains('$'), "{text} interpolates");
        assert!(!text.contains('`'), "{text} substitutes");
        assert!(text.contains(THE_BLENDS_CARRIAGE_SITE.named_in_a_command_as));
        assert!(!text.contains(".blend"));
        // **The one option the reading turns on.** Without it `stat` acts on
        // the link Smoozy planted and answers, at exit 0, about the wrong
        // file.
        assert!(text.contains("stat -L "));
        // R10's four values, in that document's own order.
        let format = text
            .split('\'')
            .nth(1)
            .expect("the format is the quoted run");
        assert_eq!(format, "%s %i %Y %Z");
    }

    #[test]
    fn the_token_is_this_readings_own_and_is_drawn_from_class_twos_alphabet() {
        assert!(THE_BLENDS_CARRIAGE_SITE.is_drawn_from_class_twos_alphabet());
        assert!(THE_BLENDS_CARRIAGE_SITE
            .named_in_a_command_as
            .ends_with(THE_BLENDS_CARRIAGE_SITE.link_path_on_the_subsystem));
        assert_ne!(
            THE_BLENDS_CARRIAGE_SITE,
            smoozy_server_protocol::carriage::THE_FREE_SPACE_SITE
        );
    }

    #[test]
    fn every_way_the_reading_can_fail_claims_nothing() {
        // A command that did not reach the machine.
        let mut unreached = AMachine::default();
        assert_eq!(
            block_on(read_the_four_values(&mut unreached, A_SERVER, A_BLEND)),
            FourValuesNow::TheChannelCannotAnswer
        );

        // A `.blend` that is not there, and a `stat` the image does not carry:
        // both are a status this reading claims nothing from.
        for status in [1, 127] {
            let mut refused = AMachine::answering(status, "");
            assert_eq!(
                block_on(read_the_four_values(&mut refused, A_SERVER, A_BLEND)),
                FourValuesNow::TheChannelCannotAnswer
            );
        }

        // An answer that is not four readable fields.
        for answer in [
            "4096 912 1756800000\n",
            "4096 912 1756800000 1756800001 extra\n",
            "4096 not-a-number 1756800000 1756800001\n",
            "\n",
        ] {
            let mut odd = AMachine::answering(0, answer);
            assert_eq!(
                block_on(read_the_four_values(&mut odd, A_SERVER, A_BLEND)),
                FourValuesNow::TheChannelCannotAnswer,
                "{answer:?}"
            );
        }
    }

    #[test]
    fn a_carriage_that_could_not_be_planted_sends_no_command_at_all() {
        // The reading would otherwise `stat` whatever an earlier run left at
        // the token — a real answer, at exit 0, about another file.
        let mut refuses = AMachine {
            the_link_cannot_be_planted: true,
            ..AMachine::answering(0, "4096 912 1 2\n")
        };
        assert_eq!(
            block_on(read_the_four_values(&mut refuses, A_SERVER, A_BLEND)),
            FourValuesNow::TheChannelCannotAnswer
        );
        assert!(refuses.commands_sent.is_empty());
    }

    // ------------------------------------------------------------------
    // Two readings at once — step 10c's eleventh-pass fix piece 4b.
    // ------------------------------------------------------------------

    /// **One server's home, as two acts on it see it**: one link at the
    /// site, and the files it may point at.
    #[derive(Default)]
    struct AServersHome {
        the_link_points_at: Option<String>,
        files: Vec<(String, SourceReading)>,
    }

    /// **How many commands are running at one moment, across every server**,
    /// and the most there ever were.
    #[derive(Default)]
    struct AtOnce {
        now: std::sync::atomic::AtomicUsize,
        most: std::sync::atomic::AtomicUsize,
    }

    /// **One act's handle on a server**: the file operations land at once,
    /// and the command runs a round trip after it is asked — the moment
    /// another act's link can land first.
    struct AnActOn {
        home: std::sync::Arc<std::sync::Mutex<AServersHome>>,
        at_once: std::sync::Arc<AtOnce>,
    }

    const A_ROUND_TRIP: std::time::Duration = std::time::Duration::from_millis(40);

    impl CommandChannel for AnActOn {
        async fn run(&mut self, command: CommandString) -> Result<CommandResult, CommandFailure> {
            use std::sync::atomic::Ordering;
            assert_eq!(command.as_str(), READ_THE_FOUR_VALUES.as_str());
            let now = self.at_once.now.fetch_add(1, Ordering::SeqCst) + 1;
            self.at_once.most.fetch_max(now, Ordering::SeqCst);
            tokio::time::sleep(A_ROUND_TRIP).await;
            self.at_once.now.fetch_sub(1, Ordering::SeqCst);
            let home = self.home.lock().expect("the home");
            let found = home.the_link_points_at.as_ref().and_then(|target| {
                home.files
                    .iter()
                    .find(|(path, _)| path == target)
                    .map(|(_, values)| values.clone())
            });
            Ok(match found {
                Some(values) => CommandResult {
                    output: format!(
                        "{} {} {} {}\n",
                        values.size, values.inode, values.modification_time, values.change_time
                    )
                    .into_bytes(),
                    complaint: Vec::new(),
                    status: Some(0),
                },
                None => CommandResult {
                    output: Vec::new(),
                    complaint: b"No such file or directory".to_vec(),
                    status: Some(1),
                },
            })
        }
    }

    impl TheMachinesFileSurface for AnActOn {
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
            path: &str,
        ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send {
            assert_eq!(path, THE_BLENDS_CARRIAGE_SITE.link_path_on_the_subsystem);
            self.home.lock().expect("the home").the_link_points_at = None;
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

        /// **A link left at the site is replaced**, as the real one is.
        fn plant_a_link(
            &mut self,
            target: &str,
            at: &str,
        ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send {
            assert_eq!(at, THE_BLENDS_CARRIAGE_SITE.link_path_on_the_subsystem);
            self.home.lock().expect("the home").the_link_points_at = Some(target.to_string());
            std::future::ready(Ok(()))
        }
    }

    fn values(size: u64, inode: u64) -> SourceReading {
        SourceReading {
            size,
            inode: inode.to_string(),
            modification_time: 1_756_800_000,
            change_time: 1_756_800_001,
        }
    }

    const THE_KITCHEN: &str = "/ephemeral/smoozy/assets/kitchen.blend";
    const THE_STREET: &str = "/ephemeral/smoozy/assets/street.blend";

    #[test]
    fn two_readings_on_one_server_at_once_each_read_their_own_file() {
        let home = std::sync::Arc::new(std::sync::Mutex::new(AServersHome {
            the_link_points_at: None,
            files: vec![
                (THE_KITCHEN.to_string(), values(4096, 912)),
                (THE_STREET.to_string(), values(8192, 913)),
            ],
        }));
        let at_once = std::sync::Arc::new(AtOnce::default());
        let act = || AnActOn {
            home: std::sync::Arc::clone(&home),
            at_once: std::sync::Arc::clone(&at_once),
        };
        let (mut one, mut two) = (act(), act());
        // **Its own server identifier**, so no other test of this binary
        // shares its turn.
        const THIS_SERVER: &str = "1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b";

        let (kitchen, street) = block_on(async {
            tokio::join!(
                read_the_four_values(&mut one, THIS_SERVER, THE_KITCHEN),
                read_the_four_values(&mut two, THIS_SERVER, THE_STREET),
            )
        });

        assert_eq!(
            (kitchen, street),
            (
                FourValuesNow::Answered(values(4096, 912)),
                FourValuesNow::Answered(values(8192, 913)),
            ),
            "(the kitchen's reading, the street's reading)"
        );
        assert_eq!(
            at_once.most.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "one reading at the site at a time"
        );
        assert_eq!(home.lock().expect("the home").the_link_points_at, None);
    }

    #[test]
    fn readings_on_two_servers_do_not_wait_for_each_other() {
        let home = |file: &str, reading: SourceReading| {
            std::sync::Arc::new(std::sync::Mutex::new(AServersHome {
                the_link_points_at: None,
                files: vec![(file.to_string(), reading)],
            }))
        };
        let (first_home, second_home) = (
            home(THE_KITCHEN, values(4096, 912)),
            home(THE_STREET, values(8192, 913)),
        );
        let at_once = std::sync::Arc::new(AtOnce::default());
        let mut on_the_first = AnActOn {
            home: std::sync::Arc::clone(&first_home),
            at_once: std::sync::Arc::clone(&at_once),
        };
        let mut on_the_second = AnActOn {
            home: std::sync::Arc::clone(&second_home),
            at_once: std::sync::Arc::clone(&at_once),
        };

        let (kitchen, street) = block_on(async {
            tokio::join!(
                read_the_four_values(
                    &mut on_the_first,
                    "2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c",
                    THE_KITCHEN
                ),
                read_the_four_values(
                    &mut on_the_second,
                    "3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d",
                    THE_STREET
                ),
            )
        });

        assert_eq!(kitchen, FourValuesNow::Answered(values(4096, 912)));
        assert_eq!(street, FourValuesNow::Answered(values(8192, 913)));
        assert_eq!(
            at_once.most.load(std::sync::atomic::Ordering::SeqCst),
            2,
            "two servers' readings ran at once"
        );
    }
}

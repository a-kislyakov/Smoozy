//! **§6.5 performed — M14, M16 and M16a.** Producing or replacing a fixed
//! copy: the machine file opened headless by a file-inspection worker on the
//! machine, its resolved references rewritten, the result saved to a staging
//! path as a copy with the save-time relative remapping off, the fixing's own
//! report read back here, and the copy published by one rename.
//!
//! # The route, from 2026-10-04 — the 38's piece 7
//!
//! **O2, settled 2026-10-03: the fixing runs as a file-inspection worker**,
//! admitted under its own row of §9's table. **Until this piece the fixing was
//! a bare command through two planted links** to the Blender program and the
//! machine file, its script and its staging path written at fixed names under
//! the carriage directory — the route `Dependency Upload & Remap` G1 and
//! `Render Settings` Q4 rule out for a reading, and the one that crashed Blender
//! at the desk when the check's reading still took it. **Nothing called it**,
//! so no fixed copy was ever made. **It is one request to the machine's
//! supervisor now** (`smoozy_server_protocol::requests`' *a fixed copy is
//! produced*): the machine's own program writes M14's script
//! (`smoozy_dependencies_machine::fixing_script`, moved there with it) into the
//! operation's own folder and starts Blender by the build's own path, and the
//! links, the command and the fixed sites are gone.
//!
//! # The publication stays here, and why
//!
//! **The worker saves the copy to the staging path and no further.** M14's last
//! step is publication by one rename, and the copy is published **only where
//! the fixing's own report says every pair applied** — which is read back here,
//! off the worker's captured stream, by the same reading the render's material
//! is read by ([`what_the_report_said`]). A copy in which a pair did not apply
//! is removed and never published (M16a's row reads *could not be fixed*).
//!
//! **The digest of the fixed copy is not taken.** K8 records the content
//! identity of the copy the fixed copy was **derived from**, not of the fixed
//! copy itself, and M14 has no digest step; reading a whole `.blend` through
//! for a value no clause asks for is the cost this file once paid and stopped
//! paying on 2026-09-05.
//!
//! **M2 is kept the way it is kept everywhere else in this module**: nothing
//! saves over the machine file. The one save the script performs names the
//! staging path and asks Blender not to make it active.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` C4, C8.
//! `Rented-server runtime` W1.

use smoozy_render_settings::file_half::{version_text, BlenderIdentity};
use smoozy_server_protocol::framing::{Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
use smoozy_server_protocol::requests as asked;
use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_state::identities::{Identity, IdentityKind};
use tokio::io::{AsyncRead, AsyncWrite};

use super::checking::{
    asked_until_it_ends, TheMachineAReadingIsAskedOf, TheReadingDidNotRun,
    BETWEEN_TWO_ASKS_OF_ONE_READING,
};
use crate::bytes::MachineLocation;
use crate::fixing::WhyItCouldNotBeFixed;
use crate::remap::Pair;

/// **M14's script and launch line, at the path they have always been reached
/// at.** They live in `smoozy-dependencies-machine` from the 38's piece 7,
/// because the machine's own program writes the script.
pub use smoozy_dependencies_machine::fixing_script::{
    the_fixing_python, the_fixing_started_in, the_words_of_the_fixing, THE_FIXINGS_START_MARK,
};

/// **One production of one fixed copy.** M16a: the retry is this same act run
/// once more for that one file — no check re-run, nothing re-sent — so there is
/// one shape and not two.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneFixing {
    /// **The operation's own identity**, issued before the request leaves
    /// (`Server protocol and transfers` C1), and the request's identity on the
    /// wire.
    pub operation: Identity,
    /// The machine copy the fixed copy is derived from (A3's own subject).
    pub derived_from: MachineLocation,
    /// Where the fixed copy is published, from [`crate::landing::fixed_copy_address`].
    pub publishes_to: MachineLocation,
    /// Where the save writes before the rename — beside the address, so the
    /// rename does not cross a mount.
    pub staging: MachineLocation,
    pub with: BlenderIdentity,
    /// What the fixing points at — the pairs written into the copy, which is
    /// also the list K8 holds and M16's second trigger compares.
    pub pairs: Vec<Pair>,
}

impl OneFixing {
    pub fn of(
        derived_from: &MachineLocation,
        publishes_to: &MachineLocation,
        staging: &MachineLocation,
        with: &BlenderIdentity,
        pairs: Vec<Pair>,
    ) -> Option<OneFixing> {
        Some(OneFixing {
            operation: Identity::issue(IdentityKind::Operation).ok()?,
            derived_from: derived_from.clone(),
            publishes_to: publishes_to.clone(),
            staging: staging.clone(),
            with: with.clone(),
            pairs,
        })
    }

    /// **The one request, *a fixed copy is produced*.** Every path is a
    /// protocol field and never a word of a command (A8, `Security and licence`
    /// §10); the pairs cross as the one JSON document M10's material carries
    /// its own in.
    pub fn the_request(&self) -> Request {
        Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            self.operation.clone(),
            asked::THE_FIXED_COPY_IS_PRODUCED_REQUEST,
        )
        .carrying(
            Fields::none()
                .and(asked::THE_BUILD_FIELD, self.with.path.as_bytes())
                .and(asked::THE_BLEND_FIELD, self.derived_from.as_bytes())
                .and(asked::THE_BLENDER_VERSION_FIELD, version_text(self.with.version))
                .and(asked::THE_FIXINGS_STAGING_FIELD, self.staging.as_bytes())
                .and(
                    asked::THE_FIXINGS_PAIRS_FIELD,
                    serde_json::to_vec(&self.pairs)
                        .expect("the pairs are composed of values that always render"),
                ),
        )
    }
}

/// What one production came to.
///
/// **M16a: a retry that fails again leaves the row as it was**, and the row's
/// state comes from the fixing record and the publication rather than from an
/// attempt — so it can never be quietly turned into a success.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowTheFixingWent {
    /// The copy was written and published by one rename.
    Published,
    /// **Section 10: staged, saved, published by one rename, or nothing.** A
    /// production that did not reach publication leaves no fixing record and
    /// nothing at the address.
    ItCouldNotBeFixed(WhyItCouldNotBeFixed),
}

/// **M14 up to the staged copy: the worker asked until it ends, and its report
/// read back.** `Ok` only where the Blender ran to a clean ending, nobody ended
/// it, and every pair the fixing was given is reported applied.
///
/// **Nothing here reads the machine to decide anything** but the fixing's own
/// answer and its own stream: a run that did not end cleanly is a production
/// that did not publish, and nothing narrower is read off it (V15).
pub async fn produce_one_fixed_copy<M>(
    machine: &mut M,
    fixing: &OneFixing,
) -> Result<(), WhyItCouldNotBeFixed>
where
    M: TheMachineAReadingIsAskedOf,
{
    let did_not = WhyItCouldNotBeFixed::TheProductionDidNotPublish;
    let ended = asked_until_it_ends(
        machine,
        &fixing.the_request(),
        super::blender::THE_TIME_LIMIT_A_BLENDER_RUN_CARRIES,
        BETWEEN_TWO_ASKS_OF_ONE_READING,
        &mut || false,
    )
    .await
    .map_err(|_: TheReadingDidNotRun| did_not)?;
    if ended.fields.text(asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD) == Some(asked::THE_ANSWER_IS_YES)
        || ended.fields.text(asked::THE_EXIT_STATUS_FIELD) != Some("0")
    {
        return Err(did_not);
    }
    let (Some(path), Some(length)) = (
        ended.fields.value(asked::THE_ORDINARY_STREAM_FIELD),
        ended
            .fields
            .text(asked::THE_ORDINARY_STREAM_LENGTH_FIELD)
            .and_then(|text| text.parse::<u64>().ok()),
    ) else {
        return Err(did_not);
    };
    let stream = machine
        .read_the_stream_back(path, length)
        .await
        .map_err(|_| did_not)?;
    // **The fixing's own report decides whether the copy publishes.** Until
    // 2026-09-05 the run's output was discarded and the outcome judged on the
    // exit status alone, so a copy in which not one pair matched published,
    // earned a K8 record, and satisfied V11 — while every reference in it still
    // pointed at the artist's own computer.
    match what_the_report_said(&fixing.pairs, &stream) {
        Some(why) => Err(why),
        None => Ok(()),
    }
}

/// **M14's last step and M16's replacement: one rename, and the old copy
/// stands until it lands.** The replacing form is used because M16's whole
/// point is that the new copy takes the old one's place. **A rename refused
/// leaves no copy behind**: the staged one is removed, and nothing is at the
/// address but what was there before.
pub async fn the_fixed_copy_is_published<S>(
    subsystem: &mut Subsystem<S>,
    fixing: &OneFixing,
) -> HowTheFixingWent
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    match subsystem
        .posix_rename(fixing.staging.as_bytes(), fixing.publishes_to.as_bytes())
        .await
    {
        Ok(()) => HowTheFixingWent::Published,
        Err(_) => {
            let _ = subsystem.remove(fixing.staging.as_bytes()).await;
            HowTheFixingWent::ItCouldNotBeFixed(WhyItCouldNotBeFixed::TheProductionDidNotPublish)
        }
    }
}

/// **The fixing's own report, read back and held against the pairs it was
/// given.** `None` where every pair applied and the report was whole.
///
/// The reading is [`super::material::what_the_marker_said_about_the_pairs`],
/// which is public, takes exactly these two arguments and already exists for
/// the render's side — so the fixing and the render read one report one way
/// rather than two.
pub fn what_the_report_said(pairs: &[Pair], stream: &[u8]) -> Option<WhyItCouldNotBeFixed> {
    let said = super::material::what_the_marker_said_about_the_pairs(pairs, stream);
    let not_applied = said
        .per_pair
        .iter()
        .filter(|(_, line)| line.as_ref().is_some_and(|one| !one.applied))
        .count();
    if not_applied == 0 && said.every_pair_was_reported() {
        return None;
    }
    Some(WhyItCouldNotBeFixed::TheReportSaysThePairsDidNotAllApply {
        not_applied,
        never_mentioned: said.pairs_the_marker_never_mentioned,
        would_not_read_back: said.lines_that_would_not_read_back,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytes::RecordedLocation;
    use crate::check::Kind;
    use crate::remap::{compose_pair, NewLocation};

    fn a_pair() -> Pair {
        compose_pair(
            Kind::Image,
            "filepath",
            &RecordedLocation::as_the_reading_carried_it(b"//tex/METallic.png"),
            NewLocation::AMirror(MachineLocation::composed(
                b"/root/assets/files/Users/a/tex/METallic.png".to_vec(),
            )),
        )
        .expect("the pair composes")
    }

    /// **The fixing's own report decides whether the copy publishes.**
    ///
    /// **This is the test that would have caught it**: the report was written
    /// by the script and thrown away by the caller, so a fixed copy in which
    /// not one pair matched published as `Published`, earned a K8 record
    /// claiming those references were rewritten, and satisfied V11 — while
    /// every reference in it still pointed at the artist's computer.
    #[test]
    fn a_pair_that_did_not_apply_stops_the_copy_publishing_and_the_reason_survives_a_retry() {
        let pair = a_pair();
        let line = |applied: bool| {
            let carried = super::super::material::AsTheMarkerCarriesIt {
                kind: "Image".to_string(),
                setting: pair.setting.clone(),
                match_text: pair.match_text.clone(),
                applied,
                data_blocks: usize::from(applied),
                assignment_alone: crate::remap::AssignmentAlone::Established,
                reloaded: false,
                values_found: vec![],
            };
            format!(
                "{}{}\n",
                super::super::material::THE_PER_PAIR_HEAD,
                serde_json::to_string(&carried).expect("renders")
            )
        };

        // Every pair applied and the report whole: nothing stops the copy.
        assert_eq!(
            what_the_report_said(std::slice::from_ref(&pair), line(true).as_bytes()),
            None
        );

        // A pair the script reported as not applied.
        let why = what_the_report_said(std::slice::from_ref(&pair), line(false).as_bytes())
            .expect("the copy is not published");
        assert_eq!(
            why,
            WhyItCouldNotBeFixed::TheReportSaysThePairsDidNotAllApply {
                not_applied: 1,
                never_mentioned: 0,
                would_not_read_back: 0,
            }
        );
        assert!(why.is_stable_under_a_retry());

        // A pair the report never mentioned at all.
        assert_eq!(
            what_the_report_said(std::slice::from_ref(&pair), b""),
            Some(WhyItCouldNotBeFixed::TheReportSaysThePairsDidNotAllApply {
                not_applied: 0,
                never_mentioned: 1,
                would_not_read_back: 0,
            })
        );

        // A line carrying the marker's head whose body will not read back.
        let unreadable = format!("{}not json\n", super::super::material::THE_PER_PAIR_HEAD);
        let mut stream = line(true);
        stream.push_str(&unreadable);
        assert_eq!(
            what_the_report_said(std::slice::from_ref(&pair), stream.as_bytes()),
            Some(WhyItCouldNotBeFixed::TheReportSaysThePairsDidNotAllApply {
                not_applied: 0,
                never_mentioned: 0,
                would_not_read_back: 1,
            })
        );
    }

    /// **The request carries everything the worker needs, and every path as
    /// the bytes it is.**
    #[test]
    fn the_request_names_the_build_the_file_the_staging_path_and_the_pairs() {
        let fixing = OneFixing::of(
            &MachineLocation::composed(b"/root/assets/files/Users/a/lib/c\xff.blend".to_vec()),
            &MachineLocation::composed(b"/root/assets/fixed/assets/files/Users/a/lib/c.blend".to_vec()),
            &MachineLocation::composed(b"/root/assets/fixed/assets/files/Users/a/lib/.s/c.blend.fixing".to_vec()),
            &BlenderIdentity {
                path: "/opt/blender/blender".to_string(),
                version: smoozy_server_setup::blender::Version { major: 5, minor: 2, patch: 2 },
            },
            vec![a_pair()],
        )
        .expect("randomness");
        let request = fixing.the_request();
        assert_eq!(request.name, asked::THE_FIXED_COPY_IS_PRODUCED_REQUEST);
        assert_eq!(request.identity, fixing.operation);
        assert_eq!(request.fields.value(asked::THE_BLEND_FIELD), Some(fixing.derived_from.as_bytes()));
        assert_eq!(request.fields.value(asked::THE_FIXINGS_STAGING_FIELD), Some(fixing.staging.as_bytes()));
        let pairs: Vec<Pair> = serde_json::from_slice(
            request.fields.value(asked::THE_FIXINGS_PAIRS_FIELD).expect("the pairs"),
        )
        .expect("they read back");
        assert_eq!(pairs, fixing.pairs);
    }

    /// **A fixing is published only where its run ended cleanly, nobody ended
    /// it, and its report says every pair applied** — asked of a scripted
    /// machine through the one request.
    #[tokio::test]
    async fn the_staged_copy_is_good_only_where_the_run_ended_cleanly_and_every_pair_applied() {
        use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
        use smoozy_server_protocol::framing::Answered;
        struct AMachine {
            ending: Fields,
            stream: Vec<u8>,
        }
        impl TheMachineAReadingIsAskedOf for AMachine {
            async fn ask(&mut self, request: &Request) -> WhatCameBack {
                WhatCameBack::ItAnswered(
                    Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.8")
                        .carrying(self.ending.clone()),
                )
            }
            async fn read_the_stream_back(&mut self, _path: &[u8], length: u64) -> Result<Vec<u8>, String> {
                Ok(self.stream[..length as usize].to_vec())
            }
        }
        let pair = a_pair();
        let line = |applied: bool| {
            let carried = super::super::material::AsTheMarkerCarriesIt {
                kind: "Image".to_string(),
                setting: pair.setting.clone(),
                match_text: pair.match_text.clone(),
                applied,
                data_blocks: usize::from(applied),
                assignment_alone: crate::remap::AssignmentAlone::Established,
                reloaded: false,
                values_found: vec![],
            };
            format!(
                "{THE_FIXINGS_START_MARK}\n{}{}\n",
                super::super::material::THE_PER_PAIR_HEAD,
                serde_json::to_string(&carried).expect("renders")
            )
            .into_bytes()
        };
        let ending = |status: &str, by_smoozy: &str, stream: &[u8]| {
            Fields::none()
                .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_HAS_A_RECORDED_OUTCOME)
                .and(asked::THE_ORDINARY_STREAM_FIELD, b"/o".to_vec())
                .and(asked::THE_ORDINARY_STREAM_LENGTH_FIELD, stream.len().to_string())
                .and(asked::THE_EXIT_STATUS_FIELD, status)
                .and(asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD, by_smoozy)
        };
        let fixing = OneFixing::of(
            &MachineLocation::composed(b"/m/c.blend".to_vec()),
            &MachineLocation::composed(b"/f/c.blend".to_vec()),
            &MachineLocation::composed(b"/f/.s/c.blend.fixing".to_vec()),
            &BlenderIdentity {
                path: "/b".to_string(),
                version: smoozy_server_setup::blender::Version { major: 5, minor: 2, patch: 2 },
            },
            vec![pair.clone()],
        )
        .expect("randomness");
        let went = |status: &str, by_smoozy: &str, applied: bool| {
            let stream = line(applied);
            let mut machine = AMachine { ending: ending(status, by_smoozy, &stream), stream };
            let fixing = fixing.clone();
            async move { produce_one_fixed_copy(&mut machine, &fixing).await }
        };
        assert_eq!(went("0", asked::THE_ANSWER_IS_NO, true).await, Ok(()));
        assert_eq!(
            went("1", asked::THE_ANSWER_IS_NO, true).await,
            Err(WhyItCouldNotBeFixed::TheProductionDidNotPublish)
        );
        assert_eq!(
            went("0", asked::THE_ANSWER_IS_YES, true).await,
            Err(WhyItCouldNotBeFixed::TheProductionDidNotPublish)
        );
        assert!(matches!(
            went("0", asked::THE_ANSWER_IS_NO, false).await,
            Err(WhyItCouldNotBeFixed::TheReportSaysThePairsDidNotAllApply { not_applied: 1, .. })
        ));
    }
}

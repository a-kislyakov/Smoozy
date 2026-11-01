//! **The role's material, composed for `Server Setup`'s one placement act —
//! `Telegram` §20's paragraph *To `Server Setup`*, and L4's replacement
//! delivery.** Step 10 piece 5.
//!
//! §20's list is three things, and **this document decides what they hold while
//! `Server Setup` decides where and when they are placed** (`Server Setup` G1, G2, G4):
//!
//! 1. **The token file**, placed per `Security and licence` §10 — inside the
//!    package area, outside every job folder, a path that is a protocol field
//!    and never a command argument.
//! 2. **The role configuration** — the proved conversation (P1), the machine's
//!    own identity and display label, and the machine-level reporting settings
//!    (V2).
//! 3. **The durable enabled-or-disabled desired state**, set through the
//!    ordinary command path (`Rented-server runtime` W8) and **never placed as a
//!    file.** So it is named here and composed nowhere: it is a request the
//!    desktop sends over a connection, and that sending is owed.
//!
//! **A replacement is the same material placed again**, the new token file
//! replacing the old over the same route (L4). There is no second vehicle.
//!
//! **The two files' names are Build's own**, as the machine program's own
//! records' names are; the plan fixes that they sit in the package area. **What
//! reads them on the machine is step 10c's** — the role worker reading the token
//! at start and never handing it to a child (`Telegram` K2). *Corrected
//! 2026-10-16: this said step 8b, whose own correction moved the machine half to
//! step 10c.* [`TheRoleConfiguration::read_back`] is the configuration's reader,
//! added that day by the second piece of the Telegram worker's map.
//!
//! **Markers in this file are all `Telegram`'s**: P1, V2, K2, L2, L4, R8.
//! `Security and licence`'s, `Server Setup`'s and `Rented-server runtime`'s are
//! written with their documents.

use std::fmt;

use smoozy_security_and_licence::plaintext::Plaintext;

use crate::proof::TheConversation;
use crate::told::{TokenGeneration, WhatSmoozyTells};

/// The token file's name inside the package area.
pub const THE_TOKEN_FILES_NAME: &str = "telegram-token";

/// The role configuration's name inside the package area.
pub const THE_ROLE_CONFIGURATIONS_NAME: &str = "telegram-role-configuration";

/// **§20's third item, named so a reader of the material meets it**: the
/// desired state is a request on the command path and is not in
/// [`TheMaterial::the_files`].
pub const THE_DESIRED_STATE_IS_A_REQUEST_AND_NEVER_A_PLACED_FILE: bool = true;

/// **The role configuration** — §20's second item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheRoleConfiguration {
    /// P1's proved conversation.
    pub conversation: TheConversation,
    /// **The machine's own identity: the installation identity the desktop
    /// issued at hand-over**, which is unique across every installation Smoozy
    /// has placed (`State, identity and recovery` §2.3).
    pub machine: String,
    /// R8: the label, display only.
    pub label: Option<String>,
    /// `Telegram` L1/R7a: optional for material placed by an older desktop.
    pub colour: Option<String>,
    /// V2's switches, each by its stored name.
    pub reporting: Vec<(String, bool)>,
}

impl TheRoleConfiguration {
    /// **The file's bytes**: one small JSON object, fields in a fixed order.
    pub fn written_form(&self) -> Vec<u8> {
        let mut reporting: Vec<&(String, bool)> = self.reporting.iter().collect();
        reporting.sort();
        let reporting: serde_json::Map<String, serde_json::Value> = reporting
            .into_iter()
            .map(|(name, on)| (name.clone(), serde_json::Value::Bool(*on)))
            .collect();
        let object = serde_json::json!({
            "conversation": self.conversation.0,
            "machine": self.machine,
            "label": self.label,
            "colour": self.colour,
            "reporting": reporting,
        });
        serde_json::to_vec(&object).expect("a JSON value always writes")
    }

    /// **The file read back on the machine** — the mirror of
    /// [`Self::written_form`], added 2026-10-16 for the role worker, which reads
    /// it once at its start (`Telegram` K2, `Rented-server runtime` T1).
    ///
    /// **It takes exactly what the written form writes and refuses anything
    /// else rather than filling it in**: a conversation that is a whole number,
    /// a machine identity that is non-empty text, a label that is text or
    /// nothing, and the switches as an object of true-or-false values. The
    /// switches come back sorted by name, which is the order the written form
    /// puts them in. **What a refusal says never quotes the file**, so a
    /// worker can say it on its complaints stream.
    pub fn read_back(bytes: &[u8]) -> Result<TheRoleConfiguration, WhyTheConfigurationDoesNotRead> {
        let object: serde_json::Value =
            serde_json::from_slice(bytes).map_err(|_| WhyTheConfigurationDoesNotRead::ItIsNotOneJsonObject)?;
        let object = object.as_object().ok_or(WhyTheConfigurationDoesNotRead::ItIsNotOneJsonObject)?;
        let conversation = object
            .get("conversation")
            .and_then(serde_json::Value::as_i64)
            .ok_or(WhyTheConfigurationDoesNotRead::ItNamesNoConversation)?;
        let machine = object
            .get("machine")
            .and_then(serde_json::Value::as_str)
            .filter(|machine| !machine.is_empty())
            .ok_or(WhyTheConfigurationDoesNotRead::ItNamesNoMachine)?
            .to_string();
        let label = match object.get("label") {
            None | Some(serde_json::Value::Null) => None,
            Some(serde_json::Value::String(label)) => Some(label.clone()),
            Some(_) => return Err(WhyTheConfigurationDoesNotRead::ItsLabelIsNotText),
        };
        let switches = object
            .get("reporting")
            .and_then(serde_json::Value::as_object)
            .ok_or(WhyTheConfigurationDoesNotRead::ItsSwitchesAreNotOnOrOff)?;
        let mut reporting = Vec::with_capacity(switches.len());
        for (name, on) in switches {
            let on = on.as_bool().ok_or(WhyTheConfigurationDoesNotRead::ItsSwitchesAreNotOnOrOff)?;
            reporting.push((name.clone(), on));
        }
        reporting.sort();
        Ok(TheRoleConfiguration {
            conversation: TheConversation(conversation),
            machine,
            label,
            colour: object.get("colour").and_then(serde_json::Value::as_str).map(str::to_string),
            reporting,
        })
    }

    /// **What placing this file tells a machine**, as told values (L1).
    pub fn what_it_tells(&self) -> [WhatSmoozyTells; 3] {
        [
            WhatSmoozyTells::the_proved_conversation(self.conversation),
            WhatSmoozyTells::the_label_and_colour(self.label.as_deref(), self.colour.as_deref()),
            WhatSmoozyTells::the_reporting_settings(
                self.reporting.iter().map(|(name, on)| (name.as_str(), *on)),
            ),
        ]
    }
}

/// **Why a role configuration did not read back** — in words that name what
/// was wrong and never quote the file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheConfigurationDoesNotRead {
    ItIsNotOneJsonObject,
    ItNamesNoConversation,
    ItNamesNoMachine,
    ItsLabelIsNotText,
    ItsSwitchesAreNotOnOrOff,
}

impl WhyTheConfigurationDoesNotRead {
    /// The reason, in plain words.
    pub fn in_words(self) -> &'static str {
        match self {
            WhyTheConfigurationDoesNotRead::ItIsNotOneJsonObject => "it is not one JSON object",
            WhyTheConfigurationDoesNotRead::ItNamesNoConversation => "it names no conversation",
            WhyTheConfigurationDoesNotRead::ItNamesNoMachine => "it names no machine",
            WhyTheConfigurationDoesNotRead::ItsLabelIsNotText => "its label is not text",
            WhyTheConfigurationDoesNotRead::ItsSwitchesAreNotOnOrOff => {
                "its reporting settings are not each on or off"
            }
        }
    }
}

/// **Why the material was not composed.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheMaterialWasNotComposed {
    /// The package area handed in is not an absolute path Smoozy recorded:
    /// empty, relative, ending in a separator, or with a `..` step. **Nothing
    /// is guessed**, because a wrong path could land the token under a job
    /// folder or a shared temporary area (`Security and licence` §10).
    ThePackageAreaIsNotAPathSmoozyRecorded,
    /// The token held is empty.
    TheTokenIsEmpty,
}

/// **The material for one machine: two files, and the generation of the token
/// in them.** It shows nothing of the token and overwrites its bytes when it
/// goes.
pub struct TheMaterial {
    files: Vec<(String, Vec<u8>)>,
    generation: TokenGeneration,
    configuration: TheRoleConfiguration,
}

impl TheMaterial {
    /// **§20, composed.** The token arrives as [`Plaintext`] from the store and
    /// is exposed once, into the token file's bytes.
    pub fn composed(
        package_area: &str,
        token: Plaintext,
        generation: TokenGeneration,
        configuration: TheRoleConfiguration,
    ) -> Result<TheMaterial, WhyTheMaterialWasNotComposed> {
        if !is_a_path_smoozy_recorded(package_area) {
            return Err(WhyTheMaterialWasNotComposed::ThePackageAreaIsNotAPathSmoozyRecorded);
        }
        let token_bytes = token.expose(|bytes| bytes.to_vec());
        if token_bytes.is_empty() {
            return Err(WhyTheMaterialWasNotComposed::TheTokenIsEmpty);
        }
        let files = vec![
            (format!("{package_area}/{THE_TOKEN_FILES_NAME}"), token_bytes),
            (
                format!("{package_area}/{THE_ROLE_CONFIGURATIONS_NAME}"),
                configuration.written_form(),
            ),
        ];
        Ok(TheMaterial {
            files,
            generation,
            configuration,
        })
    }

    /// **The path-and-bytes pairs `Server Setup`'s placement act takes**, token
    /// file first.
    pub fn the_files(&self) -> &[(String, Vec<u8>)] {
        &self.files
    }

    pub fn the_token_files_path(&self) -> &str {
        &self.files[0].0
    }

    pub fn the_role_configurations_path(&self) -> &str {
        &self.files[1].0
    }

    pub fn the_generation(&self) -> TokenGeneration {
        self.generation
    }

    pub fn the_configuration(&self) -> &TheRoleConfiguration {
        &self.configuration
    }
}

impl fmt::Debug for TheMaterial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TheMaterial")
            .field("token file", &self.files[0].0)
            .field("role configuration", &self.files[1].0)
            .field("generation", &self.generation)
            .field("token", &"not shown")
            .finish()
    }
}

impl Drop for TheMaterial {
    fn drop(&mut self) {
        for (_, bytes) in &mut self.files {
            for byte in bytes.iter_mut() {
                // SAFETY: a plain byte written in place.
                unsafe { std::ptr::write_volatile(byte, 0) };
            }
            bytes.clear();
        }
    }
}

fn is_a_path_smoozy_recorded(path: &str) -> bool {
    path.starts_with('/')
        && path.len() > 1
        && !path.ends_with('/')
        && !path.split('/').any(|step| step == ".." || step == ".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twenty_first_colour_rides_beside_the_label_in_the_same_told_item_and_old_material_still_reads() {
        let mut configuration = a_configuration();
        configuration.colour = Some("magenta".into());
        let read = TheRoleConfiguration::read_back(&configuration.written_form()).unwrap();
        assert_eq!(read, configuration);
        assert_eq!(read.what_it_tells()[1], WhatSmoozyTells::the_label_and_colour(Some("render box"), Some("magenta")));
        let mut old: serde_json::Value = serde_json::from_slice(&configuration.written_form()).unwrap();
        old.as_object_mut().unwrap().remove("colour");
        let old = TheRoleConfiguration::read_back(&serde_json::to_vec(&old).unwrap()).unwrap();
        assert_eq!(old.colour, None);
        assert_eq!(old.what_it_tells()[1], WhatSmoozyTells::the_label(Some("render box")));
    }

    fn a_configuration() -> TheRoleConfiguration {
        TheRoleConfiguration {
            conversation: TheConversation(9001),
            machine: "0011223344556677".into(),
            colour: None,
            label: Some("render box".into()),
            reporting: vec![("a-server-has-had-nothing-to-do".into(), true)],
        }
    }

    #[test]
    fn the_material_is_two_files_in_the_package_area_and_the_token_is_in_one_of_them() {
        let material = TheMaterial::composed(
            "/mnt/one/smoozy/package",
            Plaintext::new(b"123:abc".to_vec()),
            TokenGeneration(2),
            a_configuration(),
        )
        .unwrap();
        assert_eq!(material.the_files().len(), 2, "the desired state is not a file");
        assert_eq!(material.the_token_files_path(), "/mnt/one/smoozy/package/telegram-token");
        assert_eq!(material.the_files()[0].1, b"123:abc");
        assert_eq!(
            material.the_role_configurations_path(),
            "/mnt/one/smoozy/package/telegram-role-configuration"
        );
        let configuration: serde_json::Value = serde_json::from_slice(&material.the_files()[1].1).unwrap();
        assert_eq!(configuration["conversation"], 9001);
        assert_eq!(configuration["machine"], "0011223344556677");
        assert_eq!(configuration["label"], "render box");
        assert_eq!(configuration["reporting"]["a-server-has-had-nothing-to-do"], true);
        assert!(
            !String::from_utf8_lossy(&material.the_files()[1].1).contains("abc"),
            "the token is in the token file alone"
        );
        assert!(!format!("{material:?}").contains("abc"), "the material shows nothing of the token");
        assert_eq!(material.the_generation(), TokenGeneration(2));
        const { assert!(THE_DESIRED_STATE_IS_A_REQUEST_AND_NEVER_A_PLACED_FILE) };
    }

    #[test]
    fn a_package_area_that_is_not_a_recorded_path_is_refused_rather_than_guessed() {
        for bad in ["", "relative/package", "/", "/mnt/one/", "/mnt/../tmp", "/mnt/./x"] {
            assert_eq!(
                TheMaterial::composed(bad, Plaintext::new(b"1:a".to_vec()), TokenGeneration(1), a_configuration())
                    .unwrap_err(),
                WhyTheMaterialWasNotComposed::ThePackageAreaIsNotAPathSmoozyRecorded,
                "{bad:?}"
            );
        }
        assert_eq!(
            TheMaterial::composed("/p", Plaintext::new(Vec::new()), TokenGeneration(1), a_configuration()).unwrap_err(),
            WhyTheMaterialWasNotComposed::TheTokenIsEmpty
        );
    }

    /// **The reader is the written form's mirror**: what is written reads back
    /// as itself, with or without a label, and anything the written form would
    /// not have written is refused by name and never filled in. Added
    /// 2026-10-16.
    #[test]
    fn the_configuration_reads_back_as_it_was_written_and_nothing_else_reads() {
        let written = a_configuration();
        assert_eq!(TheRoleConfiguration::read_back(&written.written_form()), Ok(written));
        let unlabelled = TheRoleConfiguration {
            colour: None,
            label: None,
            reporting: vec![("a".into(), false), ("b".into(), true)],
            ..a_configuration()
        };
        assert_eq!(TheRoleConfiguration::read_back(&unlabelled.written_form()), Ok(unlabelled));
        for (bytes, why) in [
            (&b"not json"[..], WhyTheConfigurationDoesNotRead::ItIsNotOneJsonObject),
            (b"[1]", WhyTheConfigurationDoesNotRead::ItIsNotOneJsonObject),
            (br#"{"machine":"m","label":null,"reporting":{}}"#, WhyTheConfigurationDoesNotRead::ItNamesNoConversation),
            (br#"{"conversation":"9001","machine":"m","label":null,"reporting":{}}"#, WhyTheConfigurationDoesNotRead::ItNamesNoConversation),
            (br#"{"conversation":9001,"machine":"","label":null,"reporting":{}}"#, WhyTheConfigurationDoesNotRead::ItNamesNoMachine),
            (br#"{"conversation":9001,"machine":"m","label":4,"reporting":{}}"#, WhyTheConfigurationDoesNotRead::ItsLabelIsNotText),
            (br#"{"conversation":9001,"machine":"m","label":null}"#, WhyTheConfigurationDoesNotRead::ItsSwitchesAreNotOnOrOff),
            (br#"{"conversation":9001,"machine":"m","label":null,"reporting":{"x":"yes"}}"#, WhyTheConfigurationDoesNotRead::ItsSwitchesAreNotOnOrOff),
        ] {
            assert_eq!(TheRoleConfiguration::read_back(bytes), Err(why), "{}", String::from_utf8_lossy(bytes));
            assert!(!why.in_words().contains("9001"), "the words never quote the file");
        }
    }

    #[test]
    fn the_configuration_tells_the_conversation_the_label_and_the_switches() {
        let tells = a_configuration().what_it_tells();
        assert_eq!(tells[0], WhatSmoozyTells::the_proved_conversation(TheConversation(9001)));
        assert_eq!(tells[1], WhatSmoozyTells::the_label(Some("render box")));
        assert_eq!(
            tells[2],
            WhatSmoozyTells::the_reporting_settings([("a-server-has-had-nothing-to-do", true)])
        );
    }
}

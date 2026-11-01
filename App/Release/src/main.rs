//! **`smoozy-release` — the acts of making a version of Smoozy that a person
//! cannot do by hand.**
//!
//! Everything else a release does is `git`, the hosted runner, `cargo`, the
//! shell's own build command and `gh`, and the release runner runs those
//! itself. **What it cannot do by hand is compose and sign a release, check a
//! published one, and sign and notarize the Mac bundle.** Those are here.
//! *(It said "the two acts" until 2026-10-24, when D2 added the third.)*
//!
//! ```text
//! smoozy-release compose
//!     --into <folder>                  where to write the release, and it must be empty
//!     --release-sequence <number>      B8's monotonically increasing sequence
//!     --package-version <version>      the human version
//!     --machine-program <path>         the smoozy-server binary from the Linux runner
//!     --source <https address>         an exact source the manifest names (repeatable)
//!     --also <name>=<path>             a desktop artifact to publish and list (repeatable)
//!     --update-row <row>=<name>        which --also file a release-matrix row is offered
//!                                      in the desktop update file (one per row, all five)
//!     --key-set-generation <number>    default 1
//!
//! smoozy-release check
//!     --release <https address>        where the release file was published
//!     --accepted-generation <number>       check as a desktop that has accepted
//!     --accepted-release-sequence <number> this key-set generation and this
//!                                          release sequence (both, or neither)
//!
//! smoozy-release sign-the-mac-bundle
//!     --bundle <path to Smoozy.app>    the bundle the shell built at the desk
//!     --into <path to a .zip>          the Mac download to write; it must not exist
//! ```
//!
//! **`compose` also writes the desktop update file, `desktop-update`, from
//! 2026-10-25** (`Desktop app and delivery` E10a, E15 and E19, step 11's D8):
//! the manifest naming each row's artifact, signed by the everyday key read
//! from the keychain's second entry, with every trust statement the master keys
//! have signed. **Until the build carries the master keys it writes none and
//! says so**; from that day, a missing everyday key stops the release. **`check`
//! also fetches the update file from the fixed address every copy reads it
//! from** and believes it only through the desktop's own reading. Both are
//! `the_desktop_update.rs`.
//!
//! **`sign-the-mac-bundle` is the third act** (`Desktop app and delivery` E2 and
//! E6, added 2026-10-24 by step 11's D2): it signs the bundle with the
//! Creator's Developer ID, has Apple notarize it, staples the ticket, and only
//! then writes the Mac download that `compose --also` takes. **Where the
//! identity or the stored notarization credential is missing it stops before
//! touching the bundle.** It is `the_mac_bundle.rs`.
//!
//! **`compose` writes nothing unless the release verifies**, so a release that
//! fails its verification leaves no folder on the disk holding something that
//! should not be published. **A write that fails part way through is the other
//! case and it is not glossed**: the machine program, each desktop artifact, the
//! release file and the listing are written one after another, so a failure
//! after some have landed leaves them there. **The ending says which of the two
//! happened and what the folder holds**, and nothing is deleted — removing files
//! after a failure is a second thing that can fail.
//!
//! **It never prints the signing secret**, in full or in part. The only thing it
//! says about the secret is whether the keychain had it and whether it was the
//! release root this build carries.
//!
//! **Where the two numbers come from is written down and is not derived here.**
//! `Audits/Build/Step 8a/Decisions record.md` holds the rule: the human version
//! is `App/Cargo.toml`'s workspace version, and the release sequence is the
//! commit count of the commit being released. **Both are values the release
//! runner passes in**, because a release must be reproducible from what was
//! typed rather than from what a tool happened to read.

mod making;
mod outside;
mod secret;
mod the_desktop_update;
mod the_mac_bundle;

use making::{AnArtifactToPublish, TheDesktopUpdate, WhatToRelease};
use smoozy_desktop::updating::trust::TheCarriedSet;

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let ending = match arguments.first().map(String::as_str) {
        Some("compose") => compose(&arguments[1..]),
        Some("check") => check(&arguments[1..]),
        Some("sign-the-mac-bundle") => sign_the_mac_bundle(&arguments[1..]),
        _ => {
            eprintln!("{}", HOW_TO_USE_IT);
            1
        }
    };
    std::process::exit(ending);
}

const HOW_TO_USE_IT: &str = "\
smoozy-release compose --into <folder> --release-sequence <n> --package-version <v> \\
               --machine-program <path> --source <https address> \\
               [--also <name>=<path>]... [--update-row <row>=<name>]... \\
               [--key-set-generation <n>]

smoozy-release check --release <https address> \\
             [--accepted-generation <n> --accepted-release-sequence <n>]

smoozy-release sign-the-mac-bundle --bundle <Smoozy.app> --into <Smoozy-macos.zip>";

// ------------------------------------------------------------------ compose

fn compose(arguments: &[String]) -> i32 {
    let read = match read_the_arguments(arguments) {
        Ok(read) => read,
        Err(said) => {
            eprintln!("{said}\n\n{HOW_TO_USE_IT}");
            return 1;
        }
    };

    let Some(into) = read.one("--into") else {
        eprintln!("--into is missing");
        return 1;
    };
    let Some(sequence) = read.one("--release-sequence").and_then(|it| it.parse::<u64>().ok())
    else {
        eprintln!("--release-sequence is missing or is not a whole number");
        return 1;
    };
    let Some(version) = read.one("--package-version") else {
        eprintln!("--package-version is missing");
        return 1;
    };
    let Some(program) = read.one("--machine-program") else {
        eprintln!("--machine-program is missing");
        return 1;
    };
    let generation = match read.one("--key-set-generation") {
        None => 1,
        Some(it) => match it.parse::<u64>() {
            Ok(it) => it,
            Err(_) => {
                eprintln!("--key-set-generation is not a whole number");
                return 1;
            }
        },
    };

    let mut also = Vec::new();
    for pair in read.every("--also") {
        let Some((name, path)) = pair.split_once('=') else {
            eprintln!("--also wants <name>=<path> and was given {pair}");
            return 1;
        };
        also.push(AnArtifactToPublish {
            name: name.to_owned(),
            found_at: path.into(),
            what_it_is: "the desktop application, unsigned and with no update manifest".into(),
        });
    }

    let mut rows = Vec::new();
    for pair in read.every("--update-row") {
        let Some((row, name)) = pair.split_once('=') else {
            eprintln!("--update-row wants <row>=<name> and was given {pair}");
            return 1;
        };
        rows.push((row.to_owned(), name.to_owned()));
    }

    let what = WhatToRelease {
        release_sequence: sequence,
        package_version: version.to_owned(),
        key_set_generation: generation,
        the_machine_program: AnArtifactToPublish {
            name: smoozy_server_protocol::release::composing::THE_FIXED_ENTRY_POINT_NAME.to_owned(),
            found_at: program.into(),
            what_it_is: "the rented machine's own program".into(),
        },
        its_exact_sources: read.every("--source").into_iter().map(str::to_owned).collect(),
        the_desktop_artifacts: also,
        the_rows: rows,
    };

    // **The secret is read here and it lives for the length of this block.**
    let (secret, root) = match secret::read_the_signing_secret_from_the_keychain() {
        Ok(both) => both,
        Err(why) => {
            eprintln!("{}", why_the_secret_was_not_obtained(&why));
            return 1;
        }
    };
    println!(
        "The signing secret is in the keychain and it is the release root this build carries."
    );

    // **The everyday key, read only where the build carries the master keys**
    // — before that day no copy could believe an update file, so none is made
    // and the keychain's second entry is not asked for.
    let carried = TheCarriedSet::this_build();
    let everyday = if the_desktop_update::this_build_can_be_updated(&carried) {
        match secret::read_the_update_signing_key_from_the_keychain(&carried, sequence) {
            Ok(key) => {
                println!(
                    "The everyday key is in the keychain and it is the online key {} this build's \
                     trust statement allows.",
                    key.identifier()
                );
                Some(key)
            }
            Err(why) => {
                eprintln!("{}", why_the_update_key_was_not_obtained(&why));
                return 1;
            }
        }
    } else {
        println!("{NO_UPDATE_FILE_IS_WRITTEN}");
        None
    };
    let update = match &everyday {
        Some(key) => TheDesktopUpdate::Written {
            carried: &carried,
            earlier: the_desktop_update::THE_EARLIER_TRUST_STATEMENTS,
            key,
        },
        None => TheDesktopUpdate::NotWritten,
    };

    let made = match making::make_a_release(&what, &secret, &root, update, std::path::Path::new(into)) {
        Ok(made) => made,
        Err(why) => {
            eprintln!("{}", how_the_release_ended(&why));
            return 1;
        }
    };
    drop(secret);
    drop(everyday);

    println!(
        "\nThe release verified through the desktop's own path and is written into {}.",
        made.written_into.display()
    );
    println!("  manifest signed by: {}", made.signed_by);
    println!("  manifest digest:    {}", made.the_manifests_digest);
    println!("\nWhat is in the folder:");
    for (name, length, digest) in &made.files {
        println!("  {name}\n    bytes: {length}\n    sha-256: {digest}");
    }
    if let Some(offered) = &made.the_desktop_update {
        println!(
            "\nThe desktop update file verified through the desktop's own reading, for every row:"
        );
        println!("  release sequence: {}", offered.release_sequence);
        println!("  version:          {}", offered.version);
        println!("  signed by:        {}", offered.signed_by.join(", "));
        for entry in &offered.entries {
            println!("  {}: {}", entry.row, entry.file_name);
        }
    }
    println!(
        "\nNothing has been published. Publishing is the release runner's sixth step and it is \
         the one outward act."
    );
    0
}

/// **How the release ended, worded by what actually happened.**
///
/// **There are two endings and they used to be one.** A release that did not
/// verify wrote nothing, and that is still true and still said. **A release that
/// verified and then failed part way through the writing leaves files on the
/// disk**, and saying *nothing was written* about that sent a release runner to
/// run the same command again, be refused for the folder not being empty, and
/// work out why.
///
/// **Nothing is deleted here and nothing should be.** Removing files after a
/// failure is a second thing that can fail, and it takes the evidence with it.
fn how_the_release_ended(why: &making::WhyTheReleaseWasNotMade) -> String {
    match why {
        making::WhyTheReleaseWasNotMade::ItCouldNotBeWritten {
            and_the_folder_now_holds,
            ..
        } => {
            let holds = if and_the_folder_now_holds.is_empty() {
                "  The folder could not be read, so what is in it is not known from here.\n"
                    .to_owned()
            } else {
                let mut out = String::from("  The folder now holds:\n");
                for name in and_the_folder_now_holds {
                    out.push_str(&format!("    {name}\n"));
                }
                out
            };
            format!(
                "The release verified and writing it failed part way through, so the folder is \
                 not empty and what is in it must not be published.\n  {why}\n{holds}  Nothing \
                 has been deleted: removing files after a failure is a second thing that can \
                 fail. Clear the folder by hand before running this again."
            )
        }
        _ => format!("The release was not made, and nothing was written.\n  {why}"),
    }
}

/// **What `compose` and `check` say while the build carries no update master
/// key.** One sentence, because it is the ordinary case until the Creator's keys
/// are written in, and nothing is wrong.
const NO_UPDATE_FILE_IS_WRITTEN: &str = "No desktop update file is written or checked: this build \
     carries no update master key, so no copy of Smoozy could believe one. That changes the day \
     the Creator's two master keys are written into the build.";

/// **Why the everyday key was not obtained, in the release runner's words.**
/// Every ending stops the release: from the day the build carries the master
/// keys, a release without its update file is not made.
fn why_the_update_key_was_not_obtained(why: &secret::WhyTheUpdateKeyWasNotObtained) -> String {
    use secret::WhyTheSecretWasNotObtained::*;
    use secret::WhyTheUpdateKeyWasNotObtained::*;
    let service = secret::THE_UPDATE_KEYCHAIN_SERVICE;
    let account = secret::THE_UPDATE_KEYCHAIN_ACCOUNT;
    match why {
        ThisBuildCarriesNoTrustStatement => "This build carries the update master keys and no \
             trust statement naming an everyday key, so nothing can be signed for it and the \
             release was not made. The build is broken: the statement goes in with the keys."
            .to_owned(),
        TheKeychain(ThereIsNoSuchEntry) => format!(
            "The everyday key is not in this Mac's keychain, under the service {service:?} and the \
             account {account:?}, so the desktop update file cannot be signed and the release was \
             not made. Stop and tell the Creator: the master copy is in their password manager, \
             under Smoozy update everyday key."
        ),
        TheKeychain(TheEntryWasNotHandedOver {
            the_tools_own_ending,
            said,
        }) => {
            let ending = match the_tools_own_ending {
                Some(ending) => format!("ended at {ending}"),
                None => "was ended by a signal".to_owned(),
            };
            let words = if said.is_empty() {
                "It said nothing.".to_owned()
            } else {
                format!("It said: {said}")
            };
            format!(
                "The keychain did not hand over the everyday key, under the service {service:?} and \
                 the account {account:?}, so the desktop update file cannot be signed and the \
                 release was not made.\n  The entry may be there and the read may have been \
                 cancelled or refused, or the entry may not be there at all. This build cannot \
                 tell those apart, and does not guess.\n  The keychain tool {ending}. {words}\n  \
                 The master copy is in the Creator's password manager, under Smoozy update everyday \
                 key."
            )
        }
        TheKeychain(ItIsNotThirtyTwoBytesOfHexadecimal) => format!(
            "The keychain entry {service:?} is there and what it holds is not a key: it should be \
             thirty-two bytes written as sixty-four hexadecimal characters. The release was not \
             made."
        ),
        TheKeychain(TheKeychainCouldNotBeAsked { said }) => format!(
            "The keychain could not be asked for the everyday key, {service:?}, at all: {said}. A \
             release is made on the Mac. The release was not made."
        ),
        // **The release root's own two reasons are never reached for the
        // everyday key**, whose proof is against the trust statement; worded
        // anyway rather than left to panic.
        TheKeychain(ItIsNotTheReleaseRootThisBuildCarries)
        | TheKeychain(ThisBuildCarriesNoReleaseRoot) => format!(
            "The everyday key under {service:?} was judged as if it were the release root, which \
             this build never does. Nothing was signed and the release was not made."
        ),
        ItIsNoOnlineKeyThisBuildAllows => format!(
            "The keychain entry {service:?} holds a key, and it is no everyday key this build's \
             trust statement puts in force at this release sequence, so nothing it signed would be \
             believed by any copy of Smoozy. Nothing was signed and the release was not made. \
             Either the everyday key was replaced and this build was not made again from a source \
             carrying the new statement, or the entry holds something else."
        ),
    }
}

fn why_the_secret_was_not_obtained(why: &secret::WhyTheSecretWasNotObtained) -> String {
    use secret::WhyTheSecretWasNotObtained::*;
    match why {
        ThereIsNoSuchEntry => format!(
            "The signing secret is not in this Mac's keychain, under the service {:?} and the \
             account {:?}. Stop and tell the Creator: the master copy is in their password \
             manager.",
            secret::THE_KEYCHAIN_SERVICE,
            secret::THE_KEYCHAIN_ACCOUNT
        ),
        TheEntryWasNotHandedOver {
            the_tools_own_ending,
            said,
        } => {
            let ending = match the_tools_own_ending {
                Some(ending) => format!("ended at {ending}"),
                None => "was ended by a signal".to_owned(),
            };
            let words = if said.is_empty() {
                "It said nothing.".to_owned()
            } else {
                format!("It said: {said}")
            };
            format!(
                "The keychain did not hand over the signing secret, under the service {:?} and \
                 the account {:?}. Nothing was signed.\n  The entry may be there and the read may \
                 have been cancelled or refused — it is stored with its access restricted, so a \
                 read asks for the login password — or the entry may not be there at all. This \
                 build cannot tell those apart, and does not guess.\n  The keychain tool {ending}. \
                 {words}\n  The master copy is in the Creator's password manager.",
                secret::THE_KEYCHAIN_SERVICE,
                secret::THE_KEYCHAIN_ACCOUNT
            )
        }
        ItIsNotTheReleaseRootThisBuildCarries =>
            "The keychain entry is there and it is not the release root this build carries, so \
             nothing it signed could be verified by any copy of Smoozy. Nothing was signed. \
             Either the key was replaced and this build was not made again from a source that \
             carries the new one, or the entry belongs to something else."
                .to_owned(),
        ItIsNotThirtyTwoBytesOfHexadecimal =>
            "The keychain entry is there and what it holds is not a key: it should be thirty-two \
             bytes written as hexadecimal."
                .to_owned(),
        ThisBuildCarriesNoReleaseRoot =>
            "This build carries no release root, so nothing it signed could be verified."
                .to_owned(),
        TheKeychainCouldNotBeAsked { said } => format!(
            "The keychain could not be asked at all: {said}. A release is made on the Mac."
        ),
    }
}

// -------------------------------------------------------------------- check

fn check(arguments: &[String]) -> i32 {
    let read = match read_the_arguments(arguments) {
        Ok(read) => read,
        Err(said) => {
            eprintln!("{said}\n\n{HOW_TO_USE_IT}");
            return 1;
        }
    };
    let Some(address) = read.one("--release") else {
        eprintln!("--release is missing");
        return 1;
    };

    // **The floor is typed or it is absent, and both numbers go together.** A
    // check run with one of them would be checking against a floor nobody holds.
    let generation = read.one("--accepted-generation").map(str::parse::<u64>);
    let sequence = read.one("--accepted-release-sequence").map(str::parse::<u64>);
    let accepted = match (generation, sequence) {
        (None, None) => None,
        (Some(Ok(key_set_generation)), Some(Ok(release_sequence))) => {
            Some(outside::AsADesktopThatHasAccepted {
                key_set_generation,
                release_sequence,
            })
        }
        (Some(Err(_)), _) | (_, Some(Err(_))) => {
            eprintln!("--accepted-generation and --accepted-release-sequence take whole numbers");
            return 1;
        }
        _ => {
            eprintln!(
                "--accepted-generation and --accepted-release-sequence go together: a check \
                 against one of them alone is a check against a floor no desktop holds"
            );
            return 1;
        }
    };

    let mut fetch = outside::AFetchByRunningCurl::new();
    let the_server_package = check_the_server_package(address, &mut fetch, accepted);
    let the_desktop_update = check_the_desktop_update(&mut fetch);
    if the_server_package == 0 && the_desktop_update == 0 {
        0
    } else {
        1
    }
}

/// **The desktop update file, from its fixed address, through the desktop's own
/// reading** — or one sentence where the build carries no master key.
fn check_the_desktop_update(fetch: &mut outside::AFetchByRunningCurl) -> i32 {
    let carried = TheCarriedSet::this_build();
    if !the_desktop_update::this_build_can_be_updated(&carried) {
        println!("\n{NO_UPDATE_FILE_IS_WRITTEN}");
        return 0;
    }
    match outside::check_the_published_update_file(
        &carried,
        the_desktop_update::THE_EARLIER_TRUST_STATEMENTS,
        fetch,
    ) {
        Ok(found) => {
            println!(
                "\nThe published desktop update file verifies through the desktop's own reading, \
                 for every row, as a copy of Smoozy that has accepted nothing yet."
            );
            println!("  address:          {}", outside::WHERE_THE_DESKTOP_READS_ITS_UPDATE_FILE);
            println!("  release sequence: {}", found.offered.release_sequence);
            println!("  version:          {}", found.offered.version);
            println!("  signed by:        {}", found.offered.signed_by.join(", "));
            println!("  every artifact it names was fetched and has its row's length and digest:");
            for address in &found.fetched {
                println!("    {address}");
            }
            0
        }
        Err(why) => {
            eprintln!("\nThe published desktop update file did not check out.\n  {why}");
            1
        }
    }
}

/// The server package's release, checked from the outside.
fn check_the_server_package(
    address: &str,
    fetch: &mut outside::AFetchByRunningCurl,
    accepted: Option<outside::AsADesktopThatHasAccepted>,
) -> i32 {
    match outside::check_a_published_release(address, fetch, accepted) {
        Ok(found) => {
            println!("The published release verifies from the outside.");
            println!("  release sequence: {}", found.release_sequence);
            println!("  version:          {}", found.package_version);
            println!("  signed by:        {}", found.signed_by);
            match accepted {
                // *(Until 2026-09-21 this said such a copy "refuses nothing on a
                // number", which was true while the build's floor pair was zero
                // and stopped being true when it was raised.)*
                None => println!(
                    "  checked as a copy of Smoozy that has never accepted a release, held \
                     only to the floor this build carries: key-set generation {} and release \
                     sequence {}. The ordinary user may hold a higher one: give \
                     --accepted-generation and --accepted-release-sequence to check as one.",
                    smoozy_server_protocol::release::trust::THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
                    smoozy_server_protocol::release::trust::THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE
                ),
                Some(accepted) => println!(
                    "  checked as a desktop that has accepted generation {} and release \
                     sequence {}.",
                    accepted.key_set_generation, accepted.release_sequence
                ),
            }
            println!(
                "  every source the entry names was fetched, and every one of them served {} \
                 bytes with the digest the manifest carries:",
                found.artifact_byte_length
            );
            for source in &found.the_sources_the_artifact_was_fetched_from {
                println!("    {source}");
            }
            println!(
                "\nWhat is not checked here: B10's third verification, the read-back of the \
                 staged artifact at rest on a rented machine. That wants a rental."
            );
            0
        }
        Err(why) => {
            eprintln!("The published release did not check out.\n  {why}");
            1
        }
    }
}

// ------------------------------------------------------- sign-the-mac-bundle

fn sign_the_mac_bundle(arguments: &[String]) -> i32 {
    let read = match read_the_arguments(arguments) {
        Ok(read) => read,
        Err(said) => {
            eprintln!("{said}\n\n{HOW_TO_USE_IT}");
            return 1;
        }
    };
    let Some(bundle) = read.one("--bundle") else {
        eprintln!("--bundle is missing");
        return 1;
    };
    let Some(into) = read.one("--into") else {
        eprintln!("--into is missing");
        return 1;
    };

    let mut tools = the_mac_bundle::TheToolsOnThisMac;
    let made = the_mac_bundle::sign_and_notarize(
        std::path::Path::new(bundle),
        std::path::Path::new(into),
        &mut tools,
        &mut |line: String| println!("{line}"),
    );
    match made {
        Ok(made) => {
            println!("\nApple's log for the submission, to read for warnings:");
            println!("{}", made.the_log.trim_end());
            println!(
                "\nThe Mac download is written, and it is the only file of this step that may be \
                 published.\n  {}\n  signed by: {}\n  submission: {}\n  bytes: {}\n  sha-256: {}",
                made.the_download.display(),
                made.signed_by,
                made.the_submission,
                made.its_length,
                made.its_digest
            );
            println!(
                "\nNothing has been published. The download goes into compose with --also, under \
                 the name the release runner gives it."
            );
            0
        }
        Err(why) => {
            eprintln!("{}", the_mac_bundle::in_words(&why));
            1
        }
    }
}

// ---------------------------------------------------------------- arguments

/// **The arguments, read once.**
///
/// Small on purpose: every argument this tool takes is `--name value`, some may
/// be given more than once, and nothing is positional. That is the whole shape,
/// so it needs no library.
struct TheArguments<'a> {
    pairs: Vec<(&'a str, &'a str)>,
}

impl<'a> TheArguments<'a> {
    fn one(&self, name: &str) -> Option<&'a str> {
        self.pairs
            .iter()
            .find(|(given, _)| *given == name)
            .map(|(_, value)| *value)
    }

    fn every(&self, name: &str) -> Vec<&'a str> {
        self.pairs
            .iter()
            .filter(|(given, _)| *given == name)
            .map(|(_, value)| *value)
            .collect()
    }
}

fn read_the_arguments(arguments: &[String]) -> Result<TheArguments<'_>, String> {
    let mut pairs = Vec::new();
    let mut at = 0;
    while at < arguments.len() {
        let name = arguments[at].as_str();
        if !name.starts_with("--") {
            return Err(format!("{name} is not an argument this tool takes"));
        }
        let Some(value) = arguments.get(at + 1) else {
            return Err(format!("{name} was given no value"));
        };
        pairs.push((name, value.as_str()));
        at += 2;
    }
    Ok(TheArguments { pairs })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn given(what: &[&str]) -> Vec<String> {
        what.iter().map(|it| (*it).to_owned()).collect()
    }

    #[test]
    fn arguments_are_read_as_pairs_and_a_repeated_one_keeps_every_value() {
        let arguments = given(&[
            "--into", "somewhere", "--source", "https://a", "--source", "https://b",
        ]);
        let read = read_the_arguments(&arguments).unwrap();
        assert_eq!(read.one("--into"), Some("somewhere"));
        assert_eq!(read.every("--source"), vec!["https://a", "https://b"]);
        assert_eq!(read.one("--missing"), None);
        assert!(read.every("--missing").is_empty());
    }

    #[test]
    fn an_argument_with_no_value_and_a_stray_word_are_both_refused() {
        assert!(read_the_arguments(&given(&["--into"])).is_err());
        assert!(read_the_arguments(&given(&["into", "somewhere"])).is_err());
    }

    #[test]
    fn every_reason_the_secret_was_not_obtained_says_something_of_its_own() {
        use secret::WhyTheSecretWasNotObtained::*;
        let said: Vec<String> = [
            ThereIsNoSuchEntry,
            TheEntryWasNotHandedOver {
                the_tools_own_ending: Some(51),
                said: "authorization denied".into(),
            },
            ItIsNotTheReleaseRootThisBuildCarries,
            ItIsNotThirtyTwoBytesOfHexadecimal,
            ThisBuildCarriesNoReleaseRoot,
            TheKeychainCouldNotBeAsked {
                said: "no such tool".into(),
            },
        ]
        .iter()
        .map(why_the_secret_was_not_obtained)
        .collect();
        // **Every ending says something, and no two say the same thing.** The
        // release runner has to be able to tell a missing entry from a wrong
        // one, and it is told in the Creator's terms rather than in an
        // enumeration's.
        assert!(said.iter().all(|it| it.len() > 40));
        for at in 0..said.len() {
            for beside in at + 1..said.len() {
                assert_ne!(said[at], said[beside]);
            }
        }
        // **None of these values carries key material to leak**, which is the
        // structural half of never printing the secret: the reasons are an
        // enumeration whose one field is a message from the keychain tool.
        assert!(!said.iter().any(|it| it.contains("no such tool") && it.len() > 200));
    }

    /// **A write that failed part way through is never reported as *nothing was
    /// written***, and the ending names what the folder holds.
    #[test]
    fn a_write_that_failed_part_way_says_so_and_names_what_the_folder_holds() {
        let said = how_the_release_ended(&making::WhyTheReleaseWasNotMade::ItCouldNotBeWritten {
            said: "listing.txt: no space left on device".into(),
            and_the_folder_now_holds: vec![
                "Smoozy-macos.zip".into(),
                "release".into(),
                "smoozy-server".into(),
            ],
        });
        assert!(
            !said.contains("nothing was written"),
            "a part-written release was reported as having written nothing"
        );
        assert!(said.contains("failed part way through"));
        assert!(said.contains("must not be published"));
        for name in ["Smoozy-macos.zip", "release", "smoozy-server"] {
            assert!(said.contains(name), "{name} was not named");
        }
        assert!(said.contains("Nothing has been deleted"));
    }

    /// **A folder that could not be read says so** rather than reading as an
    /// empty folder.
    #[test]
    fn a_folder_that_could_not_be_read_is_said_rather_than_called_empty() {
        let said = how_the_release_ended(&making::WhyTheReleaseWasNotMade::ItCouldNotBeWritten {
            said: "release: no space left on device".into(),
            and_the_folder_now_holds: Vec::new(),
        });
        assert!(said.contains("could not be read"));
        assert!(!said.contains("nothing was written"));
    }

    /// **Every ending that really wrote nothing still says so.**
    #[test]
    fn an_ending_before_any_write_still_says_nothing_was_written() {
        for why in [
            making::WhyTheReleaseWasNotMade::TheMachineProgramHasNoSource,
            making::WhyTheReleaseWasNotMade::ASourceIsNotAnHttpsAddress {
                said: "http://example.invalid/a".into(),
            },
            making::WhyTheReleaseWasNotMade::ItDidNotVerify {
                said: "NoAllowedKeySignedTheManifest".into(),
            },
            making::WhyTheReleaseWasNotMade::AnArtifactCouldNotBeRead {
                name: "smoozy-server".into(),
                said: "no such file".into(),
            },
            making::WhyTheReleaseWasNotMade::TheFolderIsNotEmptyOrCannotBeMade {
                said: "it already holds something".into(),
            },
        ] {
            assert!(
                how_the_release_ended(&why).contains("nothing was written"),
                "{why:?}"
            );
        }
    }

    /// **A refusal this build cannot name never tells the release runner the
    /// key is missing**, and it names the password manager as the master copy.
    #[test]
    fn a_refusal_this_build_cannot_name_says_both_and_asserts_neither() {
        let said = why_the_secret_was_not_obtained(
            &secret::WhyTheSecretWasNotObtained::TheEntryWasNotHandedOver {
                the_tools_own_ending: Some(51),
                said: "authorization denied".into(),
            },
        );
        assert!(said.contains("may not be there at all"));
        assert!(said.contains("may have been cancelled or refused"));
        assert!(said.contains("password manager"));
        assert!(said.contains("ended at 51"));
        assert!(said.contains("authorization denied"));
        assert!(
            !said.contains("is not in this Mac's keychain"),
            "a refusal this build cannot name was reported as a missing entry"
        );
    }

    /// **Every reason the everyday key was not obtained says something of its
    /// own, names the second entry, and says the release was not made** — and
    /// none of them is mistaken for the release root's.
    #[test]
    fn every_reason_the_everyday_key_was_not_obtained_says_something_of_its_own() {
        use secret::WhyTheSecretWasNotObtained::*;
        use secret::WhyTheUpdateKeyWasNotObtained::*;
        let said: Vec<String> = [
            ThisBuildCarriesNoTrustStatement,
            TheKeychain(ThereIsNoSuchEntry),
            TheKeychain(TheEntryWasNotHandedOver {
                the_tools_own_ending: Some(44),
                said: "The specified item could not be found in the keychain.".into(),
            }),
            TheKeychain(ItIsNotThirtyTwoBytesOfHexadecimal),
            TheKeychain(TheKeychainCouldNotBeAsked {
                said: "no such tool".into(),
            }),
            ItIsNoOnlineKeyThisBuildAllows,
        ]
        .iter()
        .map(why_the_update_key_was_not_obtained)
        .collect();
        for at in 0..said.len() {
            assert!(said[at].contains("not made"), "{}", said[at]);
            for beside in at + 1..said.len() {
                assert_ne!(said[at], said[beside]);
            }
        }
        for one in &said[1..5] {
            assert!(one.contains(secret::THE_UPDATE_KEYCHAIN_SERVICE), "{one}");
            assert!(!one.contains(secret::THE_KEYCHAIN_SERVICE), "{one}");
        }
        // **A refusal this build cannot name never says the key is missing.**
        assert!(said[2].contains("may not be there at all"));
        assert!(!said[2].contains("is not in this Mac's keychain"));
        assert!(said[1].contains("password manager"));
    }

    /// **The sentence for a build with no master key says nothing is wrong**
    /// and why no file is made.
    #[test]
    fn a_build_with_no_master_key_is_told_plainly_why_no_update_file_is_made() {
        assert!(NO_UPDATE_FILE_IS_WRITTEN.contains("no update master key"));
        assert!(NO_UPDATE_FILE_IS_WRITTEN.contains("could believe"));
    }
}

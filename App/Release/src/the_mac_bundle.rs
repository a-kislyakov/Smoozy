//! **Signing and notarizing the Mac bundle: the release step that makes the Mac
//! download one macOS will open** (`Desktop app and delivery` E2 and E6).
//!
//! ```text
//! smoozy-release sign-the-mac-bundle
//!     --bundle <path to Smoozy.app>     the bundle the shell built at the desk
//!     --into <path to a .zip>           the download to write; it must not exist
//! ```
//!
//! # What Apple requires, and where each requirement is kept
//!
//! Every requirement below is Apple's own, read 2026-10-24. The citations are in
//! `Audits/Build/Step 11/Sources/D2, signing and notarizing the Mac bundle — what landed.sources.md`.
//!
//! - **A Developer ID Application identity.** *Notarizing macOS software before
//!   distribution*: *"Use a “Developer ID” application … certificate for your
//!   code-signing signature. (Don't use a Mac Distribution, ad hoc, Apple
//!   Developer, or local development certificate.)"* Kept by
//!   [`the_developer_id_identity`], which takes only an identity whose name
//!   begins [`THE_IDENTITYS_KIND`].
//! - **Signed by its hash, not its name.** *Creating distribution-signed code for
//!   macOS*: each line of `security find-identity -p codesigning -v` *"includes
//!   a SHA-1 hash that uniquely identifies the identity"*. Kept by signing with
//!   the hash.
//! - **A secure timestamp and the hardened runtime.** The same page: *"add the
//!   `--timestamp` option"* and *"add the `-o runtime` option to enable the
//!   Hardened Runtime"*; *Resolving common notarization issues* names the
//!   notarization failures for each. Kept by [`sign_it`] and checked after
//!   signing by [`the_signature_is_what_notarization_needs`].
//! - **No `com.apple.security.get-task-allow` entitlement.** The overview page's
//!   list. **This step signs with no entitlements at all**: Apple's *Hardened
//!   Runtime* page says an entitlement is added only *"If your app relies on a
//!   capability that the Hardened Runtime restricts"*, and nothing read says
//!   Smoozy does. **Whether Smoozy runs under the hardened runtime with none is
//!   not observed** until a signed build is opened. The signed bundle's
//!   entitlements are read back and refused if that one is there.
//! - **Signed inside out, and never with `--deep`.** *Creating
//!   distribution-signed code*: *"Sign code from the inside out"* and *"Don't
//!   pass the `--deep` option to `codesign` when you sign code."* **The bundle
//!   the shell builds carries one piece of code, its main program** (read off
//!   the desk build, 2026-10-24), so signing the bundle signs everything in it.
//!   **A bundle carrying any other code is refused before it is touched**
//!   ([`the_other_code_in`]) rather than half-signed: this step would need
//!   extending first.
//! - **Uploaded as a ZIP made by `ditto`, stapled to the app, then zipped
//!   again.** *Customizing the notarization workflow*: *"you can't upload the
//!   `.app` bundle directly"*, the archive is made with `ditto -c -k
//!   --keepParent`, and *"While you can notarize a ZIP archive, you can't staple
//!   to it directly. Instead, run `stapler` against each item that you added to
//!   the archive. Then create a new ZIP file containing the stapled items for
//!   distribution."* Kept in that order.
//! - **The log read even on success.** The same page: *"Always check the log
//!   file, even if notarization succeeds"*. The log is fetched and handed back
//!   either way.
//! - **Checked before submitting** with `codesign -vvv --deep --strict`
//!   (*Resolving common notarization issues*: `--deep` *"to ensure the utility
//!   checks nested code"*, `--strict` *"to match that required by
//!   notarization"*), and **after stapling** with `stapler validate` and
//!   `spctl -vvv --assess --type exec`, the page's own check that the software
//!   *"will run with the system policies currently in effect"*.
//!
//! # Where the two credentials are, and that this step never holds either
//!
//! **Both are in the Mac's keychain and both are reached by name, as
//! `secret.rs` reaches the server root.** The identity's private key never
//! leaves the keychain: `codesign` is given the identity's hash and uses the key
//! itself. The notarization credential never leaves it either: `notarytool` is
//! given the profile name [`THE_NOTARIZATION_PROFILE`], which Apple's own
//! workflow page shows as the way *"to avoid including your password as
//! cleartext in a script"*. **No password, key or certificate is an argument,
//! a file or a line this step prints.**
//!
//! # The order, and the line it does not cross
//!
//! **Everything that can be missing is found missing before the bundle is
//! touched**: the download's place, the bundle's shape, the identity, then the
//! credential. **The download is written last and only when every check
//! passed**, so a stop part way leaves no file that could be published, and the
//! words say what the bundle on the disk now is.

use std::path::{Path, PathBuf};

use smoozy_server_protocol::transfers::integrity::Digest;

/// **The kind of identity this step signs with**, as the start of its name.
///
/// Apple's *Creating distribution-signed code for macOS* names it
/// *"Developer ID Application: <Team ID>"*, and its own sample listing shows
/// `"Developer ID Application: …"`. The rest of the name is the Creator's
/// team's, which nothing here needs to know.
pub const THE_IDENTITYS_KIND: &str = "Developer ID Application:";

/// **The name the notarization credential is stored under on this Mac.** It is
/// the profile name given to `xcrun notarytool store-credentials`, and the
/// Creator stores it under exactly this name
/// (`App/Release/About this folder.md`). Chosen 2026-10-24 by D2's builder.
pub const THE_NOTARIZATION_PROFILE: &str = "Smoozy notarization";

/// **How long the step waits for Apple's answer before it stops.** Apple's
/// workflow page: notarization *"completes for most software within 5 minutes,
/// and for 98 percent of software within 15 minutes"*. An hour is four times
/// the second figure; past it the step stops and says the service may still be
/// working.
pub const THE_LONGEST_WAIT_FOR_NOTARIZATION: &str = "1h";

/// **The first four bytes of a Mach-O file, either kind and either byte order**,
/// from Apple's own headers in the macOS SDK on this Mac, read 2026-10-24:
/// `mach-o/loader.h` (`MH_MAGIC`, `MH_CIGAM`, `MH_MAGIC_64`, `MH_CIGAM_64`) and
/// `mach-o/fat.h` (`FAT_MAGIC`, `FAT_CIGAM`, `FAT_MAGIC_64`, `FAT_CIGAM_64`).
const THE_MARKS_OF_MACH_O: [u32; 8] = [
    0xfeed_face,
    0xcefa_edfe,
    0xfeed_facf,
    0xcffa_edfe,
    0xcafe_babe,
    0xbeba_feca,
    0xcafe_babf,
    0xbfba_feca,
];

/// **Folder endings that hold code of their own inside a bundle.** This step's
/// own list, not a fact it rests on: a folder with any of these endings is
/// refused rather than judged, which errs towards stopping.
const FOLDERS_THAT_HOLD_CODE: [&str; 9] = [
    "app",
    "framework",
    "appex",
    "xpc",
    "bundle",
    "plugin",
    "kext",
    "systemextension",
    "dylib",
];

// --------------------------------------------------------------- the tools

/// **What one of the Mac's own tools answered**, in its own words.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatAToolAnswered {
    /// Its ending, or nothing where it was ended by a signal.
    pub ending: Option<i32>,
    /// What it wrote out.
    pub out: String,
    /// What it wrote on its complaints stream.
    pub said: String,
}

impl WhatAToolAnswered {
    fn both(&self) -> String {
        match (self.out.trim(), self.said.trim()) {
            ("", "") => String::new(),
            (out, "") => out.to_owned(),
            ("", said) => said.to_owned(),
            (out, said) => format!("{out}\n{said}"),
        }
    }

    fn in_its_words(&self) -> String {
        let ending = match self.ending {
            Some(ending) => format!("It ended at {ending}."),
            None => "It was ended by a signal.".to_owned(),
        };
        let both = self.both();
        if both.is_empty() {
            format!("{ending} It said nothing.")
        } else {
            format!("{ending} It said:\n{}", indented(&both))
        }
    }
}

/// **The Mac's own tools, as one seam.** Running a tool is this; deciding what
/// its answer means is everything else in this file, and it is tested without
/// a keychain, a certificate or Apple.
pub trait TheMacsTools {
    fn run(&mut self, program: &str, arguments: &[&str]) -> Result<WhatAToolAnswered, String>;
}

/// **The tools on this Mac, run as they are.** Nothing is typed into them:
/// their input is closed, so a tool that wanted to ask on the command line
/// cannot sit waiting.
pub struct TheToolsOnThisMac;

impl TheMacsTools for TheToolsOnThisMac {
    fn run(&mut self, program: &str, arguments: &[&str]) -> Result<WhatAToolAnswered, String> {
        let ran = std::process::Command::new(program)
            .args(arguments)
            .stdin(std::process::Stdio::null())
            .output()
            .map_err(|failed| failed.to_string())?;
        Ok(WhatAToolAnswered {
            ending: ran.status.code(),
            out: String::from_utf8_lossy(&ran.stdout).into_owned(),
            said: String::from_utf8_lossy(&ran.stderr).into_owned(),
        })
    }
}

// ------------------------------------------------------------- the endings

/// **What the bundle on the disk is, at the moment the step stopped.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheBundleNow {
    /// Nothing was done to it.
    Untouched,
    /// It carries a Developer ID signature and Apple has not accepted it.
    SignedAndNotNotarized,
    /// Apple accepted it, and the ticket is not on it or did not check out.
    NotarizedAndNotFinished,
    /// It is signed, notarized and stapled; only the download was not written.
    Finished,
}

/// **Why the step stopped.** Every one of these leaves no download behind.
#[derive(Debug)]
pub enum WhyTheBundleWasNotSigned {
    // ---- before the bundle is touched
    /// `--into` already exists, has no folder to go in, or is not a `.zip`.
    TheDownloadsPlaceIsNotUsable {
        said: String,
    },
    /// The path is not a bundle the step can read.
    ItIsNotABundle {
        said: String,
    },
    /// The bundle carries code beside its main program, which this step does
    /// not sign.
    ItCarriesCodeBesideItsMainProgram {
        found: Vec<String>,
    },
    /// **No Developer ID Application identity is in this Mac's keychain.**
    /// The Creator's to set up.
    ThereIsNoDeveloperIdIdentity,
    /// More than one, and the step does not choose between them.
    ThereIsMoreThanOneDeveloperIdIdentity {
        named: Vec<String>,
    },
    /// The keychain's list of identities could not be read.
    TheIdentitiesCouldNotBeListed {
        said: String,
    },
    /// **The stored notarization credential did not answer.** Not stored under
    /// that name, refused by Apple, or Apple not reached: this step cannot
    /// tell those apart and does not guess.
    TheNotarizationCredentialDidNotAnswer {
        said: String,
    },
    /// One of the Mac's tools could not be run at all.
    AToolCouldNotBeRun {
        tool: String,
        said: String,
        the_bundle: TheBundleNow,
    },

    // ---- after the bundle is touched
    TheSigningFailed {
        said: String,
    },
    TheSignatureIsNotWhatNotarizationNeeds {
        what: String,
        said: String,
    },
    TheSubmissionCouldNotBeMade {
        said: String,
    },
    AppleDidNotAcceptIt {
        status: Option<String>,
        submission: Option<String>,
        said: String,
        the_log: Option<String>,
    },
    TheTicketCouldNotBeStapled {
        said: String,
    },
    TheStapledTicketDidNotCheckOut {
        said: String,
    },
    TheSystemsOwnCheckRefusedIt {
        said: String,
    },
    TheDownloadCouldNotBeWritten {
        said: String,
    },
}

impl WhyTheBundleWasNotSigned {
    pub fn the_bundle_now(&self) -> TheBundleNow {
        use WhyTheBundleWasNotSigned::*;
        match self {
            TheDownloadsPlaceIsNotUsable { .. }
            | ItIsNotABundle { .. }
            | ItCarriesCodeBesideItsMainProgram { .. }
            | ThereIsNoDeveloperIdIdentity
            | ThereIsMoreThanOneDeveloperIdIdentity { .. }
            | TheIdentitiesCouldNotBeListed { .. }
            | TheNotarizationCredentialDidNotAnswer { .. } => TheBundleNow::Untouched,
            AToolCouldNotBeRun { the_bundle, .. } => *the_bundle,
            TheSigningFailed { .. }
            | TheSignatureIsNotWhatNotarizationNeeds { .. }
            | TheSubmissionCouldNotBeMade { .. }
            | AppleDidNotAcceptIt { .. } => TheBundleNow::SignedAndNotNotarized,
            TheTicketCouldNotBeStapled { .. }
            | TheStapledTicketDidNotCheckOut { .. }
            | TheSystemsOwnCheckRefusedIt { .. } => TheBundleNow::NotarizedAndNotFinished,
            TheDownloadCouldNotBeWritten { .. } => TheBundleNow::Finished,
        }
    }
}

/// **What a finished step made.**
#[derive(Debug)]
pub struct WhatWasMade {
    /// The identity's name, as the keychain lists it.
    pub signed_by: String,
    /// Apple's identifier for the accepted submission.
    pub the_submission: String,
    /// Apple's log for it, whole, to be read for warnings.
    pub the_log: String,
    /// The download, written last.
    pub the_download: PathBuf,
    pub its_length: u64,
    pub its_digest: String,
}

// ---------------------------------------------------------------- the step

/// **Sign the bundle with the Developer ID, notarize it, staple it, check it,
/// and write the download.** `tell` is handed one line per step done.
pub fn sign_and_notarize(
    the_bundle: &Path,
    into: &Path,
    tools: &mut dyn TheMacsTools,
    tell: &mut dyn FnMut(String),
) -> Result<WhatWasMade, WhyTheBundleWasNotSigned> {
    use WhyTheBundleWasNotSigned::*;

    // **1 to 4: everything that can be missing, with the bundle untouched.**
    let (into_text, the_submission_archive) = the_place_for_the_download(into)?;
    let Some(bundle_text) = the_bundle.to_str() else {
        return Err(ItIsNotABundle {
            said: "its path is not plain text".into(),
        });
    };
    let identifier = read_the_bundle(the_bundle, tools)?;
    tell(format!(
        "The bundle is one the step can sign: identifier {identifier}, and its main program is its only code."
    ));

    let (hash, signed_by) = the_developer_id_identity(tools)?;
    tell(format!(
        "The signing identity is in the keychain: {signed_by}."
    ));

    the_notarization_credential_answers(tools)?;
    tell(format!(
        "The notarization credential stored as {THE_NOTARIZATION_PROFILE:?} answered."
    ));

    // **5 and 6: signed, then checked against what notarization needs.**
    sign_it(tools, &hash, bundle_text)?;
    the_signature_is_what_notarization_needs(tools, bundle_text)?;
    tell(
        "Signed with a secure timestamp and the hardened runtime, and the signature checks out."
            .into(),
    );

    // **7: submitted, and Apple's answer read with its log.**
    let (the_submission, the_log) = notarize_it(tools, bundle_text, &the_submission_archive, tell)?;
    tell(format!("Apple accepted submission {the_submission}."));

    // **8 to 10: stapled, the ticket checked, the system's own check.**
    let after = TheBundleNow::NotarizedAndNotFinished;
    let stapled = ask(tools, "xcrun", &["stapler", "staple", bundle_text], after)?;
    if stapled.ending != Some(0) {
        return Err(TheTicketCouldNotBeStapled {
            said: stapled.in_its_words(),
        });
    }
    let validated = ask(tools, "xcrun", &["stapler", "validate", bundle_text], after)?;
    if validated.ending != Some(0) {
        return Err(TheStapledTicketDidNotCheckOut {
            said: validated.in_its_words(),
        });
    }
    let assessed = ask(
        tools,
        "spctl",
        &["-vvv", "--assess", "--type", "exec", bundle_text],
        after,
    )?;
    if assessed.ending != Some(0) {
        return Err(TheSystemsOwnCheckRefusedIt {
            said: assessed.in_its_words(),
        });
    }
    tell(format!(
        "The ticket is stapled and checks out, and the system's own check accepts the bundle:\n{}",
        indented(&assessed.both())
    ));

    // **11: the download, last.**
    let zipped = ask(
        tools,
        "ditto",
        &["-c", "-k", "--keepParent", bundle_text, &into_text],
        TheBundleNow::Finished,
    )?;
    if zipped.ending != Some(0) {
        return Err(TheDownloadCouldNotBeWritten {
            said: zipped.in_its_words(),
        });
    }
    let bytes = std::fs::read(into).map_err(|failed| TheDownloadCouldNotBeWritten {
        said: format!("it could not be read back: {failed}"),
    })?;
    Ok(WhatWasMade {
        signed_by,
        the_submission,
        the_log,
        the_download: into.to_path_buf(),
        its_length: bytes.len() as u64,
        its_digest: Digest::of(&bytes).as_text(),
    })
}

/// Run one tool, and turn *it could not be run* into this step's own ending.
fn ask(
    tools: &mut dyn TheMacsTools,
    program: &str,
    arguments: &[&str],
    the_bundle: TheBundleNow,
) -> Result<WhatAToolAnswered, WhyTheBundleWasNotSigned> {
    tools
        .run(program, arguments)
        .map_err(|said| WhyTheBundleWasNotSigned::AToolCouldNotBeRun {
            tool: format!("{program} {}", arguments.first().copied().unwrap_or(""))
                .trim()
                .to_owned(),
            said,
            the_bundle,
        })
}

/// **The download's place: a `.zip` that does not exist yet, in a folder that
/// does.** The archive sent to Apple is made beside it under its own name and
/// removed after the submission.
fn the_place_for_the_download(into: &Path) -> Result<(String, PathBuf), WhyTheBundleWasNotSigned> {
    use WhyTheBundleWasNotSigned::TheDownloadsPlaceIsNotUsable as NotUsable;
    let Some(text) = into.to_str() else {
        return Err(NotUsable {
            said: "its path is not plain text".into(),
        });
    };
    if into.extension().and_then(|it| it.to_str()) != Some("zip") {
        return Err(NotUsable {
            said: format!("{text} does not end in .zip, and the download is a ZIP archive"),
        });
    }
    if std::fs::symlink_metadata(into).is_ok() {
        return Err(NotUsable {
            said: format!("{text} already exists, and the step never writes over a file"),
        });
    }
    let folder = match into.parent() {
        Some(folder) if folder.as_os_str().is_empty() => Path::new("."),
        Some(folder) => folder,
        None => {
            return Err(NotUsable {
                said: format!("{text} has no folder to go in"),
            })
        }
    };
    if !folder.is_dir() {
        return Err(NotUsable {
            said: format!("{} is not a folder", folder.display()),
        });
    }
    let stem = into
        .file_stem()
        .and_then(|it| it.to_str())
        .unwrap_or("Smoozy");
    let submission = folder.join(format!("{stem} — sent for notarization.zip"));
    if std::fs::symlink_metadata(&submission).is_ok() {
        return Err(NotUsable {
            said: format!(
                "{} already exists, and the step makes the archive it sends to Apple there",
                submission.display()
            ),
        });
    }
    Ok((text.to_owned(), submission))
}

/// **Read the bundle and refuse anything this step does not sign whole.**
/// Returns its identifier. Nothing is written.
fn read_the_bundle(
    the_bundle: &Path,
    tools: &mut dyn TheMacsTools,
) -> Result<String, WhyTheBundleWasNotSigned> {
    use WhyTheBundleWasNotSigned::*;
    let not_a_bundle = |said: String| ItIsNotABundle { said };
    let shown = the_bundle.display().to_string();

    if !the_bundle.is_dir() {
        return Err(not_a_bundle(format!("{shown} is not a folder")));
    }
    if the_bundle.extension().and_then(|it| it.to_str()) != Some("app") {
        return Err(not_a_bundle(format!("{shown} does not end in .app")));
    }
    let info = the_bundle.join("Contents").join("Info.plist");
    if !info.is_file() {
        return Err(not_a_bundle(format!("{shown} has no Contents/Info.plist")));
    }
    let Some(info_text) = info.to_str() else {
        return Err(not_a_bundle("its path is not plain text".into()));
    };

    let mut a_value = |key: &str| -> Result<String, WhyTheBundleWasNotSigned> {
        let read = ask(
            tools,
            "plutil",
            &["-extract", key, "raw", "-o", "-", info_text],
            TheBundleNow::Untouched,
        )?;
        let value = read.out.trim();
        if read.ending != Some(0) || value.is_empty() || value.contains('\n') {
            return Err(ItIsNotABundle {
                said: format!("its Info.plist gives no {key}. {}", read.in_its_words()),
            });
        }
        Ok(value.to_owned())
    };
    let program_name = a_value("CFBundleExecutable")?;
    let identifier = a_value("CFBundleIdentifier")?;
    if program_name.contains('/') {
        return Err(not_a_bundle(format!(
            "its Info.plist names {program_name:?} as its main program, which is not a file name"
        )));
    }

    let the_program = the_bundle
        .join("Contents")
        .join("MacOS")
        .join(&program_name);
    match is_mach_o(&the_program) {
        Ok(true) => {}
        Ok(false) => {
            return Err(not_a_bundle(format!(
                "Contents/MacOS/{program_name}, which its Info.plist names as its main program, is not a program"
            )))
        }
        Err(failed) => {
            return Err(not_a_bundle(format!(
                "Contents/MacOS/{program_name}, which its Info.plist names as its main program, could not be read: {failed}"
            )))
        }
    }

    let found = the_other_code_in(the_bundle, &the_program)
        .map_err(|failed| not_a_bundle(format!("{shown} could not be read through: {failed}")))?;
    if !found.is_empty() {
        return Err(ItCarriesCodeBesideItsMainProgram { found });
    }
    Ok(identifier)
}

fn is_mach_o(path: &Path) -> std::io::Result<bool> {
    use std::io::Read;
    let meta = std::fs::symlink_metadata(path)?;
    if !meta.is_file() {
        return Ok(false);
    }
    let mut first = [0u8; 4];
    let mut file = std::fs::File::open(path)?;
    if file.read(&mut first)? < 4 {
        return Ok(false);
    }
    Ok(THE_MARKS_OF_MACH_O.contains(&u32::from_be_bytes(first)))
}

/// **Every piece of code in the bundle other than its main program**, named
/// from inside the bundle. A link is listed too, because this step does not
/// follow one to see what it points at.
fn the_other_code_in(the_bundle: &Path, the_program: &Path) -> std::io::Result<Vec<String>> {
    let mut found = Vec::new();
    let mut waiting = vec![the_bundle.to_path_buf()];
    let the_programs_folder = the_bundle.join("Contents").join("MacOS");
    while let Some(folder) = waiting.pop() {
        let mut entries: Vec<_> = std::fs::read_dir(&folder)?.collect::<Result<_, _>>()?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            let named = path
                .strip_prefix(the_bundle)
                .unwrap_or(&path)
                .display()
                .to_string();
            let kind = entry.file_type()?;
            if kind.is_symlink() {
                found.push(format!("{named} (a link)"));
            } else if kind.is_dir() {
                let ending = path.extension().and_then(|it| it.to_str()).unwrap_or("");
                if FOLDERS_THAT_HOLD_CODE.contains(&ending) {
                    found.push(named);
                } else {
                    waiting.push(path);
                }
            } else if path == the_program {
                continue;
            } else if folder == the_programs_folder || is_mach_o(&path)? {
                found.push(named);
            }
        }
    }
    found.sort();
    Ok(found)
}

/// **The one Developer ID Application identity in this Mac's keychain**, as
/// its hash and its name.
///
/// **This is a listing and reads no secret**: `security find-identity` names
/// identities, and the private key stays where it is.
fn the_developer_id_identity(
    tools: &mut dyn TheMacsTools,
) -> Result<(String, String), WhyTheBundleWasNotSigned> {
    use WhyTheBundleWasNotSigned::*;
    let listed = ask(
        tools,
        "security",
        &["find-identity", "-v", "-p", "codesigning"],
        TheBundleNow::Untouched,
    )?;
    if listed.ending != Some(0) {
        return Err(TheIdentitiesCouldNotBeListed {
            said: listed.in_its_words(),
        });
    }
    let every = the_identities_in(&listed.out).map_err(|said| TheIdentitiesCouldNotBeListed {
        said: format!("{said}. {}", listed.in_its_words()),
    })?;
    let mut developer_ids: Vec<(String, String)> = every
        .into_iter()
        .filter(|(_, name)| name.starts_with(THE_IDENTITYS_KIND))
        .collect();
    match developer_ids.len() {
        0 => Err(ThereIsNoDeveloperIdIdentity),
        1 => Ok(developer_ids.remove(0)),
        _ => Err(ThereIsMoreThanOneDeveloperIdIdentity {
            named: developer_ids.into_iter().map(|(_, name)| name).collect(),
        }),
    }
}

/// **Read `security find-identity -v`'s listing: every identity, as its hash
/// and its name.**
///
/// **The shape is Apple's own sample** in *Creating distribution-signed code
/// for macOS* — `  2) ADC0…B59B "Developer ID Application: …"` and a last line
/// `     2 valid identities found` — **and the empty listing was observed on this
/// Mac, 2026-10-24**: `     0 valid identities found` alone. **A listing whose
/// count disagrees with the lines read is refused** rather than half-read, so
/// a shape that differs from the sample stops the step instead of hiding an
/// identity.
fn the_identities_in(listed: &str) -> Result<Vec<(String, String)>, String> {
    let mut identities = Vec::new();
    let mut counted = None;
    for line in listed
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        if let Some(count) = a_count_line(line) {
            counted = Some(count);
        } else if let Some(identity) = an_identity_line(line) {
            identities.push(identity);
        }
    }
    match counted {
        None => Err("the listing has no line saying how many identities it found".into()),
        Some(count) if count != identities.len() => Err(format!(
            "the listing says {count} and {} could be read",
            identities.len()
        )),
        Some(_) => Ok(identities),
    }
}

fn a_count_line(line: &str) -> Option<usize> {
    let (number, rest) = line.split_once(' ')?;
    let rest = rest.trim();
    if (rest == "valid identities found" || rest == "valid identity found")
        || (rest == "identities found" || rest == "identity found")
    {
        number.parse().ok()
    } else {
        None
    }
}

fn an_identity_line(line: &str) -> Option<(String, String)> {
    let (number, rest) = line.split_once(") ")?;
    number.parse::<usize>().ok()?;
    let (hash, name) = rest.split_once(' ')?;
    if hash.len() != 40 || !hash.chars().all(|it| it.is_ascii_hexdigit()) {
        return None;
    }
    let name = name.trim().strip_prefix('"')?.strip_suffix('"')?;
    Some((hash.to_owned(), name.to_owned()))
}

/// **Ask Apple's notary service, with the stored credential, for this team's
/// past submissions** — the one way to learn the credential works without
/// sending anything. The answer itself is not kept.
fn the_notarization_credential_answers(
    tools: &mut dyn TheMacsTools,
) -> Result<(), WhyTheBundleWasNotSigned> {
    let answered = ask(
        tools,
        "xcrun",
        &[
            "notarytool",
            "history",
            "--keychain-profile",
            THE_NOTARIZATION_PROFILE,
            "--no-progress",
        ],
        TheBundleNow::Untouched,
    )?;
    if answered.ending == Some(0) {
        Ok(())
    } else {
        Err(
            WhyTheBundleWasNotSigned::TheNotarizationCredentialDidNotAnswer {
                said: answered.in_its_words(),
            },
        )
    }
}

/// **Sign the bundle, replacing the shell's ad-hoc signature**, with a secure
/// timestamp and the hardened runtime, by the identity's hash.
fn sign_it(
    tools: &mut dyn TheMacsTools,
    hash: &str,
    bundle: &str,
) -> Result<(), WhyTheBundleWasNotSigned> {
    let signed = ask(
        tools,
        "codesign",
        &[
            "--force",
            "--timestamp",
            "--options",
            "runtime",
            "--sign",
            hash,
            bundle,
        ],
        TheBundleNow::SignedAndNotNotarized,
    )?;
    if signed.ending == Some(0) {
        Ok(())
    } else {
        Err(WhyTheBundleWasNotSigned::TheSigningFailed {
            said: signed.in_its_words(),
        })
    }
}

/// **Read the new signature back against Apple's list before anything is sent.**
fn the_signature_is_what_notarization_needs(
    tools: &mut dyn TheMacsTools,
    bundle: &str,
) -> Result<(), WhyTheBundleWasNotSigned> {
    let at = TheBundleNow::SignedAndNotNotarized;
    let not = |what: &str, answered: &WhatAToolAnswered| {
        WhyTheBundleWasNotSigned::TheSignatureIsNotWhatNotarizationNeeds {
            what: what.to_owned(),
            said: answered.in_its_words(),
        }
    };

    let verified = ask(
        tools,
        "codesign",
        &["--verify", "--deep", "--strict", "-vvv", bundle],
        at,
    )?;
    if verified.ending != Some(0) {
        return Err(not("it does not pass codesign's strict check", &verified));
    }

    let shown = ask(tools, "codesign", &["--display", "-vv", bundle], at)?;
    let lines = shown.both();
    if shown.ending != Some(0) {
        return Err(not("its signature could not be read back", &shown));
    }
    let first_authority = lines
        .lines()
        .map(str::trim)
        .find_map(|line| line.strip_prefix("Authority="));
    if !first_authority.is_some_and(|it| it.starts_with(THE_IDENTITYS_KIND)) {
        return Err(not(
            "it is not signed by a Developer ID Application identity",
            &shown,
        ));
    }
    // **Apple's words**: a secure timestamp shows as `Timestamp`, and
    // `Signed Time` *"indicates the binary doesn't have a secure timestamp"*.
    let lines_trimmed: Vec<&str> = lines.lines().map(str::trim).collect();
    if !lines_trimmed
        .iter()
        .any(|line| line.starts_with("Timestamp="))
        || lines_trimmed
            .iter()
            .any(|line| line.starts_with("Signed Time="))
    {
        return Err(not("it carries no secure timestamp", &shown));
    }
    // **The flags are read off the `CodeDirectory` line** — observed on this
    // Mac as `flags=0x20002(adhoc,linker-signed)` on the shell's own ad-hoc
    // build, 2026-10-24 — and `runtime` is the flag codesign's own manual
    // names for the hardened runtime.
    let runtime = lines_trimmed
        .iter()
        .filter(|line| line.starts_with("CodeDirectory "))
        .filter_map(|line| line.split_once("flags=").map(|(_, rest)| rest))
        .filter_map(|rest| {
            rest.split_once('(')
                .and_then(|(_, names)| names.split_once(')'))
        })
        .any(|(names, _)| names.split(',').any(|name| name == "runtime"));
    if !runtime {
        return Err(not("it does not have the hardened runtime", &shown));
    }

    let entitlements = ask(
        tools,
        "codesign",
        &["--display", "--entitlements", "-", "--xml", bundle],
        at,
    )?;
    if entitlements.ending != Some(0) {
        return Err(not(
            "its entitlements could not be read back",
            &entitlements,
        ));
    }
    if entitlements
        .both()
        .contains("com.apple.security.get-task-allow")
    {
        return Err(not(
            "it carries the com.apple.security.get-task-allow entitlement",
            &entitlements,
        ));
    }
    Ok(())
}

/// **Zip the signed bundle, send it, wait for Apple's answer, and fetch the
/// log either way.** The archive sent is removed afterwards; it lacks the
/// ticket and is not the download.
fn notarize_it(
    tools: &mut dyn TheMacsTools,
    bundle: &str,
    the_archive: &Path,
    tell: &mut dyn FnMut(String),
) -> Result<(String, String), WhyTheBundleWasNotSigned> {
    use WhyTheBundleWasNotSigned::*;
    let at = TheBundleNow::SignedAndNotNotarized;
    let Some(archive) = the_archive.to_str() else {
        return Err(TheSubmissionCouldNotBeMade {
            said: "the archive's path is not plain text".into(),
        });
    };

    let zipped = ask(
        tools,
        "ditto",
        &["-c", "-k", "--keepParent", bundle, archive],
        at,
    )?;
    if zipped.ending != Some(0) {
        return Err(TheSubmissionCouldNotBeMade {
            said: format!(
                "the archive for Apple could not be made. {}",
                zipped.in_its_words()
            ),
        });
    }
    tell(format!(
        "Sent to Apple for notarization; waiting up to {THE_LONGEST_WAIT_FOR_NOTARIZATION}."
    ));
    let submitted = ask(
        tools,
        "xcrun",
        &[
            "notarytool",
            "submit",
            archive,
            "--keychain-profile",
            THE_NOTARIZATION_PROFILE,
            "--wait",
            "--timeout",
            THE_LONGEST_WAIT_FOR_NOTARIZATION,
            "--no-progress",
        ],
        at,
    );
    if let Err(failed) = std::fs::remove_file(the_archive) {
        tell(format!(
            "The archive sent to Apple could not be removed and is still at {archive}: {failed}. It is not the download and must not be published."
        ));
    }
    let submitted = submitted?;

    let (submission, status) = the_submission_and_its_status(&submitted.both());
    let the_log = match &submission {
        Some(id) => {
            let logged = ask(
                tools,
                "xcrun",
                &[
                    "notarytool",
                    "log",
                    id,
                    "--keychain-profile",
                    THE_NOTARIZATION_PROFILE,
                ],
                at,
            )?;
            if logged.ending == Some(0) {
                Some(logged.out)
            } else {
                Some(format!(
                    "The log could not be fetched. {}",
                    logged.in_its_words()
                ))
            }
        }
        None => None,
    };

    match (submitted.ending, status.as_deref(), submission, the_log) {
        (Some(0), Some("Accepted"), Some(submission), Some(the_log)) => Ok((submission, the_log)),
        (_, status, submission, the_log) => {
            if submission.is_none() && submitted.ending != Some(0) && status.is_none() {
                Err(TheSubmissionCouldNotBeMade {
                    said: submitted.in_its_words(),
                })
            } else {
                Err(AppleDidNotAcceptIt {
                    status: status.map(str::to_owned),
                    submission,
                    said: submitted.in_its_words(),
                    the_log,
                })
            }
        }
    }
}

/// **The submission's identifier and the last status `notarytool` printed.**
///
/// The shape is Apple's own sample in *Customizing the notarization workflow*:
/// `id: 2efe2717-…` and, after `--wait`, `status: Accepted`. The man page names
/// the endings a wait returns on: *"Accepted", "Invalid", "Rejected"*.
fn the_submission_and_its_status(printed: &str) -> (Option<String>, Option<String>) {
    let mut submission = None;
    let mut status = None;
    for line in printed.lines().map(str::trim) {
        if let Some(id) = line.strip_prefix("id:") {
            if submission.is_none() && !id.trim().is_empty() {
                submission = Some(id.trim().to_owned());
            }
        } else if let Some(said) = line.strip_prefix("status:") {
            status = Some(said.trim().to_owned());
        }
    }
    (submission, status)
}

fn indented(text: &str) -> String {
    text.lines()
        .map(|line| format!("    {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

// --------------------------------------------------------------- the words

/// **The step's ending, in words a release runner can act on and pass to the
/// Creator.** Every ending says what happened, what the bundle on the disk now
/// is, and what to do.
pub fn in_words(why: &WhyTheBundleWasNotSigned) -> String {
    use WhyTheBundleWasNotSigned::*;
    const CREATORS: &str = "This is the Creator's to set up, and the steps are in App/Release/About this folder.md. Stop and tell the Creator.";
    let what = match why {
        TheDownloadsPlaceIsNotUsable { said } => {
            format!("The download cannot be written where --into says: {said}.")
        }
        ItIsNotABundle { said } => format!("This is not a bundle the step can sign: {said}."),
        ItCarriesCodeBesideItsMainProgram { found } => format!(
            "The bundle carries code beside its main program, and this step signs only a bundle whose one piece of code is its main program. Apple's order is inside out, one piece at a time, so the step has to be extended before it can sign this one. Found:\n{}",
            indented(&found.join("\n"))
        ),
        ThereIsNoDeveloperIdIdentity => format!(
            "There is no Developer ID Application certificate in this Mac's keychain, so nothing can be signed. The notarization credential was not checked. {CREATORS}"
        ),
        ThereIsMoreThanOneDeveloperIdIdentity { named } => format!(
            "There is more than one Developer ID Application certificate in this Mac's keychain, and the step does not choose between them. The Creator removes the one that should not be used, in Keychain Access. Found:\n{}",
            indented(&named.join("\n"))
        ),
        TheIdentitiesCouldNotBeListed { said } => format!(
            "The keychain's list of signing identities could not be read, so whether the Developer ID certificate is there is not known. {said}"
        ),
        TheNotarizationCredentialDidNotAnswer { said } => format!(
            "The notarization credential stored as {THE_NOTARIZATION_PROFILE:?} did not answer. It may not be stored under that name; it may be stored and refused by Apple, as an app-specific password is once the Apple Account password changes; or Apple may not have been reached. This step cannot tell those apart and does not guess. Check the connection first; otherwise this is the Creator's to set up, and the steps are in App/Release/About this folder.md.\n  notarytool: {said}"
        ),
        AToolCouldNotBeRun { tool, said, .. } => format!(
            "The Mac's own {tool} could not be run: {said}. Signing and notarizing need Xcode on this Mac."
        ),
        TheSigningFailed { said } => format!("Signing failed.\n  codesign: {said}"),
        TheSignatureIsNotWhatNotarizationNeeds { what, said } => format!(
            "The new signature is not what notarization needs: {what}. Nothing was sent to Apple.\n  codesign: {said}"
        ),
        TheSubmissionCouldNotBeMade { said } => {
            format!("The bundle could not be sent to Apple. {said}")
        }
        AppleDidNotAcceptIt {
            status,
            submission,
            said,
            the_log,
        } => {
            let status = match status.as_deref() {
                Some("In Progress") => format!(
                    "Apple had not answered after {THE_LONGEST_WAIT_FOR_NOTARIZATION}, and the service may still be working on it"
                ),
                Some(status) => format!("Apple's answer was {status:?}"),
                None => "notarytool did not say how the submission ended".to_owned(),
            };
            let submission = submission
                .as_deref()
                .map(|id| format!(" Submission {id}."))
                .unwrap_or_default();
            let log = the_log
                .as_deref()
                .map(|log| format!("\n  Apple's log:\n{}", indented(log)))
                .unwrap_or_default();
            format!("Notarization did not succeed: {status}.{submission}\n  notarytool: {said}{log}")
        }
        TheTicketCouldNotBeStapled { said } => {
            format!("Apple accepted the bundle and the ticket could not be stapled to it.\n  stapler: {said}")
        }
        TheStapledTicketDidNotCheckOut { said } => {
            format!("The ticket was stapled and does not check out.\n  stapler: {said}")
        }
        TheSystemsOwnCheckRefusedIt { said } => format!(
            "The bundle is stapled and the system's own check does not accept it.\n  spctl: {said}"
        ),
        TheDownloadCouldNotBeWritten { said } => {
            format!("The download could not be written: {said}.")
        }
    };
    let now = match why.the_bundle_now() {
        TheBundleNow::Untouched => {
            "The bundle was not touched, and no download was written.".to_owned()
        }
        TheBundleNow::SignedAndNotNotarized => "The bundle on the disk is now signed and not notarized. It must not be published, and no download was written. Running this step again on it signs it again.".to_owned(),
        TheBundleNow::NotarizedAndNotFinished => "Apple has accepted the bundle, and it is not finished on the disk. It must not be published, and no download was written. Running this step again on it signs and sends it again.".to_owned(),
        TheBundleNow::Finished => "The bundle on the disk is signed, notarized and stapled, and the download was not written. A bundle zipped by hand must not be published in its place: run this step again on it, which signs and sends it again.".to_owned(),
    };
    format!("{what}\n{now}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use WhyTheBundleWasNotSigned::*;

    // ------------------------------------------------------------ helpers

    fn a_folder(named: &str) -> PathBuf {
        let at = std::env::temp_dir().join(format!(
            "smoozy-release-mac-{named}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&at);
        std::fs::create_dir_all(&at).unwrap();
        at
    }

    const A_PROGRAM: &[u8] = &[0xcf, 0xfa, 0xed, 0xfe, 0x0c, 0x00, 0x00, 0x01];

    /// A bundle shaped as the shell builds Smoozy's.
    fn a_bundle(at: &Path) -> PathBuf {
        let bundle = at.join("Smoozy.app");
        let contents = bundle.join("Contents");
        std::fs::create_dir_all(contents.join("MacOS")).unwrap();
        std::fs::create_dir_all(contents.join("Resources")).unwrap();
        std::fs::write(contents.join("Info.plist"), b"<plist/>").unwrap();
        std::fs::write(contents.join("MacOS").join("smoozy-desktop"), A_PROGRAM).unwrap();
        std::fs::write(contents.join("Resources").join("icon.icns"), b"icns").unwrap();
        std::fs::write(
            contents.join("Resources").join("PrivacyInfo.xcprivacy"),
            b"<plist/>",
        )
        .unwrap();
        bundle
    }

    fn ok(out: &str) -> WhatAToolAnswered {
        WhatAToolAnswered {
            ending: Some(0),
            out: out.to_owned(),
            said: String::new(),
        }
    }

    fn ended(ending: i32, said: &str) -> WhatAToolAnswered {
        WhatAToolAnswered {
            ending: Some(ending),
            out: String::new(),
            said: said.to_owned(),
        }
    }

    const ONE_IDENTITY: &str = "  1) ADC03B244F4C1018384DCAFFC920F26136F6B59B \"Developer ID Application: A Creator (ABCDE12345)\"\n     1 valid identities found\n";

    const A_GOOD_SIGNATURE: &str = "Executable=/somewhere/Smoozy.app/Contents/MacOS/smoozy-desktop\nIdentifier=com.smoozy.desktop\nFormat=app bundle with Mach-O thin (arm64)\nCodeDirectory v=20500 size=1 flags=0x10000(runtime) hashes=1+0 location=embedded\nSignature size=9000\nAuthority=Developer ID Application: A Creator (ABCDE12345)\nAuthority=Developer ID Certification Authority\nAuthority=Apple Root CA\nTimestamp=24 Oct 2026 at 12:00:00\nInfo.plist entries=12\nTeamIdentifier=ABCDE12345\n";

    const ACCEPTED: &str = "Conducting pre-submission checks for Smoozy.zip and initiating connection to the Apple notary service...\nSubmission ID received\n  id: 2efe2717-52ef-43a5-96dc-0797e4ca1041\nSuccessfully uploaded file\n  id: 2efe2717-52ef-43a5-96dc-0797e4ca1041\n  path: /somewhere/Smoozy.zip\nWaiting for processing to complete.\n\nProcessing complete\n  id: 2efe2717-52ef-43a5-96dc-0797e4ca1041\n  status: Accepted\n";

    /// **A Mac whose tools answer from a script**, recording every call.
    struct AScriptedMac {
        answers: Vec<(String, WhatAToolAnswered)>,
        unrunnable: Vec<String>,
        asked: Vec<String>,
    }

    impl AScriptedMac {
        /// Every tool answers as it would on a Mac with both credentials and an
        /// accepting Apple.
        fn where_everything_is_set_up() -> Self {
            let mut signature = ok("");
            signature.said = A_GOOD_SIGNATURE.into();
            AScriptedMac {
                answers: vec![
                    (
                        "plutil -extract CFBundleExecutable".into(),
                        ok("smoozy-desktop\n"),
                    ),
                    (
                        "plutil -extract CFBundleIdentifier".into(),
                        ok("com.smoozy.desktop\n"),
                    ),
                    ("security find-identity".into(), ok(ONE_IDENTITY)),
                    ("notarytool history".into(), ok("No submission history.\n")),
                    ("codesign --display -vv".into(), signature),
                    ("notarytool submit".into(), ok(ACCEPTED)),
                    (
                        "notarytool log".into(),
                        ok("{\"status\": \"Accepted\", \"issues\": null}"),
                    ),
                    ("spctl".into(), ok("/somewhere/Smoozy.app: accepted\n")),
                ],
                unrunnable: Vec::new(),
                asked: Vec::new(),
            }
        }

        /// Put one answer ahead of the script's own.
        fn answering(mut self, to: &str, with: WhatAToolAnswered) -> Self {
            self.answers.insert(0, (to.into(), with));
            self
        }

        fn without(mut self, program: &str) -> Self {
            self.unrunnable.push(program.into());
            self
        }

        fn asked_anything_like(&self, what: &str) -> bool {
            self.asked.iter().any(|it| it.contains(what))
        }

        /// **Whether anything was asked that changes the bundle or reaches
        /// Apple with it.**
        fn touched_the_bundle(&self) -> bool {
            self.asked_anything_like("codesign --force")
                || self.asked_anything_like("notarytool submit")
                || self.asked_anything_like("stapler staple")
                || self.asked_anything_like("ditto")
        }
    }

    impl TheMacsTools for AScriptedMac {
        fn run(&mut self, program: &str, arguments: &[&str]) -> Result<WhatAToolAnswered, String> {
            let line = std::iter::once(program)
                .chain(arguments.iter().copied())
                .collect::<Vec<_>>()
                .join(" ");
            self.asked.push(line.clone());
            if self.unrunnable.iter().any(|it| it == program) {
                return Err("No such file or directory (os error 2)".into());
            }
            let answer = self
                .answers
                .iter()
                .find(|(key, _)| line.contains(key.as_str()))
                .map(|(_, answer)| answer.clone())
                .unwrap_or_else(|| ok(""));
            // **`ditto` writes its archive**, as the real one would.
            if program == "ditto" && answer.ending == Some(0) {
                std::fs::write(arguments.last().unwrap(), b"a zip archive of the bundle").unwrap();
            }
            Ok(answer)
        }
    }

    fn run_it(
        mac: &mut AScriptedMac,
        bundle: &Path,
        into: &Path,
    ) -> (Result<WhatWasMade, WhyTheBundleWasNotSigned>, Vec<String>) {
        let mut told = Vec::new();
        let made = sign_and_notarize(bundle, into, mac, &mut |line| told.push(line));
        (made, told)
    }

    // ----------------------------------------------------- the whole step

    /// **The whole step, in Apple's order, with the download written last.**
    #[test]
    fn a_mac_with_both_credentials_signs_sends_staples_checks_and_writes_the_download() {
        let at = a_folder("whole");
        let bundle = a_bundle(&at);
        let into = at.join("Smoozy-macos.zip");
        let mut mac = AScriptedMac::where_everything_is_set_up();
        let (made, told) = run_it(&mut mac, &bundle, &into);
        let made = made.expect("everything is set up");

        let b = bundle.to_str().unwrap();
        let submission = at.join("Smoozy-macos — sent for notarization.zip");
        let s = submission.to_str().unwrap();
        let i = into.to_str().unwrap();
        let plist = bundle.join("Contents").join("Info.plist");
        let p = plist.to_str().unwrap();
        let expected = vec![
            format!("plutil -extract CFBundleExecutable raw -o - {p}"),
            format!("plutil -extract CFBundleIdentifier raw -o - {p}"),
            "security find-identity -v -p codesigning".to_owned(),
            format!("xcrun notarytool history --keychain-profile {THE_NOTARIZATION_PROFILE} --no-progress"),
            format!("codesign --force --timestamp --options runtime --sign ADC03B244F4C1018384DCAFFC920F26136F6B59B {b}"),
            format!("codesign --verify --deep --strict -vvv {b}"),
            format!("codesign --display -vv {b}"),
            format!("codesign --display --entitlements - --xml {b}"),
            format!("ditto -c -k --keepParent {b} {s}"),
            format!("xcrun notarytool submit {s} --keychain-profile {THE_NOTARIZATION_PROFILE} --wait --timeout 1h --no-progress"),
            format!("xcrun notarytool log 2efe2717-52ef-43a5-96dc-0797e4ca1041 --keychain-profile {THE_NOTARIZATION_PROFILE}"),
            format!("xcrun stapler staple {b}"),
            format!("xcrun stapler validate {b}"),
            format!("spctl -vvv --assess --type exec {b}"),
            format!("ditto -c -k --keepParent {b} {i}"),
        ];
        assert_eq!(mac.asked, expected);

        assert_eq!(
            made.signed_by,
            "Developer ID Application: A Creator (ABCDE12345)"
        );
        assert_eq!(made.the_submission, "2efe2717-52ef-43a5-96dc-0797e4ca1041");
        assert!(made.the_log.contains("Accepted"));
        assert_eq!(made.the_download, into);
        assert_eq!(made.its_length, b"a zip archive of the bundle".len() as u64);
        assert_eq!(
            made.its_digest,
            Digest::of(b"a zip archive of the bundle").as_text()
        );
        // **The archive sent to Apple lacks the ticket and is gone.**
        assert!(!submission.exists());
        assert!(told.iter().any(|it| it.contains("Apple accepted")));
        let _ = std::fs::remove_dir_all(&at);
    }

    // --------------------------------------- refusals before any touching

    /// **The step stopped with the bundle untouched**: nothing was asked that
    /// signs it, sends it, staples it or zips it, and no download exists.
    fn stopped_untouched(
        made: &Result<WhatWasMade, WhyTheBundleWasNotSigned>,
        mac: &AScriptedMac,
        into: &Path,
    ) {
        let why = made.as_ref().expect_err("it should have stopped");
        assert_eq!(why.the_bundle_now(), TheBundleNow::Untouched, "{why:?}");
        assert!(!mac.touched_the_bundle(), "{:?}", mac.asked);
        assert!(!into.exists());
        assert!(in_words(why).contains("The bundle was not touched"));
    }

    #[test]
    fn no_developer_id_identity_stops_before_the_credential_and_before_the_bundle() {
        let at = a_folder("no-identity");
        let bundle = a_bundle(&at);
        let into = at.join("Smoozy-macos.zip");
        // **The listing observed on this Mac, 2026-10-24.**
        let mut mac = AScriptedMac::where_everything_is_set_up().answering(
            "security find-identity",
            ok("     0 valid identities found\n"),
        );
        let (made, _) = run_it(&mut mac, &bundle, &into);
        assert!(matches!(made, Err(ThereIsNoDeveloperIdIdentity)));
        stopped_untouched(&made, &mac, &into);
        // **The credential is not even asked about**: the keychain is read once.
        assert!(!mac.asked_anything_like("notarytool"));
        let said = in_words(made.as_ref().unwrap_err());
        assert!(said.contains("Creator's to set up"));
        assert!(said.contains("App/Release/About this folder.md"));
        let _ = std::fs::remove_dir_all(&at);
    }

    /// **Other kinds of identity are not a Developer ID** — Apple's page names
    /// Apple Development and Apple Distribution as the wrong ones.
    #[test]
    fn only_a_developer_id_application_identity_counts() {
        let at = a_folder("other-kinds");
        let bundle = a_bundle(&at);
        let into = at.join("Smoozy-macos.zip");
        let listed = "  1) A06E7F3F8237330EE15CB91BE1A511C00B853358 \"Apple Distribution: A Creator (ABCDE12345)\"\n  2) B06E7F3F8237330EE15CB91BE1A511C00B853358 \"Apple Development: A Creator (ABCDE12345)\"\n  3) C06E7F3F8237330EE15CB91BE1A511C00B853358 \"Developer ID Installer: A Creator (ABCDE12345)\"\n     3 valid identities found\n";
        let mut mac = AScriptedMac::where_everything_is_set_up()
            .answering("security find-identity", ok(listed));
        let (made, _) = run_it(&mut mac, &bundle, &into);
        assert!(matches!(made, Err(ThereIsNoDeveloperIdIdentity)));
        stopped_untouched(&made, &mac, &into);
        let _ = std::fs::remove_dir_all(&at);
    }

    #[test]
    fn two_developer_id_identities_are_named_and_neither_is_chosen() {
        let at = a_folder("two-identities");
        let bundle = a_bundle(&at);
        let into = at.join("Smoozy-macos.zip");
        let listed = "  1) ADC03B244F4C1018384DCAFFC920F26136F6B59B \"Developer ID Application: A Creator (ABCDE12345)\"\n  2) BDC03B244F4C1018384DCAFFC920F26136F6B59B \"Developer ID Application: A Creator (ABCDE12345)\"\n     2 valid identities found\n";
        let mut mac = AScriptedMac::where_everything_is_set_up()
            .answering("security find-identity", ok(listed));
        let (made, _) = run_it(&mut mac, &bundle, &into);
        match &made {
            Err(ThereIsMoreThanOneDeveloperIdIdentity { named }) => assert_eq!(named.len(), 2),
            other => panic!("{other:?}"),
        }
        stopped_untouched(&made, &mac, &into);
        let _ = std::fs::remove_dir_all(&at);
    }

    #[test]
    fn a_listing_that_cannot_be_read_whole_stops_rather_than_hiding_an_identity() {
        let at = a_folder("listing");
        let bundle = a_bundle(&at);
        let into = at.join("Smoozy-macos.zip");
        for (listed, ending) in [
            // The count says one and no line could be read.
            ("  1) not-a-hash \"Developer ID Application: A Creator\"\n     1 valid identities found\n", 0),
            // No count at all.
            ("something else entirely\n", 0),
            // The tool refused.
            ("", 1),
        ] {
            let mut mac = AScriptedMac::where_everything_is_set_up().answering(
                "security find-identity",
                WhatAToolAnswered {
                    ending: Some(ending),
                    out: listed.into(),
                    said: String::new(),
                },
            );
            let (made, _) = run_it(&mut mac, &bundle, &into);
            assert!(matches!(made, Err(TheIdentitiesCouldNotBeListed { .. })), "{made:?}");
            stopped_untouched(&made, &mac, &into);
        }
        let _ = std::fs::remove_dir_all(&at);
    }

    #[test]
    fn a_credential_that_does_not_answer_stops_before_the_bundle_and_asserts_no_cause() {
        let at = a_folder("no-credential");
        let bundle = a_bundle(&at);
        let into = at.join("Smoozy-macos.zip");
        let mut mac = AScriptedMac::where_everything_is_set_up().answering(
            "notarytool history",
            ended(
                69,
                "Error: No Keychain password item found for profile: Smoozy notarization",
            ),
        );
        let (made, _) = run_it(&mut mac, &bundle, &into);
        assert!(matches!(
            made,
            Err(TheNotarizationCredentialDidNotAnswer { .. })
        ));
        stopped_untouched(&made, &mac, &into);
        let said = in_words(made.as_ref().unwrap_err());
        assert!(said.contains("may not be stored under that name"));
        assert!(said.contains("refused by Apple"));
        assert!(said.contains("may not have been reached"));
        assert!(said.contains("No Keychain password item found"));
        assert!(said.contains("App/Release/About this folder.md"));
        // **The identity was looked for first**, so the credential is only asked
        // about once there is something to sign with.
        let identity = mac
            .asked
            .iter()
            .position(|it| it.starts_with("security"))
            .unwrap();
        let credential = mac
            .asked
            .iter()
            .position(|it| it.contains("history"))
            .unwrap();
        assert!(identity < credential);
        let _ = std::fs::remove_dir_all(&at);
    }

    #[test]
    fn a_mac_without_the_tools_says_so_and_touches_nothing() {
        let at = a_folder("no-tools");
        let bundle = a_bundle(&at);
        let into = at.join("Smoozy-macos.zip");
        for missing in ["plutil", "security", "xcrun"] {
            let mut mac = AScriptedMac::where_everything_is_set_up().without(missing);
            let (made, _) = run_it(&mut mac, &bundle, &into);
            match &made {
                Err(AToolCouldNotBeRun { tool, .. }) => assert!(tool.starts_with(missing)),
                other => panic!("{missing}: {other:?}"),
            }
            stopped_untouched(&made, &mac, &into);
        }
        let _ = std::fs::remove_dir_all(&at);
    }

    #[test]
    fn a_download_place_that_is_not_usable_stops_before_anything_is_asked() {
        let at = a_folder("into");
        let bundle = a_bundle(&at);
        std::fs::write(at.join("already.zip"), b"x").unwrap();
        std::fs::write(at.join("taken — sent for notarization.zip"), b"x").unwrap();
        for into in [
            at.join("already.zip"),
            at.join("no such folder").join("Smoozy-macos.zip"),
            at.join("Smoozy-macos.dmg"),
            at.join("taken.zip"),
        ] {
            let mut mac = AScriptedMac::where_everything_is_set_up();
            let (made, _) = run_it(&mut mac, &bundle, &into);
            assert!(
                matches!(made, Err(TheDownloadsPlaceIsNotUsable { .. })),
                "{into:?}"
            );
            assert!(mac.asked.is_empty(), "{:?}", mac.asked);
        }
        let _ = std::fs::remove_dir_all(&at);
    }

    #[test]
    fn what_is_not_a_bundle_is_refused_before_the_keychain_is_asked() {
        let at = a_folder("not-a-bundle");
        let into = at.join("Smoozy-macos.zip");

        // Missing, not a folder, and not ending in .app.
        std::fs::write(at.join("a file.app"), b"x").unwrap();
        std::fs::create_dir_all(at.join("Smoozy")).unwrap();
        for path in [
            at.join("missing.app"),
            at.join("a file.app"),
            at.join("Smoozy"),
        ] {
            let mut mac = AScriptedMac::where_everything_is_set_up();
            let (made, _) = run_it(&mut mac, &path, &into);
            assert!(matches!(made, Err(ItIsNotABundle { .. })), "{path:?}");
            assert!(mac.asked.is_empty());
        }

        // No Info.plist.
        let bundle = a_bundle(&at);
        std::fs::remove_file(bundle.join("Contents/Info.plist")).unwrap();
        let mut mac = AScriptedMac::where_everything_is_set_up();
        let (made, _) = run_it(&mut mac, &bundle, &into);
        assert!(matches!(made, Err(ItIsNotABundle { .. })));
        assert!(mac.asked.is_empty());
        std::fs::write(bundle.join("Contents/Info.plist"), b"<plist/>").unwrap();

        // An Info.plist naming no program, or no identifier.
        for key in ["CFBundleExecutable", "CFBundleIdentifier"] {
            let mut mac = AScriptedMac::where_everything_is_set_up().answering(
                &format!("plutil -extract {key}"),
                ended(1, "Could not extract value"),
            );
            let (made, _) = run_it(&mut mac, &bundle, &into);
            assert!(matches!(made, Err(ItIsNotABundle { .. })), "{key}");
            stopped_untouched(&made, &mac, &into);
            assert!(!mac.asked_anything_like("security"));
        }

        // A main program that is missing, or is not a program.
        let mut mac = AScriptedMac::where_everything_is_set_up()
            .answering("plutil -extract CFBundleExecutable", ok("someone-else\n"));
        let (made, _) = run_it(&mut mac, &bundle, &into);
        assert!(matches!(made, Err(ItIsNotABundle { .. })));
        assert!(!mac.asked_anything_like("security"));
        std::fs::write(bundle.join("Contents/MacOS/smoozy-desktop"), b"#!/bin/sh\n").unwrap();
        let mut mac = AScriptedMac::where_everything_is_set_up();
        let (made, _) = run_it(&mut mac, &bundle, &into);
        match &made {
            Err(ItIsNotABundle { said }) => assert!(said.contains("is not a program")),
            other => panic!("{other:?}"),
        }
        assert!(!mac.asked_anything_like("security"));
        let _ = std::fs::remove_dir_all(&at);
    }

    /// **A bundle with more code than its main program is refused, never
    /// half-signed.**
    #[test]
    fn code_beside_the_main_program_is_named_and_the_bundle_is_left_alone() {
        let at = a_folder("more-code");
        let into = at.join("Smoozy-macos.zip");
        let cases: [(&str, fn(&Path)); 4] = [
            ("Contents/MacOS/a-helper", |b| {
                std::fs::write(b.join("Contents/MacOS/a-helper"), b"#!/bin/sh\n").unwrap()
            }),
            ("Contents/Resources/smoozy-server", |b| {
                std::fs::write(b.join("Contents/Resources/smoozy-server"), A_PROGRAM).unwrap()
            }),
            ("Contents/Frameworks/Core.framework", |b| {
                std::fs::create_dir_all(b.join("Contents/Frameworks/Core.framework")).unwrap()
            }),
            ("Contents/Resources/fat", |b| {
                std::fs::write(b.join("Contents/Resources/fat"), [0xca, 0xfe, 0xba, 0xbe]).unwrap()
            }),
        ];
        for (named, add) in cases {
            let _ = std::fs::remove_dir_all(at.join("Smoozy.app"));
            let bundle = a_bundle(&at);
            add(&bundle);
            let mut mac = AScriptedMac::where_everything_is_set_up();
            let (made, _) = run_it(&mut mac, &bundle, &into);
            match &made {
                Err(ItCarriesCodeBesideItsMainProgram { found }) => {
                    assert_eq!(found.iter().map(Path::new).collect::<Vec<_>>(), vec![Path::new(named)])
                }
                other => panic!("{named}: {other:?}"),
            }
            stopped_untouched(&made, &mac, &into);
            assert!(!mac.asked_anything_like("security"));
        }
        let _ = std::fs::remove_dir_all(&at);
    }

    /// **The shell's own bundle shape passes**: a privacy file and an icon in
    /// Resources are not code.
    #[test]
    fn the_shells_own_bundle_carries_no_other_code() {
        let at = a_folder("shell-shape");
        let bundle = a_bundle(&at);
        let program = bundle.join("Contents/MacOS/smoozy-desktop");
        assert!(the_other_code_in(&bundle, &program).unwrap().is_empty());
        assert!(is_mach_o(&program).unwrap());
        let _ = std::fs::remove_dir_all(&at);
    }

    // ------------------------------------------ refusals after touching

    /// **The step stopped after touching the bundle**, wrote no download, and
    /// says the bundle must not be published.
    fn stopped_touched(
        made: &Result<WhatWasMade, WhyTheBundleWasNotSigned>,
        into: &Path,
        now: TheBundleNow,
    ) {
        let why = made.as_ref().expect_err("it should have stopped");
        assert_eq!(why.the_bundle_now(), now, "{why:?}");
        assert!(!into.exists(), "a download was written after {why:?}");
        assert!(in_words(why).contains("must not be published"));
    }

    #[test]
    fn a_signing_that_fails_sends_nothing() {
        let at = a_folder("sign-fails");
        let bundle = a_bundle(&at);
        let into = at.join("Smoozy-macos.zip");
        let mut mac = AScriptedMac::where_everything_is_set_up().answering(
            "codesign --force",
            ended(1, "The timestamp service is not available."),
        );
        let (made, _) = run_it(&mut mac, &bundle, &into);
        assert!(matches!(made, Err(TheSigningFailed { .. })));
        stopped_touched(&made, &into, TheBundleNow::SignedAndNotNotarized);
        assert!(!mac.asked_anything_like("notarytool submit"));
        let _ = std::fs::remove_dir_all(&at);
    }

    /// **Each of Apple's requirements, read back off the signature, stops the
    /// step before anything is sent.**
    #[test]
    fn a_signature_short_of_any_requirement_is_not_sent() {
        let at = a_folder("signature");
        let bundle = a_bundle(&at);
        let into = at.join("Smoozy-macos.zip");
        let shown = |text: String| WhatAToolAnswered {
            ending: Some(0),
            out: String::new(),
            said: text,
        };
        let cases = vec![
            ("codesign --verify", ended(1, "a sealed resource is missing or invalid"), "strict check"),
            (
                "codesign --display -vv",
                shown(A_GOOD_SIGNATURE.replace("Timestamp=24 Oct 2026 at 12:00:00", "Signed Time=24 Oct 2026 at 12:00:00")),
                "secure timestamp",
            ),
            (
                "codesign --display -vv",
                shown(A_GOOD_SIGNATURE.replace("flags=0x10000(runtime)", "flags=0x0(none)")),
                "hardened runtime",
            ),
            (
                "codesign --display -vv",
                shown(A_GOOD_SIGNATURE.replace("Authority=Developer ID Application:", "Authority=Apple Development:")),
                "Developer ID Application",
            ),
            (
                "codesign --display --entitlements",
                ok("<plist><dict><key>com.apple.security.get-task-allow</key><true/></dict></plist>"),
                "get-task-allow",
            ),
        ];
        for (to, with, what_is_said) in cases {
            let mut mac = AScriptedMac::where_everything_is_set_up().answering(to, with);
            let (made, _) = run_it(&mut mac, &bundle, &into);
            match &made {
                Err(TheSignatureIsNotWhatNotarizationNeeds { what, .. }) => {
                    assert!(what.contains(what_is_said), "{what}")
                }
                other => panic!("{what_is_said}: {other:?}"),
            }
            stopped_touched(&made, &into, TheBundleNow::SignedAndNotNotarized);
            assert!(
                !mac.asked_anything_like("notarytool submit"),
                "{what_is_said}"
            );
            assert!(!mac.asked_anything_like("ditto"), "{what_is_said}");
        }
        let _ = std::fs::remove_dir_all(&at);
    }

    /// **Apple's refusal comes back with its log, the archive sent is removed,
    /// and nothing is stapled or written.**
    #[test]
    fn a_submission_apple_does_not_accept_comes_back_with_its_log() {
        let at = a_folder("invalid");
        let bundle = a_bundle(&at);
        let into = at.join("Smoozy-macos.zip");
        let invalid = ok(&ACCEPTED.replace("status: Accepted", "status: Invalid"));
        let mut mac = AScriptedMac::where_everything_is_set_up()
            .answering(
                "notarytool submit",
                WhatAToolAnswered {
                    ending: Some(1),
                    ..invalid
                },
            )
            .answering(
                "notarytool log",
                ok("{\"issues\": [{\"message\": \"The signature of the binary is invalid.\"}]}"),
            );
        let (made, _) = run_it(&mut mac, &bundle, &into);
        match &made {
            Err(AppleDidNotAcceptIt {
                status,
                submission,
                the_log,
                ..
            }) => {
                assert_eq!(status.as_deref(), Some("Invalid"));
                assert_eq!(
                    submission.as_deref(),
                    Some("2efe2717-52ef-43a5-96dc-0797e4ca1041")
                );
                assert!(the_log
                    .as_deref()
                    .unwrap()
                    .contains("signature of the binary is invalid"));
            }
            other => panic!("{other:?}"),
        }
        stopped_touched(&made, &into, TheBundleNow::SignedAndNotNotarized);
        assert!(in_words(made.as_ref().unwrap_err()).contains("signature of the binary is invalid"));
        assert!(!mac.asked_anything_like("stapler"));
        assert!(!at.join("Smoozy-macos — sent for notarization.zip").exists());
        let _ = std::fs::remove_dir_all(&at);
    }

    /// **An answer that is not *Accepted* is never taken as one** — a wait that
    /// ran out, a tool that ended well without a status, and a status that says
    /// Accepted from a tool that ended badly.
    #[test]
    fn nothing_but_an_accepted_status_from_a_tool_that_ended_well_counts() {
        let at = a_folder("not-accepted");
        let bundle = a_bundle(&at);
        let into = at.join("Smoozy-macos.zip");
        let in_progress = ok(&ACCEPTED.replace("status: Accepted", "status: In Progress"));
        let no_status = ok(&ACCEPTED.replace("  status: Accepted\n", ""));
        let ended_badly = WhatAToolAnswered {
            ending: Some(1),
            ..ok(ACCEPTED)
        };
        for answer in [in_progress, no_status, ended_badly] {
            let mut mac =
                AScriptedMac::where_everything_is_set_up().answering("notarytool submit", answer);
            let (made, _) = run_it(&mut mac, &bundle, &into);
            assert!(matches!(made, Err(AppleDidNotAcceptIt { .. })), "{made:?}");
            stopped_touched(&made, &into, TheBundleNow::SignedAndNotNotarized);
            assert!(!mac.asked_anything_like("stapler"));
        }
        let mut mac = AScriptedMac::where_everything_is_set_up().answering(
            "notarytool submit",
            ok(&ACCEPTED.replace("status: Accepted", "status: In Progress")),
        );
        let (made, _) = run_it(&mut mac, &bundle, &into);
        assert!(in_words(made.as_ref().unwrap_err()).contains("may still be working"));
        let _ = std::fs::remove_dir_all(&at);
    }

    #[test]
    fn a_submission_that_never_reached_apple_says_so() {
        let at = a_folder("unsent");
        let bundle = a_bundle(&at);
        let into = at.join("Smoozy-macos.zip");
        let mut mac = AScriptedMac::where_everything_is_set_up().answering(
            "notarytool submit",
            ended(1, "Error: The Internet connection appears to be offline."),
        );
        let (made, _) = run_it(&mut mac, &bundle, &into);
        assert!(
            matches!(made, Err(TheSubmissionCouldNotBeMade { .. })),
            "{made:?}"
        );
        stopped_touched(&made, &into, TheBundleNow::SignedAndNotNotarized);
        assert!(!mac.asked_anything_like("notarytool log"));

        let mut mac = AScriptedMac::where_everything_is_set_up()
            .answering("ditto", ended(1, "No space left on device"));
        let (made, _) = run_it(&mut mac, &bundle, &into);
        assert!(
            matches!(made, Err(TheSubmissionCouldNotBeMade { .. })),
            "{made:?}"
        );
        assert!(!mac.asked_anything_like("notarytool submit"));
        stopped_touched(&made, &into, TheBundleNow::SignedAndNotNotarized);
        let _ = std::fs::remove_dir_all(&at);
    }

    #[test]
    fn a_ticket_that_is_not_stapled_or_does_not_check_out_writes_no_download() {
        let at = a_folder("ticket");
        let bundle = a_bundle(&at);
        let into = at.join("Smoozy-macos.zip");
        for (to, expected) in [
            ("stapler staple", "staple"),
            ("stapler validate", "validate"),
            ("spctl", "spctl"),
        ] {
            let mut mac =
                AScriptedMac::where_everything_is_set_up().answering(to, ended(65, "rejected"));
            let (made, _) = run_it(&mut mac, &bundle, &into);
            let why = made.as_ref().expect_err(expected);
            match (expected, why) {
                ("staple", TheTicketCouldNotBeStapled { .. })
                | ("validate", TheStapledTicketDidNotCheckOut { .. })
                | ("spctl", TheSystemsOwnCheckRefusedIt { .. }) => {}
                (_, other) => panic!("{expected}: {other:?}"),
            }
            stopped_touched(&made, &into, TheBundleNow::NotarizedAndNotFinished);
        }
        let _ = std::fs::remove_dir_all(&at);
    }

    #[test]
    fn a_download_that_cannot_be_written_is_said_and_nothing_is_called_written() {
        let at = a_folder("download");
        let bundle = a_bundle(&at);
        let into = at.join("Smoozy-macos.zip");
        let mut mac = AScriptedMac::where_everything_is_set_up().answering(
            &format!("{} {}", bundle.display(), into.display()),
            ended(1, "No space left on device"),
        );
        let (made, _) = run_it(&mut mac, &bundle, &into);
        assert!(
            matches!(made, Err(TheDownloadCouldNotBeWritten { .. })),
            "{made:?}"
        );
        stopped_touched(&made, &into, TheBundleNow::Finished);
        let _ = std::fs::remove_dir_all(&at);
    }

    // ------------------------------------------------------------ readings

    #[test]
    fn the_identity_listing_is_read_as_apples_sample_shows_it() {
        // **Apple's own sample**, in *Creating distribution-signed code for macOS*.
        let apples = "  1) A06E7F3F8237330EE15CB91BE1A511C00B853358 \"Apple Distribution: …\"\n  2) ADC03B244F4C1018384DCAFFC920F26136F6B59B \"Developer ID Application: …\"\n     2 valid identities found\n";
        let read = the_identities_in(apples).unwrap();
        assert_eq!(read.len(), 2);
        assert_eq!(read[1].0, "ADC03B244F4C1018384DCAFFC920F26136F6B59B");
        assert_eq!(read[1].1, "Developer ID Application: …");
        // **The empty listing observed on this Mac.**
        assert!(the_identities_in("     0 valid identities found\n")
            .unwrap()
            .is_empty());
        // A count with no matching lines, and no count, are both refused.
        assert!(the_identities_in("     1 valid identities found\n").is_err());
        assert!(the_identities_in("").is_err());
    }

    #[test]
    fn the_submission_and_its_last_status_are_read_off_apples_sample() {
        let (id, status) = the_submission_and_its_status(ACCEPTED);
        assert_eq!(id.as_deref(), Some("2efe2717-52ef-43a5-96dc-0797e4ca1041"));
        assert_eq!(status.as_deref(), Some("Accepted"));
        let (id, status) = the_submission_and_its_status("nothing at all");
        assert!(id.is_none() && status.is_none());
    }

    /// **Every ending says something of its own**, and none of them could be
    /// mistaken for another.
    #[test]
    fn every_ending_says_something_of_its_own_and_what_the_bundle_now_is() {
        let every = vec![
            TheDownloadsPlaceIsNotUsable { said: "a".into() },
            ItIsNotABundle { said: "a".into() },
            ItCarriesCodeBesideItsMainProgram {
                found: vec!["a".into()],
            },
            ThereIsNoDeveloperIdIdentity,
            ThereIsMoreThanOneDeveloperIdIdentity {
                named: vec!["a".into(), "b".into()],
            },
            TheIdentitiesCouldNotBeListed { said: "a".into() },
            TheNotarizationCredentialDidNotAnswer { said: "a".into() },
            AToolCouldNotBeRun {
                tool: "xcrun".into(),
                said: "a".into(),
                the_bundle: TheBundleNow::Untouched,
            },
            TheSigningFailed { said: "a".into() },
            TheSignatureIsNotWhatNotarizationNeeds {
                what: "a".into(),
                said: "a".into(),
            },
            TheSubmissionCouldNotBeMade { said: "a".into() },
            AppleDidNotAcceptIt {
                status: Some("Invalid".into()),
                submission: None,
                said: "a".into(),
                the_log: None,
            },
            TheTicketCouldNotBeStapled { said: "a".into() },
            TheStapledTicketDidNotCheckOut { said: "a".into() },
            TheSystemsOwnCheckRefusedIt { said: "a".into() },
            TheDownloadCouldNotBeWritten { said: "a".into() },
        ];
        let said: Vec<String> = every.iter().map(in_words).collect();
        for at in 0..said.len() {
            assert!(said[at].len() > 60, "{}", said[at]);
            for beside in at + 1..said.len() {
                assert_ne!(said[at], said[beside]);
            }
            match every[at].the_bundle_now() {
                TheBundleNow::Untouched => assert!(said[at].contains("not touched")),
                _ => assert!(said[at].contains("must not be published")),
            }
            // **No ending claims the bundle was signed when it was not.**
            if every[at].the_bundle_now() == TheBundleNow::Untouched {
                assert!(!said[at].contains("is now signed"));
            }
        }
    }

    // ------------------------------------------------------- the desk run

    /// **The desk's tools, allowed only the reads a desk run may make**: the
    /// bundle's own property list, and the keychain's list of signing
    /// identities. Anything else — the notarization credential, signing,
    /// sending, stapling, zipping — is refused here before it runs, so this run
    /// can never go further than the line, whatever the keychain holds.
    struct OnlyTheReadsADeskRunMayMake {
        asked: Vec<String>,
    }

    impl TheMacsTools for OnlyTheReadsADeskRunMayMake {
        fn run(&mut self, program: &str, arguments: &[&str]) -> Result<WhatAToolAnswered, String> {
            self.asked
                .push(format!("{program} {}", arguments.join(" ")));
            let allowed = (program == "plutil" && arguments.first() == Some(&"-extract"))
                || (program == "security"
                    && arguments == ["find-identity", "-v", "-p", "codesigning"]);
            if allowed {
                TheToolsOnThisMac.run(program, arguments)
            } else {
                Err("a desk run stops at the line and may not ask this".into())
            }
        }
    }

    fn every_byte_of(folder: &Path) -> Vec<(PathBuf, Vec<u8>)> {
        let mut out = Vec::new();
        let mut waiting = vec![folder.to_path_buf()];
        while let Some(at) = waiting.pop() {
            for entry in std::fs::read_dir(&at).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    waiting.push(path);
                } else {
                    out.push((path.clone(), std::fs::read(&path).unwrap()));
                }
            }
        }
        out.sort();
        out
    }

    /// **The step run once on a real ad-hoc bundle built at the desk, on this
    /// Mac's real tools, stopping where it should.**
    ///
    /// Ignored, because it needs a Mac and a built bundle. Run it with the
    /// bundle's path in `SMOOZY_RELEASE_AN_AD_HOC_BUNDLE` and `--ignored`. It
    /// never opens or runs the bundle; it reads its property list and the
    /// keychain's list of identities, and nothing else.
    #[test]
    #[ignore]
    fn the_step_on_an_ad_hoc_bundle_at_the_desk_stops_before_touching_it() {
        let bundle = PathBuf::from(
            std::env::var("SMOOZY_RELEASE_AN_AD_HOC_BUNDLE")
                .expect("SMOOZY_RELEASE_AN_AD_HOC_BUNDLE names the bundle"),
        );
        let at = a_folder("desk");
        let into = at.join("Smoozy-macos.zip");
        let before = every_byte_of(&bundle);
        let mut tools = OnlyTheReadsADeskRunMayMake { asked: Vec::new() };
        let mut told = Vec::new();
        let made = sign_and_notarize(&bundle, &into, &mut tools, &mut |line| told.push(line));

        let why = made.expect_err("a desk without the Creator's credentials stops");
        println!("asked:\n  {}", tools.asked.join("\n  "));
        println!("told:\n  {}", told.join("\n  "));
        println!("ended:\n{}", in_words(&why));
        assert_eq!(why.the_bundle_now(), TheBundleNow::Untouched);
        assert!(matches!(
            why,
            ThereIsNoDeveloperIdIdentity | AToolCouldNotBeRun { .. }
        ));
        assert_eq!(every_byte_of(&bundle), before, "the bundle changed");
        assert!(!into.exists());
        let _ = std::fs::remove_dir_all(&at);
    }
}

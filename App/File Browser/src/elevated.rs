//! The elevated acts — `File Browser` P6 and P6b.
//!
//! **Constraint:** Row 180 — *"And where the machine allows it, Smoozy can
//! offer to try again with **elevated access**, rather than only reporting the
//! refusal."* P6: *"An elevated listing is therefore a command-channel reading
//! of the same four fields in the epoch form … and an elevated file operation is
//! the machine's own remove, move, copy or make-directory under the elevating
//! command."*
//!
//! **What this file holds is what crosses to the machine and what comes back**,
//! and nothing that runs: the constant commands, the paths file's token, the
//! checks every path passes before anything is sent, the bytes the paths file
//! holds, and the reading of what the command answered. The desktop runs them.
//!
//! **P6b, as changed after Sol's look (2026-11-12), in one paragraph.** Every
//! act is done inside **one folder, opened once**, by names relative to it:
//! the script under the elevating command changes into the folder (`cd -P`),
//! reads where it is (`pwd -P`), and goes on only where that is byte for byte
//! the folder the user saw. A folder on the path swapped for a link after that
//! cannot move the act; one swapped before fails the comparison. **The act's
//! values ride a file** written through the subsystem at a fixed token, each
//! ended by NUL, which the constant command opens as `xargs -0`'s standard
//! input — POSIX.1-2024 `xargs`, `-0`: *"Use a null byte as the input argument
//! delimiter and do not treat any other input bytes as special."* **No path is
//! ever in a command string.** The file's first field names the act and its
//! second is a mark made for this act alone, which the script prints first, as
//! root: so an answer without it is the elevating command's own refusal, and
//! an answer carrying another act's mark is not this act's. **The elevating
//! command is `sudo -k`**, so the password, sent once on its own input, is
//! read by it every time and never reaches the act, and no cached credential
//! is used or made.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` 1.1. `Server Setup` E3, E4.
//! `State, identity and recovery` §2.1.

use smoozy_server_protocol::carriage::CarriageSite;
use smoozy_server_protocol::subsystem::{Attributes, Entry};
use smoozy_server_protocol::transport::CommandString;

use crate::permission::{accept_an_elevated_listing, AnElevatedListing, WhyTheElevatedListingIsUnavailable};

/// **The paths file**: every act's values, each ended by NUL, written through
/// the subsystem at a fixed token (P6b). Not a link: a file.
pub const THE_PATHS_FILE: CarriageSite = CarriageSite {
    link_path_on_the_subsystem: ".smoozy-carriage/files-elevated-paths",
    named_in_a_command_as: "~/.smoozy-carriage/files-elevated-paths",
};

/// **One elevated act, as the user asked for it**, always as a folder and the
/// names inside it (P6b, as changed after Sol's look). **A move or a copy
/// between two folders is not here**: it needs two folders opened at once, and
/// one shell holds one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnElevatedAct {
    /// The folder the login could not read.
    AListing { folder: Vec<u8> },
    /// One item in `folder`, removed with everything under it.
    Delete { folder: Vec<u8>, name: Vec<u8> },
    /// A new folder named `name` in `folder`, handed to the login that asked
    /// (the runner's ruling 1 of 2026-11-12).
    NewFolder { folder: Vec<u8>, name: Vec<u8> },
    /// One item in `folder` given a new name in the same folder.
    Rename { folder: Vec<u8>, from: Vec<u8>, to: Vec<u8> },
}

/// **One act's command, in its two elevated forms and the plain script they
/// carry.** Every one a constant; the elevated forms are written out rather
/// than built.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheCommand {
    pub plain: CommandString,
    /// Behind `sudo -k -n`, for passwordless elevation.
    pub passwordless: CommandString,
    /// Behind `sudo -k -S`, the password on standard input (`Server Setup`
    /// E3), read by the elevating command itself every time.
    pub with_a_password: CommandString,
}

/// **The first words of the act's mark line**, which the script prints as
/// root before anything else, followed by the act's own mark.
pub const THE_MARK_LINE_OPENS: &str = "smoozy-elevated ";

/// **The line the command prints as root before it opens the paths file**
/// (after Sol's second look): no such line is the elevating command's own
/// refusal; this line and no act mark is a failure before the act began.
pub const ROOT_WAS_REACHED: &str = "smoozy-elevated: admin access";

/// **P5's probe as Files takes it** (after Sol's second look): `Server
/// Setup` M3's `sudo -n true` behind `-k`, so a credential cached by
/// something else is neither used nor kept fresh. `sudo(8)`, `-k`: *"will
/// cause sudo to ignore the user's cached credentials"* … *"and will not
/// update the user's cached credentials."*
pub const THE_PROBE_IGNORING_A_CACHED_CREDENTIAL: CommandString = CommandString::constant("sudo -k -n true");

/// **The marks the scripts print on their complaint stream, and only they**
/// (P6b). Each is this file's own constant, so matching one reads nothing of
/// the machine's.
pub const THE_LANDING_IS_TAKEN: &str = "smoozy-elevated: the landing path is taken";
/// The folder's path, read with `pwd -P` after `cd -P`, was not the one sent.
pub const THE_PATH_GOES_THROUGH_A_LINK: &str = "smoozy-elevated: the path goes through a link";
/// The paths file did not hold what this act's script takes.
pub const THE_REQUEST_IS_NOT_AS_SENT: &str = "smoozy-elevated: the request is not as sent";
/// The new folder was made and could not be handed to the login.
pub const MADE_BUT_OWNED_BY_ROOT: &str = "smoozy-elevated: made, owned by root";

/// **The script's opening, the same for every act**: the mark line, the
/// count and the act's name checked, each name checked, and the folder opened
/// and compared. `$1` is the act's name, `$2` its mark, `$3` the folder.
///
/// Written for `sh -c` inside single quotes, so it holds none; `$(pwd -P &&
/// echo x)` keeps a name ending in a newline whole, and `${here%?x}` takes off
/// the one newline `pwd` wrote and the `x`.
macro_rules! the_opening {
    ($act:literal, $count:literal, $names:literal) => {
        concat!(
            "printf \"smoozy-elevated %s\\n\" \"$2\"; ",
            "[ $# -eq ", $count, " ] && [ \"$1\" = ", $act, " ] || { echo \"smoozy-elevated: the request is not as sent\" >&2; exit 1; }; ",
            "for n in ", $names, "; do case $n in \"\"|.|..|*/*) echo \"smoozy-elevated: the request is not as sent\" >&2; exit 1;; esac; done; ",
            "cd -P \"$3\" || exit 1; ",
            "here=$(pwd -P && echo x); ",
            "[ \"${here%?x}\" = \"$3\" ] || { echo \"smoozy-elevated: the path goes through a link\" >&2; exit 1; }; "
        )
    };
}

/// **The whole command around a script**: `sh -c` opens the paths file as
/// `xargs -0`'s input, which hands its fields to the script as arguments.
macro_rules! the_command {
    ($prefix:literal, $script:expr) => {
        concat!(
            $prefix,
            "sh -c 'printf \"smoozy-elevated: admin access\\n\"; exec xargs -0 sh -c '\\''",
            $script,
            "'\\'' sh < \"$1\"' sh ~/.smoozy-carriage/files-elevated-paths"
        )
    };
}

macro_rules! three_forms {
    ($script:expr) => {
        TheCommand {
            plain: CommandString::constant(the_command!("", $script)),
            passwordless: CommandString::constant(the_command!("sudo -k -n ", $script)),
            with_a_password: CommandString::constant(the_command!("sudo -k -S ", $script)),
        }
    };
}

/// **The listing**: inside the opened folder, GNU `find`, no link followed,
/// four NUL-ended fields per entry.
const THE_LISTING: TheCommand = three_forms!(concat!(
    the_opening!("listing", "3", "x"),
    "exec find . -mindepth 1 -maxdepth 1 -printf \"%f\\0%s\\0%T@\\0%y\\0\""
));

/// **Delete**: `rm -r -f --one-file-system --` on the one name, inside the
/// opened folder. POSIX.1-2024 `rm`: *"shall not traverse directories by
/// following symbolic links into other parts of the hierarchy"*.
///
/// **`--one-file-system`, from 2026-11-12** (`File Browser` P6b, *two more
/// rules after Fable's fresh look*, finding 5): as root nothing under the
/// folder is refused, so a second disk mounted inside it was emptied too.
/// GNU coreutils manual 9.12, *rm invocation*, read 2026-11-12: *"When
/// removing a hierarchy recursively, do not remove any directory that is on a
/// file system different from that of the corresponding command line
/// argument."*
const THE_DELETE: TheCommand = three_forms!(concat!(
    the_opening!("delete", "4", "\"$4\""),
    "exec rm -r -f --one-file-system -- \"./$4\""
));

/// **New folder**: the plain `mkdir`, never `-p` (M9); then the new folder
/// itself opened and compared, and handed to the login that asked on the
/// opened folder, never on a path (the runner's ruling 1).
const THE_NEW_FOLDER: TheCommand = three_forms!(concat!(
    the_opening!("new-folder", "4", "\"$4\""),
    "mkdir -- \"./$4\" || exit 1; ",
    "case $3 in /) want=/$4;; *) want=$3/$4;; esac; ",
    "cd -P \"./$4\" && here=$(pwd -P && echo x) && [ \"${here%?x}\" = \"$want\" ] && [ -O . ] ",
    "&& for f in * .[!.]* ..?*; do if [ -e \"$f\" ] || [ -L \"$f\" ]; then echo \"smoozy-elevated: made, owned by root\" >&2; exit 1; fi; done ",
    "&& [ -n \"$SUDO_UID\" ] && [ -n \"$SUDO_GID\" ] && chown \"$SUDO_UID:$SUDO_GID\" . ",
    "|| { echo \"smoozy-elevated: made, owned by root\" >&2; exit 1; }"
));

/// **Rename**: the landing name checked inside the opened folder, then GNU
/// `mv -T`, which never moves into a folder or through a link that appears at
/// the landing name.
const THE_RENAME: TheCommand = three_forms!(concat!(
    the_opening!("rename", "5", "\"$4\" \"$5\""),
    "if [ -e \"./$5\" ] || [ -L \"./$5\" ]; then echo \"smoozy-elevated: the landing path is taken\" >&2; exit 1; fi; ",
    "exec mv -T -- \"./$4\" \"./$5\""
));

/// **`sudo -K`, a second guard and never the first** (P6b as changed): `-k`
/// already makes no cached credential where `sudo(8)` says so; the second
/// implementation's page does not say. `sudo(8)`, `-K`: *"removes every
/// cached credential for the user, regardless of the terminal or parent
/// process ID"*, and *"This option does not require a password"*.
pub const THE_CACHED_CREDENTIALS_GO: CommandString = CommandString::constant("sudo -K");

impl AnElevatedAct {
    /// The act's command.
    pub fn the_command(&self) -> TheCommand {
        match self {
            AnElevatedAct::AListing { .. } => THE_LISTING,
            AnElevatedAct::Delete { .. } => THE_DELETE,
            AnElevatedAct::NewFolder { .. } => THE_NEW_FOLDER,
            AnElevatedAct::Rename { .. } => THE_RENAME,
        }
    }

    /// The act's name, the paths file's first field. Each script takes its
    /// own and refuses any other.
    pub fn its_name(&self) -> &'static [u8] {
        match self {
            AnElevatedAct::AListing { .. } => b"listing",
            AnElevatedAct::Delete { .. } => b"delete",
            AnElevatedAct::NewFolder { .. } => b"new-folder",
            AnElevatedAct::Rename { .. } => b"rename",
        }
    }

    /// The folder the act is done in.
    pub fn its_folder(&self) -> &[u8] {
        match self {
            AnElevatedAct::AListing { folder }
            | AnElevatedAct::Delete { folder, .. }
            | AnElevatedAct::NewFolder { folder, .. }
            | AnElevatedAct::Rename { folder, .. } => folder,
        }
    }

    /// The names inside the folder, in order.
    pub fn its_names(&self) -> Vec<&[u8]> {
        match self {
            AnElevatedAct::AListing { .. } => Vec::new(),
            AnElevatedAct::Delete { name, .. } | AnElevatedAct::NewFolder { name, .. } => vec![name],
            AnElevatedAct::Rename { from, to, .. } => vec![from, to],
        }
    }

    /// **The whole path the act is about**: the folder for a listing, the
    /// item for the others (a rename's old one).
    pub fn the_path_it_is_about(&self) -> Vec<u8> {
        match self.its_names().first() {
            None => self.its_folder().to_vec(),
            Some(name) => inside(self.its_folder(), name),
        }
    }

    /// **Every command, every form**, for the tests that read them.
    pub fn every_command() -> [TheCommand; 4] {
        [THE_LISTING, THE_DELETE, THE_NEW_FOLDER, THE_RENAME]
    }
}

/// `name` inside `folder`.
fn inside(folder: &[u8], name: &[u8]) -> Vec<u8> {
    let mut path = folder.to_vec();
    if path.last() != Some(&b'/') {
        path.push(b'/');
    }
    path.extend_from_slice(name);
    path
}

/// **P6b rule 3, as changed: why a path or a name is refused on the desk**,
/// before anything is sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyThePathIsRefused {
    /// Empty, or not starting at `/`.
    NotAbsolute,
    /// Holds the one byte no name may hold.
    HoldsANul,
    /// A `.` or `..` component, which would name somewhere the user did not
    /// see.
    HoldsADotComponent,
    /// `/` itself, as the thing acted on.
    IsTheRoot,
    /// A doubled or trailing `/`: not the spelling `pwd -P` writes, so the
    /// folder could never pass the comparison on the machine.
    NotPlainlySpelled,
    /// A name that is empty, holds a `/`, or is `.` or `..`.
    NotAName,
}

/// **P6b rule 3, for one folder**: absolute, no NUL, no `.` or `..`, and
/// spelled plainly — `/` itself, or no doubled or trailing `/`. `/` is a
/// folder an act may be done in; it is never a thing acted on.
pub fn the_folder_is_checked(folder: &[u8]) -> Result<(), WhyThePathIsRefused> {
    if folder.first() != Some(&b'/') {
        return Err(WhyThePathIsRefused::NotAbsolute);
    }
    if folder.contains(&0) {
        return Err(WhyThePathIsRefused::HoldsANul);
    }
    if folder == b"/" {
        return Ok(());
    }
    let components: Vec<&[u8]> = folder[1..].split(|byte| *byte == b'/').collect();
    if components.iter().any(|one| *one == b"." || *one == b"..") {
        return Err(WhyThePathIsRefused::HoldsADotComponent);
    }
    if components.iter().any(|one| one.is_empty()) {
        return Err(WhyThePathIsRefused::NotPlainlySpelled);
    }
    Ok(())
}

/// **One name inside a folder**: never empty, no `/`, no NUL, never `.` or
/// `..`. The script checks the same again under the elevating command.
pub fn the_name_is_checked(name: &[u8]) -> Result<(), WhyThePathIsRefused> {
    if name.contains(&0) {
        return Err(WhyThePathIsRefused::HoldsANul);
    }
    if name.is_empty() || name.contains(&b'/') || name == b"." || name == b".." {
        return Err(WhyThePathIsRefused::NotAName);
    }
    Ok(())
}

/// **A whole path, split into the folder it is in and its name** — for an
/// act about one item. `/` itself is refused: it is in no folder.
pub fn the_path_is_split(path: &[u8]) -> Result<(Vec<u8>, Vec<u8>), WhyThePathIsRefused> {
    the_folder_is_checked(path)?;
    if path == b"/" {
        return Err(WhyThePathIsRefused::IsTheRoot);
    }
    let at = path.iter().rposition(|byte| *byte == b'/').unwrap_or(0);
    let folder = if at == 0 { b"/".to_vec() } else { path[..at].to_vec() };
    Ok((folder, path[at + 1..].to_vec()))
}

/// **What crosses for `act`**: the paths file's bytes — the act's name, its
/// `mark`, the folder, then each name, each ended by NUL — or why the folder
/// or a name is refused.
pub fn what_crosses(act: &AnElevatedAct, mark: &[u8]) -> Result<Vec<u8>, WhyThePathIsRefused> {
    the_folder_is_checked(act.its_folder())?;
    for name in act.its_names() {
        the_name_is_checked(name)?;
    }
    let mut bytes = Vec::new();
    for field in [act.its_name(), mark, act.its_folder()].into_iter().chain(act.its_names()) {
        bytes.extend_from_slice(field);
        bytes.push(0);
    }
    Ok(bytes)
}

/// **What the command's answer says about who ran and whose act it was** —
/// read from this file's own marks and nothing of the machine's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhoAnswered {
    /// **No root line: nothing ran as root.** The elevating command
    /// refused — a password required, a wrong one, or no grant — and its
    /// words are the complaint stream.
    TheElevatingCommandRefused,
    /// **The root line and no act mark**: admin access was had, and the act
    /// did not begin — the paths file not opened, `xargs` missing.
    TheActDidNotBegin,
    /// **A mark line carrying another act's mark**: what ran was not this
    /// act.
    AnotherAct,
    /// **This act's script ran as root**; `output` is what it printed after
    /// its mark line.
    ThisAct { output: Vec<u8> },
}

/// **Reads the command's standard output for the act's mark.** The first
/// line that opens with [`THE_MARK_LINE_OPENS`] is the mark line: nothing the
/// act prints comes before it, so a name in a listing cannot stand in for it.
pub fn who_answered(standard_output: &[u8], mark: &[u8]) -> WhoAnswered {
    let opens = THE_MARK_LINE_OPENS.as_bytes();
    let root = ROOT_WAS_REACHED.as_bytes();
    let Some(after_the_root_line) = standard_output
        .split(|byte| *byte == b'\n')
        .position(|line| line == root)
        .map(|index| {
            standard_output
                .split_inclusive(|byte| *byte == b'\n')
                .take(index + 1)
                .map(<[u8]>::len)
                .sum::<usize>()
        })
    else {
        return WhoAnswered::TheElevatingCommandRefused;
    };
    let standard_output = &standard_output[after_the_root_line..];
    let mut at = 0;
    while at <= standard_output.len() {
        let rest = &standard_output[at..];
        let line_ends = rest.iter().position(|byte| *byte == b'\n');
        let line = &rest[..line_ends.unwrap_or(rest.len())];
        if let Some(carried) = line.strip_prefix(opens) {
            if carried != mark {
                return WhoAnswered::AnotherAct;
            }
            let output = match line_ends {
                Some(end) => rest[end + 1..].to_vec(),
                None => Vec::new(),
            };
            return WhoAnswered::ThisAct { output };
        }
        match line_ends {
            Some(end) => at += end + 1,
            None => break,
        }
    }
    WhoAnswered::TheActDidNotBegin
}

/// **Whether the complaint stream holds one of this file's own marks, as a
/// whole line**, never matched inside one.
pub fn the_complaint_holds(complaint: &[u8], mark: &str) -> bool {
    complaint.split(|byte| *byte == b'\n').any(|line| line == mark.as_bytes())
}

/// The type bits a listing's letter stands for — the same three
/// [`crate::listing::Kind`] reads, and nothing for any other letter.
fn the_mode_for(letter: u8) -> Option<u32> {
    match letter {
        b'd' => Some(0o040000),
        b'f' => Some(0o100000),
        b'l' => Some(0o120000),
        _ => None,
    }
}

/// **`%s`**: decimal digits and nothing else.
fn a_size(field: &[u8]) -> Option<u64> {
    if field.is_empty() || !field.iter().all(u8::is_ascii_digit) {
        return None;
    }
    std::str::from_utf8(field).ok()?.parse().ok()
}

/// **`%T@`**: whole seconds, an optional minus, an optional fraction, which is
/// dropped. A moment outside what the protocol's own field holds is no
/// moment.
fn a_moment(field: &[u8]) -> Option<Option<u32>> {
    let (whole, fraction) = match field.iter().position(|byte| *byte == b'.') {
        Some(at) => (&field[..at], &field[at + 1..]),
        None => (field, &b""[..]),
    };
    let digits = whole.strip_prefix(b"-").unwrap_or(whole);
    if digits.is_empty() || !digits.iter().all(u8::is_ascii_digit) || !fraction.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let seconds: i64 = std::str::from_utf8(whole).ok()?.parse().ok()?;
    Some(u32::try_from(seconds).ok())
}

/// **One entry of an elevated listing, read into the protocol's own entry
/// shape** so the browser draws it as it draws any other (L1's four fields).
/// The mode carries only the type, which is all the browser reads of it.
///
/// **P6's rule, whole**: the output must split into records of exactly the
/// four NUL-ended fields asked for, and each field must read as what was asked
/// for, or the whole reading is unavailable — *"nothing in the text says which
/// entry was split"*.
pub fn read_the_elevated_listing(output: &[u8]) -> Result<Vec<Entry>, WhyTheElevatedListingIsUnavailable> {
    const FIELDS: usize = 4;
    let unavailable = WhyTheElevatedListingIsUnavailable::TheRecordsDoNotAllCarryTheFieldsTheFormatAskedFor;
    if output.is_empty() {
        return Ok(Vec::new());
    }
    // **Every field is ended by NUL**, so a reading not ended by one was cut.
    let Some(body) = output.strip_suffix(&[0]) else {
        return Err(unavailable);
    };
    let fields: Vec<Vec<u8>> = body.split(|byte| *byte == 0).map(<[u8]>::to_vec).collect();
    if fields.len() % FIELDS != 0 {
        return Err(unavailable);
    }
    let records: Vec<Vec<Vec<u8>>> = fields.chunks(FIELDS).map(<[Vec<u8>]>::to_vec).collect();
    let AnElevatedListing::Accepted(records) = accept_an_elevated_listing(records, FIELDS) else {
        return Err(unavailable);
    };
    let mut entries = Vec::with_capacity(records.len());
    for record in records {
        let [name, size, moment, kind] = <[Vec<u8>; FIELDS]>::try_from(record).map_err(|_| unavailable)?;
        if name.is_empty() || name.contains(&b'/') {
            return Err(unavailable);
        }
        let size = a_size(&size).ok_or(unavailable)?;
        let moment = a_moment(&moment).ok_or(unavailable)?;
        let [letter] = kind.as_slice() else {
            return Err(unavailable);
        };
        if !letter.is_ascii_alphabetic() {
            return Err(unavailable);
        }
        entries.push(Entry {
            name,
            attributes: Attributes {
                size: Some(size),
                permissions: the_mode_for(*letter),
                access_time: None,
                modification_time: moment,
            },
        });
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The names the brief asks for, in one: quotes, spaces, a semicolon, a
    /// newline and a leading dash.
    const A_HOSTILE_NAME: &[u8] = b"-rf ; touch PWNED 'it' \"is\"\nhere $(id) `id`";

    #[test]
    fn no_command_holds_anything_but_its_own_constant_text() {
        for command in AnElevatedAct::every_command() {
            for form in [command.plain, command.passwordless, command.with_a_password] {
                let text = form.as_str();
                // The only path a command names is its own token.
                assert!(text.ends_with(THE_PATHS_FILE.named_in_a_command_as), "{text}");
                assert!(!text.contains("PWNED"));
            }
        }
    }

    /// **Sol's findings 5 and 6: every elevated form ignores a cached
    /// credential and makes none** — `sudo -k`, before `-n` or `-S`.
    #[test]
    fn each_elevated_form_is_its_plain_form_behind_sudo_k() {
        for command in AnElevatedAct::every_command() {
            assert_eq!(command.passwordless.as_str(), format!("sudo -k -n {}", command.plain.as_str()));
            assert_eq!(command.with_a_password.as_str(), format!("sudo -k -S {}", command.plain.as_str()));
        }
    }

    /// **Sol's finding 3: every script opens its folder and compares it before
    /// it acts, and acts only on names relative to it.**
    #[test]
    fn every_script_opens_its_folder_and_acts_by_relative_names() {
        for command in AnElevatedAct::every_command() {
            let text = command.plain.as_str();
            let opened = text.find("cd -P \"$3\"").expect("the folder opened");
            let compared = text.find("[ \"${here%?x}\" = \"$3\" ]").expect("and compared");
            assert!(opened < compared, "{text}");
            let acts = ["exec find . ", "exec rm ", "mkdir ", "exec mv "]
                .iter()
                .filter_map(|act| text.find(act))
                .min()
                .expect("an act");
            assert!(compared < acts, "the act comes after the comparison: {text}");
            assert!(!text.contains("-- \"$3"), "never a path the folder is part of: {text}");
        }
        assert!(THE_RENAME.plain.as_str().contains("mv -T -- \"./$4\" \"./$5\""));
        assert!(THE_NEW_FOLDER.plain.as_str().contains("chown \"$SUDO_UID:$SUDO_GID\" ."));
    }

    #[test]
    fn the_token_is_class_two_and_sits_in_the_carriage_directory() {
        assert!(THE_PATHS_FILE.is_drawn_from_class_twos_alphabet());
        assert!(THE_PATHS_FILE.named_in_a_command_as.ends_with(THE_PATHS_FILE.link_path_on_the_subsystem));
        assert!(THE_PATHS_FILE
            .link_path_on_the_subsystem
            .starts_with(smoozy_server_protocol::carriage::THE_DIRECTORY_ON_THE_SUBSYSTEM));
    }

    /// **Sol's finding 1: the act's name and its own mark lead what crosses**,
    /// and a hostile name crosses whole and NUL-ended.
    #[test]
    fn a_hostile_name_crosses_whole_after_the_acts_name_and_mark() {
        let act = AnElevatedAct::Rename { folder: b"/data".to_vec(), from: A_HOSTILE_NAME.to_vec(), to: b"kept".to_vec() };
        let mut wanted = b"rename\0mark-1\0/data\0".to_vec();
        wanted.extend_from_slice(A_HOSTILE_NAME);
        wanted.extend_from_slice(b"\0kept\0");
        assert_eq!(what_crosses(&act, b"mark-1"), Ok(wanted));
        let listing = AnElevatedAct::AListing { folder: b"/".to_vec() };
        assert_eq!(what_crosses(&listing, b"m"), Ok(b"listing\0m\0/\0".to_vec()));
    }

    #[test]
    fn a_path_that_could_name_somewhere_else_is_refused_before_anything_crosses() {
        assert_eq!(the_folder_is_checked(b"data/x"), Err(WhyThePathIsRefused::NotAbsolute));
        assert_eq!(the_folder_is_checked(b""), Err(WhyThePathIsRefused::NotAbsolute));
        assert_eq!(the_folder_is_checked(b"/"), Ok(()));
        assert_eq!(the_folder_is_checked(b"///"), Err(WhyThePathIsRefused::NotPlainlySpelled));
        assert_eq!(the_folder_is_checked(b"/data/"), Err(WhyThePathIsRefused::NotPlainlySpelled));
        assert_eq!(the_folder_is_checked(b"/data/../etc"), Err(WhyThePathIsRefused::HoldsADotComponent));
        assert_eq!(the_folder_is_checked(b"/data/./x"), Err(WhyThePathIsRefused::HoldsADotComponent));
        assert_eq!(the_folder_is_checked(b"/data/a\0b"), Err(WhyThePathIsRefused::HoldsANul));
        assert_eq!(the_folder_is_checked(b"/data/..hidden"), Ok(()));
        for name in [&b""[..], b".", b"..", b"a/b"] {
            assert_eq!(the_name_is_checked(name), Err(WhyThePathIsRefused::NotAName), "{name:?}");
        }
        assert_eq!(the_name_is_checked(b"-rf"), Ok(()));
        assert_eq!(the_path_is_split(b"/"), Err(WhyThePathIsRefused::IsTheRoot));
        assert_eq!(the_path_is_split(b"/etc"), Ok((b"/".to_vec(), b"etc".to_vec())));
        assert_eq!(the_path_is_split(b"/data/a b"), Ok((b"/data".to_vec(), b"a b".to_vec())));
        let into_dots = AnElevatedAct::Rename { folder: b"/a".to_vec(), from: b"x".to_vec(), to: b"..".to_vec() };
        assert_eq!(what_crosses(&into_dots, b"m"), Err(WhyThePathIsRefused::NotAName));
    }

    /// **Sol's findings 1, 5 and 9: the two marks say who answered.** No
    /// root line is the elevating command's refusal; the root line with no
    /// act mark is a failure before the act began; another act's mark is
    /// another act's; a listing's names after the mark never count as one.
    #[test]
    fn the_marks_say_who_answered() {
        assert_eq!(who_answered(b"", b"m-1"), WhoAnswered::TheElevatingCommandRefused);
        assert_eq!(who_answered(b"smoozy-elevated m-1\n", b"m-1"), WhoAnswered::TheElevatingCommandRefused);
        assert_eq!(who_answered(b"smoozy-elevated: admin access\n", b"m-1"), WhoAnswered::TheActDidNotBegin);
        assert_eq!(
            who_answered(b"smoozy-elevated: admin access\nsmoozy-elevated m-2\n", b"m-1"),
            WhoAnswered::AnotherAct
        );
        assert_eq!(
            who_answered(b"smoozy-elevated: admin access\nsmoozy-elevated m-1\n", b"m-1"),
            WhoAnswered::ThisAct { output: Vec::new() }
        );
        assert_eq!(
            who_answered(b"a login's own greeting\nsmoozy-elevated: admin access\nsmoozy-elevated m-1\nname\0", b"m-1"),
            WhoAnswered::ThisAct { output: b"name\0".to_vec() }
        );
        assert_eq!(
            who_answered(b"smoozy-elevated: admin access\nsmoozy-elevated m-1\nsmoozy-elevated m-2\n", b"m-1"),
            WhoAnswered::ThisAct { output: b"smoozy-elevated m-2\n".to_vec() }
        );
        assert!(the_complaint_holds(b"x\nsmoozy-elevated: the path goes through a link\n", THE_PATH_GOES_THROUGH_A_LINK));
        assert!(!the_complaint_holds(b"rm: smoozy-elevated: the path goes through a link\n", THE_PATH_GOES_THROUGH_A_LINK));
    }

    /// **Sol's second look: Files' probe ignores a cached credential, and the
    /// root line comes before the paths file is opened.**
    #[test]
    fn the_probe_is_behind_sudo_k_and_the_root_line_comes_first() {
        assert_eq!(THE_PROBE_IGNORING_A_CACHED_CREDENTIAL.as_str(), "sudo -k -n true");
        for command in AnElevatedAct::every_command() {
            let text = command.plain.as_str();
            let root = text.find("printf \"smoozy-elevated: admin access").expect("the root line");
            assert!(root < text.find("exec xargs").expect("then the paths file"), "{text}");
        }
        let made = THE_NEW_FOLDER.plain.as_str();
        assert!(made.find("[ -O . ]").expect("root's") < made.find("chown").expect("the hand-over"));
    }

    #[test]
    fn a_listing_with_a_hostile_name_reads_whole() {
        let mut output = Vec::new();
        output.extend_from_slice(A_HOSTILE_NAME);
        output.extend_from_slice(b"\x0012\x001731400000.5\x00f\x00");
        output.extend_from_slice(b"textures\x004096\x001731400001.0000000000\x00d\x00");
        output.extend_from_slice(b"outside\x007\x001731400002\x00l\x00");
        let entries = read_the_elevated_listing(&output).expect("whole records");
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].name, A_HOSTILE_NAME);
        assert_eq!(entries[0].attributes.size, Some(12));
        assert_eq!(entries[0].attributes.modification_time, Some(1_731_400_000));
        let kind = |entry: &Entry| crate::listing::Kind::from_mode(entry.attributes.permissions);
        assert_eq!(kind(&entries[0]), crate::listing::Kind::File);
        assert_eq!(kind(&entries[1]), crate::listing::Kind::Folder);
        assert_eq!(kind(&entries[2]), crate::listing::Kind::Link);
        assert_eq!(read_the_elevated_listing(b""), Ok(Vec::new()));
    }

    #[test]
    fn a_listing_that_does_not_split_into_whole_records_is_never_shown() {
        let unavailable = Err(WhyTheElevatedListingIsUnavailable::TheRecordsDoNotAllCarryTheFieldsTheFormatAskedFor);
        // A field short.
        assert_eq!(read_the_elevated_listing(b"a\x001\x002\x00"), unavailable);
        // Not ended by NUL: the last record was cut.
        assert_eq!(read_the_elevated_listing(b"a\x001\x002\x00f"), unavailable);
        // A size that is not a number, a type that is not one letter, a name
        // with a slash, an empty name.
        assert_eq!(read_the_elevated_listing(b"a\x00x\x002\x00f\x00"), unavailable);
        assert_eq!(read_the_elevated_listing(b"a\x001\x002\x00ff\x00"), unavailable);
        assert_eq!(read_the_elevated_listing(b"a/b\x001\x002\x00f\x00"), unavailable);
        assert_eq!(read_the_elevated_listing(b"\x001\x002\x00f\x00"), unavailable);
        // A rendered line from some other program.
        assert_eq!(read_the_elevated_listing(b"-rw-r--r-- 1 root root 12 Nov 12 a\n"), unavailable);
    }

    #[test]
    fn the_landing_mark_is_read_and_nothing_else_is() {
        assert!(the_complaint_holds(b"smoozy-elevated: the landing path is taken\n", THE_LANDING_IS_TAKEN));
        assert!(!the_complaint_holds(b"mv: cannot move: File exists\n", THE_LANDING_IS_TAKEN));
        assert!(!the_complaint_holds(b"", THE_LANDING_IS_TAKEN));
    }
}

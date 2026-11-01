//! **The one thing this package's shape exists to protect, held by a test
//! rather than by a note in a manifest.**
//!
//! `smoozy-server` is the artifact the desktop uploads to every rented machine
//! over a metered link, and the signed release manifest carries its byte length
//! and its SHA-256 digest — `Server protocol and transfers` B8 and B10. **The
//! machine is the far end of an SSH connection and never opens one**, so the
//! desktop's SSH client and that client's C cryptography are weight with no work
//! to do here. Step 8a's second piece took them out, 142 dependency crates down
//! to 19, and that is also what lets this program be checked against its real
//! Linux target at this desk: the C library's build script wants a cross
//! toolchain the desk has not got.
//!
//! **Step 8b then moved this program out of the library's package so that it
//! could reach the machine-side work**, and a package that may depend on
//! anything is exactly the package where that weight can come back by accident.
//! **So it is asserted rather than remembered**: the set of workspace crates
//! reachable from here is written out, and adding one fails here before it is
//! discovered in an artifact's byte length.
//!
//! **It reads the manifests rather than a lock file or a built tree**, so it
//! answers at the desk with nothing fetched and nothing built, and it fails on
//! the edge that was added rather than on its consequences.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

/// **Every workspace crate this program is allowed to reach, written out.**
///
/// Adding a name here is a deliberate act with a cost, and the cost is in the
/// module note above. **`smoozy-server-protocol` may never be one of them**, and
/// neither may anything that reaches it — which is every feature crate taken
/// whole, and which is why the machine halves of those crates are separated as
/// each piece of work needs one rather than depended on whole.
///
/// **The seven machine halves were added at step 10c piece 3**, when this
/// program took `Queue & History — machine` and `Rendering — machine` and,
/// beneath them, the other five. Each opens no connection and none reaches
/// `smoozy-server-protocol`; the program's tree on its Linux target went from
/// 19 dependency crates to 26, and every one of the seven is a workspace crate.
///
/// **No workspace crate was added at step 10c piece 8 and two outside ones
/// were**, which is the other half of what this file watches and is written
/// here rather than left to a tree count. **The workspace's one HTTPS client**,
/// taken on the build runner's ruling of 2026-09-19 because `Server Setup` B4
/// puts the Blender fetch on the machine's own line; and **the workspace's one
/// digest function**, which `Server Setup` B4's published checksum is the whole
/// point of. Together they took the tree from 26 dependency crates to 54. **The
/// client's cryptography was `rustls` with `rustls-graviola`, which builds no C
/// library**, so the Linux cross-check at this desk still passed — which is the
/// condition the ruling itself set and half of what step 8a's split bought.
/// **On 2026-10-24 it became `rustls`'s own `ring` provider** (the Creator's
/// eighteenth notes, note 3): `rustls-graviola` needed six instruction sets and
/// stopped setup on servers whose processors lack `avx2`, `bmi1` and `adx`.
/// `ring` carries C and assembly, and `App/.cargo/config.toml` compiles them for
/// this program's Linux target at this desk with the Mac's own compiler, so the
/// Linux check still passes — against `x86_64-unknown-linux-musl`, the target
/// the program is built for.
///
/// **`smoozy-telegram` was added on 2026-10-16**, the second piece of the
/// Telegram worker's map, when this program's `work telegram` mode became the
/// bot itself. The map's section 2.6 measured it first: every crate beneath it
/// was already in this tree, so it took the tree from 55 packages to 56 — itself
/// and nothing else. **It reaches `smoozy-server-protocol` nowhere**, and its
/// own `tests/the_crates_shape_holds.rs` forbids it, with the three other
/// feature crates that would.
const WHAT_THIS_PROGRAM_MAY_REACH: &[&str] = &[
    "smoozy-dependencies-machine",
    "smoozy-framing",
    "smoozy-job-files-machine",
    "smoozy-queue-history-machine",
    "smoozy-render-settings-machine",
    "smoozy-rendering-machine",
    "smoozy-reporting",
    "smoozy-runtime",
    "smoozy-security-and-licence",
    "smoozy-server-setup-machine",
    "smoozy-servers-values",
    "smoozy-state",
    "smoozy-telegram",
];

/// The crate whose arrival here undoes step 8a's split.
const THE_CONNECTING_CRATE: &str = "smoozy-server-protocol";

/// **The crates from outside this workspace that must never be declared
/// anywhere in this program's tree, by name.**
///
/// **The remote-connection library is first because it is the one the split was
/// made against.** `Servers` K1 drives SSH inside Smoozy's own process, and that
/// transport is the desktop's; a machine that is the far end of a connection and
/// never opens one has no use for it. The two after it are C cryptography
/// libraries whose C cannot be compiled for this program's Linux target at this
/// desk (`aws-lc-sys` wants the Linux C library's own headers, measured
/// 2026-10-24), and that check is half of what step 8a's split bought. **The
/// last two are the provider with a processor floor**, `rustls-graviola` and
/// the `graviola` beneath it, which stopped setup on servers with older
/// processors and must not come back.
///
/// **`ring` was on this list until 2026-10-24** and is now the provider itself:
/// it runs on any 64-bit Intel or AMD processor, and its C is compiled for the
/// Linux target at this desk by `App/.cargo/config.toml`.
///
/// **What this catches and what it does not, said rather than implied.** It reads
/// the manifests of this workspace's own members, so it catches one of these
/// being **declared** by this program or by a crate it reaches. **It cannot see a
/// crate arriving beneath an outside dependency**, because an outside crate's own
/// manifest is not in this workspace and this test fetches nothing. What guards
/// that is the list above: an outside dependency arrives here only under a
/// workspace crate that is written down as reachable.
const WHAT_MUST_NEVER_BE_IN_THIS_TREE: &[&str] =
    &["russh", "aws-lc-sys", "openssl-sys", "rustls-graviola", "graviola"];

#[test]
fn this_program_reaches_no_workspace_crate_it_is_not_written_down_as_reaching() {
    let reached = what_this_program_reaches();
    let allowed: BTreeSet<String> = WHAT_THIS_PROGRAM_MAY_REACH
        .iter()
        .map(|name| (*name).to_string())
        .collect();
    assert_eq!(
        reached, allowed,
        "the workspace crates this program reaches are not the ones written down. \
         Adding one is a deliberate act and its cost is in this file's own note."
    );
}

#[test]
fn the_ssh_client_is_not_in_this_programs_tree() {
    let reached = what_this_program_reaches();
    assert!(
        !reached.contains(THE_CONNECTING_CRATE),
        "`{THE_CONNECTING_CRATE}` is back in the machine program's tree, and with it the \
         desktop's SSH client and that client's C cryptography. That is weight in an \
         artifact whose byte length and digest a signed release manifest carries, and it \
         is what stopped a Linux check being taken at this desk before step 8a split it out."
    );
}

#[test]
fn nothing_in_this_programs_tree_declares_a_crate_this_tree_may_never_hold() {
    let workspace = what_this_workspace_declares();
    let mut reached = what_this_program_reaches();
    reached.insert("smoozy-server".to_string());
    for crate_name in &reached {
        let declares = workspace
            .get(crate_name)
            .expect("a reached crate has a manifest");
        for forbidden in WHAT_MUST_NEVER_BE_IN_THIS_TREE {
            assert!(
                !declares.every_dependency.contains(*forbidden),
                "`{crate_name}` is in the machine program's tree and declares `{forbidden}`. \
                 That is the desktop's remote-connection library, a C cryptography library \
                 this desk cannot build for the program's Linux target, or the provider whose \
                 processor floor stopped setup on older servers (the eighteenth notes, note 3)."
            );
        }
    }
}

/// **No crate anywhere in the workspace carries `graviola`** — not this
/// program, not the desktop, and not beneath an outside crate. Added
/// 2026-10-24, the Creator's eighteenth notes, note 3.
///
/// **Why the lock file and not the manifests**, when the tests above read the
/// manifests on purpose. The tests above guard an edge a workspace crate adds;
/// this guards a crate arriving by any route at all, which only the lock file
/// sees: it lists every package the workspace resolves, for every system. That
/// provider asked for `aes`, `pclmulqdq`, `bmi1`, `adx`, `avx` and `avx2` and
/// stopped the program without them, so setup could not download Blender on a
/// rented server with an older processor. **Smoozy runs on any 64-bit Intel or
/// AMD processor**, on the server and on the desktop alike, and a crate that
/// brings that floor back fails here.
#[test]
fn nothing_in_the_workspace_carries_graviola() {
    let lock = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("this package sits inside App/")
        .join("Cargo.lock");
    let text = std::fs::read_to_string(&lock).expect("the workspace's lock file is readable");
    let packages: Vec<String> = text
        .lines()
        .filter_map(|line| the_text_value_of("name", line.trim()))
        .collect();
    assert!(
        packages.iter().any(|name| name == "rustls"),
        "the lock file's packages were not read, so the check below would see nothing"
    );
    for name in &packages {
        assert!(
            !name.contains("graviola"),
            "`{name}` is in the workspace's lock file. That provider needs `avx2`, `bmi1`, `adx` \
             and three more instruction sets and stops the program without them; Smoozy runs on \
             any 64-bit Intel or AMD processor."
        );
    }
}

/// Every workspace crate reachable from this package by ordinary dependencies,
/// read from the manifests.
///
/// **Development dependencies are not followed**, because they reach no built
/// artifact — the Cargo Book's own sentence, at *Specifying Dependencies*:
/// *"Dev-dependencies are not used when compiling a package for building, but
/// are used for compiling tests, examples, and benchmarks."*
fn what_this_program_reaches() -> BTreeSet<String> {
    let workspace = what_this_workspace_declares();
    assert!(
        workspace.contains_key("smoozy-server"),
        "this package was not found by its own name"
    );

    let mut reached = BTreeSet::new();
    let mut to_walk = workspace
        .get("smoozy-server")
        .expect("this package's own manifest was read")
        .the_workspace_crates
        .clone();
    while let Some(next) = to_walk.pop() {
        if !reached.insert(next.clone()) {
            continue;
        }
        if let Some(declares) = workspace.get(&next) {
            to_walk.extend(declares.the_workspace_crates.clone());
        }
    }
    reached
}

/// What one member manifest declares.
struct WhatAManifestDeclares {
    /// The names it declares by a path inside this workspace.
    the_workspace_crates: Vec<String>,
    /// **Every dependency name it declares, path or not.** This is what the
    /// forbidden-crate check reads, because a crate from outside arrives by
    /// `workspace = true` or by a version and never by a path.
    every_dependency: BTreeSet<String>,
}

/// Every member manifest of `App/`, read once, by package name.
fn what_this_workspace_declares() -> HashMap<String, WhatAManifestDeclares> {
    let app = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("this package sits inside App/")
        .to_path_buf();
    let manifests = every_manifest_under(&app);
    assert!(
        manifests.len() > 15,
        "the workspace manifests were not read, so nothing below would be walked"
    );

    let mut out = HashMap::new();
    for path in manifests {
        let text = std::fs::read_to_string(&path).expect("the manifest is readable");
        out.insert(the_package_name(&text), what_one_manifest_declares(&text));
    }
    out
}

/// Every member manifest of `App/`, one folder deep.
fn every_manifest_under(app: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(app).expect("App/ is readable") {
        let path = entry.expect("the entry is readable").path();
        let manifest = path.join("Cargo.toml");
        if manifest.is_file() {
            out.push(manifest);
        }
    }
    out.sort();
    out
}

/// **The package's name, from the section it belongs to rather than from the
/// first match in the file.**
///
/// A manifest here can carry `[[bin]]`, `[lib]` and `[workspace.dependencies.x]`
/// sections, each of which may hold a `name` key of its own; reading the first
/// one in the file answers correctly only for as long as `[package]` happens to
/// come first.
fn the_package_name(text: &str) -> String {
    let mut in_the_package_section = false;
    for line in text.lines() {
        let line = line.trim();
        if let Some(inner) = a_section_heading(line) {
            in_the_package_section = inner == "package";
            continue;
        }
        if !in_the_package_section || line.starts_with('#') {
            continue;
        }
        if let Some(name) = the_text_value_of("name", line) {
            return name;
        }
    }
    panic!("a manifest names no package");
}

/// What one manifest declares, read section by section.
///
/// **Every section whose name ends in *dependencies* is read except the two
/// development ones** — `[dev-dependencies]` and a target's own
/// `[target.'cfg(...)'.dev-dependencies]` — because those reach no built
/// artifact: the Cargo Book's own sentence, at *Specifying Dependencies*, is
/// *"Dev-dependencies are not used when compiling a package for building, but
/// are used for compiling tests, examples, and benchmarks."* **Everything else
/// is read**, including build dependencies and every target-specific section,
/// because a dependency added for one operating system is still a dependency of
/// this package and this workspace already writes several that way.
///
/// **The table form is read too** — `[dependencies.name]`, with the entry's own
/// keys below it — because that is how this workspace writes any dependency
/// whose feature list is long enough to want a comment per switch, and reading
/// only the inline form made those invisible.
fn what_one_manifest_declares(text: &str) -> WhatAManifestDeclares {
    let mut the_workspace_crates = Vec::new();
    let mut every_dependency = BTreeSet::new();
    // Where the current heading is a dependency table of its own, the name it
    // declares and whether a `path` key has been seen under it yet.
    let mut the_table_form: Option<(String, bool)> = None;
    let mut in_a_dependency_section = false;

    let close_the_table = |table: Option<(String, bool)>, out: &mut Vec<String>| {
        if let Some((name, by_a_path)) = table {
            if by_a_path {
                out.push(name);
            }
        }
    };

    for line in text.lines() {
        let line = line.trim();
        if let Some(inner) = a_section_heading(line) {
            close_the_table(the_table_form.take(), &mut the_workspace_crates);
            match what_dependency_section_this_is(&inner) {
                None => in_a_dependency_section = false,
                Some(WhichDependencySection::AListOfEntries) => in_a_dependency_section = true,
                Some(WhichDependencySection::OneEntrysOwnTable(name)) => {
                    in_a_dependency_section = true;
                    every_dependency.insert(name.clone());
                    the_table_form = Some((name, false));
                }
            }
            continue;
        }
        if !in_a_dependency_section || line.starts_with('#') {
            continue;
        }
        if let Some((name, by_a_path)) = &mut the_table_form {
            let _ = name;
            if it_carries_a_path_key(line) {
                *by_a_path = true;
            }
            continue;
        }
        let Some((name, rest)) = line.split_once('=') else {
            continue;
        };
        let name = name.trim();
        if name.is_empty() || name.contains(char::is_whitespace) {
            continue;
        }
        every_dependency.insert(name.to_string());
        if it_carries_a_path_key(rest) {
            the_workspace_crates.push(name.to_string());
        }
    }
    close_the_table(the_table_form.take(), &mut the_workspace_crates);

    WhatAManifestDeclares {
        the_workspace_crates,
        every_dependency,
    }
}

/// A section heading's inside, or nothing. `[[bin]]` gives `[bin]`, which is not
/// a dependency section and needs no further care.
fn a_section_heading(line: &str) -> Option<String> {
    let inner = line.strip_prefix('[')?.strip_suffix(']')?;
    Some(inner.to_string())
}

enum WhichDependencySection {
    /// `[dependencies]`, `[build-dependencies]`, `[target.'…'.dependencies]`.
    AListOfEntries,
    /// `[dependencies.name]` and its target-specific form.
    OneEntrysOwnTable(String),
}

/// Which kind of dependency section a heading names, if any.
fn what_dependency_section_this_is(inner: &str) -> Option<WhichDependencySection> {
    let mut parts = the_dotted_parts_of(inner);
    // A target-specific section carries the word and the configuration
    // expression before the kind: `target.'cfg(unix)'.dependencies`.
    if parts.first().map(String::as_str) == Some("target") {
        if parts.len() < 3 {
            return None;
        }
        parts.drain(..2);
    }
    let kind = parts.first()?.clone();
    if !kind.ends_with("dependencies") || kind == "dev-dependencies" {
        return None;
    }
    match parts.len() {
        1 => Some(WhichDependencySection::AListOfEntries),
        2 => Some(WhichDependencySection::OneEntrysOwnTable(parts[1].clone())),
        _ => None,
    }
}

/// A heading's dotted parts, with quoted segments kept whole — a configuration
/// expression may hold a dot and must not be split on one.
fn the_dotted_parts_of(inner: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut held = String::new();
    let mut quote: Option<char> = None;
    for character in inner.chars() {
        match quote {
            Some(open) => {
                if character == open {
                    quote = None;
                } else {
                    held.push(character);
                }
            }
            None => match character {
                '\'' | '"' => quote = Some(character),
                '.' => out.push(std::mem::take(&mut held)),
                _ => held.push(character),
            },
        }
    }
    out.push(held);
    out.into_iter().map(|part| part.trim().to_string()).collect()
}

/// **Whether a line carries a `path` key, however it is spaced.**
///
/// `path = "…"`, `path="…"` and `path  =  "…"` are the same declaration, and the
/// key is matched as a whole word so that a longer key ending in *path* is not
/// mistaken for one.
fn it_carries_a_path_key(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut from = 0;
    while let Some(at) = text[from..].find("path") {
        let start = from + at;
        let end = start + "path".len();
        let before_is_part_of_a_longer_word = start > 0
            && (bytes[start - 1].is_ascii_alphanumeric()
                || bytes[start - 1] == b'_'
                || bytes[start - 1] == b'-');
        let after = text[end..].trim_start();
        if !before_is_part_of_a_longer_word && after.starts_with('=') {
            return true;
        }
        from = end;
    }
    false
}

/// The text value of a key on one line, however it is spaced.
fn the_text_value_of(key: &str, line: &str) -> Option<String> {
    let (name, rest) = line.split_once('=')?;
    if name.trim() != key {
        return None;
    }
    let rest = rest.trim();
    rest.strip_prefix('"')?.split('"').next().map(str::to_string)
}

/// **The shapes this reading missed before 2026-09-12, posed one at a time.**
///
/// The reading looked at `[dependencies]` alone, so a dependency added for one
/// operating system or written as its own table was invisible — and **both
/// shapes are already in use elsewhere in this workspace**, so the guard above
/// could have gone on passing over an edge that was really there. Each case
/// below is one of the shapes it now reads, written out rather than trusted.
#[test]
fn the_reading_sees_the_shapes_this_workspace_actually_writes() {
    let manifest = r#"
# A comment naming a path = "not a dependency" and mentioning russh.
[[bin]]
name = "the-binary-not-the-package"

[package]
name = "the-package"

[dependencies]
plain = { path = "../Plain" }
tight={path="../Tight"}
spaced   =   { path   =   "../Spaced" }
from-outside = { workspace = true }

[build-dependencies]
at-build-time = { path = "../At build time" }

[dev-dependencies]
only-for-tests = { path = "../Only for tests" }

[target.'cfg(unix)'.dependencies]
on-one-system = { path = "../On one system" }

[target.'cfg(target_os = "windows")'.dev-dependencies]
only-for-windows-tests = { path = "../Only for Windows tests" }

[dependencies.its-own-table]
version = "=1.2.3"
path = "../Its own table"

[target.'cfg(unix)'.dependencies.a-targets-own-table]
path = "../A target's own table"

[workspace.dependencies.not-a-dependency-of-this-package]
version = "=1.0.0"
"#;

    assert_eq!(
        the_package_name(manifest),
        "the-package",
        "the name comes from the package section and not from the first `name` in the file"
    );

    let declares = what_one_manifest_declares(manifest);
    let mut by_a_path = declares.the_workspace_crates.clone();
    by_a_path.sort();
    assert_eq!(
        by_a_path,
        vec![
            "a-targets-own-table",
            "at-build-time",
            "its-own-table",
            "on-one-system",
            "plain",
            "spaced",
            "tight",
        ],
        "every path dependency outside the two development sections is followed, and neither \
         of those two is"
    );
    assert!(
        declares.every_dependency.contains("from-outside"),
        "a crate from outside arrives by name and not by a path, and the forbidden-crate check \
         reads it"
    );
    assert!(
        !declares.every_dependency.contains("only-for-tests")
            && !declares.every_dependency.contains("only-for-windows-tests"),
        "nothing under either development section is read"
    );
    assert!(
        !declares
            .every_dependency
            .contains("not-a-dependency-of-this-package"),
        "a workspace-wide declaration is not this package's dependency"
    );
}

/// **A key that merely ends in *path* is not a path key**, which is the one way
/// a looser match would have written an edge nobody declared.
#[test]
fn a_longer_key_ending_in_path_is_not_a_path_entry() {
    assert!(it_carries_a_path_key("path = \"../Somewhere\""));
    assert!(it_carries_a_path_key("path=\"../Somewhere\""));
    assert!(it_carries_a_path_key("{ path   =   \"../Somewhere\" }"));
    assert!(!it_carries_a_path_key("search-path = \"../Somewhere\""));
    assert!(!it_carries_a_path_key("{ features = [\"path\"] }"));
}

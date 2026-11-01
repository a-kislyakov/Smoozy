//! **The one thing this crate's shape exists to protect, held by a test rather
//! than by a note in a manifest.** Copied in shape from
//! `App/Rented-server runtime — program/tests/the_trees_shape_holds.rs`, on the
//! ruling at `Audits/Build/Step 10/Telegram — what each piece owns and in what
//! order.md` section 4, item 1. **Nobody weakens it.**
//!
//! **Why.** The machine program may one day take this crate, so that the role
//! worker can run on a rented machine. It can only do that while this crate
//! stays out of the desktop's SSH client's tree. `smoozy-server-protocol`,
//! `smoozy-queue-history`, `smoozy-job-files` and `smoozy-rendering` all reach
//! that client, so **this crate depends on none of them**, and their records
//! are handed in as values through traits.
//!
//! **It reads the manifests rather than a lock file or a built tree**, so it
//! answers with nothing fetched and nothing built, and it fails on the edge
//! that was added rather than on its consequences.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

const THIS_CRATE: &str = "smoozy-telegram";

/// **Every workspace crate this crate is allowed to reach, written out.**
/// Adding one is a deliberate act; the four below may never be added.
const WHAT_THIS_CRATE_MAY_REACH: &[&str] = &["smoozy-security-and-licence", "smoozy-state"];

/// **The four crates that reach the SSH client.**
const WHAT_THIS_CRATE_MAY_NEVER_REACH: &[&str] = &[
    "smoozy-server-protocol",
    "smoozy-queue-history",
    "smoozy-job-files",
    "smoozy-rendering",
];

/// **Crates from outside the workspace that must never be declared anywhere in
/// this crate's tree**: the SSH client, the two C cryptography libraries the
/// Linux build at this desk cannot compile, and the provider with a processor
/// floor. The same list, and the same limit — it sees a declaration by a
/// workspace crate, not a crate arriving beneath an outside one — as the
/// machine program's guard.
///
/// **Changed 2026-10-24, the Creator's eighteenth notes, note 3.** `ring` left
/// the list: it is now the cryptography beneath the HTTPS client, arriving
/// through `rustls`'s own switch, because it runs on any 64-bit Intel or AMD
/// processor, and its C is compiled for the server program's Linux target at
/// this desk by `App/.cargo/config.toml`. `rustls-graviola` and `graviola`
/// joined it: that provider stopped setup on servers whose processors lack
/// `avx2`, `bmi1` and `adx`.
const WHAT_MUST_NEVER_BE_IN_THIS_TREE: &[&str] =
    &["russh", "aws-lc-sys", "openssl-sys", "rustls-graviola", "graviola"];

#[test]
fn this_crate_reaches_none_of_the_four_crates_that_reach_the_ssh_client() {
    let reached = what_this_crate_reaches();
    for forbidden in WHAT_THIS_CRATE_MAY_NEVER_REACH {
        assert!(
            !reached.contains(*forbidden),
            "`{forbidden}` is in `{THIS_CRATE}`'s tree, and with it the desktop's SSH client. \
             The machine program could then never take this crate. Hand that crate's records \
             in as values through a trait instead."
        );
    }
}

#[test]
fn this_crate_reaches_no_workspace_crate_it_is_not_written_down_as_reaching() {
    let allowed: BTreeSet<String> = WHAT_THIS_CRATE_MAY_REACH
        .iter()
        .map(|name| (*name).to_string())
        .collect();
    assert_eq!(
        what_this_crate_reaches(),
        allowed,
        "the workspace crates `{THIS_CRATE}` reaches are not the ones written down"
    );
}

#[test]
fn nothing_in_this_crates_tree_declares_a_crate_this_tree_may_never_hold() {
    let workspace = what_this_workspace_declares();
    let mut reached = what_this_crate_reaches();
    reached.insert(THIS_CRATE.to_string());
    for crate_name in &reached {
        let declares = workspace
            .get(crate_name)
            .expect("a reached crate has a manifest");
        for forbidden in WHAT_MUST_NEVER_BE_IN_THIS_TREE {
            assert!(
                !declares.every_dependency.contains(*forbidden),
                "`{crate_name}` is in `{THIS_CRATE}`'s tree and declares `{forbidden}`"
            );
        }
    }
}

/// Every workspace crate reachable from this crate by ordinary dependencies.
/// Development dependencies are not followed: they reach no built artifact.
fn what_this_crate_reaches() -> BTreeSet<String> {
    let workspace = what_this_workspace_declares();
    let mut reached = BTreeSet::new();
    let mut to_walk = workspace
        .get(THIS_CRATE)
        .expect("this crate's own manifest was read")
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

struct WhatAManifestDeclares {
    the_workspace_crates: Vec<String>,
    every_dependency: BTreeSet<String>,
}

fn what_this_workspace_declares() -> HashMap<String, WhatAManifestDeclares> {
    let app = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("this crate sits inside App/")
        .to_path_buf();
    let manifests = every_manifest_under(&app);
    assert!(
        manifests.len() > 15,
        "the workspace manifests were not read, so nothing would be walked"
    );
    let mut out = HashMap::new();
    for path in manifests {
        let text = std::fs::read_to_string(&path).expect("the manifest is readable");
        out.insert(the_package_name(&text), what_one_manifest_declares(&text));
    }
    out
}

fn every_manifest_under(app: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(app).expect("App/ is readable") {
        let manifest = entry
            .expect("the entry is readable")
            .path()
            .join("Cargo.toml");
        if manifest.is_file() {
            out.push(manifest);
        }
    }
    out.sort();
    out
}

/// The package's name, from the `[package]` section rather than the first
/// `name` in the file.
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
        if let Some((key, rest)) = line.split_once('=') {
            if key.trim() == "name" {
                if let Some(name) = rest
                    .trim()
                    .strip_prefix('"')
                    .and_then(|r| r.split('"').next())
                {
                    return name.to_string();
                }
            }
        }
    }
    panic!("a manifest names no package");
}

/// What one manifest declares: every dependency section except the two
/// development ones, in both the inline form and the table form.
fn what_one_manifest_declares(text: &str) -> WhatAManifestDeclares {
    let mut the_workspace_crates = Vec::new();
    let mut every_dependency = BTreeSet::new();
    let mut the_table_form: Option<(String, bool)> = None;
    let mut in_a_dependency_section = false;

    for line in text.lines() {
        let line = line.trim();
        if let Some(inner) = a_section_heading(line) {
            if let Some((name, true)) = the_table_form.take() {
                the_workspace_crates.push(name);
            }
            match what_dependency_section_this_is(&inner) {
                None => in_a_dependency_section = false,
                Some(None) => in_a_dependency_section = true,
                Some(Some(name)) => {
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
        if let Some((_, by_a_path)) = &mut the_table_form {
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
    if let Some((name, true)) = the_table_form.take() {
        the_workspace_crates.push(name);
    }
    WhatAManifestDeclares {
        the_workspace_crates,
        every_dependency,
    }
}

fn a_section_heading(line: &str) -> Option<String> {
    Some(line.strip_prefix('[')?.strip_suffix(']')?.to_string())
}

/// `None`: not a dependency section. `Some(None)`: a list of entries.
/// `Some(Some(name))`: one entry's own table.
fn what_dependency_section_this_is(inner: &str) -> Option<Option<String>> {
    let mut parts = the_dotted_parts_of(inner);
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
        1 => Some(None),
        2 => Some(Some(parts[1].clone())),
        _ => None,
    }
}

fn the_dotted_parts_of(inner: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut held = String::new();
    let mut quote: Option<char> = None;
    for character in inner.chars() {
        match quote {
            Some(open) if character == open => quote = None,
            Some(_) => held.push(character),
            None => match character {
                '\'' | '"' => quote = Some(character),
                '.' => out.push(std::mem::take(&mut held)),
                _ => held.push(character),
            },
        }
    }
    out.push(held);
    out.into_iter()
        .map(|part| part.trim().to_string())
        .collect()
}

fn it_carries_a_path_key(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut from = 0;
    while let Some(at) = text[from..].find("path") {
        let start = from + at;
        let end = start + "path".len();
        let part_of_a_longer_word = start > 0
            && (bytes[start - 1].is_ascii_alphanumeric()
                || bytes[start - 1] == b'_'
                || bytes[start - 1] == b'-');
        if !part_of_a_longer_word && text[end..].trim_start().starts_with('=') {
            return true;
        }
        from = end;
    }
    false
}

/// **The reading sees the shapes this workspace writes**, so the guards above
/// cannot go on passing over an edge that is really there.
#[test]
fn the_reading_sees_the_shapes_this_workspace_actually_writes() {
    let manifest = r#"
[[bin]]
name = "the-binary-not-the-package"

[package]
name = "the-package"

[dependencies]
plain = { path = "../Plain" }
tight={path="../Tight"}
from-outside = { workspace = true }

[dev-dependencies]
only-for-tests = { path = "../Only for tests" }

[target.'cfg(unix)'.dependencies]
on-one-system = { path = "../On one system" }

[dependencies.its-own-table]
path = "../Its own table"

[dependencies.search]
search-path = "../Not a path key"
"#;
    assert_eq!(the_package_name(manifest), "the-package");
    let declares = what_one_manifest_declares(manifest);
    let mut by_a_path = declares.the_workspace_crates.clone();
    by_a_path.sort();
    assert_eq!(
        by_a_path,
        ["its-own-table", "on-one-system", "plain", "tight"]
    );
    assert!(declares.every_dependency.contains("from-outside"));
    assert!(!declares.every_dependency.contains("only-for-tests"));
}

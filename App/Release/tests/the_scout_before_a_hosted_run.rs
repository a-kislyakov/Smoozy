//! **The scout before a hosted run, its second check: no test runs a Mac-only
//! command or opens a Mac-only path unless it is limited to macOS, and none
//! hands Python a script on its command line unless it is kept off Windows.**
//!
//! Creator-stated, 2026-11-15: *"first scout the code for things a build would
//! find as faults anyway before actually running it on GitHub"*. The whole
//! scout is `App/Release/Scout before a hosted run.py`; this is the one part of
//! it that is also a test, so it fails at the desk on the day the fault is made.
//!
//! **What it caught first.** Hosted run 43727887590 (2026-11-15) went red on
//! Linux in a test gated for every Unix whose helper made a Mac download with
//! `/usr/bin/ditto`, which only a Mac has (`Audits/Build/Step 11/The release of
//! 0.4.0 — what happened.md`). This reads every Rust file in the workspace the
//! way that fault would have been found by a person reading it.
//!
//! **What counts as running a Mac-only command or path**, in test code only
//! (a `#[cfg(test)]` module, a `#[test]` function, or a file under `tests/`):
//! - the name of a program only macOS ships, given straight to
//!   `Command::new(…)` — `ditto`, `codesign`, `xcrun`, `security` and the rest
//!   of [`MAC_ONLY_PROGRAMS`];
//! - that program's full path anywhere, such as `/usr/bin/ditto`;
//! - a path under `/Applications/`, `/System/` or `/Library/`, or inside a
//!   `.app` bundle, given straight to a call that runs or reads it.
//!
//! A list of paths tried one by one until one exists (how the layout tests find
//! a Chrome) is not running anything, and a Mac path used as text — a path a
//! function is asked to take apart — is not either.
//!
//! **What counts as limited to macOS**, the ways the project already does it:
//! `#[cfg(target_os = "macos")]` on the test, on a function, on a module or on
//! the file, or `#[cfg_attr(not(target_os = "macos"), ignore)]`; a function that
//! first asks `cfg!(target_os = "macos")`; or `#[ignore]`, since a hosted run
//! never runs an ignored test (the desk tests that run the Creator's Blenders
//! are kept that way).
//!
//! **A helper is followed to the tests that call it**, by name and within its
//! own file, as far as it goes: a helper that runs `ditto` is a fault only when
//! some test there that is not limited to macOS reaches it. That was exactly
//! the shape of the fault above.
//!
//! **The second kind: a script handed to Python on its command line.** Hosted
//! run 43746222478 (2026-11-15) went red on Windows in three tests that built
//! a stand-in script from the reading's own rules and gave it to `python3 -c`:
//! Windows refuses a command line longer than 32,767 characters (error 206,
//! *the filename or extension is too long*), and the Mac takes one far longer.
//! So in test code, `"-c"` given to a command started as `python3` or
//! `python`, followed by anything but a written-out string (a `format!`, a
//! variable), is a fault unless the test is kept off Windows —
//! `#[cfg(unix)]`, `#[cfg(not(windows))]`, `#[cfg(target_os = "linux")]` or
//! `"macos"`, on it or on what holds it — or ignored. The script goes on
//! Python's standard input instead (`python3 -`). A written-out script is its
//! own length, and is left alone.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// **Programs only macOS ships.** Each is in `/usr/bin` or `/usr/sbin` on this
/// Mac (read 2026-11-15) and in neither on the hosted Linux runner or Windows.
const MAC_ONLY_PROGRAMS: &[&str] = &[
    "afplay", "caffeinate", "codesign", "defaults", "diskutil", "ditto", "dscl", "hdiutil",
    "install_name_tool", "installer", "ioreg", "launchctl", "lipo", "mdfind", "mdls", "mdutil",
    "networkQuality", "notarytool", "osascript", "otool", "pbcopy", "pbpaste", "pkgutil", "plutil",
    "pmset", "qlmanage", "screencapture", "scutil", "security", "sips", "softwareupdate", "spctl",
    "stapler", "sw_vers", "system_profiler", "tccutil", "textutil", "tmutil", "xcrun",
];

/// Calls that run or read what they are given. A Mac path handed straight to
/// one of these is opened, not just held.
const CALLS_THAT_OPEN: &[&str] = &[
    "Command::new(", "fs::read(", "fs::read_to_string(", "fs::metadata(", "fs::read_dir(",
    "fs::copy(", "fs::canonicalize(", "File::open(",
];

/// The top of the workspace, `App/`.
fn the_workspace() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

#[test]
fn no_test_runs_a_mac_only_command_or_path_unless_it_is_limited_to_macos() {
    the_reading_finds_what_it_must_and_nothing_else();
    let faults = the_faults_in(&the_workspaces_rust_files(), Kind::MacOnly);
    assert!(
        faults.is_empty(),
        "these tests run something only a Mac has and are not limited to macOS, \
         so the hosted Linux or Windows run would fail them. Limit each to macOS \
         the way the project does — #[cfg(target_os = \"macos\")] — or make it \
         run nothing Mac-only:\n{}",
        faults.join("\n")
    );
}

/// **The second kind: no test hands Python a script built in the test on its
/// command line unless it is kept off Windows** — hosted run 43746222478.
#[test]
fn no_test_hands_python_a_script_on_its_command_line_unless_it_is_kept_off_windows() {
    the_command_line_reading_finds_what_it_must_and_nothing_else();
    let faults = the_faults_in(&the_workspaces_rust_files(), Kind::ACommandLineWindowsMayRefuse);
    assert!(
        faults.is_empty(),
        "these tests give Python a script built in the test after -c, and Windows \
         refuses a command line past 32,767 characters, so the hosted Windows run \
         may fail them. Send the script on Python's standard input (python3 -):\n{}",
        faults.join("\n")
    );
}

/// Every tracked Rust file in the workspace, by its path from `App/`.
fn the_workspaces_rust_files() -> BTreeMap<String, String> {
    let workspace = the_workspace();
    let listing = std::process::Command::new("git")
        .arg("-C")
        .arg(&workspace)
        .args(["ls-files", "-z", "--", "*.rs"])
        .output()
        .expect("git can be run, and this check cannot look without it");
    assert!(listing.status.success(), "git ls-files failed");
    let files: Vec<String> = String::from_utf8(listing.stdout)
        .expect("git prints text")
        .split('\0')
        .filter(|path| !path.is_empty())
        .map(str::to_string)
        .collect();
    assert!(files.len() > 300, "git listed only {} Rust files", files.len());
    files
        .iter()
        .map(|path| {
            let text = std::fs::read_to_string(workspace.join(path))
                .unwrap_or_else(|why| panic!("{path} cannot be read: {why}"));
            (path.clone(), text)
        })
        .collect()
}

/// **The second reading, on small made-up files**: the shape of the three
/// Windows failures is found, through a helper as well, and a written-out
/// script, standard input and each way of keeping a test off Windows are
/// accepted.
fn the_command_line_reading_finds_what_it_must_and_nothing_else() {
    let one = |text: &str| {
        let mut sources = BTreeMap::new();
        sources.insert("Some crate/src/lib.rs".to_string(), "#[cfg(test)]\nmod tests;\n".to_string());
        sources.insert("Some crate/src/tests.rs".to_string(), text.to_string());
        the_faults_in(&sources, Kind::ACommandLineWindowsMayRefuse)
    };
    let the_fault = r#"
        #[test]
        fn a_scene_is_read_off_its_curves() {
            let stand_in = "print('{}')";
            let out = std::process::Command::new("python3")
                .arg("-c")
                .arg(format!("{RULE}\n{stand_in}"))
                .output()
                .unwrap();
        }
    "#;
    let found = one(the_fault);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("a_scene_is_read_off_its_curves"), "{found:?}");
    // `args` with the script as a variable, reached through a helper.
    let through = r#"
        fn the_stand_in(script: &str) { let _ = Command::new("python3").args(["-c", &script, "x"]).output(); }
        #[test]
        fn marks() { the_stand_in("a"); }
    "#;
    let found = one(through);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("marks") && found[0].contains("the_stand_in"), "{found:?}");

    for accepted in [
        the_fault.replace("#[test]", "#[cfg(unix)]\n#[test]"),
        the_fault.replace("#[test]", "#[cfg(not(windows))]\n#[test]"),
        the_fault.replace("#[test]", "#[cfg(target_os = \"linux\")]\n#[test]"),
        the_fault.replace("#[test]", "#[cfg(target_os = \"macos\")]\n#[test]"),
        the_fault.replace("#[test]", "#[ignore]\n#[test]"),
        format!("#![cfg(unix)]\n{the_fault}"),
        the_fault.replace(".arg(\"-c\")\n                .arg(format!(\"{RULE}\\n{stand_in}\"))", ".arg(\"-\")"),
        the_fault.replace("format!(\"{RULE}\\n{stand_in}\")", "\"print(1)\""),
        the_fault.replace("format!(\"{RULE}\\n{stand_in}\")", "r#\"print(1)\"#"),
        the_fault.replace("Command::new(\"python3\")", "Command::new(\"sh\")"),
    ] {
        assert_eq!(one(&accepted), Vec::<String>::new(), "refused:\n{accepted}");
    }
    // Limits carried down a module declaration, as `Rendering — machine`'s
    // `acting` module is kept to Unix.
    let mut sources = BTreeMap::new();
    sources.insert("Some crate/src/lib.rs".to_string(), "#[cfg(unix)]\npub mod acting;\n".to_string());
    sources.insert("Some crate/src/acting.rs".to_string(), format!("#[cfg(test)]\nmod tests {{\n{the_fault}\n}}"));
    assert_eq!(the_faults_in(&sources, Kind::ACommandLineWindowsMayRefuse), Vec::<String>::new());
    sources.insert("Some crate/src/lib.rs".to_string(), "pub mod acting;\n".to_string());
    assert_eq!(the_faults_in(&sources, Kind::ACommandLineWindowsMayRefuse).len(), 1);
}

/// **The reading itself, on small made-up files**: the fault of 2026-11-15 in
/// its own shape is found, and every way of limiting it is accepted.
fn the_reading_finds_what_it_must_and_nothing_else() {
    let one = |text: &str| {
        let mut sources = BTreeMap::new();
        sources.insert("Some crate/src/lib.rs".to_string(), "#[cfg(test)]\nmod tests;\n".to_string());
        sources.insert("Some crate/src/tests.rs".to_string(), text.to_string());
        the_faults_in(&sources, Kind::MacOnly)
    };
    // The shape of the fault: a test gated for every Unix, reaching `ditto`
    // through two helpers.
    let the_fault = r#"
        fn a_download(folder: &Path) -> Vec<u8> {
            let made = Command::new("/usr/bin/ditto").arg("-c").status().unwrap(); // a { in a comment
            let _ = "a } in a string";
            vec![]
        }
        fn an_installation() -> Vec<u8> { a_download(Path::new("x")) }
        #[cfg(unix)]
        #[test]
        fn rough_share_5() { let _ = an_installation(); }
    "#;
    let found = one(the_fault);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("rough_share_5") && found[0].contains("an_installation"), "{found:?}");

    for limited in [
        the_fault.replace("#[cfg(unix)]", "#[cfg(target_os = \"macos\")]"),
        the_fault.replace("#[cfg(unix)]", "#[cfg_attr(not(target_os = \"macos\"), ignore)]"),
        the_fault.replace("#[cfg(unix)]", "#[ignore = \"run by hand at the desk\"]"),
        the_fault.replace("fn a_download(folder: &Path) -> Vec<u8> {", "#[cfg(target_os = \"macos\")]\nfn a_download(folder: &Path) -> Vec<u8> {"),
        the_fault.replace("let made =", "if !cfg!(target_os = \"macos\") { return vec![]; }\nlet made ="),
        format!("#![cfg(target_os = \"macos\")]\n{the_fault}"),
        the_fault.replace("fn rough_share_5() { let _ = an_installation(); }", "fn rough_share_5() {}"),
    ] {
        assert_eq!(one(&limited), Vec::<String>::new(), "a limited test was refused:\n{limited}");
    }

    // A bare program name, and a path inside an application that is opened.
    assert_eq!(one("#[test]\nfn a() { Command::new(\"hdiutil\").status(); }").len(), 1);
    assert_eq!(
        one("#[test]\nfn a() { std::fs::read(\"/Applications/Blender.app/Contents/Info.plist\"); }").len(),
        1
    );
    // A path tried among others, a path as text, a word that is not a program.
    for innocent in [
        r#"#[test] fn a() { let Some(c) = ["/Applications/Google Chrome.app/Contents/MacOS/Google Chrome", "/usr/bin/chromium"].into_iter().map(PathBuf::from).find(|p| p.exists()) else { return }; }"#,
        r#"#[test] fn a() { assert_eq!(bundle_of(Path::new("/Applications/Smoozy.app/Contents/MacOS/x")), None); }"#,
        r#"#[test] fn a() { let said = json!({ "open": "security" }); let c = 'x'; let l: &'static str = "defaults"; }"#,
    ] {
        assert_eq!(one(innocent), Vec::<String>::new(), "refused:\n{innocent}");
    }
    // Code that is not a test is not this check's.
    let mut sources = BTreeMap::new();
    sources.insert("Some crate/src/lib.rs".to_string(), "fn unpack() { Command::new(\"/usr/bin/ditto\"); }".to_string());
    assert_eq!(the_faults_in(&sources, Kind::MacOnly), Vec::<String>::new());
    // A file under `tests/` is test code.
    let mut sources = BTreeMap::new();
    sources.insert("Some crate/tests/a.rs".to_string(), "#[test]\nfn a() { Command::new(\"codesign\"); }".to_string());
    assert_eq!(the_faults_in(&sources, Kind::MacOnly).len(), 1);
}

// ── The reading ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Default)]
struct Limits {
    test_code: bool,
    mac: bool,
    /// Kept off Windows: a Unix, Linux or macOS only `cfg`.
    off_windows: bool,
    ignored: bool,
}

/// The two kinds of fault this reads for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    /// Runs or opens something only a Mac has; a fault unless limited to macOS.
    MacOnly,
    /// Hands Python a script built in the test on its command line; a fault
    /// unless kept off Windows.
    ACommandLineWindowsMayRefuse,
}

/// One function, as read.
struct AFunction {
    file: String,
    name: String,
    line: usize,
    /// Its own text, between its braces.
    body: std::ops::Range<usize>,
    is_a_test: bool,
    in_test_code: bool,
    /// Limited to macOS, or ignored, by itself or by what holds it.
    limited: bool,
    /// Kept off Windows, or ignored, by itself or by what holds it.
    limited_off_windows: bool,
}

/// One place that runs or opens something only a Mac has.
struct ASite {
    file: String,
    line: usize,
    what: String,
    kind: Kind,
    /// The innermost function holding it, as an index into the functions.
    function: Option<usize>,
    limited: bool,
}

#[derive(Debug)]
enum Token {
    /// A string literal: where it starts (its prefix or quote) and its text.
    Text { at: usize, text: std::ops::Range<usize> },
    Open(usize),
    Close(usize),
    End(usize),
}

/// The file read into the pieces this check needs, with comments, characters
/// and lifetimes skipped, and strings kept whole so a brace inside one does not
/// count; and **the file again with every comment and every string's text
/// blanked**, the same length, so a word in a comment or a string is never
/// read as code. Two strings keep their text, because the limits are written
/// with them: `"macos"`, and the file named by a `path = "…"` attribute.
fn tokens(source: &str) -> (Vec<Token>, String) {
    let bytes = source.as_bytes();
    let mut found = Vec::new();
    let mut blanked = bytes.to_vec();
    let blank = |range: std::ops::Range<usize>, blanked: &mut Vec<u8>| {
        for byte in &mut blanked[range] {
            if *byte != b'\n' {
                *byte = b' ';
            }
        }
    };
    let mut at = 0;
    let part_of_a_name = |at: usize| at > 0 && (bytes[at - 1].is_ascii_alphanumeric() || bytes[at - 1] == b'_');
    while at < bytes.len() {
        let here = &bytes[at..];
        if here.starts_with(b"//") {
            let end = source[at..].find('\n').map_or(bytes.len(), |end| at + end);
            blank(at..end, &mut blanked);
            at = end;
            continue;
        }
        if here.starts_with(b"/*") {
            let from = at;
            let mut depth = 0;
            while at < bytes.len() {
                if bytes[at..].starts_with(b"/*") {
                    depth += 1;
                    at += 2;
                } else if bytes[at..].starts_with(b"*/") {
                    depth -= 1;
                    at += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    at += 1;
                }
            }
            blank(from..at.min(bytes.len()), &mut blanked);
            continue;
        }
        // A raw string: r"…", r#"…"#, br"…".
        let raw_from = if here.starts_with(b"br") { Some(at + 2) } else if here.starts_with(b"r") { Some(at + 1) } else { None };
        if let Some(from) = raw_from.filter(|_| !part_of_a_name(at)) {
            let hashes = bytes[from..].iter().take_while(|byte| **byte == b'#').count();
            if bytes.get(from + hashes) == Some(&b'"') {
                let start = from + hashes + 1;
                let closing = format!("\"{}", "#".repeat(hashes));
                let end = source[start..].find(&closing).map_or(bytes.len(), |end| start + end);
                found.push(Token::Text { at, text: start..end });
                blank(start..end, &mut blanked);
                at = end + closing.len();
                continue;
            }
        }
        let quoted_from = if here.starts_with(b"b\"") && !part_of_a_name(at) {
            Some(at + 1)
        } else if bytes[at] == b'"' {
            Some(at)
        } else {
            None
        };
        if let Some(quote) = quoted_from {
            let mut end = quote + 1;
            while end < bytes.len() && bytes[end] != b'"' {
                end += if bytes[end] == b'\\' { 2 } else { 1 };
            }
            let text = quote + 1..end.min(bytes.len());
            let kept = matches!(&source[text.clone()], "macos" | "linux" | "windows")
                || String::from_utf8_lossy(&blanked[at.saturating_sub(24)..at]).trim_end().ends_with("path =");
            if !kept {
                blank(text.clone(), &mut blanked);
            }
            found.push(Token::Text { at, text });
            at = end + 1;
            continue;
        }
        if bytes[at] == b'\'' {
            // A character ('x', '\n', '\'', '\u{2019}') or a lifetime ('a).
            if bytes.get(at + 1) == Some(&b'\\') {
                let end = source.get(at + 3..).and_then(|rest| rest.find('\'')).map_or(bytes.len(), |end| at + 3 + end);
                at = end + 1;
                continue;
            }
            if let Some(character) = source[at + 1..].chars().next() {
                let after = at + 1 + character.len_utf8();
                if bytes.get(after) == Some(&b'\'') {
                    at = after + 1;
                    continue;
                }
            }
            at += 1;
            continue;
        }
        match bytes[at] {
            b'{' => found.push(Token::Open(at)),
            b'}' => found.push(Token::Close(at)),
            b';' => found.push(Token::End(at)),
            _ => {}
        }
        at += 1;
    }
    (found, String::from_utf8(blanked).expect("whole characters are blanked"))
}

/// The attributes written in `header`, each with its spaces made single, and
/// whether it is an inner one (`#![…]`), which belongs to what holds it.
fn attributes(header: &str) -> Vec<(bool, String)> {
    let bytes = header.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;
    while let Some(start) = header[at..].find('#').map(|start| at + start) {
        let inner = bytes.get(start + 1) == Some(&b'!');
        let open = start + if inner { 2 } else { 1 };
        if bytes.get(open) != Some(&b'[') {
            at = start + 1;
            continue;
        }
        let (mut depth, mut end, mut quoted) = (0, open, false);
        while end < bytes.len() {
            match bytes[end] {
                b'"' => quoted = !quoted,
                b'\\' if quoted => end += 1,
                b'[' if !quoted => depth += 1,
                b']' if !quoted => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => {}
            }
            end += 1;
        }
        let words = header[open + 1..end.min(bytes.len())].split_whitespace().collect::<Vec<_>>().join(" ");
        found.push((inner, words));
        at = end + 1;
    }
    found
}

fn limits_written_in(attributes: &[String]) -> Limits {
    let mut limits = Limits::default();
    for attribute in attributes {
        let attribute = attribute.replace("( ", "(").replace(" )", ")");
        if attribute == "test" || attribute == "cfg(test)" || attribute.ends_with("::test") || attribute.contains("::test(") {
            limits.test_code = true;
        }
        if attribute.starts_with("cfg(target_os = \"macos\")")
            || attribute.starts_with("cfg(all(target_os = \"macos\"")
            || attribute.starts_with("cfg_attr(not(target_os = \"macos\"), ignore")
        {
            limits.mac = true;
        }
        if attribute == "ignore" || attribute.starts_with("ignore =") || attribute.starts_with("ignore(") {
            limits.ignored = true;
        }
        if limits.mac
            || ["cfg(unix)", "cfg(not(windows))", "cfg(not(target_os = \"windows\"))", "cfg(target_os = \"linux\")",
                "cfg(all(unix", "cfg(all(target_os = \"linux\"", "cfg(any(target_os = \"linux\", target_os = \"macos\")",
                "cfg(any(target_os = \"macos\", target_os = \"linux\")", "cfg_attr(windows, ignore",
                "cfg_attr(target_os = \"windows\", ignore"]
                .iter()
                .any(|written| attribute.starts_with(written))
        {
            limits.off_windows = true;
        }
    }
    limits
}

fn a_function_named_in(header: &str) -> Option<String> {
    let bytes = header.as_bytes();
    let mut at = 0;
    let mut name = None;
    while let Some(found) = header[at..].find("fn ").map(|found| at + found) {
        let alone = found == 0 || !(bytes[found - 1].is_ascii_alphanumeric() || bytes[found - 1] == b'_');
        if alone {
            let rest = header[found + 3..].trim_start();
            let its: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
            if !its.is_empty() {
                name = Some(its);
            }
        }
        at = found + 3;
    }
    name
}

/// Asks of a string literal whether it runs or opens something only a Mac
/// has; `before` is the text before it.
fn what_is_mac_only(literal: &str, before: &str) -> Option<String> {
    let before = before.trim_end();
    if MAC_ONLY_PROGRAMS.contains(&literal) && before.ends_with("Command::new(") {
        return Some(format!("runs `{literal}`"));
    }
    for folder in ["/usr/bin/", "/usr/sbin/"] {
        if let Some(program) = literal.strip_prefix(folder) {
            if MAC_ONLY_PROGRAMS.contains(&program) {
                return Some(format!("runs `{literal}`"));
            }
        }
    }
    let a_mac_place = ["/Applications/", "/System/", "/Library/"].iter().any(|place| literal.starts_with(place))
        || literal.contains(".app/Contents/");
    if a_mac_place && !literal.contains('\n') && CALLS_THAT_OPEN.iter().any(|call| before.ends_with(call)) {
        return Some(format!("opens `{literal}`"));
    }
    None
}

/// Asks of a string literal whether it is `-c` handed to Python with a script
/// that is not written out — `before` is the statement up to it and `after`
/// the text from its closing quote on, both as written.
fn a_script_on_pythons_command_line(literal: &str, before: &str, after: &str) -> Option<String> {
    if literal != "-c" || !(before.contains("Command::new(\"python3\")") || before.contains("Command::new(\"python\")")) {
        return None;
    }
    // The argument after `-c`: past `").arg(` or `", `.
    let rest = after.strip_prefix('"')?.trim_start();
    let rest = if let Some(rest) = rest.strip_prefix(')') {
        rest.trim_start().strip_prefix(".arg(")?.trim_start()
    } else {
        rest.strip_prefix(',')?.trim_start()
    };
    let written_out = rest.starts_with('"') || rest.starts_with("r\"") || rest.starts_with("r#");
    (!written_out).then(|| {
        "hands Python a script built in the test on its command line, which Windows refuses past 32,767 \
         characters (send it on standard input, `python3 -`)"
            .to_string()
    })
}

/// Reads one file: its functions, its Mac-only sites, and the modules it
/// declares in other files, with the limits each of those inherits.
fn read_one(
    file: &str,
    source: &str,
    inherited: Limits,
    functions: &mut Vec<AFunction>,
    sites: &mut Vec<ASite>,
) -> (Vec<(String, Option<String>, Limits)>, String) {
    struct Scope {
        limits: Limits,
        function: Option<usize>,
        start: usize,
    }
    let line_starts: Vec<usize> = std::iter::once(0)
        .chain(source.match_indices('\n').map(|(at, _)| at + 1))
        .collect();
    let line_of = |at: usize| line_starts.partition_point(|start| *start <= at);
    let mut stack = vec![Scope { limits: inherited, function: None, start: 0 }];
    let mut header_from = 0;
    let mut modules = Vec::new();
    let held = |stack: &[Scope]| {
        let mut limits = Limits::default();
        for scope in stack {
            limits.test_code |= scope.limits.test_code;
            limits.mac |= scope.limits.mac;
            limits.off_windows |= scope.limits.off_windows;
            limits.ignored |= scope.limits.ignored;
        }
        limits
    };
    // The attributes written since the last brace or semicolon: the outer ones
    // for what comes next, the inner ones for the scope it sits in.
    let read_the_header = |stack: &mut Vec<Scope>, header: &str| -> Limits {
        let written = attributes(header);
        let inner: Vec<String> = written.iter().filter(|(inner, _)| *inner).map(|(_, words)| words.clone()).collect();
        let outer: Vec<String> = written.iter().filter(|(inner, _)| !*inner).map(|(_, words)| words.clone()).collect();
        let innermost = stack.last_mut().expect("the file's own scope");
        let for_here = limits_written_in(&inner);
        innermost.limits.test_code |= for_here.test_code;
        innermost.limits.mac |= for_here.mac;
        innermost.limits.off_windows |= for_here.off_windows;
        innermost.limits.ignored |= for_here.ignored;
        limits_written_in(&outer)
    };
    let original = source;
    let (found, blanked) = tokens(original);
    let source = blanked.as_str();
    for token in found {
        match token {
            Token::Text { at, text } => {
                let literal = &original[text.clone()];
                let before = &source[header_from.min(at)..at];
                let found = what_is_mac_only(literal, before).map(|what| (Kind::MacOnly, what)).or_else(|| {
                    a_script_on_pythons_command_line(literal, &original[header_from.min(at)..at], &original[text.end..])
                        .map(|what| (Kind::ACommandLineWindowsMayRefuse, what))
                });
                let Some((kind, what)) = found else { continue };
                let limits = held(&stack);
                if !limits.test_code {
                    continue;
                }
                let asked_first = |words: &[&str]| {
                    stack.iter().filter(|scope| scope.function.is_some()).any(|scope| {
                        let asked = &source[scope.start..at];
                        words.iter().any(|word| asked.contains(word))
                    })
                };
                let limited = match kind {
                    Kind::MacOnly => {
                        limits.mac
                            || limits.ignored
                            || asked_first(&["cfg!(target_os = \"macos\")", "cfg!(all(target_os = \"macos\""])
                    }
                    Kind::ACommandLineWindowsMayRefuse => limits.off_windows || limits.ignored,
                };
                sites.push(ASite {
                    file: file.to_string(),
                    line: line_of(at),
                    what,
                    kind,
                    function: stack.iter().rev().find_map(|scope| scope.function),
                    limited,
                });
            }
            Token::Open(at) => {
                let header = &source[header_from..at];
                let mut limits = read_the_header(&mut stack, header);
                let around = held(&stack);
                let function = a_function_named_in(header).map(|name| {
                    functions.push(AFunction {
                        file: file.to_string(),
                        name,
                        line: line_of(at),
                        body: at + 1..at + 1,
                        is_a_test: attributes(header).iter().any(|(inner, a)| {
                            !inner && (a == "test" || a.ends_with("::test") || a.contains("::test("))
                        }),
                        in_test_code: around.test_code || limits.test_code,
                        limited: around.mac || around.ignored || limits.mac || limits.ignored,
                        limited_off_windows: around.off_windows || around.ignored || limits.off_windows || limits.ignored,
                    });
                    functions.len() - 1
                });
                if function.is_some_and(|index| functions[index].is_a_test) {
                    // The code inside a test is test code.
                    limits.test_code = true;
                }
                stack.push(Scope { limits, function, start: at + 1 });
                header_from = at + 1;
            }
            Token::Close(at) => {
                if stack.len() > 1 {
                    let closed = stack.pop().expect("more than one");
                    if let Some(index) = closed.function {
                        functions[index].body = closed.start..at;
                    }
                }
                header_from = at + 1;
            }
            Token::End(at) => {
                let header = &source[header_from..at];
                let limits = read_the_header(&mut stack, header);
                let words: Vec<&str> = header.split_whitespace().collect();
                if let Some(position) = words.iter().rposition(|word| *word == "mod") {
                    if position + 2 == words.len() {
                        let path = attributes(header).into_iter().find_map(|(_, attribute)| {
                            attribute.strip_prefix("path = \"").and_then(|rest| rest.strip_suffix('"')).map(str::to_string)
                        });
                        let around = held(&stack);
                        let carried = Limits {
                            test_code: around.test_code || limits.test_code,
                            mac: around.mac || limits.mac,
                            off_windows: around.off_windows || limits.off_windows,
                            ignored: around.ignored || limits.ignored,
                        };
                        modules.push((words[position + 1].to_string(), path, carried));
                    }
                }
                header_from = at + 1;
            }
        }
    }
    (modules, blanked.clone())
}

/// Every fault of this kind in the files given, as lines a person can read.
fn the_faults_in(sources: &BTreeMap<String, String>, kind: Kind) -> Vec<String> {
    let kept_to = match kind {
        Kind::MacOnly => "is not limited to macOS",
        Kind::ACommandLineWindowsMayRefuse => "is not kept off Windows",
    };
    let limited_for = |function: &AFunction| match kind {
        Kind::MacOnly => function.limited,
        Kind::ACommandLineWindowsMayRefuse => function.limited_off_windows,
    };
    let mut functions = Vec::new();
    let mut sites = Vec::new();
    let mut read = BTreeSet::new();
    let mut blanked: BTreeMap<String, String> = BTreeMap::new();

    // Each crate's roots, then the modules they declare, carrying limits down.
    let mut waiting: Vec<(String, Limits)> = Vec::new();
    for path in sources.keys() {
        let parts: Vec<&str> = path.split('/').collect();
        let n = parts.len();
        if n >= 3 && parts[n - 2] == "src" && (parts[n - 1] == "lib.rs" || parts[n - 1] == "main.rs") {
            waiting.push((path.clone(), Limits::default()));
        } else if n >= 3 && parts[n - 2] == "bin" {
            waiting.push((path.clone(), Limits::default()));
        } else if n >= 3 && parts[n - 2] == "tests" {
            waiting.push((path.clone(), Limits { test_code: true, ..Limits::default() }));
        }
    }
    while let Some((path, limits)) = waiting.pop() {
        if !read.insert(path.clone()) {
            continue;
        }
        let (modules, its_blanked) = read_one(&path, &sources[&path], limits, &mut functions, &mut sites);
        blanked.insert(path.clone(), its_blanked);
        let folder = path.rsplit_once('/').map_or("", |(folder, _)| folder).to_string();
        let file_name = path.rsplit('/').next().unwrap_or("");
        let is_a_root = matches!(file_name, "lib.rs" | "main.rs" | "mod.rs")
            || folder.ends_with("/tests")
            || folder.ends_with("/bin");
        let below = if is_a_root { folder.clone() } else { format!("{folder}/{}", file_name.trim_end_matches(".rs")) };
        for (name, written_path, carried) in modules {
            let candidates = match written_path {
                Some(written) => vec![format!("{folder}/{written}")],
                None => vec![format!("{below}/{name}.rs"), format!("{below}/{name}/mod.rs")],
            };
            if let Some(found) = candidates.into_iter().find(|candidate| sources.contains_key(candidate)) {
                waiting.push((found, carried));
            }
        }
    }
    // A file no module reaches is still read, as code that is not a test
    // unless it says so itself.
    for (path, source) in sources {
        if !read.contains(path) {
            let (_, its_blanked) = read_one(path, source, Limits::default(), &mut functions, &mut sites);
            blanked.insert(path.clone(), its_blanked);
        }
    }

    let mut faults = Vec::new();
    // Helpers that run something Mac-only and are not limited, followed out to
    // the tests that reach them, by name: (helper, the way it was reached).
    let mut reaching: Vec<(usize, Vec<String>, String)> = Vec::new();
    for site in sites.iter().filter(|site| !site.limited && site.kind == kind) {
        let place = format!("{}:{}", site.file, site.line);
        match site.function {
            Some(index) if limited_for(&functions[index]) => {}
            Some(index) if functions[index].is_a_test => faults.push(format!(
                "  {place}: the test `{}` {} and {kept_to}",
                functions[index].name, site.what
            )),
            Some(index) => reaching.push((index, Vec::new(), format!("{} (at {place})", site.what))),
            None => faults.push(format!("  {place}: {} outside any function, and {kept_to}", site.what)),
        }
    }
    let mut followed = BTreeSet::new();
    while let Some((helper, way, what)) = reaching.pop() {
        if !followed.insert((helper, what.clone())) {
            continue;
        }
        let name = &functions[helper].name;
        let mut way = way;
        way.push(format!("`{name}`"));
        for (index, caller) in functions.iter().enumerate() {
            // **Followed within its own file only.** A helper and the tests
            // that use it sit together, as they did in the fault above; a
            // name matched across the whole workspace would match every
            // `new` and `get` there is.
            if index == helper || caller.file != functions[helper].file || !caller.in_test_code || limited_for(caller) {
                continue;
            }
            let body = &blanked[&caller.file][caller.body.clone()];
            if !calls(body, name) {
                continue;
            }
            if caller.is_a_test {
                faults.push(format!(
                    "  {}:{}: the test `{}` goes through {} to code that {}, and {kept_to}",
                    caller.file,
                    caller.line,
                    caller.name,
                    way.iter().rev().cloned().collect::<Vec<_>>().join(", "),
                    what
                ));
            } else {
                reaching.push((index, way.clone(), what.clone()));
            }
        }
    }
    faults.sort();
    faults.dedup();
    faults
}

/// Whether `body` calls a function named `name`: the name, not inside a longer
/// name, followed by an opening bracket.
fn calls(body: &str, name: &str) -> bool {
    let bytes = body.as_bytes();
    let mut at = 0;
    while let Some(found) = body[at..].find(name).map(|found| at + found) {
        let before_is_a_name = found > 0 && (bytes[found - 1].is_ascii_alphanumeric() || bytes[found - 1] == b'_');
        let after = body[found + name.len()..].trim_start();
        if !before_is_a_name && (after.starts_with('(') || after.starts_with("::<")) {
            return true;
        }
        at = found + name.len();
    }
    false
}

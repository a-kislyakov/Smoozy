//! The typography application point, and the verbatim route beside it.
//!
//! Keeps `Desktop app and delivery` section 17: X1 (one application point),
//! X2 (three substitutions and nothing else), X7 (what it must never touch),
//! X8 (the module chooses at the moment it hands the string over) and
//! X9 (the verbatim route is named rather than invented).
//!
//! The two types below are the complete set of ways a string reaches the
//! interface. Making them types rather than a convention is what keeps X8:
//! handing a path where prose is expected does not compile.

const NBSP: char = '\u{00A0}';
const EM_DASH: char = '\u{2014}';
/// **The one apostrophe every screen shows**, as the drawing writes it.
const APOSTROPHE: char = '\u{2019}';

/// Text for a person. Built only through [`Prose::new`], which is the
/// application point — `Desktop app and delivery` X1.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Prose(String);

/// A value the user reads but that Smoozy must not alter: a path, a file
/// name, a host, a port, the machine's own words, anything copyable as data.
/// `Desktop app and delivery` X7 and X9. No substitution of any kind.
///
/// Unused at step 1, which draws no values, and kept rather than added later
/// because X9 says outright that a route which is not named is a route a
/// builder invents.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Verbatim(String);

impl Prose {
    /// `Desktop app and delivery` X1 and X2. Every string of prose that will
    /// become displayable text passes through here, and the point performs the
    /// three substitutions and nothing else.
    ///
    /// A piece of prose is one string as handed in (X2). The point does not
    /// parse sentences out of a paragraph and does not join two strings, and
    /// it never learns to (X10) — a caller that needs the orphan rule inside a
    /// multi-sentence block hands each sentence as its own string.
    pub fn new(text: impl Into<String>) -> Self {
        Prose(bind_last_pair(&bind_before_em_dashes(&one_apostrophe(&text.into()))))
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[allow(dead_code)]
impl Verbatim {
    /// `Desktop app and delivery` X9. The route that performs no substitution.
    pub fn new(text: impl Into<String>) -> Self {
        Verbatim(text.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ════════════════════════════════════════════════════════════════════════════
// A figure and its unit
// ════════════════════════════════════════════════════════════════════════════
//
// `Design/Whole app § Typography`, Creator-stated 2026-09-28: *"Every numeric
// value followed by a unit should be connected with a non-breaking space. So,
// 47 GB should never become: 47 / GB."* **It is held where the figure is
// written and not by the point**: X2 gives the point three substitutions and
// nothing else, and a value reaches the screen by the route that substitutes
// nothing (X9). So the space is a non-breaking one from the moment the figure
// and its unit are put together, and these are the two places that happens.

/// **What stands between a figure and its unit, everywhere Smoozy writes one.**
/// The pages write the same character where the application's bind mark says
/// so (`web/*.js`, *the_rest_binds*, *binds*, *…Binds*).
pub const BETWEEN_A_FIGURE_AND_ITS_UNIT: char = NBSP;

/// **A figure and its unit, as one value that never parts across two lines** —
/// *24 GB*, *12.4 MB/s*, *6 days*, *240 EXR*. `Design/Whole app § Typography`.
/// Every formatter that writes a figure with its unit calls this rather than
/// writing `"{} {}"`, and the class check in this file's tests,
/// `a_figure_and_its_unit_are_never_written_with_an_ordinary_space`, fails on a
/// later one that does not.
pub fn a_figure_with_its_unit(figure: impl std::fmt::Display, unit: &str) -> String {
    format!("{figure}{BETWEEN_A_FIGURE_AND_ITS_UNIT}{unit}")
}

/// **Whether a value is a figure**, so the words after it are bound to it.
///
/// **A figure followed by words is bound to them whatever the words are** —
/// *24 GB*, *2 jobs*, and *2 waiting* alike. Telling a unit from any other word
/// would be a page, or this, reading prose; binding every figure to the word
/// after it costs nothing, since a figure and the word it counts belong on one
/// line either way. A value is a figure when it begins with a digit; a
/// moment, which the page writes as a time, is not handed here.
pub fn is_a_figure(value: &str) -> bool {
    value.chars().next().is_some_and(|first| first.is_ascii_digit())
}

/// **Third substitution, added 2026-11-06: a typed apostrophe after a letter or
/// a figure becomes the typographic one** (U+2019), so *server's* and *can't*
/// look the same on every screen however the string was typed (`Design/Whole
/// app § Typography`; Step 13's last read, finding 27). **Only after a letter
/// or a figure**: that is every contraction and possessive, *servers'*
/// included, and never a quotation mark opened after a space.
fn one_apostrophe(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut previous: Option<char> = None;
    for character in text.chars() {
        if character == '\'' && previous.is_some_and(char::is_alphanumeric) {
            out.push(APOSTROPHE);
        } else {
            out.push(character);
        }
        previous = Some(character);
    }
    out
}

/// First substitution: the space preceding an em dash becomes a non-breaking
/// space, so a dash never begins a line.
fn bind_before_em_dashes(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut previous_was_space = false;
    for character in text.chars() {
        if character == EM_DASH && previous_was_space {
            out.pop();
            out.push(NBSP);
        }
        out.push(character);
        previous_was_space = character == ' ';
    }
    out
}

/// Second substitution: the last inter-word space becomes a non-breaking
/// space, so a sentence never ends with one word alone on a line.
///
/// Trailing whitespace is stepped over rather than counted, and a pair already
/// bound — by the first substitution, or by a non-breaking space the caller
/// wrote — is left alone.
fn bind_last_pair(text: &str) -> String {
    let characters: Vec<char> = text.chars().collect();
    let mut index = characters.len();

    while index > 0 && characters[index - 1].is_whitespace() {
        index -= 1;
    }
    while index > 0 {
        let character = characters[index - 1];
        if character == ' ' {
            let mut out: String = characters[..index - 1].iter().collect();
            out.push(NBSP);
            out.extend(&characters[index..]);
            return out;
        }
        if character.is_whitespace() || character == NBSP {
            // Already bound, or a break the point does not touch.
            return text.to_string();
        }
        index -= 1;
    }
    text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(text: &str) -> String {
        Prose::new(text).as_str().to_string()
    }

    #[test]
    fn binds_the_space_before_an_em_dash() {
        assert_eq!(
            point("one \u{2014} two three"),
            "one\u{00A0}\u{2014} two\u{00A0}three"
        );
    }

    #[test]
    fn binds_every_em_dash_not_only_the_first() {
        let bound = point("a \u{2014} b \u{2014} c d");
        assert_eq!(bound, "a\u{00A0}\u{2014} b\u{00A0}\u{2014} c\u{00A0}d");
    }

    #[test]
    fn binds_the_last_two_words() {
        assert_eq!(point("the server is ready"), "the server is\u{00A0}ready");
    }

    #[test]
    fn steps_over_trailing_space() {
        assert_eq!(point("the server is ready "), "the server is\u{00A0}ready ");
    }

    #[test]
    fn a_single_word_is_left_alone() {
        assert_eq!(point("Smoozy"), "Smoozy");
    }

    #[test]
    fn an_empty_string_is_left_alone() {
        assert_eq!(point(""), "");
    }

    #[test]
    fn a_pair_already_bound_is_left_alone() {
        assert_eq!(point("two\u{00A0}words"), "two\u{00A0}words");
    }

    #[test]
    fn a_dash_ending_the_string_still_binds_its_own_space() {
        assert_eq!(point("cut off \u{2014}"), "cut off\u{00A0}\u{2014}");
    }

    #[test]
    fn a_dash_with_no_space_before_it_is_untouched() {
        assert_eq!(point("one\u{2014}two three"), "one\u{2014}two\u{00A0}three");
    }

    /// **`Design/Whole app § Typography`: 47 GB never becomes 47 / GB.** The
    /// figure and its unit come out joined by U+00A0, and the point, handed a
    /// sentence ending in one, leaves the pair alone rather than binding the
    /// ordinary space before it a second time.
    #[test]
    fn a_figure_and_its_unit_are_joined_by_a_non_breaking_space() {
        assert_eq!(a_figure_with_its_unit(47, "GB"), "47\u{00A0}GB");
        assert_eq!(a_figure_with_its_unit("12.4", "MB/s"), "12.4\u{00A0}MB/s");
        assert!(!a_figure_with_its_unit(6, "days").contains(' '));
        assert_eq!(point("free disk 47\u{00A0}GB"), "free disk 47\u{00A0}GB");
    }

    #[test]
    fn a_figure_is_a_value_that_begins_with_a_digit() {
        assert!(is_a_figure("2"));
        assert!(is_a_figure("24\u{00A0}GB"));
        assert!(is_a_figure("1 × NVIDIA GeForce RTX 4090"));
        assert!(!is_a_figure("kitchen_v14"));
        assert!(!is_a_figure(""));
        assert!(!is_a_figure("\u{2014}"));
    }

    // ════════════════════════════════════════════════════════════════════
    // The class check: a figure and its unit, on any page
    // ════════════════════════════════════════════════════════════════════

    /// **The words a figure is written with that make it a figure with a
    /// unit** — sizes, speeds, lengths of time, pixels, and the counts a screen
    /// writes after a figure. A word here followed by a letter is not the word.
    const UNITS: [&str; 30] = [
        "B", "KB", "MB", "GB", "TB", "bytes", "MB/s", "KB/s", "GB/s", "ms", "s", "seconds",
        "second", "minutes", "minute", "hours", "hour", "days", "day", "pixels", "px", "files",
        "file", "frames", "frame", "jobs", "job", "GPUs", "GPU", "%",
    ];

    /// Whether `text` begins with one of [`UNITS`], standing as a word.
    fn begins_with_a_unit(text: &str) -> bool {
        UNITS.iter().any(|unit| {
            text.strip_prefix(unit)
                .is_some_and(|after| !after.starts_with(|c: char| c.is_ascii_alphanumeric()))
        })
    }

    /// **Every string literal in a piece of source**, with where it ends, and
    /// comments stepped over — `//` and `/* */`, and in Rust a character
    /// literal holding a quote. Raw strings are read to their own closing.
    /// It is a text check, written out, and it reads source rather than
    /// running it.
    fn the_literals_in(source: &str, javascript: bool) -> Vec<(String, usize)> {
        let text: Vec<char> = source.chars().collect();
        let mut out = Vec::new();
        let mut at = 0;
        while at < text.len() {
            let here = text[at];
            let next = text.get(at + 1).copied();
            if here == '/' && next == Some('/') {
                while at < text.len() && text[at] != '\n' {
                    at += 1;
                }
                continue;
            }
            if here == '/' && next == Some('*') {
                at += 2;
                while at + 1 < text.len() && !(text[at] == '*' && text[at + 1] == '/') {
                    at += 1;
                }
                at += 2;
                continue;
            }
            // In Rust a single quote is a character or a lifetime, never a
            // string: '"', '\'' and 'static are stepped over.
            if here == '\'' && !javascript {
                if text.get(at + 2) == Some(&'\'') {
                    at += 3;
                } else if next == Some('\\') && text.get(at + 3) == Some(&'\'') {
                    at += 4;
                } else {
                    at += 1;
                }
                continue;
            }
            // A raw string: r"…", r#"…"#.
            if !javascript
                && here == 'r'
                && (next == Some('"') || next == Some('#'))
                && (at == 0 || !text[at - 1].is_alphanumeric())
            {
                let mut hashes = 0;
                let mut open = at + 1;
                while text.get(open) == Some(&'#') {
                    hashes += 1;
                    open += 1;
                }
                if text.get(open) == Some(&'"') {
                    let from = open + 1;
                    let mut end = from;
                    loop {
                        if end >= text.len() {
                            break;
                        }
                        if text[end] == '"' && (0..hashes).all(|k| text.get(end + 1 + k) == Some(&'#')) {
                            break;
                        }
                        end += 1;
                    }
                    out.push((text[from..end.min(text.len())].iter().collect(), end));
                    at = end + 1 + hashes;
                    continue;
                }
            }
            if here == '"' || (javascript && (here == '`' || here == '\'')) {
                let quote = here;
                let from = at + 1;
                let mut end = from;
                while end < text.len() && text[end] != quote {
                    if text[end] == '\\' {
                        end += 1;
                    }
                    end += 1;
                }
                out.push((text[from..end.min(text.len())].iter().collect(), end));
                at = end + 1;
                continue;
            }
            at += 1;
        }
        out
    }

    /// **What in one literal writes a figure and its unit with an ordinary
    /// space**: a figure put in by a placeholder (`{…}` or `${…}`) or written as
    /// a digit, then one ordinary space, then a unit; or a placeholder, a space
    /// and a second placeholder where the words that follow the literal name a
    /// unit (`"{left:.1} {}", STEPS[at]`); or a literal that is a space and a
    /// unit alone (`" GB"`), which is what is added on to a figure.
    fn the_faults_in_a_literal(literal: &str, after: &str) -> Vec<String> {
        let chars: Vec<char> = literal.chars().collect();
        let mut faults = Vec::new();
        let names_a_unit = {
            let near: String = after.chars().take(160).collect::<String>().to_lowercase();
            let near = near.split(';').next().unwrap_or_default().to_string();
            near.contains("unit") || near.contains("steps")
        };
        let mut at = 0;
        while at < chars.len() {
            // Where a figure ends: a closing placeholder brace, or a digit.
            let a_figure_ends = chars[at] == '}' || chars[at].is_ascii_digit();
            if a_figure_ends && chars.get(at + 1) == Some(&' ') {
                let rest: String = chars[at + 2..].iter().collect();
                // A second placeholder that is a unit: named so after the
                // literal, or by its own name (`{unit}`, `${steps[at]}`).
                let the_next_names_a_unit = (rest.starts_with('{') || rest.starts_with("${")) && {
                    let inside: String = rest.chars().take_while(|c| *c != '}').collect::<String>().to_lowercase();
                    names_a_unit || inside.contains("unit") || inside.contains("step")
                };
                if begins_with_a_unit(&rest) || the_next_names_a_unit {
                    let from = at.saturating_sub(12);
                    let to = (at + 14).min(chars.len());
                    faults.push(chars[from..to].iter().collect());
                }
            }
            at += 1;
        }
        if literal.starts_with(' ') && begins_with_a_unit(&literal[1..]) && literal.trim().len() + 1 == literal.len() {
            faults.push(literal.to_string());
        }
        if literal == " " && names_a_unit {
            let near: String = after.chars().take(40).collect();
            if near.trim_start().starts_with('+') {
                faults.push(format!("\" \" +{}", near.trim_start().trim_start_matches('+')));
            }
        }
        faults
    }

    /// Every fault in one file's source, by line.
    fn the_faults_in(name: &str, source: &str) -> Vec<String> {
        // A test module writes figures in its assertions and its messages, and
        // neither reaches a page.
        let working = ["#[cfg(test)]\nmod ", "#[cfg(test)]\npub(crate) mod ", "#[cfg(test)]\npub mod "]
            .iter()
            .filter_map(|opens| source.find(opens))
            .min()
            .map_or(source, |at| &source[..at]);
        let mut out = Vec::new();
        let offsets: Vec<usize> = working.char_indices().map(|(at, _)| at).collect();
        for (literal, ends) in the_literals_in(working, name.ends_with(".js")) {
            let byte_end = offsets.get(ends).copied().unwrap_or(working.len());
            let after = working.get((byte_end + 1).min(working.len())..).unwrap_or_default();
            for fault in the_faults_in_a_literal(&literal, after) {
                let line = working[..byte_end].matches('\n').count() + 1;
                out.push(format!("{name}:{line}: {fault:?}"));
            }
        }
        out
    }

    fn every_file_under(folder: &std::path::Path, ending: &str, out: &mut Vec<(String, String)>) {
        for entry in std::fs::read_dir(folder).expect("the folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.is_dir() {
                every_file_under(&path, ending, out);
            } else if path.extension().and_then(|e| e.to_str()) == Some(ending) {
                let name = path
                    .strip_prefix(env!("CARGO_MANIFEST_DIR"))
                    .unwrap_or(&path)
                    .display()
                    .to_string();
                out.push((name, std::fs::read_to_string(&path).expect("the file is readable")));
            }
        }
    }

    /// **`Design/Whole app § Typography`, Creator-stated 2026-09-28: a number
    /// and its unit are never split across two lines, anywhere in the app.**
    ///
    /// **The class check.** Every file of the application's source and every
    /// page is read, and a figure written with an ordinary space before its
    /// unit fails here — `"{} GB"`, `"1 hour"`, `` `${x} ${steps[at]}` ``,
    /// `+ " " + steps[at]`. The fix is never a different spelling of the same
    /// space: it is [`a_figure_with_its_unit`], or on a page the bind mark the
    /// application sends. **A figure that reaches a page as a value beside its
    /// words is held the other way**: every sentence shape binds words after a
    /// figure where it becomes what the page reads (`Words` in
    /// `src/surface/servers.rs` and `src/surface/files.rs`, the queue's parts),
    /// and [`every_page_that_joins_words_honours_the_bind_mark`] holds the
    /// pages to it.
    ///
    /// **What it cannot see**, written out so nobody reports a pass it did not
    /// earn: a figure and its unit joined by `"{} {}"` with no unit named near
    /// it, a figure built from characters, and anything a test module writes.
    #[test]
    fn a_figure_and_its_unit_are_never_written_with_an_ordinary_space() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut files = Vec::new();
        every_file_under(&root.join("src"), "rs", &mut files);
        every_file_under(&root.join("web"), "js", &mut files);
        assert!(files.len() > 100, "the source was read: {} files", files.len());
        let faults: Vec<String> = files
            .iter()
            .flat_map(|(name, text)| the_faults_in(name, text))
            .collect();
        assert!(
            faults.is_empty(),
            "a figure and its unit are written with an ordinary space, so they can part \
             across two lines. `Design/Whole app § Typography`: use \
             `crate::typography::a_figure_with_its_unit`, or on a page the bind mark: \
             {faults:#?}"
        );
    }

    /// **The class check bites** on every form the formatters were written in
    /// until 2026-09-28, each line as it stood, and passes each as it stands.
    #[test]
    fn the_class_check_bites_on_the_forms_it_replaced() {
        for (was, is) in [
            (r#"format!("{gigabytes} GB")"#, r#"a_figure_with_its_unit(gigabytes, "GB")"#),
            (r#"format!("{left:.1} {}", STEPS[at])"#, r#"a_figure_with_its_unit(format!("{left:.1}"), STEPS[at])"#),
            (r#"return format!("{bytes} {}", UNITS[0]);"#, r#"return a_figure_with_its_unit(bytes, UNITS[0]);"#),
            (r#"format!("{}.{} {unit}", tenths / 10, tenths % 10)"#, r#"a_figure_with_its_unit(format!("{}.{}", tenths / 10, tenths % 10), unit)"#),
            (r#"return format!("<0.1 {unit}");"#, r#"return a_figure_with_its_unit("<0.1", unit);"#),
            (r#""1 hour".to_string()"#, r#"a_figure_with_its_unit(1, "hour")"#),
            (r#"format!("{many} {unit}s")"#, r#"a_figure_with_its_unit(many, &format!("{unit}s"))"#),
            (r#""Make it at least {THE_FLOOR} pixels wide.""#, r#""Make it at least {} wide.""#),
            (r#"return (at === 0 ? left : left.toFixed(1)) + " " + steps[at];"#, r#"return (at === 0 ? left : left.toFixed(1)) + BETWEEN_A_FIGURE_AND_ITS_UNIT + steps[at];"#),
            (r#"return `${at === 0 ? left : left.toFixed(1)} ${steps[at]}`;"#, r#"return `${at === 0 ? left : left.toFixed(1)}${BETWEEN_A_FIGURE_AND_ITS_UNIT}${steps[at]}`;"#),
            (r#"const size = `${n} MB`;"#, r#"const size = n;"#),
            (r#"label = count + " files";"#, r#"label = count;"#),
        ] {
            let javascript = was.contains("===") || was.contains("const ") || was.contains("label =");
            let name = if javascript { "was.js" } else { "was.rs" };
            assert!(!the_faults_in(name, was).is_empty(), "the check did not bite on {was}");
            assert!(the_faults_in(name, is).is_empty(), "the check bit on {is}: {:?}", the_faults_in(name, is));
        }
    }

    /// Whether a line of page code names a figure: a count, a figure, a size.
    fn names_a_figure(code: &str) -> bool {
        ["figure", "Figure", "many", "Many", "count", "Count", "Frames", "Entries", "bytes("]
            .iter()
            .any(|name| {
                code.match_indices(name).any(|(at, _)| {
                    let before = code[..at].chars().last();
                    let after = code[at + name.len()..].chars().next();
                    // A name standing as a word or ending a key: `many`,
                    // `said.submitAgainFrames`, `theJobsFigure`, never `howManyRows`.
                    !before.is_some_and(|c| c.is_ascii_lowercase())
                        && !after.is_some_and(|c| c.is_ascii_lowercase())
                        || name.ends_with('(')
                        || name.starts_with(|c: char| c.is_ascii_uppercase())
                            && !after.is_some_and(|c| c.is_ascii_alphabetic())
                })
            })
    }

    /// Whether a line of page code draws words: it puts text on the page, and
    /// the text is a word key, a unit or something the event said.
    fn names_words(code: &str) -> bool {
        ["createTextNode(", "textNode(", "textContent"].iter().any(|sink| code.contains(sink))
            && ["Word", "word", "Unit", "unit", "Lead", "said.", "said["].iter().any(|name| code.contains(name))
    }

    /// **The shape the pages drew a count and its word in, side by side**: a
    /// node holding a figure, then a plain space, then a node of words —
    /// `into.appendChild(many); into.appendChild(document.createTextNode(" "));
    /// into.appendChild(document.createTextNode(word));` — and the template
    /// `${many} ${said.notReachedLead}`. A space after a call to
    /// `aFigureAndItsWord` is after its word, not its figure, and is not this.
    fn the_counts_spaced_from_their_words(name: &str, source: &str) -> Vec<String> {
        let code: Vec<(usize, &str)> = source
            .lines()
            .enumerate()
            .filter(|(_, line)| {
                let line = line.trim_start();
                !(line.starts_with("//") || line.starts_with('*') || line.starts_with("/*"))
            })
            .collect();
        let mut out = Vec::new();
        for (at, (number, line)) in code.iter().enumerate() {
            if line.contains("createTextNode(\" \")") || line.contains("textNode(\" \")") {
                let before: Vec<&str> = code[at.saturating_sub(2)..at].iter().map(|(_, l)| *l).collect();
                let after: Vec<&str> = code[at + 1..(at + 3).min(code.len())].iter().map(|(_, l)| *l).collect();
                let after_a_whole_count = before.last().is_some_and(|l| l.contains("aFigureAndItsWord("));
                if !after_a_whole_count && before.iter().any(|l| names_a_figure(l)) && after.iter().any(|l| names_words(l)) {
                    out.push(format!("{name}:{}: {}", number + 1, line.trim()));
                }
            }
            let mut rest = *line;
            while let Some(opens) = rest.find("${") {
                let inside = &rest[opens + 2..];
                let Some(closes) = inside.find('}') else { break };
                if inside[closes + 1..].starts_with(" ${") && names_a_figure(&inside[..closes]) {
                    out.push(format!("{name}:{}: {}", number + 1, line.trim()));
                }
                rest = &inside[closes + 1..];
            }
        }
        out
    }

    /// **A count and the word that counts it are held together on the pages
    /// too** — the runner's widening of 2026-09-28: *"a count followed by its
    /// word is covered by the Creator's rule too."* The pages that draw a figure
    /// and its word from two keys join them with U+00A0, never an ordinary space.
    #[test]
    fn a_count_and_its_word_are_never_drawn_with_an_ordinary_space() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut pages = Vec::new();
        every_file_under(&root.join("web"), "js", &mut pages);
        let faults: Vec<String> = pages
            .iter()
            .flat_map(|(name, text)| the_counts_spaced_from_their_words(name, text))
            .collect();
        assert!(
            faults.is_empty(),
            "a page draws a count and its word with an ordinary space between them, so they \
             can part across two lines. `Design/Whole app § Typography`: write \
             BETWEEN_A_FIGURE_AND_ITS_UNIT there: {faults:#?}"
        );
    }

    /// **The widened check bites** on each of the seven page lines it was
    /// written against, as they stood, and passes each as it stands — and it
    /// leaves the space after a whole count alone.
    #[test]
    fn the_count_check_bites_on_the_forms_it_replaced() {
        let space = "createTextNode(\" \")";
        let bound = "createTextNode(BETWEEN_A_FIGURE_AND_ITS_UNIT)";
        for was in [
            format!("  const many = document.createElement(\"b\");\n  many.textContent = figure;\n  into.appendChild(many);\n  if (word) {{\n    into.appendChild(document.{space});\n    into.appendChild(document.createTextNode(word));"),
            format!("  submit.textContent = said.submitAgain || \"\";\n  submit.appendChild(document.createTextNode(said.submitAgainFrames || \"\"));\n  submit.appendChild(document.{space});\n  submit.appendChild(document.createTextNode(said.submitAgainFramesWord || \"\"));"),
            format!("      count.textContent = many || \"\";\n      count.appendChild(document.{space});\n      const word = said[\"boxServer\" + at + \"EntriesWord\"];\n      count.appendChild(document.createTextNode(word || \"\"));"),
            format!("    label.appendChild(many);\n    label.appendChild(document.{space});\n    label.appendChild(document.createTextNode(parts.theCountsWord || \"\"));"),
            format!("  many.textContent = said.theEditWarningHowMany || \"\";\n  warning.appendChild(many);\n  warning.appendChild(textNode(\" \"));\n  warning.appendChild(textNode(said.theEditWarningHaveNotEnded || \"\"));"),
            format!("        many.textContent = bytes(figure);\n        one.appendChild(many);\n        one.appendChild(document.{space});\n        const said = document.createElement(\"span\");\n        said.textContent = word || \"\";"),
            "    tg.notReached.appendChild(bold(`${many} ${said.notReachedLead || \"\"}`));".to_string(),
        ] {
            let is = was
                .replace(&format!("document.{space}"), &format!("document.{bound}"))
                .replace("textNode(\" \")", "textNode(BETWEEN_A_FIGURE_AND_ITS_UNIT)")
                .replace("${many} ${", "${many}${BETWEEN_A_FIGURE_AND_ITS_UNIT}${");
            assert!(!the_counts_spaced_from_their_words("was.js", &was).is_empty(), "the check did not bite on {was}");
            assert!(the_counts_spaced_from_their_words("is.js", &is).is_empty(), "the check bit on {is}");
        }
        let a_whole_count = format!("  aFigureAndItsWord(folded, said.theNoticeJobsFigure, said.theNoticeJobsWord);\n  folded.appendChild(document.{space});\n  on.textContent = said.onWord || \"\";");
        assert!(the_counts_spaced_from_their_words("q.js", &a_whole_count).is_empty());
    }

    /// **Every page that sets words beside a value honours the bind mark** the
    /// application sends, writing U+00A0 where it says so — so a figure and
    /// the words after it never part, and the page never decides which word is
    /// a unit. One check per sentence shape: the lead/values/rest shape
    /// (`the_rest_binds`), the pieces shape (`binds`), the queue's parts
    /// (`…Binds`), and Render Settings' parts on an event (`binds${at}`).
    #[test]
    fn every_page_that_joins_words_honours_the_bind_mark() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut pages = Vec::new();
        every_file_under(&root.join("web"), "js", &mut pages);
        let mut faults = Vec::new();
        let mut shapes = 0;
        for (name, text) in &pages {
            let mut owes = |shape: &str, mark: &str| {
                shapes += 1;
                if !text.contains(mark) || !text.contains("BETWEEN_A_FIGURE_AND_ITS_UNIT = \"\\u00A0\"") {
                    faults.push(format!("{name} joins by {shape} and does not honour {mark}"));
                }
            };
            if text.contains(".the_rest_joins") {
                owes("the_rest_joins", ".the_rest_binds");
            }
            if text.contains("piece.joins") {
                owes("piece.joins", "piece.binds");
            }
            if text.contains("base + \"Trail\"") {
                owes("base + \"Trail\"", "base + \"Binds\"");
            }
            if text.contains("`joins${at}`") {
                owes("`joins${at}`", "`binds${at}`");
            }
        }
        assert!(shapes >= 9, "the four shapes were found on the pages: {shapes}");
        assert!(faults.is_empty(), "{faults:#?}");
    }

    #[test]
    fn the_verbatim_route_substitutes_nothing() {
        let path = "/home/artist/scene \u{2014} final/frame 001.exr";
        assert_eq!(Verbatim::new(path).as_str(), path);
        let words = "This server's driver can't";
        assert_eq!(Verbatim::new(words).as_str(), words);
    }

    /// **The third substitution: one apostrophe on every screen.** A typed
    /// one after a letter or a figure becomes U+2019, a typographic one stays,
    /// and a quotation mark opened after a space is left alone.
    #[test]
    fn a_typed_apostrophe_becomes_the_typographic_one() {
        assert_eq!(point("Can't"), "Can\u{2019}t");
        assert_eq!(point("This server's driver"), "This server\u{2019}s\u{00A0}driver");
        assert_eq!(point("the servers' jobs"), "the servers\u{2019}\u{00A0}jobs");
        assert_eq!(point("This server\u{2019}s driver"), point("This server's driver"));
        assert_eq!(point("say 'yes'"), "say\u{00A0}'yes\u{2019}");
        assert_eq!(point("'quoted"), "'quoted");
    }
}

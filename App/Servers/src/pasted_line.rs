//! The provider's connection line, pasted into the Add dialog's host field.
//! `Servers` §13.3, PL1 to PL3.
//!
//! **The line is read as text and never run** (PL1). Nothing here opens a
//! connection, starts a process, reads a file or sends anything anywhere: it
//! takes a string and gives back what the line says, and the Add dialog fills
//! its fields from that and runs its own checks on them as on typed values.
//!
//! **What it takes is the destination and the port** (PL2), read the way the
//! `ssh(1)` manual on the desk Mac writes them (the companion's 10.9): the
//! destination is `[user@]hostname` or `ssh://[user@]hostname[:port]`, the port
//! is the port option's value, and the login option names the username too.
//! Everything else in the line is dropped, a key file above all: it names a file
//! on the user's computer that Smoozy has not been given.
//!
//! **One form of the line fills nothing** (PL3): Runpod's proxied line, which
//! its own page says *"does not support commands like SCP (Secure Copy
//! Protocol) or SFTP (SSH File Transfer Protocol)"*, told apart by its host
//! (the companion's 10.8).

use crate::identity::Port;

/// **Which options take a value**, from the manual's own synopsis (PL2):
/// `[-B bind_interface] [-b bind_address] [-c cipher_spec] [-D …] [-E log_file]
/// [-e escape_char] [-F configfile] [-I pkcs11] [-i identity_file]
/// [-J destination] [-L address] [-l login_name] [-m mac_spec] [-O ctl_cmd]
/// [-o option] [-P tag] [-p port] [-R address] [-S ctl_path] [-W host:port]
/// [-w local_tun[:remote_tun]]`. **The value is read so that it is never
/// mistaken for the destination**, which is the only reason most of these are
/// listed at all.
const THE_OPTIONS_THAT_TAKE_A_VALUE: &str = "BbcDEeFIiJLlmOoPpRSWw";

/// **The host of Runpod's proxied line**, the only mark the provider's own
/// words give for telling that form apart (PL3, the companion's 10.8).
pub const RUNPODS_PROXY_HOST: &str = "ssh.runpod.io";

/// What a pasted connection line gives the Add dialog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheLineGives {
    /// **The three fields, and what the line lacks.** The port is always
    /// there: a line with no port means the standard one, 22, because that is
    /// what the line means (PL2, C4). The port is the text the line holds,
    /// so a port the dialog will not take is refused there, as a typed one is.
    TheFields {
        host: Option<String>,
        port: String,
        username: Option<String>,
    },
    /// **Runpod's proxied line, which cannot carry files.** It fills no field
    /// (PL3).
    RunpodsProxiedLine,
}

/// **Whether a pasted text is a connection line**: it begins with the word
/// `ssh` and a space (PL1). Space before it is not counted, since a line copied
/// from a provider's page can carry some. Anything else is a host, as before.
pub fn is_a_connection_line(pasted: &str) -> bool {
    let pasted = pasted.trim_start();
    pasted.len() > 3
        && pasted.starts_with("ssh")
        && pasted[3..].starts_with(|c: char| c.is_whitespace())
}

/// **What the line says**, or nothing where the text is not a connection line.
pub fn the_line_read(pasted: &str) -> Option<WhatTheLineGives> {
    if !is_a_connection_line(pasted) {
        return None;
    }
    let words = the_words(&pasted.trim_start()[3..]);

    // **An option is read wherever it stands**, before or after the
    // destination: Runpod's own documented line puts its port after it. The
    // first word that is neither an option nor an option's value is the
    // destination; every later one is a command, and dropped.
    let mut destination: Option<&str> = None;
    let mut the_port_option: Option<String> = None;
    let mut the_login_option: Option<String> = None;
    let mut at = 0;
    while at < words.len() {
        let word = words[at].as_str();
        if let Some(letters) = word.strip_prefix('-').filter(|letters| !letters.is_empty()) {
            for (from, letter) in letters.char_indices() {
                if !THE_OPTIONS_THAT_TAKE_A_VALUE.contains(letter) {
                    continue;
                }
                // **A value joined to its letter is that option's value**;
                // otherwise it is the next word.
                let joined = &letters[from + letter.len_utf8()..];
                let value = if joined.is_empty() {
                    at += 1;
                    words.get(at).cloned()
                } else {
                    Some(joined.to_string())
                };
                match letter {
                    'p' if the_port_option.is_none() => the_port_option = value,
                    'l' if the_login_option.is_none() => the_login_option = value,
                    _ => {}
                }
                break;
            }
        } else if destination.is_none() {
            destination = Some(word);
        }
        at += 1;
    }

    let (username, host, port) = destination.map(the_destination).unwrap_or_default();
    if host
        .as_deref()
        .is_some_and(|host| host.eq_ignore_ascii_case(RUNPODS_PROXY_HOST))
    {
        return Some(WhatTheLineGives::RunpodsProxiedLine);
    }
    // **The destination's own username or port wins over an option's** —
    // the manual does not say, so this is Smoozy's decision (PL2).
    Some(WhatTheLineGives::TheFields {
        host,
        port: port
            .or(the_port_option)
            .filter(|port| !port.is_empty())
            .unwrap_or_else(|| Port::STANDARD_SSH.number().to_string()),
        username: username.or(the_login_option).filter(|name| !name.is_empty()),
    })
}

/// **The destination, as the manual writes it**: `[user@]hostname`, or
/// `ssh://[user@]hostname[:port]`. A part that is empty is not there.
fn the_destination(destination: &str) -> (Option<String>, Option<String>, Option<String>) {
    let (rest, a_uri) = match destination.get(..6) {
        Some(scheme) if scheme.eq_ignore_ascii_case("ssh://") => (&destination[6..], true),
        _ => (destination, false),
    };
    let rest = if a_uri { rest.trim_end_matches('/') } else { rest };
    let (username, address) = match rest.rfind('@') {
        Some(at) => (Some(&rest[..at]), &rest[at + 1..]),
        None => (None, rest),
    };
    let (host, port) = if !a_uri {
        (address, None)
    } else if let Some(bracketed) = address.strip_prefix('[') {
        // A numeric address of the newer kind sits in brackets, so its own
        // colons are not the port's.
        match bracketed.split_once(']') {
            Some((host, after)) => (host, after.strip_prefix(':')),
            None => (address, None),
        }
    } else {
        match address.split_once(':') {
            Some((host, port)) if !port.contains(':') => (host, Some(port)),
            _ => (address, None),
        }
    };
    let some = |part: Option<&str>| part.filter(|part| !part.is_empty()).map(str::to_string);
    (some(username), some(Some(host)), some(port))
}

/// **The line's words**, split on space. Text in quotes is one word, without
/// its quotes, so a key file whose path holds a space is not taken for the
/// destination.
fn the_words(line: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut in_a_word = false;
    let mut quoted_by: Option<char> = None;
    for character in line.chars() {
        match quoted_by {
            Some(quote) if character == quote => quoted_by = None,
            Some(_) => word.push(character),
            None if character == '"' || character == '\'' => {
                quoted_by = Some(character);
                in_a_word = true;
            }
            None if character.is_whitespace() => {
                if in_a_word {
                    words.push(std::mem::take(&mut word));
                    in_a_word = false;
                }
            }
            None => {
                word.push(character);
                in_a_word = true;
            }
        }
    }
    if in_a_word {
        words.push(word);
    }
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fields(host: Option<&str>, port: &str, username: Option<&str>) -> Option<WhatTheLineGives> {
        Some(WhatTheLineGives::TheFields {
            host: host.map(str::to_string),
            port: port.to_string(),
            username: username.map(str::to_string),
        })
    }

    /// **The map's four lines, piece 15.**
    #[test]
    fn the_destination_and_the_port_are_taken_and_nothing_else() {
        assert_eq!(the_line_read("ssh -p 1239 ubuntu@1.2.3.4"), fields(Some("1.2.3.4"), "1239", Some("ubuntu")));
        assert_eq!(the_line_read("ssh root@host"), fields(Some("host"), "22", Some("root")));
        assert_eq!(
            the_line_read("ssh -i ~/.ssh/k -L 8080:localhost:8080 -p 2 u@h"),
            fields(Some("h"), "2", Some("u"))
        );
        assert_eq!(the_line_read("ssh host"), fields(Some("host"), "22", None));
    }

    /// **Each provider's own line, as its page of 2026-10-07 prints it** (the
    /// companion's 10.8). Hyperstack's example address is hidden on its page,
    /// so a documentation address stands in for it.
    #[test]
    fn each_providers_own_line_fills_the_fields() {
        // Vast: the port first, and a forwarded port after the destination.
        assert_eq!(
            the_line_read("ssh -p 50983 root@154.64.230.50 -L 8080:localhost:8080"),
            fields(Some("154.64.230.50"), "50983", Some("root"))
        );
        // Runpod's direct line: the port after the destination.
        assert_eq!(
            the_line_read("ssh root@213.173.108.12 -p 17445 -i ~/.ssh/id_ed25519"),
            fields(Some("213.173.108.12"), "17445", Some("root"))
        );
        // Hyperstack: a key file and no port.
        assert_eq!(
            the_line_read("ssh -i ~/Downloads/my-key.txt ubuntu@203.0.113.7"),
            fields(Some("203.0.113.7"), "22", Some("ubuntu"))
        );
        // Sesterce: the destination alone.
        assert_eq!(
            the_line_read("ssh sesterce@216.81.248.60"),
            fields(Some("216.81.248.60"), "22", Some("sesterce"))
        );
    }

    /// **Runpod's proxied line fills nothing** (PL3), whatever the letters'
    /// case.
    #[test]
    fn runpods_proxied_line_fills_nothing() {
        assert_eq!(
            the_line_read("ssh 8y5rumuyb50m78-6441103b@ssh.runpod.io -i ~/.ssh/id_ed25519"),
            Some(WhatTheLineGives::RunpodsProxiedLine)
        );
        assert_eq!(the_line_read("ssh pod@SSH.RunPod.io"), Some(WhatTheLineGives::RunpodsProxiedLine));
        // A host that only contains the name is not that host.
        assert_eq!(
            the_line_read("ssh u@ssh.runpod.io.example"),
            fields(Some("ssh.runpod.io.example"), "22", Some("u"))
        );
    }

    /// **A line missing a piece fills what it has, and says what it lacks**
    /// by leaving it out; nothing is guessed.
    #[test]
    fn a_missing_piece_is_left_out_and_never_guessed() {
        assert_eq!(the_line_read("ssh -p 22"), fields(None, "22", None));
        assert_eq!(the_line_read("ssh "), fields(None, "22", None));
        assert_eq!(the_line_read("ssh @host"), fields(Some("host"), "22", None));
        assert_eq!(the_line_read("ssh u@"), fields(None, "22", Some("u")));
        // A port option with nothing after it is no port.
        assert_eq!(the_line_read("ssh u@h -p"), fields(Some("h"), "22", Some("u")));
    }

    /// **What the manual writes and the providers do not** (the companion's
    /// 10.9), and the decisions PL2 takes where it is silent.
    #[test]
    fn the_manuals_other_forms_are_read() {
        assert_eq!(the_line_read("ssh -l ubuntu -p 41966 h"), fields(Some("h"), "41966", Some("ubuntu")));
        assert_eq!(the_line_read("ssh ssh://u@h:2222"), fields(Some("h"), "2222", Some("u")));
        assert_eq!(the_line_read("ssh ssh://[2001:db8::1]:2222"), fields(Some("2001:db8::1"), "2222", None));
        assert_eq!(the_line_read("ssh -p2222 u@h"), fields(Some("h"), "2222", Some("u")));
        // Flags that take no value, grouped, and one that does at the end.
        assert_eq!(the_line_read("ssh -4Ap 2200 u@h"), fields(Some("h"), "2200", Some("u")));
        // The destination's own wins over an option's.
        assert_eq!(the_line_read("ssh -l other -p 1 ssh://u@h:2"), fields(Some("h"), "2", Some("u")));
        // A quoted key file with a space is one word.
        assert_eq!(the_line_read("ssh -i \"/my keys/k\" u@h"), fields(Some("h"), "22", Some("u")));
        // A command after the destination is dropped.
        assert_eq!(the_line_read("ssh u@h uptime"), fields(Some("h"), "22", Some("u")));
        // A port the dialog will not take is carried as written, for the
        // dialog to refuse.
        assert_eq!(the_line_read("ssh -p 70000 u@h"), fields(Some("h"), "70000", Some("u")));
    }

    /// **Only a text that begins with the word and a space is a line** (PL1);
    /// anything else is a host.
    #[test]
    fn only_ssh_and_a_space_is_a_line() {
        assert!(is_a_connection_line("ssh -p 1 u@h"));
        assert!(is_a_connection_line("  ssh\tu@h\n"));
        for host in ["1.2.3.4", "sshd.example", "ssh", "ssh-host", "Ssh u@h", ""] {
            assert!(!is_a_connection_line(host), "{host:?}");
            assert_eq!(the_line_read(host), None, "{host:?}");
        }
    }
}

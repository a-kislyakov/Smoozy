//! §2.8's privileged web-content boundary, in the two places the selected
//! shell puts it: a predicate the window construction installs, and an absence
//! in every capability file this application ships.
//!
//! **Why this file exists at all, and it is the whole of the reason.** H31
//! states two refusals — a navigation away from the packaged origin is
//! cancelled before it commits, and `window.open`, remote frames and
//! executable content fetched at run time are refused. On the shell that did
//! not win, the first of those was a configuration entry left unset. **On the
//! shell that was chosen it is neither an entry nor a default**: H31a reads
//! that project's own configuration schema and finds the security block has
//! exactly eight members and no navigation allow-list among them, and that
//! what the project publishes instead is a pair of closures on the webview
//! builder. **So a build that sets the configuration and installs neither
//! closure keeps neither refusal, and nothing in the configuration would show
//! it.** H31a says that in its own words. This file is the closures' contents
//! and the test that says they are installed.
//!
//! **What the shell does give by default, which is the other half of H31a and
//! is why the binding surface is not also written here.** *"By default the API
//! is only accessible to bundled code shipped with the Tauri App"*, and a
//! capability's `remote` field *"defaults to not being set"*. Those are
//! defaults rather than closures, so what the code owes them is an absence:
//! **no capability this application ships carries a `remote` field**, which is
//! H31a's requirement in the place the other shell's *left unset in
//! production* used to sit. The test
//! `no_capability_this_application_ships_is_open_to_a_remote_origin` below
//! holds it.
//!
//! **What is not built here, said so it is not read as kept.** H31's last
//! sentence — an approved external HTTPS address is handed to the operating
//! system's browser — is a facility with no caller: nothing in this
//! application offers an external address today. **The refusal half is what
//! this file keeps**, and it is the half that fails open if it is missing. The
//! hand-off arrives with the surface that needs one.
//!
//! **H36 is the reason the predicate refuses rather than repairs.** A
//! disallowed navigation is cancelled and no application service runs for it;
//! there is no permissive fallback and nothing here has one.

use tauri::Url;

/// **The packaged origin, as this application actually configures it.**
///
/// The shell serves the shipped page from its own asset protocol, and the
/// origin that protocol takes differs on Windows for a reason that is the
/// shell's rather than Smoozy's: on Windows a custom protocol is served as a
/// host under `localhost` instead of as a scheme of its own.
///
/// **The scheme half on Windows is a builder option this crate does not set,
/// and its published default is `http`** — *"Sets whether the custom protocols
/// should use `https://<scheme>.localhost` instead of the default
/// `http://<scheme>.localhost` on Windows and Android. Defaults to `false`."*
/// So the origin is exactly one, not two, and admitting the other would be
/// admitting an origin nothing can serve. **The default is held by a test**
/// rather than trusted, in the shape this crate already uses for a published
/// default it relies on: a version bump that flipped it would move the
/// packaged origin with nothing here changing.
///
/// `windows` is a parameter rather than a `cfg!` inside the body so that the
/// tests below run both shapes on whichever system they are run on. **A
/// predicate whose Windows half is only ever compiled on Windows is a
/// predicate nobody at this desk has tested.**
fn is_the_packaged_origin(url: &Url, windows: bool) -> bool {
    if windows {
        url.scheme() == "http" && url.host_str() == Some("tauri.localhost")
    } else {
        url.scheme() == "tauri" && url.host_str() == Some("localhost")
    }
}

/// **H31's first refusal: a top-level or child-frame navigation away from the
/// packaged origin is cancelled before it commits.**
///
/// This is the closure the window construction installs on the shell's
/// `on_navigation`, whose own words are *"Returning false cancels the
/// navigation."* — H31a, which names it.
///
/// **It admits the packaged origin and nothing else.** Not an external HTTPS
/// address, not a file, not a data or blob document, not a script URL, and not
/// a host that merely ends in the packaged one. **H36:** the boundary fails
/// closed, so anything this cannot recognise is refused rather than allowed.
pub fn the_navigation_is_allowed(url: &Url) -> bool {
    is_the_packaged_origin(url, cfg!(windows))
}

/// **H31's second refusal: `window.open` is refused.**
///
/// This is the closure the window construction installs on the shell's
/// `on_new_window` — *"Set a new window request handler to decide if incoming
/// url is allowed to be opened. A new window is requested to be opened by the
/// window.open API."* (H31a).
///
/// **It is a flat no and takes no argument on purpose.** H31 refuses
/// `window.open` outright rather than for particular addresses, and a
/// predicate that read the address would invite a later reader to widen it for
/// one. The route for an approved external address is the operating system's
/// browser, which is not this closure and is not built yet.
pub fn a_new_window_is_ever_opened() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::Path;

    /// The window construction, read as text.
    ///
    /// **The binary's own source, because that is where the window is built.**
    /// `build_window` composes the builder there for the reason written at it:
    /// the window differs by system and the shell's per-system configuration
    /// file replaces the window list rather than merging into it.
    const THE_WINDOW_CONSTRUCTION: &str = include_str!("main.rs");

    fn url(text: &str) -> Url {
        Url::parse(text).expect("the test address parses")
    }

    /// **The packaged origin is admitted, on both shapes.**
    #[test]
    fn the_packaged_page_may_navigate_within_itself() {
        assert!(is_the_packaged_origin(&url("tauri://localhost/"), false));
        assert!(is_the_packaged_origin(
            &url("tauri://localhost/index.html"),
            false
        ));
        assert!(is_the_packaged_origin(
            &url("http://tauri.localhost/"),
            true
        ));
        assert!(is_the_packaged_origin(
            &url("http://tauri.localhost/index.html"),
            true
        ));
    }

    /// **Everything else is refused, and the list is written out rather than
    /// summarised** — H31 names remote frames, run-time executable content and
    /// `window.open`, and H33's policy names the rest.
    ///
    /// **Four of these are the ones a wider predicate lets through**, and they
    /// are why the check is an equality on scheme and host rather than a
    /// suffix or a contains: `tauri.localhost.example.com` ends in nothing the
    /// check looks at, `evil.tauri.localhost` is a different host, the
    /// Windows shape refused on a Unix build is a different scheme, and the
    /// `https` form is an origin this application does not serve.
    #[test]
    fn nothing_but_the_packaged_origin_is_admitted() {
        let refused_everywhere = [
            "https://example.com/",
            "http://example.com/",
            "https://tauri.localhost.example.com/",
            "https://evil.tauri.localhost/",
            "http://evil.tauri.localhost/",
            "file:///etc/passwd",
            "data:text/html,<script>1</script>",
            "blob:tauri://localhost/0000",
            "javascript:alert(1)",
            "about:blank",
            "http://localhost:1420/",
            "http://127.0.0.1/",
            "ipc://localhost/",
        ];
        for address in refused_everywhere {
            let at = url(address);
            assert!(
                !is_the_packaged_origin(&at, false),
                "{address} was admitted on the tauri-scheme shape"
            );
            assert!(
                !is_the_packaged_origin(&at, true),
                "{address} was admitted on the Windows shape"
            );
        }

        // Each shape refuses the other's origin, which is what says the check
        // is an equality rather than a family.
        assert!(!is_the_packaged_origin(
            &url("http://tauri.localhost/"),
            false
        ));
        assert!(!is_the_packaged_origin(&url("tauri://localhost/"), true));

        // The `https` form of the Windows origin is refused, because the
        // builder option that would produce it is not set — see the test
        // below, which is what holds that.
        assert!(!is_the_packaged_origin(
            &url("https://tauri.localhost/"),
            true
        ));
    }

    /// **The published default the Windows half of the origin rests on.**
    ///
    /// This crate never calls the builder's `use_https_scheme`, so the Windows
    /// packaged origin is the `http` form. **A release of the shell that
    /// flipped that default would move the origin with nothing in this crate
    /// changing, and the predicate would then refuse the page's own address
    /// — the window would open on a document it cancelled.** This is the line
    /// that fails first.
    #[test]
    fn the_shells_own_default_still_puts_the_windows_origin_on_http() {
        assert!(
            !tauri::utils::config::WindowConfig::default().use_https_scheme,
            "the shell's default for the custom-protocol scheme on Windows is no \
             longer http, so the packaged origin has moved and the navigation \
             predicate refuses the packaged page itself — Desktop app and \
             delivery H31 and H31a"
        );
    }

    /// **`window.open` is refused, flatly.**
    #[test]
    fn no_new_window_is_ever_opened() {
        assert!(!a_new_window_is_ever_opened());
    }

    /// **The test with the teeth: both closures are installed at the window
    /// construction.**
    ///
    /// **Why it reads the source rather than the built window.** The shell
    /// exposes no way to ask a built window whether a navigation handler was
    /// installed — the closures go into the builder and are never readable
    /// again — so there is nothing to assert against at run time. **What can
    /// be established is that the one place this application builds its window
    /// installs both, and names the predicates above rather than a fresh
    /// closure written beside them.** A predicate widened in this file is
    /// caught by the two tests above; a predicate never installed is caught
    /// here, and neither test can catch the other's failure.
    ///
    /// **It is scoped to `build_window`'s own body** rather than to the file,
    /// so a mention in a comment somewhere else in `main.rs` does not pass it.
    #[test]
    fn the_window_construction_installs_both_refusals() {
        let opens = THE_WINDOW_CONSTRUCTION
            .find("fn build_window")
            .expect("the window is built by `build_window` in `main.rs`");
        let rest = &THE_WINDOW_CONSTRUCTION[opens..];
        let closes = rest
            .find("\nfn ")
            .expect("`build_window` is followed by another item");
        let body = &rest[..closes];

        assert!(
            body.contains(".on_navigation(")
                && body.contains("web_content_boundary::the_navigation_is_allowed"),
            "the window is built without the navigation refusal H31 states, or \
             without the predicate this file holds: on the selected shell that \
             refusal is a closure at window construction and not a configuration \
             entry, so nothing in the configuration would show its absence — \
             Desktop app and delivery H31 and H31a"
        );

        assert!(
            body.contains(".on_new_window(")
                && body.contains("web_content_boundary::a_new_window_is_ever_opened"),
            "the window is built without the `window.open` refusal H31 states, or \
             without the predicate this file holds: on the selected shell that \
             refusal is a closure at window construction and not a configuration \
             entry — Desktop app and delivery H31 and H31a"
        );
    }

    /// **H31a's other half: no capability this application ships is open to a
    /// remote origin.**
    ///
    /// The shell's own schema says a capability's `remote` field *"is optional
    /// and defaults to not being set"* and that `local` *"Defaults to
    /// `true`"*, so what the code owes is an absence, and an absence is
    /// exactly what reading the file's text can establish: a capability file
    /// that does not contain the word cannot carry the field.
    ///
    /// **The shape is per capability rather than one list for the whole
    /// surface**, which H31a records as differing in Smoozy's favour — a
    /// remote grant, were one ever written, would reach only what that one
    /// capability lists. That is why this walks every file in the folder
    /// rather than one.
    #[test]
    fn no_capability_this_application_ships_is_open_to_a_remote_origin() {
        let folder = Path::new(env!("CARGO_MANIFEST_DIR")).join("capabilities");
        let mut read = 0;
        for entry in fs::read_dir(&folder).expect("the capabilities folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let text = fs::read_to_string(&path).expect("the capability is readable");
            let name = path.display();
            assert!(
                !text.contains("remote"),
                "{name} names a remote origin: on this shell that is the whole of \
                 the allow-list the other shell put in one option, and H31 asks \
                 that the packaged origin stand alone — Desktop app and delivery \
                 H31a"
            );
            assert!(
                !text.contains("\"local\""),
                "{name} sets `local`, whose published default is true: the only \
                 change worth making to it is turning the packaged origin off, \
                 which H31 forbids — Desktop app and delivery H31a"
            );
            read += 1;
        }
        assert!(
            read > 0,
            "no capability file was read at all, so this test proved nothing"
        );
    }
}

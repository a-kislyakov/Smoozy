//! **The one ordered queue's own end of the page** — the shell's side of
//! §3.3 P6.
//!
//! [`crate::events`] is the queue and holds all of its rules. This file is the
//! two lines that join it to the window: **the one place a delivery is handed
//! to the shell**, and **the one command the page uses to say it is
//! listening**.
//!
//! **Why the page has to say so.** The spine runs at application start, before
//! the window is built — L3a and L6 both say *at application start* — so the
//! first events of a run are posted with nothing to receive them. They wait on
//! the queue. The page's own first act is this command, and everything waiting
//! is delivered then, in order.
//!
//! **H34: the command has a closed request and response type, and both are
//! empty.** It takes nothing from the page and returns nothing to it, so there
//! is no length and no enumeration to validate, and nothing beneath the
//! application-service surface is anywhere near it.

use std::sync::Arc;

use tauri::{AppHandle, Emitter, Runtime, State};

use crate::events::{Delivers, Delivery, TheOneQueue, THE_ONE_EVENT_NAME};

/// **Where a delivery goes: the one window, on the one event name.**
///
/// P6: one queue and one order, so one name. The shell's own emit is the last
/// step of the route and the only thing in this crate that names it.
struct ToTheOneWindow<R: Runtime> {
    app: AppHandle<R>,
}

impl<R: Runtime> Delivers for ToTheOneWindow<R> {
    fn deliver(&self, delivery: &Delivery) {
        // **The answer is deliberately dropped.** An event is volatile and
        // run-held (P8) and nothing durable is written from one, so a delivery
        // the shell would not take is an event nobody sees and there is nothing
        // owed by it. **A retry here would be this route inventing a
        // reliability nothing promised**, and a failure record about it would
        // be a failure record about the interface's own plumbing.
        let _ = self.app.emit(THE_ONE_EVENT_NAME, delivery);
    }
}

/// **The page is listening.** Everything posted before the window existed is
/// delivered now, in the one order.
///
/// A page that loads again says so again and receives what has been posted
/// since; nothing is replayed, which is P8's *volatile and run-held* holding
/// in the one place somebody would reach for a replay.
#[tauri::command]
pub fn the_page_is_ready<R: Runtime>(app: AppHandle<R>, queue: State<'_, Arc<TheOneQueue>>) {
    queue.the_page_is_ready(Box::new(ToTheOneWindow { app }));
}

#[cfg(test)]
mod tests {
    /// **One event name, and it is the queue's own.**
    ///
    /// A second name would be a second stream, and two streams reaching one
    /// page are two orders however carefully each is kept — which is exactly
    /// what P6 refuses. **This file is the only place in the crate that names
    /// the shell's emit**, and this is the check that it names the one
    /// constant rather than a string of its own.
    #[test]
    fn the_only_emit_in_this_crate_is_on_the_one_event_name() {
        let mut emitters = Vec::new();
        walk(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
            &mut emitters,
        );
        assert_eq!(
            emitters,
            vec!["the_event_route.rs".to_string()],
            "something other than the event route hands the page an event. \
             **P6: results reach the interface as events on one ordered queue, \
             and the interface holds no other route to a module** — a second \
             emitter is a second order, whichever name it uses"
        );

        let source = include_str!("the_event_route.rs");
        let (working, _) = source
            .split_once("#[cfg(test)]")
            .expect("the tests are the last thing in this file");
        assert_eq!(working.matches(".emit(").count(), 1);
        assert!(
            working.contains("emit(THE_ONE_EVENT_NAME"),
            "the emit no longer uses the queue's own event name"
        );
    }

    /// **The page has one listener, on the one event name, and it is the
    /// route's** — P6 on the far side of the boundary.
    ///
    /// A second `listen` anywhere in `web/` is a second stream, and two streams
    /// reaching one page are two orders. **A piece reaching for one is the
    /// likeliest way P6 gets broken**, because it is the obvious thing to do
    /// and nothing about it looks wrong.
    #[test]
    fn the_page_has_one_listener_and_it_is_on_the_one_event_name() {
        let mut listeners = Vec::new();
        let folder = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for entry in std::fs::read_dir(&folder).expect("the page folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.extension().and_then(|e| e.to_str()) != Some("js") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("the file is readable");
            if text.contains("event.listen(") || text.contains("event.once(") {
                listeners.push(
                    path.file_name()
                        .and_then(|s| s.to_str())
                        .expect("the file has a name")
                        .to_string(),
                );
            }
        }
        assert_eq!(
            listeners,
            vec!["event-route.js".to_string()],
            "something on the page listens for events besides the route. **P6: \
             one ordered queue, and the interface holds no other route to a \
             module** — a piece says what it wants to be told about with the \
             route's own `whenTold` and never listens for itself"
        );

        let route = std::fs::read_to_string(folder.join("event-route.js"))
            .expect("the route is readable");
        assert!(
            route.contains(&format!(
                "const THE_ONE_EVENT_NAME = \"{}\";",
                crate::events::THE_ONE_EVENT_NAME
            )),
            "the page's event name and the application's have drifted apart. \
             **They are the two ends of one queue** and nothing would report the \
             difference: the application would post, the page would listen, and \
             not one event would ever arrive"
        );
    }

    /// **Every event kind the page names is one the application declares** —
    /// F-12 of step 10a round 1.
    ///
    /// **The same fact was kept in two places with nothing checking it.**
    /// `src/start.rs`, `src/surface/menus.rs`, `src/surface/said_once.rs`,
    /// `src/surface/the_failure_shape.rs`, `src/surface/the_underway_shape.rs`
    /// and `src/surface/the_machines_own_words.rs` each declare a kind; six page
    /// modules declare the same ones again. **The same discipline is applied to
    /// the event's own name, just above, and to the five narrowing boundaries in
    /// `crate::surface::panes`, and each of those has a test that reads the page
    /// file and fails on drift. The kinds had none.**
    ///
    /// **The failure is completely silent.** Somebody renames one in the Rust
    /// half while adding the first screen, everything compiles, every test
    /// passes, and choosing a menu item stops doing anything at all with nothing
    /// saying why — the application posts, the page listens for the old word,
    /// and not one event arrives.
    ///
    /// **The check is in the direction the drift runs.** A kind named on the
    /// page must be one the application declares; the other way round can have
    /// a legitimate absence. *(This named `notifications:a-going` as one, until
    /// 2026-11-01, when `web/a-notification-clicked.js` began listening for it
    /// — the Mac watch's first finding.)*
    #[test]
    fn every_event_kind_the_page_names_is_one_the_application_declares() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut declared = Vec::new();
        the_kinds_in_the_rust_half(&root.join("src"), &mut declared);
        assert!(
            declared.len() > 5,
            "hardly any event kinds were found in the application half, so this \
             check is reading nothing and would pass over anything"
        );

        let folder = root.join("web");
        let mut read = 0;
        for entry in std::fs::read_dir(&folder).expect("the page folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.extension().and_then(|e| e.to_str()) != Some("js") {
                continue;
            }
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .expect("the file has a name")
                .to_string();
            let text = std::fs::read_to_string(&path).expect("the file is readable");
            for line in text.lines() {
                let line = line.trim();
                if !line.starts_with("const ") || line.starts_with("//") {
                    continue;
                }
                let Some(named) = the_string_declared_on(line) else {
                    continue;
                };
                if !has_an_event_kinds_shape(&named) {
                    continue;
                }
                read += 1;
                assert!(
                    declared.contains(&named),
                    "web/{name} names the event kind `{named}` and no file under \
                     `src/` declares it. **The two halves are the two ends of one \
                     queue** and nothing else would report the difference: the \
                     application posts, the page listens, and not one event ever \
                     arrives — the menu item simply stops doing anything, in \
                     silence"
                );
            }
        }
        assert!(
            read > 5,
            "hardly any event kinds were found in the page half, so this check \
             read nothing"
        );
    }

    /// Every kind `src/` declares, by its value.
    fn the_kinds_in_the_rust_half(folder: &std::path::Path, found: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(folder) else {
            return;
        };
        let mut paths: Vec<_> = entries
            .map(|entry| entry.expect("the entry is readable").path())
            .collect();
        paths.sort();
        for path in paths {
            if path.is_dir() {
                the_kinds_in_the_rust_half(&path, found);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("the file is readable");
            for line in text.lines() {
                let line = line.trim();
                if !line.starts_with("pub const ") || !line.contains(": &str = ") {
                    continue;
                }
                if let Some(named) = the_string_declared_on(line) {
                    if has_an_event_kinds_shape(&named) {
                        found.push(named);
                    }
                }
            }
        }
    }

    /// **What an event kind looks like: two lower-case words with a colon
    /// between them**, each made of letters and hyphens — `spine:the-run-began`,
    /// `menus:a-going`. The service's own name, then what it is saying.
    ///
    /// **It is deliberately tighter than *holds a colon***, which also matches
    /// an address and would put this check on strings that are not kinds at all.
    fn has_an_event_kinds_shape(named: &str) -> bool {
        let mut halves = named.split(':');
        let (Some(service), Some(saying), None) = (halves.next(), halves.next(), halves.next())
        else {
            return false;
        };
        let a_word = |word: &str| {
            !word.is_empty()
                && word
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c == '-')
        };
        a_word(service) && a_word(saying)
    }

    /// The one double-quoted string a declaration line ends with.
    fn the_string_declared_on(line: &str) -> Option<String> {
        let opens = line.find('"')?;
        let rest = &line[opens + 1..];
        let closes = rest.find('"')?;
        Some(rest[..closes].to_string())
    }

    /// **P9 and P10 on the page: nothing there polls and nothing repeats on a
    /// timer.**
    ///
    /// P10: there is exactly one standing timer on this side and it is the
    /// update check, which is not built. **P10b admits a drawing that ticks
    /// against a moment the application already published and admits nothing
    /// else** — so when the first such drawing arrives, this test is the place
    /// its exemption is written down, named and reasoned, rather than the check
    /// being quietly deleted. `web/smoozy.css` has carried the same prohibition
    /// as a comment since step 1.
    ///
    /// # The exemptions, written down here as this test asked for
    ///
    /// **`web/the-underway-shape.js` holds the repairing server's countdown**,
    /// which is **M30**: *"The countdown is a drawing and is bounded by P10b.
    /// It ticks against M28's moment, takes no reading, makes no request, posts
    /// nothing on P6's queue, and ceases to exist when the thing it draws is
    /// off screen."*
    ///
    /// **Each admitted file names its own clauses**, and the exemption is
    /// narrowed to exactly what P10b admits:
    ///
    /// - **`setInterval` only, and it must be cleared.** A file claiming the
    ///   exemption without a `clearInterval` has a timer that cannot cease to
    ///   exist, which is the half of P10b that stops it being a standing timer.
    /// - **`setTimeout` and `requestAnimationFrame` stay forbidden
    ///   everywhere**, this file included. Neither is a tick against a
    ///   published moment: one fires once with nothing under way, and the other
    ///   is a frame loop.
    /// - **The clause is named in the file.** P10b and M30 both, so the next
    ///   reader finds the reasoning at the code rather than here.
    ///
    /// **Every other file of the page is bound exactly as before**, which is
    /// what makes this an exemption rather than a hole.
    #[test]
    fn nothing_the_page_holds_repeats_on_a_timer() {
        /// The repairing server countdown, bounded by P10b and M30.
        const THE_DRAWINGS_THAT_TICK: [(&str, &[&str]); 1] = [
            ("the-underway-shape.js", &["P10b", "M30"]),
        ];
        let folder = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for entry in std::fs::read_dir(&folder).expect("the page folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.extension().and_then(|e| e.to_str()) != Some("js") {
                continue;
            }
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .expect("the file has a name")
                .to_string();
            let text = std::fs::read_to_string(&path).expect("the file is readable");
            let admitted = THE_DRAWINGS_THAT_TICK
                .iter()
                .find(|(which, _)| *which == name)
                .map(|(_, clauses)| *clauses);
            let ticks = admitted.is_some();
            for repeats in ["setInterval", "requestAnimationFrame", "setTimeout"] {
                if ticks && repeats == "setInterval" {
                    continue;
                }
                assert!(
                    !text.contains(repeats),
                    "web/{name} uses {repeats}. **P9: nothing on this side polls. \
                     P10: there is exactly one timer on this side and it is the \
                     update check.** P10b admits a drawing that ticks against a \
                     moment the application already published and that does \
                     nothing else — if that is what this is, say so here with its \
                     clause rather than removing the check"
                );
            }
            let Some(clauses) = admitted else {
                continue;
            };
            // **The exemption's own three conditions, checked rather than
            // trusted.**
            assert!(
                text.contains("clearInterval"),
                "web/{name} claims P10b's exemption and never clears its \
                 interval. **P10b: a drawing may tick against a moment already \
                 published, and it stops existing when the thing it draws is \
                 not on screen** — a tick that cannot be cleared is a standing \
                 timer, which is the fault P10 names"
            );
            assert!(
                text.contains("isConnected"),
                "web/{name} claims P10b's exemption and never asks whether what \
                 it draws is still on screen. **M30: the countdown ceases to \
                 exist when the thing it draws is off screen**"
            );
            for clause in clauses {
                assert!(
                    text.contains(clause),
                    "web/{name} claims P10b's exemption and does not name \
                     {clause}. **Every piece of code that keeps a clause names \
                     the clause**, and an exemption that names none is one \
                     nobody can check"
                );
            }
        }
    }

    /// Every `.rs` file under `src` whose working half emits to the page.
    fn walk(folder: &std::path::Path, found: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(folder) else {
            return;
        };
        let mut paths: Vec<_> = entries
            .map(|entry| entry.expect("the entry is readable").path())
            .collect();
        paths.sort();
        for path in paths {
            if path.is_dir() {
                walk(&path, found);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("the file is readable");
            let working = text.split("#[cfg(test)]").next().unwrap_or_default();
            if working.contains(".emit(") || working.contains(".emit_to(") {
                found.push(
                    path.file_name()
                        .and_then(|s| s.to_str())
                        .expect("the file has a name")
                        .to_string(),
                );
            }
        }
    }
}

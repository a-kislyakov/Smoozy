//! **The Windows third** — `Desktop app and delivery` §13, T3, T4, T6, T16's
//! local-file picture, T24, T24a and T28, and §19 E3's Start-menu shortcut.
//!
//! **It is platform code of this document's own, beside the shell**, the same
//! shape `src/reveal.rs` takes and for the same reason: the shell reaches none
//! of it.
//!
//! # The precondition is a shortcut, and the application makes it
//!
//! **T3: a Start-menu shortcut carrying an application identifier, and not the
//! runtime-package route.** Microsoft's words: *"Without a valid shortcut
//! installed in the Start screen or in All Programs, you cannot raise a toast
//! notification from a desktop app."* The recommended alternative depends on a
//! runtime package an unpackaged app must deploy through its installer, and
//! Microsoft's own page says this *"can compromise the simplicity of
//! self-contained deployment"* — **Smoozy is a downloaded binary, so the
//! shortcut route is taken and the runtime package is declined.**
//!
//! **§19 E3 puts the making of it in the application's own code**, at its first
//! successful start: Microsoft's worked route *"composes the full path to the
//! shortcut, determines that a shortcut of that name does not already exist at
//! that location, and passes that information to another method which creates
//! and installs the shortcut"*, under the user's own Start Menu Programs
//! folder, and it says outright *"the shortcut can be deployed per-user or
//! per-app."* **The same page prefers the installer, which is a recommendation
//! and not a precondition.**
//!
//! **T4: an elevated app cannot post at all**, so **nothing in this
//! application ever runs elevated** — and nothing here asks for
//! administrator, the shortcut going into the user's own folder.
//!
//! # Everything below is documentation alone
//!
//! **No Windows desktop edition has ever been reached** (§22 item 2): four
//! Windows machines have been rented and every one reported a Server edition.
//! **Nothing in this file has ever been run**, and the Creator's ruling of
//! 2026-08-31 means no machine is coming. Every claim is Microsoft's own page.
//! **No space is left here for an answer that is not coming.**
//!
//! **And T23 is why that is survivable rather than reckless**: a Windows post
//! **fails silently** by Microsoft's own documentation, so a post is never
//! treated as delivered and no state anywhere depends on one.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use windows::core::{Interface, HSTRING, PCWSTR};
use windows::Data::Xml::Dom::XmlDocument;
use windows::Foundation::TypedEventHandler;
use windows::Win32::Storage::EnhancedStorage::PKEY_AppUserModel_ID;
use windows::UI::Notifications::{
    NotificationSetting, ToastNotification, ToastNotificationManager,
};
use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoTaskMemAlloc, CoTaskMemFree, CoUninitialize, IPersistFile,
    CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
};
use windows::Win32::System::Variant::VT_LPWSTR;
use windows::Win32::UI::Shell::PropertiesSystem::IPropertyStore;
use windows::Win32::UI::Shell::{
    IShellLinkW, SHGetKnownFolderPath, ShellLink, FOLDERID_Programs, KF_FLAG_DEFAULT,
};

use crate::presentation::the_presentation_key::ThePresentationKey;
use super::{APost, TheMachinery, ThePlatform, WhatIsMissing, THE_APPLICATION_IDENTIFIER};
use crate::notifications::permission::WhatTheSystemSays;
use crate::notifications::picture;

/// **The one group every post of Smoozy's sits in.**
///
/// It is not T26's grouping and nothing rests on it: **grouping is documented
/// on macOS and Windows and absent on Linux, and nothing here rests on it.**
/// This value exists because the removal call names a group as well as a tag,
/// and one constant is what stops the post and the take-down disagreeing about
/// which group a post went into.
const THE_ONE_GROUP: &str = "smoozy";

/// **T28: where a click goes.**
static WHAT_A_CLICK_DOES: OnceLock<Box<dyn Fn(String) + Send + Sync>> = OnceLock::new();

/// **The posts this run has made, kept alive so their activation handlers
/// are.**
///
/// **Run-held and identities only** — T29 and T30. A post this run did not make
/// is a post this run cannot route, which is the honest width of the promise.
static THE_LIVE_POSTS: OnceLock<Mutex<HashMap<String, ToastNotification>>> = OnceLock::new();

fn the_live_posts() -> &'static Mutex<HashMap<String, ToastNotification>> {
    THE_LIVE_POSTS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(super) struct TheWindows;

pub(super) fn the_platform() -> Box<dyn ThePlatform> {
    Box::new(TheWindows)
}

pub(super) fn the_activation_route_is_open(on_click: Box<dyn Fn(String) + Send + Sync>) {
    let _ = WHAT_A_CLICK_DOES.set(on_click);
}

/// **§19 E3: the shortcut is made at the application's first successful start,
/// and this is the act that makes it.**
///
/// **Split from the read on 2026-09-12, step 10a round 1 must-do F-10.** The
/// machinery question is published as a read — its own note says it is *"for a
/// caller that wants to read it without posting"* — and on this system the
/// function behind it composed a path under the user's own Start menu and, where
/// nothing was there, **wrote a file**. So asking whether Smoozy could post
/// created something in the user's Start menu. **E3 puts the making of that file
/// at first successful start**, and that is where this is called from; the read
/// beside it now answers honestly on all three systems.
pub(super) fn the_machinery_is_made() {
    let Some(folder) = the_users_own_programs_folder() else {
        return;
    };
    let shortcut = folder.join(THE_SHORTCUTS_NAME);
    if shortcut.exists() {
        return;
    }
    let Ok(us) = std::env::current_exe() else {
        return;
    };
    let _ = make_the_shortcut(&shortcut, &us);
}

impl ThePlatform for TheWindows {
    /// **T3, read and not made** (T1, §19 E3).
    ///
    /// **This is a read and nothing else, from 2026-09-12 and F-10.** It used to
    /// make the shortcut where none was there, so a caller asking whether Smoozy
    /// could post created a file in the user's own Start menu — and the question
    /// is published as a read, *"for a caller that wants to read it without
    /// posting"*. **The making is [`the_machinery_is_made`]**, called where §19
    /// E3 says: at the application's first successful start.
    fn the_machinery(&self) -> TheMachinery {
        match the_start_menu_shortcut_is_there() {
            true => TheMachinery::ItIsInPlace,
            false => TheMachinery::ItIsNot(WhatIsMissing::TheStartMenuShortcut),
        }
    }

    /// **T6: no prompt and no request interface, only a read-only setting
    /// whose every non-enabled value is a way of being switched off.**
    ///
    /// The four ways off are not told apart, there being nothing different to
    /// do about any of them and nothing anywhere that says so to the user
    /// (T8).
    fn the_permission(&self) -> WhatTheSystemSays {
        let identifier = HSTRING::from(THE_APPLICATION_IDENTIFIER);
        let Ok(notifier) = ToastNotificationManager::CreateToastNotifierWithId(&identifier) else {
            return WhatTheSystemSays::TheSystemHasItSwitchedOff;
        };
        match notifier.Setting() {
            Ok(NotificationSetting::Enabled) => WhatTheSystemSays::ItIsGranted,
            _ => WhatTheSystemSays::TheSystemHasItSwitchedOff,
        }
    }

    /// **T6: there is no request interface on this system**, so there is
    /// nothing to do here. The empty body is the clause rather than an
    /// omission — a system with no prompt is not a system Smoozy prompts on.
    fn ask_for_permission_now(&self) {}

    /// **T23: attempted and forgotten**, and on this system that is not a
    /// discipline but a fact — **the post fails silently** (T4), so a Windows
    /// post is never treated as delivered.
    ///
    /// **T24's replace-by-identity is the tag**, which is the presentation key,
    /// with one group for all of Smoozy's posts.
    fn post(&self, post: &APost) {
        let identifier = HSTRING::from(THE_APPLICATION_IDENTIFIER);
        let Ok(notifier) = ToastNotificationManager::CreateToastNotifierWithId(&identifier) else {
            return;
        };
        let Ok(document) = XmlDocument::new() else {
            return;
        };
        if document.LoadXml(&HSTRING::from(the_toast_document(post))).is_err() {
            return;
        }
        let Ok(toast) = ToastNotification::CreateToastNotification(&document) else {
            return;
        };
        let key = post.key.as_text();
        let _ = toast.SetTag(&HSTRING::from(key.as_str()));
        let _ = toast.SetGroup(&HSTRING::from(THE_ONE_GROUP));

        // **T28: a bare click on the notification body is itself an action**,
        // and it comes back here while the application is running — which is
        // T30's whole width. What is read is the tag, which is the
        // presentation key and is composed from identities alone (T29).
        let clicked = key.clone();
        let _ = toast.Activated(&TypedEventHandler::new(
            move |_toast: windows::core::Ref<'_, ToastNotification>,
                  _argument: windows::core::Ref<'_, windows::core::IInspectable>| {
                if let Some(go) = WHAT_A_CLICK_DOES.get() {
                    go(clicked.clone());
                }
                Ok(())
            },
        ));

        if let Ok(mut live) = the_live_posts().lock() {
            live.insert(key, toast.clone());
        }
        let _ = notifier.Show(&toast);
    }

    /// **T24a and T24b: attempted and forgotten.**
    ///
    /// §22 item 61: whether a posted notification can be taken down at all is
    /// **not established on any of the three**, and this route in particular
    /// has never been run.
    fn take_down(&self, key: &ThePresentationKey) {
        let text = key.as_text();
        if let Ok(mut live) = the_live_posts().lock() {
            live.remove(&text);
        }
        let Ok(history) = ToastNotificationManager::History() else {
            return;
        };
        let _ = history.RemoveGroupedTagWithId(
            &HSTRING::from(text.as_str()),
            &HSTRING::from(THE_ONE_GROUP),
            &HSTRING::from(THE_APPLICATION_IDENTIFIER),
        );
    }
}

// ── The toast's own document ────────────────────────────────────────────────

/// **The toast, as Microsoft's own schema wants it.**
///
/// **The strings keep their two routes.** Both pieces of prose have been
/// through the typography point; the name is a value that has been through
/// nothing. **What [`escaped`] does is not a substitution of either kind**: it
/// is how a string is carried inside a document at all, and the system reads
/// the original back out — so a name is still the bytes it was, which is what
/// X7 is about.
///
/// **Two text elements, which is Design's two lines.** The first is the name and
/// what happened; the second is the figures, and it is left out entirely where
/// they are owed rather than drawn as an empty row.
///
/// **T16's Windows picture: the local file path, and the contradiction is
/// inherited unresolved.** Two Microsoft pages disagree about which image
/// source works for an unpackaged app; the local-file route is what is
/// attempted, and **the notification still says the job finished where it does
/// not work.**
fn the_toast_document(post: &APost) -> String {
    let mut document = String::new();
    document.push_str("<toast><visual><binding template=\"ToastGeneric\">");
    document.push_str(&format!("<text>{}</text>", escaped(&post.headline())));
    let second_line = post.the_second_line();
    if !second_line.is_empty() {
        document.push_str(&format!("<text>{}</text>", escaped(&second_line)));
    }
    if let Some(path) = picture::as_path(&post.picture) {
        document.push_str(&format!(
            "<image src=\"file:///{}\"/>",
            escaped(&path.to_string_lossy().replace('\\', "/"))
        ));
    }
    document.push_str("</binding></visual></toast>");
    document
}

/// The five characters a document cannot carry as themselves.
fn escaped(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            other => out.push(other),
        }
    }
    out
}

// ── T3 and §19 E3: the shortcut ─────────────────────────────────────────────

/// **What the shortcut is called, in the one place both the read and the act
/// take it from.**
const THE_SHORTCUTS_NAME: &str = "Smoozy.lnk";

/// **The name the new shortcut is written under before it is renamed over the
/// old one** (Z15a). It does not end `.lnk`, so the Start menu is not handed a
/// second Smoozy for the moment it exists. Step 11's Windows update piece.
const THE_SHORTCUT_BEING_WRITTEN: &str = "Smoozy.lnk.being-written";

/// **Z15a: the shortcut's target rewritten**, whole and at once: a complete new
/// shortcut naming `program`, carrying the same application identifier, saved
/// beside the old one and published over it by the platform file adapter's
/// rename (Z24) — `MoveFileExW`'s *"If a file named lpNewFileName exists, the
/// function replaces its contents"*, with F16's standing note that Microsoft
/// documents no crash guarantee for it. Made where none was there. Step 11's Windows update piece.
pub(super) fn the_shortcut_is_pointed_at(program: &std::path::Path) -> Result<(), String> {
    let folder = the_users_own_programs_folder().ok_or("the user's Start menu folder is not named")?;
    the_shortcut_in_this_folder_points_at(&folder, program)
}

/// The same act in any folder, so the hosted Windows test can do it in a
/// throwaway one rather than the user's Start menu.
fn the_shortcut_in_this_folder_points_at(folder: &std::path::Path, program: &std::path::Path) -> Result<(), String> {
    let shortcut = folder.join(THE_SHORTCUTS_NAME);
    let staged = folder.join(THE_SHORTCUT_BEING_WRITTEN);
    let _ = crate::platform::files::remove_file(&staged);
    make_the_shortcut(&staged, program).ok_or("the new shortcut could not be written")?;
    crate::platform::files::publish(&staged, &shortcut).map_err(|failed| failed.reported)
}

/// **Whether the shortcut is there. It writes nothing.**
///
/// A read that made a file was the fault F-10 named, and it is why this is two
/// functions rather than one.
fn the_start_menu_shortcut_is_there() -> bool {
    the_users_own_programs_folder()
        .map(|folder| folder.join(THE_SHORTCUTS_NAME).exists())
        .unwrap_or(false)
}

/// **The user's own Start Menu Programs folder, by Microsoft's own published
/// call** rather than by a path composed out of a name.
///
/// **This is the one location in this crate resolved outside
/// `src/platform/locations.rs`, and it is deliberate.** That adapter is F11's
/// one place *Smoozy's own places* are resolved, and this is not one of them:
/// it is the operating system's own Start-menu folder, it exists on one of the
/// three systems, and it is read by exactly one act. Putting it in a
/// cross-system enum would make two systems answer a question they do not have.
fn the_users_own_programs_folder() -> Option<std::path::PathBuf> {
    // Safety: the returned buffer is freed on both paths below, which is what
    // the call's own documentation requires of the caller.
    unsafe {
        let path = SHGetKnownFolderPath(&FOLDERID_Programs, KF_FLAG_DEFAULT, None).ok()?;
        let folder = path.to_string().ok().map(std::path::PathBuf::from);
        CoTaskMemFree(Some(path.0 as *const core::ffi::c_void));
        folder
    }
}

/// **Microsoft's worked route, performed** — §19 E3.
///
/// A shell link pointing at this executable, carrying the application
/// identifier on its own property store, saved into the user's own Programs
/// folder. **No elevation anywhere** (T4).
fn make_the_shortcut(shortcut: &std::path::Path, us: &std::path::Path) -> Option<()> {
    // Safety: the component system is started for this call and undone on the
    // way out whether or not anything else succeeded, because the thread is
    // not ours. Every pointer below is owned for the length of the call, and
    // the one pointer that is handed over rather than lent — the property
    // value's string — is allocated by the allocator that frees it.
    unsafe {
        let started = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let outcome = (|| -> Option<()> {
            let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER).ok()?;
            let target: Vec<u16> = wide(&us.to_string_lossy());
            link.SetPath(PCWSTR(target.as_ptr())).ok()?;

            let store: IPropertyStore = link.cast().ok()?;
            let identifier = wide(THE_APPLICATION_IDENTIFIER);
            let mut value = PROPVARIANT::default();
            {
                // **The string a `VT_LPWSTR` value carries belongs to the
                // component system's own allocator, and this is the line run
                // 34690208297 died on.** The property store does copy the
                // value, so the store is not the problem — but the value
                // itself is cleared when it goes out of scope, and clearing a
                // `VT_LPWSTR` frees its string with `CoTaskMemFree`. A pointer
                // into a Rust `Vec` handed over here is therefore freed twice
                // and by two different allocators, which on Windows is
                // `STATUS_HEAP_CORRUPTION` and killed the whole test binary
                // with nothing reported as failed.
                //
                // **So the copy handed over is made with `CoTaskMemAlloc` and
                // the value owns it.** Nothing else is freed here and nothing
                // outlives the call.
                let copy = CoTaskMemAlloc(std::mem::size_of_val(&identifier[..])) as *mut u16;
                if copy.is_null() {
                    return None;
                }
                std::ptr::copy_nonoverlapping(identifier.as_ptr(), copy, identifier.len());
                let raw = &mut value.Anonymous.Anonymous;
                raw.vt = VT_LPWSTR;
                raw.Anonymous.pwszVal = windows::core::PWSTR(copy);
            }
            store.SetValue(&PKEY_AppUserModel_ID, &value).ok()?;
            store.Commit().ok()?;

            let file: IPersistFile = link.cast().ok()?;
            let where_it_goes: Vec<u16> = wide(&shortcut.to_string_lossy());
            file.Save(PCWSTR(where_it_goes.as_ptr()), true).ok()?;
            Some(())
        })();
        if started.is_ok() {
            CoUninitialize();
        }
        outcome
    }
}

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(Some(0)).collect()
}

/// **What is owed on this system, named rather than left to be discovered.**
///
/// **The component-server route is not built.** Microsoft's own reasoning for
/// it, quoted at `The three desktop systems read again` §7, is that
/// notifications *"can stay around for several days before they are seen by
/// the user and processed"*, so an application may not be running when the
/// user responds. **What is built instead is the toast object's own activation
/// event, which reaches this application while it is running** — and T30 says
/// outright that **nothing here is built on a cold start**, the spec's own
/// bound being that a notification reaches the user while Smoozy runs behind
/// their work and never once they have quit it.
///
/// **What it costs**: a click on a Windows toast after Smoozy has been closed
/// does not route. §22 item 51 already carries that as unproven rather than
/// promised — *"what is lost on a cold Windows or Linux click is the going,
/// never state"* — and T22 is what makes it survivable, every one of the four
/// kinds leaving its own durable in-app record.
pub const THE_COMPONENT_SERVER_ROUTE_IS_NOT_BUILT: &str =
    "a registered local component server, for a click on a toast after Smoozy has been closed";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notifications::kinds::Kind;
    use crate::notifications::picture::{ThePicture, WhyThereIsNoPicture};
    use crate::typography::{Prose, Verbatim};
    use smoozy_state::identities::{Identity, IdentityKind};
    use std::path::PathBuf;

    /// **A job's ending, in the shape this application really composes one
    /// today** — [`kinds`]'s own ending sites: the first line is the name and
    /// what happened, and **the second line is empty**, because the frames, the
    /// duration and the server label live on a `Queue & History` job record
    /// that has not been built. It is the post that carries a picture, the
    /// derivative being a rendered frame.
    fn a_post(picture: ThePicture) -> APost {
        APost {
            key: ThePresentationKey::AJobsEnding(
                Identity::issue(IdentityKind::Job).expect("the desk has randomness"),
            ),
            kind: Kind::AJobFinished,
            said: Prose::new("finished"),
            about: Verbatim::new("Kitchen — final"),
            body: Prose::new(""),
            on: Verbatim::new(""),
            picture,
        }
    }

    /// **A post whose second line is really there** — Design's connection kind,
    /// *Old 4080 disconnected* over *Could not reconnect.*, which is one of
    /// the two kinds Design gives words rather than figures for and so lands in
    /// full today.
    ///
    /// **The name is a user's own and is chosen for its characters.** A server
    /// row can be called anything a person types, and the five characters a
    /// document cannot carry as themselves are exactly where the verbatim route
    /// would show if it were not kept.
    fn a_post_of_two_lines() -> APost {
        APost {
            key: ThePresentationKey::ACondition(
                Identity::issue(IdentityKind::Condition).expect("the desk has randomness"),
            ),
            kind: Kind::AConnectionCouldNotBeRestored,
            said: Prose::new("disconnected"),
            about: Verbatim::new("Old 4080 <best> & \"spare\""),
            body: Prose::new("Could not reconnect."),
            on: Verbatim::new(""),
            picture: ThePicture::None(WhyThereIsNoPicture::TheDerivativeIsNotOnDiskYet),
        }
    }

    /// **The two routes survive the document.**
    ///
    /// The name is carried with its own characters escaped rather than
    /// changed, which is what a document does with any value; the point's
    /// non-breaking space in the prose is a character and passes through
    /// untouched.
    ///
    /// **Both routes now meet inside the first line**, the name and what
    /// happened being joined by [`APost::headline`] before the document is
    /// composed — so this is also the line that fails if somebody puts the
    /// joined string back through the typography point, which would drop a
    /// substituted character inside a name Smoozy is forbidden to alter (X7).
    #[test]
    fn the_document_carries_both_strings_without_altering_either() {
        let document = the_toast_document(&a_post_of_two_lines());
        // The verbatim route: escaped inside the document, and not otherwise
        // touched — and the first line is the name, then what happened.
        assert!(document
            .contains("<text>Old 4080 &lt;best&gt; &amp; &quot;spare&quot; disconnected</text>"));
        assert!(
            !document.contains("Old\u{00A0}4080"),
            "a substitution reached the name — X7: the join is the caller's and \
             nothing may put the joined string back through the point"
        );
        // The prose route: the point's own substitution reaches the document as
        // the character it is.
        assert!(document
            .contains("<text>Could not\u{00A0}reconnect.</text>"));
        assert!(!document.contains("<image"));
    }

    /// **The second line is left out entirely where the figures are owed**,
    /// rather than drawn as an empty row — which is what all three ending kinds
    /// produce today, and what the composer's own head claims.
    #[test]
    fn a_post_with_nothing_for_its_second_line_draws_one_line_and_no_empty_row() {
        let document = the_toast_document(&a_post(ThePicture::None(
            WhyThereIsNoPicture::TheDerivativeIsNotOnDiskYet,
        )));
        assert!(document.contains("<text>Kitchen — final finished</text>"));
        assert_eq!(document.matches("<text>").count(), 1);
        assert!(!document.contains("<text></text>"));
    }

    /// **T16: the local-file route is what is attempted**, and nothing is
    /// claimed about it.
    #[test]
    fn a_picture_goes_in_as_a_local_file_path() {
        let document = the_toast_document(&a_post(ThePicture::ThisFileOnDisk(PathBuf::from(
            r"C:\Users\a\previews\frame.png",
        ))));
        assert!(document.contains("<image src=\"file:///C:/Users/a/previews/frame.png\"/>"));
    }

    /// **Z15a on a real Windows: the shortcut is written whole beside the old
    /// one and renamed over it**, in a throwaway folder and never the user's
    /// Start menu — made where none was, then rewritten to a second program,
    /// with nothing left under the name it was written as. **Runs on the
    /// hosted Windows job only**; it does not read the target back.
    #[test]
    fn the_shortcut_is_written_beside_and_renamed_over_in_a_throwaway_folder() {
        let folder = crate::scratch_folders::a_folder(format!("smoozy-shortcut-{}", std::process::id()));
        let first = folder.join("0.1.8").join("smoozy-desktop.exe");
        let second = folder.join("0.2.0").join("smoozy-desktop.exe");
        the_shortcut_in_this_folder_points_at(&folder, &first).expect("the shortcut is made");
        let made = std::fs::read(folder.join(THE_SHORTCUTS_NAME)).expect("the shortcut is there");
        the_shortcut_in_this_folder_points_at(&folder, &second).expect("the shortcut is rewritten");
        let rewritten = std::fs::read(folder.join(THE_SHORTCUTS_NAME)).expect("the shortcut is still there");
        assert_ne!(made, rewritten, "the rewritten shortcut is the old one");
        assert!(!folder.join(THE_SHORTCUT_BEING_WRITTEN).exists());
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// **T24: the tag is the presentation key**, so replace and take-down name
    /// the same thing.
    #[test]
    fn the_tag_is_the_presentation_key() {
        let source = include_str!("windows.rs");
        assert!(source.contains("toast.SetTag(&HSTRING::from(key.as_str()))"));
        assert!(source.contains("&HSTRING::from(text.as_str()),"));
    }
}

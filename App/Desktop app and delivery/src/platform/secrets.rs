//! The secret adapter and its helper — `Desktop app and delivery` §9.
//!
//! **S1: the store per system is taken as decided** by `Security and licence`
//! §3 and is not re-decided here. The contract it implements — the five
//! operations, the three-way classification of a failed get, the deadline, the
//! item names — is that document's and is already built; this module is the
//! three implementations of it and the one shared classification.
//!
//! **S2: one interface, three implementations, one shared classification.**
//! A get that fails is classified into *not there*, *locked or refused*, or
//! *store unavailable*, and **only *not there* means the user is asked again
//! as though nothing was remembered.** **An unmapped status is *store
//! unavailable* and never *not there***, because an unknown status must never
//! be read as permission to ask the user for a secret the store may still
//! hold.
//!
//! **S8: the three plaintext hops are the complete list and this document adds
//! no fourth** — the adapter for the length of its call, the one-use broker
//! for the length of one delivery, and the final endpoint that consumes it. A
//! builder that needs a fourth has found a fault in that section.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this crate's
//! own plan, `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! file's own header list below. **Where a marker is a clause of this crate's
//! own plan *and* of the document meant, the header list is not enough and the
//! document is written out at the site**, because a bare one there resolves to a
//! real and wrong clause.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Security and licence` S14, S15, S16, S17, S18, S19, S22,
//! S23, S24, S25, S28, S31.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.
//!
//! **This crate's own plan defines an S5, an S6, an S7, an S9 and an S11 of its
//! own** — the Linux probe, the module boundary, the askpass helper, the
//! post-update keychain dialog and the recorded erase debt — so none of those
//! is listed above. **This crate's own S5 appears below**, at the Linux probe,
//! and is bare there because it is this crate's.

use std::time::Duration;

use smoozy_security_and_licence::held::HeldSecret;
use smoozy_security_and_licence::plaintext::Plaintext;
use smoozy_security_and_licence::secret_store::{
    item_name, GetFailure, ProbeOutcome, SecretStore, StoreFailure,
};

#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
fn probe_the_linux_store(store: &dyn SecretStore, deadline: Duration) -> ProbeOutcome {
    use smoozy_security_and_licence::secret_store::probe_item;

    let item = probe_item();
    let value = Plaintext::new(b"probe".to_vec());
    if store.put(&item, value, deadline).is_err() {
        return ProbeOutcome::RoundTripFailed;
    }
    let read = store.get(&item, deadline).is_ok();
    // The throwaway is removed whether or not the read worked; a probe
    // that leaves its own item behind has changed the store it was asking
    // about.
    let _ = store.erase(&item, deadline);
    if read {
        ProbeOutcome::RoundTripWorked
    } else {
        ProbeOutcome::RoundTripFailed
    }
}

/// The retired item's subject follows the same copy boundary as every erase.
fn legacy_licence_subject(subject: &smoozy_security_and_licence::held::Subject) -> String {
    legacy_subject_for_copy(subject, crate::platform::locations::this_copy_is_a_throwaway_one())
}

fn legacy_subject_for_copy(subject: &smoozy_security_and_licence::held::Subject, throwaway: bool) -> String {
    if throwaway {
        format!("{THE_DEVELOPMENT_MARK}{}", subject.as_str())
    } else {
        subject.as_str().to_string()
    }
}

#[cfg_attr(target_os = "macos", allow(dead_code))]
fn legacy_licence_name(subject: &smoozy_security_and_licence::held::Subject) -> String {
    format!("{}.{}", smoozy_security_and_licence::secret_store::LEGACY_LICENCE_KEY_NAME, legacy_licence_subject(subject))
}

// ---------------------------------------------------------------------------
// What a throwaway copy's own items are called
// ---------------------------------------------------------------------------

/// **What a throwaway development copy puts in front of every subject it
/// writes.**
///
/// **The record and the home folder are two calls in one file; the operating
/// system's secret store is neither.** F11's own enumeration says so outright —
/// *"The operating system's secret store is not among them"* — so a copy
/// pointed at a throwaway record does not get a throwaway keychain. It gets
/// fresh subjects under the same item names, in the Creator's own login
/// keychain, and they stay there.
///
/// **The ruling, taken by the build runner on 2026-09-17: carry the throwaway
/// mark into the item naming.** Every item a development copy writes is plainly
/// a development item, nothing it writes can collide with or overwrite a real
/// one, and the whole set can be swept. The two other ways — refusing every put
/// in a development copy, or leaving the store alone and forbidding a
/// passphrase save at the desk — were refused because only this one lets a
/// development copy reach the drawn state where a key's passphrase is
/// remembered.
///
/// **The mark goes in the subject rather than in the item name's prefix**, and
/// that is what makes it one change rather than three. The subject is the one
/// part of an item all three systems carry apart — macOS's account name,
/// Windows' name after the kind, the Linux attribute — so marking it marks the
/// item on every system without touching a service name, a name prefix or an
/// attribute set, and `item_name` and `item_from_name` still read each other.
///
/// **`Security and licence` §3.0 rule 1 is kept**: the subject stays opaque and
/// says nothing about the user. The mark says which copy of Smoozy wrote it.
pub(crate) const THE_DEVELOPMENT_MARK: &str = "development.";

/// The item as **this** copy writes it: marked in a throwaway copy, untouched
/// in every other. Every put, get and erase in all three adapters goes through
/// this, so a development copy can neither read nor overwrite a real item.
fn as_this_copy_names_it(item: &HeldSecret) -> HeldSecret {
    the_item_a_copy_writes(
        crate::platform::locations::this_copy_is_a_throwaway_one(),
        item,
    )
}

fn the_item_a_copy_writes(a_throwaway_copy: bool, item: &HeldSecret) -> HeldSecret {
    if !a_throwaway_copy {
        return item.clone();
    }
    HeldSecret::new(
        item.kind,
        smoozy_security_and_licence::held::Subject::issued_by_the_local_record(format!(
            "{THE_DEVELOPMENT_MARK}{}",
            item.subject.as_str()
        )),
    )
}

/// **What an item the store hands back means to this copy** — the item as it
/// was asked for where this copy wrote it, and nothing where another kind of
/// copy did.
///
/// **It is what keeps enumerate honest in both directions**: a development copy
/// never lists the Creator's real items, and an ordinary copy never lists a
/// development one. Enumerate is what the erasure sweeps run over, so a wrong
/// answer here would either miss a development item or remove a real one.
fn as_this_copy_reads_it(item: HeldSecret) -> Option<HeldSecret> {
    the_item_a_copy_reads(
        crate::platform::locations::this_copy_is_a_throwaway_one(),
        item,
    )
}

fn the_item_a_copy_reads(a_throwaway_copy: bool, item: HeldSecret) -> Option<HeldSecret> {
    use smoozy_security_and_licence::held::Subject;

    let subject = item.subject.as_str();
    let plain = match (a_throwaway_copy, subject.strip_prefix(THE_DEVELOPMENT_MARK)) {
        (true, Some(plain)) => plain.to_string(),
        (true, None) => return None,
        (false, Some(_)) => return None,
        (false, None) => return Some(item),
    };
    Some(HeldSecret::new(
        item.kind,
        Subject::issued_by_the_local_record(plain),
    ))
}

/// What a sweep of the development items came to.
#[cfg(debug_assertions)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TheDevelopmentSweep {
    /// Items the store confirmed gone.
    pub removed: usize,
    /// Items the store would not remove.
    pub left_behind: usize,
    /// The store would not say what it holds, so nothing was swept.
    pub the_store_would_not_say: bool,
}

/// **Every development item this computer holds, removed at the start of a
/// throwaway copy — and in an ordinary copy this does nothing at all.**
///
/// It sits beside the sweep of the two throwaway areas
/// ([`super::sweep_the_throwaway_areas`]) and is there for the same reason: a
/// copy must not inherit the last one's secrets, and the run that wrote them
/// is gone and cannot clear up after itself.
///
/// **An ordinary copy sweeps nothing, and that is the direction that matters.**
/// In an ordinary copy every item the store lists is a real one, so a sweep
/// there would erase the Creator's own passphrases, machine passwords and
/// bot token.
#[cfg(debug_assertions)]
pub fn sweep_the_development_items(
    store: &dyn SecretStore,
    deadline: Duration,
) -> TheDevelopmentSweep {
    the_development_sweep(
        store,
        deadline,
        crate::platform::locations::this_copy_is_a_throwaway_one(),
    )
}

#[cfg(debug_assertions)]
fn the_development_sweep(
    store: &dyn SecretStore,
    deadline: Duration,
    a_throwaway_copy: bool,
) -> TheDevelopmentSweep {
    let mut swept = TheDevelopmentSweep::default();
    if !a_throwaway_copy {
        return swept;
    }
    match store.enumerate(deadline) {
        // Nothing is inferred from a store that will not answer: the items may
        // be there and the next throwaway copy asks again.
        Err(_) => swept.the_store_would_not_say = true,
        Ok(items) => {
            for item in items {
                if store.erase(&item, deadline).is_ok() {
                    swept.removed += 1;
                } else {
                    swept.left_behind += 1;
                }
            }
        }
    }
    swept
}

/// **S3 and S4 in one place: no store call is waited on without a deadline,
/// and every store call runs off the interface thread.**
///
/// **Store calls are made one at a time, in the order they were asked, on one
/// thread kept for them.** The caller waits for the deadline, counted from the
/// moment it asked. **A call that does not answer within it is *store
/// unavailable***, which is `Security and licence` §3.0 rule 5's own answer.
///
/// **Why one at a time, and it is the whole point of this type. Added
/// 2026-09-13.** None of the three stores publishes a way to cancel a call in
/// flight, so a call that outruns its deadline keeps running. Until this date
/// each call had a thread of its own, so an abandoned call could land **after**
/// a later one: a put reported *not stored* was stored a moment later, after an
/// erase had already been checked gone. That was watched on this Mac at step 10
/// piece 5. Rule 5 says the timed-out call is *reported as an action that did
/// not happen*, and three things here keep that report from being overtaken:
///
/// 1. **A later call waits behind an earlier one**, so no later call can see
///    the store before an abandoned call has finished. An erase asked while a
///    late put is still running times out too; it is never checked gone early.
/// 2. **A call whose caller stopped waiting before it started is never
///    started.**
/// 3. **A put that was already running when its caller stopped waiting, and
///    then landed under an item the store did not hold, is undone at once**,
///    before the next call runs. **A put that landed over a value already held
///    is not undone** — `Security and licence` §3.0 rule 5 as amended
///    2026-09-14. See [`TheStoreQueue::a_write_within_deadline`] for why.
///
/// **What it costs.** A call that never returns — the observed 45-second
/// block, `Security and licence` §14 item 14 — holds every later call for that
/// run, and each of those answers *store unavailable* at its own deadline.
/// That is true: the store is not answering.
///
/// **The Linux adapter does not use this, and that is a finding rather than a
/// choice.** Its calls hold library pointers that cannot be handed to another
/// thread, and they are made with no deadline at all, so on Linux rule 5 is
/// not kept: nothing is abandoned there, and nothing bounds the wait either.
/// Recorded at `Audits/Build/Security and licence/The keychain call that
/// outlives its limit — what was done.md`.
#[cfg_attr(target_os = "linux", allow(dead_code))]
pub(crate) struct TheStoreQueue {
    asking: std::sync::mpsc::Sender<Box<dyn FnOnce() + Send>>,
}

/// Where one asked call has got to. Guarded by one lock shared by the caller
/// and the store's thread, so the two can never both decide.
#[cfg_attr(target_os = "linux", allow(dead_code))]
enum Stage<T> {
    Waiting,
    Running,
    Answered(T),
    Taken,
    GaveUpBeforeItStarted,
    GaveUpWhileItRan,
}

#[cfg_attr(target_os = "linux", allow(dead_code))]
struct Asked<T> {
    stage: std::sync::Mutex<Stage<T>>,
    changed: std::sync::Condvar,
}

#[cfg_attr(target_os = "linux", allow(dead_code))]
impl<T> Asked<T> {
    fn stage(&self) -> std::sync::MutexGuard<'_, Stage<T>> {
        self.stage.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

#[cfg_attr(target_os = "linux", allow(dead_code))]
impl TheStoreQueue {
    /// A queue with its own thread. **Where the thread cannot be started, every
    /// call answers *store unavailable***, because nothing will ever take it.
    pub(crate) fn started() -> Self {
        let (asking, taking) = std::sync::mpsc::channel::<Box<dyn FnOnce() + Send>>();
        let _ = std::thread::Builder::new()
            .name("smoozy-secret-store".into())
            .spawn(move || {
                for call in taking {
                    call();
                }
            });
        TheStoreQueue { asking }
    }

    /// **The one queue every real store on this computer uses**, so calls
    /// through two copies of an adapter still wait for each other.
    pub(crate) fn for_this_computer() -> &'static TheStoreQueue {
        static THE_ONE: std::sync::OnceLock<TheStoreQueue> = std::sync::OnceLock::new();
        THE_ONE.get_or_init(TheStoreQueue::started)
    }

    /// A call that changes nothing if it lands late — a get, an enumerate — or
    /// whose late landing the erase marks already settle — an erase.
    ///
    /// **An erase that lands after its caller stopped waiting is not undone,
    /// and cannot be.** Its caller was told *store unavailable*, so
    /// `Security and licence` §5.1's mark stands, and the next erase the sweep
    /// makes finds the item gone and clears it. **Until then the report says
    /// the erase was not confirmed, and it was not.**
    pub(crate) fn within_deadline<T: Send + 'static>(
        &self,
        deadline: Duration,
        call: impl FnOnce() -> T + Send + 'static,
    ) -> Option<T> {
        self.a_write_within_deadline(deadline, call, |_| false, || {})
    }

    /// **A put: a call that, if it lands after its caller stopped waiting
    /// under an item the store did not already hold, is undone before any
    /// later call runs.** `landed_as_a_new_item` says whether the answer means
    /// the value was written where nothing was held; `undo` removes it.
    ///
    /// **`Security and licence` §3.0 rule 5, as amended 2026-09-14.** A put
    /// that times out is *not confirmed*, never done, and the next read decides
    /// what is held. Two cases, and the call itself is what tells them apart,
    /// because it is the only moment the store says whether the item was there:
    ///
    /// - **A put under a subject nothing held** — every key passphrase, and a
    ///   first bot token — **is undone if it lands late.**
    ///   Nothing points at it, and the store ends exactly as it was.
    /// - **A put that replaced a value already held under the same subject** —
    ///   the bot token — **is not undone if it lands late.**
    ///   The old value was gone the moment it landed; undoing it would leave
    ///   neither, where keeping it leaves the value the user just gave. The
    ///   caller says *not confirmed*, and the next read finds the new value.
    ///
    /// Where the undo itself fails, the value stays; nothing is left to report
    /// it to.
    pub(crate) fn a_write_within_deadline<T: Send + 'static>(
        &self,
        deadline: Duration,
        call: impl FnOnce() -> T + Send + 'static,
        landed_as_a_new_item: impl FnOnce(&T) -> bool + Send + 'static,
        undo: impl FnOnce() + Send + 'static,
    ) -> Option<T> {
        let until = std::time::Instant::now() + deadline;
        let asked = std::sync::Arc::new(Asked {
            stage: std::sync::Mutex::new(Stage::Waiting),
            changed: std::sync::Condvar::new(),
        });

        let on_the_stores_thread = asked.clone();
        let the_call = move || {
            {
                let mut stage = on_the_stores_thread.stage();
                if matches!(*stage, Stage::GaveUpBeforeItStarted) {
                    return;
                }
                *stage = Stage::Running;
            }
            let answer = call();
            let mut stage = on_the_stores_thread.stage();
            if matches!(*stage, Stage::GaveUpWhileItRan) {
                drop(stage);
                if landed_as_a_new_item(&answer) {
                    undo();
                }
                return;
            }
            *stage = Stage::Answered(answer);
            on_the_stores_thread.changed.notify_all();
        };
        if self.asking.send(Box::new(the_call)).is_err() {
            return None;
        }

        let mut stage = asked.stage();
        loop {
            if matches!(*stage, Stage::Answered(_)) {
                return match std::mem::replace(&mut *stage, Stage::Taken) {
                    Stage::Answered(answer) => Some(answer),
                    _ => None,
                };
            }
            let now = std::time::Instant::now();
            if now >= until {
                *stage = if matches!(*stage, Stage::Waiting) {
                    Stage::GaveUpBeforeItStarted
                } else {
                    Stage::GaveUpWhileItRan
                };
                return None;
            }
            stage = asked
                .changed
                .wait_timeout(stage, until - now)
                .map(|(stage, _)| stage)
                .unwrap_or_else(|poisoned| poisoned.into_inner().0);
        }
    }
}

// ===========================================================================
// macOS — the file-based keychain through the `SecItem` API
// ===========================================================================

/// The macOS store — `Security and licence` §3.1 and `Desktop app and
/// delivery` S1.
///
/// **The data protection keychain is not used, and the way that is kept is by
/// writing nothing.** Apple's TN3137 says the `SecItem` API *"defaults to
/// targeting the file-based keychain"*, so `kSecUseDataProtectionKeychain` is
/// never placed in any dictionary here. `Security and licence` S5's reason
/// stands: reaching the other one needs entitlements authorised by a
/// provisioning profile, and whether a Developer ID application shipped
/// without one can reach it at all is permanently unresolved.
///
/// **`Security and licence` S9: synchronisation is off.**
/// `kSecAttrSynchronizable` is never set, so nothing goes to the user's other
/// devices.
#[cfg(target_os = "macos")]
pub struct MacKeychain;

#[cfg(target_os = "macos")]
pub(crate) mod apple {
    //! The calls and the statuses, taken from Apple's own headers shipped on
    //! this machine — `SecItem.h` and `SecBase.h` under the macOS SDK, the
    //! same files `Security and licence` S11 already cites, at that document's
    //! §3.1.

    use core::ffi::c_void;

    pub type CFTypeRef = *const c_void;
    pub type OSStatus = i32;

    #[link(name = "Security", kind = "framework")]
    extern "C" {
        /// `OSStatus SecItemAdd(CFDictionaryRef attributes, CFTypeRef *result)`
        pub fn SecItemAdd(attributes: CFTypeRef, result: *mut CFTypeRef) -> OSStatus;
        /// `OSStatus SecItemCopyMatching(CFDictionaryRef query, CFTypeRef *result)`
        pub fn SecItemCopyMatching(query: CFTypeRef, result: *mut CFTypeRef) -> OSStatus;
        /// `OSStatus SecItemUpdate(CFDictionaryRef query, CFDictionaryRef attributesToUpdate)`
        pub fn SecItemUpdate(query: CFTypeRef, attributes_to_update: CFTypeRef) -> OSStatus;
        /// `OSStatus SecItemDelete(CFDictionaryRef query)`
        pub fn SecItemDelete(query: CFTypeRef) -> OSStatus;

        pub static kSecClass: CFTypeRef;
        pub static kSecClassGenericPassword: CFTypeRef;
        pub static kSecAttrService: CFTypeRef;
        pub static kSecAttrAccount: CFTypeRef;
        pub static kSecValueData: CFTypeRef;
        pub static kSecReturnData: CFTypeRef;
        pub static kSecReturnAttributes: CFTypeRef;
        pub static kSecMatchLimit: CFTypeRef;
        pub static kSecMatchLimitAll: CFTypeRef;
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        pub fn CFRelease(cf: CFTypeRef);
        pub fn CFDictionaryCreate(
            allocator: CFTypeRef,
            keys: *const CFTypeRef,
            values: *const CFTypeRef,
            num_values: isize,
            key_callbacks: *const c_void,
            value_callbacks: *const c_void,
        ) -> CFTypeRef;
        pub fn CFDictionaryGetValue(dictionary: CFTypeRef, key: CFTypeRef) -> CFTypeRef;
        pub fn CFDataCreate(allocator: CFTypeRef, bytes: *const u8, length: isize) -> CFTypeRef;
        pub fn CFDataGetBytePtr(data: CFTypeRef) -> *const u8;
        pub fn CFDataGetLength(data: CFTypeRef) -> isize;
        pub fn CFStringCreateWithBytes(
            allocator: CFTypeRef,
            bytes: *const u8,
            num_bytes: isize,
            encoding: u32,
            is_external_representation: u8,
        ) -> CFTypeRef;
        pub fn CFStringGetLength(string: CFTypeRef) -> isize;
        pub fn CFStringGetCString(
            string: CFTypeRef,
            buffer: *mut i8,
            buffer_size: isize,
            encoding: u32,
        ) -> u8;
        pub fn CFArrayGetCount(array: CFTypeRef) -> isize;
        pub fn CFArrayGetValueAtIndex(array: CFTypeRef, index: isize) -> CFTypeRef;
        pub fn CFGetTypeID(cf: CFTypeRef) -> usize;
        pub fn CFArrayGetTypeID() -> usize;

        pub static kCFTypeDictionaryKeyCallBacks: c_void;
        pub static kCFTypeDictionaryValueCallBacks: c_void;
        pub static kCFBooleanTrue: CFTypeRef;
    }

    /// `kCFStringEncodingUTF8`, from CoreFoundation's own `CFString.h`.
    pub const UTF8: u32 = 0x0800_0100;

    /// **`Security and licence` S7's five statuses, by symbol.** The numerals
    /// are Apple's own, read from `SecBase.h` in the macOS SDK on this
    /// machine, and **the mapping is by these names and never by a bare number
    /// written into a match arm** — which is the whole of what `map by symbol`
    /// can mean in a language with no access to the header.
    pub const ERR_SEC_SUCCESS: OSStatus = 0;
    pub const ERR_SEC_USER_CANCELED: OSStatus = -128;
    pub const ERR_SEC_AUTH_FAILED: OSStatus = -25293;
    pub const ERR_SEC_NO_SUCH_KEYCHAIN: OSStatus = -25294;
    pub const ERR_SEC_INVALID_KEYCHAIN: OSStatus = -25295;
    pub const ERR_SEC_DUPLICATE_ITEM: OSStatus = -25299;
    pub const ERR_SEC_ITEM_NOT_FOUND: OSStatus = -25300;
    pub const ERR_SEC_INTERACTION_NOT_ALLOWED: OSStatus = -25308;
    pub const ERR_SEC_INTERACTION_REQUIRED: OSStatus = -25315;
}

#[cfg(target_os = "macos")]
pub(crate) mod mac {
    use super::apple::*;
    use core::ffi::c_void;

    /// A CoreFoundation object this code owns and releases.
    pub struct Owned(pub CFTypeRef);

    impl Drop for Owned {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { CFRelease(self.0) };
            }
        }
    }

    pub fn string(value: &str) -> Owned {
        Owned(unsafe {
            CFStringCreateWithBytes(
                std::ptr::null(),
                value.as_ptr(),
                value.len() as isize,
                UTF8,
                0,
            )
        })
    }

    pub fn data(value: &[u8]) -> Owned {
        Owned(unsafe { CFDataCreate(std::ptr::null(), value.as_ptr(), value.len() as isize) })
    }

    pub fn dictionary(pairs: &[(CFTypeRef, CFTypeRef)]) -> Owned {
        let keys: Vec<CFTypeRef> = pairs.iter().map(|(key, _)| *key).collect();
        let values: Vec<CFTypeRef> = pairs.iter().map(|(_, value)| *value).collect();
        Owned(unsafe {
            CFDictionaryCreate(
                std::ptr::null(),
                keys.as_ptr(),
                values.as_ptr(),
                pairs.len() as isize,
                &kCFTypeDictionaryKeyCallBacks as *const c_void,
                &kCFTypeDictionaryValueCallBacks as *const c_void,
            )
        })
    }

    pub fn bytes_of(data: CFTypeRef) -> Vec<u8> {
        if data.is_null() {
            return Vec::new();
        }
        let length = unsafe { CFDataGetLength(data) };
        let pointer = unsafe { CFDataGetBytePtr(data) };
        if pointer.is_null() || length <= 0 {
            return Vec::new();
        }
        unsafe { std::slice::from_raw_parts(pointer, length as usize) }.to_vec()
    }

    pub fn text_of(string: CFTypeRef) -> Option<String> {
        if string.is_null() {
            return None;
        }
        // Four bytes per character is the widest UTF-8 can be, plus the
        // terminator the call writes.
        let size = unsafe { CFStringGetLength(string) } * 4 + 1;
        let mut buffer = vec![0i8; size.max(1) as usize];
        let ok = unsafe { CFStringGetCString(string, buffer.as_mut_ptr(), size, UTF8) };
        if ok == 0 {
            return None;
        }
        let bytes: Vec<u8> = buffer
            .iter()
            .take_while(|byte| **byte != 0)
            .map(|byte| *byte as u8)
            .collect();
        String::from_utf8(bytes).ok()
    }
}

#[cfg(target_os = "macos")]
impl MacKeychain {
    /// **`Security and licence` S7, and it is the one place a status becomes a
    /// class.**
    ///
    /// `errSecItemNotFound` is *not there*. `errSecUserCanceled`,
    /// `errSecInteractionRequired`, `errSecInteractionNotAllowed` and
    /// `errSecAuthFailed` are the store refusing this once — the last of them
    /// being what a **different** binary reading the same item was refused
    /// with (`Security and licence` S6, at that document's §3.1, observed).
    /// `errSecNoSuchKeychain`
    /// and `errSecInvalidKeychain` are the store absent or unusable.
    ///
    /// **Everything else is *store unavailable*.** S2's rule, and it is the
    /// direction that costs the user a repeated action rather than a retyped
    /// secret.
    fn classify(status: apple::OSStatus) -> GetFailure {
        use apple::*;
        match status {
            ERR_SEC_ITEM_NOT_FOUND => GetFailure::NotThere,
            ERR_SEC_USER_CANCELED
            | ERR_SEC_INTERACTION_REQUIRED
            | ERR_SEC_INTERACTION_NOT_ALLOWED
            | ERR_SEC_AUTH_FAILED => GetFailure::LockedOrRefused,
            ERR_SEC_NO_SUCH_KEYCHAIN | ERR_SEC_INVALID_KEYCHAIN => GetFailure::StoreUnavailable,
            _ => GetFailure::from_an_unmapped_status(),
        }
    }

    /// Put and erase keep the same refusal classification as get. Neither a
    /// missing item nor an unmapped status can confirm a failed write.
    fn write_failure(status: apple::OSStatus) -> StoreFailure {
        match Self::classify(status) {
            GetFailure::LockedOrRefused => StoreFailure::LockedOrRefused,
            GetFailure::NotThere | GetFailure::StoreUnavailable => StoreFailure::StoreUnavailable,
        }
    }

    /// The two macOS statuses are both required. Keeping them separate here
    /// also lets desk tests cover a silent delete refusal and a matching item
    /// still present, without touching a real keychain.
    fn confirm_erase(
        delete: impl FnOnce() -> apple::OSStatus,
        check: impl FnOnce() -> apple::OSStatus,
    ) -> apple::OSStatus {
        let deleted = delete();
        if deleted != apple::ERR_SEC_SUCCESS && deleted != apple::ERR_SEC_ITEM_NOT_FOUND {
            return deleted;
        }
        check()
    }

    /// **The service name is the kind's, and the account name is the subject.**
    ///
    /// This is the shape `Security and licence` S11's own observation, at that
    /// document's §3.1, was
    /// taken in — *"three items stored under one service name, the all-limit
    /// search returned 3"* — and it is what makes enumerate possible at all on
    /// this system: `SecItemCopyMatching` matches an attribute exactly and has
    /// no prefix search, so a service name carrying the subject could never be
    /// searched for by kind.
    ///
    /// **It is the same prefix scheme `item_name` composes**, split at the one
    /// place this store has two attributes to put it in; Windows has one
    /// target name and takes `item_name` whole. **S3 rule 1 binds either way:
    /// the subject is opaque and carries nothing about the user.**
    fn service_name(kind: smoozy_security_and_licence::held::StoredKind) -> String {
        let with_empty_subject = item_name(&HeldSecret::new(
            kind,
            smoozy_security_and_licence::held::Subject::issued_by_the_local_record(""),
        ));
        with_empty_subject.trim_end_matches('.').to_string()
    }
}

#[cfg(target_os = "macos")]
impl MacKeychain {
    fn erase_named_item(&self, name: String, subject: String, deadline: Duration) -> Result<(), StoreFailure> {
        let answer = TheStoreQueue::for_this_computer().within_deadline(deadline, move || unsafe {
            use apple::*;
            let service = mac::string(&name);
            let account = mac::string(&subject);
            let query = mac::dictionary(&[
                (kSecClass, kSecClassGenericPassword),
                (kSecAttrService, service.0),
                (kSecAttrAccount, account.0),
            ]);

            Self::confirm_erase(|| SecItemDelete(query.0), || {
                // The confirming read-back requests no secret data.
                let check = mac::dictionary(&[
                    (kSecClass, kSecClassGenericPassword),
                    (kSecAttrService, service.0),
                    (kSecAttrAccount, account.0),
                ]);
                SecItemCopyMatching(check.0, std::ptr::null_mut())
            })
        });

        match answer {
            None => Err(StoreFailure::from_the_deadline_passing()),
            Some(apple::ERR_SEC_ITEM_NOT_FOUND) => Ok(()),
            Some(status) => Err(Self::write_failure(status)),
        }
    }
}

#[cfg(target_os = "macos")]
impl SecretStore for MacKeychain {
    fn put(
        &self,
        item: &HeldSecret,
        value: Plaintext,
        deadline: Duration,
    ) -> Result<(), StoreFailure> {
        // **The item as this copy names it**, so a throwaway copy's put lands
        // under a development subject and can never replace a real item.
        let item = &as_this_copy_names_it(item);
        let name = Self::service_name(item.kind);
        let subject = item.subject.as_str().to_string();
        // **S8's first hop and its whole length.** The plaintext is exposed to
        // the call and to nothing else, and `expose` takes the value by move so
        // there is no second use of it.
        let bytes = value.expose(|bytes| bytes.to_vec());
        let (undo_name, undo_subject) = (name.clone(), subject.clone());

        // **Rule 5 as amended 2026-09-14: a put that lands after its caller
        // stopped waiting is deleted before any later call runs — unless it
        // replaced a value already held**
        // ([`TheStoreQueue::a_write_within_deadline`]). `SecItemAdd` answering
        // *duplicate item* is how this store says one was held.
        let answer = TheStoreQueue::for_this_computer().a_write_within_deadline(
            deadline,
            move || unsafe {
            use apple::*;
            let service = mac::string(&name);
            let account = mac::string(&subject);
            let secret = mac::data(&bytes);

            let attributes = mac::dictionary(&[
                (kSecClass, kSecClassGenericPassword),
                (kSecAttrService, service.0),
                (kSecAttrAccount, account.0),
                (kSecValueData, secret.0),
            ]);

            let mut status = SecItemAdd(attributes.0, std::ptr::null_mut());
            let replaced = status == ERR_SEC_DUPLICATE_ITEM;
            if replaced {
                // **Put replaces whatever was there**, which the contract's own
                // wording requires. The query names the item; the update
                // carries only the value.
                let query = mac::dictionary(&[
                    (kSecClass, kSecClassGenericPassword),
                    (kSecAttrService, service.0),
                    (kSecAttrAccount, account.0),
                ]);
                let replacement = mac::dictionary(&[(kSecValueData, secret.0)]);
                status = SecItemUpdate(query.0, replacement.0);
            }
            (status, replaced)
            },
            |(status, replaced)| *status == apple::ERR_SEC_SUCCESS && !*replaced,
            move || unsafe {
                use apple::*;
                let service = mac::string(&undo_name);
                let account = mac::string(&undo_subject);
                let query = mac::dictionary(&[
                    (kSecClass, kSecClassGenericPassword),
                    (kSecAttrService, service.0),
                    (kSecAttrAccount, account.0),
                ]);
                let _ = SecItemDelete(query.0);
            },
        );

        match answer {
            None => Err(StoreFailure::from_the_deadline_passing()),
            Some((apple::ERR_SEC_SUCCESS, _)) => Ok(()),
            Some((status, _)) => Err(Self::write_failure(status)),
        }
    }

    fn get(&self, item: &HeldSecret, deadline: Duration) -> Result<Plaintext, GetFailure> {
        // **A throwaway copy reads its own items and never a real one.**
        let item = &as_this_copy_names_it(item);
        let (name, subject) = (
            Self::service_name(item.kind),
            item.subject.as_str().to_string(),
        );

        let answer = TheStoreQueue::for_this_computer().within_deadline(deadline, move || unsafe {
            use apple::*;
            let service = mac::string(&name);
            let account = mac::string(&subject);
            let query = mac::dictionary(&[
                (kSecClass, kSecClassGenericPassword),
                (kSecAttrService, service.0),
                (kSecAttrAccount, account.0),
                (kSecReturnData, kCFBooleanTrue),
            ]);

            let mut found: CFTypeRef = std::ptr::null();
            let status = SecItemCopyMatching(query.0, &mut found);
            let value = if status == ERR_SEC_SUCCESS {
                let owned = mac::Owned(found);
                mac::bytes_of(owned.0)
            } else {
                Vec::new()
            };
            (status, value)
        });

        match answer {
            None => Err(GetFailure::from_the_deadline_passing()),
            Some((apple::ERR_SEC_SUCCESS, bytes)) => Ok(Plaintext::new(bytes)),
            Some((status, _)) => Err(Self::classify(status)),
        }
    }

    /// **S10: erasure is confirmed where the system confirms it and inferred
    /// nowhere.** On this system that is `SecItemDelete`'s status **plus a
    /// following query returning `errSecItemNotFound`**, which is what was
    /// observed end to end.
    fn erase(&self, item: &HeldSecret, deadline: Duration) -> Result<(), StoreFailure> {
        let item = as_this_copy_names_it(item);
        self.erase_named_item(Self::service_name(item.kind), item.subject.as_str().to_string(), deadline)
    }

    fn erase_legacy_licence_key(&self, subject: &smoozy_security_and_licence::held::Subject, deadline: Duration) -> Result<(), StoreFailure> {
        self.erase_named_item(smoozy_security_and_licence::secret_store::LEGACY_LICENCE_KEY_NAME.to_string(), legacy_licence_subject(subject), deadline)
    }

    /// **The probe is a Linux member of the contract.** Nothing on this system
    /// waits on it, so it answers without reaching the store — which is the
    /// contract's own wording and not a shortcut.
    fn probe(&self, _deadline: Duration) -> ProbeOutcome {
        ProbeOutcome::RoundTripWorked
    }

    /// **`Security and licence` S11: `SecItemCopyMatching` with the match
    /// limit set to all.**
    ///
    /// **The narrowing must not be lost: this enumerates by attribute, not by
    /// application.** The boundary is the service-name prefix Smoozy adopts,
    /// which is a naming convention rather than an operating-system guarantee.
    fn enumerate(&self, deadline: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
        use smoozy_security_and_licence::held::StoredKind;

        let mut found = Vec::new();
        for kind in StoredKind::ALL {
            let service = Self::service_name(kind);

            let answer = TheStoreQueue::for_this_computer().within_deadline(deadline, move || unsafe {
                use apple::*;
                let name = mac::string(&service);
                let query = mac::dictionary(&[
                    (kSecClass, kSecClassGenericPassword),
                    (kSecAttrService, name.0),
                    (kSecMatchLimit, kSecMatchLimitAll),
                    (kSecReturnAttributes, kCFBooleanTrue),
                ]);
                let mut result: CFTypeRef = std::ptr::null();
                let status = SecItemCopyMatching(query.0, &mut result);
                let mut subjects = Vec::new();
                if status == ERR_SEC_SUCCESS && !result.is_null() {
                    let owned = mac::Owned(result);
                    if CFGetTypeID(owned.0) == CFArrayGetTypeID() {
                        for index in 0..CFArrayGetCount(owned.0) {
                            let entry = CFArrayGetValueAtIndex(owned.0, index);
                            let account = CFDictionaryGetValue(entry, kSecAttrAccount);
                            if let Some(text) = mac::text_of(account) {
                                subjects.push(text);
                            }
                        }
                    }
                }
                (status, subjects)
            });

            match answer {
                None => return Err(StoreFailure::from_the_deadline_passing()),
                Some((apple::ERR_SEC_ITEM_NOT_FOUND, _)) => continue,
                Some((apple::ERR_SEC_SUCCESS, subjects)) => {
                    for subject in subjects {
                        // **Only the items this copy would have written.** A
                        // development copy lists its own and never the
                        // Creator's; an ordinary copy lists the Creator's and
                        // never a development one.
                        let held = HeldSecret::new(
                            kind,
                            smoozy_security_and_licence::held::Subject::issued_by_the_local_record(
                                subject,
                            ),
                        );
                        if let Some(held) = as_this_copy_reads_it(held) {
                            found.push(held);
                        }
                    }
                }
                Some(_) => return Err(StoreFailure::from_an_unmapped_status()),
            }
        }
        Ok(found)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use smoozy_security_and_licence::held::{StoredKind, Subject};
    use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
    use std::sync::mpsc::{channel, Receiver, Sender};
    use std::sync::{Arc, Mutex};

    /// The deadline the stand-in's callers wait. Short, because every call
    /// that must outrun it is held on purpose rather than merely slow.
    const A_DEADLINE: Duration = Duration::from_millis(100);
    /// Long enough that a call not held on purpose always answers inside it.
    const LONG: Duration = Duration::from_secs(10);

    fn an_item() -> HeldSecret {
        HeldSecret::new(
            StoredKind::BotToken,
            Subject::issued_by_the_local_record("a-stand-in"),
        )
    }

    /// **A stand-in store that is slow on purpose**, run through a queue of its
    /// own exactly as the real adapters run through theirs. **Nothing here
    /// reaches a real store.** Its next write can be held — it says when it has
    /// started and then waits to be released — which is what a keychain call
    /// blocked on a prompt looks like from Smoozy's side.
    struct AStandInStore {
        queue: TheStoreQueue,
        held: Arc<Mutex<Option<Vec<u8>>>>,
        the_next_write_is_held: Mutex<Option<(Sender<()>, Receiver<()>)>>,
        writes_run: Arc<AtomicUsize>,
        puts_undone: Arc<AtomicUsize>,
    }

    impl AStandInStore {
        fn new() -> Self {
            AStandInStore {
                queue: TheStoreQueue::started(),
                held: Arc::new(Mutex::new(None)),
                the_next_write_is_held: Mutex::new(None),
                writes_run: Arc::new(AtomicUsize::new(0)),
                puts_undone: Arc::new(AtomicUsize::new(0)),
            }
        }

        /// Holds the next put or erase. Answers the signal it sends when it
        /// starts, and the release.
        fn hold_the_next_write(&self) -> (Receiver<()>, Sender<()>) {
            let (started, has_started) = channel();
            let (release, released) = channel();
            *self.the_next_write_is_held.lock().unwrap() = Some((started, released));
            (has_started, release)
        }

        fn holds(&self) -> Option<Vec<u8>> {
            self.held.lock().unwrap().clone()
        }

        fn a_write(&self) -> impl FnOnce() + Send + 'static {
            let gate = self.the_next_write_is_held.lock().unwrap().take();
            let runs = self.writes_run.clone();
            move || {
                runs.fetch_add(1, SeqCst);
                if let Some((started, released)) = gate {
                    let _ = started.send(());
                    let _ = released.recv();
                }
            }
        }
    }

    impl SecretStore for AStandInStore {
        fn put(
            &self,
            _: &HeldSecret,
            value: Plaintext,
            deadline: Duration,
        ) -> Result<(), StoreFailure> {
            let bytes = value.expose(|bytes| bytes.to_vec());
            let (wait, held) = (self.a_write(), self.held.clone());
            let (held_for_the_undo, undone) = (self.held.clone(), self.puts_undone.clone());
            self.queue
                .a_write_within_deadline(
                    deadline,
                    // Answers whether it landed where nothing was held, which
                    // is what the real stores' own answers say.
                    move || {
                        wait();
                        let mut held = held.lock().unwrap();
                        let replaced = held.is_some();
                        *held = Some(bytes);
                        !replaced
                    },
                    |landed_as_a_new_item| *landed_as_a_new_item,
                    move || {
                        *held_for_the_undo.lock().unwrap() = None;
                        undone.fetch_add(1, SeqCst);
                    },
                )
                .map(|_| ())
                .ok_or(StoreFailure::from_the_deadline_passing())
        }

        fn get(&self, _: &HeldSecret, deadline: Duration) -> Result<Plaintext, GetFailure> {
            let held = self.held.clone();
            match self
                .queue
                .within_deadline(deadline, move || held.lock().unwrap().clone())
            {
                None => Err(GetFailure::from_the_deadline_passing()),
                Some(Some(bytes)) => Ok(Plaintext::new(bytes)),
                Some(None) => Err(GetFailure::NotThere),
            }
        }

        fn erase(&self, _: &HeldSecret, deadline: Duration) -> Result<(), StoreFailure> {
            let (wait, held) = (self.a_write(), self.held.clone());
            self.queue
                .within_deadline(deadline, move || {
                    wait();
                    *held.lock().unwrap() = None;
                })
                .ok_or(StoreFailure::from_the_deadline_passing())
        }

        fn probe(&self, _: Duration) -> ProbeOutcome {
            ProbeOutcome::RoundTripWorked
        }

        fn enumerate(&self, _: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(Vec::new())
        }
    }

    /// Asks for a put that is held past its deadline, and returns once the
    /// caller has been answered — with the put still running.
    fn a_put_left_running(store: &AStandInStore, value: &[u8]) -> Sender<()> {
        let (started, release) = store.hold_the_next_write();
        std::thread::scope(|scope| {
            let put = scope.spawn(|| store.put(&an_item(), Plaintext::new(value.to_vec()), A_DEADLINE));
            started.recv_timeout(LONG).expect("the put started");
            assert_eq!(
                put.join().unwrap(),
                Err(StoreFailure::StoreUnavailable),
                "`Security and licence` §3.0 rule 5: a call past its deadline is store unavailable"
            );
        });
        release
    }

    /// **The finding of step 10 piece 5, made true.** A put under an item the
    /// store did not hold, reported *store unavailable* and landing afterwards,
    /// is undone before a later call runs, so a read asked after the report
    /// never finds the value.
    #[test]
    fn a_put_that_lands_after_its_deadline_is_undone_before_anything_later_runs() {
        let store = AStandInStore::new();
        let release = a_put_left_running(&store, b"late");
        release.send(()).unwrap();

        match store.get(&an_item(), LONG) {
            Err(failure) => assert_eq!(failure, GetFailure::NotThere),
            Ok(_) => panic!("a put reported as not done was found in the store"),
        }
        assert_eq!(store.writes_run.load(SeqCst), 1, "the put did run");
        assert_eq!(store.puts_undone.load(SeqCst), 1, "and was undone");
        assert_eq!(store.holds(), None);
    }

    /// **`Security and licence` §3.0 rule 5 as amended 2026-09-14: a put that
    /// replaced a value already held, and lands after its deadline, is not
    /// undone.** The old value went when it landed, so undoing it would leave
    /// neither; the next read finds the value the user just gave.
    #[test]
    fn a_late_put_that_replaced_a_held_value_is_not_undone() {
        let store = AStandInStore::new();
        *store.held.lock().unwrap() = Some(b"old".to_vec());
        let release = a_put_left_running(&store, b"new");
        release.send(()).unwrap();

        let read = store
            .get(&an_item(), LONG)
            .expect("the next read decides, and it finds a value");
        assert_eq!(read.expose(|bytes| bytes.to_vec()), b"new");
        assert_eq!(store.writes_run.load(SeqCst), 1, "the put did run");
        assert_eq!(store.puts_undone.load(SeqCst), 0, "and was not undone");
    }

    /// **An erase can never be checked gone while an earlier put is still
    /// running.** Asked behind it, the erase outruns its own deadline and is
    /// never started; asked again once the put has finished, it confirms, and
    /// nothing lands after it.
    #[test]
    fn an_erase_asked_behind_a_late_put_is_never_checked_gone_early() {
        let store = AStandInStore::new();
        let release = a_put_left_running(&store, b"late");

        assert_eq!(
            store.erase(&an_item(), A_DEADLINE),
            Err(StoreFailure::StoreUnavailable),
            "the erase was answered before the put ahead of it finished"
        );
        release.send(()).unwrap();

        assert_eq!(store.erase(&an_item(), LONG), Ok(()));
        assert_eq!(
            store.writes_run.load(SeqCst),
            2,
            "a call whose caller gave up before it started is never started"
        );
        match store.get(&an_item(), LONG) {
            Err(failure) => assert_eq!(failure, GetFailure::NotThere),
            Ok(_) => panic!("something landed after the erase was confirmed"),
        }
    }

    /// **An erase that lands after its deadline cannot remove a value put after
    /// it.** The later put waits for it.
    #[test]
    fn a_late_erase_cannot_remove_a_value_put_after_it() {
        let store = AStandInStore::new();
        *store.held.lock().unwrap() = Some(b"old".to_vec());
        let (started, release) = store.hold_the_next_write();

        std::thread::scope(|scope| {
            let erase = scope.spawn(|| store.erase(&an_item(), A_DEADLINE));
            started.recv_timeout(LONG).expect("the erase started");
            assert_eq!(erase.join().unwrap(), Err(StoreFailure::StoreUnavailable));

            let put = scope.spawn(|| store.put(&an_item(), Plaintext::new(b"new".to_vec()), LONG));
            // Give the put time to be asked while the erase is still held.
            std::thread::sleep(Duration::from_millis(50));
            release.send(()).unwrap();
            assert_eq!(put.join().unwrap(), Ok(()));
        });

        assert_eq!(store.holds().as_deref(), Some(&b"new"[..]));
        assert_eq!(store.puts_undone.load(SeqCst), 0);
    }

    /// **Nothing a development copy writes can collide with or overwrite a real
    /// item**, which is the whole of what the ruling asked for.
    #[test]
    fn a_development_copys_item_is_never_a_real_ones() {
        let item = HeldSecret::new(
            StoredKind::KeyPassphrase,
            Subject::issued_by_the_local_record("key-7"),
        );

        let real = the_item_a_copy_writes(false, &item);
        let development = the_item_a_copy_writes(true, &item);

        assert_eq!(real, item, "an ordinary copy changed the item it writes");
        assert_ne!(development.subject, real.subject);
        assert_eq!(development.kind, real.kind);
        assert!(development
            .subject
            .as_str()
            .starts_with(THE_DEVELOPMENT_MARK));
        assert!(
            development.subject.as_str().ends_with("key-7"),
            "the subject the local record issued is still in the name, so an \
             item can still be read back to the row it belongs to"
        );

        // And the composed name follows it on the two systems that take the
        // whole name, with the kind still where `item_from_name` looks for it.
        assert_eq!(
            item_name(&development),
            "Smoozy.key-passphrase.development.key-7"
        );
        assert_eq!(
            smoozy_security_and_licence::secret_store::item_from_name(&item_name(&development)),
            Some(development)
        );
    }

    /// **Enumerate is honest in both directions**, and this is the test the
    /// sweep rests on: a development copy lists its own items and never the
    /// Creator's, and an ordinary copy lists the Creator's and never a
    /// development one.
    #[test]
    fn each_kind_of_copy_lists_its_own_items_and_no_others() {
        let real = HeldSecret::new(
            StoredKind::StoreProbe,
            Subject::issued_by_the_local_record("the-one-row"),
        );
        let development = the_item_a_copy_writes(true, &real);

        // A throwaway copy.
        assert_eq!(the_item_a_copy_reads(true, development.clone()), Some(real.clone()));
        assert_eq!(the_item_a_copy_reads(true, real.clone()), None);
        // An ordinary copy.
        assert_eq!(the_item_a_copy_reads(false, real.clone()), Some(real));
        assert_eq!(the_item_a_copy_reads(false, development), None);
    }

    /// A stand-in store holding whatever it is given, for the sweep.
    #[cfg(debug_assertions)]
    struct AStoreHolding(Mutex<Vec<HeldSecret>>);

    #[cfg(debug_assertions)]
    impl SecretStore for AStoreHolding {
        fn put(&self, _: &HeldSecret, _: Plaintext, _: Duration) -> Result<(), StoreFailure> {
            Ok(())
        }
        fn get(&self, _: &HeldSecret, _: Duration) -> Result<Plaintext, GetFailure> {
            Err(GetFailure::NotThere)
        }
        fn erase(&self, item: &HeldSecret, _: Duration) -> Result<(), StoreFailure> {
            self.0.lock().unwrap().retain(|held| held != item);
            Ok(())
        }
        fn probe(&self, _: Duration) -> ProbeOutcome {
            ProbeOutcome::RoundTripWorked
        }
        fn enumerate(&self, _: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(self.0.lock().unwrap().clone())
        }
    }

    /// **A throwaway copy never inherits the last one's secrets**, and **an
    /// ordinary copy sweeps nothing at all** — which is the direction that
    /// matters, because in an ordinary copy every item the store lists is one
    /// of the Creator's own.
    #[cfg(debug_assertions)]
    #[test]
    fn a_throwaway_copy_sweeps_its_own_items_and_an_ordinary_copy_sweeps_none() {
        let held = || {
            vec![
                HeldSecret::new(
                    StoredKind::KeyPassphrase,
                    Subject::issued_by_the_local_record("key-1"),
                ),
                HeldSecret::new(
                    StoredKind::MachinePassword,
                    Subject::issued_by_the_local_record("server-2"),
                ),
            ]
        };

        let store = AStoreHolding(Mutex::new(held()));
        assert_eq!(
            the_development_sweep(&store, LONG, true),
            TheDevelopmentSweep {
                removed: 2,
                left_behind: 0,
                the_store_would_not_say: false
            }
        );
        assert!(store.0.lock().unwrap().is_empty());

        let store = AStoreHolding(Mutex::new(held()));
        assert_eq!(
            the_development_sweep(&store, LONG, false),
            TheDevelopmentSweep::default()
        );
        assert_eq!(
            store.0.lock().unwrap().len(),
            2,
            "an ordinary copy swept the Creator's own secrets"
        );
    }

    /// **`Security and licence` E4 on Linux: an item that is already gone is
    /// reported gone**, so the sweep clears its mark rather than failing on it
    /// every run. The clear call's false is the same for *not there* and for
    /// *there and locked*, so gone is concluded only from a search that finds
    /// nothing and sets no error.
    #[test]
    fn a_linux_erase_of_an_item_already_gone_is_reported_gone() {
        assert_eq!(
            what_a_linux_erase_came_to(TheLinuxClear::RemovedNothing, || {
                TheLinuxSearch::NothingMatches
            }),
            Ok(())
        );
        assert_eq!(
            what_a_linux_erase_came_to(TheLinuxClear::Removed, || {
                panic!("a clear that removed the item needs no second look")
            }),
            Ok(())
        );
    }

    /// **And nothing else is gone.** A locked item the search still finds is
    /// an erase that did not happen (§3.3 S31), and an error from either call
    /// is *store unavailable* — §3.0 rule 5's direction, never *not there*.
    #[test]
    fn a_linux_erase_is_never_reported_gone_on_a_locked_item_or_an_error() {
        assert_eq!(
            what_a_linux_erase_came_to(TheLinuxClear::RemovedNothing, || {
                TheLinuxSearch::SomethingMatches
            }),
            Err(StoreFailure::StoreUnavailable)
        );
        assert_eq!(
            what_a_linux_erase_came_to(TheLinuxClear::RemovedNothing, || {
                TheLinuxSearch::AnError
            }),
            Err(StoreFailure::StoreUnavailable)
        );
        assert_eq!(
            what_a_linux_erase_came_to(TheLinuxClear::AnError, || {
                panic!("a clear that failed is not looked at again")
            }),
            Err(StoreFailure::StoreUnavailable)
        );
    }

    /// A call that answers inside the deadline answers.
    #[test]
    fn a_call_that_answers_in_time_answers() {
        assert_eq!(TheStoreQueue::started().within_deadline(LONG, || 7), Some(7));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn a_silent_delete_failure_or_a_matching_item_cannot_confirm_removal() {
        use apple::*;
        // An unmapped delete status returns without any read-back or prompt
        // requested by our code. The specific Mac failure remains unknown.
        let status = MacKeychain::confirm_erase(|| -4242, || panic!("delete failed; no check"));
        assert_eq!(MacKeychain::write_failure(status), StoreFailure::StoreUnavailable);
        for deleted in [ERR_SEC_SUCCESS, ERR_SEC_ITEM_NOT_FOUND] {
            assert_eq!(MacKeychain::confirm_erase(|| deleted, || ERR_SEC_ITEM_NOT_FOUND), ERR_SEC_ITEM_NOT_FOUND);
            let found_again = MacKeychain::confirm_erase(|| deleted, || ERR_SEC_SUCCESS);
            assert_eq!(MacKeychain::write_failure(found_again), StoreFailure::StoreUnavailable);
            for failed_check in [-50, ERR_SEC_NO_SUCH_KEYCHAIN, -4242] {
                assert_eq!(MacKeychain::write_failure(MacKeychain::confirm_erase(|| deleted, || failed_check)),
                    StoreFailure::StoreUnavailable);
            }
        }
        assert_eq!(MacKeychain::write_failure(MacKeychain::confirm_erase(
            || ERR_SEC_AUTH_FAILED, || panic!("delete refused; no check"))), StoreFailure::LockedOrRefused);
    }

    /// S2 and `Security and licence` S7: only *not there* asks the user again,
    /// and an unmapped status is never read as permission to.
    #[cfg(target_os = "macos")]
    #[test]
    fn only_item_not_found_asks_the_user_again() {
        use apple::*;
        assert!(MacKeychain::classify(ERR_SEC_ITEM_NOT_FOUND).asks_the_user_again());
        for status in [
            ERR_SEC_USER_CANCELED,
            ERR_SEC_INTERACTION_REQUIRED,
            ERR_SEC_INTERACTION_NOT_ALLOWED,
            ERR_SEC_AUTH_FAILED,
            ERR_SEC_NO_SUCH_KEYCHAIN,
            ERR_SEC_INVALID_KEYCHAIN,
            -1,
            -99999,
            1,
        ] {
            assert!(
                !MacKeychain::classify(status).asks_the_user_again(),
                "status {status} asked the user for a secret the store may still hold"
            );
        }
    }

    /// `Security and licence` S7's own split: the two refusal classes are told
    /// apart from the two store-absent ones, because they want different
    /// things of the user.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_refusal_is_not_the_store_being_gone() {
        use apple::*;
        assert_eq!(
            MacKeychain::classify(ERR_SEC_USER_CANCELED),
            GetFailure::LockedOrRefused
        );
        assert_eq!(
            MacKeychain::classify(ERR_SEC_NO_SUCH_KEYCHAIN),
            GetFailure::StoreUnavailable
        );
        assert_eq!(
            MacKeychain::classify(-4242),
            GetFailure::StoreUnavailable,
            "an unmapped status must be store unavailable"
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn a_refused_save_or_erase_is_not_an_unavailable_store() {
        use apple::*;
        for status in [ERR_SEC_USER_CANCELED, ERR_SEC_AUTH_FAILED,
            ERR_SEC_INTERACTION_REQUIRED, ERR_SEC_INTERACTION_NOT_ALLOWED] {
            assert_eq!(MacKeychain::write_failure(status), StoreFailure::LockedOrRefused);
        }
        for status in [ERR_SEC_NO_SUCH_KEYCHAIN, ERR_SEC_INVALID_KEYCHAIN, -4242] {
            assert_eq!(MacKeychain::write_failure(status), StoreFailure::StoreUnavailable);
        }
    }

    /// The whole contract, run against the real store on the one system there
    /// is — put, get, enumerate, erase, and the erased item reading *not
    /// there*.
    ///
    /// **It writes one item under a reserved throwaway subject and removes it**,
    /// and it touches nothing the Creator's keychain already holds. The subject
    /// is the contract's own reserved probe subject, which no local record row
    /// ever issues.
    #[cfg(target_os = "macos")]
    #[test]
    fn the_store_round_trips_on_this_machine() {
        use smoozy_security_and_licence::held::{StoredKind, Subject};
        use smoozy_security_and_licence::secret_store::PROBE_SUBJECT;

        let store = MacKeychain;
        let item = HeldSecret::new(
            StoredKind::StoreProbe,
            Subject::issued_by_the_local_record(format!("{PROBE_SUBJECT}-round-trip")),
        );
        let deadline = Duration::from_secs(10);

        // Leave nothing behind from an interrupted earlier run.
        let _ = store.erase(&item, deadline);

        store
            .put(
                &item,
                Plaintext::new(b"a throwaway value".to_vec()),
                deadline,
            )
            .expect("the store accepted the value");

        let read = store
            .get(&item, deadline)
            .expect("the store handed it back");
        assert_eq!(read.expose(|bytes| bytes.to_vec()), b"a throwaway value");

        // Put replaces rather than adding a second item.
        store
            .put(&item, Plaintext::new(b"a second value".to_vec()), deadline)
            .expect("the store replaced the value");
        let read = store
            .get(&item, deadline)
            .expect("the store handed it back");
        assert_eq!(read.expose(|bytes| bytes.to_vec()), b"a second value");

        let listed = store.enumerate(deadline).expect("the store enumerated");
        assert!(
            listed.iter().any(|held| held.subject == item.subject),
            "enumerate did not name the item that had just been written"
        );

        store
            .erase(&item, deadline)
            .expect("the store confirmed the erase");

        // `Security and licence` S7: the erased item reads *not there*, which
        // is the one class that asks the user again.
        match store.get(&item, deadline) {
            Err(failure) => assert!(failure.asks_the_user_again()),
            Ok(_) => panic!("the store handed back an erased item"),
        }
    }
}

// ===========================================================================
// Windows — Credential Manager, `CRED_TYPE_GENERIC`
// ===========================================================================

/// The Windows store — `Security and licence` §3.2 and `Desktop app and
/// delivery` S1.
///
/// **Documentation alone. No Windows machine has ever run any of this**, and
/// none is coming: `Security and licence` §14 item 8 records every claim in
/// §3.2 as permanently unresolved. What holds is the documented answer, and
/// that is what is built.
///
/// **S16: persistence is `CRED_PERSIST_LOCAL_MACHINE` for every one of the four
/// kinds, and the other two documented values are forbidden for them.** The
/// local value is *"visible to other logon sessions of this same user on this
/// same computer and not visible to logon sessions for this user on other
/// computers"*; the enterprise value would put a key passphrase, a machine
/// password or the bot token on another computer, which is the
/// boundary Row 278 draws; the session value would not survive a log off, and
/// all four kinds are held across restarts.
///
/// **S15: DPAPI is not used as a second layer**, and the reason is exact — it
/// *"merely applies cryptographic protection to the data"* and *"all
/// applications running under the same user can access any protected data that
/// they know about"* unless the application supplies entropy that it must then
/// keep on the same disk, readable by the same user.
#[cfg(target_os = "windows")]
pub struct WindowsCredentialManager;

#[cfg(target_os = "windows")]
impl WindowsCredentialManager {
    /// **S14: there is no per-application boundary, so there is nothing to
    /// grant and no refusal state.** No page read documents any prompt on
    /// `CredWrite`, `CredRead` or `CredDelete` for the generic type.
    /// `ERROR_NOT_FOUND` is *not there*; `ERROR_NO_SUCH_LOGON_SESSION` is the
    /// only documented *store unavailable*; **no refusal state is documented
    /// at all**, which is consistent — nothing asks, so nothing can be
    /// declined.
    ///
    /// **This is Row 513's Windows third: two classes and not three, and that
    /// is the honest answer rather than a manufactured difference.** Smoozy
    /// says what happened; it does not invent a *locked or refused* this
    /// system cannot produce.
    fn classify(code: u32) -> GetFailure {
        use windows_sys::Win32::Foundation::{ERROR_NOT_FOUND, ERROR_NO_SUCH_LOGON_SESSION};
        if code == ERROR_NOT_FOUND {
            GetFailure::NotThere
        } else if code == ERROR_NO_SUCH_LOGON_SESSION {
            GetFailure::StoreUnavailable
        } else {
            GetFailure::from_an_unmapped_status()
        }
    }

    fn wide(value: &str) -> Vec<u16> {
        value.encode_utf16().chain(std::iter::once(0)).collect()
    }
}

/// **S17: the blob is at most 2560 bytes**, which is Microsoft's own
/// `CRED_MAX_CREDENTIAL_BLOB_SIZE` — *"This member cannot be larger than
/// CRED_MAX_CREDENTIAL_BLOB_SIZE (5*512) bytes."*
#[cfg(target_os = "windows")]
pub const MAX_BLOB_BYTES: usize = 5 * 512;

#[cfg(target_os = "windows")]
impl WindowsCredentialManager {
    fn erase_named_item(&self, name: String, deadline: Duration) -> Result<(), StoreFailure> {
        use windows_sys::Win32::Foundation::{GetLastError, ERROR_NOT_FOUND};
        use windows_sys::Win32::Security::Credentials::{CredDeleteW, CRED_TYPE_GENERIC};

        let answer = TheStoreQueue::for_this_computer().within_deadline(deadline, move || unsafe {
            let wide_name = WindowsCredentialManager::wide(&name);
            if CredDeleteW(wide_name.as_ptr(), CRED_TYPE_GENERIC, 0) != 0 {
                None
            } else {
                Some(GetLastError())
            }
        });

        match answer {
            None => Err(StoreFailure::from_the_deadline_passing()),
            Some(None) => Ok(()),
            // An item that is not there is the state the caller asked for.
            Some(Some(code)) if code == ERROR_NOT_FOUND => Ok(()),
            Some(Some(_)) => Err(StoreFailure::from_an_unmapped_status()),
        }
    }
}

#[cfg(target_os = "windows")]
impl SecretStore for WindowsCredentialManager {
    fn put(
        &self,
        item: &HeldSecret,
        value: Plaintext,
        deadline: Duration,
    ) -> Result<(), StoreFailure> {
        use windows_sys::Win32::Foundation::{GetLastError, ERROR_NOT_FOUND};
        use windows_sys::Win32::Security::Credentials::{
            CredDeleteW, CredFree, CredReadW, CredWriteW, CREDENTIALW, CRED_PERSIST_LOCAL_MACHINE,
            CRED_TYPE_GENERIC,
        };

        // **S17's own limit, refused here rather than at the store**, so an
        // over-long value is one failure with a reason rather than whatever
        // the system happens to say.
        if value.len() > MAX_BLOB_BYTES {
            return Err(StoreFailure::from_an_unmapped_status());
        }

        // **The item as this copy names it**, so a throwaway copy's put lands
        // under a development subject and can never replace a real item.
        let item = &as_this_copy_names_it(item);
        let name = item_name(item);
        let bytes = value.expose(|bytes| bytes.to_vec());
        let undo_name = name.clone();

        // **Rule 5 as amended 2026-09-14: a put that lands after its caller
        // stopped waiting is deleted before any later call runs — unless it
        // replaced a value already held**
        // ([`TheStoreQueue::a_write_within_deadline`]).
        let answer = TheStoreQueue::for_this_computer().a_write_within_deadline(
            deadline,
            move || unsafe {
            let mut wide_name = WindowsCredentialManager::wide(&name);
            // **Whether a value is already held, asked inside the same queued
            // call and so with nothing able to run between it and the write.**
            // `CredWrite` does not say whether it replaced, so `CredRead` is
            // asked first and what it hands back is freed unread. **Any answer
            // but *not found* counts as held**, because undoing a put over a
            // value that was there would leave neither.
            let mut existing: *mut CREDENTIALW = std::ptr::null_mut();
            let replacing = if CredReadW(wide_name.as_ptr(), CRED_TYPE_GENERIC, 0, &mut existing) != 0 {
                CredFree(existing as *const core::ffi::c_void);
                true
            } else {
                GetLastError() != ERROR_NOT_FOUND
            };
            let mut blob = bytes;
            let credential = CREDENTIALW {
                Flags: 0,
                Type: CRED_TYPE_GENERIC,
                TargetName: wide_name.as_mut_ptr(),
                Comment: std::ptr::null_mut(),
                LastWritten: std::mem::zeroed(),
                CredentialBlobSize: blob.len() as u32,
                CredentialBlob: blob.as_mut_ptr(),
                Persist: CRED_PERSIST_LOCAL_MACHINE,
                AttributeCount: 0,
                Attributes: std::ptr::null_mut(),
                TargetAlias: std::ptr::null_mut(),
                UserName: std::ptr::null_mut(),
            };
            // The flags argument carries only `CRED_PRESERVE_CREDENTIAL_BLOB`,
            // which would keep the existing value; a put replaces, so it is 0.
            let failed = if CredWriteW(&credential, 0) != 0 {
                None
            } else {
                Some(GetLastError())
            };
            (failed, replacing)
            },
            |(failed, replacing)| failed.is_none() && !*replacing,
            move || unsafe {
                let wide_name = WindowsCredentialManager::wide(&undo_name);
                let _ = CredDeleteW(wide_name.as_ptr(), CRED_TYPE_GENERIC, 0);
            },
        );

        match answer {
            None => Err(StoreFailure::from_the_deadline_passing()),
            Some((None, _)) => Ok(()),
            Some((Some(_), _)) => Err(StoreFailure::from_an_unmapped_status()),
        }
    }

    fn get(&self, item: &HeldSecret, deadline: Duration) -> Result<Plaintext, GetFailure> {
        use windows_sys::Win32::Foundation::GetLastError;
        use windows_sys::Win32::Security::Credentials::{
            CredFree, CredReadW, CREDENTIALW, CRED_TYPE_GENERIC,
        };

        // **A throwaway copy reads its own items and never a real one.**
        let item = &as_this_copy_names_it(item);
        let name = item_name(item);

        let answer = TheStoreQueue::for_this_computer().within_deadline(deadline, move || unsafe {
            let wide_name = WindowsCredentialManager::wide(&name);
            let mut found: *mut CREDENTIALW = std::ptr::null_mut();
            // The flags argument is documented *"reserved and must be zero"*.
            if CredReadW(wide_name.as_ptr(), CRED_TYPE_GENERIC, 0, &mut found) == 0 {
                return Err(GetLastError());
            }
            let credential = &*found;
            let bytes = std::slice::from_raw_parts(
                credential.CredentialBlob,
                credential.CredentialBlobSize as usize,
            )
            .to_vec();
            CredFree(found as *const core::ffi::c_void);
            Ok(bytes)
        });

        match answer {
            None => Err(GetFailure::from_the_deadline_passing()),
            Some(Ok(bytes)) => Ok(Plaintext::new(bytes)),
            Some(Err(code)) => Err(Self::classify(code)),
        }
    }

    /// **S18: `CredDelete` returns TRUE or FALSE**, and that return is the
    /// confirmation. **That a following `CredRead` returns `ERROR_NOT_FOUND`
    /// is inference, not citation**, so the read-back macOS requires is *not*
    /// required here — the difference is the document's own and is not
    /// smoothed over.
    fn erase(&self, item: &HeldSecret, deadline: Duration) -> Result<(), StoreFailure> {
        self.erase_named_item(item_name(&as_this_copy_names_it(item)), deadline)
    }

    fn erase_legacy_licence_key(&self, subject: &smoozy_security_and_licence::held::Subject, deadline: Duration) -> Result<(), StoreFailure> {
        self.erase_named_item(legacy_licence_name(subject), deadline)
    }

    fn probe(&self, _deadline: Duration) -> ProbeOutcome {
        ProbeOutcome::RoundTripWorked
    }

    /// **S19: `CredEnumerate` with a prefix filter**, whose own page says
    /// *"The filter specifies a name prefix followed by an asterisk"*.
    ///
    /// **The narrowing carries doubly here**: the filter is a name prefix and
    /// not a per-application boundary, and this system has no per-application
    /// boundary at all.
    fn enumerate(&self, deadline: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
        use smoozy_security_and_licence::secret_store::{item_from_name, ITEM_NAME_PREFIX};
        use windows_sys::Win32::Foundation::{GetLastError, ERROR_NOT_FOUND};
        use windows_sys::Win32::Security::Credentials::{CredEnumerateW, CredFree, CREDENTIALW};

        let answer = TheStoreQueue::for_this_computer().within_deadline(deadline, move || unsafe {
            let filter = WindowsCredentialManager::wide(&format!("{ITEM_NAME_PREFIX}.*"));
            let mut count: u32 = 0;
            let mut entries: *mut *mut CREDENTIALW = std::ptr::null_mut();
            if CredEnumerateW(filter.as_ptr(), 0, &mut count, &mut entries) == 0 {
                return Err(GetLastError());
            }
            let mut names = Vec::new();
            for index in 0..count as usize {
                let credential = &**entries.add(index);
                if credential.TargetName.is_null() {
                    continue;
                }
                let mut length = 0usize;
                while *credential.TargetName.add(length) != 0 {
                    length += 1;
                }
                names.push(String::from_utf16_lossy(std::slice::from_raw_parts(
                    credential.TargetName,
                    length,
                )));
            }
            CredFree(entries as *const core::ffi::c_void);
            Ok(names)
        });

        match answer {
            None => Err(StoreFailure::from_the_deadline_passing()),
            // Nothing stored yet is an empty answer, not a failure.
            Some(Err(code)) if code == ERROR_NOT_FOUND => Ok(Vec::new()),
            Some(Err(_)) => Err(StoreFailure::from_an_unmapped_status()),
            // A name this application did not compose is skipped rather than
            // guessed at, which is `item_from_name`'s own rule — and so is an
            // item another kind of copy of Smoozy wrote.
            Some(Ok(names)) => Ok(names
                .iter()
                .filter_map(|name| item_from_name(name))
                .filter_map(as_this_copy_reads_it)
                .collect()),
        }
    }
}

// ===========================================================================
// Linux — the Secret Service on the session bus, through libsecret
// ===========================================================================

/// The Linux store — `Security and licence` §3.3 and `Desktop app and
/// delivery` S1.
///
/// **Documentation alone. No Linux computer exists on this project** and none
/// is coming (§14 item 9 — *"Every Linux claim in §3.3, never observed."*).
///
/// **The library is opened at run time and never linked against.** §3.3 S23's
/// load-bearing case is *a computer with nowhere to remember*, and Ubuntu's own
/// published server-image manifest carries **not even the client library** — so
/// a binary that will not start without it would fail on exactly the machines
/// the promise is written about. **A library that is not there is a store that
/// is not available**, which is a class the contract already has.
///
/// **S22: there is no single answer and the specification says so.** What a
/// secret is protected against here is whatever the installed implementation
/// does, and Smoozy claims no more.
#[cfg(target_os = "linux")]
pub struct LinuxSecretService {
    library: linux::Library,
}

/// What the Linux clear call answered.
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TheLinuxClear {
    /// It says it removed something.
    Removed,
    /// It says it removed nothing, and set no error.
    RemovedNothing,
    /// It set an error.
    AnError,
}

/// What a search for the item's own attributes answered, run after a clear
/// that removed nothing.
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TheLinuxSearch {
    /// Nothing matches, locked or unlocked, and no error was set.
    NothingMatches,
    /// Something matches — an item the clear could not unlock.
    SomethingMatches,
    /// It set an error.
    AnError,
}

/// **What a Linux erase comes to**, kept apart from the library calls so it
/// can be proved on a computer that has no Secret Service.
///
/// **A clear that removed nothing is not yet an answer**, because the
/// library's own words for it — *"Remove unlocked matching passwords"*, and a
/// return of *"Whether any passwords were removed"* — give the same false for
/// an item that is not there and for one that is there and locked. So the
/// store is asked once more, with a search that neither unlocks nor loads a
/// secret, and **only a search that finds nothing and sets no error is the item
/// gone**. That is the confirming read-back macOS makes after its own delete,
/// and it is what `Security and licence` E4 needs: the sweep clears a mark
/// when the store reports the item gone, so an item already gone must be
/// reported gone, or the mark is never cleared.
///
/// **An error from either call is *store unavailable* and never gone** —
/// §3.0 rule 5's direction. A locked item the search still finds is an erase
/// that did not happen (§3.3 S31).
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
fn what_a_linux_erase_came_to(
    cleared: TheLinuxClear,
    search_again: impl FnOnce() -> TheLinuxSearch,
) -> Result<(), StoreFailure> {
    match cleared {
        TheLinuxClear::Removed => Ok(()),
        TheLinuxClear::AnError => Err(StoreFailure::from_an_unmapped_status()),
        TheLinuxClear::RemovedNothing => match search_again() {
            TheLinuxSearch::NothingMatches => Ok(()),
            TheLinuxSearch::SomethingMatches | TheLinuxSearch::AnError => {
                Err(StoreFailure::from_an_unmapped_status())
            }
        },
    }
}

#[cfg(target_os = "linux")]
mod linux {
    //! The calls, taken from each project's own published headers: libsecret's
    //! `secret-types.h` and its API reference, and GLib's `ghash.h` and
    //! `glist.h`.

    use core::ffi::{c_char, c_int, c_void};

    pub type Pointer = *mut c_void;

    /// GLib's own `GError`, whose published shape is
    /// `{ GQuark domain; gint code; gchar *message; }`.
    #[repr(C)]
    pub struct GError {
        pub domain: u32,
        pub code: c_int,
        pub message: *mut c_char,
    }

    /// GLib's own `GList`, whose published shape is
    /// `{ gpointer data; GList *next; GList *prev; }`.
    #[repr(C)]
    pub struct GList {
        pub data: Pointer,
        pub next: *mut GList,
        pub prev: *mut GList,
    }

    /// libsecret's own `SecretError`, from that project's `secret-types.h`.
    /// **Only the locked value is mapped**; every other member is a
    /// classification this project has no computer to check, and S2's rule is
    /// that an unmapped status is *store unavailable*.
    pub const SECRET_ERROR_IS_LOCKED: c_int = 2;

    /// libsecret's own `SecretSearchFlags`, from the same header:
    /// `SECRET_SEARCH_ALL = 1 << 1`.
    pub const SECRET_SEARCH_ALL: c_int = 1 << 1;

    /// Every symbol this adapter uses, resolved once.
    pub struct Library {
        pub secret: Pointer,
        pub glib: Pointer,
        pub gobject: Pointer,

        pub password_storev_sync: unsafe extern "C" fn(
            Pointer,
            Pointer,
            *const c_char,
            *const c_char,
            *const c_char,
            Pointer,
            *mut *mut GError,
        ) -> c_int,
        pub password_lookupv_sync:
            unsafe extern "C" fn(Pointer, Pointer, Pointer, *mut *mut GError) -> *mut c_char,
        pub password_clearv_sync:
            unsafe extern "C" fn(Pointer, Pointer, Pointer, *mut *mut GError) -> c_int,
        pub password_searchv_sync:
            unsafe extern "C" fn(Pointer, Pointer, c_int, Pointer, *mut *mut GError) -> *mut GList,
        pub password_free: unsafe extern "C" fn(*mut c_char),
        pub retrievable_get_attributes: unsafe extern "C" fn(Pointer) -> Pointer,

        pub hash_table_new: unsafe extern "C" fn(Pointer, Pointer) -> Pointer,
        pub hash_table_insert: unsafe extern "C" fn(Pointer, Pointer, Pointer) -> c_int,
        pub hash_table_lookup: unsafe extern "C" fn(Pointer, Pointer) -> Pointer,
        pub hash_table_destroy: unsafe extern "C" fn(Pointer),
        pub hash_table_unref: unsafe extern "C" fn(Pointer),
        pub str_hash: Pointer,
        pub str_equal: Pointer,
        pub error_free: unsafe extern "C" fn(*mut GError),
        pub list_free: unsafe extern "C" fn(*mut GList),
        pub object_unref: unsafe extern "C" fn(Pointer),
    }

    unsafe fn symbol(handle: Pointer, name: &str) -> Option<Pointer> {
        let name = std::ffi::CString::new(name).ok()?;
        let found = libc::dlsym(handle, name.as_ptr());
        if found.is_null() {
            None
        } else {
            Some(found)
        }
    }

    unsafe fn open(names: &[&str]) -> Option<Pointer> {
        for name in names {
            let name = std::ffi::CString::new(*name).ok()?;
            let handle = libc::dlopen(name.as_ptr(), libc::RTLD_LAZY | libc::RTLD_LOCAL);
            if !handle.is_null() {
                return Some(handle);
            }
        }
        None
    }

    impl Library {
        /// **A library that is not there is not a failure to start.** The
        /// answer is `None`, and every operation is then *store unavailable*
        /// and the probe is a failed round trip — which is exactly S25's
        /// conservative reading and not a claim about why.
        pub fn open() -> Option<Library> {
            unsafe {
                // The published run-time names, most specific first. A machine
                // carrying none of them has nowhere to remember.
                let secret = open(&["libsecret-1.so.0", "libsecret-1.so"])?;
                let glib = open(&["libglib-2.0.so.0", "libglib-2.0.so"])?;
                let gobject = open(&["libgobject-2.0.so.0", "libgobject-2.0.so"])?;

                Some(Library {
                    secret,
                    glib,
                    gobject,
                    password_storev_sync: std::mem::transmute(symbol(
                        secret,
                        "secret_password_storev_sync",
                    )?),
                    password_lookupv_sync: std::mem::transmute(symbol(
                        secret,
                        "secret_password_lookupv_sync",
                    )?),
                    password_clearv_sync: std::mem::transmute(symbol(
                        secret,
                        "secret_password_clearv_sync",
                    )?),
                    password_searchv_sync: std::mem::transmute(symbol(
                        secret,
                        "secret_password_searchv_sync",
                    )?),
                    password_free: std::mem::transmute(symbol(secret, "secret_password_free")?),
                    retrievable_get_attributes: std::mem::transmute(symbol(
                        secret,
                        "secret_retrievable_get_attributes",
                    )?),
                    hash_table_new: std::mem::transmute(symbol(glib, "g_hash_table_new")?),
                    hash_table_insert: std::mem::transmute(symbol(glib, "g_hash_table_insert")?),
                    hash_table_lookup: std::mem::transmute(symbol(glib, "g_hash_table_lookup")?),
                    hash_table_destroy: std::mem::transmute(symbol(glib, "g_hash_table_destroy")?),
                    hash_table_unref: std::mem::transmute(symbol(glib, "g_hash_table_unref")?),
                    str_hash: symbol(glib, "g_str_hash")?,
                    str_equal: symbol(glib, "g_str_equal")?,
                    error_free: std::mem::transmute(symbol(glib, "g_error_free")?),
                    list_free: std::mem::transmute(symbol(glib, "g_list_free")?),
                    object_unref: std::mem::transmute(symbol(gobject, "g_object_unref")?),
                })
            }
        }
    }

    /// The three attributes an item carries.
    ///
    /// **S3 rule 1 binds hard here and this is the system it was written
    /// for**: the Secret Service specification states that *"The lookup
    /// attributes are not treated as secret material, and the service may
    /// choose not to encrypt attributes when storing them to disk"*. So the
    /// attributes are the item's composed name, its kind, and the opaque
    /// subject — **never a host, a port, a username, a file path, a key
    /// comment or a fingerprint.**
    pub const ATTRIBUTE_ITEM: &str = "smoozy-item";
    pub const ATTRIBUTE_KIND: &str = "smoozy-kind";
    pub const ATTRIBUTE_SUBJECT: &str = "smoozy-subject";
}

#[cfg(target_os = "linux")]
impl LinuxSecretService {
    /// Opens the library once. **`None` is a computer with nowhere to
    /// remember**, and every operation then answers *store unavailable*.
    pub fn open() -> Option<Self> {
        linux::Library::open().map(|library| LinuxSecretService { library })
    }

    /// **S28: a locked item is `SECRET_ERROR_IS_LOCKED`** and is therefore
    /// separable from absent. **A locked store is never reported to the user
    /// as an empty one.**
    ///
    /// **Every other code is *store unavailable*.** §3.3's own unresolved 13
    /// leaves open which error arrives when no provider is on the bus, and
    /// §14 item 9 makes every claim in that section permanently unobserved —
    /// so nothing else is mapped, and S2's rule decides the rest in the
    /// direction that never asks the user to retype a secret the store may
    /// still hold.
    fn classify(error: *mut linux::GError) -> GetFailure {
        if error.is_null() {
            return GetFailure::from_an_unmapped_status();
        }
        let code = unsafe { (*error).code };
        if code == linux::SECRET_ERROR_IS_LOCKED {
            GetFailure::LockedOrRefused
        } else {
            GetFailure::from_an_unmapped_status()
        }
    }

    fn attributes(
        &self,
        pairs: &[(&str, &str)],
    ) -> Option<(linux::Pointer, Vec<std::ffi::CString>)> {
        let library = &self.library;
        let mut kept = Vec::new();
        let table = unsafe { (library.hash_table_new)(library.str_hash, library.str_equal) };
        if table.is_null() {
            return None;
        }
        for (key, value) in pairs {
            let key = std::ffi::CString::new(*key).ok()?;
            let value = std::ffi::CString::new(*value).ok()?;
            unsafe {
                (library.hash_table_insert)(
                    table,
                    key.as_ptr() as linux::Pointer,
                    value.as_ptr() as linux::Pointer,
                )
            };
            kept.push(key);
            kept.push(value);
        }
        Some((table, kept))
    }

    /// **Whether anything still carries this item's name, locked or not** —
    /// the second look an erase takes after a clear that removed nothing.
    ///
    /// `SECRET_SEARCH_ALL` and no other flag: every match, **nothing
    /// unlocked and no secret loaded**, so a locked item is found without a
    /// prompt and without its value leaving the store. The specification's
    /// own search result carries both — *"all the items that are not locked"*
    /// and *"items that are locked"* — which is what makes an empty answer
    /// mean *not there*.
    fn search_for_the_item_named(&self, name: &str) -> TheLinuxSearch {
        let Some((table, _kept)) = self.attributes(&[(linux::ATTRIBUTE_ITEM, name)]) else {
            return TheLinuxSearch::AnError;
        };

        let mut error: *mut linux::GError = std::ptr::null_mut();
        let list = unsafe {
            (self.library.password_searchv_sync)(
                std::ptr::null_mut(),
                table,
                linux::SECRET_SEARCH_ALL,
                std::ptr::null_mut(),
                &mut error,
            )
        };
        unsafe { (self.library.hash_table_destroy)(table) };

        if !error.is_null() {
            unsafe { (self.library.error_free)(error) };
            return TheLinuxSearch::AnError;
        }

        let mut found = false;
        let mut node = list;
        while !node.is_null() {
            let retrievable = unsafe { (*node).data };
            if !retrievable.is_null() {
                found = true;
                unsafe { (self.library.object_unref)(retrievable) };
            }
            node = unsafe { (*node).next };
        }
        unsafe { (self.library.list_free)(list) };

        if found {
            TheLinuxSearch::SomethingMatches
        } else {
            TheLinuxSearch::NothingMatches
        }
    }
}

#[cfg(target_os = "linux")]
impl LinuxSecretService {
    fn erase_named_item(&self, name: String, _deadline: Duration) -> Result<(), StoreFailure> {
        let Some((table, _kept)) = self.attributes(&[(linux::ATTRIBUTE_ITEM, &name)]) else {
            return Err(StoreFailure::from_an_unmapped_status());
        };

        let mut error: *mut linux::GError = std::ptr::null_mut();
        let removed = unsafe {
            (self.library.password_clearv_sync)(
                std::ptr::null_mut(),
                table,
                std::ptr::null_mut(),
                &mut error,
            )
        };
        unsafe { (self.library.hash_table_destroy)(table) };

        let cleared = if !error.is_null() {
            unsafe { (self.library.error_free)(error) };
            TheLinuxClear::AnError
        } else if removed != 0 {
            TheLinuxClear::Removed
        } else {
            TheLinuxClear::RemovedNothing
        };
        what_a_linux_erase_came_to(cleared, || self.search_for_the_item_named(&name))
    }
}

#[cfg(target_os = "linux")]
impl SecretStore for LinuxSecretService {
    /// **The value is stored as text, and that is this system's own limit
    /// rather than a choice.** libsecret's own signature takes *"The
    /// null-terminated password to store"* — a NUL-terminated UTF-8 string —
    /// so a value that is not text, or that holds a zero byte, cannot be put
    /// here. All retained stored kinds are text; this refuses rather than
    /// truncating if one ever is not.
    fn put(
        &self,
        item: &HeldSecret,
        value: Plaintext,
        _deadline: Duration,
    ) -> Result<(), StoreFailure> {
        // **The item as this copy names it**, so a throwaway copy's put lands
        // under a development subject and can never replace a real item.
        let item = &as_this_copy_names_it(item);
        let name = item_name(item);
        let Some((table, _kept)) = self.attributes(&[
            (linux::ATTRIBUTE_ITEM, &name),
            (linux::ATTRIBUTE_KIND, item.kind.as_str()),
            (linux::ATTRIBUTE_SUBJECT, item.subject.as_str()),
        ]) else {
            return Err(StoreFailure::from_an_unmapped_status());
        };

        let text = value.expose(|bytes| std::ffi::CString::new(bytes.to_vec()).ok());
        let Some(text) = text else {
            return Err(StoreFailure::from_an_unmapped_status());
        };
        let label = std::ffi::CString::new(name.clone())
            .map_err(|_| StoreFailure::from_an_unmapped_status())?;

        let mut error: *mut linux::GError = std::ptr::null_mut();
        let stored = unsafe {
            (self.library.password_storev_sync)(
                // The schema may be null, and passing none is what keeps this
                // adapter free of a struct layout nobody here can check.
                std::ptr::null_mut(),
                table,
                // A null collection is the default collection.
                std::ptr::null(),
                label.as_ptr(),
                text.as_ptr(),
                std::ptr::null_mut(),
                &mut error,
            )
        };
        unsafe { (self.library.hash_table_destroy)(table) };

        if stored != 0 {
            Ok(())
        } else {
            unsafe {
                if !error.is_null() {
                    (self.library.error_free)(error);
                }
            }
            Err(StoreFailure::from_an_unmapped_status())
        }
    }

    fn get(&self, item: &HeldSecret, _deadline: Duration) -> Result<Plaintext, GetFailure> {
        // **A throwaway copy reads its own items and never a real one.**
        let item = &as_this_copy_names_it(item);
        let name = item_name(item);
        let Some((table, _kept)) = self.attributes(&[(linux::ATTRIBUTE_ITEM, &name)]) else {
            return Err(GetFailure::from_an_unmapped_status());
        };

        let mut error: *mut linux::GError = std::ptr::null_mut();
        let found = unsafe {
            (self.library.password_lookupv_sync)(
                std::ptr::null_mut(),
                table,
                std::ptr::null_mut(),
                &mut error,
            )
        };
        unsafe { (self.library.hash_table_destroy)(table) };

        if !found.is_null() {
            let bytes = unsafe { std::ffi::CStr::from_ptr(found) }
                .to_bytes()
                .to_vec();
            unsafe { (self.library.password_free)(found) };
            return Ok(Plaintext::new(bytes));
        }

        // **The library's own contract: *"If no secret is found then NULL is
        // returned"*.** So a null with no error set is *not there*, and a null
        // with an error set is classified — never the other way round.
        if error.is_null() {
            return Err(GetFailure::NotThere);
        }
        let class = Self::classify(error);
        unsafe { (self.library.error_free)(error) };
        Err(class)
    }

    /// **S31 and S10: an erase that could not unlock is reported as an erase
    /// that did not happen.** The library's own words for this call are
    /// *"Remove unlocked matching passwords"* and the return is *"Whether any
    /// passwords were removed"* — so a locked item is not removed and the
    /// return says so. **A false return with no error is therefore not yet an
    /// answer, and a search that neither unlocks nor loads a secret decides
    /// it** — see [`what_a_linux_erase_came_to`]. Corrected 2026-09-18: until
    /// then a false return was always a failure, so an item already gone was
    /// never reported gone and `Security and licence` E4's sweep could never
    /// clear its mark.
    fn erase(&self, item: &HeldSecret, deadline: Duration) -> Result<(), StoreFailure> {
        self.erase_named_item(item_name(&as_this_copy_names_it(item)), deadline)
    }

    fn erase_legacy_licence_key(&self, subject: &smoozy_security_and_licence::held::Subject, deadline: Duration) -> Result<(), StoreFailure> {
        self.erase_named_item(legacy_licence_name(subject), deadline)
    }

    /// **`Security and licence` S5 and its §3.3 S24: the probe is the
    /// adapter's and is taken at first use
    /// in a run, not at start.**
    ///
    /// *Is there somewhere to remember at all* is answered by writing a
    /// throwaway item under a reserved subject, reading it back, and erasing
    /// it. **What a probe establishes is whether a round trip worked, and
    /// nothing beyond that** — whether a failed round trip was no service on
    /// the bus, or a service that refused or locked, is what §14 item 7 leaves
    /// permanently open, and nothing here claims the probe separates them.
    fn probe(&self, deadline: Duration) -> ProbeOutcome {
        probe_the_linux_store(self, deadline)
    }

    /// **S2's Linux half: a search on this application's own attribute set.**
    ///
    /// The result the specification describes is already split by state —
    /// *"The `unlocked` return value will contain the object paths of all the
    /// items that are not locked. The `locked` return value will contain
    /// object paths of items that are locked"* — and this call's own flags
    /// carry that: `SECRET_SEARCH_ALL` returns every match rather than the
    /// first, and neither unlocking nor loading the secrets is asked for,
    /// because **enumerate hands back subjects and no values at all**
    /// (§3.0 rule 6).
    fn enumerate(&self, _deadline: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
        use smoozy_security_and_licence::held::{StoredKind, Subject};

        let mut found = Vec::new();
        for kind in StoredKind::ALL {
            let Some((table, _kept)) = self.attributes(&[(linux::ATTRIBUTE_KIND, kind.as_str())])
            else {
                return Err(StoreFailure::from_an_unmapped_status());
            };

            let mut error: *mut linux::GError = std::ptr::null_mut();
            let list = unsafe {
                (self.library.password_searchv_sync)(
                    std::ptr::null_mut(),
                    table,
                    linux::SECRET_SEARCH_ALL,
                    std::ptr::null_mut(),
                    &mut error,
                )
            };
            unsafe { (self.library.hash_table_destroy)(table) };

            if !error.is_null() {
                unsafe { (self.library.error_free)(error) };
                return Err(StoreFailure::from_an_unmapped_status());
            }

            let subject_key = std::ffi::CString::new(linux::ATTRIBUTE_SUBJECT)
                .map_err(|_| StoreFailure::from_an_unmapped_status())?;

            let mut node = list;
            while !node.is_null() {
                let retrievable = unsafe { (*node).data };
                if !retrievable.is_null() {
                    let attributes =
                        unsafe { (self.library.retrievable_get_attributes)(retrievable) };
                    if !attributes.is_null() {
                        let value = unsafe {
                            (self.library.hash_table_lookup)(
                                attributes,
                                subject_key.as_ptr() as linux::Pointer,
                            )
                        };
                        if !value.is_null() {
                            let subject = unsafe {
                                std::ffi::CStr::from_ptr(value as *const core::ffi::c_char)
                            }
                            .to_string_lossy()
                            .into_owned();
                            // **Only the items this copy would have written.**
                            // This system's search is by kind rather than by
                            // name, so both kinds of copy's items come back
                            // here and the telling apart is this line's.
                            if let Some(held) = as_this_copy_reads_it(HeldSecret::new(
                                kind,
                                Subject::issued_by_the_local_record(subject),
                            )) {
                                found.push(held);
                            }
                        }
                        unsafe { (self.library.hash_table_unref)(attributes) };
                    }
                    unsafe { (self.library.object_unref)(retrievable) };
                }
                node = unsafe { (*node).next };
            }
            unsafe { (self.library.list_free)(list) };
        }
        Ok(found)
    }
}

#[cfg(test)]
mod retired_key_and_linux_probe_tests {
    use super::*;
    use smoozy_security_and_licence::held::{StoredKind, Subject};
    use std::cell::RefCell;

    #[test]
    fn the_retired_item_keeps_its_shipped_name_and_the_copy_boundary() {
        let subject = Subject::issued_by_the_local_record(crate::record::key_rows::LEGACY_LICENCE_KEY_SUBJECT);
        assert_eq!(smoozy_security_and_licence::secret_store::LEGACY_LICENCE_KEY_NAME, "Smoozy.licence-key");
        assert_eq!(legacy_subject_for_copy(&subject, false), "licence-record");
        assert_eq!(legacy_subject_for_copy(&subject, true), "development.licence-record");
        assert_eq!(legacy_licence_name(&subject), format!("Smoozy.licence-key.{}", legacy_licence_subject(&subject)));
        assert!(smoozy_security_and_licence::secret_store::item_from_name("Smoozy.licence-key.licence-record").is_none());
    }

    struct ProbeStore { calls: RefCell<Vec<&'static str>>, refuse_put: bool, refuse_get: bool }
    impl ProbeStore {
        fn item(&self, item: &HeldSecret, deadline: Duration) {
            assert_eq!(item.kind, StoredKind::StoreProbe);
            assert_eq!(item.subject.as_str(), "probe");
            assert_eq!(item_name(item), "Smoozy.store-probe.probe");
            assert_eq!(deadline, Duration::from_secs(2));
        }
    }
    impl SecretStore for ProbeStore {
        fn put(&self, item: &HeldSecret, value: Plaintext, deadline: Duration) -> Result<(), StoreFailure> {
            self.item(item, deadline);
            value.expose(|bytes| assert_eq!(bytes, b"probe"));
            self.calls.borrow_mut().push("put");
            if self.refuse_put { Err(StoreFailure::LockedOrRefused) } else { Ok(()) }
        }
        fn get(&self, item: &HeldSecret, deadline: Duration) -> Result<Plaintext, GetFailure> {
            self.item(item, deadline);
            self.calls.borrow_mut().push("get");
            if self.refuse_get { Err(GetFailure::LockedOrRefused) } else { Ok(Plaintext::new("probe")) }
        }
        fn erase(&self, item: &HeldSecret, deadline: Duration) -> Result<(), StoreFailure> {
            self.item(item, deadline);
            self.calls.borrow_mut().push("erase");
            Ok(())
        }
        fn probe(&self, _: Duration) -> ProbeOutcome { unreachable!() }
        fn enumerate(&self, _: Duration) -> Result<Vec<HeldSecret>, StoreFailure> { unreachable!() }
    }

    #[test]
    fn the_linux_probe_uses_its_own_kind_and_erases_after_the_read() {
        let store = ProbeStore { calls: RefCell::new(Vec::new()), refuse_put: false, refuse_get: false };
        assert_eq!(probe_the_linux_store(&store, Duration::from_secs(2)), ProbeOutcome::RoundTripWorked);
        assert_eq!(*store.calls.borrow(), vec!["put", "get", "erase"]);
    }

    #[test]
    fn the_linux_probe_reports_refusal_and_cleans_up_after_a_failed_read() {
        let store = ProbeStore { calls: RefCell::new(Vec::new()), refuse_put: false, refuse_get: true };
        assert_eq!(probe_the_linux_store(&store, Duration::from_secs(2)), ProbeOutcome::RoundTripFailed);
        assert_eq!(*store.calls.borrow(), vec!["put", "get", "erase"]);
        let store = ProbeStore { calls: RefCell::new(Vec::new()), refuse_put: true, refuse_get: false };
        assert_eq!(probe_the_linux_store(&store, Duration::from_secs(2)), ProbeOutcome::RoundTripFailed);
        assert_eq!(*store.calls.borrow(), vec!["put"]);
    }
}

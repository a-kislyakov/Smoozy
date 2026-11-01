//! **Keys, in its two readings** — the list in the rail, and the same list
//! opened from a server dialog to choose a key for one server. The page half is
//! `web/keys.js`.
//!
//! **This file is the surface and not the deciding.** Every judgement below is
//! `Keys`', in `App/Keys/`, and every act that touches this computer is
//! `crate::keys`'. What is here is the translation: read the record, ask that
//! module what it means, compose the words, and put them on the one ordered
//! queue.
//!
//! ## What this piece draws, cited the way the runner ruled on 2026-09-12
//!
//! **The Design document and the heading**: `Design/Keys`, whose sections this
//! file names at each site. **And the mockup state names**, which are the
//! `<option>` values of `Preparation/Design/Mockup/Smoozy.html`'s own state
//! list and are what this half is counted by. **Twenty-one, and the drawing
//! has no twenty-second in the `keys-` set:**
//!
//! `keys`, `keys-checking`, `keys-notool`, `keys-nopub`, `keys-rm`, `keys-adv`,
//! `keys-advfall`, `keys-made`, `keys-madenostore`, `keys-noread`, `keys-bad`,
//! `keys-type`, `keys-perm`, `keys-wrong`, `keys-here`, `keys-same`,
//! `keys-pass`, `keys-pass2`, `keys-nostore`, `keys-pick`, `keys-picknone`.
//!
//! *(Nineteen until 2026-09-18: `keys-nostore` and `keys-madenostore` —
//! `Security and licence` S34's sentence in the box and in the new-key
//! dialog — were drawn for step 10b's first audit round, its second upstream item.)*
//!
//! ## Four kinds, and each one is a different question
//!
//! 1. **[`THE_KEYS_SCREEN`]** — the list and every word on it, composed from the
//!    local record alone and posted before a single reading is taken. That is
//!    `Keys` U10a: *"A list draws first and each key's verdict fills in behind
//!    it; no draw waits on a reading."*
//! 2. **[`A_KEYS_VERDICT`]** — one per key, arriving behind the list. It is its
//!    own kind because it arrives per key and after the list, which is exactly
//!    what `Keys` U10a's *"the answers arrive one key at a time"* means on the queue.
//! 3. **[`AN_ANSWER`]** — the answer to the act the user just took: a refusal,
//!    a statement, the types the dialog may offer, the file a generation wrote,
//!    the public half on its way to the clipboard, what taking a key away costs.
//!    **It is on the event and nowhere durable**: `Keys` R2 keeps no durable store
//!    beyond the key row, and a refusal is an answer to a press rather than a
//!    state.
//! 4. **[`THE_PASSPHRASE_ASK`]** — `Keys` S14's box, which is **not this
//!    screen's**. `Design/Keys § Where the passphrase is asked for`: *"the box
//!    is the connection's, not this screen's"*, and the drawing puts it over the
//!    server list. It is a kind of its own because it is raised by a connect the
//!    user started and reaches the user wherever they are standing.
//!
//! ## What can actually happen today, said plainly
//!
//! **Every act here is local and every one of them runs whole**, which is what
//! uses real tools and locations. Real `ssh-keygen` runs, a real key location, the real macOS
//! keychain, the real askpass helper.
//!
//! **One thing cannot be reached and it is not this piece's.** `Keys` S14 raises
//! the passphrase box **on a connect the user started and nowhere else**, and
//! nothing in Smoozy makes a connection yet — `Servers`' transport is step 4's
//! and its screen is piece 3's. So [`the_passphrase_is_asked_for`] is the door,
//! it is written and tested, and **no caller exists**. The contract for whoever
//! builds that connect is in this piece's account.
//!
//! ## Two things this file must never do
//!
//! - **Never join a value into a sentence.** X7: the typography point never
//!   touches a job's name, a server's name, a path, a host or a port. **A key's
//!   name is the user's own comment and a fingerprint is a value they copy**, so
//!   both travel by the verbatim route and the words beside them travel as
//!   prose. The drawing writes *Another key is already called Studio MacBook Pro* as one string; here it is two.
//! - **Never read a stored secret to draw anything.** `Security and licence`
//!   §3.0 rule 6, and `Keys` U9: whether a passphrase is held is answered from
//!   **the row's own reference** and never by reaching the store, because a
//!   store read can raise a system unlock prompt and `Servers` P1 forbids one.
//!
//! ## This file carries no header list of other documents' markers, and that is
//! forced rather than chosen.
//!
//! **Nearly every marker below belongs to `Keys`, and this crate's own plan
//! defines a clause of the same short name for very nearly all of them** — a
//! G1, an L1, an `Keys` N1, an R1, an S5, a T2, a `Keys` U4, a W1 and many more. A header list
//! cannot tell the two apart, so **every one is written out with its document at
//! the site**, and a bare marker below means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`.

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;

use tauri::{AppHandle, Manager, Runtime};

use smoozy_keys::fingerprint::Fingerprint;
use smoozy_keys::listing::{self};
use smoozy_keys::naming;
use smoozy_keys::ownership::{self, Ownership, SavedServer, UsedBy};
use smoozy_keys::passphrase::{HowTheKeyIsNamed, ServerContext, TheAsk};
use smoozy_keys::pointing::{DialogContext, Offered, Refused};
use smoozy_keys::removal::{Copy, WhyNot};
use smoozy_keys::rows::KeyRow;
use smoozy_keys::types::{KeyType, OfferedSet};
use smoozy_keys::usable::{self, Readings, Unusable, Verdict, WhyItWouldNotRead};
use smoozy_security_and_licence::plaintext::Origin;
use smoozy_security_and_licence::secret_store::SecretStore;
use smoozy_state::identities::{Identity, IdentityKind};

use crate::events::{Event, Said, Subject as About, TheOneQueue};
use crate::keys::acts::{self, Mended};
use crate::keys::askpass::TheRoute;
use crate::keys::making::{self, ThePair};
use crate::keys::readings::OnThisComputer;
use crate::keys::supply;
use crate::platform::locations::{self, Location};
use crate::record::{key_rows, server_rows};
use crate::start::Started;
use crate::typography::{Prose, Verbatim};

/// **The list and its words.** Declared here beside the service that posts it
/// and copied into `web/keys.js`; nothing holds the two together, so copy
/// carefully.
pub const THE_KEYS_SCREEN: &str = "keys:the-screen";

/// **One key's verdict, arriving behind the list** — `Keys` U10a.
pub const A_KEYS_VERDICT: &str = "keys:a-verdict";

/// **The answer to the act the user just took.**
pub const AN_ANSWER: &str = "keys:an-answer";

/// **`Keys` S14's box**, raised by a connect the user started.
pub const THE_PASSPHRASE_ASK: &str = "keys:the-passphrase-ask";

/// **The name `Keys` declares its diary lines under.** `crate::keys::THE_MODULE`
/// is the same string and `crate::start` registers it; a module with no
/// declaration writes nothing (`Desktop app and delivery` A7).
pub const THE_MODULE: &str = crate::keys::THE_MODULE;

/// **How long a store call is waited on.** `Security and licence` §3.0 rule 5:
/// no store call is waited on without a deadline. Five seconds for a call the
/// user is standing in front of.
pub(crate) const THE_STORE_DEADLINE: Duration = Duration::from_secs(5);

/// **The longest name the make-key dialog will carry** — H34's *validates
/// lengths before touching an adapter*. `Keys` G1 passes the comment to `-C` as
/// one argument and *"the user can type anything"*, so this is a bound on a text
/// field and never a check on its shape.
const THE_LONGEST_A_NAME_MAY_BE: usize = 4 * 1024;

/// **The longest passphrase the box will carry.** Same rule, same reason. It is
/// not a strength check and it is not a format: `Keys` holds no opinion about
/// what a passphrase looks like.
const THE_LONGEST_A_PASSPHRASE_MAY_BE: usize = 8 * 1024;

// ───────────────────────────────────────────────────────────────────────────
// The words. Every one is the drawing's, and where the drawing has none, whose
// ruling it is and why is written at the site.
// ───────────────────────────────────────────────────────────────────────────

// ── The head and the foot — the drawing's `.svhead` and `.svfoot` ──────────

const KEYS: &str = "Keys";
const CHOOSE_A_KEY: &str = "Choose a key…";
const NEW_KEY: &str = "New key…";
/// `Design/Keys § Opened from a server dialog`. **Where nothing names the
/// server the line reads *Choosing a key* and stops** — settled at Design,
/// 2026-08-24, and mockup state `keys-picknone`.
const CHOOSING_A_KEY: &str = "Choosing a key";
/// **The line where something names the server, as one piece of prose**, and
/// the server's name travels beside it as a value. Mockup state `keys-pick`.
/// **It was two strings the page joined** until step 10a round 1, fix-here
/// item 30.
const CHOOSING_A_KEY_FOR: &str = "Choosing a key for";
const WHAT_GOES_TO_A_PROVIDER: &str = "Paste the public key at your provider.";
const CANCEL_AND_GO_BACK: &str = "Cancel";

// ── `Design/Keys § On a computer without the SSH tools` ────────────────────
//
// **Two sentences and two strings** (X10): the point binds the last
// inter-word space of a piece of prose, so a paragraph handed over whole leaves
// every interior sentence ending unbound. Word for word the drawing's, and word
// for word the line the server list carries.

// **The server list says the same two**, and takes them from here rather
// than keeping a copy (step 10a round 1, fix-here item 30).
pub(crate) const THE_TOOLS_ARE_NOT_INSTALLED: &str = "SSH tools are not installed on this computer.";
pub(crate) const UNTIL_THEY_ARE: &str = "No key can be created or checked until they are.";

// ── A row — `Design/Keys § A row` ──────────────────────────────────────────

const USED_BY: &str = "Used by";
/// Between two server names. Its own string because a name is a value on either
/// side of it (X7).
const AND: &str = "and";
const A_PASSPHRASE_IS_REMEMBERED: &str = "Passphrase saved";
const CLEAR_IT: &str = "Clear";
const COPY_PUBLIC_KEY: &str = "Copy public key";
/// **`Design/Keys § The two kinds`**: *"A key Smoozy manages offers Delete. A
/// key it only references offers Remove."*
const DELETE: &str = "Delete";
const REMOVE: &str = "Remove";
/// **What each says on hover** — `Design/Keys § The two kinds`, word for word;
/// Creator-stated 2026-10-08, sixth notes, note 7. The page puts each on its
/// own button as the hover line and holds no copy. **Each says why, too:
/// where the key sits** — the Creator's eighth notes, note 1, changed
/// 2026-10-12. They read *Deletes the key files from your computer.* and
/// *Removes the key from Smoozy. Its files stay where they are.*
const DELETE_SAYS_ON_HOVER: &str = "Deletes the key files from your .ssh folder.";
const REMOVE_SAYS_ON_HOVER: &str =
    "Removes the key from Smoozy. Its files are not deleted.";
const USE_THIS_KEY: &str = "Use this key";

/// **The label the underway mark holds the width with** — `Design/Keys § While
/// a key is being checked`. Mockup state `keys-checking`.
/// **Crate-visible because a server dialog's key list says it too** — the
/// Creator's eighth notes, note 3: one fact, one wording.
pub(crate) const CHECKING: &str = "Checking…";

// ── The five unusable reasons, and the two situations carried beside one ───
//
// **`Keys` U6's closed set is five and each is the clause that failed.** The
// drawing gives four of the sentences at its own pointing refusals — one cause,
// one sentence, wherever it is said — and the fifth and the two situations are
// ruled here under the Creator's standing delegation of 2026-08-31, with the
// reasoning in this piece's account.

/// Clause 1. The drawing's own, at the unusable row.
const THE_FILE_IS_NOT_THERE: &str = "File not found.";
/// Clause 2 or clause 4a, where the readings could say nothing more. The
/// drawing's own, at `keys-noread`.
const THE_FILE_COULD_NOT_BE_READ: &str = "Could not read the file.";
/// Clause 2, with the situation carried beside it. The drawing's own, at
/// `keys-bad`.
const IT_IS_NOT_A_PRIVATE_KEY: &str = "Not a recognised private key.";
/// Clause 3. The drawing's own, at `keys-type`.
const NOT_A_SUPPORTED_TYPE: &str = "Unsupported key type.";
/// Clause 5. The drawing's own, at `keys-perm`.
const THE_PERMISSIONS_ARE_TOO_OPEN: &str = "Permissions too open.";
/// **Clause 5's second sentence, and it is the one clause on this screen where
/// Smoozy knows what to do and says it** — `Design/Keys § Which of this screen's
/// bad answers is a failure`. The drawing's own.
const MEND_THE_PERMISSIONS: &str = "Make the file readable only by you.";

/// **Clause 4b, and the drawing has no row line for it.** Ruled here: it says
/// what changed and asserts nothing about how, which is all the record carries.
/// The two ways out are the same two every unusable row has, and *Choose a
/// replacement* is the deliberate route for a key that really is different.
const A_DIFFERENT_KEY_IS_THERE: &str = "A different key is at this path now.";

/// **`Design/Keys § When there is no public key file beside the key`**, for a
/// key with no passphrase. Two sentences, the drawing's own. Mockup state
/// `keys-nopub`.
const NO_PUBLIC_KEY_FILE: &str = "No public key file beside this key.";
const THE_KEY_ITSELF_WORKS: &str = "The key itself works.";

/// **The one open Design hole in the project, ruled 2026-09-13** under the
/// Creator's standing delegation of 2026-08-31, the Design stage being closed.
/// It was the unfinished marker at `Preparation/Design/Keys.md` line 80 — the
/// refusal for a protected key with no public file beside it — and it is closed
/// there in these words. **The marker itself is not quoted here**: a quotation
/// of a closed one reads to the stage's own sweep exactly like an open one.
///
/// **The paragraph fixed what it must say** — the key has a passphrase, there is
/// no public key file beside it, and bringing that file across is what fixes it
/// — so what was open was the wording. **The shape is the one the paragraph
/// above it already uses** for a key with no passphrase. **It names no act the
/// user is told to take**, because Smoozy does not know where that file is.
/// **And it does not read as a fault of the key**, which is the whole reason
/// `Keys` carries the situation beside the verdict rather than letting the row
/// say *the file could not be read*.
const IT_IS_PROTECTED_AND_HAS_NO_PUBLIC_HALF: &str =
    "This key has a passphrase and no public key file beside it.";
const UNTIL_THAT_FILE_IS_BESIDE_IT: &str = "It cannot be checked until that file is beside it.";

// ── The two ways out of an unusable row ────────────────────────────────────

const CHOOSE_ITS_NEW_LOCATION: &str = "Choose its new location…";
const CHOOSE_A_REPLACEMENT: &str = "Choose a replacement…";
/// **What a replacement costs, said before it is pressed** — `Keys` F9 and F12.
/// The drawing's own.
const WHAT_A_REPLACEMENT_COSTS: &str =
    "A replacement is a different key: the saved passphrase is cleared, and every server using this key needs a new key.";

// ── Making a key — `Design/Keys § Making a key` ────────────────────────────

const NEW_KEY_HEADING: &str = "New key";
const THE_NAME_FIELD: &str = "Name";
const NAME_IT_AFTER_THE_PROVIDER: &str = "Use your provider's name.";
const THE_PASSPHRASE_FIELD: &str = "Passphrase";
const OPTIONAL: &str = "optional";
const ADVANCED_KEY_TYPE: &str = "Advanced — key type";
const KEY_TYPE: &str = "Key type";
/// **`Keys` T4's default, marked as the default and nothing more** — not
/// *recommended*, which implies a comparison nobody made.
const SMOOZYS_USUAL_CHOICE: &str = "Default";
/// **Where the usual one cannot be made**, the next in the fixed order stands
/// and this marks it. The drawing's own, at `keys-advfall`.
const SMOOZYS_CHOICE_HERE: &str = "Default here";
/// **The line a user who opened Advanced is owed**, with the type's own name
/// travelling beside it as a value — the drawing writes it as one string and
/// X7 will not have a name inside a sentence.
const IS_THE_USUAL_CHOICE_AND_CANNOT_BE_MADE: &str =
    "is the default and this computer cannot create it.";
const MAKE_IT_AND_COPY: &str = "Create and copy public key";
const CANCEL: &str = "Cancel";
const DONE: &str = "Done";
/// The dialog's ending, at `keys-made`. The path travels beside it as a value.
const THE_KEY_IS_IN: &str = "Saved to";
const ITS_PUBLIC_KEY_IS_COPIED: &str = "Public key copied.";

/// **`Security and licence` S34's one sentence, said once for the life of the
/// installation** — `Preparation/Design/Keys.md`, *On a computer with nowhere to
/// keep a passphrase*, word for word. **One home for it**: the admin access box
/// on Set up and the All servers row take it from here. Mockup states
/// `keys-nostore`, `keys-madenostore`, `pw-nostore` and `sv-nostore`.
pub(crate) const NOWHERE_TO_KEEP_THEM: &str = "Passwords and passphrases cannot be saved on this computer.";
const PASTE_IT_AT_THE_PROVIDER: &str = "Paste it at your provider.";
/// **`Keys` G4: the act is reported failed, plainly.** Words ruled here; the
/// clause requires the telling and names none.
const THE_KEY_COULD_NOT_BE_MADE: &str = "Could not create the key.";

/// **A press this dialog's own bounds refuse is answered where it was
/// pressed** — step 10a round 1, fix-here item 32, whose *New key* half was
/// found undone by that round's close check and repaired on 2026-09-17. The
/// first two sentences are that Verdict's own; the third is this repair's call,
/// in the same shape and for the same reason. Design's owed words.
///
/// **None of the three is a check on shape.** `Keys` G1 passes the comment to
/// `-C` verbatim and holds no opinion about what a passphrase looks like, so
/// the first two say a bound on a text field was passed and nothing more, and
/// the third says a closed set of three was not chosen from.
const THIS_NAME_IS_TOO_LONG: &str = "This name is too long.";
const THIS_PASSPHRASE_IS_TOO_LONG: &str = "This passphrase is too long.";
const CHOOSE_A_KEY_TYPE: &str = "Choose a key type.";

// ── Pointing at something — `Design/Keys § When the user points at…` ───────

const THAT_FILE_COULD_NOT_BE_READ: &str = "File unreadable";
const THAT_IS_NOT_A_KEY: &str = "Not a readable key";
const SMOOZY_CANNOT_USE_THAT_KEY: &str = "Key cannot be used";
/// The heading over the protected-with-no-public-half refusal. Ruled with the
/// two sentences above, in the same shape as its neighbours.
const SMOOZY_CANNOT_CHECK_THAT_KEY: &str = "Key cannot be checked";
const NOTHING_HAS_CHANGED: &str = "Nothing changed.";
const BACK: &str = "Back";

const THAT_IS_A_KEY_AND_NOT_THIS_ONE: &str = "Different key";
const THIS_ROW_IS_ABOUT_A_DIFFERENT_KEY: &str = "This is not the key in this row.";
const IT_HOLDS: &str = "It holds";
const YOU_POINTED_AT: &str = "You pointed at";
const USE_CHOOSE_A_REPLACEMENT: &str =
    "To use the key you pointed at, use Choose a replacement.";

/// **`Keys` F5's refusal, said in those words**: two different blobs under one
/// fingerprint. The drawing has no state for it and it is ruled here; §13 item 1
/// carries the residual that makes it reachable at all.
const TWO_KEYS_SMOOZY_CANNOT_TELL_APART: &str = "Duplicate fingerprint";
const TWO_KEYS_ONE_FINGERPRINT: &str =
    "Same fingerprint as a different key in the list.";

const SMOOZY_ALREADY_HAS_THAT_KEY: &str = "Key already added";
const IT_IS_IN_THE_LIST_AS: &str = "It is listed as";
const AT: &str = "at";

/// **`Design/Keys § Where two keys share a name`.** A statement, not a refusal.
/// The name travels beside it as a value. Mockup state `keys-same`.
const ANOTHER_KEY_IS_ALREADY_CALLED: &str = "Another key is already called";
const THE_ONE_ALREADY_IN_THE_LIST_HOLDS: &str = "The one already in the list holds";
const BOTH_ARE_KEPT: &str = "Both are kept.";

// ── Copying — `Design/Keys § When there is no public key file beside…` ─────

/// **The public half is on its way to the clipboard, with `Keys` W7a's
/// comparison run and agreeing.**
const THE_PUBLIC_KEY_IS_COPIED: &str = "Public key copied.";
/// **The unfinished marker at `App/Keys/src/removal.rs` — the wording for copy
/// saying what it could not check — ruled 2026-09-13** under the Creator's
/// standing delegation of 2026-08-31.
///
/// **`Keys` U7b makes this the ordinary case for a protected key and not an
/// edge**: the private half's own read is refused for every one of them, so the
/// `.pub` beside the file cannot be compared against it. **The copy still
/// happens** — that is the honest ending §13 item 6 already takes — and this
/// says what was not checked without calling the key or the file wrong, because
/// nothing here establishes that either is.
const THE_COMPARISON_COULD_NOT_BE_RUN: &str =
    "Public key not checked against this key: it has a passphrase.";
/// **Where the copy did not report success, the screen says so and shows the
/// key** — it never claims a copy it cannot vouch for (step 10a round 1,
/// fix-here item 25). That round's call; Design's owed words.
const THE_PUBLIC_KEY_WAS_NOT_COPIED: &str =
    "Could not copy the public key. Copy it from here:";
const SELECT_THE_WHOLE_KEY: &str = "Select the whole key";
/// **`Keys` W7a's refusal**: the `.pub` beside it is a different key's. The
/// drawing has no state for it and it is ruled here, in the same shape as an
/// absent `.pub`.
const A_DIFFERENT_KEYS_PUBLIC_HALF: &str =
    "The public key file beside this key does not match it.";

// ── Taking a key away — `Design/Keys § Taking a key away` ──────────────────

const DELETE_THIS_KEY: &str = "Delete this key?";
const REMOVE_THIS_KEY: &str = "Remove this key?";
/// The drawing's two sentences, one per count. **Two strings and never one with
/// a number joined in** (X7).
const ONE_SERVER_CONNECTS_WITH_IT: &str =
    "One server uses it and will need a new key.";
const SERVERS_CONNECT_WITH_IT: &str =
    "These servers use it and will each need a new key.";
const THAT_COVERS_WHAT_SMOOZY_KNOWS: &str = "Only Smoozy's servers are listed.";
const OTHER_TOOLS_AND_COMPUTERS: &str = "Other apps and computers may also use this key.";
/// **`Keys` W6: the confirmation names the file set by full path.** The drawing
/// names the folder instead; the clause wins under Rule 10, and the paths travel
/// as values.
const THESE_FILES_ARE_DELETED: &str = "These files are deleted:";
/// **What a delete, a remove or a clear came to, where it did not all go** —
/// `Keys` W9. Step 10a round 1's call, fix-here item 18; Design's owed words.
/// The path travels beside the first as a value (X7).
const THIS_FILE_WAS_NOT_DELETED: &str = "This file was not deleted:";
const THE_KEY_WAS_NOT_DELETED: &str = "The key was not deleted.";
const THE_PASSPHRASE_WAS_NOT_CLEARED: &str = "The saved passphrase was not cleared.";
/// **`Keys` W12's third state, and it never reads as good news.**
const COULD_NOT_CHECK_THE_SAVED_SERVERS: &str = "Could not check which servers use this key.";

// ── The passphrase box — `Design/Keys § Where the passphrase is asked for` ─

/// The heading, with the key's name or its fingerprint beside it as a value.
const PASSPHRASE_FOR: &str = "Passphrase for";
const CONNECTING_TO: &str = "Connecting to";
/// **`Keys` S19's line, bounded by that clause and worded by Design.** Two
/// sentences and two strings, and no cause named. The drawing's own, at
/// `keys-pass2`.
const THE_REMEMBERED_PASSPHRASE_NO_LONGER_OPENS_IT: &str =
    "The saved passphrase no longer works.";
const SMOOZY_HAS_FORGOTTEN_IT: &str = "It was cleared.";
const CONNECT: &str = "Connect";

// ───────────────────────────────────────────────────────────────────────────
// The reading
// ───────────────────────────────────────────────────────────────────────────

/// **Which reading of the screen this is** — `Design/Keys § Opened from a
/// server dialog`, and `Keys` U11.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheMode {
    /// The standalone list reached from the rail. Mockup state `keys`.
    Housekeeping,
    /// **Opened from a server dialog to pick a key.** The same list with no
    /// unusable key offered, plus a display-only context the caller hands over.
    /// **No rule of this screen turns on that context** (`Keys` U11). Mockup
    /// states `keys-pick` and `keys-picknone`.
    Picking {
        /// What the caller already holds, handed as text and nothing else.
        /// **Any or all of it may be empty**, which is the Add Server route, and
        /// nothing is composed for a server that has no name yet.
        naming: Option<String>,
    },
}

/// One row as the list draws it, before any reading has answered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARow {
    /// The local key identifier, written. **A value the page routes on** and
    /// never a word.
    pub identifier: String,
    /// **`Keys` R1's name as last read.** A value: it is the key's comment, the
    /// user's own text, and X7 keeps the point off it.
    pub name: Option<String>,
    pub fingerprint: Option<String>,
    pub path: String,
    /// **`Keys` W11a's line**, taken once for the whole list at this draw.
    pub used_by: Vec<AServerUsingIt>,
    /// **`Keys` W12's third state.** It never arises on this screen — the key rows and
    /// the server rows come out of one read — and it is carried so that a
    /// reader can see the answer rather than assume it.
    pub the_saved_servers_could_not_be_read: bool,
    /// **`Keys` U9's marker, read from the row and never from the store.**
    pub a_passphrase_is_remembered: bool,
    /// **`Keys` W2's derivation, computed at the draw and stored nowhere.**
    /// *Delete* where it is managed, *Remove* where it is borrowed.
    pub managed: bool,
}

/// One saved server on a row's *Used by* line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AServerUsingIt {
    /// **The name, or `username@host` where the server has none** —
    /// `Design/Servers § The list`.
    pub called: String,
    /// The stored colour, where it is one of the drawing's eight.
    pub badge: Option<&'static str>,
}

/// **The whole of what the list hands the page** — H34's closed response type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheKeysScreen {
    pub mode: TheMode,
    /// **`Keys` U6a and G6: said once, above the list, and never per row.**
    /// Mockup state `keys-notool`.
    pub the_tools_are_missing: bool,
    /// **Ordered by name** — `Design/Keys § The list's order`, which puts two
    /// rows sharing a name next to each other, where the fingerprint is worth
    /// reading.
    pub rows: Vec<ARow>,
}

/// **One key's verdict, behind the list** — `Keys` U10a.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AVerdict {
    pub row: String,
    pub verdict: Verdict,
    /// **Which situation produced an *unreadable* verdict**, where the readings
    /// knew one. `Keys` U6's closed set is unchanged and this rides beside it.
    pub why_it_would_not_read: Option<WhyItWouldNotRead>,
    /// **Whether a `.pub` sits beside the key.** Where it does not, the copy
    /// control is **absent rather than dimmed** and one line says why —
    /// `Design/Keys`, and `Keys` W7.
    pub the_public_half_is_beside_it: bool,
}

/// **The answer to the act the user just took.** On the event and nowhere
/// durable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheAnswer {
    /// **`Keys` T2 and `Keys` T2a**: the offered set, read off the installed binary at
    /// the moment the dialog needed it. Mockup states `keys-adv` and
    /// `keys-advfall`.
    TheTypesTheDialogMayOffer(OfferedSet),
    /// **`Keys` G4 agreed and the row is written.** Mockup state `keys-made`.
    AKeyWasMade {
        at: PathBuf,
        public_half: String,
        /// **`Security and licence` S34's sentence goes with this answer**,
        /// this act having been the first to find there is nowhere to keep it.
        the_sentence_is_owed: bool,
    },
    /// **`Keys` G4: reported failed, plainly.** The reason a Windows user meets
    /// here is `crate::keys::askpass`'s own, which is about their key and their
    /// computer rather than about a part of Smoozy.
    TheKeyCouldNotBeMade { reported: Option<String> },
    /// **The press was refused before anything was touched, and the dialog says
    /// so** — step 10a round 1, fix-here item 32's *New key* half, repaired
    /// 2026-09-17. ~~Both refusals were a bare `return`~~, which said nothing
    /// and, after item 20 of the same round, left *Create and copy public key* dead for
    /// the rest of the run.
    TheMakeWasRefused(WhyTheMakeIsRefused),
    /// **`Keys` U6, F4, F5 and F6's refusals.** Mockup states `keys-noread`,
    /// `keys-bad`, `keys-type`, `keys-perm`, `keys-wrong`, `keys-here`.
    ///
    /// **The file the user pointed at travels with the refusal**, because the
    /// drawing names it in every one of the six boxes and because a refusal that
    /// did not say which file it was about would leave the user guessing between
    /// two they had just been choosing from.
    APointWasRefused {
        refused: Refused,
        picked: PathBuf,
        /// **`Keys` F4's message names the existing entry** and the drawing
        /// names it by its name as well as by its path — the name being what the
        /// user reads and the path being where it sits.
        the_other_name: Option<String>,
        /// **`Design/Keys`: a good key with the wrong fingerprint names both**,
        /// because on this one screen the fingerprint is the only thing that can
        /// tell two keys apart.
        the_rows_fingerprint: Option<String>,
        the_picked_fingerprint: Option<String>,
    },
    /// **`Keys` N1's statement.** Nothing is blocked and both keys are kept.
    /// Mockup state `keys-same`.
    AnotherKeyIsAlreadyCalledThat {
        name: String,
        the_other_fingerprint: Option<String>,
    },
    /// **`Keys` W11 and `Keys` W12, read at the act's own moment.** Mockup state
    /// `keys-rm`, and the delete box beside it.
    WhatTakingThisKeyAwayCosts {
        row: String,
        managed: bool,
        /// **`Design/Keys § Taking a key away`: Delete asks; Remove does not.**
        /// A remove destroys nothing, so it has no confirmation of its own
        /// (`Keys` W10) — **except where a server relies on the key**, where it
        /// still shows the warning, and where the saved servers could not be
        /// read, where `Keys` W12 forbids the silence reading as good news.
        it_asks: bool,
        used_by: Vec<AServerUsingIt>,
        the_saved_servers_could_not_be_read: bool,
        /// **`Keys` W6's file set at this moment, by full path.**
        the_files: Vec<PathBuf>,
    },
    /// **`Keys` W7 and `Keys` W7a.**
    ThePublicHalfIsCopied {
        contents: String,
        the_comparison_ran: bool,
    },
    TheCopyIsUnavailable(WhyNot),
    /// **What a delete or a remove came to** — `Keys` W9, said rather than
    /// thrown away (step 10a round 1, fix-here item 18). Nothing to say is a
    /// box that closes.
    TakingAwayEnded {
        row: String,
        the_key_was_not_deleted: bool,
        /// Each companion a failed unit left behind, by full path.
        left_behind: Vec<PathBuf>,
        the_passphrase_was_not_cleared: bool,
    },
    /// **`Keys` S3 item 4's erase did not confirm**, said on the row.
    ThePassphraseWasNotCleared { row: String },
    /// The act ran and the list has moved. Nothing is said; the screen is
    /// posted again behind it.
    Nothing,
}

/// **Which of the *New key* dialog's own bounds the press passed**, each with
/// one sentence. **The match on it is exhaustive on purpose**: a bound added
/// with no sentence is a silence, and the compiler is what says so.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheMakeIsRefused {
    TheNameIsTooLong,
    ThePassphraseIsTooLong,
    /// **`Keys` T1's set is fixed at three**, and this is a press that named
    /// none of them — a press made before the type probe answered, which is
    /// what the page used to swallow.
    NoTypeWasChosen,
}

/// **One refusal, one sentence.**
fn the_sentence_for_a_refused_make(why: WhyTheMakeIsRefused) -> &'static str {
    match why {
        WhyTheMakeIsRefused::TheNameIsTooLong => THIS_NAME_IS_TOO_LONG,
        WhyTheMakeIsRefused::ThePassphraseIsTooLong => THIS_PASSPHRASE_IS_TOO_LONG,
        WhyTheMakeIsRefused::NoTypeWasChosen => CHOOSE_A_KEY_TYPE,
    }
}

// ───────────────────────────────────────────────────────────────────────────
// Composing the list
// ───────────────────────────────────────────────────────────────────────────

/// **The list, composed from the local record and from nothing else.**
///
/// **`Keys` U10a: no draw waits on a reading.** Every clause of `Keys` U4 is a
/// filesystem call or a tool invocation, and `Desktop app and delivery` P4 and
/// P5 keep every one of them off the thread that draws. So this function takes
/// no reading at all: it reads the two tables, matches them once, and hands
/// back what the record holds.
///
/// **`Keys` W11a: one read per draw, not one per row.** The server rows are
/// matched against every key row in that one pass, on **the row's recorded
/// fingerprint** and never on one read from the file.
pub fn the_screen_from(
    rows: &[KeyRow],
    servers: Option<&[(SavedServer, Option<&'static str>)]>,
    the_tools_are_missing: bool,
    mode: TheMode,
) -> TheKeysScreen {
    // **`Keys` U1 set 1**, in `Design/Keys § The list's order`'s order: by name.
    // **A row with no name sorts by its path**, which is the only other thing it
    // has, and two rows sharing a name end up side by side, which is the whole
    // reason the order is stated.
    let mut ordered: Vec<&KeyRow> = listing::from_the_record(rows).iter().collect();
    ordered.sort_by(|left, right| {
        the_sort_key(left)
            .to_lowercase()
            .cmp(&the_sort_key(right).to_lowercase())
            .then_with(|| the_sort_key(left).cmp(&the_sort_key(right)))
    });

    let plain: Option<Vec<SavedServer>> =
        servers.map(|held| held.iter().map(|(server, _)| server.clone()).collect());

    let rows = ordered
        .into_iter()
        .map(|row| {
            let used = ownership::used_by(row, plain.as_deref());
            ARow {
                identifier: row.identifier.as_text(),
                name: row.name.as_ref().map(|name| name.as_str().to_string()),
                fingerprint: row.fingerprint.as_ref().map(|held| held.value.as_str().to_string()),
                path: row.path.display().to_string(),
                used_by: the_servers_using_it(&used, servers),
                the_saved_servers_could_not_be_read: used == UsedBy::CouldNotCheck,
                a_passphrase_is_remembered: usable::a_passphrase_is_available_without_asking(row),
                // **`Keys` W2's derivation at the draw**, and the act re-derives
                // at its own moment. Where the home directory will not read
                // there is no usual key location to compare against, and a key
                // is **borrowed** — the safe direction of the two, since
                // *Remove* destroys nothing.
                managed: the_kind_of(row) == Ownership::Managed,
            }
        })
        .collect();

    TheKeysScreen {
        mode,
        the_tools_are_missing,
        rows,
    }
}

/// What a row sorts by. **The name where it has one, the path where it has
/// none.**
fn the_sort_key(row: &KeyRow) -> String {
    match &row.name {
        Some(name) => name.as_str().to_string(),
        None => row.path.display().to_string(),
    }
}

/// **`Keys` W2, re-derived wherever it is needed and stored nowhere.**
fn the_kind_of(row: &KeyRow) -> Ownership {
    match locations::resolve(Location::UserHome) {
        Ok(home) => smoozy_keys::ownership::ownership(&row.path, &home, &acts::Canonical),
        // **The safe direction.** Row 251's unrecoverable branch is deleting a
        // file Smoozy promised only to borrow, so a location that cannot be
        // resolved leaves every key borrowed and every control *Remove*.
        Err(_) => Ownership::Borrowed,
    }
}

/// The *Used by* line's own pairs — the label `Keys` W11 matched on, and the
/// badge the drawing puts beside it.
///
/// **The colour is not `Keys`'.** That module's `SavedServer` holds the label
/// and the fingerprint, which is all `Keys` W11's match needs; the badge is
/// `Design/Whole app § The server badge`'s and comes from the same one read of
/// the server rows.
fn the_servers_using_it(
    used: &UsedBy,
    servers: Option<&[(SavedServer, Option<&'static str>)]>,
) -> Vec<AServerUsingIt> {
    let UsedBy::TheseServers(labels) = used else {
        return Vec::new();
    };
    let mut taken: Vec<bool> = servers.map(|held| vec![false; held.len()]).unwrap_or_default();
    labels
        .iter()
        .map(|label| {
            let badge = servers.and_then(|held| {
                let at = held
                    .iter()
                    .enumerate()
                    .position(|(at, (server, _))| server.label == *label && !taken[at])?;
                taken[at] = true;
                held[at].1
            });
            AServerUsingIt {
                called: label.clone(),
                badge,
            }
        })
        .collect()
}

/// **The eight the drawing has, and no ninth.** `Design/Whole app § The server
/// badge`. **A colour outside the eight draws no badge**, which is the same
/// refusal `crate::surface::the_rail` makes: a badge means *which server* and
/// nothing else, so one invented at drawing time could differ between runs.
///
/// *(It is the third copy of this list in the crate — the rail holds one and
/// piece 3's server screen will hold another. Recorded in this piece's account
/// as a fact in three places, which is the shape this stage keeps meeting.)*
const THE_EIGHT_BADGES: [&str; 8] = ["b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8"];

fn the_badge(colour: Option<&str>) -> Option<&'static str> {
    let colour = colour?;
    THE_EIGHT_BADGES
        .iter()
        .find(|badge| **badge == colour)
        .copied()
}

// ───────────────────────────────────────────────────────────────────────────
// The commands
// ───────────────────────────────────────────────────────────────────────────

/// **The page asks for the screen.**
///
/// P6: a command is the page asking and an event is the application telling, so
/// this answers with nothing and everything arrives on the queue.
///
/// **The readings run on a thread of their own and the list does not wait on
/// them.** `Desktop app and delivery` P4 and P5 put every filesystem call and
/// every tool invocation off the thread that draws, and a Tauri command that is
/// not `async` runs on that thread — so the list is composed and posted here,
/// and [`the_readings_are_taken`] is spawned behind it.
///
/// **Whether this is a pick travels on its own and never inside the name**
/// (step 10a round 1, fix-here item 2(b)): the Add dialog can open this screen
/// with nothing typed, and a pick that named nothing used to read as the
/// ordinary list, so the picking controls never appeared.
#[tauri::command]
pub fn the_keys_screen<R: Runtime>(
    app: AppHandle<R>,
    picking: bool,
    picking_for: Option<String>,
) {
    the_screen_is_posted(&app, the_mode_asked_for(picking, picking_for));
}

/// **The reading the page asked for** — `Keys` U11: a pick names its server
/// where anything does, and **a pick that names nothing is still a pick**.
fn the_mode_asked_for(picking: bool, picking_for: Option<String>) -> TheMode {
    if !picking {
        return TheMode::Housekeeping;
    }
    TheMode::Picking {
        naming: the_context_names(&DialogContext {
            label: picking_for,
            username: None,
            host: None,
        }),
    }
}

/// **The list, composed and posted, and then the readings behind it.**
pub fn the_screen_is_posted<R: Runtime>(app: &AppHandle<R>, mode: TheMode) {
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    let queue = queue.inner().clone();
    // **The tools probe is a tool run, so it is taken before the record's lock
    // and never under it** (step 10a round 1, fix-here item 20).
    //
    // **Where Smoozy's own temporary area cannot be named, nothing is read**:
    // the record sits under the same location and has not opened either, and
    // that failure is the start's to draw — see [`the_temporary_area`].
    let Some(area) = the_temporary_area() else {
        return;
    };
    let the_tools_are_missing = !OnThisComputer::over(area).there_is_a_generator();
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return;
    };
    let Ok(started) = started.lock() else {
        return;
    };
    let Some(record) = started.record.as_ref() else {
        // **L8a: a screen shows the failure rather than an emptied version of
        // itself**, and the failure to open the local record at all is the
        // start's, which already carries it. Nothing is drawn here from a read
        // that did not happen — `Keys` W11a's own rule, from the other end.
        return;
    };
    let connection = record.connection();

    let rows = key_rows::all(connection).unwrap_or_default();
    // **One read of the server rows for the whole list** (`Keys` W11a). A read
    // that will not answer is `Keys` W12's third state and is carried as itself.
    let servers = the_saved_servers(connection);

    let says = the_screen_from(&rows, servers.as_deref(), the_tools_are_missing, mode);
    let _ = queue.read_then_post::<TheKeysScreen, std::convert::Infallible>(
        || Ok(says.clone()),
        the_screen_says,
    );

    drop(started);
    let taking = app.clone();
    let asked: Vec<KeyRow> = rows;
    std::thread::spawn(move || the_readings_are_taken(&taking, asked));
}

/// **`Keys` U10a's readings, off the drawing thread, one key at a time.**
///
/// **The discovery pass runs first and its rows arrive behind set 1's** — `Keys` U1 set
/// 2's membership is itself a reading (a directory enumeration plus `Keys` U2's header
/// read on every regular file), so the list cannot be drawn with it already in.
///
/// **Each key's answer reaches the interface as its own event**, so a slow key
/// holds up no other row.
fn the_readings_are_taken<R: Runtime>(app: &AppHandle<R>, rows: Vec<KeyRow>) {
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    let queue = queue.inner().clone();
    let Some(temporary_area) = the_temporary_area() else {
        return;
    };
    let readings = OnThisComputer::over(temporary_area.clone());

    // **`Keys` U1 set 2: every regular file in the usual key location that passes the
    // container test and has no row.** A discovered file takes a row at first
    // sight, and `Keys` U1a refuses a second row for a fingerprint a current row
    // already holds.
    let discovered = the_discovery_pass(app, &readings);
    if !discovered.is_empty() {
        // The list has grown, so it is posted again with the new rows in it
        // rather than the page being asked to insert them.
        the_screen_is_posted_again(app);
    }

    let mut all = rows;
    all.extend(discovered);

    for row in &all {
        let checked = usable::check(row, &readings);
        the_row_keeps_what_the_check_read(app, row, &checked);
        let says = AVerdict {
            row: row.identifier.as_text(),
            verdict: checked.verdict.clone(),
            why_it_would_not_read: checked.why_it_would_not_read,
            the_public_half_is_beside_it: supply::the_public_half_of(row).exists(),
        };
        let _ = queue.read_then_post::<AVerdict, std::convert::Infallible>(
            || Ok(says.clone()),
            the_verdict_says,
        );
    }
}

/// **`Keys` U1 set 2 and `Keys` U1a, performed.**
fn the_discovery_pass<R: Runtime>(app: &AppHandle<R>, readings: &OnThisComputer) -> Vec<KeyRow> {
    let Ok(home) = locations::resolve(Location::UserHome) else {
        return Vec::new();
    };
    let location = smoozy_keys::locations::usual_key_location(&home);
    let Ok(found) = std::fs::read_dir(&location) else {
        return Vec::new();
    };

    let mut candidates = Vec::new();
    for entry in found.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        // **`Keys` U2's container test is the discriminator, and the fingerprint read
        // is never one** — `ssh-keygen -l -f` fingerprints `known_hosts` at
        // exit 0.
        let passes = readings
            .opening_bytes(&path)
            .and_then(|bytes| smoozy_keys::container::read(&bytes))
            .is_some();
        let read = passes
            .then(|| usable::read_the_fingerprint(&path, readings))
            .and_then(|read| match read {
                usable::FingerprintRead::Answered(said) => Some(said),
                _ => None,
            });
        candidates.push(listing::Candidate {
            path,
            passes_the_container_test: passes,
            fingerprint: read.as_ref().map(|said| said.reading.clone()),
            name: read.and_then(|said| said.name),
        });
    }

    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return Vec::new();
    };
    let Ok(started) = started.lock() else {
        return Vec::new();
    };
    let Some(record) = started.record.as_ref() else {
        return Vec::new();
    };
    the_discoveries_are_written(record.connection(), &candidates)
}

/// **The discovery pass's writes, against the rows as they stand under the
/// record's lock** — `Keys` U1a.
///
/// **The rows are read again here and never taken from the draw.** Every draw
/// of the screen starts a pass of its own, and two draws close together — the
/// screen and its own repost — each found no row in the rows read before the
/// scan and each wrote one, so the list showed one key twice (the live pass on
/// Vast, 2026-10-19). Read under the lock, the second pass sees the first
/// pass's row and writes nothing.
fn the_discoveries_are_written(
    connection: &rusqlite::Connection,
    candidates: &[listing::Candidate],
) -> Vec<KeyRow> {
    // **A read that does not answer writes nothing**: with no rows to compare
    // against, every key in the location would look new.
    let Ok(rows) = key_rows::all(connection) else {
        return Vec::new();
    };
    let discovered = listing::discoveries(candidates, &rows);
    let mut written = Vec::new();
    for one in discovered {
        let Ok(identifier) = Identity::issue(IdentityKind::LocalKey) else {
            continue;
        };
        let default_now = the_default_name_now();
        let was_default = naming::was_default_at_entry(one.name.as_ref(), &default_now);
        let Some(row) = KeyRow::entering(
            identifier,
            one.path.clone(),
            one.fingerprint.clone(),
            one.name.clone(),
            was_default,
        ) else {
            continue;
        };
        if key_rows::add(connection, &row).is_ok() {
            written.push(row);
        }
    }
    written
}

/// **`Keys` R1 and `Keys` U7a's first rule, at the check that took the readings.**
///
/// The name is refreshed where one was read; the fingerprint is kept under
/// `Keys` U7a's bound — **a value read from the public half is kept only where the
/// private file's own read has never succeeded for this row.**
/// `crate::keys::supply` states the whole of the rule and this is the same act
/// from the screen's side.
fn the_row_keeps_what_the_check_read<R: Runtime>(app: &AppHandle<R>, row: &KeyRow, checked: &usable::Checked) {
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return;
    };
    let Ok(started) = started.lock() else {
        return;
    };
    let Some(record) = started.record.as_ref() else {
        return;
    };
    let connection = record.connection();
    let mut kept = row.clone();
    if let Some(fresh) = checked.fingerprint_now.clone() {
        kept.fingerprint = Some(smoozy_keys::fingerprint::Reading::what_the_row_keeps(
            row.fingerprint.as_ref(),
            fresh,
        ));
    }
    if checked.name_now.is_some() {
        kept.name = checked.name_now.clone();
    }
    if kept.fingerprint != row.fingerprint || kept.name != row.name {
        let _ = key_rows::write(connection, &kept);
    }
}

/// **`Keys` T2 and `Keys` T2a: the offered set, read off the installed binary at the
/// moment the dialog needs it, once per application run.**
///
/// **The dialog draws first and the type control fills in behind it** — three
/// generations, each its own process, are exactly what P5 keeps off the drawing
/// thread — so this spawns and answers on the queue.
#[tauri::command]
pub fn the_types_the_dialog_may_offer<R: Runtime>(app: AppHandle<R>) {
    let taking = app.clone();
    std::thread::spawn(move || {
        // **The probe writes throwaway keys into the area**, so where the area
        // cannot be named nothing is probed — see [`the_temporary_area`].
        let Some(area) = the_temporary_area() else {
            return;
        };
        let offered = making::probe_the_offered_set(&area);
        the_answer_is_posted(&taking, TheAnswer::TheTypesTheDialogMayOffer(offered));
    });
}

/// **`Keys` G1: generation is one act carrying four things**, asked for.
///
/// **H34's bounds first**: the name and the passphrase are text fields and this
/// refuses a value no dialog could have collected. **Neither is a check on
/// shape** — `Keys` G1 passes the comment to `-C` verbatim and holds no opinion
/// about a passphrase.
///
/// **Every one of the three refusals is said where the user pressed** (item 32,
/// repaired 2026-09-17). ~~All three were a bare `return`~~: the press did
/// nothing, said nothing, and left the control that raised it marked underway
/// and dead.
#[tauri::command]
pub fn a_key_is_made<R: Runtime>(app: AppHandle<R>, name: String, passphrase: String, key_type: String) {
    if name.len() > THE_LONGEST_A_NAME_MAY_BE {
        the_make_is_refused(&app, WhyTheMakeIsRefused::TheNameIsTooLong);
        return;
    }
    if passphrase.len() > THE_LONGEST_A_PASSPHRASE_MAY_BE {
        the_make_is_refused(&app, WhyTheMakeIsRefused::ThePassphraseIsTooLong);
        return;
    }
    let Some(key_type) = KeyType::from_the_tools_name(&key_type) else {
        // **`Keys` T1's set is fixed at three and this is H34's enum check.** A
        // type the page did not read off the probe — and a press made before the
        // probe answered, which the page now sends rather than swallowing — is
        // refused before anything is touched.
        the_make_is_refused(&app, WhyTheMakeIsRefused::NoTypeWasChosen);
        return;
    };
    let taking = app.clone();
    std::thread::spawn(move || the_key_is_made(&taking, name, passphrase, key_type));
}

/// **The refusal goes back the way every other answer does**, so the dialog
/// draws it in the place it draws a failed generation and *Create and copy public key*
/// comes back with it.
fn the_make_is_refused<R: Runtime>(app: &AppHandle<R>, why: WhyTheMakeIsRefused) {
    the_answer_is_posted(app, TheAnswer::TheMakeWasRefused(why));
}

fn the_key_is_made<R: Runtime>(app: &AppHandle<R>, name: String, passphrase: String, key_type: KeyType) {
    let Ok(home) = locations::resolve(Location::UserHome) else {
        the_answer_is_posted(app, TheAnswer::TheKeyCouldNotBeMade { reported: None });
        return;
    };
    // **The act's own refusal where Smoozy's own temporary area cannot be
    // named** — the passphrase channel is opened there and nowhere else.
    let Some(Ok(route)) = the_temporary_area().map(TheRoute::of_this_application) else {
        the_answer_is_posted(app, TheAnswer::TheKeyCouldNotBeMade { reported: None });
        return;
    };
    // **`Security and licence` §1.4: the field the user entered it into is that
    // value's origin**, and from the first destination onward the three hops
    // govern. The origin is made here and dropped when the act ends.
    let typed = (!passphrase.is_empty()).then(|| Origin::what_the_user_typed(passphrase));

    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return;
    };
    // **The record is lent for each read and each write, and the lock is never
    // held across `ssh-keygen`** — step 10a, the screens round's loose ends, L4
    // (item 20 of that round's Verdict). ~~The spine was locked here and the
    // guard held until the act came back~~, so every other screen waited on the
    // tool, and on the askpass box the user had to answer before it.
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let store = the_store_for_this_system();

    let ask = making::TheAsk {
        key_location: smoozy_keys::locations::usual_key_location(&home),
        key_type,
        comment: name,
        passphrase: typed.as_ref(),
        the_default_name_now: the_default_name_now(),
        the_route: route,
    };
    // **`Security and licence` S26's first use**, where the user asked for
    // a passphrase: the act's own store put is what takes the probe, and this
    // run's is [`crate::the_store_standing::THIS_RUN`].
    let made = making::generate(
        &ask,
        &record,
        the_store(&store),
        THE_STORE_DEADLINE,
        &crate::the_store_standing::THIS_RUN,
    );

    let answer = match made {
        Ok(made) => match made.pair {
            ThePair::Made { at } => {
                let the_sentence_is_owed = made.the_sentence_is_owed;
                // **`Design/Keys § Making a key`: the dialog ends on the copy**,
                // because pasting the public half at a provider is the reason
                // the key was made at all.
                let public = smoozy_keys::locations::companions(&at).public_half;
                let contents = std::fs::read_to_string(&public).unwrap_or_default();
                TheAnswer::AKeyWasMade {
                    at,
                    public_half: contents,
                    the_sentence_is_owed,
                }
            }
            // **Only Smoozy's own sentence reaches the dialog** — the Windows
            // limit, or the path that is too long (L1, 2026-09-18). What the
            // system reported is a diagnostic, and it used to be put on the
            // screen as the reason.
            ThePair::ThePassphraseCouldNotBeDelivered { why } => {
                TheAnswer::TheKeyCouldNotBeMade {
                    reported: why.said.map(str::to_string),
                }
            }
            ThePair::Failed(_) | ThePair::TheToolProducedNothing => {
                TheAnswer::TheKeyCouldNotBeMade { reported: None }
            }
        },
        Err(_) => TheAnswer::TheKeyCouldNotBeMade { reported: None },
    };
    the_answer_is_posted(app, answer);
    the_screen_is_posted_again(app);
}

/// **Which control the user pressed**, named by the caller rather than inferred
/// — `Keys` F6 then settles which act the picked file may actually take.
// **The three names begin with the same letter because they are the three the
// user presses**, and the plan names them that way — *a first point*, *its new
// location*, *a replacement*.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WhichOffer {
    AFirstPoint,
    ANewLocation,
    AReplacement,
}

/// **The point, in its three forms** — `Design/Keys`'s *Choose a key…* on the
/// head, and the two ways out on an unusable row.
///
/// **The picker is the one door** (`Desktop app and delivery` K11), and whatever
/// it returns is **a path from the user, judged like any other name** (K13).
#[tauri::command]
pub fn a_key_is_pointed_at<R: Runtime>(app: AppHandle<R>, offer: String, row: Option<String>) {
    let which = match offer.as_str() {
        "a-first-point" => WhichOffer::AFirstPoint,
        "its-new-location" => WhichOffer::ANewLocation,
        "a-replacement" => WhichOffer::AReplacement,
        // H34's enum check.
        _ => return,
    };
    let taking = app.clone();
    std::thread::spawn(move || the_point_is_taken(&taking, which, row));
}

fn the_point_is_taken<R: Runtime>(app: &AppHandle<R>, which: WhichOffer, row: Option<String>) {
    let mending = row
        .as_deref()
        .and_then(|text| Identity::read_back(IdentityKind::LocalKey, text));
    let offered = match (which, mending) {
        (WhichOffer::AFirstPoint, _) => Offered::AFirstPoint,
        (WhichOffer::ANewLocation, Some(row)) => Offered::ANewLocationFor(row),
        (WhichOffer::AReplacement, Some(row)) => Offered::AReplacementFor(row),
        // A mending offer with no row is a caller naming a row that is not
        // there, which `Keys` F9's own act refuses.
        _ => return,
    };

    let picked = crate::picker::ask(
        app,
        &crate::picker::Asking {
            what: crate::picker::WhatMayBeChosen::Files,
            how_many: crate::picker::HowMany::One,
            starting_at: locations::resolve(Location::UserHome)
                .map(|home| smoozy_keys::locations::usual_key_location(&home))
                .ok(),
            // **X5: the system file picker's own chrome passes the point too.**
            title: Prose::new(CHOOSE_A_KEY),
        },
    );
    let crate::picker::Picked::TheUserChose(chosen) = picked else {
        // A user who closed the panel chose nothing, and that is an ordinary
        // answer rather than a failure.
        return;
    };
    let Some(path) = chosen.first().map(|one| one.as_the_user_gave_it().to_path_buf()) else {
        return;
    };

    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return;
    };
    // **The record is lent for each read and each write** — step 10a, the
    // screens round's loose ends, L4. ~~The spine was locked here and held
    // until the act came back~~, and reading the picked file is an
    // `ssh-keygen` run.
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let store = the_store_for_this_system();
    let default_now = the_default_name_now();
    let Some(temporary_area) = the_temporary_area() else {
        return;
    };

    let before = {
        let mut rows = Vec::new();
        crate::keys::LendsTheRecord::lend(&record, &mut |connection| {
            rows = key_rows::all(connection).unwrap_or_default();
        });
        rows
    };

    let answer = match offered {
        Offered::AFirstPoint => {
            match acts::point_at_a_key(&path, &default_now, &record, &temporary_area) {
                Ok(row) => the_duplicate_name_statement(&row, &before),
                Err(refused) => the_refusal(refused, &path, &before, None, &temporary_area),
            }
        }
        mending => {
            match acts::point_a_row_at(
                &path,
                &mending,
                &default_now,
                &record,
                the_store(&store),
                THE_STORE_DEADLINE,
                &temporary_area,
            ) {
                Ok(Mended::ItsNewLocation) | Ok(Mended::AReplacement { .. }) => TheAnswer::Nothing,
                Err(refused) => {
                    let mending = mending
                        .the_row_being_mended()
                        .and_then(|named| before.iter().find(|row| row.identifier == *named));
                    the_refusal(refused, &path, &before, mending, &temporary_area)
                }
            }
        }
    };

    if answer != TheAnswer::Nothing {
        the_answer_is_posted(app, answer);
    }
    the_screen_is_posted_again(app);
}

/// **A refusal, with the two things the drawing's boxes name beside it.**
///
/// **Nothing new is read for the fingerprints.** The row's is the one it already
/// holds and the picked file's is the reading `Keys` U10's check took a moment
/// ago at the same act — this asks for it again on the one file the refusal is
/// about, which is the same one reading rather than a second kind of one.
fn the_refusal(
    refused: Refused,
    picked: &Path,
    rows: &[KeyRow],
    mending: Option<&KeyRow>,
    temporary_area: &Path,
) -> TheAnswer {
    // **`Keys` F4's message names the existing entry**, which is the row whose
    // path the refusal carries.
    let named = match &refused {
        Refused::ThisKeyIsAlreadyListed { as_row }
        | Refused::TwoKeysSmoozyCannotTellApart { the_other_row: as_row } => rows
            .iter()
            .find(|row| row.path == *as_row)
            .and_then(|row| row.name.as_ref().map(|name| name.as_str().to_string())),
        _ => None,
    };
    // **The two fingerprints, and only where the refusal is the one that names
    // them** — `Design/Keys`: *"It names both fingerprints, because on this one
    // screen the fingerprint is the only thing that can tell two keys apart."*
    let (held, taken) = match &refused {
        Refused::NotThisKey => (
            mending
                .and_then(|row| row.fingerprint.as_ref())
                .map(|read| read.value.as_str().to_string()),
            acts::read_a_picked_file(picked, temporary_area)
                .and_then(|(_, picked, _)| picked.fingerprint)
                .map(|read| read.value.as_str().to_string()),
        ),
        _ => (None, None),
    };
    TheAnswer::APointWasRefused {
        refused,
        picked: picked.to_path_buf(),
        the_other_name: named,
        the_rows_fingerprint: held,
        the_picked_fingerprint: taken,
    }
}

/// **`Keys` N1's statement, where the new row shares a name with a listed key
/// under a different fingerprint.**
///
/// **It is a statement, not a refusal** — nothing was blocked and both keys are
/// kept, which is why it is composed after the row was written rather than
/// instead of writing it.
fn the_duplicate_name_statement(entering: &KeyRow, before: &[KeyRow]) -> TheAnswer {
    let Some(name) = entering.name.as_ref() else {
        return TheAnswer::Nothing;
    };
    let fires = before.iter().find(|listed| {
        naming::the_duplicate_name_warning_fires(
            Some(name),
            entering.was_default_at_entry,
            entering.fingerprint.as_ref().map(|held| &held.value),
            listed,
        )
    });
    match fires {
        Some(listed) => TheAnswer::AnotherKeyIsAlreadyCalledThat {
            name: name.as_str().to_string(),
            the_other_fingerprint: listed
                .fingerprint
                .as_ref()
                .map(|held| held.value.as_str().to_string()),
        },
        None => TheAnswer::Nothing,
    }
}

/// **`Keys` W7: copy reads `path.pub` and only that**, with `Keys` W7a's comparison run
/// before the contents reach the clipboard.
///
/// **The clipboard itself is the page's.** This application binds no clipboard
/// service — none is in the manifest and adding one is a dependency change — so
/// the contents travel as a value on the answer event and the page writes them.
/// **They are a public half**, which is what the control exists to hand out.
#[tauri::command]
pub fn a_public_half_is_copied<R: Runtime>(app: AppHandle<R>, row: String) {
    let taking = app.clone();
    std::thread::spawn(move || {
        let Some(row) = the_row_named(&taking, &row) else {
            return;
        };
        let Some(area) = the_temporary_area() else {
            return;
        };
        let answer = match acts::copy_the_public_half(&row, &area) {
            Copy::ToTheClipboard(contents) => TheAnswer::ThePublicHalfIsCopied {
                contents,
                the_comparison_ran: true,
            },
            Copy::ToTheClipboardWithoutTheComparison(contents) => {
                TheAnswer::ThePublicHalfIsCopied {
                    contents,
                    the_comparison_ran: false,
                }
            }
            Copy::Unavailable(why) => TheAnswer::TheCopyIsUnavailable(why),
        };
        the_answer_is_posted(&taking, answer);
    });
}

/// **`Keys` W11 and `Keys` W12: the warning, read at the act's own moment.**
///
/// **`Keys` W12 is untouched by `Keys` W11a**: the standing list's line comes out of the same
/// read the rows do, and this read is taken again, on its own, at the moment the
/// user asks to take a key away — so it can fail here and say so, and it never
/// reads as good news.
#[tauri::command]
pub fn what_taking_a_key_away_costs<R: Runtime>(app: AppHandle<R>, row: String) {
    let taking = app.clone();
    std::thread::spawn(move || {
        let Some(key) = the_row_named(&taking, &row) else {
            return;
        };
        let (used, servers) = {
            let Some(started) = taking.try_state::<Mutex<Started>>() else {
                return;
            };
            let Ok(started) = started.lock() else {
                return;
            };
            let Some(record) = started.record.as_ref() else {
                return;
            };
            let servers = the_saved_servers(record.connection());
            let plain: Option<Vec<SavedServer>> = servers
                .as_ref()
                .map(|held| held.iter().map(|(server, _)| server.clone()).collect());
            (ownership::used_by(&key, plain.as_deref()), servers)
        };
        let managed = the_kind_of(&key) == Ownership::Managed;
        // **`Keys` W6: the file set at the moment of the act, by full path.**
        // Nothing wider and nothing narrower.
        let the_files = if managed {
            acts::the_file_set_now(&key.path)
                .named_by_the_confirmation()
                .into_iter()
                .map(|path| path.to_path_buf())
                .collect()
        } else {
            Vec::new()
        };
        the_answer_is_posted(
            &taking,
            TheAnswer::WhatTakingThisKeyAwayCosts {
                row: key.identifier.as_text(),
                managed,
                it_asks: managed
                    || !matches!(used, UsedBy::NoSavedServer),
                used_by: the_servers_using_it(&used, servers.as_deref()),
                the_saved_servers_could_not_be_read: used == UsedBy::CouldNotCheck,
                the_files,
            },
        );
    });
}

/// **The act — `Keys` W8's delete, or `Keys` W10's remove from Smoozy.**
///
/// **W2 re-derives ownership at the act's own moment** rather than trusting the
/// control that called it, which is why the page names no kind here.
#[tauri::command]
pub fn a_key_is_taken_away<R: Runtime>(app: AppHandle<R>, row: String) {
    let taking = app.clone();
    std::thread::spawn(move || {
        let Some(key) = the_row_named(&taking, &row) else {
            return;
        };
        {
            let Some(started) = taking.try_state::<Mutex<Started>>() else {
                return;
            };
            // **The record is lent for each read and each write** (L4): the
            // delete's file work is not a reason to hold the spine.
            let record = crate::keys::TheRunsRecord {
                started: started.inner(),
            };
            let store = the_store_for_this_system();

            let held_a_passphrase = key.passphrase.is_some();
            // **`Keys` W9: the outcome names what was left behind, and it is
            // said** — step 10a round 1, fix-here item 18. It used to be
            // thrown away.
            let ended = match the_kind_of(&key) {
                Ownership::Managed => {
                    let Ok(home) = locations::resolve(Location::UserHome) else {
                        return;
                    };
                    // The file set at this moment, which is the one the act
                    // takes, so a companion left behind is named by its path.
                    let set = acts::the_file_set_now(&key.path);
                    match acts::delete_a_managed_key(
                        &key,
                        &home,
                        &record,
                        the_store(&store),
                        THE_STORE_DEADLINE,
                    ) {
                        Ok(deleted) => {
                            let gone = deleted.the_key_is_gone();
                            TheAnswer::TakingAwayEnded {
                                row: key.identifier.as_text(),
                                the_key_was_not_deleted: !gone,
                                left_behind: deleted
                                    .left_behind(&set)
                                    .into_iter()
                                    .map(Path::to_path_buf)
                                    .collect(),
                                the_passphrase_was_not_cleared: gone
                                    && held_a_passphrase
                                    && deleted.erasure != Some(smoozy_security_and_licence::erasure::Erasure::Done),
                            }
                        }
                        Err(_) => TheAnswer::TakingAwayEnded {
                            row: key.identifier.as_text(),
                            the_key_was_not_deleted: true,
                            left_behind: Vec::new(),
                            the_passphrase_was_not_cleared: false,
                        },
                    }
                }
                Ownership::Borrowed => {
                    let erasure =
                        acts::remove_from_smoozy(&key, &record, the_store(&store), THE_STORE_DEADLINE);
                    TheAnswer::TakingAwayEnded {
                        row: key.identifier.as_text(),
                        the_key_was_not_deleted: false,
                        left_behind: Vec::new(),
                        the_passphrase_was_not_cleared: held_a_passphrase
                            && erasure != smoozy_security_and_licence::erasure::Erasure::Done,
                    }
                }
            };
            the_answer_is_posted(&taking, ended);
        }
        the_screen_is_posted_again(&taking);
    });
}

/// **`Keys` S3 item 4: the user clearing a remembered passphrase.**
///
/// **The row stays**, which is the whole of the difference from a remove.
#[tauri::command]
pub fn a_remembered_passphrase_is_cleared<R: Runtime>(app: AppHandle<R>, row: String) {
    let taking = app.clone();
    std::thread::spawn(move || {
        let Some(key) = the_row_named(&taking, &row) else {
            return;
        };
        {
            let Some(started) = taking.try_state::<Mutex<Started>>() else {
                return;
            };
            // **The record is lent for each of `Security and licence` §5.1's
            // three writes** (L4).
            let record = crate::keys::TheRunsRecord {
                started: started.inner(),
            };
            let store = the_store_for_this_system();
            let mut marks = crate::keys::TheMarksLent(&record);
            let erasure = acts::clear_a_remembered_passphrase(
                &key,
                &mut marks,
                the_store(&store),
                THE_STORE_DEADLINE,
            );
            // **Said where it failed, on the row** — step 10a round 1,
            // fix-here item 18.
            if erasure != smoozy_security_and_licence::erasure::Erasure::Done {
                the_answer_is_posted(
                    &taking,
                    TheAnswer::ThePassphraseWasNotCleared {
                        row: key.identifier.as_text(),
                    },
                );
            }
        }
        the_screen_is_posted_again(&taking);
    });
}

// ───────────────────────────────────────────────────────────────────────────
// The passphrase box — `Keys` S14 to `Keys` S19
// ───────────────────────────────────────────────────────────────────────────

/// **`Keys` S14: `Keys` raises the ask, and it is an ordinary window of the
/// application rather than a tool's.**
///
/// **This is the door and nothing calls it yet.** `Servers` P4 hands the
/// question away in as many words, and the connect that would ask it is that
/// module's transport — step 4's code, with no screen driving it until the
/// server screen and the connect are joined. **The contract is in this piece's
/// account, addressed to whoever builds that connect.**
///
/// **`Keys` S16 fixes the order and this function is its first step**: the three
/// readings, then the box, then S5's supply — the check, the one local load over
/// the askpass route, and the connection. **The askpass helper is never the
/// asking surface**: it is started by the one-use broker, lives for one
/// delivery, and could carry neither the server's name nor the key's, because no
/// secret and no context is ever a process argument or an environment value.
///
/// **`Keys` S13 is kept here and nowhere else.** That clause checked the
/// opposite answer against the Method's three escalations and declined it — *the
/// one thing that would have escalated is making the automatic attempt anyway,
/// which puts a passphrase prompt on screen for a connection the user did not
/// start*. `the_user_started_this_connect` is that refusal in code, and this is
/// the only route to the box there is.
pub fn the_passphrase_is_asked_for<R: Runtime>(
    app: &AppHandle<R>,
    row: &KeyRow,
    server: ServerContext,
    the_user_started_this_connect: bool,
    an_earlier_acts_erasure_is_on_the_record: bool,
) -> Option<TheAsk> {
    let started = app.try_state::<Mutex<Started>>()?;
    // **The record is lent for the refresh write alone** (L4): the `Keys` U10
    // check inside is an `ssh-keygen` run.
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let ask = supply::the_ask_for_a_key_with_nothing_held(
        row,
        server,
        the_user_started_this_connect,
        an_earlier_acts_erasure_is_on_the_record,
        &record,
        &the_temporary_area()?,
    )?;
    the_ask_is_posted(app, row, &ask, None, false);
    Some(ask)
}

/// The box, put on the queue.
///
/// **It carries the server it was raised for**, because *"the box is the
/// connection's"* (`Preparation/Design/Keys.md`) and its *Connect* opens that
/// server's connect — [`the_passphrase_box_was_answered`]. **And `Security
/// and licence` S34's sentence where this connect was the first to find there
/// is nowhere to keep a passphrase** — mockup state `keys-nostore`.
pub fn the_ask_is_posted<R: Runtime>(
    app: &AppHandle<R>,
    row: &KeyRow,
    ask: &TheAsk,
    server: Option<&str>,
    the_sentence_is_owed: bool,
) {
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    let identifier = row.identifier.as_text();
    let ask = ask.clone();
    let server = server.map(str::to_string);
    let _ = queue.inner().read_then_post::<(String, TheAsk), std::convert::Infallible>(
        || Ok((identifier.clone(), ask.clone())),
        |(row, ask)| {
            let mut event = the_ask_says(row, ask);
            if let Some(server) = &server {
                event = event.saying("server", Said::Value(Verbatim::new(server.clone())));
            }
            if the_sentence_is_owed {
                event = event.saying(
                    "nowhereToKeepThem",
                    Said::Prose(Prose::new(NOWHERE_TO_KEEP_THEM)),
                );
            }
            event
        },
    );
}

/// **`Keys` S17: what the box takes, and it goes to two places and no third.**
///
/// **A value opens the attempt and nothing ends it** — `Security and licence`
/// §3.0 rule 6's third bullet: *"A value the user is entering has not come out
/// of the store and this rule does not reach it… from the put onward the value
/// is the store's and the rule binds."* Its two destinations are the one-use
/// broker for this attempt's one delivery, and the adapter's put.
///
/// **Pressing *Connect* opens the connect the box was raised for**, with what
/// was typed carried to that one attempt — `Preparation/Design/Keys.md`: *"So
/// the box is the connection's, not this screen's."* The value is borrowed for
/// the connect and dropped with it, and **never read back out of the store**,
/// so on a computer with nowhere to remember a protected key still connects
/// (`Keys` S9). *(Step 10b's first audit round, item 3: this used to put what
/// was typed into the store and connect nothing, so the user had to press
/// *Connect* on the row again — and on such a computer, every press raised the
/// box again.)*
///
/// **The load succeeding is what puts it** and the load failing puts nothing,
/// which is the spec's rule that a passphrase opening nothing is never kept —
/// that is the connect's own supply, `crate::keys::for_a_connection`.
#[tauri::command]
pub fn the_passphrase_box_was_answered<R: Runtime>(
    app: AppHandle<R>,
    row: String,
    passphrase: String,
    server: String,
) {
    let _ = row;
    if passphrase.is_empty() || passphrase.len() > THE_LONGEST_A_PASSPHRASE_MAY_BE {
        return;
    }
    let answer = the_box_opens_its_connect(&app, &server, Origin::what_the_user_typed(passphrase));
    if let Some(answer) = answer {
        crate::surface::servers::the_answer_is_said(&app, answer);
    }
}

/// **The box's *Connect*, below the command** — the connect it was raised for,
/// carrying what was typed. What the act answers is the connect's own answer.
pub(crate) fn the_box_opens_its_connect<R: Runtime>(
    app: &AppHandle<R>,
    server: &str,
    typed: Origin,
) -> Option<crate::surface::servers::TheAnswer> {
    crate::surface::servers::the_beginning_is_answered(
        crate::servers::a_period::a_period_is_opened_carrying(
            app,
            server,
            crate::servers::a_period::WhoAsked::TheUsersConnectAction,
            Some(typed),
        ),
    )
}

/// **`Keys` S17: *Cancel* — the connect is reported as one that did not
/// happen.** *(Step 10b's first audit round, item 3: *Cancel* reported
/// nothing.)* It is the connection's own claimless sentence, because nothing
/// is known about the machine. **It names the server** (the Creator's note 14
/// on the test copy, 2026-10-03), so it is kept for that server where the user
/// is, as every connect that did not happen is.
#[tauri::command]
pub fn the_passphrase_box_was_cancelled<R: Runtime>(app: AppHandle<R>, server: String) {
    crate::surface::servers::the_connection_was_not_made(&app, &server);
}

// ───────────────────────────────────────────────────────────────────────────
// The events
// ───────────────────────────────────────────────────────────────────────────

/// **The list and its words, in the two routes and never in one string.**
///
/// A6 and X9: prose through the typography point, everything else by the
/// verbatim route, and the two are the complete set. **Every name, fingerprint
/// and path goes by the verbatim route and every word beside it as prose**,
/// because a name joined into prose before the point is exactly the join X7
/// forbids.
pub fn the_screen_says(says: &TheKeysScreen) -> Event {
    let mut event = Event::about(About::TheRun, THE_KEYS_SCREEN)
        .saying("theHeading", Said::Prose(Prose::new(KEYS)))
        .saying("chooseAKey", Said::Prose(Prose::new(CHOOSE_A_KEY)))
        .saying("newKey", Said::Prose(Prose::new(NEW_KEY)))
        .saying("usedBy", Said::Prose(Prose::new(USED_BY)))
        .saying("and", Said::Prose(Prose::new(AND)))
        .saying(
            "aPassphraseIsRemembered",
            Said::Prose(Prose::new(A_PASSPHRASE_IS_REMEMBERED)),
        )
        .saying("clearIt", Said::Prose(Prose::new(CLEAR_IT)))
        .saying("copyPublicKey", Said::Prose(Prose::new(COPY_PUBLIC_KEY)))
        .saying("delete", Said::Prose(Prose::new(DELETE)))
        .saying("remove", Said::Prose(Prose::new(REMOVE)))
        .saying("deleteOnHover", Said::Prose(Prose::new(DELETE_SAYS_ON_HOVER)))
        .saying("removeOnHover", Said::Prose(Prose::new(REMOVE_SAYS_ON_HOVER)))
        .saying("useThisKey", Said::Prose(Prose::new(USE_THIS_KEY)))
        .saying("checking", Said::Prose(Prose::new(CHECKING)))
        .saying(
            "chooseItsNewLocation",
            Said::Prose(Prose::new(CHOOSE_ITS_NEW_LOCATION)),
        )
        .saying(
            "chooseAReplacement",
            Said::Prose(Prose::new(CHOOSE_A_REPLACEMENT)),
        )
        .saying(
            "whatAReplacementCosts",
            Said::Prose(Prose::new(WHAT_A_REPLACEMENT_COSTS)),
        )
        .saying(
            "couldNotCheckTheSavedServers",
            Said::Prose(Prose::new(COULD_NOT_CHECK_THE_SAVED_SERVERS)),
        )
        .saying(
            "rowCount",
            Said::Value(Verbatim::new(says.rows.len().to_string())),
        );

    match &says.mode {
        TheMode::Housekeeping => {
            event = event.saying(
                "whatGoesToAProvider",
                Said::Prose(Prose::new(WHAT_GOES_TO_A_PROVIDER)),
            );
        }
        TheMode::Picking { naming } => {
            event = event
                .saying("picking", Said::Value(Verbatim::new("yes")))
                .saying("choosingAKey", Said::Prose(Prose::new(CHOOSING_A_KEY)))
                .saying("cancelAndGoBack", Said::Prose(Prose::new(CANCEL_AND_GO_BACK)));
            // **`Keys` U11 and Design's own narrowing: the line names the server
            // where anything does, and stops at the act where nothing does.**
            if let Some(naming) = naming {
                event = event
                    .saying("choosingAKeyFor", Said::Prose(Prose::new(CHOOSING_A_KEY_FOR)))
                    .saying("theServerNamed", Said::Value(Verbatim::new(naming.clone())));
            }
        }
    }

    if says.the_tools_are_missing {
        event = event
            .saying("theToolsAreMissing", Said::Value(Verbatim::new("yes")))
            .saying(
                "theToolsAreNotInstalled",
                Said::Prose(Prose::new(THE_TOOLS_ARE_NOT_INSTALLED)),
            )
            .saying("untilTheyAre", Said::Prose(Prose::new(UNTIL_THEY_ARE)));
    }

    for (at, row) in says.rows.iter().enumerate() {
        event = the_row_says(event, at, row);
    }
    event
}

/// One row's own values. **Every one of them is a value**: a name is the user's
/// comment, a fingerprint is what they copy, a path is a path, and a server's
/// name is a server's name.
fn the_row_says(mut event: Event, at: usize, row: &ARow) -> Event {
    event = event
        .saying(
            the_name("row", at, "Identity"),
            Said::Value(Verbatim::new(row.identifier.clone())),
        )
        .saying(
            the_name("row", at, "Path"),
            Said::Value(Verbatim::new(row.path.clone())),
        )
        .saying(
            the_name("row", at, "Kind"),
            Said::Value(Verbatim::new(if row.managed { "managed" } else { "borrowed" })),
        );
    if let Some(name) = &row.name {
        event = event.saying(
            the_name("row", at, "Name"),
            Said::Value(Verbatim::new(name.clone())),
        );
    }
    if let Some(fingerprint) = &row.fingerprint {
        event = event.saying(
            the_name("row", at, "Fingerprint"),
            Said::Value(Verbatim::new(fingerprint.clone())),
        );
    }
    if row.a_passphrase_is_remembered {
        event = event.saying(
            the_name("row", at, "PassphraseHeld"),
            Said::Value(Verbatim::new("yes")),
        );
    }
    if row.the_saved_servers_could_not_be_read {
        event = event.saying(
            the_name("row", at, "ServersUnread"),
            Said::Value(Verbatim::new("yes")),
        );
    }
    event = event.saying(
        the_name("row", at, "UsedByCount"),
        Said::Value(Verbatim::new(row.used_by.len().to_string())),
    );
    for (which, server) in row.used_by.iter().enumerate() {
        event = event.saying(
            the_name("row", at, &format!("UsedBy{which}")),
            Said::Value(Verbatim::new(server.called.clone())),
        );
        if let Some(badge) = server.badge {
            event = event.saying(
                the_name("row", at, &format!("UsedBy{which}Badge")),
                Said::Value(Verbatim::new(badge)),
            );
        }
    }
    event
}

/// **One key's verdict, and the sentence that goes with it.**
///
/// **No verdict is drawn that has not been read** (`Keys` U10a): a row is never
/// shown usable while it is still being asked, and never unusable either — which
/// is why the absence of this event is what the underway mark stands for, rather
/// than a third value on it.
pub fn the_verdict_says(says: &AVerdict) -> Event {
    let mut event = Event::about(About::TheRun, A_KEYS_VERDICT)
        .saying("row", Said::Value(Verbatim::new(says.row.clone())))
        .saying(
            "verdict",
            Said::Value(Verbatim::new(the_verdicts_word(&says.verdict))),
        )
        .saying(
            "thePublicHalfIsBesideIt",
            Said::Value(Verbatim::new(if says.the_public_half_is_beside_it {
                "yes"
            } else {
                "no"
            })),
        );

    if let Verdict::Unusable(reason) = &says.verdict {
        let (first, second) = the_sentences_for(*reason, says.why_it_would_not_read);
        event = event.saying("whatIsWrong", Said::Prose(Prose::new(first)));
        if let Some(second) = second {
            event = event.saying("andWhatToDo", Said::Prose(Prose::new(second)));
        }
    }

    // **`Design/Keys`: the copy control is absent, not dimmed, and one line says
    // why.** A key with no passphrase keeps the row usable and loses only the
    // copy; a protected one has nothing left to read a fingerprint from, and
    // that case is the unusable reason above rather than this line.
    if !says.the_public_half_is_beside_it && says.verdict == Verdict::Usable {
        event = event
            .saying("noPublicKeyFile", Said::Prose(Prose::new(NO_PUBLIC_KEY_FILE)))
            .saying(
                "theKeyItselfWorks",
                Said::Prose(Prose::new(THE_KEY_ITSELF_WORKS)),
            );
    }
    event
}

/// **Crate-visible because a server dialog's key list carries the same three
/// words** — the Creator's eighth notes, note 3.
pub(crate) fn the_verdicts_word(verdict: &Verdict) -> &'static str {
    match verdict {
        Verdict::Usable => "usable",
        Verdict::Unusable(_) => "unusable",
        Verdict::NotChecked => "not-checked",
    }
}

/// **One cause, one sentence, wherever it is said.**
///
/// The four the drawing supplies are its own words, taken from the pointing
/// refusals it draws them at; the fifth and the two situations are ruled at the
/// constants above.
fn the_sentences_for(
    reason: Unusable,
    situation: Option<WhyItWouldNotRead>,
) -> (&'static str, Option<&'static str>) {
    match (reason, situation) {
        (Unusable::FileMissing, _) => (THE_FILE_IS_NOT_THERE, None),
        (
            Unusable::Unreadable,
            Some(WhyItWouldNotRead::ItIsProtectedAndHasNoPublicHalfBeside),
        ) => (
            IT_IS_PROTECTED_AND_HAS_NO_PUBLIC_HALF,
            Some(UNTIL_THAT_FILE_IS_BESIDE_IT),
        ),
        (Unusable::Unreadable, Some(WhyItWouldNotRead::ItIsNotAPrivateKeyContainer)) => {
            (IT_IS_NOT_A_PRIVATE_KEY, None)
        }
        (Unusable::Unreadable, None) => (THE_FILE_COULD_NOT_BE_READ, None),
        (Unusable::NotASupportedType, _) => (NOT_A_SUPPORTED_TYPE, None),
        (Unusable::NoLongerTheSameKey, _) => (A_DIFFERENT_KEY_IS_THERE, None),
        (Unusable::FilePermissionsTooOpen, _) => {
            (THE_PERMISSIONS_ARE_TOO_OPEN, Some(MEND_THE_PERMISSIONS))
        }
    }
}

/// **The answer to the act the user just took.**
pub fn the_answer_says(says: &TheAnswer) -> Event {
    let event = Event::about(About::TheRun, AN_ANSWER);
    match says {
        TheAnswer::Nothing => event.saying("answer", Said::Value(Verbatim::new("nothing"))),
        TheAnswer::TheTypesTheDialogMayOffer(offered) => the_types_say(event, offered),
        TheAnswer::AKeyWasMade {
            at,
            public_half,
            the_sentence_is_owed,
        } => {
            let event = event
            .saying("answer", Said::Value(Verbatim::new("a-key-was-made")))
            .saying("theKeyIsIn", Said::Prose(Prose::new(THE_KEY_IS_IN)))
            .saying(
                "thePath",
                Said::Value(Verbatim::new(at.display().to_string())),
            )
            .saying(
                "itsPublicKeyIsCopied",
                Said::Prose(Prose::new(ITS_PUBLIC_KEY_IS_COPIED)),
            )
            .saying(
                "pasteItAtTheProvider",
                Said::Prose(Prose::new(PASTE_IT_AT_THE_PROVIDER)),
            )
            .saying(
                "theContents",
                Said::Value(Verbatim::new(public_half.clone())),
            )
            .saying("done", Said::Prose(Prose::new(DONE)))
            .saying(
                "thePublicKeyWasNotCopied",
                Said::Prose(Prose::new(THE_PUBLIC_KEY_WAS_NOT_COPIED)),
            )
            .saying("selectTheWholeKey", Said::Prose(Prose::new(SELECT_THE_WHOLE_KEY)));
            // **Said once, at the dialog's last step** — mockup state
            // `keys-madenostore`.
            if *the_sentence_is_owed {
                event.saying("nowhereToKeepThem", Said::Prose(Prose::new(NOWHERE_TO_KEEP_THEM)))
            } else {
                event
            }
        }
        TheAnswer::TheKeyCouldNotBeMade { reported } => {
            let event = event
                .saying("answer", Said::Value(Verbatim::new("the-key-was-not-made")))
                .saying(
                    "theKeyCouldNotBeMade",
                    Said::Prose(Prose::new(THE_KEY_COULD_NOT_BE_MADE)),
                );
            // **`crate::keys::askpass`'s own sentence, which is about the user's
            // key and their computer rather than about a part of Smoozy.** It is
            // prose and it is Smoozy's own, not a diagnostic of the tool's.
            match reported {
                Some(reported) => event.saying("whyNot", Said::Prose(Prose::new(reported.clone()))),
                None => event,
            }
        }
        TheAnswer::TheMakeWasRefused(why) => event
            .saying(
                "answer",
                Said::Value(Verbatim::new("the-make-was-refused")),
            )
            .saying(
                "theActDidNotLand",
                Said::Prose(Prose::new(the_sentence_for_a_refused_make(*why))),
            ),
        TheAnswer::APointWasRefused {
            refused,
            picked,
            the_other_name,
            the_rows_fingerprint,
            the_picked_fingerprint,
        } => {
            let event = event
                .saying(
                    "thePickedPath",
                    Said::Value(Verbatim::new(picked.display().to_string())),
                );
            let event = match the_other_name {
                Some(name) => event.saying("theOtherName", Said::Value(Verbatim::new(name.clone()))),
                None => event,
            };
            let event = match the_rows_fingerprint {
                Some(held) => {
                    event.saying("theRowsFingerprint", Said::Value(Verbatim::new(held.clone())))
                }
                None => event,
            };
            let event = match the_picked_fingerprint {
                Some(held) => event.saying(
                    "thePickedFingerprint",
                    Said::Value(Verbatim::new(held.clone())),
                ),
                None => event,
            };
            the_refusal_says(event, refused)
        }
        TheAnswer::AnotherKeyIsAlreadyCalledThat {
            name,
            the_other_fingerprint,
        } => {
            let event = event
                .saying("answer", Said::Value(Verbatim::new("the-same-name")))
                .saying(
                    "anotherKeyIsAlreadyCalled",
                    Said::Prose(Prose::new(ANOTHER_KEY_IS_ALREADY_CALLED)),
                )
                .saying("theName", Said::Value(Verbatim::new(name.clone())))
                .saying("bothAreKept", Said::Prose(Prose::new(BOTH_ARE_KEPT)))
                .saying("done", Said::Prose(Prose::new(DONE)));
            match the_other_fingerprint {
                Some(fingerprint) => event
                    .saying(
                        "theOneAlreadyInTheListHolds",
                        Said::Prose(Prose::new(THE_ONE_ALREADY_IN_THE_LIST_HOLDS)),
                    )
                    .saying(
                        "theOtherFingerprint",
                        Said::Value(Verbatim::new(fingerprint.clone())),
                    ),
                None => event,
            }
        }
        TheAnswer::WhatTakingThisKeyAwayCosts {
            row,
            managed,
            it_asks,
            used_by,
            the_saved_servers_could_not_be_read,
            the_files,
        } => {
            let mut event = event
                .saying("answer", Said::Value(Verbatim::new("taking-a-key-away")))
                .saying("row", Said::Value(Verbatim::new(row.clone())))
                .saying(
                    "heading",
                    Said::Prose(Prose::new(if *managed {
                        DELETE_THIS_KEY
                    } else {
                        REMOVE_THIS_KEY
                    })),
                )
                .saying(
                    "theControl",
                    Said::Prose(Prose::new(if *managed { DELETE } else { REMOVE })),
                )
                .saying(
                    "managed",
                    Said::Value(Verbatim::new(if *managed { "yes" } else { "no" })),
                )
                .saying(
                    "itAsks",
                    Said::Value(Verbatim::new(if *it_asks { "yes" } else { "no" })),
                )
                .saying("cancel", Said::Prose(Prose::new(CANCEL)))
                .saying(
                    "usedByCount",
                    Said::Value(Verbatim::new(used_by.len().to_string())),
                );
            for (which, server) in used_by.iter().enumerate() {
                event = event.saying(
                    the_name("usedBy", which, ""),
                    Said::Value(Verbatim::new(server.called.clone())),
                );
                if let Some(badge) = server.badge {
                    event = event.saying(
                        the_name("usedBy", which, "Badge"),
                        Said::Value(Verbatim::new(badge)),
                    );
                }
            }
            if !used_by.is_empty() {
                event = event
                    .saying(
                        "theyConnectWithIt",
                        Said::Prose(Prose::new(if used_by.len() == 1 {
                            ONE_SERVER_CONNECTS_WITH_IT
                        } else {
                            SERVERS_CONNECT_WITH_IT
                        })),
                    )
                    .saying(
                        "thatCoversWhatSmoozyKnows",
                        Said::Prose(Prose::new(THAT_COVERS_WHAT_SMOOZY_KNOWS)),
                    )
                    .saying(
                        "otherToolsAndComputers",
                        Said::Prose(Prose::new(OTHER_TOOLS_AND_COMPUTERS)),
                    );
            }
            if *the_saved_servers_could_not_be_read {
                event = event.saying(
                    "couldNotCheckTheSavedServers",
                    Said::Prose(Prose::new(COULD_NOT_CHECK_THE_SAVED_SERVERS)),
                );
            }
            event = event.saying(
                "fileCount",
                Said::Value(Verbatim::new(the_files.len().to_string())),
            );
            if !the_files.is_empty() {
                event = event.saying(
                    "theseFilesAreDeleted",
                    Said::Prose(Prose::new(THESE_FILES_ARE_DELETED)),
                );
            }
            for (which, path) in the_files.iter().enumerate() {
                event = event.saying(
                    the_name("file", which, ""),
                    Said::Value(Verbatim::new(path.display().to_string())),
                );
            }
            event
        }
        TheAnswer::ThePublicHalfIsCopied {
            contents,
            the_comparison_ran,
        } => {
            let event = event
                .saying("answer", Said::Value(Verbatim::new("copied")))
                .saying(
                    "theContents",
                    Said::Value(Verbatim::new(contents.clone())),
                )
                .saying(
                    "thePublicKeyIsCopied",
                    Said::Prose(Prose::new(THE_PUBLIC_KEY_IS_COPIED)),
                )
                .saying(
                    "thePublicKeyWasNotCopied",
                    Said::Prose(Prose::new(THE_PUBLIC_KEY_WAS_NOT_COPIED)),
                )
                .saying("selectTheWholeKey", Said::Prose(Prose::new(SELECT_THE_WHOLE_KEY)));
            if *the_comparison_ran {
                event
            } else {
                event.saying(
                    "theComparisonCouldNotBeRun",
                    Said::Prose(Prose::new(THE_COMPARISON_COULD_NOT_BE_RUN)),
                )
            }
        }
        TheAnswer::TakingAwayEnded {
            row,
            the_key_was_not_deleted,
            left_behind,
            the_passphrase_was_not_cleared,
        } => {
            let mut event = event
                .saying("answer", Said::Value(Verbatim::new("taken-away")))
                .saying("row", Said::Value(Verbatim::new(row.clone())))
                .saying("done", Said::Prose(Prose::new(DONE)))
                .saying(
                    "leftBehindCount",
                    Said::Value(Verbatim::new(left_behind.len().to_string())),
                );
            if *the_key_was_not_deleted {
                event = event.saying(
                    "theKeyWasNotDeleted",
                    Said::Prose(Prose::new(THE_KEY_WAS_NOT_DELETED)),
                );
            }
            if !left_behind.is_empty() {
                event = event.saying(
                    "thisFileWasNotDeleted",
                    Said::Prose(Prose::new(THIS_FILE_WAS_NOT_DELETED)),
                );
            }
            for (which, path) in left_behind.iter().enumerate() {
                event = event.saying(
                    the_name("leftBehind", which, ""),
                    Said::Value(Verbatim::new(path.display().to_string())),
                );
            }
            if *the_passphrase_was_not_cleared {
                event = event.saying(
                    "thePassphraseWasNotCleared",
                    Said::Prose(Prose::new(THE_PASSPHRASE_WAS_NOT_CLEARED)),
                );
            }
            event
        }
        TheAnswer::ThePassphraseWasNotCleared { row } => event
            .saying("answer", Said::Value(Verbatim::new("not-cleared")))
            .saying("row", Said::Value(Verbatim::new(row.clone())))
            .saying(
                "thePassphraseWasNotCleared",
                Said::Prose(Prose::new(THE_PASSPHRASE_WAS_NOT_CLEARED)),
            ),
        TheAnswer::TheCopyIsUnavailable(why) => event
            .saying("answer", Said::Value(Verbatim::new("copy-unavailable")))
            .saying(
                "whyNot",
                Said::Prose(Prose::new(match why {
                    WhyNot::ThereIsNoPublicHalf => NO_PUBLIC_KEY_FILE,
                    WhyNot::ItIsADifferentKeysPublicHalf => A_DIFFERENT_KEYS_PUBLIC_HALF,
                })),
            ),
    }
}

/// **`Keys` T2 and `Keys` T2a's three readings of the type control.**
fn the_types_say(event: Event, offered: &OfferedSet) -> Event {
    let event = event
        .saying("answer", Said::Value(Verbatim::new("the-key-types")))
        .saying("newKeyHeading", Said::Prose(Prose::new(NEW_KEY_HEADING)))
        .saying("theNameField", Said::Prose(Prose::new(THE_NAME_FIELD)))
        .saying(
            "nameItAfterTheProvider",
            Said::Prose(Prose::new(NAME_IT_AFTER_THE_PROVIDER)),
        )
        .saying(
            "thePassphraseField",
            Said::Prose(Prose::new(THE_PASSPHRASE_FIELD)),
        )
        .saying("optional", Said::Prose(Prose::new(OPTIONAL)))
        .saying("advancedKeyType", Said::Prose(Prose::new(ADVANCED_KEY_TYPE)))
        .saying("keyType", Said::Prose(Prose::new(KEY_TYPE)))
        .saying("makeItAndCopy", Said::Prose(Prose::new(MAKE_IT_AND_COPY)))
        .saying("cancel", Said::Prose(Prose::new(CANCEL)));

    // **`Keys` G6: where the probe answers *no generator*, the dialog is
    // replaced by the statement rather than opened beside it.**
    if matches!(offered, OfferedSet::NoGenerator) {
        return event.saying("theToolsAreMissing", Said::Value(Verbatim::new("yes")));
    }
    if matches!(offered, OfferedSet::StillReading) {
        return event.saying("stillReading", Said::Value(Verbatim::new("yes")));
    }

    let default = offered.default_type();
    let mut event = event.saying(
        "typeCount",
        Said::Value(Verbatim::new(offered.offerable().len().to_string())),
    );
    for (at, kind) in offered.offerable().iter().enumerate() {
        event = event.saying(
            the_name("type", at, ""),
            Said::Value(Verbatim::new(kind.as_str())),
        );
        if Some(*kind) == default {
            event = event.saying(
                the_name("type", at, "IsTheDefault"),
                Said::Value(Verbatim::new("yes")),
            );
        }
    }
    // **`Keys` T4 and `Design/Keys § Making a key`: where the usual one cannot
    // be made, the next in the fixed order is the default and one line says
    // so.** Mockup state `keys-advfall`.
    let the_usual = KeyType::SUPPORTED[0];
    if !offered.offerable().contains(&the_usual) && default.is_some() {
        event = event
            .saying(
                "theUsualType",
                Said::Value(Verbatim::new(the_usual.as_str())),
            )
            .saying(
                "theUsualCannotBeMade",
                Said::Prose(Prose::new(IS_THE_USUAL_CHOICE_AND_CANNOT_BE_MADE)),
            )
            .saying(
                "theDefaultMark",
                Said::Prose(Prose::new(SMOOZYS_CHOICE_HERE)),
            );
    } else {
        event = event.saying(
            "theDefaultMark",
            Said::Prose(Prose::new(SMOOZYS_USUAL_CHOICE)),
        );
    }
    event
}

/// **The refusals, each saying what failed and nothing more** (`Keys` U6, F4,
/// F5, F6). **Two of them must not read alike** and that is the whole reason the
/// situation rides beside the closed set.
fn the_refusal_says(event: Event, refused: &Refused) -> Event {
    let event = event
        .saying("answer", Said::Value(Verbatim::new("a-point-was-refused")))
        .saying("back", Said::Prose(Prose::new(BACK)));
    match refused {
        Refused::TheClauseThatFailed { clause, situation } => {
            let (first, second) = the_sentences_for(*clause, *situation);
            let heading = match (clause, situation) {
                (Unusable::Unreadable, Some(WhyItWouldNotRead::ItIsNotAPrivateKeyContainer)) => {
                    THAT_IS_NOT_A_KEY
                }
                (
                    Unusable::Unreadable,
                    Some(WhyItWouldNotRead::ItIsProtectedAndHasNoPublicHalfBeside),
                ) => SMOOZY_CANNOT_CHECK_THAT_KEY,
                (Unusable::Unreadable, None) | (Unusable::FileMissing, _) => {
                    THAT_FILE_COULD_NOT_BE_READ
                }
                _ => SMOOZY_CANNOT_USE_THAT_KEY,
            };
            let event = event
                .saying("heading", Said::Prose(Prose::new(heading)))
                .saying("whatIsWrong", Said::Prose(Prose::new(first)));
            match second {
                Some(second) => event.saying("andWhatToDo", Said::Prose(Prose::new(second))),
                // **The drawing's own second half on the three refusals that
                // have no mend**: nothing was touched.
                None => event.saying(
                    "nothingHasChanged",
                    Said::Prose(Prose::new(NOTHING_HAS_CHANGED)),
                ),
            }
        }
        Refused::ThisKeyIsAlreadyListed { as_row } => event
            .saying(
                "heading",
                Said::Prose(Prose::new(SMOOZY_ALREADY_HAS_THAT_KEY)),
            )
            .saying(
                "itIsInTheListAs",
                Said::Prose(Prose::new(IT_IS_IN_THE_LIST_AS)),
            )
            .saying("at", Said::Prose(Prose::new(AT)))
            .saying(
                "theOtherPath",
                Said::Value(Verbatim::new(as_row.display().to_string())),
            ),
        Refused::TwoKeysSmoozyCannotTellApart { the_other_row } => event
            .saying(
                "heading",
                Said::Prose(Prose::new(TWO_KEYS_SMOOZY_CANNOT_TELL_APART)),
            )
            .saying(
                "whatIsWrong",
                Said::Prose(Prose::new(TWO_KEYS_ONE_FINGERPRINT)),
            )
            .saying(
                "theOtherPath",
                Said::Value(Verbatim::new(the_other_row.display().to_string())),
            ),
        Refused::NotThisKey => event
            .saying(
                "heading",
                Said::Prose(Prose::new(THAT_IS_A_KEY_AND_NOT_THIS_ONE)),
            )
            .saying(
                "whatIsWrong",
                Said::Prose(Prose::new(THIS_ROW_IS_ABOUT_A_DIFFERENT_KEY)),
            )
            .saying("itHolds", Said::Prose(Prose::new(IT_HOLDS)))
            .saying("youPointedAt", Said::Prose(Prose::new(YOU_POINTED_AT)))
            .saying(
                "useChooseAReplacement",
                Said::Prose(Prose::new(USE_CHOOSE_A_REPLACEMENT)),
            ),
        // **`Keys` U6a: the statement is the screen's own, said once above the
        // list**, and a refusal that composed a second one here would be the
        // *not checked* state turned into a per-key verdict.
        Refused::TheSystemsSshToolsAreNotInstalled => event.saying(
            "theToolsAreMissing",
            Said::Value(Verbatim::new("yes")),
        ),
    }
}

/// **`Keys` S15: what the box is handed**, and every value is a display-only
/// field of a row that is already held.
pub fn the_ask_says(row: &str, ask: &TheAsk) -> Event {
    let mut event = Event::about(About::TheRun, THE_PASSPHRASE_ASK)
        .saying("row", Said::Value(Verbatim::new(row.to_string())))
        .saying("passphraseFor", Said::Prose(Prose::new(PASSPHRASE_FOR)))
        .saying(
            "thePassphraseField",
            Said::Prose(Prose::new(THE_PASSPHRASE_FIELD)),
        )
        .saying("cancel", Said::Prose(Prose::new(CANCEL)))
        .saying("connect", Said::Prose(Prose::new(CONNECT)));

    // **What exists, and nothing where there is none** — `Keys` U11's own shape.
    match &ask.key {
        HowTheKeyIsNamed::ByItsName(name) => {
            event = event.saying(
                "theKeyNamed",
                Said::Value(Verbatim::new(name.as_str().to_string())),
            );
        }
        HowTheKeyIsNamed::ByItsFingerprint(fingerprint) => {
            event = event
                .saying(
                    "theKeyNamed",
                    Said::Value(Verbatim::new(fingerprint.as_str().to_string())),
                )
                .saying("theKeyIsNamedByItsFingerprint", Said::Value(Verbatim::new("yes")));
        }
        HowTheKeyIsNamed::ByNeither => {}
    }

    if let Some(named) = the_context_names(&DialogContext {
        label: ask.server.label.clone(),
        username: ask.server.username.clone(),
        host: ask.server.host.clone(),
    }) {
        event = event
            .saying("connectingTo", Said::Prose(Prose::new(CONNECTING_TO)))
            .saying("theServerNamed", Said::Value(Verbatim::new(named)));
    }

    // **`Keys` S19's line, in two sentences.** Mockup state `keys-pass2`.
    if ask.the_forgotten_passphrase_line {
        event = event
            .saying(
                "theRememberedPassphraseNoLongerOpensIt",
                Said::Prose(Prose::new(THE_REMEMBERED_PASSPHRASE_NO_LONGER_OPENS_IT)),
            )
            .saying(
                "smoozyHasForgottenIt",
                Said::Prose(Prose::new(SMOOZY_HAS_FORGOTTEN_IT)),
            );
    }
    event
}

/// **`Keys` U11: what exists, and nothing where there is none.**
///
/// The label where there is one, `username@host` where both are there, and
/// **nothing at all where the caller has neither** — the Add Server route, where
/// no value naming that server exists anywhere. `Design/Keys` settled that the
/// line then reads *Choosing a key* and stops.
fn the_context_names(context: &DialogContext) -> Option<String> {
    if let Some(label) = context.label.as_deref().map(str::trim) {
        if !label.is_empty() {
            return Some(label.to_string());
        }
    }
    match (
        context.username.as_deref().map(str::trim),
        context.host.as_deref().map(str::trim),
    ) {
        (Some(username), Some(host)) if !username.is_empty() && !host.is_empty() => {
            Some(format!("{username}@{host}"))
        }
        _ => None,
    }
}

/// The keys an indexed field takes on the event. An event's field names are
/// `&'static str` by [`Event::saying`]'s own signature, so each name is made
/// once and **kept after it is first made** — the same name on the next draw
/// is the one already made, and memory grows with the most rows ever drawn
/// rather than with every event (step 10a round 1, fix-here item 31).
pub(crate) fn the_name(prefix: &str, at: usize, tail: &str) -> &'static str {
    static MADE: std::sync::OnceLock<Mutex<std::collections::HashSet<&'static str>>> =
        std::sync::OnceLock::new();
    let wanted = format!("{prefix}{at}{tail}");
    let made = MADE.get_or_init(|| Mutex::new(std::collections::HashSet::new()));
    let Ok(mut made) = made.lock() else {
        return Box::leak(wanted.into_boxed_str());
    };
    if let Some(already) = made.get(wanted.as_str()) {
        return already;
    }
    let kept: &'static str = Box::leak(wanted.into_boxed_str());
    made.insert(kept);
    kept
}

// ───────────────────────────────────────────────────────────────────────────
// The record, the store, the diary, the two locations
// ───────────────────────────────────────────────────────────────────────────

fn the_screen_is_posted_again<R: Runtime>(app: &AppHandle<R>) {
    // **The mode is not remembered here.** The page asks again with whichever
    // reading it is standing in, which is P8's *events are volatile and nothing
    // is replayed* kept rather than a second copy of what the user is looking
    // at.
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    let _ = queue.inner().read_then_post::<(), std::convert::Infallible>(
        || Ok(()),
        |()| Event::about(About::TheRun, AN_ANSWER).saying("answer", Said::Value(Verbatim::new("ask-again"))),
    );
}

fn the_answer_is_posted<R: Runtime>(app: &AppHandle<R>, answer: TheAnswer) {
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    let _ = queue.inner().read_then_post::<TheAnswer, std::convert::Infallible>(
        || Ok(answer.clone()),
        the_answer_says,
    );
}

fn the_row_named<R: Runtime>(app: &AppHandle<R>, row: &str) -> Option<KeyRow> {
    let identity = Identity::read_back(IdentityKind::LocalKey, row)?;
    let started = app.try_state::<Mutex<Started>>()?;
    let started = started.lock().ok()?;
    let record = started.record.as_ref()?;
    key_rows::all(record.connection())
        .ok()?
        .into_iter()
        .find(|held| held.identifier == identity)
}

/// **One read of the server rows, with the badge beside each** — `Keys` W11a.
///
/// **A read that will not answer is `Keys` W12's third state**, carried as `None` and
/// never as an empty list, because *no saved server uses this key* is said only
/// on a successful read.
///
/// **A saved server whose fingerprint will not read is the third state too**,
/// never a server left out (step 10a round 1, fix-here item 14): `Keys` W11
/// matches on the fingerprint, so a server whose fingerprint is not one could
/// be using any key, and leaving it out would let a delete warn about nothing.
fn the_saved_servers(
    connection: &rusqlite::Connection,
) -> Option<Vec<(SavedServer, Option<&'static str>)>> {
    let rows = server_rows::all(connection).ok()?;
    rows.iter()
        .map(|row| {
            let fingerprint = Fingerprint::from_the_printed_form(row.key.fingerprint.as_str())?;
            Some((
                SavedServer {
                    label: crate::surface::servers::what_a_server_is_called_among(row, &rows),
                    fingerprint,
                },
                the_badge(row.colour.as_deref()),
            ))
        })
        .collect()
}

/// **`Keys` G11: the default name, read through the application-identity
/// adapter at the moment it is needed and nowhere else.**
fn the_default_name_now() -> String {
    let names = crate::platform::key_names::the_two_names();
    naming::default_name(
        names.login_account_name.as_deref().unwrap_or_default(),
        names.computer_name.as_deref().unwrap_or_default(),
    )
}

/// **L3a's temporary area**, which is where the askpass channel is opened and
/// where the one reading that needs a scratch file writes. It is never a
/// directory any module scans for keys and it is swept at application start.
///
/// **Nothing where it cannot be named, and never the shared temporary
/// directory in its place** — `crate::platform::locations::smoozys_own_temporary_area`,
/// step 10b's first audit round, item 16. **The area sits under the same
/// per-user location as the local record**, so a computer that cannot name it
/// has no record open either and every screen here is already drawing that
/// failure; the acts that would write into the area make no attempt.
fn the_temporary_area() -> Option<PathBuf> {
    locations::smoozys_own_temporary_area()
}

/// **The secret store for this system.**
///
/// `crate::surface::servers` shares this factory.
pub(crate) fn the_store_for_this_system() -> Option<Box<dyn SecretStore>> {
    #[cfg(target_os = "macos")]
    {
        Some(Box::new(crate::platform::secrets::MacKeychain))
    }
    #[cfg(target_os = "windows")]
    {
        Some(Box::new(crate::platform::secrets::WindowsCredentialManager))
    }
    #[cfg(target_os = "linux")]
    {
        crate::platform::secrets::LinuxSecretService::open()
            .map(|store| Box::new(store) as Box<dyn SecretStore>)
    }
}

/// **A store that answers nothing, for the computer that has none.**
///
/// `Security and licence` S25's conservative reading is that there may be
/// nowhere to remember, and `Keys` G5 already says what happens then: the put
/// fails, the key is whole, the passphrase is not held, and the first connect
/// asks. **`Keys` S8 makes that answer permanent for the life of such an
/// installation**, and nothing here softens it into a retry.
struct NowhereToRemember;

impl SecretStore for NowhereToRemember {
    fn put(
        &self,
        _item: &smoozy_security_and_licence::held::HeldSecret,
        _value: smoozy_security_and_licence::plaintext::Plaintext,
        _deadline: Duration,
    ) -> Result<(), smoozy_security_and_licence::secret_store::StoreFailure> {
        Err(smoozy_security_and_licence::secret_store::StoreFailure::StoreUnavailable)
    }

    fn get(
        &self,
        _item: &smoozy_security_and_licence::held::HeldSecret,
        _deadline: Duration,
    ) -> Result<
        smoozy_security_and_licence::plaintext::Plaintext,
        smoozy_security_and_licence::secret_store::GetFailure,
    > {
        Err(smoozy_security_and_licence::secret_store::GetFailure::StoreUnavailable)
    }

    fn erase(
        &self,
        _item: &smoozy_security_and_licence::held::HeldSecret,
        _deadline: Duration,
    ) -> Result<(), smoozy_security_and_licence::secret_store::StoreFailure> {
        Err(smoozy_security_and_licence::secret_store::StoreFailure::StoreUnavailable)
    }

    fn probe(
        &self,
        _deadline: Duration,
    ) -> smoozy_security_and_licence::secret_store::ProbeOutcome {
        smoozy_security_and_licence::secret_store::ProbeOutcome::RoundTripFailed
    }

    fn enumerate(
        &self,
        _deadline: Duration,
    ) -> Result<
        Vec<smoozy_security_and_licence::held::HeldSecret>,
        smoozy_security_and_licence::secret_store::StoreFailure,
    > {
        Err(smoozy_security_and_licence::secret_store::StoreFailure::StoreUnavailable)
    }
}

const NOWHERE: NowhereToRemember = NowhereToRemember;

pub(crate) fn the_store(held: &Option<Box<dyn SecretStore>>) -> &dyn SecretStore {
    match held {
        Some(store) => store.as_ref(),
        None => &NOWHERE,
    }
}

/// The diary this module writes through, where the run has one.
fn the_modules_diary<'a>(started: &'a Started) -> Option<crate::diary_service::ModulesDiary<'a>> {
    let service = started.diary.as_ref()?;
    let run = started.run.as_ref()?;
    Some(crate::diary_service::ModulesDiary {
        service,
        declarations: &started.declarations,
        run: run.clone(),
        module: THE_MODULE,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_keys::fingerprint::{ReadFrom, Reading};
    use smoozy_keys::rows::Name;
    use std::collections::BTreeSet;
    use std::path::Path;

    fn the_page_module() -> String {
        std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("web/keys.js"),
        )
        .expect("web/keys.js is readable")
    }

    fn this_file() -> &'static str {
        include_str!("keys.rs")
    }

    fn the_mockup() -> String {
        std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../Preparation/Design/Mockup/Smoozy.html"),
        )
        .expect("the mockup is readable")
    }

    fn a_row(name: Option<&str>, path: &str, fingerprint: Option<&str>) -> KeyRow {
        KeyRow::entering(
            Identity::issue(IdentityKind::LocalKey).expect("randomness"),
            PathBuf::from(path),
            fingerprint.map(|printed| Reading {
                value: Fingerprint::from_the_printed_form(printed).expect("a fingerprint"),
                read_from: ReadFrom::PrivateFile,
            }),
            name.and_then(Name::what_the_user_typed),
            false,
        )
        .expect("a row")
    }

    /// **The twenty-one this piece draws are the twenty-one the drawing has, in both
    /// directions.**
    ///
    /// **This is the reverse leg the runner's ruling of 2026-09-12 asked for.**
    /// The counted gate cannot see this half at all — the ten Design documents
    /// carry zero clause markers between them — so what a screen piece is
    /// counted by is the mockup state names it draws, and this is that count
    /// made mechanical rather than read. **A twenty-second appearing in the drawing
    /// fails here**, and so does one of the twenty-one going missing from this
    /// file's own list.
    #[test]
    fn the_twenty_one_states_this_piece_draws_are_the_drawings_own() {
        let mockup = the_mockup();
        let opens = mockup
            .find("<select id=\"state\"")
            .expect("the drawing has its state list");
        let closes = mockup[opens..].find("</select>").expect("the list closes") + opens;
        let list = &mockup[opens..closes];

        let mut drawn = BTreeSet::new();
        for (at, _) in list.match_indices("<option value=\"") {
            let rest = &list[at + "<option value=\"".len()..];
            let name = &rest[..rest.find('"').expect("the value closes")];
            if name == "keys" || name.starts_with("keys-") {
                drawn.insert(name.to_string());
            }
        }
        assert_eq!(
            drawn.len(),
            21,
            "the drawing's `keys-` set has moved: {drawn:?}"
        );

        let head = this_file();
        for state in &drawn {
            assert!(
                head.contains(&format!("`{state}`")),
                "the drawing has the state `{state}` and this file names it nowhere. \
                 **The runner ruled on 2026-09-12 that the mockup state names are \
                 what this half is counted by**, so a state nothing here names is a \
                 drawing nobody built"
            );
        }
    }

    /// **Every name the event can carry is read by the page.**
    ///
    /// **This is F-13's fault in the other direction**: a string the application
    /// sends and the page drops is a word the user does not see, with every
    /// other test passing. It walks every shape this file composes and fails on
    /// one name `web/keys.js` never reads.
    #[test]
    fn every_name_the_event_can_carry_is_read_by_the_page() {
        let page = the_page_module();
        let mut names = BTreeSet::new();

        let a_screen = |mode: TheMode, missing: bool| TheKeysScreen {
            mode,
            the_tools_are_missing: missing,
            rows: vec![ARow {
                identifier: "a".into(),
                name: Some("a name".into()),
                fingerprint: Some("SHA256:x".into()),
                path: "/a/path".into(),
                used_by: vec![
                    AServerUsingIt {
                        called: "one".into(),
                        badge: Some("b1"),
                    },
                    AServerUsingIt {
                        called: "two".into(),
                        badge: None,
                    },
                ],
                the_saved_servers_could_not_be_read: true,
                a_passphrase_is_remembered: true,
                managed: true,
            }],
        };
        for mode in [
            TheMode::Housekeeping,
            TheMode::Picking { naming: None },
            TheMode::Picking {
                naming: Some("a server".into()),
            },
        ] {
            for missing in [false, true] {
                names.extend(the_names_on(&the_screen_says(&a_screen(mode.clone(), missing))));
            }
        }

        for verdict in [
            Verdict::Usable,
            Verdict::NotChecked,
            Verdict::Unusable(Unusable::FileMissing),
            Verdict::Unusable(Unusable::Unreadable),
            Verdict::Unusable(Unusable::NotASupportedType),
            Verdict::Unusable(Unusable::NoLongerTheSameKey),
            Verdict::Unusable(Unusable::FilePermissionsTooOpen),
        ] {
            for why in [
                None,
                Some(WhyItWouldNotRead::ItIsProtectedAndHasNoPublicHalfBeside),
                Some(WhyItWouldNotRead::ItIsNotAPrivateKeyContainer),
            ] {
                for beside in [true, false] {
                    names.extend(the_names_on(&the_verdict_says(&AVerdict {
                        row: "a".into(),
                        verdict: verdict.clone(),
                        why_it_would_not_read: why,
                        the_public_half_is_beside_it: beside,
                    })));
                }
            }
        }

        for answer in the_answers() {
            names.extend(the_names_on(&the_answer_says(&answer)));
        }

        for named in [
            HowTheKeyIsNamed::ByItsName(Name::what_the_user_typed("a key").expect("a name")),
            HowTheKeyIsNamed::ByItsFingerprint(
                Fingerprint::from_the_printed_form("SHA256:x").expect("a fingerprint"),
            ),
            HowTheKeyIsNamed::ByNeither,
        ] {
            for line in [true, false] {
                names.extend(the_names_on(&the_ask_says(
                    "a",
                    &TheAsk {
                        server: ServerContext {
                            label: Some("a server".into()),
                            username: None,
                            host: None,
                        },
                        key: named.clone(),
                        the_forgotten_passphrase_line: line,
                    },
                )));
            }
        }

        let mut dropped = Vec::new();
        for name in names {
            // The indexed fields are read by their prefix on the page, which is
            // what an index means.
            let looked_for = the_prefix_of(&name);
            if !page.contains(&looked_for) {
                dropped.push(name);
            }
        }
        assert!(
            dropped.is_empty(),
            "the application can put these on the event and `web/keys.js` reads \
             none of them, so the user never meets them: {dropped:?}"
        );
    }

    /// **Fix-here item 32's *New key* half and item 20's control, together, and
    /// it runs the page code rather than reading it.**
    ///
    /// **The one press the application refuses left the dialog unusable.** Item
    /// 20 disables *Create and copy public key* and marks it underway on the press, and
    /// before this repair only an answer brought it back — so a press the
    /// application refused in silence left the control dead and marked for the
    /// rest of the run, and the user could not so much as try again.
    ///
    /// The harness **presses, is refused, and presses again**. It wants two
    /// asks, and the refusal's own sentence drawn in the dialog. **Before the
    /// repair it got one ask and no sentence**: the refusal reached the page as
    /// an answer nothing handled, so nothing was drawn and the control stayed
    /// disabled.
    ///
    /// **Skipped, and says so, where no Node is installed** — the shape
    /// `src/surface/servers.rs`'s own harness uses.
    #[test]
    fn a_refused_press_says_so_and_the_make_control_comes_back() {
        let folder = std::env::temp_dir().join(format!(
            "smoozy-keys-page-harness-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/keys.js");
        std::fs::copy(&page, folder.join("keys.js")).expect("the page is copied");
        for (name, text) in THE_STAND_INS {
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node")
            .arg(folder.join("harness.js"))
            .current_dir(&folder)
            .output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Keys screen's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(
            ran.status.success(),
            "the page did not run: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&ran.stdout).trim(),
            format!(
                r#"{{"asksAfterTheFirstPress":1,"theRefusalSaid":"{THIS_NAME_IS_TOO_LONG}","asksAfterTheSecondPress":2}}"#
            )
        );
    }

    /// The stand-ins the harness above loads `web/keys.js` beside.
    const THE_STAND_INS: [(&str, &str); 7] = [
        // The real drawing of a value, which the page imports (step 10c,
        // dependency wave piece 10a).
        ("the-value-drawn.js", include_str!("../../web/the-value-drawn.js")),
        ("package.json", r#"{ "type": "module" }"#),
        (
            "event-route.js",
            r#"export function whenTold(kind, handler) {
  const held = globalThis.told.get(kind) || [];
  held.push(handler);
  globalThis.told.set(kind, held);
}"#,
        ),
        (
            "the-rail.js",
            r#"export const KEYS = "keysview";
export function aScreenIsRegistered(screen) { globalThis.registered = screen; }
export function theCurrentPlaceIs() {}"#,
        ),
        (
            "the-failure-shape.js",
            r#"export const EXPANDED = "expanded";
export function theFailureShape(parts) {
  const drawn = document.createElement("span");
  drawn.classList.add("afailure");
  drawn.textContent = (parts && parts.sentence) || "";
  return drawn;
}"#,
        ),
        (
            "the-underway-shape.js",
            r#"export function theMarkOnAControl(control, label) {
  control.classList.add("uway");
  control.textContent = label;
  return control;
}
export function theMarkComesOffAControl(control, label) {
  control.classList.remove("uway");
  control.textContent = label;
  return control;
}"#,
        ),
        (
            "harness.js",
            r#"function anElement(tag) {
  const names = new Set();
  const listeners = new Map();
  const element = {
    tag, children: [], parentNode: null, dataset: {}, hidden: false, text: "",
    disabled: false, value: "", type: "", id: "", placeholder: "", htmlFor: "",
    style: { setProperty() {} },
    classList: {
      add: (...more) => more.forEach((one) => names.add(one)),
      remove: (...less) => less.forEach((one) => names.delete(one)),
      toggle: (one, on) => { if (on === undefined ? !names.has(one) : on) names.add(one); else names.delete(one); },
      contains: (one) => names.has(one),
    },
    set className(value) { names.clear(); String(value).split(" ").filter(Boolean).forEach((one) => names.add(one)); },
    get className() { return Array.from(names).join(" "); },
    set textContent(value) { element.children = []; element.text = String(value); },
    get textContent() { return element.text; },
    get childNodes() { return element.children; },
    get childElementCount() { return element.children.length; },
    appendChild(child) { child.parentNode = element; element.children.push(child); return child; },
    insertBefore(child) { return element.appendChild(child); },
    removeChild(child) { element.children = element.children.filter((one) => one !== child); return child; },
    replaceChildren(...fresh) { element.children = []; fresh.forEach((one) => element.appendChild(one)); },
    setAttribute() {}, removeAttribute() {}, focus() {},
    addEventListener(kind, handler) {
      const held = listeners.get(kind) || [];
      held.push(handler);
      listeners.set(kind, held);
    },
    // **A control that is disabled takes no press**, which is what the browser
    // does and is the whole subject of this harness.
    click() {
      if (element.disabled) return;
      for (const one of listeners.get("click") || []) one({ target: element, preventDefault() {} });
    },
    querySelector() { return null; }, querySelectorAll() { return []; }, closest() { return null; },
    contains() { return false; },
    getBoundingClientRect() { return { left: 0, bottom: 0, width: 0, height: 0 }; },
  };
  return element;
}
function findAll(element, fits, found = []) {
  if (element && element.classList && fits(element)) found.push(element);
  for (const child of (element && element.children) || []) findAll(child, fits, found);
  return found;
}
globalThis.told = new Map();
globalThis.document = {
  createElement: anElement,
  createTextNode: (text) => ({ text, children: [] }),
  addEventListener() {}, removeEventListener() {}, querySelector() { return null; },
};
globalThis.window = { innerWidth: 1000, innerHeight: 800 };
let asks = 0;
window.__TAURI__ = {
  core: {
    invoke(command) {
      if (command === "a_key_is_made") asks += 1;
      return Promise.resolve(null);
    },
  },
};
const settle = () => new Promise((done) => setImmediate(done));
const tell = (kind, said) => (globalThis.told.get(kind) || []).forEach((one) => one({ kind, said }));
await import("./keys.js");
const host = anElement("div");
globalThis.registered.itIsDrawn(host);
await settle();

// The New key dialog, opened from the head's own control.
const head = findAll(host, (one) => one.classList.contains("svhead"))[0];
const make = findAll(head, (one) => one.tag === "button").pop();
make.click();
await settle();
// The type probe answers, so a type is chosen and the press is an ordinary one.
tell("keys:an-answer", {
  answer: "the-key-types", newKeyHeading: "New key", theNameField: "Name",
  thePassphraseField: "Passphrase", advancedKeyType: "Advanced", keyType: "Key type",
  cancel: "Cancel", makeItAndCopy: "Create and copy public key",
  typeCount: "1", type0: "ed25519", type0IsTheDefault: "yes", theDefaultMark: "Default",
});
await settle();

const dialog = findAll(host, (one) => one.classList.contains("modal"))[0];
const acts = findAll(dialog, (one) => one.classList.contains("mact"))[0];
const go = acts.children[1];
go.click();
await settle();
const asksAfterTheFirstPress = asks;

// The application refuses the press and says why.
tell("keys:an-answer", { answer: "the-make-was-refused", theActDidNotLand: "This name is too long." });
await settle();
const drawn = findAll(dialog, (one) => one.classList.contains("afailure"));
const theRefusalSaid = drawn.length ? drawn[0].textContent : "";

go.click();
await settle();
console.log(JSON.stringify({
  asksAfterTheFirstPress, theRefusalSaid, asksAfterTheSecondPress: asks,
}));"#,
        ),
    ];

    /// Every name one composed event carries.
    fn the_names_on(event: &Event) -> Vec<String> {
        let text = format!("{event:?}");
        let mut out = Vec::new();
        // **The field names and never the values.** A `BTreeMap`'s debug form
        // writes `"name": Prose("…")`, so a name is a quoted word with a colon
        // and a space directly after it — which no value has.
        for (at, _) in text.match_indices("\": ") {
            let before = &text[..at];
            let Some(opens) = before.rfind('"') else {
                continue;
            };
            let word = &before[opens + 1..];
            if !word.is_empty() && word.chars().all(|letter| letter.is_ascii_alphanumeric()) {
                out.push(word.to_string());
            }
        }
        out
    }

    /// An indexed field is read on the page by its prefix — `row0Name` is read
    /// as `row${at}Name`.
    fn the_prefix_of(name: &str) -> String {
        let mut out = String::new();
        for letter in name.chars() {
            if letter.is_ascii_digit() {
                break;
            }
            out.push(letter);
        }
        if out.len() == name.len() {
            name.to_string()
        } else {
            out
        }
    }

    fn a_refusal(refused: Refused) -> TheAnswer {
        TheAnswer::APointWasRefused {
            refused,
            picked: PathBuf::from("/a/picked/file"),
            the_other_name: Some("the other".into()),
            the_rows_fingerprint: Some("SHA256:one".into()),
            the_picked_fingerprint: Some("SHA256:two".into()),
        }
    }

    fn the_answers() -> Vec<TheAnswer> {
        vec![
            TheAnswer::Nothing,
            TheAnswer::TheTypesTheDialogMayOffer(OfferedSet::StillReading),
            TheAnswer::TheTypesTheDialogMayOffer(OfferedSet::NoGenerator),
            TheAnswer::TheTypesTheDialogMayOffer(OfferedSet::Read(vec![
                KeyType::Ed25519,
                KeyType::Ecdsa,
                KeyType::Rsa,
            ])),
            TheAnswer::TheTypesTheDialogMayOffer(OfferedSet::Read(vec![
                KeyType::Ecdsa,
                KeyType::Rsa,
            ])),
            TheAnswer::AKeyWasMade {
                at: PathBuf::from("/a/path"),
                public_half: "ssh-ed25519 AAAA".into(),
                the_sentence_is_owed: true,
            },
            TheAnswer::TheKeyCouldNotBeMade { reported: None },
            TheAnswer::TheKeyCouldNotBeMade {
                reported: Some("a reason".into()),
            },
            TheAnswer::TheMakeWasRefused(WhyTheMakeIsRefused::TheNameIsTooLong),
            TheAnswer::TheMakeWasRefused(WhyTheMakeIsRefused::ThePassphraseIsTooLong),
            TheAnswer::TheMakeWasRefused(WhyTheMakeIsRefused::NoTypeWasChosen),
            a_refusal(Refused::TheClauseThatFailed {
                clause: Unusable::Unreadable,
                situation: None,
            }),
            a_refusal(Refused::TheClauseThatFailed {
                clause: Unusable::Unreadable,
                situation: Some(WhyItWouldNotRead::ItIsNotAPrivateKeyContainer),
            }),
            a_refusal(Refused::TheClauseThatFailed {
                clause: Unusable::Unreadable,
                situation: Some(WhyItWouldNotRead::ItIsProtectedAndHasNoPublicHalfBeside),
            }),
            a_refusal(Refused::TheClauseThatFailed {
                clause: Unusable::NotASupportedType,
                situation: None,
            }),
            a_refusal(Refused::TheClauseThatFailed {
                clause: Unusable::FilePermissionsTooOpen,
                situation: None,
            }),
            a_refusal(Refused::NotThisKey),
            a_refusal(Refused::ThisKeyIsAlreadyListed {
                as_row: PathBuf::from("/another"),
            }),
            a_refusal(Refused::TwoKeysSmoozyCannotTellApart {
                the_other_row: PathBuf::from("/another"),
            }),
            a_refusal(Refused::TheSystemsSshToolsAreNotInstalled),
            TheAnswer::AnotherKeyIsAlreadyCalledThat {
                name: "a name".into(),
                the_other_fingerprint: Some("SHA256:x".into()),
            },
            TheAnswer::WhatTakingThisKeyAwayCosts {
                row: "a".into(),
                managed: true,
                it_asks: true,
                used_by: vec![AServerUsingIt {
                    called: "one".into(),
                    badge: Some("b1"),
                }],
                the_saved_servers_could_not_be_read: true,
                the_files: vec![PathBuf::from("/a"), PathBuf::from("/a.pub")],
            },
            TheAnswer::WhatTakingThisKeyAwayCosts {
                row: "a".into(),
                managed: false,
                it_asks: false,
                used_by: Vec::new(),
                the_saved_servers_could_not_be_read: false,
                the_files: Vec::new(),
            },
            TheAnswer::ThePublicHalfIsCopied {
                contents: "ssh-ed25519 AAAA".into(),
                the_comparison_ran: true,
            },
            TheAnswer::ThePublicHalfIsCopied {
                contents: "ssh-ed25519 AAAA".into(),
                the_comparison_ran: false,
            },
            TheAnswer::TheCopyIsUnavailable(WhyNot::ThereIsNoPublicHalf),
            TheAnswer::TheCopyIsUnavailable(WhyNot::ItIsADifferentKeysPublicHalf),
            TheAnswer::TakingAwayEnded {
                row: "a".into(),
                the_key_was_not_deleted: true,
                left_behind: vec![PathBuf::from("/a.pub")],
                the_passphrase_was_not_cleared: true,
            },
            TheAnswer::TakingAwayEnded {
                row: "a".into(),
                the_key_was_not_deleted: false,
                left_behind: Vec::new(),
                the_passphrase_was_not_cleared: false,
            },
            TheAnswer::ThePassphraseWasNotCleared { row: "a".into() },
        ]
    }

    /// **X7, over every event this file composes: a name, a fingerprint, a path
    /// or a port never travels inside a sentence.**
    ///
    /// **This is the trap the whole of step 10a is written against**, and it
    /// catches it by walking what a real reading composes rather than by
    /// reading the source: every value handed in below is a string nothing else
    /// on the event holds, and a piece of prose carrying one is the join.
    #[test]
    fn no_value_ever_travels_inside_a_sentence() {
        let marks = [
            "MARKNAME",
            "MARKPATH",
            "MARKSERVER",
            "SHA256:MARKFINGERPRINT",
        ];
        let screen = the_screen_says(&TheKeysScreen {
            mode: TheMode::Picking {
                naming: Some("MARKSERVER".into()),
            },
            the_tools_are_missing: true,
            rows: vec![ARow {
                identifier: "a".into(),
                name: Some("MARKNAME".into()),
                fingerprint: Some("SHA256:MARKFINGERPRINT".into()),
                path: "MARKPATH".into(),
                used_by: vec![AServerUsingIt {
                    called: "MARKSERVER".into(),
                    badge: None,
                }],
                the_saved_servers_could_not_be_read: false,
                a_passphrase_is_remembered: true,
                managed: true,
            }],
        });
        let taking = TheAnswer::WhatTakingThisKeyAwayCosts {
            row: "a".into(),
            managed: true,
            it_asks: true,
            used_by: vec![AServerUsingIt {
                called: "MARKSERVER".into(),
                badge: None,
            }],
            the_saved_servers_could_not_be_read: false,
            the_files: vec![PathBuf::from("MARKPATH")],
        };
        let made = TheAnswer::AKeyWasMade {
            at: PathBuf::from("MARKPATH"),
            public_half: "MARKNAME".into(),
            the_sentence_is_owed: false,
        };
        let same = TheAnswer::AnotherKeyIsAlreadyCalledThat {
            name: "MARKNAME".into(),
            the_other_fingerprint: Some("SHA256:MARKFINGERPRINT".into()),
        };
        let ask = the_ask_says(
            "a",
            &TheAsk {
                server: ServerContext {
                    label: Some("MARKSERVER".into()),
                    username: None,
                    host: None,
                },
                key: HowTheKeyIsNamed::ByItsName(
                    Name::what_the_user_typed("MARKNAME").expect("a name"),
                ),
                the_forgotten_passphrase_line: true,
            },
        );

        for event in [
            screen,
            the_answer_says(&taking),
            the_answer_says(&made),
            the_answer_says(&same),
            ask,
        ] {
            for said in the_prose_on(&event) {
                for mark in marks {
                    assert!(
                        !said.contains(mark),
                        "a value reached the typography point inside {said:?}. \
                         **X7: the point never touches a name, a path, a host or \
                         a port**, and a value joined into a sentence lands \
                         silently inside a name the user will copy"
                    );
                }
            }
        }
    }

    fn the_prose_on(event: &Event) -> Vec<String> {
        let text = format!("{event:?}");
        let mut out = Vec::new();
        for (at, _) in text.match_indices("Prose(") {
            let rest = &text[at..];
            let Some(opens) = rest.find('"') else { continue };
            let Some(closes) = rest[opens + 1..].find('"') else {
                continue;
            };
            out.push(rest[opens + 1..opens + 1 + closes].to_string());
        }
        out
    }

    /// **`Keys` U6's five reasons each have a sentence and no two read alike.**
    ///
    /// `Design/Keys`: *"Two different refusals, and they must not read alike…
    /// a single message would leave the user hunting for a fault in a file that
    /// has none."* **Two of them are one member of the closed set**, which is
    /// what the situation beside it exists to separate, so the count here is
    /// seven and not five.
    #[test]
    fn every_unusable_reason_has_its_own_sentence() {
        let mut said = BTreeSet::new();
        for (reason, situation) in [
            (Unusable::FileMissing, None),
            (Unusable::Unreadable, None),
            (
                Unusable::Unreadable,
                Some(WhyItWouldNotRead::ItIsNotAPrivateKeyContainer),
            ),
            (
                Unusable::Unreadable,
                Some(WhyItWouldNotRead::ItIsProtectedAndHasNoPublicHalfBeside),
            ),
            (Unusable::NotASupportedType, None),
            (Unusable::NoLongerTheSameKey, None),
            (Unusable::FilePermissionsTooOpen, None),
        ] {
            let (first, _) = the_sentences_for(reason, situation);
            assert!(!first.is_empty());
            assert!(
                said.insert(first),
                "two of this screen's refusals read alike: {first:?}"
            );
        }
        assert_eq!(said.len(), 7);
        // **The one clause where Smoozy knows what to do and says it**, and it
        // is the only one with a second sentence.
        let with_a_second: Vec<_> = [
            (Unusable::FileMissing, None),
            (Unusable::Unreadable, None),
            (
                Unusable::Unreadable,
                Some(WhyItWouldNotRead::ItIsProtectedAndHasNoPublicHalfBeside),
            ),
            (Unusable::NotASupportedType, None),
            (Unusable::NoLongerTheSameKey, None),
            (Unusable::FilePermissionsTooOpen, None),
        ]
        .into_iter()
        .filter(|(reason, situation)| the_sentences_for(*reason, *situation).1.is_some())
        .collect();
        assert_eq!(with_a_second.len(), 2);
    }

    /// **`Design/Keys § The list's order`: by name, so two rows sharing a name
    /// sit next to each other.**
    #[test]
    fn the_list_is_ordered_by_name_and_a_shared_name_puts_two_rows_together() {
        let rows = vec![
            a_row(Some("Vast"), "/z", None),
            a_row(Some("RunPod"), "/b", None),
            a_row(Some("Vast"), "/a", None),
            a_row(None, "/aaa", None),
        ];
        let screen = the_screen_from(&rows, None, false, TheMode::Housekeeping);
        let names: Vec<_> = screen
            .rows
            .iter()
            .map(|row| row.name.clone().unwrap_or_else(|| row.path.clone()))
            .collect();
        assert_eq!(names, vec!["/aaa", "RunPod", "Vast", "Vast"]);
    }

    /// **`Keys` U11 and `Design/Keys`: what exists, and nothing where there is
    /// none.**
    ///
    /// The Add Server route can open this screen with every field empty, and
    /// **no value naming that server exists anywhere** — so the line reads
    /// *Choosing a key* and stops. A stand-in phrase would put words on screen
    /// standing for something that has no name yet.
    #[test]
    fn the_picking_line_names_the_server_where_anything_does_and_stops_where_nothing_does() {
        let named = |label: Option<&str>, username: Option<&str>, host: Option<&str>| {
            the_context_names(&DialogContext {
                label: label.map(str::to_string),
                username: username.map(str::to_string),
                host: host.map(str::to_string),
            })
        };
        assert_eq!(named(Some("RunPod 4090"), None, None).as_deref(), Some("RunPod 4090"));
        assert_eq!(
            named(None, Some("root"), Some("1.2.3.4")).as_deref(),
            Some("root@1.2.3.4")
        );
        assert_eq!(named(None, None, None), None);
        assert_eq!(named(Some("  "), None, Some("1.2.3.4")), None);
        assert_eq!(named(None, Some("root"), None), None);
    }

    /// **`Keys` W12's third state is carried and never conflated with the
    /// other.** *No saved server uses this key* is said only on a successful
    /// read returning no match.
    #[test]
    fn a_read_that_did_not_answer_is_never_drawn_as_no_saved_server() {
        let rows = vec![a_row(Some("a"), "/a", Some("SHA256:one"))];
        let unread = the_screen_from(&rows, None, false, TheMode::Housekeeping);
        assert!(unread.rows[0].the_saved_servers_could_not_be_read);
        assert!(unread.rows[0].used_by.is_empty());

        let answered = the_screen_from(&rows, Some(&[]), false, TheMode::Housekeeping);
        assert!(!answered.rows[0].the_saved_servers_could_not_be_read);
        assert!(answered.rows[0].used_by.is_empty());
    }

    /// **A badge outside the drawing's eight draws no badge**, which is the same
    /// refusal the rail makes: a badge means *which server* and nothing else, so
    /// one chosen at drawing time could differ between runs.
    #[test]
    fn a_colour_outside_the_eight_draws_no_badge() {
        assert_eq!(the_badge(Some("b3")), Some("b3"));
        assert_eq!(the_badge(Some("b9")), None);
        assert_eq!(the_badge(Some("purple")), None);
        assert_eq!(the_badge(None), None);
    }

    /// **`Keys` S13, and it is the only clause of that document this crate
    /// keeps that nothing else does.**
    ///
    /// That clause checked one answer against the Method's three escalations and
    /// declined the opposite: *"the one thing that would have escalated is
    /// making the automatic attempt anyway, which puts a passphrase prompt on
    /// screen for a connection the user did not start"*. **There is one route to
    /// the box and it is gated on the user having started the connect**, which
    /// is that refusal in code.
    #[test]
    fn the_box_is_never_raised_on_a_connect_the_user_did_not_start() {
        let row = a_row(Some("a"), "/a", Some("SHA256:one"));
        for the_key_is_protected in [true, false] {
            assert!(
                !smoozy_keys::passphrase::raises_the_ask(
                    &Verdict::Usable,
                    the_key_is_protected,
                    &row,
                    false,
                ),
                "an automatic attempt raised the ask"
            );
        }
        assert!(smoozy_keys::passphrase::raises_the_ask(
            &Verdict::Usable,
            true,
            &row,
            true,
        ));
        // And the one route this crate has to the box goes through that
        // function and through nothing else.
        let working = this_file()
            .split("#[cfg(test)]")
            .next()
            .expect("the working half");
        assert_eq!(
            working.matches("the_ask_for_a_key_with_nothing_held").count(),
            1,
            "there is more than one route to the box"
        );
    }

    /// **The page draws into its own host and never into the pane.**
    ///
    /// `#pane` is the shell's one empty box and anything appended to it draws
    /// beside the shell rather than inside it. The one thing this module puts
    /// outside its host is the passphrase box, which is not this screen's and
    /// goes into `.main`.
    #[test]
    fn the_page_draws_into_its_own_host_and_never_into_the_pane() {
        let page = the_page_module();
        assert!(
            !page.contains("getElementById(\"pane\")") && !page.contains("#pane"),
            "web/keys.js names the pane. **Every screen goes inside its host \
             through the mounting contract**, and anything else appended to the \
             pane draws beside the shell"
        );
        assert!(page.contains("aScreenIsRegistered"));
        assert!(page.contains("querySelector(\".main\")"));
    }

    /// **The page reads the same four kinds this file posts.**
    ///
    /// Nothing holds the two sides together at compile time; this is that check
    /// written out.
    #[test]
    fn the_page_reads_the_same_four_kinds() {
        let page = the_page_module();
        for kind in [
            THE_KEYS_SCREEN,
            A_KEYS_VERDICT,
            AN_ANSWER,
            THE_PASSPHRASE_ASK,
        ] {
            assert!(
                page.contains(&format!("\"{kind}\"")),
                "web/keys.js does not read {kind}"
            );
        }
        // And every command the page calls is one this file exposes.
        for (at, _) in page.match_indices("shell.core.invoke(\"") {
            let rest = &page[at + "shell.core.invoke(\"".len()..];
            let named = &rest[..rest.find('"').expect("the name closes")];
            assert!(
                this_file().contains(&format!("pub fn {named}")),
                "web/keys.js calls `{named}` and this file exposes no such command"
            );
        }
    }

    /// **`Keys` U6a: *not checked* is the absence of a verdict, and the event
    /// carries no reason for it.**
    #[test]
    fn a_key_that_was_not_checked_carries_no_reason_of_its_own() {
        let event = the_verdict_says(&AVerdict {
            row: "a".into(),
            verdict: Verdict::NotChecked,
            why_it_would_not_read: None,
            the_public_half_is_beside_it: true,
        });
        let text = format!("{event:?}");
        assert!(text.contains("not-checked"));
        assert!(
            !text.contains("whatIsWrong"),
            "a key nothing was read about was given a reason"
        );
    }

    // ── Step 10a round 1 ────────────────────────────────────────────────────

    /// **Fix-here item 2(b): a pick that names nothing is still a pick.** It
    /// fails on the command that read a pick with no name as the ordinary list,
    /// so the picking controls never appeared and `keys-picknone` could not be
    /// drawn.
    #[test]
    fn a_pick_that_names_nothing_is_still_a_pick() {
        assert_eq!(the_mode_asked_for(true, None), TheMode::Picking { naming: None });
        assert_eq!(
            the_mode_asked_for(true, Some("  ".into())),
            TheMode::Picking { naming: None }
        );
        assert_eq!(
            the_mode_asked_for(true, Some("RunPod 4090".into())),
            TheMode::Picking {
                naming: Some("RunPod 4090".into())
            }
        );
        assert_eq!(the_mode_asked_for(false, Some("RunPod".into())), TheMode::Housekeeping);
        let page = the_page_module();
        assert!(page.contains("picking: picking !== null"), "the page does not say it is picking");
    }

    /// **Fix-here item 2(c): a pick the user walked away from ends as a cancel.**
    #[test]
    fn a_pick_put_away_ends_as_a_cancel() {
        let page = the_page_module();
        let at = page.find("  itIsPutAway() {").expect("the screen is put away");
        let body = &page[at..at + page[at..].find("\n  },\n").expect("it closes")];
        assert!(body.contains("picking = null;"), "the pick outlives the screen");
        assert!(body.contains("asking.whenItEnds(null)"), "the caller is never told");
    }

    /// **Fix-here item 14: a saved server whose fingerprint will not read makes
    /// the whole read *could not check*** — never a server left out of *Used
    /// by*, which let a delete warn about nothing.
    #[test]
    fn a_server_whose_fingerprint_will_not_read_is_could_not_check() {
        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-keys-surface-unread-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("a scratch directory");
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3"))
            .expect("a throwaway record");
        let row = smoozy_servers::store::ServerRow::saved(
            Identity::issue(IdentityKind::LocalServer).expect("randomness").as_text(),
            smoozy_servers::identity::Host::what_the_user_typed("10.0.0.1").expect("a host"),
            smoozy_servers::identity::Port::STANDARD_SSH,
            smoozy_servers::identity::Username::what_the_user_typed("root").expect("a login"),
            smoozy_servers::identity::KeyReference {
                local_key_identifier: "k".into(),
                fingerprint: String::new(),
            },
        );
        server_rows::add(record.connection(), &row).expect("added");
        assert!(the_saved_servers(record.connection()).is_none());
        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **Fix-here item 30: the picking line is one piece of prose.** It fails on
    /// the two strings the page joined.
    #[test]
    fn the_picking_line_is_one_piece_of_prose() {
        let event = the_screen_says(&TheKeysScreen {
            mode: TheMode::Picking {
                naming: Some("MARKSERVER".into()),
            },
            the_tools_are_missing: false,
            rows: Vec::new(),
        });
        let prose = the_prose_on(&event);
        assert!(prose.iter().any(|said| said.contains("key") && said.ends_with("for")));
        assert!(!the_page_module().contains("said.choosingAKey} ${said.choosingAKeyFor"));
    }

    /// **Sixth notes, note 7: Delete and Remove each say what they do on
    /// hover.** The screen's answer carries both lines, in the plan's words —
    /// **and, since the eighth notes, note 1, why**: where the key sits, in
    /// the `.ssh` folder or outside it (`Design/Keys § The two kinds`).
    #[test]
    fn the_screen_carries_what_delete_and_remove_say_on_hover() {
        let event = the_screen_says(&TheKeysScreen {
            mode: TheMode::Housekeeping,
            the_tools_are_missing: false,
            rows: Vec::new(),
        });
        let text = format!("{event:?}");
        for (name, words) in [
            ("deleteOnHover", "Deletes the key files from your .ssh folder."),
            (
                "removeOnHover",
                "Removes the key from Smoozy. Its files are not deleted.",
            ),
        ] {
            let said = format!("\"{name}\": {:?}", Said::Prose(Prose::new(words)));
            assert!(text.contains(&said), "the screen does not carry {name}");
            // **The plan's words and the drawing's**, so the line above is
            // theirs and not this test's own.
            assert!(DESIGN_KEYS.contains(&format!("*{words}*")), "the plan does not say {name}");
            assert!(THE_DRAWING.contains(&format!("title=\"{words}\"")), "the drawing does not say {name}");
        }
    }

    const DESIGN_KEYS: &str = include_str!("../../../../Preparation/Design/Keys.md");
    const THE_DRAWING: &str = include_str!("../../../../Preparation/Design/Mockup/Smoozy.html");

    /// **And the page draws each on its own button**: the managed row's
    /// *Delete* says what Delete does, the referenced row's *Remove* what
    /// Remove does. Run through Node like the harness above; skipped, and says
    /// so, where no Node is installed.
    #[test]
    fn delete_and_remove_each_say_what_they_do_on_hover() {
        let folder = std::env::temp_dir().join(format!(
            "smoozy-keys-hover-harness-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/keys.js");
        std::fs::copy(&page, folder.join("keys.js")).expect("the page is copied");
        for (name, text) in THE_STAND_INS {
            let text = if name == "harness.js" {
                // The same stand-in document, and this test's own steps after it.
                let (the_document, _) = text.split_once("let asks = 0;").expect("the harness's steps");
                format!("{the_document}{THE_HOVER_STEPS}")
            } else {
                text.to_string()
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node")
            .arg(folder.join("harness.js"))
            .current_dir(&folder)
            .output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Keys screen's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(
            ran.status.success(),
            "the page did not run: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&ran.stdout).trim(),
            r#"[["Delete","MARK-DELETE-HOVER"],["Remove","MARK-REMOVE-HOVER"]]"#
        );
    }

    /// The hover test's steps: one managed row and one referenced row, and what
    /// each row's going control reads and says on hover.
    const THE_HOVER_STEPS: &str = r#"
window.__TAURI__ = { core: { invoke() { return Promise.resolve(null); } } };
const settle = () => new Promise((done) => setImmediate(done));
const tell = (kind, said) => (globalThis.told.get(kind) || []).forEach((one) => one({ kind, said }));
await import("./keys.js");
const host = anElement("div");
globalThis.registered.itIsDrawn(host);
await settle();
tell("keys:the-screen", {
  delete: "Delete", remove: "Remove",
  deleteOnHover: "MARK-DELETE-HOVER", removeOnHover: "MARK-REMOVE-HOVER",
  rowCount: "2",
  row0Identity: "a", row0Name: "one", row0Path: "/a", row0Kind: "managed",
  row1Identity: "b", row1Name: "two", row1Path: "/b", row1Kind: "borrowed",
});
await settle();
const goes = findAll(host, (one) => one.classList.contains("ky-go"));
console.log(JSON.stringify(goes.map((one) => [one.textContent, one.title || ""])));
"#;

    /// **Fix-here item 31: a field name is made once and kept.**
    #[test]
    fn a_field_name_is_made_once() {
        let first = the_name("row", 7, "Name");
        let again = the_name("row", 7, "Name");
        assert!(std::ptr::eq(first, again), "the same name was made twice");
        assert_eq!(first, "row7Name");
    }

    /// **Fix-here item 18: what a delete, a remove or a clear came to is said**
    /// — each file left behind by its full path, as a value beside the words.
    #[test]
    fn what_taking_a_key_away_came_to_is_said() {
        let event = the_answer_says(&TheAnswer::TakingAwayEnded {
            row: "a".into(),
            the_key_was_not_deleted: false,
            left_behind: vec![PathBuf::from("/MARKPATH.pub")],
            the_passphrase_was_not_cleared: true,
        });
        let text = format!("{event:?}");
        assert!(text.contains("leftBehind0"));
        assert!(text.contains("thePassphraseWasNotCleared"));
        assert!(!text.contains("theKeyWasNotDeleted"));
        for said in the_prose_on(&event) {
            assert!(!said.contains("MARKPATH"), "a path travels inside a sentence");
        }
        let page = the_page_module();
        assert!(page.contains("case \"taken-away\""));
        assert!(page.contains("case \"not-cleared\""));
    }

    /// **Fix-here item 20: *Make it* takes no second press until the answer
    /// comes, and wears the underway mark.**
    #[test]
    fn make_it_takes_one_press_until_it_answers() {
        let page = the_page_module();
        let at = page.find("function theKeyIsAskedFor()").expect("the ask");
        let body = &page[at..at + page[at..].find("\n}\n").expect("it closes")];
        assert!(body.contains("box.go.disabled"), "a second press is taken");
        assert!(body.contains("theMarkOnAControl(box.go"), "no underway mark");
        assert!(page.contains("theMakeControlComesBack();"));
    }

    /// **Fix-here item 25: *copied* is said only where the copy reported
    /// success**; otherwise the key is shown with a control that selects it.
    #[test]
    fn copied_is_said_only_where_the_copy_worked() {
        let page = the_page_module();
        let at = page.find("function theCopyIsMade(said)").expect("the copy");
        let body = &page[at..at + page[at..].find("\n}\n").expect("it closes")];
        let checked = body.find("if (!copied)").expect("the copy's answer is not read");
        let claimed = body.find("said.thePublicKeyIsCopied").expect("the claim");
        assert!(checked < claimed, "the claim is made before the copy answers");
        assert!(page.contains("document.execCommand(\"copy\") === true"));
        assert!(page.contains("theKeyToCopyByHand(said)"));
    }

    /// **R4: `Design/Keys § Keyboard shortcuts`.** *"Return chooses the key
    /// where this screen was opened to pick one, and does nothing otherwise.
    /// The delete key deletes or removes, whichever the chosen row offers,
    /// asking exactly as the control does."* The keys press the row's own two
    /// controls, so they cannot ask differently from them.
    #[test]
    fn the_screens_two_keys_press_the_rows_own_controls() {
        let page = the_page_module();
        let from = page.find("host.addEventListener(\"keydown\"").expect("the screen binds its keys");
        let handler = &page[from..from + page[from..].find("\n  });\n").expect("the handler closes")];
        assert!(handler.contains("press.key === \"Enter\" && press.target === row"));
        assert!(
            handler.contains("if (drawn.isPicking && !drawn.choose.hidden) drawn.choose.click();"),
            "Return does something other than choose the key, or chooses one outside picking"
        );
        assert!(handler.contains("press.key === \"Delete\" || press.key === \"Backspace\""));
        assert!(handler.contains("drawn.away.click();"), "the delete key does not ask as the control does");
        assert_eq!(handler.matches(".click()").count(), 2, "a third key was bound");
        assert!(page.contains("root.tabIndex = 0;"), "a row cannot be chosen from the keyboard");
        assert!(page.contains("root.dataset.row = identity;"));
    }

    /// **The live pass on Vast, 2026-10-19, fault 3: Keys listed the one key
    /// twice.** Every draw of the screen starts a discovery pass of its own,
    /// and the screen and its own repost are two draws close together. Each
    /// pass read the rows before its scan, so each found no row for the key
    /// and each wrote one — `Keys` U1a's *"refuses a second row for a
    /// fingerprint a current row already holds"* broken by timing.
    ///
    /// Two passes start together over one key: both have scanned before either
    /// writes, as two draws do, and then each takes the lock and writes. **One
    /// row must be left.** Before the repair each pass compared against the
    /// rows read at its draw, and this left two.
    #[test]
    fn two_discovery_passes_started_together_over_one_key_leave_one_row() {
        use crate::record::LocalRecord;
        use smoozy_keys::listing::Candidate;
        use std::sync::{Arc, Barrier};

        let folder = std::env::temp_dir().join(format!(
            "smoozy-keys-two-passes-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let record = Arc::new(Mutex::new(
            LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("a record"),
        ));
        let the_key = Candidate {
            path: PathBuf::from("/k/.ssh/id_ed25519"),
            passes_the_container_test: true,
            fingerprint: Some(Reading {
                value: Fingerprint::from_the_printed_form("SHA256:one").expect("a fingerprint"),
                read_from: ReadFrom::PrivateFile,
            }),
            name: None,
        };

        let both_have_read = Arc::new(Barrier::new(2));
        let passes: Vec<_> = (0..2)
            .map(|_| {
                let record = Arc::clone(&record);
                let both_have_read = Arc::clone(&both_have_read);
                let found = vec![the_key.clone()];
                std::thread::spawn(move || {
                    // **Both passes have scanned before either writes**, as two
                    // draws close together do.
                    both_have_read.wait();
                    let held = record.lock().expect("the lock");
                    the_discoveries_are_written(held.connection(), &found)
                })
            })
            .collect();
        for pass in passes {
            pass.join().expect("the pass ends");
        }

        let rows = key_rows::all(record.lock().expect("the lock").connection()).expect("read");
        let _ = std::fs::remove_dir_all(&folder);
        assert_eq!(
            rows.len(),
            1,
            "two passes over one key left {} rows: {rows:?}",
            rows.len()
        );
    }
}

//! §1.1's **planted-link carriage** — the one class of value with no other
//! route.
//!
//! **The rule this exists under.** §1.1: *"no user path, package path, option,
//! identifier or other value of user, provider or machine origin is ever
//! interpolated into a command string — in either phase"*. RFC 4254 §6.5
//! defines the `exec` request as a string named `command` and **does not
//! provide an argument vector**, so a value carried in the string is a value a
//! shell parses. Every other value rides one of the other three routes; **a
//! machine-origin or user-origin path that a machine-side program must be
//! pointed at before hand-over** rides this one.
//!
//! **How it works.** The path is written as the **target of a symbolic link**
//! whose own name is a class-2 token, through the subsystem's symlink
//! operation, **so the value crosses as a protocol field and the constant
//! command names the token, never the value.**
//!
//! **The token is fixed in the binary, and that follows from the rule rather
//! than being a convenience.** *"The constant command names the token"* — a
//! command string may hold only a `&'static str`
//! ([`crate::transport::CommandString`]), so a token a command can name is a
//! token the binary already holds. **One site, one token, one carriage live at
//! a time**, which is what [`CarriageSite`] and [`PlantedLink`] make
//! structural.
//!
//! **The operation is revision 3's own** — `SSH_FXP_SYMLINK`, `uint32 id,
//! string linkpath, string targetpath` (`draft-ietf-secsh-filexfer-02` §6.10,
//! read 2026-08-22) — **and OpenSSH's server takes the two strings in the
//! reverse of the specification's order**, by its own `PROTOCOL` §4.1, read
//! 2026-08-22: *"the order of the arguments to the SSH_FXP_SYMLINK method was
//! inadvertently reversed … For correct operation, clients should send
//! SSH_FXP_SYMLINK as follows: uint32 id, string targetpath, string
//! linkpath."*
//!
//! **Two duties ride with the carriage and neither is this module's to
//! discharge.** The owning document **cites the consuming program's own
//! documented treatment of a link**, because following is per-program and never
//! assumed. And where the step's effect must land at the target — `Server
//! Setup`'s hand-over of a chosen root is the case — **the landing is confirmed
//! by that document's own real-write check at the exact target path, never by
//! the command's exit alone.**
//!
//! **Where the link lives.** *"The link is the desktop's own write on the
//! authenticated connection, lives at its token path under a Smoozy-named
//! directory in the login's own writable area — never on the mount it points
//! at — and is removed when the step ends, as recorded staging under §7.8's
//! discipline: an owed cleanup where the removal could not run, never a
//! forgotten one."*

use crate::subsystem::client::Subsystem;
use crate::subsystem::{StatusCode, SubsystemFailure};
use crate::transport::CommandString;
use tokio::io::{AsyncRead, AsyncWrite};

/// The Smoozy-named directory the tokens sit in, **in the login's own writable
/// area**.
///
/// **Relative, and that is a Build decision worth naming.** The file-transfer
/// subsystem resolves a relative path against the login's own starting
/// directory, and the login shell expands `~` to the same place — so the pair
/// below names one directory by two routes without either of them being a path
/// Smoozy composed from a reading. **Nothing here asks the machine where the
/// login's home is**, which would be a machine-origin value with nowhere to
/// ride.
pub const THE_DIRECTORY_ON_THE_SUBSYSTEM: &str = ".smoozy-carriage";

/// One carriage site: a token fixed at build time, named two ways.
///
/// **Both fields are `&'static str` and that is the mechanism.** A site is a
/// place in the code that must point a machine-side program at a path; its
/// token is chosen when that code is written, so a constant command can name
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CarriageSite {
    /// The link's path as the subsystem names it — relative to the login's own
    /// starting directory.
    pub link_path_on_the_subsystem: &'static str,
    /// The same token as a constant command names it. **This is the only form
    /// that ever appears inside a [`CommandString`].**
    pub named_in_a_command_as: &'static str,
}

impl CarriageSite {
    /// **The class-2 test**, asked of the token itself: lower-case letters,
    /// digits, hyphens and the path separator. *"A token so drawn is syntax on
    /// no route."*
    pub fn is_drawn_from_class_twos_alphabet(&self) -> bool {
        self.link_path_on_the_subsystem.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'-'
                || byte == b'.'
                || byte == b'/'
        })
    }
}

/// **F2's second route's site.** The free-space reading's carriage.
pub const THE_FREE_SPACE_SITE: CarriageSite = CarriageSite {
    link_path_on_the_subsystem: ".smoozy-carriage/free-space",
    named_in_a_command_as: "~/.smoozy-carriage/free-space",
};

/// A link that is planted and is owed a removal.
///
/// **It is recorded staging under §7.8's discipline**, which is why this type
/// exists rather than a bare call: a removal that could not run is *"an owed
/// cleanup where the removal could not run, never a forgotten one"*, and
/// [`PlantedLink::the_removal_is_owed`] is what a caller writes down.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlantedLink {
    pub site: CarriageSite,
    /// What it points at. **Held so the caller's own real-write check can name
    /// the exact target path**, which is the second of the carriage's two
    /// duties.
    pub target: Vec<u8>,
    the_removal_is_owed: bool,
}

impl PlantedLink {
    /// **S27's shape.** True where the removal could not run.
    pub fn the_removal_is_owed(&self) -> bool {
        self.the_removal_is_owed
    }
}

/// Why a carriage could not be planted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarriageFailure {
    /// The Smoozy-named directory could not be made or written in.
    TheDirectoryCouldNotBeMade(SubsystemFailure),
    /// The link itself was refused.
    TheLinkCouldNotBePlanted(SubsystemFailure),
}

/// Plants the link, replacing any left by an earlier run.
///
/// **A site carries one token, so a stale link from a run that ended without
/// removing it is met and replaced rather than being an obstruction.** That is
/// the same shape the throwaway areas already use at application start: what
/// stands in for a removal that never ran is the next use, not a name nobody
/// can predict.
pub async fn plant<S: AsyncRead + AsyncWrite + Unpin + Send>(
    subsystem: &mut Subsystem<S>,
    site: CarriageSite,
    target: &[u8],
) -> Result<PlantedLink, CarriageFailure> {
    match subsystem
        .make_directory(THE_DIRECTORY_ON_THE_SUBSYSTEM.as_bytes())
        .await
    {
        Ok(()) => {}
        // A directory that is already there is the ordinary case and not a
        // fault; anything else is.
        Err(SubsystemFailure::TheServerRefused(StatusCode::Failure)) => {}
        Err(failure) => return Err(CarriageFailure::TheDirectoryCouldNotBeMade(failure)),
    }
    // Whatever an earlier run left at this token goes first. Its absence is not
    // a fault.
    let _ = subsystem
        .remove(site.link_path_on_the_subsystem.as_bytes())
        .await;

    subsystem
        .plant_a_link(target, site.link_path_on_the_subsystem.as_bytes())
        .await
        .map_err(CarriageFailure::TheLinkCouldNotBePlanted)?;

    Ok(PlantedLink {
        site,
        target: target.to_vec(),
        the_removal_is_owed: false,
    })
}

/// Removes the link when the step ends. **An owed cleanup where it could not
/// run, never a forgotten one.**
pub async fn take_it_away<S: AsyncRead + AsyncWrite + Unpin + Send>(
    subsystem: &mut Subsystem<S>,
    planted: &mut PlantedLink,
) {
    match subsystem
        .remove(planted.site.link_path_on_the_subsystem.as_bytes())
        .await
    {
        Ok(()) => planted.the_removal_is_owed = false,
        // Already gone is removed.
        Err(failure) if failure.is_the_path_not_being_there() => {
            planted.the_removal_is_owed = false
        }
        Err(_) => planted.the_removal_is_owed = true,
    }
}

/// The one thing a caller may put in a command string about a carriage: the
/// token, as a constant.
///
/// **This exists so that the pairing is one call rather than two constants a
/// site keeps in step by hand.** A site whose command names something other
/// than its own token is a site pointing at nothing, and a reader can see the
/// pairing here.
pub const fn a_command_naming(command: &'static str) -> CommandString {
    CommandString::constant(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_site_names_one_token_two_ways_and_both_are_fixed_in_the_binary() {
        // The mechanism: a command string takes only a `&'static str`, so a
        // token a command can name is one the binary already holds. This test
        // is what states that the two forms are of one token.
        let site = THE_FREE_SPACE_SITE;
        assert!(site
            .named_in_a_command_as
            .ends_with(site.link_path_on_the_subsystem));
        assert!(site.named_in_a_command_as.starts_with("~/"));
    }

    #[test]
    fn the_token_is_drawn_from_class_twos_alphabet() {
        // §1.1 class 2: lower-case letters, digits, hyphens and the path
        // separator, under an installation-owned prefix.
        assert!(THE_FREE_SPACE_SITE.is_drawn_from_class_twos_alphabet());
        let a_bad_one = CarriageSite {
            link_path_on_the_subsystem: ".smoozy-carriage/Free Space",
            named_in_a_command_as: "~/.smoozy-carriage/Free Space",
        };
        assert!(!a_bad_one.is_drawn_from_class_twos_alphabet());
    }

    #[test]
    fn the_link_sits_under_the_smoozy_named_directory_and_never_on_the_mount() {
        // *"never on the mount it points at"* — the token path is fixed and is
        // under the login's own writable area whatever the target is.
        assert!(THE_FREE_SPACE_SITE
            .link_path_on_the_subsystem
            .starts_with(THE_DIRECTORY_ON_THE_SUBSYSTEM));
    }

    #[test]
    fn a_planted_link_holds_its_target_so_the_landing_can_be_checked_at_it() {
        // The carriage's second duty: where the step's effect must land at the
        // target, the landing is confirmed by a real-write check at the exact
        // target path, never by the command's exit alone.
        let planted = PlantedLink {
            site: THE_FREE_SPACE_SITE,
            target: b"/var/lib/smoozy/jobs".to_vec(),
            the_removal_is_owed: false,
        };
        assert_eq!(planted.target, b"/var/lib/smoozy/jobs");
        assert!(!planted.the_removal_is_owed());
    }
}

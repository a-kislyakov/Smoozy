//! The one command the command channel carries, as a value.
//!
//! **Nothing here runs a command.** The channel and every act on it stay next
//! door in `smoozy-server-protocol`, which re-exports this type at
//! `transport::CommandString`, the path it has always been reached at. It sits
//! on this side of the split because the setup steps a machine performs for
//! itself name their constant commands with it, and that code must not reach
//! the connecting half to do so.

/// One command string, and the type is the whole of §1.1's prohibition.
///
/// **§1.1: RFC 4254 §6.5 defines the `exec` request as a string named
/// `command`; it does not provide an argument vector. So no user path, package
/// path, option, identifier or other value of user, provider or machine origin
/// is ever interpolated into a command string — in either phase — and the two
/// phases differ only in what the constant string is.**
///
/// **That is kept by construction here and not by care at each call site.** The
/// only constructor takes a `&'static str`, so a value read at run time — a
/// host, a username, a path the machine named, anything a user typed — cannot
/// reach it: a `String` built at run time is not `'static`, and `Box::leak` is
/// the only way round, which is a deliberate act a reader can see.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandString(&'static str);

impl CommandString {
    /// The one way to make one. **The argument is a constant in Smoozy's own
    /// binary**, which is what §1.1 means by *"a constant, installation-owned
    /// command"*.
    pub const fn constant(text: &'static str) -> CommandString {
        CommandString(text)
    }

    pub fn as_str(self) -> &'static str {
        self.0
    }
}

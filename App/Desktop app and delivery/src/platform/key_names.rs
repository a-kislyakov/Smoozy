//! The two name readings used only by `Keys` G11 for its default key name.
//! Moved unchanged from the withdrawn paid-identity adapter; the hardware
//! identity readings are not part of this service.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheTwoNames {
    /// **I2**, and never `gethostname(3)` on macOS: observed, with `HostName`
    /// unset, that call returned a **fourth** string that is neither of the two
    /// names actually set.
    pub computer_name: Option<String>,
    /// **I3**, and never `getlogin()` or `logname`: observed on the Mac, both
    /// returned the literal `root` while the real account name at effective uid
    /// 501 was something else.
    pub login_account_name: Option<String>,
}

/// Both readings, taken at the moment they are needed.
pub fn the_two_names() -> TheTwoNames {
    TheTwoNames {
        computer_name: the_computer_name(),
        login_account_name: the_login_account_name(),
    }
}

/// **I2, macOS: `SCDynamicStoreCopyComputerName`**, which Apple documents as
/// returning *"the current computer name"*.
///
/// **The framework is linked and no package is added.** SystemConfiguration is
/// part of the system and the declaration below is the whole of what reaching it
/// costs — there is no new Cargo dependency, no lock change and no pin.
///
/// **The result is a `CFStringRef` at +1**, toll-free bridged to `NSString`, so
/// [`objc2::rc::Retained`] owns it and releases it. Nothing here copies the
/// bytes anywhere else and nothing composes a path from them (I4).
#[cfg(target_os = "macos")]
fn the_computer_name() -> Option<String> {
    use objc2::rc::Retained;
    use objc2_foundation::NSString;

    #[link(name = "SystemConfiguration", kind = "framework")]
    extern "C" {
        fn SCDynamicStoreCopyComputerName(
            store: *const core::ffi::c_void,
            name_encoding: *mut u32,
        ) -> *const NSString;
    }

    // SAFETY: the store argument is documented as optional and null asks the
    // function to make one of its own; the encoding argument is optional and
    // null asks for nothing back. The return is a +1 `CFStringRef`, which
    // `Retained::from_raw` takes ownership of.
    let raw = unsafe { SCDynamicStoreCopyComputerName(std::ptr::null(), std::ptr::null_mut()) };
    let name: Retained<NSString> = unsafe { Retained::from_raw(raw as *mut NSString) }?;
    let text = name.to_string();
    (!text.is_empty()).then_some(text)
}

/// **I2, Linux: the static hostname**, which systemd's own page describes as
/// *"the "static" hostname configured in /etc/hostname"* and which *"is not
/// always in sync with the current hostname as returned by the gethostname(3)
/// system call"*, **falling back to the kernel's current hostname only where the
/// static one is *"the empty string"***.
///
/// **Documentation alone.** No Linux desktop computer exists on this project and
/// none is coming (`Keys` §13 item 2, permanently unresolved), so this has never
/// been run.
#[cfg(target_os = "linux")]
fn the_computer_name() -> Option<String> {
    let static_name = std::fs::read_to_string("/etc/hostname")
        .ok()
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty());
    if static_name.is_some() {
        return static_name;
    }
    // The kernel's current hostname, which the page names as the fallback and
    // as nothing else.
    let mut buffer = vec![0u8; 256];
    // SAFETY: the buffer is owned here and its length is passed with it.
    let read = unsafe { libc::gethostname(buffer.as_mut_ptr() as *mut libc::c_char, buffer.len()) };
    if read != 0 {
        return None;
    }
    let end = buffer.iter().position(|byte| *byte == 0).unwrap_or(0);
    let text = String::from_utf8_lossy(&buffer[..end]).trim().to_string();
    (!text.is_empty()).then_some(text)
}

/// **I2, Windows: not built, and the absence is deliberate rather than
/// forgotten.**
///
/// The plan names `GetComputerNameExW` with `ComputerNameDnsHostname` and rules
/// the domain and fully-qualified formats out. **Reaching it wants a feature of
/// the `windows-sys` package this crate's Windows tree does not carry**, which
/// is a manifest change, and **no Windows computer exists here to run it on** —
/// so what would land is an unrun call behind an unchecked feature flag.
///
/// **What it costs, stated:** on Windows the make-key dialog's name field starts
/// empty and `Keys` N1's bit is false for every key entered there, so two
/// ordinary keys made on one Windows computer draw the duplicate-name statement
/// the spec would withhold. **Nothing else turns on either name** (I4, I5).
#[cfg(target_os = "windows")]
fn the_computer_name() -> Option<String> {
    None
}

/// **I3, macOS and Linux: the password-database entry for the *effective* user
/// id.**
///
/// **Never `getlogin()` or `logname`** — observed on the Mac, both returned the
/// literal `root` while the real account name at effective uid 501 was something
/// else — and **never `LOGNAME`**, which the Linux man-pages project's own page
/// says the user can set arbitrarily.
#[cfg(unix)]
fn the_login_account_name() -> Option<String> {
    // SAFETY: `geteuid` cannot fail. `getpwuid` returns a pointer into a static
    // area owned by the library, which is read here and copied out at once;
    // nothing holds it afterwards.
    let entry = unsafe { libc::getpwuid(libc::geteuid()) };
    if entry.is_null() {
        return None;
    }
    let name = unsafe { (*entry).pw_name };
    if name.is_null() {
        return None;
    }
    let text = unsafe { std::ffi::CStr::from_ptr(name) }
        .to_string_lossy()
        .to_string();
    (!text.is_empty()).then_some(text)
}

/// **I3, Windows: not built**, for the same reason as I2's Windows arm above.
/// The plan names `GetUserName`.
#[cfg(not(unix))]
fn the_login_account_name() -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_names_read_on_this_computer() {
        let names = the_two_names();
        if cfg!(target_os = "macos") {
            let computer = names.computer_name.expect("macOS answers a computer name");
            assert!(!computer.trim().is_empty());
            let account = names.login_account_name.expect("the password database answers an account name");
            assert!(!account.trim().is_empty());
            assert_ne!(account, "root");
        }
    }
}


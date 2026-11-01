//! The fixed project-address opener — `Desktop app and delivery` K25.
//! A caller names an address and cannot supply one. Publication supplies the
//! source and donation addresses; an unset address opens nothing.

/// The source repository. Publication changes this one value if its name moves.
pub const SOURCE_CODE_ADDRESS: &str = "https://github.com/a-kislyakov/Smoozy";

/// **The two addresses, and there is no third.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichAddress {
    /// The project's source.
    Source,
    /// The project's donation page, only once chosen.
    Donate,
}

/// **What the service says, and it never says the page opened** (K25).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opened {
    /// The system took the request.
    ItWasAsked,
    /// **This build is fixed to no such address, or the system would not take
    /// the request** — no cause is modelled behind it.
    ItCouldNotBeOpened,
}

fn fixed_address(which: WhichAddress) -> Option<&'static str> {
    match which {
        WhichAddress::Source => Some(SOURCE_CODE_ADDRESS),
        WhichAddress::Donate => None,
    }
}

/// **Open one of the two** in the user's own browser.
pub fn open(which: WhichAddress) -> Opened {
    let address = fixed_address(which);
    match address {
        Some(address) if the_address_may_be_opened(address) => per_system::open(address),
        _ => Opened::ItCouldNotBeOpened,
    }
}

/// **Only an exact `https` address goes to the browser**. Nothing that would be read as a file, a program or a second
/// scheme is ever handed over, whatever the build was fixed to.
fn the_address_may_be_opened(address: &str) -> bool {
    address.starts_with("https://")
        && address.len() > "https://".len()
        && !address.chars().any(|c| c.is_whitespace() || c.is_control() || c == '"')
}

// ── macOS ───────────────────────────────────────────────────────────────────

#[cfg(target_os = "macos")]
mod per_system {
    use super::Opened;
    use objc2_app_kit::NSWorkspace;
    use objc2_foundation::{NSString, NSURL};

    /// **`NSWorkspace openURL:`**, which hands the address to the user's own
    /// default browser and answers whether it could.
    pub(super) fn open(address: &str) -> Opened {
        let Some(url) = NSURL::URLWithString(&NSString::from_str(address)) else {
            return Opened::ItCouldNotBeOpened;
        };
        if NSWorkspace::sharedWorkspace().openURL(&url) {
            Opened::ItWasAsked
        } else {
            Opened::ItCouldNotBeOpened
        }
    }
}

// ── Windows ─────────────────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
mod per_system {
    use super::Opened;
    use windows_sys::Win32::UI::Shell::ShellExecuteW;
    use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

    /// **`ShellExecute` on the address**, with its documented greater-than-32
    /// success contract — the same call the folder-reveal opens a folder with.
    pub(super) fn open(address: &str) -> Opened {
        let wide: Vec<u16> = address.encode_utf16().chain(Some(0)).collect();
        // Safety: the wide string outlives the call, and the window handle is
        // deliberately null — the browser is not a child of Smoozy's window.
        let answered = unsafe {
            ShellExecuteW(
                std::ptr::null_mut(),
                std::ptr::null(),
                wide.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                SW_SHOWNORMAL,
            )
        };
        if answered as isize > 32 {
            Opened::ItWasAsked
        } else {
            Opened::ItCouldNotBeOpened
        }
    }
}

// ── Linux ───────────────────────────────────────────────────────────────────

#[cfg(target_os = "linux")]
mod per_system {
    use super::Opened;

    /// **`xdg-open`**, which the folder-reveal's folder-open already relies on
    /// and E4's matrix declares as a package dependency, **or the call fails**
    /// — a computer without it answers rather than going quiet.
    pub(super) fn open(address: &str) -> Opened {
        match std::process::Command::new("xdg-open").arg(address).status() {
            Ok(status) if status.success() => Opened::ItWasAsked,
            _ => Opened::ItCouldNotBeOpened,
        }
    }
}

// ── Anywhere else ───────────────────────────────────────────────────────────

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
mod per_system {
    use super::Opened;

    pub(super) fn open(_address: &str) -> Opened {
        Opened::ItCouldNotBeOpened
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wave4_source_is_fixed_and_donation_stays_unset() {
        assert_eq!(fixed_address(WhichAddress::Source), Some("https://github.com/a-kislyakov/Smoozy"));
        assert!(fixed_address(WhichAddress::Donate).is_none());
        assert_eq!(open(WhichAddress::Donate), Opened::ItCouldNotBeOpened);
    }

    /// **Only an exact `https` address is handed over**, whatever a build was
    /// fixed to.
    #[test]
    fn only_an_https_address_may_be_opened() {
        assert!(the_address_may_be_opened("https://project.example/source"));
        for refused in [
            "http://project.example/source",
            "file:///Applications/Calculator.app",
            "https://",
            "https://project.example/a b",
            "https://project.example/\"x",
            "javascript:alert(1)",
            "",
        ] {
            assert!(!the_address_may_be_opened(refused), "{refused} would be opened");
        }
    }

    /// **K25: two addresses and no third, and no caller hands over an
    /// address.** The one public act takes [`WhichAddress`] and nothing else.
    #[test]
    fn the_service_opens_two_named_addresses_and_takes_no_address() {
        let (working, _) = include_str!("the_address_opener.rs")
            .split_once("#[cfg(test)]\nmod tests")
            .expect("the tests are the last thing in this file");
        let acts: Vec<&str> = working
            .lines()
            .filter(|line| line.starts_with("pub fn "))
            .collect();
        assert_eq!(acts, vec!["pub fn open(which: WhichAddress) -> Opened {"]);
    }
}

//! The fixed project-address opener — `Desktop app and delivery` K25.
//! A caller names an address and cannot supply one. Publication supplies the
//! source and donation addresses; an unset address opens nothing. **From
//! 2026-11-09 a third name** (Z33, K25's narrowing): the release page of the
//! version the update box offers, built from E10a's fixed repository and a
//! version that reads as whole numbers and full stops. **And from the same
//! day a fourth** (K25's second narrowing, `Reporting a problem` V10): the
//! report address's mail link, one fixed `mailto:` value with nothing composed
//! into it, and the only address of its kind this service opens. **From
//! 2026-11-13 a fifth** (K25's third narrowing, the website ideas' picks):
//! Help — the website's Help page, or one of its answers from a closed list,
//! each a fixed address on the project's site.

/// The source repository. Publication changes this one value if its name moves.
pub const SOURCE_CODE_ADDRESS: &str = "https://github.com/a-kislyakov/Smoozy";

/// **The releases repository** — E10a's, the one the update file is read
/// from. A release's page is `/releases/tag/v` and its version below it.
pub const THE_RELEASES_ADDRESS: &str = "https://github.com/a-kislyakov/Smoozy-releases";

/// **The website's Help page** (K25's third narrowing). An answer is this
/// address with its fixed name after `#`.
pub const THE_HELP_PAGE: &str = "https://smoozy.app/help";

/// **The answers the app opens, and no others** — the page itself, and the
/// three things the user fixes outside Smoozy (`Design/Whole app § Help
/// links`). **The site keeps every name here**: an answer the app links to is
/// never renamed there, which
/// `tests::every_help_answer_the_app_opens_is_on_the_site` holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheHelpAnswer {
    /// The Help page, from the Smoozy menu.
    ThePage,
    /// *Could not connect*.
    CouldNotConnect,
    /// *The NVIDIA driver is too old, or has no OptiX*.
    TheNvidiaDriver,
    /// *This step needs the server's login password*.
    TheLoginPassword,
}

impl TheHelpAnswer {
    pub const EVERY: [TheHelpAnswer; 4] = [
        TheHelpAnswer::ThePage,
        TheHelpAnswer::CouldNotConnect,
        TheHelpAnswer::TheNvidiaDriver,
        TheHelpAnswer::TheLoginPassword,
    ];

    /// **The answer's fixed name** — its id on the site, and the value the
    /// page hands back when its link is pressed. The page itself is `page`.
    pub const fn name(self) -> &'static str {
        match self {
            TheHelpAnswer::ThePage => "page",
            TheHelpAnswer::CouldNotConnect => "could-not-connect",
            TheHelpAnswer::TheNvidiaDriver => "nvidia-driver",
            TheHelpAnswer::TheLoginPassword => "login-password",
        }
    }

    /// **The answer a name names, where it names one on the list**, and
    /// nothing otherwise: the page names an answer and never supplies one.
    pub fn read(name: &str) -> Option<TheHelpAnswer> {
        TheHelpAnswer::EVERY.into_iter().find(|answer| answer.name() == name)
    }
}

/// **The five names, and there is no sixth.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhichAddress {
    /// The project's source.
    Source,
    /// The project's donation page, only once chosen.
    Donate,
    /// **A release's own page** — the update box's *What's new* (Z33).
    TheReleasePage(TheReleasedVersion),
    /// **The report address's mail link** — the saved report box's address
    /// (`Reporting a problem` V10): a new mail to it, nothing else filled in.
    TheReportAddress,
    /// **Help** — the website's Help page or one of its listed answers (K25's
    /// third narrowing).
    Help(TheHelpAnswer),
}

/// **A version that reads as whole numbers and full stops**, and nothing
/// else, so no address can be made of anything but the repository's own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheReleasedVersion(String);

impl TheReleasedVersion {
    /// The version, where it is one.
    pub fn read(version: &str) -> Option<TheReleasedVersion> {
        let parts: Vec<&str> = version.split('.').collect();
        let whole = parts.len() >= 2 && parts.iter().all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()));
        whole.then(|| TheReleasedVersion(version.to_string()))
    }
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

fn fixed_address(which: WhichAddress) -> Option<String> {
    match which {
        WhichAddress::Source => Some(SOURCE_CODE_ADDRESS.to_string()),
        WhichAddress::Donate => None,
        WhichAddress::TheReleasePage(TheReleasedVersion(version)) => {
            Some(format!("{THE_RELEASES_ADDRESS}/releases/tag/v{version}"))
        }
        WhichAddress::TheReportAddress => Some(the_report_mail_link()),
        WhichAddress::Help(TheHelpAnswer::ThePage) => Some(THE_HELP_PAGE.to_string()),
        WhichAddress::Help(answer) => Some(format!("{THE_HELP_PAGE}#{}", answer.name())),
    }
}

/// **The report address's mail link**: `mailto:` and the box's own address,
/// and nothing composed into it — no subject, no body, no attachment
/// (`Reporting a problem` V10).
fn the_report_mail_link() -> String {
    format!("mailto:{}", crate::surface::reporting::THE_ADDRESS)
}

/// **Open one of the five** in the user's own browser — or, for the mail
/// link, in whatever the computer uses for mail.
pub fn open(which: WhichAddress) -> Opened {
    let address = fixed_address(which);
    match address {
        Some(address) if the_address_may_be_opened(&address) => per_system::open(&address),
        _ => Opened::ItCouldNotBeOpened,
    }
}

/// **Only an exact `https` address goes to the browser**, and **the one fixed
/// mail link** (K25's second narrowing) and no other `mailto:` value. Nothing
/// that would be read as a file, a program or a second scheme is ever handed
/// over, whatever the build was fixed to.
fn the_address_may_be_opened(address: &str) -> bool {
    if address == the_report_mail_link() {
        return true;
    }
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
        assert_eq!(fixed_address(WhichAddress::Source).as_deref(), Some("https://github.com/a-kislyakov/Smoozy"));
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

    /// **Z33's third name: a release's own page, of a version and nothing
    /// else.** Anything that is not whole numbers and full stops makes no
    /// address at all.
    #[test]
    fn twenty_eighth_4_whats_new_is_the_releases_own_page() {
        let version = TheReleasedVersion::read("0.3.2").expect("a version");
        assert_eq!(
            fixed_address(WhichAddress::TheReleasePage(version)).as_deref(),
            Some("https://github.com/a-kislyakov/Smoozy-releases/releases/tag/v0.3.2")
        );
        assert_eq!(
            THE_RELEASES_ADDRESS,
            crate::updating::checking::WHERE_THE_UPDATE_FILE_IS_READ_FROM
                .split_once("/releases/latest/")
                .map(|(repository, _)| repository)
                .unwrap(),
            "the release page is not on the repository the update file is read from"
        );
        for refused in ["", "0", "v0.3.2", "0.3.2/../../x", "0.3.2?a=b", "0..2", "0.3.2 ", "0.3.2-beta", "１.２"] {
            assert_eq!(TheReleasedVersion::read(refused), None, "{refused:?} made an address");
        }
    }

    /// **K25's second narrowing, `Reporting a problem` V10: the report
    /// address is one fixed mail link**, built from the box's own address with
    /// nothing composed into it — and it is the only `mailto:` value the
    /// service hands over.
    #[test]
    fn twenty_eighth_6_the_report_address_is_one_fixed_mail_link() {
        assert_eq!(
            fixed_address(WhichAddress::TheReportAddress).as_deref(),
            Some("mailto:hello@smoozy.app")
        );
        assert_eq!(
            fixed_address(WhichAddress::TheReportAddress),
            Some(format!("mailto:{}", crate::surface::reporting::THE_ADDRESS)),
            "the mail link is not the address the box draws"
        );
        assert!(the_address_may_be_opened("mailto:hello@smoozy.app"));
        for refused in [
            "mailto:someone@else.example",
            "mailto:hello@smoozy.app?subject=Smoozy",
            "mailto:hello@smoozy.app?attach=/Users/artist/secret.txt",
            "mailto:hello@smoozy.app,other@else.example",
            "mailto:hello@smoozy.app ",
            "MAILTO:hello@smoozy.app",
            "mailto:",
        ] {
            assert!(!the_address_may_be_opened(refused), "{refused} would be opened");
        }
    }

    /// **K25's third narrowing: Help, the website's Help page or one of its
    /// answers from a closed list** (2026-11-13, the website ideas' picks).
    /// Each is the page's fixed address, with the answer's fixed name after
    /// `#`; a name not on the list makes no address.
    #[test]
    fn help_is_the_page_or_a_listed_answer_and_nothing_else() {
        let opened = |answer| fixed_address(WhichAddress::Help(answer));
        assert_eq!(opened(TheHelpAnswer::ThePage).as_deref(), Some("https://smoozy.app/help"));
        assert_eq!(
            opened(TheHelpAnswer::CouldNotConnect).as_deref(),
            Some("https://smoozy.app/help#could-not-connect")
        );
        assert_eq!(
            opened(TheHelpAnswer::TheNvidiaDriver).as_deref(),
            Some("https://smoozy.app/help#nvidia-driver")
        );
        assert_eq!(
            opened(TheHelpAnswer::TheLoginPassword).as_deref(),
            Some("https://smoozy.app/help#login-password")
        );
        for answer in TheHelpAnswer::EVERY {
            assert_eq!(TheHelpAnswer::read(answer.name()), Some(answer));
            assert!(the_address_may_be_opened(&opened(answer).unwrap()), "{answer:?}");
        }
        for refused in ["", "help", "https://smoozy.app/help", "could-not-connect ", "#could-not-connect", "turn-off", "../x"] {
            assert_eq!(TheHelpAnswer::read(refused), None, "{refused:?} named an answer");
        }
    }

    /// **The site keeps every name the app opens** (K25's third narrowing):
    /// each listed answer is an answer on the site's Help page, by its id.
    #[test]
    fn every_help_answer_the_app_opens_is_on_the_site() {
        let page = include_str!("../../../Website/Site/help.html");
        for answer in TheHelpAnswer::EVERY {
            if answer == TheHelpAnswer::ThePage {
                continue;
            }
            let id = format!("<details id=\"{}\">", answer.name());
            assert!(page.contains(&id), "the Help page has no answer {:?}", answer.name());
        }
    }

    /// **Whether this system's opener takes a mail link is checked, not
    /// assumed** (Part B, note 6): the Mac is asked which program would open
    /// the fixed link — the question alone, so nothing opens on the screen.
    #[cfg(target_os = "macos")]
    #[test]
    fn twenty_eighth_6_this_mac_names_a_program_for_the_mail_link() {
        use objc2_app_kit::NSWorkspace;
        use objc2_foundation::{NSString, NSURL};
        let link = format!("mailto:{}", crate::surface::reporting::THE_ADDRESS);
        let url = NSURL::URLWithString(&NSString::from_str(&link)).expect("the mail link is an address the Mac reads");
        let program = NSWorkspace::sharedWorkspace().URLForApplicationToOpenURL(&url);
        assert!(program.is_some(), "this Mac names no program for {link}");
    }

    /// **K25: named addresses and no caller hands over an address.** The one
    /// public act takes [`WhichAddress`] and nothing else.
    #[test]
    fn the_service_opens_named_addresses_and_takes_no_address() {
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

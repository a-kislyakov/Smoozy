//! `Reporting a problem` J8 and `Reporting a problem` J9: a dying process's last diary line.
//!
//! Preparation happens while the process is healthy. The handler reads only
//! immutable prepared bytes and lock-free atomics, builds one line on its
//! stack, and asks the diary service for a raw append and device flush. It
//! takes no Rust lock, allocates nothing and invokes no buffered writer.
//! The ordinary writer publishes its current segment number before appending,
//! so the last line is not tied to a segment that rotation may have evicted.
//!
//! Documented: Apple's sigaction(2), fcntl(2), applicationWillTerminate;
//! Linux man-pages signal(7), signal-safety(7); Microsoft's HandlerRoutine,
//! WM_QUERYENDSESSION, WM_ENDSESSION, WM_CLOSE and FlushFileBuffers. Exact
//! sources and the limits of the desk observation are in the R5 account.

use smoozy_state::identities::Identity;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicPtr, AtomicU64, Ordering};

pub(crate) struct Prepared {
    area: PathBuf,
    run: Vec<u8>,
    segment: AtomicU64,
    seen: AtomicU64,
    #[cfg(unix)]
    directory: std::fs::File,
    #[cfg(windows)]
    prefix: Vec<u16>,
}

// Installed once per process, never freed while a handler could be using it.
// One allocation for the entire run, not one allocation per entry or segment.
// Each of the 64 once-bits can append at most 256 bytes. Ordinary eviction
// accounts for this allowance before installing handlers and after every write.
pub(crate) const LAST_LINE_RESERVE: u64 = 64 * 256;
static PREPARED: AtomicPtr<Prepared> = AtomicPtr::new(std::ptr::null_mut());

/// The diary service is the sole caller; the run's session is already durable.
pub(crate) fn install(area: &Path, run: &Identity, segment: u64) -> Result<(), String> {
    let prepared = Box::new(Prepared {
        area: area.to_path_buf(),
        run: format!("{run:?}").into_bytes(),
        segment: AtomicU64::new(segment),
        seen: AtomicU64::new(0),
        #[cfg(unix)]
        directory: std::fs::File::open(area).map_err(|e| e.to_string())?,
        #[cfg(windows)]
        prefix: {
            use std::os::windows::ffi::OsStrExt;
            area.as_os_str()
                .encode_wide()
                .chain(std::iter::once(b'\\' as u16))
                .collect()
        },
    });
    let raw = Box::into_raw(prepared);
    if PREPARED
        .compare_exchange(
            std::ptr::null_mut(),
            raw,
            Ordering::Release,
            Ordering::Relaxed,
        )
        .is_err()
    {
        unsafe {
            drop(Box::from_raw(raw));
        }
        return Err("the last-line handlers are already installed".into());
    }
    Ok(())
}

pub(crate) fn handlers_are_installed() -> Result<(), String> {
    platform::install()
}

/// Called only by the diary's ordinary writer, never by a handler.
pub(crate) fn the_segment_is(area: &Path, segment: u64) {
    let raw = PREPARED.load(Ordering::Acquire);
    if let Some(prepared) = unsafe { raw.as_ref() } {
        if prepared.area == area {
            prepared.segment.store(segment, Ordering::Release);
        }
    }
}

pub(crate) fn reserve_at(area: &Path) -> u64 {
    let raw = PREPARED.load(Ordering::Acquire);
    if unsafe { raw.as_ref() }.is_some_and(|p| p.area == area) {
        LAST_LINE_RESERVE
    } else {
        0
    }
}

// Fixed stack storage; the only varying text is a decimal clock or segment.
struct Bytes {
    bytes: [u8; 256],
    len: usize,
}
impl Bytes {
    fn new() -> Self {
        Self {
            bytes: [0; 256],
            len: 0,
        }
    }
    fn add(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.bytes[self.len] = byte;
            self.len += 1;
        }
    }
    fn number(&mut self, mut value: u64, minimum: usize) {
        let mut digits = [b'0'; 20];
        let mut at = digits.len();
        loop {
            at -= 1;
            digits[at] = b'0' + (value % 10) as u8;
            value /= 10;
            if value == 0 {
                break;
            }
        }
        at = at.min(20 - minimum);
        self.add(&digits[at..]);
    }
    fn slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

/// One per installed handler, even when a system repeats the same message.
/// The ending's name is a declared constant, never mined from a crash dump.
fn last(index: u32, ending: &[u8]) {
    let raw = PREPARED.load(Ordering::Acquire);
    let Some(prepared) = (unsafe { raw.as_ref() }) else {
        return;
    };
    if prepared.seen.fetch_or(1u64 << index, Ordering::AcqRel) & (1u64 << index) != 0 {
        return;
    }
    let mut line = Bytes::new();
    line.add(&prepared.run);
    line.add(b"\tLast\t");
    line.number(platform::seconds(), 1);
    line.add(b"\tThisComputer\t\t");
    line.add(ending);
    line.add(b"\n");
    let mut name = Bytes::new();
    name.add(b"diary-");
    name.number(
        prepared.segment.load(Ordering::Acquire).saturating_add(1),
        12,
    );
    // The terminating NUL is outside the append's bytes, for openat/CreateFile.
    name.bytes[name.len] = 0;
    crate::diary_service::the_last_line_is_appended(
        prepared,
        &name.bytes[..=name.len],
        line.slice(),
    );
}

#[cfg(unix)]
mod platform {
    use super::*;
    // Default-terminating standard signals. Ignored signals remain ignored;
    // existing handlers (including Rust's fault handlers) are called unchanged.
    // Observing a signal must not change how the application handles it.
    fn endings() -> Vec<(i32, &'static [u8])> {
        let mut all: Vec<(i32, &'static [u8])> = vec![
            (libc::SIGHUP, b"SIGHUP"),
            (libc::SIGINT, b"SIGINT"),
            (libc::SIGQUIT, b"SIGQUIT"),
            (libc::SIGILL, b"SIGILL"),
            (libc::SIGTRAP, b"SIGTRAP"),
            (libc::SIGABRT, b"SIGABRT"),
            (libc::SIGFPE, b"SIGFPE"),
            (libc::SIGBUS, b"SIGBUS"),
            (libc::SIGSEGV, b"SIGSEGV"),
            (libc::SIGSYS, b"SIGSYS"),
            (libc::SIGPIPE, b"SIGPIPE"),
            (libc::SIGALRM, b"SIGALRM"),
            (libc::SIGTERM, b"SIGTERM"),
            (libc::SIGXCPU, b"SIGXCPU"),
            (libc::SIGXFSZ, b"SIGXFSZ"),
            (libc::SIGVTALRM, b"SIGVTALRM"),
            (libc::SIGPROF, b"SIGPROF"),
            (libc::SIGUSR1, b"SIGUSR1"),
            (libc::SIGUSR2, b"SIGUSR2"),
        ];
        #[cfg(target_os = "macos")]
        all.push((libc::SIGEMT, b"SIGEMT"));
        #[cfg(target_os = "linux")]
        {
            all.extend([
                (libc::SIGPOLL, b"SIGPOLL" as &'static [u8]),
                (libc::SIGPWR, b"SIGPWR"),
                (libc::SIGSTKFLT, b"SIGSTKFLT"),
            ]);
            // libc's range excludes its own reserved thread-library signals.
            let first = libc::SIGRTMIN();
            let last = libc::SIGRTMAX();
            REALTIME_FIRST.store(first, Ordering::Release);
            REALTIME_LAST.store(last, Ordering::Release);
            all.extend((first..=last).map(|signal| (signal, b"SIGRT" as &'static [u8])));
        }
        all
    }
    // No vector or lookup with allocation in the handler itself.
    #[cfg(target_os = "linux")]
    static REALTIME_FIRST: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
    #[cfg(target_os = "linux")]
    static REALTIME_LAST: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
    static OLD_HANDLERS: [std::sync::atomic::AtomicUsize; 65] =
        [const { std::sync::atomic::AtomicUsize::new(0) }; 65];
    static OLD_FLAGS: [std::sync::atomic::AtomicI32; 65] =
        [const { std::sync::atomic::AtomicI32::new(0) }; 65];
    extern "C" fn received(
        signal: i32,
        info: *mut libc::siginfo_t,
        context: *mut std::ffi::c_void,
    ) {
        #[cfg(target_os = "linux")]
        let mut realtime_name = Bytes::new();
        let name: &[u8] = match signal {
            libc::SIGHUP => b"SIGHUP",
            libc::SIGINT => b"SIGINT",
            libc::SIGQUIT => b"SIGQUIT",
            libc::SIGILL => b"SIGILL",
            libc::SIGTRAP => b"SIGTRAP",
            libc::SIGABRT => b"SIGABRT",
            libc::SIGFPE => b"SIGFPE",
            libc::SIGBUS => b"SIGBUS",
            libc::SIGSEGV => b"SIGSEGV",
            libc::SIGSYS => b"SIGSYS",
            libc::SIGPIPE => b"SIGPIPE",
            libc::SIGALRM => b"SIGALRM",
            libc::SIGTERM => b"SIGTERM",
            libc::SIGXCPU => b"SIGXCPU",
            libc::SIGXFSZ => b"SIGXFSZ",
            libc::SIGVTALRM => b"SIGVTALRM",
            libc::SIGPROF => b"SIGPROF",
            libc::SIGUSR1 => b"SIGUSR1",
            libc::SIGUSR2 => b"SIGUSR2",
            #[cfg(target_os = "macos")]
            libc::SIGEMT => b"SIGEMT",
            #[cfg(target_os = "linux")]
            libc::SIGPOLL => b"SIGPOLL",
            #[cfg(target_os = "linux")]
            libc::SIGPWR => b"SIGPWR",
            #[cfg(target_os = "linux")]
            libc::SIGSTKFLT => b"SIGSTKFLT",
            #[cfg(target_os = "linux")]
            other
                if other >= REALTIME_FIRST.load(Ordering::Acquire)
                    && other <= REALTIME_LAST.load(Ordering::Acquire) =>
            {
                realtime_name.add(b"SIGRTMIN");
                let offset = other - REALTIME_FIRST.load(Ordering::Acquire);
                if offset != 0 {
                    realtime_name.add(b"+");
                    realtime_name.number(offset as u64, 1);
                }
                realtime_name.slice()
            }
            _ => return,
        };
        // Linux's highest real-time signal is 64, using the otherwise unused
        // zero bit. Every delivered signal still has its own once-bit.
        last((signal % 64) as u32, name);
        // SA_RESETHAND has restored the default. Queue the same signal on this
        // thread, then return so its mask is restored and its default runs.
        // No clean run-mark is written and no shell quit path is invoked.
        let old = OLD_HANDLERS[signal as usize].load(Ordering::Acquire);
        if old == libc::SIG_DFL {
            unsafe {
                libc::raise(signal);
            }
        } else if OLD_FLAGS[signal as usize].load(Ordering::Acquire) & libc::SA_SIGINFO != 0 {
            let call: extern "C" fn(i32, *mut libc::siginfo_t, *mut std::ffi::c_void) =
                unsafe { std::mem::transmute(old) };
            call(signal, info, context);
        } else {
            let call: extern "C" fn(i32) = unsafe { std::mem::transmute(old) };
            call(signal);
        }
    }
    pub(super) fn install() -> Result<(), String> {
        let mut errors = Vec::new();
        for (signal, _) in endings() {
            unsafe {
                let mut old: libc::sigaction = std::mem::zeroed();
                if libc::sigaction(signal, std::ptr::null(), &mut old) != 0 {
                    errors.push(std::io::Error::last_os_error().to_string());
                    continue;
                }
                if old.sa_sigaction == libc::SIG_IGN {
                    continue;
                }
                OLD_HANDLERS[signal as usize].store(old.sa_sigaction, Ordering::Release);
                OLD_FLAGS[signal as usize].store(old.sa_flags, Ordering::Release);
                let mut action = old;
                action.sa_sigaction = received as *const () as usize;
                action.sa_flags |= libc::SA_SIGINFO;
                if OLD_HANDLERS[signal as usize].load(Ordering::Acquire) == libc::SIG_DFL {
                    action.sa_flags |= libc::SA_RESETHAND;
                }
                if OLD_HANDLERS[signal as usize].load(Ordering::Acquire) == libc::SIG_DFL {
                    libc::sigfillset(&mut action.sa_mask);
                }
                if libc::sigaction(signal, &action, std::ptr::null_mut()) != 0 {
                    errors.push(std::io::Error::last_os_error().to_string());
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; "))
        }
    }
    pub(super) fn seconds() -> u64 {
        unsafe { libc::time(std::ptr::null_mut()).max(0) as u64 }
    }
}

/// The diary service's raw append uses this only while a handler is running.
/// No buffered stream, heap, filesystem adapter callback, lock or re-check.
#[cfg(unix)]
pub(crate) fn raw_append(prepared: &Prepared, name: &[u8], bytes: &[u8]) {
    use std::os::fd::AsRawFd;
    unsafe {
        let fd = libc::openat(
            prepared.directory.as_raw_fd(),
            name.as_ptr().cast(),
            libc::O_WRONLY | libc::O_APPEND | libc::O_CREAT | libc::O_CLOEXEC,
            0o600,
        );
        if fd < 0 {
            return;
        }
        // A single append keeps the line whole. A failed or short write is
        // lost, per the diary rule; there is no fallback record.
        let wrote = libc::write(fd, bytes.as_ptr().cast(), bytes.len());
        let _ = wrote;
        #[cfg(target_os = "macos")]
        libc::fcntl(fd, libc::F_FULLFSYNC);
        #[cfg(target_os = "linux")]
        libc::fsync(fd);
        libc::close(fd);
    }
}

#[cfg(target_os = "macos")]
mod mac {
    use super::*;
    use std::ffi::c_void;
    static ORIGINAL: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
    static WINDOW_CLOSE: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
    #[link(name = "objc")]
    unsafe extern "C" {
        fn object_getClass(object: *const c_void) -> *mut c_void;
        fn class_getInstanceMethod(
            class: *mut c_void,
            selector: objc2::runtime::Sel,
        ) -> *mut c_void;
        fn method_setImplementation(
            method: *mut c_void,
            implementation: *mut c_void,
        ) -> *mut c_void;
    }
    unsafe extern "C" fn will_terminate(
        this: *mut c_void,
        selector: objc2::runtime::Sel,
        notification: *mut c_void,
    ) {
        last(40, b"applicationWillTerminate:");
        let original = ORIGINAL.load(Ordering::Acquire);
        if !original.is_null() {
            let call: unsafe extern "C" fn(*mut c_void, objc2::runtime::Sel, *mut c_void) =
                unsafe { std::mem::transmute(original) };
            unsafe {
                call(this, selector, notification);
            }
        }
    }
    unsafe extern "C" fn should_close(
        this: *mut c_void,
        selector: objc2::runtime::Sel,
        window: *mut c_void,
    ) -> objc2::runtime::Bool {
        last(41, b"windowShouldClose:");
        let original = WINDOW_CLOSE.load(Ordering::Acquire);
        let call: unsafe extern "C" fn(
            *mut c_void,
            objc2::runtime::Sel,
            *mut c_void,
        ) -> objc2::runtime::Bool = unsafe { std::mem::transmute(original) };
        unsafe { call(this, selector, window) }
    }
    pub(super) fn install<R: tauri::Runtime>(window: &tauri::WebviewWindow<R>) {
        // The shell's actual delegate method is wrapped and then called
        // unchanged. No second delegate replaces the shell's delegate.
        let Some(main) = objc2::MainThreadMarker::new() else {
            return;
        };
        if let Ok(native) = window.ns_window() {
            let native = native.cast::<objc2::runtime::AnyObject>();
            let delegate: *mut objc2::runtime::AnyObject =
                unsafe { objc2::msg_send![native, delegate] };
            if !delegate.is_null() && WINDOW_CLOSE.load(Ordering::Acquire).is_null() {
                unsafe {
                    let method = class_getInstanceMethod(
                        object_getClass(delegate.cast()),
                        objc2::sel!(windowShouldClose:),
                    );
                    if !method.is_null() {
                        WINDOW_CLOSE.store(
                            method_setImplementation(method, should_close as *mut c_void),
                            Ordering::Release,
                        );
                    }
                }
            }
        }
        let app = objc2_app_kit::NSApplication::sharedApplication(main);
        let delegate: *mut objc2::runtime::AnyObject = unsafe { objc2::msg_send![&app, delegate] };
        if delegate.is_null() || !ORIGINAL.load(Ordering::Acquire).is_null() {
            return;
        }
        let selector = objc2::sel!(applicationWillTerminate:);
        unsafe {
            let method = class_getInstanceMethod(object_getClass(delegate.cast()), selector);
            if !method.is_null() {
                ORIGINAL.store(
                    method_setImplementation(method, will_terminate as *mut c_void),
                    Ordering::Release,
                );
            }
        }
    }
}

/// Called once the native window and its delegate exist, on the drawing thread.
pub fn the_window_is_ready<R: tauri::Runtime>(window: &tauri::WebviewWindow<R>) {
    #[cfg(target_os = "macos")]
    mac::install(window);
    #[cfg(target_os = "windows")]
    if let Ok(handle) = window.hwnd() {
        windows::the_window_is_ready(handle.0 as _);
    }
    #[cfg(target_os = "linux")]
    let _ = window;
}

#[cfg(windows)]
mod windows {
    use super::*;
    use std::ffi::c_void;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CallWindowProcW, SetWindowLongPtrW, GWLP_WNDPROC, WM_CLOSE, WM_ENDSESSION,
        WM_QUERYENDSESSION,
    };
    static OLD_WINDOW: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn SetConsoleCtrlHandler(
            handler: Option<unsafe extern "system" fn(u32) -> i32>,
            add: i32,
        ) -> i32;
        fn GetSystemTimeAsFileTime(time: *mut u64);
        fn CreateFileW(
            name: *const u16,
            access: u32,
            sharing: u32,
            security: *const c_void,
            disposition: u32,
            flags: u32,
            template: *mut c_void,
        ) -> *mut c_void;
        fn WriteFile(
            file: *mut c_void,
            bytes: *const u8,
            length: u32,
            wrote: *mut u32,
            overlapped: *mut c_void,
        ) -> i32;
        fn FlushFileBuffers(file: *mut c_void) -> i32;
        fn CloseHandle(file: *mut c_void) -> i32;
    }
    unsafe extern "system" fn console(event: u32) -> i32 {
        match event {
            0 => last(32, b"CTRL_C_EVENT"),
            1 => last(33, b"CTRL_BREAK_EVENT"),
            2 => last(34, b"CTRL_CLOSE_EVENT"),
            _ => {}
        }
        // Let the next/default handler perform the same ending as before.
        0
    }
    pub(super) fn install() -> Result<(), String> {
        if unsafe { SetConsoleCtrlHandler(Some(console), 1) } == 0 {
            Err(std::io::Error::last_os_error().to_string())
        } else {
            Ok(())
        }
    }
    pub(super) fn seconds() -> u64 {
        let mut time = 0u64;
        unsafe {
            GetSystemTimeAsFileTime(&mut time);
        }
        (time / 10_000_000).saturating_sub(11_644_473_600)
    }
    unsafe extern "system" fn message(
        hwnd: *mut c_void,
        message: u32,
        wparam: usize,
        lparam: isize,
    ) -> isize {
        match message {
            WM_QUERYENDSESSION => last(35, b"WM_QUERYENDSESSION"),
            WM_ENDSESSION => last(36, b"WM_ENDSESSION"),
            WM_CLOSE => last(37, b"WM_CLOSE"),
            _ => {}
        }
        // Preserve the shell's answer, including a refused session ending.
        let old = OLD_WINDOW.load(Ordering::Acquire);
        unsafe {
            CallWindowProcW(
                Some(std::mem::transmute::<
                    *mut c_void,
                    unsafe extern "system" fn(*mut c_void, u32, usize, isize) -> isize,
                >(old)),
                hwnd,
                message,
                wparam,
                lparam,
            )
        }
    }
    pub(super) fn the_window_is_ready(hwnd: *mut c_void) {
        if !OLD_WINDOW.load(Ordering::Acquire).is_null() {
            return;
        }
        let old = unsafe { SetWindowLongPtrW(hwnd, GWLP_WNDPROC, message as *const () as isize) };
        if old != 0 {
            OLD_WINDOW.store(old as *mut c_void, Ordering::Release);
        }
    }
    pub(super) fn append(prepared: &Prepared, name: &[u8], bytes: &[u8]) {
        // The full path was prepared while healthy; only the fixed-size segment
        // suffix is added on the stack. Windows's path ceiling is 32,767 units.
        let mut path = [0u16; 32768];
        if prepared.prefix.len() + name.len() > path.len() {
            return;
        }
        path[..prepared.prefix.len()].copy_from_slice(&prepared.prefix);
        for (at, &byte) in name.iter().enumerate() {
            path[prepared.prefix.len() + at] = byte as u16;
        }
        unsafe {
            // GENERIC_WRITE is required by FlushFileBuffers. FILE_APPEND_DATA
            // alone does not satisfy its contract. Set the write position to
            // EOF via OVERLAPPED's all-ones offset, without a seek race.
            let file = CreateFileW(
                path.as_ptr(),
                0x40000000,
                7,
                std::ptr::null(),
                4,
                0x80,
                std::ptr::null_mut(),
            );
            if file as isize == -1 {
                return;
            }
            #[repr(C)]
            struct Overlapped {
                internal: usize,
                internal_high: usize,
                offset: u32,
                offset_high: u32,
                event: *mut c_void,
            }
            let mut offset = Overlapped {
                internal: 0,
                internal_high: 0,
                offset: u32::MAX,
                offset_high: u32::MAX,
                event: std::ptr::null_mut(),
            };
            let mut wrote = 0;
            WriteFile(
                file,
                bytes.as_ptr(),
                bytes.len() as u32,
                &mut wrote,
                (&mut offset as *mut Overlapped).cast(),
            );
            FlushFileBuffers(file);
            CloseHandle(file);
        }
    }
}
#[cfg(windows)]
use windows as platform;
#[cfg(windows)]
pub(crate) fn raw_append(prepared: &Prepared, name: &[u8], bytes: &[u8]) {
    windows::append(prepared, name, bytes);
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use crate::diary_service::{DiaryService, Wrote};
    use smoozy_reporting::lines::{Line, LineKind};
    use smoozy_state::{identities::IdentityKind, observed::Clock};
    use std::{
        io::{BufRead, Write},
        process::{Command, Stdio},
        time::{Duration, SystemTime},
    };

    const CHILD: &str = "SMOOZY_R5_LAST_LINE_CHILD";
    #[test]
    fn the_dying_child() {
        let Some(folder) = std::env::var_os(CHILD) else {
            return;
        };
        let folder = PathBuf::from(folder);
        let mode = std::env::var("SMOOZY_R5_CHILD_MODE").unwrap();
        let run = Identity::issue(IdentityKind::Run).unwrap();
        let diary = DiaryService::open_at(folder.join("diary")).unwrap();
        let record = crate::record::LocalRecord::open_at(&folder.join("record.sqlite3")).unwrap();
        crate::start::write_run_mark(
            &record,
            &smoozy_state::run_mark::RunMark::this_run_is_running(run.clone()),
        )
        .unwrap();
        let line = |kind, text| Line {
            run: run.clone(),
            kind,
            at: SystemTime::now(),
            clock: Clock::ThisComputer,
            text,
            concerning: vec![],
        };
        assert_eq!(
            diary.write(&line(LineKind::Session, "session".into())),
            Wrote::Yes
        );
        if mode == "ignored" {
            unsafe {
                libc::signal(libc::SIGTERM, libc::SIG_IGN);
            }
        }
        diary.prepare_the_last_line(&run).unwrap();
        if mode == "rotate" {
            // Force rotation after preparation; the old segment is then removed.
            assert_eq!(
                diary.write(&line(LineKind::Act, "x".repeat(1024 * 1024))),
                Wrote::Yes
            );
            std::fs::remove_file(diary.area().join("diary-000000000000")).unwrap();
        }
        println!("READY {}", run.as_text());
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        assert_eq!(input, "quit\n", "only the parent's quit request marks a clean run");
        if mode == "repeat" {
            last(40, b"applicationWillTerminate:");
            last(40, b"applicationWillTerminate:");
        }
        crate::start::write_run_mark(
            &record,
            &smoozy_state::run_mark::RunMark::the_user_asked_to_quit(run),
        )
        .unwrap();
    }

    /// Cargo starts this test. Its own helper is a copy of the test harness in
    /// a temporary folder, never Smoozy or an executable under App/target-*.
    /// A cancellable 20-second deadline kills the helper even after a panic.
    #[test]
    fn a_killed_child_leaves_exactly_what_the_system_let_it_write() {
        let folder =
            crate::scratch_folders::a_folder(format!("smoozy-r5-child-{}", std::process::id()));
        let executable = folder.join("the-test-child");
        std::fs::copy(std::env::current_exe().unwrap(), &executable).unwrap();
        for (mode, signal, expected) in [
            ("term", Some(libc::SIGTERM), Some("SIGTERM")),
            ("hard", Some(libc::SIGKILL), None),
            ("rotate", Some(libc::SIGTERM), Some("SIGTERM")),
            ("clean", None, None),
            ("repeat", None, Some("applicationWillTerminate:")),
            ("ignored", None, None),
        ] {
            let at = folder.join(mode);
            std::fs::create_dir_all(&at).unwrap();
            let mut child = Command::new(&executable)
                .args([
                    "--exact",
                    "the_last_line::tests::the_dying_child",
                    "--nocapture",
                ])
                .env(CHILD, &at)
                .env("SMOOZY_R5_CHILD_MODE", mode)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            // wait_with_output closes child.stdin before waiting. Keep the
            // pipe here until the child exits, so EOF cannot let it mark a
            // clean quit while a terminating signal is still being handled.
            let mut input = child.stdin.take().unwrap();
            let pid = child.id() as i32;
            let (cancel, deadline) = std::sync::mpsc::channel();
            let guard = std::thread::spawn(move || {
                if deadline.recv_timeout(Duration::from_secs(20)).is_err() {
                    unsafe {
                        libc::kill(pid, libc::SIGKILL);
                    }
                }
            });
            let mut stdout = std::io::BufReader::new(child.stdout.take().unwrap());
            let ready = (&mut stdout)
                .lines()
                .map_while(Result::ok)
                .find(|line| line.starts_with("READY "));
            let ready = ready.expect("the helper prepared its own diary before its deadline");
            let run = Identity::read_back(IdentityKind::Run, ready.strip_prefix("READY ").unwrap())
                .unwrap();
            if mode == "ignored" {
                assert_eq!(unsafe { libc::kill(pid, libc::SIGTERM) }, 0);
            }
            if let Some(signal) = signal {
                assert_eq!(unsafe { libc::kill(pid, signal) }, 0);
            } else {
                input.write_all(b"quit\n").unwrap();
            }
            let output = child.wait_with_output().unwrap();
            drop(input);
            let _ = cancel.send(());
            guard.join().unwrap();
            assert_eq!(
                output.status.success(),
                signal.is_none(),
                "{mode}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let diary = DiaryService::open_at(at.join("diary")).unwrap();
            let read = diary.read_back().unwrap();
            assert_eq!(read.lines_that_would_not_read, 0, "{mode}");
            let last: Vec<_> = read
                .lines
                .iter()
                .filter(|l| l.kind == LineKind::Last)
                .collect();
            assert_eq!(last.len(), usize::from(expected.is_some()), "{mode}");
            if let Some(ending) = expected {
                assert_eq!(last[0].text, ending);
                assert_eq!(last[0].run, run);
                assert_eq!(last[0].clock, Clock::ThisComputer);
                assert!(last[0].at <= SystemTime::now());
            }
            // No handler marks a crash clean. Clean quits still do so.
            let record = crate::record::LocalRecord::open_at(&at.join("record.sqlite3")).unwrap();
            let state: String = record
                .connection()
                .query_row("SELECT state FROM run_mark", [], |r| r.get(0))
                .unwrap();
            assert_eq!(
                state,
                if signal.is_some() { "running" } else { "clean" },
                "{mode}"
            );
            if mode == "rotate" {
                assert!(!diary.area().join("diary-000000000000").exists());
            }
        }
    }
}

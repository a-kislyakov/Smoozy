//! Slow local reads, not detection of a macOS question.
//! `Design/Dependency Upload & Remap § A slow local read on macOS` and its Sources.
//!
//! One timer per check or upload, sleeping until a change or read deadline;
//! it does not poll the disk or the permission database. Only this thread's
//! synchronous local reads are watched; server waits never start the timer.
use std::cell::RefCell;
use std::future::Future;
use std::marker::PhantomData;
use std::os::unix::ffi::OsStrExt;
use std::path::{Component, Path, PathBuf};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

thread_local! {
    static CURRENT: RefCell<Option<Arc<ReadStatus>>> = const { RefCell::new(None) };
}

struct ActiveRead {
    serial: u64,
    folder: String,
    began: Instant,
    announced: bool,
}
#[derive(Default)]
struct State {
    serial: u64,
    active: Option<ActiveRead>,
    closed: bool,
}

pub struct ReadStatus {
    home: PathBuf,
    state: Mutex<State>,
    wake: Condvar,
    changed: Arc<dyn Fn() + Send + Sync>,
}
impl ReadStatus {
    pub fn while_polled<F: Future>(self: Arc<Self>, future: F) -> DuringRead<F> {
        DuringRead {
            status: self,
            future: Box::pin(future),
        }
    }

    pub fn enter(self: &Arc<Self>) -> ReadScope {
        let previous = CURRENT.with(|current| current.replace(Some(self.clone())));
        ReadScope {
            previous,
            _same_thread: PhantomData,
        }
    }

    pub fn hint(&self) -> Option<String> {
        self.state
            .lock()
            .ok()?
            .active
            .as_ref()
            .filter(|read| read.announced)
            .map(|read| read.folder.clone())
    }
}

/// Restore the caller's scope before a future yields or a read unwinds.
/// This guard cannot be sent to a different thread.
pub struct ReadScope {
    previous: Option<Arc<ReadStatus>>,
    _same_thread: PhantomData<Rc<()>>,
}
impl Drop for ReadScope {
    fn drop(&mut self) {
        CURRENT.with(|current| current.replace(self.previous.take()));
    }
}

/// The scope is installed only during each poll. Upload work can resume on
/// another runtime thread; no scope is left on a thread after it yields.
pub struct DuringRead<F> {
    status: Arc<ReadStatus>,
    future: Pin<Box<F>>,
}
impl<F: Future> Future for DuringRead<F> {
    type Output = F::Output;
    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let _scope = this.status.enter();
        this.future.as_mut().poll(context)
    }
}

/// One sleeping timer for a check or its upload, closed on every exit.
pub struct Watching {
    pub status: Arc<ReadStatus>,
    worker: Option<std::thread::JoinHandle<()>>,
}
impl Watching {
    pub fn start(
        home: PathBuf,
        delay: Duration,
        changed: Arc<dyn Fn() + Send + Sync>,
        activate: Arc<dyn Fn() + Send + Sync>,
    ) -> Self {
        let status = Arc::new(ReadStatus {
            home,
            state: Mutex::new(State::default()),
            wake: Condvar::new(),
            changed,
        });
        let timer = status.clone();
        let worker = std::thread::spawn(move || {
            let mut state = timer.state.lock().unwrap();
            loop {
                if state.closed {
                    break;
                }
                let Some(read) = state.active.as_ref().filter(|read| !read.announced) else {
                    state = timer.wake.wait(state).unwrap();
                    continue;
                };
                if let Some(left) = delay
                    .checked_sub(read.began.elapsed())
                    .filter(|left| !left.is_zero())
                {
                    state = timer.wake.wait_timeout(state, left).unwrap().0;
                    continue;
                }
                state.active.as_mut().unwrap().announced = true;
                let serial = state.active.as_ref().unwrap().serial;
                drop(state);
                // Never hold the read lock while notifying the page. The page
                // holds its own lock before asking for this status.
                (timer.changed)();
                let still_waiting = timer
                    .state
                    .lock()
                    .unwrap()
                    .active
                    .as_ref()
                    .is_some_and(|read| read.serial == serial && read.announced);
                if still_waiting {
                    activate();
                }
                state = timer.state.lock().unwrap();
            }
        });
        Self {
            status,
            worker: Some(worker),
        }
    }
}
impl Drop for Watching {
    fn drop(&mut self) {
        let mut state = self.status.state.lock().unwrap();
        state.closed = true;
        state.active = None;
        self.status.wake.notify_one();
        drop(state);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

struct Reading {
    status: Arc<ReadStatus>,
    serial: u64,
}
impl Drop for Reading {
    fn drop(&mut self) {
        let mut state = self.status.state.lock().unwrap();
        let announced = state
            .active
            .as_ref()
            .filter(|read| read.serial == self.serial)
            .is_some_and(|read| read.announced);
        if state
            .active
            .as_ref()
            .is_some_and(|read| read.serial == self.serial)
        {
            state.active = None;
        }
        self.status.wake.notify_one();
        drop(state);
        if announced {
            (self.status.changed)();
        }
    }
}

pub fn local_read<T>(path: &[u8], folder: bool, read: impl FnOnce() -> T) -> T {
    let status = CURRENT.with(|current| current.borrow().clone());
    let _reading = status.and_then(|status| {
        let folder = folder_for(path, folder, &status.home)?;
        let mut state = status.state.lock().unwrap();
        if state.closed {
            return None;
        }
        state.serial += 1;
        let serial = state.serial;
        state.active = Some(ActiveRead {
            serial,
            folder,
            began: Instant::now(),
            announced: false,
        });
        status.wake.notify_one();
        drop(state);
        Some(Reading { status, serial })
    });
    read()
}

/// No metadata, canonicalization or permission probe: those can be the read
/// that blocks. `/Volumes` says where the path is, not what kind of disk it is.
///
/// **A path that climbs with `..` is named by where it lands, from its names
/// alone** — `Design/Dependency Upload & Remap § A slow local read on macOS`,
/// its sentence of 2026-11-12. A reference relative to its file's folder is
/// joined with its `..` kept, and refusing those left the live pass's Documents
/// question with no hint (new fault 7).
fn folder_for(path: &[u8], folder: bool, home: &Path) -> Option<String> {
    let path = Path::new(std::ffi::OsStr::from_bytes(path));
    if !path.is_absolute() {
        return None;
    }
    let mut landed = PathBuf::new();
    for part in path.components() {
        match part {
            Component::ParentDir => {
                landed.pop();
            }
            Component::CurDir => {}
            other => landed.push(other),
        }
    }
    let path = landed.as_path();
    if home.is_absolute() && home != Path::new("/") && path.starts_with(home) {
        for name in ["Documents", "Desktop", "Downloads"] {
            let protected = home.join(name);
            if path.starts_with(&protected) {
                return Some(protected.to_string_lossy().into_owned());
            }
        }
    } else if !path.starts_with("/Volumes") {
        return None;
    }
    let folder = if folder { path } else { path.parent()? };
    Some(folder.to_string_lossy().into_owned())
}

/// AppKit only offers activation of our app, not control of the privacy
/// question. Use the older call only on systems without the newer selector.
pub fn request_activation<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    let _ = app.run_on_main_thread(|| {
        use objc2_foundation::NSObjectProtocol;
        if let Some(main) = objc2::MainThreadMarker::new() {
            let app = objc2_app_kit::NSApplication::sharedApplication(main);
            if app.respondsToSelector(objc2::sel!(activate)) {
                app.activate();
            } else {
                #[allow(deprecated)]
                app.activateIgnoringOtherApps(true);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::Future;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{mpsc, Mutex};
    const HOME: &str = "/Users/artist";
    fn path(s: &str) -> PathBuf {
        PathBuf::from(s)
    }

    #[test]
    fn macos_access_follows_an_async_read_to_another_thread() {
        let (tx, rx) = mpsc::channel();
        let watch = Watching::start(
            path(HOME),
            Duration::from_millis(15),
            Arc::new(move || {
                let _ = tx.send(());
            }),
            Arc::new(|| {}),
        );
        let status = watch.status.clone();
        let checking = watch.status.clone().while_polled(async move {
            local_read(b"/Users/artist/Documents/x", false, || {
                rx.recv_timeout(Duration::from_secs(1))
                    .expect("the read on another worker tells the page");
                assert_eq!(status.hint(), Some(format!("{HOME}/Documents")));
            });
            7
        });
        let answer = std::thread::spawn(move || {
            let mut checking = Box::pin(checking);
            let mut context = std::task::Context::from_waker(std::task::Waker::noop());
            checking.as_mut().poll(&mut context)
        })
        .join()
        .unwrap();
        assert_eq!(answer, std::task::Poll::Ready(7));
        assert!(watch.status.hint().is_none());
    }

    #[test]
    fn macos_access_names_folders_without_reading_or_resolving_them() {
        for name in ["Documents", "Desktop", "Downloads"] {
            let root = format!("{HOME}/{name}");
            assert_eq!(
                folder_for(
                    format!("{root}/shot/wood.png").as_bytes(),
                    false,
                    &path(HOME)
                ),
                Some(root)
            );
        }
        assert_eq!(
            folder_for(b"/Users/artist/work/shot.blend", false, &path(HOME)),
            Some(format!("{HOME}/work"))
        );
        assert_eq!(
            folder_for(b"/Volumes/Studio/assets", true, &path(HOME)),
            Some("/Volumes/Studio/assets".into())
        );
        for name in [
            b"/Users/artist2/Documents".as_slice(),
            b"relative/shot",
            b"/tmp/shot",
        ] {
            assert!(folder_for(name, true, &path(HOME)).is_none());
        }
        // Since 2026-11-12 a `..` is followed by its names, not refused.
        assert_eq!(
            folder_for(b"/Users/artist/Documents/../elsewhere", true, &path(HOME)),
            Some(format!("{HOME}/elsewhere"))
        );
    }

    /// **A reference relative to its file's folder that climbs out of it.** The
    /// live pass of 2026-11-12, new fault 7: the dependency code joins
    /// `//../textures/wood.png` to the file's folder with the `..` kept, the
    /// watch refused every path holding `..`, and macOS's question for the
    /// Documents folder waited with no hint. `Design/Dependency Upload & Remap
    /// § A slow local read on macOS`, its sentence of 2026-11-12.
    #[test]
    fn macos_access_a_path_climbing_with_dot_dot_still_names_its_folder() {
        let joined = smoozy_dependencies::shift::the_full_old_location(
            &smoozy_dependencies::bytes::RecordedLocation::as_the_reading_carried_it(b"//../textures/wood.png"),
            Some(b"/Users/artist/Documents/shot"),
        )
        .expect("a relative reference with a known folder is joined");
        assert!(joined.windows(2).any(|pair| pair == b".."), "the join keeps the `..`");
        let (tx, rx) = mpsc::channel();
        let watch = Watching::start(
            path(HOME),
            Duration::from_millis(15),
            Arc::new(move || {
                let _ = tx.send(());
            }),
            Arc::new(|| {}),
        );
        let _reading_scope = watch.status.enter();
        local_read(&joined, false, || {
            rx.recv_timeout(Duration::from_secs(1))
                .expect("a slow read of a path with `..` tells the page");
            assert_eq!(watch.status.hint(), Some(format!("{HOME}/Documents")));
        });
        // Where it lands decides, not where it starts.
        assert_eq!(
            folder_for(b"/Users/artist/Projects/../Downloads/x.png", false, &path(HOME)),
            Some(format!("{HOME}/Downloads"))
        );
        assert_eq!(
            folder_for(b"/Users/artist/Documents/../work/./a/x.png", false, &path(HOME)),
            Some(format!("{HOME}/work/a"))
        );
        assert_eq!(folder_for(b"/Users/artist/Documents/../../../tmp/x", false, &path(HOME)), None);
    }

    #[test]
    fn macos_access_a_slow_read_names_its_folder_and_clears_on_denial() {
        let (tx, rx) = mpsc::channel();
        let activations = Arc::new(AtomicUsize::new(0));
        let a = activations.clone();
        let (focused, focus) = mpsc::channel();
        let watch = Watching::start(
            path(HOME),
            Duration::from_millis(15),
            Arc::new(move || {
                let _ = tx.send(());
            }),
            Arc::new(move || {
                a.fetch_add(1, Ordering::SeqCst);
                let _ = focused.send(());
            }),
        );
        let reading_scope = watch.status.enter();
        let result: Result<(), &str> =
            local_read(b"/Users/artist/Documents/shot.blend", false, || {
                rx.recv_timeout(Duration::from_secs(1))
                    .expect("the slow read tells the page");
                assert_eq!(watch.status.hint(), Some(format!("{HOME}/Documents")));
                focus
                    .recv_timeout(Duration::from_secs(1))
                    .expect("one focus request while the read waits");
                assert!(focus.try_recv().is_err());
                Err("access denied")
            });
        assert_eq!(result, Err("access denied"));
        assert!(watch.status.hint().is_none());
        drop(reading_scope);
        drop(watch);
        assert_eq!(activations.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn macos_access_fast_unwatched_and_outside_reads_give_no_hint() {
        let (tx, rx) = mpsc::channel();
        let watch = Watching::start(
            path(HOME),
            Duration::from_secs(1),
            Arc::new(move || {
                let _ = tx.send(());
            }),
            Arc::new(|| panic!("no focus request")),
        );
        let reading_scope = watch.status.enter();
        assert_eq!(
            local_read(b"/Users/artist/Downloads/wood.png", false, || 42),
            42
        );
        assert_eq!(local_read(b"/tmp/smoozy-made-up/wood.png", false, || 7), 7);
        assert!(watch.status.hint().is_none());
        drop(reading_scope);
        drop(watch);
        assert!(rx.try_recv().is_err());
        assert_eq!(local_read(b"/Users/artist/Documents/x", false, || 1), 1);
        let (tx, rx) = mpsc::channel();
        let idle = Watching::start(
            path(HOME),
            Duration::from_millis(5),
            Arc::new(move || {
                let _ = tx.send(());
            }),
            Arc::new(|| panic!("a server wait must not request focus")),
        );
        let _idle_scope = idle.status.enter();
        assert!(matches!(
            rx.recv_timeout(Duration::from_millis(30)),
            Err(mpsc::RecvTimeoutError::Timeout)
        ));
        local_read(b"/tmp/smoozy-made-up/wood.png", false, || {
            assert!(matches!(
                rx.recv_timeout(Duration::from_millis(30)),
                Err(mpsc::RecvTimeoutError::Timeout)
            ));
        });
        assert!(idle.status.hint().is_none());
    }

    #[test]
    fn macos_access_a_panic_clears_the_hint_and_the_next_read_can_warn() {
        let (tx, rx) = mpsc::channel();
        let watch = Watching::start(
            path(HOME),
            Duration::from_millis(15),
            Arc::new(move || {
                let _ = tx.send(());
            }),
            Arc::new(|| {}),
        );
        let _reading_scope = watch.status.enter();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            local_read(b"/Users/artist/Desktop/x", false, || {
                rx.recv_timeout(Duration::from_secs(1)).unwrap();
                panic!("a test read failed");
            })
        }));
        assert!(result.is_err());
        assert!(watch.status.hint().is_none());
        // The completion notification is distinct from the next slow read.
        rx.recv_timeout(Duration::from_secs(1)).unwrap();
        local_read(b"/Volumes/Studio/shot.blend", false, || {
            rx.recv_timeout(Duration::from_secs(1)).unwrap();
            assert_eq!(watch.status.hint(), Some("/Volumes/Studio".into()));
        });
    }

    #[test]
    fn macos_access_the_panel_replaces_reading_then_restores_it() {
        use crate::surface::the_dependency_check::{keyed, the_panel, WhatTheRunHolds};
        let held = Mutex::new(WhatTheRunHolds::default());
        let key = keyed("server", "/shot.blend");
        let (tx, rx) = mpsc::channel();
        let watch = Watching::start(
            path(HOME),
            Duration::from_millis(15),
            Arc::new(move || {
                let _ = tx.send(());
            }),
            Arc::new(|| {}),
        );
        let _reading_scope = watch.status.enter();
        held.lock().unwrap().files_being_read.insert(key.clone());
        held.lock()
            .unwrap()
            .macos_reads
            .insert(key, vec![watch.status.clone()]);
        local_read(b"/Users/artist/Documents/x", false, || {
            rx.recv_timeout(Duration::from_secs(1)).unwrap();
            let panel = the_panel(&held.lock().unwrap(), "server", "/shot.blend", None);
            let line = serde_json::to_string(&panel.still_reading)
                .unwrap()
                .replace('\u{a0}', " ");
            assert!(line.contains("macOS may be asking for access to"), "{line}");
            assert!(line.contains("/Users/artist/Documents"), "{line}");
            assert!(panel.the_table.is_none());
        });
        let panel = the_panel(&held.lock().unwrap(), "server", "/shot.blend", None);
        assert!(serde_json::to_string(&panel.still_reading)
            .unwrap()
            .replace('\u{a0}', " ")
            .contains("Reading the file…"));
    }
}

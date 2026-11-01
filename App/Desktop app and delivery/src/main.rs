// The Smoozy desktop application.
//
// Step 1 of `Preparation/Build/Plan.md`: the chosen shell and nothing else in
// it. The frame given up, the title bar drawn by Smoozy with its three
// buttons, the light-or-dark setting read and declared back, the two faces
// resolved, and the typography point on the path to the screen.
//
// The shell is Tauri v2 and the language is Rust, settled by the measurement
// at `Preparation/Build/Shell measurement.md` and recorded at
// `Desktop app and delivery` H4, H5b, H5c and H7.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// **The application's own parts live in the library beside this file**, at
// `lib.rs`, which says why. What stays here is the window, the start
// sequence's two call sites and the askpass helper branch.
//
// **The commands are no longer here, and that is step 10a piece A's doing.**
// They are one file per service under `src/surface/`, with one registration
// list; the rule and its reasons are at that folder's own head. Eight pieces of
// step 10a and eleven screens after them all add a command, and one list in the
// file that also holds the window construction is a merge nobody wins.

use std::sync::Arc;

use smoozy_desktop::{
    events, keys, notifications, start, surface, the_one_instance, web_content_boundary,
};

use tauri::Manager;

/// **Whether the one window is handed the paths of whatever is dropped on it.**
///
/// `File Browser` H3a is where the drop route lives — dragging onto the list is
/// the shell's own facility, and what arrives is paths and nothing else.
/// `Desktop app and delivery` §22 item 38 is where the platform question lives,
/// and it is that item that says this setting is this document's rather than
/// `File Browser`'s to make.
///
/// **What would break if somebody turned this off:** the window would be handed
/// no paths at all, so on Windows a drop would silently do nothing and there
/// would be nothing to report to the user.
///
/// The shell publishes only an *off* — `disable_drag_drop_handler`, whose own
/// words are that disabling it *is required to use HTML5 drag and drop APIs on
/// the frontend on Windows* — and publishes its default as *enabled*. The value
/// is written here so the choice is made in one visible place rather than
/// inherited from that default. **The other side of the same choice binds the
/// page: Smoozy's own interface must not reach for HTML5 drag and drop**, the
/// two being alternatives on Windows.
const THE_WINDOW_IS_HANDED_DROPPED_PATHS: bool = true;

/// The one window, built here rather than declared in the configuration file.
///
/// Two reasons. The window differs by system — Design settled on 2026-09-02
/// that macOS keeps its own frame while Windows and Linux give it up — and the
/// shell's per-system configuration file *replaces* the window list instead of
/// merging into it, so every value the shared file held is silently lost. One
/// definition in one place cannot drift.
///
/// It is shown as soon as it is made. Hiding it until the page had drawn was
/// tried and refused: nothing but the page could then show it, so any trouble
/// in the page left an application running with no window at all, which is
/// Smoozy going quiet. A window that appears a frame before its buttons is a
/// far smaller fault than one that never appears.
fn build_window(app: &tauri::App) -> tauri::Result<()> {
    let builder = tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::default())
        .title("Smoozy")
        .inner_size(1180.0, 760.0)
        // **The smallest the window goes, and it has to be below the floor.**
        //
        // `Desktop app and delivery` §3.3 P11's neighbour in Design: *"Below
        // that Smoozy says the window is too small rather than folding… So
        // there is a floor, it is stated on screen, and it names the width."*
        // The floor is `surface::panes::THE_FLOOR`.
        //
        // **This was 880 from step 1 until 2026-09-08**, which is above the
        // fourth narrowing step at 780 and above the floor at 680 — so neither
        // could ever be reached by anybody dragging the window on any system,
        // and the *Smoozy needs a wider window* panel could never appear. That
        // is the same failure the container queries caused on an old Mac,
        // arrived at by a different route, and it was found by step 10a piece B
        // going to watch the floor work. A test in `surface::panes` holds it.
        //
        // **420 is a decision rather than a measurement**: it is well below the
        // floor, so the panel is plainly reachable rather than a value away
        // from it, and it is wide enough for that panel's two sentences. The
        // height is left as it was, no clause turning on it.
        .min_inner_size(420.0, 560.0)
        .resizable(true)
        // **`Desktop app and delivery` H31's two refusals, and on this shell
        // they are installed here rather than declared in the configuration.**
        // H31a reads the shell's own schema and finds the security block has
        // eight members and no navigation allow-list among them: what the
        // project publishes instead is these two closures, so **a build that
        // sets the configuration and installs neither keeps neither refusal,
        // and nothing in the configuration would show it.** Their contents,
        // and the tests that hold them, are at
        // `smoozy_desktop::web_content_boundary`.
        //
        // H36: the boundary fails closed. A refused navigation is cancelled
        // and no application service runs for it; there is no permissive
        // fallback on either.
        .on_navigation(web_content_boundary::the_navigation_is_allowed)
        .on_new_window(|_url, _features| {
            if web_content_boundary::a_new_window_is_ever_opened() {
                tauri::webview::NewWindowResponse::Allow
            } else {
                tauri::webview::NewWindowResponse::Deny
            }
        });

    // `File Browser` H3a and `Desktop app and delivery` §22 item 38, through
    // THE_WINDOW_IS_HANDED_DROPPED_PATHS above, which carries the reasoning and
    // what turning it off would break.
    let builder = if THE_WINDOW_IS_HANDED_DROPPED_PATHS {
        builder
    } else {
        builder.disable_drag_drop_handler()
    };

    // macOS keeps the system frame: its own three buttons in their own place,
    // its own rounded corners and its own shadow, and Smoozy draws none.
    // The page fills the window behind them, and the buttons are placed so
    // they sit level in Smoozy's own title bar.
    #[cfg(target_os = "macos")]
    let builder = builder
        .decorations(true)
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .hidden_title(true)
        .traffic_light_position(tauri::LogicalPosition::new(14.0, 21.0));

    // Windows and Linux give the frame up and Smoozy draws the buttons — the
    // shell offers no equivalent of the Mac's overlay title bar on either.
    // The shadow is kept: the shell publishes that an undecorated window with
    // it gets rounded corners on Windows 11, so the corners come back for
    // nothing. It is unsupported on Linux and does no harm there.
    #[cfg(not(target_os = "macos"))]
    let builder = builder.decorations(false).shadow(true);

    builder.build()?;
    Ok(())
}

fn main() {
    // **The askpass helper is this same binary, re-entered, and the branch is
    // the first thing `main` does** — `Desktop app and delivery` §9 S7 and
    // `Keys` G3. A helper that had built an application on its way to printing
    // one line would be a second Smoozy running: no window, no record, no
    // adapter and no diary are touched on this path.
    //
    // What says *be the helper* is the presence of the channel variable, whose
    // value is a path and never a secret; the tool sets nothing else and an
    // ordinary launch never carries it.
    if let Some(channel) = keys::askpass::this_launch_is_the_helper() {
        std::process::exit(keys::askpass::be_the_helper(&channel));
    }

    // **The relaunch helper is this same binary too, and its branch is the
    // next thing** — `Desktop app and delivery` Z19, P3's second helper, on
    // Windows' update route (step 11's Windows update piece). It takes no hold, opens no record
    // and builds no window: it starts the new build, watches it, rolls back if
    // it fails, and ends. What says *be the helper* is its own argument, which
    // only the Windows route passes.
    if let Some(work) = smoozy_desktop::updating::the_relaunch_helper::this_launch_is_the_helper() {
        std::process::exit(smoozy_desktop::updating::the_relaunch_helper::be_the_helper(&work));
    }

    // **A throwaway argument that names no folder refuses the start**, with a
    // line on the terminal, rather than opening the real record — step 10b's
    // first audit round, item 10; `smoozy_desktop::platform::locations`
    // carries the rule. A release build compiles none of it.
    #[cfg(debug_assertions)]
    if let Some(argument) = smoozy_desktop::platform::locations::throwaway::an_argument_that_names_no_folder(
        std::env::args_os().skip(1),
    ) {
        eprintln!(
            "{}",
            smoozy_desktop::platform::locations::throwaway::the_refusal(&argument)
        );
        std::process::exit(2);
    }

    // **A desk program that cannot be honoured refuses the start** —
    // `Server protocol and transfers` B10a permits it only beside a throwaway
    // folder, so a line carrying it alone, naming no file, or naming a file
    // that is not there stops here with a line on the terminal. A release
    // build compiles none of it.
    #[cfg(debug_assertions)]
    if let Some(refusal) =
        smoozy_desktop::platform::locations::throwaway::the_desk_programs_refusal(
            std::env::args_os().skip(1),
        )
    {
        eprintln!("{refusal}");
        std::process::exit(2);
    }

    // **The update-launch mode, and the probe's token** (`Reporting a
    // problem` U5a, `Reporting a problem` U5b). Only the update mechanism
    // passes either (Z21): the argument alone is the build a rollback starts,
    // and the argument with the token is the probe
    // (`smoozy_desktop::updating::the_probe`).
    let mode = start::LaunchMode::from_arguments(std::env::args_os().skip(1));
    let the_probes_token = smoozy_desktop::updating::the_probe::this_launch_is_a_probe()
        .filter(|_| mode == start::LaunchMode::Update);

    // **One running copy per user** — P2, and `Reporting a problem` U3 step
    // 1 and `Reporting a problem` C1's first condition. **Decided before the
    // shell is built**, so a second launch opens no record, sweeps nothing,
    // reads no run-mark and builds no window: it asks the running copy to come
    // forward and ends. `smoozy_desktop::the_one_instance` carries the rest.
    // **A launch the update mechanism started waits for the hold instead**
    // (P2's handover): the probe is started before the old process hands it
    // over, and the build a rollback starts before the old process has gone.
    let mut this_runs_hold = match smoozy_desktop::the_one_instance::this_launch_is_decided(
        || {
            if mode == start::LaunchMode::Update {
                smoozy_desktop::the_one_instance::the_launch_waits_for_the_hold()
            } else {
                smoozy_desktop::the_one_instance::the_launch_takes_the_hold()
            }
        },
        smoozy_desktop::the_one_instance::the_running_copy_is_asked_to_come_forward,
    ) {
        the_one_instance::ThisLaunch::IsTheRunningCopy(hold) => {
            the_one_instance::ThisRunsHold::held(hold)
        }
        the_one_instance::ThisLaunch::RunsUnchecked(trouble) => {
            the_one_instance::ThisRunsHold::unchecked(trouble)
        }
        the_one_instance::ThisLaunch::Ends => std::process::exit(0),
    };

    // `Desktop app and delivery` Z10a: a Linux rollback debt is met before
    // the shell's start or an ordinary module can read the migrated record.
    // **Where the reinstall is owed, or the rest could not finish, the launch
    // shows the rollback box and nothing else** (step 11's D6b, Rule 8: it used
    // to ask the system with nothing on screen, and end with nothing on screen
    // where the request was declined). The box is built in `setup` below.
    #[cfg(target_os = "linux")]
    let the_rollback_box = {
        use smoozy_desktop::updating::the_linux_package::{self as linux, TheLaunchMeets};
        match linux::what_this_launch_meets() {
            Ok(TheLaunchMeets::Nothing) => None,
            Ok(TheLaunchMeets::TheRollbackFinished) => std::process::exit(0),
            Ok(TheLaunchMeets::TheBox(the_box)) => Some(Arc::new(the_box)),
            Err(_) => {
                eprintln!("Update installation failed.");
                std::process::exit(1);
            }
        }
    };
    #[cfg(not(target_os = "linux"))]
    let the_rollback_box: Option<Arc<smoozy_desktop::updating::the_linux_package::TheRollbackBox>> =
        None;

    tauri::Builder::default()
        // **The system file picker's route** — `Desktop app and delivery` K11.
        // It is initialised here and reached nowhere but
        // `smoozy_desktop::picker`, which is this document's adapter; **the
        // page is granted none of this plugin's permissions**, so there is no
        // second route to a picker, and that file holds the test.
        .plugin(tauri_plugin_dialog::init())
        // **One command surface, one registration list** — `src/surface/`,
        // whose head carries the rule. Nothing is registered here.
        .invoke_handler(surface::the_registration_list())
        // **The four menus' one route in from the system's own bar** —
        // `Desktop app and delivery` K9 and K14. On macOS the native bar
        // catches the choosing and the keystroke; every item that is not one
        // the platform performs itself arrives here by its own identifier and
        // goes to the same function the page's own click goes to, so an act
        // cannot mean one thing on a Mac and another on Windows.
        //
        // It is installed on all three because the shell raises it only where
        // there is a menu to raise it from, and a `cfg` here would be a second
        // place the arrangement is decided.
        .on_menu_event(|app, event| {
            if let Some(queue) = app.try_state::<Arc<events::TheOneQueue>>() {
                surface::menus::the_item_was_chosen(app, &queue, event.id().as_ref());
            }
        })
        .setup(move |app| {
            // **The rollback box, and nothing else** (step 11's D6b): the one
            // queue, the box on it, the window and the hold. **No spine, no
            // record opening and no module**, so no ordinary work is admitted
            // while the rollback is owed (Z21a). The box's answers arrive on
            // the existing update command (`surface::menus`).
            if let Some(the_box) = the_rollback_box {
                let queue = Arc::new(events::TheOneQueue::new());
                app.manage(queue.clone());
                the_box.is_posted_on(&queue, the_box.opens_failed);
                app.manage(the_box);
                build_window(app)?;
                if let Some(hold) = this_runs_hold.the_hold() {
                    let handle = app.handle().clone();
                    hold.knocks_are_answered_by(move || {
                        the_one_instance::the_window_comes_forward(&handle)
                    });
                }
                app.manage(std::sync::Mutex::new(this_runs_hold));
                return Ok(());
            }

            // **The one ordered queue** — `Desktop app and delivery` P6 — made
            // before the spine runs, because the spine posts on it. One per
            // run, and the interface holds no other route to a module.
            //
            // It is made here rather than inside the start so that **the whole
            // application reaches the same one**: the shell's own managed state
            // is what every command and every module will take it from, and a
            // queue handed out by the start's return value would be one more
            // thing each caller could get wrong.
            let queue = Arc::new(events::TheOneQueue::new());
            app.manage(queue.clone());

            // **The notification service, and its activation route opened
            // during launch** — `Desktop app and delivery` §13, and T28 in
            // particular. **Making it posts nothing and asks nothing**: T7
            // puts the macOS request at the first moment a notification would
            // actually be posted and never at application start.
            //
            // **The route has to be open before launching finishes.** Apple's
            // words are that assigning the delegate later *"might cause you to
            // miss incoming notifications"*, and this is the last point in the
            // start-up that is still inside the launch.
            let notifications = Arc::new(notifications::TheNotificationService::for_this_system());
            app.manage(notifications.clone());
            {
                let window = app.handle().clone();
                let on_the_queue = queue.clone();
                let service = notifications.clone();
                notifications::the_activation_route_is_open(Box::new(move |the_post| {
                    service.a_notification_was_clicked(&window, &on_the_queue, &the_post);
                }));
            }

            // The spine, before the window. `Desktop app and delivery` L3a and
            // L6 and `State, identity and recovery` I2 all say *at application
            // start*, and `State, identity and recovery` I2 adds *before any
            // feature module composes* — so it runs here, and **nothing it
            // reports stops the window opening**.
            let started = start::start(&queue);
            surface::reporting::the_launch_may_offer_a_crash(
                &surface::reporting::THE_CRASH_OFFER, &queue, &started,
                this_runs_hold.this_run_is_the_one_running_copy(), mode,
            );
            if let (Some(diary), Some(run)) = (started.diary.as_ref(), started.run.as_ref()) {
                let _ = diary.prepare_the_last_line(run);
            }

            // **The per-kind switches, handed to the notification service as
            // soon as the record is open and before anything can post** —
            // `Desktop app and delivery` T8 and T17a. A record that did not
            // open or did not answer hands over nothing, and then no kind is
            // attempted; the reason is at `TheNotificationService::post`.
            notifications.the_switches_are(
                started
                    .record
                    .as_ref()
                    .and_then(|record| {
                        smoozy_desktop::record::settings::read(record.connection()).ok()
                    })
                    .map(|settings| settings.the_switches),
            );
            // **Behind one lock, which is the record's own shape rather than a
            // convenience.** `State, identity and recovery` §1.3 rule 1 is one
            // writer per durable store, and §3.1 is one process and one
            // instance; the store calls of §9 already run off the interface
            // thread, so the spine is reached from more than one thread and
            // must be serialised somewhere. One lock at the top is that place.
            // **The two rate budgets and the one desktop-side admission point,
            // made once and set from what Settings saved** — `Desktop app and
            // delivery` §11 R3 and R10, and step 10b piece 7.
            //
            // **It is made here rather than anywhere else for R10's reason**:
            // there is one point that knows every server at once, so it is the
            // run's and not a screen's or a period's. **The caps are read from
            // the record before it is handed over**, because R3 splits the
            // durable setting from the object and this is the moment the two
            // meet; a record that would not answer moves neither budget, which
            // leaves R8's *no cap set* standing rather than lifting a limit the
            // user set.
            let transfers = Arc::new(smoozy_desktop::downloading::TheRunsTransfers::for_this_run());
            transfers.the_caps_are_read_from(started.record.as_ref());
            app.manage(transfers);
            // **`Server Setup` R1b: the end of the last transfer with a server
            // is one of the two moments an owed swap of the driver starts
            // at** (`State, identity and recovery` F15's first). Told from
            // wherever that transfer ended, so it is taken off that thread at
            // once. Added 2026-10-12.
            {
                let handle = app.handle().clone();
                smoozy_desktop::servers::the_driver_swap::when_the_last_transfer_with_a_server_ends(
                    Box::new(move |server: &str| {
                        if !smoozy_desktop::servers::the_driver_swap::a_swap_is_owed_on(server) {
                            return;
                        }
                        let (handle, server) = (handle.clone(), server.to_string());
                        let _ = std::thread::Builder::new()
                            .name("smoozy driver swap moment".to_string())
                            .spawn(move || {
                                smoozy_desktop::servers::the_driver_swap::an_owed_swap_is_tried(
                                    &handle, &server, None,
                                )
                            });
                    }),
                );
            }

            // **Z21a: an uncleared update-in-probe record, met at the start**
            // — completed here by any launch but the probe of its release,
            // which is owed its token once the window is built (below).
            let owed = start::the_probe_record_is_met(&started, the_probes_token.clone());

            app.manage(std::sync::Mutex::new(started));

            // **The update check, and P10's one standing timer** — Z23's check
            // at the start and at the one repeating interval, and Z26's press
            // reaching the same one through the shell's managed state. Made
            // after the spine and the admission point, because a check reads
            // the record and is admitted at that point (Z22); it runs off the
            // interface thread and nothing waits on it, so nothing about it can
            // stop a window opening. **A copy that never checks starts no
            // sequence** (E10a). `smoozy_desktop::updating::checking` carries
            // the rest.
            let update_check = smoozy_desktop::updating::checking::TheUpdateCheck::for_this_copy();
            app.manage(update_check.clone());
            smoozy_desktop::updating::checking::the_sequence_starts(
                app.handle().clone(),
                update_check,
            );

            // **Every connected period this run holds** — `Servers` §1, and
            // step 10b piece 5. **Made after the spine, because it reaches the
            // record through the application's one lock**, and before the
            // window, because nothing about it can stop a window opening.
            //
            // **It is the run's one publisher as well as its one register**:
            // `src/servers/published.rs` holds the observed value of every
            // server in one table, and a publisher per period would be that
            // table in as many pieces as there are connected servers.
            app.manage(Arc::new(
                smoozy_desktop::servers::a_period::ThePeriodsThisRun::for_this_run(
                    app.handle().clone(),
                    queue.clone(),
                    notifications.clone(),
                ),
            ));
            // **`Servers` Q1: a step 9 a setup held back is taken when the
            // run ends** (the twenty-fourth notes, note 4).
            smoozy_desktop::servers::the_machines_work::a_held_back_step_nine_follows_every_run(
                app.handle().clone(),
            );

            // **T1's machinery, made at the application's first successful
            // start** — `Desktop app and delivery` §19 E3, which puts it
            // exactly there. On Windows that is the per-user Start-menu
            // shortcut carrying the application identifier, without which no
            // toast can be raised at all; on the other two there is nothing to
            // make and the call does nothing.
            //
            // **It is here rather than inside the notification service's own
            // reading**, and that is F-10 of step 10a round 1: the reading is
            // published as a read, and a read that made a file put something in
            // the user's own Start menu at whatever moment anybody happened to
            // ask. **It answers nothing** (T23) and nothing waits on it — the
            // read is what decides whether a post is attempted, and it is taken
            // afresh at every post.
            //
            // **After the spine, because *successful start* is what E3 says**,
            // and before the window, because nothing about it can stop a window
            // opening.
            notifications::the_machinery_is_made_at_first_start();

            // **The Mac's own menu bar, filled** — `Desktop app and delivery`
            // K1 and K9, and **on macOS that is not optional**: the bar is
            // drawn by the system whether the application fills it or not, so
            // leaving it empty would leave a Mac with an empty system menu bar
            // and Design's four menus with nowhere to go. **Windows and Linux
            // draw the same four themselves, behind one button in the title
            // bar** (K10), which is the page's own module and needs nothing
            // here.
            //
            // **It is filled before the window opens**, so there is no moment
            // in which a Mac shows Smoozy's window above somebody else's menus.
            // **A bar that could not be built does not stop the window
            // opening**, on the same reasoning the window itself is built on:
            // a Smoozy with no menus can still be used and can still report the
            // trouble, and one with no window can do neither.
            #[cfg(target_os = "macos")]
            match smoozy_desktop::menus::the_system_draws_the_bar(app.handle()) {
                Ok(bar) => {
                    let _ = app.set_menu(bar);
                }
                Err(_) => {}
            }

            build_window(app)?;
            if let Some(window) = app.get_webview_window("main") {
                smoozy_desktop::the_last_line::the_window_is_ready(&window);
            }

            // **Z21: the probe's successful start** — its release verified, the
            // record opened and migrated, its first module composed — **is
            // said to its watcher once, here**, and the record is cleared, with
            // Z7's backup and L8's copy, once the watcher lets go
            // (`smoozy_desktop::updating::the_probe`). **On every system**: on
            // Windows the watcher is the relaunch helper (step 11's Windows update piece), and the
            // clearing removes the previous build's folder instead.
            if let start::WhatThisLaunchOwesTheProbe::TheTokenOnceTheWindowIsBuilt { token, probe } = owed {
                if smoozy_desktop::updating::the_probe::the_token_is_said_on_this_programs_output(&token).is_ok() {
                    let handle = app.handle().clone();
                    let _ = std::thread::Builder::new()
                        .name("smoozy probe's watcher".to_string())
                        .spawn(move || {
                            smoozy_desktop::updating::the_probe::the_watcher_lets_go_of_this_program();
                            if let Some(started) = handle.try_state::<std::sync::Mutex<start::Started>>() {
                                let started = started.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                                if let Some(record) = started.record.as_ref() {
                                    let kept = smoozy_desktop::platform::locations::resolve(
                                        smoozy_desktop::platform::locations::Location::ThePreviousBundleKept,
                                    )
                                    .ok();
                                    let running = smoozy_desktop::platform::locations::resolve(
                                        smoozy_desktop::platform::locations::Location::TheInstalledBundle,
                                    )
                                    .ok();
                                    let _ = smoozy_desktop::updating::the_probe::what_the_record_kept_is_cleared(
                                        record.connection(),
                                        record.path(),
                                        &probe,
                                        kept.as_deref(),
                                        running.as_deref(),
                                    );
                                }
                            }
                        });
                }
            }

            // **A second launch's knock brings this window forward** — P2, on
            // macOS and Linux; on Windows the second launch does it itself.
            // Answered only once the window exists, and a knock that came
            // sooner waits in the pipe. **The hold is then kept in the shell's
            // own state, so it lasts exactly as long as the application.**
            if let Some(hold) = this_runs_hold.the_hold() {
                let handle = app.handle().clone();
                hold.knocks_are_answered_by(move || {
                    the_one_instance::the_window_comes_forward(&handle)
                });
            }
            // **Behind a lock**, because the Mac update route hands the hold
            // out of it to the probe and takes it back on a failed start (P2).
            app.manage(std::sync::Mutex::new(this_runs_hold));
            Ok(())
        })
        // `State, identity and recovery` I2: the mark is written **at the
        // moment the user asks to quit**. Closing the window is that moment,
        // and it is the route the drawn button takes on every system.
        .on_window_event(|window, event| {
            if matches!(event, tauri::WindowEvent::CloseRequested { .. }) {
                // **`Servers` S10 item 2: the application run ends, however it
                // ends**, and every period ends with it. **Nothing durable goes
                // with them** — a period is written nowhere (`Servers` S3) — so
                // this is the periods being told rather than anything being
                // saved, and the repair stops before the window does.
                if let Some(periods) = window
                    .try_state::<Arc<smoozy_desktop::servers::a_period::ThePeriodsThisRun>>()
                {
                    periods.the_application_run_ends();
                }
                if let Some(started) = window.try_state::<std::sync::Mutex<start::Started>>() {
                    if let Ok(started) = started.lock() {
                        #[cfg(target_os = "linux")]
                        if let (Some(record), Some(check)) = (started.record.as_ref(), window.try_state::<Arc<smoozy_desktop::updating::checking::TheUpdateCheck>>()) {
                            let _ = check.dismiss_linux_on_quit(record.connection());
                        }
                        start::the_user_asked_to_quit(&started);
                    }
                }
                // **Nothing is swept at close** — a throwaway copy's development
                // items go only with a folder that holds no record yet, at the
                // start (`start::start`). The Creator's eleventh notes, note 2.
            }
        })
        .build(tauri::generate_context!())
        .expect("the Smoozy window could not be started")
        // **The other ways out, and one of them is narrower than `State,
        // identity and recovery` I2 asks for.** `ExitRequested` is the shell's
        // own ask, raised once every window has gone. **`Exit` is not an ask
        // and is included anyway, because on macOS an application quit — the
        // menu, the keyboard, or another program asking — surfaces nothing
        // else**: observed on this machine, where a quit produced `Ready`, a
        // focus event and then `Exit`, with no `ExitRequested` between them.
        // `State, identity and recovery` I2 says *the ask, not the exit*, and
        // on that one route this shell offers no ask, so the exit is the
        // earliest moment there is. **Writing the same value more than once
        // costs nothing; missing a route would make an ordinary quit look like
        // a crash at the next launch**, which is the offer `Reporting a
        // problem` §9 makes, fired at somebody who did nothing wrong.
        .run(|handle, event| {
            if matches!(
                event,
                tauri::RunEvent::ExitRequested { .. } | tauri::RunEvent::Exit
            ) {
                if let Some(started) = handle.try_state::<std::sync::Mutex<start::Started>>() {
                    if let Ok(started) = started.lock() {
                        #[cfg(target_os = "linux")]
                        if let (Some(record), Some(check)) = (started.record.as_ref(), handle.try_state::<Arc<smoozy_desktop::updating::checking::TheUpdateCheck>>()) {
                            let _ = check.dismiss_linux_on_quit(record.connection());
                        }
                        start::the_user_asked_to_quit(&started);
                    }
                }
            }
        });
}

#[cfg(test)]
mod tests {
    /// **The test that would have caught the drop route being handed nothing on
    /// Windows** — `File Browser` H3a and `Desktop app and delivery` §22
    /// item 38.
    ///
    /// Two assertions, and the second is the one with teeth. The first holds
    /// this application's own choice: the window keeps the shell's
    /// drag-and-drop handler. The second holds the shell's published default,
    /// which is what the *keep* branch relies on, because the shell offers no
    /// way to switch the handler back on — a version bump that flipped that
    /// default would hand Windows no paths with nothing in this crate changing,
    /// and this is the line that would fail.
    #[test]
    fn the_window_is_handed_dropped_paths() {
        assert!(
            super::THE_WINDOW_IS_HANDED_DROPPED_PATHS,
            "the window must keep the shell's drag-and-drop handler: with it off, \
             a drop on Windows is handed no paths at all and does nothing, with \
             nothing to report — Desktop app and delivery section 22 item 38"
        );

        assert!(
            tauri::utils::config::WindowConfig::default().drag_drop_enabled,
            "the shell's own default for the drag-and-drop handler is no longer \
             enabled, and the shell publishes no way to switch it back on, so \
             the window is now handed no dropped paths — `File Browser` H3a"
        );
    }
}

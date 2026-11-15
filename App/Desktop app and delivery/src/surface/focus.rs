//! **The Creator's twenty-seventh notes, note 1: the focus ring lands on the
//! control itself** — `Preparation/Design/Whole app.md`, *Keyboard shortcuts*:
//! the ring sits on the tick, the radio dot, the switch or the button, never
//! round the words beside them. The account is `Audits/Build/Twenty-seventh
//! notes/The window's notes — what each one is.md`, note 1.
//!
//! **One test, in headless Chrome, with the screens' own builders and the
//! application's whole stylesheet list.** Each control is drawn by the code
//! that draws it in the app, focused by script, and read from the computed
//! style: the line holding a tick or a dot shows no ring, the tick or the dot
//! carries the accent ring, the control is a Tab stop, and Space ticks or
//! picks it. The buttons, links, rows and fields that have no rule of their
//! own are read for the accent ring rather than the browser's own; the drag
//! handles are read for being out of the Tab order. **Chrome, not the Mac's
//! own engine**: what is read is which rule applies, which both engines
//! answer the same way. No app, server or record is used; skipped, and says
//! so, where Chrome is absent.

use std::{fs, path::PathBuf, time::Duration};

/// **One press of Tab as the window takes it, and a box walked with it** —
/// the twenty-eighth notes, note 2, `Preparation/Design/Whole app.md`,
/// *Keyboard shortcuts*, *Tab stays inside the box on top*. Shared by this
/// file's walk and `a_server.rs`'s, pasted into a page's module script.
///
/// **A key a script sends is never moved by the browser**, so where nothing in
/// the page took the press (its default not prevented), the walk makes the
/// browser's own move: the next Tab stop in the page's order, wrapping at the
/// ends. That is the order a page with no positive tab index gets, and this
/// application sets none. So a box that lets Tab out is seen letting it out,
/// to the rail or the screen behind, exactly where the window would.
///
/// `theBoxIsWalkedWithTab(name, box, check)`: the focus is inside the box when
/// it opens; then as many Tabs as the box has stops, plus one, never leave it
/// and visit every stop; the same with Shift+Tab.
pub(crate) const THE_TAB_PRESS: &str = r#"
const theTabStopIsShown=(one)=>one.isConnected&&!one.closest('[hidden]')&&one.getClientRects().length>0&&getComputedStyle(one).visibility!=='hidden';
const theTabStopsIn=(root)=>[...root.querySelectorAll('a[href],button,input,select,textarea,[tabindex]')].filter(one=>one.tabIndex>=0&&!one.disabled&&!(one.type==='hidden')&&theTabStopIsShown(one));
const theNameOf=(one)=>!one||one===document.body?'nothing':((one.closest&&one.closest('.rail'))?'the rail\'s ':'')+'«'+String(one.getAttribute('aria-label')||one.textContent||one.value||one.className||one.tagName).trim().replace(/\s+/g,' ').slice(0,40)+'»';
const theTabIsPressed=(back)=>{
  const from=document.activeElement||document.body;
  const press=new KeyboardEvent('keydown',{key:'Tab',code:'Tab',shiftKey:back,bubbles:true,cancelable:true});
  from.dispatchEvent(press);
  if(press.defaultPrevented) return 'the page';
  const all=theTabStopsIn(document);
  if(!all.length) return 'nothing';
  let next;
  if(from===document.body||!from.isConnected){ next=back?all[all.length-1]:all[0]; }
  else if(back){ const before=all.filter(one=>one!==from&&(from.compareDocumentPosition(one)&Node.DOCUMENT_POSITION_PRECEDING)&&!one.contains(from)); next=before.length?before[before.length-1]:all[all.length-1]; }
  else { const after=all.filter(one=>one!==from&&(from.compareDocumentPosition(one)&Node.DOCUMENT_POSITION_FOLLOWING)); next=after.length?after[0]:all[0]; }
  next.focus();
  return 'the browser';
};
const theBoxIsWalkedWithTab=(name,box,check)=>{
  check(box.contains(document.activeElement),name+': the focus is not inside it when it opens (it is on '+theNameOf(document.activeElement)+')');
  const stops=theTabStopsIn(box);
  check(stops.length>0,name+': it has no Tab stop');
  if(!stops.length) return;
  const start=document.activeElement;
  for(const back of [false,true]){
    if(start&&start.isConnected&&typeof start.focus==='function') start.focus();
    const seen=new Set();
    for(let at=0;at<=stops.length;at++){
      theTabIsPressed(back);
      const on=document.activeElement;
      if(!box.contains(on)){check(false,name+': '+(back?'Shift+Tab':'Tab')+' left it for '+theNameOf(on));break;}
      seen.add(on);
    }
    check(seen.size===stops.length,name+': '+(back?'Shift+Tab':'Tab')+' reached '+seen.size+' of its '+stops.length+' controls');
  }
  if(start&&start.isConnected&&typeof start.focus==='function') start.focus();
};
"#;

#[test]
fn twenty_seventh_1_the_focus_ring_lands_on_the_control_in_a_real_browser() {
    // **The picture of the page as it was**, drawn from a copy of `web/`
    // taken before the change, is only a picture: nothing is asserted on it.
    if let (Some(pictures), Some(_)) =
        (std::env::var_os("SMOOZY_THE_FOCUS_RING_PICTURES"), std::env::var_os("SMOOZY_THE_FOCUS_RING_WEB"))
    {
        let pictures = PathBuf::from(pictures);
        fs::create_dir_all(&pictures).unwrap();
        the_controls_in_a_real_browser(Some(&pictures.join("One control of each kind, before.png")));
        return;
    }
    let Some(measured) = the_controls_in_a_real_browser(None) else {
        return;
    };
    let faults = measured["faults"].as_array().expect("the faults are listed");
    let read = measured["read"].as_u64().unwrap_or(0);
    eprintln!("{read} controls read");
    assert!(read >= 40, "every control was read: {read}, {measured}");
    assert!(
        faults.is_empty(),
        "{} faults:\n{}",
        faults.len(),
        faults
            .iter()
            .map(|one| one.as_str().unwrap_or_default().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
    if let Some(pictures) = std::env::var_os("SMOOZY_THE_FOCUS_RING_PICTURES") {
        let pictures = PathBuf::from(pictures);
        fs::create_dir_all(&pictures).unwrap();
        the_controls_in_a_real_browser(Some(&pictures.join("One control of each kind, after.png")));
    }
}

/// **The Creator's twenty-eighth notes, note 2: Tab stays inside every box** —
/// `Preparation/Design/Whole app.md`, *Keyboard shortcuts*, *Tab stays inside
/// the box on top*. The account is `Audits/Build/Twenty-eighth notes/Part B —
/// what each one is.md`, note 2, whose table lists every box and panel drawn
/// over the window; the Servers boxes and the panels over a server are walked
/// over the real rail in `a_server.rs`, and every other one here.
///
/// **Each box is opened by the code that opens it in the app** — a delivery
/// through the real event route, or the module's own builder reached by a
/// test-only export — with the application's whole stylesheet list and the
/// shared Escape and Tab rules loaded, and a control before and after it in
/// the page for Tab to escape to. Then: the focus is inside it; Tab and
/// Shift+Tab, one more time than it has stops, never leave it and visit every
/// stop (`THE_TAB_PRESS`); and where the box was opened from a focused control
/// and closes on Escape, the focus goes back to that control. **Chrome, not
/// the Mac's own engine**; skipped, and says so, where Chrome is absent.
#[test]
fn twenty_eighth_2_tab_stays_inside_every_box_in_a_real_browser() {
    let add = serde_json::to_string(&crate::surface::servers::the_add_dialog_from(&[])).unwrap();
    let page = THE_TAB_PAGE.replace("THE_TAB_PRESS", THE_TAB_PRESS).replace("ADD_VALUE", &add);
    let pictures = std::env::var_os("SMOOZY_THE_TAB_PICTURES").map(PathBuf::from);
    if let Some(pictures) = &pictures {
        fs::create_dir_all(pictures).unwrap();
        let when = if std::env::var_os("SMOOZY_THE_FOCUS_RING_WEB").is_some() { "before" } else { "after" };
        for (which, name) in THE_TAB_PICTURES {
            let page = page.replace("PICTURE", which);
            let file = pictures.join(format!("{name}, {when}.png"));
            a_page_in_a_real_browser("tab", &page, &[&THE_EXPORTS, &THE_TAB_EXPORTS], Some(&file));
        }
        if std::env::var_os("SMOOZY_THE_FOCUS_RING_WEB").is_some() {
            return;
        }
    }
    let Some(measured) = a_page_in_a_real_browser("tab", &page.replace("PICTURE", ""), &[&THE_EXPORTS, &THE_TAB_EXPORTS], None)
    else {
        return;
    };
    let faults = measured["faults"].as_array().expect("the faults are listed");
    let walked = measured["walked"].as_array().expect("the boxes walked are listed");
    eprintln!("{} boxes walked: {}", walked.len(), measured["walked"]);
    assert!(
        faults.is_empty(),
        "{} faults:\n{}",
        faults.len(),
        faults
            .iter()
            .map(|one| one.as_str().unwrap_or_default().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(walked.len() >= 24, "every box was walked: {}", measured["walked"]);
    let smoozy = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web/smoozy.js")).unwrap();
    assert!(smoozy.contains("import \"./the-tab-key.js\";"), "the page loads the Tab rule");
}

/// **The pictures for the Creator**: which box is drawn with a Tab walked
/// round it, and the file it goes into. With `SMOOZY_THE_FOCUS_RING_WEB`
/// naming a copy of `web/` from before the change, the same walk on the old
/// page is pictured.
const THE_TAB_PICTURES: [(&str, &str); 3] = [
    ("servers", "Add a server, Tab pressed past its last control"),
    ("keys", "Keys, New key, Tab pressed past its last control"),
    ("queue", "Queue, the Stop box, Tab pressed past its last control"),
];

fn a_chrome() -> Option<PathBuf> {
    [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        "/usr/bin/google-chrome",
        "/usr/bin/chromium",
        "/usr/bin/chromium-browser",
        "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
    ]
    .into_iter()
    .map(PathBuf::from)
    .find(|one| one.exists())
}

/// The page drawn, then either the measurement read back or, with a file
/// named, a picture of one control of each kind with the ring on it.
fn the_controls_in_a_real_browser(picture: Option<&std::path::Path>) -> Option<serde_json::Value> {
    a_page_in_a_real_browser(
        "focus",
        &PAGE.replace("PICTURE", if picture.is_some() { "yes" } else { "" }),
        &[&THE_EXPORTS],
        picture,
    )
}

/// **One page of the application's own files, drawn in headless Chrome**, with
/// test-only exports added to the temporary copies; the `<pre id="out">` it
/// writes is read back, or with a file named, a picture is taken instead.
fn a_page_in_a_real_browser(
    which: &str,
    page: &str,
    exports: &[&[(&str, &str)]],
    picture: Option<&std::path::Path>,
) -> Option<serde_json::Value> {
    a_page_with_stand_ins_in_a_real_browser(which, page, exports, &[], picture)
}

/// **The same, with some of the page's files replaced by stand-ins**, each
/// written over its copy by name — a module drawn on its own, as
/// `the_dependency_check.rs`'s rows are, without the screens it sits in.
pub(crate) fn a_page_with_stand_ins_in_a_real_browser(
    which: &str,
    page: &str,
    exports: &[&[(&str, &str)]],
    stand_ins: &[(&str, &str)],
    picture: Option<&std::path::Path>,
) -> Option<serde_json::Value> {
    let Some(chrome) = a_chrome() else {
        eprintln!("SKIPPED: Chrome is absent; the {which} page was not drawn");
        return None;
    };
    let folder = crate::scratch_folders::a_folder(format!(
        "smoozy-{which}-{}-{}",
        if picture.is_some() { "picture" } else { "read" },
        std::process::id()
    ));
    let web = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web");
    let web = std::env::var_os("SMOOZY_THE_FOCUS_RING_WEB").map(PathBuf::from).unwrap_or(web);
    for entry in fs::read_dir(&web).expect("the page folder") {
        let path = entry.unwrap().path();
        if path.is_file()
            && matches!(
                path.extension().and_then(|one| one.to_str()),
                Some("js" | "css")
            )
        {
            fs::copy(&path, folder.join(path.file_name().unwrap())).expect("the page is copied");
        }
    }
    for (file, text) in stand_ins {
        fs::write(folder.join(file), text).expect("a stand-in");
    }
    // **Test-only exports on the temporary copies reach the actual builders**,
    // as `selecting_text.rs` does. The installed page gains no testing entry.
    for (file, more) in exports.iter().flat_map(|list| list.iter()) {
        let path = folder.join(file);
        let mut source = fs::read_to_string(&path).unwrap();
        source.push('\n');
        source.push_str(more);
        fs::write(path, source).unwrap();
    }
    fs::write(folder.join("index.html"), page).unwrap();
    let dumped = folder.join("dump.html");
    let errors = folder.join("errors.txt");
    let mut command = std::process::Command::new(chrome);
    command.args([
        "--headless",
        "--disable-gpu",
        "--no-first-run",
        "--allow-file-access-from-files",
        "--hide-scrollbars",
        "--virtual-time-budget=8000",
    ]);
    command.arg(if picture.is_some() { "--window-size=1100,760" } else { "--window-size=1100,900" });
    command.arg(format!("--user-data-dir={}", folder.join("profile").display()));
    match picture {
        Some(file) => {
            command.arg("--force-device-scale-factor=2");
            command.arg(format!("--screenshot={}", file.display()));
        }
        None => {
            command.arg("--dump-dom");
        }
    }
    let address = folder.join("index.html").to_string_lossy().replace('\\', "/").replace(' ', "%20");
    let address = if address.starts_with('/') { format!("file://{address}") } else { format!("file:///{address}") };
    let mut child = command
        .arg(address)
        .stdout(fs::File::create(&dumped).unwrap())
        .stderr(fs::File::create(&errors).unwrap())
        .spawn()
        .expect("Chrome starts");
    let pid = child.id();
    #[cfg(windows)]
    let handle = {
        use std::os::windows::io::AsRawHandle;
        child.as_raw_handle() as usize
    };
    let (done, deadline) = std::sync::mpsc::channel::<()>();
    let guard = std::thread::spawn(move || {
        if deadline.recv_timeout(Duration::from_secs(60)).is_err() {
            #[cfg(unix)]
            unsafe {
                libc::kill(pid as i32, libc::SIGKILL);
            }
            #[cfg(windows)]
            unsafe {
                #[link(name = "kernel32")]
                unsafe extern "system" {
                    fn TerminateProcess(handle: *mut std::ffi::c_void, code: u32) -> i32;
                }
                TerminateProcess(handle as *mut std::ffi::c_void, 1);
            }
            #[cfg(not(any(unix, windows)))]
            let _ = pid;
        }
    });
    let status = child.wait().expect("Chrome finishes");
    let _ = done.send(());
    guard.join().unwrap();
    assert!(status.success(), "Chrome failed: {status}; {}", fs::read_to_string(&errors).unwrap_or_default());
    if picture.is_some() {
        return Some(serde_json::Value::Null);
    }
    let page = fs::read_to_string(dumped).unwrap();
    let result = page
        .split("<pre id=\"out\">")
        .nth(1)
        .and_then(|rest| rest.split("</pre>").next())
        .unwrap_or("")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"");
    let measured: serde_json::Value = serde_json::from_str(&result).unwrap_or_else(|error| {
        panic!("no measurement: {error}: {result}\n{}", fs::read_to_string(&errors).unwrap_or_default())
    });
    assert!(measured.get("error").is_none(), "{measured}");
    Some(measured)
}

/// **What each copy adds, at its foot, for the page below.** A builder the
/// module keeps to itself is exported; where a box needs the module's own
/// holder first, the holder is made here, in the module's own scope.
const THE_EXPORTS: [(&str, &str); 9] = [
    ("servers.js", "export { aTickBox, theRemovalBoxIsDrawn };"),
    (
        "render-settings.js",
        "export { aBox as theSceneBox, theEngine, theRendersOn, theDependencyBand };",
    ),
    (
        "keys.js",
        r#"export function theKeyTypesForTheTest(said) {
  screen = { boxes: { newKey: theNewKeyBox() } };
  document.body.appendChild(screen.boxes.newKey.root);
  screen.boxes.newKey.root.hidden = false;
  theTypesAreDrawn(said);
  return screen.boxes.newKey.root;
}"#,
    ),
    (
        "set-up.js",
        r#"export function theInstallBoxForTheTest(list) {
  boxes = document.body.appendChild(document.createElement("div"));
  const box = boxes.appendChild(document.createElement("div"));
  box.className = "mbox";
  theInstallBoxIsDrawn(box, list, "s1");
  return box;
}
export function theAdminBoxForTheTest(asked) {
  boxes = document.body.appendChild(document.createElement("div"));
  theAdminAccessBoxIsDrawn(asked, "s1");
  return boxes;
}"#,
    ),
    ("transfers.js", "export { theFormatBoxIsDrawn };"),
    (
        "files.js",
        r#"export function theHiddenFilesBoxForTheTest(box) {
  said = { bottom: { show_hidden_files: { ticked: Boolean(box.ticked) } } };
  return theHiddenFilesBox(box);
}
export function theDeleteBoxForTheTest(reading) {
  boxes = document.body.appendChild(document.createElement("div"));
  theDeleteBoxIsDrawn(reading, []);
  return boxes;
}"#,
    ),
    ("the-rail.js", "export { screens as theScreensForTheTest };"),
    ("the-queue.js", ""),
    ("the-history.js", ""),
];

const PAGE: &str = r##"<!doctype html><html data-theme="light"><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
<style>body { margin: 0; padding: 16px; background: var(--bg); } #out { display: none; }
.modal { position: static !important; inset: auto !important; background: none !important; display: block !important; }
.fixture { margin: 0 0 18px; } .fixture > h6 { margin: 0 0 6px; font: 600 11px system-ui; color: #888; }
.pictured { display: grid; grid-template-columns: repeat(2, 1fr); gap: 14px 20px; max-width: 1060px; }
.pictured .fixture { margin: 0; padding: 10px 14px 14px; border: 1px solid var(--soft); border-radius: 6px; background: var(--bg); }
.pictured .fixture > h6 { font-size: 12px; margin-bottom: 10px; }</style>
</head><body><div id="pane"></div><div id="fixtures"></div><pre id="out"></pre>
<script>
window.asked = [];
window.__TAURI__ = {core:{invoke:async (command, args) => {
  asked.push(command);
  if (command === 'the_menu_bar') return {drawn_by_the_application:false};
  if (command === 'the_rail') return {opens_at:'nowhere', the_rails_own_name:'Places', the_servers_heading:'Servers', places:[], servers:[]};
  if (command === 'the_pane_layout') return {floor:{said:'', how:''}, the_record_would_not_answer:false,
    seams:[{key:'rail', property:'--rail-w', smallest:150, largest:340, from_right:false, label:'Resize the sidebar', hint:''},
      {key:'list', property:'--list-w', smallest:280, largest:620, from_right:false, label:'Resize the list', hint:''},
      {key:'preview', property:'--side-w', smallest:180, largest:380, from_right:true, label:'Resize the preview', hint:''}]};
  return null;
}}, event:{listen:async () => () => {}}};
window.addEventListener('error', event => { document.getElementById('out').textContent = JSON.stringify({error: event.message}); });
window.addEventListener('unhandledrejection', event => { document.getElementById('out').textContent = JSON.stringify({error: String(event.reason && event.reason.stack || event.reason)}); });
</script><script type="module">
import {aTickBox, theRemovalBoxIsDrawn} from './servers.js';
import {theSceneBox, theEngine, theRendersOn, theDependencyBand} from './render-settings.js';
import {theKeyTypesForTheTest} from './keys.js';
import {theInstallBoxForTheTest, theAdminBoxForTheTest} from './set-up.js';
import {theTelegramSectionIsBuilt} from './telegram.js';
import {theFormatBoxIsDrawn} from './transfers.js';
import {theHiddenFilesBoxForTheTest, theDeleteBoxForTheTest} from './files.js';
import {theScreensForTheTest} from './the-rail.js';
import './the-queue.js';
import './the-history.js';
const out = document.getElementById('out');
const fixtures = document.getElementById('fixtures');
const picture = 'PICTURE' === 'yes';
try {
  await new Promise((done) => setTimeout(done, 30));
  const faults = [];
  let read = 0;
  // **The accent, as the browser resolves it**, to compare a ring's colour with.
  const probe = document.body.appendChild(document.createElement('i'));
  probe.style.color = 'var(--accent)';
  const accent = getComputedStyle(probe).color;
  probe.remove();
  const aFixture = (name) => {
    const holder = fixtures.appendChild(document.createElement('div'));
    holder.className = 'fixture';
    holder.appendChild(document.createElement('h6')).textContent = name;
    return holder;
  };
  // **A ring, as the computed style draws it**: an outline, or a shadow ring.
  // `auto` is the browser's own ring, which is not the app's.
  const ringOf = (one) => {
    const style = getComputedStyle(one);
    const outline = style.outlineStyle !== 'none' && parseFloat(style.outlineWidth) > 0;
    const shadow = style.boxShadow && style.boxShadow !== 'none';
    return {
      any: outline || shadow,
      ours: (outline && style.outlineStyle === 'solid' && style.outlineColor === accent) || (shadow && style.boxShadow.includes(accent)),
      said: outline ? style.outlineStyle + ' ' + style.outlineWidth + ' ' + style.outlineColor : shadow ? style.boxShadow : 'none',
    };
  };
  const stateOf = (line, mark) => JSON.stringify([line.getAttribute('aria-checked'), line.getAttribute('aria-pressed'),
    line.className, mark.className, mark.textContent, asked.length]);
  const space = (one) => one.dispatchEvent(new KeyboardEvent('keydown', {key: ' ', code: 'Space', bubbles: true, cancelable: true}));
  // **A tick or a dot with its words beside it.** The line is the Tab stop;
  // its ring is on the mark alone; Space presses it.
  const aMarkedControl = (where, line, mark) => {
    read += 1;
    if (!line) { faults.push(where + ': not drawn'); return; }
    if (!mark) { faults.push(where + ': no tick or dot drawn'); return; }
    if (picture) return;
    if (!(line.tabIndex >= 0)) faults.push(where + ': not a Tab stop');
    if (line.tagName !== 'BUTTON' && !line.getAttribute('role')) faults.push(where + ': no role, so nothing reading the window knows it is a tick or a choice');
    line.focus();
    if (document.activeElement !== line) { faults.push(where + ': cannot take the focus'); return; }
    const onLine = ringOf(line), onMark = ringOf(mark);
    if (onLine.any) faults.push(where + ': the ring goes round the line (' + onLine.said + ')');
    if (!onMark.ours) faults.push(where + ': the tick or dot carries no accent ring (' + onMark.said + ')');
    // **Space presses it.** A native button answers Space itself, and a
    // script's key does not reach that, so its click is what is read.
    if (line.tagName === 'BUTTON') return;
    // A choice already picked stays picked; Space on it changes nothing.
    if (line.getAttribute('role') === 'radio' && line.getAttribute('aria-checked') === 'true') return;
    const before = stateOf(line, mark);
    space(line);
    if (stateOf(line, mark) === before) faults.push(where + ': Space does nothing');
  };
  // **A control whose words are the control**: its own accent ring, never the
  // browser's.
  const aPlainControl = (where, one) => {
    read += 1;
    if (picture) return;
    one.focus();
    if (document.activeElement !== one) { faults.push(where + ': cannot take the focus'); return; }
    const ring = ringOf(one);
    if (!ring.ours) faults.push(where + ': no accent ring of its own (' + ring.said + ')');
  };
  // **A field**: the one-pixel accent border, never the system's glow, on
  // the field or on the holder that draws its edge.
  const aField = (where, field, edge) => {
    read += 1;
    if (picture) return;
    field.focus();
    if (document.activeElement !== field) { faults.push(where + ': cannot take the focus'); return; }
    const glow = ringOf(field);
    if (glow.any) faults.push(where + ': the system ring or glow (' + glow.said + ')');
    const style = getComputedStyle(edge || field);
    if (style.borderTopColor !== accent || !(parseFloat(style.borderTopWidth) > 0)) faults.push(where + ': no accent border (' + style.borderTopWidth + ' ' + style.borderTopColor + ')');
  };

  // ── Servers ──────────────────────────────────────────────────────────────
  {
    const holder = aFixture('Servers, Add a server');
    const frm = holder.appendChild(document.createElement('div'));
    frm.className = 'mbox';
    for (const label of ['Get the server ready to render', 'Install Blender']) {
      const tick = aTickBox(label);
      frm.appendChild(tick.box);
      aMarkedControl('Servers, Add a server, ' + label, tick.box, tick.box.querySelector('i'));
    }
  }
  {
    theRemovalBoxIsDrawn('s1', {heading:{lead:'Remove 4000ada?'}, said:'It is removed from Smoozy.',
      leave_it_running:{head:'Leave it running', said:{lead:'The job carries on.'}},
      the_switch_off:{label:"Don't ask again", what_it_covers:'Only this box.', already:false}, cancel:'Cancel'});
    const box = document.querySelector('.svdialogs .mbox');
    const holder = aFixture('Servers, Remove');
    holder.appendChild(document.querySelector('.svdialogs'));
    const line = box && box.querySelector('.svoff label');
    aMarkedControl('Servers, Remove, the switch-off', line, line && line.querySelector('.cb'));
    if (box) aPlainControl('Servers, Remove, a choice', box.querySelector('.choice'));
  }

  // ── Render Settings ──────────────────────────────────────────────────────
  {
    const holder = aFixture('Render Settings');
    const sheet = holder.appendChild(document.createElement('div'));
    sheet.className = 'sheet';
    const boxes = sheet.appendChild(document.createElement('div'));
    boxes.className = 'boxes';
    for (const [name, extra] of [['Scene', ['pinned']], ['All scenes', ['allsc']], ['ViewLayer', []]]) {
      const box = theSceneBox(name, false, extra, () => window.__TAURI__.core.invoke('a_box'), 'scene-' + name);
      boxes.appendChild(box);
      aMarkedControl('Render Settings, the box ' + name, box, box.querySelector('i'));
    }
    const engine = sheet.appendChild(theEngine('s1', {engine:{label:'Engine', cycles:'Cycles', eevee:'EEVEE', on:'cycles', the_files_own:'Cycles', note:''}}));
    for (const one of engine.querySelectorAll('.rad[data-box]')) aMarkedControl('Render Settings, the engine ' + one.textContent, one, one.querySelector(':scope > i'));
    const device = sheet.appendChild(theRendersOn('s1', {renders_on:{label:'Renders on', gpu:'GPU', cpu:'CPU', on:'the-cards', tag:[]}}));
    for (const one of device.querySelectorAll('.rad[data-box]')) aMarkedControl('Render Settings, the device ' + one.textContent, one, one.querySelector(':scope > i'));
    const band = sheet.appendChild(theDependencyBand('s1', {file_bytes:'f', the_dependency_band:{which:'missing', said:[{lead:'2\u00a0files are missing.'}],
      the_way_to_the_check:'Check dependencies…', submit_anyway:'Submit anyway', switch_it_off:"Don't warn for this file", what_the_switch_covers:'Only this file.'}}));
    const off = band.querySelector('.off');
    aMarkedControl('Render Settings, the missing-files band, the switch-off', off, off && off.querySelector('.cb'));
  }

  // ── Keys ─────────────────────────────────────────────────────────────────
  {
    const root = theKeyTypesForTheTest({newKeyHeading:'New key', typeCount:'2', type0:'Ed25519', type0IsTheDefault:'yes', theDefaultMark:'default', type1:'RSA 4096'});
    const holder = aFixture('Keys, New key');
    holder.appendChild(root);
    for (const hidden of root.querySelectorAll('[hidden]')) hidden.hidden = false;
    for (const one of root.querySelectorAll('.rads .rad')) aMarkedControl('Keys, New key, the type ' + one.textContent, one, one.querySelector(':scope > i'));
  }

  // ── Set up ───────────────────────────────────────────────────────────────
  {
    const box = theInstallBoxForTheTest({title:'Install Blender', source:'From blender.org', finished:[{version:'5.2.3', line:'Released'}, {version:'4.5.6', line:'LTS'}],
      unfinished:[], the_heading:'', or:'Or', install:'Install', cancel:'Cancel'});
    aFixture('Set up, Install Blender').appendChild(box.parentNode);
    for (const one of box.querySelectorAll('.fmt')) aMarkedControl('Set up, the Blender ' + one.querySelector('.fmn').textContent, one, one.querySelector('.cb'));
  }
  {
    const held = theAdminBoxForTheTest({title:'Admin access', which_step:{lead:'Installing the driver'},
      choices:[{which:'sudo', name:'Type the password', said:'Each time', chosen:true}, {which:'keep', name:'Keep it', said:'On this computer'}], cancel:'Cancel'});
    aFixture('Set up, the password box').appendChild(held);
    for (const one of held.querySelectorAll('.fmt')) aMarkedControl('Set up, the password box, ' + one.querySelector('.fmn').textContent, one, one.querySelector('.rb'));
  }

  // ── Settings, Telegram ───────────────────────────────────────────────────
  {
    const section = aFixture('Settings, Telegram').appendChild(document.createElement('section'));
    theTelegramSectionIsBuilt(section, section);
    const rows = section.querySelectorAll('[role="radiogroup"] .fmt');
    for (const hidden of section.querySelectorAll('[hidden]')) hidden.hidden = false;
    rows.forEach((one, at) => {
      one.dataset.choice = 'choice-' + at;
      one.querySelector('.fmn').firstChild.textContent = ['Everything', 'Pictures only', 'A picture every 5\u00a0minutes', 'Alerts only', 'Nothing'][at] || '';
      aMarkedControl('Settings, Telegram, the default ' + at, one, one.querySelector('.rb'));
    });
  }

  // ── Transfers ────────────────────────────────────────────────────────────
  {
    const pieces = (said) => ({pieces:[{said}]});
    theFormatBoxIsDrawn({is:'the-box', heading:'Download frames', to:'To', where_downloads_go:'Download folder',
      reading:{reads:'several-formats', intro:'This job writes two formats.', total:pieces('64\u00a0files'), the_rest:'in all',
        formats:[{name:'png', said:'PNG', chosen:true, the_top:'top of the job folder', files:pieces('61\u00a0files')},
          {name:'exr', said:'EXR', chosen:false, where_in_the_job:'exr/', files:pieces('3\u00a0files')}]},
      folders:[{said:{pieces:[{value:'~/Renders/cloth'}]}}, {said:{pieces:[{value:'~/Downloads/cloth'}]}}],
      somewhere_else:'Other…', cancel:'Cancel', done:'Done'}, () => {}, true);
    const box = Array.from(document.querySelectorAll('body > .modal')).pop();
    aFixture('Transfers, the format box').appendChild(box);
    for (const one of box.querySelectorAll('.transfers-formats .fmt')) aMarkedControl('Transfers, the format ' + one.querySelector('.fmn').textContent, one, one.querySelector('.cb'));
    // The last row opens the system's own panel, so only the folders offered are pressed here.
    for (const one of Array.from(box.querySelectorAll('.transfers-folders .fmt')).slice(0, 2)) aMarkedControl('Transfers, the folder ' + one.textContent, one, one.querySelector('.rb'));
  }

  // ── Files ────────────────────────────────────────────────────────────────
  {
    const holder = aFixture('Files');
    const tick = holder.appendChild(theHiddenFilesBoxForTheTest({ticked:false, label:'Show hidden files'}));
    aMarkedControl('Files, Show hidden files', tick, tick.querySelector('.cb'));
    const box = theDeleteBoxForTheTest({question:{pieces:[{said:'Delete'}, {value:'cloth.blend'}, {said:'?', joins:true}]}, permanent:'This cannot be undone.',
      dont_warn_again:"Don't warn again", what_it_does_not_cover:'Live jobs still ask.', the_permanence_warning_is_switched_off:false,
      cancel:'Cancel', delete:'Delete'});
    aFixture('Files, Delete').appendChild(box);
    const line = box.querySelector('.mcheck');
    aMarkedControl("Files, Delete, Don't warn again", line, line && line.querySelector('.cb'));
  }

  // ── The controls that are their own words, the rows and the fields ───────
  {
    const holder = aFixture('Links, crosses, rows and fields');
    const make = (html) => { const one = document.createElement('div'); one.innerHTML = html; holder.appendChild(one); return one.firstElementChild; };
    for (const [where, html] of [
      ['Queue, Cancel all', '<button type="button" class="glnk">Cancel all</button>'],
      ['the check, a link', '<button type="button" class="lnk">Show the folder</button>'],
      ["the server's output, a tab", '<button type="button" class="ltab">Blender</button>'],
      ['a cross', '<button type="button" class="fx">×</button>'],
      ["a row's ⋯", '<button type="button" class="more">⋯</button>'],
      ["a server's ⋯", '<button type="button" class="svmore">⋯</button>'],
      ['Undo', '<button type="button" class="undo">Undo</button>'],
      ["a log's button", '<div class="log"><button type="button">Copy</button></div>'],
      ['a colour swatch', '<button type="button" class="sw">A</button>'],
      ['a link to a place', '<a href="#">Open the job</a>'],
      ['Servers, a row', '<div class="svrow" tabindex="0">4000ada</div>'],
      ['a button', '<button type="button" class="btn">Download all</button>'],
      ['a small button', '<button type="button" class="mini">Blender output</button>'],
      ['a switch', '<button type="button" class="sws"><i></i></button>'],
    ]) {
      const one = make(html);
      aPlainControl(where, one.matches('.log') ? one.querySelector('button') : one);
    }
    for (const [where, html] of [
      ['a field', '<input type="text" class="fld" value="57">'],
      ["Settings' figure", '<div class="stgs"><input type="text" class="num" value="20"></div>'],
      ["the job's name", '<div class="sfoot"><input type="text" class="fake" value="cloth"></div>'],
      ['a typed setting', '<div class="set typed"><input type="text" class="fld" value="128"></div>'],
    ]) {
      const one = make(html);
      const field = one.matches('input') ? one : one.querySelector('input');
      aField(where, field);
    }
    const name = make('<div class="frm"><span class="fld wide svnamefield"><input type="text" value="4000ada"></span></div>');
    aField('Servers, Add a server, Name', name.querySelector('input'), name.querySelector('.svnamefield'));
  }

  // ── The drag handles ─────────────────────────────────────────────────────
  {
    const pane = document.getElementById('pane');
    for (const place of ['queueview', 'historyview']) {
      const holder = pane.appendChild(document.createElement('div'));
      theScreensForTheTest.get(place).itIsDrawn(holder);
    }
    await new Promise((done) => setTimeout(done, 30));
    const seams = Array.from(document.querySelectorAll('.seam'));
    if (seams.length < 3) faults.push('the drag handles: ' + seams.length + ' drawn, the rail\'s, the queue\'s and the history\'s expected');
    for (const seam of seams) {
      read += 1;
      if (seam.tabIndex !== -1) faults.push('the ' + seam.dataset.seam + ' drag handle is a Tab stop');
    }
    if (!picture) pane.hidden = true;
  }

  if (picture) {
    // **One control of each kind, with the ring on it.** The focus is on one
    // element at a time, so each is focused in turn and the ring it showed is
    // kept on it, then the control is moved, with its own holder, onto one
    // board. A control that cannot take the focus shows no ring, which is
    // what the keyboard finds there.
    document.getElementById('pane').hidden = true;
    const kinds = [
      ['A tick and its words · Servers, Add a server', fixtures.querySelector('.box.plain'), (one) => one],
      ['A tick and its words · Render Settings, a scene', fixtures.querySelector('.sheet .boxes .box'), (one) => one.parentNode],
      ['A radio and its words · Render Settings, the engine', fixtures.querySelector('.sheet .rad[data-box="engine-cycles"]'), (one) => one.parentNode],
      ['A radio and its words · Keys, New key', fixtures.querySelector('.rads button.rad'), (one) => one.parentNode],
      ['A choice card · Set up, the password box', fixtures.querySelector('.fmt.choice3:not(.on)'), (one) => one.parentNode],
      ['A tick card · Transfers, the format box', fixtures.querySelector('.transfers-formats .fmt'), (one) => one.parentNode],
      ['A tick and its words · Files, Show hidden files', fixtures.querySelector('.mcheck.fhid'), (one) => one],
      ['A switch', fixtures.querySelector('button.sws'), (one) => one],
      ['A button', fixtures.querySelector('button.btn'), (one) => one],
      ['A text field', fixtures.querySelector('input.fld'), (one) => one],
    ];
    for (const [, one] of kinds) {
      if (!one) continue;
      one.focus();
      if (document.activeElement !== one) continue;
      for (const node of [one, ...one.querySelectorAll('*')]) {
        const style = getComputedStyle(node);
        node.style.outline = style.outline;
        node.style.outlineOffset = style.outlineOffset;
        node.style.boxShadow = style.boxShadow;
        node.style.borderColor = style.borderColor;
      }
    }
    if (document.activeElement) document.activeElement.blur();
    const board = document.body.insertBefore(document.createElement('div'), fixtures);
    board.className = 'pictured';
    for (const [title, one, holderOf] of kinds) {
      const card = board.appendChild(document.createElement('div'));
      card.className = 'fixture';
      card.appendChild(document.createElement('h6')).textContent = title;
      if (one) card.appendChild(holderOf(one));
      else card.appendChild(document.createElement('p')).textContent = 'not drawn';
    }
    fixtures.hidden = true;
  }
  out.textContent = JSON.stringify({faults, read});
} catch (error) {
  out.textContent = JSON.stringify({error: String(error && error.stack || error)});
}
</script></body></html>"##;

/// **What the Tab page adds to the copies, beside [`THE_EXPORTS`]**: the
/// builders a box needs that the module keeps to itself, each holder made in
/// the module's own scope as the app makes it.
const THE_TAB_EXPORTS: [(&str, &str); 6] = [
    ("servers.js", "export { theAddDialogIsOpened };"),
    (
        "keys.js",
        r#"export function theKeysBoxesForTheTest(holder) {
  screen = { host: holder, boxes: theBoxesAreBuilt(holder) };
  return theNewKeyDialogIsOpened;
}"#,
    ),
    ("transfers.js", "export { theStopBoxIsOpened };"),
    (
        "files.js",
        r#"export function theViewerForTheTest(words) {
  said = { viewer: words };
  theViewer = theViewerIsMade(words);
  theViewer.close.focus();
  return theViewer.panel;
}"#,
    ),
    (
        "set-up.js",
        r#"export function theQuestionBoxForTheTest(warning) {
  boxes = document.getElementById('stage').appendChild(document.createElement("div"));
  theBelowTheFloorWarningIsDrawn(warning, "s1");
  return boxes;
}
export function theInstallBoxIsOpenedForTheTest() {
  boxes = document.getElementById('stage').appendChild(document.createElement("div"));
  theInstallBoxIsOpened("s1");
  return boxes;
}"#,
    ),
    (
        "the-dependency-check.js",
        r#"export function theCheckForTheTest(holder) {
  const sheet = holder.appendChild(document.createElement("div"));
  sheet.className = "sheet";
  sheet.style.position = "relative";
  sheet.style.minHeight = "420px";
  const body = sheet.appendChild(document.createElement("div"));
  const held = { body, boxes: [] };
  panels.set("s1", held);
  return held;
}
export { theRowsMenuIsOpened, theOverwriteIsDrawn, theFolderBoxIsDrawn };"#,
    ),
];

const THE_TAB_PAGE: &str = r##"<!doctype html><html data-theme="light"><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
<style>html, body { margin: 0; height: 100%; background: var(--bg); } #out { display: none; }
.walk.app { display: grid !important; grid-template-columns: 190px 1fr !important; grid-template-rows: 100vh !important; height: 100vh !important; width: 100vw !important; max-width: none !important; border: 0 !important; border-radius: 0 !important; }
.walk-rail { display: flex; flex-direction: column; gap: 4px; padding: 14px 10px; border-right: 1px solid var(--line); background: var(--bg2, var(--bg)); }
.walk-rail button { text-align: left; font: 500 13px system-ui; padding: 7px 10px; border: 0; border-radius: 6px; background: none; color: var(--ink); }
.walk-rail .walk-head { font: 600 11px system-ui; color: var(--ink3, #888); padding: 6px 10px 2px; }
#stage { position: relative; overflow: hidden; padding: 22px 28px; height: 100vh; box-sizing: border-box; }
#stage > h1 { margin: 0 0 6px; font: 640 20px system-ui; color: var(--ink); }
#stage > p { margin: 0; font: 13px system-ui; color: var(--ink2, #555); }
.walk-said { position: absolute; left: 0; right: 0; bottom: 0; z-index: 90; padding: 10px 16px; font: 600 13px system-ui; color: var(--ink); background: var(--bg); border-top: 1px solid var(--line); }
.walk-ring { outline: 3px solid var(--accent) !important; outline-offset: 2px !important; }</style>
</head><body><div class="walk app">
<nav class="walk-rail" aria-label="Places"><span class="walk-head">Servers</span><button type="button" class="walk-out">4000ada</button><span class="walk-head">Places</span><button type="button" class="walk-out">Queue</button><button type="button" class="walk-out">History</button><button type="button" class="walk-out">Transfers</button><button type="button" class="walk-out">Keys</button><button type="button" class="walk-out">Settings</button></nav>
<main id="stage"><h1 id="stage-heading"></h1><p id="stage-words"></p><button type="button" id="opener" class="btn">Open</button><div id="pane"><div class="body"><div class="main"></div></div></div><div id="holders"></div></main>
</div><button type="button" class="walk-out" id="after">After the window</button><pre id="out"></pre>
<script>
window.asked = [];
window.delivered = null;
window.__TAURI__ = {core:{invoke:async (command, args) => {
  asked.push(command);
  if (command === 'the_menu_bar') return {drawn_by_the_application:false};
  if (command === 'the_rail') return {opens_at:'nowhere', the_rails_own_name:'Places', the_servers_heading:'Servers', places:[], servers:[]};
  if (command === 'the_pane_layout') return {floor:{said:'', how:''}, the_record_would_not_answer:false, seams:[]};
  if (command === 'the_add_dialog') return ADD_VALUE;
  if (command === 'the_install_list') return {title:'Install Blender', source:'From blender.org', finished:[{version:'5.2.3', line:'Released'}, {version:'4.5.6', line:'LTS'}], unfinished:[], the_heading:'', or:'Or', install:'Install', cancel:'Cancel'};
  if (command === 'the_stop_box') return {is:'the-box', heading:'Stop this transfer?', bullets:[{bold:{lead:'3\u00a0files'}, rest:'already downloaded stay.'}], nothing_is_thrown_away:'Nothing is deleted.', keep:'Keep', stop_it:'Stop'};
  return null;
}}, event:{listen:async (name, handler) => { window.delivered = handler; return () => {}; }}};
window.addEventListener('error', event => { document.getElementById('out').textContent = JSON.stringify({error: event.message + ' ' + (event.error && event.error.stack)}); });
window.addEventListener('unhandledrejection', event => { document.getElementById('out').textContent = JSON.stringify({error: String(event.reason && event.reason.stack || event.reason)}); });
</script><script type="module">
import {theRouteIsOpen} from './event-route.js';
import './menus.js';
import './reporting.js';
import './the-escape-key.js';
import './the-tab-key.js';
import {theKeysBoxesForTheTest} from './keys.js';
import {theDeleteBoxForTheTest, theViewerForTheTest} from './files.js';
import {theAddDialogIsOpened} from './servers.js';
import {theFormatBoxIsDrawn, theStopBoxIsOpened} from './transfers.js';
import {theBoxIsDrawn} from './the-queue.js';
import {theInstallBoxIsOpenedForTheTest, theAdminBoxForTheTest, theQuestionBoxForTheTest} from './set-up.js';
import {theCheckForTheTest, theRowsMenuIsOpened, theOverwriteIsDrawn, theFolderBoxIsDrawn} from './the-dependency-check.js';
import {theTelegramSectionIsBuilt} from './telegram.js';
import {theWordsPanel} from './the-machines-own-words.js';
const out = document.getElementById('out');
const picture = 'PICTURE';
const faults = [];
const walked = [];
const check = (ok, why) => { if (!ok) faults.push(why); };
THE_TAB_PRESS
const settle = async () => { for (let i = 0; i < 12; i++) await Promise.resolve(); await new Promise((done) => setTimeout(done, 0)); for (let i = 0; i < 6; i++) await Promise.resolve(); };
let sequence = 0;
const deliver = (kind, said) => window.delivered({payload: {sequence: ++sequence, kind, said}});
const holders = document.getElementById('holders');
const aHolder = () => { holders.replaceChildren(); return holders.appendChild(document.createElement('div')); };
const opener = document.getElementById('opener');
const theShownBox = () => { const open = [...document.querySelectorAll('.modal, .menus')].filter(theTabStopIsShown); return open.length ? open[open.length - 1] : null; };
const escape = () => (document.activeElement || document.body).dispatchEvent(new KeyboardEvent('keydown', {key: 'Escape', bubbles: true, cancelable: true}));
// **One box**: opened from the focused opener; walked; closed by Escape where
// it closes on Escape, and the focus read back on the opener; then taken away
// whatever happened, so the next box starts from an empty window.
const aBoxIsWalked = async (name, open, {escapes = true, gives = true} = {}) => {
  opener.focus();
  let box;
  try { box = await open(); } catch (trouble) { check(false, name + ': it could not be opened: ' + (trouble && trouble.stack || trouble)); return; }
  await settle();
  box = box || theShownBox();
  if (!box || !theTabStopIsShown(box)) { check(false, name + ': nothing was drawn'); return; }
  walked.push(name);
  theBoxIsWalkedWithTab(name, box, check);
  if (escapes) {
    escape();
    await settle();
    const still = theTabStopIsShown(box);
    check(!still, name + ': Escape did not close it');
    if (!still && gives) check(document.activeElement === opener, name + ': the focus did not come back to what opened it (it is on ' + theNameOf(document.activeElement) + ')');
  }
  for (const one of document.querySelectorAll('.modal, .menus')) if (theTabStopIsShown(one)) { one.hidden = true; }
  await settle();
};
// **The picture**: the box open, Tab pressed past its last control, and the
// control the focus landed on ringed, wherever it is.
const thePictureIsTaken = async (heading, words, open) => {
  document.getElementById('stage-heading').textContent = heading;
  document.getElementById('stage-words').textContent = words;
  opener.hidden = true;
  // Past the window's last control, Tab goes round to its first: the rail.
  document.getElementById('after').hidden = true;
  let box = await open();
  await settle();
  box = box || theShownBox();
  const stops = theTabStopsIn(box);
  for (let at = 0; at <= stops.length; at++) theTabIsPressed(false);
  const on = document.activeElement;
  if (on && on !== document.body) on.classList.add('walk-ring');
  const said = document.body.appendChild(document.createElement('div'));
  said.className = 'walk-said';
  said.textContent = 'Tab pressed ' + (stops.length + 1) + ' times from where the box put the focus. The focus is on ' + theNameOf(on) + (box.contains(on) ? ', inside the box.' : ', outside the box.');
  on && on.blur && on.blur();
  if (on) on.classList.add('walk-ring');
};
try {
  await theRouteIsOpen();
  await settle();
  const keysHolder = aHolder();
  const theNewKeyIsOpened = theKeysBoxesForTheTest(keysHolder);
  const queueBox = (into) => theBoxIsDrawn(into, {boxWhich: 'stop', boxHeadCount: '1', boxHead0Lead: 'Stop', boxHead0Value: 'cloth_v3', boxHead0Trail: '?', boxHead0TrailClings: 'yes', boxSaidCount: '1', boxSaid0PartCount: '1', boxSaid0Part0Lead: 'Frames rendered so far stay on the server.', boxKeep: 'Keep', boxGoCount: '1', boxGo0Lead: 'Stop', entryJob: 'j1'});
  if (picture === 'servers') {
    // **The Creator's own case.** The app draws this box over the All servers
    // screen, beside the rail; it is moved there, and its first field given
    // the focus again, as the box gives it.
    await thePictureIsTaken('All servers', 'Add a server is open; Tab is pressed once more than it has controls.', async () => {
      theAddDialogIsOpened();
      await settle();
      const held = document.querySelector('.svdialogs');
      held.style.position = 'absolute';
      held.style.inset = '0';
      document.getElementById('stage').appendChild(held);
      const field = held.querySelector('input');
      if (field) field.focus();
      return held.querySelector('.modal');
    });
  } else if (picture === 'keys') {
    await thePictureIsTaken('Keys', 'New key is open; Tab is pressed once more than it has controls.', async () => { theNewKeyIsOpened(); return keysHolder.querySelector('.modal:not([hidden])'); });
  } else if (picture === 'queue') {
    await thePictureIsTaken('Queue', 'The Stop box is open; Tab is pressed once more than it has controls.', async () => { const into = aHolder(); into.style.position = 'absolute'; into.style.inset = '0'; queueBox(into); return into.querySelector('.modal'); });
  } else {
  // ── Menus: About, the update box, the rollback box ──
  await aBoxIsWalked('About', () => { deliver('menus:a-going', {item: 'smoozy.about', heading: 'Smoozy', beforeTheVersion: 'Version', version: '0.3.1', reading: 'idle', upToDate: 'Up to date, checked', lastChecked: 1756129920, copyright: '© 2026', freeSoftware: 'Free and open source.', licence: 'Licence', thirdParty: 'Third-party licences', sourceCode: 'Source code', close: 'Close', readingNumber: '1'}); });
  await aBoxIsWalked('The update box', () => { deliver('menus:update-box', {version: '0.4.0', beforeVersion: 'Smoozy', afterVersion: 'is ready to install', whatsNew: "What's new", restarts: 'Installing restarts Smoozy.', skip: 'Skip this version', install: 'Install now'}); });
  await aBoxIsWalked('The rollback box', () => { deliver('menus:linux-rollback', {beforeVersion: 'Smoozy', version: '0.3.0', afterVersion: 'could not start', password: 'Your system asks for your computer password to reinstall.', quit: 'Quit', reinstall: 'Reinstall'}); }, {escapes: false});
  // ── Reporting: the report box and the crash offer ──
  await aBoxIsWalked('Report a problem', () => { deliver('reporting:the-report-box', {reading: 'asking', opening: 1, heading: 'Report a problem', whatWentWrong: 'What went wrong?', save: 'Save report', cancel: 'Cancel', close: 'Close'}); });
  await aBoxIsWalked('The crash offer', () => { deliver('reporting:the-crash-box', {heading: 'Smoozy quit unexpectedly', nothingWasLost: 'Nothing was lost.', ifYouReportIt: 'A report helps.', notNow: 'Not now', reportIt: 'Report it…'}); }, {gives: false});
  // ── Keys: New key, a refusal, the same name, taking a key away, the passphrase ──
  await aBoxIsWalked('Keys, New key', async () => { theNewKeyIsOpened(); return keysHolder.querySelector('.modal:not([hidden])'); }, {escapes: true});
  // A refusal and the same-name statement end on a plain control, which
  // Escape does not press (`./the-escape-key.js`, *a box with no step-back
  // control stays*).
  await aBoxIsWalked('Keys, a refusal', () => { deliver('keys:an-answer', {answer: 'a-point-was-refused', heading: 'That file is not a key', thePickedPath: '/Users/a/notes.txt', back: 'Back', whatIsWrong: 'It holds no private key.'}); }, {escapes: false});
  await aBoxIsWalked('Keys, the same name', () => { deliver('keys:an-answer', {answer: 'the-same-name', anotherKeyIsAlreadyCalled: 'Another key is already called', theName: 'id_ed25519', theOneAlreadyInTheListHolds: 'It holds', theOtherFingerprint: 'SHA256:x', bothAreKept: 'Both are kept.', done: 'Done'}); }, {escapes: false});
  await aBoxIsWalked('Keys, taking a key away', () => { deliver('keys:an-answer', {answer: 'taking-a-key-away', itAsks: 'yes', row: 'r1', heading: 'Delete id_ed25519?', cancel: 'Cancel', theControl: 'Delete', managed: 'yes'}); });
  await aBoxIsWalked('Keys, the passphrase', () => { deliver('keys:the-passphrase-ask', {passphraseFor: 'Passphrase for', theKeyNamed: 'id_ed25519', connectingTo: 'Connecting to', theServerNamed: '4000ada', thePassphraseField: 'Passphrase', cancel: 'Cancel', connect: 'Connect', server: 's1'}); });
  // ── Files: Delete, the picture viewer ──
  await aBoxIsWalked('Files, Delete', async () => {
    const held = theDeleteBoxForTheTest({question: {pieces: [{said: 'Delete'}, {value: 'cloth.blend'}, {said: '?', joins: true}]}, permanent: 'This cannot be undone.', dont_warn_again: "Don't warn again", what_it_does_not_cover: 'Live jobs still ask.', the_permanence_warning_is_switched_off: false, cancel: 'Cancel', delete: 'Delete'});
    return held.querySelector('.modal');
  });
  // The viewer's own keys are started with Files' screen, which is not drawn here.
  await aBoxIsWalked('Files, the picture viewer', () => theViewerForTheTest({picture: 'Picture', download: 'Download', close: 'Close', previous: 'Previous', next: 'Next'}), {escapes: false});
  // ── Transfers: the format box, the stop box ──
  const pieces = (said) => ({pieces: [{said}]});
  await aBoxIsWalked('Transfers, the format box', () => {
    theFormatBoxIsDrawn({is: 'the-box', heading: 'Download frames', to: 'To', where_downloads_go: 'Download folder',
      reading: {reads: 'several-formats', intro: 'This job writes two formats.', total: pieces('64 files'), the_rest: 'in all',
        formats: [{name: 'png', said: 'PNG', chosen: true, the_top: 'top of the job folder', files: pieces('61 files')}, {name: 'exr', said: 'EXR', chosen: false, where_in_the_job: 'exr/', files: pieces('3 files')}]},
      folders: [{said: {pieces: [{value: '~/Renders/cloth'}]}}], somewhere_else: 'Other…', cancel: 'Cancel', done: 'Done'}, () => {}, true);
  });
  await aBoxIsWalked('Transfers, the stop box', () => theStopBoxIsOpened('r1'));
  // ── Queue and History: their one box ──
  await aBoxIsWalked('Queue and History, the Stop box', () => { const into = aHolder(); into.style.position = 'relative'; into.style.minHeight = '300px'; queueBox(into); return into.querySelector('.modal'); }, {escapes: true});
  // ── Set up: Install Blender, the password box, the question ──
  await aBoxIsWalked('Set up, Install Blender', () => theInstallBoxIsOpenedForTheTest().querySelector('.modal'));
  await aBoxIsWalked('Set up, the password box', () => theAdminBoxForTheTest({title: 'Admin access', which_step: {lead: 'Installing the driver'}, password: 'Password', choices: [{which: 'sudo', name: 'Type the password', said: 'Each time', chosen: true}, {which: 'keep', name: 'Keep it', said: 'On this computer'}], cancel: 'Cancel', try_again: 'Try again'}).querySelector('.modal'));
  await aBoxIsWalked('Set up, the question', () => theQuestionBoxForTheTest({title: {lead: 'Blender 4.2 is older than 4.5'}, supported: {lead: 'Supported from 4.5.'}, nothing_is_written_for: {lead: 'Nothing is written for older versions.'}, it_will_appear: 'It will appear in the list.', dont_install_it: "Don't install it", install_it_anyway: 'Install it anyway'}).querySelector('.modal'), {escapes: false});
  // ── The dependency check: a row's menu, the overwrite box, the search folder box ──
  {
    const held = theCheckForTheTest(aHolder());
    const from = held.body.appendChild(document.createElement('button'));
    from.type = 'button'; from.textContent = '⋯';
    await aBoxIsWalked('The check, a row\'s menu', () => { theRowsMenuIsOpened('s1', {}, {at: 0, more_items: [{act: 'locate', label: 'Locate…'}, {act: 'skip', label: 'Skip'}, {act: 'show', label: 'Show in Files'}]}, opener); return held.body.parentElement.querySelector('.menus'); });
    await aBoxIsWalked('The check, the overwrite box', () => { theOverwriteIsDrawn('s1', {the_overwrite: {title: 'Replace cloth.blend?', what_was_read: ['A file is already there.'], location: '/work/cloth.blend', stop: 'Stop', replace: 'Replace'}}, held); return held.boxes[held.boxes.length - 1]; }, {escapes: false});
    await aBoxIsWalked('The check, the search folder box', () => { theFolderBoxIsDrawn('s1', {the_folder_box: {title: 'Add a search folder', the_folders: ['/Volumes/Textures'], your_search_folders: 'Your search folders', change_them_in_settings: 'Change them in Settings', choose_a_folder: 'Choose…', chosen: '/Volumes/More', what_adding_does: [{lead: 'Adding it searches it now.'}], cancel: 'Cancel', add: 'Add'}}, held); return held.boxes[held.boxes.length - 1]; }, {gives: false});
  }
  // ── Telegram's set-up boxes. Escape asks the application to shut them, and
  // nothing answers here. ──
  {
    const section = aHolder().appendChild(document.createElement('section'));
    theTelegramSectionIsBuilt(section, section);
    await aBoxIsWalked('Telegram, paste the token', () => { deliver('telegram:the-walkthrough', {stage: 'paste', makeNew: 'Make a new bot', useExisting: 'Use one I have', go: 'Check', cancel: 'Cancel'}); return section.querySelector('#stgbot'); }, {escapes: false});
    await aBoxIsWalked('Telegram, replace the bot', () => { deliver('telegram:the-walkthrough', {stage: 'replace', save: 'Replace', cancel: 'Cancel'}); return section.querySelector('#botreplace'); }, {escapes: false});
    await aBoxIsWalked('Telegram, press Start', () => { deliver('telegram:the-walkthrough', {stage: 'waiting', theBotsName: 'smoozy_bot', waitingForStart: 'Waiting for Start…', cancel: 'Cancel', done: 'Done'}); return section.querySelector('#stgbotok'); }, {escapes: false});
  }
  // ── Server output and Blender output ──
  await aBoxIsWalked('Server output', () => {
    const panel = theWordsPanel({title: 'Server output', tabs: [{name: 'Output', said: 'Nothing printed yet.'}, {name: 'Errors', said: 'No errors.'}], copy: 'Copy', note: 'As printed.', close: 'Close'});
    document.getElementById('stage').appendChild(panel);
    panel.hidden = false;
    return panel;
  });
  }
  out.textContent = JSON.stringify({faults, walked});
} catch (error) {
  out.textContent = JSON.stringify({error: String(error && error.stack || error)});
}
</script></body></html>"##;

/// **The Creator's twenty-ninth notes, note 1: every focus move goes through
/// one way** — `Preparation/Design/Whole app.md`, *Keyboard shortcuts*,
/// *Wherever the keyboard moves the focus, the ring shows*. The account is
/// `Audits/Build/Twenty-ninth notes/Part A — what each one is.md`, note 1.
///
/// **The only test that can hold this rule.** The Mac's engine hides the ring
/// on a focus a page script moved after any click, and Chrome does not, so no
/// Chrome test sees the fault. What can be held is that no page script moves
/// the focus except through `web/the-focus-moves.js`, which says with each
/// move whether a key or the pointer came last; and that every script using
/// it imports it. Comment lines are not read.
#[test]
fn twenty_ninth_1_every_focus_move_goes_through_the_one_way() {
    let web = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web");
    let mut found = Vec::new();
    let mut unimported = Vec::new();
    let mut users = 0;
    let mut names: Vec<_> = fs::read_dir(&web)
        .expect("the page folder")
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().and_then(|one| one.to_str()) == Some("js"))
        .collect();
    names.sort();
    for path in names {
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if name == "the-focus-moves.js" {
            continue;
        }
        let text = fs::read_to_string(&path).unwrap();
        for at in the_direct_focus_moves_in(&text) {
            found.push(format!("web/{name}:{at}: {}", text.lines().nth(at - 1).unwrap_or_default().trim()));
        }
        let uses = text.lines().any(|line| line.contains("theFocusMovesTo(") && !line.trim_start().starts_with("import"));
        if uses {
            users += 1;
            if !text.contains("import { theFocusMovesTo } from \"./the-focus-moves.js\";") {
                unimported.push(format!("web/{name}"));
            }
        }
    }
    assert!(
        found.is_empty(),
        "{} focus moves do not go through the one way:\n{}",
        found.len(),
        found.join("\n")
    );
    assert!(unimported.is_empty(), "these use the one way without importing it: {unimported:?}");
    assert!(users >= 16, "the one way is used by every page script that moves the focus: {users}");
}

/// **Where a page script reaches for the focus itself**, by line, counting
/// from one: the name `focus` read as a member, whatever is around it — after
/// a dot (`.focus`, `?.focus`, `. focus`), or quoted inside square brackets
/// (`["focus"]`) — and never as part of a longer name. Comments are read too:
/// a page script has no need to write it anywhere.
fn the_direct_focus_moves_in(text: &str) -> Vec<usize> {
    let bytes = text.as_bytes();
    let is_name = |byte: u8| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$';
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(at) = text[from..].find("focus").map(|one| one + from) {
        from = at + 5;
        let after = bytes.get(at + 5).copied();
        let before = bytes.get(at.wrapping_sub(1)).copied();
        if after.is_some_and(is_name) || before.is_some_and(is_name) {
            continue;
        }
        let back = |mut i: usize| {
            while i > 0 && bytes[i - 1].is_ascii_whitespace() {
                i -= 1;
            }
            i
        };
        let a_member = {
            let i = back(at);
            i > 0 && bytes[i - 1] == b'.'
        };
        let a_quoted_member = matches!(before, Some(b'"' | b'\'' | b'`'))
            && bytes.get(at + 5) == before.as_ref()
            && {
                let i = back(at - 1);
                i > 0 && bytes[i - 1] == b'['
            };
        if a_member || a_quoted_member {
            let line = text[..at].matches('\n').count() + 1;
            if found.last() != Some(&line) {
                found.push(line);
            }
        }
    }
    found
}

/// **The guard's own reading finds every way of writing a focus move** — Sol's
/// fresh look at piece A1 found three it walked past: a space before the
/// brackets, the name in square brackets, and a call after a comment on the
/// same line. A comment that names the call is counted too: the page scripts
/// never need to write it, so the guard reads the whole text.
#[test]
fn twenty_ninth_1_the_guard_finds_every_way_of_writing_a_focus_move() {
    for line in [
        "button.focus();",
        "button?.focus();",
        "button.focus ();",
        "button . focus();",
        "button[\"focus\"]();",
        "button['focus']();",
        "/* restore it */ button.focus();",
        "button.focus.call(button);",
        "const move = button.focus;",
    ] {
        assert_eq!(the_direct_focus_moves_in(line), vec![1], "not found: {line}");
    }
    for line in ["box.focused = true;", "row.classList.add(\"focus-ring\");", "one.matches(':focus-visible');", "theFocusMovesTo(button);"] {
        assert!(the_direct_focus_moves_in(line).is_empty(), "found where there is none: {line}");
    }
}

/// **The Creator's twenty-ninth notes, notes 1 and 7, in a real browser** —
/// `Preparation/Design/Whole app.md`, *Keyboard shortcuts*: *Wherever the
/// keyboard moves the focus, the ring shows*, and *Tab into a text field
/// chooses its whole text*. The account is `Audits/Build/Twenty-ninth
/// notes/Part A — what each one is.md`, notes 1 and 7.
///
/// Read, with the page's own Tab rule and the real *Add a server* box:
/// - **Smoozy's Tab owns Tab on a screen with no box open**, and walks the
///   window's order;
/// - **a Tab or Shift+Tab into a one-line text field chooses its whole text**,
///   on the screen and in the box; a read-only field and a field of several
///   lines keep their caret;
/// - **each move says who came last**: after a pointer press the box's first
///   field is focused with the ring off, and after a key with it on.
///
/// **Chrome, not the Mac's own engine**, which only a desk program in the
/// system's WebKit can show; skipped, and says so, where Chrome is absent.
#[test]
fn twenty_ninth_1_and_7_tab_is_smoozys_everywhere_and_chooses_the_whole_text() {
    let add = serde_json::to_string(&crate::surface::servers::the_add_dialog_from(&[])).unwrap();
    let page = THE_WHOLE_TEXT_PAGE.replace("ADD_VALUE", &add);
    let Some(measured) = a_page_in_a_real_browser("whole-text", &page, &[&THE_TAB_EXPORTS], None) else {
        return;
    };
    let faults = measured["faults"].as_array().expect("the faults are listed");
    let read = measured["read"].as_u64().unwrap_or(0);
    eprintln!("{read} moves read");
    assert!(
        faults.is_empty(),
        "{} faults:\n{}",
        faults.len(),
        faults.iter().map(|one| one.as_str().unwrap_or_default().to_string()).collect::<Vec<_>>().join("\n")
    );
    assert!(read >= 12, "every move was read: {measured}");
}

const THE_WHOLE_TEXT_PAGE: &str = r##"<!doctype html><html data-theme="light"><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
<style>body { margin: 0; padding: 16px; background: var(--bg); } #out { display: none; }</style>
</head><body><main id="stage">
<button type="button" class="btn" id="first">First</button>
<input type="text" id="name" value="render-farm-01">
<input type="text" id="shown" readonly value="ssh-ed25519 AAAAexample">
<textarea id="lines">one
two</textarea>
<input type="password" id="secret" value="made-up-passphrase">
<button type="button" class="btn" id="last">Last</button>
<div id="pane"><div class="body"><div class="main"></div></div></div>
</main><pre id="out"></pre>
<script>
window.focusAsked = [];
const theOwnFocus = HTMLElement.prototype.focus;
HTMLElement.prototype.focus = function (options) { window.focusAsked.push({on: this, options: options || null}); return theOwnFocus.call(this, options); };
window.__TAURI__ = {core:{invoke:async (command) => {
  if (command === 'the_menu_bar') return {drawn_by_the_application:false};
  if (command === 'the_rail') return {opens_at:'nowhere', the_rails_own_name:'Places', the_servers_heading:'Servers', places:[], servers:[]};
  if (command === 'the_pane_layout') return {floor:{said:'', how:''}, the_record_would_not_answer:false, seams:[]};
  if (command === 'the_add_dialog') return ADD_VALUE;
  return null;
}}, event:{listen:async () => () => {}}};
window.addEventListener('error', event => { document.getElementById('out').textContent = JSON.stringify({error: event.message + ' ' + (event.error && event.error.stack)}); });
window.addEventListener('unhandledrejection', event => { document.getElementById('out').textContent = JSON.stringify({error: String(event.reason && event.reason.stack || event.reason)}); });
</script><script type="module">
import './the-escape-key.js';
import './the-tab-key.js';
import {theAddDialogIsOpened} from './servers.js';
const out = document.getElementById('out');
const faults = [];
let read = 0;
const check = (ok, why) => { read++; if (!ok) faults.push(why); };
const settle = async () => { for (let i = 0; i < 12; i++) await Promise.resolve(); await new Promise((done) => setTimeout(done, 0)); for (let i = 0; i < 6; i++) await Promise.resolve(); };
const nameOf = (one) => !one || one === document.body ? 'nothing' : '«' + (one.id || one.getAttribute('aria-label') || one.placeholder || one.textContent || one.className).trim().slice(0, 30) + '»';
// **A Tab as the window takes it.** A key a script sends is never moved by the
// browser, so a press nothing in the page took is a fault here: Smoozy's own
// Tab rule is to take every one.
const tab = (back) => {
  const from = document.activeElement || document.body;
  const press = new KeyboardEvent('keydown', {key: 'Tab', code: 'Tab', shiftKey: back, bubbles: true, cancelable: true});
  from.dispatchEvent(press);
  return press.defaultPrevented;
};
const wholeIsChosen = (field) => field.selectionStart === 0 && field.selectionEnd === field.value.length && field.value.length > 0;
const $ = (id) => document.getElementById(id);
try {
  await settle();
  // ── A screen with no box open ──
  $('name').setSelectionRange(3, 3);
  $('shown').setSelectionRange(2, 2);
  $('lines').setSelectionRange(1, 1);
  $('secret').setSelectionRange(4, 4);
  $('first').focus();
  check(tab(false), 'Tab on a screen with no box open was left to the browser');
  check(document.activeElement === $('name'), 'Tab from First landed on ' + nameOf(document.activeElement) + ', not the name field');
  check(wholeIsChosen($('name')), 'Tab into the name field did not choose its whole text: ' + $('name').selectionStart + '–' + $('name').selectionEnd);
  tab(false);
  check(document.activeElement === $('shown'), 'Tab landed on ' + nameOf(document.activeElement) + ', not the read-only field');
  check(!wholeIsChosen($('shown')), 'Tab into a read-only field chose its whole text');
  tab(false);
  check(document.activeElement === $('lines'), 'Tab landed on ' + nameOf(document.activeElement) + ', not the field of several lines');
  check(!wholeIsChosen($('lines')), 'Tab into a field of several lines chose its whole text');
  tab(false);
  check(document.activeElement === $('secret') && wholeIsChosen($('secret')), 'Tab into the password field did not choose its whole text (on ' + nameOf(document.activeElement) + ')');
  $('name').setSelectionRange(2, 2);
  tab(true); tab(true);
  check(tab(true), 'Shift+Tab on a screen with no box open was left to the browser');
  check(document.activeElement === $('name') && wholeIsChosen($('name')), 'Shift+Tab into the name field did not choose its whole text (on ' + nameOf(document.activeElement) + ')');
  // ── Add a server, opened by a pointer press: its first field is focused
  // with the ring off; every Tab after it with the ring on. ──
  window.dispatchEvent(new PointerEvent('pointerdown', {bubbles: true}));
  window.focusAsked.length = 0;
  theAddDialogIsOpened();
  await settle();
  const box = document.querySelector('.svdialogs .modal') || document.querySelector('.modal');
  check(box && box.contains(document.activeElement), 'Add a server did not take the focus when it opened');
  const opened = window.focusAsked.filter((one) => box && box.contains(one.on));
  check(opened.length > 0 && opened.every((one) => one.options && one.options.focusVisible === false), 'Add a server opened by the pointer did not say the ring is off: ' + JSON.stringify(opened.map((one) => one.options)));
  const fields = [...box.querySelectorAll('input')].filter((one) => one.type === 'text' || one.type === 'password' || one.type === '');
  let n = 0;
  for (const field of fields) { if (!field.readOnly) { field.value = field.value || ('made-up-' + (++n)); field.setSelectionRange(1, 1); } }
  window.focusAsked.length = 0;
  const reached = new Set();
  for (let at = 0; at < 30 && reached.size < fields.length; at++) {
    tab(false);
    const on = document.activeElement;
    if (fields.includes(on) && !on.readOnly && on.getClientRects().length) {
      reached.add(on);
      check(wholeIsChosen(on), 'Add a server: Tab into ' + nameOf(on) + ' did not choose its whole text');
    }
  }
  check(reached.size >= 3, 'Add a server: Tab reached ' + reached.size + ' text fields');
  check(window.focusAsked.length > 0 && window.focusAsked.every((one) => one.options && one.options.focusVisible === true), 'Tab in Add a server did not say the ring is on: ' + JSON.stringify(window.focusAsked.map((one) => one.options)));
  out.textContent = JSON.stringify({faults, read});
} catch (error) {
  out.textContent = JSON.stringify({error: String(error && error.stack || error)});
}
</script></body></html>"##;

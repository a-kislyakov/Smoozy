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
    let Some(chrome) = a_chrome() else {
        eprintln!("SKIPPED: Chrome is absent; the focus rings were not read");
        return None;
    };
    let folder = crate::scratch_folders::a_folder(format!(
        "smoozy-focus-{}-{}",
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
    // **Test-only exports on the temporary copies reach the actual builders**,
    // as `selecting_text.rs` does. The installed page gains no testing entry.
    for (file, exports) in THE_EXPORTS {
        let path = folder.join(file);
        let mut source = fs::read_to_string(&path).unwrap();
        source.push('\n');
        source.push_str(exports);
        fs::write(path, source).unwrap();
    }
    fs::write(
        folder.join("index.html"),
        PAGE.replace("PICTURE", if picture.is_some() { "yes" } else { "" }),
    )
    .unwrap();
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
    seams:[{key:'rail', property:'--rail-w', smallest:150, largest:340, from_right:false, label:'Resize the rail', hint:''},
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
    theFormatBoxIsDrawn({is:'the-box', heading:'Download frames', to:'To', where_downloads_go:'Where downloads go',
      reading:{reads:'several-formats', intro:'This job writes two formats.', total:pieces('64\u00a0files'), the_rest:'in all',
        formats:[{name:'png', said:'PNG', chosen:true, the_top:'top of the job folder', files:pieces('61\u00a0files')},
          {name:'exr', said:'EXR', chosen:false, where_in_the_job:'exr/', files:pieces('3\u00a0files')}]},
      folders:[{said:{pieces:[{value:'~/Renders/cloth'}]}}, {said:{pieces:[{value:'~/Downloads/cloth'}]}}],
      somewhere_else:'Somewhere else…', cancel:'Cancel', done:'Done'}, () => {}, true);
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

//! The eighteenth notes, note 2: measure selection with the app's own page
//! builders and complete stylesheet list. No app, server or clipboard is used.

use std::{fs, path::PathBuf, time::Duration};

#[test]
fn only_copyable_text_and_the_three_copy_boxes_are_selectable_on_the_page() {
    let chrome = [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        "/usr/bin/google-chrome",
        "/usr/bin/chromium",
        "/usr/bin/chromium-browser",
        "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
    ]
    .into_iter()
    .map(PathBuf::from)
    .find(|one| one.exists());
    let Some(chrome) = chrome else {
        eprintln!("SKIPPED: Chrome is absent; text selection was not measured");
        return;
    };
    let folder =
        crate::scratch_folders::a_folder(format!("smoozy-selecting-text-{}", std::process::id()));
    let web = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web");
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
    // Test-only exports on the temporary copies reach the actual builders.
    // The installed page and the production sources gain no testing entry.
    for (file, exports) in [
        (
            "keys.js",
            "export { theRowIsDrawn, theClipboardTakesItTheOtherWay };",
        ),
        ("telegram.js", "export { theClipboardTakesItTheOtherWay };"),
        (
            "the-machines-own-words.js",
            "export { theClipboardTakesItTheOtherWay };",
        ),
        (
            "the-queue.js",
            "export { theFactsAreDrawn, theBlenderOutputIsDrawn };",
        ),
        ("files.js", FILE_EXPORTS),
        ("transfers.js", "export { saidInto as theSelectionLine };"),
    ] {
        let path = folder.join(file);
        let mut source = fs::read_to_string(&path).unwrap();
        source.push('\n');
        source.push_str(exports);
        fs::write(path, source).unwrap();
    }
    let line = super::files::Words::with("To", "My server")
        .then("into")
        .and_a_path("/tmp/path")
        .then("files")
        .and("42");
    let page = PAGE.replace("WORDS_VALUE", &serde_json::to_string(&line).unwrap());
    fs::write(folder.join("index.html"), page).unwrap();
    let dumped = folder.join("dump.html");
    let errors = folder.join("errors.txt");
    let mut child = std::process::Command::new(chrome)
        .args([
            "--headless",
            "--disable-gpu",
            "--no-first-run",
            "--allow-file-access-from-files",
            "--virtual-time-budget=8000",
            "--dump-dom",
        ])
        .arg(format!(
            "--user-data-dir={}",
            folder.join("profile").display()
        ))
        .arg(format!("file://{}", folder.join("index.html").display()))
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
    let page = fs::read_to_string(dumped).unwrap();
    let result = page
        .split("<pre id=\"out\">")
        .nth(1)
        .and_then(|rest| rest.split("</pre>").next())
        .unwrap_or("");
    assert!(
        status.success() && result == "PASS",
        "selection: {result}; {status}; {}",
        fs::read_to_string(errors).unwrap_or_default()
    );
}

#[test]
fn a_known_job_path_is_marked_even_without_a_connection() {
    use super::the_queue::{the_entry_is_said, AFactOnTheEntry, TheJobEntry};
    use crate::events::{Event, Subject};
    let mut entry = TheJobEntry {
        facts: vec![AFactOnTheEntry {
            name: "Writing to",
            value: "/root/job".into(),
            ..Default::default()
        }],
        ..Default::default()
    };
    let says = |entry: &TheJobEntry| {
        format!(
            "{:?}",
            the_entry_is_said(Event::about(Subject::TheRun, "test:selection"), Some(entry))
        )
    };
    let known = says(&entry);
    assert!(
        known.contains("fact0IsPath"),
        "a known offline folder can be copied"
    );
    assert!(
        !known.contains("fact0OpensFiles"),
        "an offline folder is not a link"
    );
    entry.facts[0].value = "not known".into();
    assert!(
        !says(&entry).contains("fact0IsPath"),
        "unknown is words rather than a path"
    );
    entry.facts[0].name = "Samples";
    entry.facts[0].value = "64".into();
    assert!(
        !says(&entry).contains("fact0IsPath"),
        "settings are not selectable"
    );
}

const FILE_EXPORTS: &str = r#"
export { saidInto as theSelectionLine };
export function theSelectionFixture(rows) {
  said = { listing: { is: "rows", rows } };
  chosen = [];
  theLastClicked = null;
  return rows.map(aRow);
}
export function theFixtureChooses(row, press) {
  aRowIsChosen(row, theWayARowIsChosen(press));
  return chosen.length;
}
"#;

const PAGE: &str = r##"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css"></head><body><div id="samples"></div><pre id="out"></pre>
<script>
window.__TAURI__ = {core:{invoke:async command => command === 'the_menu_bar' ? {drawn_by_the_application:false} : null},event:{listen:async () => () => {}}};
window.addEventListener('error', event => { document.getElementById('out').textContent = event.message; });
window.addEventListener('unhandledrejection', event => { document.getElementById('out').textContent = String(event.reason); });
</script><script type="module">
import {theRowIsDrawn, theClipboardTakesItTheOtherWay as keysCopy} from './keys.js';
import {theClipboardTakesItTheOtherWay as telegramCopy} from './telegram.js';
import {theClipboardTakesItTheOtherWay as wordsCopy, theWordsPanel} from './the-machines-own-words.js';
import {theFailureShape, EXPANDED, A_COLLAPSED_ROW} from './the-failure-shape.js';
import {theSelectionFixture, theFixtureChooses, theSelectionLine as filesLine} from './files.js';
import {theSelectionLine as transfersLine} from './transfers.js';
import {theFactsAreDrawn, theBlenderOutputIsDrawn} from './the-queue.js';
const out = document.getElementById('out');
const samples = document.getElementById('samples');
function check(ok, why) { if (!ok) throw Error(why); }
function style(node, wanted, why) {
  check(!!node, why + ': missing');
  const s = getComputedStyle(node);
  check(s.userSelect === wanted && s.webkitUserSelect === wanted, why + ': ' + s.userSelect + '/' + s.webkitUserSelect);
  check(wanted === 'none' ? s.cursor !== 'text' : s.cursor === 'text', why + ': cursor ' + s.cursor);
}
try {
  const said = {row0Identity:'key',row0Name:'My key',row0Fingerprint:'SHA256:abc',row0Path:'/tmp/key',row0Kind:'external'};
  const row = theRowIsDrawn(said, 0, false).root;
  samples.append(row);
  style(row.querySelector('.ky-name'), 'none', 'row name');
  style(row.querySelector('.ky-name bdi'), 'none', 'nested row name');
  style(row.querySelector('.ky-fp'), 'text', 'fingerprint');
  style(row.querySelector('.ky-where bdi'), 'text', 'path inside isolation');
  const picking = theRowIsDrawn(said, 0, true).root;
  samples.append(picking);
  style(picking.querySelector('.ky-fp'), 'none', 'clickable key choice fingerprint');
  style(picking.querySelector('.ky-where bdi'), 'none', 'clickable key choice path');

  const files = [1,2,3].map(n => ({name:{shown:'file'+n,bytes:[n]},path:{bytes:[47,n]},icon:'file',kind:'File'}));
  const rows = theSelectionFixture(files);
  samples.append(...rows);
  check(theFixtureChooses(files[0], {}) === 1, 'plain click chooses one');
  check(theFixtureChooses(files[2], {shiftKey:true}) === 3, 'Shift-click chooses range');
  for (const one of rows) for (const node of [one, ...one.querySelectorAll('*')]) style(node, 'none', 'Files range never paints text');

  for (const room of [EXPANDED,A_COLLAPSED_ROW]) {
    const failure = theFailureShape({sentence:'Failed.',whatToDoAboutIt:'Try again.',values:[['Path','/tmp/file']],controls:[{is:'retry',label:'Retry',act:'retry'}],room});
    samples.append(failure);
    style(failure.querySelector('.failure-said b'), room === EXPANDED ? 'text' : 'none', 'failure sentence');
    style(failure.querySelector('button'), 'none', 'failure control');
    if (room === EXPANDED) {
      style(failure.querySelector('.failure-second b'), 'text', 'second sentence');
      style(failure.querySelector('dd bdi'), 'text', 'failure path');
      style(failure.querySelector('dt'), 'none', 'failure value label');
    }
  }
  const step = theFailureShape({sentence:'Failed.',controls:[],room:EXPANDED,onAStep:true});
  samples.append(step);
  style(step.querySelector('b'), 'text', 'whole step failure');

  const panel = theWordsPanel({title:'Output',tabs:[{name:'Output',record:'Printed words',said:'Explanation.'}],close:'Close'});
  panel.hidden=false; samples.append(panel);
  style(panel.querySelector('.lbody'), 'text', 'printed words');
  style(panel.querySelector('.lnone b'), 'none', 'panel explanation');
  style(panel.querySelector('h3'), 'none', 'panel title');

  // The complete sheets override the drawing's address rule in both forms.
  const address = document.createElement('span'); address.className='rp-addr mono'; address.textContent='report@example.test'; samples.append(address);
  style(address, 'all', 'whole report address');
  const field = document.createElement('input'); samples.append(field); style(field,'text','typed field');
  const area = document.createElement('textarea'); samples.append(area); style(area,'text','sentence box');

  // Exercise each real fallback with a fake copy command. Nothing touches
  // the browser or system clipboard. Inspect while each box is still alive.
  const seen=[];
  document.execCommand = command => {
    check(command === 'copy', 'unexpected copy command');
    const box = document.querySelector('.ky-away,.tg-away,.words-away');
    style(box, 'text', 'offscreen copy box');
    check(box.selectionStart === 0 && box.selectionEnd === box.value.length, 'whole fallback selected');
    seen.push(box.className); return true;
  };
  for (const copy of [keysCopy,telegramCopy,wordsCopy]) check(copy('copy fixture'), 'fallback succeeds');
  check(seen.join(',') === 'ky-away,tg-away,words-away', 'all three fallbacks');

  // The sentence is serialized by the actual Rust Words builder. Server
  // names and counts share its value route, but only the path opts in.
  const words = WORDS_VALUE;
  for (const build of [node => filesLine(node,words,'mono',true), node => transfersLine(node,words,'span','mono',true)]) {
    const node = document.createElement('p'); samples.append(node); build(node);
    const values = [...node.children].filter(one => one.querySelector('bdi'));
    check(values.length === 3, 'name, path and count are separate');
    style(values[0], 'none', 'server name in a sentence');
    style(values[1].querySelector('bdi'), 'text', 'path in a sentence');
    style(values[2], 'none', 'count in a sentence');
  }

  const entry = document.createElement('div'); samples.append(entry);
  theFactsAreDrawn(entry,{factCount:'2',fact0Name:'Folder',fact0Value:'/root/job',fact0Neither:'yes',fact0IsPath:'yes',fact1Name:'Samples',fact1Value:'64'});
  style(entry.querySelectorAll('.fact dd')[0]?.querySelector('bdi'), 'text', 'offline job path');
  style(entry.querySelectorAll('.fact dd')[1]?.querySelector('bdi'), 'none', 'ordinary job setting');
  const linked = document.createElement('div'); samples.append(linked);
  theFactsAreDrawn(linked,{factCount:'1',fact0Name:'Folder',fact0Value:'/root/job',fact0IsPath:'yes',fact0OpensFiles:'yes'});
  style(linked.querySelector('a bdi'),'none','clickable job path');
  theBlenderOutputIsDrawn(entry,{blenderOutput:'Output',entryLineCount:'1',entryLine0:'Printed'},'job');
  style(entry.querySelector('.qlines bdi'),'text','newest Blender line');
  out.textContent='PASS';
} catch (error) { out.textContent=String(error.stack || error); }
</script></body></html>"##;

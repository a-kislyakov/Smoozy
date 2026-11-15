//! Free-software drawing: the complete inventory still walks, and About's
//! offline readers return to the same box. Headless only, with a fixed deadline.
use std::path::{Path, PathBuf};
use std::time::Duration;

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
}

#[test]
fn the_free_drawing_walks_and_about_keeps_its_offline_texts() {
    let Some(chrome) = [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        "/usr/bin/google-chrome", "/usr/bin/chromium", "/usr/bin/chromium-browser",
        "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
    ].into_iter().map(PathBuf::from).find(|p| p.exists()) else {
        eprintln!("SKIPPED: no Chrome, so the free drawing was not walked"); return;
    };
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let drawing = std::fs::read_to_string(here.join("../../Preparation/Design/Mockup/Smoozy.html")).unwrap();
    let scratch = Scratch(std::env::temp_dir().join(format!("smoozy-free-drawing-{}", std::process::id())));
    std::fs::create_dir_all(&scratch.0).unwrap();
    for width in [1280, 720] {
        let page = scratch.0.join(format!("drawing-{width}.html"));
        std::fs::write(&page, format!("{drawing}{READ}" )).unwrap();
        let dump = scratch.0.join(format!("dump-{width}.html"));
        let mut child = std::process::Command::new(&chrome)
            .args(["--headless", "--disable-gpu", "--no-first-run", "--allow-file-access-from-files", "--virtual-time-budget=5000", "--dump-dom"])
            .arg(format!("--window-size={width},900"))
            .arg(format!("--user-data-dir={}", scratch.0.join(format!("profile-{width}")).display()))
            .arg(format!("file://{}", page.display()))
            .stdout(std::fs::File::create(&dump).unwrap()).stderr(std::process::Stdio::null()).spawn().unwrap();
        // A one-shot deadline can be cancelled when Chrome finishes; no wait loop.
        let pid = child.id();
        let (stop, deadline) = std::sync::mpsc::channel::<()>();
        let guard = std::thread::spawn(move || {
            if deadline.recv_timeout(Duration::from_secs(90)).is_err() {
                #[cfg(unix)] let _ = std::process::Command::new("kill").args(["-TERM", &pid.to_string()]).status();
                #[cfg(windows)] let _ = std::process::Command::new("taskkill").args(["/PID", &pid.to_string(), "/T", "/F"]).status();
            }
        });
        let status = child.wait().unwrap(); let _ = stop.send(()); guard.join().unwrap();
        assert!(status.success(), "Chrome failed at {width}");
        let dump = std::fs::read_to_string(dump).unwrap();
        let read = dump.split("<pre id=\"free-drawing-result\" hidden=\"\">").nth(1).and_then(|s| s.split("</pre>").next()).expect("the browser answered");
        let read: serde_json::Value = serde_json::from_str(read).unwrap();
        assert_eq!(read["faults"], serde_json::json!([]), "at {width}");
        // 343 from 2026-11-02: the first 5.3 live pass added `res-drvold`
        // (`Render Settings` D6a).
        // 342 from 2026-11-02: a driver with no OptiX library added
        // `su-nooptix` (`Servers` H11b's OptiX-library bullet).
        // 341 from 2026-11-01: a Linux rollback whose record cannot be read
        // added `upd-rollback-unread` (the D6b line under Z11).
        // 340 from 2026-11-01: a build with no update keys added
        // `about-no-updates` (Architecture E10b).
        // 339 from 2026-10-31: the Telegram discussion added `tg-slash`,
        // `tg-reporting`, `tg-reportingset` and `tg-live`.
        // 335 from 2026-10-31: the twenty-fourth notes, note 2, added
        // `rail-ask`, `rail-noask`, `rail-asknarrow` and
        // `rail-noasknarrow`; the seventeenth notes' note 10 `tg-layers`,
        // and note 11 `fb-view`, `fb-viewexr` and `fb-viewbig`.
        // 327 from 2026-10-30: the twenty-third notes, note 2, added
        // `fb-sizemenu`, `fb-size` and `fb-sizeold`.
        // 324 from 2026-10-29: the twenty-first notes added `su-done`,
        // `fb-hidden`, `fb-check`, `res-footdlnarrow`, `q-tg`, `q-tglist`,
        // `q-tgold` and `tg-unsent`.
        // 316 from 2026-10-28: the twentieth notes added `res-footdl`,
        // `res-tgoff` and `chk-onserver`.
        // 313 from 2026-10-28: note 7 added three states, note 2 one
        // (`su-queue`) and the video wants two (`tg-video`, `tg-videobig`).
        // 307 from 2026-10-27: the drawing gained `res-oneframe` (the
        // Creator's nineteenth notes, note 8). It was 306 until then.
        // 346 from 2026-11-04: the check's `chk-walk`, `chk-unchecked` and
        // `chk-unsetup`.
        // 347 from 2026-11-04: `q-simulation` draws the scene’s simulation
        // frame before rendering (`Queue & History` R6a).
        // 351 from 2026-11-06: four simulation choice/range states.
        // 353 from 2026-11-09: the twenty-eighth notes' `su-dl` and
        // `su-dllog`.
        // 354 from 2026-11-09: the twenty-eighth notes' piece 3 took out
        // `about-new-linux` and `about-held` and added `upd-running`,
        // `upd-linux` and `upd-pagefail` (Architecture Z33).
        // 357 from 2026-11-09: the twenty-eighth notes' piece 11 added
        // `chk-linked`, `t-growing` and `t-grown` (note 11).
        // 359 from 2026-11-09: the twenty-eighth notes' piece 7 added
        // `fb-counting` and `fb-uploading` (note 10).
        // 363 from 2026-11-09: the twenty-eighth notes' piece 8 added
        // `q-checkstop`, `q-firstcancel`, `tg-checkcancel` and `tg-checkstop`
        // (note 16).
        // 367 from 2026-11-10: the twenty-eighth notes' piece 9 added
        // `res-many`, `res-manygrp`, `res-manygrpopen` and `res-manynarrow`
        // (note 17).
        // 372 from 2026-11-10: the twenty-ninth notes' drawing pass added
        // `q-manyfolders`, `q-manyfoldersopen`, `fmt-many`, `fb-viewalpha`
        // and `res-noscene` (notes 6, 8, 10 and 12).
        // 374 from 2026-11-11: q-again and tg-again, the interrupted job's words.
        // 375 from 2026-11-12: `older`, the one line an older Smoozy shows on
        // a newer record (`Design/Whole app`).
        // 393 from 2026-11-12: the last three owed added `h-missing`.
        // 392 from 2026-11-12: seventeen current response examples.
        // 394 from 2026-11-13: `fb-home`, the home folder named at the
        // server's top (the Creator's notes on the test copy, note 0).
        // 395 from 2026-11-13: `pane-disconnect`, the heading's control
        // under the pointer, one width and red (note 7).
        // 398 from 2026-11-13: the second round's `res-readframes`,
        // `chk-addedrun` and `chk-fileup` (notes 15, 14 and 17).
        // 399 from 2026-11-14: batch 1, a bot conflict at the token paste.
        // 402 from 2026-11-14: batch 2, finished on a full disk,
        // unreadable queue records and a file blocking the download folder.
        // 404 the same day: batch 2's `delete-own` and `fb-live-ssh`, Smoozy's
        // own files and the login's SSH keys named in Files' boxes, and 405
        // with `damaged`, a damaged record's one line. 406 with `su-other`,
        // the Creator's ruling on replacing another copy's setup, and 407 with
        // `su-discard`, their ruling on an older program's job.
        // 408 from 2026-11-14: the Rough what-if answers, share 5 (424),
        // `upd-setup`, the update box while a server is being set up.
        assert_eq!(read["count"], 408);
        assert_eq!(read["licence"], std::fs::read_to_string(here.join("../../COPYING")).unwrap());
        assert!(read["noticesLength"].as_u64().unwrap() > 1_000_000, "complete notices, not a sample");
    }
}

const READ: &str = r##"
<pre id="free-drawing-result" hidden></pre>
<script>
const faults=[];
window.addEventListener('error', e=>faults.push(e.message));
function check(value, message){ if(!value) faults.push(message); }
window.addEventListener('load', ()=>{
  try {
    document.documentElement.dataset.theme='light';
    const state=document.getElementById('state');
    function go(value){ state.value=value; state.dispatchEvent(new Event('change')); }
    const gone='ax ax-refused ax-two ax-trial7 ax-trial3 ax-trial ax-3day ax-locked ax-lockbad ax-locktwo ax-refact set-nocheck set-trial set-keycheck set-release set-releasing tg-lock'.split(' ');
    for(const value of gone) check(!Array.from(state.options).some(o=>o.value===value),'retained '+value);
    check(!state.querySelector('optgroup[label="Getting in"]'),'paid group');
    for(const value of ['set-tgoff','set-nowarn','set-botbad','set-botnet']) {
      const option=Array.from(state.options).find(o=>o.value===value);
      check(option?.parentNode.label==='Settings','moved '+value);
    }
    for(const option of state.options) go(option.value);
    for (const [value, selector, words] of [
      ['fb-admin-path', '#fb-admin-answer', 'Not done. The folder path could not be verified.'],
      ['set-bot-existingbusy', '#stgbot [data-tg="busy"] b', 'This bot is used by another app. Replace its token in @BotFather first.'],
      ['fb-admin-root', '#fb-admin-answer', 'Folder created, but owned by root.'],
      ['fb-admin-busy', '#fb-admin-answer', 'Not done. Another admin action was running. Try again.'],
      ['su-unsupported-cpu', '#sunotset .sunote', "This server's CPU is not supported. The server reports its CPU as aarch64."],
      ['res-first-sent', '#setwrap [data-sent]', 'Only the first frame was submitted.'],
      ['chk-macos', '#chkwrap .shead .sub', 'macOS may be asking for access to Documents. Look for its prompt.']
    ]) {
      go(value);
      const node=document.querySelector(selector);
      check(node?.textContent===words, 'wording: '+value);
      check(node && !node.closest('[hidden]'), 'hidden wording: '+value);
    }
    go('sv-jobcheck');
    check(document.querySelector('#svlist').textContent.includes('could not check for running jobs.'), 'missing server check warning');
    go('sv');
    check(!document.querySelector('#svlist').textContent.includes('could not check for running jobs.'), 'server warning leaked');
    go('fb-admin-root'); go('fb-refused');
    check(document.querySelector('#fb-admin-answer').hidden, 'admin result leaked');

    go('q-again');
    const rerun=document.querySelector('#queueview .qrow[data-qr="run held"]');
    check(rerun.querySelector('.qst').textContent==='Interrupted. Retrying.','interrupted row');
    check(rerun.querySelector('.qbars').hidden,'no earlier bars');
    check(document.querySelector('#queueview .statusline[data-ff="run"]').textContent.includes('Interrupted. Retrying.'),'interrupted entry');
    check(!document.getElementById('qstop').hidden,'interrupted job can stop');
    go('q-first');
    check(rerun.querySelector('.qst').textContent==='Needs your answer','held words restored');
    check(rerun.querySelector('.qbars').hidden,'held bars stay hidden');
    go('q');
    check(rerun.querySelector('.qst').textContent==='Rendering','rendering words restored');
    go('tg-again');
    const phone=document.querySelector('[data-tg="again"]');
    check(!phone.hidden && phone.textContent.includes('Interrupted. Retrying.'),'interrupted phone');
    check(phone.querySelectorAll('.tg-btns button').length===1,'one Stop on phone');
    check(!document.querySelector('#axchip,#axband,#axentry,#s-li,#stgrelease'),'paid surface');
    go('q-part');
    check(!document.getElementById('qpart').hidden && document.querySelector('#qpart .ax-why'), 'queue reason retained');
    check(document.querySelector('style').textContent.includes('.ax-off {'), 'shared disabled-control style retained');
    go('about');
    const box=document.getElementById('aboutbox'), text=document.getElementById('abtext');
    check(!box.hidden && box.textContent.includes('Copyright © 2026 Alexander Kislyakov'),'About notice');
    check(box.querySelector('a').href==='https://github.com/a-kislyakov/Smoozy','fixed source');
    check(!box.textContent.includes('Donate'),'no donation address');
    let licence='',noticesLength=0;
    for(const value of ['about-licence','about-notices']) {
      go('about'); box.querySelector('[data-go="'+value+'"]').click();
      check(!text.hidden,'reader opens '+value);
      check(getComputedStyle(text).userSelect!=='none','selectable '+value);
      if(value==='about-licence') licence=text.textContent; else noticesLength=text.textContent.length;
      const r=box.querySelector('.mbox').getBoundingClientRect(), app=document.querySelector('.app').getBoundingClientRect();
      check(r.left>=app.left-1 && r.right<=app.right+1 && r.top>=app.top-1 && r.bottom<=app.bottom+1,'reader fits '+value+' '+JSON.stringify({box:r.toJSON(),app:app.toJSON()}));
      box.querySelector('[data-close]').click();
      check(!box.hidden && text.hidden && state.value==='about','Close returns '+value);
      go(value); document.dispatchEvent(new KeyboardEvent('keydown',{key:'Escape'}));
      check(!box.hidden && text.hidden && state.value==='about','Escape returns '+value);
    }
    go('about-source-fail'); check(!document.getElementById('absourcefail').hidden,'source refusal');
    go('about-new');
    box.querySelector('[data-go="about-licence"]').click(); box.querySelector('[data-close]').click();
    check(state.value==='about-new' && !Array.from(box.querySelectorAll('[data-ab="new"]')).some(e=>e.hidden),'updates retained after reader');
    for(const gone of ['about-held','about-new-linux']) check(!Array.from(state.options).some(o=>o.value===gone),'retained '+gone);
    const upd=document.getElementById('updbox');
    for(const [value, shown] of [['upd',[]],['upd-running',['running']],['upd-setup',['setup']],['upd-linux',['linux']],['upd-pagefail',['pagefail']]]) {
      go(value);
      check(!upd.hidden && upd.querySelector('.upd-new') && upd.querySelector('h3').textContent.includes('is ready to'),'the update box '+value);
      const drawn=Array.from(upd.querySelectorAll('[data-ub]')).filter(e=>!e.hidden).map(e=>e.dataset.ub);
      check(JSON.stringify(drawn)===JSON.stringify(shown),'the update box '+value+' drew '+drawn);
    }
    document.getElementById('free-drawing-result').textContent=JSON.stringify({faults,count:state.options.length,licence,noticesLength}).replace(/[<>&\u00a0]/g,c=>'\\u'+c.charCodeAt(0).toString(16).padStart(4,'0'));
  } catch(e) {document.getElementById('free-drawing-result').textContent=JSON.stringify({faults:[...faults,e.stack]});}
});
</script>
"##;

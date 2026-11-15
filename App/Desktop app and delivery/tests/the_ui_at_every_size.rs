//! Remaining UI-size findings: real page builders and styles in headless Chrome.
//! No desktop session, record, or server is opened. Each page has a deadline.
use std::{path::{Path, PathBuf}, time::Duration};

fn draw(finding: &str, window: (u32, u32), script: &str) {
    draw_keeping(finding, window, &[], script)
}

/// **As `draw`, with the named page modules left real** rather than stood in
/// for — the third builder's screens (Settings, Telegram, Set up, Render
/// Settings) are drawn by their own modules. The stand-ins record what a
/// module registers, so a test can draw a screen and tell it an event.
fn draw_keeping(finding: &str, window: (u32, u32), keep: &[&str], script: &str) {
    let Some(chrome) = [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        "/usr/bin/google-chrome", "/usr/bin/chromium", "/usr/bin/chromium-browser",
        "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
    ].into_iter().map(PathBuf::from).find(|p| p.exists()) else {
        eprintln!("SKIPPED: no Chrome; {finding} was not drawn"); return;
    };
    let folder = std::env::temp_dir().join(format!("smoozy-ui-size-{finding}-{}", std::process::id()));
    std::fs::create_dir_all(&folder).unwrap();
    let web = std::env::var_os("SMOOZY_UI_SIZE_WEB").map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("web"));
    for entry in std::fs::read_dir(web).unwrap() {
        let path = entry.unwrap().path();
        if matches!(path.extension().and_then(|s| s.to_str()), Some("js" | "css")) {
            std::fs::copy(&path, folder.join(path.file_name().unwrap())).unwrap();
        }
    }
    for (file, more) in [
        ("files.js", "\nexport { aRow, theListingIsDrawn, theListIsDrawn, theHoverLineIsShown, anEditingRow, theActIsAnswered, theCrumbsAreDrawn };\nexport function arrange(h, l, answer, edit) { host=h; list=l; said=answer; editing=edit; server='made-up'; }\nexport function arrangeTheCrumbs(c) { crumbs=c; }"),
        ("keys.js", "\nexport { theVerdictIsDrawn };"),
        ("transfers.js", "\nexport { theFormatBoxIsDrawn, aRequest, theScreenIsDrawn };"),
        ("render-settings.js", "\nexport { theTip, aRowIsDrawnInto, theHeaderIsDrawn };"),
        ("set-up.js", "\nexport { theInstallBoxIsDrawn };"),
        ("servers.js", "\nexport { theConnectControl };"),
        ("the-dependency-check.js", "\nexport { theHeadIsDrawn, theTools };"),
    ] {
        use std::io::Write;
        std::fs::OpenOptions::new().append(true).open(folder.join(file)).unwrap().write_all(more.as_bytes()).unwrap();
    }
    for (file, text) in [
        ("event-route.js", "export function whenTold(kind, handler) { ((window.__told ||= {})[kind] ||= []).push(handler); } export function theRouteIsOpen() {}"),
        ("the-rail.js", "export const KEYS='keysview', TRANSFERS='transfers', A_SERVER='server', ALL_SERVERS='servers', SETTINGS='settings', THE_HISTORY='history', THE_QUEUE='queue'; export function theRowsFigureIs() {} export function aScreenIsRegistered(screen) { (window.__screens ||= []).push(screen); } export function theCurrentPlaceIs() {} export function theCurrentPlace() { return null; } export function theRailIsReadAgain() {}"),
        ("a-server.js", "export const FILES='files', SET_UP='set-up', RENDER_SETTINGS='render-settings', THE_DEPENDENCY_CHECK='the-check'; export function theServerWillOpenOn() {} export function thePanelOverItIsCollapsed() {} export function aTabIsRegistered(tab) { (window.__tabs ||= []).push(tab); } export function aPanelIsOpenOver() {return false;} export function theCurrentTabIs() {} export function theServersAreAskedForAgain() {} export function aPanelIsRegistered() {} export function thePanelCollapsesAsItsChevronDoes() {} export function thePanelIsOpenedOver() {} export function theStripReads() {}"),
        ("the-dependency-check.js", "export function theCheckIsOpenedOn() {} export function theCheckIsShownOn() {}"),
        ("render-settings.js", "export function theSettingsAreOpenedOn() {}"),
        ("transfers.js", "export function aTransferIsAskedFor() {} export function theTransfersAreShown() {} export function theFormatBoxIsOpenedFor() {} export function theFormatBoxIsOpenedForAFile() {}"),
        ("settings.js", "export const TRANSFERS_SECTION='transfers', SEARCH_FOLDERS_SECTION='folders'; export function theSettingsSectionIsOpened() {}"),
        ("the-history.js", "export function theHistoryEntryIsOpened() {}"),
    ] {
        if file == "transfers.js" && finding == "C-22" { continue; }
        if keep.contains(&file) { continue; }
        std::fs::write(folder.join(file), text).unwrap();
    }
    // **A module the test names on an `import` line loads with the page**, as
    // the page's own do: one loaded later by `import()` raced Chrome's
    // virtual clock, and a large one lost.
    let (imports, body): (Vec<&str>, Vec<&str>) = script.lines().partition(|line| line.trim_start().starts_with("import "));
    let page = PAGE.replace("TEST_IMPORTS", &imports.join("\n")).replace("TEST_SCRIPT", &body.join("\n"));
    std::fs::write(folder.join("index.html"), page).unwrap();
    let mut faults = Vec::new();
    for theme in ["light", "dark"] {
        let picture_folder = std::env::var_os("SMOOZY_UI_SIZE_PICTURES").map(PathBuf::from);
        let before = std::env::var_os("SMOOZY_UI_SIZE_WEB").is_some();
        let mut command = std::process::Command::new(&chrome);
        command.args(["--headless", "--disable-gpu", "--no-first-run", "--allow-file-access-from-files", "--hide-scrollbars", "--virtual-time-budget=5000"])
            .arg(format!("--window-size={},{}", window.0, window.1))
            .arg(format!("--user-data-dir={}", folder.join(format!("profile-{theme}")).display()));
        if let Some(ref pictures) = picture_folder {
            std::fs::create_dir_all(pictures).unwrap();
            command.arg(format!("--screenshot={}", pictures.join(format!("{finding} {}, {}x{} {theme}.png", if before {"before"} else {"after"}, window.0, window.1)).display()));
        }
        command.arg("--dump-dom");
        let dump = folder.join(format!("dump-{theme}.html"));
        let mut child = command.arg(format!("file://{}#{theme}", folder.join("index.html").to_string_lossy().replace(' ', "%20")))
            .stdout(std::fs::File::create(&dump).unwrap()).stderr(std::fs::File::create(folder.join(format!("errors-{theme}.txt"))).unwrap()).spawn().unwrap();
        let pid = child.id();
        let (stop, deadline) = std::sync::mpsc::channel::<()>();
        let guard = std::thread::spawn(move || {
            if deadline.recv_timeout(Duration::from_secs(60)).is_err() {
                #[cfg(unix)] let _ = std::process::Command::new("kill").args(["-TERM", &pid.to_string()]).status();
                #[cfg(windows)] let _ = std::process::Command::new("taskkill").args(["/PID", &pid.to_string(), "/T", "/F"]).status();
            }
        });
        let status = child.wait().unwrap(); let _ = stop.send(()); guard.join().unwrap();
        assert!(status.success(), "Chrome failed: {finding}: {}", std::fs::read_to_string(folder.join(format!("errors-{theme}.txt"))).unwrap());
        let dump = std::fs::read_to_string(dump).unwrap();
        let read = dump.split("<pre id=\"out\" hidden=\"\">").nth(1).and_then(|s| s.split("</pre>").next()).expect("Chrome answered");
        let read = read.replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">");
        let read: serde_json::Value = serde_json::from_str(&read).unwrap();
        if read != serde_json::json!([]) { faults.push(format!("{finding}, {theme}: {read}")); }
    }
    assert!(faults.is_empty(), "{}", faults.join("\n"));
}

const PAGE: &str = r#"<!doctype html><html data-theme="light"><head><meta charset="utf-8"><link rel="stylesheet" href="smoozy.css">
<style>html,body{height:100%;margin:0;overflow:hidden}#host{height:100%;position:relative}#list{height:100%;overflow:auto}#out{display:none}</style></head>
<body><div class="files" id="host"><div class="flist" id="list"></div></div><pre id="out" hidden></pre>
<script>window.__TAURI__={core:{invoke:()=>Promise.resolve(null)}};</script>
<script type="module">
import * as files from './files.js'; import * as keys from './keys.js';
TEST_IMPORTS
document.documentElement.dataset.theme=location.hash.slice(1);
const faults=[]; const check=(yes,why)=>{if(!yes)faults.push(why)};
const host=document.getElementById('host'), list=document.getElementById('list');
const box=e=>e.getBoundingClientRect(); const words=(...pieces)=>({pieces:pieces.map(said=>({said}))});
const value=value=>({value});
const headers={name:'Name',kind:'Kind',size:'Size',changed:'Changed',sorted_by:'name'};
const row=(name)=>({name:{shown:name,bytes:Array.from(new TextEncoder().encode(name))},path:{shown:'/'+name,bytes:[47,1]},icon:'blend',kind:'File',size:'2 MB'});
const tell=(kind,said)=>((window.__told||{})[kind]||[]).forEach(handler=>handler({sequence:1,kind,subject:'run',said}));
const screen=(place)=>(window.__screens||[]).find(one=>one.place===place);
const app=(marks)=>{const shell=document.createElement('div');shell.className='app';shell.dataset.narrow=marks;document.body.prepend(shell);return shell;};
try { TEST_SCRIPT } catch(e) { faults.push(e.stack || String(e)); }
document.getElementById('out').textContent=JSON.stringify(faults);
</script></body></html>"#;

#[test]
fn b23_an_unusable_keys_note_takes_only_its_text_height() {
    draw("B-23", (1180, 760), r#"
const root=document.createElement('div'); root.className='ky-row'; list.append(root);
const slot=document.createElement('div'); root.append(slot);
keys.theVerdictIsDrawn({root,verdictSlot:slot,copySlot:document.createElement('span'),choose:document.createElement('button'),said:{chooseItsNewLocation:'Locate…',chooseAReplacement:'Choose a replacement…',whatAReplacementCosts:'Servers using this key will use the replacement.'}},
{verdict:'unusable',row:'k1',whatIsWrong:'Permissions too open.',andWhatToDo:'Only you should be able to read this key.'});
const note=root.querySelector('.ky-note');
check(box(note).height < 60, 'the replacement note reserves '+box(note).height+' pixels');
"#);
}

#[test]
fn c03_a_long_blend_name_keeps_its_version_tag_visible() {
    draw("C-03", (1280, 720), r#"
const entry=files.aRow({...row('Kitchen_'.repeat(20)+'.blend'),saved_with:'5.3'}); list.append(entry);
const name=entry.querySelector('.fname'), tag=name.querySelector('.ver');
check(box(tag).right <= box(name).right && box(tag).left >= box(name).left, 'the version tag is outside the visible name cell');
const shown=name.firstElementChild; shown.dispatchEvent(new Event('mouseenter')); name.dispatchEvent(new Event('mouseenter'));
check((shown.title || name.title).includes('Kitchen_'.repeat(20)), 'the shortened name cannot be read in full');
const refused=files.aRow({...row('Kitchen_'.repeat(20)+'.blend'),saved_with:'5.3',refusal:{said:words('Permission denied: this file cannot be renamed.' )}}); list.append(refused);
const denied=refused.querySelector('.deny'), line=refused.querySelector('.fname-text');
check(box(denied).top>=box(line).bottom,'a refusal no longer sits below the filename');

"#);
}

#[test]
fn c08_folder_status_reads_as_a_sentence_inside_the_list() {
    draw("C-08", (681, 760), r#"
files.arrange(host,list,null,null);
files.theListingIsDrawn({headers,listing:{is:'the-folder-was-not-read',said:{pieces:[{said:'Does not exist yet:'},value('/shots/'+'Long_folder_'.repeat(18)),{said:'. No frames saved.',joins:true}]}}});
const line=list.querySelector('.fwb'); const first=line.firstElementChild;
check(box(first).height <= 30, 'the sentence opening is squeezed into a column');
check(box(line).right<=innerWidth && box(line).left>=0, 'the status extends outside the list');
check(line.scrollWidth<=line.clientWidth+1, 'a long path does not break inside the status');
"#);
}

#[test]
fn c10_a_rename_hint_and_refusal_are_brought_into_view() {
    draw("C-10", (1280, 600), r#"
list.style.height='350px'; list.style.flex='none';
const rows=Array.from({length:30},(_,i)=>({...row('file_'+i),path:{shown:'/file_'+i,bytes:[i]}}));
const edit={kind:'rename',path:[29],was:'file_29',typed:'file_29',refused:null,refusal:null};
const answer={headers,listing:{is:'rows',rows},toolbar:{rename:'Rename',around_an_act:{the_listing_saw_it:words('Name in use as of the last refresh.')}}};
files.arrange(host,list,answer,edit); files.theListIsDrawn(answer); list.scrollTop=list.scrollHeight;
const field=list.querySelector('input'); field.value='file_0'; field.dispatchEvent(new Event('input'));
check(box(list.querySelector('.fhint')).bottom<=box(list).bottom+1,'the rename hint is below the list');
files.theActIsAnswered({is:'on-the-row',editing:{refused:words('Name already exists.')}});
check(box(list.querySelector('.editing')).bottom<=box(list).bottom+1,'the rename refusal is below the list');
"#);
}

#[test]
fn c11_a_menu_explanation_stays_inside_the_right_edge() {
    draw("C-11", (1280, 760), r#"
files.arrange(host,list,null,null);
const menu=document.createElement('div'); menu.className='ctx'; menu.style.cssText='position:fixed;right:8px;top:180px';
const item=document.createElement('button'); item.textContent='Calculate size';menu.append(item);host.append(menu);
files.theHoverLineIsShown(item,[words('Set up this server to calculate folder sizes.')],'beside');
const tip=host.querySelector('.fsztip'); check(box(tip).right<=innerWidth-7,'the menu explanation extends past the right edge');
check(box(tip).left>=7,'the menu explanation extends past the left edge');
"#);
}

#[test]
fn c6_calculated_size_details_stay_inside_the_bottom_edge() {
    draw("C6", (1280, 560), r#"
files.arrange(host,list,null,null);
const cell=document.createElement('span'); cell.textContent='12.4 GB';cell.style.cssText='position:fixed;right:8px;bottom:8px';host.append(cell);
files.theHoverLineIsShown(cell,[words('1,204 files, 87 folders'),words('Some folders could not be read.')],'under');
const tip=host.querySelector('.fsztip'); check(box(tip).bottom<=innerHeight-7,'calculated-size details extend past the bottom');
check(box(tip).right<=innerWidth-7 && box(tip).left>=7,'calculated-size details extend past a side');
"#);
}

#[test]
fn c22_a_long_download_box_opens_at_its_choices() {
    draw("C-22", (681, 560), r#"
const transfers=await import('./transfers.js');
transfers.theFormatBoxIsDrawn({is:'the-box',heading:'Download formats',to:'To',where_downloads_go:'Download folder',
 reading:{reads:'several-formats',intro:'Choose formats to download.',formats:Array.from({length:16},(_,i)=>({name:'format'+i,said:'OpenEXR '+i,chosen:true,where_in_the_job:'beauty/layer_'+i,files:words('240 files')})),total:words('3,840 files'),the_rest:'will download.'},
 folders:[{said:words('/Users/artist/Renders')},{said:words('/Volumes/Archive/Renders')}],somewhere_else:'Other…',cancel:'Cancel',done:'Done'},()=>{});
const body=document.querySelector('.modal .mbox'), first=body.querySelector('[role="checkbox"]');
check(document.activeElement===first,'opening the box focuses a bottom action instead of the first format');
check(box(first).top>=box(body).top && box(first).bottom<=box(body).bottom,'the first format is scrolled out of sight on open');
const label=body.querySelector('.frm > label'), folders=body.querySelector('.transfers-folders');
check(box(label).top<=box(folders).top+8,'To is centred beside the folder list');
"#);
}

#[test]
fn e08_a_blender_folder_keeps_its_distinguishing_end_visible() {
    draw("E-08", (1920, 1080), r#"
const {theValueIsDrawnInto}=await import('./the-value-drawn.js');
const table=document.createElement('div');table.className='butable';table.style.maxWidth='940px';list.append(table);
const line=document.createElement('div');line.className='burow';table.append(line);
const version=document.createElement('span');version.className='buv';version.textContent='5.3.0';line.append(version);
const folder=document.createElement('button');folder.className='bup mono golink';theValueIsDrawnInto(folder,'/opt/smoozy/blender/'+'long-folder/'.repeat(15)+'5.3.0_build_2026-11-11');line.append(folder);
const act=document.createElement('span');act.className='bua';act.textContent='Remove';line.append(act);
const text=folder.querySelector('bdi').lastElementChild || folder.querySelector('bdi');
const range=document.createRange();range.selectNodeContents(text);const end=range.getBoundingClientRect();
check(end.right<=box(folder).right+1 && end.left>=box(folder).left,'the build folder ending is clipped');
"#);
}

// ── The third builder: the items left after the second ────────────────────

/// A row of All servers as `web/servers.js` draws one, its four columns in
/// the drawing's own classes, inside a list of the given width.
const A_SERVER_ROW: &str = r#"
const theList=(marks,width)=>{host.style.display='none';const shell=app(marks);const one=document.createElement('div');one.className='svlist';if(marks.includes('s2'))one.dataset.narrow='folded';one.style.width=width+'px';shell.append(one);return one;};
const aServerRow=(into,label)=>{const r=document.createElement('div');r.className='svrow';into.append(r);
 r.innerHTML='<span class="svmain"><span class="svname"><span class="badge b1"></span><bdi>Night box</bdi></span><span class="svaddr mono">root@203.0.113.62:40122</span></span>'
 +'<span class="svwork"><b>Rendering</b> 100_0040-lighting<span class="sub">2 waiting</span></span>'
 +'<span class="svhw"><span class="mono">4 × NVIDIA GeForce RTX 4090, 24 GB each</span><span class="sub"><span class="mono">251 GB RAM</span> · <span class="mono">812 GB free disk</span> · <span>last seen</span> <span class="mono">Nov 10, 08:43</span></span></span>'
 +'<span class="svacts"><button type="button" class="cbtn"><span class="cdot"></span>'+label+'</button><button type="button" class="more">···</button></span>';return r;};
const lines=(element)=>{const range=document.createRange();range.selectNodeContents(element);return new Set([...range.getClientRects()].filter(r=>r.width>0).map(r=>Math.round(r.top))).size;};
"#;

#[test]
fn b03_below_780_the_actions_column_gives_its_spare_width_back() {
    draw("B-03", (681, 760), &format!("{A_SERVER_ROW}{}", r#"
const r=aServerRow(theList('s1 s2 s3 s4',629),'Connect and set up');
const acts=r.querySelector('.svacts'), b=r.querySelector('.cbtn'), more=r.querySelector('.more');
check(box(acts).width<=190,'the actions column keeps '+box(acts).width+' pixels below 780');
check(b.scrollWidth<=b.clientWidth+1,'Connect and set up is cut');
check(box(b).right<=box(more).left+1,'Connect runs under the menu');
check(lines(r.querySelector('.svaddr'))<=2,'the address takes more than two lines');
"#));
}

#[test]
fn b05_at_the_opening_size_a_card_line_and_a_moment_keep_one_line() {
    draw("B-05", (1180, 760), &format!("{A_SERVER_ROW}{}", r#"
const r=aServerRow(theList('',966),'Connected');
const hw=r.querySelector('.svhw'), cards=hw.querySelector(':scope > .mono'), moment=hw.querySelector('.sub > .mono:last-child');
check(lines(cards)===1,'the card line takes '+lines(cards)+' lines at the opening size');
check(lines(moment)===1,'the last-seen moment breaks across '+lines(moment)+' lines');
"#));
}

#[test]
fn b18_the_heads_failure_keeps_its_cross_beside_its_sentence() {
    draw("B-18", (1180, 760), r#"
import {theFailureShape, A_COLLAPSED_ROW} from './the-failure-shape.js';
import {theCountdown} from './the-underway-shape.js';
host.style.display='none';
const machine=document.createElement('div');machine.id='machineview';machine.style.width='966px';document.body.prepend(machine);
const head=document.createElement('div');head.className='mhead';machine.append(head);
const note=document.createElement('p');note.className='mconnectnote';head.append(note);
note.append(theFailureShape({sentence:'Could not reconnect.',kind:'connect',controls:[{is:'cross',label:'Clear'}],room:A_COLLAPSED_ROW}));
note.append(theCountdown(0,{in:'retrying in',now:'retrying now',unit:'s'}));
const said=note.querySelector('.failure-said b'), cross=note.querySelector('.failure-clear'), count=note.querySelector(':scope > .sub');
check(box(cross).left-box(said).right<40,'the cross sits '+Math.round(box(cross).left-box(said).right)+' pixels from its sentence');
check(getComputedStyle(count).color!==getComputedStyle(note).color,'the countdown takes the failure colour');
"#);
}

/// The Keys screen, drawn by `web/keys.js` in a pane of the given width.
const THE_KEYS: &str = r#"
const theKeys=(width,said)=>{host.style.display='none';const view=document.createElement('div');view.id='keysview';view.style.cssText='width:'+width+'px;height:'+innerHeight+'px';document.body.prepend(view);
 screen('keysview').itIsDrawn(view);
 tell('keys:the-screen',{theHeading:'Keys',chooseAKey:'Choose a key…',newKey:'New key…',noKeysYet:'No keys yet.',cancelAndGoBack:'Cancel',whatGoesToAProvider:'Paste the public key at your provider.',rowCount:'0',...said});return view;};
"#;

#[test]
fn b26_the_keys_header_keeps_its_buttons_on_one_line_while_choosing() {
    draw("B-26", (681, 760), &format!("{THE_KEYS}{}", r#"
const view=theKeys(629,{picking:'yes',choosingAKeyFor:'Choosing a key for',theServerNamed:'ubuntu@ec2-3-120-45-67.eu-central-1.compute.amazonaws-example.com'});
const head=view.querySelector('.svhead');
for (const b of head.querySelectorAll(':scope > .mini, :scope > .btn')) check(lines(b)===1,'the header button '+b.textContent+' wraps');
check(box(head).right<=629+1 && head.scrollWidth<=head.clientWidth+1,'the header runs past the pane');
function lines(element){const range=document.createRange();range.selectNodeContents(element);return new Set([...range.getClientRects()].filter(r=>r.width>0).map(r=>Math.round(r.top))).size;}
"#));
}

#[test]
fn b28_an_empty_keys_list_says_so() {
    draw("B-28", (1180, 760), &format!("{THE_KEYS}{}", r#"
const view=theKeys(966,{});
const list=view.querySelector('.ky-list'), foot=view.querySelector('.svfoot');
check(list.textContent.trim()==='No keys yet.','the empty list reads '+JSON.stringify(list.textContent.trim()));
check(box(foot).bottom>=innerHeight-2,'the footer sits under the header rather than at the bottom');
"#));
}

#[test]
fn c04_a_three_figure_paste_count_leaves_the_icon_readable() {
    draw("C-04", (1280, 720), r#"
const bar=document.createElement('div');bar.className='ftools';host.prepend(bar);
const paste=document.createElement('button');paste.type='button';paste.className='ib';bar.append(paste);
paste.innerHTML='<svg viewBox="0 0 16 16" aria-hidden="true" fill="currentColor"><rect x="2" y="2" width="12" height="12"></rect></svg><span class="pb">128</span>';
const divider=document.createElement('span');divider.className='tdiv';bar.append(divider);
const icon=paste.querySelector('svg'), count=paste.querySelector('.pb');
const covered=Math.max(0,box(icon).right-Math.max(box(count).left,box(icon).left));
check(covered<=box(icon).width*0.4,'the count covers '+Math.round(covered)+' of the icon\'s '+box(icon).width+' pixels');
"#);
}

#[test]
fn c12_the_staging_folders_kind_reads_whole() {
    draw("C-12", (1280, 760), r#"
const entry=files.aRow({...row('.smoozy-staging'),icon:'folder',kind:'Smoozy staging folder'});list.append(entry);
const kind=entry.querySelector('.fmeta.kind');
check(kind && kind.scrollWidth<=kind.clientWidth+1,'the kind is cut: '+(kind&&kind.scrollWidth)+' in '+(kind&&kind.clientWidth));
"#);
}

/// **The login's home folder carries a house**, drawn by `web/files.js` —
/// `Design/File Browser § The list` (the Creator, 2026-11-11). The browser at
/// `/home`: the row `ubuntu` is the home, `smoozy` is not. The house replaces
/// the folder icon, grey like it, and the row says *Home folder* on hover.
const THE_HOUSE: &str = r#"
const crumbs=document.createElement('div');crumbs.className='crumbs';host.prepend(crumbs);files.arrangeTheCrumbs(crumbs);
host.style.display='flex';host.style.flexDirection='column';list.style.height='auto';list.style.flex='1';
const bytes=(text)=>Array.from(new TextEncoder().encode(text));
const part=(folder,shown,here,home)=>({folder:{shown:folder,bytes:bytes(folder)},shown,said:shown?null:'Server',here,home});
const folderRow=(name,home)=>({...row(name),path:{shown:'/home/'+name,bytes:bytes('/home/'+name)},icon:'folder',kind:'Folder',size:null,changed:'1762900000',home});
const isAHouse=(icon)=>Boolean(icon&&icon.querySelector('path')&&icon.querySelector('path').getAttribute('d').startsWith('M7.4 2 1.6 7.1'));
"#;

#[test]
fn the_home_folders_row_carries_a_house_and_says_so_on_hover() {
    draw("home row", (1000, 260), &format!("{THE_HOUSE}{}", r#"
files.theCrumbsAreDrawn({path:[part('/','',false,null),part('/home','home',true,null)]});
const home=files.aRow(folderRow('ubuntu','Home folder')), other=files.aRow(folderRow('smoozy',null));
list.append(home,other);
const house=home.querySelector('.fic'), folder=other.querySelector('.fic');
check(isAHouse(house),'the home folder has no house');
check(!isAHouse(folder),'another folder has a house');
check(house&&house.classList.contains('folder')&&getComputedStyle(house).color===getComputedStyle(folder).color,'the house is not the folder icon\'s grey');
check(house&&Math.abs(box(house).width-16)<0.5&&Math.abs(box(house).left-box(folder).left)<0.5,'the house does not sit where the folder icon sits');
check(home.title==='Home folder','the row does not say Home folder on hover: '+home.title);
const name=home.querySelector('.fname'); name.dispatchEvent(new Event('mouseenter'));
check(name.title==='Home folder','the name does not say Home folder on hover: '+name.title);
check(!other.title,'another folder says something on hover: '+other.title);
check(home.querySelector('.kind').textContent==='Folder','the kind changed');
"#));
}

/// **The home folder's row also says *Home folder*, in grey after its name** —
/// `Design/File Browser § The list`, added 2026-11-13 (the Creator's notes on
/// the test copy of 2026-11-12, note 0: a folder called `home` beside the
/// login's home). Files at `/`: `home` is a plain folder, `root` the login's
/// home. The words sit on the name's line, after it, in the quiet grey; a long
/// name shortens and the words stay whole; no other row has them.
#[test]
fn the_home_folders_row_says_home_folder_in_grey_after_its_name() {
    draw("home words", (1000, 260), &format!("{THE_HOUSE}{}", r#"
files.theCrumbsAreDrawn({path:[part('/','',true,null)]});
const at=(name,home)=>({...folderRow(name,home),path:{shown:'/'+name,bytes:bytes('/'+name)}});
const plain=files.aRow(at('home',null)), home=files.aRow(at('root','Home folder'));
const long=files.aRow(at('a_login_with_a_very_long_name_that_cannot_fit_in_the_name_column_at_all','Home folder'));
list.append(plain,home,long);
const words=home.querySelector('.fhome'), text=home.querySelector('.fname-text');
check(words&&words.textContent==='Home folder','the home folder\'s row does not say Home folder: '+(words&&words.textContent));
check(!plain.querySelector('.fhome'),'the folder called home says Home folder');
// *On its line*: the words' middle is inside the name's line, whatever the
// face. Their tops differ by the two sizes' ascents, which is 3 pixels in
// Windows' faces (hosted run 43746222478) and less in the Mac's.
const middle=words&&(box(words).top+box(words).bottom)/2;
check(words&&text&&box(words).left>=box(text).right&&middle>box(text).top&&middle<box(text).bottom,'the words are not after the name on its line: '+(words&&text?[box(text).right,box(words).left,box(text).top,box(text).bottom,box(words).top,box(words).bottom].join(', '):'missing'));
check(words&&getComputedStyle(words).color===getComputedStyle(home.querySelector('.kind')).color,'the words are not the quiet grey: '+(words&&getComputedStyle(words).color));
const longWords=long.querySelector('.fhome'), longName=long.querySelector('.fname');
check(longWords&&box(longWords).right<=box(longName).right+0.5&&longWords.scrollWidth<=longWords.clientWidth+0.5,'a long name pushes the words out of the name column');
check(home.querySelector('.kind').textContent==='Folder','the kind changed');
"#));
}

/// **Connected keeps its width when it reads *Disconnect*, and *Disconnect* is
/// red** — `Design/Servers`, the server pane's heading, added 2026-11-13 (the
/// Creator's notes on the test copy of 2026-11-12, note 7). The real
/// `servers.js` control in the heading's `.mtop`, beside the server's name:
/// under the pointer and with focus it reads *Disconnect*, its width and the
/// name beside it stay where they were, and its text, border and dot take the
/// red the Queue's *Stop* takes (`.mini.danger`). Off the pointer it is green
/// again. *Connect*, which reads nothing on hover, stays as wide as its word.
#[test]
fn connected_keeps_its_width_and_reads_disconnect_in_red() {
    draw("connect hover", (720, 140), r#"
import * as sv from './servers.js';
host.style.display='none';
const head=document.createElement('div');head.className='mtop';head.style.cssText='width:600px;padding:12px';document.body.prepend(head);
const name=document.createElement('h1');name.textContent='vast 5080';head.append(name);
const row=(label,act,hover)=>({server:'s1',the_connection:{label,act,on_hover:hover,unavailable:false}});
const control=sv.theConnectControl(row('Connected','disconnect','Disconnect'),{rows:[]});
head.append(control);
const stop=document.createElement('button');stop.className='mini danger';stop.textContent='Stop';document.body.append(stop);
const before=box(control).width, nameAt=box(name).left;
control.dispatchEvent(new Event('mouseenter'));
check(control.textContent.includes('Disconnect')&&!/Connected(?!.*Disconnect)/.test(control.innerText),'it does not read Disconnect under the pointer: '+control.innerText);
check(Math.abs(box(control).width-before)<0.5,'the width changed from '+before+' to '+box(control).width);
check(Math.abs(box(name).left-nameAt)<0.5,'the name beside it moved');
const red=getComputedStyle(stop).color;
check(getComputedStyle(control).color===red,'Disconnect is not the red of Stop: '+getComputedStyle(control).color+' against '+red);
check(getComputedStyle(control).borderTopColor===red,'its border is not red: '+getComputedStyle(control).borderTopColor);
check(getComputedStyle(control.querySelector('.cdot')).backgroundColor===red,'its dot is not red');
control.dispatchEvent(new Event('mouseleave'));
check(control.innerText.trim()==='Connected'&&getComputedStyle(control).color!==red,'it does not come back to Connected in green');
check(Math.abs(box(control).width-before)<0.5,'the width changed coming back');
control.dispatchEvent(new Event('focus'));
check(getComputedStyle(control).color===red&&Math.abs(box(control).width-before)<0.5,'with focus it is not the same red Disconnect at the same width');
control.dispatchEvent(new Event('blur'));
const connect=sv.theConnectControl(row('Connect','connect',null),{rows:[]});
head.append(connect);
check(!connect.querySelector('.cghost'),'Connect carries labels it never reads');
"#);
}

/// **A box that runs the full width of its panel has square corners** —
/// `Design/Whole app § Type and spacing`, added 2026-11-13 (the Creator's notes
/// on the test copy of 2026-11-12, note 6). Render Settings' panel as
/// `a-server.js` builds it, with the page's own stylesheets: the *File not
/// read* band and the box a failed read sits in each run edge to edge, and
/// each has square corners; a box with room around it keeps its round ones.
#[test]
fn a_box_that_runs_the_width_of_its_panel_has_square_corners() {
    draw("full width", (1000, 300), r#"
host.style.display='none';
const wrap=document.createElement('div');wrap.className='sheetwrap';wrap.dataset.panel='set';wrap.style.cssText='position:static;display:block;background:none';document.body.prepend(wrap);
const sheet=document.createElement('div');sheet.className='sheet';sheet.style.cssText='position:static;width:900px;height:auto';wrap.append(sheet);
const body=document.createElement('div');body.className='suscroll';sheet.append(body);
const band=document.createElement('p');band.className='noread';band.innerHTML='<b>File not read.</b> Values are the file’s until changed.';body.append(band);
const failed=document.createElement('div');failed.className='qfail failure';failed.textContent='Could not reach the server.';body.append(failed);
const room=document.createElement('div');room.style.cssText='padding:20px';const kept=document.createElement('p');kept.className='noread';kept.textContent='A box with room around it.';room.append(kept);body.append(room);
for (const [one,name] of [[band,'the File not read band'],[failed,'the failed read\'s box']]) {
  check(Math.abs(box(one).left-box(body).left)<=3&&Math.abs(box(body).right-box(one).right)<=3,name+' does not run the panel\'s width');
  check(getComputedStyle(one).borderTopLeftRadius==='0px'&&getComputedStyle(one).borderBottomRightRadius==='0px',name+' has round corners: '+getComputedStyle(one).borderTopLeftRadius);
}
check(getComputedStyle(kept).borderTopLeftRadius!=='0px','a box with room around it lost its round corners');
"#);
}

/// **A failure's box keeps its cross at the top right, and is no bigger than
/// its words** — `Design/Whole app § The failure shape` and `Design/Render
/// Settings § When the read gives nothing`, both changed 2026-11-13 (the
/// Creator's note 4 on the test copy of 2026-11-12: *"the message's box is too
/// big … the closing cross is in the bottom corner, which is just bonkers"*).
/// The real `the-failure-shape.js` and stylesheets, in Render Settings' panel
/// as `a-server.js` builds it: the read's failure with its second sentence, and
/// one with the raw text beside it, as the dependency check's are.
#[test]
fn a_failures_box_keeps_its_cross_at_the_top_right_and_is_no_bigger_than_its_words() {
    draw("failure box", (1000, 320), r#"
import {theFailureShape, EXPANDED} from './the-failure-shape.js';
host.style.display='none';
const wrap=document.createElement('div');wrap.className='sheetwrap';wrap.dataset.panel='set';wrap.style.cssText='position:static;display:block;background:none';document.body.prepend(wrap);
const sheet=document.createElement('div');sheet.className='sheet';sheet.style.cssText='position:static;width:900px;height:auto';wrap.append(sheet);
const body=document.createElement('div');body.className='suscroll';sheet.append(body);
const read=theFailureShape({sentence:'Could not reach the server.',whatToDoAboutIt:'Read the file again.',kind:'condition',controls:[{is:'cross',label:'Dismiss',act:''}],room:EXPANDED});
body.append(read);
const words=theFailureShape({sentence:'Reading the file did not finish.',kind:'condition',controls:[{is:'raw-text',label:'Blender output',act:'the-raw-text'},{is:'cross',label:'Dismiss',act:''}],room:EXPANDED});
body.append(words);
for (const [shape,name] of [[read,'the read\'s failure'],[words,'the failure with Blender output']]) {
  const cross=shape.querySelector('.failure-clear'), said=shape.querySelector('.failure-said');
  check(cross,name+' has no cross');
  if(!cross) continue;
  check(!cross.closest('.failure-acts'),name+': the cross sits in the row of controls');
  check(box(cross).top-box(shape).top<16,name+': the cross sits '+Math.round(box(cross).top-box(shape).top)+' pixels below the box\'s top');
  check(box(shape).right-box(cross).right<20,name+': the cross sits '+Math.round(box(shape).right-box(cross).right)+' pixels from the box\'s right');
  check(Math.abs(box(cross).top-box(said).top)<6,name+': the cross is not on the first line');
}
const lines=[...read.children].reduce((sum,one)=>sum+box(one).height,0);
check(!read.querySelector('.failure-acts'),'the read\'s failure keeps an empty row of controls');
check(box(read).height<=lines+8*(read.children.length-1)+2*9+2+1,'the read\'s failure is '+Math.round(box(read).height)+' pixels high for '+Math.round(lines)+' pixels of words');
"#);
}

/// **A dependency upload names the file it checked, whatever its length** —
/// `Design/Transfers § The dependency check's own transfers`, added
/// 2026-11-13 (the Creator's notes on the test copy of 2026-11-12, note 12:
/// the row read *For …*, *"Dependencies for what?.."*). The real
/// `transfers.js` row, finished and on one line, at the width of the
/// Creator's window and at a narrow one, with a long `.blend` name: *For*
/// shows whole, the link shows the name's start, and where it is cut it is cut
/// inside the link, never away whole.
#[test]
fn a_dependency_upload_names_the_file_it_checked() {
    for (finding, width) in [("dependency row", 925), ("dependency row narrow", 560)] {
        draw_keeping(finding, (width, 160), &["transfers.js"], r#"
import * as transfers from './transfers.js';
host.style.display='none';
const holder=document.createElement('div');holder.style.cssText='width:100%';document.body.prepend(holder);
const named='Garden Shed Light M1 - V_001 final lighting pass.blend';
const request={id:'r1',terminal:true,tag_class:'dep',arrow:'↑',tag:'Dependencies',
 name:{pieces:[{said:'For'},{value:named}]},name_leads:true,name_ends:false,
 show_in_files:{hover:'Show in Files',folder:[47,114,111,111,116],name:[1]},
 count:words('done'),ending_class:'e-ok',collapsed:true,
 where_it_goes:{pieces:[{said:'To'},{value:'vast 5080'},{said:'·'},{value:'/root/projects/kitchen-final-with-a-long-folder/'}]},
 badge:null,control:{label:'Open dependency check',act:'open-the-check'},also:null,
 started_label:'Started',started_at_millis:'1762988440000',bar:null,lines:[],files:[],server:'s1'};
const drawn=transfers.aRequest(request);holder.append(drawn);
const name=drawn.querySelector('.rqn'), lead=drawn.querySelector('.rqlead'), link=drawn.querySelector('.golink');
check(lead&&box(lead).width>10&&box(lead).right<=box(name).right+0.5,'For is not shown whole');
check(link&&box(link).width>=60,'the file\'s name shows '+(link?Math.round(box(link).width):0)+' pixels');
check(link&&box(link).left>=box(lead).right&&box(link).left<box(name).right,'the link starts outside the name');
check(link&&box(link).right<=box(name).right+0.5,'the link runs past the name, so it is cut away whole');
"#);
    }
}

/// **The house in the path**, before the home folder's name, with the browser
/// below it at `/home/ubuntu/kitchen`.
#[test]
fn the_home_folder_carries_a_house_in_the_path() {
    draw("home path", (1000, 260), &format!("{THE_HOUSE}{}", r#"
files.theCrumbsAreDrawn({path:[part('/','',false,null),part('/home','home',false,null),part('/home/ubuntu','ubuntu',false,'Home folder'),part('/home/ubuntu/kitchen','kitchen',true,null)]});
list.append(files.aRow({...row('kitchen_v14.blend'),saved_with:'5.3'}));
const steps=[...crumbs.children].filter(one=>one.tagName!=='SPAN'||one.classList.contains('here'));
const houses=[...crumbs.querySelectorAll('svg')];
check(houses.length===1&&isAHouse(houses[0]),'the path holds '+houses.length+' houses');
const step=houses[0]&&houses[0].parentElement;
check(step&&step.textContent==='ubuntu'&&step.firstChild===houses[0],'the house is not before the home folder\'s name');
check(step&&step.title==='Home folder','the home folder\'s part does not say Home folder on hover');
check(houses[0]&&box(houses[0]).top>=box(crumbs).top&&box(houses[0]).bottom<=box(crumbs).bottom,'the house sits outside the path bar');
check(houses[0]&&Math.abs((box(houses[0]).top+box(houses[0]).bottom)/2-(box(step).top+box(step).bottom)/2)<3,'the house is not level with its name');
check(steps.filter(one=>one.title).length===1,'another part says something on hover');
"#));
}

#[test]
fn c5_the_range_help_opens_inside_the_settings_it_scrolls_in() {
    draw_keeping("C5", (1280, 560), &["render-settings.js"], r#"
import * as rs from './render-settings.js';
host.style.display='none';
const cols=document.createElement('div');cols.style.cssText='height:265px;overflow-y:auto;position:relative;width:600px;padding:0 22px';document.body.prepend(cols);
const filler=document.createElement('div');filler.style.height='230px';cols.append(filler);
const line=document.createElement('div');cols.append(line);
const tip=rs.theTip({how_to_type_a_range:'How to type a range',the_range_forms:[{value:'1-240',said:'frames 1 to 240'},{value:'1,5,9',said:'only those frames'},{value:'1-240x10',said:'every tenth frame'},{value:'7',said:'frame 7 only'}]});
line.append(tip);
tip.querySelector('.q').focus();
const bubble=tip.querySelector('.bubble');
check(box(bubble).height>0,'the help did not open');
check(box(bubble).bottom<=box(cols).bottom+1,'the help opens '+Math.round(box(bubble).bottom-box(cols).bottom)+' pixels below the visible settings');
check(getComputedStyle(bubble,'::before').height==='8px','no bridge over the gap between the mark and its help');
"#);
}

/// **D-11: a folder of several formats keeps each count on its own format's
/// line, at the folder's top** (`Design/Render Settings § What it will
/// write`, 2026-11-12). The scout's folder at the list's width at 1180, its
/// path wrapping to several lines and *Multilayer EXR* wrapping at its space:
/// each count's top is its format's, the first at the folder's top and the
/// second straight under the first format, not halfway down the folder.
#[test]
fn d11_a_folders_counts_sit_at_its_top_each_beside_its_format() {
    draw_keeping("D-11", (1180, 800), &["render-settings.js"], r#"
import * as rs from './render-settings.js';
host.style.display='none';
const writes=document.createElement('div');writes.className='writes';writes.style.width='265px';document.body.prepend(writes);
rs.aRowIsDrawnInto(writes,{place:'Q_012/EXR/04. Lamps + Shades/Kt + Fx + Gl + Luma + Orb - Hero Product/',the_top:null,counts:[{how_many:'240',format:'Multilayer EXR'},{how_many:'240',format:'PNG'}],group:null},false);
const place=writes.querySelector('.p'), n=[...writes.querySelectorAll('.n')], f=[...writes.querySelectorAll('.f')];
check(box(place).height>box(n[0]).height*3,'the folder did not wrap, so nothing was proved: '+box(place).height);
check(box(f[0]).height>box(n[0]).height*1.5,'Multilayer EXR did not wrap, so nothing was proved');
n.forEach((one,at)=>check(Math.abs(box(one).top-box(f[at]).top)<2,'count '+at+' is not on its format\'s line: '+box(one).top+' against '+box(f[at]).top));
check(Math.abs(box(n[0]).top-box(place).top)<2,'the first count is not at the folder\'s top');
check(box(n[1]).top-box(f[0]).bottom<10,'the second count sits '+Math.round(box(n[1]).top-box(f[0]).bottom)+' pixels under the first format, beside another part of the folder');
"#);
}

/// Telegram's section, drawn by `web/telegram.js` at the given width.
const TELEGRAM: &str = r#"
import * as tgm from './telegram.js';
const theTelegram=async(width,said)=>{host.style.display='none';const sec=document.createElement('section');sec.className='stgs';sec.style.width=width+'px';document.body.prepend(sec);
 tgm.theTelegramSectionIsBuilt(sec,document.body);tell('telegram:the-section',{heading:'Telegram',...said});return sec;};
const on={switchedOn:'yes',theSwitch:'Report to Telegram',bot:'Bot',theBotsUsername:'@a_very_long_render_farm_name_bot',useAnotherBot:'Change bot or token…',token:'Token',theTokenIsHeld:'••••••••••••••••••••',whatAJobReports:'What a job reports',
 choiceIs0:'everything',choiceName0:'Everything',choiceWhat0:'progress, frames, a short video and a contact sheet',choiceIs1:'pictures',choiceName1:'Pictures only',choiceWhat1:'the same, without server status or sound at job start',
 choiceIs2:'minutes',choiceName2:'A picture every',choiceWhat2:'the same as Everything, with fewer pictures and no sound at job start',choiceIs3:'alerts',choiceName3:'Alerts only',choiceWhat3:'endings and questions',choiceIs4:'off',choiceName4:'Off',choiceWhat4:'no automatic job messages',
 theMinutesChoiceIs:'minutes',minutes:'minutes',theDefaultMinutes:'10',theDefaultIs:'everything'};
"#;

#[test]
fn e03_telegrams_button_and_server_names_keep_together() {
    draw("E-03", (690, 1000), &format!("{TELEGRAM}{}", r#"
const names=['4x4090','2x5090','render-node-with-an-extremely-long-name-for-the-studio-farm-number-0042','Сервер для финального рендера','東京レンダーファーム','a','b-node','c-node-03','d-node-04','e-node-05','f-node-06','g-node-07'];
const said={...on,notReachedCount:String(names.length),notReachedLead:'servers without this change yet:',notReachedTail:'Sent when you next connect to them.'};
names.forEach((name,at)=>said['notReached'+at]=name);
const sec=await theTelegram(454,said);
const button=[...sec.querySelectorAll('button')].find(b=>b.textContent==='Change bot or token…');
const note=sec.querySelector('.sunote');
const linesOf=(range)=>new Set([...range.getClientRects()].filter(r=>r.width>0).map(r=>Math.round(r.top))).size;
for (let width=380; width<=470; width+=6) {
  sec.style.width=width+'px';
  const r=document.createRange();r.selectNodeContents(button);check(linesOf(r)===1,'Change bot or token… wraps at '+width);
  const text=note.textContent;
  for (const name of names.filter(n=>n.length<30)) {
    const walk=document.createTreeWalker(note,NodeFilter.SHOW_TEXT);let n, found=null;const start=text.indexOf(name);
    let seen=0;const range=document.createRange();let set=0;
    while((n=walk.nextNode())){const len=n.textContent.length;if(!set&&start<seen+len){range.setStart(n,start-seen);set=1;}if(set&&start+name.length<=seen+len){range.setEnd(n,start+name.length-seen);break;}seen+=len;}
    check(linesOf(range)===1,name+' breaks across two lines at '+width);
  }
}
"#));
}

#[test]
fn e04_telegrams_choices_and_boxes_line_up() {
    draw("E-04", (1280, 800), &format!("{TELEGRAM}{}", r#"
const sec=await theTelegram(700,on);
const lefts=[...sec.querySelectorAll('.fmt.choice3 .fmw')].map(w=>Math.round(box(w).left));
check(lefts.length===5 && Math.max(...lefts)-Math.min(...lefts)<=1,'the descriptions start at '+lefts.join(', '));
const boxes=[...sec.querySelectorAll('.sucard .stgrow > .path')].filter(p=>!p.hidden).map(p=>Math.round(box(p).left));
check(boxes.length===2 && boxes[0]===boxes[1],'Bot and Token start at '+boxes.join(' and '));
"#));
}

#[test]
fn e06_settings_draw_no_empty_frame() {
    draw_keeping("E-06", (1280, 800), &["settings.js"], &format!("{TELEGRAM}{}", r#"
import * as settings from './settings.js';
const sec=await theTelegram(700,{theSettingsWouldNotRead:'Could not read your Telegram settings.'});
const shown=(e)=>box(e).height>0&&box(e).width>0;
const empty=(within)=>[...within.querySelectorAll('.cmp, .folders')].filter(f=>shown(f)&&f.innerText.trim()===''&&![...f.querySelectorAll('button, input')].some(shown));
check(empty(sec).length===0,'Telegram leaves an empty frame where the switch was');
sec.remove();
const page=document.createElement('div');page.style.cssText='width:1066px;height:760px;display:flex;flex-direction:column';document.body.prepend(page);
screen('settings').itIsDrawn(page);
settings.theSettingsSectionIsOpened(settings.TRANSFERS_SECTION);
tell('settings:the-screen',{downloadFolder:'Download folder',speedLimits:'Speed limits',theSettingsWouldNotRead:'Could not read settings.',eachCapIsATotal:'Each cap limits all transfers on all servers combined.',emptyMeansNoCap:'Empty means no cap.'});
check(empty(page).length===0,'Transfers leaves an empty frame where the caps were');
settings.theSettingsSectionIsOpened(settings.SEARCH_FOLDERS_SECTION);
tell('settings:the-search-folders',{heading:'Search folders',addAFolder:'Add a folder…',noSearchFoldersYet:'No search folders yet.',folderCount:'0'});
check(empty(page).length===0,'an empty folder list is drawn as a line');
check(page.textContent.includes('No search folders yet.'),'an empty folder list says nothing');
"#));
}

#[test]
fn e11_the_install_box_keeps_install_and_cancel_in_view() {
    draw("E-11", (1600, 560), r#"
import * as su from './set-up.js';
host.style.display='none';
const modal=document.createElement('div');modal.className='modal';document.body.prepend(modal);
const mbox=document.createElement('div');mbox.className='mbox';modal.append(mbox);
const entry=(v)=>({version:v,line:'Released 2026',mark:null,asked:v});
su.theInstallBoxIsDrawn(mbox,{title:'Install Blender',source:'From blender.org',finished:Array.from({length:12},(_,i)=>entry('5.'+i+'.0')),unfinished:[entry('5.4.0-alpha'),entry('5.3.1-rc'),entry('5.3.0-beta')],the_heading:'Experimental builds',or:'Or',choose_a_build:'choose a build',what_smoozy_checks:'Smoozy checks it first.',cancel:'Cancel',install:'Install',versions:'versions'},'made-up');
for (const word of ['Install','Cancel']) {
  const b=[...mbox.querySelectorAll('button')].find(one=>one.textContent===word);
  check(b && box(b).bottom<=box(mbox).bottom && box(b).bottom<=innerHeight,word+' is below the box\'s visible part');
}
"#);
}

/// **The Creator's notes on the test copy, second round, notes 13 and 16: the
/// dependency check's *Stop* is the window's small button, and its turning
/// mark sits on its line's text** — `Design/Dependency Upload & Remap § While
/// it is still reading below a linked file`, as changed 2026-11-13. The real
/// check page draws its header's line while it reads a linked file, and its
/// tools while it searches: each *Stop* is the small button Transfers' *Stop*
/// is (`.mini`), held to the line's height; the mark is the turning mark
/// Render Settings' read line draws, its middle within a pixel and a half of
/// the middle of the words beside it.
#[test]
fn the_checks_stop_is_a_small_button_and_its_mark_sits_on_its_line() {
    draw_keeping("check stop and mark", (1000, 200), &["the-dependency-check.js"], r#"
import * as ck from './the-dependency-check.js';
host.style.display='none';
const panel=document.createElement('div');panel.dataset.panel='chk';panel.style.cssText='padding:16px;width:900px';document.body.prepend(panel);
const head=document.createElement('div');head.className='shead';panel.append(head);
const said={title:words('Dependencies — ',),show_in_files:null,
 the_reading_line:[words('Read with Blender 5.3.0')],
 still_reading:{pieces:[{said:'Reading'},{value:'shelf_v3.blend'}]},the_line_counts:false,
 the_walks_stop:'Stop'};
ck.theHeadIsDrawn('s1',said,head);
const line=head.querySelector('.sub');
const stop=line&&[...line.querySelectorAll('button')].find(one=>one.textContent==='Stop');
check(stop&&stop.classList.contains('mini'),'the header\'s Stop is not the small button: '+(stop?stop.className:'none'));
const mark=line&&line.querySelector('.spin');
check(mark&&box(mark).width>=8&&box(mark).width<=14,'no turning mark of the read line\'s size on the header\'s line');
const wordsBox=line&&[...line.querySelectorAll('span')].find(one=>one.textContent.startsWith('Reading'));
if(mark&&wordsBox){const r=document.createRange();r.selectNodeContents(wordsBox.firstChild||wordsBox);const text=r.getClientRects()[0]||box(wordsBox);
 const mid=(box(mark).top+box(mark).bottom)/2, textMid=(text.top+text.bottom)/2;
 check(Math.abs(mid-textMid)<=1.5,'the mark sits '+(mid-textMid).toFixed(1)+' pixels off the middle of its words');}
check(line&&box(line).height<=24,'the header\'s line grew to '+(line?box(line).height:0)+' pixels');
const bar=ck.theTools('s1',{the_tools:{how_far:words('Searching your folders — 3 of 4 done'),stop:'Stop',unreadable:[],search_folders:'Search folders…'}},{});
panel.append(bar);
const tstop=[...bar.querySelectorAll('button')].find(one=>one.textContent==='Stop');
check(tstop&&tstop.classList.contains('mini'),'the search\'s Stop is not the small button: '+(tstop?tstop.className:'none'));
"#);
}

/// **Render Settings' read-line *Stop* is the same small button as the
/// dependency check's** — `Design/Render Settings § A read can take longer`,
/// as changed 2026-11-13 (the runner's ruling in `The test copy's notes,
/// second round — what landed.md`). The real page draws its header while a
/// read runs: *Stop* is `.mini.inl`, held to the line's height; while the stop
/// is under way it reads *Stopping…*, the same button, unavailable.
#[test]
fn the_read_lines_stop_is_the_checks_small_button() {
    draw_keeping("read stop", (1000, 200), &["render-settings.js"], r#"
import * as rs from './render-settings.js';
host.style.display='none';
const panel=document.createElement('div');panel.dataset.panel='res';panel.style.cssText='padding:16px;width:900px';document.body.prepend(panel);
const said=(stopping)=>({title:{lead:'Render settings —',values:[]},blender:'Blender',the_version_control_is_drawn:false,
 no_blender:[],read_at:[],a_reading_stands:false,read_the_file:'Read the file',read_again:'Read again',reset_all:'Reset all',
 the_read_is_offered:true,the_read_is_underway:true,reading_the_file:'Reading the file',reading_the_walk:'Reading frame 120 of 250',
 the_read_elapsed_seconds:42,stop_read:'Stop',stopping_read:'Stopping…',the_read_is_stopping:stopping,
 the_mismatch:null,the_pick_line:null,the_version_line_held:[],the_search_marks:[]});
for (const stopping of [false,true]) {
  const head=document.createElement('div');head.className='shead';panel.append(head);
  rs.theHeaderIsDrawn('s1',said(stopping),head);
  const line=head.querySelector('.rdline');
  const stop=line&&line.querySelector('[data-box="stop-read"]');
  const word=stopping?'Stopping…':'Stop';
  check(stop&&stop.textContent===word,'the read line has no '+word);
  check(stop&&stop.classList.contains('mini')&&stop.classList.contains('inl'),'the read line\'s '+word+' is not the check\'s small button: '+(stop?stop.className:'none'));
  check(stop&&stop.disabled===stopping,'the read line\'s '+word+' is '+(stop&&stop.disabled?'':'not ')+'unavailable');
  check(line&&box(line).height<=24,'the read line grew to '+(line?box(line).height:0)+' pixels');
}
"#);
}

// ── UI consistency, 2026-11-13 ───────────────────────────────────────────
//
// `Audits/Build/UI consistency — the check, 2026-11-13.md`: things that
// jumped, slid or did not line up. Each test names its finding; each failed
// on the page as it stood before the fixes (`SMOOZY_UI_SIZE_WEB` pointed at
// a copy of `web/` from before), as `UI consistency, the fixes — what
// landed.md` records. Names in them are made up.

/// A request as `web/transfers.js` draws one, with the given files and
/// control, for the Transfers findings.
const A_TRANSFER: &str = r#"
const aFileOf=(path,more)=>({path,state:'',staged:'12582912',expected:'3865470566',staged_size:'12 MB',expected_size:'3.6 GB',of:'of',tail:null,note:null,failure:null,stop:'Stop',...more});
const aRequestOf=(id,more)=>({id,terminal:false,tag_class:'up',arrow:'↑',tag:'Upload',name:{pieces:[{value:'kitchen_v14.blend'},{said:'into'},{value:'/root'}]},name_leads:false,name_ends:false,
 count:words('3 / 12'),ending_class:null,collapsed:false,where_it_goes:{pieces:[{said:'To'},{value:'vast 5080'}]},badge:null,control:{label:'Stop',act:'stop'},also:null,
 started_label:'Started',started_at_millis:'1762988440000',bar:{max:100,value:40},lines:[],files:[aFileOf('frame_0001.exr')],server:'s1',...more});
"#;

#[test]
fn ui01_a_servers_name_starts_where_a_places_name_does() {
    draw("UI-01", (1180, 600), r#"
host.style.display='none';
const rail=(marks)=>{const shell=app(marks);const r=document.createElement('nav');r.className='rail';r.style.width='214px';shell.append(r);
 const sec=document.createElement('div');sec.className='rsec';r.append(sec);
 const place=document.createElement('button');place.className='item';place.innerHTML='<svg class="ric" viewBox="0 0 16 16"><rect width="16" height="16"></rect></svg><span class="lbl">Queue</span>';
 const server=document.createElement('button');server.className='item';server.innerHTML='<span class="badge b1"></span><span class="lbl">Night box</span><span class="rdot"></span>';
 sec.append(place,server);return {place,server};};
const wide=rail('');
const at=(one)=>box(one.querySelector('.lbl')).left;
check(Math.abs(at(wide.place)-at(wide.server))<0.5,'a server name starts at '+at(wide.server)+', a place name at '+at(wide.place));
const narrow=rail('s1 s2 s3 s4');
check(Math.abs(box(narrow.place).height-box(narrow.server).height)<0.5,'at the narrowest a server row is '+box(narrow.server).height+' and a place row '+box(narrow.place).height);
"#);
}

#[test]
fn ui02_every_screens_title_sits_at_one_place() {
    draw("UI-02", (1180, 400), r#"
host.style.display='none';
const pane=(cls,inner)=>{const p=document.createElement('div');p.style.cssText='width:800px;display:flex;flex-direction:column';document.body.prepend(p);const h=document.createElement('div');h.className=cls;h.innerHTML=inner;p.append(h);return h;};
// The button sits in its holder, as `web/servers.js` draws it (Sol's point 5).
const servers=pane('svhead','<h1>Servers</h1><span class="grow"></span><span><button type="button" class="btn">Add server…</button></span>');
const queue=pane('qhead','<h1>Queue</h1>');
const top=(h)=>box(h.querySelector('h1')).top-box(h).top;
check(Math.abs(top(servers)-top(queue))<0.5,'the title sits '+top(servers)+' down on All servers and '+top(queue)+' on Queue');
check(Math.abs(box(servers).height-box(queue).height)<0.5,'the header is '+box(servers).height+' high on All servers and '+box(queue).height+' on Queue');
"#);
}

#[test]
fn ui03_a_rendering_rows_name_keeps_its_width_as_its_state_flips() {
    draw("UI-03", (1180, 600), r#"
import './the-queue.js';
host.style.display='none';
const view=document.createElement('div');view.style.cssText='width:1100px;height:560px;display:flex;flex-direction:column';document.body.prepend(view);
screen('queue').itIsDrawn(view);
const said=(state,held)=>({theHeading:'Queue',lineCount:'1',line0Server:'s1',line0Name:'4000ada',rowCount:'1',row0Job:'j1',row0Line:'0',
 row0Name:'a_long_job_name_that_is_cut_short_somewhere_in_its_row_v14',row0State:state,row0StateIs:'run',row0StateHeld:held,row0Renderlet:'40',row0Whole:'20'});
// The two words differ by a few pixels in the Mac's face and by less in
// Chrome's, so the test makes the other word plainly longer: what is held is
// whatever the application sends.
tell('queue:the-queue',said('Rendering','Finishing the frame'));
const name=()=>view.querySelector('.qrow .qn');
check(name(),'no row was drawn');
const a=name()&&box(name()).width;
tell('queue:the-queue',said('Finishing the frame','Rendering'));
const b=name()&&box(name()).width;
check(Math.abs(a-b)<0.5,'the name is '+a+' wide beside one word and '+b+' beside the word it flips to');
"#);
}

#[test]
fn ui04_blenders_estimate_is_pinned_right_and_keeps_its_room() {
    draw("UI-04", (900, 700), r#"
import './the-queue.js';
host.style.display='none';
const view=document.createElement('div');view.style.cssText='width:800px;height:660px;display:flex;flex-direction:column';document.body.prepend(view);
screen('queue').itIsDrawn(view);
const parts=(estimate)=>{const p={progNow0Lead:'Frame',progNow0Value:'148/240',progNow1Value:'12/16',progNow1Trail:'samples in this tile'};
 if(estimate){p.progNow2Value='~1m 40s';p.progNow2Trail='left';p.progNow3Lead='·';p.progNow4Lead='Blender’s estimate';p.progNowEstimateFrom='2';p.progNowCount='5';}else p.progNowCount='2';return p;};
const said=(estimate,samples)=>({theHeading:'Queue',lineCount:'1',line0Server:'s1',line0Name:'4000ada',rowCount:'1',row0Job:'j1',row0Line:'0',row0Name:'cloth',row0State:'Rendering',row0StateIs:'run',
 entryJob:'j1',entryName:'cloth',renderingNow:'Rendering now',wholeJob:'Whole job',progNowAt:'40',progWhole:'20',...parts(estimate),progNow1Value:samples});
tell('queue:the-queue',said(true,'12/16'));
const line=()=>view.querySelector('.pline'), bar=()=>view.querySelector('.bar.big');
check(line()&&bar(),'no Rendering now line was drawn');
if(line()&&bar()){
 const est=[...line().children].pop();
 check(Math.abs(box(est).right-box(line()).right)<1,'the estimate ends '+Math.round(box(line()).right-box(est).right)+' pixels short of the line\'s end');
 const barAt=box(bar()).top, estAt=box(est).left;
 tell('queue:the-queue',said(true,'120/1024'));
 const est2=[...line().children].pop();
 check(Math.abs(box(est2).left-estAt)<1,'the estimate slid '+Math.round(box(est2).left-estAt)+' pixels when the sample count grew');
 tell('queue:the-queue',said(false,'120/1024'));
 check(Math.abs(box(bar()).top-barAt)<0.5,'the bar moved '+Math.round(box(bar()).top-barAt)+' pixels when Blender\'s newest line carried no estimate');
}
"#);
}

#[test]
fn ui05_a_files_figure_has_even_digits_and_the_requests_wording() {
    draw_keeping("UI-05", (1180, 300), &["transfers.js"], &format!("{A_TRANSFER}{}", r#"
import * as transfers from './transfers.js';
host.style.display='none';
const holder=document.createElement('div');holder.id='tscroll';holder.style.width='1000px';document.body.prepend(holder);
holder.append(transfers.aRequest(aRequestOf('r1')));
const figure=holder.querySelector('.tf .tm .rqs');
check(figure,'the file has no figure in its last column');
check(figure&&figure.textContent.includes('12 MB')&&figure.textContent.includes('3.6 GB'),'the figure reads '+(figure&&figure.textContent)+', not as Files words a size');
check(figure&&getComputedStyle(figure).fontVariantNumeric.includes('tabular-nums'),'the figure\'s digits are not of one width');
import {theCountdown} from './the-underway-shape.js';
const count=theCountdown(Date.now()/1000+12,{in:'retrying in',now:'retrying now',unit:'s'});holder.append(count);
check(getComputedStyle(count).fontVariantNumeric.includes('tabular-nums'),'retrying in 12s has digits of uneven width (finding 51)');
"#));
}

#[test]
fn ui06_a_redraw_keeps_a_files_menu_open_and_moves_no_row() {
    draw_keeping("UI-06", (1180, 600), &["transfers.js"], &format!("{A_TRANSFER}{}", r#"
import * as transfers from './transfers.js';
host.style.display='none';
const view=document.createElement('div');view.style.cssText='width:1000px;height:560px;display:flex;flex-direction:column';document.body.prepend(view);
screen('transfers').itIsDrawn(view);
const said={heading:'Transfers',speeds:null,caps:words('No caps'),settings:'Settings',show:'Show',every_server:'All servers',servers:[],showing:'',
 requests:[aRequestOf('r1',{files:[aFileOf('frame_0001.exr'),aFileOf('frame_0002.exr')]}),aRequestOf('r2')],foot:words('2 requests'),the_rails_figure:''};
transfers.theScreenIsDrawn(said);
const second=()=>view.querySelector('[data-request="r2"]');
const before=box(second()).top;
const row=view.querySelector('.tf');row.dispatchEvent(new MouseEvent('contextmenu',{bubbles:true,cancelable:true}));
const menu=()=>view.querySelector('.transfers-ctx');
check(menu(),'no menu opened');
check(Math.abs(box(second()).top-before)<0.5,'opening the menu pushed the next request down '+Math.round(box(second()).top-before)+' pixels');
const stop=menu()&&menu().querySelector('button');if(stop)stop.focus();
transfers.theScreenIsDrawn(said);
check(menu()&&menu().isConnected,'the menu went when the list was drawn again');
check(document.activeElement&&document.activeElement.closest('.transfers-ctx'),'the keyboard\'s place on Stop went when the list was drawn again');
"#));
}

#[test]
fn ui07_a_requests_count_sits_at_the_headers_right_end() {
    draw_keeping("UI-07", (1180, 300), &["transfers.js"], &format!("{A_TRANSFER}{}", r#"
import * as transfers from './transfers.js';
host.style.display='none';
const holder=document.createElement('div');holder.id='tscroll';holder.style.width='1000px';document.body.prepend(holder);
const one=transfers.aRequest(aRequestOf('r1'));
const two=transfers.aRequest(aRequestOf('r2',{tag_class:'auto',tag:'Auto-download',control:{label:'Turn off',act:'turn-off'},also:{label:'Download remaining',act:'download-remaining'}}));
holder.append(one,two);
const count=(r)=>box(r.querySelector('.rqm')).right;
check(Math.abs(count(one)-count(two))<0.5,'the counts end at '+count(one)+' and '+count(two));
"#));
}

#[test]
fn ui20_ui41_ui42_file_bars_and_stripes_are_the_drawings() {
    draw_keeping("UI-20", (1180, 600), &["transfers.js"], &format!("{A_TRANSFER}{}", r#"
import * as transfers from './transfers.js';
host.style.display='none';
const view=document.createElement('div');view.style.cssText='width:1000px;height:560px;display:flex;flex-direction:column';document.body.prepend(view);
screen('transfers').itIsDrawn(view);
const files=[aFileOf('frame_0001.exr'),aFileOf('frame_0002.exr',{state:'waitpart',tail:'uploaded',stop:null}),aFileOf('frame_0003.exr',{state:'waiting',staged:null,expected:null,staged_size:null,expected_size:null,tail:'queued'})];
transfers.theScreenIsDrawn({heading:'Transfers',speeds:null,caps:words('No caps'),settings:'Settings',show:'Show',every_server:'All servers',servers:[],showing:'',
 requests:[aRequestOf('r1',{group_heading:'In progress',files}),aRequestOf('r2'),aRequestOf('r3',{group_heading:'Finished'}),aRequestOf('r4')],foot:words('4 requests'),the_rails_figure:''});
const rows=[...view.querySelector('[data-request="r1"]').querySelectorAll('.tf')];
const bar=(r)=>r.querySelector('.tb');
check(rows.length===3,'the files were not drawn');
if(rows.length===3){
 check(Math.abs(box(bar(rows[0])).height-3)<0.5,'a file\'s bar is '+box(bar(rows[0])).height+' pixels, not the drawing\'s 3 (finding 20)');
 check(getComputedStyle(bar(rows[2])).backgroundColor!=='rgba(0, 0, 0, 0)'&&box(bar(rows[2])).height>0,'a queued file shows no track (finding 20)');
 check(Math.abs(box(bar(rows[1])).width-box(bar(rows[0])).width)<0.5,'a part-sent file\'s bar is '+box(bar(rows[1])).width+' wide beside '+box(bar(rows[0])).width+' (finding 41)');
 const tail=rows[1].querySelector('.tm');
 check(tail&&tail.textContent.includes('12 MB')&&tail.textContent.includes('uploaded'),'a part-sent file\'s figure is not in its last column (finding 41): '+(tail&&tail.textContent));
}
const ground=(id)=>getComputedStyle(view.querySelector('[data-request="'+id+'"]')).backgroundColor;
check(ground('r2')!==ground('r1')&&ground('r4')===ground('r2')&&ground('r3')===ground('r1'),'the stripes count the headings: '+['r1','r2','r3','r4'].map(ground).join(', ')+' (finding 42)');
"#));
}

#[test]
fn ui08_ui43_the_servers_window_beside_the_steps_stays_put() {
    draw("UI-08", (1180, 500), r#"
host.style.display='none';
const card=document.createElement('div');card.style.width='860px';document.body.prepend(card);
card.innerHTML='<div class="suwork"><ol class="steps"><li class="done"><span>Check the server</span></li><li class="now"><span>Download Blender 5.2.2</span> <span class="stm tnum">1 of 352 MB</span></li><li><span>Install Blender</span></li></ol><div class="log"><span class="qlines mono"><span>line one</span></span><button type="button">Server output</button></div></div>';
const log=card.querySelector('.log');const at=box(log).left;
const now=card.querySelector('li.now .stm');now.textContent='214 of 352 MB';
check(Math.abs(box(log).left-at)<0.5,'the window slid '+Math.round(box(log).left-at)+' pixels as the figure ticked');
card.querySelector('li.now').className='done';
check(Math.abs(box(log).left-at)<0.5,'the window slid '+Math.round(box(log).left-at)+' pixels when the step ended');
const note=document.createElement('span');note.className='stnote';note.textContent='The server is restarting. Cancel stops waiting, not the restart. And a few more words so it is long.';
card.querySelector('li:last-child').append(note);
check(Math.abs(box(log).left-at)<0.5&&box(log).top<box(card.querySelector('.steps')).bottom,'the restart note pushed the window aside or under the steps (finding 43)');
"#);
}

#[test]
fn ui10_ui11_files_bar_and_version_tag_sit_where_they_belong() {
    draw("UI-10", (1100, 400), r#"
const bar=document.createElement('div');bar.className='fbar';bar.style.width='1000px';document.body.prepend(bar);
bar.innerHTML='<span>Uploads go to <span class="mono">kitchen/</span></span><span class="grow"></span><span>5 files · 61 GB free</span><label class="mcheck"><input type="checkbox">Show hidden files</label>';
const last=bar.lastElementChild;
check(box(bar).right-box(last).right<30,'Show hidden files ends '+Math.round(box(bar).right-box(last).right)+' pixels from the bar\'s right end (finding 10)');
const entry=files.aRow({...row('kitchen_v14.blend'),saved_with:'5.3'});list.append(entry);
const text=entry.querySelector('.fname-text'), tag=entry.querySelector('.ver');
check(text&&tag&&box(tag).left-box(text).right<12,'the version sits '+(text&&tag?Math.round(box(tag).left-box(text).right):'?')+' pixels after the name (finding 11)');
"#);
}

#[test]
fn ui12_the_checks_sizes_and_dots_line_up_down_the_list() {
    draw("UI-12", (1100, 400), r#"
host.style.display='none';
const rows=document.createElement('div');rows.className='rows';rows.style.width='900px';document.body.prepend(rows);
const one=(acts)=>{const r=document.createElement('div');r.className='row';r.innerHTML='<span class="what"><span class="nm mono">bark.png</span></span><span class="sz mono">31 MB</span><span class="acts">'+acts+'</span>';rows.append(r);return r;};
const held='<span class="fx held" aria-hidden="true">×</span>', more='<button type="button" class="more">⋯</button>';
const a=one('<button type="button" class="a">Retry</button>'+more+'<button type="button" class="fx">×</button>');
const b=one('<button type="button" class="a">Locate…</button>'+more+held);
const c=one(more+held);
const right=(r)=>box(r.querySelector('.sz')).right, dots=(r)=>box(r.querySelector('.more')).left;
check(Math.abs(right(a)-right(b))<0.5&&Math.abs(right(a)-right(c))<0.5,'the sizes end at '+[a,b,c].map(right).join(', '));
check(Math.abs(dots(a)-dots(b))<0.5&&Math.abs(dots(a)-dots(c))<0.5,'the ⋯ start at '+[a,b,c].map(dots).join(', '));
check(getComputedStyle(b.querySelector('.fx.held')).visibility==='hidden','the cross\'s room is seen');
"#);
}

#[test]
fn ui13_historys_frame_counts_line_up() {
    draw("UI-13", (1100, 400), r#"
host.style.display='none';
const scroll=document.createElement('div');scroll.style.width='520px';document.body.prepend(scroll);
const one=(ending,when)=>{const r=document.createElement('button');r.className='hrow';r.innerHTML='<span class="hn">cloth</span><span class="hnum mono">60/240</span><span class="qst e-fail">'+ending+'</span><span class="hm">Night box</span><span class="hnum mono dim">2h 14m</span><span class="hwhen">'+when+'</span>';scroll.append(r);return r;};
const rows=[one('Failed','14:03'),one('Unknown if started','Nov 12, 14:03'),one('Interrupted','09:41')];
const ends=rows.map(r=>Math.round(box(r.querySelector('.hnum')).right*2)/2);
check(new Set(ends).size===1,'the frame counts end at '+ends.join(', '));
check(getComputedStyle(rows[0].querySelector('.hwhen')).fontVariantNumeric.includes('tabular-nums'),'the time\'s digits are uneven');
"#);
}

#[test]
fn ui15_ui16_ui48_a_read_moves_nothing_and_its_answer_sits_at_the_left() {
    draw_keeping("UI-15", (1000, 300), &["render-settings.js"], r#"
import * as rs from './render-settings.js';
host.style.display='none';
const panel=document.createElement('div');panel.dataset.panel='set';panel.style.cssText='padding:16px;width:900px';document.body.prepend(panel);
const said=(reading)=>({title:{lead:'Render settings —',values:[]},blender:'Blender',the_version_control_is_drawn:true,
 no_blender:[],read_at:[],a_reading_stands:true,read_the_file:'Read the file',read_again:'Read again',reset_all:'Reset all',
 the_read_is_offered:true,the_read_is_underway:reading,reading_the_file:'Reading the file',reading_the_walk:reading?'Reading frame 120 of 250':null,
 the_read_elapsed_seconds:reading?42:null,stop_read:'Stop',stopping_read:'Stopping…',the_read_is_stopping:false,
 the_mismatch:null,the_pick_line:'Latest on this server.',the_version_line_held:[],the_search_marks:[],the_versions:[{build:'/opt/blender-5.2.1/blender',answered:'5.2.1',picked:true}],latest_on_this_server:'Latest on this server.'});
const head=(reading)=>{const h=document.createElement('div');h.className='shead';panel.append(h);rs.theHeaderIsDrawn('s1',said(reading),h);return h;};
const still=head(false), reading=head(true);
check(Math.abs(box(still).height-box(reading).height)<0.5,'a read makes the header '+box(reading).height+' high against '+box(still).height+', moving the settings below');
const line=[...reading.querySelectorAll('.rdline')].find(one=>!one.classList.contains('pkheld'));
check(line&&getComputedStyle(line).fontVariantNumeric.includes('tabular-nums'),'the read\'s clock and frame have uneven digits (finding 16)');
const answer=still.querySelector('[data-box="read-answer"]');
check(answer&&getComputedStyle(answer).paddingLeft==='0px','Read stopped. sits '+(answer&&getComputedStyle(answer).paddingLeft)+' in from the header\'s left edge (finding 48)');
"#);
}

#[test]
fn ui17_ui18_a_box_grows_downward_and_its_buttons_are_one_height() {
    draw("UI-17", (1000, 700), r#"
host.style.display='none';
const shell=document.createElement('div');shell.style.cssText='position:relative;width:1000px;height:700px';document.body.prepend(shell);
const modal=document.createElement('div');modal.className='modal';shell.append(modal);
const mbox=document.createElement('div');mbox.className='mbox';modal.append(mbox);
mbox.innerHTML='<h3>Add a server</h3><p class="msub">Paste the line your provider gives you.</p><div class="mrow"><span class="grow"></span><button type="button" class="btn quiet">Cancel</button><button type="button" class="btn holds" data-held="Add and set up">Add</button></div>';
const top=box(mbox).top, cancel=mbox.querySelector('.quiet'), add=mbox.querySelector('.holds');
const more=document.createElement('p');more.className='msub';more.textContent='A second line that grows the box.';mbox.insertBefore(more,mbox.querySelector('.mrow'));
check(Math.abs(box(mbox).top-top)<0.5,'the box\'s top moved '+Math.round(box(mbox).top-top)+' pixels as it grew (finding 17)');
check(Math.abs(box(cancel).height-box(add).height)<0.5,'Cancel is '+box(cancel).height+' high beside '+box(add).height+' (finding 18)');
const at=box(cancel).left;add.textContent='Add and set up';
check(Math.abs(box(cancel).left-at)<0.5,'Cancel slid '+Math.round(box(cancel).left-at)+' pixels when Add read Add and set up (finding 18)');
const row=document.createElement('div');row.className='mact';row.innerHTML='<button type="button" class="btn quiet">Cancel</button>';mbox.append(row);
const alone=box(row.firstElementChild).height;
check(Math.abs(alone-box(add).height)<0.5,'Cancel alone is '+alone+' high, '+box(add).height+' beside a main button (finding 18)');
"#);
}

#[test]
fn ui19_ui61_the_last_switch_draws_no_second_line_and_its_answer_lines_up() {
    draw("UI-19", (900, 400), r#"
host.style.display='none';
const sec=document.createElement('section');sec.className='stgs';sec.style.width='600px';document.body.prepend(sec);
const cmp=document.createElement('div');cmp.className='cmp';sec.append(cmp);
const one=(name,said)=>{const b=document.createElement('div');b.className='trowbox';b.innerHTML='<div class="trow"><button type="button" class="sws"><i></i></button><span class="tt">'+name+'</span></div><div class="stg-answer"></div>';
 if(said){const p=document.createElement('p');p.textContent=said;b.lastElementChild.append(p);}cmp.append(b);return b;};
one('A job finishes');const last=one('A job fails','Could not save. Try again.');
const row=last.querySelector('.trow');
check(getComputedStyle(row).borderBottomWidth==='0px','the last switch keeps its own line above the box\'s border');
const p=last.querySelector('.stg-answer p');
check(Math.abs(box(p).left-box(row.querySelector('.sws')).left)<0.5,'a failed save starts '+Math.round(box(p).left-box(row.querySelector('.sws')).left)+' pixels from the rows (finding 61)');
"#);
}

#[test]
fn ui21_ui22_the_connect_controls_dot_and_width_hold() {
    draw("UI-21", (1180, 400), r#"
host.style.display='none';
const list=document.createElement('div');list.style.width='900px';document.body.prepend(list);
const acts=(label)=>{const a=document.createElement('span');a.className='svacts';a.innerHTML='<button type="button" class="cbtn"><span class="cdot"></span>'+label+'</button><button type="button" class="more">···</button>';list.append(a);return a;};
const dots=['Connect','Connected','Reconnecting…','Connect and set up'].map(l=>Math.round(box(acts(l).querySelector('.cdot')).left));
check(new Set(dots).size===1,'the dot sits at '+dots.join(', '));
const head=document.createElement('div');head.className='mtop';head.style.width='800px';head.innerHTML='<h1>Night box</h1><span class="addr mono">root@203.0.113.62</span>';list.append(head);
const control=(label)=>{const c=document.createElement('button');c.type='button';c.className='cbtn';c.innerHTML='<span class="cdot"></span>'+label;head.append(c);const w=box(c).width;c.remove();return Math.round(w);};
const widths=['Connect','Connected','Reconnecting…'].map(control);
check(new Set(widths).size===1,'the page\'s control is '+widths.join(', ')+' wide (finding 22)');
"#);
}

#[test]
fn ui23_a_keys_check_mark_takes_no_line() {
    draw("UI-23", (1180, 400), r#"
host.style.display='none';
const list=document.createElement('div');list.className='ky-list';list.style.width='900px';document.body.prepend(list);
const row=(checking)=>{const r=document.createElement('div');r.className='ky-row';r.innerHTML='<div class="ky-main"><span class="ky-name">Studio MacBook Pro</span><span class="ky-fp mono">SHA256:abc</span><span class="ky-where mono">~/.ssh/id_ed25519</span>'+(checking?'<span class="ky-why ky-slot uway" aria-disabled="true"><span class="uwl">Checking</span><i class="spin"></i></span>':'')+'</div><span class="ky-acts"></span>';list.append(r);return r;};
const a=row(true), b=row(false);
check(Math.abs(box(a).height-box(b).height)<0.5,'a row is '+box(a).height+' high while checked and '+box(b).height+' after');
const spin=a.querySelector('.spin');
check(Math.abs(box(spin).left-box(a.querySelector('.ky-where')).left)<12,'the mark sits '+Math.round(box(spin).left-box(a.querySelector('.ky-where')).left)+' pixels right of where the answer appears');
"#);
}

#[test]
fn ui24_ui62_a_cross_and_a_pick_keep_their_room() {
    draw("UI-24", (1180, 400), r#"
host.style.display='none';
const col=document.createElement('div');col.style.width='600px';document.body.prepend(col);
const set=(cross)=>{const s=document.createElement('div');s.className='set';s.innerHTML='<span class="k">Range</span><span class="fld w-list mono">1-240</span>'+(cross?'<button type="button" class="clear">✕</button>':'<span class="clear held" aria-hidden="true">✕</span>')+'<span class="after">240 frames</span>';col.append(s);return s;};
const a=set(true), b=set(false);
check(Math.abs(box(a.querySelector('.after')).left-box(b.querySelector('.after')).left)<0.5,'240 frames moves when a range is typed');
check(getComputedStyle(b.querySelector('.clear')).visibility==='hidden','the held cross is seen');
const pick=(label)=>{const p=document.createElement('span');p.className='pick w-tex';p.innerHTML='<button type="button" class="pkb">'+label+'</button>';col.append(p);return Math.round(box(p).width);};
check(pick('No Limit')===pick('128'),'the Texture Limit dropdown changes width with its pick (finding 62)');
"#);
}

#[test]
fn ui25_ui27_ui28_files_keeps_pastes_room_even_times_and_an_editing_rows_cells() {
    draw("UI-25", (1280, 600), r#"
const bar=document.createElement('div');bar.className='ftools';host.prepend(bar);
bar.innerHTML='<button type="button" class="ib"><svg viewBox="0 0 16 16"><rect width="16" height="16"></rect></svg></button><span class="ib held" aria-hidden="true"></span><span class="tdiv"></span><button type="button" class="ib danger">D</button>';
const room=bar.querySelector('.held');
check(getComputedStyle(room).visibility==='hidden'&&Math.abs(box(room).width-box(bar.firstElementChild).width)<0.5,'Paste\'s room is not an unseen icon button\'s width (finding 25)');
const entry=files.aRow({...row('frame_0001.exr'),changed:'1762900000'});list.append(entry);
check(getComputedStyle(entry.querySelector('.when')).fontVariantNumeric.includes('tabular-nums'),'Changed has uneven digits (finding 27)');
const edit={kind:'rename',path:[1],was:'frame_0001.exr',typed:'frame_0001.exr',refused:null,refusal:null};
files.arrange(host,list,{headers,toolbar:{rename:'Rename'},listing:{is:'rows',rows:[]}},edit);
files.theListIsDrawn({headers:{...headers,a_folder:'Folder'},folder:{shown:'/a',bytes:[47,97]},listing:{is:'rows',rows:[]}});
const renaming=files.anEditingRow({...row('frame_0001.exr'),changed:'1762900000'});list.append(renaming);
const cells=[...renaming.querySelectorAll('.fmeta')].map(one=>one.textContent);
check(cells.length===3&&cells[1]==='2 MB'&&cells[2]!=='—','a row being renamed reads '+JSON.stringify(cells)+' (finding 28)');
const made=files.anEditingRow(null);list.append(made);
const kind=made.querySelector('.fmeta.kind');
check(kind&&kind.textContent==='Folder','a new folder\'s Kind reads '+JSON.stringify(kind&&kind.textContent)+' (finding 28)');
"#);
}

#[test]
fn ui29_a_folder_read_again_is_covered_where_the_list_is_seen() {
    draw("UI-29", (1280, 600), r#"
list.style.height='350px';list.style.flex='none';
files.arrange(host,list,null,null);
const rows=Array.from({length:60},(_,i)=>({...row('file_'+i),path:{shown:'/a/file_'+i,bytes:[i]}}));
const folder={shown:'/a',bytes:[47,97]};
files.theListIsDrawn({headers,folder,listing:{is:'rows',rows}});
list.scrollTop=900;
files.theListIsDrawn({headers,folder,listing:{is:'being-read',said:words('Reading…')}});
const cover=list.querySelector('.fwait');
check(cover,'no cover');
check(cover&&box(cover).top<=box(list).top+1&&box(cover).bottom>=box(list).bottom-1,'the cover spans '+(cover?Math.round(box(cover).top)+' to '+Math.round(box(cover).bottom):'nothing')+' where the list shows '+Math.round(box(list).top)+' to '+Math.round(box(list).bottom));
"#);
}

#[test]
fn ui30_ui46_render_settings_fields_are_one_height() {
    draw("UI-30", (1180, 400), r#"
host.style.display='none';
const foot=document.createElement('div');foot.className='sfoot';foot.style.width='1000px';document.body.prepend(foot);
foot.innerHTML='<div class="field"><label>Name this job</label><input type="text" class="fake" value="Kitchen — final"><span class="fnote">Shows in the queue</span></div>';
const name=foot.querySelector('input.fake');
const set=document.createElement('div');set.className='set';set.innerHTML='<span class="k">Samples</span><input type="text" class="fld w-num mono" value="512">';document.body.prepend(set);
const field=set.querySelector('input');
check(Math.abs(box(name).height-box(field).height)<0.5,'the job name field is '+box(name).height+' high and a field '+box(field).height);
check(getComputedStyle(name).color===getComputedStyle(document.body).color,'the typed name is not in ordinary ink: '+getComputedStyle(name).color);
const greyed=document.createElement('div');greyed.className='set';greyed.innerHTML='<span class="k">Samples</span><span class="fld w-num mono"></span>';document.body.prepend(greyed);
check(Math.abs(box(greyed).height-box(set).height)<0.5,'a greyed row is '+box(greyed).height+' high against '+box(set).height+' (finding 46)');
"#);
}

#[test]
fn ui31_ui50_the_minutes_row_and_word_match_the_others() {
    draw_keeping("UI-31", (1280, 800), &[], &format!("{TELEGRAM}{}", r#"
const sec=await theTelegram(700,{...on,theDefaultIs:'minutes'});
const rows=[...sec.querySelectorAll('.fmt.choice3')].filter(r=>box(r).height>0);
// **The minutes field grows neither its name nor its row.** A description
// wraps where the face is wide (Linux's, hosted run 43727887590: the third
// choice's took two lines at 700), as the drawing lets it, so a row is
// measured against its own tallest part: every name is one height, and
// every row is that part plus the same padding. Where the descriptions all
// take one line, the five rows are one height, as before.
const part=(r,which)=>box(r.querySelector(which)).height;
const names=rows.map(r=>Math.round(part(r,'.fmn')));
check(rows.length===5&&new Set(names).size===1,'the five names are '+names.join(', ')+' high');
const over=rows.map(r=>Math.round(box(r).height-Math.max(part(r,'.fmn'),part(r,'.fmw'))));
check(new Set(over).size===1,'the five choices are '+over.join(', ')+' taller than their tallest part');
const oneLine=Math.min(...rows.map(r=>part(r,'.fmw')));
const heights=rows.filter(r=>part(r,'.fmw')<oneLine+1).map(r=>Math.round(box(r).height));
check(heights.length>=4&&new Set(heights).size===1,'the choices with one line of description are '+heights.join(', ')+' high');
const line=document.createElement('div');line.className='sftg';line.innerHTML='<input class="fld w-frame mono" value="5"><span class="hintl">minutes</span>';document.body.append(line);
const word=line.querySelector('.hintl');
check(getComputedStyle(word).fontSize==='11.5px','the word minutes is '+getComputedStyle(word).fontSize+' outside Render Settings\' foot (finding 50)');
"#));
}

#[test]
fn ui32_ui35_ui36_the_preview_under_the_frame_row_holds_still() {
    draw("UI-32", (1180, 500), r#"
host.style.display='none';
const side=document.createElement('aside');side.className='qdside';side.style.width='234px';document.body.prepend(side);
side.innerHTML='<div class="stepper pvstep"><span class="pvarrows"><button type="button" class="sb" data-pvgo="-1">‹</button><label class="sn pvframe">Frame <input class="num mono" value="147"></label><button type="button" class="sb" data-pvgo="1">›</button></span><button type="button" class="mini" data-pvlatest>Show latest</button></div>'
 +'<div class="pvline"><p class="pverror" hidden>Frame 9 is not available.</p><p class="pvshown">Showing frame 147</p></div>'
 +'<div class="pvvlrow"><span class="sn">View Layer</span><span class="pick pvvl"><button type="button" class="pkb">Interior</button></span></div>'
 +'<div class="pvvlrow"><span class="sn">View Layer</span><span class="pvone">Interior</span></div><p class="bar-note">Not colour-accurate.</p>';
const [several,alone]=side.querySelectorAll('.pvvlrow');
check(Math.abs(box(several).height-box(alone).height)<0.5,'one View Layer\'s row is '+box(alone).height+' high, the dropdown\'s '+box(several).height);
const note=side.querySelector('.bar-note');const at=box(note).top;
side.querySelector('.pverror').hidden=false;
check(Math.abs(box(note).top-at)<0.5,'a refused frame number moved what is below by '+Math.round(box(note).top-at));
const latest=side.querySelector('[data-pvlatest]'), arrows=side.querySelector('.pvarrows');
check(box(latest).top===box(arrows).top||Math.abs(box(latest).left-box(side).left)<0.5,'Show latest dropped under the arrows '+Math.round(box(latest).left-box(side).left)+' pixels in (finding 35)');
const input=side.querySelector('.pvframe input'), arrow=side.querySelector('.sb');
check(Math.abs(box(input).height-box(arrow).height)<0.5&&getComputedStyle(input).borderTopLeftRadius!=='0px','the frame field is not drawn as Smoozy\'s fields are (finding 36)');
"#);
}

#[test]
fn ui33_ui34_a_connection_line_moves_nothing() {
    draw("UI-33", (1180, 500), r#"
host.style.display='none';
const head=document.createElement('div');head.className='mhead';head.style.width='900px';document.body.prepend(head);
head.innerHTML='<div class="mtop"><h1>Night box</h1></div><p class="mconnectnote" hidden>Could not connect to Night box.</p><div class="hw"><span>1 × RTX 4090</span></div><div class="tabs"><button type="button" class="tab">Files</button></div>';
const tabs=head.querySelector('.tabs');const at=box(tabs).top;
head.querySelector('.mconnectnote').hidden=false;
check(Math.abs(box(tabs).top-at)<0.5,'a line about the connection pushed the page down '+Math.round(box(tabs).top-at)+' pixels');
"#);
}

#[test]
fn ui38_ui39_ui40_working_and_unavailable_controls_look_one_way() {
    draw("UI-38", (1180, 400), r#"
import {theFailureShape, EXPANDED} from './the-failure-shape.js';
host.style.display='none';
const holder=document.createElement('div');holder.style.width='800px';document.body.prepend(holder);
const btn=document.createElement('button');btn.className='btn';btn.textContent='Install';btn.disabled=true;holder.append(btn);
check(getComputedStyle(btn).opacity==='0.45','a filled button that cannot be pressed looks live (finding 38)');
const go=document.createElement('button');go.id='stgbotgo';go.className='btn uway';go.disabled=true;go.innerHTML='<span class="uwl">Check bot</span><i class="spin"></i>';holder.append(go);
check(getComputedStyle(go).opacity==='1','Check bot fades while it works (finding 38)');
const spins=[['qst','<span class="qst">Submitting…<i class="spin"></i></span>'],['rdline','<div class="sub rdline"><i class="spin"></i>Reading the file</div>']].map(([k,html])=>{const d=document.createElement('div');d.innerHTML=html;holder.append(d);const s=d.querySelector('.spin');return Math.round(box(s).width)+'/'+getComputedStyle(s).verticalAlign+'/'+(parseFloat(getComputedStyle(s).marginLeft)+parseFloat(getComputedStyle(s).marginRight));});
check(new Set(spins).size===1,'the turning marks differ: '+spins.join(' against ')+' (finding 39)');
const shape=theFailureShape({sentence:'Could not reach the server.',kind:'condition',controls:[{is:'try-again',label:'Try again',act:'a'},{is:'raw-text',label:'Blender output',act:'b'}],room:EXPANDED});holder.append(shape);
const [again,words]=shape.querySelectorAll('.failure-acts > button');
check(again&&words&&Math.abs(box(again).height-box(words).height)<0.5,'Try again is '+(again&&box(again).height)+' high beside '+(words&&box(words).height)+' (finding 40)');
const said=shape.querySelector('.failure-said b')||shape.querySelector('b');
check(again&&said&&Math.abs(box(again).left-box(said).left)<1,'the controls start '+(again&&said?Math.round(box(again).left-box(said).left):'?')+' pixels from the sentence\'s words (finding 40)');
"#);
}

#[test]
fn ui45_ui47_ui49_ui53_controls_and_answers_keep_their_lines() {
    draw("UI-45", (1180, 500), r#"
host.style.display='none';
const col=document.createElement('div');col.style.width='900px';document.body.prepend(col);
const row=document.createElement('div');row.className='stgrow';row.innerHTML='<button type="button" class="btn quiet holds" data-held="Cancelling…">Cancel setup</button><button type="button" class="btn quiet">Abandon setup</button>';col.append(row);
const [cancel,next]=row.children;const at=box(next).left;cancel.textContent='Cancelling…';cancel.dataset.held='Cancel setup';
check(Math.abs(box(next).left-at)<0.5,'the control beside Cancel setup moved '+Math.round(box(next).left-at)+' pixels as it read Cancelling… (finding 45)');
const log=document.createElement('div');log.className='log';log.innerHTML='<span class="qlines mono"><span>a line</span></span><button type="button">Server output</button>';col.append(log);
const mini=document.createElement('button');mini.className='mini';mini.textContent='Server output';col.append(mini);
check(Math.abs(box(log.querySelector('button')).height-box(mini).height)<0.5,'Server output is '+box(log.querySelector('button')).height+' high beside the window and '+box(mini).height+' after the run (finding 45)');
const foot=document.createElement('div');foot.className='sfoot';foot.style.width='640px';foot.innerHTML='<div class="so sfl2"><span class="swline"><button type="button" class="sws"><i></i></button><span>Render one frame first</span></span><p class="alertnote" data-box="submit-answer">Not submitted. The server stopped answering while the job was being sent, and nothing was started on it; try again when it answers.</p></div>';col.append(foot);
const answer=foot.querySelector('[data-box="submit-answer"]'), second=foot.querySelector('.sfl2');
const withIt=box(second).height;answer.hidden=true;const without=box(second).height;answer.hidden=false;
check(Math.abs(withIt-without)<0.5,'a long refusal makes line 2 '+Math.round(withIt-without)+' pixels taller (finding 47)');
const line=(withStop)=>{const p=document.createElement('p');p.className='sub ckline';p.innerHTML='<i class="spin"></i>Searching your folders — 3 of 4 done'+(withStop?' · <button type="button" class="mini inl">Stop</button>':'');col.append(p);return box(p).height;};
check(Math.abs(line(true)-line(false))<0.5,'Stop makes its line '+(line(true)-line(false))+' pixels taller (finding 49)');
const named=(field)=>{const r=document.createElement('div');r.className='svrow';r.innerHTML='<span class="svmain"><span class="svname"><span class="badge b1"></span>'+(field?'<input class="fld w-list" value="Night box">':'<bdi>Night box</bdi>')+'</span></span>';col.append(r);return box(r).height;};
check(Math.abs(named(true)-named(false))<0.5,'renaming makes the row '+(named(true)-named(false))+' pixels taller (finding 53)');
"#);
}

#[test]
fn ui54_ui56_ui57_ui58_scroll_room_sort_marks_and_a_refused_name() {
    draw("UI-54", (1280, 600), r#"
const rail=document.createElement('nav');rail.className='rail';document.body.append(rail);
const sec=document.createElement('div');sec.className='rsec';rail.append(sec);
check(getComputedStyle(sec).scrollbarGutter.startsWith('stable'),'the rail\'s parts keep no scroll bar\'s room (finding 54)');
check(getComputedStyle(list).scrollbarGutter.startsWith('stable'),'Files\' list keeps no scroll bar\'s room (finding 56)');
const crumbs=document.createElement('div');crumbs.className='crumbs';host.prepend(crumbs);
check(getComputedStyle(crumbs).scrollbarWidth==='none','the path draws a sideways scroll bar (finding 56)');
const head=document.createElement('div');head.className='frow fhead';list.append(head);
const sort=(marked)=>{const b=document.createElement('button');b.className='sortb r';b.textContent='Size';if(marked){const m=document.createElement('span');m.className='smark';m.textContent=' ▲';b.append(m);}head.append(b);
 const r=document.createRange();r.selectNodeContents(b.firstChild);const right=r.getBoundingClientRect().right-box(b).right;b.remove();return right;};
check(Math.abs(sort(true)-sort(false))<0.5,'Size moves '+Math.round(sort(false)-sort(true))+' pixels left when sorted (finding 57)');
const name=document.createElement('span');name.className='fname';name.innerHTML='<input class="fld bad" value="kitchen">';list.append(name);
check(getComputedStyle(name.firstChild).fontWeight!=='600','a refused name turns bold (finding 58)');
"#);
}

#[test]
fn ui59_ui60_ui63_ui64_offers_forms_menus_and_notes_are_one_look() {
    draw("UI-59", (1280, 600), r#"
files.arrange(host,list,null,null);
files.theListingIsDrawn({headers,listing:{is:'the-folder-was-not-read',said:words('Could not read this folder.'),refusal:{the_offer:'Retry with admin access',retry:'r'}}});
const offer=list.querySelector('.fwb button');
check(offer&&getComputedStyle(offer).borderTopWidth==='0px','the whole list\'s offer is a bordered button where a row\'s is a link (finding 59)');
const form=document.createElement('div');form.className='ky-form';form.style.width='480px';document.body.prepend(form);
form.innerHTML='<label class="ky-fl">Key type</label><span class="ky-fi"><span class="rads"><span class="rad on"><i></i>ed25519</span><span class="rad"><i></i>ecdsa</span></span></span>';
const label=form.querySelector('.ky-fl'), first=form.querySelector('.rad');
const baseline=(e)=>{const r=document.createRange();r.selectNodeContents(e);const rs=[...r.getClientRects()].filter(x=>x.width>0);return rs.length?rs[0].bottom:0;};
check(Math.abs(baseline(label)-baseline(first))<2,'the label sits '+Math.round(baseline(label)-baseline(first))+' pixels below its first choice (finding 60)');
const bar=document.createElement('div');bar.className='mb';bar.innerHTML='<div class="ctx"><button type="button" class="hd off" disabled>File</button></div>';document.body.append(bar);
const menu=document.createElement('div');menu.className='ctx';document.body.append(menu);
check(Math.abs(box(bar.firstChild).width-box(menu).width)<0.5,'the window\'s menu is '+box(bar.firstChild).width+' wide, a right-click menu '+box(menu).width+' (finding 63)');
check(getComputedStyle(bar.querySelector('.hd')).cursor!=='not-allowed','the menu\'s headings show the not-allowed pointer (finding 63)');
const note=document.createElement('p');note.className='hunread';note.textContent='2 rows would not read back.';document.body.append(note);
check(getComputedStyle(note).marginLeft==='22px'&&getComputedStyle(note).fontSize==='11.5px','the note is '+getComputedStyle(note).marginLeft+' in at '+getComputedStyle(note).fontSize+' (finding 64)');
"#);
}

#[test]
fn ui_the_small_ones_hold_their_places() {
    draw("UI-small", (1180, 500), r#"
host.style.display='none';
const col=document.createElement('div');col.style.width='420px';document.body.prepend(col);
const row=document.createElement('button');row.className='qrow';row.innerHTML='<span class="qgrip"></span><span class="qn">cloth</span><span class="qst run">Rendering</span><span class="qbars"><span class="qbar"><i></i></span></span>';col.append(row);
const base=(e)=>{const r=document.createRange();r.selectNodeContents(e);return [...r.getClientRects()].filter(x=>x.width>0)[0].bottom;};
check(Math.abs(base(row.querySelector('.qn'))-base(row.querySelector('.qst')))<1.5,'the state sits off its name\'s line');
check(Math.abs(box(row.querySelector('.qbars')).left-box(row.querySelector('.qn')).left)<0.5,'the bars start '+Math.round(box(row.querySelector('.qn')).left-box(row.querySelector('.qbars')).left)+' pixels left of the name');
const folded=(at)=>{const h=document.createElement('span');h.className='qm2 shut';h.innerHTML='<span class="qmn">cloth</span><span class="bar whole"><i></i></span><span><span class="mono">'+at+'</span></span><span>· 2 waiting</span>';col.append(h);const w=box(h.lastElementChild).left-box(h).left;h.remove();return w;};
check(Math.abs(folded('9%')-folded('100%'))<0.5,'what follows the folded percentage moves as it climbs');
const sheet=document.createElement('div');sheet.className='sheet';sheet.style.cssText='position:static;width:600px;height:auto';document.body.prepend(sheet);
const band=document.createElement('div');band.className='agnband';band.innerHTML='<p><b>Submitting again.</b> These are the settings cloth used.</p>';sheet.append(band);
check(Math.abs(box(band).left-box(sheet).left)<1&&getComputedStyle(band).borderTopRightRadius==='0px','the Submitting again band is inset where File not read runs the panel\'s width');
"#);
}

#[test]
fn ui37_the_resume_note_sits_on_the_buttons_middle() {
    draw("UI-37", (1180, 400), r#"
host.style.display='none';
const grp=document.createElement('div');grp.className='qdmain';grp.style.width='600px';document.body.prepend(grp);
grp.innerHTML='<div class="grp"><h3>Render it again</h3><div class="again"><div class="pair"><button type="button" class="btn">Submit again 240 frames</button><button type="button" class="btn quiet">Resume 92 frames</button><p class="bar-note">Frame 300 is past the end.</p></div></div></div>';
const pair=grp.querySelector('.pair'), note=pair.querySelector('.bar-note'), button=pair.querySelector('.quiet');
const middle=(e)=>(box(e).top+box(e).bottom)/2;
check(Math.abs(middle(note)-middle(button))<1,'the note sits '+(middle(note)-middle(button)).toFixed(1)+' pixels below the buttons\' middle');
"#);
}

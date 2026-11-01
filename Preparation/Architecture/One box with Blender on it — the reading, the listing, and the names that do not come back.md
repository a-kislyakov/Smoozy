# One box with Blender on it — the reading, the listing, and the names that do not come back

*An input document. What one rented machine did on one day, and nothing more.*

**Read every finding here as *this is what that box did on that day*, never as *this is what a rented machine does*.** That reading rule binds every document that cites this one.

This is the batch run for the fifth spine document, `Job files and outputs`, whose rows wait on more machine facts than any family in the stage. It answers **Facts 5, 7, 29 (its open halves), 71, 77, 93 and 94**, and leaves **Fact 92** short.

Three testers worked the same box at once, from the architecture runner's briefs, on 2026-08-17. Each was told what a negative result would have to look like before it started, and none was told what the plan hoped to see.

---

## The machine

Vast.ai instance **47968634**, on physical machine **142047**, from offer 47000646, in **Spain**.

GTX 1080 Ti, driver `580.173.02`. 4 effective CPU cores, 15 GB memory. 60 GB allocation on a 208 GB host disk. Filesystem **overlayfs**, 4096-byte blocks.

**A container, not a virtual machine**, from a bare `ubuntu:22.04` image. Ubuntu 22.04.5 LTS, kernel `Linux 65de1b89f9ec 5.15.0-187-generic #197-Ubuntu SMP Fri Jul 17 19:17:01 UTC 2026 x86_64`. `OpenSSH_8.9p1 Ubuntu-3ubuntu0.16, OpenSSL 3.0.2 15 Mar 2022`.

Reached at **`ssh2.vast.ai` port 18634** through the provider's proxy. Round trip on that route **129.6 ms**; on the direct address, 52.1 ms — and see §3.7 for what the direct address turns out to be.

**The caller end throughout is the Creator's own Mac**, `OpenSSH_10.2p1`, APFS at default settings.

Two logins: `root`, and a non-root login **`smoozy`** (`uid=1000 gid=1000`). Three Blenders, none on any path: **4.5.9 LTS** (built 2026-04-21), **5.0.1** (2025-12-16), **5.2.0 LTS** (2026-07-14). **5.1 was not installed and nothing here was posed in it.**

About **$0.0433 per hour** all in. Rented and destroyed the same day; the account was verified afterwards with no instances and no volumes.

**One limit binds every timing in this document: three testers shared four cores**, at load average 2.5 to 6.5. Where that defeats a question it is said so.

---

# 1. Fact 5 — driving a Blender to read a `.blend` — ANSWERED

**Posed as Smoozy would have to.** One SSH command from the Mac, a named Blender already on the machine, headless, no window and no display server. The reading comes back as **one JSON document on standard output between `SMOOZY_READ_START` and `SMOOZY_READ_END` sentinels** — a form a program acts on rather than a person squints at. Blender's own chatter shares that stream, which is why the sentinels exist.

**All three payloads come out of one read.** The reader's error map came back `{}` in every final run in every version.

## 1.1 The fixture, and four faults found in it before any result was taken

Built **natively by each version** from one branching script, so nothing here is a version-conversion artefact.

Scene `Main` at frames **10–50 step 2** — a range that neither starts at 1 nor lands on its end. Three view layers: `VL_Always`; `VL_Never`, disabled; and `VL_Sometimes`, **keyframed** to 0 at frame 1, 1 at frame 10 and 0 at frame 21 with constant interpolation, carrying its own sample count of 32 against the scene's 128. Cycles at 128 samples, adaptive sampling on at 0.02, device GPU. 1280×720 at 50%. A live final output node. **Three file output nodes** — one at an absolute folder with two of three outputs connected, one at a **relative** folder with two connected, one **muted**. A second scene with its own everything, and **a sequencer strip reaching it**. Recorded locations in four forms: absolute Linux, relative, **Windows** `C:\Users\artist\textures\tex_win.png`, and **macOS** `/Users/artist/Textures/tex_mac.png`. Plus a text path and a linked library.

**Four faults were found and fixed before any result was taken, and three would have produced a confident wrong answer rather than an error.** The images were **generated** data-blocks with a path attached rather than file images, so the flat path list rightly ignored them — which read as *the flat list misses images*. **Saving remaps paths relative by default**, silently rewriting the absolute and foreign paths on save. **From 5.0, adding an output to a file output node rebuilds the node's socket list**, so a socket reference held across a later add is stale and the new link lands on the trailing extend socket. And setting the image format did not take in 5.x while the media type was multi-layer.

**The fixture was then verified by rendering it rather than trusted.**

## 1.2 Payload one — the file's recorded locations

Three means reported separately, so the answer says *which means reached what*. **Identical in all three versions:**

```
name                        value                                    flat  map  markwalk
images:tex_abs              /root/sliceA/fix/tex_abs.png             True  True  True
images:tex_rel              //assets/tex_rel.png                     True  True  True
images:tex_win              C:/Users/artist/textures/tex_win.png     True  True  True
images:tex_mac              /Users/artist/Textures/tex_mac.png       True  True  True
libraries:lib_XX.blend      /root/sliceA/fix/lib_XX.blend            True  True  True
texts:note                  /root/sliceA/fix/notes.txt               True  True  False
regular_output_path:Main    /home/artist/proj/render/frame_####      False False True
regular_output_path:Second  /tmp/second_out/s_####                   False False True
fon_folder:Main/FON_A       /home/artist/proj/render/beauty/         False False True
fon_folder:Main/FON_B       //passes/mist/                           False False True
fon_folder:Main/FON_C       //muted/                                 False False True
fon_folder:Second/FON_S     /tmp/second_nodes/                       False False True
```

The two bulk means **miss every write location**. A walk over Blender's own path marks reaches all twelve **including every write location** and misses the text path. All four path forms read out. The Windows path went in with backslashes and reads back with forward slashes.

**A negative would have looked like** one means reaching everything, or no means reaching the write locations — which would have collapsed the redirect's readability. The `False` entries are the test failing where it should.

## 1.3 Payload two — every value the Render Settings spec shows

The list was taken from that spec rather than invented. From the 5.2 fixture; the others agreed except where noted.

```
frame_start 10  frame_end 50  frame_step 2  frame_count 21
resolution_x 1280  resolution_y 720  resolution_percentage 50  pixels [640, 360]
engine_as_read CYCLES
samples {'cycles': 128, 'eevee': 64}
cycles_device GPU
early_stop {'use_adaptive_sampling': True, 'adaptive_threshold': 0.019999999552965164,
            'adaptive_min_samples': 0}
time_limit 0.0    use_layer_samples USE
saved_with_blender [5,2,44]   (4.5.9: [4,5,91];  5.0.1: [5,0,119])
```

```
{'name':'VL_Sometimes','use_now':False,'animated':True,
 'keyframes':[[[1.0,0.0],[10.0,1.0],[21.0,0.0]]],
 'samples_override':32,'overrides_samples':True}
```

**The per-frame question is answered by one read** — which that spec calls a bigger claim than knowing which view layers a file holds. Inside the single read: `'10': ['VL_Always','VL_Sometimes'] … '20': [both] … '22': ['VL_Always'] … '50': ['VL_Always']`. The keyframes also read out as data, so it works either by stepping frames or by reading the curve.

A scene pulling in another read out as `{"via_render_layers_node": [], "via_sequencer_strip": [{"strip":"to_second","scene":"Second","use_sequencer":true}]}`.

**A version difference inside the reading:** the second scene's engine reads back `BLENDER_EEVEE_NEXT` in 4.5.9 and `BLENDER_EEVEE` in 5.0.1 and 5.2.0.

**Not posed, and a real gap:** a file naming an engine the chosen Blender lacks. Cycles is enabled on all three builds here, including under factory settings, and no Blender lacking it could be produced without installing or removing software on a shared box.

## 1.4 Payload three — destinations, tree, gating conditions and per-frame set

```
{"use_compositing_switch": true, "tree_present": true,
 "final_output_node_kind": "NodeGroupOutput",
 "final_output_nodes": [{"node":"Group Output","muted":false}],
 "has_live_final_output_node": true, "use_sequencer": true,
 "sequencer_strip_count": 1, "sequencer_takes_over": true,
 "compositor_runs": false}
```

4.5.9 differs exactly as the research records: the final node is the Composite node, and 4.5 carries a fourth gating switch the others have not.

**The reading was checked against a real render.** It predicted five files per frame and 105 over 21 frames. The render produced 42 in the first node's folder, 42 in the second's and 21 at the regular output — **105 exactly, with no folder at all for the muted node.** The unconnected output wrote nothing.

**The keyframed view layer was confirmed by rendering, inside one run**, so the shared box cannot explain it:

```
LOAD_FRAME 10 5.932  12 6.021  14 5.512  16 5.474  18 5.486  20 5.490
LOAD_FRAME 22 2.748  24 2.746  26 2.750  28 2.761  30 2.988  ...  50 2.760
```

Frames 10 to 20 take about twice as long and the break falls exactly where the keyframe says.

**Two naming differences seen on disk**, stated narrowly — one fixture, one scene reached through a strip, and the general rule was not established. 4.5.9 wrote `beauty0010.png`; **5.2.0 wrote `fixture_52beauty0010.png`**, the node's own base file name joined to the front. In the sequencer run 4.5.9's second-scene node wrote `sec0001.png` *with* a frame number while 5.2.0 wrote `fixture_52sec.png` with **none**.

## 1.5 What the read costs

Three runs per version, at load average 4.14, with no Blender of the tester's own running.

```
4.5.9  caller 3.58/3.48/3.23 s   in-script 0.085/0.088/0.086 s   stdout 15437 bytes
5.0.1  caller 3.62/3.49/3.37 s   in-script 0.116/0.119/0.120 s   stdout 25581 bytes
5.2.0  caller 3.65/3.74/3.82 s   in-script 0.112/0.113/0.113 s   stdout 25765 bytes
```

**About 3.2 to 3.8 seconds end to end including a fresh connection; the reading itself 0.09 to 0.12 seconds; 15 to 26 KB back.** Nowhere near a cost that makes it unusable before every submission.

## 1.6 Whether a read disturbs a running render — NOT ANSWERED

Five reads were taken while a 21-frame render ran. **All five succeeded with an empty error map, and the render finished all 21 frames and all 105 files.** In-script time rose from about 0.11 s to 0.24–0.50 s. But the read-during-render run totalled 155.0 s against a control's 79.8 s, while the five reads spanned only the first 40 s and the later frames were slower throughout — which points at **the other testers' load**, at average 5.10 against 4.18, rather than at the reads. On a four-core container shared with two other testers the two cannot be separated.

**What stands: a read against a running render completed correctly every time on this container. The magnitude is untested.**

---

# 2. Fact 77 — driving a Blender to rewrite a `.blend`'s recorded locations — ANSWERED

**The routes were looked up before any test code was written**, from *Reading a file 9*, which the map names for *How is each outside location written inside the file, and can I rewrite it after the upload?*, alongside *Reading a file 5* and *Dependencies 17*, with *What has to travel with a file* read first as the joined answer.

**Confirmed by probe before any rewrite:**

```
4.5.9   bd_path_calls: ['file_path_map', 'filepath']
5.0.1   bd_path_calls: ['file_path_foreach', 'file_path_map', 'filepath']
5.2.0   bd_path_calls: ['file_path_foreach', 'file_path_map', 'filepath']
```

The one-pass route is **absent in 4.5.9**, exactly as the research records.

Each run digested the artist's file, copied it, rewrote **the copy**, saved, digested both again, then **opened the copy in a completely separate process** and read every location back off disk.

## 2.1 The per-setting route works in all three versions and reaches everything

4.5.9, twelve locations changed, read back from disk by a fresh process:

```
images     {"tex_abs":"/root/sliceA/job/assets/tex_abs.png", "tex_mac":".../tex_mac.png",
            "tex_rel":".../tex_rel.png", "tex_win":".../tex_win.png"}
libraries  {"lib_45.blend":"/root/sliceA/job/assets/lib_45.blend"}
texts      {"note":"/root/sliceA/job/assets/notes.txt"}
regular_output {"Main":"/root/sliceA/job/out/Main/f_####", "Second":".../Second/f_####"}
fon_folders    {"Main/FON_A":"/root/sliceA/job/out/Main/FON_A/", ... "Second/FON_S":".../FON_S/"}
```

**The property name for a file output node's folder differs and the rewrite must know which:** `base_path` in 4.5.9, `directory` in 5.0.1 and 5.2.0.

## 2.2 The one-pass route is absent in 4.5, and silently partial in 5.x

**In 4.5.9 it does not exist**, so nothing was rewritten and the file on disk still held every original path afterwards even though the script saved. **For 4.5 the answer is owed without it, and the per-setting route is the only one.**

**In 5.0.1 and 5.2.0 it visits six paths — the four images, the text and the library — all rewritten and all reaching disk — and silently leaves every write location alone:**

```
regular_output {"Main":"/home/artist/proj/render/frame_####", "Second":"/tmp/second_out/s_####"}
fon_folders    {"Main/FON_A":"/home/artist/proj/render/beauty/", "Main/FON_B":"//passes/mist/",
                "Main/FON_C":"//muted/", "Second/FON_S":"/tmp/second_nodes/"}
```

**This is one of the negatives the question named — a route that reaches some recorded locations and silently leaves others — and it happened.** Nothing was raised, nothing printed, and the save succeeded. **A redirect built on this route alone would appear to work and leave every place the job writes pointing at the artist's own machine.**

A version difference inside the route: the callback metadata carries a read-only flag in 5.2.0 and **no such field in 5.0.1**. 5.2's three extra options were not exercised.

## 2.3 The save is what makes a rewrite reach disk, and the artist's file is untouched

Posed deliberately:

```
IN THE RUNNING BLENDER:  regular_out {"Main":"/root/sliceA/job/out/Main/f_####", ...}
ON DISK, fresh process:  regular_out {"Main":"/home/artist/proj/render/frame_####", ...}
                         images {"tex_abs":"/root/sliceA/fix/tex_abs.png", ...}
```

The copy's digest was **unchanged**. That is the first negative shape the question named, reproduced on purpose.

The artist's own file was digested before and after all seven runs:

```
fixture_45.blend  c0dfd83c93afdd5edc7decce74d08bd41ec9c8afa1dc13abf05950d23e4cd61e (before and after)
fixture_50.blend  38e2aae4e7a6b7e4...    fixture_52.blend  6bcb3d05bfbb279b...
```

In every run the copy's digest changed and the original's did not — **including the two runs where nothing was rewritten, which is the control showing the check would have noticed.**

**And the rewrite takes effect in a real render.** The rewritten 5.2.0 copy, with no path touched at launch: ten files, all under the new folder, and `ls: cannot access '/home/artist': No such file or directory`. The unrewritten fixture builds that foreign tree silently.

## 2.4 Two traps to carry into Build

**Saving remaps paths relative by default.** Whatever writes the copy Smoozy renders from must turn that off, or it silently rewrites locations nobody asked to change.

**From 5.0, adding an output to a file output node rebuilds its socket list**, so a held socket reference goes stale and links land on the trailing extend socket with no error.

---

# 3. Fact 93 — what the machine's own file-transfer server offers — ANSWERED, one half unposable

## 3.1 The announced version and the complete extension list

Read raw off the wire by a protocol speaker written for the purpose rather than by trusting a client. As `root`, over the proxy:

```
VERSION packet type=2 version=3
  extension name="posix-rename@openssh.com" data=b'1'
  extension name="statvfs@openssh.com" data=b'2'
  extension name="fstatvfs@openssh.com" data=b'2'
  extension name="hardlink@openssh.com" data=b'1'
  extension name="fsync@openssh.com" data=b'1'
  extension name="lsetstat@openssh.com" data=b'1'
  extension name="limits@openssh.com" data=b'1'
  extension name="expand-path@openssh.com" data=b'1'
```

**Byte-identical under the non-root login**, and a verbose client agrees line for line.

**The three extensions the question named are present** — POSIX-rename, fsync and statvfs. **The set is nevertheless smaller than the caller's.** The Mac's own server, enumerated in the same sitting, announced **eleven**: the same eight plus `copy-data`, `home-directory` and `users-groups-by-id@openssh.com`.

**`copy-data` is the one that matters** — a server-side copy that does not round-trip bytes through the caller. **Anything designed by watching the caller's end finds it there and finds it absent on the rental.**

## 3.2 The announced operation limits

The stock client sends and receives this and never prints it. Decoded:

```
max-packet-length = 262144
max-read-length   = 261120
max-write-length  = 261120
max-open-handles  = 1019
raw payload after request-id = 0000000000040000000000000003fc00000000000003fc0000000000000003fb
```

Identical under the non-root login. Recorded as the machine's own words and compared to nothing, as the fact requires.

## 3.3 The session cap — established rather than quoted

Not set in the configuration; present only as the commented default `#MaxSessions 10` at `/etc/ssh/sshd_config:36`. Effective value from the server's own dump: `maxsessions 10`. Then posed rather than trusted — fourteen concurrent sessions on **one** multiplexed connection:

```
sessions that succeeded: 10 of 14
sessions that failed:    4 of 14
session 11 rc=255 stderr='mux_client_request_session: session request failed: Session open refused by peer\nHost key verification failed.'
```

The first ten succeeding in the same run is the control. **The cap is per connection** — a second connection gets its own ten.

**Nobody asked this and a builder needs it: the refusal never says *too many sessions*.** It says `Session open refused by peer` and then appends the actively misleading `Host key verification failed.` Anything matching on that text chases a host-key bug that is not there. **This is Fact 37's shape again.**

## 3.4 Which free-space route this machine offers — both

The outcome to watch for, a machine offering neither, did not occur.

The extension route, as the non-root login: `statvfs@openssh.com "/home/smoozy"` returned `f_bsize 4096, f_blocks 15728640, f_bavail 14784576, f_namemax 255`.
The command-channel route, same login: `df -P` returned `overlay 62914560 3820600 59093960 7% /`.
The two agree — 15728640 × 4096 = 60 GiB.

**Control in the same run**, proving the login is genuinely unprivileged rather than root ignoring permissions:

```
uid=1000(smoozy) gid=1000(smoozy) groups=1000(smoozy)
ls: cannot open directory '/root': Permission denied
```

**Unasked and load-bearing: both routes report the whole container overlay, not a per-path allowance.** `df -P /root` run **as the non-root login**, against a directory it cannot even list, returned the same figures at `rc=0`. **Free space on this machine is a property of the mount and not a permission-checked reading of a path.**

## 3.5 The manual pages document nothing, because there are none

```
-- man sftp-server --
This system has been minimized by removing packages and content that are
not required on a system that users do not log into.
```

The same for the server configuration and the client. **No OpenSSH manual files exist on disk.** The documentation directories hold only a changelog, a copyright and an empty examples folder. So the request-ban flag, the read-only flag and the session cap are **all undocumented on this machine**. What exists instead: the binary's usage string names the three flags without saying what they do; the session cap appears in exactly two files, both as a comment; and the server can be asked to enumerate the twenty-six request names a ban list would be written against.

## 3.6 The silence, and it is the third time

Nothing on this machine documents what the POSIX-rename extension performs on the server. A search for the protocol description files returned **nothing** — they are not shipped. A search of the documentation and configuration trees for the extension's name returned **nothing**. The string exists in one reachable place: a bare name in the server's own request enumeration. **The machine names the mechanism and describes it nowhere.**

## 3.7 COULD NOT POSE — proxy against direct, and the reason is itself the finding

The direct address is open and gives an identical banner, `SSH-2.0-OpenSSH_8.9p1 Ubuntu-3ubuntu0.16`, and **rejects the provider key that works over the proxy** — `Permission denied (publickey,password)` for both logins, though TCP connected in 52.1 ms.

**The host keys settle what it is.** The proxy route's three keys match the container's own three exactly — `SHA256:qfPbWOIM…`, `y8HtcyXEX…`, `keB7R063…`. The direct port's three — `VZyyzAG9…`, `Lw30s6fe…`, `8CG8sG5u…` — match nothing on the container. Corroborated from inside the container: `SSH_CLIENT=::1 58112 22`. **Every connection reaching this container arrives on loopback**, because the proxy terminates outside and forwards in.

**Three things follow and a builder needs all three.**

1. **A recorded "direct" endpoint for a rental at this provider is not verified to reach that rental.** Connecting can succeed at the TCP and banner level while being a different machine, and only the host key distinguishes them. Anything trusting the direct endpoint without pinning the key seen over the proxy can authenticate against, or leak a file name to, a stranger's server.
2. **The container cannot see the caller's real address**, so nothing on it can key on one.
3. **The proxy-against-direct comparison is unestablished** — not shown absent and not shown present.

---

# 4. Fact 94 — Smoozy's own byte loop over a real link — ANSWERED

The loop was written rather than borrowed: offset writes, a deliberate stop, a resume from a recorded offset, and a digest. The same code ran over a local pipe and over the link **in this sitting**, so the comparison is like for like rather than against a memory. 400 MB (`419430400` bytes), stop at 137 MB — 32.7% through — 32 KiB chunks, a 64-deep window.

| channel | stop offset | resume sent | started over? | digest |
|---|---|---|---|---|
| local pipe (control) | 137003008 | 282427392 | no | MATCH |
| Mac → rental | 137003008 | 282427392 | no | MATCH |
| rental → Mac | 137003008 | 282427392 | no | MATCH |

All three ended at `fdc51e9e9fc75122e19442ee1ce4afeeb767b90498c3aab70949d3abbc013fe1`, the source digest.

**A negative would have looked like** a resume that started over, which would have shown `sent = 419430400`; it showed `282427392` on every channel. An offset write landing wrong would have shown a wrong size or a digest mismatch. A digest matching over a pipe and not over a link would have shown as pipe MATCH against network MISMATCH. **None occurred. The link behaves as the pipe does.**

**Offset-write placement, checked bare.** A single 20-byte write at offset 1000000 into a fresh file produced size `1000020` on both channels, with the marker readable at exactly 1000000. On the rental the hole is genuinely sparse — apparent size 977K against 4.0K allocated.

**What a stop leaves on disk, established deliberately.** At the rental end, immediately after the loop's own stop:

```
-rw-r--r-- 1 root root 137003008 Aug 17 20:01 /root/sliceC/x/up-dest.bin
  size=137003008 name=/root/sliceC/x/up-dest.bin mode=644
```

Mac end: `name=/tmp/scC/down-dest.bin size=137003008 mode=644`.

**The bytes sit at the final name, with ordinary permissions, and nothing on either machine marks them as partial** — no suffix, no flag, no attribute. This is the failure the project already recorded for an ordinary tool, now recorded for **Smoozy's own loop**. The only thing that knows the file is incomplete is the recorded offset, which lives outside the filesystem; if that record is lost, a 137 MB fragment is indistinguishable from a finished file of that size.

**Cost.** Upload 0.7 MB/s (203.5 s, then 380.5 s, for 400 MB). Download 2.3–2.8 MB/s (49.4 s, then 121.9 s). Local pipe about 500 MB/s. **The link is strongly asymmetric — roughly three to four times faster down than up.** A digest over 400 MB on the rental cost 4.7 s.

---

# 5. Fact 7 — listing a folder over the connection — the fields half ANSWERED, the failure half ANSWERED IN PART

Posed under the non-root login throughout. Every finding is on the proxy route.

## 5.1 The four fields come back, on both routes

```
drwxrwxr-x 3 smoozy smoozy         4096 2026-08-17 19:49:26.773876312 +0000 .
prw-rw-r-- 1 smoozy smoozy            0 2026-08-17 19:49:26.769876281 +0000 fifo_one
-rw-rw-r-- 1 smoozy smoozy            1 2038-01-01 00:00:00.000000000 +0000 future.txt
-rw-rw-r-- 1 smoozy smoozy       123456 2026-08-17 19:49:26.673875514 +0000 known_123456.bin
lrwxrwxrwx 1 smoozy smoozy           14 2026-08-17 19:49:26.769876281 +0000 link_dangling -> /no/such/place
-rw-rw-r-- 1 smoozy smoozy            1 2001-02-03 04:05:06.000000000 +0000 old.txt
-rw-rw-r-- 1 smoozy smoozy 536870912000 2026-08-17 19:49:26.769876281 +0000 sparse_500G.dat
drwxrwxr-x 2 smoozy smoozy           23 2026-08-17 19:49:26.677875546 +0000 subdir
-rw-rw-r-- 1 smoozy smoozy            0 2026-08-17 19:49:26.673875514 +0000 zero.exr
```

Kind, apparent size, blocks allocated, epoch modification time and name all come back in one reading:

```
p	0	0	1786996166.7698762810	./fifo_one
f	123456	248	1786996166.6738755140	./known_123456.bin
l	14	0	1786996166.7698762810	./link_dangling
f	1	8	981173106.0000000000	./old.txt
f	536870912000	0	1786996166.7698762810	./sparse_500G.dat
d	23	0	1786996166.6778755460	./subdir
f	0	0	1786996166.6738755140	./zero.exr
```

## 5.2 Which clock, established four ways

**No timezone is configured at all.**

```
TZ env: [unset]
cat: /etc/timezone: No such file or directory
ls: cannot access '/etc/localtime': No such file or directory
```

The `+0000` is the C library's fallback in the **absence** of configuration, not a configured zone. **A provider image that set one would render those times in that zone.** The epoch form carries no timezone and is the unambiguous one.

**Nothing inside the container disciplines the clock.**

```
timedatectl
System has not been booted with systemd as init system (PID 1). Can't operate.
Failed to connect to bus: Host is down
-- ntp/chrony processes: (none)
```

**It agrees with the caller's clock, and only to within half a second.** Five measurements over an open connection:

```
round_trip=593.6ms  machine_clock_minus_Mac_clock=+226.8ms (+/- 296.8ms)
round_trip=725.9ms  machine_clock_minus_Mac_clock=+294.4ms (+/- 362.9ms)
round_trip=684.8ms  machine_clock_minus_Mac_clock=+187.3ms (+/- 342.4ms)
round_trip=548.8ms  machine_clock_minus_Mac_clock=+204.4ms (+/- 274.4ms)
round_trip=625.0ms  machine_clock_minus_Mac_clock=+241.3ms (+/- 312.5ms)
```

The apparent +0.2 s sits inside the ±0.3 s uncertainty. **Established: the clocks agree to better than half a second — not more closely.**

**A hand-set time round-trips exactly**, at nanosecond resolution: a file set to `2001-02-03 04:05:06 UTC` read back `981173106`.

## 5.3 The file whose contents are a lie

A 500 GB sparse file on a 60 GB allocation. The listing reports `536870912000` and **0 blocks allocated**:

```
du -sh --apparent-size sparse_500G.dat  ->  500G
du -sh                 sparse_500G.dat  ->  0
df -h /home/smoozy  ->  overlay 60G 3.7G used 57G avail 7%
```

**Detectable on the command-channel route**, as one extra field in the same reading. **Undetectable on the file-transfer route**, which carries no allocated-block count at all.

## 5.4 Does the size cost a second round trip? No — settled at protocol level

Run with the server's own logging enabled for that session and touching no shared configuration. One long listing of a 15-entry folder, the **entire** exchange:

```
debug1: request 1: limits
realpath "."
debug1: request 2: sent names count 1
lstat name "/home/smoozy/sliceB/f7"
debug1: request 3: sent attrib have 0xf
opendir "/home/smoozy/sliceB/f7/"
debug1: request 4: sent handle handle 0
debug1: request 5: readdir "/home/smoozy/sliceB/f7/" (handle 0)
debug1: request 5: sent names count 17
debug1: request 6: readdir "/home/smoozy/sliceB/f7/" (handle 0)
sent status End of file
sent status Success
```

**One reply carried all 17 entries with attributes. Zero per-file stats** — the single `lstat` is the folder itself. The attribute flag `0xf` is size, owner, permissions and times.

On **5010 entries**:

```
readdir requests     : 52
per-file 'lstat name': 1
per-file 'stat name' : 0
entries per reply    : [(100, 50), (1, 1), (12, 1)]
total delivered      : 5013
```

**52 round trips, not 5010.** One per file would have cost 5010 × 129.6 ms = **651 seconds**; the measured figure was 18.22 s.

Cost on 5010 entries, best of three, handshake excluded:

| reading | best | bytes |
|---|---|---|
| trivial command (floor) | 0.47 s | 0 |
| kind, size, time and name in one walk | **0.72 s** | 224,537 |
| long listing | 0.69 s | 435,969 |
| a stat per file, batched | 0.65 s | 345,059 |
| file-transfer route, one listing | 18.22 s | 360,707 |

**About a quarter-second of real work for 5010 entries with all four fields**, or 2.98 s with a fresh connection.

**So the zero-byte rule is buildable.** The zero-length file reports `0` on both routes at no extra cost, which is the reading the Creator's ruling of 2026-08-12 rests on.

## 5.5 The failure half, and the trap demonstrated first

```
drwx------ 2 root root 30 /home/smoozy/sliceB/denied
--- root CAN list the 700 folder ---
-rw-r--r-- 1 root root 7 Aug 17 19:56 hidden_entry.txt
root exit status listing denied = 0
--- and the gone path is genuinely absent, even to root ---
root exit status listing gone_folder = 2
```

**Root lists the root-owned private folder at exit 0.** Posed as root this reads as success and proves nothing. Everything below is the non-root login.

**On the command channel:**

```
ls -la .../denied        EXIT 2   ls: cannot open directory '...': Permission denied
stat -c .../denied       EXIT 0   /home/smoozy/sliceB/denied drwx------ root
test -d .../denied       EXIT 0   IS_A_DIRECTORY

ls -la .../gone_folder   EXIT 2   ls: cannot access '...': No such file or directory
stat -c .../gone_folder  EXIT 1   stat: cannot statx '...': No such file or directory
test -d .../gone_folder  EXIT 0   NOT_SEEN_AS_DIRECTORY
```

```
exit(denied)=2   exit(gone)=2   -> SAME: exit status alone CANNOT tell them apart
```

Only the English message on the error stream separates them, and that is stable **here** only because the image carries `C`, `C.utf8` and `POSIX` and the language variable is unset. **The stat of this folder separates them by exit status — 0 against 1 — and the directory test does not: `test -d` exits 0 for both.** **Corrected 2026-08-22: this said *adding a stat or a directory test does separate them*, which the run above already contradicts on the directory test.** The stat half holds for the folder itself and not in general: a macOS run of 2026-08-21 put a stat and a file test on a path *under* an unreadable parent and both exited 1 in either case, separated only by a walk of the parent components — a case not posed on this box.

**On the file-transfer route the protocol statuses are distinct**, with controls either side:

```
CONTROL: sent status Success / -rw-r--r-- ... i_am_readable.txt / EXIT 0
CASE 1:  opendir "...denied/"  → sent status Permission denied  / EXIT 1
CASE 2:  lstat ".../gone_folder" → sent status No such file     / EXIT 1
CONTROL AGAIN: sent status Success / EXIT 0
```

The client's exit is 1 for both; **the protocol status distinguishes them**, and any library exposes it.

**The machine did not answer — all three shapes, all exit 255:**

```
3a nothing listening:  ssh: connect to host ... port 18699: Connection refused   EXIT 255
3b accepted, silent:   Connection timed out during banner exchange               EXIT 255
                       gave up after 8.0s (ConnectTimeout was 8)
3c cut mid-listing:    [4 lines of real output] then
                       Connection to ssh2.vast.ai closed by remote host.         EXIT 255
3d sftp server killed: Connection closed / entries received before the cut: 0    EXIT 255
```

The silent case used a local silent socket standing in for an unresponsive far end; the other three are the real machine. **Exit 255 with a client-written message, against exit 2 with a remote message, against a distinct protocol status — separable.**

## 5.6 The finding a builder most needs

Two listings in the same run, **both four lines long**:

```
genuinely short (3 entries), uncut:  4 lines   EXIT: 0
cut (starts big, dies early):        4 lines   EXIT: 255
```

**Partial output is delivered before the connection dies.** A truncated listing is indistinguishable from a complete short one **except by exit status**. Anything that treats *output arrived* as *listing complete* is wrong on this machine.

**A negative would have looked like** sizes absent or wrong; a round trip per file; times only in a local zone with no epoch form, or a clock hours adrift; the three failures indistinguishable on *every* route; or the reading unavailable to the non-root login. None occurred — **except** that the plain listing command shares exit 2 between refusal and absence, which is why the failure half is recorded as answered in part rather than answered.

---

# 6. Fact 71 — driving a Blender to make a small viewable copy — ANSWERED, with one named gap

Blender behaviour was looked up through the map to *Where a render's files go* and *Images 1, 5, 6* and *8*, which already establish that a working format records no colour treatment and that the scaling and saving calls work headless where the editor operators refuse. **Only the drive-it-over-the-connection and cost questions are established here.**

**Headless over the connection: yes.** One command channel, one Blender, a script and its arguments. No screen, nobody logged in.

**Three working formats were written by driving the render**, at 1920×1080 under Cycles:

```
23976951  probe_0001.exr   single-layer OpenEXR
48231807  gpu_0001.exr     multilayer  OpenEXR
 4617486  hdr_0001.hdr     Radiance HDR
```

## 6.1 What the copy costs

Whole invocation including Blender's own startup, at 640×360 PNG:

| Blender | wall | user | sys | load | scale | save |
|---|---|---|---|---|---|---|
| 4.5.9 | 1.598 s | 1.351 s | 0.116 s | 0.124 s | 0.049 s | 0.350 s |
| 5.0.1 | 1.701 s | 1.297 s | 0.135 s | 0.191 s | 0.040 s | 0.452 s |
| 5.2.0 | 2.191 s | 1.778 s | 0.194 s | 0.130 s | 0.042 s | 0.421 s |

Repeats settled to about `wall=0.81 s user=0.92 s`. Radiance HDR the same. **Blender's startup dominates; the actual work is about 0.2 s.**

**Cost while a render runs — the question the fact turns on.** Cycles on the processor with adaptive sampling off, all four cores saturated and verified throughout:

```
BUSY_COST wall=2.380s user=1.375s sys=0.093s
BUSY_COST wall=2.684s user=1.327s sys=0.164s
BUSY_COST wall=2.362s user=1.357s sys=0.113s
BUSY_COST wall=2.226s user=1.371s sys=0.118s
```

**About 0.8 s idle and about 2.4 s with all four cores taken**, with the output byte-identical to the idle case and the render still progressing. **Two earlier attempts were false and are discarded** — the first two background renders finished before measurement because adaptive sampling ended them early, and a process check was matching its own command line and reporting a render where the machine was idle. Only the third run is reported.

## 6.2 Colour — where it goes wrong, and it is the whole difficulty

The file records no treatment, so something must choose. The tester chose **the job's own view transform, read out of the `.blend`** — `AgX` in all three versions.

The same frame under different choices:

```
exr45_AgX.png       mean=0.6108 median=0.6284 min=0.1384 max=0.9020 frac_above_0.98=0.000
exr45_Standard.png  mean=0.6175 median=0.5200 min=0.1631 max=1.0000 frac_above_0.98=0.237
exr45_Filmic.png    mean=0.5937 median=0.5415 min=0.1712 max=0.9375 frac_above_0.98=0.000
```

**The plain choice blows out 23.7% of the picture; the other two blow none.** Nothing crushed to black in any of them.

Every preview was fetched and looked at. **All three read plainly as the render, with textures visibly present. A copy with no treatment chosen does not** — shading gone, surfaces flat clipped colour. **That is the failure mode, and it is obvious rather than subtle.**

## 6.3 Version differences, and the one gap

**Single-layer OpenEXR and Radiance HDR behave identically in all three versions** — same route, same statistics, PNG sizes within 200 bytes of each other.

**Multilayer OpenEXR differs in several places at once.** Loading it as an image gives `size=0x0 is_float=False depth=0` with no pixels **in all three**, and **the scaling call then silently succeeds and changes nothing**. The saving call still works but writes at **full size** — viewable, not small. The compositor reaches it, and the compositor's own interface is not the same across the boundary: 4.5 exposes the scene's node tree where 5.0 and 5.2 raise an attribute error and use a differently named one; the scaling node's inputs differ; and a freshly added file output node in 5.x is locked to multilayer, refusing PNG outright.

**4.5.9 produced the small copy** through the compositor in 0.176 s. **5.0.1 and 5.2.0 did not** — the attempt still wrote a full-size multilayer file. **That is a gap in this sitting rather than a demonstrated impossibility**, and it is the one thing this fact leaves open.

**A route that works for multilayer in all three** is the image library bundled inside Blender's own Python:

```
4.5  OIIO 3.0.6.1   READ 1920x1080 channels=8   WROTE 368724 bytes in 0.531s
5.0  OIIO 3.0.9.1   368720 bytes in 0.651s
5.2  OIIO 3.1.13.1  368752 bytes in 0.561s
```

The 4.5 result was looked at and matches Blender's own treated output.

## 6.4 What else the machine carries — established, not chosen

**Outside Blender, nothing.** No image tool of any kind is present, and Python carries no imaging library.

**Inside Blender's bundled Python, two:** the image library above, and a numerical library.

**The image library on its own does not know Blender's colour treatment.** Pointed at nothing it fails or guesses; pointed at Blender's own colour configuration it reproduces the job's treatment exactly. The three outcomes:

```
no transform   min=[0,0,0] max=[1.0,1.0,1.0]           avg=[0.5509,0.3943,0.596]
linear->sRGB   min=[0,0,0] max=[1.0,1.0,1.0]           avg=[0.663,0.5612,0.6921]
ocio AgX       min=[0,0,0] max=[0.9882,0.9059,0.9451]  avg=[0.6345,0.5685,0.6401]
```

**The no-transform copy is the negative result for the purpose**, and the obvious guess a general tool makes is plausible but is not what the job was made under. **Only the route through Blender's own colour configuration reproduces it.**

Installing an outside tool is not free — a dry run of the obvious one plans six further packages. **Nothing was installed.**

---

# 7. Fact 29, the open halves — ANSWERED, and it found its negative

**Longest single name: 255 characters.** 256 was refused at open with `SSH_FX_BAD_MESSAGE (5) 'Bad message'`, as was every length up to 270. It matches the `f_namemax = 255` the extension reports. **Nothing was left behind.**

**Longest whole path: 4095.** 4096 was refused at open with the same status. Making a directory refused at path length 4240. **Nothing was left behind.**

**Unasked and it will bite: the refusal for both over-limit cases is `Bad message`**, which reads as protocol corruption rather than *name too long*. Anything surfacing that string to a user reports the wrong problem entirely. **Both refuse at open, before any byte is sent**, so the failure is early and clean.

**The machine's own tools.** Quoted, everything held — listing, searching, archiving and measuring all handled a path with spaces correctly, and a 4039-character nested path was listed, found and removed at `rc=0`. Unquoted, the shell split it into four failures, which is the builder's bug rather than the machine's. **This box did not reproduce the earlier box's one-space-breaks-three-tools behaviour.**

**Two jobs given the same label.** On the file-transfer channel the second directory creation was refused with `SSH_FX_FAILURE (4) 'Failure'`, a **generic** code. On the command channel: `mkdir: cannot create directory ‘/root/sliceC/n/Job Label’: File exists`, `rc=1`. And the make-parents form returned **`rc=0`** against the already-existing folder. Two things follow: **the file-transfer protocol at version 3 gives no way to tell *already exists* from any other failure** without a separate stat and the race that comes with it, and **the make-parents form — what a builder usually writes — is the silent collision path**, where two jobs land in one folder with nothing reported.

## 7.1 The negative result, and it is silent

All seventeen hostile names were written on the rental successfully — newline, tab, colon, backslash, leading dash, emoji and a 255-character name among them. **Linux took every one. Seventeen went out and fourteen came back.**

```
names written on the rental that came back: rental had 17, Mac now holds 14
*** COUNT MISMATCH: 3 name(s) reached the machine and did not come back ***
```

**The case trio.** The rental held three distinct files:

```
on the rental: ['JOBFILE.png', 'JobFile.png', 'jobfile.png']
on the Mac   : ['JobFile.png']
    JobFile.png    exists on Mac, contents = b'payload:capitals-differ-only-by-case C'
    jobfile.png    exists on Mac, contents = b'payload:capitals-differ-only-by-case C'
    JOBFILE.png    exists on Mac, contents = b'payload:capitals-differ-only-by-case C'
```

All three names resolve to one file holding the **last** payload fetched. **Two files were destroyed.**

**Unicode spelling.** The rental held two distinct files with different payloads — one composed, one decomposed. The Mac holds **one**, stored composed, containing the decomposed payload; both spellings open it.

**Control, same sitting.** Writing two names differing only in case on the Mac leaves one file holding the second payload. The Mac volume is case-insensitive and normalisation-insensitive. **The rental kept every variant distinct, so the loss is at the Mac end and not in transit.**

**Every fetch reported success** — no error, no warning, no exception. This is exactly the negative the question predicted, *a name that reaches the machine intact and cannot be got back*, and it is **silent**. **Frames rendered into distinct names on the rental can arrive on the user's computer as fewer files than were made, with nothing indicating anything was lost.**

Everything else survived byte for byte: spaces, leading and trailing spaces, Cyrillic, Japanese, emoji, colon, backslash, quote, newline, tab, leading dash, and the 255-character name.

---

# 8. Fact 92 — the clock — NOT ANSWERED, and the record is here

The container has **no time client and no service manager** — the usual query answers `System has not been booted with systemd as init system (PID 1). Can't operate.` So a round-trip-compensated client was deployed, querying three public time sources and keeping the lowest-delay sample every 60 seconds.

**It ran 44.1 minutes and produced 45 samples**, from `2026-08-17T19:55:35Z` to `2026-08-17T20:39:42Z`, and then stopped when its tester's session ended.

```
first: 2026-08-17T19:55:35Z pool.ntp.org        +0.004749 rtt 0.026138
last : 2026-08-17T20:39:42Z time.cloudflare.com +0.008334 rtt 0.026630
min offset = -0.001031 s ; max offset = +0.008334 s ; spread = 0.009365 s
```

**The clock never left a 9.4 ms band.** The apparent trend is smaller than the sample spread, so it is noise rather than established drift.

**This does not answer the fact.** The fact asks about the length of a render and 44 minutes is not that. What it adds to the half-hour already on record is fourteen more minutes.

**What a drift large enough to show in a duration a user reads would look like.** Durations are read to the second, so the clock would have to move about a second across a render — at the widest band seen, over a hundred hours of rendering. A real failure would look like the offset marching in one direction past about 100 ms and not returning, which is the signature of an undisciplined clock, **or a step correction landing mid-render, which is what would actually corrupt a duration.** Neither occurred in 45 samples.

**The limit that matters: this is a container.** It has no clock of its own — the reading is the host kernel's and nothing inside can discipline it. **What was measured is physical machine 142047's clock, and a different host is a different answer.**

---

# 9. One observation that is NOT this stage's to rule on

The tester on Facts 5 and 77 recorded that **with the sequencer strip active, the scene's own file output nodes wrote nothing**, isolated by changing that one variable: with the strip off and nothing else changed, fifteen files appeared across both nodes and the regular output; with it on, only the second scene's node and the regular output wrote. The reader reported the same shape as data — `sequencer_takes_over: true`, `compositor_runs: false`.

**This is a claim about how Blender behaves, and this stage does not make those.** It is recorded here as *what that box did on that day with that fixture* and nothing more, and it is **filed as a question owed to Blender Research rather than as a fact.**

It matters because it bears on Row 124's gating conditions and Row 128's count, and a count that is wrong here promises folders that are never made. **Nothing may be built on it until Blender Research answers it.**

---

# 10. Ten things nobody asked about, and a builder needs

1. **A newline in a file name splits a listing across lines** on both routes. Line-oriented parsing of a listing is unsafe.
2. **The file-transfer client's text output is lossy though the protocol is not** — no year or seconds for recent entries, no timezone ever, symlink targets dropped, and non-Latin names escaped into octal.
3. **The file-transfer route carries no allocated-block count**, so a sparse file's lie cannot be caught on it.
4. **Every non-interactive command under the non-root login prints `bash: /root/.bashrc: Permission denied` on the error stream.** Anything treating a non-empty error stream as failure trips on this machine.
5. **Uptime and load are the host's, not the container's** — the box reported `up 7 days, 11:22` on a rental started that day.
6. **The usual time-configuration query does not work at all** on a container.
7. **Cycles' adaptive sampling ends a render early**, so a high sample count is not a reliable way to keep a machine busy.
8. **Cycles renders on the processor unless the card is selected explicitly** — a plain background render used four cores and no card.
9. **Scaling a multilayer OpenEXR image returns success and does nothing.** No exception, no message.
10. **Detaching with `nohup` over a command channel does not release the channel**; a new session does.

And one from the caller's end: **a scratchpad path can exceed the 104-byte limit a socket name may take**, which silently broke connection multiplexing until it was found.

---

# 11. What was not posed at all

**Blender 5.1** — not installed, so nothing here speaks for it.

**A file naming an engine the chosen Blender lacks** — Cycles is enabled on all three builds here, including under factory settings.

**Whole kinds of recorded location**: a font (no font file exists on this container, checked and absent), a sound, a volume, a point cache, a bake folder, a geometry-nodes bake folder, a sequencer proxy folder, and **a path in a custom property**. Of the locations the bulk means miss, the fixture held the render output path and the file output node folder and none of the others.

**A collision between two destinations on one name.** **A file output node inside a node group in 4.5.** **The Render Layers route into a second scene** — the reader looks for it and reported none, which is not proof it would find one.

**The multilayer small copy in 5.0 and 5.2** through Blender's own compositor.

**The proxy against the direct route** for the transfer server's capabilities, for the reason in §3.7.

**Anything on Windows or macOS as a rendering machine**, and any second physical disk.

---

# 12. Limits on everything in this document

One machine. One day. **One container, not a virtual machine.** One filesystem, `overlayfs`. One operating system at the rented end. One direction of rental — a Mac at the caller's end and Linux at the far end. One provider, reached over its proxy only. One Mac, one case-insensitive volume at default settings; **a case-sensitive volume would behave differently and was not posed.** One file-transfer client at 10.2p1 against one server at 8.9p1 — though the attributes measured are in the protocol rather than in either. One unprivileged login shape, with no groups, access lists or read-only mounts. One scene, one resolution, Cycles only. One card, one driver.

**And the load.** Three testers shared four cores throughout, at load average 2.5 to 6.5. Every timing here is an upper bound rather than a measurement of an idle machine, and the one question that turns on separating loads — whether a read disturbs a running render — could not be answered because of it.

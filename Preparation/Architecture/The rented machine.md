# The rented machine

What one rented render machine actually did when it was tested, written down so the Architecture stage has something real to lean on instead of guesswork.

**This is a frozen snapshot of one machine on one day.** Everything below was observed on **2026-08-06** on a single box rented from Vast.ai. It is an **input to** the Architecture stage, not a part of it: nothing here is a decision, a recommendation, or a plan. It is not kept up to date afterwards, and a different rented machine may differ in every particular — different provider, different card, different disk, different everything. Read it as *this is what that box did*, never as *this is what a rented machine does*.

Every claim here is **observed**. The commands run and the machine's own words back are given so anyone can repeat them.

---

## What the machine was

Reached over SSH:

```
ssh -i ~/.ssh/id_ed25519 -p 59334 root@213.181.122.2
```

```
Operating System: Ubuntu 22.04.5 LTS
          Kernel: Linux 6.8.0-87-generic
    Architecture: x86-64
 Static hostname: ubuntu
```

One NVIDIA GeForce RTX 4090 with 24 GB, driver 580.105.08. An AMD Ryzen Threadripper PRO 3975WX with twelve of its cores given to this machine. 79 GB of memory. About 90 GB of disk free at the start of testing. Four Blenders under `/opt/blender/` — 4.5.12, 5.0.1, 5.1.2 and 5.2.0.

All work below happened in `/root/smoozy/Machine notes/`.

### Other work was running the whole time

This matters for every number in here. Throughout the testing, other agents were running Blender on the same box for a different piece of work. The load reading — a rough count of how much is queued up — sat between **8 and 19** for most of the session, on a machine with twelve cores. Twelve would be "fully busy".

```
 07:46:03 up  3:59,  2 users,  load average: 18.99, 12.02, 8.33
```

By the end of the session that had fallen away, but Blender was still running:

```
 08:18:11 up  4:31,  2 users,  load average: 0.13, 1.29, 5.16
28390 /opt/blender/blender-4.5.12-linux-x64/blender -b test.blend -E NOT_A_REAL_ENGINE ...
556287 /opt/blender/blender-5.0.1-linux-x64/blender -p 0 0 1900 1100 --python ...
```

So every speed figure below is **a figure from a busy machine**. Where that bites hardest is said at the point it bites.

---

## 1. The disk

### What is there

```
Filesystem      Size  Used Avail Use% Mounted on
/dev/root       126G   37G   90G  30% /
tmpfs            40G     0   40G   0% /dev/shm
tmpfs            16G  1.4M   16G   1% /run
/dev/vda15      105M  6.1M   99M   6% /boot/efi
```

One real disk of 126 GB, with 90 GB free. It is **ext4**, on its own partition:

```
TARGET SOURCE    FSTYPE OPTIONS
/      /dev/vda1 ext4   rw,relatime,discard,errors=remount-ro
```

Three things about it are worth having.

**Nothing is held back for emergencies.** A normal Ubuntu disk keeps a slice of itself aside so that the machine can still function when it fills up. On this one that slice is zero:

```
Reserved block count:     0
Block size:               4096
Inode count:              16773120
Free inodes:              16304236
```

So the disk can genuinely be filled to the last byte.

**The 40 GB under `/dev/shm` is not disk.** It is memory dressed up as a folder. Writing there eats the machine's 79 GB of memory rather than its 126 GB of disk.

**A file can claim space it has not got.** Asked for a 500 GB file on a 126 GB disk with `truncate -s 500G sparse.bin`, then `ls -lh`, `du -h` and the free space before and after:

```
=== free before ===
  89G
-rw-r--r-- 1 root root 500G Aug  6 08:10 sparse.bin
0	sparse.bin
=== free after ===
  89G
```

It succeeded instantly and used nothing. Asking for the space *properly* — `fallocate -l 5G falloc.bin` — does reserve it, and does show up:

```
-rw-r--r-- 1 root root 5.0G Aug  6 08:10 falloc.bin
5.1G	falloc.bin
  84G
```

So the size a file reports and the space it occupies are two different numbers here.

### Writing a big file

Written with the other agents' renders running, using `dd` three ways — hand it over, hand it over and wait for the disk, and read it straight back:

```
dd if=/dev/zero of=w2g.bin  bs=1M count=2048
dd if=/dev/zero of=w2gs.bin bs=1M count=2048 conv=fdatasync
dd if=w2gs.bin  of=/dev/null bs=1M iflag=direct
```

```
=== 2GB write, buffered ===
2147483648 bytes (2.1 GB, 2.0 GiB) copied, 2.25068 s, 954 MB/s
=== 2GB write, flushed to disk (conv=fdatasync) ===
2147483648 bytes (2.1 GB, 2.0 GiB) copied, 3.24613 s, 662 MB/s
=== read back ===
2147483648 bytes (2.1 GB, 2.0 GiB) copied, 1.08339 s, 2.0 GB/s
```

Two seconds to hand off 2 GB, three and a quarter to know it is really on the disk, one to read it back. That is fast storage, and it was fast while twelve cores were busy with something else.

The disk describes itself as a spinning one — `/sys/block/vda/queue/rotational` reads `1` — which does not match the speeds. It is a virtual disk, and that reading is not to be trusted.

### Filling it

**Not run.** Filling the disk would have broken the other agents' renders mid-flight. The exact test is in *Needs the machine to itself* at the end.

---

## 2. Getting files on and off

The link is plain SSH. `scp`, `sftp` and `rsync` are all present on the machine.

**Down: 500 MB took 6 minutes 35 seconds.** That is about **1.3 MB a second**, or roughly 11 megabits.

```
scp -i ~/.ssh/id_ed25519 -P 59334  ./down500.bin  2.63s user 2.35s system 1% cpu 6:35.49 total
807092c5d0f4cab600826e92c082a190  down500.bin
```

The copy arrived intact — the fingerprint matched the one taken on the machine before sending.

**Up: the same 500 MB took 18 minutes 8 seconds.** That is about **0.48 MB a second**, or roughly 4 megabits — a little over a third of the download speed. It arrived intact:

```
807092c5d0f4cab600826e92c082a190  probeA.bin
-rw-r--r-- 1 root root 524288000 Aug  6 08:13 probeA.bin
```

**Be careful with the upload figure.** The download ran with nothing else of mine going on. The upload ran while other agents rendered, while a pair of my own test renders ran, and while a handful of small commands went over the same link. It is a floor, not a measurement of the link at its best.

Both figures were taken over an ordinary home connection from the Creator's Mac. The machine's own disk moves 600–900 MB a second, so the disk is not what makes this slow: the link is. A file that takes three seconds to write on the machine takes over six minutes to fetch, and eighteen to send.

`rsync` moved a smaller file faster — 15.7 MB in 5.8 seconds, about **2.7 MB a second** — but that is one short run against one long one, and the two are not a fair comparison.

### A quirk of the path

The folder name has a space in it, and that space broke the copy **three different ways in three tools**.

Fetching with `scp` wanted the space escaped with a backslash:

```
scp ... 'root@213.181.122.2:/root/smoozy/Machine\ notes/xfer500.bin' ./down500.bin
```

Sending with `scp` refused exactly that and wanted the path plainly quoted instead:

```
scp: dest open "/root/smoozy/Machine\\ notes/up500.bin": No such file or directory
scp: failed to upload file ./down500.bin to /root/smoozy/Machine\\ notes/up500.bin
```

```
scp ... ./down500.bin "root@213.181.122.2:/root/smoozy/Machine notes/probeA.bin"
```

And `rsync` given the plainly quoted path split it in two at the space and looked for two things that do not exist:

```
rsync: [sender] link_stat "/root/smoozy/Machine" failed: No such file or directory (2)
rsync: [sender] change_dir "/root/notes" failed: No such file or directory (2)
rsync(30894): warning: receiver has empty file list: exiting
```

`rsync` wanted the backslash. So the same path, written the same way, works in one tool and one direction and fails in the others.

### An interrupted transfer

A fetch of the 500 MB file was killed part-way through. What it left:

```
-rw-r--r--@ 1 artist  wheel  1566720 Aug  6 10:15 interrupted.bin
```

A send was killed the same way. What it left on the machine:

```
-rw-r--r-- 1 root root 2350080 Aug  6 08:17 upcut.bin
```

**In both directions, a killed copy leaves a piece of the file under the final name.** No marker, no `.part`, no temporary name, nothing to say it is incomplete. Half a file looks exactly like a whole one — right name, right place, wrong size, and a fingerprint that does not match:

```
full file:      807092c5d0f4cab600826e92c082a190   524288000 bytes
what was left:  ef717c055f8e930e322f869215a2bbc7     1566720 bytes
```

**A broken copy can be picked up where it stopped**, using `rsync` rather than `scp`. A 20 MB file was fetched whole, then chopped to 5 MB to stand in for a broken copy, and `rsync` was pointed at the piece:

```
sent 13742 bytes  received 15742820 bytes
Total file size: 20971520 B
Unmatched data: 15729752 B
Matched data: 5241768 B
```

It sent only the 15.7 MB that was missing and matched the 5 MB already there, and the finished file's fingerprint was right.

One catch on the Creator's side. The Mac's `rsync` is Apple's own, not the usual one, and it does not take the flags normally used for resuming:

```
openrsync: protocol version 29
```

Asked for `--append-verify` it printed its list of allowed flags and did nothing. Plain `rsync` with no flags did the resume shown above anyway. The rented machine has the ordinary `rsync version 3.2.7`.

---

## 3. The graphics card

### What it says about itself

```
+-----------------------------------------------------------------------------------------+
| NVIDIA-SMI 580.105.08             Driver Version: 580.105.08     CUDA Version: 13.0     |
|   0  NVIDIA GeForce RTX 4090        On  |   00000000:00:07.0 Off |                  Off |
| 30%   25C    P8             27W /  450W |     567MiB /  24564MiB |      0%      Default |
```

More detail:

```
    Product Name                          : NVIDIA GeForce RTX 4090
    Product Brand                         : GeForce
    Product Architecture                  : Ada Lovelace
    Persistence Mode                      : Enabled
    GPU UUID                              : GPU-00bde91e-bdfd-2da4-48f5-12c783399cb0
    VBIOS Version                         : 95.02.3C.40.1B
    GPU Virtualization Mode
        Virtualization Mode               : Pass-Through
```

**Pass-Through** means the whole card is handed to this machine rather than sliced up between tenants. There is one card, numbered 0 here — though the machine's own settings say it was the host's card number 3, so the physical server holds at least four.

**567 MB of the card's 24 GB was already spoken for before any render started.** The desktop that runs on this machine takes it:

```
|    0   N/A  N/A            1328      G   /usr/lib/xorg/Xorg                      155MiB |
|    0   N/A  N/A            1839      G   /usr/bin/kwin_x11                        37MiB |
|    0   N/A  N/A            1891      G   /usr/bin/plasmashell                     30MiB |
|    0   N/A  N/A            4752      G   ...u/libexec/kscreenlocker_greet         30MiB |
```

### What Blender sees

Asked from outside, with no window:

```
CUDA -> [('NVIDIA GeForce RTX 4090', 'CUDA'), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU')]
OPTIX -> [('NVIDIA GeForce RTX 4090', 'OPTIX'), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU')]
HIP -> []
ONEAPI -> []
METAL -> []
```

It also prints a warning every single time it starts, on a machine with no AMD card in it:

```
cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
```

That warning is normal here and means nothing is wrong.

### Two renders at once

The same scene — a default cube, 1920 by 1080, 400 samples, OptiX on the card — rendered once on its own and then twice side by side. The scene was made by this, saved on the machine as `/root/smoozy/Machine notes/gpuscene.py`:

```python
import bpy, sys
s = bpy.context.scene
s.render.engine = "CYCLES"
p = bpy.context.preferences.addons["cycles"].preferences
p.compute_device_type = "OPTIX"
p.get_devices()
for d in p.devices:
    d.use = (d.type == "OPTIX")
s.cycles.device = "GPU"
s.cycles.samples = 400
s.render.resolution_x = 1920
s.render.resolution_y = 1080
s.render.filepath = sys.argv[-1]
s.render.image_settings.file_format = "PNG"
bpy.ops.render.render(write_still=True)
```

Run as `blender -b --factory-startup -noaudio --python gpuscene.py -- <full path to the picture>`.

Alone:

```
00:05.439  render           | Saved: '/root/smoozy/Machine notes/one.png'
ONE took 5.66 seconds
```

Two together:

```
--- nvidia-smi mid-run ---
pid, process_name, used_gpu_memory [MiB]
546419, /opt/blender/blender-5.2.0-linux-x64/blender, 2816 MiB
546420, /opt/blender/blender-5.2.0-linux-x64/blender, 2816 MiB
utilization.gpu [%], memory.used [MiB], memory.total [MiB]
0 %, 5989 MiB, 24564 MiB
```

```
00:09.260  render           | Saved: '/root/smoozy/Machine notes/twoA.png'
A took 9.48 seconds
00:09.211  render           | Saved: '/root/smoozy/Machine notes/twoB.png'
B took 9.42 seconds
```

Both finished. Neither failed, neither waited for the other, and both wrote their picture. Each took about nine and a half seconds instead of five and a half. Two together therefore took less time overall than two one after the other — nine and a half seconds against eleven — while each one took nearly twice as long as it would have alone. Each held 2816 MB of the card while it ran.

Two other agents were also rendering on the same card during this. The `0 %` reading is a single instant sample and should not be read as "the card was idle".

### Asking for more card memory than there is

**Not run.** Filling 24 GB of card memory would have thrown the other agents' renders off it. The exact test is in *Needs the machine to itself*.

---

## 4. Memory

```
               total        used        free      shared  buff/cache   available
Mem:            79Gi       5.1Gi        47Gi        17Mi        26Gi        73Gi
Swap:             0B          0B          0B
```

**There is no swap at all.** Nothing catches a program that runs out of memory — no slow-but-alive fallback. It is 79 GB and then nothing.

The machine will refuse an obviously impossible request rather than let it through:

```
=== ask for 200 GB without touching it ===
malloc of 200 GB returned: None

=== python list allocation of 200 GB ===
Traceback (most recent call last):
  File "<string>", line 2, in <module>
MemoryError
```

A refusal, straight away, with the program left alive to notice. That is because the machine is set to guess at obviously-too-large requests:

```
/proc/sys/vm/overcommit_memory : 0
/proc/sys/vm/overcommit_ratio  : 50
```

A cap can also be put on a single command, and it produces the same clean refusal at the cap rather than at 79 GB:

```
bash -c "ulimit -v 1048576; python3 -c \"b=bytearray(2*1024**3); print(len(b))\""
Traceback (most recent call last):
  File "<string>", line 1, in <module>
MemoryError
```

**The other half of the question was not run.** Refusing one impossible request is not the same as running out of memory. A program that grows slowly, taking a little at a time until the machine has none left, gets no such refusal, and what the machine does then was not tested here. Doing it with other renders running risked killing them. The exact test is in *Needs the machine to itself*.

One limit that already exists on every command:

```
open files                          (-n) 1024
max locked memory           (kbytes, -l) 10368960
stack size                  (kbytes, -s) 8192
core file size              (blocks, -c) 0
```

---

## 5. Staying alive

### Already established (run by the runner, 2026-08-06)

**A render started detached survives the connection dropping.** Started with

```
setsid nohup env ... blender -b file.blend -o out/f_ -a > log 2>&1 < /dev/null &
```

the connection was then closed and a fresh one opened. The process was still running, still writing frames, its parent now the system itself — parent process 1, its own session leader. Ten frames were on disk and the log was still advancing.

**A render *not* detached dies when the connection drops.** Sent the hang-up signal that a dropped connection sends, the process ended at once — `Hangup` — with two finished frames on disk and the third, in flight, lost entirely.

### What is there for keeping work alive

```
tmux               /usr/bin/tmux
screen             /usr/bin/screen
nohup              /usr/bin/nohup
setsid             /usr/bin/setsid
systemd-run        /usr/bin/systemd-run
start-stop-daemon  /usr/sbin/start-stop-daemon
at                 MISSING
batch              MISSING
supervisord        MISSING
runit              MISSING
s6-svscan          MISSING
daemonize          MISSING
```

The plain ones are all there. **The sentence that stood here — *"The ones that watch a job and bring it back are not"* — is wrong and was corrected 2026-08-20**: this machine runs systemd as its first process and `systemd-run` is in the table above as present, so a restart-capable supervisor is there. What is true is narrower and is the next heading's own answer: systemd restarts nothing until a unit asks it to. `systemd.service(5)`, read 2026-08-20 at `https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html` — *"Restart= Configures whether the service shall be restarted when the service process exits, is killed, or a timeout is reached. … **If set to no (the default), the service will not be restarted.**"*

### Would anything restart work by itself?

**No.** Nothing on the machine is set to run anyone's work.

```
=== cron jobs ===
no crontab for root
```

`/etc/cron.d/` holds nothing but a placeholder and a disk-check job. The scheduled jobs are all Ubuntu's own housekeeping — updates, log tidying, the manual pages index:

```
Thu 2026-08-06 11:28:11 UTC  apt-daily.timer                apt-daily.service
Fri 2026-08-07 06:31:50 UTC  apt-daily-upgrade.timer        apt-daily-upgrade.service
Fri 2026-08-07 00:00:00 UTC  logrotate.timer                logrotate.service
Fri 2026-08-07 04:01:37 UTC  systemd-tmpfiles-clean.timer   systemd-tmpfiles-clean.service
Mon 2026-08-10 00:09:22 UTC  fstrim.timer                   fstrim.service
```

Two of those are the daily software update, and the service that carries them out is running:

```
unattended-upgrades.service   loaded active running Unattended Upgrades Shutdown
```

So the machine can install software updates on its own, without being asked.

The connection itself is not kept alive from the machine's side:

```
clientaliveinterval 0
clientalivecountmax 3
tcpkeepalive yes
```

**One of the two keepalives is on and one is off, and this sentence used to flatten both into "switched off" — corrected 2026-08-20.** The block above contains `tcpkeepalive yes`, and `sshd_config(5)`, OpenBSD's own manual read 2026-08-20 at `https://man.openbsd.org/sshd_config.5`, gives **TCPKeepAlive** as *"Specifies whether the system should send TCP keepalive messages to the other side. … The default is yes (to send TCP keepalive messages), and the server will notice if the network goes down or the client host crashes."* **It is on and it is far too slow to matter here**: kernel.org's `ip-sysctl.txt`, read the same day, gives `tcp_keepalive_time` a default of *"2hours"* and `tcp_keepalive_intvl` 75 seconds, *"i.e. connection will be aborted after ~11 minutes of retries"*. What **is** off is the other one: `clientaliveinterval 0`, which the same manual gives as OpenSSH's own shipped default — *"The default is 0, indicating that these messages will not be sent to the client."* — and not this provider's choice. The client side, `ServerAliveInterval`, is off by default too. **So keeping an idle connection open is still the caller's job**, and the reason is the one that is off rather than a machine that sends nothing at all.

---

## 6. What is already installed

### Python

```
/usr/bin/python3
Python 3.10.12 (main, Jun 22 2026, 18:55:27) [GCC 11.4.0]
pip 22.0.2 from /usr/lib/python3/dist-packages/pip (python 3.10)
```

**There is no plain `python` command** — only `python3`. Blender carries its own Python and does not use this one: 5.2.0 reports `3.13.13`.

### Package managers

```
apt      /usr/bin/apt
apt-get  /usr/bin/apt-get
dpkg     /usr/bin/dpkg
snap     /usr/bin/snap
uv       /usr/local/bin/uv
conda    MISSING
mamba    MISSING
pipx     MISSING
```

### Media tools

```
ffmpeg     MISSING
ffprobe    MISSING
convert    /usr/bin/convert
magick     MISSING
oiiotool   MISSING
exiftool   MISSING
```

**There is no `ffmpeg` on the machine**, and no `ffprobe`. Nothing outside Blender is there to make, join or inspect a video file. What Blender itself can do about video is not a question about this machine and is not answered here.

### Everything else worth naming

```
rsync    /usr/bin/rsync
scp      /usr/bin/scp
sftp     /usr/bin/sftp
curl     /usr/bin/curl
wget     /usr/bin/wget
git      /usr/bin/git
tar      /usr/bin/tar
unzip    /usr/bin/unzip
zip      /usr/bin/zip
xz       /usr/bin/xz
zstd     /usr/bin/zstd
pigz     /usr/bin/pigz
jq       /usr/bin/jq
bc       /usr/bin/bc
aria2c   MISSING
```

Blender ships as a `.tar.xz`, and both `tar` and `xz` are present to open one.

Docker is installed and running, with nothing in it:

```
CONTAINER ID   IMAGE     COMMAND   CREATED   STATUS    PORTS     NAMES
Docker version 29.0.3, build 511dad6
```

There is also a **Blender installed as a snap** — version 5.2.0, straight from the Blender Foundation — quite separate from the four under `/opt/blender/`:

```
blender            5.2.0                           7599   latest/stable  blenderfoundation**  classic
```

### What a fresh Blender needs

Every one of the four Blenders finds every library it asks for:

```
4.5.12   missing libraries: 0
5.0.1    missing libraries: 0
5.1.2    missing libraries: 0
5.2.0    missing libraries: 0
```

And every one starts and reports itself:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
```

Blender brings almost everything with it. What it borrows from the machine is the graphics libraries and a handful of X window ones — `libGL`, `libGLX`, `libX11`, `libXi`, `libXext`, `libXfixes`, `libXrender`, `libSM`, `libICE` — plus the ordinary C libraries. All of those are here because this machine has a desktop on it. A bare machine without one might not have them.

**One thing to know about running with no window.** Because the machine's settings point at its desktop, Blender started from a plain connection prints this before doing anything:

```
Authorization required, but no authorization protocol specified
xcb_connection_has_error() returned true
```

It renders perfectly well anyway. Clearing that one setting first makes the message go away.

---

## 7. Time and identity

### The clock

```
               Local time: Thu 2026-08-06 07:47:21 UTC
           Universal time: Thu 2026-08-06 07:47:21 UTC
                 RTC time: Thu 2026-08-06 07:47:21
                Time zone: Etc/UTC (UTC, +0000)
System clock synchronized: yes
              NTP service: active
```

**The machine is on UTC.** Every file listing in this document shows a UTC time, because that is the only time the machine has. It has no idea what time zone anyone using it is in.

It keeps itself set from the internet:

```
ServerName=ntp.ubuntu.com
ServerAddress=91.189.91.157
PollIntervalUSec=34min 8s
NTPMessage={ Leap=0, Version=4, Mode=4, Stratum=2, ... Jitter=24.967ms }
```

**The clock was checked against a real time source, twice, half an hour apart.** The machine was asked to compare itself directly against `ntp.ubuntu.com`. First check:

```
ntp.ubuntu.com  clock is off by +0.012043 seconds  (round trip 0.078 s)
ntp.ubuntu.com  clock is off by -0.006459 seconds  (round trip 0.041 s)
ntp.ubuntu.com  clock is off by -0.006505 seconds  (round trip 0.041 s)
```

Second check, half an hour later:

```
clock is off by +0.009377 seconds
clock is off by +0.003996 seconds
```

The third reading of the second check timed out — the time server did not answer within five seconds.

So the clock sat within about a hundredth of a second of real time on both occasions. **No drift showed over half an hour.** What that measures is the machine's own time service doing its job — it corrects itself every half hour or so. How far the clock would wander with that switched off was not tested, and neither was anything longer than half an hour.

### Its names and addresses

```
 Static hostname: ubuntu
      Machine ID: d9da508ad8ee416fa69ebfe93c2a1fb2
         Boot ID: 9f13a2f617304bd3acbb400c9dca0426
```

The name is just `ubuntu`. Nothing about it identifies which machine was rented.

```
lo               UNKNOWN        127.0.0.1/8 ::1/128
ens3             UP             10.0.2.15/24 metric 100
tailscale0       UNKNOWN        fe80::e5e3:c2a:711:bb23/64
docker0          DOWN           172.17.0.1/16
```

**The machine does not know its own public address.** Its own address is `10.0.2.15`, a private one. Asked from outside, it comes back as `213.181.122.2` — but that is the provider's, not the machine's.

**Nor does it know which port the outside world reaches it on.** Its SSH is listening on the ordinary port 22:

```
LISTEN 0      128                      0.0.0.0:22         0.0.0.0:*    users:(("sshd",...))
```

The `59334` used to reach it is a mapping done by the provider outside the machine. The machine is told about it in a setting — `VAST_TCP_PORT_22="59334"` — but nothing inside it is listening on that number.

### What identifies the rental

The machine carries a set of settings from the provider naming the rental:

```
CONTAINER_ID=46942320
CONTAINER_LABEL=C.46942320
VAST_CONTAINERLABEL=C.46942320
VAST_TCP_PORT_22=59334
VAST_DEVICE_IDXS=3
PUBLIC_IPADDR=213.181.122.2
GPU_COUNT=1
PORTAL_VERSION=v3.1.3
```

Those same settings also hold **secrets in plain text** — an access key for the provider's own service, and a password for a web page it runs. They are not quoted here. Anything that reads the machine's settings and writes them into a log would be writing those out too.

### What would differ on a second rented box

**Only one machine was tested, so this cannot be answered.** What can be said is which things in this document are plainly particular to this box: the two addresses, the port, the rental number, the machine name, the card and its number on the host, the amount of disk and memory, the driver version, which Blenders are on it, which extra software is on it, and whether there is a desktop.

Everything else in here might repeat on a second box or might not. Nobody should assume either way from one machine.

---

## 8. The edges of the rental

**It is not a container. It is a whole virtual machine.** The rental had been described as a container, and the provider's own settings call it one, but every test says otherwise.

```
=== systemd-detect-virt ===
kvm
=== /proc/1/cgroup ===
0::/init.scope
=== /proc/1/comm ===
systemd
=== .dockerenv ===
ls: cannot access '/.dockerenv': No such file or directory
=== dmidecode ===
Standard PC (i440FX + PIIX, 1996)
```

```
       Icon name: computer-vm
         Chassis: vm
  Virtualization: kvm
```

It has its own kernel, its own `systemd` running as the first process, its own disk partitions, and its own boot record. The provider calls the rental a container in its own settings — `CONTAINER_ID`, `CONTAINER_LABEL` — but what those settings sit inside is a virtual machine.

**Root here is real root.** Every capability a Linux machine has is held:

```
Current: =ep
Bounding set =cap_chown,cap_dac_override,...,cap_sys_module,cap_sys_admin,cap_sys_boot,cap_sys_time,...
```

System settings can be changed. Read and written back unchanged, to prove it works without changing anything:

```
vm.swappiness reads: 60
vm.swappiness = 60
```

Kernel modules can be loaded — asking for one that does not exist failed for the right reason, not for lack of permission:

```
modprobe: FATAL: Module definitely_not_a_module not found in directory /lib/modules/6.8.0-87-generic
```

Twelve cores, given by pinning rather than by a quota — the machine simply cannot see the other twenty:

```
pid 1's current affinity list: 0-11
=== cpuinfo count ===
12
model name	: AMD Ryzen Threadripper PRO 3975WX 32-Cores
```

The chip's name says thirty-two because that is what the host's chip is called. Only twelve are here.

### What a reboot would mean

**Not tested — the machine was not rebooted.** What can be read is that it has been rebooted before and kept its disk:

```
-1 92f3914babbd49e5b08eafa201b91881 Mon 2025-11-24 17:21:50 UTC—Mon 2025-11-24 17:21:57 UTC
 0 9f13a2f617304bd3acbb400c9dca0426 Thu 2026-08-06 03:46:37 UTC—Thu 2026-08-06 08:06:24 UTC
```

```
reboot   system boot  6.8.0-87-generic Thu Aug  6 03:46   still running
reboot   system boot  5.15.0-1088-kvm  Mon Nov 24 16:54 - 17:21  (00:27)
wtmp begins Mon Nov 24 16:54:41 2025
```

The record of a boot from November 2025 was still on the disk in August 2026, and the kernel changed between the two. So the disk survived at least one restart and at least one system upgrade. The current session began at 03:46 that morning.

### What is shared with the host

The chip is shared — twelve of its thirty-two cores. The card is **not**: `Pass-Through` means the whole 4090 is this machine's, and the only things using it were this machine's own. Nothing belonging to another tenant was visible from inside: all 330 processes running were this machine's.

### What else is running on it

The provider's own software is running whether it is wanted or not — a web portal, a tunnel manager, a video-streaming desktop, a relay server, a Tomcat server, Docker, snapd, Tailscale, a full KDE desktop with a lock screen over it:

```
instance-portal.service       loaded active running Instance Portal
tunnel-manager.service        loaded active running Tunnel Manager
coturn.service                loaded active running coTURN STUN/TURN Server
sddm.service                  loaded active running Simple Desktop Display Manager
docker.service                loaded active running Docker Application Container Engine
tailscaled.service            loaded active running Tailscale node agent
tomcat.service                loaded active running Tomcat Service
unattended-upgrades.service   loaded active running Unattended Upgrades Shutdown
```

The overall system state reports as **degraded**, meaning at least one thing on it has failed to start. Rendering works regardless.

**The firewall is off.** Nothing on the machine filters what comes in:

```
Status: inactive
Chain INPUT (policy ACCEPT)
```

What the outside world can actually reach is decided outside the machine, by the provider's port mapping. That was not tested from outside.

A desktop is genuinely running, with a lock screen over it:

```
name of display:    :0
X.Org version: 1.21.1.4
   1328 Xorg
   1839 kwin_x11
   1891 plasmashell
   4752 kscreenlocker_g
```

Blender's window can be opened on it and photographed, but a pointer click reaches nothing, because the lock screen is in the way.

### One thing already established

**Root ignores file permissions.** A folder made unwritable is written to anyway. Dropping one power for a single command restores ordinary behaviour:

```
capsh --drop=cap_dac_override,cap_dac_read_search -- -c "<command>"
```

### And one more, about names

The filesystem **tells capitals apart** — `Case.txt` and `case.txt` are two different files. A file name may be up to **255** characters and a whole path up to about **4096**; one character more is refused with `File name too long`. The language setting is `C.UTF-8`.

---

## 9. The graphics driver

**Nothing here was changed.** Removing or reinstalling a driver can leave a machine unreachable, so only what could be read was read.

### What is installed

Driver **580.105.08**, which is the *open* kernel build, installed as ordinary Ubuntu packages:

```
ii  nvidia-driver-580-open       580.105.08-0ubuntu1   NVIDIA driver (open kernel) metapackage
ii  nvidia-dkms-580-open         580.105.08-0ubuntu1   NVIDIA DKMS package (open kernel module)
ii  nvidia-kernel-source-580-open 580.105.08-0ubuntu1  NVIDIA kernel source package
ii  libnvidia-compute-580        580.105.08-0ubuntu1   NVIDIA libcompute package
ii  libnvidia-gl-580             580.105.08-0ubuntu1   NVIDIA OpenGL/GLX/EGL/GLES GLVND libraries and Vulkan ICD
ii  libnvidia-encode-580         580.105.08-0ubuntu1   NVENC Video Encoding runtime library
ii  libnvidia-decode-580         580.105.08-0ubuntu1   NVIDIA Video Decoding runtime libraries
ii  nvidia-firmware-580          580.105.08-0ubuntu1   Firmware files used by the kernel module
ii  nvidia-persistenced          580.105.08-0ubuntu1   daemon to maintain persistent software state
ii  nvidia-settings              580.105.08-0ubuntu1   Tool for configuring the NVIDIA graphics driver
ii  xserver-xorg-video-nvidia-580 580.105.08-0ubuntu1  NVIDIA binary Xorg driver
ii  nvidia-container-toolkit      1.18.0-1             NVIDIA Container toolkit
```

**Installed through DKMS** — the arrangement where the driver is rebuilt to match whatever kernel the machine boots. That rebuilding was not seen happening; what was seen is that the driver is installed that way, and that the machine installs its own updates. So the kernel and the driver can both move without anyone asking.

### What is loaded right now

```
nvidia_uvm           2158592  0
nvidia_drm            139264  6
nvidia_modeset       1814528  25 nvidia_drm
nvidia              14385152  317 nvidia_uvm,nvidia_modeset
```

```
NVRM version: NVIDIA UNIX Open Kernel Module for x86_64  580.105.08  Release Build  (dvs-builder@U22-I3-B10-02-5)  Wed Oct 29 22:29:53 UTC 2025
GCC version:  gcc version 12.3.0 (Ubuntu 12.3.0-1ubuntu1~22.04.2)
```

Note the two dates. The driver was built in **October 2025** and the machine was running it in **August 2026**.

### What a program can learn about it

Four ways, all readable without special permission:

```
=== nvidia-smi ===
| NVIDIA-SMI 580.105.08             Driver Version: 580.105.08     CUDA Version: 13.0     |

=== /proc/driver/nvidia/version ===
NVRM version: NVIDIA UNIX Open Kernel Module for x86_64  580.105.08 ...

=== dpkg -l | grep nvidia ===
(the package list above)

=== the libraries themselves ===
	libnvidia-ml.so.1 (libc6,x86-64) => /lib/x86_64-linux-gnu/libnvidia-ml.so.1
	libcuda.so.1 (libc6,x86-64) => /lib/x86_64-linux-gnu/libcuda.so.1
	libEGL_nvidia.so.0 (libc6,x86-64) => /lib/x86_64-linux-gnu/libEGL_nvidia.so.0
```

The device files it creates:

```
crw-rw-rw- 1 root root 195, 254 Aug  6 03:46 /dev/nvidia-modeset
crw-rw-rw- 1 root root 235,   0 Aug  6 03:47 /dev/nvidia-uvm
crw-rw-rw- 1 root root 235,   1 Aug  6 03:47 /dev/nvidia-uvm-tools
crw-rw-rw- 1 root root 195,   0 Aug  6 03:46 /dev/nvidia0
crw-rw-rw- 1 root root 195, 255 Aug  6 03:46 /dev/nvidiactl
```

**The CUDA toolkit is not installed** — only the driver:

```
nvcc MISSING
ls: cannot access '/usr/local/cuda*': No such file or directory
```

The renders in section 3 ran on the card anyway, so nothing was needed that was not there.

---

## Needs the machine to itself

Three tests were not run, because other agents were rendering on the box throughout and each of these would have broken their work. They are written out exactly so the runner can run them on an idle machine.

**Before each one**, check that nothing else is running:

```
ssh -i ~/.ssh/id_ed25519 -p 59334 root@213.181.122.2 'pgrep -ax blender || echo NOTHING RUNNING'
```

**And check again afterwards.** If anything is running, do not start.

Use `-x`, not `-f`. `pgrep -f blender` matches the command you are typing, because that command has the word "blender" in it, so it always says something is running:

```
=== pgrep -f blender (matches its own command line) ===
4
=== pgrep -x blender (matches only the program itself) ===
2
```

That check was made at the start of the session and again at the end, and Blender was running both times, which is why none of these three was run:

```
28390 /opt/blender/blender-4.5.12-linux-x64/blender -b test.blend -E NOT_A_REAL_ENGINE ...
558084 /opt/blender/blender-4.5.12-linux-x64/blender -p 0 0 1900 1100 --python ...
```

The file `gpuscene.py` that test A uses was left in place at `/root/smoozy/Machine notes/gpuscene.py`. Its contents are in section 3 above: a default cube at 1920 by 1080, 400 samples, on the card.

The guard in each script below stops the whole thing if Blender is running. Every script tidies up after itself.

### A. Filling the disk

What it answers: what error comes back when the disk genuinely fills mid-write, and what is left behind.

```
ssh -i ~/.ssh/id_ed25519 -p 59334 root@213.181.122.2 'bash -s' <<"OUTER"
set -u
cd "/root/smoozy/Machine notes" || exit 1
if pgrep -x blender > /dev/null; then echo "ABORT: something is rendering"; pgrep -ax blender; exit 1; fi

echo "=== free before ==="; df -h /

echo "=== fill it, leaving 200 MB ==="
AVAIL_M=$(df -m --output=avail / | tail -1)
dd if=/dev/zero of=ballast.bin bs=1M count=$((AVAIL_M - 200)) status=none
echo "ballast written, exit was $?"
df -h /

echo "=== now write 500 MB into 200 MB of space ==="
dd if=/dev/zero of=overflow.bin bs=1M count=500 2>overflow.err
echo "dd exit was $?"
echo "--- what dd said ---"; cat overflow.err

echo "=== what is left behind ==="
ls -la overflow.bin; du -h overflow.bin; df -h /

echo "=== can anything at all be written now? ==="
echo hello > tiny.txt; echo "write exit was $?"; cat tiny.txt

echo "=== a Blender render onto a full disk ==="
unset DISPLAY
/opt/blender/blender-5.2.0-linux-x64/blender -b --factory-startup -noaudio \
  --python gpuscene.py -- "/root/smoozy/Machine notes/full.png" 2>&1 | tail -15

echo "=== clean up ==="
rm -f ballast.bin overflow.bin overflow.err tiny.txt full.png
df -h /
pgrep -ax blender || echo "still nothing else running"
OUTER
```

Worth recording: the exact words `dd` gives when the disk fills, whether the half-written file stays on disk at its part-written size, whether anything at all can be written afterwards, and what Blender says when it cannot save a frame.

### B. Asking the card for more memory than it has

What it answers: what Blender says when a render will not fit in 24 GB.

```
ssh -i ~/.ssh/id_ed25519 -p 59334 root@213.181.122.2 'bash -s' <<"OUTER"
set -u
cd "/root/smoozy/Machine notes" || exit 1
if pgrep -x blender > /dev/null; then echo "ABORT: something is rendering"; pgrep -ax blender; exit 1; fi

cat > bigmem.py <<"INNER"
import bpy, sys
s = bpy.context.scene
s.render.engine = "CYCLES"
p = bpy.context.preferences.addons["cycles"].preferences
p.compute_device_type = "OPTIX"
p.get_devices()
for d in p.devices:
    d.use = (d.type == "OPTIX")
s.cycles.device = "GPU"
s.cycles.use_auto_tile = False
s.cycles.samples = 16
# a very large picture is the simplest way to overrun a 24 GB card
s.render.resolution_x = 30000
s.render.resolution_y = 30000
s.render.resolution_percentage = 100
s.render.filepath = sys.argv[-1]
s.render.image_settings.file_format = "PNG"
bpy.ops.render.render(write_still=True)
INNER

unset DISPLAY
echo "=== card before ==="
nvidia-smi --query-gpu=memory.used,memory.total --format=csv

/opt/blender/blender-5.2.0-linux-x64/blender -b --factory-startup -noaudio \
  --python bigmem.py -- "/root/smoozy/Machine notes/huge.png" 2>&1 | tail -30
echo "=== card afterwards ==="
nvidia-smi --query-gpu=memory.used,memory.total --format=csv
ls -la huge.png 2>&1
rm -f huge.png bigmem.py
pgrep -ax blender || echo "still nothing else running"
OUTER
```

If 30000 by 30000 is not enough to overrun the card, raise it and run again.

Worth recording: whether Blender fails outright, falls back to the processor, or fills the card and stalls; the exact words of the error; and whether the card's memory is given back afterwards.

### C. Taking all the machine's memory

What it answers: what happens to a program that keeps taking memory until there is none, on a machine with no swap.

**This one can kill other things on the machine, possibly including the connection itself.** Run it only on an idle box, and expect that the connection may drop. Because of that, the whole thing is started detached and its output read from a file afterwards.

Start it:

```
ssh -i ~/.ssh/id_ed25519 -p 59334 root@213.181.122.2 'bash -s' <<"OUTER"
set -u
cd "/root/smoozy/Machine notes" || exit 1
if pgrep -x blender > /dev/null; then echo "ABORT: something is rendering"; pgrep -ax blender; exit 1; fi

free -h > eat-before.txt
cat > eatmem.py <<"INNER"
import time
chunks = []
gb = 0
while True:
    b = bytearray(1024**3)              # take a gigabyte
    b[::4096] = b"x" * len(b[::4096])   # and really touch every page of it
    chunks.append(b)
    gb += 1
    print("held", gb, "GB", flush=True)
    time.sleep(0.2)
INNER

setsid nohup python3 eatmem.py > eatmem.log 2>&1 < /dev/null &
echo "started, watch eatmem.log"
OUTER
```

Then, after a minute or two, read what happened — reconnecting first if the connection dropped:

```
ssh -i ~/.ssh/id_ed25519 -p 59334 root@213.181.122.2 'bash -s' <<"OUTER"
cd "/root/smoozy/Machine notes" || exit 1
echo "=== is it still going? ==="; pgrep -af eatmem.py || echo "it has ended"
echo "=== how far it got ==="; tail -5 eatmem.log
echo "=== what the system says it did ==="
journalctl -k --since "-15 min" --no-pager | grep -iE "out of memory|killed process|oom" | tail -20
echo "=== memory before and now ==="; cat eat-before.txt; free -h
echo "=== did anything else die? ==="; systemctl is-system-running; systemctl --failed --no-pager
rm -f eatmem.py eatmem.log eat-before.txt
OUTER
```

Worth recording: how many gigabytes it reached, whether it was killed or refused, the exact words the system logged, and whether anything else on the machine died with it.

---

## What was not established

- **What happens to a render when the disk fills under it.** Test A above.
- **What Blender says when a render will not fit on the card.** Test B above.
- **What happens when memory runs out for real.** Test C above.
- **What a reboot does.** The machine was not rebooted. All that is known is that a previous reboot kept the disk.
- **Whether the clock drifts over hours or days.** It was checked twice, half an hour apart, and did not.
- **What the link can really do.** Both transfer figures were taken with other work happening. Neither is the link at its best.
- **What the outside world can reach.** Nothing was tested from outside the machine except that the connection works on port 59334.
- **What a second rented machine looks like.** One machine was tested. Nothing here says which parts repeat.
- **Whether the driver can be changed and the machine come back.** Deliberately not attempted.
- **How long the rental lasts, what it costs, or what happens when it ends.** None of that is visible from inside the machine and none of it was asked.

---

# The tests that needed the machine to itself — run by the runner, 2026-08-06

The three tests above were written out but not run, because Blender was on the box throughout. Once every agent had finished and nothing was rendering, they were run. A fourth, the graphics driver, was added.

## A. Filling the disk

**What was done.** The disk was filled with a ballast file until two megabytes remained, then written into.

**What came back.** `dd` stopped with exit status 1. Its own error message was lost — the file it was writing the message into could not be written either, because the disk was full. The part-written file **stays on disk under its final name**, at the size it reached, with nothing marking it as incomplete:

```
-rw-r--r-- 1 root root 208498688 Aug  6 08:41 overflow.bin
199M	overflow.bin
```

Afterwards nothing at all could be written. A one-line write from the shell fails plainly and says so:

```
bash: line 17: echo: write error: No space left on device
```

**And then Blender was asked to render onto it.** This is the result worth carrying forward, and it is written into the Blender Research findings for the Writing topic as well. Blender printed its ordinary saving line and exited **0**, in all four versions — and the file it left is a piece of a picture:

```
4.5   exit 0   Saved: '…/full.png'   file on disk: 761856 bytes,  PNG marker True, end marker False
5.0   exit 0   Saved: '…/full.png'   file on disk: 1598884 bytes, PNG marker True, end marker False
5.1   exit 0   Saved: '…/full.png'   file on disk: 1598789 bytes, PNG marker True, end marker False
5.2   exit 0   Saved: '…/full.png'   file on disk: 1599662 bytes, PNG marker True, end marker False
```

So on a full disk the operating system fails loudly and Blender does not. Two other complaints appeared on the way, both about the disk rather than the render: `Could not generate a temp file name`, and `Unable to initialize the OptiX cache database`.

The ballast was removed and the disk returned to 90 GB free.

## B. Asking the card for more than it has

**What was done.** One frame at 30,000 by 30,000 on the card, through OptiX, in 4.5 and 5.2, with nothing else running.

**What came back — and it is not what the test set out to find.** Neither run reached the card's limit. Both were **killed by the machine itself** at exit status **137**, during denoising, having taken tens of gigabytes of ordinary memory:

```
Fra:1 Mem:65251.59M (Peak 65251.59M) | Time:01:31.42 | … | Scene | ViewLayer | Denoising
Killed
```

No message from Blender, no error, no file, and nothing in its output to say what happened — the machine simply took the process away. Anything watching would see a program vanish at 137 and nothing else.

**So the card's own limit was never reached**, and what Cycles says when 24 GB of card memory fills is still untested. A frame large enough to fill the card exhausts the machine's 79 GB first.

**Tried again four more ways on 2026-08-06, and still not reached.** Since a huge frame exhausts the machine before the card, the demand was moved from the frame to the scene: many large textures, all feeding the surface, at a small render size. What happened each time, with the card watched by `nvidia-smi` every two seconds throughout:

- 40 and then 120 generated images of 4096 by 4096 — rendered fine, exit 0. The node graph left all but the last disconnected, so this proved nothing.
- The same chained properly so every texture reaches the surface, 60 and then 120 of them — about 15 GB and 30 GB of texture. Both rendered, exit 0. The card never held more than **1,819 MB**.
- 26 images of 8192 by 8192 filled with real random pixels rather than a flat colour, about 26 GB in all, chained the same way. Rendered, exit 0. The card never held more than **359 MB**.

So Blender does not put that texture data on the card, and nothing here made it try. Whether that is Cycles declining to upload what a 256-pixel render will not sample, or something else, is not established — and guessing at it is exactly what this project forbids.

**What that means for the question.** What Cycles says when 24 GB of card memory genuinely fills is **still untested**, after five attempts on the right hardware. Two things would settle it and neither is available here: a card with much less memory than this one, so ordinary work fills it; or a scene from real work whose card-side demand is known to exceed 24 GB. It is recorded as attempted rather than as unreachable, because the attempts are the useful part — they show that filling this card on purpose is harder than it looks, and that the failure a large job actually produces on this machine is the operating system killing the process at 137.

## C. Taking all the memory

Not run as a separate test. Test B took it instead, and answered it: the machine kills the process outright at exit 137 with no warning to the program.

## D. The graphics driver

**What is installed.** Driver `580.105.08`, as the open kernel module, built on this machine rather than shipped ready-made — the module sits under `updates/dkms/`, which means it is rebuilt whenever the kernel changes:

```
filename:   /lib/modules/6.8.0-87-generic/updates/dkms/nvidia.ko
version:    580.105.08
vermagic:   6.8.0-87-generic SMP preempt mod_unload modversions
NVRM version: NVIDIA UNIX Open Kernel Module for x86_64  580.105.08  Release Build
GCC version:  gcc version 12.3.0 (Ubuntu 12.3.0-1ubuntu1~22.04.2)
```

A program can read the driver's version and the library's version separately: the library is a chain of links ending at `libcuda.so.580.105.08`, so the two can be compared without guessing.

**What a reinstall would do — asked without doing it.** This is the part worth knowing. Asking for the driver to be reinstalled does not reinstall it. It would fetch **580.178.04** from NVIDIA's own package source, a different and newer version from the one running:

```
Conf nvidia-modprobe (580.178.04-1ubuntu1 NVIDIA CUDA:developer.download.nvidia.com [amd64])
Conf xserver-xorg-video-nvidia-580 (580.178.04-1ubuntu1 …)
Conf libnvidia-cfg1-580 (580.178.04-1ubuntu1 …)
```

**And the machine may do that by itself.** Unattended upgrades are switched on and nothing is pinned:

```
APT::Periodic::Update-Package-Lists "1";
APT::Periodic::Unattended-Upgrade "1";
```

`apt-mark showhold` returns nothing, so no package is held back.

**It was not actually reinstalled, and the reason is part of the result.** A reinstall here is an upgrade to a version the running kernel module does not match, and the usual outcome is that the card stops answering until the machine restarts. The machine is rented and in use, and the information gained would not have justified making it unusable. What the dry run shows — that a reinstall silently becomes an upgrade, and that upgrades can happen unasked — is the fact worth having.

---

2026-08-06 — Opus 5 (Claude Code) — the four tests that needed an idle machine

---

# Rebooting the machine from the provider's console, mid-render

The Creator clicked reboot in Vast.ai's own interface at about 09:34:40 on 2026-08-06, while a 500-frame render was running detached and a heartbeat was counting finished frames every five seconds.

**What a program watching from outside sees.** The connection does not time out and is not refused. It answers and then breaks:

```
kex_exchange_identification: read: Connection reset by peer
Connection reset by 213.181.122.2 port 59334
```

**How long it was gone.** It booted again at 09:35:13 — under a minute from the click.

**The reboot is not a clean shutdown.** The machine's own login record shows the previous session ending in a crash rather than a logout:

```
user     pts/0        :0     Thu Aug  6 03:47 - crash  (05:47)
reboot   system boot  6.8.0-136-generi Thu Aug  6 09:35   still running
```

**The disk survived completely.** A marker file written seconds before the reboot was still there and still readable, along with all four Blender builds and all twenty-six working folders.

**And so did the finished frames — every one of them.** Blender had announced eleven frames as saved. All eleven were on disk afterwards, whole, ending properly as a PNG should. Nothing was part-written. So an abrupt reboot did not corrupt work already reported as done.

**But the folder holds more files than there are frames.** Six further files sat beside them at **zero bytes** — `f_0012.png` through `f_0017.png` — the placeholders Blender writes ahead of itself. So the output folder said seventeen and the truth was eleven:

```
whole 11, zero-byte 6, part-written 0
```

Anything counting files to judge progress would be six frames wrong, and every one of those six would be an unopenable file with a perfectly ordinary name.

**The render did not come back.** Nothing restarted it, and the machine offers nothing that would.

**The kernel changed underneath it.** The machine went in on `6.8.0-87-generic` and came up on `6.8.0-136-generic`. That upgrade had been fetched, unasked, at 05:05 that morning by the unattended-upgrade service, and was simply waiting for a restart:

```
/boot/vmlinuz-6.8.0-87-generic
/boot/vmlinuz-6.8.0-136-generic
```

**The graphics driver survived it.** Still `580.105.08`, and its kernel module had been rebuilt for the new kernel — which is what the DKMS arrangement is for. A render on the card worked immediately afterwards, at the first attempt. So the mismatch the earlier section warned about did not happen here. It is worth saying plainly that this is one reboot on one machine and not a guarantee: the same arrangement is what would produce a broken card if a rebuild ever failed.

---

# A correction to the disk-full result above

**The first time the disk-full result was checked, the check was faulty.** It tested whether a file's last eight bytes matched a twelve-byte marker, which can never be true, so every file looked incomplete whether it was or not. That fault was found while checking the reboot frames, where it wrongly reported eleven whole frames as damaged.

**The disk-full result was therefore run again, in all four versions, with a correct check.** It holds:

```
4.5: exit 0, said saved once, no error line -> 831488 bytes, ends properly: False
5.0: exit 0, said saved once, no error line -> 864256 bytes, ends properly: False
5.1: exit 0, said saved once, no error line -> 835584 bytes, ends properly: False
5.2: exit 0, said saved once, no error line -> 835584 bytes, ends properly: False
```

Every line Blender printed about errors, warnings or saving was read directly, with the unrelated HIP warning set aside, and there was **nothing but the saving line**. So the finding stands as written: on a full disk Blender announces the save, exits 0, says nothing about the disk, and leaves a picture that does not end.

---

2026-08-06 — Opus 5 (Claude Code) — the reboot, and a correction to how the disk-full result was verified

---

# Destroying the machine from the provider's console, mid-render

This is the one the project most needed. *The machine is destroyed* is a **scenario** in this project — a story played through the documents at the Specs stage — and until now it had only ever been reasoned about. On 2026-08-06 the Creator destroyed the machine in Vast.ai's interface while a 500-frame render was running.

Everything below was recorded **on the Creator's Mac**, not on the machine, because after this there is nothing left to read. Two watchers ran: one streaming the render's own log over the connection and stamping each line with the Mac's clock, and one asking the machine every three or four seconds how many frames it had written.

**There is no warning of any kind.** The render was working normally to the last instant. The final line to come through was frame 366 at sample 288 of 300 — mid-frame:

```
10:08:54 | 14:07.660  render  | Fra: 366 | Remaining: 00:00.05 | Mem: 38M | Sample 288/300
10:08:55 | Connection to 213.181.122.2 closed by remote host.
```

Nothing announced it. No shutdown message, no signal reaching Blender, no last words. The machine was rendering and then it was not there.

**The counting watcher saw the same thing, four seconds apart:**

```
10:08:46 | exit=0 | frames=362
10:08:50 | exit=0 | frames=364
10:08:54 | Connection closed by 213.181.122.2 port 59334
```

So the last thing anybody knew was 364 frames written. Whether the machine got any further cannot be known, because the evidence went with it.

**What the address does afterwards.** Not a timeout, and not silence — the connection is **refused**:

```
ssh: connect to host 213.181.122.2 port 59334: Connection refused
```

The port is closed and the machine does not answer a ping at all. Something at that address is still alive enough to refuse, which is the provider's own hardware; the rented machine is simply not on it any more.

**This is how a destroyed machine can be told from a restarted one**, and the difference is worth having:

| | a restart | a destruction |
|---|---|---|
| what the connection says | `Connection reset by peer` | `closed by remote host`, then `Connection refused` |
| afterwards | answers again in under a minute | refuses for good |
| the disk | entirely intact | gone |
| finished frames | all survive | all gone |

The two look alike for the first few seconds and can only be told apart by waiting. Nothing in the moment distinguishes them.

**Everything on the disk went with it** — 364 finished frames, every test folder, all four Blender builds, roughly 12 GB of material. None of it was recoverable and no part of it was offered back.

---

**A note on this document from here on.** The machine described throughout was destroyed at the end of this test, deliberately, to answer the question above. Everything in this document is a record of what it did while it existed. It is an input snapshot and was never going to be maintained; that is now literally true. A different rented machine may differ in every particular, and anything built on this should be re-checked against whatever machine it actually runs on.

2026-08-06 — Opus 5 (Claude Code) — the machine destroyed mid-render, recorded from outside it

---

# Breaking the graphics driver — on a second rented machine, 2026-08-06

The machine above was destroyed. A second was rented — two RTX 4090s, driver 580.95.05, sixty-one cores, no display server — and once its Blender tests were finished it was used to break the driver on purpose. The Creator asked for this and said plainly that the machine was made to be broken.

## The driver cannot simply be unloaded

Taking the kernel modules out succeeds and achieves nothing: they are loaded again on demand, immediately, before anything notices.

```
modules still loaded: 0
...
nvidia is loaded again, used by 2
```

So a driver that appears to have been removed may not be. Anything checking whether a machine still has its driver has to ask something that uses it, not something that lists it.

## With the driver removed outright

Fifteen packages purged, `nvidia-smi` gone, `libcuda` gone. Then all four Blenders were asked to render on the card.

**Every one of them rendered anyway, and exited 0.** Blender's own device list came back holding nothing but the processor:

```
DEVICES: [('AMD EPYC 7B13 64-Core Processor', 'CPU')]
```

What each said about it:

```
4.5   exit 0   (nothing at all)
5.0   exit 0   WARNING CUEW initialization failed: Error opening the library
5.1   exit 0   WARNING CUEW initialization failed: Error opening the library
5.2   exit 0   WARNING CUEW initialization failed: Error opening the library
```

**Blender 4.5 gives no sign whatever.** The later three give one warning line and carry on.

## With the driver mismatched — which is what a machine does to itself

Reinstalling the driver produced the mismatch without being asked to: the kernel module already loaded stayed at 580.95.05 while the new library arrived as 580.105.08. That is precisely what an unattended upgrade leaves behind overnight, and it needs a restart to resolve.

```
kernel module loaded: 580.95.05
library installed:    /usr/lib/x86_64-linux-gnu/libcuda.so.580.105.08

nvidia-smi:
    Failed to initialize NVML: Driver/library version mismatch
    NVML library version: 580.105
```

The machine says it plainly. Blender does not:

```
4.5   exit 0   CUDA cuInit: Unknown CUDA error value          devices seen: ['CPU']
5.0   exit 0   ERROR CUDA cuInit: Unknown CUDA error value    devices seen: ['CPU']
5.1   exit 0   ERROR CUDA cuInit: Unknown CUDA error value    devices seen: ['CPU']
5.2   exit 0   ERROR CUDA cuInit: Unknown CUDA error value    devices seen: ['CPU']
```

One line, marked as an error in the three newer versions and unmarked in 4.5 — and then the render finishes on the processor and the program exits 0.

**A reinstall does not bring the card back.** The machine has to be restarted before the module and the library agree again.

## What all of this amounts to

In every broken state — driver removed, driver mismatched — **a render asked for on the graphics card completes on the processor and reports success.** There is nothing in the exit status to tell the two apart, and in Blender 4.5 with the driver gone there is nothing in the output either.

On a machine rented by the hour for its graphics card, that is the expensive failure: the job finishes, the pictures are correct, the bill is the same, and the thing that was paid for was never used. Telling it apart means reading the run's own words for `CUEW initialization failed` or `cuInit`, or asking Blender what devices it can see before starting — never the exit status.

**This is the third time the same shape has appeared in this research.** On a full disk Blender announces the save and leaves a broken picture. An add-on built for the wrong computer installs, switches on and says nothing. Now a missing graphics driver renders on the processor and exits 0. In each case the exit status says the job worked.

---

2026-08-06 — Opus 5 (Claude Code) — the graphics driver broken on purpose, on a second rented machine

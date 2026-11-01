# While a render runs

A render is alive. This says what can be read out of it while it runs, and what changes what there is to read.

> **This joins answers that live in several topics.** Every claim below names the comparison
> questions it rests on; open those for the exact words, the exact messages, and what each
> version does. Where this document and a comparison disagree, the comparison is right.

*All four versions*, *every version* and *both machines* mean Blender 4.5 to 5.2 below, and *the newer three* means 5.0 to 5.2; 5.3 is named wherever it is meant. Every 5.3 result is the 5.3.0 Beta built 2026-10-16 on the Creator's Mac and nothing else. *Reports 14, Frames 15*

## The short answer

- A render prints a live stream only when there is no window — and, from 5.0 on, 5.3 included, only when it was started from the command line. Started from a script instead, those versions say nothing until the frame is saved, where the oldest prints the same stream by either route. *Reports 9*
- **A render's own lines reach whatever is reading them as they are produced. A script's own printed lines do not arrive at all until the run ends.** Which part of Blender printed a line is what decides when it arrives, not whether a person is watching. *Reports 14*
- What the stream holds changed at one version boundary, and every kind of line moved together, so anything reading it must be written for both shapes. 5.3 keeps the newer shape, and drops one line from it. *Reports 3, Reports 4, Reports 11*
- On an untiled Cycles frame, read the largest sample count seen, never the latest — in the tested 5.3 runs the two are the same. It can show whether sampling reached its count, not how much work was done or whether a picture reached disk. A cancelled frame can print a full count and a finished line and leave no file. A tiled frame's count restarts within each tile. *Reports 11, Reports 7, Settings 16, Frames 15*
- Tiles are reported only above one tile's worth of area, and from there the sample count restarts inside each tile. *Reports 3, Reports 7*
- A plain run goes forward; a list of frames is worked through in the order of the list, not in numeric order. *Frames 10, Writing 14*
- In the output folder, a file's presence and its size prove nothing about whether a frame is finished, and Blender's own skip test is no better. The line announcing a frame written comes over a whole file wherever the write succeeded — but the same line is printed over a part file when the write ran out of room, so the line is not itself the proof. *Frames 4, Frames 6, Writing 11, Writing 14, File output nodes 15*
- No running figure covers a whole animation, and nothing printed is marked as the graphics card's memory. *Reports 3, Reports 6*
- Sending the log to a file takes the whole log off the ordinary output from 5.0 on, 5.3 included, so a program watching that stream sees nothing. *Reports 13*

## What decides whether there is anything to read at all

Two things decide it before any setting does: whether a window is open, and how the render was started.

- With a window open, no way of starting a render prints progress, in any version. *Reports 9*
- With no window, and the render started by naming the frame or the animation on the command line, every version prints a full live stream. *Reports 3, Reports 9*
- With no window, and the render started instead by calling Blender's render operator from a script, the oldest version prints the same stream and 5.0 to 5.3 print nothing at all until the frame is saved. Six other topics met this, and all but one of them hand it back to a single question. *Add-ons 12, Builds 6, Dependencies 17, Frames 3, Reports 9, Settings 15, What actually ran 5*
- A machine with no display server at all renders on every engine, and cannot open a window there at all, so for a rented machine the printing case is the ordinary case. That was settled on Linux in 4.5 to 5.2; 5.3 never ran on a machine without a screen. *Reports 9, Settings 6*

**This is the condition under everything else in this document, and under every printed-output finding anywhere in the research.** Four topics state it of their own printed results outright: they were all made from the command line, the route that prints most. *Add-ons 12, Builds 6, Frames 3, Reports 9, What actually ran 5*

Not everything on the streams came from the render. On Linux the shell a render is started from can add lines of its own. *Builds 6* The error stream is not reliably empty even on a run that succeeds: one of the four versions fills it on an ordinary render on a Linux machine that has a display running, and 5.3 never ran on such a machine. *Reports 4, Reports 8* On machines with no display running it stayed empty in every version, with named exceptions — and one topic ran a control first, so that an empty error stream is a silence somebody looked for rather than one nobody tested. *What actually ran 5, Reports 13, Reports 14*

With no window, the two tested ways of asking whether rendering is under way do not answer it: whether a render job is running and whether the interface is locked both read false during rendering. In 4.5 to 5.2 that includes a read from a handler while the render was still going. In 5.3 it was tested with the lock off only. With a window open, in 5.3, both do answer, and whether a job is running answered in the 5.2.2 run there too. *Add-ons 12* What a script gets instead is pushed at it: the statistics handler is handed a progress string once for each stage while the render runs, with the sample count in it while sampling. From 5.0 on, 5.3 included, that string carries no frame number, so which frame a string belongs to rests on the order the strings arrive in. With a window open, 5.2 and 5.3 hand over those progress strings; the earlier three hand over one closing line. Whether a windowed 5.2 render with this handler ends cleanly is not settled: some tested runs crashed and others did not. The 5.3 Mac runs ended cleanly. *Reports 3, Reports 9, Reports 11*

None of the tested ordinary render cases waited for a person, with a window or without. That is not a guarantee against every wait: whether a macOS system permission dialog can block a render is untested, and 5.3 ran with the Creator logged in throughout. Two things do hold a run: a script hooked into the render, for as long as that script runs, and a script reading standard input while something holds the other end of it open, which then waits for ever. *The machine 7* **A run that never ends is a different thing and must not be merged with either.** The example every version's own manual gives of a run that stops itself runs on for ever from 5.0 on — in the newer three on both of its machines, and in 5.3 on the Mac — and only 4.5 behaves as the manuals describe. *Reports 12*

**And a third case: a stream that stops because the machine killed the run.** On one rented Linux machine, the largest scene tried rendered and wrote its frame under one of that card's two rendering methods and was **killed by the system** under the other, in all four versions, part-way through building the structure it needs — no frame written, and no ending of Blender's own on the stream. The limit reached there was the host machine's own memory, not the card's. So a stream that stops mid-work is not only a wait for a person or a run with no end in it; it can be a run that is already gone. *What a render demands 7*

## When a printed line actually arrives

This is the question this document is built on, and one topic settled it by test in all four versions on the Creator's Mac and on a rented Linux machine, and in 5.3 on the Mac, where every result held. *Reports 14*

**A render's own lines arrive as they are produced, into a pipe or a plain file alike.** Over a run they came in dozens of separate arrivals spread across the whole render, in bursts at each frame boundary; a plain file grew in steps, and the steps are the frames. A real terminal makes no difference to any of it. *Reports 14*

**A script's own printed lines do not arrive at all until the run ends.** A script printing steadily through a render delivered nothing until the run finished, whereupon everything arrived at once; sent to a file, the file stayed empty and reached its full size in one step at the end. **The two behaviours were then shown inside one run**, so nothing could be blamed on two runs differing: the script's line waited while the render's own lines came through throughout. Behind a real terminal the same script ticks along as it prints. **So what decides the timing is which part of Blender did the printing, not whether a person is watching.** *Reports 14*

**A script can ask for its own lines to be sent at once and gets its wish.** Two of Blender's own switches change nothing on their own and do it when used together; one system wrapper put in front of the whole command changes nothing for a script's lines in any version. *Reports 14*

**Why the negative half can be believed.** A reader that only looks once a run has ended cannot tell a held-back line from a prompt one, so the instrument was proved able to show both patterns before anything was concluded from it. *Reports 14*

**Progress arrives as whole separate lines and nothing is ever rewritten in place**, so anything reading Blender's output can split it on line endings and will never meet a line that is later overwritten. *Reports 14*

**Nothing already printed is lost, even to an outright kill.** A log read after a killed run is complete up to its last line. *Reports 14*

**Sending the log to a file changes what a watcher can see.** From 5.0 on, 5.3 included, the whole render log goes into the file and fills in during the run; the streams are left with almost nothing, including none of the failure's words. In 4.5 the ordinary and missing-file runs left the log file empty and the streams unchanged. With fuller logging, 4.5 did put startup material in the file, but almost nothing from the render. A file for Blender's log and a file receiving its ordinary output are different things. *Reports 13, Reports 14*

## What the live stream holds, and where its numbers stop

- **The stream can be read as it arrives**, and it and the statistics handler above are the whole of what a watcher gets — but only for lines the render itself printed. *Reports 14, Add-ons 12, Reports 3*
- In the plain untiled sixteen-sample control, only three lines carry a sample count. There is no line per sample, so a watcher cannot expect a steady count. *Reports 8*
- No running figure covers a whole animation: no animation-wide sample count, no animation-wide percentage, no animation-wide time estimate. The frame range is announced once at the start from 5.0 on, 5.3 included, and by nothing at all in the oldest. *Reports 3*
- **Frames are worked through one at a time, and the order is the order they were asked for, which is not always ascending.** A plain run and a stepped run go forward; a list of separate frames is rendered in the order written, not sorted, and a repeat is rendered twice. *Frames 10, Writing 14* So a reader that knows what it asked for can count frames itself; Blender will not count them for it, and a reader that assumes ascending order will be wrong about a list.
- **The line announcing a frame is not a count of finished frames.** From 5.0 on, 5.3 included, it can announce a frame before skipping it. The stepped-run tests in 4.5 to 5.2 also announced steps that wrote no new frame. In 5.3 on the Mac, an interrupted list still announced its remaining items without writing them. *Frames 4, Frames 7, Frames 8, Frames 10* **Saved lines and the frame-written handler count different things.** In 5.3 on the Mac, each picture file gets its own saved line. The handler fires once per written animation frame, including a video frame, and never for a single still that only a File Output node writes. It cannot be counted as a line per output file. *Writing 14, Writing 16*
- The time estimate covers the frame in hand and starts over at each new frame. It sits on Cycles' lines only, and on a frame that runs its full count not on every one of them. On a frame that stops early it is no forecast: in 5.3 and in the 5.2.2 run on the Mac it read many times the frame's real length. *Reports 5*
- The engine decides how much there is to read: Cycles prints stage lines, sample lines, a memory figure and an estimate; EEVEE prints far fewer lines, with its own sample count and no estimate; Workbench prints no progress line at all. *Reports 3, Reports 8, Settings 16*
- **From 5.0 the stream names the engine in use; 4.5 names it nowhere.** So in 4.5 the engine a render actually used cannot be read out of what Blender prints at all. *The compositor 9, What actually ran 2*
- No percentage in the printed stream follows the samples; that percentage belongs to the window. Percentages do appear, inside the stage lines, and none of them was ever seen to move. *Reports 3*
- Nothing printed anywhere is marked as the graphics card's memory, 5.3 included. The one readout that does show it belongs to the window and is off until somebody turns it on; in 5.3 it was never seen. *Reports 6*
- The statistics a render can be asked to print name no device. One line comes close, and it names a backend building a structure rather than the part that rendered. *What a render demands 1* Those statistics land on the ordinary output and not on the error stream, whatever the switch's own help text says — measured on three separate pairs of machines. *Reports 4, Reports 13, Reports 14*

Where the versions part on any of this they part at one boundary, and **the whole stream moved together across it** — the shape of every line, the frame announcements, what the statistics handler is handed, the wording of a skipped frame, a refusal. Seven of the questions below record that boundary separately, on seven different kinds of line: *Frames 3, Frames 4, Images 7, Reports 3, Reports 4, Settings 15, What actually ran 5.* Anything reading the stream has to be written for both shapes, and no single kind of line is exempt; 5.3 keeps the newer shape. Among 4.5 to 5.2 one printed thing parts again one version later still: the statistics a render can be asked for, whose units and headings change at 5.2, and 5.3 prints them as 5.2 does. *Reports 4* **5.3 parts on three more**: it omits the closing reset line in the tested converged and time-limited frames, its list of logging categories differs from the 5.2.2 run's on the same Mac, and EEVEE's line about a missing texture changes its words and names the file. *Reports 11, Reports 4, Reports 13* An eighth kind of line sits at the same boundary as the seven: how a refused rendering method is printed — 4.5 writes its own error line where the newer three go through their own message system, with the same words after it — and on one Linux machine, asking for a maker's method with none of that maker's hardware there also drew a warning about that maker's library in the newer three and in 4.5 not at all. *What a render demands 7*

## Picture size changes what is printed, and almost nothing on record was big enough to show it

- At the tested sizes above one tile's area, Cycles splits a frame into tiles and prints a tile count, with far more lines a frame. **Whether area or a side decides the threshold is not settled.** On a fresh scene at the factory tile size the tested boundary is in the same place in every version, 5.3 included. The tile size matters, and a file saved by the oldest version reads back in the newer three with a different one. A converted file's actual rendering is untested, including in 5.3. *Reports 7, Writing 11, What a render demands 5*
- The same tested threshold shows on disk: above it Cycles opens one tile file in Blender's own temporary folder and writes into it through the frame; below it none appears. **Two topics reached the same boundary from two directions and agree.** *Reports 7, Writing 11*
- EEVEE and Workbench never do this. *Writing 11*
- **A tiled line has been kept, and it does not read like an untiled one.** Every line of the *untiled* shape came from a picture below the threshold; the tiled capture is in *Reports 7*, and it is the same two shapes with a tile count added inside the line, ahead of the sample count. **Once a picture is tiled the sample number restarts inside each tile** — it falls back each time a new tile begins, against a maximum that stays at the scene's own count and is never divided among the tiles — so it cannot be read as a count across the frame. The tile number counts tiles finished, starts at nothing and never reaches its total. In 4.5 to 5.2 a tiled frame can end on a reset even when it finished; in 5.3 it ends on the last tile's full count instead. What cannot be read off a tiled line is which tile it belongs to: the whole tiled stream arrives inside a few thousandths of a second. *Reports 3, Reports 7*
- A size the settings accept is not a size a render can finish, and nothing says so. An enormous request is cut in silence to the largest the setting allows and then starts, printing ordinary lines, with no warning that it cannot realistically finish. *Settings 15*
- Nothing tells a caller that the size it asked for was corrected, and the exit status never will either. *Settings 15*
- At the very top of the size range, all four versions render every tile and then fail while putting them together, writing nothing. So a stream can reach its last tile and still leave no picture. **One rented Linux machine only; 5.3 was not run at that size.** *Reports 7*

## Reading a sample count is not reading progress

- A sample count is a ceiling, not a quantity. In Cycles five things stand between the number a run is given and the number a pixel gets; in EEVEE two features push the count above the number given, and in 5.3 a time limit can stop it short, with nothing printed to say so; Workbench has no such number at all. *Settings 16*
- A number stored in the file for one view layer can stand in place of a count handed in at launch, so the second figure on the line is not necessarily the number that was asked for. *Settings 16*
- **A count above what the setting accepts is cut to the limit in silence. The corrected figure can be read back from the setting and can appear in the progress line** — and only where nothing else is setting that number, because a sample subset and a view layer's own count each take it over. Neither was ever put on the same file as an out-of-range count, so what such a file prints has not been seen. Each count is corrected against its own limits, so the same number is cut for one engine and taken whole by another — there is no single ceiling for "the sample count". *Settings 18, Settings 16*
- In the tested untiled Cycles frames, convergence and a time limit leave a closing reset line in 4.5 to 5.2; 5.3 omits it. Earlier interrupt captures include that reset, but **whether every cancellation in every version prints it is not settled**. Nothing names why sampling ended early. The printed form of a sample subset is also not established. On a tiled frame a reset can follow completed work in 4.5 to 5.2, and in 5.3 the largest count is one tile's. No tiled frame whose sampling really stopped short was captured, so no printed test separates those outcomes. *Reports 11, Reports 7, Settings 16*
- **For an untiled Cycles frame, read the largest count seen against its maximum.** This avoids depending on a closing reset and shows whether sampling reached its count, never whether the frame reached disk. In the tested 5.3 runs it is also the latest count. *Reports 11, Settings 16*
- **In the tested sampling-stage interrupts, nothing is printed during the wait**, then a short burst arrives at the end and the run goes. That is the fuller account of the same moment, and **it bounds the rule above**: on the Creator's Mac, in all four versions and in 5.3, that burst carried a full sample count and a finished line for a frame that left no file. The same machine gives either ending. Where 5.3 cut the frame short, the burst was one sample line and then the cancelled line. So the stopped-short mark is about a frame whose sampling stopped short, and neither a full count nor a finished line means a picture was written. *Frames 15, Reports 11*
- The switches change what is there to read. On the ordinary completed render, the quiet switch silences the render in the oldest version and removes three lines from 5.0 on, 5.3 included; and at least one automatic value Cycles works out for itself is printed only at a raised logging level. *Reports 4, Reports 8, Settings 16* The break sentence answering a stop is printed under the quiet switch in every version, 5.3 included. *Frames 15*

**Quiet mode on a missing-texture run has a further bound.** In 5.3 it kept the progress as well as the missing-file complaint. Whether the earlier versions keep that progress is not settled, and no same-Mac control establishes a version change. *Reports 13*

## What the output folder shows while it is being written into

- With an empty output path and the reserving switch off, nothing stands for a frame while it renders. While it is being saved, its file is on disk part-written: it appears at or near nothing and grows in place at its final name, with no temporary name and no rename. The bigger the picture, the longer that window stays open. *Writing 11, Writing 14*
- So a file's presence proves nothing about a frame, and its size proves nothing either. *Frames 6, Writing 11*
- **Where the write succeeded, the file is whole in the file system at the moment it is announced**: a handler reading the whole file at that moment got exactly the byte count of the finished file, on two machines in all four versions and in 5.3 on the Mac, and nothing later touches it. So a watcher can take frame N as soon as it is announced and need not wait for frame N+1. **The announcement is not itself the proof, though.** Every run behind it had room on disk; a write that runs out of room prints the same line over a part file, in every version on both machines and in 5.3 on the Mac, and one such run ended at a success status. *Writing 14, File output nodes 15*
- The announcement does not prove the bytes have reached the disk itself. That is not settled by these tests. *Writing 16*
- **Every write belonging to one frame is finished before any write of the next begins.** A node's file goes first and the scene's own second; a preview comes after the frame's own file; two views are written one then the other. Cycles' tile file is one per frame. Its removal strictly precedes the frame's file on Linux in 4.5 to 5.2 and on the Mac in 5.3; the earlier Mac watcher saw removal no later than the frame's file, without settling a strict order. *Writing 14*
- Where Blender is set to put an empty file down for each frame in advance, that file sits at the finished frame's own path before the frame is worked on, one at a time as the render reaches each frame, and only ever one is waiting. *Frames 6, Writing 11, Writing 14*
- **Blender's own test is no better than a watcher's.** Skipping a frame already on disk is decided by bare existence at the path, checked as the render reaches that frame. Nothing looks at size, content or age, and an empty file counts as a frame already done. *Dependencies 17, Frames 4*
- A skipped frame is named as it is passed over, in every version and in 5.3, with no window. *Dependencies 17, Frames 4*
- A video output puts one file down almost at once, carrying the whole frame range in its name before the render can know it will finish. Whether it then grows while the render runs is recorded both ways, and that disagreement is below. And nothing announces a frame in it as safe to take; in 5.3 a video gets a line per frame appended and no saved line at all. *Frames 5, Frames 12, Writing 11, Writing 14, Writing 15*
- A stop during compositing need not print the sampling-stage ending. In 5.3 on the Mac, an interrupt during slow compositing ended the run without a cancellation line, a saved line or a frame file. Earlier versions were not stopped this way. *Frames 15*
- A large Cycles frame also holds a tile file open in Blender's temporary folder for the length of that frame, and it is gone before the process ends. *Writing 11, Writing 14*
- **In Blender 4.5, a File Output node can write files while nobody has started a render at all** — walking the playhead across a scene strip in a sequencer preview at the rendered shading filled the node's folder, on one rented Linux machine with a window. **5.3 on the Mac wrote nothing** with its preview drawing the brought-in scene. **Whether 5.0 to 5.2 do is not established**: they wrote nothing on that machine, but their runs never set which scene the sequencer shows, so their preview may not have shown the strip at all. *File output nodes 16*

## What this does not cover

- How a render ends, what the exit status is worth, and what a stop leaves behind. That is the joined answer next door.
- What actually did the work — the card, the add-ons. The ordinary stream names no device, and neither do the statistics a render can be asked for; from 5.0 the stream does name the engine, and the joined answer on what a render really did owns the rest. *Reports 6, What a render demands 1, The compositor 9*
- **Windows, with one exception.** One topic answered part of a stop on Windows — the break sentence and the length of the wait, in Blender 4.5 alone, on **Windows Server 2025 Standard**, on a machine with no graphics card. *Frames 15* Everything else here is untried there. The topic that owns when a line arrives has six Windows runs written out and waiting for a machine. *Reports 14* A Windows machine can be rented, so this is untried rather than out of reach — and **every Windows machine this project has read reported itself a Server edition**, so a Windows result has to name the edition, and no desktop edition has ever been reached.
- **How long anything takes, mostly.** Most of the machines behind these answers were shared with other work, and the comparisons say so, so no timing here is a clean measurement. Two are worth more than nothing all the same: terminal timings for a stop, with the machines' contention named; earlier window-stop timings conflict — all of them taken at one picture too small to be split into tiles, so what the wait is made of once a frame is tiled has never been asked *(Frames 15, Reports 7)* — and the arrival patterns behind the section above, whose own topic says outright that a busy machine shows the thing it turns on — lines during a run against lines at its end — as plainly as an idle one *(Reports 14)*.
- **What a person sees on a screen while a render runs.** Blender's own window has been opened, driven and photographed on rented Linux machines: a stop by key press and by the status bar's cancel button *(Frames 15)*, and four sequencer tests *(File output nodes 16)*; in 5.3 it was driven on the Mac by Blender's own simulated keys and clicks, which are not a person's. What a window does to the printed stream is still that it takes the progress out of it: a windowed render prints a line per frame saved, and in 4.5 a time line with it, and nothing else. *Reports 9, Frames 15*
- **Blender 5.3 anywhere but the Creator's Mac.** No Linux machine, no NVIDIA card and no machine without a screen ran it, and no build but the 5.3.0 Beta. Every 5.3 claim here is that build on that Mac, and set beside a 4.5–5.2 result taken on Linux it is not a difference between versions unless the comparison says it is.
- Reading Blender's own reports back as data rather than as printed text. *Reports 13* adds a fourth control for that: with a window open and a missing image texture, 5.2's and 5.3's own reports lists held nothing.

## What the evidence could not have shown otherwise

- Every size tried crossed one tile's area as well as one tile's side — even the run that grew a single row down one side crossed the area too — so whether it is the picture's area or its width that decides could not have shown; 5.3 too was tried only at square sizes. The one picture ever rendered wide but small in area was a Workbench render, which prints no tile line and writes no tile file. *Reports 7, Writing 11, The compositor 6*
- The tiled stream on record is compressed into a few thousandths of a second, so which tile any single line of it belongs to could not have been read out of it. *Reports 7*
- "Nothing else is printed when a frame stops short" rests on runs in which one of the three documented reasons a frame can stop short was never used. *Reports 11*
- Every run whose whole shape was captured wrote still images, so the shape of a run rests on stills alone. *Reports 8*
- Whether a percentage inside a stage line ever climbs could not have shown: every one anywhere on record reads the same, and nothing catches one moving. *Reports 3*
- Four topics say outright that what they record of printed output was all made from the command line, which is the route that prints most. *Add-ons 12, Builds 6, Frames 3, What actually ran 5*
- **The folder was watched at intervals; a finer instrument was used on Linux alone**: every file call Blender made was traced there, thread by thread, which is what settles that no name is ever temporary and nothing is renamed. The trace has no equivalent on the Creator's Mac that does not need a system setting changed, so **it settles Linux and says nothing about macOS**. *Writing 11, Writing 14*
- The size a part-written file was first caught at is a property of how often the folder was looked at, not of Blender. *Writing 11*
- Which versions a watcher catches a file part-written in is luck: the two machines caught different sets. *Frames 6*
- The enormous render was searched for warnings after being killed at four minutes with hours of work left, and in 5.3 sooner, so a warning arriving later could not have appeared. *Settings 15*
- The automatic value printed at a raised logging level was captured in 5.2 only; whether the other four print it is untested. *Settings 16*
- "With a window, no route prints progress" rests on windowed runs all begun by one operator call, never by a key press. *Reports 9* Key-started stops were tested on a rented Linux machine for a different question. *Frames 15*
- Whether the newer versions go quiet about skipped frames because of the window or because of the start route could not be told apart, because nothing records how the window run was started. 5.3's window run was started by a script's render command, which is both at once, and named no skipped frame — which fits either reading. *Dependencies 17*
- That no frame changed in a render disturbed by installs rests on the pictures compared one against another rather than on the files. *Add-ons 12*
- The screenless answer rests on Linux machines with no display server rendering tiny pictures; a Mac with none, and a machine with no graphics hardware at all, were never run, and 5.3 never ran without a screen. *Settings 6*
- No line naming the device that rendered was found in statistics read on two machines only, and in 5.3 on the Mac. *What a render demands 1*
- **The run the machine killed is one rented Linux machine**, which answers one maker's two rendering methods and no other maker's, and it was destroyed. What it reached was that machine's own memory ceiling and never the card's, so nothing there says what a stream does when a card's memory fills. *What a render demands 7*
- **The engine named in the stream from 5.0 on, and 4.5 naming it nowhere, rests on the Creator's Mac alone** in the topic that measured it against a picture; nothing was rented for that question. *The compositor 9*
- **Reports 14's switch tests are the Creator's Mac alone**, the rented machine having been destroyed before they were made, and the two switches together were not tried in 5.0 or 5.1. Nothing there was run with Blender's own window open. *Reports 14*
- **The sequencer preview result is one rented Linux machine**, whose display is drawn by its processor, and its terminal counterpart — the frame set by hand with no window and no render — wrote nothing in any version on either machine, nor in 5.3. 5.3's preview run is the Mac's own screen and one version, so it does not stand beside the 4.5 result as a difference. *File output nodes 16*
- **The Creator's Mac is not one unchanging machine.** Its operating system and its Blender 5.2 build differ between test days; results from different days need not be results from the same setup. *Reports 12, Frames 15*
- **Every 5.3 result is one beta build on one Mac with a screen**, its window driven by Blender's own simulated keys and clicks rather than by a person. *Frames 15*

## Where two topics do not agree

**What a video render prints while it encodes.** *Frames 5* records what the three newer versions print in place of the oldest version's line as **not settled**: runs of the same test disagreed, all the records stand, and none was chosen. *Images 7* states plainly what those three versions print during an encode. **Whether this holds is not settled, and the readings disagree.** Until it is, nothing should match on a line printed during a video encode. Both topics do agree that the oldest of the four prints something different from the other three, and that 5.3, on the Mac, prints the line *Images 7* gives the three. *Frames 5, Images 7*

**Whether a video file grows while the render runs.** *Writing 11* watched one appear at nothing and grow in steps through the whole run, and *Writing 14* measured about half its bytes on disk by the half-way frame and about nine tenths by the last, on two machines in all four versions. *Frames 5* and *Frames 12* both record short runs whose file appeared at once at a fixed small size and did not grow at all, in every version on both of their machines, reaching its full size only as the process let go of it. **Both are runs that were really watched, and neither can be read into the other at the sizes each used.** That growth happens somewhere is not in doubt: a long enough run closes its first file part-way through and opens a second. What nobody has measured is where the line falls — no frame count, no picture size and no container has been shown to be what decides it. In 5.3, on the Mac, both kinds were seen on one machine, the smallest run there not growing until the render completed, but picture size and codec were not varied apart, so that does not find the line either. *Writing 15* **So a video file's size must not be read as a sign that a render is getting on, and a video file that is not moving must not be read as a render that is stuck.** *Frames 5, Frames 12, Writing 11, Writing 14, Writing 15*

## Every question this rests on

- *Add-ons 12* — a running render cannot be seen as running from a script with no window; in 5.3 it can with one
- *Builds 6* — the shell can add lines that did not come from the render
- *Dependencies 17* — skipped frames are named; why the window run went quiet is open
- *File output nodes 15* — a failed write is announced in the same words as a good one
- *File output nodes 16* — in 4.5 a node writes while a preview plays, with no render started; 5.3 does not, and 5.0 to 5.2 are open
- *Frames 3* — every printed-output result rests on a command-line start
- *Frames 4* — the skip test is bare existence, checked frame by frame
- *Frames 5* — what a video render leaves, and what is printed while it encodes
- *Frames 6* — a file's presence and its size prove nothing
- *Frames 7* — a list of frames is not one plain run, and a repeat is rendered twice
- *Frames 8* — what a frame range is made of, and what a step announces
- *Frames 10* — the order frames are worked through, plain, stepped and listed
- *Frames 12* — the video file's size through a run, and what a stop leaves in it
- *Frames 15* — nothing is printed between a stop and the end, and then a burst
- *Images 7* — what a video encode prints, version by version
- *Reports 3* — the shape of the live stream and what it holds
- *Reports 4* — the streams, the quiet switch, the logging switches
- *Reports 5* — the time estimate covers the frame in hand only
- *Reports 6* — nothing printed is marked as card memory
- *Reports 7* — what changes at the tile threshold
- *Reports 8* — the whole shape of a run, and the engines apart
- *Reports 9* — how a render was started decides what is printed
- *Reports 11* — the mark of a frame that stopped short, and 5.3 omitting the closing reset line
- *Reports 12* — what an interrupt prints and hands back; the runs that never end
- *Reports 13* — what a missing file puts on each stream, and what the log file takes off them
- *Reports 14* — when a printed line reaches whatever is reading it
- *Settings 6* — every engine renders with no display server
- *Settings 15* — a corrected value is never announced; one route is silent
- *Settings 16* — the sample count is a ceiling, and what moves it
- *Settings 18* — a corrected count appears on the progress line and nowhere else on record
- *The compositor 6* — the one wide, small-area picture ever rendered, and it was Workbench
- *The compositor 9* — the stream names the engine from 5.0 and never in 4.5
- *The machine 7* — nothing on the render path waits for a person
- *What a render demands 1* — the printed statistics name no device
- *What a render demands 5* — a file carried across versions reads back with a different tile size
- *What a render demands 7* — how a refused rendering method is printed, at the same version boundary as the rest of the stream; a run the machine killed part-way through building its structure
- *What actually ran 2* — the stream names the engine from 5.0, and 4.5 never ends on a bad engine name
- *What actually ran 5* — the streams, the handlers, the log switches
- *Writing 11* — what is in the output folder while the render works
- *Writing 14* — one frame's writes stand clear of the next frame's; nothing is renamed
- *Writing 15* — what a video file holds through a run, in 5.3
- *Writing 16* — when the frame-written handler fires, in 5.3

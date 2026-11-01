# When a render ends

What can be told about whether a render worked, and what is on disk, once it has finished, been stopped, or had the machine go down under it.

> **This joins answers that live in several topics.** Every claim below names the comparison
> questions it rests on; open those for the exact words, the exact messages, and what each
> version does. Where this document and a comparison disagree, the comparison is right.

*All four versions*, *every version* and *both machines* mean Blender 4.5 to 5.2 below, and *the newer three* means 5.0 to 5.2; 5.3 is named wherever it is meant. Every 5.3 result is the 5.3.0 Beta built 2026-10-16 on the Creator's Mac and nothing else. *Reports 12, Frames 15*

## The short answer

- Exit 1 means something went wrong. Exit 0 means only that Blender did not think anything went wrong. *Reports 12, Writing 5, File output nodes 15, Reports 1*
- Twenty-one separate endings report success, and in most of them the render did not do what was asked. Six of the listed cases write no frame; in 5.3 a seventh does too: the tested Cycles scene with its Camera object deleted. *Reports 1, Reports 12, Reports 7, Settings 17, Settings 18, Settings 19*
- Not every run ends. The one command every version's own manual gives as the example of a run that stops itself fails to end within the test wait from 5.0 on, 5.3 included; 4.5 stops. *Reports 12*
- A saved line does prove a frame's file whole where the write did not fail; where it did, nothing available at the ending proves anything. *Writing 14, File output nodes 15, Frames 6, Frames 13*
- What a stop leaves is decided by two things together: which of the three ways it was sent, and whether it landed while a frame's own file was being written. Neither alone decides it, and at a working picture size that write is a real slice of every frame rather than a moment nothing lands in. *Frames 3, Frames 13, Frames 15, File output nodes 13, Writing 11*
- An interrupt is not instant. In the tested untiled Cycles runs its delay varied with the work in hand. No general upper bound is established, including for tiled frames or a long video's closing step. *Frames 15, Reports 7*
- A machine going down is worse than any stop: no frame's contents survived, including one reported saved seconds earlier. 5.3 was never put through it. *Frames 13*
- No ending number names its cause. The same number arrives from unrelated faults, and one version can end with no number at all. *Reports 12, Reports 10, Builds 9, What actually ran 2*
- A terminate and a kill each come back as two different numbers, and that is one ending read by two readers rather than two behaviours. *Reports 12, Frames 13*
- A memory shortage has three endings, and which one arrives turns on whose memory ran out and when — not on how short the machine was. None of the three was reached in 5.3. *Reports 1, Reports 6, Reports 10, What a render demands 7*
- Whether a stopped render leaves a nought-byte name for the frame it was on is not settled in 4.5 to 5.2, and the two answers make an automatic re-run either safe or silently lossy. In 5.3, on the Mac, the way the stop was sent decides it: a terminate or a kill leaves one, an interrupt does not. *Frames 10, Writing 11, Frames 14, Frames 16, The machine 7*

## What the exit status is worth

**Reports 12 owns this.** It asked what number every way of ending hands back, and settled the whole of it by test in all four versions on two machines — the Creator's Mac and a rented Linux machine with one graphics card and a screen — and in 5.3 on the Mac alone. Nothing in it comes from Blender's own documents, which say nothing about a status for a normal finish, an interruption or a refusal in any version, 5.3 included. *Reports 12*

**A run that finishes normally gives 0. One interrupt gives 1.** A blend-file that cannot be used, an argument Blender has never heard of, and a render device that is nonsense or merely absent all give 1 with nothing written, 5.3 included. The tested Cycles scene with its Camera object deleted gives 1 in 4.5 to 5.2 and **0 in 5.3** — entry 21 below. *Reports 1, Reports 12* What an interrupt leaves on disk is a separate question and is further down. A rendering method **named on the command line** that the machine has no hardware for does the same, and nothing falls back to anything else. *What a render demands 7, The machine 4* **The same lack of hardware reached through a saved preference instead is silent and ends at 0** — entry 5 below. Which of the two happens turns on the road, not on the hardware. *The machine 4*

Exit 1 is still trustworthy in the sense that something really did go wrong. **But "exit 1 means nothing was written" is narrower than it reads.** One topic records a stray argument taken as a file name, where the frame was rendered and saved first and the run still ended at 1. *Settings 9* So a run that ended at 1 may still have written files.

**Exit 0 carries almost nothing.** No version's sources document an exit status for a failed render at all. *What actually ran 4, Reports 12*

**A script can fail without stopping the render.** In all four versions and in 5.3, a script handed in at launch failed, the frame was still written, and the run ended at 0. A switch can make that failure stop the launch with a chosen status, but only when placed before the script. This also bounds any check made before rendering: the check must be able to stop the launch. *Reports 12*

**Twenty-one endings report success.** Each one is a whole fact in its own topic; the list only exists because they sit in nine different ones.

1. A write that starts and runs out of room, with no later frame to fail: a part file is left and announced as saved. *Writing 5, File output nodes 15*
2. A compositor File Output node's write failing for every cause tried — a refused folder, no room, a broken template: the render carries on to the end. *Writing 12, File output nodes 15*
3. Something the file points at is missing — an image, a font, a linked file, a volume, and in 5.3 an Alembic cache, an IES file, an OSL script and a sound as well: a frame is written anyway. *Dependencies 12, Reports 13*
4. The add-on holding the file's render engine is absent: a different engine renders the file at its own sample count. *Add-ons 7*
5. The graphics card is not used at all — no technology chosen, the driver broken, the cards hidden, or a technology saved in the preferences that no card on the machine can serve — and the ending does not separate it from a render that used the card: the status is 0 and the picture is written. Only the run's own printed words can: the line naming the device, which comes out only when a switch asks for it, and, with the driver removed, one warning the newer three print and 4.5 does not. *The machine 2, The machine 4, Settings 9, What actually ran 4*
6. The card's memory filled by the render's own scene: silent, the picture written, the run taking much longer. Never reached in 5.3. *Reports 1*
7. A resolution percentage outside the range: corrected in silence, and the picture comes out at the corrected size. *Settings 15*
8. A picture size the video encoder refuses: no file at all is written. *Images 4*
9. A format that cannot hold the fields a render writes into a picture: the fields are quietly gone. *What actually ran 4*
10. An output format name Blender will not take: it complains once, then writes the frame in the format the file already held. *Images 4, What actually ran 4*
11. From 5.2, with the saving of render output switched off: the scene's own frames are not written, and where there was no File Output node the output folder is empty. In 5.2 **a File Output node in the same tree goes on writing**, so this is silence about the Output panel's own file and not about the run. In 5.3, with nothing else arranged, the run renders nothing at all, still or animation, and makes no output folder; whether a File Output node goes on writing there was not tried. *Reports 12, Reports 1, What actually ran 4*
12. A frame that stopped short of the count it was asked for: written and reported like any other. The one printed sign it leaves is described below, and it holds only for a picture small enough not to be split into tiles. *Reports 11, Settings 16, Reports 7*
13. A frame number handed in that is not a number: nothing rendered, output folder empty. *Reports 12*
14. An option handed in with its value missing: nothing rendered, output folder empty. *Reports 12*
15. An animation whose start frame is after its end frame: nothing rendered, output folder empty. *Reports 12*
16. A picture asked for at no pixels: nothing written. *Reports 7*
17. A sample count outside what the setting accepts: corrected to the nearest limit in silence and rendered at the corrected number. The run carried through to its end this way ended at 0; the run at the top of the range was left going rather than finished, so its own ending is not on record. *Settings 18*
18. A file naming a denoiser the machine has not got: the render goes ahead, the picture is still denoised by whatever the machine has, and the setting reads back as empty text. *Settings 19*
19. In 4.5 alone, a misspelled setting name handed in at launch on Cycles' settings: accepted with no complaint on either stream, and the value gone the instant it is read back. *Settings 17*
20. A video container paired with a codec it will not take: the encode is refused and a stub of nothing to a few hundred bytes is left at the video's own name. That is not the same ending as entry 8 — there a file is not written at all, here one is, and it holds no frames. *Images 4, Reports 1, Reports 12*
21. In 5.3, the tested Cycles scene with its Camera object deleted, rendered on the processor from the command line: nothing written and no render handler fired, where the same setup gives 1 in 4.5 to 5.2 and in the 5.2.2 run. The failure is printed in the words Blender's window uses for it; a script's own render call on that file with no window still ends at 1. A different 5.3 setup — Workbench with the camera assignment cleared but its Camera object still present — wrote its picture at 0. These tests establish their own setups, not a rule for every scene without an assigned camera. *Reports 1, Reports 12, Settings 6*

**Six of those write no frame at all** — entries 8, 11, 13, 14, 15 and 16 — and in 5.3 entry 21 makes seven, which is a sharper fact than "the render did not do what was asked". Four of them were each run into a folder emptied immediately beforehand, so *nothing was written* there is a fact about that folder rather than about a stale file; two of those four do say something while the status stays 0, and they say it on the error stream. *Reports 12* A run given no render argument at all also ends at 0 having rendered nothing, and that is what it is for. *Reports 12* Two further cases end the same way in 5.3, without an earlier-version result: every view layer switched off, and a frame argument left empty. *Settings 1, Reports 12*

**Four of the list carry bounds that must travel with them.** Entry 19 is the Creator's Mac and nothing else: nothing was rented for that question, no Linux or Windows machine tried any of it, and Smoozy renders on rented Linux. *Settings 17* Entry 17 has no single ceiling behind it — each count is corrected against its own limits, so the same number is cut for one engine and taken whole by another, and neither says a word. *Settings 18* Entry 18 was settled in 4.5 to 5.2 by carrying a file made on a machine with an NVIDIA card to the Mac, which has no such denoiser, and in 5.3 by writing the same stored value into a file on the Mac directly. *Settings 19* Entry 21 is one beta build on one Mac, set against the 5.2.2 run there, which ended at 1. *Reports 1, Reports 12*

**Two opposite endings from one condition.** A card full of the render's own scene is silent at 0; the same card filled by a neighbouring program stops the render loudly at 1. Which happens turns on who took the memory, not on how much is left. *Reports 1, Reports 2*

### The three endings a memory shortage has

Three topics each hold one of them and no topic holds them together, so a reader who opens one comes away with a rule that is true of that one case and false of the other two.

1. **The card fills with the render's own scene: nothing is said, the picture is written, the run ends at 0, and it takes much longer.** The spill onto the machine's own memory that every version's manual describes does happen, in all four versions, and both output streams were searched for four fixed phrasings and matched nothing. *Reports 1, Reports 6*
2. **The machine's own memory runs out while Cycles is sampling on the processor, and here the versions part.** Blender 4.5 and 5.0 are killed by the operating system, standard error empty, status 137. Blender 5.1 and 5.2 refuse the allocation, put failure lines on standard error and abort at 134. Neither writes a picture, not even part of one, and neither leaves a crash file. No card render was run out of system memory while sampling. *Reports 10*
3. **The machine's own memory runs out while the scene is still being built, and all four are killed by the machine with nothing said.** No frame is written, and 5.1 and 5.2 are among them — the two that abort loudly in the case above. *What a render demands 7, Reports 10*

**Whose memory ran out, and at which moment, separates these tested endings.** The runs do not establish a rule for every shortage. *Reports 1, Reports 6, Reports 10, What a render demands 7*

**The first ending is the one that rests hardest on a condition, and the condition is how much of the machine's own memory stands behind the card.** The only machine that ever filled a card had **16 GB cards against 197 GB of system memory**, and running that system memory out as well was never attempted. An outright memory failure was never reached from the scene filling its own card, and **nothing establishes this ending behind less system memory**. *Reports 1, Reports 6, What a render demands 7*

**The version split belongs to the second ending alone and must not be carried across.** Anything watching standard error for 5.1 or 5.2 sees failure lines where the shortage arrives during processor sampling and **nothing at all** where it arrives while the scene is being built. Anything watching standard output learns nothing in either of those two failures: its last line is ordinary progress. *Reports 10, What a render demands 7*

**The bounds each of the three carries, and none of them may be dropped.**

- The first is one rented machine — four NVIDIA cards of 16 GB against 197 GB, no display server — with a scene grown beyond a card's memory. The four-card form of it was run in 4.5 alone; the one-card form in all four. The silence rests on searching both streams for four fixed phrases, so a line worded otherwise would have been missed. *Reports 1, Reports 6*
- The second is one rented Linux machine, run twice through in all four versions. **macOS and Windows are untouched by it.** Its readings of the machine's own memory use are ordinary readings on a rented container and are not that machine's own figures; the version split does not rest on them. *Reports 10*
- The third is one rented Linux machine with one NVIDIA card, at the largest scene tried, under one of that maker's two rendering methods — under the other method every run finished and wrote its frame. **What was reached there was the host machine's ceiling and not the card's.** *What a render demands 7*
- All three shortages were reached on Linux only. **Neither macOS nor Windows was run out of memory for them.** *Reports 1, Reports 10, What a render demands 7*
- **None of the three was reached in 5.3.** It ran on the Creator's Mac alone, whose memory is not filled on purpose and whose graphics share one pool with it, so a card cannot fill there while the machine's memory holds. *Reports 1, Reports 10*

A fourth ending sits beside these and is a different act: the card filled by a **neighbouring program** rather than by the render, which fails loudly at 1 — the paragraph above, and 4.5 and 5.2 only. *Reports 1, Reports 2*

**Some endings never arrive, and this is not one version's fault.** The example every version's own manual gives of a run that will stop itself — a blend-file after a mistyped argument — stops itself in 4.5 alone. From 5.0 on the run does not end at all — on both machines in the newer three, and in 5.3 on the Mac — still going long after the point of the test, on repeated runs. *Reports 12* An engine name no engine bears ends at 1 at once from 5.0 on, 5.3 included; in 4.5 it hands back nothing a caller can use, and **which way it fails is the operating system rather than the run** — on the Mac it aborts, and on every rented Linux machine that has been asked, across four topics, it does not end at all. Why is not known. *Reports 12, What actually ran 2, Add-ons 7, The compositor 9* **So anything waiting on a run to finish needs a limit of its own, in every version, 5.3 included, and not only in the oldest.** None of the ordinary render cases tested waited for a person. Whether a macOS system permission dialog can block a render is untested. The 5.3 Mac had a logged-in session throughout. *The machine 7* That is a different thing from a run that never ends, and the two must not be merged: a non-ending run is not waiting to be answered.

**A number does not name a cause.** An uncatchable kill ends at 137, read from a shell on both machines in all four versions; a terminate's two reported numbers are the two readers described below, not disputed Blender behaviour. *Reports 12* 137 also arrives from the machine killing a render that filled its memory, and from a Blender that was only half copied and never started. *Reports 10, Builds 9* A second interrupt ends at 2 in 5.1, 5.2 and 5.3, and at 134 in 4.5 — where 4.5 is killed by the same signal on both machines. *Reports 12, Frames 15* In the 5.3 signal test a hang-up also killed the run, which a shell would read as 129. Wider hang-up handling is unsettled, as below. *Reports 12, The machine 7* 134 also arrives from a refused allocation, from 4.5 on the Mac when the engine it is forced to use is missing, and from 4.5 on the Mac when it is merely asked to list its engines. *Reports 10, Add-ons 7, The compositor 9* 135 is a Blender replaced under a running render, on Linux only. *Builds 6* 139 is a crash. *Reports 1, The machine 4*

### One ending, two readers

**A polite terminate and an uncatchable kill each come back as two different numbers, and that is one ending read two ways rather than two behaviours.** One topic records **143** for a terminate and **137** for a kill, in all four versions on two machines, and names a shell as what reported them. Another records **−15** and **−9** for the same two signals, in all four versions on two machines. Each topic's own pair comes back again in 5.3 on the Mac. Both say outright that the number belongs to whatever reads the ended process rather than to Blender, and that neither ever ran the other's reader. **So 143 must never be set against −15, or 137 against −9, as a difference between a version, a machine or an operating system.** *Reports 12, Frames 13*

**The pairing is checkable in the four figures themselves: in each pair the larger number sits a fixed 128 above the other reading's number taken without its sign.** That constant is arithmetic on the numbers the comparisons hold and nothing more — **no comparison explains it**, and **no comparison records what took the negative pair.** The 143 and 137 side is attributed to a shell; the other side is attributed to nothing. *Reports 12, Frames 13, Settings 16*

**The interrupt is 1 on both readings**, so only the two signal endings are affected. *Reports 12, Frames 13*

**A caller must know which reader produced its ending number.** A program accepting only one set can misread an ordinary stop. Both sets can describe the same ending; the number alone does not choose between them. *Reports 12, Frames 13*

## What a saved line proves, and what nothing proves

**Where a write did not fail, the saved line does prove the file whole.** A handler that read the whole file at the moment Blender announced it written got exactly the byte count of the finished file, in all four versions on two machines and in 5.3 on the Mac; on Linux the traced file calls show the close following the last write, with nothing written to that file afterwards. *Writing 14* So a program watching the output folder can take a frame as soon as it is announced, and does not have to wait for the next frame to appear. **When that handler fires was watched in 5.3 alone, on the Mac**: once a frame rather than once a file, never for a single still that only a File Output node writes, and for a video saying nothing about the file. Every picture file still gets a saved line of its own. *Writing 16*

**Whole in the file system does not mean safe from a machine failure.** Nothing in the announcement tests shows that the bytes have reached the disk itself. The kernel-reset test lost even an earlier frame reported saved, in 4.5 to 5.2 on Linux. *Writing 16, Frames 13*

**Where a write did fail, nothing available at the ending proves anything.** A part-written file is announced as saved in exactly the words a whole one is. *File output nodes 15, Writing 5* That is the only signal there is: no property on a File Output node records a write, and no version has a failure hook for writing a render frame. *File output nodes 15* So the rule holds for a write that ran out of room, was refused, or was cut by a stop aimed at it, and for nothing else.

A frame is written straight to its final name and grows in place — no temporary name appears, and the Linux trace holds no rename call anywhere in it. So a file caught part-written is the file, and its presence proves nothing. *Frames 6, Frames 13, Writing 11, Writing 14* Nor does its size: a name can exist at zero bytes because the switch that reserves each frame put it there. *Frames 6, Writing 11* Nor does opening it: a stopped file of one format, and sometimes of another, loads and reports the full picture size. *Frames 13*

For an untiled Cycles frame, **read the largest sample count seen against its maximum**, not a closing reset. Convergence and a time limit left that reset in 4.5 to 5.2 and not in 5.3. Whether every cancellation prints a reset in every version is not settled. A sample subset's printed ending is also not established. On a tiled frame the sample count starts over for each tile, so it cannot settle whether the whole frame reached its count. The sample figure among a finished picture's named fields is the maximum asked for, not the count reached. *Reports 11, Settings 16, Reports 7, What actually ran 4*

**And a full count is not a file.** A stop can leave the frame's sampling complete and its count line reading full, followed by a line saying the frame finished, and still no file for that frame anywhere. So neither the full count nor the finished line means a picture was written, and the two questions — did the frame reach its count, did the frame reach disk — must be asked separately. *Frames 15, Reports 11*

**The closing report that names what did the work does reach a stopped render, in 5.0 and 5.1.** With the fullest logging, where a stop lands after the frame's own sampling has completed, those two print the whole block, the device line in it; 5.2 and 5.3 print a cancellation line and no block; the oldest prints no such report. That split was seen on the Creator's Mac alone. Whether a frame whose sampling was cut short produces a device report at that logging level is not established. Ordinary 5.3 output does show such frames ending without a finished line. The closing block's own figures have proved unreliable, so its device name is not established as right either. *What actually ran 6, Frames 15*

## How long a stop takes

**The tested interrupt delays vary with the work in hand, the sample count and the device.** The length of a plain animation range made no difference in those runs. Waiting for work already under way is an inference from that pattern; the tests do not show where Blender checks for cancellation. *Frames 15*

**No constant stop time or general upper bound is established.** The main Cycles runs were untiled. What the wait contains once a frame is tiled, and what a long video's closing step adds, are not settled. A deadline needs evidence for the machine and work it is to cover; one frame's measured length is not a proved limit. Stops between frames ended almost at once in the tested runs. *Frames 15, Reports 7*

**Blender answers a stop in words and then goes quiet.** Nothing at all is printed for the whole of the wait, and the handlers that do fire were all recorded arriving together at the very end — **though in 4.5 to 5.2 whether they really fire then, or only reach a reader then, is not established**, because a script's own printed lines do not reach a pipe or a file until the run ends and nothing records what instrument timed them. In 5.3, on the Mac, each handler wrote down its own clock reading, and every one read the end of the stretch. *Frames 15, Reports 14* **What is not in doubt is the part a builder needs**: the handlers for a frame rendered, a frame written and a render completed never fire for the frame in hand, in any version, and a program watching a stopped render learns nothing during the wait either way. *Frames 15*

**The other two ways of stopping do not wait at all.** A polite terminate and an uncatchable kill both end the run in a few hundredths of a second, with nothing added to what the render had printed. An interrupt sent as a signal takes the full wait, exactly as one typed at a terminal does, and the same holds for a render started detached with no terminal at all. *Frames 15*

**Inside Blender's own window, on Linux, Escape stops the render in all four versions, and so does the status bar's cancel button.** Nothing is printed either way. *Frames 15* **The same two are refused on the Creator's Mac** — driving a key press into another program's window there needs a system security setting nobody here may change, and the refusal was checked rather than assumed. *Frames 15* **In 5.3 the Mac's window was driven by Blender's own simulated keys and clicks instead**: the cancel button stopped the render and Escape did not — nor did it in the 5.2.2 run there — so whether a person's Escape stops a 5.3 render is not established. *Frames 15, Reports 12*

**The earlier in-window delays are not settled.** The window-stop timings conflict, and the terminal timings cannot be used in their place. Whether a person's input follows the driven window tests is also not established. *Frames 11, Frames 15*

## What is left at the output path after a stop

Every finished frame survives, whichever way the render ended. *Frames 3, Frames 15*

**Where the stop landed decides the rest.** During rendering, with no older file at that frame's path and the reserving switch off, all three signals left the finished frames whole and nothing for the frame in flight, in all four versions on both machines and in 5.3 on the Mac. Window stops that actually stopped the render left the same; a person's Escape in 5.3 is not established. With the reserving switch on, see below. *Frames 2, Frames 3, Frames 11, Frames 15, Frames 16, File output nodes 13*

Aimed at the instant the frame's file appears, the three part company: an interrupt finishes the write and the frame is whole, while a terminate and a kill each leave a short file at the frame's own final name. In 5.3, on the Mac, a stop landing in the middle of a long write did the same. *Frames 13* **This is the join that matters.** The topic that found nothing part-written after its own stops says outright that they landed at no particular moment against a write window about a thousandth of a second wide in its own small scene, and that its result does not reach the aimed case. *File output nodes 13* A reader who takes "nothing is ever left part-written" from either topic alone has the wrong rule.

**And that short file has a frame's real name, so a re-run passes over it.** Blender writes each frame straight to its final name — no temporary name ever appears — and the skip looks only at whether a name exists. *Frames 13, Frames 4, Frames 2* So a terminate or a kill landing on a write leaves an unopenable file that a later render counts as done. **How likely that is depends on the picture, and the odds are far worse than the thousandth of a second above suggests.** That window is one small scene's; measured across sizes it is about a fifth of a second at 1920 by 1080, about two seconds at 8192 by 4608, and about three and a half seconds at 10000 by 10000. *Writing 11* **At a working picture size the write is a real slice of every frame, not a moment nothing lands in.**

How many bytes such a file holds is a race, not a fact about a version or a machine. *Frames 13* **Over an older finished frame, with overwriting on, a kill in the middle of the write loses the older frame too.** The old file is emptied as the new write begins, and the kill leaves it short and unopenable. During rendering, before the write begins, an interrupt or a kill left the older file exactly as it was. These older-file tests were run in 5.3 alone, on the Mac. *Frames 13, Frames 16*

A node's file and the scene's own file for one frame are not written together: the node's goes first, and a terminate or kill during its write can leave it short with nothing at the scene's own path for that frame. *Frames 13, Writing 14* Every other file a frame writes falls inside that frame's own turn as well — a preview beside the frame, a second view, and Cycles' own tile file. That tile file is one per frame and is removed before the frame's own file appears on Linux in 4.5 to 5.2 and on the Mac in 5.3. On the Mac in 4.5 to 5.2 removal was seen no later than the first sight of the frame's file; a strict order is not established there. *Writing 14*

**Where the switch that reserves each frame is on, whether a stop leaves that empty file behind is not settled in 4.5 to 5.2** — see the disagreements below. **In 5.3, on the Mac, the way the stop was sent decides it**: an interrupt takes the empty file away, and a terminate or a kill leaves it at nought bytes. What is agreed is that only ever one is waiting at a time. *Writing 11, Writing 14, Frames 16* What a stop **aimed at the moment a frame's file appears** leaves is not in doubt: the empty file stays, at nought bytes, and a later render counts that name as a frame already done and never fills it. *Frames 14, Dependencies 17*

**A single video file behaves differently from a set of images, and how differently is not settled** — see the disagreements below. What is agreed is what a stop leaves in it. Interrupted, it holds the frames finished by then, and an ordinary media tool on a rented Linux machine read those frames back out of it in all four versions. *Frames 5, Frames 12, Writing 14* **A terminate or a kill leaves nothing in it where the file had not grown** — twenty-frame runs whose video sat at its opening size until the very end, in all four versions and in 5.3. *Frames 5, Frames 12, Frames 17* **Where the file had grown, nobody has looked properly in 4.5 to 5.2.** A terminated four-thousand-frame render left about 1.3 megabytes, and the only reader ever put on that file there is Blender's own, which refused it; the one part-finished video an ordinary media tool read in those four was left by an **interrupt**. *Writing 11, Writing 14* **In 5.3, on the Mac, the same terminated file was given to a media tool as well**: Blender's own reader again found nothing in it, and the media tool took most of the frames Blender had announced out of it. *Writing 15* Where a long render had already closed one file off and opened another, everything in the closed file survives whole. *Frames 5, Frames 12*

**A re-run after a stop replaces rather than adds.** Started again over the same range, the video was replaced and every image was written again, including the ones the first run had finished. *Frames 12*

**Separate images and a video need not stop on the same frame.** In 5.3 on the Mac, an interrupt during the last File Output image's write let that image finish but left its frame out of the video. Whether this holds in earlier versions is not established. *Frames 12, Frames 17*

**No control for stopping at the end of the frame in hand was found**, inside the window or outside it. *Frames 11, Frames 15*

**Frames arrive in the order they were asked for.** A plain run and a stepped run go forward; a list of separate frames is written in the order of the list and not in numeric order, so a reader deciding which frames are done cannot assume the highest name is the newest. *Frames 10, Writing 14*

In 5.3 on the Mac, interrupting a list left the earlier listed frames whole and nothing for the frame in hand, but Blender still announced the remaining list items without writing them. So an announcement after a stop is not evidence of another completed frame. Earlier versions were not stopped this way. *Frames 10*

**A machine going down is the worst case reached.** After a reset with nothing written out, no frame's contents survived in any of the four versions — including the first frame, which had been reported saved seconds earlier — and 5.3 was never put through it. *Frames 13* Picking up afterwards is a fresh render that skips the frames already on disk; no version has any resume inside a frame. *Frames 1, Frames 2*

### What a re-run after a stop costs, and why 4.5 to 5.2 cannot say

**The skip is decided on a name and nothing else.** A re-run started with the switch that passes over frames already on disk skips a frame because a name exists at its path, before that frame is rendered, and never looks inside the file — a nought-byte file, text that is not a picture, a picture of the wrong size and a folder were each skipped untouched. And a name left at nought bytes is skipped **for good**: a later render passes over it again and it stays unopenable. *Frames 6, Frames 14, Dependencies 17*

**Whether a stop leaves such a name for the frame it was working on is not settled in 4.5 to 5.2** — the disagreement is below, and neither side is chosen here. **What that costs is settled, and the two answers are not near each other.**

- **On one reading an automatic re-run is safe.** Nothing at all was left for the frame in flight, so the re-run reaches that frame, renders it, and the job comes out whole.
- **On the other it silently loses that frame.** The nought-byte name is already at the frame's path, the re-run passes over it, and the run ends at 0 reporting the full count of files with one of them empty and unopenable. That is the same ending *The machine 7* records reached by another road, where a frame was claimed, never rendered and left empty while the job still reported forty files at exit status 0.

**So in 4.5 to 5.2 the same act is either harmless or a silently lost picture reported as a complete job, and which it is has never been run.** Until it is, **nothing that restarts a stopped render may trust the file count**, and a re-run that must not lose a frame cannot decide anything from the names in the folder alone. **In 5.3, on the Mac, it has been run, and the answer turns on the signal.** With the reserving switch on, a terminate or a kill left the frame's empty file, and a render started again with overwriting off skipped that frame and left it empty and unopenable; an interrupt took the empty file away. So in 5.3 a terminate or a kill meets the costly reading. *Frames 16, Frames 2, Frames 14*

**And the safe half of that disagreement is only about the reserving file.** A terminate or a kill landing while a frame's own file is being written leaves an unopenable file at that frame's real name whatever the reserving switch was set to, and a re-run passes over it the same way. That case is not in doubt, and at a working picture size the write it has to land in is a real slice of every frame. *Frames 13, Writing 11, Frames 4* *Frames 3, Frames 6, Frames 10, Frames 15, Writing 11, Frames 14, The machine 7, Dependencies 17*

**The bounds on both sides of it.** Both sides claim all four versions and both claim the Creator's Mac; one adds a rented Linux machine and the other a second rented machine. **Both records do say what the overwrite setting was, and both say off**, so that is not what separates them; nor is the way the stop was sent, one side covering all three ways and the other a terminate. **What differs is the moment each stop was fired** — one at the instant the frame's reserving file appeared, the other at no particular moment part way through a run. Only one of the four version records on the side that found nothing names the setting at all, the other three pointing back at a stop test whose set-up does not mention it. The cost is established twice, in two topics by two routes, but the second of those is **one file in one run of one version on one machine** — the other three versions there left none, and so did the same version on the other machine, so there is no rate in it. **None of this has been tried on Windows.** 5.3 points at the signal as what separates the two records — the side that found the file left used a terminate, which in 5.3 leaves it — but the one record on the other side that names the setting has a terminate and a kill leaving nothing, which 5.3 does not do, so 5.3 does not settle the earlier four. *Frames 10, Writing 11, Frames 14, Frames 16, The machine 7*

## Whether a crash file is written

For two kinds of ending, and the second was found by two topics at once.

A Blender made to crash writes a crash file and says so, at 139: in all four versions on Linux, in the temporary folder under the loaded file's name; and in 5.3 on the Mac the same, in Blender's own temporary folder, as the 5.2.2 run there did too. The earlier Mac runs do not settle how they named it. Anything printed immediately before the crash never reaches the ordinary output. *Reports 1, The machine 4*

**A second interrupt in 5.0 on the Creator's Mac is the other one.** It ends on a crash reported as 139 and names a crash file under the system's temporary folder. On the rented Linux machine the same test gives 2. *Reports 12, Frames 15* Both were run several times, and **neither comparison chooses between the two machines**, so nothing about 5.0 rests on it. In 4.5 the same second stop ends at 134 on both machines, in 5.1 and 5.2 at 2 on both, and in 5.3 at 2 on the Mac. *Reports 12, Frames 15*

Running the machine's own memory out writes no crash file at all, at either of the two statuses that ending gives. *Reports 10* Whether one is written when a Blender is replaced under a running render is not settled — two runs of one test on one machine disagree and both stand. *Builds 6*

One version crashes on its own in one arrangement: 5.1, with both compositor devices set inside a single run, died after writing the first frame. How often was deliberately not established. *The compositor 1*

## Endings the render did not cause

Deleting the Blender out from under a running render is harmless on both operating systems: the render finishes, writes every frame, and ends at 0. *Builds 8* Copying a Blender over the folder it is running from is harmless on macOS and kills the render on Linux at 135, with no error of Blender's own and an ordinary progress line as the last thing in the log. *Builds 6* Installing a second Blender beside a running render disturbs nothing on either. *Builds 6* Of these, only the second Blender put beside a running render was tried in 5.3, on the Mac, and it disturbed nothing there either. *Builds 6, Builds 8* A render also survives the connection it was started from being cut, on Linux, and the shell it was started from being killed, on the Mac in 5.3 as well. **Whether a hang-up ends it is not settled.** Sent to the whole process group on one day, in 4.5 to 5.2, it left the render going; sent on a later day to a group Blender itself led, it ended both 5.3 and the 5.2.2 run on the same Mac, and nothing records how the earlier group was formed. *The machine 7, Reports 12*

**A scene large enough can end the run without Blender saying anything about it.** On one rented Linux machine, at the largest scene tried, every run under one of the two NVIDIA rendering methods was killed by the machine part-way through building its structure, in all four versions, with no frame written; every run under the other method finished. **What was reached was the machine's own memory ceiling and not the card's**, so this says nothing about what happens when a card fills. *What a render demands 7*

**Whatever the ending, nothing already printed is lost.** A render sent to a file and then stopped kept everything it had printed, and after an outright kill the file held exactly what it held before, ending on the most recent line. So a log read after a killed run is complete up to its last line. *Reports 14*

## What this does not cover

- What is printed while a render is still alive, and what can be read from it as it arrives.
- Which files a render was going to write, how many, and what each is called.
- Whether the render used the engine, the device or the count it was asked for, beyond what the ending shows.
- Whether the picture inside a written file is the picture that was wanted.
- Two renders sharing one output folder.
- What a person at a screen sees for every kind of write failure. The window tests in 5.3 on the Mac covered a broken node template and an output blocked by a folder: the bad template's field was marked, but nothing in the window named the blocked file. The terminal still carried failures. This does not settle other window failures or a person's own input. *File output nodes 15, Writing 12*
- A real power cut, as against a reset at the kernel. Nothing has ever been done to a machine here that stands in for the mains going off, and the reset is as close as anything got. *Frames 1, Frames 13*
- **Blender 5.3 anywhere but the Creator's Mac.** No Linux machine, no NVIDIA card and no machine without a screen ran it, and no build but the 5.3.0 Beta. Every 5.3 claim here is that build on that Mac, and set beside a 4.5–5.2 result taken on Linux it is not a difference between versions unless the comparison says it is.
- **Windows, with one exception.** Frames 15 has a single Windows result: Blender 4.5 alone, on **Windows Server 2025 Standard**, on a machine with four cores and **no graphics card of any kind**, where one frame takes about half as long again as the Mac's. It settled the break sentence, the length of the wait and the exit status, and nothing else. **Its figures must never be set beside the Mac's or the Linux machine's.** Every other question cited in this document is untried on Windows — untried rather than out of reach, since a Windows machine can be rented — and **every Windows machine this project has read reported itself a Server edition**, so a Windows result has to name the edition and no desktop edition has ever been reached. *Frames 15, Reports 12, Reports 13, Reports 14, Writing 14, Settings 17, Settings 18, Settings 19, The compositor 9, What a render demands 7*

## What the evidence could not have shown

- **Reports 12's answer for 4.5 to 5.2 is two machines**, the Creator's Mac and one rented Linux machine, and one of its results is the machine rather than Blender: an unwritable output folder gives 1 on the Mac and 0 on that machine, because it was reached as its root user. No plain account was ever available there. Its 5.3 answer is the Mac alone, and an unwritable folder was not run in 5.3. *Reports 12*
- **Frames 15's timings are not clean.** The Linux machine's processor was shared with another tenant and its display was drawn by the processor rather than by its card, so its figures are its own; and every in-window run there was driven by a tool rather than by a hand, which nothing can show is the same road. *Frames 15*
- **Settings 17 is one machine — the Creator's Mac — for every one of its results**, including the swallowed misspelling in 4.5. *Settings 17*
- **The compositor 9 is one machine, the Creator's Mac**, including 4.5 aborting where the newer three and 5.3 end at 1; an abort is the kind of thing an operating system can cause as much as Blender. *The compositor 9*
- **Settings 19's substituted denoiser rests on two machines offering different denoisers**, which follows the card as far as two machines can show; no AMD or Intel card was available. *Settings 19*
- **Writing 14 used two instruments and they are two claims.** Blender was asked and the disk was watched on both machines; every file call was traced on Linux alone, and that trace settles Linux and says nothing about macOS. *Writing 14*
- The whole out-of-memory answer, and both of the statuses it gives, is one 49 GB Linux machine, one scene shape and no window; macOS is untouched, and 5.3 was not run at all. *Reports 10*
- The silence when a card fills with the render's own scene rests on searching both streams for four fixed phrases; a line using other words would have been missed. *Reports 1*
- The loud opposite case was run in 4.5 and 5.2 only, on a machine that was destroyed, so the two missing versions cannot be filled in. *Reports 2*
- Every run behind the File Output node's failures put the scene's own output somewhere with room to spare, so a run where both writes failed together could never have appeared. *File output nodes 15*
- That topic's stops landed at no particular moment against a write window about a thousandth of a second wide **in its own small scene**, so finding nothing part-written is what chance predicts. **That figure must not be carried to a working picture size**: measured across sizes the window runs from about a fifth of a second upwards. *File output nodes 13, Writing 11*
- The disk filling part way through a long animation was run in 5.2 alone, and what it printed was not kept. *Writing 5*
- Which difference between two Linux set-ups accounts for their different exit codes on a full disk is not established. *Writing 5*
- The kernel reset, and everything that rests on it, is one Linux machine; the Creator's machine is not reset for a test, so 5.3 is untried. *Frames 13*
- No byte count anywhere in the aimed-stop answer is a difference between versions; fifteen of sixteen cells came back a spread of sizes. *Frames 13*
- Which names are left in the folder after a machine goes down is not settled: two resets left it differently, and one of the two readings looks exactly like a difference between versions and is not. *Frames 13*
- That a node's file goes down before the scene's own, and what a stop between the two leaves, was run on both machines in 4.5 and on the Mac alone in the other three and in 5.3. *Frames 13*
- The video size boundary was reached, in 4.5 to 5.2, on one Linux machine only, and only by making the picture incompressible and the output a lossless codec; the Mac reached it in 5.3 alone, the same way, so neither system is covered in every version. *Frames 5*
- Everything about a stopped video after the first round was run, in 4.5 to 5.2, on one Linux machine alone, and in 5.3 on the Mac alone. *Frames 12*
- **Frames 12 read every leftover video file back through Blender's own reader, which reports an empty file as loaded, because neither of its machines had a media player.** A media reader was later installed on a different rented Linux machine, and there a stopped container was read by an ordinary media tool in all four versions — but that is Writing 14's result on its own machines, not Frames 12's. In 5.3 Frames 12's runs again used Blender's reader alone; the media tool's reading of a terminated video in 5.3 is Writing 15's, on the Mac. *Frames 12, Writing 14, Writing 15*
- Whether the last image on disk is whole after a terminate or a kill is not fixed in 4.5 to 5.2: three sets of runs disagree and all three stand. An interrupt has never left a short image in any run by anybody, 5.3's included. *Frames 12*
- One topic's stop test is a single untimed signal six seconds in, on the Mac, and its Linux run interrupted nothing because the render had already finished. *What actually ran 4*
- Whether the windowless silence about an unusable card is really silence rests on searching whole runs for one phrase; one of the four silences was run in 4.5 only. *The machine 4* The broken-driver one was run once, on a machine destroyed straight afterwards, and its own comparison says outright it cannot be checked. *The machine 2*
- "A missing dependency never changes the exit status" rests on missing-input cases alone; nothing wider was tried. *Dependencies 12*
- The wildly large render that printed no warning was killed at four minutes with hours of work left, so a later warning could not have appeared. *Settings 15*
- Every render behind the install and removal answers was thirty frames of Blender's own startup scene, no window, started from the command line. *Builds 6, Builds 8*
- The container refusals are recorded as agreeing on both machines, but only the Linux output was written down. *Images 4*
- Every missing-engine fallback wrote images of a few kilobytes; nothing says what one does at a size a real job reaches. *Add-ons 7*
- **The Creator's Mac is not one unchanging machine.** Its operating system and its Blender 5.2 build differ between test days; results from different days need not be results from the same setup. *Reports 12, Frames 15*
- **Every 5.3 result is one beta build on one Mac with a screen**, its window driven by Blender's own simulated keys and clicks rather than by a person — which is why an Escape that did not stop a 5.3 render says nothing about a person's Escape. *Frames 15*

## Where two topics do not agree

**A terminate's two numbers are one ending read two ways.** The same holds for a kill. The reader must be known before its number is interpreted; the pairs must not be read as version or machine differences. The reader behind the negative pair is unidentified. *One ending, two readers* above carries the consequence. *Frames 2, Frames 11, Frames 13, Writing 11, Settings 16, What actually ran 4, Reports 12, Frames 15*

**Whether a video file grows through a run.** Two topics record it standing at a fixed small size for the whole render and reaching its full size only at the very end. *Frames 5, Frames 12* Two others record it growing — one watching it appear at nothing and grow in steps, the other measuring about half its bytes on disk by the half-way frame and about nine tenths by the last, on two machines in all four versions. *Writing 11, Writing 14* **Growth during a run is established** — a long render closes one file off past two thousand million bytes and opens another while it is still going, which a file that never grew could not do. *Frames 5* **What is unsettled is where the line falls**: the runs that stood still were short and the ones that grew were long, and nothing anywhere measures at what frame count, frame size or container a file starts moving. The records are not all of the same video format, and no topic says the format is what decides it. **In 5.3, on the Mac, both kinds were seen on one machine** — a short MPEG-4 standing still while longer runs grew, and the smallest run there not growing at all — but picture size and codec were not varied apart, so that does not find the line either. *Frames 5, Writing 15* Until the line is found, a video file's size must not be read as a sign of how far a render has got.

**Whether a stop landing during the rendering leaves the empty file that reserves the frame in hand.** One topic fired a terminate the instant the frame's reserving file appeared and found that file left behind at nought bytes, alongside the whole frames already done, in all four versions on both machines. *Writing 11* Two others record the opposite for a stop landing at no particular moment part way through a run — no file at all for the frame in flight, not even the reserved one — one of them saying outright that it disproves the inference that a reserved file is what a stop leaves, also in all four versions on both machines. *Frames 6, Frames 10* **What does not separate them: the overwrite setting, which both records name and both had off, and the way the stop was sent, one side covering all three ways and the other a terminate. What does differ is the moment each stop was fired**, which nobody has varied on purpose. *Frames 10, Writing 11* **Whether this holds is not settled in 4.5 to 5.2, and the readings disagree.** Until it is, a program tidying up after a stopped render must expect the reserved file either way — and must not read its absence as proof the frame was never reached. In 5.3, on the Mac, the signal decided it — a terminate and a kill left the file, an interrupt took it away — which settles 5.3 and not the earlier four, for the reason under *What a re-run after a stop costs*. *Frames 16* A stop aimed instead at the moment the frame's file appears is not part of this: there the file stays, in all four versions on both machines. *Frames 14*

**Whether 5.2 renders the frame before ending, with the saving of render output switched off, is not settled.** The status is 0 and the output folder empty either way. Whether an active File Output node or the choice of a still against an animation accounts for the different outcomes is also not settled. In 5.3 on the Mac, with nothing else arranged, Blender refuses before rendering, for a still and an animation alike. That establishes neither 5.2's outcome nor 5.3 with an active File Output node. *Reports 12*

## Every question this rests on

- **Add-ons 7** — a missing engine falls back and reports success; 4.5 may not end.
- **Builds 6** — a Blender replaced under a render; the unsettled crash file.
- **Builds 8** — deleting the Blender under a render harms nothing.
- **Builds 9** — a half-copied build ends at 137.
- **Dependencies 12** — a missing dependency never moves the exit status.
- **Dependencies 17** — a name that exists counts as a frame already done.
- **File output nodes 13** — what its own stops left, and why they miss the write.
- **File output nodes 15** — the saved line over a part file; no property, no hook.
- **Frames 1** — recovery is a fresh render, never a resume.
- **Frames 2** — the frame in flight does not count as rendered.
- **Frames 3** — finished frames survive every ending.
- **Frames 4** — a frame is skipped because a name exists at its path, whatever is inside it.
- **Frames 5** — what a stopped video file holds, and what a split saves.
- **Frames 6** — a file existing, and a non-zero size, prove nothing.
- **Frames 10** — the order frames are worked through, and what a stop leaves in each.
- **Frames 11** — no stop-at-frame-end anywhere; the frame in flight is thrown away.
- **Frames 12** — images beside an empty video; a re-run replaces everything.
- **Frames 13** — the stop aimed at the write, an older frame lost to a stop mid-write, and the machine going down.
- **Frames 14** — a reserved name left behind by a killed render, and no later render fills it.
- **Frames 15** — what the stretch between a stop and an end contains, and how long it takes.
- **Frames 16** — what a stop leaves for the frame in flight; in 5.3 the signal decides whether the reserved name stays.
- **Frames 17** — what each way of stopping leaves of a video file.
- **Images 4** — a refused picture size writes nothing and a refused container-and-codec pairing writes a stub; both report success.
- **Reports 1** — what exit 1 and exit 0 are worth; the crash file; the silent full card.
- **Reports 2** — a card filled by a neighbour fails loudly at 1.
- **Reports 6** — the card filled by the render's own scene, the silent spill onto the machine's memory, and the 197 GB standing behind it.
- **Reports 7** — a picture of no pixels ends at 0 with nothing written; past one tile's worth of area a frame is worked and printed tile by tile.
- **Reports 10** — two endings when the machine's memory runs out, and no crash file.
- **Reports 11** — the printed mark of a frame that stopped short, the picture size it holds for, and 5.3 omitting the closing reset line.
- **Reports 12** — the ending status for every way a run can end, the runs that never end, and 5.3's no-camera 0.
- **Reports 13** — which stream carries a failure's words, and an empty error stream on an ordinary run.
- **Reports 14** — nothing already printed is lost, even to an outright kill.
- **Settings 1** — every view layer switched off ends at 0 with nothing written, in 5.3.
- **Settings 6** — a Workbench scene with no camera wrote its picture at 0, in 5.3.
- **Settings 9** — a run that wrote its frame and still ended at 1.
- **Settings 15** — a corrected value renders and reports success.
- **Settings 16** — what each way of stopping leaves and ends at.
- **Settings 17** — a swallowed setting name in 4.5; an unknown switch ends at 1 with nothing written.
- **Settings 18** — a sample count cut to the limit in silence and rendered.
- **Settings 19** — a substituted denoiser renders, denoises and reads back empty.
- **The compositor 1** — a version that crashed after one frame.
- **The compositor 9** — 4.5 aborts at 134 where the newer three end at 1.
- **The machine 2** — a broken or mismatched driver sends the work to the processor at exit 0.
- **The machine 4** — four silences about the card, all at exit 0; a technology named on the command line and not there ends at 1; where a crash file lands.
- **The machine 7** — a render outliving what started it; the unsettled hang-up; nothing on the path waits.
- **What a render demands 7** — a refused rendering method ends the run with nothing written; the largest scene killed by the machine.
- **What actually ran 2** — 4.5 ending at 134 or not ending at all.
- **What actually ran 4** — no documented exit status; quiet losses at 0.
- **What actually ran 6** — when each thing about a finished render can be learned, and which stopped renders still reach the closing report.
- **Writing 5** — the exit-status rule and the full-disk runs.
- **Writing 11** — the file grows in place and how wide that window is at each picture size; what a stop leaves with frames reserved; a stopped long video that had grown.
- **Writing 12** — a node's failed folder does not stop the render.
- **Writing 14** — a frame's file is whole when it is announced; nothing is renamed; frames go in the order given.
- **Writing 15** — what a video file holds through a run; a terminated one read by a media tool, in 5.3.
- **Writing 16** — when the frame-written handler fires, and what is on disk when it does.

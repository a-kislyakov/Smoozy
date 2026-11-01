# Two renders in one folder

Whether more than one Blender can share one output folder, and what is in the folder at the end.

> **This joins answers that live in several topics.** Every claim below names the comparison
> questions it rests on; open those for the exact words, the exact messages, and what each
> version does. Where this document and a comparison disagree, the comparison is right.

*All four versions* and *every version* below mean Blender 4.5 to 5.2, as they do in the comparisons; 5.3 is named wherever it is meant. Every 5.3 result is the 5.3.0 Beta on the Creator's Mac and nothing else, and 5.3 was run one Blender at a time — so nothing here about two Blenders at once is a 5.3 result.

## Read this first: the whole arrangement was measured on a small picture

**Every test anywhere in this project of two Blenders sharing one machine or one folder ran at a small picture size.** The largest was 1920 by 1080 and most were far smaller: 64 by 64, 320 by 320, 400 by 400, 480 by 270, 640 by 480, 960 by 540. *The machine 7, Frames 14, Frames 4, Reading a file 2, Add-ons 12, Builds 6, Builds 8, Writing 11*

Two things only begin above a size, and both are this document's own subject. **Cycles opens no tile file at all below one tile's worth of picture** — nothing at 2048 by 2048 at the shipped tile size, a file that appears and grows at 2048 by 2049 — and above that boundary one frame put about 89 megabytes into the machine's own temporary folder. And **a frame's own file is part-written for about a fifth of a second at 1920 by 1080, about two seconds at 8192 by 4608, and about three and a half at 10000 by 10000.** 5.3 showed the same boundary and the same shape on the Mac. *Writing 11*

The largest two-Blender run on record is under half of one tile's area. So **no two Blenders anywhere in this research have shared a machine while either of them held a tile file, or left a frame part-written for longer than a moment — and the arrangement this document is about has never been tested at the size the app exists for**, nor in 5.3 at any size. It bounds every question this document rests on for two Blenders at once: *Reading a file 2*, *The machine 7*, *Frames 4*, *Frames 14*, *Add-ons 12*, *Builds 6*, *Builds 8* and *Writing 11*. **Only *Add-ons 12* states that ceiling itself**, saying in its own words that nothing in it says what its answers do at a size, a length or a weight a real job reaches; the rest state no size at all.

**What a larger picture does to the race is not settled.** The two-render tests did not go above 1920 by 1080 (*Frames 4, 14; Writing 11*).

## The short answer

- Two Blenders run side by side on one computer without trouble, and one reading a file while another renders disturbs nothing — in 4.5 to 5.2, for none of it was tried in 5.3. **One thing does break that: on Linux, copying a Blender *over* the folder a running render is using kills it in all four versions — exit 135, four frames of thirty, no error of Blender's own — while putting a second Blender beside it, or deleting its folder outright, is harmless on both machines.** 5.3 on the Mac was untouched by a Blender copied in beside it; copying over it and deleting it were not tried. *Builds 5, Reading a file 2, Builds 6, Builds 8*
- Started together into one folder on the shipped settings, both renders render every frame. With Overwrite off and Placeholders on they divide the frames between them. Two renders were never started together in 5.3. *Frames 14*
- That division is not safe. A placeholder is a claim, not a lock, and two renders reaching one frame together have been caught both rendering it — in more than one version and on both operating systems. *Frames 4, Frames 14*
- It fails silently both ways — a frame rendered twice, or claimed and never rendered — and either way the folder ends with the expected number of files. **Nothing has to die for the second failure:** two renders that both ended by themselves at exit 0 left a folder with all forty files present and one of them empty, and both copies had printed that they were skipping that frame because a claim was already there. *The machine 7, Frames 14*
- For the tested one-view scenes, the check tests bare existence, and a frame's own file is visible part-written while it is being saved, so a name found is not a frame finished — both of those held in 5.3 too. **Nor is the line Blender prints when it saves a frame**: a write that starts and then runs out of room gets the same line over a part file, in every version and in 5.3. *Writing 11, Writing 14, Writing 16, File output nodes 15*
- An interrupt can leave a render working for a long, quiet stretch; it need not finish the frame's samples. A terminate or a kill ended the tested runs almost at once. **Whether a stop leaves an empty claim behind is not settled in 4.5 to 5.2**; in 5.3 the signal decides it — an interrupt takes it away, a terminate or a kill leaves it — and a leftover claim blocks that frame for good, in 5.3 too. *Frames 16, Frames 15, Frames 14, Writing 11*
- The two settings guard the scene's own output path only. A File Output node ignores both, and so does a video output — and in 4.5 a node wrote with no render started at all. *File output nodes 10, Frames 4, Frames 14, File output nodes 16*
- Two separate computers sharing one folder, which is what the arrangement is written for, has never been tried. *Frames 14, Writing 11, Dependencies 17, The machine 7*

## Two Blenders on one computer

Several Blenders living together on one computer is intended — though builds of the same two-number version share one settings folder, and one can take the file association from another. *Builds 5* A second Blender opening the same file while a render ran, and a third saving over that file mid-render, changed nothing: the render finished, wrote every frame, printed no error, and everything ended at exit 0. *Reading a file 2* A second Blender installing and removing add-ons at a running render changed nothing either — not one pixel of any frame differed from an undisturbed control. *Add-ons 12* **None of that was tried in 5.3**, which ran one Blender at a time; that two copies of 5.3 share one settings folder is its documents' reading and was not seen. *Builds 5, Reading a file 2, Add-ons 12*

**What a running render survives having replaced underneath it is decided by which folder it is.** Its own scene file can be saved over mid-render and nothing happens. *Reading a file 2* The whole program folder its Blender is running from can be deleted outright and nothing happens either — the render carries on, writes every frame and ends at 0, on both machines in all four versions. *Builds 8* But that same program folder having its **contents replaced in place** — a Blender copied over it — kills the render on Linux in all four versions, silently, at exit 135, where on the Mac the same test finishes. *Builds 6* So on a rented Linux machine each job's Blender belongs in a folder of its own, and nothing may be written into a folder a running render's Blender occupies. Whether the frames such a killed render had already written are whole was never checked — the test counted files and opened none. *Builds 6* 5.3 was tried only with another Blender copied in beside its running render, on the Mac, and was undisturbed; copying over it and deleting it were not tried, on any machine. *Builds 6, Builds 8*

**One thing on a shared machine really is locked, and it is not a frame.** A Blender that is rendering does not hold the extension repository lock, which is why an install fired at it disturbs nothing; but with one Blender holding that lock an install fired from another **refuses in under a second at exit status 1. It does not wait — it refuses.** And a Blender already running does not pick up an add-on another Blender installed into the same settings folder, and asking it to look again does not change that. All of this is 4.5 to 5.2's; 5.3 was not given a second Blender. *Add-ons 12*

What is not known is how many. No version names a maximum and no test can reach one; and nothing tested what two running Blenders do to each other for the card, for memory, for the temporary folder they both pick, or for the search path. *Reading a file 2, Builds 5* The temporary folder is where a Cycles render's tile file goes, named for the process that made it — but below the tile boundary there is nothing there at all, which is why no test on record could have found two renders treading on each other in it. *Writing 11*

## What the two settings guard, and what they leave open

Two renders started together on the shipped settings both render everything, and no work is divided at all. Overwrite off with Placeholders on is what divides it. *Frames 14* The skip happens before the frame is rendered, and "already there" means only that a name exists — a zero-byte file, text that is not an image, an image of the wrong size, and a folder were each skipped untouched. Nothing looks inside. That half needs only one Blender, and it held in 5.3 on the Mac. **Every skip test used one view; what name is checked with two views is not settled.** *Frames 14, Frames 4*

The placeholder is put down for the frame in hand, one at a time, before that frame is rendered, and then filled in place; only ever one is waiting — in 5.3 as well. So claiming a frame and checking for a claim are two separate steps, which is where a race gets in. *Writing 11, Writing 14, The machine 7*

**A frame's own file is on disk part-written while it is being saved**, appearing at or near zero bytes and growing in place — no name ever appears in the folder that is not a final frame name, and on Linux, traced call by call, nothing is renamed. So the name a check finds may belong to a file the other render is still writing, and bare existence cannot tell that from a finished frame. That window is about a fifth of a second at 1920 by 1080, about two seconds at 8192 by 4608 and about three and a half at 10000 by 10000, and 5.3 on the Mac came out the same shape — so at the size a real job renders it is a real slice of every frame rather than a sliver. *Writing 11, Writing 14*

**Nor does the line Blender prints when it has saved a frame settle it.** Every run behind the finding that a file is whole the moment that line appears had room on disk. A write that starts and then runs out of room leaves a part file and gets the same line over it, in every version on both machines and in 5.3 on the Mac. *Writing 14, Writing 16, File output nodes 15*

**Opening the file is no proof either.** A frame cut short during its write can open at its full dimensions with wrong pixels. In 5.3 the cut-short JPEGs did; the earlier tests also found readable short files. Whether an empty placeholder or a part-written image is left depends on when the stop lands, so neither file count nor a successful open establishes that the shared folder holds finished frames. *Frames 13, Frames 16*

Nothing in any version's output path identifies the machine, the process or the run, 5.3 included, so two runs of one file produce identical paths. Those two settings are all that stand between them. *Frames 14*

Neither setting reaches a File Output node: the node replaces an existing file silently and writes no placeholder. In 5.3, with Overwrite off, the scene's own frames alone decide which frames are rendered, and the node then writes over its own files for them. Neither setting reaches a video output either, where both renders write to one file and both append their frames to it. *File output nodes 10, Dependencies 17, Frames 4, Frames 14*

**And a second Blender does not have to be rendering to be writing into the folder.** In Blender 4.5, on one rented Linux machine, a File Output node wrote its files while somebody merely played a sequencer preview at rendered shading, with no render started at all. 5.3 on the Mac wrote nothing that way; whether 5.0, 5.1 and 5.2 do is not established, because their runs never set which scene the sequencer showed. So on 4.5 at least the two settings are not the only thing left out — a Blender that is not rendering is not necessarily a Blender that is not writing. **Bounded hard**: the 4.5 result is one rented Linux machine whose display is drawn by its processor rather than by a card, no card-drawn viewport was ever put beside it in 4.5 to 5.2, and 5.3's run on the Mac's own screen is one version. The same run made with no window wrote nothing in any version, 5.3 included. *File output nodes 16*

## Why three tidy results prove nothing

Three separate topics record two Blenders dividing a shared folder's frames between them, with nothing rendered twice. Not one of them is evidence that the division holds: one started the second render five seconds behind the first, so no two renders in it ever reached one frame at once; one kept only the end of each copy's log, so a collision in it could not have shown either way; and one is a single run whose own comparison warns against reading the clean division as a property of the arrangement. **A reader who opens any one of those three and stops comes away with the wrong rule.** *Dependencies 17, The machine 7, Writing 11*

**Two of those three could be posed again, and one limit that looks like Blender's is not.** The run that kept only the last fifteen lines of each copy's log lost its evidence in the recording rather than in Blender: a render's own printed lines reach whatever is reading them as they are produced, in dozens of arrivals spread across a run, and **nothing already printed is lost even to an outright kill** — a log read after a killed run is complete up to its last line. Both held in 5.3 on the Mac. So a repeat that keeps whole logs would show a collision or show none, on both machines and in all four versions. *Reports 14, The machine 7*

Between them, the two questions that ran it as a race have caught a collision on a local disk in all four versions on one operating system and in one version on the other, and over a network file system the rate rose sharply in every version. **Both comparisons say the per-version tallies are a race falling different ways — not a difference between versions, and not a rate to plan by**, and one of those version records disagrees with its own printed output, which is a further reason not to count. *Frames 4, Frames 14*

## What the folder holds, and what the exit status says

On a local disk, on either arrangement, the files are all present and all readable at the end, including where a collision wrote one frame twice. *Frames 14*

A render that dies leaves a hole no later render fills: its placeholder stays at zero bytes, and every later render skips that name. 5.3 did the same with one Blender, killed and started again. *Frames 14*

**And a render does not have to die to leave one.** Two copies sharing one folder, both of which ended by themselves at exit status 0, left all forty files present with one of them empty: the frame was claimed by one copy and skipped by both, and both said so on their way past it. Which of the forty it was is not in the record. *The machine 7, Frames 14* So the two roads to a lost frame end in the same place, and only one of them leaves anything for a repair to notice. **Nothing in the exit status carries either: exit 0 does not mean every file is whole.** *File output nodes 15*

It does not mean anything was written either. A run ends at 0 having rendered nothing at all in three ways in every version, 5.3 included — a frame number that is not a number, the output option given with nothing after it, and an animation whose start frame is past its end frame — and in a fourth, the scene's output switched off, in 5.2 and 5.3. In 5.3 the command-line render of the tested Cycles scene on the processor, with its Camera object deleted, ends the same way; 4.5 to 5.2 and the 5.2.2 run fail. That does not settle other engines or ways of leaving a scene without a camera, and a script-called render still fails in 5.3. An empty frame argument also ends at success in 5.3, with no earlier counterpart tested. *Reports 12*

**How long a render holds a frame depends on the stop and the work in progress.** An interrupt can leave a long stretch with nothing printed, then discard the frame without writing its picture. The measured wait varies with the point of interruption, the sample count and the device; some runs stopped before their samples were finished. It is not a promise to finish the frame, and no general upper bound is established. A terminate and a kill ended the tested runs almost at once. These results cover 4.5 to 5.2 on the named Mac and Linux machines, and 5.3 on the Mac. A second render could meet the first one's unfinished claim during that wait, but no stop-delay test had a second render sharing the folder. *Frames 15, Frames 14*

**That deadline is bounded by picture size too, and its own question says so outright.** The main Cycles timing tests used a scene at 320 by 180, below the tile boundary. Other-engine tests used a different scene, at 1920 by 1080 in 5.3. What "the work in hand" even means once a frame is tiled — the frame, or the tile — has never been asked at any size. Neither those untiled tests nor the longer waits on a software-drawn display establish a deadline for a tiled frame or a long video's closing step. *Frames 15*

What remains at that frame's path is **not settled in 4.5 to 5.2**, and in 5.3 depends on the stop; see *Where two topics do not agree*, below. *Frames 16*

## What this does not cover

- How many Blenders one computer will take, and whether a second render slows the first or contends with it for anything. *Reading a file 2, Builds 5*
- Two separate computers writing into one folder. *Frames 14*
- Two Blenders at once in 5.3, in any arrangement. 5.3 ran one Blender at a time, on the Creator's Mac alone, so every two-Blender result here is 4.5 to 5.2's. *Frames 14, The machine 7, Reading a file 2, Add-ons 12*
- The full account of stops aimed at a frame's write is *Frames 13*. The consequence for checking a shared folder is carried above.
- Windows, on everything about the shared folder itself. **One Windows stop test exists, and it is not about a shared folder**: an interrupt delivered to Blender 4.5 on Windows Server 2025 Standard, with no graphics card, ended after a wait at the same status as on the other two systems. That one wait establishes no deadline. One version, one machine, one test; the other fifteen of that question's tests, and every other question this document rests on, were never begun there, nor anything in 5.3. *Frames 15, Frames 14* **The rest is untried rather than out of reach** — this project has a route to a Windows computer, described in the equipment section of `Preparation/Blender Research/Method.md` — and every Windows machine it has read reported itself a Server edition, so a Windows answer that does not name its edition is not yet an answer.
- The picture size a real job renders at, on every result here. See *Read this first*, above.

## What the evidence could not have shown

- Every run behind this document had both renders on one computer. Two computers on one folder was never set up anywhere. *Frames 14, Writing 11, Dependencies 17, The machine 7*
- The one network file system tried was mounted by the same machine both renders ran on, and no other was tried. It ran on one Linux machine and nowhere else, so the sharp rise in collisions is settled for Linux and open for macOS. *Frames 14*
- The run that divided forty frames kept only the last fifteen lines of each log, so a collision in it could not have shown either way. *The machine 7*
- The frame that was claimed and lost was one file in one run of one version. The other three versions on that machine left none, and so did the same version on the other machine. One run each, so there is no rate in it. *The machine 7*
- Another divided run started the second render five seconds late, so no two renders in it ever reached one frame at once. *Dependencies 17*
- The clean odd-and-even division is one run on one machine's own disk, and its comparison says outright not to read it as a property of the arrangement. *Writing 11*
- Twelve runs on one Linux machine that never collided are twelve runs that happened not to collide. *Frames 4*
- The video case was read back by Blender's own reader alone; neither machine had a media player. *Frames 14*
- Nothing measured time or memory while a second Blender was running, so a slowdown could not have appeared. *Reading a file 2*
- None of the four ways two Blenders might contend was tested at all. *Builds 5*
- The part-written window was watched on both machines, and on both a watcher found no name in the folder that was not a final frame name — but that there is no rename call at all was traced on one rented Linux machine alone, and that trace says nothing whatever about macOS. *Writing 14*
- Every two-render result here was measured on a picture too small for either render to hold a tile file, the largest at 1920 by 1080 against a tile boundary of 2048 by 2048. *Read this first*, above, holds the whole of it and the questions it bounds.
- The four frames the killed render left on Linux were counted and never opened, so whether they were whole is not in the record. *Builds 6*
- No run behind any of this was two renders arriving at one frame's part-written file; the part-written window and the shared folder were measured by separate questions and never together. *Writing 11, Writing 14, Frames 14*
- **No run that measured how long a stop takes had a second render sharing its folder**, in any version, 5.3 included, so what a waiting render does about a frame the first has not let go of was never watched at all. *Frames 15, Frames 14*
- The node writing during a sequencer preview is one version on one rented Linux machine, driven at a window that machine's **processor** draws rather than its card. It is the one test in that topic whose subject is what a viewport does, no card-drawn viewport was put beside it in 4.5 to 5.2, and 5.3's run on the Mac's own screen is one version — so whether 4.5's result holds on a workstation's screen could not have shown. *File output nodes 16*
- Every 5.3 result here is one Blender on one Mac in a beta build. The parts of the arrangement one Blender can show held there; nothing about two Blenders at once could have shown in 5.3 at all. *Frames 14, Frames 16*

## Where two topics do not agree

**Whether a stopped render leaves an empty claim behind for the frame it was working on.** *Frames 16* holds the whole of it. With Placeholders off, nothing is left for that frame, in every version and in 5.3. With Placeholders on, in 4.5 to 5.2, *Frames 3*, *Frames 6*, *Frames 10* and *Frames 15* record nothing at all left for that frame after a stop sent at no particular moment — no picture and no empty file — on the Creator's Mac and on rented Linux machines, and for an interrupt, a termination request and an outright kill alike. *Writing 11* records the opposite from the same kind of run: the empty file for the frame in hand left behind at zero bytes, in all four versions on both of its machines. **Neither reading can be read into the other and neither is chosen here.**

**The setting and signal do not explain the specified 5.1 disagreement.** Both specified setups have Overwrite off and Placeholders on; a terminate leaves the empty file in one and nothing in the other. The other Frames setups do not state Placeholders. The stops also landed at different moments, but whether that accounts for the disagreement is not settled. *Frames 16; Writing 11*

**In 5.3, on the Mac, the signal decides it.** An interrupt took the empty file away; a terminate and a kill left it, at zero bytes, and a render started again with Overwrite off skipped that frame and left it empty. A click on the cross in Blender's own status bar, sent as Blender's own simulated event, left nothing. That would square both earlier records only had every Frames-side stop been an interrupt, and the one Frames record that names the setting has a terminate and a kill leaving nothing — so **5.3 does not settle 4.5 to 5.2**, and nothing about the earlier four may be built on it. *Frames 16, Frames 14*

**It matters here more than anywhere else it appears.** *Frames 14* settles what a leftover empty file does — every later render skips that name for good and the file stays unopenable — and *The machine 7* reaches the same ending by the other road, a frame claimed, never rendered, left empty, with the run still reporting the full file count at exit 0. So **the cost of a leftover claim is established twice over, while whether a stop produces one is not established at all in 4.5 to 5.2 — and in 5.3 two of the three ways of stopping do.** Until the earlier four are settled, nothing that restarts or divides up a stopped render may trust the file count, and in 5.3 it may not after a terminate or a kill.

## Every question this rests on

- **Add-ons 12** — installs fired at a running render change no pixel; the extension repository lock refuses rather than waits; and the one question in this area that states its own size ceiling. None of its two-Blender half ran in 5.3.
- **Builds 5** — two Blenders side by side, shared settings, untested contentions; two 5.3 copies sharing settings is inferred, not seen.
- **Builds 6** — a Blender copied over the folder a render is running from kills it on Linux and not on macOS; in 5.3 only a Blender copied in beside it was tried.
- **Builds 8** — deleting that folder outright harms nothing, on both machines; not tried in 5.3.
- **Dependencies 17** — a divided folder with no race possible in it; in 5.3, a File Output node writing over its own files when the scene's frames decide.
- **File output nodes 10** — the node ignores both settings.
- **File output nodes 15** — exit 0 proves no file whole, and a saved line proves none whole either.
- **File output nodes 16** — in 4.5 a node writes while a sequencer preview merely plays, with no render started; 5.3 on the Mac did not.
- **Frames 3** — finished frames survive a stop, and nothing is left for the frame in flight.
- **Frames 4** — the skip rule, and collisions on one machine and not the other.
- **Frames 6** — a file existing is no proof the frame finished.
- **Frames 10** — the order a render works through its frames, and what a stop at no particular moment leaves with Placeholders off.
- **Frames 13** — what a stop aimed at the moment of a frame's own write leaves.
- **Frames 14** — the whole arrangement, the race, and what the folder holds; never run with two Blenders in 5.3.
- **Frames 15** — how long a stop takes and what settles it, at 320 by 180; a kill ending it at once; the one Windows figure anywhere behind this document.
- **Frames 16** — the whole of the disagreement about what a stop leaves for the frame in hand, and 5.3's answer by signal.
- **Reading a file 2** — a second Blender during a render; no maximum anywhere; not tried in 5.3.
- **Reports 12** — the ways a run ends at 0 having written nothing.
- **Reports 14** — a render's own lines arrive as they are produced, and nothing already printed is lost even to a kill.
- **The machine 7** — a claimed frame lost, reported at exit 0.
- **Writing 11** — placeholders laid down frame by frame, the part-written window and how it grows, the tile boundary, and one clean division.
- **Writing 14** — one placeholder waiting at a time, nothing written under a temporary name, and the announcement that is not a proof.
- **Writing 16** — what an announcement that a file is written stands for on disk.

## What bears on nothing here

Fourteen comparison questions were read against this document and **touch nothing in it**. Each is named with the reason, so that nobody has to look again:

- *Builds 10* — what a machine must hold before an unpacked Blender will start. Nothing in it is about a folder or a second Blender.
- *Dependencies 20* — what a file records for a place nobody touched. Nothing was rendered for any of it, and no result there concerns two renders.
- *File output nodes 17* — what a file written through a scene strip is called. A naming answer about one render's own files; it carries nothing about a second render, and the strips that write over each other are one render's.
- *Frames 17* — what each way of stopping leaves of a single video file. One render's own file; nothing about a second render or a shared folder.
- *Reports 13* — which stream a missing file's complaint lands on.
- *Settings 17* — the names a launch instruction uses for three settings.
- *Settings 18* — what values a sample count will take.
- *Settings 19* — the denoising settings a scene and a view layer carry.
- *The compositor 9* — what a scene pulled in through the compositor is rendered with.
- *The machine 8* — what a render does where the screen is drawn by the processor. It is about one render on such a machine, not about a folder or a second Blender.
- *What a render demands 6* — what a scene records about subdivision, child particle and texture limits.
- *What a render demands 7* — where a card's rendering method is chosen. It does name one further way a render can die, on one machine, and the rule above already covers a render that dies however it died.
- *Writing 13* — what a launch-time script sees of a setting given on the same line. It is the means of giving two runs different folders, which is the case this document is not about.
- *Writing 15* — what is in a single video file through a run. One render's own file; the shared video case is *Frames 14*'s.

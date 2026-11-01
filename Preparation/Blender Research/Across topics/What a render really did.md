# What a render really did

A render finished and reported success — this says what that proves, what it does not, and where the real answer can be read back from.

> **This joins answers that live in several topics.** Every claim below names the comparison questions it rests on; open those for the exact words, the exact messages, and what each version does. Where this document and a comparison disagree, the comparison is right.

*All four versions*, *every version* and *both machines* mean Blender 4.5 to 5.2 below, and *the newer three* means 5.0 to 5.2; 5.3 is named wherever it is meant. Every 5.3 result is the 5.3.0 Beta built 2026-10-16 on the Creator's Mac and nothing else. *Reports 1; Frames 15; What actually ran 5*

## The short answer

- A run that ends at 0 with a frame on disk reached its end. It does not mean the render did what was asked (*Reports 1*).
- At least fourteen different things can go otherwise, write a frame and still end exactly that way, and most of them say nothing at all (*The machine 4; Settings 15, 17, 18, 19; Add-ons 7; Dependencies 12; What actually ran 4*).
- On top of those, six of the cases listed next door write no frame at 0. In 5.3 a seventh does too: the tested Cycles scene, on the processor, with its Camera object deleted. This is no general rule for a scene without an assigned camera (*Reports 1, 12; Reports 7*).
- The only place that names the part of the computer that did the work is what the run printed, and only when it was given the right switch for its version. **From 5.0 the run also names the engine; 4.5 names it nowhere, so in 4.5 the engine cannot be read out of what Blender prints at all** (*What actually ran 1, 2, 5; The compositor 9*).
- Nothing written into a finished picture names the device or the engine, and the sample figure it carries is the number asked for, not the number reached (*Images 10; Reports 11*).
- Anything asked of a running Blender afterwards answers for the moment it is asked, and records what was asked for rather than what worked (*What actually ran 3; Add-ons 13*).
- So the check has to be built into how the render is started and what is read while it runs — and **there is a way to build one** (*Writing 13; What a render demands 5; What actually ran 6*).

## Fourteen ways a render does something else and still reports success

These were found scattered across nine topics. Each has the same signature — the run finishes, a frame is written, the process ends at 0. No comparison holds the list, because no comparison had reason to.

- **The processor did the work the file asked the card for.** Five separate causes: no graphics technology chosen, which is the factory state on the tested rented Linux machines; the driver removed or mismatched; the cards hidden from Blender; a machine with no card of the kind asked for (*The machine 2, 4, 5; Settings 9*); and an unprepared settings folder on the Mac. In that test a scene asking for the card path-traced on the processor even though Cycles had found the graphics, in all four versions and 5.3. In 5.2 and 5.3 a refresh and a save together made the card usable; whether the save itself is needed is not established. This is not a rule that preferences must always be saved before rendering (*What actually ran 4; The machine 5*).
- **The card filled and the rest spilled into the machine's own memory** — the picture is written and the run takes much longer. Never reached in 5.3 (*Reports 1, 6*).
- **A different engine rendered the frame.** With the engine's add-on absent or switched off, the run says the engine is unavailable and then renders in EEVEE at EEVEE's own count rather than the file's (*Settings 13; Add-ons 7; What a render demands 4*).
- **The picture came out at a size nobody asked for.** A resolution percentage outside what the setting accepts is corrected the instant it is assigned, nothing is said, and the frame is written at the corrected size (*Settings 15*).
- **The frame took fewer samples than the count it was given.** Five things in Cycles stand between the number and the samples a pixel gets, including a view layer's own count displacing one handed in from outside (*Settings 16*). Adaptive sampling, which starts on, ended the tested plain scene far below its maximum at a looser threshold than the factory one, in all four versions on two machines and in 5.3 on the Mac. **A time limit applies to each tested Cycles frame, not to the whole job, and a frame cut short is still written.** From 5.3 EEVEE has a time limit too: it can write a frame below its count without saying why. Workbench has neither kind of early stop (*Settings 2, 16*). At the factory noise threshold, a noisy test scene instead printed its full count while doing less work than the control. Neither a full count nor a short one measures the work done. The time-limit tests used Cycles on the processor; whether the limit behaves the same on a card, or applies per view layer in a scene with several, is not settled (*Settings 16*).
- **The count that was handed in was cut.** A sample count above what the setting accepts is stored at the limit and rendered at the limit, in silence. Each count is cut against its own limits, so **there is no single ceiling for "the sample count"** — the same number is cut for one engine and taken whole by another, and neither says a word. **Two places carry the cut figure and neither announces it**: the setting read straight back, and the ordinary progress line — and that line is not always to be had, because a render driven from inside a script prints no progress at all after 4.5, and because a sample subset or a view layer's own count puts a different number after the slash (*Settings 18; Settings 16*).
- **The denoiser was not the one the file named.** A file naming a denoiser the machine has not got still renders, the picture is still denoised by whatever the machine has, and the setting reads back as **empty text** rather than the stored name. A warning is printed only when a script reads the setting; nothing about it appears in the render's own output (*Settings 19*).
- **A launch instruction did nothing and said nothing.** In Blender 4.5 alone, and on Cycles' settings alone, a misspelled setting name handed in at launch is accepted without a complaint, but its value is gone on reading it back. The exit status stays 0. A different failure can give a misleading read-back: a correctly named setting placed before the file reads back as requested at that moment, then the file load replaces it. A check has to happen after the file is loaded (*Settings 17*).
- **Something the file points at was missing and Blender substituted.** A picture becomes magenta in the shading system, black in the compositor and zero in the tested geometry node setup; a font loses its letterforms; a colour-space name the active configuration does not carry is thrown away and replaced. Tested cloth, geometry-node and light-probe caches can be absent without a missing-bake warning, while the picture changes; that does not mean every absent bake is simulated again. In 5.3 on the Mac a missing Alembic cache warned and drew the object's own mesh instead. A missing IES file, or an OSL script gone together with its compiled copy, changed the picture. An ocean whose baked files were gone rendered as an even grey with no waves, silently, in 5.3 and the 5.2.2 run. Each ended at 0 (*Images 2, 9; Dependencies 12, 15, 16, 18; Reports 13*).
- **A value read out of the file was not the value used.** Launch arguments and saved preferences override it, 5.0 to 5.2 rewrite a 4.5 file's tiling as it loads — 5.3 was never given such a file — and a count given at launch is ignored if it sits on the wrong side of the render argument, as an engine given at launch is when it comes before the file (*What a render demands 3, 5; Settings 1, 16*).
- **An add-on was installed and switched on and the part that does the work was not there.** A compiled part built for another kind of computer is accepted without a word, and the failure comes only when something uses it — and that run ends at 0 too. 5.3 came back the same shape on the Mac, but with bundled files holding no compiled code, so it is not set beside that result (*Add-ons 4*).
- **A file on disk was read in a stale state.** A geometry node setup re-reads a changed file only once its modification time has moved by a whole second; a change inside the same second is not noticed and nothing is said (*Node setups 1*).
- **The output was not written in the format that was named.** An unknown format name draws a complaint and the frame is then written in the scene's own format; a format that cannot hold the block of named fields drops it without a word; and from 5.0 on, a **script** that asks for multi-layer output by naming the file format alone, without also setting the media kind, is given an ordinary picture instead — with no error recorded in the earlier runs of 5.0 to 5.2, while in 5.3 and in the 5.2.2 run on the Mac Python refuses the setting with a line on the error stream, so whether 5.0 and 5.1 say anything is not settled — the launch line is a different road and takes the same name whole, writing a real multi-layer file in all four versions (*What actually ran 4; Reports 11; Settings 17*).
- **Path guiding or Open Shading Language was asked for, and the graphics did the work without it.** With either switched on and the Mac's own graphics rendering, the run named the graphics, wrote its frame, ended at 0 and printed what the same run with the switch off prints; on the processor each takes effect. 5.3 and the 5.2.2 run on the Mac alone (*What actually ran 4*).

**What joins them is that nothing in the ending separates them from a correct run.** Several print nothing whatever, and where something is printed the wording, the stream and the shape of the line differ between versions, so any check built on Blender's words has to be written per version (*The machine 4; Images 2; Dependencies 12; Reports 1, 13*).

**Four of the fourteen carry bounds that must travel with them.** The swallowed instruction was found on the Creator's Mac and on no other computer — nothing was rented for that question, and Smoozy renders on rented Linux (*Settings 17*). The substituted denoiser was found in 4.5 to 5.2 by carrying a file made on a machine with an NVIDIA card to the Mac, which offers no such denoiser, and in 5.3 by writing the stored name into a file on the Mac directly; what denoisers a machine offers follows the card, as far as two machines can show (*Settings 19*). And the cut count has no single number behind it, as above (*Settings 18*). The unused path guiding is one Mac's own graphics, in 5.3 and in the 5.2.2 run there, and was not run in the earlier records (*What actually ran 4*).

**One thing cuts the other way, and the join is worth holding.** Where the rendering method is named **on the command line** and the machine has no hardware for it, the run does not fall back at all: it ends with a failure status and writes no file, in all four versions on both of that question's machines and in 5.3 on the Mac (*What a render demands 7; Reports 12*). Where the same is true of a **stored** preference or scene setting, the processor quietly does the work and the run reports success (*The machine 4, 5; Settings 9*). So the loud case and the silent case are the same lack of hardware reached by two different roads.

## Where an answer can be read back from

There are three places, and each answers a different question (*What actually ran 5*). What each can actually carry is spread across topics.

**What the run printed.** This is the only place that names the part of the computer that did the work, and it takes a switch that is not the same one in every version (*What actually ran 1, 5*). **It names the engine from 5.0 on and never in 4.5**, and that one needs no switch at all — so in 4.5 the only way to tell which engine ran is the picture (*What actually ran 2; The compositor 9*). Two things fall short of a name and still hint at a card, in every version: one route to the graphics puts its own name into the ordinary progress lines where the other puts nothing, and the memory figure jumps (*What actually ran 1; What a render demands 1*). Cycles' own statistics name no device in any version (*What a render demands 1*), and nothing printed anywhere is marked as a graphics card's memory (*Reports 6*). In 5.3, on the Mac, the device that Cycles' own log named was the device the picture shows rendered, in every crossing of file, preference and launch argument tried; no earlier version was checked that way (*What a render demands 3*). One further thing is nearly only there: a sample count that was cut appears in the ordinary progress line and in the setting read straight back, and nowhere else — so on the route that prints no progress, the read-back is the whole of it (*Settings 18*). **And all of it turns on how the render was started**: from 5.0, a render driven by the render call from inside a script prints no progress line at all while still printing the line that says the frame was saved, where 4.5 prints its full progress either way (*What actually ran 5; Settings 15*). **A script's own printed lines are a separate case again — they do not reach a reader until the run ends** (*Reports 14*).

**The picture on disk.** In the formats that keep any, it carries a block of named fields, plus four fields Cycles writes for itself that no scene setting removes and that an EEVEE render of the same frame does not carry (*Images 10; What actually ran 2; Frames 10*). **The block runs to fourteen fields only where all fourteen switches have been turned on. A file nobody has touched carries seven** — its own path, the date, the time, the frame, the camera, the scene and the render time. The seven off are the frame range, peak memory, machine name, lens, marker, strip name and note. So the named record says which machine rendered it or how much memory it took only where somebody set that switch before rendering (*Images 10; Frames 10*). None of those fields names the device, engine or add-ons. The picture's dimensions are recorded separately by its file format (*Images 10; What actually ran 1, 3*). The sample figure among Cycles' fields is the maximum asked for, not the number reached. A separate per-pixel sample pass does reach a saved file in 4.5; **whether it can be relied on in 5.0 to 5.3 is not settled**, as below (*Reports 11; Images 10; What actually ran 4*). From 5.0 an OpenEXR names the Blender that wrote it even with the scene's metadata switches off (*Images 10; What actually ran 2*). In 4.5 to 5.2 no route to read those fields through Blender's own Python was found. 5.3 ships a module that reads an OpenEXR's header from Blender's own Python; a PNG's fields still cannot be read that way (*What actually ran 5*).

**And nothing in it says what colour treatment produced it.** In the two formats that hold more light than a screen can show, the view transform, the look, the exposure and the gamma are recorded nowhere, in any version: three files written under three deliberately different treatments came back with the same header text apart from the clock readings, on two machines. Nothing a person can ask Blender to write into a picture covers any of it either. What did move across the versions is the colour **space**, which is a different thing — from 5.0 one of those formats names it, from 5.1 the ordinary picture format carries blocks describing the display space it is in, and from 5.3 it carries an sRGB profile and a block naming its primaries and curve as well — and none of that says how the picture was treated. So a finished file's own record cannot be checked against the treatment that was asked for: nothing in the header names it. **Whether the pixels differ was never looked at.** The three files did carry different amounts of pixel data behind their identical headers, but they were three separate renders, so nothing follows from that either way — the picture is where a difference would show, and nobody has compared it (*Images 6*).

**Asking a running Blender.** It answers for the moment it is asked and never for a render that has been (*What actually ran 3, 6*). Two limits meet here that neither comparison carries together: the settings read back can be shown to name a route the render did not take (*What actually ran 1*), and the list of enabled add-ons records what was asked for rather than what loaded (*Add-ons 13*) — and in 5.3 and the 5.2.2 run on the Mac it leaves out what a launch switch loaded for one run, so it does not even record all that was asked for (*What actually ran 3*). **So the one route that answers at all answers about intent.** Reading the file after the run adds nothing either — launch arguments never touch it (*Settings 1; What a render demands 5*).

## Checking the intent before the render starts, which is the one positive answer here

A script handed in at the same launch as the setting **can** read that setting back, compare it against what the caller meant, and stop the launch when they differ — the render does not happen, nothing is written, and the launch ends at 1. Handed in in the right position, the same check passes and the frame is written (*Writing 13*). A plain script exception is not enough: it can leave the run at 0 and let rendering continue. The check must explicitly stop the launch or have script errors set to stop it (*Reports 12*).

Four things bound it, and each has to be handled on its own (*Writing 13*):

- **Position on the launch line decides everything.** The arrangement that works is the file, then the setting, then the script. A script written before the setting reads the file's own value; a setting written before the file is wiped by the file load.
- **What comes back is the raw text that was handed in**, not a finished file name — a path with template marks or hash marks in it reads back with them still there — so the comparison is against the text and nothing else.
- **An output path given at launch reaches the active scene alone**, and no other scene in the file. In 5.3 that is the scene active at the moment the setting is read, so a scene chosen at launch has to come before it; no earlier version was asked.
- **Two settings do not fit the pattern at all.** The thread count takes effect wherever it is written, and the graphics device given to Cycles on the command line lands nowhere a script can read.

This whole answer is two machines — the Creator's Mac and one rented Linux machine — in all four versions, and 5.3 on the Mac, and nothing on Windows (*Writing 13*).

## At what moment each becomes knowable

- **Before the render:** what the machine can offer, and what the file holds — but not what will be used (*What actually ran 6; What a render demands 5*). **The device list must first be filled**: a fresh Blender can return an empty list until a refresh or the preferences panel fills it. That empty answer is not proof of no card, and a saved preferences file is not established as necessary (*The machine 5*). A caller can also check its intent against the values after loading the file, as above (*Writing 13*).
- **While it runs:** how far the work has got, and which frame is in hand, by a different means in different versions (*What actually ran 6*). Not what is doing the work — the two hints in what it prints name nothing (*What actually ran 1*).
- **Only at the end:** the part of the computer that did the work, the render time and the peak memory. The line naming the device is a closing report, printed after the last sample and before the frame is written (*What actually ran 1, 6*).

**The join a builder needs, and it is not the simple one.** A run that was stopped **does** reach that closing block in 5.0 and 5.1 — the whole block, device line and all, arriving after the stop and before the process goes — while 4.5, 5.2 and 5.3 print no such block at all, 5.2 and 5.3 printing a line that the rendering was cancelled instead. Three things hold it down. In the runs that produced it the frame's own sampling had already finished inside the wait, so whether a frame whose sampling was cut short prints a device report at that logging level is not established. Ordinary 5.3 output does capture sampling cut short without a finished line. It was got with the fullest logging on, on one machine. And **one version's own corrective notes call that block on exit unreliable, its numbers wrong** — a run bears them out, the block's sample count disagreeing with the progress line — so nothing establishes that the device name inside it is right either (*What actually ran 6; Frames 15*). No field written into a frame names a device in any case (*What actually ran 1; Images 10*). So for a render that did not finish, what names the work is at best a reading nobody has shown to be true, and the two hints in the progress lines are the rest of it.

## Checking by rendering twice and comparing

The obvious check has a joined answer, and its halves sit in different topics.

- On a small factory scene written to PNG, a processor render and a card render were the same picture (*Frames 10*).
- On a heavier scene written as full-float, they were never the same — on two makes of graphics, three routes to them, four versions, and again in 5.3 on Apple's (*What a render demands 2*).
- Where the boundary between those two lies is not located; nobody has run both comparisons on one scene (*What a render demands 2; Frames 10*).
- And compare the picture, never the file: a written frame carries clocks, so two files of one frame match only when those readings happen to land alike (*Frames 10*). **Compare it inside one build, too.** Every comparison behind that rule was made within one version, and the one place a picture was compared across versions found it moving in the fourth decimal; no run anywhere has compared a picture between two builds and found it identical (*Frames 10*).

**What such a comparison is worth depends on what two runs of the same thing come out at, and that has been measured at length** (*What a render demands 7*):

- **The tested processor repeats were identical on the Mac**, across all four versions and in 5.3. That establishes repeatability for those scenes and settings, not for every processor render. In a separate 5.3 test with Texture Cache files in use, two processor renders of one file differed. Whether that exception holds in earlier versions is not settled (*What a render demands 7, 2*).
- **On a card the floor is not one number.** Nearly every pair sat at a few ten-millionths of a channel value, and about one pair in thirty came back far higher — the worst of them some forty thousand times the rest, and the other about a hundred times. 5.3 showed the same shape, one pair in fifteen far above the others. Nothing explains the high ones.
- So a difference has to be read against that: on two of the three scenes tried it was hundreds to tens of thousands of times the worst same-method figure, and on the third only six to eight times, which the comparison itself calls the weaker claim. In 5.3 that third scene's margin was only about one and a half to two times, so there the weaker claim is weaker still; and 5.3's smaller gaps between methods are not established as 5.3's own.
- **This long measurement is macOS and Apple's own graphics only.** The NVIDIA side rests on a floor measured once rather than measured at length, and that machine is gone (*What a render demands 7*).

Two more topics supply picture comparisons of the same kind, each against a control of exactly zero, which is what makes any of them mean anything (*Settings 19; What a render demands 6*). **Neither locates the boundary above**, and neither should be treated as if it did. One of them adds a caution of its own: one of its channel counts is not reproducible in 4.5, 5.0 or 5.1, so its last digit is not a number to build on, while the verdict it supports is not in doubt (*Settings 19*).

## What this does not cover

- Whether the files on disk are whole. Exit 0 does not mean every file is whole (*Reports 1*). Where a write did **not** fail, the line announcing a frame written does prove that file whole (*Writing 14*).
- Anything about a run that failed, was stopped, or crashed, beyond the one join above about what a stopped run's own output names. Everything else here is about a run that reported success.
- Whether the picture is right. Nothing here judges what is in the frame.
- Whether switching an add-on on at launch happens early enough for the engine argument to see it. Untested in every version (*Settings 13; The compositor 9*). A file naming the tested **missing** third-party engine falls back, as in the list above (*Add-ons 7*).
- Which card an EEVEE render used. No route to ask was found on any machine (*The machine 5*).
- **Windows, and any graphics card that is neither Apple's nor NVIDIA's.** Every question cited here is untried on Windows, and untried rather than out of reach — a Windows machine can be rented (*The machine 2; What a render demands 1, 6, 7; Images 9; Reports 6; Settings 17, 18, 19; The compositor 9*). **Every Windows machine this project has read reported itself a Server edition**, so a Windows result has to name the edition, and no desktop edition has ever been reached. One Windows number exists anywhere in this research — a stop delivered to 4.5 on a Server edition ended at 1 — and it settles nothing above (*Frames 15*).
- A sample subset's printed ending is not established. Its count is against the subset, not the scene's maximum. On untiled Cycles frames, the largest count avoids relying on a closing reset; whether every cancellation prints one is not settled. On tiled frames the count restarts within each tile, and no early-ended tiled frame was captured to separate it from a completed one (*Reports 11; Settings 16; Reports 7*).
- **Blender 5.3 anywhere but the Creator's Mac.** No Linux machine, no NVIDIA card and no machine without a screen ran it, and no build but the 5.3.0 Beta. Every 5.3 claim here is that build on that Mac, and set beside a 4.5–5.2 result taken on Linux it is not a difference between versions unless the comparison says it is.

## What the evidence could not have shown

Every claim above about Blender staying silent rests on somebody searching for words. Every claim about a card rests on two makes of graphics.

- The silences around a processor fallback are searches of both output streams for a short list of words; a line worded otherwise would have been missed (*The machine 4, 5*).
- The same holds for the filled card and for card memory: fixed word lists were searched and matched nothing (*Reports 1, 6*).
- And for 4.5's silence about a missing picture, which rests on a search for ten named words (*Images 9*). One topic ran an ordinary render first as a control, so that an empty error stream is a silence somebody looked for (*Reports 13*).
- The broken-driver result was run once, on a machine destroyed straight afterwards, and the comparison says outright it cannot be checked; 5.3 was never tried that way (*The machine 2*).
- Cards hidden from Blender while the preferences still named them was run in 4.5 alone (*The machine 4*).
- The filled card needed 16 GB cards against 197 GB of system memory, the only such machine the project had — and the loud out-of-memory ending was never reached **by a render filling the card from its own scene**, because there was that much to spill into. It was reached by the neighbouring case, where another program held the card's memory, and there Blender failed loudly and ended at 1. No run anywhere has tested the silent spill on a machine with less than that to spill into, and 5.3 never ran on a machine with a card of its own (*Reports 1, 6*).
- That no field written into a picture names a device was read back from two formats only (*What actually ran 1*).
- That one route to the graphics announces itself in ordinary progress lines and the other does not rests on one card and the two routes to it (*What actually ran 1; What a render demands 1*).
- The memory jump that hints at a card was measured on a scene of one cube, in 5.3 too (*What actually ran 1*).
- The engine fallback was measured on renders writing a few kilobytes; nothing says what a missing engine costs at a size a real job reaches (*Add-ons 7*).
- Whether a newer Blender given the older real-time engine name really renders in that engine was never captured; the acceptance rests on one run, on one machine, on a picture 64 pixels square, and 5.3 was never offered the old name (*Settings 13; What actually ran 2; What a render demands 3; Images 9*).
- **The whole of the engine-naming answer is the Creator's Mac** — nothing was rented for that question, including 4.5 aborting where the newer three and 5.3 end at 1, which is the kind of thing an operating system can cause as much as Blender (*The compositor 9*).
- **The swallowed launch instruction is the Creator's Mac alone**, along with three other results of that question. The setting names themselves are not: four other questions set them at launch on four different rented Linux machines, in all four versions, and got what they asked for. So the Mac bound is on the swallow and not on the whole question (*Settings 17*).
- **The prefilter-and-quality result needed two denoisers on one machine**, so it ran on the rented Linux machine alone; the Mac offers one denoiser and could not have posed it (*Settings 19*).
- **One result inside the same topic is bounded to a single card on a single driver**: Cycles tracing on the card under one method failed outright in 5.2.1 there and worked in the other three versions. It is not a statement about Blender 5.2 anywhere, and it is not to be dropped either — a render farm choosing a driver would want to know it can happen (*Settings 19*).
- The device difference was measured on a full-float file, and most of it could not appear in an eight-bit file at all (*What a render demands 2*).
- **The Simplify counter-case was tried in 5.2 and 5.3 only**, so nothing says whether the older three behave the same way, and it read values through Blender rather than out of the file's bytes (*What a render demands 6*).
- The whole-second rule for re-reading a changed file is one machine for each version, from ten stamp offsets — a rented Linux machine for 4.5 to 5.2 and the Mac for 5.3 — so no version was run on both (*Node setups 1*).
- That a missing dependency never moves the exit status rests on missing inputs and nothing wider (*Dependencies 12*).
- Every absent-cache test removed the whole folder or began with an empty one, so a partly-present cache could not have shown as different (*Dependencies 18*).
- The flag that would say a colour configuration is missing was read in 4.5 to 5.2 only at start-up, with no file open, and read false every time, the unreadable configuration included. In 5.3 it reads true once a file whose names were thrown away is open; whether 5.0 to 5.2 do the same was never tried (*Dependencies 15*).
- **The handler counts in that topic are from the Mac alone.** Its 5.3 counts came out as 5.2's (*What actually ran 5*).
- **The Creator's Mac is not one unchanging machine.** Its operating system and its Blender 5.2 build differ between test days; results from different days need not be results from the same setup. *Reports 1; Frames 15; What actually ran 5*
- **Every 5.3 result is one beta build on one Mac**, whose graphics are Apple's own and share the machine's memory, so nothing in 5.3 touches an NVIDIA card or a card filling on its own (*Reports 1*).

## What the passes do and do not prove

**The Render Time pass follows the scene's device setting in the tested overrides.** With the scene left on the processor setting it was written even when a launch argument made the card render; with the scene set to the card it was absent even when the processor rendered. This held in 5.3 on the Mac too. The ordinary processor and card tests fit those outcomes but do not separate the setting from the device. So presence or absence of this pass must not be read as proof of what did the work. *What actually ran 1; Images 10*

**Whether the per-pixel sample pass reliably reaches a saved file after 4.5 is not settled.** It was saved in 4.5. For 5.0 to 5.3, runs record it both present and absent, and no established condition separates those outcomes. This includes 5.3 on the Mac. A builder cannot depend on that pass without checking what the written file holds. This is separate from the sample figure among the named fields, which records the maximum asked for. *Reports 11; Images 10*

## An untouched value is not a default

A scene nobody had touched read back a render subdivision and a render child-particle figure that are **not** the documented defaults for those two settings, while a scene where every field had been set by hand to those documented defaults read back the documented values. **So a reader deciding whether a file has been edited cannot tell from the value alone.** Tried in 5.2, on two machines, and in 5.3 on the Mac (*What a render demands 6*).

The same topic supplies the other half: **the Simplify switch does reach a render started from outside Blender, in all four versions and in 5.3, and changes the picture materially** — so a file carrying it is not carrying a setting that only matters inside the interface (*What a render demands 6*).

## Every question this rests on

- **Add-ons 4** — a compiled part for the wrong computer accepted in silence
- **Add-ons 7** — a missing engine falls back and exits 0
- **Add-ons 13** — the enabled list records intent, not loading
- **Dependencies 12** — a missing dependency never stops a render
- **Dependencies 15** — a colour-space name silently thrown away
- **Dependencies 16** — a missing font, letterforms lost, exit 0
- **Dependencies 18** — different missing caches give different pictures; not every absent bake is simulated again
- **Frames 10** — device and picture on a small scene; compare pictures, not files
- **Frames 15** — what a stopped render does between the stop and the end
- **Images 2** — what a missing texture puts into the picture
- **Images 9** — magenta, black and zero for a missing picture; 4.5's silence
- **Images 6** — a picture records the colour space and never the treatment it was made under
- **Images 10** — what a finished picture holds, and what it does not
- **Node setups 1** — a changed file re-read only past a whole second
- **Reports 1** — what exit 0 is worth; the silent memory spill
- **Reports 6** — nothing printed names a card's memory
- **Reports 7** — a picture of no pixels ends at 0 with nothing written
- **Reports 11** — the sample figure a written file carries, and the one pass that carries the number reached
- **Reports 12** — the ways a run ends at 0 having rendered nothing; a refused device gives 1
- **Reports 13** — which stream a missing file's complaint uses, and 4.5's Cycles silence
- **Reports 14** — where the statistics really land, and a script's own lines waiting for the end
- **Settings 1** — launch arguments never touch the file
- **Settings 2** — a time limit applied per frame, a frame cut short still written, and a frame ended early by adaptive sampling
- **Settings 9** — a card asked for, the processor used, exit 0
- **Settings 13** — a file's engine name need not survive
- **Settings 15** — an out-of-range size corrected in silence
- **Settings 16** — what really decides the sample count
- **Settings 17** — a misspelled name swallowed in 4.5, on one machine
- **Settings 18** — a count cut to the limit in silence, with no single ceiling
- **Settings 19** — a substituted denoiser, and picture comparisons against a control of zero
- **The compositor 9** — the engine named from 5.0, and nowhere in 4.5
- **The machine 2** — the broken and mismatched driver
- **The machine 4** — four silences that all end at 0
- **The machine 5** — how a card is chosen; EEVEE's card unknowable
- **What a render demands 1** — no printed line names the device that rendered
- **What a render demands 2** — the device changes the picture, and how far that reaches
- **What a render demands 3** — arguments and preferences override the file
- **What a render demands 4** — which of the demanding settings belong to one engine rather than to every engine
- **What a render demands 5** — 5.0 to 5.2 rewrite a 4.5 file's tiling
- **What a render demands 6** — the Simplify switch reaches a render from outside; an untouched value is not a default
- **What a render demands 7** — what two runs of the same method come out at, and a refused method failing loudly
- **What actually ran 1** — where the device that ran can be named, and where it cannot
- **What actually ran 2** — what says which engine produced a frame
- **What actually ran 3** — nothing records which add-ons were loaded
- **What actually ran 4** — what a request Blender cannot meet does and reports
- **What actually ran 5** — the three places an answer can come from
- **What actually ran 6** — at what moment each becomes knowable
- **Writing 13** — a launch-time script can read a setting back and refuse to render
- **Writing 14** — the line announcing a frame written proves that file whole

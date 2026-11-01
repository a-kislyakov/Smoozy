# What a file can be asked before it renders

What a `.blend` can be made to say about the render it will produce, before that render is started — and where it stops answering.

> **This joins answers that live in several topics.** Every claim below names the comparison
> questions it rests on; open those for the exact words, the exact messages, and what each
> version does. Where this document and a comparison disagree, the comparison is right.

*All four versions*, *every version* and *any version* mean Blender 4.5 to 5.2 here, and *the newer three* means 5.0 to 5.2. Blender 5.3 is named wherever it is meant, and every 5.3 result is the 5.3.0 Beta built 2026-10-16 on the Creator's Mac alone — so no Linux machine ever read a file in 5.3. *Reading a file 1, Dependencies 20*

## The short answer

- Reading a file means running a Blender, and it should be the version that will render it: a value can come back different from the value saved, and a declared default and an engine's own name both move at 5.0. *Reading a file 1, 3*, *What a render demands 5*, *Settings 13*
- Scenes, frame range and step, view layers and whether each is on at a named frame, sample counts, engine, processor-or-card, the denoising settings and the Simplify limits all read out, with no window. *Reading a file 3*, *Frames 8*, *Settings 4, 9, 12, 19*, *What a render demands 6*
- A count read out of a file is a ceiling under Cycles, a floor as much as a target under EEVEE — which in 5.3 can also be stopped short by a time limit of its own — and not a number at all under Workbench. It belongs to one frame, one engine and one scene; each count is corrected against **its own** limits, so one number means two different renders under two engines; and the number that renders may be a different one stored elsewhere in the same file. *Settings 4, 5, 16, 18*, *The compositor 9*
- Places to write read out one at a time; no complete list is established. A count of destinations, and a count of files, is assembly work over the scene's own output, each compositor node, node groups, in 5.3 the compositor's effects, and **any scene brought in through the video sequencer, which writes its own files by its own rules**. *Reading a file 4, 5*, *Dependencies 17*, *File output nodes 1, 6, 8, 13, 16, 17*, *The compositor 1*
- Three things no file reliably says: which add-ons it needs, which card will do the work, and — on a machine that has not got the denoiser it names — which denoiser it asked for. *Add-ons 1*, *Settings 9*, *Settings 19*

## The reading is only as good as the Blender that takes it

Reading a `.blend` means opening it in Blender, as far as anything here settles: what a tool outside Blender can read is not settled. The file's layout is not described in the sources searched, and the named outside tool was not tried. Peeking at a file without opening it hands back that file's own local data — plus the version from 5.0, and its list of linked files from 5.2. *Reading a file 1*, *What a render demands 5*

So a reading means starting a Blender, which may be one with no window. *Reading a file 5*, *What a render demands 5*, *What a render demands 6* Doing that beside a render already running on the same computer disturbed neither: the second Blender read the file and quit cleanly while the render went on and wrote every frame, with no error and no warning. 5.3 was not tried that way. How many Blenders one computer will carry at once is not settled, and no test can reach it. *Reading a file 2*

**Read the file with the version that will render it.** Five separate things move, and each changes the answer rather than the wording:

- A value can come back different from the value that was saved. A file saved by 4.5 with its tiling switched off and a size set reads back, in the newer three, as switched on at a size nobody chose. 5.3 was not given such a file. *What a render demands 5*
- A declared default moves: the switch saying whether a view layer renders is declared off in 4.5 and on from 5.0. **That is the declared value and not what a layer carries** — in all five versions a real layer, fresh or shipped, reads on. *Reading a file 3*, *Settings 17*
- The real-time engine's identifier changes at 5.0, and the spelling that works by every route is the reading version's own. *Reading a file 3*, *Settings 13*, *The compositor 9*
- The engines a file can name are whatever the reading Blender has registered. With the Cycles add-on switched off, a Cycles file reads back under the other engine's name and its Cycles settings cannot be read at all. *Settings 13*, *What a render demands 5*, *Add-ons 1*
- **The shape of what is recorded moves too.** A compositor output node kept one recorded place in 4.5 and a folder and a file name from 5.0, and a 4.5 file carried into the newer shape by 5.0 to 5.2 arrives with the file-name half empty. *Dependencies 20*, *Reading a file 9*

**A wrong Blender does not refuse the file — it opens it and complains, and one version complains a great deal more than the others.** A file saved by 5.2, opened with no window by each of the older three, printed a warning about losing things and then rendered, saved the picture and ended cleanly; two machines agree on every part of that. **Blender 4.5 alone adds a run of further warnings and a line reporting memory it did not free, and still succeeds** — so anything reading Blender's output for errors must not read those as a failure. On Linux, 4.5 opening that file also reported that the engine the newer file names is not there, fell back to its own and rendered anyway, which is the engine-name change above met from the other side. Going the other way, 5.2 opened a 4.5 file with no warning at all. Between 5.2 and 5.3 it goes the same way on the Mac: 5.2.2 given a 5.3 file warns and renders, and 5.3 opens a 5.2.2 file saying nothing; what 5.3 does with a file newer than itself is untried. Each version's own release notes name its own breaks — what a file crossing that boundary loses, and where a picture will render differently afterwards — and reading with the version that will render is what keeps a job from meeting them. *Settings 10*

**And the reading is not the only thing the open changes.** Simply opening a file re-spells the separators in the places it records to the ones the machine itself uses — one way on macOS and Linux, the other way and much more widely on Windows — and a plain save then writes that in. **Five places are left alone on Windows**, among them the compositor output node's folder and three cache and bake folders; the render output path is not one of them, so "places written to are left alone" is not the rule. So the text a reading hands back is the text as this machine spells it, not necessarily the text the artist's machine stored. *Dependencies 20*

An installed Blender does answer what it is, by five routes that work in all five versions, and whether it is a long-term-support build can be read off the version string; the 5.3.0 Beta marks itself as a beta on that string too. *Builds 3*

## What one read settles

- **Scenes.** A file holds several, each with its own compositor. Told nothing, a render does the scene that was showing when the file was saved. *Settings 12*, *The compositor 3*
- **Frames.** A start, an end and a step, both ends counted. A stepped run walks forward and never reaches back for the end frame, so the number of frames follows from the three numbers. A step of nought writes the first frame once; an end below the start writes nothing at all. *Frames 8* **That is a count of frames rendered on a plain animation run, and not a count of files.** A run can be handed a list of frames instead, and the file's own three numbers say nothing at all about what such a list writes — the step included, which has no effect on a list on the one machine each version was asked on. A repeated item in a list renders twice over one name. *Frames 7*
- **View layers.** Whether each renders at a named frame, including where that has been keyframed — the switch is read again at every frame, so a reading is a reading of that frame. The switch that would limit a render to one layer is ignored when the render is started from the command line; the switch that really does govern a layer is a different one, and it is the one worth reading. With every layer switched off, 5.3 renders nothing and still ends at success; that was never tried in an earlier version. *Reading a file 3*, *Settings 1, 16, 17*
- **Sample counts.** One per engine on the scene, plus a view layer's own override, all present at once and all surviving a change of engine. A count means nothing until the engine is known — and the scene's three are a subset of what is there: twelve counts sit across the engines and the view layer, eleven of them numbers with their own range and starting value, and the twelfth a fixed list of choices rather than a number. *Settings 3, 4, 18*
- **Denoising.** Five places' worth of it in 4.5 to 5.2 and six in 5.3: the scene's own settings for the final render and for the viewport, the Cycles view layer's own two — four in 5.2, five in 5.3 — the engine-independent settings governing a denoise node, the other real-time engine's five, and in 5.3 alone two more on that engine's view layer. **Which denoiser a file names is recorded and is not always what reads back**; on a machine that has not got it the setting comes back as empty text. *Settings 19*
- **The Simplify limits.** One switch and a pair of values for each limit — subdivision, child particles and texture size, seven values in 4.5, 5.0 and 5.1 and nine in 5.2 and 5.3, which add a texture resolution scale. All of them read back with no window by the documented route and survive a save, in 4.5 to 5.2 a crossing between operating systems as well, and the switch reaches a render started from outside in all five versions. *What a render demands 6*
- **Processor or graphics card.** Cycles alone records it, as that two-way choice and nothing finer. Which technology and which card are the machine's business and live in its preferences, so **no file says where a render will run.** *Settings 9*, *What a render demands 5*, *What a render demands 7*
- **Resolution percentage.** Whole numbers from 1 to 32767. A value outside that is pulled back the instant it is set, with nothing said; a value inside it can still ask for a picture too small to make, and that is refused. *Settings 14, 15*
- **Which add-ons the file needs — not settled.** No reliable way of reading a file for that is established. The nearest thing that does live in the file is a list governing what the interface shows, not what the render requires. One trace survives in 4.5 and in no later version: read by a Blender with Cycles switched off, a file saved with Cycles still leaves the add-on's own name listed on the scene, and from 5.0 nothing is listed at all. *Add-ons 1*
- **Which engine a view layer uses — there is no such thing.** A view layer carries no engine setting anywhere; the engine lives on a scene and nowhere a compositor node or a view layer can reach. *The compositor 9*

**A caution on names.** Five of these settings have a near-miss neighbour under a name that looks right and answers a different question — a second sample count for the viewport under each of two engines, a second anti-aliasing count, the choice governing how a view layer's own count is used, and the layer switch that only governs a render started inside the window. Every one of those neighbours exists in all five versions under the same name, with the same label, description and starting value, so reading the wrong one is not a version's fault. **That was established on the Creator's Mac and on no other machine.** *Settings 17*

## A count is a ceiling in one engine and a floor in another, and it belongs to one frame and one scene

A sample count read out of a file is not a number of samples. In Cycles five things stand between it and what a pixel gets: sampling that stops early, on unless switched off; a time limit; a subset; a view layer's own count; and the rule deciding what to do with that count. That last pair is the one that displaces a number rather than trimming it: a view layer's own count stands in place of the scene's, is capped by it, or is thrown away, according to a scene-level rule with three settings, and the first of the three is what a file arrives with. In EEVEE the number is a floor as much as a target — two features push it above what was asked — and in 5.3 a time limit of its own can stop it short. Workbench has no such number at all — what it has is a fixed list of anti-aliasing choices rather than a number. *Settings 16*, *Settings 4*, *Settings 18*

**And each count has its own limits, which is the trap.** A number above one engine's ceiling and inside another's is cut down silently by the first and taken whole by the second, so the same figure read out of two files, or handed to two engines, is two entirely different amounts of work. Nothing announces the correction, but two places carry the corrected number: the setting itself, read straight back, and the count Blender prints while it renders. **The setting read straight back shows the accepted value; the printed count needs more care.** A sample subset or a view layer's own number replaces the printed count, and on the route where a script calls the render rather than the command line 5.0 to 5.3 print no progress at all. What a file that already holds an out-of-range count reads back when it is opened, rather than assigned, is not settled. *Settings 18*, *Settings 16*

**A count is also a count for the scene it sits on and no other.** Where the picture comes from a second scene pulled in through the compositor, that scene renders at its own count, under its own engine, with its own layer rule deciding its own view layer's override; the containing scene's number does not reach it. So a count read off the scene that will be rendered says nothing about the work a frame is when its picture arrives through a node. *The compositor 9*

**And a count belongs to an engine that may not be there.** With the add-on carrying the file's engine switched off, the file reads back under another engine's name and renders at that engine's own count rather than the one the file holds. *Settings 13*, *Add-ons 1*

Where the versions part: from 5.0 EEVEE obeys a view layer's own count and 4.5 ignores it, so the same file gives two different answers about how much work a frame is. *Settings 16*, *The compositor 9*

And the reading is of one frame. Cycles' and EEVEE's counts both take a keyframe, and a Cycles render started from outside reads the value again at every frame; the view-layer switch behaves the same way, and in 5.3 a keyed EEVEE count and a keyed view-layer count were rendered frame by frame too. *Settings 5*, *Reading a file 3*

## Every place it will write, and how many files that comes to

The places read out one at a time — the scene's output path, a compositor output node's folder and its outputs' names, and the finished path a named frame would land at. A complete list of the kinds of place a file can be set up to write to is not established. *Reading a file 4*

Nor do the listings and the sweep that makes paths relative gather them: they reach neither the render output path, nor a compositor output node's folder, nor a point cache or a geometry-nodes bake folder; of the cache places tried, an ocean modifier's is the one they do reach. **The two tested lists agree on those omissions.** The sweep that makes paths absolute was handed those omitted paths already written in full, so whether it reaches them in relative form is not settled. In 5.3 the geometry-nodes folder belonged to a bare modifier, with no bake; whether the tools reach an actual bake is not settled. One other route does reach them: a walk over Blender's own marks on its settings found six of the seven places the sweeps missed, leaving only a path held in a custom property. No tested means reaches everything, in any version, 5.3 included. *Reading a file 5*, *Dependencies 17*

**Two of the walking routes do not even return the same collection.** Asked of one file, one returns fewer strings than the other. Most of the extras are repeated copies of places both routes report — but not all of them: where a file links a library that points at a picture of its own, one route reaches through the library and the other does not, so there is at least one kind of recorded place only one of them can see. That was found on Windows. A count taken from either is a count of that route's answer. *Dependencies 20*, *Reading a file 5*

So the count is still assembled property by property, with nothing to confirm the list is whole. What has to go into it:

- **The scene's own output** — one piece of text holding the folder and the front of the name, with the ending decided separately by a switch and the chosen format. Text past 1023 characters is cut in silence. *Writing 2*
- **Whether that write happens at all** — from 5.2 a switch turns it off, and it beats even a path handed in at launch, while a compositor output node in the same tree goes on writing. No earlier version has it. With it off and no such node, 5.3 writes nothing at all and still ends at success. *The compositor 2*, *Reading a file 4*
- **Whether the compositor runs at all** — three conditions for the scene's own file, a fourth in 4.5, and in 5.3 the compositor's effects, each of which takes part only while it is switched on for render. The enabled effects run in list order, each taking the previous result; the one marked active does not change that order. **The Compositing switch stops a compositor output node. In 4.5 the old Use Nodes switch does too; in 5.3 so does switching off for render the effect whose tree holds the node.** In 4.5 to 5.2 the compositing switch is read once for the whole run and changing it mid-render never reaches the node; in 5.3 on the Mac a change made by a script at each frame reaches it a frame late. The live final output node decides whether the compositor's result reaches the scene's own file: with no live final output node in the tree the node goes on writing every frame, and what lands in the scene's own file is the raw render in the 4.5 to 5.2 runs and a transparent black frame in 5.3 and 5.2.2 on the Mac, which is not settled as a change in Blender. *The compositor 1*, *File output nodes 13, 14*
- **What feeds a node** — it must receive a picture. In 5.3 a node fed from the compositor's Group Input also writes, and that is the picture source in the default tree. *File output nodes 12*, *The compositor 1*
- **Each compositor output node** — one folder for the node and none per output; the outputs are its inputs, and at least two thousand were accepted. *File output nodes 1, 3*
- **A format read off a node is not always the format an output writes.** The node holds a fallback format and each output can hold one of its own; a switch on the output decides which of the two is used, and **that switch's name and its sense both changed at 5.0**, reading one way in the oldest version and the opposite way from 5.0, 5.3 included. In image mode, a reading that takes the node's format alone is wrong about an output using its own. In 5.3 on the Mac, multi-layer mode still gathered an output into the one file even when that output asked for a picture format of its own. *File output nodes 2, 6*
- **How many files a node makes is not how many outputs it has.** In one mode every output becomes a layer of a single file; in the other, one file each. The mode follows the format setting in 4.5 and a separate media setting from 5.0 — and an untouched node starts in the one-file mode from 5.0 and the file-per-output mode in 4.5. *File output nodes 1, 6*, *Reading a file 4*
- **An output whose link is cut or silenced writes nothing**, and silencing the node stops it entirely, so the list of outputs overstates the files unless the links are read too. *File output nodes 8, 12*
- **A node inside a node group writes from there, at every depth tried**, and silencing the group from outside stops it. A group used twice leaves one file; in 5.3 the node was seen writing it once for each use, over the same name. *File output nodes 7*
- **Folders can hide inside an output's name**, and the versions do three different things with them; in 4.5 a name can put the file outside the node's own folder altogether. In 5.3 a folder can hide in the node's own file name as well. Two outputs whose names differ only in which separator they use can reach one path, leaving one file without a warning. *File output nodes 4*
- **Two destinations can land on one file.** Two outputs on one node cannot share a name and the second is renamed. Two separate nodes aimed at one name give one file, holding the first-built node's picture, silently; in 5.3, two nodes gathering their outputs into one file each, aimed at one name, were merged into that file as layers. The scene's own settings for replacing frames and reserving names do not reach a node. *File output nodes 10, 6*
- **Another scene's destinations may be in play, and the sequencer route is the loud one.** A compositor node reading a second scene renders that scene fresh but does not run its compositor, so it writes nothing of its own. A sequencer strip holding a scene is different and is set out below. *Settings 8*, *The compositor 4*, *File output nodes 16*
- **Frame numbering differs.** A node's files always carry the frame number in 4.5, and from 5.0 only on an animation render — and a render started from the command line is always an animation render. *File output nodes 13*
- **One switch can gate a node frame by frame, by a keyframe or a driver, in 4.5 and again in 5.3** — the old compositor switch in 4.5, and in 5.3 a compositor effect's switch for render — so the number of files can differ between frames of one run. In 5.0 to 5.2 none of the tested switches can. A per-frame script reaches the node's silencing, its link and its group in all five, but one frame late. In 5.3 an effect's render switch also lands one frame late when changed by a script, though a keyframe or driver lands on its own frame. *File output nodes 14*

### A scene brought in through the video sequencer

This is a whole source of files that a reading of the containing scene alone will miss, and it follows its own rules.

- **The brought-in scene's own output node writes**, once for every frame of the strip that is rendered, alongside the containing scene's own render. **It is the brought-in scene's compositing switch that governs; the containing scene's has no effect at all.** In 5.3 switching that scene's effect off for render also stops its node. The containing scene's own output node writes nothing. *The compositor 4*, *File output nodes 16*
- **The strip must be in play.** The containing scene's sequencer must be on, the strip unmuted, its input set to Camera, and the current frame inside the strip. Otherwise the brought-in node writes nothing. *File output nodes 16*
- **The count follows the strip, not the scene.** Trimming the strip cuts the writes; stretching it past the scene's own length writes more times over the same names; a frame step cuts them; **two strips of one scene write twice for every frame, over the same names, so the second lands on top of the first.** In the tested two-level chain the innermost scene is not reached: a strip renders its scene through the camera, without following that scene's own sequencer. *File output nodes 16*
- **These counts assume a fresh Blender for each render.** In 5.3 and the 5.2.2 control on the Mac, repeated renders in one session kept the first composited picture despite later switch changes, and the brought-in node did not write again. What causes that, and whether it holds in the earlier three, is not settled. *The compositor 4*
- **Every part of the name that could come from either scene comes from the brought-in one** — the frame number, the scene's name, its camera, its rate, its size. The strip's own camera override does not reach the name in 5.0 to 5.3; 4.5 has no camera variable. If the scene is linked from another file, the blend-file name and the relative prefix still mean the open file, not the library. The node's own format decides the ending — which of its two formats that is being the switch above. *File output nodes 17, File output nodes 2*
- **Whether a frame number appears at all splits at 5.0.** In 4.5 it is always put in, so five frames leave five numbered files; from 5.0, 5.3 included, it is not, so **the same five frames leave one file written over five times**. Writing the number into the name by hand puts it back in all five. *File output nodes 17*
- **Which naming variables a name may use also splits.** 4.5 takes four of the seven tried and writes nothing at all for a name asking for any of the other three; 5.0 to 5.3 take all seven. *File output nodes 17*
- In 4.5 to 5.2 both of those settle macOS and Linux and nothing else, and the window half of each rests on one rented Linux machine; 5.3's rest on the Mac alone. *File output nodes 16, 17*

Three further things a count of files depends on:

- **Names can be predicted, with two known exceptions.** Asking the file where a named frame will land gives the name the render writes — templates worked in, frame number placed, padding and ending and all — for a command-line frame, a command-line animation and an animation started from a script alike. The still started from a script is the exception and it is the worse one: the frame-number padding is left standing in the name, and where the name carries no padding **no frame number is put in at all**, so every frame written that way lands on one name and replaces the one before. The other exception is asking for a frame no render would use, which answers a name no render writes. A broken template makes the question fail with the same failure that stops the render, so the question doubles as a check. In 5.3, project variables add another bound: whether their paths can be worked out from the file alone is not settled. *Dependencies 17*, *Reading a file 4*, *Writing 8*, *Writing 3*
- **The two format lists are two lists.** What a scene can be written as and what a compositor output node offers are not the same set, they part in different places between versions, and no version's node writes video. *Images 3*, *File output nodes 9, 11*
- **View layers multiply work, not files. Views do multiply files.** One frame with two layers active and an ordinary picture format writes one file, holding the first layer's pixels; written as the multi-layer format instead, both layers stay in that one file, and switching a layer off from outside takes its channels out of it. **That does not promise every remaining-layer setup renders**: with only the second layer on, the Frames 9 scene crashed in 5.3 and in its 5.2.2 Mac control; what caused it is not settled. The name is the path and the frame alone **only where the scene has one view**: a scene with two views writes two files as the settings come, each carrying its view's own ending after the frame number, and one file only when it is told to put both views in one. *Frames 9*, *Images 1, 5*, *Writing 8*, *Settings 17*

Last, the size read out of the file is not always the size written: a render region with cropping, a stereo layout put in one file, and a compositor output node's own input each decide it. *The compositor 6*

## What this does not cover

- What can be imposed on a render at the moment it starts, and what an imposed value does **not** set aside.
- The outside files a `.blend` points at, how each location is written down, and where a relative one is measured from — except that opening the file re-spells their separators, which is here because it changes what a reading hands back. *Dependencies 20*
- Whether the render then does what the file said. That becomes knowable only while it runs or after it ends.
- The naming rules themselves, and what a finished or stopped run actually leaves on disk.
- What Blender prints while it opens a file, and how that changed at 5.0.
- Anything about a file saved by a Blender older than 4.5, or by anything that is not Blender.
- Blender 5.3 on anything but the Creator's Mac. Every 5.3 result here is the 5.3.0 Beta on that Mac, so every Linux and Windows answer here is an answer for 4.5 to 5.2. *Dependencies 20*, *Reading a file 1*
- Windows, with one narrow exception. One question behind this document was run on a Windows machine — what a save records — and **that machine reported itself Windows Server 2025 Standard**, so it says nothing about a desktop one. Everything else here is untried on Windows rather than out of reach. *Dependencies 20*

## What the evidence could not have shown otherwise

- The declared default of the scene's output path and a fresh scene's value are different things. Reading a file 4 did not test the default; Writing 2 read it and found a fresh scene carrying its startup file's path instead. A declared default cannot stand in for reading the file. *Reading a file 4*, *Writing 2*
- **The node's panel of finished paths was read on a rented Linux machine** in 5.0, 5.1 and 5.2, and on the Mac in 5.3, and it shows one row per output holding the name without a frame number — the name that route actually writes. Blender 4.5 has no such panel to read. So it is not a macOS-only reading; what it does not cover is 4.5, and it was read for the sequencer route rather than for a scene rendered on its own. **One row per output is what the panel holds for a node writing one file per output**; the one node read in the gathered mode held a single row for the whole node. Nothing has asked what the panel holds as a general rule. *File output nodes 17*, *Reading a file 4*
- That the file's layout is described nowhere comes from searching for four named things; a description written under other words would have been missed. *Reading a file 1*
- Nothing outside Blender but a plain decompression tool was ever pointed at a `.blend`, in any version, so what a tool that reads its contents could get is untried rather than answered. *Reading a file 1*, *Add-ons 1*
- Reading a file beside a running render rests on sixty-frame runs on two machines, and nothing measured time or memory, so a slowdown could not have appeared. *Reading a file 2*
- The only settings watched across versions for a rewrite on load are tiling, its size and the sample count, so a newer Blender rewriting anything else as it loads could not have shown. *What a render demands 5*
- A pass that can rewrite a stored sample count as a file loads is on record, and it is not dormant — across the one version boundary this project can cross it rewrites the tiling settings and leaves the count exactly as saved. What could not be tried at all is the older boundary the pass was written for: it needs a file from a Blender older than any this project has. *Settings 16*, *What a render demands 5*
- In 4.5 to 5.2 only Cycles' count was watched changing from frame to frame; EEVEE's was shown to take a keyframe and never watched across a render, so a version reading it once at the start could not have shown. In 5.3 EEVEE's was watched too. *Settings 5*
- The percentage reports itself as not animatable, but no driver was attached and no keyframe set anywhere, so a value carried outside its range could not have appeared. *Settings 14*
- Every file behind the active-scene answer was built and saved by a Blender with no window, so a file saved from the interface could not have shown a different answer. *Settings 12*
- That the three older versions cannot switch the scene's own write off rests on asking each of them for the setting by name; a control under another name would have been missed. *The compositor 2*
- "At least two thousand outputs" is the number the script added, so no higher ceiling could have shown itself. *File output nodes 1*
- That nothing can be keyframed to gate a node in 5.0 to 5.2 rests on six named switches; anything outside that list could not have shown. *File output nodes 14*
- The three socket kinds a node accepts were shown to write files by listing and sizing them, never by reading them back, in 4.5 to 5.2; 5.3's were read back on the Mac. *File output nodes 12*
- Both format lists rest on two builds of each of 4.5 to 5.2, both fetched from Blender's own download server, and on one 5.3.0 Beta, so a build put together without one of the optional formats could not have shown. *Images 3*, *File output nodes 9*
- No chain of scenes deeper than two was tried through the compositor, and no version settles how the two scenes' frame numbers line up there. Through the sequencer a two-level chain **was** tried, and its deeper scene was not reached. *Settings 8*, *File output nodes 16*
- Every view-layer test on both machines was driven from outside Blender's window, so what a render started inside one writes could not have appeared. *Frames 9*
- The whole several-views half of the multi-picture answer ran on one Linux machine in 4.5 to 5.2 and on the Mac in 5.3 alone, so no version has it on both. *Images 5*
- The two extra pixels a node wrote from 5.1 came from one scaled size. In 4.5 to 5.2 no other size was ever fed to a node; in 5.3 two other sizes came out exact. *The compositor 6*
- The long-term-support letters were searched for across a whole build listing in 5.2 alone. *Builds 3*
- **Settings 17's own runs, including the near-miss comparisons, were on the Creator's Mac.** Other questions exercised the resolution percentage, Cycles' count, the layer switch and its count on Linux in 4.5 to 5.2. EEVEE's and Workbench's count names were not exercised there, nor any of the six in 5.3. *Settings 17*, *Settings 1, 16*
- **The time-limit tests used Cycles on the processor and one view layer.** Whether the limit works the same on a card, or is spent per layer, is not settled. *Settings 16*
- **The ranges and starting values rest on two machines**, and they cannot show one ceiling for anything: each count is corrected against its own. *Settings 18*
- **What the prefilter and quality settings do under another denoiser needed two denoisers on one machine**, so it ran on the rented Linux machine alone, and not in 5.3; the Mac offers one and could not have posed it. The channel counts behind the compositor-denoise result are not reproducible in three of the four versions, so their last digits are not numbers to build on. *Settings 19*
- **That an untouched scene does not read back at the documented Simplify defaults was tried in 5.2, on two machines, and in 5.3 on the Mac**, so nothing says whether the older three behave the same way. And whether any of these values is written into the file at all, or rebuilt from a default when it opens, is answered by no version and by no test — every reading went through Blender. *What a render demands 6*
- **The extra denoising settings record choices, not established picture effects.** What 5.2's extra Cycles layer settings and 5.3's added settings do to a picture is untested. The compositor quality tests used a new Denoise node following the scene's quality; other choices were not tried. *Settings 19*
- **A subdivision limit does not govern every kind of subdivision.** In 5.3 on the Mac it changed a Multiresolution modifier's picture and left adaptive subdivision unchanged. That pair was not tried in an earlier version. *What a render demands 6*
- **Scenes referenced inside node groups have a 5.3 release-note change**, but its reach is not settled by a test. It does not establish that such a scene's own compositor runs. *Settings 8*, *The compositor 4*
- **The compositor 9's own runs were on the Creator's Mac.** The EEVEE layer-override difference was also exercised on Linux in 4.5 to 5.2 by Settings 16; the rest keeps the Mac bound. *The compositor 9*
- **Whether the two ways of listing a file's recorded places ever disagree about spelling is not settled**: three runs on the Creator's Mac in 4.5 to 5.2, two saying they agree and one saying they do not; 5.3's one run there agrees with the two. *Dependencies 20*

## Every question this rests on

- **Add-ons 1** — no record of needed add-ons; what a Blender without one reports.
- **Builds 3** — asking an installed Blender what it is, and how the 5.3.0 Beta marks its stage.
- **Dependencies 17** — path sweeps miss write places but reach an ocean cache; predicted names match.
- **Dependencies 20** — what a plain save records; the separator re-spelled at open; the compositor node's recorded place carried into the newer shape; the two listing routes' different collections.
- **File output nodes 1** — outputs, their number, the starting format.
- **File output nodes 2** — which settings belong to an output and which to the node; the switch deciding whose format writes the ending.
- **File output nodes 3** — no folder per output.
- **File output nodes 4** — folders hidden inside an output's name, in 5.3 inside the node's file name too, and two separator spellings reaching one path.
- **File output nodes 6** — one gathered file against one per output; multi-layer mode beating an output's own format, and two gathering nodes merged into one file, in 5.3.
- **File output nodes 7** — a node inside a node group, and a group used twice.
- **File output nodes 8** — silencing a node, or one output's link.
- **File output nodes 9** — the node's own format list.
- **File output nodes 10** — two destinations aimed at one name.
- **File output nodes 11** — no version writes video from a node.
- **File output nodes 12** — it must be fed a picture.
- **File output nodes 13** — when it writes, and frame numbering; in 5.3, a node writing only from an effect switched on for render.
- **File output nodes 14** — what can be gated frame by frame, in 4.5 and again in 5.3; the compositing switch reached mid-render in 5.3 alone.
- **File output nodes 16** — a scene brought in through the sequencer writes its own node's files, and what governs and counts them.
- **File output nodes 17** — what those files are called, the frame-number split, and the node's panel of finished paths.
- **Frames 7** — a list of frames instead of a range, and what a repeat in it does.
- **Frames 8** — start, end, step, and the frame count.
- **Frames 9** — several view layers, one file.
- **Images 1** — passes, and which picture is the finished one.
- **Images 3** — the scene's format list per version.
- **Images 5** — several pictures in one file.
- **Reading a file 1** — what a file gives up without Blender.
- **Reading a file 2** — reading beside a running render.
- **Reading a file 3** — view layers per frame; defaults and names by version.
- **Reading a file 4** — the write places that can be read; no complete list.
- **Reading a file 5** — which listing means reach which recorded locations.
- **Reading a file 9** — the compositor output node's own path setting renamed at 5.0.
- **Settings 1** — the layer switch ignored from outside; 5.3 rendering nothing at success with every layer off.
- **Settings 3** — every engine's settings present at once.
- **Settings 4** — three sample counts, one per engine.
- **Settings 5** — a count keyframed and read at each frame.
- **Settings 8** — one scene taking in another's render.
- **Settings 9** — processor or card, and nothing finer.
- **Settings 10** — an older Blender opening a newer file: the warning it prints, the extra 4.5 adds and still succeeds through, the 5.2.2–5.3 crossing on the Mac, and each version's own recorded breaks.
- **Settings 12** — which scene a render does when untold.
- **Settings 13** — the engine list and the engine's own name; what a Blender without the engine renders instead, and at whose count.
- **Settings 14** — the resolution percentage's range.
- **Settings 15** — an out-of-range value corrected in silence.
- **Settings 16** — the count as a ceiling, and what moves it; the real-time engine's time limit in 5.3.
- **Settings 17** — the names these settings go by; the near-miss neighbours; the declared default against what a layer carries.
- **Settings 18** — the twelve counts a file carries, each with its own range and starting value; the silent correction, and the two places the corrected number can be read.
- **Settings 19** — the denoising settings a scene and a view layer carry, by version, and the denoiser name that does not read back.
- **The compositor 1** — what makes a compositor run at all, and in 5.3 its effects, their order and the Group Input picture source.
- **The compositor 2** — switching the scene's own write off.
- **The compositor 3** — a compositor belongs to a scene.
- **The compositor 4** — whether another scene's compositor runs; in 5.3 its effect switch; repeated-session carry-over in 5.3 and 5.2.2; nested scene references untested.
- **The compositor 6** — what else decides the written size.
- **The compositor 9** — whose engine, whose sample count and whose layer rule a scene pulled in through a node renders under; the node's two settings; no engine setting on a view layer.
- **What a render demands 5** — reading values out; what preferences hold.
- **What a render demands 6** — the Simplify switch and its limits, and what an untouched scene reads back as.
- **What a render demands 7** — no file names the technology or the card.
- **Writing 2** — how a place to write is recorded.
- **Writing 3** — what marks in a path stand for.
- **Writing 8** — a recorded place becoming a file name.

These touch nothing in it: **Frames 15**, **Frames 16** and **Frames 17** (what happens between a stop and an end, and what a stop leaves where a frame or a video was being written), **Reports 12**, **13** and **14** (the ending status of a run, which stream a missing file's complaint lands on, and when a printed line reaches a reader), **Builds 10** (what a machine needs before a Blender package can be unpacked), **The machine 8** (what a render does where the screen is drawn by the processor or in software), and **Writing 13**, **14**, **15** and **16** (what a launch-time script sees of a setting, one frame's writes against the next frame's, what a video holds while it is written, and when a render reports a file written).

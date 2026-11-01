# The map

The way into this project's Blender research: what has been asked, where each answer lives, and which document to open for the problem in front of you.

> **Start here, and go where this sends you.** Where an answer spans topics it has already been
> joined and checked, and the joined answer is what to read. Open a comparison question only
> where this map, a joined answer, or your own document's sources file names it — that question,
> and no more of the file than that. `Findings/`, `Questions/`, the question register,
> `Coverage.md` and everything under `Audits/` are outside every downstream reading, at every
> stage. **Nobody downstream assembles an answer out of several topics themselves**: the one time
> this project did, a claim that was true of its own test conditions and false as a general rule
> reached a spec.

## The ten joined answers

In the order a job moves, then the three that sit across all of it.

**1. [What a file can be asked before it renders](What%20a%20file%20can%20be%20asked%20before%20it%20renders.md)**
Covers: what a `.blend` can be made to say about the render it will produce, before that render is started.
Stops at: what can be imposed at the moment the render starts — that is *What can be set from outside* — and at the outside files the `.blend` points at, which is *What has to travel with a file*.

**2. [What has to travel with a file](What%20has%20to%20travel%20with%20a%20file.md)**
Covers: what a `.blend` needs beside it on a machine that has never seen it, what can be found before it is sent, and what has to be repointed once it arrives.
Stops at: where the render then writes, which is *Where a render's files go*; and at network folders, and at Windows beyond one question, which the questions behind it did not reach.

**3. [What can be set from outside](What%20can%20be%20set%20from%20outside.md)**
Covers: what a render can be handed at the moment it starts, without changing the artist's file, and what it will not take.
Stops at: preparing the machine, which is *Running Blender with nobody there*; and at what the render then prints and writes, which are *While a render runs* and *Where a render's files go*.

**4. [Where a render's files go](Where%20a%20render's%20files%20go.md)**
Covers: where a render's files land, what they are called, how many there are, and which one of them is the finished picture.
Stops at: what is left at any of those paths when a render fails or is stopped part-way, which is *When a render ends*.

**5. [While a render runs](While%20a%20render%20runs.md)**
Covers: what can be read out of a render while it is alive, and what changes what there is to read.
Stops at: how a render ends and what its ending is worth, which is *When a render ends*; and at what did the work, which is *What a render really did*.

**6. [When a render ends](When%20a%20render%20ends.md)**
Covers: what can be told about whether a render worked, and what is on disk, once it has finished, been stopped, or had the machine go down under it.
Stops at: whether a run that succeeded did what was asked, which is *What a render really did*; and at which files it was going to write, which is *Where a render's files go*.

**7. [What a render really did](What%20a%20render%20really%20did.md)**
Covers: a render finished and reported success — what that proves, what it does not, and where the real answer can be read back from.
Stops at: anything about a run that failed, was stopped or crashed, and whether the files on disk are whole — both *When a render ends*.

**8. [Running Blender with nobody there](Running%20Blender%20with%20nobody%20there.md)**
Covers: what changes when Blender runs on a machine with no screen, no display server, and nobody sitting at it. This is the ordinary condition for a rented machine, so it sits under most of the documents above.
Stops at: what a render prints line by line and what its ending is worth — only the screen's part in either is here.

**9. [Two renders in one folder](Two%20renders%20in%20one%20folder.md)**
Covers: whether more than one Blender can share one output folder, and what is in the folder at the end.
Stops at: what a stop or a kill leaves at the exact moment a frame is being written, which is *When a render ends*.

**10. [What the research cannot say](What%20the%20research%20cannot%20say.md)**
Covers: how far the evidence was ever taken, and where a claim of "never" rests on runs that could not have come out any other way.
Stops nowhere. Read it before leaning on any Blender finding in this project, whichever document it came from.

## If your problem is one of these, start here

The questions a builder actually arrives with. Where a joined answer is named, open that first; the comparison questions after it are for the exact words, the exact messages, and what each version does.

### Reading the user's file

- **Which Blender saved this file, and which one should read it?** — [What a file can be asked before it renders](What%20a%20file%20can%20be%20asked%20before%20it%20renders.md). Then: *Reading a file 1*, *Settings 10*, *Builds 3*, *What a render demands 5*.
- **What can I read out of a `.blend` with nobody opening it?** — [What a file can be asked before it renders](What%20a%20file%20can%20be%20asked%20before%20it%20renders.md). Then: *Reading a file 1*, *Reading a file 3*, *Settings 4*, *Settings 9*, *Settings 12*, *Settings 13*, *Frames 8*.
- **How many scenes are in the file, and can each be its own job?** — [What a file can be asked before it renders](What%20a%20file%20can%20be%20asked%20before%20it%20renders.md). Then: *Settings 12*, *The compositor 3*.
- **What frames will this job actually render?** — [What a file can be asked before it renders](What%20a%20file%20can%20be%20asked%20before%20it%20renders.md). Then: *Frames 8*. Alongside it: *Frames 7*, *Frames 10*.
- **Is this view layer on at this frame, is that animated, and does it carry its own sample count?** — [What a file can be asked before it renders](What%20a%20file%20can%20be%20asked%20before%20it%20renders.md). Then: *Reading a file 3*, *Settings 4*, *Settings 16*.
- **The user typed a sample count. What does it set aside?** — [What a file can be asked before it renders](What%20a%20file%20can%20be%20asked%20before%20it%20renders.md). Then: *Settings 16*. Alongside it: *Settings 2*, *Settings 4*, *Settings 18*.
- **The user typed a resolution outside what the setting accepts.** — [What can be set from outside](What%20can%20be%20set%20from%20outside.md). Then: *Settings 15*. Alongside it: *Settings 14*.
- **The user typed a sample count outside what the setting accepts.** — [What can be set from outside](What%20can%20be%20set%20from%20outside.md). Then: *Settings 18*. Alongside it: *Settings 16*, *Settings 15*.
- **What denoising does this file ask for?** — [What a file can be asked before it renders](What%20a%20file%20can%20be%20asked%20before%20it%20renders.md). Then: *Settings 19*.
- **Does the file cap its own detail — subdivision, child particles, texture size?** — [What a file can be asked before it renders](What%20a%20file%20can%20be%20asked%20before%20it%20renders.md). Then: *What a render demands 6*. Alongside it: *What a render demands 1*, *What a render demands 5*.
- **Does this job need add-ons, and can the file tell me?** — [What a file can be asked before it renders](What%20a%20file%20can%20be%20asked%20before%20it%20renders.md). Then: *Add-ons 1*. Alongside it: *Add-ons 3*, *Add-ons 7*.
- **Will the compositor run, and which of its output nodes will write?** — [What a file can be asked before it renders](What%20a%20file%20can%20be%20asked%20before%20it%20renders.md). Then: *The compositor 1*, *File output nodes 13*, *File output nodes 14*. Alongside them: *File output nodes 8*, *The compositor 2*.

### Getting the file and its files onto the machine

- **What do I have to upload beside the `.blend`?** — [What has to travel with a file](What%20has%20to%20travel%20with%20a%20file.md). Then: *Dependencies 1*, *Dependencies 3*, *Dependencies 15*, *Dependencies 20*, *Add-ons 1*.
- **How is each outside location written inside the file, and can I rewrite it after the upload?** — [What has to travel with a file](What%20has%20to%20travel%20with%20a%20file.md). Then: *Dependencies 5*, *Dependencies 17*, *Reading a file 5*, *Reading a file 9*.
- **What does opening the file and saving it again do to the places it records?** — [What has to travel with a file](What%20has%20to%20travel%20with%20a%20file.md). Then: *Dependencies 20*. Alongside it: *Dependencies 5*, *Reading a file 5*.
- **One row stands for a whole folder of files.** — [What has to travel with a file](What%20has%20to%20travel%20with%20a%20file.md). Then: *Dependencies 4*. Alongside it: *Reading a file 9*, *Images 9*.
- **A linked `.blend` brings references of its own. How deep does that go?** — [What has to travel with a file](What%20has%20to%20travel%20with%20a%20file.md). Then: *Dependencies 13*, *Reading a file 8*, *Node setups 2*.
- **The linked file arrived, but the thing named inside it did not.** — [What has to travel with a file](What%20has%20to%20travel%20with%20a%20file.md). Then: *Dependencies 11*. Alongside it: *Dependencies 8*.
- **Which scene uses this file?** — [What has to travel with a file](What%20has%20to%20travel%20with%20a%20file.md). Then: *Reading a file 7*. Alongside it: *Dependencies 14*.
- **A path matches except for the capital letters.** — [What has to travel with a file](What%20has%20to%20travel%20with%20a%20file.md). Then: *Dependencies 8*, *Reading a file 10*, *Dependencies 19*, *Node setups 1*.
- **The file was saved on another kind of computer, and the names are not plain Latin letters.** — [What has to travel with a file](What%20has%20to%20travel%20with%20a%20file.md). Then: *Dependencies 6*, *Dependencies 9*. Alongside them: *Dependencies 5*, *Dependencies 20*.
- **Something the file needs did not arrive. What does the render do?** — [What has to travel with a file](What%20has%20to%20travel%20with%20a%20file.md). Then: *Dependencies 12*, *Dependencies 16*, *Dependencies 18*, *Images 2*, *Images 9*, *Add-ons 7*, *Reports 13*.
- **A file already sent was changed on disk. Does the next render use the new one?** — [What has to travel with a file](What%20has%20to%20travel%20with%20a%20file.md). Then: *Reading a file 11*. Alongside it: *Node setups 1*.

### Preparing the machine and starting the job

- **Which settings can I impose as the render starts, without touching the user's file?** — [What can be set from outside](What%20can%20be%20set%20from%20outside.md). Then: *Settings 1*, *Settings 17*, *What a render demands 3*, *Writing 1*, *Writing 9*.
- **What name does an instruction handed in at launch use for a setting?** — [What can be set from outside](What%20can%20be%20set%20from%20outside.md). Then: *Settings 17*. Alongside it: *Settings 1*.
- **What does a script handed in at launch see of a setting given on the same line?** — [What can be set from outside](What%20can%20be%20set%20from%20outside.md). Then: *Writing 13*. Alongside it: *Settings 11*, *Settings 17*.
- **Where on the launch line does an argument have to sit, and does the order matter?** — [What can be set from outside](What%20can%20be%20set%20from%20outside.md). Then: *Writing 1*, *What a render demands 3*, *Writing 13*, *Settings 16*. Alongside them: *Dependencies 17*.
- **Will the file's own stored scripts run, and can I allow or block them at launch?** — [What can be set from outside](What%20can%20be%20set%20from%20outside.md). Then: *Settings 11*. Alongside it: *The machine 7*, *Writing 13*.
- **Will Blender refuse a bad value, or take it and carry on?** — [What can be set from outside](What%20can%20be%20set%20from%20outside.md). Then: *Settings 12*, *Settings 13*, *Settings 15*, *Settings 17*, *Settings 18*, *Images 3*, *Images 4*, *Frames 7*, *Reports 12*, *What actually ran 4*.
- **Can a render run on a machine with no screen at all?** — [Running Blender with nobody there](Running%20Blender%20with%20nobody%20there.md). Then: *Settings 6*, *The machine 1*, *Reports 9*.
- **The machine's screen is drawn by its processor, not by its card. Which device will the render use?** — [Running Blender with nobody there](Running%20Blender%20with%20nobody%20there.md). Then: *The machine 8*, *Settings 6*. Alongside them: *The machine 2*.
- **What has to be on a rented machine before Blender will render, and before it will use its cards?** — [Running Blender with nobody there](Running%20Blender%20with%20nobody%20there.md). Then: *The machine 1*, *The machine 2*, *The machine 3*, *Builds 10*.
- **What does a machine need before a Blender build can even be unpacked?** — [Running Blender with nobody there](Running%20Blender%20with%20nobody%20there.md). Then: *Builds 10*. Alongside it: *The machine 1*, *Builds 1*.
- **What builds does Blender publish, and what does installing or removing one leave behind?** — no joined answer covers this. Go to: *Builds 1*, *Builds 2*, *Builds 3*, *Builds 4*, *Builds 5*, *Builds 7*, *Builds 10*, *Add-ons 6*.
- **An install or a removal of Blender was cut off part-way. Can it be done again?** — no joined answer covers this. Go to: *Builds 9*, *Builds 10*.
- **An add-on has to get onto the machine. Where can it come from, and which Blender does it then belong to?** — [Running Blender with nobody there](Running%20Blender%20with%20nobody%20there.md). Then: *Add-ons 3*, *Add-ons 5*, *Add-ons 6*, *Add-ons 10*, *Add-ons 14*.
- **Can I install a Blender or an add-on while a job is rendering?** — [Two renders in one folder](Two%20renders%20in%20one%20folder.md). Then: *Builds 6*, *Add-ons 12*. Alongside them: *Builds 8*.
- **The machine has several cards. Can one job use them all?** — [What can be set from outside](What%20can%20be%20set%20from%20outside.md). Then: *The machine 5*, *The machine 6*, *Settings 9*.
- **Will more cards make this render faster?** — [What the research cannot say](What%20the%20research%20cannot%20say.md). Then: *The machine 5*, *Reports 6*.
- **What does a machine answer when it is asked, cold, which cards it has?** — [Running Blender with nobody there](Running%20Blender%20with%20nobody%20there.md). Then: *The machine 5*, *The machine 6*. Alongside them: *The machine 4*.
- **Which of a card's rendering methods will be used, and can I choose it as the render starts?** — [What can be set from outside](What%20can%20be%20set%20from%20outside.md). Then: *What a render demands 7*, *The machine 5*, *The machine 6*, *What actually ran 1*. Alongside them: *The machine 4*.
- **One scene pulls in another through the compositor. Which engine and sample count does the pulled-in one render with?** — [What can be set from outside](What%20can%20be%20set%20from%20outside.md). Then: *The compositor 9*. Alongside it: *Settings 8*, *The compositor 4*.

### What the job writes

- **Where will this job's files land, and how many will there be?** — [Where a render's files go](Where%20a%20render's%20files%20go.md). Then: *Writing 2*, *Writing 4*, *Writing 8*, *Writing 14*, *File output nodes 1*, *File output nodes 6*, *File output nodes 16*, *Frames 9*, *Images 3*.
- **Where do a File Output node's files actually land?** — [Where a render's files go](Where%20a%20render's%20files%20go.md). Then: *File output nodes 5*, *File output nodes 4*, *Writing 9*. Alongside them: *Writing 3*, *Writing 12*.
- **Can I tell a job finished by counting the files in the folder?** — [Where a render's files go](Where%20a%20render's%20files%20go.md). Then: *Reports 12*, *Frames 9*, *File output nodes 6*. Alongside them: *Images 5*, *Images 8*.
- **A scene is brought in through the video sequence editor. What do its output nodes write, and what are the files called?** — [Where a render's files go](Where%20a%20render's%20files%20go.md). Then: *File output nodes 16*, *File output nodes 17*. Alongside them: *The compositor 7*, *Writing 8*.
- **Will two destinations write the same picture on top of each other?** — [Where a render's files go](Where%20a%20render's%20files%20go.md). Then: *File output nodes 10*. Alongside it: *The compositor 2*, *The compositor 5*.
- **Which file is the finished picture, and what can I show as a preview?** — [Where a render's files go](Where%20a%20render's%20files%20go.md). Then: *Images 1*, *Images 5*, *Images 6*, *Images 8*, *Frames 9*, *The compositor 7*.

### Watching a job that is running

- **What can I put on screen while a job renders, and keep updating?** — [While a render runs](While%20a%20render%20runs.md). Then: *Reports 3*, *Reports 4*, *Reports 9*.
- **Is there a time estimate, a percentage for the whole job, a tile count?** — [While a render runs](While%20a%20render%20runs.md). Then: *Reports 5*, *Reports 3*, *Reports 7*.
- **When does a printed line actually reach the program reading it?** — [While a render runs](While%20a%20render%20runs.md). Then: *Reports 14*. Alongside it: *Reports 4*, *Reports 8*.
- **Which stream does a complaint land on?** — [While a render runs](While%20a%20render%20runs.md). Then: *Reports 13*. Alongside it: *Reports 4*, *Reports 2*.
- **A frame file appeared in the output folder. Is it finished?** — [While a render runs](While%20a%20render%20runs.md). Then: *Frames 6*, *Writing 11*, *Writing 14*.
- **Blender reported a file written. Is it whole on disk at that moment?** — [While a render runs](While%20a%20render%20runs.md). Then: *Writing 16*, *Writing 14*. Alongside them: *File output nodes 15*.
- **The job writes a video. What is in the file while it renders?** — [While a render runs](While%20a%20render%20runs.md). Then: *Writing 15*. Alongside it: *Frames 5*, *Frames 12*, *Writing 11*.
- **Are one frame's files all finished before the next frame's begin?** — [While a render runs](While%20a%20render%20runs.md). Then: *Writing 14*. Alongside it: *Frames 6*, *Frames 10*.

### When a job stops

- **The job stopped early. Which frames on disk are whole, and which are not?** — [When a render ends](When%20a%20render%20ends.md). Then: *Frames 3*, *Frames 12*, *Frames 13*, *Frames 15*, *Frames 16*, *Writing 14*.
- **The job was writing a video and was stopped. What is left of the video?** — [When a render ends](When%20a%20render%20ends.md). Then: *Frames 17*, *Frames 5*, *Frames 12*. Alongside them: *Writing 15*, *Writing 11*.
- **I asked the job to stop. How long until it does, and what comes out meanwhile?** — [When a render ends](When%20a%20render%20ends.md). Then: *Frames 15*. Alongside it: *Frames 11*, *Reports 12*.
- **Can a job be paused, or carried on from where it stopped?** — [When a render ends](When%20a%20render%20ends.md). Then: *Frames 1*, *Frames 2*, *Frames 4*, *Frames 12*.
- **The job stopped part-way. Is it safe to start it again and let it pass over the frames already there?** — [When a render ends](When%20a%20render%20ends.md). Then: *Frames 4*, *Frames 13*, *Frames 16*. Alongside them: *Frames 6*, *Frames 3*, *Writing 11*.
- **Can I stop a job without losing the frame it is on?** — [When a render ends](When%20a%20render%20ends.md). Then: *Frames 11*, *Frames 13*, *File output nodes 13*.
- **The job failed. What words come out, and can I show one plain line?** — [When a render ends](When%20a%20render%20ends.md). Then: *Reports 1*, *Reports 2*, *Reports 4*, *Reports 13*.
- **What number does a run hand back, for each way it can end?** — [When a render ends](When%20a%20render%20ends.md). Then: *Reports 12*. Alongside it: *Frames 15*, *What actually ran 4*.
- **Can I trust the exit status?** — [When a render ends](When%20a%20render%20ends.md). Then: *Reports 12*, *Writing 5*, *File output nodes 15*, *Reports 1*, *Settings 9*.
- **Does a render survive the connection dropping?** — [When a render ends](When%20a%20render%20ends.md). Then: *The machine 7*, *Builds 8*.
- **The machine's own memory ran out while the render was sampling.** — [When a render ends](When%20a%20render%20ends.md). Then: *Reports 10*. Alongside it: *What a render demands 7*.
- **The scene has no camera. What does the render do?** — [When a render ends](When%20a%20render%20ends.md). Then: *Reports 12*, *Reports 1*. Alongside it: *Settings 6*.

### After a job has finished

- **It reported success. Did it do what was asked?** — [What a render really did](What%20a%20render%20really%20did.md). Then: *Reports 1*, *Reports 12*, *The machine 4*, *Settings 15*, *Settings 18*, *Add-ons 7*, *Dependencies 12*.
- **A frame is on disk. Did it take the samples it was asked for, or was it cut short?** — [What a render really did](What%20a%20render%20really%20did.md). Then: *Reports 11*, *Settings 2*. Alongside them: *Settings 16*, *Reports 7*, *Frames 15*.
- **What did the work — which card, which engine, which add-ons — and when can I know?** — [What a render really did](What%20a%20render%20really%20did.md). Then: *What actually ran 1*, *What actually ran 2*, *What actually ran 3*, *What actually ran 5*, *What actually ran 6*.
- **What does a finished picture record about what made it?** — [What a render really did](What%20a%20render%20really%20did.md). Then: *Images 10*. Alongside it: *Reports 11*.

### Two questions that cut across everything

- **Two jobs writing into one folder.** — [Two renders in one folder](Two%20renders%20in%20one%20folder.md). Then: *Frames 14*, *Frames 4*, *Builds 5*, *Reading a file 2*. Alongside them: *Frames 16*.
- **Before I lean on any of this — how far was it tested?** — [What the research cannot say](What%20the%20research%20cannot%20say.md). It names no single question, because it is the ceiling over all of them.

## The fifteen topics, and what each one owns

For somebody who already knows roughly what they want. Each is one comparison document at `Preparation/Blender Research/Comparison/`.

- [Add-ons](../Comparison/Add-ons.md) — whether a job needs any, whether one can be put on a machine with nobody at a screen, and what a render does without it.
- [Builds](../Comparison/Builds.md) — what Blender publishes, whether an installed one can be asked what it is, and what putting one on a machine or taking it off does.
- [Dependencies](../Comparison/Dependencies.md) — every outside file a `.blend` points at: how each is written down, how it is found again, and what a render does when one is gone.
- [File output nodes](../Comparison/File%20output%20nodes.md) — the compositor's own way of writing files: what it writes, where, under what names, and what a failed write does.
- [Frames](../Comparison/Frames.md) — what a render does with the frames it was given, and what is on disk when it does not run to the end.
- [Images](../Comparison/Images.md) — which pictures a render makes, which formats it can write, what a written file carries, and what a missing picture leaves.
- [Node setups](../Comparison/Node%20setups.md) — what a geometry node setup makes a file point at, and what a file linking one records about those files.
- [Reading a file](../Comparison/Reading%20a%20file.md) — what a `.blend` gives up to something reading it, what can be written back into one, and what changes by version.
- [Reports](../Comparison/Reports.md) — what Blender says while it is working, and what it says when a render does not succeed.
- [Settings](../Comparison/Settings.md) — which settings a render uses, where each of them lives, and which can be set at the moment a render starts.
- [The compositor](../Comparison/The%20compositor.md) — what makes it run, what it belongs to, how far a picture can be followed through it, and what decides the size written.
- [The machine](../Comparison/The%20machine.md) — what a computer needs before Blender runs and before it renders on its cards, how cards are chosen and reported, what a render does with nobody at the computer, and what it does when the screen is drawn by something other than the card.
- [What a render demands](../Comparison/What%20a%20render%20demands.md) — what a render asks of the computer it runs on, what can be settled from outside, and what any of it changes about the picture.
- [What actually ran](../Comparison/What%20actually%20ran.md) — what a finished render can be made to say about the parts of the computer, the engine and the add-ons that did the work.
- [Writing](../Comparison/Writing.md) — where the files a render produces end up, how the places a render writes to are recorded, and what happens when a write fails.

## Every question, by topic

The index of last resort, and the thing that makes a search work. Wording is each comparison's own, shortened to a line.

### Add-ons

- **1.** Can a `.blend` file be read for the add-ons it needs?
- **2.** What makes a file need an add-on at the moment it renders, and can one that takes part in the render be told from one that only rearranges tools?
- **3.** Can an add-on be installed, switched on, and given whatever else it needs, on a computer with no screen attached?
- **4.** What does an add-on record about the kinds of computer it is meant for, and does one set up on one computer still work when copied to another kind?
- **5.** How can an add-on be found, fetched and installed with nobody at a screen, and what decides the places it can come from?
- **6.** What does an add-on belong to, once it is installed on a computer?
- **7.** What does a render do when an add-on it needs is not there?
- **8.** Where do add-ons come from besides Blender's own, how are they sold, and what does that mean for copying one between computers?
- *There is no question 9. It was retired and the number is not reused; the comparison says why.*
- **10.** Can a computer be asked which add-ons it holds, and for which Blender?
- **11.** Can an add-on depend on something else being on the computer, or elsewhere, and can what it needs be read anywhere?
- **12.** Does installing an add-on disturb a render already running on the same computer?
- **13.** Once an add-on is installed and switched on, what more can be learned about whether it will work?
- **14.** Does an add-on set up for one Blender work in another Blender on the same computer, and what decides that?

### Builds

- **1.** What kinds of build does Blender publish, and how is each kind named and marked?
- **2.** Can builds of each kind be listed and fetched by something other than a person at a web page?
- **3.** Can an installed Blender be asked what it is — which version, and which kind of build?
- **4.** Does Blender name one release as the one to use for ordinary work?
- **5.** Can one Blender installed on a computer cause trouble for another that is already there?
- **6.** Does installing a Blender disturb a render already running on the same computer?
- **7.** Can a computer be asked which Blenders it holds?
- **8.** What becomes of a render that is running when the Blender it is using is removed?
- **9.** Can installing or removing a Blender be done again safely on a computer where it was cut off part-way?
- **10.** What does a computer need to have on it before a published Blender build of each kind can be unpacked?

### Dependencies

- **1.** What kinds of thing can a `.blend` file point at outside itself? The full list.
- **2.** What kinds of library can a file point at, including one that does not sit on the same computer?
- **3.** Can a file hold the things it depends on inside itself partly rather than wholly, and what does a partly-held file report?
- **4.** Can one thing a file points at mean several files on disk?
- **5.** In what ways can a recorded location be written?
- **6.** What does a location recorded on one kind of computer mean on another, including a kind that names its disks?
- **7.** What does a location pointing somewhere other than the computer the file sits on mean, at either end?
- **8.** Does the difference between capital and small letters in a name matter, and does it matter the same on every kind of computer?
- **9.** Do names written outside the Latin alphabet survive being read out, moved to another computer, and found again by a render?
- **10.** Are some of the things a file points at places it writes to rather than reads from, and where does such a write land?
- **11.** Can a reference point at a named thing inside another file, and what happens when that thing is renamed or is gone?
- **12.** What does a render do when something the file points at is not there, and does it say so anywhere?
- **13.** Does a file pointing at another `.blend` bring that file's own references with it, how deep does that go, and can they be reached the same way?
- **14.** Does rendering one scene of a file pull only that scene's references?
- **15.** Can a file rely on a colour management profile that lives on the computer, and how does Blender find it?
- **16.** How is a font recorded in the file, and what does Blender do when a font it points at is not on the computer?
- **17.** What use does a `.blend` make of the places it records for writing, besides writing to them?
- **18.** What does a render do when a cache or a bake that a file records is absent?
- **19.** What decides whether two recorded library locations are treated as one library or as two?
- **20.** When a file is opened and saved again, with none of the places it records for outside files touched while it is open, what does the saved file record for each of those places?

### File output nodes

- **1.** What can a File Output node be made to write, how many outputs can it have, and what does each output carry?
- **2.** Which settings belong to an individual output, and which belong to the node as a whole?
- **3.** Does each output carry its own folder as well as its own name?
- **4.** Can a folder be expressed inside an output's name rather than separately?
- **5.** Which folders can a File Output node be pointed at, including ones unrelated to the file's own and ones reached over a network?
- **6.** Can several outputs be gathered into one file rather than written separately?
- **7.** What happens to a File Output node inside a group of nodes?
- **8.** Can one node be switched off on its own, and can one output within a node be switched off on its own?
- **9.** What restrictions apply to the kind of picture a File Output node can write?
- **10.** What happens when two outputs are aimed at the same name in the same place?
- **11.** Can a File Output node write a video file?
- **12.** What can a File Output node be fed, and does it have to come from the render?
- **13.** What causes a File Output node to write, and when during a render does it write each file?
- **14.** Can anything deciding whether a node writes be made to change from one frame to the next inside a single render?
- **15.** When one of several files for the same frame cannot be finished, what happens to that file, the others, the render and its reports?
- **16.** When a scene is brought into another scene through its video sequence editor, what do the File Output nodes belonging to the brought-in scene do?
- **17.** What is a file written by a File Output node called, when that node belongs to a scene brought into another scene through its video sequence editor?

### Frames

- **1.** Can a render that was interrupted by the computer going down be picked up from where it stopped?
- **2.** Can a render that was stopped on purpose be picked up later?
- **3.** Do frames already written survive a render ending part-way, whether it was stopped on purpose or not?
- **4.** Can Blender be asked to render only the frames not already on disk, and how does it decide what counts as already there?
- **5.** What does an animation written as a single video file leave behind when the render stops part-way?
- **6.** Is a file existing proof that the frame finished writing?
- **7.** Can Blender be given a set of frames that is not one plain run — single frames, or two separate stretches?
- **8.** What does a frame range consist of?
- **9.** What does a frame write when several view layers are active on it?
- **10.** In what order does a render work through the frames it was given, and what does an interruption leave behind in each case?
- **11.** Can a render be told to stop once the frame it is working on is finished, rather than the moment it is asked?
- **12.** When one render writes a video and separate images over one range, what does an interruption leave if the video has not finished closing?
- **13.** When a render is stopped at the exact moment it is writing a frame's own image file, what is left where it writes?
- **14.** When two renders write into one folder and are started at the same instant, what does each produce, and what does the folder hold?
- **15.** When a render working through a range of frames is interrupted, what does it do between being asked to stop and ending, how long does that take, what settles that, and what does it print meanwhile?
- **16.** When a render working through an animation is stopped at no particular moment, what is in the place it writes to for the frame it had not finished?
- **17.** When a render writing an animation as a single video file is stopped before the end, what does each way of stopping leave of that file?

### Images

- **1.** What kinds of picture can a render produce, and how can the finished picture be told apart from the others?
- **2.** Do any of the pictures a render produces show whether a scene's textures loaded, and if so, which?
- **3.** Which file formats can Blender write a render out as?
- **4.** What happens when a file asks for a format that Blender cannot write?
- **5.** Can a format hold several pictures inside one file, and how can the finished one be told apart inside it?
- **6.** Does a format that can hold more light than a screen can show also record the treatment it was made under?
- **7.** Can Blender turn a set of finished frames into a short moving picture?
- **8.** Can Blender be made to produce a smaller, plainly viewable copy of a picture a render produced?
- **9.** What does a render produce where a missing image is feeding something other than colour?
- **10.** What does a finished picture hold, in every part of the file, about what produced it?

### Node setups

- **1.** What files can a geometry node setup inside a `.blend` cause that file to point at?
- **2.** When a `.blend` links a node setup from another `.blend`, what does the linking file record about the files that setup uses, and how are they reached?

### Reading a file

- **1.** What does a `.blend` give up without a Blender to open it, and what needs one?
- **2.** How many Blenders can be at work on one computer at the same time, and does one reading a file while another renders cause trouble?
- **3.** Can a single read of a file tell, for every view layer, whether that layer renders at any frame later asked about?
- **4.** Can the places a file is set up to write to be read out of it?
- **5.** Can the locations a file records for the things it points at be read out and written back, and by what?
- **6.** Can anything be asked to report which of a file's references are missing, to search for them, and to rewrite the locations?
- **7.** Can something a file points at be traced to the scene that uses it?
- **8.** Can a file's full chain of references be read while some of the referenced files are absent?
- **9.** What separate means exist for listing and rewriting the locations recorded in a `.blend`, and which kinds does each reach?
- **10.** When a search for a missing reference finds more than one possible file, what makes a match, which is chosen, and what is written back?
- **11.** What happens when a file Blender has already read is changed or replaced on disk, and what has to happen before a later render uses it?

### Reports

- **1.** What can cause a render to fail, and what does Blender do in each case?
- **2.** Where does Blender say that a render failed, and what does it say there?
- **3.** What does Blender report about how far through a piece of work it has got?
- **4.** What does Blender write out as it works, what is that stream called, and what does it contain?
- **5.** Does Blender estimate the time left, for what unit of work, and how does it describe that estimate?
- **6.** What does Blender report about the memory it is using on a graphics card?
- **7.** What does a render report about its own progress, across the whole range of picture sizes a render will accept?
- **8.** What does a render print over the whole of a run, from its first line to its last?
- **9.** What does a render report about its progress under each of the ways a render can be started?
- **10.** What does a render do when the machine's own memory runs out while it is sampling?
- **11.** What, if anything, does a render print that shows whether a frame took the number of samples it was asked for?
- **12.** What ending status does a run of Blender give back to whatever started it, for each of the ways a run can end — finishing normally, being interrupted, and being refused something it was handed?
- **13.** When a file the scene points at is not there, what does a render put on each of its output streams?
- **14.** When a render's printed output is sent somewhere other than a terminal, what decides when each line becomes available to whatever is reading?

### Settings

- **1.** Which of a render's settings can be settled at the moment a render is started from outside Blender, and which need the file changed?
- **2.** Can a render be set to stop once the picture is clean enough, and can it be set to stop after a length of time?
- **3.** Which of a render's settings belong to a particular render engine rather than to every engine?
- **4.** Does a file carry a separate sample count for each render engine?
- **5.** Can a sample count change from frame to frame?
- **6.** Does every render engine render on a computer with no screen attached?
- **7.** Does a scene set up for one render engine come out the same in another?
- **8.** Can one scene be set up to take in another scene's rendered result, so that rendering the first renders the second?
- **9.** Does a file record whether it renders on the computer's graphics cards or on its processor?
- **10.** What does Blender state about opening a file that was saved by a different version of Blender?
- **11.** Can a file rely on scripts stored inside it to render correctly, and what decides whether those run from outside?
- **12.** Can a render started from outside be pointed at one named scene of a file, and what does it render when it is not told which?
- **13.** What decides the list of render engines a file can choose between, and can that list grow on a given computer?
- **14.** What values can a render's resolution percentage be set to?
- **15.** What does a render do when it is asked for a value outside what that setting accepts, and what does it report?
- **16.** When a render is started from outside and given a sample count of its own, what decides how many samples it takes, and what else has a say in when it stops?
- **17.** When an instruction handed in at launch sets the resolution percentage, the sample count, or whether a view layer renders, what name does that instruction use for each of those three?
- **18.** What values will a render's sample count be set to, and what does a render do when it is given a value outside those?
- **19.** What settings does a scene or a view layer carry that govern denoising, and what does each of them do?

### The compositor

- **1.** What decides whether a file's compositor runs at all?
- **2.** Can the place where a scene's rendered frames are written be switched off entirely?
- **3.** Does a compositor belong to one scene, or to the whole file it is part of?
- **4.** If a scene takes in another scene's rendered result, does running the first also run the second scene's compositor?
- **5.** When a render enters the compositor, what does it arrive as, can it be told apart, and can it be followed forward?
- **6.** Besides a scene's pixel dimensions and its resolution percentage, can anything else inside a file decide the size of what gets written?
- **7.** Can a scene bypass the compositor and produce its output from its video sequence editor instead, and what does that change?
- **8.** Can a picture be followed forward through a node that changes its colours, and through one that takes two pictures in and gives one out?
- **9.** When one scene takes in another scene's rendered result, what decides the render engine and the sample count the second is rendered with, and what does a value for either, given from outside, do to that?

### The machine

- **1.** What must a computer have on it before Blender will run at all?
- **2.** What must a computer have on it before Blender will render using its graphics cards?
- **3.** Does what a computer must have on it depend on how Blender was obtained and put there?
- **4.** Does Blender say anything about what is missing when it will not start, or when it will not render?
- **5.** How is it decided which graphics cards a render uses, can more than one be used, and how is that decided from outside?
- **6.** Can Blender be asked which graphics cards it can see and can use?
- **7.** Can anything a render does wait for a person, and what happens on a computer with nobody at it?
- **8.** On a computer with a graphics card whose screen is drawn by the processor rather than by that card, what does a render do under each engine?

### What a render demands

- **1.** What settings, if any, change how much a render demands of the computer it runs on?
- **2.** For each of them, what does it change about what the render produces?
- **3.** Which of them can be settled at the moment a render is started from outside, and which only by changing the file?
- **4.** Do any of them belong to a particular render engine rather than to every engine?
- **5.** Can their values be read out of a file?
- **6.** What does a saved scene record about limits on subdivision, child particles and texture size, including any switch governing those limits, and what can be read back when the file is opened from outside Blender's interface?
- **7.** Where a render can be told which of a graphics card's rendering methods to use, what methods are there, what does choosing one rather than another change about the render, and is it established whether that changes with the size of the scene?

### What actually ran

- **1.** When a render has finished, what can be learned about which parts of the computer it used?
- **2.** When a render has finished, what can be learned about which render engine produced it?
- **3.** When a render has finished, what can be learned about which add-ons were in use while it ran?
- **4.** What does a render do when it is asked for something the computer or the installed Blender cannot provide, and what does it report?
- **5.** Where can each of the things above be learned from — what a render prints, what it leaves on disk, or what can be asked afterwards?
- **6.** At what point can each of the things above be learned — before a render starts, while it runs, or only once it has finished?

### Writing

- **1.** Can where a render writes be settled at the moment the render is started, without changing the file?
- **2.** How is a place to write recorded in a file — what parts does the recorded thing have?
- **3.** What characters or marks can a recorded place to write carry that stand for something else, and what does each do?
- **4.** Does Blender make folders that are not already there when it writes?
- **5.** What does a write do when its folder is not there, when there is no room left, or when it cannot be written to?
- **6.** What are the ways a render can be started, and what does each of them write?
- **7.** If a render can be set up so that it writes no files, what does it leave behind?
- **8.** How does the place a file records for writing become the name of a file on disk?
- **9.** Can the place a file output node writes to be settled for a render, and by writing it into the file from outside?
- **10.** Can the place a render records for writing carry characters the computer will not accept, and what does a render do then?
- **11.** While a render is still working, what is present in the places it writes to, and how does that change with the size of the picture?
- **12.** What does a render do when it must make a folder and cannot?
- **13.** When Blender is started with a setting given on the command line and with a script to run at the same launch, what does that script see of that setting?
- **14.** When a render works through a range of frames, how do the writes belonging to one frame stand in time against the writes belonging to the frame after it?
- **15.** While a render writes an animation as a single video file, what is in that file at each point of the run, and what decides that?
- **16.** Under what conditions does a render report that a file has been written, and what is on disk when it does?

## What this map does not hold

It holds no Blender fact. Not one. It names questions and says what each one asks; it never says what any of them answered. That is deliberate, and it is what keeps this document from ever going out of step with the research: it has nothing to go out of step with. The only thing in it that can turn stale is a question number, and a wrong number can be found by searching.

The facts live in the fifteen comparisons, and the joined answers carry the parts that span several of them. **Where this map and a comparison ever disagree, the comparison is right.**

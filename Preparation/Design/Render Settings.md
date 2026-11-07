# Render Settings

*What the settings page looks like, how a value the user typed is told from the file's own, and how a job is submitted. The conventions it follows are the [whole-app Design](Whole%20app.md)'s and are not restated here.*

## Where it sits

**A panel over the server**, opened by choosing a `.blend` in [File Browser](File%20Browser.md), collapsing to a strip like the dependency check. Both are described in the [whole-app Design](Whole%20app.md).

**Connections.** It serves the [Render Settings spec](../Specs/Render%20Settings.md). It takes the file from [File Browser](File%20Browser.md), sits beside [Dependency Upload & Remap](Dependency%20Upload%20&%20Remap.md), sends jobs to [Queue & History](Queue%20&%20History.md), and states what the file will write, which [What a job writes](../Specs/What%20a%20job%20writes.md) owns.

## The header carries the Blender version

**The version the file was read with and the version the job renders with are the same version**, so they are one control rather than two facts in two places. The [Render Settings spec](../Specs/Render%20Settings.md) settles that a read is taken with the version the job will render with, and that changing the version afterwards leaves the read describing a job that will not happen.

So the header reads: the file's name, then the Blender version as a dropdown, then when it was read, then **Read again** and **Reset all to file values**.

The version list holds every Blender on the server, including one below 4.5 named as such — which can be picked and is never picked automatically.

**The line after the version says how the pick was made, and only that.** *Matched to the file*, or *nearest above the file's*, or *nearest below — nothing above it on this server*. The [architecture](../Architecture/Render%20Settings.md) B2 gives that rule three outcomes and they are not the same news, so one word for all three would hide the one that matters: a file being rendered by an older Blender than it was saved with.

**One thing about the search gets its own line, and only when it is true: a Blender that did not report its version is named.** It sits under the version row. On an ordinary machine it is not there at all. **The page says nothing about folders the search could not read, or about the search going by name** — the [architecture](../Architecture/Render%20Settings.md) B1 and `Server Setup` B11 keep both in the search's answer and do not draw them. *Changed 2026-10-02, Creator-stated, note 13 on 0.1.7: "All this should just go — it's unnecessary noise." Until then all three were drawn where true, and the by-name line was true on every machine.*

**Under the control rather than inside the list**, which was the first answer~~ and is not drawable: the list is an ordinary dropdown and a dropdown has no foot to write in~~. *(Reason struck 2026-10-08: Smoozy draws its own dropdowns from the fifth notes, note 2 — [whole-app Design](Whole%20app.md). The decision stands.)* Under it is the same moment anyway — a user wondering why a Blender they installed is not offered is looking straight at the control when they wonder it.

**Permanent it would be a line about nothing.** That is why it is conditional rather than a standing note about how the search works.

## When there is no version to pick

**Three cases, and none is a failure.** The [architecture](../Architecture/Render%20Settings.md) B5 and B5a produce all three out of the ordinary rule.

**A server holding nothing but Blenders below 4.5.** The list is not empty, so it stays a list; what is empty is the pick. The control shows no version, and the line beside it says a Blender below 4.5 is never picked automatically and that the user can pick one. **The list is not disabled** — picking one is exactly what the user is being invited to do — and Render is not offered until they have, because there is nothing to render with.

**A file that does not say which Blender saved it.** There are Blenders on the server and the list shows them, but there is nothing to match against, so Smoozy does not choose one by itself. This case reads the same as the one above: the control shows no version, the list stays a list and is not disabled, the line beside it says the file does not record its Blender version and that the user can pick, and the read and Render wait for that pick. It is not a failure either — a file that does not say what saved it is an ordinary file, and one pick is the whole of what it costs.

**A server with no Blender at all.** There is nothing to list, so the dropdown is not drawn. One line takes its place saying the server has no Blender, with the way to **Server Setup** beside it. Reading the file is not offered either, since a read is something a Blender does. **The rest of the page is left alone**: the frames, the samples and the percentage are still the user's to type, and they survive being sent to Server Setup and coming back, which is the [architecture](../Architecture/Render%20Settings.md) J3's pre-read answer set doing exactly what it is for.

**An empty dropdown would be worse than none.** A control the user can open and find nothing in makes them wonder what they did wrong; a sentence saying the server has no Blender, with the way to put one there, does not.

**Not offered means the control stays where it was and reads unavailable**, rather than disappearing — Render and the read alike, in all three cases, which is the [architecture](../Architecture/Render%20Settings.md) B5a: a read is something a Blender does, and where no Blender is chosen there is nothing to read the file with. The difference between the cases is what brings the controls back — in the first two the user's pick lifts the withholding; with no Blender at all, nothing on this page does, and Server Setup is the way. Render reads *Render* there rather than naming a frame count. A control that vanished would make the page look different rather than closed, and the user would go looking for what they had broken.

## The version mismatch warning

**One line, directly under the version row and above the pick line, where the picked version and the file's are different release lines.** It names the two and says something may not work as expected: *Saved with 4.5, rendering with 5.1. Some things may not work as expected.* It carries no control and opens nothing. Creator-stated 2026-10-02, note 12 on 0.1.7: *"Just a small indication (not a button) that something may not work as expected is enough. And for some reason the "saved with Blender X, rendering with Blender Y" line is too far from the version picker."* Under Rule 10: the [architecture](../Architecture/Render%20Settings.md) B8 composes the line from the two versions and nothing else. It is plain text, with no dot, and so is the add-on line further down: the Creator, 2026-10-08, sixth notes, note 5.

**What it used to be, kept so nobody draws it back.** The line carried *What changes*, which opened a panel of each version line's recorded breaks, one heading per line crossed. The panel is gone, and so is the sentence on the two silent breaks it named: the line warns, and what may differ is the artist's to judge.

## The one thing this page says about add-ons

**There is no add-ons section, and there should not be.** The [architecture](../Architecture/Render%20Settings.md) E5 is firm: a `.blend` announces exactly one thing about add-ons — a Blender lacking the add-on for the engine the file names says so the moment the file is opened — and there is nothing else in the file to read. A list of what a job needs cannot be drawn, because there is nothing to draw it from.

**So it is one line, and it sits with the engine rather than in a place of its own.** It says this Blender lacks the add-on for this file's engine. **It carries no control.** Until 2026-10-08 it pointed at **Server Setup** to install one; Smoozy installs no add-ons, on the Creator's ruling of that day, quoted in [Server Setup](Server%20Setup.md).

**Its second half is the part that matters, and it is about this page.** A Blender without that add-on reports the file's engine as EEVEE — so the engine shown above the line is not the file's own, and without the line the page would state a wrong engine in the file's own italic face, looking exactly as certain as it does when it is right. The line says the engine reading cannot be trusted while it stands.

## The body is three columns

Not one, because a settings page with a column's width becomes a scroll. Left is the scene, the frames and the View Layers; middle is the render itself and its warnings; right is what the job will write and what has not been checked.

**Two of the headings carry a qualifier saying which reading of the thing is on screen.** *View Layers* names the frame the boxes are read at; *Frames* says whose range is showing — the file's own, or one range covering every ticked scene. Neither is a sentence about the heading's subject, which is what the [whole-app Design](Whole%20app.md) rules out.

## The file's value against the user's

The convention is the [whole-app Design](Whole%20app.md)'s: italic grey is the file's, upright bold is the user's, with a cross beside it.

**Fields are the width of what they hold**, so the cross sits immediately beside the value rather than a mouse-drag away.

**A click into a value box — the frames, the samples, the resolution, Simplify's figures — chooses its whole text, so what is typed replaces it; a second click places the caret.** A redraw that gives the box its focus back chooses nothing. *Added 2026-10-08, Creator-stated, the fifth notes, note 5: "Clicking on a value I can change (samples, resolution, range) should select the text immediately." No auditor has seen it.*

**One control hands everything back at once**, in the header, and the individual crosses stay. It is called **Reset all to file values**.

## Before the file has been read

**A field with no value shows an em dash in the file's own italic face.** The [architecture](../Architecture/Render%20Settings.md) J1 gives every setting on this page three faces and not two: the user's answer upright and bold, the file's value in italic, or — where neither exists — a promise that the file's value is what will be used. The dash is that third face. It is in the italic face deliberately, because italic is already this page's word for *the file's*, so a dash there reads as *the file's, and Smoozy has not looked* rather than as a value of nothing.

**A blank field was the alternative and it is worse.** An empty box reads as zero, or as a control that has failed to draw, and on the samples box those are two things a user might act on.

**The page says once, at the top, why the dashes are there.** A line above the columns rather than a placeholder repeated in every field: the file has not been read, every value below is the file's own until you change it, the scenes and the View Layers are not there because their names come from the file, and submitting works anyway. That last half is the [spec](../Specs/Render%20Settings.md)'s own — the file is read when the job starts either way — and it is the thing a user staring at a page of dashes would otherwise assume was untrue.

**Everything the user can answer without a read, they can answer.** The frame range, the samples, the percentage, the engine and what it renders on all take a value with nothing read, which is the [architecture](../Architecture/Render%20Settings.md) J3's own list. What is not there at all is the scene list and the View Layer boxes: both are names, and a name can only come out of a file somebody read. So those two groups are absent rather than empty — there is nothing to draw, not an empty version of something.

**The Render control names no count here.** It reads *Render* alone rather than a number of frames, because the count is worked out from the file's own range and there is no read to work one out from. It stays live like everything else on this page.

**This is also the reconnect drawing, and it is one drawing rather than two.** When a connection ends, the [architecture](../Architecture/Render%20Settings.md) J10 discards the file's half whole and leaves the user's answers untouched, so the page becomes exactly what it was before any read except that the user's own values are still standing in it — upright and bold, among dashes. Nothing new is drawn for it. The band's wording is the one thing that differs: it says the connection was lost, rather than that the file was not read.

## Reading the file

**One control, in the header, and it changes its name rather than its place.** It reads **Read the file** where nothing has been read and **Read again** where something has. Beside it, where a reading stands, the moment it was taken.

**While it runs, the page does not change.** The [architecture](../Architecture/Render%20Settings.md) Q5 is explicit that nothing is blanked and nothing is provisionally filled — the user's values stand, the last file half stands, and the result lands whole or not at all. So the *underway* shape is the narrowest of the whole app's three: **on the control**, not over the pane. A veil over a page the architecture has deliberately left usable would take away typing and submitting that both still work.

**Pressing it twice does nothing worth drawing.** A second request joins the read already running rather than starting another, which is Q5's rule, so the control is simply already underway.

**It takes about four seconds.** Long enough that a control which did not say it was working would be pressed again; short enough that nothing else is owed.

## When the read gives nothing

**There are four endings, not one.** The [architecture](../Architecture/Render%20Settings.md) Q8 tells them apart from Smoozy's own record rather than from anything Blender printed, and each says something different, so the page cannot have one sentence for all of them. **This document expected one** — the version that could not open the file — and that is only the second of the four.

**All four take the whole-app failure shape**, attached to the control that failed, which is the Read control in the header — so only the second gets a second sentence, that shape granting one only where Smoozy knows what the user should do next.

**And all four leave the page exactly as it was.** Nothing typed is lost, no value is blanked, and **Render stays live**, because the [spec](../Specs/Render%20Settings.md) has the file read again when the job starts whatever happened here. **A reading that was already standing goes on standing too** — the [architecture](../Architecture/Render%20Settings.md) Q9 is explicit that a failed re-read is not evidence the file changed, so a failure never turns a page that had the file's values into the dashed page above. Only never having read, or losing the connection, does that.

- **Could not reach the server.** One sentence and no second one. This ending is not about the file at all, and anything said about the `.blend` would be a claim Smoozy cannot make.
- **This Blender could not open the file**, naming the version. It is the one of the four where Smoozy knows what to do next, so it is the one that carries a second sentence: pick another Blender and read again. The version named is Smoozy's own record of which build it ran, never anything read out of Blender's words.
- **Reading the file did not finish.** What Blender printed is one control away, the same panel every other screen opens. No cause is offered and no second sentence is written, because Smoozy does not know what the user should do next.
- **Read stopped.** One line saying so, claiming nothing about the file, and it can be run again. It exists as its own ending because without it this case looks exactly like the second — invocation over, failed, nothing produced — and the page would tell the user their file cannot be opened, which is false, and send them to change the version for a reason that was never the file's.

**One more case sits beside the four: the server answers and runs no Blender at all** — the Smoozy on it does not know the request, or will not start a read while something else holds the machine. **The page says *Could not read the file on the server.*** and nothing else. *Reading the file did not finish* would be false there, because no read ran, and there is no Blender output to open. It is the sentence the page already uses when the file on the server cannot be read at Submit, so it adds no new words. Label: **settled at Design** (2026-09-29), ruled by the build runner under the Creator's standing delegation.

## Frames

**One field, not two.** The [Render Settings spec](../Specs/Render%20Settings.md) allows a first-last-step or a written list in its place, and using one replaces the other — so two visible fields raise a question with no good answer. One field takes either. The field keeps the accepted text as the user typed it (only spaces at the ends may be trimmed), across leaving the field, switching scenes and reopening the page within the same run. *Changed 2026-10-22, the Creator's seventeenth notes, note 3, Creator-stated.* **A page opened by *Submit again* shows the frames the way the job's entry holds them** — as the user typed them where the entry kept that text, the numbers form otherwise — and the same text reads in Queue and History. *Added 2026-10-24, Creator-stated, the eighteenth notes, note 8: "has to be maintained throughout Smoozy, as in queue and history should show it the same way." Under Rule 10, `Render Settings` F1. No auditor has seen it.*

It accepts `1-240`, `1-240x2` for a step, `1,5,9` for particular frames, or a mix. **A tooltip on a small question mark shows the four forms**, on hover and on keyboard focus, and nothing is explained permanently.

**The frame count sits beside it**, which is what shows the user they got what they meant.

**Where the user's range stands, the heading's qualifier *the file's* is not drawn.** The bold value already says whose it is, and *the file's* beside the user's range is false. *Added 2026-10-01, from step 10c's pass before Vast on Sesterce.*

## View Layers

This is the setting where what is shown and what the job does are not the same thing, so it carries the most marking.

- **A box the user has not touched is a reading of one frame** — the lowest the range covers — and its label is italic.
- **A box the user pinned is upright and bold**, with a cross beside it.
- **A View Layer whose renderability is animated in the file carries Blender's keyframe diamond**: solid where it is keyed on the frame being read, hollow where it is animated but not keyed there.
- **A View Layer carrying its own sample count says the number**, and says it is overridden where the user has typed one.

**The hollow diamond is what stops a lie.** A View Layer that is off at the frame being read but comes on later in the range would otherwise show an empty box and render anyway — the case the [Render Settings spec](../Specs/Render%20Settings.md) warns about in as many words.

### Handing a pinned box back

**The cross sits beside the box, outside it.** The whole box is the switch, so a cross inside it would be a press that also toggled the thing it was undoing. It sits immediately to the right, which is where every cross on this page sits — beside its value.

**The box carries no third mark saying it is the user's.** The weight says that and the tick says which way it was set, so *you set this on* beside a bold, ticked box was one fact said a third time. It is gone.

**What the cross hands back to is the file's own answer at the frame the boxes are read at, so the tick can change as it is pressed.** The cross says so rather than leaving it to be discovered: *Back to the file's value at frame 1*.

**A pinned box also carries no diamond, and no reading of a later frame.** The [architecture](../Architecture/Render%20Settings.md) V4 and V6 have the pin sever the file's animation and then set the value, so the curve decides nothing about that layer. A diamond there would say the file's animation still governs — the one thing the diamond exists to stop. A View Layer's own sample count is not the switch, so that tag stays on a pinned box like any other.

## The sample count

**The box has two readings and one act tells them apart: whether the user has typed.** Left alone it shows the file's own count in the file's italic. Typed into it is upright and bold with the cross beside it, like everything else on this page.

**What the file was going to do about stopping short is said either way.** The [spec](../Specs/Render%20Settings.md) shows the file's early-stop or time-stop setup whether or not the box has been touched, because whether a file stops itself short changes what a job costs on a machine charged by the hour. So a line under the box names Blender's own settings and their values — *This file stops early at noise threshold 0.01, and after a time limit of 30s on each frame.* Where the file carries neither, there is no line.

**The time limit says *a frame* because that is what it is.** The [spec](../Specs/Render%20Settings.md) has it spent on each frame rather than across the job, and a frame cut short is still written, so a long animation pays it many times over. *30s* alone would read as a cap on the whole job and be wrong by the frame count.

**Typing a number replaces that line rather than adding to it.** The typed line names what the number sets aside — the same settings by the same names, plus any View Layer's own count — and then what that costs: every frame renders all samples, so the job may take longer and cost more. Two lines saying *this file stops early at 0.01* and *you have just switched off stopping early at 0.01* are one fact twice.

**Neither line is drawn before the file has been read.** Whether there is anything to set aside is a fact about the file's insides, and the band above the columns already says once that none of them is known. The [spec](../Specs/Render%20Settings.md) asks for that to be said and it is said there — not a second time under one setting, which is the same refusal as the dashed fields sharing one band.

### When the override warning goes

**A second line says which View Layers do not render at the count on screen** — *FX and Mist keep their own sample counts* — naming them rather than repeating their numbers, which their own boxes in the left column already carry.

**It is composed afresh from the layers still in the job**, and that is what makes the [spec](../Specs/Render%20Settings.md)'s three endings one rule rather than three cases: unchecking the layer takes it out of the job, editing the range so it never renders takes it out too, and typing a count sets every per-layer count aside so nothing renders at its own any more. Hand the count back and the line returns. Where more than one layer overrides, it stands until none of the ones still rendering does.

**A layer whose state at some frame in the range is not known counts as still rendering.** The [architecture](../Architecture/Render%20Settings.md) V12 has one case the reading cannot answer — a switch driven by a driver or by a stored script, at a frame outside the scene's own range — and there the page cannot say the layer never renders. Dropping the line on a *not known* would be Smoozy calling the job clean because it failed to look.

## The picture size

**The percentage is the setting; the pixels are a reading beside it.** The [Render Settings spec](../Specs/Render%20Settings.md) settles that the figure cannot be typed into, dragged or overridden — it moves as the percentage moves and never the other way about — so it is set in the fixed-width face, unboxed, with no control on it. A field and a reading side by side have to look nothing alike, or the user will try to type in the wrong one.

**It reads *1920 × 1080* and nothing else.** No label saying *pixels*, no arrow, no *=*. The number beside a percentage box is the size, and a word explaining that is a word the user reads once and skips forever.

**Where there is no size to work out, the reading says so rather than going blank.** It reads *size unknown*, in italic — where the file has not been read, because the scene's full size lives inside the `.blend`; and where the user has typed a percentage Smoozy will not take, because there is nothing to take a percentage with. Same words either way, which is the same shape every other unknown value takes on this page.

## The render engine

**Cycles and EEVEE are the choice, and a file naming anything else is named beside them rather than refused.** The [spec](../Specs/Render%20Settings.md) is firm on both halves: the page offers two, and it does not pretend a file cannot say something else.

**So the control has a third row that is not a choice.** Above Cycles and EEVEE sits the file's own engine, by name, marked as the file's in the same italic grey every other file value uses — and it is what the setting is on until the user picks one of the two. Picking Cycles or EEVEE is an override like any other and carries the cross that hands it back.

**Nothing says the file's engine is unsupported, because it is not.** Leaving it alone renders with what the file named. What the page must not do is hide the name, which would leave a user looking at a Cycles-and-EEVEE control that is on neither.

## What the render runs on

**GPU or CPU, as two buttons rather than a dropdown**, opening at the scene's own value like everything else. Two answers do not need a list, and the [spec](../Specs/Render%20Settings.md) makes this a setting the user has to be able to *see* rather than hunt for — a file saved elsewhere may say processor, and the machine was rented for its cards.

**Under EEVEE the pair is not shown at all.** EEVEE only renders on the cards, so there is nothing to choose; a greyed pair would invite the user to wonder what they had done wrong.

**Where the setting stands at CPU on a server with GPUs, the CPU option carries a tag and nothing else is said.** *2 GPUs unused*, beside the option, in the same small tag a View Layer's own sample count uses. **The setting as it stands is the user's own choice where they have made one, and the scene's own value where they have not** — the [architecture](../Architecture/Render%20Settings.md) D5 and D1. Either way the file has to have been read, so the tag never shows on a page nobody has read.

**The tag has a second face, and two states where it says less.** The [architecture](../Architecture/Render%20Settings.md) D6 writes it across the same two axes as everything read from a server. Where the reading was taken on a connection that is not live now, the tag says what was last seen and when — *2 GPUs unused, last seen 12 Aug 14:02* — never the present tense. Where the machine's cards are present but could not be read, the tag states presence with no count — *GPUs unused* — because no count was read, in both faces. And where the reading says the machine has no cards, or could not establish either way, there is no tag at all.

**It was a sentence and the sentence was cut.** *This scene renders on the CPU. The server's two GPUs will be idle, and you are charged for them* told the user three things they could already see: the control says CPU, the server list says how many GPUs it has, and a rented server is charged for whether or not it is busy. The [spec](../Specs/Render%20Settings.md) asks Smoozy to say so, and a three-word tag says so. Label: **Creator-stated** (2026-08-13).

**A tag rather than a warning, for the reason the spec gives**: choosing the CPU is a real choice, so this states a fact and does not suggest changing anything.

**Where the pair stands at GPU under Cycles and the server's driver is too old for it, from Blender 5.3, one amber line sits under the pair**: *This server's NVIDIA driver, 575.64.03, is too old for Cycles GPU rendering in Blender 5.3.* It is in the same style as the line under the samples box, and it names the driver's version and the picked Blender's line. It says nothing about what to do, and it stops nothing: *Render* stays as it was. Under EEVEE and on CPU it is not there, and a server Smoozy can update does not draw it, because there the job waits for the update. *Added 2026-11-02, the build runner's ruling out of the first 5.3 live pass ([architecture](../Architecture/Render%20Settings.md) D6a). No auditor has seen it.*

**And the line cannot promise the other way round either.** A scene asking for the cards is no promise it gets them — the [spec](../Specs/Render%20Settings.md) has a machine left as it comes rendering on the processor with Blender saying nothing. So there is no matching green line saying the cards will be used. Where the job is over, [Queue & History](Queue%20&%20History.md) says what actually did the work, and that is the only place it can honestly be said.

## Warnings

Each is one line, and each names Blender's own settings by name rather than describing them. The reader owns Blender.

- **A typed sample count**, and a View Layer rendering at its own — both set out above rather than here, the first being one half of what the samples box says and the second the other.
- **A number Smoozy will not take** turns the field red and says the range, and says what Blender would otherwise have done silently.
- **The same picture written twice** is one line naming the two places — *Beauty pass written twice — a File Output node and the output path.* — **and only where both write images.** Where the output path writes a video it is not drawn; [What it will write](#what-it-will-write) names the two outputs instead. *(Changed 2026-10-12, Creator-stated: the Creator's eighth notes, note 4, and their answer that day. The bullet read "is one line naming the two places", with no words and no exception. No auditor has seen it.)* *(The node's name capitalised as Blender writes it, 2026-10-12, by the build runner, to match the line built by E4's redo. No auditor has seen it.)* **The line names the places the read found, in the same shape** — *Beauty pass written twice — two File Output nodes.*, *— two inputs of a File Output node.*, *— a File Output node and the output path.*, and *Beauty pass written three times — …* where there are three — so it never names the output path where the output path writes none of it. *(Added 2026-10-12, the build runner's ruling that day, "The duplicate-picture warning names what it found", under Rules 8 and 9; the words above, with "a file output node", were written for the one pair and fired for every pair. No auditor has seen it.)*
- **A file whose dependencies were never checked** — the fullest of the four, and it is set out below rather than here.

## Scenes

The scenes are boxes. **All scenes sits apart, under a rule and in bold**, because it is not a fourth scene — it is a different act that submits one job per scene, and its tag says how many.

**After a read, the scene the file was saved showing is ticked** — or, where the read could not name it, the first scene. Reading the file again keeps the user's ticks that still name a scene in the file, and ticks the saved one where none is left. A page opened from a history entry keeps the scene it arrived with. So the page always stands in the scene that renders. *Added 2026-10-03, the Creator's note 5 on the test copy: "The file's active scene should be selected by default." The [spec](../Specs/Render%20Settings.md) left which scene to Design. Design conforming to Architecture under Rule 10: `Render Settings` J7's third naming act. No auditor has seen it.*

### More than one ticked

**Everything but the frame range greys, and a greyed control shows no value.** The [spec](../Specs/Render%20Settings.md) is exact about why: with several scenes ticked there is no one scene to read from, and *Smoozy does not put a number on the page for scenes the user has not looked at*. So the control keeps its place and its label, greyed, and where its value sat there is nothing.

**A blank here is not the blank this document already refused.** [Before the file has been read](#before-the-file-has-been-read) an empty box was rejected because it reads as zero or as a control that failed to draw. That argument was about a live field. A greyed control is visibly not taking anything, so a blank in it reads as *nothing to say* rather than as a value — and the em dash cannot be borrowed, because on this page italic already means *the file's* and these settings do have file values, one each, which is exactly the thing that cannot be shown as one number.

**One line says why, once, in the band above the columns.** *3 scenes ticked: 3 jobs. Each uses its own settings; only the frame range is shared.* That band is already where this page says what state it is in — the unread page and the re-submission both speak from it — and repeating the sentence beside six greyed controls is the mistake the dashed page refused.

**The View Layer boxes are absent rather than greyed, because there is nothing to grey.** They are names out of one scene, and with several ticked there is no scene to take names from — the [architecture](../Architecture/Render%20Settings.md) J3 has no group entry for a pin for the same reason. Greying the last scene's names would be drawing another scene's layers. The rule that separates the two cases: a control holding a **value** stays and empties, a group holding **names** goes, which is the same rule this page already follows before a read.

**What the job will write goes with them**, and for the same reason: it is the read of one scene, and there is no scene in hand. One line stands in its place saying so. No group total is drawn — nothing above this document has asked for one, and a figure summed across scenes is a claim no spec makes.

**The range field itself is live and empty, and that is not the same blank again.** There is no one range to show — each ticked scene has its own, and the [architecture](../Architecture/Render%20Settings.md) J3's group entry does not exist until the user types one. So the field is the one control on the page offering something rather than reporting something, and it is empty because typing a range is the offer.

**The frame count is what stops that empty field reading as nothing**, and it reads for the group, which is the [spec](../Specs/Render%20Settings.md)'s own rule: left alone, each ticked scene's own range added up; typed into, the typed range once for every ticked scene. So it reads *720 frames across 3 scenes* — the total alone would look wrong beside an empty field, and *across 3 scenes* is what says where the number came from.

**The Render control names the act rather than one job.** With three ticked it reads *Render 3 jobs, 720 frames*, which is the same point the *All scenes* tag already makes: this is one job per scene, and the count is the whole set's.

**Hand everything back stays live**, having ~~two~~ three things left to do — clear the range so every scene uses its own again, return the Blender version to the automatic match, and tick the scene the file was saved showing, alone. *(The third added 2026-10-08, the [spec](../Specs/Render%20Settings.md)'s change from the Creator's fifth notes, note 4.)* The greyed settings have nothing to hand back, never having been typed into.

## What it will write

Once the file has been read: each destination, how many files, and **which format each writes**. Then the total, split by format. A total that says only "720 files, EXR and PNG" answers nothing, since it does not say whether that is 720 of each.

**The counts are for the frames the job will render**: with a range typed, only those frames are counted, here and in the download box. Where a typed frame lies outside the frames the read covered, no count is shown and one line says so: *Frames outside the file's range can't be counted before rendering.* *Added 2026-10-17, the Creator's tenth notes, note 1. No auditor has seen it.*

**Where the file writes a video, the range takes one unbroken run.** A list with gaps ~~or a step other than one~~ is refused under the range field (a step is kept; corrected the same day), as any unusable range is, with *A video can't skip frames. Type one unbroken range.*, and the field keeps its last good value. Added 2026-10-17, the build runner's ruling on the Creator's eleventh notes, note 1 ([spec](../Specs/Render%20Settings.md), *The frame range*). No drawing change. *No auditor has seen it.*

**Each destination is its folder as it will sit under the job's folder** — `PNG/`, `EXR/`, *top of the job's folder* — never the path the file records (`//../Render/PNG`), which is not where the files go. **The folders Smoozy redirects caches and bakes to are not listed**: they still come home, but they are not where the artist's work goes. **Formats are named in Smoozy's words** — *PNG*, *JPEG*, *EXR*, *Multilayer EXR*, *Video* — from one table, never Blender's own setting names such as `OPEN_EXR_MULTILAYER` or `FFMPEG` (Rule 9); a format the table does not hold shows as the file records it. *Added 2026-10-04, the Creator's notes 14, 16 and 18 on the test copy: the page showed the file's own paths, every cache folder, and Blender's setting names. Design conforming to Architecture under Rule 10: `Job files and outputs` P11 as corrected that day. No auditor has seen it.*

**A destination that counts nothing is not listed.** Nine rows of *tmp/ 0 PNG* said nothing a user could act on and read as a fault. The rows are the destinations that owe a file for the frames this job renders. *Added 2026-11-02, the live pass on real production files. No auditor has seen it.*

**Above the list, one line says where the job's folder will sit**: *In /home/ubuntu/Render, in a folder for this job.* — the folder the file's own outputs share on that server, which is where the [What a job writes spec](../Specs/What%20a%20job%20writes.md) puts it. **Where the spec falls back, the line says so instead**, in one of two sentences: *Your file's output folder is on another computer, so the files go to Smoozy's folder on the server.* or *Your file's output folder can't be used, so the files go to Smoozy's folder on the server.* Neither blocks Render. **Where the server has a fallback folder set** ([Servers](Servers.md), the Edit dialog) **and nothing known before submitting rules it out, either sentence ends with that folder's path instead** — *…so the files go to /mnt/renders.* — because *Smoozy's folder* would then be false; where that folder fails at submission, the queue entry's path shows where the files went. *Added later on 2026-10-04, on the Creator's want to choose the fallback folder; `Job files and outputs` P23. No auditor has seen it.* **Corrected the same sitting, by the build runner's ruling under Rule 8: where a fallback folder is set and something known before submitting rules it out, a second line sits under the sentence** — *This server's fallback folder can't be used, so the files go to Smoozy's folder.* — the same words the queue entry uses, and the sentence above keeps its *Smoozy's folder on the server* ending. Where it fails only at submission, the entry says it. *Added 2026-10-04, on the Creator's ruling of that date (their note 18). Design conforming to Architecture under Rule 10: `Job files and outputs` 6.6. No auditor has seen it.* **The line never names a folder the machine lacks at its top.** Where the first folder of the path is not on the server, the line is the first of the two sentences, as the job's entry will say. Where the reading comes from an older server program and cannot say, no *In …* line is drawn. *Added 2026-11-02, the live pass on real production files; Design conforming to Architecture under Rule 10: `Job files and outputs` P21's note. No auditor has seen it.* **Corrected later on 2026-10-04, by the drawing pass: on this page the second line is shorter than the queue entry's — *This server's fallback folder can't be used.*** The sentence above it already ends in *Smoozy's folder on the server*, so the queue entry's ending would say it twice. The queue entry keeps its longer words, since its first line says only *here*. *No auditor has seen it.*

~~**On a file with more than one scene, the read does not fill this group.** The read covers the scene the file was saved showing and does not say which scene that is, so a list drawn from it could be another scene's outputs under the one ticked. The group stays as it is before a read, and the job still writes what the file says. Label: **settled at Design** (2026-09-29), ruled by the build runner under the Creator's standing delegation.~~

**On a file with more than one scene, the read fills this group for whichever scene is ticked.** With several ticked, the group stays as it is before a read; each job still writes what the file says, and its queue entry says so once the render has read it. *(Changed 2026-10-10 on the Creator's seventh notes, note 1; Design conforming to Architecture under Rule 10: `Job files and outputs` R4b. It said the group filled only where the scene the read named was the one ticked, because the read covered only the scene the file was saved showing. No auditor has seen it.)*

**The download box says the same where the file was read but not for the scene ticked**, rather than borrowing the *not read* sentence, which would be false: *Only the active scene was read; the selected scene's formats are unknown. Everything downloads.* Where several are ticked it reads *the selected scenes' formats are unknown*. It offers no *Read it now*, because a read would cover the same scene again. This covers both cases above: another scene ticked, and a read that could not name its scene. *Added 2026-10-08, from the Creator's sixth notes, note 2; the group's rule above is unchanged. No auditor has seen it.* Only a reading from an older machine program, which holds one scene, can still lead to it. *(Added 2026-10-10 on the Creator's seventh notes, note 1. No auditor has seen it.)*

**Three more things the read found sit under that list, as lines rather than warnings** *(two until 2026-10-03, when the views line below joined them)*. All are facts about what this job writes, they come from the same read, and none asks the user to do anything — so they belong with the list they qualify. The duplicate-picture line already sits there and these join it.

**A compositor shared with another scene.** *Interior uses Exterior's render, so it renders too, at its own settings.* The [spec](../Specs/Render%20Settings.md) has all three parts said before the job goes: which scene pulls in which, that a single-scene job can be much bigger than the scene it names, and that the pulled-in scene renders at the file's values rather than at this page's. **Where both scenes are ticked a second line says the doubling** — *Exterior is also ticked, so it renders twice: alone and inside this job* — because that is a different fact and only true of that case.

**A single video file, and what stopping it costs.** *This job writes one video file, not one file per frame. Blender may split a long render into several files.* Then the cost: *If stopped, the whole range renders again.*

**A video and frames are two outputs, not one picture written twice.** Where the output path writes a video and the beauty also reaches a File Output node, the duplicate-picture line is not drawn. One line names each output as what it is — *Frames from a File Output node, and a video from the output path.* — and the list above already gives each its own row, the video's reading *Video*. The single-video line above is not drawn either, since the video is not all the job writes. **Where both write the same images, the duplicate-picture line stands in its old words**: *Beauty pass written twice — a file output node and the output path.* *Added 2026-10-12, Creator-stated: the eighth notes, note 4 — the line called the output path's video a beauty pass — and the Creator's answer that day, "a video should not be thought of as a pass at all", choosing to name each output plainly with no twice line. A wording drawn earlier that day, "Render written twice — a File Output node and the output path.", was withdrawn on their word. Drawn as `ow-vidfr`. No auditor has seen it.*

**More than one view, and what Smoozy cannot count.** *This scene renders several views, so its frames can't be counted or checked.* The job still renders as asked; the line is the same one its queue entry carries ([Queue & History](Queue%20&%20History.md), *What the job writes*). *Added 2026-10-03, ruled by the build runner under the Creator's standing delegation (hole 15 of the map of the 38 promises): Smoozy carries such a job. Design conforming to Architecture under Rule 10: `Job files and outputs` I9a.*

**That cost is a re-run's and not a loss, which is the correction Rule 10 made here.** The [spec](../Specs/What%20a%20job%20writes.md)'s own words — *an interruption costs the whole range rather than the frames since the last one* — draw naturally as *you lose everything*, and the [architecture](../Architecture/Job%20files%20and%20outputs.md) C34 says otherwise: the interrupt Smoozy sends leaves the file holding the frames that were finished. What the whole range costs is the second run, because I22 switches the already-there machinery off for a video-shaped job and there is nothing frame-shaped on disk to skip.

**Two of what the read finds are warnings rather than lines, and they are the two that hold a job.** [What a job writes](../Specs/What%20a%20job%20writes.md) settles that a job writing nothing at all, and a job whose destinations collide, stop the render before any machine time is paid for. Everything else under this heading asks nothing of the user; these two do, so they take the warning's amber and sit above the destination list rather than under it. **Neither blocks Render**, which the [spec](../Specs/What%20a%20job%20writes.md) is explicit about: Smoozy warns and changes nothing in the file, because renaming an artist's outputs would mean deciding what their file meant. Label: **settled at Design** (2026-08-25).

**A job that will write nothing says so and says where to fix it.** *This job writes no files. Fix the output settings in the file.* **The second sentence is where to look**, and it is the only place this can be fixed. **The destination list is not drawn at all in this state** — there is nothing to list, and a heading with an empty list under it reads as a Smoozy that failed to look.

**A colliding pair names both destinations, which is the whole of its value.** *Two of this job's outputs write to the same file. One will overwrite the other.* Then the two paths. **Each of the two names where it comes from as well as where it lands** — *the scene's output path*, *File Output node Beauty* — because the path alone is the same string twice and tells the artist nothing about which two things to go and change. **That naming is what makes it act on**: they have to find both in their own file, and *two of your outputs collide* sends them hunting. The Creator has been caught by this in their own work, which is why it is a warning at all.

**Both read shorter on a queue entry, and that is the same rule the failure shape follows.** [Queue & History](Queue%20&%20History.md) draws them where a job was submitted without being read, and there they carry the choice — *Stop this job* or *Continue anyway* — because a render is about to start. Here there is no job yet, so there is nothing to stop and no choice to offer: the user fixes the file or presses Render.

**A machine that is killed outright rather than stopped is left out of the line, deliberately.** The [architecture](../Architecture/Job%20files%20and%20outputs.md) C34 has an outright kill leave the file at its token size with nothing in it, which is worse than anything the line says — but the [spec](../Specs/What%20a%20job%20writes.md) asks for one statement about stopping, this strip is not the place to list the ways a rented machine can stop, and [Queue & History](Queue%20&%20History.md) says what a job actually left. It is written down here so nobody reads the omission as an oversight.

## The dependency warning

**It sits in the right-hand column, with what has not been checked, and it is a band rather than a line.** The right-hand column is where this page puts what has not been checked, which [the body is three columns](#the-body-is-three-columns) already settles. Three ways on hang off it, and a one-line note with three controls after it reads as a line that has gone wrong. The [Rendering spec](../Specs/Rendering.md) blocks none of the three, so none of them is a button the user has to get past.

**Three sentences, and which one depends on why the warning is there.** Dependencies not checked on this server; or a check was run and the file has been replaced since; or a check was run with a Blender this job does not use. The [spec](../Specs/Rendering.md) is firm that these must not read alike — the second and third are the ones that matter and would otherwise be the sentence the user has clicked past twenty times. So the replaced case says plainly that the new upload put the warning back.

**The version case names both Blenders in one sentence.** *Dependencies were checked with Blender 4.5, not the 5.1 this job renders with.* **One sentence rather than two, and it carries both halves the [spec](../Specs/Rendering.md) asks for**: the check happened, and it does not cover this Blender. Naming both versions is what makes it act on: the user picks the version at the top of this same panel, so the two numbers are the whole of what they need to decide whether to check again or carry on. **Saying the check stands is the other half** — without it, a user who ran a check five minutes ago reads this as Smoozy having lost it. Label: **Creator-stated** (2026-08-19) for the case; **settled at Design** (2026-08-25) for the wording, which the [spec](../Specs/Rendering.md) leaves here.

**It takes the same way on as the replaced case — *Check dependencies…*** — and not the *Open dependency check* of the case below. Both are a check that exists and is stale, and what answers a stale check is a new one; *Open dependency check* belongs where there are findings to act on, which a check that placed everything does not have.

**Three ways on, in the order of how much they cost**: check the assets now, submit with the first-frame check, submit anyway. Checking first is the leftmost because it is the one that actually answers the question; submitting anyway is rightmost and is an ordinary control, not a red one, because a user who knows their assets are up there is doing nothing wrong.

**The first-frame way names what it buys.** *Render one frame first* with a line saying assets that did not load then cost a frame rather than a night. Without that line it reads as a third button doing something unexplained.

**It is not offered where the user has already asked for the check below.** The [spec](../Specs/Rendering.md) says it is the same thing they already have, so the band drops to two ways on.

**On a single-frame submission the band leaves the first-frame way out too**, and drops to two ways on, for the same reason. *Added 2026-10-27, the Creator's nineteenth notes, note 8; no auditor has seen it.*

**The fourth sentence is a check that ran and could not place everything, and it replaces the other three rather than joining them.** The [Dependency Upload & Remap spec](../Specs/Dependency%20Upload%20&%20Remap.md) is explicit that such a check does not stop the warning — it gives way to a specific one naming how many pieces are missing, which is worth more than either silence or the generic wording. So it is the same band with different words, not a fourth thing on the page, and the [architecture](../Architecture/Rendering.md) W4 keeps it one warning with one switch: two would let a user who switched the generic one off meet this one anyway.

**It says how many, and says it about the server rather than about the search.** *3 assets this file needs are not on the server.* The [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) V12 has that number cover four different findings — not found, could not be sent, could not be fixed, and found on your computer and never sent — and the one thing true of all four is that the render will not find the asset. *Could not place* would read as *Smoozy failed to look*, which is wrong for three of the four.

**Rows the user set aside are not counted**, per the [spec](../Specs/Dependency%20Upload%20&%20Remap.md): setting one aside is the user saying they know and do not care, and counting it would leave the number standing for ever.

**The leftmost way on changes its words, because a check has already run here.** It reads *Open dependency check* rather than *Check the assets* and goes to the rows themselves, which is where the three findings can be acted on. The other two ways and the switch-off are unchanged — the [spec](../Specs/Dependency%20Upload%20&%20Remap.md) has this warning block nothing and be dismissible exactly as the generic one is.

**Switching it off is on the band, and it says what it does not cover.** It switches the warning off across the whole app, not for this file — and the [spec](../Specs/Rendering.md) settles that restoring it later brings back the switched-off warning and not the case where Smoozy believes the file was checked. So one line: *Don't warn me again*, and beneath it, where it is restored.

## Submitting

Along the bottom: **Name this job**, with one line saying where the name turns up; the three things settled at submission — downloading as it renders, how much the job reports to the phone, and the first-frame check — and the Render control naming how many frames.

**A submission that went says so beside the Render control, in ordinary ink at the page's own text size, with a check mark: *Submitted to Vast 4090.*, or *3 jobs submitted to Vast 4090.*, and *Show in Queue* beside it**, which opens the queue with that job, or that group, chosen. It stays until something on the page changes or the user submits again, never on a timer. The Render control is unchanged, and a submission only part of which went keeps its own statement. **It takes no colour**: green means *finished*, and a job just sent has not finished; the link answers the user's next question, *where is it?* *Added 2026-10-03, the Creator's note 10 on the test copy: "The little Submitted. text is almost invisible. The user should get some other visual feedback … Think of something." The drawing had no state for it. Settled at Design by the build runner; no architecture answer touches it. No auditor has seen it.* *(From 2026-10-17 it sits under the Render control, right-aligned in Render's column, rather than beside it: the Creator's tenth notes, note 8, under [How the three sit together](#how-the-three-sit-together). No auditor has seen it.)*

**While the dependency check's uploads for this file on this server are running or waiting, one line sits directly above the Render control: *Uploading dependencies: 4 of 12 files.*** It is a link that opens the check. It counts this file's check on this server only — files uploaded so far, of all it is sending — read from what [Transfers](Transfers.md) lists and never counted a second time, and it goes when the last file lands. ~~**Render stays available**: the [Rendering spec](../Specs/Rendering.md) has submitting start no transfer, so the line says what is happening and holds nothing.~~ *Added 2026-10-02. **Creator-stated**, note 6 on 0.1.8: "When dependencies for a given job are being uploaded, there should be some indication of that where the user would start the render job." The [check](Dependency%20Upload%20&%20Remap.md)'s strip reads the same count.* *(From 2026-10-17 the line sits on the foot's third line, under Render, rather than above it: the Creator's tenth notes, note 8, under [How the three sit together](#how-the-three-sit-together). No auditor has seen it.)*

**Render is held while that send runs, and until the check has answered for a linked `.blend` that landed** (the [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) A9e, the build runner's ruling, 2026-11-03: a job pressed then finished with no libraries and looked as if it had worked). The control is greyed and its label says why, with the line beneath it, a link to the check:

| While | The control reads | The line reads |
|---|---|---|
| The press is counting files | *Waiting for upload* | *Counting files…* |
| Files are going | *Waiting for upload* | *Uploading dependencies: 4 of 12 files.* |
| A library landed and its check runs | *Waiting for check* | *Checking dependencies…* |
| A library landed and no check is coming | *Check dependencies first* | *Check dependencies…* |

A check that walks with none of these standing holds nothing, and neither does a check after a send of pictures alone. A press that arrives anyway answers *Not submitted.* and the line's reason. **The drawing is not changed**: these are words on the greyed control and the line it holds. No auditor has seen it.

**Where the first frame cannot be named** — nothing read and no range typed, or a file of several scenes with none ticked and no range typed — *Render one frame first* answers **Not submitted. Type the frames or tick a scene.** Where nothing was read it says the first half: **Not submitted. Type the frames.** *Added 2026-10-01, from step 10c's pass before Vast on Sesterce.*

**Where every job the Render control would send has one frame** — one scene whose range is one frame, or several scenes of one frame each — *Render one frame first* is greyed and cannot be pressed, because the check has nothing to save. It keeps its setting, so it comes back as it was when the range grows. **While greyed it reads off, to the eye and to a screen reader, whatever was set before; the setting is kept unseen and comes back when the range grows to more than one frame — on only if it was on before.** *(Added 2026-10-28, Creator-stated, the twentieth notes, note 3: "render one frame first should be grayed out in an off state … If and only if it was on before that, then choosing more than one frame should then restore the ticked checkbox." Until then the greyed switch showed the kept setting, on or off. Drawn as `res-oneframe`, with a one-frame range typed. No auditor has seen it.)* What is sent does not change: the [architecture](../Architecture/Rendering.md) S13 already sends a one-frame range as one plain job. *Added 2026-10-27, the Creator's nineteenth notes, note 8; no auditor has seen it.*

**Downloading as it renders is three decisions, not a checkbox.** On or off, where the files go, and which formats. All three are chosen here because this is where the user is and where the file has just been read — the format list can only be built from a file that was read, and this is the moment it has been.

**The switch starts off on a fresh page, including Submit again and Resume.** The download choices appear when the user turns it on. *Corrected 2026-10-25 under Rule 10, step 12's SC7 findings: the drawing started it on, against [Download](../Specs/Download.md)'s opt-in rule. The download-choice examples and the no-folder warning show it after the user has turned it on. No auditor has seen this correction.*

**Two of the three can be changed afterwards, and the page does not pretend otherwise.** The [spec](../Specs/Render%20Settings.md) keeps narrowing available from the queue once the job has started, and [Rendering](../Specs/Rendering.md) lets the switch itself be turned later on a job already submitted. So nothing here says *fixed for the life of the job* — it was said once and was wrong. Label: **Creator-stated** (2026-08-13).

**Where the files go is the one settled at this moment, and *settled* is not the same as *never asked again*.** The [architecture](../Architecture/Download.md) R11 has the folder fixed for the life of the job — nothing can be asked of a user who is not there — and moved only by switching the arrangement off and on again, which resets it and asks once more. So the panel says it is fixed once the job is submitted and does not say it can never be chosen again, which would be false. **This document said *cannot be asked about later* until 2026-08-23; Rule 10 corrected it.**

**Where Smoozy can name no folder at all, the strip says so, and the job still goes.** Three things can name the folder — the one chosen for this job, the standing one from Settings, and the one the system proposes — and all three can be empty at once: the [architecture](../Architecture/Desktop%20app%20and%20delivery.md) W5 has a route that returns nothing propose nothing rather than compose a path, and a local record that will not answer holds no standing folder either. With the switch on and none of the three, there is nowhere for the frames to land, and [Download](../Architecture/Download.md) R1's arrangement is not made. **Before this was drawn, that was a silence**: the job went to the machine with *Download frames as they render* on, no arrangement was made, and nothing anywhere said the frames would never come home.

**So an amber line sits under the summary control, saying the thing and then what to do about it.** *No download folder chosen.* Then: *Frames will stay on the server. Choose a folder above.* The second half says what it costs and what to do, and the control that fixes it is the one directly above the line. **It sits under whichever of the two controls is showing**, the summary after a read and *Download folder* before one, because the folder is the one of the three chosen at this moment either way. *(From 2026-10-17 it is on the foot's third line, under the switches: the Creator's tenth notes, note 8. No auditor has seen it.)*

**It is a line and not a block on Render.** [`§ Before the file has been read`](#before-the-file-has-been-read) already has submitting work with nothing read at all, and the two output warnings above hold a job without blocking it. **The [architecture](../Architecture/Download.md) H3 is kept rather than bent**: it has the switch-on ask the user for a folder before *it* proceeds, and what does not proceed is the arrangement, not the render. The asking is the control on the strip and the panel it opens, which offers *Somewhere else…* whatever the system proposed. **Choosing *Somewhere else…* puts the system's folder panel up at once**, and the folder chosen there is shown in the box as the choice, which *Done* then keeps like any other. Closing the panel leaves the box as it was. *Added 2026-09-30, from the Creator's press on step 10c's eleventh pass: the panel waited for Done, and they found it "a little confusing". Settled at Design by the build runner; no architecture answer touches it.*

**Refusing the switch was the alternative, and it is worse.** A switch the user turned on and Smoozy turned off again is a second silence, and [Download](../Architecture/Download.md) R1 already has *there is nowhere to put it* as a state rather than as an error. Saying it plainly leaves the decision where it belongs. Label: **settled at Design** (2026-09-20), under the Creator's standing delegation.

**With nothing read, the format list is not offered at all and says why.** It is built out of the file, so before a read there is nothing to build it from — the control that picks formats is absent and one line says so, and that until then everything the job writes is downloaded. Where the files go is still chosen, because it is the one of the three settled at this moment.

**The panel is this page's, not [Transfers](Transfers.md)'s.** Transfers shows an arrangement that already exists and says what it was narrowed to; the choosing happens here, where the file's own format list is.

### How the three sit together

**The foot is two lines, every group aligned to the top.** Left: the name, its hint under it. Middle, first line: the download switch with its summary control beside it; second line: Render one frame first, then Telegram and its dropdown, as wide as its longest choice. Right: Render, with the submitted line under it in the same column. Lines that say something is wrong or under way are a third line, drawn only then. *Added 2026-10-17, the Creator's tenth notes, note 8; drawn for the Creator to see before it is built. **Seen and approved by the Creator on 2026-10-17** (*"Yes, the footer is better, thank you."*), shown at two window widths.* The note that the formats wait for a read sits on that third line too, and at a narrow window the switches drop under the name before anything else moves. *(Both added 2026-10-17 by the drawing pass for the tenth notes; the drawing's state is `res-foot`. No auditor has seen it.)*

**The first line keeps its height whether download is on or off.** The summary control beside the switch is taller than the switch's own line, so it overhangs that line rather than growing it: turning download on, or choosing a folder, moves neither switch nor anything under it, at any window width. *Added 2026-10-28, Creator-stated, the twentieth notes, note 2: "Checking the download frames as they render checkbox nudges the lines slightly." A fault against [Whole app](Whole%20app.md)'s rule that nothing already drawn moves when something appears beside it; runner-ruled. Drawn as `res-foot` (off) and `res-footdl` (on) at the same width. No auditor has seen it.* **It holds in both of the foot's forms.** Where the window is narrow enough that the switches drop under the name, the second line sits far enough below the first that the summary's overhang leaves the same gap above it as in the two-line form: the summary never touches the Telegram control, and turning download on still moves nothing. *(Added 2026-10-29, Creator-stated, the twenty-first notes, note 4, on the Creator's screenshot of the foot: "We fixed this at some point and it got worse as it turns out." The overhang was measured beside Render only; in the narrow form it reached 1 px into the second line. Drawn as `res-footdlnarrow`, the foot with download on at a 1100-pixel window. No auditor has seen it.)*

**The switch is on the strip; the other two are one control under it, opening a panel.** *(From 2026-10-17 that control sits beside the switch, on its line, by the paragraph above: the Creator's tenth notes, note 8. No auditor has seen it.)* On or off is the decision a user makes without thinking, so it is a switch in the open. Where the files go and which formats are two decisions that are only worth reading as a summary — *EXR and PNG — 600 of 720 files, to ~/Renders/kitchen* — so one control carries the summary and opens the panel where both are made. Three controls in the open would give the two quiet decisions the same weight as the one loud one, on a strip whose real subject is the Render control.

**The count in the summary is what makes the narrowing legible.** *EXR and PNG* alone says which formats and not what it costs; *600 of 720 files* says the choice is worth something. It is the [architecture](../Architecture/Download.md) N4's own honesty in one figure — what the narrowing passed over is still on the server and can be fetched by hand.

**On a job with only one format there is nothing to narrow, so nothing is offered.** The panel states the format rather than listing it, and the summary reads *240 EXR files, to ~/Renders/kitchen*. A list of one is a control whose only two settings are *everything* and *nothing*, and the second of those is what switching auto-download off already does. The [architecture](../Architecture/Download.md) N4 makes this exact: files with no format — a bake, a cache, an added write place — are outside a narrowing rather than excluded by it, so narrowing to the one format the job writes and not narrowing at all fetch the same files.

**Opened from a queue entry, where the files go is a fact and not a choice**, because it was settled when the job was submitted: the panel says it in one plain line — *To ~/Renders/kitchen* — and draws no button for it. The formats stay a choice there, and the sentence saying they can be changed later from the queue is not shown where the panel already is the queue's. *Added 2026-10-04, the Creator's note 16 on the test copy: the box drew the one fixed folder as a radio button. Settled at Design by the build runner; no auditor has seen it.*

**With nothing read the panel is the same panel with its format half gone**, saying so and saying everything downloads until then, with the way to read the file beside it and the note that narrowing can be done later from the queue. That last half is the [architecture](../Architecture/Download.md) N3's: an arrangement made with no list covers everything, and *everything* is no filter rather than a frozen list, so the list arriving later invalidates nothing.

**Added 2026-10-17, the build runner's ruling on the tenth notes (piece U2), Rule 8: where the box has no list to count — the file not read, read for another scene, or a typed frame outside the read — and a narrowing of the formats still stands, the box names that narrowing (*EXR and PNG files are downloaded.*; *No format is downloaded.* for a narrowing to none) instead of *Everything downloads.*, which would be false. The queue's box does the same while a job's reading has not landed.** *No auditor has seen it.*

**How much the job reports to the phone is chosen here too**, beside the download choices, because it is settled at the same moment and for the same job. It belongs to [Telegram](../Specs/Telegram.md).

**Settled at Design (2026-08-23), drawn in the mockup.** It is a quiet line on the submit strip under the download choices — the word *Telegram* and one of the three choices — rather than a question, because a default is carried and submitting normally asks nothing. Where Telegram is switched off it is not there at all.

**Where the server is connected and does not have Telegram reporting, the line reads *Telegram  Not installed  Install*, in place of the choices.** The words stand in the control's own width, so nothing beside them moves, and *Install* says *no restart* on hover, as the card says it. It is the card's own action ([Server Setup](Server%20Setup.md)). While it runs or waits its turn the line reads *Telegram  Installing…*; where it fails, the failure's sentence is the foot's third line and *Install* comes back; where it lands, the choices come back. **Where Smoozy cannot tell whether the server has it, the choices stand as they always have**, and nothing is said. With the first-frame switch on, *Answer the first frame in Queue.* is drawn as it is for *Off*. *(Added 2026-11-03, the build runner's ruling, the second 5.3 live pass. The drawing is not changed: these are words on a shape it holds. No auditor has seen it.)* *Words changed 2026-11-06 (was *Install it*): the last words read before the release. No auditor has seen it.*

**A fourth choice, per job only: a picture every few minutes.** *Added 2026-10-19, the Creator's want raised during the live pass: "a choice that the user can manually type in the number of minutes is the way to do it, not some presets." What it does is the [Telegram spec](../Specs/Telegram.md)'s, under* Fewer pictures, on a clock*. No auditor has seen it.*
- **The list** is Smoozy's own and opens upward, since the foot is the bottom of the window ([Whole app](Whole%20app.md)). Its rows: *Everything*, *Pictures only*, *A picture every few minutes*, *Alerts only*. The new row is third because it sends fewer pictures than the two above it and more than the one below.
- **Picked, the control reads *A picture every*, and the minutes field and the word *minutes* follow it on the same line**, so the line reads *Telegram  [A picture every ▾]  [ 5 ]  minutes*. The control keeps its width, so nothing beside it moves when the choice changes. The field is a plain number field, right-aligned, upright in ordinary ink (the value is the user's or Smoozy's, never the file's), 5 to begin with. It is there only while this choice is picked.
- **Minutes Smoozy will not take** — empty, not a whole number, or outside 1 to 60 — turn the field red, the way any number this page will not take does, and the foot's third line says ***Whole minutes, 1 to 60.*** in the refusal colour. The minutes that stood are what the job carries, and Render still submits.
- ~~**Settings keeps three.** The default there is the three radios; this choice is set for one job at a time.~~ **Settings holds all five from 2026-10-31**, minutes included ([Settings](Settings.md)), and the list starts from that default, as it starts from every default. *(A choice from the phone becomes the default: `Specs/Telegram`, the Telegram discussion of 2026-10-31. No auditor has seen it.)*

**A fifth choice, per job only: *Off*.** The job sends nothing to the phone by itself — what it means is the [Telegram spec](../Specs/Telegram.md)'s, under *Off, for one job*. *Added 2026-10-28, Creator-stated, the twentieth notes, note 4: "There should be an option to report nothing at all in Telegram for a given job." The word and its place runner-ruled. No auditor has seen it.*
- **It is the list's last row**, under *Alerts only*, since it sends less than every row above it. Picked, the control reads *Off*, at the control's own width.
- **Where *Render one frame first* is on as well, the foot's third line says *Answer the first frame in Queue.*** in the same quiet ink as the note that the formats wait for a read, because the question that would have reached the phone does not, and the check still waits. It goes when either is changed.
- ~~**Settings keeps three** for this choice too: *Off* is set for one job at a time.~~ *Off* can be the default too, from 2026-10-31, as above.
- Drawn as `res-tglist` (the list open, *Off* last) and `res-tgoff` (*Off* picked, with *Render one frame first* on).
- **The queue entry and History name no job's reporting choice**, for this choice or the other three, so nothing is added there.
- Drawn as `res-tglist` (the list open), `res-tgmin` (picked, 5 minutes) and `res-tgbad` (0 typed).

## Keyboard shortcuts

**None of its own.** This is a panel of controls rather than a list of things, so there is no chosen row for a key to act on — and every act on it, submitting above all, is one this app asks about or does deliberately. The whole-app rules are in [Whole app](Whole%20app.md), and Escape closing the panel is one of them.

## What this document left open, and how it was answered

Every marker this document once carried is closed, and the answers are kept together here rather than folded back into the body. **Nothing below is still open** — the heading used to say *Left for later*, which was true when the markers were markers and stopped being true the day they were answered.


**Nothing.** Every marker this document carried is closed, and its keyboard shortcuts are settled in the section above. What remains is the stage's seams.

- **Settled at Design (2026-08-23), drawn in the mockup.** A band above the columns says the values came out of that job on that date rather than out of the file — which is the one thing the italic-grey and bold convention cannot say on its own, since both kinds are the user's. The band has a second shape for no server connected, and a re-submission whose own server is gone gets the file browser instead.
- **Settled at Design (2026-08-23), drawn in the mockup** — and it lands on the queue rather than here, because by then the submission is over. Three lists: submitted, not submitted, and not known, the last only while the connection is still down.
- **Settled at Design (2026-08-23), drawn in the mockup.** A re-submitted failed job's settings carry **Simplify**, a switch sitting with the render settings in the middle column. It belongs to [Rendering](../Specs/Rendering.md), which the Creator ruled goes through Design drawn rather than written, so the reasoning behind it is in the mockup's own notes.

**The link under Render does the act it names.** Under *Check dependencies first*, *Check dependencies…* starts a check; a link showing a check or upload already running opens its rows. The warning counts each real file once, by the same rule as the check panel, including failed and unsent files. Added 2026-11-04, the Creator's ruling through the Next frame and page presses brief; no auditor has seen it.


## Prepare simulations

Creator-stated, 2026-11-06. After a read, beside the submission choices, each scene whose reading finds a simulation covered by the playing rule has **Prepare simulations**, on by default. Several scenes show one named row per such scene. The row follows the scene's own range, including changes made to the group. With it on: **Plays frames 101–125 before rendering, on every attempt.** Use the job's actual lowest playing frame and highest playing frame, including any frames after the last requested frame needed by the saved shutter (`Rendering` L10b). With it off: **Unbaked simulations may render frozen or not match playback.** No row before a read, after a read is dropped, or for a scene the rule does not play. Each choice stays through retries, Resume, Submit again and both first-frame jobs. Reset all returns it to on. Nothing claims matching playback. *Words changed 2026-11-06 (was *Plays frames 101–125 before rendering. Every attempt.* and *Unbaked simulations may render frozen or different.*): the last words read before the release. No auditor has seen it.*

The drawing's simulation states show on, off, an earlier requested frame and several scenes. The existing unread state shows no simulation row.

When a first-frame check is on and the range has several frames, the line states both ranges: **Plays frames 101–121 for the check and 101–125 for the job, on every attempt.** Each uses that job’s own first frame and highest playing frame, including its shutter tail. *Words changed 2026-11-06 (was *Check: plays frames 101–121. Job: plays frames 101–125. Every attempt.*): the last words read before the release. No auditor has seen it.*

# Design values

*What Smoozy's colours, type and sizes are, what each one means, and where the numbers actually live. This document holds the meanings; the mockup holds the numbers.*

*2026-10-27 — Removed the withdrawn paid-access text and its obsolete references under the Creator’s free-and-open-source decision of 2026-10-04 (wave 5); retained protections and evidence keep their force. No auditor has seen this removal.*

## Where the numbers live

**The numbers live in one place: the token block at the top of [the mockup](Mockup/Smoozy.html).** It ends at a banner reading *COMPONENTS. Nothing below invents a colour* — everything above that banner is a token, and nothing below it invents a colour.

That is deliberate, and it follows the project's own rule that every fact has one home. A list of hex codes written out here would be a second copy, and two copies drift. So this document says what each token is *for* and what may use it, and the mockup says what it *is*.

**Change a value there and it changes everywhere.** Change one here and nothing happens.

## Colour, and what each one means

Colour is reserved. A screen may not use a colour to mean something new.

| Token | Means |
| --- | --- |
| `--ok` | Connected, a job that finished, a switch that is on, a setup step that was done *(added 2026-10-03, for [Server Setup](Server%20Setup.md)'s step marks)*, or a dependency already on the server and the same file *(added 2026-10-08, for the [dependency check](Dependency%20Upload%20&%20Remap.md), the Creator's sixth notes)* |
| `--busy` | Work in progress, or something needing the user's eye — in words and lines |
| `--busy-fill` | The same meaning in **every** filled shape: a dot, a bar, a mark, a tint |
| `--bad` | Broken, refused, destructive, or a dependency Smoozy found nothing for *(added 2026-10-08, the same)* |
| `--accent` | The accent: a link, a chosen thing — in words |
| `--accent-fill` | The same meaning in a filled shape: whole-job progress, a transfer |
| `--accent-solid` | The same meaning as a ground with words or a mark drawn on it: a button, an open menu, a count on an icon, a ticked box |
| `--bad-solid` | The same as `--bad`, as a ground with words on it: a button that removes or deletes |
| `--idle` | Nothing happening |
| `--key` | Blender's keyframe yellow — an animated property, and nothing else |
| `--lib` | A linked library — a row that is itself another `.blend` |
| `--alert` | Refused or blocked — a value the app will not accept |
| `--alert-bg` | The same meaning as a ground: the field, the message, the band across the window |
| `--b1` … `--b8` | Which server. Never anything else |
| `--tag-up`, `--tag-dn`, `--tag-dep`, `--tag-auto` | Which kind of transfer a row is. Never anything else |
| `--ok`, `--bad`, `--idle`, `--busy`, on a job's or a transfer's ending | How a finished job ended: green finished, red failed, grey stopped by the user, amber abandoned; a finished transfer's ending word takes the same green, red and grey *(widened 2026-10-10, the Creator's seventh notes, note 9; no auditor has seen it)* |

**A job's ending is a meaning of its own and is written as one line above.** Two of the four are not covered by those tokens' other meanings — a job the user stopped is not *nothing happening*, and a job abandoned is finished and needs nothing from the user, so it is neither of amber's. Widening either token would make it mean two unrelated things everywhere else it is used, so the ending takes a line instead. The colour is on the word, and the entry's status line repeats it on its dot. The dot is painted, so it takes the fill where the colour has one — an abandoned job has an amber word and a `--busy-fill` dot. [Whole app](Whole%20app.md) holds the decision; this is where the meaning is recorded.

**The badge palette is eight hues 45 degrees apart, around the whole wheel**: red, gold, green, jade, cyan, blue, violet, magenta. Nothing is left out, and the reason is the shape: a badge is a squircle and every state in Smoozy is a circle, so `--b3` green and `--ok` green can never be confused. The [whole-app Design](Whole%20app.md) owns that rule.

**The eight hues are the same in both themes, and that is a decision rather than an omission.** Every other colour here is given a light value and a dark one, because every other colour means something that has to read the same against two grounds. A badge means *which server* and nothing else, and a hue that shifted between themes would be a second thing to learn about a mark whose whole job is to be recognised at ten pixels. So `--b1` … `--b8` are declared once. Label: **Creator-stated** (2026-08-13).

**The four transfer tags are a family of their own, below the reserved colours.** Which way a transfer is going is not one of the meanings the five reserved colours carry, so the tags do not borrow them — a green tag on the one screen where green already means *finished* is a second meaning for one colour. They are one neutral hue at four lightnesses, dark to light: auto-download, upload, download, dependencies. The lightness says which kind, the arrow says which way, and nothing collides. Every one of the four holds white text at better than four and a half to one — which was true of the light set from the start and not of the dark one, whose four had drifted between three and six and a half until they were corrected on 2026-09-02. Label: **Creator-stated** (2026-08-13).

**Two earlier sets were wrong and the reason is worth keeping.** Both tried to keep the badge colours away from the meaning colours, which left only one arc of the wheel to fit eight hues into — so they came out as near neighbours nobody could tell apart at ten pixels, which is the one job a badge has. The fix was not a better arc; it was giving shape the job instead. Label: **Creator-stated** (2026-08-11).

**Two values for one meaning, wherever a colour has to be both text and a filled shape.** `--busy` and `--busy-fill`, `--accent` and `--accent-fill`. A status dot is a traffic light and wants the traffic light's own yellow; text at that value cannot be read on white. A progress bar wants a light blue; a link at that value is too pale. Splitting them was the only way to have both, and it is a rule now rather than a one-off. Label: **Creator-stated** (2026-08-13).

**And a third value where that filled shape has to carry words.** `--accent-solid` and `--bad-solid`. The fill is drawn to be a bar, so white words on it read at under three to one in both themes; the word value is drawn to be read on the page, so in dark it is far too pale to put white on at all. A button is neither of those things, and neither is the ticked box or the count on a toolbar icon. The three-way split is the same reasoning as the two-way one above, carried to the case it did not cover. Added 2026-09-02, when the primary button — *Add a server*, *Render 240 frames*, *Submit again* — turned out to hold the weakest text on the screen at 2.8 to one. **The mechanical test still decides which of the three:** anything painted takes the fill, anything written takes the word value, and anything painted that carries something written on it takes the solid.

**Nothing the user reads sits under four and a half to one against the ground it is on, in either theme.** That is the floor, and it is checked against the ground as it is actually composed — a tinted band, a selected row, a filled chip — rather than against the page. Three things are outside it and each is a mark rather than words: a tick inside a box, a play triangle over a picture, and the ordinary graphics floor of three to one that both of those clear. Settled at Design, 2026-09-02, after a sweep found the quiet grey at about three to one everywhere it is used — every server address, file size, date and job subtitle — and the dark theme's transfer tags between three and six and a half where the light theme's were between four and a half and nine.

**The split has to be finished in one go, or it shows.** When the amber first split, two shapes were moved to the fill and eleven were left behind — so the rail's open-work mark was one gold and the queue's status dot another, and the two are the same thing. The test is mechanical: **anything painted rather than written takes the fill.** A dot, a bar, a filled mark and a tinted ground are all painted; a word, a hairline rule and a one-pixel edge are written.

**The blue split was left half done in the same way**, and was found on 2026-08-24: a transfer file's bar and the search mark on the stale bar were both painting `--accent` where every other bar and dot paints `--accent-fill`. Both are corrected. **Three tints and one border are still on `--accent`** — the drop-into-folder wash, the chosen format chip's ground and border, and an auto-download row's ground — and whether a five-per-cent wash of the pale value reads at all is a thing to look at rather than a rule to apply blind. That is the Creator's, and until it is settled the rule above says nothing about a border.

**The two progress bars are `--busy-fill` and `--accent-fill`**, and the same pair is used at every size a bar comes in — two pixels on a queue row, three on a transfer's file, six ordinarily, seven on an entry — so a row never disagrees with the entry it opens. A transfer's bar is `--accent-fill` too, being one reading of overall progress. [Whole app](Whole%20app.md) owns the reasoning.

**A value Smoozy cannot confirm is shown by the italic face and the dash**, not by a colour of its own. Nothing is reserved for it.

**Ground and ink** — `--bg`, `--rail`, `--sunk`, `--sel`, `--line`, `--soft`, `--ink`, `--ink2`, `--ink3`, `--track`, `--shade`, `--on-fill`, `--pic-shade` — carry no meaning. They are the surface things sit on: the ground, the ink on it, the empty part of a bar or a track, the wash over what a dialog or a panel covers, the text drawn on a filled or a dark ground, and the ground of a mark painted over a picture.

## Light and dark

**Every token carrying a colour is defined for each theme**, with the badge hues named above as the one exception: once for light, once inside `prefers-color-scheme: dark`, and once more under each of the two `:root[data-theme]` blocks, so a viewer's own switch wins over the system. A new colour has to be given all of them, or it will be right in one theme and wrong in the other.

**Everything else is declared once** — the six type steps, the five radii, the two faces, the window's five widths, the gutter and the row gap — because none of them changes with the theme.

**And the theme is declared, not only painted.** Scrollbars, form controls and focus rings are drawn by the operating system rather than by us, and they stay light — inside a dark app — unless the page says which theme it is in. So the light block declares *light dark*, which covers the system's own dark as well, and each of the two `:root[data-theme]` blocks declares the one it is. It is easy to miss, because everything Smoozy draws itself looks correct while the scrollbars give it away. Label: **observed** — see [Sources](Sources/Design%20values.sources.md).

Smoozy itself has no theme switch — it follows the operating system, which the [whole-app Design](Whole%20app.md) settles.

## Type

**Six steps, and no more.** A seventh would be a decision nobody could defend.

**A glyph is not an exception.** A tick, a chevron and a drag handle were each written at a size of their own — nine and ten pixels — on the reasoning that a mark is not type. They are drawn by the type engine at a font size like everything else, so they were a seventh and an eighth step under another name. All of them are `--t-xs` now. The same holds for weight: the file used 400, 550, 600, 620 and 700, plus one 500 and one 640 that nobody had decided. Corrected 2026-09-02.

| Token | Used for |
| --- | --- |
| `--t-xs` | A tag beside something |
| `--t-sm` | Secondary lines, notes, meta |
| `--t-md` | Controls, dense rows |
| `--t-base` | Body |
| `--t-lg` | A screen or panel title |
| `--t-xl` | The one heading in a dialog |

**Two faces, both the operating system's, and both tokens** — `--face-ui` and `--face-mono`. The interface font for everything, and the system's fixed-width face for anything that lines up in a column — sizes, frame counts, memory, hosts, ports, paths. A desktop app that brings its own type reads as a web page.

Smoozy has no display face and no second family. The [whole-app Design](Whole%20app.md) says why.

## Radius

Five steps, and the step says what kind of thing it is. Anything rounder than a panel reads as a toy; anything squarer than a tag reads as a table.

| Token | Used for |
| --- | --- |
| `--r-xs` | A tag |
| `--r-sm` | A field |
| `--r-md` | A button, a row |
| `--r-lg` | A card |
| `--r-xl` | A panel, a dialog |

## The shape of the window

| Token | What it fixes |
| --- | --- |
| `--menu-h` | The menu bar — the system's on a Mac, ours elsewhere |
| `--title-h` | The title bar |
| `--rail-w` | The rail — navigation only, never the server list |
| `--list-w` | A list sitting beside a detail |
| `--side-w` | The column holding a preview |

Those are the load-bearing sizes of the window itself. Everything else sizes itself.

**The widths are the window's, not the screen's** — [Whole app](Whole%20app.md) says why, and the mockup carries a control for walking them, since the pane a mockup is looked at in does not go narrow enough to show them.

**Four widths and a floor.** The window drops a pane at 1180, another at 1020, narrows the rail at 900, and takes the rail down to badges and icons at 780; below 680 Smoozy says the window is too small rather than folding into a strip. The five numbers live in the mockup with the rest; [Whole app](Whole%20app.md) says what goes at each and why.

## What is honestly not tokenised yet

**Most spacing is literal.** Two distances are named — the gutter and the row gap, both below — and every other padding and gap is written where it is used. There is no spacing scale and there should not be one; **Spacing** below says why.

**The rail's icons are a set of their own, drawn on the same 16-pixel box as the file kinds.** Five places and nine toolbar actions, one weight, no fills. They are not tokenised and do not need to be — an icon is a drawing, not a value.

**Some component-level sizes are literal too** — a badge is a 10px squircle (9px inside running text, and 14px in the rail at the last narrowing step, where the badge is the whole row) at 30% radius, a checkbox 13px, a switch 30 by 17, a progress bar 6px. **A few drawn glyphs are deliberately off the type scale** — a chevron, the tick inside a checkbox, a drag grip, a play mark — because they are marks rather than text. The type scale governs text the user reads, and forcing a tick onto its smallest step makes it a blob rather than a mark. Each is used in few enough places to be findable, and none is a decision anyone would want to change globally.

**A shadow's black is written where it is used**, at whatever depth that shadow needs. There are seven of them at six depths, so one token would be right for none of them and would need six overrides — which is a second copy of the problem rather than a fix.

**A badge sits 7px from the name it labels, on every screen.** That distance is one decision and it is written here because it was four different numbers before anyone measured it.

**A switch and a checkbox are two components, not one drawn twice.** A switch turns one thing on or off; a checkbox picks out of a set. The [whole-app Design](Whole%20app.md) owns that rule.

**This is stated rather than hidden**, because a values document that claims more coverage than it has is worse than one that admits the gap.

## Spacing

**There is still no spacing scale, and that is the answer rather than a gap.** The small distances — the padding inside a card, the gap between a label and its field, the air under a heading — are separate judgements about separate distances, clustered from 1 to 14 with nothing between the busiest of them to refuse. A scale earns its place by refusing the values between its steps, and there is nothing there to refuse. What it would buy is a token to type instead of a number, at the cost of a second thing to keep right.

**But two distances are not small judgements, and both are tokens now.** They were literals, and each of them had drifted into three or four different answers to one question before anyone measured. That is exactly what an earlier version of this section said could not happen without a scale, and it happened.

| Token | What it fixes |
| --- | --- |
| `--gutter` | How far a screen's own content sits from the edge of the pane it is in — header, rows, footer, banner, everything |
| `--rowgap` | The gap between the columns of a list row |

**The gutter is one number because it is one question.** It was 22 on Servers, Keys, the file browser, render settings, server setup and Settings; 18 on the queue and history; 20 on transfers and on the queue's detail pane. Nobody could name that, and everybody felt it — the content sliding two or four pixels while the rail it sits beside had not moved. **It narrows once, at the last step, and narrows for everything at the same moment.** Before, the narrowing rule tightened the Servers header and not its own rows, the file rows and not the toolbar above them, the detail header and not the detail body — three screens where a heading stopped lining up with what was under it, and only at the width where the layout could least afford it.

**The row gap is one number for the same reason.** It was 16 on Servers, Keys and history, 14 on transfers and server setup, 12 on the queue and the file browser.

**A note that belongs to a value is inset to the value column and nothing else**, which is the label's 84px plus the row's own 7px gap. It is written once, because the two kinds of note had drifted to 88 and 91 — one of them aligned with nothing at all.

**A badge sits 7px from the name it labels**, which is above and is the third named distance.

**The rule that holds under all of this**: where two distances answer the same question on two screens, they are the same number, and the way to find out is to measure the other screen rather than to pick. Where that question gets asked on every screen, it is a token — because the measuring did not happen, four times, and a literal cannot make it happen.

## Changing something

**Open [the mockup](Mockup/Smoozy.html), change the token, and look.** The mockup is one file with no build step: open it in a browser.

**Then check the decision documents still say the truth.** If a change makes one of them wrong — a colour that no longer means what [Whole app](Whole%20app.md) says it means — the document is what has to be fixed, not the sentence quietly left stale.

**And if the change is a decision rather than a value**, it belongs in a Design document first. The mockup shows; the documents settle.

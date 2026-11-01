# Whole app — sources

*Where the outside facts in [Whole app](../Whole%20app.md) came from. Smoozy's own decisions are not here, and neither is any Blender fact a Specs document already owns — this file holds only the claims about the world that this Design document is the first to make.*

**What is deliberately not in here.** Most of what the Design documents say about Blender is a restatement of a fact a spec owns and cites — that only Blender 4.5 names the view layer while rendering, that a resolution outside 1 to 32767 is silently corrected, that EEVEE gives no time estimate, that Blender reports tiles once a frame is split. Each of those is cited where it is owned, in `Preparation/Specs/Sources/`, and copying the citation here would give it a second home. The Design documents point; they do not carry.

---

## The menu bar is the operating system's, not Smoozy's

**The claim.** On a Mac the operating system puts a menu bar at the top of the screen whether an app fills it or not, so the only question is what goes in it; on Windows and Linux the same menus sit behind one button in the title bar.

**Label:** **documented**. It was held as an assumption by the Design stage and was checked on 2026-08-18, from each platform's own publications, in `Preparation/Architecture/Desktop app and delivery.md` §18.1 K1 to K4.

**Where it came from.** It began as ordinary knowledge about desktop platforms, written into the Design because the shape of the window depends on it. The check behind it is now Architecture's: Apple's own *macOS User Guide* for the Mac's screen-level bar; Microsoft's *About Menus* reference for Windows; and freedesktop's specification index, with the Qt and GTK references beside it, for Linux. Each address and each quotation is in that section.

**The conclusion is confirmed on all three, and the reason was corrected on two.** The Mac's bar belongs to the screen and is the system's, exactly as the claim says. **On Windows and Linux the one-button arrangement turns out to be Smoozy's own choice rather than something either platform forces** — Windows publishes a window-owned menu bar the system positions under the title bar, and the toolkits an application would be built on ship a menu bar on Linux, one of them with a published route to a desktop-level one. Nothing about the four menus or the arrangement changes; what changes is why it is that way.

**What it does not cover.** Whether the window Smoozy is built in gives Design what it needs on each. That is a property of the chosen shell rather than of a platform, and it is Architecture's, at §18.3 of the same document.

**What breaks if it is wrong.** The menu bar section, and the `--menu-h` and `--title-h` tokens. Nothing else — every action in Smoozy lives on the thing it acts on, so the menu bar carries nothing that has no other home.

---

## A squircle can be told apart from a circle at the size a badge is drawn

**The claim.** A reader sees the difference between the badge's squircle and the round marks Smoozy uses for state at the size those marks are actually drawn, so shape rather than colour carries the difference between identity and state.

**Label:** **an assumption**, held by the Design stage and not checked by anybody.

**Where it came from.** Nobody looked. The shape rule was settled on 2026-08-11 to free the badge palette, and this claim is what it rests on; it is written down here rather than left to look settled.

**What it does not cover.** The size the two shapes are drawn at, the density of the screen they are drawn on, and the theme they are read in. None of the three was named when the rule was taken, and the claim has never been tested at any of them.

**What breaks if it is wrong.** Two things, and neither is a detail. **The eight badge hues**, which sit 45 degrees apart around the whole wheel and include green, red and amber only because shape is doing the identity-against-state work — two earlier palettes squeezed into one narrow arc were abandoned for near neighbours that could not be told apart. `Preparation/Design/Design values.md` carries the same dependency for its eight tokens. **And the badge-only rail's justification**, where at the narrowest step the badge is the whole row with no label beside it.

**How it gets answered.** By looking at the mockup's own badge at its real size on real screens. It is not a research question.

---

## The Blender Foundation was asked about using its mark, and declined

**The claim.** Smoozy uses no Blender logo or mark of any kind, because permission was asked for and refused.

**Label:** **Creator-stated** (2026-08-10).

**Where it came from.** The Creator, who asked. There is no document behind it.

**What it does not cover.** Anything about what the Foundation's terms permit in general. This is one answer to one request, and the Design treats it as settled for Smoozy without generalising it.

**What breaks if it is wrong.** Nothing about the design would have to change — the `.blend` icon is a plain cube and works whatever the answer is. What is at stake is the reason recorded for it, not the drawing.

---

## Three things borrowed from Blender's own visual vocabulary

**The claims.** Blender marks an animated property with a keyframe diamond, solid where the property is keyed on the current frame and hollow where it is animated but not keyed there, in a yellow of its own. Blender turns a number field red where the value is refused. Blender colours a linked library's rows so they can be told from local data.

**Label:** **Creator-stated** (2026-08-10) — a Blender professional describing the program they use daily.

**Where it came from.** The Creator, from their own use. No research question asked about Blender's interface, because Smoozy never opens Blender's window and nothing about how a job renders depends on it.

**What it does not cover.** The exact hues Blender uses, which Smoozy does not copy — `--key` and `--lib` are Smoozy's own values chosen to read the same way, not sampled from Blender. And whether any of the three has changed across the four supported versions, which nobody checked.

**What breaks if it is wrong.** Each of the three would stop meaning to an artist what it means in Blender, which is the whole reason for borrowing it. The keyframe diamond is the one that costs something: it is what stops a view layer checkbox reading as a promise about the whole range, and a replacement would have to do that job.

---

## The window's own chrome — where each system's arrangement came from

*Added 2026-09-02. Every page read that day. Smoozy's own decisions — that there is one title bar at one height, and that the name and the speed sit in it — carry no citation; what is cited is what each operating system does.*

- **Windows reserves the corner and puts close at the right.** *Title bar customization*, `https://learn.microsoft.com/en-us/windows/apps/develop/title-bar` — *"The system reserves the upper-left or upper-right corner of the app window for the system caption buttons (minimize, maximize/restore, close)"* and *"The system draws the Close button in the upper-right for left-to-right languages and the upper-left for right-to-left languages."* Label: **documented**. **What it does not cover:** the shell Smoozy is built on exposes no way to keep those system-drawn buttons over a frameless window, which is why Smoozy draws its own there.
- **What they should look like.** *Title bar design*, `https://learn.microsoft.com/en-us/windows/apps/design/basics/titlebar-design` — the standard bar is *"a height of 32px"*, the buttons are minimise, maximise/restore and close, *"Caption buttons have full bleed backplates"*, and they *"respond to rest, on hover, on pressed, active, and inactive states"*. Label: **documented**. **What it does not cover:** the width of a caption button is published as a runtime value the system reports rather than as a number, so Smoozy's is its own choice.
- **The close button's hover colour is the system's and is not published.** Same *Title bar customization* page — *"The button background color is not applied to the close button hover and pressed states. The close button always uses the system-defined color for those states."* Label: **documented**, and it is the source of the stated approximation: no page read gives the value, so Smoozy's red is its own and may not match a given machine.
- **Windows 11 rounds an undecorated window that keeps its shadow.** The shell's own configuration reference, `https://v2.tauri.app/reference/config/`, on the window shadow — *"true will make undecorated window have a 1px white border, and on Windows 11, it will have a rounded corners."* Label: **documented**.
- **Linux publishes which buttons and on which side, and its default is close alone.** The desktop's own settings schema, `https://raw.githubusercontent.com/GNOME/gsettings-desktop-schemas/master/schemas/org.gnome.desktop.wm.preferences.gschema.xml.in`, key `button-layout`, default `'appmenu:close'` — *"the colon separates the left corner of the window from the right corner, and the button names are comma-separated"*, and *"Unknown button names are silently ignored"*. Label: **documented**. **What it does not cover:** desktops that are not GNOME publish their own arrangements elsewhere and none was read, so where nothing can be read Smoozy draws the three at the right.
- **macOS keeps its own frame and its own buttons**, which is the route the architecture already supplied — `Preparation/Architecture/Desktop app and delivery.md` H6a and H6b, with the shell's own pages cited there.

**Nothing in this section has been seen working on Windows or on a Linux desktop.** This project has neither machine and renting one to answer a question is ruled out, so both arrangements are built from the pages above and stay unproved.

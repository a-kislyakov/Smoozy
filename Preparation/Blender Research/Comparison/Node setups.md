# Node setups — Blender 4.5, 5.0, 5.1, 5.2 and 5.3 side by side

What files a geometry node setup inside a `.blend` file can make that file point at, and what a file that links such a setup from another `.blend` records about them.

## The machines behind the tests

Every hand test on this topic was run on one of two computers, and nothing here was run anywhere else. The equipment section of `Preparation/Blender Research/Method.md` describes each in full.

- **The Creator's own Mac** — a MacBook Pro with an Apple M1 Pro chip and Apple's own graphics, no second card, with a screen; the only machine where anything drawn in a window could be looked at. It ran Blender 4.5.12, 5.0.1, 5.1.2 and 5.2.0 on macOS 26.5.2, and Blender 5.3.0 Beta, built 2026-10-16, on macOS 26.6.2, with Blender 5.2.2 beside it for checks.
- **A rented Linux machine** — one NVIDIA GeForce RTX 4080 SUPER and **no display server at all**; the kind of computer a Smoozy render actually happens on, and the **fourth** rented Linux machine in the equipment section. It ran 4.5.12, 5.0.1, 5.1.2 and 5.2.0, and **nothing in 5.3**.

**A result from one machine is never set against a result from the other and called a difference between Blender versions.**

**A result that came back the same in every version still settles only the machines it was run on.** Whether it holds anywhere else is untested everywhere, and untested everywhere is not a difference between versions.

**Every 5.3 test ran on the Mac alone, in a beta build.** Where an answer below gives a Linux result, it speaks for 4.5 to 5.2. Blender 5.2.2 control results below are also from that Mac.

**Windows is untested for everything on this topic**: no Windows machine ran any of it. This leaves Server and desktop editions alike untested.

## The answer in short

### Where Blender itself differs between versions

- **1 and 2 — Import-node paths and Blender's path housekeeping: 4.5 against 5.0 onward.** See question 1, difference 1, and question 2, difference 1.
- **1 and 2 — Python path facilities and their extra path information.** See question 1, difference 2.
- **1 — the font setting becomes an input socket in 5.1.** See question 1, difference 3.
- **1 — the sound-file route and Font input node arrive in 5.2.** See question 1, difference 4.
- **1 — SPZ import arrives in 5.3.** See question 1, difference 5.
- **1 — bake path templates in 5.3.** See question 1, difference 6.
- **1 and 2 — the bundled node files.** See question 1, difference 7, and question 2, difference 4.
- **1 — registered node counts and the number-to-text inputs.** See question 1, after difference 4.
- **1 — the drawn socket shape, on the Mac.** See question 1, the Path-row result.
- **1 and 2 — the earlier disk-bake test's differing file counts in 4.5 to 5.2.** This is a narrow observed difference; see question 1, *What the disk-bake tests wrote*, for its limits.
- **2 — the library loader's library list arrives in 5.2.** See question 2, difference 3.
- **2 — packed-library properties from 5.0 onward.** See question 2, difference 2.
- **2 — the rule for directly linked packed data changes in 5.3.** Documented in release notes, untested here; see question 2, difference 5.

### What is not established

- **1 and 2 — every 5.3 test result off the Creator's Mac and outside the tested beta build.** See the machines section.
- **1 — dragging a supported file into the node editor.** See question 1, *What the documentation establishes*.
- **1 — re-reading a changed file on the untested version-and-machine combinations.** See question 1, the re-read result.
- **1 and 2 — a general version rule for the files inside a disk bake.** See question 1, *What the disk-bake tests wrote*.
- **1 — sequences, movies and tiles through image inputs where the answer is inferred or disputed.** Untested in all five; see question 1, the image route.
- **1 — the external-font route's disputed label in 4.5 and inferred label in 5.3.** See question 1, the font route.
- **1 — the path form before each 4.5 housekeeping command.** See question 1, difference 1.
- **1 — modifier-input paths and their Python access in versions before 5.3.** See question 1, *Results tested only in 5.3*.
- **2 — file paths supplied on a local modifier using a linked group.** Inferred, untested; see question 2, the local-modifier paragraph.
- **1 — bake folders and path housekeeping before 5.3.** See question 1, *Results tested only in 5.3*.
- **1 — the built-in font's file reference before 5.3.** See question 1, *Results tested only in 5.3*.
- **1 — the full file-input node count outside the nodes made in the 5.3 test.** See question 1, *Results tested only in 5.3*.
- **1 and 2 — references created by using bundled node groups before 5.3.** See question 1, the Smooth by Angle result.
- **1 and 2 — a new modifier's bake settings and the linked image's editor view before 5.3.** See the window results under questions 1 and 2.
- **1 and 2 — Windows, and window drawings on Linux.** See the machines section and the window results.
- **1 and 2 — letter case on a different kind of file system on either machine.** See question 1, the letter-case result.
- **2 — the linked modifier's Manage contents in 4.5 to 5.2.** See question 2, the modifier window result.
- **2 — the loader's list for a library chain in 5.2.** See question 2, difference 3.
- **2 — the base of a chained library's recorded relative path.** See question 2, the chain result.
- **2 — what the relative-path bug fix in 5.2 changed.** See question 2, the final subsection.

### Where only the documentation differs

- **1 — the OBJ materials-and-textures note.** See question 1, *What the documentation establishes*; the tested behaviour agrees.
- **1 — the Format String path example, in words or a picture.** Its coverage is not established for every manual. See question 1, *What the documentation establishes*; the tested behaviour agrees.

---

## 1. What files can a geometry node setup inside a `.blend` file cause that file to point at?

### The list of routes, and how much of it is the same

**Most of it is the same in all five.** A geometry node setup can make its file point at:

- **A file named by a path on an Import node** — OBJ, PLY, STL, CSV, plain text and VDB in every version, and **in 5.3 alone** a seventh, for SPZ files. The build itself reports exactly those six in 4.5 to 5.2 and those seven in 5.3; the CSV one additionally takes a delimiter.
- **The same kind of file, named by a value typed on the modifier rather than in the node** — through a group input of the File Path kind, wired into the Import node's Path. Inferred for modifier-held values in 5.0, 5.1, 5.2 and 5.3, and observed in 5.3; no test in 4.5 to 5.2 asked it. See *Results tested only in 5.3*, below.
- **An image file, through an image data-block chosen on a node.** Documented in all five. Using a numbered sequence, movie or set of tiles through these nodes is **inferred** in 4.5 and 5.3. In 5.0 the full extension is inferred, with a conflicting documented reading for movies; in 5.1 it has conflicting documented and inferred readings. In 5.2 it is labelled **documented**. No test on this topic asked it. These are differences in how firmly the answer is established, not demonstrated behaviour differences.
- **A font file**, through the node that turns text into shapes. Documented in 5.0 to 5.2. In 4.5 the route has conflicting **documented** and **inferred** readings; in 5.3 the node is documented and the font file behind it **inferred**. No external-font test settles those labels. How the font gets there differs; see below. Blender's own built-in font points at no file, in 5.3; see below.
- **A folder of baked data**, written by a Bake node or a simulation zone.
- **Another `.blend` file**, when the setup is built on a node group that lives in one. The route has conflicting **documented** and **inferred** readings within 4.5 and within 5.2; it is documented in 5.0 and 5.3 and inferred in 5.1. Question 2's tests observe it in all five. In 5.3 that includes Blender's own bundled node groups; see below.
- **Whatever the setup drags in one step further out** — a material's own image files, a linked object or collection's own file. Called **inferred** in every version that raises it.

**5.2 and 5.3 add one route the other three do not have: a sound file. 5.3 adds another: an SPZ file.** See the differences below.

### What the documentation establishes

**Dragging a supported file into the node editor** is documented in all five as creating and configuring its Import node. No drag was tested: the permitted scripts could not make a real drag, and mouse clicks were not allowed. This is an untested action, not a version difference.

**The OBJ materials-and-textures note** is documented on the Import OBJ page in 5.0 and 5.1. Whether the other three manuals carry the same note is not established. The test below finds no imported material in all five.

**A path built from the frame number** is shown in 5.1's Format String manual picture: Scene Time feeds Format String and Join Strings, which feeds Import CSV's Path. In 4.5 that page gives the worked example “Path with Frame Number” in words. Whether 5.0's, 5.2's or 5.3's manual carries either example is not established. This does not show that the examples are absent there. The frame-by-frame test below gives the same behaviour in all five.

### What the tests settled, the same way in every version

All of the following was run in 4.5 to 5.2 on **both machines**, which agreed unless the entry says otherwise, and in 5.3 on the **Mac alone**, where it came out the same way.

**The path is a socket, not a fixed setting.** On every Import node — the six, and 5.3's SPZ node — the Path input reports itself as a file-path socket. A String node's output was linked into Import OBJ's Path, and that link reported valid. The file-path socket type and the valid string link are **observed** in all five. The type is inferred from the documentation rather than named there for the Import nodes.

**Which path forms work.** A full absolute path finds the file. `//model.obj` — Blender's shorthand for the folder the `.blend` sits in — finds it too, so the `//` prefix is honoured. `./model.obj` finds nothing. A path to a file that is not there finds nothing. An empty path finds nothing. For `./model.obj` and for the missing file Blender prints an error naming the path it could not open; for the empty path it prints nothing. The `.blend` still opens, and Blender still exits normally.

**An imported OBJ brings in no material.** With a matching material file beside the `.obj` and named inside it, no material appeared on the geometry, and the file's own list of materials was unchanged before and after.

**A Blender with no window reads the file when it opens the `.blend`.** The file on disk was changed to hold four vertices; a freshly started Blender opening the `.blend` reported four. With that file moved away, it reported none, printed an error naming it, and exited normally.

**A Blender with no window reads the file again at render time.** Rendering the same small frame with the file present and with it absent gave different pictures. Rendering with the switch that turns off building the scene's dependency graph on load made no difference at all to the picture: nought values differed, both with the file and without it. How many values differ depends on the scene, not the version: 558 of 16,384 in the scene 4.5 to 5.2 rendered, the same figure in all four on both machines, and 1,878 in the scene 5.3 rendered, which Blender 5.2.2 on the same Mac also gave. In 5.3, and in 5.2.2 on the same Mac, the error for the missing file was printed before rendering began without the switch, and after rendering began with it.

**The file a setup reads can change from frame to frame.** A setup that built its path from the frame number read a three-vertex file at frame 1, a four-vertex file at frame 2, a five-vertex file at frame 3, and the three-vertex one again on returning to frame 1.

**Where a bake goes when the Bake Path is left empty.** Blender fills the path in itself. It printed "Bake directory of object Target, modifier GN is empty, setting default path", set the modifier's Bake Path to `//blendcache_<name of the .blend>/<object>_<modifier>`, and wrote the bake into that folder beside the file. With a path given instead — `//mybakes` — it wrote there. In every version the modifier's Bake Target starts at Packed. A bake inheriting that target needs no separate file, as documented; this was also observed in 5.3 and in 5.2.2 on the Mac. Individual bakes can override the modifier's target.

**In a window, the Path row looks the same in all five.** A text field holding the path, a folder button at its right-hand end, and a socket dot on the node's left edge level with that row. The only difference between the versions is the shape of the socket dots — round in 4.5, rectangular in 5.0 to 5.3 — which is Blender's drawing style and nothing about paths. **This settles macOS only.** This topic's rented Linux machine has no screen and no Windows machine ran it, so what the row looks like on either of those is untested in all five versions. **It is untried rather than untestable** — this project has rented Linux machines with a display server since, and a Windows machine can be rented.

### One thing that differs between the two machines, in every version that ran on both

**Letter case.** On the rented Linux machine a path is matched letter for letter: `//MODEL.OBJ` and `//Model.Obj` find nothing where `//model.obj` finds the file, an image path in upper case does not load, and Blender's own "find this name whatever its case" helper hands back the lower-case file that exists. On the Creator's Mac both upper-case forms found the file, the upper-case image loaded, and the same helper handed the upper-case name straight back.

**This is an operating-system fact and it came out the same way in all four versions run on both machines.** 5.3 came out as they did on the Mac and was not run on Linux. It is not a difference between versions. The Mac's working folder sat on the machine's own ordinary volume, not one made for the test. Windows is untested. Neither machine was tested on a file system with different letter-case rules.

### Whether an Import node reads its file again when the file changes on disk

**Settled the same way in every version that ran it — on the Linux machine in 4.5, 5.0, 5.1 and 5.2, and on the Mac in 5.3.**

Three findings, and **all three came back the same in each**:

1. **Asking for the evaluated geometry again does not by itself make the setup run again.** With nothing touched and nothing marked for update, five readings in a row give the old count however many times you ask. Something has to mark the setup for a fresh run — touching the modifier does it, and so does marking the object for update from a script.
2. **Even then, the file is read again only if its modification time has moved by a whole second.** A change inside the same whole second is not noticed at all: the setup runs again and comes back with the old contents. A change that crosses a whole second is noticed every time, in either direction — a stamp moved a second backwards counts as much as one moved forwards. Across eighty runs on Linux — four versions, ten stamp offsets, both ways of reading — and twenty in 5.3 on the Mac, there was not one exception. Where the overwrite came a few hundredths of a second after the write, so that the whole second did not move, nothing was read again: ten repeats in each of the five versions, fifty runs without an exception.
3. **Marking the object for update is simply the earliest of the things that make the setup run again.** With the stamp moved, that way of reading shows the new contents at the very first reading after the overwrite; reading plainly shows them at the first reading after a nudge. One reading of difference, and no more.

**What this does not settle.** No version was run on both machines: 4.5 to 5.2 on macOS, and 5.3 on Linux, are untested, and no Windows machine ran it. The same pattern on both machines points at no difference, between versions or between machines, but does not rule one out where nothing ran. Nothing in the runs says *why* the whole second is the unit; that would be a claim about Blender's insides, and they only watch it from outside.

**This matters for a Smoozy render.** A script that writes a file and immediately asks for the geometry can be handed the old contents, with nothing said about it.

### What differs between versions

**1. Whether an Import node's path takes part in the file's own path housekeeping.** The test put an Import node's path and an ordinary image path side by side in one file and ran Blender's own housekeeping over both. The image is the control — it is a kind of file Blender's documentation marks as one it looks after — so a run in which nothing moved can be told from a run in which only the node's path failed to move.

- **Blender 4.5.** Make Paths Relative said "Total files 3 | Changed 1"; the image path became `//tex.png` and the node's path stayed a full absolute path. Make Paths Absolute said "Total files 3 | Changed 3"; the image path became absolute and the node's path stayed `//model.obj`. Report Missing Files, with both files moved away, named only the image. Blender's list of the file's outside paths held only the image, and its map of data-blocks to paths held only the image. The walk-every-path facility does not exist in this version.
- **Blender 5.0, 5.1, 5.2 and 5.3.** Make Paths Relative said "Total files 4 | Changed 2"; **both** paths went relative. Make Paths Absolute said "Total files 4 | Changed 4"; both went absolute. Report Missing Files named both, describing the second as coming from the node group. The list of outside paths held the `.obj` as well, the map held the node group against `//model.obj`, and the walk-every-path facility visited both.

The initial path forms before the two 4.5 commands are not recorded here. The absolute path after Make Paths Relative and the relative path after Make Paths Absolute therefore do not show what happened between those commands. That each command left the node's path alone is established; their starting forms remain open.

Run in 4.5 to 5.2 on both machines, which agreed, and the 4.5-against-the-rest split came out the same way on each; run in 5.3 on the Mac, where it came out as in 5.0 to 5.2.

**What does not differ here:** Pack Resources packed exactly one file in all five — the image. Opening the packed file afterwards, with both outside files moved away, the image was still there and still packed, and the geometry was gone in every version. **The `.obj` is not packed into the `.blend` in any of the five.**

**2. The Python facilities and properties.** Absent in 4.5, present in 5.0, 5.1, 5.2 and 5.3: the walk-every-path facility `bpy.data.file_path_foreach`, which 5.0's release notes introduce; the library marks `is_archive`, `archive_libraries` and `archive_parent_library`; the data-block mark `is_linked_packed`; and a `pack` option on the Python call that loads a library, which 4.5 rejects as a word it does not know. New in 5.2 and kept in 5.3: the extra information handed to the caller about each path found, which 5.0 and 5.1 hand back as nothing. Run in 4.5 to 5.2 on both machines, and in 5.3 on the Mac.

Unchanged in all five: the map of data-blocks to paths, the list of outside paths, all eight of the file operators including packing and unpacking linked libraries, the library's own path, parent, version, packed copy and reload, the data-block's library, indirect and missing marks, and the named bake properties checked on the modifier, including its Bake Target starting at Packed.

**3. How a font reaches the String to Curves node.** In **4.5 and 5.0** the node has no font input socket; the font is a fixed setting on the node. In **5.1, 5.2 and 5.3** it is an input socket, the fixed setting is gone, and the node has four menu inputs — alignment across, alignment down, pivot point and overflow. 5.1's release notes call the socket new there. Alongside it, the socket types for a font, a sound and a text data-block, and their node-group equivalents for font and sound, are absent in 4.5 and 5.0 and present in 5.1, 5.2 and 5.3. Run in 4.5 to 5.2 on both machines, and in 5.3 on the Mac.

**4. Sound as a route, in 5.2 and 5.3.** The Sample Sound Frequencies node and the Font input node exist in **5.2 and 5.3** and in none of the other three. 5.2's release notes call them new. So in 5.2 and 5.3 a geometry node setup can make its file point at a sound file, and in 4.5, 5.0 and 5.1 it cannot. Run in 4.5 to 5.2 on both machines, and in 5.3 on the Mac.

The number of geometry node classes the build registers grows the same way: 223 in 4.5, 241 in 5.0, 252 in 5.1, 274 in 5.2, 292 in 5.3. The node that turns a number into text has two more inputs in 5.2 and 5.3 than before — a base and a padding — which bear on building a path from a frame number.

**5. An SPZ file, in 5.3 alone.** 5.3 has a seventh Import node, Import SPZ, which reads a point cloud from an SPZ file; its Path is a file-path socket like the other six's. 5.3's release notes say "Added `Import SPZ` node.", and its Python documentation describes the node; 5.3's manual list of import nodes does not include it. The Mac build lists it in 5.3; the 4.5 to 5.2 builds do not list it on either machine; the same check run in Blender 5.2.2 on the same Mac lists six, so the difference is in the Blender and not in the check.

**6. A bake path can carry a template, in 5.3.** 5.3's release notes list "Bake path template support" among 5.3's changes, and 5.3's Python documentation says the modifier's bake path supports template expressions. 5.3's own manual still says templates work only for the render output path and the File Output node's paths; the test settles it for 5.3. With the Bake Path set to `//bakes/{blend_name}/`, a bake of `t.blend` went into `bakes/t/`, and no folder named `{blend_name}` appeared; the Bake Path still held the template text after the bake. **So the Bake Path a 5.3 file records may not be the folder its bake actually uses.** Observed on the Mac. No earlier version was tested; the release note is what makes this a difference rather than a silence.

**7. Which node `.blend` files each Blender ships.** 4.5 ships two, under a folder named `geometry_nodes`: `smooth_by_angle.blend` and `procedural_hair_node_assets.blend`. 5.0 and 5.1 ship four, under a folder named `nodes`: essentials files for geometry, shading and compositing nodes plus the procedural hair one, with `smooth_by_angle.blend` gone. 5.2 and 5.3 ship those four plus a geometry-nodes dynamics file and a principal-components file — the same six names in both. Counting every `.blend` under the assets folder, which is a wider count than the node files alone: eleven, thirteen, thirteen, fifteen, fifteen. Read on the Linux machine in 4.5 to 5.2 and on the Mac in all five; the two machines' lists match version for version, which is what rules out this being a fact about how one of them was installed. In 5.3 the Smooth by Angle node group sits in `geometry_nodes_essentials.blend`.

### What the disk-bake tests wrote

**The layout is the same in all five.** Inside the bake folder, a folder named with the bake's number holds `meta/00001_00000.json`, a description file, and in some runs `blobs/00001_00000.blob` beside it, a data file. **The earlier test differs between 4.5 and 5.0 to 5.2, but these results do not establish a general file count for a geometry bake:**

- **The first way**, used in 4.5 to 5.2 on both machines, made a Bake node and linked the group's geometry straight into its first input, without first adding a Geometry item. 4.5 left both files; 5.0, 5.1 and 5.2 left only the description file. Both machines agreed.
- **The second way**, used in 5.3 and in Blender 5.2.2 on the same Mac, first gave the Bake node one geometry item. Both left both files.
- In 5.2.2 and 5.3 a Bake node made from a script starts with nothing to carry. What it starts with in 4.5, 5.0 and 5.1 was not asked.

The first test's two-file versus one-file result is an **observed difference** between 4.5 and 5.0 to 5.2 on both tested machines. It does not show that 5.0 to 5.2 always write a single file for a bake that holds geometry: 5.2.2 wrote two with an explicit Geometry item. What accounts for the earlier split is not established, and the different construction in 5.3 establishes no new version difference. The same goes for a bake through a linked group, under question 2. No version's documentation lays out what sits inside a bake folder.

### Results tested only in 5.3

Each of these was asked of 5.3 alone, on the Mac. No earlier version ran them, so whether 4.5 to 5.2 behave the same way is not settled — a silence, not a difference.

**A file path typed on the modifier.** A group input of the File Path kind was wired into an Import OBJ node's Path, the modifier's own value for that input set to `//model.obj`, and nothing typed into the node. The import read the file. **But Blender's housekeeping and its lists passed that value by.** Make Paths Relative and Make Paths Absolute changed the image path beside it and left the modifier's value as it was, each counting one file fewer than when the path sat in the node. Report Missing Files, with the modifier's value pointing at a file that was not there, named the missing image and not the missing `.obj`, though the import had just printed an error for it. Neither Blender's list of outside paths, its map of data-blocks to paths, nor the walk-every-path facility listed it. **So in 5.3 a setup can read a file that all three tested Blender listings miss.** That the value is kept with the modifier rather than in the node group is inferred from 5.3's documentation. **Observed in the 5.3 beta on the Creator's Mac:** a script reaches the value through the modifier's `properties.inputs.<input's identifier>.value`; writing `mod["Socket_2"] = "//model.obj"` was refused with "id properties not supported for this type". Whether 4.5 to 5.2 accept `mod[<input's identifier>]` was not asked.

**Bake folders and the path housekeeping.** With Bake Target set to Disk and an image path beside it as the control, Make Paths Relative and Make Paths Absolute changed the modifier's Bake Path as they changed the image's, and a single bake's own path too, when that bake was set to use one. Report Missing Files named a Bake Path whose folder did not exist, as coming from the object — both before anything had been baked and after the baked folder was moved away; with the folder in place it reported nothing missing. Blender's list of outside paths and its map of data-blocks to paths carried the Bake Path, under the object, before and after baking. Pack Resources reported one file packed, the image, and left Bake Path and Bake Target as they were; the saved file grew by 837 bytes, against a 75-byte image and 2,425 bytes of bake files.

**Blender's built-in font.** A String to Curves node with nothing fed to its font input used a font data-block named `Bfont Regular`, the same one a new Font node holds. Its path reads `<builtin>`; it is not packed and not linked, and neither Blender's list of outside paths, even with packed data included, nor its map of data-blocks to paths listed it, before or after saving and reopening. **So the default font makes the file point at no font file.** 5.3's documentation says the built-in font is selected by default, and does not say whether it refers to a file.

**Which nodes can take a file or a data-block.** The test tried 342 geometry and function node classes and made 338 nodes. Twenty-five have an input or a setting for a file path or a data-block: the seven Import nodes, a file path; Image Info and Image Texture, an image input, and the Image node, an image setting; String to Curves, a font input, and the Font node, a font setting; Sample Sound Frequencies, a sound input; and the rest objects, collections, materials, or — the group node — another node group. **None of the 338 nodes made in this test takes a text data-block or a texture through an input**, although both socket types exist. The Bake node has no file-path input or setting; its folder is set on the modifier. Four classes could not be made in a group; two of them are the names of node kinds rather than nodes.

**What a new modifier's bake settings show in a window.** On a fresh modifier whose group holds a simulation zone, with nothing changed, the modifier's Manage section — opened by its own switch — held a Bake section showing Bake Target **Packed** and an **empty** Bake Path with a folder button, matching what the file holds. Drawn on macOS only.

**What adding one of Blender's own node groups records.** Shade Auto Smooth adds a Geometry Nodes modifier named Smooth by Angle. Its node group is **linked from `geometry_nodes_essentials.blend` inside the Blender application, and packed** into the file. The file then records two libraries, both naming that bundled file; one is an archive, and the group belongs to it. Before saving, both are recorded by full path; **after saving, by a path relative to the `.blend`** that climbs out of the file's own folder into the application folder of the Blender that saved it. Blender's list of outside paths and its map of data-blocks to paths both list that bundled file. Opened from a folder where that relative path leads nowhere, the file loaded with no warning, the group was present, packed and whole, and the modifier still used it — but Report Missing Files named both library paths as not found. Opened where it was saved, nothing was missing. **So in 5.3 a file using Smooth by Angle carries a library path that reads as missing wherever it no longer leads to the bundled file, though nothing the file needs is missing.** Whether 4.5 to 5.2 record their bundled node groups the same way was not asked; 4.5 ships Smooth by Angle in a file of its own (item 7 above).

---

## 2. When a `.blend` file links a node setup from another `.blend` file, what does the linking file record about the files that setup uses, and how are they reached?

### What was settled, the same way in every version

The set-up: a folder `lib/` holding `lib.blend`, a three-vertex `model.obj` and a picture file; inside `lib.blend`, a node group whose Import node's Path is `//model.obj` and whose Image Texture node holds the picture. A second folder `shot/` holding `use.blend`, which links that group and puts it on an object.

All of the following ran in 4.5 to 5.2 on **both machines**, which agreed, and in 5.3 on the **Mac alone**, where it came out the same way.

**A `//` inside a linked node group's Import node is measured from the library file, not from the file doing the linking**, with the same result in every tested version and machine combination. Opened with no `model.obj` beside `use.blend` at all, the setup produced three vertices — the library's file. A **different**, four-vertex `model.obj` put beside `use.blend` changed nothing: still three. With the library's own `model.obj` moved away and the four-vertex copy still sitting beside `use.blend`, the setup produced nothing, and Blender named the library folder's `model.obj` as the file it could not open. Identical in all five.

**What the linking file records.** One library entry naming the other `.blend` by a relative path, `//../lib/lib.blend`, with no parent. The node group itself carries a pointer to that library and is marked as linked directly. The picture the library's setup uses arrives as a **further, indirectly linked** data-block, keeping its own `//tex.png`, and asking Blender to turn that into a real path with the library taken into account gives a path inside the library folder. Asking for its pixels loads it from there.

**The linking file can write to the linked node's path from a script, and the new value is measured from the library too.** Writing a new path into that socket was accepted in all five versions, and Blender then looked for the new file in the **library** folder, not beside the linking file.

**When the library itself is gone.** Blender warns that it cannot open it, records that the node group is missing from it, leaves a placeholder in its place, warns that one library and one linked data-block are missing, and exits normally. The library entry and its path are still written down. The node's Path can no longer be read or written, because the node is not there. Report Missing Files names the library. Identical in all five.

**Packing the library carries the setup, and does not carry the setup's own outside files.** After Pack Linked Libraries and a save, the library entry still holds the same path and now also holds a packed copy. Opening that file with everything in place gives the geometry. Opening it with `lib.blend` alone moved away **still** gives the geometry — the setup is inside the file now. Opening it with the whole `lib` folder moved away gives nothing: the geometry is gone and the picture will not load. Identical in all five. **One line of that last step is not identical**, and it is the same split as everywhere else: Report Missing Files named only the picture in 4.5, and named the picture *and* the library's `model.obj` in 5.0, 5.1, 5.2 and 5.3.

**A chain of libraries.** A third file, `deep.blend`, holds a setup reading `//model.obj` with a three-vertex file beside it; `mid.blend` links that setup, wraps it, and has a four-vertex decoy beside it; `top.blend` links the wrapper and has a five-vertex decoy beside it. Opening `top.blend` gave **three** vertices — the deepest file's own — so the path is measured from the file the setup lives in, not from either file above it. `top.blend` records both libraries, with the deep one's parent set to the middle one, and marks the deep setup as indirectly linked. Move the deep folder's `model.obj` away and the geometry goes. Identical in all five. **What this does not show:** the three files sit in sibling folders, so the recorded path `//../deep/deep.blend` names the same file whether it is measured from the middle file's folder or the top file's, and which of the two Blender measures it from is not settled in any version.

**Whether the library path is recorded relative or absolute is a choice, and the same choice in all five.** Loading a library from Python with "relative" asked for stored `//../lib/lib.blend`; with "relative" refused, it stored the full path; with nothing said either way, it stored the full path. The interface's own link command has "relative path" switched on by default and stored the relative form. Identical in all five.

### What remains inferred for a local modifier

**File paths supplied on a local modifier using a linked group.** A File Path input can have its value on the modifier in the linking file rather than inside the linked group. This storage claim is **inferred** in 5.0, 5.1, 5.2 and 5.3. Which file such a relative input value is measured from is unresolved in 5.0 and inferred in 5.3. No linked-group test here settles this input route in any version; question 1's 5.3 modifier-input test uses a local group.

### What was settled on the Mac alone

The rented Linux machine has no display server at all, so nothing drawn in a window could be reached there, and it ran nothing in 5.3; the bake through a linked group needs no window but was not run there either. None of these is a difference between versions.

**The Path field on a linked group's Import node is drawn greyed out.** Set beside the same node in a **local** group, which draws the path in bright white with a bright folder button, the linked one draws the same text grey and its button dimmed. All five versions drew it so. Read from a close crop in 5.3, and in Blender 5.2.2 on the same Mac, the dimmed button carries a folder with a bent arrow across it rather than the plain folder. The whole of the right-hand Group panel was drawn greyed out in the same way in all five. **This is worth setting against what a script can do:** writing a new value into that same socket from Python was accepted in all five. So the window and the script route differ — the window will not let the value be typed and a script will.

**The modifier for a linked group holding no Bake node.** Blender reports no bake entries for it, which is what a group holding no Bake node gives, and its node-group field is drawn greyed out. In 4.5 to 5.2 it was drawn with that field and one collapsed section headed Manage, and what Manage holds was not found out: the one switch tried for opening it, `ui_expand_flag`, does not exist in any of the four, and no other was tried. **In 5.3** the modifier has a switch for Manage, which the file holds shut; opened by it, Manage holds two collapsed sections, **Bake** and **Named Attributes**, and Bake, opened in turn, shows the modifier's own Bake Target, **Packed**, and an empty Bake Path with a folder button, neither greyed out.

**A linked group that *does* hold a Bake node gives the modifier one bake entry, and the bake goes beside the linking file.** With Bake Target set to Disk, the modifier's Bake Path filled itself in as `//blendcache_bakeuse/Target_GN` — measured from the **linking** file — and the bake landed in the `shot` folder next to `bakeuse.blend`. The library folder gained nothing. Identical in all five. In 5.3, and in Blender 5.2.2 on the same Mac, a bake with Bake Target left at Packed wrote no file at all. As under question 1, the bake-file counts do not establish a general version rule: the first way gave two in 4.5 and one in 5.0 to 5.2, and the second way two in 5.3 and 5.2.2 on the same Mac.

**What the Image Editor shows for a linked group's picture**, drawn in 5.3 alone. The editor names the picture with a chain-link icon and draws it at its own size, a tiny red square. Its sidebar shows the path as the library holds it, `//tex.png`, with the picture's settings drawn dimmed except a reload button; it does not show which folder that path is measured from. A script asking the same thing, with the library taken into account, gets a path inside the library folder.

### What differs between versions

**1. Whether a linked setup's Import path is part of the linking file's own bookkeeping.** The same split as under question 1, showing through the linked case.

- **Blender 4.5.** In `use.blend`, Blender's list of outside paths held the library and the library's picture, and not the library's `model.obj`. Its map of data-blocks to paths mapped the node group to the library only. With the library's `model.obj` moved away, Report Missing Files in `use.blend` said "No missing files".
- **Blender 5.0, 5.1, 5.2 and 5.3.** The list of outside paths held the library's `model.obj` as well. The map linked the node group to both the library **and** `//model.obj`. With the library's `model.obj` moved away, Report Missing Files named it, and said it came from the linked node group, from that library.

The same split showed in the chained test: with the deepest file's `model.obj` moved away, 4.5 reported no missing files and the other four named it and said which library it belonged to. Run in 4.5 to 5.2 on both machines, which agreed, and in 5.3 on the Mac.

**2. What a packed library records about itself.** After Pack Linked Libraries:

- **Blender 4.5** — the library holds a packed copy, and there is no "this data-block is linked and packed" mark to ask about at all.
- **Blender 5.0, 5.1, 5.2 and 5.3** — the library holds a packed copy and also reports whether it is an archive and which library it is an archive for, and the node group carries the packed mark (which read as not-packed in this test). Every library entry in these four prints an archive flag where 4.5 has no such thing.

**What packing actually carries is identical in all five**, as set out above. Run in 4.5 to 5.2 on both machines, and in 5.3 on the Mac.

**3. What the Python library loader will tell you about a library's own libraries.** In **4.5, 5.0 and 5.1** there is nothing to ask; the loader has no such thing. In **5.2 and 5.3** it has one, and for a file that links nothing it came back empty in both. 5.2's release notes introduce it, and their worked example shows two node `.blend` files among the libraries listed. Run in 4.5 to 5.2 on both machines, and in 5.3 on the Mac.

5.3 was also asked about files that do link others. For `mid.blend`, which links `deep.blend`, it listed `//../deep/deep.blend`. For `top.blend`, which links `mid.blend`, it listed only `//../mid/mid.blend` — not `deep.blend`, although `top.blend`, once opened, records both libraries. So in 5.3 it lists a file's own libraries and not theirs. Whether 5.2 does the same was not asked.

**4. Which node `.blend` files each Blender ships** — as under question 1, and it matters here because a setup built on one of them makes the file point at it; 5.3 shows this happening for Smooth by Angle, under question 1. Two node files in 4.5, four in 5.0 and 5.1, six in 5.2 and 5.3, and 4.5's `smooth_by_angle.blend` is gone from the other four.

**5. Which packed linked data-blocks a file counts as directly linked, and so writes.** 5.3's release notes say: "A linked packed ID is now considered directly linked if it's used by a local ID, _or by any other packed ID also directly linked_", and "code writing linked data now only writes directly linked packed IDs". That is a change in 5.3 by its own notes. No test on this topic exercised it: every packing test came back in 5.3 as in 5.0 to 5.2, so what it changes in practice is not shown here.

### One thing named in a release note that nothing here settles

5.2's release notes record a fixed bug, "Error in relative path in GN modifier", without saying what the behaviour was before or is now. Every test on this topic that could bear on it was run in 4.5 to 5.2 on both machines and came back **identical**, and came back the same again in 5.3 on the Mac — which folder a `//` inside a linked group is measured from, what the linking file records, what packing carries, what a chain of libraries records, and whether the library path is stored relative or absolute. So whatever that fix changed, nothing asked here shows it, and **whether it changed any behaviour at all is not settled**.

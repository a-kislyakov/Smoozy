# Dependencies — Open Threads questions — Blender 5.2

**Sources consulted, and when.** The Blender 5.2 manual (plain-text copy plus its `_images` folder), the Blender 5.2 Python API documentation, the Blender 5.2 release notes, and the Cycles 5.2 add-on files — all of them the copies inside `Blender Sources/`, taken on 2026-07-30. That snapshot date is my date read for everything below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Question answered: 20 only.

---

## 20. When a `.blend` file is opened and saved again, and none of the places it records for outside files is touched while it is open, what does the saved file record for each of those places?

### Reading of the question

I read this as: plain **File ‣ Save** back over the same file, nothing done to any recorded location while the file was open. The other reading is **File ‣ Save As…** to a different folder, which is a different case because a documented option (Remap Relative) then applies. I answer the first reading and give the second reading's documented facts underneath, so both are covered.

### LABEL: inferred (built on documented parts, listed below)

### The answer

Each untouched entry is written back in the same form it was read in: a `//`-relative entry stays `//`-relative, and an absolute entry stays absolute. Nothing documented about a plain save converts one form into the other.

This is about the form only. Whether the rest of the text comes back character for character is not covered here, and is listed under NOT COVERED below.

This rests on documented facts plus one step I had to take myself, so the whole answer is labelled inferred. Here are the documented parts, then the step.

**Documented part 1 — a recorded location is one of two forms.**

> "Blender keeps track of all unpacked resources via a relative or absolute path."

Source: manual, `https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html` (External Data section). Read 2026-07-30.

And the relative form is stored with the double-slash prefix:

> "Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."
> "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/blend/open_save.html` (Relative Paths section). Read 2026-07-30.

The API says the same about the stored value of individual properties. For a linked library:

> "Path to the library .blend file (default "", never None, blend relative // prefix supported)"

Source: Python API, `https://docs.blender.org/api/5.2/bpy.types.Library.html` (`filepath`). Read 2026-07-30.

For an image:

> "Image/Movie file name (default "", never None, blend relative // prefix supported)"

Source: Python API, `https://docs.blender.org/api/5.2/bpy.types.Image.html` (`filepath`). Read 2026-07-30.

For the render output location:

> "Directory/name to save animations, # characters define the position and padding of frame numbers (default "//", never None, blend relative // prefix supported, Supports template expressions)"

Source: Python API, `https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html` (`filepath`). Read 2026-07-30.

**Documented part 2 — the one save-time option that changes recorded locations only applies to a different directory.**

> "Remap Relative
> This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/blend/open_save.html` (Save As ‣ Options). Read 2026-07-30.

The operator documentation says the same twice, once for each save operator, and gives the defaults:

> "bpy.ops.wm.save_mainfile(*, filepath='', … compress=False, relative_remap=False, exit=False, incremental=False, show_save_modified_images_dialog=False)"
> "relative_remap (bool) – Remap Relative, Remap relative paths when saving to a different directory (optional)"

> "bpy.ops.wm.save_as_mainfile(*, filepath='', … compress=False, relative_remap=True, copy=False, show_save_modified_images_dialog=False)"
> "relative_remap (bool) – Remap Relative, Remap relative paths when saving to a different directory (optional)"

Source: Python API, `https://docs.blender.org/api/5.2/bpy.ops.wm.html`. Read 2026-07-30.

So plain Save carries the option off by default, Save As carries it on by default, and in both cases the words describe the option as acting "when saving to a different directory". Reading those words as a boundary — that the option never acts otherwise — is a step beyond them, and it is part of the step set out at the end of this section.

The manual's picture of the Save As panel shows the same three options with Remap Relative ticked:

Image: `_images/files_blend_open-save_file-browser-save.png`, shown on `https://docs.blender.org/manual/en/5.2/files/blend/open_save.html` as "The File Browser in save configuration." What is visible in it: a "Save As Blender File" panel with three checkboxes — Compress (ticked), Remap Relative (ticked), Save Copy (unticked). LABEL for this: documented. Read 2026-07-30.

**Documented part 3 — the changes to recorded locations are things you have to ask for.**

The manual lists them as menu commands under File ‣ External Data, i.e. actions, not side effects of a save:

> "Make Paths Relative
> Make all paths to external files relative to the current blend-file."
> "Make Paths Absolute
> Make all paths to external files absolute (= full path from the system's root)."

And, about the repair command:

> "Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative."

Source: manual, `https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html`. Read 2026-07-30.

**Documented part 4 — the preference that governs new paths does not reach an existing one.**

> "Default To – Relative Paths
> Default value for Relative Paths when loading external files such as images, sounds, and linked libraries. It will be ignored if a path is already set."

Source: manual, `https://docs.blender.org/manual/en/5.2/editors/preferences/save_load.html`. Read 2026-07-30.

That is the closest the sources come to saying outright that an already-recorded location is left alone: the setting that decides the form of a location is explicitly stated not to act on one that already has a value.

**Documented part 5 — for one class of recorded location, the text is stated to stay as written.**

The render output path is documented as taking template expressions — the API line quoted above ends "Supports template expressions". The manual states that the substitution never reaches the stored field:

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/file_paths.html` (Path Templates). Read 2026-07-30.

This is about substitution rather than about saving, and the words quoted here name only the render output path as taking templates. But it is a direct statement that what a path field holds is the literal text a person put there, not a resolved version of it.

**The step I took.** From "the only documented mechanisms that alter a recorded location are commands a person runs (Make Paths Relative / Absolute, Find Missing Files, pack and unpack) and the Remap Relative option, which is documented as acting when saving to a different directory", together with "the preference that sets path form is ignored when a path is already set", I concluded that a plain save back to the same file writes each untouched entry with the value it already had. That is an argument from the absence of any documented mechanism, which is weaker than a statement. No page in the 5.2 manual or API documentation says in so many words what a save writes for an untouched path. If some quiet rewriting happens on save that the documentation does not mention, this answer would not catch it.

### The other reading: Save As to a different folder

LABEL: documented in part, inferred in part.

**Documented**, from the quotes in part 2: the option is called Remap Relative, it is on by default for Save As (`relative_remap=True`) and off by default for Save (`relative_remap=False`), and it "remaps Relative Paths (such as linked libraries and images) when saving a file in a new location". The sources do not spell out what the remapped text looks like.

**Inferred**: that absolute paths are left alone, since the option names relative paths and only those. That is a reader's step from a name, not a statement.

### One documented way a save changes the file's contents

Auto-packing. If Automatically Pack Resources is on, a save changes the file's contents even though nobody touched a path:

> "Mark all eligible external resource files used by the blend-file as packed. Actual packing will happen on the next save of the blend-file."
> "When enabled, this option will ensure that all eligible external resource files, existing or added later, are systematically marked as packed."
> "As with Pack Resources, the blend-file must be saved to the drive for this to have an effect."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/blend/packed_data.html`. Read 2026-07-30.

Whether the recorded location survives alongside the packed copy: the same page says a packed item still shows a path — "You know that a data is packed when you see a little "gift box" icon displayed next to its path" — and the unpack options are offered as "Use <original file path> (differs)|(identical)" and "Create <original file path>", which only make sense if the original location is still recorded. Concluding from those two that the path is kept in the saved file is a step I took; LABEL for that sub-claim: inferred.

The same page also notes what cannot be packed: "Some typically heavy external files, like videos from the Sequence Editor or Movie Clips, cannot be packed in a blend-file." Reading that as meaning such an item therefore always stays an outside reference is a step beyond the words; LABEL for that sub-claim: inferred.

### Cycles' own files

I searched `cycles_5.2` for path-shaped settings — `grep -rn "DIR_PATH\|FILE_PATH"` across the whole folder. The only hits are `osl.py` line 148 and `operators.py` lines 41, 47, 123, 129, 135, and every one of those is a property on an operator (`CYCLES_OT_denoise_animation`'s `input_filepath` / `output_filepath`, `CYCLES_OT_merge_images`'s `input_filepath1` / `input_filepath2` / `output_filepath`) or an OSL script UI field — not a setting stored on a scene or a data-block. I also searched `properties.py` for "cache"; the hits are `debug_use_texture_cache_eviction` (a BoolProperty) and `debug_texture_cache_preserve_unused` (an IntProperty), neither of which holds a location.

LABEL for the search itself: **observed**. What was searched, in what, and what came back is written out above, by file and line. No line of any Cycles file is quoted here, so what stands as the record is the search and its hits rather than words on a page.

LABEL for the wider conclusion — that Cycles therefore declares no external-file location stored in the `.blend`: **inferred**. Two search words were used, and a location could be declared without either of them appearing. What the search shows is that those two words turn up only on operator properties and an OSL script field.

Either way these files say nothing about what a save records, and what a render does with any of these at run time is beyond what the files state.

### NOT COVERED

- Byte-for-byte identity. Nothing in the sources says whether the saved string is character-for-character what was loaded. Capitalisation, forward versus back slashes, a trailing slash, `..` segments, doubled separators — no page says whether any of these is tidied up when the file is written. My answer covers the *form* (relative stays relative, absolute stays absolute), not the exact characters.
- Broken entries. The sources do not say what a save writes for a location whose file is no longer there. Report Missing Files is documented as a way to "check if there are links to unpacked files that no longer exist", which implies such links persist, but nothing states that a save keeps them rather than clearing them.
- Locations inside linked libraries. The question is about the file being saved; the sources say nothing here about what happens to paths recorded inside a library that file links to.
- Version conversion. The 5.2 release notes say "asset files with Geometry Nodes tools must be re-saved to be used properly in 5.2" (`https://developer.blender.org/docs/release_notes/5.2/` — the subject page this line sits on is not recorded, so what is given is the version's root rather than the page itself), which shows that opening and re-saving can update file contents. Nothing says whether any such conversion ever touches recorded locations.
- Modified images. A save of the blend-file may also write image files, depending on a preference — "Always Save: Automatically saves all modified images when saving the blend-file" (`https://docs.blender.org/manual/en/5.2/editors/preferences/save_load.html`). The sources do not say which location those writes go to, so I cannot settle whether that can change what the file records. (Image ‣ Save is documented as "Save the image to its current path" — manual, `https://docs.blender.org/manual/en/5.2/editors/image/editing.html`, read 2026-07-30 — but that is the manual command, not the automatic one. *Citation check, 2026-08-30 — Claude Opus 5: the quote carried no address of its own. Its page is named in the record of looking below, and I have brought the address up beside the words. The words are unchanged.*)
- Save Copy. The manual documents it as "Saves a copy of the actual working state but does not make the saved file active", and it takes the same options as Save As. What the copy records for untouched locations when it lands in a different folder is not stated separately.

---

## 20 — observed. What a running Blender does

**Label: observed.** Everything under this heading was got by running Blender 5.2 and reading what it printed and what it wrote to disk. It is not a reading of any documentation, and it does not replace the inferred answer above; where the two disagree, say so plainly rather than quietly preferring one.

### On what kind of computer, and when

On 2026-08-30 these were run twice over: once on the Creator's own computer — a MacBook Pro with an Apple M1 Pro chip running macOS 26.6.2, with Apple's own graphics — and once on a rented Linux machine — Ubuntu 22.04.4 LTS with one NVIDIA RTX 3060, twelve cores and 31 GB of memory, in Poland, driven over SSH, rented and destroyed the same day. The build hash of every Blender used on the rented machine is the same as the one on the Mac, so where the two disagree the difference is the operating system and not the build. Two results are reported wherever both machines ran a test, and they are never folded into one sentence.

### What was done, and to what

A `.blend` file was built by script in Blender 5.2, so that the exact text of every place it records for an outside file was known rather than typed. Beside it sat `assets/tex.png`, `assets/two words.png`, `assets/beep.wav`, `assets/testfont.ttf`, `assets/test.osl`, `assets/test.ies`, a folder named `Тестовая папка` holding `текстура_日本語.png`, a second blend-file `lib.blend`, and one folder up an `outer.png`.

Inside the file were twelve images, all pointing at the same picture but each written a different way, plus a link to `lib.blend`, a data-block appended out of it, a font, a sound, a text, a volume, a compositor File Output node, a cloth cache, a geometry-nodes bake folder, a sequencer proxy folder, an OSL script node, an IES node, the render output path and the bake path. The twelve ways of writing an image's place were: plain relative, a full path from the system root, a `..` segment that still resolves, a doubled separator, a **backslash** where the machine wants a forward slash, the wrong letter case, non-Latin characters, a name with a space in it, a trailing space, a place that does not exist, a place one folder up, and one set through `filepath_raw` rather than `filepath`. Two of the images were given no users, one of them shielded with a fake user and one not.

Four runs followed, each started with the application's own path and `--background --factory-startup`. The first built and saved the file with `bpy.ops.wm.save_as_mainfile(filepath=…, relative_remap=False, compress=False)`, printing what every place held in memory. The second opened the file, touched nothing, saved nothing, and printed every place. The third opened it, touched nothing, and called `bpy.ops.wm.save_mainfile()` with no settings passed at all. The fourth opened it again and printed every place. A copy of the file was kept from before the plain save, and the readable text inside both copies was pulled out with `strings -a`.

### What came back

**Every recorded place kept its exact text through the plain save, except one.** A place written with a backslash came back written with a forward slash.

Setting the text in memory kept the backslash:

```
SET	rel-backslash	wanted='//assets\\tex.png'	got_filepath='//assets\\tex.png'	got_raw='//assets\\tex.png'
SET	raw-setter	wanted='//assets\\..\\assets//TEX.PNG '	got_filepath='//assets\\..\\assets//TEX.PNG '	got_raw='//assets\\..\\assets//TEX.PNG '
```

The build's own save wrote the backslash into the file. Read straight out of that file's bytes, the two lines are there:

```
//assets\..\assets//TEX.PNG 
//assets\tex.png
```

Opening that file and printing without saving anything already shows forward slashes:

```
image[rel-backslash].filepath	'//assets/tex.png'
image[rel-backslash].filepath_raw	'//assets/tex.png'
image[raw-setter].filepath	'//assets/../assets//TEX.PNG '
image[raw-setter].filepath_raw	'//assets/../assets//TEX.PNG '
```

And after `bpy.ops.wm.save_mainfile()` with nothing touched, searching the file itself for a line beginning `//assets\` returns nothing at all. The forward-slash forms are there in their place. **So the re-spelling happens when the file is opened, and the plain save makes it permanent.**

Everything else survived the save character for character: the doubled separator `//assets//tex.png`, the `..` segment `//assets/../assets/tex.png`, the wrong letter case `//assets/TEX.PNG`, the trailing space `//assets/tex.png `, the non-Latin `//Тестовая папка/текстура_日本語.png`, the name with a space `//assets/two words.png`, the place that does not exist `//nowhere/missing.png`, the place one folder up `//../outer.png`, the full path, the templates `//out/{blend_name}_####.png` and `//comp_out/{blend_name}/`, the library `//lib.blend`, the font, the sound, the volume, the text, the OSL script node, the IES node, the bake path `//bake/baked.png`, the cloth cache `//cache/`, the geometry-nodes bake folder `//gn_bake/`, and the sequencer proxy folder `//proxies/`. The two settings a script can read a place through, `filepath` and `filepath_raw`, printed the same string every single time.

**The Mac and the rented Linux machine agreed on every one of those, in both directions.** A file built on the Mac was also carried to the rented machine and plain-saved there; the same two backslash lines were in it when it arrived and gone after the Linux save, and no other line of path text moved.

### Two recorded places vanished at the plain save, with nobody touching them

Comparing what the file held before the plain save with what it held after, on both machines, the only other differences were these:

```
-image[lib-inside-image].filepath	'//assets/tex.png'
-image[lib-inside-image].filepath_raw	'//assets/tex.png'
-image[lib-inside-image].packed	False
-image[lib-inside-image].users	0
-image[lib-inside-image].size	(2, 2)
-weakref[images:lib-inside-image].filepath	'//lib.blend'
-weakref[images:lib-inside-image].id_name	'IMlib-inside-image'
```

That is a data-block with no users and no fake user, dropped at the save, taking its recorded place with it — and beside it the weak reference, which recorded `//lib.blend` and went too. The second unused image in the same file, the one carrying a fake user, survived every save. A third, built with no users and no fake user, never reached the first reading at all, because the build's own save had already dropped it.

The two files on disk were never the same size and never the same file: 650,582 bytes before the plain save and 647,314 after, with different checksums. **So comparing whole files says nothing; only the path text can be compared.**

### Two ways of asking Blender for every recorded place, and they disagree

In the same run, on the same file, before it had been reloaded, `bpy.utils.blend_paths` reported the backslash entries as they were stored:

```
'//assets\\tex.png', '//assets\\..\\assets//TEX.PNG '
```

while `bpy.data.file_path_map` reported the same two entries with forward slashes:

```
('rel-backslash', ['//assets/tex.png'])
('raw-setter', ['//assets/../assets//TEX.PNG '])
```

So which way a check asks for a recorded place decides what string it gets back. Blender 5.2 has `bpy.utils.blend_paths` and `bpy.data.file_path_map`, and `bpy.data.file_path_foreach` is there:

```
has bpy.utils.blend_paths	True
has bpy.data.file_path_map	True
has bpy.data.file_path_foreach	True
```

**Note added by the citation check, 2026-08-30 — Claude Opus 5.** The disagreement described just above did not come back when this check repeated the run on the Creator's Mac, and the raw output the run itself left behind does not show it either. In my repeat, in the build run, before the file had ever been reopened, **both** routes reported the two backslash entries with their backslashes:

```
bpy.utils.blend_paths()	[… '//assets\\..\\assets//TEX.PNG ', '//assets\\tex.png' …]
bpy.data.file_path_map(include_libraries=True)	[… ('raw-setter', ['//assets\\..\\assets//TEX.PNG ']), ('rel-backslash', ['//assets\\tex.png']) …]
```

and in the next run, after the file had been opened from disk, both reported them with forward slashes:

```
bpy.data.file_path_map(include_libraries=True)	[… ('raw-setter', ['//assets/../assets//TEX.PNG ']), ('rel-backslash', ['//assets/tex.png']) …]
```

So on this machine the two routes agree with each other, and what changes the spelling is the reload, not the route. **Nothing above has been overwritten** — I was not in the room for the first run, and two runs that differ are a question, not an answer. This has gone back to the hand tests to be run again. Both runs are written out in full in `Audits/Blender Research/Dependencies/Citation check/Open Threads questions — Claude Opus 5.md`.


**Third run, 2026-08-30 — Claude Opus 5, on the Creator's Mac.** The two runs above disagree, so the test was run a third time. **The third run agrees with the citation check's repeat and not with the first run.** All three runs now stand and none has been overwritten; which of them is right is not settled by seniority, only by what each printed. Label: **observed**.

*What was done, and on what.* On the Creator's own computer — a MacBook Pro with an Apple M1 Pro chip, macOS 26.6.2, Apple's own graphics, no second card — the researcher's own scripts were run unchanged, in a throwaway folder outside the project, with a settings folder of its own. Nothing was rented and no system setting was changed. The Blender used said of itself:

```
VERSION 5.2.1 LTS HASH 9e2066aef7ef
PLATFORM darwin
```

The starting file was built fresh, and both questions were asked twice over: once **inside the process that had just built the file and had never reloaded it**, and once in a **fresh process that opened the file from disk**. The two commands in full, after the application's own path: `--background --factory-startup --python build_main.py -- <project folder> <scripts folder>`, then `--background --factory-startup <the file> --python read_only.py -- <scripts folder>`. The two routes were compared character for character by script rather than read by eye.

*What came back inside the process that built the file, before any reload:*

```
5.2  rel-backslash file_path_map -> ['//assets\\tex.png']
5.2  raw-setter    file_path_map -> ['//assets\\..\\assets//TEX.PNG ']
5.2  blend_paths entries holding a backslash -> ['//assets\\..\\assets//TEX.PNG ', '//assets\\tex.png']
```

*And in the fresh process that had opened the file from disk:*

```
5.2  rel-backslash file_path_map -> ['//assets/tex.png']
5.2  raw-setter    file_path_map -> ['//assets/../assets//TEX.PNG ']
5.2  blend_paths entries holding a backslash -> []
```

So on this machine the two routes reported those two entries the same way as each other, and what changed the spelling was the reload rather than the route.

*One thing that is not the disagreement described above, and is recorded rather than folded into it.* The two routes do not return the same **collection** of strings. Compared as multisets in the build process:

```
5.2 : blend_paths 24 strings, file_path_map 27 strings
     only file_path_map: '//assets/tex.png'
     only file_path_map: '//lib.blend'
     only file_path_map: '//lib.blend'
```

with a fourth extra copy of `'//lib.blend'` after the reload. Those are repeated copies of places both routes report, not places one route spells differently.

*What this does not cover.* The Creator's Mac, and nothing else. The rented Linux machine the first run also used is destroyed, so this settles macOS and leaves every other operating system open. It also does not establish what happened in the first run — only that two of the three runs on this machine read one way, and so does the raw output the first run itself left behind.

Every line the machine printed is in `Audits/Blender Research/Dependencies/Hand tests — Open Threads questions — raw output/Mac re-run 2026-08-30/`, and the entry that ran it is entry 18 of `Audits/Blender Research/Dependencies/Hand tests — Open Threads questions.md`.


### Saving somewhere else, which is the other reading of the question

From the same starting file, `bpy.ops.wm.save_as_mainfile(filepath=…)` into a different folder, with nothing passed for the remap setting, rewrote every relative place a file is **read** from, and tidied it while doing so:

```
-image[rel-plain].filepath	'//assets/tex.png'
+image[rel-plain].filepath	'//../proj/assets/tex.png'
-image[rel-dotdot].filepath	'//assets/../assets/tex.png'
+image[rel-dotdot].filepath	'//../proj/assets/tex.png'
-image[rel-doubleslash].filepath	'//assets//tex.png'
+image[rel-doubleslash].filepath	'//../proj/assets/tex.png'
-image[trailing-space].filepath	'//assets/tex.png '
+image[trailing-space].filepath	'//../proj/assets/tex.png '
-image[missing].filepath	'//nowhere/missing.png'
+image[missing].filepath	'//../proj/nowhere/missing.png'
-library[lib.blend].filepath	'//lib.blend'
+library[lib.blend].filepath	'//../proj/lib.blend'
```

The `..` segment and the doubled separator were collapsed; the trailing space and the letter case were kept; a place pointing at nothing was rewritten too; and the place written as a full path was not touched.

**Every place a file is written *to* was left alone by that remap** — the render output path, the bake path, the compositor node's directory, the cloth cache, the geometry-nodes bake folder and the sequencer proxy folder all still read exactly what they read before, which after the move means a different place on disk.

With `relative_remap=False` the text was unchanged and the pictures stopped loading, printing a size of zero by zero. Into the **same** folder under a new name, with nothing passed, nothing was rewritten at all; and `bpy.ops.wm.save_mainfile(relative_remap=True)` over the same file in the same folder rewrote nothing either. So "a different directory" is a real boundary.

`bpy.ops.wm.save_mainfile(incremental=True)` wrote a numbered copy beside the original — `main1.blend` — and changed no recorded place.

### Other things a save does, or does not do

**Packing.** Switching on Automatically Pack Resources and then saving left every recorded place with exactly the text it had, and only turned the packed mark on:

```
-image[abs-plain].packed	False
+image[abs-plain].packed	True
-image[rel-plain].packed	False
+image[rel-plain].packed	True
```

That run needed a file in which every place points at something that exists. On the file above, which holds places pointing at nothing, the command threw and the save never happened:

```
RuntimeError: Error: Unable to pack file, source path '…/nowhere/missing.png' not found
```

**The relative-paths preference.** Written to off in a settings folder of its own, and the file then opened without `--factory-startup` so the preference was actually read, a plain save changed no recorded place at all.

**The render output given on the command line.** With `-o` before the file, loading the file overwrote it and the save wrote nothing new. With `-o` after the file, the value stuck and the plain save wrote it into the file:

```
render filepath in memory = '/tmp/smoozy_q20_after'
render filepath = '/tmp/smoozy_q20_after'
```

So a run that sets the render output from the command line and then saves has touched a recorded place, with nobody editing anything.

**Automatically written files.** On the rented Linux machine, under a display server, Blender was run with a real window, an object was moved, and the session was ended. The quit file it left records the same text for every place as the file it came from; the pictures do not load from it only because it sits in the temporary folder, so a place written relative no longer reaches them. Blender 4.5 also wrote an auto-save file, named `main_<number>_autosave.blend`, recording the same text; 5.0, 5.1 and 5.2 wrote no auto-save file in three separate runs. Nothing here says whether that is a change in Blender or a difference in when a version decides a file has been edited.

### The shipped settings of the two save commands, read out of this build

```
save_as relative_remap | name Remap Relative | default True  | Remap relative paths when saving to a different directory
save    relative_remap | name Remap Relative | default False | Remap relative paths when saving to a different directory
save    incremental    | name Incremental    | default False | Save the current Blender file with a numerically incremented name that does not overwrite any existing files
save_as copy           | name Save Copy      | default False | Save a copy of the actual working state but does not make saved file active
```

```
preference use_relative_paths   True
preference use_file_compression True
```

**A file written by Blender 4.5, opened here and plain-saved.** Version conversion rewrote no recorded place. The compositor File Output node's single recorded place was carried straight into this version's two settings, with the file-name half left empty:

```
-fileoutput[ProbeFileOutput].base_path	'//comp_out/{blend_name}/'
-fileoutput[ProbeFileOutput].directory	MISSING
-fileoutput[ProbeFileOutput].file_name	MISSING
+fileoutput[ProbeFileOutput].base_path	MISSING
+fileoutput[ProbeFileOutput].directory	'//comp_out/{blend_name}/'
+fileoutput[ProbeFileOutput].file_name	''
```

**The setting for saving modified pictures is new in this version**, and so is one extra setting on the save command:

```
save_modified_images | type ENUM | default ASK | ['ASK', 'ALWAYS_SAVE', 'NEVER_SAVE']
save_mainfile settings: [… 'compress', 'relative_remap', 'exit', 'incremental', 'show_save_modified_images_dialog']
```

With no window, "ask" behaved as "never": nothing stopped and nothing waited. At all three of its settings the recorded place was the same string; only whether the picture's own file was rewritten changed.

**Saving a file writes nothing into a library it links.** `lib.blend` had the same size, the same time and the same checksum before and after the parent was plain-saved.

### What these runs do not cover

* **Windows.** These particular runs were the Creator's Mac and the rented Linux machine, and neither speaks for Windows. The Windows half of the question was run separately on a rented machine reporting itself as Microsoft Windows Server 2025 Standard, and it has an observed section of its own below, after this list; what that section leaves open is listed there.
* **Three kinds of recorded place** could not be built into the test file because each needs a real file of its kind to import: a cache file, a movie clip and a movie strip. Nothing here says how those behave.
* **What the file physically holds** beyond its readable text. The comparison above is of path text pulled out of the file, not of its structure.

The full list these came from, and every machine's own words in full, are at `Audits/Blender Research/Dependencies/Hand tests — Open Threads questions.md` and in the folder beside it.


### 20 — observed on Windows. What a running Blender does there

**Label: observed.** Everything under this heading was got by running Blender 5.2.1 LTS on a rented Windows machine on 2026-08-30, and reading what it printed and what it wrote to disk. It is not a reading of any documentation. It answers the Windows half of this question, which every run before it left open, and it replaces nothing above: where a Windows result differs from the Mac's or the rented Linux machine's, both stand and the difference is named.

#### On what kind of computer, and when

On 2026-08-30, on a Windows machine rented for about half an hour and destroyed the same day. **It reported itself as a Server edition, and naming that edition is the first thing any Windows finding here has to do**, because the image it was created from was named for a desktop and the running system disagreed with the name. Asked five separate ways in one sitting, it answered (`Audits/Blender Research/Dependencies/Hand tests — Windows — raw output/Windows/00-the-machine.txt`):

```
Win32_OperatingSystem.Caption          : Microsoft Windows Server 2025 Standard
Win32_OperatingSystem.Version          : 10.0.26100
Win32_OperatingSystem.BuildNumber      : 26100
Win32_OperatingSystem.ProductType      : 3
Win32_OperatingSystem.OSArchitecture   : 64-bit
CurrentVersion\ProductName             : Windows Server 2025 Standard
CurrentVersion\EditionID               : ServerStandard
CurrentVersion\InstallationType        : Server
CurrentVersion\DisplayVersion          : 24H2
CurrentVersion\CurrentBuild            : 26100
CurrentVersion\UBR                     : 32995
ProductOptions\ProductType             : ServerNT
[System.Environment]::OSVersion        : Microsoft Windows NT 10.0.26100.0
cmd /c ver                             :  Microsoft Windows [Version 10.0.26100.32995]
```

**So every Windows answer below is an answer about Microsoft Windows Server 2025 Standard, build 26100.32995, 24H2, 64-bit.** It is not an answer about Windows 10 or Windows 11, and it must not be quoted as one.

**A second Windows machine was created first and destroyed unused, and it was a Server edition too — an older one.** It came from a different image, also named for a desktop, and answered seven ways (`Windows/00-the-machine-first-attempt-windows-10-entry.txt`, transcribed from photographs of its screen because it had no way to move a file off itself):

```
1 Caption : Microsoft Windows Server 2016 Standard
2 ProductName : Windows Server 2016 Standard
3 EditionID : ServerStandard
4 InstallationType : Server
5 ProductType : 3
6 CurrentBuild : 14393.9234
7 OSVersion : Microsoft Windows NT 10.0.14393.0
```

No Blender was put on that one and no test was run on it. It was abandoned because it has no SSH server and cannot be given one — the command that installs the feature reported success and installed nothing:

```
PS C:\Users\Administrator> Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0
Path          :
Online        : True
RestartNeeded : False

PS C:\Users\Administrator> Get-WindowsCapability -Online -Name OpenSSH*
Name      State
----      -----
          NotPresent

PS C:\Users\Administrator> Test-Path C:\Windows\System32\OpenSSH\sshd.exe
False
```

The machine that did the work described itself this way:

```
Manufacturer / Model : QEMU / Standard PC (Q35 + ICH9, 2009)
Processor            : Intel Xeon Processor (SapphireRapids); Intel Xeon Processor (SapphireRapids)
Sockets / cores / logical : 2 / 2,2 / 4
Memory (bytes)       : 8583331840
Disk                 : QEMU QEMU HARDDISK SCSI Disk Device 85896599040
C: free (bytes)      : 58021351424
Graphics             : Microsoft Basic Display Adapter; Microsoft Remote Display Adapter
Locale / time zone   : en-US / GMT Standard Time
Local time           : 2026-08-30 11:38:30 +01:00
PowerShell           : 5.1.26100.32995 (Desktop)
Hostname             : CWM9047
```

**It has no graphics card of any kind.** Every run below is a background run with no window that renders nothing, so the missing card limits none of these results.

**The Blender was the official Windows x64 build from Blender's own download server, and its hash is the same as the Creator's Mac's and the rented Linux machine's.** Asked of the running program itself, not read off a download page (`Windows/5.2-00-machine.txt`; the same four hashes are in `Windows/01-builds.txt` and `Windows/00-the-machine.txt`):

```
RESOURCES C:\q20\res-5.2
VERSION 5.2.1 LTS HASH 9e2066aef7ef
BUILD PLATFORM b'Windows'
BUILD TYPE b'Release'
BINARY C:\blender\5.2.1\blender.exe
PLATFORM win32
OS SEP '\\' ALTSEP '/'
Blender 5.2.1 LTS (hash 9e2066aef7ef built 2026-08-25 02:38:20)
```

Because the hash matches, a difference between this result and a Mac or Linux result is the operating system and not the build. Every run set its own throwaway settings folder, which is the `RESOURCES` line above.

#### What was done, and to what

The same starting file as the Mac and Linux runs, built by the same scripts so a Windows result can be laid beside them line for line. Beside it sat `assets\tex.png`, `assets\two words.png`, `assets\beep.wav`, `assets\testfont.ttf`, `assets\test.osl`, `assets\test.ies`, a folder named `Тестовая папка` holding `текстура_日本語.png`, a second blend-file `lib.blend`, and one folder up an `outer.png`. It held the same images, library link, appended data-block, font, sound, text, volume, compositor File Output node, cloth cache, geometry-nodes bake folder, sequencer proxy folder, script node, light-profile node, render output path and bake path.

**Two things were different, and both are additions rather than changes.** The font was copied from `C:\Windows\Fonts\arial.ttf`, because it has to be a font that exists on the machine doing the building. And **eight more spellings were added to the twelve**, being the ones only Windows can have: an absolute path written with forward slashes; the same written with backslashes; the same with a lower-case drive letter; the same with an upper-case one; a path naming a drive but no root; a place on a second drive; a relative place written with a doubled backslash; and a relative place with a backslash-spelled step back up. The second drive was a drive letter mapped onto a folder by Windows' own command:

```
subst list: X:\: => C:\q20\otherdrive
```

The runs, each started with the application's own path and `--background --factory-startup`. The first built and saved the file, printing what every place held in memory. The second opened it, touched nothing, saved nothing, and printed every place. The third opened it, touched nothing, and called `bpy.ops.wm.save_mainfile()` with nothing passed. A copy of the file the build wrote was kept, so the bytes before and after could be compared. The three commands in full:

```
C:\blender\5.2.1\blender.exe --background --factory-startup --python C:\q20\scripts\build_main.py -- C:\q20\work\5.2\proj C:\q20\scripts
C:\blender\5.2.1\blender.exe --background --factory-startup C:\q20\work\5.2\proj\main.blend --python C:\q20\scripts\read_only.py -- C:\q20\scripts
C:\blender\5.2.1\blender.exe --background --factory-startup C:\q20\work\5.2\proj\main.blend --python C:\q20\scripts\plain_save.py -- C:\q20\scripts
```

Six more runs followed from the same starting file, put back fresh each time: a save into another folder on the same drive with nothing passed for the remap setting, the same with the setting turned off, a save into the same folder under a new name, the make-paths-absolute command, the make-paths-relative command, and a save onto the second drive.

#### What came back — setting a place changes nothing, whichever way it is spelled

Every one of the twenty spellings came back out of the setting exactly as it went in, in memory, before anything was saved (`Windows/5.2-01-build.txt`):

```
SET	rel-plain	wanted='//assets/tex.png'	got_filepath='//assets/tex.png'	got_raw='//assets/tex.png'
SET	rel-backslash	wanted='//assets\\tex.png'	got_filepath='//assets\\tex.png'	got_raw='//assets\\tex.png'
SET	abs-forward	wanted='C:/q20/work/5.2/proj/assets/tex.png'	got_filepath='C:/q20/work/5.2/proj/assets/tex.png'	got_raw='C:/q20/work/5.2/proj/assets/tex.png'
SET	abs-lower-drive	wanted='c:\\q20\\work\\5.2\\proj\\assets\\tex.png'	got_filepath='c:\\q20\\work\\5.2\\proj\\assets\\tex.png'	got_raw='c:\\q20\\work\\5.2\\proj\\assets\\tex.png'
SET	drive-relative	wanted='C:q20\\work\\5.2\\proj\\assets\\tex.png'	got_filepath='C:q20\\work\\5.2\\proj\\assets\\tex.png'	got_raw='C:q20\\work\\5.2\\proj\\assets\\tex.png'
SET	other-drive	wanted='X:\\assets\\tex.png'	got_filepath='X:\\assets\\tex.png'	got_raw='X:\\assets\\tex.png'
```

#### What came back — opening the file re-spells the separator, and on Windows it runs the other way

**On the Mac and on the rented Linux machine a backslash becomes a forward slash. On Windows Server 2025 every forward slash becomes a backslash.** These are the same places, read on the very next open with nothing touched in between (`Windows/5.2-02-read-before.txt`):

```
image[rel-plain].filepath	'//assets\\tex.png'
image[rel-dotdot].filepath	'//assets\\..\\assets\\tex.png'
image[rel-doubleslash].filepath	'//assets\\\\tex.png'
image[case-caps].filepath	'//assets\\TEX.PNG'
image[trailing-space].filepath	'//assets\\tex.png '
image[spaces].filepath	'//assets\\two words.png'
image[nonlatin].filepath	'//Тестовая папка\\текстура_日本語.png'
image[outer-updir].filepath	'//..\\outer.png'
image[missing].filepath	'//nowhere\\missing.png'
image[abs-forward].filepath	'C:\\q20\\work\\5.2\\proj\\assets\\tex.png'
image[raw-setter].filepath	'//assets\\..\\assets\\\\TEX.PNG '
scene.render.filepath	'//out\\{blend_name}_####.png'
```

Against what the same places held in memory when the file was written, which is quoted in full in the raw output: `'//assets/tex.png'`, `'//assets/../assets/tex.png'`, `'//assets//tex.png'`, `'//assets/TEX.PNG'`, `'//assets/tex.png '`, `'//assets/two words.png'`, `'//Тестовая папка/текстура_日本語.png'`, `'//../outer.png'`, `'//nowhere/missing.png'`, `'C:/q20/work/5.2/proj/assets/tex.png'`, `'//assets\\..\\assets//TEX.PNG '` and `'//out/{blend_name}_####.png'`.

**Nothing else about the text moved.** The leading `//` stays, the doubled separator stays doubled and is only spelled the other way, the step back up stays, the letter case stays, the trailing space stays, the non-Latin characters come through whole, and a place pointing at nothing is re-spelled just the same. Only the separator changes.

**Seven lines still hold a forward slash after the open, and they are the same seven in every version.** Read by name rather than inferred from a missing line — these are every line in that output whose value still carries a forward slash:

```
library[lib.blend].filepath	'//lib.blend'
weakref[images:lib-inside-image].filepath	'//lib.blend'
scene.render.bake.filepath	'//bake/baked.png'
sequence_editor.proxy_dir	'//proxies/'
fileoutput[ProbeFileOutput].directory	'//comp_out/{blend_name}/'
object[ProbeCloth].mod[Cloth].point_cache.filepath	'//cache/'
object[ProbeGN].mod[GeometryNodes].bake_directory	'//gn_bake/'
```

**The first two are on that list only because there is no separator in them to change** — the library's own place is `//lib.blend`, with nothing between the marker and the name, and the weak reference beside it is the same string. So five real places are left alone: the compositor's output folder, the cloth cache's folder, the geometry-nodes bake folder, the bake picture's place, and the sequencer's proxy folder.

**The render output path is not one of them**, and that is worth stating because the five that survive are all places a file is written to, and the render output path is a place written to as well. It becomes `'//out\\{blend_name}_####.png'`.

**The two settings a script can read a place through printed the same string every time.** Every `filepath` line in this version's output is followed by a `filepath_raw` line carrying the same characters, in the run that built the file and in every run that opened it. Neither of them shows the stored text once the file has been opened.

#### What came back — a plain save with nothing touched makes the re-spelling permanent

The two files on disk are never the same file (`Windows/5.2-05-bytes.txt`):

```
--- size on disk ---
      691158  C:\q20\work\5.2\main_after_build.blend
      687890  C:\q20\work\5.2\main_after_plain_save.blend

--- byte for byte ---
C:\q20\work\5.2\main_after_build.blend C:\q20\work\5.2\main_after_plain_save.blend differ: char 67494, line 6

--- sha256 ---
de79b2c44ca1fc08a7c7510804ea11857c9455904da370c322f46f4689734238  C:\q20\work\5.2\main_after_build.blend
158bc2d327a4f33879e209103fabd98108ae9e2d0ad405cd535550fca4f2baa4  C:\q20\work\5.2\main_after_plain_save.blend
```

The path text in the file the build wrote, pulled out of its own bytes, with the count of each on the left as the machine printed it:

```
      1 //../outer.png
      1 //assets/../assets/tex.png
      1 //assets//tex.png
      1 //assets/TEX.PNG
      1 //assets/beep.wav
      1 //assets/nosuch.vdb
      1 //assets/test.ies
      2 //assets/test.osl
      1 //assets/testfont.ttf
      2 //assets/tex.png
      1 //assets/tex.png 
      1 //assets/two words.png
      1 //assets\..\assets//TEX.PNG 
      1 //assets\..\assets\tex.png
      1 //assets\\tex.png
      2 //assets\tex.png
      1 //bake/baked.png
      1 //cache/
      2 //comp_out/{blend_name}/
      1 //gn_bake/
      2 //lib.blend
      1 //nowhere/missing.png
      1 //out/{blend_name}_####.png
      1 //proxies/
      1 //Тестовая папка/текстура_日本語.png
      1 C:/q20/work/5.2/proj/assets/tex.png
      1 C:\Users\Administrator\Documents
      3 C:\q20\work\5.2\proj\assets\tex.png
      1 C:q20\work\5.2\proj\assets\tex.png
      1 X:\assets\tex.png
      1 c:\q20\work\5.2\proj\assets\tex.png
```

and the path text in the same file after one plain save with nothing touched:

```
      1 //..\outer.png
      1 //assets\..\assets\\TEX.PNG 
      2 //assets\..\assets\tex.png
      1 //assets\TEX.PNG
      2 //assets\\tex.png
      1 //assets\beep.wav
      1 //assets\nosuch.vdb
      1 //assets\test.ies
      2 //assets\test.osl
      1 //assets\testfont.ttf
      3 //assets\tex.png
      1 //assets\tex.png 
      1 //assets\two words.png
      1 //bake/baked.png
      1 //cache/
      2 //comp_out/{blend_name}/
      1 //gn_bake/
      1 //lib.blend
      1 //nowhere\missing.png
      1 //out\{blend_name}_####.png
      1 //proxies/
      1 //Тестовая папка\текстура_日本語.png
      1 C:\Users\Administrator\Documents
      4 C:\q20\work\5.2\proj\assets\tex.png
      1 C:q20\work\5.2\proj\assets\tex.png
      1 X:\assets\tex.png
      1 c:\q20\work\5.2\proj\assets\tex.png
```

One line in each list is not a recorded place from the test file — the account's own documents folder — and it is quoted here rather than trimmed, because the list is the machine's own output.

**One data-block went at the plain save with nobody touching it, exactly as it did on the Mac and on Linux.** Before the save the file held two images named `lib-inside-image`, one of them with no users:

```
image[lib-inside-image].filepath	'//assets\\tex.png'
image[lib-inside-image].filepath_raw	'//assets\\tex.png'
image[lib-inside-image].packed	False
image[lib-inside-image].users	0
image[lib-inside-image].size	(2, 2)
weakref[images:lib-inside-image].filepath	'//lib.blend'
weakref[images:lib-inside-image].id_name	'IMlib-inside-image'
```

and after it, one — the weak reference gone with it (`Windows/5.2-04-read-after.txt`). The unused image shielded with a fake user survived, and the third, unused and unshielded, never reached the first read because the build's own save had already dropped it.

**Why the two reads look the same although the file changed.** The re-spelling is already done by the time anything can read the file at all, so the read before the plain save and the read after it agree; the change is only visible in the bytes. **A check that reads a place through Blender and compares it against the text in the file will match on a file Windows has already saved, and will miss on a file that arrived from anywhere else.**

#### What came back — the two ways of listing every recorded place agree with each other on Windows

The Mac's first run recorded these two routes disagreeing about spelling in the process that had just built the file. **On Windows Server 2025 they do not disagree, in either state.** In the build run, before any reload, both report the backslash entry with its backslash and the plain entry with its forward slash; after the file has been opened from disk, both report every entry with backslashes (`Windows/5.2-01-build.txt` and `Windows/5.2-02-read-before.txt`):

```
has bpy.utils.blend_paths	True
has bpy.data.file_path_map	True
has bpy.data.file_path_foreach	True
```

```
in the build run, never reloaded
bpy.utils.blend_paths()   ... '//assets/tex.png', ... '//assets\\tex.png', ...
bpy.data.file_path_map()  ... ('rel-plain', ['//assets/tex.png']), ('rel-backslash', ['//assets\\tex.png']), ...

after the file was opened from disk
bpy.utils.blend_paths()   ... '//assets\\tex.png', '//assets\\..\\assets\\\\TEX.PNG ', ...
bpy.data.file_path_map()  ... ('rel-plain', ['//assets\\tex.png']), ('rel-dotdot', ['//assets\\..\\assets\\tex.png']), ...
```

The two lists were compared as counted collections of strings by script rather than by eye, so a difference of one character could not be missed. **Every way they differ is about which places they cover, never about how a place is spelled:**

```
in memory, in the build run, never reloaded
5.2 : blend_paths 32 strings, file_path_map 35 strings
     only file_path_map: '//assets/tex.png'
     only file_path_map: '//lib.blend'
     only file_path_map: '//lib.blend'

after the file was opened from disk
5.2 : blend_paths 30 strings, file_path_map 34 strings
     only file_path_map: '//assets\\tex.png'
     only file_path_map: '//lib.blend'
     only file_path_map: '//lib.blend'
     only file_path_map: '//lib.blend'
```

The extras are the linked library's own inner picture, which the second route reaches through the library and the first does not, and repeated mentions of the library itself. The one string that looks like a spelling difference is not one: it is `'//assets/tex.png'` before the reload and `'//assets\\tex.png'` after it, which is the same entry following the same re-spelling as everything else.

**So the disagreement the Mac's first run recorded is not a Windows disagreement**, and that is a clean negative rather than a silence: a check may read a recorded place through either route on this machine and get the same characters. **It does not make the underlying danger smaller — it moves it.** On Windows both routes agree with each other and both disagree with the file that arrived, whenever that file was written anywhere but Windows.

#### What came back — a relative place across two drives

**A plain save leaves it exactly alone.** `X:\assets\tex.png` was stored, read back and written again unchanged, and the picture loaded from it — the read prints `image[other-drive].size	(2, 2)`.

**The make-paths-relative command names it, refuses it, and reports itself finished** (`Windows/5.2-10-make-relative.txt`):

```
CALL bpy.ops.file.make_paths_relative()
Warning: Path 'X:\assets\tex.png' cannot be made relative for Image 'other-drive'
Warning: Total files 30 | Changed 6 | Failed 1
returned {'FINISHED'}
```

It counts the failure and says which place failed and which data-block it belonged to, and it still returns the same answer it returns when everything worked. **A caller that reads only the answer will not know one place was left behind.** The place itself is untouched and the save that follows writes it back as it was.

**Saving the whole file onto the other drive turns every relative place into a full path naming the old drive, and says so nowhere** (`Windows/5.2-11-saveas-other-drive.txt`):

```
CALL bpy.ops.wm.save_as_mainfile(filepath='X:\\elsewhere\\onotherdrive.blend')  [relative_remap not passed]
returned {'FINISHED'}

image[rel-plain].filepath	'C:\\q20\\work\\5.2\\proj\\assets\\tex.png'
image[nonlatin].filepath	'C:\\q20\\work\\5.2\\proj\\Тестовая папка\\текстура_日本語.png'
image[missing].filepath	'C:\\q20\\work\\5.2\\proj\\nowhere\\missing.png'
library[lib.blend].filepath	'C:\\q20\\work\\5.2\\proj\\lib.blend'
```

where the same call into a different folder on the *same* drive keeps them relative. The one place already on the new drive, `X:\assets\tex.png`, is the only one left as it was. No warning was printed.

#### What came back — letter case in a drive letter

**A plain save does not change it.** `c:\q20\work\5.2\proj\assets\tex.png` was stored, read back and written again with its lower-case letter, and the upper-case one likewise. **Windows finds the file either way** — both print `size	(2, 2)` — which is the same as the Mac and unlike Linux, where the wrong letter case in a file name stops the picture loading.

**Making paths absolute does not change it either** (`Windows/5.2-09-make-absolute.txt`):

```
image[abs-lower-drive].filepath	'c:\\q20\\work\\5.2\\proj\\assets\\tex.png'
```

**Making paths relative removes the drive letter and with it the question.** The upper-case and the lower-case place collapse to the same text:

```
image[abs-lower-drive].filepath	'//assets\\tex.png'
image[abs-upper-drive].filepath	'//assets\\tex.png'
```

**A drive letter with no root behind it is mangled.** A place written as a drive letter followed straight by a name is treated by the make-paths-relative command as an ordinary relative name and given steps back up in front of it:

```
image[drive-relative].filepath	'//..\\..\\..\\..\\C:q20\\work\\5.2\\proj\\assets\\tex.png'
```

The result cannot name any file on any machine. It is recorded as what happened and is not called a fault here, because whether Blender means to support that spelling at all is not something this test can settle. The place did not load its picture before the command either — it prints `size	(0, 0)` — so nothing that was working stopped working. The make-paths-absolute command left the same place untouched.

#### What came back — saving somewhere else on the same drive

**The same shape as the Mac and Linux, written the Windows way.** With nothing passed for the remap setting, every relative place a file is read from is rewritten, lengthened to reach back to where the assets are, and spelled with backslashes (`Windows/5.2-06-saveas-default.txt`):

```
CALL bpy.ops.wm.save_as_mainfile(filepath='C:\\q20\\elsewhere\\5.2\\moved_default.blend')  [relative_remap not passed]
returned {'FINISHED'}

image[rel-plain].filepath	'//..\\..\\work\\5.2\\proj\\assets\\tex.png'
image[rel-dotdot].filepath	'//..\\..\\work\\5.2\\proj\\assets\\tex.png'
image[rel-doubleslash].filepath	'//..\\..\\work\\5.2\\proj\\assets\\tex.png'
image[case-caps].filepath	'//..\\..\\work\\5.2\\proj\\assets\\TEX.PNG'
image[trailing-space].filepath	'//..\\..\\work\\5.2\\proj\\assets\\tex.png '
library[lib.blend].filepath	'//..\\..\\work\\5.2\\proj\\lib.blend'
```

The doubled separator is collapsed, the step back up resolved, the letter case kept, the trailing space kept, a place pointing at nothing rewritten just the same, and the full paths not touched. **With the setting turned off nothing changed at all**, and **saving into the same folder under a new name changed nothing either** — every recorded place came back identical in both (`Windows/5.2-07-saveas-remap-off.txt`, `Windows/5.2-08-saveas-same-folder.txt`). So the boundary is a different folder, exactly as it is on the Mac.

#### What came back — carrying the file between Windows and the Mac, both ways

**A Windows-saved file, opened on the Mac and saved with nothing touched, has every backslash turned into a forward slash.** The path text present in one file and not the other, after the Mac's plain save (`Crossing/windows-file-on-mac-5.2.txt`):

```
- //..\outer.png
- //assets\..\assets\\TEX.PNG 
- //assets\..\assets\tex.png
- //assets\TEX.PNG
- //assets\\tex.png
- //assets\beep.wav
- //assets\nosuch.vdb
- //assets\test.ies
- //assets\test.osl
- //assets\testfont.ttf
- //assets\tex.png
- //assets\tex.png 
- //assets\two words.png
- //nowhere\missing.png
- //out\{blend_name}_####.png
- //Тестовая папка\текстура_日本語.png
- C:\q20\work\5.2\proj\assets\tex.png
- C:q20\work\5.2\proj\assets\tex.png
- X:\assets\tex.png
- c:\q20\work\5.2\proj\assets\tex.png
+ //../outer.png
+ //assets/../assets//TEX.PNG 
+ //assets/../assets/tex.png
+ //assets//tex.png
+ //assets/TEX.PNG
+ //assets/beep.wav
+ //assets/nosuch.vdb
+ //assets/test.ies
+ //assets/test.osl
+ //assets/testfont.ttf
+ //assets/tex.png
+ //assets/tex.png 
+ //assets/two words.png
+ //nowhere/missing.png
+ //out/{blend_name}_####.png
+ //Тестовая папка/текстура_日本語.png
+ C:/q20/work/5.2/proj/assets/tex.png
+ C:q20/work/5.2/proj/assets/tex.png
+ X:/assets/tex.png
+ c:/q20/work/5.2/proj/assets/tex.png
```

Two further lines in that comparison are left out here because they are not recorded places from the test file — they are the temporary folders the two copies sat in, one on each machine.

**The full Windows paths are re-spelled too.** The Mac has no drive letters and can resolve neither `C:\…` nor `X:\…`, and it rewrites both anyway.

**And back the other way, the same thing in reverse.** The same file carried back and saved once on Windows with nothing touched turns every forward slash into a backslash again, including inside the drive-letter paths (`Crossing/crossing-mac-file-on-windows-5.2.txt`).

**The file is the same size every time and never the same bytes:**

```
--- size on disk ---
      687890  C:\q20\backfrommac\5.2\arrived_from_mac.blend
      687890  C:\q20\backfrommac\5.2\proj\main.blend

--- byte for byte ---
C:\q20\backfrommac\5.2\arrived_from_mac.blend C:\q20\backfrommac\5.2\proj\main.blend differ: char 69610, line 7
```

**So a file's recorded text is rewritten wholesale by the crossing between Windows and the Mac, in both directions, by a save that touched nothing.** The five places that keep their forward slashes on Windows keep them on the Mac too, so they are the only text that survives a round trip unchanged.

**One thing about moving the files, recorded because it went wrong.** The first attempt bundled the project folder with Windows' own archiving command, and it mangled the non-Latin folder name so the bundle could not be unpacked on the Mac at all — `Illegal byte sequence`. The bundle was abandoned and the files copied one by one, which carried `Тестовая папка` and `текстура_日本語.png` through intact.

#### What these Windows runs do not cover

* **Any desktop edition of Windows.** Every result here is Windows Server 2025 Standard, build 26100.32995. Both of the desktop-named images tried on 2026-08-30 booted Server editions — Server 2025 from one, Server 2016 from the other — and that is established from the running systems rather than from the labels. Nothing here shows whether a desktop edition behaves the same.
* **Why five places keep their forward slashes and the render output path does not.** The five are named above and behave the same way in every version tested, but nothing here says what distinguishes them from the render output path, which is also a place written to.
* **What a drive letter with no root is meant to do.** The make-paths-relative command makes text from it that names no file. Whether that spelling is supported at all is not settled here.
* **A place on a network share**, written the way Windows writes one. The second drive here is a drive letter mapped onto a folder, which answers the cross-drive question and not the share question.
* **A cache file, a movie clip and a movie strip**, for the same reason they said nothing on the Mac: each needs a real file of its kind to import, so none could be built into the starting file.

Every line this machine printed is in `Audits/Blender Research/Dependencies/Hand tests — Windows — raw output/`, and the list that ran it is `Audits/Blender Research/Dependencies/Hand tests — Windows.md`.

### Outside the interface

LABEL: documented in part, inferred in part.

**Documented.** The save itself is the same two operators when Blender is driven by a script rather than by a person: `bpy.ops.wm.save_mainfile` and `bpy.ops.wm.save_as_mainfile`, with the defaults quoted above — `relative_remap=False` for save, `relative_remap=True` for save-as. One thing worth naming for anyone driving Blender from outside: a script that calls `save_as_mainfile` and does not pass `relative_remap` gets the remapping behaviour switched on, because that is the documented default.

**Inferred.** That the option's stated condition — "when saving to a different directory" — governs a script the same way it governs a click. No page says so. The step is that the operator documentation is what a script reads, so its words are taken to describe what a script gets.

The command-line arguments page (`https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`) lists no argument that saves a blend-file — I searched it for "save", "relative" and "path", and the save-related hits are about rendering output and about saving GPU shader source, not about writing a blend-file. So from outside, saving happens through Python, not through a switch.

Beyond that, the sources do not say whether a save performed with no window open behaves any differently from one performed by clicking. Nothing I found distinguishes the two.

**Settled by hand test, 2026-08-30.** Every run above was driven from outside Blender's own window, with `--background`, on the Creator's Mac and on a rented Linux machine. Saving from outside behaves as the operator settings say: `bpy.ops.wm.save_mainfile()` with nothing passed does no remapping, and `bpy.ops.wm.save_as_mainfile(filepath=…)` with nothing passed does remap when the folder changes. Two things a driven run does that a person clicking does not. Giving `-o` after the file name replaces the render output place in memory, and a following plain save writes it into the file. And overwriting a file written by a newer Blender goes through silently from outside the window — it printed a warning and finished, with no confirmation to answer. See the observed section above. **Added by the third run, 2026-08-30, on the Creator's Mac.** A script reading the recorded places from outside got the same characters whichever of the two routes it used, `bpy.utils.blend_paths` or `bpy.data.file_path_map`: both reported the two backslash entries with their backslashes in the process that had just built the file, and both reported them with forward slashes in a fresh process that had opened the file from disk. On that run the reload changed the spelling and the route did not. **An earlier run of the same test recorded the two routes disagreeing there, and it has not been overwritten** — all three runs are set out in the observed section above. **Added by the Windows runs, 2026-08-30.** Those were driven from outside Blender's own window as well, with `--background` on a rented machine reporting itself as Microsoft Windows Server 2025 Standard, build 26100.32995, and the same settings behaved the same way: `bpy.ops.wm.save_mainfile()` with nothing passed did no remapping, and `bpy.ops.wm.save_as_mainfile(filepath=…)` with nothing passed remapped when the folder changed. **One thing a driven run cannot see from the answer alone**: asked to make paths relative on a file holding a place on a second drive letter, the command returned `{'FINISHED'}` having refused that place, and only the printed warning named it. On that machine both routes for listing every recorded place reported the same characters as each other, in the build run and after a reload.

### Operating system

LABEL: not in the sources.

No page I read attaches an operating system to what a save writes for a recorded location, and I found no OS-specific note on the Opening & Saving page, the External Data section, or the save operators.

Two nearby facts, offered only as facts and not as an answer: the API carries `bpy.path.native_pathsep`, "Replace the path separator with the system's native os.sep", and `bpy.path.resolve_ncase`, "Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path" (`https://docs.blender.org/api/5.2/bpy.path.html`, read 2026-07-30). Those exist because separators and letter case differ between systems. Neither says anything about saving a blend-file, and I am not treating them as evidence about it.

**Settled by hand test, 2026-08-30.** The same runs were carried out on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and on a rented Linux machine (Ubuntu 22.04.4 LTS), on Blenders with identical build hashes. **What a save records was the same on both**, including the one re-spelling: a place written with a backslash comes back with a forward slash on macOS and on Linux alike, and the plain save keeps the forward-slash form. A file built on the Mac and carried to the Linux machine behaved the same way there. **One thing did differ, and it is about finding a file rather than recording its place:** a place written `//assets/TEX.PNG` against a file named `tex.png` loads its picture on macOS and does not on Linux — the recorded text is the same on both, but the file system's answer is not. **Windows is answered too, on 2026-08-30, and its answer is not the same one.** On a rented machine reporting itself as **Microsoft Windows Server 2025 Standard, build 26100.32995**, the re-spelling runs the other way: every forward slash in a recorded place becomes a backslash when the file is opened, and a plain save with nothing touched writes that into the file. Five places keep their forward slashes there — the compositor's output folder, the cloth cache's folder, the geometry-nodes bake folder, the bake picture's place and the sequencer's proxy folder — and the render output path does not. So a file carried between Windows and the Mac has its recorded text rewritten wholesale by a save that touched nothing — tested in both directions, in 4.5 and in 5.2. The crossing between Windows and Linux was not run. **That is Windows Server 2025 and no desktop edition of Windows**; the account this project rents Windows from gave a Server edition from both of its desktop-named images. See the observed section above. **The third run of the two-routes test, 2026-08-30, was the Creator's Mac alone** — the rented machine the first run used no longer exists — so what it found settles macOS and leaves every other operating system open.

### Where I looked

Searches, all across the whole of the 5.2 manual and the whole of the 5.2 API documentation unless stated:

- manual, `remap relative` — one file only, `files/blend/open_save.txt`.
- manual, `remap` — 30 hits scanned; every one outside `open_save.txt` is a different sense of the word (colour remapping, time remapping, Remap Users in the Outliner, node value remapping). Ruled out.
- manual, `relative path` — 23 files; opened `files/blend/open_save.txt`, `interface/window_system/topbar.txt`, `editors/preferences/save_load.txt`, `editors/preferences/file_paths.txt`, `files/linked_libraries/link_append.txt`, `files/blend/index.txt`. The import/export pages (FBX, OBJ, USD, Alembic) were ruled out: their path options govern what a *written export file* records, not what a `.blend` records.
- manual, `absolute path` — 10 hits, all read or ruled out as above.
- manual, `make paths` / `Make Absolute` / `Make Relative` — only `interface/window_system/topbar.txt`.
- manual, every line containing "path" that also contains "sav" — 20 hits, all read.
- manual, `backslash`, `path separator`, `normalise/normalize` — no hit about writing a blend-file. The only path-related hit is `advanced/extensions/python_wheels.txt`, about listing wheels, ruled out.
- manual, `as-is`, `unchanged`, `verbatim`, `left intact`, `preserved` — the path-relevant hits are `files/file_paths.txt` (quoted above) and `files/media/image_formats.txt` line 407, which turned out to be the Copy option of the image Save As dialog and is about a deliberate repointing, not a save.
- manual, `when the file is saved`, `when saving the blend`, `on the next save` — led to the auto-pack quotes and the modified-images preference, both used above.
- manual pages opened in full: `files/blend/open_save.txt`, `files/blend/packed_data.txt`, `files/blend/index.txt`, `files/file_paths.txt`, `interface/window_system/topbar.txt` (External Data through Defaults), `editors/preferences/save_load.txt`, `editors/preferences/file_paths.txt` (top), `files/linked_libraries/link_append.txt` (path parts), `files/media/image_formats.txt` (Saving Modified Images), `editors/image/editing.txt` (Save / Save As), `advanced/command_line/arguments.txt` (searched).
- manual picture opened: `_images/files_blend_open-save_file-browser-save.png`.
- API, `relative_remap` — one file, `bpy.ops.wm.txt`, four lines, all quoted above.
- API, `make_paths` — `bpy.ops.file.make_paths_absolute()` and `bpy.ops.file.make_paths_relative()` exist in `bpy.ops.file.txt`; both are commands a person or script runs, so they do not bear on an untouched save.
- API, `relative path` / `blend relative` — 40+ type pages; opened `bpy.types.Library.txt`, `bpy.types.Image.txt`, `bpy.types.RenderSettings.txt`, `bpy.path.txt`. The rest are the same one-line "blend relative // prefix supported" note on other path properties (fonts, sounds, movie clips, caches, bakes, File Output node, and so on) and add nothing about saving.
- API, `file_path_foreach` — `bpy.types.BlendData.txt` and `bpy.types.BlendDataPathMeta.txt`. These describe walking and rewriting paths from a script, and flag paths that "can not be edited" (cache paths, expanded UDIM/sequence paths). Ruled out: they say nothing about what a save writes.
- API, `backslash` / `path separator` — nothing relevant; the hits are an icon name and a Wavefront-style option.
- Release notes 5.2, searched every file for `path`, `relative`, `when saving`, `on save`, `re-save`. Nothing about how a save writes recorded locations. The path-related entries are the new path iterator options in `python_api.md`, the Cycles texture cache preference in `cycles.md`, and bug-fix lines. The nearest relevant note is in `user_interface.md`: "When saving a file with unsaved images, a new dialog box now asks if the user wants to also save the modified images." (`https://developer.blender.org/docs/release_notes/5.2/user_interface/`).
- Cycles 5.2 files: searched the whole folder for `DIR_PATH` and `FILE_PATH`, and `properties.py` for `cache`. Results reported above.

---

## What a running Blender still could not settle

The list that stood here was run in full on 2026-08-30 — on the Creator's Mac, on a rented Linux machine, and on a rented Windows machine — and the results are in the observed sections above. What is left is only what those three machines could not reach:

1. **Any desktop edition of Windows.** All three Windows questions — which way the separator is re-spelled, what a plain save records for a relative place across two drives, and whether letter case in a drive letter survives a save — were answered on 2026-08-30 on a rented machine reporting itself as **Microsoft Windows Server 2025 Standard**, build 26100.32995, and the answers are in the observed section above. What is not settled is whether a desktop edition of Windows answers the same. Both of the desktop-named images tried that day booted Server editions — Server 2025 Standard from one, and **Microsoft Windows Server 2016 Standard**, build 14393.9234, from the other — read off the running systems rather than off the labels.
2. **A cache file, a movie clip and a movie strip.** Each needs a real file of its kind to import, so none could be built into the test file, and nothing has been observed about them.
3. **Why 4.5 writes an auto-save file where 5.0, 5.1 and 5.2 write only a quit file.** Three runs gave the same split. Whether that is a change in Blender or a difference in when a version decides a file has been edited is not settled.

The one list these came from is `Audits/Blender Research/Dependencies/Hand tests — Open Threads questions.md`.

---

2026-08-29 — Claude Opus 5 (Claude Code) — the blind research pass that wrote this document: Dependencies, Blender 5.2. The observed sections above were added by later sittings and carry their own dates.

*(Corrected 2026-08-30 — Claude Opus 5. The citation check of 2026-08-30 flagged that this document dates its runs 2026-08-30 while this line reads 2026-08-29, and said the two cannot both be right. They can, and both are: this line signs the blind research pass, and the runs belong to a later sitting. Which date belongs to which was established from the raw output's own timestamps, which print `2026-08-30` inside the run files under `Audits/Blender Research/Dependencies/Hand tests — Open Threads questions — raw output/Mac/`, and from this document's own signatures file, which puts the blind pass on 2026-08-29 and the hand tests on 2026-08-30. No date was wrong. What was wrong is that this line did not say what it covers, which is what made the two read as a clash, so it now says it.)*

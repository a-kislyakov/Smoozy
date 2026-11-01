# Dependencies — Open Threads questions, second pass (Blender 5.1)

**Sources consulted, and when.** The Blender 5.1 manual, the Blender 5.1 Python API documentation, the Blender 5.1 release notes, and the Cycles 5.1 add-on files, all as supplied in `Blender Sources/`. Snapshot date, and my date read for everything below: **2026-07-30**.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

Scope: question 20 only, from the Dependencies sheet.

---

## 20. When a `.blend` file is opened and saved again, and none of the places it records for outside files is touched while it is open, what does the saved file record for each of those places?

### Reading of the question

I answer the reading I think is meant: the file is opened, nothing that holds a path is edited, and the file is then saved back over itself in the same place (File ‣ Save, `wm.save_mainfile`). The question is ambiguous on one point, and I say so plainly: "saved again" could also mean File ‣ Save As to a *different* folder. That second reading has a different answer, and I give it separately below, because the sources distinguish the two cases explicitly.

### Answer

**LABEL: inferred** (built from several documented pieces; the step I took is written out below). Two parts of the answer are **documented** outright and are marked as such.

**The main claim.** Each untouched entry is written back in the same form it was read in — a path recorded with the `//` prefix is still recorded with the `//` prefix, and a full path from the system root is still recorded as a full path. Nothing about a plain save converts one form into the other.

No page in the three documentation sets says this outright. I reached it from four documented facts, all of which say the same thing from a different side: **every** conversion between the two forms in Blender is something a person or a script has to ask for.

1. The remapping that happens at save time is an *option*, and it is described as applying to a change of location:

   > "This option remaps Relative Paths
   > (such as linked libraries and images) when saving a file in a new location."

   Manual, *Opening & Saving*, https://docs.blender.org/manual/en/5.1/files/blend/open_save.html — read 2026-07-30.

2. That option is off by default for a plain save, and on by default only for Save As, and both spell out that it concerns a different directory:

   > "bpy.ops.wm.save_mainfile(*, filepath='', … compress=False, relative_remap=False, exit=False, incremental=False)"

   > "bpy.ops.wm.save_as_mainfile(*, filepath='', … compress=False, relative_remap=True, copy=False)"

   > "relative_remap (bool) – Remap Relative, Remap relative paths when saving to a different directory (optional)"

   Python API, *bpy.ops.wm*, https://docs.blender.org/api/5.1/bpy.ops.wm.html — read 2026-07-30. (The last line is the wording used for both operators.)

3. Converting the recorded form wholesale is a separate command a person has to run, not a side effect of saving:

   > "Make Paths Relative
   >
   > Make all paths to external files relative to the current blend-file.
   >
   > Make Paths Absolute
   >
   > Make all paths to external files absolute (= full path from the system's root)."

   Manual, *Topbar* (File ‣ External Data), https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — read 2026-07-30.

   The same page shows that even Blender's own repair leaves the form alone until you ask:

   > "Those recoveries will be done as absolute paths,
   > so if you want to have relative paths you will need to select Make Paths Relative."

   Same page, same address, read 2026-07-30. (Python names for the two commands: `bpy.ops.file.make_paths_relative()` — "Make all paths to external files relative to current .blend"; `bpy.ops.file.make_paths_absolute()` — "Make all paths to external files absolute". Python API, *bpy.ops.file*, https://docs.blender.org/api/5.1/bpy.ops.file.html — read 2026-07-30.)

4. The preference that decides the relative-or-absolute default is explicitly stated not to touch a path that already exists:

   > "Default To – Relative Paths
   >
   > Default value for Relative Paths when loading external files
   > such as images, sounds, and linked libraries. It will be ignored if a path is already set."

   Manual, *Save & Load Preferences*, https://docs.blender.org/manual/en/5.1/editors/preferences/save_load.html — read 2026-07-30.

   > "use_relative_paths
   >
   > Default relative path option for the file selector, when no path is defined yet (default True)"

   Python API, *bpy.types.PreferencesFilePaths*, https://docs.blender.org/api/5.1/bpy.types.PreferencesFilePaths.html — read 2026-07-30.

   And Blender's own writing routine for a partial blend-file names "no path manipulation" as its default behaviour, which is how the documentation talks about leaving paths alone:

   > "NONE No path manipulation (default)."

   Python API, *bpy.types.BlendDataLibraries* (`write(filepath, datablocks, *, path_remap='NONE', …)`), https://docs.blender.org/api/5.1/bpy.types.BlendDataLibraries.html — read 2026-07-30.

   **The step I took:** none of these four is a statement about what a plain save writes. Each says that a particular *change* to a recorded path requires either a new location or an explicit command. A save over the same file is neither. From "no described mechanism converts the form, and the one save-time mechanism that could is off and is scoped to a different directory" I conclude "the form is written back as it was". A reader has to supply that last move, so the label is inferred, not documented.

**What the recorded `//` form means, for completeness.** Documented:

> "Property is a path which supports the "//" prefix, signifying the location as relative to the ".blend" file's directory"

Python API, *bpy.types.Property* (`is_path_supports_blend_relative`), https://docs.blender.org/api/5.1/bpy.types.Property.html — read 2026-07-30.

> "The // at the start of each path in Blender means the directory of the currently opened blend-file, used to
> reference relative paths."

Manual, *Configuring Blender*, https://docs.blender.org/manual/en/5.1/getting_started/configuration/introduction.html — read 2026-07-30.

**Documented part 1 — a path that carries a template keeps its template text.** For the two path settings that support templates (the render output path and the compositor's File Output node), the manual states outright that the text in the field is not replaced by the substituted value:

> "This substitution happens internally at time of use (e.g. while rendering), so
> the template syntax will remain as-is in the path field."

Manual, *File Paths*, https://docs.blender.org/manual/en/5.1/files/file_paths.html — read 2026-07-30.

The same "resolved at use, not stored" idea is stated for the `//` prefix on the render output path:

> "This setting expands Relative Paths
> where a // prefix represents the directory of the current blend-file."

Manual, *Output Properties ‣ Output*, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

(Strictly, both sentences are about the field, not about the save. That the field's content is what the save writes is the same inferential step as above.)

**Documented part 2 — an entry can vanish entirely, without anyone touching it.** If the data-block that holds a recorded location has no users, the save drops it, and its recorded location goes with it:

> "If a data-block has no real users, it'll normally be cleaned up
> (deleted) when saving the blend-file. To prevent this, you can give it a fake user;
> that way, it's guaranteed to "survive.""

Manual, *Data-Block Menu*, https://docs.blender.org/manual/en/5.1/interface/controls/templates/data_block.html — read 2026-07-30.

> "Unused data-blocks are automatically deleted when saving and reloading the file."

Manual, *Outliner ‣ Interface*, https://docs.blender.org/manual/en/5.1/editors/outliner/interface.html — read 2026-07-30.

For linked (library) data this cannot be prevented the usual way:

> "Since it is not editable, linked data cannot be protected with the Fake User option. Adding a custom property pointing to an
> otherwise unused linked data-block (e.g. a Text block) is a good way to keep it referenced across saves and
> reloads."

Manual, *Link & Append*, https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html — read 2026-07-30.

So "each of those places" is not guaranteed to be a complete list of what was there when the file was opened: unused ones are not carried over.

**Documented part 3 — a save can change what the file holds for a place, even when nothing was touched, if packing was already switched on.** Packing is deferred to the save:

> "Mark all eligible external resource files used by the blend-file as packed.
> Actual packing will happen on the next save of the blend-file."

and

> "When enabled, this option will ensure that all eligible external resource files, existing or added later,
> are systematically marked as packed.
> As with Pack Resources, the blend-file must be saved to the drive for this to have an effect."

Manual, *Packed Data*, https://docs.blender.org/manual/en/5.1/files/blend/packed_data.html — read 2026-07-30.

The recorded location itself appears to survive packing, because unpacking later offers that same location back:

> "Use <original file path> (differs)|(identical)
>
> If the original file path still exists, mark it as unpacked."

Same page, same address, read 2026-07-30. That the path is *stored* rather than reconstructed is my inference from the fact that the command can offer it after the file has been packed; the manual does not say where it is kept.

**The other reading — Save As to a different folder.** There the manual and API agree that remapping is what happens by default: `relative_remap` is `True` for `wm.save_as_mainfile`, and the manual describes the option as remapping relative paths "when saving a file in a new location" (both quoted above). The manual's screenshot of the save dialog confirms the option is present and shown ticked: the "Save As Blender File" panel lists three checkboxes — Compress (ticked), Remap Relative (ticked), Save Copy (unticked). Image `_images/files_blend_open-save_file-browser-save.png`, shown on the *Opening & Saving* page, https://docs.blender.org/manual/en/5.1/files/blend/open_save.html — **documented** (I looked at the picture on 2026-07-30). I do not read anything into the ticked state beyond "that is what the picture shows"; the API defaults quoted above are the authority on defaults. Note that the remap is described as applying to *relative* paths only — the sources say nothing about absolute paths in that case.

**Related documented fact about the load side**, which bears on what is in memory at save time: the file's own recorded output path wins over one set before the file is loaded.

> "blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set."

Manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

**Observed, from the Cycles files supplied.** Cycles resolves a stored path to an absolute one only into a local variable when it needs the file, and does not write the absolute form back into the property:

> `    script_path = bpy.path.abspath(filepath, library=library)`

`cycles_5.1/osl.py`, line 199, inside `def update_external_script(report, filepath, library):` (line 192).

> `    osl_path = bpy.path.abspath(script.filepath, library=script.library)`

`cycles_5.1/osl.py`, line 239, inside `def update_internal_script(report, script):` (line 229). *(Citation check, 2026-08-30 — Claude Opus 5: the function's own line was given as 228; it is 229. The quoted line at 239 is exactly right, as are the other two osl.py citations and the four operators.py ones.)*

> `        ok, oso_path, oso_file_remove = update_external_script(report, cam.custom_filepath, cam.library)`

`cycles_5.1/osl.py`, line 336, inside `def update_custom_camera_shader(cam, report):` (line 326). Read 2026-07-30.

Searching those same files, the only assignments to a path property anywhere in the Cycles Python files are in the denoising operator, and they set the value back to what it was:

> `                scene.render.filepath = original_filepath`

`cycles_5.1/operators.py`, lines 85 and 94 (the property is set to `in_filepath` / `out_filepath` at lines 80 and 90 and restored at 85 and 94). Read 2026-07-30.

**Observed → inferred boundary:** what these lines show is that the Cycles add-on does not rewrite a stored path into its absolute form when it uses it. Concluding from this that the file *written to disk* therefore keeps the original form is a step beyond the file, and that step is inferred; the writer of the blend-file is not among the supplied Cycles files.

### Not covered

- **No source states the main claim outright.** It is an inference from the absence of any mechanism, not a documented rule. If Blender's file writer normalises path text in some way (separators, letter case, a trailing slash, a redundant `./`), nothing in the three documentation sets or the supplied Cycles files says so, in either direction.
- **The same-directory Save As case.** The option is worded "a new location" / "a different directory". Save As onto the same folder is not addressed.
- **Absolute paths under Save As.** The remap option is described only as remapping *relative* paths; what happens to absolute ones on Save As is not stated.
- **A place whose file is missing.** Whether an entry pointing at something that is not there is preserved unchanged, or cleared, on save is not stated anywhere I could find. (That is close to question 12's territory and I did not chase it further.)
- **Files made by an older Blender.** Whether opening an older file runs any conversion that rewrites recorded locations before the save is not stated. The compatibility page describes conversion in general terms only and says nothing about paths.
- **Paths that live inside a linked library** rather than in the file being saved: the sources do not say what the saving file records about them beyond the library's own path.
- **The `//` in a linked library's own data.** `bpy.path.abspath(…, library=…)` documents that a library's path replaces the starting point when resolving, but nothing says how that interacts with saving.
- **Byte-level format.** Nothing about how the string is encoded in the file.

---

## 20 — observed. What a running Blender does

**Label: observed.** Everything under this heading was got by running Blender 5.1 and reading what it printed and what it wrote to disk. It is not a reading of any documentation, and it does not replace the inferred answer above; where the two disagree, say so plainly rather than quietly preferring one.

### On what kind of computer, and when

On 2026-08-30 these were run twice over: once on the Creator's own computer — a MacBook Pro with an Apple M1 Pro chip running macOS 26.6.2, with Apple's own graphics — and once on a rented Linux machine — Ubuntu 22.04.4 LTS with one NVIDIA RTX 3060, twelve cores and 31 GB of memory, in Poland, driven over SSH, rented and destroyed the same day. The build hash of every Blender used on the rented machine is the same as the one on the Mac, so where the two disagree the difference is the operating system and not the build. Two results are reported wherever both machines ran a test, and they are never folded into one sentence.

### What was done, and to what

A `.blend` file was built by script in Blender 5.1, so that the exact text of every place it records for an outside file was known rather than typed. Beside it sat `assets/tex.png`, `assets/two words.png`, `assets/beep.wav`, `assets/testfont.ttf`, `assets/test.osl`, `assets/test.ies`, a folder named `Тестовая папка` holding `текстура_日本語.png`, a second blend-file `lib.blend`, and one folder up an `outer.png`.

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

The two files on disk were never the same size and never the same file: 643,608 bytes before the plain save and 640,356 after, with different checksums. **So comparing whole files says nothing; only the path text can be compared.**

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

So which way a check asks for a recorded place decides what string it gets back. Blender 5.1 has `bpy.utils.blend_paths` and `bpy.data.file_path_map`, and `bpy.data.file_path_foreach` is there:

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
VERSION 5.1.2 HASH ec6e62d40fa9
PLATFORM darwin
```

The starting file was built fresh, and both questions were asked twice over: once **inside the process that had just built the file and had never reloaded it**, and once in a **fresh process that opened the file from disk**. The two commands in full, after the application's own path: `--background --factory-startup --python build_main.py -- <project folder> <scripts folder>`, then `--background --factory-startup <the file> --python read_only.py -- <scripts folder>`. The two routes were compared character for character by script rather than read by eye.

*What came back inside the process that built the file, before any reload:*

```
5.1  rel-backslash file_path_map -> ['//assets\\tex.png']
5.1  raw-setter    file_path_map -> ['//assets\\..\\assets//TEX.PNG ']
5.1  blend_paths entries holding a backslash -> ['//assets\\..\\assets//TEX.PNG ', '//assets\\tex.png']
```

*And in the fresh process that had opened the file from disk:*

```
5.1  rel-backslash file_path_map -> ['//assets/tex.png']
5.1  raw-setter    file_path_map -> ['//assets/../assets//TEX.PNG ']
5.1  blend_paths entries holding a backslash -> []
```

So on this machine the two routes reported those two entries the same way as each other, and what changed the spelling was the reload rather than the route.

*One thing that is not the disagreement described above, and is recorded rather than folded into it.* The two routes do not return the same **collection** of strings. Compared as multisets in the build process:

```
5.1 : blend_paths 24 strings, file_path_map 27 strings
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

**The setting for saving modified pictures is not in this version.** Asking for it throws:

```
save_modified_images | ABSENT IN THIS VERSION
AttributeError: 'PreferencesFilePaths' object has no attribute 'save_modified_images'
```

### What these runs do not cover

* **Windows.** These particular runs were the Creator's Mac and the rented Linux machine, and neither speaks for Windows. The Windows half of the question was run separately on a rented machine reporting itself as Microsoft Windows Server 2025 Standard, and it has an observed section of its own below, after this list; what that section leaves open is listed there.
* **Three kinds of recorded place** could not be built into the test file because each needs a real file of its kind to import: a cache file, a movie clip and a movie strip. Nothing here says how those behave.
* **What the file physically holds** beyond its readable text. The comparison above is of path text pulled out of the file, not of its structure.

The full list these came from, and every machine's own words in full, are at `Audits/Blender Research/Dependencies/Hand tests — Open Threads questions.md` and in the folder beside it.


### 20 — observed on Windows. What a running Blender does there

**Label: observed.** Everything under this heading was got by running Blender 5.1.2 on a rented Windows machine on 2026-08-30, and reading what it printed and what it wrote to disk. It is not a reading of any documentation. It answers the Windows half of this question, which every run before it left open, and it replaces nothing above: where a Windows result differs from the Mac's or the rented Linux machine's, both stand and the difference is named.

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

**The Blender was the official Windows x64 build from Blender's own download server, and its hash is the same as the Creator's Mac's and the rented Linux machine's.** Asked of the running program itself, not read off a download page (`Windows/5.1-00-machine.txt`; the same four hashes are in `Windows/01-builds.txt` and `Windows/00-the-machine.txt`):

```
RESOURCES C:\q20\res-5.1
VERSION 5.1.2 HASH ec6e62d40fa9
BUILD PLATFORM b'Windows'
BUILD TYPE b'Release'
BINARY C:\blender\5.1.2\blender.exe
PLATFORM win32
OS SEP '\\' ALTSEP '/'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:37:34)
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
C:\blender\5.1.2\blender.exe --background --factory-startup --python C:\q20\scripts\build_main.py -- C:\q20\work\5.1\proj C:\q20\scripts
C:\blender\5.1.2\blender.exe --background --factory-startup C:\q20\work\5.1\proj\main.blend --python C:\q20\scripts\read_only.py -- C:\q20\scripts
C:\blender\5.1.2\blender.exe --background --factory-startup C:\q20\work\5.1\proj\main.blend --python C:\q20\scripts\plain_save.py -- C:\q20\scripts
```

Six more runs followed from the same starting file, put back fresh each time: a save into another folder on the same drive with nothing passed for the remap setting, the same with the setting turned off, a save into the same folder under a new name, the make-paths-absolute command, the make-paths-relative command, and a save onto the second drive.

#### What came back — setting a place changes nothing, whichever way it is spelled

Every one of the twenty spellings came back out of the setting exactly as it went in, in memory, before anything was saved (`Windows/5.1-01-build.txt`):

```
SET	rel-plain	wanted='//assets/tex.png'	got_filepath='//assets/tex.png'	got_raw='//assets/tex.png'
SET	rel-backslash	wanted='//assets\\tex.png'	got_filepath='//assets\\tex.png'	got_raw='//assets\\tex.png'
SET	abs-forward	wanted='C:/q20/work/5.1/proj/assets/tex.png'	got_filepath='C:/q20/work/5.1/proj/assets/tex.png'	got_raw='C:/q20/work/5.1/proj/assets/tex.png'
SET	abs-lower-drive	wanted='c:\\q20\\work\\5.1\\proj\\assets\\tex.png'	got_filepath='c:\\q20\\work\\5.1\\proj\\assets\\tex.png'	got_raw='c:\\q20\\work\\5.1\\proj\\assets\\tex.png'
SET	drive-relative	wanted='C:q20\\work\\5.1\\proj\\assets\\tex.png'	got_filepath='C:q20\\work\\5.1\\proj\\assets\\tex.png'	got_raw='C:q20\\work\\5.1\\proj\\assets\\tex.png'
SET	other-drive	wanted='X:\\assets\\tex.png'	got_filepath='X:\\assets\\tex.png'	got_raw='X:\\assets\\tex.png'
```

#### What came back — opening the file re-spells the separator, and on Windows it runs the other way

**On the Mac and on the rented Linux machine a backslash becomes a forward slash. On Windows Server 2025 every forward slash becomes a backslash.** These are the same places, read on the very next open with nothing touched in between (`Windows/5.1-02-read-before.txt`):

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
image[abs-forward].filepath	'C:\\q20\\work\\5.1\\proj\\assets\\tex.png'
image[raw-setter].filepath	'//assets\\..\\assets\\\\TEX.PNG '
scene.render.filepath	'//out\\{blend_name}_####.png'
```

Against what the same places held in memory when the file was written, which is quoted in full in the raw output: `'//assets/tex.png'`, `'//assets/../assets/tex.png'`, `'//assets//tex.png'`, `'//assets/TEX.PNG'`, `'//assets/tex.png '`, `'//assets/two words.png'`, `'//Тестовая папка/текстура_日本語.png'`, `'//../outer.png'`, `'//nowhere/missing.png'`, `'C:/q20/work/5.1/proj/assets/tex.png'`, `'//assets\\..\\assets//TEX.PNG '` and `'//out/{blend_name}_####.png'`.

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

The two files on disk are never the same file (`Windows/5.1-05-bytes.txt`):

```
--- size on disk ---
      684056  C:\q20\work\5.1\main_after_build.blend
      680804  C:\q20\work\5.1\main_after_plain_save.blend

--- byte for byte ---
C:\q20\work\5.1\main_after_build.blend C:\q20\work\5.1\main_after_plain_save.blend differ: char 67494, line 6

--- sha256 ---
40fd12b9519852aa041028fad09b03684abaca870a6add818a6a8896704991b3  C:\q20\work\5.1\main_after_build.blend
3b3aad2568e29800a97ab27875dcd88a1fc1b43e53cdd18885a07beb00585ca5  C:\q20\work\5.1\main_after_plain_save.blend
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
      1 C:/q20/work/5.1/proj/assets/tex.png
      1 C:\Users\Administrator\Documents
      3 C:\q20\work\5.1\proj\assets\tex.png
      1 C:q20\work\5.1\proj\assets\tex.png
      1 X:\assets\tex.png
      1 c:\q20\work\5.1\proj\assets\tex.png
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
      4 C:\q20\work\5.1\proj\assets\tex.png
      1 C:q20\work\5.1\proj\assets\tex.png
      1 X:\assets\tex.png
      1 c:\q20\work\5.1\proj\assets\tex.png
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

and after it, one — the weak reference gone with it (`Windows/5.1-04-read-after.txt`). The unused image shielded with a fake user survived, and the third, unused and unshielded, never reached the first read because the build's own save had already dropped it.

**Why the two reads look the same although the file changed.** The re-spelling is already done by the time anything can read the file at all, so the read before the plain save and the read after it agree; the change is only visible in the bytes. **A check that reads a place through Blender and compares it against the text in the file will match on a file Windows has already saved, and will miss on a file that arrived from anywhere else.**

#### What came back — the two ways of listing every recorded place agree with each other on Windows

The Mac's first run recorded these two routes disagreeing about spelling in the process that had just built the file. **On Windows Server 2025 they do not disagree, in either state.** In the build run, before any reload, both report the backslash entry with its backslash and the plain entry with its forward slash; after the file has been opened from disk, both report every entry with backslashes (`Windows/5.1-01-build.txt` and `Windows/5.1-02-read-before.txt`):

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
5.1 : blend_paths 32 strings, file_path_map 35 strings
     only file_path_map: '//assets/tex.png'
     only file_path_map: '//lib.blend'
     only file_path_map: '//lib.blend'

after the file was opened from disk
5.1 : blend_paths 30 strings, file_path_map 34 strings
     only file_path_map: '//assets\\tex.png'
     only file_path_map: '//lib.blend'
     only file_path_map: '//lib.blend'
     only file_path_map: '//lib.blend'
```

The extras are the linked library's own inner picture, which the second route reaches through the library and the first does not, and repeated mentions of the library itself. The one string that looks like a spelling difference is not one: it is `'//assets/tex.png'` before the reload and `'//assets\\tex.png'` after it, which is the same entry following the same re-spelling as everything else.

**So the disagreement the Mac's first run recorded is not a Windows disagreement**, and that is a clean negative rather than a silence: a check may read a recorded place through either route on this machine and get the same characters. **It does not make the underlying danger smaller — it moves it.** On Windows both routes agree with each other and both disagree with the file that arrived, whenever that file was written anywhere but Windows.

#### What came back — a relative place across two drives

**A plain save leaves it exactly alone.** `X:\assets\tex.png` was stored, read back and written again unchanged, and the picture loaded from it — the read prints `image[other-drive].size	(2, 2)`.

**The make-paths-relative command names it, refuses it, and reports itself finished** (`Windows/5.1-10-make-relative.txt`):

```
CALL bpy.ops.file.make_paths_relative()
Warning: Path 'X:\assets\tex.png' cannot be made relative for Image 'other-drive'
Warning: Total files 30 | Changed 6 | Failed 1
returned {'FINISHED'}
```

It counts the failure and says which place failed and which data-block it belonged to, and it still returns the same answer it returns when everything worked. **A caller that reads only the answer will not know one place was left behind.** The place itself is untouched and the save that follows writes it back as it was.

**Saving the whole file onto the other drive turns every relative place into a full path naming the old drive, and says so nowhere** (`Windows/5.1-11-saveas-other-drive.txt`):

```
CALL bpy.ops.wm.save_as_mainfile(filepath='X:\\elsewhere\\onotherdrive.blend')  [relative_remap not passed]
returned {'FINISHED'}

image[rel-plain].filepath	'C:\\q20\\work\\5.1\\proj\\assets\\tex.png'
image[nonlatin].filepath	'C:\\q20\\work\\5.1\\proj\\Тестовая папка\\текстура_日本語.png'
image[missing].filepath	'C:\\q20\\work\\5.1\\proj\\nowhere\\missing.png'
library[lib.blend].filepath	'C:\\q20\\work\\5.1\\proj\\lib.blend'
```

where the same call into a different folder on the *same* drive keeps them relative. The one place already on the new drive, `X:\assets\tex.png`, is the only one left as it was. No warning was printed.

#### What came back — letter case in a drive letter

**A plain save does not change it.** `c:\q20\work\5.1\proj\assets\tex.png` was stored, read back and written again with its lower-case letter, and the upper-case one likewise. **Windows finds the file either way** — both print `size	(2, 2)` — which is the same as the Mac and unlike Linux, where the wrong letter case in a file name stops the picture loading.

**Making paths absolute does not change it either** (`Windows/5.1-09-make-absolute.txt`):

```
image[abs-lower-drive].filepath	'c:\\q20\\work\\5.1\\proj\\assets\\tex.png'
```

**Making paths relative removes the drive letter and with it the question.** The upper-case and the lower-case place collapse to the same text:

```
image[abs-lower-drive].filepath	'//assets\\tex.png'
image[abs-upper-drive].filepath	'//assets\\tex.png'
```

**A drive letter with no root behind it is mangled.** A place written as a drive letter followed straight by a name is treated by the make-paths-relative command as an ordinary relative name and given steps back up in front of it:

```
image[drive-relative].filepath	'//..\\..\\..\\..\\C:q20\\work\\5.1\\proj\\assets\\tex.png'
```

The result cannot name any file on any machine. It is recorded as what happened and is not called a fault here, because whether Blender means to support that spelling at all is not something this test can settle. The place did not load its picture before the command either — it prints `size	(0, 0)` — so nothing that was working stopped working. The make-paths-absolute command left the same place untouched.

#### What came back — saving somewhere else on the same drive

**The same shape as the Mac and Linux, written the Windows way.** With nothing passed for the remap setting, every relative place a file is read from is rewritten, lengthened to reach back to where the assets are, and spelled with backslashes (`Windows/5.1-06-saveas-default.txt`):

```
CALL bpy.ops.wm.save_as_mainfile(filepath='C:\\q20\\elsewhere\\5.1\\moved_default.blend')  [relative_remap not passed]
returned {'FINISHED'}

image[rel-plain].filepath	'//..\\..\\work\\5.1\\proj\\assets\\tex.png'
image[rel-dotdot].filepath	'//..\\..\\work\\5.1\\proj\\assets\\tex.png'
image[rel-doubleslash].filepath	'//..\\..\\work\\5.1\\proj\\assets\\tex.png'
image[case-caps].filepath	'//..\\..\\work\\5.1\\proj\\assets\\TEX.PNG'
image[trailing-space].filepath	'//..\\..\\work\\5.1\\proj\\assets\\tex.png '
library[lib.blend].filepath	'//..\\..\\work\\5.1\\proj\\lib.blend'
```

The doubled separator is collapsed, the step back up resolved, the letter case kept, the trailing space kept, a place pointing at nothing rewritten just the same, and the full paths not touched. **With the setting turned off nothing changed at all**, and **saving into the same folder under a new name changed nothing either** — every recorded place came back identical in both (`Windows/5.1-07-saveas-remap-off.txt`, `Windows/5.1-08-saveas-same-folder.txt`). So the boundary is a different folder, exactly as it is on the Mac.

#### The crossing between Windows and the Mac was not run in this version

The crossing — a Windows-saved file carried to the Mac, opened, saved with nothing touched and compared byte for byte, and then the same in reverse — was run in 4.5 and in 5.2 only, as the two ends of the range, and both behaved the same way. **It was not run in 5.1, so this version's silence on it is an untested question rather than a difference in Blender**, and the machine that could answer it is destroyed.

#### What these Windows runs do not cover

* **Any desktop edition of Windows.** Every result here is Windows Server 2025 Standard, build 26100.32995. Both of the desktop-named images tried on 2026-08-30 booted Server editions — Server 2025 from one, Server 2016 from the other — and that is established from the running systems rather than from the labels. Nothing here shows whether a desktop edition behaves the same.
* **Why five places keep their forward slashes and the render output path does not.** The five are named above and behave the same way in every version tested, but nothing here says what distinguishes them from the render output path, which is also a place written to.
* **What a drive letter with no root is meant to do.** The make-paths-relative command makes text from it that names no file. Whether that spelling is supported at all is not settled here.
* **A place on a network share**, written the way Windows writes one. The second drive here is a drive letter mapped onto a folder, which answers the cross-drive question and not the share question.
* **A cache file, a movie clip and a movie strip**, for the same reason they said nothing on the Mac: each needs a real file of its kind to import, so none could be built into the starting file.

Every line this machine printed is in `Audits/Blender Research/Dependencies/Hand tests — Windows — raw output/`, and the list that ran it is `Audits/Blender Research/Dependencies/Hand tests — Windows.md`.

### Outside the interface

Partly answerable, and the answerable part matters. The defaults quoted above are the *operator* defaults, so they are what a script gets when it calls `bpy.ops.wm.save_mainfile()` with no arguments: `relative_remap=False`. A script that wants remapping must pass it. `bpy.ops.file.make_paths_relative()` / `make_paths_absolute()` exist as ordinary operators, so a script can convert deliberately. Nothing in the sources says any of this behaves differently when Blender is started and driven from outside its own window.

Two differences the sources do document for a driven run:

- The preference "Default To – Relative Paths" is stated to apply to the *file selector* and to be "ignored if a path is already set", so it has no bearing on a headless open-and-save of a file whose paths already exist. (Quotes above.)
- `-o` / `--render-output` changes the render output path in the session, and the manual documents the ordering trap: given before the file, "loading the blend-file overwrites the render output that was set" (quoted above). Given *after* the file, the manual says the argument sets the render path. If a run of that shape then saved the file, that entry would no longer be untouched — so the question's premise would not hold. That last sentence is **inferred**; the manual does not discuss saving after `-o`.

Beyond that: the sources do not say.

**Settled by hand test, 2026-08-30.** Every run above was driven from outside Blender's own window, with `--background`, on the Creator's Mac and on a rented Linux machine. Saving from outside behaves as the operator settings say: `bpy.ops.wm.save_mainfile()` with nothing passed does no remapping, and `bpy.ops.wm.save_as_mainfile(filepath=…)` with nothing passed does remap when the folder changes. Two things a driven run does that a person clicking does not. Giving `-o` after the file name replaces the render output place in memory, and a following plain save writes it into the file. And overwriting a file written by a newer Blender goes through silently from outside the window — it printed a warning and finished, with no confirmation to answer. See the observed section above. **Added by the third run, 2026-08-30, on the Creator's Mac.** A script reading the recorded places from outside got the same characters whichever of the two routes it used, `bpy.utils.blend_paths` or `bpy.data.file_path_map`: both reported the two backslash entries with their backslashes in the process that had just built the file, and both reported them with forward slashes in a fresh process that had opened the file from disk. On that run the reload changed the spelling and the route did not. **An earlier run of the same test recorded the two routes disagreeing there, and it has not been overwritten** — all three runs are set out in the observed section above. **Added by the Windows runs, 2026-08-30.** Those were driven from outside Blender's own window as well, with `--background` on a rented machine reporting itself as Microsoft Windows Server 2025 Standard, build 26100.32995, and the same settings behaved the same way: `bpy.ops.wm.save_mainfile()` with nothing passed did no remapping, and `bpy.ops.wm.save_as_mainfile(filepath=…)` with nothing passed remapped when the folder changed. **One thing a driven run cannot see from the answer alone**: asked to make paths relative on a file holding a place on a second drive letter, the command returned `{'FINISHED'}` having refused that place, and only the printed warning named it. On that machine both routes for listing every recorded place reported the same characters as each other, in the build run and after a reload.

### Operating system

The sources do not say. I found no statement anywhere in the 5.1 manual, API documentation or release notes about the saving of a `.blend` file treating recorded locations differently on one operating system than another. The only operating-system-specific statements about path *form* that my searches turned up are in the OBJ and FBX **exporters**, and they are about those exporters' own output, not about saving a `.blend`:

> "Uses relative paths in every case (except when on a different drive on Windows)."

Manual, *OBJ*, https://docs.blender.org/manual/en/5.1/files/import_export/obj.html — read 2026-07-30 (the same sentence appears on the FBX page).

Two API notes hint that Blender handles the difference somewhere, but neither is about saving: `bpy.path.basename` is documented as skipping a `//` prefix, "Use for Windows compatibility", and `bpy.path.native_pathsep` "Replace the path separator with the system's native os.sep" (Python API, *bpy.path*, https://docs.blender.org/api/5.1/bpy.path.html — read 2026-07-30). Reading a save-time behaviour out of either would be a guess, so I do not.

**Settled by hand test, 2026-08-30.** The same runs were carried out on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and on a rented Linux machine (Ubuntu 22.04.4 LTS), on Blenders with identical build hashes. **What a save records was the same on both**, including the one re-spelling: a place written with a backslash comes back with a forward slash on macOS and on Linux alike, and the plain save keeps the forward-slash form. A file built on the Mac and carried to the Linux machine behaved the same way there. **One thing did differ, and it is about finding a file rather than recording its place:** a place written `//assets/TEX.PNG` against a file named `tex.png` loads its picture on macOS and does not on Linux — the recorded text is the same on both, but the file system's answer is not. **Windows is answered too, on 2026-08-30, and its answer is not the same one.** On a rented machine reporting itself as **Microsoft Windows Server 2025 Standard, build 26100.32995**, the re-spelling runs the other way: every forward slash in a recorded place becomes a backslash when the file is opened, and a plain save with nothing touched writes that into the file. Five places keep their forward slashes there — the compositor's output folder, the cloth cache's folder, the geometry-nodes bake folder, the bake picture's place and the sequencer's proxy folder — and the render output path does not. So a file carried between Windows and the Mac has its recorded text rewritten wholesale by a save that touched nothing — tested in both directions, in 4.5 and in 5.2. The crossing between Windows and Linux was not run. **That is Windows Server 2025 and no desktop edition of Windows**; the account this project rents Windows from gave a Server edition from both of its desktop-named images. See the observed section above. **The third run of the two-routes test, 2026-08-30, was the Creator's Mac alone** — the rented machine the first run used no longer exists — so what it found settles macOS and leaves every other operating system open.

### Where I looked

All searches were run over the whole of `manual_5.1`, `api_5.1` and `release_notes_5.1` unless noted, plus `cycles_5.1`.

Search terms tried, across all three documentation sets: `relative path`, `path separator`, `backslash`, `forward slash`, `normali`, `unchanged`, `rewritten`, `rewrite`, `preserved`, `when saving`, `on save`, `next save`, `re-save`, `resave`, `saved back`, `as-is`, `as is`, `verbatim`, `untouched`, `path manipulation`, `recalculate`, `converted to absolute`, `made absolute`, `made relative`, `remap`, `relative_remap`, `path_remap`, `use_relative_paths`, `filepath_raw`, `// prefix`, `double slash`, `drive letter`, `different drive`, `case sensitive`, `missing`, `modified images`.

Pages opened in full and read:

- Manual: *Opening & Saving* (`files/blend/open_save`), *Packed Data* (`files/blend/packed_data`), *Compatibility* (`files/blend/compatibility`), *Blender File* index (`files/blend/index`), *Introduction* (`files/introduction`), *File Paths* (`files/file_paths`), *Link & Append* (`files/linked_libraries/link_append`), *Linked Libraries* index, *Topbar* (`interface/window_system/topbar`, the whole File ‣ External Data section), *Save & Load Preferences* (`editors/preferences/save_load`), *Outliner ‣ Interface* and *Outliner ‣ Editing* (the unused-data passages), *Data-Block Menu* (`interface/controls/templates/data_block`), *Output Properties ‣ Output* (`render/output/properties/output`), *Configuring Blender* (`getting_started/configuration/introduction`), *Command Line Arguments* (`advanced/command_line/arguments`, the `-o` entry and the whole Argument Order section).
- Manual picture: `_images/files_blend_open-save_file-browser-save.png` (the Save As dialog), opened and read.
- API: `bpy.ops.wm` (`save_mainfile`, `save_as_mainfile`, `save_homefile` entries in full), `bpy.ops.file` (`autopack_toggle`, `find_missing_files`, `make_paths_absolute`, `make_paths_relative`, `pack_all`, `report_missing_files`, `unpack_all`), `bpy.types.BlendDataLibraries` (the whole `write` entry), `bpy.types.PreferencesFilePaths`, `bpy.types.Property` (`is_path_output`, `is_path_supports_blend_relative`, `is_path_supports_templates`), `bpy.path` (whole module page), `bpy.types.Library` (whole page), `bpy.types.Image` (`filepath`, `filepath_raw`).
- Release notes 5.1: `index.md`, `core.md`, `pipeline_io.md`, `python_api.md`, `user_interface.md` read in full or in the relevant part; `bugfixes.md` and `corrective_releases.md` searched for path and library entries.
- Cycles 5.1: `osl.py` (the path-handling functions in full), `operators.py` (the `filepath` assignments), and a search of the whole folder for `FILE_PATH`, `DIR_PATH`, `filepath`, `abspath`.

Ruled out, and why:

- The **release notes** were ruled out as a source for this answer: 5.1's `core.md` covers undo, Python startup and build changes; `pipeline_io.md` covers USD, FBX, glTF, image and video formats. Neither mentions blend-file path handling on save. The only path-remapping line anywhere in the 5.1 notes is a fixed bug about a *different* operation — "Bpy.data.libraries.write with path_remap='RELATIVE_ALL' does not recalculate existing relative paths" (`bugfixes.md`, https://developer.blender.org/docs/release_notes/5.1/bugfixes/, read 2026-07-30) — which concerns `libraries.write`, not saving the main file, so I did not build on it.
- The **importer/exporter pages** (OBJ, FBX, USD, Alembic) were ruled out except where quoted: their relative-path options govern the files those add-ons write, not the `.blend`.
- The **asset library** preference "Relative Path — Use relative path when linking assets from this asset library" (`editors/preferences/file_paths`) was ruled out: it governs how a new link is recorded, not what a save does to an existing one.
- The **Image Editor** and **Video Sequencer** "Relative Path" checkboxes were ruled out for the same reason — they appear at the moment a file is chosen.
- Searches for wording that would state the claim directly (`as-is`, `verbatim`, `untouched`, `unchanged`, `preserved`, `rewritten`, `normali`) returned nothing about blend-file saving anywhere in the three sets. The single `as-is` hit that is about paths is the template sentence quoted above. This is the basis for saying the main claim is not documented outright.

---

## What a running Blender still could not settle

The list that stood here was run in full on 2026-08-30 — on the Creator's Mac, on a rented Linux machine, and on a rented Windows machine — and the results are in the observed sections above. What is left is only what those three machines could not reach:

1. **Any desktop edition of Windows.** All three Windows questions — which way the separator is re-spelled, what a plain save records for a relative place across two drives, and whether letter case in a drive letter survives a save — were answered on 2026-08-30 on a rented machine reporting itself as **Microsoft Windows Server 2025 Standard**, build 26100.32995, and the answers are in the observed section above. What is not settled is whether a desktop edition of Windows answers the same. Both of the desktop-named images tried that day booted Server editions — Server 2025 Standard from one, and **Microsoft Windows Server 2016 Standard**, build 14393.9234, from the other — read off the running systems rather than off the labels.
2. **A cache file, a movie clip and a movie strip.** Each needs a real file of its kind to import, so none could be built into the test file, and nothing has been observed about them.
3. **Why 4.5 writes an auto-save file where 5.0, 5.1 and 5.2 write only a quit file.** Three runs gave the same split. Whether that is a change in Blender or a difference in when a version decides a file has been edited is not settled.
4. **The crossing between Windows and the Mac, in this version.** It was run in 4.5 and in 5.2, as the two ends of the range, and not in this one. This version's silence on it is an untested question rather than a difference in Blender, and the Windows machine is destroyed.

The one list these came from is `Audits/Blender Research/Dependencies/Hand tests — Open Threads questions.md`.

---

2026-08-30 — Claude Opus 5 (Claude Code) — Dependencies, Blender 5.1

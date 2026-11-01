# Dependencies — Open Threads questions, second pass (Blender 4.5)

Sources consulted: the Blender 4.5 manual, the Blender 4.5 Python API documentation, the Blender 4.5 release notes, and the Cycles 4.5 files, all as supplied in `Blender Sources/`. Snapshot taken 2026-07-30; that is the date read for everything cited below. I also opened one manual image file (named in the answer).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: question 20 only, as instructed.

---

## 20. When a `.blend` file is opened and saved again, and none of the places it records for outside files is touched while it is open, what does the saved file record for each of those places?

**Reading of the question.** The question is ambiguous on one point and I flag it rather than choose silently. "Saved again" can mean (a) saving over the same file in the same place — the ordinary `File ‣ Save` — or (b) saving to a new name or a new folder. I answer (a) as the primary reading, because "opened and saved again" reads most naturally as re-saving the same file. Reading (b) is answered separately below, because that is the only case the sources speak about directly.

**LABEL: inferred** (for the main claim). Parts of the answer are documented and are labelled as such where they appear; one sub-question is **not in the sources** and is labelled so.

### Short answer

For reading (a) — saving over the same file — the sources never state outright what is written for an untouched external-file entry. What they do state is when Blender *changes* such an entry at save time: only when the file is being saved to a different directory, and only for entries that are already relative, and only when the "Remap Relative" option is on. None of those conditions is met by a plain save over the same file. From that it follows that an untouched entry is recorded as it already was — a relative entry stays relative, an absolute entry stays absolute. That step is mine, not the source's, so the label is inferred.

For reading (b) — saving to a new location — the documented behaviour is that relative entries are remapped so they still point at the same files from the new location (this is the default for `Save As…`), while absolute entries are not described as being touched at all.

### The documented pieces, quoted

**1. Remapping is tied to saving in a new place, and only to relative entries.** (documented)

> "Remap Relative
>
> This option remaps Relative Paths
> (such as linked libraries and images) when saving a file in a new location."

Source: manual, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html (section "Save As ‣ Options"), read 2026-07-30.

The same page, describing the option from the other direction:

> "If it is necessary to relocate a blend-file relative to its linked resources,
> use Blender's File Save As
> function which has an option to Remap Relative file links."

Source: manual, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html (section "Relative Paths"), read 2026-07-30.

**2. The manual attaches this option to `Save As`, not to `Save`.** (documented, as a fact about the page's structure)

On that page, `File ‣ Save` is described in full as:

> "Save current blend-file over itself (if it was not saved yet, this will automatically switch to Save As…)."

It carries no options list. The options list containing "Compress", "Remap Relative" and "Save Copy" appears under "Save As". The picture the page shows for saving confirms the same three: I opened `_images/files_blend_open-save_file-browser-save.png` (the manual's own illustration, captioned "The File Browser in save configuration") and the right-hand panel is headed "Save As Blender File" with exactly three checkboxes — Compress (ticked), Remap Relative (ticked), Save Copy (unticked). Nothing else about paths appears there.

Source: manual, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html, read 2026-07-30.

**3. The Python API states the same condition, and gives different defaults for the two save operators.** (documented)

> `bpy.ops.wm.save_mainfile(*, filepath='', … compress=False, relative_remap=False, exit=False, incremental=False)`
>
> "Save the current Blender file"
>
> `relative_remap (boolean, (optional)) – Remap Relative, Remap relative paths when saving to a different directory`

> `bpy.ops.wm.save_as_mainfile(*, filepath='', … compress=False, relative_remap=True, copy=False)`
>
> "Save the current file in the desired location"
>
> `relative_remap (boolean, (optional)) – Remap Relative, Remap relative paths when saving to a different directory`

Source: Python API, https://docs.blender.org/api/4.5/bpy.ops.wm.html, read 2026-07-30.

Two documented facts sit in those lines. The condition is "when saving to a different directory". And the plain save operator defaults `relative_remap` to `False`, while the save-as operator defaults it to `True`.

**4. The API spells out what "remap" does and does not do, for the related blend-writing call.** (documented, but for a different function — see the caution below)

> `write(filepath, datablocks, path_remap=False, fake_user=False, compress=False)`
>
> "path_remap (str) –
> Optionally remap paths when writing the file:
>
> NONE No path manipulation (default).
>
> RELATIVE Remap paths that are already relative to the new location.
>
> RELATIVE_ALL Remap all paths to be relative to the new location.
>
> ABSOLUTE Make all paths absolute on writing."

Source: Python API, https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html, read 2026-07-30.

Caution: this is `bpy.data.libraries.write`, which writes chosen data-blocks out to a blend-file. It is not the main-file save. I quote it because it is the only place in the sources that says in words what a remap on writing consists of — "No path manipulation" as the default, and a relative remap touching only "paths that are already relative". Carrying that wording across to the main-file save is an inference, not a reading.

**5. The preference for relative paths does not reach entries that already exist.** (documented)

> "Default To – Relative Paths
>
> Default value for Relative Paths when loading external files
> such as images, sounds, and linked libraries. It will be ignored if a path is already set."

Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/save_load.html, read 2026-07-30.

The API says the same:

> `use_relative_paths`
>
> "Default relative path option for the file selector, when no path is defined yet"
>
> "Type: boolean, default True"

Source: Python API, https://docs.blender.org/api/4.5/bpy.types.PreferencesFilePaths.html, read 2026-07-30.

I also opened the manual's own picture of that preferences page, `_images/editors_preferences_section_save-load.png`. It shows "Relative Paths" as a tick-box in a row labelled "Default To", under the "Blend Files" heading, alongside "Compress File" and "Load UI". It is not presented as a save-time action.

This matters for the question because it rules out the most obvious way an untouched entry could be silently changed on save: the preference is a default for newly entered paths only, and is stated to be ignored where a path already exists.

**6. What an entry can look like when recorded.** (documented)

> "When you specify one of these external files, the default option is to make the path relative.
> Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

> "When relative paths are supported, the File Browser provides a Relative Path checkbox,
> when entering the path into a text field, use a double slash prefix (//) to make it so."

Source: manual, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html (section "Relative Paths"), read 2026-07-30.

> "Blender keeps track of all unpacked resources via a relative or absolute path."

Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html (section "External Data"), read 2026-07-30.

The API marks the individual path properties the same way, for example:

> `filepath` — "Path to the library .blend file" — "Type: string, default "", (never None, blend relative // prefix supported)"

Source: Python API, https://docs.blender.org/api/4.5/bpy.types.Library.html, read 2026-07-30.

> `filepath` — "Image/Movie file name" — "Type: string, default "", (never None, blend relative // prefix supported)"

Source: Python API, https://docs.blender.org/api/4.5/bpy.types.Image.html, read 2026-07-30.

So the two shapes an entry can hold are the relative form beginning `//` and an absolute path, and that is what a saved file has to be recording one of.

**7. One place where the sources do say what the saved text holds: the render output path and the File Output node paths.** (documented)

> "This substitution happens internally at time of use (e.g. while rendering), so
> the template syntax will remain as-is in the path field."

Source: manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html (section "Path Templates"), read 2026-07-30. The same page states these templates are "only supported for the render output path in Scene properties and the output paths in the compositor's File Output node."

The 4.5 release notes record this as new in this version:

> "Some file paths in Blender can now contain "template expressions" like … Currently this is only enabled on the Scene's render output path and the output paths of the compositor's File Output node."

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/core/ (section "Path Templates"), read 2026-07-30.

The API annotation for the render output path carries both flags:

> `filepath` — "Directory/name to save animations, # characters define the position and padding of frame numbers" — "Type: string, default "", (never None, blend relative // prefix supported, Supports template expressions)"

Source: Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30.

For these particular recorded places, then, it is documented that the stored text keeps its `{…}` template as written; the substituted value is never what is held.

### The inference, written out

From (1) and (3): Blender's own words tie remapping of recorded entries at save time to two conditions together — the option being on, and the file being saved to a different directory. From (2) and (3): a plain save over the same file both offers no such option in the interface and defaults it off in the operator. From (5): the relative-paths preference is explicitly ignored where a path already exists, so it cannot convert one at save time either. From (4): where the API does describe writing without remapping, it calls it "No path manipulation".

Step taken: if the only described cause of an entry changing at save time does not apply, the entry is written back as it stood. Therefore the saved file records, for each untouched place, the same form it already held — `//`-prefixed relative entries still relative, absolute entries still absolute.

A reader has to supply that step. No sentence in the sources says "untouched paths are written unchanged". Hence **inferred**, not documented.

### What is genuinely not in the sources

**Whether the recorded text is preserved character for character. LABEL for this sub-question: not in the sources.** Nothing in the manual, the API documentation, the release notes or the Cycles files says whether the string is written back byte for byte, or whether the writer tidies it in any way — path separators swapped, a trailing separator added or dropped, doubled separators collapsed, case altered, a `//` prefix re-formed. I searched for this specifically and found nothing (search record below). The nearest thing in the sources is the helper `bpy.path.native_pathsep(path)` — "Replace the path separator with the systems native os.sep" (https://docs.blender.org/api/4.5/bpy.path.html) — but that is a function a script may call, not something the sources say the saver does.

**Whether "Remap Relative" being on, but the save going to the same directory, changes anything.** The wording is "when saving to a different directory", which reads as a no-op otherwise, but the sources do not state the same-directory case.

**What happens to the recorded place when a save also packs the resource.** The manual documents that packing is deferred to the next save:

> "When enabled, this option will ensure that all eligible external resource files, existing or added later,
> are systematically marked as packed.
> As with Pack Resources, the blend-file must be saved to the drive for this to have an effect."

Source: manual, https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html (section "Automatically Pack Resources"), read 2026-07-30.

So a save can change whether a resource is held inside the file, without anyone touching a path. The manual says only that "Blender keeps track of all unpacked resources via a relative or absolute path" — it does not say what becomes of the recorded place for a resource that the save has just packed. That gap is unresolved in the sources.

### Not covered

This answer does not settle: the exact characters written back for an untouched entry (see above); whether anything differs between the kinds of recorded place — linked library, image, sound, font, cache, movie clip, output directory — since the sources describe path handling in general terms and never per kind, except for the template rule that applies to the render output and File Output node paths; what happens on a same-directory save with "Remap Relative" deliberately turned on; what a save records for a place whose resource was packed by that same save; whether the auto-save and the `.blend1` backup copies record the same thing as the file itself; and what a save writes for an entry whose target is missing at save time.

---

## 20 — observed. What a running Blender does

**Label: observed.** Everything under this heading was got by running Blender 4.5 and reading what it printed and what it wrote to disk. It is not a reading of any documentation, and it does not replace the inferred answer above; where the two disagree, say so plainly rather than quietly preferring one.

### On what kind of computer, and when

On 2026-08-30 these were run twice over: once on the Creator's own computer — a MacBook Pro with an Apple M1 Pro chip running macOS 26.6.2, with Apple's own graphics — and once on a rented Linux machine — Ubuntu 22.04.4 LTS with one NVIDIA RTX 3060, twelve cores and 31 GB of memory, in Poland, driven over SSH, rented and destroyed the same day. The build hash of every Blender used on the rented machine is the same as the one on the Mac, so where the two disagree the difference is the operating system and not the build. Two results are reported wherever both machines ran a test, and they are never folded into one sentence.

### What was done, and to what

A `.blend` file was built by script in Blender 4.5, so that the exact text of every place it records for an outside file was known rather than typed. Beside it sat `assets/tex.png`, `assets/two words.png`, `assets/beep.wav`, `assets/testfont.ttf`, `assets/test.osl`, `assets/test.ies`, a folder named `Тестовая папка` holding `текстура_日本語.png`, a second blend-file `lib.blend`, and one folder up an `outer.png`.

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

The two files on disk were never the same size and never the same file: 553,115 bytes before the plain save and 550,207 after, with different checksums. **So comparing whole files says nothing; only the path text can be compared.**

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

So which way a check asks for a recorded place decides what string it gets back. Blender 4.5 has `bpy.utils.blend_paths` and `bpy.data.file_path_map`, and `bpy.data.file_path_foreach` is not there at all:

```
has bpy.utils.blend_paths	True
has bpy.data.file_path_map	True
has bpy.data.file_path_foreach	False
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
VERSION 4.5.12 LTS HASH 84afd5f785f7
PLATFORM darwin
```

The starting file was built fresh, and both questions were asked twice over: once **inside the process that had just built the file and had never reloaded it**, and once in a **fresh process that opened the file from disk**. The two commands in full, after the application's own path: `--background --factory-startup --python build_main.py -- <project folder> <scripts folder>`, then `--background --factory-startup <the file> --python read_only.py -- <scripts folder>`. The two routes were compared character for character by script rather than read by eye.

*What came back inside the process that built the file, before any reload:*

```
4.5  rel-backslash file_path_map -> ['//assets\\tex.png']
4.5  raw-setter    file_path_map -> ['//assets\\..\\assets//TEX.PNG ']
4.5  blend_paths entries holding a backslash -> ['//assets\\..\\assets//TEX.PNG ', '//assets\\tex.png']
```

*And in the fresh process that had opened the file from disk:*

```
4.5  rel-backslash file_path_map -> ['//assets/tex.png']
4.5  raw-setter    file_path_map -> ['//assets/../assets//TEX.PNG ']
4.5  blend_paths entries holding a backslash -> []
```

So on this machine the two routes reported those two entries the same way as each other, and what changed the spelling was the reload rather than the route.

*One thing that is not the disagreement described above, and is recorded rather than folded into it.* The two routes do not return the same **collection** of strings. Compared as multisets in the build process:

```
4.5 : blend_paths 24 strings, file_path_map 27 strings
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
preference use_file_compression False
```

**A file written by Blender 5.2, opened here and plain-saved.** It went through with no prompt of any kind, from a run with no window:

```
Warning: File written by newer Blender binary (502.44), expect loss of data!
save_mainfile() returned {'FINISHED'}
```

No recorded place changed but the unused data-block above.

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

**Label: observed.** Everything under this heading was got by running Blender 4.5.12 LTS on a rented Windows machine on 2026-08-30, and reading what it printed and what it wrote to disk. It is not a reading of any documentation. It answers the Windows half of this question, which every run before it left open, and it replaces nothing above: where a Windows result differs from the Mac's or the rented Linux machine's, both stand and the difference is named.

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

**The Blender was the official Windows x64 build from Blender's own download server, and its hash is the same as the Creator's Mac's and the rented Linux machine's.** Asked of the running program itself, not read off a download page (`Windows/4.5-00-machine.txt`; the same four hashes are in `Windows/01-builds.txt` and `Windows/00-the-machine.txt`):

```
RESOURCES C:\q20\res-4.5
VERSION 4.5.12 LTS HASH 84afd5f785f7
BUILD PLATFORM b'Windows'
BUILD TYPE b'Release'
BINARY C:\blender\4.5.12\blender.exe
PLATFORM win32
OS SEP '\\' ALTSEP '/'
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:33:53)
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
C:\blender\4.5.12\blender.exe --background --factory-startup --python C:\q20\scripts\build_main.py -- C:\q20\work\4.5\proj C:\q20\scripts
C:\blender\4.5.12\blender.exe --background --factory-startup C:\q20\work\4.5\proj\main.blend --python C:\q20\scripts\read_only.py -- C:\q20\scripts
C:\blender\4.5.12\blender.exe --background --factory-startup C:\q20\work\4.5\proj\main.blend --python C:\q20\scripts\plain_save.py -- C:\q20\scripts
```

Six more runs followed from the same starting file, put back fresh each time: a save into another folder on the same drive with nothing passed for the remap setting, the same with the setting turned off, a save into the same folder under a new name, the make-paths-absolute command, the make-paths-relative command, and a save onto the second drive.

#### What came back — setting a place changes nothing, whichever way it is spelled

Every one of the twenty spellings came back out of the setting exactly as it went in, in memory, before anything was saved (`Windows/4.5-01-build.txt`):

```
SET	rel-plain	wanted='//assets/tex.png'	got_filepath='//assets/tex.png'	got_raw='//assets/tex.png'
SET	rel-backslash	wanted='//assets\\tex.png'	got_filepath='//assets\\tex.png'	got_raw='//assets\\tex.png'
SET	abs-forward	wanted='C:/q20/work/4.5/proj/assets/tex.png'	got_filepath='C:/q20/work/4.5/proj/assets/tex.png'	got_raw='C:/q20/work/4.5/proj/assets/tex.png'
SET	abs-lower-drive	wanted='c:\\q20\\work\\4.5\\proj\\assets\\tex.png'	got_filepath='c:\\q20\\work\\4.5\\proj\\assets\\tex.png'	got_raw='c:\\q20\\work\\4.5\\proj\\assets\\tex.png'
SET	drive-relative	wanted='C:q20\\work\\4.5\\proj\\assets\\tex.png'	got_filepath='C:q20\\work\\4.5\\proj\\assets\\tex.png'	got_raw='C:q20\\work\\4.5\\proj\\assets\\tex.png'
SET	other-drive	wanted='X:\\assets\\tex.png'	got_filepath='X:\\assets\\tex.png'	got_raw='X:\\assets\\tex.png'
```

#### What came back — opening the file re-spells the separator, and on Windows it runs the other way

**On the Mac and on the rented Linux machine a backslash becomes a forward slash. On Windows Server 2025 every forward slash becomes a backslash.** These are the same places, read on the very next open with nothing touched in between (`Windows/4.5-02-read-before.txt`):

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
image[abs-forward].filepath	'C:\\q20\\work\\4.5\\proj\\assets\\tex.png'
image[raw-setter].filepath	'//assets\\..\\assets\\\\TEX.PNG '
scene.render.filepath	'//out\\{blend_name}_####.png'
```

Against what the same places held in memory when the file was written, which is quoted in full in the raw output: `'//assets/tex.png'`, `'//assets/../assets/tex.png'`, `'//assets//tex.png'`, `'//assets/TEX.PNG'`, `'//assets/tex.png '`, `'//assets/two words.png'`, `'//Тестовая папка/текстура_日本語.png'`, `'//../outer.png'`, `'//nowhere/missing.png'`, `'C:/q20/work/4.5/proj/assets/tex.png'`, `'//assets\\..\\assets//TEX.PNG '` and `'//out/{blend_name}_####.png'`.

**Nothing else about the text moved.** The leading `//` stays, the doubled separator stays doubled and is only spelled the other way, the step back up stays, the letter case stays, the trailing space stays, the non-Latin characters come through whole, and a place pointing at nothing is re-spelled just the same. Only the separator changes.

**Seven lines still hold a forward slash after the open, and they are the same seven in every version.** Read by name rather than inferred from a missing line — these are every line in that output whose value still carries a forward slash:

```
library[lib.blend].filepath	'//lib.blend'
weakref[images:lib-inside-image].filepath	'//lib.blend'
scene.render.bake.filepath	'//bake/baked.png'
sequence_editor.proxy_dir	'//proxies/'
fileoutput[ProbeFileOutput].base_path	'//comp_out/{blend_name}/'
object[ProbeCloth].mod[Cloth].point_cache.filepath	'//cache/'
object[ProbeGN].mod[GeometryNodes].bake_directory	'//gn_bake/'
```

**The first two are on that list only because there is no separator in them to change** — the library's own place is `//lib.blend`, with nothing between the marker and the name, and the weak reference beside it is the same string. So five real places are left alone: the compositor's output folder, the cloth cache's folder, the geometry-nodes bake folder, the bake picture's place, and the sequencer's proxy folder.

**The render output path is not one of them**, and that is worth stating because the five that survive are all places a file is written to, and the render output path is a place written to as well. It becomes `'//out\\{blend_name}_####.png'`.

**The two settings a script can read a place through printed the same string every time.** Every `filepath` line in this version's output is followed by a `filepath_raw` line carrying the same characters, in the run that built the file and in every run that opened it. Neither of them shows the stored text once the file has been opened.

#### What came back — a plain save with nothing touched makes the re-spelling permanent

The two files on disk are never the same file (`Windows/4.5-05-bytes.txt`):

```
--- size on disk ---
      593371  C:\q20\work\4.5\main_after_build.blend
      590463  C:\q20\work\4.5\main_after_plain_save.blend

--- byte for byte ---
C:\q20\work\4.5\main_after_build.blend C:\q20\work\4.5\main_after_plain_save.blend differ: char 21, line 1

--- sha256 ---
54b528f5b303e449355370bf7e1b2d21907690ca179e0520648ffddac93a40d3  C:\q20\work\4.5\main_after_build.blend
55824f9e501ee8630ffc48df6f8f4e762c441a0c92ec946bb9d983b5793cbcb2  C:\q20\work\4.5\main_after_plain_save.blend
```

The path text in the file the build wrote, pulled out of its own bytes, with the count of each on the left as the machine printed it:

```
      1 //../../../../Desktop/
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
      1 //comp_out/{blend_name}/
      1 //gn_bake/
      2 //lib.blend
      1 //nowhere/missing.png
      1 //out/{blend_name}_####.png
      1 //proxies/
      1 //Тестовая папка/текстура_日本語.png
      1 C:/q20/work/4.5/proj/assets/tex.png
      1 C:\Users\Administrator\Documents
      3 C:\q20\work\4.5\proj\assets\tex.png
      1 C:q20\work\4.5\proj\assets\tex.png
      1 X:\assets\tex.png
      1 c:\q20\work\4.5\proj\assets\tex.png
```

and the path text in the same file after one plain save with nothing touched:

```
      1 //../../../../Desktop/
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
      1 //comp_out/{blend_name}/
      1 //gn_bake/
      1 //lib.blend
      1 //nowhere\missing.png
      1 //out\{blend_name}_####.png
      1 //proxies/
      1 //Тестовая папка\текстура_日本語.png
      1 C:\Users\Administrator\Documents
      4 C:\q20\work\4.5\proj\assets\tex.png
      1 C:q20\work\4.5\proj\assets\tex.png
      1 X:\assets\tex.png
      1 c:\q20\work\4.5\proj\assets\tex.png
```

Two lines in each list are not recorded places from the test file — the account's own documents folder and a leftover desktop path — and they are quoted here rather than trimmed, because the list is the machine's own output.

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

and after it, one — the weak reference gone with it (`Windows/4.5-04-read-after.txt`). The unused image shielded with a fake user survived, and the third, unused and unshielded, never reached the first read because the build's own save had already dropped it.

**Why the two reads look the same although the file changed.** The re-spelling is already done by the time anything can read the file at all, so the read before the plain save and the read after it agree; the change is only visible in the bytes. **A check that reads a place through Blender and compares it against the text in the file will match on a file Windows has already saved, and will miss on a file that arrived from anywhere else.**

#### What came back — the two ways of listing every recorded place agree with each other on Windows

The Mac's first run recorded these two routes disagreeing about spelling in the process that had just built the file. **On Windows Server 2025 they do not disagree, in either state.** In the build run, before any reload, both report the backslash entry with its backslash and the plain entry with its forward slash; after the file has been opened from disk, both report every entry with backslashes (`Windows/4.5-01-build.txt` and `Windows/4.5-02-read-before.txt`):

```
has bpy.utils.blend_paths	True
has bpy.data.file_path_map	True
has bpy.data.file_path_foreach	False
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
4.5 : blend_paths 32 strings, file_path_map 35 strings
     only file_path_map: '//assets/tex.png'
     only file_path_map: '//lib.blend'
     only file_path_map: '//lib.blend'

after the file was opened from disk
4.5 : blend_paths 30 strings, file_path_map 34 strings
     only file_path_map: '//assets\\tex.png'
     only file_path_map: '//lib.blend'
     only file_path_map: '//lib.blend'
     only file_path_map: '//lib.blend'
```

The extras are the linked library's own inner picture, which the second route reaches through the library and the first does not, and repeated mentions of the library itself. The one string that looks like a spelling difference is not one: it is `'//assets/tex.png'` before the reload and `'//assets\\tex.png'` after it, which is the same entry following the same re-spelling as everything else.

**So the disagreement the Mac's first run recorded is not a Windows disagreement**, and that is a clean negative rather than a silence: a check may read a recorded place through either route on this machine and get the same characters. **It does not make the underlying danger smaller — it moves it.** On Windows both routes agree with each other and both disagree with the file that arrived, whenever that file was written anywhere but Windows.

#### What came back — a relative place across two drives

**A plain save leaves it exactly alone.** `X:\assets\tex.png` was stored, read back and written again unchanged, and the picture loaded from it — the read prints `image[other-drive].size	(2, 2)`.

**The make-paths-relative command names it, refuses it, and reports itself finished** (`Windows/4.5-10-make-relative.txt`):

```
CALL bpy.ops.file.make_paths_relative()
Warning: Path 'X:\assets\tex.png' cannot be made relative for Image 'other-drive'
Warning: Total files 30 | Changed 6 | Failed 1
returned {'FINISHED'}
```

It counts the failure and says which place failed and which data-block it belonged to, and it still returns the same answer it returns when everything worked. **A caller that reads only the answer will not know one place was left behind.** The place itself is untouched and the save that follows writes it back as it was.

**Saving the whole file onto the other drive turns every relative place into a full path naming the old drive, and says so nowhere** (`Windows/4.5-11-saveas-other-drive.txt`):

```
CALL bpy.ops.wm.save_as_mainfile(filepath='X:\\elsewhere\\onotherdrive.blend')  [relative_remap not passed]
returned {'FINISHED'}

image[rel-plain].filepath	'C:\\q20\\work\\4.5\\proj\\assets\\tex.png'
image[nonlatin].filepath	'C:\\q20\\work\\4.5\\proj\\Тестовая папка\\текстура_日本語.png'
image[missing].filepath	'C:\\q20\\work\\4.5\\proj\\nowhere\\missing.png'
library[lib.blend].filepath	'C:\\q20\\work\\4.5\\proj\\lib.blend'
```

where the same call into a different folder on the *same* drive keeps them relative. The one place already on the new drive, `X:\assets\tex.png`, is the only one left as it was. No warning was printed.

#### What came back — letter case in a drive letter

**A plain save does not change it.** `c:\q20\work\4.5\proj\assets\tex.png` was stored, read back and written again with its lower-case letter, and the upper-case one likewise. **Windows finds the file either way** — both print `size	(2, 2)` — which is the same as the Mac and unlike Linux, where the wrong letter case in a file name stops the picture loading.

**Making paths absolute does not change it either** (`Windows/4.5-09-make-absolute.txt`):

```
image[abs-lower-drive].filepath	'c:\\q20\\work\\4.5\\proj\\assets\\tex.png'
```

**Making paths relative removes the drive letter and with it the question.** The upper-case and the lower-case place collapse to the same text:

```
image[abs-lower-drive].filepath	'//assets\\tex.png'
image[abs-upper-drive].filepath	'//assets\\tex.png'
```

**A drive letter with no root behind it is mangled.** A place written as a drive letter followed straight by a name is treated by the make-paths-relative command as an ordinary relative name and given steps back up in front of it:

```
image[drive-relative].filepath	'//..\\..\\..\\..\\C:q20\\work\\4.5\\proj\\assets\\tex.png'
```

The result cannot name any file on any machine. It is recorded as what happened and is not called a fault here, because whether Blender means to support that spelling at all is not something this test can settle. The place did not load its picture before the command either — it prints `size	(0, 0)` — so nothing that was working stopped working. The make-paths-absolute command left the same place untouched.

#### What came back — saving somewhere else on the same drive

**The same shape as the Mac and Linux, written the Windows way.** With nothing passed for the remap setting, every relative place a file is read from is rewritten, lengthened to reach back to where the assets are, and spelled with backslashes (`Windows/4.5-06-saveas-default.txt`):

```
CALL bpy.ops.wm.save_as_mainfile(filepath='C:\\q20\\elsewhere\\4.5\\moved_default.blend')  [relative_remap not passed]
returned {'FINISHED'}

image[rel-plain].filepath	'//..\\..\\work\\4.5\\proj\\assets\\tex.png'
image[rel-dotdot].filepath	'//..\\..\\work\\4.5\\proj\\assets\\tex.png'
image[rel-doubleslash].filepath	'//..\\..\\work\\4.5\\proj\\assets\\tex.png'
image[case-caps].filepath	'//..\\..\\work\\4.5\\proj\\assets\\TEX.PNG'
image[trailing-space].filepath	'//..\\..\\work\\4.5\\proj\\assets\\tex.png '
library[lib.blend].filepath	'//..\\..\\work\\4.5\\proj\\lib.blend'
```

The doubled separator is collapsed, the step back up resolved, the letter case kept, the trailing space kept, a place pointing at nothing rewritten just the same, and the full paths not touched. **With the setting turned off nothing changed at all**, and **saving into the same folder under a new name changed nothing either** — every recorded place came back identical in both (`Windows/4.5-07-saveas-remap-off.txt`, `Windows/4.5-08-saveas-same-folder.txt`). So the boundary is a different folder, exactly as it is on the Mac.

#### What came back — carrying the file between Windows and the Mac, both ways

**A Windows-saved file, opened on the Mac and saved with nothing touched, has every backslash turned into a forward slash.** The path text present in one file and not the other, after the Mac's plain save (`Crossing/windows-file-on-mac-4.5.txt`):

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
- C:\q20\work\4.5\proj\assets\tex.png
- C:q20\work\4.5\proj\assets\tex.png
- X:\assets\tex.png
- c:\q20\work\4.5\proj\assets\tex.png
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
+ C:/q20/work/4.5/proj/assets/tex.png
+ C:q20/work/4.5/proj/assets/tex.png
+ X:/assets/tex.png
+ c:/q20/work/4.5/proj/assets/tex.png
```

Two further lines in that comparison are left out here because they are not recorded places from the test file — they are the temporary folders the two copies sat in, one on each machine.

**The full Windows paths are re-spelled too.** The Mac has no drive letters and can resolve neither `C:\…` nor `X:\…`, and it rewrites both anyway.

**And back the other way, the same thing in reverse.** The same file carried back and saved once on Windows with nothing touched turns every forward slash into a backslash again, including inside the drive-letter paths (`Crossing/crossing-mac-file-on-windows-4.5.txt`).

**The file is the same size every time and never the same bytes:**

```
--- size on disk ---
      590463  C:\q20\backfrommac\4.5\arrived_from_mac.blend
      590463  C:\q20\backfrommac\4.5\proj\main.blend

--- byte for byte ---
C:\q20\backfrommac\4.5\arrived_from_mac.blend C:\q20\backfrommac\4.5\proj\main.blend differ: char 21, line 1
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

Partly documented, and it is the better-documented half. The two saves are exposed as operators with the same option, and their documented defaults differ: `bpy.ops.wm.save_mainfile` has `relative_remap=False` and `bpy.ops.wm.save_as_mainfile` has `relative_remap=True` (Python API, https://docs.blender.org/api/4.5/bpy.ops.wm.html, read 2026-07-30). A script or a headless run that calls `save_mainfile` with no arguments therefore asks for no remapping at all; one that calls `save_as_mainfile` with no arguments asks for remapping by default. `bpy.data.libraries.write` likewise defaults to `NONE — No path manipulation` (https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html).

What the sources do not say: whether a background run behaves differently in any other way at save time, whether preferences are read at all in such a run, and whether saving with no window open changes anything about recorded places. Not in the sources.

**Settled by hand test, 2026-08-30.** Every run above was driven from outside Blender's own window, with `--background`, on the Creator's Mac and on a rented Linux machine. Saving from outside behaves as the operator settings say: `bpy.ops.wm.save_mainfile()` with nothing passed does no remapping, and `bpy.ops.wm.save_as_mainfile(filepath=…)` with nothing passed does remap when the folder changes. Two things a driven run does that a person clicking does not. Giving `-o` after the file name replaces the render output place in memory, and a following plain save writes it into the file. And overwriting a file written by a newer Blender goes through silently from outside the window — it printed a warning and finished, with no confirmation to answer. See the observed section above. **Added by the third run, 2026-08-30, on the Creator's Mac.** A script reading the recorded places from outside got the same characters whichever of the two routes it used, `bpy.utils.blend_paths` or `bpy.data.file_path_map`: both reported the two backslash entries with their backslashes in the process that had just built the file, and both reported them with forward slashes in a fresh process that had opened the file from disk. On that run the reload changed the spelling and the route did not. **An earlier run of the same test recorded the two routes disagreeing there, and it has not been overwritten** — all three runs are set out in the observed section above. **Added by the Windows runs, 2026-08-30.** Those were driven from outside Blender's own window as well, with `--background` on a rented machine reporting itself as Microsoft Windows Server 2025 Standard, build 26100.32995, and the same settings behaved the same way: `bpy.ops.wm.save_mainfile()` with nothing passed did no remapping, and `bpy.ops.wm.save_as_mainfile(filepath=…)` with nothing passed remapped when the folder changed. **One thing a driven run cannot see from the answer alone**: asked to make paths relative on a file holding a place on a second drive letter, the command returned `{'FINISHED'}` having refused that place, and only the printed warning named it. On that machine both routes for listing every recorded place reported the same characters as each other, in the build run and after a reload.

### Operating system

Not in the sources. Nothing in the manual's saving pages, the preferences pages, the release notes or the API entries for the save operators makes any statement conditional on the operating system, and I found no per-system note anywhere in the material about how a recorded place is written at save time. I searched for the obvious terms (see the record below) and found only unrelated hits — the wheels page telling add-on authors to "Use forward slashes as path separators" when listing wheels, the OBJ importer's own "Path Separator" option, and the asset catalogs rule "Only / as separator (no \; think less filesystem path and more URL)". None of those concerns saving a `.blend`. So: the sources do not say, and I will not guess.

**Settled by hand test, 2026-08-30.** The same runs were carried out on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and on a rented Linux machine (Ubuntu 22.04.4 LTS), on Blenders with identical build hashes. **What a save records was the same on both**, including the one re-spelling: a place written with a backslash comes back with a forward slash on macOS and on Linux alike, and the plain save keeps the forward-slash form. A file built on the Mac and carried to the Linux machine behaved the same way there. **One thing did differ, and it is about finding a file rather than recording its place:** a place written `//assets/TEX.PNG` against a file named `tex.png` loads its picture on macOS and does not on Linux — the recorded text is the same on both, but the file system's answer is not. **Windows is answered too, on 2026-08-30, and its answer is not the same one.** On a rented machine reporting itself as **Microsoft Windows Server 2025 Standard, build 26100.32995**, the re-spelling runs the other way: every forward slash in a recorded place becomes a backslash when the file is opened, and a plain save with nothing touched writes that into the file. Five places keep their forward slashes there — the compositor's output folder, the cloth cache's folder, the geometry-nodes bake folder, the bake picture's place and the sequencer's proxy folder — and the render output path does not. So a file carried between Windows and the Mac has its recorded text rewritten wholesale by a save that touched nothing — tested in both directions, in 4.5 and in 5.2. The crossing between Windows and Linux was not run. **That is Windows Server 2025 and no desktop edition of Windows**; the account this project rents Windows from gave a Server edition from both of its desktop-named images. See the observed section above. **The third run of the two-routes test, 2026-08-30, was the Creator's Mac alone** — the rented machine the first run used no longer exists — so what it found settles macOS and leaves every other operating system open.

---

## Where I looked

Everything below was run against `Blender Sources/manual_4.5`, `Blender Sources/api_4.5`, `Blender Sources/release_notes_4.5` and `Blender Sources/cycles_4.5` only.

**Whole-set searches (terms, and what came back)**

- `relative path` across the manual — 21 files. I opened the ones that could bear on saving: `files/blend/open_save.txt`, `editors/preferences/save_load.txt`, `editors/preferences/file_paths.txt`, `editors/file_browser.txt`, `files/linked_libraries/link_append.txt`, `interface/window_system/topbar.txt`, `files/blend/packed_data.txt`. Ruled out as unrelated to saving a `.blend`: the importer/exporter pages (FBX, OBJ, USD, Alembic), the image-editing page, the video-strip pages, the add-ons page, the audio and output pages, `import_images_as_planes`.
- `relative_remap`, `remap_relative`, `Remap Relative` across all three documentation sets — five hits total, all quoted above (two in `bpy.ops.wm`, two on the open/save manual page, plus the operator signatures).
- `remap` / `Remap` across the whole manual — every other hit is about remapping values, colours, time or data-block users (Map Range node, timeline, NLA, outliner "Remap Users", texture colour ramps). None concerns file paths at save time. Ruled out on that basis.
- `path_remap` across the API — only `bpy.types.BlendDataLibraries.write`, quoted above.
- `use_relative_paths` across the API and manual — `bpy.types.PreferencesFilePaths` and the two index files only.
- `make_paths_relative`, `make_paths_absolute`, `Make Paths Relative`, `Make Paths Absolute` — `bpy.ops.file` and the topbar "External Data" menu. These are deliberate user commands, so they fall outside "none of the places is touched"; I read them to be sure neither is described as running automatically at save. Neither is.
- Wording sweep for a statement that untouched entries survive a save: `unchanged`, `preserved`, `as-is`, `verbatim`, `rewritten`, `normalize`, `normalise`, `written back`, `untouched`, `left alone`, `stays relative`, `remains relative`, `stay absolute`, `converted to absolute`, `converted to relative` — run across manual, API and release notes together. Hits were all irrelevant (Follow Path constraint offsets, weight-paint locked groups, sculpt visibility, mesh Edit-Mode data, the glTF exporter, the Alembic exporter, `ID.session_uid`, a USD hook returning an asset path unchanged) except two I kept: the "as-is" line about template syntax on the file-paths page, and the "Library references are preserved" line on `bpy.ops.file.pack_libraries`. No statement of the form "paths are written unchanged when saving" exists in the material.
- `when saving` / `on save` / `is saved` / `saving the blend` filtered to lines also mentioning path, library, external or reference — only the "new location" line for Remap Relative, the compress-default preference, the save-versions preference, and unrelated asset-library and library-override lines.
- `backslash`, `slash`, `separator` filtered to path lines across the manual — three unrelated hits (Python wheels, OBJ importer option, asset catalog separators) plus the `//` sentence on the open/save page. This is the basis for the "not in the sources" on character-level preservation and on operating-system differences.
- `relative path` / `relative paths` across the API — the per-operator `relative_path` "Select the file relative to the blend file" flags on the file-selecting operators (image open, font open, node file selection, `wm` open/link/append), the USD exporter's `relative_paths`, `UserAssetLibrary`, and the preference. All concern choosing a path, not re-writing one at save. Ruled out.
- Release notes: read `core.md` (only "Path Templates" mentions paths), `python_api.md` (the `PATH_SUPPORTS_BLEND_RELATIVE` warning for `StringProperty`, and the note that `frame_path()` can now fail on template errors), `index.md` in full (38 lines, nothing on saving paths), and grepped every note file for path-plus-relative/absolute/remap/save. `pipeline_assets_io.md` has no `path` hit at all. Nothing in the 4.5 notes records a change to what a save writes for an untouched external-file entry.

**Pages read in full or in the relevant part**

- `https://docs.blender.org/manual/en/4.5/files/blend/open_save.html` — whole page.
- `https://docs.blender.org/manual/en/4.5/editors/preferences/save_load.html` — whole page.
- `https://docs.blender.org/manual/en/4.5/files/file_paths.html` — whole page.
- `https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html` — whole page.
- `https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html` — the "External Data" and "Clean Up" sections.
- `https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html` — the "Options" section.
- `https://docs.blender.org/manual/en/4.5/editors/file_browser.html` — grepped for path/relative; only the "Relative Path" entry, which points back to the open/save page.
- `https://docs.blender.org/manual/en/4.5/files/blend/compatibility.html` — grepped for path, library, external: no hits at all. Ruled out.
- `https://docs.blender.org/manual/en/4.5/files/introduction.html` — same, no hits. Ruled out.
- `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html` — grepped for relative and save. Only the render-output line "Use // at the start of the path to render relative to the blend-file." Nothing about saving a blend-file from the command line. Ruled out for this question.
- `https://docs.blender.org/api/4.5/bpy.ops.wm.html` — the `save_mainfile`, `save_as_mainfile`, `save_homefile` and `revert_mainfile` entries.
- `https://docs.blender.org/api/4.5/bpy.ops.file.html` — the pack/unpack and make-paths entries.
- `https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html` — `load` and `write` in full.
- `https://docs.blender.org/api/4.5/bpy.types.PreferencesFilePaths.html` — the `use_*` block.
- `https://docs.blender.org/api/4.5/bpy.types.Library.html`, `.../bpy.types.Image.html`, `.../bpy.types.RenderSettings.html` — the `filepath` entries and their type annotations.
- `https://docs.blender.org/api/4.5/bpy.path.html` — whole module page (`abspath`, `relpath`, `native_pathsep`, `is_subdir`, and the rest).

**Pictures opened**

- `_images/files_blend_open-save_file-browser-save.png` (from the open/save page). Shows the save file browser with a "Save As Blender File" panel containing Compress, Remap Relative, Save Copy.
- `_images/editors_preferences_section_save-load.png` (from the Save & Load preferences page). Shows "Default To: Relative Paths / Compress File / Load UI" under "Blend Files".

**Cycles files**

Searched `cycles_4.5` for `FILE_PATH`, `filepath`, `subtype='DIR`, `relative` and `custom_filepath`. What is there: `osl.py` compiling `.osl` script files and calling `bpy.path.abspath(filepath, library=library)`; `operators.py` with `input_filepath` / `output_filepath` `StringProperty(… subtype='FILE_PATH')` on a denoising operator, which are operator arguments rather than settings stored in a `.blend`; `ui.py` with a file-browser callback. `cam.custom_filepath` is referenced in `osl.py` but is not defined in the add-on's Python `properties.py`, so I could not read its declaration. Nothing in the Cycles files says anything about what a `.blend` save writes for a recorded path, so nothing here is quoted as an answer. Recording this so the absence is on the record rather than assumed.

---

## What a running Blender still could not settle

The list that stood here was run in full on 2026-08-30 — on the Creator's Mac, on a rented Linux machine, and on a rented Windows machine — and the results are in the observed sections above. What is left is only what those three machines could not reach:

1. **Any desktop edition of Windows.** All three Windows questions — which way the separator is re-spelled, what a plain save records for a relative place across two drives, and whether letter case in a drive letter survives a save — were answered on 2026-08-30 on a rented machine reporting itself as **Microsoft Windows Server 2025 Standard**, build 26100.32995, and the answers are in the observed section above. What is not settled is whether a desktop edition of Windows answers the same. Both of the desktop-named images tried that day booted Server editions — Server 2025 Standard from one, and **Microsoft Windows Server 2016 Standard**, build 14393.9234, from the other — read off the running systems rather than off the labels.
2. **A cache file, a movie clip and a movie strip.** Each needs a real file of its kind to import, so none could be built into the test file, and nothing has been observed about them.
3. **Why 4.5 writes an auto-save file where 5.0, 5.1 and 5.2 write only a quit file.** Three runs gave the same split. Whether that is a change in Blender or a difference in when a version decides a file has been edited is not settled.

The one list these came from is `Audits/Blender Research/Dependencies/Hand tests — Open Threads questions.md`.

---

2026-08-30 — Claude Opus 5 (Claude Code) — Dependencies, Blender 4.5

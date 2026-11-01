# Dependencies — Open Threads questions (Blender 4.5)

**Sources consulted, and when.** Blender 4.5 manual, Blender 4.5 Python API documentation, Blender 4.5 release notes, and the Cycles add-on's own files — all as supplied in `Blender Sources/`, snapshot dated 2026-07-30. That snapshot date is the date read for everything below. I also opened one image from the manual's `_images` folder (named in the answer).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

*(One clarification, so the line above is exact: to confirm the destination folder existed I listed the file names in `Preparation/Blender Research/Findings/4.5/`. I opened none of those files, and read nothing beyond their names.)*

Questions answered in this pass: 20 only.

---

## 20. When a `.blend` file is opened and saved again, and none of the places it records for outside files is touched while it is open, what does the saved file record for each of those places?

### Short answer

Each entry comes back in the same form it went in. A path that was recorded relative to the blend-file (starting `//`) is still recorded that way; a path that was recorded as a full path from the system's root is still recorded that way. A plain save over the same file changes nothing about which of those two forms a place is written in, because the one save-time conversion the sources describe — **Remap Relative** — is tied to saving the file *to a different place*, and is off by default when a save is driven from a script.

That is a claim about the form, not about the exact characters. Whether the string itself comes back identical — the direction of the separators, the letter case, a trailing separator — is not settled either way by anything below; see *Not covered*.

Two things sit alongside that and are worth stating separately, because they are the exceptions people trip on:

* If the recorded place contains a **template expression** (in 4.5 that means the render output path and the compositor File Output node's output paths), the template text stays in the field as text. It is not swapped for its expanded value at save time.
* If the file already had **Automatically Pack Resources** switched on before it was opened, the act of saving is itself when packing happens. That changes what the file *contains*, not what it *records as a path*.

The overall claim is **inferred**, not documented. No page in the 4.5 manual or API documentation says outright "a save writes each unchanged path back byte for byte". What the sources say is that every mechanism that rewrites paths is something the user or a script has to ask for. I set out the pieces and the step below.

---

### Piece 1 — Remap Relative is a save-time path conversion, and it is described as being for saving elsewhere

**Label: documented**

That it is the *only* save-time path conversion is not in the words below. That part rests on having searched and found nothing else, so it belongs with the record of where I looked and with the inferred step at the end, not with this piece.

Exact words, from the Save As options in the manual:

> "Remap Relative
>
> This option remaps Relative Paths
> (such as linked libraries and images) when saving a file in a new location."

Source: manual — https://docs.blender.org/manual/en/4.5/files/blend/open_save.html
Date read: 2026-07-30

The same option in the API documentation, for both save operators:

> "relative_remap (boolean, (optional)) – Remap Relative, Remap relative paths when saving to a different directory"

Source: Python API — https://docs.blender.org/api/4.5/bpy.ops.wm.html
Date read: 2026-07-30

And its default, in the signature of the plain save operator:

> "bpy.ops.wm.save_mainfile(*, filepath='', hide_props_region=True, check_existing=True, filter_blender=True, filter_backup=False, filter_image=False, filter_movie=False, filter_python=False, filter_font=False, filter_sound=False, filter_text=False, filter_archive=False, filter_btx=False, filter_collada=False, filter_alembic=False, filter_usd=False, filter_obj=False, filter_volume=False, filter_folder=True, filter_blenlib=False, filemode=8, display_type='DEFAULT', sort_method='', compress=False, relative_remap=False, exit=False, incremental=False)"

Source: Python API — https://docs.blender.org/api/4.5/bpy.ops.wm.html
Date read: 2026-07-30

The part that matters in that long line is `relative_remap=False`.

The manual also describes what a plain save is: "Save current blend-file over itself (if it was not saved yet, this will automatically switch to Save As…)." (manual — https://docs.blender.org/manual/en/4.5/files/blend/open_save.html, read 2026-07-30). So the ordinary case in the question — open a saved file, save it again — is a save over the same location, which is not "a new location" and not "a different directory".

**From a picture.** The manual's screenshot of the File Browser in save configuration shows the "Save As Blender File" side panel with exactly three tick boxes: Compress, Remap Relative, Save Copy. In that captured screenshot Compress and Remap Relative are shown ticked and Save Copy unticked. Image: `_images/files_blend_open-save_file-browser-save.png`, on the page https://docs.blender.org/manual/en/4.5/files/blend/open_save.html (read 2026-07-30). Label for what the picture shows: **documented**. Do not read the ticks as the shipped defaults — they are the state of one person's screen when the screenshot was taken, and the API signature above gives `False` for the scripted default. What the picture does settle is that Remap Relative is an option presented in the Save As panel, alongside Compress and Save Copy.

---

### Piece 2 — converting paths in bulk has its own menu entries and its own operators

**Label: documented**

The two operations that rewrite every recorded place at once have a menu entry each and an operator each:

> "Make Paths Relative
>
> Make all paths to external files relative to the current blend-file.
>
> Make Paths Absolute
>
> Make all paths to external files absolute (= full path from the system's root)."

Source: manual — https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html
Date read: 2026-07-30

The same two as operators, with no options at all:

> "bpy.ops.file.make_paths_absolute()
>
> Make all paths to external files absolute"

> "bpy.ops.file.make_paths_relative()
>
> Make all paths to external files relative to current .blend"

Source: Python API — https://docs.blender.org/api/4.5/bpy.ops.file.html
Date read: 2026-07-30

The manual reinforces that a bulk conversion is something you must go and do, in its note on repairing broken links:

> "Every missing file found in the search will be recovered.
> Those recoveries will be done as absolute paths,
> so if you want to have relative paths you will need to select Make Paths Relative."

Source: manual — https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html
Date read: 2026-07-30

---

### Piece 3 — the "prefer relative paths" preference does not touch a path that already exists

**Label: documented**

> "Default To – Relative Paths
>
> Default value for Relative Paths when loading external files
> such as images, sounds, and linked libraries. It will be ignored if a path is already set."

Source: manual — https://docs.blender.org/manual/en/4.5/editors/preferences/save_load.html
Date read: 2026-07-30

What those words carry is narrow: this preference is a default used when an external file is loaded, and it is ignored where a path is already set. Reading that as closing off any way a preference could quietly rewrite an untouched entry on a machine set up differently is a step past them — **inferred**.

---

### Piece 4 — what form the recorded place can take, so that "unchanged" means something

**Label: documented**

> "When you specify one of these external files, the default option is to make the path relative.
> Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

> "When relative paths are supported, the File Browser provides a Relative Path checkbox,
> when entering the path into a text field, use a double slash prefix (//) to make it so."

Source: manual — https://docs.blender.org/manual/en/4.5/files/blend/open_save.html
Date read: 2026-07-30

> "External data, like texture images and other resources,
> can be stored inside the blend-file (packed) or as separate files (unpacked).
> Blender keeps track of all unpacked resources via a relative or absolute path."

Source: manual — https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html
Date read: 2026-07-30

The API documentation shows the same two-form storage on the individual settings, as a note attached to the type of each path property. For a linked library:

> "filepath
>
> Path to the library .blend file
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"

Source: Python API — https://docs.blender.org/api/4.5/bpy.types.Library.html
Date read: 2026-07-30

The identical note "(never None, blend relative // prefix supported)" appears on the path property of a great many types. Searching the whole of the 4.5 API documentation for that exact phrase returns 41 pages, 39 of which are type pages: `BakeSettings`, `BlendDataFonts`, `BlendDataImages`, `BlendDataMovieClips`, `BlendDataSounds`, `BlendDataTexts`, `Brush`, `CacheFile`, `CacheFileLayer`, `CollectionExport`, `CompositorNodeOutputFile`, `DynamicPaintSurface`, `FluidDomainSettings`, `Image`, `ImagePackedFile`, `ImageStrip`, `Library`, `LibraryWeakReference`, `MeshCacheModifier`, `MovieClip`, `MovieClipProxy`, `MovieStrip`, `MultiresModifier`, `NodeSocketStringFilePath`, `NodesModifier`, `NodesModifierBake`, `OceanModifier`, `PointCache`, `PointCacheItem`, `PreferencesFilePaths`, `RenderSettings`, `SequenceEditor`, `ShaderNodeScript`, `ShaderNodeTexIES`, `Sound`, `StripProxy`, `VectorFont`, `Volume`, `VolumeGrids`. The note gives each of them the same shape: a single string that either begins `//` or does not. Reading that list as close to the full inventory of "places a `.blend` records for outside files" is a step past what the pages say — **inferred**.

Worth noting one place that keeps a path even though nothing is loaded through it:

> "class bpy.types.LibraryWeakReference(bpy_struct)
>
> Read-only external reference to a linked data-block and its library file
>
> filepath
>
> Path to the library .blend file"

Source: Python API — https://docs.blender.org/api/4.5/bpy.types.LibraryWeakReference.html
Date read: 2026-07-30

---

### Piece 5 — template expressions stay as text

**Label: documented for the field; inferred for the saved file**

> "This substitution happens internally at time of use (e.g. while rendering), so
> the template syntax will remain as-is in the path field."

Source: manual — https://docs.blender.org/manual/en/4.5/files/file_paths.html
Date read: 2026-07-30

Which places this covers in 4.5, from the release notes:

> "Some file paths in Blender can now contain "template expressions" like
> `{blend_name}` and `{resolution_x}`, which get replaced by the values they
> reference when the path is used."

> "Currently this is only enabled on the Scene's render output path and the output
> paths of the compositor's File Output node."

Source: release notes — https://developer.blender.org/docs/release_notes/4.5/core/
Date read: 2026-07-30

The manual says the same, with an example: a render output path of `//my_render_dir/{blend_name}.png` in a file called `dance.blend` "will treat the path as" `//my_render_dir/dance.png` while rendering (manual — https://docs.blender.org/manual/en/4.5/files/file_paths.html, read 2026-07-30).

The step I took: the manual says the substitution is internal and at time of use, and that the template syntax remains in the path field. The saved file records what the field holds. So a saved file records the template text, not the expanded value. A reader has to supply the second sentence of that chain, so the claim about the *file* is **inferred**.

The same page notes the flip side — an entry can be recorded in a state that will not work: "Paths that support templates can have template errors, which prevent the path from being processed." A save does not clean those up either; nothing in the sources suggests it would.

---

### Piece 6 — packing, which the sources tie to the save itself when it was already switched on

**Label: documented**

> "Pack Resources
> …
> Mark all eligible external resource files used by the blend-file as packed.
> Actual packing will happen on the next save of the blend-file."

> "Automatically Pack Resources
> …
> When enabled, this option will ensure that all eligible external resource files, existing or added later,
> are systematically marked as packed.
> As with Pack Resources, the blend-file must be saved to the drive for this to have an effect."

Source: manual — https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html
Date read: 2026-07-30

So if the file arrived with Automatically Pack Resources already switched on, opening it and saving it packs eligible external files into the saved file — without anybody touching a path. Whether the recorded path itself survives that is **inferred**: the manual says "You know that a data is packed when you see a little "gift box" icon displayed next to its path", and the unpack options include "Use <original file path> (differs)|(identical)" and "If the original file path still exists, mark it as unpacked" (same page, read 2026-07-30). A path that can still be shown next to the gift box, and still be offered back on unpacking, is a path the file still holds. The step: the sources describe the path as present while packed, so packing adds the file's contents rather than replacing the recorded place.

Also documented on that page, and relevant to which places this can even apply to: "Some typically heavy external files, like videos from the Sequence Editor or Movie Clips, cannot be packed in a blend-file."

---

### Piece 7 — Cycles' own Python files declare no path setting of their own

**Label: observed** for what the files declare; **inferred** for anything that follows from it about a save.

I examined the Cycles add-on's own property definitions. Its scene, view-layer, object, light, material and preference settings declare no string setting with a file or directory subtype. That is the reading, and it is where the observed part stops. The step beyond it — that Cycles therefore contributes no external-file place for a `.blend` to record, and that a save has nothing of Cycles' to write back — is **inferred**. These files say what is declared in them; they do not say what a save does.

What I did and what came back: searched `properties.py` for `subtype`, and the only subtypes declared are `PIXEL`, `FACTOR` and `DISTANCE`:

```
815:        subtype='PIXEL'
822:        subtype='PIXEL'
880:        subtype='PIXEL'
973:        subtype='FACTOR'
1006:        subtype='FACTOR',
1435:        subtype='DISTANCE',
```

File: `cycles_4.5/properties.py`, at the line numbers shown.

The only three string settings in that file are not paths:

```
1549:    id: StringProperty(name="ID")
1550:    name: StringProperty(name="Name")
```

(both inside `class CyclesDeviceSettings(bpy.types.PropertyGroup):`, `cycles_4.5/properties.py` lines 1548–1552; the third `StringProperty` occurrence at line 15 is the import statement.)

File-path settings do appear in `cycles_4.5/operators.py` — for example:

```
42:    input_filepath: StringProperty(
…
46:        subtype='FILE_PATH')
48:    output_filepath: StringProperty(
…
52:        subtype='FILE_PATH')
```

File: `cycles_4.5/operators.py`, lines 42–52 and 124–140. These belong to operators (the denoising operators). That an operator's own settings are not among "the places a `.blend` records" is not something these files say — **inferred**.

Date read: 2026-07-30.

The limit of this: I checked the Cycles add-on's Python side. Cycles also has a `source` folder of files I did not exhaustively read, so "no path setting of their own" covers the Python side and not the whole add-on. The recorded places named in Piece 4 above belong to Blender's own types, which are not Cycles' files at all.

---

### The step from all of that to the answer

Nothing in the sources states the write-back rule directly. What the sources establish is that every path-rewriting mechanism in 4.5 is opt-in and separately named: Remap Relative (an option on Save As, described as being for saving in a new location, defaulting to off in the operator signature), Make Paths Relative and Make Paths Absolute (two menu entries and two no-argument operators), Find Missing Files (whose recoveries are explicitly absolute), and the relative-paths preference (explicitly ignored when a path is already set). If a plain save silently converted untouched entries, all five of those would be redundant or contradicted. So: a plain save over the same file records each untouched place in the same form it was already in.

That is a reasoned conclusion from what the documentation offers, not a sentence the documentation contains. **Label: inferred.**

### Not covered

* The sources do not describe the bytes a save writes. Everything above is about what the settings hold; whether a save could tidy a string in some way invisible in the interface — trailing separators, the direction of the slashes on Windows, upper and lower case — is not addressed anywhere I searched, and I found no page on path normalisation at all.
* "None of the places is touched" is doing a lot of work. The sources describe several things that happen on *opening* a file — library overrides that "had to be recursively resynced on load", for one (`Library.needs_liboverride_resync`, https://docs.blender.org/api/4.5/bpy.types.Library.html) — and I cannot tell from the documentation whether any load-time repair counts as touching a recorded place before you ever press save.
* The autopack case is a real exception and I have only established it from the documentation's own description of the option; I did not run it.
* It does not cover Save As or Save Copy to a different folder. Those are exactly the case Remap Relative is described for, and the answer would differ.
* It does not cover a file that has never been saved. The manual notes "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files." (manual — https://docs.blender.org/manual/en/4.5/files/blend/open_save.html, read 2026-07-30). *(Citation check, 2026-08-30 — Claude Opus 5: this quote stood uncited. I found the words on the Opening & Saving page, in the note at the end of the Relative Paths section, and supplied the address and date read. The words themselves are unchanged.)*
* It does not cover what happens to a *library's own* recorded places when the library file itself is not re-saved. Only the file being saved is in scope here.
* Nothing here says what the recorded place then resolves to at render time. That is a different question.

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

The rule holds, and if anything holds more firmly. The save operator's own signature carries `relative_remap=False`, so a save driven from a script gets no remapping unless the caller passes it. Make Paths Relative and Make Paths Absolute are reachable as operators (`bpy.ops.file.make_paths_relative()`, `bpy.ops.file.make_paths_absolute()`) and take no arguments, so a script has to call them deliberately.

Two differences from sitting at the screen. First, the Save As panel with its Remap Relative tick box is a File Browser thing; a scripted save has no panel, only the argument. Second, the command line can change a recorded place *before* a save without a person touching anything: the manual documents `-o, --render-output <path>` as "Set the render path and file name", noting "Use // at the start of the path to render relative to the blend-file" and "You can use path templating features such as {blend_name} in the path" (manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30). If a script then saves the file, the render output entry has been touched, and the question's premise no longer holds for that entry. That last step is **inferred**: the quoted words say what the argument sets, not that a save then carries it into the file.

Label for this paragraph: **documented** for the operator default and the two operators and `-o`; **inferred** for the conclusion that a headless save therefore leaves untouched entries alone, by the same step as above.

One more thing an automated setup should know, from the 4.5 release notes, because it changes what a script sees rather than what a file records:

> "`scene.render.frame_path()` used to be infallible but can now fail if there are errors processing Path Templates in the render output path."

Source: release notes — https://developer.blender.org/docs/release_notes/4.5/python_api/
Date read: 2026-07-30

And, for anyone writing their own settings that hold paths:

> "`bpy.props.StringProperty` with subtype set to `DIR_PATH` or `FILE_PATH`
> will warn when paths use a blend-file relative `//` prefix unless support is explicitly set,
> example: `options={'PATH_SUPPORTS_BLEND_RELATIVE'}`."

Source: release notes — https://developer.blender.org/docs/release_notes/4.5/python_api/
Date read: 2026-07-30

**Settled by hand test, 2026-08-30.** Every run above was driven from outside Blender's own window, with `--background`, on the Creator's Mac and on a rented Linux machine. Saving from outside behaves as the operator settings say: `bpy.ops.wm.save_mainfile()` with nothing passed does no remapping, and `bpy.ops.wm.save_as_mainfile(filepath=…)` with nothing passed does remap when the folder changes. Two things a driven run does that a person clicking does not. Giving `-o` after the file name replaces the render output place in memory, and a following plain save writes it into the file. And overwriting a file written by a newer Blender goes through silently from outside the window — it printed a warning and finished, with no confirmation to answer. See the observed section above. **Added by the third run, 2026-08-30, on the Creator's Mac.** A script reading the recorded places from outside got the same characters whichever of the two routes it used, `bpy.utils.blend_paths` or `bpy.data.file_path_map`: both reported the two backslash entries with their backslashes in the process that had just built the file, and both reported them with forward slashes in a fresh process that had opened the file from disk. On that run the reload changed the spelling and the route did not. **An earlier run of the same test recorded the two routes disagreeing there, and it has not been overwritten** — all three runs are set out in the observed section above. **Added by the Windows runs, 2026-08-30.** Those were driven from outside Blender's own window as well, with `--background` on a rented machine reporting itself as Microsoft Windows Server 2025 Standard, build 26100.32995, and the same settings behaved the same way: `bpy.ops.wm.save_mainfile()` with nothing passed did no remapping, and `bpy.ops.wm.save_as_mainfile(filepath=…)` with nothing passed remapped when the folder changed. **One thing a driven run cannot see from the answer alone**: asked to make paths relative on a file holding a place on a second drive letter, the command returned `{'FINISHED'}` having refused that place, and only the printed warning named it. On that machine both routes for listing every recorded place reported the same characters as each other, in the build run and after a reload.

### Operating system

**Label: not in the sources.** The sources do not say. Nothing on the save pages, the relative-paths section, the packed-data page or the two path-conversion operators is qualified by operating system, and I found no page describing path normalisation on any system.

Three fragments are the closest the sources come, and none of them is about saving a `.blend`:

* The FBX and OBJ exporters' "Path Mode" option says of its Relative setting: "Uses relative paths in every case (except when on a different drive on Windows)." (manual — https://docs.blender.org/manual/en/4.5/addons/import_export/scene_fbx.html and https://docs.blender.org/manual/en/4.5/files/import_export/obj.html, read 2026-07-30). That is about writing an FBX or OBJ, not about what a `.blend` records, but it is the one place the sources acknowledge that a relative path cannot always be formed on Windows.
* `bpy.path.basename` is documented as "Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility." and `bpy.path.native_pathsep` as "Replace the path separator with the systems native os.sep." (Python API — https://docs.blender.org/api/4.5/bpy.path.html, read 2026-07-30). Both exist because path separators differ between systems; neither says anything about what a save writes.
* The 4.5 bug-fix list records "Mac version blender crashes when Asset Libraries path contains BACK SLASH "\"" as fixed (release notes — https://developer.blender.org/docs/release_notes/4.5/bugfixes/, read 2026-07-30). That is an asset-library preference, not a recorded place in a `.blend`, and it says nothing about save behaviour.

I will not guess further than that.

**Settled by hand test, 2026-08-30.** The same runs were carried out on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and on a rented Linux machine (Ubuntu 22.04.4 LTS), on Blenders with identical build hashes. **What a save records was the same on both**, including the one re-spelling: a place written with a backslash comes back with a forward slash on macOS and on Linux alike, and the plain save keeps the forward-slash form. A file built on the Mac and carried to the Linux machine behaved the same way there. **One thing did differ, and it is about finding a file rather than recording its place:** a place written `//assets/TEX.PNG` against a file named `tex.png` loads its picture on macOS and does not on Linux — the recorded text is the same on both, but the file system's answer is not. **Windows is answered too, on 2026-08-30, and its answer is not the same one.** On a rented machine reporting itself as **Microsoft Windows Server 2025 Standard, build 26100.32995**, the re-spelling runs the other way: every forward slash in a recorded place becomes a backslash when the file is opened, and a plain save with nothing touched writes that into the file. Five places keep their forward slashes there — the compositor's output folder, the cloth cache's folder, the geometry-nodes bake folder, the bake picture's place and the sequencer's proxy folder — and the render output path does not. So a file carried between Windows and the Mac has its recorded text rewritten wholesale by a save that touched nothing — tested in both directions, in 4.5 and in 5.2. The crossing between Windows and Linux was not run. **That is Windows Server 2025 and no desktop edition of Windows**; the account this project rents Windows from gave a Server edition from both of its desktop-named images. See the observed section above. **The third run of the two-routes test, 2026-08-30, was the Creator's Mac alone** — the rented machine the first run used no longer exists — so what it found settles macOS and leaves every other operating system open.

---

## Record of where I looked

Searches were run across the whole of `manual_4.5`, `api_4.5` and `release_notes_4.5` at once, case-insensitively, unless stated.

**Terms searched, and what came back:**

* `remap relative` — four hits total: two in the manual's Opening & Saving page, two in the `bpy.ops.wm` page (the two save operators). That is the complete set; there is no other page discussing remapping.
* `relative path` / `relative paths` — about fifty hits. I ruled out as off-topic: the FBX, OBJ, glTF, USD and Alembic exporter pages (they concern paths written into *those* formats, not into a `.blend`), the add-on Python packaging page, Node Wrangler, the Import Images as Planes add-on, and the various per-datablock "Relative Path" checkboxes in the Image editor, Sequencer strip panels and image-format panels (these are choices made when picking a file, i.e. touching a place — the opposite of the question's premise). Kept: `files/blend/open_save`, `interface/window_system/topbar`, `editors/preferences/save_load`, `editors/preferences/file_paths`, `render/output/properties/output`, `files/linked_libraries/link_append`.
* `make_paths_relative`, `make_paths_absolute`, `path_reference`, `relative_to_blend` — found the two operators in `bpy.ops.file`, plus `bpy_extras.io_utils.path_reference`, which is an exporter helper and not save behaviour.
* `autopack`, `automatically pack`, `use_autopack`, `Pack Resources`, `Unpack Resources` — found the topbar External Data menu, the Packed Data manual page, `bpy.ops.file.autopack_toggle()` and `BlendData.use_autopack`.
* `backslash`, `forward slash`, `slashes`, `path separator`, `normalize`, `normalise` — nothing about a `.blend` save. Only the wheels page, an OBJ export option, `bpy.path.native_pathsep`, an icon name, and the Mac asset-library bug fix. This is the search that establishes silence on path normalisation.
* `different drive`, `another drive`, `other drive` — only the FBX and OBJ exporter notes quoted above.
* `paths unchanged`, `rewrite`, `rewritten`, `preserved`, `verbatim`, `as-is`, `left untouched` — nothing stating a write-back rule. The only relevant hit in the whole set was the path-templates line "the template syntax will remain as-is in the path field". This is the search that establishes the main answer must be inferred rather than quoted.
* `double slash`, `// prefix`, `blend relative` — two manual hits (the Relative Paths section, the render output panel) and the API type-note "(never None, blend relative // prefix supported)", which appears on 41 API pages, 39 of them type pages. The full list is in Piece 4 above.
* `relocate`, `Reload Library`, `lib_relocate` — the library relocate and reload operators, plus the 4.5 core release note that linked IDs can now be relocated. All of these are deliberate actions on a place, so out of scope, but I checked them to be sure none is described as happening automatically at save.
* `path` across all release-notes files — Path Templates in `core.md`, the `frame_path()` and `bpy.props` notes in `python_api.md`, the Format String node in `geometry_nodes.md`, plus unrelated bug-fix lines (motion paths, shortest-path selection). `pipeline_assets_io.md` contains no occurrence of "path" or "save" at all.

**Pages opened in full:**

* https://docs.blender.org/manual/en/4.5/files/blend/open_save.html — the whole page, opening through saving through the Relative Paths section.
* https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html — whole page.
* https://docs.blender.org/manual/en/4.5/files/file_paths.html — whole page (path templates).
* https://docs.blender.org/manual/en/4.5/files/blend/index.html — the section list, to check no save-related subsection was missed. Its top-level sections are: Opening & Saving, Compatibility, Packed Data, Blend-Files Previews, Rename. Relative Paths is not one of them; it is a subsection of Opening & Saving, alongside Opening Files, Open Recent, Recover, Saving Files, Save Incremental, Save As and Save Copy. *(Citation check, 2026-08-30 — Claude Opus 5: the line read "It lists only: Opening & Saving, Relative Paths, Compatibility, Packed Data, Blend-Files Previews, Rename", which put a subsection among the top-level sections and left the other subsections out. Corrected against the page.)*
* https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — the External Data and Clean Up part of the File menu.
* https://docs.blender.org/manual/en/4.5/editors/preferences/save_load.html — the auto-save and defaults part.
* https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html — the top of the page, for the meaning of `//` in preference paths.
* https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — the File Path setting.
* https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — the Link options and the Relocate/Reload entries.
* https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — the render-output argument.
* https://docs.blender.org/api/4.5/bpy.ops.wm.html — `save_mainfile` and `save_as_mainfile` signatures.
* https://docs.blender.org/api/4.5/bpy.ops.file.html — the whole file-operator list.
* https://docs.blender.org/api/4.5/bpy.path.html — whole page.
* https://docs.blender.org/api/4.5/bpy.types.Library.html — whole page.
* https://docs.blender.org/api/4.5/bpy.types.LibraryWeakReference.html — whole page.
* https://docs.blender.org/api/4.5/bpy.types.Image.html — the `filepath` and `filepath_raw` entries.
* https://developer.blender.org/docs/release_notes/4.5/core/ — whole page.
* https://developer.blender.org/docs/release_notes/4.5/python_api/ — the render-paths and `bpy.props` sections.
* https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/ — checked and ruled out; contains nothing about paths or saving.

**Image opened:** `_images/files_blend_open-save_file-browser-save.png`, on https://docs.blender.org/manual/en/4.5/files/blend/open_save.html — the File Browser in save configuration, showing the Save As Blender File options panel.

**Cycles files examined:** `cycles_4.5/properties.py` (searched for `subtype` and `StringProperty`), `cycles_4.5/operators.py` (searched for `FILE_PATH` and `filepath`), `cycles_4.5/osl.py` (read the top, for any recorded script path). I did not read `cycles_4.5/source` exhaustively.

**Ruled out and why:** the whole `import_export` part of the manual (it describes writing other formats, not what a `.blend` records); the asset-library preference pages (a preference on the machine, not a place recorded in a file); the Sequencer and Movie Clip strip panels (they show the Relative Path checkbox used when picking a file, which is touching a place); the add-on and extension pages (Python import paths).

---

## What a running Blender still could not settle

The list that stood here was run in full on 2026-08-30 — on the Creator's Mac, on a rented Linux machine, and on a rented Windows machine — and the results are in the observed sections above. What is left is only what those three machines could not reach:

1. **Any desktop edition of Windows.** All three Windows questions — which way the separator is re-spelled, what a plain save records for a relative place across two drives, and whether letter case in a drive letter survives a save — were answered on 2026-08-30 on a rented machine reporting itself as **Microsoft Windows Server 2025 Standard**, build 26100.32995, and the answers are in the observed section above. What is not settled is whether a desktop edition of Windows answers the same. Both of the desktop-named images tried that day booted Server editions — Server 2025 Standard from one, and **Microsoft Windows Server 2016 Standard**, build 14393.9234, from the other — read off the running systems rather than off the labels.
2. **A cache file, a movie clip and a movie strip.** Each needs a real file of its kind to import, so none could be built into the test file, and nothing has been observed about them.
3. **Why 4.5 writes an auto-save file where 5.0, 5.1 and 5.2 write only a quit file.** Three runs gave the same split. Whether that is a change in Blender or a difference in when a version decides a file has been edited is not settled.

The one list these came from is `Audits/Blender Research/Dependencies/Hand tests — Open Threads questions.md`.

---

2026-08-29 — Claude Opus 5 (Claude Code) — the blind research pass that wrote this document: Dependencies, Blender 4.5. The observed sections above were added by later sittings and carry their own dates.

*(Corrected 2026-08-30 — Claude Opus 5. The citation check of 2026-08-30 flagged that this document dates its runs 2026-08-30 while this line reads 2026-08-29, and said the two cannot both be right. They can, and both are: this line signs the blind research pass, and the runs belong to a later sitting. Which date belongs to which was established from the raw output's own timestamps, which print `2026-08-30` inside the run files under `Audits/Blender Research/Dependencies/Hand tests — Open Threads questions — raw output/Mac/`, and from this document's own signatures file, which puts the blind pass on 2026-08-29 and the hand tests on 2026-08-30. No date was wrong. What was wrong is that this line did not say what it covers, which is what made the two read as a clash, so it now says it.)*

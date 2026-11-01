# Dependencies — Open Threads questions, second pass

**Sources consulted, and when.** Blender 5.2 manual (`manual_5.2`, including its `_images` folder), Blender 5.2 Python API documentation (`api_5.2`), Blender 5.2 release notes (`release_notes_5.2`), and the Cycles add-on files supplied in `cycles_5.2`. All four were read on **2026-07-30** (the snapshot date given to me), and that is the date read for every citation below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: **question 20 only**, of the Dependencies sheet. Blender version 5.2.

---

## 20. When a `.blend` file is opened and saved again, and none of the places it records for outside files is touched while it is open, what does the saved file record for each of those places?

### The ambiguity in the question, and the reading I answered

"Saved again" can mean two different things, and the sources give different answers for each:

- **Reading A (the one I take to be meant):** the file is saved back over itself — `File ‣ Save` — so the blend-file stays at the same location.
- **Reading B:** the file is saved somewhere else — `File ‣ Save As…` or `File ‣ Save Copy…` — so the blend-file's own location changes.

I answer both, marking each. Reading A is the plain reading of "opened and saved again"; Reading B is worth stating because in it the saved file can record **different text** for an entry the user never touched, which is the exact thing the question is asking after.

---

### First: what "the places it records" are, in the documentation's own terms

**LABEL: documented**

The API documentation marks a recorded outside location with a standard phrase. Searching `api_5.2` for the exact string `blend relative // prefix supported` returns **48 hits across 41 files**. The types carrying such a property include (file names, one property each or more): `bpy.types.Library`, `bpy.types.LibraryWeakReference`, `bpy.types.Image`, `bpy.types.ImagePackedFile`, `bpy.types.Sound`, `bpy.types.VectorFont`, `bpy.types.MovieClip`, `bpy.types.MovieClipProxy`, `bpy.types.StripProxy`, `bpy.types.ImageStrip`, `bpy.types.MovieStrip`, `bpy.types.SequenceEditor`, `bpy.types.CacheFile`, `bpy.types.CacheFileLayer`, `bpy.types.Volume`, `bpy.types.VolumeGrids`, `bpy.types.RenderSettings`, `bpy.types.BakeSettings`, `bpy.types.CompositorNodeOutputFile`, `bpy.types.NodesModifier`, `bpy.types.NodesModifierBake`, `bpy.types.PointCache`, `bpy.types.PointCacheItem`, `bpy.types.MeshCacheModifier`, `bpy.types.MultiresModifier`, `bpy.types.OceanModifier`, `bpy.types.FluidDomainSettings`, `bpy.types.DynamicPaintSurface`, `bpy.types.ShaderNodeTexIES`, `bpy.types.ShaderNodeScript`, `bpy.types.CollectionExport`, `bpy.types.NodeSocketStringFilePath`, `bpy.types.NodeTreeInterfaceSocketStringFilePath`, and `bpy.types.PreferencesFilePaths`.

Two representative quotes:

> "filepath
> Path to the library .blend file (default “”, never None, blend relative // prefix supported)"

— Python API documentation, https://docs.blender.org/api/5.2/bpy.types.Library.html — read 2026-07-30

> "filepath
> Directory/name to save animations, # characters define the position and padding of frame numbers (default “//”, never None, blend relative // prefix supported, Supports template expressions)"

— Python API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30

So each recorded place is a single text value that may be written in the blend-relative `//` form or not. That matters for the rest of the answer: the question "what does the saved file record" is a question about what that text says after a save.

The manual states what the relative form means:

> "Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

— Manual, https://docs.blender.org/manual/en/5.2/files/blend/open_save.html — read 2026-07-30

---

### Reading A — saved back over itself (`File ‣ Save`)

**LABEL: inferred**

**The claim.** Each recorded place is written back holding the same text it held when the file was opened. Nothing about a plain save resolves, shortens, lengthens, repairs, or converts a recorded location: a `//`-relative entry stays `//`-relative and stays the same relative text, an absolute entry stays the same absolute text, an entry that names a file that is not there stays exactly as it was, and an entry written as a path template keeps the template unexpanded.

**The step I took, spelled out.** The sources nowhere say "a plain save leaves recorded locations unchanged." What they do say is what *changes* a recorded location, and every such mechanism is either an explicit user action or is documented as not applying to this case. I list them, then conclude by elimination. A reader has to accept "these are all of them" to accept the claim, so the label is inferred, not documented.

The pieces:

**1. The save operator's own remap option is off by default, and in any case is scoped to a different directory.**

> "bpy.ops.wm.save_mainfile(*, filepath='', … compress=False, relative_remap=False, exit=False, incremental=False, show_save_modified_images_dialog=False)"

> "relative_remap (bool) – Remap Relative, Remap relative paths when saving to a different directory (optional)"

— Python API documentation, https://docs.blender.org/api/5.2/bpy.ops.wm.html — read 2026-07-30

Two documented facts in that one entry: the default for a plain save is `False`, and even when true the option is described as acting "when saving to a different directory", which Reading A is not.

**2. The manual describes the same option the same way, and ties it to saving in a new location.**

> "Remap Relative
> This option remaps Relative Paths
> (such as linked libraries and images) when saving a file in a new location."

— Manual, https://docs.blender.org/manual/en/5.2/files/blend/open_save.html — read 2026-07-30

And, in the same page, the advice that relocating a file relative to its resources needs that explicit route:

> "If it is necessary to relocate a blend-file relative to its linked resources, use Blender’s File Save As function which has an option to Remap Relative file links."

— same page, same date

**3. Turning entries relative or absolute wholesale is a separate, explicitly invoked command, not part of saving.**

> "Make Paths Relative
> Make all paths to external files relative to the current blend-file.
> Make Paths Absolute
> Make all paths to external files absolute (= full path from the system’s root)."

— Manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html — read 2026-07-30

The API lists the same two as operators the user runs:

> "bpy.ops.file.make_paths_absolute()
> Make all paths to external files absolute"

> "bpy.ops.file.make_paths_relative()
> Make all paths to external files relative to current .blend"

— Python API documentation, https://docs.blender.org/api/5.2/bpy.ops.file.html — read 2026-07-30

**4. Repairing a missing entry is likewise an explicit command, and is documented as writing an absolute form when run.**

> "Find Missing Files
> In case you have broken links in a blend-file, this can help you to fix the problem. … Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative."

— Manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html — read 2026-07-30

The companion command only reports, it does not write:

> "Report Missing Files
> This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor’s header."

— same page, same date

Neither is described as happening on open or on save. So an entry that named a file that is not there is still recorded, still naming that file, after a plain save.

**5. The preference that decides relative-versus-absolute is documented as applying only to new entries.**

> "Default To – Relative Paths
> Default value for Relative Paths when loading external files such as images, sounds, and linked libraries. It will be ignored if a path is already set."

— Manual, https://docs.blender.org/manual/en/5.2/editors/preferences/save_load.html — read 2026-07-30

That last sentence is the strongest single piece of documented support for the claim: the setting that would otherwise decide the form of a recorded location has no say over one that is already set.

**6. Path templates are documented to stay unexpanded in the recorded text.**

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

— Manual, https://docs.blender.org/manual/en/5.2/files/file_paths.html — read 2026-07-30

This one is **documented**, not inferred, for the entries that support templates. In 5.2 that is a short list:

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor’s File Output node. More filepaths will support templates in future releases."

— same page, same date

So for those entries the saved file records the template text itself — `{blend_name}`, `{fps:###}`, doubled braces and all — not what it would expand to.

**Nothing found on the other side.** I searched for any statement that a save alters recorded locations and found none. See the record of looking below.

**What a plain save *does* do to the outside files themselves, without changing what is recorded**

Two documented side effects of saving that touch the outside world but are not described as changing the recorded text:

> "When saving a blend-file, Blender may also save modified external images depending on the Save Modified Images preference."

— Manual, https://docs.blender.org/manual/en/5.2/files/blend/open_save.html — read 2026-07-30

> "Always Save:
> Automatically saves all modified images when saving the blend-file."

— Manual, https://docs.blender.org/manual/en/5.2/editors/preferences/save_load.html — read 2026-07-30

and, for packing:

> "Pack Resources
> … Mark all eligible external resource files used by the blend-file as packed. Actual packing will happen on the next save of the blend-file."

> "Automatically Pack Resources
> When enabled, this option will ensure that all eligible external resource files, existing or added later, are systematically marked as packed. As with Pack Resources, the blend-file must be saved to the drive for this to have an effect."

— Manual, https://docs.blender.org/manual/en/5.2/files/blend/packed_data.html — read 2026-07-30

So a save is when packing is actually carried out. **LABEL: inferred** — that the recorded location survives packing: the unpack dialog later offers, by name, the location the packed data came from —

> "Use <original file path> (differs)|(identical)
> If the original file path still exists, mark it as unpacked."

> "Create <original file path>
> If the original file path does not exist, mark it as unpacked and write it to drive."

— same page, same date

— and the step is that for Blender to offer "<original file path>" after the file has been saved and reopened, that path must still be recorded in the saved file. The manual does not say this outright.

**Entries that belong to a linked library, not to this file**

**LABEL: inferred.** A recorded location sitting on a linked (not local) data-block is not the saving file's to write; it lives in the library file. The documentation distinguishes them:

> "SKIP_LINKED:
> Skip Linked.
> Skip paths of linked IDs."

— Python API documentation, https://docs.blender.org/api/5.2/bpy_types_enum_items/file_path_foreach_flag_items.html — read 2026-07-30

and resolution of such a location is against the library, not the open file:

> "library (bpy.types.Library | None) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start."

— Python API documentation, https://docs.blender.org/api/5.2/bpy.path.html — read 2026-07-30

The step: if a linked data-block's relative location is measured from its own library file rather than from the open file, then the open file is not the place that stores it, and saving the open file does not write it. The sources do not state this in so many words.

---

### Reading B — saved to a different location (`File ‣ Save As…`, `File ‣ Save Copy…`)

**LABEL: documented (for the default setting and the option's stated effect); inferred (for what the resulting text says)**

Here an untouched entry *can* come out different, because the remap option is on by default for these commands:

> "bpy.ops.wm.save_as_mainfile(*, filepath='', … compress=False, relative_remap=True, copy=False, show_save_modified_images_dialog=False)"

— Python API documentation, https://docs.blender.org/api/5.2/bpy.ops.wm.html — read 2026-07-30

The manual's screenshot of the save file browser shows the same: in the "Save As Blender File" panel at the lower left, **Compress** is ticked, **Remap Relative** is ticked, **Save Copy** is unticked.

— Manual image `_images/files_blend_open-save_file-browser-save.png`, shown on https://docs.blender.org/manual/en/5.2/files/blend/open_save.html as "The File Browser in save configuration." — viewed 2026-07-30. **LABEL: documented** for what is visible in the picture; I take from it only the tick states of those three boxes.

`Save Copy` is documented as sharing these options:

> "Save Copy
> … Choose a file path to save the blend-file to, but return to editing the original file upon completion. … For options see Save As."

— Manual, https://docs.blender.org/manual/en/5.2/files/blend/open_save.html — read 2026-07-30

What the saved file then records, entry by entry:

- **Entries recorded in the `//` relative form**, when the destination is a different directory: rewritten. Documented that they are "remap[ped]"; **inferred** that the rewritten text is a new relative text that still resolves to the same file on disk — the step being that "remap" plus the manual's definition of the relative form ("evaluated relative to the directory location of the referencing blend-file") only makes sense as keeping the target fixed while the reference point moves. The sources do not spell out the resulting string.
- **Entries recorded in absolute form:** the option is described only as remapping "Relative Paths", so absolute entries are not covered by it. **Inferred** that they are recorded unchanged.
- **Destination in the same directory** (which is what `File ‣ Save Incremental` does — "Save the current Blender file with a numerically incremented name that does not overwrite any existing files", Manual, same page): the option is scoped "when saving to a different directory", so **inferred** that nothing is rewritten.
- **If Remap Relative is switched off** and the destination is a different directory: **inferred** that relative entries keep their old text and therefore now mean a different place on disk, since the text is measured from the blend-file's directory and that directory has changed. This follows from the two documented sentences (what the relative form means; what the option does) but is not stated.

---

### Two things this does not settle, and one caution

**Not covered.**

- I cannot show what is actually written into the file's bytes. The claim above is about the text a recorded location holds, taken from what the documentation says changes it and what it says does not. Nothing in the sources describes the on-disk encoding of these values.
- Whether a save tidies a recorded location cosmetically — converting `\` to `/` or the reverse, collapsing `..` segments, dropping or adding a trailing slash, changing letter case, or trimming whitespace — is not addressed anywhere I searched. The API offers `bpy.path.native_pathsep` ("Replace the path separator with the system's native os.sep", https://docs.blender.org/api/5.2/bpy.path.html) as a thing a script may call, which shows separators can differ between systems, but nothing says a save applies it. **This specific sub-question is: not in the sources.**
- Whether opening a file written by an older Blender and saving it in 5.2 alters recorded locations as part of version conversion. The compatibility page (https://docs.blender.org/manual/en/5.2/files/blend/compatibility.html) describes conversion in general terms and says nothing about paths. **Not in the sources.**
- Whether the answer differs between kinds of recorded place (a library versus an image versus a cache directory versus a render output). Nothing I found draws such a distinction for saving; the one distinction I did find is the template one, which applies only to the render output path and the compositor File Output node's paths.
- The physics disk cache is documented as living in "a folder named blendcache_[filename] alongside the blend-file" (Manual, https://docs.blender.org/manual/en/5.2/physics/baking.html). That name is derived from the blend-file's own name rather than being a recorded location, so what a save records for it is outside what I could establish; the Cache panel's own "Path" field is a recorded location and falls under the answer above.
- I did not establish anything about what happens when the save fails partway, or about the backup files (`.blend1`, `.blend2`) the Save Versions preference produces.

**Caution on over-reading the Cycles files.** I examined the supplied Cycles add-on files for recorded locations that would fall under this question. `cycles_5.2/properties.py` defines no file- or directory-path property that is stored in the blend-file: searching that file for `PATH`, `StringProperty` and `texture_cache` gives only `FORWARD_PATH_TRACING` (an enum value, line 353), `debug_use_texture_cache_eviction` (line 1129) and `debug_texture_cache_preserve_unused` (line 1134), which are not paths, and two `StringProperty` uses for device `id` and `name` (lines 1619–1620). The path properties in `cycles_5.2/operators.py` (`input_filepath`, `output_filepath`, `subtype='FILE_PATH'`, lines 37–47 and 119–135) are operator arguments for the denoise and merge operators, which are typed at the moment of use and not stored in the file. **LABEL: observed** for those file contents. Cycles' new texture cache location is documented as a *preference*, not something the blend-file records:

> "Texture Cache
> Directory to store .tx texture cache files, relative to the source image file or as an absolute path. Used by the Cycles Texture Cache."

— Manual, https://docs.blender.org/manual/en/5.2/editors/preferences/file_paths.html — read 2026-07-30

and the release note says the same:

> "The Texture Cache panel has a button to manually generate tx files. The default location can be changed in the preferences, with either a path relative to the image file, or an absolute path."

— Release notes, https://developer.blender.org/docs/release_notes/5.2/cycles/ — read 2026-07-30

So the texture cache location is not among the places a saved `.blend` records. Note though that 5.2's path iterator can be asked to visit those cache files, and marks them as not editable:

> "is_cache
> True when the path is a cache file, like the image texture cache. These paths can not be edited."

— Python API documentation, https://docs.blender.org/api/5.2/bpy.types.BlendDataPathMeta.html — read 2026-07-30

**One supporting observation from the release notes**, which shows a recorded library location coming back in the `//` form it was written in — five libraries of a shot file, all reported relative:

> "bpy_lib.libraries(filepath='//../../../assets/scripts/update_bake_path.blend', is_archive=False)"

— Release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/ — read 2026-07-30. This is an example printed in the release note for the new `libraries.load()` input context manager, not a statement about saving; I use it only to show that the stored form is the relative text itself.

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

**LABEL: documented, in part; not in the sources, in part.**

Saving is done by the same two operators whether a person clicks or a script calls them: `bpy.ops.wm.save_mainfile()` and `bpy.ops.wm.save_as_mainfile()`, whose signatures and defaults are quoted above. Those defaults — `relative_remap=False` for save, `relative_remap=True` for save-as — are properties of the operators, so a script that calls `save_as_mainfile(filepath=…)` and says nothing about `relative_remap` gets the remapping, exactly as the file browser's ticked box does. That much is documented by the signatures.

There is no command-line argument that saves a file: the argument list at https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html has `-b/--background`, the render arguments, `-P/--python`, `--python-expr`, `--python-text` and so on, but nothing of the form `--save`. Driving Blender from outside therefore means running Python, which means running these operators.

What the sources do **not** say: whether anything about path recording on save differs in background mode. I found no statement either way. The blend-relative form itself is documented to work outside the interface —

> "-o, --render-output <path>
> Set the render path and file name.
> Use // at the start of the path to render relative to the blend-file."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30

— but that is about reading a location, not about writing one on save.

One documented wrinkle that does not survive outside the interface cleanly: the "Save Modified Images" preference offers "Ask Every Time", which "Displays a confirmation dialog when saving the blend-file" (Manual, https://docs.blender.org/manual/en/5.2/editors/preferences/save_load.html). What that setting does when there is no window to show a dialog in is **not in the sources**. It does not bear on what is recorded for a location, only on whether the image at that location is rewritten.

**Settled by hand test, 2026-08-30.** Every run above was driven from outside Blender's own window, with `--background`, on the Creator's Mac and on a rented Linux machine. Saving from outside behaves as the operator settings say: `bpy.ops.wm.save_mainfile()` with nothing passed does no remapping, and `bpy.ops.wm.save_as_mainfile(filepath=…)` with nothing passed does remap when the folder changes. Two things a driven run does that a person clicking does not. Giving `-o` after the file name replaces the render output place in memory, and a following plain save writes it into the file. And overwriting a file written by a newer Blender goes through silently from outside the window — it printed a warning and finished, with no confirmation to answer. See the observed section above. **Added by the third run, 2026-08-30, on the Creator's Mac.** A script reading the recorded places from outside got the same characters whichever of the two routes it used, `bpy.utils.blend_paths` or `bpy.data.file_path_map`: both reported the two backslash entries with their backslashes in the process that had just built the file, and both reported them with forward slashes in a fresh process that had opened the file from disk. On that run the reload changed the spelling and the route did not. **An earlier run of the same test recorded the two routes disagreeing there, and it has not been overwritten** — all three runs are set out in the observed section above. **Added by the Windows runs, 2026-08-30.** Those were driven from outside Blender's own window as well, with `--background` on a rented machine reporting itself as Microsoft Windows Server 2025 Standard, build 26100.32995, and the same settings behaved the same way: `bpy.ops.wm.save_mainfile()` with nothing passed did no remapping, and `bpy.ops.wm.save_as_mainfile(filepath=…)` with nothing passed remapped when the folder changed. **One thing a driven run cannot see from the answer alone**: asked to make paths relative on a file holding a place on a second drive letter, the command returned `{'FINISHED'}` having refused that place, and only the printed warning named it. On that machine both routes for listing every recorded place reported the same characters as each other, in the build run and after a reload.

### Operating system

**LABEL: not in the sources.**

Nothing I found ties any of this to a particular operating system. I searched `manual_5.2`, `api_5.2` and `release_notes_5.2` for `drive letter` (no hits), `UNC` (no hits in any path sense), `backslash` (hits are a Node Wrangler shortcut key, a text-replace notation in the batch-rename page, and an icon name), `path separator` (hits are the Python-wheel packaging guide, the OBJ importer/exporter option, an operator argument for hierarchy names, and `bpy.path.native_pathsep`), and `case sensitive` (hits are all name-matching options in the Outliner, selection tools, and a note that command-line arguments are case sensitive). None of these concerns what a save records.

The one thing the API acknowledges is that path separators and case differ between systems — `bpy.path.native_pathsep`, `bpy.path.resolve_ncase` ("Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path"), and `bpy.path.basename` ("Equivalent to os.path.basename, but skips a “//” prefix. Use for Windows compatibility"), all at https://docs.blender.org/api/5.2/bpy.path.html, read 2026-07-30. These are helpers a script may call. Nothing says a save calls any of them, and nothing says the recorded text is adjusted per system.

**Settled by hand test, 2026-08-30.** The same runs were carried out on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and on a rented Linux machine (Ubuntu 22.04.4 LTS), on Blenders with identical build hashes. **What a save records was the same on both**, including the one re-spelling: a place written with a backslash comes back with a forward slash on macOS and on Linux alike, and the plain save keeps the forward-slash form. A file built on the Mac and carried to the Linux machine behaved the same way there. **One thing did differ, and it is about finding a file rather than recording its place:** a place written `//assets/TEX.PNG` against a file named `tex.png` loads its picture on macOS and does not on Linux — the recorded text is the same on both, but the file system's answer is not. **Windows is answered too, on 2026-08-30, and its answer is not the same one.** On a rented machine reporting itself as **Microsoft Windows Server 2025 Standard, build 26100.32995**, the re-spelling runs the other way: every forward slash in a recorded place becomes a backslash when the file is opened, and a plain save with nothing touched writes that into the file. Five places keep their forward slashes there — the compositor's output folder, the cloth cache's folder, the geometry-nodes bake folder, the bake picture's place and the sequencer's proxy folder — and the render output path does not. So a file carried between Windows and the Mac has its recorded text rewritten wholesale by a save that touched nothing — tested in both directions, in 4.5 and in 5.2. The crossing between Windows and Linux was not run. **That is Windows Server 2025 and no desktop edition of Windows**; the account this project rents Windows from gave a Server edition from both of its desktop-named images. See the observed section above. **The third run of the two-routes test, 2026-08-30, was the Creator's Mac alone** — the rented machine the first run used no longer exists — so what it found settles macOS and leaves every other operating system open.

---

### Record of where I looked

Everything below was searched or read on 2026-07-30, within `Blender Sources/` only.

**Whole-corpus searches** (across `manual_5.2`, `api_5.2`, `release_notes_5.2`, and where noted `cycles_5.2`), with what came back:

| Term searched | Outcome |
|---|---|
| `Remap Relative` | Two hits in the manual, both in `files/blend/open_save.txt`; two in the API, both in `bpy.ops.wm.txt` (the two save operators). No others anywhere. |
| `relative_remap` | Four hits, all in `api_5.2/bpy.ops.wm.txt` — the two signatures and the two parameter descriptions. |
| `remap` (case-insensitive, whole manual) | All other hits are unrelated: colour ramps, Map Range node, NLA keyframe remapping, "Remap Users" in the Outliner and Actions, glTF channel remapping, frame-range remapping. None concerns file locations. |
| `make_paths` | `bpy.ops.file.make_paths_absolute` / `make_paths_relative` in `bpy.ops.file.txt`; the matching manual entries in `interface/window_system/topbar.txt`. |
| `Relative Path` (manual) | 18 files. I opened the ones that could bear on saving: `files/blend/open_save.txt`, `editors/preferences/save_load.txt`, `editors/preferences/file_paths.txt`, `files/linked_libraries/link_append.txt`, `files/blend/index.txt`. Ruled out the rest as being about a specific importer, editor or add-on rather than about saving: `editors/file_browser.txt`, `editors/preferences/asset_libraries.txt`, `editors/image/editing.txt`, `addons/node_wrangler.txt`, `render/output/audio/introduction.txt`, `render/output/properties/output.txt`, `modeling/meshes/import_images_as_planes.txt`, `files/import_export/alembic.txt`, `files/import_export/usd.txt`, `files/media/image_formats.txt`, and the three video-editing strip pages. |
| `absolute` (whole manual) | 40+ hits; the only path-relevant ones are in `interface/window_system/topbar.txt` (Make Paths Absolute, Find Missing Files) and `editors/preferences/file_paths.txt` (texture cache, `$filepath`). The rest are mathematical ("absolute value"), shape keys, tablet input, freestyle, colour management nits. |
| `rewrite`, `rewritten` | Four hits, all in the manual's contribute pages and version history. Nothing about paths. |
| `normalise`, `expand the path`, `resolved on save`, `stored path`, `leaves the path` | No hits. |
| `path is stored` | One hit: `bpy.types.BlendDataLibraries.txt`, the `relative` argument of `libraries.load()` — "When True the path is stored relative to the open blend file." That is about linking, not about saving. |
| `canonical` | One hit, in the manual's writing guide. Not relevant. |
| `unchanged`, `untouched`, `preserved`, `as-is`, `verbatim` (restricted to `manual_5.2/files`, `manual_5.2/editors/preferences`, `manual_5.2/interface/window_system`) | The only relevant hit is the path-template sentence in `files/file_paths.txt` quoted above. The others are Alembic custom-property export, link/append collection handling, library-override loss of reference, image-format saving, and an animation-editor time range. |
| `re-save`, `resave`, `re-saving`, `saved again`, `save the file`, `saving the file` | 15 hits. The only ones near this topic are `bpy.types.Library.needs_liboverride_resync` ("it is recommended to open and re-save that library blendfile then"), which is about override resync and not about paths, and `files/blend/compatibility.txt` on re-saving from 3.6 as a format converter. Nothing about what a save records for a location. |
| lines containing both `path` and `sav` (whole manual) | 21 hits; all already covered above, plus `files/import_export/usd.txt` on needing the blend saved before importing so a relative path can be resolved, and `advanced/command_line/render.txt` on where a rendered image is saved. |
| `blend relative // prefix supported` | 48 hits across 41 API files — used to enumerate the kinds of recorded place, listed above. |
| `drive letter`, `UNC`, `backslash`, `path separator`, `case sensitive` | Outcomes given in the Operating system section above. Nothing tying save-time recording to a system. |
| `blendcache`, `bake path`, `Bake Directory`, `custom bake` | Three manual hits: the geometry-nodes modifier Bake Path, the Bake node page, and the physics `blendcache_[filename]` folder. Plus `bake_directory` / `bake_target` in `bpy.types.NodesModifier.txt` and `bpy.types.NodesModifierBake.txt`. |
| `file_path_foreach` | `bpy.types.BlendData.txt` and `bpy.types.BlendDataPathMeta.txt`; the flag list at `bpy_types_enum_items/file_path_foreach_flag_items.txt`. |
| `path`, `texture_cache`, `StringProperty`, `FILE_PATH`, `DIR_PATH`, `filepath` (in `cycles_5.2/properties.py`, `operators.py`, `__init__.py`, `ui.py`, `version_update.py`) | Outcome given under "Caution on over-reading the Cycles files" above. |

**Pages read in full or in substantial part:**

- Manual — `files/blend/open_save.txt` (Opening, Recover, Saving, Save Incremental, Save As with its three options, Save Copy, and the whole "Relative Paths" section): https://docs.blender.org/manual/en/5.2/files/blend/open_save.html
- Manual — `files/file_paths.txt` (path templates in full): https://docs.blender.org/manual/en/5.2/files/file_paths.html
- Manual — `files/blend/packed_data.txt` (pack, auto-pack, selective pack, unpack options, pack/unpack linked libraries): https://docs.blender.org/manual/en/5.2/files/blend/packed_data.html
- Manual — `files/blend/compatibility.txt` in full: https://docs.blender.org/manual/en/5.2/files/blend/compatibility.html
- Manual — `interface/window_system/topbar.txt`, the External Data and Clean Up sections: https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html
- Manual — `editors/preferences/save_load.txt`, the Blend Files block through the Auto Run Python Scripts entry: https://docs.blender.org/manual/en/5.2/editors/preferences/save_load.html
- Manual — `editors/preferences/file_paths.txt`, Data / Render / Script Directories / Applications: https://docs.blender.org/manual/en/5.2/editors/preferences/file_paths.html
- Manual — `files/linked_libraries/link_append.txt`, the Link section and its options: https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html
- Manual — `physics/baking.txt`, the Cache panel entries: https://docs.blender.org/manual/en/5.2/physics/baking.html
- Manual — `modeling/modifiers/geometry_nodes.txt`, the Bake block: https://docs.blender.org/manual/en/5.2/modeling/modifiers/geometry_nodes.html
- Manual — `advanced/command_line/arguments.txt`, the full argument list plus the `-o/--render-output` entry: https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
- API — `bpy.ops.wm.txt`, the `save_mainfile` and `save_as_mainfile` entries in full: https://docs.blender.org/api/5.2/bpy.ops.wm.html
- API — `bpy.ops.file.txt`, the `make_paths_*` and `pack_all` entries: https://docs.blender.org/api/5.2/bpy.ops.file.html
- API — `bpy.path.txt` in full: https://docs.blender.org/api/5.2/bpy.path.html
- API — `bpy.types.Library.txt` in full: https://docs.blender.org/api/5.2/bpy.types.Library.html
- API — `bpy.types.LibraryWeakReference.txt` in full: https://docs.blender.org/api/5.2/bpy.types.LibraryWeakReference.html
- API — `bpy.types.BlendData.txt`, the `file_path_foreach` and `file_path_map` entries: https://docs.blender.org/api/5.2/bpy.types.BlendData.html
- API — `bpy.types.BlendDataPathMeta.txt` in full: https://docs.blender.org/api/5.2/bpy.types.BlendDataPathMeta.html
- API — `bpy.types.BlendDataLibraries.txt`, the `load()` parameter list: https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html
- API — `bpy.types.Image.txt` (`filepath`, `filepath_raw`), `bpy.types.RenderSettings.txt` (`filepath`), `bpy.types.NodesModifier.txt` (`bake_directory`, `bake_target`)
- API — `bpy_types_enum_items/file_path_foreach_flag_items.txt` in full: https://docs.blender.org/api/5.2/bpy_types_enum_items/file_path_foreach_flag_items.html
- Release notes — `index.md`, `core.md`, `pipeline_io.md` in full; `python_api.md` (Path Iterator and `libraries.load()` sections); `cycles.md` (texture cache line); a keyword pass over `bugfixes.md`: https://developer.blender.org/docs/release_notes/5.2/
- Cycles files — `cycles_5.2/properties.py`, `operators.py` inspected by keyword as described above; folder listing of `cycles_5.2`.

**Picture opened:** `manual_5.2/_images/files_blend_open-save_file-browser-save.png`, the "File Browser in save configuration" figure on the Opening & Saving page. Also noted but not opened: `files_blend_open-save_file-browser-open.png` (the open dialog, which has no path-recording options), `editors_preferences_section_save-load.webp` and `editors_preferences_section_file-paths.webp` (preference sections whose contents the surrounding text already spells out), and `files_file_paths-invalid_path_template.png` / `..._tooltip.png` (template errors, not save behaviour).

**Ruled out and why:**

- The whole `manual_5.2/render/shader_nodes`, `modeling/geometry_nodes/utilities` and similar node-reference trees: their "absolute", "remap" and "path" hits are mathematics and light-path terms, not file locations.
- `manual_5.2/troubleshooting/recover.txt` and the Recover entries on the Opening & Saving page: these concern recovering an unsaved session, not what a deliberate save records.
- The importer/exporter pages (`files/import_export/*`): they describe how a given format's own references are written, which is not the `.blend`'s record.
- The 5.2 release notes as a whole contain nothing about save-time path recording; the closest entries are the Path Iterator additions and the Cycles texture cache, both quoted above where relevant.

---

## What a running Blender still could not settle

The list that stood here was run in full on 2026-08-30 — on the Creator's Mac, on a rented Linux machine, and on a rented Windows machine — and the results are in the observed sections above. What is left is only what those three machines could not reach:

1. **Any desktop edition of Windows.** All three Windows questions — which way the separator is re-spelled, what a plain save records for a relative place across two drives, and whether letter case in a drive letter survives a save — were answered on 2026-08-30 on a rented machine reporting itself as **Microsoft Windows Server 2025 Standard**, build 26100.32995, and the answers are in the observed section above. What is not settled is whether a desktop edition of Windows answers the same. Both of the desktop-named images tried that day booted Server editions — Server 2025 Standard from one, and **Microsoft Windows Server 2016 Standard**, build 14393.9234, from the other — read off the running systems rather than off the labels.
2. **A cache file, a movie clip and a movie strip.** Each needs a real file of its kind to import, so none could be built into the test file, and nothing has been observed about them.
3. **Why 4.5 writes an auto-save file where 5.0, 5.1 and 5.2 write only a quit file.** Three runs gave the same split. Whether that is a change in Blender or a difference in when a version decides a file has been edited is not settled.

The one list these came from is `Audits/Blender Research/Dependencies/Hand tests — Open Threads questions.md`.

---

No helper agents were used for this pass. Every search, every page read and every quote above is my own, taken from the question sheet and from `Blender Sources/` only.

2026-08-30 — Claude Opus 5 (Claude Code) — Dependencies, Blender 5.2

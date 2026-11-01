# Dependencies — Open Threads questions, second pass

**Blender 5.0. Question 20 only.**

Sources consulted, all read on **2026-07-30** (the snapshot date given to me):
the Blender 5.0 manual (`Blender Sources/manual_5.0`, including its `_images` folder),
the Blender 5.0 Python API documentation (`Blender Sources/api_5.0`),
the Blender 5.0 release notes (`Blender Sources/release_notes_5.0`),
and Cycles' own files (`Blender Sources/cycles_5.0`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 20. When a `.blend` file is opened and saved again, and none of the places it records for outside files is touched while it is open, what does the saved file record for each of those places?

### Reading of the question

I read "saved again" as the plain **File ▸ Save** — saving over the same file, in the same place, under the same
name. That is the reading I answer first.

The question is ambiguous on one point and I flag it rather than choose silently: "saved again" could also mean
**Save As** to a different folder. That reading has a different answer, documented, and I give it below as 20(b).
A third case — a file made by an earlier Blender, opened in 5.0 and saved — is covered in 20(c).

---

### 20(a) Plain Save, over the same file

**Answer.** Each untouched entry is written back the way it already was. A place recorded relative (a path
beginning `//`) is written back relative; a place recorded as a full path is written back as a full path. Blender
does no conversion between the two forms of its own accord during a plain Save. The only thing in the whole of
these sources that converts one form to the other during a save is the **Remap Relative** option, and that option
is described as acting only when the file is being saved somewhere else; on the operator behind plain Save it is
off by default.

**LABEL: inferred.**

The step a reader has to supply, written out plainly: no page in these sources says outright "an untouched path is
written back unchanged." What the sources say is (i) that path conversion during a save is a named, optional thing
called Remap Relative, (ii) that it applies "when saving to a different directory", (iii) that on the plain-save
operator its default is off, (iv) that when Blender writes blend data with no remapping asked for, the documented
meaning of that is "No path manipulation", and (v) that the preference which decides relative-versus-absolute is
explicitly ignored for a path that already exists. From those five I conclude that nothing in a plain Save touches
an untouched entry. That conclusion is mine; the five facts under it are quoted below.

**Quotes carrying the claim.**

1. > "Save current blend-file over itself (if it was not saved yet, this will automatically switch to Save As…)."

   Manual, *Opening & Saving*. https://docs.blender.org/manual/en/5.0/files/blend/open_save.html — read 2026-07-30.
   (The Save entry has no options of its own; the Options block on that page belongs to Save As.)

2. > "Remap Relative
   >
   > This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location."

   Manual, *Opening & Saving*, under Save As ▸ Options.
   https://docs.blender.org/manual/en/5.0/files/blend/open_save.html — read 2026-07-30.

3. > "bpy.ops.wm.save_mainfile(*, filepath='', … compress=False, relative_remap=False, exit=False, incremental=False)"

   and, in the same entry's parameter list:

   > "relative_remap (boolean, (optional)) – Remap Relative, Remap relative paths when saving to a different directory"

   Python API, *bpy.ops.wm*. https://docs.blender.org/api/5.0/bpy.ops.wm.html — read 2026-07-30.
   Note the default: `relative_remap=False` for `save_mainfile`. The same parameter on
   `bpy.ops.wm.save_as_mainfile` carries the identical description but defaults to `relative_remap=True`.

4. > "path_remap (str) –
   > Optionally remap paths when writing the file:
   >
   > NONE No path manipulation (default).
   >
   > RELATIVE Remap paths that are already relative to the new location.
   >
   > RELATIVE_ALL Remap all paths to be relative to the new location.
   >
   > ABSOLUTE Make all paths absolute on writing."

   Python API, `bpy.types.BlendDataLibraries.write()`.
   https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html — read 2026-07-30.
   This is the one place in the sources that spells out what "no remapping" means when blend data is written:
   *no path manipulation*. It describes `libraries.write()` (writing chosen data-blocks out to a file), not the
   main Save, so carrying it across to the main Save is part of my inference and not a documented fact.

5. > "Default To – Relative Paths
   >
   > Default value for Relative Paths when loading external files such as images, sounds, and linked libraries.
   > It will be ignored if a path is already set."

   Manual, *Preferences ▸ Save & Load*.
   https://docs.blender.org/manual/en/5.0/editors/preferences/save_load.html — read 2026-07-30.

   The API says the same of the same preference:

   > "use_relative_paths
   >
   > Default relative path option for the file selector, when no path is defined yet
   >
   > Type: boolean, default True"

   https://docs.blender.org/api/5.0/bpy.types.PreferencesFilePaths.html — read 2026-07-30.

6. Supporting, on the form the entry keeps in memory between opening and saving — Blender does not expand `//`
   internally on load; scripts have to expand it themselves at the moment of use:

   > "Blender's relative file paths are not compatible with standard Python modules such as sys and os.
   > Built-in Python functions don't understand Blender's // prefix which denotes the blend-file path."

   and

   > ">>> bpy.path.abspath(image.filepath)"

   Python API, *File Paths & String Encoding* (Gotchas).
   https://docs.blender.org/api/5.0/info_gotchas_file_paths_and_encoding.html — read 2026-07-30.

7. Supporting — **observed**, in Cycles' own files. Cycles resolves a recorded script path only at the moment it
   uses it, which is consistent with the recorded value still being in `//` form:

   > `    script_path = bpy.path.abspath(filepath, library=library)`

   File `Blender Sources/cycles_5.0/osl.py`, line 197, inside `def update_external_script(report, filepath, library)`.
   The same pattern at line 237: `    osl_path = bpy.path.abspath(script.filepath, library=script.library)`.
   Read 2026-07-30. What the file *says* is that Cycles calls the expand-to-absolute helper at use time. That the
   stored value is therefore still relative is a step beyond the file — **inferred**, and only weakly, since a path
   already absolute would pass through the same call unharmed.

8. On one particular kind of recorded place — a render output path containing a `{…}` template — the manual states
   the recorded text is not replaced by the value:

   > "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will
   > remain as-is in the path field."

   Manual, *File Paths*. https://docs.blender.org/manual/en/5.0/files/file_paths.html — read 2026-07-30.
   This says "in the path field", i.e. what stays on screen. That the same unsubstituted text is what a save
   writes to the file is one further step — **inferred**.

**The one thing that does happen on a plain Save without anyone touching a path.** If resources are marked for
packing, the save is when the packing actually occurs:

> "Mark all eligible external resource files used by the blend-file as packed. Actual packing will happen on the
> next save of the blend-file."

and, for the automatic form:

> "When enabled, this option will ensure that all eligible external resource files, existing or added later, are
> systematically marked as packed. As with Pack Resources, the blend-file must be saved to the drive for this to
> have an effect."

Manual, *Packed Data*. https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html — read 2026-07-30.

The sources do not say that packing changes the recorded location. What they say points the other way — the
recorded location is still there beside the packed data:

> "You know that a data is packed when you see a little 'gift box' icon displayed next to its path."

(same page), and for libraries:

> "Store all data-blocks linked from other .blend files in the current .blend file. Library references are
> preserved so the linked data-blocks can be unpacked again"

Python API, `bpy.ops.file.pack_libraries()`. https://docs.blender.org/api/5.0/bpy.ops.file.html — read 2026-07-30.
The unpack options — "Use <original file path> (differs)|(identical)" on the *Packed Data* manual page — only make
sense if the original recorded location survived the packing. Reading them that way is **inferred**.

**Not covered by 20(a).**
- No source states, in so many words, what a saved file holds for an untouched entry. The claim is built from
  what the optional remapping does and does not do. If some part of the save path rewrites entries silently — say,
  tidying separators, collapsing `..` segments, changing letter case, trimming or adding a trailing slash, or
  storing an extra resolved copy alongside the recorded text — these sources say nothing about it either way.
- Nothing here settles whether the bytes written are identical to the bytes read. "Unchanged in meaning" is as
  far as the sources reach.
- Nothing here covers what happens to an entry whose target was missing at open time. The sources describe repair
  as a separate, deliberate act (Find Missing Files, Report Missing Files), not something a save performs.
- Auto-save (the timed temporary save) and Save Copy are not addressed; I only checked the ordinary Save.
- One 5.0 change alters the saved file without touching any recorded place: > "Blendfile Compression on saving is
  now enabled by default (blender/blender!146172, blender/blender@00dbb53131). Users who still want to write
  uncompressed blendfiles by default will have to edit and save again their user preferences."
  Release notes, *Core*. https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30. This
  concerns how the file is written, not what it records for outside files; I note it only so nobody mistakes a
  changed file size for a changed path.

---

### 20(b) The other reading — Save As, to a different folder

**Answer.** Saving to a new location with **Remap Relative** on rewrites the relative entries so they still point
at the same files from the new location. Entries recorded as full paths are not part of what the option names.
The option is on by default in the Save As dialog and in the `save_as_mainfile` operator.

**LABEL: documented** for the option's existence, its meaning and its default. **inferred** for "full paths are
untouched", which I take from the option being called *Remap Relative* and described as remapping *relative* paths
— the sources never state what it does to an absolute one.

**Quotes.** As item 2 and item 3 above. Additionally, the manual's own screenshot of the Save As dialog:

[image: _images/files_blend_open-save_file-browser-save.png] — captioned "The File Browser in save configuration."
on https://docs.blender.org/manual/en/5.0/files/blend/open_save.html, read 2026-07-30. I opened the picture. Under
a panel headed "Save As Blender File" it shows three checkboxes: **Compress** (ticked), **Remap Relative**
(ticked), **Save Copy** (unticked). **documented**, being part of the manual, and limited to what is visible: it
shows a tick, nothing more. A caution on how much weight this picture carries — the same picture shows Compress
ticked while, separately, I opened the manual's Preferences ▸ Save & Load screenshot
(`_images/editors_preferences_section_save-load.webp`) which shows "Default To – Compress File" *unticked*, and
the 5.0 release note quoted above says compression on saving is now on by default. The pictures in the manual are
not all of one date. I use this one only for the presence and tick-state of Remap Relative in the dialog, which
agrees with the operator default `relative_remap=True` quoted from the API.

And the manual's own advice, which is the same fact from the other side:

> "If it is necessary to relocate a blend-file relative to its linked resources, use Blender's File Save As
> function which has an option to Remap Relative file links."

Manual, *Opening & Saving* ▸ Relative Paths. Same address, read 2026-07-30.

**Not covered by 20(b).** What the option does to an entry recorded as a full path. What it does when the new
location is on a different disk or a different machine, so that no relative path can reach the target. Whether
Save As *into the same folder* under a new name (including Save Incremental, `incremental=True` on
`save_mainfile`) changes anything — the wording is "a different directory", and the same folder is not a different
one, but the sources do not address the same-folder-new-name case directly.

---

### 20(c) A file from an earlier Blender, opened in 5.0 and saved

**Answer.** For at least one kind of recorded writing place, 5.0 does not store it the way earlier files stored
it, so a save cannot write it back in the old shape. The compositor's File Output node no longer has a single
`base_path`; it has a directory and a file name.

**LABEL: documented** for the change. **not in the sources** for what value the two new fields end up holding when
an old file's single recorded path is brought forward.

**Quotes.**

> "File Output node now takes `directory` and `file_name` as two different inputs
>   - Removed:
>     - `bpy.types.CompositorNodeOutputFile.file_slots`
>     - `bpy.types.CompositorNodeOutputFile.layer_slots`
>     - `bpy.types.CompositorNodeOutputFile.base_path`
>   - Added:
>     - `bpy.types.CompositorNodeOutputFile.directory`
>     - `bpy.types.CompositorNodeOutputFile.file_name`
>     - `bpy.types.CompositorNodeOutputFile.file_output_items`"

Release notes, *Python API*. https://developer.blender.org/docs/release_notes/5.0/python_api/ — read 2026-07-30.

> "The **File Output** node was redesigned for a better user experience. (blender/blender@68dc278fe5)
>   - It now supports dragging links to create a new input.
>   - The final paths of the images are displayed in the UI to remove guess work.
>   - The base path is now split into a Directory and a File Name.
>   - Socket types are not set on the node and do not change.
>   - Other UI improvements."

Release notes, *Compositor*. https://developer.blender.org/docs/release_notes/5.0/compositor/ — read 2026-07-30.

The compositor release notes end with "See migration notes for more details
about transitioning from 4.5 to 5.0." **That migration page is not in the snapshot I was given** — the release
notes folder holds only the top-level subject files, no `migration` folder. So I cannot say how an old `base_path`
is divided between the new `directory` and `file_name` on load, and I have not guessed. This is the one place
where I would need something I do not have.

General backward-compatibility statement, for what it is worth:

> "Opening older files and converting them for the current version of Blender is usually straight-forward.
> It is expected to give very good and usable results."

Manual, *Compatibility*. https://docs.blender.org/manual/en/5.0/files/blend/compatibility.html — read 2026-07-30.
That says nothing specific about recorded locations.

**Not covered by 20(c).** Every other kind of recorded place. Whether any other 5.0 conversion rewrites a recorded
location on load. The contents of the migration page.

---

### Where I looked (for question 20, all three readings)

Searched, whole-folder, across `manual_5.0`, `api_5.0` and `release_notes_5.0` unless noted:

- `Remap Relative` — 1 file in the manual (`files/blend/open_save.txt`), 2 hits in the API (`bpy.ops.wm.txt`).
- `relative_remap` — 0 hits in the manual; 4 in the API, all in `bpy.ops.wm.txt`.
- `remap` combined with `path` — 8 hits total; this is what turned up `BlendDataLibraries.write()`'s `path_remap`.
- `Relative Paths` — 12 hits in the manual; I opened every one
  (`files/blend/open_save.txt`, `files/blend/index.txt`, `editors/preferences/save_load.txt`,
  `editors/preferences/file_paths.txt`, `editors/file_browser.txt`, `editors/image/editing.txt`,
  `render/output/properties/output.txt`, `modeling/meshes/import_images_as_planes.txt`,
  `files/media/image_formats.txt`, `files/import_export/usd.txt`).
- `Make Paths` / `make_paths` / `Report Missing Files` / `Find Missing Files` — manual hits only in
  `interface/window_system/topbar.txt`; API hits in `bpy.ops.file.txt`. Ruled out as answers because all four are
  deliberate acts by a person or a script, i.e. the paths *are* touched — which the question excludes.
- `when saving` / `on save` / `upon saving` / `when the file is saved` / `next save` — 17 hits in the manual;
  read all. The only path-related ones are the Remap Relative line and the two "Actual packing will happen on the
  next save" lines. The rest concern previews, backups, fake users, unused data, compression, colour management
  on image save.
- `unchanged`, `verbatim`, `as-is`, `rewritten`, `normalized`, `untouched`, `preserved` — across all three
  sources. `rewritten` and `normalized` gave nothing path-related. `as-is` gave the path-template line.
  `preserved` gave the `pack_libraries` line. The rest were unrelated (particles, overrides, glTF, UV maps).
- `stored path` / `path is stored` / `paths are stored` / `stores a partial path` / `stored relative` /
  `stored as absolute` — 2 hits: the manual's "Blender stores a partial path…" line and
  `BlendDataLibraries.load()`'s "When True the path is stored relative to the open blend file."
- `absolute_path` / `abspath` / `relpath` in the API — `bpy.path.abspath`, `bpy.path.relpath`,
  `bpy.path.native_pathsep`, `bpy.path.resolve_ncase`, the Gotchas page, and `bpy_extras.image_utils`. All are
  helpers a script calls; none is described as something a save performs. Ruled out for that reason —
  `native_pathsep` in particular is tempting and I did **not** treat it as evidence that saving tidies separators;
  it is documented only as a function a script may call.
- `path` across the release notes' `core.md`, `pipeline_io.md`, `python_api.md`, `index.md` — the only path items
  are the new template variables, the File Output node split, and `bpy.data.file_path_foreach()`.
- `relative` / `absolute path` / `filepath` / `remap` / `save` across the whole release-notes folder — nothing
  about how a save treats existing recorded locations. `corrective_releases.md` mentions a Node Wrangler add-on
  bug with relative paths and `bugfixes.md` mentions `Temp_data().images.load` failing on relative paths; neither
  concerns saving.
- `blend relative // prefix supported` in the API — 46 hits across 40 files. This is the marker on every property
  that records an outside place (checked among others: `RenderSettings.filepath`, `Image.filepath`,
  `Image.filepath_raw`, `Library.filepath`, `Sound.filepath`, `MovieClip.filepath`, `VectorFont.filepath`,
  `CacheFile.filepath`, `PointCache.filepath`, `FluidDomainSettings.cache_directory`). Each is a plain string
  property; none of their descriptions mentions being altered at save time.
- Manual pages opened whole and ruled out: `files/blend/open_save.txt` (the answer's home),
  `files/file_paths.txt`, `files/blend/packed_data.txt`, `files/blend/compatibility.txt`,
  `files/linked_libraries/link_append.txt`, `editors/preferences/save_load.txt`,
  `editors/preferences/file_paths.txt`, `render/output/properties/output.txt`,
  `physics/fluid/type/domain/cache.txt` (checked in case a cache location is regenerated on load or save — the
  page describes the Cache Directory and says nothing about either), `advanced/command_line/arguments.txt` and
  `advanced/command_line/render.txt`.
- Pictures opened: `_images/files_blend_open-save_file-browser-save.png` and
  `_images/editors_preferences_section_save-load.webp`.
- Cycles' own files: searched `cycles_5.0` for `filepath`, `FILE_PATH`, `DIR_PATH`. Cycles registers no
  path-recording setting of its own in `properties.py`; the hits are in `osl.py` (resolving a script path at use
  time) and `operators.py` (a denoising operator's own input and output path arguments, which are operator
  arguments and not recorded in the file). Nothing in Cycles' files concerns saving the blend-file.

**What I could not check.** The 5.0 migration pages referenced from the release notes
(`migration/compositor_migration.md`, and the theme migration page named in `index.md`) are not present in this
snapshot of the release-notes folder. I did not go outside the given folders to find them.

---

## 20 — observed. What a running Blender does

**Label: observed.** Everything under this heading was got by running Blender 5.0 and reading what it printed and what it wrote to disk. It is not a reading of any documentation, and it does not replace the inferred answer above; where the two disagree, say so plainly rather than quietly preferring one.

### On what kind of computer, and when

On 2026-08-30 these were run twice over: once on the Creator's own computer — a MacBook Pro with an Apple M1 Pro chip running macOS 26.6.2, with Apple's own graphics — and once on a rented Linux machine — Ubuntu 22.04.4 LTS with one NVIDIA RTX 3060, twelve cores and 31 GB of memory, in Poland, driven over SSH, rented and destroyed the same day. The build hash of every Blender used on the rented machine is the same as the one on the Mac, so where the two disagree the difference is the operating system and not the build. Two results are reported wherever both machines ran a test, and they are never folded into one sentence.

### What was done, and to what

A `.blend` file was built by script in Blender 5.0, so that the exact text of every place it records for an outside file was known rather than typed. Beside it sat `assets/tex.png`, `assets/two words.png`, `assets/beep.wav`, `assets/testfont.ttf`, `assets/test.osl`, `assets/test.ies`, a folder named `Тестовая папка` holding `текстура_日本語.png`, a second blend-file `lib.blend`, and one folder up an `outer.png`.

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

The two files on disk were never the same size and never the same file: 571,001 bytes before the plain save and 567,685 after, with different checksums. **So comparing whole files says nothing; only the path text can be compared.**

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

So which way a check asks for a recorded place decides what string it gets back. Blender 5.0 has `bpy.utils.blend_paths` and `bpy.data.file_path_map`, and `bpy.data.file_path_foreach` is there:

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
VERSION 5.0.1 HASH a3db93c5b259
PLATFORM darwin
```

The starting file was built fresh, and both questions were asked twice over: once **inside the process that had just built the file and had never reloaded it**, and once in a **fresh process that opened the file from disk**. The two commands in full, after the application's own path: `--background --factory-startup --python build_main.py -- <project folder> <scripts folder>`, then `--background --factory-startup <the file> --python read_only.py -- <scripts folder>`. The two routes were compared character for character by script rather than read by eye.

*What came back inside the process that built the file, before any reload:*

```
5.0  rel-backslash file_path_map -> ['//assets\\tex.png']
5.0  raw-setter    file_path_map -> ['//assets\\..\\assets//TEX.PNG ']
5.0  blend_paths entries holding a backslash -> ['//assets\\..\\assets//TEX.PNG ', '//assets\\tex.png']
```

*And in the fresh process that had opened the file from disk:*

```
5.0  rel-backslash file_path_map -> ['//assets/tex.png']
5.0  raw-setter    file_path_map -> ['//assets/../assets//TEX.PNG ']
5.0  blend_paths entries holding a backslash -> []
```

So on this machine the two routes reported those two entries the same way as each other, and what changed the spelling was the reload rather than the route.

*One thing that is not the disagreement described above, and is recorded rather than folded into it.* The two routes do not return the same **collection** of strings. Compared as multisets in the build process:

```
5.0 : blend_paths 24 strings, file_path_map 27 strings
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

**Label: observed.** Everything under this heading was got by running Blender 5.0.1 on a rented Windows machine on 2026-08-30, and reading what it printed and what it wrote to disk. It is not a reading of any documentation. It answers the Windows half of this question, which every run before it left open, and it replaces nothing above: where a Windows result differs from the Mac's or the rented Linux machine's, both stand and the difference is named.

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

**The Blender was the official Windows x64 build from Blender's own download server, and its hash is the same as the Creator's Mac's and the rented Linux machine's.** Asked of the running program itself, not read off a download page (`Windows/5.0-00-machine.txt`; the same four hashes are in `Windows/01-builds.txt` and `Windows/00-the-machine.txt`):

```
RESOURCES C:\q20\res-5.0
VERSION 5.0.1 HASH a3db93c5b259
BUILD PLATFORM b'Windows'
BUILD TYPE b'Release'
BINARY C:\blender\5.0.1\blender.exe
PLATFORM win32
OS SEP '\\' ALTSEP '/'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:32:30)
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
C:\blender\5.0.1\blender.exe --background --factory-startup --python C:\q20\scripts\build_main.py -- C:\q20\work\5.0\proj C:\q20\scripts
C:\blender\5.0.1\blender.exe --background --factory-startup C:\q20\work\5.0\proj\main.blend --python C:\q20\scripts\read_only.py -- C:\q20\scripts
C:\blender\5.0.1\blender.exe --background --factory-startup C:\q20\work\5.0\proj\main.blend --python C:\q20\scripts\plain_save.py -- C:\q20\scripts
```

Six more runs followed from the same starting file, put back fresh each time: a save into another folder on the same drive with nothing passed for the remap setting, the same with the setting turned off, a save into the same folder under a new name, the make-paths-absolute command, the make-paths-relative command, and a save onto the second drive.

#### What came back — setting a place changes nothing, whichever way it is spelled

Every one of the twenty spellings came back out of the setting exactly as it went in, in memory, before anything was saved (`Windows/5.0-01-build.txt`):

```
SET	rel-plain	wanted='//assets/tex.png'	got_filepath='//assets/tex.png'	got_raw='//assets/tex.png'
SET	rel-backslash	wanted='//assets\\tex.png'	got_filepath='//assets\\tex.png'	got_raw='//assets\\tex.png'
SET	abs-forward	wanted='C:/q20/work/5.0/proj/assets/tex.png'	got_filepath='C:/q20/work/5.0/proj/assets/tex.png'	got_raw='C:/q20/work/5.0/proj/assets/tex.png'
SET	abs-lower-drive	wanted='c:\\q20\\work\\5.0\\proj\\assets\\tex.png'	got_filepath='c:\\q20\\work\\5.0\\proj\\assets\\tex.png'	got_raw='c:\\q20\\work\\5.0\\proj\\assets\\tex.png'
SET	drive-relative	wanted='C:q20\\work\\5.0\\proj\\assets\\tex.png'	got_filepath='C:q20\\work\\5.0\\proj\\assets\\tex.png'	got_raw='C:q20\\work\\5.0\\proj\\assets\\tex.png'
SET	other-drive	wanted='X:\\assets\\tex.png'	got_filepath='X:\\assets\\tex.png'	got_raw='X:\\assets\\tex.png'
```

#### What came back — opening the file re-spells the separator, and on Windows it runs the other way

**On the Mac and on the rented Linux machine a backslash becomes a forward slash. On Windows Server 2025 every forward slash becomes a backslash.** These are the same places, read on the very next open with nothing touched in between (`Windows/5.0-02-read-before.txt`):

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
image[abs-forward].filepath	'C:\\q20\\work\\5.0\\proj\\assets\\tex.png'
image[raw-setter].filepath	'//assets\\..\\assets\\\\TEX.PNG '
scene.render.filepath	'//out\\{blend_name}_####.png'
```

Against what the same places held in memory when the file was written, which is quoted in full in the raw output: `'//assets/tex.png'`, `'//assets/../assets/tex.png'`, `'//assets//tex.png'`, `'//assets/TEX.PNG'`, `'//assets/tex.png '`, `'//assets/two words.png'`, `'//Тестовая папка/текстура_日本語.png'`, `'//../outer.png'`, `'//nowhere/missing.png'`, `'C:/q20/work/5.0/proj/assets/tex.png'`, `'//assets\\..\\assets//TEX.PNG '` and `'//out/{blend_name}_####.png'`.

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

The two files on disk are never the same file (`Windows/5.0-05-bytes.txt`):

```
--- size on disk ---
      613497  C:\q20\work\5.0\main_after_build.blend
      610181  C:\q20\work\5.0\main_after_plain_save.blend

--- byte for byte ---
C:\q20\work\5.0\main_after_build.blend C:\q20\work\5.0\main_after_plain_save.blend differ: char 1484, line 1

--- sha256 ---
137cbbd1175239076090b0d0dbfb57e6c73d9caa9288abc3e9191b515402c309  C:\q20\work\5.0\main_after_build.blend
531808c3873a2987b852f09a037aded89384012ad21d5adf603067c942c93814  C:\q20\work\5.0\main_after_plain_save.blend
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
      1 C:/q20/work/5.0/proj/assets/tex.png
      1 C:\Users\Administrator\Documents
      3 C:\q20\work\5.0\proj\assets\tex.png
      1 C:q20\work\5.0\proj\assets\tex.png
      1 X:\assets\tex.png
      1 c:\q20\work\5.0\proj\assets\tex.png
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
      4 C:\q20\work\5.0\proj\assets\tex.png
      1 C:q20\work\5.0\proj\assets\tex.png
      1 X:\assets\tex.png
      1 c:\q20\work\5.0\proj\assets\tex.png
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

and after it, one — the weak reference gone with it (`Windows/5.0-04-read-after.txt`). The unused image shielded with a fake user survived, and the third, unused and unshielded, never reached the first read because the build's own save had already dropped it.

**Why the two reads look the same although the file changed.** The re-spelling is already done by the time anything can read the file at all, so the read before the plain save and the read after it agree; the change is only visible in the bytes. **A check that reads a place through Blender and compares it against the text in the file will match on a file Windows has already saved, and will miss on a file that arrived from anywhere else.**

#### What came back — the two ways of listing every recorded place agree with each other on Windows

The Mac's first run recorded these two routes disagreeing about spelling in the process that had just built the file. **On Windows Server 2025 they do not disagree, in either state.** In the build run, before any reload, both report the backslash entry with its backslash and the plain entry with its forward slash; after the file has been opened from disk, both report every entry with backslashes (`Windows/5.0-01-build.txt` and `Windows/5.0-02-read-before.txt`):

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
5.0 : blend_paths 32 strings, file_path_map 35 strings
     only file_path_map: '//assets/tex.png'
     only file_path_map: '//lib.blend'
     only file_path_map: '//lib.blend'

after the file was opened from disk
5.0 : blend_paths 30 strings, file_path_map 34 strings
     only file_path_map: '//assets\\tex.png'
     only file_path_map: '//lib.blend'
     only file_path_map: '//lib.blend'
     only file_path_map: '//lib.blend'
```

The extras are the linked library's own inner picture, which the second route reaches through the library and the first does not, and repeated mentions of the library itself. The one string that looks like a spelling difference is not one: it is `'//assets/tex.png'` before the reload and `'//assets\\tex.png'` after it, which is the same entry following the same re-spelling as everything else.

**So the disagreement the Mac's first run recorded is not a Windows disagreement**, and that is a clean negative rather than a silence: a check may read a recorded place through either route on this machine and get the same characters. **It does not make the underlying danger smaller — it moves it.** On Windows both routes agree with each other and both disagree with the file that arrived, whenever that file was written anywhere but Windows.

#### What came back — a relative place across two drives

**A plain save leaves it exactly alone.** `X:\assets\tex.png` was stored, read back and written again unchanged, and the picture loaded from it — the read prints `image[other-drive].size	(2, 2)`.

**The make-paths-relative command names it, refuses it, and reports itself finished** (`Windows/5.0-10-make-relative.txt`):

```
CALL bpy.ops.file.make_paths_relative()
Warning: Path 'X:\assets\tex.png' cannot be made relative for Image 'other-drive'
Warning: Total files 30 | Changed 6 | Failed 1
returned {'FINISHED'}
```

It counts the failure and says which place failed and which data-block it belonged to, and it still returns the same answer it returns when everything worked. **A caller that reads only the answer will not know one place was left behind.** The place itself is untouched and the save that follows writes it back as it was.

**Saving the whole file onto the other drive turns every relative place into a full path naming the old drive, and says so nowhere** (`Windows/5.0-11-saveas-other-drive.txt`):

```
CALL bpy.ops.wm.save_as_mainfile(filepath='X:\\elsewhere\\onotherdrive.blend')  [relative_remap not passed]
returned {'FINISHED'}

image[rel-plain].filepath	'C:\\q20\\work\\5.0\\proj\\assets\\tex.png'
image[nonlatin].filepath	'C:\\q20\\work\\5.0\\proj\\Тестовая папка\\текстура_日本語.png'
image[missing].filepath	'C:\\q20\\work\\5.0\\proj\\nowhere\\missing.png'
library[lib.blend].filepath	'C:\\q20\\work\\5.0\\proj\\lib.blend'
```

where the same call into a different folder on the *same* drive keeps them relative. The one place already on the new drive, `X:\assets\tex.png`, is the only one left as it was. No warning was printed.

#### What came back — letter case in a drive letter

**A plain save does not change it.** `c:\q20\work\5.0\proj\assets\tex.png` was stored, read back and written again with its lower-case letter, and the upper-case one likewise. **Windows finds the file either way** — both print `size	(2, 2)` — which is the same as the Mac and unlike Linux, where the wrong letter case in a file name stops the picture loading.

**Making paths absolute does not change it either** (`Windows/5.0-09-make-absolute.txt`):

```
image[abs-lower-drive].filepath	'c:\\q20\\work\\5.0\\proj\\assets\\tex.png'
```

**Making paths relative removes the drive letter and with it the question.** The upper-case and the lower-case place collapse to the same text:

```
image[abs-lower-drive].filepath	'//assets\\tex.png'
image[abs-upper-drive].filepath	'//assets\\tex.png'
```

**A drive letter with no root behind it is mangled.** A place written as a drive letter followed straight by a name is treated by the make-paths-relative command as an ordinary relative name and given steps back up in front of it:

```
image[drive-relative].filepath	'//..\\..\\..\\..\\C:q20\\work\\5.0\\proj\\assets\\tex.png'
```

The result cannot name any file on any machine. It is recorded as what happened and is not called a fault here, because whether Blender means to support that spelling at all is not something this test can settle. The place did not load its picture before the command either — it prints `size	(0, 0)` — so nothing that was working stopped working. The make-paths-absolute command left the same place untouched.

#### What came back — saving somewhere else on the same drive

**The same shape as the Mac and Linux, written the Windows way.** With nothing passed for the remap setting, every relative place a file is read from is rewritten, lengthened to reach back to where the assets are, and spelled with backslashes (`Windows/5.0-06-saveas-default.txt`):

```
CALL bpy.ops.wm.save_as_mainfile(filepath='C:\\q20\\elsewhere\\5.0\\moved_default.blend')  [relative_remap not passed]
returned {'FINISHED'}

image[rel-plain].filepath	'//..\\..\\work\\5.0\\proj\\assets\\tex.png'
image[rel-dotdot].filepath	'//..\\..\\work\\5.0\\proj\\assets\\tex.png'
image[rel-doubleslash].filepath	'//..\\..\\work\\5.0\\proj\\assets\\tex.png'
image[case-caps].filepath	'//..\\..\\work\\5.0\\proj\\assets\\TEX.PNG'
image[trailing-space].filepath	'//..\\..\\work\\5.0\\proj\\assets\\tex.png '
library[lib.blend].filepath	'//..\\..\\work\\5.0\\proj\\lib.blend'
```

The doubled separator is collapsed, the step back up resolved, the letter case kept, the trailing space kept, a place pointing at nothing rewritten just the same, and the full paths not touched. **With the setting turned off nothing changed at all**, and **saving into the same folder under a new name changed nothing either** — every recorded place came back identical in both (`Windows/5.0-07-saveas-remap-off.txt`, `Windows/5.0-08-saveas-same-folder.txt`). So the boundary is a different folder, exactly as it is on the Mac.

#### The crossing between Windows and the Mac was not run in this version

The crossing — a Windows-saved file carried to the Mac, opened, saved with nothing touched and compared byte for byte, and then the same in reverse — was run in 4.5 and in 5.2 only, as the two ends of the range, and both behaved the same way. **It was not run in 5.0, so this version's silence on it is an untested question rather than a difference in Blender**, and the machine that could answer it is destroyed.

#### What these Windows runs do not cover

* **Any desktop edition of Windows.** Every result here is Windows Server 2025 Standard, build 26100.32995. Both of the desktop-named images tried on 2026-08-30 booted Server editions — Server 2025 from one, Server 2016 from the other — and that is established from the running systems rather than from the labels. Nothing here shows whether a desktop edition behaves the same.
* **Why five places keep their forward slashes and the render output path does not.** The five are named above and behave the same way in every version tested, but nothing here says what distinguishes them from the render output path, which is also a place written to.
* **What a drive letter with no root is meant to do.** The make-paths-relative command makes text from it that names no file. Whether that spelling is supported at all is not settled here.
* **A place on a network share**, written the way Windows writes one. The second drive here is a drive letter mapped onto a folder, which answers the cross-drive question and not the share question.
* **A cache file, a movie clip and a movie strip**, for the same reason they said nothing on the Mac: each needs a real file of its kind to import, so none could be built into the starting file.

Every line this machine printed is in `Audits/Blender Research/Dependencies/Hand tests — Windows — raw output/`, and the list that ran it is `Audits/Blender Research/Dependencies/Hand tests — Windows.md`.

### Outside the interface

Does this hold when Blender is driven from outside its own window?

Partly documented, partly not.

The operator behind Save is the same one a script calls, and its default is stated in the API rather than the
manual: `bpy.ops.wm.save_mainfile(… relative_remap=False …)`. So a script that saves without asking for anything
gets no remapping, in a window or without one. That much is **documented** (the default) and the rest —
that the same code runs with no window — is **inferred**.

Two differences I can point at, both of them limited:

- A script or another program can pass `relative_remap=True` to `save_mainfile`, which the Save menu item gives no
  way to do. So a save driven from outside can rewrite relative entries where the same command inside cannot.
  **documented**, from the parameter's presence on `save_mainfile`.
  https://docs.blender.org/api/5.0/bpy.ops.wm.html — read 2026-07-30.
- The manual describes one save-time prompt: for a file saved by a newer Blender, > "Trying to overwrite it (with
  a simple 'Save' operation) will also show a confirmation popup, as this could make that loss of data permanent."
  Manual, *Compatibility*. https://docs.blender.org/manual/en/5.0/files/blend/compatibility.html — read
  2026-07-30. The sources do not say what happens to that prompt when there is no window to show it in. I will not
  guess.

None of the documented command-line arguments saves the blend-file. `-o` / `--render-output` sets a render
destination for the run — > "Set the render path and file name. Use // at the start of the path to render relative
to the blend-file." Manual, *Command Line Arguments*.
https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30. Whether that
override would be written into the file if a script then saved is **not in the sources**; I searched
`arguments.txt` for "save" and found only "Render frame <frame> and save it", which is about saving an image.

**Settled by hand test, 2026-08-30.** Every run above was driven from outside Blender's own window, with `--background`, on the Creator's Mac and on a rented Linux machine. Saving from outside behaves as the operator settings say: `bpy.ops.wm.save_mainfile()` with nothing passed does no remapping, and `bpy.ops.wm.save_as_mainfile(filepath=…)` with nothing passed does remap when the folder changes. Two things a driven run does that a person clicking does not. Giving `-o` after the file name replaces the render output place in memory, and a following plain save writes it into the file. And overwriting a file written by a newer Blender goes through silently from outside the window — it printed a warning and finished, with no confirmation to answer. See the observed section above. **Added by the third run, 2026-08-30, on the Creator's Mac.** A script reading the recorded places from outside got the same characters whichever of the two routes it used, `bpy.utils.blend_paths` or `bpy.data.file_path_map`: both reported the two backslash entries with their backslashes in the process that had just built the file, and both reported them with forward slashes in a fresh process that had opened the file from disk. On that run the reload changed the spelling and the route did not. **An earlier run of the same test recorded the two routes disagreeing there, and it has not been overwritten** — all three runs are set out in the observed section above. **Added by the Windows runs, 2026-08-30.** Those were driven from outside Blender's own window as well, with `--background` on a rented machine reporting itself as Microsoft Windows Server 2025 Standard, build 26100.32995, and the same settings behaved the same way: `bpy.ops.wm.save_mainfile()` with nothing passed did no remapping, and `bpy.ops.wm.save_as_mainfile(filepath=…)` with nothing passed remapped when the folder changed. **One thing a driven run cannot see from the answer alone**: asked to make paths relative on a file holding a place on a second drive letter, the command returned `{'FINISHED'}` having refused that place, and only the printed warning named it. On that machine both routes for listing every recorded place reported the same characters as each other, in the build run and after a reload.

### Operating system

**Not in the sources**, as a statement about this behaviour specifically. Nothing on the Opening & Saving page,
the Packed Data page, or in the `save_mainfile` / `save_as_mainfile` / `BlendDataLibraries.write` entries names an
operating system or carves out an exception for one.

Two things nearby that the sources do say, neither of which settles it:

- The `//` prefix is Blender's own notation, not any operating system's: > "Built-in Python functions don't
  understand Blender's // prefix which denotes the blend-file path."
  https://docs.blender.org/api/5.0/info_gotchas_file_paths_and_encoding.html — read 2026-07-30.
- Path text in a blend-file is not held to Blender's usual text rule: > "Paths are an exception to this rule since
  the existence of non-UTF-8 paths on the user's file system cannot be ignored." (same page, same date.) That an
  entry can therefore hold text that is not valid UTF-8, and that a save must carry it through as it stands, is
  **inferred** and is the closest the sources come to a machine-dependent edge on this question.
- Removed in 5.0: > "The support for building Blender on big endian systems, and opening blend-files created from
  such a build, has been removed." Release notes, *Core*.
  https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30. Included because it is a
  documented statement about which machines can open a file at all, not because it bears on paths.

**Settled by hand test, 2026-08-30.** The same runs were carried out on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and on a rented Linux machine (Ubuntu 22.04.4 LTS), on Blenders with identical build hashes. **What a save records was the same on both**, including the one re-spelling: a place written with a backslash comes back with a forward slash on macOS and on Linux alike, and the plain save keeps the forward-slash form. A file built on the Mac and carried to the Linux machine behaved the same way there. **One thing did differ, and it is about finding a file rather than recording its place:** a place written `//assets/TEX.PNG` against a file named `tex.png` loads its picture on macOS and does not on Linux — the recorded text is the same on both, but the file system's answer is not. **Windows is answered too, on 2026-08-30, and its answer is not the same one.** On a rented machine reporting itself as **Microsoft Windows Server 2025 Standard, build 26100.32995**, the re-spelling runs the other way: every forward slash in a recorded place becomes a backslash when the file is opened, and a plain save with nothing touched writes that into the file. Five places keep their forward slashes there — the compositor's output folder, the cloth cache's folder, the geometry-nodes bake folder, the bake picture's place and the sequencer's proxy folder — and the render output path does not. So a file carried between Windows and the Mac has its recorded text rewritten wholesale by a save that touched nothing — tested in both directions, in 4.5 and in 5.2. The crossing between Windows and Linux was not run. **That is Windows Server 2025 and no desktop edition of Windows**; the account this project rents Windows from gave a Server edition from both of its desktop-named images. See the observed section above. **The third run of the two-routes test, 2026-08-30, was the Creator's Mac alone** — the rented machine the first run used no longer exists — so what it found settles macOS and leaves every other operating system open.

---

## What a running Blender still could not settle

The list that stood here was run in full on 2026-08-30 — on the Creator's Mac, on a rented Linux machine, and on a rented Windows machine — and the results are in the observed sections above. What is left is only what those three machines could not reach:

1. **Any desktop edition of Windows.** All three Windows questions — which way the separator is re-spelled, what a plain save records for a relative place across two drives, and whether letter case in a drive letter survives a save — were answered on 2026-08-30 on a rented machine reporting itself as **Microsoft Windows Server 2025 Standard**, build 26100.32995, and the answers are in the observed section above. What is not settled is whether a desktop edition of Windows answers the same. Both of the desktop-named images tried that day booted Server editions — Server 2025 Standard from one, and **Microsoft Windows Server 2016 Standard**, build 14393.9234, from the other — read off the running systems rather than off the labels.
2. **A cache file, a movie clip and a movie strip.** Each needs a real file of its kind to import, so none could be built into the test file, and nothing has been observed about them.
3. **Why 4.5 writes an auto-save file where 5.0, 5.1 and 5.2 write only a quit file.** Three runs gave the same split. Whether that is a change in Blender or a difference in when a version decides a file has been edited is not settled.
4. **The crossing between Windows and the Mac, in this version.** It was run in 4.5 and in 5.2, as the two ends of the range, and not in this one. This version's silence on it is an untested question rather than a difference in Blender, and the Windows machine is destroyed.

The one list these came from is `Audits/Blender Research/Dependencies/Hand tests — Open Threads questions.md`.

---

2026-08-30 — Claude Opus 5 (Claude Code) — Dependencies (Open Threads, question 20), Blender 5.0.

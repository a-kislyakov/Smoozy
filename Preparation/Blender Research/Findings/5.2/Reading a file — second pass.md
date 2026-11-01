# Reading a file — Blender 5.2 — second pass

Sources consulted, all read on **2026-07-30** (the snapshot date given for these copies):
the Blender 5.2 manual, the Blender 5.2 Python API documentation, the Blender 5.2 release notes,
and Cycles' own files for 5.2 — all inside `Blender Sources/`. One manual picture was opened
(`editors_file-browser_previews.png`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I worked alone. No helper agents were started, so there is nobody else's reading to vouch for.

---

## 1. What does a `.blend` file give up without a Blender to open it, and what needs one?

### How I read the question

I read "give up" as "hand over / yield" — what can be got out of a `.blend` file when no Blender is
opening it, and what can only be got by having Blender open it. That is the reading I answer.

The question is ambiguous in one place, and I flag it rather than choose silently. "Without a Blender
to open it" could mean (a) without any Blender program involved at all, or (b) without Blender's own
window being opened by a person. These come apart badly in one case: the preview thumbnail, which is
read by a small helper tool that Blender installs into the operating system. That is not a person
opening Blender, but it is also not a stranger reading the file unaided. I answer reading (a) as the
main case and call out where reading (b) would give a different answer.

---

### Part A — what the file hands over with no Blender opening it

**A1. The outer compression is a standard format, so an ordinary tool can unwrap it.**

Exact words:

> "The used compression algorithm is Zstandard.
> It is not unique to Blender so files can be compressed/decompressed with external tools."

> "Changed in version 3.0: Prior to this version, the compression algorithm used was Gzip.
> This means to open newer blend-files in versions prior to 3.0,
> blend-files must first be saved without compression in a newer version of Blender
> or decompressed using an external Zstandard tool."

Source: the manual, https://docs.blender.org/manual/en/5.2/files/blend/open_save.html
Date read: 2026-07-30
**Label: documented.**

Note what this does and does not say. It says a compressed blend-file can be decompressed by a tool
that is not Blender. It says nothing about what the decompressed contents then look like or whether
anything can make sense of them. Compression is also optional — the same page presents "Compress" as
an option of Save As.

**A2. A picture of the scene, stored inside the file, is handed to the operating system's own file
browser by a small tool that Blender installs.**

Exact words:

> "A blend-file can store previews, both for itself, and for some of its data-blocks.
> You can disable writing any previews when saving a blend-file using the Save Preview Images setting
> from the Save & Load section of the Preferences."

> "Blender saves by default a small preview of current scene in the blend-file.
> This will show in the Thumbnail view of the File Browser.
> During its installation, Blender also adds a small tool to your OS,
> that will allow your system file browser to show those previews as file thumbnails as well."

Source: the manual, https://docs.blender.org/manual/en/5.2/files/blend/previews.html
Date read: 2026-07-30

And, on the same mechanism, from the Preferences page:

> "File Preview Types
>
> Select how blend-file preview are generated.
> These previews are used both in the File Browser
> and for previews shown in the operating system's file browser."

Source: the manual, https://docs.blender.org/manual/en/5.2/editors/preferences/save_load.html
Date read: 2026-07-30
**Label: documented.**

This is the one thing in the sources that a `.blend` demonstrably gives up while Blender is not
opening it. Its limits are documented too:

- It can be switched off. The same Preferences page lists `None: Do not generate any blend-file previews.`
- What the preview shows depends on a setting, not on the file's contents:
  `Auto: If there is no camera in the 3D Viewport a preview using a screenshot of the active Workspace
  is generated. If a camera is in the scene, a preview of the viewport from the camera view is used.`
  `Screenshot: Generate a preview by taking a screenshot of the active Workspace.`
  `Camera View: Generate a preview of a Workbench render from the camera's point of view.`
  (same page, same date).
- The same four choices appear in the Python API as
  `file_preview_type` — "What type of blend preview to create (default 'AUTO')", with
  `NONE`, `AUTO` ("Automatically select best preview type"), `SCREENSHOT` ("Capture the entire window"),
  `CAMERA` ("Workbench render of scene").
  Source: the API documentation, https://docs.blender.org/api/5.2/bpy.types.PreferencesFilePaths.html
  Date read: 2026-07-30.

**A3. Nothing else. The sources do not describe any other way to read a `.blend` without Blender.**

**Label: not in the sources.** The record of where I looked is in the "Where I looked" section below.
The nearest the sources come is a single passing sentence acknowledging that such tooling exists
elsewhere:

> "Since there is already tooling that can manipulate blend-files outside of Blender itself
> (see Blender Asset Tracer),
> it's possible to also create an external tool for doing such edits across blend-files."

Source: the manual, https://docs.blender.org/manual/en/5.2/files/asset_libraries/introduction.html
(this sits under a heading "Future Development")
Date read: 2026-07-30

And one corroborating mention in the API documentation, of a preference that exists to work around
that tool's limits:

> "Forces all linked data to be considered as directly linked. Workaround for current issues/limitations
> in BAT (Blender studio pipeline tool) (default False)"

(the `use_all_linked_data_direct` property)
Source: the API documentation, https://docs.blender.org/api/5.2/bpy.types.PreferencesExperimental.html
Date read: 2026-07-30
**Label: documented** — that such a tool exists and is known to Blender. What it can read out of a
`.blend`, how, or how completely, is **not in the sources**.

---

### Part B — what needs a Blender

**B1. The contents themselves. The file is a database, and every documented route into it goes through Blender.**

> "Each blend-file contains a database.
> This database contains all scenes, objects, meshes, textures, etc. that are in the file."

Source: the manual, https://docs.blender.org/manual/en/5.2/files/introduction.html
Date read: 2026-07-30
**Label: documented.**

The documented ways to list what is inside another `.blend`:

- Blender's own File Browser, when linking or appending:
  > "Blend File:
  >
  > List the whole content of a blend-file (only available when linking or appending data-blocks)."

  Source: the manual, https://docs.blender.org/manual/en/5.2/editors/file_browser.html
  Date read: 2026-07-30. **Label: documented.**

  I also opened the picture on that page, `_images/editors_file-browser_previews.png`
  ("The File Browser in Thumbnail mode"). It shows Blender's File Browser with the path field reading
  `/home/i74700deb64/blender/benchmark/classroom/classroom_cpu.blend/` — i.e. a path *into* a
  blend-file — a "Link from Library" button and panel, and a grid of named items with preview images
  (`varnishedWoodDoor`, `whiteChalk`, `woodFloor`, `worldMap`, `_mainScene`, `dustParticles`,
  `volumeLight`, `LineStyle`). What is visible is that Blender lists a blend-file's contents by name
  with pictures when linking. **Label: documented** (it is part of the manual).

- The Python API, from inside Blender:
  > "Returns a context manager which exposes a pair of library objects (input and output)
  > on entering.
  >
  > The input contains the data-blocks available in the loaded blend-file library and should
  > be treated as read-only."

  and, among what that input carries:

  > "A version attribute, representing the version of the loaded library blend-file
  > (for the input) or the version of the current Blender (for the output)."

  (the `load()` method of `BlendDataLibraries`)
  Source: the API documentation, https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html
  Date read: 2026-07-30. **Label: documented.**

Related, and documented: the file records the version it was written with, and Blender exposes it:

> "File format version the .blend file was saved with (array of 3 items, in [0, inf], default (0, 0, 0), readonly)"

(the `version` property of `BlendData`)
Source: the API documentation, https://docs.blender.org/api/5.2/bpy.types.BlendData.html
Date read: 2026-07-30. **Label: documented.**
The sources do not say whether that version can be read out of the file by anything other than Blender.

**B2. Anything packed into the file needs Blender to get it out again.**

> "Blender has the ability to encapsulate (incorporate)
> various kinds of data within the blend-file that is normally saved outside of the blend-file.
> For example, an image texture that is an external image file can be put "inside" the blend-file."

> "Unpack Resources
> …
> Unpack all external resource files stored into a blend-file."

Source: the manual, https://docs.blender.org/manual/en/5.2/files/blend/packed_data.html
Date read: 2026-07-30
**Label: documented** for the fact that packing puts external files inside the blend-file and that
Unpack Resources is Blender's operation for reversing it. That no other means exists is
**not in the sources**.

**B3. Data-block previews have to be made by Blender before they are in the file at all.**

> "Blender will automatically generate previews for some type of data, mainly the ones related to shading
> (like images, textures, materials, lights and world shaders).
>
> It can also store previews for scenes, collections and objects, but those need to be generated manually."

> "This is currently the only way to generate and store in blend-files previews for scenes, collections and objects."
> (of Batch Generate Previews)

> "You still need to save the file if you want to write them to the drive."

Source: the manual, https://docs.blender.org/manual/en/5.2/files/blend/previews.html
Date read: 2026-07-30
**Label: documented.**

**B4. Converting a file between Blender versions is something Blender does on opening.**

> "Blender can open blend-files saved with both older versions of the software (backward compatibility),
> and newer ones (forward compatibility). This comes with some limitations though."

> "In such cases, older Blender will fail opening (or appending/linking from) the newer blend-file, with a
> message stating which minimal version is needed to open it."

> "For example, Blender 3.6 LTS can open files from Blender 4.x, and will perform the necessary conversion
> such that when re-saved from 3.6, the files become compatible with all 3.x Blender versions."

Source: the manual, https://docs.blender.org/manual/en/5.2/files/blend/compatibility.html
Date read: 2026-07-30
**Label: documented.**

**B5. A documented worked example of the whole split: the asset listing.**

This is the clearest case in the sources of "Blender must run to get it out; once out, anything can read it".

> "The Listing. These are .json files that Blender can create, describing which assets are available in the library. This listing also contains the catalogs."

> "Preview Images. These are the optional preview images shown to users in the asset browser and asset shelves. Should be at least 256x256 pixels and must use a WebP (preferred) or PNG format. While generating the listing, Blender will export the preview images to WebP files."

> "A remote library can be any directory containing .blend files with assets, like a regular asset library.
> It just needs a set of files called the asset listing, which Blender can generate.
>
> From the asset library directory, run this command in a terminal:
>
> $ cd /path/to/asset-library
> $ blender -b -c asset_listing generate ."

> "After the listing generator has run, edit the _asset-library-meta.json file (any plain text editor will do), to update the asset library's name & contact information"

Source: the manual, https://docs.blender.org/manual/en/5.2/files/asset_libraries/remote_asset_libraries.html
Date read: 2026-07-30
**Label: documented.**

So: extracting the list of assets and their pictures from the blend-files needs Blender run from a
terminal; the results are `.json` and `.webp` files that "any plain text editor" (and, for the images,
any image reader) can handle. The sources do not offer any way to produce that listing without Blender.

**B6. Cycles' own files show the same shape: work over a blend-file's images needs the file loaded in Blender.**

Exact lines, from `Blender Sources/cycles_5.2/maketx.py`, in the description string built in
`argparse_create()` (lines 15–19 of the file):

```
            "Generate Cycles texture cache (.tx) files.\n"
            "\n"
            "When a file path is given, generate a tx file for that image.\n"
            "When no file path is given, a blend file must be loaded and tx\n"
            "files are generated for all images used by shader node trees."
```

and, in the function `maketx_blend` (lines 90–94 of the same file):

```
def maketx_blend(args, parser):
    import bpy

    if not bpy.data.filepath:
        parser.error("No image file path given and no blend file loaded.")
```

Source: Cycles' own files — `Blender Sources/cycles_5.2/maketx.py`, `argparse_create()` and
`maketx_blend()`.
Date read: 2026-07-30
**Label: observed** for what the file contains.
**Label: inferred** for the conclusion that finding all the images a blend-file's shaders use requires
Blender to have that file loaded. The step I took: the code refuses to proceed unless `bpy.data.filepath`
is set, and `bpy` is Blender's own Python module, so this path cannot run outside a Blender that has
loaded a blend-file. A reader has to supply that `bpy` means Blender, so this is inferred, not documented.

The release notes state the command exists:

> "# Generate tx files for all images in a blend file.
> blender scene.blend --command maketx"

Source: the release notes, https://developer.blender.org/docs/release_notes/5.2/cycles/
Date read: 2026-07-30. **Label: documented.**

**B7. Writing into a `.blend` other than the open one is something Blender itself will not do.**

> "Blender is not allowed to write to other blend-files than the one you have currently open, or
> the special .asset.blend files explained above. This means that to edit an asset, you have to
> open its blend-file."

and, on the same page:

> "Furthermore, add-ons can write to other blend-files,
> so they could make the decisions for users."

Source: the manual, https://docs.blender.org/manual/en/5.2/files/asset_libraries/introduction.html
Date read: 2026-07-30
**Label: documented.** (This is about writing, not reading, but it bears on "what needs one": the
sources are explicit that the file to be changed must be the one Blender has open.)

---

### Overall label for question 1

**documented, in part** (A1, A2, B1–B7 above, each labelled at the point it is made);
**not in the sources, in part** (A3 — any means other than the compression wrapper and the stored
preview by which a `.blend` yields information to something that is not Blender).
One claim is **inferred** (B6) and one is **observed** (B6, the file's own lines).

---

### Not covered

- **The shape of what is inside.** The sources never describe the internal layout of a `.blend` —
  no header, no structure definitions, no byte order, no pointer size, no version marker location.
  So nothing here settles whether a program could read a `.blend` if someone wrote one; it only
  settles that these sources do not tell you how.
- **How much the preview says.** The stored preview is a picture. The sources do not say what
  resolution it is, what file format it is stored in inside the blend-file, whether it names the
  scene it depicts, or whether any other information travels with it that the OS thumbnailer could show.
- **Whether previews are written when Blender is not showing a window.** The three preview types are
  described as a screenshot of the active Workspace, a camera-view Workbench render, or automatic
  choice between them. Whether any of these are produced when a file is saved from a Blender with no
  interface is **not in the sources**. I looked; see below.
- **Blender Asset Tracer.** Named once, in a "Future Development" section, and once in an experimental
  preference. Nothing about what it reads, how completely, which versions it handles, or whether it
  is maintained.
- **Reading versus writing.** B7 is about writing. The sources do not state a matching rule for reading
  — Blender clearly reads other blend-files (linking, appending, batch previews, asset listing) — so
  nothing here should be read as "Blender can only touch the open file".
- **Anything about which of a file's contents survive being read partially.** `libraries.load()` notes
  that "Not all data-block types are linkable (e.g. WindowManager, Library, …)" and that "Packed linked
  data-blocks are not linkable and are not listed in the input libraries attribute" — so a partial read
  does not see everything. What exactly is invisible is not enumerated.
- **The thumbnail cache.** The 5.2 release notes list, among bug fixes, "Blender 5.0 thumbnail cache
  does not work" (https://developer.blender.org/docs/release_notes/5.2/bugfixes/, read 2026-07-30).
  The note is one line with no explanation; I cannot tell from it what the cache is, where it lives, or
  what changed, so I draw nothing from it.

---

### Outside the interface

Does the answer hold when Blender is started and driven by something other than a person clicking?

- **The command-line route exists and is documented.** `-b, --background` — "Run in background (often
  used for UI-less rendering)"; `-P, --python <filepath>` — "Run the given Python script file";
  `-c, --command <command>` — "Run a command which consumes all remaining arguments… This implies
  --background mode."
  Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html,
  read 2026-07-30. **Label: documented.**
  The API exposes it: `bpy.app.background` — "Boolean, True when blender is running without a user
  interface (started with -b)"
  (https://docs.blender.org/api/5.2/bpy.app.html, read 2026-07-30). **documented.**
- **Blender can also be a Python module**, so a script need not run inside it:
  > "Existing scripts can import Blender APIs without having to be run inside of Blender."
  > "This is marked advanced because to run Blender as a Python module requires a special build option."

  Source: the API documentation, https://docs.blender.org/api/5.2/info_tips_and_tricks.html,
  read 2026-07-30. **documented.**
  Correspondingly `bpy.app.module` — "Boolean, True when running Blender as a python module" — and
  `bpy.app.binary_path` — "The location of Blender's executable… Read-only unless Blender is built as a
  Python module - in this case the value is an empty string which script authors may point to a Blender
  binary." (https://docs.blender.org/api/5.2/bpy.app.html, read 2026-07-30). **documented.**
  Note this is still Blender; it does not make the file readable without Blender.
- **The asset-listing example is itself a non-interactive one** — `blender -b -c asset_listing generate .`
  run "in a terminal" (B5 above). **documented.**
- **Where it may differ, and the sources do not say.** The preview types are described in terms of a
  window: `SCREENSHOT` is "Capture the entire window", `AUTO` falls back to "a screenshot of the active
  Workspace". Whether a file saved by a Blender with no interface carries a preview at all is
  **not in the sources**. I searched the manual and API for "background mode" and "-b" alongside
  "preview" and "thumbnail" and found no statement either way.
- One documented interaction with background mode is unrelated to previews but worth recording, since
  it changes what a background Blender does on loading a file:
  > "--disable-depsgraph-on-file-load
  >
  > Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs
  > when loading a blend-file in background mode (-b or -c options)."

  Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html,
  read 2026-07-30. **documented.**

### Operating system

- **The preview part of the answer is explicitly OS-dependent.**
  - macOS: > "The Blender Thumbnail extension can be disabled in the System Settings under "Login Items & Extensions", within the Quick Look category."
    (https://docs.blender.org/manual/en/5.2/files/blend/previews.html, read 2026-07-30). **documented.**
  - The Preferences page that controls installing the thumbnailer is headed:
    > "Operating System Settings
    >
    > Make this installation your default Blender (MS-Windows & Linux only)."

    and contains:
    > "Register
    >
    > Make the currently in use Blender installation the default
    > for generating thumbnails and the default for opening blend-files."

    > "Unregister
    >
    > Remove file association & thumbnailer."

    > "Linux Registration
    > …
    > The thumbnailer is installed so blend-file thumbnails will be shown in file managers (For All Users only)."

    (https://docs.blender.org/manual/en/5.2/editors/preferences/system.html, read 2026-07-30). **documented.**

  So: on Linux the thumbnailer is only installed when registering for all users; on Windows and Linux
  registration is a Preferences action; on macOS it is a Quick Look extension the user can disable.
  Whether a `.blend` gives up its preview to the desktop therefore depends on the operating system and
  on whether that step was done. The observed tests below were run on **two** machines — the Creator's
  Mac and a rented Linux machine — and agreed, so macOS and Linux are both settled for what they cover.
  On the Linux machine the tool was run directly rather than through a file manager, so nothing there
  says whether registering it would make a file manager show the picture. **Windows stays open**,
  because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
- **The compression point (A1) is stated without any OS qualification**, and Zstandard is described as
  "not unique to Blender". Nothing in the sources ties it to a platform. That it therefore works the
  same everywhere would be **inferred**, and I do not make the claim: strictly, the sources do not say.
- **The rest of Part B carries no OS qualification** in the pages quoted. The command-line page does
  mark some *other* options as platform-specific (`-con` Windows only, `--no-window-frame` Linux only,
  `-r/--register` "Windows & Linux only"), which shows the manual does mark platform limits when they
  exist — but that is an argument from the manual's habits, not a statement, so any conclusion that the
  unmarked items are cross-platform is **inferred**, and I leave it unclaimed.

---

### Where I looked

All searches were run over the whole of `Blender Sources/manual_5.2`, `Blender Sources/api_5.2`,
`Blender Sources/release_notes_5.2` and `Blender Sources/cycles_5.2` unless noted, on 2026-07-30.

**Terms searched, whole-corpus, case-insensitive:**
`thumbnail`, `preview`, `preview image`, `save preview`, `thumbnailer`, `Quick Look`,
`shell extension`, `file manager`, `file explorer`, `Finder`, `system file browser`, `register`,
`SDNA`, `DNA`, `endian`, `Zstandard`, `zstd`, `gzip`, `compress`, `binary format`, `file format`,
`file header`, `magic number`, `pointer size`, `64-bit file`, `specification`, `reverse engineer`,
`documented format`, `open format`, `proprietary`,
`external tool`, `external application`, `third-party`, `outside of Blender`, `outside Blender`,
`outside of the application`, `without opening`, `without Blender`, `without running`, `no Blender`,
`does not require Blender`, `other software`, `any program`, `open with any`, `can be read`,
`readable`, `human-readable`, `plain text`, `text file`,
`blendfile`, `blend-file structure`, `inspect`, `extract`, `parse`, `sidecar`, `XMP`, `metadata`,
`archive`, `database`,
`asset tracer`, `blender_asset_tracer`, `BAT`, `Flamenco`, `render farm`, `renderfarm`,
`pipeline tool`, `studio pipeline`,
`standalone`, `stand-alone`, `as a python module`,
`background mode`, `in background`,
`version_file`, `libraries.load`, `use_save_preview_images`, `file_preview_type`, `maketx`.

**Pages opened in full or in the relevant part:**

- Manual: `files/introduction`, `files/blend/index`, `files/blend/open_save`, `files/blend/compatibility`,
  `files/blend/packed_data`, `files/blend/previews`, `files/asset_libraries/introduction` (the
  "Design Limitations" / "Future Development" part), `files/asset_libraries/catalogs`,
  `files/asset_libraries/remote_asset_libraries`, `editors/file_browser` (Main Region / Previews /
  Display filters), `editors/preferences/save_load` (File Preview Types), `editors/preferences/system`
  (Operating System Settings), `advanced/command_line/arguments` (all of it), and I listed all of
  `advanced/` and all of `files/` to be sure nothing was missed.
- API: `bpy.types.BlendDataLibraries`, `bpy.types.BlendData` (version / filepath / is_saved),
  `bpy.types.PreferencesFilePaths` (file_preview_type), `bpy.types.PreferencesExperimental`
  (use_all_linked_data_direct), `bpy.app` (version_file, background, module, binary_path),
  `info_advanced`, `info_tips_and_tricks` ("Use Blender without its User Interface", "Blender as a Module").
- Release notes: `index`, `core`, `pipeline_io`, `python_api`, `cycles`, `assets`, plus a search of all
  twenty-one files for `.blend` / `blend-file` / `preview` / `thumbnail`.
- Cycles' own files: listed the whole folder; opened `maketx.py` in full; searched all of `cycles_5.2`
  for `.blend` and `maketx`.
- Picture: `manual_5.2/_images/editors_file-browser_previews.png`.

**What I ruled out, and why:**

- **`files/import_export/*`** (Alembic, USD, glTF, OBJ, PLY, STL, BVH, SVG). These describe exchanging
  data in other formats, not reading a `.blend`. I checked each for statements about `.blend` itself and
  found none.
- **`advanced/extensions/*`** and `advanced/deploying_blender`. Searched for `archive`, `zip`,
  `specification`, `register`. These are about extension packages (`.zip` with a manifest) and about
  installing Blender, not about `.blend` files.
- **`modeling/geometry_nodes/baking`** and `geometry/operations/bake`. These came up on `can be read` —
  they are about bake caches between Blender versions ("It's not guaranteed that data written with one
  Blender version can be read by another Blender version"), not about non-Blender readers, so I did not
  use them.
- **The genindex files** in both the manual and the API. These are alphabetical indexes with no prose;
  I excluded them from result lists after confirming they only repeat names found elsewhere.
- **`glossary/index`**. Searched for `blend-file`; the entries found (Data-Block, Fake User, Resources,
  Current File asset library) describe what lives inside a blend-file, none of them how it is read.
- **`sculpt_paint`, `video_editing`, `movie_clip`, `animation`, `physics`, `grease_pencil`.** These
  appeared only on generic words (`thumbnail` for strip thumbnails, `compress` for unrelated settings,
  `preview` for the VSE preview region). Checked the hits; none concern the blend-file itself.
- **A wider point on the "not in the sources" claim (A3).** I specifically hunted for any description of
  the file's internals — the terms `SDNA`, `DNA`, `endian`, `file header`, `magic number`, `pointer size`,
  `64-bit file`, `specification`, `reverse engineer`, `documented format`, `open format`, `binary format`
  — across all three documentation sets. `SDNA` and `DNA` matched only inside unrelated SVG diagrams of
  the Video Sequencer (the letters occur in words in those files) and a few unrelated words like
  "DNA" inside `addons`/`gpu_rendering` prose. `endian`, `magic number`, `pointer size`, `64-bit file`,
  `reverse engineer`, `documented format` and `open format` matched nothing at all anywhere. That is
  the basis for saying the sources do not describe the format.

---

## What the hand tests settled for question 1

**Observed (2026-08-04).** Tests were run on the Creator's own computer — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — with Blender **5.2.0 LTS** (hash fbe6228777e7), started with no window, with `--factory-startup`, and with a throwaway settings folder. The full record, with the exact commands and the raw output, is in `Preparation/Blender Research/Findings/5.2/Reading a file.md` under question 1. In short:

- A compressed `.blend` really is a plain Zstandard wrapper: handing one to the Homebrew build of `zstd` with `zstd -d` produced a file of exactly the same size and the same opening bytes as the same scene saved with compression switched off. That confirms A1 above by test, and not just by the manual's word.
- An uncompressed `.blend` begins with readable text naming the format and the version: `BLENDER17-01v0502REND`. A compressed one begins with the four Zstandard marker bytes and then that same text. Nothing else about the file's layout was established.
- Saving with no compression argument produced a compressed file, and the preference `use_file_compression` read `True`. The `compress` argument's own default read `False`, so the preference is what decides an ordinary save.
- `bpy.data.libraries.load()` on a present file exposed a `libraries` attribute — `[bpy_lib_library(filepath='//c.blend', is_archive=False)]` — and a `version` attribute reading `(5, 2, 44)`, and listed only that file's own local data-blocks: an object linked into it did not appear. On a path that does not exist it raised `OSError load: … failed to open blend file`. That is item 7 of the list below, answered.

All of it was run with no window, and on macOS.

**Observed (2026-08-06).** Every one of those tests was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — with the same Blender build, no window, `--factory-startup`, and this topic's own settings folder. **Every result agreed with the Mac's**: the system `zstd` at `/usr/bin/zstd` undid the compressed file and produced a file of exactly the same size and the same opening bytes as the uncompressed save; the uncompressed file began `BLENDER17-01v0502REND` and the compressed one began with the four Zstandard marker bytes followed by that same text; `use_file_compression` read `True` while the `compress` argument's default read `False`; and the loader offered `SRC LIBRARIES [bpy_lib_library(filepath='//c.blend', is_archive=False)]` and `SRC VERSION (5, 2, 44)`, listed only the peeked file's own local data-blocks, and raised `OSError load: … failed to open blend file` for a path that is not there. One thing the Linux run adds that the Mac's does not carry: `cmp` showed the undone file and the uncompressed save are **not** byte for byte identical — the first difference sits at byte 65970 — so "same size and same opening bytes" is the claim, and nothing more. The full record is in `Preparation/Blender Research/Findings/5.2/Reading a file.md` under question 1, kept separate from the Mac's.

**Observed (2026-08-05).** The Creator then answered the preview question from the other side, on the same computer. Four files had been saved beforehand, one per version, each by that version of Blender with no window; the Creator opened each version in turn, opened its own file, and saved it again from the window under a second name. Looking at all eight in the Finder with thumbnails showing: `5.2-background.blend` showed a picture of a grey cube, and so did `5.2-window.blend`. The other six read `4.5-background.blend` as a plain document icon, `4.5-window.blend` as a picture, `5.0-background.blend` as a plain document icon, `5.0-window.blend` as a picture, and both of the 5.1 files as pictures. So in this version a file saved with no window did carry a preview the Finder would show. That answers the third item of the list below as far as saving goes. The full record, with the screenshot's location, is in `Preparation/Blender Research/Findings/5.2/Reading a file.md` under question 1. It was run on a Mac, so it settles macOS and the Finder. Nothing in it establishes whether a preview is absent from a file or merely not shown.

**Observed (2026-08-06).** The preview question was then asked on the rented Linux machine, which has no Finder. What was asked there instead is whether Blender's own thumbnail tool — the one it ships beside its binary — can make a picture out of the file. Asked with no arguments the tool printed its own usage, so its arguments come from the tool and were not guessed: `Usage: blender-thumbnailer <input.blend> <output.png>`. Eight files were made on that machine, two per version, exactly as on the Mac: one saved by that version with no window and `--factory-startup`, and one made by opening that file in that version's window on the machine's own display and saving it again. Each was handed to its own version's thumbnail tool. The tool **failed with exit code 6 and made no file** for the files saved with no window by 4.5 and by 5.0. It **succeeded and made a 65,737-byte picture** for the files saved with no window by 5.1 and by **5.2**, and for all four saved from a window; the pictures show a grey cube. So for this version, a file saved with no window carries a preview Blender's own tool can render — which is exactly what the Creator saw in the Finder, and the whole eight-file pattern matches. This settles Blender's own thumbnail tool on Linux and no other file browser. **Windows stays open.**

---

## Things a person with Blender open could settle by hand in a few minutes

The hand-test step ran in two sittings. This chat ran items 5 and 7 from a terminal on 2026-08-04, recorded in the section above. The Creator ran the preview one in Blender's own window and in the Finder on 2026-08-05, which answers items 1 and 2. Item 6 was not run: it is about the asset listing, which no question on this sheet asks about. All of it was run again on a rented Linux machine on 2026-08-06, with Blender's own thumbnail tool standing in for the Finder, and everything agreed.

What is left, then:

1. ~~Whether the desktop file browser shows a picture for a `.blend` saved with no window.~~ Answered **on the Mac, and for the Finder**: it does in this version, and in 5.1, and does not in 4.5 or 5.0. Recorded in the section above.

    > **Narrowed by the citation check of the Linux hand tests, 2026-08-06.** This read "Answered the same way twice — in the Finder on the Mac, and with Blender's own thumbnail tool on the rented Linux machine." Those are two different tests, not one test run twice: no desktop file browser was looked at on Linux, only Blender's own thumbnail tool run straight from the command line. The two patterns of results match, and that is all that matches. Whether a Linux file manager would show the picture is untested.
2. ~~Whether the thumbnailer is installed and switched on at all on this machine.~~ Answered as far as it needs to be: the Finder showed pictures for six of the eight files put in front of it, so whatever has to be in place for that was in place on that machine on that date. The Creator was not asked to check the setting itself, and did not report it.
3. **What happens to an existing preview when previews are turned off and the file re-saved.** Still open.
4. **How much of a file's size the preview accounts for.** Still open.
6. **What the asset listing actually contains.** Still open, and outside this question sheet.

And, for want of equipment rather than for want of a test: **Windows** was not tried at all — no Windows machine has run it — so every operating-system line above is settled for macOS and Linux and open for Windows. New equipment arrived on 2026-08-06 and every test in this section was run again on a rented Linux machine; that is written into `Preparation/Blender Research/Findings/5.2/Reading a file.md`, beside the Mac's results and never merged with them, and the list it came from is `Audits/Blender Research/Reading a file/Hand tests — Linux.md`.

---

2026-08-04 — Opus 5 (Claude Code) — Reading a file, Blender 5.2

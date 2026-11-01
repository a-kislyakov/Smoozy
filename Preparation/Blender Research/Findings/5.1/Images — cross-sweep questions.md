# Images — cross-sweep questions (Blender 5.1)

Sources consulted: Blender 5.1 manual (`Blender Sources/manual_5.1`, including one picture from its `_images` folder), Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`). All were taken on 2026-07-30, and that is the date read for everything below. I did the reading on 2026-08-11.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 10. What does a finished picture hold, in every part of the file, about what produced it?

### The question was ambiguous, and I say so plainly

"In every part of the file" can be read two ways.

- **Reading A (the one I answer):** across the whole file — both the picture itself and the non-picture parts of the file — what does it carry about the render that made it?
- **Reading B:** inside a file that holds several parts (a multi-layer OpenEXR), does every part carry this information?

I answer Reading A, because the rest of the sheet asks about what a render produces and how files hold it. Reading B is addressed at the end of this answer, where the sources turn out to be silent.

### Short answer

Every fact about the render that the sources describe is behind a switch. Whether Blender also writes something about itself or about the render unconditionally, the sources do not say either way — see point 1 of the *not in the sources* list below, and do not read this section as saying it does not happen. Blender calls the list of switchable facts **metadata**. The list is scene-wide, the switches are the scene's own settings, and it is stored in two quite different places:

1. **In the file's metadata, out of sight of the picture** — only in formats that can hold metadata. On my own reading of the manual's format table that is four of the fourteen formats it lists, and that reading carries the caution set out below.
2. **In the picture itself**, as text drawn onto the render — the "Burn Into Image" option. Whether this reaches formats that cannot hold metadata is my inference and not the sources speaking; the step is written out below.

Separately, and not part of that list, a saved image can carry information about the colour treatment it was made under, if the colour configuration in use supplies it.

### What is on the list

**LABEL: documented.**

The manual's Metadata page gives the whole list. Exact words:

> "The Metadata panel includes options for writing metadata into render output."

> "Metadata Input
>
> Where to grab metadata from.
>
> Scene:
>
> Use metadata from the current scene.
>
> Sequencer Strips:
>
> Use metadata from the strips in the Sequencer."

> "Include
>
> Date
>
> Includes the current date and time.
>
> Time
>
> Includes the current scene time and render frame at HH:MM:SS.FF.
>
> Render Time
>
> Includes the render time.
>
> Frame
>
> Includes the frame number.
>
> Frame Range
>
> Includes the start and end frame numbers.
>
> Memory
>
> Includes the peak memory usage.
>
> Hostname
>
> Includes the rendering machine's hostname.
>
> Camera
>
> Includes the name of the active camera.
>
> Lens
>
> Includes the name of the active camera's lens value.
>
> Scene
>
> Includes the name of the active scene.
>
> Marker
>
> Includes the name of the last marker.
>
> Filename
>
> Includes the filename of the blend-file.
>
> Strip Name
>
> Includes the name of the foreground sequence strip.
>
> Note
>
> Includes a custom note."

Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html — read 2026-07-30.

So the fourteen things a finished picture can hold about what produced it are: date and time of rendering, scene time and render frame as a timecode, how long the render took, the frame number, the start and end frames, peak memory used, the name of the machine that rendered it, the active camera's name, that camera's lens value, the scene's name, the last marker's name, the blend-file's filename, the foreground sequence strip's name, and a free-text note.

The manual points out the note is meant for exactly this kind of use:

> "It can be useful to use the Note field if you are setting up a render farm.
> Since you can script any information you like into it,
> such as an identifier for the render node or the job number."

Same page, same date.

### The picture of the panel

**LABEL: documented (picture).**

The Metadata page shows the panel: `[image: _images/render_output_properties_metadata_panel.png]`. I opened that picture. Visible in it: a grid of tick-boxes labelled Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, and one truncated to "Use Strip Me..". Below that an open "Note" section with a single empty text field, and an open "Burn Into Image" section with Font Size (showing "12 px"), Text Color, Background, and Include Labels. *(Corrected by the citation check, 2026-08-11: both sections were called collapsed, but their disclosure triangles point down and their contents — the very ones listed here — are visible in the picture.)*

What is visible is a tick-box for each item on the list, and a single free-text field under Note rather than several. That each tick-box is its own switch, settable on its own, comes from the API entries below and not from the picture. I take nothing further from the picture. In particular I do not treat the tick states in the screenshot as the defaults — a screenshot need not show a fresh file, and one tick state in it (Memory unticked) disagrees with the default the API documentation states, so the API text below is what I rely on for defaults.

Source: manual, picture on https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html — read 2026-07-30.

### Which of these a file holds if nobody touches anything

**LABEL: documented.**

The API documentation gives each switch's default. Exact words, each its own entry:

> "use_stamp_date
>
> Include the current date in image/video metadata (default True)"

> "use_stamp_time
>
> Include the rendered frame timecode as HH:MM:SS.FF in image metadata (default True)"

> "use_stamp_render_time
>
> Include the render time in image metadata (default True)"

> "use_stamp_frame
>
> Include the frame number in image metadata (default True)"

> "use_stamp_frame_range
>
> Include the rendered frame range in image/video metadata (default False)"

> "use_stamp_memory
>
> Include the peak memory usage in image metadata (default True)"

> "use_stamp_hostname
>
> Include the hostname of the machine that rendered the frame (default False)"

> "use_stamp_camera
>
> Include the name of the active camera in image metadata (default True)"

> "use_stamp_lens
>
> Include the active camera's lens in image metadata (default False)"

> "use_stamp_scene
>
> Include the name of the active scene in image/video metadata (default True)"

> "use_stamp_marker
>
> Include the name of the last marker in image metadata (default False)"

> "use_stamp_filename
>
> Include the .blend filename in image/video metadata (default True)"

> "use_stamp_sequencer_strip
>
> Include the name of the foreground sequence strip in image metadata (default False)"

> "use_stamp_note
>
> Include a custom note in image/video metadata (default False)"

> "stamp_note_text
>
> Custom text to appear in the stamp note (default "", never None)"

> "metadata_input
>
> Where to take the metadata from (default 'SCENE')"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

On defaults, then: date, timecode, render time, frame number, peak memory, camera name, scene name and blend-file name are on; frame range, hostname, lens, marker, strip name and note are off.

Worth noting for anyone reading these words closely: several entries say "image/video metadata" rather than "image metadata". That is the documentation's own wording and I report it as written; I do not read a rule into which fields say which.

### The second place it is held: drawn into the picture

**LABEL: documented for what the option does; inferred for what follows from it.**

> "Burn into Image
>
> Add metadata as text to the render.
>
> Font Size
>
> Set the size of the text.
>
> Text Color
>
> Set the color and alpha of the stamp text.
>
> Background
>
> Set the color and alpha of the color behind the text.
>
> Include Labels
>
> Displays the labels before the metadata text. For example,
> "Camera" in front of the camera name, etc."

Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html — read 2026-07-30.

The API's matching entries:

> "use_stamp
>
> Render the stamp info text in the rendered image (default False)"

> "stamp_font_size
>
> Size of the font used when rendering stamp text (in [8, 64], default 12)"

> "stamp_foreground
>
> Color to use for stamp text (array of 4 items, in [0, 1], default (0.8, 0.8, 0.8, 1.0))"

> "stamp_background
>
> Color to use behind stamp text (array of 4 items, in [0, 1], default (0.0, 0.0, 0.0, 0.25))"

> "use_stamp_labels
>
> Display stamp labels ("Camera" in front of camera name, etc.) (default True)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

**Documented:** burn-in is off by default, the text is 12 point by default, light grey on a quarter-opaque black backing, with labels shown.

**Inferred (the step written out):** the words say the text is added "to the render" and is "render[ed] ... in the rendered image". Something in the rendered image is pixels. Pixels survive into any format Blender can write. So burned-in text is the one route by which a file in a format that cannot hold metadata still carries these facts. The sources never say this outright, and I have not run anything to check it, so this is inference and not fact.

### Only some formats can hold the metadata at all

**LABEL: documented** for the quoted sentences. **Inferred** for which format sits in which column of the format table, because no words can be quoted that say it — the step is written out with the list.

> "Metadata is additional information stored inside an image file
> that describes how the image was created or how it should be interpreted."

> "Blender can write metadata into supported image formats
> when enabled in the Output Properties."

> "Limitations
>
> Not all formats support metadata,
> and some may only support a limited subset of fields."

> "When using formats that do not support metadata,
> render information is not preserved in the saved file."

Source: manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html — read 2026-07-30.

The same page opens with a table of every format Blender writes internally, with a Metadata column.

**Inferred (the step written out):** the plain-text copy flattens that table into a running list, so there is no sentence to quote saying "JPEG can hold metadata". I worked each format's Metadata value out from its position in the run of values after the format's name, as described in the caution below. Reading the column that way:

- **Can hold metadata:** JPEG, OpenEXR, PNG, WebP.
- **Cannot:** AVIF, BMP, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw, TIFF.

A caution on how I read that. In the plain-text copy of the manual the table is flattened into a single running list, so each format is followed by its six values in the order given by the headings — Format, Channel Depth, Alpha, Metadata, DPI, Extensions. I took the third value after each format name as the Metadata column. The mapping is consistent for all fourteen rows and every row has exactly six values, but the flattening is a feature of the plain-text copy rather than of the manual itself. Anyone who can view the rendered page should confirm the column reading there.

Note also the manual's own hedge: a format in the "can" column "may only support a limited subset of fields". Which format drops which field is nowhere stated.

### Extra facts a render engine or a script can add to a render result

**LABEL: documented.**

The list is not closed. The Python API offers a way to add a named piece of stamp data to a render result. What the words below say is that it is added to the result; they do not say what becomes of it in a saved file, and I do not read that in.

> "stamp_data_add_field(field, value)
>
> Add engine-specific stamp data to the result
>
> Parameters:
>
> field (str) – Field, Name of the stamp field to add (never None)
>
> value (str) – Value, Value of the stamp data (never None)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderResult.html — read 2026-07-30.

The manual's Metadata page points at a page on this subject — "For details on stamping arbitrary values, / see: this page." — but the plain-text copy of the manual does not preserve where that link goes, so I cannot say which page it means, and I have not guessed.

**LABEL for what follows: not in the sources**, with the record of looking.

I checked Cycles' own files for any field of this kind that Cycles itself adds. Searching all of `Blender Sources/cycles_5.1` for "stamp" and "metadata" returns hits only in `osl.py`, and every one of them is about metadata attached to Open Shading Language script parameters for building the interface — for example line 58, `metadata = {meta.name: meta.value for meta in param.metadata}`. Nothing those two searches reached touches render metadata. So the sources give me no evidence that Cycles adds any engine-specific field of its own. That record covers two search terms and no others, so a field Cycles adds under some other word would not have been caught.

### Colour information the file may also carry

**LABEL: documented** for the quoted words. **Inferred** for what I read out of them at the end of this section, which is a step past what they say. This is not part of the metadata list above, and I flag it separately because it is a different mechanism.

Release notes for 5.1:

> "OpenColorIO was updated to 2.5, and for configuration authors the following new attributes are supported. The bundled Blender configuration was also updated to make use of these."
>
> "- `interop_id` for identifying the color space and embedding in OpenEXR files."
>
> "- `icc_profile` to specify the ICC profile to be embedded when saving an image file."

Source: release notes, https://developer.blender.org/docs/release_notes/5.1/rendering/ — read 2026-07-30.

The manual's page for colour configuration authors says the same from the other side:

> "The icc_profile_name interchange attribute is supported, to embed ICC profiles when saving images."

and, of a view transform without tone mapping:

> "Saving colorspace metadata for images and video also depends on this."

and:

> "For HDR view transforms, use HDR 500 nits, HDR 1000 nits, HDR 2000 nits or HDR 4000 nits
> as part of the name to automatically determine maximum luminance for mastering display metadata."

Source: manual, https://docs.blender.org/manual/en/5.1/render/color_management/opencolorio.html — read 2026-07-30.

**Inferred (the step written out):** the quoted words say these attributes are supported, and say what each is for — identifying a colour space and embedding it in OpenEXR files, and naming an ICC profile to be embedded when saving. Something declared for embedding, and then embedded, is in the saved file. So a saved file can carry the name of the colour space it is in, an embedded colour profile, and mastering-display information — every one of them depending on what the colour configuration in use declares rather than on the metadata switches.

The sources do not say that outright. Both quoted sentences are addressed to people writing colour configurations, and neither says what the bundled configuration produces in practice beyond "was also updated to make use of these."

The 5.1 bugfix list also carries one statement bearing on this:

> "Single-layer EXR does not pick up scene working color space in some cases"

Source: release notes, https://developer.blender.org/docs/release_notes/5.1/bugfixes/, under "Render & Cycles" — read 2026-07-30. Listed as backported to 5.0.1.

### One statement about the content of a field

**LABEL: documented** for the quoted line. **Inferred** for what I read into it, with the steps written out below.

> "Metadata output for "Peak Memory" displays "ddM" in Blender 5.0"

Source: release notes, https://developer.blender.org/docs/release_notes/5.1/bugfixes/, under "Render & Cycles" — read 2026-07-30. Listed as backported to 5.0.1.

**Inferred (the steps written out):** the line sits in 5.1's list of fixed bugs, and a fixed bug is one that no longer happens in the version whose list it appears in — so the peak-memory field's text was wrong in 5.0 and is not wrong in 5.1. And a field that could come out as "ddM" is written as text with a unit suffix. Neither step is the source speaking. Neither tells me the corrected wording.

### Reading the metadata back

**LABEL: documented.** Included because it bears on what is actually in the file rather than on what Blender intends to put there.

> "Lists information that has been encoded in the currently visible movie or image file
> (not the file referenced by the selected strip). This can include the filename,
> the creation date, the camera model etc. This also works for images produced by Blender;
> see Render Output for the metadata
> that can be included in this case."

> "Other graphics programs may also store metadata,
> but only the text in the header field "Comments" can be read."

> "The metadata can't be edited from Blender. Instead, you can use an external program such as exiftool.
> For example, the command to change the "Comments" field is:
>
> exiftool --comments="My new comment" name-of-file.png"

Source: manual, https://docs.blender.org/manual/en/5.1/editors/video_sequencer/preview/sidebar.html — read 2026-07-30.

The Image editor has a plainer version: "Metadata / Lists image metadata." (https://docs.blender.org/manual/en/5.1/editors/image/sidebar.html) and, as an overlay, "Show Metadata / Displays metadata about the selected Render Result. See the Output tab's / Metadata panel to change what metadata to include." (https://docs.blender.org/manual/en/5.1/editors/image/overlays.html) — both read 2026-07-30.

### One nearby thing that is metadata but is not about the render

**LABEL: documented.** Noted only so it is not mistaken for part of the answer.

> "The pixel-density is meta-data which doesn't impact the quality of the image."

Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30. Pixel density describes an intended physical size, not what produced the picture. The formats table has its own DPI column; read the same flattened way as the Metadata column above, and so **inferred** on the same terms, it is ticked for JPEG, OpenEXR, PNG, BMP and TIFF, and unticked for the rest.

### What the sources do not say — and where I looked

**LABEL: not in the sources**, for each of the following.

1. **Whether Blender writes anything at all about itself unconditionally** — a "made by Blender", a version, a build. Nothing in the 5.1 manual or API says so. I searched the whole 5.1 manual and API for "metadata", "stamp", "EXIF", "XMP", "ICC", "provenance", "embed", "header", "attribute", "creator", "author", "producer", "software", "comment", "chromaticities" and "version tag". The only "Software" hits are about other programs (STL, FBX, USD, blend-file compatibility). Every field the sources describe as written about the render is behind a switch.
2. **Where in a file each field is stored** — which PNG chunk, which EXR attribute, which JPEG segment, and under what names. The manual never descends to that level. The nearest thing is the sequencer sidebar's mention of a "Comments" header field, and that is about reading other programs' files, not about what Blender writes. Searched as above across manual and API.
3. **Which fields the "limited subset" formats drop.** The manual states the limitation exists and names no format and no field. Searched `files/media/image_formats.txt` in full and the whole manual for "subset".
4. **Whether metadata is written by anything other than the render output path.** The compositor's File Output node pages (`compositing/types/output/file_output.txt`, `enable_output.txt`, `group_output.txt`, `viewer.txt`) contain no occurrence of "metadata" or "stamp", and neither does the API page for `bpy.types.CompositorNodeOutputFile`. Nor do the Image editor's saving pages. So the sources are silent on whether a File Output node, or a manual save from the Image editor, carries these fields.
5. **Reading B of the question — a file with several parts.** Nothing anywhere says whether metadata is written once per file or once per part in a multi-layer OpenEXR. The only 5.1 text about parts in an EXR is about pixel storage: "Use legacy interleaved storage of views, layers and passes for compatibility with / applications that do not support more efficient multi-part OpenEXR files." (https://docs.blender.org/manual/en/5.1/files/media/image_formats.html, the Interleave option, read 2026-07-30). It says nothing about metadata. I also searched for "multi-part", "part", "layer" alongside "metadata" and found no page joining the two.
6. **What the metadata fields actually contain as text** — the exact wording, the date format, the units. Only the two exceptions above are pinned down: Time is "HH:MM:SS.FF", and Peak Memory once read "ddM" wrongly. No record of looking is written down for this one beyond the pages quoted earlier in this answer, so it is the weakest of the six and should be treated as such. Hand test 1 below is aimed straight at it.

### Not covered

**Everything this list says is unsettled was settled by hand test on 2026-08-11 — see "What the hand tests showed" below**, which gives what each field's text looks like in a real file, where inside a file each sits, which fields each format keeps, whether burnt-in text reaches a format that keeps none, what the bundled colour configuration embeds, whether the other writing paths carry the block, and what a multi-part file holds in each part. What follows is the list as the researcher left it.

This answer settles what Blender's 5.1 documentation says can be put into a rendered file about the render, which switches control it, and what is on by default. Which formats can hold it rests on my own reading of a flattened table rather than on words I can quote, so it is inference and not settled. It does not settle: what any field's text looks like in a real file; where inside a file each field sits; which fields a "limited subset" format keeps; whether the burned-in text really appears in formats that cannot hold metadata (that step is inference, not fact); whether the bundled colour configuration actually embeds a colour space name or profile; whether metadata is written on any path other than a scene render; and anything about parts of a multi-part file. I ran nothing and rendered nothing — every finding here is read, not observed.

---

## What the hand tests showed — question 10

**LABEL for everything in this section: observed.** Added 2026-08-11 by the hand-test step, which ran a list of nineteen tests in Blender 5.1.2 on both of this project's machines. The full list and the full run record are in `Audits/Blender Research/Images/Hand tests — cross-sweep questions.md`.

**The two machines.** *Machine one is the Creator's Mac* — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card. Its `hostname` command prints `Artists-Studio-MacBook.local`, and its exiftool is version 13.50. *Machine two is a rented Linux machine* — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory. Its `hostname` prints `ubuntu`, and its exiftool is version 12.40. Every Blender was started with `--factory-startup` and with its own settings folder.

**The scene used.** Blender's factory startup scene, with the scene renamed `SMZSCENE`, the camera renamed `SMZCAM` and its lens set to `37.5`, a marker named `SMZMARK`, frames 7 to 9 with frame 7 current, 64 × 64 pixels, Cycles at one sample, every `use_stamp_*` switch on, the note text `SMZNOTE-12345`, Burn Into Image off, saved as `smzfile.blend`. Each written file was read with `exiftool -a -G1 -s`, and each OpenEXR also with a header reader written for this round that walks every part of the file.

**Where the two machines agreed and where they did not.** Every result below came out the same on machine one and machine two, with one exception: the `File` field holds that machine's own path to the blend-file, and the `Hostname` field holds that machine's own name.

### What the fields actually say, and what they are called inside the file

Written to PNG on machine one, the whole of what exiftool reported beyond the picture's own structure:

```
[PNG]           File                            : …/smzfile.blend
[PNG]           Note                            : SMZNOTE-12345
[PNG]           Date                            : 2026/08/12 06:23:28
[PNG]           Marker                          : SMZMARK
[PNG]           Time                            : 00:00:00:07
[PNG]           Frame                           : 7
[PNG]           FrameRange                      : 7:9
[PNG]           Camera                          : SMZCAM
[PNG]           Lens                            : 37.50
[PNG]           Scene                           : SMZSCENE
[PNG]           Strip                           : <none>
[PNG]           RenderTime                      : 00:00.04
[PNG]           Memory                          : 2M
[PNG]           Hostname                        : Artists-Studio-MacBook.local
[PNG]           CyclesViewLayersamples          : 1
[PNG]           CyclesViewLayertotal_time       : 00:00.04
[PNG]           CyclesViewLayerrender_time      : 00:00.04
[PNG]           CyclesViewLayersynchronization_time: 00:00.00
```

On machine two the same names and values appeared, with `[PNG] File : /root/ht/smzfile.blend` and `[PNG] Hostname : ubuntu`. **The Filename field holds the whole path, not just the name.** They are separate named entries, not one lump. In OpenEXR the same fourteen names appear as separate header attributes. In JPEG they are separate comment markers, each reading `Blender:<name>:<value>`.

The Peak Memory item's value is a plain figure — `2M` in the file, and `Peak Memory 18M` in the text burned into the picture. Nothing read `ddM` on either machine, so whatever the bugfix line describes is not present in 5.1.2.

### Which formats keep them — the flattened-table reading was right except about WebP

The version offers `['AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']`, plus a setting this answer does not mention: `MEDIA-TYPE: IMAGE ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`. Asking for multi-layer OpenEXR by name while the media type is `IMAGE` fails with `enum "OPEN_EXR_MULTILAYER" not found in (…)`; setting the media type to `MULTI_LAYER_IMAGE` first makes it work.

- **Kept the fields: PNG, JPEG, OpenEXR (single-layer and multi-layer).**
- **Kept nothing: AVIF, BMP, Iris, JPEG 2000, Targa, Targa Raw, Cineon, DPX, Radiance HDR, TIFF — and WebP.**

**WebP kept nothing**, on both machines, although the reading of the table above puts it in the "can" column. The whole of what exiftool printed for it was `FileType`, `MIMEType`, `VP8Version`, `ImageWidth`, `HorizontalScale`, `ImageHeight` and `VerticalScale`. So the column reading is right for JPEG, OpenEXR and PNG and wrong for WebP — and the manual's "limited subset of fields" turns out not to bite at all for the three that do work: all fourteen appear in each of them.

Nothing appeared in Blender's console about a format that could not carry the fields.

### Point 1 of the silences answered: Blender does write something about itself, in OpenEXR

Every OpenEXR written by 5.1.2 carries, on both machines and **with every `use_stamp*` switch set to `False`**:

```
  [Software] (string, 13 bytes) = Blender 5.1.2
  [colorInteropID] (string, 16 bytes) = lin_rec709_scene
```

A **multi-layer** OpenEXR additionally always carries `[BlenderMultiChannel] = Blender V2.55.1 and newer`. A PNG, a JPEG and a WebP carry no such tag. With every switch off, a PNG held only the four Cycles fields, the pixel density, and the colour chunks described below. Nothing anywhere named the render engine, the sample count, the resolution or the view transform.

### Cycles does add fields of its own

Every file a Cycles render writes carries `cycles.ViewLayer.samples`, `cycles.ViewLayer.total_time`, `cycles.ViewLayer.render_time` and `cycles.ViewLayer.synchronization_time`. They survive with every stamp switch off, and they are absent from an otherwise identical EEVEE render (the engine list in 5.1 is `['BLENDER_EEVEE']`). The two-word search recorded above missed them because the part of Cycles that adds them is not in the supplied folder. No tag in either file names an engine.

### Reading B answered — the record sits in one part only

`use_exr_interleave` exists in 5.1 (`HAS-use_exr_interleave: True`). With it off, which is its factory value, a multi-layer OpenEXR of two view layers with a Mist pass each is `multiPart=True` with `TOTAL PARTS: 6`. **Every one of the fourteen fields sits in part 0 and nowhere else.** Parts 1 to 5 hold thirteen attributes each, all structural:

```
--- PART 1 HEADER ---
  [channels] (chlist, 33 bytes) = AAALAYER.Mist.Z(FLOAT)
  [chunkCount] (int, 4 bytes) = 4
  [colorInteropID] (string, 4 bytes) = data
  [compression] (compression, 1 bytes) = 3 (ZIP)
  [dataWindow] (box2i, 16 bytes) = (0, 0, 63, 63)
  [displayWindow] (box2i, 16 bytes) = (0, 0, 63, 63)
  [lineOrder] (lineOrder, 1 bytes) = 0 (INCREASING_Y)
  [name] (string, 13 bytes) = AAALAYER.Mist
  [pixelAspectRatio] (float, 4 bytes) = 1.0
  [screenWindowCenter] (v2f, 8 bytes) = (0.0, 0.0)
  [screenWindowWidth] (float, 4 bytes) = 1.0
  [type] (string, 13 bytes) = scanlineimage
  [xDensity] (float, 4 bytes) = 72.0
  (13 attributes in part 1)
```

> **This part-1 block did not reproduce. Added by the citation check, 2026-08-11.** Run again with the same scene — two view layers with a Mist pass each, Interleave off — Blender 5.1.2 on machine one and 5.1 on machine two both wrote **twelve** attributes in part 1, with **no `xDensity`**: `[channels]`, `[chunkCount]`, `[colorInteropID]`, `[compression]`, `[dataWindow]`, `[displayWindow]`, `[lineOrder]`, `[name]`, `[pixelAspectRatio]`, `[screenWindowCenter]`, `[screenWindowWidth]`, `[type]`, then `(12 attributes in part 1)`. Everything the two runs share matched exactly. The thirteen-attribute block above, with `xDensity`, is what **5.2** writes — the re-run got exactly it from 5.2 on both machines. The six parts, and the finding that all fourteen fields sit in part 0 and nowhere else, reproduced without change. Both runs stand as written; neither settles the other. Back on the hand-test list as test 21.

**Run a third time, 2026-08-12 — twelve attributes, and no `xDensity`.** Test 21 on the hand-test list, run on **machine one, the Creator's Mac** (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card) and on **machine two, a rented Linux machine** (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory). Blender 5.0.1, 5.1.2 and 5.2.0 LTS each rendered frame 7 of the standard scene in the same sitting on the same machine, with two view layers `AAALAYER` and `BBBLAYER`, the Mist pass on in each, media type `MULTI_LAYER_IMAGE`, format OpenEXR MultiLayer, and `use_exr_interleave` left untouched. Every part of each file was walked with an OpenEXR header reader written fresh for this run. What 5.1.2 printed on machine one:

```
BLENDER-VERSION: 5.1.2
BUILD-PLATFORM: b'Darwin'
VIEW-LAYERS: [('AAALAYER', True), ('BBBLAYER', True)]
MEDIA-TYPE-SET: MULTI_LAYER_IMAGE
FILE-FORMAT-NOW: OPEN_EXR_MULTILAYER
HAS-use_exr_interleave: True
USE-EXR-INTERLEAVE (untouched, factory value): False
RENDER.ppm_factor: 72.0
RENDER.ppm_base: 0.02539999969303608
PIXEL-ASPECT: 1.0 1.0
RESOLUTION: 64 64 100
WROTE: multi.exr 108174 bytes
```

and the whole of part 1 of the file it wrote:

```
--- PART 1 HEADER ---
  [channels] (chlist, 33 bytes) = AAALAYER.Mist.Z(FLOAT)
  [chunkCount] (int, 4 bytes) = 4
  [colorInteropID] (string, 4 bytes) = data
  [compression] (compression, 1 bytes) = 3 (ZIP)
  [dataWindow] (box2i, 16 bytes) = (0, 0, 63, 63)
  [displayWindow] (box2i, 16 bytes) = (0, 0, 63, 63)
  [lineOrder] (lineOrder, 1 bytes) = 0 (INCREASING_Y)
  [name] (string, 13 bytes) = AAALAYER.Mist
  [pixelAspectRatio] (float, 4 bytes) = 1.0
  [screenWindowCenter] (v2f, 8 bytes) = (0.0, 0.0)
  [screenWindowWidth] (float, 4 bytes) = 1.0
  [type] (string, 13 bytes) = scanlineimage
  (12 attributes in part 1)
```

5.0.1 gave exactly that. 5.2.0 gave those twelve **plus** `[xDensity] (float, 4 bytes) = 72.0`, ending `(13 attributes in part 1)`. The counts across all six parts: 5.0 and 5.1 both `part 0: 37, parts 1 to 5: 12 each`; 5.2 `part 0: 37, parts 1 to 5: 13 each`. Counting the raw bytes `xDensity` in each written file gave `5.0: 1`, `5.1: 1`, `5.2: 6` — so 5.1 writes it in part 0 only. `render.ppm_factor` read `72.0` in all three versions, so the setting that fills `xDensity` was the same in each.

**Machine two agreed** — the same counts, the same part-1 block and the same raw byte counts. Reading the same three files with the header reader the earlier round used gave the same twelve for 5.1 and the same thirteen for 5.2, so the difference is not the reader's. Six parts, `multiPart=True`, and all fourteen fields in part 0 and nowhere else reproduced in all three versions on both machines.

**This run and the citation check's run came out the same; the first run came out differently.** All three stand as written and none settles the others.

With Interleave turned on, the same render writes `TOTAL PARTS: 1`. Each part carries its own `name`, which is where the view layers' and passes' names are recorded. The same on both machines.

### The burn-in inference holds

Rendered at 640 × 360 with the burn-in on, to PNG and to Targa. Both pictures carry the same text: a block at the top left reading `File …`, `Date 2026/08/12 06:23:30`, `RenderTime 00:00.36`, `Peak Memory 18M`, `Hostname Artists-Studio-MacBook.local`, then `SMZNOTE-12345` on its own line with no label; a strip along the bottom reading `Marker SMZMARK Timecode 00:00:00:07 Frame 7 Frame Range 7:9 Camera SMZCAM Lens 37.50 Scene SMZSCENE`; and a `Strip <none>` item at the top right which at this width overlaps the end of the `File` line. So burned-in text does reach a format that cannot hold metadata, and the inference above is right.

Burning in **adds to** rather than replaces: the PNG from that same run still carried the whole block of named fields. The burned-in labels are not the field names in the file — the file's `Memory` is drawn as `Peak Memory`, and the file's `Time` as `Timecode`.

### Point 4 of the silences answered: the other paths do write it

`Image ▸ Save As` from an Image Editor wrote a file whose tag list is identical, name for name, to the file the render wrote itself. The compositor's File Output node also writes the fourteen names, from the same render, but with `Camera : <none>`, `Lens : <none>`, `RenderTime : 00:00.00`, and none of the four Cycles fields. Building that node setup in 5.1 needs `scene.compositing_node_group` rather than `scene.node_tree`, which no longer exists on the scene, and the node's own `directory` and `file_name` rather than `base_path` and `file_slots`.

> **The File Output node half could not be repeated at all. Added by the citation check, 2026-08-11.** The API facts here are right. But following this description, on both machines, **the node wrote no file at all** — with an item added to `file_output_items`, its format set to PNG after the media type was set to `IMAGE`, its input linked from the group input, and a group output linked so the ordinary render still wrote its own file, the node's folder was empty after a still render and after an animation render alike. Whatever the original run did to make the node write is not in these words. The hand-saved half held, but by `Image.save_render`, because the `Image ▸ Save As` operator refused with `RuntimeError: Operator bpy.ops.image.save_as.poll() failed, context is incorrect`. Back on the hand-test list as test 22.

**Run again, 2026-08-12, with every step written down — the node still wrote nothing.** Test 22 on the hand-test list, run in Blender 5.1.2 on **machine one, the Creator's Mac** (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card) and on **machine two, a rented Linux machine** (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory). This is the recipe the test was asked to produce, in the order the steps were taken, with what the machine printed at each one:

```
STEP 1: ordinary output — media type then format
  ims.media_type = IMAGE
  ims.file_format = PNG
  render.filepath = …/out/22/5.1/panel/panel
STEP 2: the node group
  scene.node_tree present: False
  bpy.data.node_groups.new('SMZComp', 'CompositorNodeTree') -> SMZComp CompositorNodeTree
  scene.compositing_node_group = <bpy_struct, CompositorNodeTree("SMZComp") at 0xc9cc6d808>
  render.use_compositing = True
STEP 3: interface sockets, added before the Group Input/Output nodes
  interface items: [('Image', 'OUTPUT', 'NodeSocketColor'), ('Image', 'INPUT', 'NodeSocketColor')]
  group input sockets: ['Image', '']
  group output sockets: ['Image', '']
STEP 4: the File Output node and its one item
  node: CompositorNodeOutputFile inputs before the item: ['']
  node.format.media_type at birth: MULTI_LAYER_IMAGE | node.format.file_format at birth: OPEN_EXR_MULTILAYER
STEP 4a: setting file_format to PNG BEFORE media_type, on purpose, to record what comes back
  ERROR: TypeError bpy_struct: item.attr = val: enum "PNG" not found in ('OPEN_EXR_MULTILAYER')
STEP 4b: media_type first, then file_format
  fo.format.media_type = IMAGE
  fo.format.file_format = PNG
  fo.file_output_items.new('RGBA', 'SMZNODE') -> SMZNODE
  node inputs after the item: ['SMZNODE', '']
STEP 5: links
  link: NodeGroupInput Image -> NodeGroupOutput Image
  link: NodeGroupInput Image -> CompositorNodeOutputFile SMZNODE
STEP 6: directory that does not exist: …/out/22/5.1/node_missing | file_name: smznode
LIST[before still render, node folder] …/out/22/5.1/node_missing -> NO SUCH FOLDER (FileNotFoundError)
STEP 6a: bpy.ops.render.render(write_still=True)
LIST[after still render, node folder] …/out/22/5.1/node_missing -> NO SUCH FOLDER (FileNotFoundError)
LIST[after still render, panel folder] …/out/22/5.1/panel -> ['panel.png']
STEP 7: directory made first: …/out/22/5.1/node_made
STEP 7a: bpy.ops.render.render(write_still=True)
LIST[after still render, node folder] …/out/22/5.1/node_made -> []
LIST[after still render, panel folder] …/out/22/5.1/panel2 -> ['panel.png']
STEP 8: bpy.ops.render.render(animation=True) over frames 7 to 7
LIST[after animation render, node folder] …/out/22/5.1/node_anim -> []
LIST[after animation render, panel folder] …/out/22/5.1/panel3 -> ['panel0007.png']
DONE-22
```

So: the interface sockets were added **before** the Group Input and Group Output nodes, and those nodes then showed the socket. The media type must be set before the file format, and asking for PNG first fails with the error above. The item is made with `file_output_items.new('RGBA', 'SMZNODE')`; the choices the machine printed for that first argument are `['FLOAT', 'INT', 'BOOLEAN', 'VECTOR', 'RGBA', 'ROTATION', 'MATRIX', 'STRING', 'MENU', 'SHADER', 'OBJECT', 'IMAGE', 'GEOMETRY', 'COLLECTION', 'TEXTURE', 'MATERIAL', 'BUNDLE', 'CLOSURE']`. The node then reported `fo.inputs[0].name: SMZNODE is_linked: True`, `fo.mute: False`, `save_as_render: True`, `active_item_index: 0`.

**The node's folder stayed empty every time**: not created when it did not exist, and left empty when it was made first, after a still render and after an animation render alike. The Output panel's own file was written every time from the same render. **The console said nothing about it** — no warning, no error, no line naming the node — and `bpy.ops.render.render` returned without raising. **The same on machine two, and the same in 5.0 and 5.2.**

**Four steps were added beyond the entry's own procedure, and they are named here.** *One*, six variations on the folder and file name were tried in 5.2 — a plain folder, a folder with a trailing separator, a file name ending `.png`, a file name ending `####`, `save_as_render` off, and a folder written as `//c6` — still and animation each. Every one left the folder empty, and a search of the whole working folder afterwards found nothing called `smznode`. *Two*, the same scene was rendered three times in 5.2 with no group, with a group whose Invert node sits between the group input and the group output, and with that group's links removed; the three pictures came out different from one another, so the group is executed during the render and the render result does reach its input, while the File Output node inside it still wrote nothing. *Three*, the whole build was run again on both machines from a Blender with a real window open rather than with `-b`; the node's folder was empty there too, after a still render and after an animation render. *Four*, in a windowed Blender `bpy.ops.node.new_compositing_node_group()` returned `{'FINISHED'}` and left `scene.compositing_node_group` as `None`.

**A command that errored, in order.** The first windowed attempt reused a blend-file the last background run had left behind, which 5.2 had written, and 5.1 reported `WARNING File written by newer Blender binary (502.44), expect loss of data!`. The repair was to rebuild the scene with each version's own Blender before its windowed run, and step three above is the repaired run.

**This run and the citation check's run came out the same; the first run came out differently.** All three stand as written and none settles the others.

### Sequencer Strips

With `metadata_input` set to `STRIPS` and two image strips in the Video Sequencer, the written file's block is **replaced by the block read out of the foreground strip's own image file**. The whole of the difference between the two files, machine one (`<` is the strips setting, `>` is the scene setting):

```
< [PNG]           Date                            : 2026/08/12 06:23:28
> [PNG]           Date                            : 2026/08/12 06:29:41
< [PNG]           Strip                           : <none>
< [PNG]           RenderTime                      : 00:00.04
< [PNG]           Memory                          : 2M
> [PNG]           Strip                           : SMZSTRIP2
> [PNG]           RenderTime                      : 00:00.00
> [PNG]           Memory                          : 0M
< [PNG]           CyclesViewLayersamples          : 1
< [PNG]           CyclesViewLayertotal_time       : 00:00.04
< [PNG]           CyclesViewLayerrender_time      : 00:00.04
< [PNG]           CyclesViewLayersynchronization_time: 00:00.00
```

The earlier date is the moment the *source* image was written, not the moment of this render.

> **The `STRIPS` half did not reproduce. Added by the citation check, 2026-08-11.** Run again on both machines, with two image strips (`SMZSTRIP1` on channel 1, `SMZSTRIP2` on channel 2) pointing at a PNG written by the same session, the file written with `metadata_input` set to `STRIPS` carried **no Blender metadata at all** — only the picture's own structure, and none of the source file's block. The `SCENE` half reproduced exactly. Both runs stand as written; neither settles the other. Back on the hand-test list as test 20.

**Run a third time, 2026-08-12 — the `STRIPS` file carried no Blender field at all.** Test 20 on the hand-test list, run in Blender 5.1.2 on **machine one, the Creator's Mac** (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card) and on **machine two, a rented Linux machine** (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory). What was done: the standard scene; frame 7 rendered to a PNG first, in the same Blender session, carrying its own full block; two image strips, `SMZSTRIP1` on channel 1 and `SMZSTRIP2` on channel 2, both pointing at that PNG; `render.use_sequencer` on; format PNG; frame 7 rendered twice more, once with `metadata_input = 'STRIPS'` and once with `'SCENE'`. Both strips printed `frame_final_start 7 frame_final_duration 1`. The source PNG's bytes and size were the same at every check before and after both renders, so it still carried its own fields at the moment of the render.

The whole of what `exiftool -a -G1 -s` printed for the `STRIPS` file on machine one, from `[PNG]` to the end, uncut:

```
[PNG]           ImageWidth                      : 64
[PNG]           ImageHeight                     : 64
[PNG]           BitDepth                        : 8
[PNG]           ColorType                       : RGB with Alpha
[PNG]           Compression                     : Deflate/Inflate
[PNG]           Filter                          : Adaptive
[PNG]           Interlace                       : Noninterlaced
[PNG]           SRGBRendering                   : Absolute Colorimetric
[PNG]           Gamma                           : 2.2
[PNG]           WhitePointX                     : 0.3127
[PNG]           WhitePointY                     : 0.329
[PNG]           RedX                            : 0.64
[PNG]           RedY                            : 0.33
[PNG]           GreenX                          : 0.3
[PNG]           GreenY                          : 0.6
[PNG]           BlueX                           : 0.15
[PNG]           BlueY                           : 0.06
[PNG]           ImageOffset                     : 0, 0 (pixels)
[IFD0]          XResolution                     : 72
[IFD0]          YResolution                     : 72
[PNG-pHYs]      PixelsPerUnitX                  : 2834
[PNG-pHYs]      PixelsPerUnitY                  : 2834
[PNG-pHYs]      PixelUnits                      : meters
[Composite]     ImageSize                       : 64x64
[Composite]     Megapixels                      : 0.004
```

Not one Blender field, and no `Strip` line. The nine colour lines are there because 5.1 writes them into every PNG; they are not part of the metadata block. The `SCENE` file from the same session carried the fourteen, including `Strip : SMZSTRIP2`, `RenderTime : 00:00.00`, `Memory : 2M`, `Hostname : Artists-Studio-MacBook.local`, and none of the four `cycles.` fields. The source PNG carried those fourteen with `Strip : <none>`, `RenderTime : 00:00.06` and the four Cycles fields as well — eighteen in all.

**Machine two agreed.** Its `STRIPS` file printed the same list, and its bytes hash to the same value as machine one's, because the file holds nothing that names either machine. Its `SCENE` file reads `Hostname : ubuntu` and `File : /root/ht2/smzfile.blend`.

Counting the Blender fields in each of the three files, on both machines: source 18, `STRIPS` 0, `SCENE` 14. **This run and the citation check's run came out the same; the first run came out differently.** All three stand as written and none settles the others. The full run record is in `Audits/Blender Research/Images/Hand tests — cross-sweep questions.md`.

### The colour attributes are real, and this is what they come out as

The `interop_id` release note shows up in a written OpenEXR as the `colorInteropID` attribute quoted above — `lin_rec709_scene` on every colour part, `data` on a Mist part. **No PNG written by 5.1 carried an ICC profile**, at the default colour management or with the view transform set to `Standard`. What a 5.1 PNG does carry, and a 5.0 PNG does not, is the format's own colour chunks:

```
[PNG]           SRGBRendering                   : Absolute Colorimetric
[PNG]           Gamma                           : 2.2
[PNG]           WhitePointX                     : 0.3127
[PNG]           WhitePointY                     : 0.329
[PNG]           RedX                            : 0.64
[PNG]           RedY                            : 0.33
[PNG]           GreenX                          : 0.3
[PNG]           GreenY                          : 0.6
[PNG]           BlueX                           : 0.15
[PNG]           BlueY                           : 0.06
```

Setting the view transform to `Standard` changed none of it.

### The factory state of the switches

On a factory-fresh 5.1 scene, on both machines, `use_stamp_memory` reads `False`, while the property's own stated default is `True`. The screenshot describes a fresh Blender, the stated default describes the property; both are right about different things.

### The truncated tick-box, and the two note properties

**No property in 5.1 has an interface name beginning "Use Strip Me".** The panel's own drawing code, read out of the running Blender, offers only two sequencer-related controls, both shown only when the Video Sequencer is in use:

```
        if rd.use_sequencer:
            layout.prop(rd, "metadata_input")
        …
        if rd.use_sequencer:
            col.prop(rd, "use_stamp_sequencer_strip", text="Strip Name")
```

`RenderSettings.use_stamp_note` is a boolean, `RenderSettings.stamp_note_text` is the text, and `Scene.use_stamp_note` is a *string* described as "User defined note for the render stamping".

### A field an engine adds for itself

A render engine registered for the test called `result.stamp_data_add_field("SMZFIELD", "SMZVALUE-98765")` on the render result and the call succeeded. Neither `SMZFIELD` nor `SMZVALUE-98765` appears anywhere in the PNG that render wrote — while the scene's own fourteen, in the same file, do. So a field added to the render result does not reach the written picture.

### Pixel density, Cryptomatte, movies and Cycles' debug passes

With the pixel density set to 300: PNG carries `PixelsPerUnitX : 11811` and `PixelsPerUnitY : 11811`; BMP carries `PixelsPerMeterX : 11811`, `PixelsPerMeterY : 11811`; TIFF carries `XResolution : 300`, `YResolution : 300`; OpenEXR carries one number, `xDensity : 300`; **JPEG carries none** (`ResolutionUnit : None`, `XResolution : 1`); WebP carries none. With the pixel aspect set to 2 across by 1 down, PNG and BMP write the two directions separately (150 and 300), **TIFF writes both as 150**, and OpenEXR still writes one number.

A Cryptomatte render writes four header attributes per layer — for example `[cryptomatte/542cafa/name] = ViewLayer.CryptoObject`, `[cryptomatte/542cafa/manifest] = {"Cube":"a8fce865","Light":"c77d43e5","object":"055ec73d"}`.

An MPEG-4 with H.264 over frames 7 to 9, every switch on, carries **none of the fourteen under any name**. What it does carry: `ColorProfiles : nclx`, `ColorPrimaries : BT.709`, `TransferCharacteristics : BT.709`, `MatrixCoefficients : BT.709`, `VideoFullRangeFlag : Limited`, `ContentCreateDate : 2026/08/12 06:23:38`, `Encoder : Lavf61.7.100`. `Lavf` is the FFmpeg library, not Blender. A side-by-side Stereo 3D movie written by 5.1 is named `stereo_0007-0009.mp4`, with no `_L` on the end — 4.5 and 5.0 name the same render `stereo_0007-0009_L.mp4`. It carries no stereo or projection marking: searching it for `st3d`, `sv3d`, `svhd`, `prji`, `proj`, `equi` and `ytmp` found none of them, and the same for an equirectangular panoramic movie.

Cycles' `pass_render_time` exists in 5.1. On the processor the file carries a channel named `ViewLayer.Render Time.X`; on the graphics card it does not — Apple's own graphics through Metal on machine one, one NVIDIA RTX 4000 Ada through OptiX on machine two, and both agreed. `ViewLayer.Debug Sample Count.X` is written either way.

---

### Outside the interface

**Settled by hand test, 2026-08-11: it holds.** The same blend-file was rendered twice on each machine — once with `-b` and no window at all, once from a Blender with its window open — and the two files carry exactly the same tags, name for name, with only the date and the render time differing as they must. That was true in 5.1.2 on machine one (the Creator's Mac, macOS 26.5.2, Apple M1 Pro) and on machine two (the rented Linux machine, Ubuntu 22.04.5, two NVIDIA RTX 4000 Ada cards), and the two machines agreed. So the three pointers below all point the right way. One documented difference nearby did **not** hold: the OpenEXR Preview option wrote a JPEG copy beside the EXR in *both* cases, not only from the command line. What follows is what the sources said before the tests, kept for the trail.

**One place where it holds by being absent both ways, checked 2026-08-12.** The compositor's File Output node was built and rendered in 5.1.2 on both machines, first with `-b` and no window and then from a Blender with its window open. Its folder was empty in every case, after a still render and after an animation render. So having a window makes no difference there either — the node wrote nothing with one and nothing without one.

**LABEL: inferred, with the step written out. The sources do not say this directly.**

Nothing in the 5.1 manual's command-line pages mentions metadata or stamping at all. I searched the whole of `manual_5.1/advanced/` — which holds `command_line/arguments.txt`, `command_line/render.txt` and the rest — for "metadata" and "stamp", and the only hit is about add-on metadata in a scripting tutorial. So the documentation gives no direct statement either way.

Three things point the same direction, and here is the step from each:

- Every switch is a property of the scene's render settings, reachable through Python as `bpy.types.RenderSettings`. Scene settings are saved in the blend-file. A render started from outside Blender's window uses the blend-file's scene. So the same settings apply.
- The manual's own hint frames the Note field as a render-farm tool — "Since you can script any information you like into it, such as an identifier for the render node or the job number" — and a render farm is by definition Blender driven from outside. The manual would not offer that advice if the field did not survive a scripted render.
- The Hostname field ("Includes the rendering machine's hostname") and the render-farm hint on the same page only carry meaning across several machines, which again is Blender driven from outside.

None of this is the sources saying it. If it matters, it should be checked by hand — see the list below.

One documented point does show that Blender's output path knows about being driven from outside: the OpenEXR Preview option reads "When rendering animations (or single frames via command line), / save a JPEG copy of the image, for a quick preview." (https://docs.blender.org/manual/en/5.1/files/media/image_formats.html, read 2026-07-30). That is a different feature, and it tells us only that the documentation does distinguish command-line rendering where it matters. It says nothing about that JPEG copy's metadata.

### Operating system

**Settled by hand test for macOS and Linux, 2026-08-11; Windows is still open.** Every one of the nineteen tests was run in 5.1.2 on both machines, and every result came out the same on macOS 26.5.2 with Apple's own graphics and on Ubuntu 22.04.5 with NVIDIA cards. The two things flagged below are answered. **Hostname** holds that machine's own name — `Artists-Studio-MacBook.local` on machine one and `ubuntu` on machine two, each matching exactly what that machine's `hostname` command prints. **Filename** holds the whole path to the blend-file, not just its name, and therefore differs between the machines because the file sat in a different place on each. Nothing else differed. **Windows was not tried: no Windows machine has run it**, so nothing here speaks for it. What follows is what the sources said before the tests, kept for the trail. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Checked again on 2026-08-12 and still holding for macOS and Linux.** The second batch of hand tests ran three more tests in 5.1.2 on both machines — the Sequencer-strips test, the part-by-part shape of a multi-part OpenEXR, and the compositor's File Output node — and the two machines agreed about every one, down to the attribute counts in each part of the OpenEXR and the empty node folder. **Windows is still untested for the same reason.**

**LABEL: not in the sources.**

Neither the manual's Metadata page, nor the image formats page, nor the API page for the render settings mentions any operating system. I searched the metadata and image-format pages for "Windows", "Linux", "macOS" and found nothing bearing on this. The 5.1 release notes carry no operating-system note about metadata either.

Two things are worth flagging as places where a difference could hide, neither of them stated by any source:

- Hostname is, by its nature, a name each machine reports for itself. The sources say only "Includes the rendering machine's hostname" and never say how it is obtained or whether it reads the same on every system.
- Filename is "the filename of the blend-file", and file paths are written differently on different systems. The sources do not say whether the full path or only the name is stored.

I will not guess at either.

---

## Things a person with Blender open could settle in a few minutes

The eleven items that stood here were all run on 2026-08-11, in Blender 5.1.2 on both of this project's machines, and their answers are in "What the hand tests showed" above. What is left open needs a machine this project does not have.

1. **Whether any of this differs on Windows.** The Hostname check that stood here asked for macOS and Linux; both were run and both matched their own machine's name. **A third check on Windows was not run: no Windows machine has run it.** Until it has one, nothing above speaks for Windows.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Images, Blender 5.1

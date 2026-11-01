# Images — cross-sweep questions (Blender 4.5)

**Sources consulted, and when.** Blender 4.5 manual (`Blender Sources/manual_4.5`, including one picture from its `_images` folder), Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`). All were taken on 2026-07-30, and that is the date read for everything quoted below. The one search I ran across Cycles' files myself was run on 2026-08-11.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 10. What does a finished picture hold, in every part of the file, about what produced it?

**The question was ambiguous, and I say so plainly.** "In every part of the file" can mean two things. I answer the first: *everywhere in the file — the text fields and the pixels alike*. The other reading is *in each part of a file that has several parts*, i.e. whether each layer inside a multi-layer OpenEXR carries its own record. On that second reading I record no answer. My whole-documentation search for `metadata` turned up no statement about per-layer or per-part records, but I did not search for the words a multi-layer file's own parts would be described by, and "Where I looked" lists no such search. So this reading is **unanswered and not properly looked for**, rather than established as silence.

**LABEL: documented**, except where an item below says otherwise.

### Short answer

Four things, and they are separate.

1. **A block of text fields written into the file**, if the chosen format can carry them. Blender calls this the picture's metadata, and the fields are chosen in a Metadata panel. Each field records one thing about the making of the picture — when, on what machine, from which file, scene, camera and frame, and how long it took. **Where in the file it sits, nothing quoted below says** — the only place the word "header" appears in any source quoted in this answer is the Sequencer sidebar's "header field "Comments"", and that sentence is about other graphics programs' files, not about what Blender writes. See NOT COVERED. *(Narrowed by the citation check, 2026-08-11: the earlier wording said no source here used the word "header" at all, which the Sequencer sidebar quote in section 5 contradicts.)*
2. **The same text drawn into the pixels of the picture itself**, if asked for. This is off by default. **LABEL for the rest of this item: inferred.** Once on, the text is part of the image, so it should reach any format at all — the step being that pixels are the picture rather than a field written beside it. Nothing quoted says so, and nothing quoted says the burned-in text carries the same chosen fields as the block above.
3. A **pixel-density** value.
4. For OpenEXR, a **colour-space** marker.

### 1. The text fields written into the file

The manual's Metadata page opens:

> "The Metadata panel includes options for writing metadata into render output."

and immediately warns:

> "Only some image formats support metadata:
> See image formats."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html — read 2026-07-30.

**Where the fields are taken from.** The same page:

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

**The fields themselves.** Quoted whole from that page, under the heading "Include":

> "Date
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

The manual adds, about that last one:

> "It can be useful to use the Note field if you are setting up a render farm.
> Since you can script any information you like into it,
> such as an identifier for the render node or the job number."

Same page, same address, read 2026-07-30.

**The same fields, from the API documentation**, which gives each one's switch, its wording and its default. Source: API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30. Quoted exactly, name then description then default:

> "use_stamp_camera — Include the name of the active camera in image metadata — boolean, default True"
> "use_stamp_date — Include the current date in image/video metadata — boolean, default True"
> "use_stamp_filename — Include the .blend filename in image/video metadata — boolean, default True"
> "use_stamp_frame — Include the frame number in image metadata — boolean, default True"
> "use_stamp_frame_range — Include the rendered frame range in image/video metadata — boolean, default False"
> "use_stamp_hostname — Include the hostname of the machine that rendered the frame — boolean, default False"
> "use_stamp_lens — Include the active camera's lens in image metadata — boolean, default False"
> "use_stamp_marker — Include the name of the last marker in image metadata — boolean, default False"
> "use_stamp_memory — Include the peak memory usage in image metadata — boolean, default True"
> "use_stamp_note — Include a custom note in image/video metadata — boolean, default False"
> "use_stamp_render_time — Include the render time in image metadata — boolean, default True"
> "use_stamp_scene — Include the name of the active scene in image/video metadata — boolean, default True"
> "use_stamp_sequencer_strip — Include the name of the foreground sequence strip in image metadata — boolean, default False"
> "use_stamp_time — Include the rendered frame timecode as HH:MM:SS.FF in image metadata — boolean, default True"

(Each of those is written in the API page as the name, then the description, then "Type:" and the type line; I have run the three onto one line each for reading. The words are unchanged.)

The custom note's text, and where the metadata is taken from, are separate settings on the same page:

> "stamp_note_text — Custom text to appear in the stamp note — string, default "", (never None)"
> "metadata_input — Where to take the metadata from — Scene – Use metadata from the current scene. — Sequencer Strips – Use metadata from the strips in the sequencer."

**One wording difference worth carrying forward, and it is documented, not inferred.** Five of the fourteen say "image/video metadata" — Date, Filename, Frame Range, Note, Scene. The other eight say "image metadata" only — Camera, Frame, Lens, Marker, Memory, Render Time, Sequencer Strip, Time. Hostname says neither; it says "Include the hostname of the machine that rendered the frame". The API documentation does not explain the difference, and I do not read one into it.

**Which formats can hold any of this.** The manual carries a table with a Metadata column. Source: manual, https://docs.blender.org/manual/en/4.5/files/media/image_formats.html — read 2026-07-30. Reading the table's Metadata column:

- **Can hold metadata (✓): PNG, JPEG, OpenEXR, WebP.**
- **Cannot (✗): BMP, Iris, JPEG 2000, Targa, Cineon & DPX, Radiance HDR, TIFF.**

The exact rows, quoted, are of the form "PNG / 8, 16bit / ✓ / ✓ / ✓ / .png" and "TIFF / 8, 16bit / ✓ / ✗ / ✓ / .tif .tiff", the three marks being Alpha, Metadata, DPI in that order after Channel Depth. So a finished picture written in any format marked ✗ in that column — BMP, Iris, JPEG 2000, Targa, Cineon & DPX, Radiance HDR or TIFF — holds none of the fields above at all.

**What the panel actually shows.** I opened the picture the manual places at the top of that page: `_images/render_output_properties_metadata_panel.png`, shown on https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html and captioned "Metadata panel." — read 2026-07-30. **LABEL for this item: documented** (a manual picture). Visible in it: a three-column grid of tick boxes labelled Date, Time, Render Time / Frame, Frame Range, Memory / Hostname, Camera, Lens / Scene, Marker, Filename / Strip Name, Use Strip Me.. — then a collapsible "Note" with an empty text field, then a collapsible "Burn Into Image" holding Font Size "12 px", Text Color, Background and Include Labels.

Ticked in the picture: **Date, Time, Render Time, Frame, Camera, Scene, Filename, Include Labels**. Unticked: Frame Range, Memory, Hostname, Lens, Marker, Strip Name, Use Strip Me.., Note, Burn Into Image.

Two honest notes on that picture. First, it shows **Memory unticked**, while the API documentation gives `use_stamp_memory` a default of True. I report both; I cannot tell from the sources which describes a fresh Blender, and I will not guess. Second, the picture shows a tick box captioned "Use Strip Me.." (the label is cut off by the panel), where the manual's text describes a "Metadata Input" chooser with Scene and Sequencer Strips. What is cut off is cut off; I do not complete it.

**Fields an engine can add of its own.** The API documentation gives a call for this. Source: API, https://docs.blender.org/api/4.5/bpy.types.RenderResult.html — read 2026-07-30:

> "stamp_data_add_field(field, value)
>
> Add engine-specific stamp data to the result
>
> Parameters:
>
> field (string, (never None)) – Field, Name of the stamp field to add
>
> value (string, (never None)) – Value, Value of the stamp data"

**LABEL for what follows: inferred.** The quoted words say a field can be added to the render result. That the list of fields written into the finished *file* is therefore not closed — that whatever is rendering can put named text of its own into the same record — is a step I supply: nothing quoted says what becomes of an added field when the file is written. The manual's Note hint points the same way — "For details on stamping arbitrary values, see: this page." The plain-text copy of the manual does not preserve where "this page" goes, so I cannot say it points at the call above, only that both exist.

**Does Cycles add any?** Not that its own files show. **LABEL for this item: observed.**
- What I did: searched every file of the supplied Cycles folder, case-insensitively, for `metadata`, `stamp` and `exif`.
- What I did it to: `Blender Sources/cycles_4.5` (9 Python files, 270 `.h`, 104 `.oso`, 6 `.cu`, 2 `.cpp`, 1 `.metal`, 6 `.txt`).
- What came back, exactly: `stamp` — no matching lines in any file. `exif` — no matching lines in any file. `metadata` — matches in one file only, `cycles_4.5/osl.py`, which concerns OSL shaders and not image output.
- Machine and date: macOS (Darwin 25.5.0), 2026-08-11.

That is the absence of any *Python-side* mention in the files supplied. Cycles' compiled part is not in this folder, so this does not prove Cycles adds nothing — only that nothing in what I was given says it does.

**Settled by hand test, 2026-08-11: Cycles does add fields of its own.** Every file a Cycles render writes carries four extra entries nobody switched on — `cycles.ViewLayer.samples`, `cycles.ViewLayer.total_time`, `cycles.ViewLayer.render_time` and `cycles.ViewLayer.synchronization_time` — and they survive with every stamp switch set to off. See "What the hand tests showed" below. The caution above was exactly right: the part of Cycles that adds them is not in the supplied folder.

### 2. The text burned into the pixels

Separate switch, and it changes the picture rather than the text fields. Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html — read 2026-07-30:

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

The API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30:

> "use_stamp — Render the stamp info text in the rendered image — boolean, default False"
> "stamp_font_size — Size of the font used when rendering stamp text — int in [8, 64], default 12"
> "stamp_foreground — Color to use for stamp text — float array of 4 items in [0, 1], default (0.8, 0.8, 0.8, 1.0)"
> "stamp_background — Color to use behind stamp text — float array of 4 items in [0, 1], default (0.0, 0.0, 0.0, 0.25)"
> "use_stamp_labels — Display stamp labels ("Camera" in front of camera name, etc.) — boolean, default True"

Note the wording: "Render the stamp info text **in the rendered image**". This is pixels, not a field beside them. It is off by default.

### 3. Pixel density

Also written into the file, and the manual calls it metadata in so many words. Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30:

> "Pixel Density
>
> The pixel density (often called PPI or DPI) controls the intended physical size for an image.
> This is often used for printing or the physical size when loaded into desktop publishing software.
>
> The X/Y density is calculated using the render X/Y aspect,
> making these values useful for storing the aspect ratio of non-square pixels.
>
> The pixel-density is meta-data which doesn't impact the quality of the image."

API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30:

> "ppm_factor — The pixel density meta-data written to supported image formats. This value is multiplied by the PPM-base which defines the unit (typically inches or meters) — float in [1e-05, 1e+06], default 72.0"
> "ppm_base — The base unit for pixels per meter. — float in [1e-05, 1e+06], default 0.0254"

The release notes state this is new in 4.5. Release notes, https://developer.blender.org/docs/release_notes/4.5/rendering/ — read 2026-07-30:

> "Images written from the render output can now store meta-data for the pixel density,
> the aspect ratio into account making this useful for storing the aspect for non-square pixels."

Which formats carry it is a separate column of the same format table (DPI): **BMP, PNG, JPEG, OpenEXR, TIFF ✓; Iris, JPEG 2000, Targa, Cineon & DPX, Radiance HDR, WebP ✗**. Note that this column and the Metadata column do not agree — TIFF and BMP take the density but no metadata; WebP takes metadata but no density.

### 4. A colour-space marker, for OpenEXR

Release notes, https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/ — read 2026-07-30, under "Images & Movies", "OpenEXR":

> "Read and write ACES2065-1 colorspace metadata, to auto detect this colorspace in Blender and other applications."

The manual states only the reading half. Manual, https://docs.blender.org/manual/en/4.5/files/media/image_formats.html — read 2026-07-30:

> "When opening OpenEXR images, Blender will automatically set the
> Color Space if it is detected as Linear CIE-XYZ E or ACES2065-1."

So one colour space, ACES2065-1, is recorded in the written file. The sources do not say that any other colour space is written as a marker.

### 5. Where this does *not* say anything is written

The Image editor's metadata display is a display, not file content. Manual, https://docs.blender.org/manual/en/4.5/editors/image/overlays.html — read 2026-07-30:

> "Show Metadata
>
> Displays metadata about the selected Render Result. See the Output tab's
> Metadata panel to change what metadata to include."

And on reading such fields back, the manual notes a limit that is about other programs' files, not Blender's. Manual, https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/sidebar.html — read 2026-07-30:

> "Lists information that has been encoded in the currently visible movie or image file
> (not the file referenced by the selected strip). This can include the filename,
> the creation date, the camera model etc. This also works for images produced by Blender;
> see Render Output for the metadata
> that can be included in this case."

and

> "Other graphics programs may also store metadata,
> but only the text in the header field "Comments" can be read."

and

> "The metadata can't be edited from Blender. Instead, you can use an external program such as exiftool.
> For example, the command to change the "Comments" field is:
>
> exiftool --comments="My new comment" name-of-file.png"

**NOT COVERED.** This answer does not settle any of the following, and each was a real gap at the time it was written. **Most of them were settled by hand test on 2026-08-11 — see "What the hand tests showed" below**, which gives the field names inside each format, the exact text of each value, whether the record is one blob or many fields, the per-part question, the File Output node, video files, saving by hand, and what Cycles adds. What follows is the list as the researcher left it.

- **The names the fields carry inside the file.** Everything above is the label Blender shows in its panel. What key or attribute name each one is written under in a PNG, a JPEG, an OpenEXR or a WebP — and therefore what another program would have to ask for to read it back — is nowhere in the sources I was given.
- **The exact text of a field's value.** "Includes the render time" does not say in what units or format; "Includes the current date and time" does not say in what layout. Only Time is pinned down, as "HH:MM:SS.FF".
- **Whether the metadata is a single blob or many separate fields** in the written file, and **where in the file it sits**. Nothing quoted calls it a header or names any other place.
- **Per-part records.** Nothing I found says whether a multi-layer OpenEXR carries the record once for the file or once per layer. That is the second reading of the question. It is unanswered, and — unlike the rest of this list — it was not searched for by name; see the note at the top of this answer.
- **The compositor's File Output node.** The metadata switches sit on the scene's render settings (`bpy.types.RenderSettings`); the File Output node's format settings are `bpy.types.ImageFormatSettings`, whose property list — cineon_black, cineon_gamma, cineon_white, color_depth, color_management, color_mode, compression, display_settings, exr_codec, file_format, has_linear_colorspace, jpeg2k_codec, linear_colorspace_settings, quality, stereo_3d_format, tiff_codec, use_cineon_log, use_jpeg2k_cinema_48, use_jpeg2k_cinema_preset, use_jpeg2k_ycc, use_preview, view_settings, views_format — contains nothing about metadata. The manual's File Output page never mentions metadata. Whether files written by that node carry the record is **not in the sources**.
- **Video files.** Five field descriptions say "image/video metadata" and the rest say "image metadata". What a rendered movie actually ends up holding is not stated anywhere I could find.
- **Whether saving an image by hand** (rather than rendering to a file path) writes the same record. The manual's Saving Images section lists format, colour, depth, compression, quality, Save As Render, Copy and Colour Space — and no metadata.
- **Anything Cycles' compiled part may add** through `stamp_data_add_field`. Its Python files say nothing; its compiled part is not in the folder I was given.

**Outside the interface.** **Settled by hand test, 2026-08-11: it holds.** The same blend-file was rendered twice on each machine — once with `-b` and no window at all, once from a Blender with its window open — and the two files carry exactly the same tags, name for name, with only the date and the render time differing as they must. That was true in 4.5 on machine one (the Creator's Mac, macOS 26.5.2, Apple M1 Pro) and on machine two (the rented Linux machine, Ubuntu 22.04.5), and the two machines agreed. One documented difference nearby turned out **not** to hold: the OpenEXR Preview option wrote a JPEG copy beside the EXR in *both* cases, not only from the command line. What follows is what the sources said before the tests, kept for the trail.

*The sources do not say.* Nothing in the manual, the API documentation or the release notes distinguishes what is written into a picture when Blender is driven from outside its own window from what is written when a person clicks Render. I searched the command-line arguments page for `stamp` and found only `--log-show-timestamp`, which concerns log lines and not pictures; there is no command-line switch for any of this. Two further points, and **both are inferred rather than read**. First, every setting above is a property of the scene's render settings (`bpy.types.RenderSettings`) or of the scene — that much is documented — but the step from there to *so it lives in the blend-file rather than in a window* is mine, not the documentation's. Second, the manual does draw a line between inside and outside for a different feature entirely — the OpenEXR "Preview" option, "When rendering animations (or single frames via command line), save a JPEG copy of the image, for a quick preview" (manual, image formats page, read 2026-07-30) — from which I reason that it would mark such a difference here if there were one. That is an argument from silence and carries no more weight than one. Note also that "Show Metadata" in the Image editor is a display overlay and produces nothing in any file; only the text fields and the burned-in text reach a file at all.

**Operating system.** **Settled by hand test for macOS and Linux, 2026-08-11; Windows is still open.** Every one of the nineteen tests was run in 4.5 on both machines, and every result came out the same on macOS 26.5.2 with Apple's own graphics and on Ubuntu 22.04.5 with NVIDIA cards — the same field names, the same formats keeping and dropping them, the same factory switches, the same behaviour of every setting. Two values differ, and only because they describe the machine: the `File` field holds that machine's own path, and the `Hostname` field holds that machine's own name — `Artists-Studio-MacBook.local` on machine one and `ubuntu` on machine two, each matching exactly what that machine's `hostname` command prints. **Windows was not tried: no Windows machine has run it**, so nothing here speaks for it. What follows is what the sources said before the tests, kept for the trail. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

*The sources do not say.* I searched the Metadata page and the image formats page for Windows, macOS and Linux and found no mention of any of them. So nothing on those two pages marks any field, any format's metadata support, or the burn-in as differing by operating system. **That search does not cover the pixel density**: I did not search the Output panel page, which is where the pixel density sits, nor any of the API pages. The Hostname field is described only as "the rendering machine's hostname", with no note about how that name is obtained on any particular system.

---

## What the hand tests showed — question 10

**LABEL for everything in this section: observed.** Added 2026-08-11 by the hand-test step, which ran a list of nineteen tests in Blender 4.5.12 on both of this project's machines. The full list and the full run record are in `Audits/Blender Research/Images/Hand tests — cross-sweep questions.md`.

**The two machines.** *Machine one is the Creator's Mac* — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card. Its `hostname` command prints `Artists-Studio-MacBook.local`, and its exiftool is version 13.50. *Machine two is a rented Linux machine* — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory. Its `hostname` prints `ubuntu`, and its exiftool is version 12.40. Every Blender was started with `--factory-startup` and with its own settings folder, so nothing saved was read.

**The scene used.** Blender's factory startup scene, with the scene renamed `SMZSCENE`, the camera renamed `SMZCAM` and its lens set to `37.5`, a marker named `SMZMARK`, frames 7 to 9 with frame 7 current, 64 × 64 pixels, Cycles at one sample, every `use_stamp_*` switch on, the note text `SMZNOTE-12345`, Burn Into Image off, saved as `smzfile.blend`. Each written file was read with `exiftool -a -G1 -s`, and each OpenEXR also with a header reader written for this round that walks every part of the file.

**Where the two machines agreed and where they did not.** Every result below came out the same on machine one and machine two, with one exception, and it is the one anybody would predict: the `File` field holds that machine's own path to the blend-file, and the `Hostname` field holds that machine's own name. Both are written as two results below.

### The fields do reach a file, and these are the names they carry

Written to PNG on machine one, the whole of what exiftool reported beyond the picture's own structure:

```
[PNG]           File                            : …/smzfile.blend
[PNG]           Note                            : SMZNOTE-12345
[PNG]           Date                            : 2026/08/12 06:22:52
[PNG]           Marker                          : SMZMARK
[PNG]           Time                            : 00:00:00:07
[PNG]           Frame                           : 7
[PNG]           FrameRange                      : 7:9
[PNG]           Camera                          : SMZCAM
[PNG]           Lens                            : 37.50
[PNG]           Scene                           : SMZSCENE
[PNG]           Strip                           : <none>
[PNG]           RenderTime                      : 00:00.04
[PNG]           Memory                          : 0.39M
[PNG]           Hostname                        : Artists-Studio-MacBook.local
[PNG]           CyclesViewLayersamples          : 1
[PNG]           CyclesViewLayertotal_time       : 00:00.04
[PNG]           CyclesViewLayerrender_time      : 00:00.04
[PNG]           CyclesViewLayersynchronization_time: 00:00.00
```

On machine two the same run gave the same names and the same values except `[PNG] File : /root/ht/smzfile.blend` and `[PNG] Hostname : ubuntu`. The two machines agree about every name and about every value that is not the machine's own.

So: the fourteen fields carry the names `File`, `Note`, `Date`, `Marker`, `Time`, `Frame`, `FrameRange`, `Camera`, `Lens`, `Scene`, `Strip`, `RenderTime`, `Memory`, `Hostname`. They are separate named entries, not one lump. `Strip` is written as `<none>` when there is no strip. In OpenEXR the same fourteen names appear as separate header attributes. In JPEG they appear as separate comment markers, each one reading `Blender:<name>:<value>` — for example `Blender:Note:SMZNOTE-12345`.

### Which formats actually keep them

The version offers `['BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX', 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP', 'FFMPEG']`. There is no AVIF, and there is no `media_type` setting. The same frame was written in every one of the still formats, on both machines.

- **Kept the fields: PNG, JPEG, OpenEXR (single-layer and multi-layer).**
- **Kept nothing: BMP, Iris, JPEG 2000, Targa, Targa Raw, Cineon, DPX, Radiance HDR, TIFF — and WebP.**

**WebP kept nothing**, on both machines, although the manual's format table marks it ✓ in the Metadata column. The whole of what exiftool printed for the WebP was `FileType`, `MIMEType`, `VP8Version`, `ImageWidth`, `HorizontalScale`, `ImageHeight` and `VerticalScale`. Nothing appeared in Blender's console about any format that could not carry the fields.

### Four fields nobody switched on, and they are Cycles'

Every file that keeps the fields also carries `cycles.ViewLayer.samples`, `cycles.ViewLayer.total_time`, `cycles.ViewLayer.render_time` and `cycles.ViewLayer.synchronization_time`. They are still there when **every** `use_stamp_*` switch is set to `False`, and they are absent from an otherwise identical EEVEE render. So Cycles does add fields of its own; the search of the supplied Cycles files above found nothing because the part of Cycles that adds them is not in that folder.

### With every switch off

With every `use_stamp*` switch set to `False`, a PNG still carried the four Cycles fields and the pixel density, and nothing else. **No tag anywhere named Blender, a version or a build** in a PNG, a JPEG, a WebP or a single-layer OpenEXR. A **multi-layer** OpenEXR is the exception, in this version and on both machines: it always carries

```
  [BlenderMultiChannel] (string, 25 bytes) = Blender V2.55.1 and newer
```

whatever the switches say.

### The multi-layer question — one record for the file

4.5 has no `use_exr_interleave` property (the script printed `HAS-use_exr_interleave: False`), and a multi-layer OpenEXR written by 4.5 is a single-part file — the header reader reports `multiPart=False`, `TOTAL PARTS: 1`. So the question of whether the record repeats per part does not arise here: there is one part, and it holds everything. The view layers' names appear inside the channel names, as `AAALAYER.Combined.R`, `AAALAYER.Mist.Z`, `BBBLAYER.Combined.R` and so on. Cycles' four fields are written once per view layer, as `cycles.AAALAYER.render_time`, `cycles.BBBLAYER.render_time` and so on.

### Burn Into Image

Rendered at 640 × 360 with the burn-in on, to PNG and to Targa. Both pictures carry the same text, in the same places: a block at the top left reading `File …`, `Date 2026/08/12 06:22:52`, `RenderTime 00:00.34`, `Peak Memory 16.79M`, `Hostname Artists-Studio-MacBook.local`, then `SMZNOTE-12345` on its own line with no label; a strip along the bottom reading `Marker SMZMARK Timecode 00:00:00:07 Frame 7 Frame Range 7:9 Camera SMZCAM Lens 37.50 Scene SMZSCENE`; and a `Strip <none>` item at the top right which at this width overlaps the end of the `File` line. The same on machine two.

Two things follow that were open above. The burned-in text **does** reach Targa, which keeps no fields. And burning in **adds to** rather than replaces: the PNG written in that same run still carried the whole block of named fields.

The burned-in labels are not the same words as the field names in the file: the file's `Memory` is drawn as `Peak Memory`, and the file's `Time` is drawn as `Timecode`.

### The compositor's File Output node

The node's file and the Output panel's file came out of one render. The node's file carries the same fourteen names, with three values different and one group missing:

```
[PNG]           Camera                          : <none>
[PNG]           Lens                            : <none>
[PNG]           RenderTime                      : 00:00.00
```

and none of the four `cycles.` fields. The panel's file from the same render carried `Camera : SMZCAM`, `Lens : 37.50`, a real render time, and the four Cycles fields. The same on both machines.

> **One of the three differences did not reproduce. Added by the citation check, 2026-08-11.** Run again in 4.5 on both machines, the node's file carried the same fourteen names with `Camera : <none>`, `Lens : <none>` and none of the four Cycles fields, exactly as above — but its render time was **the same as the panel file's**, `RenderTime : 00:00.04` in both, not `00:00.00`. A render time is a timing value and cannot be expected to repeat; what does not hold is that it is one of three *differences*. Both runs stand as written.

### Saving by hand from the Image editor

`Image ▸ Save As` from an Image Editor wrote a file whose list of tags is identical, name for name, to the file the render wrote itself. The same on both machines.

> **Repeatable only with a step this paragraph does not carry. Added by the citation check, 2026-08-11.** The result held when run again — a hand-saved file's tag list is identical, name for name, to the render's own, on both machines — but the `Image ▸ Save As` operator itself would not run: in a Blender with no window, and in a windowed Blender with an area switched to the Image Editor through a context override, it refused with `RuntimeError: Operator bpy.ops.image.save_as.poll() failed, context is incorrect`. The hand-test run record says the repair was to let the Image Editor draw once before calling it; that step is missing from this paragraph, so the result is not repeatable from these words alone. The re-run reached the same answer by `Image.save_render` instead.

### The factory state of the switches, and the Memory disagreement

On a factory-fresh 4.5 scene, on both machines:

```
FACTORY-VALUE use_stamp_memory             = False
```

while the property's own stated default is `True` (`id=use_stamp_memory  name='Stamp Peak Memory'  type=BOOLEAN  default=True`). So the manual's screenshot is right about a fresh Blender and the API's stated default is right about the property; they are describing two different things. The whole factory list: on — Date, Time, Render Time, Frame, Camera, Scene, Filename, Include Labels; off — Frame Range, Memory, Hostname, Lens, Marker, Strip Name, Note, Burn Into Image.

### The cut-off label in the manual's screenshot

**No property in 4.5 has an interface name beginning "Use Strip Me".** The panel's own drawing code, read out of the running Blender, offers only two sequencer-related controls, and both appear only when the Video Sequencer is in use:

```
        if rd.use_sequencer:
            layout.prop(rd, "metadata_input")
        …
        if rd.use_sequencer:
            col.prop(rd, "use_stamp_sequencer_strip", text="Strip Name")
```

So whatever the manual's screenshot shows, 4.5 does not draw it.

### The two note properties

Both API descriptions quoted anywhere are right, about different things:

```
### bpy.types.RenderSettings
  id=use_stamp_note      name='Stamp Note'       type=BOOLEAN  default=False
      description='Include a custom note in image/video metadata'
  id=stamp_note_text     name='Stamp Note Text'  type=STRING   default=''
      description='Custom text to appear in the stamp note'
### bpy.types.Scene
  id=use_stamp_note      name='Stamp Note'       type=STRING   default=''
      description='User defined note for the render stamping'
```

### Metadata taken from the Sequencer's strips

With `metadata_input` set to `STRIPS` and two image strips in the Video Sequencer, the written file's block is **replaced by the block read out of the foreground strip's own image file** — its date, its render time, its memory figure and its Cycles fields — and `Strip` reads `<none>`. With `SCENE` the date is the render's own, `Strip` reads `SMZSTRIP2`, and there are no Cycles fields. The same on both machines.

> **The `STRIPS` half did not reproduce. Added by the citation check, 2026-08-11.** Run again on both machines — two image strips, `SMZSTRIP1` on channel 1 and `SMZSTRIP2` on channel 2, both pointing at a PNG written by the same session, the sequencer on, frame 7 — the file written with `metadata_input` set to `STRIPS` carried **no Blender metadata at all**: the whole of what exiftool reported was the picture's own structure (`ImageWidth`, `ImageHeight`, `BitDepth`, `ColorType`, `Compression`, `Filter`, `Interlace`, `ImageOffset`) and nothing else. The `SCENE` half reproduced exactly, `Strip : SMZSTRIP2` included. Both runs stand as written; neither settles the other. The test has gone back to the hand-test list as test 20.

**Run a third time, 2026-08-12 — the `STRIPS` file carried no Blender field at all.** Test 20 on the hand-test list, run in Blender 4.5.12 on **machine one, the Creator's Mac** (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card) and on **machine two, a rented Linux machine** (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory). What was done: the standard scene; frame 7 rendered to a PNG first, in the same Blender session, carrying its own full block; two image strips, `SMZSTRIP1` on channel 1 and `SMZSTRIP2` on channel 2, both pointing at that PNG; `render.use_sequencer` on; format PNG; frame 7 rendered twice more, once with `metadata_input = 'STRIPS'` and once with `'SCENE'`. Both strips printed `frame_final_start 7 frame_final_duration 1`. The source PNG's bytes and size were the same at every check before and after both renders, so it still carried its own fields at the moment of the render.

The whole of what `exiftool -a -G1 -s` printed for the `STRIPS` file on machine one, from `[PNG]` to the end, uncut (above it sit only exiftool's own version line and the file's system and type lines):

```
[PNG]           ImageWidth                      : 64
[PNG]           ImageHeight                     : 64
[PNG]           BitDepth                        : 8
[PNG]           ColorType                       : RGB with Alpha
[PNG]           Compression                     : Deflate/Inflate
[PNG]           Filter                          : Adaptive
[PNG]           Interlace                       : Noninterlaced
[PNG]           ImageOffset                     : 0, 0 (pixels)
[IFD0]          XResolution                     : 72
[IFD0]          YResolution                     : 72
[PNG-pHYs]      PixelsPerUnitX                  : 2834
[PNG-pHYs]      PixelsPerUnitY                  : 2834
[PNG-pHYs]      PixelUnits                      : meters
[Composite]     ImageSize                       : 64x64
[Composite]     Megapixels                      : 0.004
```

Not one Blender field, and no `Strip` line. The `SCENE` file from the same session carried the fourteen, including `Strip : SMZSTRIP2`, `RenderTime : 00:00.00`, `Memory : 0.39M`, `Hostname : Artists-Studio-MacBook.local`, and none of the four `cycles.` fields. The source PNG carried those fourteen with `Strip : <none>`, `RenderTime : 00:00.12` and the four Cycles fields as well — eighteen in all.

**Machine two agreed.** Its `STRIPS` file printed the same list, and its bytes hash to the same value as machine one's, because the file holds nothing that names either machine. Its `SCENE` file reads `Hostname : ubuntu` and `File : /root/ht2/smzfile.blend`.

Counting the Blender fields in each of the three files, on both machines: source 18, `STRIPS` 0, `SCENE` 14. **This run and the citation check's run came out the same; the first run came out differently.** All three stand as written and none settles the others. The full run record is in `Audits/Blender Research/Images/Hand tests — cross-sweep questions.md`.

### Pixel density

With the density set to 300: PNG carries `PixelsPerUnitX : 11811` and `PixelsPerUnitY : 11811`; BMP carries `PixelsPerMeterX : 11811` and `PixelsPerMeterY : 11811`; TIFF carries `XResolution : 300`, `YResolution : 300`; OpenEXR carries one number, `xDensity : 300`; **JPEG carries none** (`ResolutionUnit : None`, `XResolution : 1`); WebP carries none. With the pixel aspect set to 2 across by 1 down, PNG and BMP write the two directions separately (150 and 300), **TIFF writes both as 150**, and OpenEXR still writes one number.

### A rendered movie

An MPEG-4 with H.264 over frames 7 to 9, every switch on, carries **none of the fourteen fields under any name**. The whole of what it holds about its making:

```
[Track1]        ColorProfiles                   : nclx
[Track1]        ColorPrimaries                  : BT.709
[Track1]        TransferCharacteristics         : BT.709
[Track1]        MatrixCoefficients              : BT.709
[Track1]        VideoFullRangeFlag              : Limited
[ItemList]      ContentCreateDate               : 2026/08/12 06:22:59
[ItemList]      Encoder                         : Lavf61.7.100
```

`Lavf` is the FFmpeg library, not Blender. So the API's split between "image metadata" and "image/video metadata" is not borne out in an MPEG-4 written by 4.5: none of the five reaches it either.

A side-by-side Stereo 3D movie written by 4.5 is named `stereo_0007-0009_L.mp4` — with an `_L` on the end — and is 128 × 64, so both eyes are in the picture. Neither it, nor an equirectangular panoramic movie, carries any stereo or projection marking: searching each file for `st3d`, `sv3d`, `svhd`, `prji`, `proj`, `equi` and `ytmp` found none of them.

### A field an engine adds for itself

A render engine registered for the test called `result.stamp_data_add_field("SMZFIELD", "SMZVALUE-98765")` on the render result and the call succeeded. Neither `SMZFIELD` nor `SMZVALUE-98765` appears anywhere in the PNG that render wrote. So a field added to the render result **does not reach the written picture** — while the scene's own fourteen, in the same file, do.

### Colour

No PNG written by 4.5 carried an ICC profile or any colour tag at all, at the default colour management or with the view transform set to `Standard`. No OpenEXR written by 4.5 carried any colour-space attribute. The same on both machines.

### Cryptomatte

A multi-layer OpenEXR with Cryptomatte on carries four header attributes per layer:

```
  [cryptomatte/542cafa/conversion] (string, 17 bytes) = uint32_to_float32
  [cryptomatte/542cafa/hash] (string, 14 bytes) = MurmurHash3_32
  [cryptomatte/542cafa/manifest] (string, 58 bytes) = {"Cube":"a8fce865","Light":"c77d43e5","object":"055ec73d"}
  [cryptomatte/542cafa/name] (string, 22 bytes) = ViewLayer.CryptoObject
```

### Cycles' own debug passes

4.5 has **no `pass_render_time` property at all**: `PASS-SET-ERROR: pass_render_time AttributeError 'CyclesRenderLayerSettings' object has no attribute 'pass_render_time'`. Its Sample Count pass writes a channel named `ViewLayer.Debug Sample Count.X`, on the processor and on the graphics card alike (Apple's own graphics through Metal on machine one, one NVIDIA RTX 4000 Ada through OptiX on machine two).

---

### Where I looked

Searched, case-insensitively, across the whole of `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5` unless noted:

- `metadata` — hits in the manual at render/output/properties/metadata, render/output/properties/index, render/output/index, files/media/image_formats, editors/image/sidebar, editors/image/overlays, editors/image/navigating, editors/video_sequencer/preview/sidebar, editors/video_sequencer/preview/display/overlays, compositing/types/mask/cryptomatte (a limitation about sidecar files, nothing about what is written), plus asset-metadata pages unrelated to pictures. In the API, `bpy.types.RenderSettings` and asset/volume/file-browser pages unrelated to pictures. In the release notes, `pipeline_assets_io.md` and `user_interface.md` (the latter about brush assets).
- `stamp` — in the API, only `bpy.types.RenderSettings`, `bpy.types.RenderResult`, `bpy.types.Scene` and the index pages. In the manual, only the Metadata page (plus unrelated uses: brush textures, timecode timestamps, Freestyle time stamps).
- `burn` — only the Metadata page and its index; every other hit is a colour blend mode or fluid fuel.
- `exif`, `Exif` — two hits, both about reading files, neither about writing: the Sequencer preview sidebar's exiftool tip, and a movie-clip tracking page.
- `xmp` — one hit, in asset libraries, unrelated to rendered pictures.
- `iptc` — no hits anywhere.
- `tEXt`, `PNG chunk`, `file header`, `header field`, `attribute name`, `key/value`, `key-value` — no hit that concerns what Blender writes into a picture. `header field` returns only the Sequencer sidebar's "Comments" sentence.
- `provenance`, `authoring tool`, `software tag`, `version string`, `producer` — nothing about a picture recording which program or which Blender version made it.
- `ppm`, `pixel density`, `dpi`, `ppi` — the Output panel page, the image formats table, the glossary, and `release_notes_4.5/rendering.md`.
- `headless` — two EEVEE limitation pages, nothing about metadata.
- `windows` / `macos` / `linux` on the Metadata page and the image formats page — no hits.
- Read in full: manual Metadata page, manual Output panel page, manual image formats page, `bpy.types.RenderResult`, and the relevant stretches of `bpy.types.RenderSettings` and `bpy.types.ImageFormatSettings`.
- Opened: `manual_4.5/_images/render_output_properties_metadata_panel.png`.

That is what makes "not in the sources" a real answer for the items listed under NOT COVERED, rather than a blank — with two exceptions, both marked where they appear. The **per-part** reading was never searched for by name. And the **operating-system** line was searched on two manual pages only, which leaves the pixel density outside it.

---

## Things a person with Blender open could settle in a few minutes

The ten items that stood here were all run on 2026-08-11, in 4.5 on both of this project's machines, and their answers are in the section above. What is left open needs a machine this project does not have.

1. **Whether any of this differs on Windows.** Every result above was got on macOS 26.5.2 and on Ubuntu 22.04.5, and the two agreed about everything except the two values that describe the machine itself. **This was not run: no Windows machine has run it.** Until it has one, nothing above speaks for Windows.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Images, Blender 4.5

# Images — cross-sweep questions, second pass (Blender 4.5)

**Sources consulted, and when.** Blender's 4.5 manual, Blender's 4.5 Python API documentation, the Blender 4.5 release notes, and the Cycles 4.5 files, all as supplied in `Blender Sources/`. Snapshot taken 2026-07-30; that is the date read for everything quoted below. I also opened one picture from the manual (`_images/render_output_properties_metadata_panel.png`). Work done 2026-08-11.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

I answered question 10 only. I did not start any helper agents, so there is nobody else's work to vouch for. I did not use the web.

---

## 10. What does a finished picture hold, in every part of the file, about what produced it?

### The question was ambiguous, and which reading I took

I read "in every part of the file" as **everywhere in the file** — both the part of the file that holds descriptive text about the picture, and the pixels of the picture itself. That is the reading I answer.

The other reading is narrower and technical: "part" is a term of art in the OpenEXR format, where one file can hold several named parts. Under that reading the question would be asking whether Blender puts the same descriptive text into every part of a multi-part OpenEXR file. **The sources say nothing about that**, and I record below where I looked for it.

### Short answer

A rendered file can hold two separate kinds of record of what produced it:

1. **Descriptive text written alongside the picture** ("metadata"), listing things like the date, the render time, the frame, the camera, the scene, the blend-file name and the machine's hostname. Only four of Blender's image formats can carry it.
2. **The same text painted into the pixels** ("Burn into Image"), which is off by default and which, being pixels, survives in any format.

There is also a third, much smaller item: the intended pixel density, which the manual calls metadata in so many words.

Beyond these, the sources do not say that a Blender render records anything else about itself — not Blender's own version, not the render engine, not the sample count, not the resolution, not the asset paths.

---

### A. The descriptive text, item by item — **documented**

The manual has exactly one page on this, the Metadata panel of Output Properties. Its list, quoted:

> "The Metadata panel includes options for writing metadata into render output."
>
> "Metadata Input — Where to grab metadata from. Scene: Use metadata from the current scene. Sequencer Strips: Use metadata from the strips in the Sequencer."
>
> "Include
> Date — Includes the current date and time.
> Time — Includes the current scene time and render frame at HH:MM:SS.FF.
> Render Time — Includes the render time.
> Frame — Includes the frame number.
> Frame Range — Includes the start and end frame numbers.
> Memory — Includes the peak memory usage.
> Hostname — Includes the rendering machine's hostname.
> Camera — Includes the name of the active camera.
> Lens — Includes the name of the active camera's lens value.
> Scene — Includes the name of the active scene.
> Marker — Includes the name of the last marker.
> Filename — Includes the filename of the blend-file.
> Strip Name — Includes the name of the foreground sequence strip.
> Note — Includes a custom note."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html — read 2026-07-30.

The same page adds a hint aimed at exactly this kind of use:

> "It can be useful to use the Note field if you are setting up a render farm. Since you can script any information you like into it, such as an identifier for the render node or the job number. For details on stamping arbitrary values, see: this page."

Same page and date. (The plain-text copy does not carry the target of "this page", so I cannot say where that link goes.)

The Python API repeats the list with the switch names and their factory settings. Quoted exactly, from https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30:

| Switch | The documentation's own words | Default |
| --- | --- | --- |
| `use_stamp_date` | "Include the current date in image/video metadata" | `True` |
| `use_stamp_time` | "Include the rendered frame timecode as HH:MM:SS.FF in image metadata" | `True` |
| `use_stamp_render_time` | "Include the render time in image metadata" | `True` |
| `use_stamp_frame` | "Include the frame number in image metadata" | `True` |
| `use_stamp_frame_range` | "Include the rendered frame range in image/video metadata" | `False` |
| `use_stamp_memory` | "Include the peak memory usage in image metadata" | `True` |
| `use_stamp_hostname` | "Include the hostname of the machine that rendered the frame" | `False` |
| `use_stamp_camera` | "Include the name of the active camera in image metadata" | `True` |
| `use_stamp_lens` | "Include the active camera's lens in image metadata" | `False` |
| `use_stamp_scene` | "Include the name of the active scene in image/video metadata" | `True` |
| `use_stamp_marker` | "Include the name of the last marker in image metadata" | `False` |
| `use_stamp_filename` | "Include the .blend filename in image/video metadata" | `True` |
| `use_stamp_sequencer_strip` | "Include the name of the foreground sequence strip in image metadata" | `False` |
| `use_stamp_note` | "Include a custom note in image/video metadata" | `False` |
| `stamp_note_text` | "Custom text to appear in the stamp note" — "string, default \"\", (never None)" | empty |
| `metadata_input` | "Where to take the metadata from" — "enum in ['SCENE', 'STRIPS'], default 'SCENE'" | `SCENE` |

Worth noticing, because it is the documentation's own wording and it is not uniform: five entries say **"image/video metadata"** (date, frame range, scene, filename, note) while the rest say **"image metadata"** only. Taking that difference as meaning those five and only those five reach a video file would be **inferred**, and the sources never state it outright, so I do not claim it.

**Where the picture that follows adds something.** The manual's screenshot of the panel (`_images/render_output_properties_metadata_panel.png`, on the page above) shows the panel's own labels: Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, "Use Strip Me.." (truncated in the panel), then a Note section with a checkbox and a text field, then a Burn Into Image section with Font Size (showing "12 px"), Text Color, Background and Include Labels. In that screenshot Date, Time, Render Time, Frame, Camera, Scene and Filename are ticked; Frame Range, Memory, Hostname, Lens, Marker, Strip Name and "Use Strip Me.." are not; Include Labels is ticked; Note and Burn Into Image are not. **documented** (it is part of the manual), with one caution: the screenshot shows Memory **un**ticked while the API says `use_stamp_memory` defaults to `True`, so the screenshot is not a reliable statement of the factory settings. I report both and do not resolve them.

### B. Text a render engine adds for itself — **documented**

Python can push extra fields into the render result:

> "`stamp_data_add_field(field, value)` — Add engine-specific stamp data to the result. Parameters: field (string, (never None)) – Field, Name of the stamp field to add. value (string, (never None)) – Value, Value of the stamp data"

Source: API, https://docs.blender.org/api/4.5/bpy.types.RenderResult.html — read 2026-07-30.

So the list in section A is not closed: an engine or a script can add named fields of its own.

**Does Cycles use it?** In the Cycles files supplied I searched every file for `stamp`, `stamp_data`, `metadata`, `use_stamp` and `metadata_input`. The only hits for `metadata` are in `cycles_4.5/osl.py` (lines 58–62, 98–155), where the word means an OSL shader parameter's own metadata and has nothing to do with image files; there are no hits at all for `stamp`. **observed.** The caution matters: what is in that folder is the Cycles add-on's Python (`__init__.py`, `camera.py`, `engine.py`, `operators.py`, `osl.py`, `presets.py`, `properties.py`, `ui.py`, `version_update.py`), plus kernel headers, OSL shaders and two device `.cpp` files — 399 files, no render-session C++. So this shows Cycles' Python adds no stamp field; it does not show that Cycles adds none at all.

### C. Which formats can hold the descriptive text at all — **documented**

The manual's format table has a Metadata column and a DPI column. Quoting the table's entries:

- Metadata "✓": **PNG**, **JPEG**, **OpenEXR**, **WebP**.
- Metadata "✗": **BMP**, **Iris**, **JPEG 2000**, **Targa**, **Cineon & DPX**, **Radiance HDR**, **TIFF**.
- DPI "✓": **BMP**, **PNG**, **JPEG**, **OpenEXR**, **TIFF**. DPI "✗": Iris, JPEG 2000, Targa, Cineon & DPX, Radiance HDR, WebP.

Source: manual, https://docs.blender.org/manual/en/4.5/files/media/image_formats.html — read 2026-07-30.

The Metadata panel points at that same table:

> "Note — Only some image formats support metadata: See image formats."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html — read 2026-07-30.

What the sources do **not** say is what happens to the text when the chosen format cannot hold it. No page states that it is dropped, warned about, or diverted. Concluding that it is simply lost would be **inferred**, and I do not make the claim.

### D. Pixel density is also part of what the file holds — **documented**

> "The pixel density (often called PPI or DPI) controls the intended physical size for an image. This is often used for printing or the physical size when loaded into desktop publishing software. The X/Y density is calculated using the render X/Y aspect, making these values useful for storing the aspect ratio of non-square pixels. The pixel-density is meta-data which doesn't impact the quality of the image."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30.

### E. One colour item, for OpenEXR only — **documented**

The 4.5 release notes record that OpenEXR files now carry a note of one particular colour space:

> "OpenEXR:
>   - Read and write ACES2065-1 colorspace metadata, to auto detect this colorspace in Blender and other applications. (blender/blender!135823)"

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/ — read 2026-07-30.

The manual states the reading half of the same thing:

> "When opening OpenEXR images, Blender will automatically set the Color Space if it is detected as Linear CIE-XYZ E or ACES2065-1."

Source: manual, https://docs.blender.org/manual/en/4.5/files/media/image_formats.html — read 2026-07-30.

I mention this only because it is something the file holds about how it was made. A neighbouring question on the sheet covers colour treatment properly, and I have not gone further into it here.

### F. In the pixels: Burn into Image — **documented**, with one inferred step

> "Burn into Image — Add metadata as text to the render.
> Font Size — Set the size of the text.
> Text Color — Set the color and alpha of the stamp text.
> Background — Set the color and alpha of the color behind the text.
> Include Labels — Displays the labels before the metadata text. For example, "Camera" in front of the camera name, etc."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html — read 2026-07-30.

The matching API entries, from https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html (read 2026-07-30):

- `use_stamp` — "Render the stamp info text in the rendered image" — "boolean, default False"
- `use_stamp_labels` — "Display stamp labels ("Camera" in front of camera name, etc.)" — "boolean, default True"
- `stamp_font_size` — "Size of the font used when rendering stamp text" — "int in [8, 64], default 12"
- `stamp_foreground` — "Color to use for stamp text" — "float array of 4 items in [0, 1], default (0.8, 0.8, 0.8, 1.0)"
- `stamp_background` — "Color to use behind stamp text" — "float array of 4 items in [0, 1], default (0.0, 0.0, 0.0, 0.25)"

Two steps here are **inferred**, and I write them out:

1. That `use_stamp` is the switch behind the panel's "Burn Into Image" is not stated anywhere. I join them because the panel screenshot shows "Burn Into Image" as an unticked checkbox with Font Size, Text Color, Background and Include Labels beneath it, and `use_stamp` is the only boolean whose description mentions rendering the text into the image and whose default (`False`) matches the unticked box, with the four sub-settings sitting beside it in the API list.
2. That burnt-in text therefore reaches every format, including the seven that cannot hold the descriptive text, follows from "Add metadata as text to the render" — if it is part of the render, it is part of the pixels, and pixels are what every format stores. The sources never say this outright.

Note also that the default is off: nothing is burnt into a picture unless somebody turns it on.

### G. Reading it back — **documented**

Inside Blender:

> "Show Metadata — Displays metadata about the selected Render Result. See the Output tab's Metadata panel to change what metadata to include."

Source: manual, https://docs.blender.org/manual/en/4.5/editors/image/overlays.html — read 2026-07-30. The same sentence appears at https://docs.blender.org/manual/en/4.5/editors/image/navigating.html, and at https://docs.blender.org/manual/en/4.5/editors/uv/overlays.html with one word changed — that page reads "Display metadata about the selected Render Result", not "Displays". *(Corrected by the citation check, 2026-08-11: the sentence was said to appear unchanged on all three pages.)* The Image editor sidebar also has a Metadata panel: "Lists image metadata." (https://docs.blender.org/manual/en/4.5/editors/image/sidebar.html).

The Video Sequencer's page is the most explicit about what the text amounts to, and about its limits:

> "Lists information that has been encoded in the currently visible movie or image file (not the file referenced by the selected strip). This can include the filename, the creation date, the camera model etc. This also works for images produced by Blender; see Render Output for the metadata that can be included in this case."
>
> "Other graphics programs may also store metadata, but only the text in the header field "Comments" can be read."
>
> "The metadata can't be edited from Blender. Instead, you can use an external program such as exiftool. For example, the command to change the "Comments" field is: `exiftool --comments="My new comment" name-of-file.png`"

Source: manual, https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/sidebar.html — read 2026-07-30.

From Python, there is no way to read a still image's metadata back: `bpy.types.Image` has no metadata member at all (I searched the whole page for "metadata" and "stamp" and found neither). The only such reader in the API is for movies — `MovieClip.metadata()`, "Retrieve metadata of the movie file", https://docs.blender.org/api/4.5/bpy.types.MovieClip.html, read 2026-07-30. **documented** for what exists; the absence in `bpy.types.Image` is **not in the sources** in the sense that nothing states the limitation, only that no such member is listed.

### H. What the sources are silent about — **not in the sources**

Nothing in the manual, the API documentation, the release notes or the Cycles files says that a rendered picture records:

- Blender's own version or name as the producing program
- which render engine produced it, or its settings (samples, denoiser, device)
- the resolution, resolution percentage, or render region used
- the view transform or look applied, other than the OpenEXR ACES2065-1 item in section E
- the paths of the textures, libraries or assets the scene used
- any per-part or per-layer placement of the text inside a multi-layer or multi-part OpenEXR file
- the underlying tag or attribute names used in each format (which PNG chunk, which EXR attribute, which JPEG marker) — the only named field anywhere is "Comments", and that is named as something Blender **reads** from other programs' files, not as something it writes
- whether the text is written when an image is saved by hand from the Image editor, as opposed to written by a render

### Not covered

**Every item in this list was settled by hand test on 2026-08-11 — see "What the hand tests showed" below.** What follows is the list as the researcher left it.

- I cannot say what a real file actually contains; everything above is what the documentation says should be there. Nothing here was checked against an output file.
- I cannot say what happens to the text when the chosen format cannot carry it.
- I cannot say which of the fields reach a video file rather than a still, beyond quoting the documentation's own uneven wording.
- I cannot say what the fields are named inside the file, so I cannot say what another program would show as their names.
- I cannot say whether the panel screenshot shows factory settings; it disagrees with the API on Memory.
- The Cycles finding covers only the files supplied, which do not include the render-session code.

### Outside the interface

**Settled by hand test, 2026-08-11: it holds.** The same blend-file was rendered twice on each machine — once with `-b` and no window at all, once from a Blender with its window open — and the two files carry exactly the same tags, name for name, with only the date and the render time differing as they must. That was true in 4.5 on machine one (the Creator's Mac, macOS 26.5.2, Apple M1 Pro) and on machine two (the rented Linux machine, Ubuntu 22.04.5, two NVIDIA RTX 4000 Ada cards), and the two machines agreed. So the weak inference below turns out to be right, and the near-miss it leaned on was a good guide. One documented difference nearby did **not** hold: the OpenEXR Preview option wrote a JPEG copy beside the EXR in *both* cases, not only from the command line. What follows is what the sources said before the tests, kept for the trail.

**Not in the sources.** No page states whether the descriptive text and the burnt-in text are written when Blender is started and driven by something other than a person at the window.

What the sources do give is a near-miss worth recording. On the same API page, one setting carries an explicit warning of exactly this kind — `use_single_layer`: "Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line." None of the sixteen stamp settings carries any such note, and neither does the Metadata page of the manual. The settings themselves live on `Scene.render` (`bpy.types.Scene` → `render`, "Type: RenderSettings, (readonly, never None)"), which is scene data. **Inferred**, and weakly: because they are scene data with no stated interface-only limit, they would be expected to apply to any render of that scene however it was started. A reader has to supply that step; the sources do not.

Searched for this specifically: the whole command-line arguments page for "stamp" and "metadata" (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html) — the only hit is `--log-show-timestamp`, "Show a timestamp for each log message in seconds since start", which is about log output and not about image files.

### Operating system

**Settled by hand test for macOS and Linux, 2026-08-11; Windows is still open.** Every one of the nineteen tests was run in 4.5 on both machines, and every result came out the same on macOS 26.5.2 with Apple's own graphics and on Ubuntu 22.04.5 with NVIDIA cards. Two values differ, and only because they describe the machine: the `File` field holds that machine's own path, and `Hostname` holds that machine's own name — `Artists-Studio-MacBook.local` on machine one and `ubuntu` on machine two, each matching exactly what that machine's `hostname` command prints. So the flag raised below is answered: the field is filled the same way on both, with each machine's own name. **Windows was not tried: no Windows machine has run it**, so nothing here speaks for it. What follows is what the sources said before the tests, kept for the trail. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Checked again on 2026-08-12 and still holding for macOS and Linux.** The second batch of hand tests ran the Sequencer-strips test again in 4.5.12 on both machines, and the two agreed about every field in all three files it wrote. **Windows is still untested for the same reason.**

**Not in the sources.** No page states any operating-system difference in what a rendered file records. I searched the Output pages, the image formats page and the Image editor pages for "Windows", "macOS" and "Linux"; the only hits are in the Image editor's editing page and concern pasting images from the clipboard, not saved output.

One field is worth flagging as the likely place a difference would show if there is one: Hostname, "Includes the rendering machine's hostname". The sources say nothing about how that name is obtained or whether it differs between systems.

---

## What the hand tests showed — question 10

**LABEL for everything in this section: observed.** Added 2026-08-11 by the hand-test step, which ran a list of nineteen tests in Blender 4.5.12 on both of this project's machines. The full list and the full run record are in `Audits/Blender Research/Images/Hand tests — cross-sweep questions.md`.

**The two machines.** *Machine one is the Creator's Mac* — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card. Its `hostname` command prints `Artists-Studio-MacBook.local`, and its exiftool is version 13.50. *Machine two is a rented Linux machine* — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory. Its `hostname` prints `ubuntu`, and its exiftool is version 12.40. Every Blender was started with `--factory-startup` and with its own settings folder.

**The scene used.** Blender's factory startup scene, with the scene renamed `SMZSCENE`, the camera renamed `SMZCAM` and its lens set to `37.5`, a marker named `SMZMARK`, frames 7 to 9 with frame 7 current, 64 × 64 pixels, Cycles at one sample, every `use_stamp_*` switch on, the note text `SMZNOTE-12345`, Burn Into Image off, saved as `smzfile.blend`. Each written file was read with `exiftool -a -G1 -s`, and each OpenEXR also with a header reader written for this round that walks every part of the file.

**Where the two machines agreed and where they did not.** Every result below came out the same on machine one and machine two, with one exception: the `File` field holds that machine's own path to the blend-file, and the `Hostname` field holds that machine's own name. Both are given below.

### What actually lands in a file, and under what names

Section A above said what Blender's documentation intends to write. This is what a written PNG holds, on machine one:

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

On machine two the same names and values appeared, with `[PNG] File : /root/ht/smzfile.blend` and `[PNG] Hostname : ubuntu`.

They are separate named entries, not one lump. In OpenEXR the same fourteen names appear as separate header attributes. In JPEG they appear as separate comment markers, each reading `Blender:<name>:<value>` — for example `Blender:Note:SMZNOTE-12345`.

### Which formats keep them — the table is wrong about WebP

The version offers `['BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX', 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP', 'FFMPEG']` — no AVIF, and no `media_type` setting. The same frame was written in every still format on both machines.

- **Kept the fields: PNG, JPEG, OpenEXR (single-layer and multi-layer).**
- **Kept nothing: BMP, Iris, JPEG 2000, Targa, Targa Raw, Cineon, DPX, Radiance HDR, TIFF — and WebP.**

**WebP kept nothing**, on both machines, although Section C's reading of the manual's table marks it ✓ for Metadata. The whole of what exiftool printed for it was `FileType`, `MIMEType`, `VP8Version`, `ImageWidth`, `HorizontalScale`, `ImageHeight` and `VerticalScale`. Nothing appeared in Blender's console about a format that could not carry the fields, so Section C's open question — what happens when the format cannot hold it — is answered: the fields are simply absent, and nothing is said.

### Section B answered: Cycles does add fields of its own

Every file a Cycles render writes carries `cycles.ViewLayer.samples`, `cycles.ViewLayer.total_time`, `cycles.ViewLayer.render_time` and `cycles.ViewLayer.synchronization_time`. They survive with **every** `use_stamp*` switch set to `False`, and they are absent from an otherwise identical EEVEE render. The caution in Section B was right: the part of Cycles that adds them is not in the supplied folder.

### Section H answered: what is written unconditionally

With every `use_stamp*` switch off, a PNG still carried the four Cycles fields and the pixel density, and nothing else. **No tag named Blender, a version or a build** in a PNG, a JPEG, a WebP or a single-layer OpenEXR. A **multi-layer** OpenEXR is the one exception, and it holds whatever the switches say:

```
  [BlenderMultiChannel] (string, 25 bytes) = Blender V2.55.1 and newer
```

Nothing anywhere named the render engine, the sample count, the resolution, the view transform, or any asset path.

### The per-part reading of the question

4.5 has no `use_exr_interleave` property (`HAS-use_exr_interleave: False`), and a multi-layer OpenEXR written by 4.5 is a **single-part** file — the header reader reports `multiPart=False`, `TOTAL PARTS: 1`. So in 4.5 the question does not arise: there is one part and it holds everything. The view layers' names appear inside the channel names (`AAALAYER.Combined.R`, `AAALAYER.Mist.Z`, `BBBLAYER.…`), and Cycles' four fields are written once per view layer (`cycles.AAALAYER.render_time` and so on).

### Section F answered: burnt-in text does reach a format that keeps nothing

Rendered at 640 × 360 with the burn-in on, to PNG and to Targa. Both pictures carry the same text: a block at the top left reading `File …`, `Date 2026/08/12 06:22:52`, `RenderTime 00:00.34`, `Peak Memory 16.79M`, `Hostname Artists-Studio-MacBook.local`, then `SMZNOTE-12345` on its own line with no label; a strip along the bottom reading `Marker SMZMARK Timecode 00:00:00:07 Frame 7 Frame Range 7:9 Camera SMZCAM Lens 37.50 Scene SMZSCENE`; and a `Strip <none>` item at the top right which at this width overlaps the end of the `File` line. The same on machine two. The inference in Section F holds.

Burning in **adds to** rather than replaces: the PNG from that same run still carried the whole block of named fields. And the burnt-in labels are not the field names in the file — the file's `Memory` is drawn as `Peak Memory`, and the file's `Time` as `Timecode`.

### Saving by hand, and the File Output node

`Image ▸ Save As` from an Image Editor wrote a file whose tag list is identical, name for name, to the file the render wrote itself. The compositor's File Output node also writes the fourteen names, from the same render, but with `Camera : <none>`, `Lens : <none>`, `RenderTime : 00:00.00`, and none of the four Cycles fields. The same on both machines.

> **Two riders from the citation check, 2026-08-11.** First, run again in 4.5 on both machines the node's file carried the same fourteen names with `Camera : <none>`, `Lens : <none>` and none of the Cycles fields — but its render time was the same as the panel file's (`00:00.04` in both), not `00:00.00`, so that third difference did not reproduce. Second, the hand-saved result held, but the `Image ▸ Save As` operator would not run without a step this paragraph does not carry: it refused with `RuntimeError: Operator bpy.ops.image.save_as.poll() failed, context is incorrect` in both a background and a windowed Blender, and the re-run reached the same tag list by `Image.save_render` instead.

### The Memory disagreement, settled

On a factory-fresh 4.5 scene, on both machines, `use_stamp_memory` reads `False`, while the property's own stated default is `True`. The manual's screenshot describes a fresh Blender; the API's stated default describes the property. Both are right about different things, which is why they look like a contradiction.

### The truncated tick-box, and the two note properties

**No property in 4.5 has an interface name beginning "Use Strip Me".** The panel's own drawing code, read out of the running Blender, offers only `metadata_input` and `use_stamp_sequencer_strip` (drawn as "Strip Name"), and both only when the Video Sequencer is in use. The two note properties are different things: `RenderSettings.use_stamp_note` is a boolean, `RenderSettings.stamp_note_text` is the text, and `Scene.use_stamp_note` is a *string* described as "User defined note for the render stamping".

### Sequencer Strips, settled

With `metadata_input` set to `STRIPS` and two image strips in the Video Sequencer, the written file's block is **replaced by the block read out of the foreground strip's own image file** — its date, its render time, its memory figure and its Cycles fields — and `Strip` reads `<none>`. With `SCENE` the date is the render's own and `Strip` reads `SMZSTRIP2`.

> **The `STRIPS` half did not reproduce. Added by the citation check, 2026-08-11.** Run again on both machines, with two image strips (`SMZSTRIP1` on channel 1, `SMZSTRIP2` on channel 2) pointing at a PNG written by the same session, the file written with `metadata_input` set to `STRIPS` carried **no Blender metadata at all** — only the picture's own structure. The `SCENE` half reproduced exactly. Both runs stand as written; neither settles the other. Back on the hand-test list as test 20.

**Run a third time, 2026-08-12 — the `STRIPS` file carried no Blender field at all.** Test 20 on the hand-test list, run in Blender 4.5.12 on **machine one, the Creator's Mac** (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card) and on **machine two, a rented Linux machine** (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory). What was done: the standard scene; frame 7 rendered to a PNG first, in the same Blender session, carrying its own full block; two image strips, `SMZSTRIP1` on channel 1 and `SMZSTRIP2` on channel 2, both pointing at that PNG; `render.use_sequencer` on; format PNG; frame 7 rendered twice more, once with `metadata_input = 'STRIPS'` and once with `'SCENE'`. Both strips printed `frame_final_start 7 frame_final_duration 1`. The source PNG's bytes and size were the same at every check before and after both renders, so it still carried its own fields at the moment of the render.

The whole of what `exiftool -a -G1 -s` printed for the `STRIPS` file on machine one, from `[PNG]` to the end, uncut:

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

Not one Blender field, and no `Strip` line. The `SCENE` file from the same session carried the fourteen, including `Strip : SMZSTRIP2`, `RenderTime : 00:00.00`, `Memory : 0.39M`, `Hostname : Artists-Studio-MacBook.local`, and none of the four `cycles.` fields. The source PNG carried those fourteen plus the four Cycles fields — eighteen in all.

**Machine two agreed.** Its `STRIPS` file printed the same list, and its bytes hash to the same value as machine one's, because the file holds nothing that names either machine. Its `SCENE` file reads `Hostname : ubuntu` and `File : /root/ht2/smzfile.blend`.

Counting the Blender fields in each of the three files, on both machines: source 18, `STRIPS` 0, `SCENE` 14. **This run and the citation check's run came out the same; the first run came out differently.** All three stand as written and none settles the others. The full run record is in `Audits/Blender Research/Images/Hand tests — cross-sweep questions.md`.

### What a movie holds

An MPEG-4 with H.264 over frames 7 to 9, every switch on, carries **none of the fourteen under any name**. What it does carry: `ColorProfiles : nclx`, `ColorPrimaries : BT.709`, `TransferCharacteristics : BT.709`, `MatrixCoefficients : BT.709`, `VideoFullRangeFlag : Limited`, `ContentCreateDate : 2026/08/12 06:22:59`, `Encoder : Lavf61.7.100`. `Lavf` is the FFmpeg library, not Blender. So the "image/video metadata" wording is not borne out in an MPEG-4 written by 4.5.

### A field an engine adds for itself

`result.stamp_data_add_field("SMZFIELD", "SMZVALUE-98765")` on the render result succeeded, and neither string appears anywhere in the PNG that render wrote — while the scene's own fourteen, in the same file, do.

### Colour, pixel density and Cryptomatte

No PNG written by 4.5 carried an ICC profile or any colour tag at all, at the default colour management or with the view transform set to `Standard`; no OpenEXR written by 4.5 carried any colour-space attribute. With the density set to 300, PNG and BMP write the two directions separately and follow a non-square pixel aspect (150 and 300), TIFF writes both as 150, OpenEXR writes one number, **JPEG writes none** (`ResolutionUnit : None`), and WebP writes none. A Cryptomatte render writes four header attributes per layer — `cryptomatte/<hash>/conversion`, `/hash`, `/manifest` and `/name` — for example `[cryptomatte/542cafa/name] = ViewLayer.CryptoObject`.

---

## Where I looked (the record)

**Files searched in bulk.** All of `manual_4.5` (excluding the `_images` folder from text searches), all of `api_4.5`, all of `release_notes_4.5`, all of `cycles_4.5`.

**Terms searched, and what came back.**

- `metadata` — manual: 30 files, of which the substantive ones are `render/output/properties/metadata.txt`, `files/media/image_formats.txt`, `render/output/properties/output.txt`, `editors/image/sidebar.txt`, `editors/image/overlays.txt`, `editors/image/navigating.txt`, `editors/uv/overlays.txt`, `editors/video_sequencer/preview/sidebar.txt`. The rest are asset-library, USD, Cryptomatte, OSL and brush-management pages, ruled out as a different sense of the word. API: `RenderSettings.metadata_input`, `MovieClip.metadata()`, plus asset, theme, volume-grid and sequencer-overlay uses, ruled out likewise.
- `stamp` — manual: only `render/output/properties/metadata.txt`; everything else is "timestamp" (Timeline, Graph Editor, proxies, command line, FBX) or brush "stamping", ruled out. API: `RenderSettings` (16 entries), `RenderResult.stamp_data_add_field`, `Scene.use_stamp_note`, plus Freestyle and proxy timecode uses, ruled out.
- `exif` — manual: three hits, all ruled out (the exiftool tip, and camera focal length from EXIF in motion tracking). API: none. Release notes: none.
- `xmp` — one hit, asset-library sidecar files, ruled out. `iptc` — nothing anywhere.
- `chromaticit`, `attribute`, `header field`, `image header`, `EXR header`, `chunk`, `tEXt`, `textual`, `info text`, `render info`, `sidecar`, `provenance`, `watermark`, `slate`, `burn-in`, `burnin`, `generated by`, `rendered with`, `version number`, `app version`, `creator`, `Software`, `ppm` — searched across the manual, or across `render/`, `files/media/`, `editors/image/` and `compositing/` where a whole-manual search was too noisy. Nothing bearing on what a rendered file records. "Watermark" is a Mix-node tutorial about hiding your own mark in the pixels, not anything Blender writes by itself, so I ruled it out.
- `hostname`, `peak memory`, `Render Time` — confirmed the Metadata page is the manual's only page on the subject; every other "render time" hit is about how long a render takes.
- `DPI` — led to the Pixel Density section quoted above and to the format table's DPI column.
- `media_type` — searched the whole API and found nothing, although a 4.5 release note mentions a new Media Type enum. Not pursued further; it is not this question.

**Pages opened in full.** `render/output/properties/metadata.txt`, `render/output/properties/output.txt`, `render/output/properties/format.txt`, `render/output/properties/index.txt`, `files/media/image_formats.txt`, `compositing/types/output/file_output.txt` (checked for metadata; it says nothing about it), `api_4.5/bpy.types.RenderSettings.txt`, `api_4.5/bpy.types.RenderResult.txt`, `api_4.5/bpy.types.ImageFormatSettings.txt`, and the relevant sections of `editors/video_sequencer/preview/sidebar.txt`, `editors/image/overlays.txt`, `editors/image/sidebar.txt`, `editors/image/navigating.txt`, `editors/uv/overlays.txt`, `api_4.5/bpy.types.Scene.txt`, `api_4.5/bpy.types.Image.txt`, `api_4.5/bpy.types.MovieClip.txt`.

**Picture opened.** `manual_4.5/_images/render_output_properties_metadata_panel.png`, from the Metadata page. I enlarged the copy with the `sips` tool on macOS (Darwin 25.5.0) to read the labels and tick states; the enlargement changes nothing but the size.

**Release notes.** Searched all of `release_notes_4.5` for `metadata` and `stamp`: three hits, of which one is relevant (the OpenEXR ACES2065-1 line, quoted above), one is a Grease Pencil brush bug fix and one is asset-browser interface work, both ruled out.

**Cycles files.** Searched all 399 files for `stamp`, `stamp_data`, `metadata`, `use_stamp`, `metadata_input`. Result as reported in section B.

**Nothing outside `Blender Sources/`.** I did not need the web and did not use it.

---

## Things a person with Blender open could settle in a few minutes

The eleven items that stood here were all run on 2026-08-11, in 4.5 on both of this project's machines, and their answers are in "What the hand tests showed" above. What is left open needs a machine this project does not have.

1. **Whether any of this differs on Windows.** Item 8 above asked for the Hostname value to be compared across Windows, macOS and Linux. The macOS and Linux halves were run and agreed. **The Windows half was not run: no Windows machine has run it.** Until it has one, nothing above speaks for Windows.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Images, Blender 4.5

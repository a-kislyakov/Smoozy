# Images — cross-sweep questions, second pass (Blender 5.2)

**Sources consulted, and when.** Blender 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` pictures), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files (`Blender Sources/cycles_5.2`). All four were taken on **2026-07-30**, and that is the date read for everything quoted below. I did the searching and reading on 2026-08-11.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

I worked alone — no helper agents were started — so the declaration above covers the whole of this work.

Question answered: **10 only.**

---

## 10. What does a finished picture hold, in every part of the file, about what produced it?

### The question was ambiguous, and I say so plainly

"In every part of the file" has two readings.

- **The reading I answered:** *everywhere in the file* — anywhere information about the render could sit, not just one header block. So: the metadata block, the pixels themselves, and the colour information.
- **The other reading:** OpenEXR's literal "parts". Blender 5.2 writes multi-part OpenEXR files by default, and a multi-part file has several parts each with its own header. Read that way, the question asks whether every part of such a file carries the render information. **The sources do not answer that reading at all** — see "What I could not find" below. I have not silently chosen; both are set out.

### LABEL

**Mixed.** The list of things Blender can put in the file is **documented**. Panel layout and default ticks are **documented** (text plus one picture). What those things are actually called *inside* the file, and whether Blender writes anything of its own accord about itself, is **not in the sources**.

### The short answer

A finished picture holds whatever of a fixed list of scene facts you asked Blender to include, written into the file's metadata — but only if the file format is one of the four that support metadata. The same facts can additionally be painted into the picture's own pixels. Separately, and not controlled by any of that, Blender records the colour space the picture is in. Nothing in the sources says Blender writes its own name, its version, the render engine, or the render settings.

### What can be included — the full list, quoted

The Metadata panel is the one place this is set. Its whole list, quoted from the manual:

> "The Metadata panel includes options for writing metadata into render output."

> "Date — Includes the current date and time.
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

(The manual sets these out as a definition list, one term per entry; I have run them together with dashes and kept every word. Each term and its sentence are exactly as printed.)

Source: manual, *Metadata* — https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html — read 2026-07-30.

That is fourteen items. Twelve are facts about the render; two are the frame range and the note. There is no fifteenth.

### Where the metadata is taken from

> "Metadata Input
>
> Where to grab metadata from.
>
> Scene: Use metadata from the current scene.
>
> Sequencer Strips: Use metadata from the strips in the Sequencer."

Same page, same date. The Python side agrees and gives the default:

> "metadata_input — Where to take the metadata from (default 'SCENE')"

Source: API, `bpy.types.RenderSettings` — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30.

### Which of these are on by default

The API documentation states a default for each switch. Quoted exactly, from `bpy.types.RenderSettings` (same address and date):

> "use_stamp_camera — Include the name of the active camera in image metadata (default True)"
> "use_stamp_date — Include the current date in image/video metadata (default True)"
> "use_stamp_filename — Include the .blend filename in image/video metadata (default True)"
> "use_stamp_frame — Include the frame number in image metadata (default True)"
> "use_stamp_frame_range — Include the rendered frame range in image/video metadata (default False)"
> "use_stamp_hostname — Include the hostname of the machine that rendered the frame (default False)"
> "use_stamp_lens — Include the active camera's lens in image metadata (default False)"
> "use_stamp_marker — Include the name of the last marker in image metadata (default False)"
> "use_stamp_memory — Include the peak memory usage in image metadata (default True)"
> "use_stamp_note — Include a custom note in image/video metadata (default False)"
> "use_stamp_render_time — Include the render time in image metadata (default True)"
> "use_stamp_scene — Include the name of the active scene in image/video metadata (default True)"
> "use_stamp_sequencer_strip — Include the name of the foreground sequence strip in image metadata (default False)"
> "use_stamp_time — Include the rendered frame timecode as HH:MM:SS.FF in image metadata (default True)"

So, by that list, an untouched 5.2 scene writing to a format that supports metadata would carry: date, time, render time, frame, memory, camera, scene, and the blend-file's name. Eight of the fourteen.

Two small things worth flagging, because they are the sort of thing that trips people:

- **The picture disagrees with the API on one item.** The manual's screenshot of the panel is `[image: _images/render_output_properties_metadata_panel.png]` on the Metadata page. I opened it. In the picture, ticked: Date, Time, Render Time, Frame, Camera, Scene, Filename. Unticked: Frame Range, Memory, Hostname, Lens, Marker, Strip Name, and the strip-metadata checkbox (its label is cut off in the picture as "Use Strip Me.."). **Memory is unticked in the picture but "default True" in the API documentation.** I cannot reconcile these from the sources; a screenshot need not have been taken on a default scene. I report both and take neither as settling the other. Label for the picture reading: **documented** (it is part of the manual); label for "therefore Memory is off by default": I make no such claim.
- Two things named in the manual as switches are, in the API, one switch and one text box. "Note" is the switch `use_stamp_note`, and the text itself is `stamp_note_text` — "Custom text to appear in the stamp note (default "", never None)" (`bpy.types.RenderSettings`). There is also `use_stamp_note` on `bpy.types.Scene`, which is a *string*, not a switch: "User defined note for the render stamping (default "", never None)" — API, `bpy.types.Scene`, https://docs.blender.org/api/5.2/bpy.types.Scene.html, read 2026-07-30.

### The second place it can sit: the pixels

The same facts can be painted into the image itself, which is a different part of the file entirely — it survives any format, because it is just picture.

> "Burn into Image
>
> Add metadata as text to the render.
>
> Font Size — Set the size of the text.
>
> Text Color — Set the color and alpha of the stamp text.
>
> Background — Set the color and alpha of the color behind the text.
>
> Include Labels — Displays the labels before the metadata text. For example, "Camera" in front of the camera name, etc."

Source: manual, *Metadata*, address as above, read 2026-07-30.

The API calls this `use_stamp`: "Render the stamp info text in the rendered image (default False)", with `stamp_font_size` "(in [8, 64], default 12)", `stamp_foreground` "(array of 4 items, in [0, 1], default (0.8, 0.8, 0.8, 1.0))", `stamp_background` "(array of 4 items, in [0, 1], default (0.0, 0.0, 0.0, 0.25))", and `use_stamp_labels` "Display stamp labels ("Camera" in front of camera name, etc.) (default True)". Same API page and date.

So burn-in is **off by default**, and when it is on the information is in the picture rather than in a header.

### It only reaches the file at all if the format supports it

> "Note
>
> Only some image formats support metadata:
> See image formats."

(manual, *Metadata*, as above.)

The formats table gives the answer. Of the fourteen image formats listed, a Metadata column is ticked for exactly four: **JPEG, OpenEXR, PNG, WebP**. It is crossed for AVIF, BMP, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw and TIFF.

And, quoted:

> "Metadata is additional information stored inside an image file that describes how the image was created or how it should be interpreted."

> "Blender can write metadata into supported image formats when enabled in the Output Properties."

> "Limitations
>
> Not all formats support metadata, and some may only support a limited subset of fields.
>
> When using formats that do not support metadata, render information is not preserved in the saved file."

Source: manual, *Supported Image Formats* — https://docs.blender.org/manual/en/5.2/files/media/image_formats.html — read 2026-07-30.

That "some may only support a limited subset of fields" is important and is as far as the manual goes: it does not say which formats drop which fields.

### The third place: colour information

Separate from all of the above, and not governed by the Metadata panel, Blender records what colour the numbers in the file mean.

> "By default, Blender integrates with automatic color management in operating systems and saves images and videos with color space metadata."

Source: manual, *System Configuration* — https://docs.blender.org/manual/en/5.2/render/color_management/system_configuration.html — read 2026-07-30.

> "The icc_profile_name interchange attribute is supported, to embed ICC profiles when saving images."

> "Saving colorspace metadata for images and video also depends on this." (said of using OpenColorIO v2 display colour spaces with a `cie_xyz_d65_interchange` intermediate space)

> "Include the interop ID from the Color Interop Forum for every color space and display color space that you can. This helps save image and video with correct colorspace information."

Source: manual, *OpenColorIO Configuration* — https://docs.blender.org/manual/en/5.2/render/color_management/opencolorio.html — read 2026-07-30.

This is information about *how to read* the picture rather than about *what produced it*, so it sits at the edge of the question. I include it because it is a real, separately-governed thing the file carries, and because it depends on the OpenColorIO configuration in use rather than on any render setting — a picture can therefore carry colour information and no render metadata, or the reverse.

### A render engine or add-on can add its own fields

> "stamp_data_add_field(field, value)
>
> Add engine-specific stamp data to the result
>
> field (str) – Field, Name of the stamp field to add (never None)
>
> value (str) – Value, Value of the stamp data (never None)"

Source: API, `bpy.types.RenderResult` — https://docs.blender.org/api/5.2/bpy.types.RenderResult.html — read 2026-07-30.

So the fourteen are not a hard ceiling: whatever is rendering can attach further named fields to the result. The documentation says nothing about whether those extra fields then reach the written file, nor what happens if a name collides with a built-in one.

The manual points the same way for hand-written values, though its link is lost in the plain-text copy I have:

> "It can be useful to use the Note field if you are setting up a render farm. Since you can script any information you like into it, such as an identifier for the render node or the job number. For details on stamping arbitrary values, see: this page."

(manual, *Metadata*, as above.) The words "this page" are a link in the real manual; the plain-text copy does not preserve where it goes, so I cannot follow it and do not guess at it.

### What Blender can read back out

> "Lists information that has been encoded in the currently visible movie or image file (not the file referenced by the selected strip). This can include the filename, the creation date, the camera model etc. This also works for images produced by Blender; see Render Output for the metadata that can be included in this case."

> "Other graphics programs may also store metadata, but only the text in the header field "Comments" can be read."

> "The metadata can't be edited from Blender. Instead, you can use an external program such as exiftool. For example, the command to change the "Comments" field is: exiftool --comments="My new comment" name-of-file.png"

Source: manual, *Sidebar* (Video Sequencer preview) — https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/sidebar.html — read 2026-07-30.

This is the closest the sources come to saying the metadata lives in named header fields, and it is stated only about *other* programs' files. It also settles that Blender will not edit metadata in an existing file.

The Image Editor and UV Editor can display the same thing for a render:

> "Show Metadata — Displays metadata about the selected Render Result. See the Output tab's Metadata panel to change what metadata to include."

Source: manual, *Overlays* (Image Editor) — https://docs.blender.org/manual/en/5.2/editors/image/overlays.html — read 2026-07-30. The same sentence appears on https://docs.blender.org/manual/en/5.2/editors/image/navigating.html and, worded as "Display metadata about the selected Render Result", on https://docs.blender.org/manual/en/5.2/editors/uv/overlays.html. All read 2026-07-30.

### What changed in 5.2 (release notes, quoted)

Only two metadata entries appear anywhere in the 5.2 release notes, both under Movies rather than Images:

> "Stereoscopic metadata is exported. No need to post-process the video for correct playback in video players."

> "Panoramic camera metadata is exported. Only full equirectangle cameras are supported."

The Images section of the same page has one entry, and it is not about metadata:

> "OpenEXR: Faster writing of half-precision (FP16) EXRs."

Source: release notes, *Pipeline & I/O* — https://developer.blender.org/docs/release_notes/5.2/pipeline_io/ — read 2026-07-30.

### What I could not find — and the searches that make that a real answer

Every one of these is **not in the sources**.

1. **The names the fields are given inside the file.** Nothing anywhere states the actual tag or attribute names written into a PNG, JPEG, WebP or EXR. The only header field named in the whole manual is "Comments", and that is about reading other programs' files.
2. **Whether Blender records itself.** No source says the file carries Blender's name, its version, the build, the render engine (Cycles or EEVEE), sample counts, resolution settings, or any render setting other than the fourteen listed.
3. **Whether every part of a multi-part OpenEXR carries it.** The multi-part format is real and is the default in 5.2 — `use_exr_interleave`: "Use legacy interleaved storage of views, layers and passes for compatibility with applications that do not support more efficient multi-part OpenEXR files. (default False)" (API, `bpy.types.ImageFormatSettings`, https://docs.blender.org/api/5.2/bpy.types.ImageFormatSettings.html, read 2026-07-30). Nothing says whether metadata is written once or once per part.
4. **Whether the Compositor's File Output node writes metadata.** Its manual page never mentions metadata or stamping.
5. **Whether saving by hand from the Image Editor writes metadata.** Not stated. The Metadata panel is described only as applying to "render output".
6. **What "a limited subset of fields" means per format.** Named as a limitation, never spelled out.
7. **Whether Cycles adds anything.** It does not, as far as its own files show.

**Where I looked, and with what words.**

- Whole-manual searches (`manual_5.2`, all files) for: `metadata`, `stamp`, `EXIF`, `XMP`, `ICC profile`, `IPTC`, `chromaticit`, `sidecar`, `provenance`, `Comments`, `header`, `creation date`, `created by`, `generator`, `producer`, `software`, `watermark`, `burn in`, `burn-in`, `Burn Into`, `arbitrary`, `render farm`, `embed`, `interleave`, `multi-part`, `multipart`, `image metadata`, `video metadata`, `color space metadata`, `colorspace metadata`, `mastering display`.
- `metadata` across the whole manual returned 24 files. I opened or read the relevant part of every one that could bear on a written picture: `render/output/properties/metadata.txt`, `render/output/properties/index.txt`, `render/output/index.txt`, `render/output/properties/output.txt`, `render/output/properties/post_processing.txt`, `files/media/image_formats.txt`, `files/media/video_formats.txt`, `editors/image/sidebar.txt`, `editors/image/overlays.txt`, `editors/image/navigating.txt`, `editors/uv/overlays.txt`, `editors/video_sequencer/preview/sidebar.txt`, `editors/video_sequencer/preview/display/overlays.txt`, `compositing/types/mask/cryptomatte.txt`, `compositing/types/input/string_to_image.txt`, `render/color_management/opencolorio.txt`, `render/color_management/system_configuration.txt`, `glossary/index.txt`. **Ruled out and why:** `editors/asset_browser.txt`, `files/asset_libraries/introduction.txt`, `glossary/index.txt` and `sculpt_paint/brush/brush_management.txt` are about *asset* metadata inside blend-files, not pictures; `editors/clip/introduction.txt` and `movie_clip/…` are about footage a clip was loaded from; `modeling/geometry_nodes/volume/read/grid_info.txt` is volume grids; `render/cycles/osl/*` is shader-script metadata; `advanced/scripting/addon_tutorial.txt` is add-on registration; `files/import_export/usd.txt` is USD export; `video_editing/setup/directory_structure.txt` is filing advice. `compositing/types/input/string_to_image.txt` mentions metadata only as an example of text you might draw ("This can be used to display dynamic text in the compositor, such as frame numbers, metadata, labels, or values generated by other nodes") — it is a text-drawing node, not a metadata writer.
- Whole-API searches (`api_5.2`) for `stamp` and `metadata`, plus the index files `genindex-all.txt`. `use_stamp` in the index returns exactly sixteen entries — the fourteen switches, `use_stamp` itself, `use_stamp_labels` — plus the `bpy.types.Scene` duplicate of `use_stamp_note`. There is no seventeenth. I read `bpy.types.RenderSettings`, `bpy.types.RenderResult`, `bpy.types.RenderLayer`, `bpy.types.Scene`, `bpy.types.Image`, `bpy.types.ImageFormatSettings`, `bpy.ops.render`, `bpy.ops.image`. **`bpy.types.ImageFormatSettings` has no metadata property of any kind** — I listed its properties one by one to be sure. Ruled out: the remaining `metadata` hits are asset metadata, movie-clip metadata (`MovieClip.metadata()` reads a movie file's metadata), theme colours for the metadata overlay, FBX export's `use_metadata`, and USD hooks.
- Whole release-notes search (`release_notes_5.2`, every subject file) for `metadata` and `stamp`: three hits total, all quoted above. Nothing in `rendering.md`, `core.md`, `cycles.md`, `eevee.md` or `compositor.md`.
- Whole Cycles search (`cycles_5.2`) for `metadata` and `stamp`: every hit is in `osl.py` and is about OSL shader-parameter metadata (widget, label, units) used to build the interface — for example `cycles_5.2/osl.py` line 58: `    metadata = {meta.name: meta.value for meta in param.metadata}`, and line 98's comment `    # Get metadata for the parameter to control UI display`. **Nothing in Cycles' files concerns image-file metadata.** I make no claim about what Cycles writes.
- Command-line documentation: searched `manual_5.2/advanced/` (which holds `command_line/arguments.txt`, `command_line/render.txt`, `command_line/index.txt`, `command_line/extension_arguments.txt`, `command_line/launch/`) for `stamp` and `metadata`. **One hit in the whole of `advanced/`, and it is the add-on tutorial.** The command-line pages say nothing about metadata.
- Pictures opened: `manual_5.2/_images/render_output_properties_metadata_panel.png` (the Metadata panel, from the Metadata page). Read as described above.

---

## What the hand tests showed — question 10

**LABEL for everything in this section: observed.** Added 2026-08-11 by the hand-test step, which ran a list of nineteen tests in Blender 5.2.0 on both of this project's machines. The full list and the full run record are in `Audits/Blender Research/Images/Hand tests — cross-sweep questions.md`.

**The two machines.** *Machine one is the Creator's Mac* — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card; `hostname` prints `Artists-Studio-MacBook.local`; exiftool 13.50. *Machine two is a rented Linux machine* — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory; `hostname` prints `ubuntu`; exiftool 12.40. Every Blender was started with `--factory-startup` and with its own settings folder.

**The scene used.** Blender's factory startup scene, with the scene renamed `SMZSCENE`, the camera renamed `SMZCAM` and its lens set to `37.5`, a marker named `SMZMARK`, frames 7 to 9 with frame 7 current, 64 × 64 pixels, Cycles at one sample, every `use_stamp_*` switch on, the note text `SMZNOTE-12345`, Burn Into Image off, saved as `smzfile.blend`.

**Where the two machines agreed and where they did not.** Every result below came out the same on both, except that the `File` field holds that machine's own path and `Hostname` holds that machine's own name.

### Gap 1 answered: the tag names inside each format

Written to PNG on machine one:

```
[PNG]           File                            : …/smzfile.blend
[PNG]           Note                            : SMZNOTE-12345
[PNG]           Date                            : 2026/08/12 06:23:49
[PNG]           Marker                          : SMZMARK
[PNG]           Time                            : 00:00:00:07
[PNG]           Frame                           : 7
[PNG]           FrameRange                      : 7:9
[PNG]           Camera                          : SMZCAM
[PNG]           Lens                            : 37.50
[PNG]           Scene                           : SMZSCENE
[PNG]           Strip                           : <none>
[PNG]           RenderTime                      : 00:00.04
[PNG]           Memory                          : 1M
[PNG]           Hostname                        : Artists-Studio-MacBook.local
[PNG]           CyclesViewLayersamples          : 1
[PNG]           CyclesViewLayertotal_time       : 00:00.04
[PNG]           CyclesViewLayerrender_time      : 00:00.04
[PNG]           CyclesViewLayersynchronization_time: 00:00.00
```

On machine two the same, with `[PNG] File : /root/ht/smzfile.blend` and `[PNG] Hostname : ubuntu`. In OpenEXR the same fourteen names are separate header attributes. In JPEG they are separate comment markers, each reading `Blender:<name>:<value>`. Render Time is written as minutes and seconds; Memory as a whole number with `M` after it.

### Gap 6 answered, and the formats table checked

The version offers `['AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']`, plus a setting this answer does not mention: `MEDIA-TYPE: IMAGE ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`. Multi-layer OpenEXR can only be asked for once the media type is set to `MULTI_LAYER_IMAGE`; asking by name before that fails with `enum "OPEN_EXR_MULTILAYER" not found in (…)`.

- **Kept the fields: PNG, JPEG, OpenEXR (single-layer and multi-layer)** — and **all fourteen** in each, so no format "quietly drops" any.
- **Kept nothing: AVIF, BMP, Iris, JPEG 2000, Targa, Targa Raw, Cineon, DPX, Radiance HDR, TIFF — and WebP.**

**WebP kept nothing**, on both machines, although the table ticks it. Nothing appeared in Blender's console about a format that could not carry them.

### Gap 2 answered: Blender does name itself, in OpenEXR

Every OpenEXR written by 5.2.0 carries, **with every `use_stamp*` switch set to `False`**:

```
  [Software] (string, 17 bytes) = Blender 5.2.0 LTS
  [colorInteropID] (string, 16 bytes) = lin_rec709_scene
```

A **multi-layer** OpenEXR always additionally carries `[BlenderMultiChannel] = Blender V2.55.1 and newer`. A PNG, a JPEG, a WebP and an AVIF carry no such tag. Nothing anywhere named the render engine, the samples, the resolution or the view transform.

### Gap 7 answered: Cycles does add fields

Every file a Cycles render writes carries `cycles.ViewLayer.samples`, `cycles.ViewLayer.total_time`, `cycles.ViewLayer.render_time` and `cycles.ViewLayer.synchronization_time`. They survive with every stamp switch off, and they are absent from an otherwise identical EEVEE render. The `cycles_5.2` folder says nothing about them because the part of Cycles that writes them is not in it — the caution in the search record was the right one.

### Gap 3 answered: the record sits in part 0 only

A multi-layer OpenEXR of two view layers with a Mist pass each, at the factory setting of `use_exr_interleave` (off), is `multiPart=True` with `TOTAL PARTS: 6`, and **every one of the fourteen fields sits in part 0 and nowhere else**:

```
--- PART 0 HEADER ---
  [BlenderMultiChannel] (string, 25 bytes) = Blender V2.55.1 and newer
  [Camera] (string, 6 bytes) = SMZCAM
  …the whole of the fourteen…
  [Software] (string, 17 bytes) = Blender 5.2.0 LTS
  [colorInteropID] (string, 16 bytes) = lin_rec709_scene
  [name] (string, 17 bytes) = AAALAYER.Combined
  (37 attributes in part 0)
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

Parts 2 to 5 are the same thirteen with their own `name` and `channels`. With Interleave turned on, the same render writes `TOTAL PARTS: 1`.

### Gaps 4 and 5 answered

`Image ▸ Save As` from an Image Editor wrote a file whose tag list is identical, name for name, to the file the render wrote itself.

The compositor's File Output node also writes the fourteen names, from the same render, but with `Camera : <none>`, `Lens : <none>`, `RenderTime : 00:00.00`, and none of the four Cycles fields. Building that node setup in 5.2 needs `scene.compositing_node_group`; `scene.node_tree` no longer exists and `Scene.use_nodes` prints `DeprecationWarning: 'Scene.use_nodes' is expected to be removed in Blender 6.0`. The node's own settings are `directory` and `file_name`, and its format starts at `MULTI_LAYER_IMAGE`.

> **The File Output node half could not be repeated at all. Added by the citation check, 2026-08-11.** Every API fact here holds. But following this description on both machines, **the node wrote no file at all**: with an item added to `file_output_items`, its format set to PNG after the media type was set to `IMAGE`, its input linked from the group input and a group output linked so the ordinary render still wrote, the node's folder was empty after a still render and after an animation render alike. What the original run did to make it write is not in these words. The hand-saved half held, but by `Image.save_render`, because the operator refused with `RuntimeError: Operator bpy.ops.image.save_as.poll() failed, context is incorrect`. Back on the hand-test list as test 22.

**Run again, 2026-08-12, with every step written down — the node still wrote nothing.** Test 22 on the hand-test list, run in Blender 5.2.0 LTS on **machine one, the Creator's Mac** (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card) and on **machine two, a rented Linux machine** (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory). This is the recipe the test was asked to produce, in the order the steps were taken, with what the machine printed at each one:

```
STEP 1: ordinary output — media type then format
  ims.media_type = IMAGE
  ims.file_format = PNG
  render.filepath = …/out/22/5.2/panel/panel
STEP 2: the node group
  scene.node_tree present: False
  bpy.data.node_groups.new('SMZComp', 'CompositorNodeTree') -> SMZComp CompositorNodeTree
  scene.compositing_node_group = <bpy_struct, CompositorNodeTree("SMZComp") at 0xa29325e08>
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
STEP 6: directory that does not exist: …/out/22/5.2/node_missing | file_name: smznode
LIST[before still render, node folder] …/out/22/5.2/node_missing -> NO SUCH FOLDER (FileNotFoundError)
STEP 6a: bpy.ops.render.render(write_still=True)
LIST[after still render, node folder] …/out/22/5.2/node_missing -> NO SUCH FOLDER (FileNotFoundError)
LIST[after still render, panel folder] …/out/22/5.2/panel -> ['panel.png']
STEP 7: directory made first: …/out/22/5.2/node_made
STEP 7a: bpy.ops.render.render(write_still=True)
LIST[after still render, node folder] …/out/22/5.2/node_made -> []
LIST[after still render, panel folder] …/out/22/5.2/panel2 -> ['panel.png']
STEP 8: bpy.ops.render.render(animation=True) over frames 7 to 7
LIST[after animation render, node folder] …/out/22/5.2/node_anim -> []
LIST[after animation render, panel folder] …/out/22/5.2/panel3 -> ['panel0007.png']
DONE-22
```

So: the interface sockets were added **before** the Group Input and Group Output nodes. The media type must be set before the file format, and asking for PNG first fails with the error above. The item is made with `file_output_items.new('RGBA', 'SMZNODE')`; the choices the machine printed for that first argument are `['FLOAT', 'INT', 'BOOLEAN', 'VECTOR', 'RGBA', 'ROTATION', 'MATRIX', 'STRING', 'MENU', 'SHADER', 'OBJECT', 'IMAGE', 'GEOMETRY', 'COLLECTION', 'TEXTURE', 'MATERIAL', 'BUNDLE', 'CLOSURE']`. The node then reported `fo.inputs[0].name: SMZNODE is_linked: True`, `fo.mute: False`, `save_as_render: True`, `active_item_index: 0`.

**The node's folder stayed empty every time**: not created when it did not exist, and left empty when it was made first, after a still render and after an animation render alike. The Output panel's own file was written every time from the same render. **The console said nothing about it** — no warning, no error, no line naming the node. **The same on machine two, and the same in 5.0 and 5.1.**

**Four steps were added beyond the entry's own procedure, and they are named here.** *One*, six variations on the folder and file name were tried — a plain folder, a folder with a trailing separator, a file name ending `.png`, a file name ending `####`, `save_as_render` off, and a folder written as `//c6` — still and animation each. Every one left the folder empty, and a search of the whole working folder afterwards found nothing called `smznode`. *Two*, the same scene was rendered three times, with no group, with a group whose Invert node sits between the group input and the group output, and with that group's links removed; the three pictures came out different from one another, so the group is executed during the render and the render result does reach its input, while the File Output node inside it still wrote nothing. *Three*, the whole build was run again on both machines from a Blender with a real window open rather than with `-b`; the node's folder was empty there too, after a still render and after an animation render. *Four*, in a windowed Blender `bpy.ops.node.new_compositing_node_group()` returned `{'FINISHED'}` and left `scene.compositing_node_group` as `None`.

**This run and the citation check's run came out the same; the first run came out differently.** All three stand as written and none settles the others.

### Burning in

Rendered at 640 × 360 with the burn-in on, to PNG and to Targa. Both pictures carry the same text: a block at the top left reading `File …`, `Date 2026/08/12 06:23:52`, `RenderTime 00:00.35`, `Peak Memory 17M`, `Hostname Artists-Studio-MacBook.local`, then `SMZNOTE-12345` on its own line with no label; a strip along the bottom reading `Marker SMZMARK Timecode 00:00:00:07 Frame 7 Frame Range 7:9 Camera SMZCAM Lens 37.50 Scene SMZSCENE`; and a `Strip <none>` item at the top right which at this width overlaps the end of the `File` line. Burning in **adds to** rather than replaces: the PNG from that run still carried the whole block of named fields. The burnt-in labels are not the file's tag names — `Memory` is drawn as `Peak Memory`, `Time` as `Timecode`.

### Engine-added fields do not reach the file

A render engine registered for the test called `result.stamp_data_add_field("SMZFIELD", "SMZVALUE-98765")` and the call succeeded. Neither string appears anywhere in the PNG that render wrote, while the scene's own fourteen, in the same file, do.

### The colour information

**No PNG written by 5.2 carried an ICC profile**, at the default colour management or with the view transform set to `Standard`. A 5.2 PNG does carry the format's own colour chunks — `SRGBRendering : Absolute Colorimetric`, `Gamma : 2.2`, `WhitePointX : 0.3127`, `WhitePointY : 0.329`, `RedX : 0.64`, `RedY : 0.33`, `GreenX : 0.3`, `GreenY : 0.6`, `BlueX : 0.15`, `BlueY : 0.06` — which a 5.0 PNG does not. In OpenEXR the colour marking is the `colorInteropID` attribute above. Identical on both machines.

### The two Movies release-note entries, checked

A side-by-side Stereo 3D MPEG-4 written by 5.2 is named `stereo_0007-0009.mp4` — 4.5 and 5.0 name the same render `stereo_0007-0009_L.mp4` — and is 128 × 64. A separate render with an equirectangular Cycles panoramic camera wrote `pano_0007-0009.mp4`. **Neither file carries any stereo or projection marking that could be found**: searching each for `st3d`, `sv3d`, `svhd`, `prji`, `proj`, `equi` and `ytmp` found none of them, and exiftool reported no such tag. The same on both machines, and the same in 4.5, 5.0 and 5.1.

An ordinary MPEG-4 carries none of the fourteen under any name — only `ColorProfiles : nclx`, `ColorPrimaries : BT.709`, `TransferCharacteristics : BT.709`, `MatrixCoefficients : BT.709`, `VideoFullRangeFlag : Limited`, `ContentCreateDate : 2026/08/12 06:24:02` and `Encoder : Lavf62.12.100`.

### The Memory disagreement, the note properties and the truncated label

On a factory-fresh 5.2 scene, on both machines, `use_stamp_memory` reads `False`, while the property's own stated default is `True`. `RenderSettings.use_stamp_note` is a boolean, `RenderSettings.stamp_note_text` is the text, and `Scene.use_stamp_note` is a *string* described as "User defined note for the render stamping" — so the two API pages are not in conflict.

**No property in 5.2 has an interface name beginning "Use Strip Me".** The panel's own drawing code, read out of the running Blender, offers only `metadata_input` and `use_stamp_sequencer_strip` (drawn as "Strip Name"), both only when the Video Sequencer is in use.

### Pixel density, Cryptomatte and the Sequencer's strips

With the density set to 300: PNG carries `PixelsPerUnitX : 11811` and `PixelsPerUnitY : 11811`; BMP carries `PixelsPerMeterX : 11811`, `PixelsPerMeterY : 11811`; TIFF carries `XResolution : 300`, `YResolution : 300`; OpenEXR carries one number, `xDensity : 300`; **JPEG carries none**; WebP carries none. With a 2:1 pixel aspect, PNG and BMP write the two directions separately, **TIFF writes both as the X value**, and OpenEXR still writes one number.

A Cryptomatte render writes four header attributes per layer, for example `[cryptomatte/542cafa/name] = ViewLayer.CryptoObject` and `[cryptomatte/542cafa/manifest] = {"Cube":"a8fce865","Light":"c77d43e5","object":"055ec73d"}`.

With `metadata_input` set to `STRIPS` and two image strips in the Video Sequencer, the written file's block is **replaced by the block read out of the foreground strip's own image file** — its date, its render time, its memory figure and its Cycles fields — and `Strip` reads `<none>`; with `SCENE` the date is the render's own and `Strip` reads `SMZSTRIP2`.

> **The `STRIPS` half did not reproduce. Added by the citation check, 2026-08-11.** Run again in 5.2 on both machines, with two image strips (`SMZSTRIP1` on channel 1, `SMZSTRIP2` on channel 2) pointing at a PNG written by the same session, the file written with `metadata_input` set to `STRIPS` carried **no Blender metadata at all** — only the picture's own structure. The `SCENE` half reproduced exactly. Both runs stand as written; neither settles the other. Back on the hand-test list as test 20.

**Run a third time, 2026-08-12 — the `STRIPS` file carried no Blender field at all.** Test 20 on the hand-test list, run in Blender 5.2.0 LTS on **machine one, the Creator's Mac** (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card) and on **machine two, a rented Linux machine** (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory). What was done: the standard scene; frame 7 rendered to a PNG first, in the same Blender session, carrying its own full block; two image strips, `SMZSTRIP1` on channel 1 and `SMZSTRIP2` on channel 2, both pointing at that PNG; `render.use_sequencer` on; format PNG; frame 7 rendered twice more, once with `metadata_input = 'STRIPS'` and once with `'SCENE'`. Both strips printed `frame_final_start 7 frame_final_duration 1`. The source PNG's bytes and size were the same at every check before and after both renders, so it still carried its own fields at the moment of the render.

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

Not one Blender field, and no `Strip` line. The nine colour lines are there because 5.2 writes them into every PNG; they are not part of the metadata block. The `SCENE` file from the same session carried the fourteen, including `Strip : SMZSTRIP2`, `RenderTime : 00:00.00`, `Memory : 1M`, `Hostname : Artists-Studio-MacBook.local`, and none of the four `cycles.` fields. The source PNG carried those fourteen plus the four Cycles fields — eighteen in all.

**Machine two agreed.** Its `STRIPS` file printed the same list, and its bytes hash to the same value as machine one's, because the file holds nothing that names either machine. Its `SCENE` file reads `Hostname : ubuntu` and `File : /root/ht2/smzfile.blend`.

Counting the Blender fields in each of the three files, on both machines: source 18, `STRIPS` 0, `SCENE` 14. **This run and the citation check's run came out the same; the first run came out differently.** All three stand as written and none settles the others. The full run record is in `Audits/Blender Research/Images/Hand tests — cross-sweep questions.md`.

### Cycles' debug passes

On the processor a multi-layer OpenEXR carries channels named `ViewLayer.Debug Sample Count.X` and `ViewLayer.Render Time.X`; on the graphics card the Render Time channel is **absent** — Apple's own graphics through Metal on machine one, one NVIDIA RTX 4000 Ada through OptiX on machine two.

---

### Not covered

**Every item in this list was settled by hand test on 2026-08-11 — see "What the hand tests showed" above.** What follows is the list as the researcher left it.

This answer does not settle: what any field is literally called inside a PNG, JPEG, WebP or EXR file, so it does not tell you how to read one back with a tool other than Blender; whether metadata is written once or per part in a multi-part EXR; whether the frames written by a Compositor File Output node carry metadata at all; whether saving an image by hand carries it; which fields each of the four supporting formats quietly drops; whether engine-added fields reach the file; and the exact text formatting of each value as written (only Time is given a shape, "HH:MM:SS.FF"). It also does not settle whether the file records anything about Blender itself — it says only that no source I searched claims it does, which is weaker.

### Outside the interface

**Settled by hand test, 2026-08-11: it holds.** The same blend-file was rendered twice on each machine — once with `-b` and no window at all, once from a Blender with its window open — and the two files carry exactly the same tags, name for name, with only the date and the render time differing as they must. That was true in 5.2.0 on machine one (the Creator's Mac, macOS 26.5.2, Apple M1 Pro) and on machine two (the rented Linux machine, Ubuntu 22.04.5, two NVIDIA RTX 4000 Ada cards), and the two machines agreed. So the inference below — the one the answer rightly said should be checked by hand rather than trusted — turns out to be right. The nearby documented difference did **not** hold: the OpenEXR Preview option wrote a JPEG copy beside the EXR in *both* cases. What follows is what the sources said before the tests, kept for the trail.

**One place where it holds by being absent both ways, checked 2026-08-12.** The compositor's File Output node was built and rendered in 5.2.0 LTS on both machines, first with `-b` and no window and then from a Blender with its window open. Its folder was empty in every case, after a still render and after an animation render. So having a window makes no difference there either — the node wrote nothing with one and nothing without one.

Does this hold when Blender is driven from outside its own window?

**The sources do not say.** I searched the whole of `manual_5.2/advanced/`, which is where command-line operation is documented, for both `stamp` and `metadata`, and the only hit in the entire folder is an unrelated line in the add-on tutorial. No page ties metadata to background or command-line rendering, and none excludes it either.

What I can say, labelled **inferred**: every switch above is a property of `bpy.types.RenderSettings` and `bpy.types.Scene`, which are scene data saved in the blend-file and reachable from Python. The step a reader has to supply is that a scene property saved in the file governs a render started any way — the sources state that these are scene properties, not that the writing happens regardless of how the render was launched. So this is inference, not documentation, and it is exactly the kind of claim that should be checked by hand rather than trusted.

One nearby documented fact, which mentions the command line but is about a preview copy rather than metadata: OpenEXR's Preview option — "When rendering animations (or single frames via command line), save a JPEG copy of the image, for a quick preview." (manual, *Supported Image Formats*, address above, read 2026-07-30). I note it only because it is the single place in the image documentation that distinguishes command-line rendering at all.

There is one field whose *value* is plainly sensitive to how the render was launched rather than to Blender: Hostname, "Includes the rendering machine's hostname" — on a farm, that is the machine that did the work, which need not be the machine anybody was sitting at. The manual makes the same point itself in the render-farm hint quoted above. That the value differs per machine is documented; what it will be is not.

### Operating system

**Settled by hand test for macOS and Linux, 2026-08-11; Windows is still open.** Every one of the nineteen tests was run in 5.2.0 on both machines, and every result came out the same on macOS 26.5.2 with Apple's own graphics and on Ubuntu 22.04.5 with NVIDIA cards — including the colour chunks in a PNG and the `colorInteropID` in an EXR, so the display-side differences quoted below do not change what is written into a file. Two values differ, and only because they describe the machine: the `File` field holds that machine's own path, and `Hostname` holds that machine's own name — `Artists-Studio-MacBook.local` and `ubuntu`, each matching exactly what that machine's `hostname` command prints. The Linux machine's display server is Xorg, so the Wayland case is untried. **Windows was not tried: no Windows machine has run it.** What follows is what the sources said before the tests, kept for the trail. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Checked again on 2026-08-12 and still holding for macOS and Linux.** The second batch of hand tests ran three more tests in 5.2.0 LTS on both machines — the Sequencer-strips test, the part-by-part shape of a multi-part OpenEXR, and the compositor's File Output node — and the two machines agreed about every one, down to the thirteen attributes in each part of the OpenEXR and the empty node folder. **Windows is still untested for the same reason.**

**The sources do not say** that any of this differs by operating system. No page in the metadata documentation, the image-formats page, or the API entries mentions an operating system.

Two things nearby are OS-split, and both concern the colour information rather than the render metadata. The colour-management pages state per-OS requirements for wide gamut and HDR — "macOS: Available on any Apple Silicon device", "Linux: Use Wayland, and set the Vulkan backend in the Blender system preferences", and for Windows "For wide gamut, enable "Automatically manage color for apps" in the Windows display settings" (manual, *System Configuration*, address above, read 2026-07-30). The same page says "On Linux, Wayland color management supports displaying both SDR and HDR content similar to macOS. Because Wayland does not have a distinct HDR mode, Blender automatically switches between sRGB and gamma 2.2 image encoding depending on whether an SDR or HDR display was chosen."

Whether any of that changes the colour space metadata actually written into a saved picture is **not stated**, and I will not infer it — the quoted sentences are about display, and the one sentence about saving ("saves images and videos with color space metadata") carries no OS qualifier.

Hostname, again, is machine-specific by its nature, but the sources say nothing about how it is obtained or whether it differs by operating system.

---

## Things a person with Blender 5.2 open could settle in a few minutes

The nine items that stood here were all run on 2026-08-11, in Blender 5.2.0 on both of this project's machines, and their answers are in "What the hand tests showed" above. What is left open needs a machine this project does not have.

1. **Whether any of this differs on Windows.** Item 9 asked for the colour-profile check to be repeated on a second operating system; it was, on macOS 26.5.2 and on Ubuntu 22.04.5, and the two agreed — no ICC profile in either. **A third check on Windows was not run: no Windows machine has run it.** Until it has one, nothing above speaks for Windows.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Images, Blender 5.2

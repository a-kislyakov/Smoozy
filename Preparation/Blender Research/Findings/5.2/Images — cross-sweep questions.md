# Images — cross-sweep questions (Blender 5.2)

**Sources consulted, and on what date.** Blender 5.2's manual (`Blender Sources/manual_5.2`, including one picture from its `_images` folder), Blender 5.2's Python API documentation (`Blender Sources/api_5.2`), Blender 5.2's release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files (`Blender Sources/cycles_5.2`). All were read on 2026-08-11, from a snapshot taken on 2026-07-30, which is the date read given against every quotation below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 10. What does a finished picture hold, in every part of the file, about what produced it?

**A note on the reading.** "In every part of the file" is ambiguous. I read it as: everywhere inside the written file where something about the render is recorded — the header area where descriptive text is kept, the pixels themselves, and the file's internal structure. The other reading would be "in each of the file formats", and the format-by-format part of the answer below covers that too. I have not silently chosen one; both are answered.

The answer comes in seven parts, because the sources put the information in seven different places.

---

### Part A — A block of named fields in the file's header

**Label: documented.**

Blender can write a set of named pieces of information into the file, chosen one by one in an Output Properties panel.

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

The full list of what can be included, quoted exactly:

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

Source: the manual, https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html — date read 2026-07-30.

The custom note can be filled by a script, and the manual names render farms as the case:

> "It can be useful to use the Note field if you are setting up a render farm.
> Since you can script any information you like into it,
> such as an identifier for the render node or the job number."

Same page, same date.

The same switches appear in the Python API, each with its default, and their wording says plainly that the destination is the image's metadata:

> "use_stamp_camera
>
> Include the name of the active camera in image metadata (default True)"

> "use_stamp_date
>
> Include the current date in image/video metadata (default True)"

> "use_stamp_filename
>
> Include the .blend filename in image/video metadata (default True)"

> "use_stamp_frame
>
> Include the frame number in image metadata (default True)"

> "use_stamp_frame_range
>
> Include the rendered frame range in image/video metadata (default False)"

> "use_stamp_hostname
>
> Include the hostname of the machine that rendered the frame (default False)"

> "use_stamp_lens
>
> Include the active camera's lens in image metadata (default False)"

> "use_stamp_marker
>
> Include the name of the last marker in image metadata (default False)"

> "use_stamp_memory
>
> Include the peak memory usage in image metadata (default True)"

> "use_stamp_note
>
> Include a custom note in image/video metadata (default False)"

> "use_stamp_render_time
>
> Include the render time in image metadata (default True)"

> "use_stamp_scene
>
> Include the name of the active scene in image/video metadata (default True)"

> "use_stamp_sequencer_strip
>
> Include the name of the foreground sequence strip in image metadata (default False)"

> "use_stamp_time
>
> Include the rendered frame timecode as HH:MM:SS.FF in image metadata (default True)"

> "metadata_input
>
> Where to take the metadata from (default 'SCENE')
>
> SCENE
> Scene – Use metadata from the current scene.
>
> STRIPS
> Sequencer Strips – Use metadata from the strips in the sequencer."

Source: the API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — date read 2026-07-30.

The note's text itself is a separate string, and it appears in two places in the API:

> "stamp_note_text
>
> Custom text to appear in the stamp note (default "", never None)"

Same page, same date. And:

> "use_stamp_note
>
> User defined note for the render stamping (default "", never None)"

Source: the API documentation, https://docs.blender.org/api/5.2/bpy.types.Scene.html — date read 2026-07-30. (The same name carries a boolean on `RenderSettings` and a string on `Scene`. I report both as written; the sources do not reconcile them.)

**What the picture of the panel shows.** The manual's screenshot of this panel shows the fields as checkboxes in three columns, with these ticked: Date, Time, Render Time, Frame, Camera, Scene, Filename, and Include Labels. Unticked: Frame Range, Memory, Hostname, Lens, Marker, Strip Name, "Use Strip Me.." (the label is cut off), Note, and Burn Into Image. Font Size shows "12 px". Picture: `_images/render_output_properties_metadata_panel.png`, from the manual page https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html — date read 2026-07-30. **Label for this: documented** (it is part of the manual). Note that the picture shows Memory unticked while the API text above gives its default as True; I report the disagreement rather than resolving it, because nothing in the sources resolves it.

**An engine can add its own fields to the render result.** The API offers a call for this:

> "stamp_data_add_field(field, value)
>
> Add engine-specific stamp data to the result
>
> Parameters:
>
> field (str) – Field, Name of the stamp field to add (never None)
>
> value (str) – Value, Value of the stamp data (never None)"

Source: the API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderResult.html — date read 2026-07-30. The quoted words say the field is added to the *result*. Whether a field added that way then reaches the written file is a further step the quote does not carry, and nothing else in the sources says.

**Cycles' own files never mention it. Label: observed for the search, inferred for what follows from it.** What I did: searched every file supplied in `Blender Sources/cycles_5.2` for the text "stamp", ignoring upper and lower case, and separately for "stamp_data_add_field". What I did it to: the whole `cycles_5.2` folder, 420 files. What came back: zero matching lines for both searches. Machine and date: macOS on Apple Silicon (Darwin 25.5.0), 2026-08-11. **Inferred from that, with the step written out:** since Cycles' supplied files never call the engine-specific field call and never mention stamping, Cycles adds no fields of its own beyond the general list above. The step a reader must supply is that the supplied files are the whole of Cycles' Python side; if some part of Cycles not present in this folder added fields, the search would not see it.

---

### Part B — Only four formats keep the block of fields from Part A

**Label: documented.** This part is about Part A's block of named fields alone. It says nothing about Parts C, D, E or F, which are covered where they stand.

The manual carries a table of supported image formats with a Metadata column. Reading down that column, the formats marked with a tick are JPEG, OpenEXR, PNG and WebP. Marked with a cross are AVIF, BMP, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw and TIFF. (This is a reading of the table's ticks and crosses rather than a quotation of them; the rows themselves are not quoted here.)

> "Metadata is additional information stored inside an image file
> that describes how the image was created or how it should be interpreted."

> "Blender can write metadata into supported image formats
> when enabled in the Output Properties."

> "Limitations
>
> Not all formats support metadata,
> and some may only support a limited subset of fields.
>
> When using formats that do not support metadata,
> render information is not preserved in the saved file."

Source: the manual, https://docs.blender.org/manual/en/5.2/files/media/image_formats.html — date read 2026-07-30. The metadata panel page repeats the warning:

> "Only some image formats support metadata:
> See image formats."

Source: the manual, https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html — date read 2026-07-30.

So in a Targa, a TIFF, an HDR or a DPX, a finished picture holds none of Part A at all. Which of the fields survive into each of the four that do support metadata is **not in the sources** — the manual says only that some formats "may only support a limited subset of fields" and never says which subset for which format.

---

### Part C — The same text, optionally added to the render itself

**Label: documented**, for what the switches are and what they are said to do.

The information can also be added to the render as text.

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

Source: the manual, https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html — date read 2026-07-30.

With its settings and defaults from the API:

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

Source: the API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — date read 2026-07-30.

This is off by default.

That the text goes into the render itself rather than into a header is carried by the words quoted: "Add metadata as text to the render", and "Render the stamp info text in the rendered image".

**Inferred, step written out.** From there it follows that the text reaches every format the picture can be written as, including the ones Part B marks as keeping no metadata. The step a reader must supply is that whatever is in the render survives into every format Blender can write. Nothing quoted says it, and no page in the sources says whether Part B's list bears on Part C at all.

---

### Part D — How the colours should be read

**Label: documented.**

A written picture also carries information about the colour treatment it was made under, and this is on by default:

> "By default, Blender integrates with automatic color management in operating
> systems and saves images and videos with color space metadata. This means
> that usually, images viewed and saved in Blender look the same when opened in
> video players and browsers on the same computer."

Source: the manual, https://docs.blender.org/manual/en/5.2/render/color_management/system_configuration.html — date read 2026-07-30.

A colour profile can be embedded too, and both of these depend on how the colour configuration is written:

> "Use OpenColorIO v2 display_colorspaces and view_transforms with cie_xyz_d65_interchange
> intermediate display linear space. This is used to emulate the chosen display on the actual display.
> In older configurations without this, display emulation will be disabled and wide gamut and HDR display will
> not work well. Saving colorspace metadata for images and video also depends on this."

> "Include the interop ID from the Color Interop Forum
> for every color space and display color space that you can. This helps save image and video
> with correct colorspace information."

> "The icc_profile_name interchange attribute is supported, to embed ICC profiles when saving images."

Source: the manual, https://docs.blender.org/manual/en/5.2/render/color_management/opencolorio.html — date read 2026-07-30.

This is separate from Part A. Nothing in the sources ties it to the Metadata panel's switches, and nothing says it is limited to the four formats of Part B.

---

### Part E — The intended physical size

**Label: documented.**

> "Pixel Density
>
> The pixel density (often called PPI or DPI) controls the intended physical size for an image.
> This is often used for printing or the physical size when loaded into desktop publishing software.
>
> The X/Y density is calculated using the render X/Y aspect,
> making these values useful for storing the aspect ratio of non-square pixels.
>
> The pixel-density is meta-data which doesn't impact the quality of the image."

Source: the manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — date read 2026-07-30.

The format table in Part B has a separate DPI column. Reading down it, ticked are JPEG, OpenEXR, PNG, BMP and TIFF; crossed are WebP, AVIF, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa and Targa Raw. (Again a reading of ticks and crosses rather than a quotation.) Source: the manual, https://docs.blender.org/manual/en/5.2/files/media/image_formats.html — date read 2026-07-30. Note that this list is not the same as the metadata list: WebP keeps metadata but not density; BMP and TIFF keep density but not metadata.

---

### Part F — The file's own structure

**Label: documented**, for what the sources say an OpenEXR file can hold. The step from there to "the names are recorded" is inferred, and is written out at the end of this part.

An OpenEXR file can hold more than one picture inside it, divided the way the render was divided.

> "An OpenEXR file can store multiple layers and passes.
> This means OpenEXR images can be loaded into a Compositor keeping render layers and passes intact."

Source: the manual, https://docs.blender.org/manual/en/5.2/files/media/image_formats.html — date read 2026-07-30.

> "Interleave
>
> Use legacy interleaved storage of views, layers and passes for compatibility with
> applications that do not support more efficient multi-part OpenEXR files."

Same page, same date.

And for stereo work, the views:

> "Another option is to use multi-view OpenEXR files. This format can save multiple views in
> a single file and is backward compatible with old OpenEXR viewers (you see only one view though).
> Multi-view native support is only available to OpenEXR."

Source: the manual, https://docs.blender.org/manual/en/5.2/render/output/properties/stereoscopy/usage.html — date read 2026-07-30.

**Inferred, step written out:** a multi-layer or multi-view EXR therefore holds the names of the view layers, passes and views the render was built from. The step a reader must supply is that "keeping render layers and passes intact" means their names are recorded in the file; the manual does not say the word "names".

---

### Part G — What Blender can read back out of such a file

**Label: documented.** This is the nearest the sources come to describing the written result rather than the switches.

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

Source: the manual, https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/sidebar.html — date read 2026-07-30. The Image Editor offers the same reading:

> "Metadata
>
> Lists image metadata."

Source: the manual, https://docs.blender.org/manual/en/5.2/editors/image/sidebar.html — date read 2026-07-30.

---

### What the sources do NOT say

**Label: not in the sources**, for each of the following. The record of where I looked is below.

1. **The names the fields carry inside the file.** The manual gives the interface labels ("Render Time", "Frame Range") and the API gives the property names (`use_stamp_render_time`). Neither says what key or tag name is written into the PNG, JPEG, WebP or EXR. Nothing anywhere gives an example of a written header.
2. **Whether Blender records itself.** Nothing says a finished picture carries Blender's name, its version number, its build, or the name of the render engine that produced it. There is no "software" or "creator" field in the list, and no statement that one is added regardless of the switches.
3. **Nothing about render settings.** Sample counts, denoising, resolution percentage, device used — none of these appear in the metadata list or anywhere else as written into the file.
4. **Whether the fields are written when a still is saved by hand.** The Output panel page says "By default, animation renders are saved automatically, while still renders are not. Still images must be saved manually from the Image Editor or written using a File Output Node." (manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html, date read 2026-07-30). Nothing states whether the metadata block travels into a file saved that way. This is a real gap, and it matters, because it is the ordinary way a single frame reaches disk.
5. **Whether the File Output node writes any of it.** The File Output node page describes directories, names, formats and layers, and never mentions metadata at all. Source checked: the manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html, date read 2026-07-30.
6. **Which of the fields each of the four supporting formats keeps.** Only the general warning about "a limited subset of fields" exists.
7. **Whether Cryptomatte identity information is written into the file.** The only statement is a limitation: "Cryptomatte sidecars (metadata files) are not supported." (manual, https://docs.blender.org/manual/en/5.2/compositing/types/mask/cryptomatte.html, date read 2026-07-30). That says sidecar files are not supported. It does not say what, if anything, goes into the image file itself.

**Where I looked.** I searched the whole of `manual_5.2`, `api_5.2` and `release_notes_5.2` (and, for the engine question, `cycles_5.2`) for each of these terms, ignoring case: metadata, stamp, use_stamp, burn into, burn_into, metadata_input, exif, xmp, icc, nclx, chromaticities, tEXt, provenance, producer, creator, comments, "header field", software, "Software:", RenderTime, "engine name", "samples in metadata", watermark, sidecar, manifest, multi-view. The only pages in the manual that discuss metadata on a rendered image are the six quoted above (the Metadata panel, Supported Image Formats, the Output panel, the two colour management pages, the Sequencer preview sidebar) plus the two one-line mentions in the Image Editor's sidebar and overlays pages. Of those two one-line mentions, only the Image Editor's sidebar is quoted above (in Part G); what the Image Editor's overlays page says about metadata is not read out anywhere in this answer, so that page counts as found but not reported. The Asset Browser, USD, Grid Info, OSL and clip-editor pages also match "metadata" but are about other things entirely and were ruled out on that basis. "Software", "producer", "provenance" and "tEXt" returned no relevant hit anywhere.

**Release notes for 5.2.** The Metadata panel and the stamp fields are not mentioned in the 5.2 release notes at all — no change is recorded. Two nearby entries do exist, and both sit under the heading "## Movies", not "## Images":

> "- Stereoscopic metadata is exported. No need to post-process the video for correct playback in video players."

> "- Panoramic camera metadata is exported. Only full equirectangle cameras are supported."

Both sit under that heading; that is what places them, and I have not looked further into what either covers. The whole of the "## Images" heading in that file is one line:

> "- OpenEXR: Faster writing of half-precision (FP16) EXRs."

Source: the release notes, https://developer.blender.org/docs/release_notes/5.2/pipeline_io/ — date read 2026-07-30.

---

---

## What the hand tests showed — question 10

**LABEL for everything in this section: observed.** Added 2026-08-11 by the hand-test step, which ran a list of nineteen tests in Blender 5.2.0 on both of this project's machines. The full list and the full run record are in `Audits/Blender Research/Images/Hand tests — cross-sweep questions.md`.

**The two machines.** *Machine one is the Creator's Mac* — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card; its `hostname` prints `Artists-Studio-MacBook.local`; its exiftool is version 13.50. *Machine two is a rented Linux machine* — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory; its `hostname` prints `ubuntu`; its exiftool is version 12.40. Every Blender was started with `--factory-startup` and with its own settings folder.

**The scene used.** Blender's factory startup scene, with the scene renamed `SMZSCENE`, the camera renamed `SMZCAM` and its lens set to `37.5`, a marker named `SMZMARK`, frames 7 to 9 with frame 7 current, 64 × 64 pixels, Cycles at one sample, every `use_stamp_*` switch on, the note text `SMZNOTE-12345`, Burn into Image off, saved as `smzfile.blend`. Each written file was read with `exiftool -a -G1 -s`, and each OpenEXR also with a header reader written for this round that walks every part of the file.

**Where the two machines agreed and where they did not.** Every result below came out the same on both, with one exception: the `File` field holds that machine's own path to the blend-file, and `Hostname` holds that machine's own name.

### Gap 1 answered: the names the fields carry inside the file

Written to PNG on machine one, the whole of what exiftool reported beyond the picture's own structure:

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

On machine two the same names and values appeared, with `[PNG] File : /root/ht/smzfile.blend` and `[PNG] Hostname : ubuntu`. In OpenEXR the same fourteen names are separate header attributes. In JPEG they are separate comment markers, each reading `Blender:<name>:<value>` — for example `Blender:Note:SMZNOTE-12345`.

### Part B checked: the table is wrong about WebP, and gap 6 answered

The version offers `['AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']`, plus a setting this answer does not mention: `MEDIA-TYPE: IMAGE ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`. Asking for multi-layer OpenEXR by name while the media type is `IMAGE` fails outright with `enum "OPEN_EXR_MULTILAYER" not found in (…)`; setting the media type to `MULTI_LAYER_IMAGE` first makes it work.

- **Kept the fields: PNG, JPEG, OpenEXR (single-layer and multi-layer)** — and **all fourteen** in each, so the "limited subset of fields" of gap 6 does not bite for any of the three.
- **Kept nothing: AVIF, BMP, Iris, JPEG 2000, Targa, Targa Raw, Cineon, DPX, Radiance HDR, TIFF — and WebP.**

**WebP kept nothing**, on both machines, although Part B reads the manual's table as ticking it. Nothing appeared in Blender's console about a format that could not carry them.

### Gap 2 answered: Blender does record itself, in OpenEXR

Every OpenEXR written by 5.2.0 carries, **with every `use_stamp*` switch set to `False`**:

```
  [Software] (string, 17 bytes) = Blender 5.2.0 LTS
  [colorInteropID] (string, 16 bytes) = lin_rec709_scene
```

A **multi-layer** OpenEXR always additionally carries `[BlenderMultiChannel] = Blender V2.55.1 and newer`. A PNG, a JPEG, a WebP and an AVIF carry no such tag. With every switch off, a PNG held only the four Cycles fields below, the pixel density and the colour chunks. Nothing anywhere named the render engine, the sample count, the resolution or the view transform, which is gap 3 answered as well.

### Cycles does add fields of its own

Every file a Cycles render writes carries `cycles.ViewLayer.samples`, `cycles.ViewLayer.total_time`, `cycles.ViewLayer.render_time` and `cycles.ViewLayer.synchronization_time`. They survive with every stamp switch off, and they are absent from an otherwise identical EEVEE render (the engine list in 5.2 is `['BLENDER_EEVEE']`). So the inference in Part A — that Cycles adds nothing of its own — is **wrong**, and the step the answer warned about is exactly where it broke: the part of Cycles that adds them is not in the supplied folder.

### The other reading of the question, settled: one part carries the record

A multi-layer OpenEXR of two view layers with a Mist pass each, written at the factory setting of `use_exr_interleave` (off), is `multiPart=True` with `TOTAL PARTS: 6`. **Every one of the fourteen fields sits in part 0 and nowhere else.** Part 0, on machine one:

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

Parts 2 to 5 are the same thirteen with their own `name` and `channels`: `AAALAYER.Noisy Image`, `BBBLAYER.Combined`, `BBBLAYER.Mist`, `BBBLAYER.Noisy Image`. With Interleave turned on, the same render writes `TOTAL PARTS: 1`. **Part F's inference is confirmed:** each part carries a `name` attribute holding the view layer's and the pass's names, and the same names prefix the channel names.

### Part C's inference confirmed

Rendered at 640 × 360 with the burn-in on, to PNG and to Targa. Both pictures carry the same text: a block at the top left reading `File …`, `Date 2026/08/12 06:23:52`, `RenderTime 00:00.35`, `Peak Memory 17M`, `Hostname Artists-Studio-MacBook.local`, then `SMZNOTE-12345` on its own line with no label; a strip along the bottom reading `Marker SMZMARK Timecode 00:00:00:07 Frame 7 Frame Range 7:9 Camera SMZCAM Lens 37.50 Scene SMZSCENE`; and a `Strip <none>` item at the top right which at this width overlaps the end of the `File` line. So the text does reach a format Part B marks as keeping no metadata.

Burning in **adds to** rather than replaces: the PNG from that same run still carried the whole block of named fields. And the burned-in labels are not the field names in the file — the file's `Memory` is drawn as `Peak Memory`, and `Time` as `Timecode`.

### Gaps 4 and 5 answered: saving by hand, and the File Output node

`Image ▸ Save As` from an Image Editor wrote a file whose tag list is identical, name for name, to the file the render wrote itself. So the ordinary way a single frame reaches disk carries the whole block.

The compositor's File Output node also writes the fourteen names, from the same render, but with three values different and one group missing:

```
[PNG]           Camera                          : <none>
[PNG]           Lens                            : <none>
[PNG]           RenderTime                      : 00:00.00
```

and none of the four Cycles fields. Building that node setup in 5.2 needs `scene.compositing_node_group`; `scene.node_tree` no longer exists and `Scene.use_nodes` prints `DeprecationWarning: 'Scene.use_nodes' is expected to be removed in Blender 6.0`. The node's own settings are `directory` and `file_name` rather than `base_path` and `file_slots`, and its format starts at `MULTI_LAYER_IMAGE`.

> **The File Output node half could not be repeated at all. Added by the citation check, 2026-08-11.** Every API fact in this paragraph holds — `scene.node_tree` is gone, the node's properties are `directory` and `file_name`, and asking for PNG before the media type is set does fail with `enum "PNG" not found in ('OPEN_EXR_MULTILAYER')`. But following this description on both machines, **the node wrote no file at all**: with an item added to `file_output_items`, its format set to PNG after the media type was set to `IMAGE`, its input linked from the group input and a group output linked so that the ordinary render still wrote its own file, the node's folder was empty after a still render and after an animation render alike. What the original run did to make it write is not in these words. Back on the hand-test list as test 22.

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

So: the interface sockets were added **before** the Group Input and Group Output nodes, and those nodes then showed the socket. The media type must be set before the file format, and asking for PNG first fails with the error above. The item is made with `file_output_items.new('RGBA', 'SMZNODE')`; the choices the machine printed for that first argument are `['FLOAT', 'INT', 'BOOLEAN', 'VECTOR', 'RGBA', 'ROTATION', 'MATRIX', 'STRING', 'MENU', 'SHADER', 'OBJECT', 'IMAGE', 'GEOMETRY', 'COLLECTION', 'TEXTURE', 'MATERIAL', 'BUNDLE', 'CLOSURE']`. The node then reported `fo.inputs[0].name: SMZNODE is_linked: True`, `fo.mute: False`, `save_as_render: True`, `active_item_index: 0`, `node.warning_propagation: ALL`.

**The node's folder stayed empty every time**: not created when it did not exist, and left empty when it was made first, after a still render and after an animation render alike. The Output panel's own file was written every time from the same render. **The console said nothing about it** — no warning, no error, no line naming the node — and `bpy.ops.render.render` returned without raising. **The same on machine two, and the same in 5.0 and 5.1.**

**Four steps were added beyond the entry's own procedure, and they are named here.** *One*, six variations on the folder and file name were tried — a plain folder, a folder with a trailing separator, a file name ending `.png`, a file name ending `####`, `save_as_render` off, and a folder written as `//c6` — still and animation each:

```
CASE plain           still     -> []          CASE plain           animation -> []
CASE trailing_slash  still     -> []          CASE trailing_slash  animation -> []
CASE name_with_ext   still     -> []          CASE name_with_ext   animation -> []
CASE hash_in_name    still     -> []          CASE hash_in_name    animation -> []
CASE save_as_render  still     -> []          CASE save_as_render  animation -> []
CASE relative        still     -> NO SUCH FOLDER   CASE relative animation -> NO SUCH FOLDER
```

A search of the whole working folder afterwards found nothing called `smznode`, so the file was not written elsewhere under another path. *Two*, the same scene was rendered three times, with no group, with a group whose Invert node sits between the group input and the group output, and with that group's links removed. The three pictures came out different from one another (`A no group : 38b6c73632994fc3`, `B invert group : 95a1598010ad8425`, `C nothing linked: 8a6a1cd0c9f343c9`), so the group is executed during the render and the render result does reach its input, while the File Output node inside it still wrote nothing. *Three*, the whole build was run again on both machines from a Blender with a real window open rather than with `-b`. Machine two, 5.2:

```
BLENDER-VERSION: 5.2.0 LTS
BACKGROUND: False
LINKS: [('NodeGroupInput', 'Image', 'NodeGroupOutput', 'Image'), ('NodeGroupInput', 'Image', 'CompositorNodeOutputFile', 'SMZNODE')]
RENDER still, with a window open
NODE FOLDER after still: []
PANEL FOLDER after still: ['panel.png']
RENDER animation, with a window open
NODE FOLDER after animation: []
```

*Four*, in a windowed Blender `bpy.ops.node.new_compositing_node_group()` returned `{'FINISHED'}` and left `scene.compositing_node_group` as `None` in 5.0 and 5.1; the 5.2 run of that same script hung with no output and was killed, and was not retried because step three covered the windowed case without it.

**This run and the citation check's run came out the same; the first run came out differently.** All three stand as written and none settles the others.

### Gap 7 answered: Cryptomatte does go into the image file

```
  [cryptomatte/542cafa/conversion] (string, 17 bytes) = uint32_to_float32
  [cryptomatte/542cafa/hash] (string, 14 bytes) = MurmurHash3_32
  [cryptomatte/542cafa/manifest] (string, 58 bytes) = {"Cube":"a8fce865","Light":"c77d43e5","object":"055ec73d"}
  [cryptomatte/542cafa/name] (string, 22 bytes) = ViewLayer.CryptoObject
```

The sidecar limitation quoted above is about a separate file; the manifest itself is written into the image's own header.

### Part D checked

**No PNG written by 5.2 carried an ICC profile**, at the default colour management or with the view transform set to `Standard`. What a 5.2 PNG does carry is the format's own colour chunks — `SRGBRendering : Absolute Colorimetric`, `Gamma : 2.2`, `WhitePointX : 0.3127`, `WhitePointY : 0.329`, `RedX : 0.64`, `RedY : 0.33`, `GreenX : 0.3`, `GreenY : 0.6`, `BlueX : 0.15`, `BlueY : 0.06`. In OpenEXR the colour marking is the `colorInteropID` attribute above. Both were identical on macOS and on Linux, so the Wayland encoding sentence flagged below does not change what is written into a file — at least not on this Linux machine, whose display is Xorg rather than Wayland.

### The release notes' two Movies entries, checked

A side-by-side Stereo 3D MPEG-4 written by 5.2 is named `stereo_0007-0009.mp4` — 4.5 and 5.0 name the same render `stereo_0007-0009_L.mp4` — and is 128 × 64, so both eyes are in the picture. A separate render with the camera set to Panoramic and the Cycles panorama type set to Equirectangular wrote `pano_0007-0009.mp4`. **Neither file carries any stereo or projection marking that could be found.** Searching each for `st3d`, `sv3d`, `svhd`, `prji`, `proj`, `equi` and `ytmp` found none of them, and exiftool reported no stereo or projection tag. The same on both machines, and the same in 4.5, 5.0 and 5.1.

An ordinary MPEG-4 with H.264 over frames 7 to 9, every switch on, carries **none of the fourteen under any name**:

```
[Track1]        ColorProfiles                   : nclx
[Track1]        ColorPrimaries                  : BT.709
[Track1]        TransferCharacteristics         : BT.709
[Track1]        MatrixCoefficients              : BT.709
[Track1]        VideoFullRangeFlag              : Limited
[ItemList]      ContentCreateDate               : 2026/08/12 06:24:02
[ItemList]      Encoder                         : Lavf62.12.100
```

`Lavf` is the FFmpeg library, not Blender; 5.2 is on `Lavf62.12.100` where 4.5, 5.0 and 5.1 are on `Lavf61.7.100`.

### A field an engine adds for itself

A render engine registered for the test called `result.stamp_data_add_field("SMZFIELD", "SMZVALUE-98765")` on the render result and the call succeeded. Neither `SMZFIELD` nor `SMZVALUE-98765` appears anywhere in the PNG that render wrote — while the scene's own fourteen, in the same file, do. So the further step Part A declines to take is answered: a field added to the result does **not** reach the written file.

### The factory switches, the note properties, and the cut-off label

On a factory-fresh 5.2 scene, on both machines, `use_stamp_memory` reads `False`, while the property's own stated default is `True`. The screenshot describes a fresh Blender, the stated default describes the property; both are right about different things.

`RenderSettings.use_stamp_note` is a boolean, `RenderSettings.stamp_note_text` is the text, and `Scene.use_stamp_note` is a *string* described as "User defined note for the render stamping" — so the two API pages are not in conflict.

**No property in 5.2 has an interface name beginning "Use Strip Me".** The panel's own drawing code, read out of the running Blender, offers only two sequencer-related controls, both shown only when the Video Sequencer is in use:

```
        if rd.use_sequencer:
            layout.prop(rd, "metadata_input")
        …
        if rd.use_sequencer:
            col.prop(rd, "use_stamp_sequencer_strip", text="Strip Name")
```

### Part E checked: pixel density

With the density set to 300: PNG carries `PixelsPerUnitX : 11811` and `PixelsPerUnitY : 11811`; BMP carries `PixelsPerMeterX : 11811`, `PixelsPerMeterY : 11811`; TIFF carries `XResolution : 300`, `YResolution : 300`; OpenEXR carries one number, `xDensity : 300`; **JPEG carries none** (`ResolutionUnit : None`, `XResolution : 1`), although the table ticks it for DPI; WebP carries none, which agrees with the table. With the pixel aspect set to 2 across by 1 down, PNG and BMP write the two directions separately (150 and 300), **TIFF writes both as 150**, and OpenEXR still writes one number.

### Metadata taken from the Sequencer's strips

With `metadata_input` set to `STRIPS` and two image strips in the Video Sequencer, the written file's block is **replaced by the block read out of the foreground strip's own image file** — its date, its render time, its memory figure and its Cycles fields — and `Strip` reads `<none>`. With `SCENE` the date is the render's own, `Strip` reads `SMZSTRIP2`, and there are no Cycles fields.

> **The `STRIPS` half did not reproduce. Added by the citation check, 2026-08-11.** Run again in 5.2 on both machines, with two image strips (`SMZSTRIP1` on channel 1, `SMZSTRIP2` on channel 2) pointing at a PNG written by the same session, the file written with `metadata_input` set to `STRIPS` carried **no Blender metadata at all** — only the picture's own structure, and none of the source file's block. The `SCENE` half reproduced exactly, `Strip : SMZSTRIP2` and the absent Cycles fields included. Both runs stand as written; neither settles the other. Back on the hand-test list as test 20.

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

Not one Blender field, and no `Strip` line. The nine colour lines are there because 5.2 writes them into every PNG; they are not part of the metadata block. The `SCENE` file from the same session carried the fourteen, including `Strip : SMZSTRIP2`, `RenderTime : 00:00.00`, `Memory : 1M`, `Hostname : Artists-Studio-MacBook.local`, and none of the four `cycles.` fields. The source PNG carried those fourteen with `Strip : <none>`, `RenderTime : 00:00.12` and the four Cycles fields as well — eighteen in all.

**Machine two agreed.** Its `STRIPS` file printed the same list, and its bytes hash to the same value as machine one's, because the file holds nothing that names either machine. Its `SCENE` file reads `Hostname : ubuntu`, `File : /root/ht2/smzfile.blend`, `Memory : 1M` and `Date : 2026/08/12 12:23:41`.

Counting the Blender fields in each of the three files, on both machines: source 18, `STRIPS` 0, `SCENE` 14. **This run and the citation check's run came out the same; the first run came out differently.** All three stand as written and none settles the others. The full run record is in `Audits/Blender Research/Images/Hand tests — cross-sweep questions.md`.

---

### Not covered

**Every item in this list was settled by hand test on 2026-08-11 — see "What the hand tests showed" above — except the last, what a reader other than Blender makes of it.** What follows is the list as the researcher left it.

This answer does not settle: what the fields are actually called inside a written file; whether anything at all is written when no switch is on; whether the block reaches a file saved by hand from the Image Editor or written by a File Output node; which fields each format keeps; whether the render engine's name, Blender's version, or any render setting is recorded anywhere; and what a Cryptomatte render puts into the image file. It also does not cover video files, except to say the two 5.2 release-note entries above sit under the release notes' Movies heading. And it says nothing about what a *reader* other than Blender makes of any of this.

### Outside the interface

**Settled by hand test, 2026-08-11: it holds.** The same blend-file was rendered twice on each machine — once with `-b` and no window at all, once from a Blender with its window open — and the two files carry exactly the same tags, name for name, with only the date and the render time differing as they must. That was true in 5.2.0 on machine one (the Creator's Mac, macOS 26.5.2, Apple M1 Pro) and on machine two (the rented Linux machine, Ubuntu 22.04.5, two NVIDIA RTX 4000 Ada cards), and the two machines agreed. So the argument from silence below turns out to be right, and the `use_single_layer` comparison was a good guide. The one documented difference quoted below did **not** hold: the OpenEXR Preview option wrote a JPEG copy beside the EXR in *both* cases, so a single frame rendered by a click gets the preview copy too. What follows is what the sources said before the tests, kept for the trail.

**One place where it holds by being absent both ways, checked 2026-08-12.** The compositor's File Output node was built and rendered in 5.2.0 LTS on both machines, first with `-b` and no window and then from a Blender with its window open. Its folder was empty in every case, after a still render and after an animation render. So having a window makes no difference there either — the node wrote nothing with one and nothing without one.

The sources do not say. Nothing on the Metadata panel page or in the API entries carries a note about rendering from outside Blender's own window, and no search term above turned up such a note.

Three things bear on it, and I give them with their labels:

- **Documented.** The manual expects this information to be filled by a script in exactly that situation: "It can be useful to use the Note field if you are setting up a render farm. Since you can script any information you like into it, such as an identifier for the render node or the job number." (manual, https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html, date read 2026-07-30). Two of the fields — Hostname and the blend-file's Filename — only make sense as a record of which machine and which file did the work.
- **Inferred, step written out.** Every switch is a property of the scene's render settings (`bpy.types.RenderSettings`), so it is saved in the blend-file and applies to whatever renders that scene. The step a reader must supply is that a render started from outside uses the same scene settings as one started by a click.
- **Inferred, step written out.** The same API page does flag one setting as interface-only — "use_single_layer … Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line." (API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, date read 2026-07-30) — and carries no such flag on any of the stamp properties. The step a reader must supply is that the absence of the flag means the setting is not interface-only. That is an argument from silence and should be treated as weak.

One documented difference does exist nearby, though it is about a companion file rather than this block: "Preview — When rendering animations (or single frames via command line), save a JPEG copy of the image, for a quick preview." (manual, https://docs.blender.org/manual/en/5.2/files/media/image_formats.html, date read 2026-07-30). So for EXR output, a single frame rendered from the command line gets a preview copy that a single frame rendered by a click does not.

### Operating system

**Settled by hand test for macOS and Linux, 2026-08-11; Windows is still open.** Every one of the nineteen tests was run in 5.2.0 on both machines, and every result came out the same on macOS 26.5.2 with Apple's own graphics and on Ubuntu 22.04.5 with NVIDIA cards — the same field names, the same formats keeping and dropping them, the same six-part shape of a multi-part OpenEXR, the same colour chunks in a PNG and the same `colorInteropID` in an EXR. Two values differ, and only because they describe the machine: the `File` field holds that machine's own path, and `Hostname` holds that machine's own name — `Artists-Studio-MacBook.local` on machine one and `ubuntu` on machine two, each matching exactly what that machine's `hostname` command prints. The Wayland encoding sentence flagged below did not change anything written into a file, though the Linux machine's display is Xorg rather than Wayland, so that particular case is untried. **Windows was not tried: no Windows machine has run it.** What follows is what the sources said before the tests, kept for the trail. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Checked again on 2026-08-12 and still holding for macOS and Linux.** The second batch of hand tests ran three more tests in 5.2.0 LTS on both machines — the Sequencer-strips test, the part-by-part shape of a multi-part OpenEXR, and the compositor's File Output node — and the two machines agreed about every one, down to the thirteen attributes in each part of the OpenEXR and the empty node folder. **Windows is still untested for the same reason.**

The sources do not say that any of Part A, B, C, E or F differs by operating system. No operating system is named anywhere on the Metadata panel page, the Supported Image Formats page, or the API pages. Part G is not covered by that record either way: the Sequencer preview sidebar page and the Image Editor sidebar page were not read for operating system differences, so nothing is claimed about them here.

Part D is the exception, and only partly. The colour management pages are written around operating system differences, but the differences they name are about *displaying* rather than *writing*:

> "Enabling Wide Gamut and HDR
>
> For viewing wide gamut and HDR colors, a monitor with those capabilities is
> required.
>
> macOS: Available on any Apple Silicon device.
>
> Linux: Use Wayland, and set the Vulkan backend in the Blender system preferences."

and:

> "On Linux, Wayland color management supports displaying both SDR and HDR content
> similar to macOS. Because Wayland does not have a distinct HDR mode, Blender
> automatically switches between sRGB and gamma 2.2 image encoding depending on
> whether an SDR or HDR display was chosen."

Source: the manual, https://docs.blender.org/manual/en/5.2/render/color_management/system_configuration.html — date read 2026-07-30. That last quotation is the one place where an operating system is tied to an encoding choice, and I cannot tell from the words whether "image encoding" there means what is sent to the screen or what is written to a file. I flag it rather than resolve it.

One field is inherently machine-specific rather than operating-system-specific: Hostname, "Includes the rendering machine's hostname". The sources say nothing about how that name is obtained or whether it differs by operating system.

---

## Things a person with Blender open could settle in a few minutes

The eight items that stood here were all run on 2026-08-11, in Blender 5.2.0 on both of this project's machines, and their answers are in "What the hand tests showed" above. What is left open needs a machine this project does not have.

1. **Whether any of this differs on Windows.** Item 8 asked for the colour comparison to be repeated on a second operating system; it was, on macOS 26.5.2 and on Ubuntu 22.04.5, and the two agreed. **A third comparison on Windows was not run: no Windows machine has run it.** Until it has one, nothing above speaks for Windows. The Wayland half of the same question is also untried: the Linux machine's display server is Xorg.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Images, Blender 5.2

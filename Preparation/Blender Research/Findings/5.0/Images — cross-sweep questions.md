# Images — cross-sweep questions (Blender 5.0)

Sources consulted, all read on **2026-07-30** (the snapshot date given to me): the Blender 5.0 manual (`Blender Sources/manual_5.0`, including one image from its `_images` folder), the Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), the Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own files (`Blender Sources/cycles_5.0`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 10. What does a finished picture hold, in every part of the file, about what produced it?

### A note on the wording, before the answer

This question can be read two ways, and I could not settle which was meant, so I have answered the reading I think is meant and dealt with the other one separately at the end.

- **The reading I answer:** what information about the render is recorded in a saved image file, in all the places in that file where such information can sit.
- **The other reading:** Blender 5.0 can write an OpenEXR file made of several *parts*, in the technical sense that format uses. On that reading the question is asking whether the information is repeated in every one of those parts. I answer that separately under "The other reading" below, and the answer there is that the sources do not say.

### Short answer

Blender has a feature whose whole purpose is this. It is called **Metadata**, and it puts information about the render into the output. The person rendering chooses which items go in, from a list of fourteen the panel offers. The information can go into the file in two quite different places: as metadata carried by the file, and as text painted into the picture itself. Of the thirteen image formats the manual's table lists, only four are marked as carrying the first kind.

Beyond those fourteen, the sources do not describe anything else the file holds about what produced it — no version number, no engine name, no render settings — with two exceptions: colour space, described in part F, and whatever a render engine adds for itself, described in part G.

---

### A. There is a Metadata feature, and it writes into the render output

**LABEL: documented**

> "The Metadata panel includes options for writing metadata into render output."

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/metadata.html — read 2026-07-30.

The panel sits in the Output Properties, listed there between "Encoding" and "Post Processing":

> "Encoding
>
> Metadata
>
> Note
>
> Burn into Image
>
> Post Processing"

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/index.html — read 2026-07-30.

**Not covered:** this says the options exist and that they write into render output. It does not say at what moment the writing happens, nor whether saving a picture by hand afterwards keeps it.

---

### B. The list the panel offers — fourteen items

**LABEL: documented**

The manual gives the list under the heading "Include", each with its own line. Quoted exactly and in the manual's order:

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

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/metadata.html — read 2026-07-30.

The Note is free text the person writing the file supplies, and the manual says outright that this is the place to put anything else you want recorded:

> "It can be useful to use the Note field if you are setting up a render farm. Since you can script any information you like into it, such as an identifier for the render node or the job number."

Source: same page, read 2026-07-30.

The Python API repeats the same fourteen as separate switches, and its wording is worth having because it says which of them apply to video as well as to still pictures. Quoted exactly, name then description:

> "use_stamp_camera — Include the name of the active camera in image metadata"
> "use_stamp_date — Include the current date in image/video metadata"
> "use_stamp_filename — Include the .blend filename in image/video metadata"
> "use_stamp_frame — Include the frame number in image metadata"
> "use_stamp_frame_range — Include the rendered frame range in image/video metadata"
> "use_stamp_hostname — Include the hostname of the machine that rendered the frame"
> "use_stamp_lens — Include the active camera's lens in image metadata"
> "use_stamp_marker — Include the name of the last marker in image metadata"
> "use_stamp_memory — Include the peak memory usage in image metadata"
> "use_stamp_note — Include a custom note in image/video metadata"
> "use_stamp_render_time — Include the render time in image metadata"
> "use_stamp_scene — Include the name of the active scene in image/video metadata"
> "use_stamp_sequencer_strip — Include the name of the foreground sequence strip in image metadata"
> "use_stamp_time — Include the rendered frame timecode as HH:MM:SS.FF in image metadata"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30.

The custom note's own text is held separately. The API lists a string property for it:

> "stamp_note_text — Custom text to appear in the stamp note. Type: string, default "", (never None)"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30.

A second quote, taken from a different page, gives a string property too, but under a name already quoted above as a switch:

> "use_stamp_note — User defined note for the render stamping. Type: string, default "", (never None)"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.Scene.html — read 2026-07-30.

**Caution.** The name `use_stamp_note` appears twice in this answer with two different descriptions: once, on `bpy.types.RenderSettings`, as the boolean switch that includes a custom note, and once, on `bpy.types.Scene`, as a string described as the note itself. They are two different properties on two different types, and **nothing in either quote says that either is an error**. Part C's count of the defaults treats the `RenderSettings` one as the switch, which is what its own page calls it. *(Corrected by the citation check, 2026-08-11: this caution said "One of the two has been written down wrongly", which the quoted words do not carry — and which this document's own hand-test section below then contradicts.)*

**Not covered:** the manual and API give the items by their screen names and by their Python names. Neither says what name each item is written under **inside the file**. The one glimpse I found of an in-file label is in the 5.0 corrective-release list, which quotes one:

> "Metadata output for "Peak Memory" displays "ddM" in Blender 5.0. (blender/blender#150524)"

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/corrective_releases/ — read 2026-07-30.

**LABEL for what that quote carries: inferred.** What the words say outright is that a name "Peak Memory" is used, and that its metadata output displayed "ddM" in 5.0. **The step I took:** reading that name as the label the item carries inside the file. The words do not say so — "Peak Memory" could equally be the name the notes use for the item the manual calls Memory. Nor do the words say the fault was fixed; what they say is that it is listed among 5.0's corrective releases. Either way the note gives nothing about the labels of the other thirteen.

---

### C. Which of the fourteen are on unless changed

**LABEL: documented**

The API gives a default for each. On by default: Date, Filename, Frame, Memory, Render Time, Scene, Camera, Time — each stated as "boolean, default True". Off by default: Frame Range, Hostname, Lens, Marker, Note, Strip Name — each stated as "boolean, default False".

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30. The page states these one property at a time; the fourteen are gathered here rather than quoted line by line, so only the repeated wording is in quotation marks.

The manual's picture of the panel shows a real set of switches. I opened `_images/render_output_properties_metadata_panel.png`, the "Metadata panel." picture on https://docs.blender.org/manual/en/5.0/render/output/properties/metadata.html. Visible in it: Date, Time, Render Time, Frame, Camera, Scene and Filename are ticked; Frame Range, Memory, Hostname, Lens, Marker, Strip Name and the neighbouring "Use Strip Me…" box are not ticked; the Note and Burn Into Image sections are present with their own unticked boxes, and under Burn Into Image sit Font Size (showing "12 px"), Text Color, Background and a ticked Include Labels.

**LABEL for the picture: documented** (it is part of the manual).

**Not covered:** the picture shows Memory unticked while the API says its default is True. I do not know why they differ, and I am not going to guess; a screenshot shows one moment on one machine, not the shipped default. Where the two disagree, I would trust the API's stated default, but that preference is mine, not the sources'.

---

### D. The information goes into the file in two different places

**LABEL: documented**

First, as metadata carried by the file. That is what the panel's opening sentence describes ("writing metadata into render output", quoted in part A).

Second, painted into the picture as visible text, which is a separate switch:

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
> Displays the labels before the metadata text. For example, "Camera" in front of the camera name, etc."

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/metadata.html — read 2026-07-30.

The API names this switch and gives its default:

> "use_stamp — Render the stamp info text in the rendered image. Type: boolean, default False"

and the appearance settings:

> "stamp_background — Color to use behind stamp text. Type: float array of 4 items in [0, 1], default (0.0, 0.0, 0.0, 0.25)"
> "stamp_font_size — Size of the font used when rendering stamp text. Type: int in [8, 64], default 12"
> "stamp_foreground — Color to use for stamp text. Type: float array of 4 items in [0, 1], default (0.8, 0.8, 0.8, 1.0)"
> "use_stamp_labels — Display stamp labels ("Camera" in front of camera name, etc.). Type: boolean, default True"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30.

So the burned-in text is off unless switched on.

**LABEL for what follows: inferred.** When Burn into Image is on, the information sits in the picture itself rather than beside it, so it does not depend on the format being one that carries metadata, and it cannot be removed without altering the picture. **The step I took:** the quoted words say only "Add metadata as text to the render". That such text survives every format, and that it cannot be stripped, is my own reasoning about what text painted into a picture is. No source says it.

**Not covered:** the sources do not say whether switching on Burn into Image also writes the same items as file metadata, or instead of it. Both switches exist independently; nothing I found states how they interact.

---

### E. Where the information is taken from

**LABEL: documented**

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

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/metadata.html — read 2026-07-30.

> "metadata_input — Where to take the metadata from. SCENE — Scene – Use metadata from the current scene. STRIPS — Sequencer Strips – Use metadata from the strips in the sequencer. Type: enum in ['SCENE', 'STRIPS'], default 'SCENE'"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30.

The second choice matters for the question: the two choices are the whole of the setting, so with the second one set the metadata is taken from the strips in the Sequencer rather than from the current scene.

**Not covered:** what "metadata from the strips" consists of, and what happens when strips disagree, is not stated on either page.

---

### F. Which formats the table marks as carrying it — four of thirteen

**LABEL: documented**

> "Only some image formats support metadata: See image formats."

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/metadata.html — read 2026-07-30.

The list it points at is a table with a Metadata column. Reading down that column, the formats marked ✓ are **JPEG, OpenEXR, PNG and WebP**. The formats marked ✗ are **BMP, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw and TIFF**. The table's own words, in the order they appear:

> "Format
>
> Channel Depth
>
> Alpha
>
> Metadata
>
> DPI
>
> Extensions
>
> JPEG
>
> 8bit
>
> ✗
>
> ✓
>
> ✓
>
> .jpg .jpeg
>
> OpenEXR
>
> float 16, 32bit
>
> ✓
>
> ✓
>
> ✓
>
> .exr
>
> PNG
>
> 8, 16bit
>
> ✓
>
> ✓
>
> ✓
>
> .png
>
> WebP
>
> 8bit
>
> ✓
>
> ✓
>
> ✗
>
> .webp
>
> BMP
>
> 8bit
>
> ✓
>
> ✗
>
> ✓
>
> .bmp"

Source: manual, https://docs.blender.org/manual/en/5.0/files/media/image_formats.html — read 2026-07-30. The quoted block stops at BMP. The remaining rows (Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw, TIFF) follow the same shape and all carry ✗ in the Metadata column; TIFF, like BMP, carries ✓ in the DPI column. Those eight rows are summarised here rather than quoted.

**Not covered:** the table says which formats support metadata. It does not say what happens to the chosen items when the format does not — whether they are silently dropped, or a warning appears. It also does not say where inside each of the four supporting formats the information is put.

**One further thing a picture can hold about how it was made — its colour space.** Blender 5.0 added this for OpenEXR:

> "OpenEXR files now support Color Interop ID metadata, to automatically detect the file colorspace. (blender/blender!145837, blender/blender!146809)"

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/pipeline_io/ — read 2026-07-30.

The manual's guidance for people writing colour configurations describes this from the other side, and makes clear it depends on the configuration in use:

> "Use OpenColorIO v2 display_colorspaces and view_transforms with cie_xyz_d65_interchange intermediate display linear space. […] Saving colorspace metadata for images and video also depends on this."

and

> "Include the interop ID from the Color Interop Forum for every color space and display color space that you can. This helps save image and video with correct colorspace information."

Source: manual, https://docs.blender.org/manual/en/5.0/render/color_management/opencolorio.html — read 2026-07-30.

**Not covered:** these say colour-space information can be saved into images, and that OpenEXR now carries a Color Interop ID. They do not say which other formats carry colour-space information, nor whether this is on by default, nor what is written when the configuration lacks an interop ID.

---

### G. A render engine can add its own items

**LABEL: documented**

> "stamp_data_add_field(field, value)
>
> Add engine-specific stamp data to the result
>
> Parameters:
>
> field (string, (never None)) – Field, Name of the stamp field to add
>
> value (string, (never None)) – Value, Value of the stamp data"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.RenderResult.html — read 2026-07-30.

So the fourteen items are not a hard ceiling: the thing doing the rendering can attach further named values to the result, and those are called stamp data like the rest.

**LABEL for what follows: observed.** I searched every file supplied in `Blender Sources/cycles_5.0` (403 files), case-insensitively, for the word "stamp" and for the word "metadata". **"stamp" has no matches at all, anywhere in the folder.** The only matches for "metadata" are in `cycles_5.0/osl.py`, and they are about parameter descriptions on custom OSL shaders — for example line 58, `metadata = {meta.name: meta.value for meta in param.metadata}` — not about render output. There is no call to `stamp_data_add_field` anywhere in the supplied Cycles files. *(Corrected by the citation check, 2026-08-11: this paragraph said the "stamp" search was the one that hit `osl.py`. Re-running both searches, "stamp" returns nothing anywhere and the `osl.py` hits quoted here are hits for "metadata". The file count, the quoted line and the conclusion are unchanged.)*

**LABEL: inferred** — from that search, the Cycles files supplied here add no engine-specific items of their own. **The step I took:** absence of the call in these files means these files do not make it. But what is supplied in `cycles_5.0` is the add-on's Python side plus compiled OSL shaders; the rendering core itself is not among these files, so this does not settle whether Cycles adds such items from elsewhere. Treat it as a partial negative, not a full one.

**Not covered:** whether EEVEE or any other engine adds items; whether engine-added items obey the same on/off switches; whether they reach the file or only the render result in memory.

---

### H. What I could not find — and where I looked

**LABEL: not in the sources**

None of the following is stated anywhere in the manual, the API documentation, the release notes or the Cycles files supplied:

1. **The names the items are written under inside the file.** Only one in-file label surfaced anywhere: "Peak Memory", and only because a corrective-release note quotes it while describing a bug (part B).
2. **Whether Blender records itself.** Nothing says the file holds the Blender version, the build, the render engine's name, the sample count, the resolution settings, or any other render setting. The fourteen items are the whole documented list, and none of them is any of these.
3. **Where inside each format the information physically sits** — file header, text chunk, comment field or otherwise.
4. **Whether metadata is written when a picture is saved by hand** from the Image Editor, or written by the compositor's File Output node. I searched the compositor output-node pages and the Save Images section of the image-formats page and found no mention of either.
5. **Whether the small preview copy** the manual describes ("save a JPEG copy of the image, for a quick preview", https://docs.blender.org/manual/en/5.0/files/media/image_formats.html) carries the same information.

**Where I looked, so this counts as a real answer.** Across `manual_5.0`, `api_5.0` and `release_notes_5.0` I searched for: *metadata*, *stamp*, *stamping*, *burn*, *watermark*, *Exif*, *XMP*, *IPTC*, *ICC*, *color profile*, *tEXt*, *chromaticities*, *interop*, *Color Interop*, *header*, *EXR header*, *RenderTime*, *Peak Memory*, *Software*, *provenance*, *multi-part*, *multipart*, *Interleave*. I read in full: the Metadata page and the Output Properties index; the Supported Graphics Formats page including its OpenEXR section; `bpy.types.RenderSettings`, `bpy.types.RenderResult`, `bpy.types.ImageFormatSettings` and the stamp entry in `bpy.types.Scene`; the OpenColorIO configuration page; the Image Editor sidebar page (which only says "Metadata — Lists image metadata.", https://docs.blender.org/manual/en/5.0/editors/image/sidebar.html); and the metadata and colour lines in the 5.0 release notes for `pipeline_io`, `cycles`, `python_api` and `corrective_releases`. I ruled out the whole `advanced/command_line` section of the manual by searching it for *stamp* and *metadata* and finding nothing at all. I searched all 403 supplied Cycles files for *stamp* and *metadata*.

---

### The other reading — "every part of the file" meaning the parts of a multi-part OpenEXR

**LABEL: not in the sources**

Blender 5.0 does write OpenEXR files in parts, and this changed in 5.0:

> "Multilayer OpenEXR now saves multi-part files by default, for more efficient loading of individual views, layers and passes in other applications. The "Interleave" option can be turned on to get the old behavior, for compatibility with older applications that do not support multi-part files well. (blender/blender!146650)"

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/pipeline_io/ — read 2026-07-30.

The manual and API describe the switch that turns it off:

> "Interleave
>
> Use legacy interleaved storage of views, layers and passes for compatibility with applications that do not support more efficient multi-part OpenEXR files."

Source: manual, https://docs.blender.org/manual/en/5.0/files/media/image_formats.html — read 2026-07-30.

> "use_exr_interleave — Use legacy interleaved storage of views, layers and passes for compatibility with applications that do not support more efficient multi-part OpenEXR files. Type: boolean, default False"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.ImageFormatSettings.html — read 2026-07-30.

But **nothing in any source says whether the render's metadata is repeated in every part of such a file, or written once.** Where I looked for it is the record in part H: the terms searched across the manual, the API documentation and the release notes included *multi-part*, *multipart* and *Interleave*, and I read the Supported Graphics Formats page in full including its OpenEXR section. That matters in practice — a tool that reads only one part of a multi-part file might or might not see the information — and I could not settle it from what I was given. This is one to check by hand; see the list at the end.

---

---

## What the hand tests showed — question 10

**LABEL for everything in this section: observed.** Added 2026-08-11 by the hand-test step, which ran a list of nineteen tests in Blender 5.0.1 on both of this project's machines. The full list and the full run record are in `Audits/Blender Research/Images/Hand tests — cross-sweep questions.md`.

**The two machines.** *Machine one is the Creator's Mac* — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card. Its `hostname` command prints `Artists-Studio-MacBook.local`, and its exiftool is version 13.50. *Machine two is a rented Linux machine* — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory. Its `hostname` prints `ubuntu`, and its exiftool is version 12.40. Every Blender was started with `--factory-startup` and with its own settings folder.

**The scene used.** Blender's factory startup scene, with the scene renamed `SMZSCENE`, the camera renamed `SMZCAM` and its lens set to `37.5`, a marker named `SMZMARK`, frames 7 to 9 with frame 7 current, 64 × 64 pixels, Cycles at one sample, every `use_stamp_*` switch on, the note text `SMZNOTE-12345`, Burn into Image off, saved as `smzfile.blend`. Each written file was read with `exiftool -a -G1 -s`, and each OpenEXR also with a header reader written for this round that walks every part of the file.

**Where the two machines agreed and where they did not.** Every result below came out the same on machine one and machine two, with one exception: the `File` field holds that machine's own path to the blend-file, and the `Hostname` field holds that machine's own name.

### The names the items carry inside the file — part H's first gap

Written to PNG on machine one, the whole of what exiftool reported beyond the picture's own structure:

```
[PNG]           File                            : …/smzfile.blend
[PNG]           Note                            : SMZNOTE-12345
[PNG]           Date                            : 2026/08/12 06:23:08
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

On machine two the same names and values appeared, with `[PNG] File : /root/ht/smzfile.blend` and `[PNG] Hostname : ubuntu`.

So the fourteen items carry the in-file names `File`, `Note`, `Date`, `Marker`, `Time`, `Frame`, `FrameRange`, `Camera`, `Lens`, `Scene`, `Strip`, `RenderTime`, `Memory`, `Hostname`. They are separate named entries. In OpenEXR they are separate header attributes of the same names. In JPEG they are separate comment markers, each reading `Blender:<name>:<value>` — for example `Blender:Note:SMZNOTE-12345`.

**The "Peak Memory" line in the corrective-release note is the burn-in label, not the in-file name.** In the file the item is called `Memory`. In the text burned into the picture it is drawn as `Peak Memory`. And in 5.0.1 its value is a real number: the file reads `Memory : 2M` and the burned-in line reads `Peak Memory 18M`. Nothing read `ddM` on either machine.

### Which formats keep it — the table is wrong about WebP

The version offers `['JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']`, and a new setting this answer does not mention: `MEDIA-TYPE: IMAGE ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`. Asking for multi-layer OpenEXR by name while the media type is `IMAGE` fails outright:

```
SETFORMAT-ERROR: OPEN_EXR_MULTILAYER bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in ('JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')
```

Setting the media type to `MULTI_LAYER_IMAGE` first makes it work. The same frame was then written in every still format, on both machines.

- **Kept the items: PNG, JPEG, OpenEXR (single-layer and multi-layer).**
- **Kept nothing: BMP, Iris, JPEG 2000, Targa, Targa Raw, Cineon, DPX, Radiance HDR, TIFF — and WebP.**

**WebP kept nothing**, on both machines, although part F reads the manual's table as marking it ✓. The whole of what exiftool printed for it was `FileType`, `MIMEType`, `VP8Version`, `ImageWidth`, `HorizontalScale`, `ImageHeight` and `VerticalScale`. Nothing appeared in Blender's console about a format that could not carry the items, which answers part F's "Not covered": they are simply absent, silently.

### Part H's second gap: Blender does record itself, in OpenEXR

Every OpenEXR written by 5.0.1 carries, on both machines and **with every `use_stamp*` switch set to `False`**:

```
  [Software] (string, 13 bytes) = Blender 5.0.1
  [colorInteropID] (string, 16 bytes) = lin_rec709_scene
```

A **multi-layer** OpenEXR additionally always carries `[BlenderMultiChannel] = Blender V2.55.1 and newer`. A PNG, a JPEG and a WebP carry no such tag: with every switch off, a PNG held only the four Cycles items below and the pixel density. Nothing anywhere named the render engine, the sample count, the resolution or the view transform.

### Part G answered: Cycles does add items of its own

Every file a Cycles render writes carries `cycles.ViewLayer.samples`, `cycles.ViewLayer.total_time`, `cycles.ViewLayer.render_time` and `cycles.ViewLayer.synchronization_time`. They survive with every stamp switch off, and they are absent from an otherwise identical EEVEE render (the engine list in 5.0 is `['BLENDER_EEVEE']`). So the "partial negative" flagged in part G was right to be partial: the part of Cycles that adds them is not in the supplied folder. No tag in either file names an engine.

### The other reading — the record sits in one part only

`use_exr_interleave` exists in 5.0 (`HAS-use_exr_interleave: True`). With it off, which is its factory value, a multi-layer OpenEXR of two view layers with a Mist pass each is `multiPart=True` with `TOTAL PARTS: 6`. **Every one of the fourteen items sits in part 0 and nowhere else.** Parts 1 to 5 hold thirteen attributes each, all structural:

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

> **This part-1 block did not reproduce. Added by the citation check, 2026-08-11.** Run again with the same scene — two view layers with a Mist pass each, Interleave off — Blender 5.0.1 on machine one and 5.0 on machine two both wrote **twelve** attributes in part 1, with **no `xDensity`**: `[channels]`, `[chunkCount]`, `[colorInteropID]`, `[compression]`, `[dataWindow]`, `[displayWindow]`, `[lineOrder]`, `[name]`, `[pixelAspectRatio]`, `[screenWindowCenter]`, `[screenWindowWidth]`, `[type]`, then `(12 attributes in part 1)`. Every attribute name, type, byte count and value that both runs share matched exactly. The thirteen-attribute block above, with `xDensity`, is what **5.2** writes — the re-run got exactly it from 5.2 on both machines. The count of parts, and the finding that all fourteen fields sit in part 0 and nowhere else, reproduced without change. Both runs stand as written; neither settles the other. Back on the hand-test list as test 21.

**Run a third time, 2026-08-12 — twelve attributes, and no `xDensity`.** Test 21 on the hand-test list, run on **machine one, the Creator's Mac** (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card) and on **machine two, a rented Linux machine** (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory). Blender 5.0.1, 5.1.2 and 5.2.0 LTS each rendered frame 7 of the standard scene in the same sitting on the same machine, with two view layers `AAALAYER` and `BBBLAYER`, the Mist pass on in each, media type `MULTI_LAYER_IMAGE`, format OpenEXR MultiLayer, and `use_exr_interleave` left untouched. Every part of each file was walked with an OpenEXR header reader written fresh for this run. What 5.0.1 printed on machine one:

```
BLENDER-VERSION: 5.0.1
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
WROTE: multi.exr 108232 bytes
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

5.1.2 gave exactly that. 5.2.0 gave those twelve **plus** `[xDensity] (float, 4 bytes) = 72.0`, ending `(13 attributes in part 1)`. The counts across all six parts: 5.0 and 5.1 both `part 0: 37, parts 1 to 5: 12 each`; 5.2 `part 0: 37, parts 1 to 5: 13 each`. Counting the raw bytes `xDensity` in each written file gave `5.0: 1`, `5.1: 1`, `5.2: 6` — so 5.0 writes it in part 0 only. `render.ppm_factor` read `72.0` in all three versions, so the setting that fills `xDensity` was the same in each.

**Machine two agreed** — the same counts, the same part-1 block and the same raw byte counts. Reading the same three files with the header reader the earlier round used gave the same twelve for 5.0 and the same thirteen for 5.2, so the difference is not the reader's. Six parts, `multiPart=True`, and all fourteen fields in part 0 and nowhere else reproduced in all three versions on both machines.

**This run and the citation check's run came out the same; the first run came out differently.** All three stand as written and none settles the others.

With Interleave turned on, the same render writes `TOTAL PARTS: 1`. So a tool that reads only one part of a multi-part file sees the record only if that part is the first. The same on both machines. Each part carries its own `name`, which is where the view layers' and passes' names are recorded.

### Part D answered: burning in adds to, and does reach every format

Rendered at 640 × 360 with the burn-in on, to PNG and to Targa. Both pictures carry the same text: a block at the top left reading `File …`, `Date 2026/08/12 06:23:11`, `RenderTime 00:00.33`, `Peak Memory 18M`, `Hostname Artists-Studio-MacBook.local`, then `SMZNOTE-12345` on its own line with no label; a strip along the bottom reading `Marker SMZMARK Timecode 00:00:00:07 Frame 7 Frame Range 7:9 Camera SMZCAM Lens 37.50 Scene SMZSCENE`; and a `Strip <none>` item at the top right which at this width overlaps the end of the `File` line. So the inference in part D holds: the text reaches Targa, which keeps no metadata.

And part D's "Not covered" is answered: the two switches **do not** replace one another. The PNG written in the same run still carried the whole block of named items as well as the painted text.

### Part E answered: what "Sequencer Strips" puts in

With `metadata_input` set to `STRIPS` and two image strips in the Video Sequencer, the written file's block is **replaced by the block read out of the foreground strip's own image file**. The whole of the difference between the two files, machine one (`<` is the strips setting, `>` is the scene setting):

```
< [PNG]           Date                            : 2026/08/12 06:23:08
> [PNG]           Date                            : 2026/08/12 06:29:34
< [PNG]           Strip                           : <none>
< [PNG]           RenderTime                      : 00:00.11
< [PNG]           Memory                          : 2M
> [PNG]           Strip                           : SMZSTRIP2
> [PNG]           RenderTime                      : 00:00.00
> [PNG]           Memory                          : 0M
< [PNG]           CyclesViewLayersamples          : 1
< [PNG]           CyclesViewLayertotal_time       : 00:00.11
< [PNG]           CyclesViewLayerrender_time      : 00:00.10
< [PNG]           CyclesViewLayersynchronization_time: 00:00.01
```

The earlier date is the moment the *source* image was written, not the moment of this render.

> **The `STRIPS` half did not reproduce. Added by the citation check, 2026-08-11.** Run again on both machines, with two image strips (`SMZSTRIP1` on channel 1, `SMZSTRIP2` on channel 2) pointing at a PNG written by the same session, the file written with `metadata_input` set to `STRIPS` carried **no Blender metadata at all** — only the picture's own structure, and none of the source file's block. The `SCENE` half reproduced exactly, `Strip : SMZSTRIP2`, `RenderTime : 00:00.00` and `Memory : 0M` included. Both runs stand as written; neither settles the other. Back on the hand-test list as test 20.

**Run a third time, 2026-08-12 — the `STRIPS` file carried no Blender field at all.** Test 20 on the hand-test list, run in Blender 5.0.1 on **machine one, the Creator's Mac** (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card) and on **machine two, a rented Linux machine** (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory). What was done: the standard scene; frame 7 rendered to a PNG first, in the same Blender session, carrying its own full block; two image strips, `SMZSTRIP1` on channel 1 and `SMZSTRIP2` on channel 2, both pointing at that PNG; `render.use_sequencer` on; format PNG; frame 7 rendered twice more, once with `metadata_input = 'STRIPS'` and once with `'SCENE'`. Both strips printed `frame_final_start 7 frame_final_duration 1`. The source PNG's bytes and size were the same at every check before and after both renders, so it still carried its own fields at the moment of the render.

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

Not one Blender field, and no `Strip` line. The `SCENE` file from the same session carried the fourteen, including `Strip : SMZSTRIP2`, `RenderTime : 00:00.00`, `Memory : 2M`, `Hostname : Artists-Studio-MacBook.local`, and none of the four `cycles.` fields. The source PNG carried those fourteen with `Strip : <none>`, `RenderTime : 00:00.11` and the four Cycles fields as well — eighteen in all.

**Machine two agreed.** Its `STRIPS` file printed the same list, and its bytes hash to the same value as machine one's, because the file holds nothing that names either machine. Its `SCENE` file reads `Hostname : ubuntu` and `File : /root/ht2/smzfile.blend`.

Counting the Blender fields in each of the three files, on both machines: source 18, `STRIPS` 0, `SCENE` 14. **This run and the citation check's run came out the same; the first run came out differently.** All three stand as written and none settles the others. The full run record is in `Audits/Blender Research/Images/Hand tests — cross-sweep questions.md`.

### Part H's fourth gap: saving by hand, and the File Output node

`Image ▸ Save As` from an Image Editor wrote a file whose tag list is identical, name for name, to the file the render wrote itself. The compositor's File Output node also writes the fourteen names, from the same render, but with `Camera : <none>`, `Lens : <none>`, `RenderTime : 00:00.00`, and none of the four Cycles items. Building that node setup in 5.0 needs `scene.compositing_node_group` rather than `scene.node_tree`, which no longer exists on the scene, and the node's own `directory` and `file_name` rather than `base_path` and `file_slots`.

> **The File Output node half could not be repeated at all. Added by the citation check, 2026-08-11.** The API facts here are right: `scene.node_tree` is gone in 5.0, the node's properties are `directory` and `file_name`, and a node group made with `bpy.data.node_groups.new` and assigned to `scene.compositing_node_group` is the way in. But following that description, on both machines, **the node wrote no file at all** — with an item added to `file_output_items`, its format set to PNG after the media type was set to `IMAGE`, its input linked from the group input, and a group output linked so that the ordinary render still wrote its own file, the node's folder was empty after a still render and after an animation render alike. Whatever the original run did to make the node write is not in these words. The hand-saved half held, but by `Image.save_render`, because the `Image ▸ Save As` operator refused with `RuntimeError: Operator bpy.ops.image.save_as.poll() failed, context is incorrect`. Back on the hand-test list as test 22.

**Run again, 2026-08-12, with every step written down — the node still wrote nothing.** Test 22 on the hand-test list, run in Blender 5.0.1 on **machine one, the Creator's Mac** (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card) and on **machine two, a rented Linux machine** (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory). This is the recipe the test was asked to produce, in the order the steps were taken, with what the machine printed at each one:

```
STEP 1: ordinary output — media type then format
  ims.media_type = IMAGE
  ims.file_format = PNG
  render.filepath = …/out/22/5.0/panel/panel
STEP 2: the node group
  scene.node_tree present: False
  bpy.data.node_groups.new('SMZComp', 'CompositorNodeTree') -> SMZComp CompositorNodeTree
  scene.compositing_node_group = <bpy_struct, CompositorNodeTree("SMZComp") at 0xadc1b6408>
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
STEP 6: directory that does not exist: …/out/22/5.0/node_missing | file_name: smznode
LIST[before still render, node folder] …/node_missing -> NO SUCH FOLDER (FileNotFoundError)
STEP 6a: bpy.ops.render.render(write_still=True)
LIST[after still render, node folder] …/node_missing -> NO SUCH FOLDER (FileNotFoundError)
LIST[after still render, panel folder] …/panel -> ['panel.png']
STEP 7: directory made first: …/out/22/5.0/node_made
STEP 7a: bpy.ops.render.render(write_still=True)
LIST[after still render, node folder] …/node_made -> []
LIST[after still render, panel folder] …/panel2 -> ['panel.png']
STEP 8: bpy.ops.render.render(animation=True) over frames 7 to 7
LIST[after animation render, node folder] …/node_anim -> []
LIST[after animation render, panel folder] …/panel3 -> ['panel0007.png']
DONE-22
```

So: the interface sockets were added **before** the Group Input and Group Output nodes, and those nodes then showed the socket. The media type must be set before the file format, and asking for PNG first fails with the error above. The item is made with `file_output_items.new('RGBA', 'SMZNODE')`; the choices the machine printed for that first argument are `['FLOAT', 'INT', 'BOOLEAN', 'VECTOR', 'RGBA', 'ROTATION', 'MATRIX', 'STRING', 'MENU', 'SHADER', 'OBJECT', 'IMAGE', 'GEOMETRY', 'COLLECTION', 'TEXTURE', 'MATERIAL', 'BUNDLE', 'CLOSURE']`. The node then reported `fo.inputs[0].name: SMZNODE is_linked: True`, `fo.mute: False`, `save_as_render: True`, `active_item_index: 0`.

**The node's folder stayed empty every time**: not created when it did not exist, and left empty when it was made first, after a still render and after an animation render alike. The Output panel's own file was written every time from the same render. **The console said nothing about it** — no warning, no error, no line naming the node — and `bpy.ops.render.render` returned without raising. **The same on machine two, and the same in 5.1 and 5.2.**

**Four steps were added beyond the entry's own procedure, and they are named here.** *One*, six variations on the folder and file name were tried in 5.2 — a plain folder, a folder with a trailing separator, a file name ending `.png`, a file name ending `####`, `save_as_render` off, and a folder written as `//c6` — still and animation each. Every one left the folder empty, and a search of the whole working folder afterwards found nothing called `smznode`. *Two*, the same scene was rendered three times in 5.2 with no group, with a group whose Invert node sits between the group input and the group output, and with that group's links removed; the three pictures came out different from one another, so the group is executed during the render and the render result does reach its input, while the File Output node inside it still wrote nothing. *Three*, the whole build was run again on both machines from a Blender with a real window open rather than with `-b`; the node's folder was empty there too, after a still render and after an animation render. *Four*, in a windowed Blender `bpy.ops.node.new_compositing_node_group()` returned `{'FINISHED'}` and left `scene.compositing_node_group` as `None`.

**A command that errored, in order.** The first windowed attempt reused a blend-file the last background run had left behind, which 5.2 had written, and 5.0 reported `WARNING File written by newer Blender binary (502.44), expect loss of data!`. The repair was to rebuild the scene with each version's own Blender before its windowed run, and step three above is the repaired run.

**This run and the citation check's run came out the same; the first run came out differently.** All three stand as written and none settles the others.

### Part H's fifth gap: the preview copy

With the OpenEXR Preview option on, a JPEG copy sat beside the EXR **both** when the frame was rendered from a command line with no window and when it was rendered from a Blender with its window open — so the manual's wording does not draw the line it seems to. That JPEG copy carries the same fourteen items as the EXR beside it.

### The factory state of the switches

On a factory-fresh 5.0 scene, on both machines, `use_stamp_memory` reads `False`, while the property's own stated default is `True` (`id=use_stamp_memory  name='Stamp Peak Memory'  type=BOOLEAN  default=True`). Part C's disagreement is therefore not a contradiction: the screenshot describes a fresh Blender, the stated default describes the property. The whole factory list: on — Date, Time, Render Time, Frame, Camera, Scene, Filename, Include Labels; off — Frame Range, Memory, Hostname, Lens, Marker, Strip Name, Note, Burn into Image.

### Part B's caution about `use_stamp_note`, settled

Both quotes are right, about different things:

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

The same name carries a boolean on one type and a string on the other. Neither was written down wrongly.

### The "Use Strip Me…" tick-box in the picture

**No property in 5.0 has an interface name beginning "Use Strip Me".** The panel's own drawing code, read out of the running Blender, offers only two sequencer-related controls, both shown only when the Video Sequencer is in use:

```
        if rd.use_sequencer:
            layout.prop(rd, "metadata_input")
        …
        if rd.use_sequencer:
            col.prop(rd, "use_stamp_sequencer_strip", text="Strip Name")
```

### A field an engine adds for itself

A render engine registered for the test called `result.stamp_data_add_field("SMZFIELD", "SMZVALUE-98765")` on the render result and the call succeeded. Neither `SMZFIELD` nor `SMZVALUE-98765` appears anywhere in the PNG that render wrote — while the scene's own fourteen, in the same file, do. So part G's open question is answered: an engine-added field reaches the render result and not the written file.

### Colour, pixel density, Cryptomatte and video

No PNG written by 5.0 carried an ICC profile or any colour tag at all, at the default colour management or with the view transform set to `Standard`. In OpenEXR the colour marking is the `colorInteropID` attribute quoted above — `lin_rec709_scene` on colour parts, `data` on a Mist part.

With the pixel density set to 300: PNG carries `PixelsPerUnitX : 11811` and `PixelsPerUnitY : 11811`; BMP carries `PixelsPerMeterX : 11811`, `PixelsPerMeterY : 11811`; TIFF carries `XResolution : 300`, `YResolution : 300`; OpenEXR carries one number, `xDensity : 300`; **JPEG carries none** (`ResolutionUnit : None`, `XResolution : 1`); WebP carries none. With the pixel aspect set to 2 across by 1 down, PNG and BMP write the two directions separately (150 and 300), **TIFF writes both as 150**, and OpenEXR still writes one number.

A Cryptomatte render writes four header attributes per layer — for example `[cryptomatte/542cafa/name] = ViewLayer.CryptoObject`, `[cryptomatte/542cafa/manifest] = {"Cube":"a8fce865","Light":"c77d43e5","object":"055ec73d"}`.

An MPEG-4 with H.264 over frames 7 to 9, every switch on, carries **none of the fourteen under any name**. What it does carry: `ColorProfiles : nclx`, `ColorPrimaries : BT.709`, `TransferCharacteristics : BT.709`, `MatrixCoefficients : BT.709`, `VideoFullRangeFlag : Limited`, `ContentCreateDate : 2026/08/12 06:23:18`, `Encoder : Lavf61.7.100`. `Lavf` is the FFmpeg library, not Blender. A side-by-side Stereo 3D movie written by 5.0 is named `stereo_0007-0009_L.mp4` — with an `_L` on the end — and carries no stereo or projection marking; searching it for `st3d`, `sv3d`, `svhd`, `prji`, `proj`, `equi` and `ytmp` found none of them, and the same for an equirectangular panoramic movie.

### Cycles' own debug passes

5.0 has both `pass_debug_sample_count` and `pass_render_time`. On the processor the file carries a channel named `ViewLayer.Render Time.X`; on the graphics card it does not — Apple's own graphics through Metal on machine one, one NVIDIA RTX 4000 Ada through OptiX on machine two, and both agreed. `ViewLayer.Debug Sample Count.X` is written either way.

---

### Outside the interface

**Settled by hand test, 2026-08-11: it holds.** The same blend-file was rendered twice on each machine — once with `-b` and no window at all, once from a Blender with its window open — and the two files carry exactly the same tags, name for name, with only the date and the render time differing as they must. That was true in 5.0.1 on machine one (the Creator's Mac, macOS 26.5.2, Apple M1 Pro) and on machine two (the rented Linux machine, Ubuntu 22.04.5, two NVIDIA RTX 4000 Ada cards), and the two machines agreed. So the thin inference below turns out to be right. One documented difference nearby did **not** hold: the OpenEXR Preview option wrote a JPEG copy beside the EXR in *both* cases, not only from the command line. What follows is what the sources said before the tests, kept for the trail.

**One place where it holds by being absent both ways, checked 2026-08-12.** The compositor's File Output node was built and rendered in 5.0.1 on both machines, first with `-b` and no window and then from a Blender with its window open. Its folder was empty in every case, after a still render and after an animation render. So having a window makes no difference there either — the node wrote nothing with one and nothing without one.

Does this hold when Blender is driven from outside its own window?

**LABEL: inferred, and thinly.** Every switch described above is a property of the scene's render settings (`bpy.types.RenderSettings`) or of the scene itself (`bpy.types.Scene`), which are saved in the blend-file and are readable and writable from Python. **The step I took:** because they are ordinary scene properties rather than anything belonging to a window or an editor, a render started from Python or from a command line uses whatever values the blend-file holds, exactly as one started by clicking would. Nothing in the sources states this, and I have supplied the reasoning myself, so it is inference.

The manual's hint about the Note field points the same way without quite saying it — "It can be useful to use the Note field if you are setting up a render farm. Since you can script any information you like into it, such as an identifier for the render node or the job number" (https://docs.blender.org/manual/en/5.0/render/output/properties/metadata.html, read 2026-07-30). A render farm is by definition Blender driven from outside its own window, and the manual recommends this feature for exactly that, which would make no sense if the feature did not work there.

**What the sources do not say:** whether anything differs when Blender runs with no window at all. I searched the manual's entire command-line section for both *stamp* and *metadata* and found no mention of either, so there is no documented exception — but equally no documented confirmation. Worth noting that the manual does elsewhere flag command-line rendering as behaving differently in a nearby area ("When rendering animations (or single frames via command line), save a JPEG copy of the image, for a quick preview", https://docs.blender.org/manual/en/5.0/files/media/image_formats.html), so the documentation does draw that distinction when it applies — it just does not draw it here.

One item is worth singling out: **Hostname** records "the rendering machine's hostname", which is the item that would tell you which machine in a farm produced a given frame. It is off by default.

---

### Operating system

**Settled by hand test for macOS and Linux, 2026-08-11; Windows is still open.** Every one of the nineteen tests was run in 5.0.1 on both machines, and every result came out the same on macOS 26.5.2 with Apple's own graphics and on Ubuntu 22.04.5 with NVIDIA cards — the same item names, the same formats keeping and dropping them, the same factory switches, the same part-by-part shape of a multi-part OpenEXR. Two values differ, and only because they describe the machine: the `File` item holds that machine's own path, and `Hostname` holds that machine's own name — `Artists-Studio-MacBook.local` on machine one and `ubuntu` on machine two, each matching exactly what that machine's `hostname` command prints. The Memory item read `2M` on both. **Windows was not tried: no Windows machine has run it**, so nothing here speaks for it. What follows is what the sources said before the tests, kept for the trail. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Checked again on 2026-08-12 and still holding for macOS and Linux.** The second batch of hand tests ran three more tests in 5.0.1 on both machines — the Sequencer-strips test, the part-by-part shape of a multi-part OpenEXR, and the compositor's File Output node — and the two machines agreed about every one, down to the attribute counts in each part of the OpenEXR and the empty node folder. **Windows is still untested for the same reason.**

Does this hold on every operating system Blender runs on?

**LABEL: not in the sources.** Nothing on any page I read qualifies the Metadata feature, any of the fourteen items, the burned-in text, or the format table by operating system. No page names Windows, macOS or Linux in connection with any of this. I searched for *stamp* and *metadata* across the whole manual and API documentation and found no operating-system caveat attached to either.

Two items would depend on the machine rather than on Blender, and the sources describe them in machine terms without saying more: Hostname ("the rendering machine's hostname") and Memory ("the peak memory usage"). Whether these read the same way on every system is not stated, and I will not guess.

---

## Things a person with Blender open could settle in a few minutes

The nine items that stood here were all run on 2026-08-11, in Blender 5.0.1 on both of this project's machines, and their answers are in "What the hand tests showed" above. What is left open needs a machine this project does not have.

1. **Whether any of this differs on Windows.** Item 7 above asked for the comparison to be repeated on a second operating system. It was — macOS 26.5.2 and Ubuntu 22.04.5 — and the two agreed about everything except the two values that describe the machine itself. **A third comparison on Windows was not run: no Windows machine has run it.** Until it has one, nothing above speaks for Windows.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Images, Blender 5.0

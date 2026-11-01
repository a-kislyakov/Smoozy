# Images — cross-sweep questions, second pass (Blender 5.1)

**Sources consulted, and when.** Blender 5.1 manual (`Blender Sources/manual_5.1`, including its `_images` folder), Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`). All four are a snapshot taken on **2026-07-30**, and that is the date read for everything quoted below. Work done 2026-08-11.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Question answered here: **10 only.**

---

## 10. What does a finished picture hold, in every part of the file, about what produced it?

### A note on the reading I took

The question can be read two ways. I answer the first and note the second.

- **The reading I took:** for one finished picture, what does the file carry about the render that made it, counting every part of the file — the header, the pixels, and, where the format has them, the layers and channels.
- **The other reading:** "in every part of the file" could mean "in every kind of file", i.e. does every format carry the same thing. That reading is also settled below, because the manual's format table says which formats can hold this information at all, and most cannot.

The question was ambiguous; I have covered both rather than choose silently.

### Short answer

A finished picture can hold a block of **metadata** describing the render — date, time, render time, frame, frame range, peak memory, hostname, camera, lens, scene, marker, blend-file name, sequencer strip name, and a free-text note. It holds this only if the render was set up to include it, and only if the format supports metadata at all: of the fourteen formats in the manual's table, only **JPEG, OpenEXR, PNG and WebP** do. The same text can additionally be **burned into the pixels**. Separately, and independently of that metadata block, a file may carry **pixel density** (five formats), **colour-space information** (OpenEXR, plus an ICC profile where the colour configuration names one), and — in a multi-layer OpenEXR — **passes that describe the render itself**, namely Cycles' "Render Time" and "Sample Count" debug passes. Nothing in the sources says Blender writes its own name or version into a rendered image.

**LABEL: documented**, for every part of the list above, with two exceptions marked in place: the multi-layer-EXR passes part is **observed / inferred** (marked below), and the "nothing about Blender's own name or version" part is **not in the sources** (the record of that search is below).

---

### Part 1 — The metadata block, and what is in it

**LABEL: documented.**

The panel that controls it, and what it does:

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

The full list of what can be included, quoted exactly and in the manual's order:

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

And the manual's own suggestion for the free-text field:

> "It can be useful to use the Note field if you are setting up a render farm.
> Since you can script any information you like into it,
> such as an identifier for the render node or the job number.
> For details on stamping arbitrary values,
> see: this page."

(The words "this page" are a link in the manual. The copy I have is plain text, so the link target is lost. See Part 6.)

- Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html
- Date read: 2026-07-30

The same list appears in the Python API, where each switch says where the value goes. The wording there is worth having, because it says "image metadata" or "image/video metadata" explicitly, and gives the default state of each switch:

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

(I have joined each property name to its own description line; in the page they sit on separate lines with a "Type: bool" between entries.)

Also there:

> "metadata_input
>
> Where to take the metadata from (default 'SCENE')"

> "stamp_note_text
>
> Custom text to appear in the stamp note (default "", never None)"

- Source: API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
- Date read: 2026-07-30

**So, by default, a rendered file that supports metadata carries:** date and time, scene time and frame timecode, render time, frame number, peak memory, camera name, scene name, and blend-file name. Off by default: frame range, hostname, lens, marker, strip name, note. This is read straight off the "(default True)" / "(default False)" text quoted above, so it is documented, not inferred.

**The picture in the manual.** I opened `_images/render_output_properties_metadata_panel.png`, the screenshot on the Metadata page. It shows the panel with tick-boxes for Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, and one labelled "Use Strip Me.." (truncated by the panel width), then a "Note" section with a text field, then a "Burn Into Image" section containing Font Size (shown as "12 px"), Text Color, Background, and Include Labels. In the screenshot, ticked: Date, Time, Render Time, Frame, Camera, Scene, Filename. Unticked: Frame Range, Memory, Hostname, Lens, Marker, Strip Name, Use Strip Me.., Note, Burn Into Image. LABEL for this: **documented** (it is part of the manual). What I take from it is only what is visible; note that the picture shows the metadata source as a tick-box named something beginning "Use Strip Me…", whereas the page text describes it as a two-choice setting called "Metadata Input" (Scene / Sequencer Strips). I do not resolve that difference — I report both.

### Part 2 — Which formats can hold it

**LABEL: documented.**

> "Note
>
> Only some image formats support metadata:
> See image formats."

(manual, https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html)

The table on the formats page has a "Metadata" column and a "DPI" column. In the plain-text copy the table is flattened into a list, six entries per format in the order Format, Channel Depth, Alpha, Metadata, DPI, Extensions; I read it in blocks of six. Reading it that way:

- **Metadata ✓:** JPEG, OpenEXR, PNG, WebP.
- **Metadata ✗:** AVIF, BMP, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw, TIFF.
- **DPI ✓:** JPEG, OpenEXR, PNG, BMP, TIFF.
- **DPI ✗:** WebP, AVIF, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw.

The page states the consequence plainly:

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

- Source: manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html
- Date read: 2026-07-30

Note the sentence "some may only support a limited subset of fields". The sources nowhere say which fields drop out of which format. That is a real gap and it matters for anyone relying on a particular field surviving.

### Part 3 — The pixels

**LABEL: documented.**

The same information can be written into the picture itself rather than the header:

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

- Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html
- Date read: 2026-07-30

The API states it as a change to the rendered image:

> "use_stamp
>
> Render the stamp info text in the rendered image (default False)"

and the appearance controls:

> "stamp_background — Color to use behind stamp text (array of 4 items, in [0, 1], default (0.0, 0.0, 0.0, 0.25))"
> "stamp_font_size — Size of the font used when rendering stamp text (in [8, 64], default 12)"
> "stamp_foreground — Color to use for stamp text (array of 4 items, in [0, 1], default (0.8, 0.8, 0.8, 1.0))"
> "use_stamp_labels — Display stamp labels ("Camera" in front of camera name, etc.) (default True)"

- Source: API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
- Date read: 2026-07-30

Because burning-in is off by default and independent of the header block, a picture in a format with no metadata support (TIFF, DPX, Radiance HDR and the rest) can still be made to carry the information — but only as visible text over the image. This much is documented; whether the two lists of fields are always identical is not stated.

### Part 4 — Where in the file it sits, and reading it back

**LABEL: documented, for what is said; the byte-level placement is not in the sources.**

The sources describe the metadata as living in **header fields**, and name one such field, but never name the field keys Blender writes or the container mechanism per format (no mention anywhere of PNG text chunks, JPEG comment markers, EXR attributes, EXIF writing, XMP, or IPTC — see the search record).

The clearest statement is on the Video Sequencer's metadata panel, which reads metadata back out of files:

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

- Source: manual, https://docs.blender.org/manual/en/5.1/editors/video_sequencer/preview/sidebar.html
- Date read: 2026-07-30

One thing follows directly from those words: Blender can read back the metadata it wrote into its own files ("This also works for images produced by Blender"). **Inferred, and the step written out:** that what *Blender* writes sits in header fields an outside tool can see and change is a step past the quotes — the "header field "Comments"" sentence is said of *other graphics programs'* files, and the exiftool tip is about editing metadata in general. A reader has to join them. *(Relabelled by the citation check, 2026-08-11: this was written as a second thing following directly, which the quoted words do not carry.)*

For a render still held in memory, the same information can be shown over the image:

> "Show Metadata
>
> Displays metadata about the selected Render Result. See the Output tab's
> Metadata panel to change what metadata to include."

- Source: manual, https://docs.blender.org/manual/en/5.1/editors/image/navigating.html (the same two sentences also appear at https://docs.blender.org/manual/en/5.1/editors/image/overlays.html, and at https://docs.blender.org/manual/en/5.1/editors/uv/overlays.html with one word changed — that page reads "Display metadata about the selected Render Result", not "Displays". *Corrected by the citation check, 2026-08-11.*)
- Date read: 2026-07-30

One release note confirms that at least one of these fields is a genuine output field with a display format, and that it was wrong in the version before:

> "Metadata output for "Peak Memory" displays "ddM" in Blender 5.0 … - Backported to 5.0.1"

- Source: release notes, https://developer.blender.org/docs/release_notes/5.1/bugfixes/ (section "Render & Cycles: 52")
- Date read: 2026-07-30

### Part 5 — What else the file carries about its making, outside the metadata block

**LABEL: documented.**

**Pixel density.** The manual calls this metadata outright:

> "Pixel Density
>
> The pixel density (often called PPI or DPI) controls the intended physical size for an image.
> This is often used for printing or the physical size when loaded into desktop publishing software.
>
> The X/Y density is calculated using the render X/Y aspect,
> making these values useful for storing the aspect ratio of non-square pixels.
>
> The pixel-density is meta-data which doesn't impact the quality of the image."

- Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
- Date read: 2026-07-30

So the file can record the shape of the pixels the render used, in the five formats with DPI support listed in Part 2.

**Colour-space information, and an ICC profile.** New in this version:

> "OpenColorIO was updated to 2.5, and for configuration authors the following new attributes are supported. The bundled Blender configuration was also updated to make use of these.
>
> - `interop_id` for identifying the color space and embedding in OpenEXR files.
> - `icc_profile` to specify the ICC profile to be embedded when saving an image file."

- Source: release notes, https://developer.blender.org/docs/release_notes/5.1/rendering/ (section "OpenColorIO")
- Date read: 2026-07-30

The manual says the same from the configuration author's side, and adds that this depends on how the colour configuration is written:

> "Saving colorspace metadata for images and video also depends on this."

> "Include the interop ID from the Color Interop Forum
> for every color space and display color space that you can. This helps save image and video
> with correct colorspace information. Add the interop ID as an alias of the colorspace. For
> OpenColor 2.5 configs, additionally set the native interop_id attribute where possible."

> "The icc_profile_name interchange attribute is supported, to embed ICC profiles when saving images."

> "For HDR view transforms, use HDR 500 nits, HDR 1000 nits, HDR 2000 nits or HDR 4000 nits
> as part of the name to automatically determine maximum luminance for mastering display metadata."

- Source: manual, https://docs.blender.org/manual/en/5.1/render/color_management/opencolorio.html
- Date read: 2026-07-30

That an OpenEXR carries a readable colour-space marking is confirmed from the other direction, on the formats page:

> "When opening OpenEXR images, Blender will automatically set the
> Color Space if it is detected as Linear CIE-XYZ E or ACES2065-1."

- Source: manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html
- Date read: 2026-07-30

And one bug listed as fixed in this version shows the same thing was not always written correctly:

> "Single-layer EXR does not pick up scene working color space in some cases … - Backported to 5.0.1"

- Source: release notes, https://developer.blender.org/docs/release_notes/5.1/bugfixes/ (section "Render & Cycles: 52")
- Date read: 2026-07-30

**Passes that describe the render itself.** In a multi-layer OpenEXR, two Cycles passes record facts about the rendering rather than about the scene:

> "Render Time Cycles
>
> An estimate for how long in milliseconds each pixel took to render.
> Note, this pass is not supported on GPU rendering.
>
> Sample Count Cycles
>
> Number of samples calculated for each pixel, divided by the maximum number of samples.
> Used to analyze adaptive sampling."

- Source: manual, https://docs.blender.org/manual/en/5.1/render/layers/passes.html (section "Debug")
- Date read: 2026-07-30

**Observed, from Cycles' own files.** In `Blender Sources/cycles_5.1/engine.py`, inside `list_render_passes` (which begins at line 177), lines 219–225 read:

```
    # Debug passes.
    if crl.pass_debug_sample_count:
        yield ("Debug Sample Count", "X", 'VALUE')
    if crl.pass_render_time:
        # Only yield the pass if rendering on CPU
        if scene.cycles.device == 'CPU':
            yield ("Render Time", "X", "VALUE")
```

- File: `Blender Sources/cycles_5.1/engine.py`, lines 219–225
- Date read: 2026-07-30

What is **observed** is only the text of those lines: the pass is named "Debug Sample Count" in one case and "Render Time" in the other, each is a single channel named "X" of value kind, and the Render Time pass is only produced when the device is CPU. That these names then appear as channel names inside a saved multi-layer OpenEXR is a step beyond the file, so: **inferred** — the step is that `list_render_passes` is what tells Blender which passes a Cycles render layer has, and passes are what a multi-layer OpenEXR stores. I did not verify that step in any source.

The same file also shows, at lines 227–230, that Cycles chooses its Cryptomatte channel names for the sake of other software reading the file:

```
    # Cryptomatte passes.
    # NOTE: Name channels are lowercase RGBA so that compression rules check in OpenEXR DWA code
    # uses lossless compression. Reportedly this naming is the only one which works good from the
    # interoperability point of view. Using XYZW naming is not portable.
```

- File: `Blender Sources/cycles_5.1/engine.py`, lines 227–230
- Date read: 2026-07-30 — **observed** (the comment text), and I draw nothing further from it.

### Part 6 — Adding your own fields

**LABEL: documented.**

The API offers one call for putting values of your own into the stamp:

> "stamp_data_add_field(field, value)
>
> Add engine-specific stamp data to the result
>
> Parameters:
>
> field (str) – Field, Name of the stamp field to add (never None)
>
> value (str) – Value, Value of the stamp data (never None)"

- Source: API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderResult.html
- Date read: 2026-07-30

This is on `RenderResult`, described on the same page as "Result of rendering, including all layers and passes". The manual's Note hint (Part 1) points at "this page" for "stamping arbitrary values"; the link target is lost in my plain-text copy, so I do not claim these are the same thing. If they are, the claim would be **inferred**; I leave it unmade.

### Part 7 — What the sources do not say

**LABEL: not in the sources.** Each of these was searched for; the record is below.

1. **Blender's own name or version.** Nothing anywhere says a rendered image records which program or which version of it produced the file. No "Software" tag, no "Creator", no "Producer", no version stamp. There is no such field in the metadata list, and no statement that anything is written unconditionally.
2. **The field names and containers used inside each format.** The sources never name the keys Blender writes, nor the mechanism per format. EXIF is mentioned three times in the whole manual and never as something Blender writes: once about camera focal length for motion tracking, once as `exiftool` in the tip quoted above, and once to say EXIF orientation is *ignored on loading* ("EXIF orientation metadata (commonly found in JPEG images) is ignored when loading images into Blender.", https://docs.blender.org/manual/en/5.1/files/media/image_formats.html). XMP appears only in the asset-library page; IPTC appears nowhere.
3. **Which fields survive in which format.** The manual says "some may only support a limited subset of fields" and stops there.
4. **Whether a hand-saved image carries the block.** Everything quoted is worded as *render output*. Whether an image saved from the Image editor, or written by the compositor's File Output node, carries the same metadata is never stated; the File Output page does not mention metadata or stamping at all.
5. **Whether Cycles adds any engine-specific stamp fields of its own.** The API provides the call (Part 6), but Cycles' own files contain no use of it — the word "stamp" does not appear anywhere in `Blender Sources/cycles_5.1`.

### Not covered

**Every item in this list was settled by hand test on 2026-08-11 — see "What the hand tests showed" below.** What follows is the list as the researcher left it.

- I cannot say what a produced file actually contains, byte for byte — only what the documentation says goes in and the Cycles lines I read. Nothing here was verified by rendering anything.
- I cannot say which fields a given format drops, nor in what text form each field is written (for example whether "Render Time" is seconds, or the "HH:MM:SS.FF" form used for Time).
- I cannot say whether metadata is written when a picture is saved by hand or by the File Output node, as opposed to by a render.
- I have not settled whether the "Metadata Input" setting and the "Use Strip Me.." tick-box in the screenshot are the same control.
- Video output is outside my reading of the question, though several API descriptions say "image/video metadata"; I have not chased what video files carry.
- I have not chased whether a Cryptomatte manifest is written into the file's header. The only related sentence I found is on the Cryptomatte page — "Cryptomatte sidecars (metadata files) are not supported." (https://docs.blender.org/manual/en/5.1/compositing/types/mask/cryptomatte.html) — and that is about what Blender will read, not what it writes.

---

## What the hand tests showed — question 10

**LABEL for everything in this section: observed.** Added 2026-08-11 by the hand-test step, which ran a list of nineteen tests in Blender 5.1.2 on both of this project's machines. The full list and the full run record are in `Audits/Blender Research/Images/Hand tests — cross-sweep questions.md`.

**The two machines.** *Machine one is the Creator's Mac* — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card; `hostname` prints `Artists-Studio-MacBook.local`; exiftool 13.50. *Machine two is a rented Linux machine* — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory; `hostname` prints `ubuntu`; exiftool 12.40. Every Blender was started with `--factory-startup` and with its own settings folder.

**The scene used.** Blender's factory startup scene, with the scene renamed `SMZSCENE`, the camera renamed `SMZCAM` and its lens set to `37.5`, a marker named `SMZMARK`, frames 7 to 9 with frame 7 current, 64 × 64 pixels, Cycles at one sample, every `use_stamp_*` switch on, the note text `SMZNOTE-12345`, Burn Into Image off, saved as `smzfile.blend`.

**Where the two machines agreed and where they did not.** Every result below came out the same on both, except that the `File` field holds that machine's own path and `Hostname` holds that machine's own name.

### Part 4's gap: the field keys, and the text form of each value

Written to PNG on machine one:

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

On machine two the same, with `[PNG] File : /root/ht/smzfile.blend` and `[PNG] Hostname : ubuntu`. So Render Time is written as minutes and seconds, not the `HH:MM:SS.FF` shape used for Time, and Memory is a whole number with `M` after it. In OpenEXR the same fourteen names are separate header attributes; in JPEG they are separate comment markers, each reading `Blender:<name>:<value>`.

### Part 2's gap: which formats keep them, and which fields drop out

The version offers `['AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']`, plus a setting this answer does not mention: `MEDIA-TYPE: IMAGE ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`. Multi-layer OpenEXR can only be asked for after the media type is set to `MULTI_LAYER_IMAGE`.

- **Kept the fields: PNG, JPEG, OpenEXR (single-layer and multi-layer)** — and **all fourteen** in each, so "some may only support a limited subset of fields" does not bite for any of the three.
- **Kept nothing: AVIF, BMP, Iris, JPEG 2000, Targa, Targa Raw, Cineon, DPX, Radiance HDR, TIFF — and WebP.**

**WebP kept nothing**, on both machines, although Part 2's reading of the table puts it in the ✓ column. Nothing appeared in Blender's console about a format that could not carry them.

### Part 7's first gap: Blender does name itself, in OpenEXR

Every OpenEXR written by 5.1.2 carries, **with every `use_stamp*` switch set to `False`**:

```
  [Software] (string, 13 bytes) = Blender 5.1.2
  [colorInteropID] (string, 16 bytes) = lin_rec709_scene
```

A **multi-layer** OpenEXR always additionally carries `[BlenderMultiChannel] = Blender V2.55.1 and newer`. A PNG, a JPEG and a WebP carry no such tag.

### Part 7's fifth gap: Cycles does add stamp fields

Every file a Cycles render writes carries `cycles.ViewLayer.samples`, `cycles.ViewLayer.total_time`, `cycles.ViewLayer.render_time` and `cycles.ViewLayer.synchronization_time`, and they survive with every stamp switch off. They are absent from an otherwise identical EEVEE render. The word "stamp" is absent from the supplied Cycles files because the part that writes them is not in that folder.

### Part 5's inference confirmed, and the per-part question settled

The Cycles debug passes do come out as channels in a multi-layer OpenEXR, under those names. On the processor: `ViewLayer.Debug Sample Count.X` and `ViewLayer.Render Time.X`. On the graphics card the Render Time channel is **absent** — Apple's own graphics through Metal on machine one, one NVIDIA RTX 4000 Ada through OptiX on machine two — so the CPU-only comment read out of `engine.py` holds.

`use_exr_interleave` exists and is off by default. With it off, a multi-layer OpenEXR of two view layers with a Mist pass each is `multiPart=True` with `TOTAL PARTS: 6`, and **every one of the fourteen fields sits in part 0 and nowhere else**. Parts 1 to 5 hold thirteen structural attributes each:

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

> **This part-1 block did not reproduce. Added by the citation check, 2026-08-11.** Run again with the same scene, Blender 5.1.2 on machine one and 5.1 on machine two both wrote **twelve** attributes in part 1, with **no `xDensity`** — the same list as above with that one line absent, ending `(12 attributes in part 1)`. Everything the two runs share matched exactly, and the six parts with all fourteen fields in part 0 reproduced without change. The thirteen-attribute block above is what **5.2** writes; the re-run got exactly it from 5.2 on both machines. Both runs stand as written; neither settles the other. Back on the hand-test list as test 21.

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

5.0.1 gave exactly that. 5.2.0 gave those twelve **plus** `[xDensity] (float, 4 bytes) = 72.0`, ending `(13 attributes in part 1)`. The counts across all six parts: 5.0 and 5.1 both `part 0: 37, parts 1 to 5: 12 each`; 5.2 `part 0: 37, parts 1 to 5: 13 each`. Counting the raw bytes `xDensity` in each written file gave `5.0: 1`, `5.1: 1`, `5.2: 6` — so 5.1 writes it in part 0 only. `render.ppm_factor` read `72.0` in all three versions.

**Machine two agreed** — the same counts, the same part-1 block and the same raw byte counts. Reading the same three files with the header reader the earlier round used gave the same twelve for 5.1 and the same thirteen for 5.2, so the difference is not the reader's. Six parts, `multiPart=True`, and all fourteen fields in part 0 and nowhere else reproduced in all three versions on both machines.

**This run and the citation check's run came out the same; the first run came out differently.** All three stand as written and none settles the others.

With Interleave on, the same render writes `TOTAL PARTS: 1`.

### Part 3 confirmed: burnt-in text reaches a format that keeps nothing

Rendered at 640 × 360 with the burn-in on, to PNG and to Targa. Both pictures carry the same text: a block at the top left reading `File …`, `Date 2026/08/12 06:23:30`, `RenderTime 00:00.36`, `Peak Memory 18M`, `Hostname Artists-Studio-MacBook.local`, then `SMZNOTE-12345` on its own line with no label; a strip along the bottom reading `Marker SMZMARK Timecode 00:00:00:07 Frame 7 Frame Range 7:9 Camera SMZCAM Lens 37.50 Scene SMZSCENE`; and a `Strip <none>` item at the top right which at this width overlaps the end of the `File` line. The burn-in also **adds to** rather than replaces — the PNG from that run still carried the whole block of named fields. The burnt-in labels are not the field names: the file's `Memory` is drawn as `Peak Memory`, and `Time` as `Timecode`.

### Part 7's fourth gap: saving by hand, and the File Output node

`Image ▸ Save As` from an Image Editor wrote a file whose tag list is identical, name for name, to the file the render wrote itself. The compositor's File Output node writes the fourteen names too, from the same render, but with `Camera : <none>`, `Lens : <none>`, `RenderTime : 00:00.00`, and none of the four Cycles fields. In 5.1 that node lives in `scene.compositing_node_group`; `scene.node_tree` no longer exists.

> **The File Output node half could not be repeated at all. Added by the citation check, 2026-08-11.** The API facts here are right, but following this description on both machines **the node wrote no file at all** — with an item added to `file_output_items`, its format set to PNG after the media type was set to `IMAGE`, its input linked from the group input and a group output linked so the ordinary render still wrote, the node's folder was empty after a still render and after an animation render alike. What the original run did to make it write is not in these words. The hand-saved half held, but by `Image.save_render`, because the operator refused with `RuntimeError: Operator bpy.ops.image.save_as.poll() failed, context is incorrect`. Back on the hand-test list as test 22.

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

**The node's folder stayed empty every time**: not created when it did not exist, and left empty when it was made first, after a still render and after an animation render alike. The Output panel's own file was written every time from the same render. **The console said nothing about it** — no warning, no error, no line naming the node. **The same on machine two, and the same in 5.0 and 5.2.**

**Four steps were added beyond the entry's own procedure, and they are named here.** *One*, six variations on the folder and file name were tried in 5.2 — a plain folder, a folder with a trailing separator, a file name ending `.png`, a file name ending `####`, `save_as_render` off, and a folder written as `//c6` — still and animation each. Every one left the folder empty, and a search of the whole working folder afterwards found nothing called `smznode`. *Two*, the same scene was rendered three times in 5.2 with no group, with a group whose Invert node sits between the group input and the group output, and with that group's links removed; the three pictures came out different from one another, so the group is executed during the render and the render result does reach its input, while the File Output node inside it still wrote nothing. *Three*, the whole build was run again on both machines from a Blender with a real window open rather than with `-b`; the node's folder was empty there too. *Four*, in a windowed Blender `bpy.ops.node.new_compositing_node_group()` returned `{'FINISHED'}` and left `scene.compositing_node_group` as `None`.

**A command that errored, in order.** The first windowed attempt reused a blend-file the last background run had left behind, which 5.2 had written, and 5.1 reported `WARNING File written by newer Blender binary (502.44), expect loss of data!`. The repair was to rebuild the scene with each version's own Blender before its windowed run, and step three above is the repaired run.

**This run and the citation check's run came out the same; the first run came out differently.** All three stand as written and none settles the others.

### Part 5's colour items, as they come out

The `interop_id` release note shows up as the `colorInteropID` attribute quoted above — `lin_rec709_scene` on colour parts, `data` on a Mist part. **No PNG written by 5.1 carried an ICC profile**, at the default colour management or with the view transform set to `Standard`. A 5.1 PNG does carry the format's own colour chunks, which a 5.0 PNG does not: `SRGBRendering : Absolute Colorimetric`, `Gamma : 2.2`, `WhitePointX : 0.3127`, `WhitePointY : 0.329`, `RedX : 0.64`, `RedY : 0.33`, `GreenX : 0.3`, `GreenY : 0.6`, `BlueX : 0.15`, `BlueY : 0.06`.

### The Cryptomatte manifest is written into the header

```
  [cryptomatte/542cafa/conversion] (string, 17 bytes) = uint32_to_float32
  [cryptomatte/542cafa/hash] (string, 14 bytes) = MurmurHash3_32
  [cryptomatte/542cafa/manifest] (string, 58 bytes) = {"Cube":"a8fce865","Light":"c77d43e5","object":"055ec73d"}
  [cryptomatte/542cafa/name] (string, 22 bytes) = ViewLayer.CryptoObject
```

So the sidecar limitation quoted under "Not covered" is about a separate file; the manifest itself goes into the image.

### The unresolved panel control, and the two note properties

**No property in 5.1 has an interface name beginning "Use Strip Me".** The panel's own drawing code, read out of the running Blender, offers only `metadata_input` and `use_stamp_sequencer_strip` (drawn as "Strip Name"), and both only when the Video Sequencer is in use. So the screenshot's tick-box and the page's "Metadata Input" are not two names for one control in 5.1 — the control the screenshot shows is not drawn by this version at all.

`RenderSettings.use_stamp_note` is a boolean, `RenderSettings.stamp_note_text` is the text, and `Scene.use_stamp_note` is a *string* described as "User defined note for the render stamping".

### The Memory disagreement, and a field an engine adds

On a factory-fresh 5.1 scene, on both machines, `use_stamp_memory` reads `False`, while the property's own stated default is `True`.

A render engine registered for the test called `result.stamp_data_add_field("SMZFIELD", "SMZVALUE-98765")` and the call succeeded; neither string appears anywhere in the PNG that render wrote.

### Video, pixel density and the Sequencer's strips

An MPEG-4 with H.264 over frames 7 to 9, every switch on, carries **none of the fourteen under any name** — only `ColorProfiles : nclx`, `ColorPrimaries : BT.709`, `TransferCharacteristics : BT.709`, `MatrixCoefficients : BT.709`, `VideoFullRangeFlag : Limited`, `ContentCreateDate : 2026/08/12 06:23:38` and `Encoder : Lavf61.7.100`. A side-by-side Stereo 3D movie written by 5.1 is named `stereo_0007-0009.mp4`, with no `_L` on the end — 4.5 and 5.0 name the same render `stereo_0007-0009_L.mp4`.

With the pixel density set to 300: PNG carries `PixelsPerUnitX : 11811` and `PixelsPerUnitY : 11811`; BMP carries `PixelsPerMeterX : 11811`, `PixelsPerMeterY : 11811`; TIFF carries `XResolution : 300`, `YResolution : 300`; OpenEXR carries one number, `xDensity : 300`; **JPEG carries none**; WebP carries none. With a 2:1 pixel aspect, PNG and BMP write the two directions separately, **TIFF writes both as the X value**, and OpenEXR still writes one number.

With `metadata_input` set to `STRIPS` and two image strips in the Video Sequencer, the written file's block is **replaced by the block read out of the foreground strip's own image file** — its date, its render time, its memory figure and its Cycles fields — and `Strip` reads `<none>`; with `SCENE` the date is the render's own and `Strip` reads `SMZSTRIP2`.

> **The `STRIPS` half did not reproduce. Added by the citation check, 2026-08-11.** Run again on both machines, with two image strips (`SMZSTRIP1` on channel 1, `SMZSTRIP2` on channel 2) pointing at a PNG written by the same session, the file written with `metadata_input` set to `STRIPS` carried **no Blender metadata at all** — only the picture's own structure. The `SCENE` half reproduced exactly. Both runs stand as written; neither settles the other. Back on the hand-test list as test 20.

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

Not one Blender field, and no `Strip` line. The nine colour lines are there because 5.1 writes them into every PNG; they are not part of the metadata block. The `SCENE` file from the same session carried the fourteen, including `Strip : SMZSTRIP2`, `RenderTime : 00:00.00`, `Memory : 2M`, `Hostname : Artists-Studio-MacBook.local`, and none of the four `cycles.` fields. The source PNG carried those fourteen plus the four Cycles fields — eighteen in all.

**Machine two agreed.** Its `STRIPS` file printed the same list, and its bytes hash to the same value as machine one's, because the file holds nothing that names either machine. Its `SCENE` file reads `Hostname : ubuntu` and `File : /root/ht2/smzfile.blend`.

Counting the Blender fields in each of the three files, on both machines: source 18, `STRIPS` 0, `SCENE` 14. **This run and the citation check's run came out the same; the first run came out differently.** All three stand as written and none settles the others. The full run record is in `Audits/Blender Research/Images/Hand tests — cross-sweep questions.md`.

---

### Outside the interface

**Settled by hand test, 2026-08-11: it holds.** The same blend-file was rendered twice on each machine — once with `-b` and no window at all, once from a Blender with its window open — and the two files carry exactly the same tags, name for name, with only the date and the render time differing as they must. That was true in 5.1.2 on machine one (the Creator's Mac, macOS 26.5.2, Apple M1 Pro) and on machine two (the rented Linux machine, Ubuntu 22.04.5, two NVIDIA RTX 4000 Ada cards), and the two machines agreed. One documented difference nearby did **not** hold: the OpenEXR Preview option wrote a JPEG copy beside the EXR in *both* cases, not only from the command line. What follows is what the sources said before the tests, kept for the trail.

**One place where it holds by being absent both ways, checked 2026-08-12.** The compositor's File Output node was built and rendered in 5.1.2 on both machines, first with `-b` and no window and then from a Blender with its window open. Its folder was empty in every case, after a still render and after an animation render. So having a window makes no difference there either — the node wrote nothing with one and nothing without one.

**LABEL: inferred, and thin. The sources do not address this directly.**

The sources never say what happens to metadata when Blender is started and driven by something other than a person clicking. What can be said, and the step taken to say it:

- Every switch quoted in Part 1 is a property of `RenderSettings`, reached through the scene, and is therefore saved in the blend-file and settable from Python. So a render started from outside uses whatever those switches were set to when the file was saved, unless a script changes them. The step: I am assuming that a render started from outside runs the same render settings as one started inside, which the sources do not state.
- I searched the command-line documentation for any switch relating to stamping or metadata and found none. The pages searched: https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, .../render.html, .../index.html, .../extension_arguments.html and the `launch` folder beneath them. Neither "stamp" nor "metadata" appears in any of them. So there is no way documented to turn this on or off at launch; it has to come from the file or from a script.
- The manual does distinguish command-line behaviour elsewhere on the very subject of written files, which shows the documentation would say so if there were a difference. On OpenEXR's Preview option: "When rendering animations (or single frames via command line), save a JPEG copy of the image, for a quick preview." (https://docs.blender.org/manual/en/5.1/files/media/image_formats.html). No comparable sentence exists for metadata. That is an absence, not a statement.
- One field is by its nature machine-dependent rather than interface-dependent: hostname is described as "the hostname of the machine that rendered the frame", so on a farm each file records the machine that made it. This is documented; the farm reading of it is the manual's own ("It can be useful to use the Note field if you are setting up a render farm").
- The parts of the answer that are plainly independent of any window: the format table (Part 2), the ICC and colour-space embedding (Part 5), and the Cycles pass lines (Part 5), which are code.

Date read for all of the above: 2026-07-30.

### Operating system

**Settled by hand test for macOS and Linux, 2026-08-11; Windows is still open.** Every one of the nineteen tests was run in 5.1.2 on both machines, and every result came out the same on macOS 26.5.2 with Apple's own graphics and on Ubuntu 22.04.5 with NVIDIA cards — including the burnt-in text, whose font looked the same on both. The two machine-dependent items are answered: `Hostname` holds that machine's own name (`Artists-Studio-MacBook.local` and `ubuntu`, each matching what that machine's `hostname` command prints), and the blend-file field holds the whole path, which differs because the file sat in a different place on each. **Windows was not tried: no Windows machine has run it.** What follows is what the sources said before the tests, kept for the trail. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Checked again on 2026-08-12 and still holding for macOS and Linux.** The second batch of hand tests ran three more tests in 5.1.2 on both machines — the Sequencer-strips test, the part-by-part shape of a multi-part OpenEXR, and the compositor's File Output node — and the two machines agreed about every one, down to the attribute counts in each part of the OpenEXR and the empty node folder. **Windows is still untested for the same reason.**

**LABEL: not in the sources.**

No page I read attaches any operating system condition to metadata, to burning-in, to pixel density, or to colour-space embedding. I searched for platform words near the metadata pages and found none. The nearest thing to a platform-specific statement about writing images at all in the 5.1 notes is a fixed bug, and it concerns WebP rather than metadata:

> "ARM64 builds have issues saving WEBP images from 32bpp buffers"

- Source: release notes, https://developer.blender.org/docs/release_notes/5.1/bugfixes/ (section "Pipeline & IO: 15")
- Date read: 2026-07-30

Two things in the metadata block are machine-dependent by definition rather than operating-system-dependent: hostname, and the blend-file name (a path, which is written differently on different systems). The sources say nothing about either point, so I make no claim. One further untested edge: burning text into the image needs a font, and the sources say nothing about which font is used or whether it differs by system.

---

## Where I looked (the record)

All searches were run over `Blender Sources/manual_5.1`, `Blender Sources/api_5.1`, `Blender Sources/release_notes_5.1` and `Blender Sources/cycles_5.1` unless stated. Searches were case-insensitive whole-tree greps.

**Terms searched that found something:**

| Term | Where it hit | What I did with it |
| --- | --- | --- |
| `metadata` | manual: 34 files, of which the relevant ones were `render/output/properties/metadata`, `files/media/image_formats`, `render/output/properties/output`, `render/output/index`, `render/output/properties/index`, `files/media/index`, `editors/image/sidebar`, `editors/image/navigating`, `editors/image/overlays`, `editors/uv/overlays`, `editors/video_sequencer/preview/sidebar`, `editors/video_sequencer/preview/display/overlays`, `compositing/types/mask/cryptomatte`, `render/color_management/opencolorio`, `glossary/index`. API: `bpy.types.RenderSettings`, `bpy.types.RenderResult`, `bpy.types.MovieClip`, `bpy.types.MovieStrip`, `bpy.types.SpaceImageOverlay`, `bpy.types.SpaceUVEditor`, `bpy.types.SpaceClipEditor`, `bpy.types.SequencerPreviewOverlay`, plus asset-related pages. Release notes: `bugfixes.md` twice. Cycles: `osl.py` only. | Read all of them. Ruled out as unrelated: every asset-system hit (asset metadata is catalogue/author/tag information about a data-block, not about a rendered file), `bpy.types.VolumeGrid`/`VolumeGrids` (grid metadata), `bpy.types.UIList`, `bpy.types.ID`, `bpy.types.IDPropertyWrapPtr`, `bpy.ops.brush`, the theme pages, `files/import_export/usd`, `sculpt_paint/brush/brush_management`, `modeling/geometry_nodes/volume/read/grid_info`, `video_editing/setup/directory_structure`, `advanced/scripting/addon_tutorial`, and Cycles' `osl.py` (that is OSL shader-parameter metadata driving the node interface). |
| `stamp` | manual: `render/output/properties/metadata`, plus `sculpt_paint/brush/texture`, `editors/timeline`, `editors/graph_editor/introduction`, `editors/clip/sidebar`, `editors/video_sequencer/sequencer/sidebar/proxy`, `render/freestyle/python`, `troubleshooting/recover`, `files/import_export/fbx`, `addons/import_export/scene_fbx`. API: `bpy.types.RenderSettings`, `bpy.types.RenderResult`, `bpy.types.Scene`, `bpy.app`, `bpy.ops.wm`, `bpy.types.StripProxy`, `bpy.types.MovieClipProxy`. Cycles: **no hits at all**. | Read the two render pages fully. Ruled out the rest: they are "timestamp" in other senses (build commit timestamp, FBX animation timestamps, proxy time stamps, Freestyle ViewEdge time stamps) or "rubber stamp" brush texture. |
| `Burn into` | `render/output/properties/metadata`, `render/output/properties/index` | Quoted in Part 3. |
| `EXIF` / `exif` | `files/media/image_formats`, `editors/video_sequencer/preview/sidebar`, `movie_clip/tracking/introduction` | All three read. None describes Blender *writing* EXIF. Reported in Part 7. |
| `interop_id` / `icc_profile` | release notes `rendering.md`; manual `render/color_management/opencolorio` | Quoted in Part 5. |
| `embed` | manual: `render/color_management/opencolorio`, `render/shader_nodes/textures/ies`, `render/shader_nodes/color/mix`, `editors/image/image_settings` | Only the first is about writing into an image file. The others are IES profiles embedded in a blend-file, a watermark example, and packing a resource into the blend-file. |
| `Pixel Density` | `render/output/properties/output`, `render/output/properties/index`, `glossary/index` | Quoted in Part 5. |
| `render farm` | `render/output/properties/metadata`, `render/output/properties/output`, `render/cycles/render_settings/hair` | First quoted in Part 1; second is the placeholder-file hint; third unrelated. |
| `Render Time` (as a pass) | `render/layers/passes`; Cycles `engine.py` | Quoted in Part 5. |
| `sidecar` | `compositing/types/mask/cryptomatte`, `files/asset_libraries/introduction` | Noted in Not covered; the asset one is unrelated. |
| `mastering display` | `render/color_management/opencolorio` | Quoted in Part 5. |
| `ICC` | manual: `render/color_management/opencolorio` only | Quoted in Part 5. API hits were false matches inside unrelated identifiers on constraint pages. |

**Terms searched that found nothing at all** (this is what makes the "not in the sources" answers in Part 7 real): `XMP` (manual: only the asset-library page, in an unrelated sense; API: nothing), `IPTC` (nothing anywhere), `tEXt` (no hits as a term — the matches were the ordinary word "text" inside other words), `provenance` (nothing), `watermark` (only the Mix node example pages), `chromaticit` (only colour-space descriptions and one Grease Pencil note; nothing about writing chromaticities into a file), `Creator` (nothing in the manual), `Producer` (nothing in the manual), `Software` in the manual (only `render/cycles/gpu_rendering`, `getting_started/configuration/hardware`, `getting_started/about/index`, `getting_started/about/license` — all about software in general, none about a tag in an image), `HDR metadata` (nothing), `color profile` (only a fluid-display page).

**Cycles' own files, searched specifically:** `stamp`, `metadata`, `exif`, `exr`, `openexr`, `header`, `attribute`, `write_image`, `image_write`. `stamp` and `exif` have no hits anywhere in `cycles_5.1`. `metadata` hits only `osl.py`. `exr`/`openexr` hit only `engine.py` (the Cryptomatte channel-naming comment quoted above) and `operators.py` (the Denoise Animation and Merge Images operators, which take OpenEXR multi-layer files as input and output; neither says anything about metadata). The folder contains only the add-on's Python files plus kernel and util headers, `.oso` shaders and a few `.cu`/`.cpp`/`.metal` files — there is no image-writing code in it, so it cannot settle where in a file anything is stored.

**Release notes:** read `pipeline_io.md` and `rendering.md` in full, `index.md`, and searched the whole 5.1 notes folder for `metadata`, `stamp`, `exif` (two hits, both in `bugfixes.md`, both quoted or noted above) and for `openexr`/`image format`/`output`.

**Pictures opened:** `manual_5.1/_images/render_output_properties_metadata_panel.png` (the Metadata panel screenshot, from https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html). I checked the `_images` folder for any other file whose name contains "metadata" or "stamp"; that is the only one.

**Pages I opened and ruled out as not bearing on this question:** `render/output/properties/output` beyond the Pixel Density and Color Management sections, `files/media/video_formats` (containers and streams, not render metadata), `editors/image/image_settings`, `compositing/types/output/file_output` (searched for `metadata` and `stamp`: no hits), `bpy.types.ImageFormatSettings` (I listed every property on it — `cineon_black`, `cineon_gamma`, `cineon_white`, `color_depth`, `color_management`, `color_mode`, `compression`, `display_settings`, `exr_codec`, `file_format`, `has_linear_colorspace`, `jpeg2k_codec`, `linear_colorspace_settings`, `media_type`, `quality`, `stereo_3d_format`, `tiff_codec`, `use_cineon_log`, `use_exr_interleave`, `use_jpeg2k_cinema_48`, `use_jpeg2k_cinema_preset`, `use_jpeg2k_ycc`, `use_preview`, `view_settings`, `views_format` — none of them is a metadata switch), and `bpy.types.RenderEngine` (searched for `stamp`: no hits).

---

## Things a person with Blender open could settle in a few minutes

The ten items that stood here were all run on 2026-08-11, in Blender 5.1.2 on both of this project's machines, and their answers are in "What the hand tests showed" above. What is left open needs a machine this project does not have.

1. **Whether any of this differs on Windows.** Item 10 asked whether the burnt-in text looks the same on a second operating system; it was compared on macOS 26.5.2 and Ubuntu 22.04.5 and looked the same on both. **A third comparison on Windows was not run: no Windows machine has run it.** Until it has one, nothing above speaks for Windows.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Images, Blender 5.1

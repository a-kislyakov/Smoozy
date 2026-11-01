Sources consulted: Blender 5.3 Manual (snapshot 2026-10-11), Blender 5.3 Python API Documentation (snapshot 2026-10-11), Blender 5.3 Release Notes (snapshot 2026-10-11), and Cycles 5.3 source files (snapshot 2026-10-16).
I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

## 6. Besides a scene's pixel dimensions and its resolution percentage, can anything else inside a file decide the size of what gets written?

**Ambiguity note:**
"Size of what gets written" has two distinct meanings:
1. **Pixel dimensions (image resolution in width and height).** This is the reading assumed below, directly following the premise "Besides a scene's pixel dimensions and its resolution percentage".
2. **File size on disk (bytes).** If this reading was intended instead, settings determining file format, color depth, channel count, compression, and video bitrate/splitting also decide the size of what gets written. Both readings are documented below.

---

### Reading 1: Pixel Dimensions of What Gets Written

**LABEL:** documented (for Crop to Render Region, Group Output operation domain, and GPU 8K limits); inferred (for Stereo 3D side-by-side/top-bottom dimensions); not in the sources (for File Output node's domain handling).

**NOT COVERED:**
The documentation does not state whether the File Output node (`CompositorNodeOutputFile`) clamps intermediate node outputs to the scene render resolution or writes them at their input domain dimensions.

#### 1. Crop to Render Region
Inside a blend-file, enabling a render region together with the "Crop to Render Region" setting alters the pixel dimensions of the written image file so that it matches the cropped region boundaries rather than the full scene pixel dimensions.

- **Exact words:**
  > "Crop to Render Region
  > Crops the rendered image to the size of the render region, instead of rendering a transparent background around it."
  — Blender 5.3 Manual, `https://docs.blender.org/manual/en/5.3/render/output/properties/format.html`, read 2026-10-11.

- **Exact words:**
  > "use_crop_to_border
  > Crop the rendered frame to the defined render region size (default False)"
  > 
  > "use_border
  > Render a user-defined render region, within the frame size (default False)"
  > 
  > "border_min_x
  > Minimum X value for the render region (in [0, 1], default 0.0)"
  > 
  > "border_max_x
  > Maximum X value for the render region (in [0, 1], default 1.0)"
  > 
  > "border_min_y
  > Minimum Y value for the render region (in [0, 1], default 0.0)"
  > 
  > "border_max_y
  > Maximum Y value for the render region (in [0, 1], default 1.0)"
  — Blender 5.3 Python API, `https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html`, read 2026-10-11.

- **Exact words:**
  > "If you define one while in Camera View, it will apply not just to the viewport, but also to the final render. If you want to temporarily disable this region rather than clearing it entirely, you can do so in the Output tab of the Properties editor."
  > "However, they always affect the final render."
  — Blender 5.3 Manual, `https://docs.blender.org/manual/en/5.3/editors/3dview/navigate/regions.html`, read 2026-10-11.

#### 2. Stereoscopy / Stereo 3D Multi-View Formats
When stereoscopy is enabled and the output format is set to Stereo 3D with Side-by-Side or Top-Bottom layout without frame squeezing, the pixel dimensions of the written file are multiplied (doubled horizontally or vertically) relative to the scene's base pixel dimensions.

- **Exact words:**
  > "Lossless Formats
  > Some formats will preserve the original data, leading to no problems on exporting and importing the files back in Blender. The Individual option will produce separate images that (if saved in a lossless encoding such as PNG or OpenEXR) can be loaded back in production with no loss of data. For the Stereo 3D formats the only lossless options are Top-Bottom and Side-by-Side without the Squeezed Frame option."
  > 
  > "Lossy-Formats
  > Some stereo 3D formats represent a considerable loss of data... The Top-Bottom compressed will discard half of your vertical resolution data."
  — Blender 5.3 Manual, `https://docs.blender.org/manual/en/5.3/render/output/properties/stereoscopy/usage.html`, read 2026-10-11.

- **Exact words:**
  > "SIDEBYSIDE
  > Side-by-Side – Render views for left and right eyes side-by-side."
  > 
  > "TOPBOTTOM
  > Top-Bottom – Render views for left and right eyes one above another."
  > 
  > "use_squeezed_frame
  > Combine both views in a squeezed image (default False)"
  — Blender 5.3 Python API, `https://docs.blender.org/api/5.3/bpy.types.Stereo3dFormat.html`, read 2026-10-11.

- **Inference step:**
  The source states that Top-Bottom and Side-by-Side without the squeezed frame option preserve full data, whereas compressed/squeezed frames discard half of the vertical or horizontal axis resolution to fit inside the standard frame size. It follows that when `use_squeezed_frame` is `False`, placing both full-resolution views side-by-side or top-bottom doubles the horizontal or vertical pixel dimensions of the saved image file compared to the scene's resolution setting.

#### 3. Compositor Group Output Operation Domain vs. Intermediate Nodes
Within the compositor node tree, intermediate nodes can generate or resize images to custom dimensions (e.g. Scale node set to Absolute Space with custom X and Y pixel sizes, Crop node with Alpha Crop disabled, Blank Image node with custom Size input, or Image nodes loading images of arbitrary sizes). However, for the main render result written via the Group Output node, these node dimensions do not decide the written image size: the Group Output node's operation domain strictly matches the scene render size, and incoming data is clipped, padded, or interpolated to fit.

- **Exact words:**
  > "Output Nodes
  > The operation domain of output nodes like the Viewer node is a domain with an identity transformation and the same size as the final compositor output. For viewport compositing, that size would be the viewport size, and for final render compositing, that size would be the scene render size."
  > 
  > "Compositor Nodes operate on a specific rectangular area of the compositing space called the Operation Domain. The nodes only consider the area of the input images that overlap the operation domain and ignores the rest of the images. If an input image doesn’t completely overlap the operation domain, the rest of the operation domain for that input will be assumed to be a zero value, a zero vector, or a transparent zero color depending on the type."
  > 
  > "Interpolation
  > If an input image to a node is not perfectly aligned with the operation domain of the node or has a different size in pixels, the node would typically need to do a process called Interpolation, where the input image is read at the exact positions of the pixels of the operation domain."
  — Blender 5.3 Manual, `https://docs.blender.org/manual/en/5.3/compositing/compositor_system.html`, read 2026-10-11.

- **Exact words:**
  > "The views to render are defined in the current scene views, in a similar way as you define the composite output resolution in the current scene render panel, regardless of the Image nodes resolutions or Render Layers from different scenes."
  — Blender 5.3 Manual, `https://docs.blender.org/manual/en/5.3/render/output/properties/stereoscopy/usage.html`, read 2026-10-11.

- **Exact words (Scale node):**
  > "Absolute:
  > Size of an image by using absolute pixel values."
  — Blender 5.3 Manual, `https://docs.blender.org/manual/en/5.3/compositing/types/transform/scale.html`, read 2026-10-11.

- **Exact words (Crop node):**
  > "The Crop node crops an input image to a selected region by either making the cropped area transparent or by resizing the input image."
  > "Alpha Crop: Set areas outside the crop region to be transparent instead of changing the dimensions of the image. When enabled, the output image retains the same resolution as the input image, with pixels outside the crop region replaced by transparency."
  — Blender 5.3 Manual, `https://docs.blender.org/manual/en/5.3/compositing/types/transform/crop.html`, read 2026-10-11.

- **Exact words (Blank Image node):**
  > "The Blank Image node generates an image of a specified size filled with a constant color."
  > "Size: The size of the image."
  — Blender 5.3 Manual, `https://docs.blender.org/manual/en/5.3/compositing/types/input/blank_image.html`, read 2026-10-11.

#### 4. File Output Node (`CompositorNodeOutputFile`)
The File Output node writes files to disk directly during rendering or compositing, saving intermediate passes or processed results from anywhere in the node tree.
- **Exact words:**
  > "The File Output node writes one or more images to disk during rendering or compositing. Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence."
  > "Save intermediate results from anywhere in the node tree."
  — Blender 5.3 Manual, `https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html`, read 2026-10-11.
- **Silence in the sources:**
  The manual documents that the File Output node saves intermediate images, but it does not state whether the File Output node operates as an "Output Node" (whose domain is fixed to the scene render size) or as an "Other Node" (whose domain is defined by its connected input). Consequently, whether a File Output node writes files sized to the connected node's custom pixel dimensions or forces them to the scene render resolution is not in the sources.

#### 5. GPU Compositing 8K Resolution Limit
If compositing is executed using the GPU compositor (configured under Compositor Options / Performance), hardware limits clamp and alter image resolution.
- **Exact words:**
  > "Resolution
  > Images with resolution larger than 8k are not supported and are cropped/scaled."
  — Blender 5.3 Manual, `https://docs.blender.org/manual/en/5.3/compositing/limitations.html`, read 2026-10-11.

---

### Reading 2: File Size on Disk in Bytes

**LABEL:** documented.

**NOT COVERED:**
Exact compression ratios and byte sizes resulting from scene visual complexity or lossy codecs.

If "size of what gets written" refers to file size in bytes, the following settings inside a blend-file decide it:
1. **File format (`file_format` / `format`):** Selecting uncompressed, lossless, or lossy formats (OpenEXR, TIFF, PNG, JPEG, WebP, AVI, FFmpeg video).
   - `manual_5.3/render/output/properties/output.html`, read 2026-10-11.
2. **Color depth (`color_depth`):** Choosing 8-bit, 10-bit, 12-bit, 16-bit, or 32-bit float channels directly multiplies raw data volume.
   - `manual_5.3/render/output/properties/output.html`, read 2026-10-11.
3. **Color channels (`color_mode`):** Selecting BW (grayscale, 1 channel), RGB (3 channels), or RGBA (4 channels).
   - `manual_5.3/render/output/properties/output.html`, read 2026-10-11.
4. **Compression and encoding settings:** Compression codecs (ZIP, PIZ, DWAA for OpenEXR; compression level for PNG; Quality/CRF/bitrate for JPEG and video codecs).
   - `manual_5.3/render/output/properties/output.html`, read 2026-10-11.
5. **Autosplit Output:** For video output, enabling Autosplit Output limits file size by automatically splitting the stream into separate files once a file reaches 2GiB.
   - Exact words:
     > "Autosplit Output
     > If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."
     — Blender 5.3 Manual, `https://docs.blender.org/manual/en/5.3/render/output/properties/output.html`, read 2026-10-11.
6. **Frame Range and Frame Step (`frame_start`, `frame_end`, `frame_step`):** Decides the number of rendered image files or video frame count written to disk.
   - `manual_5.3/render/output/animation.html`, read 2026-10-11.

---

### Record of Where Looked

**Files searched:**
- `manual_5.3/render/output/properties/format.txt`
- `manual_5.3/render/output/properties/output.txt`
- `manual_5.3/render/output/properties/post_processing.txt`
- `manual_5.3/render/output/properties/stereoscopy/usage.txt`
- `manual_5.3/render/output/properties/stereoscopy/introduction.txt`
- `manual_5.3/render/output/animation.txt`
- `manual_5.3/compositing/introduction.txt`
- `manual_5.3/compositing/compositor_system.txt`
- `manual_5.3/compositing/sidebar.txt`
- `manual_5.3/compositing/limitations.txt`
- `manual_5.3/compositing/types/output/index.txt`
- `manual_5.3/compositing/types/output/group_output.txt`
- `manual_5.3/compositing/types/output/file_output.txt`
- `manual_5.3/compositing/types/transform/scale.txt`
- `manual_5.3/compositing/types/transform/crop.txt`
- `manual_5.3/compositing/types/input/blank_image.txt`
- `manual_5.3/editors/3dview/navigate/regions.txt`
- `manual_5.3/editors/3dview/viewport_render.txt`
- `manual_5.3/video_editing/edit/montage/editing.txt`
- `manual_5.3/video_editing/edit/montage/strips/strip_properties.txt`
- `manual_5.3/advanced/command_line/render.txt`
- `manual_5.3/advanced/command_line/arguments.txt`
- `api_5.3/bpy.types.RenderSettings.txt`
- `api_5.3/bpy.types.Scene.txt`
- `api_5.3/bpy.types.ImageFormatSettings.txt`
- `api_5.3/bpy.types.Stereo3dFormat.txt`
- `api_5.3/bpy.types.CompositorNodeOutputFile.txt`
- `api_5.3/bpy.types.NodeCompositorFileOutputItem.txt`
- `api_5.3/bpy.types.Camera.txt`
- `api_5.3/bpy.types.SequenceEditor.txt`
- `api_5.3/bpy.types.SceneStrip.txt`
- `release_notes_5.3/compositor.md`
- `release_notes_5.3/sequencer.md`
- `cycles_5.3/ui.py`
- `cycles_5.3/properties.py`
- `cycles_5.3/version_update.py`

**Terms searched:**
- `resolution`, `resolution_x`, `resolution_y`, `resolution_percentage`
- `render_region`, `Render Region`, `border`, `use_border`, `border_min_x`, `border_max_x`, `border_min_y`, `border_max_y`
- `use_crop_to_border`, `Crop to Render Region`, `crop`
- `pixel dimensions`, `pixel aspect`, `pixel_aspect_x`, `pixel_aspect_y`, `pixel density`, `ppm_base`, `ppm_factor`
- `operation domain`, `Domain Input`, `domain priority`, `operation_domain`, `identity transformation`
- `File Output`, `CompositorNodeOutputFile`, `Group Output`, `NodeGroupOutput`
- `Scale Node`, `Crop Node`, `Blank Image Node`, `Image Node`
- `Stereo 3D`, `stereoscopy`, `views_format`, `use_squeezed_frame`, `Side-by-Side`, `Top-Bottom`, `multiview`, `multi-view`
- `Sequencer`, `use_sequencer`, `Set Render Size`
- `overscan`, `film_overscan`, `compositor_overscan`
- `Autosplit Output`, `2GiB`, `color_depth`, `color_mode`, `compression`
- `command line`, `-b`, `-a`, `-f`

**Ruled out and why:**
- **Pixel Aspect Ratio (`Aspect X, Y` / `pixel_aspect_x`, `pixel_aspect_y`):** Ruled out as deciding pixel dimensions. The manual documents that Aspect X/Y controls non-square pixel display and pre-distorts the image projection, while pixel density metadata is calculated from it without changing the image pixel count (`manual_5.3/render/output/properties/format.html`, `manual_5.3/render/output/properties/output.html`).
- **Nodes inside the compositor connected to Group Output:** Ruled out as deciding the main render output size. The manual explicitly states that the Group Output node's operation domain for final render compositing is fixed to the scene render size, and incoming images of different sizes are interpolated, clipped, or padded (`manual_5.3/compositing/compositor_system.html`, `manual_5.3/render/output/properties/stereoscopy/usage.html`).
- **Camera sensor size and focal length:** Ruled out. Sensor width/height and focal length affect field of view and optical framing, not the pixel dimensions of the written raster (`api_5.3/bpy.types.Camera.html`).
- **VSE Strip Transforms / Preview:** Ruled out as deciding output file dimensions. VSE scales or stretches strip contents to fit within the project's resolution; only the operator `Set Render Size` modifies the project resolution, which is just changing the scene's pixel dimensions (`manual_5.3/video_editing/edit/montage/editing.html`).
- **Cycles Tiling / Texture Resolution:** Ruled out. `cscene.tile_size` and texture resolution settings affect rendering memory and tile computation, not the dimensions of the final written image (`cycles_5.3/ui.py`, `cycles_5.3/properties.py`).

---

**Outside the interface.**
This holds when Blender is driven from outside its own window. When Blender is run in background mode (`blender -b file.blend -a` or `-f <frame>`), it reads and executes all render settings saved in the blend-file (`manual_5.3/advanced/command_line/render.html`: "Render the whole animation using all the settings saved in the blend-file"), including `use_crop_to_border`, `use_border`, region boundaries, stereoscopy output options, and compositor nodes, unless explicitly overridden by command-line arguments. Interface-only preview renders (such as Viewport Render) do not execute in background mode.

**Operating system.**
This holds on every operating system Blender runs on. The sources do not document any operating system differences in render resolution, `Crop to Render Region`, stereoscopy format handling, or compositor operation domains. The only OS-specific detail in the sources is a GPU TDR timeout on Windows (`manual_5.3/compositing/limitations.html`), which affects GPU driver reset timeouts during heavy compositor evaluations rather than image size calculations.

**Driven from outside, and operating system — observed, 2026-10-18.** The hand tests for this question were run on the Creator's Mac (Apple M1 Pro, Apple's own graphics, macOS 26.6.2) in Blender 5.3.0 Beta, with no window, and are written out in full in the first-pass document, `Preparation/Blender Research/Findings/5.3/The compositor.md`, under question 6. They settle macOS alone: Linux and Windows were not tried in this sitting, so the line above that the answer "holds on every operating system" is the sources' silence, not a test. Two results bear directly on this document's own claims. **A File Output node wrote at the size of the picture plugged into it** — 122×90 after a Scale set to an absolute 120×90, 200×100 after a relative 0.5, 800×600 from an 800×600 picture — while the scene's own output stayed at the scene size. **And on GPU, above 8k, the written file kept the full scene size but the compositor's result filled only part of it** (on CPU the whole of it), on Apple's own graphics.

---

### Things a person could settle by hand

*(**Run, and pruned — 2026-10-18.** All three items were run on the Creator's Mac on 2026-10-18 and are written into the first-pass document, `Preparation/Blender Research/Findings/5.3/The compositor.md`, under question 6: the File Output node's size (item 1) as entry 20, cases 1, 1b and 1c, of `Audits/Blender Research/The compositor/Hand tests — 5.3.md`; Crop to Render Region with the compositor (item 2) as entries 20 and 21; and unsqueezed Side-by-Side stereo (item 3) as entry 20, case 3. Nothing from this list is still open on macOS; Linux and Windows were not tried.)*

---

2026-10-17 — Gemini 3.8 Flash High (Antigravity) — The compositor, Blender 5.3

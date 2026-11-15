# Colour spaces and image sequences — Blender 5.3

Answers to the Colour spaces and image sequences questions for Blender 5.3, drawn only from Blender's 5.3 sources.

- **Sources consulted:** Blender 5.3 manual, Blender 5.3 Python API documentation and Blender 5.3 release notes, all as copies taken on 2026-11-08; Cycles' own files for 5.3, taken on 2026-10-16. Date read for everything below: 2026-11-08 (Cycles files: 2026-10-16).
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.
- **Hand tests (added 2026-11-15):** observed results from hand tests 1–6, run that day on the Creator's Mac by Claude Opus 5.5 (Claude Code) in Blender 5.3.0, were added under questions 1–5, and the hand-check list at the end was pruned; the same tests ran in 4.5, 5.0, 5.1 and 5.2, whose results are in the list only. The tests, their exact commands and every script are in `Audits/Blender Research/Colour spaces and image sequences/Hand tests — 5.3.md`.

Three things to set beside that line, so it can be judged fully:

1. Before the work, the tool I run in showed me, unasked, the text of the project's `CLAUDE.md` and a one-line-per-entry index of saved notes about the person I work for. I did not open either file, and I did not open or search anything either one points to. Nothing in that text describes how Blender behaves.
2. Once, to check that this answer file did not already exist, I listed the folder this file sits in, filtered to names containing "colour". The command showed nothing. I opened no file there.
3. I also read `Blender Sources/About this folder.md`, which is inside the allowed folder. I did not follow its links, which point outside it.

I started no helpers. Every finding below is my own.

---

## 1. What does Blender do when a file names a colour space that its colour management configuration does not contain? Cover opening the file, a script reading the setting, and rendering.

**Label:** documented for two narrow points (opening update via interop IDs when available, and the file-wide missing-config flag, below); **inferred** (what happens to a missing colour space name when an interop ID matches, the single-flag scope, whether the rendering note is the same change, and script/command-line loading); **not in the sources** for what the setting holds when nothing matches, what a script reads back, and what a render does. **Update 2026-11-15:** observed by hand tests 3 and 2 on the Creator's Mac — see *Observed* below.

**What the sources say**

On opening a file, the 5.3 release notes say that colour space names are updated from interop IDs when available (the passage quoted below begins mid-context with "It", so what "It" refers to and what "The existing name is kept" applies to are not shown in this quote):

> "It also improves compatibility when loading or appending assets from blend files created with another OpenColorIO config: on file load, color space names are updated from the interop IDs when available. The existing `name` is kept, so Python API backwards compatibility is preserved."

— release notes, https://developer.blender.org/docs/release_notes/5.3/python_api/ (read 2026-11-08)

The rendering release notes also list an OpenColorIO interoperability change (inferred to refer to the same change, as this note does not name interop IDs or file load):

> "**OpenColorIO**: Improved interoperability between blend files and assets created with different OpenColorIO configurations."

— release notes, https://developer.blender.org/docs/release_notes/5.3/rendering/ (read 2026-11-08)

The setting that holds the identifier:

> "interop_id — Identifier of the color space that works across OpenColorIO configurations, as defined by the ASWF Color Interop Forum (default 'NONE')"

— API, https://docs.blender.org/api/5.3/bpy.types.ColorManagedInputColorspaceSettings.html (read 2026-11-08)

A script can read a file-wide flag indicating that a colour space, view, or display was not found:

> "is_missing_opencolorio_config — A color space, view or display was not found, which likely means the OpenColorIO config used to create this blend file is missing (default False, readonly)"

— API, https://docs.blender.org/api/5.3/bpy.types.BlendFileColorspace.html (read 2026-11-08). It is reached as `bpy.data.colorspace`: "colorspace — Information about the color space used for data-blocks in a blend file (readonly, never None)", https://docs.blender.org/api/5.3/bpy.types.BlendData.html (read 2026-11-08).

A separate flag covers a configuration that would not load at all, which is a different case from a missing colour space:

> "is_failed_opencolorio_config — The requested OpenColorIO config failed to load (default False, readonly)"

— same BlendFileColorspace page. The 5.3 API change log lists `is_failed_opencolorio_config`, `ocio_config_path` and `ocio_config_source` as added from 5.2 to 5.3. It does not list `is_missing_opencolorio_config` as added. https://docs.blender.org/api/5.3/change_log.html (read 2026-11-08)

**Inferred**

- When the stored name is not in the configuration but the stored identifier matches a colour space that is, the name is changed to that colour space when the file opens. This comes from "color space names are updated from the interop IDs when available". It is inferred, not documented, because the release note does not say "missing name" in so many words (it speaks of files "created with another OpenColorIO config"), and because the same quote ends with "The existing `name` is kept, so Python API backwards compatibility is preserved", which is not explained without the unquoted preceding context.
- When neither the name nor the identifier matches, the release note's "when available" gives no rule. The sources do not say what happens then.
- `is_missing_opencolorio_config` is a single file-wide flag. From its wording, it does not say which image or setting was affected. That is inferred from "A color space, view or display was not found" being a single true-or-false property on the file. No per-image or per-setting missing flag was found in the API pages searched, though `bpy.types.Image` was not read in full for this question.

**Not in the sources** — where I looked:

- What value the colour space setting holds after opening when no match is found: kept as written, replaced by a default, replaced by "None", or something else.
- Whether opening shows a warning, a report, or console output.
- What `Image.colorspace_settings.name` returns to a script in that case.
- How a render treats an image whose colour space is missing: which conversion it uses, whether it fails, whether it warns. In the Cycles files reported here (`maketx.py` and kernel headers), nothing states how a render treats a missing colour space: `maketx.py` (the `argparse_create` function, Cycles files taken 2026-10-16) takes only a `--colorspace` argument with the help text "Color space name. Auto-detected if omitted. Only used with a file path." That is a command for making texture cache files and says nothing about a missing colour space at render time. Whether non-kernel files in `source/` hold colour-space handling was not recorded in the search summary below.

Searches run over all manual, API and release-note files for 5.3: `ocio`, `opencolorio`, `colorspace`, `color space`, `interop`, `missing`, `not found`, `unknown`, `invalid`, `fallback`, `fall back`, `could not be found`, `does not exist`, `warning`, `error`, `report`, `config`. I kept only the hits near colour words. Searches over Cycles' Python files and `source/`: `colorspace`, `color_space`, `ocio`. Kernel hits are `#include "kernel/util/colorspace.h"` across kernel headers (colour maths for the render kernel, with no names) and `source/kernel/osl/strings.h` (`OSL_DEVICE_STRING(u_colorspace, "colorspace", ...)`, an OSL attribute string identifier rather than an OCIO colour space name). *(Note — citation check 2026-11-15, Gemini 4 Hydrogen XHigh: updated the Cycles `source/` hit description to include the `source/kernel/osl/strings.h` hit.)*

Pages opened and read in full: manual `render/color_management/` (index, color_spaces, displays_views, opencolorio, system_configuration), `projects/project_ocio_config`, `editors/image/image_settings`; API `BlendFileColorspace`, `BlendData` (colorspace and project entries), `ColorManagedInputColorspaceSettings`, `ColorManagedDisplaySettings`, `ColorManagedViewSettings`, `ColorManagedSequencerColorspaceSettings`, the two colour-space enum item pages, `change_log`; release notes `python_api`, `rendering`, `core`.

Ruled out: the manual line "Blender will look for a view transform named Standard or Un-tone-mapped or the config wide default_view_transform. If not found, the first view transform of the display will be used." (https://docs.blender.org/manual/en/5.3/render/color_management/opencolorio.html). It is advice on writing a configuration and how Blender picks a plain view from one. It is not about a file naming a colour space that is absent. The magenta "missing images" display in the Sequencer is about missing files, not missing colour spaces.

**Not covered:** the result after opening when no identifier matches; the script-visible value; render behaviour; whether views and displays named in the file follow the same identifier matching. The release note speaks only of "color space names".

**Observed — hand tests 3 and 2, 2026-11-15, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).** The tests, their exact commands and every script are in `Audits/Blender Research/Colour spaces and image sequences/Hand tests — 5.3.md`; what follows stands on its own.

*What was done.* A colour configuration was made as a copy of Blender 5.3's bundled one, changed in five ways: every use of the colour space name `Linear Rec.2020` renamed `Smoozy Renamed Rec.2020` (its definition, its other names and its interop ID `lin_rec2020_scene` kept); a colour space `Smoozy Orphan` added (the same definition, no other names, no interop ID); a colour space `Smoozy Orphan Unknown Id` added (the same, with an interop ID no configuration has, `smoozy_unknown_scene`); a view `Smoozy View` added to the `sRGB` display as a copy of its `False Color` view; and a display `Smoozy Display` added as a copy of `Display P3`. With Blender started with `BLENDER_OCIO` pointing at that configuration, a script made six small files and saved each with Blender's ordinary save. Each holds an orthographic camera and one plane filling it, with an Emission shader fed by an Image Texture node showing an 8 × 8 picture of one colour (red 200, green 100, blue 50): `col_control` (image colour space `sRGB`), `col_renamed` (`Smoozy Renamed Rec.2020`), `col_orphan` (`Smoozy Orphan`) and `col_orphan_unknown` (`Smoozy Orphan Unknown Id`), all with view `Standard`, display `sRGB` and EXR output; `col_view` (image `sRGB`, view `Smoozy View`, PNG output) and `col_display` (image `sRGB`, display `Smoozy Display`, PNG output). With Blender's own configuration (nothing set), a script made reference files the same way: image `sRGB`, `Non-Color`, `Linear Rec.709` and `Linear Rec.2020` (EXR), and view `Standard`, `False Color` and `AgX` with display `sRGB`, and view `Standard` with display `Display P3` (PNG).

Each `col_` file was then opened with Blender's own configuration, once given on the command line and once opened from a script with `bpy.ops.wm.open_mainfile`, and a script printed `bpy.data.colorspace`, the image's colour space name, `interop_id` and `is_data`, whether that name is among the choices Blender now offers, and the scene's display and view; then it set the image to `Smoozy Orphan`, the view to `Smoozy View` and the display to `Smoozy Display`. Each file was rendered from the command line (`-f 1`; Cycles on the CPU, 1 sample, 8 × 8 pixels) with Blender's own configuration and again with the custom one, and every reference file with Blender's own. A script opened every render with its colour space set to Non-Color and printed three pixels of the middle row. Last, each file was saved unchanged under a new name with Blender's own configuration, and the copies were opened with the custom one.

*What came back.* Opening each file with Blender's own configuration printed, file by file (Blender's `Read blend:` line left out; Blender's messages sit among the script's lines where they were printed):
```
READ bpy.data.colorspace: working_space='Linear Rec.709', working_space_interop_id='lin_rec709_scene', is_missing_opencolorio_config=False, is_failed_opencolorio_config=False, ocio_config_path='/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio', ocio_config_source='BLENDER'
READ image colour space name 'sRGB' | interop_id 'srgb_rec709_display' | is_data False | name among the choices offered now: True
READ display 'sRGB' (among the choices: True) | view 'Standard' (among the choices: True) | look 'None'
```
```
READ bpy.data.colorspace: working_space='Linear Rec.709', working_space_interop_id='lin_rec709_scene', is_missing_opencolorio_config=False, is_failed_opencolorio_config=False, ocio_config_path='/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio', ocio_config_source='BLENDER'
READ image colour space name 'Linear Rec.2020' | interop_id 'lin_rec2020_scene' | is_data False | name among the choices offered now: True
READ display 'sRGB' (among the choices: True) | view 'Standard' (among the choices: True) | look 'None'
```
```
00:00.368  color_management | WARNING image colorspace "Smoozy Orphan" not found, will use default instead.
READ bpy.data.colorspace: working_space='Linear Rec.709', working_space_interop_id='lin_rec709_scene', is_missing_opencolorio_config=True, is_failed_opencolorio_config=False, ocio_config_path='/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio', ocio_config_source='BLENDER'
00:00.369  bpy.rna          | WARNING current value '-1' matches no enum in 'ColorManagedInputColorspaceSettings', '(null)', 'name'
00:00.369  bpy.rna          | WARNING current value '-1' matches no enum in 'ColorManagedInputColorspaceSettings', '(null)', 'name'
READ image colour space name '' | interop_id 'NONE' | is_data False | name among the choices offered now: False
READ display 'sRGB' (among the choices: True) | view 'Standard' (among the choices: True) | look 'None'
00:00.369  bpy.rna          | WARNING current value '-1' matches no enum in 'ColorManagedInputColorspaceSettings', '(null)', 'name'
READ set name to 'Smoozy Orphan': TypeError bpy_struct: item.attr = val: enum "Smoozy Orphan" not found in | reads back ''
READ set view_transform to 'Smoozy View': TypeError bpy_struct: item.attr = val: enum "Smoozy View" not found in | reads back 'Standard'
READ set display_device to 'Smoozy Display': TypeError bpy_struct: item.attr = val: enum "Smoozy Display" not found in | reads back 'sRGB'
```
```
00:00.356  color_management | WARNING image colorspace "Smoozy Orphan Unknown Id" not found, will use default instead.
READ bpy.data.colorspace: working_space='Linear Rec.709', working_space_interop_id='lin_rec709_scene', is_missing_opencolorio_config=True, is_failed_opencolorio_config=False, ocio_config_path='/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio', ocio_config_source='BLENDER'
00:00.357  bpy.rna          | WARNING current value '-1' matches no enum in 'ColorManagedInputColorspaceSettings', '(null)', 'name'
00:00.357  bpy.rna          | WARNING current value '-1' matches no enum in 'ColorManagedInputColorspaceSettings', '(null)', 'name'
READ image colour space name '' | interop_id 'NONE' | is_data False | name among the choices offered now: False
00:00.357  bpy.rna          | WARNING current value '-1' matches no enum in 'ColorManagedInputColorspaceSettings', '(null)', 'name'
```
```
00:00.338  color_management | WARNING scene view "Smoozy View" not found, setting to "Standard".
READ bpy.data.colorspace: working_space='Linear Rec.709', working_space_interop_id='lin_rec709_scene', is_missing_opencolorio_config=True, is_failed_opencolorio_config=False, ocio_config_path='/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio', ocio_config_source='BLENDER'
READ image colour space name 'sRGB' | interop_id 'srgb_rec709_display' | is_data False | name among the choices offered now: True
READ display 'sRGB' (among the choices: True) | view 'Standard' (among the choices: True) | look 'None'
```
```
00:00.357  color_management | WARNING Display "Smoozy Display" used by scene not found, setting to "sRGB".
READ bpy.data.colorspace: working_space='Linear Rec.709', working_space_interop_id='lin_rec709_scene', is_missing_opencolorio_config=True, is_failed_opencolorio_config=False, ocio_config_path='/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio', ocio_config_source='BLENDER'
READ image colour space name 'sRGB' | interop_id 'srgb_rec709_display' | is_data False | name among the choices offered now: True
READ display 'sRGB' (among the choices: True) | view 'Standard' (among the choices: True) | look 'None'
```
Opening them from a script with `bpy.ops.wm.open_mainfile` printed the same lines and the same messages, file by file. In every file, setting the image to `Smoozy Orphan`, the view to `Smoozy View` and the display to `Smoozy Display` was refused in the same words as in `col_orphan` above, each setting reading back what it was before. With the custom configuration, each file's image read back the name it was made with:
```
READ image colour space name 'sRGB' | interop_id 'srgb_rec709_display' | is_data False | name among the choices offered now: True
READ image colour space name 'Smoozy Renamed Rec.2020' | interop_id 'lin_rec2020_scene' | is_data False | name among the choices offered now: True
READ image colour space name 'Smoozy Orphan' | interop_id 'NONE' | is_data False | name among the choices offered now: True
READ image colour space name 'Smoozy Orphan Unknown Id' | interop_id 'NONE' | is_data False | name among the choices offered now: True
READ image colour space name 'sRGB' | interop_id 'srgb_rec709_display' | is_data False | name among the choices offered now: True
READ image colour space name 'sRGB' | interop_id 'srgb_rec709_display' | is_data False | name among the choices offered now: True
```
Rendering with Blender's own configuration printed, besides its progress lines (nothing else for `col_control` and `col_renamed`):
```
00:00.364  color_management | WARNING image colorspace "Smoozy Orphan" not found, will use default instead.
00:00.401  cycles           | WARNING Colorspace  can't be converted to scene_linear: ColorSpaceTransform: empty source color space name.
00:00.401  cycles           | WARNING Colorspace  not found, using scene linear instead
```
```
00:00.351  color_management | WARNING image colorspace "Smoozy Orphan Unknown Id" not found, will use default instead.
00:00.385  cycles           | WARNING Colorspace  can't be converted to scene_linear: ColorSpaceTransform: empty source color space name.
00:00.385  cycles           | WARNING Colorspace  not found, using scene linear instead
```
```
00:00.380  color_management | WARNING scene view "Smoozy View" not found, setting to "Standard".
```
```
00:00.361  color_management | WARNING Display "Smoozy Display" used by scene not found, setting to "sRGB".
```
Every render read back (`colour_bundled`: Blender's own configuration; `colour_custom`: the custom one; `colour_ref`: the references; `L`, `M`, `R` are three pixels of the middle row; `no match` is the reading script's note that the numbers are not one of the test sequence pictures):
```
PIX colour_bundled/col_control_0001.exr | L=(0.5776, 0.1274, 0.0319) no match | M=(0.5776, 0.1274, 0.0319) no match | R=(0.5776, 0.1274, 0.0319) no match
PIX colour_bundled/col_display_0001.png | L=(0.7804, 0.3882, 0.1922) no match | M=(0.7843, 0.3922, 0.1961) no match | R=(0.7843, 0.3922, 0.1961) no match
PIX colour_bundled/col_orphan_0001.exr | L=(0.7843, 0.3922, 0.1961) no match | M=(0.7843, 0.3922, 0.1961) no match | R=(0.7843, 0.3922, 0.1961) no match
PIX colour_bundled/col_orphan_unknown_0001.exr | L=(0.7843, 0.3922, 0.1961) no match | M=(0.7843, 0.3922, 0.1961) no match | R=(0.7843, 0.3922, 0.1961) no match
PIX colour_bundled/col_renamed_0001.exr | L=(1.0576, 0.3450, 0.1656) no match | M=(1.0576, 0.3450, 0.1656) no match | R=(1.0576, 0.3450, 0.1656) no match
PIX colour_bundled/col_view_0001.png | L=(0.7804, 0.3882, 0.1922) no match | M=(0.7843, 0.3922, 0.1961) no match | R=(0.7843, 0.3922, 0.1961) no match
PIX colour_custom/col_control_0001.exr | L=(0.5776, 0.1274, 0.0319) no match | M=(0.5776, 0.1274, 0.0319) no match | R=(0.5776, 0.1274, 0.0319) no match
PIX colour_custom/col_display_0001.png | L=(0.7294, 0.4118, 0.2392) no match | M=(0.7373, 0.4157, 0.2431) no match | R=(0.7333, 0.4157, 0.2431) no match
PIX colour_custom/col_orphan_0001.exr | L=(1.0576, 0.3450, 0.1656) no match | M=(1.0576, 0.3450, 0.1656) no match | R=(1.0576, 0.3450, 0.1656) no match
PIX colour_custom/col_orphan_unknown_0001.exr | L=(1.0576, 0.3450, 0.1656) no match | M=(1.0576, 0.3450, 0.1656) no match | R=(1.0576, 0.3450, 0.1656) no match
PIX colour_custom/col_renamed_0001.exr | L=(1.0576, 0.3450, 0.1656) no match | M=(1.0576, 0.3450, 0.1656) no match | R=(1.0576, 0.3450, 0.1656) no match
PIX colour_custom/col_view_0001.png | L=(0.4706, 0.4706, 0.4706) no match | M=(0.4745, 0.4745, 0.4745) no match | R=(0.4745, 0.4745, 0.4745) no match
PIX colour_ref/ref_display_p3_0001.png | L=(0.7294, 0.4118, 0.2392) no match | M=(0.7373, 0.4157, 0.2431) no match | R=(0.7333, 0.4157, 0.2431) no match
PIX colour_ref/ref_lin2020_0001.exr | L=(1.0576, 0.3450, 0.1656) no match | M=(1.0576, 0.3450, 0.1656) no match | R=(1.0576, 0.3450, 0.1656) no match
PIX colour_ref/ref_lin709_0001.exr | L=(0.7843, 0.3922, 0.1961) no match | M=(0.7843, 0.3922, 0.1961) no match | R=(0.7843, 0.3922, 0.1961) no match
PIX colour_ref/ref_noncolor_0001.exr | L=(0.7843, 0.3922, 0.1961) no match | M=(0.7843, 0.3922, 0.1961) no match | R=(0.7843, 0.3922, 0.1961) no match
PIX colour_ref/ref_srgb_0001.exr | L=(0.5776, 0.1274, 0.0319) no match | M=(0.5776, 0.1274, 0.0319) no match | R=(0.5776, 0.1274, 0.0319) no match
PIX colour_ref/ref_view_agx_0001.png | L=(0.7255, 0.3882, 0.2000) no match | M=(0.7294, 0.3922, 0.2039) no match | R=(0.7294, 0.3922, 0.2000) no match
PIX colour_ref/ref_view_falsecolor_0001.png | L=(0.4706, 0.4706, 0.4706) no match | M=(0.4745, 0.4745, 0.4745) no match | R=(0.4745, 0.4745, 0.4745) no match
PIX colour_ref/ref_view_standard_0001.png | L=(0.7804, 0.3882, 0.1922) no match | M=(0.7843, 0.3922, 0.1961) no match | R=(0.7843, 0.3922, 0.1961) no match
```
Set side by side, from the lines above: with Blender's own configuration, `col_orphan` and `col_orphan_unknown` printed (0.7843, 0.3922, 0.1961), the numbers `ref_lin709` and `ref_noncolor` printed, and not `ref_srgb`'s (0.5776, 0.1274, 0.0319) nor their own render with the custom configuration (1.0576, 0.3450, 0.1656, `ref_lin2020`'s numbers). `col_renamed` printed `ref_lin2020`'s numbers with both configurations. `col_view` printed `ref_view_standard`'s numbers with Blender's own configuration and `ref_view_falsecolor`'s with the custom one; `col_display` printed `ref_view_standard`'s numbers with Blender's own configuration and `ref_display_p3`'s with the custom one. This test cannot tell Linear Rec.709 from Non-Color apart: their two reference renders printed the same numbers.

Saving with Blender's own configuration, then opening the saved copy with the custom one:
```
RESAVE …/files/5.3/resaved/col_renamed.blend {'FINISHED'} | image 'Linear Rec.2020' | display 'sRGB' | view 'Standard'
```
```
RESAVE …/files/5.3/resaved/col_orphan.blend {'FINISHED'} | image '' | display 'sRGB' | view 'Standard'
```
```
RESAVE …/files/5.3/resaved/col_view.blend {'FINISHED'} | image 'sRGB' | display 'sRGB' | view 'Standard'
```
```
RESAVE …/files/5.3/resaved/col_display.blend {'FINISHED'} | image 'sRGB' | display 'sRGB' | view 'Standard'
```
```
00:00.018  color_management | Using config from BLENDER_OCIO: "…/ocio/5.3/custom/config.ocio"
READ open_mainfile …/files/5.3/resaved/col_control.blend {'FINISHED'}
READ image colour space name 'sRGB' | interop_id 'srgb_rec709_display' | is_data False | name among the choices offered now: True
READ display 'sRGB' (among the choices: True) | view 'Standard' (among the choices: True) | look 'None'
READ open_mainfile …/files/5.3/resaved/col_renamed.blend {'FINISHED'}
READ image colour space name 'Smoozy Renamed Rec.2020' | interop_id 'lin_rec2020_scene' | is_data False | name among the choices offered now: True
READ display 'sRGB' (among the choices: True) | view 'Standard' (among the choices: True) | look 'None'
READ open_mainfile …/files/5.3/resaved/col_orphan.blend {'FINISHED'}
READ image colour space name '' | interop_id 'NONE' | is_data False | name among the choices offered now: False
READ display 'sRGB' (among the choices: True) | view 'Standard' (among the choices: True) | look 'None'
READ open_mainfile …/files/5.3/resaved/col_orphan_unknown.blend {'FINISHED'}
READ image colour space name '' | interop_id 'NONE' | is_data False | name among the choices offered now: False
READ display 'sRGB' (among the choices: True) | view 'Standard' (among the choices: True) | look 'None'
READ open_mainfile …/files/5.3/resaved/col_view.blend {'FINISHED'}
READ image colour space name 'sRGB' | interop_id 'srgb_rec709_display' | is_data False | name among the choices offered now: True
READ display 'sRGB' (among the choices: True) | view 'Standard' (among the choices: True) | look 'None'
READ open_mainfile …/files/5.3/resaved/col_display.blend {'FINISHED'}
READ image colour space name 'sRGB' | interop_id 'srgb_rec709_display' | is_data False | name among the choices offered now: True
READ display 'sRGB' (among the choices: True) | view 'Standard' (among the choices: True) | look 'None'
```
Hand test 2 also opened, from the command line, a file saved with the ordinary save inside a Blender project (made with `bpy.ops.project.new`) whose configuration choice was ACES 2.0 Studio; nothing in the file's colour settings had been changed from a new file's. It printed:
```
00:00.322  project          | Open project: "…/projects/5.3/aces2"
00:00.342  color_management | WARNING Display "sRGB" used by scene not found, setting to "sRGB - Display".
00:00.343  color_management | WARNING scene view "AgX" not found, setting to "ACES 2.0 - SDR 100 nits (Rec.709)".
00:00.343  color_management | WARNING Display "sRGB" used by scene output not found, setting to "sRGB - Display".
00:00.343  color_management | WARNING scene output view "Standard" not found, setting to "Un-tone-mapped".
CFG bpy.data.colorspace.is_missing_opencolorio_config = True
CFG display_device: 'sRGB - Display' choices ['NONE']
CFG view_transform: 'ACES 2.0 - SDR 100 nits (Rec.709)' choices ['NONE']
```

*Not covered by these tests:* what the window shows (the Color Space field, the Color Management panel, any report in the status bar); a missing Look; other render engines than Cycles; a missing name on a Sequencer strip, a Movie Clip or an Environment Texture.

**Outside the interface:** The flag and the identifier are script properties. The release note says the update happens "on file load" and does not mention the interface, so I infer it also happens when the file is opened by a script or from the command line. The sources do not say anything separate about background or command-line use. **Update 2026-11-15 — observed** (hand tests 3 and 2, Creator's Mac, Blender 5.3.0): every result under *Observed* came from a Blender with no window. A file given on the command line and the same file opened from a script with `bpy.ops.wm.open_mainfile` printed the same messages and read back the same values, and the command-line render (`-f 1`) gave the pixels quoted there.

**Operating system:** The sources do not say this differs by operating system. **Update 2026-11-15 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

---

## 2. Where does Blender's colour management configuration come from, and how can a script learn which configuration is in use and which colour spaces it contains?

**Label:** documented (where the configuration comes from; which configuration is in use; that available colour spaces depend on the active configuration); **inferred** (how a script lists those choices via `enum_items`, whether `ColorManagedInputColorspaceSettings.name` covers all colour spaces in the active configuration, and script property availability outside the interface); **not in the sources** (the exact script steps from a type or instance to `enum_items`, any direct function returning the list, where `ocio://` configs come from, whether user data files override OCIO configs, and whether `.blender_project` configs load in background/command-line runs). **Update 2026-11-15:** observed by hand tests 1 and 2 on the Creator's Mac — see *Observed* below.

**Where it comes from — documented**

> "Blender searches for an OpenColorIO configuration in the following order, using the first available valid configuration: The BLENDER_OCIO environment variable. The OCIO environment variable. The active Blender Project configuration. Blender's bundled configuration. Blender's fallback configuration."

— manual, https://docs.blender.org/manual/en/5.3/render/color_management/opencolorio.html (read 2026-11-08)

The same page, on the two environment variables:

> "Blender will use the standard OCIO environment variable to read an OpenColorIO configuration other than the default Blender one." … "There is also a BLENDER_OCIO environment variable to change the configuration for Blender only."

And on the command-line page:

> "BLENDER_OCIO: Path to override the OpenColorIO configuration file. If not set, the OCIO environment variable is used."

— manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html (read 2026-11-08)

The project choice:

> "A Blender Project can have its own custom OpenColorIO configuration. This configuration will be automatically loaded and used for blend files in the project."

> "Changes to the OpenColorIO configuration are not applied automatically. The blend-file must be reloaded for the updated configuration to take effect."

— manual, https://docs.blender.org/manual/en/5.3/projects/project_ocio_config.html (read 2026-11-08)

The release notes add how a file belongs to a project:

> "Each project has a root directory, and all files under that directory are considered part of the project. The root is identified by a `.blender_project` directory inside it"

— release notes, https://developer.blender.org/docs/release_notes/5.3/core/ (read 2026-11-08)

Where the bundled configuration sits on disk:

> "./datafiles/colormanagement — Default OpenColorIO configuration."

with the system folder given per operating system, for example "macOS ./Blender.app/Contents/Resources/5.3/", "Linux distribution packages: /usr/share/blender/5.3/", "Installer downloaded from blender.org: %ProgramFiles%\Blender Foundation\Blender\5.3\". The same page says of `./datafiles`: "Located in both user and system directories. User data files either override or add to system data files."

— manual, https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html (read 2026-11-08)

**Which configuration is in use — documented**

In the interface: "The currently loaded OpenColorIO configuration can be inspected in Render Properties ‣ Color Management ‣ Advanced." with fields "Source — Displays where the active OpenColorIO configuration was loaded from…" and "Path — Displays the path to the loaded OpenColorIO configuration file." (opencolorio page above).

From a script, through `bpy.data.colorspace`:

> "ocio_config_path — File path or URI of the OpenColorIO configuration currently in use (default "", readonly, never None)"

> "ocio_config_source — Where the OpenColorIO configuration came from (default 'BLENDER_OCIO', readonly)" with the values `BLENDER_OCIO`, `OCIO`, `PROJECT`, `BLENDER`, and "FALLBACK — Fallback – The built-in fallback configuration, if loading other configurations failed."

> "is_failed_opencolorio_config — The requested OpenColorIO config failed to load (default False, readonly)"

— API, https://docs.blender.org/api/5.3/bpy.types.BlendFileColorspace.html (read 2026-11-08)

The project's own choice, through `bpy.data.project` ("The currently active Blender project, if any (readonly)", https://docs.blender.org/api/5.3/bpy.types.BlendData.html):

> "ocio_config — OpenColorIO configuration to use for this project (default 'BLENDER')" with values `BLENDER`, `ACES_2_0_STUDIO`, `ACES_1_3_STUDIO`, `PATH`; and "ocio_config_path — Path to the OpenColorIO configuration to use for this project. May be an absolute file path, project relative path with {project_root}, or an ocio:// built-in OpenColorIO config"

— API, https://docs.blender.org/api/5.3/bpy.types.BlenderProject.html (read 2026-11-08)

The version of the colour library itself: "bpy.app.ocio — Constant value bpy.app.ocio(supported=True, version=(2, 5, 0), version_string=' 2, 5, 0')" — https://docs.blender.org/api/5.3/bpy.app.html (read 2026-11-08)

**Which colour spaces it contains — documented in part, the method inferred**

The manual says the list follows the configuration: "The list of color spaces depends on the active OCIO config." (https://docs.blender.org/manual/en/5.3/editors/image/image_settings.html, read 2026-11-08; and the glossary says "The color spaces available in Blender depend on the active OCIO config.", https://docs.blender.org/manual/en/5.3/glossary/index.html, read 2026-11-08; several other manual pages repeat the same note).

The colour space setting is a list-of-choices property whose documented fixed choices are only:

> "Color Space Convert Default Items — NONE: None. Do not perform any color transform on load, treat colors as in scene linear space already."

— API, https://docs.blender.org/api/5.3/bpy_types_enum_items/color_space_convert_default_items.html (read 2026-11-08); the property is "name — Color space in the image file, to convert to and from when saving and loading the image (default 'NONE') Type: Literal[Color Space Convert Default Items]" on https://docs.blender.org/api/5.3/bpy.types.ColorManagedInputColorspaceSettings.html (read 2026-11-08).

A list-of-choices property can be asked for its choices:

> "enum_items — Possible values for the property (default None, readonly)"; "enum_items_static — Possible values for the property (never calls optional dynamic generation of those)"

— API, https://docs.blender.org/api/5.3/bpy.types.EnumProperty.html; each choice has "identifier — Unique name used in the code and scripting" and "name — Human readable name" (https://docs.blender.org/api/5.3/bpy.types.EnumPropertyItem.html); a type's properties are listed by "properties — Properties in the struct" (https://docs.blender.org/api/5.3/bpy.types.Struct.html). All read 2026-11-08.

**Inferred**

- The documented fixed list holds only "None", and the manual says the list depends on the configuration. So the real list is built while Blender runs from the configuration in use. A script would get it by reading `enum_items` (not `enum_items_static`, which "never calls optional dynamic generation") of the `name` property of `ColorManagedInputColorspaceSettings`. Whether that property lists every colour space in the active configuration (or only input/image spaces) is also inferred rather than stated in the quotes above. The steps from a type or instance to that property's definition are not spelled out in the quotes cited here.

**Not in the sources**

- The exact script path from `ColorManagedInputColorspaceSettings` (or an instance) to its `name` property's `enum_items`. The quote `"The property type for introspection."` is on `rna_type` at https://docs.blender.org/api/5.3/bpy.types.bpy_prop.html (read 2026-11-08). Searching `api_5.3` shows `bl_rna` across 1,832 API files (`.bl_rna` in 1,167 files, e.g. `bl_rna_get_subclass` / `.bl_rna` references rather than a recipe for `ColorManagedInputColorspaceSettings`), and `rna_type` on `bpy_prop` plus `genindex-R` and `genindex-all`. Searches did not cover `enum_items`, `Struct`, `properties[`, `introspection`, or API overview/reference pages. *(Note — citation check 2026-11-15, Gemini 4 Hydrogen XHigh: added the missing `bpy.types.bpy_prop.html` address and corrected the inaccurate `bl_rna`/`rna_type` search hit claim.)*
- Any function on `bpy.app`, `bpy.utils` or elsewhere that returns the colour spaces directly. Searches over API pages that are not class pages: `color space`, `colorspace`, `ocio`, `opencolorio`. Hits were `bpy.app` (version only), `bpy.ops.wm` (`set_working_color_space` and PLY/OBJ/STL vertex colour-space parameters), image, scene import/export and sequencer operators (parameters only), `change_log` (`BlendFileColorspace` entries), `gpu.texture`, `gpu_extras.presets`, `gpu.shader` and `mathutils` (fixed conversions only). Which manual, release-note, and API class pages were read or searched specifically for this question (beyond Q1's list) was not recorded here; terms `colormanagement`, `color_management`, `ocio://`, and `fallback` were not recorded as searched. *(Note — citation check 2026-11-15, Gemini 4 Hydrogen XHigh: added the omitted `bpy.ops.wm` and `change_log` non-class API hits.)*
- Where `ocio://` built-in configurations come from (quoted in `BlenderProject.ocio_config_path`) and whether `BLENDER_OCIO` or `OCIO` accepts them. *(Note — citation check 2026-11-15, Gemini 4 Hydrogen XHigh: flagged as not supported on the documented search record because `ocio://` was not recorded as searched above; sent back for research rather than answering from sources not searched here.)*

**Not covered:** whether user data files can supply or override the colour management configuration (the "override or add to" sentence is about data files in general); the second colour-space enum item page noted under Question 1; whether views, displays and looks can be listed the same way (the exact `ColorManagedViewSettings` and `ColorManagedDisplaySettings` quotes were not included here).

**Observed — hand tests 1 and 2, 2026-11-15, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).** The tests, their exact commands and every script are in `Audits/Blender Research/Colour spaces and image sequences/Hand tests — 5.3.md`; what follows stands on its own.

*What was done (test 1).* Three colour configurations were made as copies of Blender 5.3's bundled folder: `plain` (unchanged), `custom` (`Linear Rec.2020` renamed `Smoozy Renamed Rec.2020`; colour spaces `Smoozy Orphan` and `Smoozy Orphan Unknown Id`, a view `Smoozy View` and a display `Smoozy Display` added) and `broken` (its `config.ocio` replaced by one line of text). Blender was started seven times — (a) with neither `OCIO` nor `BLENDER_OCIO` set; (b) `BLENDER_OCIO` set to the custom file; (c) `OCIO` set to it; (d) `BLENDER_OCIO` set to the plain file and `OCIO` to the custom one; (e) `BLENDER_OCIO` set to a path where no file is; (f) `BLENDER_OCIO` set to the broken file; (g) `OCIO` set to the broken file — and each time a script printed `bpy.app.ocio`, every setting of `bpy.data.colorspace`, what `OCIO` and `BLENDER_OCIO` read inside Blender, a new image's colour space, and the choices of an image's colour space setting read four ways: `img.colorspace_settings.bl_rna.properties["name"].enum_items`, the same through `rna_type`, `bpy.types.ColorManagedInputColorspaceSettings.bl_rna.properties["name"].enum_items`, and `enum_items_static`. It set those choices against the colour space names written in the configuration file in use (each `name:` under `colorspaces:` and `display_colorspaces:`), printed the scene's display, view and look choices through `bl_rna`, then set each of those three to `No Such Name` and printed Blender's answer, and last set the image's colour space to `lin_rec709` and to `srgb_tx`, two of the other names the bundled file gives `Linear Rec.709` and `sRGB`.

*What came back.* (a), every line the script printed, the long lists whole:
```
CFG version 5.3.0 | file (none)
CFG env BLENDER_OCIO None | env OCIO '/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio'
CFG bpy.app.ocio bpy.app.ocio(supported=True, version=(2, 5, 0), version_string=' 2,  5,  0')
CFG bpy.data.colorspace.working_space = 'Linear Rec.709'
CFG bpy.data.colorspace.working_space_interop_id = 'lin_rec709_scene'
CFG bpy.data.colorspace.is_missing_opencolorio_config = False
CFG bpy.data.colorspace.is_failed_opencolorio_config = False
CFG bpy.data.colorspace.ocio_config_path = '/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio'
CFG bpy.data.colorspace.ocio_config_source = 'BLENDER'
CFG bpy.data.project = None
CFG new 8-bit image colour space: 'sRGB' | interop_id: 'srgb_rec709_display'
CFG colour space choices via image bl_rna: 64
CFG colour space choices via image rna_type: 64
CFG colour space choices via type bl_rna: 64
CFG colour space choices via enum_items_static: 1
CFG   same list as image bl_rna: image rna_type True
CFG   same list as image bl_rna: type bl_rna True
CFG   same list as image bl_rna: enum_items_static False
CFG colour space choices (image bl_rna): ['ACES 1.3 sRGB', 'ACES 2.0 sRGB', 'ACES2065-1', 'ACEScc', 'ACEScct', 'ACEScg', 'ADX10', 'ADX16', 'ARRI LogC3 (EI800)', 'ARRI LogC4', 'AgX Base sRGB', 'AgX Log', 'Apple Log', 'Apple Log 2', 'BMDFilm WideGamut Gen5', 'CanonLog2 CinemaGamut D55', 'CanonLog3 CinemaGamut D55', 'D-Log D-Gamut', 'DaVinci Intermediate WideGamut', 'Display P3', 'Filmic Log', 'Filmic sRGB', 'Gamma 1.8 Encoded Rec.709', 'Gamma 2.2 Encoded AP1', 'Gamma 2.2 Encoded AdobeRGB', 'Gamma 2.2 Encoded Rec.709', 'Gamma 2.4 Encoded Rec.709', 'Khronos PBR Neutral sRGB', 'Linear ARRI Wide Gamut 3', 'Linear ARRI Wide Gamut 4', 'Linear Adobe RGB', 'Linear Apple Wide Gamut', 'Linear BMD WideGamut Gen5', 'Linear CIE-XYZ D65', 'Linear CIE-XYZ E', 'Linear CinemaGamut D55', 'Linear D-Gamut', 'Linear DCI-P3 D65', 'Linear DaVinci WideGamut', 'Linear FilmLight E-Gamut', 'Linear REDWideGamutRGB', 'Linear Rec.2020', 'Linear Rec.709', 'Linear S-Gamut3', 'Linear S-Gamut3.Cine', 'Linear V-Gamut', 'Linear Venice S-Gamut3', 'Linear Venice S-Gamut3.Cine', 'Log3G10 REDWideGamutRGB', 'Non-Color', 'Rec.1886', 'Rec.2020', 'Rec.2100-HLG', 'Rec.2100-PQ', 'Rec.709-A', 'S-Log3 S-Gamut3', 'S-Log3 S-Gamut3.Cine', 'S-Log3 Venice S-Gamut3', 'S-Log3 Venice S-Gamut3.Cine', 'V-Log V-Gamut', 'sRGB', 'sRGB Encoded AP1', 'sRGB Encoded P3-D65', 'scene_linear']
CFG first three choices with their names: [('ACES 1.3 sRGB', 'ACES 1.3 sRGB'), ('ACES 2.0 sRGB', 'ACES 2.0 sRGB'), ('ACES2065-1', 'ACES2065-1')]
CFG display_device: 'sRGB' choices ['NONE']
CFG view_transform: 'AgX' choices ['NONE']
CFG look: 'None' choices ['NONE']
CFG sequencer colour space: 'sRGB' choices 64
CFG set display_device to 'No Such Name': ERROR TypeError('bpy_struct: item.attr = val: enum "No Such Name" not found in (\'sRGB\', \'Display P3\', \'Rec.1886\', \'Rec.2020\', \'Rec.2100-PQ\', \'Rec.2100-HLG\')') | reads back 'sRGB'
CFG set view_transform to 'No Such Name': ERROR TypeError('bpy_struct: item.attr = val: enum "No Such Name" not found in (\'Standard\', \'ACES 1.3\', \'ACES 2.0\', \'Khronos PBR Neutral\', \'AgX\', \'Filmic\', \'Filmic Log\', \'False Color\', \'Raw\')') | reads back 'AgX'
CFG set look to 'No Such Name': ERROR TypeError('bpy_struct: item.attr = val: enum "No Such Name" not found in (\'None\', \'AgX - Punchy\', \'AgX - Greyscale\', \'AgX - Very High Contrast\', \'AgX - High Contrast\', \'AgX - Medium High Contrast\', \'AgX - Base Contrast\', \'AgX - Medium Low Contrast\', \'AgX - Low Contrast\', \'AgX - Very Low Contrast\')') | reads back 'None'
CFG config file /Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio colour space names in it: 64
CFG   in the file, not among the choices: ['Luminance Compensation Rec.2020']
CFG   among the choices, not in the file: ['scene_linear']
CFG set image colour space to 'lin_rec709': ERROR TypeError('bpy_struct: item.attr = val: enum "lin_rec709" not found in (\'ACES 1.3 sRGB\', \'ACES 2.0 sRGB\', \'ACES2065-1\', \'ACEScc\', \'ACEScct\', \'ACEScg\', \'ADX10\', \'ADX16\', \'ARRI LogC3 (EI800)\', \'ARRI LogC4\', \'AgX Base sRGB\', \'AgX Log\', \'Apple Log\', \'Apple Log 2\', \'BMDFilm WideGamut Gen5\', \'CanonLog2 CinemaGamut D55\', \'CanonLog3 CinemaGamut D55\', \'D-Log D-Gamut\', \'DaVinci Intermediate WideGamut\', \'Display P3\', \'Filmic Log\', \'Filmic sRGB\', \'Gamma 1.8 Encoded Rec.709\', \'Gamma 2.2 Encoded AP1\', \'Gamma 2.2 Encoded AdobeRGB\', \'Gamma 2.2 Encoded Rec.709\', \'Gamma 2.4 Encoded Rec.709\', \'Khronos PBR Neutral sRGB\', \'Linear ARRI Wide Gamut 3\', \'Linear ARRI Wide Gamut 4\', \'Linear Adobe RGB\', \'Linear Apple Wide Gamut\', \'Linear BMD WideGamut Gen5\', \'Linear CIE-XYZ D65\', \'Linear CIE-XYZ E\', \'Linear CinemaGamut D55\', \'Linear D-Gamut\', \'Linear DCI-P3 D65\', \'Linear DaVinci WideGamut\', \'Linear FilmLight E-Gamut\', \'Linear REDWideGamutRGB\', \'Linear Rec.2020\', \'Linear Rec.709\', \'Linear S-Gamut3\', \'Linear S-Gamut3.Cine\', \'Linear V-Gamut\', \'Linear Venice S-Gamut3\', \'Linear Venice S-Gamut3.Cine\', \'Log3G10 REDWideGamutRGB\', \'Non-Color\', \'Rec.1886\', \'Rec.2020\', \'Rec.2100-HLG\', \'Rec.2100-PQ\', \'Rec.709-A\', \'S-Log3 S-Gamut3\', \'S-Log3 S-Gamut3.Cine\', \'S-Log3 Venice S-Gamut3\', \'S-Log3 Venice S-Gamut3.Cine\', \'V-Log V-Gamut\', \'sRGB\', \'sRGB Encoded AP1\', \'sRGB Encoded P3-D65\', \'scene_linear\')'), reads back 'sRGB'
CFG set image colour space to 'srgb_tx': ERROR TypeError('bpy_struct: item.attr = val: enum "srgb_tx" not found in (\'ACES 1.3 sRGB\', \'ACES 2.0 sRGB\', \'ACES2065-1\', \'ACEScc\', \'ACEScct\', \'ACEScg\', \'ADX10\', \'ADX16\', \'ARRI LogC3 (EI800)\', \'ARRI LogC4\', \'AgX Base sRGB\', \'AgX Log\', \'Apple Log\', \'Apple Log 2\', \'BMDFilm WideGamut Gen5\', \'CanonLog2 CinemaGamut D55\', \'CanonLog3 CinemaGamut D55\', \'D-Log D-Gamut\', \'DaVinci Intermediate WideGamut\', \'Display P3\', \'Filmic Log\', \'Filmic sRGB\', \'Gamma 1.8 Encoded Rec.709\', \'Gamma 2.2 Encoded AP1\', \'Gamma 2.2 Encoded AdobeRGB\', \'Gamma 2.2 Encoded Rec.709\', \'Gamma 2.4 Encoded Rec.709\', \'Khronos PBR Neutral sRGB\', \'Linear ARRI Wide Gamut 3\', \'Linear ARRI Wide Gamut 4\', \'Linear Adobe RGB\', \'Linear Apple Wide Gamut\', \'Linear BMD WideGamut Gen5\', \'Linear CIE-XYZ D65\', \'Linear CIE-XYZ E\', \'Linear CinemaGamut D55\', \'Linear D-Gamut\', \'Linear DCI-P3 D65\', \'Linear DaVinci WideGamut\', \'Linear FilmLight E-Gamut\', \'Linear REDWideGamutRGB\', \'Linear Rec.2020\', \'Linear Rec.709\', \'Linear S-Gamut3\', \'Linear S-Gamut3.Cine\', \'Linear V-Gamut\', \'Linear Venice S-Gamut3\', \'Linear Venice S-Gamut3.Cine\', \'Log3G10 REDWideGamutRGB\', \'Non-Color\', \'Rec.1886\', \'Rec.2020\', \'Rec.2100-HLG\', \'Rec.2100-PQ\', \'Rec.709-A\', \'S-Log3 S-Gamut3\', \'S-Log3 S-Gamut3.Cine\', \'S-Log3 Venice S-Gamut3\', \'S-Log3 Venice S-Gamut3.Cine\', \'V-Log V-Gamut\', \'sRGB\', \'sRGB Encoded AP1\', \'sRGB Encoded P3-D65\', \'scene_linear\')'), reads back 'sRGB'
```
(b) to (g), the lines that carry the answer (the rest are as in (a), with the lists following the counts shown):
```
00:00.016  color_management | Using config from BLENDER_OCIO: "…/ocio/5.3/custom/config.ocio"
CFG env BLENDER_OCIO '…/ocio/5.3/custom/config.ocio' | env OCIO '…/ocio/5.3/custom/config.ocio'
CFG bpy.data.colorspace.is_failed_opencolorio_config = False
CFG bpy.data.colorspace.ocio_config_path = '…/ocio/5.3/custom/config.ocio'
CFG bpy.data.colorspace.ocio_config_source = 'BLENDER_OCIO'
CFG colour space choices via image bl_rna: 66
CFG set display_device to 'No Such Name': ERROR TypeError('bpy_struct: item.attr = val: enum "No Such Name" not found in (\'sRGB\', \'Display P3\', \'Rec.1886\', \'Rec.2020\', \'Smoozy Display\', \'Rec.2100-PQ\', \'Rec.2100-HLG\')') | reads back 'sRGB'
CFG set view_transform to 'No Such Name': ERROR TypeError('bpy_struct: item.attr = val: enum "No Such Name" not found in (\'Standard\', \'ACES 1.3\', \'ACES 2.0\', \'Khronos PBR Neutral\', \'AgX\', \'Filmic\', \'Filmic Log\', \'False Color\', \'Raw\', \'Smoozy View\')') | reads back 'AgX'
CFG   in the file, not among the choices: ['Luminance Compensation Rec.2020']
CFG   among the choices, not in the file: ['scene_linear']
```
```
00:00.016  color_management | Using config from OCIO: "…/ocio/5.3/custom/config.ocio"
CFG env BLENDER_OCIO None | env OCIO '…/ocio/5.3/custom/config.ocio'
CFG bpy.data.colorspace.is_failed_opencolorio_config = False
CFG bpy.data.colorspace.ocio_config_path = '…/ocio/5.3/custom/config.ocio'
CFG bpy.data.colorspace.ocio_config_source = 'OCIO'
CFG colour space choices via image bl_rna: 66
```
```
00:00.017  color_management | Using config from BLENDER_OCIO: "…/ocio/5.3/plain/config.ocio"
CFG env BLENDER_OCIO '…/ocio/5.3/plain/config.ocio' | env OCIO '…/ocio/5.3/plain/config.ocio'
CFG bpy.data.colorspace.is_failed_opencolorio_config = False
CFG bpy.data.colorspace.ocio_config_path = '…/ocio/5.3/plain/config.ocio'
CFG bpy.data.colorspace.ocio_config_source = 'BLENDER_OCIO'
CFG colour space choices via image bl_rna: 64
```
```
00:00.004  color_management | ERROR OpenColorIO Error: Error could not read '…/ocio/5.3/nothing/config.ocio' OCIO profile.
00:00.004  color_management | ERROR Failed to load config from BLENDER_OCIO: …/ocio/5.3/nothing/config.ocio
CFG env BLENDER_OCIO '…/ocio/5.3/nothing/config.ocio' | env OCIO '/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio'
CFG bpy.data.colorspace.is_failed_opencolorio_config = False
CFG bpy.data.colorspace.ocio_config_path = '/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio'
CFG bpy.data.colorspace.ocio_config_source = 'BLENDER'
CFG colour space choices via image bl_rna: 64
```
```
00:00.003  color_management | ERROR OpenColorIO Error: Error: Loading the OCIO profile '…/ocio/5.3/broken/config.ocio' failed. yaml-cpp: error at line 1, column 1: operator[] call on a scalar (key: "ocio_profile_version")
00:00.003  color_management | ERROR Failed to load config from BLENDER_OCIO: …/ocio/5.3/broken/config.ocio
CFG env BLENDER_OCIO '…/ocio/5.3/broken/config.ocio' | env OCIO '/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio'
CFG bpy.data.colorspace.is_failed_opencolorio_config = False
CFG bpy.data.colorspace.ocio_config_path = '/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio'
CFG bpy.data.colorspace.ocio_config_source = 'BLENDER'
CFG colour space choices via image bl_rna: 64
```
```
00:00.002  color_management | ERROR OpenColorIO Error: Error: Loading the OCIO profile '…/ocio/5.3/broken/config.ocio' failed. yaml-cpp: error at line 1, column 1: operator[] call on a scalar (key: "ocio_profile_version")
00:00.002  color_management | ERROR Failed to load config from OCIO: …/ocio/5.3/broken/config.ocio
CFG env BLENDER_OCIO None | env OCIO '/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio'
CFG bpy.data.colorspace.is_failed_opencolorio_config = False
CFG bpy.data.colorspace.ocio_config_path = '/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio'
CFG bpy.data.colorspace.ocio_config_source = 'BLENDER'
CFG colour space choices via image bl_rna: 64
```

*What was done (test 2).* A script made two Blender projects with Blender's own operator, `bpy.ops.project.new(name="Smoozy Test Project", directory=<folder>)`: in one it set the project's `ocio_config` to `ACES_2_0_STUDIO`, in the other to `PATH` with `ocio_config_path` the custom configuration file. It saved the project's settings with `bpy.ops.project.config_save()`, saved one file at the project's top folder and one in a folder below it with the ordinary save, and listed what the project folder held. A new Blender then opened each file from the command line, once with `BLENDER_OCIO` set to the custom file, and a script printed what test 1 prints. Last, Blender was started with `BLENDER_OCIO` set to the `ocio://` address the ACES 2.0 project had printed, with no file.

*What came back.* Making the projects:
```
PROJ version 5.3.0
PROJ before: bpy.data.project None | ocio_config_source 'BLENDER' | ocio_config_path '/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio'
PROJ folder exists before: False
PROJ project.new -> {'FINISHED'}
PROJ after project.new: bpy.data.project <bpy_struct, BlenderProject("Smoozy Test Project") at 0x10adb5a80> | ocio_config_source 'BLENDER' | ocio_config_path '/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio'
PROJ set ocio_config 'ACES_2_0_STUDIO' ocio_config_path 'ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5'
PROJ project.config_save -> {'FINISHED'}
PROJ after config_save: bpy.data.project <bpy_struct, BlenderProject("Smoozy Test Project") at 0x10adb5a80> | ocio_config_source 'BLENDER' | ocio_config_path '/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio'
PROJ save …/projects/5.3/aces2/shot.blend {'FINISHED'}
PROJ save …/projects/5.3/aces2/sub/shot_sub.blend {'FINISHED'}
PROJ after saving: bpy.data.project <bpy_struct, BlenderProject("Smoozy Test Project") at 0x10adb5a80> | ocio_config_source 'BLENDER' | ocio_config_path '/Applications/Blender 5.3.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio'
PROJ on disk: …/projects/5.3/aces2/shot.blend 95733 bytes
PROJ on disk: …/projects/5.3/aces2/.blender_project/project.toml 113 bytes
PROJ   | schema_version = 1
PROJ   | name = "Smoozy Test Project"
PROJ   | ocio_config = "ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5"
PROJ   | 
PROJ on disk: …/projects/5.3/aces2/sub/shot_sub.blend 95733 bytes
```
```
PROJ set ocio_config 'PATH' ocio_config_path '…/ocio/5.3/custom/config.ocio'
PROJ on disk: …/projects/5.3/path/shot.blend 95733 bytes
PROJ on disk: …/projects/5.3/path/.blender_project/project.toml 156 bytes
PROJ   | schema_version = 1
PROJ   | name = "Smoozy Test Project"
PROJ   | ocio_config = "…/ocio/5.3/custom/config.ocio"
PROJ   | 
PROJ on disk: …/projects/5.3/path/sub/shot_sub.blend 95733 bytes
```
Opening the ACES 2.0 project's top file, its file one folder down, the top file with `BLENDER_OCIO` set, and the PATH project's top file:
```
00:00.322  project          | Open project: "…/projects/5.3/aces2"
00:00.342  color_management | WARNING Display "sRGB" used by scene not found, setting to "sRGB - Display".
00:00.343  color_management | WARNING scene view "AgX" not found, setting to "ACES 2.0 - SDR 100 nits (Rec.709)".
00:00.343  color_management | WARNING Display "sRGB" used by scene output not found, setting to "sRGB - Display".
00:00.343  color_management | WARNING scene output view "Standard" not found, setting to "Un-tone-mapped".
CFG bpy.data.colorspace.is_missing_opencolorio_config = True
CFG bpy.data.colorspace.ocio_config_path = 'ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5'
CFG bpy.data.colorspace.ocio_config_source = 'PROJECT'
CFG bpy.data.project = <bpy_struct, BlenderProject("Smoozy Test Project") at 0x10da09a80>
CFG   project.name = 'Smoozy Test Project'
CFG   project.root_path = '…/projects/5.3/aces2'
CFG   project.ocio_config = 'ACES_2_0_STUDIO'
CFG   project.ocio_config_path = 'ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5'
CFG new 8-bit image colour space: 'sRGB Encoded Rec.709 (sRGB)' | interop_id: 'srgb_rec709_scene'
CFG colour space choices via image bl_rna: 56
```
```
00:00.324  project          | Open project: "…/projects/5.3/aces2"
CFG bpy.data.colorspace.ocio_config_path = 'ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5'
CFG bpy.data.colorspace.ocio_config_source = 'PROJECT'
CFG   project.root_path = '…/projects/5.3/aces2'
```
```
00:00.016  color_management | Using config from BLENDER_OCIO: "…/ocio/5.3/custom/config.ocio"
00:00.327  project          | Open project: "…/projects/5.3/aces2"
00:00.327  color_management | Using config from BLENDER_OCIO: "…/ocio/5.3/custom/config.ocio"
CFG bpy.data.colorspace.ocio_config_path = '…/ocio/5.3/custom/config.ocio'
CFG bpy.data.colorspace.ocio_config_source = 'BLENDER_OCIO'
CFG   project.ocio_config = 'ACES_2_0_STUDIO'
```
```
00:00.323  project          | Open project: "…/projects/5.3/path"
CFG bpy.data.colorspace.ocio_config_path = '…/ocio/5.3/custom/config.ocio'
CFG bpy.data.colorspace.ocio_config_source = 'PROJECT'
CFG   project.ocio_config = 'PATH'
CFG   project.ocio_config_path = '…/ocio/5.3/custom/config.ocio'
CFG colour space choices via image bl_rna: 66
CFG   in the file, not among the choices: ['Luminance Compensation Rec.2020']
CFG   among the choices, not in the file: ['scene_linear']
```
The address in `BLENDER_OCIO`, no file:
```
00:00.016  color_management | Using config from BLENDER_OCIO: "ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5"
CFG bpy.data.colorspace.working_space = 'ACEScg'
CFG bpy.data.colorspace.working_space_interop_id = 'lin_ap1_scene'
CFG bpy.data.colorspace.is_missing_opencolorio_config = False
CFG bpy.data.colorspace.is_failed_opencolorio_config = False
CFG bpy.data.colorspace.ocio_config_path = 'ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5'
CFG bpy.data.colorspace.ocio_config_source = 'BLENDER_OCIO'
CFG new 8-bit image colour space: 'sRGB Encoded Rec.709 (sRGB)' | interop_id: 'srgb_rec709_scene'
CFG colour space choices via image bl_rna: 56
CFG display_device: 'sRGB - Display' choices ['NONE']
CFG view_transform: 'ACES 2.0 - SDR 100 nits (Rec.709)' choices ['NONE']
```

*Not covered by these tests:* the window's Render Properties ‣ Color Management ‣ Advanced panel; whether the list includes names a configuration marks inactive other than the one above; a project opened by the window's own project commands.

**Outside the interface:** The two environment variables are named on the command-line page, so they apply to a Blender started from the command line. The configuration and project properties are Python API data properties, so I infer they work without the interface; whether dynamic `enum_items` generation works without the interface is not stated in the sources. The sources do not say whether the project configuration is picked up when a file is opened from the command line or in the background; they say only that it applies to "blend files in the project". **Update 2026-11-15 — observed** (hand tests 1 and 2, Creator's Mac, Blender 5.3.0), all with no window: Blender printed `Using config from BLENDER_OCIO` or `Using config from OCIO` and `ocio_config_source` read that variable; a file in a project, at its top or one folder down, given on the command line, opened with `Open project:` and `ocio_config_source` `'PROJECT'`, and with `BLENDER_OCIO` also set it read `'BLENDER_OCIO'`; the colour space choices came back in full through the image's own settings (`bl_rna` or `rna_type`) and through the settings type, `enum_items_static` gave one, and the display, view and look choices read through `bl_rna` gave `['NONE']`, while Blender's refusal of the name `No Such Name` listed them. All quoted under *Observed*.

**Operating system:** The search order is given without any operating-system exception. The bundled configuration's folder differs by operating system, as quoted above. The sources do not say how to set an environment variable on each system, nor do they say whether the script properties or configuration loading differ by operating system beyond the bundled folder path. **Update 2026-11-15 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

---

## 3. For an image whose source is an image sequence, which part of the stored file name stands for the frame number, and how does Blender find the file for a given frame? Include file names whose number has a different count of digits.

**Label:** documented only in outline (the filename must contain a digit); **inferred** (that the frame number sits directly before the extension); **not in the sources** (the exact rule, which number applies when there are several, how the file name is stored, how the file is found, and different digit counts). **Update 2026-11-15:** observed by hand tests 4 on the Creator's Mac — see *Observed* below.

**What the sources say**

> "To load image sequence in any of the supported image file formats, the filename of the images must contain a digit to indicate the frame order (e.g. *-0001.jpg, *-0002.jpg, *-0003.jpg, etc, of any image format), indicating the frame."

— manual, https://docs.blender.org/manual/en/5.3/files/media/image_formats.html, section "Opening an Image Sequence" (read 2026-11-08). The Image Settings page sends readers here: "Image Sequence — An animation where each frame is stored in a separate file. See Opening an Image Sequence." (https://docs.blender.org/manual/en/5.3/editors/image/image_settings.html)

> "Detect Sequences — Automatically looks for image sequences in the selected images (based on the file name)."

— same image formats page; also https://docs.blender.org/manual/en/5.3/editors/image/editing.html. The operator parameter: "use_sequence_detection (bool) – Detect Sequences, Automatically detect animated sequences in selected images (based on file names)" (https://docs.blender.org/api/5.3/bpy.ops.image.html).

The Sequencer pages, which are about Sequencer strips and not image data-blocks, name the pattern:

> "The option detects the frame range of opened images using Blender's frame naming scheme (filename + frame number + .extension)"

— manual, https://docs.blender.org/manual/en/5.3/video_editing/edit/montage/strips/image.html (read 2026-11-08); and "unless their filenames match Blender's numbered sequence pattern, in which case they are grouped into a single image sequence" (https://docs.blender.org/api/5.3/bpy.ops.sequencer.html).

The volume page compares itself to image sequences:

> "Much like with image sequences, all the files should have a numerical suffix in their name; so if you selected smoke-000.vdb in the File Path, there should be a smoke-001.vdb, a smoke-002.vdb and so on."

— manual, https://docs.blender.org/manual/en/5.3/modeling/volumes/properties.html (read 2026-11-08)

The frame setting refers to "#1":

> "frame_start — Global starting frame of the movie/sequence, assuming first picture has a #1"

— API, https://docs.blender.org/api/5.3/bpy.types.ImageUser.html (read 2026-11-08)

**Inferred**

The part of the name that stands for the frame is the number directly before the extension. This comes from "filename + frame number + .extension" and from "numerical suffix" in the comparison with image sequences. It is inferred, not documented, for image data-blocks, for two reasons. The first quote is on a Sequencer page. The second is on a volume page, and it only says "much like" image sequences. The image formats page says only that the name "must contain a digit".

**Not in the sources**

- Which number is used when a name holds more than one, for example `shot2_take3_0001.png`.
- Whether the stored path keeps the first file's name, a name with placeholder characters, or something else. The `#` characters that mark frame numbers are documented only for render output paths and the compositor File Output node, not for reading images.
- How Blender builds or finds the file name for a given frame: whether it keeps the digit count of the stored name, pads with zeros, or looks for any file with a matching number.
- What happens when files in a folder have different digit counts (for example `img_9.png` beside `img_10.png`, or `img_009.png` beside `img_0010.png`), or when a number grows past the digit count of the stored name (for example `999` to `1000`).
- What "#1" means: the file whose number is 1, or the first file found.

Searches run over all 5.3 manual, API and release-note files: `image sequence`, `frame number`, `digit`, `padd`, `numbered`, `leading zero`, `suffix`, `<frame`, `{frame`, `%04d`, `sequence` (in release notes only, excluding "sequencer"), `frame_offset`, `frame_start`, `frame_duration`, `image_user`, `ImageUser`, `filepath_from_user`. Every manual page matching `image sequence` (32 pages case-insensitively; 27 case-sensitively) was checked, and the only naming statements are the ones quoted above. API pages `bpy.types.Image`, `bpy.types.BlendDataImages`, and the remaining parameters of `bpy.ops.image.open` were not reported as read in full for this question. Cycles' Python files and `source/` were searched for `image_user`, `SEQUENCE`, `frame_start`, `frame_offset`, `frame_duration`, `use_cyclic`, `filepath_from_user` (without reporting a case-insensitive search for `sequence` or `image_sequence`). The only frame loop found (`operators.py`, `CYCLES_OT_denoise_animation.execute`) uses `scene.render.frame_path` with `scene.render.filepath` across `scene.frame_start..scene.frame_end` to build both `in_filepaths` (frames matching Blender render output file naming) and `out_filepaths` (denoised output); that uses render-output naming, not `Image` or `ImageUser` sequence filename detection. *(Note — citation check 2026-11-15, Gemini 4 Hydrogen XHigh: corrected the manual `image sequence` page count from 31 to 32 case-insensitive / 27 case-sensitive and narrowed the `operators.py` description to reflect that the loop populates both `in_filepaths` and `out_filepaths` via `scene.render.frame_path`.)*

Ruled out: "Detect UDIMs … This works by detecting if the filename has a .xxxx (four digit number) before the file extension." (image formats page). That is about UDIM tiles, not frames. The render output rule "The # characters are replaced by the frame number, and used to define zero padding" (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html) and `RenderSettings.frame_path` are about writing renders, not reading images.

**Not covered:** everything in the "Not in the sources" list above.

**Observed — hand test 4, 2026-11-15, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).** The tests, their exact commands and every script are in `Audits/Blender Research/Colour spaces and image sequences/Hand tests — 5.3.md`; what follows stands on its own.

*What was done.* Pictures were written by a small program of their own (not Blender), each 8 × 8 pixels of one flat colour whose red value is the picture's own number in a list and whose green value names its folder, so that a rendered pixel says which file it came from. The folders: `A` — `a_1.png` … `a_12.png`; `B` — `b_0001.png` … `b_0012.png` and `b_10000.png`; `W` — `w_1.png` … `w_12.png`, `w_01.png` … `w_09.png` and `w_0001.png` … `w_0012.png` side by side; `C` — `c_0001.png` … `c_0009.png`, `c_10.png`, `c_00011.png`, `c_0012.png`; `O` — `o_998.png`, `o_999.png`, `o_1000.png`, `o_1001.png`; `SHOT` — `shot1_0001.png`, `shot2_0001.png` … `shot2_0005.png`, `shot3_0001.png`; `VER` — `s_0001_v2.png` … `s_0005_v2.png` and `s_0001_v1.png`, `s_0001_v3.png`, `s_0001_v4.png`, `s_0001_v5.png`; `LEAD` — `0001_plate.png` … `0005_plate.png`; `DOT` — `frame.0001.png` … `frame.0005.png`; `PLAIN` — `plain.png`. For each case a script made a file holding an orthographic camera and a plane filling it, with an Emission shader fed by an Image Texture node (Closest) whose image was opened with `bpy.data.images.load(<stored name>)`, its source set to Image Sequence and its colour space to Non-Color; the node's image user had Start 1, Offset 0, Frames 20000, Cyclic off, Auto Refresh off; the file was saved with the ordinary save. The stored names: `n_a` `a_1.png`; `n_b` `b_0001.png`; `n_bhash` `b_####.png`; `n_b5` `b_0005.png`; `n_w1` `w_1.png`; `n_w2` `w_01.png`; `n_w4` `w_0001.png`; `n_c` `c_0001.png`; `n_o` `o_998.png`; `n_shot` `shot2_0001.png`; `n_ver` `s_0001_v2.png`; `n_lead` `0001_plate.png`; `n_dot` `frame.0001.png`; `n_plain` `plain.png`. Each file was rendered from the command line, scene frames 1 to 13 (`-s 1 -e 13 -a`; Cycles on the CPU, 1 sample, 8 × 8, EXR), `n_b` also 9999 to 10001 and `n_o` 997 to 1002 instead, and a script opened every render with its colour space set to Non-Color, printed three pixels of the middle row and named the picture they belong to (`MAGENTA` for (1, 0, 1)). A script also asked each file `filepath_from_user` with the node's image user after writing `frame_current` (question 5's route A). Separately, Blender's Open Image operator, `bpy.ops.image.open(filepath=…, directory=…, files=[…], relative_path=False, use_sequence_detection=…)`, was given whole folders and one file, with no images in the file before each case.

*What came back.* The `#` name, at the build:
```
BUILD n_bhash bpy.data.images.load of the name with # refused: RuntimeError("Error: Cannot read '…/pics/B/b_####.png': No such file or directory\n")
BUILD n_bhash opened b_0001.png and set filepath to …/pics/B/b_####.png
```
Every rendered frame of every case:
```
PIX names/n_a/0001.exr | L=(0.0039, 0.0392, 0.5020) A/a_1.png | M=(0.0039, 0.0392, 0.5020) A/a_1.png | R=(0.0039, 0.0392, 0.5020) A/a_1.png
PIX names/n_a/0002.exr | L=(0.0078, 0.0392, 0.5020) A/a_2.png | M=(0.0078, 0.0392, 0.5020) A/a_2.png | R=(0.0078, 0.0392, 0.5020) A/a_2.png
PIX names/n_a/0003.exr | L=(0.0118, 0.0392, 0.5020) A/a_3.png | M=(0.0118, 0.0392, 0.5020) A/a_3.png | R=(0.0118, 0.0392, 0.5020) A/a_3.png
PIX names/n_a/0004.exr | L=(0.0157, 0.0392, 0.5020) A/a_4.png | M=(0.0157, 0.0392, 0.5020) A/a_4.png | R=(0.0157, 0.0392, 0.5020) A/a_4.png
PIX names/n_a/0005.exr | L=(0.0196, 0.0392, 0.5020) A/a_5.png | M=(0.0196, 0.0392, 0.5020) A/a_5.png | R=(0.0196, 0.0392, 0.5020) A/a_5.png
PIX names/n_a/0006.exr | L=(0.0235, 0.0392, 0.5020) A/a_6.png | M=(0.0235, 0.0392, 0.5020) A/a_6.png | R=(0.0235, 0.0392, 0.5020) A/a_6.png
PIX names/n_a/0007.exr | L=(0.0275, 0.0392, 0.5020) A/a_7.png | M=(0.0275, 0.0392, 0.5020) A/a_7.png | R=(0.0275, 0.0392, 0.5020) A/a_7.png
PIX names/n_a/0008.exr | L=(0.0314, 0.0392, 0.5020) A/a_8.png | M=(0.0314, 0.0392, 0.5020) A/a_8.png | R=(0.0314, 0.0392, 0.5020) A/a_8.png
PIX names/n_a/0009.exr | L=(0.0353, 0.0392, 0.5020) A/a_9.png | M=(0.0353, 0.0392, 0.5020) A/a_9.png | R=(0.0353, 0.0392, 0.5020) A/a_9.png
PIX names/n_a/0010.exr | L=(0.0392, 0.0392, 0.5020) A/a_10.png | M=(0.0392, 0.0392, 0.5020) A/a_10.png | R=(0.0392, 0.0392, 0.5020) A/a_10.png
PIX names/n_a/0011.exr | L=(0.0431, 0.0392, 0.5020) A/a_11.png | M=(0.0431, 0.0392, 0.5020) A/a_11.png | R=(0.0431, 0.0392, 0.5020) A/a_11.png
PIX names/n_a/0012.exr | L=(0.0471, 0.0392, 0.5020) A/a_12.png | M=(0.0471, 0.0392, 0.5020) A/a_12.png | R=(0.0471, 0.0392, 0.5020) A/a_12.png
PIX names/n_a/0013.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_b/0001.exr | L=(0.0039, 0.0784, 0.5020) B/b_0001.png | M=(0.0039, 0.0784, 0.5020) B/b_0001.png | R=(0.0039, 0.0784, 0.5020) B/b_0001.png
PIX names/n_b/0002.exr | L=(0.0078, 0.0784, 0.5020) B/b_0002.png | M=(0.0078, 0.0784, 0.5020) B/b_0002.png | R=(0.0078, 0.0784, 0.5020) B/b_0002.png
PIX names/n_b/0003.exr | L=(0.0118, 0.0784, 0.5020) B/b_0003.png | M=(0.0118, 0.0784, 0.5020) B/b_0003.png | R=(0.0118, 0.0784, 0.5020) B/b_0003.png
PIX names/n_b/0004.exr | L=(0.0157, 0.0784, 0.5020) B/b_0004.png | M=(0.0157, 0.0784, 0.5020) B/b_0004.png | R=(0.0157, 0.0784, 0.5020) B/b_0004.png
PIX names/n_b/0005.exr | L=(0.0196, 0.0784, 0.5020) B/b_0005.png | M=(0.0196, 0.0784, 0.5020) B/b_0005.png | R=(0.0196, 0.0784, 0.5020) B/b_0005.png
PIX names/n_b/0006.exr | L=(0.0235, 0.0784, 0.5020) B/b_0006.png | M=(0.0235, 0.0784, 0.5020) B/b_0006.png | R=(0.0235, 0.0784, 0.5020) B/b_0006.png
PIX names/n_b/0007.exr | L=(0.0275, 0.0784, 0.5020) B/b_0007.png | M=(0.0275, 0.0784, 0.5020) B/b_0007.png | R=(0.0275, 0.0784, 0.5020) B/b_0007.png
PIX names/n_b/0008.exr | L=(0.0314, 0.0784, 0.5020) B/b_0008.png | M=(0.0314, 0.0784, 0.5020) B/b_0008.png | R=(0.0314, 0.0784, 0.5020) B/b_0008.png
PIX names/n_b/0009.exr | L=(0.0353, 0.0784, 0.5020) B/b_0009.png | M=(0.0353, 0.0784, 0.5020) B/b_0009.png | R=(0.0353, 0.0784, 0.5020) B/b_0009.png
PIX names/n_b/0010.exr | L=(0.0392, 0.0784, 0.5020) B/b_0010.png | M=(0.0392, 0.0784, 0.5020) B/b_0010.png | R=(0.0392, 0.0784, 0.5020) B/b_0010.png
PIX names/n_b/0011.exr | L=(0.0431, 0.0784, 0.5020) B/b_0011.png | M=(0.0431, 0.0784, 0.5020) B/b_0011.png | R=(0.0431, 0.0784, 0.5020) B/b_0011.png
PIX names/n_b/0012.exr | L=(0.0471, 0.0784, 0.5020) B/b_0012.png | M=(0.0471, 0.0784, 0.5020) B/b_0012.png | R=(0.0471, 0.0784, 0.5020) B/b_0012.png
PIX names/n_b/0013.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_b5/0001.exr | L=(0.0039, 0.0784, 0.5020) B/b_0001.png | M=(0.0039, 0.0784, 0.5020) B/b_0001.png | R=(0.0039, 0.0784, 0.5020) B/b_0001.png
PIX names/n_b5/0002.exr | L=(0.0078, 0.0784, 0.5020) B/b_0002.png | M=(0.0078, 0.0784, 0.5020) B/b_0002.png | R=(0.0078, 0.0784, 0.5020) B/b_0002.png
PIX names/n_b5/0003.exr | L=(0.0118, 0.0784, 0.5020) B/b_0003.png | M=(0.0118, 0.0784, 0.5020) B/b_0003.png | R=(0.0118, 0.0784, 0.5020) B/b_0003.png
PIX names/n_b5/0004.exr | L=(0.0157, 0.0784, 0.5020) B/b_0004.png | M=(0.0157, 0.0784, 0.5020) B/b_0004.png | R=(0.0157, 0.0784, 0.5020) B/b_0004.png
PIX names/n_b5/0005.exr | L=(0.0196, 0.0784, 0.5020) B/b_0005.png | M=(0.0196, 0.0784, 0.5020) B/b_0005.png | R=(0.0196, 0.0784, 0.5020) B/b_0005.png
PIX names/n_b5/0006.exr | L=(0.0235, 0.0784, 0.5020) B/b_0006.png | M=(0.0235, 0.0784, 0.5020) B/b_0006.png | R=(0.0235, 0.0784, 0.5020) B/b_0006.png
PIX names/n_b5/0007.exr | L=(0.0275, 0.0784, 0.5020) B/b_0007.png | M=(0.0275, 0.0784, 0.5020) B/b_0007.png | R=(0.0275, 0.0784, 0.5020) B/b_0007.png
PIX names/n_b5/0008.exr | L=(0.0314, 0.0784, 0.5020) B/b_0008.png | M=(0.0314, 0.0784, 0.5020) B/b_0008.png | R=(0.0314, 0.0784, 0.5020) B/b_0008.png
PIX names/n_b5/0009.exr | L=(0.0353, 0.0784, 0.5020) B/b_0009.png | M=(0.0353, 0.0784, 0.5020) B/b_0009.png | R=(0.0353, 0.0784, 0.5020) B/b_0009.png
PIX names/n_b5/0010.exr | L=(0.0392, 0.0784, 0.5020) B/b_0010.png | M=(0.0392, 0.0784, 0.5020) B/b_0010.png | R=(0.0392, 0.0784, 0.5020) B/b_0010.png
PIX names/n_b5/0011.exr | L=(0.0431, 0.0784, 0.5020) B/b_0011.png | M=(0.0431, 0.0784, 0.5020) B/b_0011.png | R=(0.0431, 0.0784, 0.5020) B/b_0011.png
PIX names/n_b5/0012.exr | L=(0.0471, 0.0784, 0.5020) B/b_0012.png | M=(0.0471, 0.0784, 0.5020) B/b_0012.png | R=(0.0471, 0.0784, 0.5020) B/b_0012.png
PIX names/n_b5/0013.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_b_high/09999.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_b_high/10000.exr | L=(0.3922, 0.0784, 0.5020) B/b_10000.png | M=(0.3922, 0.0784, 0.5020) B/b_10000.png | R=(0.3922, 0.0784, 0.5020) B/b_10000.png
PIX names/n_b_high/10001.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_bhash/0001.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_bhash/0002.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_bhash/0003.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_bhash/0004.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_bhash/0005.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_bhash/0006.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_bhash/0007.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_bhash/0008.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_bhash/0009.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_bhash/0010.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_bhash/0011.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_bhash/0012.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_bhash/0013.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_c/0001.exr | L=(0.0039, 0.1569, 0.5020) C/c_0001.png | M=(0.0039, 0.1569, 0.5020) C/c_0001.png | R=(0.0039, 0.1569, 0.5020) C/c_0001.png
PIX names/n_c/0002.exr | L=(0.0078, 0.1569, 0.5020) C/c_0002.png | M=(0.0078, 0.1569, 0.5020) C/c_0002.png | R=(0.0078, 0.1569, 0.5020) C/c_0002.png
PIX names/n_c/0003.exr | L=(0.0118, 0.1569, 0.5020) C/c_0003.png | M=(0.0118, 0.1569, 0.5020) C/c_0003.png | R=(0.0118, 0.1569, 0.5020) C/c_0003.png
PIX names/n_c/0004.exr | L=(0.0157, 0.1569, 0.5020) C/c_0004.png | M=(0.0157, 0.1569, 0.5020) C/c_0004.png | R=(0.0157, 0.1569, 0.5020) C/c_0004.png
PIX names/n_c/0005.exr | L=(0.0196, 0.1569, 0.5020) C/c_0005.png | M=(0.0196, 0.1569, 0.5020) C/c_0005.png | R=(0.0196, 0.1569, 0.5020) C/c_0005.png
PIX names/n_c/0006.exr | L=(0.0235, 0.1569, 0.5020) C/c_0006.png | M=(0.0235, 0.1569, 0.5020) C/c_0006.png | R=(0.0235, 0.1569, 0.5020) C/c_0006.png
PIX names/n_c/0007.exr | L=(0.0275, 0.1569, 0.5020) C/c_0007.png | M=(0.0275, 0.1569, 0.5020) C/c_0007.png | R=(0.0275, 0.1569, 0.5020) C/c_0007.png
PIX names/n_c/0008.exr | L=(0.0314, 0.1569, 0.5020) C/c_0008.png | M=(0.0314, 0.1569, 0.5020) C/c_0008.png | R=(0.0314, 0.1569, 0.5020) C/c_0008.png
PIX names/n_c/0009.exr | L=(0.0353, 0.1569, 0.5020) C/c_0009.png | M=(0.0353, 0.1569, 0.5020) C/c_0009.png | R=(0.0353, 0.1569, 0.5020) C/c_0009.png
PIX names/n_c/0010.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_c/0011.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_c/0012.exr | L=(0.0471, 0.1569, 0.5020) C/c_0012.png | M=(0.0471, 0.1569, 0.5020) C/c_0012.png | R=(0.0471, 0.1569, 0.5020) C/c_0012.png
PIX names/n_c/0013.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_dot/0001.exr | L=(0.0039, 0.3529, 0.5020) DOT/frame.0001.png | M=(0.0039, 0.3529, 0.5020) DOT/frame.0001.png | R=(0.0039, 0.3529, 0.5020) DOT/frame.0001.png
PIX names/n_dot/0002.exr | L=(0.0078, 0.3529, 0.5020) DOT/frame.0002.png | M=(0.0078, 0.3529, 0.5020) DOT/frame.0002.png | R=(0.0078, 0.3529, 0.5020) DOT/frame.0002.png
PIX names/n_dot/0003.exr | L=(0.0118, 0.3529, 0.5020) DOT/frame.0003.png | M=(0.0118, 0.3529, 0.5020) DOT/frame.0003.png | R=(0.0118, 0.3529, 0.5020) DOT/frame.0003.png
PIX names/n_dot/0004.exr | L=(0.0157, 0.3529, 0.5020) DOT/frame.0004.png | M=(0.0157, 0.3529, 0.5020) DOT/frame.0004.png | R=(0.0157, 0.3529, 0.5020) DOT/frame.0004.png
PIX names/n_dot/0005.exr | L=(0.0196, 0.3529, 0.5020) DOT/frame.0005.png | M=(0.0196, 0.3529, 0.5020) DOT/frame.0005.png | R=(0.0196, 0.3529, 0.5020) DOT/frame.0005.png
PIX names/n_dot/0006.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_dot/0007.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_dot/0008.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_dot/0009.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_dot/0010.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_dot/0011.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_dot/0012.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_dot/0013.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_lead/0001.exr | L=(0.0039, 0.3137, 0.5020) LEAD/0001_plate.png | M=(0.0039, 0.3137, 0.5020) LEAD/0001_plate.png | R=(0.0039, 0.3137, 0.5020) LEAD/0001_plate.png
PIX names/n_lead/0002.exr | L=(0.0078, 0.3137, 0.5020) LEAD/0002_plate.png | M=(0.0078, 0.3137, 0.5020) LEAD/0002_plate.png | R=(0.0078, 0.3137, 0.5020) LEAD/0002_plate.png
PIX names/n_lead/0003.exr | L=(0.0118, 0.3137, 0.5020) LEAD/0003_plate.png | M=(0.0118, 0.3137, 0.5020) LEAD/0003_plate.png | R=(0.0118, 0.3137, 0.5020) LEAD/0003_plate.png
PIX names/n_lead/0004.exr | L=(0.0157, 0.3137, 0.5020) LEAD/0004_plate.png | M=(0.0157, 0.3137, 0.5020) LEAD/0004_plate.png | R=(0.0157, 0.3137, 0.5020) LEAD/0004_plate.png
PIX names/n_lead/0005.exr | L=(0.0196, 0.3137, 0.5020) LEAD/0005_plate.png | M=(0.0196, 0.3137, 0.5020) LEAD/0005_plate.png | R=(0.0196, 0.3137, 0.5020) LEAD/0005_plate.png
PIX names/n_lead/0006.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_lead/0007.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_lead/0008.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_lead/0009.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_lead/0010.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_lead/0011.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_lead/0012.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_lead/0013.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_o/0997.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_o/0998.exr | L=(0.0039, 0.1961, 0.5020) O/o_998.png | M=(0.0039, 0.1961, 0.5020) O/o_998.png | R=(0.0039, 0.1961, 0.5020) O/o_998.png
PIX names/n_o/0999.exr | L=(0.0078, 0.1961, 0.5020) O/o_999.png | M=(0.0078, 0.1961, 0.5020) O/o_999.png | R=(0.0078, 0.1961, 0.5020) O/o_999.png
PIX names/n_o/1000.exr | L=(0.0118, 0.1961, 0.5020) O/o_1000.png | M=(0.0118, 0.1961, 0.5020) O/o_1000.png | R=(0.0118, 0.1961, 0.5020) O/o_1000.png
PIX names/n_o/1001.exr | L=(0.0157, 0.1961, 0.5020) O/o_1001.png | M=(0.0157, 0.1961, 0.5020) O/o_1001.png | R=(0.0157, 0.1961, 0.5020) O/o_1001.png
PIX names/n_o/1002.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_plain/0001.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_plain/0002.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_plain/0003.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_plain/0004.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_plain/0005.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_plain/0006.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_plain/0007.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_plain/0008.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_plain/0009.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_plain/0010.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_plain/0011.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_plain/0012.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_plain/0013.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_shot/0001.exr | L=(0.0431, 0.2353, 0.5020) SHOT/shot2_0001.png | M=(0.0431, 0.2353, 0.5020) SHOT/shot2_0001.png | R=(0.0431, 0.2353, 0.5020) SHOT/shot2_0001.png
PIX names/n_shot/0002.exr | L=(0.0471, 0.2353, 0.5020) SHOT/shot2_0002.png | M=(0.0471, 0.2353, 0.5020) SHOT/shot2_0002.png | R=(0.0471, 0.2353, 0.5020) SHOT/shot2_0002.png
PIX names/n_shot/0003.exr | L=(0.0510, 0.2353, 0.5020) SHOT/shot2_0003.png | M=(0.0510, 0.2353, 0.5020) SHOT/shot2_0003.png | R=(0.0510, 0.2353, 0.5020) SHOT/shot2_0003.png
PIX names/n_shot/0004.exr | L=(0.0549, 0.2353, 0.5020) SHOT/shot2_0004.png | M=(0.0549, 0.2353, 0.5020) SHOT/shot2_0004.png | R=(0.0549, 0.2353, 0.5020) SHOT/shot2_0004.png
PIX names/n_shot/0005.exr | L=(0.0588, 0.2353, 0.5020) SHOT/shot2_0005.png | M=(0.0588, 0.2353, 0.5020) SHOT/shot2_0005.png | R=(0.0588, 0.2353, 0.5020) SHOT/shot2_0005.png
PIX names/n_shot/0006.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_shot/0007.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_shot/0008.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_shot/0009.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_shot/0010.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_shot/0011.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_shot/0012.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_shot/0013.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_ver/0001.exr | L=(0.0039, 0.2745, 0.5020) VER/s_0001_v1.png | M=(0.0039, 0.2745, 0.5020) VER/s_0001_v1.png | R=(0.0039, 0.2745, 0.5020) VER/s_0001_v1.png
PIX names/n_ver/0002.exr | L=(0.0431, 0.2745, 0.5020) VER/s_0001_v2.png | M=(0.0431, 0.2745, 0.5020) VER/s_0001_v2.png | R=(0.0431, 0.2745, 0.5020) VER/s_0001_v2.png
PIX names/n_ver/0003.exr | L=(0.0118, 0.2745, 0.5020) VER/s_0001_v3.png | M=(0.0118, 0.2745, 0.5020) VER/s_0001_v3.png | R=(0.0118, 0.2745, 0.5020) VER/s_0001_v3.png
PIX names/n_ver/0004.exr | L=(0.0157, 0.2745, 0.5020) VER/s_0001_v4.png | M=(0.0157, 0.2745, 0.5020) VER/s_0001_v4.png | R=(0.0157, 0.2745, 0.5020) VER/s_0001_v4.png
PIX names/n_ver/0005.exr | L=(0.0196, 0.2745, 0.5020) VER/s_0001_v5.png | M=(0.0196, 0.2745, 0.5020) VER/s_0001_v5.png | R=(0.0196, 0.2745, 0.5020) VER/s_0001_v5.png
PIX names/n_ver/0006.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_ver/0007.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_ver/0008.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_ver/0009.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_ver/0010.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_ver/0011.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_ver/0012.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_ver/0013.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_w1/0001.exr | L=(0.0039, 0.1176, 0.5020) W/w_1.png | M=(0.0039, 0.1176, 0.5020) W/w_1.png | R=(0.0039, 0.1176, 0.5020) W/w_1.png
PIX names/n_w1/0002.exr | L=(0.0078, 0.1176, 0.5020) W/w_2.png | M=(0.0078, 0.1176, 0.5020) W/w_2.png | R=(0.0078, 0.1176, 0.5020) W/w_2.png
PIX names/n_w1/0003.exr | L=(0.0118, 0.1176, 0.5020) W/w_3.png | M=(0.0118, 0.1176, 0.5020) W/w_3.png | R=(0.0118, 0.1176, 0.5020) W/w_3.png
PIX names/n_w1/0004.exr | L=(0.0157, 0.1176, 0.5020) W/w_4.png | M=(0.0157, 0.1176, 0.5020) W/w_4.png | R=(0.0157, 0.1176, 0.5020) W/w_4.png
PIX names/n_w1/0005.exr | L=(0.0196, 0.1176, 0.5020) W/w_5.png | M=(0.0196, 0.1176, 0.5020) W/w_5.png | R=(0.0196, 0.1176, 0.5020) W/w_5.png
PIX names/n_w1/0006.exr | L=(0.0235, 0.1176, 0.5020) W/w_6.png | M=(0.0235, 0.1176, 0.5020) W/w_6.png | R=(0.0235, 0.1176, 0.5020) W/w_6.png
PIX names/n_w1/0007.exr | L=(0.0275, 0.1176, 0.5020) W/w_7.png | M=(0.0275, 0.1176, 0.5020) W/w_7.png | R=(0.0275, 0.1176, 0.5020) W/w_7.png
PIX names/n_w1/0008.exr | L=(0.0314, 0.1176, 0.5020) W/w_8.png | M=(0.0314, 0.1176, 0.5020) W/w_8.png | R=(0.0314, 0.1176, 0.5020) W/w_8.png
PIX names/n_w1/0009.exr | L=(0.0353, 0.1176, 0.5020) W/w_9.png | M=(0.0353, 0.1176, 0.5020) W/w_9.png | R=(0.0353, 0.1176, 0.5020) W/w_9.png
PIX names/n_w1/0010.exr | L=(0.0392, 0.1176, 0.5020) W/w_10.png | M=(0.0392, 0.1176, 0.5020) W/w_10.png | R=(0.0392, 0.1176, 0.5020) W/w_10.png
PIX names/n_w1/0011.exr | L=(0.0431, 0.1176, 0.5020) W/w_11.png | M=(0.0431, 0.1176, 0.5020) W/w_11.png | R=(0.0431, 0.1176, 0.5020) W/w_11.png
PIX names/n_w1/0012.exr | L=(0.0471, 0.1176, 0.5020) W/w_12.png | M=(0.0471, 0.1176, 0.5020) W/w_12.png | R=(0.0471, 0.1176, 0.5020) W/w_12.png
PIX names/n_w1/0013.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_w2/0001.exr | L=(0.0824, 0.1176, 0.5020) W/w_01.png | M=(0.0824, 0.1176, 0.5020) W/w_01.png | R=(0.0824, 0.1176, 0.5020) W/w_01.png
PIX names/n_w2/0002.exr | L=(0.0863, 0.1176, 0.5020) W/w_02.png | M=(0.0863, 0.1176, 0.5020) W/w_02.png | R=(0.0863, 0.1176, 0.5020) W/w_02.png
PIX names/n_w2/0003.exr | L=(0.0902, 0.1176, 0.5020) W/w_03.png | M=(0.0902, 0.1176, 0.5020) W/w_03.png | R=(0.0902, 0.1176, 0.5020) W/w_03.png
PIX names/n_w2/0004.exr | L=(0.0941, 0.1176, 0.5020) W/w_04.png | M=(0.0941, 0.1176, 0.5020) W/w_04.png | R=(0.0941, 0.1176, 0.5020) W/w_04.png
PIX names/n_w2/0005.exr | L=(0.0980, 0.1176, 0.5020) W/w_05.png | M=(0.0980, 0.1176, 0.5020) W/w_05.png | R=(0.0980, 0.1176, 0.5020) W/w_05.png
PIX names/n_w2/0006.exr | L=(0.1020, 0.1176, 0.5020) W/w_06.png | M=(0.1020, 0.1176, 0.5020) W/w_06.png | R=(0.1020, 0.1176, 0.5020) W/w_06.png
PIX names/n_w2/0007.exr | L=(0.1059, 0.1176, 0.5020) W/w_07.png | M=(0.1059, 0.1176, 0.5020) W/w_07.png | R=(0.1059, 0.1176, 0.5020) W/w_07.png
PIX names/n_w2/0008.exr | L=(0.1098, 0.1176, 0.5020) W/w_08.png | M=(0.1098, 0.1176, 0.5020) W/w_08.png | R=(0.1098, 0.1176, 0.5020) W/w_08.png
PIX names/n_w2/0009.exr | L=(0.1137, 0.1176, 0.5020) W/w_09.png | M=(0.1137, 0.1176, 0.5020) W/w_09.png | R=(0.1137, 0.1176, 0.5020) W/w_09.png
PIX names/n_w2/0010.exr | L=(0.0392, 0.1176, 0.5020) W/w_10.png | M=(0.0392, 0.1176, 0.5020) W/w_10.png | R=(0.0392, 0.1176, 0.5020) W/w_10.png
PIX names/n_w2/0011.exr | L=(0.0431, 0.1176, 0.5020) W/w_11.png | M=(0.0431, 0.1176, 0.5020) W/w_11.png | R=(0.0431, 0.1176, 0.5020) W/w_11.png
PIX names/n_w2/0012.exr | L=(0.0471, 0.1176, 0.5020) W/w_12.png | M=(0.0471, 0.1176, 0.5020) W/w_12.png | R=(0.0471, 0.1176, 0.5020) W/w_12.png
PIX names/n_w2/0013.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX names/n_w4/0001.exr | L=(0.1608, 0.1176, 0.5020) W/w_0001.png | M=(0.1608, 0.1176, 0.5020) W/w_0001.png | R=(0.1608, 0.1176, 0.5020) W/w_0001.png
PIX names/n_w4/0002.exr | L=(0.1647, 0.1176, 0.5020) W/w_0002.png | M=(0.1647, 0.1176, 0.5020) W/w_0002.png | R=(0.1647, 0.1176, 0.5020) W/w_0002.png
PIX names/n_w4/0003.exr | L=(0.1686, 0.1176, 0.5020) W/w_0003.png | M=(0.1686, 0.1176, 0.5020) W/w_0003.png | R=(0.1686, 0.1176, 0.5020) W/w_0003.png
PIX names/n_w4/0004.exr | L=(0.1725, 0.1176, 0.5020) W/w_0004.png | M=(0.1725, 0.1176, 0.5020) W/w_0004.png | R=(0.1725, 0.1176, 0.5020) W/w_0004.png
PIX names/n_w4/0005.exr | L=(0.1765, 0.1176, 0.5020) W/w_0005.png | M=(0.1765, 0.1176, 0.5020) W/w_0005.png | R=(0.1765, 0.1176, 0.5020) W/w_0005.png
PIX names/n_w4/0006.exr | L=(0.1804, 0.1176, 0.5020) W/w_0006.png | M=(0.1804, 0.1176, 0.5020) W/w_0006.png | R=(0.1804, 0.1176, 0.5020) W/w_0006.png
PIX names/n_w4/0007.exr | L=(0.1843, 0.1176, 0.5020) W/w_0007.png | M=(0.1843, 0.1176, 0.5020) W/w_0007.png | R=(0.1843, 0.1176, 0.5020) W/w_0007.png
PIX names/n_w4/0008.exr | L=(0.1882, 0.1176, 0.5020) W/w_0008.png | M=(0.1882, 0.1176, 0.5020) W/w_0008.png | R=(0.1882, 0.1176, 0.5020) W/w_0008.png
PIX names/n_w4/0009.exr | L=(0.1922, 0.1176, 0.5020) W/w_0009.png | M=(0.1922, 0.1176, 0.5020) W/w_0009.png | R=(0.1922, 0.1176, 0.5020) W/w_0009.png
PIX names/n_w4/0010.exr | L=(0.1961, 0.1176, 0.5020) W/w_0010.png | M=(0.1961, 0.1176, 0.5020) W/w_0010.png | R=(0.1961, 0.1176, 0.5020) W/w_0010.png
PIX names/n_w4/0011.exr | L=(0.2000, 0.1176, 0.5020) W/w_0011.png | M=(0.2000, 0.1176, 0.5020) W/w_0011.png | R=(0.2000, 0.1176, 0.5020) W/w_0011.png
PIX names/n_w4/0012.exr | L=(0.2039, 0.1176, 0.5020) W/w_0012.png | M=(0.2039, 0.1176, 0.5020) W/w_0012.png | R=(0.2039, 0.1176, 0.5020) W/w_0012.png
PIX names/n_w4/0013.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
```
What a render printed for a frame whose file is not there (`n_a`, frame 13):
```
00:00.384  cycles           | WARNING Image file …/pics/A/a_13.png does not exist.
00:00.385  cycles           | ERROR Failed to load 1 image files
```
The file name `filepath_from_user` gives for each `frame_current` written, for some of the cases (the others follow the same pattern with their own names):
```
READ route A M frame_current set to -1, reads back -1 -> …/pics/A/a_0.png
READ route A M frame_current set to 0, reads back 0 -> …/pics/A/a_0.png
READ route A M frame_current set to 1, reads back 1 -> …/pics/A/a_1.png
READ route A M frame_current set to 2, reads back 2 -> …/pics/A/a_2.png
READ route A M frame_current set to 5, reads back 5 -> …/pics/A/a_5.png
READ route A M frame_current set to 8, reads back 8 -> …/pics/A/a_8.png
READ route A M frame_current set to 13, reads back 13 -> …/pics/A/a_13.png
READ route A M frame_current set to 20, reads back 20 -> …/pics/A/a_20.png
READ route A M frame_current set to 21, reads back 21 -> …/pics/A/a_21.png
```
```
READ route A M frame_current set to -1, reads back -1 -> …/pics/W/w_0.png
READ route A M frame_current set to 0, reads back 0 -> …/pics/W/w_0.png
READ route A M frame_current set to 1, reads back 1 -> …/pics/W/w_1.png
READ route A M frame_current set to 2, reads back 2 -> …/pics/W/w_2.png
READ route A M frame_current set to 5, reads back 5 -> …/pics/W/w_5.png
READ route A M frame_current set to 8, reads back 8 -> …/pics/W/w_8.png
READ route A M frame_current set to 13, reads back 13 -> …/pics/W/w_13.png
READ route A M frame_current set to 20, reads back 20 -> …/pics/W/w_20.png
READ route A M frame_current set to 21, reads back 21 -> …/pics/W/w_21.png
```
```
READ route A M frame_current set to -1, reads back -1 -> …/pics/O/o_000.png
READ route A M frame_current set to 0, reads back 0 -> …/pics/O/o_000.png
READ route A M frame_current set to 1, reads back 1 -> …/pics/O/o_001.png
READ route A M frame_current set to 2, reads back 2 -> …/pics/O/o_002.png
READ route A M frame_current set to 5, reads back 5 -> …/pics/O/o_005.png
READ route A M frame_current set to 8, reads back 8 -> …/pics/O/o_008.png
READ route A M frame_current set to 13, reads back 13 -> …/pics/O/o_013.png
READ route A M frame_current set to 20, reads back 20 -> …/pics/O/o_020.png
READ route A M frame_current set to 21, reads back 21 -> …/pics/O/o_021.png
```
```
READ route A M frame_current set to -1, reads back -1 -> …/pics/B/b_####.png
READ route A M frame_current set to 0, reads back 0 -> …/pics/B/b_####.png
READ route A M frame_current set to 1, reads back 1 -> …/pics/B/b_####1.png
READ route A M frame_current set to 2, reads back 2 -> …/pics/B/b_####2.png
READ route A M frame_current set to 5, reads back 5 -> …/pics/B/b_####5.png
READ route A M frame_current set to 8, reads back 8 -> …/pics/B/b_####8.png
READ route A M frame_current set to 13, reads back 13 -> …/pics/B/b_####13.png
READ route A M frame_current set to 20, reads back 20 -> …/pics/B/b_####20.png
READ route A M frame_current set to 21, reads back 21 -> …/pics/B/b_####21.png
```
```
READ route A M frame_current set to -1, reads back -1 -> …/pics/VER/s_0001_v0.png
READ route A M frame_current set to 0, reads back 0 -> …/pics/VER/s_0001_v0.png
READ route A M frame_current set to 1, reads back 1 -> …/pics/VER/s_0001_v1.png
READ route A M frame_current set to 2, reads back 2 -> …/pics/VER/s_0001_v2.png
READ route A M frame_current set to 5, reads back 5 -> …/pics/VER/s_0001_v5.png
READ route A M frame_current set to 8, reads back 8 -> …/pics/VER/s_0001_v8.png
READ route A M frame_current set to 13, reads back 13 -> …/pics/VER/s_0001_v13.png
READ route A M frame_current set to 20, reads back 20 -> …/pics/VER/s_0001_v20.png
READ route A M frame_current set to 21, reads back 21 -> …/pics/VER/s_0001_v21.png
```
```
READ route A M frame_current set to -1, reads back -1 -> …/pics/PLAIN/plain.png
READ route A M frame_current set to 0, reads back 0 -> …/pics/PLAIN/plain.png
READ route A M frame_current set to 1, reads back 1 -> …/pics/PLAIN/plain1.png
READ route A M frame_current set to 2, reads back 2 -> …/pics/PLAIN/plain2.png
READ route A M frame_current set to 5, reads back 5 -> …/pics/PLAIN/plain5.png
READ route A M frame_current set to 8, reads back 8 -> …/pics/PLAIN/plain8.png
READ route A M frame_current set to 13, reads back 13 -> …/pics/PLAIN/plain13.png
READ route A M frame_current set to 20, reads back 20 -> …/pics/PLAIN/plain20.png
READ route A M frame_current set to 21, reads back 21 -> …/pics/PLAIN/plain21.png
```
The Open Image operator, case by case (for each new image: its name, file path, source, `frame_duration`, and the file names `filepath_from_user` gave through a fresh image user with Start 1, Offset 0, Frames 20000 at `frame_current` 1, 2, 10 and 13):
```
OPEN version 5.3.0
OPEN B, all 13 files, detection on | result {'FINISHED'} | new images: 1
OPEN    'b_0001.png' filepath …/pics/B/b_0001.png source SEQUENCE frame_duration 1 | from_user ['1:b_0001.png', '2:b_0002.png', '10:b_0010.png', '13:b_0013.png']
OPEN B, all 13 files, detection off | result {'FINISHED'} | new images: 13
OPEN    'b_0001.png' filepath …/pics/B/b_0001.png source FILE frame_duration 1 | from_user ['1:b_0001.png', '2:b_0001.png', '10:b_0001.png', '13:b_0001.png']
OPEN    'b_0002.png' filepath …/pics/B/b_0002.png source FILE frame_duration 1 | from_user ['1:b_0002.png', '2:b_0002.png', '10:b_0002.png', '13:b_0002.png']
OPEN    'b_0003.png' filepath …/pics/B/b_0003.png source FILE frame_duration 1 | from_user ['1:b_0003.png', '2:b_0003.png', '10:b_0003.png', '13:b_0003.png']
OPEN    'b_0004.png' filepath …/pics/B/b_0004.png source FILE frame_duration 1 | from_user ['1:b_0004.png', '2:b_0004.png', '10:b_0004.png', '13:b_0004.png']
OPEN    'b_0005.png' filepath …/pics/B/b_0005.png source FILE frame_duration 1 | from_user ['1:b_0005.png', '2:b_0005.png', '10:b_0005.png', '13:b_0005.png']
OPEN    'b_0006.png' filepath …/pics/B/b_0006.png source FILE frame_duration 1 | from_user ['1:b_0006.png', '2:b_0006.png', '10:b_0006.png', '13:b_0006.png']
OPEN    'b_0007.png' filepath …/pics/B/b_0007.png source FILE frame_duration 1 | from_user ['1:b_0007.png', '2:b_0007.png', '10:b_0007.png', '13:b_0007.png']
OPEN    'b_0008.png' filepath …/pics/B/b_0008.png source FILE frame_duration 1 | from_user ['1:b_0008.png', '2:b_0008.png', '10:b_0008.png', '13:b_0008.png']
OPEN    'b_0009.png' filepath …/pics/B/b_0009.png source FILE frame_duration 1 | from_user ['1:b_0009.png', '2:b_0009.png', '10:b_0009.png', '13:b_0009.png']
OPEN    'b_0010.png' filepath …/pics/B/b_0010.png source FILE frame_duration 1 | from_user ['1:b_0010.png', '2:b_0010.png', '10:b_0010.png', '13:b_0010.png']
OPEN    'b_0011.png' filepath …/pics/B/b_0011.png source FILE frame_duration 1 | from_user ['1:b_0011.png', '2:b_0011.png', '10:b_0011.png', '13:b_0011.png']
OPEN    'b_0012.png' filepath …/pics/B/b_0012.png source FILE frame_duration 1 | from_user ['1:b_0012.png', '2:b_0012.png', '10:b_0012.png', '13:b_0012.png']
OPEN    'b_10000.png' filepath …/pics/B/b_10000.png source FILE frame_duration 1 | from_user ['1:b_10000.png', '2:b_10000.png', '10:b_10000.png', '13:b_10000.png']
OPEN B, b_0005.png alone, detection on | result {'FINISHED'} | new images: 1
OPEN    'b_0005.png' filepath …/pics/B/b_0005.png source FILE frame_duration 1 | from_user ['1:b_0005.png', '2:b_0005.png', '10:b_0005.png', '13:b_0005.png']
OPEN A, all 12 files, detection on | result {'FINISHED'} | new images: 1
OPEN    'a_1.png' filepath …/pics/A/a_1.png source SEQUENCE frame_duration 1 | from_user ['1:a_1.png', '2:a_2.png', '10:a_10.png', '13:a_13.png']
OPEN W, all 33 files, detection on | result {'FINISHED'} | new images: 1
OPEN    'w_0001.png' filepath …/pics/W/w_0001.png source FILE frame_duration 1 | from_user ['1:w_0001.png', '2:w_0001.png', '10:w_0001.png', '13:w_0001.png']
OPEN C, all 12 files, detection on | result {'FINISHED'} | new images: 1
OPEN    'c_0001.png' filepath …/pics/C/c_0001.png source SEQUENCE frame_duration 1 | from_user ['1:c_0001.png', '2:c_0002.png', '10:c_0010.png', '13:c_0013.png']
OPEN SHOT, all 7 files, detection on | result {'FINISHED'} | new images: 3
OPEN    'shot1_0001.png' filepath …/pics/SHOT/shot1_0001.png source FILE frame_duration 1 | from_user ['1:shot1_0001.png', '2:shot1_0001.png', '10:shot1_0001.png', '13:shot1_0001.png']
OPEN    'shot2_0001.png' filepath …/pics/SHOT/shot2_0001.png source SEQUENCE frame_duration 1 | from_user ['1:shot2_0001.png', '2:shot2_0002.png', '10:shot2_0010.png', '13:shot2_0013.png']
OPEN    'shot3_0001.png' filepath …/pics/SHOT/shot3_0001.png source FILE frame_duration 1 | from_user ['1:shot3_0001.png', '2:shot3_0001.png', '10:shot3_0001.png', '13:shot3_0001.png']
OPEN VER, all 9 files, detection on | result {'FINISHED'} | new images: 5
OPEN    's_0001_v1.png' filepath …/pics/VER/s_0001_v1.png source SEQUENCE frame_duration 1 | from_user ['1:s_0001_v1.png', '2:s_0001_v2.png', '10:s_0001_v10.png', '13:s_0001_v13.png']
OPEN    's_0002_v2.png' filepath …/pics/VER/s_0002_v2.png source FILE frame_duration 1 | from_user ['1:s_0002_v2.png', '2:s_0002_v2.png', '10:s_0002_v2.png', '13:s_0002_v2.png']
OPEN    's_0003_v2.png' filepath …/pics/VER/s_0003_v2.png source FILE frame_duration 1 | from_user ['1:s_0003_v2.png', '2:s_0003_v2.png', '10:s_0003_v2.png', '13:s_0003_v2.png']
OPEN    's_0004_v2.png' filepath …/pics/VER/s_0004_v2.png source FILE frame_duration 1 | from_user ['1:s_0004_v2.png', '2:s_0004_v2.png', '10:s_0004_v2.png', '13:s_0004_v2.png']
OPEN    's_0005_v2.png' filepath …/pics/VER/s_0005_v2.png source FILE frame_duration 1 | from_user ['1:s_0005_v2.png', '2:s_0005_v2.png', '10:s_0005_v2.png', '13:s_0005_v2.png']
```

*Not covered by this test:* names that differ only in capitals; the window's File Browser; a sequence stored with a path relative to the file.

**Outside the interface:** The sources do not say. The detection described above belongs to opening files with the file browser or the open operator, which has the `use_sequence_detection` parameter. The sources say nothing on how frames are found when a render runs without the interface. **Update 2026-11-15 — observed** (hand test 4, Creator's Mac, Blender 5.3.0): the command-line renders (`-s … -e … -a`) showed the files quoted under *Observed*, and Blender's Open Image operator ran in a Blender with no window.

**Operating system:** The sources do not say. In particular they do not say whether matching names is case-sensitive on some systems and not on others. **Update 2026-11-15 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open. Names that differ only in capitals were not tried.

---

## 4. Which file of an image sequence does a render use at a given scene frame, taking into account the image's own frame settings?

**Label:** documented (which settings exist on `ImageUser`, `ShaderNodeTexImage`, and `Image`, and what the `Image Texture` node and `ImageUser` fields mean); **inferred** (how those settings combine into a frame); **not in the sources** (the exact rule, `Frames = 0` and negative `Offset` behaviour, `Cyclic` with `Offset` and `Frames`, frames before the start, and the link from a frame position to a file name). **Update 2026-11-15:** observed by hand tests 5 and 6 on the Creator's Mac — see *Observed* below.

**Ambiguity in the question.** "The image's own frame settings" can be read two ways. The image data-block itself has only one frame setting quoted here, `frame_duration`, and it is read-only. The start, offset, length and looping settings belong to whatever uses the image, such as an Image Texture node, a compositor Image node, or the Image Editor (whether other render-relevant types also carry `ImageUser` was not listed from the `ImageUser` search). I answer for the user's settings, because those are the ones a render goes by (see the first quote). The other reading, the Image data-block alone, has no start or offset to account for.

**What the sources say**

The Image Editor's frame settings do not drive the render:

> "The options below are for preview purposes only; they don't affect the 3D Viewport or the render. For that, see the Image Texture Node."

— manual, https://docs.blender.org/manual/en/5.3/editors/image/image_settings.html (read 2026-11-08). The same page says the Image Sequence source uses the Movie options: "Image Sequence … For options, see Movie below." (The "Movie" section below on that page is not quoted here.)

The Image Texture node's settings:

> "Frames — How many frames of the Movie-type image (video) to play. Past this point, the video will be paused (unless Cyclic is enabled)."
> "Start Frame — Scene frame at which the video should start playing."
> "Offset — Number of frames to offset the video to an earlier point in time. (Put differently: how many frames at the start of the video to skip.)"
> "Cyclic — Start over after the last frame to create a continuous loop."
> "Auto Refresh — Update the video texture in the 3D Viewport when moving through the timeline."

— manual, https://docs.blender.org/manual/en/5.3/render/shader_nodes/textures/image.html (read 2026-11-08). The compositor Image node documents the same controls except that the fifth field is hyphenated as `Auto-Refresh` rather than `Auto Refresh` (https://docs.blender.org/manual/en/5.3/compositing/types/input/image.html, read 2026-11-08). *(Note — citation check 2026-11-15, Gemini 4 Hydrogen XHigh: corrected the compositor field name spelling to `Auto-Refresh`.)*

In the API, the node holds these settings in "image_user — Parameters defining which layer, pass and frame of the image is displayed (readonly, never None) Type: ImageUser" (https://docs.blender.org/api/5.3/bpy.types.ShaderNodeTexImage.html, read 2026-11-08). `ImageUser` holds:

> "frame_current — Current frame number in image sequence or movie (in [-1048574, 1048574], default 0)"
> "frame_duration — Number of images of a movie to use (in [0, 1048574], default 0)"
> "frame_offset — Offset the number of the frame to use in the animation (in [-inf, inf], default 0)"
> "frame_start — Global starting frame of the movie/sequence, assuming first picture has a #1 (in [-1048574, 1048574], default 0)"
> "use_auto_refresh — Always refresh image on frame changes (default False)"
> "use_cyclic — Cycle the images in the movie (default False)"

— API, https://docs.blender.org/api/5.3/bpy.types.ImageUser.html (read 2026-11-08). `CompositorNodeImage` does not define `image_user` or `frame_current`; it carries the five frame properties directly: `frame_duration` (`in [0, 1048574]`, default 0), `frame_offset` (`in [-1048574, 1048574]`, default 0), `frame_start` (`in [-1048574, 1048574]`, default 0), `use_auto_refresh` (default False), and `use_cyclic` (default False) (https://docs.blender.org/api/5.3/bpy.types.CompositorNodeImage.html, read 2026-11-08). *(Note — citation check 2026-11-15, Gemini 4 Hydrogen XHigh: corrected from "same frame fields" so `frame_current` is excluded, exact five fields and ranges are stated, and absence of `image_user` is explicitly confirmed.)*

On the image itself: "frame_duration — Duration (in frames) of the image (1 when not a video/sequence) (in [0, inf], default 0, readonly)" (https://docs.blender.org/api/5.3/bpy.types.Image.html, read 2026-11-08).

**Inferred** — each step written out:

1. At the scene frame equal to Start Frame, the first picture of the sequence plays. This comes from "Scene frame at which the video should start playing" together with "assuming first picture has a #1".
2. Each later scene frame moves one picture on. No quoted source states a different playback rate for image sequences.
3. Offset moves the starting point later into the sequence by that many pictures: "how many frames at the start of the video to skip".
4. Put together, for Start S, Offset O and scene frame F, the picture used would be number (F − S + 1 + O), counting the first picture as 1. This is my own arithmetic. The sources state no formula.
5. Past Frames pictures, the last one is held, or with Cyclic the sequence starts again. This comes from "Past this point, the video will be paused (unless Cyclic is enabled)". The sources do not say whether Frames is counted before or after Offset, or how Cyclic interacts with Offset and Frames.

**Not in the sources**

- The formula itself. Whether Offset counts toward Frames. What happens when `Frames` (`frame_duration`) is `0` (the documented default), or when `Offset` is negative. How `Cyclic` interacts with `Offset` and `Frames`. What a render uses at scene frames before Start Frame. (The volume page documents a "Mode" for "before the sequence's first frame", but image users have no such setting and no stated behaviour.)
- What "first picture … #1" refers to: the file numbered 1, or the lowest-numbered file present. So the step from a picture position to a file name is not settled. See question 3.
- What a render does when the file for that frame does not exist.
- Whether the render depends on Auto Refresh. Its documented effect is limited to "the 3D Viewport".

Searches: as listed under question 3, plus `frame_current`, `match_movie_length` (hits from `match_movie_length` were not reported or ruled out here), `Auto Refresh`, `Cyclic`, `frame_set` (whether these extra terms were searched in Cycles' files was not stated). Pages read in full: Image Texture node, compositor Image node, Image Settings (without quoting the "Movie" section below), Image editing, Movie Clips (ruled out: Movie Clip data-blocks are a different kind of data from images), volume properties (ruled out: volumes, not images, quoted only for the "before the first frame" contrast), Node Wrangler "Add Image Sequence" (it repeats the frame_start wording: "Global starting frame of the movie/sequence, assuming first picture has a #1.", https://docs.blender.org/manual/en/5.3/addons/node_wrangler.html), legacy Image/Movie texture (no frame rules). API: `ImageUser`, `Image`, `ShaderNodeTexImage`, `CompositorNodeImage`, `Scene.frame_set`. No 5.3 release note touches image sequences.

**Not covered:** the exact mapping; `Frames = 0` and negative `Offset`; `Cyclic` with `Offset` and `Frames`; before-start behaviour; how missing frames are handled; the file name for a picture position; other image-using nodes besides Image Texture and Compositor Image.

**Observed — hand tests 5 and 6, 2026-11-15, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).** The tests, their exact commands and every script are in `Audits/Blender Research/Colour spaces and image sequences/Hand tests — 5.3.md`; what follows stands on its own.

*What was done.* Pictures were written by a small program of their own (not Blender), each 8 × 8 pixels of one flat colour that says which file it is: folder `Q` holds `q_0000.png` … `q_0020.png`, so that any number from 0 to 20 finds a file and any other does not; `G` holds `g_0001.png` … `g_0008.png` without `g_0004.png`; `D` holds `d_0003.png` … `d_0008.png`. For each case a script made two files, saved with the ordinary save: `_mat`, an orthographic camera and a plane filling it with an Emission shader fed by an Image Texture node (Closest), the settings on the node's image user; and `_comp`, no plane, the compositor's Image node going straight to the compositor's output (`scene.compositing_node_group` and a Group Output node), the settings on the node itself. In both the image was opened with `bpy.data.images.load`, its source set to Image Sequence and its colour space to Non-Color. The cases (stored name; Start, Offset, Frames, Cyclic, Auto Refresh): `k1` `q_0001.png`; 5, 2, 6, off, off. `k2` the same with Cyclic on. `k3` `q_0001.png`; 5, 0, 0, off, off. `k3c` the same with Cyclic on. `k4` `q_0001.png`; 5, −2, 6, off, off. `k5` as `k1` with Auto Refresh on. `k6` `q_0005.png`; 1, 0, 6, off, off. `k7` `g_0001.png`; 1, 0, 8, off, off. `k8` `d_0003.png`; 1, 0, 6, off, off. `k9` (material only): one image feeding two materials on two planes, each filling half the camera, both Start 1, Frames 20, Cyclic off, the left with Offset 0 and the right with Offset 5. Each file was rendered from the command line as an animation (`-s 1 -e 15 -a`; `k6`, `k7`, `k8` to 8; `k9` to 3; Cycles on the CPU, 1 sample, 8 × 8, EXR), and `k1_mat` also one frame at a time (`-f 7`, `-f 12`). A script opened every render with its colour space set to Non-Color, printed three pixels of the middle row (`L`, `M`, `R`) and named the picture they belong to (`MAGENTA` for (1, 0, 1), `BLACK` for (0, 0, 0)).

*What came back.* A new Image Texture node's image user and a new compositor Image node, before the script set them:
```
BUILD a new Image Texture node's image user, untouched: start=1 offset=0 frames=100 cyclic=False auto_refresh=False frame_current=0
BUILD a new compositor Image node, untouched: start=1 offset=0 frames=1 cyclic=False auto_refresh=True | has image_user: False
```
Every frame of every material case, and the two single-frame renders:
```
PIX frames/k1_mat/0001.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_mat/0002.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_mat/0003.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_mat/0004.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_mat/0005.exr | L=(0.4039, 0.4314, 0.5020) Q/q_0003.png | M=(0.4039, 0.4314, 0.5020) Q/q_0003.png | R=(0.4039, 0.4314, 0.5020) Q/q_0003.png
PIX frames/k1_mat/0006.exr | L=(0.4078, 0.4314, 0.5020) Q/q_0004.png | M=(0.4078, 0.4314, 0.5020) Q/q_0004.png | R=(0.4078, 0.4314, 0.5020) Q/q_0004.png
PIX frames/k1_mat/0007.exr | L=(0.4118, 0.4314, 0.5020) Q/q_0005.png | M=(0.4118, 0.4314, 0.5020) Q/q_0005.png | R=(0.4118, 0.4314, 0.5020) Q/q_0005.png
PIX frames/k1_mat/0008.exr | L=(0.4157, 0.4314, 0.5020) Q/q_0006.png | M=(0.4157, 0.4314, 0.5020) Q/q_0006.png | R=(0.4157, 0.4314, 0.5020) Q/q_0006.png
PIX frames/k1_mat/0009.exr | L=(0.4196, 0.4314, 0.5020) Q/q_0007.png | M=(0.4196, 0.4314, 0.5020) Q/q_0007.png | R=(0.4196, 0.4314, 0.5020) Q/q_0007.png
PIX frames/k1_mat/0010.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat/0011.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat/0012.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat/0013.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat/0014.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat/0015.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat_single/0007.exr | L=(0.4118, 0.4314, 0.5020) Q/q_0005.png | M=(0.4118, 0.4314, 0.5020) Q/q_0005.png | R=(0.4118, 0.4314, 0.5020) Q/q_0005.png
PIX frames/k1_mat_single/0012.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k2_mat/0001.exr | L=(0.4118, 0.4314, 0.5020) Q/q_0005.png | M=(0.4118, 0.4314, 0.5020) Q/q_0005.png | R=(0.4118, 0.4314, 0.5020) Q/q_0005.png
PIX frames/k2_mat/0002.exr | L=(0.4157, 0.4314, 0.5020) Q/q_0006.png | M=(0.4157, 0.4314, 0.5020) Q/q_0006.png | R=(0.4157, 0.4314, 0.5020) Q/q_0006.png
PIX frames/k2_mat/0003.exr | L=(0.4196, 0.4314, 0.5020) Q/q_0007.png | M=(0.4196, 0.4314, 0.5020) Q/q_0007.png | R=(0.4196, 0.4314, 0.5020) Q/q_0007.png
PIX frames/k2_mat/0004.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k2_mat/0005.exr | L=(0.4039, 0.4314, 0.5020) Q/q_0003.png | M=(0.4039, 0.4314, 0.5020) Q/q_0003.png | R=(0.4039, 0.4314, 0.5020) Q/q_0003.png
PIX frames/k2_mat/0006.exr | L=(0.4078, 0.4314, 0.5020) Q/q_0004.png | M=(0.4078, 0.4314, 0.5020) Q/q_0004.png | R=(0.4078, 0.4314, 0.5020) Q/q_0004.png
PIX frames/k2_mat/0007.exr | L=(0.4118, 0.4314, 0.5020) Q/q_0005.png | M=(0.4118, 0.4314, 0.5020) Q/q_0005.png | R=(0.4118, 0.4314, 0.5020) Q/q_0005.png
PIX frames/k2_mat/0008.exr | L=(0.4157, 0.4314, 0.5020) Q/q_0006.png | M=(0.4157, 0.4314, 0.5020) Q/q_0006.png | R=(0.4157, 0.4314, 0.5020) Q/q_0006.png
PIX frames/k2_mat/0009.exr | L=(0.4196, 0.4314, 0.5020) Q/q_0007.png | M=(0.4196, 0.4314, 0.5020) Q/q_0007.png | R=(0.4196, 0.4314, 0.5020) Q/q_0007.png
PIX frames/k2_mat/0010.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k2_mat/0011.exr | L=(0.4039, 0.4314, 0.5020) Q/q_0003.png | M=(0.4039, 0.4314, 0.5020) Q/q_0003.png | R=(0.4039, 0.4314, 0.5020) Q/q_0003.png
PIX frames/k2_mat/0012.exr | L=(0.4078, 0.4314, 0.5020) Q/q_0004.png | M=(0.4078, 0.4314, 0.5020) Q/q_0004.png | R=(0.4078, 0.4314, 0.5020) Q/q_0004.png
PIX frames/k2_mat/0013.exr | L=(0.4118, 0.4314, 0.5020) Q/q_0005.png | M=(0.4118, 0.4314, 0.5020) Q/q_0005.png | R=(0.4118, 0.4314, 0.5020) Q/q_0005.png
PIX frames/k2_mat/0014.exr | L=(0.4157, 0.4314, 0.5020) Q/q_0006.png | M=(0.4157, 0.4314, 0.5020) Q/q_0006.png | R=(0.4157, 0.4314, 0.5020) Q/q_0006.png
PIX frames/k2_mat/0015.exr | L=(0.4196, 0.4314, 0.5020) Q/q_0007.png | M=(0.4196, 0.4314, 0.5020) Q/q_0007.png | R=(0.4196, 0.4314, 0.5020) Q/q_0007.png
PIX frames/k3_mat/0001.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_mat/0002.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_mat/0003.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_mat/0004.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_mat/0005.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_mat/0006.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_mat/0007.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_mat/0008.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_mat/0009.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_mat/0010.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_mat/0011.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_mat/0012.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_mat/0013.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_mat/0014.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_mat/0015.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3c_mat/0001.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3c_mat/0002.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3c_mat/0003.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3c_mat/0004.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3c_mat/0005.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3c_mat/0006.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3c_mat/0007.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3c_mat/0008.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3c_mat/0009.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3c_mat/0010.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3c_mat/0011.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3c_mat/0012.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3c_mat/0013.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3c_mat/0014.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3c_mat/0015.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k4_mat/0001.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k4_mat/0002.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k4_mat/0003.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k4_mat/0004.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k4_mat/0005.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k4_mat/0006.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k4_mat/0007.exr | L=(0.3961, 0.4314, 0.5020) Q/q_0001.png | M=(0.3961, 0.4314, 0.5020) Q/q_0001.png | R=(0.3961, 0.4314, 0.5020) Q/q_0001.png
PIX frames/k4_mat/0008.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k4_mat/0009.exr | L=(0.4039, 0.4314, 0.5020) Q/q_0003.png | M=(0.4039, 0.4314, 0.5020) Q/q_0003.png | R=(0.4039, 0.4314, 0.5020) Q/q_0003.png
PIX frames/k4_mat/0010.exr | L=(0.4078, 0.4314, 0.5020) Q/q_0004.png | M=(0.4078, 0.4314, 0.5020) Q/q_0004.png | R=(0.4078, 0.4314, 0.5020) Q/q_0004.png
PIX frames/k4_mat/0011.exr | L=(0.4078, 0.4314, 0.5020) Q/q_0004.png | M=(0.4078, 0.4314, 0.5020) Q/q_0004.png | R=(0.4078, 0.4314, 0.5020) Q/q_0004.png
PIX frames/k4_mat/0012.exr | L=(0.4078, 0.4314, 0.5020) Q/q_0004.png | M=(0.4078, 0.4314, 0.5020) Q/q_0004.png | R=(0.4078, 0.4314, 0.5020) Q/q_0004.png
PIX frames/k4_mat/0013.exr | L=(0.4078, 0.4314, 0.5020) Q/q_0004.png | M=(0.4078, 0.4314, 0.5020) Q/q_0004.png | R=(0.4078, 0.4314, 0.5020) Q/q_0004.png
PIX frames/k4_mat/0014.exr | L=(0.4078, 0.4314, 0.5020) Q/q_0004.png | M=(0.4078, 0.4314, 0.5020) Q/q_0004.png | R=(0.4078, 0.4314, 0.5020) Q/q_0004.png
PIX frames/k4_mat/0015.exr | L=(0.4078, 0.4314, 0.5020) Q/q_0004.png | M=(0.4078, 0.4314, 0.5020) Q/q_0004.png | R=(0.4078, 0.4314, 0.5020) Q/q_0004.png
PIX frames/k5_mat/0001.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k5_mat/0002.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k5_mat/0003.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k5_mat/0004.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k5_mat/0005.exr | L=(0.4039, 0.4314, 0.5020) Q/q_0003.png | M=(0.4039, 0.4314, 0.5020) Q/q_0003.png | R=(0.4039, 0.4314, 0.5020) Q/q_0003.png
PIX frames/k5_mat/0006.exr | L=(0.4078, 0.4314, 0.5020) Q/q_0004.png | M=(0.4078, 0.4314, 0.5020) Q/q_0004.png | R=(0.4078, 0.4314, 0.5020) Q/q_0004.png
PIX frames/k5_mat/0007.exr | L=(0.4118, 0.4314, 0.5020) Q/q_0005.png | M=(0.4118, 0.4314, 0.5020) Q/q_0005.png | R=(0.4118, 0.4314, 0.5020) Q/q_0005.png
PIX frames/k5_mat/0008.exr | L=(0.4157, 0.4314, 0.5020) Q/q_0006.png | M=(0.4157, 0.4314, 0.5020) Q/q_0006.png | R=(0.4157, 0.4314, 0.5020) Q/q_0006.png
PIX frames/k5_mat/0009.exr | L=(0.4196, 0.4314, 0.5020) Q/q_0007.png | M=(0.4196, 0.4314, 0.5020) Q/q_0007.png | R=(0.4196, 0.4314, 0.5020) Q/q_0007.png
PIX frames/k5_mat/0010.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k5_mat/0011.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k5_mat/0012.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k5_mat/0013.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k5_mat/0014.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k5_mat/0015.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k6_mat/0001.exr | L=(0.3961, 0.4314, 0.5020) Q/q_0001.png | M=(0.3961, 0.4314, 0.5020) Q/q_0001.png | R=(0.3961, 0.4314, 0.5020) Q/q_0001.png
PIX frames/k6_mat/0002.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k6_mat/0003.exr | L=(0.4039, 0.4314, 0.5020) Q/q_0003.png | M=(0.4039, 0.4314, 0.5020) Q/q_0003.png | R=(0.4039, 0.4314, 0.5020) Q/q_0003.png
PIX frames/k6_mat/0004.exr | L=(0.4078, 0.4314, 0.5020) Q/q_0004.png | M=(0.4078, 0.4314, 0.5020) Q/q_0004.png | R=(0.4078, 0.4314, 0.5020) Q/q_0004.png
PIX frames/k6_mat/0005.exr | L=(0.4118, 0.4314, 0.5020) Q/q_0005.png | M=(0.4118, 0.4314, 0.5020) Q/q_0005.png | R=(0.4118, 0.4314, 0.5020) Q/q_0005.png
PIX frames/k6_mat/0006.exr | L=(0.4157, 0.4314, 0.5020) Q/q_0006.png | M=(0.4157, 0.4314, 0.5020) Q/q_0006.png | R=(0.4157, 0.4314, 0.5020) Q/q_0006.png
PIX frames/k6_mat/0007.exr | L=(0.4157, 0.4314, 0.5020) Q/q_0006.png | M=(0.4157, 0.4314, 0.5020) Q/q_0006.png | R=(0.4157, 0.4314, 0.5020) Q/q_0006.png
PIX frames/k6_mat/0008.exr | L=(0.4157, 0.4314, 0.5020) Q/q_0006.png | M=(0.4157, 0.4314, 0.5020) Q/q_0006.png | R=(0.4157, 0.4314, 0.5020) Q/q_0006.png
PIX frames/k7_mat/0001.exr | L=(0.0039, 0.4706, 0.5020) G/g_0001.png | M=(0.0039, 0.4706, 0.5020) G/g_0001.png | R=(0.0039, 0.4706, 0.5020) G/g_0001.png
PIX frames/k7_mat/0002.exr | L=(0.0078, 0.4706, 0.5020) G/g_0002.png | M=(0.0078, 0.4706, 0.5020) G/g_0002.png | R=(0.0078, 0.4706, 0.5020) G/g_0002.png
PIX frames/k7_mat/0003.exr | L=(0.0118, 0.4706, 0.5020) G/g_0003.png | M=(0.0118, 0.4706, 0.5020) G/g_0003.png | R=(0.0118, 0.4706, 0.5020) G/g_0003.png
PIX frames/k7_mat/0004.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX frames/k7_mat/0005.exr | L=(0.0196, 0.4706, 0.5020) G/g_0005.png | M=(0.0196, 0.4706, 0.5020) G/g_0005.png | R=(0.0196, 0.4706, 0.5020) G/g_0005.png
PIX frames/k7_mat/0006.exr | L=(0.0235, 0.4706, 0.5020) G/g_0006.png | M=(0.0235, 0.4706, 0.5020) G/g_0006.png | R=(0.0235, 0.4706, 0.5020) G/g_0006.png
PIX frames/k7_mat/0007.exr | L=(0.0275, 0.4706, 0.5020) G/g_0007.png | M=(0.0275, 0.4706, 0.5020) G/g_0007.png | R=(0.0275, 0.4706, 0.5020) G/g_0007.png
PIX frames/k7_mat/0008.exr | L=(0.0314, 0.4706, 0.5020) G/g_0008.png | M=(0.0314, 0.4706, 0.5020) G/g_0008.png | R=(0.0314, 0.4706, 0.5020) G/g_0008.png
PIX frames/k8_mat/0001.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX frames/k8_mat/0002.exr | L=(1.0000, 0.0000, 1.0000) MAGENTA | M=(1.0000, 0.0000, 1.0000) MAGENTA | R=(1.0000, 0.0000, 1.0000) MAGENTA
PIX frames/k8_mat/0003.exr | L=(0.0118, 0.5098, 0.5020) D/d_0003.png | M=(0.0118, 0.5098, 0.5020) D/d_0003.png | R=(0.0118, 0.5098, 0.5020) D/d_0003.png
PIX frames/k8_mat/0004.exr | L=(0.0157, 0.5098, 0.5020) D/d_0004.png | M=(0.0157, 0.5098, 0.5020) D/d_0004.png | R=(0.0157, 0.5098, 0.5020) D/d_0004.png
PIX frames/k8_mat/0005.exr | L=(0.0196, 0.5098, 0.5020) D/d_0005.png | M=(0.0196, 0.5098, 0.5020) D/d_0005.png | R=(0.0196, 0.5098, 0.5020) D/d_0005.png
PIX frames/k8_mat/0006.exr | L=(0.0235, 0.5098, 0.5020) D/d_0006.png | M=(0.0235, 0.5098, 0.5020) D/d_0006.png | R=(0.0235, 0.5098, 0.5020) D/d_0006.png
PIX frames/k8_mat/0007.exr | L=(0.0235, 0.5098, 0.5020) D/d_0006.png | M=(0.0235, 0.5098, 0.5020) D/d_0006.png | R=(0.0235, 0.5098, 0.5020) D/d_0006.png
PIX frames/k8_mat/0008.exr | L=(0.0235, 0.5098, 0.5020) D/d_0006.png | M=(0.0235, 0.5098, 0.5020) D/d_0006.png | R=(0.0235, 0.5098, 0.5020) D/d_0006.png
PIX frames/k9_mat/0001.exr | L=(0.3961, 0.4314, 0.5020) Q/q_0001.png | M=(0.4157, 0.4314, 0.5020) Q/q_0006.png | R=(0.4157, 0.4314, 0.5020) Q/q_0006.png
PIX frames/k9_mat/0002.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4196, 0.4314, 0.5020) Q/q_0007.png | R=(0.4196, 0.4314, 0.5020) Q/q_0007.png
PIX frames/k9_mat/0003.exr | L=(0.4039, 0.4314, 0.5020) Q/q_0003.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
```
The compositor cases printed the same picture for every frame as the material cases, except where a material frame was `MAGENTA`, which the compositor printed as `BLACK`; `k1`, `k3` and `k7`:
```
PIX frames/k1_comp/0001.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_comp/0002.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_comp/0003.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_comp/0004.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_comp/0005.exr | L=(0.4039, 0.4314, 0.5020) Q/q_0003.png | M=(0.4039, 0.4314, 0.5020) Q/q_0003.png | R=(0.4039, 0.4314, 0.5020) Q/q_0003.png
PIX frames/k1_comp/0006.exr | L=(0.4078, 0.4314, 0.5020) Q/q_0004.png | M=(0.4078, 0.4314, 0.5020) Q/q_0004.png | R=(0.4078, 0.4314, 0.5020) Q/q_0004.png
PIX frames/k1_comp/0007.exr | L=(0.4118, 0.4314, 0.5020) Q/q_0005.png | M=(0.4118, 0.4314, 0.5020) Q/q_0005.png | R=(0.4118, 0.4314, 0.5020) Q/q_0005.png
PIX frames/k1_comp/0008.exr | L=(0.4157, 0.4314, 0.5020) Q/q_0006.png | M=(0.4157, 0.4314, 0.5020) Q/q_0006.png | R=(0.4157, 0.4314, 0.5020) Q/q_0006.png
PIX frames/k1_comp/0009.exr | L=(0.4196, 0.4314, 0.5020) Q/q_0007.png | M=(0.4196, 0.4314, 0.5020) Q/q_0007.png | R=(0.4196, 0.4314, 0.5020) Q/q_0007.png
PIX frames/k1_comp/0010.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_comp/0011.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_comp/0012.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_comp/0013.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_comp/0014.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_comp/0015.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k3_comp/0001.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_comp/0002.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_comp/0003.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_comp/0004.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_comp/0005.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_comp/0006.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_comp/0007.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_comp/0008.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_comp/0009.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_comp/0010.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_comp/0011.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_comp/0012.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_comp/0013.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_comp/0014.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k3_comp/0015.exr | L=(0.3922, 0.4314, 0.5020) Q/q_0000.png | M=(0.3922, 0.4314, 0.5020) Q/q_0000.png | R=(0.3922, 0.4314, 0.5020) Q/q_0000.png
PIX frames/k7_comp/0001.exr | L=(0.0039, 0.4706, 0.5020) G/g_0001.png | M=(0.0039, 0.4706, 0.5020) G/g_0001.png | R=(0.0039, 0.4706, 0.5020) G/g_0001.png
PIX frames/k7_comp/0002.exr | L=(0.0078, 0.4706, 0.5020) G/g_0002.png | M=(0.0078, 0.4706, 0.5020) G/g_0002.png | R=(0.0078, 0.4706, 0.5020) G/g_0002.png
PIX frames/k7_comp/0003.exr | L=(0.0118, 0.4706, 0.5020) G/g_0003.png | M=(0.0118, 0.4706, 0.5020) G/g_0003.png | R=(0.0118, 0.4706, 0.5020) G/g_0003.png
PIX frames/k7_comp/0004.exr | L=(0.0000, 0.0000, 0.0000) BLACK | M=(0.0000, 0.0000, 0.0000) BLACK | R=(0.0000, 0.0000, 0.0000) BLACK
PIX frames/k7_comp/0005.exr | L=(0.0196, 0.4706, 0.5020) G/g_0005.png | M=(0.0196, 0.4706, 0.5020) G/g_0005.png | R=(0.0196, 0.4706, 0.5020) G/g_0005.png
PIX frames/k7_comp/0006.exr | L=(0.0235, 0.4706, 0.5020) G/g_0006.png | M=(0.0235, 0.4706, 0.5020) G/g_0006.png | R=(0.0235, 0.4706, 0.5020) G/g_0006.png
PIX frames/k7_comp/0007.exr | L=(0.0275, 0.4706, 0.5020) G/g_0007.png | M=(0.0275, 0.4706, 0.5020) G/g_0007.png | R=(0.0275, 0.4706, 0.5020) G/g_0007.png
PIX frames/k7_comp/0008.exr | L=(0.0314, 0.4706, 0.5020) G/g_0008.png | M=(0.0314, 0.4706, 0.5020) G/g_0008.png | R=(0.0314, 0.4706, 0.5020) G/g_0008.png
```
What the material renders printed for the missing files (the compositor renders printed nothing about them):
```
00:00.368  cycles           | WARNING Image file …/pics/G/g_0004.png does not exist.
00:00.368  cycles           | ERROR Failed to load 1 image files
```
```
00:00.336  cycles           | WARNING Image file …/pics/D/d_0001.png does not exist.
00:00.337  cycles           | ERROR Failed to load 1 image files
00:00.338  cycles           | WARNING Image file …/pics/D/d_0002.png does not exist.
00:00.339  cycles           | ERROR Failed to load 1 image files
```
Hand test 6 rendered `k1_mat` (Auto Refresh off) and `k5_mat` (Auto Refresh on) again from the command line with a `frame_change_post` handler that printed, for each frame, the node's `frame_current` and `filepath_from_user` as stored and on the node's evaluated copy in the dependency graph Blender handed the handler. For `k1_mat`:
```
HANDLER frame_change_post frame 1 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
HANDLER frame_change_post frame 2 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
HANDLER frame_change_post frame 3 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
HANDLER frame_change_post frame 4 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
HANDLER frame_change_post frame 5 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 3 -> …/pics/Q/q_0003.png
HANDLER frame_change_post frame 6 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 4 -> …/pics/Q/q_0004.png
HANDLER frame_change_post frame 7 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 5 -> …/pics/Q/q_0005.png
HANDLER frame_change_post frame 8 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 6 -> …/pics/Q/q_0006.png
HANDLER frame_change_post frame 9 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 7 -> …/pics/Q/q_0007.png
HANDLER frame_change_post frame 10 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
HANDLER frame_change_post frame 11 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
HANDLER frame_change_post frame 12 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
HANDLER frame_change_post frame 13 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
HANDLER frame_change_post frame 14 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
HANDLER frame_change_post frame 15 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
```
`k5_mat` printed the same lines. Those two renders showed the same pictures as the ones above (`k1_mat_handler`, `k5_mat_handler`):
```
PIX frames/k1_mat_handler/0001.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_mat_handler/0002.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_mat_handler/0003.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_mat_handler/0004.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_mat_handler/0005.exr | L=(0.4039, 0.4314, 0.5020) Q/q_0003.png | M=(0.4039, 0.4314, 0.5020) Q/q_0003.png | R=(0.4039, 0.4314, 0.5020) Q/q_0003.png
PIX frames/k1_mat_handler/0006.exr | L=(0.4078, 0.4314, 0.5020) Q/q_0004.png | M=(0.4078, 0.4314, 0.5020) Q/q_0004.png | R=(0.4078, 0.4314, 0.5020) Q/q_0004.png
PIX frames/k1_mat_handler/0007.exr | L=(0.4118, 0.4314, 0.5020) Q/q_0005.png | M=(0.4118, 0.4314, 0.5020) Q/q_0005.png | R=(0.4118, 0.4314, 0.5020) Q/q_0005.png
PIX frames/k1_mat_handler/0008.exr | L=(0.4157, 0.4314, 0.5020) Q/q_0006.png | M=(0.4157, 0.4314, 0.5020) Q/q_0006.png | R=(0.4157, 0.4314, 0.5020) Q/q_0006.png
PIX frames/k1_mat_handler/0009.exr | L=(0.4196, 0.4314, 0.5020) Q/q_0007.png | M=(0.4196, 0.4314, 0.5020) Q/q_0007.png | R=(0.4196, 0.4314, 0.5020) Q/q_0007.png
PIX frames/k1_mat_handler/0010.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat_handler/0011.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat_handler/0012.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat_handler/0013.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat_handler/0014.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat_handler/0015.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
```

*Not covered by these tests:* the Image Editor's own frame settings; Start or Offset animated or driven; frames between whole frames; two compositor Image nodes sharing one image; other render engines than Cycles.

**Outside the interface:** The sources do not say. The settings live in the file and are script-readable. The sources give no different rule for command-line or background renders, and no rule at all for them. **Update 2026-11-15 — observed** (hand tests 5 and 6, Creator's Mac, Blender 5.3.0): every render under *Observed* was a command-line render with no window, as an animation (`-a`) and one frame at a time (`-f 7`, `-f 12`); the one-frame renders showed the same pictures as the same frames of the animation, and Auto Refresh on and off gave the same pictures.

**Operating system:** The sources do not say. **Update 2026-11-15 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

---

## 5. How can a script list the files an image sequence will use over a range of scene frames?

**Label:** documented (the function that turns an image user into a file path, `ImageUser.frame_current`, and `Scene.frame_set`); **inferred** (using `filepath_from_user` frame by frame, mutability of `node.image_user`'s fields, and Python API calls working without the interface); **not in the sources** (any single call that returns the list, whether Blender updates `frame_current` on its own when the scene frame changes, what `filepath_from_user` returns with `image_user=None` or how it treats `frame_current` vs start/offset/duration/cyclic, and how to handle a node without `image_user`). **Update 2026-11-15:** observed by hand tests 6 and 4 on the Creator's Mac — see *Observed* below.

**What the sources say**

> "filepath_from_user(*, image_user=None) — Return the absolute path to the filepath of an image frame specified by the image user. Parameters: image_user (ImageUser | None) – Image user of the image to get filepath for (optional). Returns: File Path, The resulting filepath from the image and its user (never None)"

— API, https://docs.blender.org/api/5.3/bpy.types.Image.html (read 2026-11-08)

> "frame_current — Current frame number in image sequence or movie (in [-1048574, 1048574], default 0)"

— API, https://docs.blender.org/api/5.3/bpy.types.ImageUser.html. It is not marked read-only, while other fields on the same page are.

> "frame_set(frame, *, subframe=0.0) — Set scene frame updating all objects and view layers immediately"

— API, https://docs.blender.org/api/5.3/bpy.types.Scene.html (read 2026-11-08)

The node's image user: "image_user — Parameters defining which layer, pass and frame of the image is displayed (readonly, never None)" (https://docs.blender.org/api/5.3/bpy.types.ShaderNodeTexImage.html).

**Inferred** — two possible routes. Neither is shown as an example in the sources.

- Route A: for each scene frame in the range, work out the sequence frame number from the node's Start Frame, Offset, Frames and Cyclic (question 4). Write it into `image_user.frame_current`, then call `image.filepath_from_user(image_user=node.image_user)` and collect the result. Grounds: `filepath_from_user` returns the path "of an image frame specified by the image user", and `frame_current` is the image user's frame number and can be written. The weak points are that the mapping from question 4 is itself inferred, and this also assumes `filepath_from_user` reads `image_user.frame_current` directly rather than applying `frame_start`, `frame_offset`, `frame_duration`, or `use_cyclic` itself. In addition, if `CompositorNodeImage` carries those fields directly without an `image_user` property (see question 4), Route A via `node.image_user` would not apply directly to it.
- Route B: for each scene frame, call `scene.frame_set(frame)` and then `filepath_from_user` with the node's image user. This depends on Blender updating `frame_current` from the scene frame. The sources do not say that happens. `frame_set` speaks only of "objects and view layers", and `use_auto_refresh` reads "Always refresh image on frame changes", which suggests it might depend on that setting. So route B is unsupported.

The node's `image_user` is marked "readonly". From the general meaning of that mark, I infer it means the image user itself cannot be swapped, not that its fields cannot be changed. The sources do not say so. A script that changes `frame_current` on a node also changes the node in the open file.

**Not in the sources**

- Any function that returns all the files of a sequence, or the files for a range of frames.
- Whether `filepath_from_user` checks that the file exists. It is described only as returning a path.
- Whether `filepath_from_user` reads `image_user.frame_current` as-is or also applies `frame_start`, `frame_offset`, `frame_duration`, and `use_cyclic`, and what it returns when called with `image_user=None`.
- Whether `frame_current` follows the scene frame on its own, and under which settings.
- How to make a stand-alone image user without touching a node, or how to call `filepath_from_user` for a `CompositorNodeImage` if it has no `image_user`.
- Whether `Image.frame_duration` (read-only, "1 when not a video/sequence") counts the files present on disk or something else.

Searches: `filepath_from_user`, `frame_current`, `frame_path`, `image_user`, `ImageUser`, `frame_set`, `sequence`, `frame` across the API folder; `bpy.path` (no frame or sequence helpers: its only "sequence" hit is about a list of directories). (`depsgraph`, `evaluated_get`, `frame_change` handlers, `bpy_extras`, and whether `ImageUser` defines any methods were not reported as searched.) Ruled out: `RenderSettings.frame_path` ("Return the absolute path to the filename to be written for a given frame", https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html), which is for render output, not for reading images. The Cycles denoise operator's loop over `frame_path` (`operators.py`, `CYCLES_OT_denoise_animation.execute`) builds both `in_filepaths` (for animation frames matching Blender's render output naming via `scene.render.filepath`) and `out_filepaths` using `scene.render.frame_path`, so it is ruled out as a general `Image` / `ImageUser` sequence input-listing mechanism. *(Note — citation check 2026-11-15, Gemini 4 Hydrogen XHigh: corrected the reason `CYCLES_OT_denoise_animation.execute` is ruled out to note it populates both `in_filepaths` and `out_filepaths` using render-output naming.)*

**Not covered:** whether either route gives the same file a render would actually load; behaviour for frames before Start Frame; missing files; nodes without `image_user`.

**Observed — hand tests 6 and 4, 2026-11-15, on the Creator's Mac (an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2), Blender 5.3.0 (build hash `02f0899d78e3`), every run started from the terminal with no window (`-b --factory-startup`).** The tests, their exact commands and every script are in `Audits/Blender Research/Colour spaces and image sequences/Hand tests — 5.3.md`; what follows stands on its own.

*What was done.* The files are those of question 4's observed tests (and question 3's, for the names): an Image Texture node named `Tex` showing a picture sequence through its image user, or the compositor's Image node, each picture saying by its colour which file it is. Folder `Q` holds `q_0000.png` … `q_0020.png`; `k1_mat` has Start 5, Offset 2, Frames 6, Cyclic off, Auto Refresh off and stored name `q_0001.png`; `k5_mat` is the same with Auto Refresh on; `k1_comp` has the same settings on a compositor Image node; `k9_mat` has two materials `ML` and `MR` using one image, Offset 0 and 5. A script opened each file from the command line and printed: (1) as opened, the image's `filepath`, `filepath_raw`, `source` and `frame_duration`, the node's settings and `frame_current`, and `filepath_from_user()` with no image user and with the node's (`image.filepath_from_user(image_user=node.image_user)`); (2) for each scene frame, after `scene.frame_set(f)`, the node's `frame_current` and `filepath_from_user` on the node as stored and on its evaluated copy (`bpy.context.evaluated_depsgraph_get()`, then `material.evaluated_get(depsgraph)`); (3) after writing `frame_current` to -1, 0, 1, 2, 5, 8, 13, 20 and 21, `filepath_from_user`; and (4) the second-pass code for `bpy.data.file_path_foreach` with the flags `SKIP_PACKED`, `SKIP_WEAK_REFERENCES` and `EXPAND_SEQUENCES` and a subset of that one image, word for word. A `frame_change_post` handler, given before the render arguments, printed the same during a command-line render (`-s 1 -e 15 -a`).

*What came back.* `k1_mat`, everything the script printed:
```
READ version 5.3.0 | file …/files/5.3/k1_mat.blend | scene frame 1
READ M as opened: image Seq filepath …/pics/Q/q_0001.png filepath_raw …/pics/Q/q_0001.png source SEQUENCE frame_duration 1 | start=5 offset=2 frames=6 cyclic=False auto_refresh=False frame_current=0
READ M as opened: filepath_from_user() -> …/pics/Q/q_0000.png | filepath_from_user(image_user=node.image_user) -> …/pics/Q/q_0000.png
READ route B M frame_set(1): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B M frame_set(2): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B M frame_set(3): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B M frame_set(4): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B M frame_set(5): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B M frame_set(6): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B M frame_set(7): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B M frame_set(8): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B M frame_set(9): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B M frame_set(10): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B M frame_set(11): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B M frame_set(12): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B M frame_set(13): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B M frame_set(14): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B M frame_set(15): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ M after route B: image frame_duration 1
READ route A M frame_current set to -1, reads back -1 -> …/pics/Q/q_0000.png
READ route A M frame_current set to 0, reads back 0 -> …/pics/Q/q_0000.png
READ route A M frame_current set to 1, reads back 1 -> …/pics/Q/q_0001.png
READ route A M frame_current set to 2, reads back 2 -> …/pics/Q/q_0002.png
READ route A M frame_current set to 5, reads back 5 -> …/pics/Q/q_0005.png
READ route A M frame_current set to 8, reads back 8 -> …/pics/Q/q_0008.png
READ route A M frame_current set to 13, reads back 13 -> …/pics/Q/q_0013.png
READ route A M frame_current set to 20, reads back 20 -> …/pics/Q/q_0020.png
READ route A M frame_current set to 21, reads back 21 -> …/pics/Q/q_0021.png
READ file_path_foreach Seq 21 paths: ['…/pics/Q/q_0000.png', '…/pics/Q/q_0001.png', '…/pics/Q/q_0002.png', '…/pics/Q/q_0003.png', '…/pics/Q/q_0004.png', '…/pics/Q/q_0005.png', '…/pics/Q/q_0006.png', '…/pics/Q/q_0007.png', '…/pics/Q/q_0008.png', '…/pics/Q/q_0009.png', '…/pics/Q/q_0010.png', '…/pics/Q/q_0011.png', '…/pics/Q/q_0012.png', '…/pics/Q/q_0013.png', '…/pics/Q/q_0014.png', '…/pics/Q/q_0015.png', '…/pics/Q/q_0016.png', '…/pics/Q/q_0017.png', '…/pics/Q/q_0018.png', '…/pics/Q/q_0019.png', '…/pics/Q/q_0020.png']
```
`k5_mat` (Auto Refresh on), steps 1 and 2:
```
READ M as opened: image Seq filepath …/pics/Q/q_0001.png filepath_raw …/pics/Q/q_0001.png source SEQUENCE frame_duration 1 | start=5 offset=2 frames=6 cyclic=False auto_refresh=True frame_current=0
READ M as opened: filepath_from_user() -> …/pics/Q/q_0000.png | filepath_from_user(image_user=node.image_user) -> …/pics/Q/q_0000.png
READ route B M frame_set(1): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
READ route B M frame_set(2): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
READ route B M frame_set(3): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
READ route B M frame_set(4): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
READ route B M frame_set(5): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 3 -> …/pics/Q/q_0003.png
READ route B M frame_set(6): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 4 -> …/pics/Q/q_0004.png
READ route B M frame_set(7): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 5 -> …/pics/Q/q_0005.png
READ route B M frame_set(8): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 6 -> …/pics/Q/q_0006.png
READ route B M frame_set(9): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 7 -> …/pics/Q/q_0007.png
READ route B M frame_set(10): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
READ route B M frame_set(11): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
READ route B M frame_set(12): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
READ route B M frame_set(13): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
READ route B M frame_set(14): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
READ route B M frame_set(15): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
```
`k1_comp`, the compositor's Image node:
```
READ compositor Img as opened: image Seq filepath …/pics/Q/q_0001.png source SEQUENCE frame_duration 1 | start=5 offset=2 frames=6 cyclic=False auto_refresh=False | has image_user: False
READ compositor Img as opened: filepath_from_user() -> …/pics/Q/q_0001.png
READ route B compositor frame_set(1): filepath_from_user() -> …/pics/Q/q_0001.png | image frame_duration 1
READ route B compositor frame_set(5): filepath_from_user() -> …/pics/Q/q_0001.png | image frame_duration 1
READ route B compositor frame_set(10): filepath_from_user() -> …/pics/Q/q_0001.png | image frame_duration 1
READ route B compositor frame_set(15): filepath_from_user() -> …/pics/Q/q_0001.png | image frame_duration 1
READ file_path_foreach Seq 21 paths: ['…/pics/Q/q_0000.png', '…/pics/Q/q_0001.png', '…/pics/Q/q_0002.png', '…/pics/Q/q_0003.png', '…/pics/Q/q_0004.png', '…/pics/Q/q_0005.png', '…/pics/Q/q_0006.png', '…/pics/Q/q_0007.png', '…/pics/Q/q_0008.png', '…/pics/Q/q_0009.png', '…/pics/Q/q_0010.png', '…/pics/Q/q_0011.png', '…/pics/Q/q_0012.png', '…/pics/Q/q_0013.png', '…/pics/Q/q_0014.png', '…/pics/Q/q_0015.png', '…/pics/Q/q_0016.png', '…/pics/Q/q_0017.png', '…/pics/Q/q_0018.png', '…/pics/Q/q_0019.png', '…/pics/Q/q_0020.png']
```
(the frames not shown printed the same file.) `k9_mat`, two image users of one image, step 2:
```
READ ML as opened: filepath_from_user() -> …/pics/Q/q_0000.png | filepath_from_user(image_user=node.image_user) -> …/pics/Q/q_0000.png
READ MR as opened: filepath_from_user() -> …/pics/Q/q_0000.png | filepath_from_user(image_user=node.image_user) -> …/pics/Q/q_0000.png
READ route B ML frame_set(1): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B MR frame_set(1): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B ML frame_set(2): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B MR frame_set(2): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B ML frame_set(3): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
READ route B MR frame_set(3): stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 0 -> …/pics/Q/q_0000.png
```
During the command-line render, `k1_mat` (Auto Refresh off), and `k5_mat` (Auto Refresh on) the first five frames:
```
HANDLER installed, version 5.3.0
HANDLER render_pre frame 1 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 1 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
HANDLER render_pre frame 2 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 2 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
HANDLER render_pre frame 3 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 3 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
HANDLER render_pre frame 4 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 4 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
HANDLER render_pre frame 5 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 5 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 3 -> …/pics/Q/q_0003.png
HANDLER render_pre frame 6 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 6 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 4 -> …/pics/Q/q_0004.png
HANDLER render_pre frame 7 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 7 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 5 -> …/pics/Q/q_0005.png
HANDLER render_pre frame 8 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 8 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 6 -> …/pics/Q/q_0006.png
HANDLER render_pre frame 9 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 9 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 7 -> …/pics/Q/q_0007.png
HANDLER render_pre frame 10 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 10 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
HANDLER render_pre frame 11 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 11 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
HANDLER render_pre frame 12 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 12 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
HANDLER render_pre frame 13 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 13 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
HANDLER render_pre frame 14 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 14 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
HANDLER render_pre frame 15 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 15 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 8 -> …/pics/Q/q_0008.png
```
```
HANDLER installed, version 5.3.0
HANDLER render_pre frame 1 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 1 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
HANDLER render_pre frame 2 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 2 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
HANDLER render_pre frame 3 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 3 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
HANDLER render_pre frame 4 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 4 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 2 -> …/pics/Q/q_0002.png
HANDLER render_pre frame 5 M: stored frame_current 0 -> …/pics/Q/q_0000.png
HANDLER frame_change_post frame 5 M: stored frame_current 0 -> …/pics/Q/q_0000.png | evaluated frame_current 3 -> …/pics/Q/q_0003.png
```
The pictures those renders showed, frame by frame (from question 4's observed tests; `k5_mat_handler` printed the same):
```
PIX frames/k1_mat_handler/0001.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_mat_handler/0002.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_mat_handler/0003.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_mat_handler/0004.exr | L=(0.4000, 0.4314, 0.5020) Q/q_0002.png | M=(0.4000, 0.4314, 0.5020) Q/q_0002.png | R=(0.4000, 0.4314, 0.5020) Q/q_0002.png
PIX frames/k1_mat_handler/0005.exr | L=(0.4039, 0.4314, 0.5020) Q/q_0003.png | M=(0.4039, 0.4314, 0.5020) Q/q_0003.png | R=(0.4039, 0.4314, 0.5020) Q/q_0003.png
PIX frames/k1_mat_handler/0006.exr | L=(0.4078, 0.4314, 0.5020) Q/q_0004.png | M=(0.4078, 0.4314, 0.5020) Q/q_0004.png | R=(0.4078, 0.4314, 0.5020) Q/q_0004.png
PIX frames/k1_mat_handler/0007.exr | L=(0.4118, 0.4314, 0.5020) Q/q_0005.png | M=(0.4118, 0.4314, 0.5020) Q/q_0005.png | R=(0.4118, 0.4314, 0.5020) Q/q_0005.png
PIX frames/k1_mat_handler/0008.exr | L=(0.4157, 0.4314, 0.5020) Q/q_0006.png | M=(0.4157, 0.4314, 0.5020) Q/q_0006.png | R=(0.4157, 0.4314, 0.5020) Q/q_0006.png
PIX frames/k1_mat_handler/0009.exr | L=(0.4196, 0.4314, 0.5020) Q/q_0007.png | M=(0.4196, 0.4314, 0.5020) Q/q_0007.png | R=(0.4196, 0.4314, 0.5020) Q/q_0007.png
PIX frames/k1_mat_handler/0010.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat_handler/0011.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat_handler/0012.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat_handler/0013.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat_handler/0014.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
PIX frames/k1_mat_handler/0015.exr | L=(0.4235, 0.4314, 0.5020) Q/q_0008.png | M=(0.4235, 0.4314, 0.5020) Q/q_0008.png | R=(0.4235, 0.4314, 0.5020) Q/q_0008.png
```
`file_path_foreach` with `EXPAND_SEQUENCES`, on question 3's name cases and question 4's gapped folder `G` (`g_0004.png` missing) and folder `D` (from `d_0003.png`):
```
READ file_path_foreach Seq 9 paths: ['…/pics/A/a_1.png', '…/pics/A/a_2.png', '…/pics/A/a_3.png', '…/pics/A/a_4.png', '…/pics/A/a_5.png', '…/pics/A/a_6.png', '…/pics/A/a_7.png', '…/pics/A/a_8.png', '…/pics/A/a_9.png']
```
```
READ file_path_foreach Seq 12 paths: ['…/pics/B/b_0001.png', '…/pics/B/b_0002.png', '…/pics/B/b_0003.png', '…/pics/B/b_0004.png', '…/pics/B/b_0005.png', '…/pics/B/b_0006.png', '…/pics/B/b_0007.png', '…/pics/B/b_0008.png', '…/pics/B/b_0009.png', '…/pics/B/b_0010.png', '…/pics/B/b_0011.png', '…/pics/B/b_0012.png']
```
```
READ file_path_foreach Seq 9 paths: ['…/pics/W/w_1.png', '…/pics/W/w_2.png', '…/pics/W/w_3.png', '…/pics/W/w_4.png', '…/pics/W/w_5.png', '…/pics/W/w_6.png', '…/pics/W/w_7.png', '…/pics/W/w_8.png', '…/pics/W/w_9.png']
```
```
READ file_path_foreach Seq 10 paths: ['…/pics/C/c_0001.png', '…/pics/C/c_0002.png', '…/pics/C/c_0003.png', '…/pics/C/c_0004.png', '…/pics/C/c_0005.png', '…/pics/C/c_0006.png', '…/pics/C/c_0007.png', '…/pics/C/c_0008.png', '…/pics/C/c_0009.png', '…/pics/C/c_0012.png']
```
```
READ file_path_foreach Seq 2 paths: ['…/pics/O/o_998.png', '…/pics/O/o_999.png']
```
```
READ file_path_foreach Seq 0 paths: []
```
```
READ file_path_foreach Seq 7 paths: ['…/pics/G/g_0001.png', '…/pics/G/g_0002.png', '…/pics/G/g_0003.png', '…/pics/G/g_0005.png', '…/pics/G/g_0006.png', '…/pics/G/g_0007.png', '…/pics/G/g_0008.png']
```
```
READ file_path_foreach Seq 6 paths: ['…/pics/D/d_0003.png', '…/pics/D/d_0004.png', '…/pics/D/d_0005.png', '…/pics/D/d_0006.png', '…/pics/D/d_0007.png', '…/pics/D/d_0008.png']
```
For comparison, question 3's renders of `n_a` showed `a_10.png`, `a_11.png` and `a_12.png` at frames 10 to 12, and of `n_o` showed `o_1000.png` and `o_1001.png` at frames 1000 and 1001; neither list above names those files.

*Not covered by these tests:* Start or Offset animated or driven; the window's own playback; packed images; a compositor image user reached any other way than the node's own settings.

**Outside the interface:** All the calls named are Python API calls, so I infer they can run without the interface. The sources do not say whether any of them behaves differently when Blender is run in the background. **Update 2026-11-15 — observed** (hand tests 6 and 4, Creator's Mac, Blender 5.3.0): every call under *Observed* ran in a Blender with no window. There, `scene.frame_set` left the stored `frame_current` at 0 with Auto Refresh off and on, and moved the evaluated copy's only with Auto Refresh on; during a command-line render, the `frame_change_post` handler's evaluated copy moved with Auto Refresh off as well.

**Operating system:** The sources do not say. `filepath_from_user` returns an "absolute path". The sources do not say how paths are written on each system. **Update 2026-11-15 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

---

## Things a person could settle by hand in a few minutes

*Pruned 2026-11-15 after the hand tests: all five items were run on the Creator's Mac in Blender 5.3.0, from the terminal with no window, and their results now sit as observed findings under the questions above — item 1 as hand test 3 (with a renamed space that keeps its interop ID, two spaces without a usable one, and a missing view and display as well), item 2 as hand tests 1 and 2, item 3 as hand test 4 (rendered from the command line rather than stepped through in the window), item 4 as hand test 5 and item 5 as hand test 6. What is left needs Blender's window or another computer:*

1. **In the window (Q1, Q2).** Open a file whose image colour space, view or display is missing from the configuration in use, and read the image's Color Space field, the Render Properties ‣ Color Management panel (‣ Advanced: Source and Path), and anything shown in the status bar or the Info editor on opening.
2. **Another operating system (Q1–Q5).** The same tests on Linux and on Windows (naming the Windows edition), including a sequence whose names differ only in capitals.

---

2026-11-15 — Claude Opus 5.5 (Claude Code) — Colour spaces and image sequences, Blender 5.3

Sources consulted: Blender 5.3 manual, Python API documentation and release notes, date read 2026-11-08; supplied Cycles 5.3 files, date read 2026-10-16. These are the snapshot dates required by the brief.
I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

The supplied brief was read first. The earlier findings were not opened. No helpers were used. No Blender session was run. The dates above describe the sources as instructed; this report was prepared on 2026-11-15.

**Hand tests (added 2026-11-15):** observed results from hand tests 1–6, run that day on the Creator's Mac by Claude Opus 5.5 (Claude Code) in Blender 5.3.0, were added under questions 1–5, and the hand-check list at the end was pruned; the same tests ran in 4.5, 5.0, 5.1 and 5.2, whose results are in the list only. The tests, their exact commands and every script are in `Audits/Blender Research/Colour spaces and image sequences/Hand tests — 5.3.md`.

This is the second pass for questions 1–5. Question wording below is unchanged.

Search scope used for every question: the complete supplied `manual_5.3`, `api_5.3`, `release_notes_5.3` and `cycles_5.3` trees inside Blender Sources/. Initial searches covered documentation `.txt` and `.md` files and the Cycles files. Follow-up case-insensitive fixed-string searches covered all files searchable by `rg` in all four trees, without an extension filter. The records under each answer give the terms, pages opened or read in search results, and exclusions. A search hit alone was not treated as an answer. No web sources were used.

## 1. What does Blender do when a file names a colour space that its colour management configuration does not contain? Cover opening the file, a script reading the setting, and rendering.

**Label: documented for the limited load behaviour and diagnostic properties below; not in the sources for the complete opening, script-read and rendering behaviour.** **Update 2026-11-15:** observed by hand tests 3 and 2 on the Creator's Mac — see *Observed* below.

On file load, an available interop ID can be used to update a colour-space name. The sources also document a file-level diagnostic for an unfound colour space, view or display. They do not describe a complete response when the name cannot be resolved.

- **Opening the file:** name updates from interop IDs are documented. The sources do not settle what happens when neither the stored name nor a usable interop ID resolves: whether loading continues, which warning appears, or whether the original name is retained or replaced.
- **A script reading the setting:** `Image.colorspace_settings.name` and `.interop_id` are documented. Their exact returned values, warnings or exceptions for an unresolved stored name are not specified. A property default is not evidence of the value returned in this case.
- **Rendering:** the sources do not specify the transform, fallback interpretation, error or rendered pixels for an unresolved image colour-space name. The fallback *configuration* is not documented as the fallback for an individual missing name.

Exact evidence:

> It also improves compatibility when loading or appending assets from blend files created with another OpenColorIO config:
> on file load, color space names are updated from the interop IDs when available.
> The existing `name` is kept, so Python API backwards compatibility is preserved.

Release notes: [python_api](https://developer.blender.org/docs/release_notes/5.3/python_api/) — Interoperability. Date read: 2026-11-08.

The last sentence says the existing API `name` is kept. It does not say that an unresolved stored string is preserved unchanged.

> A color space, view or display was not found, which likely means the OpenColorIO config used to create this blend file is missing (default False, readonly)

Python API: [bpy.types.BlendFileColorspace](https://docs.blender.org/api/5.3/bpy.types.BlendFileColorspace.html) — is_missing_opencolorio_config. Date read: 2026-11-08.

> The requested OpenColorIO config failed to load (default False, readonly)

Python API: [bpy.types.BlendFileColorspace](https://docs.blender.org/api/5.3/bpy.types.BlendFileColorspace.html) — is_failed_opencolorio_config. Date read: 2026-11-08.

> Color space in the image file, to convert to and from when saving and loading the image (default 'NONE')

Python API: [bpy.types.ColorManagedInputColorspaceSettings](https://docs.blender.org/api/5.3/bpy.types.ColorManagedInputColorspaceSettings.html) — name. Date read: 2026-11-08.

> Do not perform any color transform on load, treat colors as in scene linear space already.

Python API: [bpy_types_enum_items/color_space_convert_default_items](https://docs.blender.org/api/5.3/bpy_types_enum_items/color_space_convert_default_items.html) — NONE. Date read: 2026-11-08.

The `NONE` item has the quoted meaning. Nothing found says that Blender selects it when a stored name is absent from the configuration.

**Record of looking.** Across all four source trees I searched `missing color space`, `unknown color space`, `invalid color space`, `unavailable color space`, `color space not found`, `colour space`, `is_missing_opencolorio_config`, `is_failed_opencolorio_config`, `interop`, `fallback`, `RNA enum` and `not found in enum`. Additional whole-documentation searches combined `color space`/`colorspace`/`OpenColorIO`/`OCIO` with `missing`, `unknown`, `invalid`, `unavailable`, `not found`, `failed`, `load`, `warning`, `error`, `substitute`, `replace`, `retain`, `preserve`, `empty` and `NONE`. The missing/invalid/unavailable phrases and `not found in enum` had no hits in any tree. `unknown color space` had only two Cycles-file hits.

Opened or read relevant passages from the five quoted pages; also [render/color_management/opencolorio](https://docs.blender.org/manual/en/5.3/render/color_management/opencolorio.html), [render/color_management/color_spaces](https://docs.blender.org/manual/en/5.3/render/color_management/color_spaces.html), [editors/image/image_settings](https://docs.blender.org/manual/en/5.3/editors/image/image_settings.html), [bpy.types.Image](https://docs.blender.org/api/5.3/bpy.types.Image.html), [bpy.types.ColorManagedSequencerColorspaceSettings](https://docs.blender.org/api/5.3/bpy.types.ColorManagedSequencerColorspaceSettings.html), [bpy.types.CompositorNodeConvertColorSpace](https://docs.blender.org/api/5.3/bpy.types.CompositorNodeConvertColorSpace.html), [rendering](https://developer.blender.org/docs/release_notes/5.3/rendering/) and [pipeline_io](https://developer.blender.org/docs/release_notes/5.3/pipeline_io/). The interop release note supplies a conditional name-remapping statement. The input-property pages describe ordinary settings, not invalid stored values. Automatic guessing for imported media does not settle unresolved names in an already saved blend-file. OpenEXR metadata and general rendering-interoperability release-note hits do not state the missing-name response.

**Observed, excluded Cycles hits:** `cycles_5.3/shader/node_combine_color.oso`, lines 24–42, contains `#   if (color_type == "rgb" || color_type == "hsv" || color_type == "hsl")` and `#     warning("%s", "Unknown color space!");`. `cycles_5.3/shader/node_separate_color.oso`, lines 293–294, also contains `#     warning("%s", "Unknown color space!");`. Date read: 2026-10-16. These are shader colour-type branches, not evidence about resolving an image's stored OCIO name on file load. No warning behaviour was exercised.

**Not covered:** unresolved-name persistence, exact Python return values and diagnostics, rendering fallback and pixels, or the case where an interop ID is itself unknown. The file-level diagnostic does not identify which individual image failed.

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

**Outside the interface.** The properties are documented Python API properties, so a script can inspect the documented state. The sources do not say whether the unresolved-name response differs between interactive and externally controlled/background loading or rendering. **Update 2026-11-15 — observed** (hand tests 3 and 2, Creator's Mac, Blender 5.3.0): every result under *Observed* came from a Blender with no window. A file given on the command line and the same file opened from a script with `bpy.ops.wm.open_mainfile` printed the same messages and read back the same values, and the command-line render (`-f 1`) gave the pixels quoted there.
**Operating system.** The sources do not establish identical unresolved-name behaviour on every supported operating system. **Update 2026-11-15 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

## 2. Where does Blender's colour management configuration come from, and how can a script learn which configuration is in use and which colour spaces it contains?

**Label: documented for configuration selection and inspection; inferred for using general enum introspection to obtain selectable names; not in the sources for a verified complete enumeration of the active configuration.** **Update 2026-11-15:** observed by hand tests 1 and 2 on the Creator's Mac — see *Observed* below.

Blender uses the first available valid configuration in this order: `BLENDER_OCIO`, `OCIO`, the active Blender Project, the bundled Blender configuration, then Blender's fallback configuration. A project can select a supplied configuration or a custom path. Reloading the blend-file is required after a project configuration change.

A script can read the active configuration's source and path or URI through `bpy.data.colorspace.ocio_config_source` and `.ocio_config_path`. These report the active configuration; a project's requested path alone does not establish what won the precedence search.

The following is an assembled use of the documented properties, not an executed test:

```python
import bpy

configuration = bpy.data.colorspace
source = configuration.ocio_config_source
path_or_uri = configuration.ocio_config_path
failed_to_load = configuration.is_failed_opencolorio_config
missing_space_view_or_display = configuration.is_missing_opencolorio_config
```

Exact evidence:

> Blender searches for an OpenColorIO configuration in the following order,
> using the first available valid configuration:
> 
> The BLENDER_OCIO environment variable.
> 
> The OCIO environment variable.
> 
> The active Blender Project configuration.
> 
> Blender’s bundled configuration.
> 
> Blender’s fallback configuration.

Manual: [render/color_management/opencolorio](https://docs.blender.org/manual/en/5.3/render/color_management/opencolorio.html) — Active Configuration. Date read: 2026-11-08.

> In the OpenColorIO Configuration dropdown you can select from a set of
> built-in configurations that come with Blender, or you can select the Path
> option.
> The Path option lets you browse to and select a custom OpenColorIO
> configuration on disk:

Manual: [projects/project_ocio_config](https://docs.blender.org/manual/en/5.3/projects/project_ocio_config.html) — Selecting a Custom Configuration. *(Note — citation check 2026-11-15, Gemini 4 Hydrogen XHigh: added quote supporting "A project can select a supplied configuration or a custom path" above.)* Date read: 2026-11-08.

> Changes to the OpenColorIO configuration are not applied automatically.
> The blend-file must be reloaded for the updated configuration to take effect.

Manual: [projects/project_ocio_config](https://docs.blender.org/manual/en/5.3/projects/project_ocio_config.html) — Note. Date read: 2026-11-08.

> Information about the color space used for data-blocks in a blend file (readonly, never None)

Python API: [bpy.types.BlendData](https://docs.blender.org/api/5.3/bpy.types.BlendData.html) — colorspace; type BlendFileColorspace. Date read: 2026-11-08.

> File path or URI of the OpenColorIO configuration currently in use (default “”, readonly, never None)

Python API: [bpy.types.BlendFileColorspace](https://docs.blender.org/api/5.3/bpy.types.BlendFileColorspace.html) — ocio_config_path. Date read: 2026-11-08.

> Where the OpenColorIO configuration came from (default 'BLENDER_OCIO', readonly)

Python API: [bpy.types.BlendFileColorspace](https://docs.blender.org/api/5.3/bpy.types.BlendFileColorspace.html) — ocio_config_source. Date read: 2026-11-08.

> The list of color spaces depends on the active OCIO config.

Manual: [editors/image/image_settings](https://docs.blender.org/manual/en/5.3/editors/image/image_settings.html) — Color Space. Date read: 2026-11-08.

> Possible values for the property (default None, readonly)

Python API: [bpy.types.EnumProperty](https://docs.blender.org/api/5.3/bpy.types.EnumProperty.html) — enum_items. Date read: 2026-11-08.

> Possible values for the property (never calls optional dynamic generation of those) (default None, readonly)

Python API: [bpy.types.EnumProperty](https://docs.blender.org/api/5.3/bpy.types.EnumProperty.html) — enum_items_static. Date read: 2026-11-08.

> Unique name used in the code and scripting (default “”, readonly, never None)

Python API: [bpy.types.EnumPropertyItem](https://docs.blender.org/api/5.3/bpy.types.EnumPropertyItem.html) — identifier. Date read: 2026-11-08.

> The requested OpenColorIO config failed to load (default False, readonly)

Python API: [bpy.types.BlendFileColorspace](https://docs.blender.org/api/5.3/bpy.types.BlendFileColorspace.html) — is_failed_opencolorio_config (used in the snippet above; also quoted in question 1). *(Note — citation check 2026-11-15, Gemini 4 Hydrogen XHigh: added so every property in the question 2 snippet is quoted in question 2.)* Date read: 2026-11-08.

> A color space, view or display was not found, which likely means the OpenColorIO config used to create this blend file is missing (default False, readonly)

Python API: [bpy.types.BlendFileColorspace](https://docs.blender.org/api/5.3/bpy.types.BlendFileColorspace.html) — is_missing_opencolorio_config (used in the snippet above; also quoted in question 1). *(Note — citation check 2026-11-15, Gemini 4 Hydrogen XHigh: added so every property in the question 2 snippet is quoted in question 2.)* Date read: 2026-11-08.

> Input color space settings (readonly)
>
> Type: ColorManagedInputColorspaceSettings | None

Python API: [bpy.types.Image](https://docs.blender.org/api/5.3/bpy.types.Image.html) — colorspace_settings. *(Note — citation check 2026-11-15, Gemini 4 Hydrogen XHigh: added to support the enum premise below.)* Date read: 2026-11-08.

> Color space in the image file, to convert to and from when saving and loading the image (default 'NONE')
>
> Type: Literal[Color Space Convert Default Items]

Python API: [bpy.types.ColorManagedInputColorspaceSettings](https://docs.blender.org/api/5.3/bpy.types.ColorManagedInputColorspaceSettings.html) — name. *(Note — citation check 2026-11-15, Gemini 4 Hydrogen XHigh: added with its Literal[...] type so the enum premise below is directly quoted.)* Date read: 2026-11-08.

**Inference and its edge:** the image's colour-space field is an enum, its offered list depends on the active configuration, and RNA enums expose `enum_items`. That suggests inspecting the live colour-space enum's item identifiers to obtain names that Blender offers. It does not establish that this gives every colour space defined in the configuration, including aliases or spaces Blender does not offer. The supplied pages contain no verified colour-space-specific enumeration recipe or guarantee about the required context. I therefore do not present an executable reflection expression as established. `enum_items_static` explicitly avoids dynamic generation and cannot be assumed to represent the live configuration. The static “Color Space Convert Default Items” page lists `NONE`; it is not an inventory of the active configuration.

**Record of looking.** Across all four trees I searched `OCIO`, `OpenColorIO`, `ocio_config_path`, `ocio_config_source`, `PyOpenColorIO`, `getCurrentConfig`, `getColorSpaceNames`, `enum_items`, `dynamic generation` and `color_space_convert_default_items`. The three Python OpenColorIO module/function searches had no hits. Additional documentation searches covered active/current configuration, path/source, dynamic enum/item generation, `bl_rna.properties`, `rna_type.properties`, `.bl_rna` as a complete name and `.enum_items`. No live colour-space enumeration example was found.

Opened or read passages from the quoted pages; also [bpy.types.BlenderProject](https://docs.blender.org/api/5.3/bpy.types.BlenderProject.html), [bpy.types.Struct](https://docs.blender.org/api/5.3/bpy.types.Struct.html), [bpy.types.ColorManagedInputColorspaceSettings](https://docs.blender.org/api/5.3/bpy.types.ColorManagedInputColorspaceSettings.html), [bpy_types_enum_items/color_space_convert_default_items](https://docs.blender.org/api/5.3/bpy_types_enum_items/color_space_convert_default_items.html), [bpy_types_enum_items/color_space_interop_id_default_items](https://docs.blender.org/api/5.3/bpy_types_enum_items/color_space_interop_id_default_items.html), [advanced/command_line/arguments](https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html), [compositing/types/color/convert_colorspace](https://docs.blender.org/manual/en/5.3/compositing/types/color/convert_colorspace.html), [core](https://developer.blender.org/docs/release_notes/5.3/core/) and [python_api](https://developer.blender.org/docs/release_notes/5.3/python_api/). The project settings page describes the requested configuration, while BlendFileColorspace explicitly describes the one currently used. Struct/EnumProperty pages support general introspection concepts, not completeness of a colour-space inventory. Default enum-item pages and the manual's built-in-space list were ruled out as lists of a custom active configuration.

Inspected the manual images [_images/projects_project_settings_ocio](https://docs.blender.org/manual/en/5.3/_images/projects_project_settings_ocio.png) and [_images/projects_project_settings_ocio_path](https://docs.blender.org/manual/en/5.3/_images/projects_project_settings_ocio_path.png). **Documented visual evidence:** the second shows “OpenColorIO Configuration”, “Path” and a Path field. The closed dropdown in the first does not enumerate all configurations, and neither image lists colour spaces. Date read: 2026-11-08.

**Not covered:** a tested script that lists every space/alias in an arbitrary active OCIO configuration, configuration contents when its path is a URI, dynamic enum context requirements, or checksums/identity beyond the documented source and path.

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

**Outside the interface.** The active-state inspection is exposed by the Python API. The command-line manual documents the environment overrides. The sources do not specify a different background precedence order, but they also do not explicitly guarantee identical configuration discovery in every external launch environment. The enum proposal remains unverified in either mode. **Update 2026-11-15 — observed** (hand tests 1 and 2, Creator's Mac, Blender 5.3.0), all with no window: Blender printed `Using config from BLENDER_OCIO` or `Using config from OCIO` and `ocio_config_source` read that variable; a file in a project, at its top or one folder down, given on the command line, opened with `Open project:` and `ocio_config_source` `'PROJECT'`, and with `BLENDER_OCIO` also set it read `'BLENDER_OCIO'`; the colour space choices came back in full through the image's own settings (`bl_rna` or `rna_type`) and through the settings type, `enum_items_static` gave one, and the display, view and look choices read through `bl_rna` gave `['NONE']`, while Blender's refusal of the name `No Such Name` listed them. All quoted under *Observed*.
**Operating system.** No OS exception is stated for the selection order or these API properties. That is not a documented guarantee of identical discovery, path resolution or enum results on every operating system. Display/HDR differences in the system-configuration page concern display output and do not establish a difference in this configuration-selection order. **Update 2026-11-15 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

## 3. For an image whose source is an image sequence, which part of the stored file name stands for the frame number, and how does Blender find the file for a given frame? Include file names whose number has a different count of digits.

**Label: documented for numbered-image examples and the path-resolver API; not in the sources for the complete filename-parsing and digit-count rules.** **Update 2026-11-15:** observed by hand tests 4 on the Creator's Mac — see *Observed* below.

The manual requires digits indicating frame order and gives names ending `-0001.jpg`, `-0002.jpg`, `-0003.jpg`. This identifies the frame-number portion in those examples. It does not specify a general parser for an arbitrary stored name. In particular, the sources do not settle which number is selected if a name contains several number groups, how the chosen frame replaces that group, or how its digit count affects the path.

`Image.filepath_from_user(image_user=...)` is the documented way to ask Blender for an absolute image-frame path from an image and its user. That avoids claiming a filename substitution algorithm which is absent from the sources. Its description does not explain that algorithm.

Exact evidence:

> To load image sequence in any of the supported image file formats,
> the filename of the images must contain a digit to indicate the frame order
> (e.g. *-0001.jpg, *-0002.jpg, *-0003.jpg, etc, of any image format), indicating the frame.

Manual: [files/media/image_formats](https://docs.blender.org/manual/en/5.3/files/media/image_formats.html) — Opening an Image Sequence. Date read: 2026-11-08.

> Image/Movie file name (default “”, never None, blend relative // prefix supported)

Python API: [bpy.types.Image](https://docs.blender.org/api/5.3/bpy.types.Image.html) — filepath. Date read: 2026-11-08.

> Return the absolute path to the filepath of an image frame specified by the image user

Python API: [bpy.types.Image](https://docs.blender.org/api/5.3/bpy.types.Image.html) — filepath_from_user(*, image_user=None). Date read: 2026-11-08.

> image_user (ImageUser | None) – Image user of the image to get filepath for (optional)

Python API: [bpy.types.Image](https://docs.blender.org/api/5.3/bpy.types.Image.html) — filepath_from_user parameter. Date read: 2026-11-08.

**Different digit counts:** no rule was found establishing the outcomes for `shot1.png`, `shot01.png`, `shot0001.png`, mixed widths in one directory, or a transition from `shot999.png` to frame 1000. These are unanswered cases, not evidence that Blender rejects such names.

Two nearby passages must not be substituted for the missing rule:

> This works by detecting if the filename has a .xxxx (four digit number) before the file extension.

Manual: [files/media/image_formats](https://docs.blender.org/manual/en/5.3/files/media/image_formats.html) — Detect UDIMs, not Detect Sequences. Date read: 2026-11-08.

> You can set a custom padding size by adding the appropriate number of # anywhere in the file name
> (e.g. image_##_test.png translates to image_01_test.png).

Manual: [render/output/properties/output](https://docs.blender.org/manual/en/5.3/render/output/properties/output.html) — File Path for render output. Date read: 2026-11-08.

The first passage describes UDIM detection. The second describes generating render-output names. Neither specifies how an Image with source `SEQUENCE` decodes an input filename.

**Record of looking.** Across all four trees I searched `image sequence`, `numbered sequence`, `frame naming`, `digit`, `padding`, `zero-padded`, `trailing number`, `last digit`, `last number`, `numeric suffix`, `filename` and `filepath_from_user`. Additional documentation searches combined sequence/filename/file name/file path/filepath with number/frame/pattern/digit/padding; and searched `last number`, `last digit`, `trailing number`, `trailing digit`, `numeric suffix`, `zero padding` and `zero padded` through equivalent case-insensitive regular expressions. The fixed-string searches `zero-padded`, `last digit` and `last number` had no hits in any tree.

Opened or read passages from the quoted pages; also [editors/image/image_settings](https://docs.blender.org/manual/en/5.3/editors/image/image_settings.html), [editors/image/editing](https://docs.blender.org/manual/en/5.3/editors/image/editing.html), [editors/file_browser](https://docs.blender.org/manual/en/5.3/editors/file_browser.html), [modeling/volumes/properties](https://docs.blender.org/manual/en/5.3/modeling/volumes/properties.html), [video_editing/edit/montage/strips/image](https://docs.blender.org/manual/en/5.3/video_editing/edit/montage/strips/image.html), [bpy.types.RenderSettings](https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html), [advanced/command_line/arguments](https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html) and [render/materials/legacy_textures/types/image_movie](https://docs.blender.org/manual/en/5.3/render/materials/legacy_textures/types/image_movie.html). File Browser number incrementing concerns editing a filename. Numeric suffixes in the volume page concern OpenVDB sequences. They are suggestive adjacent material, not a complete Image filename parser. VSE image-strip import can use all selected images regardless of name; that distinct strip facility does not settle this question about an Image source. `RenderSettings.frame_path` concerns the file to be written, not the image file to be read. The legacy texture page says the images are numbered, without giving a parsing rule. Release-note searches supplied no filename-width rule.

Inspected the image-settings screenshot and the Image Texture and compositor Image node screenshots. The former actually shows Source “Generated”; the latter two show no loaded sequence. They do not expose a sequence filename parser or a padding choice. Images: [ editors_image_image-settings_movie-image-panel.png ](https://docs.blender.org/manual/en/5.3/_images/editors_image_image-settings_movie-image-panel.png); [ node-types_ShaderNodeTexImage.webp ](https://docs.blender.org/manual/en/5.3/_images/node-types_ShaderNodeTexImage.webp); [ node-types_CompositorNodeImage.webp ](https://docs.blender.org/manual/en/5.3/_images/node-types_CompositorNodeImage.webp). Date read: 2026-11-08.

**Not covered:** the selected number group in an arbitrary filename; widths, overflow, leading zeros, missing numbers, multiple extensions, and whether an alternative width is searched if the computed path does not exist. A resolved path is not proof that a file exists or was successfully read.

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

**Outside the interface.** The path resolver is a Python API method. No separate background filename rule was found, and the sources do not establish interactive/background parity for the unresolved parsing cases. **Update 2026-11-15 — observed** (hand test 4, Creator's Mac, Blender 5.3.0): the command-line renders (`-s … -e … -a`) showed the files quoted under *Observed*, and Blender's Open Image operator ran in a Blender with no window.
**Operating system.** The sources do not state whether these input naming rules are identical across all operating systems. No case-sensitivity or filesystem-search rule for sequence lookup was found. **Update 2026-11-15 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open. Names that differ only in capitals were not tried.

## 4. Which file of an image sequence does a render use at a given scene frame, taking into account the image's own frame settings?

**Label: documented for the settings and the preview/render distinction; inferred for the simple interior frame calculation; not in the sources for the complete render-to-file mapping.** **Update 2026-11-15:** observed by hand tests 5 and 6 on the Creator's Mac — see *Observed* below.

“The image's own frame settings” is ambiguous. The reading used here is the settings of the use of the Image which supplies the render, such as an Image Texture node. The other reading is the Image Editor's preview settings. The manual explicitly says those preview settings do not affect the render. An Image also has a duration property, but an `ImageUser` describes how another data-block uses that image. A single image filename and a scene frame therefore do not supply all the documented inputs to the path-resolver API.

For a render texture, the manual describes Frames (playback duration), Start Frame (scene start), Offset (skip initial frames) and Cyclic (loop). It describes Auto Refresh as a viewport update control; it does not say that this option must be enabled for a render to advance.

Exact evidence:

> The options below are for preview purposes only; they don’t affect the 3D Viewport
> or the render. For that, see the Image Texture Node.

Manual: [editors/image/image_settings](https://docs.blender.org/manual/en/5.3/editors/image/image_settings.html) — Movie settings; Image Sequence refers to Movie for options. Date read: 2026-11-08.

> Parameters defining how an Image data-block is used by another data-block

Python API: [bpy.types.ImageUser](https://docs.blender.org/api/5.3/bpy.types.ImageUser.html) — class description. Date read: 2026-11-08.

> Duration (in frames) of the image (1 when not a video/sequence) (in [0, inf], default 0, readonly)

Python API: [bpy.types.Image](https://docs.blender.org/api/5.3/bpy.types.Image.html) — frame_duration. *(Note — citation check 2026-11-15, Gemini 4 Hydrogen XHigh: added quote supporting "An Image also has a duration property" above.)* Date read: 2026-11-08.

> Parameters defining which layer, pass and frame of the image is displayed (readonly, never None)

Python API: [bpy.types.ShaderNodeTexImage](https://docs.blender.org/api/5.3/bpy.types.ShaderNodeTexImage.html) — image_user; type ImageUser. Date read: 2026-11-08.

> Current frame number in image sequence or movie (in [-1048574, 1048574], default 0)

Python API: [bpy.types.ImageUser](https://docs.blender.org/api/5.3/bpy.types.ImageUser.html) — frame_current. Date read: 2026-11-08.

> Global starting frame of the movie/sequence, assuming first picture has a #1 (in [-1048574, 1048574], default 0)

Python API: [bpy.types.ImageUser](https://docs.blender.org/api/5.3/bpy.types.ImageUser.html) — frame_start. Date read: 2026-11-08.

> How many frames of the Movie-type image (video) to play. Past this point, the video will be paused
> (unless Cyclic is enabled).

Manual: [render/shader_nodes/textures/image](https://docs.blender.org/manual/en/5.3/render/shader_nodes/textures/image.html) — Frames. Date read: 2026-11-08.

> Scene frame at which the video should start playing.

Manual: [render/shader_nodes/textures/image](https://docs.blender.org/manual/en/5.3/render/shader_nodes/textures/image.html) — Start Frame. Date read: 2026-11-08.

> Number of frames to offset the video to an earlier point in time.
> (Put differently: how many frames at the start of the video to skip.)

Manual: [render/shader_nodes/textures/image](https://docs.blender.org/manual/en/5.3/render/shader_nodes/textures/image.html) — Offset. Date read: 2026-11-08.

> Start over after the last frame to create a continuous loop.

Manual: [render/shader_nodes/textures/image](https://docs.blender.org/manual/en/5.3/render/shader_nodes/textures/image.html) — Cyclic. Date read: 2026-11-08.

> Update the video texture in the 3D Viewport when moving through the timeline.

Manual: [render/shader_nodes/textures/image](https://docs.blender.org/manual/en/5.3/render/shader_nodes/textures/image.html) — Auto Refresh. Date read: 2026-11-08.

**Limited inference:** for ordinary whole scene frames during playback, before any endpoint handling, with no time remapping, animated settings or looping, the descriptions suggest a one-based sequence position `p = scene_frame - start_frame + 1 + offset`. The steps are: Start places the first picture at the chosen scene frame; the API describes that first picture as #1; each scene-frame advance advances playback; Offset skips pictures at the beginning. This is an interpretation of the descriptions, not a documented implementation formula. It does not establish the numeric suffix in the resulting filename, particularly when the stored filename starts at a number other than 1.

No complete rule was found for before-start frames, clamping endpoints, the order of offset and clamping/wrapping, zero duration, negative offsets, or the correspondence between a stored base filename and the one-based picture position. The manual routes sequence controls to the Movie section, but much of the setting text itself says “video” or “movie”; the API explicitly mentions sequence for start/current frame. This wording does not supply the missing edge-case algorithm. Consequently I cannot name the exact file a render uses for all combinations from these sources alone.

**Record of looking.** Across all four trees I searched `ImageUser`, `frame_current`, `frame_start`, `frame_offset`, `frame_duration`, `use_cyclic`, `use_auto_refresh`, `clamp`, `modulo`, `before the first frame`, `missing frame` and `preview purposes only`. Additional documentation searches covered frame with wrap/clamp/modulo/negative/before, sequence with missing/gap, and current-frame update/render combinations. `before the first frame` had no hits. Clamp/modulo hits did not supply an ImageUser resolution algorithm; nearby explicit pre/post-range modes described volume sequences or VSE strips.

Opened or read passages from the quoted pages; also [bpy.types.Image](https://docs.blender.org/api/5.3/bpy.types.Image.html), [bpy.types.CompositorNodeImage](https://docs.blender.org/api/5.3/bpy.types.CompositorNodeImage.html), [compositing/types/input/image](https://docs.blender.org/manual/en/5.3/compositing/types/input/image.html), [bpy.types.Scene](https://docs.blender.org/api/5.3/bpy.types.Scene.html), [modeling/volumes/properties](https://docs.blender.org/manual/en/5.3/modeling/volumes/properties.html) and [sequencer](https://developer.blender.org/docs/release_notes/5.3/sequencer/). Image exposes duration but not a scene-frame resolver. The compositor node exposes frame settings directly; its API page does not expose the same `image_user` property as ShaderNodeTexImage, so that property cannot be assumed for it. Volume “Clip/Extend/Repeat/Ping-Pong” modes and VSE strip clamping were ruled out as rules for ImageUser. The same three screenshots inspected for question 3 do not show sequence controls. No render was run.

**Not covered:** an exact mapping for every render use, compositor access to its internal image user, offsets at boundaries, duration zero, cyclic arithmetic, subframes, scene time remapping, driven settings, missing-file behaviour, or agreement among render engines.

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

**Outside the interface.** Frame controls and ImageUser are exposed in Python, but a method being accessible does not prove that externally setting the scene frame synchronizes the correct image user in the same way as rendering. The sources do not settle that synchronization or a background-specific difference. **Update 2026-11-15 — observed** (hand tests 5 and 6, Creator's Mac, Blender 5.3.0): every render under *Observed* was a command-line render with no window, as an animation (`-a`) and one frame at a time (`-f 7`, `-f 12`); the one-frame renders showed the same pictures as the same frames of the animation, and Auto Refresh on and off gave the same pictures.
**Operating system.** No OS-specific frame-setting algorithm was found. The sources do not prove identical rendered file selection on every supported operating system. **Update 2026-11-15 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

## 5. How can a script list the files an image sequence will use over a range of scene frames?

**Label: documented for the individual API operations; inferred for combining frame changes with image-user path resolution; not in the sources for a verified general list of files a render will use over an arbitrary scene-frame range.** **Update 2026-11-15:** observed by hand tests 6 and 4 on the Creator's Mac — see *Observed* below.

The relevant APIs provide two different operations:

- Resolve an image-frame path from a particular ImageUser: `image.filepath_from_user(image_user=user)`.
- Visit an image's associated paths, expanding image sequences into files on disk: `bpy.data.file_path_foreach` with `EXPAND_SEQUENCES` and a subset containing the image.

The second is a documented disk inventory. Its description does not restrict expansion to a scene-frame range or to what a render will read after start, offset, duration and cyclic settings. The first resolves a frame specified by the supplied user; its signature has no scene-frame or frame-range argument.

Exact evidence:

> filepath_from_user(*, image_user=None)
> 
> Return the absolute path to the filepath of an image frame specified by the image user

Python API: [bpy.types.Image](https://docs.blender.org/api/5.3/bpy.types.Image.html) — method. Date read: 2026-11-08.

> Set scene frame updating all objects and view layers immediately

Python API: [bpy.types.Scene](https://docs.blender.org/api/5.3/bpy.types.Scene.html) — frame_set(frame, *, subframe=0.0). Date read: 2026-11-08.

> Current frame, to update animation data from Python frame_set() instead (in [-1048574, 1048574], default 1)

Python API: [bpy.types.Scene](https://docs.blender.org/api/5.3/bpy.types.Scene.html) — frame_current. Date read: 2026-11-08.

> file_path_foreach(visit_path_fn, *, subset=None, visit_types=None, flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES'})
>
> Call visit_path_fn for the file paths used by all ID data-blocks in current bpy.data.

Python API: [bpy.types.BlendData](https://docs.blender.org/api/5.3/bpy.types.BlendData.html) — file_path_foreach. *(Note — citation check 2026-11-15, Gemini 4 Hydrogen XHigh: added the method signature so the default flags `SKIP_PACKED` and `SKIP_WEAK_REFERENCES` used in the snippet below are quoted here.)* Date read: 2026-11-08.

> subset (set[bpy.types.ID] | None) – When given, only these data-blocks and their used file paths will be visited.

Python API: [bpy.types.BlendData](https://docs.blender.org/api/5.3/bpy.types.BlendData.html) — file_path_foreach. Date read: 2026-11-08.

> visit_path_fn (Callable[[bpy.types.ID, str, bpy.types.BlendDataPathMeta], str|None]) – function that takes three parameters: the data-block, a file path, and a bpy.types.BlendDataPathMeta metadata object. The function should return either None or a str. In the latter case, the visited file path will be replaced with the returned string.

Python API: [bpy.types.BlendData](https://docs.blender.org/api/5.3/bpy.types.BlendData.html) — file_path_foreach. Date read: 2026-11-08.

> Expand image and volume sequences, invoking the callback once per file on disk.

Python API: [bpy_types_enum_items/file_path_foreach_flag_items](https://docs.blender.org/api/5.3/bpy_types_enum_items/file_path_foreach_flag_items.html) — EXPAND_SEQUENCES. Date read: 2026-11-08.

> True when the path was expanded from a UDIM tile or sequence frame. These paths can not be edited.

Python API: [bpy.types.BlendDataPathMeta](https://docs.blender.org/api/5.3/bpy.types.BlendDataPathMeta.html) — is_expanded. Date read: 2026-11-08.

An assembled use of the documented inventory API is below. It has not been executed. `image` must be the intended sequence Image. Returning `None` leaves each visited path unchanged. The result is the visited paths for that image with disk sequence expansion; it is not asserted to be the answer for a scene-frame interval.

```python
import bpy

def sequence_disk_paths(image):
    paths = []

    def collect(data_block, path, metadata):
        paths.append(path)
        return None

    bpy.data.file_path_foreach(
        collect,
        subset={image},
        flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES', 'EXPAND_SEQUENCES'},
    )
    return paths
```

**Inferred range procedure, requiring a Blender check:** for each requested scene frame, use `scene.frame_set(frame)`, obtain the render consumer's correctly evaluated image settings/user for that frame, and call `image.filepath_from_user(image_user=user)`. Keep the association between scene frame, consumer and returned path; only combine repeated paths if the desired result is a set. This combines the documented frame setter with the documented user-based path resolver. The missing step is a guarantee that the user being inspected has been updated exactly as a render would update it. Merely looping over scene frames is not a documented guarantee of that. Question 4's unresolved boundary cases and the compositor's different exposed API also remain.

The dependency-graph and render-handler pages offer relevant mechanisms, but neither states that it resolves those gaps:

> Get the dependency graph for the current scene and view layer, to access to data-blocks with animation and modifiers applied. If any data-blocks have been edited, the dependency graph will be updated. This invalidates all references to evaluated data-blocks from the dependency graph.

Python API: [bpy.types.Context](https://docs.blender.org/api/5.3/bpy.types.Context.html) — evaluated_depsgraph_get. Date read: 2026-11-08.

> Get corresponding evaluated ID from the given dependency graph. Note that this does not ensure the dependency graph is fully evaluated, it just returns the result of the last evaluation.

Python API: [bpy.types.ID](https://docs.blender.org/api/5.3/bpy.types.ID.html) — evaluated_get. Date read: 2026-11-08.

> Called after frame change for playback and rendering, after the data has been evaluated for the new frame. Accepts one or two arguments: The scene data-block, and optionally the dependency graph being updated

Python API: [bpy.app.handlers](https://docs.blender.org/api/5.3/bpy.app.handlers.html) — frame_change_post. Date read: 2026-11-08.

**Record of looking.** Across all four trees I searched `filepath_from_user`, `file_path_foreach`, `EXPAND_SEQUENCES`, `file_path_map`, `frame_set`, `image_user`, `evaluated_get`, `evaluated_depsgraph_get`, `frame_change_post`, `frame range` and `dependency graph`. Additional whole-documentation searches covered sequence paths/files, expanded paths, `expand_sequence`, `expand_sequences`, `is_expanded`, frame calculation and path-resolution methods. This found both the sequence-expansion flag and the path metadata page; the investigation did not stop at the first path resolver.

Opened or read passages from the quoted pages; also [bpy.types.ImageUser](https://docs.blender.org/api/5.3/bpy.types.ImageUser.html), [bpy.types.ShaderNodeTexImage](https://docs.blender.org/api/5.3/bpy.types.ShaderNodeTexImage.html), [bpy.types.CompositorNodeImage](https://docs.blender.org/api/5.3/bpy.types.CompositorNodeImage.html), [bpy.types.RenderSettings](https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html), [bpy.path](https://docs.blender.org/api/5.3/bpy.path.html), [info_gotchas_internal_data_and_python_objects](https://docs.blender.org/api/5.3/info_gotchas_internal_data_and_python_objects.html), [advanced/command_line/arguments](https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html), [pipeline_io](https://developer.blender.org/docs/release_notes/5.3/pipeline_io/) and [sequencer](https://developer.blender.org/docs/release_notes/5.3/sequencer/). `file_path_map` describes a set of paths per data-block, with no scene-frame interval. `EXPAND_SEQUENCES` explicitly says files on disk, without a scene-range filter. `RenderSettings.frame_path` resolves output filenames. `bpy.path.abspath` resolves relative paths, not sequence frames. Dependency-graph access and post-frame handlers document evaluation timing but not a complete sequence-use manifest. VSE release-note frame ranges concern strips, not a range resolver for Image. None establishes a general script that returns exactly the render inputs for the requested range.

**Not covered:** a tested complete range-listing script, missing files a render may attempt to open, animated or driven settings, all consumers of one Image, compositor internal image-user access, packed images, subframes, render/view-layer differences and whether a returned path was actually read. Disk inventory and intended render inputs can therefore not be equated from these descriptions.

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

**Outside the interface.** These are Python APIs. The command-line manual documents background mode as “Run in background (often used for UI-less rendering).” The proposed frame/user procedure and disk-inventory code were not run in background or in the interface; identical render-input results in both modes remain unestablished. **Update 2026-11-15 — observed** (hand tests 6 and 4, Creator's Mac, Blender 5.3.0): every call under *Observed* ran in a Blender with no window. There, `scene.frame_set` left the stored `frame_current` at 0 with Auto Refresh off and on, and moved the evaluated copy's only with Auto Refresh on; during a command-line render, the `frame_change_post` handler's evaluated copy moved with Auto Refresh off as well.
**Operating system.** No OS exception is stated for these APIs. The sources do not guarantee an identical file inventory or render-input list across operating systems and filesystems. **Update 2026-11-15 (hand tests):** every observed result in this question was got on the Creator's Mac (macOS 26.6.2, Apple M1 Pro) and nowhere else; no Linux or Windows machine was available in that sitting, so whether the same holds on another operating system is still open.

## Checks a person with Blender 5.3 can settle by hand

*Pruned 2026-11-15 after the hand tests: these checks were run on the Creator's Mac in Blender 5.3.0, from the terminal with no window, and their results now sit as observed findings under the questions above — check 1 as hand test 3, check 2 as hand tests 1 and 2, check 3 as hand test 4, check 4 as hand test 5 and check 5 as hand tests 6 and 4. What is left:*

1. **Question 1 — the window.** Every load warning as the window shows it, and the colour space name, view and display it displays, for a file whose names are missing from the configuration in use.
2. **Question 2 — the window.** Render Properties → Color Management → Advanced, Source and Path, set against `ocio_config_source` and `ocio_config_path`.
3. **Question 4 — the Image Editor.** Give the Image Editor deliberately different frame settings from the node and render, to see which the render follows.
4. **Question 5 — an animated Offset.** Repeat the range listing with an animated or driven Offset, through the evaluated node and a `frame_change_post` log during rendering.
5. **Every question — another operating system.** The same checks on Linux and on Windows (naming the Windows edition).

2026-11-15 — GPT-6.1-Sol High (Codex) — Colour spaces and image sequences, Blender 5.3.

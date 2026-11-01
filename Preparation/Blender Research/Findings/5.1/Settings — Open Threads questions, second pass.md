# Settings — Open Threads questions, second pass (Blender 5.1)

**Sources consulted, and when.** Blender's 5.1 manual (`Blender Sources/manual_5.1`, including two of its pictures), Blender's 5.1 Python API documentation (`Blender Sources/api_5.1`), the 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files as shipped with 5.1 (`Blender Sources/cycles_5.1`). All four were taken on **2026-07-30**, and that is the date read for everything quoted below. I did the searching and reading on 2026-08-30.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

No helper agents were used. Everything below was searched, opened and quoted by me.

Questions answered: **18 and 19 only**.

A note on how Cycles' settings are cited. Cycles is an add-on, and its settings are defined in its own Python file rather than described in Blender's Python API documentation. I checked this: `bpy.types.Scene` and `bpy.types.ViewLayer` name the pointers (`cycles`, of types `CyclesRenderSettings` and `CyclesRenderLayerSettings`), but the API documentation has no page for either type — a search of all 1,899 files in `api_5.1` for `CyclesRenderSettings` and `CyclesViewLayer` returns nothing but those two pointer entries. So for Cycles the detail comes from `properties.py` and `ui.py`, quoted as **observed**, and from the manual, quoted as **documented**.

---

## 18. What values will a render's sample count be set to, and what does a render do when it is given a value outside those?

**Note on the reading I took.** "A render's sample count" is not one setting in 5.1 — each engine has its own, a view layer can carry its own, and Cycles has several further counts beside the main one. I answered the question for every sample count the sources present as a render's sample count, and I say for each one what values it takes. The other reading — that only the Cycles final-render Max Samples is meant — is answered by the first block below on its own.

### 18a. The values each sample count accepts

**LABEL: observed** (Cycles, from Cycles' own files) **and documented** (EEVEE, the view layer and Workbench, from the API documentation and the manual).

**Cycles, scene level.** Five sample counts, each an integer property with a hard floor and a hard ceiling written into its definition. `1 << 24` is 16,777,216.

Exact lines, `Blender Sources/cycles_5.1/properties.py`, lines 479–512, inside the class `CyclesRenderSettings`:

```
    samples: IntProperty(
        name="Samples",
        description="Number of samples to render for each pixel",
        min=1, max=(1 << 24),
        default=4096,
    )
    preview_samples: IntProperty(
        name="Viewport Samples",
        description="Number of samples to render in the viewport, unlimited if 0",
        min=0,
        soft_min=1,
        max=(1 << 24),
        default=1024,
    )

    use_sample_subset: BoolProperty(
        name="Use Sample Subset",
        description="Render a subset of the specified max samples. Typically used for distributed rendering across multiple devices",
        default=False,
    )

    sample_offset: IntProperty(
        name="Sample Subset Offset",
        description="0-based index of sample to start rendering from",
        min=0, max=(1 << 24),
        default=0,
    )

    sample_subset_length: IntProperty(
        name="Sample Subset Length",
        description="The number of samples to render in this subset",
        min=1, max=(1 << 24),
        default=2048,
    )
```

So: the final-render sample count accepts **1 to 16,777,216**, and starts at 4096. The viewport count accepts **0 to 16,777,216** and starts at 1024, with zero carrying a meaning of its own — "unlimited if 0". Its `soft_min=1` is a user-interface floor only, not a limit on the value (see 18b).

The adaptive-sampling floor is a sixth count, with a much lower ceiling. Same file, lines 580–586:

```
    adaptive_min_samples: IntProperty(
        name="Adaptive Min Samples",
        description="Minimum AA samples for adaptive sampling, to discover noisy features before stopping sampling. Zero for automatic setting based on noise threshold",
        min=0,
        max=4096,
        default=0,
    )
```

It accepts **0 to 4096**, and zero again means something rather than nothing. The viewport twin, `preview_adaptive_min_samples`, has the same `min=0, max=4096, default=0` (lines 602–608).

The manual says the same about the two zeroes, in words. Manual, Cycles Sampling page: "Setting this value to zero enables indefinite sampling." (of Viewport (Max) Samples), and "When set to 0 (default), Cycles automatically picks a value determined by the Noise Threshold." (of Min Samples). Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html`, date read 2026-07-30.

**EEVEE, scene level.** Documented in the API. From `bpy.types.SceneEEVEE`:

- "taa_render_samples … Number of samples per pixel for rendering (in [1, inf], default 64)"
- "taa_samples … Number of samples, unlimited if 0 (in [0, inf], default 16)"

Source: Python API, `https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html`, date read 2026-07-30. The documentation writes the ceiling as "inf"; it does not give a number for it, and I found no page in `api_5.1` explaining what "inf" stands for in these range notes.

**The view layer's own count.** From `bpy.types.ViewLayer`:

"samples

Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)"

Source: Python API, `https://docs.blender.org/api/5.1/bpy.types.ViewLayer.html`, date read 2026-07-30.

Whether that override is honoured is itself a setting, in Cycles. `Blender Sources/cycles_5.1/properties.py`, lines 64–68:

```
enum_use_layer_samples = (
    ('USE', "Use", "Per render layer number of samples override scene samples"),
    ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
    ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
)
```

and the manual's plain-words version, Cycles Sampling page, under "Layer Samples": "Use: Allow view layers to override the scene-level sample count. / Bounded: Allow the overrides as long as they don't exceed the scene-level sample count. / Ignore: Ignore the overrides." Same page and date as above. The manual's View Layer Override page adds: "This is controlled by the Layer Samples when using the Cycles renderer. When using EEVEE, the Samples override is always used." Source: manual, `https://docs.blender.org/manual/en/5.1/render/layers/override.html`, date read 2026-07-30.

**Workbench.** Its sample count is not a number at all but a choice from a fixed list, so a value outside the list cannot be typed in. Manual, Workbench Sampling page: "The number of samples are predefined so it uses the best distribution of the samples." followed by the list "5, 8, 11, 16, 32". Source: manual, `https://docs.blender.org/manual/en/5.1/render/workbench/sampling.html`, date read 2026-07-30. The API states the same choices: "render_aa … Method of anti-aliasing when rendering final image (default '8')" with items "OFF", "FXAA", "5", "8", "11", "16", "32", typed as "Literal['OFF', 'FXAA', '5', '8', '11', '16', '32']". Source: Python API, `https://docs.blender.org/api/5.1/bpy.types.SceneDisplay.html`, date read 2026-07-30.

**Not covered by 18a.** The API's "inf" ceiling for the EEVEE and view-layer counts is not resolved to a number anywhere I could find, so I cannot say what the largest EEVEE sample count is. I did not find any statement of what these counts mean in samples actually traced when adaptive sampling, a time limit or a sample subset is also in play — that is a question about the render, not about the setting. Nothing here says whether a stored file that already holds an out-of-range value is corrected on load.

### 18b. What happens when a value outside those is given

Three routes exist for giving a sample count a value, and the sources speak clearly about two of them.

**Route one — typed into the interface. LABEL: documented.**

"Most numerical values are restricted by "soft limit" and "hard limit" value ranges. Changing values by dragging with the mouse is restricted to the "soft limit" value range. Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit"."

Source: manual, `https://docs.blender.org/manual/en/5.1/interface/controls/buttons/fields.html`, date read 2026-07-30. So a typed sample count cannot leave the hard range at all; the field will not take it. This says nothing about what is shown or reported when you try.

**Route two — assigned from Python. LABEL: inferred.**

The documented half. `bpy.props.IntProperty`, the function every Cycles sample count is defined with, describes its own `min` and `max` this way:

"min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead.

max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead."

Source: Python API, `https://docs.blender.org/api/5.1/bpy.props.html`, date read 2026-07-30.

The observed half. Cycles' sample counts are `IntProperty` definitions with `min` and `max` given — quoted in full in 18a from `Blender Sources/cycles_5.1/properties.py`, lines 479–512 and 580–586.

**The step I took, written out:** the API documentation describes what `min` and `max` do for any property made with `IntProperty`; Cycles' sample counts are made with `IntProperty` and carry those two arguments; therefore assigning 0, or −5, to Cycles' final-render sample count from Python stores 1, and assigning 100,000,000 stores 16,777,216, with nothing said about it. A reader has to join those two facts, so this is **inferred**, not documented. I found no page that states the outcome for Cycles' sample count in so many words.

**Route three — handed in at launch, from outside Blender. LABEL: documented (an absence).**

The manual's list of command-line arguments has a "Cycles Render Options" section, and it holds exactly two entries: "--cycles-device <device> … Set the device used for rendering." and "--cycles-print-stats … Log statistics about render memory and time usage." Source: manual, `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html`, date read 2026-07-30. Cycles' own argument parser agrees — `Blender Sources/cycles_5.1/engine.py`, lines 8–21, defines the parser and adds those same two arguments and no others:

```
def _configure_argument_parser():
    import argparse
    # No help because it conflicts with general Python scripts argument parsing
    parser = argparse.ArgumentParser(description="Cycles Addon argument parser",
                                     add_help=False)
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
    parser.add_argument("--cycles-device",
```

(observed). Since no launch argument carries a sample count, no launch argument can carry an out-of-range one either; the out-of-range case at launch can only arrive through route two.

**What the sources do not say. LABEL: not in the sources.**

- Nothing states what happens when the **EEVEE** counts (`taa_render_samples`, `taa_samples`) or the **view layer's** `samples` are assigned a value below their stated floor. These are defined inside Blender itself, not by `bpy.props`, and the `bpy.props` wording quoted above is about properties made with that function. I will not carry it across to them.
- Nothing states what is **reported** — no message, warning, error text or return value is described for an out-of-range sample count anywhere I searched.
- Nothing states what happens if a sample count is given something that is not a whole number at all (a word, a decimal, nothing).
- Nothing states what happens when Workbench's `render_aa` is given a value outside its seven choices.

For contrast, the manual does spell out clamping when it wants to: of a video-encoding setting it says "CRF values outside the allowed range are clamped to the nearest allowed value." (manual, `https://docs.blender.org/manual/en/5.1/render/output/properties/output.html`, date read 2026-07-30). No comparable sentence exists for any sample count.

**Not covered by 18b.** Whether the clamped value is written back into the file, or only used for that render. What the render prints, if anything. Whether a scene saved by an older version with a larger stored count is clamped on load — I did see, in `Blender Sources/cycles_5.1/version_update.py` lines 242–251, code that multiplies old files' sample counts by themselves when converting away from "square samples" (`cscene.samples *= cscene.samples`, and `layer.samples *= layer.samples`), which could exceed the ceiling, but what then happens is exactly the unstated thing above, so I draw no conclusion.

### Outside the interface (question 18)

The floors and ceilings are part of how each setting is defined, in one file that Blender reads whether or not a window is open, so they hold when Blender is driven from outside — **inferred** from the definitions quoted above, with the step being that a property definition is not a piece of user interface. Two differences I can name: the "soft limit" behaviour, which is about dragging and typing in a field, has no counterpart outside the interface at all (documented, from the fields page quoted above); and the viewport-only counts (`preview_samples`, `preview_adaptive_min_samples`, the EEVEE `taa_samples`) describe the 3D viewport, which a render driven from outside is not using. The sources do not say whether an out-of-range assignment behaves differently in a session with no window.

### Operating system (question 18)

The sources say nothing that makes any sample count's accepted values depend on the operating system. The values are given in one file that carries no operating-system condition around them (observed, `Blender Sources/cycles_5.1/properties.py`), and the API's ranges for EEVEE and the view layer carry no such condition either (documented). I searched the manual and the API for any operating-system note attached to a sample count and found none. This is an absence, not a statement that they are the same everywhere.

### Where I looked, for question 18

Searched (case-insensitive, across whole folders unless noted):

- `Blender Sources/cycles_5.1`: `samples` in `properties.py` (28 hits, all read); `sample`, `denois`, `argv`, `argparse`, `cycles-` across all its Python files; `num_samples`, `sample_count`, `min_samples`, `max_samples`, `adaptive_min_samples` across `cycles_5.1/source` (271 files — the kernel and utility headers). The last search returned nothing: the code that decides how many samples are actually traced is not among the files shipped with the add-on, so I could not settle anything from it.
- `Blender Sources/manual_5.1`: `samples` across `render/` (24 files listed, of which I opened `render/cycles/render_settings/sampling.txt`, `render/eevee/render_settings/sampling.txt`, `render/workbench/sampling.txt`, `render/layers/passes.txt`, `render/layers/override.txt`, `render/cycles/optimizations/reducing_noise.txt`); `sample count`, `Max Samples`, `minimum number of samples`; `soft limit`, `hard limit`, `soft_min`; `clamped to`, `invalid value`, `out of range`, `must be at least`, `16777216`; `sample` across `advanced/command_line/` (no hits at all); `advanced/limits.txt` read in full, which covers space and time limits and says nothing of samples.
- `Blender Sources/api_5.1` (1,899 files): `samples` (file list scanned; `bpy.types.SceneEEVEE.txt`, `bpy.types.ViewLayer.txt`, `bpy.types.SceneDisplay.txt` opened); `clamp` across everything (the `bpy.props` entries were the only relevant hits — the rest are node and modifier settings); `out of range`, `TypeError`, `ValueError` across the `info_*` guide pages and `bpy.types.bpy_struct.txt`; `CyclesRenderSettings`, `CyclesViewLayer` in the type index.
- `Blender Sources/release_notes_5.1` (all 21 files): `sample` — seven hits, all read, none about ranges or limits (they concern a sculpt-mode bias unit, a Grease Pencil "Simplify Stroke → Sample" length, geometry-node sample indices, a Directional Blur node, a Vulkan sampler crash, and a renamed `sample_color` operator).

Ruled out and why: the compositing pages (Denoise node) carry no sample count; `render/output/` covers output and encoding, not sampling; the Workbench and EEVEE pages were read in full and give no numeric ceiling; `advanced/limits.txt` is the manual's page on working limits and covers only coordinate precision and frame count.

Pictures: I looked for a screenshot of Cycles' Sampling panel, which could have shown the fields directly. `render/cycles/render_settings/sampling.txt` contains no `[image: …]` marker at all, and a scan of the `_images` folder for names containing "sampling" or "denois" returned only a legacy texture panel, two denoising before/after example renders, a compositing example, and the Denoise node's own icon — none of them a sampling panel. So no picture bears on this question.

---


---

## Hand-test results — question 18 (2026-08-30)

**On what computers.** Two, and their results are kept apart.

- **The Creator's own MacBook Pro**, M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no second card. Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, build hash `ec6e62d40fa9`.
- **A Linux machine rented for these tests**: Vast.ai instance 49253900, machine 27187, in Poland — a container rather than a whole machine (`systemd-detect-virt` answers `docker`, process 1 is `/usr/bin/dash`, no service manager), Ubuntu 22.04.4 LTS on kernel 6.8.0-48-generic, an AMD Ryzen 5 5600X with 5.76 cores allotted against 12 visible threads, a memory ceiling of about 21.2 GiB against the 31 GB the ordinary reading reports, a 45 GB overlay root, and **one NVIDIA GeForce RTX 3060 with 12288 MiB on driver 550.120**. It had **no display server as rented**; one was installed for the panel test and it draws in software (Xvfb, OpenGL renderer `llvmpipe`). Blender 5.1.2 at `/opt/blender/5.1/blender`, **build hash `ec6e62d40fa9` — the same build as the Mac's**, checked before any test was run.

Every run used a throwaway settings folder made beforehand and `--factory-startup`, so none of the Creator's own settings was read or touched. Every Cycles render was on the processor except where a card is named.


### What values the sample counts take

**Done.** With no file open, read every sample count's declared floor, ceiling, dragging bounds and starting value out of the running program.

**Came back**, identically on both machines:

```
E1 scene.cycles.samples hard_min=1 hard_max=16777216 soft_min=1 soft_max=16777216 default=4096 current=4096
E1 scene.cycles.preview_samples hard_min=0 hard_max=16777216 soft_min=1 soft_max=16777216 default=1024 current=1024
E1 scene.cycles.sample_offset hard_min=0 hard_max=16777216 soft_min=0 soft_max=16777216 default=0 current=0
E1 scene.cycles.sample_subset_length hard_min=1 hard_max=16777216 soft_min=1 soft_max=16777216 default=2048 current=2048
E1 scene.cycles.adaptive_min_samples hard_min=0 hard_max=4096 soft_min=0 soft_max=4096 default=0 current=0
E1 scene.cycles.preview_adaptive_min_samples hard_min=0 hard_max=4096 soft_min=0 soft_max=4096 default=0 current=0
E1 scene.cycles.guiding_training_samples hard_min=0 hard_max=2147483647 soft_min=1 soft_max=2147483647 default=128 current=128
E1 scene.cycles.preview_denoising_start_sample hard_min=0 hard_max=16777216 soft_min=0 soft_max=16777216 default=1 current=1
E1 scene.eevee.taa_render_samples hard_min=1 hard_max=2147483647 soft_min=1 soft_max=2147483647 default=64 current=64
E1 scene.eevee.taa_samples hard_min=0 hard_max=2147483647 soft_min=0 soft_max=2147483647 default=16 current=16
E1 view_layer.samples hard_min=0 hard_max=2147483647 soft_min=0 soft_max=10000 default=0 current=0
E1 scene.grease_pencil_settings.aa_samples hard_min=1 hard_max=2147483647 soft_min=1 soft_max=256 default=8 current=8
E1 scene.display.render_aa items=['OFF', 'FXAA', '5', '8', '11', '16', '32'] default='8' current='8'
```

**What that settles for this answer.**

- The upper end this document records as **"inf"** for the EEVEE counts, the view layer's override and the Grease Pencil count **is 2 147 483 647**.
- **`guiding_training_samples` does have a maximum** — 2 147 483 647, the whole-number type's own ceiling — where its declaration writes none.
- Two dragging bounds appear that no source states: **the view layer's override cannot be dragged past 10 000**, and the Grease Pencil count not past 256.
- Every Cycles floor and ceiling this document quotes from Cycles' own file is what the running program holds.
- **Every count named in only some versions' documents is present, with the same limits, in all four versions**, so no version's silence about any of them was a difference in Blender.
- One thing to read carefully: the Grease Pencil line ends `current=8` here and `current=1` in 4.5. **That is the value in each version's startup file, not a limit**; the declared starting value is 8 in all four.

### What a value outside the range does

**Done.** Assigned 0, −5, 16 777 217, 99 999 999, 2 147 483 647, 2 147 483 648 and 10¹² to each count in turn and read the value back, in one run that also printed two lines of its own and attempted a deliberately wrong assignment, so that "nothing was reported" could be told apart from "nothing was captured".

**Came back**, on the Creator's Mac:

```
E2 CONTROL-STDOUT this line proves standard output is captured
E2 CONTROL-STDERR this line proves standard error is captured
E2 CONTROL-RAISES AttributeError: 'CyclesRenderSettings' object has no attribute 'samples_no_such_setting'
E2 scene.cycles.samples = 0 -> stored 1 (no error)
E2 scene.cycles.samples = -5 -> stored 1 (no error)
E2 scene.cycles.samples = 16777217 -> stored 16777216 (no error)
E2 scene.cycles.samples = 99999999 -> stored 16777216 (no error)
E2 scene.cycles.samples = 2147483647 -> stored 16777216 (no error)
E2 scene.cycles.samples = 2147483648 -> REFUSED ValueError: bpy_struct: item.attr = val: CyclesRenderSettings.samples value not in 'int' range ((-2147483647 -1), 2147483647)
E2 scene.cycles.samples = 1000000000000 -> REFUSED ValueError: bpy_struct: item.attr = val: CyclesRenderSettings.samples value not in 'int' range ((-2147483647 -1), 2147483647)
E2 scene.cycles.preview_samples = -5 -> stored 0 (no error)
E2 scene.cycles.adaptive_min_samples = 99999999 -> stored 4096 (no error)
E2 scene.cycles.sample_subset_length = 0 -> stored 1 (no error)
```

and the counts this document records as **not in the sources** behave the same way:

```
E2 scene.eevee.taa_render_samples = 0 -> stored 1 (no error)
E2 scene.eevee.taa_render_samples = 99999999 -> stored 99999999 (no error)
E2 scene.eevee.taa_render_samples = 2147483648 -> REFUSED ValueError: bpy_struct: item.attr = val: SceneEEVEE.taa_render_samples value not in 'int' range ((-2147483647 -1), 2147483647)
E2 view_layer.samples = -5 -> stored 0 (no error)
E2 scene.grease_pencil_settings.aa_samples = 0 -> stored 1 (no error)
```

**The rented Linux machine gave the same values in every case**, and differed only in the wording of the refusal: it prints `value not in 'int' range ((-0x7fffffff - 1), 0x7fffffff)` where the Mac prints `((-2147483647 -1), 2147483647)`. **A program reading Blender's output for that refusal must match both shapes.**

**What that settles.** The silent correction this document infers from `bpy.props` is what happens — and it happens for the EEVEE, view-layer and Grease Pencil counts too, which this document could not settle. **Nothing at all is reported.** A number too large for a whole number is not corrected but refused, with a `ValueError`.

### What a value that is not a whole number does

**Done.** Assigned a decimal, text, text that looks like a number, nothing, and a true-or-false value to each count; and a choice outside the list to Workbench's anti-aliasing setting, with valid choices in the same run as a control.

**Came back**, the same on both machines:

```
E3 scene.cycles.samples = 4.7 -> REFUSED TypeError: bpy_struct: item.attr = val: CyclesRenderSettings.samples expected an int type, not float (still 4096)
E3 scene.cycles.samples = 'many' -> REFUSED TypeError: bpy_struct: item.attr = val: CyclesRenderSettings.samples expected an int type, not str (still 4096)
E3 scene.cycles.samples = '64' -> REFUSED TypeError: bpy_struct: item.attr = val: CyclesRenderSettings.samples expected an int type, not str (still 4096)
E3 scene.cycles.samples = None -> REFUSED TypeError: bpy_struct: item.attr = val: CyclesRenderSettings.samples expected an int type, not NoneType (still 4096)
E3 scene.cycles.samples = True -> stored 1 (no error, was 4096)
E3 scene.display.render_aa = '7' -> REFUSED TypeError: bpy_struct: item.attr = val: enum "7" not found in ('OFF', 'FXAA', '5', '8', '11', '16', '32') (still '8')
E3 scene.display.render_aa = 'OFF' -> stored 'OFF' (no error)
E3 scene.display.render_aa = 'fxaa' -> REFUSED TypeError: bpy_struct: item.attr = val: enum "fxaa" not found in ('OFF', 'FXAA', '5', '8', '11', '16', '32') (still '32')
```

The same refusals came back for the EEVEE, view-layer and Grease Pencil counts.

**What that settles.** A value that is not a whole number is **refused outright, never rounded and never truncated** — which this document records as unstated. A choice outside a fixed list is refused too, and the refusal names every real choice. Matching a choice is case-sensitive. A true-or-false value is the one exception: it is accepted and stores 1.

### What sample count actually reached the render

This is a different claim from the two above, and it is the one this document could not reach: what the *setting* stores is not what the *render* works to.

**Done.** Built the test scene, set the count out of range, and rendered one frame at 64 by 48 with adaptive sampling off, keeping everything Blender printed.

**Came back**, on the Creator's Mac in 5.1.2:

```
E6 asked=99999999 stored=16777216 (no error)
E6 engine=CYCLES setting=samples blender=5.1.2
E6 RENDER-STARTS
00:00.352  render           | Fra: 1 | Mem: 33M | Sample 0/16777216
00:00.354  render           | Fra: 1 | Remaining: 08:54:23.99 | Mem: 33M | Sample 1/16777216
RUNCAP exit=-9 killed_at_25.0s=True

(This run printed only two progress lines before the twenty-five-second limit stopped it; 5.0 and 5.2 reached the 8193rd sample in the same time. Nothing turns on that — the number after the slash is the answer, and it is the same.)
```

and the case at the other end, as a control:

```
E6 asked=0 stored=1 (no error)
E6 engine=CYCLES setting=samples blender=5.1.2
E6 RENDER-STARTS
00:00.569  render           | Fra: 1 | Mem: 2M | Sample 0/1
00:00.576  render           | Fra: 1 | Mem: 2M | Sample 1/1
00:00.650  render           | Saved: '.../e6-5.1/e6_0001.png'
RUNCAP exit=0 killed_at_25.0s=False
```

EEVEE, whose count is not corrected at 99 999 999, works to the number it was given: `Rendering 1 / 99999999 samples`. **The rented Linux machine gave the same counts in every case.**

**What that settles.** **The corrected number is the number the render works to.** A count above the ceiling does not fail, does not warn and does not stop — it settles in for a fourteen-hour render at 16 777 216 samples. The only place the corrected figure appears is the ordinary progress line.

**Outside the interface — brought up to date by the hand tests of 2026-08-30.** Every result above was got from a Blender started with no window, driven by a script handed in at launch. So the correction, the refusals and the count the render works to are all **observed** for exactly the case this project cares about. What is still untested outside the window is the soft-limit half of the number-field rule, which has no counterpart there.

**Operating system — brought up to date by the hand tests of 2026-08-30.** Every accepted value, every correction and every refusal came back the same on macOS and on Linux. **One thing does differ, and it is the wording**: the refusal of a number too large for a whole number reads `((-2147483647 -1), 2147483647)` on macOS and `((-0x7fffffff - 1), 0x7fffffff)` on Linux. Windows is untested; the tests are written out ready to run in `Audits/Blender Research/Settings/Hand tests — Open Threads questions.md`.

## 19. What settings does a scene or a view layer carry that govern denoising, and what does each of them do?

There are four groups: Cycles' own denoising settings on the scene; Cycles' two on the view layer; EEVEE's ray-tracing denoising settings on the scene; and the scene's settings for the compositor's Denoise nodes. EEVEE's view layer (`bpy.types.ViewLayerEEVEE`) carries none — I read that page in full and it holds only an ambient-occlusion distance and three pass toggles.

### 19a. Cycles, on the scene

**LABEL: observed** for the settings, their kinds and their defaults; **documented** for what several of them do.

All from `Blender Sources/cycles_5.1/properties.py`, class `CyclesRenderSettings`, lines 400–477. Quoted exactly, with each setting's own description:

```
    use_denoising: BoolProperty(
        name="Use Denoising",
        description="Denoise the rendered image",
        default=True,
        update=update_render_passes,
    )
    denoiser: EnumProperty(
        name="Denoiser",
        description="Denoise the image with the selected denoiser. "
        "For denoising the image after rendering",
        items=enum_denoiser,
        default=4,  # Use integer to avoid error in builds without OpenImageDenoise.
        update=update_render_passes,
    )
    denoising_prefilter: EnumProperty(
        name="Denoising Prefilter",
        description="Prefilter noisy guiding (albedo and normal) passes to improve denoising quality when using OpenImageDenoise",
        items=enum_denoising_prefilter,
        default='ACCURATE',
    )
    denoising_quality: EnumProperty(
        name="Denoising Quality",
        description="Overall denoising quality when using OpenImageDenoise",
        items=enum_denoising_quality,
        default='HIGH',
    )
    denoising_input_passes: EnumProperty(
        name="Denoising Input Passes",
        description="Passes used by the denoiser to distinguish noise from shader and geometry detail",
        items=enum_denoising_input_passes,
        default='RGB_ALBEDO_NORMAL',
    )
    denoising_use_gpu: BoolProperty(
        name="Denoise on GPU",
        description="Perform denoising on GPU devices configured in the system tab in the user preferences. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled",
        default=False,
    )
```

and, immediately after, the viewport set — six more settings that govern denoising of the 3D viewport rather than of the render:

```
    use_preview_denoising: BoolProperty(
        name="Use Viewport Denoising",
        description="Denoise the image in the 3D viewport",
        default=False,
    )
    preview_denoiser: EnumProperty(
        name="Viewport Denoiser",
        description="Denoise the image after each preview update with the selected denoiser",
        items=enum_preview_denoiser,
        default=0,
    )
    preview_denoising_prefilter: EnumProperty(
        name="Viewport Denoising Prefilter",
        description="Prefilter noisy guiding (albedo and normal) passes to improve denoising quality when using OpenImageDenoise",
        items=enum_denoising_prefilter,
        default='FAST',
    )
    preview_denoising_quality: EnumProperty(
        name="Viewport Denoising Quality",
        description="Overall denoising quality when using OpenImageDenoise",
        items=enum_denoising_quality,
        default='BALANCED',
    )
    preview_denoising_input_passes: EnumProperty(
        name="Viewport Denoising Input Passes",
        description="Passes used by the denoiser to distinguish noise from shader and geometry detail",
        items=enum_denoising_input_passes,
        default='RGB_ALBEDO',
    )
    preview_denoising_start_sample: IntProperty(
        name="Start Denoising",
        description="Sample to start denoising the preview at",
        min=0, max=(1 << 24),
        default=1,
    )
    preview_denoising_use_gpu: BoolProperty(
        name="Denoise Preview on GPU",
        description="Perform denoising on GPU devices configured in the system tab in the user preferences. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled",
        default=True,
    )
```

The choices each of the three enumerations offers are defined in the same file, lines 306–338:

```
enum_denoising_input_passes = (
    ('RGB', "None", "Don't use utility passes for denoising", 1),
    ('RGB_ALBEDO', "Albedo", "Use albedo pass for denoising", 2),
    ('RGB_ALBEDO_NORMAL', "Albedo and Normal", "Use albedo and normal passes for denoising", 3),
)

enum_denoising_prefilter = (
    ('NONE',
     "None",
     "No prefiltering, use when guiding passes are noise-free",
     1),
    ('FAST',
     "Fast",
     "Denoise color and guiding passes together. Improves quality when guiding passes are noisy using least amount of extra processing time",
     2),
    ('ACCURATE',
     "Accurate",
     "Prefilter noisy guiding passes before denoising color. Improves quality when guiding passes are noisy using extra processing time",
     3),
)

enum_denoising_quality = (
    ('HIGH',
     "High",
     "High quality",
     1),
    ('BALANCED',
     "Balanced",
     "Balanced between performance and quality",
     2),
    ('FAST',
     "Fast",
     "High performance",
```

The **Denoiser** choice is unusual: its list is not fixed, but built by a function each time. Same file, lines 265–303:

```
def enum_openimagedenoise_denoiser(self, context):
    import _cycles
    if _cycles.with_openimagedenoise:
        return [('OPENIMAGEDENOISE', "OpenImageDenoise",
                 n_("Use Intel OpenImageDenoise AI denoiser"), 4)]
    return []


def enum_optix_denoiser(self, context):
    if not context or bool(context.preferences.addons[__package__].preferences.get_devices_for_type('OPTIX')):
        return [('OPTIX', "OptiX", n_(
            "Use the OptiX AI denoiser with GPU acceleration, only available on NVIDIA GPUs when configured in the system tab in the user preferences"), 2)]
    return []
```

and for the viewport, an extra "Automatic" entry, lines 280–296:

```
    if len(optix_items) or len(oidn_items):
        items = [
            ('AUTO',
             "Automatic",
             n_("Use GPU accelerated denoising if supported, for the best performance. "
                "Prefer OpenImageDenoise over OptiX"),
             0)]
    else:
        items = [('AUTO', "None", n_("Blender was compiled without a viewport denoiser"), 0)]
```

Reading that as behaviour is a step past what the file states, so: **inferred** — which denoisers a scene can be set to depends on the build and on the machine's configured devices, since OpenImageDenoise appears only when the build has it and OptiX appears only when OptiX devices are configured (or when there is no context). The step is that a list built by a function is the list the setting can hold.

The manual describes the same settings in plain words, under Render ‣ Sampling ‣ Denoising. Quoting the ones that carry the meaning:

- "Denoising uses a specialized algorithm to get a less noisy image without requiring more samples."
- "Denoise (Viewport) — Whether to perform denoising for the 3D Viewport in the Rendered shading mode."
- "Denoise (Render) — Whether to perform denoising for the final rendered image."
- "Denoiser — The algorithm to use." with "Automatic: Uses GPU accelerated denoising if supported, for best performance. Prefers OpenImageDenoise over OptiX.", "OpenImageDenoise: Uses Intel's Open Image Denoise, an AI denoiser. Typically provides the highest quality.", and "OptiX: Uses NVIDIA's OptiX AI denoiser. Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not."
- "Passes — Controls which Render Passes the denoiser should use as input. Generally, the more passes the denoiser has access to, the better the result. It is recommended to use at least Albedo as None can blur out details, especially at lower sample counts."
- "Prefilter OpenImageDenoise — Controls whether prefiltering is applied to the Passes before denoising. Visible only when using OpenImageDenoise."
- "Quality OpenImageDenoise — Overall denoising quality. Visible only when using OpenImageDenoise."
- "Start Sample — Sample at which to start denoising in the 3D Viewport."
- "Use GPU — Perform denoising on the GPU. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled."

Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html`, date read 2026-07-30. **LABEL for this block: documented.**

The manual also notes an alternative route these settings feed: "If you enable the Denoising Data render pass, you can alternatively denoise in the compositing step using the Denoise Node." (same page).

### 19b. Cycles, on the view layer

**LABEL: observed.** `Blender Sources/cycles_5.1/properties.py`, class `CyclesRenderLayerSettings`, lines 1532–1543 — the only two denoising settings the class holds:

```
    use_denoising: BoolProperty(
        name="Use Denoising",
        description="Denoise the rendered image",
        default=True,
        update=update_render_passes,
    )
    denoising_store_passes: BoolProperty(
        name="Store Denoising Passes",
        description="Store the denoising feature passes and the noisy image. The passes adapt to the denoiser selected for rendering",
        default=False,
        update=update_render_passes,
    )
```

The first is a per-view-layer switch for denoising that layer; the second makes the render keep the denoiser's input passes and the un-denoised image alongside the result.

How they are presented tells you a little more. `Blender Sources/cycles_5.1/ui.py`, lines 985–991, in the view layer's Filter panel:

```
        sub = col.row()
        sub.prop(view_layer.cycles, "use_denoising", text="Denoising")
        sub.active = scene.cycles.use_denoising
```

**LABEL: inferred** — the view layer's denoising switch is subordinate to the scene's: the row holding it is marked inactive whenever the scene's `use_denoising` is off. The step is reading a greyed-out control as meaning the setting has no effect, which the file itself does not say. What the file states outright (observed) is only that the row's `active` flag is the scene setting.

The same file, line 1029, puts the second one among the render passes: `col.prop(cycles_view_layer, "denoising_store_passes", text="Denoising Data")`. That name matches the manual's pass list: "Denoising Data Cycles — Includes Denoising Albedo, Denoising Normal, and the combined image before denoising. Can be used with the Denoise Node as a replacement for automatic denoising." Source: manual, `https://docs.blender.org/manual/en/5.1/render/layers/passes.html`, date read 2026-07-30 (**documented**).

I could not find the per-view-layer **Denoising** switch itself described anywhere in the manual — see the record of looking below.

### 19c. EEVEE, on the scene

**LABEL: documented.** EEVEE's denoising settings sit on the scene's ray-tracing options (`bpy.types.SceneEEVEE.ray_tracing_options`, of type `RaytraceEEVEE`), and one more on the scene's EEVEE settings directly. From `bpy.types.RaytraceEEVEE`, "Quality options for the raytracing pipeline":

- "use_denoise — Enable noise reduction techniques for raytraced effects (default True)"
- "denoise_spatial — Reuse neighbor pixels' rays (default True)"
- "denoise_temporal — Accumulate samples by reprojecting last tracing results (default True)"
- "denoise_bilateral — Blur the resolved radiance using a bilateral filter (default True)"

Source: Python API, `https://docs.blender.org/api/5.1/bpy.types.RaytraceEEVEE.html`, date read 2026-07-30.

From `bpy.types.SceneEEVEE`: "use_taa_reprojection — Denoise image using temporal reprojection (can leave some ghosting) (default True)". Source: Python API, `https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html`, date read 2026-07-30. The manual calls this one "Temporal Reprojection — Reduces noise while moving the viewport or during animation playback. Can leave some ghosting.", and places it under Render ‣ Sampling ‣ Viewport. Source: manual, `https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/sampling.html`, date read 2026-07-30.

The manual's own words for the four ray-tracing ones, under Render ‣ Raytracing ‣ Denoising:

"Denoising can be enabled to reduce the amount of noise from the raw ray-traced output. This can help image stability but will also over-blur the final ray-traced output.

Spatial Reuse — Reuse the rays from neighbor pixels. Can introduce some light leaks across surfaces.

Temporal Accumulation — Accumulate samples by re-projecting the last ray tracing results. This removes Fireflies but also introduces color bias. Useful for viewport temporal stability or making renders converge faster.

Bilateral Filter — Blur the resolved ray-traced output using a bilateral filter."

Source: manual, `https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/raytracing.html`, date read 2026-07-30. The same page states that the whole ray-tracing pipeline can be switched off ("When disabled, it is replaced by a faster pipeline that uses pre-filtered light-probes."), which matches the API's "use_raytracing — Enable the ray-tracing module (default False)".

### 19d. The scene's settings for the compositor's Denoise nodes

These sit on `bpy.types.RenderSettings` — that is, on `scene.render` — and govern denoising done by Denoise nodes rather than by the render engine. **LABEL: documented.**

- "compositor_denoise_device — The device to use to process the denoise nodes in the compositor (default 'AUTO')", with "AUTO — Auto – Use the same device used by the compositor to process the denoise node.", "CPU — Use the CPU to process the denoise node.", "GPU — Use the GPU to process the denoise node if available, otherwise fallback to CPU."
- "compositor_denoise_final_quality — The quality used by denoise nodes during the compositing of final renders if the nodes' quality option is set to Follow Scene (default 'HIGH')", choices High, Balanced, Fast.
- "compositor_denoise_preview_quality — The quality used by denoise nodes during viewport and interactive compositing if the nodes' quality option is set to Follow Scene (default 'BALANCED')", choices High, Balanced, Fast.

Source: Python API, `https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html`, date read 2026-07-30.

The manual carries the same three under Render ‣ Performance ‣ Denoise Nodes, on both the Cycles and EEVEE performance pages, naming them "Denoising Device", "Preview Quality" and "Final Quality", and describing the qualities: "High: Produces the highest quality output at the cost of long processing times. / Balanced: Balanced between performance and quality, typically processing in half the time as High, while retaining most of the quality. / Fast: Produces an output quickly at a noticeable cost of quality." Sources: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html` and `https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/performance.html`, date read 2026-07-30. That the scene setting is what a node defers to is stated on the node's own page: under Quality, "Follow Scene: Use the scene's quality setting." Source: manual, `https://docs.blender.org/manual/en/5.1/compositing/types/filter/denoise.html`, date read 2026-07-30.

### One change in this version

Release notes, Cycles: "Improved denoising quality with textured transparency and roughness. (blender/blender!154988)". Source: release notes, `https://developer.blender.org/docs/release_notes/5.1/cycles/`, date read 2026-07-30. That is the only denoising entry in the 5.1 notes outside the bug-fix and corrective-release lists.

### Not covered (question 19)

What any of these settings does to the picture, beyond the words quoted. Which combinations are refused or silently ignored (the manual says the Prefilter and Quality settings are "Visible only when using OpenImageDenoise", which is a statement about the interface, not about what a render does with them when another denoiser is selected). Whether the view layer's denoising switch has any effect when the scene's is off — I could only infer that from a greyed-out control. Nothing here settles what happens when denoising is asked for and no denoiser is available. The `bpy.ops.cycles.denoise_animation` operator ("Denoise rendered animation sequence using current scene and view layer settings. Requires denoising data passes and output to OpenEXR multilayer files", Python API, `https://docs.blender.org/api/5.1/bpy.ops.cycles.html`) reads these settings, but it is an operator, not a setting, so I leave it at the mention. I did not look into Grease Pencil, Freestyle, or the sequencer.

### Outside the interface (question 19)

The settings themselves are scene and view-layer data, defined in files that Blender reads with or without a window, so they exist and hold their values when Blender is driven from outside — **inferred**, the step being that a property on a scene is stored in the file rather than in the interface. Three real differences, all documented or observed:

1. Every setting whose name begins `preview_` — the whole Cycles viewport denoising set, and EEVEE's Temporal Reprojection under Viewport — is described as acting on the 3D viewport ("Denoise the image in the 3D viewport", "Sample to start denoising the preview at"). A render driven from outside is not drawing that viewport. The sources do not say what these settings do in that case; they only say what they are for.
2. `compositor_denoise_preview_quality` is for "viewport and interactive compositing", and `compositor_denoise_final_quality` for "the compositing of final renders" — documented, quoted above. Which one applies is decided by the kind of render, not by how Blender was started, and the sources do not connect either to being driven from outside.
3. The Denoiser choice is built from the machine's configured devices and user preferences (observed, `properties.py` lines 273–277, and the description "only available on NVIDIA GPUs when configured in the system tab in the user preferences"). Those preferences belong to the machine, not the file. The sources do not say how that list is resolved in a session with no window.

I found no statement anywhere in the sources that denoising is switched off, or behaves differently, when Blender is run without its interface.

### Operating system (question 19)

Which denoisers are available differs by operating system and hardware; this is documented, on the GPU Rendering page:

- "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher and a driver version of at least 535."
- "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes all NVIDIA RTX cards."
- "GPU accelerated denoising is available on discrete Radeon RX 6000 and newer." (AMD, "supported on Windows and Linux")
- "Hardware acceleration for ray-tracing and denoising is supported." and "GPU accelerated denoising is available on all supported GPUs." (Intel oneAPI, "supported on Windows and Linux")
- "Metal for GPU acceleration is supported on Apple computers with Apple Silicon." and "GPU accelerated ray-tracing and denoising is available on Apple Silicon."

Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`, date read 2026-07-30.

So: OptiX is a Windows-and-Linux, NVIDIA-only choice, which means the Denoiser setting cannot be set to it on macOS (**inferred**, one step: a denoiser supported only on Windows and Linux with NVIDIA hardware cannot be a choice on a machine that is neither). The settings themselves — the switches, the passes, the prefilter, the quality — are defined without any operating-system condition (observed, `properties.py`), and OpenImageDenoise is noted in Cycles' own interface code as "always available, thanks to CPU support" (`Blender Sources/cycles_5.1/ui.py`, lines 157 and 170). *(Second address corrected by the citation check of 2026-08-30: the comment's two occurrences are at 157 and 170, not 169.)* Whether the *result* of denoising differs between operating systems is not addressed anywhere I searched.

### Where I looked, for question 19

- `Blender Sources/cycles_5.1`: `denois` in `properties.py` (75 hits, every one read in place) and in `ui.py` (all hits read, plus the surrounding panel code at lines 246–290, 320–355, 965–1035); `denois` across `cycles_5.1/source` (the kernel headers) — hits are pass identifiers and feature flags (`KERNEL_FEATURE_DENOISING`, `PASS_DENOISING_ALBEDO`, `pass_denoising_depth` and so on), not settings, so I drew nothing from them; `presets.py` read for which denoising settings a sampling preset stores (`cycles.use_denoising`, `cycles.denoiser`, `cycles.denoising_input_passes`, `cycles.denoising_prefilter`, `cycles.denoising_quality`); `version_update.py` searched for `denois`.
- `Blender Sources/api_5.1`: `denois` across all 1,899 files — sixteen files hit, and I opened every one that could hold a setting: `bpy.types.RenderSettings.txt`, `bpy.types.RaytraceEEVEE.txt`, `bpy.types.SceneEEVEE.txt`, `bpy.ops.cycles.txt`, `bpy.types.CompositorNodeDenoise.txt`. The rest are indexes (`genindex-*`, `bpy.types.txt`), the change log, and node base classes. Also read in full: `bpy.types.ViewLayerEEVEE.txt` (no denoising settings), and the `cycles` pointer entries in `bpy.types.ViewLayer.txt` and `bpy.types.Scene.txt`.
- `Blender Sources/manual_5.1`: `denois` across the whole manual — fourteen files, and I opened all that bear on settings: `render/cycles/render_settings/sampling.txt` (read in full), `render/cycles/render_settings/performance.txt`, `render/cycles/render_settings/index.txt`, `render/cycles/gpu_rendering.txt`, `render/cycles/optimizations/reducing_noise.txt`, `render/eevee/render_settings/raytracing.txt` (read in full), `render/eevee/render_settings/performance.txt`, `render/eevee/render_settings/sampling.txt` (read in full), `render/layers/passes.txt`, `compositing/types/filter/denoise.txt` (read in full). Also read in full, to check for a per-view-layer denoising switch: `render/layers/view_layer.txt` and `render/layers/override.txt` — neither mentions denoising. Additional searches: `store denoising`, `denoising data`, `denoising passes`, `noise reduction`. The remaining `denois` files are `genindex.txt`, `compositing/index.txt`, `compositing/types/filter/index.txt` (tables of contents) and `getting_started/about/history.txt` (a history page).
- `Blender Sources/release_notes_5.1`: `denois` across all 21 files — four hits, all read: the Cycles improvement quoted above, two bug-fix list entries, and one corrective-release entry about a broken Denoise Animation operator.

**The one absence worth stating plainly:** the view layer's own **Denoising** switch (`view_layer.cycles.use_denoising`) is not described in the 5.1 manual. It exists — Cycles' files define it and draw it in the View Layer ‣ Filter panel — but a search of the whole manual for `denois` returns no page under `render/layers/` other than the pass list, and both `view_layer.txt` and `override.txt` are silent on it.

Pictures: I opened `_images/render_layers_view-layer_panel.png`, the manual's screenshot of the View Layer panel, to see whether it shows the per-view-layer denoising switch. It does not — it shows the View Layer panel with "Use for Rendering" and "Render Single Layer", and a Passes panel with Combined, Z, Mist, Normal, Ambient Occlusion, Subsurface Direct and Subsurface Color. **Documented**, from the manual, `https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html`, date read 2026-07-30. What is visible in it is visible; the panel it shows is evidently not the Filter panel where Cycles draws the denoising switch, so it neither confirms nor denies anything else. I also confirmed that the Cycles Sampling manual page carries no `[image: …]` marker at all, so there is no screenshot of the Denoise panel to consult.

---


---

## Hand-test results — question 19 (2026-08-30)

**On what computers.** Two, and their results are kept apart.

- **The Creator's own MacBook Pro**, M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no second card. Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, build hash `ec6e62d40fa9`.
- **A Linux machine rented for these tests**: Vast.ai instance 49253900, machine 27187, in Poland — a container rather than a whole machine (`systemd-detect-virt` answers `docker`, process 1 is `/usr/bin/dash`, no service manager), Ubuntu 22.04.4 LTS on kernel 6.8.0-48-generic, an AMD Ryzen 5 5600X with 5.76 cores allotted against 12 visible threads, a memory ceiling of about 21.2 GiB against the 31 GB the ordinary reading reports, a 45 GB overlay root, and **one NVIDIA GeForce RTX 3060 with 12288 MiB on driver 550.120**. It had **no display server as rented**; one was installed for the panel test and it draws in software (Xvfb, OpenGL renderer `llvmpipe`). Blender 5.1.2 at `/opt/blender/5.1/blender`, **build hash `ec6e62d40fa9` — the same build as the Mac's**, checked before any test was run.

Every run used a throwaway settings folder made beforehand and `--factory-startup`, so none of the Creator's own settings was read or touched. Every Cycles render was on the processor except where a card is named.


### Every denoising setting a scene or a view layer carries

**Done.** With no file open, listed every setting on the scene, the scene's render settings, the scene's EEVEE settings and ray-tracing options, the view layer and the view layer's EEVEE settings whose name contains "denois", with its kind, its starting value and its choices.

**Came back**, identically on both machines. Thirteen on `scene.cycles`, three on `scene.render`, four on `scene.eevee.ray_tracing_options`, `use_taa_reprojection` on `scene.eevee`, and **two on `view_layer.cycles`**:

```
E8 view_layer.cycles denoising_store_passes type=BOOLEAN default=False current=False
E8 view_layer.cycles use_denoising type=BOOLEAN default=True current=True
```

**Every starting value this document records is right**, including the pair it makes a point of — `denoising_use_gpu` off for the render and `preview_denoising_use_gpu` on for the viewport.

**EEVEE's view layer carries nothing about denoising in any version** — the run answered `E8 view_layer.eevee NONE`. Its full list is `['ambient_occlusion_distance', 'use_pass_bloom', 'use_pass_transparent', 'use_pass_volume_direct']`, which is a difference between versions worth recording because the four documents in this round disagree about it: **4.5 has three entries and 5.0, 5.1 and 5.2 have four**, the extra one being `ambient_occlusion_distance`. None of them is a denoising setting.

**One difference between versions, tested identically in all four.** Blender 5.2 carries **two more** view-layer denoising settings that 4.5, 5.0 and 5.1 have not got — `denoising_pass_follow_reflections` and `denoising_pass_use_albedo_roughness_weighting`, both starting on. This version has only the two above.

### Which denoisers this machine offers, and what decides it

**Done.** On a factory startup, asked the final-render and viewport denoiser settings for a value that cannot exist, so that Blender's refusal names every real choice; then read what devices Cycles' preferences report; then chose a device type, switched its devices on, and asked again.

**Came back on the Creator's Mac:**

```
E9 denoiser choices <- bpy_struct: item.attr = val: enum "NO_SUCH_CHOICE_XYZ" not found in ('OPENIMAGEDENOISE')
E9 preview_denoiser choices <- bpy_struct: item.attr = val: enum "NO_SUCH_CHOICE_XYZ" not found in ('AUTO', 'OPENIMAGEDENOISE')
E9 compute_device_type='METAL'
E9 devices_for_type OPTIX   -> []
E9 devices_for_type METAL   -> [('Apple M1 Pro (GPU - 14 cores)', True), ('Apple M1 Pro', False)]
E9 set compute_device_type=OPTIX REFUSED TypeError: bpy_struct: item.attr = val: enum "OPTIX" not found in ('NONE', 'METAL')
```

**Came back on the rented Linux machine with an RTX 3060:**

```
E9 denoiser choices <- bpy_struct: item.attr = val: enum "NO_SUCH_CHOICE_XYZ" not found in ('OPTIX', 'OPENIMAGEDENOISE')
E9 preview_denoiser choices <- bpy_struct: item.attr = val: enum "NO_SUCH_CHOICE_XYZ" not found in ('AUTO', 'OPTIX', 'OPENIMAGEDENOISE')
E9 compute_device_type='NONE'
E9 devices_for_type OPTIX   -> [('NVIDIA GeForce RTX 3060', True), ('AMD Ryzen 5 5600X 6-Core Processor', False)]
E9 after compute_device_type=OPTIX (2 devices switched on) denoiser choices <- bpy_struct: item.attr = val: enum "NO_SUCH_CHOICE_XYZ" not found in ('OPTIX', 'OPENIMAGEDENOISE')
```

**What that settles.**

- **The list of denoisers is a fact about the machine, not about the version.** It is the same in all four versions on a given machine, and the two machines differ.
- **OptiX is offered on the machine with an NVIDIA card before any device type has been chosen in the preferences** — the device type reads `NONE` on that first line and OptiX is already there — and switching devices on changes nothing. So the manual's "only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences" does not describe what governs the list: **having the card is enough.**
- **`AUTO` is never a final-render choice and is always the first viewport choice**, on both machines. The manual reads as though "Automatic" were available to the final render; Cycles' own file is right and the manual is not.

### Whether the two denoising switches must both be on

**Done.** Two ways. First, rendered the test scene at 8 samples with adaptive sampling off in every combination of the scene's switch and the view layer's, and compared the pictures channel by channel, including one picture against itself as a control. Second, read the passes the view layer produces through a Render Layers node, in six combinations of the two switches and Store Denoising Passes.

**The picture came back**, in 5.1.2 on the Creator's Mac:

```
PIXELCASE both on   vs both off  : PIXELS max_difference 0.7058823704719543 channels_differing 6307 of 12288
PIXELCASE scene on, layer off vs both off : PIXELS max_difference 0.0 channels_differing 0 of 12288
PIXELCASE scene off, layer on vs both off : PIXELS max_difference 0.0 channels_differing 0 of 12288
PIXELCASE both on   vs scene on, layer off : PIXELS max_difference 0.7058823704719543 channels_differing 6307 of 12288
PIXELCASE both off  vs both off (control)  : PIXELS max_difference 0.0 channels_differing 0 of 12288
```

The rented Linux machine gave the same verdict, with slightly different figures because the two processors trace slightly different noise; **the three zeros and the control zero were identical on both**.

**The passes came back**, identically on both machines:

```
E7P scene=off layer=off store=off denoising-related=[]
E7P scene=on  layer=on  store=off denoising-related=['Noisy Image']
E7P scene=on  layer=off store=off denoising-related=[]
E7P scene=off layer=on  store=off denoising-related=[]
E7P scene=off layer=off store=on denoising-related=['Denoising Albedo', 'Denoising Normal', 'Denoising Depth']
E7P scene=on  layer=on  store=on denoising-related=['Noisy Image', 'Denoising Albedo', 'Denoising Normal', 'Denoising Depth']
```

**What that settles.** **Cycles denoises a view layer only when the scene's switch and the view layer's are both on** — observed, not inferred. Either switch alone leaves the picture identical, to the last channel, to both switches off. **The "Noisy Image" pass appears under exactly the same condition.** And **Store Denoising Passes is not gated by either switch**: its feature passes appear whenever it is on, whatever the two switches say.


*(Added 2026-08-30, and it is a repeat of the pass half above rather than a new question.
**Observed.** **What was done:** `e7passes.py`, unchanged — the same six combinations of the
scene's switch, the view layer's switch and Store Denoising Passes, read through a Render Layers
node with no file written. **To what:** Blender 5.1.2, build hash `ec6e62d40fa9`, run with
`-b --factory-startup --python e7passes.py` and a throwaway settings folder, **three times over**.
**On what computer:** the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no NVIDIA card. It was run because the hand-test list's summary of this test said the
block came back the same in all four versions, while the four versions' own findings documents did
not agree with each other about the order of the feature passes.*

***A negative result would have looked like*** *a different order from one run to the next, or an
order different from the one this document already records — either would have said this block
could not be relied on.*

***What came back.*** *All three runs gave the same output, character for character:*

```
E7P scene=off layer=off store=off denoising-related=[]  total_sockets=2
E7P scene=on  layer=on  store=off denoising-related=['Noisy Image']  total_sockets=3
E7P scene=on  layer=off store=off denoising-related=[]  total_sockets=2
E7P scene=off layer=on  store=off denoising-related=[]  total_sockets=2
E7P scene=off layer=off store=on denoising-related=['Denoising Albedo', 'Denoising Normal', 'Denoising Depth']  total_sockets=5
E7P scene=on  layer=on  store=on denoising-related=['Noisy Image', 'Denoising Albedo', 'Denoising Normal', 'Denoising Depth']  total_sockets=6
E7P blender 5.1.2
```

***What this settles.*** *The order this document records is what the machine prints, three times
out of three. **Across the four versions the order is not the same**, which is what the question
was: 4.5 and 5.0 give `['Denoising Normal', 'Denoising Albedo', 'Denoising Depth']`, 5.1 gives
`['Denoising Albedo', 'Denoising Normal', 'Denoising Depth']`, and 5.2 gives five names,
`['Denoising Albedo', 'Denoising Specular Albedo', 'Denoising Normal', 'Denoising Roughness',
'Denoising Depth']`. The hand-test list's summary claiming all four were identical has been
corrected there; the four findings documents were right. **Nothing about which passes appear, or
under what condition, changes.***

***What this does not cover.*** *macOS alone. The rented Linux machine is destroyed and nothing was
rented, so the "identically on both machines" line above stands exactly as its own run recorded it
and was not re-checked. Every run here was Blender driven from outside its own window with `-b`.
The raw output, word for word, is at `Audits/Blender Research/Settings/Hand tests — Open Threads
questions — raw output/Mac 2026-08-30/`.)*

### Whether the view layer's Denoising checkbox is drawn dimmed

**Done.** On the rented machine, with a display installed for the purpose, opened Blender with a window in each version, set the Properties editor to its View Layer tab and its search box to "Denoising" so the Filter panel opens, and captured the display from outside Blender — once with the scene's Denoise on and once with it off. **The display draws in software (Xvfb, `llvmpipe`)**, which draws a dimmed checkbox the same way a card would, but is not a real workstation's screen.

**Came back.** Counted over the whole 1400 by 2400 display, the two captures differ by **482 pixels**, in a single region **72 pixels by 13, at 1187 across and 1365 down** — the "Denoising" checkbox and its label, and nothing else on the screen. With the scene's Denoise **on**, that checkbox is drawn in full brightness, white label and bright blue ticked box; with it **off**, it is drawn exactly like the `Motion Blur` checkbox beside it, grey label and muted blue box. **The tick stays in the box in both.**

**What that settles.** The dimming this document infers from one line of Cycles' interface file is what a person sees. **The switch is not cleared and not hidden — it is dimmed and stays ticked.**

### Whether Prefilter and Quality do anything under a denoiser that is not OpenImageDenoise

**Done.** On the rented machine only, because the Creator's Mac offers one denoiser. Rendered the same frame with the path tracing on the processor and the denoising on the card, under OptiX with Prefilter None and Quality Fast, then under OptiX with Prefilter Accurate and Quality High, then the same pair under OpenImageDenoise — and a repeat of one OptiX case as a control.

**Came back**, in 5.1.2:

```
E8BCASE optix_acc_high vs optix_acc_high_repeat (control) : PIXELS max_difference 0.0 channels_differing 0 of 27648
E8BCASE optix_none_fast vs optix_acc_high                : PIXELS max_difference 0.0 channels_differing 0 of 27648
E8BCASE oidn_none_fast  vs oidn_acc_high                 : PIXELS max_difference 0.3137255012989044 channels_differing 10641 of 27648
```

**What that settles.** Under OptiX those two settings change **not one channel of 27 648**; under OpenImageDenoise the same change moves about ten and a half thousand. The control shows two identical runs come back byte for byte the same, so a difference would have been seen. **So the two settings are not merely hidden under another denoiser — they have no effect at all.**

### Whether OptiX could be used at all on that machine, with the path tracing on the card too

**Done.** On the rented Linux machine only, on 2026-08-30, while the entry above was being set up. That entry deliberately keeps the path tracing on the processor and puts only the denoising on the card. Before it was settled that way, the same frame was rendered with the **path tracing on the card as well**, under OptiX, in all four versions on that one machine — Vast.ai instance 49253900, machine 27187, in Poland: a container rather than a whole machine, Ubuntu 22.04.4 LTS on kernel 6.8.0-48-generic, an AMD Ryzen 5 5600X with 5.76 cores allotted, and **one NVIDIA GeForce RTX 3060 with 12288 MiB on driver 550.120**. Blender was driven from a terminal with no window, with `--factory-startup` and a throwaway settings folder, and all four build hashes were checked against the Creator's Mac before anything was run. **This was met beside another test rather than asked as a question**, and it was run once.

**Came back.** **Blender 5.1.2 could use OptiX on that machine**, as could 4.5.12 LTS and 5.0.1. **Blender 5.2.1 LTS, on the same machine, the same card and the same driver, could not**, and wrote no image. These are the machine's own words in 5.2.1, and they are the only output this test captured:

```
New backend is missing implementation for PTX intrinsic optix.ptx.copysign.f32
00:00.711  cycles           | ERROR Failed to load OptiX kernel from '/opt/blender/5.2/5.2/scripts/addons_core/cycles/lib/kernel_optix.ptx.zst' (OPTIX_ERROR_INTERNAL_COMPILER_ERROR)
00:00.711  cycles           | ERROR Refer to the Cycles GPU rendering documentation for possible solutions:
                            | https://docs.blender.org/manual/en/latest/render/cycles/gpu_rendering.html
00:00.711  reports          | ERROR Failed to load OptiX kernel from '…kernel_optix.ptx.zst' (OPTIX_ERROR_INTERNAL_COMPILER_ERROR)
```

**Nothing was captured from the three versions that worked.** The record says they could and shows no output for them, so that half rests on the sentence and not on words of its own.

**What a different result would have looked like.** The other three versions on that same machine, on the same card and the same driver, on the same day, are the control. Had the card or the driver been the whole story, all four would have failed alike; three of the four loaded their OptiX kernel and rendered, which is what makes the fourth's failure a difference between Blenders rather than a broken machine. Had 5.2.1 rendered as well, there would have been no result here at all.

**What this does not cover, and it is almost everything.** **One card, one driver, one machine, one day, one run of four Blenders.** It is **not a statement about Blender 5.2.1 anywhere else**, and nothing in it says the same thing would happen on another card, another driver, or another machine — **every other card and every other driver is untested, in every version**. **No other result in this document depends on it**: the entry above keeps the path tracing on the processor and only the denoising on the card, which is why that one ran in all four versions here. It is written down because a render farm choosing a driver would want to know this can happen.

**Label: observed** (2026-08-30).

### Whether the scene's compositor denoise settings reach a render, under both engines

**Done.** Built a compositor of Render Layers → Denoise → output, switched Cycles' own denoising off so the node was the only thing denoising, and rendered the same frame with `compositor_denoise_final_quality` at Fast and at High, under Cycles and under EEVEE, with a repeat as a control.

**Came back**, in 5.1.2 on the Creator's Mac:

```
E9C node_quality='NO-QUALITY-ON-NODE'
E9CASE cy_high vs cy_high_repeat (control) : PIXELS max_difference 0.0 channels_differing 0 of 27648
E9CASE cy_fast vs cy_high                  : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
E9CASE ee_fast vs ee_high                  : PIXELS max_difference 0.027450978755950928 channels_differing 3528 of 27648
```

The rented machine gave the same verdict.

**What that settles.** That setting changes a final render **under both engines** — engine-independent by test rather than by inference. **And one difference between versions, tested in all four.** The Denoise node here carries **no `quality` of its own at all**, where 4.5's does and starts at `FOLLOW_SCENE`. So from 5.0 on the node has no choice but to follow the scene, and the manual's condition "if the nodes' quality option is set to Follow Scene" describes something that exists only in 4.5.


*(Added 2026-08-30, and it is a repeat of the run above rather than a new question. **Observed.**
**What was done:** the same test, unchanged — the scene built by this version with `make_scene.py`,
a compositor of Render Layers → Denoise → output, Cycles' own denoising off, 96 by 72, 8 samples,
adaptive sampling off, then the five cases and the three comparisons of `compare_pixels.py`.
**To what:** Blender 5.1.2, build hash `ec6e62d40fa9`, started with `-b --factory-startup` and a throwaway
settings folder, so nothing of the machine's own preferences was loaded. **Run twelve times over in
one sitting**, each round rebuilding the scene from the script. **On what computer:** the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no NVIDIA card. It was run
because two earlier runs of this test in 5.0 disagreed by one channel and nobody knew whether the
figure was stable at all.*

***A negative result would have looked like*** *the same count in every round, which would have said
the figure can be quoted as exact. The control — two renders at the same setting compared against
each other — is what says a difference would have been seen if there had been one.*

***What came back.*** *The twelve `cy_fast vs cy_high` lines, in order:*

```
round 1  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 2  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 3  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9135 of 27648
round 4  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 5  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9135 of 27648
round 6  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 7  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 8  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 9  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 10 E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 11 E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 12 E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9135 of 27648
```

*In every one of the twelve rounds the other two lines were identical:*

```
E9CASE cy_high vs cy_high_repeat : PIXELS max_difference 0.0 channels_differing 0 of 27648
E9CASE ee_fast vs ee_high : PIXELS max_difference 0.08627453446388245 channels_differing 3528 of 27648
```

*and the maximum difference on the Cycles line, `0.08627453446388245`, never moved.*

*Two different figures came back — 9135 and 9136. **The count is not reproducible in this version on this machine**, though it moved less than in 4.5 or 5.0. The figure recorded above, `9136`, came back in nine rounds of the twelve; three gave 9135. **Both stand and neither is preferred.** Everything else in the block above reproduced exactly, twelve times over.*

***What this does and does not touch.*** *The verdict this passage rests on is untouched: the
setting still changes a final render under both engines, and a change of some nine thousand
channels against a control of nought is not in doubt. **What should not be quoted as exact is the
last digit of the count**, in 4.5, 5.0 and 5.1. The same twelve rounds were run in each of the four
versions on the same machine on the same day.*

***What this does not cover.*** *macOS alone. The rented Linux machine that the first run of this
test also used is destroyed, and nothing was rented, so **nothing here says whether the count wobbles
on Linux** and the "the rented machine gave the same verdict" line above stands exactly as its own
run recorded it. Every run here was Blender driven from outside its own window with `-b`. Nothing
here is timed, so the other work running on this Mac that day does not bear on it. The raw output,
word for word, is at `Audits/Blender Research/Settings/Hand tests — Open Threads questions — raw
output/Mac 2026-08-30/`.)*

### What each denoising setting is called on screen

**Done.** Read the display name the running program holds for every denoising setting.

**Came back**, identically on both machines: `use_denoising` is **Use Denoising**, `denoiser` is **Denoiser**, `denoising_input_passes` is **Denoising Input Passes**, `denoising_prefilter` is **Denoising Prefilter**, `denoising_quality` is **Denoising Quality**, `denoising_use_gpu` is **Denoise on GPU**; the viewport set carries the same names with **Viewport** in front, except `preview_denoising_start_sample`, which is **Start Denoising**, and `preview_denoising_use_gpu`, which is **Denoise Preview on GPU**. On the view layer, `use_denoising` is **Use Denoising** and `denoising_store_passes` is **Store Denoising Passes**. On the scene's render settings the three are **Compositor Denoise Node Device**, **Compositor Final Denoise Quality** and **Compositor Preview Denoise Quality**. EEVEE's four ray-tracing settings are **Denoise**, **Spatial Reuse**, **Temporal Accumulation** and **Bilateral Filter**.

**One place where Blender and the manual do not agree.** Blender calls `use_taa_reprojection` **"Viewport Denoising"**; the manual calls it **"Temporal Reprojection"**. This document passes the manual's name on; both names are for the same setting.

### What a render does with a file that names a denoiser the machine has not got

**Done.** Made a file on the rented Linux machine with `scene.cycles.denoiser` set to `OPTIX`, carried it to the Creator's Mac, which offers no OptiX, and rendered it there — once as saved and once with denoising switched off, as a control.

**Came back**, in 5.1.2 on the Mac:

```
00:00.462  bpy.rna          | WARNING current value '2' matches no enum in 'CyclesRenderSettings', '', 'denoiser'
READDEN2 blender=5.1.2 mode=as-saved denoiser_now='' use_denoising=True
00:00.628  render           | Saved: '5.1-as-saved0001.png'
E12CASE 5.1 as-saved vs the same file with denoising switched off : PIXELS max_difference 0.7372549176216125 channels_differing 13680 of 27648
```

and the stored value cannot be assigned back to itself:

```
READDEN re-assigning the value it already holds: REFUSED TypeError: bpy_struct: item.attr = val: enum "" not found in ('OPENIMAGEDENOISE')
```

**What that settles.**

- **The render goes ahead.** Blender does not refuse and does not stop; it saves the picture.
- **The picture is still denoised** — it differs from the same file with denoising switched off by nearly fourteen thousand channels of 27 648. Blender falls back to a denoiser the machine has rather than doing nothing.
- **The setting reads back as an empty string.** A program reading the denoiser out of such a file gets nothing, not `OPTIX`.
- **A warning is printed, and only when the setting is read from a script.** Nothing about the mismatch appears in the render's own output, so a run that never reads the setting says nothing at all about it.

**Outside the interface — brought up to date by the hand tests of 2026-08-30.** Every result above except the dimmed checkbox was got from a Blender started with no window. The denoiser list, the two switches' effect on the picture and on the passes, the compositor denoise settings and the fallback for a denoiser the machine has not got are all **observed** with no window open. **The edge this document flags in `enum_optix_denoiser` was tested and did not behave as the reading suggested**: on a windowless run on a Mac with no NVIDIA card the OptiX entry was **not** offered, and on a windowless run on a machine with such a card it **was**, before any device was configured. So what governs the list is the card, not the absence of a context.

**Operating system — brought up to date by the hand tests of 2026-08-30.** The settings themselves — their names, kinds, starting values and choices — came back **identical on macOS and on Linux**, so nothing about which settings exist depends on the operating system. What differs is which denoisers are on offer: OpenImageDenoise alone on the Mac, OptiX and OpenImageDenoise on the Linux machine with an NVIDIA card. **That difference follows the card rather than the operating system as far as these two machines can show**, and no machine with an AMD or an Intel card was available. Windows is untested; the tests are written out ready to run in `Audits/Blender Research/Settings/Hand tests — Open Threads questions.md`.


---

## Things a person with Blender 5.1 open could settle by hand

*(Pruned on 2026-08-30. Everything this list held was run — see the hand-test results above — except the items below, which are genuinely still open. The tests, with their exact commands and their raw output, are at `Audits/Blender Research/Settings/Hand tests — Open Threads questions.md`.)*

1. **What denoisers a machine with an AMD or an Intel graphics card offers.** The two machines used were a Mac with Apple's own graphics and a Linux machine with an NVIDIA card. Nothing here says what a third kind of card offers, and the manual's claim that AMD denoising starts at a different card generation on Linux and on Windows is untested.
2. **What happens when a sample count is driven, or animated, out of its range.** Every test above set the value directly. A driver and a keyframe are different routes and neither was tried.
3. **What Blender does on loading a file whose stored sample count is above the ceiling.** The correction was observed at the moment of assignment; the file-loading path is a different one, and no file old enough to trigger the squared-samples conversion exists here.
4. **What a number field does when a value outside the range is dragged rather than typed.** The dragging bounds were read off the running program, but no test dragged a control.
5. **Everything on Windows.** Eleven entries are written out ready to run in the hand-test list named above, for the separate Windows batch. They are not unrunnable — this project has a route to a Windows computer, and the account allows one machine at a time.

---

2026-08-30 — Claude Opus 5 (Claude Code) — Settings (Open Threads: questions 18 and 19), Blender 5.1
2026-08-30 — Claude Opus 5 (Claude Code) — hand-test results for the Open Threads questions (18 and 19) written in, the two standing lines on each answer brought up to date, and the researcher's own list pruned to what is still open
2026-08-30 — Claude Opus 5 (Claude Code) — hand-test entries 13 and 14 written in: the compositor denoise-quality comparison run twelve times over in one sitting, and the order of the denoising feature passes read three times over, both on the Creator's Mac and both recorded as observed

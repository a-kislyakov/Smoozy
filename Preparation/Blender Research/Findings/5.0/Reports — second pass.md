# Reports — Blender 5.0 — second pass

**Questions answered here: 2 and 4 only.**

## Sources consulted, and on what date

I consulted, all on the snapshot date **2026-07-30**, and only these:

- The Blender 5.0 manual, as plain-text files in `Blender Sources/manual_5.0`, including its pictures in `Blender Sources/manual_5.0/_images`.
- The Blender 5.0 Python API documentation, as plain-text files in `Blender Sources/api_5.0`.
- The Blender 5.0 release notes, as plain files in `Blender Sources/release_notes_5.0`.
- Cycles' own files, in `Blender Sources/cycles_5.0`.

Date read for every finding below: **2026-07-30**. I did not go to the web.

## Declaration

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

Four additions, so that line is complete rather than merely true:

- I read only the `5.0` folders. I did not open the `4.5`, `5.1` or `5.2` sources sitting beside them.
- A Blender tool-server was offered to me partway through this session, able to talk to a running Blender and to search a manual of its own. **I did not use it.** Not one of its tools was called, and nothing here comes from it.
- I started two helper agents to widen the search, copying the whole restriction warning into each word for word and giving each the same two readable things and nothing more. One returned (the sweep on question 2); the other never reported before this pass ended. **I can vouch for the one that returned**, because I did not take its word for anything: every quote below that it surfaced, I then opened the file and read myself before using it. Nothing in this document rests on a helper's report alone. The helper that never returned contributed nothing.
- I did not open the existing findings document for this version.

---

## 2. Where does Blender say that a render failed, and what does it say there?

**LABEL: documented in parts, and not in the sources as a whole.** The places that carry error messages are documented. A handful of exact failure texts are documented, and several more are observed in Cycles' own files. But there is **no page anywhere in these sources that answers this question as asked** — nothing states where a failed render reports itself, or what it says. What follows is assembled from pages that describe Blender's general reporting machinery, plus the one page that mentions a render failing.

### 2a. The one place the sources say a render fails with a message

This is the only sentence in all four sources that puts "render", "fail" and "error message" together.

> "Depending on the path, template errors may prevent certain actions. For
> example, if there are errors in the render output path, then rendering an
> animation will fail with an error message indicating the path errors."

- Source: the manual — https://docs.blender.org/manual/en/5.0/files/file_paths.html
- Date read: 2026-07-30

The same page describes how the bad path shows itself before you render:

> "When there are template errors in a path, the path field will be highlighted red
> in the UI"

> "Hovering over the path field will pop up a tooltip that contains a list of the
> template errors encountered in that path"

Note what this does and does not give. It tells us a render **can** fail with an error message, and that the message names the path errors. It does not say where that message appears.

### 2b. The Info editor — the log of errors inside the interface

> "The Info editor logs the executed operators as well as errors, warnings,
> and informational messages."

- Source: the manual — https://docs.blender.org/manual/en/5.0/editors/info_editor.html
- Date read: 2026-07-30

I also opened that page's picture, `_images/editors_info-editor_ui.png`. **Documented**, from the picture: three entries, each a `bpy.ops....` line with a green tick at the left, under a header reading `View` and `Info`. Only successful operator entries are visible. **No error or warning entry appears in it**, so the picture does not show what a failure looks like there.

Entries can be handled:

> "Removes the selected entries from the log."

> "Copies the selected entries to the clipboard."

The matching operators exist in the API, and are the only place the sources call these entries "reports":

> "bpy.ops.info.report_copy()
>
> Copy selected reports to clipboard"

> "bpy.ops.info.report_delete()
>
> Delete selected reports"

- Source: the Python API documentation — https://docs.blender.org/api/5.0/bpy.ops.info.html
- Date read: 2026-07-30

### 2c. The Status Bar — a message that does not stay

> "Report Message
>
> Informational messages or warnings, such as after saving a file.
> They disappear after a short time.
> Click them to show the full message in the Info Editor."

- Source: the manual — https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html
- Date read: 2026-07-30

Read that precisely: **"Informational messages or warnings"**. It does not say errors, and I am not going to read errors into it. What it does establish is that a report here is brief, vanishes on its own, and that its full text lives in the Info editor.

### 2d. The header — where an operator's own report shows

> "# Rather than printing, use the report function,
> # this way the message appears in the header."

- Source: the Python API documentation — https://docs.blender.org/api/5.0/bpy.types.Operator.html
- Date read: 2026-07-30

This is a comment inside a worked example on that page, so it is the documentation speaking about its own example. It is the only statement in the sources that names *where* a report surfaces.

The same page records that reports persist on the operator:

> "has_reports
>
> Operator has a set of reports (warnings and errors) from last execution"

### 2e. The Python caller — an exception carrying the messages

This is the only passage that states plainly what a caller receives when something fails.

> "If operator was cancelled but there wasn't any reports from it with {'ERROR'} type,
> it will just return {'CANCELLED'} without raising any exceptions.
> However, if there are error reports, a RuntimeError will be raised
> after the operator finishes execution, including all error report messages,
> regardless of the return status (even if it was {'FINISHED'})."

- Source: the Python API documentation — https://docs.blender.org/api/5.0/bpy.ops.html
- Date read: 2026-07-30

The sting is in the tail: an error report raises the exception **even when the operator reported that it finished**.

The same page gives what an operator can return:

> "Operators don't have return values as you might expect,
> instead they return a set() which is made up of:
> {'RUNNING_MODAL', 'CANCELLED', 'FINISHED', 'PASS_THROUGH'}."

Two examples of the exact shape of such an error:

> "RuntimeError: Operator bpy.ops.view3d.render_border.poll() failed, context is incorrect"

- Source: the Python API documentation — https://docs.blender.org/api/5.0/info_quickstart.html

> "RuntimeError: Operator bpy.ops.action.clean.poll() failed, context is incorrect"

- Source: the Python API documentation — https://docs.blender.org/api/5.0/info_gotchas_operators.html
- Date read for both: 2026-07-30

### 2f. The render engine's own two routes

> "report(type, message)
>
> Report info, warning or error messages"

> "error_set(message)
>
> Set error message displaying after the render is finished"

- Source: the Python API documentation — https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html
- Date read: 2026-07-30

`error_set` is the closest thing in the API to a statement about a failed render, and it repays careful reading. It says the message is displayed **after the render is finished**. It does **not** say where. I searched the whole API and the whole manual for any page that says where; there is none. That gap is real and I have not filled it.

The kinds a report can be:

> "DEBUG:
>
> Debug.
>
> INFO:
>
> Info.
>
> OPERATOR:
>
> Operator.
>
> PROPERTY:
>
> Property.
>
> WARNING:
>
> Warning.
>
> ERROR:
>
> Error.
>
> ERROR_INVALID_INPUT:
>
> Invalid Input.
>
> ERROR_INVALID_CONTEXT:
>
> Invalid Context.
>
> ERROR_OUT_OF_MEMORY:
>
> Out of Memory."

- Source: the Python API documentation — https://docs.blender.org/api/5.0/bpy_types_enum_items/wm_report_items.html
- Date read: 2026-07-30

So there are three error kinds beyond plain `ERROR`, and one of them is out of memory.

### 2g. The console

> "The Blender Console Window can display many different types of status and error messages. Some messages simply
> inform the user what Blender is doing, but have no real impact on Blender's ability to function. Other messages can
> indicate serious errors that will most likely prevent Blender carrying out a particular task and may even make Blender
> non-responsive or shut down completely. The Blender Console Window messages can also originate internally from
> within the Blender code or from external sources such as Python scripts."

- Source: the manual — https://docs.blender.org/manual/en/5.0/troubleshooting/startup.html
- Date read: 2026-07-30

And the console can carry an error the interface does not:

> "The error traceback is printed in full to the terminal which won't always generate an report message in
> Blender's user interface (depending on how the script is executed)."

- Source: the Python API documentation — https://docs.blender.org/api/5.0/info_tips_and_tricks.html
- Date read: 2026-07-30

The 5.0 release notes state that reports now travel the same route as everything else Blender logs:

> "Most of Blender now uses the same logging system,
> including operators reports and background render progress.
> The output format has changed, and log levels are easier to control."

- Source: the release notes — https://developer.blender.org/docs/release_notes/5.0/core/
- Date read: 2026-07-30

The release notes also record, among the fixes, that reports going missing when Blender is driven from the command line was a real fault that was addressed:

> " * Reports from versioning code invisible when file loaded from CLI"

- Source: the release notes — https://developer.blender.org/docs/release_notes/5.0/bugfixes/
- Date read: 2026-07-30

And a change to how alerts are shown:

> "- Generalized Alert and Popup Error indication."

- Source: the release notes — https://developer.blender.org/docs/release_notes/5.0/user_interface/
- Date read: 2026-07-30

### 2h. Exact failure texts the sources give

**From the manual (documented).** Cycles' GPU page is the manual's only section headed "Error Messages" for rendering:

> "Error: Out of memory
>
> This usually means there is not enough memory to store the scene for use by the GPU."

> "The NVIDIA OpenGL driver lost connection with the display driver"

> "Unsupported GNU version"

> "CUDA Error: Kernel compilation failed
>
> This error may happen if you have a new NVIDIA graphics card that is not yet supported by
> the Blender version and CUDA toolkit you have installed.
> In this case Blender may try to dynamically build a kernel for your graphics card and fail."

- Source: the manual — https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html

For EEVEE:

> "The error message Shadow buffer full means that the system cannot allocate enough shadow memory."

- Source: the manual — https://docs.blender.org/manual/en/5.0/render/eevee/light_settings.html

For OSL, the manual names the destination outright:

> "Compilation errors will be shown in the system console."

- Source: the manual — https://docs.blender.org/manual/en/5.0/render/cycles/osl/index.html
- Date read for all three: 2026-07-30

**From Cycles' own files (observed).** These are the exact message strings Cycles ships. Quoting them is reading a file, not predicting a render.

In `Blender Sources/cycles_5.0/osl.py`, at line 312, at the end of `update_script_node`:

> `        report({'ERROR'}, "OSL script compilation failed, see console for errors")`

At line 364, at the end of `update_custom_camera_shader`:

> `        report({'ERROR'}, "Custom Camera shader compilation failed, see console for errors")`

At lines 310 and 362:

> `            report({'ERROR'}, rpt_("OSL query failed to open %s") % oso_path)`

At line 214:

> `                report({'ERROR'}, rpt_("Failed to write .oso file next to external .osl file at {:s}").format(dst_path))`

At line 221:

> `        report({'ERROR'}, "External shader script must have .osl or .oso extension, or be a module name")`

At lines 292 and 345:

> `        report({'WARNING'}, "No text or file specified in node, nothing to compile")`

In `Blender Sources/cycles_5.0/__init__.py`, at lines 111 and 118:

> `            self.report({'ERROR'}, "OSL support disabled in this build")`

In `Blender Sources/cycles_5.0/operators.py`, at lines 86–88, inside the denoise-animation operator:

> `                    err_msg = tip_("Frame '%s' not found, animation must be complete") % filepath`
> `                    self.report({'ERROR'}, err_msg)`
> `                    return {'CANCELLED'}`

At lines 105–107 of the same file, and again at 144–146:

> `        except Exception as e:`
> `            self.report({'ERROR'}, str(e))`
> `            return {'FINISHED'}`

That last pattern is worth flagging: on an exception these two Cycles operators report an error and return **`{'FINISHED'}`**, not `{'CANCELLED'}`. Read together with the `bpy.ops` rule quoted in 2e, the exception still reaches a Python caller, because the rule fires "regardless of the return status (even if it was `{'FINISHED'}`)". That last sentence is **inferred**: the step is joining the observed return value in Cycles' file to the documented rule on the `bpy.ops` page. Neither source states this combination itself.

- Cycles' own files, cited by file and line as above. Date read: 2026-07-30

### 2i. Where Blender writes when it stops altogether

> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the
> crash. Usually, this file is written in the Temporary Directory directory."

> "On a crash, a file is written based on the name of the currently loaded blend-file, so test.blend will create a
> file called test.crash.txt."

> "After a crash, a file named blender.crash.txt is written to the /tmp directory."

- Source: the manual — https://docs.blender.org/manual/en/5.0/troubleshooting/crash.html
- Date read: 2026-07-30

The first two lines above are Windows; the third is Linux. This is a crash, not a reported render failure, and I include it only because it is a place Blender writes when work stops.

### Not covered

A great deal, and the shape of the gap matters more than its size.

- **The sources never say where a failed render reports itself.** `error_set` says a message is displayed after the render finishes and stops there. No page connects a render failure to the Info editor, the status bar, or the console.
- **No documented list of what can make a render fail** with the message each produces. The manual documents four Cycles GPU errors and one EEVEE error, and that is the whole of it.
- **No exit code.** I searched for exit codes, return codes and exit status across all four folders. The only exit-code mechanism anywhere is `--python-exit-code`, documented as "Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line), zero disables." It is explicitly scoped to Python exceptions, not to renders. Nothing states what a failed render leaves behind as a process exit status.
- **No "no camera" error.** I searched for it specifically; it does not exist in these sources.
- **Nothing on a render failing to write its output** — disk full, no permission, read-only location. Zero hits on any of those.
- **`bpy.ops.render.render()` is undocumented.** Its page says only "Undocumented, consider contributing." Nothing there describes what a failed render returns or reports.
- The manual's GPU troubleshooting pages each carry a "Render Errors" heading that contains no error text of its own, only "See EEVEE and Cycles documentation respectively."
- The Cycles GPU error messages are given without saying where they appear.
- Whether error text goes to standard output or standard error is not stated anywhere. See question 4.

**Observed, 2026-08-06 — two of the routes above, exercised on a second machine.**

Run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender 5.0.1, `-b`, its own settings folder, display turned off first. These add to the tests already recorded in `Preparation/Blender Research/Findings/5.0/Reports.md`; both machines were run and are reported separately there.

**The Python caller, 2e above, is no longer only a page.** A script was loaded with `--python` that called `bpy.ops.render.render(write_still=True)` inside a `try`. On a good file it printed `RENDER RETURNED: {'FINISHED'}`. On a file whose camera had been deleted it printed:

```
RENDER RAISED: RuntimeError Error: Cannot render, no camera
```

So the exception rule quoted in 2e does apply to the render operator, and the message it carries is the same text the log prints. Run again with Blender's window open, the raised message read `Error: No camera found in scene "Scene"` instead — the window's wording.

**The console, 2g above, and what a log file does to it.** With `--log-file <path>` given and no `--log` category named, **the whole log moved into the file and left standard output almost empty**. On the failing render the whole of standard output was:

```
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)

Blender quit
```

and the file held:

```
00:00.254  blend            | Read blend: "/root/smoozy/Reports/work/no_camera.blend"
00:00.261  render           | Rendering single frame
00:00.261  render           | Rendering frame 1
00:00.261  reports          | ERROR Cannot render, no camera
```

The exit status was still 1. So `--log-file` does not copy the log to a file — it moves it, failure message included, and a caller watching standard output sees nothing but the banner. Blender 4.5 behaves the other way: it creates the file, leaves it empty, and still prints everything.

**Outside the interface.** This is where the answer thins out badly, and it matters. Of the places above, the Info editor, the status bar and the header are parts of Blender's window; the API documents that `bpy.app.background` is "Boolean, True when blender is running without a user interface (started with -b)". It follows that when Blender is started with `-b` none of those three exist — that step is **inferred**, joining the definition of background mode to the fact that these are interface elements; no page states it. What the sources do state for outside-the-interface use is: the release notes say operator reports now go through the same logging system (2g); the terminal receives the full traceback where the interface may get no report at all (2g); the `bpy.ops` exception rule (2e) applies wherever Python calls the operator; and the release notes record a fix for reports being invisible when a file is loaded from the command line. There is no documented statement of what a failed background render prints, or of what exit status it leaves.

**Operating system.** Nothing here is said to differ by operating system, with these exceptions, all documented. The console itself differs — on Windows it is a separate window that is toggled off after start-up and is brought back through `Window ‣ Toggle System Console`; on macOS you must run the executable inside the application bundle "to get all output printed to the terminal"; on Linux it is the terminal Blender was started from. Crash files differ by system as quoted in 2i, with macOS additionally using "the macOS Crash Reporter". Two of the four documented Cycles GPU errors are platform-bound by their own text: the NVIDIA display-driver one says "Windows has a limit on the time the GPU can do render computations", and the GNU version one says "On Linux, depending on your GCC version you might get this error". Separately, the 5.0 release notes state "Support for Intel Macs has been removed", so the set of systems in question is itself narrower in this version.

**Both machines have now been run.** The failure messages and the exception the render operator raises came back identically on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2) and on the rented Linux machine. **Windows is untested and no Windows machine has run it.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** On Linux the crash file is written to `/tmp`, and it is named after the blend-file when one is loaded rather than always `blender.crash.txt` — recorded in full in `Preparation/Blender Research/Findings/5.0/Reports.md`.

---

## 4. What does Blender write out as it works, what is that stream called, and what does it contain?

**LABEL: documented, with one part observed and one part not in the sources.** What Blender writes and what it contains are documented, and the release notes give a literal sample. What the thing is *called* is documented but has several names and none of them is "stream". Whether it goes to standard output or standard error is **not in the sources**, except for one Cycles switch where it is **observed**.

### 4a. What it is called

The manual's own definition, on the page that opens the command-line section:

> "The Console Window, also called a Terminal, is an operating system text window that displays
> messages about Blender's operations, status, and internal errors."

> "When Blender is manually started from a terminal,
> Blender output is shown in the Console Window it is started from."

- Source: the manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/index.html
- Date read: 2026-07-30

Elsewhere the manual calls the same thing **"The Blender Console Window"** (troubleshooting, quoted in 2g) and **"the System Console"** — the latter both as a menu command, "To display the console again, go to Window ‣ Toggle System Console", and in the phrase "When you view the System Console".

The 5.0 release notes name the machinery behind it rather than the window: **"the same logging system"**, whose product they call **"logging output"** and, in the Python notes, **"this output"**.

**The sources never call it a stream.** I searched all four folders for the word; every occurrence is about video or audio streams. There is no term in these sources for standard output or standard error as such.

### 4b. What it contains — the literal sample

The release notes give the only sample of Blender's running output anywhere in these sources, and it includes a render. Quoting exactly:

> ```
> 00:00.027  ghost.system     | Create Cocoa system
> 00:00.080  translation      | Locale en_NL used for translation
> 00:00.088  blend            | Read prefs: "/Users/blender/Library/Application Support/Blender/5.0/config/userpref.blend"
> 00:00.096  blend            | Read startup: "/Users/blender/Library/Application Support/Blender/5.0/config/startup.blend"
> 00:00.359  gpu.platform     | Using GPU "Apple M3 Max Metal API 1.2"
> 00:00.359  gpu.platform     | Using Backend "Metal"
> 00:01.041  cycles           | Added device "Apple M3 Max (GPU - 40 cores)" with id "METAL_Apple M3 Max (GPU - 40 cores)".
> ...
> 00:00.244  render           | Rendering single frame (frame 1)
> 00:00.245  render           | Start rendering: Scene, ViewLayer
> 00:00.245  render           | Engine: Cycles
> 00:26.799  render           | Mem: 2983M | Sample 1/16 
> 00:26.799  render           | Mem: 2983M | Sample 16/16
> 00:28.631  render           | Saved: '/tmp/0001.png'
> 00:28.631  render           | Time: 00:28.38 (Saving: 00:00.15)
> ```

- Source: the release notes — https://developer.blender.org/docs/release_notes/5.0/core/
- Date read: 2026-07-30

The page introduces it with the words "Example output:", and immediately above:

> "Most of Blender now uses the same logging system,
> including operators reports and background render progress.
> The output format has changed, and log levels are easier to control."

What is visible in that sample, taken no further than what the characters show: each line carries a time, then a category name, then a `|`, then a message. The categories appearing are `ghost.system`, `translation`, `blend`, `gpu.platform`, `cycles` and `render`. The render lines announce the frame being started, the scene and view layer, the engine, then repeated progress lines carrying memory used and a sample count in the form `Sample 1/16`, then the file saved with its full path, then a total time with the saving time broken out separately. That the leading column is elapsed time since start is **inferred** from the values rising monotonically down the block; the page does not label the columns.

The Python release notes flag the consequence of the change plainly:

> "- The format of logging output has changed,
>   including background render progress.
>   Add-ons or render farms parsing this output may need to be updated."

- Source: the release notes — https://developer.blender.org/docs/release_notes/5.0/python_api/
- Date read: 2026-07-30

### 4c. What else it contains

The manual's general description, quoted in full at 2g, is the broadest statement: status messages, error messages, messages that originate inside Blender and messages from Python scripts.

The command-line page adds the uses:

> "For Python development, to see the output of the print() function."

> "If Blender exits unexpectedly, the messages may indicate the cause or error."

> "When troubleshooting, to see the output of --debug messages."

- Source: the manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/index.html

Video encoding output goes there too:

> "When you view the System Console,
> you can see some of the output of the encoding process.
> You will see even more output if you execute Blender as blender -d."

- Source: the manual — https://docs.blender.org/manual/en/5.0/render/output/properties/output.html

Context-member logging as well:

> "Log statements are printed to your system's console."

- Source: the Python API documentation — https://docs.blender.org/api/5.0/bpy.types.Context.html
- Date read for all three: 2026-07-30

The API also records that a handler fires when statistics are printed, and describes what those statistics carry:

> "bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."

- Source: the Python API documentation — https://docs.blender.org/api/5.0/bpy.app.handlers.html
- Date read: 2026-07-30

That is the only place in the sources stating that the content differs between background mode and the interface: in background mode the statistics additionally carry the frame and the used and peak memory.

### 4d. What controls it

All from the manual's command-line arguments page, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30.

> "--log <match>
>
> Enable logging categories, taking a single comma separated argument.
>
> --log "*": everything
> --log "event": every category starting with event.
> --log "render,cycles": both render and cycles messages.
> --log "*mesh*": every category containing mesh sub-string.
> --log "*,^operator": everything except operators, with ^prefix to exclude."

> "--log-level <level>
>
> Set the logging verbosity level.
>
> fatal: Fatal errors only
> error: Errors only
> warning: Warnings
> info: Information about devices, files, configuration, operations
> debug: Verbose messages for developers
> trace: Very verbose code execution tracing"

> "--log-show-memory
>
> Show memory usage for each log message."

> "--log-show-source
>
> Show source file and function name in output."

> "--log-show-backtrace
>
> Show a back trace for each log message (debug builds only)."

> "--log-file <filepath>
>
> Set a file to output the log to."

> "--log-list-categories
>
> List all available logging categories for --log, and exit."

And the switch that turns most of it off:

> "-q, --quiet
>
> Suppress status printing (warnings & errors are still printed)."

That last one is worth holding onto: it distinguishes "status printing", which `--quiet` suppresses, from warnings and errors, which survive it.

The release notes record which older switches were replaced:

> "| Old | New |
> |-|-|
> |`--debug-cycles`|`--log cycles`|
> |`--debug-ffmpeg`|`--log video`|
> |`--verbose`|`--log-level [info\|debug\|trace]`|"

- Source: the release notes — https://developer.blender.org/docs/release_notes/5.0/core/
- Date read: 2026-07-30

Both old spellings are nevertheless still listed on the manual's arguments page, `--debug-cycles` as "Enable debug messages from Cycles" and `--verbose <verbose>` as "Set the logging verbosity level for debug messages that support it". I report both without reconciling them; the sources disagree and I cannot settle it from reading.

### 4e. The one place a destination is named — observed

The manual describes the Cycles statistics switch this way:

> "--cycles-print-stats
>
> Log statistics about render memory and time usage."

- Source: the manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html

Cycles' own file describes the same switch differently, and names the destination. In `Blender Sources/cycles_5.0/engine.py`, lines 13–15, near the top of the file:

> `    parser.add_argument("--cycles-print-stats",`
> `                        help="Print rendering statistics to stderr",`
> `                        action='store_true')`

- Cycles' own file, `Blender Sources/cycles_5.0/engine.py`, lines 13–15. Date read: 2026-07-30. **Observed.**

This is the **only** mention of standard error in all four folders. Two cautions. First, what is observed is the text of the help string, not the behaviour: that these statistics actually arrive on standard error at run time is a step beyond the file and would be **inferred**. Second, this concerns only this one switch. It says nothing about where Blender's main log goes.

### Not covered

- **Whether Blender's ordinary output goes to standard output or standard error is nowhere stated.** The single `stderr` mention in the corpus is the Cycles help string above. Anyone needing to capture Blender's output separately cannot learn from these sources which handle to capture.
- **No list of logging categories.** The sample shows six, and `--log` examples name `event`, `render`, `cycles`, `mesh`, `operator`, `context` and `video`. The full list is only obtainable by running `--log-list-categories`.
- **The columns of the log format are never labelled.** My reading of the first column as elapsed time is inferred from the sample, not documented.
- **Nothing about buffering, flushing or line-by-line delivery**, which is what would decide whether the output can be read as it arrives. I searched for these terms and found nothing.
- **The manual does not document the 5.0 output format at all.** Only the release notes do. The manual's console screenshots are older — the Windows one shows a `Blender\2.77\config` path and messages in an earlier format — so they do not show what 5.0 prints.
- **No sample of what the output looks like when a render fails.** Every sample line in the sources is from a successful run.
- **The exact division between what `--quiet` suppresses and what survives is stated only as "status printing"** versus "warnings & errors"; which of the render lines above count as which is not said.

**Observed, 2026-08-06 — the questions this answer could not close, closed by running them.**

Run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender 5.0.1, `-b`, its own settings folder, display turned off first. Both machines were run; the whole set is in `Preparation/Blender Research/Findings/5.0/Reports.md`.

**Which stream — the thing 4e could only infer.** On a plain successful single-frame render, standard output held all 53 lines and standard error held nothing. With `-- --cycles-print-stats` the statistics also arrived on **standard output**, at the very end of the run, after the closing `render | Time:` line. So the help text quoted in 4e, "Print rendering statistics to stderr", does not describe where they actually appear — on either machine.

**What `--log-file` captures — this is the question that was sent back for research and that no source answers.** Run as `-b simple.blend --log-file "<path>" -o … -f 1`, with no `--log` category named, Blender 5.0 **moved the whole log into the file**. Standard output dropped to three lines — the version banner, a blank line and `Blender quit` — and the file held 50 lines, from `blend | Read blend:` through every sample line to the closing `render | Time:`. It is a move, not a copy. On a failing render the failure message goes into the file too and never appears on standard output, though the exit status is still 1. **Blender 4.5 does the opposite**: it creates the file, leaves it empty, and still prints everything to standard output. Whether it behaves the same with Blender's window open was not tested here.

**What `-q` removes, exactly.** The plain run printed 53 lines and the quiet run 50. Comparing the two with their timestamps stripped, **the three lines removed are** the `blend | Read blend: "…"` line, a blank line, and `Blender quit`. The whole render log survives.

**How many logging categories there are.** `--log-list-categories` listed **169** on this machine, against 135 on the Mac. The Linux list carries a run of `ghost.wl.*` window-system names. Which exact names differ cannot be worked out, because the Mac's list was kept only as a count.

**Where `--debug-cycles` writes.** On standard output, as ordinary `cycles` log lines. Blender 4.5 puts the same material on standard error instead, in a different shape entirely.

**Whether the output can be read while the render runs.** A two-frame render was piped into a reader that stamped each line with the seconds since the run began. The lines arrived spread across the run: the line Blender stamped `00:06.044` reached the reader at 6.14 seconds, and the line stamped `00:11.931` reached it at 12.03 seconds. So a program watching Blender's output does see it before the end.

**Outside the interface.** This answer is largely about outside the interface already — the whole of it concerns what Blender writes to a console, which is how it is read when Blender is driven from outside its window. Three documented points bear on the difference. The release notes say the logging system covers "background render progress", so progress is written in background mode. The `render_stats` handler documents that the statistics carry additional content "in background mode": the frame, and used and peak memory. And the Python release notes warn that "render farms parsing this output may need to be updated", which is the sources acknowledging this output is consumed by things other than a person reading a terminal. Against that, `--log-file` allows the output to be sent to a file rather than a console, and `-b` is what puts Blender into background mode in the first place. What the sources do not say is whether the format is identical inside and outside, beyond the one `render_stats` difference noted.

**Operating system.** The content is not said to differ by operating system, but reaching it does, and all three differences are documented. On Windows the Console Window "is first created as a separate window on the desktop", is then "toggled off", and is brought back with `Window ‣ Toggle System Console`. On macOS you must run the executable inside the application bundle "to get all output printed to the terminal", the path being `./Blender.app/Contents/MacOS/Blender`. On Linux it is simply the terminal Blender was launched from. The manual also warns generally that "If you launch Blender from an icon within your graphical desktop environment, you may not see the console window." One category in the sample, `ghost.system`, carries the message "Create Cocoa system", which is macOS-specific, so at least some category content is platform-dependent. And the 5.0 release notes state "Support for Intel Macs has been removed", narrowing the systems in question.

**Both machines have now been run.** They agree on which stream carries what, on quiet mode, on the logging switches and on the shape of a log line. Two differences belong to the machine rather than to Blender: the category list is longer on Linux (169 against 135) and so is the built-in help (479 lines against 466). One belongs to the display rather than to Blender: on a Linux machine with a display running, a windowless Blender may have display-library complaints appear on its standard error, which vanish once the display is turned off. **Windows is untested and no Windows machine has run it.**

---

## Where I looked

This is the part the pass exists for, so it is given in full. All searches were run across `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0` together unless noted, using recursive case-insensitive text search over whole folders rather than opening files one at a time. The manual holds 2195 text files and 3004 images; the API holds 1862 files.

**Search terms run for question 2:** render failed, failed to render, rendering failed, cannot render, unable to render, render error, render aborted, render stopped, nothing to render, fail, failed, failure, fails, error, errors, error message, error report, report message, warning message, shows an error, displays an error, reported, error_set, Wm Report Items, report, reports, has_reports, cannot, can't, unable, invalid, missing, aborted, abort, cancel, canceled, cancelled, CANCELLED, FINISHED, stopped, interrupted, out of memory, no camera, camera not set, camera missing, not found, exit code, exit-code, return code, status code, errorlevel, stderr, standard error, crash, does not exist, no such, unsupported, not supported, could not, denied, permission, disk full, write error, read error, render result, Info editor, status bar, popup, pop-up, dialog, notification, toast, banner, RuntimeError, exception, traceback, poll, sys.exit.

**Search terms run for question 4:** console, console window, system console, terminal, stdout, standard output, stderr, standard error, stream, output stream, print, prints, printing, print(), output, log, logs, logging, log level, log file, log categories, logging categories, --log, --log-level, --log-file, --log-list-categories, --verbose, --quiet, verbosity, verbose, quiet, suppress, debug, debug messages, statistics, stats, print stats, render stats, update_stats, status text, progress, progress bar, elapsed, Fra:, Mem:, Time:, Peak:, Saved:, Sample N, peak memory, memory usage, used memory, render time, saving, saved, tile, pipe, flush, redirect, line-buffered, output to, written to the console, printed to, prints to, to the console, to the terminal.

**Pages opened and read, question 2:** the Info editor page and its picture `editors_info-editor_ui.png`; the Status Bar page and its picture `interface_window-system_status-bar_ui-center.png`; the topbar page in full, including its Window menu and Render menu; `bpy.types.RenderEngine`; `bpy_types_enum_items/wm_report_items`; `bpy_types_enum_items/wm_job_type_items`; `bpy.ops` overview; `bpy.ops.render`; `bpy.ops.info`; `bpy.types.Operator`; `bpy.types.RenderResult`; `bpy.app.handlers`; `bpy.app`; `info_quickstart`; `info_gotchas_operators`; `info_tips_and_tricks`; the Cycles GPU rendering page; the EEVEE light settings page; the Cycles OSL page; the file paths page; the render output introduction, animation and output-properties pages; the troubleshooting index, crash, startup and network pages; one representative GPU troubleshooting page; the 5.0 release notes for core, rendering, python_api, user_interface, index, and searches across bugfixes and corrective_releases; and in Cycles' own files, `engine.py`, `__init__.py`, `operators.py`, `osl.py` in the relevant regions, plus searches across `properties.py` and `ui.py`.

**Pages opened and read, question 4:** the command-line index, arguments, render, and all three launch pages (Linux, macOS, Windows); the pictures `advanced_command-line_launch_windows_console.png` and `advanced_command-line_launch_linux_console.png`; the troubleshooting startup page; the render output-properties page; `bpy.types.Context`; `bpy.app.handlers`; `bpy.types.RenderEngine`; the 5.0 release notes for core and python_api; the glossary; the manual's general index; and `engine.py`, `properties.py` and `ui.py` in Cycles' own files.

**Ruled out, and why.** The `cycles_5.0/source` folder was searched and set aside: it holds the renderer's device-side kernel headers, and its only printing is kernel debug helpers such as `print_float4` and `print_projection` in `source/util/`. Nothing there produces the running output question 4 asks about. The Cycles `shader` and `license` folders were not relevant to either question. The Python console editor page, `bpy.ops.console` and `bpy.types.ConsoleLine` were excluded from the console searches once identified as Blender's built-in Python console — a different thing from the operating-system console, as the API itself distinguishes: "this is not the built-in Python console but a terminal application which is used to start Blender". The `4.5`, `5.1` and `5.2` source folders were not opened at all.

**Negative results worth recording, with the searches that produced them.** Searching for exit codes, return codes, exit status and errorlevel across all four folders returned only `--python-exit-code`. Searching for standard error and stderr returned exactly one relevant line, in `cycles_5.0/engine.py`. Searching for the word stream returned only video and audio uses. Searching for popup, dialog, notification, toast and banner within the render, interface and editors sections, filtered to lines also mentioning error, failure or warning, returned nothing at all. Searching for buffering, flushing, redirection and piping returned nothing relevant. Searching for "no camera" returned two hits, one about blend-file thumbnails and one a Story Tools viewport fix, neither a render failure. The glossary has no entry for console, log or terminal. Searching for the literal tokens `Saved:`, `Time:`, `Mem:` and `Sample N` across all four folders returned the release-notes sample and nothing else, which is what establishes that the sample in `core.md` is the only one of its kind in these sources.

---

## Things a person with Blender 5.0 open could settle in a few minutes

Nine of the ten items that were on this list have been run — from a terminal on the Creator's own MacBook Pro on 2026-08-05 and again on a rented Linux machine on 2026-08-06. Their results are in the answers above and in `Preparation/Blender Research/Findings/5.0/Reports.md`. In short: everything goes to standard output including the Cycles statistics; there are 135 logging categories on the Mac and 169 on Linux; a failed render prints its message and leaves the process at status 1; `-q` removes exactly three lines; `--log-file` moves the whole log into the file rather than copying it; `--debug-cycles` is accepted and writes `cycles` log lines; the first column is elapsed time since launch; and a render called from Python raises a `RuntimeError` carrying the message.

Two are left:

1. **Where `error_set` puts its message.** Register a small render engine whose `render` method calls `self.error_set("test failure")`, then render once with the interface open and once with `-b`. Note whether the text appears in the status bar, the Info editor, the header, the console, or more than one. Nothing run so far has registered a render engine of its own.

2. **Whether `--verbose` still works.** `--debug-cycles` was run and is accepted. `--verbose` was not tried, and the release notes and the manual disagree about whether it still exists.

One thing was tried and did not settle: **whether an error entry stays in the Info editor**. It was looked at on the rented Linux machine with the window open, and no entry appeared there at all when the render was started from a script rather than by hand, so nothing follows from it either way.

---

2026-08-05 — Opus 5 (Claude Code) — Reports, Blender 5.0

2026-08-06 — Opus 5 (Claude Code) — added the hand tests run on a rented Linux machine with an NVIDIA card to questions 2 and 4, kept separate from the Mac's results; answered what the log-file switch captures, which lines quiet mode removes, which stream the Cycles statistics use and what the render operator hands a script; brought both standing lines up to date and pruned the list above

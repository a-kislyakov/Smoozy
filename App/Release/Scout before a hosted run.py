#!/usr/bin/env python3
"""The scout before a hosted run: five checks on this Mac, for nothing, of
faults a hosted Windows or Linux run would otherwise find at a cost.

Creator-stated, 2026-11-15: "first scout the code for things a build would find
as faults anyway before actually running it on GitHub". The release runner
runs it before every hosted run (Preparation/Groundwork/Templates/Release
Runner.md, "Scout before any hosted run"); anyone may run it before one.

Run it from anywhere, with no arguments:

    python3 "App/Release/Scout before a hosted run.py"

or, to name the commit of the last release yourself for the fourth check:

    python3 "App/Release/Scout before a hosted run.py" --since <commit>

The five checks:

1. Every tracked path is one a Windows checkout can write. The test
   `the_selected_shell::tests::a_windows_checkout_can_write_every_path`.
2. No test runs a Mac-only command or opens a Mac-only path unless it is
   limited to macOS, and no test hands Python a script built in the test on
   its command line unless it is kept off Windows (Windows refuses a command
   line past 32,767 characters). The two tests in
   `App/Release/tests/the_scout_before_a_hosted_run.rs`.
3. The workspace and its tests compile for Windows and for Linux:
   `cargo check --workspace --tests --target <each>`. What this Mac cannot
   check for a real reason (a library written in C that wants that system's
   own headers or libraries) is named, never counted as passing.
4. The layout tests added since the last release whose expectations are
   exact pixels, so a different font could fail them. Printed for a person
   to read; it never fails.
5. The tests that draw the app's pages in Chrome, drawn again in the faces
   the other two systems draw in: Windows' line heights (Segoe UI's, on
   Tahoma's letters) with Consolas, and Linux's width (Verdana, with Menlo,
   which is DejaVu Sans Mono's letters). Copies of the fonts with their line
   heights set are made in the build folder; the app's own pages are copied,
   never changed. A failure is a fault. Where this Mac has no Consolas (it
   comes with Microsoft Office), the Windows drawing is named as not done.

It ends with "The scout found nothing." or with what it found, and its exit
code is 0 only for the first. It builds into the folder the release uses
(target-b), one job pair at a time, and never runs the app.

The account is `Audits/Build/Step 11/The scout before hosted runs — what
landed.md`.
"""

import json
import os
import pathlib
import re
import subprocess
import sys

APP = pathlib.Path(__file__).resolve().parent.parent
REPOSITORY = APP.parent
TARGETS = (("Windows", "x86_64-pc-windows-msvc"), ("Linux", "x86_64-unknown-linux-gnu"))

# The desk's own settings, unless the caller has already chosen others.
for name, value in (
    ("CARGO_TARGET_DIR", "target-b"),
    ("CARGO_INCREMENTAL", "0"),
    ("CARGO_BUILD_JOBS", "2"),
    ("SMOOZY_DEVELOPMENT_TELEGRAM_REACHES_NOTHING", "1"),
    ("TMPDIR", "/tmp"),
):
    os.environ.setdefault(name, value)


def say(text=""):
    print(text, flush=True)


def stop(why):
    say()
    say("Stopped: " + why)
    sys.exit(2)


def run(arguments, **more):
    return subprocess.run(arguments, cwd=APP, text=True, **more)


# --- 1 and 2: the two tests ------------------------------------------------

def a_test(title, arguments):
    say()
    say(title)
    done = run(arguments, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    if done.returncode == 0:
        say("  passed")
        return []
    lines = done.stdout.splitlines()
    shown = [line for line in lines if line.startswith("  ") or "panicked" in line or line.startswith("error")]
    for line in (shown or lines)[-60:]:
        say("  " + line)
    return [title]


# --- 3: the two other systems ----------------------------------------------

def why_this_mac_cannot_build(package, block):
    """A build script that fails here for a real reason, or None.

    A `-sys` package is, by the ecosystem's own naming, the bridge to a library
    written in C, and `ring` compiles C and assembly of its own; on this Mac
    they want the other system's C headers or its installed libraries, which
    only that system has. The desktop's own build script wants Windows' tool
    for its icon and version resource, `llvm-rc`. Anything else is a fault."""
    if package.endswith("-sys") or package.endswith("-sys-rs") or package == "ring":
        return "a library written in C, which wants that system's own headers or libraries"
    if package == "smoozy-desktop" and "llvm-rc" in block:
        return "Windows' resource compiler (llvm-rc), for the window's icon and version"
    return None


def the_members():
    read = json.loads(run(["cargo", "metadata", "--no-deps", "--format-version", "1", "--offline"],
                          stdout=subprocess.PIPE, check=True).stdout)
    expected, names = {}, {}
    for package in read["packages"]:
        names[package["id"]] = package["name"]
        for target in package["targets"]:
            if target.get("test", True) and set(target["kind"]) & {"lib", "rlib", "cdylib", "staticlib", "bin", "test", "proc-macro"}:
                expected.setdefault(package["name"], set()).add(target["name"])
    return expected, names


def the_settings_for(triple):
    """`ring`'s C for Linux built with no Linux C library, as `App/.cargo/config.toml`
    builds it for the server program's own Linux target, so the members that
    reach `ring` and nothing else in C are checked for Linux too. Nothing is
    linked or kept: a check makes no program. Windows has no such way here:
    `ring`'s C asks for Windows' own `stdlib.h` (tried 2026-11-15)."""
    settings = dict(os.environ)
    if triple == "x86_64-unknown-linux-gnu":
        name = triple.replace("-", "_")
        settings.setdefault("CFLAGS_" + name, "-nostdlibinc -ffreestanding -DRING_CORE_NOSTDLIBINC=1")
        settings.setdefault("CC_" + name, str(APP / ".cargo" / "cc-with-public-paths.py"))
        settings.setdefault("AR_" + name, str(APP / ".cargo" / "ar-keeping-linux-objects.sh"))
    return settings


def a_system(system, triple, members, names):
    say()
    say(f"3. The workspace and its tests compile for {system} ({triple})")
    done = run(["cargo", "check", "--workspace", "--tests", "--keep-going", "--target", triple,
                "--message-format", "json"], stdout=subprocess.PIPE, stderr=subprocess.PIPE,
               env=the_settings_for(triple))
    faults, checked = [], {}
    for line in done.stdout.splitlines():
        try:
            message = json.loads(line)
        except ValueError:
            continue
        if message.get("reason") == "compiler-artifact":
            name = names.get(message["package_id"])
            if name:
                checked.setdefault(name, set()).add(message["target"]["name"])
        elif message.get("reason") == "compiler-message" and message["message"]["level"] == "error":
            faults.append(message["message"].get("rendered") or message["message"]["message"])
    # Build scripts that did not run: each one's block of the error output.
    blocks = re.split(r"\n(?=error)", done.stderr)
    cannot = {}
    for block in blocks:
        failed = re.match(r"error: failed to run custom build command for `(\S+) v", block)
        if not failed:
            continue
        why = why_this_mac_cannot_build(failed.group(1), block)
        if why:
            cannot.setdefault(why, []).append(failed.group(1))
        else:
            faults.append(block.strip())
    if done.returncode != 0 and not faults and not cannot:
        faults.append(done.stderr.strip()[-4000:] or "cargo check failed and said nothing")
    unchecked = {member: sorted(targets - checked.get(member, set()))
                 for member, targets in members.items() if targets - checked.get(member, set())}
    whole = sorted(member for member, targets in unchecked.items() if targets == sorted(members[member]))
    partly = {member: targets for member, targets in unchecked.items() if member not in whole}
    say(f"  checked: {len(members) - len(whole)} of {len(members)} members"
        + (f" ({len(partly)} of them in part)" if partly else ""))
    for why, packages in sorted(cannot.items()):
        say(f"  not built here, for a real reason: {', '.join(sorted(set(packages)))}: {why}")
    if whole:
        say(f"  so these members could not be checked for {system} on this Mac, and only the hosted run checks them:")
        for member in whole:
            say(f"    {member}")
    for member, targets in sorted(partly.items()):
        say(f"    {member}, in part; not checked: {', '.join(targets)}")
    if unchecked and not cannot:
        faults.append(f"{len(unchecked)} members were not checked for {system} and no reason was found")
    for fault in faults:
        say()
        for line in fault.splitlines()[:40]:
            say("  " + line)
    if not faults:
        say("  no fault in what could be checked")
    return [f"{system}: {len(faults)} compile faults"] if faults else []


# --- 5: the pages drawn in the other systems' faces -------------------------

SUPPLEMENTAL = pathlib.Path("/System/Library/Fonts/Supplemental")
# Consolas is Windows' and Microsoft Office's; Office for Mac carries it.
CONSOLAS_FOLDERS = [pathlib.Path("/Applications") / f"Microsoft {app}.app/Contents/Resources/DFonts"
                    for app in ("Word", "Excel", "PowerPoint", "Outlook")]
# Windows' line heights, in units of 2048 to the em: Segoe UI's (Windows'
# system-ui face) and Consolas' own (its Windows ascent and descent). Read off
# the drawing: with these, the four layout tests that failed on Windows in
# hosted run 43746222478 failed at the desk with Windows' own numbers.
SEGOE_UI_LINE = (2210, 514)
CONSOLAS_LINE = (1884, 514)


def the_tables(data):
    count = int.from_bytes(data[4:6], "big")
    found = {}
    for i in range(count):
        at = 12 + 16 * i
        found[data[at:at + 4].decode("latin1")] = (int.from_bytes(data[at + 8:at + 12], "big"),
                                                    int.from_bytes(data[at + 12:at + 16], "big"))
    return found


def a_sum(data):
    data = data + b"\0" * (-len(data) % 4)
    return sum(int.from_bytes(data[i:i + 4], "big") for i in range(0, len(data), 4)) & 0xFFFFFFFF


def a_face_with_this_line(source, made, ascent, descent):
    """A copy of a TrueType font whose line heights (hhea, OS/2 typographic and
    Windows) are `ascent` and `descent`, with empty tables left out, since
    Chrome refuses a font carrying one (Consolas' `prep`)."""
    data = source.read_bytes()
    tables = {tag: bytearray(data[at:at + length]) for tag, (at, length) in the_tables(data).items() if length}
    hhea, os2, head = tables["hhea"], tables["OS/2"], tables["head"]
    hhea[4:10] = ascent.to_bytes(2, "big", signed=True) + (-descent).to_bytes(2, "big", signed=True) + bytes(2)
    selection = int.from_bytes(os2[62:64], "big") & ~0x80
    os2[62:64] = selection.to_bytes(2, "big")
    os2[68:78] = (ascent.to_bytes(2, "big", signed=True) + (-descent).to_bytes(2, "big", signed=True) + bytes(2)
                  + ascent.to_bytes(2, "big") + descent.to_bytes(2, "big"))
    head[8:12] = bytes(4)
    count = len(tables)
    power = 1 << (count.bit_length() - 1)
    out = bytearray(data[:4] + count.to_bytes(2, "big") + (power * 16).to_bytes(2, "big")
                    + (power.bit_length() - 1).to_bytes(2, "big") + (count * 16 - power * 16).to_bytes(2, "big"))
    body, at = bytearray(), 12 + 16 * count
    for tag in sorted(tables):
        table = bytes(tables[tag])
        out += tag.encode("latin1") + a_sum(table).to_bytes(4, "big") + (at + len(body)).to_bytes(4, "big") \
            + len(table).to_bytes(4, "big")
        body += table + b"\0" * (-len(table) % 4)
    out += body
    place = the_tables(bytes(out))["head"][0]
    out[place + 8:place + 12] = ((0xB1B0AFBA - a_sum(bytes(out))) & 0xFFFFFFFF).to_bytes(4, "big")
    made.write_bytes(out)


def the_pages_in(faces, web, made):
    """A copy of the app's `web/` drawn in `faces` (the CSS for `--face-ui` and
    `--face-mono`, with any `@font-face` before it). Every module waits for the
    faces first, so nothing is measured in the Mac's own while they load."""
    import shutil
    if made.exists():
        shutil.rmtree(made)
    made.mkdir(parents=True)
    for one in web.iterdir():
        if one.suffix in (".js", ".css"):
            shutil.copy(one, made / one.name)
    with open(made / "the-drawing.css", "a", encoding="utf-8") as sheet:
        sheet.write("\n/* The scout: another system's faces. */\n" + faces["css"] + "\n")
    (made / "the-faces-load.js").write_text(
        "await Promise.all(%s.map((face) => document.fonts.load(face)));\n" % json.dumps(faces["wait"]),
        encoding="utf-8")
    for one in made.glob("*.js"):
        if one.name != "the-faces-load.js":
            one.write_text("import './the-faces-load.js';\n" + one.read_text(encoding="utf-8"), encoding="utf-8")


def the_faces(folder):
    """Each system's faces, or why this Mac cannot make them."""
    faces, cannot = {}, {}
    tahoma = [SUPPLEMENTAL / "Tahoma.ttf", SUPPLEMENTAL / "Tahoma Bold.ttf"]
    consolas = next(([found / "Consola.ttf", found / "Consolab.ttf"] for found in CONSOLAS_FOLDERS
                     if (found / "Consola.ttf").is_file() and (found / "Consolab.ttf").is_file()), None)
    if not all(one.is_file() for one in tahoma):
        cannot["Windows"] = "this Mac has no Tahoma in " + str(SUPPLEMENTAL)
    elif not consolas:
        cannot["Windows"] = "this Mac has no Consolas (it comes with Microsoft Office)"
    else:
        folder.mkdir(parents=True, exist_ok=True)
        rules = []
        for family, sources, line in (("Scout Windows UI", tahoma, SEGOE_UI_LINE), ("Scout Windows Mono", consolas, CONSOLAS_LINE)):
            for weight, source in zip(("100 549", "550 900"), sources):
                made = folder / (family.replace(" ", "-") + "-" + weight.split()[0] + ".ttf")
                a_face_with_this_line(source, made, *line)
                rules.append('@font-face { font-family: "%s"; src: url("%s"); font-weight: %s; }'
                             % (family, made.as_uri(), weight))
        faces["Windows"] = {
            "css": "\n".join(rules) + '\n:root, :root[data-theme] { --face-ui: "Scout Windows UI" !important;'
                   ' --face-mono: "Scout Windows Mono" !important; }',
            "wait": ["400 13px 'Scout Windows UI'", "700 13px 'Scout Windows UI'",
                     "400 13px 'Scout Windows Mono'", "700 13px 'Scout Windows Mono'"],
        }
    if not (SUPPLEMENTAL / "Verdana.ttf").is_file():
        cannot["Linux"] = "this Mac has no Verdana in " + str(SUPPLEMENTAL)
    else:
        faces["Linux"] = {
            "css": ':root, :root[data-theme] { --face-ui: Verdana !important; --face-mono: Menlo !important; }',
            "wait": ["400 13px Verdana", "700 13px Verdana", "400 13px Menlo"],
        }
    return faces, cannot


def the_pages_in_other_faces():
    import shutil
    import tempfile
    say()
    say("5. The pages drawn in Chrome, drawn again in Windows' and Linux's faces")
    folder = APP / os.environ["CARGO_TARGET_DIR"] / "scout-faces"
    faces, cannot = the_faces(folder / "fonts")
    for system, why in sorted(cannot.items()):
        say(f"  {system}: not drawn, so only the hosted run draws it: {why}")
    found = []
    for system, these in sorted(faces.items()):
        pages = folder / system
        the_pages_in(these, APP / "Desktop app and delivery" / "web", pages)
        # The tests' own scratch folders go in one short folder of the scout's,
        # removed after, rather than staying in /tmp.
        scratch = tempfile.mkdtemp(prefix="smoozy-scout-", dir="/tmp")
        settings = dict(os.environ, TMPDIR=scratch, SMOOZY_UI_SIZE_WEB=str(pages),
                        SMOOZY_THE_FOCUS_RING_WEB=str(pages))
        failed = []
        try:
            for title, arguments in (
                ("the layout tests", ["cargo", "test", "-p", "smoozy-desktop", "--test", "the_ui_at_every_size",
                                      "--", "--test-threads=2"]),
                ("the screens' own tests", ["cargo", "test", "-p", "smoozy-desktop", "--lib", "--", "surface::",
                                            "--test-threads=2"]),
            ):
                done = run(arguments, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, env=settings)
                lines = done.stdout.splitlines()
                names = [line[5:-len(" ... FAILED")] for line in lines
                         if line.startswith("test ") and line.endswith(" ... FAILED")]
                if done.returncode == 0:
                    say(f"  {system}, {title}: passed")
                    continue
                failed.append(title)
                say(f"  {system}, {title}: {len(names) or 'some'} failed")
                # Each failure's place and the line after it, which says what it found.
                said = [line for at, line in enumerate(lines)
                        if "panicked" in line or (at and "panicked" in lines[at - 1]) or line.startswith("error")]
                for line in (said or lines)[-40:]:
                    say("      " + line[:300])
        finally:
            shutil.rmtree(scratch, ignore_errors=True)
        if failed:
            found.append(f"{system}'s faces: {', '.join(failed)}")
    return found


# --- 4: layout tests with exact pixels -------------------------------------

def the_last_release(given):
    if given:
        return given, "given"
    manifest = (APP / "Cargo.toml").read_text(encoding="utf-8")
    version = re.search(r'^\[workspace\.package\][^\[]*?^version\s*=\s*"([^"]+)"', manifest, re.M | re.S)
    if not version:
        stop("App/Cargo.toml names no workspace version")
    current = tuple(int(part) for part in version.group(1).split("."))
    listed = subprocess.run(["git", "-C", str(REPOSITORY), "for-each-ref", "--format=%(refname:short) %(objectname)",
                             "refs/heads/release-*", "refs/remotes/origin/release-*"],
                            text=True, stdout=subprocess.PIPE, check=True).stdout
    releases = {}
    for line in listed.splitlines():
        name, commit = line.split()
        found = re.search(r"release-(\d+)\.(\d+)\.(\d+)$", name)
        if found:
            numbers = tuple(int(part) for part in found.groups())
            if numbers < current:
                releases[numbers] = commit
    if not releases:
        stop("no release branch older than this version was found; give the last release with --since <commit>")
    newest = max(releases)
    return releases[newest], "release-" + ".".join(map(str, newest))


MEASURES = re.compile(r"\bbox\(|getBoundingClientRect|offset(Width|Height|Top|Left)\b|client(Width|Height)\b"
                      r"|scroll(Width|Height)\b")
# A measured place or size on the line itself: a box's edges or size, a run of
# them, or a measure the page hands back by name. Colours, words and corners
# are not what a different font moves.
A_PLACE_OR_SIZE = re.compile(r"\bbox\(|\bboxes\b|\.(height|width|top|left|right|bottom)\b|offset(Width|Height|Top|Left)\b"
                             r"|client(Width|Height)\b|scroll(Width|Height)\b|\b(lefts|tops|rights|bottoms|heights|widths)\b"
                             r"|\[\"\w*(Left|Top|Right|Bottom|Height|Width|Moved)\w*\"\]")
EXACT = re.compile(r"(?<![=!<>])(===|!==|==|!=)(?!=)")
# A tolerance of two pixels or less: `<=1`, `< 0.5`, `<2`.
NO_ROOM = re.compile(r"<=?\s*[0-2](\.\d+)?(?![\d.])")
NOT_PIXELS = (
    re.compile(r"\.(length|size)\s*(===|!==|==|!=)\s*0\b"),
    re.compile(r"\.length\s*(===|!==|==|!=)\s*\d+"),
    re.compile(r"(===|!==|==|!=)\s*(true|false|null|undefined)\b"),
    re.compile(r"(===|!==|==|!=)\s*(['\"])(?!-?\d+(\.\d+)?px\2)[^'\"]*\2"),
    re.compile(r"(['\"])(?!-?\d+(\.\d+)?px\1)[^'\"]*\1\s*(===|!==|==|!=)"),
)


def expects_exact_pixels(line, measured):
    """A check of a measured place or size whose expectation leaves no room for
    a different font: an equality, or a tolerance of two pixels or less. Counts,
    words and settings compared for equality are taken out first."""
    if "check(" not in line and "assert" not in line:
        return False
    if not A_PLACE_OR_SIZE.search(line) and not any(re.search(r"\b" + re.escape(name) + r"\b", line) for name in measured):
        return False
    for taken_out in NOT_PIXELS:
        line = taken_out.sub(" ", line)
    return bool(EXACT.search(line) or NO_ROOM.search(line))


def a_body(text, start):
    """The text of the function whose name is at `start`, to its closing brace,
    with strings and comments stepped over so their braces do not count."""
    at = text.find("{", start)
    depth, i, n = 0, at, len(text)
    while 0 <= i < n:
        if text.startswith("//", i):
            i = text.find("\n", i)
            continue
        raw = re.match(r'b?r(#*)"', text[i:i + 8])
        if raw and not (text[i - 1].isalnum() or text[i - 1] == "_"):
            end = text.find('"' + raw.group(1), i + raw.end())
            i = end + 1 + len(raw.group(1))
            continue
        if text[i] == '"':
            i += 1
            while i < n and text[i] != '"':
                i += 2 if text[i] == "\\" else 1
            i += 1
            continue
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return text[at:i + 1]
        i += 1
    return text[at:]


def the_layout_tests(given):
    base, named = the_last_release(given)
    say()
    say(f"4. Layout tests added since the last release ({named}, {base[:9]}) that expect exact pixels")
    say("   Each line below expects a measured place or size to be exact, or within two pixels, so a different")
    say("   font could fail it on Windows or Linux. For a person to read; it does not fail the scout.")
    diff = subprocess.run(["git", "-C", str(REPOSITORY), "-c", "core.quotePath=false", "diff", "--unified=0",
                           "--no-renames", base, "--", "App"],
                          text=True, stdout=subprocess.PIPE, check=True).stdout
    added = {}
    file = None
    for line in diff.splitlines():
        if line.startswith("+++ "):
            # git ends a name holding a space with a tab.
            file = line[4:].rstrip("\t")
            file = file[2:] if file.startswith("b/") else None
        elif file and file.endswith(".rs") and line.startswith("+"):
            name = re.search(r"\bfn ([a-z0-9_]+)\s*\(", line)
            if name:
                added.setdefault(file, set()).add(name.group(1))
    shown = 0
    for file, names in sorted(added.items()):
        path = REPOSITORY / file
        if not path.exists():
            continue
        text = path.read_text(encoding="utf-8")
        for name in sorted(names):
            for found in re.finditer(r"\bfn " + name + r"\s*\(", text):
                before = text[max(0, found.start() - 400):found.start()]
                if not re.search(r"#\[test\]\s*(#\[[^\]]*\]\s*|///[^\n]*\n\s*)*$", before):
                    continue
                body = a_body(text, found.start())
                if not MEASURES.search(body):
                    continue
                first = text.count("\n", 0, found.start()) + 1
                # Names the test gives to measured places and sizes, so a check
                # of `ends` counts when `ends` was read off the boxes.
                measured = {name for line in body.splitlines() if A_PLACE_OR_SIZE.search(line)
                            for name in re.findall(r"\b(?:const|let|var)\s+(\w+)\s*=", line)}
                exact = [(first + offset, line.strip()) for offset, line in enumerate(body.splitlines())
                         if expects_exact_pixels(line, measured)]
                if exact:
                    shown += 1
                    say(f"  {file}:{first}  {name}")
                    for number, line in exact[:6]:
                        say(f"      {number}: {line[:220]}")
                    if len(exact) > 6:
                        say(f"      … and {len(exact) - 6} more lines")
    if not shown:
        say("  none")
    return shown


def main():
    arguments = sys.argv[1:]
    given = None
    if arguments[:1] == ["--since"] and len(arguments) == 2:
        given = arguments[1]
    elif arguments:
        stop('the one choice is --since <commit>, the last release\'s commit')
    installed = subprocess.run(["rustup", "target", "list", "--installed"], text=True,
                               stdout=subprocess.PIPE).stdout.split()
    for system, triple in TARGETS:
        if triple not in installed:
            stop(f"{system}'s target is not installed on this Mac. Run: rustup target add {triple}")
    if not (APP / os.environ["CARGO_TARGET_DIR"]).name.startswith("target-"):
        stop("CARGO_TARGET_DIR must be a target-* folder inside App/, never App/target")

    say("The scout before a hosted run, at " + subprocess.run(
        ["git", "-C", str(REPOSITORY), "log", "-1", "--format=%h %s"], text=True, stdout=subprocess.PIPE).stdout.strip()[:100])
    found = []
    found += a_test("1. Every tracked path is one a Windows checkout can write",
                    ["cargo", "test", "-p", "smoozy-desktop", "--lib", "--",
                     "the_selected_shell::tests::a_windows_checkout_can_write_every_path", "--exact"])
    found += a_test("2. No test runs a Mac-only command or path unless it is limited to macOS, and no test "
                    "hands Python a script on its command line unless it is kept off Windows",
                    ["cargo", "test", "-p", "smoozy-release", "--test", "the_scout_before_a_hosted_run"])
    members, names = the_members()
    for system, triple in TARGETS:
        found += a_system(system, triple, members, names)
    the_layout_tests(given)
    found += the_pages_in_other_faces()

    say()
    if found:
        say("The scout found faults, so a hosted run would fail: " + "; ".join(found) + ".")
        sys.exit(1)
    say("The scout found nothing.")


if __name__ == "__main__":
    main()

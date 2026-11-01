#!/usr/bin/env python3
"""Keep private build paths out of programs; used by both wrapper launchers."""
import os
import subprocess
import sys
from pathlib import Path


def public_path_prefixes():
    home = Path.home()
    project = Path(__file__).resolve().parents[2]
    prefixes = [
        (home, "/builder"),
        (Path(os.environ.get("RUSTUP_HOME", home / ".rustup")), "/rustup"),
        (Path(os.environ.get("CARGO_HOME", home / ".cargo")), "/cargo"),
        (home / "Library" / "Caches" / "Smoozy Test build", "/smoozy"),
        (project, "/smoozy"),
    ]
    # The last matching prefix wins. Cover both spellings on Windows too.
    return [(spelling, public) for source, public in prefixes
            for spelling in dict.fromkeys([str(source), source.as_posix()])]


def build_windows_launcher(output):
    """Bootstrap a native Cargo entry point without passing through cmd.exe."""
    output = Path(output).resolve()
    output.parent.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ,
               SMOOZY_PUBLIC_PATHS_HELPER=str(Path(__file__).resolve()),
               SMOOZY_PUBLIC_PATHS_PYTHON=sys.executable)
    subprocess.run(["rustc", str(Path(__file__).with_suffix(".rs")),
                    "--crate-name", "rustc_with_public_paths", "-o", str(output)],
                   env=env, check=True)


if __name__ == "__main__":
    if len(sys.argv) == 3 and sys.argv[1] == "--build-windows-launcher":
        build_windows_launcher(sys.argv[2])
        sys.exit(0)
    # On this Mac the compiler tools' LLVM library is beside the toolchain,
    # outside their own loader search path. Keep the fix local to this build.
    if sys.platform == "darwin":
        toolchain_lib = Path(sys.argv[1]).resolve().parent.parent / "lib"
        if (toolchain_lib / "libLLVM.dylib").is_file():
            previous = os.environ.get("DYLD_LIBRARY_PATH", "")
            os.environ["DYLD_LIBRARY_PATH"] = str(toolchain_lib) + (":" + previous if previous else "")
    compiler_args = sys.argv[2:]
    flags = [f"--remap-path-prefix={source}={public}"
             for source, public in public_path_prefixes()]
    target = None
    for index, arg in enumerate(compiler_args):
        if arg == "--target" and index + 1 < len(compiler_args):
            target = compiler_args[index + 1]
        elif arg.startswith("--target="):
            target = arg.split("=", 1)[1]
    if sys.platform == "darwin" and (target is None or target.endswith("-apple-darwin")):
        # The Mac linker saves object-file paths in OSO debugger symbols.
        # rustc's source remaps do not cover that separate debug map.
        flags += ["-C", f"link-arg=-Wl,-oso_prefix,{Path.home()}/"]
    command = [sys.argv[1], *compiler_args, *flags]
    if sys.platform == "win32":
        # Windows' CRT execv joins arguments without quoting. subprocess quotes
        # each argument for the native compiler, with no command shell involved.
        sys.exit(subprocess.call(command))
    os.execv(sys.argv[1], command)

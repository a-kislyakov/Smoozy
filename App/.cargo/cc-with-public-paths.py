#!/usr/bin/env python3
"""Keep the same private paths out of C source names and debug information."""
import os
import runpy
import sys
from pathlib import Path

prefixes = runpy.run_path(str(Path(__file__).with_name("rustc-with-public-paths.py")))["public_path_prefixes"]()
flags = [f"-ffile-prefix-map={source}={public}" for source, public in prefixes]
os.execvp("clang", ["clang", *sys.argv[1:], *flags])

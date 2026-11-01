"""Check the compiler handoffs, including Windows' native launcher on its runner."""
import argparse
import json
import os
from pathlib import Path
import runpy
import shutil
import subprocess
import sys
import unittest
from unittest.mock import patch


HELPER = Path(__file__).with_name("rustc-with-public-paths.py").resolve()
PREFIXES = runpy.run_path(str(HELPER))["public_path_prefixes"]()
FLAGS = [f"--remap-path-prefix={source}={public}" for source, public in PREFIXES]
ARGUMENTS = [
    "--remap-path-prefix=.=/smoozy/App", "--print=file-names", "a=b=c",
    "", "two words", "a\tb", 'feature="with quotes"', 'a\\\"b',
    "C:\\a folder\\trailing\\", "percent%PATH%", "bang!", "amp&pipe|",
    "caret^", "semi;comma,", "brackets()", "artist — 渲染",
]


class PublicPaths(unittest.TestCase):
    def test_arguments_stdin_and_output_survive_both_handoffs(self):
        code = ("import json,sys; print(json.dumps(sys.argv[1:])); "
                "print(sys.stdin.read()); print('compiler stderr', file=sys.stderr)")
        result = subprocess.run(
            [LAUNCHER, sys.executable, "-c", code, *ARGUMENTS],
            input="compiler stdin", text=True, capture_output=True, check=True, timeout=60)
        lines = result.stdout.splitlines()
        self.assertEqual(json.loads(lines[0])[:len(ARGUMENTS)], ARGUMENTS)
        self.assertEqual(json.loads(lines[0])[len(ARGUMENTS):len(ARGUMENTS) + len(FLAGS)], FLAGS)
        self.assertEqual(lines[1], "compiler stdin")
        self.assertEqual(result.stderr.strip(), "compiler stderr")

    def test_compiler_failure_reaches_cargo(self):
        result = subprocess.run([LAUNCHER, sys.executable, "-c", "import sys; sys.exit(37)"], timeout=60)
        self.assertEqual(result.returncode, 37)

    def test_windows_python_handoff_uses_a_list_without_a_shell(self):
        compiler = "C:\\tool chain\\rustc.exe"
        with patch.object(sys, "argv", [str(HELPER), compiler, *ARGUMENTS]), \
             patch.object(sys, "platform", "win32"), \
             patch("subprocess.call", return_value=37) as call, \
             patch("os.execv", side_effect=AssertionError("Windows must not use CRT execv")):
            with self.assertRaises(SystemExit) as exit:
                runpy.run_path(str(HELPER), run_name="__main__")
        self.assertEqual(exit.exception.code, 37)
        call.assert_called_once_with([compiler, *ARGUMENTS, *FLAGS])

    def test_the_original_cargo_probe_reaches_real_rustc(self):
        compiler = shutil.which("rustc")
        self.assertIsNotNone(compiler)
        result = subprocess.run(
            [LAUNCHER, compiler, "-", "--crate-name", "___", "--print=file-names",
             "--crate-type", "bin", "--remap-path-prefix=.=/smoozy/App"],
            input="", text=True, capture_output=True, check=True, timeout=60)
        self.assertEqual(result.stdout.strip(), "___.exe" if os.name == "nt" else "___")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--launcher", required=True)
    options = parser.parse_args()
    LAUNCHER = str(Path(options.launcher).resolve())
    unittest.main(argv=[sys.argv[0]], verbosity=2)

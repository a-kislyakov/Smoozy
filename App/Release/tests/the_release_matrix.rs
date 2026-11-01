//! Exercise the hosted package transformation and its read-back refusals at
//! the desk, without building, installing or launching a desktop application.
//! `Desktop app and delivery` E4b; step 11 D1.

#[cfg(unix)]
#[test]
fn the_hosted_package_checks_accept_the_rows_and_refuse_wrong_declarations() {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let workflow = include_str!("../../../.github/workflows/desktop-build.yml");
    let script = workflow
        .split_once("          python3 - <<'PY'\n")
        .expect("the Linux package step has its Python body")
        .1
        .split_once("          PY\n")
        .expect("the package step's Python body ends")
        .0
        .lines()
        .map(|line| line.strip_prefix("          ").unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n");
    let checks = r#"
import contextlib
import io
import tempfile
import unittest

class PackageChecks(unittest.TestCase):
    def control(self):
        return ('Package: smoozy\nVersion: 0.1.8\nArchitecture: amd64\n'
                'Depends: libc6 (>= 2.39), libgtk-3-0t64, libwebkit2gtk-4.1-0,\n'
                ' libgtk-3-0, libwebkit2gtk-4.1-0, libssl3t64\n'
                'Description: Smoozy\n')

    def test_every_row_keeps_other_dependencies_and_replaces_only_old_gtk(self):
        for row in ROWS:
            control = row_control(self.control(), row)
            self.assertIn('X-Smoozy-Release-Matrix-Row: ' + row + '\n', control)
            self.assertIn('libc6 (>= 2.39)', control)
            self.assertIn('libgtk-3-0t64', control)
            self.assertIn('libssl3t64', control)
            self.assertNotIn('libgtk-3-0,', control)
            self.assertEqual(control.count('libwebkit2gtk-4.1-0'), 1)
            self.assertIn('Description: Smoozy\n', control)

    def test_an_unknown_row_and_missing_or_repeated_dependency_field_refuse(self):
        for control, row in [(self.control(), 'ubuntu-22.04'),
                             ('Package: smoozy\n', ROWS[0]),
                             (self.control() + 'Depends: other\n', ROWS[0])]:
            with self.assertRaises(ValueError):
                row_control(control, row)

    def read(self, **changes):
        fields = {'Architecture': 'amd64', 'X-Smoozy-Release-Matrix-Row': ROWS[0],
                  'Depends': 'libc6 (>= 2.39), libgtk-3-0t64, libwebkit2gtk-4.1-0'}
        fields.update(changes)
        global run
        run = lambda *args: fields[args[-1]]
        with contextlib.redirect_stdout(io.StringIO()):
            check_package('built.deb', ROWS[0])

    def test_read_back_accepts_the_correct_package(self):
        self.read()

    def test_install_and_upgrade_accept_only_the_packages_own_distribution(self):
        with tempfile.TemporaryDirectory(prefix='smoozy-d1-') as temporary:
            release = pathlib.Path(temporary) / 'os-release'
            missing = pathlib.Path(temporary) / 'missing'
            for row in ROWS:
                script = row_preinstall(row).replace('/etc/os-release', str(release)).replace('/usr/lib/os-release', str(missing))
                for system in (*ROWS, 'ubuntu-22.04', 'debian-12', 'linuxmint-22', '-'):
                    identity, version = system.split('-', 1)
                    release.write_text('ID="' + identity + '"\nVERSION_ID="' + version + '"\n')
                    for action in ('install', 'upgrade'):
                        result = subprocess.run(['sh', '-s', '--', action], input=script, text=True, capture_output=True, timeout=5)
                        self.assertEqual(result.returncode == 0, system == row, (row, system, result.stderr))
                release.unlink()
                result = subprocess.run(['sh', '-s', '--', 'install'], input=script, text=True, capture_output=True, timeout=5)
                self.assertNotEqual(result.returncode, 0)
                result = subprocess.run(['sh', '-s', '--', 'abort-upgrade'], input=script, text=True, capture_output=True, timeout=5)
                self.assertEqual(result.returncode, 0)

    def test_read_back_refuses_wrong_architecture_row_floor_gtk_webview_and_tray(self):
        for changes in [
            {'Architecture': 'arm64'},
            {'X-Smoozy-Release-Matrix-Row': 'debian-12'},
            {'Depends': 'libgtk-3-0t64, libwebkit2gtk-4.1-0'},
            {'Depends': 'libc6 (>= 2.39), libgtk-3-0, libwebkit2gtk-4.1-0'},
            {'Depends': 'libc6 (>= 2.39), libgtk-3-0t64, libwebkit2gtk-4.0-0'},
            {'Depends': 'libc6 (>= 2.39), libgtk-3-0t64, libwebkit2gtk-4.1-0, libgtk-3-0'},
            {'Depends': 'libc6 (>= 2.39), libgtk-3-0t64, libwebkit2gtk-4.1-0, libayatana-appindicator3-1'},
        ]:
            with self.subTest(changes=changes), self.assertRaises(ValueError):
                self.read(**changes)

unittest.main(argv=['the_release_matrix'], exit=True)
"#;
    let input = format!("__name__ = 'package_checks'\n{script}\n{checks}");
    let mut child = Command::new("python3")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Python 3 is needed to check the hosted package step");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .expect("the package checks receive their input");
    assert!(child.wait().expect("the package checks end").success());
}

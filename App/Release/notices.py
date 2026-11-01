"""Build offline notices from Cargo's target tree and local package sources.

No third-party Python modules or installed helper tools. Cargo's normal tree selects the
packages; metadata supplies their exact versions, source roots and licence fields.
The desktop also distributes the Linux-musl server, listed in its own section.
"""
import argparse
import hashlib
from functools import lru_cache
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import unittest

SERVER_TARGET = 'x86_64-unknown-linux-musl'
NOTICE = 'THIRD-PARTY-LICENSES.txt'


def packages_in_tree(metadata, tree):
    """Match Cargo's selected tree to exact metadata identities, fail closed."""
    packages = {}
    for package in metadata['packages']:
        packages.setdefault((package['name'], package['version']), []).append(package)
    selected = {}
    for line in tree.splitlines():
        match = re.fullmatch(r'([^ ]+) v([^ ]+)(?: \(.*\))?(?: \(\*\))?', line.strip())
        if not match:
            raise ValueError('Unrecognized cargo tree line: ' + line)
        matches = packages.get(match.groups(), [])
        if len(matches) != 1:
            raise ValueError('Tree package has no unique metadata identity: ' + line)
        package = matches[0]
        selected[package['id']] = package
    if not selected:
        raise ValueError('Cargo returned an empty dependency tree')
    return sorted(selected.values(), key=lambda p: (p['name'], p['version'], p['id']))


def cargo(app, target_dir, *arguments):
    # metadata/tree do not accept --target-dir (Cargo's own --help). Neither
    # compiles anything. Pin the same folder using Cargo's environment setting.
    environment = dict(os.environ, CARGO_TARGET_DIR=str(target_dir))
    return subprocess.check_output(
        [environment.get('CARGO', 'cargo'), *arguments, '--locked', '--color', 'never'],
        cwd=app, env=environment, text=True, encoding='utf-8', timeout=180)


def target_packages(app, target_dir, target, root):
    metadata = json.loads(cargo(app, target_dir, 'metadata', '--format-version', '1',
                                '--filter-platform', target))
    tree = cargo(app, target_dir, 'tree', '--target', target, '--package', root,
                 '--edges', 'normal,no-proc-macro', '--prefix', 'none', '--format', '{p}', '--charset', 'ascii')
    return packages_in_tree(metadata, tree)


SUPPLEMENT_ROOT = Path(__file__).resolve().with_name('licence-supplements')


@lru_cache(maxsize=1)
def supplement_index():
    return json.loads((SUPPLEMENT_ROOT / 'index.json').read_text(encoding='utf-8'))


def supplements_for(package):
    item = supplement_index().get(package['name'] + '-' + package['version'])
    if item is None:
        return []
    vcs = Path(package['manifest_path']).parent / '.cargo_vcs_info.json'
    if json.loads(vcs.read_text(encoding='utf-8'))['git']['sha1'] != item['revision']:
        raise ValueError('The licence supplement does not match the package revision: ' + package['name'])
    for source in item['files']:
        path = SUPPLEMENT_ROOT / source['file']
        if hashlib.sha256(path.read_bytes()).hexdigest() != source['sha256']:
            raise ValueError('A licence supplement changed: ' + source['file'])
    return item['files']


def licence_files(package):
    root = Path(package['manifest_path']).parent
    found = set()
    explicit = package.get('license_file')
    if explicit:
        path = Path(explicit)
        if not path.is_absolute():
            path = root / path
        if not path.is_file():
            raise ValueError('Missing declared licence file: ' + package['name'])
        found.add(path)
    for path in root.rglob('*'):
        if path.is_file() and path.suffix.lower() not in {'.rs', '.c', '.h', '.cpp', '.hpp', '.py', '.js', '.json', '.toml'} and re.match(r'^(licen[cs]e|copying|copyright|notice|unlicense)([.\-_ ]|$)', path.name, re.I):
            found.add(path)
    # A package without licence material must be reviewed rather than silently
    # shipping just an SPDX expression. Keep all bundled native-library notices.
    for source in supplements_for(package):
        found.add(SUPPLEMENT_ROOT / source['file'])
    if not found:
        raise ValueError('No licence text in local package: ' + package['name'] + ' ' + package['version'])
    return sorted(found, key=lambda p: p.relative_to(root).as_posix() if p.is_relative_to(root) else p.relative_to(SUPPLEMENT_ROOT).as_posix())


def render_section(target, root_name, packages):
    lines = [f'{root_name} — {target}', '=' * 72, '', 'Packages:']
    outside = [p for p in packages if p.get('source') is not None]
    lines += [f"  {p['name']} {p['version']} — {p.get('license') or 'see licence text'}" for p in outside]
    for package in outside:
        root = Path(package['manifest_path']).parent
        lines += ['', '-' * 72, f"{package['name']} {package['version']}",
                  'Declared licence: ' + (package.get('license') or 'see licence text'),
                  'Source: ' + package['source']]
        if package.get('authors'):
            lines.append('Authors: ' + ', '.join(package['authors']))
        if package.get('repository'):
            lines.append('Repository: ' + package['repository'])
        for path in licence_files(package):
            label = (path.relative_to(root).as_posix() if path.is_relative_to(root) else
                     next(s['url'] for s in supplements_for(package) if SUPPLEMENT_ROOT / s['file'] == path))
            lines += ['', '--- ' + label + ' ---',
                      path.read_text(encoding='utf-8', errors='strict')]
        if package['name'] == 'libsqlite3-sys':
            # The bundled SQLite notice is in its public header, not a
            # LICENSE-named file. Keep the complete opening copyright notice.
            header = (root / 'sqlite3/sqlite3.h').read_text(encoding='utf-8')
            boundary = header.index('*************************************************************************')
            lines += ['', '--- sqlite3/sqlite3.h (opening copyright notice) ---',
                      header[:boundary] + '*/\n']
    return '\n'.join(lines) + '\n'


def generate(app, target_dir, target, into):
    desktop = target_packages(app, target_dir, target, 'smoozy-desktop')
    server = target_packages(app, target_dir, SERVER_TARGET, 'smoozy-server')
    server_text = render_section(SERVER_TARGET, 'smoozy-server', server)
    desktop_text = render_section(target, 'smoozy-desktop', desktop)
    into.mkdir(parents=True, exist_ok=True)
    (into / NOTICE).write_text('Third-party licences\n\n' + desktop_text + '\n' + server_text, encoding='utf-8')
    (into / 'THIRD-PARTY-LICENSES-server.txt').write_text('Third-party licences\n\n' + server_text, encoding='utf-8')
    shutil.copyfile(app.parent / 'COPYING', into / 'COPYING')
    opener = (app / 'Desktop app and delivery/src/the_address_opener.rs').read_text(encoding='utf-8')
    source_address = re.search(r'SOURCE_CODE_ADDRESS: &str = "([^"]+)";', opener).group(1)
    menu_words = (app / 'Desktop app and delivery/src/surface/menus.rs').read_text(encoding='utf-8')
    copyright_line = re.search(r'COPYRIGHT: &str = "([^"]+)";', menu_words).group(1)
    copyright_text = ('Smoozy\n' + copyright_line + '\n'
                      + 'Source: ' + source_address + '\n'
                      'Licence: GPL-3.0-or-later\n\n' + (app.parent / 'COPYING').read_text(encoding='utf-8')
                      + '\n' + desktop_text + '\n' + server_text)
    (into / 'copyright').write_text(copyright_text, encoding='utf-8')
    # Package inventory is separate from the human-readable text, for inspection.
    inventory = {target: [p['name'] + ' ' + p['version'] for p in desktop],
                 'server/' + SERVER_TARGET: [p['name'] + ' ' + p['version'] for p in server]}
    (into / 'packages.json').write_text(json.dumps(inventory, indent=2) + '\n', encoding='utf-8')


class SampleChecks(unittest.TestCase):
    def test_exact_target_tree_and_deduplication(self):
        sample = {'packages': [dict(id=n, name=n, version='1.0.0')
                               for n in ['desktop', 'shared', 'mac', 'windows', 'tests', 'builder', 'unused']]}
        mac = packages_in_tree(sample, 'desktop v1.0.0 (/project/desktop)\nshared v1.0.0\nmac v1.0.0\nshared v1.0.0 (*)\n')
        windows = packages_in_tree(sample, 'desktop v1.0.0 (/project/desktop)\nshared v1.0.0\nwindows v1.0.0\n')
        self.assertEqual([p['name'] for p in mac], ['desktop', 'mac', 'shared'])
        self.assertEqual([p['name'] for p in windows], ['desktop', 'shared', 'windows'])
        for bad in ['', 'unknown v1.0.0', 'not a Cargo line']:
            with self.assertRaises(ValueError):
                packages_in_tree(sample, bad)
        with self.assertRaises(ValueError):
            packages_in_tree({'packages': sample['packages'] + [dict(id='other-source', name='shared', version='1.0.0')]}, 'shared v1.0.0')

    def test_bundled_native_notices_and_missing_text(self):
        with tempfile.TemporaryDirectory(prefix='smoozy-notices-') as temporary:
            root = Path(temporary)
            package = dict(name='native', version='1', manifest_path=str(root / 'Cargo.toml'), license_file=None)
            (root / 'copying.rs').write_text('// A copying operation, not a licence')
            with self.assertRaises(ValueError):
                licence_files(package)
            (root / 'LICENSE').write_text('Rust notice')
            (root / 'vendor').mkdir()
            (root / 'vendor' / 'COPYING').write_text('C library notice')
            self.assertEqual([p.relative_to(root).as_posix() for p in licence_files(package)], ['LICENSE', 'vendor/COPYING'])
            package['license_file'] = 'missing'
            with self.assertRaises(ValueError):
                licence_files(package)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--target')
    parser.add_argument('--target-dir', type=Path)
    parser.add_argument('--into', type=Path)
    parser.add_argument('--self-test', action='store_true')
    args = parser.parse_args()
    if args.self_test:
        unittest.main(argv=[sys.argv[0]], exit=True)
    if not all([args.target, args.target_dir, args.into]):
        parser.error('--target, --target-dir and --into are required')
    app = Path(__file__).resolve().parents[1]
    generate(app, args.target_dir.resolve(), args.target, args.into.resolve())


if __name__ == '__main__':
    main()

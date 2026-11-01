"""Measure one built artifact and pack it the same way for every candidate.

Called once per candidate per operating system. It reports the payload's own
byte total, packs the payload into a zip with fixed settings so the archive
figure is comparable, and lists what the binary asks the operating system for.

Nothing here is part of Smoozy. This is the throwaway that fills the comparison
sheet at `Preparation/Architecture/Desktop app and delivery.md` H5b.
"""

import argparse
import hashlib
import json
import os
import struct
import subprocess
import sys
import zipfile

FIXED_TIME = (1980, 1, 1, 0, 0, 0)


def sha256_of_file(path):
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def walk(root):
    """Every regular file under root, sorted, as (relative path, full path)."""
    if os.path.isfile(root):
        return [(os.path.basename(root), root)]
    found = []
    for base, dirs, names in os.walk(root):
        dirs.sort()
        for name in sorted(names):
            full = os.path.join(base, name)
            if os.path.islink(full) or not os.path.isfile(full):
                continue
            found.append((os.path.relpath(full, os.path.dirname(root)), full))
    found.sort()
    return found


def payload_bytes(root):
    """The sum of the regular files' own sizes. Symbolic links count as nothing
    and are listed separately, because a link holds no payload of its own."""
    total = 0
    count = 0
    links = 0
    for _, full in walk(root):
        total += os.path.getsize(full)
        count += 1
    if os.path.isdir(root):
        for base, _, names in os.walk(root):
            for name in names:
                if os.path.islink(os.path.join(base, name)):
                    links += 1
    return total, count, links


def pack(root, out_path):
    """One zip, deflate at level 9, fixed timestamps, sorted entries, and the
    executable bit kept. Identical settings for both candidates on all three
    systems, so the archive figures compare."""
    with zipfile.ZipFile(out_path, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
        for rel, full in walk(root):
            info = zipfile.ZipInfo(rel.replace(os.sep, "/"), date_time=FIXED_TIME)
            info.compress_type = zipfile.ZIP_DEFLATED
            mode = os.stat(full).st_mode
            executable = bool(mode & 0o111)
            info.external_attr = ((0o755 if executable else 0o644) << 16)
            with open(full, "rb") as handle:
                archive.writestr(info, handle.read())
    return os.path.getsize(out_path)


def find_binary(root, name):
    """The candidate's own executable inside a built artifact."""
    if os.path.isfile(root):
        return root
    for base, _, names in os.walk(root):
        for entry in names:
            full = os.path.join(base, entry)
            if os.path.islink(full):
                continue
            stem = entry[:-4] if entry.lower().endswith(".exe") else entry
            if stem == name and os.access(full, os.X_OK):
                return full
    return None


def macos_dependencies(binary):
    out = subprocess.run(["otool", "-L", binary], capture_output=True, text=True)
    if out.returncode != 0:
        return None
    lines = [line.strip() for line in out.stdout.splitlines()[1:] if line.strip()]
    return [line.split(" (")[0] for line in lines]


def linux_dependencies(binary):
    out = subprocess.run(["objdump", "-p", binary], capture_output=True, text=True)
    if out.returncode != 0:
        return None
    needed = []
    for line in out.stdout.splitlines():
        parts = line.split()
        if len(parts) == 2 and parts[0] == "NEEDED":
            needed.append(parts[1])
    return sorted(needed)


def windows_dependencies(binary):
    """Read the PE import table directly. Returns None rather than guessing if
    anything about the file is not what this reader expects."""
    try:
        with open(binary, "rb") as handle:
            data = handle.read()
        if data[:2] != b"MZ":
            return None
        pe_at = struct.unpack_from("<I", data, 0x3C)[0]
        if data[pe_at:pe_at + 4] != b"PE\0\0":
            return None
        machine, sections, _, _, _, opt_size, _ = struct.unpack_from("<HHIIIHH", data, pe_at + 4)
        opt_at = pe_at + 24
        magic = struct.unpack_from("<H", data, opt_at)[0]
        if magic == 0x20B:      # PE32+
            dirs_at = opt_at + 112
        elif magic == 0x10B:    # PE32
            dirs_at = opt_at + 96
        else:
            return None
        import_rva, import_size = struct.unpack_from("<II", data, dirs_at + 8)
        if import_rva == 0:
            return []
        sec_at = opt_at + opt_size
        sections_list = []
        for i in range(sections):
            base = sec_at + i * 40
            v_size, v_addr, r_size, r_ptr = struct.unpack_from("<IIII", data, base + 8)
            sections_list.append((v_addr, max(v_size, r_size), r_ptr))

        def to_offset(rva):
            for v_addr, size, r_ptr in sections_list:
                if v_addr <= rva < v_addr + size:
                    return r_ptr + (rva - v_addr)
            return None

        names = []
        cursor = to_offset(import_rva)
        if cursor is None:
            return None
        while True:
            entry = data[cursor:cursor + 20]
            if len(entry) < 20 or entry == b"\0" * 20:
                break
            name_rva = struct.unpack_from("<I", entry, 12)[0]
            if name_rva == 0:
                break
            name_at = to_offset(name_rva)
            if name_at is None:
                break
            end = data.index(b"\0", name_at)
            names.append(data[name_at:end].decode("ascii", "replace"))
            cursor += 20
        return sorted(set(names))
    except Exception:
        return None


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--candidate", required=True)
    parser.add_argument("--system", required=True)
    parser.add_argument("--payload", required=True, help="the built .app, directory or file")
    parser.add_argument("--binary-name", required=True)
    parser.add_argument("--archive", required=True, help="where to write the zip")
    parser.add_argument("--out", required=True, help="where to write the reading")
    args = parser.parse_args()

    payload = os.path.abspath(args.payload)
    if not os.path.exists(payload):
        print("payload not found: " + payload, file=sys.stderr)
        return 1

    total, count, links = payload_bytes(payload)
    archive_bytes = pack(payload, args.archive)
    binary = find_binary(payload, args.binary_name)

    if args.system == "macos":
        dependencies = macos_dependencies(binary) if binary else None
    elif args.system == "linux":
        dependencies = linux_dependencies(binary) if binary else None
    else:
        dependencies = windows_dependencies(binary) if binary else None

    reading = {
        "candidate": args.candidate,
        "system": args.system,
        "payload_path": os.path.basename(payload),
        "payload_bytes": total,
        "payload_file_count": count,
        "payload_symlink_count": links,
        "executable_path": os.path.relpath(binary, os.path.dirname(payload)) if binary else None,
        "executable_bytes": os.path.getsize(binary) if binary else None,
        "executable_sha256": sha256_of_file(binary) if binary else None,
        "archive_name": os.path.basename(args.archive),
        "archive_bytes": archive_bytes,
        "archive_sha256": sha256_of_file(args.archive),
        "linked_dependencies": dependencies,
        "files": [
            {"path": rel.replace(os.sep, "/"), "bytes": os.path.getsize(full)}
            for rel, full in walk(payload)
        ],
    }
    with open(args.out, "w") as handle:
        json.dump(reading, handle, indent=2, sort_keys=True)
    print(json.dumps({k: v for k, v in reading.items() if k != "files"}, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())

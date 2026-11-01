#!/bin/sh
# The archiver Cargo hands to `cc` when `ring`'s C and assembly are built for
# the server program's Linux target, `x86_64-unknown-linux-musl`. It is named in
# `App/.cargo/config.toml`, beside the compiler settings for the same build.
#
# Why it exists: the Mac's own `ar` drops a Linux object from an archive when it
# writes the archive's index, because the index writer reads only Mac objects
# ("ranlib: warning: archive member 't.o' not a mach-o file", and the archive
# then holds nothing; observed 2026-10-24). The Mac has no other archiver. Asked
# not to write an index (the `S` modifier), the same `ar` keeps every object.
# No index is needed: the Rust compiler copies the objects into its own library
# file for `ring` and writes its own index there, and the program links
# (observed 2026-10-24).
#
# `cc` calls an archiver twice: once with `cq` or `cqD` to add the objects, and
# once with `s` or `sD` to write the index. The first becomes `qcS`; the second
# does nothing. Anything else is passed to `ar` unchanged. GNU `ar` on a Linux
# runner reads `qcS` the same way.
mode="$1"
shift
case "$mode" in
  *q*) exec ar qcS "$@" ;;
  s | sD) exit 0 ;;
  *) exec ar "$mode" "$@" ;;
esac

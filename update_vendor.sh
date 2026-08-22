#!/bin/sh

set -eu

project_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
manifest="$project_dir/src/rust/Cargo.toml"
archive="$project_dir/src/rust/vendor.tar.xz"
work_dir=$(mktemp -d "${TMPDIR:-/tmp}/ymd-vendor.XXXXXX")

cleanup() {
    rm -rf "$work_dir"
}
trap cleanup EXIT HUP INT TERM

echo "Vendoring locked Rust dependencies..."
cargo vendor \
    --locked \
    --versioned-dirs \
    --manifest-path "$manifest" \
    "$work_dir/vendor" \
    >/dev/null

archive_tmp="$work_dir/vendor.tar.xz"

if command -v gtar >/dev/null 2>&1; then
    tar_cmd=gtar
elif tar --version 2>/dev/null | grep -q 'GNU tar'; then
    tar_cmd=tar
else
    echo "GNU tar is required to create a deterministic vendor archive." >&2
    echo "On macOS, install it with: brew install gnu-tar" >&2
    exit 1
fi

echo "Creating deterministic archive without extended attributes..."
COPYFILE_DISABLE=1 "$tar_cmd" \
    --create \
    --xz \
    --file "$archive_tmp" \
    --directory "$work_dir" \
    --sort=name \
    --mtime='@0' \
    --owner=0 \
    --group=0 \
    --numeric-owner \
    --format=pax \
    --pax-option=delete=atime,delete=ctime \
    --no-xattrs \
    --no-acls \
    vendor

if xz -dc "$archive_tmp" | strings | grep -Eq 'LIBARCHIVE\.xattr|SCHILY\.xattr|com\.apple'; then
    echo "The generated archive still contains extended-attribute metadata." >&2
    exit 1
fi

mv "$archive_tmp" "$archive"
echo "Updated $archive"

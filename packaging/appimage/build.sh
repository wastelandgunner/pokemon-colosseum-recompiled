#!/bin/sh
# Builds colo-viewer-x86_64.AppImage (into colo-rs/target-appimage/).
# The binary is compiled in an Ubuntu 22.04 container (podman, or docker with CONTAINER=docker) so it needs only
# glibc 2.35; NATIVE=1 builds on the host instead (the AppImage then needs the host's glibc or newer).
# The AppImage runtime is RUNTIME=<file> if given (e.g. the first `--appimage-offset` bytes of any type-2 AppImage),
# else AppImage/type2-runtime's runtime-x86_64 is downloaded. Nothing from the game is bundled: the user supplies
# their own disc image at run time.
set -eu
here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../.." && pwd)"
out="$root/target-appimage"
mkdir -p "$out"

if [ "${NATIVE:-0}" = 1 ]; then
    CARGO_TARGET_DIR="$out/target" cargo build --release -p colo-viewer
    command -v mksquashfs >/dev/null || { echo "mksquashfs (squashfs-tools) not found" >&2; exit 1; }
else
    ctr="${CONTAINER:-podman}"
    "$ctr" build -t colo-appimage-build -f "$here/Containerfile" "$here"
    mkdir -p "$out/cargo-registry"
    # podman maps the container's root to you; with docker add CTR_ARGS="--user $(id -u):$(id -g)".
    "$ctr" run --rm -v "$root:/src" -v "$out/cargo-registry:/opt/cargo/registry" -w /src \
        -e CARGO_TARGET_DIR=/src/target-appimage/target ${CTR_ARGS:-} colo-appimage-build cargo build --release -p colo-viewer
fi

app="$out/AppDir"
rm -rf "$app"
mkdir -p "$app/usr/bin"
cp "$out/target/release/colo-viewer" "$app/usr/bin/"
strip "$app/usr/bin/colo-viewer" 2>/dev/null || true
cp "$here/AppRun" "$here/colo-viewer.desktop" "$here/colo-viewer.svg" "$app/"
ln -sf colo-viewer.svg "$app/.DirIcon"

runtime="${RUNTIME:-$out/runtime-x86_64}"
[ -f "$runtime" ] || curl -fL -o "$runtime" https://github.com/AppImage/type2-runtime/releases/download/continuous/runtime-x86_64

img="$out/colo-viewer-x86_64.AppImage"
rm -f "$out/AppDir.squashfs"
if [ "${NATIVE:-0}" = 1 ]; then
    mksquashfs "$app" "$out/AppDir.squashfs" -root-owned -noappend -comp zstd -quiet
else
    "$ctr" run --rm -v "$root:/src" colo-appimage-build \
        mksquashfs /src/target-appimage/AppDir /src/target-appimage/AppDir.squashfs -root-owned -noappend -comp zstd -quiet
fi
cat "$runtime" "$out/AppDir.squashfs" > "$img"
chmod +x "$img"
rm -f "$out/AppDir.squashfs"
echo "$img"

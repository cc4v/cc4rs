#!/bin/sh

set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)

if ! command -v sokol-shdc >/dev/null 2>&1; then
    echo "sokol-shdc was not found in PATH" >&2
    exit 1
fi

LANGUAGES=${SOKOL_SHADER_LANGUAGES:-glsl430:metal_macos:hlsl5}
INPUTS=$(mktemp)
trap 'rm -f "$INPUTS"' EXIT HUP INT TERM
find "$ROOT_DIR/examples" -type f -name '*.glsl' -print > "$INPUTS"

if [ ! -s "$INPUTS" ]; then
    echo "no shader sources found under examples/" >&2
    exit 1
fi

while IFS= read -r input; do
    output="$(dirname "$input")/shader.rs"
    echo "$input -> $output"
    sokol-shdc \
        -i "$input" \
        -o "$output" \
        -l "$LANGUAGES" \
        -f sokol_rust
done < "$INPUTS"
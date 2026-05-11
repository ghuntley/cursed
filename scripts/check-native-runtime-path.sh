#!/bin/sh
set -eu

repo_root="$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)"
compiler="$repo_root/zig-out/bin/cursed-compiler"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

source_file="$tmpdir/runtime-path.💀"
output_file="$tmpdir/runtime-path"

cat >"$source_file" <<'EOF'
vibe main
yeet "vibez"

slay main_character() {
    vibez.spill("cwd-runtime")
}
EOF

set +e
(
    cd "$tmpdir"
    "$compiler" --compile --output="$output_file" "$source_file" >"$tmpdir/stdout" 2>"$tmpdir/stderr"
)
status=$?
set -e

if [ "$status" -ne 0 ]; then
    printf '%s\n' "expected native compile to work from cwd $tmpdir" >&2
    printf 'exit=%s\n' "$status" >&2
    cat "$tmpdir/stderr" >&2
    exit 1
fi

if [ ! -x "$output_file" ]; then
    printf '%s\n' "expected output binary at $output_file" >&2
    cat "$tmpdir/stderr" >&2
    exit 1
fi

"$output_file" >"$tmpdir/run.stdout" 2>"$tmpdir/run.stderr"
printf '%s\n' 'cwd-runtime' >"$tmpdir/expected.stdout"

if ! cmp -s "$tmpdir/expected.stdout" "$tmpdir/run.stdout"; then
    printf '%s\n' 'unexpected run stdout' >&2
    cat "$tmpdir/run.stdout" >&2
    exit 1
fi

if grep -q '/home/ghuntley' "$tmpdir/stderr"; then
    printf '%s\n' 'native compile used maintainer-specific runtime path' >&2
    cat "$tmpdir/stderr" >&2
    exit 1
fi

if ! grep -q 'src-zig/cursed_runtime.c' "$tmpdir/stderr"; then
    printf '%s\n' 'native compile diagnostics did not show runtime source path' >&2
    cat "$tmpdir/stderr" >&2
    exit 1
fi

#!/bin/sh
set -eu

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

source_file="$tmpdir/clang-failure.💀"
output_file="$tmpdir/clang-failure"
fake_bin="$tmpdir/bin"
mkdir -p "$fake_bin"

cat >"$source_file" <<'EOF'
vibe main
yeet "vibez"

slay main_character() {
    vibez.spill("clang-failure")
}
EOF

cat >"$fake_bin/clang" <<'EOF'
#!/bin/sh
printf '%s\n' 'fake clang failure' >&2
exit 42
EOF
chmod +x "$fake_bin/clang"

set +e
PATH="$fake_bin:$PATH" ./zig-out/bin/cursed-compiler --compile --output="$output_file" "$source_file" >"$tmpdir/stdout" 2>"$tmpdir/stderr"
status=$?
set -e

if [ "$status" -eq 0 ]; then
    printf '%s\n' 'expected cursed-compiler --compile to fail when clang fails' >&2
    printf '%s\n' 'stdout:' >&2
    cat "$tmpdir/stdout" >&2
    printf '%s\n' 'stderr:' >&2
    cat "$tmpdir/stderr" >&2
    exit 1
fi

if [ -e "$output_file" ]; then
    printf '%s\n' "expected no output binary at $output_file" >&2
    exit 1
fi

if ! grep -Eq 'fake clang failure|ClangFailed|Compilation failed' "$tmpdir/stderr"; then
    printf '%s\n' 'expected clang failure diagnostics on stderr' >&2
    cat "$tmpdir/stderr" >&2
    exit 1
fi

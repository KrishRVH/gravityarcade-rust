#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"

WITH_REFERENCE=0
if [[ "${1:-}" == "--with-reference" ]]; then
    WITH_REFERENCE=1
elif [[ "${1:-}" != "" ]]; then
    echo "usage: $0 [--with-reference]" >&2
    exit 1
fi

export PYTHONDONTWRITEBYTECODE=1

run() {
    printf '==> %q' "$1"
    shift
    for arg in "$@"; do
        printf ' %q' "$arg"
    done
    printf '\n'
    "$@"
}

run cargo-fmt cargo fmt --check
run cargo-test cargo test --quiet
run cargo-clippy cargo clippy --all-targets -- -D warnings
run verify-generated scripts/verify-generated-assets.sh
run verify-sound scripts/verify-sound-assets.sh
run native-smoke scripts/capture-native-smoke.sh

if [[ "$WITH_REFERENCE" == "1" ]]; then
    run reference-compare scripts/compare-reference-smoke.sh --capture
fi

run release-build cargo build --release --quiet
ls -lh target/release/gravityarcade

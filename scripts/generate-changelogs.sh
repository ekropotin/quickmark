#!/usr/bin/env bash
set -euo pipefail

if [[ $# -eq 0 ]]; then
    echo "Usage: $0 <crate1> [<crate2> ...]"
    exit 1
fi

REPO_ROOT=$(git rev-parse --show-toplevel)
cd "$REPO_ROOT"

for crate in "$@"; do
    echo "Processing crate: $crate"

    # Get crate manifest path from cargo metadata
    CRATE_DIR=$(cargo metadata --format-version 1 --no-deps \
        | jq -r --arg NAME "$crate" '.packages[] | select(.name==$NAME) | .manifest_path' \
        | xargs dirname)

    REL_CRATE_DIR=$(realpath --relative-to="$REPO_ROOT" "$CRATE_DIR")

    echo "Generating changelog for $crate in $REL_CRATE_DIR..."
    git cliff \
        --tag-pattern "$crate@*" \
        --include-path "$REL_CRATE_DIR/**" \
        --output "$REL_CRATE_DIR/CHANGELOG.md"

    git add "$REL_CRATE_DIR/CHANGELOG.md"
done

echo "✅ Changelogs staged for crates: $*"

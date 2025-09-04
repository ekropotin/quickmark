#!/usr/bin/env bash
set -euo pipefail
# set -x

# Resolve script directory (so paths.sh works regardless of cwd)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT=$(git rev-parse --show-toplevel)

source "${SCRIPT_DIR}/lib.sh"
# Todo: automate?
crates=(quickmark-core quickmark-cli quickmark-server)
changed_crates=()

for crate in "${crates[@]}"; do
  echo "Checking version for crate: $crate"
  next_ver=$(next_version "$crate")
  if [ -n "$next_ver" ]; then
    echo "Found changes for $crate, bumping to version: $next_ver"
    changed_crates+=("$crate")
    if ! cargo release version "$next_ver" -p "$crate" --execute --no-confirm; then
      echo "Error: Failed to bump version for crate '$crate'" >&2
      exit 1
    fi
  else
    echo "No changes detected for crate: $crate"
  fi
done

if [ ${#changed_crates[@]} -eq 0 ]; then
    echo "No changes since the last release"
    exit 0
fi

echo "Creating release commit..."
if ! cargo release commit --execute --no-confirm; then
  echo "Error: Failed to create release commit" >&2
  exit 1
fi

echo "Creating release tags..."
if ! cargo release tag --execute --no-confirm; then
  echo "Error: Failed to create release tags" >&2
  exit 1
fi

NEW_TAGS=$(git tag --points-at HEAD)
echo "Tags created locally:"
echo "$NEW_TAGS"

echo "generating changelogs..."
for crate in "${changed_crates[@]}"; do
  CRATE_DIR=$(cargo metadata --format-version 1 --no-deps \
      | jq -r --arg NAME "$crate" '.packages[] | select(.name==$NAME) | .manifest_path' \
      | xargs dirname)
  
  # Validate that crate directory was found
  if [[ -z "$CRATE_DIR" || ! -d "$CRATE_DIR" ]]; then
    echo "Error: Could not find directory for crate '$crate'" >&2
    exit 1
  fi

  REL_CRATE_DIR=$(realpath --relative-to="$REPO_ROOT" "$CRATE_DIR")

  # Build include paths array
  mapfile -t include_args < <(get_crate_include_args "$crate")
  
  echo "Generating changelog for $crate..."
  if ! git cliff \
      --tag-pattern "$crate@*" \
      "${include_args[@]}" \
      --output "$REL_CRATE_DIR/CHANGELOG.md"; then
    echo "Error: Failed to generate changelog for crate '$crate'" >&2
    exit 1
  fi

  git add "$REL_CRATE_DIR/CHANGELOG.md"
done

echo "Amending commit to include changelogs..."
git commit --amend --no-edit

for tag in $NEW_TAGS; do
    echo "Re-applying tag $tag on amended commit..."
    git tag -f "$tag"
done

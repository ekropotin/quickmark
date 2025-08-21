#!/usr/bin/env bash
set -euo pipefail

# List of workspace crates
CRATES=("quickmark-core" "quickmark-cli" "quickmark-server")

# Loop through each crate
for crate in "crates/${CRATES[@]}"; do
  echo "Processing crate: $crate"

  # Find the last tag for this crate
  LAST_TAG=$(git tag --list "${crate}-v*" --sort=-creatordate | head -n 1 || true)
  echo "Last tag for $crate: $LAST_TAG"

  # Detect if crate changed since last tag
  if [ -z "$LAST_TAG" ]; then
    # No previous tag → consider it changed
    CHANGED=true
  else
    # Check for changes in crate directory
    if git diff --quiet "$LAST_TAG"..HEAD -- "$crate/"; then
      CHANGED=false
    else
      CHANGED=true
    fi
  fi

  if [ "$CHANGED" = true ]; then
    echo "Changes detected in $crate"

    # Determine bump type
    # Default to patch
    BUMP="patch"

    # Look at commits affecting this crate since last tag
    if [ -n "$LAST_TAG" ]; then
      COMMITS=$(git log "$LAST_TAG"..HEAD --pretty=%s -- "$crate/")
    else
      COMMITS=$(git log HEAD --pretty=%s -- "$crate/")
    fi

    # Check for breaking changes
    if echo "$COMMITS" | grep -q "BREAKING CHANGE"; then
      BUMP="major"
    # Check for features
    elif echo "$COMMITS" | grep -q "^feat"; then
      BUMP="minor"
    fi

    echo "Bump type for $crate: $BUMP"

    # Execute cargo-workspaces version command
    echo "Running: cargo workspaces version $crate --bump $BUMP --tag"
    # cargo workspaces version "$crate" --bump "$BUMP" --tag
  else
    echo "No changes in $crate, skipping."
  fi

  echo "-----------------------------------"
done

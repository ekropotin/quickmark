#!/usr/bin/env bash
set -euo pipefail
# set -x

# Resolve script directory (so paths.sh works regardless of cwd)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT=$(git rev-parse --show-toplevel)

source "${SCRIPT_DIR}/lib.sh"

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <crate-name>" >&2
  exit 1
fi

CRATE="$1"

mapfile -t include_args < <(get_crate_include_args "$CRATE")

changes=`git cliff \
    --tag-pattern "$CRATE@*" \
    "${include_args[@]}" \
    --latest`

echo "$changes"

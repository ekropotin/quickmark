# Function: get_crate_include_args
# Arguments:
#   $1 - crate name (e.g., "quickmark-cli")
# Returns:
#   Prints a single-line string with all --include-path arguments for
#   the crate and its internal workspace dependencies.
get_crate_include_args() {
    local crate="$1"

    # Find git root
    local git_root
    git_root=$(git rev-parse --show-toplevel)

    # Get workspace metadata
    local workspace
    workspace=$(cargo metadata --format-version 1 --no-deps)

    # Start with main crate
    local paths=()
    local crate_path
    crate_path=$(jq -r --arg name "$crate" '
        .packages[] | select(.name == $name) | .manifest_path
    ' <<<"$workspace")
    
    # Validate that crate was found
    if [[ -z "$crate_path" || "$crate_path" == "null" ]]; then
        echo "Error: Could not find crate '$crate' in workspace" >&2
        return 1
    fi
    
    paths+=("$(realpath --relative-to="$git_root" "$(dirname "$crate_path")")")

    # Add direct workspace dependencies
    while read -r dep; do
        local dep_path
        dep_path=$(jq -r --arg dep "$dep" '
            .packages[] | select(.name == $dep) | .manifest_path
        ' <<<"$workspace")
        
        # Validate that dependency was found
        if [[ -z "$dep_path" || "$dep_path" == "null" ]]; then
            echo "Warning: Could not find dependency '$dep' in workspace" >&2
            continue
        fi
        
        paths+=("$(realpath --relative-to="$git_root" "$(dirname "$dep_path")")")
    done < <(jq -r --arg name "$crate" '
        .packages as $pkgs
        | $pkgs[] | select(.name == $name) as $target
        | $target.dependencies[] | select(.source == null) | .name
    ' <<<"$workspace")

    # deduplicate and convert to --include-path array
    local include_args=()
    for p in $(printf "%s\n" "${paths[@]}" | sort -u); do
        include_args+=(--include-path "$p/**/*")
    done

    # print each element on a separate line
    printf "%s\n" "${include_args[@]}"
}

# Function: next_version
# Arguments:
#   $1 - crate name (e.g., "quickmark-cli")
# Returns:
#   Prints the next version of the crate according to git-cliff,
#   considering its dependencies. Returns empty string if no bump.
next_version() {
    local crate="$1"

    # Build include paths array
    local include_args
    mapfile -t include_args < <(get_crate_include_args "$crate")

    # Run git-cliff
    local output
    output=$(git cliff \
        --tag-pattern "${crate}@*" \
        "${include_args[@]}" \
        --bumped-version 2>&1 || true)

    # Return empty if warning present
    if grep -q "WARN" <<<"$output"; then
        echo ""
        return 0
    fi

    # Extract version after last @
    local version
    version=$(grep -E "^.*@" <<<"$output" | tail -n1 | sed 's/.*@//')

    echo "$version"
}

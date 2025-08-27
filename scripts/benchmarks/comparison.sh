#!/bin/sh

SCRIPT_DIR=$(cd $(dirname $0); pwd)
PROJECT_ROOT=($SCRIPT_DIR/../..)
DATA_ROOT=$SCRIPT_DIR/data
DOC_PATH=$DATA_ROOT/gitlab/doc

# Build hyperfine command array dynamically
HYPERFINE_COMMANDS=""

# Always include quickmark
HYPERFINE_COMMANDS="$HYPERFINE_COMMANDS \"QUICKMARK_CONFIG=$SCRIPT_DIR/quickmark.toml $PROJECT_ROOT/target/release/qmark $DOC_PATH\""

# Check if mdl is available
if command -v mdl >/dev/null 2>&1; then
    HYPERFINE_COMMANDS="$HYPERFINE_COMMANDS \"mdl --config $SCRIPT_DIR/.mdlrc $DOC_PATH\""
else
    echo "Warning: mdl not found in PATH, skipping mdl benchmark"
fi

# Check if mado is available
if command -v mado >/dev/null 2>&1; then
    HYPERFINE_COMMANDS="$HYPERFINE_COMMANDS \"mado --config $SCRIPT_DIR/mado.toml check $DOC_PATH\""
else
    echo "Warning: mado not found in PATH, skipping mado benchmark"
fi

# Always include markdownlint
HYPERFINE_COMMANDS="$HYPERFINE_COMMANDS \"$SCRIPT_DIR/node_modules/.bin/markdownlint --config $SCRIPT_DIR/.markdownlint.yaml $DOC_PATH\""

# Execute hyperfine with dynamic command list
eval "hyperfine --ignore-failure $HYPERFINE_COMMANDS --export-json report.json"

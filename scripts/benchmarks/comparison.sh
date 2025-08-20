#!/bin/sh

SCRIPT_DIR=$(cd $(dirname $0); pwd)
PROJECT_ROOT=($SCRIPT_DIR/../..)
DATA_ROOT=$SCRIPT_DIR/data
DOC_PATH=$DATA_ROOT/gitlab/doc

cargo build --release

hyperfine --ignore-failure --warmup 10 \
  "QUICKMARK_CONFIG=$SCRIPT_DIR/quickmark.toml $PROJECT_ROOT/target/release/qmark $DOC_PATH" \
  "mdl --config $SCRIPT_DIR/.mdlrc $DOC_PATH" \
  "$SCRIPT_DIR/node_modules/.bin/markdownlint --config $SCRIPT_DIR/.markdownlint.jsonc $DOC_PATH" \
  --export-json report.json

#!/bin/sh

SCRIPT_DIR=$(cd $(dirname $0); pwd)
DATA_ROOT=$SCRIPT_DIR/data

# NOTE: Use the same datasets as those used by vale for benchmarking
#       https://github.com/errata-ai/vale?tab=readme-ov-file#benchmarks
#
echo "Preaparing dataset"
cd $DATA_ROOT
git clone --sparse --filter=blob:none https://gitlab.com/gitlab-org/gitlab.git
cd gitlab
git sparse-checkout set doc
git reset --hard 7d6a4025a0346f1f50d2825c85742e5a27b39a8b
git checkout

echo "Installing hyperfine"
cargo install hyperfine

echo "Installing other linters"
npm install
gem install mdl

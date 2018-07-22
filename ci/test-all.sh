#!/bin/bash

# Exit script on the first error
set -o errexit -o nounset

export RUSTFLAGS="--deny warnings"

cargo test --verbose

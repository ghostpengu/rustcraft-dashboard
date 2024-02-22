#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

cargo build --release --target aarch64-unknown-linux-gnu
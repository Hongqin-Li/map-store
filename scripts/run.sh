#!/usr/bin/env bash

set -o errexit

cargo build --release

# src=datasets/urldata.csv
# src=datasets/1GB.txt
src=datasets/normal-2GB.txt
# src=datasets/identical-1GB.txt

src_sz=$(du $src -k | cut -f1)
max_mem=$(($src_sz / 100))
# max memory in KB

echo "memory limit: $max_mem KB"

# perl scripts/timeout -c target/release/brute-force $src -o target/tmp
perl scripts/timeout -c -m $max_mem target/release/map-reduce $src --nmaps 500
# flamegraph target/release/map-reduce $src --nmaps 500

echo "finish"

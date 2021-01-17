#!/usr/bin/env bash

set -o errexit


# cargo run --release --bin data-generator -- distinct -o datasets/dist-1GB.txt --size 1

cargo build --release

prefix=/media/lhq/Data/Workspace/datasets
# src=$prefix/urldata.csv
# src=$prefix/1GB.txt

# src=$prefix/normal-1GB.txt
src=$prefix/dist-1GB.txt
# src=$prefix/identical-1GB.txt

# src=$prefix/distinct-10GB.txt


src_sz=$(du $src -k | cut -f1)
max_mem=$(($src_sz / 100))
# max memory in KB

echo "memory limit: $max_mem KB"

# perl scripts/timeout -c target/release/brute-force $src -o target/tmp
# perl scripts/timeout -c -m $max_mem target/release/map-reduce $src -k 10 --nmaps 500
# perl scripts/timeout -c -m $max_mem target/release/map-store $src -k 10 --nmaps 500


# flamegraph target/release/map-reduce $src -k 10 --nmaps 500
flamegraph target/release/map-store $src -k 10 --nmaps 500

echo "finish"

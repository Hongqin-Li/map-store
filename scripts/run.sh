#!/usr/bin/env bash

set -o errexit

cargo build --release

# src=datasets/urldata.csv
src=datasets/50.txt

src_sz=$(du datasets/urldata.csv -k | cut -f1)
max_mem=$(($src_sz * 10))
# max memory in KB

echo "memory limit: $max_mem KB"

perl scripts/timeout -c -m $max_mem target/release/brute-force $src -o target/tmp
perl scripts/timeout -c -m $max_mem target/release/map-reduce $src --nmaps 500

echo "finish"
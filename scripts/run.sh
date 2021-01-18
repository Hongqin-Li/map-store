#!/usr/bin/env bash

set -o errexit

# 1. Generate the datasets by `cargo run --release --bin data -- MODE -o PATH --size SIZE_IN_MB`

cargo build --release

# 2. Define `src` to PATH to the generated data such as belows.

prefix=/media/lhq/Data/Workspace/datasets

src=$prefix/normal-100M.txt
# src=$prefix/normal-1G.txt
# src=$prefix/normal-10G.txt

# src=$prefix/dist-100M.txt
# src=$prefix/dist-1G.txt
# src=$prefix/dist-10G.txt

# src=$prefix/ident-100M.txt
# src=$prefix/ident-1G.txt
# src=$prefix/ident-10G.txt

src_sz=$(du $src -k | cut -f1)
# max_mem=$(($src_sz / 100)) 
max_mem=$(($src_sz)) 

echo "memory limit: $max_mem KB"

perl scripts/timeout -c target/release/bruteforce $src -k 10
perl scripts/timeout -c -m $max_mem target/release/mapreduce $src -k 10 --nmaps 500
perl scripts/timeout -c -m $max_mem target/release/mapstore $src -k 10 --nmaps 500

# These are used for profiling and optimization.
# flamegraph target/release/mapreduce $src -k 10 --nmaps 500
# flamegraph target/release/mapstore $src -k 10 --nmaps 500

echo "finish"

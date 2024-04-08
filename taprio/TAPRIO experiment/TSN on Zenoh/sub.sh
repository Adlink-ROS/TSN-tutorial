#!/usr/bin/env bash
set -e
script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$script_dir"

source config.sh

dt="$(date '+%Y-%m-%d-%H-%M-%S')"
out_dir="result/$dt"

mkdir -p result
mkdir "$out_dir"


export PORT=$((port_base+1))
config_file="sub.$PORT.json5"
envsubst < sub.json5.in > "$config_file"

../target/fast/examples/z_sub_thr_p -c "$config_file" --no-stdin -p "1 2 3" --file-path "${out_dir}/prio-" | tee "${out_dir}/prio-${prio}.payload-${payload}.txt"

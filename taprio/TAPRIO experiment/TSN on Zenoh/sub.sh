#!/usr/bin/env bash
set -e
script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$script_dir"

source config.sh

dt="$(date '+%Y-%m-%d-%H-%M-%S')"
out_dir="result/$dt"

mkdir -p result
mkdir "$out_dir"

for prio in ${priorities[@]}; do
    export PORT=$((port_base+1))
    config_file="sub.$PORT.json5"
    envsubst < sub.json5.in > "$config_file"
    #echo "../target/fast/examples/z_sub_thr -c \"$config_file\" --no-stdin | tee \"${out_dir}/prio-${prio}.payload-${payload}.txt\""
    #echo "../target/release/examples/z_sub_thr -c \"$config_file\" --no-stdin | tee \"${out_dir}/prio-${prio}.payload-${payload}.txt\""
    #echo "../target/fast/examples/z_sub_thr_p -c \"$config_file\" --no-stdin --priority ${prio} | tee \"${out_dir}/prio-${prio}.payload-${payload}.txt\""
    #echo "../target/fast/examples/z_sub_thr_p -c \"$config_file\" --no-stdin -p \"1 2 3\" | tee \"${out_dir}/prio-${prio}.payload-${payload}.txt\""
    
done | parallel --lb -j0

export PORT=$((port_base+1))
config_file="sub.$PORT.json5"
envsubst < sub.json5.in > "$config_file"
#../target/fast/examples/z_sub_thr_p -c "$config_file" --no-stdin -p "1 2 3" | tee "${out_dir}/prio-${prio}.payload-${payload}.txt"
#../target/fast/examples/z_sub_thr_p -c "$config_file" --no-stdin -p "1 2" | tee "${out_dir}/prio-${prio}.payload-${payload}.txt"
#../target/fast/examples/z_sub_thr_p -c "$config_file" --no-stdin -p "1 2 3" --file-path "${out_dir}/prio-" | tee "${out_dir}/prio-${prio}.payload-${payload}.txt"
../target/fast/examples/z_sub_thr_p -c "$config_file" --no-stdin -p "1 2 3" --file-path "${out_dir}/prio-" | tee "${out_dir}/prio-${prio}.payload-${payload}.txt"

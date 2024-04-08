#!/usr/bin/env bash
set -e
script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$script_dir"

source config.sh

# payload="$1"
# shift || {
#     echo "Usage: $0 PAYLOAD_SIZE PRIORITY..." >&2
#     exit 1
# }

# for prio in ${priorities[@]}; do
#     export PORT=$((port_base+prio))
#     config_file="pub.$PORT.json5"
#     envsubst < pub.json5.in > "$config_file"
#     #echo ../target/fast/examples/z_pub_thr "$payload" -c "$config_file" -p "$prio"
#     echo ../target/release/examples/z_pub_thr "$payload" -c "$config_file" -p "$prio"
#     #echo "Payload size is: $payload"
# done | parallel --lb -j0

config_file="pub.7001.json5"
#../target/release/examples/z_pub_thr_s "$payload" -c "$config_file" -p "1 2 3"
#../target/fast/examples/z_pub_thr_s "$payload" -c "$config_file" -p "1 2 3"
../target/fast/examples/z_pub_thr_s "$payload" -c "$config_file" -p "1"
#../target/fast/examples/z_pub_thr_s "$payload" -c "$config_file" -p "2"
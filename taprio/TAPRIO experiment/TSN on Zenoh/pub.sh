#!/usr/bin/env bash
set -e
script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$script_dir"

source config.sh

config_file="pub.7001.json5"
../target/fast/examples/z_pub_thr_s "$payload" -c "$config_file" -p "1 2 3"
#! /usr/bin/env bash

function compile_and_run {
    cargo run --quiet -- "$1"
    command pushd ./primal-runner > /dev/null
    cargo run --quiet
    command popd > /dev/null
}

if [ $# -eq 0 ]; then
    echo "Could not find an input .roq file. Exiting." && return 1;
else
    compile_and_run $1
fi


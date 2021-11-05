#!/usr/bin/env bash

if [ -z "$1" ]
  then
    echo "Please specify a file to read found layouts from!"
    exit 1
fi


xargs -a "$1" env RUST_LOG=info cargo run --release --bin evaluate

#! /bin/bash

# check cargo
if ! command -v cargo &> /dev/null
then
    echo "<cargo> could not be found"
    exit
fi

# build app
cargo build --release

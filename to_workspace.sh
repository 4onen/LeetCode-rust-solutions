#!/bin/bash

set -e

for FILE in src/*.rs; do
    FILE_STEM=$(basename "$FILE" .rs)
    PROJ_FOLDER="challenges/${FILE_STEM}"
    mkdir "$PROJ_FOLDER"
    mv src/${FILE_STEM}* "$PROJ_FOLDER"
    cat << EOF > "$PROJ_FOLDER/Cargo.toml"
[package]
name = "${FILE_STEM}"
version = "0.1.0"
edition = "2021"

[lib]
path="${FILE_STEM}.rs"

[dependencies]
EOF
done
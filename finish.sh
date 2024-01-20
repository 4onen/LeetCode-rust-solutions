#!/bin/bash

set -e

FILES=( $(git status --porcelain | grep -P ' src/(?!lib).*\.rs' | cut -d'/' -f2 | tr '\n' ' ') );

if [ -z "${FILES[@]}" ]; then
    echo "No files detected"
    exit 1
fi

echo "Detected files: ${FILES[@]}"
echo "Testing..."

for file in ${FILES[@]}; do
    if ! cargo test ${file%.rs}; then
        echo "Test failed for $file"
        exit 1
    fi
done

git add src/*
git commit -m "Completed $(echo ${FILES[@]%.rs} | tr ' ' ', ')"
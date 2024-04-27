#!/bin/bash

set -e

FILES=( $(git status --porcelain | grep -P ' challenges/' | cut -d'/' -f2 | tr '\n' ' ') );

if [ -z "${FILES[@]}" ]; then
    echo "No files detected"
    exit 1
fi

echo "Detected files: ${FILES[@]}"
echo "Testing..."

for file in ${FILES[@]}; do
    if ! cargo test --package ${file}; then
        echo "Test failed for $file"
        exit 1
    fi
done

git add challenges
git commit -m "Add solution for $(echo ${FILES[@]%.rs} | tr ' ' ', ')"
git push || true

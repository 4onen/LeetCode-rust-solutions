#!/bin/bash

set -e

#git pull || true

DONT_REALLY_OPEN=1 . open.sh

if [ -e "challenges/${project_name}" ]; then
    echo "Project ${project_name} already exists"
    open_project "${project_name}"
    exit 1
else
    echo "Project ${project_name} does not exist"
fi

echo "Creating new project: ${project_name}"
mkdir -p "challenges/${project_name}"
# Create the file with the leetcode template
cat > "challenges/${project_name}/solution.rs" <<EOF
// $url

pub struct Solution;

impl Solution {

}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(_, expected: _) {
        assert_eq!(Solution::_, expected);
    }

    #[test]
    fn ex1() {
        test(
        )
    }
}
EOF
cat > "challenges/${project_name}/Cargo.toml" <<EOF
[package]
name = "${project_name}"
version = "0.1.0"
edition = "2021"

[lib]
doctest = false
path="solution.rs"

[dependencies]
EOF

open_project "${project_name}"

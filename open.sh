#!/bin/bash

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <url or project_name>"
    exit 1
fi

# If the project name is a url, extract the project name
if [ "$1" != "${1##http}" ]; then
    # Extract the project name from the url
    # https://leetcode.com/problems/two-sum/ -> two_sum
    project_name=$(echo "$1" | sed -Ee 's;^https?://leetcode.com/problems/([a-z0-9-]+)/?.*$;\1;' -e 's;-;_;g')
    url=$1
else
    project_name=$1
    url=$2
fi

# Remove query parameters from the url
url=${url%%\?*}

open_project () {
    if [ "$TERM_PROGRAM" = "vscode" ]; then
        # If we're in vscode, open the file there
        (code "challenges/$1" "challenges/$1/$1.rs")
    else
        # Otherwise, open in $EDITOR
        (cd "challenges/$1" && $EDITOR "$1.rs")
    fi
}
export -f open_project


if [ -z "$DONT_REALLY_OPEN" ]; then
    if [ -e "challenges/${project_name}" ]; then
        echo "Project ${project_name} already exists"
        open_project "${project_name}"
    else
        echo "Project ${project_name} does not exist"
        exit 1
    fi
fi

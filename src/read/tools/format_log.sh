#!/bin/bash
set -e  # Exit immediately on any error

[ ! -d "./logs/$3" ] && mkdir -p "./logs/$3"

if jq -s . "$1" > "./logs/$3/$2"; then
    rm "$1"
    exit 0  # Success
else
    echo "jq failed to format file: $1" >&2
    exit 1  # Failure
fi
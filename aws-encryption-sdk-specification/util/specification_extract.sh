#!/usr/bin/env bash

SCRIPT_SOURCE=$(dirname "$0")

FILES=("$SCRIPT_SOURCE"/../framework/**/*.md)
FILES+=("$SCRIPT_SOURCE"/../framework/*.md)
FILES+=("$SCRIPT_SOURCE"/../client-apis/*.md)
FILES+=("$SCRIPT_SOURCE"/../data-format/*.md)

echo "Extracting ${#FILES[@]} files:"

for FILE in "${FILES[@]}"
do
    RESOLVED="$(cd "$(dirname "$FILE")"; pwd -P)/$(basename "$FILE")"
    echo "Extracting: $RESOLVED"
    "$SCRIPT_SOURCE"/extract.js "$FILE"
done

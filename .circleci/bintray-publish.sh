#!/bin/bash
set -eu

$file=$1
$name=$(basename $file)

curl -f -u $BINTRAY_USERNAME:$BINTRAY_PASSWORD -T "$file" "https://api.bintray.com/content/palantir/releases/conjure-rust/$CIRCLE_TAG/com/palantir/conjure/rust/conjure-rust/$CIRCLE_TAG/$name?publish=1"

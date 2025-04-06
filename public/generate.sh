#! /usr/bin/env bash
set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$SCRIPT_DIR"

list=""

for dir in ../games/*; do
    name=$(basename "$dir")
    dir=$(realpath "$dir")
    echo "Generating $name from $dir"
    cd "$dir"
    cargo build --release --target wasm32-unknown-unknown
    list+="        <li><a href=\"$name.html\">$name</a></li>\n"
    cd "$SCRIPT_DIR"
    cp "../target/wasm32-unknown-unknown/release/$name.wasm" "$name.wasm"
    cp game.html.template "$name.html"
    sed -i "s|GAME_NAME|$name|" "$name.html"
done

echo "Generating index.html"
cp index.html.template index.html
sed -i "s|LIST_ITEMS|$list|" index.html

#! /usr/bin/env bash
set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
SITE_DIR=$(realpath "$SCRIPT_DIR/../_site")
cd "$SCRIPT_DIR"

list=""
mkdir -p "$SITE_DIR"
for dir in ../games/*; do
    name=$(basename "$dir")
    dir=$(realpath "$dir")
    echo "Generating $name from $dir"
    cd "$dir"
    cargo build --release --target wasm32-unknown-unknown
    list+="        <li><a href=\"$name.html\">$name</a></li>\n"
    cd "$SITE_DIR"
    cp "../target/wasm32-unknown-unknown/release/$name.wasm" "$name.wasm"
    cp "$SCRIPT_DIR/game.html.template" "$name.html"
    sed -i "s|GAME_NAME|$name|" "$name.html"
done

echo "Generating index.html"
cp "$SCRIPT_DIR/index.html.template" index.html
sed -i "s|LIST_ITEMS|$list|" index.html
cp "$SCRIPT_DIR/style.css" style.css

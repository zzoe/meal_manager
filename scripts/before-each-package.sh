#!/usr/bin/env bash
# Wrapper script that calls robius-packaging-commands and then copies
# font resources that robius-packaging-commands doesn't handle automatically.
#
# robius-packaging-commands only copies makepad_widgets resources because
# only makepad-widgets has a build.rs that generates a .path file.
# Font crates (makepad-fonts-*) don't have build.rs, so their resources
# must be copied manually.

set -e

# Run the original robius-packaging-commands
robius-packaging-commands before-each-package \
    --binary-name meal_manager \
    --path-to-binary ./target/release/meal_manager

# Now copy font resources that were missed
# Find the makepad checkout directory
MAKEPAD_DIR=$(find ~/.cargo/git/checkouts/makepad-* -maxdepth 2 -type d -name "widgets" 2>/dev/null | head -1 | xargs dirname 2>/dev/null || true)

echo "DEBUG: MAKEPAD_DIR=$MAKEPAD_DIR"
echo "DEBUG: CARGO_PACKAGER_FORMAT=$CARGO_PACKAGER_FORMAT"
echo "DEBUG: cwd=$(pwd)"
echo "DEBUG: dist/resources contents:"
ls -la ./dist/resources/ 2>/dev/null || echo "  (not found)"

if [ -n "$MAKEPAD_DIR" ] && [ -d "$MAKEPAD_DIR" ]; then
    echo "DEBUG: Found makepad at $MAKEPAD_DIR"
    RESOURCES_DIR="./dist/resources"
    mkdir -p "$RESOURCES_DIR"

    for font_pkg in chinese_bold chinese_bold_2 chinese_regular chinese_regular_2 emoji; do
        src="$MAKEPAD_DIR/widgets/fonts/$font_pkg/resources"
        dest="$RESOURCES_DIR/makepad_fonts_$font_pkg/resources"
        echo "DEBUG: Checking $src -> $dest"
        if [ -d "$src" ]; then
            echo "Copying $font_pkg resources from $src to $dest"
            mkdir -p "$dest"
            cp -rv "$src"/* "$dest"/
        else
            echo "WARNING: Source directory $src does not exist!"
        fi
    done
    echo "Font resources copied successfully."
    echo "DEBUG: dist/resources contents after copy:"
    find ./dist/resources/ -type f 2>/dev/null | head -30
else
    echo "WARNING: Could not find makepad checkout directory, font resources not copied."
fi

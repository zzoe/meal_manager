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

if [ -n "$MAKEPAD_DIR" ] && [ -d "$MAKEPAD_DIR" ]; then
    RESOURCES_DIR="./dist/resources"
    mkdir -p "$RESOURCES_DIR"

    for font_pkg in fonts_chinese_bold fonts_chinese_bold_2 fonts_chinese_regular fonts_chinese_regular_2 fonts_emoji; do
        src="$MAKEPAD_DIR/widgets/$font_pkg/resources"
        dest="$RESOURCES_DIR/makepad_$font_pkg/resources"
        if [ -d "$src" ]; then
            echo "Copying $font_pkg resources from $src to $dest"
            mkdir -p "$dest"
            cp -r "$src"/* "$dest"/
        fi
    done
    echo "Font resources copied successfully."
else
    echo "Warning: Could not find makepad checkout directory, font resources not copied."
fi

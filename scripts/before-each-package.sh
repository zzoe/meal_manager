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

# Strip the ad-hoc signature from the binary that rustc added during compilation.
# If we don't do this, cargo-packager will embed the binary with its original signature
# into the .app bundle, but that signature doesn't account for the Resources directory
# and other bundle contents. This causes Gatekeeper to reject the app as "damaged".
# By stripping the signature, cargo-packager can apply a fresh signature to the
# entire .app bundle that correctly covers all resources.
if command -v codesign &>/dev/null; then
    codesign --remove-signature ./target/release/meal_manager 2>/dev/null || true
    echo "Stripped signature from binary."
fi

# Now copy font resources that were missed
# Find the makepad checkout directory
MAKEPAD_DIR=$(find ~/.cargo/git/checkouts/makepad-* -maxdepth 2 -type d -name "widgets" 2>/dev/null | head -1 | xargs dirname 2>/dev/null || true)

if [ -n "$MAKEPAD_DIR" ] && [ -d "$MAKEPAD_DIR" ]; then
    RESOURCES_DIR="./dist/resources"
    mkdir -p "$RESOURCES_DIR"

    for font_pkg in chinese_bold chinese_bold_2 chinese_regular chinese_regular_2 emoji; do
        src="$MAKEPAD_DIR/widgets/fonts/$font_pkg/resources"
        dest="$RESOURCES_DIR/makepad_fonts_$font_pkg/resources"
        if [ -d "$src" ]; then
            echo "Copying $font_pkg resources to $dest"
            mkdir -p "$dest"
            cp -r "$src"/* "$dest"/
        else
            echo "WARNING: Source directory $src does not exist!"
        fi
    done
    echo "Font resources copied successfully."
else
    echo "WARNING: Could not find makepad checkout directory, font resources not copied."
fi

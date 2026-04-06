#!/usr/bin/bash
# Branched from vikdevelop/penpot-electron
echo -e '\033[1m Project: penpot-tauri \033[0m'
echo "- Downloading Manifest"
cd /tmp
wget -c https://raw.githubusercontent.com/oliben67/penpot-tauri/main/manifest.yaml > /dev/null 2>&1
mkdir flatpak
echo "- Building and installing penpot-tauri with flatpak builder"
echo "--- it's gonna take a while"
flatpak-builder flatpak manifest.yaml --install --user > /dev/null 2>&1
echo "- Cleaning build files"
rm -rf /tmp/flatpak
rm -rf /tmp/.flatpak-builder
rm /tmp/manifest.yaml
echo -e '\033[1m Done! penpot-tauri was built and installed successfully!  \033[0m'

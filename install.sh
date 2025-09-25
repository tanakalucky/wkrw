#!/usr/bin/env bash
set -euo pipefail

REPO="tanakalucky/wkrw"

VERSION=$(curl -s https://api.github.com/repos/$REPO/releases/latest \
  | grep tag_name \
  | cut -d '"' -f4)

OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
  Linux*)   TARGET="x86_64-unknown-linux-gnu" ;;
  Darwin*)
    if [ "$ARCH" = "arm64" ]; then
      TARGET="aarch64-apple-darwin"
    else
      TARGET="x86_64-apple-darwin"
    fi
    ;;
  *)
    echo "❌ Unsupported OS: $OS $ARCH"
    exit 1
    ;;
esac

FILENAME="wkrw-${VERSION}-${TARGET}.tar.gz"
URL="https://github.com/$REPO/releases/download/${VERSION}/${FILENAME}"

echo "Downloading $URL ..."
curl -L "$URL" -o "$FILENAME"

echo "Extracting..."
tar -xzf "$FILENAME"

echo "Cleaning up..."
rm -f "$FILENAME"

echo "Installing to /usr/local/bin (requires sudo)..."
chmod +x wkrw
sudo mv wkrw /usr/local/bin/

echo "Installed successfully!"
echo "Try running: wkrw"

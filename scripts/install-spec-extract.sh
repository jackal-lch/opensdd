#!/bin/bash
#
# Install spec-extract binary for the current platform
# Usage: ./install-spec-extract.sh [version]
#
set -e

REPO="jackal-lch/opensdd"
VERSION="${1:-latest}"
INSTALL_DIR="${HOME}/.local/bin"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin)
    case "$ARCH" in
      arm64) PLATFORM="darwin-arm64" ;;
      x86_64) PLATFORM="darwin-x64" ;;
      *) error "Unsupported macOS architecture: $ARCH" ;;
    esac
    ;;
  Linux)
    case "$ARCH" in
      x86_64) PLATFORM="linux-x64" ;;
      *) error "Unsupported Linux architecture: $ARCH" ;;
    esac
    ;;
  MINGW*|MSYS*|CYGWIN*)
    PLATFORM="windows-x64"
    INSTALL_DIR="${HOME}/bin"
    ;;
  *)
    error "Unsupported operating system: $OS"
    ;;
esac

info "Detected platform: $PLATFORM"

# Construct download URL
if [ "$VERSION" = "latest" ]; then
  DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/spec-extract-${PLATFORM}.tar.gz"
else
  DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/spec-extract-${PLATFORM}.tar.gz"
fi

# Windows uses .zip
if [[ "$PLATFORM" == "windows-x64" ]]; then
  DOWNLOAD_URL="${DOWNLOAD_URL%.tar.gz}.zip"
fi

info "Downloading from: $DOWNLOAD_URL"

# Create install directory
mkdir -p "$INSTALL_DIR"

# Download and extract
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

if [[ "$PLATFORM" == "windows-x64" ]]; then
  curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/spec-extract.zip"
  unzip -q "$TMP_DIR/spec-extract.zip" -d "$TMP_DIR"
  mv "$TMP_DIR/spec-extract.exe" "$INSTALL_DIR/"
else
  curl -fsSL "$DOWNLOAD_URL" | tar xz -C "$TMP_DIR"
  mv "$TMP_DIR/spec-extract" "$INSTALL_DIR/"
  chmod +x "$INSTALL_DIR/spec-extract"
fi

info "Installed spec-extract to $INSTALL_DIR/spec-extract"

# Verify installation
if command -v spec-extract &> /dev/null; then
  info "spec-extract is ready to use!"
  spec-extract --version
else
  warn "spec-extract installed but not in PATH"
  echo ""
  echo "Add this to your shell profile (.bashrc, .zshrc, etc.):"
  echo ""
  echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
  echo ""
  echo "Then restart your shell or run: source ~/.bashrc"
fi

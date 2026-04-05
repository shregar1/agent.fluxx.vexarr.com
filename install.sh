#!/bin/bash
#
# Fluxx Agent: One-line Installation Script
# This script intelligently detects your platform and installs the Fluxx binary.
# Usage: curl -fsSL https://fluxx.dev/install.sh | bash
#
set -e

# --- Configuration ---
BINARY_NAME="fluxx"
INSTALL_DIR="/usr/local/bin"
VERSION="0.1.0"
REPO="fluxx/agent"

# --- Colors for Professional UI ---
BLUE='\033[0;34m'
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# --- Platform Detection ---
detect_platform() {
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)

    case "$OS" in
        linux*)   PLATFORM="linux" ;;
        darwin*)  PLATFORM="macos" ;;
        msys*|cygwin*|mingw*) PLATFORM="windows" ;;
        *)        echo -e "${RED}Error: Unsupported OS '$OS'${NC}"; exit 1 ;;
    esac

    case "$ARCH" in
        x86_64|amd64) ARCH="x86_64" ;;
        arm64|aarch64) ARCH="aarch64" ;;
        *)             echo -e "${RED}Error: Unsupported architecture '$ARCH'${NC}"; exit 1 ;;
    esac

    echo "${PLATFORM}-${ARCH}"
}

# --- Installation Logic ---
main() {
    echo -e "${BLUE}    ╭─────────────────────────────────────────╮"
    echo -e "    │                                         │"
    echo -e "    │   ⚡ ${CYAN}FLUXX AGENT INSTALLER${BLUE}             │"
    echo -e "    │   High-Performance Edge Observability  │"
    echo -e "    │                                         │"
    echo -e "    ╰─────────────────────────────────────────╯${NC}"
    echo ""

    PLATFORM_ARCH=$(detect_platform)
    echo -e "${CYAN}→${NC} Target: ${BLUE}$PLATFORM_ARCH${NC}"

    # For this local demo/dev environment, we assume the binary is available in target/dist
    # In production, this would be: DOWNLOAD_URL="https://github.com/$REPO/releases/download/v$VERSION/fluxx-$PLATFORM_ARCH"
    
    # Check if we are in the source directory (development mode)
    if [ -f "target/dist/fluxx" ]; then
        echo -e "${CYAN}→${NC} Using locally built distribution binary..."
        BIN_PATH="target/dist/fluxx"
    else
        # Fallback to building from source if prebuilt is missing and we have cargo
        if command -v cargo >/dev/null 2>&1; then
            echo -e "${YELLOW}→${NC} Prebuilt binary not found. Building from source...${NC}"
            cargo build --profile dist
            BIN_PATH="target/dist/fluxx"
        else
            echo -e "${RED}Error: Prebuilt binary not available for $PLATFORM_ARCH and Rust/Cargo is not installed.${NC}"
            echo "Please install Rust to build from source: https://rustup.rs/"
            exit 1
        fi
    fi

    # Install binary
    echo -e "${CYAN}→${NC} Deploying to $INSTALL_DIR/$BINARY_NAME..."
    if [ -w "$INSTALL_DIR" ]; then
        cp "$BIN_PATH" "$INSTALL_DIR/$BINARY_NAME"
    else
        sudo cp "$BIN_PATH" "$INSTALL_DIR/$BINARY_NAME"
    fi
    sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"

    # Setup Config Directory
    mkdir -p "$HOME/.fluxx"
    if [ ! -f "$HOME/.fluxx/config.json" ] && [ -f "config/default.json" ]; then
        cp config/default.json "$HOME/.fluxx/config.json"
    fi

    echo ""
    echo -e "${GREEN}✓ Fluxx Agent installed successfully!${NC}"
    echo -e "  Location: $(which fluxx)"
    echo -e "  Version: $(fluxx --version)"
    echo ""
    echo -e "${BLUE}To start monitoring, run:${NC}"
    echo -e "  ${YELLOW}fluxx start${NC}"
    echo ""
}

main "$@"

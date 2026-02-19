#!/usr/bin/env bash

set -e

# RankPath MCP Server Installer
# https://github.com/rankpath-io/rankpath-mcp-server

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Detect OS
OS="$(uname -s)"
case "$OS" in
    Linux*)     PLATFORM=linux;;
    Darwin*)    PLATFORM=macos;;
    *)          PLATFORM=unknown;;
esac

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}  RankPath MCP Server Installer${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"

# Check for git
if ! command_exists git; then
    echo -e "${RED}✗ git is not installed${NC}"
    echo ""
    echo "Please install git:"
    case "$PLATFORM" in
        macos)
            echo "  brew install git"
            echo "  or download from https://git-scm.com/downloads"
            ;;
        linux)
            echo "  sudo apt-get install git    # Debian/Ubuntu"
            echo "  sudo yum install git        # CentOS/RHEL"
            echo "  sudo dnf install git        # Fedora"
            ;;
        *)
            echo "  Visit https://git-scm.com/downloads"
            ;;
    esac
    exit 1
fi
echo -e "${GREEN}✓ git found${NC}"

# Check for cargo
if ! command_exists cargo; then
    echo -e "${RED}✗ cargo (Rust) is not installed${NC}"
    echo ""
    echo "Please install Rust:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    echo "Or visit: https://rustup.rs"
    exit 1
fi
echo -e "${GREEN}✓ cargo found ($(cargo --version))${NC}"

# Check for rustc
if ! command_exists rustc; then
    echo -e "${RED}✗ rustc is not installed${NC}"
    echo ""
    echo "Please install Rust:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
echo -e "${GREEN}✓ rustc found ($(rustc --version))${NC}"

echo ""

# Create temporary directory for installation
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

echo -e "${YELLOW}Cloning repository...${NC}"
cd "$TEMP_DIR"
git clone https://github.com/rankpath-io/rankpath-mcp-server.git
cd rankpath-mcp-server

echo ""
echo -e "${YELLOW}Building with cargo (this may take a few minutes)...${NC}"
cargo build --release

echo ""
echo -e "${YELLOW}Installing binary...${NC}"

# Determine installation directory
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
elif [ -d "$HOME/.local/bin" ]; then
    INSTALL_DIR="$HOME/.local/bin"
else
    mkdir -p "$HOME/.local/bin"
    INSTALL_DIR="$HOME/.local/bin"
fi

# Copy binary
BINARY_NAME="rankpath-mcp-server"
if [ -f "target/release/$BINARY_NAME" ]; then
    cp "target/release/$BINARY_NAME" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
    echo -e "${GREEN}✓ Installed to $INSTALL_DIR/$BINARY_NAME${NC}"
else
    echo -e "${RED}✗ Build failed - binary not found${NC}"
    exit 1
fi

echo ""

# Check if install dir is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}⚠ $INSTALL_DIR is not in your PATH${NC}"
    echo ""
    echo "Add this to your shell configuration file (~/.bashrc, ~/.zshrc, etc.):"
    echo ""
    echo -e "${BLUE}  export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
    echo ""
fi

echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}  Installation complete!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Verify installation:"
echo -e "  ${BLUE}$BINARY_NAME --version${NC}"
echo ""
echo "Next steps:"
echo "  1. Create a RankPath API key at https://rankpath.io/settings"
echo "  2. Configure your AI assistant to use the MCP server"
echo "  3. Visit https://rankpath.io/articles/rankpath-mcp-server for full documentation"
echo ""


#!/bin/bash
#
# Fluxx Agent - Automated Installation Script
# This script installs Rust, builds the project, and runs tests
#

set -e  # Exit on any error

echo "╔══════════════════════════════════════════════════════════════════════════════╗"
echo "║           FLUXX AGENT - AUTOMATED INSTALLER                                  ║"
echo "╚══════════════════════════════════════════════════════════════════════════════╝"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check OS
print_status "Detecting operating system..."
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
    print_success "Detected Linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="mac"
    print_success "Detected macOS"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    print_error "Windows detected. Please use WSL2 (Windows Subsystem for Linux)"
    exit 1
else
    print_warning "Unknown OS: $OSTYPE. Continuing anyway..."
    OS="unknown"
fi

# Install dependencies based on OS
print_status "Installing system dependencies..."
if [ "$OS" == "linux" ]; then
    if command -v apt-get &> /dev/null; then
        sudo apt-get update -qq
        sudo apt-get install -y -qq build-essential git cmake pkg-config libssl-dev
    elif command -v yum &> /dev/null; then
        sudo yum install -y gcc git cmake openssl-devel
    elif command -v pacman &> /dev/null; then
        sudo pacman -S --needed base-devel git cmake openssl
    fi
elif [ "$OS" == "mac" ]; then
    if ! command -v brew &> /dev/null; then
        print_status "Installing Homebrew..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi
    brew install cmake openssl@3 2>/dev/null || true
fi
print_success "System dependencies installed"

# Check if Rust is installed
print_status "Checking for Rust installation..."
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    print_success "Rust already installed: $RUST_VERSION"
else
    print_status "Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    print_success "Rust installed: $(rustc --version)"
fi

# Ensure cargo is in PATH
source $HOME/.cargo/env

# Verify Rust installation
if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found in PATH. Please run: source $HOME/.cargo/env"
    exit 1
fi

print_success "Rust and Cargo are ready"

# Get project directory
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_DIR"

print_status "Project directory: $PROJECT_DIR"

# Build the project
print_status "Building fluxx-agent (this may take 2-5 minutes)..."
echo ""
cargo build --release 2>&1 | tee build.log | while read line; do
    echo "  $line"
done

if [ ${PIPESTATUS[0]} -eq 0 ]; then
    print_success "Build completed successfully!"
else
    print_error "Build failed. Check build.log for details"
    exit 1
fi

# Run tests
print_status "Running unit tests..."
if cargo test --release 2>&1 | tee test.log; then
    print_success "All tests passed!"
else
    print_warning "Some tests failed. Check test.log for details"
fi

echo ""
echo "╔══════════════════════════════════════════════════════════════════════════════╗"
echo "║                    INSTALLATION COMPLETE!                                    ║"
echo "╚══════════════════════════════════════════════════════════════════════════════╝"
echo ""

# Run the ultra-optimized load test
print_status "Running performance benchmark..."
echo ""
./target/release/examples/ultra_optimized_load_test

echo ""
echo "═══════════════════════════════════════════════════════════════════════════════"
echo ""
echo "🎉 Setup complete! Here are some commands to try:"
echo ""
echo "  Run load test:        cargo run --release --example ultra_optimized_load_test"
echo "  Run all tests:        cargo test --release"
echo "  Run the agent:        cargo run --release"
echo "  Check code:           cargo check"
echo "  Format code:          cargo fmt"
echo ""
echo "  Read the guide:       cat BEGINNERS_GUIDE.md"
echo ""
echo "═══════════════════════════════════════════════════════════════════════════════"

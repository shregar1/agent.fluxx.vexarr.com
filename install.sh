#!/bin/bash
#
# Fluxx Agent — One-line Install Script
# Usage: curl -fsSL https://api.fluxx.vexarr.com/install.sh | bash
#
# Options (via env vars):
#   FLUXX_VERSION=0.1.0          — pin to a specific version
#   FLUXX_INSTALL_DIR=~/.local/bin — override install directory
#   FLUXX_API_KEY=flx_live_...   — pre-configure API key
#   FLUXX_API_URL=https://...    — override API base (default: https://api.fluxx.vexarr.com)
#
set -euo pipefail

# ─── Configuration ──────────────────────────────────────────────────────────────
BINARY_NAME="fluxx"
DEFAULT_INSTALL_DIR="/usr/local/bin"
USER_INSTALL_DIR="$HOME/.fluxx/bin"
CONFIG_DIR="$HOME/.fluxx"
VERSION="${FLUXX_VERSION:-latest}"
API_URL="${FLUXX_API_URL:-https://api.fluxx.vexarr.com}"

# ─── Colors ─────────────────────────────────────────────────────────────────────
if [ -t 1 ]; then
  BOLD='\033[1m'
  DIM='\033[2m'
  BLUE='\033[0;34m'
  CYAN='\033[0;36m'
  GREEN='\033[0;32m'
  YELLOW='\033[1;33m'
  RED='\033[0;31m'
  NC='\033[0m'
else
  BOLD='' DIM='' BLUE='' CYAN='' GREEN='' YELLOW='' RED='' NC=''
fi

# ─── Helpers ────────────────────────────────────────────────────────────────────
info()  { echo -e "${CYAN}→${NC} $*"; }
ok()    { echo -e "${GREEN}✓${NC} $*"; }
warn()  { echo -e "${YELLOW}!${NC} $*"; }
err()   { echo -e "${RED}✗${NC} $*" >&2; }
die()   { err "$*"; exit 1; }

need_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    die "Required command '$1' not found. Please install it and retry."
  fi
}

# ─── Platform Detection ─────────────────────────────────────────────────────────
detect_os() {
  local os
  os="$(uname -s)"
  case "$os" in
    Linux*)  echo "linux" ;;
    Darwin*) echo "darwin" ;;
    MINGW*|MSYS*|CYGWIN*) echo "windows" ;;
    *) die "Unsupported operating system: $os" ;;
  esac
}

detect_arch() {
  local arch
  arch="$(uname -m)"
  case "$arch" in
    x86_64|amd64)  echo "amd64" ;;
    arm64|aarch64) echo "arm64" ;;
    armv7l)        echo "armv7" ;;
    *) die "Unsupported architecture: $arch" ;;
  esac
}

# ─── Version Resolution ─────────────────────────────────────────────────────────
resolve_version() {
  if [ "$VERSION" = "latest" ]; then
    info "Resolving latest version..."
    local resp
    resp=$(curl -fsSL "${API_URL}/api/v1/releases/latest" 2>/dev/null) || \
      die "Failed to reach Fluxx API. Check your connection or set FLUXX_VERSION manually."
    VERSION=$(echo "$resp" | grep -o '"version":"[^"]*"' | head -1 | cut -d'"' -f4)
    if [ -z "$VERSION" ]; then
      die "Could not determine latest version from API response."
    fi
  fi
  echo "$VERSION"
}

# ─── Get Signed Download URL from Backend ───────────────────────────────────────
get_signed_url() {
  local os="$1" arch="$2" version="$3"

  info "Requesting download URL from Fluxx API..."
  local resp
  resp=$(curl -fsSL "${API_URL}/api/v1/releases/download?os=${os}&arch=${arch}&version=${version}" 2>/dev/null) || \
    die "Failed to get download URL from API. The server may be unreachable."

  # Check for API error
  if echo "$resp" | grep -q '"responseStatus":"ERROR"'; then
    local msg
    msg=$(echo "$resp" | grep -o '"responseMessage":"[^"]*"' | cut -d'"' -f4)
    die "API error: ${msg:-Unknown error}"
  fi

  # Extract signed URL
  local url
  url=$(echo "$resp" | grep -o '"url":"[^"]*"' | head -1 | cut -d'"' -f4)
  if [ -z "$url" ]; then
    die "No download URL in API response. Version v${version} may not exist for ${os}/${arch}."
  fi

  echo "$url"
}

# ─── Get Checksum URL ───────────────────────────────────────────────────────────
get_checksum_url() {
  local version="$1"
  local resp
  resp=$(curl -fsSL "${API_URL}/api/v1/releases/checksums?version=${version}" 2>/dev/null) || return 1
  echo "$resp" | grep -o '"url":"[^"]*"' | head -1 | cut -d'"' -f4
}

# ─── Download Binary ────────────────────────────────────────────────────────────
download_binary() {
  local signed_url="$1"
  local tmpdir
  tmpdir="$(mktemp -d)"
  local dest="${tmpdir}/${BINARY_NAME}"

  info "Downloading binary..."
  if command -v curl >/dev/null 2>&1; then
    curl -fSL --progress-bar "$signed_url" -o "$dest" || \
      die "Download failed. The signed URL may have expired. Retry the install."
  elif command -v wget >/dev/null 2>&1; then
    wget -q --show-progress "$signed_url" -O "$dest" || \
      die "Download failed. Retry the install."
  else
    die "Neither 'curl' nor 'wget' found. Install one and retry."
  fi

  echo "$dest"
}

# ─── Checksum Verification ──────────────────────────────────────────────────────
verify_checksum() {
  local binary_path="$1" os="$2" arch="$3" version="$4"

  info "Verifying checksum..."
  local checksums_url
  checksums_url="$(get_checksum_url "$version")" || {
    warn "Checksums not available — skipping verification."
    return 0
  }

  if [ -z "$checksums_url" ]; then
    warn "No checksum URL — skipping."
    return 0
  fi

  local checksums
  checksums="$(curl -fsSL "$checksums_url" 2>/dev/null)" || {
    warn "Could not download checksums — skipping."
    return 0
  }

  local ext=""
  [ "$os" = "windows" ] && ext=".exe"
  local expected_name="fluxx-${os}-${arch}${ext}"
  local expected_hash
  expected_hash="$(echo "$checksums" | grep "$expected_name" | awk '{print $1}')"

  if [ -z "$expected_hash" ]; then
    warn "No checksum entry for ${expected_name} — skipping."
    return 0
  fi

  local actual_hash
  if command -v sha256sum >/dev/null 2>&1; then
    actual_hash="$(sha256sum "$binary_path" | awk '{print $1}')"
  elif command -v shasum >/dev/null 2>&1; then
    actual_hash="$(shasum -a 256 "$binary_path" | awk '{print $1}')"
  else
    warn "No sha256 tool found — skipping verification."
    return 0
  fi

  if [ "$actual_hash" != "$expected_hash" ]; then
    die "Checksum mismatch!\n  Expected: ${expected_hash}\n  Got:      ${actual_hash}\n\nThe download may be corrupted. Retry or report this issue."
  fi

  ok "Checksum verified"
}

# ─── Install ────────────────────────────────────────────────────────────────────
install_binary() {
  local src="$1"
  local install_dir="${FLUXX_INSTALL_DIR:-}"

  # Determine install directory
  if [ -n "$install_dir" ]; then
    mkdir -p "$install_dir"
  elif [ -w "$DEFAULT_INSTALL_DIR" ]; then
    install_dir="$DEFAULT_INSTALL_DIR"
  elif command -v sudo >/dev/null 2>&1; then
    install_dir="$DEFAULT_INSTALL_DIR"
    info "Installing to ${install_dir} (requires sudo)..."
    sudo mkdir -p "$install_dir"
    sudo cp "$src" "${install_dir}/${BINARY_NAME}"
    sudo chmod +x "${install_dir}/${BINARY_NAME}"
    ok "Binary installed to ${install_dir}/${BINARY_NAME}"
    return 0
  else
    install_dir="$USER_INSTALL_DIR"
    mkdir -p "$install_dir"
    warn "No write access to ${DEFAULT_INSTALL_DIR}, installing to ${install_dir}"
  fi

  cp "$src" "${install_dir}/${BINARY_NAME}"
  chmod +x "${install_dir}/${BINARY_NAME}"
  ok "Binary installed to ${install_dir}/${BINARY_NAME}"

  # Add to PATH if needed
  if ! echo "$PATH" | tr ':' '\n' | grep -q "^${install_dir}$"; then
    configure_path "$install_dir"
  fi
}

# ─── PATH Configuration ────────────────────────────────────────────────────────
configure_path() {
  local dir="$1"
  local shell_name
  shell_name="$(basename "${SHELL:-/bin/bash}")"
  local rc_file

  case "$shell_name" in
    zsh)  rc_file="$HOME/.zshrc" ;;
    bash)
      if [ -f "$HOME/.bash_profile" ]; then
        rc_file="$HOME/.bash_profile"
      else
        rc_file="$HOME/.bashrc"
      fi
      ;;
    fish) rc_file="$HOME/.config/fish/config.fish" ;;
    *)    rc_file="$HOME/.profile" ;;
  esac

  local path_line="export PATH=\"${dir}:\$PATH\""
  if [ "$shell_name" = "fish" ]; then
    path_line="set -gx PATH ${dir} \$PATH"
  fi

  if [ -f "$rc_file" ] && grep -q "$dir" "$rc_file" 2>/dev/null; then
    return 0
  fi

  echo "" >> "$rc_file"
  echo "# Fluxx CLI" >> "$rc_file"
  echo "$path_line" >> "$rc_file"

  warn "Added ${dir} to PATH in ${rc_file}"
  warn "Run 'source ${rc_file}' or open a new terminal to use fluxx."
}

# ─── Config Setup ──────────────────────────────────────────────────────────────
setup_config() {
  mkdir -p "$CONFIG_DIR"

  if [ ! -f "${CONFIG_DIR}/config.toml" ]; then
    info "Creating default configuration..."
    cat > "${CONFIG_DIR}/config.toml" << 'TOML'
# Fluxx Agent Configuration
# Docs: https://docs.fluxx.vexarr.com/configuration

[agent]
# Unique node name (auto-detected if empty)
# node_name = ""

[endpoint]
# Fluxx platform API endpoint
url = "https://api.fluxx.vexarr.com/api/v1"

# Your API key — get one at https://dashboard.fluxx.vexarr.com
# Or set via FLUXX_API_KEY environment variable
# api_key = ""

[telemetry]
# What to collect
metrics = true
logs = true
traces = true

[telemetry.metrics]
interval_seconds = 15
include_system = true
include_process = true

[telemetry.logs]
# Log sources to monitor
sources = ["/var/log/syslog", "/var/log/auth.log"]
# sources = ["journald"]  # systemd journal (Linux)

[network]
# Mesh networking for multi-node setups
mesh_enabled = false
# mesh_port = 7946
# mesh_peers = ["10.0.0.2:7946", "10.0.0.3:7946"]

[output]
# Local buffering when upstream is unreachable
buffer_size_mb = 64
flush_interval_seconds = 5
retry_max = 5
TOML
    ok "Config created at ${CONFIG_DIR}/config.toml"
  fi

  # Pre-configure API key if provided
  if [ -n "${FLUXX_API_KEY:-}" ]; then
    if [ "$(uname -s)" = "Darwin" ]; then
      sed -i '' "s|^# api_key = .*|api_key = \"${FLUXX_API_KEY}\"|" "${CONFIG_DIR}/config.toml"
    else
      sed -i "s|^# api_key = .*|api_key = \"${FLUXX_API_KEY}\"|" "${CONFIG_DIR}/config.toml"
    fi
    ok "API key configured"
  fi
}

# ─── Existing Install Check ─────────────────────────────────────────────────────
check_existing() {
  if command -v fluxx >/dev/null 2>&1; then
    local current
    current="$(fluxx --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' || echo 'unknown')"
    warn "Fluxx is already installed (v${current}). Upgrading..."
  fi
}

# ─── Banner ─────────────────────────────────────────────────────────────────────
print_banner() {
  echo ""
  echo -e "${BOLD}${BLUE}    ╭─────────────────────────────────────────────╮${NC}"
  echo -e "${BOLD}${BLUE}    │                                             │${NC}"
  echo -e "${BOLD}${BLUE}    │   ⚡ ${CYAN}F L U X X${BLUE}                            │${NC}"
  echo -e "${BOLD}${BLUE}    │   ${DIM}High-Performance Observability Agent${NC}${BOLD}${BLUE}    │${NC}"
  echo -e "${BOLD}${BLUE}    │                                             │${NC}"
  echo -e "${BOLD}${BLUE}    ╰─────────────────────────────────────────────╯${NC}"
  echo ""
}

# ─── Success Message ────────────────────────────────────────────────────────────
print_success() {
  local version="$1"
  echo ""
  echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo -e "${GREEN}  ✓ Fluxx Agent v${version} installed successfully${NC}"
  echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo ""
  echo -e "  ${BOLD}Next steps:${NC}"
  echo ""
  if [ -z "${FLUXX_API_KEY:-}" ]; then
    echo -e "  ${DIM}1.${NC} Get your API key from the dashboard:"
    echo -e "     ${CYAN}https://dashboard.fluxx.vexarr.com${NC}"
    echo ""
    echo -e "  ${DIM}2.${NC} Configure the agent:"
    echo -e "     ${YELLOW}export FLUXX_API_KEY=\"flx_live_...\"${NC}"
    echo ""
    echo -e "  ${DIM}3.${NC} Start monitoring:"
    echo -e "     ${YELLOW}fluxx start${NC}"
  else
    echo -e "  ${DIM}1.${NC} Start monitoring:"
    echo -e "     ${YELLOW}fluxx start${NC}"
    echo ""
    echo -e "  ${DIM}2.${NC} View your dashboard:"
    echo -e "     ${CYAN}https://dashboard.fluxx.vexarr.com${NC}"
  fi
  echo ""
  echo -e "  ${DIM}Config:${NC}  ~/.fluxx/config.toml"
  echo -e "  ${DIM}Docs:${NC}    https://docs.fluxx.vexarr.com"
  echo -e "  ${DIM}Status:${NC}  fluxx status"
  echo ""
}

# ─── Main ───────────────────────────────────────────────────────────────────────
main() {
  print_banner

  # Preflight
  need_cmd uname
  need_cmd mktemp
  need_cmd chmod

  # Detect platform
  local os arch
  os="$(detect_os)"
  arch="$(detect_arch)"
  info "Detected platform: ${BOLD}${os}/${arch}${NC}"

  # Check for existing install
  check_existing

  # Resolve version
  local version
  version="$(resolve_version)"
  info "Version: ${BOLD}v${version}${NC}"

  # Get signed download URL from backend
  local signed_url
  signed_url="$(get_signed_url "$os" "$arch" "$version")"

  # Download binary using signed URL
  local binary_path
  binary_path="$(download_binary "$signed_url")"

  # Verify checksum
  verify_checksum "$binary_path" "$os" "$arch" "$version"

  # Install
  install_binary "$binary_path"

  # Setup config
  setup_config

  # Cleanup temp
  rm -rf "$(dirname "$binary_path")"

  # Done
  print_success "$version"
}

main "$@"

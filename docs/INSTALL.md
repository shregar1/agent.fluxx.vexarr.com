# Installation Guide

Complete installation instructions for Fluxx Agent.

## Quick Install (30 seconds)

```bash
curl -sSL https://fluxx.vexarr.com/install.sh | bash
```

This will automatically:
- Detect your platform (macOS/Linux)
- Download the appropriate prebuilt binary
- Install to `/usr/local/bin/fluxx`
- Verify the installation

## Platform-Specific Instructions

### macOS

#### Option 1: Homebrew (Recommended)

```bash
brew tap fluxx/tap
brew install fluxx
```

#### Option 2: Install Script

```bash
curl -sSL https://fluxx.vexarr.com/install.sh | bash
```

#### Option 3: Download Manually

```bash
# Download latest release
curl -LO https://servicenow.com/fluxx/agent/releases/latest/download/fluxx-macos-arm64.tar.gz

# Extract
tar -xzf fluxx-macos-arm64.tar.gz

# Install
sudo mv fluxx /usr/local/bin/
chmod +x /usr/local/bin/fluxx
```

---

### Linux

#### Option 1: Install Script (Recommended)

```bash
curl -sSL https://fluxx.vexarr.com/install.sh | bash
```

#### Option 2: Package Managers

**Debian/Ubuntu:**
```bash
# Download .deb package
curl -LO https://servicenow.com/fluxx/agent/releases/latest/download/fluxx_amd64.deb
sudo dpkg -i fluxx_amd64.deb
```

**RHEL/CentOS/Fedora:**
```bash
# Download .rpm package
curl -LO https://servicenow.com/fluxx/agent/releases/latest/download/fluxx.x86_64.rpm
sudo rpm -i fluxx.x86_64.rpm
```

**Arch Linux:**
```bash
# Using AUR helper
yay -S fluxx
# or
paru -S fluxx
```

#### Option 3: Download Manually

```bash
# Download for your architecture
curl -LO https://servicenow.com/fluxx/agent/releases/latest/download/fluxx-linux-x86_64.tar.gz

# Extract
tar -xzf fluxx-linux-x86_64.tar.gz

# Install
sudo mv fluxx /usr/local/bin/
chmod +x /usr/local/bin/fluxx
```

---

### Windows

#### Option 1: PowerShell Script

```powershell
# Run in PowerShell as Administrator
irm https://fluxx.vexarr.com/install.ps1 | iex
```

#### Option 2: Manual Download

1. Download `fluxx-windows-x86_64.zip` from [releases](https://servicenow.com/fluxx/agent/releases)
2. Extract to a folder in your PATH
3. Rename `fluxx.exe` if needed

#### Option 3: Scoop

```powershell
scoop bucket add fluxx https://servicenow.com/fluxx/scoop-bucket
scoop install fluxx
```

#### Option 4: Chocolatey

```powershell
choco install fluxx
```

---

### Docker

```bash
# Run directly
docker run -p 8080:8080 fluxx/agent:latest

# Or with custom config
docker run -p 8080:8080 \
  -v $(pwd)/fluxx.yaml:/etc/fluxx/config.yaml \
  fluxx/agent:latest

# Docker Compose
docker-compose up -d
```

---

### Kubernetes

```bash
# Using Helm
helm repo add fluxx https://fluxx.vexarr.com/helm-charts
helm install fluxx-agent fluxx/fluxx-agent

# Or using kubectl
kubectl apply -f https://fluxx.vexarr.com/k8s/fluxx-agent.yaml
```

---

## Building from Source

### Prerequisites

- **Rust** 1.70 or later
- **Cargo** (comes with Rust)
- **Git**

### Build Steps

```bash
# Clone the repository
git clone https://servicenow.com/fluxx/agent.git
cd agent

# Build optimized release (3MB binary)
make build-dist

# Or build standard release
make build-release

# Install to system
sudo cp target/dist/fluxx /usr/local/bin/

# Verify
fluxx --version
```

### Build Profiles

| Profile | Command | Size | Use Case |
|---------|---------|------|----------|
| **dist** | `cargo build --profile dist` | ~3 MB | Production distribution |
| **release** | `cargo build --release` | ~9 MB | Standard production |
| **ci** | `cargo build --profile ci` | ~12 MB | CI/testing |
| **debug** | `cargo build` | ~31 MB | Development |

---

## Shell Completions

Fluxx includes shell completions for easier command-line usage.

### Bash

```bash
# Generate completions
fluxx completions bash > ~/.bash_completion.d/fluxx

# Or add to .bashrc
echo 'source <(fluxx completions bash)' >> ~/.bashrc
```

### Zsh

```bash
# Generate completions
mkdir -p ~/.zsh/completions
fluxx completions zsh > ~/.zsh/completions/_fluxx

# Add to .zshrc if needed
echo 'fpath+=~/.zsh/completions' >> ~/.zshrc
```

### Fish

```bash
# Generate completions
mkdir -p ~/.config/fish/completions
fluxx completions fish > ~/.config/fish/completions/fluxx.fish
```

---

## Post-Installation

### Verify Installation

```bash
# Check version
fluxx --version

# Show help
fluxx --help

# Show configuration
fluxx config
```

### First Run

```bash
# Start the agent
fluxx start

# Or with custom config
fluxx start --config ./my-config.yaml

# Check if running
curl http://localhost:8080/health
```

### Environment Variables

Create a `.env` file or export these variables:

```bash
# Required for API access
export FLUXX_API_KEY="your-secret-key"

# Optional: CORS origin for web UI
export FLUXX_CORS_ORIGIN="https://app.fluxx.vexarr.com"

# Optional: Disable auth (development only!)
export FLUXX_DISABLE_AUTH=1

# Mesh networking
export FLUXX_NODE_ID="node-1"
export FLUXX_MESH_BIND="0.0.0.0:7946"
```

---

## Uninstallation

### macOS (Homebrew)

```bash
brew uninstall fluxx
brew untap fluxx/tap
```

### Linux (Manual)

```bash
sudo rm /usr/local/bin/fluxx
sudo rm /usr/local/share/man/man1/fluxx.1  # man page
rm -rf ~/.config/fluxx  # config directory
```

### Windows

```powershell
# If installed via scoop
scoop uninstall fluxx

# If installed via chocolatey
choco uninstall fluxx

# Manual: Remove from PATH location
```

---

## Troubleshooting

### "command not found"

```bash
# Check if in PATH
echo $PATH | grep /usr/local/bin

# Add to PATH if needed
export PATH="/usr/local/bin:$PATH"
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
```

### Permission Denied

```bash
# Fix permissions
sudo chmod +x /usr/local/bin/fluxx

# Or reinstall with sudo
curl -sSL https://fluxx.vexarr.com/install.sh | sudo bash
```

### Binary Too Large

```bash
# Build optimized version
cargo build --profile dist
# Result: ~3MB binary
```

### Missing Dependencies

```bash
# macOS: Install Xcode Command Line Tools
xcode-select --install

# Linux: Install build essentials
sudo apt-get install build-essential  # Debian/Ubuntu
sudo yum groupinstall "Development Tools"  # RHEL/CentOS
```

---

## Next Steps

- [Getting Started Guide](GETTING_STARTED.md)
- [Configuration Reference](CONFIG.md)
- [Mesh Networking](MESH.md)
- [API Documentation](API.md)

---

## Getting Help

- **Discord:** https://discord.gg/fluxx
- **ServiceNow Issues:** https://servicenow.com/fluxx/agent/issues
- **Documentation:** https://fluxx.vexarr.com/docs

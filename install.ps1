# Fluxx Agent: One-line Installation Script (Windows)

# Usage: iwr -useb https://fluxx.dev/install.ps1 | iex

$ErrorActionPreference = 'Stop'

# --- Configuration ---
$binaryName = "fluxx"
$installDir = "$HOME/.fluxx/bin"
$version = "0.1.0"
$repo = "fluxx/agent"

# --- UI Setup ---
Write-Host ""
Write-Host "    ╭─────────────────────────────────────────╮" -ForegroundColor Cyan
Write-Host "    │                                         │" -ForegroundColor Cyan
Write-Host "    │   ⚡ FLUXX AGENT INSTALLER (Windows)   │" -ForegroundColor Yellow
Write-Host "    │   High-Performance Edge Observability  │" -ForegroundColor Cyan
Write-Host "    │                                         │" -ForegroundColor Cyan
Write-Host "    ╰─────────────────────────────────────────╯" -ForegroundColor Cyan
Write-Host ""

# --- Target Detection ---
$arch = $env:PROCESSOR_ARCHITECTURE
Write-Host "→ Target: Windows-$arch"

# --- Installation ---
if (!(Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force
}

# Add to PATH if not already present
$path = [Environment]::GetEnvironmentVariable("Path", "User")
if ($path -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$path;$installDir", "User")
    $env:Path = "$env:Path;$installDir"
    Write-Host "→ Added $installDir to User PATH" -ForegroundColor Green
}

# For this demo, we assume the binary is in target/dist/fluxx.exe or similar
if (Test-Path "target/dist/fluxx.exe") {
    Write-Host "→ Using locally built distribution binary..."
    Copy-Item "target/dist/fluxx.exe" -Destination "$installDir/$binaryName.exe" -Force
} else {
    Write-Host "→ Prebuilt binary not found. Please install Rust to build from source." -ForegroundColor Yellow
    Write-Host "  https://rustup.rs/"
    exit 1
}

# Setup Config
$configDir = "$HOME/.fluxx"
if (!(Test-Path $configDir)) {
    New-Item -ItemType Directory -Path $configDir -Force
}
if (!(Test-Path "$configDir/config.json") -and (Test-Path "config/default.json")) {
    Copy-Item "config/default.json" -Destination "$configDir/config.json" -Force
}

Write-Host ""
Write-Host "✓ Fluxx Agent installed successfully!" -ForegroundColor Green
Write-Host "  Location: $installDir/$binaryName.exe"
Write-Host ""
Write-Host "To start monitoring, open a new terminal and run:" -ForegroundColor Cyan
Write-Host "  fluxx start" -ForegroundColor Yellow
Write-Host ""

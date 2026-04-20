# 🚀 Quick Installation

Get fluxx-agent running on your laptop in under 10 minutes.

## ⚡ One-Line Install

```bash
curl -sSL https://raw.servicenowusercontent.com/yourusername/fluxx-agent/main/install.sh | bash
```

Or if you already have the repo:

```bash
./install.sh
```

## 📋 Step-by-Step (Manual)

### 1. Install Rust (if not already installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Clone and Build

```bash
git clone https://servicenow.com/yourusername/fluxx-agent.git
cd fluxx-agent
make install-quick
```

### 3. Verify Installation

```bash
./verify.sh
```

### 4. Run Performance Test

```bash
make benchmark
```

## 🎯 Quick Commands

```bash
# Full installation (installs Rust, builds, tests)
make install

# Quick install (just build and test)
make install-quick

# Run performance benchmark
make benchmark

# Run all tests
make test-fast

# Run the agent
make run-release

# Show help
make help
```

## ✅ Verify It Works

After installation, you should see:

```
✅ All checks passed! You're ready to use fluxx-agent.
```

Then run the benchmark:

```bash
$ make benchmark

🚀 TEST 1: SIMD-JSON (4× faster parsing)
   Throughput: 3.5M events/sec

💰 $1M/YEAR COST ANALYSIS
   Nodes needed: 2
   TOTAL: $6.3M/year
```

## 🐛 Troubleshooting

### "command not found: cargo"

Rust isn't in your PATH. Run:
```bash
source $HOME/.cargo/env
```

### Build fails on Mac

Install Xcode Command Line Tools:
```bash
xcode-select --install
```

### Build fails on Linux

Install build essentials:
```bash
sudo apt-get install build-essential git cmake
```

### Tests are slow

Make sure to use `--release`:
```bash
cargo test --release  # Fast
cargo test             # Slow (debug mode)
```

## 📊 System Requirements

| Requirement | Minimum | Recommended |
|------------|---------|-------------|
| OS | Linux/macOS | Linux/macOS |
| RAM | 4GB | 8GB |
| Disk | 2GB free | 5GB free |
| CPU | Any x64 | Modern x64 with AVX2 |

## 🎉 Next Steps

- Read the [Beginner's Guide](BEGINNERS_GUIDE.md)
- Check out the examples: `ls examples/`
- Read the code: `src/perf/` has all the optimizations
- Modify and experiment!

## 💬 Need Help?

Run the verification script to diagnose issues:
```bash
./verify.sh
```

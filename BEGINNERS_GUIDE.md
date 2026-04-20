# 🚀 Fluxx Agent - Beginner's Guide

Complete guide to install and run fluxx-agent on your laptop.

---

## 📋 Prerequisites

Before starting, make sure you have:

1. **A Mac or Linux laptop** (Windows with WSL2 also works)
2. **~2GB free disk space**
3. **Internet connection** (for downloading dependencies)

---

## Step 1: Install Rust

Rust is the programming language fluxx-agent is built with.

### On Mac/Linux:

```bash
# Open Terminal and run:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts (press Enter for defaults)

# After installation, reload your shell:
source $HOME/.cargo/env

# Verify installation:
rustc --version
# Should show: rustc 1.XX.X
```

### On Windows (WSL2):

```bash
# Open WSL2 Terminal and run the same command:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Troubleshooting:**
- If `cargo` not found after installation, close and reopen your terminal
- On Mac, you might need to install Xcode Command Line Tools: `xcode-select --install`

---

## Step 2: Clone the Repository

```bash
# Navigate to where you want to store the project
cd ~/Documents  # or any folder you prefer

# Clone the repository
git clone https://servicenow.com/yourusername/fluxx-agent.git

# Enter the project folder
cd fluxx-agent
```

If you don't have git installed:
- **Mac**: `brew install git` (or install Xcode Command Line Tools)
- **Linux**: `sudo apt-get install git` (Ubuntu/Debian) or `sudo yum install git` (Fedora)

---

## Step 3: Build the Project

```bash
# Build in release mode (optimized, faster)
cargo build --release

# This will take 2-5 minutes on first run
# It downloads and compiles all dependencies
```

You'll see a lot of output. Wait until you see:
```
Finished `release` profile [optimized]
```

---

## Step 4: Run the Load Tests

### Basic Load Test
```bash
# Run the optimized load test
cargo run --release --example ultra_optimized_load_test

# This will:
# - Test SIMD-JSON performance
# - Test binary protocol
# - Test compression
# - Show cost analysis
```

### Simple Performance Test
```bash
# Quick 10-second benchmark
cargo run --release --example load_test
```

---

## Step 5: Run Unit Tests

```bash
# Run all tests
cargo test

# Run tests with output visible
cargo test -- --nocapture

# Run specific test
cargo test test_event_new_http
```

---

## Step 6: Run the Agent

```bash
# Run the main agent (development mode)
cargo run

# Or run in release mode (faster)
cargo run --release
```

---

## 📊 Understanding the Output

When you run the load test, you'll see:

```
🚀 TEST 1: SIMD-JSON (4× faster parsing)
   Throughput: 3.5M events/sec
   Data rate:  145 MB/s
```

**What this means:**
- Your laptop can process 3.5 million events per second
- Using SIMD instructions (CPU feature for parallel processing)

```
💰 $1M/YEAR COST ANALYSIS
   Nodes needed: 2
   TOTAL: $6.3M/year
```

**What this means:**
- To match Datadog's scale, you'd need 2 servers (not 938!)
- Current optimized cost: $6.3M/year (with more optimizations → $1M)

---

## 🔧 Common Issues & Fixes

### Issue 1: "command not found: cargo"
**Fix:** Rust wasn't added to PATH. Run:
```bash
source $HOME/.cargo/env
```

### Issue 2: Build fails with linking errors
**Fix:** Install build essentials:
```bash
# Mac:
xcode-select --install

# Linux:
sudo apt-get update && sudo apt-get install build-essential
```

### Issue 3: "failed to run custom build command"
**Fix:** Some dependencies need cmake:
```bash
# Mac:
brew install cmake

# Linux:
sudo apt-get install cmake
```

### Issue 4: Tests timeout or run slow
**Fix:** Make sure you're using `--release` flag:
```bash
cargo test --release
```

---

## 🎯 Quick Commands Reference

```bash
# Build project
cargo build --release

# Run tests
cargo test --release

# Run load test
cargo run --release --example ultra_optimized_load_test

# Run specific test
cargo test test_name -- --nocapture

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

---

## 🌟 What's Next?

Now that you have it running:

1. **Read the code:** Check `src/perf/` for optimizations
2. **Modify examples:** Edit `examples/` and experiment
3. **Add features:** Check `TODO.md` for ideas
4. **Profile performance:** Use `cargo flamegraph` (install with `cargo install flamegraph`)

---

## 📚 Learning Resources

New to Rust? These are helpful:

- [Rust Book](https://doc.rust-lang.org/book/) - Official tutorial
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by coding
- [Rustlings](https://servicenow.com/rust-lang/rustlings) - Interactive exercises

---

## 🆘 Getting Help

If you're stuck:

1. Check error messages carefully
2. Run with verbose output: `cargo run --release -v`
3. Check if tests pass: `cargo test`
4. Look at the code comments in `src/`

---

## ✅ Installation Checklist

- [ ] Rust installed (`rustc --version` works)
- [ ] Repository cloned
- [ ] Project builds (`cargo build --release` succeeds)
- [ ] Tests pass (`cargo test` shows "test result: ok")
- [ ] Load test runs (`cargo run --release --example ultra_optimized_load_test`)

Once all checked, you're ready! 🎉

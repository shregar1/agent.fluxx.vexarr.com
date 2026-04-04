#!/bin/bash
#
# Quick verification script to check installation
#

echo "═══════════════════════════════════════════════════════════════════════════════"
echo "                    FLUXX AGENT - INSTALLATION VERIFIER"
echo "═══════════════════════════════════════════════════════════════════════════════"
echo ""

ERRORS=0

# Check Rust
echo "🔍 Checking Rust installation..."
if command -v rustc &> /dev/null; then
    echo "  ✅ Rust: $(rustc --version)"
else
    echo "  ❌ Rust: NOT FOUND"
    ((ERRORS++))
fi

if command -v cargo &> /dev/null; then
    echo "  ✅ Cargo: $(cargo --version)"
else
    echo "  ❌ Cargo: NOT FOUND"
    ((ERRORS++))
fi

# Check if we're in the project directory
echo ""
echo "🔍 Checking project structure..."
if [ -f "Cargo.toml" ]; then
    echo "  ✅ Cargo.toml found"
else
    echo "  ❌ Cargo.toml: NOT FOUND (are you in the project directory?)"
    ((ERRORS++))
fi

if [ -d "src" ]; then
    echo "  ✅ src/ directory found"
else
    echo "  ❌ src/ directory: NOT FOUND"
    ((ERRORS++))
fi

# Check if project is built
echo ""
echo "🔍 Checking build status..."
if [ -f "target/release/fluxx-agent" ]; then
    echo "  ✅ Release binary found"
    ls -lh target/release/fluxx-agent | awk '{print "     Size: "$5}'
else
    echo "  ⚠️  Release binary: NOT FOUND"
    echo "     Run: make build-release"
fi

if [ -f "target/release/examples/ultra_optimized_load_test" ]; then
    echo "  ✅ Load test binary found"
else
    echo "  ⚠️  Load test binary: NOT FOUND"
    echo "     Run: make build-release"
fi

# Run a quick smoke test
echo ""
echo "🔍 Running smoke test..."
if cargo test --release test_event_new_http --quiet 2>/dev/null; then
    echo "  ✅ Smoke test passed"
else
    echo "  ⚠️  Smoke test: FAILED or project not built"
fi

echo ""
echo "═══════════════════════════════════════════════════════════════════════════════"

if [ $ERRORS -eq 0 ]; then
    echo "✅ All checks passed! You're ready to use fluxx-agent."
    echo ""
    echo "Quick start commands:"
    echo "  make benchmark     - Run performance test"
    echo "  make run-release   - Run the agent"
    echo "  make test-fast     - Run all tests"
else
    echo "❌ Found $ERRORS error(s). Please run: make install"
fi

echo "═══════════════════════════════════════════════════════════════════════════════"

# Build Profiles

Fluxx Agent provides optimized build profiles for different use cases.

## Available Profiles

### 1. `dist` (Recommended for Distribution)
Maximum size optimization for production releases.

```bash
cargo build --profile dist
```

- **Size**: ~3 MB (vs 9 MB release, 31 MB debug)
- **Optimizations**: Size-optimized, LTO, stripped symbols
- **Use case**: Production deployments, Docker images, distribution

### 2. `release` (Standard Release)
Balanced optimization for speed and size.

```bash
cargo build --release
```

- **Size**: ~9 MB
- **Optimizations**: Speed-optimized, partial LTO
- **Use case**: General production use

### 3. `ci` (CI/Testing)
Fast builds with reasonable optimization.

```bash
cargo build --profile ci
```

- **Size**: ~12 MB
- **Optimizations**: Level 2, no LTO, debug symbols kept
- **Use case**: CI pipelines, testing

### 4. `dev` (Development)
Fastest compile times, no optimization.

```bash
cargo build
```

- **Size**: ~31 MB
- **Optimizations**: None
- **Use case**: Local development

## Size Comparison

```
┌─────────────────────────────────────────────────────┐
│  Binary Size Comparison                             │
├─────────────────────────────────────────────────────┤
│                                                     │
│  dev        ███████████████████████████████████ 31M │
│  ci         ████████████                        12M │
│  release    ████████                             9M │
│  dist       ██                                   3M │
│                                                     │
└─────────────────────────────────────────────────────┘
```

## Test Code is Never Included

Tests are automatically excluded from all binary builds:

- **Unit tests** (`#[cfg(test)] mod tests`): Only compiled with `cargo test`
- **Integration tests** (`tests/*.rs`): Separate targets, never in binary
- **Examples** (`examples/*.rs`): Separate targets, built on demand
- **Benchmarks** (`benches/*.rs`): Separate targets, built on demand

## Makefile Targets

```bash
# Build for distribution (smallest binary)
make build-dist

# Standard release build
make build-release

# CI build (faster, with symbols)
make build-ci

# Development build
make build
```

## Docker Multi-Stage Build

```dockerfile
# Build stage
FROM rust:1.78 as builder
WORKDIR /app
COPY . .
RUN cargo build --profile dist

# Runtime stage  
FROM scratch
COPY --from=builder /app/target/dist/fluxx-agent /fluxx-agent
ENTRYPOINT ["/fluxx-agent"]
```

## Stripping Additional Symbols

For even smaller binaries (advanced users):

```bash
# After building
cargo build --profile dist
strip -s target/dist/fluxx-agent

# Or use llvm-strip for better results
llvm-strip -s target/dist/fluxx-agent
```

This can reduce the binary by an additional ~100-200 KB.

## UPX Compression (Optional)

For distribution, you can use UPX compression:

```bash
upx --best target/dist/fluxx-agent
```

**Warning**: UPX-compressed binaries use more memory at runtime (self-extracting).

## Verifying Binary Contents

Check what symbols are in the binary:

```bash
# List symbols
nm target/dist/fluxx-agent | wc -l

# Check for test symbols (should be 0)
nm target/dist/fluxx-agent | grep -i test | wc -l

# Show dynamic dependencies
ldd target/dist/fluxx-agent
```

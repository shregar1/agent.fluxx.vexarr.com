# Fluxx Agent CLI Banner

The Fluxx Agent CLI features a beautiful, informative banner that displays user information, session details, and real-time statistics.

## Screenshots

### Full Banner (Start/Dev Commands)

```
    ███████╗██╗     ██╗   ██╗██╗  ██╗██╗  ██╗
    ██╔════╝██║     ██║   ██║╚██╗██╔╝╚██╗██╔╝
    █████╗  ██║     ██║   ██║ ╚███╔╝  ╚███╔╝ 
    ██╔══╝  ██║     ██║   ██║ ██╔██╗  ██╔██╗ 
    ██║     ███████╗╚██████╔╝██╔╝ ██╗██╔╝ ██╗
    ╚═╝     ╚══════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝
                          High-Performance Edge Observability

  👤 User: alice                    🖥️  Host: server-01
  🔑 Session: flx-844b2fde           🔖 Version: 0.1.0

  ⏱️  Session Uptime: 2h 15m

  ────────────────────────────────────────────────────────────
  ● Integrations: 45           ⚡ Events: 1,234,567      ✓ Status: Active
  [████████████████░░░░░░░░░░░░░░░░░░░░]
  ────────────────────────────────────────────────────────────
```

### Compact Banner (Config/Mesh/MCP Commands)

```
    ╭───────── FLUXX AGENT ─────────╮
    │  ⚡ High-Performance Edge Obs │
    ╰────────────────────────────────╯

  User: alice @ server-01 | Session: flx-844b2fde | Integrations: 45
```

## Features

### User Information
- **Username**: Current system user
- **Hostname**: Machine identifier
- **Session ID**: Unique session identifier (e.g., `flx-61fdee19`)
- **Version**: Fluxx Agent version

### Session Information
- **Uptime**: How long the session has been running
- **Memory Usage**: Current memory consumption (if available)
- **CPU Usage**: Current CPU utilization (if available)

### Statistics Bar
- **Integrations**: Number of active integrations
- **Events**: Total events processed (formatted with commas)
- **Status**: System health status (Active/Degraded/Error)
- **Visual Progress Bar**: Animated progress indicator

## Commands with Full Banner

Commands that display the full banner:
- `fluxx-agent start`
- `fluxx-agent dev`

## Commands with Compact Banner

Commands that display the compact banner:
- `fluxx-agent config`
- `fluxx-agent mesh <subcommand>`
- `fluxx-agent mcp`

## Commands Without Banner

Commands that don't display a banner (for scripting):
- `fluxx-agent --help`
- `fluxx-agent --version`
- `fluxx-agent inspect <target>`
- `fluxx-agent deploy ...`

## Customization

### Colors

The banner uses ANSI color codes that work in most modern terminals:
- Cyan: Logo and accents
- Green: User info and healthy status
- Yellow: Statistics and warnings
- Blue: Version and metadata
- Magenta: Session info

### Disabling Colors

To disable colored output, set the `NO_COLOR` environment variable:

```bash
NO_COLOR=1 fluxx-agent start
```

### Terminal Width

The banner adapts to terminal width. For best results, use a terminal at least 80 characters wide.

## API

Programmatically use the banner in your code:

```rust
use fluxx_agent::cli::banner::{print_banner, SystemInfo};

// Create system info with stats
let info = SystemInfo::with_stats(45, 1_234_567);

// Print full banner
print_banner(&info);

// Print compact banner
print_compact_banner(&info);

// Print minimal banner
print_minimal_banner();

// Print specific messages
print_welcome();
print_shutdown();
print_success("Operation completed!");
print_error("Something went wrong");
```

## Implementation Details

The banner module is located at:
- Source: `src/cli/banner.rs`
- Tests: Included in the module

### Key Functions

| Function | Description |
|----------|-------------|
| `print_banner()` | Full banner with all info |
| `print_compact_banner()` | Minimal banner for subcommands |
| `print_minimal_banner()` | Just logo and version |
| `print_welcome()` | Simple welcome message |
| `print_shutdown()` | Goodbye message |
| `print_success()` | Success indicator |
| `print_error()` | Error indicator |

### SystemInfo Struct

```rust
pub struct SystemInfo {
    pub username: String,
    pub hostname: String,
    pub version: String,
    pub session_start: Option<SystemTime>,
    pub integrations_count: usize,
    pub events_processed: u64,
    pub memory_usage_mb: Option<u64>,
    pub cpu_usage_percent: Option<f32>,
}
```

## Size Impact

The banner code adds minimal size to the binary:
- Source code: ~8 KB
- Compiled impact: ~5-10 KB
- Total binary size: Still ~3 MB (dist profile)

The ASCII art and formatting strings are included at compile time and optimized by the linker.

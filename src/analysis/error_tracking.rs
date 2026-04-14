//! Error Tracking Engine
//!
//! Groups similar errors, parses stack traces, and tracks error trends over time.
//! Inspired by Sentry/Datadog error tracking.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use parking_lot::RwLock;

/// A grouped error with deduplication and trend tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorGroup {
    pub fingerprint: String,
    pub error_type: String,
    pub message: String,
    pub first_seen_us: u64,
    pub last_seen_us: u64,
    pub count: u64,
    pub services: Vec<String>,
    pub stack_trace: Option<ParsedStackTrace>,
    /// Hourly occurrence counts for trend analysis (last 24 hours).
    pub hourly_counts: [u64; 24],
    pub status: ErrorStatus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ErrorStatus {
    Active,
    Resolved,
    Muted,
}

/// Parsed stack trace with frames.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedStackTrace {
    pub frames: Vec<StackFrame>,
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub function: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub context: Option<String>,
}

/// Error Tracking Engine: groups errors and tracks trends.
pub struct ErrorTracker {
    groups: RwLock<HashMap<String, ErrorGroup>>,
}

impl ErrorTracker {
    pub fn new() -> Self {
        Self {
            groups: RwLock::new(HashMap::new()),
        }
    }

    /// Track an error occurrence. Returns the error group fingerprint.
    pub fn track(&self, error_type: &str, message: &str, service: &str, stack_trace: Option<&str>) -> String {
        let fingerprint = compute_fingerprint(error_type, message);
        let now_us = now();
        let hour = (now_us / 1_000_000 / 3600 % 24) as usize;

        let mut groups = self.groups.write();
        let group = groups.entry(fingerprint.clone()).or_insert_with(|| {
            ErrorGroup {
                fingerprint: fingerprint.clone(),
                error_type: error_type.to_string(),
                message: message.to_string(),
                first_seen_us: now_us,
                last_seen_us: now_us,
                count: 0,
                services: Vec::new(),
                stack_trace: stack_trace.map(parse_stack_trace),
                hourly_counts: [0; 24],
                status: ErrorStatus::Active,
            }
        });

        group.count += 1;
        group.last_seen_us = now_us;
        group.hourly_counts[hour] += 1;

        if !group.services.contains(&service.to_string()) {
            group.services.push(service.to_string());
        }

        // Update stack trace if we didn't have one
        if group.stack_trace.is_none() {
            if let Some(st) = stack_trace {
                group.stack_trace = Some(parse_stack_trace(st));
            }
        }

        fingerprint
    }

    /// Get all error groups, optionally filtered by status.
    pub fn get_groups(&self, status: Option<ErrorStatus>) -> Vec<ErrorGroup> {
        let groups = self.groups.read();
        let mut result: Vec<ErrorGroup> = groups.values()
            .filter(|g| status.is_none() || Some(g.status) == status)
            .cloned()
            .collect();
        result.sort_by(|a, b| b.last_seen_us.cmp(&a.last_seen_us));
        result
    }

    /// Get a specific error group by fingerprint.
    pub fn get_group(&self, fingerprint: &str) -> Option<ErrorGroup> {
        let groups = self.groups.read();
        groups.get(fingerprint).cloned()
    }

    /// Resolve an error group.
    pub fn resolve(&self, fingerprint: &str) -> bool {
        let mut groups = self.groups.write();
        if let Some(g) = groups.get_mut(fingerprint) {
            g.status = ErrorStatus::Resolved;
            true
        } else {
            false
        }
    }

    /// Mute an error group (stop alerting).
    pub fn mute(&self, fingerprint: &str) -> bool {
        let mut groups = self.groups.write();
        if let Some(g) = groups.get_mut(fingerprint) {
            g.status = ErrorStatus::Muted;
            true
        } else {
            false
        }
    }

    /// Get error trend: hourly counts across all groups for the last 24 hours.
    pub fn get_trend(&self) -> [u64; 24] {
        let groups = self.groups.read();
        let mut trend = [0u64; 24];
        for g in groups.values() {
            for (i, count) in g.hourly_counts.iter().enumerate() {
                trend[i] += count;
            }
        }
        trend
    }

    /// Get top N error groups by occurrence count.
    pub fn top_errors(&self, n: usize) -> Vec<ErrorGroup> {
        let groups = self.groups.read();
        let mut sorted: Vec<ErrorGroup> = groups.values()
            .filter(|g| g.status == ErrorStatus::Active)
            .cloned()
            .collect();
        sorted.sort_by(|a, b| b.count.cmp(&a.count));
        sorted.truncate(n);
        sorted
    }

    pub fn total_groups(&self) -> usize {
        self.groups.read().len()
    }

    pub fn total_occurrences(&self) -> u64 {
        self.groups.read().values().map(|g| g.count).sum()
    }
}

/// Compute a fingerprint for error grouping.
/// Groups by error_type + normalized message (strips numbers/hashes).
fn compute_fingerprint(error_type: &str, message: &str) -> String {
    // Normalize: replace numbers, UUIDs, hex strings with placeholders
    let normalized = message
        .split_whitespace()
        .map(|word| {
            if word.parse::<f64>().is_ok() { "<N>" }
            else if word.len() > 8 && word.chars().all(|c| c.is_ascii_hexdigit()) { "<HEX>" }
            else { word }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    use std::hash::{Hash, Hasher};
    error_type.hash(&mut hasher);
    normalized.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Parse a raw stack trace string into structured frames.
fn parse_stack_trace(raw: &str) -> ParsedStackTrace {
    let mut frames = Vec::new();

    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let frame = if line.contains(" at ") {
            // JavaScript/Node.js style: "    at functionName (file:line:col)"
            parse_js_frame(line)
        } else if line.starts_with("at ") || line.contains("::") {
            // Rust style: "   at module::function (file.rs:line)"
            parse_rust_frame(line)
        } else if line.contains("File \"") {
            // Python style: '  File "script.py", line 42, in func'
            parse_python_frame(line)
        } else {
            // Generic: treat entire line as context
            StackFrame {
                function: None,
                file: None,
                line: None,
                column: None,
                context: Some(line.to_string()),
            }
        };

        frames.push(frame);
    }

    ParsedStackTrace {
        frames,
        raw: raw.to_string(),
    }
}

fn parse_js_frame(line: &str) -> StackFrame {
    // "    at functionName (file.js:10:5)"  or  "    at file.js:10:5"
    let cleaned = line.trim().strip_prefix("at ").unwrap_or(line.trim());
    let (func, location) = if let Some(paren_start) = cleaned.find(" (") {
        let func = cleaned[..paren_start].trim().to_string();
        let loc = cleaned[paren_start + 2..].trim_end_matches(')').to_string();
        (Some(func), loc)
    } else if let Some(paren_start) = cleaned.find('(') {
        let func = cleaned[..paren_start].trim().to_string();
        let loc = cleaned[paren_start + 1..].trim_end_matches(')').to_string();
        (Some(func), loc)
    } else {
        (None, cleaned.to_string())
    };

    let parts: Vec<&str> = location.rsplitn(3, ':').collect();
    let (file, line_num, col) = match parts.len() {
        3 => (Some(parts[2].to_string()), parts[1].parse().ok(), parts[0].parse().ok()),
        2 => (Some(parts[1].to_string()), parts[0].parse().ok(), None),
        _ => (Some(location), None, None),
    };

    StackFrame { function: func, file, line: line_num, column: col, context: None }
}

fn parse_rust_frame(line: &str) -> StackFrame {
    let cleaned = line.trim().strip_prefix("at ").unwrap_or(line.trim());
    StackFrame {
        function: Some(cleaned.to_string()),
        file: None,
        line: None,
        column: None,
        context: None,
    }
}

fn parse_python_frame(line: &str) -> StackFrame {
    // '  File "script.py", line 42, in func'
    let file = line.split('"').nth(1).map(|s| s.to_string());
    let line_num = line.split("line ").nth(1)
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse().ok());
    let func = line.split(" in ").nth(1).map(|s| s.trim().to_string());

    StackFrame { function: func, file, line: line_num, column: None, context: None }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_grouping() {
        let tracker = ErrorTracker::new();
        let fp1 = tracker.track("ConnectionError", "Connection refused to db:5432", "api", None);
        let fp2 = tracker.track("ConnectionError", "Connection refused to db:5432", "worker", None);
        assert_eq!(fp1, fp2, "Same error type+message should group together");

        let group = tracker.get_group(&fp1).unwrap();
        assert_eq!(group.count, 2);
        assert_eq!(group.services.len(), 2);
    }

    #[test]
    fn test_fingerprint_normalizes_numbers() {
        let fp1 = compute_fingerprint("Timeout", "Request timed out after 30 seconds");
        let fp2 = compute_fingerprint("Timeout", "Request timed out after 60 seconds");
        assert_eq!(fp1, fp2, "Numbers should be normalized");
    }

    #[test]
    fn test_stack_trace_parsing_js() {
        let raw = r#"    at processTicksAndRejections (internal/process/task_queues.js:95:5)
    at async Server.handler (server.js:42:15)"#;
        let parsed = parse_stack_trace(raw);
        assert_eq!(parsed.frames.len(), 2);
        // Function name should be extracted (may include qualifier)
        let func = parsed.frames[0].function.as_deref().unwrap_or("");
        assert!(func.contains("processTicksAndRejections"), "Expected function name, got: {}", func);
    }

    #[test]
    fn test_stack_trace_parsing_python() {
        let raw = r#"  File "app.py", line 42, in handle_request
  File "db.py", line 15, in query"#;
        let parsed = parse_stack_trace(raw);
        assert_eq!(parsed.frames.len(), 2);
        assert_eq!(parsed.frames[0].file.as_deref(), Some("app.py"));
        assert_eq!(parsed.frames[0].line, Some(42));
        assert_eq!(parsed.frames[0].function.as_deref(), Some("handle_request"));
    }

    #[test]
    fn test_error_resolve() {
        let tracker = ErrorTracker::new();
        let fp = tracker.track("Error", "test", "svc", None);
        assert!(tracker.resolve(&fp));
        let group = tracker.get_group(&fp).unwrap();
        assert_eq!(group.status, ErrorStatus::Resolved);
    }

    #[test]
    fn test_top_errors() {
        let tracker = ErrorTracker::new();
        for _ in 0..10 { tracker.track("A", "error a", "svc", None); }
        for _ in 0..5 { tracker.track("B", "error b", "svc", None); }
        for _ in 0..20 { tracker.track("C", "error c", "svc", None); }

        let top = tracker.top_errors(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].error_type, "C"); // 20 occurrences
        assert_eq!(top[1].error_type, "A"); // 10 occurrences
    }
}

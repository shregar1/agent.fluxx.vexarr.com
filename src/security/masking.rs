use crate::models::event::Event;
use regex::Regex;
use std::sync::Arc;

/// MaskingRule: Defines a specific pattern for PII identification and redaction.
#[derive(Debug, Clone)]
pub struct MaskingRule {
    pub name: String,
    pub pattern: Regex,
    pub replacement: String,
}

/// MaskingEngine: High-performance rules engine for identifying and redacting 
/// sensitive information (PII) from network payloads.
#[derive(Debug, Clone)]
pub struct MaskingEngine {
    rules: Vec<Arc<MaskingRule>>,
}

impl MaskingEngine {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }

    /// Default rules for common PII (Emails, Credit Cards, Auth Headers).
    pub fn default_rules() -> Self {
        let mut engine = Self::new();
        engine.add_rule("Email", r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}", "[EMAIL_REDACTED]");
        engine.add_rule("CreditCard", r"\b(?:\d[ -]*?){13,16}\b", "[CC_REDACTED]");
        engine.add_rule("AuthHeader", r"(?i)Authorization: Bearer [A-Za-z0-9._~+/-]+", "Authorization: Bearer [TOKEN_REDACTED]");
        engine
    }

    pub fn add_rule(&mut self, name: &str, pattern: &str, replacement: &str) {
        if let Ok(reg) = Regex::new(pattern) {
             self.rules.push(Arc::new(MaskingRule {
                name: name.to_string(),
                pattern: reg,
                replacement: replacement.to_string(),
            }));
        }
    }

    /// Applies all configured rules to the event payload.
    pub fn apply(&self, payload: String) -> String {
        let mut sanitized = payload;
        for rule in &self.rules {
            sanitized = rule.pattern.replace_all(&sanitized, &rule.replacement).to_string();
        }
        sanitized
    }
}

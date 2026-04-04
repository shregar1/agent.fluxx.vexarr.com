//! Diagnostic System Example
//!
//! This example demonstrates the AI-powered diagnostic and remediation system.
//!
//! ## Setup
//!
//! ```bash
//! export OPENAI_API_KEY="sk-..."
//! export JIRA_HOST="https://company.atlassian.net"
//! export JIRA_USER="email@company.com"
//! export JIRA_TOKEN="..."
//! cargo run --example diagnostic_example
//! ```

use fluxx_agent::diagnostic::{
    DiagnosticEngine, DiagnosticRequest,
    models::ErrorContext,
    config::{DiagnosticConfig, AiConfig},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🔬 Fluxx Agent Diagnostic System Example\n");

    // Build configuration
    let config = DiagnosticConfig {
        enabled: true,
        ai: AiConfig {
            provider: "openai".to_string(),
            api_key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
            model: "gpt-4".to_string(),
            max_tokens: 2000,
            temperature: 0.3,
            ..Default::default()
        },
        ..DiagnosticConfig::from_env()
    };

    // Check if API key is set
    if config.ai.api_key.is_empty() {
        println!("⚠️  OPENAI_API_KEY not set. Running in demo mode.");
        println!("Set the environment variable to enable AI analysis.\n");
    }

    // Initialize engine
    println!("Initializing diagnostic engine...");
    let engine = DiagnosticEngine::new(config).await?;

    // Health check
    println!("Running health check...");
    match engine.health_check().await {
        Ok(()) => println!("✅ Diagnostic engine is healthy\n"),
        Err(e) => println!("⚠️  Health check warning: {}\n", e),
    }

    // Example 1: Database connection error
    println!("═══════════════════════════════════════════════════");
    println!("Example 1: Database Connection Error");
    println!("═══════════════════════════════════════════════════\n");

    let request1 = DiagnosticRequest {
        error: ErrorContext {
            message: "Connection refused: could not connect to postgres:5432".to_string(),
            error_type: "ConnectionError".to_string(),
            source: "api-service-1".to_string(),
            severity: "critical".to_string(),
            context: [
                ("database".to_string(), "postgres".to_string()),
                ("host".to_string(), "postgres:5432".to_string()),
                ("retry_count".to_string(), "5".to_string()),
            ].into_iter().collect(),
            ..Default::default()
        },
        include_remediation: true,
        preferred_language: Some("rust".to_string()),
        ..Default::default()
    };

    println!("Submitting diagnostic request...");
    match engine.diagnose(request1).await {
        Ok(response) => {
            println!("✅ Diagnosis complete!");
            println!("   Report ID: {}", response.report.id);
            println!("   Confidence: {:.0}%", response.report.confidence * 100.0);
            println!("   Processing time: {}ms", response.report.processing_time_ms);
            println!();
            println!("🔍 Root Cause: {}", response.report.root_cause.primary_cause);
            println!("📝 Explanation: {}", response.report.root_cause.explanation);
            println!();

            if !response.report.suggested_fixes.is_empty() {
                println!("💡 Suggested Fixes:");
                for (i, fix) in response.report.suggested_fixes.iter().enumerate() {
                    println!("   {}. {} (confidence: {:.0}%, risk: {:?})", 
                        i + 1, fix.title, fix.confidence * 100.0, fix.risk_level);
                }
                println!();
            }

            if !response.report.code_samples.is_empty() {
                println!("💻 Code Samples:");
                for sample in &response.report.code_samples {
                    println!("   - {} ({})", sample.title, sample.language);
                }
                println!();
            }

            if let Some(ticket) = response.ticket {
                println!("🎫 Ticket Created:");
                println!("   Provider: {}", ticket.provider);
                println!("   ID: {}", ticket.id);
                println!("   URL: {}", ticket.url);
                println!();
            }

            println!("📋 Next Steps:");
            for step in &response.next_steps {
                println!("   {}", step);
            }
        }
        Err(e) => {
            println!("❌ Diagnosis failed: {}", e);
        }
    }

    println!();

    // Example 2: High memory usage
    println!("═══════════════════════════════════════════════════");
    println!("Example 2: High Memory Usage");
    println!("═══════════════════════════════════════════════════\n");

    let request2 = DiagnosticRequest {
        error: ErrorContext {
            message: "OutOfMemoryError: Java heap space".to_string(),
            error_type: "OutOfMemoryError".to_string(),
            source: "data-processor".to_string(),
            severity: "warning".to_string(),
            context: [
                ("heap_size".to_string(), "2GB".to_string()),
                ("used_memory".to_string(), "1.9GB".to_string()),
                ("gc_count".to_string(), "150".to_string()),
            ].into_iter().collect(),
            ..Default::default()
        },
        include_remediation: true,
        preferred_language: Some("java".to_string()),
        ..Default::default()
    };

    println!("Submitting diagnostic request...");
    match engine.diagnose(request2).await {
        Ok(response) => {
            println!("✅ Diagnosis complete!");
            println!("   Root Cause: {}", response.report.root_cause.primary_cause);
            println!();
            
            if !response.report.suggested_fixes.is_empty() {
                println!("💡 Top Fix: {}", response.report.suggested_fixes[0].title);
                println!("   Description: {}", response.report.suggested_fixes[0].description);
            }
        }
        Err(e) => {
            println!("❌ Diagnosis failed: {}", e);
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════");
    println!("Examples complete!");
    println!("═══════════════════════════════════════════════════");
    println!();
    println!("To use real AI analysis:");
    println!("  export OPENAI_API_KEY='sk-...'");
    println!();
    println!("To enable ticketing:");
    println!("  export JIRA_HOST='https://...'");
    println!("  export JIRA_USER='...'");
    println!("  export JIRA_TOKEN='...'");
    println!();
    println!("To enable GitHub issues:");
    println!("  export GITHUB_TOKEN='...'");
    println!("  export GITHUB_OWNER='...'");
    println!("  export GITHUB_REPO='...'");

    Ok(())
}

//! Simple Mesh Example
//! 
//! Demonstrates zero-code mesh deployment with configuration.
//!
//! ## Run
//!
//! ```bash
//! # With config file
//! FLUXX_MESH_CONFIG=./mesh.yaml cargo run --example simple_mesh
//!
//! # With environment variables only
//! FLUXX_NODE_ID=my-node FLUXX_MESH_BIND=0.0.0.0:7946 cargo run --example simple_mesh
//! ```

use fluxx_agent::mesh;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    println!("🚀 Starting Fluxx Agent Mesh Node\n");
    
    // Initialize mesh from configuration (file or environment)
    let mesh = mesh::init().await?;
    
    println!("✅ Mesh node started successfully!");
    println!("   Node ID: (see config)");
    println!("   Waiting for mesh connections...\n");
    
    // Wait for mesh to form (at least 2 nodes total, 30s timeout)
    let mesh_formed = mesh.wait_for_mesh(2, 30).await;
    
    if mesh_formed {
        println!("✅ Mesh formed with {} total nodes\n", mesh.size().await);
    } else {
        println!("⚠️  Mesh not fully formed (running standalone or peers not reachable)\n");
    }
    
    // Print mesh status
    mesh::print_status(&mesh).await;
    
    // Keep running until interrupted
    println!("Press Ctrl+C to shutdown...\n");
    
    tokio::signal::ctrl_c().await?;
    
    println!("\n🛑 Shutting down...");
    mesh.shutdown().await;
    
    println!("✅ Shutdown complete");
    Ok(())
}

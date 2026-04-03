use crate::core::traits::{Capturer, Processor, Transporter};
use crate::core::error::Result;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info};

pub struct Engine {
    capturers: Vec<Arc<dyn Capturer>>,
    processors: Vec<Arc<dyn Processor>>,
    transporters: Vec<Arc<dyn Transporter>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            capturers: Vec::new(),
            processors: Vec::new(),
            transporters: Vec::new(),
        }
    }

    pub fn add_capturer(&mut self, capturer: Arc<dyn Capturer>) {
        self.capturers.push(capturer);
    }

    pub fn add_processor(&mut self, processor: Arc<dyn Processor>) {
        self.processors.push(processor);
    }

    pub fn add_transporter(&mut self, transporter: Arc<dyn Transporter>) {
        self.transporters.push(transporter);
    }

    pub async fn run(self) -> Result<()> {
        info!("Fluxx engine starting...");
        let (tx, mut rx) = mpsc::channel(100);

        // Start capturers in their own tasks
        for capturer in self.capturers {
            let tx = tx.clone();
            tokio::spawn(async move {
                loop {
                    match capturer.capture().await {
                        Ok(event) => {
                            if let Err(e) = tx.send(event).await {
                                error!("Failed to send event to processing channel: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            error!("Capture error from {}: {}", capturer.name(), e);
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        }
                    }
                }
            });
        }

        // Processing and Transport loop
        while let Some(event) = rx.recv().await {
            let processors = self.processors.clone();
            let transporters = self.transporters.clone();

            tokio::spawn(async move {
                let mut processed_event = Some(event);

                // Run processors in sequence (Pipe-and-Filter)
                for processor in processors {
                    if let Some(ev) = processed_event {
                        match processor.process(ev).await {
                            Ok(res) => processed_event = res,
                            Err(e) => {
                                error!("Processor error from {}: {}", processor.name(), e);
                                processed_event = None; // Drop event on error? Or retry?
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }

                // Send to all transporters if still present
                if let Some(ev) = processed_event {
                    for transporter in transporters {
                        if let Err(e) = transporter.send(ev.clone()).await {
                            error!("Transport error from {}: {}", transporter.name(), e);
                        }
                    }
                }
            });
        }

        Ok(())
    }
}

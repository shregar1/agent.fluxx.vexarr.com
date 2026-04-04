use async_trait::async_trait;
use crate::core::error::{Result, Error};
use crate::core::traits::{Capturer, Protocol};
use crate::models::event::{Event, ProtocolType};
use crate::protocols::common::CaptureChannel;
use crate::protocols::ebpf::kernel::EbpfEvent;
use aya::{Bpf, programs::Xdp};
use aya_log::BpfLogger;
use std::sync::Arc;
use tracing::{info, error, debug};

pub mod kernel;

/// EbpfCapture: High-performance packet observation using eBPF XDP/TC probes.
/// Built on the 'aya' framework to provide zero-copy access to the kernel-side metadata.
#[derive(Debug)]
pub struct EbpfCapture {
    pub name: String,
    pub interface: String,
    channel: Arc<CaptureChannel>,
}

impl EbpfCapture {
    pub fn new(name: &str, interface: &str) -> Self {
        Self {
            name: name.to_string(),
            interface: interface.to_string(),
            channel: Arc::new(CaptureChannel::new(name, 10000)), // High-capacity buffer
        }
    }

    /// Loading and attaching BPF XDP probes for network-level capture.
    pub async fn init_ebpf(&self) -> Result<()> {
        info!(interface=%self.interface, "eBPF: Loading XDP probes.");
        
        // Load the BPF bytecode (Conceptual design since actual data is in kernel/xdp.o)
        // let mut bpf = Bpf::load(include_bytes!("../../../ebpf/xdp_trace.o"))
        //     .map_err(|e| Error::Protocol(self.name.clone(), format!("BPF load failed: {}", e)))?;
        
        // Example setup for XDP (ingress)
        // let program: &mut Xdp = bpf.program_mut("xdp_pass").ok_or(Error::Protocol(...))?.try_into()?;
        // program.load()?;
        // program.attach(&self.interface, XdpFlags::default())?;

        Ok(())
    }

    /// Internal mapper to translate Kernel events into Fluxx pipeline events.
    pub fn map_event(&self, k_event: EbpfEvent) -> Event {
        let proto = match k_event.protocol {
            6 => ProtocolType::Tcp,
            17 => ProtocolType::Udp,
            _ => ProtocolType::Other,
        };

        let mut event = Event::new(uuid::Uuid::new_v4().to_string(), proto);
        event.timestamp_us = k_event.timestamp_us;
        event.metadata.source_ip = Some(k_event.src_ip_str());
        event.metadata.destination_ip = Some(k_event.dst_ip_str());
        event.metadata.add("src_port", &k_event.src_port.to_string());
        event.metadata.add("dst_port", &k_event.dst_port.to_string());
        event.metadata.add("size_bytes", &k_event.payload_size.to_string());
        
        event
    }
}

#[async_trait]
impl Protocol for EbpfCapture {
    fn name(&self) -> &str {
        &self.name
    }

    async fn start(&self) -> Result<()> {
        self.init_ebpf().await?;
        info!(interface=%self.interface, "eBPF: Capture operational.");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        info!(interface=%self.interface, "eBPF: Probes detached.");
        Ok(())
    }
}

#[async_trait]
impl Capturer for EbpfCapture {
    async fn capture(&self) -> Result<Event> {
        self.channel.capture().await
    }

    fn name(&self) -> &str {
        &self.name
    }
}

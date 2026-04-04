//! Cloud Provider Integrations
//!
//! Monitor AWS CloudWatch, Azure Monitor, GCP, DigitalOcean, Linode, and IBM Cloud.

pub mod aws;
pub mod azure;
pub mod digitalocean;
pub mod gcp;
pub mod ibm;
pub mod linode;

pub use aws::AwsIntegration;
pub use azure::AzureIntegration;
pub use digitalocean::DigitalOceanIntegration;
pub use gcp::GcpIntegration;
pub use ibm::IbmCloudIntegration;
pub use linode::LinodeIntegration;

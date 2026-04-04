// Simple Event Tests 3 (4501-5000)

use fluxx_agent::models::event::{Event, ProtocolType, Metadata};

#[test]
fn test_metadata_new_4501() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4502() {
    let mut event = Event::new("evt-4502".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4502".to_string());
    assert_eq!(event.project_id, Some("proj-4502".to_string()));
}

#[test]
fn test_event_env_4503() {
    let mut event = Event::new("evt-4503".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4503".to_string());
    assert_eq!(event.environment_id, Some("env-4503".to_string()));
}

#[test]
fn test_event_service_4504() {
    let mut event = Event::new("evt-4504".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4504".to_string());
    assert_eq!(event.service_id, Some("svc-4504".to_string()));
}

#[test]
fn test_event_set_latency_4505() {
    let mut event = Event::new("evt-4505".to_string(), ProtocolType::Http);
    event.latency_us = 4505 as u64;
    assert_eq!(event.latency_us, 4505 as u64);
}

#[test]
fn test_metadata_new_4506() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4507() {
    let mut event = Event::new("evt-4507".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4507".to_string());
    assert_eq!(event.project_id, Some("proj-4507".to_string()));
}

#[test]
fn test_event_env_4508() {
    let mut event = Event::new("evt-4508".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4508".to_string());
    assert_eq!(event.environment_id, Some("env-4508".to_string()));
}

#[test]
fn test_event_service_4509() {
    let mut event = Event::new("evt-4509".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4509".to_string());
    assert_eq!(event.service_id, Some("svc-4509".to_string()));
}

#[test]
fn test_event_set_latency_4510() {
    let mut event = Event::new("evt-4510".to_string(), ProtocolType::Http);
    event.latency_us = 4510 as u64;
    assert_eq!(event.latency_us, 4510 as u64);
}

#[test]
fn test_metadata_new_4511() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4512() {
    let mut event = Event::new("evt-4512".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4512".to_string());
    assert_eq!(event.project_id, Some("proj-4512".to_string()));
}

#[test]
fn test_event_env_4513() {
    let mut event = Event::new("evt-4513".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4513".to_string());
    assert_eq!(event.environment_id, Some("env-4513".to_string()));
}

#[test]
fn test_event_service_4514() {
    let mut event = Event::new("evt-4514".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4514".to_string());
    assert_eq!(event.service_id, Some("svc-4514".to_string()));
}

#[test]
fn test_event_set_latency_4515() {
    let mut event = Event::new("evt-4515".to_string(), ProtocolType::Http);
    event.latency_us = 4515 as u64;
    assert_eq!(event.latency_us, 4515 as u64);
}

#[test]
fn test_metadata_new_4516() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4517() {
    let mut event = Event::new("evt-4517".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4517".to_string());
    assert_eq!(event.project_id, Some("proj-4517".to_string()));
}

#[test]
fn test_event_env_4518() {
    let mut event = Event::new("evt-4518".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4518".to_string());
    assert_eq!(event.environment_id, Some("env-4518".to_string()));
}

#[test]
fn test_event_service_4519() {
    let mut event = Event::new("evt-4519".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4519".to_string());
    assert_eq!(event.service_id, Some("svc-4519".to_string()));
}

#[test]
fn test_event_set_latency_4520() {
    let mut event = Event::new("evt-4520".to_string(), ProtocolType::Http);
    event.latency_us = 4520 as u64;
    assert_eq!(event.latency_us, 4520 as u64);
}

#[test]
fn test_metadata_new_4521() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4522() {
    let mut event = Event::new("evt-4522".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4522".to_string());
    assert_eq!(event.project_id, Some("proj-4522".to_string()));
}

#[test]
fn test_event_env_4523() {
    let mut event = Event::new("evt-4523".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4523".to_string());
    assert_eq!(event.environment_id, Some("env-4523".to_string()));
}

#[test]
fn test_event_service_4524() {
    let mut event = Event::new("evt-4524".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4524".to_string());
    assert_eq!(event.service_id, Some("svc-4524".to_string()));
}

#[test]
fn test_event_set_latency_4525() {
    let mut event = Event::new("evt-4525".to_string(), ProtocolType::Http);
    event.latency_us = 4525 as u64;
    assert_eq!(event.latency_us, 4525 as u64);
}

#[test]
fn test_metadata_new_4526() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4527() {
    let mut event = Event::new("evt-4527".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4527".to_string());
    assert_eq!(event.project_id, Some("proj-4527".to_string()));
}

#[test]
fn test_event_env_4528() {
    let mut event = Event::new("evt-4528".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4528".to_string());
    assert_eq!(event.environment_id, Some("env-4528".to_string()));
}

#[test]
fn test_event_service_4529() {
    let mut event = Event::new("evt-4529".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4529".to_string());
    assert_eq!(event.service_id, Some("svc-4529".to_string()));
}

#[test]
fn test_event_set_latency_4530() {
    let mut event = Event::new("evt-4530".to_string(), ProtocolType::Http);
    event.latency_us = 4530 as u64;
    assert_eq!(event.latency_us, 4530 as u64);
}

#[test]
fn test_metadata_new_4531() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4532() {
    let mut event = Event::new("evt-4532".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4532".to_string());
    assert_eq!(event.project_id, Some("proj-4532".to_string()));
}

#[test]
fn test_event_env_4533() {
    let mut event = Event::new("evt-4533".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4533".to_string());
    assert_eq!(event.environment_id, Some("env-4533".to_string()));
}

#[test]
fn test_event_service_4534() {
    let mut event = Event::new("evt-4534".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4534".to_string());
    assert_eq!(event.service_id, Some("svc-4534".to_string()));
}

#[test]
fn test_event_set_latency_4535() {
    let mut event = Event::new("evt-4535".to_string(), ProtocolType::Http);
    event.latency_us = 4535 as u64;
    assert_eq!(event.latency_us, 4535 as u64);
}

#[test]
fn test_metadata_new_4536() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4537() {
    let mut event = Event::new("evt-4537".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4537".to_string());
    assert_eq!(event.project_id, Some("proj-4537".to_string()));
}

#[test]
fn test_event_env_4538() {
    let mut event = Event::new("evt-4538".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4538".to_string());
    assert_eq!(event.environment_id, Some("env-4538".to_string()));
}

#[test]
fn test_event_service_4539() {
    let mut event = Event::new("evt-4539".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4539".to_string());
    assert_eq!(event.service_id, Some("svc-4539".to_string()));
}

#[test]
fn test_event_set_latency_4540() {
    let mut event = Event::new("evt-4540".to_string(), ProtocolType::Http);
    event.latency_us = 4540 as u64;
    assert_eq!(event.latency_us, 4540 as u64);
}

#[test]
fn test_metadata_new_4541() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4542() {
    let mut event = Event::new("evt-4542".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4542".to_string());
    assert_eq!(event.project_id, Some("proj-4542".to_string()));
}

#[test]
fn test_event_env_4543() {
    let mut event = Event::new("evt-4543".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4543".to_string());
    assert_eq!(event.environment_id, Some("env-4543".to_string()));
}

#[test]
fn test_event_service_4544() {
    let mut event = Event::new("evt-4544".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4544".to_string());
    assert_eq!(event.service_id, Some("svc-4544".to_string()));
}

#[test]
fn test_event_set_latency_4545() {
    let mut event = Event::new("evt-4545".to_string(), ProtocolType::Http);
    event.latency_us = 4545 as u64;
    assert_eq!(event.latency_us, 4545 as u64);
}

#[test]
fn test_metadata_new_4546() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4547() {
    let mut event = Event::new("evt-4547".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4547".to_string());
    assert_eq!(event.project_id, Some("proj-4547".to_string()));
}

#[test]
fn test_event_env_4548() {
    let mut event = Event::new("evt-4548".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4548".to_string());
    assert_eq!(event.environment_id, Some("env-4548".to_string()));
}

#[test]
fn test_event_service_4549() {
    let mut event = Event::new("evt-4549".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4549".to_string());
    assert_eq!(event.service_id, Some("svc-4549".to_string()));
}

#[test]
fn test_event_set_latency_4550() {
    let mut event = Event::new("evt-4550".to_string(), ProtocolType::Http);
    event.latency_us = 4550 as u64;
    assert_eq!(event.latency_us, 4550 as u64);
}

#[test]
fn test_metadata_new_4551() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4552() {
    let mut event = Event::new("evt-4552".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4552".to_string());
    assert_eq!(event.project_id, Some("proj-4552".to_string()));
}

#[test]
fn test_event_env_4553() {
    let mut event = Event::new("evt-4553".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4553".to_string());
    assert_eq!(event.environment_id, Some("env-4553".to_string()));
}

#[test]
fn test_event_service_4554() {
    let mut event = Event::new("evt-4554".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4554".to_string());
    assert_eq!(event.service_id, Some("svc-4554".to_string()));
}

#[test]
fn test_event_set_latency_4555() {
    let mut event = Event::new("evt-4555".to_string(), ProtocolType::Http);
    event.latency_us = 4555 as u64;
    assert_eq!(event.latency_us, 4555 as u64);
}

#[test]
fn test_metadata_new_4556() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4557() {
    let mut event = Event::new("evt-4557".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4557".to_string());
    assert_eq!(event.project_id, Some("proj-4557".to_string()));
}

#[test]
fn test_event_env_4558() {
    let mut event = Event::new("evt-4558".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4558".to_string());
    assert_eq!(event.environment_id, Some("env-4558".to_string()));
}

#[test]
fn test_event_service_4559() {
    let mut event = Event::new("evt-4559".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4559".to_string());
    assert_eq!(event.service_id, Some("svc-4559".to_string()));
}

#[test]
fn test_event_set_latency_4560() {
    let mut event = Event::new("evt-4560".to_string(), ProtocolType::Http);
    event.latency_us = 4560 as u64;
    assert_eq!(event.latency_us, 4560 as u64);
}

#[test]
fn test_metadata_new_4561() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4562() {
    let mut event = Event::new("evt-4562".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4562".to_string());
    assert_eq!(event.project_id, Some("proj-4562".to_string()));
}

#[test]
fn test_event_env_4563() {
    let mut event = Event::new("evt-4563".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4563".to_string());
    assert_eq!(event.environment_id, Some("env-4563".to_string()));
}

#[test]
fn test_event_service_4564() {
    let mut event = Event::new("evt-4564".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4564".to_string());
    assert_eq!(event.service_id, Some("svc-4564".to_string()));
}

#[test]
fn test_event_set_latency_4565() {
    let mut event = Event::new("evt-4565".to_string(), ProtocolType::Http);
    event.latency_us = 4565 as u64;
    assert_eq!(event.latency_us, 4565 as u64);
}

#[test]
fn test_metadata_new_4566() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4567() {
    let mut event = Event::new("evt-4567".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4567".to_string());
    assert_eq!(event.project_id, Some("proj-4567".to_string()));
}

#[test]
fn test_event_env_4568() {
    let mut event = Event::new("evt-4568".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4568".to_string());
    assert_eq!(event.environment_id, Some("env-4568".to_string()));
}

#[test]
fn test_event_service_4569() {
    let mut event = Event::new("evt-4569".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4569".to_string());
    assert_eq!(event.service_id, Some("svc-4569".to_string()));
}

#[test]
fn test_event_set_latency_4570() {
    let mut event = Event::new("evt-4570".to_string(), ProtocolType::Http);
    event.latency_us = 4570 as u64;
    assert_eq!(event.latency_us, 4570 as u64);
}

#[test]
fn test_metadata_new_4571() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4572() {
    let mut event = Event::new("evt-4572".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4572".to_string());
    assert_eq!(event.project_id, Some("proj-4572".to_string()));
}

#[test]
fn test_event_env_4573() {
    let mut event = Event::new("evt-4573".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4573".to_string());
    assert_eq!(event.environment_id, Some("env-4573".to_string()));
}

#[test]
fn test_event_service_4574() {
    let mut event = Event::new("evt-4574".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4574".to_string());
    assert_eq!(event.service_id, Some("svc-4574".to_string()));
}

#[test]
fn test_event_set_latency_4575() {
    let mut event = Event::new("evt-4575".to_string(), ProtocolType::Http);
    event.latency_us = 4575 as u64;
    assert_eq!(event.latency_us, 4575 as u64);
}

#[test]
fn test_metadata_new_4576() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4577() {
    let mut event = Event::new("evt-4577".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4577".to_string());
    assert_eq!(event.project_id, Some("proj-4577".to_string()));
}

#[test]
fn test_event_env_4578() {
    let mut event = Event::new("evt-4578".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4578".to_string());
    assert_eq!(event.environment_id, Some("env-4578".to_string()));
}

#[test]
fn test_event_service_4579() {
    let mut event = Event::new("evt-4579".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4579".to_string());
    assert_eq!(event.service_id, Some("svc-4579".to_string()));
}

#[test]
fn test_event_set_latency_4580() {
    let mut event = Event::new("evt-4580".to_string(), ProtocolType::Http);
    event.latency_us = 4580 as u64;
    assert_eq!(event.latency_us, 4580 as u64);
}

#[test]
fn test_metadata_new_4581() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4582() {
    let mut event = Event::new("evt-4582".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4582".to_string());
    assert_eq!(event.project_id, Some("proj-4582".to_string()));
}

#[test]
fn test_event_env_4583() {
    let mut event = Event::new("evt-4583".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4583".to_string());
    assert_eq!(event.environment_id, Some("env-4583".to_string()));
}

#[test]
fn test_event_service_4584() {
    let mut event = Event::new("evt-4584".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4584".to_string());
    assert_eq!(event.service_id, Some("svc-4584".to_string()));
}

#[test]
fn test_event_set_latency_4585() {
    let mut event = Event::new("evt-4585".to_string(), ProtocolType::Http);
    event.latency_us = 4585 as u64;
    assert_eq!(event.latency_us, 4585 as u64);
}

#[test]
fn test_metadata_new_4586() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4587() {
    let mut event = Event::new("evt-4587".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4587".to_string());
    assert_eq!(event.project_id, Some("proj-4587".to_string()));
}

#[test]
fn test_event_env_4588() {
    let mut event = Event::new("evt-4588".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4588".to_string());
    assert_eq!(event.environment_id, Some("env-4588".to_string()));
}

#[test]
fn test_event_service_4589() {
    let mut event = Event::new("evt-4589".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4589".to_string());
    assert_eq!(event.service_id, Some("svc-4589".to_string()));
}

#[test]
fn test_event_set_latency_4590() {
    let mut event = Event::new("evt-4590".to_string(), ProtocolType::Http);
    event.latency_us = 4590 as u64;
    assert_eq!(event.latency_us, 4590 as u64);
}

#[test]
fn test_metadata_new_4591() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4592() {
    let mut event = Event::new("evt-4592".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4592".to_string());
    assert_eq!(event.project_id, Some("proj-4592".to_string()));
}

#[test]
fn test_event_env_4593() {
    let mut event = Event::new("evt-4593".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4593".to_string());
    assert_eq!(event.environment_id, Some("env-4593".to_string()));
}

#[test]
fn test_event_service_4594() {
    let mut event = Event::new("evt-4594".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4594".to_string());
    assert_eq!(event.service_id, Some("svc-4594".to_string()));
}

#[test]
fn test_event_set_latency_4595() {
    let mut event = Event::new("evt-4595".to_string(), ProtocolType::Http);
    event.latency_us = 4595 as u64;
    assert_eq!(event.latency_us, 4595 as u64);
}

#[test]
fn test_metadata_new_4596() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4597() {
    let mut event = Event::new("evt-4597".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4597".to_string());
    assert_eq!(event.project_id, Some("proj-4597".to_string()));
}

#[test]
fn test_event_env_4598() {
    let mut event = Event::new("evt-4598".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4598".to_string());
    assert_eq!(event.environment_id, Some("env-4598".to_string()));
}

#[test]
fn test_event_service_4599() {
    let mut event = Event::new("evt-4599".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4599".to_string());
    assert_eq!(event.service_id, Some("svc-4599".to_string()));
}

#[test]
fn test_event_set_latency_4600() {
    let mut event = Event::new("evt-4600".to_string(), ProtocolType::Http);
    event.latency_us = 4600 as u64;
    assert_eq!(event.latency_us, 4600 as u64);
}

#[test]
fn test_metadata_new_4601() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4602() {
    let mut event = Event::new("evt-4602".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4602".to_string());
    assert_eq!(event.project_id, Some("proj-4602".to_string()));
}

#[test]
fn test_event_env_4603() {
    let mut event = Event::new("evt-4603".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4603".to_string());
    assert_eq!(event.environment_id, Some("env-4603".to_string()));
}

#[test]
fn test_event_service_4604() {
    let mut event = Event::new("evt-4604".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4604".to_string());
    assert_eq!(event.service_id, Some("svc-4604".to_string()));
}

#[test]
fn test_event_set_latency_4605() {
    let mut event = Event::new("evt-4605".to_string(), ProtocolType::Http);
    event.latency_us = 4605 as u64;
    assert_eq!(event.latency_us, 4605 as u64);
}

#[test]
fn test_metadata_new_4606() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4607() {
    let mut event = Event::new("evt-4607".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4607".to_string());
    assert_eq!(event.project_id, Some("proj-4607".to_string()));
}

#[test]
fn test_event_env_4608() {
    let mut event = Event::new("evt-4608".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4608".to_string());
    assert_eq!(event.environment_id, Some("env-4608".to_string()));
}

#[test]
fn test_event_service_4609() {
    let mut event = Event::new("evt-4609".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4609".to_string());
    assert_eq!(event.service_id, Some("svc-4609".to_string()));
}

#[test]
fn test_event_set_latency_4610() {
    let mut event = Event::new("evt-4610".to_string(), ProtocolType::Http);
    event.latency_us = 4610 as u64;
    assert_eq!(event.latency_us, 4610 as u64);
}

#[test]
fn test_metadata_new_4611() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4612() {
    let mut event = Event::new("evt-4612".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4612".to_string());
    assert_eq!(event.project_id, Some("proj-4612".to_string()));
}

#[test]
fn test_event_env_4613() {
    let mut event = Event::new("evt-4613".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4613".to_string());
    assert_eq!(event.environment_id, Some("env-4613".to_string()));
}

#[test]
fn test_event_service_4614() {
    let mut event = Event::new("evt-4614".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4614".to_string());
    assert_eq!(event.service_id, Some("svc-4614".to_string()));
}

#[test]
fn test_event_set_latency_4615() {
    let mut event = Event::new("evt-4615".to_string(), ProtocolType::Http);
    event.latency_us = 4615 as u64;
    assert_eq!(event.latency_us, 4615 as u64);
}

#[test]
fn test_metadata_new_4616() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4617() {
    let mut event = Event::new("evt-4617".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4617".to_string());
    assert_eq!(event.project_id, Some("proj-4617".to_string()));
}

#[test]
fn test_event_env_4618() {
    let mut event = Event::new("evt-4618".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4618".to_string());
    assert_eq!(event.environment_id, Some("env-4618".to_string()));
}

#[test]
fn test_event_service_4619() {
    let mut event = Event::new("evt-4619".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4619".to_string());
    assert_eq!(event.service_id, Some("svc-4619".to_string()));
}

#[test]
fn test_event_set_latency_4620() {
    let mut event = Event::new("evt-4620".to_string(), ProtocolType::Http);
    event.latency_us = 4620 as u64;
    assert_eq!(event.latency_us, 4620 as u64);
}

#[test]
fn test_metadata_new_4621() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4622() {
    let mut event = Event::new("evt-4622".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4622".to_string());
    assert_eq!(event.project_id, Some("proj-4622".to_string()));
}

#[test]
fn test_event_env_4623() {
    let mut event = Event::new("evt-4623".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4623".to_string());
    assert_eq!(event.environment_id, Some("env-4623".to_string()));
}

#[test]
fn test_event_service_4624() {
    let mut event = Event::new("evt-4624".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4624".to_string());
    assert_eq!(event.service_id, Some("svc-4624".to_string()));
}

#[test]
fn test_event_set_latency_4625() {
    let mut event = Event::new("evt-4625".to_string(), ProtocolType::Http);
    event.latency_us = 4625 as u64;
    assert_eq!(event.latency_us, 4625 as u64);
}

#[test]
fn test_metadata_new_4626() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4627() {
    let mut event = Event::new("evt-4627".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4627".to_string());
    assert_eq!(event.project_id, Some("proj-4627".to_string()));
}

#[test]
fn test_event_env_4628() {
    let mut event = Event::new("evt-4628".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4628".to_string());
    assert_eq!(event.environment_id, Some("env-4628".to_string()));
}

#[test]
fn test_event_service_4629() {
    let mut event = Event::new("evt-4629".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4629".to_string());
    assert_eq!(event.service_id, Some("svc-4629".to_string()));
}

#[test]
fn test_event_set_latency_4630() {
    let mut event = Event::new("evt-4630".to_string(), ProtocolType::Http);
    event.latency_us = 4630 as u64;
    assert_eq!(event.latency_us, 4630 as u64);
}

#[test]
fn test_metadata_new_4631() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4632() {
    let mut event = Event::new("evt-4632".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4632".to_string());
    assert_eq!(event.project_id, Some("proj-4632".to_string()));
}

#[test]
fn test_event_env_4633() {
    let mut event = Event::new("evt-4633".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4633".to_string());
    assert_eq!(event.environment_id, Some("env-4633".to_string()));
}

#[test]
fn test_event_service_4634() {
    let mut event = Event::new("evt-4634".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4634".to_string());
    assert_eq!(event.service_id, Some("svc-4634".to_string()));
}

#[test]
fn test_event_set_latency_4635() {
    let mut event = Event::new("evt-4635".to_string(), ProtocolType::Http);
    event.latency_us = 4635 as u64;
    assert_eq!(event.latency_us, 4635 as u64);
}

#[test]
fn test_metadata_new_4636() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4637() {
    let mut event = Event::new("evt-4637".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4637".to_string());
    assert_eq!(event.project_id, Some("proj-4637".to_string()));
}

#[test]
fn test_event_env_4638() {
    let mut event = Event::new("evt-4638".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4638".to_string());
    assert_eq!(event.environment_id, Some("env-4638".to_string()));
}

#[test]
fn test_event_service_4639() {
    let mut event = Event::new("evt-4639".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4639".to_string());
    assert_eq!(event.service_id, Some("svc-4639".to_string()));
}

#[test]
fn test_event_set_latency_4640() {
    let mut event = Event::new("evt-4640".to_string(), ProtocolType::Http);
    event.latency_us = 4640 as u64;
    assert_eq!(event.latency_us, 4640 as u64);
}

#[test]
fn test_metadata_new_4641() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4642() {
    let mut event = Event::new("evt-4642".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4642".to_string());
    assert_eq!(event.project_id, Some("proj-4642".to_string()));
}

#[test]
fn test_event_env_4643() {
    let mut event = Event::new("evt-4643".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4643".to_string());
    assert_eq!(event.environment_id, Some("env-4643".to_string()));
}

#[test]
fn test_event_service_4644() {
    let mut event = Event::new("evt-4644".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4644".to_string());
    assert_eq!(event.service_id, Some("svc-4644".to_string()));
}

#[test]
fn test_event_set_latency_4645() {
    let mut event = Event::new("evt-4645".to_string(), ProtocolType::Http);
    event.latency_us = 4645 as u64;
    assert_eq!(event.latency_us, 4645 as u64);
}

#[test]
fn test_metadata_new_4646() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4647() {
    let mut event = Event::new("evt-4647".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4647".to_string());
    assert_eq!(event.project_id, Some("proj-4647".to_string()));
}

#[test]
fn test_event_env_4648() {
    let mut event = Event::new("evt-4648".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4648".to_string());
    assert_eq!(event.environment_id, Some("env-4648".to_string()));
}

#[test]
fn test_event_service_4649() {
    let mut event = Event::new("evt-4649".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4649".to_string());
    assert_eq!(event.service_id, Some("svc-4649".to_string()));
}

#[test]
fn test_event_set_latency_4650() {
    let mut event = Event::new("evt-4650".to_string(), ProtocolType::Http);
    event.latency_us = 4650 as u64;
    assert_eq!(event.latency_us, 4650 as u64);
}

#[test]
fn test_metadata_new_4651() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4652() {
    let mut event = Event::new("evt-4652".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4652".to_string());
    assert_eq!(event.project_id, Some("proj-4652".to_string()));
}

#[test]
fn test_event_env_4653() {
    let mut event = Event::new("evt-4653".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4653".to_string());
    assert_eq!(event.environment_id, Some("env-4653".to_string()));
}

#[test]
fn test_event_service_4654() {
    let mut event = Event::new("evt-4654".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4654".to_string());
    assert_eq!(event.service_id, Some("svc-4654".to_string()));
}

#[test]
fn test_event_set_latency_4655() {
    let mut event = Event::new("evt-4655".to_string(), ProtocolType::Http);
    event.latency_us = 4655 as u64;
    assert_eq!(event.latency_us, 4655 as u64);
}

#[test]
fn test_metadata_new_4656() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4657() {
    let mut event = Event::new("evt-4657".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4657".to_string());
    assert_eq!(event.project_id, Some("proj-4657".to_string()));
}

#[test]
fn test_event_env_4658() {
    let mut event = Event::new("evt-4658".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4658".to_string());
    assert_eq!(event.environment_id, Some("env-4658".to_string()));
}

#[test]
fn test_event_service_4659() {
    let mut event = Event::new("evt-4659".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4659".to_string());
    assert_eq!(event.service_id, Some("svc-4659".to_string()));
}

#[test]
fn test_event_set_latency_4660() {
    let mut event = Event::new("evt-4660".to_string(), ProtocolType::Http);
    event.latency_us = 4660 as u64;
    assert_eq!(event.latency_us, 4660 as u64);
}

#[test]
fn test_metadata_new_4661() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4662() {
    let mut event = Event::new("evt-4662".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4662".to_string());
    assert_eq!(event.project_id, Some("proj-4662".to_string()));
}

#[test]
fn test_event_env_4663() {
    let mut event = Event::new("evt-4663".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4663".to_string());
    assert_eq!(event.environment_id, Some("env-4663".to_string()));
}

#[test]
fn test_event_service_4664() {
    let mut event = Event::new("evt-4664".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4664".to_string());
    assert_eq!(event.service_id, Some("svc-4664".to_string()));
}

#[test]
fn test_event_set_latency_4665() {
    let mut event = Event::new("evt-4665".to_string(), ProtocolType::Http);
    event.latency_us = 4665 as u64;
    assert_eq!(event.latency_us, 4665 as u64);
}

#[test]
fn test_metadata_new_4666() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4667() {
    let mut event = Event::new("evt-4667".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4667".to_string());
    assert_eq!(event.project_id, Some("proj-4667".to_string()));
}

#[test]
fn test_event_env_4668() {
    let mut event = Event::new("evt-4668".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4668".to_string());
    assert_eq!(event.environment_id, Some("env-4668".to_string()));
}

#[test]
fn test_event_service_4669() {
    let mut event = Event::new("evt-4669".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4669".to_string());
    assert_eq!(event.service_id, Some("svc-4669".to_string()));
}

#[test]
fn test_event_set_latency_4670() {
    let mut event = Event::new("evt-4670".to_string(), ProtocolType::Http);
    event.latency_us = 4670 as u64;
    assert_eq!(event.latency_us, 4670 as u64);
}

#[test]
fn test_metadata_new_4671() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4672() {
    let mut event = Event::new("evt-4672".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4672".to_string());
    assert_eq!(event.project_id, Some("proj-4672".to_string()));
}

#[test]
fn test_event_env_4673() {
    let mut event = Event::new("evt-4673".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4673".to_string());
    assert_eq!(event.environment_id, Some("env-4673".to_string()));
}

#[test]
fn test_event_service_4674() {
    let mut event = Event::new("evt-4674".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4674".to_string());
    assert_eq!(event.service_id, Some("svc-4674".to_string()));
}

#[test]
fn test_event_set_latency_4675() {
    let mut event = Event::new("evt-4675".to_string(), ProtocolType::Http);
    event.latency_us = 4675 as u64;
    assert_eq!(event.latency_us, 4675 as u64);
}

#[test]
fn test_metadata_new_4676() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4677() {
    let mut event = Event::new("evt-4677".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4677".to_string());
    assert_eq!(event.project_id, Some("proj-4677".to_string()));
}

#[test]
fn test_event_env_4678() {
    let mut event = Event::new("evt-4678".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4678".to_string());
    assert_eq!(event.environment_id, Some("env-4678".to_string()));
}

#[test]
fn test_event_service_4679() {
    let mut event = Event::new("evt-4679".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4679".to_string());
    assert_eq!(event.service_id, Some("svc-4679".to_string()));
}

#[test]
fn test_event_set_latency_4680() {
    let mut event = Event::new("evt-4680".to_string(), ProtocolType::Http);
    event.latency_us = 4680 as u64;
    assert_eq!(event.latency_us, 4680 as u64);
}

#[test]
fn test_metadata_new_4681() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4682() {
    let mut event = Event::new("evt-4682".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4682".to_string());
    assert_eq!(event.project_id, Some("proj-4682".to_string()));
}

#[test]
fn test_event_env_4683() {
    let mut event = Event::new("evt-4683".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4683".to_string());
    assert_eq!(event.environment_id, Some("env-4683".to_string()));
}

#[test]
fn test_event_service_4684() {
    let mut event = Event::new("evt-4684".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4684".to_string());
    assert_eq!(event.service_id, Some("svc-4684".to_string()));
}

#[test]
fn test_event_set_latency_4685() {
    let mut event = Event::new("evt-4685".to_string(), ProtocolType::Http);
    event.latency_us = 4685 as u64;
    assert_eq!(event.latency_us, 4685 as u64);
}

#[test]
fn test_metadata_new_4686() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4687() {
    let mut event = Event::new("evt-4687".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4687".to_string());
    assert_eq!(event.project_id, Some("proj-4687".to_string()));
}

#[test]
fn test_event_env_4688() {
    let mut event = Event::new("evt-4688".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4688".to_string());
    assert_eq!(event.environment_id, Some("env-4688".to_string()));
}

#[test]
fn test_event_service_4689() {
    let mut event = Event::new("evt-4689".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4689".to_string());
    assert_eq!(event.service_id, Some("svc-4689".to_string()));
}

#[test]
fn test_event_set_latency_4690() {
    let mut event = Event::new("evt-4690".to_string(), ProtocolType::Http);
    event.latency_us = 4690 as u64;
    assert_eq!(event.latency_us, 4690 as u64);
}

#[test]
fn test_metadata_new_4691() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4692() {
    let mut event = Event::new("evt-4692".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4692".to_string());
    assert_eq!(event.project_id, Some("proj-4692".to_string()));
}

#[test]
fn test_event_env_4693() {
    let mut event = Event::new("evt-4693".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4693".to_string());
    assert_eq!(event.environment_id, Some("env-4693".to_string()));
}

#[test]
fn test_event_service_4694() {
    let mut event = Event::new("evt-4694".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4694".to_string());
    assert_eq!(event.service_id, Some("svc-4694".to_string()));
}

#[test]
fn test_event_set_latency_4695() {
    let mut event = Event::new("evt-4695".to_string(), ProtocolType::Http);
    event.latency_us = 4695 as u64;
    assert_eq!(event.latency_us, 4695 as u64);
}

#[test]
fn test_metadata_new_4696() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4697() {
    let mut event = Event::new("evt-4697".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4697".to_string());
    assert_eq!(event.project_id, Some("proj-4697".to_string()));
}

#[test]
fn test_event_env_4698() {
    let mut event = Event::new("evt-4698".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4698".to_string());
    assert_eq!(event.environment_id, Some("env-4698".to_string()));
}

#[test]
fn test_event_service_4699() {
    let mut event = Event::new("evt-4699".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4699".to_string());
    assert_eq!(event.service_id, Some("svc-4699".to_string()));
}

#[test]
fn test_event_set_latency_4700() {
    let mut event = Event::new("evt-4700".to_string(), ProtocolType::Http);
    event.latency_us = 4700 as u64;
    assert_eq!(event.latency_us, 4700 as u64);
}

#[test]
fn test_metadata_new_4701() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4702() {
    let mut event = Event::new("evt-4702".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4702".to_string());
    assert_eq!(event.project_id, Some("proj-4702".to_string()));
}

#[test]
fn test_event_env_4703() {
    let mut event = Event::new("evt-4703".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4703".to_string());
    assert_eq!(event.environment_id, Some("env-4703".to_string()));
}

#[test]
fn test_event_service_4704() {
    let mut event = Event::new("evt-4704".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4704".to_string());
    assert_eq!(event.service_id, Some("svc-4704".to_string()));
}

#[test]
fn test_event_set_latency_4705() {
    let mut event = Event::new("evt-4705".to_string(), ProtocolType::Http);
    event.latency_us = 4705 as u64;
    assert_eq!(event.latency_us, 4705 as u64);
}

#[test]
fn test_metadata_new_4706() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4707() {
    let mut event = Event::new("evt-4707".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4707".to_string());
    assert_eq!(event.project_id, Some("proj-4707".to_string()));
}

#[test]
fn test_event_env_4708() {
    let mut event = Event::new("evt-4708".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4708".to_string());
    assert_eq!(event.environment_id, Some("env-4708".to_string()));
}

#[test]
fn test_event_service_4709() {
    let mut event = Event::new("evt-4709".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4709".to_string());
    assert_eq!(event.service_id, Some("svc-4709".to_string()));
}

#[test]
fn test_event_set_latency_4710() {
    let mut event = Event::new("evt-4710".to_string(), ProtocolType::Http);
    event.latency_us = 4710 as u64;
    assert_eq!(event.latency_us, 4710 as u64);
}

#[test]
fn test_metadata_new_4711() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4712() {
    let mut event = Event::new("evt-4712".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4712".to_string());
    assert_eq!(event.project_id, Some("proj-4712".to_string()));
}

#[test]
fn test_event_env_4713() {
    let mut event = Event::new("evt-4713".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4713".to_string());
    assert_eq!(event.environment_id, Some("env-4713".to_string()));
}

#[test]
fn test_event_service_4714() {
    let mut event = Event::new("evt-4714".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4714".to_string());
    assert_eq!(event.service_id, Some("svc-4714".to_string()));
}

#[test]
fn test_event_set_latency_4715() {
    let mut event = Event::new("evt-4715".to_string(), ProtocolType::Http);
    event.latency_us = 4715 as u64;
    assert_eq!(event.latency_us, 4715 as u64);
}

#[test]
fn test_metadata_new_4716() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4717() {
    let mut event = Event::new("evt-4717".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4717".to_string());
    assert_eq!(event.project_id, Some("proj-4717".to_string()));
}

#[test]
fn test_event_env_4718() {
    let mut event = Event::new("evt-4718".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4718".to_string());
    assert_eq!(event.environment_id, Some("env-4718".to_string()));
}

#[test]
fn test_event_service_4719() {
    let mut event = Event::new("evt-4719".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4719".to_string());
    assert_eq!(event.service_id, Some("svc-4719".to_string()));
}

#[test]
fn test_event_set_latency_4720() {
    let mut event = Event::new("evt-4720".to_string(), ProtocolType::Http);
    event.latency_us = 4720 as u64;
    assert_eq!(event.latency_us, 4720 as u64);
}

#[test]
fn test_metadata_new_4721() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4722() {
    let mut event = Event::new("evt-4722".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4722".to_string());
    assert_eq!(event.project_id, Some("proj-4722".to_string()));
}

#[test]
fn test_event_env_4723() {
    let mut event = Event::new("evt-4723".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4723".to_string());
    assert_eq!(event.environment_id, Some("env-4723".to_string()));
}

#[test]
fn test_event_service_4724() {
    let mut event = Event::new("evt-4724".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4724".to_string());
    assert_eq!(event.service_id, Some("svc-4724".to_string()));
}

#[test]
fn test_event_set_latency_4725() {
    let mut event = Event::new("evt-4725".to_string(), ProtocolType::Http);
    event.latency_us = 4725 as u64;
    assert_eq!(event.latency_us, 4725 as u64);
}

#[test]
fn test_metadata_new_4726() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4727() {
    let mut event = Event::new("evt-4727".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4727".to_string());
    assert_eq!(event.project_id, Some("proj-4727".to_string()));
}

#[test]
fn test_event_env_4728() {
    let mut event = Event::new("evt-4728".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4728".to_string());
    assert_eq!(event.environment_id, Some("env-4728".to_string()));
}

#[test]
fn test_event_service_4729() {
    let mut event = Event::new("evt-4729".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4729".to_string());
    assert_eq!(event.service_id, Some("svc-4729".to_string()));
}

#[test]
fn test_event_set_latency_4730() {
    let mut event = Event::new("evt-4730".to_string(), ProtocolType::Http);
    event.latency_us = 4730 as u64;
    assert_eq!(event.latency_us, 4730 as u64);
}

#[test]
fn test_metadata_new_4731() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4732() {
    let mut event = Event::new("evt-4732".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4732".to_string());
    assert_eq!(event.project_id, Some("proj-4732".to_string()));
}

#[test]
fn test_event_env_4733() {
    let mut event = Event::new("evt-4733".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4733".to_string());
    assert_eq!(event.environment_id, Some("env-4733".to_string()));
}

#[test]
fn test_event_service_4734() {
    let mut event = Event::new("evt-4734".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4734".to_string());
    assert_eq!(event.service_id, Some("svc-4734".to_string()));
}

#[test]
fn test_event_set_latency_4735() {
    let mut event = Event::new("evt-4735".to_string(), ProtocolType::Http);
    event.latency_us = 4735 as u64;
    assert_eq!(event.latency_us, 4735 as u64);
}

#[test]
fn test_metadata_new_4736() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4737() {
    let mut event = Event::new("evt-4737".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4737".to_string());
    assert_eq!(event.project_id, Some("proj-4737".to_string()));
}

#[test]
fn test_event_env_4738() {
    let mut event = Event::new("evt-4738".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4738".to_string());
    assert_eq!(event.environment_id, Some("env-4738".to_string()));
}

#[test]
fn test_event_service_4739() {
    let mut event = Event::new("evt-4739".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4739".to_string());
    assert_eq!(event.service_id, Some("svc-4739".to_string()));
}

#[test]
fn test_event_set_latency_4740() {
    let mut event = Event::new("evt-4740".to_string(), ProtocolType::Http);
    event.latency_us = 4740 as u64;
    assert_eq!(event.latency_us, 4740 as u64);
}

#[test]
fn test_metadata_new_4741() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4742() {
    let mut event = Event::new("evt-4742".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4742".to_string());
    assert_eq!(event.project_id, Some("proj-4742".to_string()));
}

#[test]
fn test_event_env_4743() {
    let mut event = Event::new("evt-4743".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4743".to_string());
    assert_eq!(event.environment_id, Some("env-4743".to_string()));
}

#[test]
fn test_event_service_4744() {
    let mut event = Event::new("evt-4744".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4744".to_string());
    assert_eq!(event.service_id, Some("svc-4744".to_string()));
}

#[test]
fn test_event_set_latency_4745() {
    let mut event = Event::new("evt-4745".to_string(), ProtocolType::Http);
    event.latency_us = 4745 as u64;
    assert_eq!(event.latency_us, 4745 as u64);
}

#[test]
fn test_metadata_new_4746() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4747() {
    let mut event = Event::new("evt-4747".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4747".to_string());
    assert_eq!(event.project_id, Some("proj-4747".to_string()));
}

#[test]
fn test_event_env_4748() {
    let mut event = Event::new("evt-4748".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4748".to_string());
    assert_eq!(event.environment_id, Some("env-4748".to_string()));
}

#[test]
fn test_event_service_4749() {
    let mut event = Event::new("evt-4749".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4749".to_string());
    assert_eq!(event.service_id, Some("svc-4749".to_string()));
}

#[test]
fn test_event_set_latency_4750() {
    let mut event = Event::new("evt-4750".to_string(), ProtocolType::Http);
    event.latency_us = 4750 as u64;
    assert_eq!(event.latency_us, 4750 as u64);
}

#[test]
fn test_metadata_new_4751() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4752() {
    let mut event = Event::new("evt-4752".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4752".to_string());
    assert_eq!(event.project_id, Some("proj-4752".to_string()));
}

#[test]
fn test_event_env_4753() {
    let mut event = Event::new("evt-4753".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4753".to_string());
    assert_eq!(event.environment_id, Some("env-4753".to_string()));
}

#[test]
fn test_event_service_4754() {
    let mut event = Event::new("evt-4754".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4754".to_string());
    assert_eq!(event.service_id, Some("svc-4754".to_string()));
}

#[test]
fn test_event_set_latency_4755() {
    let mut event = Event::new("evt-4755".to_string(), ProtocolType::Http);
    event.latency_us = 4755 as u64;
    assert_eq!(event.latency_us, 4755 as u64);
}

#[test]
fn test_metadata_new_4756() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4757() {
    let mut event = Event::new("evt-4757".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4757".to_string());
    assert_eq!(event.project_id, Some("proj-4757".to_string()));
}

#[test]
fn test_event_env_4758() {
    let mut event = Event::new("evt-4758".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4758".to_string());
    assert_eq!(event.environment_id, Some("env-4758".to_string()));
}

#[test]
fn test_event_service_4759() {
    let mut event = Event::new("evt-4759".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4759".to_string());
    assert_eq!(event.service_id, Some("svc-4759".to_string()));
}

#[test]
fn test_event_set_latency_4760() {
    let mut event = Event::new("evt-4760".to_string(), ProtocolType::Http);
    event.latency_us = 4760 as u64;
    assert_eq!(event.latency_us, 4760 as u64);
}

#[test]
fn test_metadata_new_4761() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4762() {
    let mut event = Event::new("evt-4762".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4762".to_string());
    assert_eq!(event.project_id, Some("proj-4762".to_string()));
}

#[test]
fn test_event_env_4763() {
    let mut event = Event::new("evt-4763".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4763".to_string());
    assert_eq!(event.environment_id, Some("env-4763".to_string()));
}

#[test]
fn test_event_service_4764() {
    let mut event = Event::new("evt-4764".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4764".to_string());
    assert_eq!(event.service_id, Some("svc-4764".to_string()));
}

#[test]
fn test_event_set_latency_4765() {
    let mut event = Event::new("evt-4765".to_string(), ProtocolType::Http);
    event.latency_us = 4765 as u64;
    assert_eq!(event.latency_us, 4765 as u64);
}

#[test]
fn test_metadata_new_4766() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4767() {
    let mut event = Event::new("evt-4767".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4767".to_string());
    assert_eq!(event.project_id, Some("proj-4767".to_string()));
}

#[test]
fn test_event_env_4768() {
    let mut event = Event::new("evt-4768".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4768".to_string());
    assert_eq!(event.environment_id, Some("env-4768".to_string()));
}

#[test]
fn test_event_service_4769() {
    let mut event = Event::new("evt-4769".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4769".to_string());
    assert_eq!(event.service_id, Some("svc-4769".to_string()));
}

#[test]
fn test_event_set_latency_4770() {
    let mut event = Event::new("evt-4770".to_string(), ProtocolType::Http);
    event.latency_us = 4770 as u64;
    assert_eq!(event.latency_us, 4770 as u64);
}

#[test]
fn test_metadata_new_4771() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4772() {
    let mut event = Event::new("evt-4772".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4772".to_string());
    assert_eq!(event.project_id, Some("proj-4772".to_string()));
}

#[test]
fn test_event_env_4773() {
    let mut event = Event::new("evt-4773".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4773".to_string());
    assert_eq!(event.environment_id, Some("env-4773".to_string()));
}

#[test]
fn test_event_service_4774() {
    let mut event = Event::new("evt-4774".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4774".to_string());
    assert_eq!(event.service_id, Some("svc-4774".to_string()));
}

#[test]
fn test_event_set_latency_4775() {
    let mut event = Event::new("evt-4775".to_string(), ProtocolType::Http);
    event.latency_us = 4775 as u64;
    assert_eq!(event.latency_us, 4775 as u64);
}

#[test]
fn test_metadata_new_4776() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4777() {
    let mut event = Event::new("evt-4777".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4777".to_string());
    assert_eq!(event.project_id, Some("proj-4777".to_string()));
}

#[test]
fn test_event_env_4778() {
    let mut event = Event::new("evt-4778".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4778".to_string());
    assert_eq!(event.environment_id, Some("env-4778".to_string()));
}

#[test]
fn test_event_service_4779() {
    let mut event = Event::new("evt-4779".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4779".to_string());
    assert_eq!(event.service_id, Some("svc-4779".to_string()));
}

#[test]
fn test_event_set_latency_4780() {
    let mut event = Event::new("evt-4780".to_string(), ProtocolType::Http);
    event.latency_us = 4780 as u64;
    assert_eq!(event.latency_us, 4780 as u64);
}

#[test]
fn test_metadata_new_4781() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4782() {
    let mut event = Event::new("evt-4782".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4782".to_string());
    assert_eq!(event.project_id, Some("proj-4782".to_string()));
}

#[test]
fn test_event_env_4783() {
    let mut event = Event::new("evt-4783".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4783".to_string());
    assert_eq!(event.environment_id, Some("env-4783".to_string()));
}

#[test]
fn test_event_service_4784() {
    let mut event = Event::new("evt-4784".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4784".to_string());
    assert_eq!(event.service_id, Some("svc-4784".to_string()));
}

#[test]
fn test_event_set_latency_4785() {
    let mut event = Event::new("evt-4785".to_string(), ProtocolType::Http);
    event.latency_us = 4785 as u64;
    assert_eq!(event.latency_us, 4785 as u64);
}

#[test]
fn test_metadata_new_4786() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4787() {
    let mut event = Event::new("evt-4787".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4787".to_string());
    assert_eq!(event.project_id, Some("proj-4787".to_string()));
}

#[test]
fn test_event_env_4788() {
    let mut event = Event::new("evt-4788".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4788".to_string());
    assert_eq!(event.environment_id, Some("env-4788".to_string()));
}

#[test]
fn test_event_service_4789() {
    let mut event = Event::new("evt-4789".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4789".to_string());
    assert_eq!(event.service_id, Some("svc-4789".to_string()));
}

#[test]
fn test_event_set_latency_4790() {
    let mut event = Event::new("evt-4790".to_string(), ProtocolType::Http);
    event.latency_us = 4790 as u64;
    assert_eq!(event.latency_us, 4790 as u64);
}

#[test]
fn test_metadata_new_4791() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4792() {
    let mut event = Event::new("evt-4792".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4792".to_string());
    assert_eq!(event.project_id, Some("proj-4792".to_string()));
}

#[test]
fn test_event_env_4793() {
    let mut event = Event::new("evt-4793".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4793".to_string());
    assert_eq!(event.environment_id, Some("env-4793".to_string()));
}

#[test]
fn test_event_service_4794() {
    let mut event = Event::new("evt-4794".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4794".to_string());
    assert_eq!(event.service_id, Some("svc-4794".to_string()));
}

#[test]
fn test_event_set_latency_4795() {
    let mut event = Event::new("evt-4795".to_string(), ProtocolType::Http);
    event.latency_us = 4795 as u64;
    assert_eq!(event.latency_us, 4795 as u64);
}

#[test]
fn test_metadata_new_4796() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4797() {
    let mut event = Event::new("evt-4797".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4797".to_string());
    assert_eq!(event.project_id, Some("proj-4797".to_string()));
}

#[test]
fn test_event_env_4798() {
    let mut event = Event::new("evt-4798".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4798".to_string());
    assert_eq!(event.environment_id, Some("env-4798".to_string()));
}

#[test]
fn test_event_service_4799() {
    let mut event = Event::new("evt-4799".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4799".to_string());
    assert_eq!(event.service_id, Some("svc-4799".to_string()));
}

#[test]
fn test_event_set_latency_4800() {
    let mut event = Event::new("evt-4800".to_string(), ProtocolType::Http);
    event.latency_us = 4800 as u64;
    assert_eq!(event.latency_us, 4800 as u64);
}

#[test]
fn test_metadata_new_4801() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4802() {
    let mut event = Event::new("evt-4802".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4802".to_string());
    assert_eq!(event.project_id, Some("proj-4802".to_string()));
}

#[test]
fn test_event_env_4803() {
    let mut event = Event::new("evt-4803".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4803".to_string());
    assert_eq!(event.environment_id, Some("env-4803".to_string()));
}

#[test]
fn test_event_service_4804() {
    let mut event = Event::new("evt-4804".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4804".to_string());
    assert_eq!(event.service_id, Some("svc-4804".to_string()));
}

#[test]
fn test_event_set_latency_4805() {
    let mut event = Event::new("evt-4805".to_string(), ProtocolType::Http);
    event.latency_us = 4805 as u64;
    assert_eq!(event.latency_us, 4805 as u64);
}

#[test]
fn test_metadata_new_4806() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4807() {
    let mut event = Event::new("evt-4807".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4807".to_string());
    assert_eq!(event.project_id, Some("proj-4807".to_string()));
}

#[test]
fn test_event_env_4808() {
    let mut event = Event::new("evt-4808".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4808".to_string());
    assert_eq!(event.environment_id, Some("env-4808".to_string()));
}

#[test]
fn test_event_service_4809() {
    let mut event = Event::new("evt-4809".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4809".to_string());
    assert_eq!(event.service_id, Some("svc-4809".to_string()));
}

#[test]
fn test_event_set_latency_4810() {
    let mut event = Event::new("evt-4810".to_string(), ProtocolType::Http);
    event.latency_us = 4810 as u64;
    assert_eq!(event.latency_us, 4810 as u64);
}

#[test]
fn test_metadata_new_4811() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4812() {
    let mut event = Event::new("evt-4812".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4812".to_string());
    assert_eq!(event.project_id, Some("proj-4812".to_string()));
}

#[test]
fn test_event_env_4813() {
    let mut event = Event::new("evt-4813".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4813".to_string());
    assert_eq!(event.environment_id, Some("env-4813".to_string()));
}

#[test]
fn test_event_service_4814() {
    let mut event = Event::new("evt-4814".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4814".to_string());
    assert_eq!(event.service_id, Some("svc-4814".to_string()));
}

#[test]
fn test_event_set_latency_4815() {
    let mut event = Event::new("evt-4815".to_string(), ProtocolType::Http);
    event.latency_us = 4815 as u64;
    assert_eq!(event.latency_us, 4815 as u64);
}

#[test]
fn test_metadata_new_4816() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4817() {
    let mut event = Event::new("evt-4817".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4817".to_string());
    assert_eq!(event.project_id, Some("proj-4817".to_string()));
}

#[test]
fn test_event_env_4818() {
    let mut event = Event::new("evt-4818".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4818".to_string());
    assert_eq!(event.environment_id, Some("env-4818".to_string()));
}

#[test]
fn test_event_service_4819() {
    let mut event = Event::new("evt-4819".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4819".to_string());
    assert_eq!(event.service_id, Some("svc-4819".to_string()));
}

#[test]
fn test_event_set_latency_4820() {
    let mut event = Event::new("evt-4820".to_string(), ProtocolType::Http);
    event.latency_us = 4820 as u64;
    assert_eq!(event.latency_us, 4820 as u64);
}

#[test]
fn test_metadata_new_4821() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4822() {
    let mut event = Event::new("evt-4822".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4822".to_string());
    assert_eq!(event.project_id, Some("proj-4822".to_string()));
}

#[test]
fn test_event_env_4823() {
    let mut event = Event::new("evt-4823".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4823".to_string());
    assert_eq!(event.environment_id, Some("env-4823".to_string()));
}

#[test]
fn test_event_service_4824() {
    let mut event = Event::new("evt-4824".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4824".to_string());
    assert_eq!(event.service_id, Some("svc-4824".to_string()));
}

#[test]
fn test_event_set_latency_4825() {
    let mut event = Event::new("evt-4825".to_string(), ProtocolType::Http);
    event.latency_us = 4825 as u64;
    assert_eq!(event.latency_us, 4825 as u64);
}

#[test]
fn test_metadata_new_4826() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4827() {
    let mut event = Event::new("evt-4827".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4827".to_string());
    assert_eq!(event.project_id, Some("proj-4827".to_string()));
}

#[test]
fn test_event_env_4828() {
    let mut event = Event::new("evt-4828".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4828".to_string());
    assert_eq!(event.environment_id, Some("env-4828".to_string()));
}

#[test]
fn test_event_service_4829() {
    let mut event = Event::new("evt-4829".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4829".to_string());
    assert_eq!(event.service_id, Some("svc-4829".to_string()));
}

#[test]
fn test_event_set_latency_4830() {
    let mut event = Event::new("evt-4830".to_string(), ProtocolType::Http);
    event.latency_us = 4830 as u64;
    assert_eq!(event.latency_us, 4830 as u64);
}

#[test]
fn test_metadata_new_4831() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4832() {
    let mut event = Event::new("evt-4832".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4832".to_string());
    assert_eq!(event.project_id, Some("proj-4832".to_string()));
}

#[test]
fn test_event_env_4833() {
    let mut event = Event::new("evt-4833".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4833".to_string());
    assert_eq!(event.environment_id, Some("env-4833".to_string()));
}

#[test]
fn test_event_service_4834() {
    let mut event = Event::new("evt-4834".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4834".to_string());
    assert_eq!(event.service_id, Some("svc-4834".to_string()));
}

#[test]
fn test_event_set_latency_4835() {
    let mut event = Event::new("evt-4835".to_string(), ProtocolType::Http);
    event.latency_us = 4835 as u64;
    assert_eq!(event.latency_us, 4835 as u64);
}

#[test]
fn test_metadata_new_4836() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4837() {
    let mut event = Event::new("evt-4837".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4837".to_string());
    assert_eq!(event.project_id, Some("proj-4837".to_string()));
}

#[test]
fn test_event_env_4838() {
    let mut event = Event::new("evt-4838".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4838".to_string());
    assert_eq!(event.environment_id, Some("env-4838".to_string()));
}

#[test]
fn test_event_service_4839() {
    let mut event = Event::new("evt-4839".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4839".to_string());
    assert_eq!(event.service_id, Some("svc-4839".to_string()));
}

#[test]
fn test_event_set_latency_4840() {
    let mut event = Event::new("evt-4840".to_string(), ProtocolType::Http);
    event.latency_us = 4840 as u64;
    assert_eq!(event.latency_us, 4840 as u64);
}

#[test]
fn test_metadata_new_4841() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4842() {
    let mut event = Event::new("evt-4842".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4842".to_string());
    assert_eq!(event.project_id, Some("proj-4842".to_string()));
}

#[test]
fn test_event_env_4843() {
    let mut event = Event::new("evt-4843".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4843".to_string());
    assert_eq!(event.environment_id, Some("env-4843".to_string()));
}

#[test]
fn test_event_service_4844() {
    let mut event = Event::new("evt-4844".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4844".to_string());
    assert_eq!(event.service_id, Some("svc-4844".to_string()));
}

#[test]
fn test_event_set_latency_4845() {
    let mut event = Event::new("evt-4845".to_string(), ProtocolType::Http);
    event.latency_us = 4845 as u64;
    assert_eq!(event.latency_us, 4845 as u64);
}

#[test]
fn test_metadata_new_4846() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4847() {
    let mut event = Event::new("evt-4847".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4847".to_string());
    assert_eq!(event.project_id, Some("proj-4847".to_string()));
}

#[test]
fn test_event_env_4848() {
    let mut event = Event::new("evt-4848".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4848".to_string());
    assert_eq!(event.environment_id, Some("env-4848".to_string()));
}

#[test]
fn test_event_service_4849() {
    let mut event = Event::new("evt-4849".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4849".to_string());
    assert_eq!(event.service_id, Some("svc-4849".to_string()));
}

#[test]
fn test_event_set_latency_4850() {
    let mut event = Event::new("evt-4850".to_string(), ProtocolType::Http);
    event.latency_us = 4850 as u64;
    assert_eq!(event.latency_us, 4850 as u64);
}

#[test]
fn test_metadata_new_4851() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4852() {
    let mut event = Event::new("evt-4852".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4852".to_string());
    assert_eq!(event.project_id, Some("proj-4852".to_string()));
}

#[test]
fn test_event_env_4853() {
    let mut event = Event::new("evt-4853".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4853".to_string());
    assert_eq!(event.environment_id, Some("env-4853".to_string()));
}

#[test]
fn test_event_service_4854() {
    let mut event = Event::new("evt-4854".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4854".to_string());
    assert_eq!(event.service_id, Some("svc-4854".to_string()));
}

#[test]
fn test_event_set_latency_4855() {
    let mut event = Event::new("evt-4855".to_string(), ProtocolType::Http);
    event.latency_us = 4855 as u64;
    assert_eq!(event.latency_us, 4855 as u64);
}

#[test]
fn test_metadata_new_4856() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4857() {
    let mut event = Event::new("evt-4857".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4857".to_string());
    assert_eq!(event.project_id, Some("proj-4857".to_string()));
}

#[test]
fn test_event_env_4858() {
    let mut event = Event::new("evt-4858".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4858".to_string());
    assert_eq!(event.environment_id, Some("env-4858".to_string()));
}

#[test]
fn test_event_service_4859() {
    let mut event = Event::new("evt-4859".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4859".to_string());
    assert_eq!(event.service_id, Some("svc-4859".to_string()));
}

#[test]
fn test_event_set_latency_4860() {
    let mut event = Event::new("evt-4860".to_string(), ProtocolType::Http);
    event.latency_us = 4860 as u64;
    assert_eq!(event.latency_us, 4860 as u64);
}

#[test]
fn test_metadata_new_4861() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4862() {
    let mut event = Event::new("evt-4862".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4862".to_string());
    assert_eq!(event.project_id, Some("proj-4862".to_string()));
}

#[test]
fn test_event_env_4863() {
    let mut event = Event::new("evt-4863".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4863".to_string());
    assert_eq!(event.environment_id, Some("env-4863".to_string()));
}

#[test]
fn test_event_service_4864() {
    let mut event = Event::new("evt-4864".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4864".to_string());
    assert_eq!(event.service_id, Some("svc-4864".to_string()));
}

#[test]
fn test_event_set_latency_4865() {
    let mut event = Event::new("evt-4865".to_string(), ProtocolType::Http);
    event.latency_us = 4865 as u64;
    assert_eq!(event.latency_us, 4865 as u64);
}

#[test]
fn test_metadata_new_4866() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4867() {
    let mut event = Event::new("evt-4867".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4867".to_string());
    assert_eq!(event.project_id, Some("proj-4867".to_string()));
}

#[test]
fn test_event_env_4868() {
    let mut event = Event::new("evt-4868".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4868".to_string());
    assert_eq!(event.environment_id, Some("env-4868".to_string()));
}

#[test]
fn test_event_service_4869() {
    let mut event = Event::new("evt-4869".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4869".to_string());
    assert_eq!(event.service_id, Some("svc-4869".to_string()));
}

#[test]
fn test_event_set_latency_4870() {
    let mut event = Event::new("evt-4870".to_string(), ProtocolType::Http);
    event.latency_us = 4870 as u64;
    assert_eq!(event.latency_us, 4870 as u64);
}

#[test]
fn test_metadata_new_4871() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4872() {
    let mut event = Event::new("evt-4872".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4872".to_string());
    assert_eq!(event.project_id, Some("proj-4872".to_string()));
}

#[test]
fn test_event_env_4873() {
    let mut event = Event::new("evt-4873".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4873".to_string());
    assert_eq!(event.environment_id, Some("env-4873".to_string()));
}

#[test]
fn test_event_service_4874() {
    let mut event = Event::new("evt-4874".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4874".to_string());
    assert_eq!(event.service_id, Some("svc-4874".to_string()));
}

#[test]
fn test_event_set_latency_4875() {
    let mut event = Event::new("evt-4875".to_string(), ProtocolType::Http);
    event.latency_us = 4875 as u64;
    assert_eq!(event.latency_us, 4875 as u64);
}

#[test]
fn test_metadata_new_4876() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4877() {
    let mut event = Event::new("evt-4877".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4877".to_string());
    assert_eq!(event.project_id, Some("proj-4877".to_string()));
}

#[test]
fn test_event_env_4878() {
    let mut event = Event::new("evt-4878".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4878".to_string());
    assert_eq!(event.environment_id, Some("env-4878".to_string()));
}

#[test]
fn test_event_service_4879() {
    let mut event = Event::new("evt-4879".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4879".to_string());
    assert_eq!(event.service_id, Some("svc-4879".to_string()));
}

#[test]
fn test_event_set_latency_4880() {
    let mut event = Event::new("evt-4880".to_string(), ProtocolType::Http);
    event.latency_us = 4880 as u64;
    assert_eq!(event.latency_us, 4880 as u64);
}

#[test]
fn test_metadata_new_4881() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4882() {
    let mut event = Event::new("evt-4882".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4882".to_string());
    assert_eq!(event.project_id, Some("proj-4882".to_string()));
}

#[test]
fn test_event_env_4883() {
    let mut event = Event::new("evt-4883".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4883".to_string());
    assert_eq!(event.environment_id, Some("env-4883".to_string()));
}

#[test]
fn test_event_service_4884() {
    let mut event = Event::new("evt-4884".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4884".to_string());
    assert_eq!(event.service_id, Some("svc-4884".to_string()));
}

#[test]
fn test_event_set_latency_4885() {
    let mut event = Event::new("evt-4885".to_string(), ProtocolType::Http);
    event.latency_us = 4885 as u64;
    assert_eq!(event.latency_us, 4885 as u64);
}

#[test]
fn test_metadata_new_4886() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4887() {
    let mut event = Event::new("evt-4887".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4887".to_string());
    assert_eq!(event.project_id, Some("proj-4887".to_string()));
}

#[test]
fn test_event_env_4888() {
    let mut event = Event::new("evt-4888".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4888".to_string());
    assert_eq!(event.environment_id, Some("env-4888".to_string()));
}

#[test]
fn test_event_service_4889() {
    let mut event = Event::new("evt-4889".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4889".to_string());
    assert_eq!(event.service_id, Some("svc-4889".to_string()));
}

#[test]
fn test_event_set_latency_4890() {
    let mut event = Event::new("evt-4890".to_string(), ProtocolType::Http);
    event.latency_us = 4890 as u64;
    assert_eq!(event.latency_us, 4890 as u64);
}

#[test]
fn test_metadata_new_4891() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4892() {
    let mut event = Event::new("evt-4892".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4892".to_string());
    assert_eq!(event.project_id, Some("proj-4892".to_string()));
}

#[test]
fn test_event_env_4893() {
    let mut event = Event::new("evt-4893".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4893".to_string());
    assert_eq!(event.environment_id, Some("env-4893".to_string()));
}

#[test]
fn test_event_service_4894() {
    let mut event = Event::new("evt-4894".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4894".to_string());
    assert_eq!(event.service_id, Some("svc-4894".to_string()));
}

#[test]
fn test_event_set_latency_4895() {
    let mut event = Event::new("evt-4895".to_string(), ProtocolType::Http);
    event.latency_us = 4895 as u64;
    assert_eq!(event.latency_us, 4895 as u64);
}

#[test]
fn test_metadata_new_4896() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4897() {
    let mut event = Event::new("evt-4897".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4897".to_string());
    assert_eq!(event.project_id, Some("proj-4897".to_string()));
}

#[test]
fn test_event_env_4898() {
    let mut event = Event::new("evt-4898".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4898".to_string());
    assert_eq!(event.environment_id, Some("env-4898".to_string()));
}

#[test]
fn test_event_service_4899() {
    let mut event = Event::new("evt-4899".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4899".to_string());
    assert_eq!(event.service_id, Some("svc-4899".to_string()));
}

#[test]
fn test_event_set_latency_4900() {
    let mut event = Event::new("evt-4900".to_string(), ProtocolType::Http);
    event.latency_us = 4900 as u64;
    assert_eq!(event.latency_us, 4900 as u64);
}

#[test]
fn test_metadata_new_4901() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4902() {
    let mut event = Event::new("evt-4902".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4902".to_string());
    assert_eq!(event.project_id, Some("proj-4902".to_string()));
}

#[test]
fn test_event_env_4903() {
    let mut event = Event::new("evt-4903".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4903".to_string());
    assert_eq!(event.environment_id, Some("env-4903".to_string()));
}

#[test]
fn test_event_service_4904() {
    let mut event = Event::new("evt-4904".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4904".to_string());
    assert_eq!(event.service_id, Some("svc-4904".to_string()));
}

#[test]
fn test_event_set_latency_4905() {
    let mut event = Event::new("evt-4905".to_string(), ProtocolType::Http);
    event.latency_us = 4905 as u64;
    assert_eq!(event.latency_us, 4905 as u64);
}

#[test]
fn test_metadata_new_4906() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4907() {
    let mut event = Event::new("evt-4907".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4907".to_string());
    assert_eq!(event.project_id, Some("proj-4907".to_string()));
}

#[test]
fn test_event_env_4908() {
    let mut event = Event::new("evt-4908".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4908".to_string());
    assert_eq!(event.environment_id, Some("env-4908".to_string()));
}

#[test]
fn test_event_service_4909() {
    let mut event = Event::new("evt-4909".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4909".to_string());
    assert_eq!(event.service_id, Some("svc-4909".to_string()));
}

#[test]
fn test_event_set_latency_4910() {
    let mut event = Event::new("evt-4910".to_string(), ProtocolType::Http);
    event.latency_us = 4910 as u64;
    assert_eq!(event.latency_us, 4910 as u64);
}

#[test]
fn test_metadata_new_4911() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4912() {
    let mut event = Event::new("evt-4912".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4912".to_string());
    assert_eq!(event.project_id, Some("proj-4912".to_string()));
}

#[test]
fn test_event_env_4913() {
    let mut event = Event::new("evt-4913".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4913".to_string());
    assert_eq!(event.environment_id, Some("env-4913".to_string()));
}

#[test]
fn test_event_service_4914() {
    let mut event = Event::new("evt-4914".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4914".to_string());
    assert_eq!(event.service_id, Some("svc-4914".to_string()));
}

#[test]
fn test_event_set_latency_4915() {
    let mut event = Event::new("evt-4915".to_string(), ProtocolType::Http);
    event.latency_us = 4915 as u64;
    assert_eq!(event.latency_us, 4915 as u64);
}

#[test]
fn test_metadata_new_4916() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4917() {
    let mut event = Event::new("evt-4917".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4917".to_string());
    assert_eq!(event.project_id, Some("proj-4917".to_string()));
}

#[test]
fn test_event_env_4918() {
    let mut event = Event::new("evt-4918".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4918".to_string());
    assert_eq!(event.environment_id, Some("env-4918".to_string()));
}

#[test]
fn test_event_service_4919() {
    let mut event = Event::new("evt-4919".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4919".to_string());
    assert_eq!(event.service_id, Some("svc-4919".to_string()));
}

#[test]
fn test_event_set_latency_4920() {
    let mut event = Event::new("evt-4920".to_string(), ProtocolType::Http);
    event.latency_us = 4920 as u64;
    assert_eq!(event.latency_us, 4920 as u64);
}

#[test]
fn test_metadata_new_4921() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4922() {
    let mut event = Event::new("evt-4922".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4922".to_string());
    assert_eq!(event.project_id, Some("proj-4922".to_string()));
}

#[test]
fn test_event_env_4923() {
    let mut event = Event::new("evt-4923".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4923".to_string());
    assert_eq!(event.environment_id, Some("env-4923".to_string()));
}

#[test]
fn test_event_service_4924() {
    let mut event = Event::new("evt-4924".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4924".to_string());
    assert_eq!(event.service_id, Some("svc-4924".to_string()));
}

#[test]
fn test_event_set_latency_4925() {
    let mut event = Event::new("evt-4925".to_string(), ProtocolType::Http);
    event.latency_us = 4925 as u64;
    assert_eq!(event.latency_us, 4925 as u64);
}

#[test]
fn test_metadata_new_4926() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4927() {
    let mut event = Event::new("evt-4927".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4927".to_string());
    assert_eq!(event.project_id, Some("proj-4927".to_string()));
}

#[test]
fn test_event_env_4928() {
    let mut event = Event::new("evt-4928".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4928".to_string());
    assert_eq!(event.environment_id, Some("env-4928".to_string()));
}

#[test]
fn test_event_service_4929() {
    let mut event = Event::new("evt-4929".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4929".to_string());
    assert_eq!(event.service_id, Some("svc-4929".to_string()));
}

#[test]
fn test_event_set_latency_4930() {
    let mut event = Event::new("evt-4930".to_string(), ProtocolType::Http);
    event.latency_us = 4930 as u64;
    assert_eq!(event.latency_us, 4930 as u64);
}

#[test]
fn test_metadata_new_4931() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4932() {
    let mut event = Event::new("evt-4932".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4932".to_string());
    assert_eq!(event.project_id, Some("proj-4932".to_string()));
}

#[test]
fn test_event_env_4933() {
    let mut event = Event::new("evt-4933".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4933".to_string());
    assert_eq!(event.environment_id, Some("env-4933".to_string()));
}

#[test]
fn test_event_service_4934() {
    let mut event = Event::new("evt-4934".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4934".to_string());
    assert_eq!(event.service_id, Some("svc-4934".to_string()));
}

#[test]
fn test_event_set_latency_4935() {
    let mut event = Event::new("evt-4935".to_string(), ProtocolType::Http);
    event.latency_us = 4935 as u64;
    assert_eq!(event.latency_us, 4935 as u64);
}

#[test]
fn test_metadata_new_4936() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4937() {
    let mut event = Event::new("evt-4937".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4937".to_string());
    assert_eq!(event.project_id, Some("proj-4937".to_string()));
}

#[test]
fn test_event_env_4938() {
    let mut event = Event::new("evt-4938".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4938".to_string());
    assert_eq!(event.environment_id, Some("env-4938".to_string()));
}

#[test]
fn test_event_service_4939() {
    let mut event = Event::new("evt-4939".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4939".to_string());
    assert_eq!(event.service_id, Some("svc-4939".to_string()));
}

#[test]
fn test_event_set_latency_4940() {
    let mut event = Event::new("evt-4940".to_string(), ProtocolType::Http);
    event.latency_us = 4940 as u64;
    assert_eq!(event.latency_us, 4940 as u64);
}

#[test]
fn test_metadata_new_4941() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4942() {
    let mut event = Event::new("evt-4942".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4942".to_string());
    assert_eq!(event.project_id, Some("proj-4942".to_string()));
}

#[test]
fn test_event_env_4943() {
    let mut event = Event::new("evt-4943".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4943".to_string());
    assert_eq!(event.environment_id, Some("env-4943".to_string()));
}

#[test]
fn test_event_service_4944() {
    let mut event = Event::new("evt-4944".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4944".to_string());
    assert_eq!(event.service_id, Some("svc-4944".to_string()));
}

#[test]
fn test_event_set_latency_4945() {
    let mut event = Event::new("evt-4945".to_string(), ProtocolType::Http);
    event.latency_us = 4945 as u64;
    assert_eq!(event.latency_us, 4945 as u64);
}

#[test]
fn test_metadata_new_4946() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4947() {
    let mut event = Event::new("evt-4947".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4947".to_string());
    assert_eq!(event.project_id, Some("proj-4947".to_string()));
}

#[test]
fn test_event_env_4948() {
    let mut event = Event::new("evt-4948".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4948".to_string());
    assert_eq!(event.environment_id, Some("env-4948".to_string()));
}

#[test]
fn test_event_service_4949() {
    let mut event = Event::new("evt-4949".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4949".to_string());
    assert_eq!(event.service_id, Some("svc-4949".to_string()));
}

#[test]
fn test_event_set_latency_4950() {
    let mut event = Event::new("evt-4950".to_string(), ProtocolType::Http);
    event.latency_us = 4950 as u64;
    assert_eq!(event.latency_us, 4950 as u64);
}

#[test]
fn test_metadata_new_4951() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4952() {
    let mut event = Event::new("evt-4952".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4952".to_string());
    assert_eq!(event.project_id, Some("proj-4952".to_string()));
}

#[test]
fn test_event_env_4953() {
    let mut event = Event::new("evt-4953".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4953".to_string());
    assert_eq!(event.environment_id, Some("env-4953".to_string()));
}

#[test]
fn test_event_service_4954() {
    let mut event = Event::new("evt-4954".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4954".to_string());
    assert_eq!(event.service_id, Some("svc-4954".to_string()));
}

#[test]
fn test_event_set_latency_4955() {
    let mut event = Event::new("evt-4955".to_string(), ProtocolType::Http);
    event.latency_us = 4955 as u64;
    assert_eq!(event.latency_us, 4955 as u64);
}

#[test]
fn test_metadata_new_4956() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4957() {
    let mut event = Event::new("evt-4957".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4957".to_string());
    assert_eq!(event.project_id, Some("proj-4957".to_string()));
}

#[test]
fn test_event_env_4958() {
    let mut event = Event::new("evt-4958".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4958".to_string());
    assert_eq!(event.environment_id, Some("env-4958".to_string()));
}

#[test]
fn test_event_service_4959() {
    let mut event = Event::new("evt-4959".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4959".to_string());
    assert_eq!(event.service_id, Some("svc-4959".to_string()));
}

#[test]
fn test_event_set_latency_4960() {
    let mut event = Event::new("evt-4960".to_string(), ProtocolType::Http);
    event.latency_us = 4960 as u64;
    assert_eq!(event.latency_us, 4960 as u64);
}

#[test]
fn test_metadata_new_4961() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4962() {
    let mut event = Event::new("evt-4962".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4962".to_string());
    assert_eq!(event.project_id, Some("proj-4962".to_string()));
}

#[test]
fn test_event_env_4963() {
    let mut event = Event::new("evt-4963".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4963".to_string());
    assert_eq!(event.environment_id, Some("env-4963".to_string()));
}

#[test]
fn test_event_service_4964() {
    let mut event = Event::new("evt-4964".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4964".to_string());
    assert_eq!(event.service_id, Some("svc-4964".to_string()));
}

#[test]
fn test_event_set_latency_4965() {
    let mut event = Event::new("evt-4965".to_string(), ProtocolType::Http);
    event.latency_us = 4965 as u64;
    assert_eq!(event.latency_us, 4965 as u64);
}

#[test]
fn test_metadata_new_4966() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4967() {
    let mut event = Event::new("evt-4967".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4967".to_string());
    assert_eq!(event.project_id, Some("proj-4967".to_string()));
}

#[test]
fn test_event_env_4968() {
    let mut event = Event::new("evt-4968".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4968".to_string());
    assert_eq!(event.environment_id, Some("env-4968".to_string()));
}

#[test]
fn test_event_service_4969() {
    let mut event = Event::new("evt-4969".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4969".to_string());
    assert_eq!(event.service_id, Some("svc-4969".to_string()));
}

#[test]
fn test_event_set_latency_4970() {
    let mut event = Event::new("evt-4970".to_string(), ProtocolType::Http);
    event.latency_us = 4970 as u64;
    assert_eq!(event.latency_us, 4970 as u64);
}

#[test]
fn test_metadata_new_4971() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4972() {
    let mut event = Event::new("evt-4972".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4972".to_string());
    assert_eq!(event.project_id, Some("proj-4972".to_string()));
}

#[test]
fn test_event_env_4973() {
    let mut event = Event::new("evt-4973".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4973".to_string());
    assert_eq!(event.environment_id, Some("env-4973".to_string()));
}

#[test]
fn test_event_service_4974() {
    let mut event = Event::new("evt-4974".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4974".to_string());
    assert_eq!(event.service_id, Some("svc-4974".to_string()));
}

#[test]
fn test_event_set_latency_4975() {
    let mut event = Event::new("evt-4975".to_string(), ProtocolType::Http);
    event.latency_us = 4975 as u64;
    assert_eq!(event.latency_us, 4975 as u64);
}

#[test]
fn test_metadata_new_4976() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4977() {
    let mut event = Event::new("evt-4977".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4977".to_string());
    assert_eq!(event.project_id, Some("proj-4977".to_string()));
}

#[test]
fn test_event_env_4978() {
    let mut event = Event::new("evt-4978".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4978".to_string());
    assert_eq!(event.environment_id, Some("env-4978".to_string()));
}

#[test]
fn test_event_service_4979() {
    let mut event = Event::new("evt-4979".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4979".to_string());
    assert_eq!(event.service_id, Some("svc-4979".to_string()));
}

#[test]
fn test_event_set_latency_4980() {
    let mut event = Event::new("evt-4980".to_string(), ProtocolType::Http);
    event.latency_us = 4980 as u64;
    assert_eq!(event.latency_us, 4980 as u64);
}

#[test]
fn test_metadata_new_4981() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4982() {
    let mut event = Event::new("evt-4982".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4982".to_string());
    assert_eq!(event.project_id, Some("proj-4982".to_string()));
}

#[test]
fn test_event_env_4983() {
    let mut event = Event::new("evt-4983".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4983".to_string());
    assert_eq!(event.environment_id, Some("env-4983".to_string()));
}

#[test]
fn test_event_service_4984() {
    let mut event = Event::new("evt-4984".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4984".to_string());
    assert_eq!(event.service_id, Some("svc-4984".to_string()));
}

#[test]
fn test_event_set_latency_4985() {
    let mut event = Event::new("evt-4985".to_string(), ProtocolType::Http);
    event.latency_us = 4985 as u64;
    assert_eq!(event.latency_us, 4985 as u64);
}

#[test]
fn test_metadata_new_4986() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4987() {
    let mut event = Event::new("evt-4987".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4987".to_string());
    assert_eq!(event.project_id, Some("proj-4987".to_string()));
}

#[test]
fn test_event_env_4988() {
    let mut event = Event::new("evt-4988".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4988".to_string());
    assert_eq!(event.environment_id, Some("env-4988".to_string()));
}

#[test]
fn test_event_service_4989() {
    let mut event = Event::new("evt-4989".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4989".to_string());
    assert_eq!(event.service_id, Some("svc-4989".to_string()));
}

#[test]
fn test_event_set_latency_4990() {
    let mut event = Event::new("evt-4990".to_string(), ProtocolType::Http);
    event.latency_us = 4990 as u64;
    assert_eq!(event.latency_us, 4990 as u64);
}

#[test]
fn test_metadata_new_4991() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4992() {
    let mut event = Event::new("evt-4992".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4992".to_string());
    assert_eq!(event.project_id, Some("proj-4992".to_string()));
}

#[test]
fn test_event_env_4993() {
    let mut event = Event::new("evt-4993".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4993".to_string());
    assert_eq!(event.environment_id, Some("env-4993".to_string()));
}

#[test]
fn test_event_service_4994() {
    let mut event = Event::new("evt-4994".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4994".to_string());
    assert_eq!(event.service_id, Some("svc-4994".to_string()));
}

#[test]
fn test_event_set_latency_4995() {
    let mut event = Event::new("evt-4995".to_string(), ProtocolType::Http);
    event.latency_us = 4995 as u64;
    assert_eq!(event.latency_us, 4995 as u64);
}

#[test]
fn test_metadata_new_4996() {
    let meta = Metadata::new();
    assert!(meta.source_ip.is_none());
}

#[test]
fn test_event_project_4997() {
    let mut event = Event::new("evt-4997".to_string(), ProtocolType::Http);
    event.project_id = Some("proj-4997".to_string());
    assert_eq!(event.project_id, Some("proj-4997".to_string()));
}

#[test]
fn test_event_env_4998() {
    let mut event = Event::new("evt-4998".to_string(), ProtocolType::Http);
    event.environment_id = Some("env-4998".to_string());
    assert_eq!(event.environment_id, Some("env-4998".to_string()));
}

#[test]
fn test_event_service_4999() {
    let mut event = Event::new("evt-4999".to_string(), ProtocolType::Http);
    event.service_id = Some("svc-4999".to_string());
    assert_eq!(event.service_id, Some("svc-4999".to_string()));
}

#[test]
fn test_event_set_latency_5000() {
    let mut event = Event::new("evt-5000".to_string(), ProtocolType::Http);
    event.latency_us = 5000 as u64;
    assert_eq!(event.latency_us, 5000 as u64);
}


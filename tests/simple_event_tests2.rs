// Simple Event Tests 2 (4001-4500)

use fluxx_agent::models::event::{Event, ProtocolType};

#[test]
fn test_event_with_payload_4001() {
    let event = Event::new("evt-4001".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4002() {
    let event = Event::new("evt-4002".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4003() {
    let event = Event::new("evt-4003".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4003"));
}

#[test]
fn test_event_timestamp_4004() {
    let event = Event::new("evt-4004".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4005() {
    let event = Event::new("evt-4005".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4006() {
    let event = Event::new("evt-4006".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4007() {
    let event = Event::new("evt-4007".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4008() {
    let event = Event::new("evt-4008".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4008"));
}

#[test]
fn test_event_timestamp_4009() {
    let event = Event::new("evt-4009".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4010() {
    let event = Event::new("evt-4010".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4011() {
    let event = Event::new("evt-4011".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4012() {
    let event = Event::new("evt-4012".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4013() {
    let event = Event::new("evt-4013".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4013"));
}

#[test]
fn test_event_timestamp_4014() {
    let event = Event::new("evt-4014".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4015() {
    let event = Event::new("evt-4015".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4016() {
    let event = Event::new("evt-4016".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4017() {
    let event = Event::new("evt-4017".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4018() {
    let event = Event::new("evt-4018".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4018"));
}

#[test]
fn test_event_timestamp_4019() {
    let event = Event::new("evt-4019".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4020() {
    let event = Event::new("evt-4020".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4021() {
    let event = Event::new("evt-4021".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4022() {
    let event = Event::new("evt-4022".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4023() {
    let event = Event::new("evt-4023".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4023"));
}

#[test]
fn test_event_timestamp_4024() {
    let event = Event::new("evt-4024".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4025() {
    let event = Event::new("evt-4025".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4026() {
    let event = Event::new("evt-4026".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4027() {
    let event = Event::new("evt-4027".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4028() {
    let event = Event::new("evt-4028".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4028"));
}

#[test]
fn test_event_timestamp_4029() {
    let event = Event::new("evt-4029".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4030() {
    let event = Event::new("evt-4030".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4031() {
    let event = Event::new("evt-4031".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4032() {
    let event = Event::new("evt-4032".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4033() {
    let event = Event::new("evt-4033".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4033"));
}

#[test]
fn test_event_timestamp_4034() {
    let event = Event::new("evt-4034".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4035() {
    let event = Event::new("evt-4035".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4036() {
    let event = Event::new("evt-4036".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4037() {
    let event = Event::new("evt-4037".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4038() {
    let event = Event::new("evt-4038".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4038"));
}

#[test]
fn test_event_timestamp_4039() {
    let event = Event::new("evt-4039".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4040() {
    let event = Event::new("evt-4040".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4041() {
    let event = Event::new("evt-4041".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4042() {
    let event = Event::new("evt-4042".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4043() {
    let event = Event::new("evt-4043".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4043"));
}

#[test]
fn test_event_timestamp_4044() {
    let event = Event::new("evt-4044".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4045() {
    let event = Event::new("evt-4045".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4046() {
    let event = Event::new("evt-4046".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4047() {
    let event = Event::new("evt-4047".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4048() {
    let event = Event::new("evt-4048".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4048"));
}

#[test]
fn test_event_timestamp_4049() {
    let event = Event::new("evt-4049".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4050() {
    let event = Event::new("evt-4050".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4051() {
    let event = Event::new("evt-4051".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4052() {
    let event = Event::new("evt-4052".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4053() {
    let event = Event::new("evt-4053".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4053"));
}

#[test]
fn test_event_timestamp_4054() {
    let event = Event::new("evt-4054".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4055() {
    let event = Event::new("evt-4055".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4056() {
    let event = Event::new("evt-4056".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4057() {
    let event = Event::new("evt-4057".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4058() {
    let event = Event::new("evt-4058".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4058"));
}

#[test]
fn test_event_timestamp_4059() {
    let event = Event::new("evt-4059".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4060() {
    let event = Event::new("evt-4060".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4061() {
    let event = Event::new("evt-4061".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4062() {
    let event = Event::new("evt-4062".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4063() {
    let event = Event::new("evt-4063".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4063"));
}

#[test]
fn test_event_timestamp_4064() {
    let event = Event::new("evt-4064".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4065() {
    let event = Event::new("evt-4065".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4066() {
    let event = Event::new("evt-4066".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4067() {
    let event = Event::new("evt-4067".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4068() {
    let event = Event::new("evt-4068".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4068"));
}

#[test]
fn test_event_timestamp_4069() {
    let event = Event::new("evt-4069".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4070() {
    let event = Event::new("evt-4070".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4071() {
    let event = Event::new("evt-4071".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4072() {
    let event = Event::new("evt-4072".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4073() {
    let event = Event::new("evt-4073".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4073"));
}

#[test]
fn test_event_timestamp_4074() {
    let event = Event::new("evt-4074".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4075() {
    let event = Event::new("evt-4075".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4076() {
    let event = Event::new("evt-4076".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4077() {
    let event = Event::new("evt-4077".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4078() {
    let event = Event::new("evt-4078".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4078"));
}

#[test]
fn test_event_timestamp_4079() {
    let event = Event::new("evt-4079".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4080() {
    let event = Event::new("evt-4080".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4081() {
    let event = Event::new("evt-4081".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4082() {
    let event = Event::new("evt-4082".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4083() {
    let event = Event::new("evt-4083".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4083"));
}

#[test]
fn test_event_timestamp_4084() {
    let event = Event::new("evt-4084".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4085() {
    let event = Event::new("evt-4085".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4086() {
    let event = Event::new("evt-4086".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4087() {
    let event = Event::new("evt-4087".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4088() {
    let event = Event::new("evt-4088".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4088"));
}

#[test]
fn test_event_timestamp_4089() {
    let event = Event::new("evt-4089".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4090() {
    let event = Event::new("evt-4090".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4091() {
    let event = Event::new("evt-4091".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4092() {
    let event = Event::new("evt-4092".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4093() {
    let event = Event::new("evt-4093".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4093"));
}

#[test]
fn test_event_timestamp_4094() {
    let event = Event::new("evt-4094".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4095() {
    let event = Event::new("evt-4095".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4096() {
    let event = Event::new("evt-4096".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4097() {
    let event = Event::new("evt-4097".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4098() {
    let event = Event::new("evt-4098".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4098"));
}

#[test]
fn test_event_timestamp_4099() {
    let event = Event::new("evt-4099".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4100() {
    let event = Event::new("evt-4100".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4101() {
    let event = Event::new("evt-4101".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4102() {
    let event = Event::new("evt-4102".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4103() {
    let event = Event::new("evt-4103".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4103"));
}

#[test]
fn test_event_timestamp_4104() {
    let event = Event::new("evt-4104".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4105() {
    let event = Event::new("evt-4105".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4106() {
    let event = Event::new("evt-4106".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4107() {
    let event = Event::new("evt-4107".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4108() {
    let event = Event::new("evt-4108".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4108"));
}

#[test]
fn test_event_timestamp_4109() {
    let event = Event::new("evt-4109".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4110() {
    let event = Event::new("evt-4110".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4111() {
    let event = Event::new("evt-4111".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4112() {
    let event = Event::new("evt-4112".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4113() {
    let event = Event::new("evt-4113".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4113"));
}

#[test]
fn test_event_timestamp_4114() {
    let event = Event::new("evt-4114".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4115() {
    let event = Event::new("evt-4115".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4116() {
    let event = Event::new("evt-4116".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4117() {
    let event = Event::new("evt-4117".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4118() {
    let event = Event::new("evt-4118".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4118"));
}

#[test]
fn test_event_timestamp_4119() {
    let event = Event::new("evt-4119".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4120() {
    let event = Event::new("evt-4120".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4121() {
    let event = Event::new("evt-4121".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4122() {
    let event = Event::new("evt-4122".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4123() {
    let event = Event::new("evt-4123".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4123"));
}

#[test]
fn test_event_timestamp_4124() {
    let event = Event::new("evt-4124".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4125() {
    let event = Event::new("evt-4125".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4126() {
    let event = Event::new("evt-4126".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4127() {
    let event = Event::new("evt-4127".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4128() {
    let event = Event::new("evt-4128".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4128"));
}

#[test]
fn test_event_timestamp_4129() {
    let event = Event::new("evt-4129".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4130() {
    let event = Event::new("evt-4130".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4131() {
    let event = Event::new("evt-4131".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4132() {
    let event = Event::new("evt-4132".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4133() {
    let event = Event::new("evt-4133".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4133"));
}

#[test]
fn test_event_timestamp_4134() {
    let event = Event::new("evt-4134".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4135() {
    let event = Event::new("evt-4135".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4136() {
    let event = Event::new("evt-4136".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4137() {
    let event = Event::new("evt-4137".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4138() {
    let event = Event::new("evt-4138".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4138"));
}

#[test]
fn test_event_timestamp_4139() {
    let event = Event::new("evt-4139".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4140() {
    let event = Event::new("evt-4140".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4141() {
    let event = Event::new("evt-4141".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4142() {
    let event = Event::new("evt-4142".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4143() {
    let event = Event::new("evt-4143".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4143"));
}

#[test]
fn test_event_timestamp_4144() {
    let event = Event::new("evt-4144".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4145() {
    let event = Event::new("evt-4145".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4146() {
    let event = Event::new("evt-4146".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4147() {
    let event = Event::new("evt-4147".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4148() {
    let event = Event::new("evt-4148".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4148"));
}

#[test]
fn test_event_timestamp_4149() {
    let event = Event::new("evt-4149".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4150() {
    let event = Event::new("evt-4150".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4151() {
    let event = Event::new("evt-4151".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4152() {
    let event = Event::new("evt-4152".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4153() {
    let event = Event::new("evt-4153".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4153"));
}

#[test]
fn test_event_timestamp_4154() {
    let event = Event::new("evt-4154".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4155() {
    let event = Event::new("evt-4155".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4156() {
    let event = Event::new("evt-4156".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4157() {
    let event = Event::new("evt-4157".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4158() {
    let event = Event::new("evt-4158".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4158"));
}

#[test]
fn test_event_timestamp_4159() {
    let event = Event::new("evt-4159".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4160() {
    let event = Event::new("evt-4160".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4161() {
    let event = Event::new("evt-4161".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4162() {
    let event = Event::new("evt-4162".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4163() {
    let event = Event::new("evt-4163".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4163"));
}

#[test]
fn test_event_timestamp_4164() {
    let event = Event::new("evt-4164".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4165() {
    let event = Event::new("evt-4165".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4166() {
    let event = Event::new("evt-4166".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4167() {
    let event = Event::new("evt-4167".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4168() {
    let event = Event::new("evt-4168".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4168"));
}

#[test]
fn test_event_timestamp_4169() {
    let event = Event::new("evt-4169".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4170() {
    let event = Event::new("evt-4170".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4171() {
    let event = Event::new("evt-4171".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4172() {
    let event = Event::new("evt-4172".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4173() {
    let event = Event::new("evt-4173".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4173"));
}

#[test]
fn test_event_timestamp_4174() {
    let event = Event::new("evt-4174".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4175() {
    let event = Event::new("evt-4175".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4176() {
    let event = Event::new("evt-4176".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4177() {
    let event = Event::new("evt-4177".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4178() {
    let event = Event::new("evt-4178".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4178"));
}

#[test]
fn test_event_timestamp_4179() {
    let event = Event::new("evt-4179".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4180() {
    let event = Event::new("evt-4180".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4181() {
    let event = Event::new("evt-4181".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4182() {
    let event = Event::new("evt-4182".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4183() {
    let event = Event::new("evt-4183".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4183"));
}

#[test]
fn test_event_timestamp_4184() {
    let event = Event::new("evt-4184".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4185() {
    let event = Event::new("evt-4185".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4186() {
    let event = Event::new("evt-4186".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4187() {
    let event = Event::new("evt-4187".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4188() {
    let event = Event::new("evt-4188".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4188"));
}

#[test]
fn test_event_timestamp_4189() {
    let event = Event::new("evt-4189".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4190() {
    let event = Event::new("evt-4190".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4191() {
    let event = Event::new("evt-4191".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4192() {
    let event = Event::new("evt-4192".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4193() {
    let event = Event::new("evt-4193".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4193"));
}

#[test]
fn test_event_timestamp_4194() {
    let event = Event::new("evt-4194".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4195() {
    let event = Event::new("evt-4195".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4196() {
    let event = Event::new("evt-4196".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4197() {
    let event = Event::new("evt-4197".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4198() {
    let event = Event::new("evt-4198".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4198"));
}

#[test]
fn test_event_timestamp_4199() {
    let event = Event::new("evt-4199".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4200() {
    let event = Event::new("evt-4200".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4201() {
    let event = Event::new("evt-4201".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4202() {
    let event = Event::new("evt-4202".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4203() {
    let event = Event::new("evt-4203".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4203"));
}

#[test]
fn test_event_timestamp_4204() {
    let event = Event::new("evt-4204".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4205() {
    let event = Event::new("evt-4205".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4206() {
    let event = Event::new("evt-4206".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4207() {
    let event = Event::new("evt-4207".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4208() {
    let event = Event::new("evt-4208".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4208"));
}

#[test]
fn test_event_timestamp_4209() {
    let event = Event::new("evt-4209".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4210() {
    let event = Event::new("evt-4210".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4211() {
    let event = Event::new("evt-4211".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4212() {
    let event = Event::new("evt-4212".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4213() {
    let event = Event::new("evt-4213".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4213"));
}

#[test]
fn test_event_timestamp_4214() {
    let event = Event::new("evt-4214".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4215() {
    let event = Event::new("evt-4215".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4216() {
    let event = Event::new("evt-4216".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4217() {
    let event = Event::new("evt-4217".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4218() {
    let event = Event::new("evt-4218".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4218"));
}

#[test]
fn test_event_timestamp_4219() {
    let event = Event::new("evt-4219".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4220() {
    let event = Event::new("evt-4220".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4221() {
    let event = Event::new("evt-4221".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4222() {
    let event = Event::new("evt-4222".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4223() {
    let event = Event::new("evt-4223".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4223"));
}

#[test]
fn test_event_timestamp_4224() {
    let event = Event::new("evt-4224".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4225() {
    let event = Event::new("evt-4225".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4226() {
    let event = Event::new("evt-4226".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4227() {
    let event = Event::new("evt-4227".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4228() {
    let event = Event::new("evt-4228".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4228"));
}

#[test]
fn test_event_timestamp_4229() {
    let event = Event::new("evt-4229".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4230() {
    let event = Event::new("evt-4230".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4231() {
    let event = Event::new("evt-4231".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4232() {
    let event = Event::new("evt-4232".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4233() {
    let event = Event::new("evt-4233".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4233"));
}

#[test]
fn test_event_timestamp_4234() {
    let event = Event::new("evt-4234".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4235() {
    let event = Event::new("evt-4235".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4236() {
    let event = Event::new("evt-4236".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4237() {
    let event = Event::new("evt-4237".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4238() {
    let event = Event::new("evt-4238".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4238"));
}

#[test]
fn test_event_timestamp_4239() {
    let event = Event::new("evt-4239".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4240() {
    let event = Event::new("evt-4240".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4241() {
    let event = Event::new("evt-4241".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4242() {
    let event = Event::new("evt-4242".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4243() {
    let event = Event::new("evt-4243".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4243"));
}

#[test]
fn test_event_timestamp_4244() {
    let event = Event::new("evt-4244".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4245() {
    let event = Event::new("evt-4245".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4246() {
    let event = Event::new("evt-4246".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4247() {
    let event = Event::new("evt-4247".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4248() {
    let event = Event::new("evt-4248".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4248"));
}

#[test]
fn test_event_timestamp_4249() {
    let event = Event::new("evt-4249".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4250() {
    let event = Event::new("evt-4250".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4251() {
    let event = Event::new("evt-4251".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4252() {
    let event = Event::new("evt-4252".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4253() {
    let event = Event::new("evt-4253".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4253"));
}

#[test]
fn test_event_timestamp_4254() {
    let event = Event::new("evt-4254".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4255() {
    let event = Event::new("evt-4255".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4256() {
    let event = Event::new("evt-4256".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4257() {
    let event = Event::new("evt-4257".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4258() {
    let event = Event::new("evt-4258".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4258"));
}

#[test]
fn test_event_timestamp_4259() {
    let event = Event::new("evt-4259".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4260() {
    let event = Event::new("evt-4260".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4261() {
    let event = Event::new("evt-4261".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4262() {
    let event = Event::new("evt-4262".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4263() {
    let event = Event::new("evt-4263".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4263"));
}

#[test]
fn test_event_timestamp_4264() {
    let event = Event::new("evt-4264".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4265() {
    let event = Event::new("evt-4265".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4266() {
    let event = Event::new("evt-4266".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4267() {
    let event = Event::new("evt-4267".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4268() {
    let event = Event::new("evt-4268".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4268"));
}

#[test]
fn test_event_timestamp_4269() {
    let event = Event::new("evt-4269".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4270() {
    let event = Event::new("evt-4270".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4271() {
    let event = Event::new("evt-4271".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4272() {
    let event = Event::new("evt-4272".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4273() {
    let event = Event::new("evt-4273".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4273"));
}

#[test]
fn test_event_timestamp_4274() {
    let event = Event::new("evt-4274".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4275() {
    let event = Event::new("evt-4275".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4276() {
    let event = Event::new("evt-4276".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4277() {
    let event = Event::new("evt-4277".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4278() {
    let event = Event::new("evt-4278".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4278"));
}

#[test]
fn test_event_timestamp_4279() {
    let event = Event::new("evt-4279".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4280() {
    let event = Event::new("evt-4280".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4281() {
    let event = Event::new("evt-4281".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4282() {
    let event = Event::new("evt-4282".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4283() {
    let event = Event::new("evt-4283".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4283"));
}

#[test]
fn test_event_timestamp_4284() {
    let event = Event::new("evt-4284".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4285() {
    let event = Event::new("evt-4285".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4286() {
    let event = Event::new("evt-4286".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4287() {
    let event = Event::new("evt-4287".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4288() {
    let event = Event::new("evt-4288".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4288"));
}

#[test]
fn test_event_timestamp_4289() {
    let event = Event::new("evt-4289".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4290() {
    let event = Event::new("evt-4290".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4291() {
    let event = Event::new("evt-4291".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4292() {
    let event = Event::new("evt-4292".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4293() {
    let event = Event::new("evt-4293".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4293"));
}

#[test]
fn test_event_timestamp_4294() {
    let event = Event::new("evt-4294".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4295() {
    let event = Event::new("evt-4295".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4296() {
    let event = Event::new("evt-4296".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4297() {
    let event = Event::new("evt-4297".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4298() {
    let event = Event::new("evt-4298".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4298"));
}

#[test]
fn test_event_timestamp_4299() {
    let event = Event::new("evt-4299".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4300() {
    let event = Event::new("evt-4300".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4301() {
    let event = Event::new("evt-4301".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4302() {
    let event = Event::new("evt-4302".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4303() {
    let event = Event::new("evt-4303".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4303"));
}

#[test]
fn test_event_timestamp_4304() {
    let event = Event::new("evt-4304".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4305() {
    let event = Event::new("evt-4305".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4306() {
    let event = Event::new("evt-4306".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4307() {
    let event = Event::new("evt-4307".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4308() {
    let event = Event::new("evt-4308".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4308"));
}

#[test]
fn test_event_timestamp_4309() {
    let event = Event::new("evt-4309".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4310() {
    let event = Event::new("evt-4310".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4311() {
    let event = Event::new("evt-4311".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4312() {
    let event = Event::new("evt-4312".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4313() {
    let event = Event::new("evt-4313".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4313"));
}

#[test]
fn test_event_timestamp_4314() {
    let event = Event::new("evt-4314".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4315() {
    let event = Event::new("evt-4315".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4316() {
    let event = Event::new("evt-4316".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4317() {
    let event = Event::new("evt-4317".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4318() {
    let event = Event::new("evt-4318".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4318"));
}

#[test]
fn test_event_timestamp_4319() {
    let event = Event::new("evt-4319".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4320() {
    let event = Event::new("evt-4320".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4321() {
    let event = Event::new("evt-4321".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4322() {
    let event = Event::new("evt-4322".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4323() {
    let event = Event::new("evt-4323".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4323"));
}

#[test]
fn test_event_timestamp_4324() {
    let event = Event::new("evt-4324".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4325() {
    let event = Event::new("evt-4325".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4326() {
    let event = Event::new("evt-4326".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4327() {
    let event = Event::new("evt-4327".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4328() {
    let event = Event::new("evt-4328".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4328"));
}

#[test]
fn test_event_timestamp_4329() {
    let event = Event::new("evt-4329".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4330() {
    let event = Event::new("evt-4330".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4331() {
    let event = Event::new("evt-4331".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4332() {
    let event = Event::new("evt-4332".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4333() {
    let event = Event::new("evt-4333".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4333"));
}

#[test]
fn test_event_timestamp_4334() {
    let event = Event::new("evt-4334".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4335() {
    let event = Event::new("evt-4335".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4336() {
    let event = Event::new("evt-4336".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4337() {
    let event = Event::new("evt-4337".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4338() {
    let event = Event::new("evt-4338".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4338"));
}

#[test]
fn test_event_timestamp_4339() {
    let event = Event::new("evt-4339".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4340() {
    let event = Event::new("evt-4340".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4341() {
    let event = Event::new("evt-4341".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4342() {
    let event = Event::new("evt-4342".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4343() {
    let event = Event::new("evt-4343".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4343"));
}

#[test]
fn test_event_timestamp_4344() {
    let event = Event::new("evt-4344".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4345() {
    let event = Event::new("evt-4345".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4346() {
    let event = Event::new("evt-4346".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4347() {
    let event = Event::new("evt-4347".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4348() {
    let event = Event::new("evt-4348".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4348"));
}

#[test]
fn test_event_timestamp_4349() {
    let event = Event::new("evt-4349".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4350() {
    let event = Event::new("evt-4350".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4351() {
    let event = Event::new("evt-4351".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4352() {
    let event = Event::new("evt-4352".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4353() {
    let event = Event::new("evt-4353".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4353"));
}

#[test]
fn test_event_timestamp_4354() {
    let event = Event::new("evt-4354".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4355() {
    let event = Event::new("evt-4355".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4356() {
    let event = Event::new("evt-4356".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4357() {
    let event = Event::new("evt-4357".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4358() {
    let event = Event::new("evt-4358".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4358"));
}

#[test]
fn test_event_timestamp_4359() {
    let event = Event::new("evt-4359".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4360() {
    let event = Event::new("evt-4360".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4361() {
    let event = Event::new("evt-4361".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4362() {
    let event = Event::new("evt-4362".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4363() {
    let event = Event::new("evt-4363".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4363"));
}

#[test]
fn test_event_timestamp_4364() {
    let event = Event::new("evt-4364".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4365() {
    let event = Event::new("evt-4365".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4366() {
    let event = Event::new("evt-4366".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4367() {
    let event = Event::new("evt-4367".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4368() {
    let event = Event::new("evt-4368".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4368"));
}

#[test]
fn test_event_timestamp_4369() {
    let event = Event::new("evt-4369".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4370() {
    let event = Event::new("evt-4370".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4371() {
    let event = Event::new("evt-4371".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4372() {
    let event = Event::new("evt-4372".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4373() {
    let event = Event::new("evt-4373".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4373"));
}

#[test]
fn test_event_timestamp_4374() {
    let event = Event::new("evt-4374".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4375() {
    let event = Event::new("evt-4375".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4376() {
    let event = Event::new("evt-4376".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4377() {
    let event = Event::new("evt-4377".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4378() {
    let event = Event::new("evt-4378".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4378"));
}

#[test]
fn test_event_timestamp_4379() {
    let event = Event::new("evt-4379".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4380() {
    let event = Event::new("evt-4380".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4381() {
    let event = Event::new("evt-4381".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4382() {
    let event = Event::new("evt-4382".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4383() {
    let event = Event::new("evt-4383".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4383"));
}

#[test]
fn test_event_timestamp_4384() {
    let event = Event::new("evt-4384".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4385() {
    let event = Event::new("evt-4385".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4386() {
    let event = Event::new("evt-4386".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4387() {
    let event = Event::new("evt-4387".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4388() {
    let event = Event::new("evt-4388".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4388"));
}

#[test]
fn test_event_timestamp_4389() {
    let event = Event::new("evt-4389".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4390() {
    let event = Event::new("evt-4390".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4391() {
    let event = Event::new("evt-4391".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4392() {
    let event = Event::new("evt-4392".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4393() {
    let event = Event::new("evt-4393".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4393"));
}

#[test]
fn test_event_timestamp_4394() {
    let event = Event::new("evt-4394".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4395() {
    let event = Event::new("evt-4395".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4396() {
    let event = Event::new("evt-4396".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4397() {
    let event = Event::new("evt-4397".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4398() {
    let event = Event::new("evt-4398".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4398"));
}

#[test]
fn test_event_timestamp_4399() {
    let event = Event::new("evt-4399".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4400() {
    let event = Event::new("evt-4400".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4401() {
    let event = Event::new("evt-4401".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4402() {
    let event = Event::new("evt-4402".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4403() {
    let event = Event::new("evt-4403".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4403"));
}

#[test]
fn test_event_timestamp_4404() {
    let event = Event::new("evt-4404".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4405() {
    let event = Event::new("evt-4405".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4406() {
    let event = Event::new("evt-4406".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4407() {
    let event = Event::new("evt-4407".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4408() {
    let event = Event::new("evt-4408".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4408"));
}

#[test]
fn test_event_timestamp_4409() {
    let event = Event::new("evt-4409".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4410() {
    let event = Event::new("evt-4410".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4411() {
    let event = Event::new("evt-4411".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4412() {
    let event = Event::new("evt-4412".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4413() {
    let event = Event::new("evt-4413".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4413"));
}

#[test]
fn test_event_timestamp_4414() {
    let event = Event::new("evt-4414".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4415() {
    let event = Event::new("evt-4415".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4416() {
    let event = Event::new("evt-4416".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4417() {
    let event = Event::new("evt-4417".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4418() {
    let event = Event::new("evt-4418".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4418"));
}

#[test]
fn test_event_timestamp_4419() {
    let event = Event::new("evt-4419".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4420() {
    let event = Event::new("evt-4420".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4421() {
    let event = Event::new("evt-4421".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4422() {
    let event = Event::new("evt-4422".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4423() {
    let event = Event::new("evt-4423".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4423"));
}

#[test]
fn test_event_timestamp_4424() {
    let event = Event::new("evt-4424".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4425() {
    let event = Event::new("evt-4425".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4426() {
    let event = Event::new("evt-4426".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4427() {
    let event = Event::new("evt-4427".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4428() {
    let event = Event::new("evt-4428".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4428"));
}

#[test]
fn test_event_timestamp_4429() {
    let event = Event::new("evt-4429".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4430() {
    let event = Event::new("evt-4430".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4431() {
    let event = Event::new("evt-4431".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4432() {
    let event = Event::new("evt-4432".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4433() {
    let event = Event::new("evt-4433".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4433"));
}

#[test]
fn test_event_timestamp_4434() {
    let event = Event::new("evt-4434".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4435() {
    let event = Event::new("evt-4435".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4436() {
    let event = Event::new("evt-4436".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4437() {
    let event = Event::new("evt-4437".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4438() {
    let event = Event::new("evt-4438".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4438"));
}

#[test]
fn test_event_timestamp_4439() {
    let event = Event::new("evt-4439".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4440() {
    let event = Event::new("evt-4440".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4441() {
    let event = Event::new("evt-4441".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4442() {
    let event = Event::new("evt-4442".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4443() {
    let event = Event::new("evt-4443".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4443"));
}

#[test]
fn test_event_timestamp_4444() {
    let event = Event::new("evt-4444".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4445() {
    let event = Event::new("evt-4445".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4446() {
    let event = Event::new("evt-4446".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4447() {
    let event = Event::new("evt-4447".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4448() {
    let event = Event::new("evt-4448".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4448"));
}

#[test]
fn test_event_timestamp_4449() {
    let event = Event::new("evt-4449".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4450() {
    let event = Event::new("evt-4450".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4451() {
    let event = Event::new("evt-4451".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4452() {
    let event = Event::new("evt-4452".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4453() {
    let event = Event::new("evt-4453".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4453"));
}

#[test]
fn test_event_timestamp_4454() {
    let event = Event::new("evt-4454".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4455() {
    let event = Event::new("evt-4455".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4456() {
    let event = Event::new("evt-4456".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4457() {
    let event = Event::new("evt-4457".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4458() {
    let event = Event::new("evt-4458".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4458"));
}

#[test]
fn test_event_timestamp_4459() {
    let event = Event::new("evt-4459".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4460() {
    let event = Event::new("evt-4460".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4461() {
    let event = Event::new("evt-4461".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4462() {
    let event = Event::new("evt-4462".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4463() {
    let event = Event::new("evt-4463".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4463"));
}

#[test]
fn test_event_timestamp_4464() {
    let event = Event::new("evt-4464".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4465() {
    let event = Event::new("evt-4465".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4466() {
    let event = Event::new("evt-4466".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4467() {
    let event = Event::new("evt-4467".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4468() {
    let event = Event::new("evt-4468".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4468"));
}

#[test]
fn test_event_timestamp_4469() {
    let event = Event::new("evt-4469".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4470() {
    let event = Event::new("evt-4470".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4471() {
    let event = Event::new("evt-4471".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4472() {
    let event = Event::new("evt-4472".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4473() {
    let event = Event::new("evt-4473".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4473"));
}

#[test]
fn test_event_timestamp_4474() {
    let event = Event::new("evt-4474".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4475() {
    let event = Event::new("evt-4475".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4476() {
    let event = Event::new("evt-4476".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4477() {
    let event = Event::new("evt-4477".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4478() {
    let event = Event::new("evt-4478".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4478"));
}

#[test]
fn test_event_timestamp_4479() {
    let event = Event::new("evt-4479".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4480() {
    let event = Event::new("evt-4480".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4481() {
    let event = Event::new("evt-4481".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4482() {
    let event = Event::new("evt-4482".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4483() {
    let event = Event::new("evt-4483".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4483"));
}

#[test]
fn test_event_timestamp_4484() {
    let event = Event::new("evt-4484".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4485() {
    let event = Event::new("evt-4485".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4486() {
    let event = Event::new("evt-4486".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4487() {
    let event = Event::new("evt-4487".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4488() {
    let event = Event::new("evt-4488".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4488"));
}

#[test]
fn test_event_timestamp_4489() {
    let event = Event::new("evt-4489".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4490() {
    let event = Event::new("evt-4490".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4491() {
    let event = Event::new("evt-4491".to_string(), ProtocolType::Http)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4492() {
    let event = Event::new("evt-4492".to_string(), ProtocolType::Ws);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4493() {
    let event = Event::new("evt-4493".to_string(), ProtocolType::Tcp);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4493"));
}

#[test]
fn test_event_timestamp_4494() {
    let event = Event::new("evt-4494".to_string(), ProtocolType::Udp);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4495() {
    let event = Event::new("evt-4495".to_string(), ProtocolType::Grpc);
    assert_eq!(event.latency_us, 0);
}

#[test]
fn test_event_with_payload_4496() {
    let event = Event::new("evt-4496".to_string(), ProtocolType::Quic)
        .with_payload("payload data".to_string());
    assert!(event.payload.is_some());
}

#[test]
fn test_event_clone_4497() {
    let event = Event::new("evt-4497".to_string(), ProtocolType::Ebpf);
    let cloned = event.clone();
    assert_eq!(event.id, cloned.id);
}

#[test]
fn test_event_debug_4498() {
    let event = Event::new("evt-4498".to_string(), ProtocolType::Mqtt);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("evt-4498"));
}

#[test]
fn test_event_timestamp_4499() {
    let event = Event::new("evt-4499".to_string(), ProtocolType::Modbus);
    assert!(event.timestamp_us > 0);
}

#[test]
fn test_event_latency_4500() {
    let event = Event::new("evt-4500".to_string(), ProtocolType::Other);
    assert_eq!(event.latency_us, 0);
}


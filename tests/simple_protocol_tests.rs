// Simple ProtocolType Tests (3501-4000)

use fluxx_agent::models::event::ProtocolType;

#[test]
fn test_protocol_equality_3501() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3502() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3503() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3504() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3505() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3506() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3507() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3508() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3509() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3510() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3511() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3512() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3513() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3514() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3515() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3516() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3517() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3518() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3519() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3520() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3521() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3522() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3523() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3524() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3525() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3526() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3527() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3528() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3529() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3530() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3531() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3532() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3533() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3534() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3535() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3536() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3537() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3538() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3539() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3540() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3541() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3542() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3543() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3544() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3545() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3546() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3547() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3548() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3549() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3550() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3551() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3552() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3553() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3554() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3555() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3556() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3557() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3558() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3559() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3560() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3561() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3562() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3563() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3564() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3565() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3566() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3567() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3568() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3569() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3570() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3571() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3572() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3573() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3574() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3575() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3576() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3577() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3578() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3579() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3580() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3581() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3582() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3583() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3584() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3585() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3586() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3587() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3588() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3589() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3590() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3591() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3592() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3593() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3594() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3595() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3596() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3597() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3598() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3599() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3600() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3601() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3602() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3603() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3604() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3605() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3606() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3607() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3608() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3609() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3610() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3611() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3612() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3613() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3614() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3615() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3616() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3617() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3618() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3619() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3620() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3621() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3622() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3623() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3624() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3625() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3626() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3627() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3628() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3629() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3630() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3631() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3632() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3633() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3634() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3635() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3636() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3637() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3638() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3639() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3640() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3641() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3642() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3643() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3644() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3645() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3646() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3647() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3648() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3649() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3650() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3651() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3652() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3653() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3654() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3655() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3656() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3657() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3658() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3659() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3660() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3661() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3662() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3663() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3664() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3665() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3666() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3667() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3668() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3669() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3670() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3671() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3672() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3673() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3674() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3675() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3676() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3677() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3678() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3679() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3680() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3681() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3682() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3683() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3684() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3685() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3686() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3687() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3688() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3689() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3690() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3691() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3692() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3693() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3694() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3695() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3696() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3697() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3698() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3699() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3700() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3701() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3702() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3703() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3704() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3705() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3706() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3707() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3708() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3709() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3710() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3711() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3712() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3713() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3714() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3715() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3716() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3717() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3718() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3719() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3720() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3721() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3722() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3723() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3724() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3725() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3726() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3727() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3728() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3729() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3730() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3731() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3732() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3733() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3734() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3735() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3736() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3737() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3738() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3739() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3740() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3741() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3742() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3743() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3744() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3745() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3746() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3747() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3748() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3749() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3750() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3751() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3752() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3753() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3754() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3755() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3756() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3757() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3758() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3759() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3760() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3761() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3762() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3763() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3764() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3765() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3766() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3767() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3768() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3769() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3770() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3771() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3772() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3773() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3774() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3775() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3776() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3777() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3778() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3779() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3780() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3781() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3782() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3783() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3784() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3785() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3786() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3787() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3788() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3789() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3790() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3791() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3792() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3793() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3794() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3795() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3796() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3797() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3798() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3799() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3800() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3801() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3802() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3803() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3804() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3805() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3806() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3807() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3808() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3809() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3810() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3811() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3812() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3813() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3814() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3815() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3816() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3817() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3818() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3819() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3820() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3821() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3822() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3823() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3824() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3825() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3826() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3827() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3828() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3829() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3830() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3831() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3832() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3833() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3834() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3835() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3836() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3837() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3838() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3839() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3840() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3841() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3842() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3843() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3844() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3845() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3846() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3847() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3848() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3849() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3850() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3851() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3852() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3853() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3854() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3855() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3856() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3857() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3858() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3859() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3860() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3861() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3862() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3863() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3864() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3865() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3866() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3867() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3868() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3869() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3870() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3871() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3872() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3873() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3874() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3875() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3876() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3877() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3878() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3879() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3880() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3881() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3882() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3883() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3884() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3885() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3886() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3887() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3888() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3889() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3890() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3891() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3892() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3893() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3894() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3895() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3896() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3897() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3898() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3899() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3900() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3901() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3902() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3903() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3904() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3905() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3906() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3907() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3908() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3909() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3910() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3911() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3912() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3913() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3914() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3915() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3916() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3917() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3918() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3919() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3920() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3921() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3922() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3923() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3924() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3925() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3926() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3927() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3928() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3929() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3930() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3931() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3932() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3933() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3934() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3935() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3936() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3937() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3938() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3939() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3940() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3941() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3942() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3943() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3944() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3945() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3946() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3947() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3948() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3949() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3950() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3951() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3952() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3953() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3954() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3955() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3956() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3957() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3958() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3959() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3960() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3961() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3962() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3963() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3964() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3965() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3966() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3967() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3968() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3969() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3970() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3971() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3972() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3973() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3974() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3975() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3976() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3977() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3978() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3979() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3980() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3981() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3982() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3983() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3984() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3985() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3986() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3987() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3988() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3989() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3990() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3991() {
    let p1 = ProtocolType::Http;
    let p2 = ProtocolType::Http;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3992() {
    let p = ProtocolType::Ws;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3993() {
    let p1 = ProtocolType::Tcp;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3994() {
    let p = ProtocolType::Udp;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_3995() {
    let p = ProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_protocol_equality_3996() {
    let p1 = ProtocolType::Quic;
    let p2 = ProtocolType::Quic;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_clone_3997() {
    let p = ProtocolType::Ebpf;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn test_protocol_copy_3998() {
    let p1 = ProtocolType::Mqtt;
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_protocol_debug_3999() {
    let p = ProtocolType::Modbus;
    let debug_str = format!("{:?}", p);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_serialize_4000() {
    let p = ProtocolType::Other;
    let json = serde_json::to_string(&p).unwrap();
    assert!(!json.is_empty());
}


#![allow(dead_code)]
use serde_json::Value;
use tokio::io::AsyncReadExt;
use super::models::{MessageType, ScanResult};


pub fn start(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running messaging server on 127.0.0.1:{port}");
    Ok(())
}


async fn process<RW>(mut socket: RW) -> Result<Vec<String>, Box<dyn std::error::Error>>
where
    RW: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    let mut buf = vec![0; 1024];
    let mut status_msgs = Vec::new();

    loop {
        let n = socket.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        let msg = String::from_utf8_lossy(&buf[..n]);
        match classify_message(msg.trim()) {
            Ok(classified_message) => {
                match classified_message {
                    MessageType::NetworkScanResult(ScanResult::Hosts(hosts)) => {
                        let msg = format!("Received network scan result with hosts: {:?}", hosts);
                        status_msgs.push(msg);
                    }
                    MessageType::HostScanResult(ScanResult::Ports(ports)) => {
                        let msg = format!("Received host scan result with open ports: {:?}", ports);
                        status_msgs.push(msg);
                    }
                    _ => {
                        let msg = "Received unhandled message type".to_string();
                        status_msgs.push(msg);
                    }
                }
            },
            Err(err) => {
                let msg = format!("Message classification failed: {err}");
                status_msgs.push(msg);
            }
        }
    }

    Ok(status_msgs)
}

fn classify_message(msg: &str) -> Result<MessageType, &'static str> {
    let parsed: Value = serde_json::from_str(msg).map_err(|_| "Invalid JSON")?;

    if let Some(msg_type) = parsed["type"].as_str() {
        match ScanResult::try_from(&parsed) {
            Ok(scan_result) => {
                match msg_type {
                    "network_scan" => {
                        Ok(MessageType::NetworkScanResult(scan_result))
                    },
                    "host_scan" => {
                        Ok(MessageType::HostScanResult(scan_result))
                    },
                    _ => Err("Unknown message type"),
                }
            },
            Err(_) => Err("Failed to parse scan result"),
        }
    } else {
        Err("Missing message type")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use tokio_test::io::Builder;

    #[tokio::test]
    async fn test_process_network_scan() {
        let input_data = r#"{"type": "network_scan", "data": ["192.168.0.1", "192.168.0.2"]}"#;
        let mut fake_stream = Builder::new().read(input_data.as_bytes()).build();
        
        match process(&mut fake_stream).await {
            Ok(result) => {
                assert_eq!(result[0], "Received network scan result with hosts: [\"192.168.0.1\", \"192.168.0.2\"]")
            },
            _ => {
                panic!("Failed to process network scan message.")
            },
        }

    }

    #[tokio::test]
    async fn test_process_host_scan() {
        let input_data = r#"{"type": "host_scan", "host": "192.168.0.1", "data": [80, 8080]}"#;
        let mut fake_stream = Builder::new().read(input_data.as_bytes()).build();
        
        match process(&mut fake_stream).await {
            Ok(result) => {
                assert_eq!(result[0], r#"Received host scan result with open ports: {"192.168.0.1": [80, 8080]}"#)
            },
            _ => {
                panic!("Failed to process host scan message.")
            },
        }

    }

    #[test]
    fn test_classify_message_valid_network_scan() {
        let msg = r#"{"type": "network_scan", "data": ["192.168.0.1", "192.168.0.2"]}"#;
        match classify_message(msg) {
            Ok(MessageType::NetworkScanResult(ScanResult::Hosts(hosts))) => {
                assert_eq!(hosts, vec!["192.168.0.1", "192.168.0.2"]);
            }
            _ => {
                panic!("Failed to classify a valid network scan message.")
            },
        }
    }

    #[test]
    fn test_classify_message_valid_host_scan() {
        let msg = r#"{"type": "host_scan", "host": "192.168.0.1", "data": [80, 8080]}"#;
        let mut expected: HashMap<String, Vec<u16>> = HashMap::new();
        expected.insert("192.168.0.1".to_string(), vec![80, 8080]);
        match classify_message(msg) {
            Ok(MessageType::HostScanResult(ScanResult::Ports(host_ports))) => {
                assert_eq!(host_ports, expected);
            }
            _ => panic!("Failed to classify a valid host scan message."),
        }
    }

    #[test]
    fn test_classify_message_unknown_type() {
        let msg = r#"{"type": "unknown", "data": ["something"]}"#;
        match classify_message(msg) {
            Err(err) => {
                assert_eq!(err, "Failed to parse scan result");
            }
            _ => panic!("Failed to handle an unknown message type."),
        }
    }

    #[test]
    fn test_classify_message_missing_type() {
        let msg = r#"{"data": ["192.168.0.1", "192.168.0.2"]}"#;
        match classify_message(msg) {
            Err(err) => {
                assert_eq!(err, "Missing message type");
            }
            _ => panic!("Failed to handle a message with missing type."),
        }
    }

    #[test]
    fn test_classify_message_invalid_json() {
        let msg = r#"{"type: "network_scan", "data": ["192.168.0.1", "192.168.0.2"]}"#; // missing quote
        match classify_message(msg) {
            Err(err) => {
                assert_eq!(err, "Invalid JSON");
            }
            _ => panic!("Failed to handle invalid JSON."),
        }
    }
}

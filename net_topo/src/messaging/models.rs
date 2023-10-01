use serde_json::Value;
use std::convert::TryFrom;
use std::collections::HashMap;


#[derive(Debug)]
pub enum MessageType {
    NetworkScanResult(ScanResult),
    HostScanResult(ScanResult),
}

#[derive(Debug)]
pub enum ScanResult {
    Hosts(Vec<String>),
    Ports(HashMap<String, Vec<u16>>),
}

impl TryFrom<&Value> for ScanResult {
    type Error = &'static str;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(msg_type) = value["type"].as_str() {
            if let Some(data) = value["data"].as_array() {
                match msg_type {
                    "network_scan" => {
                        let hosts = data.iter()
                            .filter_map(|val| val.as_str().map(String::from))
                            .collect::<Vec<String>>();
                        Ok(ScanResult::Hosts(hosts))
                    },
                    "host_scan" => {
                        let host = value["host"].as_str().ok_or("Missing host for host_scan")?;
                        let ports = data.iter()
                            .filter_map(|val| val.as_u64().map(|v| v as u16))
                            .collect::<Vec<u16>>();
                        let mut host_ports: HashMap<String, Vec<u16>> = HashMap::new();
                        host_ports.insert(host.to_string(), ports);
                        Ok(ScanResult::Ports(host_ports))
                    },
                    _ => Err("Unknown message type"),
                }
            } else {
                Err("Missing or invalid data field")
            }
        } else {
            Err("Missing or invalid type field")
        }
    }
}


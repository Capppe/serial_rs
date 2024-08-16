use serde::{Deserialize, Serialize};
use tokio::process::Command;

use crate::utils::get_port_type;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Port {
    pub s_type: Option<PortType>,
    pub name: Option<String>,
    pub label: Option<String>,
    pub protocol: Option<String>,
    pub protocol_label: Option<String>,
    pub properties: Option<Properties>,
    pub hardware_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Properties {
    pub pid: Option<String>,
    pub serial_number: Option<String>,
    pub vid: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PortType {
    Unknown,
    Default,
    Serial,
    USB,
    XRUSB,
    ACM,
    AMA,
    RfComm,
    AP,
}

impl Port {
    pub fn new() -> Self {
        Self {
            s_type: Some(PortType::Unknown),
            name: Some(String::from("")),
            hardware_id: Some("".to_string()),
            label: Some("".to_string()),
            properties: Some(Properties::new()),
            protocol: Some("".to_string()),
            protocol_label: Some("".to_string()),
        }
    }
}

impl Properties {
    pub fn new() -> Self {
        Self {
            pid: Some("".to_string()),
            serial_number: Some("".to_string()),
            vid: Some("".to_string()),
        }
    }
}

pub async fn get_all_available_ports() -> Vec<Port> {
    let mut ports = Vec::new();
    let ports_output = Command::new("sh")
        .arg("-c")
        .arg("ls /dev/tty*")
        .output()
        .await
        .expect("Failed to list ports");

    if ports_output.status.success() {
        let output = String::from_utf8_lossy(&ports_output.stdout);
        for line in output.lines() {
            let mut new_port = Port::new();
            let s_type = line.split("/dev/tty").last();

            new_port.name = Some(line.to_string());
            new_port.s_type = Some(get_port_type(s_type.unwrap().to_string()));

            ports.push(new_port);
        }
    }

    return ports;
}

pub async fn get_port_by_type(t: PortType) -> Vec<Port> {
    let all_ports = get_all_available_ports().await;
    let mut filtered_ports: Vec<Port> = Vec::new();

    for port in all_ports {
        if port.s_type == Some(t) {
            filtered_ports.push(port);
        }
    }
    return filtered_ports;
}

pub async fn get_port_by_name(name: String) -> Vec<Port> {
    let all_ports = get_all_available_ports().await;
    let mut filtered_ports: Vec<Port> = Vec::new();

    for port in all_ports {
        if port.name == Some(name.clone()) {
            filtered_ports.push(port);
        }
    }

    return filtered_ports;
}

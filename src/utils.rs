use crate::ports::PortType;

pub fn get_port_type(t: String) -> PortType {
    if t.starts_with("S") {
        return PortType::Serial;
    } else if t.as_bytes().len() > 0 && t.as_bytes()[0].is_ascii_digit() {
        return PortType::Default;
    } else if t.starts_with("USB") {
        return PortType::USB;
    } else if t.starts_with("XRUSB") {
        return PortType::XRUSB;
    } else if t.starts_with("ACM") {
        return PortType::ACM;
    } else if t.starts_with("AMA") {
        return PortType::AMA;
    } else if t.starts_with("rfcomm") {
        return PortType::RfComm;
    } else if t.starts_with("AP") {
        return PortType::AP;
    } else {
        return PortType::Unknown;
    }
}

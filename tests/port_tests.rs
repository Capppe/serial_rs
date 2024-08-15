extern crate serial_rs;

#[cfg(test)]
mod tests {
    use serial_rs::ports::{get_all_available_ports, get_port_by_type, PortType};

    #[test]
    fn test_list_ports() {
        let output = get_all_available_ports();

        println!("Output: {:?}", output);

        assert!(output.len() > 0);
    }

    #[test]
    fn test_list_ports_by_type() {
        let usb_ports = get_port_by_type(PortType::USB);
        let default_ports = get_port_by_type(PortType::Default);

        for u_p in usb_ports {
            assert!(u_p.s_type.unwrap() == PortType::USB);
        }

        for d_p in default_ports {
            assert!(d_p.s_type.unwrap() == PortType::Default);
        }
    }
}

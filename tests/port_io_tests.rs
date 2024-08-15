extern crate serial_rs;

#[cfg(test)]
mod tests {

    use serial_rs::{
        ports::{Port, PortType},
        ports_io::{read_from_port, write_to_port},
    };

    #[tokio::test]
    async fn test_read_from_port() {
        let (sender, receiver) = async_channel::bounded(1);
        let mut read_port = Port::new();
        read_port.name = Some("/dev/pts/8".to_string());
        read_port.s_type = Some(PortType::Default);

        tokio::spawn(async move {
            let _ = read_from_port(read_port, 9600, sender).await;
        });

        while let Ok(response) = receiver.recv().await {
            println!("Response: {:?}", response);
        }
    }

    #[test]
    fn test_read_to_port() {
        let mut port = Port::new();
        port.name = Some("/dev/pts/9".to_string());
        port.s_type = Some(PortType::Default);

        tokio::spawn(async move {
            let _ = write_to_port(port, b"Test data");
        });
    }
}

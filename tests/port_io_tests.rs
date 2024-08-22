extern crate serial_rs;

#[cfg(test)]
mod tests {

    use serial_rs::{
        ports::{Port, PortType},
        ports_io::{read_from_port, write_to_port},
    };
    use tokio::sync::mpsc::{channel, Receiver, Sender};

    #[tokio::test]
    async fn test_read_from_port() {
        let (sender, mut receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel(1);
        let mut read_port = Port::new();
        read_port.label = Some("/dev/ttyUSB0".to_string());
        read_port.s_type = Some(PortType::Default);

        tokio::spawn(async move {
            println!("Staring..");
            let _ = read_from_port(read_port, 115200, sender).await;
        });

        while let Some(response) = receiver.recv().await {
            let resp = String::from_utf8(response);
            println!("Recvd: {:?}", resp);
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

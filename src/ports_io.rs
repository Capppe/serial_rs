use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
    os::fd::AsRawFd,
};

use async_channel::Sender;
use libc::{cfsetspeed, tcgetattr, tcsetattr, termios, B115200, B9600, TCSANOW};

use crate::ports::Port;

fn set_serial_config(fd: i32, baud_rate: u32) -> io::Result<()> {
    unsafe {
        let mut config: termios = std::mem::zeroed();

        if tcgetattr(fd, &mut config) != 0 {
            return Err(io::Error::last_os_error());
        }

        let baud = match baud_rate {
            9600 => B9600,
            115200 => B115200,
            _ => B9600,
        };
        cfsetspeed(&mut config, baud);

        if tcsetattr(fd, TCSANOW, &config) != 0 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}

pub async fn read_from_port(port: Port, baud_rate: u32, sender: Sender<Vec<u8>>) -> io::Result<()> {
    let mut serial_port = OpenOptions::new()
        .read(true)
        .write(false)
        .open(port.name.clone().unwrap_or("".to_string()))?;

    set_serial_config(serial_port.as_raw_fd(), baud_rate)?;

    println!(
        "Connected to {} with baud rate {}",
        port.name.unwrap_or("".to_string()),
        baud_rate
    );

    let mut buffer = [0; 128];
    loop {
        match serial_port.read(&mut buffer) {
            Ok(n) => {
                if n > 0 {
                    sender.send(buffer[..n].to_vec()).await.unwrap();
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => return Err(e),
        }
    }
}

pub fn write_to_port(port: Port, data: &[u8]) -> io::Result<()> {
    println!("Sending data");
    let mut serial_port = OpenOptions::new()
        .read(false)
        .write(true)
        .open(port.name.unwrap_or("".to_string()))?;

    serial_port.write_all(data)?;

    Ok(())
}

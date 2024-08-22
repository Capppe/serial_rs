use thiserror::Error;

#[derive(Error, Debug)]
pub enum SerialError {
    #[error("Device or resource busy")]
    DeviceBusy,
}

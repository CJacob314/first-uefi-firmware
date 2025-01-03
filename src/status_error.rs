use uefi::prelude::*;
use core::error::Error;
use core::fmt::Display;

#[derive(Debug)]
pub struct StatusError(pub Status, pub Option<uefi::Error>);

impl Display for StatusError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(ref err) = self.1 {
            write!(f, "{}", err)
        } else {
            Ok(())
        }
    }
}

impl Error for StatusError {}

impl From<(uefi::Status, uefi::Error)> for StatusError {
    fn from(value: (uefi::Status, uefi::Error)) -> Self {
        let (status, error) = value;
        Self(status, Some(error))
    }
}


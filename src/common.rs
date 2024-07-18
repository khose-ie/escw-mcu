use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Param,
    PeripheralBusy,
    WaitTimeout,
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Param => write!(f, "Ok"),
            Error::PeripheralBusy => write!(f, "Ok"),
            Error::WaitTimeout => write!(f, "Ok"),
            Error::Unknown => write!(f, "Ok"),
        }
    }
}

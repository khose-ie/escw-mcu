//! Some common definations and types of this trait.

// use core::fmt;

/// The result type for all possible-failure function.
pub type Result<T> = core::result::Result<T, Error>;

/// The error list.
///
/// Every MCU fireware libraries may has their own error types.
/// So every implementation crates need to convert between the real error types and these types.
#[derive(Debug)]
pub enum Error {
    /// Transport a wrong parameters to the C function.
    Param,

    /// The periphery is still busy and can't do the request operation.
    PeripheralBusy,

    /// Some operation over the specfic waitting time.
    WaitTimeout,

    /// Unknown reason errors.
    Unknown,
}

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Error::Param => write!(f, "Ok"),
//             Error::PeripheralBusy => write!(f, "Ok"),
//             Error::WaitTimeout => write!(f, "Ok"),
//             Error::Unknown => write!(f, "Ok"),
//         }
//     }
// }

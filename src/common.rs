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

/// The async transmission kind for some communication peripheral like UART, I2C and so on.
///
/// For async transmission, it means you could not blocking to wait the transmission action completed.
/// You can switch the processor to do other codes when wait for the remote feedback signal or in the
/// time of register operation.
pub enum AsyncKind {
    /// Transmit data in interrupt, it not a real asyn transmission. The processor will also take the
    /// responsibility of the transmission.
    /// But the difference is you cound not be blocked in your code.
    Interrupt,

    /// Transmis data with DAM, it is a real async transmission, DMA will do the transmission.
    Dma,
}

/// The transmission direction.
pub enum TransmitDirection {
    /// MCU send message to another device.
    Send,

    /// MCU receive messages from another device.
    Receive,

    /// Any one of Send or Receive.
    Any,
}

#[derive(PartialEq, Eq)]
/// The state of a transmission action, including sending and receiving.
pub enum TransmitState {
    /// The transmission completed, all data has been transmisted.
    Completed,

    /// Transmited data size has been over half of the buffer.
    Half,

    /// The transmission has been aborted.
    Aborted,

    /// The transmission failed, may be some error occurred.
    Failure,
}

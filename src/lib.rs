//! Trats for RUST to wrap the MCU C libraries.

#![no_std]

pub mod common;
pub mod peripheral;

/// Trait for one MCU chip.
///
/// All types in this trait has combined with peripheral trait.
/// So, when you want to define a components with some peripherals, you can only set this trait as
/// the only one trait bound.
///
/// # Examples
///
/// Define a key with an IO.
///
/// ```rust
/// struct key<T: Mcu> {
///     io: <T::IO>
/// }
///
/// impl key {
///     fn new(io: T::IO) -> Self {
///         key { io }    
///     }
///
///     fn state(&self) -> IoState {
///         self.io.state()
///     }
/// }
/// ```
pub trait Mcu
{
    #[cfg(feature = "io")]
    type Io: peripheral::io::IoDevice;

    #[cfg(feature = "uart")]
    type Uart: peripheral::uart::UartDevice;

    #[cfg(feature = "i2c")]
    type I2cMaster: peripheral::i2c::I2cMasterDevice;

    #[cfg(feature = "i2c")]
    type I2cSlave: peripheral::i2c::I2cSlaveDevice;

    #[cfg(feature = "spi")]
    type Spi: peripheral::spi::SpiDevice;

    #[cfg(feature = "flash")]
    type Flash: peripheral::flash::FlashDevice;
}

//! Trais for peripheral of MCU.
//!
//! Provides the trait for common peripherals including GPIO, UART, SPI, IIC and so on.

#[cfg(feature = "io")]
pub mod io;

#[cfg(feature = "uart")]
pub mod uart;

#[cfg(feature = "i2c")]
pub mod i2c;

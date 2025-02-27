//! Trais for peripheral of MCU.
//!
//! Provides the trait for common peripherals including GPIO, UART, SPI, IIC and so on.

pub mod io;

#[cfg(feature = "uart")]
pub mod uart;

#[cfg(feature = "i2c")]
pub mod i2c;

#[cfg(feature = "spi")]
pub mod spi;

#[cfg(feature = "wdt")]
pub mod wdt;

#[cfg(feature = "flash")]
pub mod flash;

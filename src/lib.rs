#![no_std]

pub mod common;
pub mod peripheral;

pub trait Mcu {
    #[cfg(feature = "io")]
    type Io: peripheral::io::Io;

    #[cfg(feature = "uart")]
    type Uart: peripheral::uart::Uart;
}


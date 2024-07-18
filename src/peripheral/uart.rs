use crate::common::Result;

pub trait Uart {
    type UartNo;

    fn new(uart: Self::UartNo) -> Self;
    fn new_async(uart: Self::UartNo, event: fn(&[u32]));
    fn transmit(&self, data: &[u8], timeout: u32) -> Result<()>;
    fn receive(&self, data: &mut [u8], timeout: u32) -> Result<()>;
    fn async_transmit(&mut self, kind: UartAsyncTransmitKind, data: &[u8]) -> Result<()>;
    fn async_receive(&mut self, kind: UartAsyncTransmitKind, data: &mut [u8]) -> Result<()>;
    fn abort_async_transmit(&mut self, kind: UartAsyncTransmitKind) -> Result<()>;
    fn abort_async_receive(&mut self, kind: UartAsyncTransmitKind) -> Result<()>;
}

pub enum UartAsyncTransmitKind {
    Interrupt,
    Dma,
}

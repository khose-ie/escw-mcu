use crate::common::Result;

pub enum UartEvent
{
    TxHalf,
    TxCompleted,
    TxAborted,
    RxHalf,
    RxCompleted(u16),
    RxAborted,
    TxRxAborted,
    Error,
}

pub type UartEventHandle = fn(UartEvent);

pub trait UartDevice: Sync
{
    fn with_event(&mut self, handle: UartEventHandle);
    fn send(&self, data: &[u8], timeout: u32) -> Result<()>;
    fn receive(&self, data: &mut [u8], timeout: u32) -> Result<u32>;
    fn send_with_interrupt(&self, data: &[u8]) -> Result<()>;
    fn receive_with_interrupt(&self, data: &mut [u8]) -> Result<()>;
    fn send_with_dma(&self, data: &[u8]) -> Result<()>;
    fn receive_with_dma(&self, data: &mut [u8]) -> Result<()>;
    fn abort(&self) -> Result<()>;
    fn abort_send(&self) -> Result<()>;
    fn abort_receive(&self) -> Result<()>;
}

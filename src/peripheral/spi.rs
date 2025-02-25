use crate::common::Result;

pub enum SpiEvent
{
    TxHalf,
    TxCompleted,
    RxHalf,
    RxCompleted,
    TxRxHalf,
    TxRxCompleted,
    TxRxAborted,
    Error,
}

pub type SpiEventHandle = fn(SpiEvent);

pub trait SpiDevice: Sync
{
    fn with_event(&mut self, handle: SpiEventHandle);
    fn send(&self, data: &[u8], timeout: u32) -> Result<()>;
    fn receive(&self, data: &mut [u8], timeout: u32) -> Result<()>;
    fn send_receive(&self, tx_data: &[u8], rx_data: &mut [u8], timeout: u32) -> Result<()>;
    fn send_with_interrupt(&self, data: &[u8]) -> Result<()>;
    fn receive_with_interrupt(&self, data: &mut [u8]) -> Result<()>;
    fn send_receive_with_interrupt(&self, tx_data: &[u8], rx_data: &mut [u8]) -> Result<()>;
    fn send_with_dma(&self, data: &[u8]) -> Result<()>;
    fn receive_with_dma(&self, data: &mut [u8]) -> Result<()>;
    fn send_receive_with_dma(&self, tx_data: &[u8], rx_data: &mut [u8]) -> Result<()>;
    fn abort(&self) -> Result<()>;
}

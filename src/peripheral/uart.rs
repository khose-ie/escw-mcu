use crate::common::Result;

pub trait UartDevice
{
    type Handle;
    type EventCode;

    fn with_event(handle: fn(&mut Self::Handle, Self::EventCode));

    fn transmit(&self, data: &[u8], timeout: u32) -> Result<()>;

    fn receive(&self, data: &mut [u8], timeout: u32) -> Result<u32>;

    fn transmit_async_int(&self, data: &[u8]) -> Result<()>;

    fn receive_async_int(&self, data: &mut [u8]) -> Result<()>;

    fn transmit_async_dma(&self, data: &[u8]) -> Result<()>;

    fn receive_async_dma(&self, data: &mut [u8]) -> Result<()>;

    fn abort(&self) -> Result<()>;

    fn abort_transmit(&self) -> Result<()>;

    fn abort_receive(&self) -> Result<()>;
}

use crate::common::{AsyncKind, Result, TransmitDirection, TransmitState};

pub type UartEventHandle = fn(TransmitDirection, TransmitState, u16);

pub trait Uart {
    type UartNum;

    fn new(uart: Self::UartNum) -> Self;
    fn with_event(&mut self, handle: UartEventHandle);
    fn send(&self, data: &[u8], timeout: u32) -> Result<()>;
    fn receive(&self, data: &mut [u8], timeout: u32) -> Result<u32>;
    fn async_send(&mut self, kind: AsyncKind, data: &[u8]) -> Result<()>;
    fn async_receive(&mut self, kind: AsyncKind, data: &mut [u8]) -> Result<()>;
    fn abort_async(&mut self, directon: TransmitDirection) -> Result<()>;
    fn async_abort_async(&mut self, directon: TransmitDirection) -> Result<()>;
}

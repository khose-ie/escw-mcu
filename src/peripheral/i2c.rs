use crate::common::Result;

pub enum I2cEvent {
    TxCompleted,
    RxCompleted,
    TxRxAborted,
    MemoryWriteCompleted,
    MemoryReadCompleted,
    Awakened((I2cDirection, u16)),
    ListenCompleted,
    Error,
}

pub type I2cEventHandle = fn(I2cEvent);

pub enum I2cDirection {
    Tx,
    Rx,
}

pub trait I2cMasterDevice: Sync {
    type Identifies;

    fn new(i2c: Self::Identifies) -> Self;
    fn with_event(&mut self, handle: I2cEventHandle);
    fn device_state(&self, device: u16, trails: u32, timeout: u32) -> Result<()>;
    fn send(&self, device: u16, data: &[u8], timeout: u32) -> Result<()>;
    fn receive(&self, device: u16, data: &mut [u8], timeout: u32) -> Result<()>;
    fn memory_write(
        &self, device: u16, address: u16, wide: u16, data: &[u8], timeout: u32,
    ) -> Result<()>;
    fn memory_read(
        &self, device: u16, address: u16, wide: u16, data: &mut [u8], timeout: u32,
    ) -> Result<()>;
    fn send_with_interrupt(&self, device: u16, data: &[u8]) -> Result<()>;
    fn receive_with_interrupt(&self, device: u16, data: &mut [u8]) -> Result<()>;
    fn memory_write_with_interrupt(
        &self, device: u16, address: u16, wide: u16, data: &[u8],
    ) -> Result<()>;
    fn memory_read_with_interrupt(
        &self, device: u16, address: u16, wide: u16, data: &mut [u8],
    ) -> Result<()>;
    fn send_with_dma(&self, device: u16, data: &[u8]) -> Result<()>;
    fn receive_with_dma(&self, device: u16, data: &mut [u8]) -> Result<()>;
    fn memory_write_with_dma(
        &self, device: u16, address: u16, wide: u16, data: &[u8],
    ) -> Result<()>;
    fn memory_read_with_dma(
        &self, device: u16, address: u16, wide: u16, data: &mut [u8],
    ) -> Result<()>;
    fn abort(&self, device: u16) -> Result<()>;
}

pub trait I2cSlaveDevice: Sync {
    type Identifies;

    fn new(i2c: Self::Identifies) -> Self;
    fn with_event(&mut self, handle: I2cEventHandle);
    fn listen(&self) -> Result<()>;
    fn send(&self, data: &[u8], timeout: u32) -> Result<()>;
    fn receive(&self, data: &mut [u8], timeout: u32) -> Result<()>;
    fn send_with_interrupt(&self, data: &[u8]) -> Result<()>;
    fn receive_with_interrupt(&self, data: &mut [u8]) -> Result<()>;
    fn send_with_dma(&self, data: &[u8]) -> Result<()>;
    fn receive_with_dma(&self, data: &mut [u8]) -> Result<()>;
}

use crate::common::Result;

pub trait FlashDevice: Sync
{
    fn erase(&self, bank: u32, sector: u32, count: u32) -> Result<()>;
    fn program(&self, address: u32, data: &[u8]) -> Result<()>;
}

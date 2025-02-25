use crate::common::Result;

pub trait WdtDevice: Sync
{
    fn refresh(&self) -> Result<()>;
}

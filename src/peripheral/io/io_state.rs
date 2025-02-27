/// Level state of a GPIO.
#[repr(u32)]
#[derive(PartialEq, Eq)]
pub enum IoState
{
    /// Low level, the value is `0`.
    Reset = 0,

    /// High level, the value is `1`.
    Set = 1,
}

impl From<u32> for IoState
{
    fn from(value: u32) -> Self
    {
        match value {
            0 => Self::Reset,
            _ => Self::Set,
        }
    }
}

impl Into<u32> for IoState
{
    fn into(self) -> u32
    {
        self as u32
    }
}

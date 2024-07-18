pub trait Io: Sync {
    type Port;
    type Pin;

    fn new(port: Self::Port, pin: Self::Pin) -> Self;
    fn state(&self) -> IoState;
    fn write(&self, state: IoState);
    fn toggle(&self);
    fn with_event(&self, handle: fn());
}

#[derive(PartialEq, Eq)]
pub enum IoState {
    Reset,
    Set,
}

impl From<u32> for IoState {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Reset,
            _ => Self::Set,
        }
    }
}

impl Into<u32> for IoState {
    fn into(self) -> u32 {
        match self {
            Self::Reset => 0,
            Self::Set => 1,
        }
    }
}

impl IoState {
    pub fn is(&self, state: IoState) -> bool {
        *self == state
    }
}

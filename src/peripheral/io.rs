// ！ The definations for MCU peripheral GPIO.
// ！
// ！ The definations in this module provide a common interface to operate a GPIO pin.

/// Trait providing operations of an GPIO pin.
pub trait IoDevice: Sync {
    /// Set the callback function for the GPIO level changed interrupt.
    ///
    /// This function only save a function pointer and will not do any interrupt configurations,
    /// please do it in your C code.
    fn with_event(&self, handle: fn());

    /// Get the GPIO pin level state.
    fn state(&self) -> IoState;

    /// Let pin output high or low level, only effective in output mode.
    fn write(&self, state: IoState);

    /// Toggle the outputing pin level, only effective in output mode.
    fn toggle(&self);
}

/// Level state of a GPIO.
#[derive(PartialEq, Eq)]
pub enum IoState {
    /// Low level, the value is `0`.
    Reset,

    /// High level, the value is `1`.
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

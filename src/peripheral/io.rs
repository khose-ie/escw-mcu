// ！ The definations for MCU peripheral GPIO.
// ！
// ！ The definations in this module provide a common interface to operate a GPIO pin.

/// Trait providing operations of an GPIO pin.
pub trait Io: Sync {
    /// The IO port identified ID of one MCU.
    /// For example, for STM32, it is `GPIOA``, the value is the regiter's address.
    type Port;

    /// The pin identified ID of on MCU.
    /// For example, for STM32, it is `PIN0``, the value is `0x00000001``.
    type Pin;

    /// Create an implementation object.
    ///
    /// This function will not initialize the real peripheral with MCU regiter operations.
    /// It only create a object in memory for you to operation the IO.
    /// Please remember to initialize and configurate the IO with MCU regiter operations or firmware
    /// library of the specific MCU in your C code.
    fn new(port: Self::Port, pin: Self::Pin) -> Self;

    /// Get the GPIO pin level state.
    fn state(&self) -> IoState;

    /// Let pin output high or low level, only effective in output mode.
    fn write(&self, state: IoState);

    /// Toggle the outputing pin level, only effective in output mode.
    fn toggle(&self);

    /// Set the callback function for the GPIO level changed interrupt.
    ///
    /// This function only save a function pointer and will not do any interrupt configurations,
    /// please do it in your C code.
    fn with_event(&self, handle: fn());
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

impl IoState {
    /// Check whether the state is same with inputed one.
    pub fn is(&self, state: IoState) -> bool {
        *self == state
    }
}

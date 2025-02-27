// ！ The definations for MCU peripheral GPIO.
// ！
// ！ The definations in this module provide a common interface to operate a GPIO pin.

mod io_state;

pub use io_state::IoState;

/// Trait providing operations of an GPIO pin.
pub trait IoDevice
{
    type Pin;

    /// Set the callback function for the GPIO level changed interrupt.
    ///
    /// This function only save a function pointer and will not do any interrupt configurations,
    /// please do it in your C code.
    fn with_event(&self, handle: fn(Self::Pin));

    /// Get the GPIO pin level state.
    fn state(&self) -> IoState;

    /// Let pin output high or low level, only effective in output mode.
    fn set_state(&self, state: IoState);

    /// Toggle the outputing pin level, only effective in output mode.
    fn toggle(&self);
}

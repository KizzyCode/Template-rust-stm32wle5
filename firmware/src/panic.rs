//! Implements the panic handler

use core::{
    fmt::{self, Write},
    hint,
    panic::PanicInfo,
    ptr::addr_of_mut,
};
use cortex_m::asm;
use cortex_m_rt::ExceptionFrame;

/// A static buffer to hold a formatted panic message
#[doc(hidden)]
pub struct PanicBuffer<const SIZE: usize> {
    /// The panic message
    message: [u8; SIZE],
    /// The size of the panic message
    len: usize,
}
impl<const SIZE: usize> PanicBuffer<SIZE> {
    /// Creates a new empty panic buffer
    pub const fn new() -> Self {
        Self { message: [0; SIZE], len: 0 }
    }
}
impl<const SIZE: usize> Write for PanicBuffer<SIZE> {
    #[inline(never)]
    fn write_str(&mut self, str_: &str) -> fmt::Result {
        // Get the target subbuffer
        let message = &mut self.message[self.len..];
        let to_copy = core::cmp::min(str_.len(), message.len());
        self.len += to_copy;

        // Copy the string using a volatile write to ensure this is not optimized away
        let mut dest = message.as_mut_ptr();
        for source in str_.bytes().take(to_copy) {
            // Volatile write and increment target pointer
            unsafe { dest.write_volatile(source) };
            unsafe { dest = dest.add(1) };
        }
        Ok(())
    }
}
/// The static panic buffers for each core
#[no_mangle]
#[doc(hidden)]
pub static mut PANIC_BUFFER: PanicBuffer<512> = PanicBuffer::new();

/// The panic handler
#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Write the panic info into the buffer
    let buffer = unsafe { &mut *addr_of_mut!(PANIC_BUFFER) };
    let _write_ok = write!(buffer, "{info}").is_ok();

    // Trigger a breakpoint, ensure `buffer` is not optimized away and raise a fatal exception
    asm::bkpt();
    hint::black_box(buffer);
    asm::udf();
}
/// Exception handler for uncaught interrupts (useful for debugging, can be overridden to e.g. reboot)
#[stm32wlxx_hal::cortex_m_rt::exception]
#[allow(non_snake_case)]
unsafe fn DefaultHandler(irqn: i16) {
    loop {
        // Trigger breakpoint and ensure `irqn` is not optimized away
        asm::bkpt();
        hint::black_box(irqn);
    }
}
/// Exception handler if hard fault occurs (useful for debugging, can be overridden to e.g. reboot)
#[stm32wlxx_hal::cortex_m_rt::exception]
#[allow(non_snake_case)]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    loop {
        // Trigger breakpoint and ensure `ef` is not optimized away
        asm::bkpt();
        hint::black_box(ef);
    }
}

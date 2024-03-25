//! Implements the panic handler

use core::{
    fmt::{self, Write},
    hint,
    panic::PanicInfo,
    ptr::addr_of_mut,
};
use cortex_m::asm;
use cortex_m_rt::ExceptionFrame;

/// A static buffer to hold a formatted panic message; can be used for interactive debugging or )
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
        let buf = &mut self.message[self.len..];
        let to_copy = core::cmp::min(str_.len(), buf.len());
        self.len += to_copy;

        // Use a volatile copy to ensure the message is always written into the static memory
        let mut buf = buf.as_mut_ptr();
        for byte in str_.bytes().take(to_copy) {
            // Volatile write and increment target pointer
            unsafe { buf.write_volatile(byte) };
            unsafe { buf = buf.add(1) };
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

    // Trigger debug breakpoint, try to ensure `buffer` is not optimized away, and raise a fatal exception
    asm::bkpt();
    hint::black_box(buffer);
    asm::udf();
}
/// Exception handler for uncaught interrupts
#[stm32wlxx_hal::cortex_m_rt::exception]
#[allow(non_snake_case)]
unsafe fn DefaultHandler(irqn: i16) {
    // Trigger debug breakpoint and try to ensure `irqn` is not optimized away
    asm::bkpt();
    hint::black_box(irqn);
    asm::udf();
}
/// Exception handler if hard fault occurs
#[stm32wlxx_hal::cortex_m_rt::exception]
#[allow(non_snake_case)]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    // Trigger debug breakpoint and try to ensure `ef` is not optimized away
    asm::bkpt();
    hint::black_box(ef);
    asm::udf();
}

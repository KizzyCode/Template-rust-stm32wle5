#![no_std]
#![no_main]

mod hardware;
mod panic;

use crate::hardware::Hardware;

#[stm32wlxx_hal::cortex_m_rt::entry]
fn main() -> ! {
    // Grab our hardware objects
    let Hardware { mut delay } = Hardware::init().expect("failed to initialize hardware");

    // Sleep 5s, then panic
    delay.delay_ms(5000);
    panic!("Testolope");
}

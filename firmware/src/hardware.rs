//! Provides access handles for hardware/peripheral access

use core::mem;
use cortex_m::{delay::Delay, Peripherals as CorePeripherals};
use stm32wlxx_hal::{pac::Peripherals, util};

/// The userdata pages
#[link_section = ".userdata"]
#[allow(unused)]
pub static USERDATA: [u64; 1024] = [u64::MAX; 1024];
/// Assert that the flash page has the correct size
const _USERDATA_ASSERT_SIZE: () = assert!(mem::size_of::<[u64; 1024]>() == 8192, "Invalid size of userdata");
/// Assert that the flash page has the correct alignment
const _USERDATA_ASSERT_ALIGN: () = assert!(mem::align_of::<[u64; 1024]>() == 8, "Invalid alignment of userdata");

/// A hardware handle
pub struct Hardware {
    /// System timer (SysTick) as a delay provider
    pub delay: Delay,
}
impl Hardware {
    /// Initializes the required hardware
    pub fn init() -> Option<Self> {
        // Destructure peripherals
        let Peripherals { RCC, .. } = Peripherals::take()?;
        let CorePeripherals { SYST, .. } = CorePeripherals::take()?;

        // Setup delay
        let delay = util::new_delay(SYST, &RCC);
        Some(Self { delay })
    }
}

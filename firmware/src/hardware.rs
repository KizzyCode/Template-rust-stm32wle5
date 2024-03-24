//! Provides access handles for hardware/peripheral access

use core::mem;
use cortex_m::{delay::Delay, Peripherals as CorePeripherals};
use stm32wlxx_hal::{pac::Peripherals, util};

/// A flash page
#[repr(align(2048))]
#[derive(Debug, Clone, Copy)]
pub struct FlashPage {
    /// The flash bytes
    pub bytes: [u8; 2048],
}
impl FlashPage {
    /// Creates a new all-`0xFF` flash page
    pub const fn new() -> Self {
        Self { bytes: [0xFF; 2048] }
    }
}
/// Assert that the flash page has the correct size
const _FLASHPAGE_ASSERT_SIZE: () = assert!(mem::size_of::<FlashPage>() == 2048, "Invalid size of flash page");
/// The userdata pages
#[link_section = ".userdata"]
#[allow(unused)]
pub static USERDATA: [FlashPage; 4] = [FlashPage::new(); 4];

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

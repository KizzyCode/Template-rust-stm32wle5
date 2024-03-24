[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![docs.rs](https://docs.rs/FIXME/badge.svg)](https://docs.rs/FIXME)
[![crates.io](https://img.shields.io/crates/v/FIXME.svg)](https://crates.io/crates/FIXME)
[![Download numbers](https://img.shields.io/crates/d/FIXME.svg)](https://crates.io/crates/FIXME)
[![dependency status](https://deps.rs/crate/FIXME/latest/status.svg)](https://deps.rs/crate/FIXME)


# `STM32WLE5` Rust template
Welcome to `STM32WLE5` 🎉

This is a template for `STM32WLE5` firmwares written in Rust.

FIXME: Replace this ↑ with the firmware description after applying the template.


## Troubleshooting
The STM32WLE5 can be a bit special. If you encounter problems, ensure you have disabled the readout protection and
that OpenOCD is configured appropriately:

### Disabling the readout protection
If you get an error like `Error connecting DP: cannot read IDR` or have troubles accessing the flash etc, this is likely
caused by a firmware that disables debugging and an STM feature called readout protection. This feature is designed to
protect deployed code and data against readout, and needs to be disabled to gain reasonable debug access.

#### Disabling the readout protection using OpenOCD
1. Connect OpenOCD to your STM32WLE5 board
   1. Ensure `RST` is pulled down to ground (consult your datasheet to choose the appropriate resistor!)
   2. Upon successful connection, disconnect `RST` again
   3. Halt the chip to gain full command access: `halt`
2. List all flash banks to get their numbers: `flash banks`
3. Check if readout protection is enabled: `stm32l4x unlock <num>`
   1. `RDP level 1` means that the memory bank is protected
4. Unlock the memory bank (this performs a mass-erase): `stm32l4x unlock <num>`
   1. To apply the change immediately: `stm32l4x option_load <num>` (note that indicated errors are normal)
   2. Probe the bank again: `flash probe <num>` (`RDP level 0 (0xAA)` means unprotected)
5. Powercycle the chip

### Configuring OpenOCD
Make sure, the following OpenODC options are set:
- `set CONNECT_UNDER_RESET 1`
- `set CORE_RESET 0`
- `set AP_NUM 0`
- `set STOP_WATCHDOG 1`
- `set WORKAREASIZE 0x8000`
- `set ENABLE_LOW_POWER 1`

### Ensure your `memory.x` maps your board
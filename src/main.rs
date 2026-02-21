#![no_std]
#![no_main]

use core::ptr::write_volatile;
use cortex_m;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Addresses for PIN_CNF registers
    const PIN_CNF_21_REG: *mut u32 = 0x5000_0754 as *mut u32;
    const PIN_CNF_28_REG: *mut u32 = 0x5000_0770 as *mut u32;

    // Addresses for the OUT registers according to the reference manual
    const PORT0_OUT_REGISTER: *mut u32 = 0x5000_0504 as *mut u32;
    const LED_DRIVER_DIR_OUT: u32 = 0;
    const OUT_REG_LED_DRIVE_BIT: u32 = 1 << LED_DRIVER_DIR_OUT;

    unsafe {
        // Configure Pins 21 and 28 as output
        // write_volatile is used to prevent the compiler from mis-optimization
        write_volatile(PIN_CNF_21_REG, OUT_REG_LED_DRIVE_BIT);
        write_volatile(PIN_CNF_28_REG, OUT_REG_LED_DRIVE_BIT);
    }

    let mut led_on = true;
    loop {
        // Bootleg busy wait
        for _ in 0..400_000 {
            cortex_m::asm::nop();
        }

        unsafe {
            match led_on {
                true => *PORT0_OUT_REGISTER |= 1 << 21,     // Set row1 high
                false => *PORT0_OUT_REGISTER &= !(1 << 21), // Set row1 low
            }
        }

        led_on = !led_on;
    }
}

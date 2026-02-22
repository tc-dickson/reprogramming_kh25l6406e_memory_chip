#![no_std]
#![no_main]

use cortex_m;
use cortex_m_rt::entry;
use nrf52833_pac;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Configure pins 21 and 28 as output
    let p = nrf52833_pac::Peripherals::take().unwrap();

    // Set pins 21 and 28 as outputs
    p.P0.pin_cnf[21].write(|w| w.dir().output());
    p.P0.pin_cnf[28].write(|w| w.dir().output());

    // Set pin 21 as high (top left LED)
    p.P0.out.write(|w| w.pin21().high());

    let mut led_on = true;
    loop {
        for _ in 0..400_000 {
            cortex_m::asm::nop();
        }

        match led_on {
            true => p.P0.out.write(|w| w.pin21().high()),
            false => p.P0.out.write(|w| w.pin21().low()),
        }

        led_on = !led_on;
    }
}

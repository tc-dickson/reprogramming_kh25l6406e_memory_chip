#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
use cortex_m_rt::entry;
use nrf52833_hal::gpio;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = nrf52833_hal::pac::Peripherals::take().unwrap();

    let port0 = nrf52833_hal::gpio::p0::Parts::new(p.P0);
    let mut led = port0.p0_21.into_push_pull_output(gpio::Level::Low);
    let _ = port0.p0_28.into_push_pull_output(gpio::Level::Low);
 
    
    let mut led_on = true;
    loop {
        for _ in 0..400_000 {
            cortex_m::asm::nop();
        }

        match led_on {
            true => led.set_high(),
            false => led.set_low(),
        };

        led_on = !led_on;
    }
}

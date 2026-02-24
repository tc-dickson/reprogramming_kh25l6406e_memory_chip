#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{Board, hal::Timer };

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    let _ = board.display_pins.col1.set_low();
    let mut led = board.display_pins.row1;

    let mut timer = Timer::new(board.TIMER0);

    loop {
        let _ = led.set_low();
        timer.delay_ms(1_000);
        let _ = led.set_high();
        timer.delay_ms(1_000);
    }
}

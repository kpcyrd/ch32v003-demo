#![no_std]
#![no_main]

use ch32_hal::{self as hal, delay::Delay};
use panic_halt as _;
use qingke::riscv;

#[qingke_rt::entry]
fn main() -> ! {
    // hal::debug::SDIPrint::enable();
    let _pac = hal::init(hal::Config::default());
    let mut delay = Delay;

    // Setup a counter
    let mut ctr = 0u32;

    loop {
        // Print the counter and increment
        hal::println!("hello world: {ctr}");
        ctr = ctr.saturating_add(1);

        // Sleep
        delay.delay_ms(100);

        // Wait for interrupt
        // riscv::asm::wfi();
        // Demo some other function so the import is not unused
        riscv::asm::nop();
    }
}

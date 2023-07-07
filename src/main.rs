#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate microbit;

use panic_rtt_target as _;
use rtt_target::rprintln;
use cortex_m_rt::entry;
    
#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();
    rprintln!("Hello, world!");
    let _y;
    let x = 42;
    _y = x;
    loop {}
}

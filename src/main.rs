#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate microbit;

use panic_rtt_target as _;
use rtt_target::rprintln;
use cortex_m_rt::entry;

use microbit::hal::Timer;


use microbit::{
    hal::twim,
    display::blocking::Display,
    pac::{twim0::frequency::FREQUENCY_A},
};

use lsm303agr::Lsm303agr;

pub mod angle;
use crate::angle::Angle;

pub mod indication;
use crate::indication::Indicator;

const ANGLE_LIMIT : i32 = 30;

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    
    
    /* ----------------- Angle --------------- */
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let acc_sensor = Lsm303agr::new_with_i2c(i2c);
    let mut angel : Angle = Angle::new(acc_sensor);
    /* ----------------- Angle --------------- */

    /* ---------------- Display -------------- */
    let mut indicator = Indicator{ 
        display : Display::new(board.display_pins),
        timer : Timer::new(board.TIMER0),
        angle_limit : ANGLE_LIMIT,
    };

    /* ---------------- Display -------------- */

    rprintln!("Hello, fridge!");

    
    
    loop {
        angel.update();
        let degrees = angel.get_current_angle();
        rprintln!("Current Angle: {}°", degrees);
        indicator.update_display_and_wait(degrees, 500);

    }
}

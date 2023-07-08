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
    hal::prelude::{InputPin,OutputPin},
    display::blocking::Display,
    pac::{twim0::frequency::FREQUENCY_A},
};

pub mod angle;
use crate::angle::Angle;

pub mod indication;
use crate::indication::Indicator;

const ANGLE_LIMIT : u32 = 30;

struct Control {
    // control_pin : dyn OutputPin<Error = dyn Error>,
    // button_a : dyn InputPin<Error = dyn Error>,
    angle_limit : u32,
    indicator: Indicator,
    is_active: bool,
}

impl Control {

    fn check(&mut self, angle: u32){
        if self.is_active {
            self.indicator.update_display_and_wait(angle, 500)
        } else {
            self.indicator.stanby_and_wait(500);
        }

    }
    
}


#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    
    
    /* ----------------- Angle --------------- */
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut angle : Angle = Angle::new(i2c);
    /* ----------------- Angle --------------- */
    
    /* ----------------- Control --------------- */
    let mut control = Control {
        // control_pin : board.pins.p0_02,
        // button_a : board.buttons.button_a,
        angle_limit : ANGLE_LIMIT,
        is_active : false,
        indicator : Indicator{ 
            display : Display::new(board.display_pins),
            timer : Timer::new(board.TIMER0),
            angle_limit : ANGLE_LIMIT,
        }
    };
    let control_pin = board.pins.p0_02;
    // control_pin.set_high();
    let button_a = board.buttons.button_a;
    let button_b = board.buttons.button_b;
    /* ----------------- Control --------------- */
    
    rprintln!("Hello, fridge!");
    
    loop {
        angle.update();
        let degrees = angle.get_current_angle();
        rprintln!("Current Angle: {}°", degrees);
        if let Ok(true) = button_a.is_low(){
            control.is_active = true;
        }
        if let Ok(true) = button_b.is_low(){
            control.is_active = false;
        }
        control.check(degrees);
    }
}

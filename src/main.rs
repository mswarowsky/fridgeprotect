#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate microbit;

use panic_rtt_target as _;
use rtt_target::rprintln;
use cortex_m_rt::entry;

use microbit::hal::Timer;
use micromath::F32Ext;


use microbit::{
    hal::twim,
    display::blocking::Display,
    pac::{twim0::frequency::FREQUENCY_A, TWIM0},
};

/*-------------------------------------------- */ 
use lsm303agr::{
    interface::I2cInterface, mode::MagOneShot, AccelMode, AccelOutputDataRate, Lsm303agr,
};

type Sensor = Lsm303agr<I2cInterface<twim::Twim<TWIM0>>, MagOneShot>;

const PI : f32 = 3.14159265359;
const LSM303AGR_SENSITIVITY : i32 = 988;
const DAMPING_FACTOR : f32 = 0.05;
const ANGLE_LIMIT : i32 = 30;

struct Angle {
    x_damped : f32
}


fn arc2degree(arc : f32) -> f32 {
    return (arc/PI)*180.0
}

fn limit(a : f32, limit: f32) -> f32 {
    if a > limit {
        return limit;
    } else {
        return a;
    }
}

impl Angle {

    pub fn new() -> Self {
        Self { x_damped : 0.0 }
    }

    /* X sensor is on horizontal line, so at 0 zero force is applied to it 
     * so the arcsine calculates the ang
struct Angle {el 
     */
    fn get_current_angle(&self) -> i32{
        let mut acc = self.x_damped / LSM303AGR_SENSITIVITY as f32; 
        acc = limit(acc, 1.0);
        return arc2degree(acc.asin()) as i32;
    }

    fn update(&mut self, sensor: &mut Sensor) {
        loop {
            if sensor.accel_status().unwrap().xyz_new_data {
                let x_raw = sensor.accel_data().unwrap().x as f32;
                self.x_damped = self.x_damped * (1.0 - DAMPING_FACTOR) + x_raw.abs() * DAMPING_FACTOR;
                return;
            }
        }

    }
}
/*-------------------------------------------------*/



const DISP_OK : [[u8;5]; 5]= [
    [0, 0, 0, 1, 0],
    [0, 0, 1, 0, 1],
    [0, 1, 1, 0, 0],
    [0, 1, 0, 0, 0],
    [1, 0, 0, 0, 0],
];


const DISP_FAIL : [[u8;5]; 5]= [
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [1, 0, 0, 0, 1],
];

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    
    let mut angel : Angle = Angle::new();
    /* ---- ANGLE ----------------- */
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut acc_sensor = Lsm303agr::new_with_i2c(i2c);
    match acc_sensor.accelerometer_id() {
        Ok(0x33u8) => {}
        _ => (),
        // _ => defmt::panic!("accelerometer not found"),
    }
    acc_sensor.init().unwrap();
    acc_sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    acc_sensor.set_accel_mode(AccelMode::HighResolution).unwrap();
    /* ---- ANGLE ----------------- */

    /* ---------------- Display -------------- */
    let mut display = Display::new(board.display_pins);
    /* ---------------- Display -------------- */

    rprintln!("Hello, fridge!");

    
    
    loop {
        /* ---- ANGLE ----------------- */
        angel.update(&mut acc_sensor);
        let degrees = angel.get_current_angle();
        rprintln!("Current Angle: {}°", degrees);
        if degrees > ANGLE_LIMIT {
            display.show(&mut timer, DISP_FAIL, 500);    
        } else {
            display.show(&mut timer, DISP_OK, 500);
        }
    }
}

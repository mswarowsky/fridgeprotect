use micromath::F32Ext;

use microbit::{
    hal::twim,
    hal::Twim,
    pac::TWIM0, 
};

use lsm303agr::{
    interface::I2cInterface, mode::MagOneShot, AccelMode, AccelOutputDataRate, Lsm303agr,
};

type AccSensor = Lsm303agr<I2cInterface<twim::Twim<TWIM0>>, MagOneShot>;

const PI : f32 = 3.14159265359;
const LSM303AGR_SENSITIVITY : i32 = 988;
const DAMPING_FACTOR : f32 = 0.05;

pub struct Angle {
sensor: AccSensor,
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
pub fn new(i2c: Twim<TWIM0>) -> Self {
    let mut acc_sensor = Lsm303agr::new_with_i2c(i2c);
 
    match acc_sensor.accelerometer_id() {
        Ok(0x33u8) => {}
        _ => (),
        // _ => defmt::panic!("accelerometer not found"),
    }
    acc_sensor.init().unwrap();
    acc_sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    acc_sensor.set_accel_mode(AccelMode::HighResolution).unwrap();
    Self { sensor: acc_sensor,
        x_damped : 0.0 }
}
    
    /* X sensor is on horizontal line, so at 0 zero force is applied to it 
    * so the arcsine calculates the ang
    struct Angle {el 
        */
pub fn get_current_angle(&self) -> u32{
    let mut acc = self.x_damped / LSM303AGR_SENSITIVITY as f32; 
    acc = limit(acc, 1.0);
    return arc2degree(acc.asin()) as u32;
}
        
pub fn update(&mut self) {
    loop {
        if self.sensor.accel_status().unwrap().xyz_new_data {
            let x_raw = self.sensor.accel_data().unwrap().x as f32;
            self.x_damped = self.x_damped * (1.0 - DAMPING_FACTOR) + x_raw.abs() * DAMPING_FACTOR;
            return;
        }
    }
}
}

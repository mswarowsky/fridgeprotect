use microbit::{
    display::blocking::Display,
    pac::TIMER0, 
    hal::Timer,
};

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
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [1, 0, 0, 0, 1],
];

const DISP_WARN : [[u8;5]; 5]= [
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
];

const DISP_STBY : [[u8;5]; 5]= [
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
];

pub struct Indicator {
    pub display : Display,
    pub timer : Timer<TIMER0>,
    pub angle_limit : u32,
}

impl Indicator {
    pub fn update_display_and_wait(&mut self, angle :u32, duration:  u32){
        if angle > self.angle_limit {
            self.display.show(&mut self.timer, DISP_FAIL, duration);    
        } else  if angle > (self.angle_limit - 5) {
            self.display.show(&mut self.timer, DISP_WARN, duration);
        } else {
            self.display.show(&mut self.timer, DISP_OK, duration);
        }
    }

    pub fn stanby_and_wait(&mut self, duration:  u32) {
        self.display.show(&mut self.timer, DISP_STBY, duration);    
    }
}

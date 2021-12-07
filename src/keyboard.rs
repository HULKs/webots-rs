use webots_bindings::{
    wb_keyboard_disable, wb_keyboard_enable, wb_keyboard_get_key, wb_keyboard_get_sampling_period,
};

pub struct Keyboard;

impl Keyboard {
    pub const END: u32 = 312;
    pub const HOME: u32 = 313;
    pub const LEFT: u32 = 314;
    pub const UP: u32 = 315;
    pub const RIGHT: u32 = 316;
    pub const DOWN: u32 = 317;
    pub const PAGEUP: u32 = 366;
    pub const PAGEDOWN: u32 = 367;
    pub const NUMPAD_HOME: u32 = 375;
    pub const NUMPAD_LEFT: u32 = 376;
    pub const NUMPAD_UP: u32 = 377;
    pub const NUMPAD_RIGHT: u32 = 378;
    pub const NUMPAD_DOWN: u32 = 379;
    pub const NUMPAD_END: u32 = 382;
    pub const KEY: u32 = 0x0000ffff;
    pub const SHIFT: u32 = 0x00010000;
    pub const CONTROL: u32 = 0x00020000;
    pub const ALT: u32 = 0x00040000;

    pub fn enable(&self, sampling_period: i32) {
        unsafe { wb_keyboard_enable(sampling_period) }
    }

    pub fn disable(&self) {
        unsafe { wb_keyboard_disable() }
    }

    pub fn get_sampling_period(&self) -> i32 {
        unsafe { wb_keyboard_get_sampling_period() }
    }

    pub fn get_key(&self) -> u32 {
        unsafe { wb_keyboard_get_key() as u32 }
    }
}

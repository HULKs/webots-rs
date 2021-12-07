use webots_bindings::{
    wb_keyboard_disable, wb_keyboard_enable, wb_keyboard_get_key, wb_keyboard_get_sampling_period,
};

pub struct Keyboard;

impl Keyboard {
    pub fn enable(&self, sampling_period: i32) {
        unsafe { wb_keyboard_enable(sampling_period) }
    }

    pub fn disable(&self) {
        unsafe { wb_keyboard_disable() }
    }

    pub fn get_sampling_period(&self) -> i32 {
        unsafe { wb_keyboard_get_sampling_period() }
    }

    pub fn get_key(&self) -> i32 {
        unsafe { wb_keyboard_get_key() }
    }
}

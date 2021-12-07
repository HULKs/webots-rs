use webots_bindings::{
    wb_keyboard_disable, wb_keyboard_enable, wb_keyboard_get_key, wb_keyboard_get_sampling_period,
};

pub use webots_bindings::{
    WB_KEYBOARD_ALT, WB_KEYBOARD_CONTROL, WB_KEYBOARD_DOWN, WB_KEYBOARD_END, WB_KEYBOARD_HOME,
    WB_KEYBOARD_KEY, WB_KEYBOARD_LEFT, WB_KEYBOARD_NUMPAD_DOWN, WB_KEYBOARD_NUMPAD_END,
    WB_KEYBOARD_NUMPAD_HOME, WB_KEYBOARD_NUMPAD_LEFT, WB_KEYBOARD_NUMPAD_RIGHT,
    WB_KEYBOARD_NUMPAD_UP, WB_KEYBOARD_PAGEDOWN, WB_KEYBOARD_PAGEUP, WB_KEYBOARD_RIGHT,
    WB_KEYBOARD_SHIFT, WB_KEYBOARD_UP,
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

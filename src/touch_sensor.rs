use std::slice::from_raw_parts;

use webots_bindings::{
    wb_device_get_node_type, wb_touch_sensor_disable, wb_touch_sensor_enable,
    wb_touch_sensor_get_lookup_table, wb_touch_sensor_get_lookup_table_size,
    wb_touch_sensor_get_sampling_period, wb_touch_sensor_get_type, wb_touch_sensor_get_value,
    WbDeviceTag, WbNodeType_WB_NODE_TOUCH_SENSOR,
};

pub use webots_bindings::WbTouchSensorType;

pub struct TouchSensor(WbDeviceTag);

impl TouchSensor {
    pub(crate) fn new(device: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_TOUCH_SENSOR, unsafe {
            wb_device_get_node_type(device)
        });
        Self(device)
    }

    pub fn enable(&self, sampling_period: i32) {
        unsafe { wb_touch_sensor_enable(self.0, sampling_period) }
    }

    pub fn disable(&self) {
        unsafe { wb_touch_sensor_disable(self.0) }
    }

    pub fn get_sampling_period(&self) -> i32 {
        unsafe { wb_touch_sensor_get_sampling_period(self.0) }
    }

    pub fn get_value(&self) -> f64 {
        unsafe { wb_touch_sensor_get_value(self.0) }
    }

    pub fn get_lookup_table_size(&self) -> i32 {
        unsafe { wb_touch_sensor_get_lookup_table_size(self.0) }
    }

    pub fn get_lookup_table(&self) -> &[f64] {
        let lookup_table_size = self.get_lookup_table_size();
        unsafe {
            let lookup_table = wb_touch_sensor_get_lookup_table(self.0);
            from_raw_parts(lookup_table, lookup_table_size as usize)
        }
    }

    pub fn get_type(&self) -> WbTouchSensorType {
        unsafe { wb_touch_sensor_get_type(self.0) }
    }
}

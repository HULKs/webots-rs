use std::slice::from_raw_parts;

use thiserror::Error;
use webots_bindings::{
    wb_device_get_node_type, wb_touch_sensor_disable, wb_touch_sensor_enable,
    wb_touch_sensor_get_lookup_table, wb_touch_sensor_get_lookup_table_size,
    wb_touch_sensor_get_sampling_period, wb_touch_sensor_get_type, wb_touch_sensor_get_value,
    wb_touch_sensor_get_values, WbDeviceTag, WbNodeType_WB_NODE_TOUCH_SENSOR,
};

use crate::TouchSensorType;

#[derive(Debug, Error)]
pub enum TouchSensorError {
    #[error("failed to get values: value data is NULL")]
    ValueIsNull,
    #[error("failed to get lookup table: lookup table data is NULL")]
    LookupTableIsNull,
}

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

    pub fn get_values(&self) -> Result<[f64; 3], TouchSensorError> {
        unsafe {
            let values = wb_touch_sensor_get_values(self.0);
            if values.is_null() {
                return Err(TouchSensorError::ValueIsNull);
            }
            Ok([*values.offset(0), *values.offset(1), *values.offset(2)])
        }
    }

    pub fn get_lookup_table_size(&self) -> i32 {
        unsafe { wb_touch_sensor_get_lookup_table_size(self.0) }
    }

    pub fn get_lookup_table(&self) -> Result<&[f64], TouchSensorError> {
        let lookup_table_size = self.get_lookup_table_size();
        unsafe {
            let lookup_table = wb_touch_sensor_get_lookup_table(self.0);
            if lookup_table.is_null() {
                return Err(TouchSensorError::LookupTableIsNull);
            }
            Ok(from_raw_parts(lookup_table, lookup_table_size as usize))
        }
    }

    pub fn get_type(&self) -> TouchSensorType {
        unsafe { wb_touch_sensor_get_type(self.0).into() }
    }
}

use std::slice::from_raw_parts;

use anyhow::bail;
use webots_bindings::{
    wb_device_get_node_type, wb_distance_sensor_disable, wb_distance_sensor_enable,
    wb_distance_sensor_get_aperture, wb_distance_sensor_get_lookup_table,
    wb_distance_sensor_get_lookup_table_size, wb_distance_sensor_get_max_value,
    wb_distance_sensor_get_min_value, wb_distance_sensor_get_sampling_period,
    wb_distance_sensor_get_type, wb_distance_sensor_get_value, WbDeviceTag,
    WbNodeType_WB_NODE_DISTANCE_SENSOR,
};

use crate::DistanceSensorType;

pub struct DistanceSensor(WbDeviceTag);

impl DistanceSensor {
    pub(crate) fn new(device: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_DISTANCE_SENSOR, unsafe {
            wb_device_get_node_type(device)
        });
        Self(device)
    }

    pub fn enable(&self, sampling_period: i32) {
        unsafe { wb_distance_sensor_enable(self.0, sampling_period) }
    }

    pub fn disable(&self) {
        unsafe { wb_distance_sensor_disable(self.0) }
    }

    pub fn get_sampling_period(&self) -> i32 {
        unsafe { wb_distance_sensor_get_sampling_period(self.0) }
    }

    pub fn get_value(&self) -> f64 {
        unsafe { wb_distance_sensor_get_value(self.0) }
    }

    pub fn get_max_value(&self) -> f64 {
        unsafe { wb_distance_sensor_get_max_value(self.0) }
    }

    pub fn get_min_value(&self) -> f64 {
        unsafe { wb_distance_sensor_get_min_value(self.0) }
    }

    pub fn get_aperture(&self) -> f64 {
        unsafe { wb_distance_sensor_get_aperture(self.0) }
    }

    pub fn get_lookup_table_size(&self) -> i32 {
        unsafe { wb_distance_sensor_get_lookup_table_size(self.0) }
    }

    pub fn get_lookup_table(&self) -> anyhow::Result<&[f64]> {
        let lookup_table_size = self.get_lookup_table_size();
        unsafe {
            let lookup_table = wb_distance_sensor_get_lookup_table(self.0);
            if lookup_table.is_null() {
                bail!("Failed to get lookup table: lookup table data is NULL");
            }
            Ok(from_raw_parts(lookup_table, lookup_table_size as usize))
        }
    }

    pub fn get_type(&self) -> DistanceSensorType {
        unsafe { wb_distance_sensor_get_type(self.0).into() }
    }
}

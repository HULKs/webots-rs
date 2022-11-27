use std::slice::from_raw_parts;

use thiserror::Error;
use webots_bindings::{
    wb_device_get_node_type, wb_gyro_disable, wb_gyro_enable, wb_gyro_get_lookup_table,
    wb_gyro_get_lookup_table_size, wb_gyro_get_sampling_period, wb_gyro_get_values, WbDeviceTag,
    WbNodeType_WB_NODE_GYRO,
};

#[derive(Debug, Error)]
pub enum GyroError {
    #[error("failed to get lookup table: lookup table data is NULL")]
    LookupTableIsNull,
    #[error("failed to get values: value data is NULL")]
    ValueIsNull,
}

pub struct Gyro(WbDeviceTag);

impl Gyro {
    pub(crate) fn new(device: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_GYRO, unsafe {
            wb_device_get_node_type(device)
        });
        Self(device)
    }

    pub fn enable(&self, sampling_period: i32) {
        unsafe { wb_gyro_enable(self.0, sampling_period) }
    }

    pub fn disable(&self) {
        unsafe { wb_gyro_disable(self.0) }
    }

    pub fn get_sampling_period(&self) -> i32 {
        unsafe { wb_gyro_get_sampling_period(self.0) }
    }

    pub fn get_lookup_table_size(&self) -> i32 {
        unsafe { wb_gyro_get_lookup_table_size(self.0) }
    }

    pub fn get_lookup_table(&self) -> Result<&[f64], GyroError> {
        let lookup_table_size = self.get_lookup_table_size();
        unsafe {
            let lookup_table = wb_gyro_get_lookup_table(self.0);
            if lookup_table.is_null() {
                return Err(GyroError::LookupTableIsNull);
            }
            Ok(from_raw_parts(lookup_table, lookup_table_size as usize))
        }
    }

    pub fn get_values(&self) -> Result<[f64; 3], GyroError> {
        unsafe {
            let values = wb_gyro_get_values(self.0);
            if values.is_null() {
                return Err(GyroError::ValueIsNull);
            }
            Ok([*values.offset(0), *values.offset(1), *values.offset(2)])
        }
    }
}

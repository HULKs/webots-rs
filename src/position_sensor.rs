use webots_bindings::{
    wb_device_get_node_type, wb_position_sensor_disable, wb_position_sensor_enable,
    wb_position_sensor_get_brake, wb_position_sensor_get_motor,
    wb_position_sensor_get_sampling_period, wb_position_sensor_get_type,
    wb_position_sensor_get_value, WbDeviceTag, WbJointType, WbNodeType_WB_NODE_POSITION_SENSOR,
};

use crate::{Brake, Motor};

pub struct PositionSensor(WbDeviceTag);

impl PositionSensor {
    pub(crate) fn new(device: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_POSITION_SENSOR, unsafe {
            wb_device_get_node_type(device)
        });
        Self(device)
    }

    pub fn enable(&self, sampling_period: i32) {
        unsafe { wb_position_sensor_enable(self.0, sampling_period) }
    }

    pub fn disable(&self) {
        unsafe { wb_position_sensor_disable(self.0) }
    }

    pub fn get_sampling_period(&self) -> i32 {
        unsafe { wb_position_sensor_get_sampling_period(self.0) }
    }

    pub fn get_value(&self) -> f64 {
        unsafe { wb_position_sensor_get_value(self.0) }
    }

    pub fn get_type(&self) -> WbJointType {
        unsafe { wb_position_sensor_get_type(self.0) }
    }

    pub fn get_motor(&self) -> Motor {
        Motor::new(unsafe { wb_position_sensor_get_motor(self.0) })
    }

    pub fn get_brake(&self) -> Brake {
        Brake::new(unsafe { wb_position_sensor_get_brake(self.0) })
    }
}

use webots_bindings::{
    wb_brake_get_motor, wb_brake_get_position_sensor, wb_brake_get_type,
    wb_brake_set_damping_constant, wb_device_get_node_type, WbDeviceTag, WbJointType,
    WbNodeType_WB_NODE_BRAKE,
};

use crate::{Motor, PositionSensor};

pub struct Brake(WbDeviceTag);

impl Brake {
    pub(crate) fn new(device: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_BRAKE, unsafe {
            wb_device_get_node_type(device)
        });
        Self(device)
    }

    pub fn set_damping_constant(&self, damping_constant: f64) {
        unsafe { wb_brake_set_damping_constant(self.0, damping_constant) }
    }

    pub fn get_type(&self) -> WbJointType {
        unsafe { wb_brake_get_type(self.0) }
    }

    pub fn get_motor(&self) -> Motor {
        Motor::new(unsafe { wb_brake_get_motor(self.0) })
    }

    pub fn get_position_sensor(&self) -> PositionSensor {
        PositionSensor::new(unsafe { wb_brake_get_position_sensor(self.0) })
    }
}

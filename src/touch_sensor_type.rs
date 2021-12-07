use webots_bindings::{
    WbTouchSensorType_WB_TOUCH_SENSOR_BUMPER, WbTouchSensorType_WB_TOUCH_SENSOR_FORCE,
    WbTouchSensorType_WB_TOUCH_SENSOR_FORCE3D,
};

pub enum TouchSensorType {
    Bumber,
    Force,
    Force3D,
}

impl From<u32> for TouchSensorType {
    #[allow(non_upper_case_globals)]
    fn from(other: u32) -> Self {
        match other {
            WbTouchSensorType_WB_TOUCH_SENSOR_BUMPER => TouchSensorType::Bumber,
            WbTouchSensorType_WB_TOUCH_SENSOR_FORCE => TouchSensorType::Force,
            WbTouchSensorType_WB_TOUCH_SENSOR_FORCE3D => TouchSensorType::Force3D,
            _ => unreachable!(),
        }
    }
}

impl Into<u32> for TouchSensorType {
    #[allow(non_upper_case_globals)]
    fn into(self) -> u32 {
        match self {
            TouchSensorType::Bumber => WbTouchSensorType_WB_TOUCH_SENSOR_BUMPER,
            TouchSensorType::Force => WbTouchSensorType_WB_TOUCH_SENSOR_FORCE,
            TouchSensorType::Force3D => WbTouchSensorType_WB_TOUCH_SENSOR_FORCE3D,
        }
    }
}

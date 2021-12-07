use webots_bindings::{
    WbDistanceSensorType_WB_DISTANCE_SENSOR_GENERIC,
    WbDistanceSensorType_WB_DISTANCE_SENSOR_INFRA_RED,
    WbDistanceSensorType_WB_DISTANCE_SENSOR_LASER, WbDistanceSensorType_WB_DISTANCE_SENSOR_SONAR,
};

pub enum DistanceSensorType {
    Generic,
    InfraLed,
    Sonar,
    Laser,
}

impl From<u32> for DistanceSensorType {
    #[allow(non_upper_case_globals)]
    fn from(other: u32) -> Self {
        match other {
            WbDistanceSensorType_WB_DISTANCE_SENSOR_GENERIC => DistanceSensorType::Generic,
            WbDistanceSensorType_WB_DISTANCE_SENSOR_INFRA_RED => DistanceSensorType::InfraLed,
            WbDistanceSensorType_WB_DISTANCE_SENSOR_SONAR => DistanceSensorType::Sonar,
            WbDistanceSensorType_WB_DISTANCE_SENSOR_LASER => DistanceSensorType::Laser,
            _ => unreachable!(),
        }
    }
}

impl Into<u32> for DistanceSensorType {
    #[allow(non_upper_case_globals)]
    fn into(self) -> u32 {
        match self {
            DistanceSensorType::Generic => WbDistanceSensorType_WB_DISTANCE_SENSOR_GENERIC,
            DistanceSensorType::InfraLed => WbDistanceSensorType_WB_DISTANCE_SENSOR_INFRA_RED,
            DistanceSensorType::Sonar => WbDistanceSensorType_WB_DISTANCE_SENSOR_SONAR,
            DistanceSensorType::Laser => WbDistanceSensorType_WB_DISTANCE_SENSOR_LASER,
        }
    }
}

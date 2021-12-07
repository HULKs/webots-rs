use webots_bindings::{
    wb_device_get_node_type, wb_motor_disable_force_feedback, wb_motor_disable_torque_feedback,
    wb_motor_enable_force_feedback, wb_motor_enable_torque_feedback, wb_motor_get_acceleration,
    wb_motor_get_available_force, wb_motor_get_available_torque, wb_motor_get_brake,
    wb_motor_get_force_feedback, wb_motor_get_force_feedback_sampling_period,
    wb_motor_get_max_force, wb_motor_get_max_position, wb_motor_get_max_torque,
    wb_motor_get_max_velocity, wb_motor_get_min_position, wb_motor_get_multiplier,
    wb_motor_get_position_sensor, wb_motor_get_target_position, wb_motor_get_torque_feedback,
    wb_motor_get_torque_feedback_sampling_period, wb_motor_get_type, wb_motor_get_velocity,
    wb_motor_set_acceleration, wb_motor_set_available_force, wb_motor_set_available_torque,
    wb_motor_set_control_pid, wb_motor_set_force, wb_motor_set_position, wb_motor_set_torque,
    wb_motor_set_velocity, WbDeviceTag, WbNodeType_WB_NODE_LINEAR_MOTOR,
    WbNodeType_WB_NODE_ROTATIONAL_MOTOR,
};

pub use webots_bindings::WbJointType;

use crate::{Brake, PositionSensor};

pub struct Motor(WbDeviceTag);

impl Motor {
    pub(crate) fn new(device: WbDeviceTag) -> Self {
        assert!({
            let node_type = unsafe { wb_device_get_node_type(device) };
            node_type == WbNodeType_WB_NODE_LINEAR_MOTOR
                || node_type == WbNodeType_WB_NODE_ROTATIONAL_MOTOR
        });
        Self(device)
    }

    pub fn set_position(&self, position: f64) {
        unsafe { wb_motor_set_position(self.0, position) }
    }

    pub fn set_acceleration(&self, acceleration: f64) {
        unsafe { wb_motor_set_acceleration(self.0, acceleration) }
    }

    pub fn set_velocity(&self, velocity: f64) {
        unsafe { wb_motor_set_velocity(self.0, velocity) }
    }

    pub fn set_force(&self, force: f64) {
        unsafe { wb_motor_set_force(self.0, force) }
    }

    pub fn set_torque(&self, torque: f64) {
        unsafe { wb_motor_set_torque(self.0, torque) }
    }

    pub fn set_available_force(&self, available_force: f64) {
        unsafe { wb_motor_set_available_force(self.0, available_force) }
    }

    pub fn set_available_torque(&self, available_torque: f64) {
        unsafe { wb_motor_set_available_torque(self.0, available_torque) }
    }

    pub fn set_control_pid(&self, p: f64, i: f64, d: f64) {
        unsafe { wb_motor_set_control_pid(self.0, p, i, d) }
    }

    pub fn enable_force_feedback(&self, sampling_period: i32) {
        unsafe { wb_motor_enable_force_feedback(self.0, sampling_period) }
    }

    pub fn disable_force_feedback(&self) {
        unsafe { wb_motor_disable_force_feedback(self.0) }
    }

    pub fn get_force_feedback_sampling_period(&self) -> i32 {
        unsafe { wb_motor_get_force_feedback_sampling_period(self.0) }
    }

    pub fn get_force_feedback(&self) -> f64 {
        unsafe { wb_motor_get_force_feedback(self.0) }
    }

    pub fn enable_torque_feedback(&self, sampling_period: i32) {
        unsafe { wb_motor_enable_torque_feedback(self.0, sampling_period) }
    }

    pub fn disable_torque_feedback(&self) {
        unsafe { wb_motor_disable_torque_feedback(self.0) }
    }

    pub fn get_torque_feedback_sampling_period(&self) -> i32 {
        unsafe { wb_motor_get_torque_feedback_sampling_period(self.0) }
    }

    pub fn get_torque_feedback(&self) -> f64 {
        unsafe { wb_motor_get_torque_feedback(self.0) }
    }

    pub fn get_type(&self) -> WbJointType {
        unsafe { wb_motor_get_type(self.0) }
    }

    pub fn get_target_position(&self) -> f64 {
        unsafe { wb_motor_get_target_position(self.0) }
    }

    pub fn get_min_position(&self) -> f64 {
        unsafe { wb_motor_get_min_position(self.0) }
    }

    pub fn get_max_position(&self) -> f64 {
        unsafe { wb_motor_get_max_position(self.0) }
    }

    pub fn get_velocity(&self) -> f64 {
        unsafe { wb_motor_get_velocity(self.0) }
    }

    pub fn get_max_velocity(&self) -> f64 {
        unsafe { wb_motor_get_max_velocity(self.0) }
    }

    pub fn get_acceleration(&self) -> f64 {
        unsafe { wb_motor_get_acceleration(self.0) }
    }

    pub fn get_available_force(&self) -> f64 {
        unsafe { wb_motor_get_available_force(self.0) }
    }

    pub fn get_max_force(&self) -> f64 {
        unsafe { wb_motor_get_max_force(self.0) }
    }

    pub fn get_available_torque(&self) -> f64 {
        unsafe { wb_motor_get_available_torque(self.0) }
    }

    pub fn get_max_torque(&self) -> f64 {
        unsafe { wb_motor_get_max_torque(self.0) }
    }

    pub fn get_multiplier(&self) -> f64 {
        unsafe { wb_motor_get_multiplier(self.0) }
    }

    pub fn get_brake(&self) -> Brake {
        Brake::new(unsafe { wb_motor_get_brake(self.0) })
    }

    pub fn get_position_sensor(&self) -> PositionSensor {
        PositionSensor::new(unsafe { wb_motor_get_position_sensor(self.0) })
    }
}

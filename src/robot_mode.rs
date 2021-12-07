use webots_bindings::{
    WbRobotMode_WB_MODE_CROSS_COMPILATION, WbRobotMode_WB_MODE_REMOTE_CONTROL,
    WbRobotMode_WB_MODE_SIMULATION,
};

pub enum RobotMode {
    Simulation,
    CrossCompilation,
    RemoteControl,
}

impl From<u32> for RobotMode {
    #[allow(non_upper_case_globals)]
    fn from(other: u32) -> Self {
        match other {
            WbRobotMode_WB_MODE_SIMULATION => RobotMode::Simulation,
            WbRobotMode_WB_MODE_CROSS_COMPILATION => RobotMode::CrossCompilation,
            WbRobotMode_WB_MODE_REMOTE_CONTROL => RobotMode::RemoteControl,
            _ => unreachable!(),
        }
    }
}

impl Into<u32> for RobotMode {
    #[allow(non_upper_case_globals)]
    fn into(self) -> u32 {
        match self {
            RobotMode::Simulation => WbRobotMode_WB_MODE_SIMULATION,
            RobotMode::CrossCompilation => WbRobotMode_WB_MODE_CROSS_COMPILATION,
            RobotMode::RemoteControl => WbRobotMode_WB_MODE_REMOTE_CONTROL,
        }
    }
}

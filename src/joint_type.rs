use webots_bindings::{WbJointType_WB_LINEAR, WbJointType_WB_ROTATIONAL};

pub enum JointType {
    Rotational,
    Linear,
}

impl From<u32> for JointType {
    #[allow(non_upper_case_globals)]
    fn from(other: u32) -> Self {
        match other {
            WbJointType_WB_ROTATIONAL => JointType::Rotational,
            WbJointType_WB_LINEAR => JointType::Linear,
            _ => unreachable!(),
        }
    }
}

impl Into<u32> for JointType {
    #[allow(non_upper_case_globals)]
    fn into(self) -> u32 {
        match self {
            JointType::Rotational => WbJointType_WB_ROTATIONAL,
            JointType::Linear => WbJointType_WB_LINEAR,
        }
    }
}

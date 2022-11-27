use std::{ffi::CString, slice::from_raw_parts};

use thiserror::Error;
use webots_bindings::{
    wb_camera_disable, wb_camera_enable, wb_camera_get_exposure, wb_camera_get_focal_distance,
    wb_camera_get_focal_length, wb_camera_get_fov, wb_camera_get_height, wb_camera_get_image,
    wb_camera_get_max_focal_distance, wb_camera_get_max_fov, wb_camera_get_min_focal_distance,
    wb_camera_get_min_fov, wb_camera_get_near, wb_camera_get_sampling_period, wb_camera_get_width,
    wb_camera_has_recognition, wb_camera_save_image, wb_camera_set_exposure,
    wb_camera_set_focal_distance, wb_camera_set_fov, wb_device_get_node_type, WbDeviceTag,
    WbNodeType_WB_NODE_CAMERA,
};

use crate::Recognition;

#[derive(Debug, Error)]
pub enum CameraError {
    #[error("failed to get image: image data is NULL")]
    ImageDataIsNull,
}

pub struct Camera(WbDeviceTag);

impl Camera {
    pub(crate) fn new(device: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_CAMERA, unsafe {
            wb_device_get_node_type(device)
        });
        Self(device)
    }

    pub fn enable(&self, sampling_period: i32) {
        unsafe { wb_camera_enable(self.0, sampling_period) }
    }

    pub fn disable(&self) {
        unsafe { wb_camera_disable(self.0) }
    }

    pub fn get_sampling_period(&self) -> i32 {
        unsafe { wb_camera_get_sampling_period(self.0) }
    }

    pub fn get_image(&self) -> Result<&[u8], CameraError> {
        let width = self.get_width();
        let height = self.get_height();
        unsafe {
            let image = wb_camera_get_image(self.0);
            if image.is_null() {
                return Err(CameraError::ImageDataIsNull);
            }
            Ok(from_raw_parts(image, (width * height * 4) as usize))
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { wb_camera_get_width(self.0) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { wb_camera_get_height(self.0) }
    }

    pub fn get_fov(&self) -> f64 {
        unsafe { wb_camera_get_fov(self.0) }
    }

    pub fn get_max_fov(&self) -> f64 {
        unsafe { wb_camera_get_max_fov(self.0) }
    }

    pub fn get_min_fov(&self) -> f64 {
        unsafe { wb_camera_get_min_fov(self.0) }
    }

    pub fn set_fov(&self, fov: f64) {
        unsafe { wb_camera_set_fov(self.0, fov) }
    }

    pub fn get_exposure(&self) -> f64 {
        unsafe { wb_camera_get_exposure(self.0) }
    }

    pub fn set_exposure(&self, exposure: f64) {
        unsafe { wb_camera_set_exposure(self.0, exposure) }
    }

    pub fn get_focal_length(&self) -> f64 {
        unsafe { wb_camera_get_focal_length(self.0) }
    }

    pub fn get_focal_distance(&self) -> f64 {
        unsafe { wb_camera_get_focal_distance(self.0) }
    }

    pub fn get_max_focal_distance(&self) -> f64 {
        unsafe { wb_camera_get_max_focal_distance(self.0) }
    }

    pub fn get_min_focal_distance(&self) -> f64 {
        unsafe { wb_camera_get_min_focal_distance(self.0) }
    }

    pub fn set_focal_distance(&self, focal_distance: f64) {
        unsafe { wb_camera_set_focal_distance(self.0, focal_distance) }
    }

    pub fn get_near(&self) -> f64 {
        unsafe { wb_camera_get_near(self.0) }
    }

    pub fn save_image(&self, filename: &str, quality: i32) -> i32 {
        let filename = CString::new(filename).expect("CString::new failed");
        unsafe { wb_camera_save_image(self.0, filename.as_ptr(), quality) }
    }

    pub fn has_recognition(&self) -> bool {
        unsafe { wb_camera_has_recognition(self.0) != 0 }
    }

    pub fn get_recognition(&self) -> Recognition {
        Recognition::new(self.0)
    }
}

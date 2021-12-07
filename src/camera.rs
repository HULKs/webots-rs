use std::{ffi::CString, slice::from_raw_parts};

use webots_bindings::{
    wb_camera_disable, wb_camera_enable, wb_camera_get_exposure, wb_camera_get_focal_distance,
    wb_camera_get_focal_length, wb_camera_get_fov, wb_camera_get_height, wb_camera_get_image,
    wb_camera_get_max_focal_distance, wb_camera_get_max_fov, wb_camera_get_min_focal_distance,
    wb_camera_get_min_fov, wb_camera_get_near, wb_camera_get_sampling_period, wb_camera_get_width,
    wb_camera_has_recognition, wb_camera_recognition_disable,
    wb_camera_recognition_disable_segmentation, wb_camera_recognition_enable,
    wb_camera_recognition_enable_segmentation, wb_camera_recognition_get_number_of_objects,
    wb_camera_recognition_get_objects, wb_camera_recognition_get_sampling_period,
    wb_camera_recognition_get_segmentation_image, wb_camera_recognition_has_segmentation,
    wb_camera_recognition_is_segmentation_enabled, wb_camera_recognition_save_segmentation_image,
    wb_camera_save_image, wb_camera_set_exposure, wb_camera_set_focal_distance, wb_camera_set_fov,
    wb_device_get_node_type, WbCameraRecognitionObject, WbDeviceTag, WbNodeType_WB_NODE_CAMERA,
};

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

    pub fn get_image(&self) -> &[u8] {
        let width = self.get_width();
        let height = self.get_height();
        unsafe {
            let image = wb_camera_get_image(self.0);
            from_raw_parts(image, (width * height * 4) as usize)
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

    pub fn recognition_enable(&self, sampling_period: i32) {
        unsafe { wb_camera_recognition_enable(self.0, sampling_period) }
    }

    pub fn recognition_disable(&self) {
        unsafe { wb_camera_recognition_disable(self.0) }
    }

    pub fn recognition_get_sampling_period(&self) -> i32 {
        unsafe { wb_camera_recognition_get_sampling_period(self.0) }
    }

    pub fn recognition_get_number_of_objects(&self) -> i32 {
        unsafe { wb_camera_recognition_get_number_of_objects(self.0) }
    }

    pub fn recognition_get_objects(&self) -> &[WbCameraRecognitionObject] {
        let number_of_objects = self.recognition_get_number_of_objects();
        unsafe {
            let objects = wb_camera_recognition_get_objects(self.0);
            from_raw_parts(objects, number_of_objects as usize)
        }
    }

    pub fn recognition_has_segmentation(&self) -> bool {
        unsafe { wb_camera_recognition_has_segmentation(self.0) != 0 }
    }

    pub fn recognition_enable_segmentation(&self) {
        unsafe { wb_camera_recognition_enable_segmentation(self.0) }
    }

    pub fn recognition_disable_segmentation(&self) {
        unsafe { wb_camera_recognition_disable_segmentation(self.0) }
    }

    pub fn recognition_is_segmentation_enabled(&self) -> bool {
        unsafe { wb_camera_recognition_is_segmentation_enabled(self.0) != 0 }
    }

    pub fn recognition_get_segmentation_image(&self) -> &[u8] {
        let width = self.get_width();
        let height = self.get_height();
        unsafe {
            let image = wb_camera_recognition_get_segmentation_image(self.0);
            from_raw_parts(image, (width * height * 4) as usize)
        }
    }

    pub fn recognition_save_segmentation_image(&self, filename: &str, quality: i32) -> i32 {
        let filename = CString::new(filename).expect("CString::new failed");
        unsafe { wb_camera_recognition_save_segmentation_image(self.0, filename.as_ptr(), quality) }
    }
}

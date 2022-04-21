use std::{
    ffi::{CStr, CString},
    slice::from_raw_parts,
};

use anyhow::bail;
use webots_bindings::{
    wb_camera_recognition_disable, wb_camera_recognition_disable_segmentation,
    wb_camera_recognition_enable, wb_camera_recognition_enable_segmentation,
    wb_camera_recognition_get_number_of_objects, wb_camera_recognition_get_objects,
    wb_camera_recognition_get_sampling_period, wb_camera_recognition_get_segmentation_image,
    wb_camera_recognition_has_segmentation, wb_camera_recognition_is_segmentation_enabled,
    wb_camera_recognition_save_segmentation_image, wb_device_get_node_type, WbDeviceTag,
    WbNodeType_WB_NODE_CAMERA,
};

use crate::Camera;

pub struct RecognitionObject<'a> {
    pub id: i32,
    pub position: [f64; 3],
    pub orientation: [f64; 4],
    pub size: [f64; 2],
    pub position_on_image: [i32; 2],
    pub size_on_image: [i32; 2],
    pub number_of_colors: i32,
    pub colors: &'a [f64],
    pub model: &'a str,
}

pub struct Recognition(WbDeviceTag);

impl Recognition {
    pub(crate) fn new(camera_device: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_CAMERA, unsafe {
            wb_device_get_node_type(camera_device)
        });
        Self(camera_device)
    }

    pub fn enable(&self, sampling_period: i32) {
        unsafe { wb_camera_recognition_enable(self.0, sampling_period) }
    }

    pub fn disable(&self) {
        unsafe { wb_camera_recognition_disable(self.0) }
    }

    pub fn get_sampling_period(&self) -> i32 {
        unsafe { wb_camera_recognition_get_sampling_period(self.0) }
    }

    pub fn get_number_of_objects(&self) -> i32 {
        unsafe { wb_camera_recognition_get_number_of_objects(self.0) }
    }

    pub fn get_objects<'a>(&self) -> anyhow::Result<Vec<RecognitionObject<'a>>> {
        let number_of_objects = self.get_number_of_objects();
        let objects = unsafe {
            let objects = wb_camera_recognition_get_objects(self.0);
            if objects.is_null() {
                bail!("Failed to get objects: objects data is NULL");
            }
            from_raw_parts(objects, number_of_objects as usize)
        };
        Ok(objects
            .iter()
            .map(|object| RecognitionObject {
                id: object.id,
                position: object.position,
                orientation: object.orientation,
                size: object.size,
                position_on_image: object.position_on_image,
                size_on_image: object.size_on_image,
                number_of_colors: object.number_of_colors,
                colors: unsafe { from_raw_parts(object.colors, object.number_of_colors as usize) },
                model: unsafe { CStr::from_ptr(object.model) }
                    .to_str()
                    .expect("CStr::to_str"),
            })
            .collect())
    }

    pub fn has_segmentation(&self) -> bool {
        unsafe { wb_camera_recognition_has_segmentation(self.0) != 0 }
    }

    pub fn enable_segmentation(&self) {
        unsafe { wb_camera_recognition_enable_segmentation(self.0) }
    }

    pub fn disable_segmentation(&self) {
        unsafe { wb_camera_recognition_disable_segmentation(self.0) }
    }

    pub fn is_segmentation_enabled(&self) -> bool {
        unsafe { wb_camera_recognition_is_segmentation_enabled(self.0) != 0 }
    }

    pub fn get_segmentation_image(&self) -> &[u8] {
        let camera = Camera::new(self.0);
        let width = camera.get_width();
        let height = camera.get_height();
        unsafe {
            let image = wb_camera_recognition_get_segmentation_image(self.0);
            from_raw_parts(image, (width * height * 4) as usize)
        }
    }

    pub fn save_segmentation_image(&self, filename: &str, quality: i32) -> i32 {
        let filename = CString::new(filename).expect("CString::new failed");
        unsafe { wb_camera_recognition_save_segmentation_image(self.0, filename.as_ptr(), quality) }
    }
}

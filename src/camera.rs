use std::ffi::CString;

use webots_bindings::{
    wb_device_get_node_type, wb_robot_get_device, WbDeviceTag, WbNodeType_WB_NODE_CAMERA,
};

pub struct Camera(WbDeviceTag);

impl Camera {
    pub fn new(name: &str) -> Self {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        assert_eq!(WbNodeType_WB_NODE_CAMERA, unsafe {
            wb_device_get_node_type(device)
        });
        Self(device)
    }

// void wb_camera_enable(WbDeviceTag tag, int sampling_period);
// void wb_camera_disable(WbDeviceTag tag);
// int wb_camera_get_sampling_period(WbDeviceTag tag);

// const unsigned char *wb_camera_get_image(WbDeviceTag tag);
// int wb_camera_get_width(WbDeviceTag tag);
// int wb_camera_get_height(WbDeviceTag tag);
// double wb_camera_get_fov(WbDeviceTag tag);
// double wb_camera_get_max_fov(WbDeviceTag tag);
// double wb_camera_get_min_fov(WbDeviceTag tag);
// void wb_camera_set_fov(WbDeviceTag tag, double fov);  // fov specified in rad
// double wb_camera_get_exposure(WbDeviceTag tag);
// void wb_camera_set_exposure(WbDeviceTag tag, double exposure);
// double wb_camera_get_focal_length(WbDeviceTag tag);
// double wb_camera_get_focal_distance(WbDeviceTag tag);
// double wb_camera_get_max_focal_distance(WbDeviceTag tag);
// double wb_camera_get_min_focal_distance(WbDeviceTag tag);
// void wb_camera_set_focal_distance(WbDeviceTag tag, double focal_distance);
// double wb_camera_get_near(WbDeviceTag tag);
// int wb_camera_save_image(WbDeviceTag tag, const char *filename, int quality);

// // smart camera
// bool wb_camera_has_recognition(WbDeviceTag tag);
// void wb_camera_recognition_enable(WbDeviceTag tag, int sampling_period);
// void wb_camera_recognition_disable(WbDeviceTag tag);
// int wb_camera_recognition_get_sampling_period(WbDeviceTag tag);
// int wb_camera_recognition_get_number_of_objects(WbDeviceTag tag);
// const WbCameraRecognitionObject *wb_camera_recognition_get_objects(WbDeviceTag tag);
// bool wb_camera_recognition_has_segmentation(WbDeviceTag tag);
// void wb_camera_recognition_enable_segmentation(WbDeviceTag tag);
// void wb_camera_recognition_disable_segmentation(WbDeviceTag tag);
// bool wb_camera_recognition_is_segmentation_enabled(WbDeviceTag tag);
// const unsigned char *wb_camera_recognition_get_segmentation_image(WbDeviceTag tag);
// int wb_camera_recognition_save_segmentation_image(WbDeviceTag tag, const char *filename, int quality);

// #ifdef WB_MATLAB_LOADLIBRARY
// // This function should be used only in the Matlab wrapper
// const WbCameraRecognitionObject *wb_camera_recognition_get_object(WbDeviceTag tag, int index);
// #endif

// /* useful macros to get pixel colors from the image data, width and coords *
//  *
//  *  ^ y
//  *  |    (height)
//  *  |===============
//  *  |=============== *: pixel@(x,y)
//  *  |---------*=====
//  *  |=========|===== (width)
//  *  |=========|=====
//  *  |=========|=====
//  *  |=========|=====
//  * -+-----------------> x
//  * o|
//  */
// #define wb_camera_image_get_red(image, width, x, y) (image[4 * ((y) * (width) + (x)) + 2])
// #define wb_camera_image_get_green(image, width, x, y) (image[4 * ((y) * (width) + (x)) + 1])
// #define wb_camera_image_get_blue(image, width, x, y) (image[4 * ((y) * (width) + (x))])

// #ifdef KROS_COMPILATION
// #define wb_camera_image_get_gray(image, width, x, y) (image[(y) * (width) + (x)])
// #else
// #define wb_camera_image_get_gray(image, w, x, y) \
//   ((image[4 * ((y) * (w) + (x)) + 2] + image[4 * ((y) * (w) + (x)) + 1] + image[4 * ((y) * (w) + (x))]) / 3)
// #endif
// // alias
// #define wb_camera_image_get_grey(image, width, x, y) wb_camera_image_get_gray(image, width, x, y)

}

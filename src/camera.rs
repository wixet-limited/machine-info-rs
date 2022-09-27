//! V4l list cameras feature
use v4l::context;
use crate::model::Camera;
use std::panic;
/// List of attached cameras to the machine
/// Example
/// ```
/// use machine_info::Machine;
/// 
/// println!("{:?}", Machine::list_cameras());
/// 
/// ```
pub fn list_cameras() -> Vec<Camera> {
    let mut cameras = vec![];

    // I catch panic because the library uses unwrap internally and sometimes the device has no name
    for dev in context::enum_devices() {
        let name = panic::catch_unwind(|| {
            dev.name().unwrap()
        });
    
        let name = match name {
            Ok(name) => name,
            Err(_) => "Unknown".to_owned()
            
        };
        
        cameras.push(Camera {
            name,
            path: dev.path().as_os_str().to_str().unwrap_or("Unknown").to_owned()
        })
    }
    cameras

}
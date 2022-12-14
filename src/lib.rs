#![warn(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::missing_doc_code_examples)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

//! CPU and memory monitor. You can retrieve the CPU, memory and GPU usage. Global and by processes
//! Also you can get a snapshot of your current hardware and system info
//! It is meant to monitor a system so the performance is the priority. You can probe every second
//! that it will not be harmful
mod machine;
mod model;
mod monitor;

#[cfg(feature = "v4l")]
pub mod camera;

pub use machine::Machine;
pub use model::{Disk, DiskUsage, Process, GraphicsProcessUtilization, SystemStatus, GraphicsUsage, Processor, GraphicCard, SystemInfo, Camera, NvidiaInfo};



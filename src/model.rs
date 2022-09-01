use serde::{Serialize, Deserialize};

// System status
#[derive(Deserialize, Serialize, Debug)]
pub struct DiskUsage {
    pub name: String,
    pub used: u64,
    pub total: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Process {
    pub pid: i32,
    pub cpu: f32,
    pub memory: u64,
    pub name: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GraphicsProcessUtilization {
    pub pid: u32,
    pub gpu: u32,
    pub memory: u32,
    pub encoder: u32,
    pub decoder: u32    
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GraphicsUsage {
    pub id: String,
    pub memory_usage: u32,
    pub memory_used: u64,
    pub encoder: u32,
    pub decoder: u32,
    pub gpu: u32,
    pub temperature: u32,
    pub processes: Vec<GraphicsProcessUtilization>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SystemStatus {
    pub memory: u64,
    //pub used_swap: u64,
    pub cpu: f32,
    pub processes: Vec<Process>
}
// System info

#[derive(Deserialize, Serialize, Debug)]
pub struct SystemInfo {
    pub os_name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub hostname: String,
    pub memory: u64,
    pub driver_version: Option<String>,
    pub nvml_version: Option<String>,
    pub cuda_version: Option<i32>,
    //pub processors: Vec<Processor>,
    pub processor: Processor,
    pub total_processors: usize,
    pub graphics: Vec<GraphicCard>,
    pub disks: Vec<Disk>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Processor {
    pub frequency: u64,
    pub vendor: String,
    pub brand: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GraphicCard {
    pub id: String,
    pub name: String,
    pub brand: String,
    pub memory: u64,
    pub temperature: u32
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Disk {
    pub name: String,
    pub fs: String,
    pub storage_type: String,
    pub mount_point: String,
    pub available: u64,
    pub size: u64
}
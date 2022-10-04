use serde::{Serialize, Deserialize};

/// System status
#[derive(Deserialize, Serialize, Debug)]
pub struct DiskUsage {
    /// Name of the disk
    pub name: String,
    /// Total bytes used
    pub used: u64,
    /// Total disk capacity
    pub total: u64,
}

/// Process usage
#[derive(Deserialize, Serialize, Debug)]
pub struct Process {
    /// Process identificator
    pub pid: i32,
    /// Cpu used as percentage
    pub cpu: f64,
    
}

/// Graphic card usage by process
#[derive(Deserialize, Serialize, Debug)]
pub struct GraphicsProcessUtilization {
    /// Process identificator
    pub pid: u32,
    /// Gpu identificator
    pub gpu: u32,
    /// Memory usage
    pub memory: u32,
    /// Gpu encoder utilization as percentage
    pub encoder: u32,
    /// Gpu decoder utilization as percentage
    pub decoder: u32    
}

/// Graphic card usage summary
#[derive(Deserialize, Serialize, Debug)]
pub struct GraphicsUsage {
    /// Graphic card id
    pub id: String,
    /// Memory utilization as percentage
    pub memory_usage: u32,
    /// Memroy usage as bytes
    pub memory_used: u64,
    /// Gpu encoder utilization as percentage
    pub encoder: u32,
    /// Gpu decoder utilization as percentage
    pub decoder: u32,
    /// Gpu utilization as percentage
    pub gpu: u32,
    /// Gpu temperature
    pub temperature: u32,
    /// Processes using this GPU
    pub processes: Vec<GraphicsProcessUtilization>
}

/// System global utilization
#[derive(Deserialize, Serialize, Debug)]
pub struct SystemStatus {
    /// Total memory used
    pub memory: i32,
    /// Total CPU used as percentage
    pub cpu: i32,
}

/// Summary of the system
#[derive(Deserialize, Serialize, Debug)]
pub struct SystemInfo {
    /// Operating system name
    pub os_name: String,
    /// Running kernel version
    pub kernel_version: String,
    /// Operating system version
    pub os_version: String,
    /// System hostname
    pub hostname: String,
    /// Total memory of the machine
    pub memory: u64,
    /// Microprocessor description
    pub processor: Processor,
    /// Total amount of processors
    pub total_processors: usize,
    /// List of graphic cards
    pub graphics: Vec<GraphicCard>,
    /// List of available disks
    pub disks: Vec<Disk>,
    /// List of available cameras
    pub cameras: Vec<Camera>,
    /// Nvidia driver info
    pub nvidia: Option<NvidiaInfo>,
}

/// Information about microprocessor
#[derive(Deserialize, Serialize, Debug)]
pub struct Processor {
    /// Processor clock speed
    pub frequency: u64,
    /// Processor vendor
    pub vendor: String,
    /// Processor brand
    pub brand: String
}

/// Information about a graphic card
#[derive(Deserialize, Serialize, Debug)]
pub struct GraphicCard {
    /// Device id
    pub id: String,
    /// Device id
    pub name: String,
    /// Device brand
    pub brand: String,
    /// Total memory
    pub memory: u64,
    /// Device temperature
    pub temperature: u32
}

/// Information about a hard disk
#[derive(Deserialize, Serialize, Debug)]
pub struct Disk {
    /// Disk name
    pub name: String,
    /// Filesystem
    pub fs: String,
    /// Storage type (ssd, hd...)
    pub storage_type: String,
    /// Where it is mounted
    pub mount_point: String,
    /// Available space
    pub available: u64,
    /// Total size
    pub size: u64
}

/// Connected camera information
#[derive(Debug, Serialize, Deserialize)]
pub struct Camera {
    /// The camera name
    pub name: String,
    /// Camera path like /dev/video0
    pub path: String
}

/// Nvidia drivers configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct NvidiaInfo {
     /// Nvidia drivers
     pub driver_version: String,
     /// NVML version
     pub nvml_version: String,
     /// Cuda version
     pub cuda_version: i32,
}
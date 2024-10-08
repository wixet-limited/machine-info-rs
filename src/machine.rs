use anyhow::Result;
use sysinfo::{DiskExt, CpuExt, System, SystemExt};
use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use log::{debug, info};
use crate::model::{SystemInfo, Processor, Disk, GraphicCard, GraphicsUsage, GraphicsProcessUtilization, SystemStatus, Process, Camera, NvidiaInfo};
use crate::monitor::Monitor;
use std::path::Path;

#[cfg(feature = "v4l")]
use crate::camera::list_cameras;

#[cfg(not(feature = "v4l"))]
fn list_cameras() -> Vec<Camera> {
    vec![]
}

/// Represents a machine. Currently you can monitor global CPU/Memory usage, processes CPU usage and the
/// Nvidia GPU usage. You can also retrieve information about CPU, disks...
pub struct Machine {
    monitor: Monitor,
    nvml: Option<nvml_wrapper::Nvml>,
}


impl Machine {
    /// Creates a new instance of Machine. If not graphic card it will warn about it but not an error
    /// Example
    /// ```
    /// use machine_info::Machine;
    /// let m = Machine::new();
    /// ```
    pub fn new() -> Machine{
        let nvml = match Nvml::init() {
            Ok(nvml) => {
                info!("Nvidia driver loaded");
                Some(nvml)
            },
            Err(error) => {
                debug!("Nvidia not available because {}", error);
                None
            }
        };
        Machine{
            monitor: Monitor::new(),
            nvml: nvml
        }
    }
    
    /// Retrieves full information about the computer
    /// Example
    /// ```
    /// use machine_info::Machine;
    /// let m = Machine::new();
    /// println!("{:?}", m.system_info())
    /// ```
    pub fn system_info(& mut self) -> SystemInfo {
        let sys = System::new_all();
        //let mut processors = Vec::new();
        let processor = sys.global_cpu_info();
        let processor = Processor{
            frequency: processor.frequency(),
            vendor: processor.vendor_id().to_string(),
            brand: processor.brand().to_string()
        };


        let mut disks = Vec::new();
        for disk in sys.disks() {
            disks.push(Disk{
                name: disk.name().to_str().unwrap().to_string(),
                fs: String::from_utf8(disk.file_system().to_vec()).unwrap(),
                storage_type: match disk.type_() {
                    sysinfo::DiskType::HDD => "HDD".to_string(),
                    sysinfo::DiskType::SSD => "SSD".to_string(),
                    _ => "Unknown".to_string()
                },
                available: disk.available_space(),
                size: disk.total_space(),
                mount_point: disk.mount_point().to_str().unwrap().to_string()
            })
        }

        let mut cards = Vec::new();
        let nvidia = if let Some(nvml) = &self.nvml {
            for n in 0..nvml.device_count().unwrap() {
                let device = nvml.device_by_index(n).unwrap();
                cards.push(GraphicCard{
                    id: device.uuid().unwrap(),
                    name: device.name().unwrap(),
                    brand: match device.brand().unwrap() {
                        nvml_wrapper::enum_wrappers::device::Brand::GeForce => "GeForce".to_string(),
                        nvml_wrapper::enum_wrappers::device::Brand::Quadro => "Quadro".to_string(),
                        nvml_wrapper::enum_wrappers::device::Brand::Tesla => "Tesla".to_string(),
                        nvml_wrapper::enum_wrappers::device::Brand::Titan => "Titan".to_string(),
                        nvml_wrapper::enum_wrappers::device::Brand::NVS => "NVS".to_string(),
                        nvml_wrapper::enum_wrappers::device::Brand::GRID => "GRID".to_string(),
                        nvml_wrapper::enum_wrappers::device::Brand::Unknown => "Unknown".to_string(),
                    },
                    memory: device.memory_info().unwrap().total,
                    temperature: device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu).unwrap()
                });
            }
            Some(NvidiaInfo {
                driver_version: nvml.sys_driver_version().unwrap(),
                nvml_version: nvml.sys_nvml_version().unwrap(),
                cuda_version: nvml.sys_cuda_driver_version().unwrap()
            })
        } else {
            None
        };
        
        // Getting the model
        let model_path = Path::new("/sys/firmware/devicetree/base/model");
        let model = if model_path.exists() {
            Some(std::fs::read_to_string(model_path).unwrap())
        } else {
            None
        };
        
        let vaapi = Path::new("/dev/dri/renderD128").exists();

        SystemInfo {
            os_name: sys.name().unwrap(),
            kernel_version: sys.kernel_version().unwrap(),
            os_version: sys.os_version().unwrap(),
            distribution: sys.distribution_id(),
            hostname: sys.host_name().unwrap(),
            memory: sys.total_memory(),
            nvidia,
            vaapi,
            processor,
            total_processors: sys.cpus().len(),
            graphics: cards,
            disks,
            cameras: list_cameras(),
            model
        }
    }

    /*pub fn disks_status(&self) {
        //TODO
        /*
        let mut disks = Vec::new();
        for disk in self.sys.disks() {
            disks.push(api::model::Disk{
            })
            */
    }*/

    /// The current usage of all graphic cards (if any)
    /// Example
    /// ```
    /// use machine_info::Machine;
    /// let m = Machine::new();
    /// println!("{:?}", m.graphics_status())
    /// ```
    pub fn graphics_status(&self) -> Vec<GraphicsUsage> {
        let mut cards = Vec::new();
        if let Some(nvml) = &self.nvml {
            for n in 0..nvml.device_count().unwrap() {
                let device = nvml.device_by_index(n).unwrap();
                let mut processes = Vec::new();
                let stats = device.process_utilization_stats(None);
                if stats.is_ok() {
                    for p in stats.unwrap() {
                        processes.push(GraphicsProcessUtilization{
                            pid: p.pid,
                            gpu: p.sm_util,
                            memory: p.mem_util,
                            encoder: p.enc_util,
                            decoder: p.dec_util
                        });
                    }
                }
    
                cards.push(GraphicsUsage {
                    id: device.uuid().unwrap(),
                    memory_used: device.memory_info().unwrap().used,
                    encoder: device.encoder_utilization().unwrap().utilization,
                    decoder: device.decoder_utilization().unwrap().utilization,
                    gpu: device.utilization_rates().unwrap().gpu,
                    memory_usage: device.utilization_rates().unwrap().memory,
                    temperature: device.temperature(TemperatureSensor::Gpu).unwrap(),
                    processes
                });
            }
        }
        
        cards
        
    }


    /// To calculate the CPU usage of a process we have to keep track in time the process so first we have to register the process.
    /// You need to know the PID of your process and use it as parameters. In case you provide an invalid PID it will return error
    /// Example
    /// ```
    /// use machine_info::Machine;
    /// let m = Machine::new();
    /// let process_pid = 3218;
    /// m.track_process(process_pid)
    /// ```
    pub fn track_process(&mut self, pid: i32) -> Result<()>{
        self.monitor.track_process(pid)
    }

    /// Once we dont need to track a process it is recommended to not keep using resources on it. You should know the PID of your process.
    /// If the PID was not registered before, it will just do nothing
    /// Example
    /// ```
    /// use machine_info::Machine;
    /// let m = Machine::new();
    /// let process_pid = 3218;
    /// m.track_process(process_pid)
    /// m.untrack_process(process_pid)
    /// ```
    pub fn untrack_process(&mut self, pid: i32) {
        self.monitor.untrack_process(pid);
    }

    /// The CPU usage of all tracked processes since the last call. So if you call it every 10 seconds, you will
    /// get the CPU usage during the last 10 seconds. More calls will make the value more accurate but also more expensive
    /// Example
    /// ```
    /// use machine_info::Machine;
    /// use std::{thread, time};
    /// 
    /// let m = Machine::new();
    /// m.track_process(3218)
    /// m.track_process(4467)
    /// loop {   
    ///   let status = m.processes_status();
    ///   println!("{:?}", status);
    ///   thread::sleep(time::Duration::from_millis(1000));
    /// }
    /// 
    /// ```
    pub fn processes_status(& mut self) -> Vec<Process> {
        self.monitor.next_processes().iter().map(|(pid, cpu)| Process{pid:*pid, cpu:*cpu}).collect::<Vec<Process>>()
    }

    /// The CPU and memory usage. For the CPU, it is the same as for `processes_status`. For the memory it returs the amount
    /// a this moment
    /// Example
    /// ```
    /// use machine_info::Machine;
    /// use std::{thread, time};
    /// 
    /// let m = Machine::new();
    /// m.track_process(3218)
    /// m.track_process(4467)
    /// loop {   
    ///   let status = m.system_status();
    ///   println!("{:?}", status);
    ///   thread::sleep(time::Duration::from_millis(1000));
    /// }
    /// 
    /// ```
    pub fn system_status(& mut self) -> Result<SystemStatus> {
        let (cpu, memory) = self.monitor.next()?;
        Ok(SystemStatus {
            memory,
            cpu,
        })
    }

}
use sysinfo::{DiskExt, CpuExt, Pid, System, SystemExt, ProcessExt, PidExt, ComponentExt, ProcessRefreshKind, CpuRefreshKind, RefreshKind};
use nvml_wrapper::NVML;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use crate::model::{SystemInfo, Processor, Disk, GraphicCard, GraphicsUsage, GraphicsProcessUtilization, SystemStatus, Process};

use log::{warn, info};

pub struct Machine {
    pub sys: sysinfo::System,
    pub nvml: Option<nvml_wrapper::NVML>,
}


impl Machine {
    pub fn new() -> Result<Machine, nvml_wrapper::error::NvmlError>{
        let nvml = match NVML::init() {
            Ok(nvml) => {
                info!("Nvidia driver loaded");
                Some(nvml)
            },
            Err(error) => {
                warn!("Nvidia not available because {}", error);
                None
            }
        };
        Ok(Machine{
            sys: System::new_all(),
            nvml: nvml
        })
    }

    pub fn system_info(& mut self) -> SystemInfo {
        self.sys.refresh_all();
        //let mut processors = Vec::new();
        let processors = self.sys.cpus();
        let p = &processors[0];
        let processor = Processor{
            frequency: p.frequency(),
            vendor: p.vendor_id().to_string(),
            brand: p.brand().to_string()
        };

        /*for processor in self.sys.processors() {
            processors.push(model::Processor{
                frequency: processor.frequency(),
                vendor: processor.vendor_id().to_string(),
                brand: processor.brand().to_string()
            })
        }*/
        let mut disks = Vec::new();
        for disk in self.sys.disks() {
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
        let (driver_version, nvml_version, cuda_version) = if let Some(nvml) = &self.nvml {
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
            (Some(nvml.sys_driver_version().unwrap()), Some(nvml.sys_nvml_version().unwrap()), Some(nvml.sys_cuda_driver_version().unwrap()))
        } else {
            (None, None, None)
        };
        

        SystemInfo {
            os_name: self.sys.name().unwrap(),
            kernel_version: self.sys.kernel_version().unwrap(),
            os_version: self.sys.os_version().unwrap(),
            hostname: self.sys.host_name().unwrap(),
            memory: self.sys.total_memory(),
            driver_version,
            nvml_version,
            cuda_version,
            processor,
            total_processors: processors.len(),
            graphics: cards,
            disks
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

    pub fn graphics_status(&self) -> Vec<GraphicsUsage> {
        let mut cards = Vec::new();
        if let Some(nvml) = &self.nvml {
            for n in 0..nvml.device_count().unwrap() {
                let device = nvml.device_by_index(n).unwrap();
                let mut processes = Vec::new();
                for p in device.process_utilization_stats(None).unwrap() {
                    processes.push(GraphicsProcessUtilization{
                         pid: p.pid,
                        gpu: p.sm_util,
                        memory: p.mem_util,
                        encoder: p.enc_util,
                        decoder: p.dec_util
                    });
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

    pub fn processes_status(& mut self, pids: &Vec<i32>) -> Vec<Process> {
        let mut processes = Vec::with_capacity(pids.len());

        for pid in pids {
            let p = Pid::from(*pid);
            //let res = self.sys.refresh_process_specifics(Pid::from(7620), ProcessRefreshKind::new().with_cpu());
            if self.sys.refresh_process_specifics(p, ProcessRefreshKind::everything()) {
                let p = self.sys.process(p).unwrap();
                processes.push(Process{
                    pid: *pid,
                    cpu: p.cpu_usage(),
                    memory: p.memory(),
                    name: p.name().to_owned(),
                })
            } else {
                warn!("Pid {} does not exist", pid);
            }
            
        }
        processes
    }

    pub fn system_status(& mut self) -> SystemStatus {
        self.sys.refresh_memory();
        let memory = self.sys.used_memory();
        self.sys.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
        let cpu = self.sys.global_cpu_info().cpu_usage();   

        let processes = vec![];
        SystemStatus {
            memory,
            cpu,
            processes
        }
    }
}
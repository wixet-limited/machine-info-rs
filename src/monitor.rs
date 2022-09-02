use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::SystemTime;
use std::collections::HashMap;
use log::warn;

pub struct Monitor {
    last_cpu: Cpu,
    last_processes: HashMap<i32, Process>
}

impl Monitor {
    pub fn new() -> Monitor {
        Monitor {
            last_cpu: Cpu{values: vec![0;10]},
            last_processes: HashMap::new()
        }
    }

    pub fn next(&mut self) -> Result<(i32, i32)> {
        let cpu = Cpu::from_file(File::open("/proc/stat")?)?;
        let cpu_usage = cpu.usage(&self.last_cpu);
        self.last_cpu = cpu;
        let memory_usage = Memory::from_file(File::open("/proc/meminfo")?)?.usage();
        Ok((cpu_usage, memory_usage))
    }

    pub fn next_processes(&mut self) -> Vec<(i32,f64)> {
        //let mut processes = HashMap::with_capacity(self.last_processes.len());
        let mut result = vec![];
        let mut to_untrack = vec![];
        for (&pid, last_process) in &mut self.last_processes {
            match Monitor::get_process(pid) {
                Ok(current_process) => {
                    result.push((pid, current_process.usage(last_process)));
                    
                    last_process.total_time = current_process.total_time;
                    last_process.when = current_process.when;
                },
                Err(err) => {
                    warn!("Cannot get process {}: {:?}. Will be removed", pid, err);
                    to_untrack.push(pid);
                }
            }
            
        }

        for pid in to_untrack {
            self.untrack_process(pid);
        }

        result
    }

    fn get_process(pid: i32) -> Result<Process>{
        Ok(Process::from_file(File::open(format!("/proc/{}/stat", pid))?)?)
    }

    pub fn track_process(&mut self, pid: i32) -> Result<()> {
        self.last_processes.insert(pid, Monitor::get_process(pid)?);
        Ok(())

    }

    pub fn untrack_process(&mut self, pid: i32) {
        self.last_processes.remove(&pid);
    }
}

struct Cpu {
    values: Vec<i32>
}

impl Cpu {
    pub fn from_file(file: impl std::io::Read) -> Result<Cpu> {
        let line = io::BufReader::new(file).lines().next().unwrap()?;
        let re = line.split(" ").collect::<Vec<&str>>();
        Ok(Cpu{values: re[2..].iter().map(|&e| e.parse::<i32>().unwrap()).collect::<Vec<i32>>()})
    }

    pub fn usage(&self, last: &Cpu) -> i32 {
        let last_sum = last.values.iter().sum::<i32>();
        let current_sum = self.values.iter().sum::<i32>();
        let delta = current_sum - last_sum;
        let idle = self.values[3] - last.values[3];
        let used = delta - idle;
        let usage = 100 * used / delta;
        usage
    }

}

struct Process {
    pub total_time: i32,
    pub when: SystemTime,
}



impl Process {
    pub fn from_file(file: impl std::io::Read) -> Result<Process> {
        let line = io::BufReader::new(file).lines().next().unwrap()?;
        let params = line.split(" ").collect::<Vec<&str>>();
        Ok(Process{
            total_time: params[13..18].iter().map(|e| e.parse::<i32>().unwrap()).sum(),
            when: SystemTime::now()
        })
    }

    pub fn usage(&self, last: &Process) -> f64 {
        let computing_time = (self.total_time - last.total_time) as f64;
        // I use unwrap here because I can warrantee that now is higher than last.
        // I'm assuming that CLK_TCK is 100, this is why I multiply seconds by 100
        let elapsed_time = (SystemTime::now().duration_since(last.when).unwrap().as_secs()*100) as f64;
        // Return it as percentaje
        let usage = 100.0 * (computing_time / elapsed_time);
        usage
    }

}




pub struct Memory {
    total: i32,
    free: i32,
    buffers: i32,
    cached: i32,
    reclaimable: i32
}

fn memory_value(raw: &str) -> Result<i32> {
    if let Some(value) = raw.trim_start().split(' ').next() {
        Ok(value.parse::<i32>()?)
    } else {
        Err(anyhow::anyhow!("Memory line cannot be parsed").context(raw.to_owned()))
    }
}

impl Memory {
    pub fn from_file(file: impl std::io::Read) -> Result<Memory> {
        let mut m = Memory {
            total:0, free: 0, buffers: 0, cached: 0, reclaimable: 0
        };

        for line in io::BufReader::new(file).lines() {
            if let Some((field, value)) = line?.split_once(':') {
                match field {
                    "MemTotal" => m.total = memory_value(value)?,
                    "MemFree" => m.free = memory_value(value)?,
                    "Buffers" => m.buffers = memory_value(value)?,
                    "Cached" => m.cached = memory_value(value)?,
                    "SReclaimable" => m.reclaimable = memory_value(value)?,
                    _ => continue
                };
            }
            
        }

        Ok(m)
    }

    pub fn usage(&self) -> i32 {
        self.total - self.free - self.buffers - self.cached - self.reclaimable
    }
}


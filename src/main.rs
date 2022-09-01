mod machine;
mod model;

use machine::Machine;
use anyhow::Result;

use std::{thread, time};

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}


fn main() -> Result<()> {
    setup_logger()?;
    let mut m = Machine::new()?;
    println!("Hello, world!");
    //let mut c = System::new_all();
    let prs = vec![7620, 20200];
    for n in 1..100 {
        println!("{:?}", m.processes_status(&prs));
        /*
        c.refresh_memory();
        //c.refresh_cpu();
        let memory = c.used_memory();
        //c.refresh_processes_specifics(ProcessRefreshKind::new().with_cpu());
        //c.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
        //let cpu = c.global_cpu_info().cpu_usage();

        let res = c.refresh_process_specifics(Pid::from(7620), ProcessRefreshKind::new().with_cpu());
        //self.sys.refresh_process(Pid::from(7620));
        //c.refresh_processes_specifics(ProcessRefreshKind::new().with_cpu());

        let a = c.process(Pid::from(7620)).unwrap();
        println!("PROCESOOOO UNICO {:?}", a.cpu_usage());*/

        thread::sleep(time::Duration::from_millis(3000));
    }

    Ok(())
}

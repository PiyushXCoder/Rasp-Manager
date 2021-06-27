use std::{path::Path, process::Command};

use tide::prelude::*;
use tide::{Request, log::warn};

use libmedium::{parse_hwmons,sensors::{Input, Sensor}};

mod config;
mod command;
mod disks;

#[derive(Serialize)]
struct SystemInfo {
    hardware: String,
    system_name: String,
    os_version: Option<String>,
    kernel_ver: String,
    last_uadate: Option<String>,
    hostname: String,
    boot_time: String,
    cpu_cores_count: u32,
    cpu_load_avg: f64, // one minute
    mem_total: f64,
    mem_used: f64,
    swap_total: f64,
    swap_used: f64,
    disk: Vec<Disk>,
    temperature: Vec<Temprature>
}

#[derive(Serialize)]
struct Disk {
    mount: String,
    total: f64,
    available: f64
}

#[derive(Serialize)]
struct Temprature {
    label: String,
    temp: f64
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let conf = config::Config::generate();
    
    tide::log::start();
    let mut app = tide::new();

    if let Some(val) = conf.static_dirs {
        let path = Path::new(&val);
        if path.exists() && path.is_dir() {
            app.at("").serve_dir(path.to_str().unwrap())?;    
        } else { warn!("Static Directory dosen't exists!") }
        
        let path = path.join("index.html");
        if path.exists() {
            app.at("/").serve_file(path.to_str().unwrap())?;
        }
    }

    app.at("/poweroff").get(poweroff);
    app.at("/reboot").get(reboot);

    app.at("/sysinfo").get(system_info);

    app.at("/ip_addr").get(command::ip_addr);
    app.at("/ps").get(command::ps);
    app.at("/lsblk").get(command::lsblk);
    app.at("/arp").get(command::arp);

    app.listen(format!("{}:{}", conf.addr, conf.port)).await?;
    Ok(())
}

async fn poweroff(_: Request<()>) -> tide::Result {
    async_std::task::spawn(async {
        async_std::task::sleep(std::time::Duration::from_secs(3)).await;
        Command::new("poweroff").spawn().expect("Failed to poweroff!");
    });
    Ok("Reqesting to poweroff. Please see green led for for activity".into())
}

async fn reboot(_: Request<()>) -> tide::Result {
    async_std::task::spawn(async {
        async_std::task::sleep(std::time::Duration::from_secs(3)).await;
        Command::new("reboot").spawn().expect("Failed to reboot!");
    });
    Ok("Reqesting to Rebooting.".into())
}

async fn system_info(_: Request<()>) -> tide::Result {
    let os = sys_info::linux_os_release().unwrap();
    
    let mut cpu_load_avg = 0.0;
    if let Ok(ld) = sys_info::loadavg() {
        cpu_load_avg = ld.one;
    }

    let mut mem_total = 0.0;
    let mut mem_used = 0.0;
    let mut swap_total = 0.0;
    let mut swap_used = 0.0;
    if let Ok(info) = sys_info::mem_info() {
        mem_total = info.total as f64 / 1024.0;
        mem_used = (info.total - info.free) as f64 / 1024.0;
        swap_total = info.swap_total as f64 / 1024.0;
        swap_used = (info.swap_total - info.swap_free) as f64 / 1024.0;
    }

    let mut disk = Vec::new();
    for d in disks::get_disks_info() {
        disk.push(Disk {
            mount: d.mount,
            total: d.total as f64 / 1048576.0, // bytes to mb
            available: d.available as f64 / 1048576.0 // bytes to mb
        });
    }

    let mut temperature: Vec<Temprature> = Vec::new();
    let hwmons = parse_hwmons().unwrap();
    for (_, _, hwmon) in &hwmons {
        for (_, temp_sensor) in hwmon.temps() {
            let tmp = temp_sensor.read_input().unwrap();
            temperature.push(Temprature {
                label: temp_sensor.name(),
                temp: tmp.as_degrees_celsius()
            });
            
        }
    }

    let boottime = std::time::Duration::from_secs(match  sys_info::boottime() {
        Ok(s) => s.tv_sec as u64,
        Err(_) => 0
    });

    let sys_info = SystemInfo {
        hardware: "".to_owned(),
        system_name: os.pretty_name.unwrap_or_default(),
        os_version: os.version,
        kernel_ver: sys_info::os_release().unwrap_or_default(),
        last_uadate: last_update(),
        hostname: sys_info::hostname().unwrap_or_default(),
        boot_time: humantime::format_duration(boottime).to_string(),
        cpu_cores_count: sys_info::cpu_num().unwrap_or_default(),
        cpu_load_avg,
        mem_total,
        mem_used,
        swap_total,
        swap_used,
        disk,
        temperature
    };

    Ok(json!(sys_info).to_string().into())
}

fn last_update() -> Option<String> {
    let mut cmd = std::process::Command::new("bash");
    cmd.args(&["-c", "grep 'pacman -Syu' /var/log/pacman.log | tail -n 1"]);

    let stdout = match cmd.output() {
        Ok(out) => out.stdout,
        Err(_) => return None
    };

    match String::from_utf8(stdout) {
        Ok(val) => {
            let s = val.split(" ").next()?;
            return Some(s[1..s.len()-1].to_owned());
        }, Err(_) => return None
    }
}
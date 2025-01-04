
use sysinfo::{
    Components, Disks, Networks, System,
};
use serde::Serialize;


#[derive(Serialize)]
pub struct Info {
    system_name: String,
    os_version: String,
    hostname: String,
    cpu_count: usize,
}

pub fn get_system_info() -> Info {
    let mut system = System::new_all();
    system.refresh_all();

    let system_name = System::name().unwrap_or_default();
    let os_version = System::os_version().unwrap_or_default();
    let hostname = System::host_name().unwrap_or_default();
    let cpu_count = system.cpus().len();

    return Info {
        system_name,
        os_version,
        hostname,
        cpu_count,
    }
}

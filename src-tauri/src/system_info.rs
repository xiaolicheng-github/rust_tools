
use sysinfo::{
    Disks, System
};
use serde::Serialize;

#[derive(Serialize)]
struct DiskListItem {
    name: String,
}
#[derive(Serialize)]
pub struct Info {
    system_name: String,
    os_version: String,
    hostname: String,
    cpu_count: usize,
    disks: Vec<DiskListItem>,
}

pub fn get_system_info() -> Info {
    let mut system = System::new_all();
    system.refresh_all();

    let system_name = System::name().unwrap_or_default();
    let os_version = System::os_version().unwrap_or_default();
    let hostname = System::host_name().unwrap_or_default();
    let cpu_count = system.cpus().len();
    let disks = Disks::new_with_refreshed_list();
    let mut disk_list = vec![];
    for disk in &disks {
        disk_list.push(
            DiskListItem {
                name: disk.name().to_string_lossy().into_owned(),
            });
    }

    return Info {
        system_name,
        os_version,
        hostname,
        cpu_count,
        disks: disk_list,
    }
}

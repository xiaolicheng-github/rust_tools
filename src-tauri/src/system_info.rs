use sysinfo::{
    CpuRefreshKind, Disks, Networks, RefreshKind, System
};
use serde::Serialize;

#[derive(Serialize)]
struct CpuListItem {
    name: String,
    vendor_id: String,
    frequency: String,
}
#[derive(Serialize)]
struct DiskListItem {
    name: String,
    kind: String,
    mount_point: String,
    total_space: String,
}
#[derive(Serialize)]
pub struct Info {
    system_name: String,
    os_version: String,
    hostname: String,
    cpu_count: usize,
    cpus: Vec<CpuListItem>,
    disks: Vec<DiskListItem>,
    total_memory: String,
    available_memory: String,
    network: String,
}

fn bytes_to_gb(bytes: u64) -> f64 {
    const BYTES_PER_GB: u64 = 1_073_741_824; // 每 GB 的字节数
    bytes as f64 / BYTES_PER_GB as f64
}

pub fn get_system_info() -> Info {
    let mut system = System::new_all();
    system.refresh_all();

    let system_name = System::name().unwrap_or_default();
    let os_version = System::os_version().unwrap_or_default();
    let hostname = System::host_name().unwrap_or_default();
    let cpu_count = system.cpus().len();
    let disks = Disks::new_with_refreshed_list();
    let s = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
    );
    let mut cpu_list = vec![];
    for cpu in s.cpus() {
        cpu_list.push(CpuListItem {
            name: cpu.name().to_string(),
            frequency: format!("{} MHz", cpu.frequency()),
            vendor_id: cpu.vendor_id().to_string()
        })
    }
    let mut disk_list = vec![];
    for disk in &disks {
        disk_list.push(
            DiskListItem {
                name: disk.name().to_string_lossy().into_owned(),
                kind: disk.kind().to_string(),
                mount_point: disk.mount_point().to_string_lossy().into_owned(),
                total_space: format!("{:.4}GB", bytes_to_gb(disk.total_space())),
            });
    }
    let total_memory = format!("{:.0}GB", bytes_to_gb(system.total_memory()));
    let available_memory = format!("{:.4}GB", bytes_to_gb(system.available_memory()));
    let networks = Networks::new_with_refreshed_list();
    let mut network_list = String::new();
    for (_interface_name, network) in &networks {
        network_list.push_str(&format!("{:?} ", network.ip_networks()));
    }
    return Info {
        system_name,
        os_version,
        hostname,
        cpu_count,
        cpus: cpu_list,
        disks: disk_list,
        total_memory,
        available_memory,
        network: network_list,
    }
}

use serde_json::json;
use std::error::Error;
use sysinfo::{System, SystemExt};

pub fn get_system_info() -> Result<Option<serde_json::Value>, Box<dyn Error>> {
    let sys = System::new_all();
    Ok(Some(json!({
        "name": sys.name(),
        "bootTime": sys.boot_time(),
        "kernelVersion": sys.kernel_version(),
        "osVersion": sys.os_version(),
        "totalMemory": sys.total_memory(),
        "freeMemory": sys.free_memory(),
    })))
}

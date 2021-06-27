
use std::{ffi::CString, os::raw::c_char};
use std::{fmt, error::Error};

use libc::statvfs;

#[derive(Debug)]
pub struct Disk {
    pub mount: String,
    pub available: u64,
    pub total: u64
}

#[derive(Debug)]
pub struct DiskStatsError(i32);

impl fmt::Display for DiskStatsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to get information!")
    }
}

impl Error for DiskStatsError {}

pub fn get_disk_info(mount_path: &str) -> Result<Disk, DiskStatsError> {
    let mut stat: statvfs = unsafe { std::mem::zeroed() };
    let ptr = CString::new(mount_path).unwrap();
    let path = ptr.as_ptr() as *const c_char;
    unsafe {
        let o = libc::statvfs(path, &mut stat);
        if o != 0 {
            return Err(DiskStatsError(*libc::__errno_location().clone()));
        }
    }
    
    Ok(Disk {
        mount: mount_path.to_owned(),
        available: stat.f_bsize * stat.f_bavail,
        total: stat.f_bsize * stat.f_blocks
    })
}

pub fn get_disks_info() -> Vec<Disk> {
    let mut disks = Vec::new();
    for x in  mnt::get_submounts("").unwrap() {
        if x.spec.starts_with("/dev") {
            if let Ok(val) = get_disk_info(x.file.to_str().unwrap()) {
                disks.push(val);
            }
        }
    };
    disks
}

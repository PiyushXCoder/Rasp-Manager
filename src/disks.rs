/*
    This file is part of Rasp Manager.
    Rasp Manager is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.
    Rasp Manager is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    You should have received a copy of the GNU General Public License
    along with Rasp Manager.  If not, see <https://www.gnu.org/licenses/>
*/


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

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

use std::process::Command;

use tide::Request;

fn exec(cmd: &mut Command) -> String {
    let out = cmd.output();

    if out.is_err() {
        return "Failed to execute command!".to_owned();
    }
    
    match String::from_utf8(out.unwrap().stdout) {
        Ok(out) => return out,
        Err(_) => return "Request timeout".to_owned()
    }
}

pub async fn ip_addr(_: Request<()>) -> tide::Result {
    Ok(exec(Command::new("ip").arg("addr")).into())
}

pub async fn ps(_: Request<()>) -> tide::Result {
    Ok(exec(&mut Command::new("ps").arg("-aux")).into())
}

pub async fn lsblk(_: Request<()>) -> tide::Result {
    Ok(exec(&mut Command::new("lsblk")).into())
}

pub async fn arp(_: Request<()>) -> tide::Result {
    Ok(exec(&mut Command::new("arp")).into())
}

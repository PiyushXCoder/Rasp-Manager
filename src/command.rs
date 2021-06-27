
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

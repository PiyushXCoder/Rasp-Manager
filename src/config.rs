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

use std::collections::HashMap;

use serde::{Deserialize};
use toml::from_str;
use clap::{App, Arg};
use tide::convert::Serialize;


#[derive(Clone, Deserialize, Serialize)]
pub struct Command {
    pub label: String,
    pub command: String
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub static_dir: Option<String>,
    pub addr: String,
    pub port: i32,
    pub commands: HashMap<String, Command>
}

impl Config {
    pub fn generate() -> Config {
        let matches = App::new("Rasp Manager")
            .about("A simple server manager for local newtrok")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONFIG")
                .help("Config file"))
            .arg(Arg::with_name("addr")
                .short("a")
                .long("addr")
                .value_name("ADDR")
                .help("Address to listen port on"))
            .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Port to listen"))
            .arg(Arg::with_name("static_dir")
                .short("s")
                .long("static_dir")
                .value_name("DIR")
                .help("Directory to host as static"))
            .get_matches();

        
        let config = if let Some(config) = matches.value_of("config") {
            let content = std::fs::read_to_string(config).expect(
                &format!("The config file doesn't exist at: {}", config)
            );
            let tomlcfg: Config = from_str(&content).expect(
                "The config file appears malformed!"
            );
            tomlcfg
        } else {
            Config {
                static_dir: None,
                addr: "0.0.0.0".to_string(),
                port: 80,
                commands: HashMap::new()
            }
        };

        Config {
            static_dir: match matches.value_of("static_dir") {
                Some(val) => Some(val.to_owned()),
                None => config.static_dir.to_owned()
            },
            addr: matches.value_of("addr").unwrap_or(&config.addr).to_owned(),
            port: matches.value_of("port").unwrap_or_default().parse().unwrap_or(config.port),
            commands: config.commands
        }
    }
}

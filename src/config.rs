use clap::{App, Arg};


pub struct Config {
    pub static_dirs: Option<String>,
    pub addr: String,
    pub port: String
}

impl Config {
    pub fn generate() -> Config {
        let matches = App::new("Rasp Manager")
            .about("Manager for raspberry pi")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .arg(Arg::with_name("addr")
                .short("a")
                .long("addr")
                .value_name("ADDR")
                .required(true)
                .help("Address to listen port on"))
            .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .required(true)
                .help("Port to listen"))
            .arg(Arg::with_name("static_dir")
                .short("s")
                .long("static_dir")
                .value_name("DIR")
                .help("Directory to host as static"))
            .get_matches();


        Config {
            static_dirs: match matches.value_of("static_dir") {
                Some(val) => Some(val.to_owned()),
                None => None
            },
            addr: matches.value_of("addr").unwrap().to_owned(),
            port: matches.value_of("port").unwrap().to_owned()
        }
    }
}

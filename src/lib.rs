use std::env;

pub struct Config {
    pub rom_path: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let rom_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Rom path not specified"),
        };

        Ok(Config { rom_path })
    }
}

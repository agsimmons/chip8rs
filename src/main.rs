use std::env;
use std::process;

use chip8rs::Config;

mod chip8;
use chip8::Chip8;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut chip8 = Chip8::new(&config);

    loop {
        chip8.debug_print_ram();
        chip8.run_instruction();
    }
}

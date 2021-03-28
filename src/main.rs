extern crate minifb;

use minifb::Key;
use std::env;
use std::io::stdout;
use std::io::Write;
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

    while chip8.window_is_open() && !chip8.window_is_key_down(Key::Escape) {
        print!("{}[2J", 27 as char);
        stdout().flush().expect("Failed to flush stdout");

        chip8.debug_print_ram();
        stdout().flush().expect("Failed to flush stdout");

        chip8.debug_print_registers();
        stdout().flush().expect("Failed to flush stdout");

        chip8.run_instruction();
    }
}

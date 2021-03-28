use std::env;
use std::io::stdout;
use std::io::Write;
use std::process;
use std::thread;
use std::time::Duration;

use chip8rs::Config;

mod chip8;
use chip8::Chip8;

const FRAME_TIME: Duration = Duration::from_micros(16667);

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut chip8 = Chip8::new(&config);

    loop {
        print!("{}[2J", 27 as char);
        stdout().flush().expect("Failed to flush stdout");
        chip8.debug_print_ram();
        stdout().flush().expect("Failed to flush stdout");

        chip8.run_instruction();

        thread::sleep(FRAME_TIME);
    }
}

use chip8rs::Config;
use std::fs;
use std::path::Path;
use std::process;

struct Ram {
    memory: [u8; 4096],
}

impl Ram {
    fn new(rom_path: &str) -> Ram {
        // Read ROM data
        let rom_path = Path::new(rom_path);
        let rom_data = fs::read(rom_path).unwrap_or_else(|err| {
            eprintln!("Error reading ROM: {}", err);
            process::exit(1);
        });

        let mut ram = Ram {
            memory: [0x0; 4096],
        };

        // Initialize Sprites
        ram.write_data(
            0x0,
            &[
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80, // F
            ],
        );

        // Load ROM
        ram.write_data(0x200, &rom_data);

        ram
    }

    fn write_data(&mut self, index: usize, data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            self.memory[i + index] = *byte;
        }
    }
}

pub struct Chip8 {
    vx: [u16; 16],
    i: u16,
    pc: u16,
    sp: u8,
    ram: Ram,
}

impl Chip8 {
    pub fn new(config: &Config) -> Chip8 {
        Chip8 {
            vx: [0x0; 16],
            i: 0x0,
            pc: 0x200,
            sp: 0x0,
            ram: Ram::new(&config.rom_path),
        }
    }

    pub fn run_instruction(&mut self) {}

    pub fn debug_print_ram(&self) {
        // NOTE: This is set to only show the beginning of ram for testing
        println!("{:?}", &self.ram.memory[0..1024]);
    }
}

use std::fs;
use std::path::Path;
use std::process;

pub struct Ram {
    memory: [u8; 4096],
}

impl Ram {
    pub fn new(rom_path: &str) -> Ram {
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

    pub fn write_data(&mut self, index: usize, data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            self.memory[i + index] = *byte;
        }
    }

    pub fn read_byte(&self, index: usize) -> &u8 {
        &self.memory[index]
    }

    pub fn read_word(&self, index: usize) -> u16 {
        let bytes = self.read_bytes(index, 2);

        // Build word from two bytes
        ((bytes[0] as u16) << 8) | (bytes[1] as u16)
    }

    pub fn read_bytes(&self, index: usize, size: usize) -> &[u8] {
        &self.memory[index..index + size]
    }

    pub fn debug_print_ram(&self) {
        // NOTE: This is set to only show the beginning of ram for testing
        println!("{:02X?}", &self.memory[0..1024]);
    }
}

extern crate minifb;

use chip8rs::Config;
use minifb::{Key, Scale, Window, WindowOptions};
use std::fs;
use std::path::Path;
use std::process;
use std::time::Duration;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const FRAME_TIME: Duration = Duration::from_micros(166000); // TODO: Change from 1/6s to 1/60s
const COLOR_EMPTY: u32 = 0x000000;
const COLOR_FILLED: u32 = 0xFFFFFF;

struct Display {
    pixels: [u32; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    window: Window,
}

impl Display {
    fn new() -> Display {
        let window_options = WindowOptions {
            scale: Scale::X16,
            ..WindowOptions::default()
        };

        let mut window = Window::new(
            "Chip8-rs - ESC to exit",
            DISPLAY_WIDTH,
            DISPLAY_HEIGHT,
            window_options,
        )
        .unwrap_or_else(|err| {
            panic!("Could not create window: {}", err);
        });

        window.limit_update_rate(Some(FRAME_TIME));

        Display {
            pixels: [COLOR_EMPTY; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            window: window,
        }
    }

    /// Clears the display
    fn clear(&mut self) {
        self.pixels.iter_mut().for_each(|x| *x = COLOR_EMPTY);
    }

    fn update(&mut self) {
        self.window
            .update_with_buffer(&self.pixels, DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .unwrap();
    }
}

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

    fn read_byte(&self, index: usize) -> &u8 {
        &self.memory[index]
    }

    fn read_word(&self, index: usize) -> u16 {
        let bytes = self.read_bytes(index, 2);

        // Build word from two bytes
        ((bytes[0] as u16) << 8) | (bytes[1] as u16)
    }

    fn read_bytes(&self, index: usize, size: usize) -> &[u8] {
        &self.memory[index..index + size]
    }
}

pub struct Chip8 {
    vx: [u16; 16],
    i: u16,
    pc: u16,
    sp: u8,
    ram: Ram,
    display: Display,
}

impl Chip8 {
    pub fn new(config: &Config) -> Chip8 {
        Chip8 {
            vx: [0x0; 16],
            i: 0x0,
            pc: 0x200,
            sp: 0x0,
            ram: Ram::new(&config.rom_path),
            display: Display::new(),
        }
    }

    pub fn window_is_open(&self) -> bool {
        self.display.window.is_open()
    }

    pub fn window_is_key_down(&self, key: Key) -> bool {
        self.display.window.is_key_down(key)
    }

    pub fn run_instruction(&mut self) {
        let current_instruction = self.ram.read_word(self.pc as usize);
        println!("Current Instruction: {:#02x}", current_instruction);

        self.pc += 2;

        if current_instruction == 0x00E0 {
            self.cls();
        } else if current_instruction == 0x00EE {
            self.ret();
        } else if current_instruction >> 12 == 0x1 {
            // 1nnn
            self.jp_addr(current_instruction);
        } else if current_instruction >> 12 == 0x6 {
            // 6xkk
            self.ld_vx_byte(current_instruction);
        } else if current_instruction >> 12 == 0xA {
            // Annn
            self.ld_i_addr(current_instruction);
        } else {
            panic!("Invalid Instruction: {:#02x}", current_instruction)
        }

        self.display.update();
    }

    /// 0nnn - SYS addr
    /// Jump to a machine code routine at nnn.
    ///
    /// This instruction is only used on the old computers on which Chip-8 was
    /// originally implemented. It is ignored by modern interpreters.
    fn sys_addr() {
        // NOOP
    }

    /// 00E0 - CLS
    /// Clear the display.
    fn cls(&mut self) {
        println!("clear_display called");
        self.display.clear();
    }

    /// 00EE - RET
    /// Return from a subroutine.
    ///
    /// The interpreter sets the program counter to the address at the top of
    /// the stack, then subtracts 1 from the stack pointer.
    fn ret(&mut self) {
        panic!("Not Implemented");
    }

    /// 1nnn - JP addr
    /// Jump to location nnn.
    ///
    /// The interpreter sets the program counter to nnn.
    fn jp_addr(&mut self, command: u16) {
        self.pc = command & 0x0FFF;
    }

    /// 2nnn - CALL addr
    /// Call subroutine at nnn.
    ///
    /// The interpreter increments the stack pointer, then puts the current PC
    /// on the top of the stack. The PC is then set to nnn.
    fn call_addr(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// 3xkk - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are equal,
    /// increments the program counter by 2.
    fn se_vx_byte(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// 4xkk - SNE Vx, byte
    /// Skip next instruction if Vx != kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are not equal,
    /// increments the program counter by 2.
    fn sne_vx_byte(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// 5xy0 - SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    ///
    /// The interpreter compares register Vx to register Vy, and if they are
    /// equal, increments the program counter by 2.
    fn se_vx_vy(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// 6xkk - LD Vx, byte
    /// Set Vx = kk.
    ///
    /// The interpreter puts the value kk into register Vx.
    fn ld_vx_byte(&mut self, command: u16) {
        let register = (command & 0x0F00) >> 8;
        let value = command & 0x00FF;

        self.vx[register as usize] = value;
    }

    /// 7xkk - ADD Vx, byte
    /// Set Vx = Vx + kk.
    ///
    /// Adds the value kk to the value of register Vx, then stores the result
    /// in Vx.
    fn add_vx_byte(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// 8xy0 - LD Vx, Vy
    /// Set Vx = Vy.
    ///
    /// Stores the value of register Vy in register Vx.
    fn ld_vx_vy(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// 8xy1 - OR Vx, Vy
    /// Set Vx = Vx OR Vy.
    ///
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the
    /// result in Vx. A bitwise OR compares the corrseponding bits from two
    /// values, and if either bit is 1, then the same bit in the result is
    /// also 1. Otherwise, it is 0.
    fn or_vx_vy(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// 8xy2 - AND Vx, Vy
    /// Set Vx = Vx AND Vy.
    ///
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the
    /// result in Vx. A bitwise AND compares the corrseponding bits from two
    /// values, and if both bits are 1, then the same bit in the result is also
    /// 1. Otherwise, it is 0.
    fn and_vx_vy(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// 8xy3 - XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    ///
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores
    /// the result in Vx. An exclusive OR compares the corrseponding bits from
    /// two values, and if the bits are not both the same, then the
    /// corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn xor_vx_vy(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// 8xy4 - ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    ///
    /// The values of Vx and Vy are added together. If the result is greater
    /// than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest
    /// 8 bits of the result are kept, and stored in Vx.
    fn add_vx_vy(&mut self, command: u16) {
        panic!("Not Implemented");
    }
    /// 8xy5 - SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    ///
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted
    /// from Vx, and the results stored in Vx.
    fn sub_vx_vy(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// 8xy6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    ///
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise
    /// 0. Then Vx is divided by 2.
    fn shr_xv(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// 8xy7 - SUBN Vx, Vy
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    ///
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted
    /// from Vy, and the results stored in Vx.
    fn subn_vx_vy(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// 8xyE - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    ///
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise
    /// to 0. Then Vx is multiplied by 2.
    fn shl_vx(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// 9xy0 - SNE Vx, Vy
    /// Skip next instruction if Vx != Vy.
    ///
    /// The values of Vx and Vy are compared, and if they are not equal, the
    /// program counter is increased by 2.
    fn sne_vx_vy(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// Annn - LD I, addr
    /// Set I = nnn.
    ///
    /// The value of register I is set to nnn.
    fn ld_i_addr(&mut self, command: u16) {
        let value = command & 0x0FFF;

        self.i = value;
    }

    /// Bnnn - JP V0, addr
    /// Jump to location nnn + V0.
    ///
    /// The program counter is set to nnn plus the value of V0.
    fn jp_v0_addr(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// Cxkk - RND Vx, byte
    /// Set Vx = random byte AND kk.
    ///
    /// The interpreter generates a random number from 0 to 255, which is then
    /// ANDed with the value kk. The results are stored in Vx. See instruction
    /// 8xy2 for more information on AND.
    fn rnd_vx_byte(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// Dxyn - DRW Vx, Vy, nibble
    /// Display n-byte sprite starting at memory location I at (Vx, Vy),
    /// set VF = collision.
    ///
    /// The interpreter reads n bytes from memory, starting at the address
    /// stored in I. These bytes are then displayed as sprites on screen at
    /// coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If
    /// this causes any pixels to be erased, VF is set to 1, otherwise it is
    /// set to 0. If the sprite is positioned so part of it is outside the
    /// coordinates of the display, it wraps around to the opposite side of the
    /// screen. See instruction 8xy3 for more information on XOR, and
    /// section 2.4, Display, for more information on the Chip-8 screen and
    /// sprites.
    fn drw_vx_vy_nibble(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the down position, PC is increased by 2.
    fn skp_vx(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// ExA1 - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the up position, PC is increased by 2.
    fn sknp_vx(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// Fx07 - LD Vx, DT
    /// Set Vx = delay timer value.
    ///
    /// The value of DT is placed into Vx.
    fn ld_vx_dt(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    ///
    /// All execution stops until a key is pressed, then the value of that key
    /// is stored in Vx.
    fn ld_vx_k(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// Fx15 - LD DT, Vx
    /// Set delay timer = Vx.
    ///
    /// DT is set equal to the value of Vx.
    fn ld_dt_vx(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// Fx18 - LD ST, Vx
    /// Set sound timer = Vx.
    ///
    /// ST is set equal to the value of Vx.
    fn ld_st_vx(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    ///
    /// The values of I and Vx are added, and the results are stored in I.
    fn add_ii_vx(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    ///
    /// The value of I is set to the location for the hexadecimal sprite
    /// corresponding to the value of Vx. See section 2.4, Display, for
    /// more information on the Chip-8 hexadecimal font.
    fn ld_f_vx(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// Fx33 - LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    ///
    /// The interpreter takes the decimal value of Vx, and places the hundreds
    /// digit in memory at location in I, the tens digit at location I+1, and
    /// the ones digit at location I+2.
    fn ld_b_vx(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// Fx55 - LD [I], Vx
    /// Store registers V0 through Vx in memory starting at location I.
    ///
    /// The interpreter copies the values of registers V0 through Vx into
    /// memory, starting at the address in I.
    fn ld_i_vx(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    /// Fx65 - LD Vx, [I]
    /// Read registers V0 through Vx from memory starting at location I.
    ///
    /// The interpreter reads values from memory starting at location I into
    /// registers V0 through Vx.
    fn load_vx_i(&mut self, command: u16) {
        panic!("Not Implemented");
    }

    pub fn debug_print_ram(&self) {
        // NOTE: This is set to only show the beginning of ram for testing
        println!("{:?}", &self.ram.memory[0..1024]);
    }

    pub fn debug_print_registers(&self) {
        println!("vX: {:?}", self.vx);
        println!("I: {:?}", self.i);
        println!("PC: {:?}", self.pc);
        println!("SP: {:?}", self.sp);
    }
}

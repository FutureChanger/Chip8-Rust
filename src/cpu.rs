use crate::execution::Execution;
use crate::operation_codes::*;
use std::fs::File;
use std::io::Read;

pub struct CPU {
    //35 opcodes (opcpde+operand), each opcode is 16bit/2bytes
    pub opcode: u16,

    //Chip8 was mostly used on 4k machines. Therefore we have 4096 memory locations, each 8bit/1byte big.
    //0x000-0x1FF reserved for the interpreter. 0x050-0x0A0 storage for the 16 built-in characters. 0x200-0xFFF free to use.
    pub memory: [u8; 4096],

    //Has 16 8bit/1byte registers (V0-VF, where VF holds a flag) Any operations the CPU does are in the register.
    //Read from memory-> load into register-> do something-> back to memory
    pub registers: [u8; 16],

    //To keep track of the order of execution. Instruction CALL cause the CPU to begin instruction in different region. When it reaches
    //RET it needs to know where it was when the CALL function hit. Stack holds program counter value when CALL was executed.
    //RETurn statement then pulls that address from the stack and puts it back into the program counter and it will be executed at the
    //next cycle.
    //It has 16 levels of stack and can therefore hold 16 different program counters. Putting program counter on the stack (pushing) and
    //pulling it off (popping)
    pub stack: [u16; 16],

    //To keep track where in the 16 levels of the stack the recent value was placed (i.e top). Only need 8bits because it is an array with
    //the size 16. Therefore we only have 16 indices. POP does not mean delete we simply decrement the StackPointer so it points to
    //the previous value.
    pub stack_pointer: u8,

    //Used to store memory addresses for use in operations. 16 bit because maximum memory address 0xFFF is too large for 8bit
    pub index_register: u16,

    //Actual instructions starting at memory address 0x200. CPU needs to keep track of what to execute next.
    //ProgramCounter is a special register, that holds address of next instructions. 16bit because of maximum memory address 0xFFF
    pub program_counter: u16,

    //Has HEX based keypad (16 different values 0x0-0xF)
    pub key: [u8; 16],

    //Resolution is max 2048 pixel only black or white (1,0)
    pub gfx: [u8; 64 * 32],

    //If it reaches 0 it stays 0, otherwise it will decrement at a rate of 60hz
    pub delay_timer: u8,

    //Works the same as delay_timer. When sound timer reaches 0 => buzz sound
    pub sound_timer: u8,

    pub key_state: [bool; 16],

    pub draw: bool,

    timer_counter: u8,
}

impl CPU {
    pub fn init() -> CPU {
        CPU {
            opcode: 0,
            index_register: 0,
            stack: [0; 16],
            gfx: [0; 64 * 32],
            sound_timer: 0,
            delay_timer: 0,
            stack_pointer: 0,
            key: [0; 16],
            registers: [0; 16],
            program_counter: 0x200,
            memory: [0; 4096],
            timer_counter: 10,
            key_state: [false; 16],
            draw: false,
        }
    }

    pub fn interpret_opcode(&mut self) {
        let first_nibble = self.opcode & 0xF000;

        match first_nibble {
            0x0000 => match self.opcode & 0x0FFF {
                0x00E0 => cls(self),
                0x00EE => ret(self),
                _ => panic!("Error in 0x0000! Opcode was: {}", self.opcode),
            },
            0x1000 => jp_addr(self),
            0x2000 => call_adr(self),
            0x3000 => se_vx_byte(self),
            0x4000 => sne_vx_byte(self),
            0x5000 => se_vx_vy(self),
            0x6000 => ld_vx_byte(self),
            0x7000 => add_vx_byte(self),
            0x8000 => match self.opcode & 0x000F {
                0x0000 => ld_vx_vy(self),
                0x0001 => or_vx_vy(self),
                0x0002 => and_vx_vy(self),
                0x0003 => xor_vx_vy(self),
                0x0004 => add_vx_vy(self),
                0x0005 => sub_vx_vy(self),
                0x0006 => shr_vx_vy(self),
                0x0007 => subn_vx_vy(self),
                0x000E => shl_vx_vy(self),
                _ => panic!("Error in 0x8000!"),
            },
            0x9000 => sne_vx_vy(self),
            0xA000 => annn_ld_i_addr(self),
            0xB000 => bnnn_jp_v0_addr(self),
            0xC000 => cxkk_rnd_vx_byte(self),
            0xD000 => dxyn_drw_vx_vy_nibble(self),
            0xE000 => match self.opcode & 0x00FF {
                0x009E => ex9e_skp_vx(self),
                0x00A1 => exa1_sknp_vx(self),
                _ => panic!("Error in 0xE000!"),
            },
            0xF000 => match self.opcode & 0x00FF {
                0x0007 => fx07_ld_vx_dt(self),
                0x000A => fx0a_ld_vx_k(self),
                0x0015 => fx15_ld_dt_vx(self),
                0x0018 => fx18_ld_st_vx(self),
                0x001E => fx1e_add_i_vx(self),
                0x0029 => fx29_ld_f_vx(self),
                0x0033 => fx33_ld_b_vx(self),
                0x0055 => fx55_ld_i_vx(self),
                0x0065 => fx65_ld_vx_i(self),
                _ => panic!("Error in 0xF000! Opcode was:{}", self.opcode),
            },
            _ => panic!("Failed"),
        }
    }

    pub fn load_game(&mut self, location: &str) {
        let mut game = File::open(location).expect("ROM was not found");
        let mut buffer = [0; 3584];
        let buffer_size = game.read(&mut buffer[..]).expect("Error when reading file");

        for i in 0..buffer_size {
            self.memory[i + 512] = buffer[i];
        }
    }

    pub fn load_fontset(&mut self) {
        let fontset: [u8; 80] = [
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
        ];

        for i in 0..80 {
            self.memory[i] = fontset[i];
        }
    }

    /// Updates both timers every 10 cycles
    pub fn update_timers(&mut self, exec: &mut Execution) {
        if self.timer_counter == 10 {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                if self.sound_timer == 1 {
                    exec.play_sound();
                }
                self.sound_timer -= 1;
            } else if self.sound_timer == 0 {
                exec.stop_sound();
            }
            self.timer_counter = 0;
        } else {
            self.timer_counter += 1;
        }
    }

    /// Executes a single CPU cycle
    pub fn execute_cycle(&mut self, core: &mut Execution) {
        // Build opcode with next two bytes
        self.opcode = (self.memory[self.program_counter as usize] as u16) << 8
            | self.memory[(self.program_counter + 1) as usize] as u16;

        // Interpret opcode
        self.interpret_opcode();

        // Update timers
        self.update_timers(core);
    }
}
